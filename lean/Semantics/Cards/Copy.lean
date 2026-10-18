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
theorem okDisplayOfPowerCopyLock : StaticSpec.check (DRS.ofList []) displayOfPowerCopyLock = [] := by decide

/-- Repeated Reverberation -/
def repeatedReverberation : Spelled := spelled <| .singleFaced
  { characteristics :=
    { name := "Repeated Reverberation", cost := some [generic 2, pip .red, pip .red],
      types := [.instant],
      text :=
        [ Primitives.Ability.spell none (Primitives.Action.delay
            (Primitives.GameEvent.does (Primitives.Subject.actor Primitives.NounPhrase.you) (Primitives.Action.procedure .cast ((a (Primitives.Predicate.and
              [instant, spell]))) Primitives.PlayPayment.itsOwnCost none none none))
            [ Primitives.GameEvent.does (Primitives.Subject.actor Primitives.NounPhrase.you) (Primitives.Action.procedure .cast ((a
              (Primitives.Predicate.and [sorcery, spell]))) Primitives.PlayPayment.itsOwnCost none none none),
              Primitives.GameEvent.does (Primitives.Subject.actor Primitives.NounPhrase.you) (Primitives.Action.procedure .activate (a (Primitives.Predicate.abilityHead .loyalty)) Primitives.PlayPayment.itsOwnCost none none none) ]
            (some thisTurn)
            (Primitives.Action.sequentially
              [ Primitives.Action.copy .fromStack (that .stack) (.lit 2) [],
                offer (Primitives.Action.chooseNewTargets (those .copy)) ])) ] } }

/-- Frontline Heroism -/
def frontlineHeroismCopy : Ability :=
  whenever
    (Primitives.GameEvent.does (Primitives.Subject.actor Primitives.NounPhrase.you) (Primitives.Action.procedure .cast ((a (Primitives.Predicate.and [ spell,
                 Primitives.Predicate.targets (a (Primitives.Predicate.and [creature,
                   Primitives.Predicate.hasPossessor .controller Primitives.NounPhrase.you]))
                   .soleTarget ]))) Primitives.PlayPayment.itsOwnCost none none none))
    (Primitives.Action.sequentially
      [ create (.lit 1)
          { characteristics :=
            { colors := [.red], types := [.creature], subtypes := [creatureType "Soldier"],
              text := [keyword (.named "haste")], power := stat 1, toughness := stat 1 } },
        Primitives.Action.copy .fromStack (that .spell) (.lit 1) [],
        Primitives.Action.copyTargets (that .copy) (that .token) ])
theorem okFrontlineHeroismCopy : Ability.check (DRS.ofList []) frontlineHeroismCopy = [] := by decide

/-- Flawless Forgery -/
def flawlessForgeryLine : Action :=
  Primitives.Action.sequentially
    [ exile (target (Primitives.Predicate.and [instantOrSorcery, Primitives.Predicate.inZone (graveyardOf (a Primitives.Predicate.opponent))])),
      Primitives.Action.copy .fromCardZone (that .card) (.lit 1) [],
      Primitives.Action.establish
        (mayPlayDeed (.action "cast") Primitives.NounPhrase.you (that .copy) none
          (Primitives.DeonticRider.play none none none false Primitives.PlayPayment.withoutPaying))
        none ]
theorem okFlawlessForgeryLine : Action.check (DRS.ofList []) flawlessForgeryLine = [] := by decide

/-- Twincast -/
def twincast : Spelled := spelled <| .singleFaced
  { characteristics :=
    { name := "Twincast", cost := some [pip .blue, pip .blue], types := [.instant],
      text :=
        [ Primitives.Ability.spell none (Primitives.Action.sequentially
            [ Primitives.Action.copy .fromStack (target (Primitives.Predicate.and [instantOrSorcery, spell])) (.lit 1) [],
              offer (Primitives.Action.chooseNewTargets (that .copy)) ]) ] } }

/-- Fork -/
def fork : Spelled := spelled <| .singleFaced
  { characteristics :=
    { name := "Fork", cost := some [pip .red, pip .red], types := [.instant],
      text :=
        [ Primitives.Ability.spell none (Primitives.Action.sequentially
            [ Primitives.Action.copy .fromStack (target (Primitives.Predicate.and [instantOrSorcery, spell])) (.lit 1)
                [Primitives.CopyExcept.color .red],
              offer (Primitives.Action.chooseNewTargets (that .copy)) ]) ] } }

