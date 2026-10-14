import Semantics.Check.Triggers

namespace Semantics.Proofs.Lookback

open Semantics

/-- "this turn", as a lookback reads it (`Macros.soFarThisTurn`). -/
def soFarThisTurn : Duration := .between (some (.beginningOf .turn none .current)) (some .now)
/-- "this game" (`Macros.thisGame`). -/
def thisGame : Duration := .between none (some .now)

theorem scopedPlayerDraw :
    LookbackClause.check .player [] (.mk (.draws (.gap .player)) soFarThisTurn none) = [] := by decide

theorem playerGapCannotStandForObject :
    LookbackClause.check .object [] (.mk (.draws (.gap .player)) soFarThisTurn none)
      = [.lookbackSubject] := by decide

theorem gapRequiresLookback :
    GameEvent.check [] (.draws (.gap .player)) = [.lookbackSubject] := by decide

theorem scopedCreatureDeath :
    LookbackClause.check .object []
      (.mk (.zoneChange (.asType .creature (.gap .object) none) (some (.zones [.zone .battlefield .bare])) (some (.zone .graveyard .bare)) .before) soFarThisTurn none) = [] := by decide

theorem nestedLookbackBindsItsOwnParticipant :
    LookbackClause.check .player []
      (.mk (.casts (.gap .player)
          (some (.described (.a .unmarked)
            (.and [.hasType .creature, .inZone (.zone .stack .bare),
              .happenedTo (.mk (.zoneChange (.asType .creature (.gap .object) none) (some (.zones [.zone .battlefield .bare])) (some (.zone .graveyard .bare)) .before) soFarThisTurn none)])))
          none) soFarThisTurn none) = [] := by decide

theorem nestedLookbackCannotCaptureOuterGap :
    LookbackClause.check .player []
      (.mk (.casts (.gap .player)
          (some (.described (.a .unmarked)
            (.and [.hasType .creature, .inZone (.zone .stack .bare),
              .happenedTo (.mk (.draws (.gap .player)) soFarThisTurn none)])))
          none) soFarThisTurn none) = [.lookbackSubject] := by decide

theorem lookbackRequiresBoundParticipant :
    LookbackClause.check .player [] (.mk (.draws .you) soFarThisTurn none)
      = [.lookbackSubject] := by decide

theorem innerGapDoesNotSupplyOuterParticipant :
    LookbackClause.check .object []
      (.mk (.stateHolds (.happened .you (.mk (.draws (.gap .player)) soFarThisTurn none))) soFarThisTurn none) = [.lookbackSubject] := by decide

theorem participantCanOccurMoreThanOnce :
    LookbackClause.check .object []
      (.mk (.damage .any (some (.asType .creature (.gap .object) none))
          (some (.asType .creature (.gap .object) none))) soFarThisTurn none) = [] := by decide

theorem historyDoesNotIntroduceDamageOutcome :
    Predicate.introduced []
      (.happenedTo
        (.mk (.damage .any (some (.asType .creature (.gap .object) none)) (some .you)) soFarThisTurn none)) = [] := by decide

theorem historyRetainsNamedParticipant :
    Predicate.introduced []
      (.happenedTo
        (.mk (.damage .any (some (.asType .creature (.gap .object) none))
            (some (.described (.a .unmarked) .opponent))) soFarThisTurn none))
      = [⟨.a, .one, .player false, []⟩] := by rfl

theorem castCanCarryExcludedOrigins :
    LookbackClause.check .player []
      (.mk (.casts (.gap .player) (some (.described (.a .unmarked) (.and [.isCard, .inZone (.zone .stack .bare)])))
          (some (.anywhereBut [.zone .hand (.possessedBy .you)]))) soFarThisTurn none) = [] := by decide

theorem namedActionCanCarryItsLocus :
    LookbackClause.check .player []
      (.thisWay (.verbedEvent (some (.gap .player)) (.action "search") none none
          (some (.zone .library (.possessedBy .you))))) = [] := by decide

theorem locusUsesDeclaredActionFeatures :
    LookbackClause.check .player []
      (.thisWay (.verbedEvent (some (.gap .player)) (.action "shuffle") none none
          (some (.zone .graveyard (.possessedBy .you)))))
      = [.lookbackLocus] := by decide

theorem nestedLookbackConsumesItsGapUse (outer : Option Kind) (kind : Kind) (bs : Bindings)
    (event : GameEvent) (within : Duration) (timing : Option Timing) :
    (LookbackClause.checkIn outer kind bs (.mk event within timing)).usedGap = false := rfl

theorem unspecifiedSpellDoesNotIntroduceAReference :
    GameEvent.mentioned [] (.casts (.gap .player) none none) = [] := rfl

theorem designatedSpellRetainsItsReference :
    NounPhrase.introduced [] (.asMarker .spell (.designated (.named "commander") .you)) = [] := rfl

theorem designatedSpellCanBeMatched :
    LookbackClause.check .player []
      (.mk (.casts (.gap .player) (some (.asMarker .spell (.designated (.named "commander") .you))) none) thisGame none) = [] := by decide

theorem eventModifiersPreserveFacts (event : GameEvent) (ordinal : Ordinal)
    (part : Option TurnPart) (cause : Causing) :
    (GameEvent.nthOccurrence ordinal part event).facts = event.facts ∧
      (GameEvent.causes cause event).facts = event.facts := ⟨rfl, rfl⟩

end Semantics.Proofs.Lookback
