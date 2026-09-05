# Wickra Benchmark — C\#

Recompute a curated benchmark case or suite with the deterministic Wickra engine
and confirm its report and hash, from .NET over the Wickra C ABI.

It calls the ABI through P/Invoke and returns the core's canonical JSON string
verbatim, so its responses are byte-identical to the Rust, Python, Node.js,
WASM, C/C++, Go, Java and R bindings: one runner behind every language.

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

`Benchmark` owns a native handle, so dispose it — `using` or `Dispose()`.

## Commands

| `cmd`         | Payload             | Response                                |
|---------------|---------------------|-----------------------------------------|
| `run_case`    | `{case, data}`      | the full `CaseResult`                   |
| `run_suite`   | `{suite, datasets}` | a `SuiteReport`                         |
| `list_cases`  | `{suite}`           | `{ids:[...]}` (sorted)                  |
| `version`     | —                   | `{version:...,engine_version:...}`      |

`data` is an array of candles; `datasets` maps each `dataset_ref` to its candle
array. One committed example of every envelope lives in
[`golden/commands/`](https://github.com/wickra-lib/wickra-benchmark/tree/main/golden/commands).

Domain errors (a bad case, an unknown command) come back in-band as
`{"ok":false,"error":...}`; only null/UTF-8/panic conditions throw.

## Building and testing the binding

Requires the .NET 8 SDK and the native library, built from the C-ABI crate:

```bash
cargo build -p wickra-benchmark-c --release  # -> target/release
dotnet test bindings/csharp/WickraBenchmark.Tests/WickraBenchmark.Tests.csproj
```

The test project copies the native library next to the test assembly; for your
own app, ensure `wickra_benchmark.dll` / `.so` / `.dylib` is on the load path.

The golden test replays every envelope in
[`golden/commands/`](https://github.com/wickra-lib/wickra-benchmark/tree/main/golden/commands)
and asserts the response equals
[`golden/expected/`](https://github.com/wickra-lib/wickra-benchmark/tree/main/golden/expected)
byte for byte. That is the cross-language parity check — the same assertion runs
in all ten languages.

## Documentation

- **Repository:** <https://github.com/wickra-lib/wickra-benchmark>
- **What a case is:** [CASES.md](https://github.com/wickra-lib/wickra-benchmark/blob/main/docs/CASES.md)
- **Reproducing from any language:** [REPRODUCING.md](https://github.com/wickra-lib/wickra-benchmark/blob/main/docs/REPRODUCING.md)
- **Built on Wickra:** <https://github.com/wickra-lib/wickra> · <https://docs.wickra.org>

## Security

Found a security issue? **Please don't open a public issue.** Report it privately
via the repository's *Security* tab (*"Report a vulnerability"*) or email
**support@wickra.org**. Full policy:
<https://github.com/wickra-lib/wickra-benchmark/blob/main/SECURITY.md>.

## Disclaimer

Not a trading system. A benchmark report is a deterministic transform of the
input data — it is not financial advice and is not indicative of future
performance. Provided **as is**, without warranty of any kind.

## License

Licensed under either of
[MIT](https://github.com/wickra-lib/wickra-benchmark/blob/main/LICENSE-MIT) or
[Apache-2.0](https://github.com/wickra-lib/wickra-benchmark/blob/main/LICENSE-APACHE)
at your option.
