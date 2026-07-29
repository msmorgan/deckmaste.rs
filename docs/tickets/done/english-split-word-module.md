---
needs: [english-structural-recovery-zero]
---
Split `crates/deckmaste_english/src/word.rs` into focused grammatical modules
after the chart-parser replacement has landed. (Gated 2026-07-24 on the
recovery campaign: rounds add senses to `word.rs` regularly, and a file split
mid-campaign buys conflicts for nothing.)

## Why

`word.rs` currently owns semantic word types, the vocabulary declaration,
noun and verb morphology, closed-class words, reverse indexing, and tests. The
single file is cumbersome to navigate and will keep growing as Oracle syntax
coverage expands.

## Scope

- Move noun, verb, modifier, pronoun/auxiliary, and reverse-indexing concerns
  into focused `word/` submodules.
- Keep one `Vocab` declaration and one definition per lexical identity. A word
  such as `target` may occupy several grammatical slots, so the split must not
  create per-part-of-speech vocabulary registries or duplicate inflection
  metadata.
- Preserve the public `crate::word` API with re-exports where practical.
- Preserve slot-directed lookup: the lexer must not choose a part of speech.
- Make no parsing, rendering, morphology, or catalog-behavior changes as part
  of the refactor.

## Gate

`cargo test -p deckmaste_english` passes; the supported-card round-trip and
unknown-phrase gates are unchanged; `cargo clippy -p deckmaste_english
--all-targets -- -D warnings` is clean.

## Completion (2026-07-28)

- Split noun, verb and predicate-frame, modifier, closed-class, and reverse
  index concerns into five `word/` submodules. `word.rs` retains the shared
  `Vocabulary` type, public re-exports, and the single `Vocab` declaration.
- `cargo test -p deckmaste_english` passes (576 unit tests and 115 public API
  tests), and all 31,685 supported faces round-trip cleanly.
- The unknown-phrase census is identical to the parent line: 4,894 unresolved
  spans, including 3,414 longer than three words, with the same ranked top 20.
- Strict clippy currently reports the same pre-existing warnings on the parent
  and feature lines in grammar, renderer, and public-API test code. With that
  parent warning set allowed, `-D warnings` is clean, including every moved
  `word` module.
