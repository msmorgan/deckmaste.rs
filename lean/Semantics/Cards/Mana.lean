import Semantics
import Semantics.Definitions
import Semantics.Check.Card
import Semantics.Cards.Anaphora
import Semantics.Cards.Keyword

/-!
# Semantics.Cards.Mana

Port of `idris/src/Experimental/Cards/Mana.idr`: the printed cards of the Mana family and the
bench items beside them.
-/

open Semantics Semantics.Templates Semantics.Definitions

namespace Semantics.Cards

def boshIronGolem : Ability :=
  activated (compound [mana [generic 3, pip .red], perform (sacrifice (a isArtifact))])
    (instruct this_
       (dealDamage
          (statOf (.stat .manaValue) (theVerbed (.action "sacrifice") .one (some artifact)))
        (target isAnyTarget) .noncombatOnly))
theorem okBoshIronGolem : Ability.check (DRS.ofList []) boshIronGolem = [] := by decide
def pyromancy : Ability :=
  activated (compound [mana [generic 3], perform (discard (aAtRandom (cardIn yourHand)))])
    (instruct thisEnchantment (dealDamage (statOf (.stat .manaValue) (theVerbed (.action "discard") .one (some .card))) (target isAnyTarget) .noncombatOnly))
theorem okPyromancy : Ability.check (DRS.ofList []) pyromancy = [] := by decide
def luckyOffering : Action :=
  sequentially
    [ destroy (target (and [isArtifact, hasManaValue .atMost (.lit 3)])),
      gainLife (.lit 3) ]
theorem okLuckyOffering : Action.check (DRS.ofList []) luckyOffering = [] := by decide
def overload : Action :=
  doOnlyIf (destroy (target isArtifact)) (compareAmt (statOf (.stat .manaValue) it) .atMost (.lit
      2))
theorem okOverload : Action.check (DRS.ofList []) overload = [] := by decide
def austereCommand : Action :=
  chooseModes (exactly 2)
    [ destroy (allOf isArtifact),
      destroy (allOf isEnchantment),
      destroy (allOf (and [isCreature, hasManaValue .atMost (.lit 3)])),
      destroy (allOf (and [isCreature, hasManaValue .atLeast (.lit 4)])) ]
theorem okAustereCommand : Action.check (DRS.ofList []) austereCommand = [] := by decide

def bondersEnclave : Spelled := spelled <| .singleFaced
  { characteristics :=
    { name := "Bonders' Enclave", types := [.land],
      text :=
        [ activated tapSymbol (addMana (.lit 1) (manaOf [[.colorless]])),
          activatedOnlyIf (compound [mana [generic 3], tapSymbol]) (draw (.lit 1))
            (exists_ (and [isCreature, youControl,
                            hasPower .atLeast (.lit 4)])) ] } }

def workhorse : Spelled := spelled <| .singleFaced
  { characteristics :=
    { name := "Workhorse", cost := some [generic 6], types := [.artifact, .creature],
      subtypes := [horse],
      text :=
        [ static (entersWithCounters thisCreature (.lit 4) p1p1Counter),
          activated
            (perform (removeCounters (exactly 1) (some (printedKind p1p1Counter)) thisCreature .many true))
            (addMana (.lit 1) (manaOf [[.colorless]])) ],
      power := stat 0, toughness := stat 0 } }

def labyrinthOfSkophos : Spelled := spelled <| .singleFaced
  { characteristics :=
    { name := "Labyrinth of Skophos", types := [.land],
      text :=
        [ activated tapSymbol (addMana (.lit 1) (manaOf [[.colorless]])),
          activated (compound [mana [generic 4], tapSymbol])
            (removeFromCombat (target (and [isCreature, or [isAttacking, isBlocking]]))) ] } }

def acceleratedMutation : Spelled := spelled <| .singleFaced
  { characteristics :=
    { name := "Accelerated Mutation", cost := some [generic 3, pip .green, pip .green],
      types := [.instant],
      text :=
        [ spell (sequentially
            [ get (target isCreature) (up (letter .x)) (up (letter .x)) (some untilEndOfTurn),
              define .x (aggregate .max (.stat .manaValue)
                (and [isPermanent, youControl])) ]) ] } }

def cullingScales : Spelled := spelled <| .singleFaced
  { characteristics :=
    { name := "Culling Scales", cost := some [generic 3], types := [.artifact],
      text :=
        [ at_ (beginningOfYour .upkeep)
            (destroy (target (and [isPermanent, non isLand,
              superlative .min (.stat .manaValue) (and [isPermanent, non isLand])]))) ] } }

def deadeyeBrawler : Spelled := spelled <| .singleFaced
  { characteristics :=
    { name := "Deadeye Brawler", cost := some [generic 2, pip .blue, pip .black],
      types := [.creature], subtypes := [human, pirate],
      text :=
        [ deathtouch,
          ascend,
          triggeredIf (does thisCreature (dealDamage (to := a isPlayer) (kind := .combatOnly)))
            (matches_ you (hasDesignation citysBlessing))
            (draw (.lit 1)) ],
      power := stat 2, toughness := stat 4 } }

def femerefEnchantress : Spelled := spelled <| .singleFaced
  { characteristics :=
    { name := "Femeref Enchantress", cost := some [pip .green, pip .white], types := [.creature],
      subtypes := [human, druid],
      text :=
        [ whenever (does any (putInto (a isEnchantment) graveyard (from_ := some battlefield)))
            (draw (.lit 1)) ],
      power := stat 1, toughness := stat 2 } }

def tocasiasWelcome : Spelled := spelled <| .singleFaced
  { characteristics :=
    { name := "Tocasia's Welcome", cost := some [generic 2, pip .white], types := [.enchantment],
      text :=
        [ triggeredOnlyOnce
            (does (counted (atLeast 1) (and [isCreature, youControl,
                                                 hasManaValue .atMost (.lit 3)])) enter)
            oncePerTurn (draw (.lit 1)) ] } }

def duskLegionDuelist : Spelled := spelled <| .singleFaced
  { characteristics :=
    { name := "Dusk Legion Duelist", cost := some [generic 1, pip .white], types := [.creature],
      subtypes := [vampire, soldier],
      text :=
        [ vigilance,
          triggeredOnlyOnce (counterEvent .put p1p1Counter .many thisCreature) oncePerTurn
            (draw (.lit 1)) ],
      power := stat 2, toughness := stat 2 } }

def mishrasFactory : Spelled := spelled <| .singleFaced
  { characteristics :=
    { name := "Mishra's Factory", types := [.land],
      text :=
        [ activated tapSymbol (addMana (.lit 1) (manaOf [[.colorless]])),
          activated (mana [generic 1])
            (establishThroughout (qualityChange thisLand .sets
                (bundle
                  { characteristics :=
                    { types := [.artifact, .creature], subtypes := [creatureType "AssemblyWorker"],
                      power := stat 2, toughness := stat 2 } }
                  (some .land))) untilEndOfTurn),
          activated tapSymbol
            (get (target (and [isCreature, hasSubtype (creatureType "AssemblyWorker")]))
              (up (.lit 1)) (up (.lit 1)) (some untilEndOfTurn)) ] } }

