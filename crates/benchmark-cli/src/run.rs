//! Load cases and datasets, run them, and render the result.

use crate::args::{Cli, Command, Format};
use benchmark_core::{load_candles, run_case, run_suite, BenchmarkCase, CaseResult, Suite};
use std::fmt::Write as _;
use std::path::Path;

/// The rendered output and the process exit code (`0` = all passed).
pub struct Output {
    /// The text to print to stdout.
    pub text: String,
    /// The process exit code.
    pub code: u8,
}

/// Dispatch the parsed CLI to its handler.
pub fn run(cli: &Cli) -> Result<Output, String> {
    match &cli.command {
        Command::RunCase {
            case,
            data_root,
            format,
        } => run_case_cmd(case, data_root, *format),
        Command::RunSuite {
            suite,
            data_root,
            format,
        } => run_suite_cmd(suite, data_root, *format),
        Command::ListCases { suite } => list_cases_cmd(suite),
    }
}

fn load_case(path: &Path) -> Result<BenchmarkCase, String> {
    let text = std::fs::read_to_string(path).map_err(|e| format!("{}: {e}", path.display()))?;
    let parsed = if is_toml(path) {
        BenchmarkCase::from_toml(&text)
    } else {
        BenchmarkCase::from_json(&text)
    };
    parsed.map_err(|e| format!("{}: {e}", path.display()))
}

fn load_suite(path: &Path) -> Result<Suite, String> {
    let text = std::fs::read_to_string(path).map_err(|e| format!("{}: {e}", path.display()))?;
    let parsed = if is_toml(path) {
        Suite::from_toml(&text)
    } else {
        Suite::from_json(&text)
    };
    parsed.map_err(|e| format!("{}: {e}", path.display()))
}

fn is_toml(path: &Path) -> bool {
    path.extension()
        .is_some_and(|e| e.eq_ignore_ascii_case("toml"))
}

fn run_case_cmd(case_path: &Path, data_root: &Path, format: Format) -> Result<Output, String> {
    let case = load_case(case_path)?;
    let candles = load_candles(&data_root.join(&case.dataset_ref)).map_err(|e| e.to_string())?;
    let result = run_case(&case, &candles).map_err(|e| e.to_string())?;
    let passed = result.passed && result.hash_match;
    let text = match format {
        Format::Json => serde_json::to_string(&result).map_err(|e| e.to_string())?,
        Format::Text => {
            let mut out = render_table(std::slice::from_ref(&result));
            let _ = writeln!(out, "{}/1 passed", usize::from(passed));
            out
        }
    };
    Ok(Output {
        text,
        code: u8::from(!passed),
    })
}

fn run_suite_cmd(suite_path: &Path, data_root: &Path, format: Format) -> Result<Output, String> {
    let suite = load_suite(suite_path)?;
    let report = run_suite(&suite, data_root).map_err(|e| e.to_string())?;
    let text = match format {
        Format::Json => serde_json::to_string(&report).map_err(|e| e.to_string())?,
        Format::Text => {
            let mut out = render_table(&report.results);
            let _ = writeln!(out, "{}/{} passed", report.passed, report.results.len());
            out
        }
    };
    Ok(Output {
        text,
        code: u8::from(report.failed > 0),
    })
}

fn list_cases_cmd(suite_path: &Path) -> Result<Output, String> {
    let suite = load_suite(suite_path)?;
    let mut text = String::new();
    for id in suite.case_ids() {
        let _ = writeln!(text, "{id}");
    }
    Ok(Output { text, code: 0 })
}

/// A left-aligned `id | passed | hash_match | hash` table with a header. The
/// hash is shown as a short prefix for readability; the JSON output carries the
/// full digest.
fn render_table(results: &[CaseResult]) -> String {
    let id_width = results
        .iter()
        .map(|r| r.id.len())
        .chain(std::iter::once("id".len()))
        .max()
        .unwrap_or(2);
    let mut out = String::new();
    let _ = writeln!(
        out,
        "{:<id_width$}  {:<6}  {:<10}  hash",
        "id", "passed", "hash_match"
    );
    for r in results {
        let _ = writeln!(
            out,
            "{:<id_width$}  {:<6}  {:<10}  {}",
            r.id,
            r.passed,
            r.hash_match,
            &r.hash[..r.hash.len().min(12)]
        );
    }
    out
}
