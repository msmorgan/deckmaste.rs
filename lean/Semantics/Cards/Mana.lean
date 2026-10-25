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
  activated (compound [[generic 3, pip .red], sacrifice (a isArtifact)])
    (instruct this_
       (dealDamage
          (statOf (.stat .manaValue) (the (and [was (sacrifice which), isArtifact])))
        (target isAnyTarget)))
theorem okBoshIronGolem : Ability.check (DRS.ofList []) boshIronGolem = [] := by decide
def pyromancy : Ability :=
  activated (compound [[generic 3], discard (aAtRandom (cardIn yourHand))])
    (instruct thisEnchantment
      (dealDamage (statOf (.stat .manaValue) (the (and [was (discard which), isCard])))
        (target isAnyTarget)))
theorem okPyromancy : Ability.check (DRS.ofList []) pyromancy = [] := by decide
def luckyOffering : Action :=
  sequentially
    [ destroy (target (and [isArtifact, hasManaValue .atMost 3])),
      gainLife 3 ]
theorem okLuckyOffering : Action.check (DRS.ofList []) luckyOffering = [] := by decide
def overload : Action :=
  doOnlyIf (destroy (target isArtifact)) (compareAmt (statOf (.stat .manaValue) it) .atMost 2)
theorem okOverload : Action.check (DRS.ofList []) overload = [] := by decide
def austereCommand : Action :=
  chooseModes (exactly 2)
    [ destroy (allOf isArtifact),
      destroy (allOf isEnchantment),
      destroy (allOf (and [isCreature, hasManaValue .atMost 3])),
      destroy (allOf (and [isCreature, hasManaValue .atLeast 4])) ]
theorem okAustereCommand : Action.check (DRS.ofList []) austereCommand = [] := by decide

def bondersEnclave : Spelled := spelled <| singleFacedCard
  { name := "Bonders' Enclave", types := [.land],
    text :=
      [ activated tapSymbol (addMana 1 (manaOf [[.colorless]])),
        activatedOnlyIf (compound [[generic 3], tapSymbol]) (draw 1)
          (exists_ (and [isCreature, youControl,
                          hasPower .atLeast 4])) ] }

def workhorse : Spelled := spelled <| singleFacedCard
  { name := "Workhorse", cost := [generic 6], types := [.artifact, .creature],
    subtypes := [horse],
    text :=
      [ static (entersWithCounters thisCreature 4 p1p1Counter),
        activated
          (removeCounters (exactly 1) p1p1Counter thisCreature)
          (addMana 1 (manaOf [[.colorless]])) ],
    power := stat 0, toughness := stat 0 }

def labyrinthOfSkophos : Spelled := spelled <| singleFacedCard
  { name := "Labyrinth of Skophos", types := [.land],
    text :=
      [ activated tapSymbol (addMana 1 (manaOf [[.colorless]])),
        activated (compound [[generic 4], tapSymbol])
          (removeFromCombat (target (and [isCreature, or [isAttacking, isBlocking]]))) ] }

def acceleratedMutation : Spelled := spelled <| singleFacedCard
  { name := "Accelerated Mutation", cost := [generic 3, pip .green, pip .green],
    types := [.instant],
    text :=
      [ spell (sequentially
          [ get (target isCreature) (up (letter .x)) (up (letter .x)) untilEndOfTurn,
            define .x (aggregate .max (.stat .manaValue)
              (and [isPermanent, youControl])) ]) ] }

def cullingScales : Spelled := spelled <| singleFacedCard
  { name := "Culling Scales", cost := [generic 3], types := [.artifact],
    text :=
      [ at_ (beginningOfYour .upkeep)
          (destroy (target (and [isPermanent, non isLand,
            superlative .min (.stat .manaValue) (and [isPermanent, non isLand])]))) ] }

def deadeyeBrawler : Spelled := spelled <| singleFacedCard
  { name := "Deadeye Brawler", cost := [generic 2, pip .blue, pip .black],
    types := [.creature], subtypes := [human, pirate],
    text :=
      [ deathtouch,
        ascend,
        whenever (does thisCreature (dealDamage (to := a isPlayer) (kind := some .combat)))
          (if_ := is you (hasDesignation citysBlessing))
          (draw 1) ],
    power := stat 2, toughness := stat 4 }

def femerefEnchantress : Spelled := spelled <| singleFacedCard
  { name := "Femeref Enchantress", cost := [pip .green, pip .white], types := [.creature],
    subtypes := [human, druid],
    text :=
      [ whenever (does any (putInto (a isEnchantment) graveyard (from_ := battlefield)))
          (draw 1) ],
    power := stat 1, toughness := stat 2 }

def tocasiasWelcome : Spelled := spelled <| singleFacedCard
  { name := "Tocasia's Welcome", cost := [generic 2, pip .white], types := [.enchantment],
    text :=
      [ whenever
          (does (counted (orMore 1) (and [isCreature, youControl,
                                               hasManaValue .atMost 3])) enter)
          (draw 1) (limit := oncePerTurn) ] }

def duskLegionDuelist : Spelled := spelled <| singleFacedCard
  { name := "Dusk Legion Duelist", cost := [generic 1, pip .white], types := [.creature],
    subtypes := [vampire, soldier],
    text :=
      [ vigilance,
        whenever (countersPut p1p1Counter anyAmount thisCreature)
          (draw 1) (limit := oncePerTurn) ],
    power := stat 2, toughness := stat 2 }

def mishrasFactory : Spelled := spelled <| singleFacedCard
  { name := "Mishra's Factory", types := [.land],
    text :=
      [ activated tapSymbol (addMana 1 (manaOf [[.colorless]])),
        activated [generic 1]
          (establishThroughout (qualityChange thisLand .sets
              (bundle
                { characteristics :=
                  { types := [.artifact, .creature], subtypes := [assemblyWorker],
                    power := stat 2, toughness := stat 2 } }
                (some .land))) untilEndOfTurn),
        activated tapSymbol
          (get (target (and [isCreature, assemblyWorker]))
            (up 1) (up 1) untilEndOfTurn) ] }

