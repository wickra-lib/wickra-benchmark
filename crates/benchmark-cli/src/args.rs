//! CLI argument parsing.

use clap::{Parser, Subcommand, ValueEnum};
use std::path::PathBuf;

/// Run curated benchmark cases and confirm their reports and hashes.
///
/// Exit code: `0` when everything passed, `1` when any case failed (so a
/// non-reproducible engine turns a CI build red) or on error.
#[derive(Parser, Debug)]
#[command(name = "wickra-benchmark", version, about)]
pub struct Cli {
    /// The subcommand to run.
    #[command(subcommand)]
    pub command: Command,
}

/// The benchmark subcommands.
#[derive(Subcommand, Debug)]
pub enum Command {
    /// Run a single case against its dataset.
    RunCase {
        /// Path to the case (JSON or TOML, chosen by extension).
        #[arg(long)]
        case: PathBuf,
        /// Directory holding the datasets the case references.
        #[arg(long)]
        data_root: PathBuf,
        /// Output format.
        #[arg(long, value_enum, default_value_t = Format::Text)]
        format: Format,
    },
    /// Run a whole suite against its datasets.
    RunSuite {
        /// Path to the suite (JSON or TOML, chosen by extension).
        #[arg(long)]
        suite: PathBuf,
        /// Directory holding the datasets the cases reference.
        #[arg(long)]
        data_root: PathBuf,
        /// Output format.
        #[arg(long, value_enum, default_value_t = Format::Text)]
        format: Format,
    },
    /// List the case ids in a suite (sorted).
    ListCases {
        /// Path to the suite (JSON or TOML, chosen by extension).
        #[arg(long)]
        suite: PathBuf,
    },
}

/// The output format.
#[derive(Clone, Copy, Debug, ValueEnum, PartialEq, Eq)]
pub enum Format {
    /// A human-readable aligned table.
    Text,
    /// The full report as JSON.
    Json,
}

#[cfg(test)]
mod tests {
    use super::*;
    use clap::CommandFactory;

    #[test]
    fn arg_config_is_valid() {
        Cli::command().debug_assert();
    }

    #[test]
    fn a_subcommand_is_required() {
        assert!(Cli::try_parse_from(["wickra-benchmark"]).is_err());
    }

    #[test]
    fn run_suite_defaults_to_text() {
        let cli = Cli::try_parse_from([
            "wickra-benchmark",
            "run-suite",
            "--suite",
            "s.json",
            "--data-root",
            "d",
        ])
        .unwrap();
        match cli.command {
            Command::RunSuite { format, .. } => assert_eq!(format, Format::Text),
            _ => panic!("expected run-suite"),
        }
    }

    #[test]
    fn run_case_json_parses() {
        let cli = Cli::try_parse_from([
            "wickra-benchmark",
            "run-case",
            "--case",
            "c.json",
            "--data-root",
            "d",
            "--format",
            "json",
        ])
        .unwrap();
        match cli.command {
            Command::RunCase { format, .. } => assert_eq!(format, Format::Json),
            _ => panic!("expected run-case"),
        }
    }
}
