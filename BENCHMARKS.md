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

Measured on a Ryzen 9 9950X (Windows, 16 cores), parallel runner, criterion
median, against `wickra-backtest-core` 0.1.4. They measure reproducibility
throughput, not a cross-engine speed comparison — the product's value is
byte-identical reproducibility, not raw speed.

| Suite size | `run_suite` (median) | Throughput |
|-----------:|---------------------:|-----------:|
| 10 cases   | 778 µs               | ~12,900 cases/s |
| 100 cases  | 4.26 ms              | ~23,500 cases/s |
| 1000 cases | 38.0 ms              | ~26,300 cases/s |

Throughput roughly doubles from 10 cases to 1000, then flattens. Each case is an
independent recompute-and-hash, so there is nothing to share between them and no
speed-up to be had from batching itself — what grows is how well the work fills
the cores. At ten cases rayon's own setup is a visible fraction of a run that
takes under a millisecond; by a thousand it is not, and the curve levels off
where the cores are saturated rather than where the per-case cost changes. That
cost — a full backtest over 128 bars, canonicalization, and a blake3 digest — is
constant throughout.

The reports are byte-identical between the parallel and sequential runners (the
results are re-sorted by case id before tallying), so `--no-default-features`
measures only the scheduling difference, not a different result.

## Method

- Machine and OS vary; treat the absolute numbers as indicative and re-run
  locally for your hardware. The shape is the durable part: flat per-case cost,
  throughput rising with suite size until the cores fill.
- The engine version moves these numbers. Most of the per-case cost is
  `wickra-backtest-core`'s, so a bump there can shift the whole table without
  anything in this repository changing. The version each measurement was taken
  against is named above for that reason.
- The nightly `bench.yml` workflow re-runs this on a schedule and uploads the
  report as a CI artifact.
