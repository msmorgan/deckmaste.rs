import Semantics.Macros
import Semantics.Check.Card

open Semantics Semantics.Macros

namespace Semantics.Proofs.CharacteristicEdits

theorem absentAxisAndSetEmptyDiffer :
    ({} : TypeLineChanges).types = none ∧
    ({types := some []} : TypeLineChanges).types = some [] := by decide

theorem setEmptyIsAnEdit :
    StaticSpec.check [] (.characteristicChange thisCreature
      [.typeLine .sets {subtypes := some []}]) = [] := by decide

theorem emptyRemovalIsRefused :
    StaticSpec.check [] (.characteristicChange thisCreature
      [.typeLine .loses {subtypes := some []}]) = [.becomesOk] := by decide

theorem absentEditIsRefused :
    StaticSpec.check [] (.characteristicChange thisCreature []) = [.becomesOk] := by decide

theorem setAndAddStayDistinct :
    CharacteristicEdit.typeLine .sets {types := some [.artifact]} ≠
      .typeLine .adds {types := some [.artifact]} := by
  intro h
  cases h

theorem compoundTypeWritesHostSubtypes :
    StaticSpec.check [] (.characteristicChange thisCreature
      [.typeLine .adds {subtypes := some [landType "Forest"]},
       .typeLine .adds {types := some [.land]}]) = [] := by decide

theorem copyCanRemoveLegendary :
    CopyExcept.check [] (.edits [.typeLine .loses {supertypes := some [.legendary]}]) = [] := by decide

theorem copyCanSetEmptyColor :
    CopyExcept.check [] (.edits [.colors .sets (.some [])]) = [] := by decide

theorem copyCannotAddEmptyColor :
    CopyExcept.check [] (.edits [.colors .adds (.some [])]) = [.becomesOk] := by decide

theorem copyCannotRemoveName :
    CopyExcept.check [] (.edits [.name .loses "A name"]) = [.becomesOk] := by decide

theorem copyCannotAddBasePower :
    CopyExcept.check [] (.edits [.stat .adds .power (.lit 2)]) = [.becomesOk] := by decide

theorem malformedManaCostIsRefused :
    CopyExcept.check [] (.edits [.manaCost (some [.hybrid (.specific (.of .blue)) .blue])]) =
      [.manaSymbolOk] := by decide

theorem specifiedAbilityRemoval :
    StaticSpec.check [] (.characteristicChange thisCreature
      [.removedAbilities (.specified [.term (.the "Flying")])]) = [] := by decide

theorem familyAbilityRemoval :
    StaticSpec.check [] (.characteristicChange thisCreature
      [.removedAbilities (.family .anyActivated)]) = [] := by decide

theorem allAbilityRemoval :
    StaticSpec.check [] (.characteristicChange thisCreature
      [.removedAbilities (.allExcept none)]) = [] := by decide

theorem unknownAbilityFamilyIsRefused :
    StaticSpec.check [] (.characteristicChange thisCreature
      [.removedAbilities (.family (.keyword "Flyign"))]) = [.knownKeywordTerm] := by decide

theorem numericEditsShareOneLocalScope :
    CopyExcept.check [] (.edits
      [.stat .sets .power (powerOf (target creature)),
       .stat .sets .toughness (powerOf it)]) = [] := by decide

theorem numericEditsDoNotPublishAcrossCopyExceptions :
    CopyExcept.checkAll []
      [.edits [.stat .sets .power (powerOf (target creature))],
       .edits [.stat .sets .toughness (powerOf it)]] =
      [.anaphor .bare .one 0] := by decide

theorem subjectIsNotPublishedBeforeItsEdits :
    StaticSpec.check [] (.characteristicChange (target creature)
      [.stat .sets .power (powerOf it)]) =
      [.anaphor .bare .one 0] := by decide

theorem copySpecificAdditionsRemainSeparate :
    CopyExcept.checkAll [] [.ability (keyword "Flying"),
      .entersWithCounters (.lit 1) plusOnePlusOne .fresh] = [] := by decide


theorem abilityOnlyCopyInputNeedsNoEmptyEditBlock :
    CopyExcept.checkAll [] (copyCharacteristics {text := [keyword "Flying"]} false) = [] := by decide

end Semantics.Proofs.CharacteristicEdits
