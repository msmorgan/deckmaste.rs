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
  activated (.compound [.mana [generic 3, pip .red], .perform (sacrifice .you (a artifact))])
    (.dealDamage .this
      (.statOf (.stat .manaValue) (theVerbed (.action "Sacrifice") (.type .artifact) .attributive .one))
      (target anyTarget))
theorem okBoshIronGolem : Ability.check [] boshIronGolem = [] := by decide
def pyromancy : Ability :=
  activated (.compound [.mana [generic 3], .perform (discard .you (aAtRandom (.inZone hand)))])
    (.dealDamage thisEnchantment
      (.statOf (.stat .manaValue) (theVerbed (.action "Discard") .card .attributive .one))
      (target anyTarget))
theorem okPyromancy : Ability.check [] pyromancy = [] := by decide
def luckyOffering : Instruction :=
  .sequentially
    [ destroy (target (.and [artifact, .compare [.stat .manaValue] .atMost (.lit 3)])),
      gainsLife .you (.lit 3) ]
theorem okLuckyOffering : Instruction.check [] luckyOffering = [] := by decide
def overload : Instruction :=
  .onlyIf (destroy (target artifact)) (.compareAmt (.statOf (.stat .manaValue) it) .atMost (.lit 2))
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
        [ activated .tapSymbol (.addMana .you (.lit 1) (.runs [[.colorless]]) []),
          activatedOnlyIf (.compound [.mana [generic 3], .tapSymbol]) (.draw .you (.lit 1))
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
            (.addMana .you (.lit 1) (.runs [[.colorless]]) []) ],
      power := stat 0, toughness := stat 0 } }

def labyrinthOfSkophos : Spelled := spelled <| .singleFaced
  { characteristics :=
    { name := "Labyrinth of Skophos", types := [.land],
      text :=
        [ activated .tapSymbol (.addMana .you (.lit 1) (.runs [[.colorless]]) []),
          activated (.compound [.mana [generic 4], .tapSymbol])
            (.removeFromCombat (target (.and [creature, .or [attacking, blocking]]))) ] } }

