import Semantics
import Semantics.Macros
import Semantics.Check.Card

/-!
# Semantics.Cards.Description

Port of `idris/src/Experimental/Cards/Description.idr`: the printed cards of the Description
family, and the phrase-level bench items beside them (a noun, predicate, condition,
instruction, or ability that elaborated in Idris), each paired with an `ok…` theorem that runs
the checker on it at the bindings its Idris type named.
-/

open Semantics Semantics.Macros

namespace Semantics.Cards

/-- Glyph of Destruction -/
def glyphOfDestruction : Instruction :=
  gets (target (.and [blocking, .hasSubtype (creatureType "Wall"), .hasPossessor .controller .you]))
    (.up (.lit 10)) (.up (.lit 0)) (some untilEndOfCombat)
theorem okGlyphOfDestruction : Instruction.check [] glyphOfDestruction = [] := by decide

def rawNonattacking : Predicate := .and [creature, .not attacking, .not blocking]
theorem okRawNonattacking : Predicate.check .object [] rawNonattacking = [] := by decide

/-- Harmony of Nature -/
def harmonyOfNature : Instruction :=
  .sequentially
    [ tap (counted anyNumber (.and [untapped, creature, .hasPossessor .controller .you])),
      .forEachOf (theVerbed (.action "Tap") (.type .creature) .thisWay .many)
        (gainsLife .you (.lit 4)) ]
theorem okHarmonyOfNature : Instruction.check [] harmonyOfNature = [] := by decide

/-- Rats of Rath -/
def ratsOfRath : Instruction :=
  destroy (target (.and [.or [artifact, creature, land], .hasPossessor .controller .you]))
theorem okRatsOfRath : Instruction.check [] ratsOfRath = [] := by decide

/-- "another creature or land", after a target on the battlefield. -/
def anotherDisjunctPhrase : Predicate := .and [.or [creature, land], .other]
theorem okAnotherDisjunctPhrase :
    Predicate.check .object [⟨.target, .one, .object none (some .battlefield) none none none⟩]
      anotherDisjunctPhrase = [] := by
  decide

/-- Defeat -/
def defeat : Instruction :=
  destroy (target (.and [creature, .compare [.stat .power] .atMost (.lit 2)]))
theorem okDefeat : Instruction.check [] defeat = [] := by decide

/-- Terashi's Verdict -/
def terashisVerdict : Instruction :=
  destroy (target (.and [creature, attacking, .compare [.stat .power] .atMost (.lit 3)]))
theorem okTerashisVerdict : Instruction.check [] terashisVerdict = [] := by decide

/-- Pillar of Light -/
def pillarOfLight : Instruction :=
  exile (target (.and [creature, .compare [.stat .toughness] .atLeast (.lit 4)]))
theorem okPillarOfLight : Instruction.check [] pillarOfLight = [] := by decide

/-- Unholy Annex -/
def unholyAnnex : Instruction :=
  .sequentially
    [ .draw .you (.lit 1),
      .if_ (exists_ (.and [.hasSubtype (creatureType "Demon"), .hasPossessor .controller .you]))
        (.sequentially [losesLife (each .opponent) (.lit 2), gainsLife .you (.lit 2)])
        (some (losesLife .you (.lit 2))) ]
theorem okUnholyAnnex : Instruction.check [] unholyAnnex = [] := by decide

/-- War Screecher -/
def warScreecher : Instruction :=
  gets (allOf (otherCreatureYouControl thisCreature)) (.up (.lit 1)) (.up (.lit 1))
    (some untilEndOfTurn)
theorem okWarScreecher : Instruction.check [] warScreecher = [] := by decide

/-- Mind Flayer -/
def mindFlayer : Instruction :=
  gainControl .you (target creature)
    (some (.forAsLongAs (.matches thisCreature (.hasPossessor .controller .you))))
theorem okMindFlayer : Instruction.check [] mindFlayer = [] := by decide

/-- Arc Lightning -/
def arcLightning : Instruction :=
  dealsDivided .this (.lit 3) (.described (.target (oneThrough 3)) anyTarget)
theorem okArcLightning : Instruction.check [] arcLightning = [] := by decide

/-- Boulderfall -/
def boulderfall : Instruction :=
  dealsDivided .this (.lit 5) (.described (.target anyNumber) anyTarget)
