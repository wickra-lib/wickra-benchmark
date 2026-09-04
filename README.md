<p align="center">
  <a href="https://wickra.org"><img src="https://raw.githubusercontent.com/wickra-lib/.github/main/profile/wickra-banner.webp?v=514" alt="Wickra Benchmark — a reproducible, golden-verified benchmark suite for quant backtests, recomputable byte-for-byte in ten languages" width="100%"></a>
</p>

[![Built on Wickra](https://img.shields.io/badge/built%20on-wickra-3b82f6)](https://github.com/wickra-lib/wickra)
[![Status](https://img.shields.io/badge/status-pre--release-orange)](https://github.com/wickra-lib/wickra-benchmark)
[![CI](https://raw.githubusercontent.com/wickra-lib/.github/main/profile/badges/wickra-benchmark/ci.svg)](https://github.com/wickra-lib/wickra-benchmark/actions/workflows/ci.yml)
[![CodeQL](https://raw.githubusercontent.com/wickra-lib/.github/main/profile/badges/wickra-benchmark/codeql.svg)](https://github.com/wickra-lib/wickra-benchmark/actions/workflows/codeql.yml)
[![codecov](https://raw.githubusercontent.com/wickra-lib/.github/main/profile/badges/wickra-benchmark/codecov.svg)](https://codecov.io/gh/wickra-lib/wickra-benchmark)
[![GitHub release](https://raw.githubusercontent.com/wickra-lib/.github/main/profile/badges/wickra-benchmark/release.svg)](https://github.com/wickra-lib/wickra-benchmark/releases/latest)
[![crates.io](https://raw.githubusercontent.com/wickra-lib/.github/main/profile/badges/wickra-benchmark/crates.svg)](https://crates.io/crates/wickra-benchmark-cli)
[![PyPI](https://raw.githubusercontent.com/wickra-lib/.github/main/profile/badges/wickra-benchmark/pypi.svg)](https://pypi.org/project/wickra-benchmark/)
[![npm](https://raw.githubusercontent.com/wickra-lib/.github/main/profile/badges/wickra-benchmark/npm.svg)](https://www.npmjs.com/package/wickra-benchmark)
[![NuGet](https://raw.githubusercontent.com/wickra-lib/.github/main/profile/badges/wickra-benchmark/nuget.svg)](https://www.nuget.org/packages/Wickra.Benchmark)
[![Maven Central](https://raw.githubusercontent.com/wickra-lib/.github/main/profile/badges/wickra-benchmark/maven.svg)](https://central.sonatype.com/artifact/org.wickra/wickra-benchmark)
[![Go module](https://raw.githubusercontent.com/wickra-lib/.github/main/profile/badges/wickra-benchmark/go.svg)](https://pkg.go.dev/github.com/wickra-lib/wickra-benchmark-go)
[![R-universe](https://raw.githubusercontent.com/wickra-lib/.github/main/profile/badges/wickra-benchmark/r-universe.svg)](https://wickra-lib.r-universe.dev)
[![License: MIT OR Apache-2.0](https://raw.githubusercontent.com/wickra-lib/.github/main/profile/badges/wickra-benchmark/license.svg)](#license)
[![OpenSSF Scorecard](https://raw.githubusercontent.com/wickra-lib/.github/main/profile/badges/wickra-benchmark/scorecard.svg)](https://scorecard.dev/viewer/?uri=github.com/wickra-lib/wickra-benchmark)
[![OpenSSF Best Practices](https://raw.githubusercontent.com/wickra-lib/.github/main/profile/badges/wickra-benchmark/best-practices.svg)](https://www.bestpractices.dev)
[![Build provenance](https://raw.githubusercontent.com/wickra-lib/.github/main/profile/badges/wickra-benchmark/provenance.svg)](https://github.com/wickra-lib/wickra-benchmark/attestations)
[![Docs](https://raw.githubusercontent.com/wickra-lib/.github/main/profile/badges/wickra-benchmark/docs.svg)](https://wickra.org)
[![Verified across 10 languages](https://raw.githubusercontent.com/wickra-lib/.github/main/profile/badges/wickra-benchmark/verified.svg)](golden/)
[![Reproduced across 10 languages](https://img.shields.io/badge/reproduced%20across-10%20languages-3b82f6)](#reproduce-the-suite-in-any-language)

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

```bash
# Recompute every curated case and confirm it still reproduces, byte for byte.
wickra-benchmark run-suite --suite cases/suite.json --data-root datasets
```

```text
id                     passed  hash_match  hash
breakout-channel-01    true    true        2b1ef11f989c
buy-and-hold-01        true    true        c1f6820a3de2
ema-trend-follow-01    true    true        97a97c31a400
rsi-mean-reversion-01  true    true        664558550a58
sma-crossover-01       true    true        8f5e84ff8862
5/5 passed
```

Exit code `0` means every case reproduced, `1` that at least one did not — so a
drifting engine turns a build red rather than going unnoticed.

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

## Building everything from source

```bash
# Rust core + tests + lints
cargo test --workspace --all-features
cargo test --workspace --no-default-features   # the sequential runner
cargo clippy --workspace --all-targets --all-features -- -D warnings
cargo bench -p benchmark-bench

# Python binding (requires a Rust toolchain + maturin)
cd bindings/python && maturin develop --release && pytest

# Node binding (requires @napi-rs/cli)
cd bindings/node && npm install && npm run build && npm test

# WASM binding (requires wasm-pack)
cd bindings/wasm && wasm-pack build --target nodejs --out-dir pkg && node --test tests/

# C ABI (cdylib + staticlib + generated header)
cargo build -p wickra-benchmark-c --release

# C# binding (requires the .NET 8 SDK; links the C ABI above)
dotnet test bindings/csharp/WickraBenchmark.Tests/WickraBenchmark.Tests.csproj

# Go binding (requires a C compiler for cgo; links the C ABI above)
cd bindings/go && go test ./...

# Java binding (requires JDK 22+ and Maven; links the C ABI above)
mvn -f bindings/java test

# R binding (requires a C toolchain / Rtools; links the C ABI above)
R CMD INSTALL bindings/r
```

The Go, Java and R bindings load the C ABI shared library at run time; put
`target/release` (or `target/debug`) on the library path. Fuzzing requires a
nightly toolchain — see [`fuzz/`](fuzz/); the same never-panic invariants are
covered on stable by the property tests.

Re-blessing after an engine bump is one command, and it writes every copy of the
corpus — `cases/`, `cases/suite.json`, `golden/` and `examples/data/` — from the
same value:

```bash
WICKRA_BLESS=1 cargo test -p benchmark-core --test golden
python scripts/check_corpus_sync.py
```

## Testing

The commands are in
[Building everything from source](#building-everything-from-source).

- `benchmark-core` — 14 unit tests over case and suite validation, the CSV candle
  loader, canonicalization and hashing. Plus four integration suites: 12
  conformance tests (determinism, ordering, the pass/fail tally), 4 property
  tests, the path-vs-inline equivalence test (`run_suite` and `run_suite_inline`
  must agree on the same data), and the golden runner.
- `benchmark-cli` — 4 tests over argument parsing.
- `bindings/c` — 6 Rust tests driving the ABI itself, including its error paths,
  so a null or malformed command is proven to be reported rather than
  dereferenced.
- `bindings/python` — 8 pytest cases: smoke, golden parity, surface
  completeness. `bindings/node` — 10 `node --test` cases, same shape.
  `bindings/wasm` — 5 against the built package.
- `bindings/csharp` — 5 xUnit cases. `bindings/java` — 5 JUnit cases.
  `bindings/go` — 5 `go test` cases. `bindings/r` — one script suite.
- `fuzz/` — four targets over the untrusted-input surface: the case parser, the
  suite parser, `run_case`, and the `command_json` envelope.

On top of those, **all ten languages** replay the shared, language-neutral golden
corpus in [`golden/`](golden/) — eight command envelopes — and assert their
response is byte-identical to the committed one.

> **What "parity" means here, precisely.** The responses are compared **byte for
> byte**, not to a tolerance. That is possible because every binding returns the
> core's canonical string verbatim — the arithmetic is not reimplemented
> anywhere — and because a report is canonicalized to sorted keys and
> round-trippable floats before it is hashed. It is not free, though: a case may
> name any indicator the engine offers, and some of those call a transcendental
> from the platform's math library (`ln`, `exp` and friends). No mainstream libm
> rounds those correctly, and implementations differ in the last bit. A case
> built on one would have to compare to a relative tolerance instead. None
> currently does, and that is a property of the corpus worth keeping
> deliberately rather than by accident.

## Requirements

Rust **1.86** (workspace) / **1.88** (Node binding). Per-binding toolchains:
Python 3.9+, Node.js 22+, .NET 8, JDK 22+, Go 1.23+, R release, and a C11/C++14
compiler with CMake for the C example.

## Benchmarks

Criterion benchmarks for `run_suite` at 10/100/1000 cases (parallel vs
sequential) live in `crates/benchmark-bench`; numbers and methodology are in
[BENCHMARKS.md](BENCHMARKS.md).

## Ecosystem

Part of the [Wickra](https://github.com/wickra-lib/wickra) family — each one a
data-driven core with a CLI and the same ten-language binding surface:

- [**wickra**](https://github.com/wickra-lib/wickra) — main library (Rust core + Python / Node.js / WASM bindings + a C ABI for C / C++ / C# / Go / Java / R)
- [**wickra-playground**](https://github.com/wickra-lib/wickra-playground) — a polyglot strategy playground: one StrategySpec live side by side in Python, Rust, JS and Go, entirely in the browser
- [**wickra-backtest**](https://github.com/wickra-lib/wickra-backtest) — the deterministic engine every case here is recomputed with
- [**wickra-exchange**](https://github.com/wickra-lib/wickra-exchange) — unified market-data + execution across ten crypto exchanges
- [**wickra-terminal**](https://github.com/wickra-lib/wickra-terminal) — the trading terminal: a TUI and a browser renderer over the stack
- [**wickra-screener**](https://github.com/wickra-lib/wickra-screener) — parallel multi-symbol screening over 514 streaming indicators
- [**wickra-xray**](https://github.com/wickra-lib/wickra-xray) — market-microstructure explorer: footprint, order-book heatmap, liquidation map, funding/OI divergence
- [**wickra-radar**](https://github.com/wickra-lib/wickra-radar) — perp-universe alert radar: OI delta, funding flip, book imbalance, liquidation clusters, OI/price divergence
- [**wickra-copilot**](https://github.com/wickra-lib/wickra-copilot) — local market copilot grounded in real order-book, liquidation and funding microstructure
- [**wickra-shazam**](https://github.com/wickra-lib/wickra-shazam) — match an asset's current microstructure fingerprint against its entire history
- **wickra-benchmark** — this repository: the curated, hash-pinned suite you check an engine against
- [**wickra-strategy-ci**](https://github.com/wickra-lib/wickra-strategy-ci) — Jest for trading strategies: golden-pin the report, catch regressions in CI, property-test against fuzzed data
- [**wickra-verify**](https://github.com/wickra-lib/wickra-verify) — confirm or refute a claimed backtest report against its strategy and data, in ten languages
- [**wickra-proof**](https://github.com/wickra-lib/wickra-proof) — Proof-of-Backtest: deterministic (spec, data) → report + blake3 hash, recomputable byte-for-byte in ten languages
- [**wickra-zk**](https://github.com/wickra-lib/wickra-zk) — prove a backtest zero-knowledge — on-chain-verifiable performance without revealing the data or the strategy
- [**wickra-impact**](https://github.com/wickra-lib/wickra-impact) — the backtester that knows you would have moved the market: agent-based fills on the real historical L2 order book
- [**wickra-darwin**](https://github.com/wickra-lib/wickra-darwin) — evolutionary strategy search at millions of backtests per second, mutating and crossing JSON specs across the 514-indicator space
- [**wickra-gym**](https://github.com/wickra-lib/wickra-gym) — a Gymnasium-compatible, microstructure-aware backtest environment with O(1) steps for deterministic RL rollouts
- [**wickra-feature-store**](https://github.com/wickra-lib/wickra-feature-store) — OHLCV and microstructure streams into ML-ready feature matrices over 514 O(1) streaming indicators
- [**wickra-genome**](https://github.com/wickra-lib/wickra-genome) — a vector database of the whole market: every asset a 514-dim live vector, for similarity search, clustering and anomaly detection
- [**wickra-timemachine**](https://github.com/wickra-lib/wickra-timemachine) — scrub the whole market like a video — every symbol, full order book, rewound to any moment via deterministic re-fold
- [**wickra-synth**](https://github.com/wickra-lib/wickra-synth) — deterministic synthetic market microstructure: OHLCV, order book, trades and funding from a single seed
- [**wickra-compile**](https://github.com/wickra-lib/wickra-compile) — compile a strategy spec into a standalone deployable: a WASM module, a self-contained binary, or a `no_std` artifact
- [**wickra-embed**](https://github.com/wickra-lib/wickra-embed) — allocation-free, `no_std` streaming indicators for bare-metal and HFT, byte-for-byte identical to the core
- [**wickra-pico**](https://github.com/wickra-lib/wickra-pico) — the O(1) indicator core running bare-metal on a $5 Raspberry Pi Pico — the LED blinks on the EMA cross

The indicator core underneath documents itself at
[docs.wickra.org](https://docs.wickra.org).

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

---

<p align="center">
  <a href="https://github.com/wickra-lib/wickra-benchmark">
    <img alt="GitHub stars" src="https://raw.githubusercontent.com/wickra-lib/.github/main/profile/badges/wickra-benchmark/stars.svg">
  </a>
  <a href="https://github.com/wickra-lib/wickra-benchmark/network/members">
    <img alt="GitHub forks" src="https://raw.githubusercontent.com/wickra-lib/.github/main/profile/badges/wickra-benchmark/forks.svg">
  </a>
  <a href="https://github.com/wickra-lib/wickra-benchmark/issues">
    <img alt="GitHub issues" src="https://raw.githubusercontent.com/wickra-lib/.github/main/profile/badges/wickra-benchmark/issues.svg">
  </a>
</p>

<p align="center">
  Built on <a href="https://github.com/wickra-lib/wickra">Wickra</a>. If it saved you time, the cheapest way to say thanks is to ⭐ the repo.
</p>

<p align="center">
  <img alt="wickra-benchmark star history" width="640"
       src="https://raw.githubusercontent.com/wickra-lib/.github/main/profile/badges/wickra-benchmark/star-history.svg">
</p>
