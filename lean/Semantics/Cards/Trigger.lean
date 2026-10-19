import Semantics
import Semantics.Definitions
import Semantics.Check.Card
import Semantics.Cards.Anaphora

/-!
# Semantics.Cards.Trigger

Port of `idris/src/Experimental/Cards/Trigger.idr`: the printed cards of the Trigger family
(headers, intervening-ifs, joined and alternative headers, delayed and replacement shapes) and
the bench items beside them.
-/

open Semantics Semantics.Templates Semantics.Definitions

namespace Semantics.Cards

def gracefulReprieve : Action :=
  delayWithin (does (target isCreature) die) thisTurn (putOntoBattlefield (that card))
theorem okGracefulReprieve : Action.check (DRS.ofList []) gracefulReprieve = [] := by decide

def cloudkinSeer : Ability := when (does thisCreature enter) (draw (.lit 1))
theorem okCloudkinSeer : Ability.check (DRS.ofList []) cloudkinSeer = [] := by decide
def promiseOfTomorrow : Ability := whenever (does (a (and [isCreature, youControl])) die) (exile it)
theorem okPromiseOfTomorrow : Ability.check (DRS.ofList []) promiseOfTomorrow = [] := by decide
def libraryLarcenist : Ability := whenever (does thisCreature attack) (draw (.lit 1))
theorem okLibraryLarcenist : Ability.check (DRS.ofList []) libraryLarcenist = [] := by decide
def jhessianThief : Ability :=
  whenever (does thisCreature (dealDamage (to := a isPlayer) (kind := .combatOnly))) (draw (.lit 1))
theorem okJhessianThief : Ability.check (DRS.ofList []) jhessianThief = [] := by decide
def scholarOfStars : Ability :=
  triggeredIf (does thisCreature enter)
    (exists_ (and [isArtifact, youControl])) (draw (.lit 1))
theorem okScholarOfStars : Ability.check (DRS.ofList []) scholarOfStars = [] := by decide

def miserysShadow : Ability :=
  static (instead
            (would (a (and [isCreature, anOpponentControls])) die) (exile it))
theorem okMiserysShadow : Ability.check (DRS.ofList []) miserysShadow = [] := by decide

def beastWhisperer : Ability :=
  whenever (does you (cast (a (and
    [isCreature, isSpell])))) (draw (.lit 1))
theorem okBeastWhisperer : Ability.check (DRS.ofList []) beastWhisperer = [] := by decide
def mesmericOrb : Ability :=
  whenever (does (a isPermanent) (become untapped))
    (instruct (controllerOf (that permanent)) (mill (.lit 1)))
theorem okMesmericOrb : Ability.check (DRS.ofList []) mesmericOrb = [] := by decide
def secretPlans : Ability :=
  whenever (does any (turnFaceUp (a (and [isPermanent, youControl]))))
    (draw (.lit 1))
theorem okSecretPlans : Ability.check (DRS.ofList []) secretPlans = [] := by decide

/-- Teferi's Imp -/
def teferisImpPhasesOut : Ability :=
  whenever (does thisCreature phaseOut) (discard (a (cardIn yourHand)))
theorem okTeferisImpPhasesOut : Ability.check (DRS.ofList []) teferisImpPhasesOut = [] := by decide
/-- Teferi's Imp -/
def teferisImpPhasesIn : Ability :=
  whenever (does thisCreature phaseIn) (draw (.lit 1))
theorem okTeferisImpPhasesIn : Ability.check (DRS.ofList []) teferisImpPhasesIn = [] := by decide

/-- Oubliette -/
def oubliette : Ability :=
  when (does thisEnchantment enter)
    (phaseOutUntil (target isCreature) (does thisEnchantment leave))
theorem okOubliette : Ability.check (DRS.ofList []) oubliette = [] := by decide

def shimmeringEfreet : Ability :=
  whenever (does thisCreature phaseIn) (instruct (target isCreature) phaseOut)
theorem okShimmeringEfreet : Ability.check (DRS.ofList []) shimmeringEfreet = [] := by decide
def hollowhengeSpirit : Ability :=
  when (does thisCreature enter)
    (removeFromCombat (target (and [isCreature, or [isAttacking, isBlocking]])))
theorem okHollowhengeSpirit : Ability.check (DRS.ofList []) hollowhengeSpirit = [] := by decide
def netcasterSpider : Ability :=
  whenever (does thisCreature (block (a (and [isCreature, hasKeyword flying]))))
    (get thisCreature (up (.lit 2)) (up (.lit 0)) (some untilEndOfTurn))
theorem okNetcasterSpider : Ability.check (DRS.ofList []) netcasterSpider = [] := by decide
def viashinoWeaponsmith : Ability :=
  whenever (does (by_ (a isCreature)) (block thisCreature))
    (get thisCreature (up (.lit 2)) (up (.lit 2)) (some untilEndOfTurn))
theorem okViashinoWeaponsmith : Ability.check (DRS.ofList []) viashinoWeaponsmith = [] := by decide
def somberwaldAlpha : Ability :=
  whenever (does any (block (a (and [isCreature, youControl]))))
    (get it (up (.lit 1)) (up (.lit 1)) (some untilEndOfTurn))
theorem okSomberwaldAlpha : Ability.check (DRS.ofList []) somberwaldAlpha = [] := by decide
def vertigoSpawn : Ability :=
  whenever (does thisCreature (block (a isCreature)))
    (sequentially
      [ tap (that creature),
        skipUntap (that creature) (.lit 1) ])
theorem okVertigoSpawn : Ability.check (DRS.ofList []) vertigoSpawn = [] := by decide
def orneryDilophosaur : Ability :=
  triggeredIf (does thisCreature attack)
    (exists_ (and [ isCreature, youControl,
                     hasPower .atLeast (.lit 4) ]))
    (get thisCreature (up (.lit 2)) (up (.lit 2)) (some untilEndOfTurn))
theorem okOrneryDilophosaur : Ability.check (DRS.ofList []) orneryDilophosaur = [] := by decide
def incisorGlider : Ability :=
  triggeredIf (does thisCreature attack)
    (compareAmt (countersOn poison anOpponent) .atLeast (.lit 3))
    (get (allOf (and [isCreature, youControl])) (up (.lit 1)) (up (.lit 1)) (some untilEndOfTurn))
