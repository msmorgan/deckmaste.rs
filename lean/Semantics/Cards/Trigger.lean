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
  delayWithin (does (target isCreature) die) thisTurn
    (returnToBattlefield (that card) [(under (ownerOf it))])
theorem okGracefulReprieve : Action.check (DRS.ofList []) gracefulReprieve = [] := by decide

def cloudkinSeer : Ability := when (does thisCreature enter) (draw 1)
theorem okCloudkinSeer : Ability.check (DRS.ofList []) cloudkinSeer = [] := by decide
def promiseOfTomorrow : Ability := whenever (does (a (and [isCreature, youControl])) die) (exile it)
theorem okPromiseOfTomorrow : Ability.check (DRS.ofList []) promiseOfTomorrow = [] := by decide
def libraryLarcenist : Ability := whenever (does thisCreature attack) (draw 1)
theorem okLibraryLarcenist : Ability.check (DRS.ofList []) libraryLarcenist = [] := by decide
def jhessianThief : Ability :=
  whenever (does thisCreature (dealDamage (to := a isPlayer) (kind := some .combat)))
    (draw 1)
theorem okJhessianThief : Ability.check (DRS.ofList []) jhessianThief = [] := by decide
def scholarOfStars : Ability :=
  when (does thisCreature enter)
    (if_ := exists_ (and [isArtifact, youControl])) (draw 1)
theorem okScholarOfStars : Ability.check (DRS.ofList []) scholarOfStars = [] := by decide

def miserysShadow : Ability :=
  static (replace
            (would (a (and [isCreature, anOpponentControls])) die) (instead (exile it)))
theorem okMiserysShadow : Ability.check (DRS.ofList []) miserysShadow = [] := by decide

def beastWhisperer : Ability :=
  whenever (does you (cast (a (and
    [isCreature, isSpell])))) (draw 1)
theorem okBeastWhisperer : Ability.check (DRS.ofList []) beastWhisperer = [] := by decide
def mesmericOrb : Ability :=
  whenever (does (a isPermanent) (become untapped))
    (instruct (controllerOf (that permanent)) (mill 1))
theorem okMesmericOrb : Ability.check (DRS.ofList []) mesmericOrb = [] := by decide
def secretPlans : Ability :=
  whenever (does any (turnFaceUp (a (and [isPermanent, youControl]))))
    (draw 1)
theorem okSecretPlans : Ability.check (DRS.ofList []) secretPlans = [] := by decide

/-- Teferi's Imp -/
def teferisImpPhasesOut : Ability :=
  whenever (does thisCreature phaseOut) (discard (a (cardIn yourHand)))
theorem okTeferisImpPhasesOut : Ability.check (DRS.ofList []) teferisImpPhasesOut = [] := by decide
/-- Teferi's Imp -/
def teferisImpPhasesIn : Ability :=
  whenever (does thisCreature phaseIn) (draw 1)
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
    (get thisCreature (up 2) (up 0) untilEndOfTurn)
theorem okNetcasterSpider : Ability.check (DRS.ofList []) netcasterSpider = [] := by decide
def viashinoWeaponsmith : Ability :=
  whenever (does (by_ (a isCreature)) (block thisCreature))
    (get thisCreature (up 2) (up 2) untilEndOfTurn)
theorem okViashinoWeaponsmith : Ability.check (DRS.ofList []) viashinoWeaponsmith = [] := by decide
def somberwaldAlpha : Ability :=
  whenever (does any (becomeBlocked (a (and [isCreature, youControl]))))
    (get it (up 1) (up 1) untilEndOfTurn)
theorem okSomberwaldAlpha : Ability.check (DRS.ofList []) somberwaldAlpha = [] := by decide
def vertigoSpawn : Ability :=
  whenever (does thisCreature (block (a isCreature)))
    (sequentially
      [ tap (that creature),
        skipUntap (that creature) 1 ])
theorem okVertigoSpawn : Ability.check (DRS.ofList []) vertigoSpawn = [] := by decide
def orneryDilophosaur : Ability :=
  whenever (does thisCreature attack)
    (if_ := exists_ (and [ isCreature, youControl,
                     hasPower .atLeast 4 ]))
    (get thisCreature (up 2) (up 2) untilEndOfTurn)
theorem okOrneryDilophosaur : Ability.check (DRS.ofList []) orneryDilophosaur = [] := by decide
def incisorGlider : Ability :=
  whenever (does thisCreature attack)
    (if_ := compareAmt (countersOn poison anOpponent) .atLeast 3)
    (get (allOf (and [isCreature, youControl])) (up 1) (up 1) untilEndOfTurn)
theorem okIncisorGlider : Ability.check (DRS.ofList []) incisorGlider = [] := by decide
def stormFleetSpy : Ability :=
  when (does thisCreature enter) (if_ := some (did you attack soFarThisTurn))
    (draw 1)
theorem okStormFleetSpy : Ability.check (DRS.ofList []) stormFleetSpy = [] := by decide
def loanShark : Ability :=
  when (does thisCreature enter)
    (if_ := compareAmt (eventCount (cast any) you soFarThisTurn) .atLeast 2) (draw
      1)
theorem okLoanShark : Ability.check (DRS.ofList []) loanShark = [] := by decide

def forceOfDespair : Action :=
  destroy (allOf
             (and
                [isCreature, did which enter soFarThisTurn]))
theorem okForceOfDespair : Action.check (DRS.ofList []) forceOfDespair = [] := by decide
def cradleToGrave : Action :=
  destroy (target (and [isCreature, not
    (hasColor .black), did which enter soFarThisTurn]))
theorem okCradleToGrave : Action.check (DRS.ofList []) cradleToGrave = [] := by decide

def aragornKingOfGondor : Ability :=
  when (does thisCreature enter) (gainDesignation you monarch)
theorem okAragornKingOfGondor : Ability.check (DRS.ofList []) aragornKingOfGondor = [] := by decide
def firmamentSage : Ability := whenever (gameBecomes night) (draw 1)
theorem okFirmamentSage : Ability.check (DRS.ofList []) firmamentSage = [] := by decide
def deeprootWarrior : Ability :=
  whenever (does any (becomeBlocked thisCreature))
    (get it (up 1) (up 1) untilEndOfTurn)
