// Batch equivalence for the WebAssembly build: a suite must produce exactly what
// the cases produce alone.
//
// `run_suite` is the batch form of `run_case`. It fans the cases out -- over
// rayon when the parallel feature is on -- and re-sorts the results by `id`
// before tallying, so the two paths share an engine but not a control flow.
// Nothing else holds them to the same answer.
//
// Three things can break independently here and none would fail another test in
// this directory: a case could pick up state from the one before it, the results
// could come back in scheduling order rather than sorted, and the tally could
// disagree with the results it counted.

"use strict";

const assert = require("node:assert");
const { test } = require("node:test");
const { Benchmark } = require("../pkg/wickra_benchmark_wasm.js");

const ZERO_HASH = "0".repeat(64);

function strategy(fast, slow) {
  return {
    symbol: "BTCUSDT",
    timeframe: "1h",
    indicators: {
      ema_fast: { type: "Ema", params: [fast] },
      ema_slow: { type: "Ema", params: [slow] },
    },
    entry: { cross_above: ["ema_fast", "ema_slow"] },
    exit: { cross_below: ["ema_fast", "ema_slow"] },
    sizing: { type: "fixed_fraction", fraction: 0.95 },
    costs: { taker_bps: 5, slippage: { type: "fixed_bps", bps: 2 } },
  };
}

function candles(seed) {
  return Array.from({ length: 40 }, (_, i) => {
    const base = 100.0 + Math.sin(i * 0.4 + seed) * 8.0;
    return {
      time: 1700000000 + i * 3600,
      open: base,
      high: base + 1.0,
      low: base - 1.0,
      close: base + 0.5,
      volume: 1000.0,
    };
  });
}

// Deliberately out of id order, each on its own dataset: a suite that only ever
// sees sorted input cannot show that it sorts.
const CASES = [3, 1, 2].map((n) => ({
  id: `case-0${n}`,
  description: "batch equivalence",
  strategy: strategy(3 + n, 12 + n),
  dataset_ref: `d${n}.csv`,
  expected: {},
  expected_hash: ZERO_HASH,
}));

const DATASETS = Object.fromEntries(
  CASES.map((c, i) => [c.dataset_ref, candles(i)]),
);

test("a suite matches the cases run alone", () => {
  const bench = new Benchmark();

  const alone = {};
  for (const theCase of CASES) {
    const result = JSON.parse(
      bench.command(
        JSON.stringify({
          cmd: "run_case",
          case: theCase,
          data: DATASETS[theCase.dataset_ref],
        }),
      ),
    );
    alone[result.id] = result;
  }

  const batched = JSON.parse(
    bench.command(
      JSON.stringify({
        cmd: "run_suite",
        suite: { name: "batch", cases: CASES },
        datasets: DATASETS,
      }),
    ),
  );

  assert.deepStrictEqual(
    batched.results.map((r) => r.id),
    Object.keys(alone).sort(),
    "the suite must sort by id",
  );
  for (const result of batched.results) {
    assert.deepStrictEqual(result, alone[result.id]);
  }

  const passed = batched.results.filter((r) => r.passed && r.hash_match).length;
  assert.strictEqual(batched.passed, passed);
  assert.strictEqual(batched.failed, batched.results.length - passed);
});

test("case order does not change the report", () => {
  const bench = new Benchmark();
  const run = (order) =>
    bench.command(
      JSON.stringify({
        cmd: "run_suite",
        suite: { name: "batch", cases: order },
        datasets: DATASETS,
      }),
    );
  assert.strictEqual(run(CASES), run([...CASES].reverse()));
});