def acceleratedMutation : Spelled := spelled <| .singleFaced
  { characteristics :=
    { name := "Accelerated Mutation", cost := some [generic 3, pip .green, pip .green],
      types := [.instant],
      text :=
        [ .spell none (.sequentially
            [ gets (target creature) (.up (.letter .x)) (.up (.letter .x)) (some untilEndOfTurn),
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
            (.draw .you (.lit 1)) ],
      power := stat 2, toughness := stat 4 } }

def femerefEnchantress : Spelled := spelled <| .singleFaced
  { characteristics :=
    { name := "Femeref Enchantress", cost := some [pip .green, pip .white], types := [.creature],
      subtypes := [creatureType "Human", creatureType "Druid"],
      text :=
        [ whenever (putIntoFrom (a enchantment) graveyard (.zones [battlefield]))
            (.draw .you (.lit 1)) ],
      power := stat 1, toughness := stat 2 } }

def tocasiasWelcome : Spelled := spelled <| .singleFaced
  { characteristics :=
    { name := "Tocasia's Welcome", cost := some [generic 2, pip .white], types := [.enchantment],
      text :=
        [ triggeredOnlyOnce
            (.enters (counted (atLeast 1) (.and [creature, .hasPossessor .controller .you,
                                                 .compare [.stat .manaValue] .atMost (.lit 3)])) none)
            .oncePerTurn (.draw .you (.lit 1)) ] } }

def duskLegionDuelist : Spelled := spelled <| .singleFaced
  { characteristics :=
    { name := "Dusk Legion Duelist", cost := some [generic 1, pip .white], types := [.creature],
      subtypes := [creatureType "Vampire", creatureType "Soldier"],
      text :=
        [ keyword "Vigilance",
          triggeredOnlyOnce (counterEvent .put plusOnePlusOne .many thisCreature) .oncePerTurn
            (.draw .you (.lit 1)) ],
      power := stat 2, toughness := stat 2 } }

def mishrasFactory : Spelled := spelled <| .singleFaced
  { characteristics :=
    { name := "Mishra's Factory", types := [.land],
      text :=
        [ activated .tapSymbol (.addMana .you (.lit 1) (.runs [[.colorless]]) []),
          activated (.mana [generic 1])
            (.continuously
              (.becomes thisLand .sets
                (.bundle
                  { characteristics :=
                    { types := [.artifact, .creature], subtypes := [creatureType "AssemblyWorker"],
                      power := stat 2, toughness := stat 2 } }
                  (some .land)))
              (some untilEndOfTurn)),
          activated .tapSymbol
            (gets (target (.and [creature, .hasSubtype (creatureType "AssemblyWorker")]))
              (.up (.lit 1)) (.up (.lit 1)) (some untilEndOfTurn)) ] } }

/-- Mutavault -/
def mutavault : Spelled := spelled <| .singleFaced
  { characteristics :=
    { name := "Mutavault", types := [.land],
      text :=
        [ activated .tapSymbol (.addMana .you (.lit 1) (.runs [[.colorless]]) []),
          activated (.mana [generic 1])
            (.continuously
              (.becomes thisLand .sets
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
        [ activated .tapSymbol (.addMana .you (.lit 1) (.runs [[.colorless]]) []),
          activated (.mana [generic 4])
            (.continuously
              (.becomes thisLand .sets
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
          activated .tapSymbol (.addMana .you (.lit 1) (.runs [[.of .red], [.of .green]]) []),
          activated (.mana [generic 2, pip .red, pip .green])
            (.continuously
              (.becomes thisLand .sets
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
            (.sequentially
              [ scry .you (.lit 1),
                .may .you
                  (.setStatus .tapped (a (.and [artifact, untapped, .hasPossessor .controller .you])))
                  (some (.draw .you (.lit 1))) none ]),
          activated (.loyaltySymbol (.down 2))
            (.sequentially
              [ create (.lit 2)
                  { characteristics :=
                    { types := [.artifact, .creature], subtypes := [creatureType "Thopter"],
                      text := [keyword "Flying"], power := stat 1, toughness := stat 1 } },
                gainsHaste them (some untilEndOfTurn) ]),
          activated (.loyaltySymbol (.down 4))
            (.getsEmblem .you
              [ .static (getsPt (allOf (.and [artifact, creature, .hasPossessor .controller .you]))
                  (.up (.lit 1)) (.up (.lit 1))),
                .static (.costs (allOf (.and [artifact, spell, castBy .you])) (.less (.lit 1) none)) ]) ],
      loyalty := stat 3 } }

def manalith : Spelled := spelled <| .singleFaced
  { characteristics :=
    { name := "Manalith", cost := some [generic 3], types := [.artifact],
      text := [activated .tapSymbol (.addMana .you (.lit 1) (.anyColor .sameColor) [])] } }

def seethingSong : Spelled := spelled <| .singleFaced
  { characteristics :=
    { name := "Seething Song", cost := some [generic 2, pip .red], types := [.instant],
      text :=
        [ .spell none
            (.addMana .you (.lit 1) (.runs [[.of .red, .of .red, .of .red, .of .red, .of .red]]) []) ] } }

def ancientZiggurat : Spelled := spelled <| .singleFaced
  { characteristics :=
    { name := "Ancient Ziggurat", types := [.land],
      text :=
        [ activated .tapSymbol
            (.addMana .you (.lit 1) (.anyColor .sameColor)
              [.spendOnly [.toCast (.and [creature, spell])]]) ] } }

def mishrasWorkshop : Spelled := spelled <| .singleFaced
  { characteristics :=
    { name := "Mishra's Workshop", types := [.land],
      text :=
        [ activated .tapSymbol
            (.addMana .you (.lit 1) (.runs [[.colorless, .colorless, .colorless]])
              [.spendOnly [.toCast (.and [artifact, spell])]]) ] } }

def boommobile : Ability :=
  when (.enters thisArtifact none)
    (.addMana .you (.lit 4) (.anyColor .sameColor) [.spendOnly [.toActivate none]])
theorem okBoommobile : Ability.check [] boommobile = [] := by decide

/-- Rosheen Meanderer -/
def rosheenMeanderer : Spelled := spelled <| .singleFaced
  { characteristics :=
    { name := "Rosheen Meanderer", cost := some [generic 3, hybridPip .red .green],
      supertypes := [.legendary], types := [.creature],
      subtypes := [creatureType "Giant", creatureType "Shaman"],
      text :=
        [ activated .tapSymbol
            (.addMana .you (.lit 1) (.runs [[.colorless, .colorless, .colorless, .colorless]])
              [.spendOnly [.toPay (.containing .variable)]]) ],
      power := stat 4, toughness := stat 4 } }

/-- Adarkar Unicorn -/
def adarkarUnicorn : Spelled := spelled <| .singleFaced
  { characteristics :=
    { name := "Adarkar Unicorn", cost := some [generic 1, pip .white, pip .white],
      types := [.creature], subtypes := [creatureType "Unicorn"],
      text :=
        [ activated .tapSymbol
            (.addMana .you (.lit 1) (.runs [[.of .blue], [.colorless, .of .blue]])
              [.spendOnly [.toPay (.ofKeyword "CumulativeUpkeep")]]) ],
      power := stat 2, toughness := stat 2 } }

/-- Overgrown Zealot -/
def overgrownZealot : Spelled := spelled <| .singleFaced
  { characteristics :=
    { name := "Overgrown Zealot", cost := some [generic 1, pip .green], types := [.creature],
      subtypes := [creatureType "Elf", creatureType "Druid"],
      text :=
        [ activated .tapSymbol (.addMana .you (.lit 1) (.anyColor .sameColor) []),
          activated .tapSymbol
            (.addMana .you (.lit 2) (.anyColor .sameColor)
              [.spendOnly [.toPay (.ofSpecialAction .turnFaceUp)]]) ],
      power := stat 0, toughness := stat 4 } }

/-- Unblinking Observer -/
def unblinkingObserver : Spelled := spelled <| .singleFaced
  { characteristics :=
    { name := "Unblinking Observer", cost := some [generic 1, pip .blue], types := [.creature],
      subtypes := [creatureType "Homunculus"],
      text :=
        [ activated .tapSymbol
            (.addMana .you (.lit 1) (.runs [[.of .blue]])
              [.spendOnly [.toPay (.ofKeyword "Disturb"), .toCast instantOrSorcery]]) ],
      power := stat 2, toughness := stat 1 } }

/-- Qarsi Deceiver -/
def qarsiDeceiverMorphSpend : Ability :=
  activated .tapSymbol
    (.addMana .you (.lit 1) (.runs [[.colorless]])
      [.spendOnly [ .toCast (.and [creature, faceDown]), .toPay (.ofSpecialAction .turnFaceUp),
                    .toPay (.ofKeyword "Morph") ]])
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
            (.addMana .you removedThisWay (.runs [[.of .red]]) []) ] } }

/-- Rootcoil Creeper -/
def rootcoilCreeperGraveyardMana : Ability :=
  activated .tapSymbol
    (.addMana .you (.lit 2) (.anyColor .sameColor)
      [.spendOnly [.toCast (.and [spell, .castFrom (graveyardOf .you)])]])
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
            (.sequentially
              [ .addMana .you (.lit 1) (.runs [[.of .black]]) [],
                .addMana .you removedThisWay (.runs [[.of .black]]) [] ]) ] } }

/-- Elemental Resonance -/
def elementalResonance : Spelled := spelled <| .singleFaced
  { characteristics :=
    { name := "Elemental Resonance", cost := some [generic 2, pip .green, pip .green],
      types := [.enchantment], subtypes := [enchantmentType "Aura"],
      text :=
        [ keywordSubject "Enchant" permanent,
          at_ (.beginningOf .the .firstMain (.byPlayer .you))
            (.addMana .you (.lit 1) (.asPrintedCost (.attachHost .enchanted .permanent)) []) ] } }

/-- Steelswarm Operator -/
def steelswarmOperatorMana : Instruction :=
  .addMana .you (.lit 1) (.runs [[.of .blue, .of .blue]])
    [.spendOnly [.toActivate (some (.and [source, artifact]))]]
theorem okSteelswarmOperatorMana : Instruction.check [] steelswarmOperatorMana = [] := by decide

def blastOfGenius : Spelled := spelled <| .singleFaced
  { characteristics :=
    { name := "Blast of Genius", cost := some [generic 4, pip .blue, pip .red], types := [.sorcery],
      text :=
        [ .spell none (.sequentially
            [ choose (target anyTarget),
              .draw .you (.lit 3),
              discard .you (a (.inZone hand)),
              .dealDamage .this
                (.statOf (.stat .manaValue) (theVerbed (.action "Discard") .card .attributive .one))
                thatJoin ]) ] } }

def riddleOfLightning : Spelled := spelled <| .singleFaced
  { characteristics :=
    { name := "Riddle of Lightning", cost := some [generic 3, pip .red, pip .red], types := [.instant],
      text :=
        [ .spell none (.sequentially
            [ choose (target anyTarget),
              scry .you (.lit 3),
              revealCards (topSlice (.lit 1)),
              .dealDamage .this (.statOf (.stat .manaValue) (that .card)) thatJoin ]) ] } }

def unstableFrontier : Spelled := spelled <| .singleFaced
  { characteristics :=
    { name := "Unstable Frontier", types := [.land],
      text :=
        [ activated .tapSymbol (.addMana .you (.lit 1) (.runs [[.colorless]]) []),
          activated .tapSymbol
            (.continuously
              (.becomes (target (.and [land, .hasPossessor .controller .you])) .sets
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
          when (.chapterMark [3, 4]) (.addMana .you (.lit 1) (.runs [[.of .red]]) []) ] } }

def theFlux : Spelled := spelled <| .singleFaced
  { characteristics :=
    { name := "The Flux", cost := some [generic 2, pip .red, pip .red], types := [.enchantment],
      subtypes := [enchantmentType "Saga"],
      text :=
        [ when (.chapterMark [1])
            (.dealDamage .this (.lit 4)
              (target (.and [creature, .hasPossessor .controller (a .opponent)]))),
          when (.chapterMark [2, 3, 4, 5])
            (.sequentially
              [ exile (topSlice (.lit 1)),
                .continuously
                  (mayPlayDeed (.action "Play") .you (that .card) none
                    (.play none none none false .itsOwnCost))
                  (some .thisTurn) ]),
          when (.chapterMark [6]) (.addMana .you (.lit 6) (.runs [[.of .red]]) []) ] } }

def dragonstormGlobe : Spelled := spelled <| .singleFaced
  { characteristics :=
    { name := "Dragonstorm Globe", cost := some [generic 3], types := [.artifact],
      text :=
        [ .static (entersWithAdditionalCounters
            (each (.and [.hasSubtype (creatureType "Dragon"), .hasPossessor .controller .you]))
            (.lit 1) plusOnePlusOne),
          activated .tapSymbol (.addMana .you (.lit 1) (.anyColor .sameColor) []) ] } }

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
            (.draw .you (.lit 1)) ],
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
            (some (.perform (exiles .you
              (a (.and [.colorIs .white, .compare [.stat .manaValue] .eq (.letter .x),
                        .inZone (handOf .you)])))))),
          .spell none
            (.continuously
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
            (some (.perform (exiles .you
              (a (.and [.colorIs .blue, .compare [.stat .manaValue] .eq (.letter .x),
                        .inZone (handOf .you)])))))),
          .spell none
            (.onlyIf (.counterSpell (target spell))
              (.compareAmt (.statOf (.stat .manaValue) it) .eq (.letter .x)) none) ] } }