/-- Mutavault -/
def mutavault : Spelled := spelled <| singleFacedCard
  { name := "Mutavault", types := [.land],
    text :=
      [ activated tapSymbol (addMana 1 (manaOf [[.colorless]])),
        activated [generic 1]
          (establishThroughout (qualityChange thisLand .sets
              (bundle
                { characteristics := { types := [.creature], power := stat 2, toughness := stat 2 },
                  qualities := [withEveryType .creature] }
                (some .land))) untilEndOfTurn) ] }

/-- Soulstone Sanctuary -/
def soulstoneSanctuary : Spelled := spelled <| singleFacedCard
  { name := "Soulstone Sanctuary", types := [.land],
    text :=
      [ activated tapSymbol (addMana 1 (manaOf [[.colorless]])),
        activated [generic 4]
          (establish
            (qualityChange thisLand .sets
              (bundle
                { characteristics :=
                  { types := [.creature], text := [vigilance],
                    power := stat 3, toughness := stat 3 },
                  qualities := [withEveryType .creature] }
                (some .land)))) ] }

def ragingRavine : Spelled := spelled <| singleFacedCard
  { name := "Raging Ravine", types := [.land],
    text :=
      [ static (entersTapped thisLand),
        activated tapSymbol (addMana 1 (manaOf [[.of .red], [.of .green]])),
        activated [generic 2, pip .red, pip .green]
          (establishThroughout (qualityChange thisLand .sets
              (bundle
                { characteristics :=
                  { colors := [.red, .green], types := [.creature],
                    subtypes := [elemental],
                    text :=
                      [ whenever (does thisCreature attack)
                          (putCounters 1 p1p1Counter thisCreature) ],
                    power := stat 3, toughness := stat 3 } }
                (some .land))) untilEndOfTurn) ] }

def saheeliFiligreeMaster : Spelled := spelled <| singleFacedCard
  { name := "Saheeli, Filigree Master", cost := [generic 2, pip .blue, pip .red],
    supertypes := [.legendary], types := [.planeswalker], subtypes := [saheeli],
    text :=
      [ activated (loyaltySymbol (up 1))
          (sequentially
            [ scry 1,
              may
                (tap (a (and [isArtifact, isUntapped, youControl])))
                (draw 1) ]),
        activated (loyaltySymbol (down 2))
          (sequentially
            [ createToken
                { characteristics :=
                  { types := [.artifact, .creature], subtypes := [thopter],
                    text := [flying], power := stat 1, toughness := stat 1 } } (count := 2),
              gainHaste them untilEndOfTurn ]),
        activated (loyaltySymbol (down 4))
          (getEmblem
            [ static (getsPt (allOf (and [isArtifact, isCreature, youControl]))
                (up 1) (up 1)),
              static (costShift (allOf (and [isArtifact, isSpell,
                  did you (cast which) thisObject])) (less 1)) ]) ],
    loyalty := stat 3 }

def manalith : Spelled := spelled <| singleFacedCard
  { name := "Manalith", cost := [generic 3], types := [.artifact],
    text := [activated tapSymbol (addMana 1 (ofAnyColor .sameColor))] }

def seethingSong : Spelled := spelled <| singleFacedCard
  { name := "Seething Song", cost := [generic 2, pip .red], types := [.instant],
    text :=
      [ spell (addMana 1 (manaOf [[.of .red, .of .red, .of .red, .of .red, .of .red]])) ] }

def ancientZiggurat : Spelled := spelled <| singleFacedCard
  { name := "Ancient Ziggurat", types := [.land],
    text :=
      [ activated tapSymbol
          (addMana 1 (ofAnyColor .sameColor)
            [spendOnly [toCast (and [isCreature, isSpell])]]) ] }

def mishrasWorkshop : Spelled := spelled <| singleFacedCard
  { name := "Mishra's Workshop", types := [.land],
    text :=
      [ activated tapSymbol
          (addMana 1 (manaOf [[.colorless, .colorless, .colorless]])
            [spendOnly [toCast (and [isArtifact, isSpell])]]) ] }

def boommobile : Ability :=
  when (does thisArtifact enter)
    (addMana 4 (ofAnyColor .sameColor) [spendOnly [toActivate none]])
theorem okBoommobile : Ability.check (DRS.ofList []) boommobile = [] := by decide

/-- Rosheen Meanderer -/
def rosheenMeanderer : Spelled := spelled <| singleFacedCard
  { name := "Rosheen Meanderer", cost := [generic 3, hybridPip .red .green],
    supertypes := [.legendary], types := [.creature],
    subtypes := [giant, shaman],
    text :=
      [ activated tapSymbol
          (addMana 1 (manaOf [[.colorless, .colorless, .colorless, .colorless]])
            [spendOnly [toPay (.containing .variable)]]) ],
    power := stat 4, toughness := stat 4 }

/-- Adarkar Unicorn -/
def adarkarUnicorn : Spelled := spelled <| singleFacedCard
  { name := "Adarkar Unicorn", cost := [generic 1, pip .white, pip .white],
    types := [.creature], subtypes := [unicorn],
    text :=
      [ activated tapSymbol
          (addMana 1 (manaOf [[.of .blue], [.colorless, .of .blue]])
            [spendOnly [toPay (.ofKeyword cumulativeUpkeep)]]) ],
    power := stat 2, toughness := stat 2 }

/-- Overgrown Zealot -/
def overgrownZealot : Spelled := spelled <| singleFacedCard
  { name := "Overgrown Zealot", cost := [generic 1, pip .green], types := [.creature],
    subtypes := [elf, druid],
    text :=
      [ activated tapSymbol (addMana 1 (ofAnyColor .sameColor)),
        activated tapSymbol
          (addMana 2 (ofAnyColor .sameColor)
            [spendOnly [toTurnFaceUp isPermanent]]) ],
    power := stat 0, toughness := stat 4 }

/-- Unblinking Observer -/
def unblinkingObserver : Spelled := spelled <| singleFacedCard
  { name := "Unblinking Observer", cost := [generic 1, pip .blue], types := [.creature],
    subtypes := [homunculus],
    text :=
      [ activated tapSymbol
          (addMana 1 (manaOf [[.of .blue]])
            [spendOnly [toPay (.ofKeyword disturb), toCast isInstantOrSorcery]]) ],
    power := stat 2, toughness := stat 1 }

