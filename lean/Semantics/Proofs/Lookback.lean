import Semantics.Check.Triggers

namespace Semantics.Proofs.Lookback

open Semantics

theorem scopedPlayerDraw :
    LookbackClause.check .player [] (.mk (.draws (.gap .player)) .thisTurn) = [] := by decide

theorem playerGapCannotStandForObject :
    LookbackClause.check .object [] (.mk (.draws (.gap .player)) .thisTurn)
      = [.lookbackSubject] := by decide

theorem gapRequiresLookback :
    GameEvent.check [] (.draws (.gap .player)) = [.lookbackSubject] := by decide

theorem scopedCreatureDeath :
    LookbackClause.check .object []
      (.mk (.dies (.asType .creature (.gap .object) none)) .thisTurn) = [] := by decide

theorem nestedLookbackBindsItsOwnParticipant :
    LookbackClause.check .player []
      (.mk
        (.casts (.gap .player)
          (some (.described (.a .unmarked)
            (.and [.hasType .creature, .inZone (.zone .stack .bare),
              .happenedTo (.mk (.dies (.asType .creature (.gap .object) none)) .thisTurn)])))
          none) .thisTurn) = [] := by decide

theorem nestedLookbackCannotCaptureOuterGap :
    LookbackClause.check .player []
      (.mk
        (.casts (.gap .player)
          (some (.described (.a .unmarked)
            (.and [.hasType .creature, .inZone (.zone .stack .bare),
              .happenedTo (.mk (.draws (.gap .player)) .thisTurn)])))
          none) .thisTurn) = [.lookbackSubject] := by decide

theorem lookbackRequiresBoundParticipant :
    LookbackClause.check .player [] (.mk (.draws .you) .thisTurn)
      = [.lookbackSubject] := by decide

theorem innerGapDoesNotSupplyOuterParticipant :
    LookbackClause.check .object []
      (.mk (.stateHolds (.happened .you (.mk (.draws (.gap .player)) .thisTurn)))
        .thisTurn) = [.lookbackSubject] := by decide

theorem participantCanOccurMoreThanOnce :
    LookbackClause.check .object []
      (.mk
        (.dealsDamage .any (.asType .creature (.gap .object) none)
          (some (.asType .creature (.gap .object) none))) .thisTurn) = [] := by decide

theorem historyDoesNotIntroduceDamageOutcome :
    Predicate.introduced []
      (.happenedTo
        (.mk (.dealsDamage .any (.asType .creature (.gap .object) none) (some .you))
          .thisTurn)) = [] := by decide

theorem historyRetainsNamedParticipant :
    Predicate.introduced []
      (.happenedTo
        (.mk
          (.dealsDamage .any (.asType .creature (.gap .object) none)
            (some (.described (.a .unmarked) .opponent))) .thisTurn))
      = [⟨.a, .one, .player false⟩] := by rfl

theorem castCanCarryExcludedOrigins :
    LookbackClause.check .player []
      (.mk
        (.casts (.gap .player) (some (.described (.a .unmarked) (.inZone (.zone .stack .bare))))
          (some (.anywhereBut [.zone .hand (.possessedBy .you)]))) .thisTurn) = [] := by decide

theorem castRejectsEmptyOriginExclusion :
    LookbackClause.check .player []
      (.mk
        (.casts (.gap .player) (some (.described (.a .unmarked) (.inZone (.zone .stack .bare))))
          (some (.anywhereBut []))) .thisTurn) = [.playableFrom] := by decide

theorem namedActionCanCarryItsLocus :
    LookbackClause.check .player []
      (.mk
        (.verbedEvent (some (.gap .player)) (.action "Search") none none
          (some (.zone .library (.possessedBy .you)))) .thisWay) = [] := by decide

theorem locusUsesDeclaredActionFeatures :
    LookbackClause.check .player []
      (.mk
        (.verbedEvent (some (.gap .player)) (.action "Shuffle") none none
          (some (.zone .graveyard (.possessedBy .you)))) .thisWay)
      = [.lookbackLocus] := by decide

theorem nestedLookbackConsumesItsGapUse (outer : Option Kind) (kind : Kind) (bs : Bindings)
    (event : GameEvent) (window : Lookback) :
    (LookbackClause.checkIn outer kind bs (.mk event window)).usedGap = false := rfl

theorem unspecifiedSpellDoesNotIntroduceAReference :
    GameEvent.mentioned [] (.casts (.gap .player) none none) = [] := rfl

theorem designatedSpellRetainsItsReference :
    NounPhrase.introduced [] (.asMarker .spell (.designated "commander" .you)) = [] := rfl

theorem designatedSpellCanBeMatched :
    LookbackClause.check .player []
      (.mk (.casts (.gap .player) (some (.asMarker .spell (.designated "commander" .you))) none)
        .thisGame) = [] := by decide

theorem eventModifiersPreserveFacts (event : GameEvent) (ordinal : Ordinal)
    (part : Option TurnPart) (cause : Causing) :
    (GameEvent.nthOccurrence ordinal part event).facts = event.facts ∧
      (GameEvent.causes cause event).facts = event.facts := ⟨rfl, rfl⟩

end Semantics.Proofs.Lookback
