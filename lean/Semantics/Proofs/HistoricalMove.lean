import Semantics.Check.AbilityRules

namespace Semantics.Proofs.HistoricalMove

private def departedCreature : Referent := {
  det := .the, plur := .one, id := 7,
  payload := .object [.creature] (some .graveyard)
    (some { deeds := [.action "destroy"], wasField := true,
            zoneChange := some ⟨some .battlefield, some .graveyard⟩ }) none none }

private def creatureRead : NounPhrase := .pro (.word (.type .creature)) .one

private def deathLookback : DidClause :=
  { subj := .any
    action := .move creatureRead (.zone .battlefield none)
      (some (.zone .graveyard none)) .departure []
    within := some .thisWay }

/-- After "Destroy that creature": the destroyed creature, and the destruction and the move from
the battlefield to a graveyard it records, which a "this way" reads back. -/
private def context : DRS := {
  referents := [departedCreature], nextId := 8, currentClause := 1
  causedEvents :=
    [ deathLookback.action.causedEvent (some .player) (some .object),
      (Action.ownScope (some (.action "destroy"))
        (.ownScope (some (.action "destroy")) deathLookback.action)).causedEvent (some .player)
        none ] }

/-- A death lookback reads the departed participant's battlefield type without moving it again. -/
theorem historicalDeathPreservesTheCurrentObject :
    let out := (stepDidClause 64 context deathLookback).1
    Condition.check context (.did deathLookback) = [] ∧
      (out.findById? 7).map (·.payload) = some departedCreature.payload ∧
      out.nextId = context.nextId ∧ out.readsLastKnown = false ∧ out.handedOrigin = none ∧
      out.historicalReferentIds = [] := by decide

/-- Historical source eligibility is confined to the movement patient read. -/
theorem historicalDeathDoesNotMakeTheDeadCreatureCurrent :
    let out := (stepDidClause 64 context deathLookback).1
    NounPhrase.check out creatureRead = [.anaphor (.word (.type .creature)) .one 0] ∧
      Action.check out (.setStatus .untapped creatureRead) =
        [.anaphor (.word (.type .creature)) .one 0] ∧
      Action.check out
        (.putCounters 1 (.printed (.named "p1p1Counter")) creatureRead) =
        [.anaphor (.word (.type .creature)) .one 0] := by decide

/-- An ordinary move and a lookback from another origin cannot borrow battlefield type reach. -/
theorem historicalEligibilityRequiresItsWrittenOrigin :
    Action.check context deathLookback.action =
      [.anaphor (.word (.type .creature)) .one 0] ∧
    Condition.check context (.did { subj := .any
                                    action := .move creatureRead (.zone .hand none)
                                      (some (.zone .graveyard none)) .departure []
                                    within := some .thisWay }) =
      [.anaphor (.word (.type .creature)) .one 0, .thisWayUngrounded] := by decide

private def mixedOrigin : Location := .coord .or [.zone .battlefield none, .zone .hand none]

/-- The move's own Observation Point decides the battlefield read. A departure-observed move
from the battlefield or a hand reads the permanent the creature was [CR#603.6c,603.10a], as
`Location.observedOnDeparture` classes it; the same move observed on arrival does not. -/
theorem mixedOriginReadsThePermanentOnlyOnDeparture :
    Location.observedOnDeparture mixedOrigin = true ∧
    Condition.check context (.did { subj := .any
                                    action := .move creatureRead mixedOrigin
                                      (some (.zone .graveyard none)) .departure []
                                    within := some .thisWay }) = [] ∧
    Condition.check context (.did { subj := .any
                                    action := .move creatureRead mixedOrigin
                                      (some (.zone .graveyard none)) .arrival []
                                    within := some .thisWay }) =
      [.anaphor (.word (.type .creature)) .one 0] := by decide

end Semantics.Proofs.HistoricalMove
