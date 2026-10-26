import Semantics
import Semantics.Definitions
import Semantics.Check.Card

/-!
# Semantics.Cards.Description

Port of `idris/src/Experimental/Cards/Description.idr`: the printed cards of the Description
family, and the phrase-level bench items beside them (a noun, predicate, condition,
action, or ability that elaborated in Idris), each paired with an `ok…` theorem that runs
the checker on it at the bindings its Idris type named.
-/

open Semantics Semantics.Templates Semantics.Definitions

namespace Semantics.Cards

/-- Glyph of Destruction -/
def glyphOfDestruction : Action :=
  get (target (and [isBlocking, wall, youControl]))
    (up 10) (up 0) untilEndOfCombat
theorem okGlyphOfDestruction : Action.check (DRS.ofList []) glyphOfDestruction = [] := by decide

def rawNonattacking : Predicate := and [isCreature, non isAttacking, non isBlocking]
theorem okRawNonattacking : Predicate.checkAt .object (DRS.ofList []) rawNonattacking = [] := by decide

/-- Harmony of Nature -/
def harmonyOfNature : Action :=
  sequentially
    [ tap (counted anyNumber (and [isUntapped, isCreature, youControl])),
      doForEach (each (and [isCreature, was (tap which) thisWay]))
        (gainLife 4) ]
theorem okHarmonyOfNature : Action.check (DRS.ofList []) harmonyOfNature = [] := by decide

/-- Rats of Rath -/
def ratsOfRath : Action :=
  destroy (target (and [or [isArtifact, isCreature, isLand], youControl]))
theorem okRatsOfRath : Action.check (DRS.ofList []) ratsOfRath = [] := by decide

/-- "another creature or land", after a target on the battlefield. -/
def anotherDisjunctPhrase : Predicate := and [or [isCreature, isLand], isOther]
theorem okAnotherDisjunctPhrase :
    Predicate.checkAt .object (DRS.ofList [{ det := .target, plur := .one, payload := .object [] (some .battlefield) none none none }]) anotherDisjunctPhrase = [] := by
  decide

/-- Defeat -/
def defeat : Action :=
  destroy (target (and [isCreature, hasPower .atMost 2]))
theorem okDefeat : Action.check (DRS.ofList []) defeat = [] := by decide

/-- Terashi's Verdict -/
def terashisVerdict : Action :=
  destroy (target (and [isCreature, isAttacking, hasPower .atMost 3]))
theorem okTerashisVerdict : Action.check (DRS.ofList []) terashisVerdict = [] := by decide

/-- Pillar of Light -/
def pillarOfLight : Action :=
  exile (target (and [isCreature, hasToughness .atLeast 4]))
theorem okPillarOfLight : Action.check (DRS.ofList []) pillarOfLight = [] := by decide

/-- Unholy Annex -/
def unholyAnnex : Action :=
  sequentially
    [ draw 1,
      doIf (exists_ (and [demon, youControl]))
        (sequentially [instruct eachOpponent (loseLife 2), gainLife 2])
        (loseLife 2) ]
theorem okUnholyAnnex : Action.check (DRS.ofList []) unholyAnnex = [] := by decide

/-- War Screecher -/
def warScreecher : Action :=
  get (allOf (and [isCreature, youControl, isOther])) (up 1) (up 1)
    untilEndOfTurn
theorem okWarScreecher : Action.check (DRS.ofList []) warScreecher = [] := by decide

/-- Mind Flayer -/
def mindFlayer : Action :=
  gainControl (target isCreature)
    (forAsLongAs (is thisCreature youControl)) (controller := you)
theorem okMindFlayer : Action.check (DRS.ofList []) mindFlayer = [] := by decide

/-- Arc Lightning -/
def arcLightning : Action :=
  dealDivided this_ 3 (targets (oneThrough 3) isAnyTarget)
theorem okArcLightning : Action.check (DRS.ofList []) arcLightning = [] := by decide

/-- Boulderfall -/
def boulderfall : Action :=
  dealDivided this_ 5 (targets anyNumber isAnyTarget)
