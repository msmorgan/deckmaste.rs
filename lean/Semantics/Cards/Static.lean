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
  dealDivided this_ (.lit 2) (targets (oneThrough 2) isAnyTarget)
theorem okForkedBolt : Action.check (DRS.ofList []) forkedBolt = [] := by decide
def thoughtReflection : Ability :=
  static (replace (would you (draw (.lit 1))) (instead (draw (.lit 2))))
theorem okThoughtReflection : Ability.check (DRS.ofList []) thoughtReflection = [] := by decide
def jorKadeen : Ability :=
  static (onlyWhile (getsPt (allOf (and [isCreature, youControl])) (up (.lit 3)) (up (.lit 0)))
    (compareAmt (countOf (and [isArtifact, youControl])) .atLeast (.lit 3)))
theorem okJorKadeen : Ability.check (DRS.ofList []) jorKadeen = [] := by decide
def abandonedOutpost : Ability := static (entersTapped thisLand)
theorem okAbandonedOutpost : Ability.check (DRS.ofList []) abandonedOutpost = [] := by decide
/-- Time Vault -/
def timeVaultLock : Ability := static (doesntUntap thisArtifact (some you))
theorem okTimeVaultLock : Ability.check (DRS.ofList []) timeVaultLock = [] := by decide
def hymnOfRebirth : Action :=
  putOntoBattlefieldUnderYourControl (target (and [isCreature, in_ graveyard]))
theorem okHymnOfRebirth : Action.check (DRS.ofList []) hymnOfRebirth = [] := by decide
def counterspell : Action := counter (target isSpell)
theorem okCounterspell : Action.check (DRS.ofList []) counterspell = [] := by decide

def anthemOfChampions : Spelled := spelled <| .singleFaced
  { characteristics :=
    { name := "Anthem of Champions", cost := some [pip .green, pip .white], types := [.enchantment],
      text := [static (getsPt (allOf (and [isCreature, youControl])) (up (.lit 1)) (up (.lit 1)))] } }

def adantoVanguard : Ability :=
  static (onlyWhile (getsPt thisCreature (up (.lit 2)) (up (.lit 0)))
    (is thisCreature isAttacking))
theorem okAdantoVanguard : Ability.check (DRS.ofList []) adantoVanguard = [] := by decide
/-- Nowhere to Run -/
def nowhereToRunTargetLine : StaticSpec :=
  can (by_ (allOf (or [isSpell, abilityHead .anyOnStack]))) (target (allOf (and [isCreature, opponentsControl]))) (asThough := some (holds (is them (not (hasKeyword hexproof)))))
theorem okNowhereToRunTargetLine : StaticSpec.check (DRS.ofList []) nowhereToRunTargetLine = [] := by decide
/-- Hithlain Rope -/
def hithlainRopeSacrificeLock : StaticSpec := cant any (sacrifice this_)
theorem okHithlainRopeSacrificeLock : StaticSpec.check (DRS.ofList []) hithlainRopeSacrificeLock = [] := by
  decide
/-- Lich's Mastery -/
def lichsMasteryGate : Ability := static (cant you loseGame)
theorem okLichsMasteryGate : Ability.check (DRS.ofList []) lichsMasteryGate = [] := by decide
def theGoldenThrone : Ability :=
  static (replace (would you loseGame) (instead (sequentially [exile thisArtifact, setLife (.lit 1)])))
theorem okTheGoldenThrone : Ability.check (DRS.ofList []) theGoldenThrone = [] := by decide
def stunningReversal : Ability :=
  spell (establishThroughout
           (replace (would you loseGame) (instead (sequentially [draw (.lit 7), setLife (.lit 1)]))
              none .nextTimeOnly) thisTurn)
theorem okStunningReversal : Ability.check (DRS.ofList []) stunningReversal = [] := by decide
def pathOfBravery : Ability :=
  static (onlyWhile (getsPt (allOf (and [isCreature, youControl])) (up (.lit 1)) (up (.lit 1)))
    (compareAmt (lifeTotalOf you) .atLeast (statOf (.playerStat .startingLifeTotal) you)))
theorem okPathOfBravery : Ability.check (DRS.ofList []) pathOfBravery = [] := by decide

def deathsShadow : Spelled := spelled <| .singleFaced
  { characteristics :=
    { name := "Death's Shadow", cost := some [pip .black], types := [.creature],
      subtypes := [avatar],
      text :=
        [ static (conjunction [ getsPt thisCreature (down (letter .x)) (down (letter .x)),
              letterDefinition .x (lifeTotalOf you) ]) ],
      power := stat 13, toughness := stat 13 } }

def spontaneousMutation : Ability :=
  static (conjunction [ getsPt (enchanted creature) (down (letter .x)) (down (.lit 0)),
      letterDefinition .x (countOf (cardIn yourGraveyard)) ])
theorem okSpontaneousMutation : Ability.check (DRS.ofList []) spontaneousMutation = [] := by decide

def maro : Spelled := spelled <| .singleFaced
  { characteristics :=
    { name := "Maro", cost := some [generic 2, pip .green, pip .green], types := [.creature],
      subtypes := [elemental],
      text := [static (ptDefinition thisCreature .bothEach (countOf (cardIn yourHand)))] } }

