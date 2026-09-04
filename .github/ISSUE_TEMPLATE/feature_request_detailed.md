---
name: Feature request (detailed)
about: A change that needs design discussion -- a new command, a corpus or schema change, a new binding surface.
title: "[Feature] <short description>"
labels: ["enhancement", "triage"]
assignees: []
---

## Problem

<!--
What can you not do today? Describe the situation, not the solution you have in
mind -- the best fix is often not the one that comes to mind first.
-->

## Proposed change

<!-- What you would add or change. -->

## Affected area

- [ ] `benchmark-core` (the runner, the case/suite types)
- [ ] The `command_json` boundary (a new `cmd`, or a new field on one)
- [ ] Canonicalization or hashing
- [ ] The curated corpus (`cases/`, `datasets/`)
- [ ] CLI (`wickra-benchmark`)
- [ ] One or more language bindings
- [ ] Documentation only

## Does this change any committed hash?

- [ ] No -- purely additive, every frozen `expected_hash` stays as it is
- [ ] Yes -- and the corpus needs re-blessing

<!--
This is the question that decides how the change lands. A hash-moving change is
a breaking change for anyone who pinned a report, so it needs a version bump and
a CHANGELOG entry saying so, not a quiet re-bless.
-->

## Cross-language impact

<!--
The boundary is one canonical JSON string that all ten bindings return verbatim.
A change to the envelope reaches every one of them plus the golden corpus. Say
which bindings need new surface, or "none -- the envelope is unchanged".
-->

## Alternatives considered

<!-- Including doing nothing, and why that is not enough. -->

## Willing to implement?

- [ ] Yes, with review
- [ ] Yes, if pointed at the right place
- [ ] No, reporting it only
