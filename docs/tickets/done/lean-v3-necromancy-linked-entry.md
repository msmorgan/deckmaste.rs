---
needs: []
---
# Necromancy: "enchant creature put onto the battlefield with Necromancy" and its leaves trigger

Standard constraints apply. Lean first (`lean/Semantics/`). Provenance: the tenth card of
[lean-v3-flash-conditional-sacrifice](../wip/lean-v3-flash-conditional-sacrifice.md), which landed
its shared first paragraph (`asThoughFlashSacrificedAtCleanup`) on the other nine. Witnesses:
*Necromancy*; *Animate Dead* and *Dance of the Dead* if they share the shape.

Settled form (owner, 2026-10-23):

- "When this enchantment leaves the battlefield, that creature's controller sacrifices it" is a
  delayed trigger the ETB creates as it resolves [CR#603.7a,603.7c]: the third step of the ETB's
  `sequentially`, after `putOntoBattlefieldUnderYourControl (target …)` and
  `attachTo thisEnchantment it`, as
  `delay (does thisEnchantment (leave battlefield)) (instruct (controllerOf (that creature)) (sacrifice it))`.
- "Enchant creature put onto the battlefield with Necromancy" [CR#607.2c] is
  `enchant (and [isCreature, did (by_ this_) (putOntoBattlefield which)])`.

What blocks it (found 2026-10-23):

- The premise that `DidClause.participialAgentOk` (it lives in `Check/Phrase.lean`, not
  `Check/Words.lean`) admits only `.any` is false: it already admits `.by`/`.actor` naming you, the
  performer, `this_` or a `headed _ .this` source. No relaxation is needed for `by_ this_`.
- The real refusal is the anchoring. Checked as the ETB above, the enchant quality gives
  `[unanchoredLookback, unanchoredLookback, participialDescription (zoneChange none battlefield) 0]`:
  enchant steps its quality as a bare predicate (`addsTarget`, `mayBeAttachedTo`), where a
  span-less historical clause is refused, and the granted ability is stepped before the put it
  reads back, so no linked object is published to it. The fix is a checker reading of a
  [CR#607.2c] "put onto the battlefield with [this object]" description inside a granted keyword's
  quality, linked to the put of the ability that grants it.
