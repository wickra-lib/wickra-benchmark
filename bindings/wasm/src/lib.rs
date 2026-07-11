//! WebAssembly bindings for `wickra-benchmark` (wasm-bindgen).
//!
//! Recompute a curated case or suite and confirm its report and hash, compiled
//! to WebAssembly for the browser: create a `Benchmark`, drive it with a command
//! JSON (`run_case`, `run_suite`, `list_cases`, `version`) and read back the
//! response JSON. The same command protocol crosses every binding, so a browser
//! front-end runs against the exact same core as the native CLI.
//!
//! The suite runs sequentially here (no rayon thread pool in a browser sandbox),
//! which is byte-identical to the native parallel run — the exact
//! cross-language golden check.

use wasm_bindgen::prelude::*;

use benchmark_core::Benchmark as CoreBenchmark;

/// A benchmark runner driven by JSON commands.
#[wasm_bindgen]
pub struct Benchmark {
    inner: CoreBenchmark,
}

#[wasm_bindgen]
impl Benchmark {
    /// Create a benchmark runner. It is stateless — the case, suite and data
    /// arrive with each command.
    #[wasm_bindgen(constructor)]
    #[allow(clippy::new_without_default)]
    pub fn new() -> Benchmark {
        Self {
            inner: CoreBenchmark::new(),
        }
    }

    /// Apply a command JSON and return the resulting response JSON.
    pub fn command(&self, cmd_json: &str) -> Result<String, JsError> {
        self.inner
            .command_json(cmd_json)
            .map_err(|e| JsError::new(&e.to_string()))
    }

    /// The library version.
    #[wasm_bindgen(js_name = version)]
    pub fn instance_version(&self) -> String {
        CoreBenchmark::version().to_string()
    }
}

/// The library version.
#[wasm_bindgen]
pub fn version() -> String {
    CoreBenchmark::version().to_string()
}
