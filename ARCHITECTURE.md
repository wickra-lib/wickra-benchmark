# Architecture

`wickra-benchmark` orchestrates and verifies; it does not re-implement the
backtest. The backtest itself belongs to [`wickra-backtest`](https://github.com/wickra-lib/wickra-backtest);
the canonicalization and hash belong to [`wickra-proof`](https://github.com/wickra-lib/wickra-proof).
Benchmark loads a case, recomputes its report with that engine, hashes it with
that canonicalizer, and compares against the frozen expectation.

## Workspace layout

```
crates/
  benchmark-core        core: BenchmarkCase / Suite / CaseResult / SuiteReport,
                        run_case / run_suite / run_suite_inline, command_json
  wickra-benchmark-cli  reference CLI: run-case / run-suite / list-cases
  benchmark-bench       criterion benches
bindings/
  c                     C ABI hub (cdylib + staticlib + cbindgen header)
  python                PyO3 + maturin (native)
  node                  napi-rs (native)
  wasm                  wasm-bindgen (native, sequential runner)
  go go / csharp / java / r   consume the C ABI hub
cases/                  curated BenchmarkCase JSONs + suite.json
datasets/               curated, hash-pinned candle CSVs
golden/                 frozen expected command_json outputs (cross-language)
examples/               one runnable consumer per language
```

## The recompute path

`run_case(case, data)`:

1. `case.validate()`.
2. Deserialize the embedded `case.strategy` JSON into a `wickra_backtest`
   `StrategySpec` (a bad spec is `Error::BadSpec`).
3. `recomputed = wickra_backtest::run(&spec, data)` — the engine is
   deterministic; the same inputs always fold to the same report.
4. `hash = blake3(proof_core::canonicalize(&recomputed))` as lowercase hex.
5. `hash_match = (hash == case.expected_hash)`.
6. `passed = (serde_json::to_string(&recomputed) == serde_json::to_string(&case.expected))`
   — a byte-exact report comparison on the same serialization axis the
   cross-language golden uses. `passed` and `hash_match` are **independent**
   checks; a case counts as passing only when both are true.

`run_suite(suite, data_root)` loads each case's dataset from
`data_root/<dataset_ref>`, runs the cases (in parallel via rayon when the
`parallel` feature is on), then inserts the results **sorted by case `id`**, so
the `SuiteReport` JSON is reproducible regardless of case order or rayon
scheduling. `run_suite_inline(suite, datasets)` is the FFI-friendly variant: the
datasets arrive inline as a `BTreeMap<dataset_ref, Vec<Candle>>` rather than a
filesystem path. Both must produce the same report for the same data.

## The data-driven boundary

Everything crosses the language boundary as JSON: a `BenchmarkCase` embeds its
`StrategySpec` as opaque JSON, and every binding drives the same `command_json`
envelope and returns the core's canonical string verbatim. No language
re-implements the logic, so there is nothing to drift. The native bindings
(Python, Node, WASM) link the Rust core directly; the C-ABI consumers (C, C++,
C#, Go, Java, R) call into the same `cdylib`.

## Determinism

The golden guarantee is byte-identical output across all ten languages and
between the parallel and sequential runners. It rests on `BTreeMap` (never
`HashMap`) in every output path, a stable sort of results by `id`, no RNG, and
report floats passed through `wickra-backtest`'s serialization verbatim (never
re-formatted here). Any divergence is a bug.
