# Benchmark cases

Each file here is one **benchmark case**: a fixed strategy, a fixed dataset, and
the exact report and hash the deterministic Wickra engine must reproduce for
that pair. A case *passes* only when a fresh run reproduces both — the recomputed
report equals the stored `expected` **and** its canonical hash equals the stored
`expected_hash`. That is the entire promise of this repository: the same inputs
produce the same bytes, on every machine and in every binding.

## Anatomy of a case

```jsonc
{
  "id": "sma-crossover-01",              // stable, unique, kebab-case
  "description": "...",                   // one human-readable line
  "strategy": { ... },                    // a wickra-backtest StrategySpec (verbatim)
  "dataset_ref": "sma-uptrend.csv",       // a file name under datasets/
  "expected": { ... },                    // the full BacktestReport, as JSON
  "expected_hash": "<64 hex>"             // blake3 of the canonical expected report
}
```

- `strategy` is a `wickra_backtest_core::StrategySpec` exactly as the engine
  deserializes it — indicators, entry/exit rules, sizing, and costs.
- `dataset_ref` names a CSV under [`../datasets/`](../datasets/); the runner
  loads it from the `--data-root` you pass.
- `expected` and `expected_hash` are **generated, never hand-written** (see
  below). `expected_hash` is the lowercase `blake3` hex of the report after
  canonicalization (sorted keys, no whitespace, quantized floats).

[`suite.json`](suite.json) bundles all cases into a named suite so the whole set
runs — and is checked for id-uniqueness — in one command.

## Running

```sh
# one case
wickra-benchmark run-case  --case cases/sma-crossover-01.json --data-root datasets
# the whole suite (exit code 1 if any case fails to reproduce)
wickra-benchmark run-suite --suite cases/suite.json          --data-root datasets
# just the ids
wickra-benchmark list-cases --suite cases/suite.json
```

## Blessing a case (do not edit `expected`/`expected_hash` by hand)

> **Never type an `expected` report or an `expected_hash` yourself.** They are a
> photograph of what the real engine produced. To create or update a case, write
> only the `id`, `description`, `strategy`, and `dataset_ref`, leave `expected`
> as `{}` and `expected_hash` as 64 zeros, then let the engine fill them in:

```sh
# run the draft, take the engine's own recomputed report + hash as the golden values
wickra-benchmark run-case --case cases/my-case.draft.json --data-root datasets --format json \
  | jq '{id, description, strategy, dataset_ref, expected: .recomputed, expected_hash: .hash}' \
  > cases/my-case.json
```

The blessed case then self-passes by construction: rerunning it recomputes the
same report and the same hash, so `passed` and `hash_match` are both `true`.

If the engine ever changes its report shape or numbers, the cases stop passing —
which is the point. Re-bless deliberately, in a reviewed commit, so a hash change
is always a visible, intentional event rather than a silent drift.

## Adding a case

1. Pick or add a dataset under [`../datasets/`](../datasets/) (see its README —
   datasets are byte-pinned).
2. Write the draft with a fresh `id`, a one-line `description`, the `strategy`,
   and the `dataset_ref`.
3. Bless it with the command above.
4. Add the case object to [`suite.json`](suite.json).
5. Run `run-suite` — it must report `failed 0`.
