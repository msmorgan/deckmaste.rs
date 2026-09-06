import Semantics
import Semantics.Macros
import Semantics.Check.Card
import Semantics.Cards.Anaphora
import Semantics.Cards.Keyword

/-!
# Semantics.Cards.Mana

Port of `idris/src/Experimental/Cards/Mana.idr`: the printed cards of the Mana family and the
bench items beside them.
-/

open Semantics Semantics.Macros

namespace Semantics.Cards

def boshIronGolem : Ability :=
  activated (Primitives.Cost.compound [Primitives.Cost.mana [generic 3, pip .red], Primitives.Cost.perform (sacrifice (a artifact) (agent :=
      Primitives.NounPhrase.you))])
    (Primitives.Instruction.dealDamage Primitives.NounPhrase.this
      (Primitives.Amount.statOf (.stat .manaValue) (theVerbed (.action "Sacrifice") (.type .artifact) .attributive .one))
      (target anyTarget))
theorem okBoshIronGolem : Ability.check [] boshIronGolem = [] := by decide
def pyromancy : Ability :=
  activated (Primitives.Cost.compound [Primitives.Cost.mana [generic 3], Primitives.Cost.perform (discard (aAtRandom (Primitives.Predicate.inZone hand)) (agent :=
      Primitives.NounPhrase.you))])
    (Primitives.Instruction.dealDamage thisEnchantment
      (Primitives.Amount.statOf (.stat .manaValue) (theVerbed (.action "Discard") .card .attributive .one))
      (target anyTarget))
theorem okPyromancy : Ability.check [] pyromancy = [] := by decide
def luckyOffering : Instruction :=
  Primitives.Instruction.sequence
    [ destroy (target (Primitives.Predicate.and [artifact, Primitives.Predicate.compare [.stat .manaValue] .atMost (.lit 3)])),
      gainLife (.lit 3) (agent := Primitives.NounPhrase.you) ]
theorem okLuckyOffering : Instruction.check [] luckyOffering = [] := by decide
def overload : Instruction :=
  Primitives.Instruction.doOnlyIf (destroy (target artifact)) (Primitives.Condition.compareAmt (Primitives.Amount.statOf (.stat .manaValue) it) .atMost (.lit
      2))
    none
theorem okOverload : Instruction.check [] overload = [] := by decide
def austereCommand : Instruction :=
  chooseModes (exactly 2)
    [ destroy (allOf artifact),
      destroy (allOf enchantment),
      destroy (allOf (Primitives.Predicate.and [creature, Primitives.Predicate.compare [.stat .manaValue] .atMost (.lit 3)])),
      destroy (allOf (Primitives.Predicate.and [creature, Primitives.Predicate.compare [.stat .manaValue] .atLeast (.lit 4)])) ]
theorem okAustereCommand : Instruction.check [] austereCommand = [] := by decide

def bondersEnclave : Spelled := spelled <| .singleFaced
  { characteristics :=
    { name := "Bonders' Enclave", types := [.land],
      text :=
        [ activated Primitives.Cost.tapSymbol (Primitives.Instruction.addMana (.lit 1) (Primitives.ProducedMana.runs [[.colorless]]) [] (agent := Primitives.NounPhrase.you)),
          activatedOnlyIf (Primitives.Cost.compound [Primitives.Cost.mana [generic 3], Primitives.Cost.tapSymbol]) (Primitives.Instruction.draw (.lit 1) (agent :=
              Primitives.NounPhrase.you))
            (exists_ (Primitives.Predicate.and [creature, Primitives.Predicate.hasPossessor .controller Primitives.NounPhrase.you,
                            Primitives.Predicate.compare [.stat .power] .atLeast (.lit 4)])) ] } }

def workhorse : Spelled := spelled <| .singleFaced
  { characteristics :=
    { name := "Workhorse", cost := some [generic 6], types := [.artifact, .creature],
      subtypes := [creatureType "Horse"],
      text :=
        [ Primitives.Ability.static (entersWithCounters thisCreature (.lit 4) plusOnePlusOne),
          activated
            (Primitives.Cost.perform (Primitives.Instruction.removeCounters (some (exactly 1)) (some (Primitives.CounterKindSource.printed plusOnePlusOne)) thisCreature))
            (Primitives.Instruction.addMana (.lit 1) (Primitives.ProducedMana.runs [[.colorless]]) [] (agent := Primitives.NounPhrase.you)) ],
      power := stat 0, toughness := stat 0 } }

def labyrinthOfSkophos : Spelled := spelled <| .singleFaced
  { characteristics :=
    { name := "Labyrinth of Skophos", types := [.land],
      text :=
        [ activated Primitives.Cost.tapSymbol (Primitives.Instruction.addMana (.lit 1) (Primitives.ProducedMana.runs [[.colorless]]) [] (agent := Primitives.NounPhrase.you)),
          activated (Primitives.Cost.compound [Primitives.Cost.mana [generic 4], Primitives.Cost.tapSymbol])
            (Primitives.Instruction.removeFromCombat (target (Primitives.Predicate.and [creature, Primitives.Predicate.or [attacking, blocking]]))) ] } }

def acceleratedMutation : Spelled := spelled <| .singleFaced
  { characteristics :=
    { name := "Accelerated Mutation", cost := some [generic 3, pip .green, pip .green],
      types := [.instant],
      text :=
        [ Primitives.Ability.spell none (Primitives.Instruction.sequence
            [ get (target creature) (Primitives.Delta.up (Primitives.Amount.letter .x)) (Primitives.Delta.up (Primitives.Amount.letter .x)) (some untilEndOfTurn),
              Primitives.Instruction.define .x (aggregate .max (.stat .manaValue)
                (Primitives.Predicate.and [permanent, Primitives.Predicate.hasPossessor .controller Primitives.NounPhrase.you])) ]) ] } }

def cullingScales : Spelled := spelled <| .singleFaced
  { characteristics :=
    { name := "Culling Scales", cost := some [generic 3], types := [.artifact],
      text :=
        [ at_ (Primitives.GameEvent.beginningOf .the .upkeep (Primitives.HeaderPossessor.byPlayer Primitives.NounPhrase.you))
            (destroy (target (Primitives.Predicate.and [permanent, Primitives.Predicate.not land,
              Primitives.Predicate.superlative .min (.stat .manaValue) (Primitives.Predicate.and [permanent, Primitives.Predicate.not land])]))) ] } }

def deadeyeBrawler : Spelled := spelled <| .singleFaced
  { characteristics :=
    { name := "Deadeye Brawler", cost := some [generic 2, pip .blue, pip .black],
      types := [.creature], subtypes := [creatureType "Human", creatureType "Pirate"],
      text :=
        [ keyword "Deathtouch",
          keyword "Ascend",
          triggeredIf (dealsCombatDamage thisCreature (a Primitives.Predicate.anyPlayer))
            (Primitives.Condition.matches Primitives.NounPhrase.you (Primitives.Predicate.hasDesignation "the city's blessing" none))
            (Primitives.Instruction.draw (.lit 1) (agent := Primitives.NounPhrase.you)) ],
      power := stat 2, toughness := stat 4 } }

def femerefEnchantress : Spelled := spelled <| .singleFaced
  { characteristics :=
    { name := "Femeref Enchantress", cost := some [pip .green, pip .white], types := [.creature],
      subtypes := [creatureType "Human", creatureType "Druid"],
      text :=
        [ whenever (putIntoFrom (a enchantment) graveyard (Primitives.EventSource.zones [battlefield]))
            (Primitives.Instruction.draw (.lit 1) (agent := Primitives.NounPhrase.you)) ],
      power := stat 1, toughness := stat 2 } }

def tocasiasWelcome : Spelled := spelled <| .singleFaced
  { characteristics :=
    { name := "Tocasia's Welcome", cost := some [generic 2, pip .white], types := [.enchantment],
      text :=
        [ triggeredOnlyOnce
            (Primitives.GameEvent.enters (counted (atLeast 1) (Primitives.Predicate.and [creature, Primitives.Predicate.hasPossessor .controller Primitives.NounPhrase.you,
                                                 Primitives.Predicate.compare [.stat .manaValue] .atMost (.lit 3)])) none)
            Primitives.UsageLimit.oncePerTurn (Primitives.Instruction.draw (.lit 1) (agent := Primitives.NounPhrase.you)) ] } }

def duskLegionDuelist : Spelled := spelled <| .singleFaced
  { characteristics :=
    { name := "Dusk Legion Duelist", cost := some [generic 1, pip .white], types := [.creature],
      subtypes := [creatureType "Vampire", creatureType "Soldier"],
      text :=
        [ keyword "Vigilance",
          triggeredOnlyOnce (counterEvent .put plusOnePlusOne .many thisCreature) Primitives.UsageLimit.oncePerTurn
            (Primitives.Instruction.draw (.lit 1) (agent := Primitives.NounPhrase.you)) ],
      power := stat 2, toughness := stat 2 } }

def mishrasFactory : Spelled := spelled <| .singleFaced
  { characteristics :=
    { name := "Mishra's Factory", types := [.land],
      text :=
        [ activated Primitives.Cost.tapSymbol (Primitives.Instruction.addMana (.lit 1) (Primitives.ProducedMana.runs [[.colorless]]) [] (agent := Primitives.NounPhrase.you)),
          activated (Primitives.Cost.mana [generic 1])
            (Primitives.Instruction.establish
              (Primitives.StaticSpec.qualityChange thisLand .sets
                (Primitives.QualityPayload.bundle
                  { characteristics :=
                    { types := [.artifact, .creature], subtypes := [creatureType "AssemblyWorker"],
                      power := stat 2, toughness := stat 2 } }
                  (some .land)))
              (some untilEndOfTurn)),
          activated Primitives.Cost.tapSymbol
            (get (target (Primitives.Predicate.and [creature, Primitives.Predicate.hasSubtype (creatureType "AssemblyWorker")]))
              (Primitives.Delta.up (.lit 1)) (Primitives.Delta.up (.lit 1)) (some untilEndOfTurn)) ] } }

