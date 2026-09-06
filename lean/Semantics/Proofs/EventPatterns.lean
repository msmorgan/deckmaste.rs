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

end Semantics.Proofs.EventPatterns
