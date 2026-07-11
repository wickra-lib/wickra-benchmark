# Hashing and canonicalization

A case's `expected_hash` — and the `hash` every run reports — is only as
trustworthy as the serialization it runs over. So the hash is taken over a
**canonical** form of the report, byte-for-byte the same contract
[`wickra-proof`](https://github.com/wickra-lib/wickra-proof) uses. Source:
[`crates/benchmark-core/src/hash.rs`](../crates/benchmark-core/src/hash.rs).

## Canonicalization

Given any `serde_json::Value`, `canonicalize` emits a single deterministic string:

- **Object keys sorted** by code point (via a `BTreeMap`), recursively.
- **No structural whitespace** — no spaces, no newlines.
- **Floats quantized** to `1e-8` (`{:.8}`), then trailing zeros trimmed and whole
  values collapsed to integers (`3.50000000` → `3.5`, `4.00000000` → `4`). Values
  above a fixed magnitude cutoff are passed through as integers to avoid precision
  loss.
- **Non-finite numbers** (`NaN`, `±inf`) collapse to `null` — JSON has no way to
  represent them, and real reports are finite by construction, so this only pins
  the boundary behavior; it never fires on genuine output.

Canonicalization is **idempotent** (re-canonicalizing a canonical string's parse
yields the identical string) and **key-order invariant** (two values differing
only in key order canonicalize equal). The property tests pin both.

## The hash

```
hash = blake3(canonicalize(value))   // lowercase, 64 hex characters
```

`blake3` over the canonical string yields each digest. Because the canonical
string is identical in every language, so is the hash — the cross-language golden
fixtures assert byte-for-byte equality of the whole `command_json` response,
which includes these hashes.

## Relationship to wickra-proof

The canonicalization here is the same determinism moat as `wickra-proof`'s: a
report's canonical hash under benchmark-core equals its hash under wickra-proof.
That is what lets a benchmark case and a proof over the same report agree — the
two products share one hashing contract rather than each inventing its own.

## Why two checks

A case reports `passed` (byte-exact equality of the recomputed report against the
frozen `expected`) **and** `hash_match` (equality of the recomputed hash against
`expected_hash`) separately. Both are computed from the same canonical form, so
in a correctly blessed case they agree. They diverge only when `expected` and
`expected_hash` disagree — i.e. a hand-edited or mis-blessed case — which is
exactly the failure the two independent booleans exist to surface.
