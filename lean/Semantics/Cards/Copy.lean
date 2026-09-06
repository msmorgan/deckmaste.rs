import Semantics
import Semantics.Macros
import Semantics.Check.Card

/-!
# Semantics.Cards.Copy

Port of `idris/src/Experimental/Cards/Copy.idr`: the printed cards of the Copy family. A bench
item that is an ability, an instruction, or a static clause rather than a whole card is a plain
definition paired with an `ok…` theorem that runs the checker on it, as its Idris elaboration
did.
-/

open Semantics Semantics.Macros

namespace Semantics.Cards

/-- Display of Power: "this spell can't be copied". -/
def displayOfPowerCopyLock : StaticSpec := objectCant (.core .copy) .this
theorem okDisplayOfPowerCopyLock : StaticSpec.check [] displayOfPowerCopyLock = [] := by decide

/-- Repeated Reverberation -/
def repeatedReverberation : Spelled := spelled <| .singleFaced
  { characteristics :=
    { name := "Repeated Reverberation", cost := some [generic 2, pip .red, pip .red],
      types := [.instant],
      text :=
        [ .spell none (.delay
            (.casts .you (a (.and [instant, spell])) none)
            [ .casts .you (a (.and [sorcery, spell])) none,
              .activates .you (a (.abilityHead .loyalty)) ]
            (some .thisTurn)
            (.sequence
              [ .copy .fromStack (that .stack) (.lit 2) [] (agent := .you),
                offer (.chooseNewTargets (those .copy)) (agent := .you) ])) ] } }

/-- Frontline Heroism -/
def frontlineHeroismCopy : Ability :=
  whenever
    (.casts .you
      (a (.and [ spell,
                 .targets (a (.and [creature, .hasPossessor .controller .you])) .soleTarget ]))
      none)
    (.sequence
      [ create (.lit 1)
          { characteristics :=
            { colors := [.red], types := [.creature], subtypes := [creatureType "Soldier"],
              text := [keyword "Haste"], power := stat 1, toughness := stat 1 } },
        .copy .fromStack (that .spell) (.lit 1) [] (agent := .you),
        .copyTargets (that .copy) (that .token) ])
theorem okFrontlineHeroismCopy : Ability.check [] frontlineHeroismCopy = [] := by decide

/-- Flawless Forgery -/
def flawlessForgeryLine : Instruction :=
  .sequence
    [ exile (target (.and [instantOrSorcery, .inZone (graveyardOf (a .opponent))])),
      .copy .fromCardZone (that .card) (.lit 1) [] (agent := .you),
      .establish
        (mayPlayDeed (.action "Cast") .you (that .copy) none
          (.play none none none false .withoutPaying))
        none ]
theorem okFlawlessForgeryLine : Instruction.check [] flawlessForgeryLine = [] := by decide

/-- Twincast -/
def twincast : Spelled := spelled <| .singleFaced
  { characteristics :=
    { name := "Twincast", cost := some [pip .blue, pip .blue], types := [.instant],
      text :=
        [ .spell none (.sequence
            [ .copy .fromStack (target (.and [instantOrSorcery, spell])) (.lit 1) [] (agent :=
                .you),
              offer (.chooseNewTargets (that .copy)) (agent := .you) ]) ] } }

/-- Fork -/
def fork : Spelled := spelled <| .singleFaced
  { characteristics :=
    { name := "Fork", cost := some [pip .red, pip .red], types := [.instant],
      text :=
        [ .spell none (.sequence
            [ .copy .fromStack (target (.and [instantOrSorcery, spell])) (.lit 1)
                [.color .red] (agent := .you),
              offer (.chooseNewTargets (that .copy)) (agent := .you) ]) ] } }