theorem okBoulderfall : Instruction.check [] boulderfall = [] := by decide

/-- Banisher Priest -/
def banisherPriest : Instruction :=
  exileUntil (target (.and [creature, .hasPossessor .controller anOpponent]))
    (leavesBattlefield thisCreature)
theorem okBanisherPriest : Instruction.check [] banisherPriest = [] := by decide

/-- Tezzeret, Artifice Master -/
def tezzeretDrawTwo : Instruction :=
  .insteadOf (.draw .you (.lit 1))
    (.if_ (.compareAmt (countOf (.and [artifact, .hasPossessor .controller .you]))
            .atLeast (.lit 3))
      (.draw .you (.lit 2)) none)
theorem okTezzeretDrawTwo : Instruction.check [] tezzeretDrawTwo = [] := by decide

/-- Zimone, Quandrix Prodigy -/
def zimoneDrawTwo : Instruction :=
  .insteadOf (.draw .you (.lit 1))
    (.if_ (.compareAmt (countOf (.and [land, .hasPossessor .controller .you])) .atLeast (.lit 8))
      (.draw .you (.lit 2)) none)
theorem okZimoneDrawTwo : Instruction.check [] zimoneDrawTwo = [] := by decide

/-- Aerial Volley {G} — Instant. "Aerial Volley deals 3 damage divided as you choose among
one, two, or three target creatures with flying." -/
def aerialVolley : Spelled := spelled <| .singleFaced
  { characteristics :=
    { name := "Aerial Volley", cost := some [pip .green], types := [.instant],
      text :=
        [ .spell none (dealsDivided .this (.lit 3)
            (.described (.target (oneThrough 3))
              (.and [creature, .hasKeyword (.the "Flying")]))) ] } }

/-- Terrifying Presence -/
def terrifyingPresenceAnchor : Predicate := .and [creature, .otherThan (target creature)]
theorem okTerrifyingPresenceAnchor : Predicate.check .object [] terrifyingPresenceAnchor = [] := by
  decide

/-- Timely Reinforcements -/
def timelyReinforcements : Spelled := spelled <| .singleFaced
  { characteristics :=
    { name := "Timely Reinforcements", cost := some [generic 2, pip .white], types := [.sorcery],
      text :=
        [ .spell none (.sequentially
            [ .if_ (.compareAmt (lifeTotalOf .you) .less (lifeTotalOf anOpponent))
                (gainsLife .you (.lit 6)) none,
              .if_ (.compareAmt (countOf creatureYouControl) .less
                      (countOf (.and [creature, .hasPossessor .controller anOpponent])))
                (create (.lit 3) (creatureToken 1 1 [.white] [creatureType "Soldier"])) none ]) ] } }

/-- Survival Cache -/
def survivalCache : Instruction :=
  .sequentially
    [ gainsLife .you (.lit 2),
      .if_ (.compareAmt (lifeTotalOf .you) .greater (lifeTotalOf anOpponent))
        (.draw .you (.lit 1)) none ]
theorem okSurvivalCache : Instruction.check [] survivalCache = [] := by decide

/-- Nightmarish End -/
def nightmarishEnd : Spelled := spelled <| .singleFaced
  { characteristics :=
    { name := "Nightmarish End", cost := some [generic 2, pip .black], types := [.instant],
      text :=
        [ .spell none (.sequentially
            [ gets (target creature) (.down (.letter .x)) (.down (.letter .x))
                (some untilEndOfTurn),
              .define .x (countOf (.inZone (handOf .you))) ]) ] } }

/-- Topple {2}{W} — Sorcery. "Exile target creature with the greatest power among creatures
on the battlefield." -/
def topple : Spelled := spelled <| .singleFaced
  { characteristics :=
    { name := "Topple", cost := some [generic 2, pip .white], types := [.sorcery],
      text :=
        [ .spell none (exile (target
            (.and [creature,
                   .superlative .max (.stat .power) (.and [creature, permanent])]))) ] } }

/-- "each opponent with three or more poison counters" -/
def corruptedOpponents : NounPhrase :=
  each (.and [.opponent, .compare [.counter (.named "Poison")] .atLeast (.lit 3)])
