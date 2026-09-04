<!--
Keep it short. One logical change per PR.

For a change that touches the runner, the command boundary, the corpus or more
than one binding, use the long form instead:
https://github.com/wickra-lib/wickra-benchmark/compare/main...HEAD?template=detailed.md
-->

## What

<!-- What does this change and why? -->

## Does this move a committed hash?

- [ ] No — every `expected_hash` in `cases/` is unchanged
- [ ] Yes — the corpus was re-blessed here, and the CHANGELOG says so

## Checklist

- [ ] `cargo fmt --all` and `cargo clippy --workspace --all-targets --all-features -- -D warnings` are clean
- [ ] `cargo test --workspace --all-features` and `--no-default-features` pass (parallel == sequential)
- [ ] `cargo deny check` is clean
- [ ] Tests added/updated (prefer hand-computed expectations for core changes)
- [ ] The corpus was re-blessed with `WICKRA_BLESS=1 cargo test -p benchmark-core --test golden`, never hand-edited
- [ ] `python scripts/check_corpus_sync.py` passes — every copy of a case agrees
- [ ] Binding surface mirrored across languages; golden fixtures regenerated if the envelope changed
- [ ] `CHANGELOG.md` updated under `[Unreleased]`
