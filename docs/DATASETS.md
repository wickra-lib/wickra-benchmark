# Datasets

Every dataset is a small, **deterministically generated** OHLCV candle series —
no market data, no randomness, no hidden state. Any machine, in any language, can
regenerate the exact bytes and therefore recompute the exact same reports and
hashes. This document explains the curation policy; the per-file formulas and the
regeneration note live in [`datasets/README.md`](../datasets/README.md).

## Format

Plain CSV, one header row, one candle per row, ascending `time`:

```
time,open,high,low,close,volume
```

- `time` — Unix seconds from `1700000000`, one 1h step (`3600`) per bar.
- `volume` — a constant `1000.0` (volume is not exercised by these strategies).
- Line endings are **LF**, pinned in `.gitattributes` (`*.csv text eol=lf`) so a
  Windows checkout cannot rewrite them to CRLF and change the digest.

## Generation

Each file is a closed-form price path. Candles share one construction rule around
a per-file base price `base(i)` and wick half-width `w`:

```
close(i) = base(i)
open(i)  = base(i - 1)          # open(0) = base(0)
high(i)  = max(open, close) + w
low(i)   = min(open, close) - w
```

The per-file `base(i)` (trend, oscillation, breakout, flat) and `w` are tabulated
in [`datasets/README.md`](../datasets/README.md). `sin` is double-precision radians;
all arithmetic is IEEE-754 `f64`, and each value is written with the shortest
round-trip decimal, so Rust's `{}` and Python's `repr` agree.

The datasets are chosen so each case trades non-trivially and produces a distinct
report — a geometric price path, for example, has constant log-returns and would
collapse returns-based metrics, so oscillating and trending paths are used instead.

## Integrity

[`datasets/MANIFEST.json`](../datasets/MANIFEST.json) maps each file name to the
lowercase `blake3` hex digest of its raw bytes:

```json
{ "<file>.csv": "<blake3-hex>", ... }
```

The `dataset-manifest` CI job (`.github/scripts/check-manifest.sh`) rehashes every
CSV, compares against the manifest, and fails on any drift or any untracked CSV.
This matters because a case hashes the report computed *over these candles*: a
single changed byte would silently change a case's expected hash, so the datasets
are frozen and guarded.

> **Never edit a dataset by hand.** If you need a new shape, add a new file with a
> new formula and record its digest in the manifest; never mutate an existing one.
