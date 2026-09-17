# Wickra Benchmark examples — R

Runnable R examples for the [Wickra Benchmark R binding](../../bindings/r). The package compiles a thin
`.Call` glue layer against the C ABI library, so build the library and install
the package first (the CI examples job does exactly this):

```bash
cargo build -p wickra-benchmark-c --release
R CMD INSTALL bindings/r
```

## Run

```bash
Rscript examples/r/<example>.R
```

## The examples

| Example | What it does |
|---------|--------------|
| `run.R` | A runnable R example: load a curated benchmark case and its dataset, recompute the report with the wickra-benchmark C ABI binding, and assert it reproduces — both `passed` (the report matches the froz |
