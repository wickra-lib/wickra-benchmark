# Wickra Benchmark — WASM

Recompute a curated benchmark case or suite with the deterministic Wickra engine
and confirm its report and hash, compiled to WebAssembly for the browser (and
any WASM host). Built with [wasm-bindgen].

The suite runs sequentially under WASM (no thread pool in a browser sandbox),
which is byte-identical to the native parallel run — the exact cross-language
golden check.

## Build

```sh
wasm-pack build --target web      # for browsers / bundlers
wasm-pack build --target nodejs   # for Node.js
```

The `pkg/` output (the `.wasm` binary plus the JS glue and TypeScript types) is
generated, not committed.

## Usage

Everything goes through a `Benchmark` driven by JSON commands — the same command
protocol every Wickra binding shares.

```js
import init, { Benchmark } from "./pkg/wickra_benchmark_wasm.js";

await init(); // load the .wasm module (web target)

const bench = new Benchmark(); // stateless: case, suite and data arrive per command

const runCase = {
  cmd: "run_case",
  case: {
    id: "sma-crossover-01",
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

| `cmd`         | Payload             | Response                                |
|---------------|---------------------|-----------------------------------------|
| `run_case`    | `{case, data}`      | the full `CaseResult`                   |
| `run_suite`   | `{suite, datasets}` | a `SuiteReport`                         |
| `list_cases`  | `{suite}`           | `{ids:[...]}` (sorted)                  |
| `version`     | —                   | `{version:...,engine_version:...}`      |

`data` is an array of candles; `datasets` maps each `dataset_ref` to its candle
array.

Domain errors (a bad case, an unknown command) come back in-band as
`{ok:false,error:...}`; a malformed command envelope throws a JS error.

## License

MIT OR Apache-2.0.

[wasm-bindgen]: https://github.com/rustwasm/wasm-bindgen
