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
def displayOfPowerCopyLock : StaticSpec := objectCant (.core .copy) Primitives.NounPhrase.this
theorem okDisplayOfPowerCopyLock : StaticSpec.check [] displayOfPowerCopyLock = [] := by decide

/-- Repeated Reverberation -/
def repeatedReverberation : Spelled := spelled <| .singleFaced
  { characteristics :=
    { name := "Repeated Reverberation", cost := some [generic 2, pip .red, pip .red],
      types := [.instant],
      text :=
        [ Primitives.Ability.spell none (Primitives.Instruction.delay
            (Primitives.GameEvent.casts Primitives.NounPhrase.you (a (Primitives.Predicate.and [instant, spell])) none)
            [ Primitives.GameEvent.casts Primitives.NounPhrase.you (a (Primitives.Predicate.and [sorcery, spell])) none,
              Primitives.GameEvent.activates Primitives.NounPhrase.you (a (Primitives.Predicate.abilityHead .loyalty)) ]
            (some Primitives.Duration.thisTurn)
            (Primitives.Instruction.sequence
              [ Primitives.Instruction.copy .fromStack (that .stack) (.lit 2) [] (agent := Primitives.NounPhrase.you),
                offer (Primitives.Instruction.chooseNewTargets (those .copy)) (agent := Primitives.NounPhrase.you) ])) ] } }

/-- Frontline Heroism -/
def frontlineHeroismCopy : Ability :=
  whenever
    (Primitives.GameEvent.casts Primitives.NounPhrase.you
      (a (Primitives.Predicate.and [ spell,
                 Primitives.Predicate.targets (a (Primitives.Predicate.and [creature, Primitives.Predicate.hasPossessor .controller Primitives.NounPhrase.you])) .soleTarget ]))
      none)
    (Primitives.Instruction.sequence
      [ create (.lit 1)
          { characteristics :=
            { colors := [.red], types := [.creature], subtypes := [creatureType "Soldier"],
              text := [keyword "Haste"], power := stat 1, toughness := stat 1 } },
        Primitives.Instruction.copy .fromStack (that .spell) (.lit 1) [] (agent := Primitives.NounPhrase.you),
        Primitives.Instruction.copyTargets (that .copy) (that .token) ])
theorem okFrontlineHeroismCopy : Ability.check [] frontlineHeroismCopy = [] := by decide

/-- Flawless Forgery -/
def flawlessForgeryLine : Instruction :=
  Primitives.Instruction.sequence
    [ exile (target (Primitives.Predicate.and [instantOrSorcery, Primitives.Predicate.inZone (graveyardOf (a Primitives.Predicate.opponent))])),
      Primitives.Instruction.copy .fromCardZone (that .card) (.lit 1) [] (agent := Primitives.NounPhrase.you),
      Primitives.Instruction.establish
        (mayPlayDeed (.action "Cast") Primitives.NounPhrase.you (that .copy) none
          (Primitives.DeonticRider.play none none none false Primitives.PlayPayment.withoutPaying))
        none ]
theorem okFlawlessForgeryLine : Instruction.check [] flawlessForgeryLine = [] := by decide

/-- Twincast -/
def twincast : Spelled := spelled <| .singleFaced
  { characteristics :=
    { name := "Twincast", cost := some [pip .blue, pip .blue], types := [.instant],
      text :=
        [ Primitives.Ability.spell none (Primitives.Instruction.sequence
            [ Primitives.Instruction.copy .fromStack (target (Primitives.Predicate.and [instantOrSorcery, spell])) (.lit 1) [] (agent :=
                Primitives.NounPhrase.you),
              offer (Primitives.Instruction.chooseNewTargets (that .copy)) (agent := Primitives.NounPhrase.you) ]) ] } }

/-- Fork -/
def fork : Spelled := spelled <| .singleFaced
  { characteristics :=
    { name := "Fork", cost := some [pip .red, pip .red], types := [.instant],
      text :=
        [ Primitives.Ability.spell none (Primitives.Instruction.sequence
            [ Primitives.Instruction.copy .fromStack (target (Primitives.Predicate.and [instantOrSorcery, spell])) (.lit 1)
                [Primitives.CopyExcept.color .red] (agent := Primitives.NounPhrase.you),
              offer (Primitives.Instruction.chooseNewTargets (that .copy)) (agent := Primitives.NounPhrase.you) ]) ] } }

/-- Meletis Charlatan -/
def meletisCharlatan : Spelled := spelled <| .singleFaced
  { characteristics :=
    { name := "Meletis Charlatan", cost := some [generic 2, pip .blue], types := [.creature],
      subtypes := [creatureType "Human", creatureType "Wizard"],
      text :=
        [ activated (Primitives.Cost.compound [Primitives.Cost.mana [generic 2, pip .blue], Primitives.Cost.tapSymbol])
            (Primitives.Instruction.sequence
              [ Primitives.Instruction.copy .fromStack it (.lit 1) [] (agent := (controllerOf (target (Primitives.Predicate.and
                  [instantOrSorcery, spell])))),
                offer (Primitives.Instruction.chooseNewTargets (that .copy)) (agent := (that .player)) ]) ],
      power := stat 2, toughness := stat 3 } }