/-- Mutavault -/
def mutavault : Spelled := spelled <| .singleFaced
  { characteristics :=
    { name := "Mutavault", types := [.land],
      text :=
        [ activated Primitives.Cost.tapSymbol (Primitives.Instruction.addMana (.lit 1) (Primitives.ProducedMana.runs [[.colorless]]) [] (agent := Primitives.NounPhrase.you)),
          activated (Primitives.Cost.mana [generic 1])
            (Primitives.Instruction.establish
              (Primitives.StaticSpec.qualityChange thisLand .sets
                (Primitives.QualityPayload.bundle
                  { characteristics := { types := [.creature], power := stat 2, toughness := stat 2 },
                    qualities := [Primitives.TokenQuality.withEveryType .creature] }
                  (some .land)))
              (some untilEndOfTurn)) ] } }

/-- Soulstone Sanctuary -/
def soulstoneSanctuary : Spelled := spelled <| .singleFaced
  { characteristics :=
    { name := "Soulstone Sanctuary", types := [.land],
      text :=
        [ activated Primitives.Cost.tapSymbol (Primitives.Instruction.addMana (.lit 1) (Primitives.ProducedMana.runs [[.colorless]]) [] (agent := Primitives.NounPhrase.you)),
          activated (Primitives.Cost.mana [generic 4])
            (Primitives.Instruction.establish
              (Primitives.StaticSpec.qualityChange thisLand .sets
                (Primitives.QualityPayload.bundle
                  { characteristics :=
                    { types := [.creature], text := [keyword "Vigilance"],
                      power := stat 3, toughness := stat 3 },
                    qualities := [Primitives.TokenQuality.withEveryType .creature] }
                  (some .land)))
              none) ] } }

def ragingRavine : Spelled := spelled <| .singleFaced
  { characteristics :=
    { name := "Raging Ravine", types := [.land],
      text :=
        [ Primitives.Ability.static (entersTapped thisLand),
          activated Primitives.Cost.tapSymbol (Primitives.Instruction.addMana (.lit 1) (Primitives.ProducedMana.runs [[.of .red], [.of .green]]) [] (agent :=
              Primitives.NounPhrase.you)),
          activated (Primitives.Cost.mana [generic 2, pip .red, pip .green])
            (Primitives.Instruction.establish
              (Primitives.StaticSpec.qualityChange thisLand .sets
                (Primitives.QualityPayload.bundle
                  { characteristics :=
                    { colors := [.red, .green], types := [.creature],
                      subtypes := [creatureType "Elemental"],
                      text :=
                        [ whenever (attacks thisCreature)
                            (Primitives.Instruction.putCounters (.lit 1) (Primitives.CounterKindSource.printed plusOnePlusOne) thisCreature) ],
                      power := stat 3, toughness := stat 3 } }
                  (some .land)))
              (some untilEndOfTurn)) ] } }

def saheeliFiligreeMaster : Spelled := spelled <| .singleFaced
  { characteristics :=
    { name := "Saheeli, Filigree Master", cost := some [generic 2, pip .blue, pip .red],
      supertypes := [.legendary], types := [.planeswalker], subtypes := [planeswalkerType "Saheeli"],
      text :=
        [ activated (Primitives.Cost.loyaltySymbol (.up 1))
            (Primitives.Instruction.sequence
              [ scry (.lit 1) (agent := Primitives.NounPhrase.you),
                Primitives.Instruction.offer
                  (Primitives.Instruction.setStatus .tapped (a (Primitives.Predicate.and [artifact, untapped, Primitives.Predicate.hasPossessor .controller Primitives.NounPhrase.you])))
                  (some (Primitives.Instruction.draw (.lit 1) (agent := Primitives.NounPhrase.you))) none (agent := Primitives.NounPhrase.you) ]),
          activated (Primitives.Cost.loyaltySymbol (.down 2))
            (Primitives.Instruction.sequence
              [ create (.lit 2)
                  { characteristics :=
                    { types := [.artifact, .creature], subtypes := [creatureType "Thopter"],
                      text := [keyword "Flying"], power := stat 1, toughness := stat 1 } },
                gainHaste them (some untilEndOfTurn) ]),
          activated (Primitives.Cost.loyaltySymbol (.down 4))
            (Primitives.Instruction.getEmblem
              [ Primitives.Ability.static (getsPt (allOf (Primitives.Predicate.and [artifact, creature, Primitives.Predicate.hasPossessor .controller Primitives.NounPhrase.you]))
                  (Primitives.Delta.up (.lit 1)) (Primitives.Delta.up (.lit 1))),
                Primitives.Ability.static (Primitives.StaticSpec.costShift (allOf (Primitives.Predicate.and [artifact, spell, castBy Primitives.NounPhrase.you])) (Primitives.CostShift.less (.lit 1)
                    none)) ] (agent := Primitives.NounPhrase.you)) ],
      loyalty := stat 3 } }

def manalith : Spelled := spelled <| .singleFaced
  { characteristics :=
    { name := "Manalith", cost := some [generic 3], types := [.artifact],
      text := [activated Primitives.Cost.tapSymbol (Primitives.Instruction.addMana (.lit 1) (Primitives.ProducedMana.anyColor .sameColor) [] (agent := Primitives.NounPhrase.you))] }
          }

def seethingSong : Spelled := spelled <| .singleFaced
  { characteristics :=
    { name := "Seething Song", cost := some [generic 2, pip .red], types := [.instant],
      text :=
        [ Primitives.Ability.spell none
            (Primitives.Instruction.addMana (.lit 1) (Primitives.ProducedMana.runs [[.of .red, .of .red, .of .red, .of .red, .of .red]]) []
                (agent := Primitives.NounPhrase.you)) ] } }

def ancientZiggurat : Spelled := spelled <| .singleFaced
  { characteristics :=
    { name := "Ancient Ziggurat", types := [.land],
      text :=
        [ activated Primitives.Cost.tapSymbol
            (Primitives.Instruction.addMana (.lit 1) (Primitives.ProducedMana.anyColor .sameColor)
              [Primitives.ManaRider.spendOnly [Primitives.SpendPurpose.toCast (Primitives.Predicate.and [creature, spell])]] (agent := Primitives.NounPhrase.you)) ] } }

def mishrasWorkshop : Spelled := spelled <| .singleFaced
  { characteristics :=
    { name := "Mishra's Workshop", types := [.land],
      text :=
        [ activated Primitives.Cost.tapSymbol
            (Primitives.Instruction.addMana (.lit 1) (Primitives.ProducedMana.runs [[.colorless, .colorless, .colorless]])
              [Primitives.ManaRider.spendOnly [Primitives.SpendPurpose.toCast (Primitives.Predicate.and [artifact, spell])]] (agent := Primitives.NounPhrase.you)) ] } }

def boommobile : Ability :=
  when (Primitives.GameEvent.enters thisArtifact none)
    (Primitives.Instruction.addMana (.lit 4) (Primitives.ProducedMana.anyColor .sameColor) [Primitives.ManaRider.spendOnly [Primitives.SpendPurpose.toActivate none]] (agent := Primitives.NounPhrase.you))
theorem okBoommobile : Ability.check [] boommobile = [] := by decide

/-- Rosheen Meanderer -/
def rosheenMeanderer : Spelled := spelled <| .singleFaced
  { characteristics :=
    { name := "Rosheen Meanderer", cost := some [generic 3, hybridPip .red .green],
      supertypes := [.legendary], types := [.creature],
      subtypes := [creatureType "Giant", creatureType "Shaman"],
      text :=
        [ activated Primitives.Cost.tapSymbol
            (Primitives.Instruction.addMana (.lit 1) (Primitives.ProducedMana.runs [[.colorless, .colorless, .colorless, .colorless]])
              [Primitives.ManaRider.spendOnly [Primitives.SpendPurpose.toPay (.containing .variable)]] (agent := Primitives.NounPhrase.you)) ],
      power := stat 4, toughness := stat 4 } }

/-- Adarkar Unicorn -/
def adarkarUnicorn : Spelled := spelled <| .singleFaced
  { characteristics :=
    { name := "Adarkar Unicorn", cost := some [generic 1, pip .white, pip .white],
      types := [.creature], subtypes := [creatureType "Unicorn"],
      text :=
        [ activated Primitives.Cost.tapSymbol
            (Primitives.Instruction.addMana (.lit 1) (Primitives.ProducedMana.runs [[.of .blue], [.colorless, .of .blue]])
              [Primitives.ManaRider.spendOnly [Primitives.SpendPurpose.toPay (.ofKeyword "CumulativeUpkeep")]] (agent := Primitives.NounPhrase.you)) ],
      power := stat 2, toughness := stat 2 } }

/-- Overgrown Zealot -/
def overgrownZealot : Spelled := spelled <| .singleFaced
  { characteristics :=
    { name := "Overgrown Zealot", cost := some [generic 1, pip .green], types := [.creature],
      subtypes := [creatureType "Elf", creatureType "Druid"],
      text :=
        [ activated Primitives.Cost.tapSymbol (Primitives.Instruction.addMana (.lit 1) (Primitives.ProducedMana.anyColor .sameColor) [] (agent := Primitives.NounPhrase.you)),
          activated Primitives.Cost.tapSymbol
            (Primitives.Instruction.addMana (.lit 2) (Primitives.ProducedMana.anyColor .sameColor)
              [Primitives.ManaRider.spendOnly [Primitives.SpendPurpose.toPay (.ofSpecialAction .turnFaceUp)]] (agent := Primitives.NounPhrase.you)) ],
      power := stat 0, toughness := stat 4 } }

/-- Unblinking Observer -/
def unblinkingObserver : Spelled := spelled <| .singleFaced
  { characteristics :=
    { name := "Unblinking Observer", cost := some [generic 1, pip .blue], types := [.creature],
      subtypes := [creatureType "Homunculus"],
      text :=
        [ activated Primitives.Cost.tapSymbol
            (Primitives.Instruction.addMana (.lit 1) (Primitives.ProducedMana.runs [[.of .blue]])
              [Primitives.ManaRider.spendOnly [Primitives.SpendPurpose.toPay (.ofKeyword "Disturb"), Primitives.SpendPurpose.toCast instantOrSorcery]] (agent :=
                  Primitives.NounPhrase.you)) ],
      power := stat 2, toughness := stat 1 } }

