#![no_main]
//! Fuzz the FFI command boundary — the surface every language binding forwards
//! verbatim. Arbitrary bytes are handed to `command_json`. It must never panic:
//! a bad envelope, an unknown command, or malformed data all come back in-band
//! as a canonical `{"ok":false,...}` (or an `Err`), never a crash. When it does
//! return a response, it must be re-parseable canonical JSON.

use benchmark_core::{canonicalize, Benchmark};
use libfuzzer_sys::fuzz_target;
use serde_json::Value;

fuzz_target!(|data: &[u8]| {
    let Ok(text) = std::str::from_utf8(data) else {
        return;
    };
    let benchmark = Benchmark::new();
    if let Ok(response) = benchmark.command_json(text) {
        // A returned response is always canonical, re-parseable JSON.
        let value: Value = serde_json::from_str(&response).expect("response is valid JSON");
        assert_eq!(
            canonicalize(&value).unwrap(),
            response,
            "command_json output is canonical"
        );
    }
});
