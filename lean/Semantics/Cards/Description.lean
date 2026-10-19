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
  get (target (and [isBlocking, hasSubtype wall, youControl]))
    (up (.lit 10)) (up (.lit 0)) (some untilEndOfCombat)
theorem okGlyphOfDestruction : Action.check (DRS.ofList []) glyphOfDestruction = [] := by decide

def rawNonattacking : Predicate := and [isCreature, non isAttacking, non isBlocking]
theorem okRawNonattacking : Predicate.checkAt .object (DRS.ofList []) rawNonattacking = [] := by decide

/-- Harmony of Nature -/
def harmonyOfNature : Action :=
  sequentially
    [ tap (counted anyNumber (and [isUntapped, isCreature, youControl])),
      doForEach (theVerbed (.action "tap") .many (some creature) .thisWay)
        (gainLife (.lit 4)) ]
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
  destroy (target (and [isCreature, hasPower .atMost (.lit 2)]))
theorem okDefeat : Action.check (DRS.ofList []) defeat = [] := by decide

/-- Terashi's Verdict -/
def terashisVerdict : Action :=
  destroy (target (and [isCreature, isAttacking, hasPower .atMost (.lit 3)]))
theorem okTerashisVerdict : Action.check (DRS.ofList []) terashisVerdict = [] := by decide

/-- Pillar of Light -/
def pillarOfLight : Action :=
  exile (target (and [isCreature, hasToughness .atLeast (.lit 4)]))
theorem okPillarOfLight : Action.check (DRS.ofList []) pillarOfLight = [] := by decide

/-- Unholy Annex -/
def unholyAnnex : Action :=
  sequentially
    [ draw (.lit 1),
      doIf (exists_ (and [hasSubtype demon, youControl]))
        (sequentially [instruct eachOpponent (loseLife (.lit 2)), gainLife (.lit 2)])
        (some (loseLife (.lit 2))) ]
theorem okUnholyAnnex : Action.check (DRS.ofList []) unholyAnnex = [] := by decide

/-- War Screecher -/
def warScreecher : Action :=
  get (allOf (and [isCreature, youControl, otherThan thisCreature])) (up (.lit 1)) (up (.lit 1))
    (some untilEndOfTurn)
theorem okWarScreecher : Action.check (DRS.ofList []) warScreecher = [] := by decide

/-- Mind Flayer -/
def mindFlayer : Action :=
  gainControl (target isCreature)
    (some (forAsLongAs (matches_ thisCreature youControl))) (controller := you)
theorem okMindFlayer : Action.check (DRS.ofList []) mindFlayer = [] := by decide

/-- Arc Lightning -/
def arcLightning : Action :=
  dealDivided this_ (.lit 3) (targets (oneThrough 3) isAnyTarget)
theorem okArcLightning : Action.check (DRS.ofList []) arcLightning = [] := by decide

/-- Boulderfall -/
def boulderfall : Action :=
  dealDivided this_ (.lit 5) (targets anyNumber isAnyTarget)
theorem okBoulderfall : Action.check (DRS.ofList []) boulderfall = [] := by decide

/-- Banisher Priest -/
def banisherPriest : Action :=
  exileUntil (target (and [isCreature, anOpponentControls]))
    (does thisCreature leave)
theorem okBanisherPriest : Action.check (DRS.ofList []) banisherPriest = [] := by decide

/-- Tezzeret, Artifice Master -/
def tezzeretDrawTwo : Action :=
  replace (draw (.lit 1))
    (doIf (compareAmt (countOf (and [isArtifact, youControl]))
            .atLeast (.lit 3))
      (instead (draw (.lit 2))))
theorem okTezzeretDrawTwo : Action.check (DRS.ofList []) tezzeretDrawTwo = [] := by decide

/-- Zimone, Quandrix Prodigy -/
def zimoneDrawTwo : Action :=
  replace (draw (.lit 1))
    (doIf (compareAmt (countOf (and [isLand, youControl])) .atLeast (.lit 8))
      (instead (draw (.lit 2))))
theorem okZimoneDrawTwo : Action.check (DRS.ofList []) zimoneDrawTwo = [] := by decide

