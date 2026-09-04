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
- `scripts/check_corpus_sync.py`, wired into CI: holds the four committed copies
  of a case — `cases/`, `cases/suite.json`, the `golden/` mirror and the runnable
  copy under `examples/data/` — plus every dataset copy, to the same bytes.
- The five missing issue templates (detailed bug report, detailed feature
  request, performance regression, documentation, question) and the long-form
  pull-request template, referenced from the short one so it is reachable —
  GitHub offers no picker.
- `LICENSES/MIT.txt`, `LICENSES/Apache-2.0.txt`, `docs/README.md` and
  `bindings/csharp/README.md`.
- README: a runnable quickstart above the fold, `Building everything from
  source` with the per-binding commands, `Testing` with what each layer actually
  covers, and `Ecosystem`.
- Dependabot entries for `fuzz/`, `examples/node/` and `examples/go/`, whose
  manifests had no watcher.

### Changed

- The backtest engine is consumed from crates.io (`wickra-backtest-core 0.1.4`)
  instead of git. A git dependency makes the whole workspace unpublishable, and
  the engine has been on crates.io since 0.1.2. The bump re-shapes every report
  (`symbol` and `timeframe` are new fields), so the corpus is re-blessed with it.
  `fuzz/` detaches from the workspace and resolves the engine on its own, so it
  names the same source — otherwise the graph carries two `wickra-backtest-core`
  crates whose types do not interoperate and the targets stop compiling.
- Blessing is one step and writes every copy of the corpus from the same value:
  `WICKRA_BLESS=1 cargo test -p benchmark-core --test golden`. Objects land with
  their keys sorted, so an authored `strategy` is normalised on the way in.

### Fixed

- `examples/data/cases/sma-crossover-01.json` kept a stale `expected_hash` after
  an earlier engine bump re-blessed `cases/` and `golden/` but not the runnable
  copy. The C and C++ examples were the only jobs that noticed, failing with
  `the case did not reproduce`.
- The NuGet Dependabot entry pointed at `bindings/csharp/WickraVerify.Tests`, a
  path from a sibling repository that does not exist here, so it silently
  matched nothing — the repository has never received a NuGet update PR. It now
  points at `WickraBenchmark.Tests`.
- Four documents and two config files described a different product: the bug
  report, pull request, `GOVERNANCE` and `SUPPORT` templates asked for a
  `ScanSpec` and a sample universe (`wickra-screener` concepts), `clippy.toml`
  allowed `ScanSpec` as a doc identifier, and `deny.toml` excepted
  `webpki-roots` for a TLS stack this crate does not have — it is not in the
  dependency graph at all.

[Unreleased]: https://github.com/wickra-lib/wickra-benchmark/commits/main
