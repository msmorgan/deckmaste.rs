---
needs: []
---
**`ActFacts` and its column types are still mirrored as generated registry
facts, and no generated table uses them any more.** Raised by
`facts-act-columns-are-lean-data`, which moved the `actFacts` table out of the
generated module: `Check/FactTypes.lean` still declares `ActFacts`, `DeedRole`,
`ReferentSort`, `EntityDomain`, `ObjectClass`, `PremiseSort` and `DeedFeature`,
and `crates/deckmaste_semantics_v2/src/facts.rs` still mirrors all seven,
though no Rust code reads any of them and every table that carries them
(`coreDeedFacts`, `abilityDeedFacts`, `actFacts`) is now hand-written in
`Check/Words.lean`.

`tests/lean_drift.rs` pins `Check/FactTypes.lean` against `facts.rs` in both
directions, so the Rust mirror cannot be dropped alone: the seven declarations
would have to move to a Lean module the drift law does not cover (the checker's
own `Check/Words.lean` is the obvious home). That changes which Lean modules
the mirror contract covers — `docs/decisions/semantics-v2.md` §10 — so decide
it there rather than in a generator ticket. Standard constraints apply.
