# Wickra Benchmark — Python

Recompute a curated benchmark case or suite with the deterministic Wickra engine
and confirm its report and hash, byte-for-byte. The value is the curated,
hash-pinned suite of `(strategy, dataset, expected report)` cases — the "ImageNet
for trading-strategy reproducibility".

## Install

```sh
pip install wickra-benchmark
```

## Usage

Everything goes through a `Benchmark` driven by JSON commands — the same command
protocol every Wickra binding shares, so this Python front-end drives the exact
same core as the native CLI.

```python
import json
from wickra_benchmark import Benchmark

bench = Benchmark()  # stateless: the case, suite and data arrive with each command

case = {
    "id": "sma-crossover-01",
    "description": "SMA crossover on a deterministic uptrend",
    "strategy": {...},                       # a wickra-backtest StrategySpec
    "dataset_ref": "sma-uptrend.csv",
    "expected": {...},                       # the frozen BacktestReport
    "expected_hash": "b3aa...",              # blake3 of its canonical form
}
candles = [{"time": ..., "open": ..., "high": ..., "low": ..., "close": ..., "volume": ...}, ...]

result = json.loads(bench.command(json.dumps({"cmd": "run_case", "case": case, "data": candles})))
print("passed" if result["passed"] and result["hash_match"] else "failed")
```

## Commands

| `cmd`         | Payload                          | Response                                   |
|---------------|----------------------------------|--------------------------------------------|
| `run_case`    | `{case, data}`                   | the full `CaseResult`                      |
| `run_suite`   | `{suite, datasets}`              | a `SuiteReport`                            |
| `list_cases`  | `{suite}`                        | `{"ids":[...]}` (sorted)                   |
| `version`     | —                                | `{"version":...,"engine_version":...}`     |

`data` is a list of candles; `datasets` maps each `dataset_ref` to its candle
list (the FFI boundary has no filesystem, so datasets are supplied inline).

Domain errors (a bad case, an unknown command) come back in-band as
`{"ok":false,"error":...}`. A malformed command envelope raises `ValueError`.

## License

MIT OR Apache-2.0.
