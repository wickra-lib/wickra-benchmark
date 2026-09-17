# Wickra Benchmark examples — Go

Runnable Go examples for the [Wickra Benchmark Go binding](../../bindings/go). The binding links against the
prebuilt C ABI library, so build and stage it once before running anything:

```bash
cargo build -p wickra-benchmark-c --release
cp target/release/libwickra_benchmark.so bindings/go/lib/linux_amd64/   # match your GOOS_GOARCH
```

## Run

As the CI examples job runs it, from the repository root:

```bash
cd examples/go && go run .
```

## The examples

| Example | What it does |
|---------|--------------|
| `run.go` | A runnable Go example: load a curated benchmark case and its dataset, recompute the report with the wickra-benchmark C ABI binding, and assert it reproduces — both `passed` (the report matches the fro |