/-- Aerial Volley {G} — Instant. "Aerial Volley deals 3 damage divided as you choose among
one, two, or three target creatures with flying." -/
def aerialVolley : Spelled := spelled <| .singleFaced
  { characteristics :=
    { name := "Aerial Volley", cost := some [pip .green], types := [.instant],
      text :=
        [ spell (dealDivided this_ (.lit 3)
            (targets (oneThrough 3) (and [isCreature, hasKeyword flying]))) ] } }

/-- Terrifying Presence -/
def terrifyingPresenceAnchor : Predicate := and [isCreature, otherThan (target isCreature)]
theorem okTerrifyingPresenceAnchor : Predicate.checkAt .object (DRS.ofList []) terrifyingPresenceAnchor = [] := by
  decide

/-- Timely Reinforcements -/
def timelyReinforcements : Spelled := spelled <| .singleFaced
  { characteristics :=
    { name := "Timely Reinforcements", cost := some [generic 2, pip .white], types := [.sorcery],
      text :=
        [ spell (sequentially
            [ doIf (compareAmt (lifeTotalOf you) .less (lifeTotalOf anOpponent))
                (gainLife (.lit 6)),
              doIf (compareAmt (countOf (and [isCreature, youControl])) .less
                      (countOf (and [isCreature, anOpponentControls])))
                (createToken (creatureToken 1 1 [.white] [soldier]) (count := .lit 3)) ]) ] } }

/-- Survival Cache -/
def survivalCache : Action :=
  sequentially
    [ gainLife (.lit 2),
      doIf (compareAmt (lifeTotalOf you) .greater (lifeTotalOf anOpponent))
        (draw (.lit 1)) ]
theorem okSurvivalCache : Action.check (DRS.ofList []) survivalCache = [] := by decide

/-- Nightmarish End -/
def nightmarishEnd : Spelled := spelled <| .singleFaced
  { characteristics :=
    { name := "Nightmarish End", cost := some [generic 2, pip .black], types := [.instant],
      text :=
        [ spell (sequentially
            [ get (target isCreature) (down (letter .x)) (down (letter .x))
                (some untilEndOfTurn),
              define .x (countOf (cardIn yourHand)) ]) ] } }

/-- Topple {2}{W} — Sorcery. "Exile target creature with the greatest power among creatures
on the battlefield." -/
def topple : Spelled := spelled <| .singleFaced
  { characteristics :=
    { name := "Topple", cost := some [generic 2, pip .white], types := [.sorcery],
      text :=
        [ spell (exile (target
            (and [isCreature,
                   superlative .max (.stat .power) (and [isCreature, isPermanent])]))) ] } }

/-- "each opponent with three or more poison counters" -/
def corruptedOpponents : NounPhrase :=
  each (and [isOpponent, compare .atLeast (.lit 3) (axes := [.counter poison])])
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
  create (.lit 1)
    (copyOf (target (and [isCreature, non (hasSubtype frog)]))
      (copyCharacteristics {
                 colors := [.green], subtypes := [frog], power := stat 1,
                 toughness := stat 1 } false))
    []
theorem okCroakingCounterpartCopy : Action.check (DRS.ofList []) croakingCounterpartCopy = [] := by decide

/-- Blessed Reversal -/
def blessedReversal : Spelled := spelled <| .singleFaced
  { characteristics :=
    { name := "Blessed Reversal", cost := some [generic 1, pip .white], types := [.instant],
      text :=
        [ spell (gainLife
            (forEach (.lit 3) (and [isCreature, inCombat .attackerOf (some you)]))) ] } }

/-- Extinction -/
def extinction : Spelled := spelled <| .singleFaced
  { characteristics :=
    { name := "Extinction", cost := some [generic 4, pip .black], types := [.sorcery],
      text :=
        [ spell (destroy (allOf (and [isCreature, ofYourChoice (.subtype .creature)]))) ] } }

/-- Defensive Maneuvers -/
def defensiveManeuvers : Spelled := spelled <| .singleFaced
  { characteristics :=
    { name := "Defensive Maneuvers", cost := some [generic 3, pip .white], types := [.instant],
      text :=
        [ spell (get (allOf (and [isCreature, ofYourChoice (.subtype .creature)]))
            (up (.lit 0)) (up (.lit 4)) (some untilEndOfTurn)) ] } }