def blazingShoal : Spelled := spelled <| .singleFaced
  { characteristics :=
    { name := "Blazing Shoal", cost := some [.variable, pip .red, pip .red], types := [.instant],
      subtypes := [spellType "Arcane"],
      text :=
        [ .static (.altCost .this
            (some (.perform (exiles .you
              (a (.and [.colorIs .red, .compare [.stat .manaValue] .eq (.letter .x),
                        .inZone (handOf .you)])))))),
          .spell none
            (gets (target creature) (.up (.letter .x)) (.up (.lit 0)) (some untilEndOfTurn)) ] } }

def sickeningShoal : Spelled := spelled <| .singleFaced
  { characteristics :=
    { name := "Sickening Shoal", cost := some [.variable, pip .black, pip .black], types := [.instant],
      subtypes := [spellType "Arcane"],
      text :=
        [ .static (.altCost .this
            (some (.perform (exiles .you
              (a (.and [.colorIs .black, .compare [.stat .manaValue] .eq (.letter .x),
                        .inZone (handOf .you)])))))),
          .spell none
            (gets (target creature) (.down (.letter .x)) (.down (.letter .x)) (some untilEndOfTurn)) ] } }

def nourishingShoal : Spelled := spelled <| .singleFaced
  { characteristics :=
    { name := "Nourishing Shoal", cost := some [.variable, pip .green, pip .green], types := [.instant],
      subtypes := [spellType "Arcane"],
      text :=
        [ .static (.altCost .this
            (some (.perform (exiles .you
              (a (.and [.colorIs .green, .compare [.stat .manaValue] .eq (.letter .x),
                        .inZone (handOf .you)])))))),
          .spell none (gainsLife .you (.letter .x)) ] } }

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
        [ .spell none (.sequentially
            [ move (target (.and [.not land, permanent, .compare [.stat .manaValue] .eq (.letter .x)]))
                hand,
              .draw .you (.lit 1) ]) ] } }

