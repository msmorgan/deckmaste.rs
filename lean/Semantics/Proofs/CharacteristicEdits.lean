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
      [.typeLine .adds {subtypes := some [forest]},
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

/-- "This creature's power is equal to …" is a Characteristic-Defining Ability, so it fills the
printed `*` [CR#604.3a,613.4a]; an ordinary set of its power ("has base power 2") does not. -/
theorem definingAbilityDefinesPower :
    (ptDefinition thisCreature .powerAlone (.lit 2)).definedStats = [.power] ∧
      (StaticSpec.characteristicChange thisCreature [.stat .sets .power (.lit 2)]).definedStats =
        [] := by decide

/-- Tarmogoyf is one Characteristic-Defining Ability setting both stats, its toughness reading
the power's number back ("that number plus 1") [CR#604.3a]. -/
theorem tarmogoyfDefinesBothStats :
    let goyf := ptDefinitions thisCreature
      (distinctCount .cardType (allOf (cardIn (graveyardOf (bare isPlayer)))))
      (plus thatMuch (.lit 1))
    StaticSpec.check (DRS.ofList []) goyf = [] ∧ goyf.definedStats = [.power, .toughness] := by
  decide

/-- A Characteristic-Defining Ability defines its object's colors ("this creature is all
colors", devoid's "this object is colorless") or subtypes (changeling's "this object is every
creature type") [CR#604.3a,702.73a,702.114a]. -/
theorem definingAbilitiesDefineColorsAndSubtypes :
    StaticSpec.check (DRS.ofList []) (qualityDefinition thisCreature .sets (colored .every)) = [] ∧
    StaticSpec.check (DRS.ofList []) (qualityDefinition this_ .sets (colored (.some []))) = [] ∧
    StaticSpec.check (DRS.ofList []) (qualityDefinition this_ .adds (everyTypeOf .creature)) =
      [] ∧
    Ability.check (DRS.ofList []) changeling = [] ∧ Ability.check (DRS.ofList []) devoid = [] := by
  decide

/-- "As this creature enters, choose a creature type. This creature is the chosen type in addition
to its other types." (Adaptive Automaton, Metallic Mimic): a Characteristic-Defining Ability
read off a choice, as Shapeshifter's power is; depending on a choice is no condition
[CR#604.3a]. -/
theorem choiceDependentSubtypeDefinitionIsAccepted :
    Ability.checkText (DRS.ofList [])
      [ static (entersChoosing thisCreature (.subtype .creature)),
        static (qualityDefinition thisCreature .adds
          (chosenQuality (ofChosen (.subtype .creature)))) ] = [] := by decide

/-- A token is created with a Characteristic-Defining Ability the creating effect gives it
[CR#604.3a,111.3]. -/
theorem tokenIsCreatedWithADefiningAbility :
    Action.check (DRS.ofList [])
      (createObject (.lit 1) (.token (.written (creatureTokenOf (.lit 0) (.lit 0) [.green] []
        [static (ptDefinition thisCreature .bothEach (countOf (and [isLand, youControl])))])) [])) =
      [] := by decide

/-- Criterion (1): a Characteristic-Defining Ability defines colors, subtypes, power or toughness
[CR#604.3a]; raising its power defines nothing. -/
theorem definingAbilityDefinesOnlyColorsSubtypesAndStats :
    StaticSpec.check (DRS.ofList [])
      (.characteristicDefining thisCreature [.stat .adds .power (.lit 1)]) = [.cdaEdit] := by
  decide

/-- Criterion (3): a Characteristic-Defining Ability defines its own object, never another
[CR#604.3a]. -/
theorem definingAbilityDefinesItsOwnObject :
    StaticSpec.check (DRS.ofList [])
      (ptDefinition (allOf (and [isCreature, youControl])) .powerAlone (.lit 2)) =
      [.cdaSubject] := by decide

/-- Criteria (2) and (4): a granted ability is no Characteristic-Defining Ability, whether the
object grants it to itself or an effect puts it in force [CR#604.3a]; granting a keyword whose
definition is one (changeling) is an ordinary grant. -/
theorem definingAbilityIsNeverGranted :
    StaticSpec.check (DRS.ofList [])
      (abilityGrant thisCreature (static (ptDefinition thisCreature .powerAlone (.lit 2)))) =
      [.cdaGranted] ∧
    Action.check (DRS.ofList [])
      (establishThroughout (qualityDefinition thisCreature .sets (colored .every))
        untilEndOfTurn) = [.cdaGranted] ∧
    StaticSpec.check (DRS.ofList []) (abilityGrant thisCreature changeling) = [] := by decide

/-- Criterion (2): a Characteristic-Defining Ability a copy effect gives the copy, as an exception
("except it has 'This creature's power is equal to …'"), is one [CR#604.3a,707.9a]. -/
theorem copyExceptionMayGiveADefiningAbility :
    CopyExcept.check (DRS.ofList [])
      (.edits [.addedAbilities [static (ptDefinition thisCreature .powerAlone (.lit 2))]]) =
      [] := by decide

/-- Criterion (5): a Characteristic-Defining Ability sets its values unconditionally
[CR#604.3a]. -/
theorem definingAbilityIsUnconditional :
    StaticSpec.check (DRS.ofList [])
      (onlyWhile (ptDefinition thisCreature .powerAlone (.lit 2))
        (exists_ (and [isCreature, youControl]))) = [.cdaConditional] := by decide

/-- "Switch target creature's power and toughness": a Characteristic Edit [CR#613.4d]. -/
theorem ptSwitchIsAnEdit :
    StaticSpec.check (DRS.ofList []) (ptSwitch (target isCreature)) = [] := by decide

end Semantics.Proofs.CharacteristicEdits