theorem okBoulderfall : Action.check (DRS.ofList []) boulderfall = [] := by decide

/-- Banisher Priest -/
def banisherPriest : Action :=
  exileUntil (target (and [isCreature, anOpponentControls]))
    (does thisCreature leave)
theorem okBanisherPriest : Action.check (DRS.ofList []) banisherPriest = [] := by decide

/-- Tezzeret, Artifice Master -/
def tezzeretDrawTwo : Action :=
  replace (draw 1)
    (doIf (compareAmt (countOf (and [isArtifact, youControl]))
            .atLeast 3)
      (instead (draw 2)))
theorem okTezzeretDrawTwo : Action.check (DRS.ofList []) tezzeretDrawTwo = [] := by decide

/-- Zimone, Quandrix Prodigy -/
def zimoneDrawTwo : Action :=
  replace (draw 1)
    (doIf (compareAmt (countOf (and [isLand, youControl])) .atLeast 8)
      (instead (draw 2)))
theorem okZimoneDrawTwo : Action.check (DRS.ofList []) zimoneDrawTwo = [] := by decide

/-- Aerial Volley {G} — Instant. "Aerial Volley deals 3 damage divided as you choose among
one, two, or three target creatures with flying." -/
def aerialVolley : Spelled := spelled <| singleFacedCard
  { name := "Aerial Volley", cost := [pip .green], types := [.instant],
    text :=
      [ spell (dealDivided this_ 3
          (targets (oneThrough 3) (and [isCreature, hasKeyword flying]))) ] }

/-- Terrifying Presence -/
def terrifyingPresenceAnchor : Predicate := and [isCreature, otherThan (target isCreature)]
theorem okTerrifyingPresenceAnchor : Predicate.checkAt .object (DRS.ofList []) terrifyingPresenceAnchor = [] := by
  decide

/-- Timely Reinforcements -/
def timelyReinforcements : Spelled := spelled <| singleFacedCard
  { name := "Timely Reinforcements", cost := [generic 2, pip .white], types := [.sorcery],
    text :=
      [ spell (sequentially
          [ doIf (compareAmt (lifeTotalOf you) .less (lifeTotalOf anOpponent))
              (gainLife 6),
            doIf (compareAmt (countOf (and [isCreature, youControl])) .less
                    (countOf (and [isCreature, anOpponentControls])))
              (create 3 (creatureToken 1 1 [.white] [soldier])) ]) ] }

/-- Survival Cache -/
def survivalCache : Action :=
  sequentially
    [ gainLife 2,
      doIf (compareAmt (lifeTotalOf you) .greater (lifeTotalOf anOpponent))
        (draw 1) ]
theorem okSurvivalCache : Action.check (DRS.ofList []) survivalCache = [] := by decide

/-- Nightmarish End -/
def nightmarishEnd : Spelled := spelled <| singleFacedCard
  { name := "Nightmarish End", cost := [generic 2, pip .black], types := [.instant],
    text :=
      [ spell (sequentially
          [ get (target isCreature) (down (letter .x)) (down (letter .x))
              untilEndOfTurn,
            define .x (countOf (cardIn yourHand)) ]) ] }

/-- Topple {2}{W} — Sorcery. "Exile target creature with the greatest power among creatures
on the battlefield." -/
def topple : Spelled := spelled <| singleFacedCard
  { name := "Topple", cost := [generic 2, pip .white], types := [.sorcery],
    text :=
      [ spell (exile (target
          (and [isCreature,
                 superlative .max (.stat .power) (and [isCreature, isPermanent])]))) ] }

/-- "each opponent with three or more poison counters" -/
def corruptedOpponents : NounPhrase :=
  each (and [isOpponent, compare .atLeast 3 (axes := [.counter poison])])
theorem okCorruptedOpponents : NounPhrase.checkAt (some .player) (DRS.ofList []) corruptedOpponents = [] := by
  decide

/-- War Tax -/
def warTaxScaledPayment : Action :=
  establishThroughout (gatedBy (scaledGenericMana
        (forEach (letter .x) (and [isCreature, isAttacking,
            controlledBy they]))) (allOf isCreature) attack) thisTurn
