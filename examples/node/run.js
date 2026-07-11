// A runnable Node.js example: load a curated benchmark case and its dataset,
// recompute the report with the wickra-benchmark binding, and assert it
// reproduces — both `passed` (the report matches the frozen expectation) and
// `hash_match` (its canonical hash matches).
//
//   ( cd bindings/node && npm install && npm run build )
//   ( cd examples/node && npm install && node run.js )

"use strict";

const assert = require("node:assert");
const fs = require("node:fs");
const path = require("node:path");
const { Benchmark, version } = require("wickra-benchmark");

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

const benchmark = new Benchmark();
const result = JSON.parse(
  benchmark.command(JSON.stringify({ cmd: "run_case", case: theCase, data })),
);

console.log("wickra-benchmark", version());
console.log(
  `${result.id}: passed=${result.passed} hash_match=${result.hash_match}`,
);
assert.ok(
  result.passed && result.hash_match,
  "the curated case must reproduce",
);
console.log("REPRODUCED (passed + hash_match)");