/-- Meletis Charlatan -/
def meletisCharlatan : Spelled := spelled <| .singleFaced
  { characteristics :=
    { name := "Meletis Charlatan", cost := some [generic 2, pip .blue], types := [.creature],
      subtypes := [creatureType "Human", creatureType "Wizard"],
      text :=
        [ activated (Primitives.Cost.compound [Primitives.Cost.mana [generic 2, pip .blue], Primitives.Cost.tapSymbol])
            (Primitives.Action.sequentially
              [ instruct (controllerOf (target (Primitives.Predicate.and [instantOrSorcery, spell]))) (Primitives.Action.copy .fromStack it (.lit 1) []),
                instruct (that .player) (offer (Primitives.Action.chooseNewTargets (that .copy))) ]) ],
      power := stat 2, toughness := stat 3 } }

/-- Echo Mage, level 4+ -/
def echoMagesFourthLevel : Ability :=
  activated (Primitives.Cost.compound [Primitives.Cost.mana [pip .blue, pip .blue], Primitives.Cost.tapSymbol])
    (Primitives.Action.sequentially
      [ Primitives.Action.copy .fromStack (target (Primitives.Predicate.and [instantOrSorcery, spell])) (.lit 2) [],
        offer (Primitives.Action.chooseNewTargets (those .copy)) ])
theorem okEchoMagesFourthLevel : Ability.check (DRS.ofList []) echoMagesFourthLevel = [] := by decide

/-- Strionic Resonator -/
def strionicResonator : Spelled := spelled <| .singleFaced
  { characteristics :=
    { name := "Strionic Resonator", cost := some [generic 2], types := [.artifact],
      text :=
        [ activated (Primitives.Cost.compound [Primitives.Cost.mana [generic 2], Primitives.Cost.tapSymbol])
            (Primitives.Action.sequentially
              [ Primitives.Action.copy .fromStack
                  (target (Primitives.Predicate.and [Primitives.Predicate.abilityHead .anyTriggered, Primitives.Predicate.hasPossessor .controller Primitives.NounPhrase.you]))
                  (.lit 1) [],
                offer (Primitives.Action.chooseNewTargets (that (.copied .ability))) ]) ] } }

/-- Mister Fantastic -/
def misterFantasticCopy : Ability :=
  activated (Primitives.Cost.compound [Primitives.Cost.mana [pip .red, pip .green, pip .white, pip .blue], Primitives.Cost.tapSymbol])
    (Primitives.Action.sequentially
      [ Primitives.Action.copy .fromStack
          (target (Primitives.Predicate.and [Primitives.Predicate.abilityHead .anyTriggered, Primitives.Predicate.hasPossessor .controller Primitives.NounPhrase.you]))
          (.lit 2) [],
        offer (Primitives.Action.chooseNewTargets (those (.copied .ability))) ])
theorem okMisterFantasticCopy : Ability.check (DRS.ofList []) misterFantasticCopy = [] := by decide

/-- Rowan's Talent -/
def rowansTalentCopy : Ability :=
  whenever
    (Primitives.GameEvent.does (Primitives.Subject.actor Primitives.NounPhrase.you) (Primitives.Action.procedure .activate (a (Primitives.Predicate.and [ Primitives.Predicate.abilityHead .loyalty,
                 Primitives.Predicate.abilityOf (Primitives.NounPhrase.attachHost .enchanted (.type .planeswalker)) ])) Primitives.PlayPayment.itsOwnCost none none none))
    (Primitives.Action.sequentially
      [ Primitives.Action.copy .fromStack (that .ability) (.lit 1) [],
        offer (Primitives.Action.chooseNewTargets (that (.copied .ability))) ])
theorem okRowansTalentCopy : Ability.check (DRS.ofList []) rowansTalentCopy = [] := by decide

/-- Rings of Brighthearth -/
def ringsOfBrighthearth : Spelled := spelled <| .singleFaced
  { characteristics :=
    { name := "Rings of Brighthearth", cost := some [generic 3], types := [.artifact],
      text :=
        [ triggeredIf (Primitives.GameEvent.does (Primitives.Subject.actor Primitives.NounPhrase.you) (Primitives.Action.procedure .activate (a (Primitives.Predicate.abilityHead .anyActivated)) Primitives.PlayPayment.itsOwnCost none none none))
            (Primitives.Condition.not (Primitives.Condition.matches (that .ability) Primitives.Predicate.isManaAbility))
            (Primitives.Action.offer (Primitives.Action.pay (Primitives.Cost.mana [generic 2]) .once .paid none none)
              (some (Primitives.Action.sequentially
                [ Primitives.Action.copy .fromStack (that .ability) (.lit 1) [],
                  offer (Primitives.Action.chooseNewTargets (that (.copied .ability))) ]))
              none) ] } }