/-- Mutavault -/
def mutavault : Spelled := spelled <| .singleFaced
  { characteristics :=
    { name := "Mutavault", types := [.land],
      text :=
        [ activated tapSymbol (addMana (.lit 1) (manaOf [[.colorless]])),
          activated (mana [generic 1])
            (establishThroughout (qualityChange thisLand .sets
                (bundle
                  { characteristics := { types := [.creature], power := stat 2, toughness := stat 2 },
                    qualities := [withEveryType .creature] }
                  (some .land))) untilEndOfTurn) ] } }

/-- Soulstone Sanctuary -/
def soulstoneSanctuary : Spelled := spelled <| .singleFaced
  { characteristics :=
    { name := "Soulstone Sanctuary", types := [.land],
      text :=
        [ activated tapSymbol (addMana (.lit 1) (manaOf [[.colorless]])),
          activated (mana [generic 4])
            (establish
              (qualityChange thisLand .sets
                (bundle
                  { characteristics :=
                    { types := [.creature], text := [vigilance],
                      power := stat 3, toughness := stat 3 },
                    qualities := [withEveryType .creature] }
                  (some .land)))) ] } }

def ragingRavine : Spelled := spelled <| .singleFaced
  { characteristics :=
    { name := "Raging Ravine", types := [.land],
      text :=
        [ static (entersTapped thisLand),
          activated tapSymbol (addMana (.lit 1) (manaOf [[.of .red], [.of .green]])),
          activated (mana [generic 2, pip .red, pip .green])
            (establishThroughout (qualityChange thisLand .sets
                (bundle
                  { characteristics :=
                    { colors := [.red, .green], types := [.creature],
                      subtypes := [elemental],
                      text :=
                        [ whenever (does thisCreature attack)
                            (putCounters (.lit 1) (printedKind p1p1Counter) thisCreature .many) ],
                      power := stat 3, toughness := stat 3 } }
                  (some .land))) untilEndOfTurn) ] } }

def saheeliFiligreeMaster : Spelled := spelled <| .singleFaced
  { characteristics :=
    { name := "Saheeli, Filigree Master", cost := some [generic 2, pip .blue, pip .red],
      supertypes := [.legendary], types := [.planeswalker], subtypes := [saheeli],
      text :=
        [ activated (loyaltySymbol (.up 1))
            (sequentially
              [ scry (.lit 1),
                may
                  (tap (a (and [isArtifact, isUntapped, youControl])))
                  (some (draw (.lit 1))) ]),
          activated (loyaltySymbol (.down 2))
            (sequentially
              [ createToken
                  { characteristics :=
                    { types := [.artifact, .creature], subtypes := [thopter],
                      text := [flying], power := stat 1, toughness := stat 1 } } (count := .lit 2),
                gainHaste them (some untilEndOfTurn) ]),
          activated (loyaltySymbol (.down 4))
            (getEmblem
              [ static (getsPt (allOf (and [isArtifact, isCreature, youControl]))
                  (up (.lit 1)) (up (.lit 1))),
                static (costShift (allOf (and [isArtifact, isSpell, castBy you])) (less (.lit 1))) ]) ],
      loyalty := stat 3 } }

def manalith : Spelled := spelled <| .singleFaced
  { characteristics :=
    { name := "Manalith", cost := some [generic 3], types := [.artifact],
      text := [activated tapSymbol (addMana (.lit 1) (ofAnyColor .sameColor))] }
          }

def seethingSong : Spelled := spelled <| .singleFaced
  { characteristics :=
    { name := "Seething Song", cost := some [generic 2, pip .red], types := [.instant],
      text :=
        [ spell (addMana (.lit 1) (manaOf [[.of .red, .of .red, .of .red, .of .red, .of .red]])) ] } }

def ancientZiggurat : Spelled := spelled <| .singleFaced
  { characteristics :=
    { name := "Ancient Ziggurat", types := [.land],
      text :=
        [ activated tapSymbol
            (addMana (.lit 1) (ofAnyColor .sameColor)
              [spendOnly [toCast (and [isCreature, isSpell])]]) ] } }

def mishrasWorkshop : Spelled := spelled <| .singleFaced
  { characteristics :=
    { name := "Mishra's Workshop", types := [.land],
      text :=
        [ activated tapSymbol
            (addMana (.lit 1) (manaOf [[.colorless, .colorless, .colorless]])
              [spendOnly [toCast (and [isArtifact, isSpell])]]) ] } }

def boommobile : Ability :=
  when (does thisArtifact enter)
    (addMana (.lit 4) (ofAnyColor .sameColor) [spendOnly [toActivate none]])
theorem okBoommobile : Ability.check (DRS.ofList []) boommobile = [] := by decide

/-- Rosheen Meanderer -/
def rosheenMeanderer : Spelled := spelled <| .singleFaced
  { characteristics :=
    { name := "Rosheen Meanderer", cost := some [generic 3, hybridPip .red .green],
      supertypes := [.legendary], types := [.creature],
      subtypes := [giant, shaman],
      text :=
        [ activated tapSymbol
            (addMana (.lit 1) (manaOf [[.colorless, .colorless, .colorless, .colorless]])
              [spendOnly [toPay (.containing .variable)]]) ],
      power := stat 4, toughness := stat 4 } }

/-- Adarkar Unicorn -/
def adarkarUnicorn : Spelled := spelled <| .singleFaced
  { characteristics :=
    { name := "Adarkar Unicorn", cost := some [generic 1, pip .white, pip .white],
      types := [.creature], subtypes := [unicorn],
      text :=
        [ activated tapSymbol
            (addMana (.lit 1) (manaOf [[.of .blue], [.colorless, .of .blue]])
              [spendOnly [toPay (.ofKeyword cumulativeUpkeep)]]) ],
      power := stat 2, toughness := stat 2 } }

/-- Overgrown Zealot -/
def overgrownZealot : Spelled := spelled <| .singleFaced
  { characteristics :=
    { name := "Overgrown Zealot", cost := some [generic 1, pip .green], types := [.creature],
      subtypes := [elf, druid],
      text :=
        [ activated tapSymbol (addMana (.lit 1) (ofAnyColor .sameColor)),
          activated tapSymbol
            (addMana (.lit 2) (ofAnyColor .sameColor)
              [spendOnly [toPay (.ofKeyword morph), toPay (.ofKeyword disguise)]]) ],
      power := stat 0, toughness := stat 4 } }

