<p align="center">
  <a href="https://wickra.org"><img src="https://raw.githubusercontent.com/wickra-lib/.github/main/profile/wickra-banner.webp?v=514" alt="Wickra Benchmark — a reproducible, golden-verified benchmark suite for quant backtests, recomputable byte-for-byte in ten languages" width="100%"></a>
</p>

[![Built on Wickra](https://img.shields.io/badge/built%20on-wickra-3b82f6)](https://github.com/wickra-lib/wickra)
[![Status](https://img.shields.io/badge/status-pre--release-orange)](https://github.com/wickra-lib/wickra-benchmark)
[![CI](https://raw.githubusercontent.com/wickra-lib/.github/main/profile/badges/wickra-benchmark/ci.svg)](https://github.com/wickra-lib/wickra-benchmark/actions/workflows/ci.yml)
[![CodeQL](https://raw.githubusercontent.com/wickra-lib/.github/main/profile/badges/wickra-benchmark/codeql.svg)](https://github.com/wickra-lib/wickra-benchmark/actions/workflows/codeql.yml)
[![codecov](https://raw.githubusercontent.com/wickra-lib/.github/main/profile/badges/wickra-benchmark/codecov.svg)](https://codecov.io/gh/wickra-lib/wickra-benchmark)
[![License: MIT OR Apache-2.0](https://raw.githubusercontent.com/wickra-lib/.github/main/profile/badges/wickra-benchmark/license.svg)](#license)
[![OpenSSF Scorecard](https://raw.githubusercontent.com/wickra-lib/.github/main/profile/badges/wickra-benchmark/scorecard.svg)](https://scorecard.dev/viewer/?uri=github.com/wickra-lib/wickra-benchmark)
[![OpenSSF Best Practices](https://raw.githubusercontent.com/wickra-lib/.github/main/profile/badges/wickra-benchmark/best-practices.svg)](https://www.bestpractices.dev/)
[![Build provenance](https://raw.githubusercontent.com/wickra-lib/.github/main/profile/badges/wickra-benchmark/provenance.svg)](https://github.com/wickra-lib/wickra-benchmark/attestations)
[![Reproduced across 10 languages](https://img.shields.io/badge/reproduced%20across-10%20languages-3b82f6)](#reproduce-the-suite-in-any-language)
[![Docs](https://raw.githubusercontent.com/wickra-lib/.github/main/profile/badges/wickra-benchmark/docs.svg)](https://wickra.org)

---

# Wickra Benchmark

**A reproducible, golden-verified benchmark suite for quant backtests. Take a
curated `(strategy, dataset, expected report)` case, recompute it, and confirm it
reproduces byte-for-byte — the same result in ten languages, or the build goes
red.**

> **Part of the [Wickra ecosystem](https://github.com/wickra-lib).** Built on the
> same deterministic backtest engine and ten-language binding surface as
> [wickra-backtest](https://github.com/wickra-lib/wickra-backtest),
> [wickra-proof](https://github.com/wickra-lib/wickra-proof),
> [wickra-verify](https://github.com/wickra-lib/wickra-verify) and the rest.

`wickra-benchmark` is the "ImageNet for trading-strategy reproducibility": not a
new backtest engine, but the curated, hash-pinned **suite** you check an engine
against. Each **case** pins a [`wickra-backtest`](https://github.com/wickra-lib/wickra-backtest)
`StrategySpec`, a deterministic candle dataset, the expected `BacktestReport`,
and the `blake3` hash of that report's canonical form. Running a case recomputes
the report with the pinned engine and returns two independent booleans:
`passed` (the recomputed report is byte-exact equal to the frozen expectation)
and `hash_match` (its canonical hash equals the frozen `expected_hash`).

It is a **free reproducibility harness**, not a hosted service: a CLI plus ten
language bindings over one small deterministic core. Nothing you run ever leaves
your machine.

## Determinism is the product

- **Recompute, never trust** — a case passes only when a fresh run *reproduces*
  the frozen report; a stale engine, a changed default, a numerical drift all
  turn the case red.
- **Two independent checks** — `passed` (byte-exact report equality) and
  `hash_match` (canonical-hash equality) are reported separately, so a case whose
  `expected` and `expected_hash` disagree is caught, not masked.
- **Canonical hashes** — every report is hashed under the same canonicalization
  [`wickra-proof`](https://github.com/wickra-lib/wickra-proof) uses (keys sorted,
  no whitespace, floats quantized to `1e-8`, no `NaN`/`±inf`), so the hash is
  identical in every language.
- **Byte-identical across languages and runners** — a `SuiteReport` is re-sorted
  by case id and is byte-for-byte the same in all ten bindings and between the
  parallel (rayon) and sequential (WASM) runners; the cross-language golden tests
  assert it.

## Status

**Pre-release — functionally complete, CI-verified, not yet published.** The
core, the CLI, all ten language bindings, the curated case registry, the golden
corpus, the property + fuzz suites, the benchmarks and one runnable example per
language are built and green across Linux, macOS and Windows. Packages are not
yet on the registries. Track progress in [ROADMAP.md](ROADMAP.md).

## Documentation

- [`ARCHITECTURE.md`](ARCHITECTURE.md) — how the pieces fit together.
- [`docs/CASES.md`](docs/CASES.md) — the `BenchmarkCase`/`Suite` schema and how
  to contribute a case.
- [`docs/DATASETS.md`](docs/DATASETS.md) — how the datasets are curated,
  generated and hash-pinned.
- [`docs/HASHING.md`](docs/HASHING.md) — the canonicalization + blake3 contract
  shared with wickra-proof.
- [`docs/REPRODUCING.md`](docs/REPRODUCING.md) — recompute the suite in every
  language.
- [`docs/Cookbook.md`](docs/Cookbook.md) — recipes, including "gate engine
  reproducibility in CI".

## Quickstart

```bash
# Recompute a whole suite against its datasets and confirm every case reproduces.
cargo run -p wickra-benchmark -- run-suite \
  --suite cases/suite.json --data-root datasets

# Or a single case, as JSON.
cargo run -p wickra-benchmark -- run-case \
  --case cases/sma-crossover-01.json --data-root datasets --format json

# Exit 0 = every case reproduced, 1 = at least one failed (CI-friendly).
```

The bundled suite self-passes — `run-suite` reports `passed 5, failed 0` — so a
red build means the engine, not the suite, changed.

## Case and suite format

A **case** is one curated reproducibility unit:

- **`id`** — a stable, unique, kebab-case key (the sort and tie key).
- **`strategy`** — the embedded [`wickra-backtest`](https://github.com/wickra-lib/wickra-backtest)
  `StrategySpec` (indicators, entry/exit rules, sizing, costs).
- **`dataset_ref`** — the dataset CSV the case runs on, under the data root.
- **`expected`** + **`expected_hash`** — the frozen `BacktestReport` and its
  canonical `blake3` hash. **Generated, never hand-written** (see
  [`cases/README.md`](cases/README.md) for the bless flow); running the case
  recomputes and checks against both.

`cases/suite.json` bundles the cases into a named, id-unique suite. Full schema
in [`docs/CASES.md`](docs/CASES.md).

## Reproduce the suite in any language

The core is a JSON-over-C-ABI data API (`Benchmark::command_json`) exposed
natively in Rust, Python, Node.js and WASM, and over the C ABI hub in C, C++, C#,
Go, Java and R. Every binding drives the same `run_case` / `run_suite` /
`list_cases` / `version` commands and returns the core's canonical response
verbatim; the [`golden/`](golden) fixtures pin one blessed response per command
and the cross-language golden tests assert byte-for-byte equality — the same
`passed`, the same `hash_match`, the same `blake3` hashes, everywhere. One
runnable example per language lives under [`examples/`](examples); per-binding
quickstarts are in each `bindings/<lang>/README.md`.

| Language | Binding | Package |
| -------- | ------- | ------- |
| Rust | `benchmark-core` (native) | crates.io |
| Python | PyO3 (native) | PyPI |
| Node.js | napi (native) | npm |
| WASM | wasm-bindgen (native) | npm |
| C / C++ | C ABI | header + library |
| C# | C ABI (P/Invoke) | NuGet |
| Go | C ABI (cgo) | Go module |
| Java | C ABI (FFM/Panama) | Maven |
| R | C ABI (`.Call`) | R-universe |

## Contributing a case

A good case is small, deterministic and non-degenerate (it actually trades).
Add or reuse a dataset under [`datasets/`](datasets), write the draft with a
fresh `id`, `description`, `strategy` and `dataset_ref`, then **bless** it — let
the engine fill in `expected` and `expected_hash` — and add it to
`cases/suite.json`. The full flow, including the never-edit-by-hand rule, is in
[`cases/README.md`](cases/README.md) and [`docs/CASES.md`](docs/CASES.md).

## Project layout

```
crates/benchmark-core       the library: case + suite + runner + hash + command
crates/benchmark-cli        reference CLI, binary `wickra-benchmark`
crates/benchmark-bench      Criterion benchmarks
bindings/{c,python,node,wasm,go,csharp,java,r}   ten-language surface
datasets/                   deterministic candle CSVs + blake3 MANIFEST.json
cases/                      curated BenchmarkCases + suite.json
golden/                     command envelopes -> byte-exact canonical responses
examples/                   runnable per-language demos
fuzz/                       cargo-fuzz targets (case/suite parse, run_case, command_json)
```

## Building from source

```bash
cargo build --workspace
cargo test --workspace --all-features
```

Each binding builds with its own toolchain; see `bindings/<lang>/README.md`. The
C-ABI consumers (C/C++, C#, Go, Java, R) need the C ABI library first:
`cargo build --release -p wickra-benchmark-c`.

## Requirements

Rust **1.86** (workspace) / **1.88** (Node binding). Per-binding toolchains:
Python 3.9+, Node.js 22+, .NET 8, JDK 22+, Go 1.23+, R release, and a C11/C++14
compiler with CMake for the C example.

## Benchmarks

Criterion benchmarks for `run_suite` at 10/100/1000 cases (parallel vs
sequential) live in `crates/benchmark-bench`; numbers and methodology are in
[BENCHMARKS.md](BENCHMARKS.md).

## Contributing

See [CONTRIBUTING.md](CONTRIBUTING.md) and the
[Code of Conduct](CODE_OF_CONDUCT.md). Every change runs the full CI matrix (all
ten languages × three OSes) plus CodeQL, Scorecard, zizmor and a dataset-manifest
integrity check.

## Security

Report vulnerabilities per [SECURITY.md](SECURITY.md). The threat model is in
[THREAT_MODEL.md](THREAT_MODEL.md).

## License

Dual-licensed under either [MIT](LICENSE-MIT) or [Apache-2.0](LICENSE-APACHE), at
your option.

## Disclaimer

`wickra-benchmark` is research and engineering tooling, not financial advice. A
passing case attests only that a report is the deterministic result of a given
strategy over given data — it makes no claim about the quality, profitability or
future performance of any strategy, nor about whether the data is representative
of any market. Trading carries risk; you are responsible for your own decisions.
`wickra-benchmark` is free software you run yourself: no hosted service, no data
collection, no warranty.
