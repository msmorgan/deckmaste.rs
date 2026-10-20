import Semantics.Definitions
import Semantics.Check.Phrase

namespace Semantics.Proofs.Lookback

open Semantics Semantics.Templates Semantics.Definitions

/-- "this turn", as a lookback reads it (`Templates.soFarThisTurn`). -/
def soFarThisTurn : Duration := .between (some (.beginningOf .turn none .current)) (some .now)
/-- "this game" (`Templates.thisGame`). -/
def thisGame : Duration := .between none (some .now)

private def relativeCheck (kind : Kind) (clause : DidClause) : List Refusal :=
  Predicate.checkAt kind (DRS.ofList []) (.did clause)

private def withinThisTurn : LookbackSpan := .duration soFarThisTurn none

theorem scopedPlayerDraw :
    relativeCheck .player ⟨.actor .which, .draw (.exactlyOf .anyAmount), withinThisTurn⟩ = [] := by decide

theorem playerGapCannotStandForObject :
    relativeCheck .object ⟨.actor .which, .draw (.exactlyOf .anyAmount), withinThisTurn⟩ =
      [.kindMismatch .player .object] := by decide

theorem gapRequiresLookback :
    GameEvent.check (DRS.ofList []) (.does (.actor .which) (.draw (.exactlyOf .anyAmount))) =
      [.anaphor (.word .permanent) .one 0] := by decide

theorem scopedCreatureDeath :
    relativeCheck .object
      ⟨.any, .move (.headed (.type .creature) .which)
        (.zone .battlefield none) (some (.zone .graveyard none)) .departure none [],
        withinThisTurn⟩ = [] := by decide

theorem nestedLookbackBindsItsOwnParticipant :
    relativeCheck .player
      ⟨.actor .which,
        .procedure .cast
          (.described (.a .unmarked)
            (.and [.hasType .creature, .in_ (.zone .stack none),
              .did ⟨.any, .move (.headed (.type .creature) .which)
                (.zone .battlefield none) (some (.zone .graveyard none))
                .departure none [], withinThisTurn⟩]))
          .itsOwnCost none none none,
        withinThisTurn⟩ = [] := by decide

theorem nestedLookbackCannotCaptureOuterGap :
    relativeCheck .player
      ⟨.actor .which,
        .procedure .cast
          (.described (.a .unmarked)
            (.and [.hasType .creature, .in_ (.zone .stack none),
              .did ⟨.actor .which, .draw (.exactlyOf .anyAmount), withinThisTurn⟩]))
          .itsOwnCost none none none,
        withinThisTurn⟩ = [.kindMismatch .player .object] := by decide

theorem lookbackRequiresBoundParticipant :
    relativeCheck .player ⟨.actor .you, .draw (.exactlyOf .anyAmount), withinThisTurn⟩ =
      [.lookbackSubject] := by decide

theorem innerGapDoesNotSupplyOuterParticipant :
    relativeCheck .object
      ⟨.actor .you, .draw (.exactlyOf .anyAmount),
        .duration soFarThisTurn
          (some (.onlyIf (.exists_ (.described .bare
            (.and [.hasType .creature,
              .did ⟨.any, .move .which (.zone .battlefield none)
                (some (.zone .graveyard none)) .departure none [], withinThisTurn⟩])))))⟩ =
      [.lookbackSubject] := by decide

theorem participantCanOccurMoreThanOnce :
    relativeCheck .object
      ⟨.actor (.headed (.type .creature) .which),
        .dealDamage .anyAmount (.headed (.type .creature) .which) .any,
        withinThisTurn⟩ = [] := by decide

private def damageHistory : Predicate :=
  .did ⟨.actor (.headed (.type .creature) .which),
    .dealDamage .anyAmount (.described (.a .unmarked) (.isOpponentOf .you)) .any, withinThisTurn⟩

theorem historyDoesNotIntroduceDamageOutcome :
    countOutcomes .damageDealt (stepPredicate 64 (DRS.ofList []) damageHistory).1 = 0 := by decide

theorem historyDoesNotExportNamedParticipant :
    NounPhrase.checkAt (some .player) (stepPredicate 64 (DRS.ofList []) damageHistory).1
      (.pro (.word .player) .one) = [.anaphor (.word .player) .one 0] := by decide

theorem historyNamedParticipantAccessibleLocally :
    relativeCheck .object
      ⟨.actor .which,
        .dealDamage (.statOf (.playerStat .lifeTotal) (.described (.a .unmarked) (.isOpponentOf .you)))
          (.pro (.word .player) .one) .any,
        withinThisTurn⟩ = [] := by decide

theorem castCanCarryExcludedOrigins :
    relativeCheck .player
      ⟨.actor .which,
        .procedure .cast (.described (.a .unmarked) (.and [.isCard, .in_ (.zone .stack none)]))
          .itsOwnCost none (some (.not (.zone .hand (some .you)))) none,
        withinThisTurn⟩ = [] := by decide

theorem namedActionCanCarryItsLocus :
    relativeCheck .player
      ⟨.actor .which,
        .enact (.action "search") (.search ((.zone .library (some .you)))
          (.range none none) (.and [])) none (some (.zone .library (some .you))),
        .thisWay⟩ = [] := by decide

theorem locusUsesDeclaredActionFeatures :
    relativeCheck .player
      ⟨.actor .which,
        .enact (.action "shuffle") (.shuffle (.zone .library (some .you)))
          none (some (.zone .graveyard (some .you))),
        .thisWay⟩ = [.lookbackLocus] := by decide

theorem nestedLookbackPreservesOuterBinder (drs : DRS) (clause : DidClause) :
    (stepPredicate 64 drs (.did clause)).1.relativeBinder = drs.relativeBinder ∧
      (stepPredicate 64 drs (.did clause)).1.relativeUses = drs.relativeUses := by
  constructor <;> rfl

theorem unspecifiedSpellDoesNotIntroduceAReference :
    (stepGameEvent 64 (DRS.ofList [])
      (.does .any (.procedure .cast .any .itsOwnCost none none none))).1.referents = [] := by rfl

theorem designatedSpellIsWellKinded :
    NounPhrase.checkAt (some .object) (DRS.ofList [])
      (.headed .spell (.designated (.named "commander") .you)) = [] := by decide

theorem designatedSpellKeepsSelectedIdentity :
    let result := stepNounPhrase 64 (DRS.ofList []) .other false
      (.headed .spell (.designated (.named "commander") .you))
    result.2.1 = [1] ∧ (result.1.findById? 1).bind (·.zone) = some .stack := by decide

theorem designatedSpellCanBeMatched :
    relativeCheck .player
      ⟨.actor .which,
        .procedure .cast (.headed .spell (.designated (.named "commander") .you))
          .itsOwnCost none none none,
        .duration thisGame none⟩ = [] := by decide

theorem eventModifiersPreserveFacts (event : GameEvent) (ordinal : Ordinal)
    (part : Option TurnPart) (cause : Causing) (fuel : Nat) :
    (GameEvent.nthOccurrence ordinal part event).facts (fuel + 1) = event.facts fuel ∧
      (GameEvent.causes cause event).facts (fuel + 1) = event.facts fuel := ⟨rfl, rfl⟩

end Semantics.Proofs.Lookback
