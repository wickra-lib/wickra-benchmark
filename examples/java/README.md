# Wickra Benchmark examples — Java

Runnable Java examples for the [Wickra Benchmark Java binding](../../bindings/java). The binding reaches the C ABI
through the Foreign Function & Memory API (JDK 22+), so build the library once
and point the JVM at it with `-Dnative.lib.dir`:

```bash
cargo build -p wickra-benchmark-c --release
```

## Run

As the CI examples job runs it, from the repository root:

```bash
javac ... && java --enable-native-access=ALL-UNNAMED ...
```

## The examples

| Example | What it does |
|---------|--------------|
| `Run.java` | A runnable Java example: load a curated benchmark case and its dataset, recompute the report with the wickra-benchmark C ABI binding, and assert it reproduces — both `passed` (the report matches the f |
