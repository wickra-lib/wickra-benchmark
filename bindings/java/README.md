# Wickra Benchmark — Java

Recompute a curated benchmark case or suite with the deterministic Wickra engine
and confirm its report and hash, on the JVM over the Wickra C ABI via the Foreign
Function & Memory API (FFM/Panama, JDK 22+).

## Usage

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

## Commands

| `cmd`         | Payload             | Response                                |
|---------------|---------------------|-----------------------------------------|
| `run_case`    | `{case, data}`      | the full `CaseResult`                   |
| `run_suite`   | `{suite, datasets}` | a `SuiteReport`                         |
| `list_cases`  | `{suite}`           | `{ids:[...]}` (sorted)                  |
| `version`     | —                   | `{version:...,engine_version:...}`      |

Domain errors (a bad case, an unknown command) come back in-band as
`{"ok":false,"error":...}`; only null/UTF-8/panic conditions throw.

## License

MIT OR Apache-2.0.
