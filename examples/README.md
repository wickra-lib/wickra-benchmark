# Examples

A runnable example in every language. Each one loads the curated
`sma-crossover-01` case and its `sma-uptrend` dataset from [`data/`](data/),
recomputes the report with the deterministic engine, and asserts the case
**reproduces** — both `passed` (the recomputed report equals the frozen
expectation) and `hash_match` (its canonical hash equals the frozen hash). That
is the whole promise of this repository: the same inputs produce the same bytes,
in every language.

| Language | Path | Run |
|----------|------|-----|
| Rust | [`rust/`](rust/) | `cargo run -p wickra-benchmark-example` |
| Python | [`python/run.py`](python/run.py) | `pip install wickra-benchmark && python examples/python/run.py` |
| Node.js | [`node/`](node/) | `cd examples/node && npm install && node run.js` |
| C / C++ | [`c/`](c/) | see below |
| Go | [`go/`](go/) | `cd examples/go && go run .` |
| C# | [`csharp/Run/`](csharp/Run/) | `dotnet run --project examples/csharp/Run` |
| Java | [`java/Run.java`](java/Run.java) | see the header comment |
| R | [`r/run.R`](r/run.R) | `Rscript examples/r/run.R` |

The native bindings (Python, Node.js) load their own compiled library. The
bindings that go through the C ABI (Go, C#, Java, R, and the C/C++ example
itself) need the C ABI library built first:

```bash
cargo build --release -p wickra-benchmark-c
```

## C / C++

The C and C++ examples build with CMake and run under ctest:

```bash
cargo build --release -p wickra-benchmark-c
cmake -S examples/c -B examples/c/build
cmake --build examples/c/build --config Release
ctest --test-dir examples/c/build -C Release --output-on-failure
```

On Windows the build copies `wickra_benchmark.dll` next to each executable, since
there is no rpath.

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

## Expected output

Every example prints the version and confirms the case reproduces:

```text
wickra-benchmark 0.1.0
sma-crossover-01: REPRODUCED (passed + hash_match)
```

The Rust and Python examples print the two booleans explicitly:

```text
wickra-benchmark 0.1.0
sma-crossover-01: passed=true hash_match=true
REPRODUCED (passed + hash_match)
```

The CLI exits `0` when every case reproduces and `1` when any case fails, so a
non-reproducible engine turns a CI build red.
