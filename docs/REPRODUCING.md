# Reproducing the suite in any language

The whole point of this repository is that the same case produces the same result
everywhere. Every binding drives one JSON command envelope and returns the core's
canonical response verbatim, so a case's `passed`, `hash_match` and `blake3` hash
are byte-identical in Rust, Python, Node.js, WASM, C, C++, C#, Go, Java and R.

## The command envelope

`Benchmark::command_json` (and every binding's `command`) dispatches on `cmd`:

| `cmd`        | Payload             | Response |
|--------------|---------------------|----------|
| `run_case`   | `{case, data}`      | a `CaseResult` |
| `run_suite`  | `{suite, datasets}` | a `SuiteReport` |
| `list_cases` | `{suite}`           | `{ids:[...]}` (sorted) |
| `version`    | —                   | `{version, engine_version}` |

- `case` is a `BenchmarkCase`; `data` is the candle array for its dataset.
- `suite` is a `Suite`; `datasets` maps each `dataset_ref` to its candle array.
- Domain errors (a bad case, an unknown command) come back **in-band** as
  `{"ok":false,"error":...}` — only null/UTF-8/panic conditions raise.

The response is a **canonical** JSON string (sorted keys, quantized floats). Byte
equality of that string across languages *is* the reproducibility guarantee.

## The CLI

```sh
cargo build --release -p wickra-benchmark
./target/release/wickra-benchmark run-suite --suite cases/suite.json --data-root datasets
```

Exit `0` when every case reproduced, `1` when any case failed — so a
non-reproducible engine turns a CI build red.

## In each language

One runnable example per language lives under [`examples/`](../examples); each
loads the `sma-crossover-01` case and its dataset, drives `run_case`, and asserts
`passed && hash_match`. Run them per the table in
[`examples/README.md`](../examples/README.md). The C-ABI consumers (C/C++, C#, Go,
Java, R) need the C ABI library first: `cargo build --release -p wickra-benchmark-c`.

## The golden harness

[`golden/`](../golden) holds one blessed command envelope and its byte-exact
canonical response per command. Every binding's golden test walks up to find
`golden/`, replays each `commands/*.json`, and asserts the response equals
`expected/<name>.json`. Because the response is the core's canonical string
verbatim, byte equality across languages is the exact cross-language parity check.
See [`golden/README.md`](../golden/README.md).