/-- Qarsi Deceiver -/
def qarsiDeceiverMorphSpend : Ability :=
  activated Primitives.Cost.tapSymbol
    (Primitives.Instruction.addMana (.lit 1) (Primitives.ProducedMana.runs [[.colorless]])
      [Primitives.ManaRider.spendOnly [ Primitives.SpendPurpose.toCast (Primitives.Predicate.and [creature, faceDown]), Primitives.SpendPurpose.toPay (.ofSpecialAction .turnFaceUp),
                    Primitives.SpendPurpose.toPay (.ofKeyword "Morph") ]] (agent := Primitives.NounPhrase.you))
theorem okQarsiDeceiverMorphSpend : Ability.check [] qarsiDeceiverMorphSpend = [] := by decide

/-- Mercadian Bazaar -/
def mercadianBazaar : Spelled := spelled <| .singleFaced
  { characteristics :=
    { name := "Mercadian Bazaar", types := [.land],
      text :=
        [ Primitives.Ability.static (entersTapped thisLand),
          activated Primitives.Cost.tapSymbol (Primitives.Instruction.putCounters (.lit 1) (Primitives.CounterKindSource.printed (.named "Storage")) thisLand),
          activated
            (Primitives.Cost.compound [Primitives.Cost.tapSymbol,
              Primitives.Cost.perform (Primitives.Instruction.removeCounters (some anyNumber) (some (Primitives.CounterKindSource.printed (.named "Storage"))) thisLand)])
            (Primitives.Instruction.addMana removedThisWay (Primitives.ProducedMana.runs [[.of .red]]) [] (agent := Primitives.NounPhrase.you)) ] } }

/-- Rootcoil Creeper -/
def rootcoilCreeperGraveyardMana : Ability :=
  activated Primitives.Cost.tapSymbol
    (Primitives.Instruction.addMana (.lit 2) (Primitives.ProducedMana.anyColor .sameColor)
      [Primitives.ManaRider.spendOnly [Primitives.SpendPurpose.toCast (Primitives.Predicate.and [spell, Primitives.Predicate.castFrom (graveyardOf Primitives.NounPhrase.you)])]] (agent := Primitives.NounPhrase.you))
theorem okRootcoilCreeperGraveyardMana : Ability.check [] rootcoilCreeperGraveyardMana = [] := by
  decide

/-- Black Mana Battery -/
def blackManaBattery : Spelled := spelled <| .singleFaced
  { characteristics :=
    { name := "Black Mana Battery", cost := some [generic 4], types := [.artifact],
      text :=
        [ activated (Primitives.Cost.compound [Primitives.Cost.mana [generic 2], Primitives.Cost.tapSymbol])
            (Primitives.Instruction.putCounters (.lit 1) (Primitives.CounterKindSource.printed (.named "Charge")) thisArtifact),
          activated
            (Primitives.Cost.compound [Primitives.Cost.tapSymbol,
              Primitives.Cost.perform (Primitives.Instruction.removeCounters (some anyNumber) (some (Primitives.CounterKindSource.printed (.named "Charge"))) thisArtifact)])
            (Primitives.Instruction.sequence
              [ Primitives.Instruction.addMana (.lit 1) (Primitives.ProducedMana.runs [[.of .black]]) [] (agent := Primitives.NounPhrase.you),
                Primitives.Instruction.addMana removedThisWay (Primitives.ProducedMana.runs [[.of .black]]) [] (agent := Primitives.NounPhrase.you) ]) ] } }

/-- Elemental Resonance -/
def elementalResonance : Spelled := spelled <| .singleFaced
  { characteristics :=
    { name := "Elemental Resonance", cost := some [generic 2, pip .green, pip .green],
      types := [.enchantment], subtypes := [enchantmentType "Aura"],
      text :=
        [ keywordSubject "Enchant" permanent,
          at_ (Primitives.GameEvent.beginningOf .the .firstMain (Primitives.HeaderPossessor.byPlayer Primitives.NounPhrase.you))
            (Primitives.Instruction.addMana (.lit 1) (Primitives.ProducedMana.asPrintedCost (Primitives.NounPhrase.attachHost .enchanted .permanent)) [] (agent :=
                Primitives.NounPhrase.you)) ] } }

/-- Steelswarm Operator -/
def steelswarmOperatorMana : Instruction :=
  Primitives.Instruction.addMana (.lit 1) (Primitives.ProducedMana.runs [[.of .blue, .of .blue]])
    [Primitives.ManaRider.spendOnly [Primitives.SpendPurpose.toActivate (some (Primitives.Predicate.and [source, artifact]))]] (agent := Primitives.NounPhrase.you)
theorem okSteelswarmOperatorMana : Instruction.check [] steelswarmOperatorMana = [] := by decide

def blastOfGenius : Spelled := spelled <| .singleFaced
  { characteristics :=
    { name := "Blast of Genius", cost := some [generic 4, pip .blue, pip .red], types := [.sorcery],
      text :=
        [ Primitives.Ability.spell none (Primitives.Instruction.sequence
            [ choose (target anyTarget),
              Primitives.Instruction.draw (.lit 3) (agent := Primitives.NounPhrase.you),
              discard (a (Primitives.Predicate.inZone hand)) (agent := Primitives.NounPhrase.you),
              Primitives.Instruction.dealDamage Primitives.NounPhrase.this
                (Primitives.Amount.statOf (.stat .manaValue) (theVerbed (.action "Discard") .card .attributive .one))
                thatJoin ]) ] } }

def riddleOfLightning : Spelled := spelled <| .singleFaced
  { characteristics :=
    { name := "Riddle of Lightning", cost := some [generic 3, pip .red, pip .red], types := [.instant],
      text :=
        [ Primitives.Ability.spell none (Primitives.Instruction.sequence
            [ choose (target anyTarget),
              scry (.lit 3) (agent := Primitives.NounPhrase.you),
              revealCards (topSlice (.lit 1)),
              Primitives.Instruction.dealDamage Primitives.NounPhrase.this (Primitives.Amount.statOf (.stat .manaValue) (that .card)) thatJoin ]) ] } }

def unstableFrontier : Spelled := spelled <| .singleFaced
  { characteristics :=
    { name := "Unstable Frontier", types := [.land],
      text :=
        [ activated Primitives.Cost.tapSymbol (Primitives.Instruction.addMana (.lit 1) (Primitives.ProducedMana.runs [[.colorless]]) [] (agent := Primitives.NounPhrase.you)),
          activated Primitives.Cost.tapSymbol
            (Primitives.Instruction.establish
              (Primitives.StaticSpec.qualityChange (target (Primitives.Predicate.and [land, Primitives.Predicate.hasPossessor .controller Primitives.NounPhrase.you])) .sets
                (Primitives.QualityPayload.chosenQuality (Primitives.Predicate.ofYourChoice (.subtype .land) (some Primitives.ChoiceDomain.basicTypesOnly))))
              (some untilEndOfTurn)) ] } }

/-- Sanctum Prelate -/
def sanctumPrelate : Spelled := spelled <| .singleFaced
  { characteristics :=
    { name := "Sanctum Prelate", cost := some [generic 1, pip .white, pip .white],
      types := [.creature], subtypes := [creatureType "Human", creatureType "Cleric"],
      text :=
        [ Primitives.Ability.static (entersChoosing thisCreature .number),
          Primitives.Ability.static (objectCant (.action "Cast")
            (allOf (Primitives.Predicate.and [spell, Primitives.Predicate.not creature, Primitives.Predicate.compare [.stat .manaValue] .eq chosenNumber]))) ],
      power := stat 2, toughness := stat 2 } }

def abruptDecay : Spelled := spelled <| .singleFaced
  { characteristics :=
    { name := "Abrupt Decay", cost := some [pip .black, pip .green], types := [.instant],
      text :=
        [ Primitives.Ability.static (objectCant (.action "Counter") Primitives.NounPhrase.this),
          Primitives.Ability.spell none
            (destroy (target (Primitives.Predicate.and [permanent, Primitives.Predicate.not land, Primitives.Predicate.compare [.stat .manaValue] .atMost (.lit 3)]))) ] } }

/-- Burn, Burn, Tree and Fern -/
def burnBurnTreeAndFern : Spelled := spelled <| .singleFaced
  { characteristics :=
    { name := "Burn, Burn, Tree and Fern", cost := some [generic 3, pip .red],
      types := [.enchantment], subtypes := [enchantmentType "Saga"],
      text :=
        [ when (Primitives.GameEvent.chapterMark [1])
            (Primitives.Instruction.dealDamage Primitives.NounPhrase.this (.lit 6)
              (target (Primitives.Predicate.and [creature, Primitives.Predicate.hasPossessor .controller (a Primitives.Predicate.opponent)]))),
          when (Primitives.GameEvent.chapterMark [2])
            (destroy (target (Primitives.Predicate.and [artifact, Primitives.Predicate.hasPossessor .controller (a Primitives.Predicate.opponent)]))),
          when (Primitives.GameEvent.chapterMark [3, 4]) (Primitives.Instruction.addMana (.lit 1) (Primitives.ProducedMana.runs [[.of .red]]) [] (agent := Primitives.NounPhrase.you)) ] }
              }

def theFlux : Spelled := spelled <| .singleFaced
  { characteristics :=
    { name := "The Flux", cost := some [generic 2, pip .red, pip .red], types := [.enchantment],
      subtypes := [enchantmentType "Saga"],
      text :=
        [ when (Primitives.GameEvent.chapterMark [1])
            (Primitives.Instruction.dealDamage Primitives.NounPhrase.this (.lit 4)
              (target (Primitives.Predicate.and [creature, Primitives.Predicate.hasPossessor .controller (a Primitives.Predicate.opponent)]))),
          when (Primitives.GameEvent.chapterMark [2, 3, 4, 5])
            (Primitives.Instruction.sequence
              [ exile (topSlice (.lit 1)),
                Primitives.Instruction.establish
                  (mayPlayDeed (.action "Play") Primitives.NounPhrase.you (that .card) none
                    (Primitives.DeonticRider.play none none none false Primitives.PlayPayment.itsOwnCost))
                  (some Primitives.Duration.thisTurn) ]),
          when (Primitives.GameEvent.chapterMark [6]) (Primitives.Instruction.addMana (.lit 6) (Primitives.ProducedMana.runs [[.of .red]]) [] (agent := Primitives.NounPhrase.you)) ] } }

