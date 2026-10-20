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

/-- A removed bundle's mana cost is removed, never set: every written value of a `loses` bundle
is removed. -/
theorem lostBundleRemovesItsManaCost :
    (match QualityPayload.edits .loses
        (.bundle { characteristics := { cost := some [generic 2] } } none) with
      | [.typeLine .loses _, .manaCost none] => true
      | _ => false) = true := by decide

/-- An added bundle sets its written name as it sets its other values. -/
theorem addedBundleSetsItsName :
    (match QualityPayload.edits .adds
        (.bundle { characteristics := { name := some "Ajani's Pridemate" } } none) with
      | [.typeLine .adds _, .name .sets _] => true
      | _ => false) = true := by decide

/-- "It's still a land" keeps every prior type, so the type line is added under a set
[CR#205.1b]. -/
theorem retainedTypeLineIsAdded :
    (match QualityPayload.edits .sets
        (.bundle { characteristics := { types := [.creature] } } (some .land)) with
      | [.typeLine .adds _] => true
      | _ => false) = true := by decide

/-- "This creature's power is equal to …" is a set of its power by its own static ability, so it
fills the printed `*` [CR#604.3a]; the same set of another object's power does not. -/
theorem selfSetDefinesPower :
    (ptDefinition thisCreature .powerAlone (.lit 2)).definedStats = [.power] ∧
      (ptDefinition (target isCreature) .powerAlone (.lit 2)).definedStats = [] := by decide

/-- "Switch target creature's power and toughness": a Characteristic Edit [CR#613.4d]. -/
theorem ptSwitchIsAnEdit :
    StaticSpec.check (DRS.ofList []) (ptSwitch (target isCreature)) = [] := by decide

end Semantics.Proofs.CharacteristicEdits
