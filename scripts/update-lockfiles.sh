#!/usr/bin/env bash
#
# Regenerate every committed lockfile in this repository:
#   - Rust:   Cargo.lock                      (cargo update)
#   - Node:   bindings/node/package-lock.json (npm install --package-lock-only)
#             examples/node/package-lock.json (same, its own module)
#   - Python: .github/requirements/*.txt      (uv pip compile --generate-hashes)
#
# Run from anywhere; it cd's to the repository root itself:
#
#     ./scripts/update-lockfiles.sh
#
# The Python locks are hash-pinned because CI installs them with
# `--require-hashes`, which is what makes the dev tooling a pinned dependency
# rather than whatever the index served that morning. They are generated with uv
# rather than pip-tools because uv resolves a *target* Python version's full
# transitive closure, with hashes, without that interpreter being installed
# locally -- which is required for the 3.9 row: it needs the tomli backport that
# later versions do not.
#
# The 3.10+ file is resolved for 3.11, the floor of its matrix rows, so the
# result installs on 3.11, 3.12 and 3.13 alike. Resolving for the newest would
# risk picking something the older rows cannot install.
set -euo pipefail

cd "$(dirname "$0")/.."

if ! command -v uv >/dev/null 2>&1; then
  echo "uv is not on PATH; install it from https://docs.astral.sh/uv/" >&2
  exit 1
fi

echo "== Rust =="
cargo update

echo "== Node =="
(cd bindings/node && npm install --package-lock-only --no-audit --no-fund)
(cd examples/node && npm install --package-lock-only --no-audit --no-fund)

echo "== Python =="
uv pip compile --generate-hashes --python-version 3.11 \
  -o .github/requirements/ci-dev-py3.txt .github/requirements/ci-dev-py3.in
# The 3.9 row installs no pytest: pytest 9 requires 3.10, and no module under
# bindings/python/tests imports pytest anyway.
uv pip compile --generate-hashes --python-version 3.9 \
  -o .github/requirements/ci-dev-py39.txt .github/requirements/ci-dev-py39.in

# The dataset-manifest job's single dependency. That job is what stands between
# an edited dataset and a silently changed case hash, so its checker is pinned
# like everything else rather than installed from whatever resolves that day.
uv pip compile --generate-hashes --python-version 3.12 \
  -o .github/requirements/manifest.txt .github/requirements/manifest.in

echo
echo "Done. Review the diff, then run the version audit:"
echo "    python3 scripts/check_version_sync.py"
