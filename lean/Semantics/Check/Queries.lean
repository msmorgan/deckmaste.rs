import Semantics.Check.Phrase

/-!
# Semantics.Check.Queries

Pure metadata queries over checked phrases and stable discourse referents.
-/

namespace Semantics

private def phraseReferents (drs : DRS) (phrase : NounPhrase) : List Referent :=
  let out := stepNounPhrase 512 drs .other false phrase
  out.2.1.filterMap out.1.findById?

def NounPhrase.ty (drs : DRS) (phrase : NounPhrase) : List CardType :=
  match phraseReferents drs phrase with
  | [] => []
  | first :: rest => first.ty.filter fun type => rest.all (fun ref => ref.ty.contains type)

private def commonZone : Payload → Option Zone
  | .join left right =>
    let zone := commonZone left
    if commonZone right == zone then zone else none
  | payload => payload.zone

def NounPhrase.zone (drs : DRS) (phrase : NounPhrase) : Option Zone :=
  match phraseReferents drs phrase with
  | [] => none
  | first :: rest => if rest.all (fun ref => commonZone ref.payload == commonZone first.payload) then
      commonZone first.payload else none

namespace Check.QueryPins

private def cardReferent : Referent :=
  { det := .the, plur := .one, payload := .object [.creature] (some .exile) none none none,
    id := 7 }

private def playerReferent : Referent :=
  { det := .the, plur := .one, payload := .player false, id := 3 }

private theorem selectedCardMetadataIgnoresUnrelatedReferentOrder :
    let phrase : NounPhrase := .pro (.word .card) .one
    let before := DRS.ofList [cardReferent, playerReferent]
    let after := DRS.ofList [playerReferent, cardReferent]
    (phrase.ty before, phrase.zone before, phrase.ty after, phrase.zone after) =
      ([.creature], some .exile, [.creature], some .exile) := by decide

private theorem literalQuantityIsExact :
    Quantity.exact (.exactlyOf (.inCaller (.lit 2))) = some 2 := by decide

private theorem rangedAndWholeSelectionsAreNotExact :
    (Quantity.exact (.range (some 1) (some 2)), SliceCount.exact .whole) =
      (none, none) := by decide

private def oneOfTwoLibraryCards (description : Option Predicate) : NounPhrase :=
  .someOf (.counted (.range (some 1) (some 1))) description
    (.slice .top (.lit 2) (.zone .library (some .you)))

private theorem selectingOneOfTwoLeavesTheOther :
    let out := stepNounPhrase 512 (DRS.ofList []) .obj true (oneOfTwoLibraryCards none)
    out.2.2 = [] ∧ theOtherOk .object out.1 = true := by decide

private theorem typedSelectionPreservesCardCarrier :
    let out := stepNounPhrase 512 (DRS.ofList []) .obj true
      (oneOfTwoLibraryCards (some (.hasType .creature)))
    let selected := out.2.1.head?.bind out.1.findById?
    selected.map (fun ref =>
      (ref.ty, ref.zone, wordReaches (.type .creature) ref,
        wordReaches (.ofType .card .creature) ref)) =
      some ([.creature], some .library, false, true) := by decide

private theorem thatTurnNeedsExactlyOneReferent :
    ((stepTurnRefWith (stepOps 64) (DRS.ofList []) .that).2,
      (stepTurnRefWith (stepOps 64) (DRS.ofList [turnRefB]) .that).2,
      (stepTurnRefWith (stepOps 64) (DRS.ofList [turnRefB, turnRefB]) .that).2) =
      ([.turnAntecedent 0], [], [.turnAntecedent 2]) := by decide

private theorem turnOwnersMustBePlayersAtEveryEntry :
    let owner : NounPhrase := .headed (.type .creature) .this
    let refused := [Refusal.kindMismatch .player .object]
    (stepGameEvent 64 (DRS.ofList []) (.beginningOf .upkeep (some owner) .current)).2 =
        refused ∧
      (stepTimingWith (stepOps 64) (DRS.ofList [])
        (.inPart .upkeep (some owner) .current)).2 = refused ∧
      (stepTurnRefWith (stepOps 64) (DRS.ofList []) (.next (some owner))).2 = refused := by
  decide

private theorem durationBoundariesNeedDefiniteOwners :
    let indefinite : NounPhrase := .described (.a .unmarked) .anyPlayer
    let rejected : Duration := .between none
      (some (.beginningOf .upkeep (some indefinite) .current))
    let accepted : Duration := .between none
      (some (.beginningOf .upkeep (some .you) .current))
    ((stepDurationWith (stepOps 64) (DRS.ofList []) rejected).2,
      (stepDurationWith (stepOps 64) (DRS.ofList []) accepted).2) =
      ([.durationPossessor], []) := by decide