/-- Witch's Vengeance -/
def witchsVengeance : Spelled := spelled <| .singleFaced
  { characteristics :=
    { name := "Witch's Vengeance", cost := some [generic 1, pip .black, pip .black],
      types := [.sorcery],
      text :=
        [ spell (get (allOf (and [isCreature, ofYourChoice (.subtype .creature)]))
            (down (.lit 3)) (down (.lit 3)) (some untilEndOfTurn)) ] } }

/-- Phyrexian Rebirth -/
def phyrexianRebirth : Spelled := spelled <| .singleFaced
  { characteristics :=
    { name := "Phyrexian Rebirth", cost := some [generic 4, pip .white, pip .white],
      types := [.sorcery],
      text :=
        [ spell (sequentially
            [ destroy (allOf isCreature),
              createToken { characteristics :=
                    { types := [.artifact, .creature],
                      subtypes := [phyrexian, horror],
                      power := some (letter .x), toughness := some (letter .x) } },
              define .x
                  (numberOf (theVerbed (.action "destroy") .many (some creature) .thisWay)) ]) ] } }

/-- Damning Verdict -/
def damningVerdict : Spelled := spelled <| .singleFaced
  { characteristics :=
    { name := "Damning Verdict", cost := some [generic 3, pip .white, pip .white],
      types := [.sorcery],
      text := [ spell (destroy (allOf (and [isCreature, not hasCounter]))) ] } }

/-- Hazardous Conditions -/
def hazardousConditions : Spelled := spelled <| .singleFaced
  { characteristics :=
    { name := "Hazardous Conditions", cost := some [generic 2, pip .black, pip .green],
      types := [.sorcery],
      text :=
        [ spell (get (allOf (and [isCreature, not hasCounter]))
            (down (.lit 2)) (down (.lit 2)) (some untilEndOfTurn)) ] } }

/-- Approach of the Second Sun -/
def approachOfTheSecondSun : Spelled := spelled <| .singleFaced
  { characteristics :=
    { name := "Approach of the Second Sun", cost := some [generic 6, pip .white],
      types := [.sorcery],
      text :=
        [ spell (doIf
            (and
              [ matches_ this_ (castFrom yourHand),
                happened (cast (a
                  (and [ isSpell, otherThan
                  this_,
                             named (printedName
                               "Approach of the Second Sun") ]))) you thisGame ])
            winGame
            (some (sequentially
              [ putInto this_ (fromTop 7 library), gainLife (.lit 7) ]))) ] } }

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
    [ rollDice (.lit 2) (.sides 6),
      doIf (compareAmt (theOutcome .rollResult) .eq (.lit 7)) (sacrifice thisCreature) ]
theorem okSparkFiendUpkeepRoll : Action.check (DRS.ofList []) sparkFiendUpkeepRoll = [] := by decide

/-- Erebos, God of the Dead -/
def devotionCondition : Condition := compareAmt (devotion you (color .black)) .less (.lit 5)
theorem okDevotionCondition : Condition.check (DRS.ofList []) devotionCondition = [] := by decide

/-- Multiple Choice -/
def multipleChoiceFirstArm : Action :=
  doIf (compareAmt (letter .x) .eq (.lit 1))
    (sequentially [scry (.lit 1), draw (.lit 1)])
theorem okMultipleChoiceFirstArm : Action.check (DRS.ofList []) multipleChoiceFirstArm = [] := by decide

/-- Multiple Choice, fourth arm -/
def multipleChoiceFourthGate : Condition := compareAmt (letter .x) .atLeast (.lit 4)
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
  spell (doIf (costWasPaid .theAlternative none this_) (instruct anOpponent (draw (.lit 1))))
theorem okBalefulMasteryPaidRead : Ability.check (DRS.ofList []) balefulMasteryPaidRead = [] := by decide

/-- Karai, Future of the Foot -/
def karaiSneakPaidThisTurn : Amount := paidCostRead (.byKeyword sneak) (some thisTurn) this_
theorem okKaraiSneakPaidThisTurn : Amount.check (DRS.ofList []) karaiSneakPaidThisTurn = [] := by decide