/-- Unblinking Observer -/
def unblinkingObserver : Spelled := spelled <| .singleFaced
  { characteristics :=
    { name := "Unblinking Observer", cost := some [generic 1, pip .blue], types := [.creature],
      subtypes := [homunculus],
      text :=
        [ activated tapSymbol
            (addMana (.lit 1) (manaOf [[.of .blue]])
              [spendOnly [toPay (.ofKeyword disturb), toCast isInstantOrSorcery]]) ],
      power := stat 2, toughness := stat 1 } }

/-- Qarsi Deceiver -/
def qarsiDeceiverMorphSpend : Ability :=
  activated tapSymbol
    (addMana (.lit 1) (manaOf [[.colorless]])
      [spendOnly [ toCast (and [isCreature, isFaceDown]), toPay (.ofKeyword morph) ]])
theorem okQarsiDeceiverMorphSpend : Ability.check (DRS.ofList []) qarsiDeceiverMorphSpend = [] := by decide

/-- Mercadian Bazaar -/
def mercadianBazaar : Spelled := spelled <| .singleFaced
  { characteristics :=
    { name := "Mercadian Bazaar", types := [.land],
      text :=
        [ static (entersTapped thisLand),
          activated tapSymbol (putCounters (.lit 1) (printedKind storageCounter) thisLand .many),
          activated
            (compound [tapSymbol,
              perform (removeCounters anyNumber (some (printedKind storageCounter)) thisLand .many true)])
            (addMana removedThisWay (manaOf [[.of .red]])) ] } }

/-- Rootcoil Creeper -/
def rootcoilCreeperGraveyardMana : Ability :=
  activated tapSymbol
    (addMana (.lit 2) (ofAnyColor .sameColor)
      [spendOnly [toCast (and [isSpell, castFrom yourGraveyard])]])
theorem okRootcoilCreeperGraveyardMana : Ability.check (DRS.ofList []) rootcoilCreeperGraveyardMana = [] := by
  decide

/-- Black Mana Battery -/
def blackManaBattery : Spelled := spelled <| .singleFaced
  { characteristics :=
    { name := "Black Mana Battery", cost := some [generic 4], types := [.artifact],
      text :=
        [ activated (compound [mana [generic 2], tapSymbol])
            (putCounters (.lit 1) (printedKind chargeCounter) thisArtifact .many),
          activated
            (compound [tapSymbol,
              perform (removeCounters anyNumber (some (printedKind chargeCounter)) thisArtifact .many true)])
            (sequentially
              [ addMana (.lit 1) (manaOf [[.of .black]]),
                addMana removedThisWay (manaOf [[.of .black]]) ]) ] } }

/-- Elemental Resonance -/
def elementalResonance : Spelled := spelled <| .singleFaced
  { characteristics :=
    { name := "Elemental Resonance", cost := some [generic 2, pip .green, pip .green],
      types := [.enchantment], subtypes := [aura],
      text :=
        [ enchant isPermanent,
          at_ (beginningOfYour .firstMain)
            (addMana (.lit 1) (matchingManaCostOf (enchanted permanent))) ] } }

/-- Steelswarm Operator -/
def steelswarmOperatorMana : Action :=
  addMana (.lit 1) (manaOf [[.of .blue, .of .blue]])
    [spendOnly [toActivate (some (and [isSource, isArtifact]))]]
theorem okSteelswarmOperatorMana : Action.check (DRS.ofList []) steelswarmOperatorMana = [] := by decide

def blastOfGenius : Spelled := spelled <| .singleFaced
  { characteristics :=
    { name := "Blast of Genius", cost := some [generic 4, pip .blue, pip .red], types := [.sorcery],
      text :=
        [ spell (sequentially
            [ choose (target isAnyTarget),
              draw (.lit 3),
              discard (a (cardIn yourHand)),
              instruct this_ (dealDamage (statOf (.stat .manaValue) (theVerbed (.action "discard") .one (some .card))) thatJoin .noncombatOnly) ]) ] } }

def riddleOfLightning : Spelled := spelled <| .singleFaced
  { characteristics :=
    { name := "Riddle of Lightning", cost := some [generic 3, pip .red, pip .red], types := [.instant],
      text :=
        [ spell (sequentially
            [ choose (target isAnyTarget),
              scry (.lit 3),
              reveal (topSlice (.lit 1)),
              instruct this_
                  (dealDamage (statOf (.stat .manaValue) (that card)) thatJoin
                     .noncombatOnly) ]) ] } }

def unstableFrontier : Spelled := spelled <| .singleFaced
  { characteristics :=
    { name := "Unstable Frontier", types := [.land],
      text :=
        [ activated tapSymbol (addMana (.lit 1) (manaOf [[.colorless]])),
          activated tapSymbol
            (establishThroughout (qualityChange (target (and [isLand, youControl])) .sets
                (chosenQuality
                   (ofYourChoice (.subtype .land) (some basicTypesOnly)))) untilEndOfTurn) ] } }

/-- Sanctum Prelate -/
def sanctumPrelate : Spelled := spelled <| .singleFaced
  { characteristics :=
    { name := "Sanctum Prelate", cost := some [generic 1, pip .white, pip .white],
      types := [.creature], subtypes := [human, cleric],
      text :=
        [ static (entersChoosing thisCreature .number),
          static
              (cant any
                 (cast (allOf (and [isSpell, non isCreature, hasManaValue .eq chosenNumber])))) ],
      power := stat 2, toughness := stat 2 } }

def abruptDecay : Spelled := spelled <| .singleFaced
  { characteristics :=
    { name := "Abrupt Decay", cost := some [pip .black, pip .green], types := [.instant],
      text :=
        [ static (cant any (counter thisSpell)),
          spell
              (destroy
                 (target (and [isPermanent, non isLand, hasManaValue .atMost (.lit 3)]))) ] } }

/-- Burn, Burn, Tree and Fern -/
def burnBurnTreeAndFern : Spelled := spelled <| .singleFaced
  { characteristics :=
    { name := "Burn, Burn, Tree and Fern", cost := some [generic 3, pip .red],
      types := [.enchantment], subtypes := [saga],
      text :=
        [ when (chapterMark [1])
            (instruct this_
               (dealDamage (.lit 6) (target (and [isCreature, controlledBy anOpponent]))
                  .noncombatOnly)),
          when (chapterMark [2])
            (destroy (target (and [isArtifact, controlledBy anOpponent]))),
          when (chapterMark [3, 4]) (addMana (.lit 1) (manaOf [[.of .red]])) ] }
              }