/-- Qarsi Deceiver: "Spend this mana only to cast a face-down creature spell, pay a mana cost to
turn a manifested creature face up, or pay a morph cost." The turn-face-up purpose does not yet
say "a mana cost", so it also admits a manifested card's disguise cost [CR#701.40d], which the text
forbids: `planned/lean-v3-turn-face-up-spend-cost.md`. -/
def qarsiDeceiverMorphSpend : Ability :=
  activated tapSymbol
    (addMana 1 (manaOf [[.colorless]])
      [spendOnly
        [ toCast (and [isCreature, isFaceDown]),
          toTurnFaceUp (and [isCreature, was (manifest which) thisObject]),
          toPay (.ofKeyword morph) ]])
theorem okQarsiDeceiverMorphSpend : Ability.check (DRS.ofList []) qarsiDeceiverMorphSpend = [] := by decide

/-- Mercadian Bazaar -/
def mercadianBazaar : Spelled := spelled <| singleFacedCard
  { name := "Mercadian Bazaar", types := [.land],
    text :=
      [ static (entersTapped thisLand),
        activated tapSymbol (putCounters 1 storageCounter thisLand),
        activated
          (compound [tapSymbol,
            (removeCounters anyNumber storageCounter thisLand)])
          (addMana removedThisWay (manaOf [[.of .red]])) ] }

/-- Rootcoil Creeper -/
def rootcoilCreeperGraveyardMana : Ability :=
  activated tapSymbol
    (addMana 2 (ofAnyColor .sameColor)
      [spendOnly [toCast (and [isSpell,
          did any (cast which (from_ := yourGraveyard)) thisObject])]])
theorem okRootcoilCreeperGraveyardMana : Ability.check (DRS.ofList []) rootcoilCreeperGraveyardMana = [] := by
  decide

/-- Black Mana Battery -/
def blackManaBattery : Spelled := spelled <| singleFacedCard
  { name := "Black Mana Battery", cost := [generic 4], types := [.artifact],
    text :=
      [ activated (compound [[generic 2], tapSymbol])
          (putCounters 1 chargeCounter thisArtifact),
        activated
          (compound [tapSymbol,
            (removeCounters anyNumber chargeCounter thisArtifact)])
          (sequentially
            [ addMana 1 (manaOf [[.of .black]]),
              addMana removedThisWay (manaOf [[.of .black]]) ]) ] }

/-- Elemental Resonance -/
def elementalResonance : Spelled := spelled <| singleFacedCard
  { name := "Elemental Resonance", cost := [generic 2, pip .green, pip .green],
    types := [.enchantment], subtypes := [aura],
    text :=
      [ enchant isPermanent,
        at_ (beginningOfYour .firstMain)
          (addMana 1 (matchingManaCostOf (enchanted permanent))) ] }

/-- Steelswarm Operator -/
def steelswarmOperatorMana : Action :=
  addMana 1 (manaOf [[.of .blue, .of .blue]])
    [spendOnly [toActivate (some (and [isSource, isArtifact]))]]
theorem okSteelswarmOperatorMana : Action.check (DRS.ofList []) steelswarmOperatorMana = [] := by decide

def blastOfGenius : Spelled := spelled <| singleFacedCard
  { name := "Blast of Genius", cost := [generic 4, pip .blue, pip .red], types := [.sorcery],
    text :=
      [ spell (sequentially
          [ choose (target isAnyTarget),
            draw 3,
            discard (a (cardIn yourHand)),
            instruct this_
              (dealDamage (statOf (.stat .manaValue) (the (and [was (discard which), isCard])))
                thatJoin) ]) ] }

def riddleOfLightning : Spelled := spelled <| singleFacedCard
  { name := "Riddle of Lightning", cost := [generic 3, pip .red, pip .red], types := [.instant],
    text :=
      [ spell (sequentially
          [ choose (target isAnyTarget),
            scry 3,
            reveal (topSlice 1),
            instruct this_
                (dealDamage (statOf (.stat .manaValue) (that card)) thatJoin) ]) ] }

def unstableFrontier : Spelled := spelled <| singleFacedCard
  { name := "Unstable Frontier", types := [.land],
    text :=
      [ activated tapSymbol (addMana 1 (manaOf [[.colorless]])),
        activated tapSymbol
          (establishThroughout (qualityChange (target (and [isLand, youControl])) .sets
              (chosenQuality
                 (ofYourChoice (.subtype .land) basicTypesOnly))) untilEndOfTurn) ] }

/-- Sanctum Prelate -/
def sanctumPrelate : Spelled := spelled <| singleFacedCard
  { name := "Sanctum Prelate", cost := [generic 1, pip .white, pip .white],
    types := [.creature], subtypes := [human, cleric],
    text :=
      [ static (entersChoosing thisCreature .number),
        static
            (cant any
               (cast (bare (and [isSpell, non isCreature, hasManaValue .eq chosenNumber])))) ],
    power := stat 2, toughness := stat 2 }

def abruptDecay : Spelled := spelled <| singleFacedCard
  { name := "Abrupt Decay", cost := [pip .black, pip .green], types := [.instant],
    text :=
      [ static (cant any (counter thisSpell)),
        spell
            (destroy
               (target (and [isPermanent, non isLand, hasManaValue .atMost 3]))) ] }

/-- Burn, Burn, Tree and Fern -/
def burnBurnTreeAndFern : Spelled := spelled <| singleFacedCard
  { name := "Burn, Burn, Tree and Fern", cost := [generic 3, pip .red],
    types := [.enchantment], subtypes := [saga],
    text :=
      [ when (chapterMark [1])
          (instruct this_
             (dealDamage 6 (target (and [isCreature, anOpponentControls])))),
        when (chapterMark [2])
          (destroy (target (and [isArtifact, anOpponentControls]))),
        when (chapterMark [3, 4]) (addMana 1 (manaOf [[.of .red]])) ] }

def theFlux : Spelled := spelled <| singleFacedCard
  { name := "The Flux", cost := [generic 2, pip .red, pip .red], types := [.enchantment],
    subtypes := [saga],
    text :=
      [ when (chapterMark [1])
          (instruct this_
             (dealDamage 4 (target (and [isCreature, anOpponentControls])))),
        when (chapterMark [2, 3, 4, 5])
          (sequentially
            [ exile (topSlice 1),
              establishThroughout (can you (play (that card))) thisTurn ]),
        when (chapterMark [6]) (addMana 6 (manaOf [[.of .red]])) ] }

