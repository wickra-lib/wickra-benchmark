# Benchmarks

Micro-benchmarks for `benchmark-core`, measured with
[criterion](https://github.com/bheisler/criterion.rs) via the `benchmark-bench`
crate:

```bash
cargo bench -p benchmark-bench                         # parallel (rayon) runner
cargo bench -p benchmark-bench --no-default-features   # sequential runner
```

The bench measures `run_suite` — the whole product path: recompute every case's
report with the pinned engine, canonicalize it, `blake3`-hash it, and compare
against the frozen expectation — across suite sizes of 10, 100 and 1000 blessed
cases. Each case runs a real EMA-cross strategy over a 128-bar universe, so the
numbers reflect production work, not a synthetic shape.

## Results

Indicative single-machine numbers (parallel runner; criterion median of 100
samples). They measure reproducibility throughput, not a cross-engine speed
comparison — the product's value is byte-identical reproducibility, not raw
speed.

| Suite size | `run_suite` (median) | Throughput |
|-----------:|---------------------:|-----------:|
| 10 cases   | ~1.27 ms             | ~7,900 cases/s |
| 100 cases  | ~11.5 ms             | ~8,700 cases/s |
| 1000 cases | ~109 ms              | ~9,200 cases/s |

Throughput is roughly flat with suite size — each case is an independent
recompute-and-hash, so the parallel runner scales with cases while the per-case
cost (a full backtest over 128 bars plus canonicalization and a blake3 digest)
stays constant.

The reports are byte-identical between the parallel and sequential runners (the
results are re-sorted by case id before tallying), so `--no-default-features`
measures only the scheduling difference, not a different result.

## Method

- Machine and OS vary; treat the absolute numbers as indicative and re-run
  locally for your hardware.
- The nightly `bench.yml` workflow re-runs this on a schedule and uploads the
  report as a CI artifact.
