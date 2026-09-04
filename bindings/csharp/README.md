# Wickra Benchmark — C#

C# binding for [wickra-benchmark](https://github.com/wickra-lib/wickra-benchmark).
It calls the stable **C ABI** through P/Invoke and returns the core's canonical
JSON string verbatim, so its responses are byte-identical to the Rust, Python,
Node.js, WASM, C/C++, Go, Java and R bindings: one runner behind every language.

This README is for working on the binding. The one shipped inside the NuGet
package is [`WickraBenchmark/README.md`](WickraBenchmark/README.md).

## Requirements

- .NET 8 SDK
- The native library `wickra_benchmark` (built from the C-ABI crate)

## Build the native library

```bash
cargo build -p wickra-benchmark-c            # debug   -> target/debug
cargo build -p wickra-benchmark-c --release  # release -> target/release
```

The test project copies the native library next to the test assembly; for your
own app, ensure `wickra_benchmark.dll` / `.so` / `.dylib` is on the load path.

## Usage

Everything goes through a `Benchmark` driven by JSON commands — the same command
protocol every Wickra binding shares.

```csharp
using System.Text.Json;
using Wickra.Benchmark;

using var bench = new Benchmark();

// Ask the suite what it contains.
string listed = bench.Command(JsonSerializer.Serialize(new
{
    cmd = "list_cases",
    suite = JsonSerializer.Deserialize<object>(File.ReadAllText("cases/suite.json")),
}));

Console.WriteLine(listed);        // {"ids":["breakout-channel-01", ...]}
Console.WriteLine(Benchmark.Version());
```

`Command` returns the canonical response string. A malformed command comes back
as an error envelope (`{"ok":false,"error":...}`) rather than throwing, and no
panic crosses the boundary. `Benchmark` owns a native handle, so dispose it —
`using` or `Dispose()`.

The commands are `run_case`, `run_suite`, `list_cases` and `version`; their
envelopes are documented in
[REPRODUCING.md](https://github.com/wickra-lib/wickra-benchmark/blob/main/docs/REPRODUCING.md),
and one committed example of each lives in
[`golden/commands/`](../../golden/commands/).

## Test

```bash
dotnet test WickraBenchmark.Tests/WickraBenchmark.Tests.csproj
```

The golden test replays every envelope in [`golden/commands/`](../../golden/commands/)
and asserts the response equals [`golden/expected/`](../../golden/expected/) byte
for byte. That is the cross-language parity check — it is the same assertion in
all ten languages.

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

Licensed under either of [MIT](https://github.com/wickra-lib/wickra-benchmark/blob/main/LICENSE-MIT) or
[Apache-2.0](https://github.com/wickra-lib/wickra-benchmark/blob/main/LICENSE-APACHE) at your option.