def peopleOfTheWoods : Spelled := spelled <| .singleFaced
  { characteristics :=
    { name := "People of the Woods", cost := some [pip .green, pip .green], types := [.creature],
      subtypes := [human],
      text :=
        [ static (ptDefinition thisCreature .toughnessAlone
            (countOf (and [hasSubtype forest, youControl]))) ],
      power := stat 1 } }

def scourgeOfTheSkyclaves : Ability :=
  static (ptDefinition thisCreature .bothEach
    (minus (.lit 20) (aggregate .max (.playerStat .lifeTotal) isPlayer)))
theorem okScourgeOfTheSkyclaves : Ability.check (DRS.ofList []) scourgeOfTheSkyclaves = [] := by decide
def aettirAndPriwen : Ability :=
  static (conjunction [ getsBase (equipped creature) (letter .x) (letter .x),
      letterDefinition .x (lifeTotalOf you) ])
theorem okAettirAndPriwen : Ability.check (DRS.ofList []) aettirAndPriwen = [] := by decide

def diminish : Spelled := spelled <| .singleFaced
  { characteristics :=
    { name := "Diminish", cost := some [pip .blue], types := [.instant],
      text :=
        [ spell
            (establishThroughout (getsBase (target isCreature) (.lit 1) (.lit 1))
               untilEndOfTurn) ] } }

def cycleOfLife : Action :=
  establishThroughout (getsBase (target (and [isCreature, castBy you])) (.lit 0)
                         (.lit 1)) (untilEvent (beginningOf .upkeep (some you) nextTime))
theorem okCycleOfLife : Action.check (DRS.ofList []) cycleOfLife = [] := by decide

def aboutFace : Spelled := spelled <| .singleFaced
  { characteristics :=
    { name := "About Face", cost := some [pip .red], types := [.instant],
      text :=
        [ spell (establishThroughout (ptSwitch (target isCreature)) untilEndOfTurn) ] } }

def roilingHorror : Ability :=
  static (ptDefinition thisCreature .bothEach
    (minus (lifeTotalOf you)
      (lifeTotalOf
        (a (and [isOpponent, superlative .max (.playerStat .lifeTotal) isOpponent])))))
theorem okRoilingHorror : Ability.check (DRS.ofList []) roilingHorror = [] := by decide

/-- Katara, the Fearless -/
def kataraTheFearless : Spelled := spelled <| .singleFaced
  { characteristics :=
    { name := "Katara, the Fearless", cost := some [pip .green, pip .white, pip .blue],
      supertypes := [.legendary], types := [.creature],
      subtypes := [human, warrior, ally],
      text :=
        [ static (additionalTriggers
            (triggers (a (and [ abilityHead .anyTriggered,
                         abilityOf (a (and [ hasSubtype ally,
                                               youControl ])) ])))
            (exactly 1)) ],
      power := stat 3, toughness := stat 3 } }

/-- Rain of Gore -/
def rainOfGore : Spelled := spelled <| .singleFaced
  { characteristics :=
    { name := "Rain of Gore", cost := some [pip .black, pip .red], types := [.enchantment],
      text :=
        [ static (replace
      (causedBy
        (bySource (a (or [isSpell,
          abilityHead .anyOnStack])))
        (would
          (controllerOf it)
          (gainLife anyAmount)))
      (instead (instruct (that player) (loseLife thatMuch)))) ] } }

/-- Master Chef -/
def masterChefGrantedAbility : Ability :=
  static (entersWithAdditionalCounters thisCreature (.lit 1) p1p1Counter)
theorem okMasterChefGrantedAbility : Ability.check (DRS.ofList []) masterChefGrantedAbility = [] := by decide

def anointedProcession : Spelled := spelled <| .singleFaced
  { characteristics :=
    { name := "Anointed Procession", cost := some [generic 3, pip .white], types := [.enchantment],
      text :=
        [ static (replace (would any (createObject anyAmount (token (copyOf (counted (atLeast 1) isToken)) []) true (some you))) (instead (create (times (.lit 2) groupSize) asThose []))) ] } }

def naturalAffinity : Spelled := spelled <| .singleFaced
  { characteristics :=
    { name := "Natural Affinity", cost := some [generic 2, pip .green], types := [.instant],
      text :=
        [ spell (establishThroughout (qualityChange (allOf isLand) .sets
              (bundle { characteristics := { types := [.creature], power := stat 2, toughness := stat 2 } }
                (some .land))) untilEndOfTurn) ] } }

def turnToFrog : Spelled := spelled <| .singleFaced
  { characteristics :=
    { name := "Turn to Frog", cost := some [generic 1, pip .blue], types := [.instant],
      text :=
        [ spell (establishThroughout (conjunction [ allAbilityLoss (target isCreature),
                qualityChange it .sets
                  (bundle { characteristics := { colors := [.blue], subtypes := [frog] } }),
                getsBase it (.lit 1) (.lit 1) ]) untilEndOfTurn) ] } }

def humility : Spelled := spelled <| .singleFaced
  { characteristics :=
    { name := "Humility", cost := some [generic 2, pip .white, pip .white], types := [.enchantment],
      text :=
        [ static (conjunction [ allAbilityLoss (allOf isCreature),
              getsBase them (.lit 1) (.lit 1) ]) ] } }