def theFlux : Spelled := spelled <| .singleFaced
  { characteristics :=
    { name := "The Flux", cost := some [generic 2, pip .red, pip .red], types := [.enchantment],
      subtypes := [saga],
      text :=
        [ when (chapterMark [1])
            (instruct this_
               (dealDamage (.lit 4) (target (and [isCreature, controlledBy anOpponent]))
                  .noncombatOnly)),
          when (chapterMark [2, 3, 4, 5])
            (sequentially
              [ exile (topSlice (.lit 1)),
                establishThroughout (can you (play (that card))) thisTurn ]),
          when (chapterMark [6]) (addMana (.lit 6) (manaOf [[.of .red]])) ] } }

def dragonstormGlobe : Spelled := spelled <| .singleFaced
  { characteristics :=
    { name := "Dragonstorm Globe", cost := some [generic 3], types := [.artifact],
      text :=
        [ static (entersWithAdditionalCounters
            (each (and [hasSubtype dragon, youControl]))
            (.lit 1) p1p1Counter),
          activated tapSymbol (addMana (.lit 1) (ofAnyColor .sameColor)) ] } }

def sageOfFables : Spelled := spelled <| .singleFaced
  { characteristics :=
    { name := "Sage of Fables", cost := some [generic 2, pip .blue], types := [.creature],
      subtypes := [merfolk, wizard],
      text :=
        [ static (entersWithAdditionalCounters
            (each (and [isCreature, hasSubtype wizard, youControl,
                         otherThan thisCreature]))
            (.lit 1) p1p1Counter),
          activated
            (compound [mana [generic 2],
              perform (removeCounters (exactly 1) (some (printedKind p1p1Counter))
                         (a (and [isCreature, youControl])) .many true)])
            (draw (.lit 1)) ],
      power := stat 2, toughness := stat 2 } }

/-- Gaddock Teeg -/
def gaddockTeeg : Spelled := spelled <| .singleFaced
  { characteristics :=
    { name := "Gaddock Teeg", cost := some [pip .green, pip .white], supertypes := [.legendary],
      types := [.creature], subtypes := [kithkin, advisor],
      text :=
        [ static
            (cant any
               (cast (allOf (and [isSpell, non isCreature, hasManaValue .atLeast (.lit 4)])))),
          static (cant any (cast (allOf (and [isSpell, non isCreature, manaCostHas .variable])))) ],
      power := stat 2, toughness := stat 2 } }

def shiningShoal : Spelled := spelled <| .singleFaced
  { characteristics :=
    { name := "Shining Shoal", cost := some [.variable, pip .white, pip .white], types := [.instant],
      subtypes := [arcane],
      text :=
        [ static (altCost this_
            (some (perform (exile
              (a (and [hasColor .white, hasManaValue .eq (letter .x),
                        in_ yourHand])))))),
          spell (establishThroughout (damageRule .any (dealtBy (aYourChoice isSource))
                (toRecipient (youAnd (allOf (and [isCreature, youControl]))))
                (redirect (theNext (letter .x)) (target isAnyTarget)) .repeatedly) thisTurn) ] } }

def disruptingShoal : Spelled := spelled <| .singleFaced
  { characteristics :=
    { name := "Disrupting Shoal", cost := some [.variable, pip .blue, pip .blue], types := [.instant],
      subtypes := [arcane],
      text :=
        [ static (altCost this_
            (some (perform (exile
              (a (and [hasColor .blue, hasManaValue .eq (letter .x),
                        in_ yourHand])))))),
          spell (doOnlyIf (counter (target isSpell))
              (compareAmt (statOf (.stat .manaValue) it) .eq (letter .x))) ] } }

def blazingShoal : Spelled := spelled <| .singleFaced
  { characteristics :=
    { name := "Blazing Shoal", cost := some [.variable, pip .red, pip .red], types := [.instant],
      subtypes := [arcane],
      text :=
        [ static (altCost this_
            (some (perform (exile
              (a (and [hasColor .red, hasManaValue .eq (letter .x),
                        in_ yourHand])))))),
          spell (get (target isCreature) (up (letter .x)) (up (.lit 0)) (some untilEndOfTurn)) ] } }

def sickeningShoal : Spelled := spelled <| .singleFaced
  { characteristics :=
    { name := "Sickening Shoal", cost := some [.variable, pip .black, pip .black], types := [.instant],
      subtypes := [arcane],
      text :=
        [ static (altCost this_
            (some (perform (exile
              (a (and [hasColor .black, hasManaValue .eq (letter .x),
                        in_ yourHand])))))),
          spell (get (target isCreature) (down (letter .x)) (down (letter .x)) (some untilEndOfTurn))
                ] } }

def nourishingShoal : Spelled := spelled <| .singleFaced
  { characteristics :=
    { name := "Nourishing Shoal", cost := some [.variable, pip .green, pip .green], types := [.instant],
      subtypes := [arcane],
      text :=
        [ static (altCost this_
            (some (perform (exile
              (a (and [hasColor .green, hasManaValue .eq (letter .x),
                        in_ yourHand])))))),
          spell (gainLife (letter .x)) ] } }

def spellSnare : Spelled := spelled <| .singleFaced
  { characteristics :=
    { name := "Spell Snare", cost := some [pip .blue], types := [.instant],
      text :=
        [ spell (counter (target (and [isSpell, hasManaValue .eq (.lit 2)]))) ] } }

def isolate : Spelled := spelled <| .singleFaced
  { characteristics :=
    { name := "Isolate", cost := some [pip .white], types := [.instant],
      text :=
        [ spell (exile (target (and [isPermanent, hasManaValue .eq (.lit 1)]))) ] } }

def disembowel : Spelled := spelled <| .singleFaced
  { characteristics :=
    { name := "Disembowel", cost := some [.variable, pip .black], types := [.instant],
      text :=
        [ spell (destroy (target (and [isCreature, hasManaValue .eq (letter .x)]))) ] } }

def repeal : Spelled := spelled <| .singleFaced
  { characteristics :=
    { name := "Repeal", cost := some [.variable, pip .blue], types := [.instant],
      text :=
        [ spell (sequentially
            [ putInto (target (and [non isLand, isPermanent, hasManaValue .eq (letter .x)]))
                hand,
              draw (.lit 1) ]) ] } }

def entrancingMelody : Spelled := spelled <| .singleFaced
  { characteristics :=
    { name := "Entrancing Melody", cost := some [.variable, pip .blue, pip .blue], types := [.sorcery],
      text :=
        [ spell (establish
              (controlGrant you
                (target (and [isCreature, hasManaValue .eq (letter .x)])))) ] } }

def ratchetBomb : Spelled := spelled <| .singleFaced
  { characteristics :=
    { name := "Ratchet Bomb", cost := some [generic 2], types := [.artifact],
      text :=
        [ activated tapSymbol (putCounters (.lit 1) (printedKind chargeCounter) thisArtifact .many),
          activated (compound [tapSymbol, perform (sacrifice thisArtifact)])
            (destroy (each (and [non isLand, isPermanent,
              hasManaValue .eq (countersOn chargeCounter thisArtifact)]))) ] } }

