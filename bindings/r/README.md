# Wickra Benchmark — R

Recompute a curated benchmark case or suite with the deterministic Wickra engine
and confirm its report and hash, from R over the Wickra C ABI hub (`.Call`).

## Build / install

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

## Usage

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

## Commands

| `cmd`         | Payload             | Response                                |
|---------------|---------------------|-----------------------------------------|
| `run_case`    | `{case, data}`      | the full `CaseResult`                   |
| `run_suite`   | `{suite, datasets}` | a `SuiteReport`                         |
| `list_cases`  | `{suite}`           | `{ids:[...]}` (sorted)                  |
| `version`     | —                   | `{version:...,engine_version:...}`      |

Domain errors (a bad case, an unknown command) come back in-band as
`{ok:false,error:...}`; only null/UTF-8/panic conditions raise an R error.

## License

MIT OR Apache-2.0.