theorem okDeeprootWarrior : Ability.check (DRS.ofList []) deeprootWarrior = [] := by decide
def borderlandMarauder : Ability :=
  whenever (does thisCreature attack) (get it (up 2) (up 0) untilEndOfTurn)
theorem okBorderlandMarauder : Ability.check (DRS.ofList []) borderlandMarauder = [] := by decide
def lichsMasteryLoss : Ability :=
  when (does thisEnchantment leave) loseGame
theorem okLichsMasteryLoss : Ability.check (DRS.ofList []) lichsMasteryLoss = [] := by decide
def phageTheUntouchable : Ability :=
  whenever (does thisCreature (dealDamage (to := a isPlayer) (kind := some .combat)))
      (instruct (that player) loseGame)
theorem okPhageTheUntouchable : Ability.check (DRS.ofList []) phageTheUntouchable = [] := by decide
def elderscaleWurm : Ability :=
  when (does thisCreature enter) (if_ := compareAmt (lifeTotalOf you) .less 7)
    (setLife 7)
theorem okElderscaleWurm : Ability.check (DRS.ofList []) elderscaleWurm = [] := by decide
/-- Krang, Master Mind -/
def krang : Ability :=
  when (does thisCreature enter)
    (if_ := compareAmt (countOf (cardIn yourHand)) .less 4) (draw theDifference)
theorem okKrang : Ability.check (DRS.ofList []) krang = [] := by decide

def carrionGrub : Spelled := spelled <| singleFacedCard
  { name := "Carrion Grub", cost := [generic 3, pip .black], types := [.creature],
    subtypes := [insect],
    text :=
      [ static (conjunction [ getsPt thisCreature (up (letter .x)) (up 0),
            letterDefinition .x
              (aggregate .max (.stat .power) (and [isCreature, in_ yourGraveyard])) ]),
        when (does thisCreature enter) (mill 4) ],
    power := stat 0, toughness := stat 5 }

/-- Once Upon a Time -/
def onceUponATimeFirstCast : Predicate :=
  did you (cast which) thisObject (ordinal := some (.nth 1, some .game))
theorem okOnceUponATimeFirstCast : Predicate.checkAt .object (DRS.ofList []) onceUponATimeFirstCast = [] := by
  decide

def youngPyromancer : Spelled := spelled <| singleFacedCard
  { name := "Young Pyromancer", cost := [generic 1, pip .red], types := [.creature],
    subtypes := [human, shaman],
    text :=
      [ whenever (does you (cast (a
        (and [isInstantOrSorcery, isSpell]))))
          (create 1 (creatureToken 1 1 [.red] [elemental]))
                ],
    power := stat 2, toughness := stat 1 }

def archivistOfGondor : Ability :=
  when (does yourCommander (dealDamage (to := a isPlayer) (kind := some .combat)))
      (if_ := thereIsNo monarch)
    (gainDesignation you monarch)
theorem okArchivistOfGondor : Ability.check (DRS.ofList []) archivistOfGondor = [] := by decide

def hissingMiasma : Spelled := spelled <| singleFacedCard
  { name := "Hissing Miasma", cost := [generic 1, pip .black, pip .black],
    types := [.enchantment],
    text :=
      [ whenever (does (a isCreature) (attack you))
          (instruct (controllerOf it) (loseLife 1)) ] }

def orimsPrayer : Spelled := spelled <| singleFacedCard
  { name := "Orim's Prayer", cost := [generic 1, pip .white, pip .white],
    types := [.enchantment],
    text :=
      [ whenever (does (counted (orMore 1) isCreature) (attack you))
          (gainLife (forEach 1 (and [isAttacking, isCreature]))) ] }

def lesserGargadon : Spelled := spelled <| singleFacedCard
  { name := "Lesser Gargadon", cost := [generic 2, pip .red, pip .red],
    types := [.creature], subtypes := [beast],
    text :=
      [ whenever (does thisCreature attack) (or_ := [does thisCreature block])
          (sacrifice (a isLand)) ],
    power := stat 6, toughness := stat 4 }

/-- Chub Toad -/
def chubToad : Spelled := spelled <| singleFacedCard
  { name := "Chub Toad", cost := [generic 2, pip .green], types := [.creature],
    subtypes := [frog],
    text :=
      [ whenever (does thisCreature block) (or_ := [does any (becomeBlocked thisCreature)])
          (get it (up 2) (up 2) untilEndOfTurn) ],
    power := stat 1, toughness := stat 1 }

/-- Giggling Skitterspike -/
def gigglingSkitterspikeTriggers : List GameEvent :=
  [does thisCreature attack, does thisCreature block, does (a isSpell) (target thisCreature)]
theorem okGigglingSkitterspikeTriggers :
    GameEvent.checkAll (DRS.ofList []) gigglingSkitterspikeTriggers = [] := by decide

/-- Naban, Dean of Iteration -/
def nabanDeanOfIteration : Spelled := spelled <| singleFacedCard
  { name := "Naban, Dean of Iteration", cost := [generic 1, pip .blue],
    supertypes := [.legendary], types := [.creature],
    subtypes := [human, wizard],
    text :=
      [ static (additionalTriggers
          (causes
            (byEvent
              (does (a (and [ wizard,
                                  youControl ])) enter))
            (triggers (a (and [ abilityHead .anyTriggered,
                         abilityOf (a (and [isPermanent, youControl])) ]))))
          (exactly 1)) ],
    power := stat 2, toughness := stat 1 }

/-- Bloom Hulk -/
def bloomHulk : Spelled := spelled <| singleFacedCard
  { name := "Bloom Hulk", cost := [generic 3, pip .green], types := [.creature],
    subtypes := [plant, elemental],
    text := [when (does thisCreature enter) proliferate],
    power := stat 4, toughness := stat 4 }

