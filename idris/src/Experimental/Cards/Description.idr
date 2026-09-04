module Experimental.Cards.Description

import Experimental
import Experimental.Macros

%default total


glyphOfDestruction : Instruction []
glyphOfDestruction =
  Macros.gets (Macros.target (And [Blocking, Macros.creature, HasPossessor ControllerAx You])) (PtUp (Lit 10)) (PtUp (Lit 0)) (Just Macros.untilEndOfCombat)

rawNonattacking : Predicate [] Object
rawNonattacking = And [Macros.creature, Not Attacking, Not Blocking]

harmonyOfNature : Instruction []
harmonyOfNature =
  Sequentially [ Macros.tap (Macros.counted Macros.anyNumber
                              (And [Macros.untapped, Macros.creature, HasPossessor ControllerAx You]))
               , ForEachOf (Macros.TheVerbed "Tap" (TypeW Creature) ThisWay ManyOf)
                           (Macros.gainsLife You (Lit 4)) ]

ratsOfRath : Instruction []
ratsOfRath = Macros.destroy (Macros.target (And [Or [Macros.artifact, Macros.creature, Macros.land], HasPossessor ControllerAx You]))

anotherDisjunctPhrase : Predicate [MkBinding TargetD Object OneOf
                                             (ObjectP Nothing (Just Battlefield) Nothing Nothing Nothing)] Object
anotherDisjunctPhrase = And [Or [Macros.creature, Macros.land], Other]

defeat : Instruction []
defeat = Macros.destroy (Macros.target (And [Macros.creature, Compare [StatAxis Power] AtMost (Lit 2)]))

terashisVerdict : Instruction []
terashisVerdict =
  Macros.destroy (Macros.target (And [Macros.creature, Attacking, Compare [StatAxis Power] AtMost (Lit 3)]))

pillarOfLight : Instruction []
pillarOfLight =
  Macros.exile You (Macros.target (And [Macros.creature, Compare [StatAxis Toughness] AtLeast (Lit 4)]))

clavilenoPhrase : Predicate [] Object
clavilenoPhrase = And [Macros.creature, Attacking, Not (HasSubtype (creatureType "Demon"))]

unholyAnnex : Instruction []
unholyAnnex =
  Sequentially [(Draw You (Lit 1)),
                If (Macros.exists (And [HasSubtype (creatureType "Demon"), HasPossessor ControllerAx You]))
                   (Sequentially [Macros.losesLife (Macros.each Opponent) (Lit 2),
                                  Macros.gainsLife You (Lit 2)])
                   (Just (Macros.losesLife You (Lit 2)))]

warScreecher : Instruction []
warScreecher =
  Macros.gets (Macros.allOf (Macros.otherCreatureYouControl Macros.thisCreature)) (PtUp (Lit 1)) (PtUp (Lit 1)) (Just Macros.untilEndOfTurn)

mindFlayer : Instruction []
mindFlayer =
  Macros.gainControl You (Macros.target Macros.creature) (Just (ForAsLongAs (Matches Macros.thisCreature (HasPossessor ControllerAx You))))

arcLightning : Instruction []
arcLightning = Macros.dealsDivided This (Lit 3) (Described (TargetDet (Macros.oneThrough 3)) Macros.anyTarget)

boulderfall : Instruction []
boulderfall = Macros.dealsDivided This (Lit 5) (Described (TargetDet Macros.anyNumber) Macros.anyTarget)

banisherPriest : Instruction []
banisherPriest = Macros.exileUntil (Macros.target (And [Macros.creature, HasPossessor ControllerAx Macros.anOpponent]))
                            (Macros.leavesBattlefield Macros.thisCreature)

||| Tezzeret, Artifice Master
tezzeretDrawTwo : Instruction []
tezzeretDrawTwo =
  InsteadOf (Draw You (Lit 1))
            (If (CompareAmt (Macros.countOf (And [Macros.artifact, HasPossessor ControllerAx You]))
                            AtLeast (Lit 3))
                ((Draw You (Lit 2)))
                Nothing)