/-- Tahngarth, First Mate -/
def tahngarthAttacksThatJoin : Action := becomeAttacking thisCreature (some thatJoin)
theorem okTahngarthAttacksThatJoin :
    Action.check (nextClause ((stepAction 512 .postEvent .obj ((stepGameEvent 512 (DRS.ofList []) tahngarthHeader).1) tahngarthChoosesDefender).1))
      tahngarthAttacksThatJoin = [] := by
  decide

def mindlockOrb : Spelled := spelled <| .singleFaced
  { characteristics :=
    { name := "Mindlock Orb", cost := some [generic 3, pip .blue], types := [.artifact],
      text := [static (cant (bare isPlayer) (search any anyNumber (and [])))] } }

def silence : Spelled := spelled <| .singleFaced
  { characteristics :=
    { name := "Silence", cost := some [pip .white], types := [.instant],
      text :=
        [ spell (establishThroughout (cant yourOpponents (cast any)) thisTurn) ] } }

def shadowOfDoubt : Spelled := spelled <| .singleFaced
  { characteristics :=
    { name := "Shadow of Doubt", cost := some [hybridPip .blue .black, hybridPip .blue .black],
      types := [.instant],
      text :=
        [ spell (sequentially
            [ establishThroughout (cant (bare isPlayer) (search any anyNumber (and [])))
                thisTurn,
              draw (.lit 1) ]) ] } }

def omenMachineDraw : Ability := static (cant (bare isPlayer) (draw anyAmount))
theorem okOmenMachineDraw : Ability.check (DRS.ofList []) omenMachineDraw = [] := by decide
def solfataraLandLock : Action :=
  establishThroughout (cant (target isPlayer) (play any)) thisTurn
theorem okSolfataraLandLock : Action.check (DRS.ofList []) solfataraLandLock = [] := by decide

/-- "each nonbasic land is a <type>": a moon effect. -/
def nonbasicLandsAre (type : Subtype) : StaticSpec :=
  qualityChange (allOf (and [isLand, non (hasSupertype .basic)])) .sets
    (bundle { characteristics := { subtypes := [type] } })

def bloodMoon : Spelled := spelled <| .singleFaced
  { characteristics :=
    { name := "Blood Moon", cost := some [generic 2, pip .red], types := [.enchantment],
      text := [static (nonbasicLandsAre mountain)] } }

def magusOfTheMoon : Spelled := spelled <| .singleFaced
  { characteristics :=
    { name := "Magus of the Moon", cost := some [generic 2, pip .red], types := [.creature],
      subtypes := [human, wizard],
      text := [static (nonbasicLandsAre mountain)], power := stat 2, toughness := stat 2 } }

def harbingerOfTheSeas : Spelled := spelled <| .singleFaced
  { characteristics :=
    { name := "Harbinger of the Seas", cost := some [generic 1, pip .blue, pip .blue],
      types := [.creature], subtypes := [merfolk, wizard],
      text := [static (nonbasicLandsAre island)], power := stat 2, toughness := stat 2 } }

def yavimayaCradleOfGrowth : Spelled := spelled <| .singleFaced
  { characteristics :=
    { name := "Yavimaya, Cradle of Growth", supertypes := [.legendary], types := [.land],
      text :=
        [ static (addSubtype (allOf isLand) forest) ] } }

def rallyTheRanks : Spelled := spelled <| .singleFaced
  { characteristics :=
    { name := "Rally the Ranks", cost := some [generic 1, pip .white], types := [.enchantment],
      text :=
        [ static (entersChoosing thisEnchantment (.subtype .creature)),
          static (getsPt
            (allOf (and [isCreature, youControl, ofChosen (.subtype .creature)]))
            (up (.lit 1)) (up (.lit 1))) ] } }

def sharedTriumph : Spelled := spelled <| .singleFaced
  { characteristics :=
    { name := "Shared Triumph", cost := some [generic 1, pip .white], types := [.enchantment],
      text :=
        [ static (entersChoosing thisEnchantment (.subtype .creature)),
          static (getsPt (allOf (and [isCreature, ofChosen (.subtype .creature)]))
            (up (.lit 1)) (up (.lit 1))) ] } }

def hallOfTriumph : Spelled := spelled <| .singleFaced
  { characteristics :=
    { name := "Hall of Triumph", cost := some [generic 3], supertypes := [.legendary],
      types := [.artifact],
      text :=
        [ static (entersChoosing thisArtifact .color),
          static (getsPt (allOf (and [isCreature, youControl, ofChosen .color]))
            (up (.lit 1)) (up (.lit 1))) ] } }

def engineeredPlague : Spelled := spelled <| .singleFaced
  { characteristics :=
    { name := "Engineered Plague", cost := some [generic 2, pip .black], types := [.enchantment],
      text :=
        [ static (entersChoosing thisEnchantment (.subtype .creature)),
          static (getsPt (allOf (and [isCreature, ofChosen (.subtype .creature)]))
            (down (.lit 1)) (down (.lit 1))) ] } }

/-- Volrath's Laboratory -/
def volrathsLaboratoryChoice : StaticSpec :=
  conjunction [entersChoosing thisArtifact .color, entersChoosing thisArtifact (.subtype .creature)]