theorem okWarTaxScaledPayment : Action.check (DRS.ofList [letterB .x]) warTaxScaledPayment = [] := by
  decide

/-- Croaking Counterpart -/
def croakingCounterpartCopy : Action :=
  create 1
    (copyOf (target (and [isCreature, non frog]))
      (copyCharacteristics {
                 colors := [.green], subtypes := [frog], power := stat 1,
                 toughness := stat 1 } false))
    []
theorem okCroakingCounterpartCopy : Action.check (DRS.ofList []) croakingCounterpartCopy = [] := by decide

/-- Blessed Reversal -/
def blessedReversal : Spelled := spelled <| singleFacedCard
  { name := "Blessed Reversal", cost := [generic 1, pip .white], types := [.instant],
    text :=
      [ spell (gainLife
          (forEach 3 (and [isCreature, inCombat .attackerOf you]))) ] }

/-- Extinction -/
def extinction : Spelled := spelled <| singleFacedCard
  { name := "Extinction", cost := [generic 4, pip .black], types := [.sorcery],
    text :=
      [ spell (destroy (allOf (and [isCreature, ofYourChoice (.subtype .creature)]))) ] }

/-- Defensive Maneuvers -/
def defensiveManeuvers : Spelled := spelled <| singleFacedCard
  { name := "Defensive Maneuvers", cost := [generic 3, pip .white], types := [.instant],
    text :=
      [ spell (get (allOf (and [isCreature, ofYourChoice (.subtype .creature)]))
          (up 0) (up 4) untilEndOfTurn) ] }

/-- Witch's Vengeance -/
def witchsVengeance : Spelled := spelled <| singleFacedCard
  { name := "Witch's Vengeance", cost := [generic 1, pip .black, pip .black],
    types := [.sorcery],
    text :=
      [ spell (get (allOf (and [isCreature, ofYourChoice (.subtype .creature)]))
          (down 3) (down 3) untilEndOfTurn) ] }

/-- Phyrexian Rebirth -/
def phyrexianRebirth : Spelled := spelled <| singleFacedCard
  { name := "Phyrexian Rebirth", cost := [generic 4, pip .white, pip .white],
    types := [.sorcery],
    text :=
      [ spell (sequentially
          [ destroy (allOf isCreature),
            create 1 (written { characteristics :=
                  { types := [.artifact, .creature],
                    subtypes := [phyrexian, horror],
                    power := letter .x, toughness := (letter .x) } }),
            define .x
                (countOf (and [isCreature, was (destroy which) thisWay])) ]) ] }

/-- Damning Verdict -/
def damningVerdict : Spelled := spelled <| singleFacedCard
  { name := "Damning Verdict", cost := [generic 3, pip .white, pip .white],
    types := [.sorcery],
    text := [ spell (destroy (allOf (and [isCreature, not hasCounter]))) ] }

/-- Hazardous Conditions -/
def hazardousConditions : Spelled := spelled <| singleFacedCard
  { name := "Hazardous Conditions", cost := [generic 2, pip .black, pip .green],
    types := [.sorcery],
    text :=
      [ spell (get (allOf (and [isCreature, not hasCounter]))
          (down 2) (down 2) untilEndOfTurn) ] }

/-- Approach of the Second Sun -/
def approachOfTheSecondSun : Spelled := spelled <| singleFacedCard
  { name := "Approach of the Second Sun", cost := [generic 6, pip .white],
    types := [.sorcery],
    text :=
      [ spell (doIf
          (and
            [ is this_ (did any (cast which (from_ := yourHand)) thisObject),
              did you (cast (a
                (and [ isSpell, isOther,
                           named "Approach of the Second Sun" ]))) thisGame ])
          winGame
          (sequentially
            [ putInto this_ (fromTop 7 library), gainLife 7 ])) ] }

/-- "a loyalty ability of enchanted planeswalker" -/
def loyaltyAbilityOfEnchanted : NounPhrase :=
  a (and [abilityHead .loyalty, abilityOf (enchanted planeswalker)])