/-- Stalwart Successor -/
def stalwartSuccessorHeader : GameEvent := does any (putCounters anyAmount anyKind (a (and [isCreature, youControl])))
theorem okStalwartSuccessorHeader : GameEvent.check (DRS.ofList []) stalwartSuccessorHeader = [] := by decide

/-- Hollowmurk Siege -/
def hollowmurkSiegeSultai : Ability :=
  whenever (does any (putCounters 1 anyKind (a (and [isCreature, youControl]))))
    (draw 1) (limit := oncePerTurn)
theorem okHollowmurkSiegeSultai : Ability.check (DRS.ofList []) hollowmurkSiegeSultai = [] := by decide

/-- Clone -/
def clone : Spelled := spelled <| singleFacedCard
  { name := "Clone", cost := [generic 3, pip .blue], types := [.creature],
    subtypes := [shapeshifter],
    text := [static (enters thisCreature [asCopyOf true (a isCreature) []])],
    power := stat 0, toughness := stat 0 }

/-- Quicksilver Gargantuan -/
def quicksilverGargantuan : Spelled := spelled <| singleFacedCard
  { name := "Quicksilver Gargantuan", cost := [generic 5, pip .blue, pip .blue],
    types := [.creature], subtypes := [shapeshifter],
    text :=
      [ static (enters thisCreature
          [asCopyOf true (a isCreature) [hasPT 7 7]]) ],
    power := stat 7, toughness := stat 7 }

/-- Sculpting Steel -/
def sculptingSteel : Spelled := spelled <| singleFacedCard
  { name := "Sculpting Steel", cost := [generic 3], types := [.artifact],
    text := [static (enters thisArtifact [asCopyOf true (a isArtifact) []])] }

/-- Sakashima's Student -/
def sakashimasStudentCopy : Ability :=
  static (enters thisCreature
    [asCopyOf true (a isCreature) [isAlso [] [ninja]]])
theorem okSakashimasStudentCopy : Ability.check (DRS.ofList []) sakashimasStudentCopy = [] := by decide
/-- Chameleon, Master of Disguise -/
def chameleonCopy : Ability :=
  static (enters thisCreature
    [asCopyOf true (a (and [isCreature, youControl]))
      [isNamed "Chameleon, Master of Disguise"]])
theorem okChameleonCopy : Ability.check (DRS.ofList []) chameleonCopy = [] := by decide

def nefariousLichGain : Ability :=
  static (replace (would you (gainLife anyAmount)) (instead (draw thatMuch)))
theorem okNefariousLichGain : Ability.check (DRS.ofList []) nefariousLichGain = [] := by decide

def piousWarrior : Spelled := spelled <| singleFacedCard
  { name := "Pious Warrior", cost := [generic 3, pip .white], types := [.creature],
    subtypes := [human, rebel, warrior],
    text :=
      [ whenever (does any (dealDamage (to := thisCreature) (kind := some .combat)))
          (gainLife thatMuch) ],
    power := stat 2, toughness := stat 3 }

def sanguineBond : Spelled := spelled <| singleFacedCard
  { name := "Sanguine Bond", cost := [generic 3, pip .black, pip .black],
    types := [.enchantment],
    text := [whenever (does you (gainLife anyAmount))
               (instruct (target isOpponent) (loseLife thatMuch))]
        }

def exquisiteBlood : Spelled := spelled <| singleFacedCard
  { name := "Exquisite Blood", cost := [generic 4, pip .black], types := [.enchantment],
    text := [whenever (does anOpponent (loseLife anyAmount)) (gainLife thatMuch)] }

/-- Agate-Blade Assassin -/
def agateBladeAssassin : Spelled := spelled <| singleFacedCard
  { name := "Agate-Blade Assassin", cost := [generic 1, pip .black], types := [.creature],
    subtypes := [lizard, assassin],
    text :=
      [ whenever (does thisCreature attack)
          (sequentially
            [instruct defendingPlayer (loseLife 1), gainLife 1]) ],
    power := stat 1, toughness := stat 3 }

/-- Bard, Heir of Girion -/
def bardHeirOfGirion : Spelled := spelled <| singleFacedCard
  { name := "Bard, Heir of Girion", cost := [generic 2, pip .white, pip .blue],
    supertypes := [.legendary], types := [.creature],
    subtypes := [human, archer],
    text :=
      [ reach, vigilance,
        static
            (getsPt (allOf (and [isCreature, youControl, isOther])) (up 1)
               (up 1)),
        whenever (does you attack) (draw 1) ],
    power := stat 4, toughness := stat 4 }

/-- Fiend Binder -/
def fiendBinder : Spelled := spelled <| singleFacedCard
  { name := "Fiend Binder", cost := [generic 3, pip .white], types := [.creature],
    subtypes := [human, soldier],
    text :=
      [ whenever (does thisCreature attack)
          (tap
            (target (and [isCreature, controlledBy defendingPlayer]))) ],
    power := stat 3, toughness := stat 2 }

/-- Souls of the Faultless -/
def soulsOfTheFaultlessDrain : Ability :=
  whenever (does any (dealDamage (to := thisCreature) (kind := some .combat)))
    (instruct (combatPlayer .attacking) (loseLife thatMuch))
theorem okSoulsOfTheFaultlessDrain : Ability.check (DRS.ofList []) soulsOfTheFaultlessDrain = [] := by decide

def unstableShapeshifter : Spelled := spelled <| singleFacedCard
  { name := "Unstable Shapeshifter", cost := [generic 3, pip .blue], types := [.creature],
    subtypes := [shapeshifter],
    text :=
      [ whenever (does (a (and [isCreature, isOther])) enter)
          (establish (copyChange thisCreature (that creature) [exceptThisAbility])) ],
    power := stat 0, toughness := stat 1 }

def prismRing : Spelled := spelled <| singleFacedCard
  { name := "Prism Ring", cost := [generic 1], types := [.artifact],
    text :=
      [ static (entersChoosing thisArtifact .color),
        whenever (does you (cast (a
          (and [isSpell, ofChosen .color])))) (gainLife 1) ] }

