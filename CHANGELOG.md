# Changelog

All notable changes to this project are documented here. The format is based on
[Keep a Changelog](https://keepachangelog.com/en/1.1.0/), and this project
adheres to [Semantic Versioning](https://semver.org/spec/v2.0.0.html).

## [Unreleased]

### Added

- `benchmark-core`: `BenchmarkCase`, `Suite`, `CaseResult`, `SuiteReport`, the
  `run_case` / `run_suite` / `run_suite_inline` runner, blake3 canonical hashing
  (shared with `wickra-proof`), and the `command_json` boundary.
- Reference CLI (`wickra-benchmark`): `run-case`, `run-suite`, `list-cases`,
  text or JSON output.
- Curated registry: deterministic candle datasets under `datasets/` and
  golden-verified cases under `cases/`, with a hash-pinned dataset manifest.
- Ten-language bindings (native Python/Node/WASM + a C ABI hub for
  C/C++/C#/Go/Java/R), each returning the core's canonical JSON verbatim.
- Byte-exact golden corpus, conformance / determinism / property tests, fuzz
  targets, per-language examples, and the full cross-OS CI matrix.

[Unreleased]: https://github.com/wickra-lib/wickra-benchmark/commits/main
