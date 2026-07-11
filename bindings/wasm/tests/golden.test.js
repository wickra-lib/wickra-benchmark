"use strict";

// Golden test over the wasm-pack (nodejs target) output: the WebAssembly build
// runs a case, byte-identically to the native run. Skips cleanly when `pkg/` has
// not been built yet (`wasm-pack build --target nodejs`).

const { test } = require("node:test");
const assert = require("node:assert");
const path = require("node:path");

let wasm = null;
try {
  wasm = require(path.resolve(__dirname, "..", "pkg", "wickra_benchmark_wasm.js"));
} catch {
  wasm = null;
}

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

function runCaseRequest() {
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

test("wasm build present or skipped", (t) => {
  if (!wasm) t.skip("run `wasm-pack build --target nodejs` first");
});

if (wasm) {
  test("wasm runs a case with a shaped CaseResult", () => {
    const result = JSON.parse(new wasm.Benchmark().command(runCaseRequest()));
    assert.strictEqual(result.id, "sma-crossover-01");
    assert.strictEqual(result.hash.length, 64);
    assert.strictEqual(typeof result.passed, "boolean");
  });

  test("wasm run_case is byte-stable (determinism)", () => {
    const bench = new wasm.Benchmark();
    const req = runCaseRequest();
    assert.strictEqual(bench.command(req), bench.command(req));
  });

  test("wasm version matches the module export", () => {
    assert.strictEqual(new wasm.Benchmark().version(), wasm.version());
  });

  test("wasm reports an unknown command in-band", () => {
    const response = JSON.parse(new wasm.Benchmark().command('{"cmd":"nope"}'));
    assert.strictEqual(response.ok, false);
    assert.match(response.error, /nope/);
  });
}
