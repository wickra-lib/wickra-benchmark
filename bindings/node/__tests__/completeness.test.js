"use strict";

// Parity guard: the Node binding must expose the full public surface of the
// benchmark, so an export dropped in a refactor fails loudly here (mirrors the
// completeness checks in the Python and R bindings).

const { test } = require("node:test");
const assert = require("node:assert");
const wickra = require("../index.js");

// The napi-rs loader adds its own diagnostics to the module object
// (`__napiBindingTarget` since @napi-rs/cli 3.10); they belong to the loader,
// not to the binding's surface, and are left out of the comparison.
const exported = () =>
  Object.keys(wickra)
    .filter((name) => !name.startsWith("__"))
    .sort();

test("module exposes Benchmark and version", () => {
  assert.strictEqual(typeof wickra.Benchmark, "function");
  assert.strictEqual(typeof wickra.version, "function");
});

test("Benchmark exposes command and version", () => {
  for (const name of ["command", "version"]) {
    assert.strictEqual(
      typeof wickra.Benchmark.prototype[name],
      "function",
      `Benchmark is missing ${name}`,
    );
  }
});

test("module surface is exactly {Benchmark, version}", () => {
  assert.deepStrictEqual(exported(), ["Benchmark", "version"]);
});

test("Benchmark surface is exactly {command, version}", () => {
  const methods = Object.getOwnPropertyNames(wickra.Benchmark.prototype)
    .filter((name) => name !== "constructor")
    .sort();
  assert.deepStrictEqual(methods, ["command", "version"]);
});
