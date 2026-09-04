#!/usr/bin/env python3
"""Assert that every place carrying the release version agrees.

The version lives in two dozen declarations across six package managers, and a
bump that misses one produces a release where, say, the npm package pins a native
binary that was never published. That failure surfaces at install time, on a
user's machine, after the tag is irreversible -- so it is worth a cheap check
before the tag rather than a patch release after it.

    python scripts/check_version_sync.py                    # all files agree
    python scripts/check_version_sync.py --previous 0.1.0   # and none is stale

The file list is explicit rather than a repository-wide grep on purpose:
`Cargo.lock` records third-party crates that will occasionally sit at the same
version as this project, and a grep matching those would either be noisy or be
silenced with exceptions that outlive their reason.

The counts are exact, not "at least one". An entry that finds fewer occurrences
than it declares means a file grew a second declaration that the bump did not
reach -- which is the failure this exists to catch, and it is invisible to a
presence check.
"""

from __future__ import annotations

import argparse
import re
import sys
from pathlib import Path

ROOT = Path(__file__).resolve().parent.parent

# (path, description, pattern with @V@ standing in for the version, expected count)
TOUCHPOINTS: list[tuple[str, str, str, int]] = [
    ("Cargo.toml", "workspace version", r'(?m)^version = "@V@"$', 1),
    ("Cargo.toml", "workspace dependency pin", r'\{ version = "@V@", path = ', 1),
    # Two crates depend on the core by path *and* version so that
    # `default-features = false` takes effect; both pins move with a bump.
    (
        "bindings/wasm/Cargo.toml",
        "core pin",
        r'path = "\.\./\.\./crates/benchmark-core", version = "@V@"',
        1,
    ),
    (
        "crates/benchmark-bench/Cargo.toml",
        "core pin",
        r'path = "\.\./benchmark-core", version = "@V@"',
        1,
    ),
    ("bindings/python/pyproject.toml", "wheel version", r'(?m)^version = "@V@"$', 1),
    ("bindings/node/package.json", "package version", r'"version": "@V@"', 1),
    (
        "bindings/node/package.json",
        "optional platform dependencies",
        r'"wickra-benchmark-[a-z0-9-]+": "@V@"',
        6,
    ),
    ("bindings/r/DESCRIPTION", "R package version", r"(?m)^Version: @V@$", 1),
    ("bindings/java/pom.xml", "Maven version", r"<version>@V@</version>", 1),
    (
        "bindings/csharp/WickraBenchmark/WickraBenchmark.csproj",
        "NuGet version",
        r"<Version>@V@</Version>",
        1,
    ),
    # The supported row and the unsupported bound.
    ("SECURITY.md", "supported version", r"@V@", 2),
    # A package-lock.json states the root package's version twice: once at the
    # top and once inside `packages[""]`. Only the first is what `npm version`
    # rewrites, so the second goes stale on its own. Both are ours, so both are
    # checked exactly.
    (
        "bindings/node/package-lock.json",
        "own version, both records",
        r'"name": "wickra-benchmark",\s+"version": "@V@"',
        2,
    ),
    (
        "bindings/node/package-lock.json",
        "optional platform dependencies",
        r'"wickra-benchmark-[a-z0-9-]+": "@V@"',
        6,
    ),
    # The Node example resolves the binding through a file: path, so it carries
    # no dependency pin -- but its lockfile still records the resolved binding's
    # version, and `npm ci` refuses a lockfile that disagrees with the manifest.
    ("examples/node/package.json", "example version", r'"version": "@V@"', 1),
    (
        "examples/node/package-lock.json",
        "example and resolved binding versions",
        r'"version": "@V@"',
        3,
    ),
    (
        "examples/node/package-lock.json",
        "optional platform dependencies",
        r'"wickra-benchmark-[a-z0-9-]+": "@V@"',
        6,
    ),
]

# Each platform package declares its own version.
PLATFORMS = [
    "darwin-arm64",
    "darwin-x64",
    "linux-arm64-gnu",
    "linux-x64-gnu",
    "win32-arm64-msvc",
    "win32-x64-msvc",
]
for platform in PLATFORMS:
    TOUCHPOINTS.append(
        (
            f"bindings/node/npm/{platform}/package.json",
            "platform package version",
            r'"version": "@V@"',
            1,
        )
    )

# CITATION.cff deliberately carries no `version` or `date-released`: nothing has
# been released yet, and a citation naming a version that does not exist is worse
# than one that names none. Add it here when the first tag lands.


def workspace_version() -> str:
    text = (ROOT / "Cargo.toml").read_text(encoding="utf-8")
    match = re.search(r'(?m)^version = "([0-9]+\.[0-9]+\.[0-9]+)"$', text)
    if not match:
        sys.exit("could not read the workspace version from Cargo.toml")
    return match.group(1)


def main() -> int:
    parser = argparse.ArgumentParser(description=__doc__)
    parser.add_argument(
        "--previous",
        help="also assert this older version appears nowhere (catches a missed bump)",
    )
    args = parser.parse_args()

    version = workspace_version()
    print(f"workspace version: {version}\n")

    problems: list[str] = []
    for rel, what, pattern, expected in TOUCHPOINTS:
        path = ROOT / rel
        if not path.exists():
            problems.append(f"{rel}: missing ({what})")
            continue
        text = path.read_text(encoding="utf-8")
        found = len(re.findall(pattern.replace("@V@", re.escape(version)), text))
        if found != expected:
            problems.append(
                f"{rel}: {what} — expected {expected} occurrence(s) of {version}, found {found}"
            )
        else:
            print(f"  ok  {rel:<52} {what}")

    if args.previous:
        stale = []
        for rel, _what, pattern, _expected in TOUCHPOINTS:
            path = ROOT / rel
            if not path.exists():
                continue
            text = path.read_text(encoding="utf-8")
            if re.search(pattern.replace("@V@", re.escape(args.previous)), text):
                stale.append(rel)
        for rel in sorted(set(stale)):
            problems.append(f"{rel}: still carries the previous version {args.previous}")

    if problems:
        print("\nversion declarations disagree:", file=sys.stderr)
        for problem in problems:
            print(f"  - {problem}", file=sys.stderr)
        return 1
    print(f"\nall {len(TOUCHPOINTS)} version declarations agree on {version}")
    return 0


if __name__ == "__main__":
    sys.exit(main())