theorem okCorruptedOpponents : NounPhrase.check (some .player) [] corruptedOpponents = [] := by
  decide

/-- War Tax -/
def warTaxScaledPayment : Instruction :=
  .continuously
    (deontic (allOf creature)
      (.gatedBy (scaledMana .generic
        (.arith .times (.letter .x)
          (countOf (.and [creature, attacking, .hasPossessor .controller they])))))
      [.core .attack] .agent .noPatient)
    (some .thisTurn)
theorem okWarTaxScaledPayment : Instruction.check [letterB .x] warTaxScaledPayment = [] := by
  decide

/-- Croaking Counterpart -/
def croakingCounterpartCopy : Instruction :=
  .create .you (.lit 1)
    (.copyOf (target (.and [creature, .not (.hasSubtype (creatureType "Frog"))]))
      [ .chars { colors := [.green], subtypes := [creatureType "Frog"], power := stat 1,
                 toughness := stat 1 } false ])
    []
theorem okCroakingCounterpartCopy : Instruction.check [] croakingCounterpartCopy = [] := by decide

/-- Blessed Reversal -/
def blessedReversal : Spelled := spelled <| .singleFaced
  { characteristics :=
    { name := "Blessed Reversal", cost := some [generic 1, pip .white], types := [.instant],
      text :=
        [ .spell none (gainsLife .you
            (times (.lit 3) (countOf (.and [creature, .inCombat .attackerOf (some .you)])))) ] } }

/-- Extinction -/
def extinction : Spelled := spelled <| .singleFaced
  { characteristics :=
    { name := "Extinction", cost := some [generic 4, pip .black], types := [.sorcery],
      text :=
        [ .spell none (destroy (allOf (.and [creature, .ofYourChoice (.subtype .creature) none]))) ] } }

/-- Defensive Maneuvers -/
def defensiveManeuvers : Spelled := spelled <| .singleFaced
  { characteristics :=
    { name := "Defensive Maneuvers", cost := some [generic 3, pip .white], types := [.instant],
      text :=
        [ .spell none (gets (allOf (.and [creature, .ofYourChoice (.subtype .creature) none]))
            (.up (.lit 0)) (.up (.lit 4)) (some untilEndOfTurn)) ] } }

/-- Witch's Vengeance -/
def witchsVengeance : Spelled := spelled <| .singleFaced
  { characteristics :=
    { name := "Witch's Vengeance", cost := some [generic 1, pip .black, pip .black],
      types := [.sorcery],
      text :=
        [ .spell none (gets (allOf (.and [creature, .ofYourChoice (.subtype .creature) none]))
            (.down (.lit 3)) (.down (.lit 3)) (some untilEndOfTurn)) ] } }

/-- Phyrexian Rebirth -/
def phyrexianRebirth : Spelled := spelled <| .singleFaced
  { characteristics :=
    { name := "Phyrexian Rebirth", cost := some [generic 4, pip .white, pip .white],
      types := [.sorcery],
      text :=
        [ .spell none (.sequentially
            [ destroy (allOf creature),
              .create .you (.lit 1)
                (.written
                  { characteristics :=
                    { types := [.artifact, .creature],
                      subtypes := [creatureType "Phyrexian", creatureType "Horror"],
                      power := some (.letter .x), toughness := some (.letter .x) } })
                [],
              .define .x (.countOf (theVerbed (.action "Destroy") (.type .creature) .thisWay .many)) ]) ] } }

/-- Damning Verdict -/
def damningVerdict : Spelled := spelled <| .singleFaced
  { characteristics :=
    { name := "Damning Verdict", cost := some [generic 3, pip .white, pip .white],
      types := [.sorcery],
      text := [ .spell none (destroy (allOf (.and [creature, .not (.hasCounters none)]))) ] } }

/-- Hazardous Conditions -/
def hazardousConditions : Spelled := spelled <| .singleFaced
  { characteristics :=
    { name := "Hazardous Conditions", cost := some [generic 2, pip .black, pip .green],
      types := [.sorcery],
      text :=
        [ .spell none (gets (allOf (.and [creature, .not (.hasCounters none)]))
            (.down (.lit 2)) (.down (.lit 2)) (some untilEndOfTurn)) ] } }