||| Zimone, Quandrix Prodigy
zimoneDrawTwo : Instruction []
zimoneDrawTwo =
  InsteadOf (Draw You (Lit 1))
            (If (CompareAmt (Macros.countOf (And [Macros.land, HasPossessor ControllerAx You]))
                            AtLeast (Lit 8))
                ((Draw You (Lit 2)))
                Nothing)

aerialVolley : Card
aerialVolley =
  Macros.card "Aerial Volley" (Just [Macros.pip Green]) [] (MkTypeLine [] [Instant])
       [Spell Nothing (Macros.dealsDivided This (Lit 3)
                            (Described (TargetDet (Macros.oneThrough 3)) (And [Macros.creature, HasKeyword (TheKeyword "Flying")])))] Nothing

||| Terrifying Presence
terrifyingPresenceAnchor : Predicate [] Object
terrifyingPresenceAnchor = And [Macros.creature, OtherThan (Macros.target Macros.creature)]

timelyReinforcements : Card
timelyReinforcements =
  Macros.card "Timely Reinforcements"
       (Just [Macros.generic 2, Macros.pip White]) []
       (MkTypeLine [] [Sorcery])
       [ Spell Nothing (Sequentially
                  [ If (CompareAmt (PlayerStatOf LifeTotal You) Less
                                   (PlayerStatOf LifeTotal Macros.anOpponent))
                       (Macros.gainsLife You (Lit 6))
                       Nothing
                  , If (CompareAmt (Macros.countOf Macros.creatureYouControl) Less
                                   (Macros.countOf (And [Macros.creature,
                                                  HasPossessor ControllerAx Macros.anOpponent])))
                       (Macros.create (Lit 3) (Macros.creatureTok 1 1 [White] [creatureType "Soldier"]))
                       Nothing ]) ]
       Nothing

survivalCache : Instruction []
survivalCache =
  Sequentially [ Macros.gainsLife You (Lit 2)
               , If (CompareAmt (PlayerStatOf LifeTotal You) Greater
                                (PlayerStatOf LifeTotal Macros.anOpponent))
                    (Draw You (Lit 1))
                    Nothing ]

nightmarishEnd : Card
nightmarishEnd =
  Macros.card "Nightmarish End"
       (Just [Macros.generic 2, Macros.pip Black]) []
       (MkTypeLine [] [Instant])
       [ Spell Nothing (Sequentially
                  [ Macros.gets (Macros.target Macros.creature)
                                (PtDown (LetterVal X)) (PtDown (LetterVal X))
                                (Just Macros.untilEndOfTurn)
                  , Define X (Macros.countOf (InZone (Macros.handOf You))) ]) ]
       Nothing

topple : Card
topple =
  Macros.card "Topple" (Just [Macros.generic 2, Macros.pip White]) []
       (MkTypeLine [] [Sorcery])
       [ Spell Nothing (Macros.exile You
                  (Macros.target
                     (And [Macros.creature,
                           Superlative MaxOf (StatAxis Power)
                                       (And [Macros.creature,
                                             InZone Macros.battlefieldZ])]))) ]
       Nothing

corruptedOpponents : Noun [] Player
corruptedOpponents =
  Macros.each (And [Opponent, Compare [CounterAxis (NamedCounter "Poison")] AtLeast (Lit 3)])

||| War Tax
public export
warTaxScaledPayment : Cost [letterB X]
warTaxScaledPayment =
  Macros.scaledMana GenericUnit
             (TimesOf (LetterVal X) (Macros.countOf (And [Macros.creature, Attacking])))

||| Croaking Counterpart
public export
croakingCounterpartCopy : Instruction []
croakingCounterpartCopy =
  Create You (Lit 1)
    (TokenCopyOf (Macros.target (And [ Macros.creature
                                     , Not (HasSubtype (creatureType "Frog")) ]))
                 [ExceptChars (MkToken (Just (Lit 1 ** Lit 1)) [Green]
                                       (MkTypeLine [creatureType "Frog"] [])
                                       [] Nothing)
                              False])
    []

||| Sorrow's Path
public export
sorrowsPathCouldBlock : Predicate [] Object
sorrowsPathCouldBlock = CombatRel CouldBlock (Macros.allOf (And [Macros.creature, Attacking]))