theorem okIncisorGlider : Ability.check (DRS.ofList []) incisorGlider = [] := by decide
def stormFleetSpy : Ability :=
  triggeredIf (does thisCreature enter) (happened attack you soFarThisTurn)
    (draw (.lit 1))
theorem okStormFleetSpy : Ability.check (DRS.ofList []) stormFleetSpy = [] := by decide
def loanShark : Ability :=
  triggeredIf (does thisCreature enter)
    (compareAmt (eventCount (cast any) you soFarThisTurn) .atLeast (.lit 2)) (draw
      (.lit 1))
theorem okLoanShark : Ability.check (DRS.ofList []) loanShark = [] := by decide

def forceOfDespair : Action :=
  destroy (allOf
             (and
                [isCreature, happenedTo any (putOntoBattlefield which) soFarThisTurn]))
theorem okForceOfDespair : Action.check (DRS.ofList []) forceOfDespair = [] := by decide
def cradleToGrave : Action :=
  destroy (target (and [isCreature, not
    (hasColor .black), happenedTo any (putOntoBattlefield which) soFarThisTurn]))
theorem okCradleToGrave : Action.check (DRS.ofList []) cradleToGrave = [] := by decide

def aragornKingOfGondor : Ability :=
  when (does thisCreature enter) (gainDesignation you monarch)
theorem okAragornKingOfGondor : Ability.check (DRS.ofList []) aragornKingOfGondor = [] := by decide
def firmamentSage : Ability := whenever (gameBecomes night) (draw (.lit 1))
theorem okFirmamentSage : Ability.check (DRS.ofList []) firmamentSage = [] := by decide
def deeprootWarrior : Ability :=
  whenever (does any (block thisCreature))
    (get it (up (.lit 1)) (up (.lit 1)) (some untilEndOfTurn))
theorem okDeeprootWarrior : Ability.check (DRS.ofList []) deeprootWarrior = [] := by decide
def borderlandMarauder : Ability :=
  whenever (does thisCreature attack) (get it (up (.lit 2)) (up (.lit 0)) (some untilEndOfTurn))
theorem okBorderlandMarauder : Ability.check (DRS.ofList []) borderlandMarauder = [] := by decide
def lichsMasteryLoss : Ability :=
  when (does thisEnchantment leave) loseGame
theorem okLichsMasteryLoss : Ability.check (DRS.ofList []) lichsMasteryLoss = [] := by decide
def phageTheUntouchable : Ability :=
  whenever (does thisCreature (dealDamage (to := a isPlayer) (kind := .combatOnly)))
      (instruct (that player) loseGame)
theorem okPhageTheUntouchable : Ability.check (DRS.ofList []) phageTheUntouchable = [] := by decide
def elderscaleWurm : Ability :=
  triggeredIf (does thisCreature enter) (compareAmt (lifeTotalOf you) .less (.lit 7))
    (setLife (.lit 7))
theorem okElderscaleWurm : Ability.check (DRS.ofList []) elderscaleWurm = [] := by decide
/-- Krang, Master Mind -/
def krang : Ability :=
  triggeredIf (does thisCreature enter)
    (compareAmt (countOf (cardIn yourHand)) .less (.lit 4)) (draw theDifference)
theorem okKrang : Ability.check (DRS.ofList []) krang = [] := by decide

def carrionGrub : Spelled := spelled <| .singleFaced
  { characteristics :=
    { name := "Carrion Grub", cost := some [generic 3, pip .black], types := [.creature],
      subtypes := [insect],
      text :=
        [ static (conjunction [ getsPt thisCreature (up (letter .x)) (up (.lit 0)),
              letterDefinition .x
                (aggregate .max (.stat .power) (and [isCreature, in_ yourGraveyard])) ]),
          when (does thisCreature enter) (mill (.lit 4)) ],
      power := stat 0, toughness := stat 5 } }

/-- Once Upon a Time -/
def onceUponATimeFirstCast : Predicate := nthCastBy (.nth 1) you .game
theorem okOnceUponATimeFirstCast : Predicate.checkAt .object (DRS.ofList []) onceUponATimeFirstCast = [] := by
  decide

def youngPyromancer : Spelled := spelled <| .singleFaced
  { characteristics :=
    { name := "Young Pyromancer", cost := some [generic 1, pip .red], types := [.creature],
      subtypes := [human, shaman],
      text :=
        [ whenever (does you (cast (a
          (and [isInstantOrSorcery, isSpell]))))
            (createToken (creatureToken 1 1 [.red] [elemental]))
                  ],
      power := stat 2, toughness := stat 1 } }

def archivistOfGondor : Ability :=
  triggeredIf (does yourCommander (dealDamage (to := a isPlayer) (kind := .combatOnly)))
      (thereIsNo monarch)
    (gainDesignation you monarch)
theorem okArchivistOfGondor : Ability.check (DRS.ofList []) archivistOfGondor = [] := by decide

def hissingMiasma : Spelled := spelled <| .singleFaced
  { characteristics :=
    { name := "Hissing Miasma", cost := some [generic 1, pip .black, pip .black],
      types := [.enchantment],
      text :=
        [ whenever (does (a isCreature) (attack (some you)))
            (instruct (controllerOf it) (loseLife (.lit 1))) ] } }

def orimsPrayer : Spelled := spelled <| .singleFaced
  { characteristics :=
    { name := "Orim's Prayer", cost := some [generic 1, pip .white, pip .white],
      types := [.enchantment],
      text :=
        [ whenever (does (counted (atLeast 1) isCreature) (attack (some you)))
            (gainLife (forEach (.lit 1) (and [isAttacking, isCreature]))) ] } }

def lesserGargadon : Spelled := spelled <| .singleFaced
  { characteristics :=
    { name := "Lesser Gargadon", cost := some [generic 2, pip .red, pip .red],
      types := [.creature], subtypes := [beast],
      text :=
        [ triggeredOr (does thisCreature attack) [does thisCreature block]
            (sacrifice (a isLand)) ],
      power := stat 6, toughness := stat 4 } }

/-- Chub Toad -/
def chubToad : Spelled := spelled <| .singleFaced
  { characteristics :=
    { name := "Chub Toad", cost := some [generic 2, pip .green], types := [.creature],
      subtypes := [frog],
      text :=
        [ triggeredOr (does thisCreature block) [does any (block thisCreature)]
            (get it (up (.lit 2)) (up (.lit 2)) (some untilEndOfTurn)) ],
      power := stat 1, toughness := stat 1 } }

