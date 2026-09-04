//! Golden parity: replay every committed `golden/commands/*.json` through the
//! same canonical `command_json` surface every binding uses, and assert the
//! response is byte-for-byte identical to `golden/expected/<name>.json`. This is
//! the Rust anchor of the cross-language determinism guarantee; the ten bindings
//! assert the same bytes. It also checks the semantic promise: every blessed
//! `run_case`/`run_suite` fixture reproduces (`passed`/`hash_match`, `failed:0`).
//!
//! The corpus is derived, never authored by hand. An engine bump re-shapes every
//! report, so after one, re-bless the whole corpus in a single step:
//!
//! ```text
//! WICKRA_BLESS=1 cargo test -p benchmark-core --test golden
//! ```
//!
//! Blessing writes all four copies of a case from the same in-memory value:
//! `cases/`, `cases/suite.json`, the self-contained mirror under `golden/`, and
//! the runnable copy under `examples/data/`. That is deliberate — those copies
//! drifted apart once, when an engine bump re-blessed `cases/` and `golden/` but
//! left `examples/data/` behind, and the C example was the only job that noticed.
//! `scripts/check_corpus_sync.py` guards the same invariant in CI.
//!
//! Blessing also normalises: a case travels through `serde_json::Value`, whose
//! map is a `BTreeMap`, so every object lands with its keys sorted. Author a new
//! case's `strategy` in whatever order reads best and let blessing settle it —
//! key order carries no meaning here, and having one writer for every byte of the
//! corpus is the property that keeps the copies from diverging. Hashes are
//! unaffected either way: `canonicalize` sorts keys itself before hashing.

use benchmark_core::{load_candles, run_case, Benchmark, BenchmarkCase, Candle, Suite};
use serde::Serialize;
use serde_json::Value;
use std::collections::BTreeMap;
use std::fs;
use std::path::{Path, PathBuf};

fn repo_root() -> PathBuf {
    Path::new(env!("CARGO_MANIFEST_DIR")).join("../..")
}

fn golden_dir() -> PathBuf {
    repo_root().join("golden")
}

/// A structurally valid placeholder digest. Blessing needs the engine's own
/// output before the real hash exists, and `BenchmarkCase::validate` rejects a
/// malformed one, so the case is run once against this and the recomputed report
/// and hash are then frozen into it.
const PLACEHOLDER_HASH: &str = "0000000000000000000000000000000000000000000000000000000000000000";

// The command envelopes are serialised from structs rather than maps so the key
// order is the declared one (`cmd` first), not alphabetical.

#[derive(Serialize)]
struct RunCaseCommand<'a> {
    cmd: &'a str,
    case: &'a BenchmarkCase,
    data: &'a [Candle],
}

#[derive(Serialize)]
struct RunSuiteCommand<'a> {
    cmd: &'a str,
    suite: &'a Suite,
    datasets: &'a BTreeMap<String, Vec<Candle>>,
}

#[derive(Serialize)]
struct ListCasesCommand<'a> {
    cmd: &'a str,
    suite: &'a Suite,
}

#[derive(Serialize)]
struct VersionCommand<'a> {
    cmd: &'a str,
}

/// Lets `write_fixture` take any of the four command envelopes behind one
/// reference while still serialising each from its own struct, so the declared
/// key order survives into the file.
mod erased_command {
    use serde::Serialize;

    pub trait Command {
        /// The compact form fed to `command_json`.
        fn to_compact(&self) -> String;
        /// The pretty form committed under `golden/commands/`.
        fn to_pretty(&self) -> String;
    }

    impl<T: Serialize> Command for T {
        fn to_compact(&self) -> String {
            serde_json::to_string(self).expect("serialize")
        }
        fn to_pretty(&self) -> String {
            serde_json::to_string_pretty(self).expect("serialize")
        }
    }
}

/// Write `value` as pretty JSON with the repository's trailing newline.
fn write_pretty<T: Serialize>(path: &Path, value: &T) {
    let text = serde_json::to_string_pretty(value).expect("serialize");
    fs::write(path, format!("{text}\n")).unwrap_or_else(|e| panic!("{}: {e}", path.display()));
}

/// Write an already-canonical string with the repository's trailing newline.
fn write_canonical(path: &Path, canonical: &str) {
    fs::write(path, format!("{canonical}\n")).unwrap_or_else(|e| panic!("{}: {e}", path.display()));
}

/// The case ids under `cases/`, sorted — `suite.json` is the collection, not a case.
fn case_ids(root: &Path) -> Vec<String> {
    let mut ids: Vec<String> = fs::read_dir(root.join("cases"))
        .expect("cases/")
        .filter_map(|e| {
            let path = e.expect("dir entry").path();
            if path.extension().and_then(|x| x.to_str()) != Some("json") {
                return None;
            }
            let stem = path.file_stem()?.to_str()?.to_owned();
            (stem != "suite").then_some(stem)
        })
        .collect();
    ids.sort();
    ids
}

