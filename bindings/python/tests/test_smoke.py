"""Smoke test: construct a benchmark, run a case, parse the result."""

import json
import math

from wickra_benchmark import Benchmark, __version__

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

ZERO_HASH = "0" * 64


def _candles() -> list[dict]:
    out = []
    for i in range(40):
        base = 100.0 + math.sin(i * 0.4) * 8.0
        out.append(
            {
                "time": 1_700_000_000 + i * 3600,
                "open": base,
                "high": base + 1.0,
                "low": base - 1.0,
                "close": base + 0.5,
                "volume": 1000.0,
            }
        )
    return out


def _case(dataset_ref: str = "d.csv") -> dict:
    # A placeholder expected/hash: the run still recomputes the real report; the
    # smoke test only checks the shape of the response, not that it passes.
    return {
        "id": "sma-crossover-01",
        "description": "smoke",
        "strategy": STRATEGY,
        "dataset_ref": dataset_ref,
        "expected": {},
        "expected_hash": ZERO_HASH,
    }


def test_run_case_shape() -> None:
    bench = Benchmark()
    req = json.dumps({"cmd": "run_case", "case": _case(), "data": _candles()})
    result = json.loads(bench.command(req))
    assert result["id"] == "sma-crossover-01"
    assert result["passed"] is False  # placeholder expected does not match
    assert result["hash_match"] is False
    assert len(result["hash"]) == 64
    assert "recomputed" in result


def test_run_suite_and_list_cases() -> None:
    bench = Benchmark()
    suite = {"name": "smoke", "cases": [_case()]}
    run = json.dumps({"cmd": "run_suite", "suite": suite, "datasets": {"d.csv": _candles()}})
    report = json.loads(bench.command(run))
    assert report["failed"] == 1
    assert len(report["results"]) == 1

    listed = json.loads(bench.command(json.dumps({"cmd": "list_cases", "suite": suite})))
    assert listed["ids"] == ["sma-crossover-01"]


def test_version_matches_module() -> None:
    assert Benchmark.version() == __version__
    reported = json.loads(Benchmark().command('{"cmd":"version"}'))
    assert reported["version"] == __version__


def test_unknown_command_is_in_band_error() -> None:
    bench = Benchmark()
    response = json.loads(bench.command('{"cmd":"nope"}'))
    assert response["ok"] is False
    assert "nope" in response["error"]