/-- Giggling Skitterspike -/
def gigglingSkitterspikeTriggers : List GameEvent :=
  [does thisCreature block, does (a isSpell) (target thisCreature)]
theorem okGigglingSkitterspikeTriggers :
    GameEvent.checkAll (DRS.ofList []) gigglingSkitterspikeTriggers = [] := by decide

/-- Naban, Dean of Iteration -/
def nabanDeanOfIteration : Spelled := spelled <| .singleFaced
  { characteristics :=
    { name := "Naban, Dean of Iteration", cost := some [generic 1, pip .blue],
      supertypes := [.legendary], types := [.creature],
      subtypes := [human, wizard],
      text :=
        [ static (additionalTriggers
            (causes
              (byEvent
                (does (a (and [ hasSubtype wizard,
                                    youControl ])) enter))
              (triggers (a (and [ abilityHead .anyTriggered,
                           abilityOf (a (and [isPermanent, youControl])) ]))))
            (exactly 1)) ],
      power := stat 2, toughness := stat 1 } }

/-- Bloom Hulk -/
def bloomHulk : Spelled := spelled <| .singleFaced
  { characteristics :=
    { name := "Bloom Hulk", cost := some [generic 3, pip .green], types := [.creature],
      subtypes := [plant, elemental],
      text := [when (does thisCreature enter) proliferate],
      power := stat 4, toughness := stat 4 } }

/-- Stalwart Successor -/
def stalwartSuccessorHeader : GameEvent := bareCounterEvent .put .many (a (and [isCreature, youControl]))
theorem okStalwartSuccessorHeader : GameEvent.check (DRS.ofList []) stalwartSuccessorHeader = [] := by decide

/-- Hollowmurk Siege -/
def hollowmurkSiegeSultai : Ability :=
  triggeredOnlyOnce (bareCounterEvent .put .one (a (and [isCreature, youControl]))) oncePerTurn
    (draw (.lit 1))
theorem okHollowmurkSiegeSultai : Ability.check (DRS.ofList []) hollowmurkSiegeSultai = [] := by decide

/-- Clone -/
def clone : Spelled := spelled <| .singleFaced
  { characteristics :=
    { name := "Clone", cost := some [generic 3, pip .blue], types := [.creature],
      subtypes := [shapeshifter],
      text := [static (entryRider thisCreature (asCopyOf true (a isCreature) []))],
      power := stat 0, toughness := stat 0 } }

/-- Quicksilver Gargantuan -/
def quicksilverGargantuan : Spelled := spelled <| .singleFaced
  { characteristics :=
    { name := "Quicksilver Gargantuan", cost := some [generic 5, pip .blue, pip .blue],
      types := [.creature], subtypes := [shapeshifter],
      text :=
        [ static (entryRider thisCreature
            (asCopyOf true (a isCreature) [hasPT (.lit 7) (.lit 7)])) ],
      power := stat 7, toughness := stat 7 } }

/-- Sculpting Steel -/
def sculptingSteel : Spelled := spelled <| .singleFaced
  { characteristics :=
    { name := "Sculpting Steel", cost := some [generic 3], types := [.artifact],
      text := [static (entryRider thisArtifact (asCopyOf true (a isArtifact) []))] } }

/-- Sakashima's Student -/
def sakashimasStudentCopy : Ability :=
  static (entryRider thisCreature
    (asCopyOf true (a isCreature) [isAlso [] [ninja]]))
theorem okSakashimasStudentCopy : Ability.check (DRS.ofList []) sakashimasStudentCopy = [] := by decide
/-- Chameleon, Master of Disguise -/
def chameleonCopy : Ability :=
  static (entryRider thisCreature
    (asCopyOf true (a (and [isCreature, youControl]))
      [isNamed "Chameleon, Master of Disguise"]))
theorem okChameleonCopy : Ability.check (DRS.ofList []) chameleonCopy = [] := by decide

def nefariousLichGain : Ability :=
  static (instead (would you (gainLife anyAmount)) (draw thatMuch))
theorem okNefariousLichGain : Ability.check (DRS.ofList []) nefariousLichGain = [] := by decide

def piousWarrior : Spelled := spelled <| .singleFaced
  { characteristics :=
    { name := "Pious Warrior", cost := some [generic 3, pip .white], types := [.creature],
      subtypes := [human, rebel, warrior],
      text := [whenever (does any (dealDamage (to := thisCreature) (kind := .combatOnly))) (gainLife thatMuch)],
      power := stat 2, toughness := stat 3 } }

def sanguineBond : Spelled := spelled <| .singleFaced
  { characteristics :=
    { name := "Sanguine Bond", cost := some [generic 3, pip .black, pip .black],
      types := [.enchantment],
      text := [whenever (does you (gainLife anyAmount))
                 (instruct (target isOpponent) (loseLife thatMuch))]
          } }

def exquisiteBlood : Spelled := spelled <| .singleFaced
  { characteristics :=
    { name := "Exquisite Blood", cost := some [generic 4, pip .black], types := [.enchantment],
      text := [whenever (does anOpponent (loseLife anyAmount)) (gainLife thatMuch)] } }

/-- Agate-Blade Assassin -/
def agateBladeAssassin : Spelled := spelled <| .singleFaced
  { characteristics :=
    { name := "Agate-Blade Assassin", cost := some [generic 1, pip .black], types := [.creature],
      subtypes := [lizard, assassin],
      text :=
        [ whenever (does thisCreature attack)
            (sequentially
              [instruct defendingPlayer (loseLife (.lit 1)), gainLife (.lit 1)]) ],
      power := stat 1, toughness := stat 3 } }

/-- Bard, Heir of Girion -/
def bardHeirOfGirion : Spelled := spelled <| .singleFaced
  { characteristics :=
    { name := "Bard, Heir of Girion", cost := some [generic 2, pip .white, pip .blue],
      supertypes := [.legendary], types := [.creature],
      subtypes := [human, archer],
      text :=
        [ reach, vigilance,
          static
              (getsPt (allOf (and [isCreature, youControl, otherThan thisCreature])) (up (.lit 1))
                 (up (.lit 1))),
          whenever (does you attack) (draw (.lit 1)) ],
      power := stat 4, toughness := stat 4 } }