theorem okVolrathsLaboratoryChoice : StaticSpec.check (DRS.ofList []) volrathsLaboratoryChoice = [] := by decide
/-- Call to Arms -/
def callToArmsChoice : StaticSpec :=
  conjunction [ entersChoosing thisEnchantment .color,
      entersChoosingPlayer thisEnchantment anOpponent ]
theorem okCallToArmsChoice : StaticSpec.check (DRS.ofList []) callToArmsChoice = [] := by decide

def encroachingMycosynth : Spelled := spelled <| .singleFaced
  { characteristics :=
    { name := "Encroaching Mycosynth", cost := some [generic 3, pip .blue], types := [.artifact],
      text :=
        [ static (qualityChange (and [
              allOf (and [isPermanent, non isLand, youControl]),
              allOf (and
                       [or [isArtifact, isBattle, isCreature, isEnchantment, isPlaneswalker],
                        isSpell, youControl]),
              allOf (and [isCard, isPermanentCard, non isLand, youOwn, not (in_ battlefield)]) ])
                .adds
              (bundle { characteristics := { types := [.artifact] } })) ] } }

/-- Darkest Hour -/
def darkestHour : Spelled := spelled <| .singleFaced
  { characteristics :=
    { name := "Darkest Hour", cost := some [pip .black], types := [.enchantment],
      text := [static (qualityChange (allOf isCreature) .sets (colored (.some [.black])))] } }

/-- Thran Lens -/
def thranLens : Spelled := spelled <| .singleFaced
  { characteristics :=
    { name := "Thran Lens", cost := some [generic 2], types := [.artifact],
      text := [static (qualityChange (allOf isPermanent) .sets (colored (.some [])))] } }

/-- Ghostflame Sliver -/
def ghostflameSliver : Spelled := spelled <| .singleFaced
  { characteristics :=
    { name := "Ghostflame Sliver", cost := some [pip .black, pip .red], types := [.creature],
      subtypes := [sliver],
      text :=
        [ static (qualityChange (allOf (hasSubtype sliver)) .sets (colored
            (.some []))) ],
      power := stat 2, toughness := stat 2 } }

/-- Nightcreep -/
def nightcreep : Spelled := spelled <| .singleFaced
  { characteristics :=
    { name := "Nightcreep", cost := some [pip .black, pip .black], types := [.instant],
      text :=
        [ spell
            (establishThroughout
               (conjunction [ qualityChange (allOf isCreature) .sets (colored (.some [.black])),
                qualityChange (allOf isLand) .sets
                  (bundle { characteristics := { subtypes := [swamp] } }) ]) untilEndOfTurn) ] } }

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
def transguildCourier : Spelled := spelled <| .singleFaced
  { characteristics :=
    { name := "Transguild Courier", cost := some [generic 4], types := [.artifact, .creature],
      subtypes := [golem],
      text := [static (qualityDefinition thisCreature .sets (colored .every))],
      power := stat 3, toughness := stat 3 } }

/-- Booby Trap -/
def boobyTrapNameChoice : StaticSpec :=
  entersChoosingFrom thisArtifact .cardName (nameOfCard (not (and [hasSupertype .basic, isLand])))
theorem okBoobyTrapNameChoice : StaticSpec.check (DRS.ofList []) boobyTrapNameChoice = [] := by decide

def dovinsVeto : Spelled := spelled <| .singleFaced
  { characteristics :=
    { name := "Dovin's Veto", cost := some [pip .white, pip .blue], types := [.instant],
      text :=
        [ static (cant any (counter thisSpell)),
          spell (counter (target (and [isSpell, non isCreature]))) ] } }

def goblinSpy : Spelled := spelled <| .singleFaced
  { characteristics :=
    { name := "Goblin Spy", cost := some [pip .red], types := [.creature],
      subtypes := [goblin, rogue],
      text := [static (visibility .reveal you topOfLibrary)],
      power := stat 1, toughness := stat 1 } }

def futureSight : Spelled := spelled <| .singleFaced
  { characteristics :=
    { name := "Future Sight", cost := some [generic 2, pip .blue, pip .blue, pip .blue],
      types := [.enchantment],
      text :=
        [ static (visibility .reveal you topOfLibrary),
          static playLandsAndCastSpellsFromTop ] } }

def magusOfTheFuture : Spelled := spelled <| .singleFaced
  { characteristics :=
    { name := "Magus of the Future", cost := some [generic 2, pip .blue, pip .blue, pip .blue],
      types := [.creature], subtypes := [human, wizard],
      text :=
        [ static (visibility .reveal you topOfLibrary),
          static playLandsAndCastSpellsFromTop ],
      power := stat 2, toughness := stat 3 } }

def telepathy : Spelled := spelled <| .singleFaced
  { characteristics :=
    { name := "Telepathy", cost := some [pip .blue], types := [.enchantment],
      text := [static (visibility .reveal yourOpponents wholeHand)] } }

def assembleThePlayers : Spelled := spelled <| .singleFaced
  { characteristics :=
    { name := "Assemble the Players", cost := some [generic 1, pip .white], types := [.enchantment],
      text :=
        [ static (visibility .lookAt you topOfLibrary),
          static castSmallCreatureFromTopOnceEachTurn ] } }