/-- Meletis Charlatan -/
def meletisCharlatan : Spelled := spelled <| .singleFaced
  { characteristics :=
    { name := "Meletis Charlatan", cost := some [generic 2, pip .blue], types := [.creature],
      subtypes := [creatureType "Human", creatureType "Wizard"],
      text :=
        [ activated (.compound [.mana [generic 2, pip .blue], .tapSymbol])
            (.sequence
              [ .copy .fromStack it (.lit 1) [] (agent := (controllerOf (target (.and
                  [instantOrSorcery, spell])))),
                offer (.chooseNewTargets (that .copy)) (agent := (that .player)) ]) ],
      power := stat 2, toughness := stat 3 } }

/-- Echo Mage, level 4+ -/
def echoMagesFourthLevel : Ability :=
  activated (.compound [.mana [pip .blue, pip .blue], .tapSymbol])
    (.sequence
      [ .copy .fromStack (target (.and [instantOrSorcery, spell])) (.lit 2) [] (agent := .you),
        offer (.chooseNewTargets (those .copy)) (agent := .you) ])
theorem okEchoMagesFourthLevel : Ability.check [] echoMagesFourthLevel = [] := by decide

/-- Strionic Resonator -/
def strionicResonator : Spelled := spelled <| .singleFaced
  { characteristics :=
    { name := "Strionic Resonator", cost := some [generic 2], types := [.artifact],
      text :=
        [ activated (.compound [.mana [generic 2], .tapSymbol])
            (.sequence
              [ .copy .fromStack
                  (target (.and [.abilityHead .anyTriggered, .hasPossessor .controller .you]))
                  (.lit 1) [] (agent := .you),
                offer (.chooseNewTargets (that (.copied .ability))) (agent := .you) ]) ] } }

/-- Mister Fantastic -/
def misterFantasticCopy : Ability :=
  activated (.compound [.mana [pip .red, pip .green, pip .white, pip .blue], .tapSymbol])
    (.sequence
      [ .copy .fromStack
          (target (.and [.abilityHead .anyTriggered, .hasPossessor .controller .you]))
          (.lit 2) [] (agent := .you),
        offer (.chooseNewTargets (those (.copied .ability))) (agent := .you) ])
theorem okMisterFantasticCopy : Ability.check [] misterFantasticCopy = [] := by decide

/-- Rowan's Talent -/
def rowansTalentCopy : Ability :=
  whenever
    (.activates .you
      (a (.and [ .abilityHead .loyalty,
                 .abilityOf (.attachHost .enchanted (.type .planeswalker)) ])))
    (.sequence
      [ .copy .fromStack (that .ability) (.lit 1) [] (agent := .you),
        offer (.chooseNewTargets (that (.copied .ability))) (agent := .you) ])
theorem okRowansTalentCopy : Ability.check [] rowansTalentCopy = [] := by decide

/-- Rings of Brighthearth -/
def ringsOfBrighthearth : Spelled := spelled <| .singleFaced
  { characteristics :=
    { name := "Rings of Brighthearth", cost := some [generic 3], types := [.artifact],
      text :=
        [ triggeredIf (.activates .you (a (.abilityHead .anyActivated)))
            (itIsntAnAbility .isManaAbility)
            (.offer (.pay (.mana [generic 2]) .once (agent := .you))
              (some (.sequence
                [ .copy .fromStack (that .ability) (.lit 1) [] (agent := .you),
                  offer (.chooseNewTargets (that (.copied .ability))) (agent := .you) ]))
              none (agent := .you)) ] } }

/-- Iron Man, Bleeding Edge -/
def ironManBleedingEdge : Spelled := spelled <| .singleFaced
  { characteristics :=
    { name := "Iron Man, Bleeding Edge", cost := some [generic 3, pip .blue, pip .blue],
      supertypes := [.legendary], types := [.artifact, .creature],
      subtypes := [creatureType "Human", creatureType "Hero"],
      text :=
        [ keyword "Flying",
          triggeredOnlyOnce (.casts .you (a (.and [artifact, spell])) none) .actionOncePerTurn
            (offer (.copy .fromStack it (.lit 1) [.nonlegendary] (agent := .you)) (agent := .you))
                ],
      power := stat 3, toughness := stat 5 } }