def entrancingMelody : Spelled := spelled <| .singleFaced
  { characteristics :=
    { name := "Entrancing Melody", cost := some [.variable, pip .blue, pip .blue], types := [.sorcery],
      text :=
        [ .spell none
            (.continuously
              (.gainsControl .you
                (target (.and [creature, .compare [.stat .manaValue] .eq (.letter .x)])))
              none) ] } }

def ratchetBomb : Spelled := spelled <| .singleFaced
  { characteristics :=
    { name := "Ratchet Bomb", cost := some [generic 2], types := [.artifact],
      text :=
        [ activated .tapSymbol (.putCounters (.lit 1) (.printed (.named "Charge")) thisArtifact),
          activated (.compound [.tapSymbol, .perform (sacrifice .you thisArtifact)])
            (destroy (each (.and [.not land, permanent,
              .compare [.stat .manaValue] .eq (countersOn (.named "Charge") thisArtifact)]))) ] } }

def solGrail : Spelled := spelled <| .singleFaced
  { characteristics :=
    { name := "Sol Grail", cost := some [generic 3], types := [.artifact],
      text :=
        [ .static (entersChoosing thisArtifact .color),
          activated .tapSymbol (.addMana .you (.lit 1) (.ofChosenColor none) []) ] } }

def unchartedHaven : Spelled := spelled <| .singleFaced
  { characteristics :=
    { name := "Uncharted Haven", types := [.land],
      text :=
        [ .static (entersTapped thisLand),
          .static (entersChoosing thisLand .color),
          activated .tapSymbol (.addMana .you (.lit 1) (.ofChosenColor none) []) ] } }

def mirageMesa : Spelled := spelled <| .singleFaced
  { characteristics :=
    { name := "Mirage Mesa", types := [.land], subtypes := [landType "Desert"],
      text :=
        [ .static (entersTapped thisLand),
          .static (entersChoosing thisLand .color),
          activated .tapSymbol (.addMana .you (.lit 1) (.ofChosenColor none) []) ] } }

def crossroadsVillage : Spelled := spelled <| .singleFaced
  { characteristics :=
    { name := "Crossroads Village", types := [.land], subtypes := [landType "Town"],
      text :=
        [ .static (entersTapped thisLand),
          .static (entersChoosing thisLand .color),
          activated .tapSymbol (.addMana .you (.lit 1) (.ofChosenColor none) []) ] } }

/-- A Thriving land: enters tapped choosing a color other than its own, taps for either. -/
def thrivingLand (name : String) (own : Color) : Card := .singleFaced
  { characteristics :=
    { name, types := [.land],
      text :=
        [ .static (entersTapped thisLand),
          .static (entersChoosingFrom thisLand .color (.colorOtherThan own)),
          activated .tapSymbol (.addMana .you (.lit 1) (.ofChosenColor (some [.of own])) []) ] } }

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
            (.draw .you (.lit 1)) ] } }

