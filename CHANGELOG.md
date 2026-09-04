# Changelog

All notable changes to this project are documented here. The format is based on
[Keep a Changelog](https://keepachangelog.com/en/1.1.0/), and this project
adheres to [Semantic Versioning](https://semver.org/spec/v2.0.0.html).

## [Unreleased]

### Changed

- The backtest engine is consumed from crates.io (`wickra-backtest-core 0.1.4`)
  instead of git. A git dependency makes the whole workspace unpublishable, and
  the engine has been on crates.io since 0.1.2. The bump re-shapes every report
  (`symbol` and `timeframe` are new fields), so the corpus is re-blessed with it.
- Blessing is one step and writes every copy of the corpus from the same value:
  `WICKRA_BLESS=1 cargo test -p benchmark-core --test golden`. Objects land with
  their keys sorted, so an authored `strategy` is normalised on the way in.

### Fixed

- `examples/data/cases/sma-crossover-01.json` kept a stale `expected_hash` after
  an earlier engine bump re-blessed `cases/` and `golden/` but not the runnable
  copy. The C and C++ examples were the only jobs that noticed, failing with
  `the case did not reproduce`.

### Added

- `scripts/check_corpus_sync.py`, wired into CI: holds the four committed copies
  of a case — `cases/`, `cases/suite.json`, the `golden/` mirror and the runnable
  copy under `examples/data/` — plus every dataset copy, to the same bytes.

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
