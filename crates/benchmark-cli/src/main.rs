//! The `wickra-benchmark` reference CLI.
//!
//! Runs a curated case or suite against its datasets, recomputing each report
//! with the pinned engine and checking it against the frozen expectation. Exits
//! `0` when everything passed, `1` when any case failed (so a non-reproducible
//! engine turns a CI build red) or on error.

mod args;
mod run;

use args::Cli;
use clap::Parser;
use std::process::ExitCode;

fn main() -> ExitCode {
    let cli = Cli::parse();
    match run::run(&cli) {
        Ok(output) => {
            print!("{}", output.text);
            ExitCode::from(output.code)
        }
        Err(err) => {
            eprintln!("wickra-benchmark: {err}");
            ExitCode::from(1)
        }
    }
}