/-- Iron Man, Bleeding Edge -/
def ironManBleedingEdge : Spelled := spelled <| .singleFaced
  { characteristics :=
    { name := "Iron Man, Bleeding Edge", cost := some [generic 3, pip .blue, pip .blue],
      supertypes := [.legendary], types := [.artifact, .creature],
      subtypes := [creatureType "Human", creatureType "Hero"],
      text :=
        [ keyword (.named "flying"),
          triggered (Primitives.GameEvent.does (Primitives.Subject.actor Primitives.NounPhrase.you) (Primitives.Action.procedure .cast ((a
            (Primitives.Predicate.and [artifact, spell]))) Primitives.PlayPayment.itsOwnCost none none none))
            (doOnlyOnceEachTurn (offer (Primitives.Action.copy .fromStack it (.lit 1) [Primitives.CopyExcept.nonlegendary])))
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
            (Primitives.GameEvent.does (Primitives.Subject.actor Primitives.NounPhrase.you) (Primitives.Action.procedure .cast ((a (Primitives.Predicate.and [ creature, spell, Primitives.Predicate.not
                (Primitives.Predicate.hasSupertype .legendary),
                         Primitives.Predicate.hasKeyword (.the (.named "flying")) ]))) Primitives.PlayPayment.itsOwnCost none none none))
            (doOnlyOnceEachTurn (offer
              (Primitives.Action.copy .fromStack it (.lit 1)
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
            (Primitives.GameEvent.does (Primitives.Subject.actor Primitives.NounPhrase.you) (Primitives.Action.procedure .cast ((a (Primitives.Predicate.and [ Primitives.Predicate.or [
                Primitives.Predicate.hasSubtype (creatureType "Beast"),
                               Primitives.Predicate.hasSubtype (creatureType "Bird") ],
                         creature, spell ]))) Primitives.PlayPayment.itsOwnCost none none none))
            (offer (Primitives.Action.copy .fromStack it (.lit 1) [Primitives.CopyExcept.types [.artifact] []])) ],
      power := stat 3, toughness := stat 5 } }

/-- Bonus Round -/
def bonusRound : Spelled := spelled <| .singleFaced
  { characteristics :=
    { name := "Bonus Round", cost := some [generic 1, pip .red, pip .red], types := [.sorcery],
      text :=
        [ Primitives.Ability.spell none (Primitives.Action.delay
            (Primitives.GameEvent.does (Primitives.Subject.actor (a Primitives.Predicate.anyPlayer)) (Primitives.Action.procedure .cast ((a
              (Primitives.Predicate.and [instantOrSorcery, spell]))) Primitives.PlayPayment.itsOwnCost none none none))
            [] (some untilEndOfTurn)
            (Primitives.Action.sequentially
              [ instruct (that .player) (Primitives.Action.copy .fromStack it (.lit 1) []),
                instruct (that .player) (offer (Primitives.Action.chooseNewTargets (that .copy))) ])) ] } }

/-- Melek, Izzet Paragon -/
def melekIzzetParagon : Spelled := spelled <| .singleFaced
  { characteristics :=
    { name := "Melek, Izzet Paragon", cost := some [generic 4, pip .blue, pip .red],
      supertypes := [.legendary], types := [.creature],
      subtypes := [creatureType "Weird", creatureType "Wizard"],
      text :=
        [ Primitives.Ability.static (Primitives.StaticSpec.visibility .reveal Primitives.NounPhrase.you Primitives.VisibleThing.topOfLibrary),
          Primitives.Ability.static
            (mayPlayDeed (.action "cast") Primitives.NounPhrase.you (allOf (Primitives.Predicate.and [spell, instantOrSorcery])) none
              (Primitives.DeonticRider.play (some onTop) none none false Primitives.PlayPayment.itsOwnCost)),
          whenever
            (Primitives.GameEvent.does (Primitives.Subject.actor Primitives.NounPhrase.you) (Primitives.Action.procedure .cast ((a (Primitives.Predicate.and
              [instantOrSorcery, spell]))) Primitives.PlayPayment.itsOwnCost none (some (Primitives.EventSource.zones [yourLibrary])) none))
            (Primitives.Action.sequentially
              [ Primitives.Action.copy .fromStack it (.lit 1) [],
                offer (Primitives.Action.chooseNewTargets (that .copy)) ]) ],
      power := stat 2, toughness := stat 4 } }

/-- Pyromancer's Goggles -/
def pyromancersGogglesMana : Ability :=
  activated Primitives.Cost.tapSymbol
    (Primitives.Action.addMana (.lit 1) (Primitives.ProducedMana.runs [[.of .red]])
      [ Primitives.ManaRider.onSpent .triggersThen false
          (a (Primitives.Predicate.and [Primitives.Predicate.colorIs .red, instantOrSorcery, spell]))
          (Primitives.Action.sequentially
            [ Primitives.Action.copy .fromStack (that .spell) (.lit 1) [],
              offer (Primitives.Action.chooseNewTargets (that .copy)) ]) ])
theorem okPyromancersGogglesMana : Ability.check (DRS.ofList []) pyromancersGogglesMana = [] := by decide

end Semantics.Cards
