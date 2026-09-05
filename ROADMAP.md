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

- [x] Grow the case registry. It holds **fifteen** cases over ten deterministic
      datasets: a downtrend, a volatility shock and a trendless chop alongside
      the original five, two long series of 600 and 750 bars, and families
      beyond crossovers and a breakout — MACD, Bollinger bands, ATR, rate of
      change and a weighted average, plus the short side, a time-based exit and
      a two-condition `all`.
- [x] A `wickra-benchmark-site` repository, at
      [wickra-benchmark-site](https://github.com/wickra-lib/wickra-benchmark-site).
- [ ] Point `benchmark.wickra.org` at it: the DNS record and the Cloudflare Pages
      project. Until both exist the site builds and serves nothing, so nothing
      here links to that host.
- [x] An entry in `wickra-lib.r-universe.dev/packages.json`. The registry tracks
      `*release`, so it builds nothing until the first tag -- which is the point:
      the entry has to exist before the tag, not after it.

## Later

- Cross-engine cases: the same `(strategy, dataset)` recomputed by more than one
  engine, which is where a benchmark suite stops describing this engine and
  starts comparing them.
- First release to the language registries (USER-GO gated).

Trading tooling only -- no financial advice; see the disclaimer in the README.