/-- Aisling Leprechaun -/
def aislingLeprechaun : Spelled := spelled <| singleFacedCard
  { name := "Aisling Leprechaun", cost := [pip .green], types := [.creature],
    subtypes := [faerie],
    text :=
      [ whenever (does thisCreature (block (a isCreature)))
          (or_ := [does (by_ (a isCreature)) (block thisCreature)])
          (becomeColor (that creature) (.some [.green])) ],
    power := stat 1, toughness := stat 1 }

def avenShrine : Spelled := spelled <| singleFacedCard
  { name := "Aven Shrine", cost := [generic 1, pip .white, pip .white],
    types := [.enchantment],
    text :=
      [ whenever (does (a isPlayer) (cast (a isSpell)))
          (sequentially
            [ instruct they (gainLife (letter .x)),
              define .x (countOf (and [in_ graveyard, (shares .cardName .share (that .spell))])) ]) ] }

def chromeReplicator : Spelled := spelled <| singleFacedCard
  { name := "Chrome Replicator", cost := [generic 5], types := [.artifact, .creature],
    subtypes := [construct],
    text :=
      [ when (does thisCreature enter)
          (if_ := thereAre (withTheSameName
            (counted (orMore 2)
              (and [isPermanent, non isLand, isNontoken, youControl]))))
          (create 1 (written
            { characteristics :=
              { types := [.artifact, .creature], subtypes := [construct],
                power := stat 4, toughness := stat 4 } })) ],
    power := stat 4, toughness := stat 4 }

def admiralsOrder : Spelled := spelled <| singleFacedCard
  { name := "Admiral's Order", cost := [generic 1, pip .blue, pip .blue], types := [.instant],
    text :=
      [ static (onlyWhile (altCost this_ [pip .blue])
          (did you attack soFarThisTurn)),
        spell (counter (target isSpell)) ] }

def fyndhornDruid : Spelled := spelled <| singleFacedCard
  { name := "Fyndhorn Druid", cost := [generic 2, pip .green], types := [.creature],
    subtypes := [elf, druid],
    text :=
      [ when (does thisCreature die) (if_ := some (was (becomeBlocked it) soFarThisTurn))
          (gainLife 4) ],
    power := stat 2, toughness := stat 2 }

def kamiOfTerribleSecrets : Spelled := spelled <| singleFacedCard
  { name := "Kami of Terrible Secrets", cost := [generic 3, pip .black], types := [.creature],
    subtypes := [spirit],
    text :=
      [ when (does thisCreature enter)
          (if_ := some (and [ exists_ (and [isArtifact, youControl]),
                  exists_ (and [isEnchantment, youControl]) ]))
          (sequentially [draw 1, gainLife 1]) ],
    power := stat 3, toughness := stat 4 }

def dreadSlaver : Spelled := spelled <| singleFacedCard
  { name := "Dread Slaver", cost := [generic 3, pip .black, pip .black],
    types := [.creature], subtypes := [zombie, horror],
    text :=
      [ whenever
          (does (a (and [ isCreature,
                            did thisCreature (dealDamage anyAmount which) soFarThisTurn ])) die)
          (sequentially
            [ returnToBattlefield it [(under you)],
              become (that creature)
                { characteristics := { colors := [.black], subtypes := [zombie] } } ]) ],
    power := stat 3, toughness := stat 5 }

/-- Steppe Lynx -/
def steppeLynx : Ability :=
  landfall
    (whenever (does (a (and [isLand, youControl])) enter)
      (get thisCreature (up 2) (up 2) untilEndOfTurn))
theorem okSteppeLynx : Ability.check (DRS.ofList []) steppeLynx = [] := by decide
/-- Owlbear -/
def owlbear : Ability :=
  flavorWord "Keen Senses" (when (does thisCreature enter) (draw 1))
theorem okOwlbear : Ability.check (DRS.ofList []) owlbear = [] := by decide
/-- Netherese Puzzle-Ward -/
def netheresePuzzleWardIllumination : Ability :=
  whenever (does you (rollDice (watch := highestNatural))) (draw 1)
theorem okNetheresePuzzleWardIllumination :
    Ability.check (DRS.ofList []) netheresePuzzleWardIllumination = [] := by decide
/-- Farideh, Devil's Chosen -/
def faridehResultRead : Ability :=
  whenever (does you (rollDice anyAmount .anyDie))
    (sequentially
      [ gain thisCreature flying untilEndOfTurn,
        gain thisCreature menace untilEndOfTurn,
        doIf (anyResultIs .atLeast 10) (draw 1) ])
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
  whenever (does (counted (orMore 1) (and [isNontoken, and [isCreature, youControl]])) die)
    (create 1 (creatureTokenOf (totalOf (.stat .power) (those card))
        (totalOf (.stat .power) (those card))
        [.green] [fungus, dinosaur]))
theorem okSkullsporeNexusTrigger : Ability.check (DRS.ofList []) skullsporeNexusTrigger = [] := by decide
/-- Wavebreak Hippocamp -/
def wavebreakHippocamp : Ability :=
  whenever (nthOccurrence (.nth 1) (does you (cast (a isSpell))))
    (during := duringPart .turn eachOpponent) (draw 1)
theorem okWavebreakHippocamp : Ability.check (DRS.ofList []) wavebreakHippocamp = [] := by decide
/-- Midnight Clock -/
def midnightClockHeader : GameEvent :=
  does any (putCounters (theNth (.nth 12)) hourCounter thisArtifact)
theorem okMidnightClockHeader : GameEvent.check (DRS.ofList []) midnightClockHeader = [] := by decide
/-- Political Triumph -/
def politicalTriumphHeader : GameEvent :=
  does any (putCounters (theNth (.nth 4)) planCounter thisEnchantment)
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
  when (does thisCreature enter)
    (if_ := costWasPaid (.byNthKeyword (.nth 1) kicker) none thisCreature)
    (gainLife 3)
theorem okStormscapeBattlemageFirstKicker :
    Ability.check (DRS.ofList []) stormscapeBattlemageFirstKicker = [] := by decide
/-- All-Seeing Arbiter -/
def allSeeingArbiterHeader : GameEvent :=
  does you (discard (a
    (cardIn hand)))