def prismaticOmen : Spelled := spelled <| .singleFaced
  { characteristics :=
    { name := "Prismatic Omen", cost := some [generic 1, pip .green], types := [.enchantment],
      text :=
        [ static (qualityChange (allOf (and [isLand, youControl])) .adds
            (everyTypeOf .basicLand)) ] } }

def mistformUltimus : Spelled := spelled <| .singleFaced
  { characteristics :=
    { name := "Mistform Ultimus", cost := some [generic 3, pip .blue], supertypes := [.legendary],
      types := [.creature], subtypes := [illusion],
      text := [static (qualityDefinition thisCreature .adds (everyTypeOf .creature))],
      power := stat 3, toughness := stat 3 } }

def volatileClaws : Spelled := spelled <| .singleFaced
  { characteristics :=
    { name := "Volatile Claws", cost := some [generic 2, pip .red], types := [.instant],
      text :=
        [ spell
            (establishFor (allOf (and [isCreature, youControl]))
              [ getsPt them (up (.lit 2)) (up (.lit 0)),
                qualityChange them .adds (everyTypeOf .creature) ]
              (some untilEndOfTurn)) ] } }

/-- Nameless Inversion -/
def namelessInversionBody : Action :=
  establishFor (target isCreature)
    [getsPt it (up (.lit 3)) (down (.lit 3)), qualityChange it .loses (everyTypeOf .creature)]
    (some untilEndOfTurn)
theorem okNamelessInversionBody : Action.check (DRS.ofList []) namelessInversionBody = [] := by decide
/-- Ego Erasure -/
def egoErasureBody : Action :=
  establishFor (allOf (and [isCreature, controlledBy (target isPlayer)]))
    [getsPt them (down (.lit 2)) (up (.lit 0)), qualityChange them .loses (everyTypeOf .creature)]
    (some untilEndOfTurn)
theorem okEgoErasureBody : Action.check (DRS.ofList []) egoErasureBody = [] := by decide
/-- Lithoform Blight -/
def lithoformBlightLoss : StaticSpec :=
  conjunction [ qualityChange (enchanted land) .loses (everyTypeOf .land),
      allAbilityLoss it ]
theorem okLithoformBlightLoss : StaticSpec.check (DRS.ofList []) lithoformBlightLoss = [] := by decide

/-- Energybending -/
def energybending : Spelled := spelled <| .singleFaced
  { characteristics :=
    { name := "Energybending", cost := some [generic 2], types := [.instant],
      subtypes := [lesson],
      text :=
        [ spell (sequentially
            [ establishThroughout (qualityChange (allOf (and [isLand, youControl])) .adds
                  (everyTypeOf .basicLand)) untilEndOfTurn,
              draw (.lit 1) ]) ] } }

/-- Seedborn Muse -/
def seedbornMuse : Spelled := spelled <| .singleFaced
  { characteristics :=
    { name := "Seedborn Muse", cost := some [generic 3, pip .green, pip .green],
      types := [.creature], subtypes := [spirit],
      text :=
        [ static (untapDuring (allOf youControl) (some (each isOtherPlayer))) ],
      power := stat 2, toughness := stat 4 } }

/-- Unwinding Clock -/
def unwindingClock : Spelled := spelled <| .singleFaced
  { characteristics :=
    { name := "Unwinding Clock", cost := some [generic 4], types := [.artifact],
      text :=
        [ static (untapDuring (allOf (and [isArtifact, youControl]))
            (some (each isOtherPlayer))) ] } }

/-- Thousand Moons Infantry -/
def thousandMoonsInfantry : Spelled := spelled <| .singleFaced
  { characteristics :=
    { name := "Thousand Moons Infantry", cost := some [generic 2, pip .white], types := [.creature],
      subtypes := [human, soldier],
      text := [static (untapDuring thisCreature (some (each isOtherPlayer)))],
      power := stat 2, toughness := stat 4 } }

/-- Blatant Thievery -/
def blatantThievery : Spelled := spelled <| .singleFaced
  { characteristics :=
    { name := "Blatant Thievery", cost := some [generic 4, pip .blue, pip .blue, pip .blue],
      types := [.sorcery],
      text :=
        [ spell (doForEach eachOpponent
            (gainControl (target (controlledBy (that player))) (controller := you))) ] } }

/-- "activated abilities of <p> can't be activated" -/
def cantActivateAbilitiesOf (p : Predicate) : StaticSpec :=
  cant any (activate (allOf (and [abilityHead .anyActivated, abilityOf (allOf p)])))

def cursedTotem : Spelled := spelled <| .singleFaced
  { characteristics :=
    { name := "Cursed Totem", cost := some [generic 2], types := [.artifact],
      text := [static (cantActivateAbilitiesOf isCreature)] } }

def nullRod : Spelled := spelled <| .singleFaced
  { characteristics :=
    { name := "Null Rod", cost := some [generic 2], types := [.artifact],
      text := [static (cantActivateAbilitiesOf isArtifact)] } }

def stonySilence : Spelled := spelled <| .singleFaced
  { characteristics :=
    { name := "Stony Silence", cost := some [generic 1, pip .white], types := [.enchantment],
      text := [static (cantActivateAbilitiesOf isArtifact)] } }

