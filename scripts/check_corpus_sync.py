#!/usr/bin/env python3
"""Hold every copy of the benchmark corpus to the same bytes.

    python scripts/check_corpus_sync.py

A case exists in more than one place on purpose: `cases/` is the registry,
`cases/suite.json` collects them, `golden/` is a self-contained mirror so the
cross-language fixtures need nothing above them, and `examples/data/` carries a
runnable copy for the per-language examples. Nothing in the type system ties
those together, so an engine bump can re-bless three of them and leave the
fourth behind. That happened: `examples/data/cases/sma-crossover-01.json` kept a
stale `expected_hash` and the C example was the only job in the matrix that
noticed, failing with `the case did not reproduce` long after the change looked
merged.

This is the guard for that. It compares the copies rather than recomputing them,
so it stays fast and needs no engine: blessing
(`WICKRA_BLESS=1 cargo test -p benchmark-core --test golden`) is what produces
the bytes, and this only asserts they all agree.

Exits non-zero on the first divergence, naming both sides.
"""
from __future__ import annotations

import json
import sys
from pathlib import Path

ROOT = Path(__file__).resolve().parent.parent


def load(path: Path):
    try:
        return json.loads(path.read_text(encoding="utf-8"))
    except FileNotFoundError:
        fail(f"missing: {path.relative_to(ROOT)}")
    except json.JSONDecodeError as exc:
        fail(f"{path.relative_to(ROOT)}: {exc}")


PROBLEMS: list[str] = []


def fail(message: str) -> None:
    PROBLEMS.append(message)


def rel(path: Path) -> str:
    return str(path.relative_to(ROOT)).replace("\\", "/")


def compare(what: str, left: Path, right: Path, lhs, rhs) -> None:
    if lhs != rhs:
        fail(f"{what} differs between {rel(left)} and {rel(right)}")


def main() -> int:
    cases_dir = ROOT / "cases"
    case_files = sorted(p for p in cases_dir.glob("*.json") if p.stem != "suite")
    if not case_files:
        fail("no cases found under cases/")
        return report()

    registry = {}
    for path in case_files:
        case = load(path)
        if case is None:
            continue
        if case.get("id") != path.stem:
            fail(f"{rel(path)}: id {case.get('id')!r} does not match its filename")
        registry[path.stem] = case

    # 1. cases/suite.json carries exactly the registry, case for case.
    suite_path = cases_dir / "suite.json"
    suite = load(suite_path)
    if suite is not None:
        in_suite = {c["id"]: c for c in suite.get("cases", [])}
        if set(in_suite) != set(registry):
            missing = sorted(set(registry) - set(in_suite))
            extra = sorted(set(in_suite) - set(registry))
            if missing:
                fail(f"{rel(suite_path)} is missing: {', '.join(missing)}")
            if extra:
                fail(f"{rel(suite_path)} has cases with no file: {', '.join(extra)}")
        for case_id in sorted(set(in_suite) & set(registry)):
            compare(
                f"case {case_id}",
                cases_dir / f"{case_id}.json",
                suite_path,
                registry[case_id],
                in_suite[case_id],
            )

    # 2. golden/suite.json is a verbatim copy of cases/suite.json.
    golden_suite = ROOT / "golden" / "suite.json"
    mirrored = load(golden_suite)
    if suite is not None and mirrored is not None:
        compare("suite", suite_path, golden_suite, suite, mirrored)

    # 3. Each golden run_case command embeds the registry's case unchanged.
    for case_id, case in registry.items():
        command_path = ROOT / "golden" / "commands" / f"{case_id}.json"
        command = load(command_path)
        if command is None:
            continue
        compare(
            f"case {case_id}",
            cases_dir / f"{case_id}.json",
            command_path,
            case,
            command.get("case"),
        )

    # 4. The runnable copies under examples/data/ match the registry, and the
    #    dataset each one names travels with it.
    for path in sorted((ROOT / "examples" / "data" / "cases").glob("*.json")):
        mirror = load(path)
        if mirror is None:
            continue
        if path.stem not in registry:
            fail(f"{rel(path)} mirrors a case that is not in cases/")
            continue
        compare(
            f"case {path.stem}",
            cases_dir / f"{path.stem}.json",
            path,
            registry[path.stem],
            mirror,
        )

    # 5. Every dataset copy is byte-identical to the one under datasets/.
    for mirror_dir in (ROOT / "golden" / "datasets", ROOT / "examples" / "data" / "datasets"):
        for path in sorted(mirror_dir.glob("*.csv")):
            source = ROOT / "datasets" / path.name
            if not source.exists():
                fail(f"{rel(path)} has no counterpart under datasets/")
                continue
            if source.read_bytes() != path.read_bytes():
                fail(f"{path.name} differs between datasets/ and {rel(mirror_dir)}")

    return report()


def report() -> int:
    if PROBLEMS:
        print("corpus is out of sync:", file=sys.stderr)
        for problem in PROBLEMS:
            print(f"  - {problem}", file=sys.stderr)
        print(
            "\nRe-bless the corpus, which writes every copy from one value:\n"
            "  WICKRA_BLESS=1 cargo test -p benchmark-core --test golden",
            file=sys.stderr,
        )
        return 1
    print("corpus is in sync across cases/, golden/ and examples/data/")
    return 0


if __name__ == "__main__":
    sys.exit(main())
