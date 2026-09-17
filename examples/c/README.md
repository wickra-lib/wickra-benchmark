# Wickra Benchmark — C / C++ examples

The Wickra Benchmark C ABI is a single shared/static library plus a generated header
([`bindings/c/include/wickra_benchmark.h`](../../bindings/c/include/wickra_benchmark.h)). Any C-capable
language links against the same artifact; these examples show the plain-C path
and, through [`wickra_benchmark.hpp`](../../bindings/c/include/wickra_benchmark.hpp), the C++ one.

## Build the library

From the workspace root:

```sh
cargo build -p wickra-benchmark-c --release
```

This produces, in `target/release/`:

| Platform | Shared library | Link target |
|----------|----------------|-------------|
| Linux    | `libwickra_benchmark.so`     | `-lwickra_benchmark` |
| macOS    | `libwickra_benchmark.dylib`  | `-lwickra_benchmark` |
| Windows (MSVC) | `wickra_benchmark.dll` | `wickra_benchmark.dll.lib` (import lib) |

A static library (`libwickra_benchmark.a` / `wickra_benchmark.lib`) is emitted alongside.

## Build and run the examples

### With CMake (portable, used by CI)

```sh
cmake -S examples/c -B examples/c/build
cmake --build examples/c/build --config Release
ctest --test-dir examples/c/build -C Release --output-on-failure
```

### Directly with a compiler

```sh
# Linux / macOS
cc examples/c/run.c -DDATA_DIR=\"examples/data\" -I bindings/c/include -L target/release -lwickra_benchmark -lm -o run
LD_LIBRARY_PATH=target/release ./run        # macOS: DYLD_LIBRARY_PATH

# Windows (MinGW gcc, linking the DLL directly)
gcc examples/c/run.c -DDATA_DIR=\"examples/data\" -I bindings/c/include target/release/wickra_benchmark.dll -lm -o run.exe
```

## The examples

| Example | What it does |
|---------|--------------|
| `run.c` | A minimal C example: load a curated benchmark case and its dataset, recompute |
| `run.cpp` | A minimal C++ example: load a curated benchmark case and its dataset, recompute the report with the wickra-benchmark C ABI, and assert it reproduces — both `passed` (the report matches the frozen expe |

## Usage shape

Every call follows the same handle discipline: construct from a spec JSON, drive
with command JSON, read the response, free the handle exactly once. `wickra_benchmark.h` is
the whole contract; the C++ header, where one ships, wraps the handle in a
move-only RAII type. See [`bindings/c/README.md`](../../bindings/c/README.md).
