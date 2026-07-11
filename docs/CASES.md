# Cases and suites

A **case** is the atom of this benchmark: a fixed strategy, a fixed dataset, and
the exact report and hash the deterministic engine must reproduce for that pair.
A **suite** is a named, id-unique collection of cases. This document is the
schema reference; the operational how-to (running, blessing, adding) lives in
[`cases/README.md`](../cases/README.md).

## `BenchmarkCase`

```jsonc
{
  "id": "sma-crossover-01",          // stable, unique, kebab-case; the sort + tie key
  "description": "...",               // one human-readable line
  "strategy": { ... },                // a wickra-backtest StrategySpec, verbatim
  "dataset_ref": "sma-uptrend.csv",   // a file name under the data root
  "expected": { ... },                // the frozen BacktestReport, as JSON
  "expected_hash": "<64 hex>"         // blake3 of the canonical expected report
}
```

Structural invariants (enforced by `BenchmarkCase::validate`, source
[`crates/benchmark-core/src/case.rs`](../crates/benchmark-core/src/case.rs)):

- `id` and `dataset_ref` are non-empty.
- `expected_hash` is exactly 64 lowercase hex characters.
- `strategy` and `expected` are both JSON objects.
- Unknown top-level keys are rejected (`deny_unknown_fields`).

`strategy` is kept as raw JSON (not a typed struct) so benchmark-core stays
decoupled from the engine's internals across the FFI boundary; it is deserialized
into a `wickra_backtest_core::StrategySpec` only when the case runs.

## `Suite`

```jsonc
{ "name": "wickra-benchmark v0.1 core suite", "cases": [ /* BenchmarkCase, ... */ ] }
```

The case order in the file is irrelevant — the runner sorts results by `id` — but
every `id` must be unique (a collision would make the report order ambiguous, so
the suite refuses to validate). `cases/suite.json` is the canonical suite.

## What running a case produces

`run_case` recomputes the report from `strategy` + the dataset with the pinned
engine and returns a `CaseResult`:

```jsonc
{
  "id": "sma-crossover-01",
  "passed": true,        // recomputed report is byte-exact equal to `expected`
  "hash_match": true,    // hash(recomputed) == expected_hash
  "recomputed": { ... }, // the fresh report, for diffing
  "hash": "<64 hex>"     // canonical blake3 of `recomputed`
}
```

`passed` and `hash_match` are **independent**: they diverge exactly when a case's
`expected` and `expected_hash` disagree, so a mis-blessed case is caught rather
than masked. `run_suite` fans out over the cases, re-sorts the results by `id`,
and counts a case as passing only when it both `passed` and `hash_match`.

## Contributing a case

1. Pick or add a dataset under [`datasets/`](../datasets) — see
   [`DATASETS.md`](DATASETS.md); datasets are byte-pinned.
2. Write the draft with a fresh `id`, a one-line `description`, the `strategy`,
   and the `dataset_ref`. Leave `expected` as `{}` and `expected_hash` as 64
   zeros.
3. **Bless** it — let the engine fill in `expected` and `expected_hash`:

   ```sh
   wickra-benchmark run-case --case cases/my-case.draft.json --data-root datasets --format json \
     | jq '{id, description, strategy, dataset_ref, expected: .recomputed, expected_hash: .hash}' \
     > cases/my-case.json
   ```

4. Add the case object to `cases/suite.json` and run `run-suite` — it must report
   `failed 0`.

> **Never type an `expected` report or an `expected_hash` by hand.** They are a
> photograph of what the real engine produced. Re-bless deliberately, in a
> reviewed commit, so a hash change is always a visible, intentional event.

A good case is small, deterministic, and non-degenerate — it should actually
trade, so the report exercises real metrics rather than an empty run.
