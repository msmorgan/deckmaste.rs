import Semantics
import Semantics.Definitions
import Semantics.Check.Card

/-!
# Semantics.Cards.Copy

Port of `idris/src/Experimental/Cards/Copy.idr`: the printed cards of the Copy family. A bench
item that is an ability, an instruction, or a static clause rather than a whole card is a plain
definition paired with an `ok…` theorem that runs the checker on it, as its Idris elaboration
did.
-/

open Semantics Semantics.Templates Semantics.Definitions

namespace Semantics.Cards

/-- Display of Power: "this spell can't be copied". -/
def displayOfPowerCopyLock : StaticSpec := cant any (copySpell «this» anyAmount)
theorem okDisplayOfPowerCopyLock : StaticSpec.check (DRS.ofList []) displayOfPowerCopyLock = [] := by decide

/-- Repeated Reverberation -/
def repeatedReverberation : Spelled := spelled <| .singleFaced
  { characteristics :=
    { name := "Repeated Reverberation", cost := some [generic 2, pip .red, pip .red],
      types := [.instant],
      text :=
        [ spell (delayWithin
            (does (actor you) (procedure .cast ((a (and
              [isInstant, isSpell]))) itsOwnCost))
            thisTurn
            (sequentially
              [ copy .fromStack (that .stack) (.lit 2) [],
                may (chooseNewTargets (those .copy)) ])
            [ does (actor you) (procedure .cast ((a
              (and [isSorcery, isSpell]))) itsOwnCost),
              does (actor you) (procedure .activate (a (abilityHead .loyalty)) itsOwnCost) ]) ] } }

/-- Frontline Heroism -/
def frontlineHeroismCopy : Ability :=
  whenever
    (does (actor you) (procedure .cast ((a (and [ isSpell,
                 targeting (a (and [isCreature,
                   hasPossessor .controller you]))
                   .soleTarget ]))) itsOwnCost))
    (sequentially
      [ createToken
          { characteristics :=
            { colors := [.red], types := [.creature], subtypes := [creatureType "Soldier"],
              text := [keyword (.named "haste")], power := stat 1, toughness := stat 1 } },
        copy .fromStack (that .spell) (.lit 1) [],
        copyTargets (that .copy) (that .token) ])
theorem okFrontlineHeroismCopy : Ability.check (DRS.ofList []) frontlineHeroismCopy = [] := by decide

/-- Flawless Forgery -/
def flawlessForgeryLine : Action :=
  sequentially
    [ exile (target (and [isInstantOrSorcery, in_ (graveyardOf (a (isOpponentOf you)))])),
      copy .fromCardZone (that .card) (.lit 1) [],
      establish
        (can (actor you) (cast (that .copy) (payment := withoutPaying))) ]
theorem okFlawlessForgeryLine : Action.check (DRS.ofList []) flawlessForgeryLine = [] := by decide

/-- Twincast -/
def twincast : Spelled := spelled <| .singleFaced
  { characteristics :=
    { name := "Twincast", cost := some [pip .blue, pip .blue], types := [.instant],
      text :=
        [ spell (sequentially
            [ copy .fromStack (target (and [isInstantOrSorcery, isSpell])) (.lit 1) [],
              may (chooseNewTargets (that .copy)) ]) ] } }

/-- Fork -/
def fork : Spelled := spelled <| .singleFaced
  { characteristics :=
    { name := "Fork", cost := some [pip .red, pip .red], types := [.instant],
      text :=
        [ spell (sequentially
            [ copy .fromStack (target (and [isInstantOrSorcery, isSpell])) (.lit 1)
                [isColor .red],
              may (chooseNewTargets (that .copy)) ]) ] } }

/-- Meletis Charlatan -/
def meletisCharlatan : Spelled := spelled <| .singleFaced
  { characteristics :=
    { name := "Meletis Charlatan", cost := some [generic 2, pip .blue], types := [.creature],
      subtypes := [creatureType "Human", creatureType "Wizard"],
      text :=
        [ activated (compound [mana [generic 2, pip .blue], tapSymbol])
            (sequentially
              [ instruct (controllerOf (target (and [isInstantOrSorcery, isSpell]))) (copy .fromStack it (.lit 1) []),
                instruct (that .player) (may (chooseNewTargets (that .copy))) ]) ],
      power := stat 2, toughness := stat 3 } }

/-- Echo Mage, level 4+ -/
def echoMagesFourthLevel : Ability :=
  activated (compound [mana [pip .blue, pip .blue], tapSymbol])
    (sequentially
      [ copy .fromStack (target (and [isInstantOrSorcery, isSpell])) (.lit 2) [],
        may (chooseNewTargets (those .copy)) ])
theorem okEchoMagesFourthLevel : Ability.check (DRS.ofList []) echoMagesFourthLevel = [] := by decide

/-- Strionic Resonator -/
def strionicResonator : Spelled := spelled <| .singleFaced
  { characteristics :=
    { name := "Strionic Resonator", cost := some [generic 2], types := [.artifact],
      text :=
        [ activated (compound [mana [generic 2], tapSymbol])
            (sequentially
              [ copy .fromStack
                  (target (and [abilityHead .anyTriggered, hasPossessor .controller you]))
                  (.lit 1) [],
                may (chooseNewTargets (that (.copied .ability))) ]) ] } }

/-- Mister Fantastic -/
def misterFantasticCopy : Ability :=
  activated (compound [mana [pip .red, pip .green, pip .white, pip .blue], tapSymbol])
    (sequentially
      [ copy .fromStack
          (target (and [abilityHead .anyTriggered, hasPossessor .controller you]))
          (.lit 2) [],
        may (chooseNewTargets (those (.copied .ability))) ])
