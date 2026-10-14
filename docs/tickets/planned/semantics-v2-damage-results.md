---
needs: []
---
**Damage results become one row shape, with keywords as statics.** Replaces the
narrow `DamageResultRule`. Decided 2026-10-13 (owner; `semantics-v2.md` §7,
Q86, Q87, Q88). Standard constraints apply; expect conflicts at refresh with
`lean-stat-projections` (both touch `Rules.lean`, `Proofs/Rules.lean`, Words
and the Rust mirror).

## Shape (Q86)

`DamageResult (kind : DamageKind) (source : DamageAgent) (recipient : Predicate)
(mode : ResultMode) (does : Instruction)` [CR#120.3,120.4c]. Inside `does`,
`thatMuch` is the damage amount and the performer is the source's controller,
or its owner if it has no controller [CR#702.15b], read in context (the
2026-10-06 performer ruling; no performer field). The two existing rows
(planeswalker loyalty [CR#120.3c], battle defense [CR#120.3h]) re-spell into it
and their pins re-spell with the same outcome. `DamageResultRule` and its Rust
mirror retire.

## Home (Q87)

`StaticSpec.damageResult (rule : DamageResult)`: a keyword's result is its own
static ability [CR#702.80a], written `static(damageResult(...))`, and card
statics write the same node. The builtin rules table (`RulesTables.damageResult`)
holds the intrinsic rows of the same type: life loss [CR#120.3a], loyalty
[CR#120.3c], marking [CR#120.3e], defense [CR#120.3h]. Marking needs an
instruction that marks damage, the counterpart of `clearDamage` [CR#120.6]; name
it in the names table.

## What "instead" replaces (Q88)

`ResultMode = intrinsic | instead (of : OutcomeSort) | inAddition`. `intrinsic`
is only for the builtin rows. `instead(of)` names the result it replaces
(`instead(damageMarked)`, `instead(lifeLost)`; `OutcomeSort` gains
`damageMarked`) and applies only while that result would still happen, so a
source with both wither and infect puts counters once [CR#120.3d]; `inAddition`
adds. Archon of Coronation's "damage doesn't cause you to lose life" is
`instead(lifeLost)` with an empty instruction.

## Bodies

- Lifelink: `inAddition`, gain that much life [CR#702.15b].
- Wither: creature recipient, `instead(damageMarked)`, put that many -1/-1
  counters [CR#702.80a].
- Infect: two rows, wither's creature row and a player row `instead(lifeLost)`
  giving that many poison counters [CR#702.90b,702.90c].
- Toxic: **STOP (waits on: Q89, toxic's amount).** Not ruled; do not implement
  or pick an option. [CR#702.164c] gives "a number of poison counters equal to
  that creature's total toxic value", the sum of N over all toxic instances
  [CR#702.164b]. Options: A, each `toxic N` instance is its own `inAddition` row
  giving N counters, additive rows stacking to the sum with no new node (the
  orchestrator's recommendation); B, an `Amount` reading the total of a
  keyword's numbers on an object. Ask the owner.

## Canon witnesses

Each Vintage-legal with matching text: one lifelink card, one wither card, one
infect card, and the statics Everlasting Torment and Phyrexian Unlife ("as
though its source had wither/infect"; decide whether that phrase composes the
keyword's rows by reference or is written out, and say which), plus Archon of
Coronation and Soul-Scar Mage if they fit.
