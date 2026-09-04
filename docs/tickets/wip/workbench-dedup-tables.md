---
needs: []
---
**Dedup the identical predicates, the positional facts row, the string-literal
deed guards and the reach case splits.** Cleanroom review 3, 2026-09-04,
findings G3, G4, G5, G6, G8, G9, G10, G11 and D-Q17. Independent edits.

- **G3 — identical predicates.** `Words.statusEventOk`, `Words.statusEffectOk`
  and `Words.statusHeaderOk` share one table (`FaceDown = True` duplicates the
  fallthrough) — collapse the `StatusEventVal`/`StatusEffectVal`/
  `statusHeaderOk` triplet into one `StatusMarkable`.
  `Events.partTriggerOk`, `Events.partAddable` and `Words.beforePartOk` are
  each `Turn ↦ False, _ ↦ True` — keep one. `Words.designationGiven` equals
  `designationChecked`; `Words.LibOrdinal` equals `Ordinal`;
  `Events.complementLocates` equals `placementDestOk` but for the `Nothing`
  row.
- **G4 — `Words.actFacts` shape.** 89 rows of 15 positional fields, 53 of them
  the all-default stub that exists only so `knownAct` holds. Make it a
  `record ActFacts` with a `plainAct : String -> ActFacts` default and field
  overrides for the non-stub rows; no all-default row survives. A field with a
  single inhabitant that a gate consults is a lexeme guard written as data:
  `actForMana` (Tap alone, deleted by `workbench-tapped-for-mana-event`),
  `actDefends` (Attack) and `actTargeted` (Target) each become the declared
  feature they encode or go together with their gate
  (`Events.verbForManaOk`, `deedDefendsOk`, `deedTargetedOk`).
- **G5 — string-literal deed guards inside core constructors.**
  `Effect.Fights` (`deedNounOk "Attack"`), `Effect.BecomesBlocking` and
  `StopsBlocking` (`"Block"`), `Effect.BecomesAttacking`,
  `Phrase.attackableKind` (`deedAltOk "Attack"`), `Events.deedAbilityRole`
  (`v == "Trigger"`, `v == "Activate"`), `Words.survivesShuffle`
  (`stampedBy "Search"`), `Effect.staticIntro (GainsControl …)`
  (`Just "GainControl"`), `Effect.instrIntro (ControllerSacrifices …)`
  (`"Sacrifice"`) and `Triggers.eventName (UnlocksDoor …)`
  (`VerbedAct "Unlock"`). A guard reads a declared feature, never a lexeme:
  give `ActFacts` the columns it needs (`actFightsAs`, `actSurvivesShuffle`,
  `actAbilityRole`) and read them.
- **G6 — `Macros.kindJoin who what = Joined what who`** is an argument swap
  used 20 times; delete it and write `Joined what who` at the sites. Keep
  `Macros.youAnd`/`youOr` only if "you and/or …" is wanted as a named printed
  lemma; otherwise they are `Both You`/`EitherOf You`.
- **G8 — `Words.Payload.ChosenPlayerP` is a facet, not a constructor.** It
  differs from `PlayerP` only in "chosen", and ten functions case-split the
  pair (`samePayload`, `unionPayload`, `halfReaches`, `pubB`, `setZone`,
  `payloadZone`/`Ty`/`Size`/`Prov`/`Orig`, `choiceBinds`). `ObjectP` already
  carries its facets as fields; make it `PlayerP : (chosen : Bool) -> Payload
  Player`.
- **G9 — reach case splits.** `Phrase.remarkTest`, `Phrase.counterMemoryOk`,
  `Phrase.moveDestOk` and `Phrase.nounEqRef` each enumerate `Pro Bare`,
  `Pro (AtSlot _)`, `Pro (Stamped _)` and `Pro TokenBorn` against the rest,
  and `Effect.counterpartNotSelf` special-cases `Pro Bare OneOf`. The four
  reaches share one property — they resolve to an object binding a move can
  re-stamp. Add `reachTracksObject : Reach -> Bool` and write all five through
  it; `nounEqRef`'s three `Pro` rows become one over a `Reach` equality.
- **G10 — dead and single-use macros.** 25 `Macros` exports have no bench or
  pin use: the scry cluster (`agentLookedDelta`, `agentLookedOuter`,
  `agentLookedPlur`, `agentRef`, `AgentRefOk`, `agentSelfOrOwn`, `lookAndSort`,
  `lookedGroup`, `lookedParted`, `lookedSlice`, `lookedSliceDelta`,
  `lookedSlicePlur`, `lookedSpilled`, `OwnRefOk`), `armyYouControl`,
  `cumulativeUpkeepExpansion`, `noCosts`, the party cluster
  (`creatureInYourParty`, `fullPartyOf`, `partyOf`, `partyRoles`,
  `partySizeOf`), `manaValueSpentToCast`, `thisPermanent` and `thisSpell`.
  Each gets a printed witness or goes; the party cluster has no witness at
  all. The scry cluster belongs to `workbench-windowed-pro`, where `agentRef`
  survives as the window re-read — leave it there. Of the 79 single-use
  exports, delete those that are pure argument re-orders and list the rest.
- **D-Q17 — proof-bundle types outside the exception.** The 2026-09-04 ruling
  keeps only `Card`'s law bundles as proof-bundle witness types; a constructor
  whose arguments are all implicit proofs is otherwise not admissible.
  `Phrase.ChoiceClause` and `ChoiceOrder`, `Effect.SpendPurposes` (which
  re-implements `NonEmpty`), and `Words.KnownCounter` and `KnownKeywordTerm`
  are of that shape. Restate each as an obligation on the constructor that
  needs it, or as the standard predicate it duplicates.
- **G11 — an obligation demanded twice.** `Phrase.ZoneExpr.LibraryAt` already
  demands `PlaceArrangementFits` and `PlaceOrdinalFits`; `Phrase.DestOk`'s
  `LibraryPosOk` demands them again to match the constructor. Take them as
  erased pattern variables.

Size: M. Done when: one predicate survives each G3 family; `actFacts` is a
record with no all-default row and no single-inhabitant gate column; `grep`
finds no string literal in a core constructor's guard; the named dead macros
are gone or witnessed; the removed count is justified name by name; build at
its module count. Standard constraints apply, including the RON-shaped
constraint.
