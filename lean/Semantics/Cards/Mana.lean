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
  activated (.compound [.mana [generic 3, pip .red], .perform (sacrifice (a artifact) (agent :=
      .you))])
    (.dealDamage .this
      (.statOf (.stat .manaValue) (theVerbed (.action "Sacrifice") (.type .artifact) .attributive .one))
      (target anyTarget))
theorem okBoshIronGolem : Ability.check [] boshIronGolem = [] := by decide
def pyromancy : Ability :=
  activated (.compound [.mana [generic 3], .perform (discard (aAtRandom (.inZone hand)) (agent :=
      .you))])
    (.dealDamage thisEnchantment
      (.statOf (.stat .manaValue) (theVerbed (.action "Discard") .card .attributive .one))
      (target anyTarget))
theorem okPyromancy : Ability.check [] pyromancy = [] := by decide
def luckyOffering : Instruction :=
  .sequence
    [ destroy (target (.and [artifact, .compare [.stat .manaValue] .atMost (.lit 3)])),
      gainLife (.lit 3) (agent := .you) ]
theorem okLuckyOffering : Instruction.check [] luckyOffering = [] := by decide
def overload : Instruction :=
  .doOnlyIf (destroy (target artifact)) (.compareAmt (.statOf (.stat .manaValue) it) .atMost (.lit
      2))
    none
theorem okOverload : Instruction.check [] overload = [] := by decide
def austereCommand : Instruction :=
  chooseModes (exactly 2)
    [ destroy (allOf artifact),
      destroy (allOf enchantment),
      destroy (allOf (.and [creature, .compare [.stat .manaValue] .atMost (.lit 3)])),
      destroy (allOf (.and [creature, .compare [.stat .manaValue] .atLeast (.lit 4)])) ]
theorem okAustereCommand : Instruction.check [] austereCommand = [] := by decide

def bondersEnclave : Spelled := spelled <| .singleFaced
  { characteristics :=
    { name := "Bonders' Enclave", types := [.land],
      text :=
        [ activated .tapSymbol (.addMana (.lit 1) (.runs [[.colorless]]) [] (agent := .you)),
          activatedOnlyIf (.compound [.mana [generic 3], .tapSymbol]) (.draw (.lit 1) (agent :=
              .you))
            (exists_ (.and [creature, .hasPossessor .controller .you,
                            .compare [.stat .power] .atLeast (.lit 4)])) ] } }

def workhorse : Spelled := spelled <| .singleFaced
  { characteristics :=
    { name := "Workhorse", cost := some [generic 6], types := [.artifact, .creature],
      subtypes := [creatureType "Horse"],
      text :=
        [ .static (entersWithCounters thisCreature (.lit 4) plusOnePlusOne),
          activated
            (.perform (.removeCounters (some (exactly 1)) (some (.printed plusOnePlusOne)) thisCreature))
            (.addMana (.lit 1) (.runs [[.colorless]]) [] (agent := .you)) ],
      power := stat 0, toughness := stat 0 } }

def labyrinthOfSkophos : Spelled := spelled <| .singleFaced
  { characteristics :=
    { name := "Labyrinth of Skophos", types := [.land],
      text :=
        [ activated .tapSymbol (.addMana (.lit 1) (.runs [[.colorless]]) [] (agent := .you)),
          activated (.compound [.mana [generic 4], .tapSymbol])
            (.removeFromCombat (target (.and [creature, .or [attacking, blocking]]))) ] } }

def acceleratedMutation : Spelled := spelled <| .singleFaced
  { characteristics :=
    { name := "Accelerated Mutation", cost := some [generic 3, pip .green, pip .green],
      types := [.instant],
      text :=
        [ .spell none (.sequence
            [ get (target creature) (.up (.letter .x)) (.up (.letter .x)) (some untilEndOfTurn),
              .define .x (aggregate .max (.stat .manaValue)
                (.and [permanent, .hasPossessor .controller .you])) ]) ] } }

def cullingScales : Spelled := spelled <| .singleFaced
  { characteristics :=
    { name := "Culling Scales", cost := some [generic 3], types := [.artifact],
      text :=
        [ at_ (.beginningOf .the .upkeep (.byPlayer .you))
            (destroy (target (.and [permanent, .not land,
              .superlative .min (.stat .manaValue) (.and [permanent, .not land])]))) ] } }

def deadeyeBrawler : Spelled := spelled <| .singleFaced
  { characteristics :=
    { name := "Deadeye Brawler", cost := some [generic 2, pip .blue, pip .black],
      types := [.creature], subtypes := [creatureType "Human", creatureType "Pirate"],
      text :=
        [ keyword "Deathtouch",
          keyword "Ascend",
          triggeredIf (dealsCombatDamage thisCreature (a .anyPlayer))
            (.matches .you (.hasDesignation "the city's blessing" none))
            (.draw (.lit 1) (agent := .you)) ],
      power := stat 2, toughness := stat 4 } }

def femerefEnchantress : Spelled := spelled <| .singleFaced
  { characteristics :=
    { name := "Femeref Enchantress", cost := some [pip .green, pip .white], types := [.creature],
      subtypes := [creatureType "Human", creatureType "Druid"],
      text :=
        [ whenever (putIntoFrom (a enchantment) graveyard (.zones [battlefield]))
            (.draw (.lit 1) (agent := .you)) ],
      power := stat 1, toughness := stat 2 } }

def tocasiasWelcome : Spelled := spelled <| .singleFaced
  { characteristics :=
    { name := "Tocasia's Welcome", cost := some [generic 2, pip .white], types := [.enchantment],
      text :=
        [ triggeredOnlyOnce
            (.enters (counted (atLeast 1) (.and [creature, .hasPossessor .controller .you,
                                                 .compare [.stat .manaValue] .atMost (.lit 3)])) none)
            .oncePerTurn (.draw (.lit 1) (agent := .you)) ] } }

def duskLegionDuelist : Spelled := spelled <| .singleFaced
  { characteristics :=
    { name := "Dusk Legion Duelist", cost := some [generic 1, pip .white], types := [.creature],
      subtypes := [creatureType "Vampire", creatureType "Soldier"],
      text :=
        [ keyword "Vigilance",
          triggeredOnlyOnce (counterEvent .put plusOnePlusOne .many thisCreature) .oncePerTurn
            (.draw (.lit 1) (agent := .you)) ],
      power := stat 2, toughness := stat 2 } }

def mishrasFactory : Spelled := spelled <| .singleFaced
  { characteristics :=
    { name := "Mishra's Factory", types := [.land],
      text :=
        [ activated .tapSymbol (.addMana (.lit 1) (.runs [[.colorless]]) [] (agent := .you)),
          activated (.mana [generic 1])
            (.establish
              (.qualityChange thisLand .sets
                (.bundle
                  { characteristics :=
                    { types := [.artifact, .creature], subtypes := [creatureType "AssemblyWorker"],
                      power := stat 2, toughness := stat 2 } }
                  (some .land)))
              (some untilEndOfTurn)),
          activated .tapSymbol
            (get (target (.and [creature, .hasSubtype (creatureType "AssemblyWorker")]))
              (.up (.lit 1)) (.up (.lit 1)) (some untilEndOfTurn)) ] } }

