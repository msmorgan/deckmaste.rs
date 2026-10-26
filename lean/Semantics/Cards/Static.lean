import Semantics
import Semantics.Definitions
import Semantics.Check.Card
import Semantics.Cards.Choice
import Semantics.Cards.Trigger

/-!
# Semantics.Cards.Static

Port of `idris/src/Experimental/Cards/Static.idr`: the printed cards of the Static family
(continuous clauses: pumps, definitions, ascriptions, locks, alternative costs, visibility) and
the bench items beside them.

`maro`, `peopleOfTheWoods` and `cultivatorColossus` print `*` boxes: those slots are `none`
under the printed-star ruling, and their Characteristic-Defining Abilities set them.
-/

open Semantics Semantics.Templates Semantics.Definitions

namespace Semantics.Cards

def forkedBolt : Action :=
  dealDivided this_ 2 (targets (oneThrough 2) isAnyTarget)
theorem okForkedBolt : Action.check (DRS.ofList []) forkedBolt = [] := by decide
def thoughtReflection : Ability :=
  static (replace (would you (draw 1)) (instead (draw 2)))
theorem okThoughtReflection : Ability.check (DRS.ofList []) thoughtReflection = [] := by decide
def jorKadeen : Ability :=
  static (onlyWhile (getsPt (allOf (and [isCreature, youControl])) (up 3) (up 0))
    (compareAmt (countOf (and [isArtifact, youControl])) .atLeast 3))
theorem okJorKadeen : Ability.check (DRS.ofList []) jorKadeen = [] := by decide
def abandonedOutpost : Ability := static (entersTapped thisLand)
theorem okAbandonedOutpost : Ability.check (DRS.ofList []) abandonedOutpost = [] := by decide
/-- Time Vault -/
def timeVaultLock : Ability := static (doesntUntap thisArtifact you)
theorem okTimeVaultLock : Ability.check (DRS.ofList []) timeVaultLock = [] := by decide
def hymnOfRebirth : Action :=
  putOntoBattlefieldUnderYourControl (target (and [isCreature, in_ graveyard]))
    (from_ := some graveyard)
theorem okHymnOfRebirth : Action.check (DRS.ofList []) hymnOfRebirth = [] := by decide
def counterspell : Action := counter (target isSpell)
theorem okCounterspell : Action.check (DRS.ofList []) counterspell = [] := by decide

def anthemOfChampions : Spelled := spelled <| singleFacedCard
  { name := "Anthem of Champions", cost := [pip .green, pip .white], types := [.enchantment],
    text := [static (getsPt (allOf (and [isCreature, youControl])) (up 1) (up 1))] }

def adantoVanguard : Ability :=
  static (onlyWhile (getsPt thisCreature (up 2) (up 0))
    (is thisCreature isAttacking))
theorem okAdantoVanguard : Ability.check (DRS.ofList []) adantoVanguard = [] := by decide
/-- Nowhere to Run -/
def nowhereToRunTargetLine : StaticSpec :=
  can (by_ (allOf (or [isSpell, abilityHead .anyOnStack]))) (target (allOf (and [isCreature, opponentsControl]))) (asThough := holds (is them (not (hasKeyword hexproof))))
theorem okNowhereToRunTargetLine : StaticSpec.check (DRS.ofList []) nowhereToRunTargetLine = [] := by decide
/-- Hithlain Rope -/
def hithlainRopeSacrificeLock : StaticSpec := cant any (sacrifice this_)
theorem okHithlainRopeSacrificeLock : StaticSpec.check (DRS.ofList []) hithlainRopeSacrificeLock = [] := by
  decide
/-- Lich's Mastery -/
def lichsMasteryGate : Ability := static (cant you loseGame)
theorem okLichsMasteryGate : Ability.check (DRS.ofList []) lichsMasteryGate = [] := by decide
def theGoldenThrone : Ability :=
  static (replace (would you loseGame) (instead (sequentially [exile thisArtifact, setLife 1])))
theorem okTheGoldenThrone : Ability.check (DRS.ofList []) theGoldenThrone = [] := by decide
def stunningReversal : Ability :=
  spell (establishThroughout
           (replace (would you loseGame) (instead (sequentially [draw 7, setLife 1]))
              none .nextTimeOnly) thisTurn)
theorem okStunningReversal : Ability.check (DRS.ofList []) stunningReversal = [] := by decide
def pathOfBravery : Ability :=
  static (onlyWhile (getsPt (allOf (and [isCreature, youControl])) (up 1) (up 1))
    (compareAmt (lifeTotalOf you) .atLeast (statOf (.playerStat .startingLifeTotal) you)))
theorem okPathOfBravery : Ability.check (DRS.ofList []) pathOfBravery = [] := by decide

def deathsShadow : Spelled := spelled <| singleFacedCard
  { name := "Death's Shadow", cost := [pip .black], types := [.creature],
    subtypes := [avatar],
    text :=
      [ static (conjunction [ getsPt thisCreature (down (letter .x)) (down (letter .x)),
            letterDefinition .x (lifeTotalOf you) ]) ],
    power := stat 13, toughness := stat 13 }

def spontaneousMutation : Ability :=
  static (conjunction [ getsPt (enchanted creature) (down (letter .x)) (down 0),
      letterDefinition .x (countOf (cardIn yourGraveyard)) ])
theorem okSpontaneousMutation : Ability.check (DRS.ofList []) spontaneousMutation = [] := by decide

def maro : Spelled := spelled <| singleFacedCard
  { name := "Maro", cost := [generic 2, pip .green, pip .green], types := [.creature],
    subtypes := [elemental],
    text := [static (ptDefinition thisCreature .bothEach (countOf (cardIn yourHand)))] }