/-- Approach of the Second Sun -/
def approachOfTheSecondSun : Spelled := spelled <| .singleFaced
  { characteristics :=
    { name := "Approach of the Second Sun", cost := some [generic 6, pip .white],
      types := [.sorcery],
      text :=
        [ .spell none (.if_
            (.and
              [ .matches .this (.castFrom (handOf .you)),
                happenedInvolving .spellCast .you .thisGame
                  (a (.and [ spell, .otherThan .this,
                             .named (.printed "Approach of the Second Sun") ])) ])
            (.concludes .winGame .you)
            (some (.sequentially
              [ move .this (nthFromTop (.nth 7)), gainsLife .you (.lit 7) ]))) ] } }

/-- "a loyalty ability of enchanted planeswalker" -/
def loyaltyAbilityOfEnchanted : NounPhrase :=
  a (.and [.abilityHead .loyalty, .abilityOf (.attachHost .enchanted (.type .planeswalker))])
theorem okLoyaltyAbilityOfEnchanted :
    NounPhrase.check (some .object) [] loyaltyAbilityOfEnchanted = [] := by
  decide

/-- Balance of Power -/
def balanceOfPower : Instruction :=
  .if_ (.compareAmt (countOf (.inZone (handOf (target .opponent)))) .greater
          (countOf (.inZone (handOf .you))))
    (.draw .you .theDifference) none
theorem okBalanceOfPower : Instruction.check [] balanceOfPower = [] := by decide

/-- Spark Fiend -/
def sparkFiendUpkeepRoll : Instruction :=
  .sequentially
    [ rollDice .you 2 6,
      if_ (.compareAmt (.theOutcome .rollResult) .eq (.lit 7)) (sacrifice .you thisCreature) ]
theorem okSparkFiendUpkeepRoll : Instruction.check [] sparkFiendUpkeepRoll = [] := by decide

/-- Erebos, God of the Dead -/
def devotionCondition : Condition := .compareAmt (.devotion .you (.lit .black) none) .less (.lit 5)
theorem okDevotionCondition : Condition.check [] devotionCondition = [] := by decide

/-- Multiple Choice -/
def multipleChoiceFirstArm : Instruction :=
  .if_ (.compareAmt (.letter .x) .eq (.lit 1))
    (.sequentially [scry .you (.lit 1), .draw .you (.lit 1)]) none
theorem okMultipleChoiceFirstArm : Instruction.check [] multipleChoiceFirstArm = [] := by decide

/-- Multiple Choice, fourth arm -/
def multipleChoiceFourthGate : Condition := .compareAmt (.letter .x) .atLeast (.lit 4)
theorem okMultipleChoiceFourthGate : Condition.check [] multipleChoiceFourthGate = [] := by decide

/-- Fell the Mighty -/
def fellTheMighty : Instruction :=
  destroy
    (allOf (.and [creature,
                  .compare [.stat .power] .greater (.statOf (.stat .power) (target creature))]))
theorem okFellTheMighty : Instruction.check [] fellTheMighty = [] := by decide

def targetPlayerOrPlaneswalker : NounPhrase := target (.or [.hasType .planeswalker, .anyPlayer])
theorem okTargetPlayerOrPlaneswalker :
    NounPhrase.check none [] targetPlayerOrPlaneswalker = [] := by decide

def targetOpponentOrPlaneswalker : NounPhrase := target (.or [.hasType .planeswalker, .opponent])
theorem okTargetOpponentOrPlaneswalker :
    NounPhrase.check none [] targetOpponentOrPlaneswalker = [] := by decide

/-- Baleful Mastery's paid read -/
def balefulMasteryPaidRead : Ability :=
  .spell none (.if_ (costWasPaid .theAlternative none .this) (.draw (a .opponent) (.lit 1)) none)
theorem okBalefulMasteryPaidRead : Ability.check [] balefulMasteryPaidRead = [] := by decide

/-- Karai, Future of the Foot -/
def karaiSneakPaidThisTurn : Amount := paidCostRead (.byKeyword "Sneak") (some .thisTurn) .this
theorem okKaraiSneakPaidThisTurn : Amount.check [] karaiSneakPaidThisTurn = [] := by decide

/-- Requiting Hex's read -/
def requitingHexAdditionalRead : Ability :=
  .spell none (.if_ (costWasPaid .theAdditional none .this) (gainsLife .you (.lit 2)) none)