theorem okAllSeeingArbiterHeader : GameEvent.check (DRS.ofList []) allSeeingArbiterHeader = [] := by decide

/-- Liliana's Caress -/
def lilianasCaress : Spelled := spelled <| singleFacedCard
  { name := "Liliana's Caress", cost := [generic 1, pip .black], types := [.enchantment],
    text :=
      [ whenever (does anOpponent (discard (a
        (cardIn hand))))
          (instruct they (loseLife 2)) ] }

/-- Scheming Aspirant -/
def schemingAspirant : Spelled := spelled <| singleFacedCard
  { name := "Scheming Aspirant", cost := [generic 1, pip .black], types := [.creature],
    subtypes := [phyrexian, advisor],
    text :=
      [ whenever (does you proliferate)
          (sequentially [instruct eachOpponent (loseLife 2), gainLife 2]) ],
    power := stat 1, toughness := stat 3 }

/-- Reciprocate -/
def reciprocate : Spelled := spelled <| singleFacedCard
  { name := "Reciprocate", cost := [pip .white], types := [.instant],
    text :=
      [ spell (exile
          (target
             (and
                [isCreature, did which (dealDamage anyAmount you) soFarThisTurn]))) ] }

/-- Tahngarth, First Mate -/
def tahngarthHeader : GameEvent := does anOpponent (attackWith (counted (orMore 1) isCreature))
theorem okTahngarthHeader : GameEvent.check (DRS.ofList []) tahngarthHeader = [] := by decide
/-- Myth Unbound -/
def mythUnboundTrigger : Ability :=
  whenever (does any (putInto yourCommander commandZone (from_ := some any))) (draw 1)
theorem okMythUnboundTrigger : Ability.check (DRS.ofList []) mythUnboundTrigger = [] := by decide

/-- Commander's Insignia -/
def commandersInsignia : Spelled := spelled <| singleFacedCard
  { name := "Commander's Insignia", cost := [generic 2, pip .white, pip .white],
    types := [.enchantment],
    text :=
      [ static (getsPt (allOf (and [isCreature, youControl]))
          (up (eventCount (cast (headed .spell yourCommander) (from_ :=
            commandZone)) you thisGame))
          (up (eventCount (cast (headed .spell yourCommander) (from_ :=
            commandZone)) you thisGame)))
            ] }

/-- Faith's Reward -/
def faithsReward : Spelled := spelled <| singleFacedCard
  { name := "Faith's Reward", cost := [generic 3, pip .white], types := [.instant],
    text :=
      [ spell (returnToBattlefield (allOf (and [ isPermanentCard, in_ yourGraveyard,
                         was (putInto which yourGraveyard (from_ := battlefield))
                           soFarThisTurn ]))) ] }

/-- Oscorp Industries -/
def oscorpIndustriesReturn : Ability :=
  when (does thisLand (enter (from_ := graveyard))) (loseLife 2)
theorem okOscorpIndustriesReturn : Ability.check (DRS.ofList []) oscorpIndustriesReturn = [] := by decide
def theLostAndTheDamnedEntryArm : GameEvent :=
  does (a (and [isLand, youControl])) (enter (from_ := some (not yourHand)))
theorem okTheLostAndTheDamnedEntryArm :
    GameEvent.check (DRS.ofList []) theLostAndTheDamnedEntryArm = [] := by decide
/-- Forsaken Wastes -/
def forsakenWastesTargeted : Ability :=
  whenever (does (a isSpell) (target thisEnchantment))
    (instruct (controllerOf (that .spell)) (loseLife 5))
theorem okForsakenWastesTargeted : Ability.check (DRS.ofList []) forsakenWastesTargeted = [] := by decide

/-- Fblthp, the Lost -/
def fblthp : Spelled := spelled <| singleFacedCard
  { name := "Fblthp, the Lost", cost := [generic 1, pip .blue],
    supertypes := [.legendary], types := [.creature], subtypes := [homunculus],
    text :=
      [ when (does this_ enter)
          (replace (draw 1)
            (doIf (or
                [ did it (enter (from_ := yourLibrary)) triggering,
                  is it (did any (cast which (from_ := yourLibrary)) thisObject) ])
              (instead (draw 2)))),
        when (does (a isSpell) (target this_)) (shuffleInto this_) ],
    power := stat 1, toughness := stat 1 }
/-- *Fblthp, the Lost*'s self-replacement "draw a card. If …, draw two cards instead" checks: a
regression guard on the replacement's starting state [CR#614.15]. -/
theorem okFblthpSelfReplacement : fblthp.card.check = [] := fblthp.ok

/-- Frost Walker -/
def frostWalker : Spelled := spelled <| singleFacedCard
  { name := "Frost Walker", cost := [generic 1, pip .blue], types := [.creature],
    subtypes := [elemental],
    text :=
      [ when (does (a (or [isSpell, abilityHead .anyOnStack])) (target thisCreature))
          (sacrifice itPermanent) ],
    power := stat 4, toughness := stat 1 }

/-- Loaming Shaman -/
def loamingShaman : Spelled := spelled <| singleFacedCard
  { name := "Loaming Shaman", cost := [generic 2, pip .green], types := [.creature],
    subtypes := [centaur, shaman],
    text :=
      [ when (does thisCreature enter)
          (instruct (target isPlayer) (shuffleInto
            (targets anyNumber (cardIn (graveyardOf they))))) ],
    power := stat 3, toughness := stat 2 }

/-- Debris Beetle -/
def debrisBeetleTrigger : Ability :=
  when (does thisVehicle enter)
    (sequentially [instruct eachOpponent (loseLife 3), gainLife 3])
theorem okDebrisBeetleTrigger : Ability.check (DRS.ofList []) debrisBeetleTrigger = [] := by decide
/-- Roads Go Ever, Ever On's chapters II and III -/
def roadsGoEverEverOnChapters : Ability :=
  when (chapterMark [2, 3]) (putInto (a (cardIn (exiledWith thisSaga))) hand)