||| General Jarkeld's
public export
generalJarkeldCouldBeBlocked : Predicate [] Object
generalJarkeldCouldBeBlocked =
  CombatRel CouldBeBlockedBy (Macros.allOf (And [Macros.creature, Blocking]))

||| Blessed Reversal
public export
blessedReversal : Card
blessedReversal =
  Macros.card "Blessed Reversal"
       (Just [Macros.generic 1, Macros.pip White]) []
       (MkTypeLine [] [Instant])
       [ Spell Nothing (Macros.gainsLife You
                  (Macros.times 3 (Macros.countOf (And [Macros.creature, CombatRel AttackerOf You])))) ]
       Nothing

public export
extinction : Card
extinction =
  Macros.card "Extinction" (Just [Macros.generic 4, Macros.pip Black]) []
       (MkTypeLine [] [Sorcery])
       [ Spell Nothing (Macros.destroy (Macros.allOf (And [Macros.creature,
                                            OfYourChoice (SubtypeQ Creature) Nothing]))) ]
       Nothing

public export
defensiveManeuvers : Card
defensiveManeuvers =
  Macros.card "Defensive Maneuvers"
       (Just [Macros.generic 3, Macros.pip White]) []
       (MkTypeLine [] [Instant])
       [ Spell Nothing (Macros.gets (Macros.allOf (And [Macros.creature,
                                         OfYourChoice (SubtypeQ Creature) Nothing]))
                            (PtUp (Lit 0)) (PtUp (Lit 4))
                            (Just Macros.untilEndOfTurn)) ]
       Nothing

public export
witchsVengeance : Card
witchsVengeance =
  Macros.card "Witch's Vengeance"
       (Just [Macros.generic 1, Macros.pip Black, Macros.pip Black]) []
       (MkTypeLine [] [Sorcery])
       [ Spell Nothing (Macros.gets (Macros.allOf (And [Macros.creature,
                                         OfYourChoice (SubtypeQ Creature) Nothing]))
                            (PtDown (Lit 3)) (PtDown (Lit 3))
                            (Just Macros.untilEndOfTurn)) ]
       Nothing

public export
storiedEnduringStory : Instruction []
storiedEnduringStory =
  If (AndCond
        [ CompareAmt (Macros.countOf (And [Permanent,
                                    Or [Macros.artifact,
                                        HasSubtype (enchantmentType "Saga"),
                                        HasSupertype Legendary],
                                    HasPossessor ControllerAx You]))
                     AtLeast (Lit 3)
        , NotCond (Matches You (HasDesignation EnduringStory)) ])
     Macros.getsEnduringStory
     Nothing

public export
phyrexianRebirth : Card
phyrexianRebirth =
  Macros.card "Phyrexian Rebirth"
       (Just [Macros.generic 4, Macros.pip White, Macros.pip White]) []
       (MkTypeLine [] [Sorcery])
       [ Spell Nothing (Sequentially
                  [ Macros.destroy (Macros.allOf Macros.creature)
                  , Create You (Lit 1)
                           (TokenWritten
                              (MkToken (Just (LetterVal X ** LetterVal X)) []
                                       (MkTypeLine [creatureType "Phyrexian", creatureType "Horror"]
                                                   [Artifact, Creature])
                                       [] Nothing))
                           []
                  , Define X (CountOf
                                (Macros.TheVerbed "Destroy" (TypeW Creature) ThisWay ManyOf)) ]) ]
       Nothing

public export
damningVerdict : Card
damningVerdict =
  Macros.card "Damning Verdict"
       (Just [Macros.generic 3, Macros.pip White, Macros.pip White]) []
       (MkTypeLine [] [Sorcery])
       [ Spell Nothing (Macros.destroy
                  (Macros.allOf (And [Macros.creature, Not (HasCounters Nothing)]))) ]
       Nothing