theorem okRequitingHexAdditionalRead : Ability.check [] requitingHexAdditionalRead = [] := by
  decide

/-- Lucid Dreams -/
def lucidDreams : Spelled := spelled <| .singleFaced
  { characteristics :=
    { name := "Lucid Dreams", cost := some [generic 3, pip .blue, pip .blue], types := [.sorcery],
      text :=
        [ .spell none (.sequentially
            [ .draw .you (.letter .x),
              .define .x (.distinctCount .cardType (allOf (.inZone (graveyardOf .you)))) ]) ] } }

/-- Rogues' Gallery -/
def roguesGallery : Spelled := spelled <| .singleFaced
  { characteristics :=
    { name := "Rogues' Gallery", cost := some [generic 2, pip .black], types := [.sorcery],
      text :=
        [ .spell none (.forEachKindOf .color none .color
            (move (.described (.target (upTo 1))
                     (.and [creature, ofChosen .color, .inZone (graveyardOf .you)]))
                  hand)) ] } }

/-- Bioplasm -/
def bioplasmAttack : GameEvent := .combat .attackerOf thisCreature none
theorem okBioplasmAttack : GameEvent.check [] bioplasmAttack = [] := by decide
def bioplasmExile : Instruction := exile (topSlice (.lit 1))
theorem okBioplasmExile :
    Instruction.check (GameEvent.after [] bioplasmAttack) bioplasmExile = [] := by decide
def bioplasmAfterExile : Bindings := Instruction.intro (GameEvent.after [] bioplasmAttack) bioplasmExile
def bioplasmExiledCard : NounPhrase := theVerbed (.action "Exile") .card .attributive .one
theorem okBioplasmExiledCard :
    NounPhrase.check (some .object) bioplasmAfterExile bioplasmExiledCard = [] := by decide

/-- "the top card of each player's library" -/
def playersTopCardSlice : NounPhrase := .librarySlice .top (.lit 1) (.playerGroup .allPlayers)
theorem okPlayersTopCardSlice : NounPhrase.check (some .object) [] playersTopCardSlice = [] := by
  decide

/-- Deepglow Skate's recipient -/
def deepglowSkateRecipient : NounPhrase := .described (.target anyNumber) permanent
theorem deepglowSkateRecipientRefused : deepglowSkateRecipient.perMemberOk = false := by decide

/-- Weftwalking -/
def weftwalkingShuffle : Instruction :=
  .sequentially
    [ shuffleInto .you (.both (allOf (.inZone (handOf .you))) (allOf (.inZone (graveyardOf .you)))),
      .draw .you (.lit 7) ]
theorem okWeftwalkingShuffle : Instruction.check [] weftwalkingShuffle = [] := by decide

/-- Gaea's Revenge's protection-shaped phrase -/
def nongreenSpellsOrAbilities : Predicate :=
  .or [ .and [spell, .not (.colorIs .green)],
        .and [ .abilityHead .anyOnStack,
               .abilityOf (a (.and [source, .not (.colorIs .green)])) ] ]
theorem okNongreenSpellsOrAbilities :
    Predicate.check .object [] nongreenSpellsOrAbilities = [] := by decide

/-- Hot Pursuit -/
def twoOrMorePlayersHaveLost : Condition :=
  .compareAmt (countOf (.and [.anyPlayer, happenedTo .gameLoss .thisGame])) .atLeast (.lit 2)
theorem okTwoOrMorePlayersHaveLost : Condition.check [] twoOrMorePlayersHaveLost = [] := by decide

def commanderCreaturesYouOwn : Predicate :=
  .and [creature, .hasDesignation "commander" none, .hasPossessor .owner .you]
theorem okCommanderCreaturesYouOwn : Predicate.check .object [] commanderCreaturesYouOwn = [] := by
  decide

/-- Idol of Endurance -/
def creatureSpellFromAmongExiled : NounPhrase :=
  fromAmong (exactly 1) (.and [creature, spell]) (allOf (.and [.isCard, exiledWithThisArtifact]))
theorem okCreatureSpellFromAmongExiled :
    NounPhrase.check (some .object) [] creatureSpellFromAmongExiled = [] := by decide