def solGrail : Spelled := spelled <| .singleFaced
  { characteristics :=
    { name := "Sol Grail", cost := some [generic 3], types := [.artifact],
      text :=
        [ static (entersChoosing thisArtifact .color),
          activated tapSymbol (addMana (.lit 1) ofTheChosenColor) ] } }

def unchartedHaven : Spelled := spelled <| .singleFaced
  { characteristics :=
    { name := "Uncharted Haven", types := [.land],
      text :=
        [ static (entersTapped thisLand),
          static (entersChoosing thisLand .color),
          activated tapSymbol (addMana (.lit 1) ofTheChosenColor) ] } }

def mirageMesa : Spelled := spelled <| .singleFaced
  { characteristics :=
    { name := "Mirage Mesa", types := [.land], subtypes := [desert],
      text :=
        [ static (entersTapped thisLand),
          static (entersChoosing thisLand .color),
          activated tapSymbol (addMana (.lit 1) ofTheChosenColor) ] } }

def crossroadsVillage : Spelled := spelled <| .singleFaced
  { characteristics :=
    { name := "Crossroads Village", types := [.land], subtypes := [town],
      text :=
        [ static (entersTapped thisLand),
          static (entersChoosing thisLand .color),
          activated tapSymbol (addMana (.lit 1) ofTheChosenColor) ] } }

/-- A Thriving land: enters tapped choosing a color other than its own, taps for either. -/
def thrivingLand (name : String) (own : Color) : Card := .singleFaced
  { characteristics :=
    { name, types := [.land],
      text :=
        [ static (entersTapped thisLand),
          static (entersChoosingFrom thisLand .color (colorOtherThan own)),
          activated tapSymbol (addMana (.lit 1) (ofTheChosenColor (some [.of own]))) ] } }

def thrivingBluff : Spelled := spelled <| (thrivingLand "Thriving Bluff" .red)
def thrivingGrove : Spelled := spelled <| (thrivingLand "Thriving Grove" .green)
def thrivingHeath : Spelled := spelled <| (thrivingLand "Thriving Heath" .white)
def thrivingIsle : Spelled := spelled <| (thrivingLand "Thriving Isle" .blue)
def thrivingMoor : Spelled := spelled <| (thrivingLand "Thriving Moor" .black)

def secretsOfTheDead : Spelled := spelled <| .singleFaced
  { characteristics :=
    { name := "Secrets of the Dead", cost := some [generic 2, pip .blue], types := [.enchantment],
      text :=
        [ whenever (does you (cast (a
          (and [isSpell, castFrom yourGraveyard]))))
            (draw (.lit 1)) ] } }

def coalStoker : Spelled := spelled <| .singleFaced
  { characteristics :=
    { name := "Coal Stoker", cost := some [generic 3, pip .red], types := [.creature],
      subtypes := [elemental],
      text :=
        [ triggeredIf (does thisCreature enter) (matches_ it (castFrom yourHand))
            (addMana (.lit 1) (manaOf [[.of .red, .of .red, .of .red]])) ],
      power := stat 3, toughness := stat 3 } }

def vegaTheWatcher : Spelled := spelled <| .singleFaced
  { characteristics :=
    { name := "Vega, the Watcher", cost := some [generic 1, pip .white, pip .blue],
      supertypes := [.legendary], types := [.creature],
      subtypes := [bird, spirit],
      text :=
        [ flying,
          whenever (does you (cast (a
            (and [isSpell, not
            (castFrom yourHand)]))))
            (draw (.lit 1)) ],
      power := stat 2, toughness := stat 2 } }

def adNauseam : Spelled := spelled <| .singleFaced
  { characteristics :=
    { name := "Ad Nauseam", cost := some [generic 3, pip .black, pip .black], types := [.instant],
      text :=
        [ spell (sequentially
            [ reveal (topSlice (.lit 1)),
              putInto (that card) hand,
              loseLife (statOf (.stat .manaValue) it),
              may (repeat_ anyNumberOfTimes) ]) ] } }

def nahiriLoyaltyRead : Predicate :=
  and [ isCreature, in_ yourGraveyard,
         hasManaValue .less (statOf (.stat .loyalty) this_) ]
theorem okNahiriLoyaltyRead : Predicate.checkAt .object (DRS.ofList []) nahiriLoyaltyRead = [] := by decide

/-- Caustic Bronco -/
def causticBroncoLoss : Action :=
  sequentially
    [ reveal (topSlice (.lit 1)),
      putInto (that card) hand,
      doOnlyIf (loseLife (statOf (.stat .manaValue) it))
        (not (matches_ thisCreature (hasDesignation saddled)))
        (some (instruct eachOpponent (loseLife thatMuch))) ]
theorem okCausticBroncoLoss : Action.check (DRS.ofList []) causticBroncoLoss = [] := by decide

/-- Dark Fortress -/
def darkFortressMana : Ability :=
  activatedOnlyIf tapSymbol (addMana (.lit 1) (manaOf [[.of .black], [.of .red]]))
    (or [ happened (putOntoBattlefield thisLand) any soFarThisTurn,
           exists_ (and [isLand, hasSupertype .basic, youControl]) ])
theorem okDarkFortressMana : Ability.check (DRS.ofList []) darkFortressMana = [] := by decide

def branchloftPathway : Spelled := spelled <| .modalDfc
  { characteristics :=
    { name := "Branchloft Pathway", types := [.land],
      text := [activated tapSymbol (addMana (.lit 1) (manaOf [[.of .green]]))] }
          }
  { characteristics :=
    { name := "Boulderloft Pathway", types := [.land],
      text := [activated tapSymbol (addMana (.lit 1) (manaOf [[.of .white]]))] }
          }

/-- Nykthos, Shrine to Nyx -/
def nykthosShrineToNyx : Spelled := spelled <| .singleFaced
  { characteristics :=
    { name := "Nykthos, Shrine to Nyx", supertypes := [.legendary], types := [.land],
      text :=
        [ activated tapSymbol (addMana (.lit 1) (manaOf [[.colorless]])),
          activated (compound [mana [generic 2], tapSymbol])
            (sequentially
              [ choose (a (quality .color)),
                addMana (devotion you thatColor) ofTheChosenColor ])
                    ] } }

/-- Karametra's Acolyte -/
def karametrasAcolyte : Ability :=
  activated tapSymbol (addMana (devotion you (color .green)) (manaOf [[.of .green]]))
theorem okKarametrasAcolyte : Ability.check (DRS.ofList []) karametrasAcolyte = [] := by decide