def coalStoker : Spelled := spelled <| .singleFaced
  { characteristics :=
    { name := "Coal Stoker", cost := some [generic 3, pip .red], types := [.creature],
      subtypes := [creatureType "Elemental"],
      text :=
        [ triggeredIf (.enters thisCreature none) (.matches it (.castFrom (handOf .you)))
            (.addMana .you (.lit 1) (.runs [[.of .red, .of .red, .of .red]]) []) ],
      power := stat 3, toughness := stat 3 } }

def vegaTheWatcher : Spelled := spelled <| .singleFaced
  { characteristics :=
    { name := "Vega, the Watcher", cost := some [generic 1, pip .white, pip .blue],
      supertypes := [.legendary], types := [.creature],
      subtypes := [creatureType "Bird", creatureType "Spirit"],
      text :=
        [ keyword "Flying",
          whenever (.casts .you (a (.and [spell, .not (.castFrom (handOf .you))])) none)
            (.draw .you (.lit 1)) ],
      power := stat 2, toughness := stat 2 } }

def adNauseam : Spelled := spelled <| .singleFaced
  { characteristics :=
    { name := "Ad Nauseam", cost := some [generic 3, pip .black, pip .black], types := [.instant],
      text :=
        [ .spell none (.sequentially
            [ revealCards (topSlice (.lit 1)),
              move (that .card) hand,
              losesLife .you (.statOf (.stat .manaValue) it),
              may .you (.repeat_ .anyNumber) ]) ] } }

def nahiriLoyaltyRead : Predicate :=
  .and [ creature, .inZone (graveyardOf .you),
         .compare [.stat .manaValue] .less (.statOf (.stat .loyalty) .this) ]
theorem okNahiriLoyaltyRead : Predicate.check .object [] nahiriLoyaltyRead = [] := by decide

/-- Caustic Bronco -/
def causticBroncoLoss : Instruction :=
  .sequentially
    [ revealCards (topSlice (.lit 1)),
      move (that .card) hand,
      .onlyIf (losesLife .you (.statOf (.stat .manaValue) it))
        (.not (.matches thisCreature (.hasDesignation "saddled" none)))
        (some (losesLife (each .opponent) .thatMuch)) ]
theorem okCausticBroncoLoss : Instruction.check [] causticBroncoLoss = [] := by decide

/-- Dark Fortress -/
def darkFortressMana : Ability :=
  activatedOnlyIf .tapSymbol (.addMana .you (.lit 1) (.runs [[.of .black], [.of .red]]) [])
    (.or [ happened .entry thisLand .thisTurn,
           exists_ (.and [land, .hasSupertype .basic, .hasPossessor .controller .you]) ])
theorem okDarkFortressMana : Ability.check [] darkFortressMana = [] := by decide

def branchloftPathway : Spelled := spelled <| .modalDfc
  { characteristics :=
    { name := "Branchloft Pathway", types := [.land],
      text := [activated .tapSymbol (.addMana .you (.lit 1) (.runs [[.of .green]]) [])] } }
  { characteristics :=
    { name := "Boulderloft Pathway", types := [.land],
      text := [activated .tapSymbol (.addMana .you (.lit 1) (.runs [[.of .white]]) [])] } }

/-- Nykthos, Shrine to Nyx -/
def nykthosShrineToNyx : Spelled := spelled <| .singleFaced
  { characteristics :=
    { name := "Nykthos, Shrine to Nyx", supertypes := [.legendary], types := [.land],
      text :=
        [ activated .tapSymbol (.addMana .you (.lit 1) (.runs [[.colorless]]) []),
          activated (.compound [.mana [generic 2], .tapSymbol])
            (.sequentially
              [ choose (a (quality .color)),
                .addMana .you (.devotion .you thatColor none) (.ofChosenColor none) [] ]) ] } }

/-- Karametra's Acolyte -/
def karametrasAcolyte : Ability :=
  activated .tapSymbol (.addMana .you (.devotion .you (.lit .green) none) (.runs [[.of .green]]) [])
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
            (.draw .you (.lit 1)),
          activated (.compound [.mana [generic 2], .perform (sacrifice .you thisArtifact)])
            (.draw .you (.lit 1)) ] } }

/-- Engineered Explosives (its sunburst [CR#702.44a] written out) -/
def engineeredExplosives : Spelled := spelled <| .singleFaced
  { characteristics :=
    { name := "Engineered Explosives", cost := some [.variable], types := [.artifact],
      text :=
        [ .static (entersWithCounters thisArtifact (colorsSpentToCast thisArtifact) (.named "Charge")),
          activated (.compound [.mana [generic 2], .perform (sacrifice .you thisArtifact)])
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
    (.compound [.mana [generic 1, phyrexianPip .green], .tapSymbol, .perform (sacrifice .you (a creature))])
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
    (may .you
      (.sequentially
        [ searchLibraryFor (exactly 1)
            (.and [creature, .compare [.stat .manaValue] .eq (countersOn (.named "Age") thisEnchantment)]),
          putOntoBattlefield (that .card),
          shuffle ]))
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
            (.draw .you (.lit 1)) ],
      power := stat 3, toughness := stat 1 } }

/-- Bloom Tender -/
def bloomTenderMana : Ability :=
  abilityWord "vivid"
    (activated .tapSymbol
      (.forEachKindOf .color (some (allOf (.and [permanent, .hasPossessor .controller .you]))) .color
        (.addMana .you (.lit 1) (.ofChosenColor none) [])))
theorem okBloomTenderMana : Ability.check [] bloomTenderMana = [] := by decide

/-- Tarnation Vista -/
def tarnationVistaMana : Ability :=
  activated (.compound [.mana [generic 1], .tapSymbol])
    (.forEachKindOf .color
      (some (allOf (.and [permanent, monocolored, .hasPossessor .controller .you]))) .color
      (.addMana .you (.lit 1) (.ofChosenColor none) []))
theorem okTarnationVistaMana : Ability.check [] tarnationVistaMana = [] := by decide

/-- Military Intelligence -/
def militaryIntelligence : Spelled := spelled <| .singleFaced
  { characteristics :=
    { name := "Military Intelligence", cost := some [generic 1, pip .blue], types := [.enchantment],
      text :=
        [ whenever (.attacksWith .you none (counted (atLeast 2) creature)) (.draw .you (.lit 1)) ] } }

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
            (.draw .you (.lit 1)) ],
      power := stat 3, toughness := stat 3 } }