public export
hazardousConditions : Card
hazardousConditions =
  Macros.card "Hazardous Conditions"
       (Just [Macros.generic 2, Macros.pip Black, Macros.pip Green]) []
       (MkTypeLine [] [Sorcery])
       [ Spell Nothing (Macros.gets (Macros.allOf (And [Macros.creature, Not (HasCounters Nothing)]))
                            (PtDown (Lit 2)) (PtDown (Lit 2))
                            (Just Macros.untilEndOfTurn)) ]
       Nothing

public export
approachOfTheSecondSun : Card
approachOfTheSecondSun =
  Macros.card "Approach of the Second Sun"
       (Just [Macros.generic 6, Macros.pip White]) [] (MkTypeLine [] [Sorcery])
       [ Spell Nothing (If (AndCond
                      [ Matches This (CastFrom (Macros.handOf You))
                      , Macros.happenedInvolving SpellCast
                                                 You
                                                 ThisGame
                                                 (Macros.a (And [Macros.spell, OtherThan This,
                                                                 Named (PrintedName
                                                                          "Approach of the Second Sun")])) ])
                   (Concludes WinGame You)
                   (Just (Sequentially
                            [ Macros.move This (Macros.nthFromTop (Nth 7))
                            , Macros.gainsLife You (Lit 7) ]))) ]
       Nothing

public export
loyaltyAbilityOfEnchanted : Noun bs Object
loyaltyAbilityOfEnchanted =
  Macros.a (And [ AbilityHead LoyaltyClass
                , AbilityOf (AttachHost Enchanted (TypeW Planeswalker)) ])

||| Balance of Power
public export
balanceOfPower : Instruction []
balanceOfPower =
  If (CompareAmt (Macros.countOf (InZone (Macros.handOf (Macros.target Opponent))))
                 Greater
                 (Macros.countOf (InZone (Macros.handOf You))))
     (Draw You TheDifference)
     Nothing

||| Spark Fiend
public export
sparkFiendUpkeepRoll : Instruction []
sparkFiendUpkeepRoll =
  Sequentially [(Macros.rollDice You 2 6),
                Macros.ifThen (CompareAmt (TheOutcome RollResult) Eq (Lit 7))
                              (Macros.sacrifice You Macros.thisCreature)]

||| Erebos, God of the Dead
public export
devotionCondition : Condition []
devotionCondition = CompareAmt (Devotion You (LitColor Black) Nothing) Less (Lit 5)

||| Multiple Choice
public export
multipleChoiceFirstArm : Instruction []
multipleChoiceFirstArm =
  If (CompareAmt (LetterVal X) Eq (Lit 1))
     (Sequentially [ Macros.scry You (Lit 1)
                   , (Draw You (Lit 1)) ])
     Nothing

||| Multiple Choice, fourth arm
public export
multipleChoiceFourthGate : Condition []
multipleChoiceFourthGate = CompareAmt (LetterVal X) AtLeast (Lit 4)

||| Fell the Mighty
public export
fellTheMighty : Instruction []
fellTheMighty =
  Macros.destroy
    (Macros.allOf (And [Macros.creature,
                 Compare [StatAxis Power] Greater
                         (StatOf Power (Macros.target Macros.creature))]))

public export
targetPlayerOrPlaneswalker : Noun bs (Object \/ Player)
targetPlayerOrPlaneswalker =
  Macros.target (Macros.kindJoin AnyPlayer (HasType Planeswalker))

public export
targetOpponentOrPlaneswalker : Noun bs (Object \/ Player)
targetOpponentOrPlaneswalker =
  Macros.target (Macros.kindJoin Opponent (HasType Planeswalker))

||| Baleful Mastery's paid read
public export
balefulMasteryPaidRead : Ability
balefulMasteryPaidRead =
  Spell Nothing (If (Macros.costWasPaid TheAlternative Nothing This)
            (Draw (Macros.a Opponent) (Lit 1)) Nothing)

||| Karai, Future of the Foot
public export
karaiSneakPaidThisTurn : Amount []
karaiSneakPaidThisTurn = Macros.paidCostRead (ByKeyword "Sneak") (Just ThisTurn) This

||| Requiting Hex's read
public export
requitingHexAdditionalRead : Ability
requitingHexAdditionalRead =
  Spell Nothing (If (Macros.costWasPaid TheAdditional Nothing This)
            (Macros.gainsLife You (Lit 2)) Nothing)