def dragonstormGlobe : Spelled := spelled <| singleFacedCard
  { name := "Dragonstorm Globe", cost := [generic 3], types := [.artifact],
    text :=
      [ static (entersWithAdditionalCounters
          (each (and [dragon, youControl]))
          1 p1p1Counter),
        activated tapSymbol (addMana 1 (ofAnyColor .sameColor)) ] }

def sageOfFables : Spelled := spelled <| singleFacedCard
  { name := "Sage of Fables", cost := [generic 2, pip .blue], types := [.creature],
    subtypes := [merfolk, wizard],
    text :=
      [ static (entersWithAdditionalCounters
          (each (and [isCreature, wizard, youControl,
                       isOther]))
          1 p1p1Counter),
        activated
          (compound [[generic 2],
            (removeCounters (exactly 1) p1p1Counter
                       (a (and [isCreature, youControl])))])
          (draw 1) ],
    power := stat 2, toughness := stat 2 }

/-- Gaddock Teeg -/
def gaddockTeeg : Spelled := spelled <| singleFacedCard
  { name := "Gaddock Teeg", cost := [pip .green, pip .white], supertypes := [.legendary],
    types := [.creature], subtypes := [kithkin, advisor],
    text :=
      [ static
          (cant any
             (cast (bare (and [isSpell, non isCreature, hasManaValue .atLeast 4])))),
        static (cant any (cast (bare (and [isSpell, non isCreature, manaCostHas .variable])))) ],
    power := stat 2, toughness := stat 2 }

def shiningShoal : Spelled := spelled <| singleFacedCard
  { name := "Shining Shoal", cost := [.variable, pip .white, pip .white], types := [.instant],
    subtypes := [arcane],
    text :=
      [ static (altCost this_
          (exile
            (a (and [hasColor .white, hasManaValue .eq (letter .x),
                      in_ yourHand])))),
        spell (establishThroughout
              (replace
                (would (aYourChoice isSource)
                  (dealDamage anyAmount (andOr [you, allOf (and [isCreature, youControl])])))
                (instead (redirectDamage (theNext (letter .x)) (target isAnyTarget))))
              thisTurn) ] }

def disruptingShoal : Spelled := spelled <| singleFacedCard
  { name := "Disrupting Shoal", cost := [.variable, pip .blue, pip .blue], types := [.instant],
    subtypes := [arcane],
    text :=
      [ static (altCost this_
          (exile
            (a (and [hasColor .blue, hasManaValue .eq (letter .x),
                      in_ yourHand])))),
        spell (doOnlyIf (counter (target isSpell))
            (compareAmt (statOf (.stat .manaValue) it) .eq (letter .x))) ] }

def blazingShoal : Spelled := spelled <| singleFacedCard
  { name := "Blazing Shoal", cost := [.variable, pip .red, pip .red], types := [.instant],
    subtypes := [arcane],
    text :=
      [ static (altCost this_
          (exile
            (a (and [hasColor .red, hasManaValue .eq (letter .x),
                      in_ yourHand])))),
        spell (get (target isCreature) (up (letter .x)) (up 0) untilEndOfTurn) ] }

def sickeningShoal : Spelled := spelled <| singleFacedCard
  { name := "Sickening Shoal", cost := [.variable, pip .black, pip .black], types := [.instant],
    subtypes := [arcane],
    text :=
      [ static (altCost this_
          (exile
            (a (and [hasColor .black, hasManaValue .eq (letter .x),
                      in_ yourHand])))),
        spell (get (target isCreature) (down (letter .x)) (down (letter .x)) untilEndOfTurn)
              ] }

def nourishingShoal : Spelled := spelled <| singleFacedCard
  { name := "Nourishing Shoal", cost := [.variable, pip .green, pip .green], types := [.instant],
    subtypes := [arcane],
    text :=
      [ static (altCost this_
          (exile
            (a (and [hasColor .green, hasManaValue .eq (letter .x),
                      in_ yourHand])))),
        spell (gainLife (letter .x)) ] }

def spellSnare : Spelled := spelled <| singleFacedCard
  { name := "Spell Snare", cost := [pip .blue], types := [.instant],
    text :=
      [ spell (counter (target (and [isSpell, hasManaValue .eq 2]))) ] }

def isolate : Spelled := spelled <| singleFacedCard
  { name := "Isolate", cost := [pip .white], types := [.instant],
    text :=
      [ spell (exile (target (and [isPermanent, hasManaValue .eq 1]))) ] }

def disembowel : Spelled := spelled <| singleFacedCard
  { name := "Disembowel", cost := [.variable, pip .black], types := [.instant],
    text :=
      [ spell (destroy (target (and [isCreature, hasManaValue .eq (letter .x)]))) ] }

def repeal : Spelled := spelled <| singleFacedCard
  { name := "Repeal", cost := [.variable, pip .blue], types := [.instant],
    text :=
      [ spell (sequentially
          [ returnToHand (target (and [non isLand, isPermanent, hasManaValue .eq (letter .x)])),
            draw 1 ]) ] }

def entrancingMelody : Spelled := spelled <| singleFacedCard
  { name := "Entrancing Melody", cost := [.variable, pip .blue, pip .blue], types := [.sorcery],
    text :=
      [ spell (establish
            (controlGrant you
              (target (and [isCreature, hasManaValue .eq (letter .x)])))) ] }

def ratchetBomb : Spelled := spelled <| singleFacedCard
  { name := "Ratchet Bomb", cost := [generic 2], types := [.artifact],
    text :=
      [ activated tapSymbol (putCounters 1 chargeCounter thisArtifact),
        activated (compound [tapSymbol, sacrifice thisArtifact])
          (destroy (each (and [non isLand, isPermanent,
            hasManaValue .eq (countersOn chargeCounter thisArtifact)]))) ] }

def solGrail : Spelled := spelled <| singleFacedCard
  { name := "Sol Grail", cost := [generic 3], types := [.artifact],
    text :=
      [ static (entersChoosing thisArtifact .color),
        activated tapSymbol (addMana 1 ofTheChosenColor) ] }

