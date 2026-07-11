# Benchmark datasets

Small, synthetic OHLCV candle series that the benchmark cases replay. Each file
is **deterministically generated** from a closed-form price path — there is no
market data here, no randomness, and no hidden state. The point is that any
machine, in any language, can regenerate these exact bytes and therefore
recompute the exact same strategy reports and hashes.

> **Do not edit these files by hand.** A single changed byte changes the CSV's
> `blake3` digest in [`MANIFEST.json`](MANIFEST.json), and — because a benchmark
> case hashes the report computed *over these candles* — every dependent case
> hash as well. If you need a new shape, add a new file with a new formula; never
> mutate an existing one. CI recomputes each digest and fails on any drift.

## Format

Plain CSV, one header row, one candle per row, sorted by ascending `time`:

```
time,open,high,low,close,volume
```

- `time` — Unix seconds, starting at `1700000000`, one 1h step (`3600`) per bar.
- `volume` — a constant `1000.0` for every bar (volume is not exercised by these
  strategies).
- Line endings are **LF**, pinned in `.gitattributes` (`*.csv text eol=lf`) so a
  Windows checkout cannot silently rewrite them to CRLF and change the digest.

## Generation

Every file shares the same candle-construction rule around a per-file base
price `base(i)` for bar index `i` (`0`-based) and a per-file wick half-width `w`:

```
close(i) = base(i)
open(i)  = base(i - 1)          # open equals the previous bar's base; open(0) = base(0)
high(i)  = max(open, close) + w
low(i)   = min(open, close) - w
```

| File                    | Bars | `w`  | `base(i)`                                   | Shape |
|-------------------------|------|------|---------------------------------------------|-------|
| `sma-uptrend.csv`       | 80   | 1.0  | `100 + 0.3*i + 8*sin(0.40*i)`               | Rising trend with pullbacks — SMA fast/slow crossovers. |
| `mean-revert-range.csv` | 80   | 1.0  | `100 + 15*sin(0.20*i)`                       | Wide oscillation around 100 — RSI dips below 30 and recovers. |
| `ema-trend.csv`         | 80   | 1.0  | `100 + 0.4*i + 6*sin(0.55*i)`               | Steady trend with fast ripple — EMA fast/slow crossovers. |
| `breakout.csv`          | 80   | 0.5  | `100.0` for `i < 40`, else `100.0 + 2.0*(i-40)` | Flat base then a clean breakout — Donchian channel break. |
| `flat.csv`              | 60   | 0.2  | `100 + 0.02*i`                              | Near-flat drift — a trivially-always-in-market buy-and-hold. |

`sin` is the standard double-precision sine in radians. All arithmetic is IEEE-754
`f64`; the CSV writes each value with the shortest round-trip decimal (Python's
`repr`, Rust's `{}`) — the two agree because both emit the shortest string that
round-trips to the same `f64`.

## Integrity

[`MANIFEST.json`](MANIFEST.json) maps each file name to the lowercase
`blake3` hex digest of its raw bytes:

```json
{ "<file>.csv": "<blake3-hex>", ... }
```

The `dataset-manifest` CI job rehashes every CSV and compares against this
manifest, so a stray edit — or a mis-generated file — fails the build before it
can poison a case hash.
