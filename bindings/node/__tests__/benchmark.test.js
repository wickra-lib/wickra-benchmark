"use strict";

// Smoke: a benchmark runs a case against inline data, exposes the pinned engine
// version, and reports unknown commands in-band.

const { test } = require("node:test");
const assert = require("node:assert");
const { Benchmark, version } = require("../index.js");

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

// A placeholder expected/hash: the run recomputes the real report; the smoke
// test only checks the response shape, not that the case passes.
function runCaseRequest() {
  return JSON.stringify({
    cmd: "run_case",
    case: {
      id: "sma-crossover-01",
      description: "smoke",
      strategy: STRATEGY,
      dataset_ref: "d.csv",
      expected: {},
      expected_hash: "0".repeat(64),
    },
    data: candles(),
  });
}

test("run_case returns a shaped CaseResult", () => {
  const result = JSON.parse(new Benchmark().command(runCaseRequest()));
  assert.strictEqual(result.id, "sma-crossover-01");
  assert.strictEqual(result.passed, false); // placeholder expected does not match
  assert.strictEqual(result.hash_match, false);
  assert.strictEqual(result.hash.length, 64);
});

test("version matches the module export", () => {
  assert.strictEqual(new Benchmark().version(), version());
  const v = JSON.parse(new Benchmark().command('{"cmd":"version"}'));
  assert.strictEqual(v.version, version());
});

test("run_case is byte-stable (determinism)", () => {
  const bench = new Benchmark();
  const req = runCaseRequest();
  assert.strictEqual(bench.command(req), bench.command(req));
});

test("unknown command is an in-band error", () => {
  const response = JSON.parse(new Benchmark().command('{"cmd":"nope"}'));
  assert.strictEqual(response.ok, false);
  assert.match(response.error, /nope/);
});
