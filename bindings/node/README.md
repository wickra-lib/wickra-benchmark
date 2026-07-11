# Wickra Benchmark — Node.js

Recompute a curated benchmark case or suite with the deterministic Wickra engine
and confirm its report and hash, byte-for-byte. The value is the curated,
hash-pinned suite of `(strategy, dataset, expected report)` cases. The native
core is Rust, bound via [napi-rs].

## Install

```sh
npm install wickra-benchmark
```

The correct prebuilt native binding is pulled in automatically as an optional
dependency for your platform.

## Usage

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

## Commands

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

## License

MIT OR Apache-2.0.

[napi-rs]: https://napi.rs