theorem okLoyaltyAbilityOfEnchanted :
    NounPhrase.checkAt (some .object) (DRS.ofList []) loyaltyAbilityOfEnchanted = [] := by
  decide

/-- Balance of Power -/
def balanceOfPower : Action :=
  doIf (compareAmt (countOf (cardIn (handOf (target isOpponent)))) .greater
          (countOf (cardIn yourHand)))
    (draw theDifference)
theorem okBalanceOfPower : Action.check (DRS.ofList []) balanceOfPower = [] := by decide

/-- Spark Fiend -/
def sparkFiendUpkeepRoll : Action :=
  sequentially
    [ rollDice 2 (.sides 6),
      doIf (compareAmt (theOutcome .rollResult) .eq 7) (sacrifice thisCreature) ]
theorem okSparkFiendUpkeepRoll : Action.check (DRS.ofList []) sparkFiendUpkeepRoll = [] := by decide

/-- Erebos, God of the Dead -/
def devotionCondition : Condition := compareAmt (devotion you (color .black)) .less 5
theorem okDevotionCondition : Condition.check (DRS.ofList []) devotionCondition = [] := by decide

/-- Multiple Choice -/
def multipleChoiceFirstArm : Action :=
  doIf (compareAmt (letter .x) .eq 1)
    (sequentially [scry 1, draw 1])
theorem okMultipleChoiceFirstArm : Action.check (DRS.ofList []) multipleChoiceFirstArm = [] := by decide

/-- Multiple Choice, fourth arm -/
def multipleChoiceFourthGate : Condition := compareAmt (letter .x) .atLeast 4
theorem okMultipleChoiceFourthGate : Condition.check (DRS.ofList []) multipleChoiceFourthGate = [] := by decide

/-- Fell the Mighty -/
def fellTheMighty : Action :=
  destroy
    (allOf (and [isCreature,
                  hasPower .greater (powerOf (target isCreature))]))
theorem okFellTheMighty : Action.check (DRS.ofList []) fellTheMighty = [] := by decide

def targetPlayerOrPlaneswalker : NounPhrase := target (or [isPlaneswalker, isPlayer])
theorem okTargetPlayerOrPlaneswalker :
    NounPhrase.checkAt none (DRS.ofList []) targetPlayerOrPlaneswalker = [] := by decide

def targetOpponentOrPlaneswalker : NounPhrase := target (or [isPlaneswalker, isOpponent])
theorem okTargetOpponentOrPlaneswalker :
    NounPhrase.checkAt none (DRS.ofList []) targetOpponentOrPlaneswalker = [] := by decide

/-- Baleful Mastery's paid read -/
def balefulMasteryPaidRead : Ability :=
  spell (doIf (costWasPaid .theAlternative none this_) (instruct anOpponent (draw 1)))
theorem okBalefulMasteryPaidRead : Ability.check (DRS.ofList []) balefulMasteryPaidRead = [] := by decide

/-- Karai, Future of the Foot -/
def karaiSneakPaidThisTurn : Amount := paidCostRead (.byKeyword sneak) thisTurn this_
theorem okKaraiSneakPaidThisTurn : Amount.check (DRS.ofList []) karaiSneakPaidThisTurn = [] := by decide

/-- Requiting Hex's read -/
def requitingHexAdditionalRead : Ability :=
  spell (doIf (costWasPaid .theAdditional none this_) (gainLife 2))
theorem okRequitingHexAdditionalRead : Ability.check (DRS.ofList []) requitingHexAdditionalRead = [] := by
  decide

/-- Lucid Dreams -/
def lucidDreams : Spelled := spelled <| singleFacedCard
  { name := "Lucid Dreams", cost := [generic 3, pip .blue, pip .blue], types := [.sorcery],
    text :=
      [ spell (sequentially
          [ draw (letter .x),
            define .x (distinctCount .cardType (allOf (cardIn yourGraveyard))) ]) ] }

