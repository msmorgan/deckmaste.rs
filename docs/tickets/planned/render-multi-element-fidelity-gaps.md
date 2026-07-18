---
needs: []
---
Several render helpers collapse a multi-element structure to its FIRST element,
silently dropping the rest. All are LATENT today (no canon card carries the
multi-element shape) but each is a byte-exact fidelity landmine the moment one
does; #3 below is self-documented in the code as "safe today only because" of an
authoring convention. parse⇄render must stay one rule, so prefer fixes that keep
the mirror.

Sites (all render-side, `crates/deckmaste_cards/src/render/`):

1. `fragment.rs::adjective_adjunct` (~642) returns only the first combat/tap
   state adjective — `And([Creature, Status(Untapped), Attacking])` renders
   "attacking creature" (drops "untapped"), never "untapped attacking creature".
2. `fragment.rs::find_bare_subtype_noun` (~1005) returns the first bare
   `Subtype` via `find_map` — `And([Permanent, Subtype("Goblin"),
   Subtype("Warrior")])` renders "Goblin" (drops "Warrior").
3. `fragment.rs::filter_noun` or_else ordering (~547): `find_card_type` is tried
   BEFORE `find_bare_subtype_noun`, so a `Creature` + `Subtype` filter renders
   the type noun and drops the subtype ("creature" not "Goblin"). The code's own
   doc comment (~996) flags this as safe-today-only, resting the noun's
   correctness on an authoring convention rather than the code.
4. `ability.rs::trigger_limit_rider` (~183) appends one rider sentence per entry
   in `t.limits` (a `Vec<UseLimit>`), so >1 limit prints multiple (possibly
   contradictory) sentences the parser's single-suffix `peel_trigger_limit`
   cannot round-trip. The parser emits ≤1 limit today.

Fix: render every present element — join multiple state adjectives / subtypes in
authored order; try the more-specific subtype noun before (or merged with) the
type noun; render at most the parser-representable single trigger limit (and
decide the multi-limit surface if/when one is authored). Keep each fix mirrored
by its parser so parse⇄render stays bidirectional.

Verify: `cargo xtask fidelity` (or the render round-trip tests) on hand-built
filters carrying two state atoms / two subtypes / `Creature`+`Subtype` / two
trigger limits round-trips byte-exact.

(Bundle of four related latents surfaced by the code review on
`engine-block-legality-query`; the findings are in the for-each-pump render
work, not that feature. The possessive "its owner's vs your" finding at
`fragment.rs::move_possessive` was intentionally excluded — it matches the
accepted `done/bounce-followups` §1 design and only an unmodeled owner-atom edge
misfires.)