/-- Fiend Binder -/
def fiendBinder : Spelled := spelled <| .singleFaced
  { characteristics :=
    { name := "Fiend Binder", cost := some [generic 3, pip .white], types := [.creature],
      subtypes := [human, soldier],
      text :=
        [ whenever (does thisCreature attack)
            (tap
              (target (and [isCreature, controlledBy defendingPlayer]))) ],
      power := stat 3, toughness := stat 2 } }

/-- Souls of the Faultless -/
def soulsOfTheFaultlessDrain : Ability :=
  whenever (does any (dealDamage (to := thisCreature) (kind := .combatOnly)))
    (instruct (combatPlayer .attacking) (loseLife thatMuch))
theorem okSoulsOfTheFaultlessDrain : Ability.check (DRS.ofList []) soulsOfTheFaultlessDrain = [] := by decide

def unstableShapeshifter : Spelled := spelled <| .singleFaced
  { characteristics :=
    { name := "Unstable Shapeshifter", cost := some [generic 3, pip .blue], types := [.creature],
      subtypes := [shapeshifter],
      text :=
        [ whenever (does (a (and [isCreature, otherThan this_])) enter)
            (establish (copyChange thisCreature (that creature) [exceptThisAbility])) ],
      power := stat 0, toughness := stat 1 } }

def prismRing : Spelled := spelled <| .singleFaced
  { characteristics :=
    { name := "Prism Ring", cost := some [generic 1], types := [.artifact],
      text :=
        [ static (entersChoosing thisArtifact .color),
          whenever (does you (cast (a
            (and [isSpell, ofChosen .color])))) (gainLife (.lit 1)) ] } }

/-- Aisling Leprechaun -/
def aislingLeprechaun : Spelled := spelled <| .singleFaced
  { characteristics :=
    { name := "Aisling Leprechaun", cost := some [pip .green], types := [.creature],
      subtypes := [faerie],
      text :=
        [ triggeredOr (does thisCreature (block (a isCreature)))
            [does (by_ (a isCreature)) (block thisCreature)]
            (becomeColor (that creature) (.some [.green])) ],
      power := stat 1, toughness := stat 1 } }

def avenShrine : Spelled := spelled <| .singleFaced
  { characteristics :=
    { name := "Aven Shrine", cost := some [generic 1, pip .white, pip .white],
      types := [.enchantment],
      text :=
        [ whenever (does (a isPlayer) (cast (a isSpell)))
            (sequentially
              [ instruct they (gainLife (letter .x)),
                define .x (countOf (and [in_ graveyard, (shares .cardName .share (that .spell))])) ]) ] } }

def chromeReplicator : Spelled := spelled <| .singleFaced
  { characteristics :=
    { name := "Chrome Replicator", cost := some [generic 5], types := [.artifact, .creature],
      subtypes := [construct],
      text :=
        [ triggeredIf (does thisCreature enter)
            (thereAre (withTheSameName
              (counted (atLeast 2)
                (and [isPermanent, non isLand, isNontoken, youControl]))))
            (createToken
              { characteristics :=
                { types := [.artifact, .creature], subtypes := [construct],
                  power := stat 4, toughness := stat 4 } }) ],
      power := stat 4, toughness := stat 4 } }

def admiralsOrder : Spelled := spelled <| .singleFaced
  { characteristics :=
    { name := "Admiral's Order", cost := some [generic 1, pip .blue, pip .blue], types := [.instant],
      text :=
        [ static (onlyWhile (altCost this_ (some (mana [pip .blue])))
            (happened attack you soFarThisTurn)),
          spell (counter (target isSpell)) ] } }

def fyndhornDruid : Spelled := spelled <| .singleFaced
  { characteristics :=
    { name := "Fyndhorn Druid", cost := some [generic 2, pip .green], types := [.creature],
      subtypes := [elf, druid],
      text :=
        [ triggeredIf (does thisCreature die) (happened (block it) any soFarThisTurn)
            (gainLife (.lit 4)) ],
      power := stat 2, toughness := stat 2 } }

def kamiOfTerribleSecrets : Spelled := spelled <| .singleFaced
  { characteristics :=
    { name := "Kami of Terrible Secrets", cost := some [generic 3, pip .black], types := [.creature],
      subtypes := [spirit],
      text :=
        [ triggeredIf (does thisCreature enter)
            (and [ exists_ (and [isArtifact, youControl]),
                    exists_ (and [isEnchantment, youControl]) ])
            (sequentially [draw (.lit 1), gainLife (.lit 1)]) ],
      power := stat 3, toughness := stat 4 } }

def dreadSlaver : Spelled := spelled <| .singleFaced
  { characteristics :=
    { name := "Dread Slaver", cost := some [generic 3, pip .black, pip .black],
      types := [.creature], subtypes := [zombie, horror],
      text :=
        [ whenever
            (does (a (and [ isCreature,
                              happenedTo thisCreature
                                (dealDamage anyAmount which)
                                soFarThisTurn ])) die)
            (sequentially
              [ putOntoBattlefieldUnderYourControl it,
                become (that creature)
                  { characteristics := { colors := [.black], subtypes := [zombie] } } ]) ],
      power := stat 3, toughness := stat 5 } }

/-- Steppe Lynx -/
def steppeLynx : Ability :=
  abilityWord "landfall"
    (whenever (does (a (and [isLand, youControl])) enter)
      (get thisCreature (up (.lit 2)) (up (.lit 2)) (some untilEndOfTurn)))
theorem okSteppeLynx : Ability.check (DRS.ofList []) steppeLynx = [] := by decide
/-- Owlbear -/
def owlbear : Ability :=
  flavorWord "Keen Senses" (when (does thisCreature enter) (draw (.lit 1)))
theorem okOwlbear : Ability.check (DRS.ofList []) owlbear = [] := by decide
/-- Netherese Puzzle-Ward -/
def netheresePuzzleWardIllumination : Ability :=
  whenever (does you (rollDice (watch := highestNatural))) (draw (.lit 1))
theorem okNetheresePuzzleWardIllumination :
    Ability.check (DRS.ofList []) netheresePuzzleWardIllumination = [] := by decide
