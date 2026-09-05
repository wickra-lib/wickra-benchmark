"""Batch equivalence: a suite must produce exactly what the cases produce alone.

`run_suite` is the batch form of `run_case`. It fans the cases out -- over rayon
when the parallel feature is on -- and re-sorts the results by `id` before
tallying, so the two paths share an engine but not a control flow. Nothing else
holds them to the same answer.

Three things can break independently here and none of them would fail another
test in this directory: a case could pick up state from the one before it, the
per-case results could come back in scheduling order rather than sorted, and the
tally could disagree with the results it counted. The suite is also the form a
caller actually uses, so a divergence would surface as "the CLI says something
different from my script" rather than as a test failure.
"""

import json
import math

from wickra_benchmark import Benchmark

ZERO_HASH = "0" * 64


def _strategy(fast: int, slow: int) -> dict:
    return {
        "symbol": "BTCUSDT",
        "timeframe": "1h",
        "indicators": {
            "ema_fast": {"type": "Ema", "params": [fast]},
            "ema_slow": {"type": "Ema", "params": [slow]},
        },
        "entry": {"cross_above": ["ema_fast", "ema_slow"]},
        "exit": {"cross_below": ["ema_fast", "ema_slow"]},
        "sizing": {"type": "fixed_fraction", "fraction": 0.95},
        "costs": {"taker_bps": 5, "slippage": {"type": "fixed_bps", "bps": 2}},
    }


def _candles(seed: int) -> list[dict]:
    out = []
    for i in range(40):
        base = 100.0 + math.sin(i * 0.4 + seed) * 8.0
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


def _cases() -> list[dict]:
    # Deliberately out of id order, and each on its own dataset: a suite that
    # only ever sees sorted input cannot show that it sorts.
    return [
        {
            "id": f"case-{n:02d}",
            "description": "batch equivalence",
            "strategy": _strategy(3 + n, 12 + n),
            "dataset_ref": f"d{n}.csv",
            "expected": {},
            "expected_hash": ZERO_HASH,
        }
        for n in (3, 1, 2)
    ]


def test_suite_matches_the_cases_run_alone() -> None:
    bench = Benchmark()
    cases = _cases()
    datasets = {c["dataset_ref"]: _candles(i) for i, c in enumerate(cases)}

    alone = {}
    for case in cases:
        req = json.dumps(
            {"cmd": "run_case", "case": case, "data": datasets[case["dataset_ref"]]}
        )
        result = json.loads(bench.command(req))
        alone[result["id"]] = result

    batched = json.loads(
        bench.command(
            json.dumps(
                {
                    "cmd": "run_suite",
                    "suite": {"name": "batch", "cases": cases},
                    "datasets": datasets,
                }
            )
        )
    )

    results = batched["results"]
    assert [r["id"] for r in results] == sorted(alone), "the suite must sort by id"
    for result in results:
        assert result == alone[result["id"]]

    # The tally has to agree with the results it counted.
    assert batched["passed"] == sum(1 for r in results if r["passed"] and r["hash_match"])
    assert batched["failed"] == len(results) - batched["passed"]


def test_case_order_does_not_change_the_report() -> None:
    bench = Benchmark()
    cases = _cases()
    datasets = {c["dataset_ref"]: _candles(i) for i, c in enumerate(cases)}

    def run(order: list[dict]) -> str:
        return bench.command(
            json.dumps(
                {
                    "cmd": "run_suite",
                    "suite": {"name": "batch", "cases": order},
                    "datasets": datasets,
                }
            )
        )

    assert run(cases) == run(list(reversed(cases)))
