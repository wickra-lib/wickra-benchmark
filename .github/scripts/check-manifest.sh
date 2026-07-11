#!/usr/bin/env bash
# Fail if any dataset CSV has drifted from its committed blake3 digest in
# datasets/MANIFEST.json. The datasets are the ground truth every case hashes
# against, so a stray edit — accidental or malicious — must break the build
# before it can silently change a case's expected hash.
#
# Uses Python + the blake3 module (installed in CI); skips cleanly if blake3 is
# unavailable so a bare checkout does not spuriously fail.
set -euo pipefail

manifest="datasets/MANIFEST.json"

if ! python -c "import blake3" >/dev/null 2>&1; then
    echo "python blake3 module not installed — skipping dataset manifest check."
    exit 0
fi

python - "$manifest" <<'PY'
import json
import sys
from pathlib import Path

import blake3

manifest_path = Path(sys.argv[1])
manifest = json.loads(manifest_path.read_text())
datasets_dir = manifest_path.parent

failed = False

# 1) Every manifest entry hashes to its recorded digest.
for name, expected in manifest.items():
    path = datasets_dir / name
    if not path.is_file():
        print(f"ERROR: {path} is listed in the manifest but missing.")
        failed = True
        continue
    actual = blake3.blake3(path.read_bytes()).hexdigest()
    if actual != expected:
        print(f"ERROR: {path} digest drift.\n  expected {expected}\n  actual   {actual}")
        failed = True
    else:
        print(f"ok {name} {actual}")

# 2) Every CSV on disk is covered by the manifest (no untracked dataset).
for csv in sorted(datasets_dir.glob("*.csv")):
    if csv.name not in manifest:
        print(f"ERROR: {csv} is not listed in {manifest_path}.")
        failed = True

if failed:
    print("\nDataset manifest is out of sync. Regenerate the digests deliberately.")
    sys.exit(1)

print("Dataset manifest is in sync.")
PY
