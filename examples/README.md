# Wickra Benchmark examples

A runnable example in every language. Each one loads the curated
`sma-crossover-01` case and its `sma-uptrend` dataset from [`data/`](data/),
recomputes the report with the deterministic engine, and asserts the case
**reproduces** — both `passed` (the recomputed report equals the frozen
expectation) and `hash_match` (its canonical hash equals the frozen hash). That
is the whole promise of this repository: the same inputs produce the same bytes,
in every language.

## Rust — `examples/rust/`

| Example | What it does |
| --- | --- |
| `src/main.rs` | A runnable Rust example: load a curated benchmark case and its dataset, then recompute the report with the real engine and confirm it reproduces — both `passed` (the report matches the frozen expectat |

## C / C++ — `examples/c/`

Build the library first (`cargo build -p wickra-benchmark-c --release`), then build and run
the examples via CMake, as the CI C ABI job does:

```bash
cmake -S examples/c -B examples/c/build
cmake --build examples/c/build --config Release
ctest --test-dir examples/c/build -C Release --output-on-failure
```

| Example | What it does |
| --- | --- |
| `run.c` | A minimal C example: load a curated benchmark case and its dataset, recompute |
| `run.cpp` | A minimal C++ example: load a curated benchmark case and its dataset, recompute the report with the wickra-benchmark C ABI, and assert it reproduces — both `passed` (the report matches the frozen expe |

## C# — `examples/csharp/`

| Example | What it does |
| --- | --- |
| `Run/Program.cs` | A runnable C# example: load a curated benchmark case and its dataset, recompute the report with the wickra-benchmark C ABI binding, and assert it reproduces — both `passed` (the report matches the fro |

## Go — `examples/go/`

| Example | What it does |
| --- | --- |
| `run.go` | A runnable Go example: load a curated benchmark case and its dataset, recompute the report with the wickra-benchmark C ABI binding, and assert it reproduces — both `passed` (the report matches the fro |

## R — `examples/r/`

| Example | What it does |
| --- | --- |
| `run.R` | A runnable R example: load a curated benchmark case and its dataset, recompute the report with the wickra-benchmark C ABI binding, and assert it reproduces — both `passed` (the report matches the froz |

## Java — `examples/java/`

| Example | What it does |
| --- | --- |
| `Run.java` | A runnable Java example: load a curated benchmark case and its dataset, recompute the report with the wickra-benchmark C ABI binding, and assert it reproduces — both `passed` (the report matches the f |

## Python — `examples/python/`

| Example | What it does |
| --- | --- |
| `run.py` | A runnable Python example: load a curated benchmark case and its dataset, |

## Node.js — `examples/node/`

| Example | What it does |
| --- | --- |
| `run.js` | A runnable Node.js example: load a curated benchmark case and its dataset, recompute the report with the wickra-benchmark binding, and assert it reproduces — both `passed` (the report matches the froz |

## WASM — `examples/wasm/`

Build the WASM package, serve the repository root, and open the page in a browser;
the module script inside it is what runs (CI parses it with `node --check`):

```bash
wasm-pack build bindings/wasm --target web
python -m http.server 8000     # then open http://localhost:8000/examples/wasm/
```

## Example datasets

The examples read from [`examples/data/`](data/): . The
cross-language golden fixtures, which every binding is checked against byte for
byte, live in [`../golden/`](../golden).

## Data

The examples load a self-contained copy of one case and its dataset from
[`data/`](data/):

| File | What it is |
|------|------------|
| [`data/cases/sma-crossover-01.json`](data/cases/sma-crossover-01.json) | a blessed `BenchmarkCase` (SMA(10/30) crossover) with its frozen `expected` report and `expected_hash` |
| [`data/datasets/sma-uptrend.csv`](data/datasets/sma-uptrend.csv) | the 80-bar deterministic price path the case runs on (`time,open,high,low,close,volume`) |

The same case runs through the CLI, pointing `--data-root` at the dataset
directory:

```bash
cargo build --release -p wickra-benchmark
./target/release/wickra-benchmark run-case \
  --case examples/data/cases/sma-crossover-01.json \
  --data-root examples/data/datasets
```
