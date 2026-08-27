---
needs: []
---
# Per-type ability licensing for command-zone cards

Routed from `workbench-card-class-and-command-zone` (close, 2026-08-26).
`cardAbilityOk CommandZoneCard` licenses by CLASS, so it overgenerates
per-type refusals: an activated ability on a conspiracy card ([CR#315.5]
gives conspiracies statics), or anything beyond [CR#309.4c]'s room triggers
on a dungeon card, is admitted. Refusing those needs `cardAbilityOk` to see
the card TYPE, not the class — deliberately not taken by the parent round.
Weigh a per-type row against the tolerated-overgeneration doctrine before
building: each refusal needs its rule named.

## Consumption boundary

`idris/src/Experimental/Card.idr`, `Words.idr`; `Cards.idr` bench;
`Proofs*.idr`. No Rust crate.

## Acceptance

- Each per-type refusal lands with its rule or is named at its zero.
- `idris/scripts/build` PASS, no witness lost, no pin silently passing.

Standard constraints apply.

## As landed

The weighing came out FOR building, on the rule and not on the count.
`cardAbilityOk` now takes the card's `List CardType` rather than its
`CardClass`, and asks two questions instead of one: the FRAME's licence
(the old table, renamed `classAbilityOk`, unchanged) and the card's own
TYPES' (`commandZoneTypeAbilityOk`, folded over the line by
`commandZoneTypesAbilityOk` because [CR#300.2] lets a card name more than
one type).

**Refusals landed, each with its rule:**

- **Conspiracy — no activated ability.** [CR#315.5]: a conspiracy card "may
  have any number of static or triggered abilities", and only those function
  from the command zone. The shared frame admits an activated one because
  [CR#311.4], [CR#313.4] and [CR#314.4] each name one; this rule does not.
- **Dungeon — triggered abilities only.** [CR#309.4c] writes out the full
  text of every ability a dungeon card has (one room ability per room, each
  triggered) and licenses their triggering and nothing else. So no static, no
  activated, and no keyword ability.

**Cells named at their zero, not shut:**

- **Plane, vanguard, scheme** — [CR#311.4], [CR#313.4] and [CR#314.4] each
  give "any number of static, triggered, and/or activated abilities", which
  is the shared frame verbatim. Nothing to narrow.
- **Phenomenon — left OPEN deliberately.** [CR#312.5] says each phenomenon
  card HAS the encounter trigger; that is a fact about what such cards carry,
  not a list of what they may carry. It is the only rule about a phenomenon's
  abilities, and it neither licenses nor refuses a static or an activated
  one — stating no licence is not refusing one. Recorded on the row rather
  than shut on a rule that does not say it.

**Measurement, recorded on the row.** Of the 443 cards in
`data/derived/cards.jsonl` carrying one of the six command-zone types — 29
conspiracies, 21 phenomena, 184 planes, 102 schemes, 107 vanguards, and no
dungeon card at all — **not one is supported**. Both refusals therefore
refuse a line no vintage-playable card writes. They are taken anyway because
a RULE refuses them: the tolerated-overgeneration doctrine records an
overgeneration no rule refuses at its zero, and closes one a rule refuses
whatever the count. No card of any of these types is benched, and none is
added: the round's only new terms are two `Unspellable` pins whose whole
content is that the grammar now says no.

**Pins** (`ProofsG.idr`): `badConspiracyActivated` and `badDungeonStatic`.
Both are refused by the per-type gate alone — the frame admits each shape
(`classAbilityOk CommandZoneCard` returns `True` for a static and for an
off-battlefield-cost activated ability), so the pin cannot pass for the wrong
reason.
