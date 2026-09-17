# Fuzzing Wickra Benchmark

[`cargo-fuzz`](https://rust-fuzz.github.io/book/cargo-fuzz.html) harnesses for the parsing and stateful entry points of Wickra Benchmark. Fuzzing requires a nightly Rust toolchain; CI runs every target for 30 seconds on the family's pinned `nightly-2026-07-01`.

## Setup

```bash
cargo install cargo-fuzz
rustup toolchain install nightly-2026-07-01
```

The date is the family's fuzz nightly, pinned in `ci.yml`: a rolling `nightly`
regressed with a codegen ICE unrelated to this code, so every repository moves
the date together, on purpose.

## Targets

| Target | What it exercises |
| --- | --- |
| `case_parse` | The case parsing surface: arbitrary bytes are parsed as a `BenchmarkCase` from both JSON and TOML. |
| `suite_parse` | The suite parsing surface: arbitrary bytes are parsed as a `Suite` from both JSON and TOML. |
| `run_case` | The run contract with genuine inputs. |
| `command_json` | The FFI command boundary — the surface every language binding forwards verbatim. |

## Run

```bash
# From the repository root:
cargo +nightly-2026-07-01 fuzz run --target x86_64-unknown-linux-gnu case_parse
cargo +nightly-2026-07-01 fuzz run --target x86_64-unknown-linux-gnu suite_parse
cargo +nightly-2026-07-01 fuzz run --target x86_64-unknown-linux-gnu run_case
cargo +nightly-2026-07-01 fuzz run --target x86_64-unknown-linux-gnu command_json
```

Each run continues until a crash is found or it is interrupted. A short
time-boxed smoke run is what CI does:

```bash
cargo +nightly-2026-07-01 fuzz run --target x86_64-unknown-linux-gnu case_parse -- -max_total_time=30
```

The expectation for every target is that it never panics: malformed or
adversarial input must surface as an `Err` or an in-band error, never a crash.
