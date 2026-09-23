<p align="center">
  <a href="https://wickra.org"><img src="https://raw.githubusercontent.com/wickra-lib/.github/main/profile/wickra-banner.webp?v=514-7" alt="Wickra Benchmark — a reproducible, golden-verified benchmark suite for quant backtests, recomputable byte-for-byte in ten languages" width="100%"></a>
</p>

[![CI](https://raw.githubusercontent.com/wickra-lib/.github/main/profile/badges/wickra-benchmark/ci.svg)](https://github.com/wickra-lib/wickra-benchmark/actions/workflows/ci.yml)
[![codecov](https://raw.githubusercontent.com/wickra-lib/.github/main/profile/badges/wickra-benchmark/codecov.svg)](https://codecov.io/gh/wickra-lib/wickra-benchmark)
[![Maven Central](https://raw.githubusercontent.com/wickra-lib/.github/main/profile/badges/wickra-benchmark/maven.svg)](https://central.sonatype.com/artifact/org.wickra/wickra-benchmark)
[![License: MIT OR Apache-2.0](https://raw.githubusercontent.com/wickra-lib/.github/main/profile/badges/wickra-benchmark/license.svg)](https://github.com/wickra-lib/wickra-benchmark#license)

# Wickra Benchmark — Java

---

**Part of the [Wickra ecosystem](https://github.com/wickra-lib) — for Java. `org.wickra:wickra-benchmark` — prebuilt native library inside the jar, no JNI, no system dependencies.**

Recompute a curated benchmark case or suite with the deterministic Wickra engine
and confirm its report and hash, on the JVM over the Wickra C ABI via the Foreign
Function & Memory API (FFM/Panama, JDK 22+).

## Requirements

- **Java 22 or later** (the FFM API is final since Java 22; no preview flag).
- The FFM API is *restricted*: pass `--enable-native-access=ALL-UNNAMED` when you
  run your application to silence the native-access warning.

## Install

Maven:

```xml
<dependency>
  <groupId>org.wickra</groupId>
  <artifactId>wickra-benchmark</artifactId>
  <version>0.1.4</version>
</dependency>
```

Gradle:

```kotlin
implementation("org.wickra:wickra-benchmark:0.1.4")
```

The native library ships prebuilt per platform inside the jar and is
extracted automatically on first use. There is nothing to compile.

## Quick start

Everything goes through a `Benchmark` driven by JSON commands — the same command
protocol every Wickra binding shares.

```java
import org.wickra.benchmark.Benchmark;

try (Benchmark bench = new Benchmark()) {
    String runCase = "{"
        + "\"cmd\":\"run_case\",\"case\":{"
        + "\"id\":\"sma-crossover-01\","
        + "\"strategy\":" + strategySpec + ","          // a wickra-backtest StrategySpec
        + "\"dataset_ref\":\"sma-uptrend.csv\","
        + "\"expected\":" + expectedReport + ","
        + "\"expected_hash\":\"" + expectedHash + "\"},"
        + "\"data\":" + candles
        + "}";
    String result = bench.command(runCase);
    System.out.println(result); // the full CaseResult as JSON
}
```

FFM needs native access enabled at runtime:

```sh
java --enable-native-access=ALL-UNNAMED ...
```

Point the loader at the native C ABI library with the `native.lib.dir` system
property (or place it on the library path). Build it with
`cargo build -p wickra-benchmark-c`.

### Commands

| `cmd`         | Payload             | Response                                |
|---------------|---------------------|-----------------------------------------|
| `run_case`    | `{case, data}`      | the full `CaseResult`                   |
| `run_suite`   | `{suite, datasets}` | a `SuiteReport`                         |
| `list_cases`  | `{suite}`           | `{ids:[...]}` (sorted)                  |
| `version`     | —                   | `{version:...,engine_version:...}`      |

Domain errors (a bad case, an unknown command) come back in-band as
`{"ok":false,"error":...}`; only null/UTF-8/panic conditions throw.

## Benchmark

Every binding forwards to the same data-driven Rust core, so what this one adds is
the call overhead of the Java Foreign Function & Memory API over the C ABI, not a different result. The core's throughput is
measured by the repository's benchmark suite and the nightly `bench.yml` run; the
numbers, the machine and how to reproduce them are in the repository
[BENCHMARKS.md](https://github.com/wickra-lib/wickra-benchmark/blob/main/BENCHMARKS.md).

## Documentation

The full guide, the spec reference and the API documentation live in the main
repository and the documentation site:

- **Repository:** <https://github.com/wickra-lib/wickra-benchmark>
- **Docs** (guides, spec reference, cookbook): <https://benchmark.wickra.org>
- **Runnable example:** [`examples/java/`](https://github.com/wickra-lib/wickra-benchmark/tree/main/examples/java)

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