/-- Mutavault -/
def mutavault : Spelled := spelled <| .singleFaced
  { characteristics :=
    { name := "Mutavault", types := [.land],
      text :=
        [ activated .tapSymbol (.addMana (.lit 1) (.runs [[.colorless]]) [] (agent := .you)),
          activated (.mana [generic 1])
            (.establish
              (.qualityChange thisLand .sets
                (.bundle
                  { characteristics := { types := [.creature], power := stat 2, toughness := stat 2 },
                    qualities := [.withEveryType .creature] }
                  (some .land)))
              (some untilEndOfTurn)) ] } }

/-- Soulstone Sanctuary -/
def soulstoneSanctuary : Spelled := spelled <| .singleFaced
  { characteristics :=
    { name := "Soulstone Sanctuary", types := [.land],
      text :=
        [ activated .tapSymbol (.addMana (.lit 1) (.runs [[.colorless]]) [] (agent := .you)),
          activated (.mana [generic 4])
            (.establish
              (.qualityChange thisLand .sets
                (.bundle
                  { characteristics :=
                    { types := [.creature], text := [keyword "Vigilance"],
                      power := stat 3, toughness := stat 3 },
                    qualities := [.withEveryType .creature] }
                  (some .land)))
              none) ] } }

def ragingRavine : Spelled := spelled <| .singleFaced
  { characteristics :=
    { name := "Raging Ravine", types := [.land],
      text :=
        [ .static (entersTapped thisLand),
          activated .tapSymbol (.addMana (.lit 1) (.runs [[.of .red], [.of .green]]) [] (agent :=
              .you)),
          activated (.mana [generic 2, pip .red, pip .green])
            (.establish
              (.qualityChange thisLand .sets
                (.bundle
                  { characteristics :=
                    { colors := [.red, .green], types := [.creature],
                      subtypes := [creatureType "Elemental"],
                      text :=
                        [ whenever (attacks thisCreature)
                            (.putCounters (.lit 1) (.printed plusOnePlusOne) thisCreature) ],
                      power := stat 3, toughness := stat 3 } }
                  (some .land)))
              (some untilEndOfTurn)) ] } }

def saheeliFiligreeMaster : Spelled := spelled <| .singleFaced
  { characteristics :=
    { name := "Saheeli, Filigree Master", cost := some [generic 2, pip .blue, pip .red],
      supertypes := [.legendary], types := [.planeswalker], subtypes := [planeswalkerType "Saheeli"],
      text :=
        [ activated (.loyaltySymbol (.up 1))
            (.sequence
              [ scry (.lit 1) (agent := .you),
                .offer
                  (.setStatus .tapped (a (.and [artifact, untapped, .hasPossessor .controller .you])))
                  (some (.draw (.lit 1) (agent := .you))) none (agent := .you) ]),
          activated (.loyaltySymbol (.down 2))
            (.sequence
              [ create (.lit 2)
                  { characteristics :=
                    { types := [.artifact, .creature], subtypes := [creatureType "Thopter"],
                      text := [keyword "Flying"], power := stat 1, toughness := stat 1 } },
                gainHaste them (some untilEndOfTurn) ]),
          activated (.loyaltySymbol (.down 4))
            (.getEmblem
              [ .static (getsPt (allOf (.and [artifact, creature, .hasPossessor .controller .you]))
                  (.up (.lit 1)) (.up (.lit 1))),
                .static (.costShift (allOf (.and [artifact, spell, castBy .you])) (.less (.lit 1)
                    none)) ] (agent := .you)) ],
      loyalty := stat 3 } }

def manalith : Spelled := spelled <| .singleFaced
  { characteristics :=
    { name := "Manalith", cost := some [generic 3], types := [.artifact],
      text := [activated .tapSymbol (.addMana (.lit 1) (.anyColor .sameColor) [] (agent := .you))] }
          }

def seethingSong : Spelled := spelled <| .singleFaced
  { characteristics :=
    { name := "Seething Song", cost := some [generic 2, pip .red], types := [.instant],
      text :=
        [ .spell none
            (.addMana (.lit 1) (.runs [[.of .red, .of .red, .of .red, .of .red, .of .red]]) []
                (agent := .you)) ] } }

def ancientZiggurat : Spelled := spelled <| .singleFaced
  { characteristics :=
    { name := "Ancient Ziggurat", types := [.land],
      text :=
        [ activated .tapSymbol
            (.addMana (.lit 1) (.anyColor .sameColor)
              [.spendOnly [.toCast (.and [creature, spell])]] (agent := .you)) ] } }

def mishrasWorkshop : Spelled := spelled <| .singleFaced
  { characteristics :=
    { name := "Mishra's Workshop", types := [.land],
      text :=
        [ activated .tapSymbol
            (.addMana (.lit 1) (.runs [[.colorless, .colorless, .colorless]])
              [.spendOnly [.toCast (.and [artifact, spell])]] (agent := .you)) ] } }

def boommobile : Ability :=
  when (.enters thisArtifact none)
    (.addMana (.lit 4) (.anyColor .sameColor) [.spendOnly [.toActivate none]] (agent := .you))
theorem okBoommobile : Ability.check [] boommobile = [] := by decide

/-- Rosheen Meanderer -/
def rosheenMeanderer : Spelled := spelled <| .singleFaced
  { characteristics :=
    { name := "Rosheen Meanderer", cost := some [generic 3, hybridPip .red .green],
      supertypes := [.legendary], types := [.creature],
      subtypes := [creatureType "Giant", creatureType "Shaman"],
      text :=
        [ activated .tapSymbol
            (.addMana (.lit 1) (.runs [[.colorless, .colorless, .colorless, .colorless]])
              [.spendOnly [.toPay (.containing .variable)]] (agent := .you)) ],
      power := stat 4, toughness := stat 4 } }

/-- Adarkar Unicorn -/
def adarkarUnicorn : Spelled := spelled <| .singleFaced
  { characteristics :=
    { name := "Adarkar Unicorn", cost := some [generic 1, pip .white, pip .white],
      types := [.creature], subtypes := [creatureType "Unicorn"],
      text :=
        [ activated .tapSymbol
            (.addMana (.lit 1) (.runs [[.of .blue], [.colorless, .of .blue]])
              [.spendOnly [.toPay (.ofKeyword "CumulativeUpkeep")]] (agent := .you)) ],
      power := stat 2, toughness := stat 2 } }

/-- Overgrown Zealot -/
def overgrownZealot : Spelled := spelled <| .singleFaced
  { characteristics :=
    { name := "Overgrown Zealot", cost := some [generic 1, pip .green], types := [.creature],
      subtypes := [creatureType "Elf", creatureType "Druid"],
      text :=
        [ activated .tapSymbol (.addMana (.lit 1) (.anyColor .sameColor) [] (agent := .you)),
          activated .tapSymbol
            (.addMana (.lit 2) (.anyColor .sameColor)
              [.spendOnly [.toPay (.ofSpecialAction .turnFaceUp)]] (agent := .you)) ],
      power := stat 0, toughness := stat 4 } }

/-- Unblinking Observer -/
def unblinkingObserver : Spelled := spelled <| .singleFaced
  { characteristics :=
    { name := "Unblinking Observer", cost := some [generic 1, pip .blue], types := [.creature],
      subtypes := [creatureType "Homunculus"],
      text :=
        [ activated .tapSymbol
            (.addMana (.lit 1) (.runs [[.of .blue]])
              [.spendOnly [.toPay (.ofKeyword "Disturb"), .toCast instantOrSorcery]] (agent :=
                  .you)) ],
      power := stat 2, toughness := stat 1 } }

