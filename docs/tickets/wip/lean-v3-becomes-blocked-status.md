---
needs: []
---
# "Becomes blocked" is the Blocked status event, not an agentless block

Standard constraints apply. Provenance: row A2 of
[lean-v3-cr-structure-principle-audit](../done/lean-v3-cr-structure-principle-audit.md).

## Ruling (owner, 2026-10-22)

"Becomes blocked" (bushido, afflict, rampage and the like) is the existing
`Action.combat subject (.participation (.blocked true))` [CR#509.1h,509.3c]. `block` stays for
"becomes blocked by a creature" [CR#509.3d]. No new constructor.

"Blocked" is a status an attacking creature gets, including when an effect says it becomes blocked
[CR#509.1h]. "Whenever [a creature] becomes blocked" triggers once per combat, effect included
[CR#509.3c]; "becomes blocked by a creature" triggers once per blocker and never for an effect
alone [CR#509.3d]. With the agentless `block`, *Curtain of Light*'s "Target unblocked attacking
creature becomes blocked" matched no written pattern.

## Verified (2026-10-22)

Paths are under `lean/Semantics/`.

- `Action.combat (subject : NounPhrase) (update : CombatUpdate)` (`Abilities.lean:1117`);
  `CombatUpdate.participation` (`:735-736`); `CombatParticipation.blocked (value : Bool)`
  (`:729-733`). Spelling: `.combat x (.participation (.blocked true))`.
- `combat` is an event head (`Check/Events.lean:34`). The checker steps its subject as the patient
  (`Check/Phrase.lean:3102-3105`); `stepCombatParticipationWith` (`:1158-1161`) does nothing more
  for `.blocked _`.
- Evaluated during verification: bushido, afflict (with "defending player") and rampage (with its
  blocker count) written over the status all check `[]`, and *Somberwald Alpha*'s and *Deeproot
  Warrior*'s readbacks resolve to the same referents as today.
- No Template spells the status event yet. The neighbours are `removeFromCombat`,
  `becomeAttacking`, `becomeBlocking` and `stopBlocking` (`Templates.lean:60-71`).

Sites written as `does any (block …)` that mean "becomes blocked" (rewrite):

- `Definitions/Keywords.lean:96` bushido, `:200` afflict, `:1259` rampage.
- `Cards/Trigger.lean:86` *Somberwald Alpha* ("a creature you control becomes blocked"), `:132`
  *Deeproot Warrior*, `:213` *Chub Toad*.

Sites that stay on `block` with a `by` subject [CR#509.3d]: flanking
(`Definitions/Keywords.lean:718`), the Ring's level-3 ability (`Definitions/Actions.lean:777`),
`Cards/Trigger.lean:82,372`, `Cards/Damage.lean:273`, `Proofs/Counters.lean:88,96,104`. Deontic
`cant any (block …)` sites mean declaring blockers and stay.

## Change

1. Add a Template for the status event beside `becomeBlocking` (`Templates.lean:60-71`), e.g.
   `becomeBlocked x := .combat x (.participation (.blocked true))`, documented with
   [CR#509.1h,509.3c]. The name is the implementer's; it carries the printed wording.
2. Rewrite the six sites above to `does any (becomeBlocked …)`.
3. ADR `docs/decisions/semantics-v3.md:307` (trigger row "becomes blocked" →
   `does(any, block(this))`) and `:411` (combat row): "becomes blocked" is the status event;
   `does(by(b), block(a))` is "becomes blocked by".
4. Adjacent, decide in the landing and say which: *Fyndhorn Druid*'s "if it was blocked this
   turn" (`Cards/Trigger.lean:410`, `was (block it) soFarThisTurn`) and the ADR lookback row
   (`semantics-v3.md:310`) read the same status. Being blocked by an effect counts for "was
   blocked" too [CR#509.1h], so the lookback should move with the trigger unless a reason is
   found.

## Tests

- Keep passing: `okBushidoWithBushidoDefinition` (`Proofs/Faces.lean:513-515`),
  `okSomberwaldAlpha` (`Cards/Trigger.lean:88`), `okDeeprootWarrior` (`:134`),
  `somberwaldAlphaPumpsItsBlockedCreature` and `deeprootWarriorPumpsItsOwnBlockedSource`
  (`Proofs/GetsBothDeltas.lean:152,156`). No test pins the old `block` tree of these keywords.
- Add `becomesBlockedIsTheStatusEvent`: bushido's trigger event is the `.blocked true` combat
  update (`rfl` on the definition).
- Add `becomesBlockedByStaysABlock`: flanking's event is still `does (by …) (block …)`.
- Add a pin that an effect that makes a creature blocked (*Curtain of Light*'s instruction,
  `combat (target …) (.participation (.blocked true))`) and bushido's event pattern use the same
  head, so the engine can match one against the other.

Noted, not in scope: the checker accepts meaningless voices on the status event
(`does thisCreature (combat thisCreature …)`, `does (by_ …) (combat …)`). If the landing wants
them refused, that is a new refusal and needs a dated ruling (`lean/CONTRACTS.md` front matter).

## Other tickets on the same files

- `lean-v3-keyword-class-guards-and-observation` and `lean-v3-alternative-cost-unification` edit
  other hunks of `Definitions/Keywords.lean`.
- `lean-v3-pay-life-action` edits `docs/decisions/semantics-v3.md:409`.
- `lean-v3-cast-predicates-and-ordinals` edits `Cards/Trigger.lean` (other cards).