def dragonstormGlobe : Spelled := spelled <| .singleFaced
  { characteristics :=
    { name := "Dragonstorm Globe", cost := some [generic 3], types := [.artifact],
      text :=
        [ Primitives.Ability.static (entersWithAdditionalCounters
            (each (Primitives.Predicate.and [Primitives.Predicate.hasSubtype (creatureType "Dragon"), Primitives.Predicate.hasPossessor .controller Primitives.NounPhrase.you]))
            (.lit 1) plusOnePlusOne),
          activated Primitives.Cost.tapSymbol (Primitives.Instruction.addMana (.lit 1) (Primitives.ProducedMana.anyColor .sameColor) [] (agent := Primitives.NounPhrase.you)) ] } }

def sageOfFables : Spelled := spelled <| .singleFaced
  { characteristics :=
    { name := "Sage of Fables", cost := some [generic 2, pip .blue], types := [.creature],
      subtypes := [creatureType "Merfolk", creatureType "Wizard"],
      text :=
        [ Primitives.Ability.static (entersWithAdditionalCounters
            (each (Primitives.Predicate.and [creature, Primitives.Predicate.hasSubtype (creatureType "Wizard"), Primitives.Predicate.hasPossessor .controller Primitives.NounPhrase.you,
                         Primitives.Predicate.otherThan thisCreature]))
            (.lit 1) plusOnePlusOne),
          activated
            (Primitives.Cost.compound [Primitives.Cost.mana [generic 2],
              Primitives.Cost.perform (Primitives.Instruction.removeCounters (some (exactly 1)) (some (Primitives.CounterKindSource.printed plusOnePlusOne))
                (a (Primitives.Predicate.and [creature, Primitives.Predicate.hasPossessor .controller Primitives.NounPhrase.you])))])
            (Primitives.Instruction.draw (.lit 1) (agent := Primitives.NounPhrase.you)) ],
      power := stat 2, toughness := stat 2 } }

/-- Gaddock Teeg -/
def gaddockTeeg : Spelled := spelled <| .singleFaced
  { characteristics :=
    { name := "Gaddock Teeg", cost := some [pip .green, pip .white], supertypes := [.legendary],
      types := [.creature], subtypes := [creatureType "Kithkin", creatureType "Advisor"],
      text :=
        [ Primitives.Ability.static (objectCant (.action "Cast")
            (allOf (Primitives.Predicate.and [spell, Primitives.Predicate.not creature, Primitives.Predicate.compare [.stat .manaValue] .atLeast (.lit 4)]))),
          Primitives.Ability.static (objectCant (.action "Cast")
            (allOf (Primitives.Predicate.and [spell, Primitives.Predicate.not creature, Primitives.Predicate.manaCostHas .variable]))) ],
      power := stat 2, toughness := stat 2 } }

def shiningShoal : Spelled := spelled <| .singleFaced
  { characteristics :=
    { name := "Shining Shoal", cost := some [.variable, pip .white, pip .white], types := [.instant],
      subtypes := [spellType "Arcane"],
      text :=
        [ Primitives.Ability.static (Primitives.StaticSpec.altCost Primitives.NounPhrase.this
            (some (Primitives.Cost.perform (exile
              (a (Primitives.Predicate.and [Primitives.Predicate.colorIs .white, Primitives.Predicate.compare [.stat .manaValue] .eq (Primitives.Amount.letter .x),
                        Primitives.Predicate.inZone (handOf Primitives.NounPhrase.you)])) (agent := some Primitives.NounPhrase.you))))),
          Primitives.Ability.spell none
            (Primitives.Instruction.establish
              (Primitives.StaticSpec.damageRule .any (Primitives.DamageAgent.dealtBy (aYourChoice source))
                (Primitives.DamageScope.toRecipient (youAnd (allOf (Primitives.Predicate.and [creature, Primitives.Predicate.hasPossessor .controller Primitives.NounPhrase.you]))))
                (Primitives.DamageOp.redirect (Primitives.PreventCut.shield (Primitives.Amount.letter .x)) (target anyTarget)) .repeatedly)
              (some Primitives.Duration.thisTurn)) ] } }

def disruptingShoal : Spelled := spelled <| .singleFaced
  { characteristics :=
    { name := "Disrupting Shoal", cost := some [.variable, pip .blue, pip .blue], types := [.instant],
      subtypes := [spellType "Arcane"],
      text :=
        [ Primitives.Ability.static (Primitives.StaticSpec.altCost Primitives.NounPhrase.this
            (some (Primitives.Cost.perform (exile
              (a (Primitives.Predicate.and [Primitives.Predicate.colorIs .blue, Primitives.Predicate.compare [.stat .manaValue] .eq (Primitives.Amount.letter .x),
                        Primitives.Predicate.inZone (handOf Primitives.NounPhrase.you)])) (agent := some Primitives.NounPhrase.you))))),
          Primitives.Ability.spell none
            (Primitives.Instruction.doOnlyIf (Primitives.Instruction.counterSpell (target spell))
              (Primitives.Condition.compareAmt (Primitives.Amount.statOf (.stat .manaValue) it) .eq (Primitives.Amount.letter .x)) none) ] } }

def blazingShoal : Spelled := spelled <| .singleFaced
  { characteristics :=
    { name := "Blazing Shoal", cost := some [.variable, pip .red, pip .red], types := [.instant],
      subtypes := [spellType "Arcane"],
      text :=
        [ Primitives.Ability.static (Primitives.StaticSpec.altCost Primitives.NounPhrase.this
            (some (Primitives.Cost.perform (exile
              (a (Primitives.Predicate.and [Primitives.Predicate.colorIs .red, Primitives.Predicate.compare [.stat .manaValue] .eq (Primitives.Amount.letter .x),
                        Primitives.Predicate.inZone (handOf Primitives.NounPhrase.you)])) (agent := some Primitives.NounPhrase.you))))),
          Primitives.Ability.spell none
            (get (target creature) (Primitives.Delta.up (Primitives.Amount.letter .x)) (Primitives.Delta.up (.lit 0)) (some untilEndOfTurn)) ] } }

def sickeningShoal : Spelled := spelled <| .singleFaced
  { characteristics :=
    { name := "Sickening Shoal", cost := some [.variable, pip .black, pip .black], types := [.instant],
      subtypes := [spellType "Arcane"],
      text :=
        [ Primitives.Ability.static (Primitives.StaticSpec.altCost Primitives.NounPhrase.this
            (some (Primitives.Cost.perform (exile
              (a (Primitives.Predicate.and [Primitives.Predicate.colorIs .black, Primitives.Predicate.compare [.stat .manaValue] .eq (Primitives.Amount.letter .x),
                        Primitives.Predicate.inZone (handOf Primitives.NounPhrase.you)])) (agent := some Primitives.NounPhrase.you))))),
          Primitives.Ability.spell none
            (get (target creature) (Primitives.Delta.down (Primitives.Amount.letter .x)) (Primitives.Delta.down (Primitives.Amount.letter .x)) (some untilEndOfTurn))
                ] } }

def nourishingShoal : Spelled := spelled <| .singleFaced
  { characteristics :=
    { name := "Nourishing Shoal", cost := some [.variable, pip .green, pip .green], types := [.instant],
      subtypes := [spellType "Arcane"],
      text :=
        [ Primitives.Ability.static (Primitives.StaticSpec.altCost Primitives.NounPhrase.this
            (some (Primitives.Cost.perform (exile
              (a (Primitives.Predicate.and [Primitives.Predicate.colorIs .green, Primitives.Predicate.compare [.stat .manaValue] .eq (Primitives.Amount.letter .x),
                        Primitives.Predicate.inZone (handOf Primitives.NounPhrase.you)])) (agent := some Primitives.NounPhrase.you))))),
          Primitives.Ability.spell none (gainLife (Primitives.Amount.letter .x) (agent := Primitives.NounPhrase.you)) ] } }

def spellSnare : Spelled := spelled <| .singleFaced
  { characteristics :=
    { name := "Spell Snare", cost := some [pip .blue], types := [.instant],
      text :=
        [ Primitives.Ability.spell none
            (Primitives.Instruction.counterSpell (target (Primitives.Predicate.and [spell, Primitives.Predicate.compare [.stat .manaValue] .eq (.lit 2)]))) ] } }

def isolate : Spelled := spelled <| .singleFaced
  { characteristics :=
    { name := "Isolate", cost := some [pip .white], types := [.instant],
      text :=
        [ Primitives.Ability.spell none
            (exile (target (Primitives.Predicate.and [permanent, Primitives.Predicate.compare [.stat .manaValue] .eq (.lit 1)]))) ] } }

def disembowel : Spelled := spelled <| .singleFaced
  { characteristics :=
    { name := "Disembowel", cost := some [.variable, pip .black], types := [.instant],
      text :=
        [ Primitives.Ability.spell none
            (destroy (target (Primitives.Predicate.and [creature, Primitives.Predicate.compare [.stat .manaValue] .eq (Primitives.Amount.letter .x)]))) ] } }

def repeal : Spelled := spelled <| .singleFaced
  { characteristics :=
    { name := "Repeal", cost := some [.variable, pip .blue], types := [.instant],
      text :=
        [ Primitives.Ability.spell none (Primitives.Instruction.sequence
            [ move (target (Primitives.Predicate.and [Primitives.Predicate.not land, permanent, Primitives.Predicate.compare [.stat .manaValue] .eq (Primitives.Amount.letter .x)]))
                hand,
              Primitives.Instruction.draw (.lit 1) (agent := Primitives.NounPhrase.you) ]) ] } }

def entrancingMelody : Spelled := spelled <| .singleFaced
  { characteristics :=
    { name := "Entrancing Melody", cost := some [.variable, pip .blue, pip .blue], types := [.sorcery],
      text :=
        [ Primitives.Ability.spell none
            (Primitives.Instruction.establish
              (Primitives.StaticSpec.controlGrant Primitives.NounPhrase.you
                (target (Primitives.Predicate.and [creature, Primitives.Predicate.compare [.stat .manaValue] .eq (Primitives.Amount.letter .x)])))
              none) ] } }

