//! Node.js bindings for `wickra-benchmark` (napi-rs).
//!
//! Thin glue over the benchmark core's command surface: create a `Benchmark`,
//! drive it with a command JSON (`run_case`, `run_suite`, `list_cases`,
//! `version`) and read back the response JSON. The same command protocol crosses
//! every binding, so a Node front-end drives the exact same core as the native
//! CLI.

#![allow(missing_debug_implementations)]
// napi exposes owned `String` arguments; the bodies only need to borrow them.
#![allow(clippy::needless_pass_by_value)]
// The napi constructor is the public entry point; a Default impl would be
// unreachable from JS and misleading here.
#![allow(clippy::new_without_default)]

use napi::Result;
use napi_derive::napi;

use benchmark_core::Benchmark as CoreBenchmark;

/// Build a napi error from a message.
fn err(message: impl Into<String>) -> napi::Error {
    napi::Error::from_reason(message.into())
}

/// The library version.
#[napi]
pub fn version() -> String {
    CoreBenchmark::version().to_string()
}

/// A benchmark runner driven by JSON commands.
#[napi]
pub struct Benchmark {
    inner: CoreBenchmark,
}

#[napi]
impl Benchmark {
    /// Create a benchmark runner. It is stateless — the case, suite and data
    /// arrive with each command.
    #[napi(constructor)]
    pub fn new() -> Self {
        Self {
            inner: CoreBenchmark::new(),
        }
    }

    /// Apply a command JSON and return the resulting response JSON.
    #[napi]
    pub fn command(&self, cmd_json: String) -> Result<String> {
        self.inner
            .command_json(&cmd_json)
            .map_err(|e| err(e.to_string()))
    }

    /// The library version.
    #[napi]
    pub fn version(&self) -> String {
        CoreBenchmark::version().to_string()
    }
}
