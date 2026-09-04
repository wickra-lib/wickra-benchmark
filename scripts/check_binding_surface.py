#!/usr/bin/env python3
"""Assert that every binding exposes the surface the C ABI declares.

    python scripts/check_binding_surface.py

Ten language reaches sit on one C ABI. Each is written and tested separately, so
a reach that falls behind fails nowhere: the golden corpus compares *responses*,
and a binding that never grew a method simply has no test to run. Nothing else in
the repository holds the ten surfaces against each other.

The header is the source of truth. Every `wickra_benchmark_<name>` in
`bindings/c/include/wickra_benchmark.h` is a promise the bindings make, and this
checks each language's public surface for that name, spelled the way that
language spells it.

Two exports are deliberately not demanded uniformly:

  new     every binding constructs a handle, but a constructor is not a named
          method in most of these languages, so it is checked as "the handle type
          exists" instead.
  free    releasing a native handle is spelled by the language's own resource
          contract -- `Dispose` in C#, `close` in Java, `Close` in Go -- and in
          Python, Node, WASM and R it is not exposed at all, because the handle
          is released by the collector, by wasm-bindgen's generated `free`, or by
          an external-pointer finaliser. Demanding one spelling everywhere would
          mean demanding a worse API in four languages, so the release idiom is
          declared per language and checked only where the language has one.
"""

from __future__ import annotations

import re
import sys
from pathlib import Path

ROOT = Path(__file__).resolve().parent.parent
HEADER = ROOT / "bindings" / "c" / "include" / "wickra_benchmark.h"

CONSTRUCTOR = "new"
RELEASE = "free"

# Per binding: the files carrying its public surface, how it spells an export,
# how it DECLARES a name, the pattern proving the handle type exists, and how it
# spells releasing one (None = released by the runtime, nothing to expose).
#
# Matching declarations rather than occurrences matters: a doc comment naming the
# function, or an internal call site, would otherwise let a renamed export pass.
BINDINGS = {
    # The methods live on the Rust pyclass, so __init__.py only re-exports the
    # name. The stub is where the public surface is declared, and it is what a
    # type checker and an editor read -- a method that reached the pyclass but
    # not the stub is invisible to every caller relying on either.
    "python": (
        [
            "bindings/python/python/wickra_benchmark/__init__.pyi",
            "bindings/python/python/wickra_benchmark/__init__.py",
        ],
        lambda n: "__version__" if n == "version" else n,
        r"(?m)^\s*(?:def @NAME@\(|@NAME@\s*[:=]|from .* import .*\b@NAME@\b(?!\s+as\b)|@NAME@,)",
        r"(?m)^class Benchmark\b",
        None,
    ),
    "node": (
        ["bindings/node/index.d.ts"],
        lambda n: re.sub(r"_(\w)", lambda m: m.group(1).upper(), n),
        r"(?m)^\s*(?:export declare (?:function |const ))?@NAME@\s*[(:]",
        r"export declare class Benchmark\b",
        None,
    ),
    "wasm": (
        ["bindings/wasm/src/lib.rs"],
        lambda n: n,
        r"pub fn @NAME@\s*\(|js_name = @NAME@",
        r"pub struct Benchmark\b",
        None,
    ),
    "csharp": (
        ["bindings/csharp/WickraBenchmark/Benchmark.cs"],
        lambda n: "".join(p.capitalize() for p in n.split("_")),
        r"public (?:static )?[^\n]*\b@NAME@\s*[({=]",
        r"public sealed class Benchmark\b",
        "Dispose",
    ),
    "go": (
        ["bindings/go/wickra.go"],
        lambda n: "".join("JSON" if p == "json" else p.capitalize() for p in n.split("_")),
        r"(?m)^func (?:\([^)]*\) )?@NAME@\s*\(",
        r"(?m)^type Benchmark struct\b",
        "Close",
    ),
    "java": (
        ["bindings/java/src/main/java/org/wickra/benchmark/Benchmark.java"],
        lambda n: re.sub(r"_(\w)", lambda m: m.group(1).upper(), n),
        r"(?m)^    public [^\n]*\b@NAME@\s*\(",
        r"public final class Benchmark\b",
        "close",
    ),
    "r": (
        ["bindings/r/NAMESPACE"],
        lambda n: "wkbench_" + n,
        r"(?m)^export\(@NAME@\)",
        r"wkbench_new",
        None,
    ),
}

problems: list[str] = []


def exports() -> list[str]:
    """The `wickra_benchmark_<name>` functions the header declares."""
    text = HEADER.read_text(encoding="utf-8")
    # Strip comments so a name mentioned in prose is not read as an export.
    text = re.sub(r"/\*.*?\*/", "", text, flags=re.S)
    text = re.sub(r"//[^\n]*", "", text)
    found = sorted(set(re.findall(r"\bwickra_benchmark_([a-z_0-9]+)\s*\(", text)))
    if not found:
        problems.append(f"no exports found in {HEADER.name}; has the header moved?")
    return found


def main() -> int:
    names = exports()
    checked = [n for n in names if n not in (CONSTRUCTOR, RELEASE)]
    print(f"C ABI declares {len(names)}: {', '.join(names)}")
    print(f"  held against every binding: {', '.join(checked)}\n")

    for lang, (files, spell, declares, handle, release) in BINDINGS.items():
        blobs = []
        for rel in files:
            path = ROOT / rel
            if not path.exists():
                problems.append(f"{lang}: {rel} is missing")
                continue
            blobs.append(path.read_text(encoding="utf-8"))
        if not blobs:
            continue
        source = "\n".join(blobs)

        missing = [
            name
            for name in checked
            if not re.search(declares.replace("@NAME@", re.escape(spell(name))), source)
        ]
        if not re.search(handle, source):
            missing.append("the handle type")
        if release and not re.search(declares.replace("@NAME@", re.escape(release)), source):
            missing.append(f"the release method ({release})")

        if missing:
            problems.append(f"{lang}: missing {', '.join(missing)}")
        else:
            how = release or "released by the runtime"
            print(f"  ok  {lang:<7} {len(checked)} methods + handle, {how}")

    if problems:
        print("\nbinding surface drift:", file=sys.stderr)
        for problem in problems:
            print(f"  - {problem}", file=sys.stderr)
        return 1
    print("\nevery binding covers the C ABI surface")
    return 0


if __name__ == "__main__":
    sys.exit(main())