private theorem unspecifiedCostCannotBePerformed :
    (stepCost 64 (DRS.ofList []) .anyCost).2 = [.concreteCost] ∧
      (stepAction 64 .postEvent .obj (DRS.ofList [])
        (.pay .anyCost .once .paid none none)).2 = [.concreteCost] := by decide

private theorem unpaidKeywordPaymentIsAPattern :
    let event : GameEvent := .does (.actor .you)
      (.pay .anyCost .once .unpaid none (some (.named "cumulativeUpkeep")))
    (stepGameEvent 64 (DRS.ofList []) event).2 = [] ∧
      (stepGameEvent 64 (DRS.ofList []) event).1.actionPattern = false := by decide

private theorem comparisonMemberIsLocalToItsMeasure :
    let member : NounPhrase := .pro (.word .player) .one
    let good : Predicate := .compareOver (.isOpponentOf .you) (.devotion member (.lit .red) none) .greater (.lit 0)
    let bad : Predicate := .compareOver (.isOpponentOf .you) (.lit 0) .greater (.devotion member (.lit .red) none)
    (stepPredicate 64 (DRS.ofList []) good).2 = [] ∧
      (stepPredicate 64 (DRS.ofList []) bad).2 = [.anaphor (.word .player) .one 0] ∧
      (stepPredicate 64 (DRS.ofList []) good).1.visibleReferents.map (·.payload) = [.difference] := by decide

private theorem enchantedPlayerAndTypedAttachmentCounterparts :
    (stepPredicate 64 { relativeKind := some .player }
      (.attachment .host (some .enchanted) none)).2 = [] ∧
      (stepPredicate 64 { relativeKind := some .player }
        (.attachment .host (some .equipped) none)).2 = [.kindLte .player .object] ∧
      (stepPredicate 64 { relativeKind := some .player }
        (.attachment .host (some .fortified) none)).2 = [.kindLte .player .object] ∧
      (stepPredicate 64 { relativeKind := some .object }
        (.attachment .host none (some .you))).2 = [.kindMismatch .object .player] := by decide

private theorem attachmentHostModifierDoesNotOverrideThePlayerHead :
    Predicate.kind? (.and [.attachment .host (some .enchanted) none, .anyPlayer]) =
      some .player := by decide

private theorem attachmentKindsUseTheMatchedSubjectAndOneHeadGate :
    Condition.check (DRS.ofList [])
      (.matches .you (.attachment .host (some .equipped) none)) = [.kindLte .player .object] ∧
      Predicate.checkAt .player (DRS.ofList []) (.attachment .attachment none none) =
        [.kindMismatch .player .object] := by decide

private theorem unionHalfControllerUsesTheSelectedKindWithoutChangingIdentity :
    let joined : Payload := .join
      (.object [.planeswalker] (some .battlefield) none none none) (.player false)
    let context := DRS.ofList [{ det := .target, plur := .one, payload := joined }]
    let controller : NounPhrase := .possessorOf .controller (.pro (.unionHalf (.type .planeswalker)) .one)
    let participant : NounPhrase := .or [.pro (.unionHalf .player) .one, controller]
    (stepNounPhrase 64 context .other false controller).2.2 = [] ∧
      ((stepNounPhrase 64 context .other false controller).1.findById? 0).map (·.payload) = some joined ∧
      NounPhrase.checkAt (some .player) context participant = [] ∧
      Action.check context (.instruct participant (.changeLife (.up (.lit 1)))) = [] ∧
      Amount.check context (.statOf (.playerStat .lifeTotal) participant) = [] ∧
      (stepNounPhrase 64 context .other false
        (.possessorOf .controller (.pro (.word .join) .one))).2.2 =
          [.possessorKind .controller (.join .object .player)] := by decide

/-- "The number of creatures" is closed after the count (`semantics-v3.md` §1.3): it leaves no
referent, so "the rest" after "one of the top N cards" takes the slice's group. -/
private theorem countDomainDoesNotCompeteWithTheRemainderGroup :
    let count : Amount := .countOf (.described .all (.hasType .creature))
    let phrase := NounPhrase.someOf (.counted (.range (some 1) (some 1))) none
      (.slice .top count (.zone .library (some .you)))
    let context := (stepNounPhrase 64 (DRS.ofList []) .other false phrase).1
    (stepAmount 64 (DRS.ofList []) count).1.referents = [] ∧
      countGroups .object context = 1 ∧ countParts .object context = 1 ∧
      theRestOk .object context = true ∧ theOtherOk .object context = false := by decide

private def prospectiveCreatureCreation : Action :=
  .createObject (.lit 1)
    (.token (.written { characteristics := { types := [.creature] } }) []) none

private def repeatProspectiveTokens : Action :=
  .createObject (.lit 3) (.token .asThose []) none