/-- Investigator's Journal -/
def investigatorsJournal : Spelled := spelled <| .singleFaced
  { characteristics :=
    { name := "Investigator's Journal", cost := some [generic 2], types := [.artifact],
      subtypes := [book, clue],
      text :=
        [ static (entersWithCounters thisArtifact greatestCreaturesAPlayerControls suspectCounter),
          activated
            (compound [mana [generic 2], tapSymbol,
              perform (removeCounters (exactly 1) (some (printedKind suspectCounter)) thisArtifact .many true)])
            (draw (.lit 1)),
          activated (compound [mana [generic 2], perform (sacrifice thisArtifact)])
            (draw (.lit 1)) ] } }

/-- Engineered Explosives (its sunburst [CR#702.44a] written out) -/
def engineeredExplosives : Spelled := spelled <| .singleFaced
  { characteristics :=
    { name := "Engineered Explosives", cost := some [.variable], types := [.artifact],
      text :=
        [ static (entersWithCounters thisArtifact (colorsSpentToCast thisArtifact) chargeCounter),
          activated (compound [mana [generic 2], perform (sacrifice thisArtifact)])
            (destroy (each (and [isPermanent, non isLand,
              hasManaValue .eq (countersOn chargeCounter thisArtifact)]))) ] } }

/-- Radiant Flames (converge is an ability word [CR#207.2c]) -/
def radiantFlames : Spelled := spelled <| .singleFaced
  { characteristics :=
    { name := "Radiant Flames", cost := some [generic 2, pip .red], types := [.sorcery],
      text := [spell (instruct this_ (dealDamage (colorsSpentToCast this_) (each isCreature) .noncombatOnly))] } }

/-- Birthing Pod -/
def birthingPodSearch : Ability :=
  activated
    (compound [mana [generic 1, phyrexianPip .green], tapSymbol, perform (sacrifice (a isCreature))])
    (searchLibraryFor (exactly 1)
      (and [isCreature,
             hasManaValue .eq (plus (.lit 1) (statOf (.stat .manaValue)
                   (theVerbed (.action "sacrifice") .one (some creature))))]))
theorem okBirthingPodSearch : Ability.check (DRS.ofList []) birthingPodSearch = [] := by decide

/-- Hibernation's End -/
def hibernationsEndTrigger : Ability :=
  whenever
      (does you (pay anyCost .once .paid (some thisEnchantment) (some cumulativeUpkeep)))
    (may
      (sequentially
        [ searchLibraryFor (exactly 1)
            (and [isCreature, hasManaValue .eq (countersOn ageCounter thisEnchantment)]),
          putOntoBattlefield (that card),
          shuffle ]))
theorem okHibernationsEndTrigger : Ability.check (DRS.ofList []) hibernationsEndTrigger = [] := by decide

/-- Latchkey Faerie -/
def latchkeyFaerie : Spelled := spelled <| .singleFaced
  { characteristics :=
    { name := "Latchkey Faerie", cost := some [generic 3, pip .blue], types := [.creature],
      subtypes := [faerie, rogue],
      text :=
        [ flying,
          prowl (mana [generic 2, pip .blue]),
          triggeredIf (does thisCreature enter) (keywordCostPaid prowl
                                                           thisCreature)
            (draw (.lit 1)) ],
      power := stat 3, toughness := stat 1 } }

/-- Bloom Tender -/
def bloomTenderMana : Ability :=
  abilityWord "vivid"
    (activated tapSymbol
      (doForEachKind .color .color (addMana (.lit 1) ofTheChosenColor)
         (some (allOf (and [isPermanent, youControl])))))
theorem okBloomTenderMana : Ability.check (DRS.ofList []) bloomTenderMana = [] := by decide

/-- Tarnation Vista -/
def tarnationVistaMana : Ability :=
  activated (compound [mana [generic 1], tapSymbol])
    (doForEachKind .color .color (addMana (.lit 1) ofTheChosenColor)
       (some (allOf (and [isPermanent, isMonocolored, youControl]))))
theorem okTarnationVistaMana : Ability.check (DRS.ofList []) tarnationVistaMana = [] := by decide

/-- Military Intelligence -/
def militaryIntelligence : Spelled := spelled <| .singleFaced
  { characteristics :=
    { name := "Military Intelligence", cost := some [generic 1, pip .blue], types := [.enchantment],
      text :=
        [ whenever (does you (attackWith (counted (atLeast 2) isCreature))) (draw (.lit 1)) ] } }

/-- Jem Lightfoote, Sky Explorer -/
def jemLightfooteSkyExplorer : Spelled := spelled <| .singleFaced
  { characteristics :=
    { name := "Jem Lightfoote, Sky Explorer", cost := some [generic 2, pip .white, pip .blue],
      supertypes := [.legendary], types := [.creature],
      subtypes := [human, scout],
      text :=
        [ flying,
          vigilance,
          triggeredIf (beginningOfYour .endStep)
            (not
               (happened (cast (a isSpell) (from_ := some yourHand))
                  you soFarThisTurn))
            (draw (.lit 1)) ],
      power := stat 3, toughness := stat 3 } }

/-- Gnarlback Rhino -/
def gnarlbackRhino : Spelled := spelled <| .singleFaced
  { characteristics :=
    { name := "Gnarlback Rhino", cost := some [generic 2, pip .green, pip .green], types := [.creature],
      subtypes := [rhino],
      text :=
        [ trample,
          whenever (does you (cast (a
            (and [isSpell, targeting thisCreature
            .someTarget]))))
            (draw (.lit 1)) ],
      power := stat 4, toughness := stat 4 } }

/-- Prismari Pianist -/
def prismariPianist : Spelled := spelled <| .singleFaced
  { characteristics :=
    { name := "Prismari Pianist", cost := some [generic 1, pip .red, pip .red], types := [.creature],
      subtypes := [djinn, bard],
      text :=
        [ whenever (does you (cast (a
          (and [isInstantOrSorcery, isSpell]))))
            (replace
              (createToken (creatureToken 1 1 [.blue, .red] [elemental]))
              (doIf (compareAmt (statOf (.stat .manaValue) (that .spell)) .atLeast (.lit 5))
                (create (.lit 3) asThose []))) ],
      power := stat 2, toughness := stat 1 } }

/-- Collected Company -/
def collectedCompany : Spelled := spelled <| .singleFaced
  { characteristics :=
    { name := "Collected Company", cost := some [generic 3, pip .green], types := [.instant],
      text :=
        [ spell (sequentially
            [ lookAt (topSlice (.lit 6)),
              putOntoBattlefield (fromAmong (upTo 2)
                                    (and [isCreature, hasManaValue .atMost (.lit 3)]) them),
              putInto (theRest .object) onBottom (riders := [(inOrder .anyOrder)]) ]) ] } }

/-- Soldevi Adnate -/
def soldeviAdnate : Spelled := spelled <| .singleFaced
  { characteristics :=
    { name := "Soldevi Adnate", cost := some [generic 1, pip .black], types := [.creature],
      subtypes := [human, cleric],
      text :=
        [ activated
            (compound [tapSymbol,
              perform (sacrifice (a (and [isCreature, or [hasColor .black, isArtifact]])))])
            (addMana (statOf (.stat .manaValue) (theVerbed (.action "sacrifice") .one))
              (manaOf [[.of .black]])) ],
      power := stat 1, toughness := stat 2 } }

/-- Delivery Moogle -/
def deliveryMoogle : Spelled := spelled <| .singleFaced
  { characteristics :=
    { name := "Delivery Moogle", cost := some [generic 3, pip .white], types := [.creature],
      subtypes := [moogle],
      text :=
        [ flying,
          when (does thisCreature enter)
            (sequentially
              [ searchLibraryOrGraveyard (and [isArtifact, hasManaValue .atMost (.lit 2)]),
                reveal it,
                putInto it hand,
                doIf (happenedThisWay (searchLibraryFor anyNumber (and []))
                        you) shuffle ]) ],
      power := stat 3, toughness := stat 2 } }

/-- Heart of Yavimaya -/
def heartOfYavimaya : Spelled := spelled <| .singleFaced
  { characteristics :=
    { name := "Heart of Yavimaya", types := [.land],
      text :=
        [ static
            (instead (would any (putOntoBattlefield this_))
               (doIfDone (sacrifice (a (and [isLand, hasSubtype forest])))
              (some (putOntoBattlefield this_)) (some (putInto this_ graveyard)))),
          activated tapSymbol (addMana (.lit 1) (manaOf [[.of .green]])),
          activated tapSymbol
            (get (target isCreature) (up (.lit 1)) (up (.lit 1)) (some untilEndOfTurn)) ] } }

/-- Mox Diamond -/
def moxDiamond : Spelled := spelled <| .singleFaced
  { characteristics :=
    { name := "Mox Diamond", cost := some [], types := [.artifact],
      text :=
        [ static
            (instead (would any (putOntoBattlefield this_))
               (may (discard (a (and [isLand, in_ yourHand])))
              (some (putOntoBattlefield this_)) (some (putInto this_ graveyard)))),
          activated tapSymbol (addMana (.lit 1) (ofAnyColor .sameColor)) ] } }

/-- Up the Beanstalk -/
def upTheBeanstalk : Spelled := spelled <| .singleFaced
  { characteristics :=
    { name := "Up the Beanstalk", cost := some [generic 1, pip .green], types := [.enchantment],
      text :=
        [ triggeredJoined (does this_ enter)
            [joinedHead (does you (cast (a
              (and [isSpell, hasManaValue .atLeast (.lit 5)]))))]
            (draw (.lit 1)) ] } }

/-- Sheltered Valley -/
def shelteredValley : Spelled := spelled <| .singleFaced
  { characteristics :=
    { name := "Sheltered Valley", types := [.land],
      text :=
        [ static (instead (would any (putOntoBattlefield this_)) (sequentially
              [ sacrifice
                  (each (and [isPermanent, otherThan thisLand, named (printedName "Sheltered Valley"),
                               youControl])),
                putOntoBattlefield this_ ])),
          triggeredIf (beginningOfYour .upkeep)
            (compareAmt (countOf (and [isLand, youControl])) .atMost (.lit 3))
            (gainLife (.lit 1)),
          activated tapSymbol (addMana (.lit 1) (manaOf [[.colorless]])) ] } }

/-- Soul Shatter -/
def soulShatter : Action :=
  instruct eachOpponent (sacrifice
    (a (and [or [isCreature, isPlaneswalker],
              superlative .max (.stat .manaValue)
                (and [or [isCreature, isPlaneswalker], controlledBy they])])))
theorem okSoulShatter : Action.check (DRS.ofList []) soulShatter = [] := by decide

/-- Padeem, Consul of Innovation -/
def padeemConsulOfInnovation : Ability :=
  triggeredIf (beginningOfYour .upkeep)
    (matches_
      (the (and [isArtifact,
                  superlative .max (.stat .manaValue) (and [isArtifact, in_ battlefield])]))
      youControl)
    (draw (.lit 1))
theorem okPadeemConsulOfInnovation : Ability.check (DRS.ofList []) padeemConsulOfInnovation = [] := by decide

/-- Talion, the Kindly Lord -/
def talionTheKindlyLord : Spelled := spelled <| .singleFaced
  { characteristics :=
    { name := "Talion, the Kindly Lord", cost := some [generic 2, pip .blue, pip .black],
      supertypes := [.legendary], types := [.creature],
      subtypes := [faerie, noble],
      text :=
        [ flying,
          static (entersChoosing thisCreature .number),
          whenever
            (does anOpponent (cast (a (and [isSpell,
                        compare .eq chosenNumber (axes := [.stat .manaValue, .stat .power, .stat
                          .toughness])]))))
            (sequentially [instruct (that player) (loseLife (.lit 2)), draw (.lit 1)]) ],
      power := stat 3, toughness := stat 4 } }

/-- Braid of Fire -/
def braidOfFire : Spelled := spelled <| .singleFaced
  { characteristics :=
    { name := "Braid of Fire", cost := some [generic 1, pip .red], types := [.enchantment],
      text := [cumulativeUpkeep (perform (addMana (.lit 1) (manaOf [[.of .red]])))] } }

/-- Delighted Halfling -/
def delightedHalflingMana : Ability :=
  activated tapSymbol
    (addMana (.lit 1) (ofAnyColor .sameColor)
      [ whenSpentOn .affectsIt true (a (and [hasSupertype .legendary, isSpell]))
          (establish (cant any (counter (that .spell)))) ])
theorem okDelightedHalflingMana : Ability.check (DRS.ofList []) delightedHalflingMana = [] := by decide

/-- Boseiju, Who Shelters All -/
def boseijuMana : Ability :=
  activated (compound [tapSymbol, payLife 2])
    (addMana (.lit 1) (manaOf [[.colorless]])
      [ whenSpentOn .affectsIt false (a (and [isInstantOrSorcery, isSpell]))
          (establish (cant any (counter (that .spell)))) ])
theorem okBoseijuMana : Ability.check (DRS.ofList []) boseijuMana = [] := by decide

/-- Generator Servant -/
def generatorServant : Spelled := spelled <| .singleFaced
  { characteristics :=
    { name := "Generator Servant", cost := some [generic 1, pip .red], types := [.creature],
      subtypes := [elemental],
      text :=
        [ activated (compound [tapSymbol, perform (sacrifice thisCreature)])
            (addMana (.lit 1) (manaOf [[.colorless, .colorless]])
              [ whenSpentOn .affectsIt false (a (and [isCreature, isSpell]))
                  (gainHaste (resolvedPermanent (that .spell)) (some untilEndOfTurn)) ]) ],
      power := stat 2, toughness := stat 1 } }

/-- Animal Attendant -/
def animalAttendant : Spelled := spelled <| .singleFaced
  { characteristics :=
    { name := "Animal Attendant", cost := some [generic 1, pip .green], types := [.creature],
      subtypes := [human, citizen],
      text :=
        [ activated tapSymbol
            (addMana (.lit 1) (ofAnyColor .sameColor)
              [ whenSpentOn .affectsIt false
                  (a (and [non (hasSubtype human), isCreature, isSpell]))
                  (establish
                    (entersWithAdditionalCounters (resolvedPermanent (that .spell)) (.lit 1)
                       p1p1Counter)) ]) ],
      power := stat 2, toughness := stat 2 } }

/-- Thran Turbine -/
def thranTurbineMana : Action :=
  addMana (.lit 1) (manaOf [[.colorless, .colorless]]) [cantSpendOn [toCast isSpell]]
theorem okThranTurbineMana : Action.check (DRS.ofList []) thranTurbineMana = [] := by decide

/-- Su-Chi Cave Guard -/
def suChiCaveGuardDies : Ability :=
  when (does thisCreature die)
    (sequentially
      [ addMana (.lit 1)
          (manaOf [[.colorless, .colorless, .colorless, .colorless,
                   .colorless, .colorless, .colorless, .colorless]]),
        establishThroughout (manaRetention you thisMana) untilEndOfTurn ])
theorem okSuChiCaveGuardDies : Ability.check (DRS.ofList []) suChiCaveGuardDies = [] := by decide

/-- Omnath, Locus of Mana -/
def omnathLocusOfManaPersistence : StaticSpec := manaRetention you (unspent (some (.of .green)))
theorem okOmnathLocusOfManaPersistence :
    StaticSpec.check (DRS.ofList []) omnathLocusOfManaPersistence = [] := by decide

/-- Upwelling -/
def upwelling : Spelled := spelled <| .singleFaced
  { characteristics :=
    { name := "Upwelling", cost := some [generic 3, pip .green], types := [.enchantment],
      text := [static (manaRetention eachPlayer (unspent none))] } }

/-- Mana Flare -/
def manaFlare : Spelled := spelled <| .singleFaced
  { characteristics :=
    { name := "Mana Flare", cost := some [generic 2, pip .red], types := [.enchantment],
      text :=
        [ whenever (does (a isPlayer) (tapForMana (a isLand)))
            (instruct (that player) (addMana (.lit 1) (ofTypeProducedBy (that land))))
                ] } }

/-- Shimmerwilds Growth -/
def shimmerwildsGrowth : Spelled := spelled <| .singleFaced
  { characteristics :=
    { name := "Shimmerwilds Growth", cost := some [generic 1, pip .green], types := [.enchantment],
      subtypes := [aura],
      text :=
        [ enchant isLand,
          static (entersChoosing thisAura .color),
          static (qualityChange (enchanted land) .sets
            (chosenQuality (ofChosen .color))),
          whenever (does any (tapForMana (enchanted land)))
            (instruct (controllerOf (that land)) (addMana (.lit 1) ofTheChosenColor)) ] } }

/-- Gauntlet of Power -/
def gauntletOfPower : Spelled := spelled <| .singleFaced
  { characteristics :=
    { name := "Gauntlet of Power", cost := some [generic 5], types := [.artifact],
      text :=
        [ static (entersChoosing thisArtifact .color),
          static (getsPt (allOf (and [isCreature, ofChosen .color])) (up (.lit 1)) (up (.lit 1))),
          whenever
            (does any (tapForMana (a (and [isLand, hasSupertype .basic])) (some (ofColor thatColor))))
            (instruct (controllerOf (that land)) (addMana (.lit 1) ofTheChosenColor)) ] } }

/-- Chrome Mox -/
def chromeMoxMana : Ability :=
  activated tapSymbol
    (addMana (.lit 1) (ofAnyColorOf (the exiledWithThisArtifact)))
theorem okChromeMoxMana : Ability.check (DRS.ofList []) chromeMoxMana = [] := by decide

/-- Fellwar Stone -/
def fellwarStone : Spelled := spelled <| .singleFaced
  { characteristics :=
    { name := "Fellwar Stone", cost := some [generic 2], types := [.artifact],
      text :=
        [ activated tapSymbol
            (addMana (.lit 1)
              (ofTypeProducibleBy (a (and [isLand, controlledBy anOpponent])))) ] } }

/-- Ice Cauldron -/
def iceCauldronNotedMana : Ability :=
  activated
    (compound [tapSymbol,
      perform (removeCounters (exactly 1) (some (printedKind chargeCounter)) thisArtifact .many true)])
    (addMana (.lit 1) (ofTheLastNotedType thisArtifact)
      [spendOnly [toCast exiledWithThisArtifact]])
theorem okIceCauldronNotedMana : Ability.check (DRS.ofList []) iceCauldronNotedMana = [] := by decide

/-- Firemind Vessel -/
def firemindVesselMana : Ability :=
  activated tapSymbol (addMana (.lit 2) (ofAnyColor .distinctColors))
theorem okFiremindVesselMana : Ability.check (DRS.ofList []) firemindVesselMana = [] := by decide

/-- Goblin Clearcutter -/
def goblinClearcutterMana : Ability :=
  activated
    (compound [tapSymbol, perform (sacrifice (a (hasSubtype forest)))])
    (addMana (.lit 3) (ofAnyColorAmong [.red, .green]))
theorem okGoblinClearcutterMana : Ability.check (DRS.ofList []) goblinClearcutterMana = [] := by decide

/-- Tolaria -/
def tolaria : Spelled := spelled <| .singleFaced
  { characteristics :=
    { name := "Tolaria", supertypes := [.legendary], types := [.land],
      text :=
        [ activated tapSymbol (addMana (.lit 1) (manaOf [[.of .blue]])),
          activatedOnlyDuring (compound [tapSymbol])
            (establishThroughout (abilityLoss (target isCreature)
                [written banding, abilities bandsWithOtherAbilities]) untilEndOfTurn)
            (duringPart .upkeep none) ] } }

/-- Psychic Vortex -/
def psychicVortexUpkeep : Ability := cumulativeUpkeep (perform (draw (.lit 1)))
theorem okPsychicVortexUpkeep : Ability.check (DRS.ofList []) psychicVortexUpkeep = [] := by decide

/-- Varchild's War-Riders -/
def varchildsWarRidersUpkeep : Ability :=
  cumulativeUpkeep
    (perform (instruct anOpponent (createToken (creatureToken 1 1 [.red] [survivor]))))
theorem okVarchildsWarRidersUpkeep : Ability.check (DRS.ofList []) varchildsWarRidersUpkeep = [] := by decide

end Semantics.Cards
