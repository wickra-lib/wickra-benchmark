---
name: Bug report
about: Report incorrect behaviour or a crash
title: "[bug] "
labels: bug
---

**Describe the bug**
A clear description of what is wrong.

**Reproduction**
The smallest input that reproduces the problem — a `BenchmarkCase` and the
dataset it names, or the exact `command` JSON.

```
# paste a minimal repro here (a case + its dataset rows, or a command JSON)
```

**Expected vs actual**
- Expected: …
- Actual: …

**Which of the two booleans moved**
`passed` and `hash_match` are independent. Both false usually means the engine
changed; only `hash_match` false means a case's `expected` and `expected_hash`
disagree with each other.

**Environment**
- `wickra-benchmark` version:
- Engine version (the `version` command):
- Language / binding (Rust, Python, Node.js, WASM, C, C++, C#, Go, Java, R):
- Runner (parallel default / sequential `--no-default-features`):
- OS:

**Additional context**
Anything else (logs, error codes).