theorem okRoadsGoEverEverOnChapters : Ability.check (DRS.ofList []) roadsGoEverEverOnChapters = [] := by decide
/-- Wurmwall Sweeper -/
def wurmwallSweeperTrigger : Ability := when (does thisSpacecraft enter) (surveil 2)
theorem okWurmwallSweeperTrigger : Ability.check (DRS.ofList []) wurmwallSweeperTrigger = [] := by decide
/-- Case of the Crimson Pulse -/
def caseOfTheCrimsonPulseTrigger : Ability :=
  when (does thisCase enter)
    (sequentially [discard (a (cardIn yourHand)), draw 2])
theorem okCaseOfTheCrimsonPulseTrigger : Ability.check (DRS.ofList []) caseOfTheCrimsonPulseTrigger = [] := by
  decide

/-- Ferocious Pup -/
def ferociousPup : Spelled := spelled <| singleFacedCard
  { name := "Ferocious Pup", cost := [generic 2, pip .green], types := [.creature],
    subtypes := [wolf],
    text :=
      [ when (does thisCreature enter)
          (create 1 (creatureToken 2 2 [.green] [wolf])) ],
    power := stat 0, toughness := stat 1 }

/-- Agency Outfitter's search -/
def agencyOutfitterSearch : Action :=
  sequentially
    [ may
        (sequentially
          [ search (andOr [yourGraveyard, yourHand, yourLibrary])
              (andOr [a (named "Magnifying Glass"), a (named "Thinking Cap")]),
            putOntoBattlefield them ]),
      doIf (did you (search yourLibrary) thisWay) shuffle ]
theorem okAgencyOutfitterSearch : Action.check (DRS.ofList []) agencyOutfitterSearch = [] := by decide
/-- Agency Outfitter's "them" reads the and/or group of the cards it searched for. -/
theorem agencyOutfitterThemIsTheFoundGroup :
    let found := nextClause (stepAction 512 .postEvent .obj (DRS.ofList [])
      (search (andOr [yourGraveyard, yourHand, yourLibrary])
        (andOr [a (named "Magnifying Glass"), a (named "Thinking Cap")]))).1
    (resolveCandidates .bare .many found).map (fun ref => (ref.plur, ref.groupKey == ref.id)) =
      [(.many, true)] := by decide

/-- Trouble in Pairs' -/
def troubleInPairsArms : Ability :=
  whenever (does anOpponent (attackWith (counted (orMore 2) isCreature) you))
    (or_ :=
      [ nthOccurrence (.nth 2) (does (actor anOpponent) (draw anyAmount)) (some .turn),
        nthOccurrence (.nth 2) (does anOpponent (cast (a isSpell))) (some .turn) ])
    (draw 1)
theorem okTroubleInPairsArms : Ability.check (DRS.ofList []) troubleInPairsArms = [] := by decide

private def castRank? : Predicate → Option (Ordinal × Option Period)
  | .did clause => clause.ordinal
  | _ => none

private def eventOrdinal? : GameEvent → Option (Ordinal × Option Period)
  | .nthOccurrence ordinal per _ => (ordinal, per)
  | _ => none

private def triggerAlternatives : Ability → List GameEvent
  | .triggered _ alternatives .. => alternatives
  | _ => []

/-- "The first spell you've cast this game" and "your second spell each turn" carry one ordinal
shape, an `Ordinal` counted within an optional `Period`. -/
theorem onceUponATimeAndTroubleInPairsShareTheOrdinalShape :
    castRank? onceUponATimeFirstCast = some (.nth 1, some .game) ∧
      (triggerAlternatives troubleInPairsArms).map eventOrdinal? =
        [some (.nth 2, some .turn), some (.nth 2, some .turn)] := by decide

/-- Bioplasm -/
def bioplasm : Spelled := spelled <| singleFacedCard
  { name := "Bioplasm", cost := [generic 3, pip .green, pip .green], types := [.creature],
    subtypes := [ooze],
    text :=
      [ whenever (does thisCreature attack)
          (sequentially
            [ exile (topSlice 1),
              doIf bioplasmCardTest
                (get thisCreature
                  (up (powerOf (the (and [was (exile which), isCreature, isCard]))))
                  (up (toughnessOf it))
                  thisTurn) ]) ],
    power := stat 4, toughness := stat 4 }

/-- Hostile Investigator -/
def hostileInvestigatorHeader : GameEvent :=
  does (counted (orMore 1) isPlayer) (discard (counted (orMore 1) isCard))
theorem okHostileInvestigatorHeader : GameEvent.check (DRS.ofList []) hostileInvestigatorHeader = [] := by decide

/-- Hallowed Moonlight -/
def hallowedMoonlight : Spelled := spelled <| singleFacedCard
  { name := "Hallowed Moonlight", cost := [generic 1, pip .white], types := [.instant],
    text :=
      [ spell (sequentially
          [ replaceEvent (would (a (and [isCreature, not (did any (cast which) thisObject)])) enter)
              (instead (exile it))
              untilEndOfTurn,
            draw 1 ]) ] }

/-- Gather Specimens -/
def gatherSpecimens : Spelled := spelled <| singleFacedCard
  { name := "Gather Specimens", cost := [generic 3, pip .blue, pip .blue, pip .blue],
    types := [.instant],
    text :=
      [ spell
          (establishThroughout
             (replace (would (a isCreature) (enter (riders := [under anOpponent])))
                (instead (instruct it (enter (riders := [under you])))))
             thisTurn) ] }

/-- Don't Blink's replacement, without its written agent -/
def dontBlinkReplacement : Action :=
  establishThroughout
    (replace
      (wouldAny
        [ would (counted (orMore 1) isCreature) (enter (from_ := exileZone)),
          would (counted (orMore 1) (and [isCreature,
              did any (cast which (from_ := exileZone)) thisObject])) enter ])
      (instead (shuffleInto them)))
    untilEndOfTurn
theorem okDontBlinkReplacement : Action.check (DRS.ofList []) dontBlinkReplacement = [] := by decide

