import Semantics.Macros
import Semantics.Check.Card

open Semantics Semantics.Macros

namespace Semantics.Proofs.EventPatterns

theorem damageNeedsParticipant :
    GameEvent.check [] (.damage .any none none) = [.nonEmpty] := by decide

theorem damageSourceIsObject :
    GameEvent.check [] (.damage .any (some .you) none) =
      [.kindMismatch .object .player] := by decide

theorem recipientMayBePlayer :
    GameEvent.check [] (.damage .combatOnly none (some .you)) = [] := by decide

theorem sourcePrecedesRecipientReference :
    GameEvent.check [] (.damage .any (some (a creature)) (some it)) = [] := by decide

theorem absentSourceCannotSupplyReference :
    GameEvent.check [] (.damage .any none (some it)) =
      [.anaphor .bare .one 0, .damageRecipient] := by decide

theorem damageIntroducesOutcomeBeforeParticipants (bs : Bindings) (kind : DamageKind)
    (source patient : NounPhrase) :
    GameEvent.after bs (.damage kind (some source) (some patient)) =
      outcomeB .damageDealt :: nomIntro (nomIntro bs source) patient := by rfl

theorem observedDamageDoesNotProduceAnOutcome (bs : Bindings) (kind : DamageKind)
    (source patient : NounPhrase) :
    GameEvent.mentioned bs (.damage kind (some source) (some patient)) =
      NounPhrase.introduced (nomIntro bs source) patient ++ NounPhrase.introduced bs source := by rfl

theorem patientOnlyKeepsSelfSubject (bs : Bindings) (kind : DamageKind) (patient : NounPhrase) :
    GameEvent.after bs (.damage kind none (some patient)) =
      outcomeB .damageDealt :: selfSubjIntro bs patient := by rfl

theorem choiceOccasionsRemainDistinct :
    StaticSpec.choice .entry thisCreature (.quality .color) none .openly ≠
      StaticSpec.choice .attachment thisCreature (.quality .color) none .openly := by
  intro equal
  cases equal

theorem choiceDisclosureRemainsDistinct :
    StaticSpec.choice .entry thisCreature .player none .openly ≠
      StaticSpec.choice .entry thisCreature .player none .secretly := by
  intro equal
  cases equal

theorem everyChoiceOccasionPublishesItsSort (occasion : ChoiceOccasion) (bs : Bindings)
    (subject : NounPhrase) (sort : ChoiceSort) (domain : Option ChoiceDomain)
    (disclosure : Disclosure) :
    StaticSpec.introducedChoices bs (.choice occasion subject sort domain disclosure) =
      [sort.binding] := by rfl

theorem everyChoiceOccasionChecksDomain (occasion : ChoiceOccasion) :
    StaticSpec.check []
      (.choice occasion thisCreature (.quality .color) (some (.players .opponent)) .openly) =
      [.kindAxisSort] := by
  cases occasion <;> decide

theorem deathPublishesArrival :
    NounPhrase.zone (GameEvent.after [] (Primitives.GameEvent.dies (a creature))) it =
      some .graveyard := by decide

theorem deathInterceptionKeepsDeparture :
    NounPhrase.zone (GameEvent.intro [] (Primitives.GameEvent.dies (a creature))) it =
      some .battlefield := by decide

theorem unknownDepartureCanBeConstrained :
    GameEvent.check [] (Primitives.GameEvent.dies (a .isCard)) = [] := by decide

theorem excludedDepartureIsRefused :
    GameEvent.check []
      (.zoneChange (a creature) (some (.anywhereBut [battlefield])) (some graveyard) .before) =
      [.zoneFits] := by decide

theorem excludingOnePlayersZoneDoesNotExcludeEveryPlayersZone :
    GameEvent.check []
      (.zoneChange (a (.and [.isCard, .inZone (graveyardOf (a .opponent))]))
        (some (.anywhereBut [graveyardOf .you])) (some exileZone) .before) = [] := by decide

theorem arrivalCannotBeDescribedInTheOrigin :
    GameEvent.check []
      (.zoneChange (a (.and [creature, .inZone library]))
        (some (.zones [library])) (some graveyard) .after) = [.zoneFits] := by decide

theorem arrivalCanBeDescribedAsCard :
    GameEvent.check []
      (.zoneChange (a (.and [.isCard, creature]))
        (some (.zones [library])) (some graveyard) .after) = [] := by decide

theorem anywhereAndBattlefieldHaveDifferentObservation :
    (match Primitives.GameEvent.putInto .this graveyard (some .anywhere) with
      | .zoneChange _ _ _ p => p | _ => .before) = .after ∧
    (match Primitives.GameEvent.putInto .this graveyard (some (.zones [battlefield])) with
      | .zoneChange _ _ _ p => p | _ => .after) = .before := by decide