theorem okMisterFantasticCopy : Ability.check (DRS.ofList []) misterFantasticCopy = [] := by decide

/-- Rowan's Talent -/
def rowansTalentCopy : Ability :=
  whenever
    (does (actor you) (procedure .activate (a (and [ abilityHead .loyalty,
                 abilityOf (attachHost .enchanted (.type .planeswalker)) ])) itsOwnCost))
    (sequentially
      [ copy .fromStack (that .ability) (.lit 1) [],
        may (chooseNewTargets (that (.copied .ability))) ])
theorem okRowansTalentCopy : Ability.check (DRS.ofList []) rowansTalentCopy = [] := by decide

/-- Rings of Brighthearth -/
def ringsOfBrighthearth : Spelled := spelled <| .singleFaced
  { characteristics :=
    { name := "Rings of Brighthearth", cost := some [generic 3], types := [.artifact],
      text :=
        [ triggeredIf (does (actor you) (procedure .activate (a (abilityHead .anyActivated)) itsOwnCost))
            (not (matches_ (that .ability) isManaAbility))
            (may (pay (mana [generic 2]) .once)
              (some (sequentially
                [ copy .fromStack (that .ability) (.lit 1) [],
                  may (chooseNewTargets (that (.copied .ability))) ]))) ] } }

/-- Iron Man, Bleeding Edge -/
def ironManBleedingEdge : Spelled := spelled <| .singleFaced
  { characteristics :=
    { name := "Iron Man, Bleeding Edge", cost := some [generic 3, pip .blue, pip .blue],
      supertypes := [.legendary], types := [.artifact, .creature],
      subtypes := [creatureType "Human", creatureType "Hero"],
      text :=
        [ keyword (.named "flying"),
          triggered (does (actor you) (procedure .cast ((a
            (and [isArtifact, isSpell]))) itsOwnCost))
            (doOnlyOnceEachTurn (may (copy .fromStack it (.lit 1) [isntLegendary])))
                ],
      power := stat 3, toughness := stat 5 } }

/-- Donal, Herald of Wings -/
def donalHeraldOfWings : Spelled := spelled <| .singleFaced
  { characteristics :=
    { name := "Donal, Herald of Wings", cost := some [generic 2, pip .blue, pip .blue],
      supertypes := [.legendary], types := [.creature],
      subtypes := [creatureType "Human", creatureType "Wizard"],
      text :=
        [ triggered
            (does (actor you) (procedure .cast ((a (and [ isCreature, isSpell, not
                (hasSupertype .legendary),
                         hasKeyword (.the (.named "flying")) ]))) itsOwnCost))
            (doOnlyOnceEachTurn (may
              (copy .fromStack it (.lit 1)
                (copyCharacteristics {
                           subtypes := [creatureType "Spirit"], power := stat 1,
                           toughness := stat 1 } true)))) ],
      power := stat 3, toughness := stat 3 } }

/-- Tawnos, the Toymaker -/
def tawnosTheToymaker : Spelled := spelled <| .singleFaced
  { characteristics :=
    { name := "Tawnos, the Toymaker", cost := some [generic 3, pip .green, pip .blue],
      supertypes := [.legendary], types := [.creature],
      subtypes := [creatureType "Human", creatureType "Artificer"],
      text :=
        [ whenever
            (does (actor you) (procedure .cast ((a (and [ or [
                hasSubtype (creatureType "Beast"),
                               hasSubtype (creatureType "Bird") ],
                         isCreature, isSpell ]))) itsOwnCost))
            (may (copy .fromStack it (.lit 1) [isAlso [.artifact] []])) ],
      power := stat 3, toughness := stat 5 } }

/-- Bonus Round -/
def bonusRound : Spelled := spelled <| .singleFaced
  { characteristics :=
    { name := "Bonus Round", cost := some [generic 1, pip .red, pip .red], types := [.sorcery],
      text :=
        [ spell (delayWithin (does (actor (a isPlayer)) (procedure .cast ((a
              (and [isInstantOrSorcery, isSpell]))) itsOwnCost)) untilEndOfTurn (sequentially
              [ instruct (that .player) (copy .fromStack it (.lit 1) []),
                instruct (that .player) (may (chooseNewTargets (that .copy))) ])) ] } }

/-- Melek, Izzet Paragon -/
def melekIzzetParagon : Spelled := spelled <| .singleFaced
  { characteristics :=
    { name := "Melek, Izzet Paragon", cost := some [generic 4, pip .blue, pip .red],
      supertypes := [.legendary], types := [.creature],
      subtypes := [creatureType "Weird", creatureType "Wizard"],
      text :=
        [ static (visibility .reveal you topOfLibrary),
          static
            (can (actor you) (cast (allOf (and [isSpell, isInstantOrSorcery])) (from_ := some onTop))),
          whenever
            (does (actor you) (procedure .cast ((a (and
              [isInstantOrSorcery, isSpell]))) itsOwnCost none (some (yourLibrary))))
            (sequentially
              [ copy .fromStack it (.lit 1) [],
                may (chooseNewTargets (that .copy)) ]) ],
      power := stat 2, toughness := stat 4 } }

/-- Pyromancer's Goggles -/
def pyromancersGogglesMana : Ability :=
  activated tapSymbol
    (addMana (.lit 1) (manaOf [[.of .red]])
      [ whenSpentOn .triggersThen false
          (a (and [hasColor .red, isInstantOrSorcery, isSpell]))
          (sequentially
            [ copy .fromStack (that .spell) (.lit 1) [],
              may (chooseNewTargets (that .copy)) ]) ])
theorem okPyromancersGogglesMana : Ability.check (DRS.ofList []) pyromancersGogglesMana = [] := by decide

end Semantics.Cards