||| Lucid Dreams
public export
lucidDreams : Card
lucidDreams =
  Macros.card "Lucid Dreams"
       (Just [Macros.generic 3, Macros.pip Blue, Macros.pip Blue]) []
       (MkTypeLine [] [Sorcery])
       [ Spell Nothing (Sequentially
                  [ Draw You (LetterVal X)
                  , Define X (DistinctCount CardTypeAxis
                                (Macros.allOf (InZone (Macros.graveyardOf You)))) ]) ]
       Nothing

||| Rogues' Gallery
public export
roguesGallery : Card
roguesGallery =
  Macros.card "Rogues' Gallery" (Just [Macros.generic 2, Macros.pip Black]) []
       (MkTypeLine [] [Sorcery])
       [ Spell Nothing (ForEachKindOf ColorAxis Nothing Color
                  (Macros.move (Described (TargetDet (Macros.upTo 1)) (And [Macros.creature, Macros.ofChosen Color,
                                        InZone (Macros.graveyardOf You)]))
                               Macros.handZ)) ]
       Nothing

||| Bioplasm
public export
bioplasmAttack : GameEvent []
bioplasmAttack = Attacks Macros.thisCreature NoDefender

public export
bioplasmExile : Instruction (eventAfter Description.bioplasmAttack)
bioplasmExile = Macros.exile You (Macros.topSlice (Lit 1))

public export
bioplasmAfterExile : Bindings
bioplasmAfterExile = instrIntro Description.bioplasmExile

public export
bioplasmExiledCard : Noun Description.bioplasmAfterExile Object
bioplasmExiledCard = Macros.TheVerbed "Exile" CardW Attributive OneOf

public export
playersTopCardSlice : Noun [] Object
playersTopCardSlice = LibrarySlice OnTop (Lit 1) (PlayerGroup AllPlayers)

||| Deepglow Skate's recipient
public export
deepglowSkateRecipient : Noun [] Object
deepglowSkateRecipient = Described (TargetDet Macros.anyNumber) Permanent

public export
deepglowSkateRecipientRefused : perMemberOk Description.deepglowSkateRecipient = False
deepglowSkateRecipientRefused = Refl

||| Weftwalking
public export
weftwalkingShuffle : Instruction []
weftwalkingShuffle =
  Sequentially
    [ Macros.shuffleInto You (Both (Macros.allOf (InZone (Macros.handOf You)))
                                 (Macros.allOf (InZone (Macros.graveyardOf You))))
    , (Draw You (Lit 7)) ]

||| Gaea's Revenge's protection-shaped phrase
public export
nongreenSpellsOrAbilities : Predicate [] Object
nongreenSpellsOrAbilities =
  Or [ And [Macros.spell, Not (ColorIs Green)]
     , And [ AbilityHead AnyOnStack
           , AbilityOf (Macros.a (And [Macros.source, Not (ColorIs Green)])) ] ]

||| Hot Pursuit
public export
twoOrMorePlayersHaveLost : Condition []
twoOrMorePlayersHaveLost =
  CompareAmt (Macros.countOf (And [AnyPlayer, Macros.happenedTo GameLoss ThisGame]))
             AtLeast (Lit 2)

public export
commanderCreaturesYouOwn : Predicate [] Object
commanderCreaturesYouOwn =
  And [Macros.creature, HasDesignation CommanderD, HasPossessor OwnerAx You]

||| Idol of Endurance
public export
creatureSpellFromAmongExiled : Noun [] Object
creatureSpellFromAmongExiled =
  Macros.fromAmong (Macros.exactly 1) (And [Macros.creature, Macros.spell])
                      (Macros.allOf (And [IsCard, Macros.exiledWithThisArtifact]))

public export
ringHasTemptedYouTwiceThisGame : Condition []
ringHasTemptedYouTwiceThisGame =
  CompareAmt (Macros.eventCount (VerbedAct "The Ring Tempts You") You ThisGame)
             AtLeast (Lit 2)