theorem historyDoesNotMoveTheCurrentReference :
    GameEvent.mentioned (nomIntro [] (a creature))
      (.zoneChange it (some (.zones [battlefield])) (some graveyard) .before) = [] := by decide

theorem anyZoneChangeIsMeaningful :
    GameEvent.check [] (.zoneChange (a .isCard) none none .after) = [] := by decide

theorem playerIsNotAChangingObject :
    GameEvent.check [] (.zoneChange .you none none .after) =
      [.kindMismatch .object .player] := by decide

theorem sharedZoneCannotBeItsOwnDestination :
    GameEvent.check [] (.zoneChange (a creature) (some (.zones [battlefield]))
      (some battlefield) .before) = [.zoneCoherent] := by decide

theorem explicitCardLocationIsRetained :
    NounPhrase.zone [] (a (.and [.isCard, creature, .inZone graveyard])) =
      some .graveyard := by decide

theorem wholeTurnConditionNeedsNoPossessor :
    Condition.check [] (.duringPart .turn none) = [] := by decide

theorem partConditionChecksPlayer :
    Condition.check [] (.duringPart .combat (some thisCreature)) =
      [.kindMismatch .player .object] := by decide

theorem applicabilityReadsTheSpecSubject :
    StaticSpec.check []
      (Primitives.StaticSpec.partScope .turn (some (that .player))
        (.manaRetention (a .anyPlayer) (.unspent none))) = [] := by decide

theorem applicabilityPreservesNumericDefinition :
    Amount.check (StaticSpec.intro []
      (Primitives.StaticSpec.partScope .turn none (.letterDefinition .x (.lit 2))))
      (.letter .x) = [] := by decide

theorem ownedOutsideCardsAreAdmitted :
    NounPhrase.check (some .object) []
      (allOf (.and [.isCard, creature, .hasPossessor .owner .you, .not (.inZone battlefield)])) =
      [] := by decide

theorem controlledGraveyardCardsAreRefused :
    NounPhrase.check (some .object) []
      (allOf (.and [.isCard, creature, .hasPossessor .controller .you, .inZone graveyard])) =
      [.zoneCoherent] := by decide

theorem controlledCreatureSpellsRemainIncluded :
    StaticSpec.check [] (.characteristicChange
      (allOf (.and [creature, spell, .hasPossessor .controller .you]))
      [.everyTypeOf .adds .creature]) = [] := by decide

theorem beforeObservationRetainsKnownUnspecifiedOrigin :
    NounPhrase.zone (GameEvent.intro [] (Primitives.GameEvent.leaves (a creature) none)) it =
      some .battlefield := by decide

theorem emptyOriginIsNotAZoneChange :
    GameEvent.check [] (.zoneChange (a .isCard) (some (.zones [])) none .before) =
      [.zoneCoherent] := by decide

theorem excludingEveryOriginIsImpossible :
    GameEvent.check [] (.zoneChange (a .isCard)
      (some (.anywhereBut [battlefield, graveyard, exileZone, hand, library, stack, commandZone]))
      none .before) = [.zoneCoherent] := by decide

theorem placelessCardsDoNotAcquireBattlefieldEvidence :
    NounPhrase.zone (nomIntro []
      (a (.and [.isCard, creature, .not (.inZone battlefield)]))) it = none := by decide

theorem destinationReadsTheSourcePossessor :
    GameEvent.check [] (.zoneChange (a .isCard)
      (some (.zones [graveyardOf (a .opponent)])) (some (handOf (that .player))) .before) =
      [] := by decide

theorem bodyReadsTheSourcePossessor :
    NounPhrase.check (some .player)
      (GameEvent.after [] (.zoneChange (a .isCard)
        (some (.zones [graveyardOf (a .opponent)])) (some (handOf (that .player))) .before))
      (that .player) = [] := by decide

theorem excludingYourGraveyardAllowsAnOpponentsGraveyard :
    NounPhrase.check (some .object) []
      (a (.and [.isCard, .inZone (graveyardOf (a .opponent)),
        .not (.inZone (graveyardOf .you))])) = [] := by decide

theorem identicalQualifiedZoneAndItsNegationConflict :
    NounPhrase.check (some .object) []
      (a (.and [.isCard, .inZone (graveyardOf .you),
        .not (.inZone (graveyardOf .you))])) = [.zoneCoherent] := by decide

end Semantics.Proofs.EventPatterns