/-- Donal, Herald of Wings -/
def donalHeraldOfWings : Spelled := spelled <| .singleFaced
  { characteristics :=
    { name := "Donal, Herald of Wings", cost := some [generic 2, pip .blue, pip .blue],
      supertypes := [.legendary], types := [.creature],
      subtypes := [creatureType "Human", creatureType "Wizard"],
      text :=
        [ triggeredOnlyOnce
            (.casts .you
              (a (.and [ creature, spell, .not (.hasSupertype .legendary),
                         .hasKeyword (.the "Flying") ]))
              none)
            .actionOncePerTurn
            (offer
              (.copy .fromStack it (.lit 1)
                [ .chars { subtypes := [creatureType "Spirit"], power := stat 1,
                           toughness := stat 1 } true ] (agent := .you)) (agent := .you)) ],
      power := stat 3, toughness := stat 3 } }

/-- Tawnos, the Toymaker -/
def tawnosTheToymaker : Spelled := spelled <| .singleFaced
  { characteristics :=
    { name := "Tawnos, the Toymaker", cost := some [generic 3, pip .green, pip .blue],
      supertypes := [.legendary], types := [.creature],
      subtypes := [creatureType "Human", creatureType "Artificer"],
      text :=
        [ whenever
            (.casts .you
              (a (.and [ .or [ .hasSubtype (creatureType "Beast"),
                               .hasSubtype (creatureType "Bird") ],
                         creature, spell ]))
              none)
            (offer (.copy .fromStack it (.lit 1) [.types [.artifact] []] (agent := .you)) (agent :=
                .you)) ],
      power := stat 3, toughness := stat 5 } }

/-- Bonus Round -/
def bonusRound : Spelled := spelled <| .singleFaced
  { characteristics :=
    { name := "Bonus Round", cost := some [generic 1, pip .red, pip .red], types := [.sorcery],
      text :=
        [ .spell none (.delay
            (.casts (a .anyPlayer) (a (.and [instantOrSorcery, spell])) none)
            [] (some untilEndOfTurn)
            (.sequence
              [ .copy .fromStack it (.lit 1) [] (agent := (that .player)),
                offer (.chooseNewTargets (that .copy)) (agent := (that .player)) ])) ] } }

/-- Melek, Izzet Paragon -/
def melekIzzetParagon : Spelled := spelled <| .singleFaced
  { characteristics :=
    { name := "Melek, Izzet Paragon", cost := some [generic 4, pip .blue, pip .red],
      supertypes := [.legendary], types := [.creature],
      subtypes := [creatureType "Weird", creatureType "Wizard"],
      text :=
        [ .static (.visibility .reveal .you .topOfLibrary),
          .static
            (mayPlayDeed (.action "Cast") .you (allOf (.and [spell, instantOrSorcery])) none
              (.play (some onTop) none none false .itsOwnCost)),
          whenever
            (.casts .you (a (.and [instantOrSorcery, spell])) (some yourLibrary))
            (.sequence
              [ .copy .fromStack it (.lit 1) [] (agent := .you),
                offer (.chooseNewTargets (that .copy)) (agent := .you) ]) ],
      power := stat 2, toughness := stat 4 } }

/-- Pyromancer's Goggles -/
def pyromancersGogglesMana : Ability :=
  activated .tapSymbol
    (.addMana (.lit 1) (.runs [[.of .red]])
      [ .onSpent .triggersThen false
          (a (.and [.colorIs .red, instantOrSorcery, spell]))
          (.sequence
            [ .copy .fromStack (that .spell) (.lit 1) [] (agent := .you),
              offer (.chooseNewTargets (that .copy)) (agent := .you) ]) ] (agent := .you))
theorem okPyromancersGogglesMana : Ability.check [] pyromancersGogglesMana = [] := by decide

end Semantics.Cards
