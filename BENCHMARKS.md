# Benchmarks

Micro-benchmarks for `benchmark-core`, measured with
[criterion](https://github.com/bheisler/criterion.rs) via the `benchmark-bench`
crate:

```bash
cargo bench -p benchmark-bench
```

The benches cover the three hot paths: `run_case` (recompute a single case's
report), `run_suite` (the full curated suite, parallel and sequential), and the
`canonicalize` + `blake3` hashing step in isolation.

Numbers land here once the core is built and the suite is blessed. They are
indicative micro-benchmarks on one machine, not a cross-engine comparison — the
product's value is reproducibility, not raw speed.