/-- Rogues' Gallery -/
def roguesGallery : Spelled := spelled <| singleFacedCard
  { name := "Rogues' Gallery", cost := [generic 2, pip .black], types := [.sorcery],
    text :=
      [ spell
          (doForEachKind .color .color
             (returnToHand
                (targets (upTo 1) (and [isCreature, ofChosen .color, in_ yourGraveyard]))
                yourGraveyard)) ] }

/-- Bioplasm -/
def bioplasmAttack : GameEvent := does thisCreature attack
theorem okBioplasmAttack : GameEvent.check (DRS.ofList []) bioplasmAttack = [] := by decide
def bioplasmExile : Action := exile (topSlice 1)
theorem okBioplasmExile :
    Action.check ((stepGameEvent 512 (DRS.ofList []) bioplasmAttack).1) bioplasmExile = [] := by decide
def bioplasmAfterExile : DRS := (stepAction 512 .postEvent .obj ((stepGameEvent 512 (DRS.ofList []) bioplasmAttack).1) bioplasmExile).1
def bioplasmExiledCard : NounPhrase := the (and [was (exile which), isCard])
theorem okBioplasmExiledCard :
    NounPhrase.checkAt (some .object) bioplasmAfterExile bioplasmExiledCard = [] := by decide

/-- "the top card of each player's library" -/
def playersTopCardSlice : NounPhrase := slice .top 1 (libraryOf eachPlayer)
theorem okPlayersTopCardSlice : NounPhrase.checkAt (some .object) (DRS.ofList []) playersTopCardSlice = [] := by
  decide

/-- Deepglow Skate's recipient -/
def deepglowSkateRecipient : NounPhrase := targets anyNumber isPermanent
theorem deepglowSkateRecipientRefused : deepglowSkateRecipient.perMemberOk = false := by decide

/-- Weftwalking -/
def weftwalkingShuffle : Action :=
  sequentially
    [ shuffleInto (both (allOf (cardIn yourHand)) (allOf (cardIn yourGraveyard))),
      draw 7 ]
theorem okWeftwalkingShuffle : Action.check (DRS.ofList []) weftwalkingShuffle = [] := by decide

/-- Gaea's Revenge's protection-shaped phrase -/
def nongreenSpellsOrAbilities : Predicate :=
  or [ and [isSpell, non (hasColor .green)],
        and [ abilityHead .anyOnStack,
               abilityOf (a (and [isSource, non (hasColor .green)])) ] ]
theorem okNongreenSpellsOrAbilities :
    Predicate.checkAt .object (DRS.ofList []) nongreenSpellsOrAbilities = [] := by decide

/-- Hot Pursuit -/
def twoOrMorePlayersHaveLost : Condition :=
  compareAmt (countOf (and
    [isPlayer, did which loseGame thisGame])) .atLeast 2
theorem okTwoOrMorePlayersHaveLost : Condition.check (DRS.ofList []) twoOrMorePlayersHaveLost = [] := by decide

def commanderCreaturesYouOwn : Predicate :=
  and [isCreature, hasDesignation commander, youOwn]
theorem okCommanderCreaturesYouOwn : Predicate.checkAt .object (DRS.ofList []) commanderCreaturesYouOwn = [] := by
  decide

/-- Idol of Endurance -/
def creatureSpellFromAmongExiled : NounPhrase :=
  fromAmong (exactly 1) (and [isCreature, isSpell]) (allOf (cardIn (exiledWith thisArtifact)))
theorem okCreatureSpellFromAmongExiled :
    NounPhrase.checkAt (some .object) (DRS.ofList []) creatureSpellFromAmongExiled = [] := by decide

def ringHasTemptedYouTwiceThisGame : Condition :=
  compareAmt (eventCount theRingTemptsYou you thisGame)
    .atLeast 2
theorem okRingHasTemptedYouTwiceThisGame :
    Condition.check (DRS.ofList []) ringHasTemptedYouTwiceThisGame = [] := by decide