/-- Echo Mage, level 4+ -/
def echoMagesFourthLevel : Ability :=
  activated (Primitives.Cost.compound [Primitives.Cost.mana [pip .blue, pip .blue], Primitives.Cost.tapSymbol])
    (Primitives.Instruction.sequence
      [ Primitives.Instruction.copy .fromStack (target (Primitives.Predicate.and [instantOrSorcery, spell])) (.lit 2) [] (agent := Primitives.NounPhrase.you),
        offer (Primitives.Instruction.chooseNewTargets (those .copy)) (agent := Primitives.NounPhrase.you) ])
theorem okEchoMagesFourthLevel : Ability.check [] echoMagesFourthLevel = [] := by decide

/-- Strionic Resonator -/
def strionicResonator : Spelled := spelled <| .singleFaced
  { characteristics :=
    { name := "Strionic Resonator", cost := some [generic 2], types := [.artifact],
      text :=
        [ activated (Primitives.Cost.compound [Primitives.Cost.mana [generic 2], Primitives.Cost.tapSymbol])
            (Primitives.Instruction.sequence
              [ Primitives.Instruction.copy .fromStack
                  (target (Primitives.Predicate.and [Primitives.Predicate.abilityHead .anyTriggered, Primitives.Predicate.hasPossessor .controller Primitives.NounPhrase.you]))
                  (.lit 1) [] (agent := Primitives.NounPhrase.you),
                offer (Primitives.Instruction.chooseNewTargets (that (.copied .ability))) (agent := Primitives.NounPhrase.you) ]) ] } }

/-- Mister Fantastic -/
def misterFantasticCopy : Ability :=
  activated (Primitives.Cost.compound [Primitives.Cost.mana [pip .red, pip .green, pip .white, pip .blue], Primitives.Cost.tapSymbol])
    (Primitives.Instruction.sequence
      [ Primitives.Instruction.copy .fromStack
          (target (Primitives.Predicate.and [Primitives.Predicate.abilityHead .anyTriggered, Primitives.Predicate.hasPossessor .controller Primitives.NounPhrase.you]))
          (.lit 2) [] (agent := Primitives.NounPhrase.you),
        offer (Primitives.Instruction.chooseNewTargets (those (.copied .ability))) (agent := Primitives.NounPhrase.you) ])
theorem okMisterFantasticCopy : Ability.check [] misterFantasticCopy = [] := by decide

/-- Rowan's Talent -/
def rowansTalentCopy : Ability :=
  whenever
    (Primitives.GameEvent.activates Primitives.NounPhrase.you
      (a (Primitives.Predicate.and [ Primitives.Predicate.abilityHead .loyalty,
                 Primitives.Predicate.abilityOf (Primitives.NounPhrase.attachHost .enchanted (.type .planeswalker)) ])))
    (Primitives.Instruction.sequence
      [ Primitives.Instruction.copy .fromStack (that .ability) (.lit 1) [] (agent := Primitives.NounPhrase.you),
        offer (Primitives.Instruction.chooseNewTargets (that (.copied .ability))) (agent := Primitives.NounPhrase.you) ])
theorem okRowansTalentCopy : Ability.check [] rowansTalentCopy = [] := by decide

/-- Rings of Brighthearth -/
def ringsOfBrighthearth : Spelled := spelled <| .singleFaced
  { characteristics :=
    { name := "Rings of Brighthearth", cost := some [generic 3], types := [.artifact],
      text :=
        [ triggeredIf (Primitives.GameEvent.activates Primitives.NounPhrase.you (a (Primitives.Predicate.abilityHead .anyActivated)))
            (itIsntAnAbility Primitives.Predicate.isManaAbility)
            (Primitives.Instruction.offer (Primitives.Instruction.pay (Primitives.Cost.mana [generic 2]) .once (agent := Primitives.NounPhrase.you))
              (some (Primitives.Instruction.sequence
                [ Primitives.Instruction.copy .fromStack (that .ability) (.lit 1) [] (agent := Primitives.NounPhrase.you),
                  offer (Primitives.Instruction.chooseNewTargets (that (.copied .ability))) (agent := Primitives.NounPhrase.you) ]))
              none (agent := Primitives.NounPhrase.you)) ] } }