def dampingMatrix : Spelled := spelled <| .singleFaced
  { characteristics :=
    { name := "Damping Matrix", cost := some [generic 3], types := [.artifact],
      text :=
        [ static (cant any (activate (allOf (and [ abilityHead .anyActivated,
                           abilityOf (allOf (or [isArtifact, isCreature])),
                           not isManaAbility ])))) ] } }

def crash : Spelled := spelled <| .singleFaced
  { characteristics :=
    { name := "Crash", cost := some [generic 2, pip .red], types := [.instant],
      text :=
        [ static (altCost this_
            (some (perform (sacrifice (a (and [isLand, hasSubtype mountain])))))),
          spell (destroy (target isArtifact)) ] } }

def moggSalvage : Spelled := spelled <| .singleFaced
  { characteristics :=
    { name := "Mogg Salvage", cost := some [generic 2, pip .red], types := [.instant],
      text :=
        [ static (onlyWhile (altCost this_)
            (and
              [ exists_ (and [ isLand, hasSubtype island,
                                anOpponentControls ]),
                exists_ (and [ isLand, hasSubtype mountain,
                                youControl ]) ])),
          spell (destroy (target isArtifact)) ] } }

def abolish : Spelled := spelled <| .singleFaced
  { characteristics :=
    { name := "Abolish", cost := some [generic 1, pip .white, pip .white], types := [.instant],
      text :=
        [ static (altCost this_
            (some (perform (discard (a (and [hasSubtype plains, in_ yourHand])))))),
          spell (destroy (target (or [isArtifact, isEnchantment]))) ] } }

def gush : Spelled := spelled <| .singleFaced
  { characteristics :=
    { name := "Gush", cost := some [generic 4, pip .blue], types := [.instant],
      text :=
        [ static (altCost this_
            (some (perform (returnToHand
              (counted (exactly 2)
                (and [isLand, hasSubtype island, youControl])))))),
          spell (draw (.lit 2)) ] } }

def sunscour : Spelled := spelled <| .singleFaced
  { characteristics :=
    { name := "Sunscour", cost := some [generic 5, pip .white, pip .white], types := [.sorcery],
      text :=
        [ static (altCost this_
            (some (perform (exile
              (counted (exactly 2) (and [hasColor .white, in_ yourHand])))))),
          spell (destroy (allOf isCreature)) ] } }

def massacre : Spelled := spelled <| .singleFaced
  { characteristics :=
    { name := "Massacre", cost := some [generic 2, pip .black, pip .black], types := [.sorcery],
      text :=
        [ static (onlyWhile (altCost this_)
            (and
              [ exists_ (and [ isLand, hasSubtype plains,
                                anOpponentControls ]),
                exists_ (and [ isLand, hasSubtype swamp,
                                youControl ]) ])),
          spell (get (allOf isCreature) (down (.lit 2)) (down (.lit 2)) (some untilEndOfTurn))
              ] } }

def rouse : Spelled := spelled <| .singleFaced
  { characteristics :=
    { name := "Rouse", cost := some [generic 1, pip .black], types := [.instant],
      text :=
        [ static (onlyWhile (altCost this_ (some (payLife 2)))
            (exists_ (and [isLand, hasSubtype swamp, youControl]))),
          spell (get (target isCreature) (up (.lit 2)) (up (.lit 0)) (some untilEndOfTurn)) ]
              } }

def thwart : Spelled := spelled <| .singleFaced
  { characteristics :=
    { name := "Thwart", cost := some [generic 2, pip .blue, pip .blue], types := [.instant],
      text :=
        [ static (altCost this_
            (some (perform (returnToHand
              (counted (exactly 3)
                (and [isLand, hasSubtype island, youControl])))))),
          spell (counter (target isSpell)) ] } }

def theLadyOfOtariaLine : StaticSpec :=
  altCost this_
    (some (perform (tap
      (counted (exactly 3)
        (and [ isCreature, hasSubtype dwarf, youControl,
                isUntapped ])))))
theorem okTheLadyOfOtariaLine : StaticSpec.check (DRS.ofList []) theLadyOfOtariaLine = [] := by decide

/-- Clergy of the Holy Nimbus -/
def clergyOfTheHolyNimbus : Ability :=
  static (replace (would any (destroy thisCreature)) (instead (regeneration it)))