def ratchetBomb : Spelled := spelled <| .singleFaced
  { characteristics :=
    { name := "Ratchet Bomb", cost := some [generic 2], types := [.artifact],
      text :=
        [ activated Primitives.Cost.tapSymbol (Primitives.Instruction.putCounters (.lit 1) (Primitives.CounterKindSource.printed (.named "Charge")) thisArtifact),
          activated (Primitives.Cost.compound [Primitives.Cost.tapSymbol, Primitives.Cost.perform (sacrifice thisArtifact (agent := Primitives.NounPhrase.you))])
            (destroy (each (Primitives.Predicate.and [Primitives.Predicate.not land, permanent,
              Primitives.Predicate.compare [.stat .manaValue] .eq (countersOn (.named "Charge") thisArtifact)]))) ] } }

def solGrail : Spelled := spelled <| .singleFaced
  { characteristics :=
    { name := "Sol Grail", cost := some [generic 3], types := [.artifact],
      text :=
        [ Primitives.Ability.static (entersChoosing thisArtifact .color),
          activated Primitives.Cost.tapSymbol (Primitives.Instruction.addMana (.lit 1) (Primitives.ProducedMana.ofChosenColor none) [] (agent := Primitives.NounPhrase.you)) ] } }

def unchartedHaven : Spelled := spelled <| .singleFaced
  { characteristics :=
    { name := "Uncharted Haven", types := [.land],
      text :=
        [ Primitives.Ability.static (entersTapped thisLand),
          Primitives.Ability.static (entersChoosing thisLand .color),
          activated Primitives.Cost.tapSymbol (Primitives.Instruction.addMana (.lit 1) (Primitives.ProducedMana.ofChosenColor none) [] (agent := Primitives.NounPhrase.you)) ] } }

def mirageMesa : Spelled := spelled <| .singleFaced
  { characteristics :=
    { name := "Mirage Mesa", types := [.land], subtypes := [landType "Desert"],
      text :=
        [ Primitives.Ability.static (entersTapped thisLand),
          Primitives.Ability.static (entersChoosing thisLand .color),
          activated Primitives.Cost.tapSymbol (Primitives.Instruction.addMana (.lit 1) (Primitives.ProducedMana.ofChosenColor none) [] (agent := Primitives.NounPhrase.you)) ] } }

def crossroadsVillage : Spelled := spelled <| .singleFaced
  { characteristics :=
    { name := "Crossroads Village", types := [.land], subtypes := [landType "Town"],
      text :=
        [ Primitives.Ability.static (entersTapped thisLand),
          Primitives.Ability.static (entersChoosing thisLand .color),
          activated Primitives.Cost.tapSymbol (Primitives.Instruction.addMana (.lit 1) (Primitives.ProducedMana.ofChosenColor none) [] (agent := Primitives.NounPhrase.you)) ] } }

/-- A Thriving land: enters tapped choosing a color other than its own, taps for either. -/
def thrivingLand (name : String) (own : Color) : Card := .singleFaced
  { characteristics :=
    { name, types := [.land],
      text :=
        [ Primitives.Ability.static (entersTapped thisLand),
          Primitives.Ability.static (entersChoosingFrom thisLand .color (Primitives.ChoiceDomain.colorOtherThan own)),
          activated Primitives.Cost.tapSymbol (Primitives.Instruction.addMana (.lit 1) (Primitives.ProducedMana.ofChosenColor (some [.of own])) [] (agent :=
              Primitives.NounPhrase.you)) ] } }

def thrivingBluff : Spelled := spelled <| (thrivingLand "Thriving Bluff" .red)
def thrivingGrove : Spelled := spelled <| (thrivingLand "Thriving Grove" .green)
def thrivingHeath : Spelled := spelled <| (thrivingLand "Thriving Heath" .white)
def thrivingIsle : Spelled := spelled <| (thrivingLand "Thriving Isle" .blue)
def thrivingMoor : Spelled := spelled <| (thrivingLand "Thriving Moor" .black)

def secretsOfTheDead : Spelled := spelled <| .singleFaced
  { characteristics :=
    { name := "Secrets of the Dead", cost := some [generic 2, pip .blue], types := [.enchantment],
      text :=
        [ whenever (Primitives.GameEvent.casts Primitives.NounPhrase.you (a (Primitives.Predicate.and [spell, Primitives.Predicate.castFrom (graveyardOf Primitives.NounPhrase.you)])) none)
            (Primitives.Instruction.draw (.lit 1) (agent := Primitives.NounPhrase.you)) ] } }

def coalStoker : Spelled := spelled <| .singleFaced
  { characteristics :=
    { name := "Coal Stoker", cost := some [generic 3, pip .red], types := [.creature],
      subtypes := [creatureType "Elemental"],
      text :=
        [ triggeredIf (Primitives.GameEvent.enters thisCreature none) (Primitives.Condition.matches it (Primitives.Predicate.castFrom (handOf Primitives.NounPhrase.you)))
            (Primitives.Instruction.addMana (.lit 1) (Primitives.ProducedMana.runs [[.of .red, .of .red, .of .red]]) [] (agent := Primitives.NounPhrase.you)) ],
      power := stat 3, toughness := stat 3 } }

def vegaTheWatcher : Spelled := spelled <| .singleFaced
  { characteristics :=
    { name := "Vega, the Watcher", cost := some [generic 1, pip .white, pip .blue],
      supertypes := [.legendary], types := [.creature],
      subtypes := [creatureType "Bird", creatureType "Spirit"],
      text :=
        [ keyword "Flying",
          whenever (Primitives.GameEvent.casts Primitives.NounPhrase.you (a (Primitives.Predicate.and [spell, Primitives.Predicate.not (Primitives.Predicate.castFrom (handOf Primitives.NounPhrase.you))])) none)
            (Primitives.Instruction.draw (.lit 1) (agent := Primitives.NounPhrase.you)) ],
      power := stat 2, toughness := stat 2 } }

def adNauseam : Spelled := spelled <| .singleFaced
  { characteristics :=
    { name := "Ad Nauseam", cost := some [generic 3, pip .black, pip .black], types := [.instant],
      text :=
        [ Primitives.Ability.spell none (Primitives.Instruction.sequence
            [ revealCards (topSlice (.lit 1)),
              move (that .card) hand,
              loseLife (Primitives.Amount.statOf (.stat .manaValue) it) (agent := Primitives.NounPhrase.you),
              offer (Primitives.Instruction.repeat_ Primitives.Repetition.anyNumber) (agent := Primitives.NounPhrase.you) ]) ] } }

def nahiriLoyaltyRead : Predicate :=
  Primitives.Predicate.and [ creature, Primitives.Predicate.inZone (graveyardOf Primitives.NounPhrase.you),
         Primitives.Predicate.compare [.stat .manaValue] .less (Primitives.Amount.statOf (.stat .loyalty) Primitives.NounPhrase.this) ]
theorem okNahiriLoyaltyRead : Predicate.check .object [] nahiriLoyaltyRead = [] := by decide

/-- Caustic Bronco -/
def causticBroncoLoss : Instruction :=
  Primitives.Instruction.sequence
    [ revealCards (topSlice (.lit 1)),
      move (that .card) hand,
      Primitives.Instruction.doOnlyIf (loseLife (Primitives.Amount.statOf (.stat .manaValue) it) (agent := Primitives.NounPhrase.you))
        (Primitives.Condition.not (Primitives.Condition.matches thisCreature (Primitives.Predicate.hasDesignation "saddled" none)))
        (some (loseLife Primitives.Amount.thatMuch (agent := (each Primitives.Predicate.opponent)))) ]
theorem okCausticBroncoLoss : Instruction.check [] causticBroncoLoss = [] := by decide

/-- Dark Fortress -/
def darkFortressMana : Ability :=
  activatedOnlyIf Primitives.Cost.tapSymbol (Primitives.Instruction.addMana (.lit 1) (Primitives.ProducedMana.runs [[.of .black], [.of .red]]) [] (agent :=
      Primitives.NounPhrase.you))
    (Primitives.Condition.or [ happened .entry thisLand .thisTurn,
           exists_ (Primitives.Predicate.and [land, Primitives.Predicate.hasSupertype .basic, Primitives.Predicate.hasPossessor .controller Primitives.NounPhrase.you]) ])
theorem okDarkFortressMana : Ability.check [] darkFortressMana = [] := by decide

def branchloftPathway : Spelled := spelled <| .modalDfc
  { characteristics :=
    { name := "Branchloft Pathway", types := [.land],
      text := [activated Primitives.Cost.tapSymbol (Primitives.Instruction.addMana (.lit 1) (Primitives.ProducedMana.runs [[.of .green]]) [] (agent := Primitives.NounPhrase.you))] }
          }
  { characteristics :=
    { name := "Boulderloft Pathway", types := [.land],
      text := [activated Primitives.Cost.tapSymbol (Primitives.Instruction.addMana (.lit 1) (Primitives.ProducedMana.runs [[.of .white]]) [] (agent := Primitives.NounPhrase.you))] }
          }

/-- Nykthos, Shrine to Nyx -/
def nykthosShrineToNyx : Spelled := spelled <| .singleFaced
  { characteristics :=
    { name := "Nykthos, Shrine to Nyx", supertypes := [.legendary], types := [.land],
      text :=
        [ activated Primitives.Cost.tapSymbol (Primitives.Instruction.addMana (.lit 1) (Primitives.ProducedMana.runs [[.colorless]]) [] (agent := Primitives.NounPhrase.you)),
          activated (Primitives.Cost.compound [Primitives.Cost.mana [generic 2], Primitives.Cost.tapSymbol])
            (Primitives.Instruction.sequence
              [ choose (a (quality .color)),
                Primitives.Instruction.addMana (Primitives.Amount.devotion Primitives.NounPhrase.you thatColor none) (Primitives.ProducedMana.ofChosenColor none) [] (agent := Primitives.NounPhrase.you) ])
                    ] } }

/-- Karametra's Acolyte -/
def karametrasAcolyte : Ability :=
  activated Primitives.Cost.tapSymbol (Primitives.Instruction.addMana (Primitives.Amount.devotion Primitives.NounPhrase.you (Primitives.ColorTerm.lit .green) none) (Primitives.ProducedMana.runs [[.of .green]]) []
      (agent := Primitives.NounPhrase.you))
theorem okKarametrasAcolyte : Ability.check [] karametrasAcolyte = [] := by decide

