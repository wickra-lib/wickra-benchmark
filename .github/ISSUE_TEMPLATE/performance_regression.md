---
name: Performance regression
about: Report a measurable slowdown, memory blowup or throughput drop.
title: "[Perf] <area> regressed in <version>"
labels: ["performance", "regression", "triage"]
assignees: []
---

## Summary

<!-- Which path got slower, by how much, and since when. -->

## Affected path

- Area: `e.g. run_suite, run_case, canonicalize, the dataset CSV loader`
- Binding: `Rust / Python / Node.js / WASM / C / C++ / C# / Go / Java / R`
- Runner: `parallel (default) / sequential (--no-default-features)`
- Suite size at which you saw it: `e.g. 1000 cases`

## Versions compared

| Version | Throughput / latency / memory | Notes |
| --- | --- | --- |
| `0.1.0` | `e.g. 109 ms / 1000 cases` | baseline |
| `0.1.1` | `e.g. 340 ms / 1000 cases` | regressed |

<!--
Say whether the engine version moved too. A case is recomputed with the pinned
`wickra-backtest-core`, so most of the per-case cost is the engine's, and an
engine bump can move these numbers without anything here changing.
-->

## Benchmark / reproducer

<!-- The command and its output. For a one-off measurement, include the timing snippet. -->

```bash
cargo bench -p benchmark-bench
```

```
```

## Hardware / environment

| Field | Value |
| --- | --- |
| CPU | `e.g. Ryzen 9 9950X` |
| Cores used | `e.g. 16 (rayon default)` |
| OS / arch | `e.g. Linux 6.8 x86_64` |
| Toolchain | `rustc 1.x.y` |

## Suspected cause

<!-- Optional. Link the commit or pull request if you bisected it. -->