def unchartedHaven : Spelled := spelled <| singleFacedCard
  { name := "Uncharted Haven", types := [.land],
    text :=
      [ static (entersTapped thisLand),
        static (entersChoosing thisLand .color),
        activated tapSymbol (addMana 1 ofTheChosenColor) ] }

def mirageMesa : Spelled := spelled <| singleFacedCard
  { name := "Mirage Mesa", types := [.land], subtypes := [desert],
    text :=
      [ static (entersTapped thisLand),
        static (entersChoosing thisLand .color),
        activated tapSymbol (addMana 1 ofTheChosenColor) ] }

def crossroadsVillage : Spelled := spelled <| singleFacedCard
  { name := "Crossroads Village", types := [.land], subtypes := [town],
    text :=
      [ static (entersTapped thisLand),
        static (entersChoosing thisLand .color),
        activated tapSymbol (addMana 1 ofTheChosenColor) ] }

/-- A Thriving land: enters tapped choosing a color other than its own, taps for either. -/
def thrivingLand (name : String) (own : Color) : Card := singleFacedCard
  { name, types := [.land],
    text :=
      [ static (entersTapped thisLand),
        static (entersChoosingFrom thisLand .color (colorOtherThan own)),
        activated tapSymbol (addMana 1 (ofTheChosenColor (some [.of own]))) ] }

def thrivingBluff : Spelled := spelled <| (thrivingLand "Thriving Bluff" .red)
def thrivingGrove : Spelled := spelled <| (thrivingLand "Thriving Grove" .green)
def thrivingHeath : Spelled := spelled <| (thrivingLand "Thriving Heath" .white)
def thrivingIsle : Spelled := spelled <| (thrivingLand "Thriving Isle" .blue)
def thrivingMoor : Spelled := spelled <| (thrivingLand "Thriving Moor" .black)

def secretsOfTheDead : Spelled := spelled <| singleFacedCard
  { name := "Secrets of the Dead", cost := [generic 2, pip .blue], types := [.enchantment],
    text :=
      [ whenever (does you (cast (a
        (and [isSpell, did any (cast which (from_ := yourGraveyard)) thisObject]))))
          (draw 1) ] }

def coalStoker : Spelled := spelled <| singleFacedCard
  { name := "Coal Stoker", cost := [generic 3, pip .red], types := [.creature],
    subtypes := [elemental],
    text :=
      [ when (does thisCreature enter)
          (if_ := is it (did you (cast which (from_ := yourHand)) thisObject))
          (addMana 1 (manaOf [[.of .red, .of .red, .of .red]])) ],
    power := stat 3, toughness := stat 3 }

def vegaTheWatcher : Spelled := spelled <| singleFacedCard
  { name := "Vega, the Watcher", cost := [generic 1, pip .white, pip .blue],
    supertypes := [.legendary], types := [.creature],
    subtypes := [bird, spirit],
    text :=
      [ flying,
        whenever (does you (cast (a
          (and [isSpell, not
          (did any (cast which (from_ := yourHand)) thisObject)]))))
          (draw 1) ],
    power := stat 2, toughness := stat 2 }

def adNauseam : Spelled := spelled <| singleFacedCard
  { name := "Ad Nauseam", cost := [generic 3, pip .black, pip .black], types := [.instant],
    text :=
      [ spell (sequentially
          [ reveal (topSlice 1),
            putInto (that card) hand,
            loseLife (statOf (.stat .manaValue) it),
            may (repeat_ anyNumberOfTimes) ]) ] }

/-- Nahiri, the Unforgiving's 0: "… card with mana value less than Nahiri's loyalty from your
graveyard". On the battlefield her loyalty is her loyalty-counter count [CR#306.5c], not her
printed number, and the bound reads it as the ability resolves. -/
def nahiriLoyaltyRead : Predicate :=
  and [ isCreature, in_ yourGraveyard,
         hasManaValue .less (statOf (.stat .loyalty) this_) ]
theorem okNahiriLoyaltyRead : Predicate.checkAt .object (DRS.ofList []) nahiriLoyaltyRead = [] := by decide

/-- Caustic Bronco -/
def causticBroncoLoss : Action :=
  sequentially
    [ reveal (topSlice 1),
      putInto (that card) hand,
      doOnlyIf (loseLife (statOf (.stat .manaValue) it))
        (not (is thisCreature (hasDesignation saddled)))
        (instruct eachOpponent (loseLife thatMuch)) ]
theorem okCausticBroncoLoss : Action.check (DRS.ofList []) causticBroncoLoss = [] := by decide

/-- Dark Fortress -/
def darkFortressMana : Ability :=
  activatedOnlyIf tapSymbol (addMana 1 (manaOf [[.of .black], [.of .red]]))
    (or [ did thisLand enter soFarThisTurn,
           exists_ (and [isLand, hasSupertype .basic, youControl]) ])
theorem okDarkFortressMana : Ability.check (DRS.ofList []) darkFortressMana = [] := by decide

def branchloftPathway : Spelled := spelled <| .modalDfc
  { characteristics :=
    { name := "Branchloft Pathway", types := [.land],
      text := [activated tapSymbol (addMana 1 (manaOf [[.of .green]]))] }
          }
  { characteristics :=
    { name := "Boulderloft Pathway", types := [.land],
      text := [activated tapSymbol (addMana 1 (manaOf [[.of .white]]))] }
          }

/-- Nykthos, Shrine to Nyx -/
def nykthosShrineToNyx : Spelled := spelled <| singleFacedCard
  { name := "Nykthos, Shrine to Nyx", supertypes := [.legendary], types := [.land],
    text :=
      [ activated tapSymbol (addMana 1 (manaOf [[.colorless]])),
        activated (compound [[generic 2], tapSymbol])
          (sequentially
            [ choose (a (quality .color)),
              addMana (devotion you thatColor) ofTheChosenColor ])
                  ] }

/-- Karametra's Acolyte -/
def karametrasAcolyte : Ability :=
  activated tapSymbol (addMana (devotion you (color .green)) (manaOf [[.of .green]]))
theorem okKarametrasAcolyte : Ability.check (DRS.ofList []) karametrasAcolyte = [] := by decide

