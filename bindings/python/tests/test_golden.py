"""Cross-language golden / determinism replay.

The determinism guarantee is that the same command yields byte-identical output
every time and in every language. Here we assert byte-stability of the response
in Python; the curated golden corpus (once blessed) is replayed by the workspace
integration tests and CI's cross-language golden step, all against the same
bytes.
"""

import json
import math

from wickra_benchmark import Benchmark

STRATEGY = {
    "symbol": "BTCUSDT",
    "timeframe": "1h",
    "indicators": {
        "ema_fast": {"type": "Ema", "params": [5]},
        "ema_slow": {"type": "Ema", "params": [15]},
    },
    "entry": {"cross_above": ["ema_fast", "ema_slow"]},
    "exit": {"cross_below": ["ema_fast", "ema_slow"]},
    "sizing": {"type": "fixed_fraction", "fraction": 0.95},
    "costs": {"taker_bps": 5, "slippage": {"type": "fixed_bps", "bps": 2}},
}


def _candles() -> list[dict]:
    return [
        {
            "time": 1_700_000_000 + i * 3600,
            "open": 100.0 + math.sin(i * 0.4) * 8.0,
            "high": 101.0 + math.sin(i * 0.4) * 8.0,
            "low": 99.0 + math.sin(i * 0.4) * 8.0,
            "close": 100.5 + math.sin(i * 0.4) * 8.0,
            "volume": 1000.0,
        }
        for i in range(40)
    ]


def _request() -> str:
    case = {
        "id": "sma-crossover-01",
        "description": "golden",
        "strategy": STRATEGY,
        "dataset_ref": "d.csv",
        "expected": {},
        "expected_hash": "0" * 64,
    }
    return json.dumps({"cmd": "run_case", "case": case, "data": _candles()})


def test_run_case_is_byte_stable() -> None:
    bench = Benchmark()
    req = _request()
    first = bench.command(req)
    second = bench.command(req)
    assert first == second


def test_version_is_byte_stable() -> None:
    bench = Benchmark()
    assert bench.command('{"cmd":"version"}') == bench.command('{"cmd":"version"}')
