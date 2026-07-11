"""A runnable Python example: load a curated benchmark case and its dataset,
recompute the report with the wickra-benchmark binding, and confirm it
reproduces — both ``passed`` (the report matches the frozen expectation) and
``hash_match`` (its canonical hash matches).

    pip install wickra-benchmark
    python examples/python/run.py
"""

import csv
import json
from pathlib import Path

from wickra_benchmark import Benchmark

DATA = Path(__file__).resolve().parent.parent / "data"


def _candles(csv_path: Path) -> list[dict]:
    with csv_path.open(newline="") as f:
        return [
            {
                "time": int(row["time"]),
                "open": float(row["open"]),
                "high": float(row["high"]),
                "low": float(row["low"]),
                "close": float(row["close"]),
                "volume": float(row["volume"]),
            }
            for row in csv.DictReader(f)
        ]


def main() -> None:
    case = json.loads((DATA / "cases" / "sma-crossover-01.json").read_text())
    data = _candles(DATA / "datasets" / case["dataset_ref"])

    benchmark = Benchmark()
    result = json.loads(
        benchmark.command(json.dumps({"cmd": "run_case", "case": case, "data": data}))
    )

    print(f"wickra-benchmark {Benchmark.version()}")
    print(
        f"{result['id']}: passed={result['passed']} hash_match={result['hash_match']}"
    )
    assert result["passed"] and result["hash_match"], "the curated case must reproduce"
    print("REPRODUCED (passed + hash_match)")


if __name__ == "__main__":
    main()