/-- Requiting Hex's read -/
def requitingHexAdditionalRead : Ability :=
  spell (doIf (costWasPaid .theAdditional none this_) (gainLife (.lit 2)))
theorem okRequitingHexAdditionalRead : Ability.check (DRS.ofList []) requitingHexAdditionalRead = [] := by
  decide

/-- Lucid Dreams -/
def lucidDreams : Spelled := spelled <| .singleFaced
  { characteristics :=
    { name := "Lucid Dreams", cost := some [generic 3, pip .blue, pip .blue], types := [.sorcery],
      text :=
        [ spell (sequentially
            [ draw (letter .x),
              define .x (distinctCount .cardType (allOf (cardIn yourGraveyard))) ]) ] } }

/-- Rogues' Gallery -/
def roguesGallery : Spelled := spelled <| .singleFaced
  { characteristics :=
    { name := "Rogues' Gallery", cost := some [generic 2, pip .black], types := [.sorcery],
      text :=
        [ spell
            (doForEachKind .color .color
               (putInto (targets (upTo 1) (and [isCreature, ofChosen .color, in_ yourGraveyard]))
                  hand)) ] } }

/-- Bioplasm -/
def bioplasmAttack : GameEvent := does thisCreature attack
theorem okBioplasmAttack : GameEvent.check (DRS.ofList []) bioplasmAttack = [] := by decide
def bioplasmExile : Action := exile (topSlice (.lit 1))
theorem okBioplasmExile :
    Action.check ((stepGameEvent 512 (DRS.ofList []) bioplasmAttack).1) bioplasmExile = [] := by decide
def bioplasmAfterExile : DRS := (stepAction 512 .postEvent .obj ((stepGameEvent 512 (DRS.ofList []) bioplasmAttack).1) bioplasmExile).1
def bioplasmExiledCard : NounPhrase := theVerbed (.action "exile") .one (some .card)
theorem okBioplasmExiledCard :
    NounPhrase.checkAt (some .object) bioplasmAfterExile bioplasmExiledCard = [] := by decide

/-- "the top card of each player's library" -/
def playersTopCardSlice : NounPhrase := slice .top (.lit 1) (libraryOf eachPlayer)
theorem okPlayersTopCardSlice : NounPhrase.checkAt (some .object) (DRS.ofList []) playersTopCardSlice = [] := by
  decide

/-- Deepglow Skate's recipient -/
def deepglowSkateRecipient : NounPhrase := targets anyNumber isPermanent
theorem deepglowSkateRecipientRefused : deepglowSkateRecipient.perMemberOk = false := by decide

/-- Weftwalking -/
def weftwalkingShuffle : Action :=
  sequentially
    [ shuffleInto (both (allOf (cardIn yourHand)) (allOf (cardIn yourGraveyard))),
      draw (.lit 7) ]
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
    [isPlayer, happenedTo which loseGame thisGame])) .atLeast (.lit 2)
theorem okTwoOrMorePlayersHaveLost : Condition.check (DRS.ofList []) twoOrMorePlayersHaveLost = [] := by decide

def commanderCreaturesYouOwn : Predicate :=
  and [isCreature, hasDesignation commander, youOwn]
theorem okCommanderCreaturesYouOwn : Predicate.checkAt .object (DRS.ofList []) commanderCreaturesYouOwn = [] := by
  decide

/-- Idol of Endurance -/
def creatureSpellFromAmongExiled : NounPhrase :=
  fromAmong (exactly 1) (and [isCreature, isSpell]) (allOf (and [isCard, exiledWithThisArtifact]))
theorem okCreatureSpellFromAmongExiled :
    NounPhrase.checkAt (some .object) (DRS.ofList []) creatureSpellFromAmongExiled = [] := by decide

def ringHasTemptedYouTwiceThisGame : Condition :=
  compareAmt (eventCount theRingTemptsYou you thisGame)
    .atLeast (.lit 2)
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
  matches_ yourOpponents
    (compare .greater (lifeTotalOf you) (axes := [.playerStat .lifeTotal]))
theorem okYourOpponentsHaveMoreLifeThanYou :
    Condition.check (DRS.ofList []) yourOpponentsHaveMoreLifeThanYou = [] := by decide