def ringHasTemptedYouTwiceThisGame : Condition :=
  .compareAmt (eventCount (.verbedAct (.action "The Ring Tempts You")) .you .thisGame)
    .atLeast (.lit 2)
theorem okRingHasTemptedYouTwiceThisGame :
    Condition.check [] ringHasTemptedYouTwiceThisGame = [] := by decide

def creatureCardAnywhere : Predicate := .and [creature, .isCard]
theorem okCreatureCardAnywhere : Predicate.check .object [] creatureCardAnywhere = [] := by decide
def permanentCardAnywhere : Predicate := permanentCard
theorem okPermanentCardAnywhere : Predicate.check .object [] permanentCardAnywhere = [] := by decide

/-- "target card on the stack" -/
def targetCardOnTheStack : NounPhrase := target cardOnTheStack
theorem okTargetCardOnTheStack : NounPhrase.check (some .object) [] targetCardOnTheStack = [] := by
  decide
/-- "each token on the battlefield" -/
def eachTokenOnTheBattlefield : NounPhrase := allOf tokenOnTheBattlefield
theorem okEachTokenOnTheBattlefield :
    NounPhrase.check (some .object) [] eachTokenOnTheBattlefield = [] := by decide
/-- "each emblem you own" -/
def eachEmblemYouOwn : NounPhrase := allOf (.and [emblem, .hasPossessor .owner .you])
theorem okEachEmblemYouOwn : NounPhrase.check (some .object) [] eachEmblemYouOwn = [] := by decide
/-- "a copy of a card" -/
def aCopyOfACard : NounPhrase := a copyOfACard
theorem okACopyOfACard : NounPhrase.check (some .object) [] aCopyOfACard = [] := by decide

/-- Keeper of the Flame, Keeper of the Light -/
def opponentWithMoreLifeThanYou : Predicate :=
  .and [.opponent, .compare [.playerStat .lifeTotal] .greater (lifeTotalOf .you)]
theorem okOpponentWithMoreLifeThanYou :
    Predicate.check .player [] opponentWithMoreLifeThanYou = [] := by decide
/-- Namor, Atlantean King -/
def playerWithMoreLifeThanYou : Predicate :=
  .and [.anyPlayer, .compare [.playerStat .lifeTotal] .greater (lifeTotalOf .you)]
theorem okPlayerWithMoreLifeThanYou :
    Predicate.check .player [] playerWithMoreLifeThanYou = [] := by decide
def noOpponentHasMoreLifeThanYou : Condition :=
  .not (exists_ (.and [.opponent, .compare [.playerStat .lifeTotal] .greater (lifeTotalOf .you)]))
theorem okNoOpponentHasMoreLifeThanYou :
    Condition.check [] noOpponentHasMoreLifeThanYou = [] := by decide
def someOpponentLacksMoreLifeThanYou : Condition :=
  exists_ (.and [.opponent,
                 .not (.compare [.playerStat .lifeTotal] .greater (lifeTotalOf .you))])
theorem okSomeOpponentLacksMoreLifeThanYou :
    Condition.check [] someOpponentLacksMoreLifeThanYou = [] := by decide
def yourOpponentsHaveMoreLifeThanYou : Condition :=
  .matches (.playerGroup .yourOpponents)
    (.compare [.playerStat .lifeTotal] .greater (lifeTotalOf .you))
theorem okYourOpponentsHaveMoreLifeThanYou :
    Condition.check [] yourOpponentsHaveMoreLifeThanYou = [] := by decide

/-- Disarm {U} — Instant. "Unattach all Equipment from target creature." -/
def disarm : Spelled := spelled <| .singleFaced
  { characteristics :=
    { name := "Disarm", cost := some [pip .blue], types := [.instant],
      text :=
        [ .spell none (.unattach (allOf
            (.and [.hasSubtype (artifactType "Equipment"),
                   .attachedTo (target creature)]))) ] } }

