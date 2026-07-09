---
needs: []
---
Extend the `no_dead_grammar` test's `GRAMMAR_FILES` sweep to cover
`crates/deckmaste_core/src/{selection.rs, filter.rs, mana.rs}`, then allowlist (or fixture-cover)
the grammar constructors those files declare that no canon card yet exercises.

## Why

`crates/deckmaste_cards/tests/no_dead_grammar.rs` guarantees every grammar constructor is either
exercised by a canon fixture or carries a `DEFERRED` allowlist entry — but only for the files
listed in its `GRAMMAR_FILES` array (~7 files). `selection.rs`, `filter.rs`, and `mana.rs` are
NOT in that list, so constructors declared there are invisible to the guarantee: they can be dead
(no fixture, engine-inert) without failing the sweep, and any future unused `Selection` / `Predicate`
/ `ManaSpec` variant will silently slip through.

Concretely, these already-landed constructors are currently dead-and-invisible:
- `filter.rs`: `Predicate::WasPutFrom(Zone)` (emit-wired + unit-tested, but no fixture, engine fizzles).
- `mana.rs`: `ManaSpec::AmongColorsOf` and `enum PlanarFace`'s faces (no fixture; Chrome Mox and the
  plane subsystem are blocked). Note `ManaSpec::ProducedByEvent` is now covered by the Dictate of
  Karametra fixture, so it will NOT need an allowlist entry once the sweep extends.

## Scope

1. Add `selection.rs`, `filter.rs`, `mana.rs` to `GRAMMAR_FILES`.
2. Run the sweep; for every newly-surfaced unused constructor (the ones above plus any pre-existing
   unused `Selection`/`Predicate`/`ManaSpec` variants the wider net catches), add a `DEFERRED`
   allowlist entry with an honest reason, or author a real-card fixture that exercises it.
3. Expanding the net may surface a batch of pre-existing dead variants — triage each (fixture vs
   DEFERRED); do not blanket-allowlist.

## Gate

`cargo test -p deckmaste_cards --test no_dead_grammar` green with the three files added.
