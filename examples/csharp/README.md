# Wickra Benchmark examples — C#

Runnable C# examples for the [Wickra Benchmark C# binding](../../bindings/csharp). The binding consumes the C ABI
library through P/Invoke, so build it once before running anything:

```bash
cargo build -p wickra-benchmark-c --release
```

## Run

As the CI examples job runs it, from the repository root:

```bash
dotnet run --project examples/csharp/<Example>
```

## The examples

| Example | What it does |
|---------|--------------|
| `Run/Program.cs` | A runnable C# example: load a curated benchmark case and its dataset, recompute the report with the wickra-benchmark C ABI binding, and assert it reproduces — both `passed` (the report matches the fro |
