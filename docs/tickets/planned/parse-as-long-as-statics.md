---
needs: []
---
Parse conditional statics — `~ gets +2/+2 as long as you control an artifact.`,
`As long as ~ is attacking, it gets +2/+0.`, `~ has vigilance as long as
there's a Lesson card in your graveyard.` — into
`StaticEffect::Conditionally(Condition, StaticEffect)`, which already exists in
the core grammar (`crates/deckmaste_core/src/continuous.rs`). Both orders
(condition-first and condition-last), and the ability-word-prefixed forms
(`Threshold — As long as …`) that reduce to the same shape.

The condition phrase should route through the `Condition`-kind macro path
(`parse_if` → `match_kind("Condition", …)`) so new condition phrasings stay
data-authored; the static body reuses the existing static-ability productions
(`parsers/static_ability.rs`) — this ticket is the composition of the two,
not new condition or static vocabulary.

Note: `Conditionally` is currently UNWIRED in the layers engine (see its doc
comment) — execution is `engine-conditional-statics-wiring`. Graduation is
parse-coverage only, so this ticket does not depend on it.

**~642 of 17,022 one-away cards** (2026-07-16 tally). Second-largest
parse-only family.

Verify: `cargo xtask generate plugins/wizards` graduation delta +
`cargo xtask fidelity` on a condition-first and a condition-last card.
