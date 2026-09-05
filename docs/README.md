# docs/

A signpost, not a documentation tree.

Rust API docs are on [docs.rs](https://docs.rs/benchmark-core), built with every
feature on. The per-language quickstarts live in each binding's own README under
[`bindings/`](../bindings/), next to the code they describe:

| | | |
|---|---|---|
| [Rust](https://docs.rs/benchmark-core) | [Python](../bindings/python/README.md) | [Node.js](../bindings/node/README.md) |
| [WebAssembly](../bindings/wasm/README.md) | [C and C++](../bindings/c/README.md) | [C#](../bindings/csharp/README.md) |
| [Go](../bindings/go/README.md) | [Java](../bindings/java/README.md) | [R](../bindings/r/README.md) |

## What is kept here, and why

Five documents live in this directory rather than anywhere else, because each
describes something the repository *owns* — and that a release can change:

- **[CASES.md](CASES.md)** — what a case is, field by field, and what makes one
  worth curating. The prose counterpart to the files in
  [`cases/`](../cases/).
- **[DATASETS.md](DATASETS.md)** — how the candle series are generated and why
  they carry no market data, plus the blake3 manifest that pins them.
- **[HASHING.md](HASHING.md)** — the canonicalization every hash is taken over.
  This is the document a second implementation would be written against, so it
  is the one that must not drift: `crates/benchmark-core/src/hash.rs` is the
  normative version and this explains it.
- **[REPRODUCING.md](REPRODUCING.md)** — the command surface, and how to
  reproduce a case from each of the ten languages.
- **[Cookbook.md](Cookbook.md)** — short recipes, including gating a build on a
  suite that must still reproduce.

A site aggregating the same quickstarts is built from
[wickra-benchmark-site](https://github.com/wickra-lib/wickra-benchmark-site). It
is not linked from here yet: `benchmark.wickra.org` does not resolve until the
DNS record and the Cloudflare Pages project point at that repository, and a link
to a host that answers nothing is worse than no link.

## What does not belong here

Anything that duplicates a binding README or the rustdoc. A second
documentation tree is the failure mode this file exists to prevent: it drifts
from the first one, and nobody notices, because both look maintained.

If a page would describe *how to call the library from a language*, it belongs
in that binding's README. If it describes *what a case is, or what the engine
must reproduce*, and a release can change the answer, it belongs beside the
code — here.
