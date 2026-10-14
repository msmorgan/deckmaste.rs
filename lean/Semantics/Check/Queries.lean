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

def NounPhrase.zone (drs : DRS) (phrase : NounPhrase) : Option Zone :=
  match phraseReferents drs phrase with
  | [] => none
  | first :: rest => if rest.all (fun ref => ref.zone == first.zone) then first.zone else none

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
    (.slice .top (.lit 2) (.library (.possessedBy .you)))

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
    let owner : NounPhrase := .described .the (.hasType .creature)
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

end Check.QueryPins
end Semantics