def creatureCardAnywhere : Predicate := and [isCreature, isCard]
theorem okCreatureCardAnywhere : Predicate.checkAt .object (DRS.ofList []) creatureCardAnywhere = [] := by decide
def permanentCardAnywhere : Predicate := isPermanentCard
theorem okPermanentCardAnywhere : Predicate.checkAt .object (DRS.ofList []) permanentCardAnywhere = [] := by decide

/-- "target card on the stack" -/
def targetCardOnTheStack : NounPhrase := target cardOnTheStack
theorem okTargetCardOnTheStack : NounPhrase.checkAt (some .object) (DRS.ofList []) targetCardOnTheStack = [] := by
  decide
/-- "each token on the battlefield" -/
def eachTokenOnTheBattlefield : NounPhrase := allOf tokenOnTheBattlefield
theorem okEachTokenOnTheBattlefield :
    NounPhrase.checkAt (some .object) (DRS.ofList []) eachTokenOnTheBattlefield = [] := by decide
/-- "each emblem you own" -/
def eachEmblemYouOwn : NounPhrase := allOf (and [isEmblem, youOwn])
theorem okEachEmblemYouOwn : NounPhrase.checkAt (some .object) (DRS.ofList []) eachEmblemYouOwn = [] := by decide
/-- "a copy of a card" -/
def aCopyOfACard : NounPhrase := a isCopyOfACard
theorem okACopyOfACard : NounPhrase.checkAt (some .object) (DRS.ofList []) aCopyOfACard = [] := by decide

/-- Keeper of the Flame, Keeper of the Light -/
def opponentWithMoreLifeThanYou : Predicate :=
  and [isOpponent, compare .greater (lifeTotalOf you) (axes := [.playerStat .lifeTotal])]
theorem okOpponentWithMoreLifeThanYou :
    Predicate.checkAt .player (DRS.ofList []) opponentWithMoreLifeThanYou = [] := by decide
/-- Namor, Atlantean King -/
def playerWithMoreLifeThanYou : Predicate :=
  and [isPlayer, compare .greater (lifeTotalOf you) (axes := [.playerStat .lifeTotal])]
theorem okPlayerWithMoreLifeThanYou :
    Predicate.checkAt .player (DRS.ofList []) playerWithMoreLifeThanYou = [] := by decide
def noOpponentHasMoreLifeThanYou : Condition :=
  not (exists_
         (and [isOpponent, compare .greater (lifeTotalOf you) (axes := [.playerStat .lifeTotal])]))
theorem okNoOpponentHasMoreLifeThanYou :
    Condition.check (DRS.ofList []) noOpponentHasMoreLifeThanYou = [] := by decide
def someOpponentLacksMoreLifeThanYou : Condition :=
  exists_ (and [isOpponent,
                 not (compare .greater (lifeTotalOf you) (axes := [.playerStat .lifeTotal]))])
theorem okSomeOpponentLacksMoreLifeThanYou :
    Condition.check (DRS.ofList []) someOpponentLacksMoreLifeThanYou = [] := by decide
def yourOpponentsHaveMoreLifeThanYou : Condition :=
  is yourOpponents
    (compare .greater (lifeTotalOf you) (axes := [.playerStat .lifeTotal]))
theorem okYourOpponentsHaveMoreLifeThanYou :
    Condition.check (DRS.ofList []) yourOpponentsHaveMoreLifeThanYou = [] := by decide

/-- Disarm {U} — Instant. "Unattach all Equipment from target creature." -/
def disarm : Spelled := spelled <| singleFacedCard
  { name := "Disarm", cost := [pip .blue], types := [.instant],
    text :=
      [ spell (unattach (allOf
          (and [equipment,
                 attachedTo (target isCreature)]))) ] }