def peopleOfTheWoods : Spelled := spelled <| singleFacedCard
  { name := "People of the Woods", cost := [pip .green, pip .green], types := [.creature],
    subtypes := [human],
    text :=
      [ static (ptDefinition thisCreature .toughnessAlone
          (countOf (and [forest, youControl]))) ],
    power := stat 1 }

def scourgeOfTheSkyclaves : Ability :=
  static (ptDefinition thisCreature .bothEach
    (minus 20 (aggregate .max (.playerStat .lifeTotal) isPlayer)))
theorem okScourgeOfTheSkyclaves : Ability.check (DRS.ofList []) scourgeOfTheSkyclaves = [] := by decide
def aettirAndPriwen : Ability :=
  static (conjunction [ getsBase (equipped creature) (letter .x) (letter .x),
      letterDefinition .x (lifeTotalOf you) ])
theorem okAettirAndPriwen : Ability.check (DRS.ofList []) aettirAndPriwen = [] := by decide

def diminish : Spelled := spelled <| singleFacedCard
  { name := "Diminish", cost := [pip .blue], types := [.instant],
    text :=
      [ spell
          (establishThroughout (getsBase (target isCreature) 1 1)
             untilEndOfTurn) ] }

def cycleOfLife : Action :=
  establishThroughout
    (getsBase (target (and [isCreature, did you (cast which) soFarThisTurn])) 0 1)
    (untilEvent (beginningOf .upkeep you nextTime))
theorem okCycleOfLife : Action.check (DRS.ofList []) cycleOfLife = [] := by decide

def aboutFace : Spelled := spelled <| singleFacedCard
  { name := "About Face", cost := [pip .red], types := [.instant],
    text :=
      [ spell (establishThroughout (ptSwitch (target isCreature)) untilEndOfTurn) ] }

def roilingHorror : Ability :=
  static (ptDefinition thisCreature .bothEach
    (minus (lifeTotalOf you)
      (lifeTotalOf
        (a (and [isOpponent, superlative .max (.playerStat .lifeTotal) isOpponent])))))
theorem okRoilingHorror : Ability.check (DRS.ofList []) roilingHorror = [] := by decide

/-- Katara, the Fearless -/
def kataraTheFearless : Spelled := spelled <| singleFacedCard
  { name := "Katara, the Fearless", cost := [pip .green, pip .white, pip .blue],
    supertypes := [.legendary], types := [.creature],
    subtypes := [human, warrior, ally],
    text :=
      [ static (additionalTriggers
          (triggers (a (and [ abilityHead .anyTriggered,
                       abilityOf (a (and [ ally,
                                             youControl ])) ])))
          (exactly 1)) ],
    power := stat 3, toughness := stat 3 }

/-- Rain of Gore -/
def rainOfGore : Spelled := spelled <| singleFacedCard
  { name := "Rain of Gore", cost := [pip .black, pip .red], types := [.enchantment],
    text :=
      [ static (replace
    (causedBy
      (bySource (a (or [isSpell,
        abilityHead .anyOnStack])))
      (would
        (controllerOf it)
        (gainLife anyAmount)))
    (instead (instruct (that player) (loseLife thatMuch)))) ] }

/-- Wilt-Leaf Liege's last line: "If a spell or ability an opponent controls causes you to discard
this card, put it onto the battlefield instead of putting it into your graveyard." -/
def wiltLeafLiegeDiscardReplacement : Ability :=
  static (replace
    (causedBy
      (bySource (a (and [or [isSpell, abilityHead .anyOnStack], anOpponentControls])))
      (would you (discard thisCard)))
    (instead (putOntoBattlefield it)))

