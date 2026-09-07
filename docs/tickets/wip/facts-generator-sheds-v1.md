---
needs: []
---
**The registry declarations are typed by the Lean facts, not by v1.**
Ruling (user, 2026-09-07). `cargo xtask facts generate` writes
`lean/Semantics/Check/Facts.lean`, the tables Lean's laws read, from the
`plugins_v2/builtin` declarations plus hand overlays. It deserialises the
designation, counter, type, and subtype declaration `body` fields through
v1 `deckmaste_semantics` structs (`DesignationDecl`, `DesignationDef`,
`DesignationScope`, `DesignationShape`, `CounterScope`, `Bearing`,
`ron::options`), because those bodies were written in the english_v2 era
before any v2 semantics type existed. v1 is deletion-bound and v2 must not
read through it (`CLAUDE.md`, Crate fates).

The shape, since Lean is the spec: `lean/Semantics/Check/FactTypes.lean`
already declares the columns (`KeywordFacts`, `DesignationFacts`, the
counter and subtype rows). Mirror it into `deckmaste_semantics_v2` as a
`facts` module (name-for-name, under the drift test's discipline; it is
`Check/`-side, so extend the drift test's file list). Make the registry
declaration bodies deserialise into that mirror — a declaration carries
its own columns — and have the generator emit `Facts.lean` from the mirror
through the v2 reader. Overlays survive only for columns no declaration can
carry (state that list); the rest are deleted with their code. Delete the
`deckmaste_semantics` import from `crates/xtask/src/facts/`. `facts check`
shows both generated tables unchanged (the Idris reference is byte-stable).
The declaration bodies are rewritten by load-and-reserialise, not by hand,
and `plugins-v2-dialect` converts them again afterwards, so land this
first. Standard constraints apply.
