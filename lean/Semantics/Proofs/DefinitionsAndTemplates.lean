import Semantics.Check
import Semantics.Check.Card
import Semantics.Definitions

open Semantics Semantics.Templates Semantics.Definitions

namespace Semantics.Proofs.DefinitionsAndTemplates

/-- error: An Action Definition must establish its own Reference Scope -/
#guard_msgs in
semantic_definition missingReferenceScope : Action := .draw (.exactlyOf 1)

open Lean Elab Command in
run_cmd do
  let env ← getEnv
  unless Authoring.definitionAttribute.hasTag env ``Semantics.Definitions.airbend &&
      Authoring.definitionAttribute.hasTag env ``Semantics.Definitions.ripple &&
      Authoring.templateAttribute.hasTag env ``Semantics.Templates.dealDamageOwnPower do
    throwError "Definition and Template declarations must retain their explicit categories"

theorem okAirbendOne :
    (airbend (target isCreature)).check (DRS.ofList []) = [] := by decide

theorem okAirbendUpToTwo :
    (airbend (.described (.target (upTo 2)) isCreature)).check (DRS.ofList []) = [] := by decide

theorem okConniveOne :
    (connive 1).check (DRS.ofList []) = [] := by decide

theorem okConniveTarget :
    (instruct (target isCreature) (connive 2)).check (DRS.ofList []) = [] := by decide

theorem okRippleFour :
    (ripple 4).check = [] := by decide

private def callerContext : DRS :=
  nextClause (stepAction 64 .postEvent .obj (DRS.ofList []) (tap (target isCreature))).1

theorem keywordCostReadsCaller :
    (cycling (.perform (tap it))).check callerContext = [] := by decide

theorem keywordBodyCannotReadCaller :
    (Ability.keyword (.named "flying") []
      [.static (.cant .any (.block it) {})]).check callerContext =
        [.anaphor .bare .one 0] := by decide

theorem callerPronoun :
    (Action.sequentially [tap (target isCreature), untap it]).check (DRS.ofList []) = [] := by
  decide

theorem definitionCannotReadUnpassedCaller :
    (Action.sequentially [tap (target isCreature),
      .ownScope (some (.action "tap")) (.setStatus .tapped it)]).check (DRS.ofList []) =
        [.anaphor .bare .one 0] := by decide

theorem templateKeepsSingleTarget :
    ((stepAction 64 .postEvent .obj (DRS.ofList [])
      (dealDamageOwnPower (target isCreature) (target isCreature))).1.referents.filter
        (fun r ↦ r.det == .target)).length = 2 := by decide

theorem parameterIdentitySurvivesTap :
    (Action.sequentially [tap (target isCreature),
      .setStatus .untapped (.laterMention "tap.subject" .any)]).check (DRS.ofList []) = [] := by
  decide

theorem movedParameterCannotBeReused :
    (Action.sequentially [exile (target isCreature),
      .setStatus .untapped (.laterMention "exile.subject" .any)]).check (DRS.ofList []) =
        [.bindingIdentity "exile.subject"] := by decide

theorem atomicDefinitionLaw :
    (tap (target isCreature)).check (DRS.ofList []) = [] := by decide

/-- An Action Definition's own Reference Scope header is its one deed: the head gate admits the
action by it, its caused event names it, and stepping it stamps what the body acts on, with no
second deed-carrying node (`Action.ownDeed?`). `cast` writes no other node at all. -/
theorem headerDeedIsReadByTheGateAndTheStepper :
    let spell := cast (target (and [isInstantOrSorcery, .isCard]))
    let after := (stepAction 64 .postEvent .obj (DRS.ofList []) spell).1
    spell.eventHeadOk = true ∧ spell.deonticHeadOk = true ∧
      spell.namedDeed = some (.action "cast") ∧
      after.referents.map (·.payload.prov.map (·.deeds)) = [some [.action "cast"]] := by decide

