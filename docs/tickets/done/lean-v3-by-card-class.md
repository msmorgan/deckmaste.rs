---
needs: []
---
# Delete the card-class lint; keywords defined by class select their half with `byCardClass`

Standard constraints apply. Lean first (`lean/Semantics/`). Resolves the ascend STOP of
`lean-v3-card-class-gate-removal` (done) and goes further: the whole card-class lint goes, and a
keyword the CR defines differently by card class says so with a meta-ability.

## Owner rulings (2026-10-22/23), binding

1. **Delete the card-class lint.** Delete the rest of the card-class lint block in
   `Check/Card.lean`: `classAbilityOk`, `StaticSpec.onInstantOrSorceryCardOk`, `cardTextOk`,
   `Refusal.cardText`, `typesCombinable`, and anything else whose only job is policing which
   abilities or types a card class may carry. A keyword or ability on the wrong card class is a
   card-authoring mistake, not a text-meaning error.
   - First check `selfCastAsThoughFlash`: if it tests text meaning rather than card class, keep it
     and say why.
   - Delete `okFlyingGuardedToPermanentCard` and every other pin that only asserts the lint.
     Delete them, do not flip them to accept: a pin that a wrong-class card checks clean asserts
     nothing wanted. Name each pin.
2. **Keep Card Class.** `CardClass` and `cardClassOf` are not deleted. Card Class is a glossary
   term: a permanent card vs an instant-or-sorcery card, by its Card Types [CR#110.4,110.4a]. Move
   them out of the checker into the model next to `CardType` (`Words.lean`) and update their docs.
3. **Add `Ability.byCardClass (onPermanent : Ability) (onInstantOrSorcery : Ability)`**, a
   meta-ability for keywords the CR defines differently by class.
   - Its checker arm (`Check/Phrase.lean`, by the `Ability.check` arms) checks both branches
     against the same incoming DRS, so the `addedCost`-chosen player in gift resolves in both.
   - The engine picks the branch by the card's Card Class; documented in `lean/CONTRACTS.md` and
     on the constructor. Once `lean-v3-card-type-composite-definition` lands, the class is
     computed from its fields.
4. **Ascend** [CR#702.131a,702.131b] becomes `byCardClass (static (.whileHolds cond …)) (spell
   (doIf cond …))`. Both guards go: `onlyIfSo … isPermanentType` and `doIf …
   isInstantOrSorcery`.
5. **Gift** [CR#702.174b] keeps its shared `static (.addedCost …)`, followed by `byCardClass (when
   (does thisPermanent enter) …) (spell (doIf …))`.
6. **Haunt** [CR#702.55a] wraps its two triggers in `byCardClass`. Its spell half keeps its
   resolution cause.
7. **Glossary.** The **Card Class** entry (`docs/contexts/game-model/CONTEXT.md`) says keywords
   defined by class are written with `byCardClass`; "ascend's halves alone are still guarded"
   goes.

## Tests

- Rewritten onto `byCardClass`: the ascend pins `okEachHalfChecksClassBlind`,
  `okBothHalvesOnPermanentCard`, `okBothHalvesOnInstantCard` (`Proofs/KeywordByCardClass.lean`).
- Keep their claims: gift's readback pins (`LinkedChoiceReadback`, `GiftVariants`,
  `declaredGiftTriggerHasInterveningIf`) and the haunt and `DuringResolution` pins.
- Added: `byCardClass` checks both branches (a branch wrong on its own is refused though the other
  is fine); an `rfl` pin of ascend's tree.
- Retired: `okFlyingGuardedToPermanentCard` and every pin that only asserts the lint, each named
  in the landing report.

## Coordination

- `lean-v3-card-type-composite-definition` (planned) assumed `typesCombinable` stays and
  `byCardClass` is pending; its text is reconciled in this landing.
