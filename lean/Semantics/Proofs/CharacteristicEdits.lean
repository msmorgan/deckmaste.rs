import Semantics.Definitions
import Semantics.Check.Card

open Semantics Semantics.Templates Semantics.Definitions

namespace Semantics.Proofs.CharacteristicEdits

theorem absentAxisAndSetEmptyDiffer :
    ({} : TypeLineChanges).types = none ∧
    ({types := some []} : TypeLineChanges).types = some [] := by decide

theorem setEmptyIsAnEdit :
    StaticSpec.check (DRS.ofList []) (.characteristicChange thisCreature
      [.typeLine .sets {subtypes := some []}]) = [] := by decide

theorem emptyRemovalIsRefused :
    StaticSpec.check (DRS.ofList []) (.characteristicChange thisCreature
      [.typeLine .loses {subtypes := some []}]) = [.lineNonEmpty] := by decide

theorem absentEditIsRefused :
    StaticSpec.check (DRS.ofList []) (.characteristicChange thisCreature []) = [.nonEmpty] := by decide

theorem setAndAddStayDistinct :
    CharacteristicEdit.typeLine .sets {types := some [.artifact]} ≠
      .typeLine .adds {types := some [.artifact]} := by
  intro h
  cases h

theorem compoundTypeWritesHostSubtypes :
    StaticSpec.check (DRS.ofList []) (.characteristicChange thisCreature
      [.typeLine .adds {subtypes := some [landType "Forest"]},
       .typeLine .adds {types := some [.land]}]) = [] := by decide

theorem copyCanRemoveLegendary :
    CopyExcept.check (DRS.ofList []) (.edits [.typeLine .loses {supertypes := some [.legendary]}]) = [] := by decide

theorem copyCanSetEmptyColor :
    CopyExcept.check (DRS.ofList []) (.edits [.colors .sets (.some [])]) = [] := by decide

theorem copyCannotAddEmptyColor :
    CopyExcept.check (DRS.ofList []) (.edits [.colors .adds (.some [])]) = [.nonEmpty] := by decide

theorem specifiedAbilityRemoval :
    StaticSpec.check (DRS.ofList []) (.characteristicChange thisCreature
      [.removedAbilities (.specified [.term (.the (.named "flying"))])]) = [] := by decide

theorem familyAbilityRemoval :
    StaticSpec.check (DRS.ofList []) (.characteristicChange thisCreature
      [.removedAbilities (.family .anyActivated)]) = [] := by decide

theorem allAbilityRemoval :
    StaticSpec.check (DRS.ofList []) (.characteristicChange thisCreature
      [.removedAbilities (.allExcept none)]) = [] := by decide

theorem unknownAbilityFamilyIsRefused :
    StaticSpec.check (DRS.ofList []) (.characteristicChange thisCreature
      [.removedAbilities (.family (.keyword (.named "flyign")))]) = [.knownKeywordTerm] := by decide

theorem numericEditsShareOneLocalScope :
    CopyExcept.check (DRS.ofList []) (.edits
      [.stat .sets .power (powerOf (target isCreature)),
       .stat .sets .toughness (powerOf it)]) = [] := by decide

theorem numericEditsDoNotPublishAcrossCopyExceptions :
    CopyExcept.checkAll (DRS.ofList [])
      [.edits [.stat .sets .power (powerOf (target isCreature))],
       .edits [.stat .sets .toughness (powerOf it)]] =
      [.anaphor .bare .one 0] := by decide

theorem subjectIsPublishedBeforeItsEdits :
    StaticSpec.check (DRS.ofList []) (.characteristicChange (target isCreature)
      [.stat .sets .power (powerOf it)]) = [] := by decide

theorem copySpecificAdditionsRemainSeparate :
    CopyExcept.checkAll (DRS.ofList []) [.ability (keyword (.named "flying")),
      .entersWithCounters (.lit 1) p1p1Counter .fresh] = [] := by decide


theorem abilityOnlyCopyInputNeedsNoEmptyEditBlock :
    CopyExcept.checkAll (DRS.ofList []) (copyCharacteristics {text := [keyword (.named "flying")]} false) = [] := by decide

end Semantics.Proofs.CharacteristicEdits
