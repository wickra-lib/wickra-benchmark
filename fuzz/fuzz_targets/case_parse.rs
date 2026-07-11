#![no_main]
//! Fuzz the case parsing surface: arbitrary bytes are parsed as a
//! `BenchmarkCase` from both JSON and TOML. None must panic; malformed input
//! must surface as a clean `Err`, never a crash.

use benchmark_core::BenchmarkCase;
use libfuzzer_sys::fuzz_target;

fuzz_target!(|data: &[u8]| {
    let Ok(text) = std::str::from_utf8(data) else {
        return;
    };
    let _ = BenchmarkCase::from_json(text);
    let _ = BenchmarkCase::from_toml(text);
});