/-- Qarsi Deceiver -/
def qarsiDeceiverMorphSpend : Ability :=
  activated .tapSymbol
    (.addMana (.lit 1) (.runs [[.colorless]])
      [.spendOnly [ .toCast (.and [creature, faceDown]), .toPay (.ofSpecialAction .turnFaceUp),
                    .toPay (.ofKeyword "Morph") ]] (agent := .you))
theorem okQarsiDeceiverMorphSpend : Ability.check [] qarsiDeceiverMorphSpend = [] := by decide

/-- Mercadian Bazaar -/
def mercadianBazaar : Spelled := spelled <| .singleFaced
  { characteristics :=
    { name := "Mercadian Bazaar", types := [.land],
      text :=
        [ .static (entersTapped thisLand),
          activated .tapSymbol (.putCounters (.lit 1) (.printed (.named "Storage")) thisLand),
          activated
            (.compound [.tapSymbol,
              .perform (.removeCounters (some anyNumber) (some (.printed (.named "Storage"))) thisLand)])
            (.addMana removedThisWay (.runs [[.of .red]]) [] (agent := .you)) ] } }

/-- Rootcoil Creeper -/
def rootcoilCreeperGraveyardMana : Ability :=
  activated .tapSymbol
    (.addMana (.lit 2) (.anyColor .sameColor)
      [.spendOnly [.toCast (.and [spell, .castFrom (graveyardOf .you)])]] (agent := .you))
theorem okRootcoilCreeperGraveyardMana : Ability.check [] rootcoilCreeperGraveyardMana = [] := by
  decide

/-- Black Mana Battery -/
def blackManaBattery : Spelled := spelled <| .singleFaced
  { characteristics :=
    { name := "Black Mana Battery", cost := some [generic 4], types := [.artifact],
      text :=
        [ activated (.compound [.mana [generic 2], .tapSymbol])
            (.putCounters (.lit 1) (.printed (.named "Charge")) thisArtifact),
          activated
            (.compound [.tapSymbol,
              .perform (.removeCounters (some anyNumber) (some (.printed (.named "Charge"))) thisArtifact)])
            (.sequence
              [ .addMana (.lit 1) (.runs [[.of .black]]) [] (agent := .you),
                .addMana removedThisWay (.runs [[.of .black]]) [] (agent := .you) ]) ] } }

/-- Elemental Resonance -/
def elementalResonance : Spelled := spelled <| .singleFaced
  { characteristics :=
    { name := "Elemental Resonance", cost := some [generic 2, pip .green, pip .green],
      types := [.enchantment], subtypes := [enchantmentType "Aura"],
      text :=
        [ keywordSubject "Enchant" permanent,
          at_ (.beginningOf .the .firstMain (.byPlayer .you))
            (.addMana (.lit 1) (.asPrintedCost (.attachHost .enchanted .permanent)) [] (agent :=
                .you)) ] } }

/-- Steelswarm Operator -/
def steelswarmOperatorMana : Instruction :=
  .addMana (.lit 1) (.runs [[.of .blue, .of .blue]])
    [.spendOnly [.toActivate (some (.and [source, artifact]))]] (agent := .you)
theorem okSteelswarmOperatorMana : Instruction.check [] steelswarmOperatorMana = [] := by decide

def blastOfGenius : Spelled := spelled <| .singleFaced
  { characteristics :=
    { name := "Blast of Genius", cost := some [generic 4, pip .blue, pip .red], types := [.sorcery],
      text :=
        [ .spell none (.sequence
            [ choose (target anyTarget),
              .draw (.lit 3) (agent := .you),
              discard (a (.inZone hand)) (agent := .you),
              .dealDamage .this
                (.statOf (.stat .manaValue) (theVerbed (.action "Discard") .card .attributive .one))
                thatJoin ]) ] } }

def riddleOfLightning : Spelled := spelled <| .singleFaced
  { characteristics :=
    { name := "Riddle of Lightning", cost := some [generic 3, pip .red, pip .red], types := [.instant],
      text :=
        [ .spell none (.sequence
            [ choose (target anyTarget),
              scry (.lit 3) (agent := .you),
              revealCards (topSlice (.lit 1)),
              .dealDamage .this (.statOf (.stat .manaValue) (that .card)) thatJoin ]) ] } }

def unstableFrontier : Spelled := spelled <| .singleFaced
  { characteristics :=
    { name := "Unstable Frontier", types := [.land],
      text :=
        [ activated .tapSymbol (.addMana (.lit 1) (.runs [[.colorless]]) [] (agent := .you)),
          activated .tapSymbol
            (.establish
              (.qualityChange (target (.and [land, .hasPossessor .controller .you])) .sets
                (.chosenQuality (.ofYourChoice (.subtype .land) (some .basicTypesOnly))))
              (some untilEndOfTurn)) ] } }

/-- Sanctum Prelate -/
def sanctumPrelate : Spelled := spelled <| .singleFaced
  { characteristics :=
    { name := "Sanctum Prelate", cost := some [generic 1, pip .white, pip .white],
      types := [.creature], subtypes := [creatureType "Human", creatureType "Cleric"],
      text :=
        [ .static (entersChoosing thisCreature .number),
          .static (objectCant (.action "Cast")
            (allOf (.and [spell, .not creature, .compare [.stat .manaValue] .eq chosenNumber]))) ],
      power := stat 2, toughness := stat 2 } }

def abruptDecay : Spelled := spelled <| .singleFaced
  { characteristics :=
    { name := "Abrupt Decay", cost := some [pip .black, pip .green], types := [.instant],
      text :=
        [ .static (objectCant (.action "Counter") .this),
          .spell none
            (destroy (target (.and [permanent, .not land, .compare [.stat .manaValue] .atMost (.lit 3)]))) ] } }

/-- Burn, Burn, Tree and Fern -/
def burnBurnTreeAndFern : Spelled := spelled <| .singleFaced
  { characteristics :=
    { name := "Burn, Burn, Tree and Fern", cost := some [generic 3, pip .red],
      types := [.enchantment], subtypes := [enchantmentType "Saga"],
      text :=
        [ when (.chapterMark [1])
            (.dealDamage .this (.lit 6)
              (target (.and [creature, .hasPossessor .controller (a .opponent)]))),
          when (.chapterMark [2])
            (destroy (target (.and [artifact, .hasPossessor .controller (a .opponent)]))),
          when (.chapterMark [3, 4]) (.addMana (.lit 1) (.runs [[.of .red]]) [] (agent := .you)) ] }
              }

def theFlux : Spelled := spelled <| .singleFaced
  { characteristics :=
    { name := "The Flux", cost := some [generic 2, pip .red, pip .red], types := [.enchantment],
      subtypes := [enchantmentType "Saga"],
      text :=
        [ when (.chapterMark [1])
            (.dealDamage .this (.lit 4)
              (target (.and [creature, .hasPossessor .controller (a .opponent)]))),
          when (.chapterMark [2, 3, 4, 5])
            (.sequence
              [ exile (topSlice (.lit 1)),
                .establish
                  (mayPlayDeed (.action "Play") .you (that .card) none
                    (.play none none none false .itsOwnCost))
                  (some .thisTurn) ]),
          when (.chapterMark [6]) (.addMana (.lit 6) (.runs [[.of .red]]) [] (agent := .you)) ] } }

