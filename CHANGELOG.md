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
- `actionlint` workflow (with shellcheck over every `run:` block) and a CodSpeed
  workflow that measures the Criterion benches under instruction counting and
  attributes a regression to the pull request that caused it.
- `.github/codeql/codeql-config.yml`, and C#, Java, Go and C/C++ in the CodeQL
  matrix — the four compiled surfaces, each built for real, since `build-mode:
  none` resolves no dependencies and GitHub then reports the analysis as low
  quality.
- `bindings/python/tests/run_without_pytest.py`, which runs the whole Python
  suite without a framework.
- Four check scripts and the `binding-surface` job that runs them:
  `check_binding_surface.py` holds all ten language surfaces to the C ABI header,
  `check_version_sync.py` holds 22 version declarations across six package
  managers to each other, `check_readme_links.py` catches a repository-relative
  link in a README that ships as a package long description, and
  `check_license_copies.py` catches a published package that would ship without
  its licence texts. Plus `scripts/update-lockfiles.sh` to regenerate every
  committed lockfile.
- CI jobs `links` (non-blocking lychee on pull requests), `semver`
  (cargo-semver-checks against the last release) and
  `python-wheel-container-smoke`, which builds the manylinux and musllinux
  wheels and imports each one — the musl one inside an alpine container, since
  the runner is glibc and would otherwise prove nothing about the artefact the
  release publishes.
- Licence texts beside every published package (`benchmark-core`,
  `benchmark-cli`, `bindings/python`).

### Changed

- The backtest engine is consumed from crates.io (`wickra-backtest-core 0.1.4`)
  instead of git. A git dependency makes the whole workspace unpublishable, and
  the engine has been on crates.io since 0.1.2. The bump re-shapes every report
  (`symbol` and `timeframe` are new fields), so the corpus is re-blessed with it.
  `fuzz/` detaches from the workspace and resolves the engine on its own, so it
  names the same source — otherwise the graph carries two `wickra-backtest-core`
  crates whose types do not interoperate and the targets stop compiling.
- Every workflow job declares `timeout-minutes` (20 of them did not), and every
  action pin carries a patch-level comment. `Swatinem/rust-cache` was pinned to a
  commit that is not any v2.x release tag, which is why its comment could only
  say `v2`; it now points at the tagged 2.9.2.
- `ci.yml` filters `pull_request` to `main`, so a pull request against another
  branch no longer builds the whole matrix twice.
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
- `.github/requirements/ci-dev.txt` was hash-locked, committed, and read by
  nothing: the Python job installed `maturin pytest` unpinned instead. It could
  not have been wired as it stood — it pinned pytest 9.1.1, which requires
  Python 3.10, while the matrix includes 3.9. It is now split into
  `ci-dev-py39.txt` (no pytest at all) and `ci-dev-py3.txt`, both installed with
  `--require-hashes`. No module in `bindings/python/tests` imports pytest, so
  the 3.9 row runs the same eight tests through `run_without_pytest.py` rather
  than pinning a pytest 8.x below the fix for GHSA-6w46-j5rx-g56g.
- `osv-scanner.toml` was committed configuration that no workflow ever loaded.
  `cargo deny` only sees the Rust graph; osv-scanner reads every other lockfile
  in the tree — npm, pip, maven, nuget, go — which is where the other nine
  language surfaces get their dependencies.
- C# was the only binding with two READMEs: a developer-facing one and a nested
  package one, with the csproj packing the nested copy. Every other binding has
  exactly one at `bindings/<lang>/README.md` that serves both. Consolidated to
  match, and verified with `dotnet pack` that the surviving file is what lands
  in the `.nupkg`.
- `examples/node/package-lock.json` recorded `@napi-rs/cli ^3.7.2` while the
  binding it links declares `^3.7.4`: a Dependabot bump reached the binding and
  not the example's lockfile, and nothing watched it.
- The dataset-manifest job installed `blake3` unpinned. That job is what stands
  between an edited dataset and a silently changed case hash, so its checker now
  comes from a hash-locked `manifest.txt`.
- Four documents and two config files described a different product: the bug
  report, pull request, `GOVERNANCE` and `SUPPORT` templates asked for a
  `ScanSpec` and a sample universe (`wickra-screener` concepts), `clippy.toml`
  allowed `ScanSpec` as a doc identifier, and `deny.toml` excepted
  `webpki-roots` for a TLS stack this crate does not have — it is not in the
  dependency graph at all.

[Unreleased]: https://github.com/wickra-lib/wickra-benchmark/commits/main
