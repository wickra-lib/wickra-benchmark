<!--
Thanks for contributing to wickra-benchmark!

This is the long form, for changes that touch the runner, the command boundary,
the corpus or more than one binding. For anything smaller the default template is
the right one -- open the PR without ?template=detailed.md.

Fill in what applies and delete the rest.
-->

## Summary

<!-- 1-3 sentences: what does this change, and why? -->

## Type of change

- [ ] Bug fix (non-breaking change that fixes an issue)
- [ ] New feature (non-breaking change that adds functionality)
- [ ] Breaking change (fix or feature that changes an existing public API)
- [ ] Performance improvement
- [ ] Refactor (no functional change)
- [ ] Documentation only
- [ ] CI / build / tooling

## Affected surfaces

- [ ] Core (`crates/benchmark-core`) — the runner, the case/suite types, hashing
- [ ] CLI (`crates/benchmark-cli`)
- [ ] C ABI (`bindings/c`) — the hub every non-native binding calls through
- [ ] Python (`bindings/python`)
- [ ] Node.js (`bindings/node`)
- [ ] WASM (`bindings/wasm`)
- [ ] C# (`bindings/csharp`)
- [ ] Go (`bindings/go`)
- [ ] Java (`bindings/java`)
- [ ] R (`bindings/r`)
- [ ] The curated corpus (`cases/`, `datasets/`)
- [ ] The golden corpus (`golden/`)
- [ ] Examples / docs

## Linked issues

<!-- "Closes #123", "Refs #456". One per line. -->

Closes #

## Does this move a committed hash?

- [ ] No — every `expected_hash` in `cases/` is unchanged
- [ ] Yes — the corpus was re-blessed in this PR

<!--
This is the question that decides how the change lands. Anyone who pinned a
report reads a moved hash as a broken promise, so a hash-moving change needs to
say so out loud: in the CHANGELOG, and in the PR title if it is the point of the
change. Re-blessing quietly is the one thing this repository must not do.
-->

If yes, what moved it?

- [ ] An engine bump (`wickra-backtest-core`) — say which versions
- [ ] A change to a dataset
- [ ] A change to canonicalization or the hash
- [ ] A change to a case's strategy

## How was this tested?

<!--
- `cargo test --workspace --all-features` and `--no-default-features`
- Which bindings did you actually run, not just build?
- Fuzz targets touched? (`fuzz/`)
- Manual repro steps, if applicable
-->

## Determinism (if you changed the runner, canonicalization or the corpus)

The product's whole claim is that the same `(case, data)` yields the same bytes
everywhere. Each of these is cheap to break and expensive to notice later.

- [ ] The parallel (rayon) and sequential runners produce the identical
      `SuiteReport` — results are re-sorted by `id` before tallying
- [ ] Case order in the suite file does not affect the report
- [ ] `canonicalize` is idempotent, and its output still sorts every object key
- [ ] The golden corpus was re-blessed with
      `WICKRA_BLESS=1 cargo test -p benchmark-core --test golden`, not hand-edited
- [ ] `python scripts/check_corpus_sync.py` passes — every copy of a case
      (`cases/`, `cases/suite.json`, `golden/`, `examples/data/`) agrees

## Cross-language parity (if you changed the command boundary)

All ten bindings return the core's canonical string verbatim, so a change to the
envelope reaches every one of them at once.

- [ ] Every binding still returns byte-identical responses for `golden/commands/*`
- [ ] New surface mirrored in each binding, or deliberately deferred (say which)
- [ ] `bindings/node/index.d.ts` and `index.js` regenerated and committed

## Checklist

- [ ] `cargo fmt --all` and `cargo clippy --workspace --all-targets --all-features -- -D warnings` are clean
- [ ] `cargo deny check` is clean
- [ ] Tests added or updated
- [ ] `CHANGELOG.md` updated under `[Unreleased]`
- [ ] No AI attribution or `Co-authored-by` trailers in the commits