/// Recompute every case's `expected`/`expected_hash` from its own strategy and
/// dataset, then rewrite every copy of the corpus from those same values.
fn bless(root: &Path) {
    let ids = case_ids(root);
    assert!(!ids.is_empty(), "no cases to bless");

    let mut cases: Vec<BenchmarkCase> = Vec::with_capacity(ids.len());
    let mut datasets: BTreeMap<String, Vec<Candle>> = BTreeMap::new();

    for id in &ids {
        let path = root.join("cases").join(format!("{id}.json"));
        let text = fs::read_to_string(&path).unwrap_or_else(|e| panic!("{}: {e}", path.display()));
        let mut case: BenchmarkCase =
            serde_json::from_str(&text).unwrap_or_else(|e| panic!("{}: {e}", path.display()));

        let candles = load_candles(&root.join("datasets").join(&case.dataset_ref))
            .unwrap_or_else(|e| panic!("{}: {e}", case.dataset_ref));

        // Run against a placeholder expectation to obtain the engine's own
        // report and hash, then freeze both into the case.
        case.expected = Value::Object(serde_json::Map::new());
        case.expected_hash = PLACEHOLDER_HASH.to_string();
        let result = run_case(&case, &candles).unwrap_or_else(|e| panic!("{id}: {e}"));
        case.expected = result.recomputed;
        case.expected_hash = result.hash;

        // Confirm the frozen case now reproduces against itself.
        let check = run_case(&case, &candles).unwrap_or_else(|e| panic!("{id}: {e}"));
        assert!(
            check.passed && check.hash_match,
            "{id} does not re-reproduce"
        );

        write_pretty(&path, &case);
        datasets.insert(case.dataset_ref.clone(), candles);
        cases.push(case);
    }

    let suite = Suite {
        name: "wickra-benchmark v0.1 core suite".to_string(),
        cases,
    };
    write_pretty(&root.join("cases/suite.json"), &suite);

    // The golden directory is a self-contained mirror: the suite, the datasets
    // the commands embed, and the command/expectation pair for every fixture.
    let golden = golden_dir();
    write_pretty(&golden.join("suite.json"), &suite);
    for name in datasets.keys() {
        fs::copy(
            root.join("datasets").join(name),
            golden.join("datasets").join(name),
        )
        .unwrap_or_else(|e| panic!("{name}: {e}"));
    }

    // The command is written from the struct itself, never round-tripped through
    // a `Value`: serde_json's map is a `BTreeMap`, so parsing to `Value` would
    // re-sort every key alphabetically and silently reshape the whole corpus.
    let benchmark = Benchmark::new();
    let write_fixture = |name: &str, command: &dyn erased_command::Command| {
        let compact = command.to_compact();
        let response = benchmark
            .command_json(&compact)
            .unwrap_or_else(|e| panic!("{name}: {e}"));
        write_canonical(
            &golden.join("commands").join(format!("{name}.json")),
            &command.to_pretty(),
        );
        write_canonical(
            &golden.join("expected").join(format!("{name}.json")),
            &response,
        );
    };

    for case in &suite.cases {
        write_fixture(
            &case.id,
            &RunCaseCommand {
                cmd: "run_case",
                case,
                data: &datasets[&case.dataset_ref],
            },
        );
    }

    write_fixture(
        "suite-run",
        &RunSuiteCommand {
            cmd: "run_suite",
            suite: &suite,
            datasets: &datasets,
        },
    );
    write_fixture(
        "suite-list",
        &ListCasesCommand {
            cmd: "list_cases",
            suite: &suite,
        },
    );
    write_fixture("version", &VersionCommand { cmd: "version" });

    // The runnable copies under examples/data/ are the same bytes as the case
    // and dataset they mirror — only the ones already committed there, so
    // blessing never invents a new example fixture.
    for case in &suite.cases {
        let mirror = root
            .join("examples/data/cases")
            .join(format!("{}.json", case.id));
        if mirror.exists() {
            write_pretty(&mirror, case);
            fs::copy(
                root.join("datasets").join(&case.dataset_ref),
                root.join("examples/data/datasets").join(&case.dataset_ref),
            )
            .unwrap_or_else(|e| panic!("{}: {e}", case.dataset_ref));
        }
    }
}

#[test]
fn golden_commands_are_byte_identical() {
    let root = repo_root();
    if std::env::var("WICKRA_BLESS").is_ok() {
        bless(&root);
    }

    let dir = golden_dir();
    let benchmark = Benchmark::new();
    let mut count = 0;
    let mut run_case_seen = 0;
    let mut suite_run_seen = false;

    for entry in fs::read_dir(dir.join("commands")).unwrap() {
        let cmd_path = entry.unwrap().path();
        if cmd_path.extension().and_then(|e| e.to_str()) != Some("json") {
            continue;
        }
        let name = cmd_path.file_name().unwrap().to_string_lossy().into_owned();
        let cmd_json = fs::read_to_string(&cmd_path).unwrap();

        // Drive the exact canonical command surface the bindings use.
        let got = benchmark.command_json(&cmd_json).unwrap();

        let expected = fs::read_to_string(dir.join("expected").join(&name)).unwrap();
        assert_eq!(
            got.trim(),
            expected.trim(),
            "golden response mismatch for {name}"
        );

        // Semantic checks per command kind.
        let envelope: Value = serde_json::from_str(&cmd_json).unwrap();
        let response: Value = serde_json::from_str(&got).unwrap();
        match envelope["cmd"].as_str().unwrap() {
            "run_case" => {
                assert_eq!(response["passed"], true, "{name} must pass");
                assert_eq!(response["hash_match"], true, "{name} hash must match");
                run_case_seen += 1;
            }
            "run_suite" => {
                assert_eq!(response["failed"], 0, "{name} suite must have no failures");
                assert!(response["passed"].as_u64().unwrap() > 0);
                suite_run_seen = true;
            }
            _ => {}
        }
        count += 1;
    }

    assert!(
        count >= 8,
        "expected at least eight golden fixtures, got {count}"
    );
    assert_eq!(run_case_seen, 5, "expected five run_case fixtures");
    assert!(suite_run_seen, "expected a run_suite fixture");
}