def dragonstormGlobe : Spelled := spelled <| .singleFaced
  { characteristics :=
    { name := "Dragonstorm Globe", cost := some [generic 3], types := [.artifact],
      text :=
        [ .static (entersWithAdditionalCounters
            (each (.and [.hasSubtype (creatureType "Dragon"), .hasPossessor .controller .you]))
            (.lit 1) plusOnePlusOne),
          activated .tapSymbol (.addMana (.lit 1) (.anyColor .sameColor) [] (agent := .you)) ] } }

def sageOfFables : Spelled := spelled <| .singleFaced
  { characteristics :=
    { name := "Sage of Fables", cost := some [generic 2, pip .blue], types := [.creature],
      subtypes := [creatureType "Merfolk", creatureType "Wizard"],
      text :=
        [ .static (entersWithAdditionalCounters
            (each (.and [creature, .hasSubtype (creatureType "Wizard"), .hasPossessor .controller .you,
                         .otherThan thisCreature]))
            (.lit 1) plusOnePlusOne),
          activated
            (.compound [.mana [generic 2],
              .perform (.removeCounters (some (exactly 1)) (some (.printed plusOnePlusOne))
                (a (.and [creature, .hasPossessor .controller .you])))])
            (.draw (.lit 1) (agent := .you)) ],
      power := stat 2, toughness := stat 2 } }

/-- Gaddock Teeg -/
def gaddockTeeg : Spelled := spelled <| .singleFaced
  { characteristics :=
    { name := "Gaddock Teeg", cost := some [pip .green, pip .white], supertypes := [.legendary],
      types := [.creature], subtypes := [creatureType "Kithkin", creatureType "Advisor"],
      text :=
        [ .static (objectCant (.action "Cast")
            (allOf (.and [spell, .not creature, .compare [.stat .manaValue] .atLeast (.lit 4)]))),
          .static (objectCant (.action "Cast")
            (allOf (.and [spell, .not creature, .manaCostHas .variable]))) ],
      power := stat 2, toughness := stat 2 } }

def shiningShoal : Spelled := spelled <| .singleFaced
  { characteristics :=
    { name := "Shining Shoal", cost := some [.variable, pip .white, pip .white], types := [.instant],
      subtypes := [spellType "Arcane"],
      text :=
        [ .static (.altCost .this
            (some (.perform (exile
              (a (.and [.colorIs .white, .compare [.stat .manaValue] .eq (.letter .x),
                        .inZone (handOf .you)])) (agent := some .you))))),
          .spell none
            (.establish
              (.damageRule .any (.dealtBy (aYourChoice source))
                (.toRecipient (youAnd (allOf (.and [creature, .hasPossessor .controller .you]))))
                (.redirect (.shield (.letter .x)) (target anyTarget)) .repeatedly)
              (some .thisTurn)) ] } }

def disruptingShoal : Spelled := spelled <| .singleFaced
  { characteristics :=
    { name := "Disrupting Shoal", cost := some [.variable, pip .blue, pip .blue], types := [.instant],
      subtypes := [spellType "Arcane"],
      text :=
        [ .static (.altCost .this
            (some (.perform (exile
              (a (.and [.colorIs .blue, .compare [.stat .manaValue] .eq (.letter .x),
                        .inZone (handOf .you)])) (agent := some .you))))),
          .spell none
            (.doOnlyIf (.counterSpell (target spell))
              (.compareAmt (.statOf (.stat .manaValue) it) .eq (.letter .x)) none) ] } }

def blazingShoal : Spelled := spelled <| .singleFaced
  { characteristics :=
    { name := "Blazing Shoal", cost := some [.variable, pip .red, pip .red], types := [.instant],
      subtypes := [spellType "Arcane"],
      text :=
        [ .static (.altCost .this
            (some (.perform (exile
              (a (.and [.colorIs .red, .compare [.stat .manaValue] .eq (.letter .x),
                        .inZone (handOf .you)])) (agent := some .you))))),
          .spell none
            (get (target creature) (.up (.letter .x)) (.up (.lit 0)) (some untilEndOfTurn)) ] } }

def sickeningShoal : Spelled := spelled <| .singleFaced
  { characteristics :=
    { name := "Sickening Shoal", cost := some [.variable, pip .black, pip .black], types := [.instant],
      subtypes := [spellType "Arcane"],
      text :=
        [ .static (.altCost .this
            (some (.perform (exile
              (a (.and [.colorIs .black, .compare [.stat .manaValue] .eq (.letter .x),
                        .inZone (handOf .you)])) (agent := some .you))))),
          .spell none
            (get (target creature) (.down (.letter .x)) (.down (.letter .x)) (some untilEndOfTurn))
                ] } }

def nourishingShoal : Spelled := spelled <| .singleFaced
  { characteristics :=
    { name := "Nourishing Shoal", cost := some [.variable, pip .green, pip .green], types := [.instant],
      subtypes := [spellType "Arcane"],
      text :=
        [ .static (.altCost .this
            (some (.perform (exile
              (a (.and [.colorIs .green, .compare [.stat .manaValue] .eq (.letter .x),
                        .inZone (handOf .you)])) (agent := some .you))))),
          .spell none (gainLife (.letter .x) (agent := .you)) ] } }

def spellSnare : Spelled := spelled <| .singleFaced
  { characteristics :=
    { name := "Spell Snare", cost := some [pip .blue], types := [.instant],
      text :=
        [ .spell none
            (.counterSpell (target (.and [spell, .compare [.stat .manaValue] .eq (.lit 2)]))) ] } }

def isolate : Spelled := spelled <| .singleFaced
  { characteristics :=
    { name := "Isolate", cost := some [pip .white], types := [.instant],
      text :=
        [ .spell none
            (exile (target (.and [permanent, .compare [.stat .manaValue] .eq (.lit 1)]))) ] } }

def disembowel : Spelled := spelled <| .singleFaced
  { characteristics :=
    { name := "Disembowel", cost := some [.variable, pip .black], types := [.instant],
      text :=
        [ .spell none
            (destroy (target (.and [creature, .compare [.stat .manaValue] .eq (.letter .x)]))) ] } }

def repeal : Spelled := spelled <| .singleFaced
  { characteristics :=
    { name := "Repeal", cost := some [.variable, pip .blue], types := [.instant],
      text :=
        [ .spell none (.sequence
            [ move (target (.and [.not land, permanent, .compare [.stat .manaValue] .eq (.letter .x)]))
                hand,
              .draw (.lit 1) (agent := .you) ]) ] } }

def entrancingMelody : Spelled := spelled <| .singleFaced
  { characteristics :=
    { name := "Entrancing Melody", cost := some [.variable, pip .blue, pip .blue], types := [.sorcery],
      text :=
        [ .spell none
            (.establish
              (.controlGrant .you
                (target (.and [creature, .compare [.stat .manaValue] .eq (.letter .x)])))
              none) ] } }

def ratchetBomb : Spelled := spelled <| .singleFaced
  { characteristics :=
    { name := "Ratchet Bomb", cost := some [generic 2], types := [.artifact],
      text :=
        [ activated .tapSymbol (.putCounters (.lit 1) (.printed (.named "Charge")) thisArtifact),
          activated (.compound [.tapSymbol, .perform (sacrifice thisArtifact (agent := .you))])
            (destroy (each (.and [.not land, permanent,
              .compare [.stat .manaValue] .eq (countersOn (.named "Charge") thisArtifact)]))) ] } }