/-- Farideh, Devil's Chosen -/
def faridehResultRead : Ability :=
  whenever (does you (rollDice anyAmount .anyDie .many))
    (sequentially
      [ gain thisCreature flying (some untilEndOfTurn),
        gain thisCreature menace (some untilEndOfTurn),
        doIf (anyResultIs .atLeast (.lit 10)) (draw (.lit 1)) ])
theorem okFaridehResultRead : Ability.check (DRS.ofList []) faridehResultRead = [] := by decide
/-- Jaws of Defeat -/
def jawsOfDefeat : Ability :=
  whenever (does (a (and [isCreature, youControl])) enter)
    (instruct (target isOpponent) (loseLife
      (arith .differenceBetween (powerOf (that creature))
        (toughnessOf (that creature)))))
theorem okJawsOfDefeat : Ability.check (DRS.ofList []) jawsOfDefeat = [] := by decide
/-- Defiling Daemogoth -/
def defilingDaemogothDrain : Action :=
  sequentially
    [instruct eachOpponent (loseLife (letter .x)),
      define .x (eventSum (gainLife anyAmount) you soFarThisTurn)]
theorem okDefilingDaemogothDrain : Action.check (DRS.ofList []) defilingDaemogothDrain = [] := by decide
def skullsporeNexusTrigger : Ability :=
  whenever (does (counted (atLeast 1) (and [isNontoken, and [isCreature, youControl]])) die)
    (createToken (creatureTokenOf (totalOf (.stat .power) (those card))
        (totalOf (.stat .power) (those card))
        [.green] [fungus, dinosaur]))
theorem okSkullsporeNexusTrigger : Ability.check (DRS.ofList []) skullsporeNexusTrigger = [] := by decide
/-- Wavebreak Hippocamp -/
def wavebreakHippocamp : Ability :=
  triggeredOnlyDuring (nthOccurrence (.nth 1) (does you (cast (a isSpell))))
    (duringPart .turn (some eachOpponent)) (draw (.lit 1))
theorem okWavebreakHippocamp : Ability.check (DRS.ofList []) wavebreakHippocamp = [] := by decide
/-- Midnight Clock -/
def midnightClockHeader : GameEvent :=
  nthOccurrence (.nth 12) (counterEvent .put hourCounter .one thisArtifact)
theorem okMidnightClockHeader : GameEvent.check (DRS.ofList []) midnightClockHeader = [] := by decide
/-- Political Triumph -/
def politicalTriumphHeader : GameEvent :=
  nthOccurrence (.nth 4) (counterEvent .put planCounter .one thisEnchantment)
theorem okPoliticalTriumphHeader : GameEvent.check (DRS.ofList []) politicalTriumphHeader = [] := by decide
/-- Thought Lash -/
def thoughtLashTrigger : Ability :=
  when (does (a isPlayer)
          (pay anyCost .once .unpaid (some thisEnchantment) (some cumulativeUpkeep)))
    (instruct (that player) (exile (each (cardIn (libraryOf they)))))
theorem okThoughtLashTrigger : Ability.check (DRS.ofList []) thoughtLashTrigger = [] := by decide
/-- Heart of Bogardan -/
def heartOfBogardanHeader : GameEvent :=
  does (a isPlayer)
      (pay anyCost .once .unpaid (some thisEnchantment) (some cumulativeUpkeep))
theorem okHeartOfBogardanHeader : GameEvent.check (DRS.ofList []) heartOfBogardanHeader = [] := by decide
/-- Balduvian Fallen -/
def balduvianFallenHeader : GameEvent := does any (pay anyCost .once .paid (some thisCreature) (some cumulativeUpkeep))
theorem okBalduvianFallenHeader : GameEvent.check (DRS.ofList []) balduvianFallenHeader = [] := by decide
/-- Stormscape Battlemage -/
def stormscapeBattlemageFirstKicker : Ability :=
  triggeredIf (does thisCreature enter)
    (costWasPaid (.byNthKeyword (.nth 1) kicker) none thisCreature) (gainLife (.lit 3))
theorem okStormscapeBattlemageFirstKicker :
    Ability.check (DRS.ofList []) stormscapeBattlemageFirstKicker = [] := by decide
/-- All-Seeing Arbiter -/
def allSeeingArbiterHeader : GameEvent :=
  does you (discard (a
    (cardIn hand)))
theorem okAllSeeingArbiterHeader : GameEvent.check (DRS.ofList []) allSeeingArbiterHeader = [] := by decide

/-- Liliana's Caress -/
def lilianasCaress : Spelled := spelled <| .singleFaced
  { characteristics :=
    { name := "Liliana's Caress", cost := some [generic 1, pip .black], types := [.enchantment],
      text :=
        [ whenever (does anOpponent (discard (a
          (cardIn hand))))
            (instruct they (loseLife (.lit 2))) ] } }

/-- Scheming Aspirant -/
def schemingAspirant : Spelled := spelled <| .singleFaced
  { characteristics :=
    { name := "Scheming Aspirant", cost := some [generic 1, pip .black], types := [.creature],
      subtypes := [phyrexian, advisor],
      text :=
        [ whenever (does you proliferate)
            (sequentially [instruct eachOpponent (loseLife (.lit 2)), gainLife (.lit 2)]) ],
      power := stat 1, toughness := stat 3 } }

/-- Reciprocate -/
def reciprocate : Spelled := spelled <| .singleFaced
  { characteristics :=
    { name := "Reciprocate", cost := some [pip .white], types := [.instant],
      text :=
        [ spell (exile
            (target
               (and
                  [isCreature, happenedTo which (dealDamage anyAmount you)
                     soFarThisTurn]))) ] } }

/-- Tahngarth, First Mate -/
def tahngarthHeader : GameEvent := does anOpponent (attackWith (counted (atLeast 1) isCreature))
theorem okTahngarthHeader : GameEvent.check (DRS.ofList []) tahngarthHeader = [] := by decide
/-- Myth Unbound -/
def mythUnboundTrigger : Ability :=
  whenever (does any (putInto yourCommander commandZone (from_ := some any))) (draw (.lit 1))
theorem okMythUnboundTrigger : Ability.check (DRS.ofList []) mythUnboundTrigger = [] := by decide

/-- Commander's Insignia -/
def commandersInsignia : Spelled := spelled <| .singleFaced
  { characteristics :=
    { name := "Commander's Insignia", cost := some [generic 2, pip .white, pip .white],
      types := [.enchantment],
      text :=
        [ static (getsPt (allOf (and [isCreature, youControl]))
            (up (eventCount (cast (headed .spell yourCommander) (from_ := some
              commandZone)) you thisGame))
            (up (eventCount (cast (headed .spell yourCommander) (from_ := some
              commandZone)) you thisGame)))
              ] } }

