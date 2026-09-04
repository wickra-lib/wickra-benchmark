---
name: Bug report (detailed)
about: A bug that needs more than a paragraph -- a case that will not reproduce, a hash that moved, a binding disagreeing with Rust.
title: "[Bug] <short description>"
labels: ["bug", "triage"]
assignees: []
---

## Summary

<!-- One or two sentences. What is wrong, and how wrong. -->

## Affected area

- [ ] A case does not reproduce (`passed: false`)
- [ ] A hash does not match (`hash_match: false`)
- [ ] `passed` and `hash_match` disagree with each other
- [ ] Suite running (`run_suite`, ordering, the pass/fail tally)
- [ ] Canonicalization or hashing (`canonicalize`, `blake3`)
- [ ] Case or suite parsing (`BenchmarkCase`, `Suite`, the dataset CSV loader)
- [ ] A language binding disagreeing with the Rust result
- [ ] CLI (`wickra-benchmark`)

## Reproduction

<!-- The case and the dataset it names. Attach both if they do not fit inline. -->

```json
{ }
```

```bash
wickra-benchmark run-case --case case.json --data-root datasets
```

## Expected vs actual

| | Expected | Actual |
| --- | --- | --- |
| `passed` | `true` | `false` |
| `hash_match` | `true` | `false` |
| `hash` | `<the frozen expected_hash>` | `<what came back>` |

<!--
For a reproduction failure, the useful detail is *where* the recomputed report
diverges from the frozen one -- the first differing field, not just that they
differ. `run-case --json` prints the recomputed report next to its hash.
-->

## Which of the two booleans moved

<!--
`passed` and `hash_match` are independent on purpose. Both false usually means
the engine changed. Only `hash_match` false means the case's `expected` and
`expected_hash` disagree with each other -- the case was edited by hand instead
of being blessed. Saying which one you saw narrows this immediately.
-->

## Cross-binding check (if relevant)

<!--
Does the Rust result differ from a binding's? All ten return the core's canonical
string verbatim and are pinned by the golden corpus, so a disagreement is a
marshalling bug and worth saying so explicitly.
-->

| Binding | `hash` returned |
| --- | --- |
| Rust | |
| The one that differs | |

## Environment

| Field | Value |
| --- | --- |
| wickra-benchmark version | `e.g. 0.1.0` |
| Engine version (`version` command) | `e.g. 0.1.4` |
| Binding | `Rust / Python / Node.js / WASM / C / C++ / C# / Go / Java / R` |
| OS / arch | `e.g. Windows 11 x86_64` |
| Toolchain | `rustc 1.x.y` |
| Runner | `parallel (default) / sequential (--no-default-features)` |

## What you already ruled out

<!-- Saves a round trip. -->
