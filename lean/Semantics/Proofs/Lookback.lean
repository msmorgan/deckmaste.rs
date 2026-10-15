import Semantics.Macros
import Semantics.Check.Triggers

namespace Semantics.Proofs.Lookback

open Semantics Semantics.Macros

/-- "this turn", as a lookback reads it (`Macros.soFarThisTurn`). -/
def soFarThisTurn : Duration := .between (some (.beginningOf .turn none .current)) (some .now)
/-- "this game" (`Macros.thisGame`). -/
def thisGame : Duration := .between none (some .now)

private def relativeCheck (kind : Kind) (clause : DidClause) : List Refusal :=
  Predicate.checkAt kind (DRS.ofList []) (.did clause)

private def withinThisTurn : LookbackSpan := .duration soFarThisTurn none

theorem scopedPlayerDraw :
    relativeCheck .player ⟨.actor .which, .draw .anyAmount, withinThisTurn⟩ = [] := by decide

theorem playerGapCannotStandForObject :
    relativeCheck .object ⟨.actor .which, .draw .anyAmount, withinThisTurn⟩ =
      [.kindMismatch .player .object] := by decide

theorem gapRequiresLookback :
    GameEvent.check (DRS.ofList []) (.does (.actor .which) (.draw .anyAmount)) =
      [.anaphor (.word .permanent) .one 0] := by decide

theorem scopedCreatureDeath :
    relativeCheck .object
      ⟨.any, .move (.asType .creature .which none)
        (.zones [.zone .battlefield .bare]) (some (.zone .graveyard .bare)) .departure none [],
        withinThisTurn⟩ = [] := by decide

theorem nestedLookbackBindsItsOwnParticipant :
    relativeCheck .player
      ⟨.actor .which,
        .procedure .cast
          (.described (.a .unmarked)
            (.and [.hasType .creature, .inZone (.zone .stack .bare),
              .did ⟨.any, .move (.asType .creature .which none)
                (.zones [.zone .battlefield .bare]) (some (.zone .graveyard .bare))
                .departure none [], withinThisTurn⟩]))
          .itsOwnCost none none none,
        withinThisTurn⟩ = [] := by decide

theorem nestedLookbackCannotCaptureOuterGap :
    relativeCheck .player
      ⟨.actor .which,
        .procedure .cast
          (.described (.a .unmarked)
            (.and [.hasType .creature, .inZone (.zone .stack .bare),
              .did ⟨.actor .which, .draw .anyAmount, withinThisTurn⟩]))
          .itsOwnCost none none none,
        withinThisTurn⟩ = [.kindMismatch .player .object] := by decide

theorem lookbackRequiresBoundParticipant :
    relativeCheck .player ⟨.actor .you, .draw .anyAmount, withinThisTurn⟩ =
      [.lookbackSubject] := by decide

theorem innerGapDoesNotSupplyOuterParticipant :
    relativeCheck .object
      ⟨.actor .you, .draw .anyAmount,
        .duration soFarThisTurn
          (some (.onlyIf (.exists_ (.described (.a .unmarked)
            (.and [.hasType .creature,
              .did ⟨.any, .move .which (.zones [.zone .battlefield .bare])
                (some (.zone .graveyard .bare)) .departure none [], withinThisTurn⟩])))))⟩ =
      [.lookbackSubject] := by decide

theorem participantCanOccurMoreThanOnce :
    relativeCheck .object
      ⟨.actor (.asType .creature .which none),
        .dealDamage .anyAmount (.asType .creature .which none) .any,
        withinThisTurn⟩ = [] := by decide

private def damageHistory : Predicate :=
  .did ⟨.actor (.asType .creature .which none),
    .dealDamage .anyAmount (.described (.a .unmarked) .opponent) .any, withinThisTurn⟩

theorem historyDoesNotIntroduceDamageOutcome :
    countOutcomes .damageDealt (stepPredicate 64 (DRS.ofList []) damageHistory).1 = 0 := by decide

theorem historyDoesNotExportNamedParticipant :
    NounPhrase.checkAt (some .player) (stepPredicate 64 (DRS.ofList []) damageHistory).1
      (.pro (.word .player) .one) = [.anaphor (.word .player) .one 0] := by decide

theorem historyNamedParticipantAccessibleLocally :
    relativeCheck .object
      ⟨.actor .which,
        .dealDamage (.statOf (.playerStat .lifeTotal) (.described (.a .unmarked) .opponent))
          (.pro (.word .player) .one) .any,
        withinThisTurn⟩ = [] := by decide

theorem castCanCarryExcludedOrigins :
    relativeCheck .player
      ⟨.actor .which,
        .procedure .cast (.described (.a .unmarked) (.and [.isCard, .inZone (.zone .stack .bare)]))
          .itsOwnCost none (some (.anywhereBut [.zone .hand (.possessedBy .you)])) none,
        withinThisTurn⟩ = [] := by decide

theorem namedActionCanCarryItsLocus :
    relativeCheck .player
      ⟨.actor .which,
        .enact (.action "search") (.search (.oneZone (.zone .library (.possessedBy .you)))
          (.range none none) (.and [])) none (some (.zone .library (.possessedBy .you))),
        .thisWay⟩ = [] := by decide

theorem locusUsesDeclaredActionFeatures :
    relativeCheck .player
      ⟨.actor .which,
        .enact (.action "shuffle") (.shuffle (.library (.possessedBy .you)))
          none (some (.zone .graveyard (.possessedBy .you))),
        .thisWay⟩ = [.lookbackLocus] := by decide

theorem nestedLookbackPreservesOuterBinder (drs : DRS) (clause : DidClause) :
    (stepPredicate 64 drs (.did clause)).1.relativeBinder = drs.relativeBinder := by rfl

theorem unspecifiedSpellDoesNotIntroduceAReference :
    (stepGameEvent 64 (DRS.ofList [])
      (.does .any (.procedure .cast .any .itsOwnCost none none none))).1.referents = [] := by rfl

theorem designatedSpellIsWellKinded :
    NounPhrase.checkAt (some .object) (DRS.ofList [])
      (.asMarker .spell (.designated (.named "commander") .you)) = [] := by decide

theorem designatedSpellKeepsSelectedIdentity :
    let result := stepNounPhrase 64 (DRS.ofList []) .other false
      (.asMarker .spell (.designated (.named "commander") .you))
    result.2.1 = [1] ∧ (result.1.findById? 1).bind (·.zone) = some .stack := by decide

theorem designatedSpellCanBeMatched :
    relativeCheck .player
      ⟨.actor .which,
        .procedure .cast (.asMarker .spell (.designated (.named "commander") .you))
          .itsOwnCost none none none,
        .duration thisGame none⟩ = [] := by decide

theorem eventModifiersPreserveFacts (event : GameEvent) (ordinal : Ordinal)
    (part : Option TurnPart) (cause : Causing) (fuel : Nat) :
    (GameEvent.nthOccurrence ordinal part event).facts (fuel + 1) = event.facts fuel ∧
      (GameEvent.causes cause event).facts (fuel + 1) = event.facts fuel := ⟨rfl, rfl⟩

end Semantics.Proofs.Lookback
