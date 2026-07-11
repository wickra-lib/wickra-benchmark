# Threat model

`wickra-benchmark` is a reproducibility tool. It places no orders, holds no
credentials, and opens no network sockets. It reads curated case JSON and candle
CSV, recomputes a backtest report with the pinned `wickra-backtest` engine,
hashes it with the `wickra-proof` canonicalizer, and compares against a frozen
expectation. The interesting surface is therefore the parsing of untrusted case
and dataset input, and the integrity of the determinism guarantee.

## Assets

- **The curated registry** (`cases/`, `datasets/`) — the value of the product.
  Its authority rests on every case being reproducible: the frozen `expected`
  report and `expected_hash` must be exactly what the pinned engine produces.
- **The determinism guarantee** — that a suite report is byte-identical across
  ten languages and between the parallel and sequential runners. A silent
  divergence would let a non-conforming engine appear conforming.

## Actors

- **A case author** submitting a `(strategy, dataset, expected report, hash)`
  tuple. The expected report and hash are recomputed by CI from the real engine,
  so a hand-doctored expectation cannot be merged — the recomputation is the gate.
- **A consumer** running the suite against their own engine to check byte
  compatibility with the reference.

## Threats and mitigations

- **A doctored expectation** (an `expected` report or `expected_hash` that the
  engine would not actually produce). Mitigation: the golden corpus is blessed by
  a committed tool running the real engine, and CI re-derives it; a case whose
  `expected`/`expected_hash` disagree with the fresh recomputation fails.
- **A malformed case or dataset causing a panic across the FFI boundary.**
  Mitigation: every parse path returns a typed `Error` surfaced in-band as
  `{"ok":false,"error":…}`; the C ABI wraps each `extern "C"` entry point in
  `catch_unwind` and the release profile is `panic = "abort"`. Fuzz targets over
  case parsing, running and hashing assert no panic.
- **A determinism regression** (a `HashMap` iteration order, an unstable result
  sort, a re-formatted float). Mitigation: `BTreeMap` in every output path,
  results sorted by `id`, floats passed through verbatim, and a cross-language
  golden plus a parallel-equals-sequential test that fail the build on any
  divergence.
- **A supply-chain compromise** in a dependency. Mitigation: `cargo-deny`
  (advisories, licenses, bans, sources), SHA-pinned Actions, OSV scanning, and
  hash-pinned CI requirements.

## Non-goals

- Not an authentication or authorization system; the registry's trust comes from
  reproducibility, not identity.
- Not a substitute for auditing the `wickra-backtest` engine itself; benchmark
  surfaces engine divergence, it does not certify the engine's correctness.
