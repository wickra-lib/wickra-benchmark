<p align="center">
  <a href="https://wickra.org"><img src="https://raw.githubusercontent.com/wickra-lib/.github/main/profile/wickra-banner.webp?v=514-7" alt="Wickra Benchmark — a reproducible, golden-verified benchmark suite for quant backtests, recomputable byte-for-byte in ten languages" width="100%"></a>
</p>

[![CI](https://raw.githubusercontent.com/wickra-lib/.github/main/profile/badges/wickra-benchmark/ci.svg)](https://github.com/wickra-lib/wickra-benchmark/actions/workflows/ci.yml)
[![codecov](https://raw.githubusercontent.com/wickra-lib/.github/main/profile/badges/wickra-benchmark/codecov.svg)](https://codecov.io/gh/wickra-lib/wickra-benchmark)
[![NuGet](https://raw.githubusercontent.com/wickra-lib/.github/main/profile/badges/wickra-benchmark/nuget.svg)](https://www.nuget.org/packages/Wickra.Benchmark)
[![License: MIT OR Apache-2.0](https://raw.githubusercontent.com/wickra-lib/.github/main/profile/badges/wickra-benchmark/license.svg)](https://github.com/wickra-lib/wickra-benchmark#license)

# Wickra Benchmark — C#

---

**Part of the [Wickra ecosystem](https://github.com/wickra-lib) — for C#. `dotnet add package Wickra.Benchmark` — prebuilt native library, no system dependencies.**

Recompute a curated benchmark case or suite with the deterministic Wickra engine
and confirm its report and hash, from .NET over the Wickra C ABI.

It calls the ABI through P/Invoke and returns the core's canonical JSON string
verbatim, so its responses are byte-identical to the Rust, Python, Node.js,
WASM, C/C++, Go, Java and R bindings: one runner behind every language.

## Install

```bash
dotnet add package Wickra.Benchmark
```

The native library ships prebuilt per platform under `runtimes/<rid>/native/`,
selected automatically. There is nothing to compile. Targets .NET 8 and later.

### Building from this repository (contributors)

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

## Quick start

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

### Commands

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

## Benchmark

Every binding forwards to the same data-driven Rust core, so what this one adds is
the call overhead of `[LibraryImport]` P/Invoke over the C ABI, not a different result. The core's throughput is
measured by the repository's benchmark suite and the nightly `bench.yml` run; the
numbers, the machine and how to reproduce them are in the repository
[BENCHMARKS.md](https://github.com/wickra-lib/wickra-benchmark/blob/main/BENCHMARKS.md).

## Documentation

The full guide, the spec reference and the API documentation live in the main
repository and the documentation site:

- **Repository:** <https://github.com/wickra-lib/wickra-benchmark>
- **Docs** (guides, spec reference, cookbook): <https://benchmark.wickra.org>
- **Runnable example:** [`examples/csharp/`](https://github.com/wickra-lib/wickra-benchmark/tree/main/examples/csharp)

- **Repository:** <https://github.com/wickra-lib/wickra-benchmark>
- **What a case is:** [CASES.md](https://github.com/wickra-lib/wickra-benchmark/blob/main/docs/CASES.md)
- **Reproducing from any language:** [REPRODUCING.md](https://github.com/wickra-lib/wickra-benchmark/blob/main/docs/REPRODUCING.md)
- **Built on Wickra:** <https://github.com/wickra-lib/wickra> · <https://docs.wickra.org>

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