/-- Disarm {U} — Instant. "Unattach all Equipment from target creature." -/
def disarm : Spelled := spelled <| .singleFaced
  { characteristics :=
    { name := "Disarm", cost := some [pip .blue], types := [.instant],
      text :=
        [ spell (unattach (allOf
            (and [hasSubtype equipment,
                   attachedTo (target isCreature)]))) ] } }

def rawExile : Action := exile (target isCreature)
theorem okRawExile : Action.check (DRS.ofList []) rawExile = [] := by decide
def disenchant : Action := destroy (target (or [isArtifact, isEnchantment]))
theorem okDisenchant : Action.check (DRS.ofList []) disenchant = [] := by decide
def icyManipulator : Action := tap (target (or [isArtifact, isCreature, isLand]))
theorem okIcyManipulator : Action.check (DRS.ofList []) icyManipulator = [] := by decide
def divination : Action := draw (.lit 2)
theorem okDivination : Action.check (DRS.ofList []) divination = [] := by decide
def ancestralRecall : Action := instruct (target isPlayer) (draw (.lit 3))
theorem okAncestralRecall : Action.check (DRS.ofList []) ancestralRecall = [] := by decide
def lifeTotalBecomesOne : Action := instruct (target isPlayer) (setLife (.lit 1))
theorem okLifeTotalBecomesOne : Action.check (DRS.ofList []) lifeTotalBecomesOne = [] := by decide

/-- Berserker's Frenzy's roll -/
def berserkersFrenzyRoll : Action :=
  sequentially [rollDice (.lit 2) (.sides 20), ignoreOutcomes (extreme .lowest)]
theorem okBerserkersFrenzyRoll : Action.check (DRS.ofList []) berserkersFrenzyRoll = [] := by decide
def ironMastiffIgnore : Action :=
  sequentially
    [ rollDice
        (countOf (and [isPlayer, inCombat .attackedBy (some (combatPlayer .attacking))]))
        (.sides 20) .many .anyResult,
      ignoreOutcomes (allBut .highest) ]
theorem okIronMastiffIgnore : Action.check (DRS.ofList []) ironMastiffIgnore = [] := by decide

/-- Xenosquirrels -/
def xenosquirrelsShift : Ability :=
  triggered (does you (rollDice anyAmount .anyDie)) (may
      (removeCounters (exactly 1) (some (printedKind p1p1Counter)) thisCreature .many true)
      (some (shiftResult (.lit 1))))
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
  triggeredIf (beginningOfYour .combat) fullParty
    (sequentially
      [ get (target isCreature) (up (.lit 1)) (up (.lit 1)) (some untilEndOfTurn),
        gain it flying (some untilEndOfTurn) ])
theorem okArchpriestOfIonaFullParty : Ability.check (DRS.ofList []) archpriestOfIonaFullParty = [] := by decide

/-- Squad Commander -/
def squadCommanderTokens : Ability :=
  when (does thisCreature enter)
    (createToken (creatureToken 1 1 [.white] [kor, warrior]) (count := partySize))
theorem okSquadCommanderTokens : Ability.check (DRS.ofList []) squadCommanderTokens = [] := by decide

/-- At Knifepoint -/
def atKnifepointOutlaws : Ability :=
  static (partScope .turn (some you)
            (abilityGrant (allOf (and [isOutlaw, youControl])) firstStrike))
def atKnifepointCrime : Ability :=
  triggeredOnlyOnce (does you commitCrime) oncePerTurn (createToken
      { characteristics :=
        { colors := [.red], types := [.creature], subtypes := [mercenary],
          text :=
            [ activatedOnlyDuring tapSymbol
                (get (target (and [isCreature, youControl])) (up (.lit 1)) (up (.lit 0))
                  (some untilEndOfTurn))
                asSorcery ],
          power := stat 1, toughness := stat 1 } })
def atKnifepoint : Spelled := spelled <| .singleFaced
  { characteristics :=
    { name := "At Knifepoint", cost := some [generic 1, pip .black, pip .red],
      types := [.enchantment], text := [atKnifepointOutlaws, atKnifepointCrime] } }

end Semantics.Cards
