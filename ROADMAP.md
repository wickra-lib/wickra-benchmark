# Roadmap

`wickra-benchmark` targets full wickra-grade parity with its sibling products
(`wickra-backtest` / `wickra-proof` / `wickra-screener`): the same versions, the
same structure, the same tests / fuzz / golden / examples / bindings / CI.

## Pre-1.0 (0.1.x)

- [x] Repository scaffold, governance, supply-chain and licensing baseline.
- [x] `benchmark-core`: `BenchmarkCase`, `Suite`, `CaseResult`, `SuiteReport`,
      canonical blake3 hashing, the `run_case` / `run_suite` runner, and the
      `command_json` boundary.
- [x] Reference CLI (`wickra-benchmark`): `run-case`, `run-suite`, `list-cases`,
      text or JSON output.
- [x] A curated, hash-pinned registry: deterministic datasets and
      golden-verified cases, small enough to recompute by hand.
- [x] Ten language bindings over the JSON-over-C-ABI boundary -- native Rust,
      Python, Node.js, WASM, plus a C ABI hub for C, C++, C#, Go, Java, R.
- [x] Byte-exact golden corpus, conformance / determinism / property / fuzz
      tests, benchmarks, one runnable example per language.
- [x] CI across all ten languages on three OSes; CodeQL, Scorecard, zizmor.

## Before the first release

- [ ] Grow the case registry. It currently holds **five** cases over five
      deterministic datasets of 60-80 bars each. That is enough to prove the
      machinery reproduces and not enough to call a benchmark suite: no regime
      variety, no long series, no strategy family beyond crossovers and a
      breakout.
- [ ] A `wickra-benchmark-site` repository. Every sibling product has one;
      `benchmark.wickra.org` does not resolve, so `docs/README.md` points at the
      binding READMEs and docs.rs instead of a site.
- [ ] An entry in `wickra-lib.r-universe.dev/packages.json`, so the R package is
      built and installable from the registry.

## Later

- Cross-engine cases: the same `(strategy, dataset)` recomputed by more than one
  engine, which is where a benchmark suite stops describing this engine and
  starts comparing them.
- First release to the language registries (USER-GO gated).

Trading tooling only -- no financial advice; see the disclaimer in the README.