/-- Iron Man, Bleeding Edge -/
def ironManBleedingEdge : Spelled := spelled <| .singleFaced
  { characteristics :=
    { name := "Iron Man, Bleeding Edge", cost := some [generic 3, pip .blue, pip .blue],
      supertypes := [.legendary], types := [.artifact, .creature],
      subtypes := [creatureType "Human", creatureType "Hero"],
      text :=
        [ keyword "Flying",
          triggeredOnlyOnce (Primitives.GameEvent.casts Primitives.NounPhrase.you (a (Primitives.Predicate.and [artifact, spell])) none) Primitives.UsageLimit.actionOncePerTurn
            (offer (Primitives.Instruction.copy .fromStack it (.lit 1) [Primitives.CopyExcept.nonlegendary] (agent := Primitives.NounPhrase.you)) (agent := Primitives.NounPhrase.you))
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
            (Primitives.GameEvent.casts Primitives.NounPhrase.you
              (a (Primitives.Predicate.and [ creature, spell, Primitives.Predicate.not (Primitives.Predicate.hasSupertype .legendary),
                         Primitives.Predicate.hasKeyword (.the "Flying") ]))
              none)
            Primitives.UsageLimit.actionOncePerTurn
            (offer
              (Primitives.Instruction.copy .fromStack it (.lit 1)
                [ Primitives.CopyExcept.chars {
                           subtypes := [creatureType "Spirit"], power := stat 1,
                           toughness := stat 1 } true ] (agent := Primitives.NounPhrase.you)) (agent := Primitives.NounPhrase.you)) ],
      power := stat 3, toughness := stat 3 } }

/-- Tawnos, the Toymaker -/
def tawnosTheToymaker : Spelled := spelled <| .singleFaced
  { characteristics :=
    { name := "Tawnos, the Toymaker", cost := some [generic 3, pip .green, pip .blue],
      supertypes := [.legendary], types := [.creature],
      subtypes := [creatureType "Human", creatureType "Artificer"],
      text :=
        [ whenever
            (Primitives.GameEvent.casts Primitives.NounPhrase.you
              (a (Primitives.Predicate.and [ Primitives.Predicate.or [ Primitives.Predicate.hasSubtype (creatureType "Beast"),
                               Primitives.Predicate.hasSubtype (creatureType "Bird") ],
                         creature, spell ]))
              none)
            (offer (Primitives.Instruction.copy .fromStack it (.lit 1) [Primitives.CopyExcept.types [.artifact] []] (agent := Primitives.NounPhrase.you)) (agent :=
                Primitives.NounPhrase.you)) ],
      power := stat 3, toughness := stat 5 } }

/-- Bonus Round -/
def bonusRound : Spelled := spelled <| .singleFaced
  { characteristics :=
    { name := "Bonus Round", cost := some [generic 1, pip .red, pip .red], types := [.sorcery],
      text :=
        [ Primitives.Ability.spell none (Primitives.Instruction.delay
            (Primitives.GameEvent.casts (a Primitives.Predicate.anyPlayer) (a (Primitives.Predicate.and [instantOrSorcery, spell])) none)
            [] (some untilEndOfTurn)
            (Primitives.Instruction.sequence
              [ Primitives.Instruction.copy .fromStack it (.lit 1) [] (agent := (that .player)),
                offer (Primitives.Instruction.chooseNewTargets (that .copy)) (agent := (that .player)) ])) ] } }

/-- Melek, Izzet Paragon -/
def melekIzzetParagon : Spelled := spelled <| .singleFaced
  { characteristics :=
    { name := "Melek, Izzet Paragon", cost := some [generic 4, pip .blue, pip .red],
      supertypes := [.legendary], types := [.creature],
      subtypes := [creatureType "Weird", creatureType "Wizard"],
      text :=
        [ Primitives.Ability.static (Primitives.StaticSpec.visibility .reveal Primitives.NounPhrase.you Primitives.VisibleThing.topOfLibrary),
          Primitives.Ability.static
            (mayPlayDeed (.action "Cast") Primitives.NounPhrase.you (allOf (Primitives.Predicate.and [spell, instantOrSorcery])) none
              (Primitives.DeonticRider.play (some onTop) none none false Primitives.PlayPayment.itsOwnCost)),
          whenever
            (Primitives.GameEvent.casts Primitives.NounPhrase.you (a (Primitives.Predicate.and [instantOrSorcery, spell])) (some yourLibrary))
            (Primitives.Instruction.sequence
              [ Primitives.Instruction.copy .fromStack it (.lit 1) [] (agent := Primitives.NounPhrase.you),
                offer (Primitives.Instruction.chooseNewTargets (that .copy)) (agent := Primitives.NounPhrase.you) ]) ],
      power := stat 2, toughness := stat 4 } }

/-- Pyromancer's Goggles -/
def pyromancersGogglesMana : Ability :=
  activated Primitives.Cost.tapSymbol
    (Primitives.Instruction.addMana (.lit 1) (Primitives.ProducedMana.runs [[.of .red]])
      [ Primitives.ManaRider.onSpent .triggersThen false
          (a (Primitives.Predicate.and [Primitives.Predicate.colorIs .red, instantOrSorcery, spell]))
          (Primitives.Instruction.sequence
            [ Primitives.Instruction.copy .fromStack (that .spell) (.lit 1) [] (agent := Primitives.NounPhrase.you),
              offer (Primitives.Instruction.chooseNewTargets (that .copy)) (agent := Primitives.NounPhrase.you) ]) ] (agent := Primitives.NounPhrase.you))
theorem okPyromancersGogglesMana : Ability.check [] pyromancersGogglesMana = [] := by decide

end Semantics.Cards