/-- Faith's Reward -/
def faithsReward : Spelled := spelled <| .singleFaced
  { characteristics :=
    { name := "Faith's Reward", cost := some [generic 3, pip .white], types := [.instant],
      text :=
        [ spell (putOntoBattlefield (allOf (and [ isPermanentCard, in_ yourGraveyard,
                           happenedTo any
                             (putInto which yourGraveyard (from_ := some battlefield))
                             soFarThisTurn ]))) ] } }

/-- Oscorp Industries -/
def oscorpIndustriesReturn : Ability :=
  when (does thisLand (enter (from_ := some graveyard))) (loseLife (.lit 2))
theorem okOscorpIndustriesReturn : Ability.check (DRS.ofList []) oscorpIndustriesReturn = [] := by decide
def theLostAndTheDamnedEntryArm : GameEvent :=
  does (a (and [isLand, youControl])) (enter (from_ := some (not yourHand)))
theorem okTheLostAndTheDamnedEntryArm :
    GameEvent.check (DRS.ofList []) theLostAndTheDamnedEntryArm = [] := by decide
/-- Forsaken Wastes -/
def forsakenWastesTargeted : Ability :=
  whenever (does (a isSpell) (target thisEnchantment))
    (instruct (controllerOf (that .spell)) (loseLife (.lit 5)))
theorem okForsakenWastesTargeted : Ability.check (DRS.ofList []) forsakenWastesTargeted = [] := by decide

/-- Fblthp, the Lost -/
def fblthp : Spelled := spelled <| .singleFaced
  { characteristics :=
    { name := "Fblthp, the Lost", cost := some [generic 1, pip .blue],
      supertypes := [.legendary], types := [.creature], subtypes := [homunculus],
      text :=
        [ when (does this_ enter)
            (replace (draw (.lit 1))
              (doIf (or
                  [ did ⟨it, enter (from_ := some yourLibrary), triggering⟩,
                    matches_ it (castFrom yourLibrary) ])
                (draw (.lit 2)))),
          when (does (a isSpell) (target this_)) (shuffleInto this_) ],
      power := stat 1, toughness := stat 1 } }

/-- Frost Walker -/
def frostWalker : Spelled := spelled <| .singleFaced
  { characteristics :=
    { name := "Frost Walker", cost := some [generic 1, pip .blue], types := [.creature],
      subtypes := [elemental],
      text :=
        [ when (does (a (or [isSpell, abilityHead .anyOnStack])) (target thisCreature))
            (sacrifice itPermanent) ],
      power := stat 4, toughness := stat 1 } }

/-- Loaming Shaman -/
def loamingShaman : Spelled := spelled <| .singleFaced
  { characteristics :=
    { name := "Loaming Shaman", cost := some [generic 2, pip .green], types := [.creature],
      subtypes := [centaur, shaman],
      text :=
        [ when (does thisCreature enter)
            (instruct (target isPlayer) (shuffleInto
              (targets anyNumber (cardIn (graveyardOf they))))) ],
      power := stat 3, toughness := stat 2 } }

/-- Debris Beetle -/
def debrisBeetleTrigger : Ability :=
  when (does thisVehicle enter)
    (sequentially [instruct eachOpponent (loseLife (.lit 3)), gainLife (.lit 3)])
theorem okDebrisBeetleTrigger : Ability.check (DRS.ofList []) debrisBeetleTrigger = [] := by decide
/-- Roads Go Ever, Ever On's chapters II and III -/
def roadsGoEverEverOnChapters : Ability :=
  when (chapterMark [2, 3]) (putInto (a (cardIn (exiledWith thisSaga))) hand)
theorem okRoadsGoEverEverOnChapters : Ability.check (DRS.ofList []) roadsGoEverEverOnChapters = [] := by decide
/-- Wurmwall Sweeper -/
def wurmwallSweeperTrigger : Ability := when (does thisSpacecraft enter) (surveil (.lit 2))
theorem okWurmwallSweeperTrigger : Ability.check (DRS.ofList []) wurmwallSweeperTrigger = [] := by decide
/-- Case of the Crimson Pulse -/
def caseOfTheCrimsonPulseTrigger : Ability :=
  when (does thisCase enter)
    (sequentially [discard (a (cardIn yourHand)), draw (.lit 2)])
theorem okCaseOfTheCrimsonPulseTrigger : Ability.check (DRS.ofList []) caseOfTheCrimsonPulseTrigger = [] := by
  decide

/-- Ferocious Pup -/
def ferociousPup : Spelled := spelled <| .singleFaced
  { characteristics :=
    { name := "Ferocious Pup", cost := some [generic 2, pip .green], types := [.creature],
      subtypes := [wolf],
      text :=
        [ when (does thisCreature enter)
            (createToken (creatureToken 2 2 [.green] [wolf])) ],
      power := stat 0, toughness := stat 1 } }

/-- Agency Outfitter's search -/
def agencyOutfitterSearch : Action :=
  sequentially
    [ may
        (sequentially
          [ searchZonesOf you (exactly 1)
              (or [named (printedName "Magnifying Glass"), named (printedName "Thinking Cap")]),
            putOntoBattlefield (theVerbed (.action "search") .one) ]),
      doIf (happenedThisWay (search any anyNumber (and [])) you) shuffle ]
theorem okAgencyOutfitterSearch : Action.check (DRS.ofList []) agencyOutfitterSearch = [] := by decide

/-- Trouble in Pairs' -/
def troubleInPairsArms : Ability :=
  triggeredOr (does anOpponent
                 (attackWith (counted (atLeast 2) isCreature) (some you))) [ nthOccurrence (.nth 2)
                                                                               (does
                                                                                  (actor
                                                                                     anOpponent) (draw anyAmount)) (some .turn),
      nthOccurrence (.nth 2) (does anOpponent
                                (cast (a isSpell))) (some .turn) ] (draw
                                                                                              (.lit
                                                                                                 1))
theorem okTroubleInPairsArms : Ability.check (DRS.ofList []) troubleInPairsArms = [] := by decide