def solGrail : Spelled := spelled <| .singleFaced
  { characteristics :=
    { name := "Sol Grail", cost := some [generic 3], types := [.artifact],
      text :=
        [ .static (entersChoosing thisArtifact .color),
          activated .tapSymbol (.addMana (.lit 1) (.ofChosenColor none) [] (agent := .you)) ] } }

def unchartedHaven : Spelled := spelled <| .singleFaced
  { characteristics :=
    { name := "Uncharted Haven", types := [.land],
      text :=
        [ .static (entersTapped thisLand),
          .static (entersChoosing thisLand .color),
          activated .tapSymbol (.addMana (.lit 1) (.ofChosenColor none) [] (agent := .you)) ] } }

def mirageMesa : Spelled := spelled <| .singleFaced
  { characteristics :=
    { name := "Mirage Mesa", types := [.land], subtypes := [landType "Desert"],
      text :=
        [ .static (entersTapped thisLand),
          .static (entersChoosing thisLand .color),
          activated .tapSymbol (.addMana (.lit 1) (.ofChosenColor none) [] (agent := .you)) ] } }

def crossroadsVillage : Spelled := spelled <| .singleFaced
  { characteristics :=
    { name := "Crossroads Village", types := [.land], subtypes := [landType "Town"],
      text :=
        [ .static (entersTapped thisLand),
          .static (entersChoosing thisLand .color),
          activated .tapSymbol (.addMana (.lit 1) (.ofChosenColor none) [] (agent := .you)) ] } }

/-- A Thriving land: enters tapped choosing a color other than its own, taps for either. -/
def thrivingLand (name : String) (own : Color) : Card := .singleFaced
  { characteristics :=
    { name, types := [.land],
      text :=
        [ .static (entersTapped thisLand),
          .static (entersChoosingFrom thisLand .color (.colorOtherThan own)),
          activated .tapSymbol (.addMana (.lit 1) (.ofChosenColor (some [.of own])) [] (agent :=
              .you)) ] } }

def thrivingBluff : Spelled := spelled (thrivingLand "Thriving Bluff" .red)
def thrivingGrove : Spelled := spelled (thrivingLand "Thriving Grove" .green)
def thrivingHeath : Spelled := spelled (thrivingLand "Thriving Heath" .white)
def thrivingIsle : Spelled := spelled (thrivingLand "Thriving Isle" .blue)
def thrivingMoor : Spelled := spelled (thrivingLand "Thriving Moor" .black)

def secretsOfTheDead : Spelled := spelled <| .singleFaced
  { characteristics :=
    { name := "Secrets of the Dead", cost := some [generic 2, pip .blue], types := [.enchantment],
      text :=
        [ whenever (.casts .you (a (.and [spell, .castFrom (graveyardOf .you)])) none)
            (.draw (.lit 1) (agent := .you)) ] } }

def coalStoker : Spelled := spelled <| .singleFaced
  { characteristics :=
    { name := "Coal Stoker", cost := some [generic 3, pip .red], types := [.creature],
      subtypes := [creatureType "Elemental"],
      text :=
        [ triggeredIf (.enters thisCreature none) (.matches it (.castFrom (handOf .you)))
            (.addMana (.lit 1) (.runs [[.of .red, .of .red, .of .red]]) [] (agent := .you)) ],
      power := stat 3, toughness := stat 3 } }

def vegaTheWatcher : Spelled := spelled <| .singleFaced
  { characteristics :=
    { name := "Vega, the Watcher", cost := some [generic 1, pip .white, pip .blue],
      supertypes := [.legendary], types := [.creature],
      subtypes := [creatureType "Bird", creatureType "Spirit"],
      text :=
        [ keyword "Flying",
          whenever (.casts .you (a (.and [spell, .not (.castFrom (handOf .you))])) none)
            (.draw (.lit 1) (agent := .you)) ],
      power := stat 2, toughness := stat 2 } }

def adNauseam : Spelled := spelled <| .singleFaced
  { characteristics :=
    { name := "Ad Nauseam", cost := some [generic 3, pip .black, pip .black], types := [.instant],
      text :=
        [ .spell none (.sequence
            [ revealCards (topSlice (.lit 1)),
              move (that .card) hand,
              loseLife (.statOf (.stat .manaValue) it) (agent := .you),
              offer (.repeat_ .anyNumber) (agent := .you) ]) ] } }

def nahiriLoyaltyRead : Predicate :=
  .and [ creature, .inZone (graveyardOf .you),
         .compare [.stat .manaValue] .less (.statOf (.stat .loyalty) .this) ]
theorem okNahiriLoyaltyRead : Predicate.check .object [] nahiriLoyaltyRead = [] := by decide

/-- Caustic Bronco -/
def causticBroncoLoss : Instruction :=
  .sequence
    [ revealCards (topSlice (.lit 1)),
      move (that .card) hand,
      .doOnlyIf (loseLife (.statOf (.stat .manaValue) it) (agent := .you))
        (.not (.matches thisCreature (.hasDesignation "saddled" none)))
        (some (loseLife .thatMuch (agent := (each .opponent)))) ]
theorem okCausticBroncoLoss : Instruction.check [] causticBroncoLoss = [] := by decide

/-- Dark Fortress -/
def darkFortressMana : Ability :=
  activatedOnlyIf .tapSymbol (.addMana (.lit 1) (.runs [[.of .black], [.of .red]]) [] (agent :=
      .you))
    (.or [ happened .entry thisLand .thisTurn,
           exists_ (.and [land, .hasSupertype .basic, .hasPossessor .controller .you]) ])
theorem okDarkFortressMana : Ability.check [] darkFortressMana = [] := by decide

def branchloftPathway : Spelled := spelled <| .modalDfc
  { characteristics :=
    { name := "Branchloft Pathway", types := [.land],
      text := [activated .tapSymbol (.addMana (.lit 1) (.runs [[.of .green]]) [] (agent := .you))] }
          }
  { characteristics :=
    { name := "Boulderloft Pathway", types := [.land],
      text := [activated .tapSymbol (.addMana (.lit 1) (.runs [[.of .white]]) [] (agent := .you))] }
          }

/-- Nykthos, Shrine to Nyx -/
def nykthosShrineToNyx : Spelled := spelled <| .singleFaced
  { characteristics :=
    { name := "Nykthos, Shrine to Nyx", supertypes := [.legendary], types := [.land],
      text :=
        [ activated .tapSymbol (.addMana (.lit 1) (.runs [[.colorless]]) [] (agent := .you)),
          activated (.compound [.mana [generic 2], .tapSymbol])
            (.sequence
              [ choose (a (quality .color)),
                .addMana (.devotion .you thatColor none) (.ofChosenColor none) [] (agent := .you) ])
                    ] } }

/-- Karametra's Acolyte -/
def karametrasAcolyte : Ability :=
  activated .tapSymbol (.addMana (.devotion .you (.lit .green) none) (.runs [[.of .green]]) []
      (agent := .you))
theorem okKarametrasAcolyte : Ability.check [] karametrasAcolyte = [] := by decide

