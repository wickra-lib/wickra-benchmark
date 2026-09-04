#!/usr/bin/env python3
"""Run the test suite without pytest, for the Python 3.9 CI row.

pytest 9.x requires Python 3.10, so the 3.9 row could only ever install pytest
8.x -- below the fix for GHSA-6w46-j5rx-g56g, with no backport. Pinning a
vulnerable package in a lock file to run these tests would be the wrong trade,
and here it buys nothing at all: not one module in this directory imports
pytest. They are plain functions with plain asserts, so the whole suite runs
without a framework.

    python bindings/python/tests/run_without_pytest.py

Deliberately a fixed list rather than a directory scan: a module that grows a
pytest import should fail loudly here rather than be skipped silently, because
skipping it on 3.9 would mean the floor interpreter quietly stops covering it.
"""

from __future__ import annotations

import importlib
import sys
import traceback
from pathlib import Path

MODULES = (
    "test_completeness",
    "test_golden",
    "test_smoke",
)


def main() -> int:
    sys.path.insert(0, str(Path(__file__).resolve().parent))

    passed = 0
    failures: list[tuple[str, str]] = []

    for name in MODULES:
        module = importlib.import_module(name)
        if "pytest" in sys.modules and getattr(module, "pytest", None) is not None:
            failures.append((name, "imports pytest; it cannot run on the 3.9 row"))
            continue
        for attr in sorted(dir(module)):
            if not attr.startswith("test_"):
                continue
            test = getattr(module, attr)
            if not callable(test):
                continue
            try:
                test()
            except Exception:  # noqa: BLE001 - report every failure, then exit non-zero
                failures.append((f"{name}.{attr}", traceback.format_exc()))
            else:
                passed += 1
                print(f"  ok  {name}.{attr}")

    print(f"\n{passed} passed, {len(failures)} failed")
    for name, detail in failures:
        print(f"\nFAILED {name}\n{detail}", file=sys.stderr)
    return 1 if failures else 0


if __name__ == "__main__":
    sys.exit(main())
