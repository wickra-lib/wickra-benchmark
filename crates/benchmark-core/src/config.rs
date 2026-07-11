//! Runner configuration.
//!
//! Benchmarking a case is a byte-exact recompute-and-compare — there are no
//! tolerances to tune (that is `wickra-verify`'s job). [`Config`] is therefore a
//! deliberately empty, forward-compatible options struct: it keeps the binding
//! constructor surface uniform with the other Wickra products (each takes a
//! config JSON string) and reserves room for future run options without a
//! breaking change. Unknown keys are rejected so a typo in a future option is
//! caught rather than silently ignored.

use serde::{Deserialize, Serialize};

/// Reserved runner options. Currently carries no fields; parsing `{}` yields the
/// default.
#[derive(Debug, Clone, Copy, Default, PartialEq, Eq, Serialize, Deserialize)]
#[serde(default, deny_unknown_fields)]
pub struct Config {}