def rawExile : Action := exile (target isCreature)
theorem okRawExile : Action.check (DRS.ofList []) rawExile = [] := by decide
def disenchant : Action := destroy (target (or [isArtifact, isEnchantment]))
theorem okDisenchant : Action.check (DRS.ofList []) disenchant = [] := by decide
def icyManipulator : Action := tap (target (or [isArtifact, isCreature, isLand]))
theorem okIcyManipulator : Action.check (DRS.ofList []) icyManipulator = [] := by decide
def divination : Action := draw 2
theorem okDivination : Action.check (DRS.ofList []) divination = [] := by decide
def ancestralRecall : Action := instruct (target isPlayer) (draw 3)
theorem okAncestralRecall : Action.check (DRS.ofList []) ancestralRecall = [] := by decide
def lifeTotalBecomesOne : Action := instruct (target isPlayer) (setLife 1)
theorem okLifeTotalBecomesOne : Action.check (DRS.ofList []) lifeTotalBecomesOne = [] := by decide

/-- Berserker's Frenzy's roll -/
def berserkersFrenzyRoll : Action :=
  sequentially [rollDice 2 (.sides 20), ignoreOutcomes (extreme .lowest)]
theorem okBerserkersFrenzyRoll : Action.check (DRS.ofList []) berserkersFrenzyRoll = [] := by decide
def ironMastiffIgnore : Action :=
  sequentially
    [ rollDice
        (countOf (and [isPlayer, inCombat .attackedBy (combatPlayer .attacking)]))
        (.sides 20) .anyResult,
      ignoreOutcomes (allBut .highest) ]
theorem okIronMastiffIgnore : Action.check (DRS.ofList []) ironMastiffIgnore = [] := by decide

/-- Xenosquirrels -/
def xenosquirrelsShift : Ability :=
  after (does you (rollDice 1 .anyDie)) (may
      (removeCounters (exactly 1) p1p1Counter thisCreature)
      (shiftResult 1))
theorem okXenosquirrelsShift : Ability.check (DRS.ofList []) xenosquirrelsShift = [] := by decide

theorem playersTopCardIsPlural : playersTopCardSlice.plur = .many := by decide

/-- Aetheric Amplifier -/
def doubleYourOwnCounters : Action := doubleCounters you
theorem okDoubleYourOwnCounters : Action.check (DRS.ofList []) doubleYourOwnCounters = [] := by decide

def permanentCardPhrase : Predicate := isPermanentCard
theorem permanentCardIsPlaceless : permanentCardPhrase.phraseZone .object = none := by decide

/-- Archpriest of Iona -/
def archpriestOfIonaPower : Ability := static (ptDefinition thisCreature .powerAlone partySize)
theorem okArchpriestOfIonaPower : Ability.check (DRS.ofList []) archpriestOfIonaPower = [] := by decide
def archpriestOfIonaFullParty : Ability :=
  at_ (beginningOfYour .combat) (if_ := fullParty)
    (establishFor (target isCreature)
      [getsPt it (up 1) (up 1), abilityGrant it flying]
      untilEndOfTurn)
theorem okArchpriestOfIonaFullParty : Ability.check (DRS.ofList []) archpriestOfIonaFullParty = [] := by decide

/-- Squad Commander -/
def squadCommanderTokens : Ability :=
  when (does thisCreature enter)
    (create partySize (creatureToken 1 1 [.white] [kor, warrior]))
theorem okSquadCommanderTokens : Ability.check (DRS.ofList []) squadCommanderTokens = [] := by decide

/-- At Knifepoint -/
def atKnifepointOutlaws : Ability :=
  static (partScope .turn you
            (abilityGrant (allOf (and [isOutlaw, youControl])) firstStrike))
def atKnifepointCrime : Ability :=
  whenever (does you commitCrime) (create 1 (written
      { characteristics :=
        { colors := [.red], types := [.creature], subtypes := [mercenary],
          text :=
            [ activatedOnlyDuring tapSymbol
                (get (target (and [isCreature, youControl])) (up 1) (up 0)
                  untilEndOfTurn)
                asSorcery ],
          power := stat 1, toughness := stat 1 } })) (limit := oncePerTurn)
def atKnifepoint : Spelled := spelled <| singleFacedCard
  { name := "At Knifepoint", cost := [generic 1, pip .black, pip .red],
    types := [.enchantment], text := [atKnifepointOutlaws, atKnifepointCrime] }

end Semantics.Cards
