//! Python bindings for `wickra-benchmark`, exposed under the `wickra_benchmark`
//! package.
//!
//! Thin glue over the benchmark core's command surface: create a [`Benchmark`],
//! drive it with a command JSON (`run_case`, `run_suite`, `list_cases`,
//! `version`) and read back the response JSON. The same command protocol
//! crosses every binding, so a Python front-end drives the exact same core as
//! the native CLI.

// PyO3 protocol methods take `self` by value/ref regardless of use.
#![allow(clippy::needless_pass_by_value)]

use pyo3::exceptions::PyValueError;
use pyo3::prelude::*;

use benchmark_core::Benchmark;

/// A benchmark runner driven by JSON commands.
#[pyclass(name = "Benchmark")]
struct PyBenchmark {
    inner: Benchmark,
}

#[pymethods]
impl PyBenchmark {
    /// Create a benchmark runner. It is stateless — the case, suite and data
    /// arrive with each command.
    #[new]
    fn new() -> Self {
        Self {
            inner: Benchmark::new(),
        }
    }

    /// Apply a command JSON and return the resulting response JSON.
    fn command(&self, cmd_json: &str) -> PyResult<String> {
        self.inner
            .command_json(cmd_json)
            .map_err(|err| PyValueError::new_err(err.to_string()))
    }

    /// The library version.
    #[staticmethod]
    fn version() -> &'static str {
        Benchmark::version()
    }
}

/// The native module (`wickra_benchmark._wickra_benchmark`).
#[pymodule]
fn _wickra_benchmark(module: &Bound<'_, PyModule>) -> PyResult<()> {
    module.add("__version__", env!("CARGO_PKG_VERSION"))?;
    module.add_class::<PyBenchmark>()?;
    Ok(())
}
