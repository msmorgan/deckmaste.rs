---
needs: []
---
# Delete the refusals that police CR classifications rather than text

Standard constraints apply. Found by the 2026-10-25 triage of an external audit ("engine
creep"), verified against `wmyykqnv`. Paths are under `lean/Semantics/`. Every edit below was
applied to a scratch copy of `lean/` on 2026-10-25 and `lake build --wfail` passed (diff:
`/tmp/claude-1000/-home-msmorgan-Projects-deckmaste-rs/4e5532ee-6640-4aad-9769-888a72f918d1/scratchpad/leanprobe/probe.diff`).

The workbench represents what cards can say; a gameplay distinction stays only where reference
resolution needs it (owner direction, 2026-10-25). Whether an ability meets [CR#604.3a]'s
criteria for a Characteristic-Defining Ability, or whether a resolving permanent spell's
"return/exile … at the beginning of the next end step" is a delayed trigger [CR#608.3g], is a
classification the engine makes. No anaphor depends on either check. The constructors stay:
`StaticSpec.characteristicDefining` has its own printed wording ("[This creature's] power is
equal to …" [CR#208.2a]) and carries anaphora (*Tarmogoyf*'s "that number plus 1" reads the
named number only that arm pushes); `StaticSpec.resolutionDelayedTrigger` and
`NounPhrase.becomes` are how dash, blitz and warp say "the permanent this spell becomes", and
`becomes` is printed (*Mystic Veil*, *Spider Climb*). This reverses the decision of
`docs/tickets/done/lean-v3-authoring-impl-14-ast-and-authoring-cleanup.md` §1c (`:65-80`) to
enforce the CDA criteria.

Two parts, each separately committable.

## Part 1: the four CDA refusals

Delete:

1. `Check/Refusal.lean:227-239`: the header `/- characteristic-defining abilities [CR#604.3a] -/`
   and `cdaEdit`, `cdaSubject`, `cdaGranted`, `cdaConditional` with their docstrings.
2. `Check/Phrase.lean:1726-1758`: from the docstring "An edit a Characteristic-Defining Ability
   may write:" up to, not including, "/-! ## Retention: an exemption from removing Attachments":
   `CharacteristicEdit.definesOk` (`:1726-1737`), `StaticSpec.writesDefining` (`:1739-1747`),
   `Ability.writesDefining` (`:1749-1757`). Each is used only by the raises below.
3. `Check/Phrase.lean:4098` (the `establish` arm): `(drs2, e1 ++ e2 ++ refuse (!spec.writesDefining)
   .cdaGranted)` → `(drs2, e1 ++ e2)`.
4. `Check/Phrase.lean:4519-4526` (the `characteristicChange` arm): delete the comment at
   `:4519-4520` and the `let granted := …` binding at `:4521-4523`; change `:4526`
   `CharacteristicEdit.shapeAll edits ++ refuse (!granted) .cdaGranted)` →
   `CharacteristicEdit.shapeAll edits)`.
5. `Check/Phrase.lean:4538-4539`: delete `refuse subj.selfDefinedOk .cdaSubject ++` and
   `refuse (edits.all CharacteristicEdit.definesOk) .cdaEdit ++`.
6. `Check/Phrase.lean:4655` (the `conditional` arm): `(drs2, e1 ++ e2 ++ refuse
   (!spec.writesDefining) .cdaConditional)` → `(drs2, e1 ++ e2)`.

Must not change: the `characteristicDefining` arm's named-number push
(`Check/Phrase.lean:4527-4537`); `NounPhrase.selfDefinedOk` (`:743`, also used at `:1064` and
`:3576`); `StaticSpec.definedStats`, `Ability.definesPt`, `textDefines` and `ptSlotOk`
(`Check/Card.lean:90-115,138-139`), which tie a printed `*` to its CDA; `ptDefinition`,
`ptDefinitions`, `qualityDefinition` (`Templates.lean:2053-2074`).

Pins, `Proofs/CharacteristicEdits.lean`:
- Retired, because each asserts only a deleted refusal: `definingAbilityDefinesOnlyColorsSubtypesAndStats`
  (`:146-151`), `definingAbilityDefinesItsOwnObject` (`:153-158`), `definingAbilityIsUnconditional`
  (`:179-184`), and conjuncts 1-2 of `definingAbilityIsNeverGranted` (`:160-170`).
- Rewritten: conjunct 3 of `definingAbilityIsNeverGranted` becomes its own theorem:
  ```lean
  /-- Granting a keyword whose definition writes a Characteristic-Defining Ability (changeling)
  is an ordinary grant [CR#604.3a]. -/
  theorem changelingGrantIsAnOrdinaryGrant :
      StaticSpec.check (DRS.ofList []) (abilityGrant thisCreature changeling) = [] := by decide
  ```
- Must stay green: `copyExceptionMayGiveADefiningAbility` (`:174`), `definingAbilityDefinesPower`
  (`:103`), `tarmogoyfDefinesBothStats` (`:110`), `definingAbilitiesDefineColorsAndSubtypes`
  (`:120`), `choiceDependentSubtypeDefinitionIsAccepted` (`:132`),
  `tokenIsCreatedWithADefiningAbility` (`:140`), `ptSwitchIsAnEdit`.

Docs:
- `Abilities.lean:976-985` (the `characteristicDefining` docstring): delete "Written as a printed
  static ability, unconditionally, and never inside a grant or an established effect: the checker
  refuses a subject other than the object itself, a condition, and a grant [CR#604.3a]."
- `lean/CONTRACTS.md:985-990`: delete from "It is the one game rule the checker enforces outside
  the card frame …" through "… a condition on it [CR#604.3a]."; keep "Its stat edits each name
  their number…".
- `docs/decisions/semantics-v3.md:620` (Checker Charter): delete ", that a Characteristic-Defining
  Ability meets the criteria of `[CR#604.3a]` (`cdaEdit`, `cdaSubject`, `cdaGranted`,
  `cdaConditional`)".

## Part 2: `createsDelayedTrigger`

Delete:

1. `Check/Refusal.lean:163-165`: the docstring and `| createsDelayedTrigger`.
2. `Check/Phrase.lean:2303-2312`: the docstring and `def Action.createsDelayedTrigger` (no other
   caller), and the blank line after it.
3. `Check/Phrase.lean:4685`: `(drs1, e1 ++ refuse does.createsDelayedTrigger
   .createsDelayedTrigger)` → `(drs1, e1)`.
4. `Abilities.lean:1053`: delete the sentence "The checker refuses a `does` that is not a delayed
   trigger (`createsDelayedTrigger`)." and keep the comment's closing `-/` on the line before.

Pins, `Proofs/OffStack.lean`:
- Rewritten: `resolutionDelayedTriggerCreatesOne` (`:229-237`) keeps only its first conjunct,
  as
  ```lean
  /-- "Return the permanent this spell becomes to its owner's hand at the beginning of the next
  end step": a Resolution Delayed Trigger's body reads the permanent the spell becomes
  [CR#608.3g,702.109a]. -/
  theorem okResolutionDelayedTrigger :
      StaticSpec.check (DRS.ofList [])
          (resolutionDelayedTrigger (atNext .endStep (returnToHand (becomes thisSpell)))) = [] := by
    decide
  ```
  Its second conjunct asserted the deleted refusal and is retired.
- Must stay green: `delayedTriggersWaitForThePermanent` (`:208`), `blitzSacrificesThePermanent`
  (`:223`), `okDashBlitzWarp` (`:239`), `okBecomesThisSpell` (`:50`), `badBecomesOfAPlayer`
  (`:54`).

## Acceptance

- `./lean/scripts/build` (`--wfail`), `cargo xtask lean check`, the CR citation gates.
- `grep -rn 'cdaEdit\|cdaSubject\|cdaGranted\|cdaConditional\|writesDefining\|definesOk\|createsDelayedTrigger'
  lean docs/decisions` finds nothing.