/-- Gnarlback Rhino -/
def gnarlbackRhino : Spelled := spelled <| .singleFaced
  { characteristics :=
    { name := "Gnarlback Rhino", cost := some [generic 2, pip .green, pip .green], types := [.creature],
      subtypes := [creatureType "Rhino"],
      text :=
        [ keyword "Trample",
          whenever (.casts .you (a (.and [spell, .targets thisCreature .someTarget])) none)
            (.draw .you (.lit 1)) ],
      power := stat 4, toughness := stat 4 } }

/-- Prismari Pianist -/
def prismariPianist : Spelled := spelled <| .singleFaced
  { characteristics :=
    { name := "Prismari Pianist", cost := some [generic 1, pip .red, pip .red], types := [.creature],
      subtypes := [creatureType "Djinn", creatureType "Bard"],
      text :=
        [ whenever (.casts .you (a (.and [instantOrSorcery, spell])) none)
            (.insteadOf
              (.create .you (.lit 1)
                (.written (creatureToken 1 1 [.blue, .red] [creatureType "Elemental"])) [])
              (.if_ (.compareAmt (.statOf (.stat .manaValue) (that .spell)) .atLeast (.lit 5))
                (.create .you (.lit 3) .asThose []) none)) ],
      power := stat 2, toughness := stat 1 } }

/-- Collected Company -/
def collectedCompany : Spelled := spelled <| .singleFaced
  { characteristics :=
    { name := "Collected Company", cost := some [generic 3, pip .green], types := [.instant],
      text :=
        [ .spell none (.sequentially
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
              .perform (sacrifice .you (a (.and [creature, .or [.colorIs .black, artifact]])))])
            (.addMana .you (.statOf (.stat .manaValue) (itVerbed (.action "Sacrifice")))
              (.runs [[.of .black]]) []) ],
      power := stat 1, toughness := stat 2 } }

/-- Delivery Moogle -/
def deliveryMoogle : Spelled := spelled <| .singleFaced
  { characteristics :=
    { name := "Delivery Moogle", cost := some [generic 3, pip .white], types := [.creature],
      subtypes := [creatureType "Moogle"],
      text :=
        [ keyword "Flying",
          when (.enters thisCreature none)
            (.sequentially
              [ searchLibraryOrGraveyard (.and [artifact, .compare [.stat .manaValue] .atMost (.lit 2)]),
                revealsIt,
                move foundCard hand,
                .if_ (happenedAt (.verbedAct (.action "Search")) .you .thisWay yourLibrary) shuffle none ]) ],
      power := stat 3, toughness := stat 2 } }

/-- Heart of Yavimaya -/
def heartOfYavimaya : Spelled := spelled <| .singleFaced
  { characteristics :=
    { name := "Heart of Yavimaya", types := [.land],
      text :=
        [ .static (.intercepts (.enters .this none) [] none
            (.ifDone (sacrifice .you (a (.and [land, .hasSubtype (landType "Forest")])))
              (some (putOntoBattlefield .this)) (some (move .this graveyard)))
            .repeatedly none),
          activated .tapSymbol (.addMana .you (.lit 1) (.runs [[.of .green]]) []),
          activated .tapSymbol
            (gets (target creature) (.up (.lit 1)) (.up (.lit 1)) (some untilEndOfTurn)) ] } }

/-- Mox Diamond -/
def moxDiamond : Spelled := spelled <| .singleFaced
  { characteristics :=
    { name := "Mox Diamond", cost := some [], types := [.artifact],
      text :=
        [ .static (.intercepts (.enters .this none) [] none
            (.may .you (discard .you (a (.and [land, .inZone hand])))
              (some (putOntoBattlefield .this)) (some (move .this graveyard)))
            .repeatedly none),
          activated .tapSymbol (.addMana .you (.lit 1) (.anyColor .sameColor) []) ] } }

/-- Up the Beanstalk -/
def upTheBeanstalk : Spelled := spelled <| .singleFaced
  { characteristics :=
    { name := "Up the Beanstalk", cost := some [generic 1, pip .green], types := [.enchantment],
      text :=
        [ triggeredJoined (.enters .this none)
            [joinedHead (.casts .you (a (.and [spell, .compare [.stat .manaValue] .atLeast (.lit 5)])) none)]
            (.draw .you (.lit 1)) ] } }

