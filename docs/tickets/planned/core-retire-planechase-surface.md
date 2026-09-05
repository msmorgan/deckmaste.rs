---
needs: []
---
**Delete the Planechase surface from core, semantics, lowering, the engine and
the renderers.** Scope ruling (`CLAUDE.md`, "Vintage-playable Magic only"):
Planechase is out of scope permanently, so the model must not model, enumerate,
or reserve room for it. Today it does: `PlanarFace { Blank, Chaos, Planeswalker }`
in `crates/deckmaste_core/src/mana.rs` (mirrored in
`crates/deckmaste_semantics/src/mana.rs`), `Action::RollPlanarDie(Reference)`
and `EventFilter::RollPlanarDie { by, face }` in both crates' `action.rs` and
`event.rs`, plus everything that carries them:

- `crates/deckmaste_lowering`: `mana.rs` (the `PlanarFace` `Lower` impl and its
  three tests), `action.rs` and `event.rs` (the two lowering arms and their
  tests), `minimal.rs` (`minimal_planar_face`).
- `crates/deckmaste_engine`: `eval.rs` (the documented never-matches arm),
  `resolve/player_action.rs` (the no-op special action), `payment/fulfill.rs`.
- `crates/deckmaste_legacy_render`: `render/ability.rs`, `render/effect.rs`.
- `crates/deckmaste_plugin`: `idris_emit.rs` (`emit_planar_face`,
  `opt_planar_face`, both emit arms) and the three `BLOCKED` entries in
  `tests/no_dead_grammar.rs` (`Action::RollPlanarDie`,
  `EventFilter::RollPlanarDie`, `PlanarFace::*`), which are exactly the
  "reserved room" the ruling forbids.
- `crates/deckmaste_semantics/src/identity_registry.rs`: the three
  `RollPlanarDie` identity rows.
- Citations: the deleted comments cite [CR#901.3a,901.8,901.9,901.9a..901.9c,311.7];
  every one of those is also cited by a `done/` ticket or `docs/designations.md`, so
  nothing leaves `cr-citations.lock`. The dice-roll comments in both crates'
  `continuous.rs`, `event.rs` and `action.rs` cite [CR#901.9d] to say the
  planar die is not a numeric roll; with no planar die in the model that
  sentence goes too. `cargo xtask cite check` must report 0 stale and
  `--list-noncompliant` must stay empty afterwards.

The Idris mirror carries the same three names (`RollPlanarDie`, `PlanarFace`
in `idris/src/Semantics.idr` and the `Experimental/*` emit tables); delete
them there too unless `idris-retirement` lands first, in which case they go
with the tree. The Lean workbench never had them (the pin port named its two
Planechase sentences as not ported). The seven `Chaos*` flavor-word stubs
under `plugins/builtin_v2/macros/stubs/flavor_words/` are unrelated: they are
card names (Chaos Warp and kin), not the chaos symbol, and stay.

Why a deletion and not a `#[deprecated]`: the taxonomy dumps (`cargo xtask map
enums`) and the dead-grammar gate both read the enum surface as the engine's
promise; a variant that can never match at runtime is a promise the ruling
says never to make. This also removes the only `Action` arm whose resolution
is a documented no-op, so `resolve/player_action.rs` loses its one
absent-subsystem branch.

Gate: `cargo xtask gate --changed --run` (the shared `deckmaste_core` change
fans out to v1, spelling, migrations, lowering, engine and the plugin gate;
compute the closure, do not hand-list). Assurance: the deleted lowering and
no-dead-grammar tests have no surviving subject, so they are removed, not
re-spelled; name each in the landing record's removed count.