/-- Investigator's Journal -/
def investigatorsJournal : Spelled := spelled <| .singleFaced
  { characteristics :=
    { name := "Investigator's Journal", cost := some [generic 2], types := [.artifact],
      subtypes := [artifactType "Book", artifactType "Clue"],
      text :=
        [ Primitives.Ability.static (entersWithCounters thisArtifact greatestCreaturesAPlayerControls (.named "Suspect")),
          activated
            (Primitives.Cost.compound [Primitives.Cost.mana [generic 2], Primitives.Cost.tapSymbol,
              Primitives.Cost.perform (Primitives.Instruction.removeCounters (some (exactly 1)) (some (Primitives.CounterKindSource.printed (.named "Suspect"))) thisArtifact)])
            (Primitives.Instruction.draw (.lit 1) (agent := Primitives.NounPhrase.you)),
          activated (Primitives.Cost.compound [Primitives.Cost.mana [generic 2], Primitives.Cost.perform (sacrifice thisArtifact (agent :=
              Primitives.NounPhrase.you))])
            (Primitives.Instruction.draw (.lit 1) (agent := Primitives.NounPhrase.you)) ] } }

/-- Engineered Explosives (its sunburst [CR#702.44a] written out) -/
def engineeredExplosives : Spelled := spelled <| .singleFaced
  { characteristics :=
    { name := "Engineered Explosives", cost := some [.variable], types := [.artifact],
      text :=
        [ Primitives.Ability.static (entersWithCounters thisArtifact (colorsSpentToCast thisArtifact) (.named "Charge")),
          activated (Primitives.Cost.compound [Primitives.Cost.mana [generic 2], Primitives.Cost.perform (sacrifice thisArtifact (agent :=
              Primitives.NounPhrase.you))])
            (destroy (each (Primitives.Predicate.and [permanent, Primitives.Predicate.not land,
              Primitives.Predicate.compare [.stat .manaValue] .eq (countersOn (.named "Charge") thisArtifact)]))) ] } }

/-- Radiant Flames (converge is an ability word [CR#207.2c]) -/
def radiantFlames : Spelled := spelled <| .singleFaced
  { characteristics :=
    { name := "Radiant Flames", cost := some [generic 2, pip .red], types := [.sorcery],
      text := [Primitives.Ability.spell none (Primitives.Instruction.dealDamage Primitives.NounPhrase.this (colorsSpentToCast Primitives.NounPhrase.this) (each creature))] } }

/-- Birthing Pod -/
def birthingPodSearch : Ability :=
  activated
    (Primitives.Cost.compound [Primitives.Cost.mana [generic 1, phyrexianPip .green], Primitives.Cost.tapSymbol, Primitives.Cost.perform (sacrifice (a creature)
        (agent := Primitives.NounPhrase.you))])
    (searchLibraryFor (exactly 1)
      (Primitives.Predicate.and [creature,
             Primitives.Predicate.compare [.stat .manaValue] .eq
               (Primitives.Amount.arith .plus (.lit 1)
                 (Primitives.Amount.statOf (.stat .manaValue)
                   (theVerbed (.action "Sacrifice") (.type .creature) .attributive .one)))]))
theorem okBirthingPodSearch : Ability.check [] birthingPodSearch = [] := by decide

/-- Hibernation's End -/
def hibernationsEndTrigger : Ability :=
  whenever (Primitives.GameEvent.paysCost (some Primitives.NounPhrase.you) .paid thisEnchantment "CumulativeUpkeep")
    (offer
      (Primitives.Instruction.sequence
        [ searchLibraryFor (exactly 1)
            (Primitives.Predicate.and [creature, Primitives.Predicate.compare [.stat .manaValue] .eq (countersOn (.named "Age") thisEnchantment)]),
          putOntoBattlefield (that .card),
          shuffle ]) (agent := Primitives.NounPhrase.you))
theorem okHibernationsEndTrigger : Ability.check [] hibernationsEndTrigger = [] := by decide

/-- Latchkey Faerie -/
def latchkeyFaerie : Spelled := spelled <| .singleFaced
  { characteristics :=
    { name := "Latchkey Faerie", cost := some [generic 3, pip .blue], types := [.creature],
      subtypes := [creatureType "Faerie", creatureType "Rogue"],
      text :=
        [ keyword "Flying",
          keywordCosting "Prowl" (Primitives.Cost.mana [generic 2, pip .blue]),
          triggeredIf (Primitives.GameEvent.enters thisCreature none) (costWasPaid (.byKeyword "Prowl") none thisCreature)
            (Primitives.Instruction.draw (.lit 1) (agent := Primitives.NounPhrase.you)) ],
      power := stat 3, toughness := stat 1 } }

/-- Bloom Tender -/
def bloomTenderMana : Ability :=
  abilityWord "vivid"
    (activated Primitives.Cost.tapSymbol
      (Primitives.Instruction.doForEachKind .color (some (allOf (Primitives.Predicate.and [permanent, Primitives.Predicate.hasPossessor .controller Primitives.NounPhrase.you])))
          .color
        (Primitives.Instruction.addMana (.lit 1) (Primitives.ProducedMana.ofChosenColor none) [] (agent := Primitives.NounPhrase.you))))
theorem okBloomTenderMana : Ability.check [] bloomTenderMana = [] := by decide

/-- Tarnation Vista -/
def tarnationVistaMana : Ability :=
  activated (Primitives.Cost.compound [Primitives.Cost.mana [generic 1], Primitives.Cost.tapSymbol])
    (Primitives.Instruction.doForEachKind .color
      (some (allOf (Primitives.Predicate.and [permanent, monocolored, Primitives.Predicate.hasPossessor .controller Primitives.NounPhrase.you]))) .color
      (Primitives.Instruction.addMana (.lit 1) (Primitives.ProducedMana.ofChosenColor none) [] (agent := Primitives.NounPhrase.you)))
theorem okTarnationVistaMana : Ability.check [] tarnationVistaMana = [] := by decide

/-- Military Intelligence -/
def militaryIntelligence : Spelled := spelled <| .singleFaced
  { characteristics :=
    { name := "Military Intelligence", cost := some [generic 1, pip .blue], types := [.enchantment],
      text :=
        [ whenever (Primitives.GameEvent.attacksWith Primitives.NounPhrase.you none (counted (atLeast 2) creature)) (Primitives.Instruction.draw (.lit 1) (agent :=
            Primitives.NounPhrase.you)) ] } }

/-- Jem Lightfoote, Sky Explorer -/
def jemLightfooteSkyExplorer : Spelled := spelled <| .singleFaced
  { characteristics :=
    { name := "Jem Lightfoote, Sky Explorer", cost := some [generic 2, pip .white, pip .blue],
      supertypes := [.legendary], types := [.creature],
      subtypes := [creatureType "Human", creatureType "Scout"],
      text :=
        [ keyword "Flying",
          keyword "Vigilance",
          triggeredIf (Primitives.GameEvent.beginningOf .the .endStep (Primitives.HeaderPossessor.byPlayer Primitives.NounPhrase.you))
            (Primitives.Condition.not (happenedFrom .spellCast Primitives.NounPhrase.you .thisTurn (a spell) (Primitives.EventSource.zones [handOf Primitives.NounPhrase.you])))
            (Primitives.Instruction.draw (.lit 1) (agent := Primitives.NounPhrase.you)) ],
      power := stat 3, toughness := stat 3 } }

/-- Gnarlback Rhino -/
def gnarlbackRhino : Spelled := spelled <| .singleFaced
  { characteristics :=
    { name := "Gnarlback Rhino", cost := some [generic 2, pip .green, pip .green], types := [.creature],
      subtypes := [creatureType "Rhino"],
      text :=
        [ keyword "Trample",
          whenever (Primitives.GameEvent.casts Primitives.NounPhrase.you (a (Primitives.Predicate.and [spell, Primitives.Predicate.targets thisCreature .someTarget])) none)
            (Primitives.Instruction.draw (.lit 1) (agent := Primitives.NounPhrase.you)) ],
      power := stat 4, toughness := stat 4 } }

/-- Prismari Pianist -/
def prismariPianist : Spelled := spelled <| .singleFaced
  { characteristics :=
    { name := "Prismari Pianist", cost := some [generic 1, pip .red, pip .red], types := [.creature],
      subtypes := [creatureType "Djinn", creatureType "Bard"],
      text :=
        [ whenever (Primitives.GameEvent.casts Primitives.NounPhrase.you (a (Primitives.Predicate.and [instantOrSorcery, spell])) none)
            (Primitives.Instruction.replace
              (Primitives.Instruction.create (.lit 1)
                (Primitives.TokenSpec.written (creatureToken 1 1 [.blue, .red] [creatureType "Elemental"])) [] (agent :=
                    Primitives.NounPhrase.you))
              (Primitives.Instruction.doIf (Primitives.Condition.compareAmt (Primitives.Amount.statOf (.stat .manaValue) (that .spell)) .atLeast (.lit 5))
                (Primitives.Instruction.create (.lit 3) Primitives.TokenSpec.asThose [] (agent := Primitives.NounPhrase.you)) none)) ],
      power := stat 2, toughness := stat 1 } }

/-- Collected Company -/
def collectedCompany : Spelled := spelled <| .singleFaced
  { characteristics :=
    { name := "Collected Company", cost := some [generic 3, pip .green], types := [.instant],
      text :=
        [ Primitives.Ability.spell none (Primitives.Instruction.sequence
            [ lookAt (topSlice (.lit 6)),
              move (fromAmong (upTo 2) (Primitives.Predicate.and [creature, Primitives.Predicate.compare [.stat .manaValue] .atMost (.lit 3)]) them)
                battlefield,
              move (theRest .object) (onBottomIn .anyOrder) ]) ] } }

/-- Soldevi Adnate -/
def soldeviAdnate : Spelled := spelled <| .singleFaced
  { characteristics :=
    { name := "Soldevi Adnate", cost := some [generic 1, pip .black], types := [.creature],
      subtypes := [creatureType "Human", creatureType "Cleric"],
      text :=
        [ activated
            (Primitives.Cost.compound [Primitives.Cost.tapSymbol,
              Primitives.Cost.perform (sacrifice (a (Primitives.Predicate.and [creature, Primitives.Predicate.or [Primitives.Predicate.colorIs .black, artifact]])) (agent :=
                  Primitives.NounPhrase.you))])
            (Primitives.Instruction.addMana (Primitives.Amount.statOf (.stat .manaValue) (itVerbed (.action "Sacrifice")))
              (Primitives.ProducedMana.runs [[.of .black]]) [] (agent := Primitives.NounPhrase.you)) ],
      power := stat 1, toughness := stat 2 } }

/-- Delivery Moogle -/
def deliveryMoogle : Spelled := spelled <| .singleFaced
  { characteristics :=
    { name := "Delivery Moogle", cost := some [generic 3, pip .white], types := [.creature],
      subtypes := [creatureType "Moogle"],
      text :=
        [ keyword "Flying",
          when (Primitives.GameEvent.enters thisCreature none)
            (Primitives.Instruction.sequence
              [ searchLibraryOrGraveyard (Primitives.Predicate.and [artifact, Primitives.Predicate.compare [.stat .manaValue] .atMost (.lit 2)]),
                revealIt,
                move foundCard hand,
                Primitives.Instruction.doIf (happenedAt (.verbedAct (.action "Search")) Primitives.NounPhrase.you .thisWay yourLibrary) shuffle
                    none ]) ],
      power := stat 3, toughness := stat 2 } }

/-- Heart of Yavimaya -/
def heartOfYavimaya : Spelled := spelled <| .singleFaced
  { characteristics :=
    { name := "Heart of Yavimaya", types := [.land],
      text :=
        [ Primitives.Ability.static (Primitives.StaticSpec.replacement (Primitives.GameEvent.enters Primitives.NounPhrase.this none) [] none
            (Primitives.Instruction.doIfDone (sacrifice (a (Primitives.Predicate.and [land, Primitives.Predicate.hasSubtype (landType "Forest")])) (agent :=
                Primitives.NounPhrase.you))
              (some (putOntoBattlefield Primitives.NounPhrase.this)) (some (move Primitives.NounPhrase.this graveyard)))
            .repeatedly none),
          activated Primitives.Cost.tapSymbol (Primitives.Instruction.addMana (.lit 1) (Primitives.ProducedMana.runs [[.of .green]]) [] (agent := Primitives.NounPhrase.you)),
          activated Primitives.Cost.tapSymbol
            (get (target creature) (Primitives.Delta.up (.lit 1)) (Primitives.Delta.up (.lit 1)) (some untilEndOfTurn)) ] } }

/-- Mox Diamond -/
def moxDiamond : Spelled := spelled <| .singleFaced
  { characteristics :=
    { name := "Mox Diamond", cost := some [], types := [.artifact],
      text :=
        [ Primitives.Ability.static (Primitives.StaticSpec.replacement (Primitives.GameEvent.enters Primitives.NounPhrase.this none) [] none
            (Primitives.Instruction.offer (discard (a (Primitives.Predicate.and [land, Primitives.Predicate.inZone hand])) (agent := Primitives.NounPhrase.you))
              (some (putOntoBattlefield Primitives.NounPhrase.this)) (some (move Primitives.NounPhrase.this graveyard)) (agent := Primitives.NounPhrase.you))
            .repeatedly none),
          activated Primitives.Cost.tapSymbol (Primitives.Instruction.addMana (.lit 1) (Primitives.ProducedMana.anyColor .sameColor) [] (agent := Primitives.NounPhrase.you)) ] } }

/-- Up the Beanstalk -/
def upTheBeanstalk : Spelled := spelled <| .singleFaced
  { characteristics :=
    { name := "Up the Beanstalk", cost := some [generic 1, pip .green], types := [.enchantment],
      text :=
        [ triggeredJoined (Primitives.GameEvent.enters Primitives.NounPhrase.this none)
            [joinedHead (Primitives.GameEvent.casts Primitives.NounPhrase.you (a (Primitives.Predicate.and [spell, Primitives.Predicate.compare [.stat .manaValue] .atLeast (.lit 5)])) none)]
            (Primitives.Instruction.draw (.lit 1) (agent := Primitives.NounPhrase.you)) ] } }

/-- Sheltered Valley -/
def shelteredValley : Spelled := spelled <| .singleFaced
  { characteristics :=
    { name := "Sheltered Valley", types := [.land],
      text :=
        [ Primitives.Ability.static (Primitives.StaticSpec.replacement (Primitives.GameEvent.enters Primitives.NounPhrase.this none) [] none
            (Primitives.Instruction.sequence
              [ sacrifice
                  (each (Primitives.Predicate.and [permanent, Primitives.Predicate.otherThan thisLand, Primitives.Predicate.named (Primitives.NameSource.printed "Sheltered Valley"),
                               Primitives.Predicate.hasPossessor .controller Primitives.NounPhrase.you])) (agent := Primitives.NounPhrase.you),
                putOntoBattlefield Primitives.NounPhrase.this ])
            .repeatedly none),
          triggeredIf (Primitives.GameEvent.beginningOf .the .upkeep (Primitives.HeaderPossessor.byPlayer Primitives.NounPhrase.you))
            (Primitives.Condition.compareAmt (countOf (Primitives.Predicate.and [land, Primitives.Predicate.hasPossessor .controller Primitives.NounPhrase.you])) .atMost (.lit 3))
            (gainLife (.lit 1) (agent := Primitives.NounPhrase.you)),
          activated Primitives.Cost.tapSymbol (Primitives.Instruction.addMana (.lit 1) (Primitives.ProducedMana.runs [[.colorless]]) [] (agent := Primitives.NounPhrase.you)) ] } }

/-- Soul Shatter -/
def soulShatter : Instruction :=
  sacrifice
    (a (Primitives.Predicate.and [Primitives.Predicate.or [creature, Primitives.Predicate.hasType .planeswalker],
              Primitives.Predicate.superlative .max (.stat .manaValue)
                (Primitives.Predicate.and [Primitives.Predicate.or [creature, Primitives.Predicate.hasType .planeswalker], Primitives.Predicate.hasPossessor .controller they])]))
                    (agent := (each Primitives.Predicate.opponent))
theorem okSoulShatter : Instruction.check [] soulShatter = [] := by decide

/-- Padeem, Consul of Innovation -/
def padeemConsulOfInnovation : Ability :=
  triggeredIf (Primitives.GameEvent.beginningOf .the .upkeep (Primitives.HeaderPossessor.byPlayer Primitives.NounPhrase.you))
    (Primitives.Condition.matches
      (the (Primitives.Predicate.and [artifact,
                  Primitives.Predicate.superlative .max (.stat .manaValue) (Primitives.Predicate.and [artifact, Primitives.Predicate.inZone battlefield])]))
      (Primitives.Predicate.hasPossessor .controller Primitives.NounPhrase.you))
    (Primitives.Instruction.draw (.lit 1) (agent := Primitives.NounPhrase.you))
theorem okPadeemConsulOfInnovation : Ability.check [] padeemConsulOfInnovation = [] := by decide

/-- Talion, the Kindly Lord -/
def talionTheKindlyLord : Spelled := spelled <| .singleFaced
  { characteristics :=
    { name := "Talion, the Kindly Lord", cost := some [generic 2, pip .blue, pip .black],
      supertypes := [.legendary], types := [.creature],
      subtypes := [creatureType "Faerie", creatureType "Noble"],
      text :=
        [ keyword "Flying",
          Primitives.Ability.static (entersChoosing thisCreature .number),
          whenever
            (Primitives.GameEvent.casts (a Primitives.Predicate.opponent)
              (a (Primitives.Predicate.and [spell,
                        Primitives.Predicate.compare [.stat .manaValue, .stat .power, .stat .toughness] .eq chosenNumber]))
              none)
            (Primitives.Instruction.sequence [loseLife (.lit 2) (agent := (that .player)), Primitives.Instruction.draw (.lit 1) (agent :=
                Primitives.NounPhrase.you)]) ],
      power := stat 3, toughness := stat 4 } }

/-- Braid of Fire -/
def braidOfFire : Spelled := spelled <| .singleFaced
  { characteristics :=
    { name := "Braid of Fire", cost := some [generic 1, pip .red], types := [.enchantment],
      text := [cumulativeUpkeep (Primitives.Cost.perform (Primitives.Instruction.addMana (.lit 1) (Primitives.ProducedMana.runs [[.of .red]]) [] (agent :=
          Primitives.NounPhrase.you)))] } }

/-- Delighted Halfling -/
def delightedHalflingMana : Ability :=
  activated Primitives.Cost.tapSymbol
    (Primitives.Instruction.addMana (.lit 1) (Primitives.ProducedMana.anyColor .sameColor)
      [ Primitives.ManaRider.onSpent .affectsIt true (a (Primitives.Predicate.and [Primitives.Predicate.hasSupertype .legendary, spell]))
          (Primitives.Instruction.establish (objectCant (.action "Counter") (that .spell)) none) ] (agent := Primitives.NounPhrase.you))
theorem okDelightedHalflingMana : Ability.check [] delightedHalflingMana = [] := by decide

/-- Boseiju, Who Shelters All -/
def boseijuMana : Ability :=
  activated (Primitives.Cost.compound [Primitives.Cost.tapSymbol, payLife Primitives.NounPhrase.you 2])
    (Primitives.Instruction.addMana (.lit 1) (Primitives.ProducedMana.runs [[.colorless]])
      [ Primitives.ManaRider.onSpent .affectsIt false (a (Primitives.Predicate.and [instantOrSorcery, spell]))
          (Primitives.Instruction.establish (objectCant (.action "Counter") (that .spell)) none) ] (agent := Primitives.NounPhrase.you))
theorem okBoseijuMana : Ability.check [] boseijuMana = [] := by decide

/-- Generator Servant -/
def generatorServant : Spelled := spelled <| .singleFaced
  { characteristics :=
    { name := "Generator Servant", cost := some [generic 1, pip .red], types := [.creature],
      subtypes := [creatureType "Elemental"],
      text :=
        [ activated (Primitives.Cost.compound [Primitives.Cost.tapSymbol, Primitives.Cost.perform (sacrifice thisCreature (agent := Primitives.NounPhrase.you))])
            (Primitives.Instruction.addMana (.lit 1) (Primitives.ProducedMana.runs [[.colorless, .colorless]])
              [ Primitives.ManaRider.onSpent .affectsIt false (a (Primitives.Predicate.and [creature, spell]))
                  (gainHaste (Primitives.NounPhrase.resolvedPermanent (that .spell)) (some untilEndOfTurn)) ] (agent :=
                      Primitives.NounPhrase.you)) ],
      power := stat 2, toughness := stat 1 } }

/-- Animal Attendant -/
def animalAttendant : Spelled := spelled <| .singleFaced
  { characteristics :=
    { name := "Animal Attendant", cost := some [generic 1, pip .green], types := [.creature],
      subtypes := [creatureType "Human", creatureType "Citizen"],
      text :=
        [ activated Primitives.Cost.tapSymbol
            (Primitives.Instruction.addMana (.lit 1) (Primitives.ProducedMana.anyColor .sameColor)
              [ Primitives.ManaRider.onSpent .affectsIt false
                  (a (Primitives.Predicate.and [Primitives.Predicate.not (Primitives.Predicate.hasSubtype (creatureType "Human")), creature, spell]))
                  (Primitives.Instruction.establish
                    (Primitives.StaticSpec.entryRider (Primitives.NounPhrase.resolvedPermanent (that .spell))
                      (Primitives.TokenRider.withCounters (.lit 1) (Primitives.CounterKindSource.printed plusOnePlusOne) .additional))
                    none) ] (agent := Primitives.NounPhrase.you)) ],
      power := stat 2, toughness := stat 2 } }

/-- Thran Turbine -/
def thranTurbineMana : Instruction :=
  Primitives.Instruction.addMana (.lit 1) (Primitives.ProducedMana.runs [[.colorless, .colorless]]) [Primitives.ManaRider.spendNotOn [Primitives.SpendPurpose.toCast spell]] (agent := Primitives.NounPhrase.you)
theorem okThranTurbineMana : Instruction.check [] thranTurbineMana = [] := by decide

/-- Su-Chi Cave Guard -/
def suChiCaveGuardDies : Ability :=
  when (Primitives.GameEvent.dies thisCreature)
    (Primitives.Instruction.sequence
      [ Primitives.Instruction.addMana (.lit 1)
          (Primitives.ProducedMana.runs [[.colorless, .colorless, .colorless, .colorless,
                   .colorless, .colorless, .colorless, .colorless]]) [] (agent := Primitives.NounPhrase.you),
        Primitives.Instruction.establish (Primitives.StaticSpec.manaRetention Primitives.NounPhrase.you Primitives.ManaHeld.thisMana) (some untilEndOfTurn) ])
theorem okSuChiCaveGuardDies : Ability.check [] suChiCaveGuardDies = [] := by decide

/-- Omnath, Locus of Mana -/
def omnathLocusOfManaPersistence : StaticSpec := Primitives.StaticSpec.manaRetention Primitives.NounPhrase.you (Primitives.ManaHeld.unspent (some (.of .green)))
theorem okOmnathLocusOfManaPersistence :
    StaticSpec.check [] omnathLocusOfManaPersistence = [] := by decide

/-- Upwelling -/
def upwelling : Spelled := spelled <| .singleFaced
  { characteristics :=
    { name := "Upwelling", cost := some [generic 3, pip .green], types := [.enchantment],
      text := [Primitives.Ability.static (Primitives.StaticSpec.manaRetention (each Primitives.Predicate.anyPlayer) (Primitives.ManaHeld.unspent none))] } }

/-- Mana Flare -/
def manaFlare : Spelled := spelled <| .singleFaced
  { characteristics :=
    { name := "Mana Flare", cost := some [generic 2, pip .red], types := [.enchantment],
      text :=
        [ whenever (Primitives.GameEvent.tappedForMana (some (a Primitives.Predicate.anyPlayer)) (a land) none)
            (Primitives.Instruction.addMana (.lit 1) (Primitives.ProducedMana.producedByEvent (that (.type .land))) [] (agent := (that .player)))
                ] } }

/-- Shimmerwilds Growth -/
def shimmerwildsGrowth : Spelled := spelled <| .singleFaced
  { characteristics :=
    { name := "Shimmerwilds Growth", cost := some [generic 1, pip .green], types := [.enchantment],
      subtypes := [enchantmentType "Aura"],
      text :=
        [ keywordSubject "Enchant" land,
          Primitives.Ability.static (entersChoosing thisAura .color),
          Primitives.Ability.static (Primitives.StaticSpec.qualityChange (Primitives.NounPhrase.attachHost .enchanted (.type .land)) .sets
            (Primitives.QualityPayload.chosenQuality (ofChosen .color))),
          whenever (Primitives.GameEvent.tappedForMana none (Primitives.NounPhrase.attachHost .enchanted (.type .land)) none)
            (Primitives.Instruction.addMana (.lit 1) (Primitives.ProducedMana.ofChosenColor none) [] (agent := (controllerOf (that (.type
                .land))))) ] } }

/-- Gauntlet of Power -/
def gauntletOfPower : Spelled := spelled <| .singleFaced
  { characteristics :=
    { name := "Gauntlet of Power", cost := some [generic 5], types := [.artifact],
      text :=
        [ Primitives.Ability.static (entersChoosing thisArtifact .color),
          Primitives.Ability.static (getsPt (allOf (Primitives.Predicate.and [creature, ofChosen .color])) (Primitives.Delta.up (.lit 1)) (Primitives.Delta.up (.lit 1))),
          whenever
            (Primitives.GameEvent.tappedForMana none (a (Primitives.Predicate.and [land, Primitives.Predicate.hasSupertype .basic]))
              (some (Primitives.ManaTypeTerm.ofColor thatColor)))
            (Primitives.Instruction.addMana (.lit 1) (Primitives.ProducedMana.ofChosenColor none) [] (agent := (controllerOf (that (.type
                .land))))) ] } }

/-- Chrome Mox -/
def chromeMoxMana : Ability :=
  activated Primitives.Cost.tapSymbol
    (Primitives.Instruction.addMana (.lit 1) (Primitives.ProducedMana.amongColorsOf (the (Primitives.Predicate.exiledWith thisArtifact))) [] (agent := Primitives.NounPhrase.you))
theorem okChromeMoxMana : Ability.check [] chromeMoxMana = [] := by decide

/-- Fellwar Stone -/
def fellwarStone : Spelled := spelled <| .singleFaced
  { characteristics :=
    { name := "Fellwar Stone", cost := some [generic 2], types := [.artifact],
      text :=
        [ activated Primitives.Cost.tapSymbol
            (Primitives.Instruction.addMana (.lit 1)
              (Primitives.ProducedMana.couldProduce (a (Primitives.Predicate.and [land, Primitives.Predicate.hasPossessor .controller anOpponent]))) [] (agent :=
                  Primitives.NounPhrase.you)) ] } }

/-- Ice Cauldron -/
def iceCauldronNotedMana : Ability :=
  activated
    (Primitives.Cost.compound [Primitives.Cost.tapSymbol,
      Primitives.Cost.perform (Primitives.Instruction.removeCounters (some (exactly 1)) (some (Primitives.CounterKindSource.printed (.named "Charge"))) thisArtifact)])
    (Primitives.Instruction.addMana (.lit 1) (Primitives.ProducedMana.lastNoted thisArtifact)
      [Primitives.ManaRider.spendOnly [Primitives.SpendPurpose.toCast (Primitives.Predicate.exiledWith thisArtifact)]] (agent := Primitives.NounPhrase.you))
theorem okIceCauldronNotedMana : Ability.check [] iceCauldronNotedMana = [] := by decide

/-- Firemind Vessel -/
def firemindVesselMana : Ability :=
  activated Primitives.Cost.tapSymbol (Primitives.Instruction.addMana (.lit 2) (Primitives.ProducedMana.anyColor .distinctColors) [] (agent := Primitives.NounPhrase.you))
theorem okFiremindVesselMana : Ability.check [] firemindVesselMana = [] := by decide

/-- Goblin Clearcutter -/
def goblinClearcutterMana : Ability :=
  activated
    (Primitives.Cost.compound [Primitives.Cost.tapSymbol, Primitives.Cost.perform (sacrifice (a (Primitives.Predicate.hasSubtype (landType "Forest"))) (agent :=
        Primitives.NounPhrase.you))])
    (Primitives.Instruction.addMana (.lit 3) (Primitives.ProducedMana.amongWritten [.red, .green]) [] (agent := Primitives.NounPhrase.you))
theorem okGoblinClearcutterMana : Ability.check [] goblinClearcutterMana = [] := by decide

/-- Tolaria -/
def tolaria : Spelled := spelled <| .singleFaced
  { characteristics :=
    { name := "Tolaria", supertypes := [.legendary], types := [.land],
      text :=
        [ activated Primitives.Cost.tapSymbol (Primitives.Instruction.addMana (.lit 1) (Primitives.ProducedMana.runs [[.of .blue]]) [] (agent := Primitives.NounPhrase.you)),
          activatedOnlyDuring (Primitives.Cost.compound [Primitives.Cost.tapSymbol])
            (Primitives.Instruction.establish
              (Primitives.StaticSpec.abilityLoss (target creature)
                [Primitives.AbilityLost.written (keyword "Banding"), Primitives.AbilityLost.term bandsWithOtherAbilities])
              (some untilEndOfTurn))
            (Primitives.Timing.duringPart .upkeep none) ] } }

/-- Psychic Vortex -/
def psychicVortexUpkeep : Ability := cumulativeUpkeep (Primitives.Cost.perform (Primitives.Instruction.draw (.lit 1) (agent := Primitives.NounPhrase.you)))
theorem okPsychicVortexUpkeep : Ability.check [] psychicVortexUpkeep = [] := by decide

/-- Varchild's War-Riders -/
def varchildsWarRidersUpkeep : Ability :=
  cumulativeUpkeep
    (Primitives.Cost.perform (Primitives.Instruction.create (.lit 1)
      (Primitives.TokenSpec.written (creatureToken 1 1 [.red] [creatureType "Survivor"])) [] (agent := anOpponent)))
theorem okVarchildsWarRidersUpkeep : Ability.check [] varchildsWarRidersUpkeep = [] := by decide

end Semantics.Cards
