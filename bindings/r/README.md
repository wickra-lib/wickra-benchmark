<p align="center">
  <a href="https://wickra.org"><img src="https://raw.githubusercontent.com/wickra-lib/.github/main/profile/wickra-banner.webp?v=514" alt="Wickra Benchmark — a reproducible, golden-verified benchmark suite for quant backtests, recomputable byte-for-byte in ten languages" width="100%"></a>
</p>

[![CI](https://raw.githubusercontent.com/wickra-lib/.github/main/profile/badges/wickra-benchmark/ci.svg)](https://github.com/wickra-lib/wickra-benchmark/actions/workflows/ci.yml)
[![codecov](https://raw.githubusercontent.com/wickra-lib/.github/main/profile/badges/wickra-benchmark/codecov.svg)](https://codecov.io/gh/wickra-lib/wickra-benchmark)
[![r-universe](https://raw.githubusercontent.com/wickra-lib/.github/main/profile/badges/wickra-benchmark/r-universe.svg)](https://wickra-lib.r-universe.dev)
[![License: MIT OR Apache-2.0](https://raw.githubusercontent.com/wickra-lib/.github/main/profile/badges/wickra-benchmark/license.svg)](https://github.com/wickra-lib/wickra-benchmark#license)

# Wickra Benchmark — R

---

**Part of the [Wickra ecosystem](https://github.com/wickra-lib) — for R. `install.packages("wickrabenchmark", repos = "https://wickra-lib.r-universe.dev")` — over the C ABI via `.Call`, prebuilt library fetched on install.**

Recompute a curated benchmark case or suite with the deterministic Wickra engine
and confirm its report and hash, from R over the Wickra C ABI hub (`.Call`).

## Install

From r-universe:

```r
install.packages("wickrabenchmark", repos = "https://wickra-lib.r-universe.dev")
```

The package's `configure` downloads the prebuilt C ABI library for this exact
version from the GitHub release and bundles it, so an ordinary install needs
nothing but a C toolchain (Rtools on Windows) for the thin `.Call` glue layer. To
build against a local checkout instead, point it at the header and library with
the environment variables below.

The package compiles a small C shim against the wickra-benchmark C ABI. Point it
at the C ABI header and shared library via environment variables:

```sh
cargo build -p wickra-benchmark-c            # build the native C ABI library
export WKBENCH_INC=/path/to/bindings/c/include
export WKBENCH_LIB=/path/to/target/release
R CMD INSTALL bindings/r
```

At run time the loader must find the shared library
(`LD_LIBRARY_PATH` / `DYLD_LIBRARY_PATH`, or `PATH` on Windows).

## Quick start

Everything goes through a benchmark handle driven by JSON commands — the same
command protocol every Wickra binding shares.

```r
library(wickrabenchmark)

bench <- wkbench_new()

run_case <- paste0(
  '{"cmd":"run_case","case":{',
  '"id":"sma-crossover-01",',
  '"strategy":', strategy_spec, ',',              # a wickra-backtest StrategySpec
  '"dataset_ref":"sma-uptrend.csv",',
  '"expected":', expected_report, ',',
  '"expected_hash":"', expected_hash, '"},',
  '"data":', candles, '}'
)
result <- wkbench_command(bench, run_case)
cat(result)  # the full CaseResult as JSON
```

### Commands

| `cmd`         | Payload             | Response                                |
|---------------|---------------------|-----------------------------------------|
| `run_case`    | `{case, data}`      | the full `CaseResult`                   |
| `run_suite`   | `{suite, datasets}` | a `SuiteReport`                         |
| `list_cases`  | `{suite}`           | `{ids:[...]}` (sorted)                  |
| `version`     | —                   | `{version:...,engine_version:...}`      |

Domain errors (a bad case, an unknown command) come back in-band as
`{ok:false,error:...}`; only null/UTF-8/panic conditions raise an R error.

## Benchmark

Every binding forwards to the same data-driven Rust core, so what this one adds is
the call overhead of R's native `.Call` interface over the C ABI, not a different result. The core's throughput is
measured by the repository's benchmark suite and the nightly `bench.yml` run; the
numbers, the machine and how to reproduce them are in the repository
[BENCHMARKS.md](https://github.com/wickra-lib/wickra-benchmark/blob/main/BENCHMARKS.md).

## Documentation

The full guide, the spec reference and the API documentation live in the main
repository and the documentation site:

- **Repository:** <https://github.com/wickra-lib/wickra-benchmark>
- **Docs** (guides, spec reference, cookbook): <https://benchmark.wickra.org>
- **Runnable example:** [`examples/r/`](https://github.com/wickra-lib/wickra-benchmark/tree/main/examples/r)

Wickra Benchmark ships native bindings for Python, Node.js, WASM and Rust, plus a C ABI hub that any
C-capable language (C, C++, C#, Go, Java, R) links against — all forwarding to the
same data-driven, `unsafe`-forbidden Rust core.

## Security

Found a security issue? **Please don't open a public issue.** Report it privately
via the repository's *Security* tab (*"Report a vulnerability"*) or email
**support@wickra.org** with a subject line starting `[wickra security]`. Full
policy: <https://github.com/wickra-lib/wickra-benchmark/blob/main/SECURITY.md>.

## Disclaimer

`wickra-benchmark` is research and engineering tooling, not financial advice. A
passing case attests only that a report is the deterministic result of a given
strategy over given data — it makes no claim about the quality, profitability or
future performance of any strategy, nor about whether the data is representative
of any market. Trading carries risk; you are responsible for your own decisions.
`wickra-benchmark` is free software you run yourself: no hosted service, no data
collection, no warranty.

## License

Licensed under either of [Apache-2.0](https://github.com/wickra-lib/wickra-benchmark/blob/main/LICENSE-APACHE)
or [MIT](https://github.com/wickra-lib/wickra-benchmark/blob/main/LICENSE-MIT) at your option.
