<p align="center">
  <a href="https://wickra.org"><img src="https://raw.githubusercontent.com/wickra-lib/.github/main/profile/wickra-banner.webp?v=514-7" alt="Wickra Benchmark — a reproducible, golden-verified benchmark suite for quant backtests, recomputable byte-for-byte in ten languages" width="100%"></a>
</p>

[![CI](https://raw.githubusercontent.com/wickra-lib/.github/main/profile/badges/wickra-benchmark/ci.svg)](https://github.com/wickra-lib/wickra-benchmark/actions/workflows/ci.yml)
[![codecov](https://raw.githubusercontent.com/wickra-lib/.github/main/profile/badges/wickra-benchmark/codecov.svg)](https://codecov.io/gh/wickra-lib/wickra-benchmark)
[![GitHub release](https://raw.githubusercontent.com/wickra-lib/.github/main/profile/badges/wickra-benchmark/release.svg)](https://github.com/wickra-lib/wickra-benchmark/releases/latest)
[![License: MIT OR Apache-2.0](https://raw.githubusercontent.com/wickra-lib/.github/main/profile/badges/wickra-benchmark/license.svg)](https://github.com/wickra-lib/wickra-benchmark#license)

# Wickra Benchmark — C / C++

---

**Part of the [Wickra ecosystem](https://github.com/wickra-lib) — for C / C++. `cargo build -p wickra-benchmark-c --release` — a prebuilt shared/static library plus a generated `wickra_benchmark.h`, no system dependencies.**

The C ABI is the hub every C-capable language (C, C++, C#, Go, Java, R) links
against. It exposes `wickra-benchmark-core` as a tiny, JSON-shaped surface built as both
a `cdylib` (dynamic library) and a `staticlib`.

## Install

Grab the prebuilt header + library for your platform from the
[GitHub releases](https://github.com/wickra-lib/wickra-benchmark/releases) — each archive
has `wickra_benchmark.h`, the C++ wrapper where the binding ships one, and the shared/static
library — or build from source:

```bash
cargo build -p wickra-benchmark-c --release
# -> target/release/libwickra_benchmark.{so,dylib} or wickra_benchmark.dll (+ import lib) + a staticlib
```

Then compile against the header and link the library.

## Quick start

[`examples/c/run.c`](https://github.com/wickra-lib/wickra-benchmark/blob/main/examples/c/run.c) is the runnable example the CI smoke job executes; in full:

```c
/* A minimal C example: load a curated benchmark case and its dataset, recompute
 * the report with the wickra-benchmark C ABI, and assert it reproduces — both
 * `passed` (the report matches the frozen expectation) and `hash_match` (its
 * canonical hash matches). This is the whole product in one file.
 *
 * No JSON parser is needed: the case JSON is read verbatim and embedded as the
 * `case` value, the CSV is turned into a candle array by hand, and the response
 * is inspected with a substring search. DATA_DIR is injected by CMake. */
#include <stdio.h>
#include <stdlib.h>
#include <string.h>

#include "wickra_benchmark.h"

/* Read an entire text file into a freshly malloc'd, NUL-terminated buffer. */
static char *slurp(const char *path) {
    FILE *f = fopen(path, "rb");
    if (!f) {
        fprintf(stderr, "cannot open %s\n", path);
        return NULL;
    }
    fseek(f, 0, SEEK_END);
    long n = ftell(f);
    fseek(f, 0, SEEK_SET);
    char *buf = (char *)malloc((size_t)n + 1);
    if (buf) {
        size_t got = fread(buf, 1, (size_t)n, f);
        buf[got] = '\0';
    }
    fclose(f);
    return buf;
}

/* Turn a `time,open,high,low,close,volume` CSV (with a header row) into a JSON
 * candle array. Returns a freshly malloc'd string. */
static char *candles_json(const char *csv_path) {
    FILE *f = fopen(csv_path, "r");
    if (!f) {
        fprintf(stderr, "cannot open %s\n", csv_path);
        return NULL;
    }
    size_t cap = 1 << 16, len = 0;
    char *out = (char *)malloc(cap);
    if (!out) {
        fclose(f);
        return NULL;
    }
    len += (size_t)sprintf(out + len, "[");
    char line[256];
    int first = 1, header = 1;
    while (fgets(line, sizeof line, f)) {
        long long t;
        double o, h, l, c, v;
        if (sscanf(line, "%lld,%lf,%lf,%lf,%lf,%lf", &t, &o, &h, &l, &c, &v) != 6) {
            header = 0; /* the header row fails to parse and is skipped */
            continue;
        }
        (void)header;
        if (cap - len < 256) {
            cap *= 2;
            out = (char *)realloc(out, cap);
        }
        len += (size_t)sprintf(out + len,
                               "%s{\"time\":%lld,\"open\":%.17g,\"high\":%.17g,"
                               "\"low\":%.17g,\"close\":%.17g,\"volume\":%.17g}",
                               first ? "" : ",", t, o, h, l, c, v);
        first = 0;
    }
    sprintf(out + len, "]");
    fclose(f);
    return out;
}

/* Run a command and return its response using the length-out protocol. */
static char *run(WickraBenchmark *b, const char *cmd) {
    int len = wickra_benchmark_command(b, cmd, NULL, 0);
    if (len < 0) {
        fprintf(stderr, "command failed: code %d\n", len);
        return NULL;
    }
    char *buf = (char *)malloc((size_t)len + 1);
    if (buf) {
        wickra_benchmark_command(b, cmd, buf, (size_t)len + 1);
    }
    return buf;
}

int main(void) {
    char *case_json = slurp(DATA_DIR "/cases/sma-crossover-01.json");
    char *data_json = candles_json(DATA_DIR "/datasets/sma-uptrend.csv");
    if (!case_json || !data_json) {
        return 1;
    }

    size_t cap = strlen(case_json) + strlen(data_json) + 64;
    char *cmd = (char *)malloc(cap);
    snprintf(cmd, cap, "{\"cmd\":\"run_case\",\"case\":%s,\"data\":%s}", case_json, data_json);

    WickraBenchmark *b = wickra_benchmark_new();
    char *resp = run(b, cmd);
    int ok = resp && strstr(resp, "\"passed\":true") && strstr(resp, "\"hash_match\":true");

    printf("wickra-benchmark %s\n", wickra_benchmark_version());
    printf("sma-crossover-01: %s\n", ok ? "REPRODUCED (passed + hash_match)" : "MISMATCH");

    free(resp);
    free(cmd);
    free(data_json);
    free(case_json);
    wickra_benchmark_free(b);
    if (!ok) {
        fprintf(stderr, "the case did not reproduce\n");
        return 1;
    }
    return 0;
}
```

### Surface

```c
#include "wickra_benchmark.h"

WickraBenchmark *wickra_benchmark_new(void);
void             wickra_benchmark_free(WickraBenchmark *handle);
int32_t          wickra_benchmark_command(WickraBenchmark *handle,
                                          const char *cmd_json,
                                          char *out, size_t cap);
const char      *wickra_benchmark_version(void);
```

- **`wickra_benchmark_new`** creates a benchmark handle. Never fails.
- **`wickra_benchmark_free`** destroys a handle (null is a no-op).
- **`wickra_benchmark_command`** applies a command JSON and writes the response
  JSON into the caller's buffer using a length-out protocol (below).
- **`wickra_benchmark_version`** returns a static, NUL-terminated version string
  (do not free).

### Command / response protocol

Everything goes through `wickra_benchmark_command`. Commands are JSON objects
with a `"cmd"` field: `run_case`, `run_suite`, `list_cases`, `version`.
Responses are JSON, e.g. the full `CaseResult` for `run_case`, a `SuiteReport`
for `run_suite`, `{"ids":[...]}` for `list_cases`.

The response is returned via a caller-owned buffer with a length-out protocol —
the callee never allocates memory the caller must free:

1. Call with `out = NULL`, `cap = 0` to learn the response length `len`
   (excluding the terminating NUL).
2. Allocate `len + 1` bytes and call again; the response plus a NUL is written.

Whenever `len < cap`, the response is written on that call, so a
sufficiently-large buffer needs only one call.

Return codes:

| Return   | Meaning                                              |
|----------|------------------------------------------------------|
| `>= 0`   | Response length in bytes (excluding the NUL).        |
| `-1`     | A required pointer (`handle` or `cmd_json`) is null. |
| `-2`     | `cmd_json` is not valid UTF-8.                        |
| `-3`     | A panic was caught at the boundary.                  |

Domain errors (a bad case, an unknown command) are **not** negative — they come
back in-band as `{"ok":false,"error":...}` JSON in the buffer.

### Header generation

`include/wickra_benchmark.h` is generated with [cbindgen] and committed; CI fails
if it drifts from the source. Regenerate after changing the ABI:

```sh
cbindgen --config cbindgen.toml --crate wickra-benchmark-c --output include/wickra_benchmark.h
```

## Benchmark

Every binding forwards to the same data-driven Rust core, so what this one adds is
the call overhead of the C ABI itself, not a different result. The core's throughput is
measured by the repository's benchmark suite and the nightly `bench.yml` run; the
numbers, the machine and how to reproduce them are in the repository
[BENCHMARKS.md](https://github.com/wickra-lib/wickra-benchmark/blob/main/BENCHMARKS.md).

## Documentation

The full guide, the spec reference and the API documentation live in the main
repository and the documentation site:

- **Repository:** <https://github.com/wickra-lib/wickra-benchmark>
- **Docs** (guides, spec reference, cookbook): <https://benchmark.wickra.org>
- **Runnable example:** [`examples/c/`](https://github.com/wickra-lib/wickra-benchmark/tree/main/examples/c)

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

[cbindgen]: https://github.com/mozilla/cbindgen