public export
creatureCardAnywhere : Predicate [] Object
creatureCardAnywhere = And [Macros.creature, IsCard]

public export
permanentCardAnywhere : Predicate [] Object
permanentCardAnywhere = And [Permanent, IsCard]

||| "target card on the stack"
public export
targetCardOnTheStack : Noun [] Object
targetCardOnTheStack = Macros.target Macros.cardOnTheStack

||| "each token on the battlefield"
public export
eachTokenOnTheBattlefield : Noun [] Object
eachTokenOnTheBattlefield = Macros.allOf Macros.tokenOnTheBattlefield

||| "each emblem you own"
public export
eachEmblemYouOwn : Noun [] Object
eachEmblemYouOwn =
  Macros.allOf (And [Macros.emblem, HasPossessor OwnerAx You])

||| "a copy of a card"
public export
aCopyOfACard : Noun [] Object
aCopyOfACard = Macros.a Macros.copyOfACard

||| Keeper of the Flame, Keeper of the Light
public export
opponentWithMoreLifeThanYou : Predicate [] Player
opponentWithMoreLifeThanYou =
  And [Opponent, Compare [PlayerStatAxis LifeTotal] Greater
                         (PlayerStatOf LifeTotal You)]

||| Namor, Atlantean King
public export
playerWithMoreLifeThanYou : Predicate [] Player
playerWithMoreLifeThanYou =
  And [AnyPlayer, Compare [PlayerStatAxis LifeTotal] Greater
                          (PlayerStatOf LifeTotal You)]

public export
noOpponentHasMoreLifeThanYou : Condition []
noOpponentHasMoreLifeThanYou =
  NotCond (Macros.exists (And [Opponent, Compare [PlayerStatAxis LifeTotal] Greater
                                          (PlayerStatOf LifeTotal You)]))

public export
someOpponentLacksMoreLifeThanYou : Condition []
someOpponentLacksMoreLifeThanYou =
  Macros.exists (And [Opponent,
               Not (Compare [PlayerStatAxis LifeTotal] Greater
                            (PlayerStatOf LifeTotal You))])

public export
yourOpponentsHaveMoreLifeThanYou : Condition []
yourOpponentsHaveMoreLifeThanYou =
  Matches (PlayerGroup YourOpponents)
          (Compare [PlayerStatAxis LifeTotal] Greater
                   (PlayerStatOf LifeTotal You))

||| Disarm
public export
disarm : Card
disarm =
  Macros.card "Disarm" (Just [Macros.pip Blue]) [] (MkTypeLine [] [Instant])
       [ Spell Nothing (Unattach
                  (Macros.allOf (And [ HasSubtype (artifactType "Equipment")
                              , AttachedTo (Macros.target Macros.creature) ]))) ]
       Nothing

rawExile : Instruction []
rawExile = Macros.exile You (Macros.target Macros.creature)

disenchant : Instruction []
disenchant = Macros.destroy (Macros.target (Or [Macros.artifact, Macros.enchantment]))

icyManipulator : Instruction []
icyManipulator = SetStatus Tapped (Macros.target (Or [Macros.artifact, Macros.creature, Macros.land]))

divination : Instruction []
divination = (Draw You (Lit 2))

ancestralRecall : Instruction []
ancestralRecall = Draw (Macros.target AnyPlayer) (Lit 3)

lifeTotalBecomesOne : Instruction []
lifeTotalBecomesOne = Macros.lifeTotalBecomes (Macros.target AnyPlayer) (Lit 1)

||| Berserker's Frenzy's roll
public export
berserkersFrenzyRoll : Instruction []
berserkersFrenzyRoll =
  Sequentially [(Macros.rollDice You 2 20), IgnoreOutcomes (IgnoreExtreme LowestRoll)]

public export
ironMastiffIgnore : Instruction []
ironMastiffIgnore =
  Sequentially [(Macros.rollDice You 1 20), IgnoreOutcomes (IgnoreAllBut HighestRoll)]

public export
xenosquirrelsShift : Instruction []
xenosquirrelsShift =
  Sequentially [(Macros.rollDice You 1 6), Macros.shiftResult (Lit 1)]

