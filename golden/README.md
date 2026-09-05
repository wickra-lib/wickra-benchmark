# Cross-language golden fixtures

This directory is the **cross-language parity harness**. Every binding — Rust,
Python, Node.js, WASM, C, C++, C#, Go, Java, R — must turn a given command
envelope into the exact same canonical response, byte for byte. That is what
proves the hashes in this benchmark are the engine's, not one language's.

## Layout

| Path                | What it is |
|---------------------|------------|
| `commands/<name>.json` | A full command envelope (`{"cmd":...}`) fed verbatim to `command_json`. |
| `expected/<name>.json` | The canonical response the core returns for that command — the golden bytes. |
| `datasets/*.csv`       | A copy of the top-level [`datasets/`](../datasets/) the commands embed, so this directory is self-contained. |
| `suite.json`           | A copy of [`cases/suite.json`](../cases/suite.json). |

Each binding's golden test walks up to find this `golden/` directory, replays
every `commands/*.json` through its own `command`/`command_json` entry point, and
asserts the response equals `expected/<name>.json` (trailing whitespace ignored).
Because the response is the core's canonical string verbatim, byte equality *is*
the cross-language equality check — same sorted keys, same quantized floats, same
`blake3` hashes, in every language.

## The fixtures

| `<name>`                | Command      | What it exercises |
|-------------------------|--------------|-------------------|
| `sma-crossover-01`       | `run_case`   | SMA(10/30) crossover — recompute + hash match. |
| `rsi-mean-reversion-01`  | `run_case`   | RSI(14) mean reversion. |
| `ema-trend-follow-01`    | `run_case`   | EMA(5/15) trend following. |
| `breakout-channel-01`    | `run_case`   | Donchian(20) channel breakout. |
| `buy-and-hold-01`        | `run_case`   | Always-in-market baseline. |
| `macd-trend-01`          | `run_case`   | MACD(12/26/9) line over signal, 600 bars. |
| `bollinger-breakout-01`  | `run_case`   | Close above the upper band, out at the middle, 750 bars. |
| `atr-breakout-filter-01` | `run_case`   | A Donchian break gated on ATR — an `all` of two conditions. |
| `roc-momentum-01`        | `run_case`   | ROC(14) above zero and `rising`. |
| `wma-crossover-01`       | `run_case`   | WMA(10/30) crossover against a falling market. |
| `ema-whipsaw-01`         | `run_case`   | EMA(5/20) crossover with no trend to catch. |
| `rsi-short-downtrend-01` | `run_case`   | RSI with `short_entry`/`short_exit` — the short side. |
| `donchian-long-range-01` | `run_case`   | Donchian(40) over 750 bars of oscillation. |
| `sma-timed-exit-01`      | `run_case`   | An entry released by `bars_since_entry`, not by a signal. |
| `sma-long-trend-01`      | `run_case`   | SMA(20/60) over 600 bars, where warmup is a small part. |
| `suite-run`              | `run_suite`  | All fifteen cases over their datasets in one report. |
| `suite-list`             | `list_cases` | The sorted case ids of the suite. |
| `version`                | `version`    | The benchmark and engine versions. |

Every `run_case`/`run_suite` fixture is *blessed*: its `expected`/`expected_hash`
come from the engine itself, so each response has `passed: true` and, for
`run_suite`, `failed: 0`.

## Regenerating

The fixtures are derived, not authored. Rebuild them whenever the datasets,
cases, or the engine's report shape changes — from the repository root, drive
each command through any binding (they all return the same canonical bytes) and
write the result to `expected/<name>.json`. The core is the single source of
truth; never hand-edit an `expected` file. CI replays these in every language and
fails on the first byte of drift.
