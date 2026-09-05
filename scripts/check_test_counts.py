#!/usr/bin/env python3
"""The test counts the README states must be the counts the sources declare.

The `Testing` section names a number for every surface, and those numbers are the
only description a reader gets of how much each binding is actually held to.
They drift silently: adding a test does not touch the README, and no build step
compares the two. It has already happened twice. A batch-equivalence test went
into eight bindings and left six of the numbers stale, and an indicator
conformance suite was added while the README went on saying `benchmark-core` has
four integration suites.

So each surface is counted from its sources and matched against the sentence that
claims it. Counting is deliberately static -- a grep for the declaration form of
each language -- because the alternative is running ten toolchains to check a
sentence, which nothing would then run on a pull request.

Two consequences worth knowing before adding a test:

  * `#[test]` covers proptest too. `proptest! { fn ... }` expands to `#[test]`
    per function, so the property suite counts the way the others do.
  * The JavaScript pattern allows leading whitespace, because `golden.test.js`
    nests its cases inside a wrapper that skips them when the package is not
    built. Anchoring at the line start would miss four of the seven.

Run from the repository root:  python scripts/check_test_counts.py
"""

from __future__ import annotations

import glob
import os
import re
import sys

ROOT = os.path.normpath(os.path.join(os.path.dirname(os.path.abspath(__file__)), ".."))

# The README spells small numbers as words where it reads better; both forms are
# accepted so the prose does not have to bend around this check.
WORDS = {
    "one": 1, "two": 2, "three": 3, "four": 4, "five": 5, "six": 6,
    "seven": 7, "eight": 8, "nine": 9, "ten": 10, "eleven": 11, "twelve": 12,
    "thirteen": 13, "fourteen": 14, "fifteen": 15, "sixteen": 16,
    "seventeen": 17, "eighteen": 18, "nineteen": 19, "twenty": 20,
}


def count(pattern: str, *globs: str) -> int:
    """Occurrences of a declaration pattern across every file the globs match."""
    regex = re.compile(pattern, re.M)
    total = 0
    for spec in globs:
        for path in sorted(glob.glob(os.path.join(ROOT, spec), recursive=True)):
            with open(path, encoding="utf-8") as handle:
                total += len(regex.findall(handle.read()))
    return total


def files(spec: str) -> int:
    return len(glob.glob(os.path.join(ROOT, spec)))


RUST = r"#\[test\]"
JS = r"^\s*test\("

# (label, counted value, the README phrase that states it). The phrase is matched
# against the Testing section with its whitespace collapsed, so a claim may wrap
# across lines in the source.
def surfaces() -> list[tuple[str, int, str]]:
    return [
        ("benchmark-core units", count(RUST, "crates/benchmark-core/src/**/*.rs"),
         r"`benchmark-core` — {n} unit tests"),
        ("integration suites", files("crates/benchmark-core/tests/*.rs"),
         r"Plus {n} integration suites"),
        ("conformance", count(RUST, "crates/benchmark-core/tests/conformance.rs"),
         r"{n} conformance tests"),
        ("indicator conformance", count(RUST, "crates/benchmark-core/tests/indicator_conformance.rs"),
         r"{n} indicator-conformance tests"),
        ("property", count(RUST, "crates/benchmark-core/tests/proptest_invariants.rs"),
         r"{n} property tests"),
        ("benchmark-cli", count(RUST, "crates/benchmark-cli/src/**/*.rs"),
         r"`benchmark-cli` — {n} tests"),
        ("bindings/c", count(RUST, "bindings/c/src/**/*.rs"),
         r"`bindings/c` — {n} Rust tests"),
        ("bindings/python", count(r"^def test_", "bindings/python/tests/*.py"),
         r"`bindings/python` — {n} pytest cases"),
        ("bindings/node", count(JS, "bindings/node/__tests__/*.js"),
         r"`bindings/node` — {n} `node --test` cases"),
        ("bindings/wasm", count(JS, "bindings/wasm/tests/*.js"),
         r"`bindings/wasm` — {n} against the built package"),
        ("bindings/csharp", count(r"\[Fact\]|\[Theory\]", "bindings/csharp/WickraBenchmark.Tests/*.cs"),
         r"`bindings/csharp` — {n} xUnit cases"),
        ("bindings/java", count(r"@Test", "bindings/java/src/test/**/*.java"),
         r"`bindings/java` — {n} JUnit cases"),
        ("bindings/go", count(r"^func Test", "bindings/go/*_test.go"),
         r"`bindings/go` — {n} `go test` cases"),
        ("bindings/r", files("bindings/r/tests/*.R"),
         r"`bindings/r` — {n} script suites"),
        ("fuzz targets", files("fuzz/fuzz_targets/*.rs"),
         r"`fuzz/` — {n} targets"),
        ("golden envelopes", files("golden/commands/*.json"),
         r"{n} command envelopes"),
    ]


def stated(section: str, phrase: str) -> int | None:
    """The number the README states in `phrase`, or None if the phrase is gone."""
    pattern = re.escape(phrase).replace(r"\{n\}", r"(\w+)")
    match = re.search(pattern, section)
    if match is None:
        return None
    token = match.group(1)
    if token.isdigit():
        return int(token)
    return WORDS.get(token.lower())


def main() -> int:
    with open(os.path.join(ROOT, "README.md"), encoding="utf-8") as handle:
        readme = handle.read()

    start = readme.find("\n## Testing")
    end = readme.find("\n## ", start + 1)
    if start < 0 or end < 0:
        print("README.md: no '## Testing' section to check against", file=sys.stderr)
        return 1
    # Collapsed, so a claim that wraps across lines still matches as one phrase.
    section = " ".join(readme[start:end].split())

    failures = []
    for label, actual, phrase in surfaces():
        claimed = stated(section, phrase)
        if claimed == actual:
            note = f"{actual}"
        elif claimed is None:
            note = f"{actual} counted, README no longer says '{phrase.format(n='N')}'"
            failures.append(f"{label}: {note}")
        else:
            note = f"{actual} counted, README says {claimed}"
            failures.append(f"{label}: {note}")
        print(f"  {label:<24} {note}")

    if failures:
        print(
            "\nthe README's Testing section is the only account a reader gets of "
            "how much each surface is held to; these no longer match the sources:",
            file=sys.stderr,
        )
        for failure in failures:
            print(f"  {failure}", file=sys.stderr)
        return 1

    print(f"\n{len(surfaces())} stated counts match the sources")
    return 0


if __name__ == "__main__":
    sys.exit(main())