/-- Seasoned Warrenguard -/
def seasonedWarrenguard : Spelled := spelled <| singleFacedCard
  { name := "Seasoned Warrenguard", cost := [pip .white], types := [.creature],
    subtypes := [rabbit, warrior],
    text :=
      [ whenever (does thisCreature attack)
          (while_ := whileTrue (exists_ (and [isToken, youControl])))
          (get thisCreature (up 2) (up 0) untilEndOfTurn) ],
    power := stat 1, toughness := stat 2 }

/-- Brazen Blademaster -/
def brazenBlademaster : Spelled := spelled <| singleFacedCard
  { name := "Brazen Blademaster", cost := [generic 2, pip .red], types := [.creature],
    subtypes := [orc, pirate],
    text :=
      [ whenever (does thisCreature attack)
          (while_ := whileTrue
            (compareAmt (countOf (and [isArtifact, youControl])) .atLeast
              2))
          (get it (up 2) (up 1) untilEndOfTurn) ],
    power := stat 2, toughness := stat 3 }

/-- Autarch Mammoth -/
def autarchMammothLine : Ability :=
  when (does thisCreature enter)
    (joins := [ joinedHeadWhile (does thisCreature attack)
        (whileTrue (is thisCreature (hasDesignation saddled))) ])
    (create 1 (creatureToken 3 3 [.green] [elephant]))
theorem okAutarchMammothLine : Ability.check (DRS.ofList []) autarchMammothLine = [] := by decide
/-- Altar of the Brood -/
def altarOfTheBrood : Ability :=
  whenever (does (a (and [isPermanent, youControl, isOther])) enter)
    (instruct eachOpponent (mill 1))
theorem okAltarOfTheBrood : Ability.check (DRS.ofList []) altarOfTheBrood = [] := by decide
/-- Foul Emissary -/
def foulEmissaryLine : Ability :=
  when (does you (sacrifice thisCreature))
    (while_ := whileDoing (does you (cast (a
      (and [isSpell, hasKeyword (.the emerge)])))))
    (create 1 (creatureToken 3 2 [] [eldrazi, horror]))
theorem okFoulEmissaryLine : Ability.check (DRS.ofList []) foulEmissaryLine = [] := by decide

/-- Bramble Elemental -/
def brambleElemental : Spelled := spelled <| singleFacedCard
  { name := "Bramble Elemental", cost := [generic 3, pip .green, pip .green],
    types := [.creature], subtypes := [elemental],
    text :=
      [ whenever (does any (attachment .attached (a aura) thisCreature))
          (create 2 (creatureToken 1 1 [.green] [saproling])) ],
    power := stat 4, toughness := stat 4 }

/-- Stone Haven Outfitter -/
def stoneHavenOutfitter : Spelled := spelled <| singleFacedCard
  { name := "Stone Haven Outfitter", cost := [generic 1, pip .white], types := [.creature],
    subtypes := [kor, artificer, ally],
    text :=
      [ static (getsPt (allOf (and [and [isCreature, youControl], isAttached (some .equipped)]))
          (up 1) (up 1)),
        whenever (does (a (and [and [isCreature, youControl], isAttached (some .equipped)])) die)
          (draw 1) ],
    power := stat 2, toughness := stat 2 }

/-- Cloud, Ex-SOLDIER -/
def cloudExSoldierAttach : Ability :=
  when (does thisCreature enter)
    (attachToIt
      (targets (upTo 1) (and [equipment, youControl])))
theorem okCloudExSoldierAttach : Ability.check (DRS.ofList []) cloudExSoldierAttach = [] := by decide
/-- Akiri, Fearless Voyager -/
def akiriEquippedAttackers : Ability :=
  whenever
    (does you
       (attackWith
          (counted (orMore 1) (and [and [isCreature, youControl], isAttached (some .equipped)]))
        (a isPlayer)))
    (draw 1)
theorem okAkiriEquippedAttackers : Ability.check (DRS.ofList []) akiriEquippedAttackers = [] := by decide
/-- Vexing Bauble -/
def vexingBaubleTrigger : Ability :=
  whenever (does (a isPlayer) (cast (a isSpell)))
    (if_ := noManaSpentToCast it)
    (counter (that .spell))
theorem okVexingBaubleTrigger : Ability.check (DRS.ofList []) vexingBaubleTrigger = [] := by decide

/-- Void Mirror -/
def voidMirror : Spelled := spelled <| singleFacedCard
  { name := "Void Mirror", cost := [generic 2], types := [.artifact],
    text :=
      [ whenever (does (a isPlayer) (cast (a
        isSpell))) (if_ := noColoredManaSpentToCast it)
          (counter (that .spell)) ] }

/-- Blood Sun -/
def bloodSun : Spelled := spelled <| singleFacedCard
  { name := "Blood Sun", cost := [generic 2, pip .red], types := [.enchantment],
    text :=
      [ when (does thisEnchantment enter) (draw 1),
        static (allAbilityLoss (allOf isLand) isManaAbility) ] }

/-- Promise of Bunrei -/
def promiseOfBunrei : Spelled := spelled <| singleFacedCard
  { name := "Promise of Bunrei", cost := [generic 2, pip .white], types := [.enchantment],
    text :=
      [ when (does (a (and [isCreature, youControl])) die)
          (doIfDone (sacrifice thisEnchantment)
            (create 4 (creatureToken 1 1 [] [spirit]))) ] }

/-- Grave Peril -/
def gravePeril : Spelled := spelled <| singleFacedCard
  { name := "Grave Peril", cost := [generic 1, pip .black], types := [.enchantment],
    text :=
      [ when (does (a (and [isCreature, non (hasColor .black)])) enter)
          (doIfDone (sacrifice thisEnchantment) (destroy (that (.type
              .creature)))) ] }

/-- Blood Reckoning -/
def bloodReckoning : Spelled := spelled <| singleFacedCard
  { name := "Blood Reckoning", cost := [generic 3, pip .black], types := [.enchantment],
    text :=
      [ whenever
          (does (a isCreature)
             (attack (youOr (a (and [isPlaneswalker, youControl])))))
          (instruct (controllerOf (that creature)) (loseLife 1)) ] }