/-- Investigator's Journal -/
def investigatorsJournal : Spelled := spelled <| singleFacedCard
  { name := "Investigator's Journal", cost := [generic 2], types := [.artifact],
    subtypes := [book, clue],
    text :=
      [ static (entersWithCounters thisArtifact greatestCreaturesAPlayerControls suspectCounter),
        activated
          (compound [[generic 2], tapSymbol,
            (removeCounters (exactly 1) suspectCounter thisArtifact)])
          (draw 1),
        activated (compound [[generic 2], sacrifice thisArtifact])
          (draw 1) ] }

/-- Engineered Explosives (its sunburst [CR#702.44a] written out) -/
def engineeredExplosives : Spelled := spelled <| singleFacedCard
  { name := "Engineered Explosives", cost := some [.variable], types := [.artifact],
    text :=
      [ static (entersWithCounters thisArtifact (colorsSpentToCast thisArtifact) chargeCounter),
        activated (compound [[generic 2], sacrifice thisArtifact])
          (destroy (each (and [isPermanent, non isLand,
            hasManaValue .eq (countersOn chargeCounter thisArtifact)]))) ] }

/-- Radiant Flames (converge is an ability word [CR#207.2c]) -/
def radiantFlames : Spelled := spelled <| singleFacedCard
  { name := "Radiant Flames", cost := [generic 2, pip .red], types := [.sorcery],
    text := [spell (instruct this_ (dealDamage (colorsSpentToCast this_) (each isCreature)))] }

/-- Birthing Pod -/
def birthingPodSearch : Ability :=
  activated
    (compound [[generic 1, phyrexianPip .green], tapSymbol, sacrifice (a isCreature)])
    (search yourLibrary
      (a (and [isCreature,
             hasManaValue .eq (plus 1 (statOf (.stat .manaValue)
                   (the (and [was (sacrifice which), isCreature]))))])))
theorem okBirthingPodSearch : Ability.check (DRS.ofList []) birthingPodSearch = [] := by decide

/-- Hibernation's End -/
def hibernationsEndTrigger : Ability :=
  whenever
      (does you (pay anyCost .once .paid (some thisEnchantment) (some cumulativeUpkeep)))
    (may
      (sequentially
        [ search yourLibrary
            (a (and [isCreature, hasManaValue .eq (countersOn ageCounter thisEnchantment)])),
          putOntoBattlefield (that card),
          shuffle ]))
theorem okHibernationsEndTrigger : Ability.check (DRS.ofList []) hibernationsEndTrigger = [] := by decide

/-- Latchkey Faerie -/
def latchkeyFaerie : Spelled := spelled <| singleFacedCard
  { name := "Latchkey Faerie", cost := [generic 3, pip .blue], types := [.creature],
    subtypes := [faerie, rogue],
    text :=
      [ flying,
        prowl [generic 2, pip .blue],
        when (does thisCreature enter) (if_ := some (keywordCostPaid prowl
                                                         thisCreature))
          (draw 1) ],
    power := stat 3, toughness := stat 1 }

/-- Bloom Tender -/
def bloomTenderMana : Ability :=
  vivid
    (activated tapSymbol
      (doForEachKind .color .color (addMana 1 ofTheChosenColor)
         (allOf (and [isPermanent, youControl]))))
theorem okBloomTenderMana : Ability.check (DRS.ofList []) bloomTenderMana = [] := by decide

/-- Tarnation Vista -/
def tarnationVistaMana : Ability :=
  activated (compound [[generic 1], tapSymbol])
    (doForEachKind .color .color (addMana 1 ofTheChosenColor)
       (allOf (and [isPermanent, isMonocolored, youControl])))
theorem okTarnationVistaMana : Ability.check (DRS.ofList []) tarnationVistaMana = [] := by decide

/-- Military Intelligence -/
def militaryIntelligence : Spelled := spelled <| singleFacedCard
  { name := "Military Intelligence", cost := [generic 1, pip .blue], types := [.enchantment],
    text :=
      [ whenever (does you (attackWith (counted (orMore 2) isCreature))) (draw 1) ] }

/-- Jem Lightfoote, Sky Explorer -/
def jemLightfooteSkyExplorer : Spelled := spelled <| singleFacedCard
  { name := "Jem Lightfoote, Sky Explorer", cost := [generic 2, pip .white, pip .blue],
    supertypes := [.legendary], types := [.creature],
    subtypes := [human, scout],
    text :=
      [ flying,
        vigilance,
        at_ (beginningOfYour .endStep)
          (if_ := some (not
             (did you (cast (a isSpell) (from_ := yourHand)) soFarThisTurn)))
          (draw 1) ],
    power := stat 3, toughness := stat 3 }

/-- Gnarlback Rhino -/
def gnarlbackRhino : Spelled := spelled <| singleFacedCard
  { name := "Gnarlback Rhino", cost := [generic 2, pip .green, pip .green], types := [.creature],
    subtypes := [rhino],
    text :=
      [ trample,
        whenever (does you (cast (a
          (and [isSpell, targeting thisCreature
          .someTarget]))))
          (draw 1) ],
    power := stat 4, toughness := stat 4 }

/-- Prismari Pianist -/
def prismariPianist : Spelled := spelled <| singleFacedCard
  { name := "Prismari Pianist", cost := [generic 1, pip .red, pip .red], types := [.creature],
    subtypes := [djinn, bard],
    text :=
      [ whenever (does you (cast (a
        (and [isInstantOrSorcery, isSpell]))))
          (replace
            (createToken (creatureToken 1 1 [.blue, .red] [elemental]))
            (doIf (compareAmt (statOf (.stat .manaValue) (that .spell)) .atLeast 5)
              (instead (create 3 asThose [])))) ],
    power := stat 2, toughness := stat 1 }

/-- Collected Company -/
def collectedCompany : Spelled := spelled <| singleFacedCard
  { name := "Collected Company", cost := [generic 3, pip .green], types := [.instant],
    text :=
      [ spell (sequentially
          [ lookAt (topSlice 6),
            putOntoBattlefield (fromAmong (upTo 2)
                                  (and [isCreature, hasManaValue .atMost 3]) them),
            putInto (theRest .object) onBottom (riders := [(inOrder .anyOrder)]) ]) ] }

/-- Soldevi Adnate -/
def soldeviAdnate : Spelled := spelled <| singleFacedCard
  { name := "Soldevi Adnate", cost := [generic 1, pip .black], types := [.creature],
    subtypes := [human, cleric],
    text :=
      [ activated
          (compound [tapSymbol,
            (sacrifice (a (and [isCreature, or [hasColor .black, isArtifact]])))])
          (addMana (statOf (.stat .manaValue) (the (and [was (sacrifice which), isCreature])))
            (manaOf [[.of .black]])) ],
    power := stat 1, toughness := stat 2 }

/-- Delivery Moogle -/
def deliveryMoogle : Spelled := spelled <| singleFacedCard
  { name := "Delivery Moogle", cost := [generic 3, pip .white], types := [.creature],
    subtypes := [moogle],
    text :=
      [ flying,
        when (does thisCreature enter)
          (sequentially
            [ searchLibraryOrGraveyard (a (and [isArtifact, hasManaValue .atMost 2])),
              reveal it,
              putInto it hand,
              doIf (did you (search yourLibrary) thisWay) shuffle ]) ],
    power := stat 3, toughness := stat 2 }

/-- Heart of Yavimaya -/
def heartOfYavimaya : Spelled := spelled <| singleFacedCard
  { name := "Heart of Yavimaya", types := [.land],
    text :=
      [ static
          (replace (would thisLand enter)
            (instead (doIfDone (sacrifice (a (and [isLand, forest])))
              (putOntoBattlefield thisLand) (putInto it graveyard)))),
        activated tapSymbol (addMana 1 (manaOf [[.of .green]])),
        activated tapSymbol
          (get (target isCreature) (up 1) (up 1) untilEndOfTurn) ] }

/-- Mox Diamond -/
def moxDiamond : Spelled := spelled <| singleFacedCard
  { name := "Mox Diamond", cost := some [], types := [.artifact],
    text :=
      [ static
          (replace (would thisArtifact enter)
            (instead (may (discard (a (and [isLand, in_ yourHand])))
              (some (putOntoBattlefield thisArtifact)) (some (putInto it graveyard))))),
        activated tapSymbol (addMana 1 (ofAnyColor .sameColor)) ] }

/-- Up the Beanstalk -/
def upTheBeanstalk : Spelled := spelled <| singleFacedCard
  { name := "Up the Beanstalk", cost := [generic 1, pip .green], types := [.enchantment],
    text :=
      [ when (does this_ enter)
          (joins := [joinedHead (does you (cast (a
            (and [isSpell, hasManaValue .atLeast 5]))))])
          (draw 1) ] }

/-- Sheltered Valley -/
def shelteredValley : Spelled := spelled <| singleFacedCard
  { name := "Sheltered Valley", types := [.land],
    text :=
      [ static
          (replace (would thisLand enter)
            (instead
              (sequentially
                [ sacrifice
                    (each
                      (and
                        [ isPermanent, isOther, named "Sheltered Valley",
                          youControl ])),
                  putOntoBattlefield thisLand ]))),
        at_ (beginningOfYour .upkeep)
          (if_ := compareAmt (countOf (and [isLand, youControl])) .atMost 3)
          (gainLife 1),
        activated tapSymbol (addMana 1 (manaOf [[.colorless]])) ] }

/-- Soul Shatter -/
def soulShatter : Action :=
  instruct eachOpponent (sacrifice
    (a (and [or [isCreature, isPlaneswalker],
              superlative .max (.stat .manaValue)
                (and [or [isCreature, isPlaneswalker], controlledBy they])])))
theorem okSoulShatter : Action.check (DRS.ofList []) soulShatter = [] := by decide

/-- Padeem, Consul of Innovation -/
def padeemConsulOfInnovation : Ability :=
  at_ (beginningOfYour .upkeep)
    (if_ := some (exists_ (and [isArtifact, youControl,
      superlative .max (.stat .manaValue) (and [isArtifact, in_ battlefield])])))
    (draw 1)
theorem okPadeemConsulOfInnovation : Ability.check (DRS.ofList []) padeemConsulOfInnovation = [] := by decide

/-- Talion, the Kindly Lord -/
def talionTheKindlyLord : Spelled := spelled <| singleFacedCard
  { name := "Talion, the Kindly Lord", cost := [generic 2, pip .blue, pip .black],
    supertypes := [.legendary], types := [.creature],
    subtypes := [faerie, noble],
    text :=
      [ flying,
        static (entersChoosing thisCreature .number),
        whenever
          (does anOpponent (cast (a (and [isSpell,
                      compare .eq chosenNumber (axes := [.stat .manaValue, .stat .power, .stat
                        .toughness])]))))
          (sequentially [instruct (that player) (loseLife 2), draw 1]) ],
    power := stat 3, toughness := stat 4 }

/-- Braid of Fire -/
def braidOfFire : Spelled := spelled <| singleFacedCard
  { name := "Braid of Fire", cost := [generic 1, pip .red], types := [.enchantment],
    text := [cumulativeUpkeep (addMana 1 (manaOf [[.of .red]]))] }

/-- Delighted Halfling -/
def delightedHalflingMana : Ability :=
  activated tapSymbol
    (addMana 1 (ofAnyColor .sameColor)
      [ whenSpentOn .affectsIt true (a (and [hasSupertype .legendary, isSpell]))
          (establish (cant any (counter (that .spell)))) ])
theorem okDelightedHalflingMana : Ability.check (DRS.ofList []) delightedHalflingMana = [] := by decide

/-- Boseiju, Who Shelters All -/
def boseijuMana : Ability :=
  activated (compound [tapSymbol, payLife 2])
    (addMana 1 (manaOf [[.colorless]])
      [ whenSpentOn .affectsIt false (a (and [isInstantOrSorcery, isSpell]))
          (establish (cant any (counter (that .spell)))) ])
theorem okBoseijuMana : Ability.check (DRS.ofList []) boseijuMana = [] := by decide

/-- Generator Servant -/
def generatorServant : Spelled := spelled <| singleFacedCard
  { name := "Generator Servant", cost := [generic 1, pip .red], types := [.creature],
    subtypes := [elemental],
    text :=
      [ activated (compound [tapSymbol, sacrifice thisCreature])
          (addMana 1 (manaOf [[.colorless, .colorless]])
            [ whenSpentOn .affectsIt false (a (and [isCreature, isSpell]))
                (gainHaste (becomes (that .spell)) untilEndOfTurn) ]) ],
    power := stat 2, toughness := stat 1 }

/-- Animal Attendant -/
def animalAttendant : Spelled := spelled <| singleFacedCard
  { name := "Animal Attendant", cost := [generic 1, pip .green], types := [.creature],
    subtypes := [human, citizen],
    text :=
      [ activated tapSymbol
          (addMana 1 (ofAnyColor .sameColor)
            [ whenSpentOn .affectsIt false
                (a (and [non human, isCreature, isSpell]))
                (establish
                  (entersWithAdditionalCounters (becomes (that .spell)) 1
                     p1p1Counter)) ]) ],
    power := stat 2, toughness := stat 2 }

/-- Thran Turbine -/
def thranTurbineMana : Action :=
  addMana 1 (manaOf [[.colorless, .colorless]]) [cantSpendOn [toCast isSpell]]
theorem okThranTurbineMana : Action.check (DRS.ofList []) thranTurbineMana = [] := by decide

/-- Su-Chi Cave Guard -/
def suChiCaveGuardDies : Ability :=
  when (does thisCreature die)
    (sequentially
      [ addMana 1
          (manaOf [[.colorless, .colorless, .colorless, .colorless,
                   .colorless, .colorless, .colorless, .colorless]]),
        establishThroughout (manaRetention you thisMana) untilEndOfTurn ])
theorem okSuChiCaveGuardDies : Ability.check (DRS.ofList []) suChiCaveGuardDies = [] := by decide

/-- Omnath, Locus of Mana -/
def omnathLocusOfManaPersistence : StaticSpec := manaRetention you (unspent (some (.of .green)))
theorem okOmnathLocusOfManaPersistence :
    StaticSpec.check (DRS.ofList []) omnathLocusOfManaPersistence = [] := by decide

/-- Upwelling -/
def upwelling : Spelled := spelled <| singleFacedCard
  { name := "Upwelling", cost := [generic 3, pip .green], types := [.enchantment],
    text := [static (manaRetention eachPlayer (unspent none))] }

/-- Mana Flare -/
def manaFlare : Spelled := spelled <| singleFacedCard
  { name := "Mana Flare", cost := [generic 2, pip .red], types := [.enchantment],
    text :=
      [ whenever (does (a isPlayer) (tapForMana (a isLand)))
          (instruct (that player) (addMana 1 (ofTypeProducedBy (that land))))
              ] }

/-- Shimmerwilds Growth -/
def shimmerwildsGrowth : Spelled := spelled <| singleFacedCard
  { name := "Shimmerwilds Growth", cost := [generic 1, pip .green], types := [.enchantment],
    subtypes := [aura],
    text :=
      [ enchant isLand,
        static (entersChoosing thisAura .color),
        static (qualityChange (enchanted land) .sets
          (chosenQuality (ofChosen .color))),
        whenever (does any (tapForMana (enchanted land)))
          (instruct (controllerOf (that land)) (addMana 1 ofTheChosenColor)) ] }

/-- Gauntlet of Power -/
def gauntletOfPower : Spelled := spelled <| singleFacedCard
  { name := "Gauntlet of Power", cost := some [generic 5], types := [.artifact],
    text :=
      [ static (entersChoosing thisArtifact .color),
        static (getsPt (allOf (and [isCreature, ofChosen .color])) (up 1) (up 1)),
        whenever
          (does any (tapForMana (a (and [isLand, hasSupertype .basic])) (some (ofColor thatColor))))
          (instruct (controllerOf (that land)) (addMana 1 ofTheChosenColor)) ] }

/-- Chrome Mox -/
def chromeMoxMana : Ability :=
  activated tapSymbol
    (addMana 1 (ofAnyColorOf (the exiledWithThisArtifact)))
theorem okChromeMoxMana : Ability.check (DRS.ofList []) chromeMoxMana = [] := by decide

/-- Fellwar Stone -/
def fellwarStone : Spelled := spelled <| singleFacedCard
  { name := "Fellwar Stone", cost := [generic 2], types := [.artifact],
    text :=
      [ activated tapSymbol
          (addMana 1
            (ofTypeProducibleBy (a (and [isLand, anOpponentControls])))) ] }

/-- Ice Cauldron -/
def iceCauldronNotedMana : Ability :=
  activated
    (compound [tapSymbol,
      (removeCounters (exactly 1) chargeCounter thisArtifact)])
    (addMana 1 (ofTheLastNotedType thisArtifact)
      [spendOnly [toCast exiledWithThisArtifact]])
theorem okIceCauldronNotedMana : Ability.check (DRS.ofList []) iceCauldronNotedMana = [] := by decide

/-- Firemind Vessel -/
def firemindVesselMana : Ability :=
  activated tapSymbol (addMana 2 (ofAnyColor .distinctColors))
theorem okFiremindVesselMana : Ability.check (DRS.ofList []) firemindVesselMana = [] := by decide

/-- Goblin Clearcutter -/
def goblinClearcutterMana : Ability :=
  activated
    (compound [tapSymbol, sacrifice (a forest)])
    (addMana 3 (ofAnyColorAmong [.red, .green]))
theorem okGoblinClearcutterMana : Ability.check (DRS.ofList []) goblinClearcutterMana = [] := by decide

/-- Tolaria -/
def tolaria : Spelled := spelled <| singleFacedCard
  { name := "Tolaria", supertypes := [.legendary], types := [.land],
    text :=
      [ activated tapSymbol (addMana 1 (manaOf [[.of .blue]])),
        activatedOnlyDuring (compound [tapSymbol])
          (establishThroughout (abilityLoss (target isCreature)
              [written banding, abilities bandsWithOtherAbilities]) untilEndOfTurn)
          (duringPart .upkeep none) ] }

/-- Psychic Vortex -/
def psychicVortexUpkeep : Ability := cumulativeUpkeep (draw 1)
theorem okPsychicVortexUpkeep : Ability.check (DRS.ofList []) psychicVortexUpkeep = [] := by decide

/-- Varchild's War-Riders -/
def varchildsWarRidersUpkeep : Ability :=
  cumulativeUpkeep
    (instruct anOpponent (createToken (creatureToken 1 1 [.red] [survivor])))
theorem okVarchildsWarRidersUpkeep : Ability.check (DRS.ofList []) varchildsWarRidersUpkeep = [] := by decide

end Semantics.Cards
