import Semantics.Check.AbilityRules

namespace Semantics.Proofs.HistoricalMove

private def departedCreature : Referent := {
  det := .the, plur := .one, id := 7,
  payload := .object [.creature] (some .graveyard)
    (some { verb := .action "destroy", wasField := true, moved := true }) none none }

private def context : DRS := {
  referents := [departedCreature], nextId := 8, currentClause := 1 }

private def creatureRead : NounPhrase := .pro (.word (.type .creature)) .one

private def deathLookback : DidClause :=
  ⟨.any, .move creatureRead (.zone .battlefield none)
    (some (.zone .graveyard none)) .departure none [], some .thisWay⟩

/-- A death lookback reads the departed participant's battlefield type without moving it again. -/
theorem historicalDeathPreservesTheCurrentObject :
    let out := (stepDidClause 64 context deathLookback).1
    Condition.check context (.did deathLookback) = [] ∧
      (out.findById? 7).map (·.payload) = some departedCreature.payload ∧
      out.nextId = 8 ∧ out.historicalMoveFrom = none ∧
      out.historicalReferentIds = [] := by decide

/-- Historical source eligibility is confined to the movement patient read. -/
theorem historicalDeathDoesNotMakeTheDeadCreatureCurrent :
    let out := (stepDidClause 64 context deathLookback).1
    NounPhrase.check out creatureRead = [.anaphor (.word (.type .creature)) .one 0] ∧
      Action.check out (.setStatus .untapped creatureRead) =
        [.anaphor (.word (.type .creature)) .one 0] ∧
      Action.check out
        (.putCounters (.lit 1) (.printed (.named "p1p1Counter")) creatureRead .many true) =
        [.anaphor (.word (.type .creature)) .one 0] := by decide

/-- An ordinary move and a lookback from another origin cannot borrow battlefield type reach. -/
theorem historicalEligibilityRequiresItsWrittenOrigin :
    Action.check context deathLookback.action =
      [.anaphor (.word (.type .creature)) .one 0] ∧
    Condition.check context (.did ⟨.any,
      .move creatureRead (.zone .hand none)
        (some (.zone .graveyard none)) .departure none [], some .thisWay⟩) =
      [.anaphor (.word (.type .creature)) .one 0] := by decide

end Semantics.Proofs.HistoricalMove
