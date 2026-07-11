# Wickra Benchmark — Go

Recompute a curated benchmark case or suite with the deterministic Wickra engine
and confirm its report and hash, from Go over the C ABI hub (cgo).

## Install

```sh
go get github.com/wickra-lib/wickra-benchmark-go
```

The binding links the prebuilt C ABI library, staged per platform under
`lib/<goos>_<goarch>/`, with the header vendored under `include/`. Building from
source requires a C toolchain (cgo) and the staged native library.

## Usage

Everything goes through a `Benchmark` driven by JSON commands — the same command
protocol every Wickra binding shares.

```go
package main

import (
    "encoding/json"
    "fmt"

    wickra "github.com/wickra-lib/wickra-benchmark-go"
)

func main() {
    b := wickra.New()
    defer b.Close()

    runCase := map[string]any{
        "cmd": "run_case",
        "case": map[string]any{
            "id":            "sma-crossover-01",
            "strategy":      strategySpec, // a wickra-backtest StrategySpec
            "dataset_ref":   "sma-uptrend.csv",
            "expected":      expectedReport,
            "expected_hash": expectedHash,
        },
        "data": candles,
    }
    cmd, _ := json.Marshal(runCase)
    out, err := b.Command(string(cmd))
    if err != nil {
        panic(err)
    }
    fmt.Println(out) // the full CaseResult as JSON
}
```

## Commands

| `cmd`         | Payload             | Response                                |
|---------------|---------------------|-----------------------------------------|
| `run_case`    | `{case, data}`      | the full `CaseResult`                   |
| `run_suite`   | `{suite, datasets}` | a `SuiteReport`                         |
| `list_cases`  | `{suite}`           | `{ids:[...]}` (sorted)                  |
| `version`     | —                   | `{version:...,engine_version:...}`      |

`data` is a slice of candles; `datasets` maps each `dataset_ref` to its candle
slice.

Domain errors (a bad case, an unknown command) come back in-band as
`{ok:false,error:...}`; only null/UTF-8/panic conditions produce a Go `error`.

## License

MIT OR Apache-2.0.