/-- Jeskai Ascendancy's first trigger -/
def jeskaiAscendancyPump : Ability :=
  whenever (does you (cast (a (and
    [isSpell, non isCreature]))))
    (sequentially
      [ get (bare (and [isCreature, youControl])) (up 1) (up 1) untilEndOfTurn,
        untap (those creature) ])
theorem okJeskaiAscendancyPump : Ability.check (DRS.ofList []) jeskaiAscendancyPump = [] := by decide
def herosDemise : Action := destroy (target (and [isCreature, hasSupertype .legendary]))
theorem okHerosDemise : Action.check (DRS.ofList []) herosDemise = [] := by decide

def repayInKind : Spelled := spelled <| singleFacedCard
  { name := "Repay in Kind", cost := [generic 5, pip .black, pip .black], types := [.sorcery],
    text :=
      [ spell (instruct eachPlayer (setLife (aggregate .min (.playerStat .lifeTotal) isPlayer)))
          ] }

/-- Squelch -/
def squelch : Spelled := spelled <| singleFacedCard
  { name := "Squelch", cost := [generic 1, pip .blue], types := [.instant],
    text :=
      [ spell (sequentially
          [counter (target (abilityHead .anyActivated)), draw 1]) ]
              }

theorem bioplasmTwoCandidates : countOnes .object bioplasmAfterExile = 2 := by decide

/-- "From anywhere" observes Dread after its arrival in the graveyard. -/
def dread : Spelled := spelled <| singleFacedCard
  { name := "Dread", cost := [generic 3, pip .black, pip .black, pip .black],
    types := [.creature], subtypes := [elemental, incarnation],
    text :=
      [ fear,
        whenever (does (a isCreature) (dealDamage (to := you))) (destroy it),
        when (does any (putInto this_ graveyard (from_ := some any)))
          (instruct (ownerOf it) (shuffleInto it)) ],
    power := stat 6, toughness := stat 6 }

private semantic_template creatureDamagedByAttachedHostThisTurn (attachment : AttachWord) : NounPhrase :=
  a (.and [isCreature,
    .did { subj := .actor (.attachHost attachment creature)
           action := .dealDamage .anyAmount .which
           within := some (.duration soFarThisTurn none) }])

/-- Vampiric Embrace. -/
def vampiricEmbrace : Spelled := spelled <| singleFacedCard
  { name := "Vampiric Embrace", cost := [generic 2, pip .black, pip .black],
    types := [.enchantment], subtypes := [aura],
    text :=
      [ enchant isCreature,
        static (getsPt
          (enchanted creature)
          (up 2) (up 2)),
        static (abilityGrant
          (enchanted creature) flying),
        whenever (does (creatureDamagedByAttachedHostThisTurn .enchanted) die)
          (putCounters 1 p1p1Counter
            (that creature)) ] }

/-- Scythe of the Wretched. -/
def scytheOfTheWretched : Spelled := spelled <| singleFacedCard
  { name := "Scythe of the Wretched", cost := [generic 2], types := [.artifact],
    subtypes := [equipment],
    text :=
      [ static (getsPt
          (equipped creature)
          (up 2) (up 2)),
        whenever (does (creatureDamagedByAttachedHostThisTurn .equipped) die)
          (sequentially
            [ returnToBattlefield (that card)
                [(under you)],
              attachTo thisEquipment (that creature) ]),
        equip [generic 4] ] }

/-- Academy Researchers: the Aura enters attached to the creature the effect names, so enchant's
own choice of host doesn't apply; one that can't legally enchant it stays in its owner's hand
[CR#303.4f,303.4i]. -/
def academyResearchers : Spelled := spelled <| singleFacedCard
  { name := "Academy Researchers", cost := [generic 1, pip .blue, pip .blue],
    types := [.creature], subtypes := [human, wizard],
    text :=
      [ when (does thisCreature enter)
          (may (putOntoBattlefieldAttachedTo (a (and [hasSubtype aura, cardIn yourHand]))
            thisCreature)) ],
    power := stat 2, toughness := stat 2 }

/-- Bruna, Light of Alabaster: Auras already on the battlefield are attached [CR#701.3a]; Aura
cards are put onto the battlefield attached to it, each one that could enchant it
[CR#303.4i]. -/
def brunaLightOfAlabaster : Spelled := spelled <| singleFacedCard
  { name := "Bruna, Light of Alabaster", cost := [generic 3, pip .white, pip .white, pip .blue],
    supertypes := [.legendary], types := [.creature], subtypes := [angel],
    text :=
      [ flying, vigilance,
        whenever (does thisCreature attack) (or_ := [does thisCreature block])
          (sequentially
            [ may (attachTo (counted anyNumber (and [hasSubtype aura, in_ battlefield])) it),
              may (putOntoBattlefieldAttachedTo
                (counted anyNumber
                  (and [hasSubtype aura, cardIn (andOr [yourGraveyard, yourHand]),
                    couldEnchant it]))
                it) ]) ],
    power := stat 5, toughness := stat 5 }

/-- Baird, Argivian Recruiter: "if you control a creature with power greater than its base
power", each creature measured against its own base power [CR#208.4b]. -/
def bairdArgivianRecruiter : Spelled := spelled <| singleFacedCard
  { name := "Baird, Argivian Recruiter", cost := [pip .red, pip .white],
    supertypes := [.legendary], types := [.creature], subtypes := [human, soldier],
    text :=
      [ at_ (beginningOfYour .endStep)
          (if_ := exists_ (comparesOwnStat .power (and [isCreature, youControl])
            .greater (basePowerOf ownMember)))
          (create 1 (creatureToken 1 1 [.white] [soldier])) ],
    power := stat 2, toughness := stat 2 }

/-- Curie, Emergent Intelligence's first ability: "Whenever Curie deals combat damage to a
player, draw cards equal to its base power" [CR#208.4b]. -/
def curieCombatDraw : Ability :=
  whenever (does thisCreature (dealDamage (to := a isPlayer) (kind := some .combat)))
    (draw (basePowerOf thisCreature))
theorem okCurieCombatDraw : Ability.check (DRS.ofList []) curieCombatDraw = [] := by decide

end Semantics.Cards