private theorem prospectiveTokenDescriptionDoesNotIntroduceAnActualToken :
    let out := (stepAction 8 .preEvent .obj (DRS.ofList []) prospectiveCreatureCreation).1
    out.prospectiveTokenTypes = [[.creature]] ∧ countTokenSpecs out = 0 ∧
      (resolveCandidates (.word .token) .one out).isEmpty = true ∧
      (stepTokenSpecMetadataWith (stepOps 8) out .asThose).2.2 = [] := by decide

private theorem prospectiveEmblemIsNotATokenDescription :
    let out := (stepAction 8 .preEvent .obj (DRS.ofList [])
      (.createObject (.lit 1) (.emblem none []) none)).1
    out.prospectiveTokenTypes = [] ∧
      (stepTokenSpecMetadataWith (stepOps 8) out .asThose).2.2 = [.tokenSpecInScope 0] := by decide

private theorem replacementTokenDescriptionStaysInItsOwnScope :
    let unused := (stepAction 8 .postEvent .obj (DRS.ofList [])
      (.replace prospectiveCreatureCreation (.instead (.draw (.exactlyOf (.lit 1)))))).1
    let created := (stepAction 8 .postEvent .obj (DRS.ofList [])
      (.replace prospectiveCreatureCreation (.instead repeatProspectiveTokens))).1
    unused.prospectiveTokenTypes = [] ∧ countTokenSpecs unused = 0 ∧
      (stepTokenSpecMetadataWith (stepOps 8) unused .asThose).2.2 = [.tokenSpecInScope 0] ∧
      created.prospectiveTokenTypes = [] ∧ countTokenSpecs created = 1 ∧
      tyOfThoseAny .token created = [.creature] := by decide

private theorem nestedReplacementUsesItsOwnTokenDescription :
    let initial := { (DRS.ofList []) with prospectiveTokenTypes := [[.land]] }
    let nested := Action.replace prospectiveCreatureCreation
      (.instead (.replace (.createObject (.lit 1) (.token .asThose []) none)
        (.instead repeatProspectiveTokens)))
    let result := stepAction 8 .postEvent .obj initial nested
    result.2 = [] ∧ result.1.prospectiveTokenTypes = [[.land]] ∧
      countTokenSpecs result.1 = 1 ∧ tyOfThoseAny .token result.1 = [.creature] := by decide +kernel

private theorem damageSourceRetainsItsIdentityOutsideTheBattlefield :
    let payload := Payload.object [.artifact] (some .graveyard) none none none
    let context := nextClause (DRS.ofList [{ det := .the, plur := .one, payload, id := 17 }])
    let source := NounPhrase.pro .bare .one
    let result := stepAction 8 .postEvent .obj context
      (.instruct source (.dealDamage (.lit 2) .you))
    result.2 = [] ∧ (result.1.findById? 17).map (·.payload) = some payload ∧
      (stepAction 8 .postEvent .obj context (.instruct source (.setStatus .tapped .any))).2 = [] ∧
      (stepAction 8 .postEvent .obj context (.instruct source (.setStatus .untapped .any))).2 = [] := by decide

private theorem kindLoopPublishesChoiceHistoryWithoutItsLocalCreature :
    let creature := Predicate.hasType .creature
    let choice := Action.choose none (.described (.a .unmarked) creature) .openly none
    let result := stepAction 8 .postEvent .obj (DRS.ofList [])
      (.doForEachKind (.value .power) (some (.described .all creature)) .number choice)
    result.2 = [] ∧ Predicate.checkAt .object result.1 .notChosen = [] ∧
      (resolveCandidates (.word (.type .creature)) .one result.1).isEmpty = true ∧
      countParts .object result.1 = 0 ∧ countChoice (.quality .number) result.1 = 0 := by decide +kernel


private theorem wildcardBallotIsOnlyAnActionPattern :
    let initial := DRS.ofList []
    (stepBallotWith (stepOps 8) initial .anyBallot).2 = [.concreteBallot] ∧
      (stepBallotWith (stepOps 8) { initial with actionPattern := true } .anyBallot).2 = [] ∧
      (stepBallotWith (stepOps 8) initial (.byLabel [])).2 = [.ballotLabelsOk] := by decide

private theorem entryRiderReadsItsWholeCallerArgument :
    let caller := nextClause (DRS.ofList [{ det := .a, plur := .one, payload := .player false, id := 7 }])
    let inner := enterOwnScope caller
    let player := NounPhrase.pro (.word .player) .one
    let rider := MoveRider.entersAttacking (some player)
    let result := stepMoveRiderWith (stepOps 8) inner (.inCaller rider)
    (stepMoveRiderWith (stepOps 8) inner rider).2 = [.anaphor (.word .player) .one 0] ∧
      result.2 = [] ∧ (result.1.findById? 7).map (·.payload) = some (.player false) := by decide +kernel

end Check.QueryPins
end Semantics