theorem okClergyOfTheHolyNimbus : Ability.check (DRS.ofList []) clergyOfTheHolyNimbus = [] := by decide
/-- Rampant Frogantua -/
def rampantFrogantuaPump : Ability :=
  static (getsPt thisCreature
    (up (forEach (.lit 10) (and [isPlayer,
      did which loseGame thisGame])))
    (up (forEach (.lit 10) (and [isPlayer,
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
  static (entersWithFewerCounters thisPlaneswalker (.lit 2) loyaltyCounter)
theorem okNahiriCompleatedEntry : Ability.check (DRS.ofList []) nahiriCompleatedEntry = [] := by decide
/-- Nimble Mongoose -/
def nimbleMongoose : Ability :=
  threshold
    (static (onlyWhile (getsPt thisCreature (up (.lit 2)) (up (.lit 2)))
      (compareAmt (countOf (cardIn yourGraveyard)) .atLeast (.lit 7))))
theorem okNimbleMongoose : Ability.check (DRS.ofList []) nimbleMongoose = [] := by decide
/-- Anax, Hardened in the Forge -/
def anaxPowerDefinition : Ability :=
  static (ptDefinition thisCreature .powerAlone (devotion you (color .red)))
theorem okAnaxPowerDefinition : Ability.check (DRS.ofList []) anaxPowerDefinition = [] := by decide
/-- Aspect of Wolf -/
def aspectOfWolf : Ability :=
  static (conjunction [ getsPt (enchanted creature) (up (letter .x)) (up (letter .y)),
      letterDefinition .x (half .down
        (countOf (and [hasSubtype forest, youControl]))),
      letterDefinition .y (half .up
        (countOf (and [hasSubtype forest, youControl]))) ])
theorem okAspectOfWolf : Ability.check (DRS.ofList []) aspectOfWolf = [] := by decide
/-- Lhurgoyf -/
def lhurgoyfDefinition : Ability :=
  static (ptDefinitions thisCreature (countOf (and [isCreature, in_ graveyard]))
    (plus thatMuch (.lit 1)))
theorem okLhurgoyfDefinition : Ability.check (DRS.ofList []) lhurgoyfDefinition = [] := by decide

/-- Invigorate -/
def invigorate : Spelled := spelled <| .singleFaced
  { characteristics :=
    { name := "Invigorate", cost := some [generic 2, pip .green], types := [.instant],
      text :=
        [ static (onlyWhile (altCost this_ (some (perform (instruct anOpponent (gainLife (.lit 3))))))
            (exists_ (and [isLand, hasSubtype forest, youControl]))),
          spell (get (target isCreature) (up (.lit 4)) (up (.lit 4)) (some untilEndOfTurn)) ]
              } }

/-- Deflecting Swat -/
def deflectingSwatCommanderAltCost : Ability :=
  static (onlyWhile (altCost this_)
    (exists_ (and [hasDesignation commander, youControl])))
theorem okDeflectingSwatCommanderAltCost :
    Ability.check (DRS.ofList []) deflectingSwatCommanderAltCost = [] := by decide

/-- Fist of Suns -/
def fistOfSuns : Spelled := spelled <| .singleFaced
  { characteristics :=
    { name := "Fist of Suns", cost := some [generic 3], types := [.artifact],
      text :=
        [ static (altCost (allOf (and [isSpell, castBy you]))
            (some (mana [pip .white, pip .blue, pip .black, pip .red, pip .green]))) ] } }

/-- Rooftop Storm -/
def rooftopStorm : Spelled := spelled <| .singleFaced
  { characteristics :=
    { name := "Rooftop Storm", cost := some [generic 5, pip .blue], types := [.enchantment],
      text :=
        [ static (altCost
            (allOf (and [hasSubtype zombie, isCreature, isSpell, castBy you]))
            (some (mana [generic 0]))) ] } }

/-- Voltage Surge's declaration -/
def voltageSurgeAddedCost : Ability :=
  static (addedCost (perform (sacrifice (a isArtifact))) true)
theorem okVoltageSurgeAddedCost : Ability.check (DRS.ofList []) voltageSurgeAddedCost = [] := by decide
/-- Tarmogoyf -/
def tarmogoyfDefinition : Ability :=
  static (ptDefinitions thisCreature
    (distinctCount .cardType (allOf (cardIn (graveyardOf (bare isPlayer)))))
    (plus thatMuch (.lit 1)))
theorem okTarmogoyfDefinition : Ability.check (DRS.ofList []) tarmogoyfDefinition = [] := by decide
/-- Consuming Blob's definition -/
def consumingBlobDefinition : Ability :=
  static (ptDefinitions thisCreature (distinctCount .cardType (allOf (cardIn yourGraveyard)))
    (plus thatMuch (.lit 1)))
theorem okConsumingBlobDefinition : Ability.check (DRS.ofList []) consumingBlobDefinition = [] := by decide
/-- Nighthawk Scavenger's definition -/
def nighthawkScavengerDefinition : Ability :=
  static (ptDefinition thisCreature .powerAlone
    (plus (.lit 1)
      (distinctCount .cardType (allOf (cardIn (graveyardOf yourOpponents))))))
theorem okNighthawkScavengerDefinition : Ability.check (DRS.ofList []) nighthawkScavengerDefinition = [] := by
  decide
/-- Faeburrow Elder's pump -/
def faeburrowElderPump : Ability :=
  static (getsPt thisCreature
    (up (times (.lit 1)
      (distinctCount .color (allOf (and [isPermanent, youControl])))))
    (up (times (.lit 1)
      (distinctCount .color (allOf (and [isPermanent, youControl]))))))
theorem okFaeburrowElderPump : Ability.check (DRS.ofList []) faeburrowElderPump = [] := by decide
/-- Bonds of Faith -/
def bondsOfFaithPump : Ability :=
  static (onlyWhile
    (getsPt (enchanted creature) (up (.lit 2)) (up (.lit 2)))
    (is it (hasSubtype human)))
theorem okBondsOfFaithPump : Ability.check (DRS.ofList []) bondsOfFaithPump = [] := by decide

/-- Field of Dreams -/
def fieldOfDreams : Spelled := spelled <| .singleFaced
  { characteristics :=
    { name := "Field of Dreams", cost := some [pip .blue], supertypes := [.world],
      types := [.enchantment],
      text := [static (visibility .reveal (bare isPlayer) topOfLibrary)] } }

/-- Lantern of Insight -/
def lanternOfInsightRider : Ability :=
  static (visibility .reveal (bare isPlayer) topOfLibrary)
theorem okLanternOfInsightRider : Ability.check (DRS.ofList []) lanternOfInsightRider = [] := by decide

/-- Revelation -/
def revelation : Spelled := spelled <| .singleFaced
  { characteristics :=
    { name := "Revelation", cost := some [pip .green], supertypes := [.world], types := [.enchantment],
      text := [static (visibility .reveal (bare isPlayer) wholeHand)] } }

/-- Phyrexian Unlife -/
def phyrexianUnlifeImmunity : Ability :=
  static (cant you (loseGame (some .nonpositiveLife)))
theorem okPhyrexianUnlifeImmunity : Ability.check (DRS.ofList []) phyrexianUnlifeImmunity = [] := by decide
/-- Umbris, Fear Manifest -/
def umbrisPump : Ability :=
  static (getsPt thisCreature
    (up (forEach (.lit 1)
      (and [isCard, opponentsOwn, in_ exileZone])))
    (up (forEach (.lit 1)
      (and [isCard, opponentsOwn, in_ exileZone]))))
theorem okUmbrisPump : Ability.check (DRS.ofList []) umbrisPump = [] := by decide
/-- Turbulent Fen -/
def turbulentFen : Ability :=
  static (entersTappedUnless thisLand
    (compareAmt (countOf (and [isLand, opponentsControl]))
      .atLeast (.lit 8)))
theorem okTurbulentFen : Ability.check (DRS.ofList []) turbulentFen = [] := by decide
/-- Bastion Protector -/
def bastionProtectorPump : Ability :=
  static (getsPt
    (allOf (and [isCreature, hasDesignation commander, youControl]))
    (up (.lit 2)) (up (.lit 2)))
theorem okBastionProtectorPump : Ability.check (DRS.ofList []) bastionProtectorPump = [] := by decide
/-- Luxior, Giada's Gift -/
def luxiorTypeSetting : Ability :=
  static (conjunction [ qualityChange (equipped permanent) .loses
        (bundle { characteristics := { types := [.planeswalker] } }),
      qualityChange it .adds (bundle { characteristics := { types := [.creature] } }) ])
theorem okLuxiorTypeSetting : Ability.check (DRS.ofList []) luxiorTypeSetting = [] := by decide
/-- Kasmina, Enigma Sage -/
def kasminaLoyaltySharing : StaticSpec :=
  abilityGrantFrom (allOf (and [isPlaneswalker, youControl, otherThan this_])) this_ [.loyalty]
theorem okKasminaLoyaltySharing : StaticSpec.check (DRS.ofList []) kasminaLoyaltySharing = [] := by decide
/-- Nicol Bolas, Dragon-God -/
def nicolBolasDragonGodSharing : StaticSpec :=
  abilityGrantFrom this_ (allOf
                             (and [isPlaneswalker, otherThan this_, in_ battlefield])) [.loyalty]
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
def skillBorrower : Spelled := spelled <| .singleFaced
  { characteristics :=
    { name := "Skill Borrower", cost := some [generic 2, pip .blue], types := [.artifact, .creature],
      subtypes := [human, wizard],
      text :=
        [ static (visibility .reveal you topOfLibrary),
          static (onlyWhile
            (abilityGrantFrom thisCreature (topSlice (.lit 1)) [.anyActivated])
            (is (topSlice (.lit 1)) (or [isArtifact, isCreature]))) ],
      power := stat 1, toughness := stat 3 } }

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

def counterspellCard : Spelled := spelled <| .singleFaced
  { characteristics :=
    { name := "Counterspell", cost := some [pip .blue, pip .blue], types := [.instant],
      text := [spell counterspell] } }

def hymnOfRebirthCard : Spelled := spelled <| .singleFaced
  { characteristics :=
    { name := "Hymn of Rebirth", cost := some [generic 3, pip .green, pip .white],
      types := [.sorcery], text := [spell hymnOfRebirth] } }

def chandrasPyrohelixCard : Spelled := spelled <| .singleFaced
  { characteristics :=
    { name := "Chandra's Pyrohelix", cost := some [generic 1, pip .red], types := [.instant],
      text := [spell forkedBolt] } }

/-- Smite -/
def smite : Spelled := spelled <| .singleFaced
  { characteristics :=
    { name := "Smite", cost := some [pip .white], types := [.instant],
      text := [spell (destroy (target (and [isCreature, isBlocked])))] } }

def eerieUltimatum : Spelled := spelled <| .singleFaced
  { characteristics :=
    { name := "Eerie Ultimatum",
      cost := some [pip .white, pip .white, pip .black, pip .black, pip .black, pip .green, pip .green],
      types := [.sorcery],
      text :=
        [ spell (returnToBattlefield (withDifferentNames
              (counted anyNumber (and [isPermanentCard, in_ yourGraveyard])))) ] } }

/-- Gray Merchant of Asphodel -/
def grayMerchantDrain : Action :=
  sequentially
    [instruct eachOpponent (loseLife (letter .x)), define .x (devotion you (color .black))]
theorem okGrayMerchantDrain : Action.check (DRS.ofList []) grayMerchantDrain = [] := by decide

end Semantics.Cards
