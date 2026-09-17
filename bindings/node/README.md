<p align="center">
  <a href="https://wickra.org"><img src="https://raw.githubusercontent.com/wickra-lib/.github/main/profile/wickra-banner.webp?v=514" alt="Wickra Benchmark — a reproducible, golden-verified benchmark suite for quant backtests, recomputable byte-for-byte in ten languages" width="100%"></a>
</p>

[![CI](https://raw.githubusercontent.com/wickra-lib/.github/main/profile/badges/wickra-benchmark/ci.svg)](https://github.com/wickra-lib/wickra-benchmark/actions/workflows/ci.yml)
[![codecov](https://raw.githubusercontent.com/wickra-lib/.github/main/profile/badges/wickra-benchmark/codecov.svg)](https://codecov.io/gh/wickra-lib/wickra-benchmark)
[![npm](https://raw.githubusercontent.com/wickra-lib/.github/main/profile/badges/wickra-benchmark/npm.svg)](https://www.npmjs.com/package/wickra-benchmark)
[![License: MIT OR Apache-2.0](https://raw.githubusercontent.com/wickra-lib/.github/main/profile/badges/wickra-benchmark/license.svg)](https://github.com/wickra-lib/wickra-benchmark#license)

# Wickra Benchmark — Node.js

---

**Part of the [Wickra ecosystem](https://github.com/wickra-lib) — for Node.js. `npm install wickra-benchmark` — prebuilt native binary, no system dependencies.**

Recompute a curated benchmark case or suite with the deterministic Wickra engine
and confirm its report and hash, byte-for-byte. The value is the curated,
hash-pinned suite of `(strategy, dataset, expected report)` cases. The native
core is Rust, bound via [napi-rs].

## Install

```bash
npm install wickra-benchmark
```

The native addon ships as a prebuilt binary per platform (Linux, macOS,
Windows — x64 and arm64), selected automatically through optional
dependencies. There is nothing to compile.

The correct prebuilt native binding is pulled in automatically as an optional
dependency for your platform.

## Quick start

Everything goes through a `Benchmark` driven by JSON commands — the same command
protocol every Wickra binding shares, so this Node front-end drives the exact
same core as the native CLI.

```js
const { Benchmark } = require("wickra-benchmark");

const bench = new Benchmark(); // stateless: case, suite and data arrive per command

const runCase = {
  cmd: "run_case",
  case: {
    id: "sma-crossover-01",
    description: "SMA crossover on a deterministic uptrend",
    strategy: {/* a wickra-backtest StrategySpec */},
    dataset_ref: "sma-uptrend.csv",
    expected: {/* the frozen BacktestReport */},
    expected_hash: "b3aa...",
  },
  data: [/* candles */],
};

const result = JSON.parse(bench.command(JSON.stringify(runCase)));
console.log(result.passed && result.hash_match ? "passed" : "failed");
```

### Commands

| `cmd`         | Payload                | Response                                   |
|---------------|------------------------|--------------------------------------------|
| `run_case`    | `{case, data}`         | the full `CaseResult`                      |
| `run_suite`   | `{suite, datasets}`    | a `SuiteReport`                            |
| `list_cases`  | `{suite}`              | `{"ids":[...]}` (sorted)                   |
| `version`     | —                      | `{"version":...,"engine_version":...}`     |

`data` is an array of candles; `datasets` maps each `dataset_ref` to its candle
array (the FFI boundary has no filesystem, so datasets are supplied inline).

Domain errors (a bad case, an unknown command) come back in-band as
`{"ok":false,"error":...}`. A malformed command envelope throws.

## Benchmark

Every binding forwards to the same data-driven Rust core, so what this one adds is
the call overhead of napi-rs, not a different result. The core's throughput is
measured by the repository's benchmark suite and the nightly `bench.yml` run; the
numbers, the machine and how to reproduce them are in the repository
[BENCHMARKS.md](https://github.com/wickra-lib/wickra-benchmark/blob/main/BENCHMARKS.md).

## Documentation

The full guide, the spec reference and the API documentation live in the main
repository and the documentation site:

- **Repository:** <https://github.com/wickra-lib/wickra-benchmark>
- **Docs** (guides, spec reference, cookbook): <https://benchmark.wickra.org>
- **Runnable example:** [`examples/node/`](https://github.com/wickra-lib/wickra-benchmark/tree/main/examples/node)

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
