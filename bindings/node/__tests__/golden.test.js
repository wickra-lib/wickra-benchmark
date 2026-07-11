"use strict";

// Cross-language golden / determinism: the same command yields byte-identical
// output every time and in every language. The curated golden corpus (once
// blessed) is replayed by the workspace integration tests and CI's
// cross-language golden step, all against these same bytes.

const { test } = require("node:test");
const assert = require("node:assert");
const { Benchmark } = require("../index.js");

const STRATEGY = {
  symbol: "BTCUSDT",
  timeframe: "1h",
  indicators: {
    ema_fast: { type: "Ema", params: [5] },
    ema_slow: { type: "Ema", params: [15] },
  },
  entry: { cross_above: ["ema_fast", "ema_slow"] },
  exit: { cross_below: ["ema_fast", "ema_slow"] },
  sizing: { type: "fixed_fraction", fraction: 0.95 },
  costs: { taker_bps: 5, slippage: { type: "fixed_bps", bps: 2 } },
};

function candles() {
  const out = [];
  for (let i = 0; i < 40; i++) {
    const base = 100.0 + Math.sin(i * 0.4) * 8.0;
    out.push({
      time: 1_700_000_000 + i * 3600,
      open: base,
      high: base + 1.0,
      low: base - 1.0,
      close: base + 0.5,
      volume: 1000.0,
    });
  }
  return out;
}

function request() {
  return JSON.stringify({
    cmd: "run_case",
    case: {
      id: "sma-crossover-01",
      description: "golden",
      strategy: STRATEGY,
      dataset_ref: "d.csv",
      expected: {},
      expected_hash: "0".repeat(64),
    },
    data: candles(),
  });
}

test("run_case output is byte-stable across calls", () => {
  const bench = new Benchmark();
  const req = request();
  assert.strictEqual(bench.command(req), bench.command(req));
});

test("version output is byte-stable across instances", () => {
  assert.strictEqual(
    new Benchmark().command('{"cmd":"version"}'),
    new Benchmark().command('{"cmd":"version"}'),
  );
});
