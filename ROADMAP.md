# Roadmap

`wickra-benchmark` targets full wickra-grade parity with its sibling products
(`wickra-backtest` / `wickra-proof` / `wickra-screener`): the same versions, the
same structure, the same tests / fuzz / golden / examples / bindings / CI.

## Pre-1.0 (0.1.x)

- [x] Repository scaffold, governance, supply-chain and licensing baseline.
- [ ] `benchmark-core`: `BenchmarkCase`, `Suite`, `CaseResult`, `SuiteReport`,
      canonical blake3 hashing, the `run_case` / `run_suite` runner, and the
      `command_json` boundary.
- [ ] Reference CLI (`wickra-benchmark`): `run-case`, `run-suite`, `list-cases`,
      text or JSON output.
- [ ] A curated, hash-pinned registry: deterministic datasets and
      golden-verified cases, small enough to recompute by hand.
- [ ] Ten language bindings over the JSON-over-C-ABI boundary -- native Rust,
      Python, Node.js, WASM, plus a C ABI hub for C, C++, C#, Go, Java, R.
- [ ] Byte-exact golden corpus, conformance / determinism / property / fuzz
      tests, benchmarks, one runnable example per language.
- [ ] CI across all ten languages on three OSes; CodeQL, Scorecard, zizmor.

## Later

- Grow the case registry to cover more strategy families and market regimes.
- First release to the language registries (USER-GO gated).

Trading tooling only -- no financial advice; see the disclaimer in the README.
