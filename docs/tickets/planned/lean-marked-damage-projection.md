---
needs: [lean-v3-action-clause-spine]
---
**[port-required: semantics-v3] The lethal-damage state-based actions have no
spelling.** Found by `lean-rules-tables` (2026-09-07): [CR#704.5g] reads the
total damage marked on a creature and [CR#704.5h] reads whether a source with
deathtouch has dealt it damage *since the last time state-based actions were
checked*. Neither is spellable: `ProjAxis` has no marked-damage axis, and no
`GameEvent` names a state-based check for a `DidClause` (`LookbackSpan`) window
to end on. So `plugins/builtin/rules/sba/`'s `lethal-damage.ron` is the one v1
row `Proofs/Rules.lean`'s builtin table cannot carry.

Decided (2026-10-13, owner; `semantics-v3.md` §§3.6, 3.8; archived
`semantics-v2.md` §7, Q83, Q90):

- `ProjAxis.markedDamage` is the damage marked on an object [CR#120.3e,120.6];
  `statOf(markedDamage, x)` reads it. The [CR#704.5g] row reads toughness > 0
  and `statOf(markedDamage, this) ≥ statOf(stat(toughness), this)`. Card
  witnesses: Pyramids ("damage marked on") if Vintage-legal and its text fits;
  otherwise one of the "damage dealt to it this turn" faces (Tangled Colony,
  Blazing Effigy).
- `GameEvent.stateBasedActionsChecked` names the check [CR#704.3]. Under the
  `semantics-v3` spine (`docs/decisions/semantics-v3.md` §3.8), the [CR#704.5h]
  row reads `did(by(a(withKeyword(deathtouch))), dealDamage(any, this),
  .duration (.between .stateBasedActionsChecked .now))` (replacing the old
  `LookbackClause` on `damage(...)`).
- Both lethal rows land in the same home as the existing `planeswalkerLoyalty`
  rule (a rule scoped to the creature card type), with "destroy" as the
  instruction; regeneration replacing it is the engine's business, as for the
  existing rows. Deathtouch's declaration stays empty (2026-09-07): its rule
  lives in the table. [CR#702.2c] (combat damage assignment) is a named residue
  for the engine.

**Narrowed (2026-10-23):** `ProjAxis.markedDamage` exists, and the [CR#704.5g] row landed in
`lean-v3-card-type-conferral-rows` as `lethalDamage` on the creature `Definition.cardType` row
(`Definitions/CardTypeRules.lean`), with "destroy" as the instruction. What remains is the
[CR#704.5h] row, which goes on the same creature row.

Do: add the axis and the event with their citations, thread both through every
exhaustive read in `Check/`, pin the two rows as `SbaRule`s in
`Proofs/Rules.lean` beside the three that already prove, mirror both in
`crates/deckmaste_semantics_v3` (when created in `semantics-v3-dsl-and-plugins`),
and note in `lean/CONTRACTS.md` that the "Rules tables" gap is closed. Standard
constraints apply.