/-- Bioplasm -/
def bioplasm : Spelled := spelled <| .singleFaced
  { characteristics :=
    { name := "Bioplasm", cost := some [generic 3, pip .green, pip .green], types := [.creature],
      subtypes := [ooze],
      text :=
        [ whenever (does thisCreature attack)
            (sequentially
              [ exile (topSlice (.lit 1)),
                doIf bioplasmCardTest
                  (get thisCreature
                    (up (powerOf
                           (theVerbed (.action "exile") .one (some (.ofType .card .creature)))))
                    (up (toughnessOf (theVerbed (.action "exile") .one)))
                    (some thisTurn)) ]) ],
      power := stat 4, toughness := stat 4 } }

/-- Hostile Investigator -/
def hostileInvestigatorHeader : GameEvent :=
  does (counted (atLeast 1) isPlayer) (discard (counted (atLeast 1) isCard))
theorem okHostileInvestigatorHeader : GameEvent.check (DRS.ofList []) hostileInvestigatorHeader = [] := by decide

/-- Hallowed Moonlight -/
def hallowedMoonlight : Spelled := spelled <| .singleFaced
  { characteristics :=
    { name := "Hallowed Moonlight", cost := some [generic 1, pip .white], types := [.instant],
      text :=
        [ spell (sequentially
            [ replaceEvent (would any (putOntoBattlefield (a (and [isCreature, not (castBy any)]))))
                (exile it)
                (some untilEndOfTurn),
              draw (.lit 1) ]) ] } }

/-- Gather Specimens -/
def gatherSpecimens : Spelled := spelled <| .singleFaced
  { characteristics :=
    { name := "Gather Specimens", cost := some [generic 3, pip .blue, pip .blue, pip .blue],
      types := [.instant],
      text :=
        [ spell
            (establishThroughout
               (entryRider (a (and [isCreature, anOpponentControls]))
                  (under you)) thisTurn) ] } }

/-- Don't Blink's replacement, without its written agent -/
def dontBlinkReplacement : Action :=
  establishThroughout
    (instead
      (wouldAny
        [ would (counted (atLeast 1) isCreature) (enter (from_ := some exileZone)),
          would (counted (atLeast 1) (and [isCreature, castFrom exileZone])) enter ])
      (shuffleInto them))
    untilEndOfTurn
theorem okDontBlinkReplacement : Action.check (DRS.ofList []) dontBlinkReplacement = [] := by decide

/-- Seasoned Warrenguard -/
def seasonedWarrenguard : Spelled := spelled <| .singleFaced
  { characteristics :=
    { name := "Seasoned Warrenguard", cost := some [pip .white], types := [.creature],
      subtypes := [rabbit, warrior],
      text :=
        [ triggeredWhile (does thisCreature attack)
            (whileTrue (exists_ (and [isToken, youControl])))
            (get thisCreature (up (.lit 2)) (up (.lit 0)) (some untilEndOfTurn)) ],
      power := stat 1, toughness := stat 2 } }

/-- Brazen Blademaster -/
def brazenBlademaster : Spelled := spelled <| .singleFaced
  { characteristics :=
    { name := "Brazen Blademaster", cost := some [generic 2, pip .red], types := [.creature],
      subtypes := [orc, pirate],
      text :=
        [ triggeredWhile (does thisCreature attack)
            (whileTrue
              (compareAmt (countOf (and [isArtifact, youControl])) .atLeast
                (.lit 2)))
            (get it (up (.lit 2)) (up (.lit 1)) (some untilEndOfTurn)) ],
      power := stat 2, toughness := stat 3 } }

/-- Autarch Mammoth -/
def autarchMammothLine : Ability :=
  triggeredJoined (does thisCreature enter)
    [ joinedHeadWhile (does thisCreature attack)
        (whileTrue (matches_ thisCreature (hasDesignation saddled))) ]
    (createToken (creatureToken 3 3 [.green] [elephant]))
theorem okAutarchMammothLine : Ability.check (DRS.ofList []) autarchMammothLine = [] := by decide
/-- Altar of the Brood -/
def altarOfTheBrood : Ability :=
  whenever (does (a (and [isPermanent, youControl, otherThan this_])) enter)
    (instruct eachOpponent (mill (.lit 1)))
theorem okAltarOfTheBrood : Ability.check (DRS.ofList []) altarOfTheBrood = [] := by decide
/-- Foul Emissary -/
def foulEmissaryLine : Ability :=
  triggeredWhile (does you (sacrifice thisCreature))
    (whileDoing (does you (cast (a
      (and [isSpell, hasKeyword (.the emerge)])))))
    (createToken (creatureToken 3 2 [] [eldrazi, horror]))
theorem okFoulEmissaryLine : Ability.check (DRS.ofList []) foulEmissaryLine = [] := by decide

/-- Bramble Elemental -/
def brambleElemental : Spelled := spelled <| .singleFaced
  { characteristics :=
    { name := "Bramble Elemental", cost := some [generic 3, pip .green, pip .green],
      types := [.creature], subtypes := [elemental],
      text :=
        [ whenever (does any (attachment .attached (a (hasSubtype aura)) (some thisCreature)))
            (createToken (creatureToken 1 1 [.green] [saproling]) (count := .lit 2)) ],
      power := stat 4, toughness := stat 4 } }

/-- Stone Haven Outfitter -/
def stoneHavenOutfitter : Spelled := spelled <| .singleFaced
  { characteristics :=
    { name := "Stone Haven Outfitter", cost := some [generic 1, pip .white], types := [.creature],
      subtypes := [kor, artificer, ally],
      text :=
        [ static (getsPt (allOf (and [and [isCreature, youControl], isAttached (some .equipped)]))
            (up (.lit 1)) (up (.lit 1))),
          whenever (does (a (and [and [isCreature, youControl], isAttached (some .equipped)])) die)
            (draw (.lit 1)) ],
      power := stat 2, toughness := stat 2 } }

/-- Cloud, Ex-SOLDIER -/
def cloudExSoldierAttach : Ability :=
  when (does thisCreature enter)
    (attachToIt
      (targets (upTo 1) (and [hasSubtype equipment, youControl])))
theorem okCloudExSoldierAttach : Ability.check (DRS.ofList []) cloudExSoldierAttach = [] := by decide
/-- Akiri, Fearless Voyager -/
def akiriEquippedAttackers : Ability :=
  whenever
    (does you
       (attackWith
          (counted (atLeast 1) (and [and [isCreature, youControl], isAttached (some .equipped)]))
        (some (a isPlayer))))
    (draw (.lit 1))