/-- The discard has a performer (you) and, orthogonally, a cause (a spell or ability an opponent
controls) [CR#609.1,701.9a], and the replacement checks clean with both. -/
theorem okWiltLeafLiegeEffectCausedDiscard :
    Ability.check (DRS.ofList []) wiltLeafLiegeDiscardReplacement = [] ∧
    (match wiltLeafLiegeDiscardReplacement with
      | .static (.replace (.causedBy (.source _) (.would (.actor .you) _)) ..) => true
      | _ => false) = true := by decide

/-- Wilt-Leaf Liege -/
def wiltLeafLiege : Spelled := spelled <| singleFacedCard
  { name := "Wilt-Leaf Liege",
    cost := [generic 1, hybridPip .green .white, hybridPip .green .white, hybridPip .green .white],
    types := [.creature], subtypes := [elf, knight],
    text :=
      [ static (getsPt (allOf (and [isCreature, hasColor .green, youControl, isOther]))
          (up 1) (up 1)),
        static (getsPt (allOf (and [isCreature, hasColor .white, youControl, isOther]))
          (up 1) (up 1)),
        wiltLeafLiegeDiscardReplacement ],
    power := stat 4, toughness := stat 4 }

/-- Master Chef -/
def masterChefGrantedAbility : Ability :=
  static (entersWithAdditionalCounters thisCreature 1 p1p1Counter)
theorem okMasterChefGrantedAbility : Ability.check (DRS.ofList []) masterChefGrantedAbility = [] := by decide

def anointedProcession : Spelled := spelled <| singleFacedCard
  { name := "Anointed Procession", cost := [generic 3, pip .white], types := [.enchantment],
    text :=
      [ static (replace (causedBy byAnEffect (would any (createObject anyAmount (token (copyOf (counted (orMore 1) isToken)) []) you))) (instead (create (times 2 groupSize) asThose []))) ] }

def naturalAffinity : Spelled := spelled <| singleFacedCard
  { name := "Natural Affinity", cost := [generic 2, pip .green], types := [.instant],
    text :=
      [ spell (establishThroughout (qualityChange (allOf isLand) .sets
            (bundle { characteristics := { types := [.creature], power := stat 2, toughness := stat 2 } }
              (some .land))) untilEndOfTurn) ] }

def turnToFrog : Spelled := spelled <| singleFacedCard
  { name := "Turn to Frog", cost := [generic 1, pip .blue], types := [.instant],
    text :=
      [ spell (establishThroughout (conjunction [ allAbilityLoss (target isCreature),
              qualityChange it .sets
                (bundle { characteristics := { colors := [.blue], subtypes := [frog] } }),
              getsBase it 1 1 ]) untilEndOfTurn) ] }

def humility : Spelled := spelled <| singleFacedCard
  { name := "Humility", cost := [generic 2, pip .white, pip .white], types := [.enchantment],
    text :=
      [ static (conjunction [ allAbilityLoss (allOf isCreature),
            getsBase them 1 1 ]) ] }

/-- Tahngarth, First Mate -/
def tahngarthAttacksThatJoin : Action := becomeAttacking thisCreature thatJoin
theorem okTahngarthAttacksThatJoin :
    Action.check (nextClause ((stepAction 512 .postEvent .obj ((stepGameEvent 512 (DRS.ofList []) tahngarthHeader).1) tahngarthChoosesDefender).1))
      tahngarthAttacksThatJoin = [] := by
  decide

def mindlockOrb : Spelled := spelled <| singleFacedCard
  { name := "Mindlock Orb", cost := [generic 3, pip .blue], types := [.artifact],
    text := [static (cant (bare isPlayer) (search libraries))] }

def silence : Spelled := spelled <| singleFacedCard
  { name := "Silence", cost := [pip .white], types := [.instant],
    text :=
      [ spell (establishThroughout (cant yourOpponents (cast any)) thisTurn) ] }

def shadowOfDoubt : Spelled := spelled <| singleFacedCard
  { name := "Shadow of Doubt", cost := [hybridPip .blue .black, hybridPip .blue .black],
    types := [.instant],
    text :=
      [ spell (sequentially
          [ establishThroughout (cant (bare isPlayer) (search libraries))
              thisTurn,
            draw 1 ]) ] }

def omenMachineDraw : Ability := static (cant (bare isPlayer) (draw anyAmount))
theorem okOmenMachineDraw : Ability.check (DRS.ofList []) omenMachineDraw = [] := by decide
def solfataraLandLock : Action :=
  establishThroughout (cant (target isPlayer) (play any)) thisTurn
theorem okSolfataraLandLock : Action.check (DRS.ofList []) solfataraLandLock = [] := by decide

/-- "each nonbasic land is a <type>": a moon effect. -/
def nonbasicLandsAre (type : Subtype) : StaticSpec :=
  qualityChange (allOf (and [isLand, non (hasSupertype .basic)])) .sets
    (bundle { characteristics := { subtypes := [type] } })

def bloodMoon : Spelled := spelled <| singleFacedCard
  { name := "Blood Moon", cost := [generic 2, pip .red], types := [.enchantment],
    text := [static (nonbasicLandsAre mountain)] }

def magusOfTheMoon : Spelled := spelled <| singleFacedCard
  { name := "Magus of the Moon", cost := [generic 2, pip .red], types := [.creature],
    subtypes := [human, wizard],
    text := [static (nonbasicLandsAre mountain)], power := stat 2, toughness := stat 2 }

def harbingerOfTheSeas : Spelled := spelled <| singleFacedCard
  { name := "Harbinger of the Seas", cost := [generic 1, pip .blue, pip .blue],
    types := [.creature], subtypes := [merfolk, wizard],
    text := [static (nonbasicLandsAre island)], power := stat 2, toughness := stat 2 }

def yavimayaCradleOfGrowth : Spelled := spelled <| singleFacedCard
  { name := "Yavimaya, Cradle of Growth", supertypes := [.legendary], types := [.land],
    text :=
      [ static (addSubtype (allOf isLand) forest) ] }

def rallyTheRanks : Spelled := spelled <| singleFacedCard
  { name := "Rally the Ranks", cost := [generic 1, pip .white], types := [.enchantment],
    text :=
      [ static (entersChoosing thisEnchantment (.subtype .creature)),
        static (getsPt
          (allOf (and [isCreature, youControl, ofChosen (.subtype .creature)]))
          (up 1) (up 1)) ] }

def sharedTriumph : Spelled := spelled <| singleFacedCard
  { name := "Shared Triumph", cost := [generic 1, pip .white], types := [.enchantment],
    text :=
      [ static (entersChoosing thisEnchantment (.subtype .creature)),
        static (getsPt (allOf (and [isCreature, ofChosen (.subtype .creature)]))
          (up 1) (up 1)) ] }

def hallOfTriumph : Spelled := spelled <| singleFacedCard
  { name := "Hall of Triumph", cost := [generic 3], supertypes := [.legendary],
    types := [.artifact],
    text :=
      [ static (entersChoosing thisArtifact .color),
        static (getsPt (allOf (and [isCreature, youControl, ofChosen .color]))
          (up 1) (up 1)) ] }

def engineeredPlague : Spelled := spelled <| singleFacedCard
  { name := "Engineered Plague", cost := [generic 2, pip .black], types := [.enchantment],
    text :=
      [ static (entersChoosing thisEnchantment (.subtype .creature)),
        static (getsPt (allOf (and [isCreature, ofChosen (.subtype .creature)]))
          (down 1) (down 1)) ] }

/-- Volrath's Laboratory -/
def volrathsLaboratoryChoice : StaticSpec :=
  conjunction [entersChoosing thisArtifact .color, entersChoosing thisArtifact (.subtype .creature)]
theorem okVolrathsLaboratoryChoice : StaticSpec.check (DRS.ofList []) volrathsLaboratoryChoice = [] := by decide
/-- Call to Arms -/
def callToArmsChoice : StaticSpec :=
  conjunction [ entersChoosing thisEnchantment .color,
      entersChoosingPlayer thisEnchantment anOpponent ]
theorem okCallToArmsChoice : StaticSpec.check (DRS.ofList []) callToArmsChoice = [] := by decide

def encroachingMycosynth : Spelled := spelled <| singleFacedCard
  { name := "Encroaching Mycosynth", cost := [generic 3, pip .blue], types := [.artifact],
    text :=
      [ static (qualityChange (and [
            allOf (and [isPermanent, non isLand, youControl]),
            allOf (and
                     [or [isArtifact, isBattle, isCreature, isEnchantment, isPlaneswalker],
                      isSpell, youControl]),
            allOf (and [isCard, isPermanentCard, non isLand, youOwn, not (in_ battlefield)]) ])
              .adds
            (bundle { characteristics := { types := [.artifact] } })) ] }

/-- Darkest Hour -/
def darkestHour : Spelled := spelled <| singleFacedCard
  { name := "Darkest Hour", cost := [pip .black], types := [.enchantment],
    text := [static (qualityChange (allOf isCreature) .sets (colored (.some [.black])))] }

/-- Thran Lens -/
def thranLens : Spelled := spelled <| singleFacedCard
  { name := "Thran Lens", cost := [generic 2], types := [.artifact],
    text := [static (qualityChange (allOf isPermanent) .sets (colored (.some [])))] }

/-- Ghostflame Sliver -/
def ghostflameSliver : Spelled := spelled <| singleFacedCard
  { name := "Ghostflame Sliver", cost := [pip .black, pip .red], types := [.creature],
    subtypes := [sliver],
    text :=
      [ static (qualityChange (allOf sliver) .sets (colored
          (.some []))) ],
    power := stat 2, toughness := stat 2 }

/-- Nightcreep -/
def nightcreep : Spelled := spelled <| singleFacedCard
  { name := "Nightcreep", cost := [pip .black, pip .black], types := [.instant],
    text :=
      [ spell
          (establishThroughout
             (conjunction [ qualityChange (allOf isCreature) .sets (colored (.some [.black])),
              qualityChange (allOf isLand) .sets
                (bundle { characteristics := { subtypes := [swamp] } }) ]) untilEndOfTurn) ] }

/-- Celestial Dawn -/
def celestialDawnAscriptions : List Ability :=
  [ static (qualityChange (allOf (and [isLand, youControl])) .sets
      (bundle { characteristics := { subtypes := [plains] } })),
    static (qualityChange (and [
              allOf (and [isPermanent, non isLand, youControl]),
              allOf (and [isSpell, youControl]),
              allOf (and [isCard, non isLand, youOwn, not (in_ battlefield)]) ]) .sets
        (colored (.some [.white]))) ]
theorem okCelestialDawnAscriptions : Ability.checkText (DRS.ofList []) celestialDawnAscriptions = [] := by decide

/-- Transguild Courier -/
def transguildCourier : Spelled := spelled <| singleFacedCard
  { name := "Transguild Courier", cost := [generic 4], types := [.artifact, .creature],
    subtypes := [golem],
    text := [static (qualityDefinition thisCreature .sets (colored .every))],
    power := stat 3, toughness := stat 3 }

/-- Booby Trap -/
def boobyTrapNameChoice : StaticSpec :=
  entersChoosingFrom thisArtifact .cardName (nameOfCard (not (and [hasSupertype .basic, isLand])))
theorem okBoobyTrapNameChoice : StaticSpec.check (DRS.ofList []) boobyTrapNameChoice = [] := by decide

def dovinsVeto : Spelled := spelled <| singleFacedCard
  { name := "Dovin's Veto", cost := [pip .white, pip .blue], types := [.instant],
    text :=
      [ static (cant any (counter thisSpell)),
        spell (counter (target (and [isSpell, non isCreature]))) ] }

def goblinSpy : Spelled := spelled <| singleFacedCard
  { name := "Goblin Spy", cost := [pip .red], types := [.creature],
    subtypes := [goblin, rogue],
    text := [static (visibility .reveal you topOfLibrary)],
    power := stat 1, toughness := stat 1 }

def futureSight : Spelled := spelled <| singleFacedCard
  { name := "Future Sight", cost := [generic 2, pip .blue, pip .blue, pip .blue],
    types := [.enchantment],
    text :=
      [ static (visibility .reveal you topOfLibrary),
        static playLandsAndCastSpellsFromTop ] }

def magusOfTheFuture : Spelled := spelled <| singleFacedCard
  { name := "Magus of the Future", cost := [generic 2, pip .blue, pip .blue, pip .blue],
    types := [.creature], subtypes := [human, wizard],
    text :=
      [ static (visibility .reveal you topOfLibrary),
        static playLandsAndCastSpellsFromTop ],
    power := stat 2, toughness := stat 3 }

def telepathy : Spelled := spelled <| singleFacedCard
  { name := "Telepathy", cost := [pip .blue], types := [.enchantment],
    text := [static (visibility .reveal yourOpponents wholeHand)] }

def assembleThePlayers : Spelled := spelled <| singleFacedCard
  { name := "Assemble the Players", cost := [generic 1, pip .white], types := [.enchantment],
    text :=
      [ static (visibility .lookAt you topOfLibrary),
        static castSmallCreatureFromTopOnceEachTurn ] }

def prismaticOmen : Spelled := spelled <| singleFacedCard
  { name := "Prismatic Omen", cost := [generic 1, pip .green], types := [.enchantment],
    text :=
      [ static (qualityChange (allOf (and [isLand, youControl])) .adds
          (everyTypeOf .basicLand)) ] }

def mistformUltimus : Spelled := spelled <| singleFacedCard
  { name := "Mistform Ultimus", cost := [generic 3, pip .blue], supertypes := [.legendary],
    types := [.creature], subtypes := [illusion],
    text := [static (qualityDefinition thisCreature .adds (everyTypeOf .creature))],
    power := stat 3, toughness := stat 3 }

def volatileClaws : Spelled := spelled <| singleFacedCard
  { name := "Volatile Claws", cost := [generic 2, pip .red], types := [.instant],
    text :=
      [ spell
          (establishFor (allOf (and [isCreature, youControl]))
            [ getsPt them (up 2) (up 0),
              qualityChange them .adds (everyTypeOf .creature) ]
            untilEndOfTurn) ] }

/-- Nameless Inversion -/
def namelessInversionBody : Action :=
  establishFor (target isCreature)
    [getsPt it (up 3) (down 3), qualityChange it .loses (everyTypeOf .creature)]
    untilEndOfTurn
theorem okNamelessInversionBody : Action.check (DRS.ofList []) namelessInversionBody = [] := by decide
/-- Ego Erasure -/
def egoErasureBody : Action :=
  establishFor (allOf (and [isCreature, controlledBy (target isPlayer)]))
    [getsPt them (down 2) (up 0), qualityChange them .loses (everyTypeOf .creature)]
    untilEndOfTurn
theorem okEgoErasureBody : Action.check (DRS.ofList []) egoErasureBody = [] := by decide
/-- Lithoform Blight -/
def lithoformBlightLoss : StaticSpec :=
  conjunction [ qualityChange (enchanted land) .loses (everyTypeOf .land),
      allAbilityLoss it ]
theorem okLithoformBlightLoss : StaticSpec.check (DRS.ofList []) lithoformBlightLoss = [] := by decide

/-- Energybending -/
def energybending : Spelled := spelled <| singleFacedCard
  { name := "Energybending", cost := [generic 2], types := [.instant],
    subtypes := [lesson],
    text :=
      [ spell (sequentially
          [ establishThroughout (qualityChange (allOf (and [isLand, youControl])) .adds
                (everyTypeOf .basicLand)) untilEndOfTurn,
            draw 1 ]) ] }

/-- Seedborn Muse -/
def seedbornMuse : Spelled := spelled <| singleFacedCard
  { name := "Seedborn Muse", cost := [generic 3, pip .green, pip .green],
    types := [.creature], subtypes := [spirit],
    text :=
      [ static (untapDuring (allOf youControl) (each isOtherPlayer)) ],
    power := stat 2, toughness := stat 4 }

/-- Unwinding Clock -/
def unwindingClock : Spelled := spelled <| singleFacedCard
  { name := "Unwinding Clock", cost := [generic 4], types := [.artifact],
    text :=
      [ static (untapDuring (allOf (and [isArtifact, youControl]))
          (each isOtherPlayer)) ] }

/-- Thousand Moons Infantry -/
def thousandMoonsInfantry : Spelled := spelled <| singleFacedCard
  { name := "Thousand Moons Infantry", cost := [generic 2, pip .white], types := [.creature],
    subtypes := [human, soldier],
    text := [static (untapDuring thisCreature (each isOtherPlayer))],
    power := stat 2, toughness := stat 4 }

/-- Blatant Thievery -/
def blatantThievery : Spelled := spelled <| singleFacedCard
  { name := "Blatant Thievery", cost := [generic 4, pip .blue, pip .blue, pip .blue],
    types := [.sorcery],
    text :=
      [ spell (doForEach eachOpponent
          (gainControl (target (controlledBy (that player))))) ] }

/-- "activated abilities of <p> can't be activated" -/
def cantActivateAbilitiesOf (p : Predicate) : StaticSpec :=
  cant any (activate (allOf (and [abilityHead .anyActivated, abilityOf (allOf p)])))

def cursedTotem : Spelled := spelled <| singleFacedCard
  { name := "Cursed Totem", cost := [generic 2], types := [.artifact],
    text := [static (cantActivateAbilitiesOf isCreature)] }

def nullRod : Spelled := spelled <| singleFacedCard
  { name := "Null Rod", cost := [generic 2], types := [.artifact],
    text := [static (cantActivateAbilitiesOf isArtifact)] }

def stonySilence : Spelled := spelled <| singleFacedCard
  { name := "Stony Silence", cost := [generic 1, pip .white], types := [.enchantment],
    text := [static (cantActivateAbilitiesOf isArtifact)] }

def dampingMatrix : Spelled := spelled <| singleFacedCard
  { name := "Damping Matrix", cost := [generic 3], types := [.artifact],
    text :=
      [ static (cant any (activate (allOf (and [ abilityHead .anyActivated,
                         abilityOf (allOf (or [isArtifact, isCreature])),
                         not isManaAbility ])))) ] }

def crash : Spelled := spelled <| singleFacedCard
  { name := "Crash", cost := [generic 2, pip .red], types := [.instant],
    text :=
      [ static (altCost this_
          (sacrifice (a (and [isLand, mountain])))),
        spell (destroy (target isArtifact)) ] }

def moggSalvage : Spelled := spelled <| singleFacedCard
  { name := "Mogg Salvage", cost := [generic 2, pip .red], types := [.instant],
    text :=
      [ static (onlyWhile (altCost this_)
          (and
            [ exists_ (and [ isLand, island,
                              anOpponentControls ]),
              exists_ (and [ isLand, mountain,
                              youControl ]) ])),
        spell (destroy (target isArtifact)) ] }

def abolish : Spelled := spelled <| singleFacedCard
  { name := "Abolish", cost := [generic 1, pip .white, pip .white], types := [.instant],
    text :=
      [ static (altCost this_
          (discard (a (and [plains, in_ yourHand])))),
        spell (destroy (target (or [isArtifact, isEnchantment]))) ] }

def gush : Spelled := spelled <| singleFacedCard
  { name := "Gush", cost := [generic 4, pip .blue], types := [.instant],
    text :=
      [ static (altCost this_
          (returnToHand
            (counted (exactly 2)
              (and [isLand, island, youControl])))),
        spell (draw 2) ] }

def sunscour : Spelled := spelled <| singleFacedCard
  { name := "Sunscour", cost := [generic 5, pip .white, pip .white], types := [.sorcery],
    text :=
      [ static (altCost this_
          (exile
            (counted (exactly 2) (and [hasColor .white, in_ yourHand])))),
        spell (destroy (allOf isCreature)) ] }

def massacre : Spelled := spelled <| singleFacedCard
  { name := "Massacre", cost := [generic 2, pip .black, pip .black], types := [.sorcery],
    text :=
      [ static (onlyWhile (altCost this_)
          (and
            [ exists_ (and [ isLand, plains,
                              anOpponentControls ]),
              exists_ (and [ isLand, swamp,
                              youControl ]) ])),
        spell (get (allOf isCreature) (down 2) (down 2) untilEndOfTurn)
            ] }

def rouse : Spelled := spelled <| singleFacedCard
  { name := "Rouse", cost := [generic 1, pip .black], types := [.instant],
    text :=
      [ static (onlyWhile (altCost this_ (payLife 2))
          (exists_ (and [isLand, swamp, youControl]))),
        spell (get (target isCreature) (up 2) (up 0) untilEndOfTurn) ]
            }

def thwart : Spelled := spelled <| singleFacedCard
  { name := "Thwart", cost := [generic 2, pip .blue, pip .blue], types := [.instant],
    text :=
      [ static (altCost this_
          (returnToHand
            (counted (exactly 3)
              (and [isLand, island, youControl])))),
        spell (counter (target isSpell)) ] }

def theLadyOfOtariaLine : StaticSpec :=
  altCost this_
    (tap
      (counted (exactly 3)
        (and [ isCreature, dwarf, youControl,
                isUntapped ])))
theorem okTheLadyOfOtariaLine : StaticSpec.check (DRS.ofList []) theLadyOfOtariaLine = [] := by decide

/-- Clergy of the Holy Nimbus -/
def clergyOfTheHolyNimbus : Ability :=
  static (replace (would any (destroy thisCreature)) (instead (regeneration it)))
theorem okClergyOfTheHolyNimbus : Ability.check (DRS.ofList []) clergyOfTheHolyNimbus = [] := by decide
/-- Rampant Frogantua -/
def rampantFrogantuaPump : Ability :=
  static (getsPt thisCreature
    (up (forEach 10 (and [isPlayer,
      did which loseGame thisGame])))
    (up (forEach 10 (and [isPlayer,
      did which loseGame thisGame]))))
theorem okRampantFrogantuaPump : Ability.check (DRS.ofList []) rampantFrogantuaPump = [] := by decide
/-- Maskwood Nexus -/
def maskwoodNexusTypes : Ability :=
  static (qualityChange (and [
              allOf (and [isCreature, youControl]),
              allOf (and [isCreature, isSpell, youControl]),
              allOf (and [isCard, isCreature, youOwn, not (in_ battlefield)]) ]) .adds (everyTypeOf
      .creature))
theorem okMaskwoodNexusTypes : Ability.check (DRS.ofList []) maskwoodNexusTypes = [] := by decide
/-- Luxior, Giada's Gift -/
def luxiorEquippedPermanent : Ability :=
  static (qualityChange (equipped permanent) .adds
    (bundle { characteristics := { types := [.creature] } }))
theorem okLuxiorEquippedPermanent : Ability.check (DRS.ofList []) luxiorEquippedPermanent = [] := by decide
/-- Nahiri, the Unforgiving's compleated reminder -/
def nahiriCompleatedEntry : Ability :=
  static (entersWithFewerCounters thisPlaneswalker 2 loyaltyCounter)
theorem okNahiriCompleatedEntry : Ability.check (DRS.ofList []) nahiriCompleatedEntry = [] := by decide
/-- Nimble Mongoose -/
def nimbleMongoose : Ability :=
  threshold
    (static (onlyWhile (getsPt thisCreature (up 2) (up 2))
      (compareAmt (countOf (cardIn yourGraveyard)) .atLeast 7)))
theorem okNimbleMongoose : Ability.check (DRS.ofList []) nimbleMongoose = [] := by decide
/-- Anax, Hardened in the Forge -/
def anaxPowerDefinition : Ability :=
  static (ptDefinition thisCreature .powerAlone (devotion you (color .red)))
theorem okAnaxPowerDefinition : Ability.check (DRS.ofList []) anaxPowerDefinition = [] := by decide
/-- Aspect of Wolf -/
def aspectOfWolf : Ability :=
  static (conjunction [ getsPt (enchanted creature) (up (letter .x)) (up (letter .y)),
      letterDefinition .x (half .down
        (countOf (and [forest, youControl]))),
      letterDefinition .y (half .up
        (countOf (and [forest, youControl]))) ])
theorem okAspectOfWolf : Ability.check (DRS.ofList []) aspectOfWolf = [] := by decide
/-- Lhurgoyf -/
def lhurgoyfDefinition : Ability :=
  static (ptDefinitions thisCreature (countOf (and [isCreature, in_ graveyard]))
    (plus thatMuch 1))
theorem okLhurgoyfDefinition : Ability.check (DRS.ofList []) lhurgoyfDefinition = [] := by decide

/-- Invigorate -/
def invigorate : Spelled := spelled <| singleFacedCard
  { name := "Invigorate", cost := [generic 2, pip .green], types := [.instant],
    text :=
      [ static (onlyWhile (altCost this_ (instruct anOpponent (gainLife 3)))
          (exists_ (and [isLand, forest, youControl]))),
        spell (get (target isCreature) (up 4) (up 4) untilEndOfTurn) ]
            }

/-- Deflecting Swat -/
def deflectingSwatCommanderAltCost : Ability :=
  static (onlyWhile (altCost this_)
    (exists_ (and [hasDesignation commander, youControl])))
theorem okDeflectingSwatCommanderAltCost :
    Ability.check (DRS.ofList []) deflectingSwatCommanderAltCost = [] := by decide

/-- Fist of Suns -/
def fistOfSuns : Spelled := spelled <| singleFacedCard
  { name := "Fist of Suns", cost := [generic 3], types := [.artifact],
    text :=
      [ static (altCost (allOf (and [isSpell, did you (cast which) thisObject]))
          [pip .white, pip .blue, pip .black, pip .red, pip .green]) ] }

/-- Rooftop Storm -/
def rooftopStorm : Spelled := spelled <| singleFacedCard
  { name := "Rooftop Storm", cost := [generic 5, pip .blue], types := [.enchantment],
    text :=
      [ static (altCost
          (allOf (and [zombie, isCreature, isSpell, did you (cast which) thisObject]))
          [generic 0]) ] }

/-- Voltage Surge's declaration -/
def voltageSurgeAddedCost : Ability :=
  static (addedCost (sacrifice (a isArtifact)) true)
theorem okVoltageSurgeAddedCost : Ability.check (DRS.ofList []) voltageSurgeAddedCost = [] := by decide
/-- Tarmogoyf -/
def tarmogoyfDefinition : Ability :=
  static (ptDefinitions thisCreature
    (distinctCount .cardType (allOf (cardIn (graveyardOf (bare isPlayer)))))
    (plus thatMuch 1))
theorem okTarmogoyfDefinition : Ability.check (DRS.ofList []) tarmogoyfDefinition = [] := by decide
/-- Consuming Blob's definition -/
def consumingBlobDefinition : Ability :=
  static (ptDefinitions thisCreature (distinctCount .cardType (allOf (cardIn yourGraveyard)))
    (plus thatMuch 1))
theorem okConsumingBlobDefinition : Ability.check (DRS.ofList []) consumingBlobDefinition = [] := by decide
/-- Nighthawk Scavenger's definition -/
def nighthawkScavengerDefinition : Ability :=
  static (ptDefinition thisCreature .powerAlone
    (plus 1
      (distinctCount .cardType (allOf (cardIn (graveyardOf yourOpponents))))))
theorem okNighthawkScavengerDefinition : Ability.check (DRS.ofList []) nighthawkScavengerDefinition = [] := by
  decide
/-- Faeburrow Elder's pump -/
def faeburrowElderPump : Ability :=
  static (getsPt thisCreature
    (up (times 1
      (distinctCount .color (allOf (and [isPermanent, youControl])))))
    (up (times 1
      (distinctCount .color (allOf (and [isPermanent, youControl]))))))
theorem okFaeburrowElderPump : Ability.check (DRS.ofList []) faeburrowElderPump = [] := by decide
/-- Bonds of Faith -/
def bondsOfFaithPump : Ability :=
  static (onlyWhile
    (getsPt (enchanted creature) (up 2) (up 2))
    (is it human))
theorem okBondsOfFaithPump : Ability.check (DRS.ofList []) bondsOfFaithPump = [] := by decide

/-- Field of Dreams -/
def fieldOfDreams : Spelled := spelled <| singleFacedCard
  { name := "Field of Dreams", cost := [pip .blue], supertypes := [.world],
    types := [.enchantment],
    text := [static (visibility .reveal (bare isPlayer) topOfLibrary)] }

/-- Lantern of Insight -/
def lanternOfInsightRider : Ability :=
  static (visibility .reveal (bare isPlayer) topOfLibrary)
theorem okLanternOfInsightRider : Ability.check (DRS.ofList []) lanternOfInsightRider = [] := by decide

/-- Revelation -/
def revelation : Spelled := spelled <| singleFacedCard
  { name := "Revelation", cost := [pip .green], supertypes := [.world], types := [.enchantment],
    text := [static (visibility .reveal (bare isPlayer) wholeHand)] }

/-- Phyrexian Unlife -/
def phyrexianUnlifeImmunity : Ability :=
  static (cant you (loseGame (some .nonpositiveLife)))
theorem okPhyrexianUnlifeImmunity : Ability.check (DRS.ofList []) phyrexianUnlifeImmunity = [] := by decide
/-- Umbris, Fear Manifest -/
def umbrisPump : Ability :=
  static (getsPt thisCreature
    (up (forEach 1
      (and [isCard, opponentsOwn, in_ exileZone])))
    (up (forEach 1
      (and [isCard, opponentsOwn, in_ exileZone]))))
theorem okUmbrisPump : Ability.check (DRS.ofList []) umbrisPump = [] := by decide
/-- Turbulent Fen -/
def turbulentFen : Ability :=
  static (entersTappedUnless thisLand
    (compareAmt (countOf (and [isLand, opponentsControl]))
      .atLeast 8))
theorem okTurbulentFen : Ability.check (DRS.ofList []) turbulentFen = [] := by decide
/-- Bastion Protector -/
def bastionProtectorPump : Ability :=
  static (getsPt
    (allOf (and [isCreature, hasDesignation commander, youControl]))
    (up 2) (up 2))
theorem okBastionProtectorPump : Ability.check (DRS.ofList []) bastionProtectorPump = [] := by decide
/-- Luxior, Giada's Gift -/
def luxiorTypeSetting : Ability :=
  static (conjunction [ qualityChange (equipped permanent) .loses
        (bundle { characteristics := { types := [.planeswalker] } }),
      qualityChange it .adds (bundle { characteristics := { types := [.creature] } }) ])
theorem okLuxiorTypeSetting : Ability.check (DRS.ofList []) luxiorTypeSetting = [] := by decide
/-- Kasmina, Enigma Sage -/
def kasminaLoyaltySharing : StaticSpec :=
  abilityGrantFrom (allOf (and [isPlaneswalker, youControl, isOther])) this_ [.loyalty]
theorem okKasminaLoyaltySharing : StaticSpec.check (DRS.ofList []) kasminaLoyaltySharing = [] := by decide
/-- Nicol Bolas, Dragon-God -/
def nicolBolasDragonGodSharing : StaticSpec :=
  abilityGrantFrom this_ (allOf
                             (and [isPlaneswalker, isOther, in_ battlefield])) [.loyalty]
theorem okNicolBolasDragonGodSharing : StaticSpec.check (DRS.ofList []) nicolBolasDragonGodSharing = [] := by
  decide
/-- Myr Welder -/
def myrWelderBorrowedAbilities : StaticSpec :=
  abilityGrantFrom thisCreature (allOf (in_ (exiledWith this_))) [.anyActivated]
theorem okMyrWelderBorrowedAbilities : StaticSpec.check (DRS.ofList []) myrWelderBorrowedAbilities = [] := by
  decide
/-- Sharkey, Tyrant of the Shire -/
def sharkeyBorrowedLandAbilities : StaticSpec :=
  abilityGrantFrom this_ (allOf (and [isLand, opponentsControl])) [.anyActivated] (some
                                                                                      isManaAbility)
theorem okSharkeyBorrowedLandAbilities : StaticSpec.check (DRS.ofList []) sharkeyBorrowedLandAbilities = [] := by
  decide

/-- Skill Borrower -/
def skillBorrower : Spelled := spelled <| singleFacedCard
  { name := "Skill Borrower", cost := [generic 2, pip .blue], types := [.artifact, .creature],
    subtypes := [human, wizard],
    text :=
      [ static (visibility .reveal you topOfLibrary),
        static (onlyWhile
          (abilityGrantFrom thisCreature (topSlice 1) [.anyActivated])
          (is (topSlice 1) (or [isArtifact, isCreature]))) ],
    power := stat 1, toughness := stat 3 }

/-- Emissary of Grudges -/
def emissaryOfGrudgesEntry : StaticSpec :=
  entersChoosingPlayerSecretly thisCreature anOpponent
theorem okEmissaryOfGrudgesEntry : StaticSpec.check (DRS.ofList []) emissaryOfGrudgesEntry = [] := by decide

def aerialAssault : Action := destroy (target (and [isCreature, isTapped]))
theorem okAerialAssault : Action.check (DRS.ofList []) aerialAssault = [] := by decide
def asphyxiate : Action := destroy (target (and [isCreature, isUntapped]))
theorem okAsphyxiate : Action.check (DRS.ofList []) asphyxiate = [] := by decide
def vindicate : Action := destroy (target isPermanent)
theorem okVindicate : Action.check (DRS.ofList []) vindicate = [] := by decide

def counterspellCard : Spelled := spelled <| singleFacedCard
  { name := "Counterspell", cost := [pip .blue, pip .blue], types := [.instant],
    text := [spell counterspell] }

def hymnOfRebirthCard : Spelled := spelled <| singleFacedCard
  { name := "Hymn of Rebirth", cost := [generic 3, pip .green, pip .white],
    types := [.sorcery], text := [spell hymnOfRebirth] }

def chandrasPyrohelixCard : Spelled := spelled <| singleFacedCard
  { name := "Chandra's Pyrohelix", cost := [generic 1, pip .red], types := [.instant],
    text := [spell forkedBolt] }

/-- Smite -/
def smite : Spelled := spelled <| singleFacedCard
  { name := "Smite", cost := [pip .white], types := [.instant],
    text := [spell (destroy (target (and [isCreature, isBlocked])))] }

def eerieUltimatum : Spelled := spelled <| singleFacedCard
  { name := "Eerie Ultimatum",
    cost := [pip .white, pip .white, pip .black, pip .black, pip .black, pip .green, pip .green],
    types := [.sorcery],
    text :=
      [ spell (returnToBattlefield (withDifferentNames
            (counted anyNumber (and [isPermanentCard, in_ yourGraveyard])))
            (from_ := some yourGraveyard)) ] }

/-- Gray Merchant of Asphodel -/
def grayMerchantDrain : Action :=
  sequentially
    [instruct eachOpponent (loseLife (letter .x)), define .x (devotion you (color .black))]
theorem okGrayMerchantDrain : Action.check (DRS.ofList []) grayMerchantDrain = [] := by decide

end Semantics.Cards
