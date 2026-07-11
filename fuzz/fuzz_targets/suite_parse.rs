#![no_main]
//! Fuzz the suite parsing surface: arbitrary bytes are parsed as a `Suite` from
//! both JSON and TOML. None must panic; malformed input (including duplicate
//! case ids) must surface as a clean `Err`, never a crash.

use benchmark_core::Suite;
use libfuzzer_sys::fuzz_target;

fuzz_target!(|data: &[u8]| {
    let Ok(text) = std::str::from_utf8(data) else {
        return;
    };
    let _ = Suite::from_json(text);
    let _ = Suite::from_toml(text);
});