theorem okAkiriEquippedAttackers : Ability.check (DRS.ofList []) akiriEquippedAttackers = [] := by decide
/-- Vexing Bauble -/
def vexingBaubleTrigger : Ability :=
  triggeredIf (does (a isPlayer) (cast (a isSpell)))
    (noManaSpentToCast it)
    (counter (that .spell))
theorem okVexingBaubleTrigger : Ability.check (DRS.ofList []) vexingBaubleTrigger = [] := by decide

/-- Void Mirror -/
def voidMirror : Spelled := spelled <| .singleFaced
  { characteristics :=
    { name := "Void Mirror", cost := some [generic 2], types := [.artifact],
      text :=
        [ triggeredIf (does (a isPlayer) (cast (a
          isSpell))) (noColoredManaSpentToCast it)
            (counter (that .spell)) ] } }

/-- Blood Sun -/
def bloodSun : Spelled := spelled <| .singleFaced
  { characteristics :=
    { name := "Blood Sun", cost := some [generic 2, pip .red], types := [.enchantment],
      text :=
        [ when (does thisEnchantment enter) (draw (.lit 1)),
          static (allAbilityLoss (allOf isLand) (some isManaAbility)) ] } }

/-- Promise of Bunrei -/
def promiseOfBunrei : Spelled := spelled <| .singleFaced
  { characteristics :=
    { name := "Promise of Bunrei", cost := some [generic 2, pip .white], types := [.enchantment],
      text :=
        [ when (does (a (and [isCreature, youControl])) die)
            (doIfDone (sacrifice thisEnchantment)
              (some (createToken (creatureToken 1 1 [] [spirit]) (count := .lit 4)))) ] } }

/-- Grave Peril -/
def gravePeril : Spelled := spelled <| .singleFaced
  { characteristics :=
    { name := "Grave Peril", cost := some [generic 1, pip .black], types := [.enchantment],
      text :=
        [ when (does (a (and [isCreature, non (hasColor .black)])) enter)
            (doIfDone (sacrifice thisEnchantment) (some (destroy (that (.type
                .creature))))) ] } }

/-- Blood Reckoning -/
def bloodReckoning : Spelled := spelled <| .singleFaced
  { characteristics :=
    { name := "Blood Reckoning", cost := some [generic 3, pip .black], types := [.enchantment],
      text :=
        [ whenever
            (does (a isCreature)
               (attack (some (youOr (a (and [isPlaneswalker, youControl]))))))
            (instruct (controllerOf (that creature)) (loseLife (.lit 1))) ] } }

/-- Jeskai Ascendancy's first trigger -/
def jeskaiAscendancyPump : Ability :=
  whenever (does you (cast (a (and
    [isSpell, non isCreature]))))
    (sequentially
      [ get (bare (and [isCreature, youControl])) (up (.lit 1)) (up (.lit 1)) (some untilEndOfTurn),
        untap (those creature) ])
theorem okJeskaiAscendancyPump : Ability.check (DRS.ofList []) jeskaiAscendancyPump = [] := by decide
def herosDemise : Action := destroy (target (and [isCreature, hasSupertype .legendary]))
theorem okHerosDemise : Action.check (DRS.ofList []) herosDemise = [] := by decide

def repayInKind : Spelled := spelled <| .singleFaced
  { characteristics :=
    { name := "Repay in Kind", cost := some [generic 5, pip .black, pip .black], types := [.sorcery],
      text :=
        [ spell (instruct eachPlayer (setLife (aggregate .min (.playerStat .lifeTotal) isPlayer)))
            ] } }

/-- Squelch -/
def squelch : Spelled := spelled <| .singleFaced
  { characteristics :=
    { name := "Squelch", cost := some [generic 1, pip .blue], types := [.instant],
      text :=
        [ spell (sequentially
            [counter (target (abilityHead .anyActivated)), draw (.lit 1)]) ]
                } }

theorem bioplasmTwoCandidates : countOnes .object bioplasmAfterExile = 2 := by decide

/-- "From anywhere" observes Dread after its arrival in the graveyard. -/
def dread : Spelled := spelled <| .singleFaced
  { characteristics :=
    { name := "Dread", cost := some [generic 3, pip .black, pip .black, pip .black],
      types := [.creature], subtypes := [elemental, incarnation],
      text :=
        [ fear,
          whenever (does (a isCreature) (dealDamage (to := you))) (destroy it),
          when (does any (putInto this_ graveyard (from_ := some any)))
            (instruct (ownerOf it) (shuffleInto it)) ],
      power := stat 6, toughness := stat 6 } }

private semantic_template creatureDamagedByAttachedHostThisTurn (attachment : AttachWord) : NounPhrase :=
  a (.and [isCreature,
    .did ⟨.actor (.attachHost attachment creature),
      .dealDamage .anyAmount .which .any, .duration soFarThisTurn none⟩])

/-- Vampiric Embrace. -/
def vampiricEmbrace : Spelled := spelled <| .singleFaced
  { characteristics :=
    { name := "Vampiric Embrace", cost := some [generic 2, pip .black, pip .black],
      types := [.enchantment], subtypes := [aura],
      text :=
        [ enchant isCreature,
          static (getsPt
            (enchanted creature)
            (up (.lit 2)) (up (.lit 2))),
          static (abilityGrant
            (enchanted creature) flying),
          whenever (does (creatureDamagedByAttachedHostThisTurn .enchanted) die)
            (putCounters (.lit 1) (printedKind p1p1Counter)
              (that creature) .many) ] } }

/-- Scythe of the Wretched. -/
def scytheOfTheWretched : Spelled := spelled <| .singleFaced
  { characteristics :=
    { name := "Scythe of the Wretched", cost := some [generic 2], types := [.artifact],
      subtypes := [equipment],
      text :=
        [ static (getsPt
            (equipped creature)
            (up (.lit 2)) (up (.lit 2))),
          whenever (does (creatureDamagedByAttachedHostThisTurn .equipped) die)
            (sequentially
              [ returnTo (that card) battlefield
                  [(under you)],
                attachTo thisEquipment (that creature) ]),
          equip (mana [generic 4]) ] } }

end Semantics.Cards
