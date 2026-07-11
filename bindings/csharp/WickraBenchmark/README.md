# Wickra Benchmark — C\#

Recompute a curated benchmark case or suite with the deterministic Wickra engine
and confirm its report and hash, from .NET over the Wickra C ABI.

## Install

```sh
dotnet add package Wickra.Benchmark
```

## Usage

Everything goes through a `Benchmark` driven by JSON commands — the same command
protocol every Wickra binding shares.

```csharp
using System.Text.Json;
using Wickra.Benchmark;

using var bench = new Benchmark();

var runCase = new
{
    cmd = "run_case",
    @case = new
    {
        id = "sma-crossover-01",
        strategy = strategySpec,      // a wickra-backtest StrategySpec
        dataset_ref = "sma-uptrend.csv",
        expected = expectedReport,
        expected_hash = expectedHash,
    },
    data = candles,
};
string outJson = bench.Command(JsonSerializer.Serialize(runCase));
Console.WriteLine(outJson); // the full CaseResult as JSON
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
`{"ok":false,"error":...}`; only null/UTF-8/panic conditions throw.

## License

MIT OR Apache-2.0.
