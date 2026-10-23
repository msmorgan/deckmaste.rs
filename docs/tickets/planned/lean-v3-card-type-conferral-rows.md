---
needs: [lean-v3-card-type-composite-definition]
---
# The card types' and subtypes' rules go on their declared `Definition` rows

Standard constraints apply. Lean first (`lean/Semantics/`). The rest of clause (1) of ruling #1 of
the owner's batch 4 (2026-10-22), deferred by `lean-v3-card-type-composite-definition` (B5).

## Where B5 left it

- `CardType` is a structure (`name`, `permanent`, `stats`, `instantOrSorcery`), tagged
  `[semantic_expression, internal_expansion]`; the ten card types are `semantic_definition
  CardType.x` in `Definitions/CardTypes.lean`, a leaf over `Semantics.Words` that the checker
  imports.
- Each card type has a `Definition.cardType` row in `Definitions/CardTypeRules.lean` (above
  Templates). The rows carry only the cast or play permission from hand
  [CR#301.1,302.1,303.1,304.1,305.1,306.1,307.1,310.1] and the creature's `combatant`
  [CR#508.1a]; kindred and dungeon carry nothing [CR#308.1,309.2c].
- `Definitions/Subtypes.lean` has no `Definition.subtype` rows.

## Ruled (owner, 2026-10-23, STOP-2 of B5)

Conferrals never go on the `CardType` or `Subtype` value. A card type's go on its
`Definition.cardType` row; a subtype's go on a `Definition.subtype` row (`Rules.lean`,
`rules : List Conferral`). The rows sit above Templates: `Definitions/Subtypes.lean` is a leaf over
Primitives, so subtype rows need their own file above Templates, as `CardTypeRules.lean` is.

## Change

Add to the declared rows, over shared templates, the rules the ruling lists (citations read
against `data/rules/cr.txt` in B5):

- **creature:** attack and block [CR#302.5]; summoning sickness [CR#302.6]; takes marked damage
  [CR#120.3e]; the toughness and lethal-damage state-based actions [CR#704.5f,704.5g].
- **planeswalker:** enters with loyalty [CR#306.5b]; can be attacked [CR#306.6]; damage removes
  loyalty [CR#120.3c]; the loyalty state-based action [CR#704.5i].
- **battle:** enters with defense [CR#310.4b]; can be attacked [CR#310.5]; damage removes defense
  [CR#120.3h]; the non-Siege state-based action [CR#704.5w,310.8]; the chosen protector
  [CR#310.9a].
- **Siege** (`Definition.subtype` row): the opponent protector [CR#310.12a], the last-counter
  trigger [CR#310.12b], and its state-based action [CR#704.5v].
- **Saga** (`Definition.subtype` row): chapters, lore counters and the sacrifice rule, from
  `sagaDefinition` [CR#714.3a,714.3c,714.4].

Several already exist as bench rows in `Proofs/Rules.lean`, written as scoped `SbaRule` and
`ConferralRule` rows: `toughnessZero`, `loyaltyZero`, `battleDefenseZero`, `planeswalkerLoyalty`,
`battleDefense`, `creatureDamageMarking`, `planeswalkerLoyaltyDamage`, `battleDefenseDamage`,
`sagaDefinition` and the Siege guard (`okSiegeGuard`). Move each onto its declared row as the
matching `Conferral` (`stateBased`, `ability`, `property`), and repoint its pin at the declared
row. The protector choice, attack and block, and summoning sickness have no bench row yet.

## Tests

- Each new row checks (`Definition.check … = []`), with its CR citation.
- The pins of the moved bench rows read the declared rows, their claims unchanged.
