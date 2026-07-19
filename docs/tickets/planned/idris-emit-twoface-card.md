---
needs: []
---
**The Rust→Idris card emitter `idris_emit::emit_card_expr` maps only
`Card::Normal`; a `Card::TwoFaced` returns a "not yet mapped" gap**, so no
double-faced card can be re-emitted to Idris for the `idris-check` cross-check.
Surfaced by `canon-delver` — Delver of Secrets is the first `Card::TwoFaced
{ layout: Transforming }`, and `cargo xtask idris-check plugins/canon "Delver of
Secrets"` reports `FAIL (emitter gap): Card::TwoFaced not yet mapped`.

This is currently SOFT, not a gate failure: `crates/xtask/src/idris_check.rs`
returns `Ok(())` on an emit gap (it reports, never fails — see `:106-121`,
`:216`), and CI does not run idris (idris2 is a local-only gate). Delver's
soundness is carried by the UNCHANGED hand-written model
`idris/src/Cards.idr` (`card_DelverOfSecrets = TwoFaced Transforming (^: {…})
(^: {…})`). So this is coverage of the emit/cross-check loop, not a correctness
hole.

**Distinct from `idris-emittables-drift-control`** (that ticket is about
`EmitTables.idr` `entailments.ron` rows deriving from the model; this is the
Rust card-expression emitter's card-shape coverage).

Fix: extend `emit_card_expr` to map `Card::TwoFaced { layout, front, back }` →
the Idris `TwoFaced <FaceLayout> (^: { … }) (^: { … })`, reusing the existing
per-`CardFace` emit path for each face and mapping `FaceLayout` →
`Transforming`/`ModalDfc`/`Split`/`Adventure`/`Flip`. (Also unblocks emitting
the other DFC layouts as their canon cards land.)

Verify: `cargo xtask idris-check plugins/canon "Delver of Secrets"` EMITS and
typechecks (no gap); the emitted expression matches the hand-written
`idris/src/Cards.idr` `card_DelverOfSecrets`; existing `Card::Normal` emission
unchanged.