/-- Sheltered Valley -/
def shelteredValley : Spelled := spelled <| .singleFaced
  { characteristics :=
    { name := "Sheltered Valley", types := [.land],
      text :=
        [ .static (.intercepts (.enters .this none) [] none
            (.sequentially
              [ sacrifice .you
                  (each (.and [permanent, .otherThan thisLand, .named (.printed "Sheltered Valley"),
                               .hasPossessor .controller .you])),
                putOntoBattlefield .this ])
            .repeatedly none),
          triggeredIf (.beginningOf .the .upkeep (.byPlayer .you))
            (.compareAmt (countOf (.and [land, .hasPossessor .controller .you])) .atMost (.lit 3))
            (gainsLife .you (.lit 1)),
          activated .tapSymbol (.addMana .you (.lit 1) (.runs [[.colorless]]) []) ] } }

/-- Soul Shatter -/
def soulShatter : Instruction :=
  sacrifice (each .opponent)
    (a (.and [.or [creature, .hasType .planeswalker],
              .superlative .max (.stat .manaValue)
                (.and [.or [creature, .hasType .planeswalker], .hasPossessor .controller they])]))
theorem okSoulShatter : Instruction.check [] soulShatter = [] := by decide

/-- Padeem, Consul of Innovation -/
def padeemConsulOfInnovation : Ability :=
  triggeredIf (.beginningOf .the .upkeep (.byPlayer .you))
    (.matches
      (the (.and [artifact,
                  .superlative .max (.stat .manaValue) (.and [artifact, .inZone battlefield])]))
      (.hasPossessor .controller .you))
    (.draw .you (.lit 1))
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
            (.sequentially [losesLife (that .player) (.lit 2), .draw .you (.lit 1)]) ],
      power := stat 3, toughness := stat 4 } }

/-- Braid of Fire -/
def braidOfFire : Spelled := spelled <| .singleFaced
  { characteristics :=
    { name := "Braid of Fire", cost := some [generic 1, pip .red], types := [.enchantment],
      text := [cumulativeUpkeep (.perform (.addMana .you (.lit 1) (.runs [[.of .red]]) []))] } }

/-- Delighted Halfling -/
def delightedHalflingMana : Ability :=
  activated .tapSymbol
    (.addMana .you (.lit 1) (.anyColor .sameColor)
      [ .onSpent .affectsIt true (a (.and [.hasSupertype .legendary, spell]))
          (.continuously (objectCant (.action "Counter") (that .spell)) none) ])
theorem okDelightedHalflingMana : Ability.check [] delightedHalflingMana = [] := by decide

/-- Boseiju, Who Shelters All -/
def boseijuMana : Ability :=
  activated (.compound [.tapSymbol, payLife .you 2])
    (.addMana .you (.lit 1) (.runs [[.colorless]])
      [ .onSpent .affectsIt false (a (.and [instantOrSorcery, spell]))
          (.continuously (objectCant (.action "Counter") (that .spell)) none) ])
theorem okBoseijuMana : Ability.check [] boseijuMana = [] := by decide

/-- Generator Servant -/
def generatorServant : Spelled := spelled <| .singleFaced
  { characteristics :=
    { name := "Generator Servant", cost := some [generic 1, pip .red], types := [.creature],
      subtypes := [creatureType "Elemental"],
      text :=
        [ activated (.compound [.tapSymbol, .perform (sacrifice .you thisCreature)])
            (.addMana .you (.lit 1) (.runs [[.colorless, .colorless]])
              [ .onSpent .affectsIt false (a (.and [creature, spell]))
                  (gainsHaste (.resolvedPermanent (that .spell)) (some untilEndOfTurn)) ]) ],
      power := stat 2, toughness := stat 1 } }

/-- Animal Attendant -/
def animalAttendant : Spelled := spelled <| .singleFaced
  { characteristics :=
    { name := "Animal Attendant", cost := some [generic 1, pip .green], types := [.creature],
      subtypes := [creatureType "Human", creatureType "Citizen"],
      text :=
        [ activated .tapSymbol
            (.addMana .you (.lit 1) (.anyColor .sameColor)
              [ .onSpent .affectsIt false
                  (a (.and [.not (.hasSubtype (creatureType "Human")), creature, spell]))
                  (.continuously
                    (.entersRider (.resolvedPermanent (that .spell))
                      (.withCounters (.lit 1) (.printed plusOnePlusOne) .additional))
                    none) ]) ],
      power := stat 2, toughness := stat 2 } }

/-- Thran Turbine -/
def thranTurbineMana : Instruction :=
  .addMana .you (.lit 1) (.runs [[.colorless, .colorless]]) [.spendNotOn [.toCast spell]]
theorem okThranTurbineMana : Instruction.check [] thranTurbineMana = [] := by decide

/-- Su-Chi Cave Guard -/
def suChiCaveGuardDies : Ability :=
  when (.dies thisCreature)
    (.sequentially
      [ .addMana .you (.lit 1)
          (.runs [[.colorless, .colorless, .colorless, .colorless,
                   .colorless, .colorless, .colorless, .colorless]]) [],
        .continuously (.keepsUnspentMana .you .thisMana) (some untilEndOfTurn) ])