public export
playersTopCardIsPlural : nounPlur Description.playersTopCardSlice = ManyOf
playersTopCardIsPlural = Refl

||| Aetheric Amplifier
public export
doubleYourOwnCounters : Instruction []
doubleYourOwnCounters = DoubleCounters You

public export
permanentCardPhrase : Predicate [] Object
permanentCardPhrase = And [Permanent, IsCard]

public export
permanentCardIsPlaceless : phraseZone Description.permanentCardPhrase = Nothing
permanentCardIsPlaceless = Refl

||| The party the game computes: a maximum assignment of distinct roles to
||| distinct members, each member filling at most one role [CR#700.8a,700.8b].
||| A member is written as the list of role indices it could fill.
public export
partyCount : List Nat -> List (List Nat) -> Nat
partyCount used [] = 0
partyCount used (roles :: ms) =
  foldl max (partyCount used ms)
        (map (\r => S (partyCount (r :: used) ms))
             (filter (\r => not (elem r used)) roles))

||| A lone Cleric Rogue is a party of one, not two [CR#700.8b].
public export
loneClericRogueIsPartyOfOne : partyCount [] [[0, 1]] = 1
loneClericRogueIsPartyOfOne = Refl

||| The party is maximal, not greedy: a Cleric beside a Cleric Rogue is two,
||| though taking the Cleric Rogue for Cleric first would leave one [CR#700.8b].
public export
clericBesideClericRogueIsTwo : partyCount [] [[0], [0, 1]] = 2
clericBesideClericRogueIsTwo = Refl

||| One creature of each role is a full party [CR#700.8c].
public export
oneOfEachRoleIsFullParty : partyCount [] [[0], [1], [2], [3]] = 4
oneOfEachRoleIsFullParty = Refl

||| Archpriest of Iona
public export
archpriestOfIonaPower : Ability
archpriestOfIonaPower =
  Static (Macros.hasBasePt Macros.thisCreature Macros.partySize (Lit 2))

||| Archpriest of Iona
public export
archpriestOfIonaFullParty : Ability
archpriestOfIonaFullParty =
  Macros.triggeredIf At (BeginningOf ThePart Combat (ByPlayer You)) Macros.fullParty
    (Sequentially
       [ Macros.gets (Macros.target Macros.creature) (PtUp (Lit 1)) (PtUp (Lit 1))
                     (Just Macros.untilEndOfTurn)
       , Macros.gains ((Macros.It OneOf)) (Macros.keyword "Flying")
                      (Just Macros.untilEndOfTurn) ])

||| Squad Commander
public export
squadCommanderTokens : Ability
squadCommanderTokens =
  Macros.triggered When (Enters Macros.thisCreature Nothing)
    (Macros.create Macros.partySize
       (Macros.creatureTok 1 1 [White] [creatureType "Kor", creatureType "Warrior"]))

||| At Knifepoint
public export
atKnifepointOutlaws : Ability
atKnifepointOutlaws =
  Static (OnlyDuring Turn (Just You)
           (Gains (Macros.allOf Macros.outlawYouControl) (Macros.keyword "FirstStrike")))

public export
atKnifepointCrime : Ability
atKnifepointCrime =
  Triggered Whenever (CommitsCrime You) [] Nothing [] Nothing (Just OncePerTurn)
            Nothing
            (Macros.create (Lit 1)
               (MkToken (Just (Lit 1 ** Lit 1)) [Red]
                        (MkTypeLine [creatureType "Mercenary"] [Creature])
                        [ Macros.activatedOnlyDuring TapSymbol
                            (Macros.gets (Macros.target Macros.creatureYouControl)
                                         (PtUp (Lit 1)) (PtUp (Lit 0))
                                         (Just Macros.untilEndOfTurn))
                            AsSorcery ]
                        Nothing))

public export
atKnifepoint : Card
atKnifepoint =
  Macros.card "At Knifepoint"
       (Just [Macros.generic 1, Macros.pip Black, Macros.pip Red]) []
       (MkTypeLine [] [Enchantment])
       [ Description.atKnifepointOutlaws, Description.atKnifepointCrime ]
       Nothing
