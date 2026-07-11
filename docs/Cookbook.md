# Cookbook

Short, practical recipes for `wickra-benchmark`. See
[`REPRODUCING.md`](REPRODUCING.md) for the command surface and
[`CASES.md`](CASES.md) for the case schema.

## Gate engine reproducibility in CI

Because `run-suite` exits `1` on any failure, it drops straight into a pipeline —
if a change to your engine (or its dependencies) alters a report, the build goes
red:

```sh
wickra-benchmark run-suite --suite cases/suite.json --data-root datasets
```

Wire this into the engine repo's CI so a silent numerical drift can never merge —
the recomputation is the gate.

## Run a single case as JSON

```sh
wickra-benchmark run-case \
  --case cases/sma-crossover-01.json --data-root datasets --format json
```

The `.passed` and `.hash_match` fields are the verdict; `.recomputed` is the fresh
report (diff it against the case's `expected` to see exactly what changed).

## List the cases in a suite

```sh
wickra-benchmark list-cases --suite cases/suite.json
```

Returns the sorted case ids — handy for sharding a large suite across CI jobs.

## Add and bless a new case

Write a draft with `expected: {}` and `expected_hash` set to 64 zeros, then let
the engine fill them in:

```sh
wickra-benchmark run-case --case cases/my-case.draft.json --data-root datasets --format json \
  | jq '{id, description, strategy, dataset_ref, expected: .recomputed, expected_hash: .hash}' \
  > cases/my-case.json
```

Add the result to `cases/suite.json` and re-run `run-suite` — it must report
`failed 0`. Never hand-edit `expected`/`expected_hash`; see [`CASES.md`](CASES.md).

## Reproduce a case from another language

Each binding drives the same envelope. For example, in Python:

```python
import json
from wickra_benchmark import Benchmark

case = json.load(open("cases/sma-crossover-01.json"))
data = [ ... ]  # the candles from datasets/sma-uptrend.csv
out = json.loads(Benchmark().command(json.dumps({"cmd": "run_case", "case": case, "data": data})))
assert out["passed"] and out["hash_match"]
```

See [`examples/`](../examples) for a runnable version in every language.

## Detect a mis-blessed case

`passed` and `hash_match` are independent. If a case reports `passed: true` but
`hash_match: false` (or vice versa), its `expected` report and `expected_hash`
disagree — re-bless it from a single fresh run so both come from the same report.