theorem okSuChiCaveGuardDies : Ability.check [] suChiCaveGuardDies = [] := by decide

/-- Omnath, Locus of Mana -/
def omnathLocusOfManaPersistence : StaticSpec := .keepsUnspentMana .you (.unspent (some (.of .green)))
theorem okOmnathLocusOfManaPersistence :
    StaticSpec.check [] omnathLocusOfManaPersistence = [] := by decide

/-- Upwelling -/
def upwelling : Spelled := spelled <| .singleFaced
  { characteristics :=
    { name := "Upwelling", cost := some [generic 3, pip .green], types := [.enchantment],
      text := [.static (.keepsUnspentMana (each .anyPlayer) (.unspent none))] } }

/-- Mana Flare -/
def manaFlare : Spelled := spelled <| .singleFaced
  { characteristics :=
    { name := "Mana Flare", cost := some [generic 2, pip .red], types := [.enchantment],
      text :=
        [ whenever (.tappedForMana (some (a .anyPlayer)) (a land) none)
            (.addMana (that .player) (.lit 1) (.producedByEvent (that (.type .land))) []) ] } }

/-- Shimmerwilds Growth -/
def shimmerwildsGrowth : Spelled := spelled <| .singleFaced
  { characteristics :=
    { name := "Shimmerwilds Growth", cost := some [generic 1, pip .green], types := [.enchantment],
      subtypes := [enchantmentType "Aura"],
      text :=
        [ keywordSubject "Enchant" land,
          .static (entersChoosing thisAura .color),
          .static (.becomes (.attachHost .enchanted (.type .land)) .sets
            (.chosenQuality (ofChosen .color))),
          whenever (.tappedForMana none (.attachHost .enchanted (.type .land)) none)
            (.addMana (controllerOf (that (.type .land))) (.lit 1) (.ofChosenColor none) []) ] } }

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
            (.addMana (controllerOf (that (.type .land))) (.lit 1) (.ofChosenColor none) []) ] } }

/-- Chrome Mox -/
def chromeMoxMana : Ability :=
  activated .tapSymbol
    (.addMana .you (.lit 1) (.amongColorsOf (the (.exiledWith thisArtifact))) [])
theorem okChromeMoxMana : Ability.check [] chromeMoxMana = [] := by decide

/-- Fellwar Stone -/
def fellwarStone : Spelled := spelled <| .singleFaced
  { characteristics :=
    { name := "Fellwar Stone", cost := some [generic 2], types := [.artifact],
      text :=
        [ activated .tapSymbol
            (.addMana .you (.lit 1)
              (.couldProduce (a (.and [land, .hasPossessor .controller anOpponent]))) []) ] } }

/-- Ice Cauldron -/
def iceCauldronNotedMana : Ability :=
  activated
    (.compound [.tapSymbol,
      .perform (.removeCounters (some (exactly 1)) (some (.printed (.named "Charge"))) thisArtifact)])
    (.addMana .you (.lit 1) (.lastNoted thisArtifact)
      [.spendOnly [.toCast (.exiledWith thisArtifact)]])
theorem okIceCauldronNotedMana : Ability.check [] iceCauldronNotedMana = [] := by decide

/-- Firemind Vessel -/
def firemindVesselMana : Ability :=
  activated .tapSymbol (.addMana .you (.lit 2) (.anyColor .distinctColors) [])
theorem okFiremindVesselMana : Ability.check [] firemindVesselMana = [] := by decide

/-- Goblin Clearcutter -/
def goblinClearcutterMana : Ability :=
  activated
    (.compound [.tapSymbol, .perform (sacrifice .you (a (.hasSubtype (landType "Forest"))))])
    (.addMana .you (.lit 3) (.amongWritten [.red, .green]) [])
theorem okGoblinClearcutterMana : Ability.check [] goblinClearcutterMana = [] := by decide

/-- Tolaria -/
def tolaria : Spelled := spelled <| .singleFaced
  { characteristics :=
    { name := "Tolaria", supertypes := [.legendary], types := [.land],
      text :=
        [ activated .tapSymbol (.addMana .you (.lit 1) (.runs [[.of .blue]]) []),
          activatedOnlyDuring (.compound [.tapSymbol])
            (.continuously
              (.losesAbilities (target creature)
                [.written (keyword "Banding"), .term bandsWithOtherAbilities])
              (some untilEndOfTurn))
            (.duringPart .upkeep none) ] } }

/-- Psychic Vortex -/
def psychicVortexUpkeep : Ability := cumulativeUpkeep (.perform (.draw .you (.lit 1)))
theorem okPsychicVortexUpkeep : Ability.check [] psychicVortexUpkeep = [] := by decide

/-- Varchild's War-Riders -/
def varchildsWarRidersUpkeep : Ability :=
  cumulativeUpkeep
    (.perform (.create anOpponent (.lit 1)
      (.written (creatureToken 1 1 [.red] [creatureType "Survivor"])) []))
theorem okVarchildsWarRidersUpkeep : Ability.check [] varchildsWarRidersUpkeep = [] := by decide

end Semantics.Cards