/-- Investigator's Journal -/
def investigatorsJournal : Spelled := spelled <| .singleFaced
  { characteristics :=
    { name := "Investigator's Journal", cost := some [generic 2], types := [.artifact],
      subtypes := [artifactType "Book", artifactType "Clue"],
      text :=
        [ .static (entersWithCounters thisArtifact greatestCreaturesAPlayerControls (.named "Suspect")),
          activated
            (.compound [.mana [generic 2], .tapSymbol,
              .perform (.removeCounters (some (exactly 1)) (some (.printed (.named "Suspect"))) thisArtifact)])
            (.draw (.lit 1) (agent := .you)),
          activated (.compound [.mana [generic 2], .perform (sacrifice thisArtifact (agent :=
              .you))])
            (.draw (.lit 1) (agent := .you)) ] } }

/-- Engineered Explosives (its sunburst [CR#702.44a] written out) -/
def engineeredExplosives : Spelled := spelled <| .singleFaced
  { characteristics :=
    { name := "Engineered Explosives", cost := some [.variable], types := [.artifact],
      text :=
        [ .static (entersWithCounters thisArtifact (colorsSpentToCast thisArtifact) (.named "Charge")),
          activated (.compound [.mana [generic 2], .perform (sacrifice thisArtifact (agent :=
              .you))])
            (destroy (each (.and [permanent, .not land,
              .compare [.stat .manaValue] .eq (countersOn (.named "Charge") thisArtifact)]))) ] } }

/-- Radiant Flames (converge is an ability word [CR#207.2c]) -/
def radiantFlames : Spelled := spelled <| .singleFaced
  { characteristics :=
    { name := "Radiant Flames", cost := some [generic 2, pip .red], types := [.sorcery],
      text := [.spell none (.dealDamage .this (colorsSpentToCast .this) (each creature))] } }

/-- Birthing Pod -/
def birthingPodSearch : Ability :=
  activated
    (.compound [.mana [generic 1, phyrexianPip .green], .tapSymbol, .perform (sacrifice (a creature)
        (agent := .you))])
    (searchLibraryFor (exactly 1)
      (.and [creature,
             .compare [.stat .manaValue] .eq
               (.arith .plus (.lit 1)
                 (.statOf (.stat .manaValue)
                   (theVerbed (.action "Sacrifice") (.type .creature) .attributive .one)))]))
theorem okBirthingPodSearch : Ability.check [] birthingPodSearch = [] := by decide

/-- Hibernation's End -/
def hibernationsEndTrigger : Ability :=
  whenever (.paysCost (some .you) .paid thisEnchantment "CumulativeUpkeep")
    (offer
      (.sequence
        [ searchLibraryFor (exactly 1)
            (.and [creature, .compare [.stat .manaValue] .eq (countersOn (.named "Age") thisEnchantment)]),
          putOntoBattlefield (that .card),
          shuffle ]) (agent := .you))
theorem okHibernationsEndTrigger : Ability.check [] hibernationsEndTrigger = [] := by decide

/-- Latchkey Faerie -/
def latchkeyFaerie : Spelled := spelled <| .singleFaced
  { characteristics :=
    { name := "Latchkey Faerie", cost := some [generic 3, pip .blue], types := [.creature],
      subtypes := [creatureType "Faerie", creatureType "Rogue"],
      text :=
        [ keyword "Flying",
          keywordCosting "Prowl" (.mana [generic 2, pip .blue]),
          triggeredIf (.enters thisCreature none) (costWasPaid (.byKeyword "Prowl") none thisCreature)
            (.draw (.lit 1) (agent := .you)) ],
      power := stat 3, toughness := stat 1 } }

/-- Bloom Tender -/
def bloomTenderMana : Ability :=
  abilityWord "vivid"
    (activated .tapSymbol
      (.doForEachKind .color (some (allOf (.and [permanent, .hasPossessor .controller .you])))
          .color
        (.addMana (.lit 1) (.ofChosenColor none) [] (agent := .you))))
theorem okBloomTenderMana : Ability.check [] bloomTenderMana = [] := by decide

/-- Tarnation Vista -/
def tarnationVistaMana : Ability :=
  activated (.compound [.mana [generic 1], .tapSymbol])
    (.doForEachKind .color
      (some (allOf (.and [permanent, monocolored, .hasPossessor .controller .you]))) .color
      (.addMana (.lit 1) (.ofChosenColor none) [] (agent := .you)))
theorem okTarnationVistaMana : Ability.check [] tarnationVistaMana = [] := by decide

/-- Military Intelligence -/
def militaryIntelligence : Spelled := spelled <| .singleFaced
  { characteristics :=
    { name := "Military Intelligence", cost := some [generic 1, pip .blue], types := [.enchantment],
      text :=
        [ whenever (.attacksWith .you none (counted (atLeast 2) creature)) (.draw (.lit 1) (agent :=
            .you)) ] } }

/-- Jem Lightfoote, Sky Explorer -/
def jemLightfooteSkyExplorer : Spelled := spelled <| .singleFaced
  { characteristics :=
    { name := "Jem Lightfoote, Sky Explorer", cost := some [generic 2, pip .white, pip .blue],
      supertypes := [.legendary], types := [.creature],
      subtypes := [creatureType "Human", creatureType "Scout"],
      text :=
        [ keyword "Flying",
          keyword "Vigilance",
          triggeredIf (.beginningOf .the .endStep (.byPlayer .you))
            (.not (happenedFrom .spellCast .you .thisTurn (a spell) (.zones [handOf .you])))
            (.draw (.lit 1) (agent := .you)) ],
      power := stat 3, toughness := stat 3 } }

/-- Gnarlback Rhino -/
def gnarlbackRhino : Spelled := spelled <| .singleFaced
  { characteristics :=
    { name := "Gnarlback Rhino", cost := some [generic 2, pip .green, pip .green], types := [.creature],
      subtypes := [creatureType "Rhino"],
      text :=
        [ keyword "Trample",
          whenever (.casts .you (a (.and [spell, .targets thisCreature .someTarget])) none)
            (.draw (.lit 1) (agent := .you)) ],
      power := stat 4, toughness := stat 4 } }

/-- Prismari Pianist -/
def prismariPianist : Spelled := spelled <| .singleFaced
  { characteristics :=
    { name := "Prismari Pianist", cost := some [generic 1, pip .red, pip .red], types := [.creature],
      subtypes := [creatureType "Djinn", creatureType "Bard"],
      text :=
        [ whenever (.casts .you (a (.and [instantOrSorcery, spell])) none)
            (.replace
              (.create (.lit 1)
                (.written (creatureToken 1 1 [.blue, .red] [creatureType "Elemental"])) [] (agent :=
                    .you))
              (.doIf (.compareAmt (.statOf (.stat .manaValue) (that .spell)) .atLeast (.lit 5))
                (.create (.lit 3) .asThose [] (agent := .you)) none)) ],
      power := stat 2, toughness := stat 1 } }

/-- Collected Company -/
def collectedCompany : Spelled := spelled <| .singleFaced
  { characteristics :=
    { name := "Collected Company", cost := some [generic 3, pip .green], types := [.instant],
      text :=
        [ .spell none (.sequence
            [ lookAt (topSlice (.lit 6)),
              move (fromAmong (upTo 2) (.and [creature, .compare [.stat .manaValue] .atMost (.lit 3)]) them)
                battlefield,
              move (theRest .object) (onBottomIn .anyOrder) ]) ] } }

/-- Soldevi Adnate -/
def soldeviAdnate : Spelled := spelled <| .singleFaced
  { characteristics :=
    { name := "Soldevi Adnate", cost := some [generic 1, pip .black], types := [.creature],
      subtypes := [creatureType "Human", creatureType "Cleric"],
      text :=
        [ activated
            (.compound [.tapSymbol,
              .perform (sacrifice (a (.and [creature, .or [.colorIs .black, artifact]])) (agent :=
                  .you))])
            (.addMana (.statOf (.stat .manaValue) (itVerbed (.action "Sacrifice")))
              (.runs [[.of .black]]) [] (agent := .you)) ],
      power := stat 1, toughness := stat 2 } }

/-- Delivery Moogle -/
def deliveryMoogle : Spelled := spelled <| .singleFaced
  { characteristics :=
    { name := "Delivery Moogle", cost := some [generic 3, pip .white], types := [.creature],
      subtypes := [creatureType "Moogle"],
      text :=
        [ keyword "Flying",
          when (.enters thisCreature none)
            (.sequence
              [ searchLibraryOrGraveyard (.and [artifact, .compare [.stat .manaValue] .atMost (.lit 2)]),
                revealIt,
                move foundCard hand,
                .doIf (happenedAt (.verbedAct (.action "Search")) .you .thisWay yourLibrary) shuffle
                    none ]) ],
      power := stat 3, toughness := stat 2 } }

/-- Heart of Yavimaya -/
def heartOfYavimaya : Spelled := spelled <| .singleFaced
  { characteristics :=
    { name := "Heart of Yavimaya", types := [.land],
      text :=
        [ .static (.replacement (.enters .this none) [] none
            (.doIfDone (sacrifice (a (.and [land, .hasSubtype (landType "Forest")])) (agent :=
                .you))
              (some (putOntoBattlefield .this)) (some (move .this graveyard)))
            .repeatedly none),
          activated .tapSymbol (.addMana (.lit 1) (.runs [[.of .green]]) [] (agent := .you)),
          activated .tapSymbol
            (get (target creature) (.up (.lit 1)) (.up (.lit 1)) (some untilEndOfTurn)) ] } }

/-- Mox Diamond -/
def moxDiamond : Spelled := spelled <| .singleFaced
  { characteristics :=
    { name := "Mox Diamond", cost := some [], types := [.artifact],
      text :=
        [ .static (.replacement (.enters .this none) [] none
            (.offer (discard (a (.and [land, .inZone hand])) (agent := .you))
              (some (putOntoBattlefield .this)) (some (move .this graveyard)) (agent := .you))
            .repeatedly none),
          activated .tapSymbol (.addMana (.lit 1) (.anyColor .sameColor) [] (agent := .you)) ] } }

/-- Up the Beanstalk -/
def upTheBeanstalk : Spelled := spelled <| .singleFaced
  { characteristics :=
    { name := "Up the Beanstalk", cost := some [generic 1, pip .green], types := [.enchantment],
      text :=
        [ triggeredJoined (.enters .this none)
            [joinedHead (.casts .you (a (.and [spell, .compare [.stat .manaValue] .atLeast (.lit 5)])) none)]
            (.draw (.lit 1) (agent := .you)) ] } }

/-- Sheltered Valley -/
def shelteredValley : Spelled := spelled <| .singleFaced
  { characteristics :=
    { name := "Sheltered Valley", types := [.land],
      text :=
        [ .static (.replacement (.enters .this none) [] none
            (.sequence
              [ sacrifice
                  (each (.and [permanent, .otherThan thisLand, .named (.printed "Sheltered Valley"),
                               .hasPossessor .controller .you])) (agent := .you),
                putOntoBattlefield .this ])
            .repeatedly none),
          triggeredIf (.beginningOf .the .upkeep (.byPlayer .you))
            (.compareAmt (countOf (.and [land, .hasPossessor .controller .you])) .atMost (.lit 3))
            (gainLife (.lit 1) (agent := .you)),
          activated .tapSymbol (.addMana (.lit 1) (.runs [[.colorless]]) [] (agent := .you)) ] } }

/-- Soul Shatter -/
def soulShatter : Instruction :=
  sacrifice
    (a (.and [.or [creature, .hasType .planeswalker],
              .superlative .max (.stat .manaValue)
                (.and [.or [creature, .hasType .planeswalker], .hasPossessor .controller they])]))
                    (agent := (each .opponent))
theorem okSoulShatter : Instruction.check [] soulShatter = [] := by decide

/-- Padeem, Consul of Innovation -/
def padeemConsulOfInnovation : Ability :=
  triggeredIf (.beginningOf .the .upkeep (.byPlayer .you))
    (.matches
      (the (.and [artifact,
                  .superlative .max (.stat .manaValue) (.and [artifact, .inZone battlefield])]))
      (.hasPossessor .controller .you))
    (.draw (.lit 1) (agent := .you))
theorem okPadeemConsulOfInnovation : Ability.check [] padeemConsulOfInnovation = [] := by decide

/-- Talion, the Kindly Lord -/
def talionTheKindlyLord : Spelled := spelled <| .singleFaced
  { characteristics :=
    { name := "Talion, the Kindly Lord", cost := some [generic 2, pip .blue, pip .black],
      supertypes := [.legendary], types := [.creature],
      subtypes := [creatureType "Faerie", creatureType "Noble"],
      text :=
        [ keyword "Flying",
          .static (entersChoosing thisCreature .number),
          whenever
            (.casts (a .opponent)
              (a (.and [spell,
                        .compare [.stat .manaValue, .stat .power, .stat .toughness] .eq chosenNumber]))
              none)
            (.sequence [loseLife (.lit 2) (agent := (that .player)), .draw (.lit 1) (agent :=
                .you)]) ],
      power := stat 3, toughness := stat 4 } }

/-- Braid of Fire -/
def braidOfFire : Spelled := spelled <| .singleFaced
  { characteristics :=
    { name := "Braid of Fire", cost := some [generic 1, pip .red], types := [.enchantment],
      text := [cumulativeUpkeep (.perform (.addMana (.lit 1) (.runs [[.of .red]]) [] (agent :=
          .you)))] } }

/-- Delighted Halfling -/
def delightedHalflingMana : Ability :=
  activated .tapSymbol
    (.addMana (.lit 1) (.anyColor .sameColor)
      [ .onSpent .affectsIt true (a (.and [.hasSupertype .legendary, spell]))
          (.establish (objectCant (.action "Counter") (that .spell)) none) ] (agent := .you))
theorem okDelightedHalflingMana : Ability.check [] delightedHalflingMana = [] := by decide

/-- Boseiju, Who Shelters All -/
def boseijuMana : Ability :=
  activated (.compound [.tapSymbol, payLife .you 2])
    (.addMana (.lit 1) (.runs [[.colorless]])
      [ .onSpent .affectsIt false (a (.and [instantOrSorcery, spell]))
          (.establish (objectCant (.action "Counter") (that .spell)) none) ] (agent := .you))
theorem okBoseijuMana : Ability.check [] boseijuMana = [] := by decide

/-- Generator Servant -/
def generatorServant : Spelled := spelled <| .singleFaced
  { characteristics :=
    { name := "Generator Servant", cost := some [generic 1, pip .red], types := [.creature],
      subtypes := [creatureType "Elemental"],
      text :=
        [ activated (.compound [.tapSymbol, .perform (sacrifice thisCreature (agent := .you))])
            (.addMana (.lit 1) (.runs [[.colorless, .colorless]])
              [ .onSpent .affectsIt false (a (.and [creature, spell]))
                  (gainHaste (.resolvedPermanent (that .spell)) (some untilEndOfTurn)) ] (agent :=
                      .you)) ],
      power := stat 2, toughness := stat 1 } }

/-- Animal Attendant -/
def animalAttendant : Spelled := spelled <| .singleFaced
  { characteristics :=
    { name := "Animal Attendant", cost := some [generic 1, pip .green], types := [.creature],
      subtypes := [creatureType "Human", creatureType "Citizen"],
      text :=
        [ activated .tapSymbol
            (.addMana (.lit 1) (.anyColor .sameColor)
              [ .onSpent .affectsIt false
                  (a (.and [.not (.hasSubtype (creatureType "Human")), creature, spell]))
                  (.establish
                    (.entryRider (.resolvedPermanent (that .spell))
                      (.withCounters (.lit 1) (.printed plusOnePlusOne) .additional))
                    none) ] (agent := .you)) ],
      power := stat 2, toughness := stat 2 } }

/-- Thran Turbine -/
def thranTurbineMana : Instruction :=
  .addMana (.lit 1) (.runs [[.colorless, .colorless]]) [.spendNotOn [.toCast spell]] (agent := .you)
theorem okThranTurbineMana : Instruction.check [] thranTurbineMana = [] := by decide

/-- Su-Chi Cave Guard -/
def suChiCaveGuardDies : Ability :=
  when (.dies thisCreature)
    (.sequence
      [ .addMana (.lit 1)
          (.runs [[.colorless, .colorless, .colorless, .colorless,
                   .colorless, .colorless, .colorless, .colorless]]) [] (agent := .you),
        .establish (.manaRetention .you .thisMana) (some untilEndOfTurn) ])
theorem okSuChiCaveGuardDies : Ability.check [] suChiCaveGuardDies = [] := by decide

/-- Omnath, Locus of Mana -/
def omnathLocusOfManaPersistence : StaticSpec := .manaRetention .you (.unspent (some (.of .green)))
theorem okOmnathLocusOfManaPersistence :
    StaticSpec.check [] omnathLocusOfManaPersistence = [] := by decide

/-- Upwelling -/
def upwelling : Spelled := spelled <| .singleFaced
  { characteristics :=
    { name := "Upwelling", cost := some [generic 3, pip .green], types := [.enchantment],
      text := [.static (.manaRetention (each .anyPlayer) (.unspent none))] } }

/-- Mana Flare -/
def manaFlare : Spelled := spelled <| .singleFaced
  { characteristics :=
    { name := "Mana Flare", cost := some [generic 2, pip .red], types := [.enchantment],
      text :=
        [ whenever (.tappedForMana (some (a .anyPlayer)) (a land) none)
            (.addMana (.lit 1) (.producedByEvent (that (.type .land))) [] (agent := (that .player)))
                ] } }

/-- Shimmerwilds Growth -/
def shimmerwildsGrowth : Spelled := spelled <| .singleFaced
  { characteristics :=
    { name := "Shimmerwilds Growth", cost := some [generic 1, pip .green], types := [.enchantment],
      subtypes := [enchantmentType "Aura"],
      text :=
        [ keywordSubject "Enchant" land,
          .static (entersChoosing thisAura .color),
          .static (.qualityChange (.attachHost .enchanted (.type .land)) .sets
            (.chosenQuality (ofChosen .color))),
          whenever (.tappedForMana none (.attachHost .enchanted (.type .land)) none)
            (.addMana (.lit 1) (.ofChosenColor none) [] (agent := (controllerOf (that (.type
                .land))))) ] } }

/-- Gauntlet of Power -/
def gauntletOfPower : Spelled := spelled <| .singleFaced
  { characteristics :=
    { name := "Gauntlet of Power", cost := some [generic 5], types := [.artifact],
      text :=
        [ .static (entersChoosing thisArtifact .color),
          .static (getsPt (allOf (.and [creature, ofChosen .color])) (.up (.lit 1)) (.up (.lit 1))),
          whenever
            (.tappedForMana none (a (.and [land, .hasSupertype .basic]))
              (some (.ofColor thatColor)))
            (.addMana (.lit 1) (.ofChosenColor none) [] (agent := (controllerOf (that (.type
                .land))))) ] } }

/-- Chrome Mox -/
def chromeMoxMana : Ability :=
  activated .tapSymbol
    (.addMana (.lit 1) (.amongColorsOf (the (.exiledWith thisArtifact))) [] (agent := .you))
theorem okChromeMoxMana : Ability.check [] chromeMoxMana = [] := by decide

/-- Fellwar Stone -/
def fellwarStone : Spelled := spelled <| .singleFaced
  { characteristics :=
    { name := "Fellwar Stone", cost := some [generic 2], types := [.artifact],
      text :=
        [ activated .tapSymbol
            (.addMana (.lit 1)
              (.couldProduce (a (.and [land, .hasPossessor .controller anOpponent]))) [] (agent :=
                  .you)) ] } }

/-- Ice Cauldron -/
def iceCauldronNotedMana : Ability :=
  activated
    (.compound [.tapSymbol,
      .perform (.removeCounters (some (exactly 1)) (some (.printed (.named "Charge"))) thisArtifact)])
    (.addMana (.lit 1) (.lastNoted thisArtifact)
      [.spendOnly [.toCast (.exiledWith thisArtifact)]] (agent := .you))
theorem okIceCauldronNotedMana : Ability.check [] iceCauldronNotedMana = [] := by decide

/-- Firemind Vessel -/
def firemindVesselMana : Ability :=
  activated .tapSymbol (.addMana (.lit 2) (.anyColor .distinctColors) [] (agent := .you))
theorem okFiremindVesselMana : Ability.check [] firemindVesselMana = [] := by decide

/-- Goblin Clearcutter -/
def goblinClearcutterMana : Ability :=
  activated
    (.compound [.tapSymbol, .perform (sacrifice (a (.hasSubtype (landType "Forest"))) (agent :=
        .you))])
    (.addMana (.lit 3) (.amongWritten [.red, .green]) [] (agent := .you))
theorem okGoblinClearcutterMana : Ability.check [] goblinClearcutterMana = [] := by decide

/-- Tolaria -/
def tolaria : Spelled := spelled <| .singleFaced
  { characteristics :=
    { name := "Tolaria", supertypes := [.legendary], types := [.land],
      text :=
        [ activated .tapSymbol (.addMana (.lit 1) (.runs [[.of .blue]]) [] (agent := .you)),
          activatedOnlyDuring (.compound [.tapSymbol])
            (.establish
              (.abilityLoss (target creature)
                [.written (keyword "Banding"), .term bandsWithOtherAbilities])
              (some untilEndOfTurn))
            (.duringPart .upkeep none) ] } }

/-- Psychic Vortex -/
def psychicVortexUpkeep : Ability := cumulativeUpkeep (.perform (.draw (.lit 1) (agent := .you)))
theorem okPsychicVortexUpkeep : Ability.check [] psychicVortexUpkeep = [] := by decide

/-- Varchild's War-Riders -/
def varchildsWarRidersUpkeep : Ability :=
  cumulativeUpkeep
    (.perform (.create (.lit 1)
      (.written (creatureToken 1 1 [.red] [creatureType "Survivor"])) [] (agent := anOpponent)))
theorem okVarchildsWarRidersUpkeep : Ability.check [] varchildsWarRidersUpkeep = [] := by decide

end Semantics.Cards
