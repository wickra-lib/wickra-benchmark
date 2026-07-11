//! The [`Benchmark`] command-JSON handle — the FFI boundary the ten language
//! bindings forward verbatim.

use crate::case::{BenchmarkCase, Candle};
use crate::error::{Error, Result};
use crate::hash::canonicalize;
use crate::runner::{run_case, run_suite_inline};
use crate::suite::Suite;
use serde::Deserialize;
use serde_json::{json, Value};
use std::collections::BTreeMap;
use wickra_backtest_core::version as engine_version;

/// A stateless benchmark handle. It carries no state — the case, suite and data
/// arrive with each command — but is handle-shaped so the ten language bindings
/// share the same surface as the other Wickra products.
#[derive(Debug, Clone, Copy, Default)]
pub struct Benchmark;

#[derive(Deserialize)]
struct RunCaseReq {
    case: BenchmarkCase,
    data: Vec<Candle>,
}

#[derive(Deserialize)]
struct RunSuiteReq {
    suite: Suite,
    #[serde(default)]
    datasets: BTreeMap<String, Vec<Candle>>,
}

#[derive(Deserialize)]
struct ListCasesReq {
    suite: Suite,
}

impl Benchmark {
    /// Construct a benchmark handle.
    #[must_use]
    pub fn new() -> Self {
        Self
    }

    /// The benchmark-core crate version.
    #[must_use]
    pub fn version() -> &'static str {
        env!("CARGO_PKG_VERSION")
    }

    /// Dispatch a command envelope `{"cmd": ...}` and return a canonical JSON
    /// string. Unknown commands and errors return an error envelope
    /// (`{"ok":false,"error":...}`), never a panic.
    pub fn command_json(&self, cmd_json: &str) -> Result<String> {
        let value = dispatch(cmd_json);
        canonicalize(&value)
    }
}

fn dispatch(cmd_json: &str) -> Value {
    match dispatch_inner(cmd_json) {
        Ok(v) => v,
        Err(e) => json!({ "ok": false, "error": e.to_string() }),
    }
}

fn dispatch_inner(cmd_json: &str) -> Result<Value> {
    let env: Value = serde_json::from_str(cmd_json).map_err(|e| Error::Parse(e.to_string()))?;
    let cmd = env.get("cmd").and_then(Value::as_str).unwrap_or("");
    match cmd {
        "run_case" => {
            let req: RunCaseReq =
                serde_json::from_value(env).map_err(|e| Error::Parse(e.to_string()))?;
            let result = run_case(&req.case, &req.data)?;
            serde_json::to_value(result).map_err(|e| Error::BadCase(e.to_string()))
        }
        "run_suite" => {
            let req: RunSuiteReq =
                serde_json::from_value(env).map_err(|e| Error::Parse(e.to_string()))?;
            let report = run_suite_inline(&req.suite, &req.datasets)?;
            serde_json::to_value(report).map_err(|e| Error::BadCase(e.to_string()))
        }
        "list_cases" => {
            let req: ListCasesReq =
                serde_json::from_value(env).map_err(|e| Error::Parse(e.to_string()))?;
            req.suite.validate()?;
            Ok(json!({ "ids": req.suite.case_ids() }))
        }
        "version" => Ok(json!({
            "version": Benchmark::version(),
            "engine_version": engine_version(),
        })),
        other => Err(Error::Parse(format!("unknown cmd: {other}"))),
    }
}