def rawExile : Instruction := exile (target creature)
theorem okRawExile : Instruction.check [] rawExile = [] := by decide
def disenchant : Instruction := destroy (target (.or [artifact, enchantment]))
theorem okDisenchant : Instruction.check [] disenchant = [] := by decide
def icyManipulator : Instruction := .setStatus .tapped (target (.or [artifact, creature, land]))
theorem okIcyManipulator : Instruction.check [] icyManipulator = [] := by decide
def divination : Instruction := .draw .you (.lit 2)
theorem okDivination : Instruction.check [] divination = [] := by decide
def ancestralRecall : Instruction := .draw (target .anyPlayer) (.lit 3)
theorem okAncestralRecall : Instruction.check [] ancestralRecall = [] := by decide
def lifeTotalBecomesOne : Instruction := lifeBecomes (target .anyPlayer) (.lit 1)
theorem okLifeTotalBecomesOne : Instruction.check [] lifeTotalBecomesOne = [] := by decide

/-- Berserker's Frenzy's roll -/
def berserkersFrenzyRoll : Instruction :=
  .sequentially [rollDice .you 2 20, .ignoreOutcomes (.extreme .lowest)]
theorem okBerserkersFrenzyRoll : Instruction.check [] berserkersFrenzyRoll = [] := by decide
def ironMastiffIgnore : Instruction :=
  .sequentially
    [ .rollDice .you
        (countOf (.and [.anyPlayer, .inCombat .attackedBy (some (.combatPlayer .attacking))]))
        (.sides 20),
      .ignoreOutcomes (.allBut .highest) ]
theorem okIronMastiffIgnore : Instruction.check [] ironMastiffIgnore = [] := by decide

/-- Xenosquirrels -/
def xenosquirrelsShift : Ability :=
  .triggered (.rollsDice .you .one none .anyResult) [] none [] none none none
    (.may .you
      (.removeCounters (some (exactly 1)) (some (.printed plusOnePlusOne)) thisCreature)
      (some (shiftResult (.lit 1))) none)
theorem okXenosquirrelsShift : Ability.check [] xenosquirrelsShift = [] := by decide

theorem playersTopCardIsPlural : playersTopCardSlice.plur = .many := by decide

/-- Aetheric Amplifier -/
def doubleYourOwnCounters : Instruction := .doubleCounters .you
theorem okDoubleYourOwnCounters : Instruction.check [] doubleYourOwnCounters = [] := by decide

def permanentCardPhrase : Predicate := permanentCard
theorem permanentCardIsPlaceless : permanentCardPhrase.phraseZone .object = none := by decide

/-- Archpriest of Iona -/
def archpriestOfIonaPower : Ability := .static (.definesPt thisCreature .powerAlone partySize)
theorem okArchpriestOfIonaPower : Ability.check [] archpriestOfIonaPower = [] := by decide
def archpriestOfIonaFullParty : Ability :=
  triggeredIf (.beginningOf .the .combat (.byPlayer .you)) fullParty
    (.sequentially
      [ gets (target creature) (.up (.lit 1)) (.up (.lit 1)) (some untilEndOfTurn),
        gains it (keyword "Flying") (some untilEndOfTurn) ])
theorem okArchpriestOfIonaFullParty : Ability.check [] archpriestOfIonaFullParty = [] := by decide

/-- Squad Commander -/
def squadCommanderTokens : Ability :=
  when (.enters thisCreature none)
    (create partySize (creatureToken 1 1 [.white] [creatureType "Kor", creatureType "Warrior"]))
theorem okSquadCommanderTokens : Ability.check [] squadCommanderTokens = [] := by decide

/-- At Knifepoint -/
def atKnifepointOutlaws : Ability :=
  .static (.onlyDuring .turn (some .you) (.gains (allOf outlawYouControl) (keyword "FirstStrike")))
def atKnifepointCrime : Ability :=
  .triggered (.commitsCrime .you) [] none [] none (some .oncePerTurn) none
    (create (.lit 1)
      { characteristics :=
        { colors := [.red], types := [.creature], subtypes := [creatureType "Mercenary"],
          text :=
            [ activatedOnlyDuring .tapSymbol
                (gets (target creatureYouControl) (.up (.lit 1)) (.up (.lit 0))
                  (some untilEndOfTurn))
                .asSorcery ],
          power := stat 1, toughness := stat 1 } })
def atKnifepoint : Spelled := spelled <| .singleFaced
  { characteristics :=
    { name := "At Knifepoint", cost := some [generic 1, pip .black, pip .red],
      types := [.enchantment], text := [atKnifepointOutlaws, atKnifepointCrime] } }

end Semantics.Cards
