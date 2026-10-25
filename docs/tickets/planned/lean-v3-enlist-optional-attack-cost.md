---
needs: []
---
# Enlist: an optional cost to attack and its linked trigger

Standard constraints apply.

Enlist means "As this creature attacks, you may tap up to one untapped creature you control that
you didn't choose to attack with and that either has haste or has been under your control
continuously since this turn began. When you do, this creature gets +X/+0 until end of turn,
where X is the tapped creature's power." [CR#702.154a]. Its static ability is an optional cost
to attack [CR#508.1g,702.154b], and its triggered ability is linked to it [CR#607.2h]. `enlist`
(`Definitions/Keywords.lean`) is bodiless.

Decided (from ADR `semantics-v3.md` §0.1 items 4 and 5): extend the cost-to-attack form,
`StaticSpec.gatedBy`, with an `offered` flag. An offered gate is a cost the player may pay as
the creature attacks, not a prohibition unless paid. Pair it with a linked reflexive trigger,
"when you do". No `asEvent`, which would make the cost an effect.

Why it was not done in `lean-v3-keyword-bodies-and-catalog-sync`: the cost's description needs
two predicates the grammar lacks.

- "That you didn't choose to attack with": the cost is chosen at [CR#508.1g], before the chosen
  creatures become attacking creatures [CR#508.1k], so `isAttacking` doesn't say it. It needs the
  declaration's chosen set.
- "Has been under your control continuously since this turn began" as a `Predicate`: it exists
  only as a `Condition` (`heldSince`, `controlledSinceTheirTurnBegan`).

The linked trigger reads the tapped creature's power. The engine counts each enlist instance's
trigger once, for its own cost [CR#702.154d].

Vintage-legal enlist cards: 12 in the MTGJSON snapshot.