/-- No deed is guessed from a body's shape: a bare move to exile, a bare status change and a bare
creation name none; each is a keyword action only as its Definition [CR#701.13a,701.26a,701.7a]. -/
theorem bareShapesNameNoDeed :
    (Action.move (target isCreature) .anyLocation (some exileZone) .arrival []).namedDeed = none ∧
      (Action.setStatus .tapped (target isCreature)).namedDeed = none ∧
      (exile (target isCreature)).namedDeed = some (.action "exile") ∧
      (tap (target isCreature)).namedDeed = some (.action "tap") := by decide

theorem revealUpdatesActionStamp :
    (Action.sequentially [tap (target isCreature), reveal it,
      tap (the (was (reveal which)))]).check (DRS.ofList []) = [] := by decide

theorem exhaustedTraversalRefuses :
    (stepAction 0 .postEvent .obj (DRS.ofList []) (tap (target isCreature))).2 =
      [.recursionLimit] := by decide

theorem exhaustedWrapperRefuses :
    (stepLocationWith (stepOps 64) (DRS.ofList []) hand 0).2 =
      [.recursionLimit] := by decide

/-! ## Put and return

"Put" and "return" name no deed: each is ordinary English for a Zone Change [CR#400.7]. Separate
Templates keep the two wordings, and they expand to the same move. -/

theorem putAndReturnExpandToOneTree (subject : NounPhrase) (destination : Location)
    (riders : List MoveRider) (origin : Location) :
    returnToHand subject = putInto subject hand ∧
    returnToHand subject origin = putInto subject hand (some origin) ∧
    returnToBattlefield subject = putOntoBattlefield subject ∧
    returnToBattlefield subject riders (some origin) = putInto subject battlefield (some origin) riders ∧
    returnTo subject destination riders = putWith subject destination riders :=
  ⟨rfl, rfl, rfl, rfl, rfl⟩

/-! ## The bare plural

A universal bare plural and "all" have one rules meaning and one plural referent: "Creatures you
control get +1/+1" and "All creatures you control get +1/+1". Separate Templates keep the two
wordings, and they expand to the same tree. -/

theorem bareAndAllOfExpandToOneTree (p : Predicate) :
    bare p = allOf p ∧
      getsPt (bare (and [isCreature, youControl])) (up 1) (up 1) =
        getsPt (allOf (and [isCreature, youControl])) (up 1) (up 1) :=
  ⟨rfl, rfl⟩

/-- A move from a graveyard to a hand is observed on its departure, whichever verb writes it: an
ability that triggers when a card leaves a graveyard looks back in time [CR#603.10a]. -/
theorem graveyardToHandObservedOnDeparture :
    returnToHand (target isCreature) yourGraveyard =
      .move (target isCreature) yourGraveyard (some hand) .departure [] ∧
    putInto (target isCreature) hand (some yourGraveyard) =
      .move (target isCreature) yourGraveyard (some hand) .departure [] :=
  ⟨rfl, rfl⟩

/-- "X and Y", "X or Y" and "X and/or Y" are three encodings of a noun phrase. -/
theorem nounPhraseCoordinatorsAreDistinct :
    (and [you, target isCreature] : NounPhrase) ≠ (andOr [you, target isCreature] : NounPhrase) ∧
      (or [you, target isCreature] : NounPhrase) ≠ (andOr [you, target isCreature] : NounPhrase) ∧
      (and [you, target isCreature] : NounPhrase) ≠ (or [you, target isCreature] : NounPhrase) := by
  refine ⟨?_, ?_, ?_⟩ <;> intro h <;> cases h

/-- "graveyard, hand, and library", "… or library" and "… and/or library" are three encodings of
a location. -/
theorem locationCoordinatorsAreDistinct :
    (and [yourGraveyard, yourHand, yourLibrary] : Location) ≠
        (andOr [yourGraveyard, yourHand, yourLibrary] : Location) ∧
      (or [yourGraveyard, yourHand, yourLibrary] : Location) ≠
        (andOr [yourGraveyard, yourHand, yourLibrary] : Location) := by
  refine ⟨?_, ?_⟩ <;> intro h <;> cases h

/-- Flying's "with flying and/or reach" is not "with flying or reach" [CR#702.9b]. -/
theorem flyingAndOrReachIsNotFlyingOrReach :
    (andOr [hasKeyword (.the (.named "flying")), hasKeyword reach] : Predicate) ≠
      (or [hasKeyword (.the (.named "flying")), hasKeyword reach] : Predicate) := by
  intro h; cases h

/-- "search your library and/or graveyard for …" is the search of that and/or location. -/
theorem searchLibraryOrGraveyardIsItsLocationSearch :
    searchLibraryOrGraveyard (a isCard) =
      search (andOr [yourLibrary, yourGraveyard] : Location) (a isCard) := rfl

/-- "search your library" with no "for …" searches for `any`. -/
theorem unqualifiedSearchIsForAny : search yourLibrary = search yourLibrary any := rfl

end Semantics.Proofs.DefinitionsAndTemplates
