// A runnable WebAssembly example: load a curated benchmark case and its
// dataset, recompute the report through the wasm build, and assert it
// reproduces — both `passed` (the report matches the frozen expectation) and
// `hash_match` (its canonical hash matches).
//
//   wasm-pack build bindings/wasm --target nodejs
//   node examples/wasm/run.cjs
//
// The surface is the same as the Node binding's, deliberately: both are consumed
// from JavaScript, so moving between the two packages should not mean relearning
// method names. The response is the core's canonical string verbatim, so the
// hash printed below is byte-identical to the one every other language prints
// for this case — which is the whole claim this repository exists to make.

"use strict";

const assert = require("node:assert");
const fs = require("node:fs");
const path = require("node:path");
const wasm = require("../../bindings/wasm/pkg/wickra_benchmark_wasm.js");

const DATA = path.join(__dirname, "..", "data");

function candles(csvPath) {
  const lines = fs.readFileSync(csvPath, "utf8").trim().split(/\r?\n/);
  return lines.slice(1).map((line) => {
    const [time, open, high, low, close, volume] = line.split(",");
    return {
      time: Number(time),
      open: Number(open),
      high: Number(high),
      low: Number(low),
      close: Number(close),
      volume: Number(volume),
    };
  });
}

const theCase = JSON.parse(
  fs.readFileSync(path.join(DATA, "cases", "sma-crossover-01.json"), "utf8"),
);
const data = candles(path.join(DATA, "datasets", theCase.dataset_ref));

const benchmark = new wasm.Benchmark();
const result = JSON.parse(
  benchmark.command(JSON.stringify({ cmd: "run_case", case: theCase, data })),
);

console.log("wickra-benchmark", wasm.version());
console.log(
  `${result.id}: passed=${result.passed} hash_match=${result.hash_match}`,
);
assert.ok(
  result.passed && result.hash_match,
  "the curated case must reproduce",
);
console.log("REPRODUCED (passed + hash_match)");
