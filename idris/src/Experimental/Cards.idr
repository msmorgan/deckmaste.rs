||| The workbench's evidence bench: typechecking positives and pinned negatives over real cards.
module Experimental.Cards

import Experimental
import Experimental.Macros

%default total


||| Lightning Bolt
bolt : Effect []
bolt = DealDamage This (Lit 3) (Macros.target AnyTarget)

barrageOfBoulders : Effect []
barrageOfBoulders = DealDamage This (Lit 1) (Each Macros.creatureYouDontControl)

rabidBite : Effect []
rabidBite = DealDamage (Macros.target Macros.creatureYouControl)
                       (Macros.powerOf It)
                       (Macros.target Macros.creatureYouDontControl)

preyUpon : Effect []
preyUpon = Fights (Macros.target Macros.creatureYouControl) (Macros.target Macros.creatureYouDontControl)

arcTrail : Effect []
arcTrail = Sequentially [DealDamage This (Lit 2) (Macros.target AnyTarget),
                         DealDamage This (Lit 1) (Macros.target Macros.anyOtherTarget)]

cloudshift : Effect []
cloudshift = Sequentially [Macros.exile (Macros.target Macros.creatureYouControl),
                           Macros.putOntoBattlefieldUnderYourControl (That CardW)]

||| Through the Breach Splice
throughTheBreach : Effect []
throughTheBreach = Sequentially [Macros.may You (Move (Macros.a (And [Macros.creature, InZone (Macros.handOf You)])) Macros.battlefieldZ),
                                 Macros.gainsHaste (That (TypeW Creature)) Nothing,
                                 Delayed (BeginningOf EndStep NoPossessor) (Macros.sacrifice You (That (TypeW Creature)))]

bitterDownfall : Effect []
bitterDownfall = Sequentially [Macros.destroy (Macros.target Macros.creature),
                               Macros.losesLife (ControllerOf It) (Lit 2)]

deadshot : Effect []
deadshot = Sequentially [SetStatus Tapped (Macros.target Macros.creature),
                         DealDamage It (Macros.powerOf It) (Macros.target (And [Macros.creature, Other]))]

immersturmSkullcairn : Ability
immersturmSkullcairn =
  Macros.activatedOnlyDuring (Compound [Mana [Macros.generic 1, Macros.pip Black, Macros.pip Red, Macros.pip Red], TapSymbol,
                                        Do (Macros.sacrifice You Macros.thisLand)])
                             (Sequentially [DealDamage It (Lit 3) (Macros.target AnyPlayer),
                                            Macros.discardsACard (That PlayerW)])
                             AsSorcery

pyriteSpellbomb : Ability
pyriteSpellbomb = Activated (Compound [Mana [Macros.pip Red], Do (Macros.sacrifice You Macros.thisArtifact)])
                            (DealDamage It (Lit 2) (Macros.target AnyTarget))

diabolicEdict : Effect []
diabolicEdict = Macros.sacrifice (Macros.target AnyPlayer) (Macros.aTheirChoice Macros.creature)

innocentBlood : Effect []
innocentBlood = Macros.sacrifice (Each AnyPlayer) (Macros.aTheirChoice Macros.creature)

cryOfContrition : Effect []
cryOfContrition = Macros.discardsACard (Macros.target AnyPlayer)

suspendedSentence : Effect []
suspendedSentence = Sequentially [Macros.destroy (Macros.target (And [Macros.creature, ControlledBy Macros.anOpponent])),
                                  Macros.losesLife (That PlayerW) (Lit 3)]

flickeringSpirit : Effect []
flickeringSpirit = Sequentially [Macros.exile Macros.thisCreature,
                                 Move It Macros.battlefieldZ]

turnToMist : Effect []
turnToMist = Sequentially [Macros.exile (Macros.target Macros.creature),
                           Delayed (BeginningOf EndStep NoPossessor) (Move (That CardW) Macros.battlefieldZ)]

jump : Effect []
jump = Macros.gains (Macros.target Macros.creature) (KeywordAbility Flying) (Just Macros.untilEndOfTurn)

giantGrowth : Effect []
giantGrowth = Macros.gets (Macros.target Macros.creature) (PtUp (Lit 3)) (PtUp (Lit 3)) (Just Macros.untilEndOfTurn)

glyphOfDestruction : Effect []
glyphOfDestruction =
  Macros.gets (Macros.target (And [Blocking, Macros.creature, ControlledBy You])) (PtUp (Lit 10)) (PtUp (Lit 0)) (Just Macros.untilEndOfCombat)

gabrielAngelfire : Effect []
gabrielAngelfire =
  Macros.gains Macros.thisCreature (KeywordAbility Flying) (Just Macros.untilYourNextUpkeep)

bondOfRevival : Effect []
bondOfRevival = Sequentially [Move (Macros.target (And [Macros.creature, InZone (Macros.graveyardOf You)])) Macros.battlefieldZ,
                              Macros.gainsHaste It (Just Macros.untilYourNextTurn)]

gracefulReprieve : Effect []
gracefulReprieve = Macros.delayedWithin (Dies (Macros.target Macros.creature))
                                        ThisTurn
                                        (Move (That CardW) Macros.battlefieldZ)

vraskasStoneglare : Effect []
vraskasStoneglare = Sequentially [Macros.destroy (Macros.target Macros.creature),
                                  Macros.gainsLife You (Macros.toughnessOf It)]

phthisis : Effect []
phthisis = Sequentially [Macros.destroy (Macros.target Macros.creature),
                         Macros.losesLife (ControllerOf It) (Plus (Macros.powerOf It) (Macros.toughnessOf It))]

karplusanYeti : Effect []
karplusanYeti = Sequentially [DealDamage Macros.thisCreature (Macros.powerOf Macros.thisCreature) (Macros.target Macros.creature),
                              DealDamage (That (TypeW Creature)) (Macros.powerOf It) Macros.thisCreature]

fulgentDistraction : Effect []
fulgentDistraction = Sequentially [Choose (TargetGroup (Macros.exactly 2) Macros.creature),
                                   SetStatus Tapped (Those (TypeW Creature))]

||| Continue?
continueSpell : Effect []
continueSpell = Sequentially [Choose (TargetGroup (Macros.upTo 4) (And [Macros.creature, InZone (Macros.graveyardOf You)])),
                              Move Them Macros.battlefieldZ]

suddenDemise : Effect []
suddenDemise = Sequentially [Choose (Macros.a (QualityNoun Color)),
                             DealDamage This XVal (Each (And [Macros.creature, OfChosen Color]))]

kindredDominance : Effect []
kindredDominance = Sequentially [Choose (Macros.a (QualityNoun CreatureType)),
                                 Macros.destroy (AllOf (And [Macros.creature, Not (OfChosen CreatureType)]))]

voyagerStaff : Ability
voyagerStaff = Activated (Compound [Mana [Macros.generic 2], Do (Macros.sacrifice You Macros.thisArtifact)])
                         (Sequentially [Macros.exile (Macros.target Macros.creature),
                                          Delayed (BeginningOf EndStep NoPossessor) (Move (TheVerbed Exile CardW) Macros.battlefieldZ)])

boshIronGolem : Ability
boshIronGolem = Activated (Compound [Mana [Macros.generic 3, Macros.pip Red],
                                     Do (Macros.sacrifice You (Macros.a (HasType Artifact)))])
                          (DealDamage This
                                      (Macros.manaValueOf (TheVerbed Sacrifice (TypeW Artifact)))
                                      (Macros.target AnyTarget))

pyromancy : Ability
pyromancy = Activated (Compound [Mana [Macros.generic 3],
                                 Do (Macros.discards You (Macros.aAtRandom (InZone Macros.handZ)))])
                      (DealDamage Macros.thisEnchantment
                                  (Macros.manaValueOf (TheVerbed Discard CardW))
                                  (Macros.target AnyTarget))

foulTongueShriek : Effect []
foulTongueShriek = Sequentially [Macros.losesLife (Macros.target Opponent)
                                           (Macros.forEach (And [Attacking, Macros.creature, ControlledBy You])),
                                 Macros.gainsLife You ThatMuch]

unsummon : Effect []
unsummon = Move (Macros.target Macros.creature) Macros.handZ

phantomBlade : Effect []
phantomBlade = Sequentially [Choose (TargetGroup (Macros.upTo 1) (And [Macros.creature, ControlledBy You])),
                             Macros.destroy (TargetGroup (Macros.upTo 1) (And [Macros.creature, Other]))]

caseOfTheGatewayExpress : Effect []
caseOfTheGatewayExpress = Sequentially [Choose (Macros.target Macros.creatureYouDontControl),
                                        DealDamage (Each Macros.creatureYouControl) (Lit 1)
                                                   (That (TypeW Creature))]

cyclingCost : Effect []
cyclingCost = Macros.discards You This

rawNonattacking : Predicate [] Object
rawNonattacking = And [Macros.creature, Not Attacking, Not Blocking]

rawExile : Effect []
rawExile = Macros.exile (Macros.target Macros.creature)


disenchant : Effect []
disenchant = Macros.destroy (Macros.target (Or [Macros.artifact, Macros.enchantment]))

icyManipulator : Effect []
icyManipulator = SetStatus Tapped (Macros.target (Or [Macros.artifact, Macros.creature, Macros.land]))

ratsOfRath : Effect []
ratsOfRath = Macros.destroy (Macros.target (And [Or [Macros.artifact, Macros.creature, Macros.land], ControlledBy You]))

arrowsOfJustice : Effect []
arrowsOfJustice = DealDamage This (Lit 4)
                             (Macros.target (And [Macros.creature, Or [Attacking, Blocking]]))

anotherDisjunctPhrase : Predicate [MkBinding TargetD Object OneOf
                                             (ObjectP Nothing (Just Battlefield) Nothing Nothing)] Object
anotherDisjunctPhrase = And [Or [Macros.creature, Macros.land], Other]


infiltrate : Effect []
infiltrate = Macros.cantBeBlocked (Macros.target Macros.creature) (Just Macros.thisTurn)

changeOfHeart : Effect []
changeOfHeart = Macros.cantAttack (Macros.target Macros.creature) (Just Macros.thisTurn)

blindblast : Effect []
blindblast = Sequentially [DealDamage This (Lit 1) (Macros.target Macros.creature),
                           Macros.cantBlock (That (TypeW Creature)) (Just Macros.thisTurn)]

blindingFlare : Effect []
blindingFlare = Macros.cantBlock (TargetGroup Macros.anyNumber Macros.creature) (Just Macros.thisTurn)


defeat : Effect []
defeat = Macros.destroy (Macros.target (And [Macros.creature, Compare Power AtMost (Lit 2)]))

terashisVerdict : Effect []
terashisVerdict =
  Macros.destroy (Macros.target (And [Macros.creature, Attacking, Compare Power AtMost (Lit 3)]))

pillarOfLight : Effect []
pillarOfLight =
  Macros.exile (Macros.target (And [Macros.creature, Compare Toughness AtLeast (Lit 4)]))

luckyOffering : Effect []
luckyOffering =
  Sequentially [Macros.destroy (Macros.target (And [Macros.artifact,
                                      Compare ManaValue AtMost (Lit 3)])),
                Macros.gainsLife You (Lit 3)]


overload : Effect []
overload = If (Macros.destroy (Macros.target Macros.artifact))
              (CompareAmt (Macros.manaValueOf It) AtMost (Lit 2))
              Nothing

builtToSmash : Effect []
builtToSmash =
  Sequentially [Macros.gets (Macros.target (And [Macros.creature, Attacking])) (PtUp (Lit 3)) (PtUp (Lit 3)) (Just Macros.untilEndOfTurn),
                If (Macros.gains It (KeywordAbility Trample) (Just Macros.untilEndOfTurn))
                   (Macros.itsA (And [Macros.artifact, Macros.creature]))
                   Nothing]

flamesOfTheRazeBoar : Effect []
flamesOfTheRazeBoar =
  Sequentially [DealDamage This (Lit 4) (Macros.target (And [Macros.creature, ControlledBy Macros.anOpponent])),
                If (DealDamage This (Lit 2)
                               (Each (And [Macros.creature, Other, ControlledBy (That PlayerW)])))
                   (Exists (And [Macros.creature, ControlledBy You,
                                 Compare Power AtLeast (Lit 4)]))
                   Nothing]

||| Braids's Frightful Return
braidsFrightfulReturn : Effect []
braidsFrightfulReturn =
  Macros.mayThen You (Macros.sacrifice You (Macros.a Macros.creature)) (Macros.discardsACard (Each Opponent))

||| Daretti, Ingenious Iconoclast
darettisMinusOne : Effect []
darettisMinusOne =
  Macros.mayThen You (Macros.sacrifice You (Macros.a Macros.artifact))
              (Macros.destroy (Macros.target (Or [Macros.artifact, Macros.creature])))

||| Yawgmoth Demon
crovaxTheCursed : Effect []
crovaxTheCursed =
  Macros.mayThenElse You (Macros.sacrifice You (Macros.a Macros.creature))
                  (PutCounters (Lit 1) Macros.plusOnePlusOne Macros.thisCreature)
                  (RemoveCounters (Lit 1) Macros.plusOnePlusOne Macros.thisCreature)

yawgmothDemon : Effect []
yawgmothDemon =
  Macros.mayElse You (Macros.sacrifice You (Macros.a Macros.artifact))
              (Sequentially [SetStatus Tapped Macros.thisCreature, DealDamage This (Lit 2) You])


raiseTheAlarm : Effect []
raiseTheAlarm = Macros.create (Lit 2) (Macros.creatureTok 1 1 [White] [Soldier])

additiveEvolution : Effect []
additiveEvolution = Sequentially [Macros.create (Lit 1) (Macros.creatureTok 0 0 [Green, Blue] [Fractal]),
                                  PutCounters (Lit 3) Macros.plusOnePlusOne It]

aviationPioneer : Effect []
aviationPioneer =
  Macros.create (Lit 1) (MkToken (Just (Lit 1, Lit 1)) [] (MkTypeLine [Thopter] [Artifact, Creature])
                          [KeywordAbility Flying] Nothing)

fireNavyTrebuchet : Effect []
fireNavyTrebuchet =
  Macros.createTappedAttacking (Lit 1)
    (MkToken (Just (Lit 2, Lit 1)) [] (MkTypeLine [Construct] [Artifact, Creature])
             [KeywordAbility Flying] (Just "Ballistic Boulder"))

amassZombiesTwo : Effect []
amassZombiesTwo =
  Sequentially [If (Macros.create (Lit 1) (Macros.creatureTok 0 0 [Black] [Zombie, Army]))
                   (Macros.notSo (Exists (And [HasSubtype Army, Macros.creature, ControlledBy You])))
                   Nothing,
                Choose (Macros.a (And [HasSubtype Army, Macros.creature, ControlledBy You])),
                PutCounters (Lit 2) Macros.plusOnePlusOne (That (TypeW Creature)),
                If (Macros.becomes It (Macros.subtypesOnly [Zombie]) Nothing)
                   (Macros.itIsntA (HasSubtype Zombie))
                   Nothing]

battlegrowth : Effect []
battlegrowth = PutCounters (Lit 1) Macros.plusOnePlusOne (Macros.target Macros.creature)

chainbreaker : Effect []
chainbreaker = RemoveCounters (Lit 1) Macros.minusOneMinusOne (Macros.target Macros.creature)

kaitoBaneOfNightmares : Effect []
kaitoBaneOfNightmares = Sequentially [SetStatus Tapped (Macros.target Macros.creature),
                                      PutCounters (Lit 2) Stun It]

jhoiraOfTheGhitu : Ability
jhoiraOfTheGhitu =
  Activated (Compound [Mana [Macros.generic 2],
                       Do (Macros.exile (Macros.a (And [Not Macros.land, InZone (Macros.handOf You)])))])
            (PutCounters (Lit 4) Time (TheVerbed Exile CardW))

alaundoTheSeer : Effect []
alaundoTheSeer = RemoveCounters (Lit 1) Time (Each (InZone Macros.exileZ))

arcBlade : Effect []
arcBlade = Sequentially [DealDamage This (Lit 2) (Macros.target AnyTarget),
                         Macros.exileWithCounters This (Lit 3) Time]

daydream : Card
daydream =
  Macros.card "Daydream" (Just [Macros.pip White]) [] (MkTypeLine [] [Sorcery])
       [Spell (Sequentially [Macros.exile (Macros.target Macros.creatureYouControl),
                             Macros.returnToBattlefieldWithCounters
                               (That CardW) (OwnerOf (That CardW))
                               (Lit 1) Macros.plusOnePlusOne])]
       Nothing

ashnodsTransmogrant : Ability
ashnodsTransmogrant =
  Activated (Compound [TapSymbol, Do (Macros.sacrifice You Macros.thisArtifact)])
            (Sequentially [PutCounters (Lit 1) Macros.plusOnePlusOne
                                         (Macros.target (And [Macros.creature, Not Macros.artifact])),
                             Macros.becomes (That (TypeW Creature)) (Macros.typesOnly [Artifact]) Nothing])

neurokTransmuter : Effect []
neurokTransmuter = Macros.becomes (Macros.target Macros.creature) (Macros.typesOnly [Artifact]) (Just Macros.untilEndOfTurn)

cowardKiller : Effect []
cowardKiller = Sequentially [Macros.cantBlock (Macros.target Macros.creature) (Just Macros.thisTurn),
                             Macros.becomes (That (TypeW Creature)) (Macros.subtypesOnly [Coward])
                                     (Just Macros.untilEndOfTurn)]

clavilenoPhrase : Predicate [] Object
clavilenoPhrase = And [Macros.creature, Attacking, Not (HasSubtype Demon)]

unholyAnnex : Effect []
unholyAnnex =
  Sequentially [Macros.drawACard,
                If (Sequentially [Macros.losesLife (Each Opponent) (Lit 2),
                                  Macros.gainsLife You (Lit 2)])
                   (Exists (And [HasSubtype Demon, ControlledBy You]))
                   (Just (Macros.losesLife You (Lit 2)))]


divination : Effect []
divination = Macros.drawCards 2

ancestralRecall : Effect []
ancestralRecall = Draw (Macros.target AnyPlayer) (Lit 3)

||| Cheering Fanatic
||| "Whenever this creature attacks, choose a card name. Spells with the
||| chosen name cost {1} less to cast this turn."
public export
cheeringFanatic : Card
cheeringFanatic =
  Macros.card "Cheering Fanatic" (Just [Macros.generic 1, Macros.pip Red]) []
       (MkTypeLine [Goblin] [Creature])
       [ Triggered Whenever (Attacks Macros.thisCreature)
                   (Sequentially
                     [ Choose (Macros.a (QualityNoun CardName))
                     , Continuously
                         (CostsToCast (AllOf (And [Macros.spell, OfChosen CardName]))
                                      (CostLess (Lit 1)))
                         (Just Macros.thisTurn) ]) ]
       (Just (2, 2))

public export
prosperity : Card
prosperity =
  Macros.card "Prosperity" (Just [Variable, Macros.pip Blue]) []
       (MkTypeLine [] [Sorcery]) [Spell (Draw (Each AnyPlayer) XVal)] Nothing

collectiveUnconscious : Effect []
collectiveUnconscious = Draw You (Macros.forEach Macros.creatureYouControl)

killiansConfidence : Effect []
killiansConfidence = Sequentially [Macros.gets (Macros.target Macros.creature) (PtUp (Lit 1)) (PtUp (Lit 1)) (Just Macros.untilEndOfTurn),
                                   Macros.drawACard]

||| Blindblast
blindblastWhole : Effect []
blindblastWhole = Sequentially [DealDamage This (Lit 1) (Macros.target Macros.creature),
                                Macros.cantBlock (That (TypeW Creature)) (Just Macros.thisTurn),
                                Macros.drawACard]

cycling : Ability
cycling = Activated (Compound [Mana [Macros.generic 2], Do (Macros.discards You This)]) Macros.drawACard

abrade : Effect []
abrade = Macros.chooseOne [DealDamage This (Lit 3) (Macros.target Macros.creature),
                    Macros.destroy (Macros.target Macros.artifact)]

austereCommand : Effect []
austereCommand =
  Macros.chooseTwo [Macros.destroy (AllOf Macros.artifact),
             Macros.destroy (AllOf Macros.enchantment),
             Macros.destroy (AllOf (And [Macros.creature, Compare ManaValue AtMost (Lit 3)])),
             Macros.destroy (AllOf (And [Macros.creature, Compare ManaValue AtLeast (Lit 4)]))]

azulaAlwaysLies : Effect []
azulaAlwaysLies =
  Macros.chooseOneOrBoth [Macros.gets (Macros.target Macros.creature) (PtDown (Lit 1)) (PtDown (Lit 1)) (Just Macros.untilEndOfTurn),
                   PutCounters (Lit 1) Macros.plusOnePlusOne (Macros.target Macros.creature)]

rainOfThorns : Effect []
rainOfThorns = Macros.chooseOneOrMore [Macros.destroy (Macros.target Macros.artifact),
                                Macros.destroy (Macros.target Macros.enchantment),
                                Macros.destroy (Macros.target Macros.land)]

rankleMasterOfPranks : Effect []
rankleMasterOfPranks =
  Macros.chooseAnyNumber [Macros.discardsACard (Each AnyPlayer),
                   Macros.sacrifice (Each AnyPlayer) (Macros.aTheirChoice Macros.creature)]

myrkulsEdict : Effect []
myrkulsEdict = Sequentially [Choose (Macros.a Opponent),
                             Macros.sacrifice (That PlayerW) (Macros.aTheirChoice Macros.creature)]



nibelheimAflame : Effect []
nibelheimAflame =
  Sequentially [Choose (Macros.target Macros.creatureYouControl),
                DealDamage It (Macros.powerOf It) (Each (Macros.otherCreature It))]

warScreecher : Effect []
warScreecher =
  Macros.gets (AllOf (Macros.otherCreatureYouControl Macros.thisCreature)) (PtUp (Lit 1)) (PtUp (Lit 1)) (Just Macros.untilEndOfTurn)

bellowingAegisaur : Effect []
bellowingAegisaur =
  PutCounters (Lit 1) Macros.plusOnePlusOne (Each (Macros.otherCreatureYouControl Macros.thisCreature))

carnifexDemon : Effect []
carnifexDemon =
  PutCounters (Lit 1) Macros.minusOneMinusOne (Each (Macros.otherCreature Macros.thisCreature))

syphonMind : Effect []
syphonMind = Macros.discardsACard (Each Macros.otherPlayer)

brashTaunter : Effect []
brashTaunter = Fights Macros.thisCreature (Macros.target (Macros.otherCreature Macros.thisCreature))

ulvenwaldTracker : Effect []
ulvenwaldTracker = Fights (Macros.target Macros.creatureYouControl) (Macros.target (And [Macros.creature, Other]))

actOfTreason : Effect []
actOfTreason = Macros.gainControl (Macros.target Macros.creature) (Just Macros.untilEndOfTurn)

mindFlayer : Effect []
mindFlayer =
  Macros.gainControl (Macros.target Macros.creature) (Just (ForAsLongAs (Matches Macros.thisCreature (ControlledBy You))))

phyrexianInfiltrator : Effect []
phyrexianInfiltrator =
  Simultaneously
    [Macros.gainsControl (ControllerOf (Macros.target Macros.creature)) Macros.thisCreature Nothing,
     Macros.gainsControl (ControllerOf Macros.thisCreature) (That (TypeW Creature)) Nothing]

grismold : Effect []
grismold = Create (Each AnyPlayer) (Lit 1)
                  (TokenWritten (Macros.creatureTok 1 1 [Green] [Plant])) []

sparkmagesGambit : Effect []
sparkmagesGambit =
  Sequentially [DealDamage This (Lit 1) (EachOf (TargetGroup (Macros.upTo 2) Macros.creature)),
                Macros.cantBlock (Those (TypeW Creature)) (Just Macros.thisTurn)]

ajaniAdversaryOfTyrants : Effect []
ajaniAdversaryOfTyrants =
  PutCounters (Lit 1) Macros.plusOnePlusOne (EachOf (TargetGroup (Macros.upTo 2) Macros.creature))

fallOfTheTitans : Effect []
fallOfTheTitans = DealDamage This XVal (EachOf (TargetGroup (Macros.upTo 2) AnyTarget))

naturesPanoply : Effect []
naturesPanoply =
  Sequentially [Choose (TargetGroup Macros.anyNumber Macros.creature),
                PutCounters (Lit 1) Macros.plusOnePlusOne (EachOf Them)]

arcLightning : Effect []
arcLightning = Macros.dealsDivided This (Lit 3) (TargetGroup (Macros.oneThrough 3) AnyTarget)

forkedBolt : Effect []
forkedBolt = Macros.dealsDivided This (Lit 2) (TargetGroup (Macros.oneThrough 2) AnyTarget)

boulderfall : Effect []
boulderfall = Macros.dealsDivided This (Lit 5) (TargetGroup Macros.anyNumber AnyTarget)

armamentCorps : Effect []
armamentCorps =
  Macros.distributeCounters (Lit 2) Macros.plusOnePlusOne
                     (TargetGroup (Macros.oneThrough 2) Macros.creatureYouControl)



peek : Effect []
peek = Sequentially [Macros.lookAtHandOf (Macros.target AnyPlayer), Macros.drawACard]

impulse : Effect []
impulse = Sequentially [ Macros.lookAt (Macros.topCards 4)
                       , Move (Macros.oneOf Them) Macros.handZ
                       , Move TheRest (Macros.onBottomIn AnyOrder)
                       ]

anticipate : Effect []
anticipate = Sequentially [ Macros.lookAt (Macros.topCards 3)
                          , Move (Macros.oneOf Them) Macros.handZ
                          , Move TheRest (Macros.onBottomIn AnyOrder)
                          ]

revealFourPartition : Effect []
revealFourPartition = Sequentially [ Macros.revealCards (Macros.topCards 4)
                                   , Move (Macros.oneOf (Those CardW)) Macros.handZ
                                   , Move TheRest Macros.graveyardZ
                                   ]

exileFourOfThem : Effect []
exileFourOfThem = Sequentially [ Macros.lookAt (Macros.topCards 8)
                               , Macros.exile (Macros.someOf 4 Them)
                               , Move TheRest (Macros.onTopIn AnyOrder)
                               ]

||| Grenzo, Dungeon Warden
grenzoBottomCard : Effect []
grenzoBottomCard = Move Macros.bottomCard Macros.graveyardZ

sylvanScrying : Effect []
sylvanScrying = Sequentially [ Macros.searchLibraryFor Macros.land
                             , Macros.revealCards It
                             , Move It Macros.handZ
                             , Macros.shuffle
                             ]

searchToBattlefield : Effect []
searchToBattlefield = Sequentially [ Macros.searchLibraryFor Macros.creature
                                   , Move It Macros.battlefieldZ
                                   , Macros.shuffle
                                   ]

glimpseTheUnthinkable : Effect []
glimpseTheUnthinkable =
  Macros.mills (Macros.target AnyPlayer) (Lit 10) They

millThenReadGroup : Effect []
millThenReadGroup =
  Sequentially [ Macros.mills You (Lit 3) You
               , Macros.exile (Those CardW) ]

lookAtTopThenBin : Effect []
lookAtTopThenBin =
  Sequentially [Macros.lookAt Macros.topCard, Macros.may You (Move (That CardW) Macros.graveyardZ)]



botBashingTime : Effect []
botBashingTime =
  Sequentially [ DealDamage This (Lit 6) (Macros.target Macros.creature)
               , Macros.ifWouldInstead (Dies (That (TypeW Creature))) (Macros.exile It) (Just Macros.thisTurn)
               ]

wordsOfWar : Effect []
wordsOfWar = Macros.nextTimeWouldInstead (Draws You)
                                  (DealDamage This (Lit 2) (Macros.target AnyTarget))
                                  (Just Macros.thisTurn)

wordsOfWorship : Effect []
wordsOfWorship = Macros.nextTimeWouldInstead (Draws You)
                                      (Macros.gainsLife You (Lit 5))
                                      (Just Macros.thisTurn)

||| Fog, Holy Day, Darkness, Root Snare
fog : Effect []
fog = Macros.preventAll CombatOnly Everywhere (Just Macros.thisTurn)

||| Indestructible Aura, Shielded Passage
indestructibleAura : Effect []
indestructibleAura = Macros.preventAll AnyDamage (Macros.shieldingIt (Macros.target Macros.creature)) (Just Macros.thisTurn)

shieldmatesBlessing : Effect []
shieldmatesBlessing =
  Macros.preventNext AnyDamage (Lit 3) (Macros.shieldingIt (Macros.target AnyTarget)) (Just Macros.thisTurn)

banisherPriest : Effect []
banisherPriest = Macros.exileUntil (Macros.target (And [Macros.creature, ControlledBy Macros.anOpponent]))
                            (Leaves Macros.thisCreature)

||| Tezzeret, Artifice Master
tezzeretDrawTwo : Effect []
tezzeretDrawTwo =
  Macros.insteadOf Macros.drawACard
            (If (Macros.drawCards 2)
                (CompareAmt (CountOf (And [Macros.artifact, ControlledBy You]))
                            AtLeast (Lit 3))
                Nothing)

||| Zimone, Quandrix Prodigy
zimoneDrawTwo : Effect []
zimoneDrawTwo =
  Macros.insteadOf Macros.drawACard
            (If (Macros.drawCards 2)
                (CompareAmt (CountOf (And [Macros.land, ControlledBy You]))
                            AtLeast (Lit 8))
                Nothing)


merrowGrimeblotter : Ability
merrowGrimeblotter =
  Activated (Compound [Mana [Macros.generic 1, Macros.hybridPip Blue Black], UntapSymbol])
            (Macros.gets (Macros.target Macros.creature) (PtDown (Lit 2)) (PtDown (Lit 0)) (Just Macros.untilEndOfTurn))

phyrexianSnowcrusher : Ability
phyrexianSnowcrusher =
  Activated (Mana [Macros.generic 1, SnowMana])
            (Macros.gets Macros.thisCreature (PtUp (Lit 1)) (PtUp (Lit 0)) (Just Macros.untilEndOfTurn))

havocSower : Ability
havocSower =
  Activated (Mana [Macros.generic 1, Macros.colorlessPip])
            (Macros.gets Macros.thisCreature (PtUp (Lit 2)) (PtUp (Lit 1)) (Just Macros.untilEndOfTurn))

||| Erebos, God of the Dead
erebos : Ability
erebos = Activated (Compound [Mana [Macros.generic 1, Macros.pip Black], Macros.payLife You 2]) Macros.drawACard

savagebornHydra : Ability
savagebornHydra =
  Macros.activatedOnlyDuring (Mana [Macros.generic 1, Macros.hybridPip Red Green])
                             (PutCounters (Lit 1) Macros.plusOnePlusOne Macros.thisCreature)
                             AsSorcery

baskingRootwalla : Ability
baskingRootwalla =
  Macros.activatedOnlyOnce (Mana [Macros.generic 1, Macros.pip Green])
                           (Macros.gets Macros.thisCreature (PtUp (Lit 2)) (PtUp (Lit 2)) (Just Macros.untilEndOfTurn))
                           OncePerTurn

bondersEnclave : Card
bondersEnclave =
  Macros.card "Bonders' Enclave" Nothing [] (MkTypeLine [] [Land])
       [ Activated TapSymbol (AddMana You (Lit 1) (Runs [[Colorless]]) [])
       , Macros.activatedOnlyIf (Compound [Mana [Macros.generic 3], TapSymbol])
                                Macros.drawACard
                                (Exists (And [Macros.creature, ControlledBy You,
                                              Compare Power AtLeast (Lit 4)])) ]
       Nothing

woeleecher : Ability
woeleecher =
  Activated (Compound [Mana [Macros.pip White], TapSymbol])
            (Macros.doThen (RemoveCounters (Lit 1) Macros.minusOneMinusOne (Macros.target Macros.creature))
                    (Macros.gainsLife You (Lit 2)))

moltingHarpy : Effect []
moltingHarpy = Macros.mayElse You (Pay You (Mana [Macros.generic 2])) (Macros.sacrifice You Macros.thisCreature)

carnophage : Effect []
carnophage = Macros.mayElse You (Pay You (Macros.payLife You 1)) (SetStatus Tapped Macros.thisCreature)

solitaryConfinement : Effect []
solitaryConfinement = Macros.mayElse You (Macros.discardsACard You) (Macros.sacrifice You Macros.thisEnchantment)


securityDetail : Ability
securityDetail =
  Macros.activatedOnlyOnceIf (Mana [Macros.pip White, Macros.pip White])
                             (Macros.create (Lit 1) (Macros.creatureTok 1 1 [White] [Soldier]))
                             OncePerTurn
                             (Macros.notSo (Exists (And [Macros.creature, ControlledBy You])))




cloudkinSeer : Ability
cloudkinSeer = Triggered When (Enters Macros.thisCreature) Macros.drawACard

moonlitWake : Ability
moonlitWake = Triggered Whenever (Dies (Macros.a Macros.creature)) (Macros.gainsLife You (Lit 1))

promiseOfTomorrow : Ability
promiseOfTomorrow = Triggered Whenever (Dies (Macros.a Macros.creatureYouControl)) (Macros.exile It)

staffOfNin : Ability
staffOfNin = Triggered At (BeginningOf Upkeep (ByWord Yours)) Macros.drawACard

libraryLarcenist : Ability
libraryLarcenist = Triggered Whenever (Attacks Macros.thisCreature) Macros.drawACard

eliteJavelineer : Ability
eliteJavelineer =
  Triggered Whenever (Blocks Macros.thisCreature Nothing)
            (DealDamage This (Lit 1) (Macros.target (And [Macros.creature, Attacking])))

jhessianThief : Ability
jhessianThief =
  Triggered Whenever (DealsCombatDamage Macros.thisCreature (Macros.a AnyPlayer)) Macros.drawACard

scholarOfStars : Ability
scholarOfStars =
  Macros.triggeredIf When
                     (Enters Macros.thisCreature)
                     (Exists (And [Macros.artifact, ControlledBy You]))
                     Macros.drawACard

||| Glacial Chasm
glacialChasmCant : Ability
glacialChasmCant = Static (Deontic (AllOf Macros.creatureYouControl) Forbid Attack Agent NoDeonticPatient)

||| Glacial Chasm
glacialChasmShield : Ability
glacialChasmShield =
  Static (Prevents AnyDamage AllOfIt (Macros.shieldingIt You) Nothing Nothing)

miserysShadow : Ability
miserysShadow =
  Static (Intercepts (Dies (Macros.a (And [Macros.creature, ControlledBy (Macros.a Opponent)])))
                     (Macros.exile It) Repeatedly)

thoughtReflection : Ability
thoughtReflection =
  Static (Intercepts (Draws You) (Draw You (Lit 2)) Repeatedly)

jorKadeen : Ability
jorKadeen =
  Static (Macros.asLongAs (CompareAmt (CountOf (And [Macros.artifact, ControlledBy You]))
                               AtLeast (Lit 3))
                   (Gets (AllOf Macros.creatureYouControl) (PtUp (Lit 3)) (PtUp (Lit 0))))

abandonedOutpost : Ability
abandonedOutpost = Static (Macros.entersTapped (AsType Land This))

aerialAssault : Effect []
aerialAssault = Macros.destroy (Macros.target (And [Macros.creature, Macros.tapped]))

asphyxiate : Effect []
asphyxiate = Macros.destroy (Macros.target (And [Macros.creature, Macros.untapped]))

aphettoAlchemist : Ability
aphettoAlchemist = Activated TapSymbol
                             (SetStatus Untapped (Macros.target (Or [Macros.artifact, Macros.creature])))

||| Time Vault
timeVaultLock : Ability
timeVaultLock = Static (DoesntUntap (AsType Artifact This))

vindicate : Effect []
vindicate = Macros.destroy (Macros.target Permanent)

yasminKhan : Ability
yasminKhan =
  Activated TapSymbol
            (Sequentially [Macros.exile Macros.topCard,
                           Continuously (MayPlay You It) (Just Macros.untilYourNextEndStep)])

||| Brazen Cannonade
brazenCannonadePermission : Effect []
brazenCannonadePermission =
  Sequentially [Macros.exile Macros.topCard,
                Continuously (MayPlay You It) (Just (Until (EndOf Combat (Just Yours))))]


corneredCrook : Ability
corneredCrook =
  Triggered When (Enters Macros.thisCreature)
    (Macros.mayWhen You (Macros.sacrifice You (Macros.a Macros.artifact))
                 (DealDamage This (Lit 3) (Macros.target AnyTarget)))

||| The Last Ronin
theLastRoninII : Effect []
theLastRoninII =
  Reflexively (Macros.mills You (Lit 4) You)
              (Move (Macros.target (And [Macros.creature, InZone (Macros.graveyardOf You)])) Macros.handZ)

thousandMoonsCrackshot : Ability
thousandMoonsCrackshot =
  Triggered Whenever (Attacks Macros.thisCreature)
    (Macros.mayWhen You (Pay You (Mana [Macros.generic 2, Macros.pip White]))
                 (SetStatus Tapped (Macros.target Macros.creature)))

anointerOfValor : Ability
anointerOfValor =
  Triggered Whenever (Attacks (Macros.a Macros.creature))
    (Macros.mayWhen You (Pay You (Mana [Macros.generic 3]))
                 (PutCounters (Lit 1) Macros.plusOnePlusOne (That (TypeW Creature))))


zimoneQuandrixProdigy : Ability
zimoneQuandrixProdigy =
  Activated (Compound [Mana [Macros.generic 1], TapSymbol])
            (Macros.may You (Macros.putOntoBattlefieldTapped
                        (Macros.a (And [Macros.land, InZone (Macros.handOf You)]))))

preeminentCaptain : Ability
preeminentCaptain =
  Triggered Whenever (Attacks Macros.thisCreature)
    (Macros.may You (Macros.putOntoBattlefieldTappedAttacking
                (Macros.a (And [HasSubtype Soldier, Macros.creature, InZone (Macros.handOf You)]))))

hymnOfRebirth : Effect []
hymnOfRebirth =
  Macros.putOntoBattlefieldUnderYourControl
    (Macros.target (And [Macros.creature, InZone Macros.graveyardZ]))

desperateCastaways : Ability
desperateCastaways =
  Static (Macros.unlessSo (Exists (And [Macros.artifact, ControlledBy You]))
                   (Deontic Macros.thisCreature Forbid Attack Agent NoDeonticPatient))

silentAssassin : Ability
silentAssassin =
  Activated (Mana [Macros.generic 3, Macros.pip Black])
            (Delayed (BeginningOf EndOfCombat NoPossessor)
                     (Macros.destroy (Macros.target (And [Blocking, Macros.creature]))))

workhorse : Card
workhorse =
  Macros.card "Workhorse" (Just [Macros.generic 6]) []
       (MkTypeLine [Horse] [Artifact, Creature])
       [ Static (Macros.entersWithCounters Macros.thisCreature 4 Macros.plusOnePlusOne)
       , Activated (Do (RemoveCounters (Lit 1) Macros.plusOnePlusOne Macros.thisCreature))
                   (AddMana You (Lit 1) (Runs [[Colorless]]) []) ]
       (Just (0, 0))


counterspell : Effect []
counterspell = Macros.counterSpell (Macros.target Macros.spell)

beastWhisperer : Ability
beastWhisperer =
  Triggered Whenever (Casts You (Macros.a (And [Macros.creature, Macros.spell]))) Macros.drawACard

skaabRuinator : Ability
skaabRuinator =
  Static (Macros.mayCastFrom You This (Macros.graveyardOf You))


escapeToTheWilds : Effect []
escapeToTheWilds =
  Sequentially [Macros.exile (Macros.topCards 5),
                Continuously
                  (MayPlay You (Macros.thoseVerbedThisWay Exile CardW))
                  (Just (Until (EndOf Turn (Just Yours))))]


scatheZombies : Card
scatheZombies = Macros.card "Scathe Zombies" (Just [Macros.generic 2, Macros.pip Black]) []
                     (MkTypeLine [Zombie] [Creature]) [] (Just (2, 2))

rorixBladewing : Card
rorixBladewing =
  Macros.card "Rorix Bladewing" (Just [Macros.generic 3, Macros.pip Red, Macros.pip Red, Macros.pip Red])
       [Legendary] (MkTypeLine [Dragon] [Creature])
       [KeywordAbility Flying, KeywordAbility Haste] (Just (6, 5))

aladdinsRing : Card
aladdinsRing =
  Macros.card "Aladdin's Ring" (Just [Macros.generic 8]) [] (MkTypeLine [] [Artifact])
       [Activated (Compound [Mana [Macros.generic 8], TapSymbol])
                  (DealDamage Macros.thisArtifact (Lit 4) (Macros.target AnyTarget))]
       Nothing

moonlitWakeCard : Card
moonlitWakeCard =
  Macros.card "Moonlit Wake" (Just [Macros.generic 2, Macros.pip White]) []
       (MkTypeLine [] [Enchantment]) [moonlitWake] Nothing

anthemOfChampions : Card
anthemOfChampions =
  Macros.card "Anthem of Champions" (Just [Macros.pip Green, Macros.pip White]) []
       (MkTypeLine [] [Enchantment])
       [Static (Gets (AllOf Macros.creatureYouControl) (PtUp (Lit 1)) (PtUp (Lit 1)))] Nothing

counterspellCard : Card
counterspellCard =
  Macros.card "Counterspell" (Just [Macros.pip Blue, Macros.pip Blue]) []
       (MkTypeLine [] [Instant]) [Spell counterspell] Nothing

hymnOfRebirthCard : Card
hymnOfRebirthCard =
  Macros.card "Hymn of Rebirth" (Just [Macros.generic 3, Macros.pip Green, Macros.pip White]) []
       (MkTypeLine [] [Sorcery]) [Spell hymnOfRebirth] Nothing

chandrasPyrohelixCard : Card
chandrasPyrohelixCard =
  Macros.card "Chandra's Pyrohelix" (Just [Macros.generic 1, Macros.pip Red]) []
       (MkTypeLine [] [Instant]) [Spell forkedBolt] Nothing

charRumbler : Card
charRumbler =
  Macros.card "Char-Rumbler" (Just [Macros.generic 2, Macros.pip Red, Macros.pip Red]) []
       (MkTypeLine [] [Creature])
       [Activated (Mana [Macros.pip Red]) (Macros.gets Macros.thisCreature (PtUp (Lit 1)) (PtUp (Lit 0)) (Just Macros.untilEndOfTurn))]
       (Just (-1, 3))

||| Sisters of Stone Death
sistersOfStoneDeathRecall : Ability
sistersOfStoneDeathRecall =
  Activated (Mana [Macros.generic 2, Macros.pip Black])
            (Macros.putOntoBattlefieldUnderYourControl
               (Macros.a (And [Macros.creature, ExiledWith Macros.thisCreature])))

||| Synod Sanctum
synodSanctumReturn : Ability
synodSanctumReturn =
  Activated (Compound [Mana [Macros.generic 2], Do (Macros.sacrifice You Macros.thisArtifact)])
            (Macros.putOntoBattlefieldUnderYourControl (AllOf Macros.exiledWithThisArtifact))

coldStorage : Card
coldStorage =
  Macros.card "Cold Storage" (Just [Macros.generic 4]) [] (MkTypeLine [] [Artifact])
       [Activated (Mana [Macros.generic 3]) (Macros.exile (Macros.target Macros.creatureYouControl)),
        Activated (Do (Macros.sacrifice You Macros.thisArtifact))
                  (Macros.putOntoBattlefieldUnderYourControl
                     (Each (And [Macros.creature, Macros.exiledWithThisArtifact])))]
       Nothing

||| Muse Vessel
museVesselPlay : Ability
museVesselPlay =
  Activated (Mana [Macros.generic 1])
            (Sequentially [Choose (Macros.a Macros.exiledWithThisArtifact),
                           Continuously (MayPlay You (That CardW)) (Just Macros.thisTurn)])

aerialVolley : Card
aerialVolley =
  Macros.card "Aerial Volley" (Just [Macros.pip Green]) [] (MkTypeLine [] [Instant])
       [Spell (Macros.dealsDivided This (Lit 3)
                            (TargetGroup (Macros.oneThrough 3)
                              (And [Macros.creature, HasKeyword Flying])))] Nothing

yotianSoldier : Card
yotianSoldier =
  Macros.card "Yotian Soldier" (Just [Macros.generic 3]) []
       (MkTypeLine [Soldier] [Artifact, Creature])
       [KeywordAbility Vigilance] (Just (1, 4))

||| Pym Particles
pymParticlesVigilanceGrant : Effect []
pymParticlesVigilanceGrant =
  Macros.gains (Macros.target Macros.creature) (KeywordAbility Vigilance) (Just Macros.untilEndOfTurn)

demonicConsultationChoice : Effect []
demonicConsultationChoice = Choose (Macros.a (QualityNoun CardName))

voidChoice : Effect []
voidChoice = Choose (Macros.a (QualityNoun Number))

basicLandCards : List Card
basicLandCards =
  [ Macros.card "Plains" Nothing [Basic] (MkTypeLine [Plains] [Land]) [] Nothing
  , Macros.card "Island" Nothing [Basic] (MkTypeLine [Island] [Land]) [] Nothing
  , Macros.card "Swamp" Nothing [Basic] (MkTypeLine [Swamp] [Land]) [] Nothing
  , Macros.card "Mountain" Nothing [Basic] (MkTypeLine [Mountain] [Land]) [] Nothing
  , Macros.card "Forest" Nothing [Basic] (MkTypeLine [Forest] [Land]) [] Nothing
  ]

snowCoveredForest : Card
snowCoveredForest =
  Macros.card "Snow-Covered Forest" Nothing [Basic, Snow]
       (MkTypeLine [Forest] [Land]) [] Nothing

bladebrand : Effect []
bladebrand =
  Macros.gains (Macros.target Macros.creature) (KeywordAbility Deathtouch) (Just Macros.untilEndOfTurn)

criticalHit : Effect []
criticalHit =
  Macros.gains (Macros.target Macros.creature) (KeywordAbility DoubleStrike) (Just Macros.untilEndOfTurn)

lightningBlow : Effect []
lightningBlow =
  Macros.gains (Macros.target Macros.creature) (KeywordAbility FirstStrike) (Just Macros.untilEndOfTurn)

avianOddity : Effect []
avianOddity = PutCounters (Lit 1) Macros.flyingCounter (Macros.target Macros.creatureYouControl)

||| Song of Eärendil
songOfEarendil : Effect []
songOfEarendil =
  PutCounters (Lit 1) Macros.flyingCounter
              (Each (And [Macros.creature, ControlledBy You, Not (HasKeyword Flying)]))


deathByDragons : Effect []
deathByDragons =
  Create (Each (And [AnyPlayer, OtherThan (Macros.target AnyPlayer)])) (Lit 1)
         (TokenWritten (MkToken (Just (Lit 5, Lit 5)) [Red] (MkTypeLine [Dragon] [Creature])
                                [KeywordAbility Flying] Nothing)) []

||| Terrifying Presence
terrifyingPresenceAnchor : Predicate [] Object
terrifyingPresenceAnchor = And [Macros.creature, OtherThan (Macros.target Macros.creature)]

lifeTotalBecomesOne : Effect []
lifeTotalBecomesOne = Macros.lifeTotalBecomes (Macros.target AnyPlayer) (Lit 1)



gideonsAvenger : Ability
gideonsAvenger =
  Triggered Whenever
            (StatusEvent (Macros.a (And [Macros.creature, ControlledBy Macros.anOpponent])) Tapped)
            (PutCounters (Lit 1) Macros.plusOnePlusOne Macros.thisCreature)

mesmericOrb : Ability
mesmericOrb =
  Triggered Whenever (StatusEvent (Macros.a Permanent) Untapped)
            (Macros.mills (ControllerOf (That PermanentW)) (Lit 1) They)


barlsCage : Ability
barlsCage = Activated (Mana [Macros.generic 3])
                      (DoesntUntapNext (Macros.target Macros.creature) (Lit 1))

takeIntoCustody : Effect []
takeIntoCustody = Sequentially [SetStatus Tapped (Macros.target Macros.creature),
                                DoesntUntapNext It (Lit 1)]

chandrasRevolution : Effect []
chandrasRevolution = Sequentially [DealDamage This (Lit 4) (Macros.target Macros.creature),
                                   SetStatus Tapped (Macros.target Macros.land),
                                   DoesntUntapNext (That (TypeW Land)) (Lit 1)]

arbalestElite : Ability
arbalestElite =
  Activated (Compound [Mana [Macros.generic 2, Macros.pip White], TapSymbol])
            (Sequentially [DealDamage Macros.thisCreature (Lit 3)
                                      (Macros.target (And [Macros.creature, Or [Attacking, Blocking]])),
                           DoesntUntapNext Macros.thisCreature (Lit 1)])


cyberConversion : Effect []
cyberConversion = SetStatus FaceDown (Macros.target Macros.creature)

breakOpen : Effect []
breakOpen = SetStatus FaceUp (Macros.target (And [Macros.creature, Macros.faceDown,
                                               ControlledBy Macros.anOpponent]))

secretPlans : Ability
secretPlans =
  Triggered Whenever
            (StatusEvent (Macros.a (And [Permanent, ControlledBy You])) FaceUp)
            Macros.drawACard

vodalianIllusionist : Ability
vodalianIllusionist =
  Activated (Compound [Mana [Macros.pip Blue, Macros.pip Blue], TapSymbol])
            (SetStatus PhasedOut (Macros.target Macros.creature))

||| Teferi's Imp
teferisImpPhasesOut : Ability
teferisImpPhasesOut =
  Triggered Whenever (StatusEvent Macros.thisCreature PhasedOut)
            (Macros.discardsACard You)

||| Teferi's Imp
teferisImpPhasesIn : Ability
teferisImpPhasesIn =
  Triggered Whenever (StatusEvent Macros.thisCreature PhasedIn)
            Macros.drawACard

shimmeringEfreet : Ability
shimmeringEfreet =
  Triggered Whenever (StatusEvent Macros.thisCreature PhasedIn)
            (SetStatus PhasedOut (Macros.target Macros.creature))


labyrinthOfSkophos : Card
labyrinthOfSkophos =
  Macros.card "Labyrinth of Skophos" Nothing [] (MkTypeLine [] [Land])
       [ Activated TapSymbol (AddMana You (Lit 1) (Runs [[Colorless]]) [])
       , Activated (Compound [Mana [Macros.generic 4], TapSymbol])
                   (RemoveFromCombat
                      (Macros.target (And [Macros.creature, Or [Attacking, Blocking]]))) ]
       Nothing

hollowhengeSpirit : Ability
hollowhengeSpirit =
  Triggered When (Enters Macros.thisCreature)
            (RemoveFromCombat (Macros.target (And [Macros.creature, Or [Attacking, Blocking]])))

netcasterSpider : Ability
netcasterSpider =
  Triggered Whenever
            (Blocks Macros.thisCreature
                    (Just (Macros.a (And [Macros.creature, HasKeyword Flying]))))
            (Macros.gets Macros.thisCreature (PtUp (Lit 2)) (PtUp (Lit 0)) (Just Macros.untilEndOfTurn))

viashinoWeaponsmith : Ability
viashinoWeaponsmith =
  Triggered Whenever
            (BecomesBlocked Macros.thisCreature (Just (Macros.a Macros.creature)))
            (Macros.gets Macros.thisCreature (PtUp (Lit 2)) (PtUp (Lit 2)) (Just Macros.untilEndOfTurn))

somberwaldAlpha : Ability
somberwaldAlpha =
  Triggered Whenever (BecomesBlocked (Macros.a Macros.creatureYouControl) Nothing)
            (Macros.gets It (PtUp (Lit 1)) (PtUp (Lit 1)) (Just Macros.untilEndOfTurn))

kjeldoranFrostbeast : Ability
kjeldoranFrostbeast =
  Triggered At (BeginningOf EndOfCombat NoPossessor)
            (Macros.destroy (AllOf (And [Macros.creature,
                                         Or [BlockerOf Macros.thisCreature,
                                             BlockedBy Macros.thisCreature]])))

vertigoSpawn : Ability
vertigoSpawn =
  Triggered Whenever (Blocks Macros.thisCreature (Just (Macros.a Macros.creature)))
            (Sequentially [SetStatus Tapped (That (TypeW Creature)),
                           DoesntUntapNext (That (TypeW Creature)) (Lit 1)])


munghaWurm : Ability
munghaWurm = Static (CantUntapMoreThan You 1 Macros.land)

dampingField : Ability
dampingField = Static (CantUntapMoreThan (PlayerGroup AllPlayers) 1 Macros.artifact)

smoke : Ability
smoke = Static (CantUntapMoreThan (PlayerGroup AllPlayers) 1 Macros.creature)

winterOrb : Ability
winterOrb =
  Static (Macros.asLongAs (Matches (AsType Artifact This) Macros.untapped)
                          (CantUntapMoreThan (PlayerGroup AllPlayers) 1 Macros.land))

staticOrb : Ability
staticOrb =
  Static (Macros.asLongAs (Matches (AsType Artifact This) Macros.untapped)
                          (CantUntapMoreThan (PlayerGroup AllPlayers) 2 Permanent))


prologueToPhyresis : Effect []
prologueToPhyresis = GetsCounters (Each Opponent) (Lit 1) Poison

screechingScorchbeast : Ability
screechingScorchbeast =
  Triggered Whenever (Attacks Macros.thisCreature)
            (GetsCounters (Each AnyPlayer) (Lit 2) Rad)

merenOfClanNelToth : Ability
merenOfClanNelToth =
  Triggered Whenever (Dies (Macros.a (Macros.otherCreatureYouControl Macros.thisCreature)))
            (GetsCounters You (Lit 1) Experience)

||| Final Act
finalActCounterMode : Effect []
finalActCounterMode = LosesAllCounters (Each Opponent) Nothing

leeches : Effect []
leeches = LosesAllCounters (Macros.target AnyPlayer) (Just Poison)

kratosStoicFather : Ability
kratosStoicFather =
  Triggered At (BeginningOf EndStep (ByWord Yours))
            (PutCounters (CountersOn Experience You) Macros.plusOnePlusOne
                         (Macros.target Macros.creature))


orneryDilophosaur : Ability
orneryDilophosaur =
  Macros.triggeredIf Whenever
                     (Attacks Macros.thisCreature)
                     (Exists (And [Macros.creature, ControlledBy You,
                                   Compare Power AtLeast (Lit 4)]))
                     (Macros.gets Macros.thisCreature (PtUp (Lit 2)) (PtUp (Lit 2)) (Just Macros.untilEndOfTurn))

incisorGlider : Ability
incisorGlider =
  Macros.triggeredIf Whenever
                     (Attacks Macros.thisCreature)
                     (CompareAmt (CountersOn Poison (Macros.a Opponent))
                                 AtLeast (Lit 3))
                     (Continuously (Gets (AllOf Macros.creatureYouControl) (PtUp (Lit 1)) (PtUp (Lit 1)))
                                   (Just Macros.untilEndOfTurn))


stormFleetSpy : Ability
stormFleetSpy =
  Macros.triggeredIf When
                     (Enters Macros.thisCreature)
                     (Happened AttackDeclaration You Lookback.ThisTurn)
                     Macros.drawACard

vashtaNerada : Ability
vashtaNerada =
  Macros.triggeredIf At
                     (BeginningOf EndStep (ByWord EachPlayers))
                     (Happened Death (Macros.a Macros.creature)
                               Lookback.ThisTurn)
                     (PutCounters (Lit 1) Macros.plusOnePlusOne Macros.thisCreature)

||| Tippy-Toe, Terrific Partner
tippyToe : Ability
tippyToe =
  Macros.triggeredIf At
                     (BeginningOf EndStep (ByWord Yours))
                     (Happened LifeGain You Lookback.ThisTurn)
                     Macros.drawACard

loanShark : Ability
loanShark =
  Macros.triggeredIf When
                     (Enters Macros.thisCreature)
                     (CompareAmt (EventCount SpellCast You Lookback.ThisTurn)
                                 AtLeast (Lit 2))
                     Macros.drawACard


sizzlingBarrage : Effect []
sizzlingBarrage =
  DealDamage This (Lit 4)
             (Macros.target (And [Macros.creature,
                                  HappenedTo BlockDeclaration Lookback.ThisTurn]))

witchsMist : Ability
witchsMist =
  Activated (Compound [Mana [Macros.generic 2, Macros.pip Black], TapSymbol])
            (Macros.destroy (Macros.target (And [Macros.creature,
                                                 HappenedTo DamageTaken Lookback.ThisTurn])))

forceOfDespair : Effect []
forceOfDespair =
  Macros.destroy (AllOf (And [Macros.creature,
                              HappenedTo Entry Lookback.ThisTurn]))

furiousSpinesplitter : Ability
furiousSpinesplitter =
  Triggered At (BeginningOf EndStep (ByWord Yours))
            (PutCounters (Macros.forEach (And [Opponent,
                                               HappenedTo DamageTaken Lookback.ThisTurn]))
                         Macros.plusOnePlusOne Macros.thisCreature)


winterMoon : Ability
winterMoon =
  Static (CantUntapMoreThan (PlayerGroup AllPlayers) 1
                            (And [Macros.land, Not (HasSupertype Basic)]))

cradleToGrave : Effect []
cradleToGrave =
  Macros.destroy (Macros.target (And [Macros.creature, Not (ColorIs Black),
                                      HappenedTo Entry Lookback.ThisTurn]))

herosDemise : Effect []
herosDemise =
  Macros.destroy (Macros.target (And [Macros.creature, HasSupertype Legendary]))

||| Pure // Simple
simpleHalf : Effect []
simpleHalf = Macros.destroy (Macros.target (And [Permanent, Multicolored]))

leitmotifComposer : Ability
leitmotifComposer =
  Activated (Mana [Macros.generic 2, Macros.pip Blue])
            (Continuously (Deontic (AllOf (And [Macros.creature,
                                                Named (PrintedName "Leitmotif Composer")]))
                                   Forbid Block Patient NoDeonticPatient)
                          (Just Macros.thisTurn))


paladinOfAtonement : Ability
paladinOfAtonement =
  Macros.triggeredIf At
                     (BeginningOf Upkeep (ByWord EachPlayers))
                     (Happened LifeLoss You Lookback.LastTurn)
                     (PutCounters (Lit 1) Macros.plusOnePlusOne Macros.thisCreature)

brazenCannonade : Ability
brazenCannonade =
  Macros.triggeredIf At
                     (BeginningOf PostcombatMain (ByWord EachYours))
                     (Happened AttackDeclaration You Lookback.ThisTurn)
                     (Macros.exile Macros.topCard)

fourKnocks : Ability
fourKnocks =
  Triggered At (BeginningOf FirstMain (ByWord Yours)) Macros.drawACard

hammerOfBogardan : Ability
hammerOfBogardan =
  Macros.activatedOnlyDuring (Mana [Macros.generic 2, Macros.pip Red, Macros.pip Red, Macros.pip Red])
                             (Move This Macros.handZ)
                             (DuringPart Upkeep (Just Yours))


berserkersOfBloodRidge : Ability
berserkersOfBloodRidge = Static (Deontic Macros.thisCreature Require Attack Agent NoDeonticPatient)

trumpetingArmodon : Ability
trumpetingArmodon =
  Activated (Mana [Macros.generic 1, Macros.pip Green])
            (Continuously (Deontic (Macros.target Macros.creature) Require Block Agent
                                   (DeonticCounterpart Macros.thisCreature))
                          (Just Macros.thisTurn))

loathsomeCatoblepas : Ability
loathsomeCatoblepas =
  Activated (Mana [Macros.generic 2, Macros.pip Green])
            (Continuously (Deontic Macros.thisCreature Require Block Patient NoDeonticPatient)
                          (Just Macros.thisTurn))

ashnodsBattleGear : Ability
ashnodsBattleGear = Static (MayDeclineUntap Macros.thisArtifact)


throneWarden : Ability
throneWarden =
  Macros.triggeredIf At
                     (BeginningOf EndStep (ByWord Yours))
                     (Matches You (HasDesignation Monarch))
                     (PutCounters (Lit 1) Macros.plusOnePlusOne Macros.thisCreature)

aragornKingOfGondor : Ability
aragornKingOfGondor =
  Triggered When (Enters Macros.thisCreature) (GainsDesignation You Monarch Instructed)

goadTargetCreature : Ability
goadTargetCreature =
  Activated (Compound [Mana [Macros.generic 3], TapSymbol])
            (GainsDesignation (Macros.target Macros.creature) Goaded Instructed)

goadedAttackTrigger : Ability
goadedAttackTrigger =
  Triggered Whenever (Attacks (Macros.a (And [Macros.creature, HasDesignation Goaded])))
            (DealDamage It (Lit 1) (ControllerOf It))

firmamentSage : Ability
firmamentSage = Triggered Whenever DayNightShift Macros.drawACard


deeprootWarrior : Ability
deeprootWarrior =
  Triggered Whenever (BecomesBlocked Macros.thisCreature Nothing)
            (Macros.gets It (PtUp (Lit 1)) (PtUp (Lit 1)) (Just Macros.untilEndOfTurn))

borderlandMarauder : Ability
borderlandMarauder =
  Triggered Whenever (Attacks Macros.thisCreature)
            (Macros.gets It (PtUp (Lit 2)) (PtUp (Lit 0)) (Just Macros.untilEndOfTurn))



hipparion : Ability
hipparion =
  Static (Deontic Macros.thisCreature (GatedBy (Mana [Macros.generic 1]))
                  Block Agent
                  (DeonticCounterpart (AllOf (And [Macros.creature,
                                                   Compare Power AtLeast (Lit 3)]))))


frodoBaggins : Ability
frodoBaggins =
  Static (Macros.asLongAs (Matches Macros.thisCreature (HasDesignation RingBearer))
                          (Deontic It Require Block Patient NoDeonticPatient))

adantoVanguard : Ability
adantoVanguard =
  Static (Macros.asLongAs (Matches Macros.thisCreature Attacking)
                          (Gets It (PtUp (Lit 2)) (PtUp (Lit 0))))


bloodshedFever : Card
bloodshedFever =
  Macros.card "Bloodshed Fever" (Just [Macros.pip Red]) []
       (MkTypeLine [Aura] [Enchantment])
       [ Macros.keywordSubject Enchant Macros.creature
       , Static (Deontic (AttachHost Enchanted (TypeW Creature))
                         Require Attack Agent NoDeonticPatient) ]
       Nothing

brainwash : Card
brainwash =
  Macros.card "Brainwash" (Just [Macros.pip White]) []
       (MkTypeLine [Aura] [Enchantment])
       [ Macros.keywordSubject Enchant Macros.creature
       , Static (Deontic (AttachHost Enchanted (TypeW Creature))
                         (GatedBy (Mana [Macros.generic 3])) Attack Agent NoDeonticPatient) ]
       Nothing

enkiraHostileScavenger : Ability
enkiraHostileScavenger =
  Static (Macros.asLongAs (Matches Macros.thisCreature (IsAttached Equipped))
                          (Deontic It Require Block Patient NoDeonticPatient))

extraArms : Card
extraArms =
  Macros.card "Extra Arms" (Just [Macros.generic 4, Macros.pip Red]) []
       (MkTypeLine [Aura] [Enchantment])
       [ Macros.keywordSubject Enchant Macros.creature
       , Triggered Whenever (Attacks (AttachHost Enchanted (TypeW Creature)))
                   (DealDamage It (Lit 2) (Macros.target AnyTarget)) ]
       Nothing


||| Lich's Mastery
lichsMasteryGate : Ability
lichsMasteryGate = Static (OutcomeGate CantLose You)

||| When Lich's Mastery
lichsMasteryLoss : Ability
lichsMasteryLoss =
  Triggered When (Leaves Macros.thisEnchantment) (Concludes LoseGame You)

phageTheUntouchable : Ability
phageTheUntouchable =
  Triggered Whenever
            (DealsCombatDamage Macros.thisCreature (Macros.a AnyPlayer))
            (Concludes LoseGame (That PlayerW))


platinumAngel : Card
platinumAngel =
  Macros.card "Platinum Angel" (Just [Macros.generic 7]) []
       (MkTypeLine [Angel] [Artifact, Creature])
       [ KeywordAbility Flying
       , Static (OutcomeGate CantLose You)
       , Static (OutcomeGate CantWin (PlayerGroup YourOpponents)) ]
       (Just (4, 4))

abyssalPersecutor : Card
abyssalPersecutor =
  Macros.card "Abyssal Persecutor"
       (Just [Macros.generic 2, Macros.pip Black, Macros.pip Black]) []
       (MkTypeLine [Demon] [Creature])
       [ KeywordAbility Flying
       , KeywordAbility Trample
       , Static (OutcomeGate CantWin You)
       , Static (OutcomeGate CantLose (PlayerGroup YourOpponents)) ]
       (Just (6, 6))


smogElemental : Card
smogElemental =
  Macros.card "Smog Elemental"
       (Just [Macros.generic 4, Macros.pip Black, Macros.pip Black]) []
       (MkTypeLine [Elemental] [Creature])
       [ KeywordAbility Flying
       , Static (Gets (AllOf (And [Macros.creature, HasKeyword Flying,
                                   ControlledBy (PlayerGroup YourOpponents)]))
                      (PtDown (Lit 1)) (PtDown (Lit 1))) ]
       (Just (3, 3))

anglerTurtle : Ability
anglerTurtle =
  Static (Deontic (AllOf Macros.creatureYourOpponentsControl) Require Attack Agent NoDeonticPatient)


bloodTyrant : Ability
bloodTyrant =
  Triggered Whenever (LosesGame (Macros.a AnyPlayer))
            (PutCounters (Lit 5) Macros.plusOnePlusOne Macros.thisCreature)

theGoldenThrone : Ability
theGoldenThrone =
  Static (Intercepts (LosesGame You)
                     (Sequentially [Macros.exile Macros.thisArtifact,
                                    ChangeLife You (Set (Lit 1))])
                     Repeatedly)

stunningReversal : Ability
stunningReversal =
  Spell (Continuously (Intercepts (LosesGame You)
                                  (Sequentially [Draw You (Lit 7),
                                                 ChangeLife You (Set (Lit 1))])
                                  NextTimeOnly)
                      (Just Macros.thisTurn))


felidarSovereign : Card
felidarSovereign =
  Macros.card "Felidar Sovereign"
       (Just [Macros.generic 4, Macros.pip White, Macros.pip White]) []
       (MkTypeLine [Cat, Beast] [Creature])
       [ KeywordAbility Vigilance
       , KeywordAbility Lifelink
       , Macros.triggeredIf At
                            (BeginningOf Upkeep (ByWord Yours))
                            (CompareAmt (PlayerStatOf LifeTotal You)
                                        AtLeast (Lit 40))
                            (Concludes WinGame You) ]
       (Just (4, 6))

exquisiteArchangel : Card
exquisiteArchangel =
  Macros.card "Exquisite Archangel"
       (Just [Macros.generic 5, Macros.pip White, Macros.pip White]) []
       (MkTypeLine [Angel] [Creature])
       [ KeywordAbility Flying
       , Static (Intercepts (LosesGame You)
                            (Sequentially [Macros.exile Macros.thisCreature,
                                           ChangeLife You (Set (PlayerStatOf StartingLifeTotal You))])
                            Repeatedly) ]
       (Just (5, 5))

spaceTimeAnomaly : Card
spaceTimeAnomaly =
  Macros.card "Space-Time Anomaly"
       (Just [Macros.generic 2, Macros.pip White, Macros.pip Blue]) []
       (MkTypeLine [] [Sorcery])
       [ Spell (Macros.mills (Macros.target AnyPlayer) (PlayerStatOf LifeTotal You) They) ]
       Nothing


timelyReinforcements : Card
timelyReinforcements =
  Macros.card "Timely Reinforcements"
       (Just [Macros.generic 2, Macros.pip White]) []
       (MkTypeLine [] [Sorcery])
       [ Spell (Sequentially
                  [ If (Macros.gainsLife You (Lit 6))
                       (CompareAmt (PlayerStatOf LifeTotal You) Less
                                   (PlayerStatOf LifeTotal Macros.anOpponent))
                       Nothing
                  , If (Macros.create (Lit 3) (Macros.creatureTok 1 1 [White] [Soldier]))
                       (CompareAmt (CountOf Macros.creatureYouControl) Less
                                   (CountOf (And [Macros.creature,
                                                  ControlledBy Macros.anOpponent])))
                       Nothing ]) ]
       Nothing

survivalCache : Effect []
survivalCache =
  Sequentially [ Macros.gainsLife You (Lit 2)
               , If Macros.drawACard
                    (CompareAmt (PlayerStatOf LifeTotal You) Greater
                                (PlayerStatOf LifeTotal Macros.anOpponent))
                    Nothing ]

pathOfBravery : Ability
pathOfBravery =
  Static (Macros.asLongAs (CompareAmt (PlayerStatOf LifeTotal You) AtLeast
                                      (PlayerStatOf StartingLifeTotal You))
                          (Gets (AllOf Macros.creatureYouControl) (PtUp (Lit 1)) (PtUp (Lit 1))))

ensnaringBridge : Card
ensnaringBridge =
  Macros.card "Ensnaring Bridge" (Just [Macros.generic 3]) []
       (MkTypeLine [] [Artifact])
       [ Static (Deontic (AllOf (And [Macros.creature,
                                      Compare Power Greater
                                              (CountOf (InZone (Macros.handOf You)))]))
                         Forbid Attack Agent NoDeonticPatient) ]
       Nothing

elderscaleWurm : Ability
elderscaleWurm =
  Macros.triggeredIf When
                     (Enters Macros.thisCreature)
                     (CompareAmt (PlayerStatOf LifeTotal You) Less (Lit 7))
                     (ChangeLife You (Set (Lit 7)))

gloriousEnforcer : Card
gloriousEnforcer =
  Macros.card "Glorious Enforcer"
       (Just [Macros.generic 5, Macros.pip White, Macros.pip White]) []
       (MkTypeLine [Angel] [Creature])
       [ KeywordAbility Flying
       , KeywordAbility Lifelink
       , Macros.triggeredIf At
                            (BeginningOf Combat (ByWord EachPlayers))
                            (CompareAmt (PlayerStatOf LifeTotal You) Greater
                                        (PlayerStatOf LifeTotal Macros.anOpponent))
                            (Macros.gains Macros.thisCreature (KeywordAbility DoubleStrike)
                                          (Just Macros.untilEndOfTurn)) ]
       (Just (5, 5))


||| Damia, Sage of Stone
damia : Ability
damia =
  Macros.triggeredIf At
                     (BeginningOf Upkeep (ByWord Yours))
                     (CompareAmt (CountOf (InZone (Macros.handOf You)))
                                 Less (Lit 7))
                     (Draw You TheDifference)

||| Krang, Master Mind
krang : Ability
krang =
  Macros.triggeredIf When
                     (Enters Macros.thisCreature)
                     (CompareAmt (CountOf (InZone (Macros.handOf You)))
                                 Less (Lit 4))
                     (Draw You TheDifference)


||| Krenko, Mob Boss
krenko : Ability
krenko =
  Activated TapSymbol
            (WhereLetter LetterX (CountOf (And [HasSubtype Goblin, ControlledBy You]))
                         (Create You (DefinedLetter LetterX)
                            (TokenWritten (Macros.creatureTok 1 1 [Red] [Goblin])) []))

chainReaction : Card
chainReaction =
  Macros.card "Chain Reaction"
       (Just [Macros.generic 2, Macros.pip Red, Macros.pip Red]) []
       (MkTypeLine [] [Sorcery])
       [ Spell (WhereLetter LetterX (CountOf Macros.creature)
                            (DealDamage This (DefinedLetter LetterX)
                                        (Each Macros.creature))) ]
       Nothing

ivoryTower : Card
ivoryTower =
  Macros.card "Ivory Tower" (Just [Macros.generic 1]) []
       (MkTypeLine [] [Artifact])
       [ Triggered At (BeginningOf Upkeep (ByWord Yours))
                  (WhereLetter LetterX
                               (Minus (CountOf (InZone (Macros.handOf You))) (Lit 4))
                               (Macros.gainsLife You (DefinedLetter LetterX))) ]
       Nothing

harshSustenance : Card
harshSustenance =
  Macros.card "Harsh Sustenance"
       (Just [Macros.generic 1, Macros.pip White, Macros.pip Black]) []
       (MkTypeLine [] [Sorcery])
       [ Spell (WhereLetter LetterX (CountOf Macros.creatureYouControl)
                            (Sequentially
                              [DealDamage This (DefinedLetter LetterX)
                                          (Macros.target AnyTarget),
                               Macros.gainsLife You (DefinedLetter LetterX)])) ]
       Nothing


nightmarishEnd : Card
nightmarishEnd =
  Macros.card "Nightmarish End"
       (Just [Macros.generic 2, Macros.pip Black]) []
       (MkTypeLine [] [Instant])
       [ Spell (WhereLetter LetterX (CountOf (InZone (Macros.handOf You)))
                            (Macros.gets (Macros.target Macros.creature)
                                         (PtDown (DefinedLetter LetterX))
                                         (PtDown (DefinedLetter LetterX))
                                         (Just Macros.untilEndOfTurn))) ]
       Nothing

adelbertSteiner : Card
adelbertSteiner =
  Macros.card "Adelbert Steiner" (Just [Macros.generic 1, Macros.pip White])
       [Legendary] (MkTypeLine [Human, Knight] [Creature])
       [ KeywordAbility Lifelink
       , Static (Gets Macros.thisCreature
               (PtUp (Macros.forEach (And [HasSubtype Equipment, ControlledBy You])))
               (PtUp (Macros.forEach (And [HasSubtype Equipment, ControlledBy You])))) ]
       (Just (2, 1))

||| Dokai, Weaver of Life
dokai : Ability
dokai =
  Activated (Compound [Mana [Macros.generic 4, Macros.pip Green, Macros.pip Green],
                       TapSymbol])
            (WhereLetter LetterX (CountOf (And [Macros.land, ControlledBy You]))
                         (Macros.create (Lit 1)
                            (Macros.creatureTokOf (DefinedLetter LetterX)
                                                  (DefinedLetter LetterX)
                                                  [Green] [Elemental])))


deathsShadow : Card
deathsShadow =
  Macros.card "Death's Shadow" (Just [Macros.pip Black]) []
       (MkTypeLine [Avatar] [Creature])
       [ Static (WhereLetterStatic LetterX (PlayerStatOf LifeTotal You)
                                   (Gets Macros.thisCreature
                                         (PtDown (DefinedLetter LetterX))
                                         (PtDown (DefinedLetter LetterX)))) ]
       (Just (13, 13))

spontaneousMutation : Ability
spontaneousMutation =
  Static (WhereLetterStatic LetterX (CountOf (InZone (Macros.graveyardOf You)))
                            (Gets (AttachHost Enchanted (TypeW Creature))
                                  (PtDown (DefinedLetter LetterX))
                                  (PtDown (Lit 0))))

stagBeetle : Card
stagBeetle =
  Macros.card "Stag Beetle"
       (Just [Macros.generic 3, Macros.pip Green, Macros.pip Green]) []
       (MkTypeLine [Insect] [Creature])
       [ Static (WhereLetterStatic LetterX
                   (CountOf (Macros.otherCreature Macros.thisCreature))
                   (EntersWithCounters Macros.thisCreature
                                       (DefinedLetter LetterX)
                                       Macros.plusOnePlusOne)) ]
       (Just (0, 0))


acceleratedMutation : Card
acceleratedMutation =
  Macros.card "Accelerated Mutation"
       (Just [Macros.generic 3, Macros.pip Green, Macros.pip Green]) []
       (MkTypeLine [] [Instant])
       [ Spell (WhereLetter LetterX
                  (Aggregate MaxOf (CharAxis ManaValue)
                             (And [Permanent, ControlledBy You]))
                  (Continuously (Gets (Macros.target Macros.creature)
                                      (PtUp (DefinedLetter LetterX))
                                      (PtUp (DefinedLetter LetterX)))
                                (Just Macros.untilEndOfTurn))) ]
       Nothing

carrionGrub : Card
carrionGrub =
  Macros.card "Carrion Grub" (Just [Macros.generic 3, Macros.pip Black]) []
       (MkTypeLine [Insect] [Creature])
       [ Static (WhereLetterStatic LetterX
                   (Aggregate MaxOf (CharAxis Power)
                              (And [Macros.creature,
                                    InZone (Macros.graveyardOf You)]))
                   (Gets Macros.thisCreature
                         (PtUp (DefinedLetter LetterX))
                         (PtUp (Lit 0))))
       , Triggered When (Enters Macros.thisCreature)
                        (Macros.mills You (Lit 4) You) ]
       (Just (0, 5))

repayInKind : Card
repayInKind =
  Macros.card "Repay in Kind"
       (Just [Macros.generic 5, Macros.pip Black, Macros.pip Black]) []
       (MkTypeLine [] [Sorcery])
       [ Spell (Macros.lifeTotalBecomes (Each AnyPlayer)
                  (Aggregate MinOf (PlayerStatAxis LifeTotal) AnyPlayer)) ]
       Nothing

toweringTitan : Ability
toweringTitan =
  Static (WhereLetterStatic LetterX
            (Aggregate SumOf (CharAxis Toughness)
                       (Macros.otherCreatureYouControl Macros.thisCreature))
            (EntersWithCounters Macros.thisCreature
                                (DefinedLetter LetterX)
                                Macros.plusOnePlusOne))


ghalta : Card
ghalta =
  Macros.card "Ghalta, Primal Hunger"
       (Just [Macros.generic 10, Macros.pip Green, Macros.pip Green]) [Legendary]
       (MkTypeLine [Elder, Dinosaur] [Creature])
       [ Static (WhereLetterStatic LetterX
                   (Aggregate SumOf (CharAxis Power) Macros.creatureYouControl)
                   (CostsToCast This (CostLess (DefinedLetter LetterX))))
       , KeywordAbility Trample ]
       (Just (12, 12))

ancientStoneIdol : Ability
ancientStoneIdol =
  Static (CostsToCast This
            (CostLess (Macros.forEach (And [Macros.creature, Attacking]))))

thornOfAmethyst : Card
thornOfAmethyst =
  Macros.card "Thorn of Amethyst" (Just [Macros.generic 2]) []
       (MkTypeLine [] [Artifact])
       [ Static (CostsToCast (AllOf (And [Not Macros.creature, Macros.spell]))
                             (CostMore (Lit 1))) ]
       Nothing

ferozsBan : Card
ferozsBan =
  Macros.card "Feroz's Ban" (Just [Macros.generic 6]) []
       (MkTypeLine [] [Artifact])
       [ Static (CostsToCast (AllOf (And [Macros.creature, Macros.spell]))
                             (CostMore (Lit 2))) ]
       Nothing

urzasFilter : Card
urzasFilter =
  Macros.card "Urza's Filter" (Just [Macros.generic 4]) []
       (MkTypeLine [] [Artifact])
       [ Static (CostsToCast (AllOf (And [Multicolored, Macros.spell]))
                             (CostLess (Lit 2))) ]
       Nothing


emeraldMedallion : Card
emeraldMedallion =
  Macros.card "Emerald Medallion" (Just [Macros.generic 2]) []
       (MkTypeLine [] [Artifact])
       [ Static (CostsToCast (AllOf (And [ColorIs Green, Macros.spell, CastBy You]))
                             (CostLess (Lit 1))) ]
       Nothing

foundryInspector : Card
foundryInspector =
  Macros.card "Foundry Inspector" (Just [Macros.generic 3]) []
       (MkTypeLine [Construct] [Artifact, Creature])
       [ Static (CostsToCast (AllOf (And [Macros.artifact, Macros.spell, CastBy You]))
                             (CostLess (Lit 1))) ]
       (Just (3, 2))

daruWarchief : Card
daruWarchief =
  Macros.card "Daru Warchief"
       (Just [Macros.generic 2, Macros.pip White, Macros.pip White]) []
       (MkTypeLine [Human, Soldier] [Creature])
       [ Static (CostsToCast (AllOf (And [HasSubtype Soldier, Macros.spell, CastBy You]))
                             (CostLess (Lit 1)))
       , Static (Gets (AllOf (And [HasSubtype Soldier, Macros.creature, ControlledBy You]))
                      (PtUp (Lit 1)) (PtUp (Lit 2))) ]
       (Just (1, 1))

grandArbiter : Card
grandArbiter =
  Macros.card "Grand Arbiter Augustin IV"
       (Just [Macros.generic 2, Macros.pip White, Macros.pip Blue]) [Legendary]
       (MkTypeLine [Human, Advisor] [Creature])
       [ Static (CostsToCast (AllOf (And [ColorIs White, Macros.spell, CastBy You]))
                             (CostLess (Lit 1)))
       , Static (CostsToCast (AllOf (And [ColorIs Blue, Macros.spell, CastBy You]))
                             (CostLess (Lit 1)))
       , Static (CostsToCast (AllOf (And [Macros.spell,
                                          CastBy (PlayerGroup YourOpponents)]))
                             (CostMore (Lit 1))) ]
       (Just (2, 3))


goblinElectromancer : Card
goblinElectromancer =
  Macros.card "Goblin Electromancer"
       (Just [Macros.pip Blue, Macros.pip Red]) []
       (MkTypeLine [Goblin, Wizard] [Creature])
       [ Static (CostsToCast (AllOf (And [Macros.instantOrSorcery, Macros.spell,
                                          CastBy You]))
                             (CostLess (Lit 1))) ]
       (Just (2, 2))

arcaneMelee : Card
arcaneMelee =
  Macros.card "Arcane Melee" (Just [Macros.generic 4, Macros.pip Blue]) []
       (MkTypeLine [] [Enchantment])
       [ Static (CostsToCast (AllOf (And [Macros.instantOrSorcery, Macros.spell]))
                             (CostLess (Lit 2))) ]
       Nothing

manaMatrix : Card
manaMatrix =
  Macros.card "Mana Matrix" (Just [Macros.generic 6]) []
       (MkTypeLine [] [Artifact])
       [ Static (CostsToCast (AllOf (And [Or [Macros.instant, Macros.enchantment],
                                          Macros.spell, CastBy You]))
                             (CostLess (Lit 2))) ]
       Nothing

auraOfSilence : Card
auraOfSilence =
  Macros.card "Aura of Silence"
       (Just [Macros.generic 1, Macros.pip White, Macros.pip White]) []
       (MkTypeLine [] [Enchantment])
       [ Static (CostsToCast (AllOf (And [Or [Macros.artifact, Macros.enchantment],
                                          Macros.spell,
                                          CastBy (PlayerGroup YourOpponents)]))
                             (CostMore (Lit 2)))
       , Activated (Do (Macros.sacrifice You Macros.thisEnchantment))
                   (Macros.destroy (Macros.target (Or [Macros.artifact,
                                                       Macros.enchantment]))) ]
       Nothing

youngPyromancer : Card
youngPyromancer =
  Macros.card "Young Pyromancer" (Just [Macros.generic 1, Macros.pip Red]) []
       (MkTypeLine [Human, Shaman] [Creature])
       [ Triggered Whenever
                   (Casts You (Macros.a (And [Macros.instantOrSorcery, Macros.spell])))
                   (Create You (Lit 1)
                            (TokenWritten (Macros.creatureTok 1 1 [Red] [Elemental])) []) ]
       (Just (2, 1))


inspiringStatuary : Card
inspiringStatuary =
  Macros.card "Inspiring Statuary" (Just [Macros.generic 3]) []
       (MkTypeLine [] [Artifact])
       [ Static (Gains (AllOf (And [Not Macros.artifact, Macros.spell, CastBy You]))
                       (KeywordAbility Improvise)) ]
       Nothing

chiefEngineer : Card
chiefEngineer =
  Macros.card "Chief Engineer" (Just [Macros.generic 1, Macros.pip Blue]) []
       (MkTypeLine [Vedalken, Artificer] [Creature])
       [ Static (Gains (AllOf (And [Macros.artifact, Macros.spell, CastBy You]))
                       (KeywordAbility Convoke)) ]
       (Just (1, 3))

firesongAndSunspeaker : Ability
firesongAndSunspeaker =
  Static (Gains (AllOf (And [ColorIs Red, Macros.instantOrSorcery, Macros.spell,
                             ControlledBy You]))
                (KeywordAbility Lifelink))

prismariTheInspiration : Card
prismariTheInspiration =
  Macros.card "Prismari, the Inspiration"
       (Just [Macros.generic 5, Macros.pip Blue, Macros.pip Red]) [Legendary]
       (MkTypeLine [Elder, Dragon] [Creature])
       [ KeywordAbility Flying
       , Macros.keywordCosting Ward (Macros.payLife You 5)
       , Static (Gains (AllOf (And [Macros.instantOrSorcery, Macros.spell, CastBy You]))
                       (KeywordAbility Storm)) ]
       (Just (7, 7))


tomakulHonorGuard : Card
tomakulHonorGuard =
  Macros.card "Tomakul Honor Guard" (Just [Macros.generic 1, Macros.pip Green]) []
       (MkTypeLine [Human, Soldier] [Creature])
       [ Macros.keywordCosting Ward (Mana [Macros.generic 2]) ]
       (Just (3, 1))

repentantBlacksmith : Card
repentantBlacksmith =
  Macros.card "Repentant Blacksmith" (Just [Macros.generic 1, Macros.pip White]) []
       (MkTypeLine [Human] [Creature])
       [ Macros.keywordQuality Protection (ColorIs Red) ]
       (Just (1, 2))


maro : Card
maro =
  Macros.cardOf "Maro" (Just [Macros.generic 2, Macros.pip Green, Macros.pip Green]) []
       (MkTypeLine [Elemental] [Creature])
       [ Static (DefinesPt Macros.thisCreature BothEach
                           (CountOf (InZone (Macros.handOf You)))) ]
       (Just (PrintedStar, PrintedStar))

battleSquadron : Card
battleSquadron =
  Macros.cardOf "Battle Squadron" (Just [Macros.generic 3, Macros.pip Red, Macros.pip Red]) []
       (MkTypeLine [Goblin] [Creature])
       [ KeywordAbility Flying
       , Static (DefinesPt Macros.thisCreature BothEach
                           (CountOf Macros.creatureYouControl)) ]
       (Just (PrintedStar, PrintedStar))

peopleOfTheWoods : Card
peopleOfTheWoods =
  Macros.cardOf "People of the Woods" (Just [Macros.pip Green, Macros.pip Green]) []
       (MkTypeLine [Human] [Creature])
       [ Static (DefinesPt Macros.thisCreature ToughnessAlone
                           (CountOf (And [HasSubtype Forest, ControlledBy You]))) ]
       (Just (PrintedNum 1, PrintedStar))

scourgeOfTheSkyclaves : Ability
scourgeOfTheSkyclaves =
  Static (DefinesPt Macros.thisCreature BothEach
                    (Minus (Lit 20) (Aggregate MaxOf (PlayerStatAxis LifeTotal) AnyPlayer)))

aettirAndPriwen : Ability
aettirAndPriwen =
  Static (WhereLetterStatic LetterX (PlayerStatOf LifeTotal You)
                            (HasBasePt (AttachHost Equipped (TypeW Creature))
                                       (DefinedLetter LetterX)
                                       (DefinedLetter LetterX)))

diminish : Card
diminish =
  Macros.card "Diminish" (Just [Macros.pip Blue]) []
       (MkTypeLine [] [Instant])
       [ Spell (Continuously (HasBasePt (Macros.target Macros.creature) (Lit 1) (Lit 1))
                             (Just Macros.untilEndOfTurn)) ]
       Nothing

aboutFace : Card
aboutFace =
  Macros.card "About Face" (Just [Macros.pip Red]) []
       (MkTypeLine [] [Instant])
       [ Spell (Continuously (SwitchesPt (Macros.target Macros.creature))
                             (Just Macros.untilEndOfTurn)) ]
       Nothing


topple : Card
topple =
  Macros.card "Topple" (Just [Macros.generic 2, Macros.pip White]) []
       (MkTypeLine [] [Sorcery])
       [ Spell (Macros.exile
                  (Macros.target
                     (And [Macros.creature,
                           Superlative MaxOf (CharAxis Power)
                                       (And [Macros.creature,
                                             InZone Macros.battlefieldZ])]))) ]
       Nothing

cullingScales : Card
cullingScales =
  Macros.card "Culling Scales" (Just [Macros.generic 3]) []
       (MkTypeLine [] [Artifact])
       [ Triggered At (BeginningOf Upkeep (ByWord Yours))
                   (Macros.destroy
                      (Macros.target
                         (And [Permanent, Not Macros.land,
                               Superlative MinOf (CharAxis ManaValue)
                                           (And [Permanent, Not Macros.land])]))) ]
       Nothing

purgingScythe : Ability
purgingScythe =
  Triggered At (BeginningOf Upkeep (ByWord Yours))
            (DealDamage Macros.thisArtifact (Lit 2)
                        (Definite (And [Macros.creature,
                                        Superlative MinOf (CharAxis Toughness)
                                                    Macros.creature])))

roilingHorror : Ability
roilingHorror =
  Static (DefinesPt Macros.thisCreature BothEach
                    (Minus (PlayerStatOf LifeTotal You)
                           (PlayerStatOf LifeTotal
                              (Macros.a (And [Opponent,
                                              Superlative MaxOf
                                                (PlayerStatAxis LifeTotal)
                                                Opponent])))))

eomerOfTheRiddermark : Card
eomerOfTheRiddermark =
  Macros.card "Éomer of the Riddermark" (Just [Macros.generic 4, Macros.pip Red])
       [Legendary]
       (MkTypeLine [Human, Knight] [Creature])
       [ KeywordAbility Haste
       , Macros.triggeredIf Whenever
                            (Attacks Macros.thisCreature)
                            (Exists (And [Macros.creature, ControlledBy You,
                                          Superlative MaxOf (CharAxis Power)
                                            (And [Macros.creature,
                                                  InZone Macros.battlefieldZ])]))
                            (Macros.create (Lit 1) (Macros.creatureTok 1 1 [White] [Human, Soldier])) ]
       (Just (5, 4))

||| Consecrate // Consume
consume : Ability
consume =
  Spell (Sequentially
           [ Macros.sacrifice (Macros.target AnyPlayer)
               (Macros.a (And [Macros.creature,
                               Superlative MaxOf (CharAxis Power)
                                 (And [Macros.creature, ControlledBy They])]))
           , Macros.gainsLife You (StatOf Power It) ])

coretapper : Card
coretapper =
  Macros.card "Coretapper" (Just [Macros.generic 2]) []
       (MkTypeLine [Myr] [Artifact, Creature])
       [ Activated TapSymbol
                   (PutCounters (Lit 1) Charge (Macros.target Macros.artifact))
       , Activated (Do (Macros.sacrifice You Macros.thisCreature))
                   (PutCounters (Lit 2) Charge (Macros.target Macros.artifact)) ]
       (Just (1, 1))

divineIntervention : Card
divineIntervention =
  Macros.card "Divine Intervention"
       (Just [Macros.generic 6, Macros.pip White, Macros.pip White]) []
       (MkTypeLine [] [Enchantment])
       [ Static (EntersWithCounters Macros.thisEnchantment (Lit 2) Intervention)
       , Triggered At (BeginningOf Upkeep (ByWord Yours))
                   (RemoveCounters (Lit 1) Intervention Macros.thisEnchantment)
       , Triggered When (Macros.lastCounterRemovedBy Intervention Macros.thisEnchantment You)
                   GameDrawn ]
       Nothing

celestialConvergence : Card
celestialConvergence =
  Macros.card "Celestial Convergence"
       (Just [Macros.generic 2, Macros.pip White, Macros.pip White]) []
       (MkTypeLine [] [Enchantment])
       [ Static (EntersWithCounters Macros.thisEnchantment (Lit 7) Omen)
       , Triggered At (BeginningOf Upkeep (ByWord Yours))
           (Sequentially
              [ RemoveCounters (Lit 1) Omen Macros.thisEnchantment
              , If (Concludes WinGame
                      (Definite (And [AnyPlayer,
                                      Superlative MaxOf (PlayerStatAxis LifeTotal)
                                                  AnyPlayer])))
                   (CompareAmt (CountersOn Omen Macros.thisEnchantment)
                               AtMost (Lit 0))
                   Nothing
              , If GameDrawn
                   (CompareAmt (CountOf (And [AnyPlayer,
                                              Superlative MaxOf
                                                (PlayerStatAxis LifeTotal)
                                                AnyPlayer]))
                               AtLeast (Lit 2))
                   Nothing ]) ]
       Nothing


angelicGift : Card
angelicGift =
  Macros.card "Angelic Gift" (Just [Macros.generic 1, Macros.pip White]) []
       (MkTypeLine [Aura] [Enchantment])
       [ Macros.keywordSubject Enchant Macros.creature
       , Triggered When (Enters Macros.thisAura) (Draw You (Lit 1))
       , Static (Gains (AttachHost Enchanted (TypeW Creature))
                       (KeywordAbility Flying)) ]
       Nothing

lionHeart : Card
lionHeart =
  Macros.card "Lion Heart" (Just [Macros.generic 4]) []
       (MkTypeLine [Equipment] [Artifact])
       [ Triggered When (Enters Macros.thisEquipment)
                   (DealDamage It (Lit 2) (Macros.target AnyTarget))
       , Static (Gets (AttachHost Equipped (TypeW Creature))
                      (PtUp (Lit 2)) (PtUp (Lit 1)))
       , Macros.keywordCosting Equip (Mana [Macros.generic 2]) ]
       Nothing

curseOfVengeance : Card
curseOfVengeance =
  Macros.card "Curse of Vengeance" (Just [Macros.pip Black]) []
       (MkTypeLine [Aura, Curse] [Enchantment])
       [ Macros.keywordSubject Enchant AnyPlayer
       , Triggered Whenever
                   (Casts (AttachHost Enchanted PlayerW) (Macros.a Macros.spell))
                   (PutCounters (Lit 1) Spite Macros.thisAura)
       , Triggered When (LosesGame (AttachHost Enchanted PlayerW))
                   (WhereLetter LetterX (CountersOn Spite Macros.thisAura)
                      (Sequentially
                         [ ChangeLife You (Up (DefinedLetter LetterX))
                         , Draw You (DefinedLetter LetterX) ])) ]
       Nothing


consulsLieutenant : Card
consulsLieutenant =
  Macros.card "Consul's Lieutenant" (Just [Macros.pip White, Macros.pip White]) []
       (MkTypeLine [Human, Soldier] [Creature])
       [ KeywordAbility FirstStrike
       , Macros.keywordNumber Renown (Lit 1)
       , Macros.triggeredIf Whenever
                            (Attacks Macros.thisCreature)
                            (Matches Macros.thisCreature
                                     (HasDesignation Renowned))
                            (Macros.gets (AllOf (And [Macros.creature, Attacking,
                                                      ControlledBy You,
                                                      OtherThan Macros.thisCreature]))
                                         (PtUp (Lit 1)) (PtUp (Lit 1))
                                         (Just Macros.untilEndOfTurn)) ]
       (Just (2, 1))

secretsOfTheGoldenCity : Card
secretsOfTheGoldenCity =
  Macros.card "Secrets of the Golden City"
       (Just [Macros.generic 1, Macros.pip Blue, Macros.pip Blue]) []
       (MkTypeLine [] [Sorcery])
       [ KeywordAbility Ascend
       , Spell (InsteadOf (Draw You (Lit 2))
                          (If (Draw You (Lit 3))
                              (Matches You (HasDesignation CitysBlessing))
                              Nothing)) ]
       Nothing

bombur : Card
bombur =
  Macros.card "Bombur, Gentle Dreamer"
       (Just [Macros.generic 2, Macros.pip Red]) [Legendary]
       (MkTypeLine [Dwarf, Bard] [Creature])
       [ KeywordAbility Storied
       , Static (Macros.unlessSo (Matches You (HasDesignation EnduringStory))
                                 (DoesntUntap Macros.thisCreature)) ]
       (Just (5, 3))


passagewaySeer : Card
passagewaySeer =
  Macros.card "Passageway Seer" (Just [Macros.generic 3, Macros.pip Black]) []
       (MkTypeLine [Tiefling, Warlock] [Creature])
       [ KeywordAbility Lifelink
       , Triggered When (Enters Macros.thisCreature)
                   (GainsDesignation You TheInitiative Instructed)
       , Macros.triggeredIf At
                            (BeginningOf EndStep (ByWord Yours))
                            (Matches You (HasDesignation TheInitiative))
                            (PutCounters (Lit 1) Macros.plusOnePlusOne Macros.thisCreature) ]
       (Just (2, 2))

deadeyeBrawler : Card
deadeyeBrawler =
  Macros.card "Deadeye Brawler"
       (Just [Macros.generic 2, Macros.pip Blue, Macros.pip Black]) []
       (MkTypeLine [Human, Pirate] [Creature])
       [ KeywordAbility Deathtouch
       , KeywordAbility Ascend
       , Macros.triggeredIf Whenever
                            (DealsCombatDamage Macros.thisCreature (Macros.a AnyPlayer))
                            (Matches You (HasDesignation CitysBlessing))
                            Macros.drawACard ]
       (Just (2, 4))

chillerpillar : Card
chillerpillar =
  Macros.card "Chillerpillar" (Just [Macros.generic 3, Macros.pip Blue]) [Snow]
       (MkTypeLine [Insect] [Creature])
       [ Activated (Mana [Macros.generic 4, SnowMana, SnowMana])
                   (Macros.monstrosity (Lit 2))
       , Static (Macros.asLongAs (Matches Macros.thisCreature (HasDesignation Monstrous))
                                 (Gains It (KeywordAbility Flying))) ]
       (Just (3, 3))

archivistOfGondor : Ability
archivistOfGondor =
  Macros.triggeredIf When
                     (DealsCombatDamage Macros.yourCommander (Macros.a AnyPlayer))
                     (Macros.thereIsNo Monarch)
                     (GainsDesignation You Monarch Instructed)


jushiApprentice : Ability
jushiApprentice =
  Activated (Compound [Mana [Macros.generic 2, Macros.pip Blue], TapSymbol])
            (Sequentially
               [ Macros.drawACard
               , If (SetStatus Flipped Macros.thisCreature)
                    (CompareAmt (CountOf (InZone (Macros.handOf You)))
                                AtLeast (Lit 9))
                    Nothing ])


chainsaw : Card
chainsaw =
  Macros.card "Chainsaw" (Just [Macros.generic 3]) []
       (MkTypeLine [Equipment] [Artifact])
       [ Triggered When (Enters Macros.thisEquipment)
                   (DealDamage It (Lit 3)
                               (TargetGroup (Macros.upTo 1) Macros.creature))
       , Triggered Whenever
                   (Dies (CountedGroup (Macros.atLeast 1) Macros.creature))
                   (PutCounters (Lit 1) Rev Macros.thisEquipment)
       , Static (WhereLetterStatic LetterX (CountersOn Rev Macros.thisEquipment)
                                   (Gets (AttachHost Equipped (TypeW Creature))
                                         (PtUp (DefinedLetter LetterX))
                                         (PtUp (Lit 0))))
       , Macros.keywordCosting Equip (Mana [Macros.generic 3]) ]
       Nothing

lightmineField : Card
lightmineField =
  Macros.card "Lightmine Field"
       (Just [Macros.generic 2, Macros.pip White, Macros.pip White]) []
       (MkTypeLine [] [Enchantment])
       [ Triggered Whenever
                   (Attacks (CountedGroup (Macros.atLeast 1) Macros.creature))
                   (DealDamage Macros.thisEnchantment
                               (CountOf (And [Attacking, Macros.creature]))
                               (EachOf (Those (TypeW Creature)))) ]
       Nothing

misterFantastic : Card
misterFantastic =
  Macros.card "Mister Fantastic, Reed Richards"
       (Just [Macros.generic 1, Macros.pip White, Macros.pip Blue]) [Legendary]
       (MkTypeLine [Human, Hero] [Creature])
       [ KeywordAbility Reach
       , Triggered Whenever
                   (Enters (CountedGroup (Macros.atLeast 1)
                                         (And [IsToken, ControlledBy You])))
                   (May Nothing Macros.drawACard Nothing Nothing) ]
       (Just (2, 4))


woodlandChampion : Card
woodlandChampion =
  Macros.card "Woodland Champion" (Just [Macros.generic 1, Macros.pip Green]) []
       (MkTypeLine [Elf, Scout] [Creature])
       [ Triggered Whenever
                   (Enters (CountedGroup (Macros.atLeast 1)
                                         (And [IsToken, ControlledBy You])))
                   (PutCounters GroupSize Macros.plusOnePlusOne
                                Macros.thisCreature) ]
       (Just (2, 2))

ingeniousArtillerist : Card
ingeniousArtillerist =
  Macros.card "Ingenious Artillerist"
       (Just [Macros.generic 2, Macros.pip Red]) []
       (MkTypeLine [Human, Artificer] [Creature])
       [ Triggered Whenever
                   (Enters (CountedGroup (Macros.atLeast 1)
                                         (And [Macros.artifact, ControlledBy You])))
                   (DealDamage Macros.thisCreature GroupSize (Each Opponent)) ]
       (Just (3, 1))

hissingMiasma : Card
hissingMiasma =
  Macros.card "Hissing Miasma"
       (Just [Macros.generic 1, Macros.pip Black, Macros.pip Black]) []
       (MkTypeLine [] [Enchantment])
       [ Triggered Whenever
                   (Macros.attacksPlayer (Macros.a Macros.creature) You)
                   (Macros.losesLife (ControllerOf It) (Lit 1)) ]
       Nothing

orimsPrayer : Card
orimsPrayer =
  Macros.card "Orim's Prayer"
       (Just [Macros.generic 1, Macros.pip White, Macros.pip White]) []
       (MkTypeLine [] [Enchantment])
       [ Triggered Whenever
                   (Macros.attacksPlayer (CountedGroup (Macros.atLeast 1) Macros.creature) You)
                   (Macros.gainsLife You
                      (Macros.forEach (And [Attacking, Macros.creature]))) ]
       Nothing


undercoverButler : Card
undercoverButler =
  Macros.card "Undercover Butler"
       (Just [Macros.generic 2, Macros.hybridPip Blue Black]) []
       (MkTypeLine [Human, Rogue] [Creature])
       [ Triggered Whenever
                   (Macros.attacksPlayer Macros.thisCreature
                                         (Definite (And [AnyPlayer,
                                             Superlative MaxOf (PlayerStatAxis LifeTotal)
                                                         AnyPlayer])))
                   (Continuously (Deontic It Forbid Block Patient NoDeonticPatient)
                                 (Just Macros.thisTurn)) ]
       (Just (2, 3))

murmursFromBeyond : Card
murmursFromBeyond =
  Macros.card "Murmurs from Beyond"
       (Just [Macros.generic 2, Macros.pip Blue]) []
       (MkTypeLine [Arcane] [Instant])
       [ Spell (Sequentially
                  [ Macros.revealCards (Macros.topCards 3)
                  , Macros.chooses (Macros.a Opponent) (Macros.oneOf Them)
                  , Move (That CardW) Macros.graveyardZ
                  , Move TheRest Macros.handZ ]) ]
       Nothing


femerefEnchantress : Card
femerefEnchantress =
  Macros.card "Femeref Enchantress"
       (Just [Macros.pip Green, Macros.pip White]) []
       (MkTypeLine [Human, Druid] [Creature])
       [ Triggered Whenever
                   (Macros.putIntoFrom (Macros.a Macros.enchantment)
                                       Macros.graveyardZ
                                       (FromZone Macros.battlefieldZ))
                   Macros.drawACard ]
       (Just (1, 2))

voraciousBrood : Card
voraciousBrood =
  Macros.card "Voracious Brood"
       (Just [Macros.generic 2, Macros.pip Green]) []
       (MkTypeLine [Alien, Insect] [Creature])
       [ Static (EntersWithCounters Macros.thisCreature
                   (Macros.forEach (And [Macros.creature,
                                         InZone (Macros.graveyardOf You)]))
                   Macros.plusOnePlusOne)
       , Triggered Whenever
                   (Macros.putIntoFrom (CountedGroup (Macros.atLeast 1) Macros.creature)
                                       (Macros.graveyardOf You)
                                       FromAnywhere)
                   (PutCounters GroupSize Macros.plusOnePlusOne
                                Macros.thisCreature) ]
       (Just (1, 1))

herdBaloth : Card
herdBaloth =
  Macros.card "Herd Baloth"
       (Just [Macros.generic 3, Macros.pip Green, Macros.pip Green]) []
       (MkTypeLine [Beast] [Creature])
       [ Triggered Whenever
                   (CounterEvent CounterPut Macros.plusOnePlusOne
                                 Macros.thisCreature)
                   (Macros.may You
                      (Macros.create (Lit 1)
                                     (Macros.creatureTok 4 4 [Green] [Beast]))) ]
       (Just (4, 4))

flourishingDefenses : Card
flourishingDefenses =
  Macros.card "Flourishing Defenses"
       (Just [Macros.generic 4, Macros.pip Green]) []
       (MkTypeLine [] [Enchantment])
       [ Triggered Whenever
                   (Macros.singleCounterEvent CounterPut
                                              Macros.minusOneMinusOne
                                              (Macros.a Macros.creature))
                   (Macros.may You
                      (Macros.create (Lit 1)
                                     (Macros.creatureTok 1 1 [Green]
                                                         [Elf, Warrior]))) ]
       Nothing

rakshasaVizier : Card
rakshasaVizier =
  Macros.card "Rakshasa Vizier"
       (Just [Macros.generic 2, Macros.pip Black, Macros.pip Green,
              Macros.pip Blue]) []
       (MkTypeLine [Demon] [Creature])
       [ Triggered Whenever
                   (Macros.putIntoFrom (CountedGroup (Macros.atLeast 1)
                                                     (InZone (Macros.graveyardOf You)))
                                       Macros.exileZ
                                       (FromZone (Macros.graveyardOf You)))
                   (PutCounters GroupSize Macros.plusOnePlusOne
                                Macros.thisCreature) ]
       (Just (4, 4))


tocasiasWelcome : Card
tocasiasWelcome =
  Macros.card "Tocasia's Welcome"
       (Just [Macros.generic 2, Macros.pip White]) []
       (MkTypeLine [] [Enchantment])
       [ Macros.triggeredOnlyOnce Whenever
                                  (Enters (CountedGroup (Macros.atLeast 1)
                                                        (And [Macros.creature, ControlledBy You,
                                                              Compare ManaValue AtMost (Lit 3)])))
                                  OncePerTurn
                                  Macros.drawACard ]
       Nothing

duskLegionDuelist : Card
duskLegionDuelist =
  Macros.card "Dusk Legion Duelist"
       (Just [Macros.generic 1, Macros.pip White]) []
       (MkTypeLine [Vampire, Soldier] [Creature])
       [ KeywordAbility Vigilance
       , Macros.triggeredOnlyOnce Whenever
                                  (CounterEvent CounterPut Macros.plusOnePlusOne
                                                Macros.thisCreature)
                                  OncePerTurn
                                  Macros.drawACard ]
       (Just (2, 2))

foeLiage : Card
foeLiage =
  Macros.card "Foe-liage"
       (Just [Macros.generic 3, Macros.pip Green]) []
       (MkTypeLine [Plant, Mutant] [Creature])
       [ Macros.triggeredOnlyDuring Whenever
                                    (Enters (Macros.a Macros.land))
                                    (DuringWindow Turn (Just Yours))
                                    (PutCounters (Lit 1) Macros.plusOnePlusOne
                                                 Macros.thisCreature) ]
       (Just (3, 3))

lesserGargadon : Card
lesserGargadon =
  Macros.card "Lesser Gargadon"
       (Just [Macros.generic 2, Macros.pip Red, Macros.pip Red]) []
       (MkTypeLine [Beast] [Creature])
       [ Macros.triggeredOr Whenever
                            (Attacks Macros.thisCreature)
                            (Blocks Macros.thisCreature Nothing)
                            (Macros.sacrifice You (Macros.a Macros.land)) ]
       (Just (6, 4))

colossalGraveReaver : Card
colossalGraveReaver =
  Macros.card "Colossal Grave-Reaver"
       (Just [Macros.generic 6, Macros.pip Black, Macros.pip Green]) []
       (MkTypeLine [Dragon] [Creature])
       [ KeywordAbility Flying
       , Macros.triggeredOr Whenever
                            (Enters Macros.thisCreature)
                            (Attacks Macros.thisCreature)
                            (Macros.mills You (Lit 3) You)
       , Triggered Whenever
                   (Macros.putIntoFrom (CountedGroup (Macros.atLeast 1)
                                                     (And [Macros.creature,
                                                           InZone Macros.yourLibrary]))
                                       (Macros.graveyardOf You)
                                       (FromZone Macros.yourLibrary))
                   (Macros.putOntoBattlefield (Macros.oneOf Them)) ]
       (Just (7, 6))

hardenedScales : Card
hardenedScales =
  Macros.card "Hardened Scales" (Just [Macros.pip Green]) []
       (MkTypeLine [] [Enchantment])
       [ Static (Intercepts
                   (CounterEvent CounterPut Macros.plusOnePlusOne
                                 (Macros.a Macros.creatureYouControl))
                   (PutCounters (Plus ThatMuch (Lit 1))
                                Macros.plusOnePlusOne It)
                   Repeatedly) ]
       Nothing

branchingEvolution : Card
branchingEvolution =
  Macros.card "Branching Evolution"
       (Just [Macros.generic 2, Macros.pip Green]) []
       (MkTypeLine [] [Enchantment])
       [ Static (Intercepts
                   (CounterEvent CounterPut Macros.plusOnePlusOne
                                 (Macros.a Macros.creatureYouControl))
                   (PutCounters (Times 2 ThatMuch)
                                Macros.plusOnePlusOne (That (TypeW Creature)))
                   Repeatedly) ]
       Nothing

shalaiAndHallar : Card
shalaiAndHallar =
  Macros.card "Shalai and Hallar"
       (Just [Macros.generic 1, Macros.pip Red, Macros.pip Green, Macros.pip White])
       [Legendary]
       (MkTypeLine [Angel, Elf] [Creature])
       [ KeywordAbility Flying
       , KeywordAbility Vigilance
       , Triggered Whenever
                   (CounterEvent CounterPut Macros.plusOnePlusOne
                                 (Macros.a Macros.creatureYouControl))
                   (DealDamage Macros.thisCreature ThatMuch
                               (Macros.target Opponent)) ]
       (Just (3, 3))

divineVisitation : Card
divineVisitation =
  Macros.card "Divine Visitation"
       (Just [Macros.generic 3, Macros.pip White, Macros.pip White]) []
       (MkTypeLine [] [Enchantment])
       [ Static (Intercepts
                   (Macros.tokensCreatedUnder (CountedGroup (Macros.atLeast 1)
                                                            (And [Macros.creature, IsToken]))
                                              You)
                   (Macros.create GroupSize
                                  (MkToken (Just (Lit 4, Lit 4)) [White]
                                           (MkTypeLine [Angel] [Creature])
                                           [KeywordAbility Flying, KeywordAbility Vigilance]
                                             Nothing))
                   Repeatedly) ]
       Nothing

anointedProcession : Card
anointedProcession =
  Macros.card "Anointed Procession"
       (Just [Macros.generic 3, Macros.pip White]) []
       (MkTypeLine [] [Enchantment])
       [ Static (Intercepts
                   (Macros.tokensCreatedByEffectUnder (CountedGroup (Macros.atLeast 1) IsToken)
                                                      You)
                   (Create You (Times 2 GroupSize) TokenAsThose [])
                   Repeatedly) ]
       Nothing

primalVigor : Card
primalVigor =
  Macros.card "Primal Vigor"
       (Just [Macros.generic 4, Macros.pip Green]) []
       (MkTypeLine [] [Enchantment])
       [ Static (Intercepts
                   (TokensCreated (CountedGroup (Macros.atLeast 1) IsToken))
                   (Create You (Times 2 GroupSize) TokenAsThose [])
                   Repeatedly)
       , Static (Intercepts
                   (CounterEvent CounterPut Macros.plusOnePlusOne
                                 (Macros.a Macros.creature))
                   (PutCounters (Times 2 ThatMuch)
                                Macros.plusOnePlusOne (That (TypeW Creature)))
                   Repeatedly) ]
       Nothing

adrixAndNev : Card
adrixAndNev =
  Macros.card "Adrix and Nev, Twincasters"
       (Just [Macros.generic 2, Macros.pip Green, Macros.pip Blue]) [Legendary]
       (MkTypeLine [Merfolk, Wizard] [Creature])
       [ Macros.keywordCosting Ward (Mana [Macros.generic 2])
       , Static (Intercepts
                   (Macros.tokensCreatedUnder (CountedGroup (Macros.atLeast 1) IsToken) You)
                   (Create You (Times 2 GroupSize) TokenAsThose [])
                   Repeatedly) ]
       (Just (2, 2))


naturalAffinity : Card
naturalAffinity =
  Macros.card "Natural Affinity" (Just [Macros.generic 2, Macros.pip Green]) []
       (MkTypeLine [] [Instant])
       [ Spell (Continuously
                  (SetsType (AllOf Macros.land)
                            (MkToken (Just (Lit 2, Lit 2)) []
                                     (MkTypeLine [] [Creature]) [] Nothing)
                            (Just Land))
                  (Just Macros.untilEndOfTurn)) ]
       Nothing

mishrasFactory : Card
mishrasFactory =
  Macros.card "Mishra's Factory" Nothing [] (MkTypeLine [] [Land])
       [ Activated TapSymbol (AddMana You (Lit 1) (Runs [[Colorless]]) [])
       , Activated (Mana [Macros.generic 1])
                   (Continuously
                      (SetsType Macros.thisLand
                                (MkToken (Just (Lit 2, Lit 2)) []
                                         (MkTypeLine [AssemblyWorker] [Artifact, Creature])
                                         [] Nothing)
                                (Just Land))
                      (Just Macros.untilEndOfTurn))
       , Activated TapSymbol
                   (Macros.gets (Macros.target (And [Macros.creature, HasSubtype AssemblyWorker]))
                                (PtUp (Lit 1)) (PtUp (Lit 1)) (Just Macros.untilEndOfTurn)) ]
       Nothing

windZendikon : Card
windZendikon =
  Macros.card "Wind Zendikon" (Just [Macros.pip Blue]) []
       (MkTypeLine [Aura] [Enchantment])
       [ Macros.keywordSubject Enchant Macros.land
       , Static (SetsType (AttachHost Enchanted (TypeW Land))
                          (MkToken (Just (Lit 2, Lit 2)) [Blue]
                                   (MkTypeLine [Elemental] [Creature])
                                   [KeywordAbility Flying] Nothing)
                          (Just Land)) ]
       Nothing

nullhideFerox : Ability
nullhideFerox =
  Activated (Mana [Macros.generic 2])
            (Continuously (LosesAllAbilities Macros.thisCreature)
                          (Just Macros.untilEndOfTurn))


awakenTheBear : Card
awakenTheBear =
  Macros.card "Awaken the Bear" (Just [Macros.generic 2, Macros.pip Green]) []
       (MkTypeLine [] [Instant])
       [ Spell (Continuously
                  (AndAlso [ Gets (Macros.target Macros.creature)
                                  (PtUp (Lit 3)) (PtUp (Lit 3))
                           , Gains It (KeywordAbility Trample) ])
                  (Just Macros.untilEndOfTurn)) ]
       Nothing

spidersilkArmor : Card
spidersilkArmor =
  Macros.card "Spidersilk Armor" (Just [Macros.generic 2, Macros.pip Green]) []
       (MkTypeLine [] [Enchantment])
       [ Static (AndAlso [ Gets (AllOf Macros.creatureYouControl)
                                (PtUp (Lit 0)) (PtUp (Lit 1))
                         , Gains Them (KeywordAbility Reach) ]) ]
       Nothing

turnToFrog : Card
turnToFrog =
  Macros.card "Turn to Frog" (Just [Macros.generic 1, Macros.pip Blue]) []
       (MkTypeLine [] [Instant])
       [ Spell (Continuously
                  (AndAlso [ LosesAllAbilities (Macros.target Macros.creature)
                           , SetsType It (MkToken Nothing [Blue]
                                                  (MkTypeLine [Frog] []) [] Nothing)
                                      Nothing
                           , HasBasePt It (Lit 1) (Lit 1) ])
                  (Just Macros.untilEndOfTurn)) ]
       Nothing

humility : Card
humility =
  Macros.card "Humility" (Just [Macros.generic 2, Macros.pip White, Macros.pip White]) []
       (MkTypeLine [] [Enchantment])
       [ Static (AndAlso [ LosesAllAbilities (AllOf Macros.creature)
                         , HasBasePt Them (Lit 1) (Lit 1) ]) ]
       Nothing


arcaneFlight : Card
arcaneFlight =
  Macros.card "Arcane Flight" (Just [Macros.pip Blue]) []
       (MkTypeLine [Aura] [Enchantment])
       [ Macros.keywordSubject Enchant Macros.creature
       , Static (AndAlso [ Gets (AttachHost Enchanted (TypeW Creature))
                                (PtUp (Lit 1)) (PtUp (Lit 1))
                         , Gains It (KeywordAbility Flying) ]) ]
       Nothing

bootsOfSpeed : Card
bootsOfSpeed =
  Macros.card "Boots of Speed" (Just [Macros.pip Red]) []
       (MkTypeLine [Equipment] [Artifact])
       [ Static (AndAlso [ Gets (AttachHost Equipped (TypeW Creature))
                                (PtUp (Lit 1)) (PtUp (Lit 0))
                         , Gains It (KeywordAbility Haste) ])
       , Macros.keywordCosting Equip (Mana [Macros.generic 1]) ]
       Nothing

aetherTunnel : Card
aetherTunnel =
  Macros.card "Aether Tunnel" (Just [Macros.generic 1, Macros.pip Blue]) []
       (MkTypeLine [Aura] [Enchantment])
       [ Macros.keywordSubject Enchant Macros.creature
       , Static (AndAlso [ Gets (AttachHost Enchanted (TypeW Creature))
                                (PtUp (Lit 1)) (PtUp (Lit 0))
                         , Deontic It Forbid Block Patient NoDeonticPatient ]) ]
       Nothing

frogify : Card
frogify =
  Macros.card "Frogify" (Just [Macros.generic 1, Macros.pip Blue]) []
       (MkTypeLine [Aura] [Enchantment])
       [ Macros.keywordSubject Enchant Macros.creature
       , Static (AndAlso [ LosesAllAbilities (AttachHost Enchanted (TypeW Creature))
                         , SetsType It (MkToken Nothing [Blue]
                                                (MkTypeLine [Frog] [Creature]) [] Nothing)
                                    Nothing
                         , HasBasePt It (Lit 1) (Lit 1) ]) ]
       Nothing


darksteelMutation : Card
darksteelMutation =
  Macros.card "Darksteel Mutation" (Just [Macros.generic 1, Macros.pip White]) []
       (MkTypeLine [Aura] [Enchantment])
       [ Macros.keywordSubject Enchant Macros.creature
       , Static (AndAlso [ SetsType (AttachHost Enchanted (TypeW Creature))
                                    (MkToken Nothing []
                                             (MkTypeLine [Insect] [Artifact, Creature])
                                             [] Nothing)
                                    Nothing
                         , HasBasePt It (Lit 0) (Lit 1)
                         , Gains It (KeywordAbility Indestructible)
                         , LosesAllAbilities It ]) ]
       Nothing

kenrithsTransformation : Card
kenrithsTransformation =
  Macros.card "Kenrith's Transformation" (Just [Macros.generic 1, Macros.pip Green]) []
       (MkTypeLine [Aura] [Enchantment])
       [ Macros.keywordSubject Enchant Macros.creature
       , Triggered When (Enters Macros.thisAura) (Draw You (Lit 1))
       , Static (AndAlso [ LosesAllAbilities (AttachHost Enchanted (TypeW Creature))
                         , SetsType It (MkToken Nothing [Green]
                                                (MkTypeLine [Elk] [Creature]) [] Nothing)
                                    Nothing
                         , HasBasePt It (Lit 3) (Lit 3) ]) ]
       Nothing

amphibianDownpour : Card
amphibianDownpour =
  Macros.card "Amphibian Downpour" (Just [Macros.generic 2, Macros.pip Blue]) []
       (MkTypeLine [Aura] [Enchantment])
       [ KeywordAbility Flash
       , KeywordAbility Storm
       , Macros.keywordSubject Enchant Macros.creature
       , Static (AndAlso [ LosesAllAbilities (AttachHost Enchanted (TypeW Creature))
                         , SetsType It (MkToken Nothing [Blue]
                                                (MkTypeLine [Frog] [Creature]) [] Nothing)
                                    Nothing
                         , HasBasePt It (Lit 1) (Lit 1) ]) ]
       Nothing


lignify : Card
lignify =
  Macros.card "Lignify" (Just [Macros.generic 1, Macros.pip Green]) []
       (MkTypeLine [Treefolk, Aura] [Kindred, Enchantment])
       [ Macros.keywordSubject Enchant Macros.creature
       , Static (AndAlso [ SetsType (AttachHost Enchanted (TypeW Creature))
                                    (MkToken Nothing [] (MkTypeLine [Treefolk] [])
                                             [] Nothing)
                                    Nothing
                         , HasBasePt It (Lit 0) (Lit 4)
                         , LosesAllAbilities It ]) ]
       Nothing

invasionOfDominaria : Card
invasionOfDominaria =
  Macros.card "Invasion of Dominaria" (Just [Macros.generic 2, Macros.pip White]) []
       (MkTypeLine [Siege] [Battle])
       [ Triggered When (Enters Macros.thisSiege)
                   (Sequentially [ ChangeLife You (Up (Lit 4))
                                 , Draw You (Lit 1) ]) ]
       Nothing


causticTar : Card
causticTar =
  Macros.card "Caustic Tar" (Just [Macros.generic 4, Macros.pip Black, Macros.pip Black]) []
       (MkTypeLine [Aura] [Enchantment])
       [ Macros.keywordSubject Enchant Macros.land
       , Static (Gains (AttachHost Enchanted (TypeW Land))
                       (Activated TapSymbol
                                  (ChangeLife (Macros.target AnyPlayer) (Down (Lit 3))))) ]
       Nothing

ragingRavine : Card
ragingRavine =
  Macros.card "Raging Ravine" Nothing [] (MkTypeLine [] [Land])
       [ Static (Macros.entersTapped Macros.thisLand)
       , Activated TapSymbol
                   (AddMana You (Lit 1)
                            (Runs [[OfColor Red], [OfColor Green]]) [])
       , Activated (Mana [Macros.generic 2, Macros.pip Red, Macros.pip Green])
                   (Continuously
                      (SetsType Macros.thisLand
                                (MkToken (Just (Lit 3, Lit 3)) [Red, Green]
                                         (MkTypeLine [Elemental] [Creature])
                                         [ Triggered Whenever
                                                     (Attacks Macros.thisCreature)
                                                     (PutCounters (Lit 1)
                                                                  Macros.plusOnePlusOne It) ]
                                         Nothing)
                                (Just Land))
                      (Just Macros.untilEndOfTurn)) ]
       Nothing



||| Ajani, Adversary of Tyrants
ajanisEmblem : Effect []
ajanisEmblem =
  GetsEmblem You
    [ Triggered At (BeginningOf EndStep (ByWord Yours))
                (Macros.create (Lit 3)
                   (MkToken (Just (Lit 1, Lit 1)) [White] (MkTypeLine [Cat] [Creature])
                            [KeywordAbility Lifelink] Nothing)) ]

||| Saheeli, Filigree Master
saheelisEmblem : Effect []
saheelisEmblem =
  GetsEmblem You
    [ Static (Gets (AllOf (And [Macros.artifact, Macros.creature, ControlledBy You]))
                   (PtUp (Lit 1)) (PtUp (Lit 1)))
    , Static (CostsToCast (AllOf (And [Macros.artifact, Macros.spell, CastBy You]))
                          (CostLess (Lit 1))) ]


jaceBeleren : Card
jaceBeleren =
  Macros.card "Jace Beleren" (Just [Macros.generic 1, Macros.pip Blue, Macros.pip Blue])
       [Legendary] (MkTypeLine [Jace] [Planeswalker])
       [ Activated (LoyaltySymbol (LoyaltyUp 2)) (Draw (Each AnyPlayer) (Lit 1))
       , Activated (LoyaltySymbol (LoyaltyDown 1))
                   (Macros.drawsACard (Macros.target AnyPlayer))
       , Activated (LoyaltySymbol (LoyaltyDown 10))
                   (Macros.mills (Macros.target AnyPlayer) (Lit 20) They) ]
       Nothing

elspethSunsChampion : Card
elspethSunsChampion =
  Macros.card "Elspeth, Sun's Champion"
       (Just [Macros.generic 4, Macros.pip White, Macros.pip White])
       [Legendary] (MkTypeLine [Elspeth] [Planeswalker])
       [ Activated (LoyaltySymbol (LoyaltyUp 1))
                   (Macros.create (Lit 3) (Macros.creatureTok 1 1 [White] [Soldier]))
       , Activated (LoyaltySymbol (LoyaltyDown 3))
                   (Macros.destroy (AllOf (And [Macros.creature,
                                                Compare Power AtLeast (Lit 4)])))
       , Activated (LoyaltySymbol (LoyaltyDown 7))
                   (GetsEmblem You
                      [ Static (AndAlso [ Gets (AllOf Macros.creatureYouControl)
                                               (PtUp (Lit 2)) (PtUp (Lit 2))
                                        , Gains Them (KeywordAbility Flying) ]) ]) ]
       Nothing

||| Chandra Nalaar
chandraNalaarsX : Ability
chandraNalaarsX =
  Activated (LoyaltySymbol LoyaltyDownX)
            (DealDamage This (DefinedLetter LetterX) (Macros.target Macros.creature))

||| Elspeth's Talent
elspethsTalentGrant : StaticEffect []
elspethsTalentGrant =
  Gains (AttachHost Enchanted (TypeW Planeswalker))
        (Activated (LoyaltySymbol (LoyaltyUp 1))
                   (Macros.create (Lit 3) (Macros.creatureTok 1 1 [White] [Soldier])))


opt : Card
opt =
  Macros.card "Opt" (Just [Macros.pip Blue]) [] (MkTypeLine [] [Instant])
       [ Spell (Sequentially [ Does You Scry (Macros.lookAt (Macros.topCards 1))
                             , Macros.drawACard ]) ]
       Nothing

serumVisions : Card
serumVisions =
  Macros.card "Serum Visions" (Just [Macros.pip Blue]) [] (MkTypeLine [] [Sorcery])
       [ Spell (Sequentially [ Macros.drawACard
                             , Does You Scry (Macros.lookAt (Macros.topCards 2)) ]) ]
       Nothing

consider : Card
consider =
  Macros.card "Consider" (Just [Macros.pip Blue]) [] (MkTypeLine [] [Instant])
       [ Spell (Sequentially [ Does You Surveil (Macros.lookAt (Macros.topCards 1))
                             , Macros.drawACard ]) ]
       Nothing

crystalBall : Card
crystalBall =
  Macros.card "Crystal Ball" (Just [Macros.generic 3]) [] (MkTypeLine [] [Artifact])
       [ Activated (Compound [Mana [Macros.generic 1], TapSymbol])
                   (Does You Scry (Macros.lookAt (Macros.topCards 2))) ]
       Nothing

nefariousImp : Card
nefariousImp =
  Macros.card "Nefarious Imp" (Just [Macros.generic 2, Macros.pip Black]) []
       (MkTypeLine [Imp] [Creature])
       [ KeywordAbility Flying
       , Triggered Whenever
                   (Leaves (CountedGroup (Macros.atLeast 1)
                                         (And [Permanent, ControlledBy You])))
                   (Does You Scry (Macros.lookAt (Macros.topCards 1))) ]
       (Just (2, 1))

saheeliFiligreeMaster : Card
saheeliFiligreeMaster =
  Macros.card "Saheeli, Filigree Master"
       (Just [Macros.generic 2, Macros.pip Blue, Macros.pip Red])
       [Legendary] (MkTypeLine [Saheeli] [Planeswalker])
       [ Activated (LoyaltySymbol (LoyaltyUp 1))
                   (Sequentially
                      [ Does You Scry (Macros.lookAt (Macros.topCards 1))
                      , Macros.mayThen You
                          (SetStatus Tapped
                             (Macros.a (And [Macros.artifact, Macros.untapped,
                                             ControlledBy You])))
                          Macros.drawACard ])
       , Activated (LoyaltySymbol (LoyaltyDown 2))
                   (Sequentially
                      [ Macros.create (Lit 2)
                          (MkToken (Just (Lit 1, Lit 1)) []
                                   (MkTypeLine [Thopter] [Artifact, Creature])
                                   [KeywordAbility Flying] Nothing)
                      , Macros.gainsHaste Them (Just Macros.untilEndOfTurn) ])
       , Activated (LoyaltySymbol (LoyaltyDown 4))
                   (GetsEmblem You
                      [ Static (Gets (AllOf (And [Macros.artifact, Macros.creature,
                                                  ControlledBy You]))
                                     (PtUp (Lit 1)) (PtUp (Lit 1)))
                      , Static (CostsToCast (AllOf (And [Macros.artifact, Macros.spell,
                                                         CastBy You]))
                                            (CostLess (Lit 1))) ]) ]
       Nothing



manalith : Card
manalith =
  Macros.card "Manalith" (Just [Macros.generic 3]) [] (MkTypeLine [] [Artifact])
       [ Activated TapSymbol (AddMana You (Lit 1) (AnyColor SameColor) []) ]
       Nothing

seethingSong : Card
seethingSong =
  Macros.card "Seething Song" (Just [Macros.generic 2, Macros.pip Red]) []
       (MkTypeLine [] [Instant])
       [ Spell (AddMana You (Lit 1)
                        (Runs [[OfColor Red, OfColor Red, OfColor Red,
                                OfColor Red, OfColor Red]]) []) ]
       Nothing

ancientZiggurat : Card
ancientZiggurat =
  Macros.card "Ancient Ziggurat" Nothing [] (MkTypeLine [] [Land])
       [ Activated TapSymbol
                   (AddMana You (Lit 1) (AnyColor SameColor)
                            [SpendOnly [ToCast (And [Macros.creature, Macros.spell])]]) ]
       Nothing

mishrasWorkshop : Card
mishrasWorkshop =
  Macros.card "Mishra's Workshop" Nothing [] (MkTypeLine [] [Land])
       [ Activated TapSymbol
                   (AddMana You (Lit 1)
                            (Runs [[Colorless, Colorless, Colorless]])
                            [SpendOnly [ToCast (And [Macros.artifact, Macros.spell])]]) ]
       Nothing

boommobile : Ability
boommobile =
  Triggered When (Enters Macros.thisArtifact)
            (AddMana You (Lit 4) (AnyColor SameColor)
                     [SpendOnly [ToActivate Nothing]])



mizziumTransreliquat : Card
mizziumTransreliquat =
  Macros.card "Mizzium Transreliquat" (Just [Macros.generic 3]) []
       (MkTypeLine [] [Artifact])
       [ Activated (Mana [Macros.generic 3])
                   (Continuously
                      (BecomesCopy Macros.thisArtifact
                                   (Macros.target Macros.artifact) [])
                      (Just Macros.untilEndOfTurn))
       , Activated (Mana [Macros.generic 1, Macros.pip Blue, Macros.pip Red])
                   (Continuously
                      (BecomesCopy Macros.thisArtifact
                                   (Macros.target Macros.artifact)
                                   [ExceptThisAbility])
                      Nothing) ]
       Nothing

irenicussVileDuplication : Card
irenicussVileDuplication =
  Macros.card "Irenicus's Vile Duplication"
       (Just [Macros.generic 3, Macros.pip Blue]) []
       (MkTypeLine [] [Sorcery])
       [ Spell (Create You (Lit 1)
                  (TokenCopyOf (Macros.target (And [Macros.creature, ControlledBy You]))
                               [ExceptAbility (KeywordAbility Flying),
                                ExceptNonlegendary])
                  []) ]
       Nothing

cacklingCounterpart : Card
cacklingCounterpart =
  Macros.card "Cackling Counterpart"
       (Just [Macros.generic 1, Macros.pip Blue, Macros.pip Blue]) []
       (MkTypeLine [] [Instant])
       [ Spell (Create You (Lit 1)
                  (TokenCopyOf (Macros.target (And [Macros.creature, ControlledBy You])) [])
                  []) ]
       Nothing

||| Saheeli Rai
saheelisCopy : Effect []
saheelisCopy =
  Sequentially [ Create You (Lit 1)
                   (TokenCopyOf (Macros.target (And [Or [Macros.artifact, Macros.creature],
                                                     ControlledBy You]))
                                [ExceptTypes (MkTypeLine [] [Artifact])])
                   []
               , Macros.gainsHaste (That TokenW) Nothing
               , Delayed (BeginningOf EndStep NoPossessor) (Macros.exile It) ]

public export
twincast : Card
twincast =
  Macros.card "Twincast" (Just [Macros.pip Blue, Macros.pip Blue]) []
       (MkTypeLine [] [Instant])
       [ Spell (Sequentially
                  [ CopyStack You
                      (Macros.target (And [Macros.instantOrSorcery, Macros.spell]))
                      (Lit 1) []
                  , Macros.may You (ChooseNewTargets (That CopyW)) ]) ]
       Nothing

public export
fork : Card
fork =
  Macros.card "Fork" (Just [Macros.pip Red, Macros.pip Red]) []
       (MkTypeLine [] [Instant])
       [ Spell (Sequentially
                  [ CopyStack You
                      (Macros.target (And [Macros.instantOrSorcery, Macros.spell]))
                      (Lit 1) [ExceptColor Red]
                  , Macros.may You (ChooseNewTargets (That CopyW)) ]) ]
       Nothing

public export
meletisCharlatan : Card
meletisCharlatan =
  Macros.card "Meletis Charlatan"
       (Just [Macros.generic 2, Macros.pip Blue]) []
       (MkTypeLine [Human, Wizard] [Creature])
       [ Activated (Compound [Mana [Macros.generic 2, Macros.pip Blue], TapSymbol])
                   (Sequentially
                      [ CopyStack
                          (ControllerOf (Macros.target
                                           (And [Macros.instantOrSorcery, Macros.spell])))
                          It (Lit 1) []
                      , Macros.may (That PlayerW) (ChooseNewTargets (That CopyW)) ]) ]
       (Just (2, 3))

public export
echoMagesFourthLevel : Ability
echoMagesFourthLevel =
  Activated (Compound [Mana [Macros.pip Blue, Macros.pip Blue], TapSymbol])
            (Sequentially
               [ CopyStack You
                   (Macros.target (And [Macros.instantOrSorcery, Macros.spell]))
                   (Lit 2) []
               , Macros.may You (ChooseNewTargets (Those CopyW)) ])

public export
tawnosTheToymaker : Card
tawnosTheToymaker =
  Macros.card "Tawnos, the Toymaker"
       (Just [Macros.generic 3, Macros.pip Green, Macros.pip Blue]) [Legendary]
       (MkTypeLine [Human, Artificer] [Creature])
       [ Triggered Whenever
                   (Casts You (Macros.a (And [Or [HasSubtype Beast, HasSubtype Bird],
                                              Macros.creature, Macros.spell])))
                   (Macros.may You
                      (CopyStack You It (Lit 1)
                                 [ExceptTypes (MkTypeLine [] [Artifact])])) ]
       (Just (3, 5))

public export
etherealHaze : Card
etherealHaze =
  Macros.card "Ethereal Haze" (Just [Macros.pip White]) []
       (MkTypeLine [Arcane] [Instant])
       [ Spell (Macros.preventAllBy AnyDamage Everywhere
                                    (AllOf Macros.creature) (Just Macros.thisTurn)) ]
       Nothing

public export
defang : Card
defang =
  Macros.card "Defang" (Just [Macros.generic 1, Macros.pip White]) []
       (MkTypeLine [Aura] [Enchantment])
       [ Macros.keywordSubject Enchant Macros.creature
       , Static (Prevents AnyDamage AllOfIt Everywhere
                          (Just (AttachHost Enchanted (TypeW Creature))) Nothing) ]
       Nothing

public export
sphereOfPurity : Card
sphereOfPurity =
  Macros.card "Sphere of Purity" (Just [Macros.generic 3, Macros.pip White]) []
       (MkTypeLine [] [Enchantment])
       [ Static (PreventsFrom AnyDamage (DealtBy (Macros.a Macros.artifact))
                              (Macros.shieldingIt You)
                              (CutSome (Lit 1)) Repeatedly Nothing) ]
       Nothing

public export
dazzlingReflection : Card
dazzlingReflection =
  Macros.card "Dazzling Reflection" (Just [Macros.generic 1, Macros.pip White]) []
       (MkTypeLine [] [Instant])
       [ Spell (Sequentially
                  [ ChangeLife You (Up (StatOf Power (Macros.target Macros.creature)))
                  , Continuously
                      (PreventsFrom AnyDamage (DealtBy (That (TypeW Creature)))
                                    Everywhere CutAll NextTimeOnly Nothing)
                      (Just Macros.thisTurn) ]) ]
       Nothing

public export
excruciator : Card
excruciator =
  Macros.card "Excruciator"
       (Just [Macros.generic 6, Macros.pip Red, Macros.pip Red]) []
       (MkTypeLine [Avatar] [Creature])
       [ Static (CantPrevent AnyDamage Everywhere (Just Macros.thisCreature)) ]
       (Just (7, 7))

public export
flaringPain : Card
flaringPain =
  Macros.card "Flaring Pain" (Just [Macros.generic 1, Macros.pip Red]) []
       (MkTypeLine [] [Instant])
       [ Spell (Continuously (CantPrevent AnyDamage Everywhere Nothing)
                             (Just Macros.thisTurn)) ]
       Nothing

public export
thunderstaff : Card
thunderstaff =
  Macros.card "Thunderstaff" (Just [Macros.generic 3]) []
       (MkTypeLine [] [Artifact])
       [ Static (Conditionally (Matches Macros.thisArtifact (HasStatus Untapped))
                               (PreventsFrom CombatOnly
                                             (DealtBy (Macros.a Macros.creature))
                                             (Macros.shieldingIt You)
                                             (CutSome (Lit 1)) Repeatedly Nothing))
       , Activated (Compound [Mana [Macros.generic 2], TapSymbol])
                   (Macros.gets (AllOf (And [Macros.creature, Attacking]))
                                (PtUp (Lit 1)) (PtUp (Lit 0))
                                (Just Macros.untilEndOfTurn)) ]
       Nothing

public export
gideonAllyOfZendikar : Card
gideonAllyOfZendikar =
  Macros.card "Gideon, Ally of Zendikar"
       (Just [Macros.generic 2, Macros.pip White, Macros.pip White]) [Legendary]
       (MkTypeLine [Gideon] [Planeswalker])
       [ Activated (LoyaltySymbol (LoyaltyUp 1))
                   (Sequentially
                      [ Continuously
                          (SetsType (AsType Planeswalker This)
                                    (MkToken (Just (Lit 5, Lit 5)) []
                                             (MkTypeLine [Human, Soldier, Ally] [Creature])
                                             [KeywordAbility Indestructible] Nothing)
                                    (Just Planeswalker))
                          (Just Macros.untilEndOfTurn)
                      , Macros.preventAll AnyDamage (Macros.shieldingIt It)
                                          (Just Macros.thisTurn) ])
       , Activated (LoyaltySymbol LoyaltyZero)
                   (Macros.create (Lit 1) (Macros.creatureTok 2 2 [White] [Knight, Ally]))
       , Activated (LoyaltySymbol (LoyaltyDown 4))
                   (GetsEmblem You [Static (Gets (AllOf Macros.creatureYouControl)
                                                 (PtUp (Lit 1)) (PtUp (Lit 1)))]) ]
       Nothing

public export
turnTheTables : Card
turnTheTables =
  Macros.card "Turn the Tables"
       (Just [Macros.generic 3, Macros.pip White, Macros.pip White]) []
       (MkTypeLine [] [Instant])
       [ Spell (Continuously
                  (Redirects CombatOnly AllOfIt (Macros.shieldingIt You) Nothing
                             (Macros.target (And [Macros.creature, Attacking])))
                  (Just Macros.thisTurn)) ]
       Nothing

public export
pariah : Card
pariah =
  Macros.card "Pariah" (Just [Macros.generic 2, Macros.pip White]) []
       (MkTypeLine [Aura] [Enchantment])
       [ Macros.keywordSubject Enchant Macros.creature
       , Static (Redirects AnyDamage AllOfIt (Macros.shieldingIt You) Nothing
                           (AttachHost Enchanted (TypeW Creature))) ]
       Nothing

public export
martyrsOfKorlis : Card
martyrsOfKorlis =
  Macros.card "Martyrs of Korlis"
       (Just [Macros.generic 3, Macros.pip White, Macros.pip White]) []
       (MkTypeLine [Human] [Creature])
       [ Static (Conditionally (Matches Macros.thisCreature (HasStatus Untapped))
                               (Redirects AnyDamage AllOfIt (Macros.shieldingIt You)
                                          (Just (AllOf Macros.artifact))
                                          Macros.thisCreature)) ]
       (Just (1, 6))

public export
wardOfPiety : Card
wardOfPiety =
  Macros.card "Ward of Piety" (Just [Macros.generic 1, Macros.pip White]) []
       (MkTypeLine [Aura] [Enchantment])
       [ Macros.keywordSubject Enchant Macros.creature
       , Activated (Mana [Macros.generic 1, Macros.pip White])
                   (Continuously
                      (Redirects AnyDamage (TheNext (Lit 1))
                                 (Macros.shieldingIt (AttachHost Enchanted (TypeW Creature)))
                                 Nothing (Macros.target AnyTarget))
                      (Just Macros.thisTurn)) ]
       Nothing

public export
mirrorwoodTreefolk : Card
mirrorwoodTreefolk =
  Macros.card "Mirrorwood Treefolk" (Just [Macros.generic 3, Macros.pip Green]) []
       (MkTypeLine [Treefolk] [Creature])
       [ Activated (Mana [Macros.generic 2, Macros.pip Red, Macros.pip White])
                   (Continuously
                      (RedirectsFrom AnyDamage Unattributed
                                     (Macros.shieldingIt Macros.thisCreature)
                                     (Macros.target AnyTarget) NextTimeOnly)
                      (Just Macros.thisTurn)) ]
       (Just (2, 4))

public export
stormwildCapridor : Card
stormwildCapridor =
  Macros.card "Stormwild Capridor" (Just [Macros.generic 2, Macros.pip White]) []
       (MkTypeLine [Bird, Goat] [Creature])
       [ KeywordAbility Flying
       , Static (PreventsFrom NoncombatOnly Unattributed
                              (Macros.shieldingIt Macros.thisCreature)
                              CutAll Repeatedly
                              (Just (PutCounters PreventedThisWay
                                                 Macros.plusOnePlusOne
                                                 Macros.thisCreature))) ]
       (Just (1, 3))

public export
carom : Card
carom =
  Macros.card "Carom" (Just [Macros.generic 1, Macros.pip White]) []
       (MkTypeLine [] [Instant])
       [ Spell (Sequentially
                  [ Continuously
                      (Redirects AnyDamage (TheNext (Lit 1))
                                 (Macros.shieldingIt (Macros.target Macros.creature))
                                 Nothing
                                 (Macros.target (And [Macros.creature, Other])))
                      (Just Macros.thisTurn)
                  , Macros.drawACard ]) ]
       Nothing

public export
daughterOfAutumn : Card
daughterOfAutumn =
  Macros.card "Daughter of Autumn"
       (Just [Macros.generic 2, Macros.pip Green, Macros.pip Green]) [Legendary]
       (MkTypeLine [Avatar] [Creature])
       [ Activated (Mana [Macros.pip White])
                   (Continuously
                      (Redirects AnyDamage (TheNext (Lit 1))
                                 (Macros.shieldingIt
                                    (Macros.target (And [Macros.creature, ColorIs White])))
                                 Nothing Macros.thisCreature)
                      (Just Macros.thisTurn)) ]
       (Just (2, 4))

public export
aegisOfHonor : Card
aegisOfHonor =
  Macros.card "Aegis of Honor" (Just [Macros.pip White]) []
       (MkTypeLine [] [Enchantment])
       [ Activated (Mana [Macros.generic 1])
                   (Continuously
                      (RedirectsFrom AnyDamage
                                     (DealtBy (Macros.a Macros.instantOrSorcery))
                                     (Macros.shieldingIt You) (ControllerOf It)
                                     NextTimeOnly)
                      (Just Macros.thisTurn)) ]
       Nothing

public export
testOfFaith : Card
testOfFaith =
  Macros.card "Test of Faith" (Just [Macros.generic 1, Macros.pip White]) []
       (MkTypeLine [] [Instant])
       [ Spell (Continuously
                  (Prevents AnyDamage (TheNext (Lit 3))
                            (Macros.shieldingIt (Macros.target Macros.creature))
                            Nothing
                            (Just (PutCounters PreventedThisWay
                                               Macros.plusOnePlusOne
                                               (That (TypeW Creature)))))
                  (Just Macros.thisTurn)) ]
       Nothing

public export
temper : Card
temper =
  Macros.card "Temper" (Just [Variable, Macros.generic 1, Macros.pip White]) []
       (MkTypeLine [] [Instant])
       [ Spell (Continuously
                  (Prevents AnyDamage (TheNext XVal)
                            (Macros.shieldingIt (Macros.target Macros.creature))
                            Nothing
                            (Just (PutCounters PreventedThisWay
                                               Macros.plusOnePlusOne
                                               (That (TypeW Creature)))))
                  (Just Macros.thisTurn)) ]
       Nothing

public export
candlesGlow : Card
candlesGlow =
  Macros.card "Candles' Glow" (Just [Macros.generic 1, Macros.pip White]) []
       (MkTypeLine [Arcane] [Instant])
       [ Spell (Continuously
                  (Prevents AnyDamage (TheNext (Lit 3))
                            (Macros.shieldingIt (Macros.target AnyTarget))
                            Nothing
                            (Just (ChangeLife You (Up PreventedThisWay))))
                  (Just Macros.thisTurn)) ]
       Nothing

||| Inkshield
inkshieldRider : Effect []
inkshieldRider =
  Continuously
    (Prevents AnyDamage AllOfIt (Macros.shieldingIt You) Nothing
              (Just (Macros.create PreventedThisWay
                                   (Macros.creatureTok 2 1 [White, Black] []))))
    (Just Macros.thisTurn)

public export
urzasArmor : Card
urzasArmor =
  Macros.card "Urza's Armor" (Just [Macros.generic 6]) []
       (MkTypeLine [] [Artifact])
       [ Static (PreventsFrom AnyDamage (DealtBy (Macros.a Macros.source))
                              (Macros.shieldingIt You)
                              (CutSome (Lit 1)) Repeatedly Nothing) ]
       Nothing

public export
circleOfProtectionRed : Card
circleOfProtectionRed =
  Macros.card "Circle of Protection: Red"
       (Just [Macros.generic 1, Macros.pip White]) []
       (MkTypeLine [] [Enchantment])
       [ Activated (Mana [Macros.generic 1])
                   (Continuously
                      (PreventsFrom AnyDamage
                                    (DealtBy (Macros.aYourChoice
                                                (And [Macros.source, ColorIs Red])))
                                    (Macros.shieldingIt You)
                                    CutAll NextTimeOnly Nothing)
                      (Just Macros.thisTurn)) ]
       Nothing

public export
healingGrace : Card
healingGrace =
  Macros.card "Healing Grace" (Just [Macros.pip White]) []
       (MkTypeLine [] [Instant])
       [ Spell (Sequentially
                  [ Continuously
                      (Prevents AnyDamage (TheNext (Lit 3))
                                (Macros.shieldingIt (Macros.target AnyTarget))
                                (Just (Macros.aYourChoice Macros.source)) Nothing)
                      (Just Macros.thisTurn)
                  , ChangeLife You (Up (Lit 3)) ]) ]
       Nothing

public export
reverseDamage : Card
reverseDamage =
  Macros.card "Reverse Damage"
       (Just [Macros.generic 1, Macros.pip White, Macros.pip White]) []
       (MkTypeLine [] [Instant])
       [ Spell (Continuously
                  (PreventsFrom AnyDamage
                                (DealtBy (Macros.aYourChoice Macros.source))
                                (Macros.shieldingIt You) CutAll NextTimeOnly
                                (Just (ChangeLife You (Up PreventedThisWay))))
                  (Just Macros.thisTurn)) ]
       Nothing

public export
deflectingPalm : Card
deflectingPalm =
  Macros.card "Deflecting Palm" (Just [Macros.pip Red, Macros.pip White]) []
       (MkTypeLine [] [Instant])
       [ Spell (Continuously
                  (PreventsFrom AnyDamage
                                (DealtBy (Macros.aYourChoice Macros.source))
                                (Macros.shieldingIt You) CutAll NextTimeOnly
                                (Just (DealDamage This ThatMuch (ControllerOf It))))
                  (Just Macros.thisTurn)) ]
       Nothing

public export
sphereOfLaw : Card
sphereOfLaw =
  Macros.card "Sphere of Law" (Just [Macros.generic 3, Macros.pip White]) []
       (MkTypeLine [] [Enchantment])
       [ Static (PreventsFrom AnyDamage
                              (DealtBy (Macros.a (And [Macros.source, ColorIs Red])))
                              (Macros.shieldingIt You)
                              (CutSome (Lit 2)) Repeatedly Nothing) ]
       Nothing

||| Steelswarm Operator
steelswarmOperatorMana : Effect []
steelswarmOperatorMana =
  AddMana You (Lit 2) (Runs [[OfColor Blue, OfColor Blue]])
          [SpendOnly [ToActivate (Just (And [Macros.source, Macros.artifact]))]]

public export
lavaAxe : Card
lavaAxe =
  Macros.card "Lava Axe" (Just [Macros.generic 4, Macros.pip Red]) []
       (MkTypeLine [] [Sorcery])
       [ Spell (DealDamage This (Lit 5)
                           (Macros.target (KindJoin JoinAnyPlayer
                                                    JoinPlaneswalker))) ]
       Nothing

public export
searingFlesh : Card
searingFlesh =
  Macros.card "Searing Flesh" (Just [Macros.generic 6, Macros.pip Red]) []
       (MkTypeLine [] [Sorcery])
       [ Spell (DealDamage This (Lit 7)
                           (Macros.target (KindJoin JoinOpponent
                                                    JoinPlaneswalker))) ]
       Nothing

public export
onakkeJavelineerBolt : Ability
onakkeJavelineerBolt =
  Activated TapSymbol
            (DealDamage Macros.thisCreature (Lit 2)
                        (Macros.target (KindJoin JoinAnyPlayer JoinBattle)))

public export
endure : Card
endure =
  Macros.card "Endure"
       (Just [Macros.generic 3, Macros.pip White, Macros.pip White]) []
       (MkTypeLine [] [Instant])
       [ Spell (Macros.preventAll AnyDamage
                                  (Macros.shieldingIt
                                     (YouAnd (AllOf (And [Permanent,
                                                          ControlledBy You]))))
                                  (Just Macros.thisTurn)) ]
       Nothing

public export
harmsWay : Card
harmsWay =
  Macros.card "Harm's Way" (Just [Macros.pip White]) []
       (MkTypeLine [] [Instant])
       [ Spell (Continuously
                  (Redirects AnyDamage (TheNext (Lit 2))
                             (Macros.shieldingIt
                                (YouAnd (AllOf (And [Permanent,
                                                     ControlledBy You]))))
                             (Just (Macros.aYourChoice Macros.source))
                             (Macros.target AnyTarget))
                  (Just Macros.thisTurn)) ]
       Nothing

public export
divineDeflection : Card
divineDeflection =
  Macros.card "Divine Deflection" (Just [Variable, Macros.pip White]) []
       (MkTypeLine [] [Instant])
       [ Spell (Continuously
                  (Prevents AnyDamage (TheNext XVal)
                            (Macros.shieldingIt
                               (YouAnd (AllOf (And [Permanent,
                                                    ControlledBy You]))))
                            Nothing
                            (Just (DealDamage This ThatMuch
                                              (Macros.target AnyTarget))))
                  (Just Macros.thisTurn)) ]
       Nothing

public export
glarecasterShield : Ability
glarecasterShield =
  Activated (Mana [Macros.generic 5, Macros.pip White])
            (Continuously
               (RedirectsFrom AnyDamage Unattributed
                              (Macros.shieldingIt (YouAnd Macros.thisCreature))
                              (Macros.target AnyTarget)
                              NextTimeOnly)
               (Just Macros.thisTurn))

public export
furnaceOfRath : Card
furnaceOfRath =
  Macros.card "Furnace of Rath"
       (Just [Macros.generic 1, Macros.pip Red, Macros.pip Red,
              Macros.pip Red]) []
       (MkTypeLine [] [Enchantment])
       [ Static (Scales AnyDamage (Macros.a Macros.source)
                        (Macros.shieldingIt
                           (Macros.a (KindJoin JoinAnyPlayer JoinPermanent)))
                        (Multiplied Doubled) Repeatedly) ]
       Nothing

public export
gratuitousViolence : Card
gratuitousViolence =
  Macros.card "Gratuitous Violence"
       (Just [Macros.generic 2, Macros.pip Red, Macros.pip Red,
              Macros.pip Red]) []
       (MkTypeLine [] [Enchantment])
       [ Static (Scales AnyDamage (Macros.a Macros.creatureYouControl)
                        (Macros.shieldingIt
                           (Macros.a (KindJoin JoinAnyPlayer JoinPermanent)))
                        (Multiplied Doubled) Repeatedly) ]
       Nothing

public export
fieryEmancipation : Card
fieryEmancipation =
  Macros.card "Fiery Emancipation"
       (Just [Macros.generic 3, Macros.pip Red, Macros.pip Red,
              Macros.pip Red]) []
       (MkTypeLine [] [Enchantment])
       [ Static (Scales AnyDamage
                        (Macros.a (And [Macros.source, ControlledBy You]))
                        (Macros.shieldingIt
                           (Macros.a (KindJoin JoinAnyPlayer JoinPermanent)))
                        (Multiplied Tripled) Repeatedly) ]
       Nothing

public export
sulfuricVapors : Card
sulfuricVapors =
  Macros.card "Sulfuric Vapors"
       (Just [Macros.generic 3, Macros.pip Red]) []
       (MkTypeLine [] [Enchantment])
       [ Static (Scales AnyDamage
                        (Macros.a (And [Macros.spell, ColorIs Red]))
                        (Macros.shieldingIt
                           (Macros.a (KindJoin JoinAnyPlayer JoinPermanent)))
                        (Shifted ShiftUp (Lit 1)) Repeatedly) ]
       Nothing

public export
lashknifeBarrier : Card
lashknifeBarrier =
  Macros.card "Lashknife Barrier"
       (Just [Macros.generic 2, Macros.pip White]) []
       (MkTypeLine [] [Enchantment])
       [ Triggered When (Enters Macros.thisEnchantment) Macros.drawACard
       , Static (Scales AnyDamage (Macros.a Macros.source)
                        (Macros.shieldingIt
                           (Macros.a Macros.creatureYouControl))
                        (Shifted ShiftDown (Lit 1)) Repeatedly) ]
       Nothing

public export
ghostsOfTheInnocent : Card
ghostsOfTheInnocent =
  Macros.card "Ghosts of the Innocent"
       (Just [Macros.generic 5, Macros.pip White, Macros.pip White]) []
       (MkTypeLine [Spirit] [Creature])
       [ Static (Scales AnyDamage (Macros.a Macros.source)
                        (Macros.shieldingIt
                           (Macros.a (KindJoin JoinAnyPlayer JoinPermanent)))
                        (Halved RoundDown) Repeatedly) ]
       (Just (4, 5))

public export
fireServant : Card
fireServant =
  Macros.card "Fire Servant"
       (Just [Macros.generic 3, Macros.pip Red, Macros.pip Red]) []
       (MkTypeLine [Elemental] [Creature])
       [ Static (Scales AnyDamage
                        (Macros.a (And [Macros.instantOrSorcery, ColorIs Red,
                                        ControlledBy You]))
                        Everywhere (Multiplied Doubled) Repeatedly) ]
       (Just (4, 3))

public export
blastOfGenius : Card
blastOfGenius =
  Macros.card "Blast of Genius"
       (Just [Macros.generic 4, Macros.pip Blue, Macros.pip Red]) []
       (MkTypeLine [] [Sorcery])
       [ Spell (Sequentially
           [ Choose (Macros.target AnyTarget)
           , Draw You (Lit 3)
           , Macros.discards You (Macros.a (InZone Macros.handZ))
           , DealDamage This (Macros.manaValueOf (TheVerbed Discard CardW))
                        (That JoinW) ]) ]
       Nothing

public export
riddleOfLightning : Card
riddleOfLightning =
  Macros.card "Riddle of Lightning"
       (Just [Macros.generic 3, Macros.pip Red, Macros.pip Red]) []
       (MkTypeLine [] [Instant])
       [ Spell (Sequentially
           [ Choose (Macros.target AnyTarget)
           , Does You Scry (Macros.lookAt (Macros.topCards 3))
           , Macros.revealCards Macros.topCard
           , DealDamage This (Macros.manaValueOf (That CardW)) (That JoinW) ]) ]
       Nothing

public export
platedPegasus : Card
platedPegasus =
  Macros.card "Plated Pegasus" (Just [Macros.generic 2, Macros.pip White]) []
       (MkTypeLine [Pegasus] [Creature])
       [ KeywordAbility Flash
       , KeywordAbility Flying
       , Static (PreventsFrom AnyDamage
                              (DealtBy (Macros.a Macros.spell))
                              (ToRecipient
                                 (Macros.a (KindJoin JoinAnyPlayer JoinPermanent)))
                              (CutSome (Lit 1))
                              Repeatedly
                              Nothing) ]
       (Just (1, 2))

public export
unstableShapeshifter : Card
unstableShapeshifter =
  Macros.card "Unstable Shapeshifter"
       (Just [Macros.generic 3, Macros.pip Blue]) []
       (MkTypeLine [Shapeshifter] [Creature])
       [ Triggered Whenever
                   (Enters (Macros.a (And [Macros.creature, OtherThan This])))
                   (Continuously
                      (BecomesCopy Macros.thisCreature (That (TypeW Creature))
                                   [ExceptThisAbility])
                      Nothing) ]
       (Just (0, 1))

public export
tranquilGrove : Card
tranquilGrove =
  Macros.card "Tranquil Grove"
       (Just [Macros.generic 1, Macros.pip Green]) []
       (MkTypeLine [] [Enchantment])
       [ Activated (Mana [Macros.generic 1, Macros.pip Green, Macros.pip Green])
                   (Macros.destroy
                      (AllOf (And [Macros.enchantment, OtherThan This]))) ]
       Nothing

public export
wordsOfWisdom : Card
wordsOfWisdom =
  Macros.card "Words of Wisdom"
       (Just [Macros.generic 1, Macros.pip Blue]) []
       (MkTypeLine [] [Instant])
       [ Spell (Sequentially
                  [ Draw You (Lit 2)
                  , Draw (Each (And [AnyPlayer, OtherThan You])) (Lit 1) ]) ]
       Nothing


public export
spitemare : Card
spitemare =
  Macros.card "Spitemare"
       (Just [Macros.generic 2, Macros.hybridPip Red White,
              Macros.hybridPip Red White]) []
       (MkTypeLine [Elemental] [Creature])
       [ Triggered Whenever
                   (IsDealtDamage Macros.thisCreature)
                   (DealDamage It ThatMuch (Macros.target AnyTarget)) ]
       (Just (3, 3))

public export
grollub : Card
grollub =
  Macros.card "Grollub" (Just [Macros.generic 2, Macros.pip Black]) []
       (MkTypeLine [Beast] [Creature])
       [ Triggered Whenever
                   (IsDealtDamage Macros.thisCreature)
                   (Macros.gainsLife (Each Opponent) ThatMuch) ]
       (Just (3, 3))

public export
moggManiac : Card
moggManiac =
  Macros.card "Mogg Maniac" (Just [Macros.generic 1, Macros.pip Red]) []
       (MkTypeLine [Goblin] [Creature])
       [ Triggered Whenever
                   (IsDealtDamage Macros.thisCreature)
                   (DealDamage It ThatMuch
                               (Macros.target (KindJoin JoinOpponent JoinPlaneswalker))) ]
       (Just (1, 1))

public export
repercussion : Card
repercussion =
  Macros.card "Repercussion"
       (Just [Macros.generic 1, Macros.pip Red, Macros.pip Red]) []
       (MkTypeLine [] [Enchantment])
       [ Triggered Whenever
                   (IsDealtDamage (Macros.a Macros.creature))
                   (DealDamage Macros.thisEnchantment ThatMuch
                               (ControllerOf (That (TypeW Creature)))) ]
       Nothing

public export
spitefulShadows : Card
spitefulShadows =
  Macros.card "Spiteful Shadows" (Just [Macros.generic 1, Macros.pip Black]) []
       (MkTypeLine [Aura] [Enchantment])
       [ Macros.keywordSubject Enchant Macros.creature
       , Triggered Whenever
                   (IsDealtDamage (AttachHost Enchanted (TypeW Creature)))
                   (DealDamage It ThatMuch (ControllerOf It)) ]
       Nothing

public export
bindingAgony : Card
bindingAgony =
  Macros.card "Binding Agony" (Just [Macros.generic 1, Macros.pip Black]) []
       (MkTypeLine [Aura] [Enchantment])
       [ Macros.keywordSubject Enchant Macros.creature
       , Triggered Whenever
                   (IsDealtDamage (AttachHost Enchanted (TypeW Creature)))
                   (DealDamage Macros.thisAura ThatMuch
                               (ControllerOf (That (TypeW Creature)))) ]
       Nothing

public export
darienKingOfKjeldor : Card
darienKingOfKjeldor =
  Macros.card "Darien, King of Kjeldor"
       (Just [Macros.generic 4, Macros.pip White, Macros.pip White]) [Legendary]
       (MkTypeLine [Human, Soldier] [Creature])
       [ Triggered Whenever
                   (IsDealtDamage You)
                   (Macros.may You
                      (Macros.create ThatMuch
                                     (Macros.creatureTok 1 1 [White] [Soldier]))) ]
       (Just (3, 3))

public export
livingHive : Card
livingHive =
  Macros.card "Living Hive"
       (Just [Macros.generic 6, Macros.pip Green, Macros.pip Green]) []
       (MkTypeLine [Elemental, Insect] [Creature])
       [ KeywordAbility Trample
       , Triggered Whenever
                   (DealsCombatDamage Macros.thisCreature (Macros.a AnyPlayer))
                   (Macros.create ThatMuch
                                  (Macros.creatureTok 1 1 [Green] [Insect])) ]
       (Just (6, 6))

public export
screamingNemesisTrigger : Ability
screamingNemesisTrigger =
  Triggered Whenever
            (IsDealtDamage Macros.thisCreature)
            (DealDamage It ThatMuch
                        (Macros.target (And [AnyTarget, OtherThan This])))


public export
callInAProfessional : Card
callInAProfessional =
  Macros.card "Call In a Professional"
       (Just [Macros.generic 2, Macros.pip Red]) []
       (MkTypeLine [] [Instant])
       [ Spell (Sequentially
                  [ Continuously
                      (PlayerCant GainsLife (PlayerGroup AllPlayers))
                      (Just Macros.thisTurn)
                  , Continuously
                      (CantPrevent AnyDamage Everywhere Nothing)
                      (Just Macros.thisTurn)
                  , DealDamage This (Lit 3) (Macros.target AnyTarget) ]) ]
       Nothing

public export
giantCindermaw : Card
giantCindermaw =
  Macros.card "Giant Cindermaw" (Just [Macros.generic 2, Macros.pip Red]) []
       (MkTypeLine [Dinosaur, Beast] [Creature])
       [ KeywordAbility Trample
       , Static (PlayerCant GainsLife (PlayerGroup AllPlayers)) ]
       (Just (4, 3))

public export
mindlockOrb : Card
mindlockOrb =
  Macros.card "Mindlock Orb" (Just [Macros.generic 3, Macros.pip Blue]) []
       (MkTypeLine [] [Artifact])
       [ Static (PlayerCant SearchesLibraries (PlayerGroup AllPlayers)) ]
       Nothing

public export
silence : Card
silence =
  Macros.card "Silence" (Just [Macros.pip White]) []
       (MkTypeLine [] [Instant])
       [ Spell (Continuously
                  (PlayerCant CastsSpells (PlayerGroup YourOpponents))
                  (Just Macros.thisTurn)) ]
       Nothing

public export
shadowOfDoubt : Card
shadowOfDoubt =
  Macros.card "Shadow of Doubt"
       (Just [Macros.hybridPip Blue Black, Macros.hybridPip Blue Black]) []
       (MkTypeLine [] [Instant])
       [ Spell (Sequentially
                  [ Continuously
                      (PlayerCant SearchesLibraries (PlayerGroup AllPlayers))
                      (Just Macros.thisTurn)
                  , Macros.drawACard ]) ]
       Nothing

public export
aggressiveMining : Card
aggressiveMining =
  Macros.card "Aggressive Mining"
       (Just [Macros.generic 3, Macros.pip Red]) []
       (MkTypeLine [] [Enchantment])
       [ Static (PlayerCant PlaysLands You)
       , Macros.activatedOnlyOnce (Do (Macros.sacrifice You (Macros.a Macros.land)))
                                  (Draw You (Lit 2))
                                  OncePerTurn ]
       Nothing

public export
omenMachineDraw : Ability
omenMachineDraw = Static (PlayerCant DrawsCards (PlayerGroup AllPlayers))

public export
grievousWoundLifeLock : Card
grievousWoundLifeLock =
  Macros.card "Grievous Wound"
       (Just [Macros.generic 3, Macros.pip Black, Macros.pip Black]) []
       (MkTypeLine [Aura] [Enchantment])
       [ Macros.keywordSubject Enchant AnyPlayer
       , Static (PlayerCant GainsLife (AttachHost Enchanted PlayerW)) ]
       Nothing

public export
solfataraLandLock : Effect []
solfataraLandLock =
  Continuously (PlayerCant PlaysLands (Macros.target AnyPlayer))
               (Just Macros.thisTurn)


public export
bloodMoon : Card
bloodMoon =
  Macros.card "Blood Moon" (Just [Macros.generic 2, Macros.pip Red]) []
       (MkTypeLine [] [Enchantment])
       [ Static (SetsType (AllOf (And [Macros.land, Not (HasSupertype Basic)]))
                          (MkToken Nothing [] (Macros.basicLandLine [Mountain])
                                   [] Nothing)
                          Nothing) ]
       Nothing

public export
magusOfTheMoon : Card
magusOfTheMoon =
  Macros.card "Magus of the Moon" (Just [Macros.generic 2, Macros.pip Red]) []
       (MkTypeLine [Human, Wizard] [Creature])
       [ Static (SetsType (AllOf (And [Macros.land, Not (HasSupertype Basic)]))
                          (MkToken Nothing [] (Macros.basicLandLine [Mountain])
                                   [] Nothing)
                          Nothing) ]
       (Just (2, 2))

public export
harbingerOfTheSeas : Card
harbingerOfTheSeas =
  Macros.card "Harbinger of the Seas"
       (Just [Macros.generic 1, Macros.pip Blue, Macros.pip Blue]) []
       (MkTypeLine [Merfolk, Wizard] [Creature])
       [ Static (SetsType (AllOf (And [Macros.land, Not (HasSupertype Basic)]))
                          (MkToken Nothing [] (Macros.basicLandLine [Island])
                                   [] Nothing)
                          Nothing) ]
       (Just (2, 2))

public export
lushGrowth : Card
lushGrowth =
  Macros.card "Lush Growth" (Just [Macros.pip Green]) []
       (MkTypeLine [Aura] [Enchantment])
       [ Macros.keywordSubject Enchant Macros.land
       , Static (SetsType (AttachHost Enchanted (TypeW Land))
                          (MkToken Nothing []
                                   (Macros.basicLandLine [Mountain, Forest, Plains])
                                   [] Nothing)
                          Nothing) ]
       Nothing

public export
yavimayaCradleOfGrowth : Card
yavimayaCradleOfGrowth =
  Macros.card "Yavimaya, Cradle of Growth" Nothing [Legendary]
       (MkTypeLine [] [Land])
       [ Static (BecomesAlso (AllOf Macros.land)
                             (MkToken Nothing [] (Macros.basicLandLine [Forest]) [] Nothing)) ]
       Nothing

public export
reefShaman : Card
reefShaman =
  Macros.card "Reef Shaman" (Just [Macros.pip Blue]) []
       (MkTypeLine [Merfolk, Shaman] [Creature])
       [ Activated TapSymbol
                   (Continuously (SetsChosenBasicType (Macros.target Macros.land))
                                 (Just Macros.untilEndOfTurn)) ]
       (Just (0, 2))

public export
grixisIllusionist : Card
grixisIllusionist =
  Macros.card "Grixis Illusionist" (Just [Macros.pip Blue]) []
       (MkTypeLine [Human, Wizard] [Creature])
       [ Activated TapSymbol
                   (Continuously
                      (SetsChosenBasicType
                         (Macros.target (And [Macros.land, ControlledBy You])))
                      (Just Macros.untilEndOfTurn)) ]
       (Just (1, 1))

public export
unstableFrontier : Card
unstableFrontier =
  Macros.card "Unstable Frontier" Nothing [] (MkTypeLine [] [Land])
       [ Activated TapSymbol (AddMana You (Lit 1) (Runs [[Colorless]]) [])
       , Activated TapSymbol
                   (Continuously
                      (SetsChosenBasicType
                         (Macros.target (And [Macros.land, ControlledBy You])))
                      (Just Macros.untilEndOfTurn)) ]
       Nothing


public export
extinction : Card
extinction =
  Macros.card "Extinction" (Just [Macros.generic 4, Macros.pip Black]) []
       (MkTypeLine [] [Sorcery])
       [ Spell (Macros.destroy (AllOf (And [Macros.creature,
                                            OfYourChoice CreatureType]))) ]
       Nothing

public export
defensiveManeuvers : Card
defensiveManeuvers =
  Macros.card "Defensive Maneuvers"
       (Just [Macros.generic 3, Macros.pip White]) []
       (MkTypeLine [] [Instant])
       [ Spell (Macros.gets (AllOf (And [Macros.creature,
                                         OfYourChoice CreatureType]))
                            (PtUp (Lit 0)) (PtUp (Lit 4))
                            (Just Macros.untilEndOfTurn)) ]
       Nothing

public export
witchsVengeance : Card
witchsVengeance =
  Macros.card "Witch's Vengeance"
       (Just [Macros.generic 1, Macros.pip Black, Macros.pip Black]) []
       (MkTypeLine [] [Sorcery])
       [ Spell (Macros.gets (AllOf (And [Macros.creature,
                                         OfYourChoice CreatureType]))
                            (PtDown (Lit 3)) (PtDown (Lit 3))
                            (Just Macros.untilEndOfTurn)) ]
       Nothing

public export
rootGreevil : Card
rootGreevil =
  Macros.card "Root Greevil" (Just [Macros.generic 3, Macros.pip Green]) []
       (MkTypeLine [Beast] [Creature])
       [ Activated (Compound [Mana [Macros.generic 2, Macros.pip Green],
                              TapSymbol,
                              Do (Macros.sacrifice You Macros.thisCreature)])
                   (Macros.destroy (AllOf (And [HasType Enchantment,
                                                OfYourChoice Color]))) ]
       (Just (2, 3))

public export
riptideChronologist : Card
riptideChronologist =
  Macros.card "Riptide Chronologist"
       (Just [Macros.generic 3, Macros.pip Blue, Macros.pip Blue]) []
       (MkTypeLine [Human, Wizard] [Creature])
       [ Activated (Compound [Mana [Macros.pip Blue],
                              Do (Macros.sacrifice You Macros.thisCreature)])
                   (SetStatus Untapped
                      (AllOf (And [Macros.creature,
                                   OfYourChoice CreatureType]))) ]
       (Just (1, 3))

public export
distantMelody : Card
distantMelody =
  Macros.card "Distant Melody" (Just [Macros.generic 3, Macros.pip Blue]) []
       (MkTypeLine [] [Sorcery])
       [ Spell (Sequentially
                  [ Choose (Macros.a (QualityNoun CreatureType))
                  , Draw You (Macros.forEach
                                (And [Permanent, ControlledBy You,
                                      OfChosen CreatureType])) ]) ]
       Nothing

public export
cripplingFear : Card
cripplingFear =
  Macros.card "Crippling Fear"
       (Just [Macros.generic 2, Macros.pip Black, Macros.pip Black]) []
       (MkTypeLine [] [Sorcery])
       [ Spell (Sequentially
                  [ Choose (Macros.a (QualityNoun CreatureType))
                  , Macros.gets (AllOf (And [Macros.creature,
                                             Not (OfChosen CreatureType)]))
                                (PtDown (Lit 3)) (PtDown (Lit 3))
                                (Just Macros.untilEndOfTurn) ]) ]
       Nothing


public export
rallyTheRanks : Card
rallyTheRanks =
  Macros.card "Rally the Ranks" (Just [Macros.generic 1, Macros.pip White]) []
       (MkTypeLine [] [Enchantment])
       [ Static (EntersChoice Macros.thisEnchantment CreatureType)
       , Static (Gets (AllOf (And [Macros.creature, ControlledBy You,
                                   OfChosen CreatureType]))
                      (PtUp (Lit 1)) (PtUp (Lit 1))) ]
       Nothing

public export
sharedTriumph : Card
sharedTriumph =
  Macros.card "Shared Triumph" (Just [Macros.generic 1, Macros.pip White]) []
       (MkTypeLine [] [Enchantment])
       [ Static (EntersChoice Macros.thisEnchantment CreatureType)
       , Static (Gets (AllOf (And [Macros.creature, OfChosen CreatureType]))
                      (PtUp (Lit 1)) (PtUp (Lit 1))) ]
       Nothing

public export
hallOfTriumph : Card
hallOfTriumph =
  Macros.card "Hall of Triumph" (Just [Macros.generic 3]) [Legendary]
       (MkTypeLine [] [Artifact])
       [ Static (EntersChoice Macros.thisArtifact Color)
       , Static (Gets (AllOf (And [Macros.creature, ControlledBy You,
                                   OfChosen Color]))
                      (PtUp (Lit 1)) (PtUp (Lit 1))) ]
       Nothing

public export
engineeredPlague : Card
engineeredPlague =
  Macros.card "Engineered Plague"
       (Just [Macros.generic 2, Macros.pip Black]) []
       (MkTypeLine [] [Enchantment])
       [ Static (EntersChoice Macros.thisEnchantment CreatureType)
       , Static (Gets (AllOf (And [Macros.creature, OfChosen CreatureType]))
                      (PtDown (Lit 1)) (PtDown (Lit 1))) ]
       Nothing

public export
prismRing : Card
prismRing =
  Macros.card "Prism Ring" (Just [Macros.generic 1]) []
       (MkTypeLine [] [Artifact])
       [ Static (EntersChoice Macros.thisArtifact Color)
       , Triggered Whenever
                   (Casts You (Macros.a (And [Macros.spell, OfChosen Color])))
                   (Macros.gainsLife You (Lit 1)) ]
       Nothing

public export
urzasIncubator : Card
urzasIncubator =
  Macros.card "Urza's Incubator" (Just [Macros.generic 3]) []
       (MkTypeLine [] [Artifact])
       [ Static (EntersChoice Macros.thisArtifact CreatureType)
       , Static (CostsToCast (AllOf (And [Macros.creature, Macros.spell,
                                          OfChosen CreatureType]))
                             (CostLess (Lit 2))) ]
       Nothing

public export
etchingsOfTheChosen : Card
etchingsOfTheChosen =
  Macros.card "Etchings of the Chosen"
       (Just [Macros.generic 1, Macros.pip White, Macros.pip Black]) []
       (MkTypeLine [] [Enchantment])
       [ Static (EntersChoice Macros.thisEnchantment CreatureType)
       , Static (Gets (AllOf (And [Macros.creature, ControlledBy You,
                                   OfChosen CreatureType]))
                      (PtUp (Lit 1)) (PtUp (Lit 1)))
       , Activated (Compound [Mana [Macros.generic 1],
                              Do (Macros.sacrifice You
                                    (Macros.a (And [Macros.creature,
                                                    OfChosen CreatureType])))])
                   (Macros.gains (Macros.target Macros.creatureYouControl)
                                 (KeywordAbility Indestructible)
                                 (Just Macros.untilEndOfTurn)) ]
       Nothing

public export
voiceOfAll : Card
voiceOfAll =
  Macros.card "Voice of All"
       (Just [Macros.generic 2, Macros.pip White, Macros.pip White]) []
       (MkTypeLine [Angel] [Creature])
       [ KeywordAbility Flying
       , Static (EntersChoice Macros.thisCreature Color)
       , Static (Gains Macros.thisCreature
                       (Macros.keywordQuality Protection (OfChosen Color))) ]
       (Just (2, 2))

public export
wardSliver : Card
wardSliver =
  Macros.card "Ward Sliver" (Just [Macros.generic 4, Macros.pip White]) []
       (MkTypeLine [Sliver] [Creature])
       [ Static (EntersChoice Macros.thisCreature Color)
       , Static (Gains (AllOf (HasSubtype Sliver))
                       (Macros.keywordQuality Protection (OfChosen Color))) ]
       (Just (2, 2))

public export
chromaticArmor : Card
chromaticArmor =
  Macros.card "Chromatic Armor"
       (Just [Macros.generic 1, Macros.pip White, Macros.pip Blue]) []
       (MkTypeLine [Aura] [Enchantment])
       [ Macros.keywordSubject Enchant Macros.creature
       , Static (EntersChoice Macros.thisAura Color)
       , Static (Prevents AnyDamage AllOfIt
                          (Macros.shieldingIt (AttachHost Enchanted (TypeW Creature)))
                          (Just (AllOf (And [Macros.source, OfLastChosenColor])))
                          Nothing) ]
       Nothing

public export
xenograft : Card
xenograft =
  Macros.card "Xenograft" (Just [Macros.generic 4, Macros.pip Blue]) []
       (MkTypeLine [] [Enchantment])
       [ Static (EntersChoice Macros.thisEnchantment CreatureType)
       , Static (AddsChosenQuality (AllOf (And [Macros.creature, ControlledBy You]))
                                (OfChosen CreatureType)) ]
       Nothing

public export
adaptiveAutomaton : Card
adaptiveAutomaton =
  Macros.card "Adaptive Automaton" (Just [Macros.generic 3]) []
       (MkTypeLine [Construct] [Artifact, Creature])
       [ Static (EntersChoice Macros.thisCreature CreatureType)
       , Static (AddsChosenQuality Macros.thisCreature (OfChosen CreatureType))
       , Static (Gets (AllOf (And [Macros.creature, ControlledBy You,
                                   OtherThan Macros.thisCreature,
                                   OfChosen CreatureType]))
                      (PtUp (Lit 1)) (PtUp (Lit 1))) ]
       (Just (2, 2))

public export
mistformDreamer : Card
mistformDreamer =
  Macros.card "Mistform Dreamer" (Just [Macros.generic 2, Macros.pip Blue]) []
       (MkTypeLine [Illusion] [Creature])
       [ KeywordAbility Flying
       , Activated (Mana [Macros.generic 1])
                   (Continuously
                      (SetsChosenQuality Macros.thisCreature
                                      (OfYourChoice CreatureType))
                      (Just Macros.untilEndOfTurn)) ]
       (Just (2, 1))

public export
arcaneAdaptation : Card
arcaneAdaptation =
  Macros.card "Arcane Adaptation" (Just [Macros.generic 2, Macros.pip Blue]) []
       (MkTypeLine [] [Enchantment])
       [ Static (EntersChoice Macros.thisEnchantment CreatureType)
       , Static (AlsoOffBattlefield
                   (AddsChosenQuality
                      (AllOf (And [Macros.creature, ControlledBy You]))
                      (OfChosen CreatureType))) ]
       Nothing

public export
conspiracy : Card
conspiracy =
  Macros.card "Conspiracy"
       (Just [Macros.generic 3, Macros.pip Black, Macros.pip Black]) []
       (MkTypeLine [] [Enchantment])
       [ Static (EntersChoice Macros.thisEnchantment CreatureType)
       , Static (AlsoOffBattlefield
                   (SetsChosenQuality
                      (AllOf (And [Macros.creature, ControlledBy You]))
                      (OfChosen CreatureType))) ]
       Nothing

public export
encroachingMycosynth : Card
encroachingMycosynth =
  Macros.card "Encroaching Mycosynth" (Just [Macros.generic 3, Macros.pip Blue]) []
       (MkTypeLine [] [Artifact])
       [ Static (AlsoOffBattlefield
                   (BecomesAlso (AllOf (And [Permanent, Not (HasType Land),
                                             ControlledBy You]))
                                (MkToken Nothing [] (MkTypeLine [] [Artifact]) [] Nothing))) ]
       Nothing

public export
declarationOfNaught : Card
declarationOfNaught =
  Macros.card "Declaration of Naught" (Just [Macros.pip Blue, Macros.pip Blue]) []
       (MkTypeLine [] [Enchantment])
       [ Static (EntersChoice Macros.thisEnchantment CardName)
       , Activated (Mana [Macros.pip Blue])
                   (Macros.counterSpell
                      (Macros.target (And [Macros.spell, Named ChosenName]))) ]
       Nothing

public export
cursedScroll : Card
cursedScroll =
  Macros.card "Cursed Scroll" (Just [Macros.generic 1]) []
       (MkTypeLine [] [Artifact])
       [ Activated (Compound [Mana [Macros.generic 3], TapSymbol])
                   (Sequentially
                      [ Choose (Macros.a (QualityNoun CardName))
                      , Macros.revealCards
                          (Macros.aAtRandom (InZone (Macros.handOf You)))
                      , If (DealDamage Macros.thisArtifact (Lit 2)
                              (Macros.target AnyTarget))
                           (Matches (That CardW) (Named ChosenName))
                           Nothing ]) ]
       Nothing

public export
magusOfTheScroll : Card
magusOfTheScroll =
  Macros.card "Magus of the Scroll" (Just [Macros.pip Red]) []
       (MkTypeLine [Human, Wizard] [Creature])
       [ Activated (Compound [Mana [Macros.generic 3], TapSymbol])
                   (Sequentially
                      [ Choose (Macros.a (QualityNoun CardName))
                      , Macros.revealCards
                          (Macros.aAtRandom (InZone (Macros.handOf You)))
                      , If (DealDamage Macros.thisCreature (Lit 2)
                              (Macros.target AnyTarget))
                           (Matches (That CardW) (Named ChosenName))
                           Nothing ]) ]
       (Just (1, 1))

public export
imagecrafter : Card
imagecrafter =
  Macros.card "Imagecrafter" (Just [Macros.pip Blue]) []
       (MkTypeLine [Human, Wizard] [Creature])
       [ Activated TapSymbol
           (Sequentially
              [ Choose (Macros.a (Macros.qualityFrom CreatureType (TypeOtherThan Wall)))
              , Continuously (SetsChosenQuality (Macros.target Macros.creature)
                                                (OfChosen CreatureType))
                             (Just Macros.untilEndOfTurn) ]) ]
       (Just (1, 1))

public export
unnaturalSelection : Card
unnaturalSelection =
  Macros.card "Unnatural Selection" (Just [Macros.generic 1, Macros.pip Blue]) []
       (MkTypeLine [] [Enchantment])
       [ Activated (Mana [Macros.generic 1])
           (Sequentially
              [ Choose (Macros.a (Macros.qualityFrom CreatureType (TypeOtherThan Wall)))
              , Continuously (SetsChosenQuality (Macros.target Macros.creature)
                                                (OfChosen CreatureType))
                             (Just Macros.untilEndOfTurn) ]) ]
       Nothing

public export
standardize : Card
standardize =
  Macros.card "Standardize" (Just [Macros.pip Blue, Macros.pip Blue]) []
       (MkTypeLine [] [Instant])
       [ Spell (Sequentially
              [ Choose (Macros.a (Macros.qualityFrom CreatureType (TypeOtherThan Wall)))
              , Continuously (SetsChosenQuality (Each Macros.creature)
                                                (OfChosen CreatureType))
                             (Just Macros.untilEndOfTurn) ]) ]
       Nothing

public export
silverquillSilencer : Card
silverquillSilencer =
  Macros.card "Silverquill Silencer" (Just [Macros.pip White, Macros.pip Black]) []
       (MkTypeLine [Human, Cleric] [Creature])
       [ Static (Macros.entersChoosingFrom Macros.thisCreature
                                           CardName
                                           (NameOfCard (Not (HasType Land))))
       , Triggered Whenever
           (Casts Macros.anOpponent
                  (Macros.a (And [Macros.spell, Named ChosenName])))
           (Sequentially [ Macros.losesLife They (Lit 3), Macros.drawACard ]) ]
       (Just (3, 2))

public export
candlesOfLeng : Card
candlesOfLeng =
  Macros.card "Candles of Leng" (Just [Macros.generic 2]) []
       (MkTypeLine [] [Artifact])
       [ Activated (Compound [Mana [Macros.generic 4], TapSymbol])
           (Sequentially
              [ Macros.revealCards Macros.topCard
              , If (Move It Macros.graveyardZ)
                   (Matches It (Named (SameNameAs
                                  (Macros.a (InZone (Macros.graveyardOf You))))))
                   (Just Macros.drawACard) ]) ]
       Nothing

public export
avenShrine : Card
avenShrine =
  Macros.card "Aven Shrine"
       (Just [Macros.generic 1, Macros.pip White, Macros.pip White]) []
       (MkTypeLine [] [Enchantment])
       [ Triggered Whenever
           (Casts (Macros.a AnyPlayer) (Macros.a Macros.spell))
           (WhereLetter LetterX
              (CountOf (And [InZone Macros.graveyardZ,
                             Named (SameNameAs (That SpellW))]))
              (Macros.gainsLife They XVal)) ]
       Nothing

public export
bifurcate : Card
bifurcate =
  Macros.card "Bifurcate" (Just [Macros.generic 3, Macros.pip Green]) []
       (MkTypeLine [] [Sorcery])
       [ Spell (Sequentially
              [ Macros.searchLibraryFor
                  (And [Permanent, Named (SameNameAs
                          (Macros.target (And [Macros.creature, Macros.nontoken])))])
              , Macros.putOntoBattlefield (That CardW)
              , Macros.shuffle ]) ]
       Nothing

public export
meddlingMage : Card
meddlingMage =
  Macros.card "Meddling Mage" (Just [Macros.pip White, Macros.pip Blue]) []
       (MkTypeLine [Human, Wizard] [Creature])
       [ Static (Macros.entersChoosingFrom Macros.thisCreature
                                           CardName
                                           (NameOfCard (Not (HasType Land))))
       , Static (ObjectCant Cast
                   (AllOf (And [Macros.spell, Named ChosenName]))) ]
       (Just (2, 2))

public export
nevermore : Card
nevermore =
  Macros.card "Nevermore"
       (Just [Macros.generic 1, Macros.pip White, Macros.pip White]) []
       (MkTypeLine [] [Enchantment])
       [ Static (Macros.entersChoosingFrom Macros.thisEnchantment
                                           CardName
                                           (NameOfCard (Not (HasType Land))))
       , Static (ObjectCant Cast
                   (AllOf (And [Macros.spell, Named ChosenName]))) ]
       Nothing

public export
abruptDecay : Card
abruptDecay =
  Macros.card "Abrupt Decay" (Just [Macros.pip Black, Macros.pip Green]) []
       (MkTypeLine [] [Instant])
       [ Static (ObjectCant Countered This)
       , Spell (Macros.destroy
                  (Macros.target (And [Permanent, Not (HasType Land),
                                       Compare ManaValue AtMost (Lit 3)]))) ]
       Nothing

public export
dovinsVeto : Card
dovinsVeto =
  Macros.card "Dovin's Veto" (Just [Macros.pip White, Macros.pip Blue]) []
       (MkTypeLine [] [Instant])
       [ Static (ObjectCant Countered This)
       , Spell (Macros.counterSpell
                  (Macros.target (And [Macros.spell, Not (HasType Creature)]))) ]
       Nothing

public export
conjurersBan : Card
conjurersBan =
  Macros.card "Conjurer's Ban" (Just [Macros.pip White, Macros.pip Black]) []
       (MkTypeLine [] [Sorcery])
       [ Spell (Sequentially
           [ Choose (Macros.a (QualityNoun CardName))
           , Continuously
               (AndAlso [ ObjectCant Cast
                            (AllOf (And [Macros.spell, Named ChosenName]))
                        , ObjectCant Played
                            (AllOf (And [Macros.land, Named ChosenName])) ])
               (Just Macros.untilYourNextTurn) ])
       , Spell Macros.drawACard ]
       Nothing

public export
historyOfBenalia : Card
historyOfBenalia =
  Macros.card "History of Benalia"
       (Just [Macros.generic 1, Macros.pip White, Macros.pip White]) []
       (MkTypeLine [Saga] [Enchantment])
       [ Triggered When (ChapterMark [ChapterI, ChapterII])
           (Macros.create (Lit 1)
              (MkToken (Just (Lit 2, Lit 2)) [White]
                       (MkTypeLine [Knight] [Creature])
                       [KeywordAbility Vigilance] Nothing))
       , Triggered When (ChapterMark [ChapterIII])
           (Macros.gets (AllOf (And [HasSubtype Knight, ControlledBy You]))
                        (PtUp (Lit 2)) (PtUp (Lit 1))
                        (Just Macros.untilEndOfTurn)) ]
       Nothing

public export
vault75MiddleSchool : Card
vault75MiddleSchool =
  Macros.card "Vault 75: Middle School"
       (Just [Macros.generic 2, Macros.pip White, Macros.pip White]) []
       (MkTypeLine [Saga] [Enchantment])
       [ Triggered When (ChapterMark [ChapterI])
           (Macros.exile (AllOf (And [Macros.creature,
                                      Compare Power AtLeast (Lit 4)])))
       , Triggered When (ChapterMark [ChapterII, ChapterIII])
           (PutCounters (Lit 1) Macros.plusOnePlusOne
                        (Each Macros.creatureYouControl)) ]
       Nothing

public export
keldonWarcaller : Card
keldonWarcaller =
  Macros.card "Keldon Warcaller"
       (Just [Macros.generic 1, Macros.pip Red]) []
       (MkTypeLine [Human, Warrior] [Creature])
       [ Triggered Whenever (Attacks Macros.thisCreature)
           (PutCounters (Lit 1) Lore
                        (Macros.target (And [HasSubtype Saga, ControlledBy You]))) ]
       (Just (2, 2))

public export
foundingOfOmashu : Card
foundingOfOmashu =
  Macros.card "Founding of Omashu"
       (Just [Macros.generic 2, Macros.pip Red]) []
       (MkTypeLine [Saga] [Enchantment])
       [ Triggered When (ChapterMark [ChapterI])
           (Macros.create (Lit 2) (Macros.creatureTok 1 1 [White] [Ally]))
       , Triggered When (ChapterMark [ChapterII])
           (Macros.mayThen You (Macros.discardsACard You) Macros.drawACard)
       , Triggered When (ChapterMark [ChapterIII])
           (Macros.gets (AllOf Macros.creatureYouControl)
                        (PtUp (Lit 1)) (PtUp (Lit 0))
                        (Just Macros.untilEndOfTurn)) ]
       Nothing

public export
vedalkenOrrery : Card
vedalkenOrrery =
  Macros.card "Vedalken Orrery" (Just [Macros.generic 4]) []
       (MkTypeLine [] [Artifact])
       [ Static (Macros.mayCastAsThough You (AllOf Macros.spell) HadFlash) ]
       Nothing

public export
shimmerMyr : Card
shimmerMyr =
  Macros.card "Shimmer Myr" (Just [Macros.generic 3]) []
       (MkTypeLine [Myr] [Artifact, Creature])
       [ KeywordAbility Flash
       , Static (Macros.mayCastAsThough You
                                        (AllOf (And [Macros.spell, HasType Artifact]))
                                        HadFlash) ]
       (Just (2, 2))

public export
quickSliver : Card
quickSliver =
  Macros.card "Quick Sliver" (Just [Macros.generic 1, Macros.pip Green]) []
       (MkTypeLine [Sliver] [Creature])
       [ KeywordAbility Flash
       , Static (Macros.mayCastAsThough (Macros.a AnyPlayer)
                                        (AllOf (And [Macros.spell, HasSubtype Sliver]))
                                        HadFlash) ]
       (Just (1, 1))

public export
vernalEquinox : Card
vernalEquinox =
  Macros.card "Vernal Equinox"
       (Just [Macros.generic 3, Macros.pip Green]) []
       (MkTypeLine [] [Enchantment])
       [ Static (Macros.mayCastAsThough (Macros.a AnyPlayer)
                                        (AllOf (And [Macros.spell,
                                                     Or [HasType Creature, HasType Enchantment]]))
                                        HadFlash) ]
       Nothing

public export
borneUponAWind : Card
borneUponAWind =
  Macros.card "Borne Upon a Wind" (Just [Macros.generic 1, Macros.pip Blue]) []
       (MkTypeLine [] [Instant])
       [ Spell (Continuously
                  (Macros.mayCastAsThough You (AllOf Macros.spell) HadFlash)
                  (Just Macros.thisTurn))
       , Spell Macros.drawACard ]
       Nothing

public export
gisaAndGeralf : Card
gisaAndGeralf =
  Macros.card "Gisa and Geralf"
       (Just [Macros.generic 2, Macros.pip Blue, Macros.pip Black]) [Legendary]
       (MkTypeLine [Human, Wizard] [Creature])
       [ Triggered When (Enters Macros.thisCreature)
           (Macros.mills You (Lit 4) You)
       , Static (Macros.mayCastFromLimited You
                                           (Macros.a (And [Macros.spell, HasType Creature,
                                                           HasSubtype Zombie]))
                                           (Macros.graveyardOf You)
                                           OnceEachYourTurn) ]
       (Just (4, 4))

public export
castCreatureSpellsFromTop : StaticEffect []
castCreatureSpellsFromTop =
  Macros.mayCastFrom You (AllOf (And [Macros.spell, HasType Creature])) Macros.onTopZ

public export
playLandsAndCastSpellsFromTop : StaticEffect []
playLandsAndCastSpellsFromTop =
  AndAlso [ Macros.mayPlayFrom You (AllOf Macros.land) Macros.onTopZ
          , Macros.mayCastFrom You (AllOf Macros.spell) Macros.onTopZ ]

public export
playAndCastFromGraveyardThisTurn : Effect []
playAndCastFromGraveyardThisTurn =
  Continuously
    (AndAlso [ Macros.mayPlayFrom You (AllOf Macros.land) (Macros.graveyardOf You)
             , Macros.mayCastFrom You (AllOf Macros.spell) (Macros.graveyardOf You) ])
    (Just Macros.untilEndOfTurn)

public export
castSmallCreatureFromTopOnceEachTurn : StaticEffect []
castSmallCreatureFromTopOnceEachTurn =
  Macros.mayCastFromLimited You
                            (Macros.a (And [Macros.spell, HasType Creature,
                                            Compare Power AtMost (Lit 2)]))
                            Macros.onTopZ
                            OnceEachTurn

public export
goblinSpy : Card
goblinSpy =
  Macros.card "Goblin Spy" (Just [Macros.pip Red]) []
       (MkTypeLine [Goblin, Rogue] [Creature])
       [ Static (Visibility Reveal You TopOfLibrary) ]
       (Just (1, 1))

public export
garruksHorde : Card
garruksHorde =
  Macros.card "Garruk's Horde"
       (Just [Macros.generic 5, Macros.pip Green, Macros.pip Green]) []
       (MkTypeLine [Beast] [Creature])
       [ KeywordAbility Trample
       , Static (Visibility Reveal You TopOfLibrary)
       , Static castCreatureSpellsFromTop ]
       (Just (7, 7))

public export
futureSight : Card
futureSight =
  Macros.card "Future Sight"
       (Just [Macros.generic 2, Macros.pip Blue, Macros.pip Blue,
              Macros.pip Blue]) []
       (MkTypeLine [] [Enchantment])
       [ Static (Visibility Reveal You TopOfLibrary)
       , Static playLandsAndCastSpellsFromTop ]
       Nothing

public export
magusOfTheFuture : Card
magusOfTheFuture =
  Macros.card "Magus of the Future"
       (Just [Macros.generic 2, Macros.pip Blue, Macros.pip Blue,
              Macros.pip Blue]) []
       (MkTypeLine [Human, Wizard] [Creature])
       [ Static (Visibility Reveal You TopOfLibrary)
       , Static playLandsAndCastSpellsFromTop ]
       (Just (2, 3))

public export
courserOfKruphix : Card
courserOfKruphix =
  Macros.card "Courser of Kruphix"
       (Just [Macros.generic 1, Macros.pip Green, Macros.pip Green]) []
       (MkTypeLine [Centaur] [Enchantment, Creature])
       [ Static (Visibility Reveal You TopOfLibrary)
       , Static (Macros.mayPlayFrom You (AllOf Macros.land) Macros.onTopZ)
       , Triggered Whenever
           (Enters (Macros.a (And [Macros.land, ControlledBy You])))
           (Macros.gainsLife You (Lit 1)) ]
       (Just (2, 4))

public export
telepathy : Card
telepathy =
  Macros.card "Telepathy" (Just [Macros.pip Blue]) []
       (MkTypeLine [] [Enchantment])
       [ Static (Visibility Reveal (PlayerGroup YourOpponents) WholeHand) ]
       Nothing

public export
wanderingEye : Card
wanderingEye =
  Macros.card "Wandering Eye"
       (Just [Macros.generic 2, Macros.pip Blue]) []
       (MkTypeLine [Illusion] [Creature])
       [ KeywordAbility Flying
       , Static (Visibility Reveal (PlayerGroup AllPlayers) WholeHand) ]
       (Just (1, 3))

public export
korlessaScaleSinger : Card
korlessaScaleSinger =
  Macros.card "Korlessa, Scale Singer"
       (Just [Macros.pip Green, Macros.pip Blue]) [Legendary]
       (MkTypeLine [Dragon, Bard] [Creature])
       [ Static (Visibility LookAt You TopOfLibrary)
       , Static (Macros.mayCastFrom You
                                    (AllOf (And [Macros.spell, HasSubtype Dragon]))
                                    Macros.onTopZ) ]
       (Just (1, 4))

public export
assembleThePlayers : Card
assembleThePlayers =
  Macros.card "Assemble the Players"
       (Just [Macros.generic 1, Macros.pip White]) []
       (MkTypeLine [] [Enchantment])
       [ Static (Visibility LookAt You TopOfLibrary)
       , Static castSmallCreatureFromTopOnceEachTurn ]
       Nothing

public export
precognitionField : Card
precognitionField =
  Macros.card "Precognition Field"
       (Just [Macros.generic 3, Macros.pip Blue]) []
       (MkTypeLine [] [Enchantment])
       [ Static (Visibility LookAt You TopOfLibrary)
       , Static (Macros.mayCastFrom You
                                    (AllOf (And [Macros.spell,
                                                 Macros.instantOrSorcery]))
                                    Macros.onTopZ)
       , Activated (Mana [Macros.generic 3])
                   (Macros.exile (LibrarySlice OnTop (Lit 1) You)) ]
       Nothing

public export
mysticForge : Card
mysticForge =
  Macros.card "Mystic Forge" (Just [Macros.generic 4]) []
       (MkTypeLine [] [Artifact])
       [ Static (Visibility LookAt You TopOfLibrary)
       , Static (AndAlso
           [ Macros.mayCastFrom You (AllOf (And [Macros.spell, Macros.artifact])) Macros.onTopZ
           , Macros.mayCastFrom You (AllOf (And [Macros.spell, IsColorless])) Macros.onTopZ ])
       , Activated (Compound [TapSymbol, Do (Macros.losesLife You (Lit 1))])
                   (Macros.exile (LibrarySlice OnTop (Lit 1) You)) ]
       Nothing

public export
exploration : Card
exploration =
  Macros.card "Exploration" (Just [Macros.pip Green]) []
       (MkTypeLine [] [Enchantment])
       [ Static (MayPlayAdditionalLands You (Macros.exactly 1)) ]
       Nothing

public export
oracleOfMulDaya : Card
oracleOfMulDaya =
  Macros.card "Oracle of Mul Daya"
       (Just [Macros.generic 3, Macros.pip Green]) []
       (MkTypeLine [Elf, Shaman] [Creature])
       [ Static (MayPlayAdditionalLands You (Macros.exactly 1))
       , Static (Visibility Reveal You TopOfLibrary)
       , Static (Macros.mayPlayFrom You (AllOf Macros.land) Macros.onTopZ) ]
       (Just (2, 2))

public export
azusaLostButSeeking : Card
azusaLostButSeeking =
  Macros.card "Azusa, Lost but Seeking"
       (Just [Macros.generic 2, Macros.pip Green]) [Legendary]
       (MkTypeLine [Human, Monk] [Creature])
       [ Static (MayPlayAdditionalLands You (Macros.exactly 2)) ]
       (Just (1, 2))

public export
summerBloom : Card
summerBloom =
  Macros.card "Summer Bloom"
       (Just [Macros.generic 1, Macros.pip Green]) []
       (MkTypeLine [] [Sorcery])
       [ Spell (Continuously (MayPlayAdditionalLands You (Macros.upTo 3))
                             (Just ThisTurn)) ]
       Nothing

public export
explore : Card
explore =
  Macros.card "Explore" (Just [Macros.generic 1, Macros.pip Green]) []
       (MkTypeLine [] [Sorcery])
       [ Spell (Sequentially
           [ Continuously (MayPlayAdditionalLands You (Macros.exactly 1))
                          (Just ThisTurn)
           , Macros.drawACard ]) ]
       Nothing

public export
urbanEvolution : Card
urbanEvolution =
  Macros.card "Urban Evolution"
       (Just [Macros.generic 3, Macros.pip Green, Macros.pip Blue]) []
       (MkTypeLine [] [Sorcery])
       [ Spell (Sequentially
           [ Draw You (Lit 3)
           , Continuously (MayPlayAdditionalLands You (Macros.exactly 1))
                          (Just ThisTurn) ]) ]
       Nothing

public export
eachPlayerPlaysAdditionalLand : StaticEffect []
eachPlayerPlaysAdditionalLand =
  MayPlayAdditionalLands (Each AnyPlayer) (Macros.exactly 1)

public export
prismaticOmen : Card
prismaticOmen =
  Macros.card "Prismatic Omen" (Just [Macros.generic 1, Macros.pip Green]) []
       (MkTypeLine [] [Enchantment])
       [ Static (AddsEveryType (AllOf (And [Macros.land, ControlledBy You]))
                               BasicLandSpace) ]
       Nothing

public export
dryadOfTheIlysianGrove : Card
dryadOfTheIlysianGrove =
  Macros.card "Dryad of the Ilysian Grove"
       (Just [Macros.generic 2, Macros.pip Green]) []
       (MkTypeLine [Nymph, Dryad] [Enchantment, Creature])
       [ Static (MayPlayAdditionalLands You (Macros.exactly 1))
       , Static (AddsEveryType (AllOf (And [Macros.land, ControlledBy You]))
                               BasicLandSpace) ]
       (Just (2, 4))

public export
mistformUltimus : Card
mistformUltimus =
  Macros.card "Mistform Ultimus"
       (Just [Macros.generic 3, Macros.pip Blue]) [Legendary]
       (MkTypeLine [Illusion] [Creature])
       [ Static (AddsEveryType Macros.thisCreature CreatureSpace) ]
       (Just (3, 3))

public export
amorphousAxe : Card
amorphousAxe =
  Macros.card "Amorphous Axe" (Just [Macros.generic 2]) []
       (MkTypeLine [Equipment] [Artifact])
       [ Static (AndAlso
           [ Gets (AttachHost Equipped (TypeW Creature)) (PtUp (Lit 3))
                  (PtUp (Lit 0))
           , AddsEveryType (AttachHost Equipped (TypeW Creature))
                           CreatureSpace ])
       , Macros.keywordCosting Equip (Mana [Macros.generic 3]) ]
       Nothing

public export
runedStalactite : Card
runedStalactite =
  Macros.card "Runed Stalactite" (Just [Macros.generic 1]) []
       (MkTypeLine [Equipment] [Artifact])
       [ Static (AndAlso
           [ Gets (AttachHost Equipped (TypeW Creature)) (PtUp (Lit 1))
                  (PtUp (Lit 1))
           , AddsEveryType (AttachHost Equipped (TypeW Creature))
                           CreatureSpace ])
       , Macros.keywordCosting Equip (Mana [Macros.generic 2]) ]
       Nothing

public export
arachnoform : Card
arachnoform =
  Macros.card "Arachnoform" (Just [Macros.generic 1, Macros.pip Green]) []
       (MkTypeLine [Aura] [Enchantment])
       [ Macros.keywordSubject Enchant Macros.creature
       , Static (AndAlso
           [ Gets (AttachHost Enchanted (TypeW Creature)) (PtUp (Lit 2))
                  (PtUp (Lit 2))
           , Gains (AttachHost Enchanted (TypeW Creature)) (KeywordAbility Reach)
           , AddsEveryType (AttachHost Enchanted (TypeW Creature))
                           CreatureSpace ]) ]
       Nothing

public export
nyleasPresence : Card
nyleasPresence =
  Macros.card "Nylea's Presence"
       (Just [Macros.generic 1, Macros.pip Green]) []
       (MkTypeLine [Aura] [Enchantment])
       [ Macros.keywordSubject Enchant Macros.land
       , Triggered When (Enters Macros.thisAura) Macros.drawACard
       , Static (AddsEveryType (AttachHost Enchanted (TypeW Land))
                               BasicLandSpace) ]
       Nothing

public export
volatileClaws : Card
volatileClaws =
  Macros.card "Volatile Claws"
       (Just [Macros.generic 2, Macros.pip Red]) []
       (MkTypeLine [] [Instant])
       [ Spell (Continuously
           (AndAlso
             [ Gets (AllOf (And [Macros.creature, ControlledBy You]))
                    (PtUp (Lit 2)) (PtUp (Lit 0))
             , AddsEveryType (AllOf (And [Macros.creature, ControlledBy You]))
                             CreatureSpace ])
           (Just Macros.untilEndOfTurn)) ]
       Nothing



public export
timeWalk : Card
timeWalk =
  Macros.card "Time Walk" (Just [Macros.generic 1, Macros.pip Blue]) []
       (MkTypeLine [] [Sorcery]) [Spell (ExtraTurn You (Lit 1))] Nothing

public export
timeStretch : Card
timeStretch =
  Macros.card "Time Stretch"
       (Just [Macros.generic 8, Macros.pip Blue, Macros.pip Blue]) []
       (MkTypeLine [] [Sorcery])
       [Spell (ExtraTurn (Macros.target AnyPlayer) (Lit 2))] Nothing

public export
timeSieve : Card
timeSieve =
  Macros.card "Time Sieve" (Just [Macros.pip Blue, Macros.pip Black]) []
       (MkTypeLine [] [Artifact])
       [Activated (Compound [TapSymbol,
                             Do (Macros.sacrifice You
                                   (CountedGroup (Macros.exactly 5) Macros.artifact))])
                  (ExtraTurn You (Lit 1))] Nothing

public export
magistratesScepter : Card
magistratesScepter =
  Macros.card "Magistrate's Scepter" (Just [Macros.generic 3]) []
       (MkTypeLine [] [Artifact])
       [ Activated (Compound [Mana [Macros.generic 4], TapSymbol])
                   (PutCounters (Lit 1) Charge Macros.thisArtifact)
       , Activated (Compound [TapSymbol,
                              Do (RemoveCounters (Lit 3) Charge Macros.thisArtifact)])
                   (ExtraTurn You (Lit 1)) ] Nothing

public export
fatigue : Card
fatigue =
  Macros.card "Fatigue" (Just [Macros.generic 1, Macros.pip Blue]) []
       (MkTypeLine [] [Sorcery])
       [Spell (SkipsNext (Macros.target AnyPlayer) DrawStep (Lit 1))] Nothing

public export
meditate : Card
meditate =
  Macros.card "Meditate" (Just [Macros.generic 2, Macros.pip Blue]) []
       (MkTypeLine [] [Instant])
       [Spell (Sequentially [Draw You (Lit 4), SkipsNext You Turn (Lit 1)])] Nothing

public export
blindingAngel : Card
blindingAngel =
  Macros.card "Blinding Angel"
       (Just [Macros.generic 3, Macros.pip White, Macros.pip White]) []
       (MkTypeLine [Angel] [Creature])
       [ KeywordAbility Flying
       , Triggered Whenever
           (DealsCombatDamage Macros.thisCreature (Macros.a AnyPlayer))
           (SkipsNext (That PlayerW) Combat (Lit 1)) ] (Just (2, 4))

public export
eonHub : Card
eonHub =
  Macros.card "Eon Hub" (Just [Macros.generic 5]) []
       (MkTypeLine [] [Artifact])
       [Static (Skips (PlayerGroup AllPlayers) Upkeep)] Nothing

public export
stasis : Card
stasis =
  Macros.card "Stasis" (Just [Macros.generic 1, Macros.pip Blue]) []
       (MkTypeLine [] [Enchantment])
       [ Static (Skips (PlayerGroup AllPlayers) UntapStep)
       , Triggered At (BeginningOf Upkeep (ByWord Yours))
           (Macros.mayElse You (Pay You (Mana [Macros.pip Blue]))
                           (Macros.sacrifice You Macros.thisEnchantment)) ] Nothing

public export
yawgmothsBargain : Card
yawgmothsBargain =
  Macros.card "Yawgmoth's Bargain"
       (Just [Macros.generic 4, Macros.pip Black, Macros.pip Black]) []
       (MkTypeLine [] [Enchantment])
       [ Static (Skips You DrawStep)
       , Activated (Macros.payLife You 1) Macros.drawACard ] Nothing

public export
sandsOfTimeSkip : StaticEffect []
sandsOfTimeSkip = Skips (Each AnyPlayer) UntapStep

public export
wormfangManta : Card
wormfangManta =
  Macros.card "Wormfang Manta"
       (Just [Macros.generic 5, Macros.pip Blue, Macros.pip Blue]) []
       (MkTypeLine [Nightmare, Fish, Beast] [Creature])
       [ KeywordAbility Flying
       , Triggered When (Enters Macros.thisCreature) (SkipsNext You Turn (Lit 1))
       , Triggered When (Leaves Macros.thisCreature) (ExtraTurn You (Lit 1)) ]
       (Just (6, 6))

public export
eaterOfDaysSkip : Effect []
eaterOfDaysSkip = SkipsNext You Turn (Lit 2)



public export
finalFortune : Card
finalFortune =
  Macros.card "Final Fortune" (Just [Macros.pip Red, Macros.pip Red]) []
       (MkTypeLine [] [Instant])
       [Spell (Sequentially [ExtraTurn You (Lit 1),
                             Delayed (BeginningOf EndStep (ByWord ThatTurns))
                                     (Concludes LoseGame You)])] Nothing

public export
lastChance : Card
lastChance =
  Macros.card "Last Chance" (Just [Macros.pip Red, Macros.pip Red]) []
       (MkTypeLine [] [Sorcery])
       [Spell (Sequentially [ExtraTurn You (Lit 1),
                             Delayed (BeginningOf EndStep (ByWord ThatTurns))
                                     (Concludes LoseGame You)])] Nothing

public export
chanceForGlory : Card
chanceForGlory =
  Macros.card "Chance for Glory"
       (Just [Macros.generic 1, Macros.pip Red, Macros.pip White]) []
       (MkTypeLine [] [Instant])
       [Spell (Sequentially
         [ Continuously (Gains (AllOf (And [Macros.creature, ControlledBy You]))
                               (KeywordAbility Indestructible)) Nothing
         , ExtraTurn You (Lit 1)
         , Delayed (BeginningOf EndStep (ByWord ThatTurns))
                   (Concludes LoseGame You)])] Nothing

public export
aggravatedAssault : Card
aggravatedAssault =
  Macros.card "Aggravated Assault" (Just [Macros.generic 2, Macros.pip Red]) []
       (MkTypeLine [] [Enchantment])
       [Macros.activatedOnlyDuring (Mana [Macros.generic 3, Macros.pip Red, Macros.pip Red])
                                   (Sequentially
                                     [ SetStatus Untapped
                                         (AllOf (And [Macros.creature, ControlledBy You]))
                                     , Macros.additionalPartThen Combat
                                                                 (Just MainPhase)
                                                                 (Lit 1)
                                                                 MainPhase])
                                   AsSorcery] Nothing

public export
relentlessAssault : Card
relentlessAssault =
  Macros.card "Relentless Assault"
       (Just [Macros.generic 2, Macros.pip Red, Macros.pip Red]) []
       (MkTypeLine [] [Sorcery])
       [Spell (Sequentially
         [ SetStatus Untapped
             (AllOf (And [Macros.creature,
                          HappenedTo AttackDeclaration Lookback.ThisTurn]))
         , Macros.additionalPartThen Combat (Just MainPhase) (Lit 1) MainPhase])] Nothing

public export
hellkiteCharger : Card
hellkiteCharger =
  Macros.card "Hellkite Charger"
       (Just [Macros.generic 4, Macros.pip Red, Macros.pip Red]) []
       (MkTypeLine [Dragon] [Creature])
       [ KeywordAbility Flying
       , KeywordAbility Haste
       , Triggered Whenever (Attacks Macros.thisCreature)
           (Macros.mayThen You
              (Pay You (Mana [Macros.generic 5, Macros.pip Red, Macros.pip Red]))
              (Sequentially
                [ SetStatus Untapped (AllOf (And [Macros.creature, Attacking]))
                , AdditionalPart Combat Nothing (Lit 1)])) ] (Just (5, 5))

public export
fullThrottleFirstLine : Effect []
fullThrottleFirstLine = AdditionalPart Combat (Just MainPhase) (Lit 2)

public export
raphaelAdditionalCombat : Effect []
raphaelAdditionalCombat = AdditionalPart Combat (Just Combat) (Lit 1)

public export
yshtolaAdditionalEndStep : Effect []
yshtolaAdditionalEndStep = AdditionalPart EndStep Nothing (Lit 1)



public export
grumgully : Card
grumgully =
  Macros.card "Grumgully, the Generous"
       (Just [Macros.generic 1, Macros.pip Red, Macros.pip Green]) [Legendary]
       (MkTypeLine [Goblin, Shaman] [Creature])
       [Static (Macros.entersWithAdditionalCounters (Each (And [Macros.creature, ControlledBy You,
                                                                Not (HasSubtype Human),
                                                                OtherThan Macros.thisCreature]))
                                                    (Lit 1)
                                                    Macros.plusOnePlusOne)]
       (Just (3, 3))

public export
dragonstormGlobe : Card
dragonstormGlobe =
  Macros.card "Dragonstorm Globe" (Just [Macros.generic 3]) []
       (MkTypeLine [] [Artifact])
       [ Static (Macros.entersWithAdditionalCounters (Each (And [HasSubtype Dragon, ControlledBy You]))
                                                     (Lit 1)
                                                     Macros.plusOnePlusOne)
       , Activated TapSymbol (AddMana You (Lit 1) (AnyColor SameColor) []) ]
       Nothing

public export
sageOfFables : Card
sageOfFables =
  Macros.card "Sage of Fables" (Just [Macros.generic 2, Macros.pip Blue]) []
       (MkTypeLine [Merfolk, Wizard] [Creature])
       [ Static (Macros.entersWithAdditionalCounters (Each (And [Macros.creature, HasSubtype Wizard, ControlledBy You,
                                                                 OtherThan Macros.thisCreature]))
                                                     (Lit 1)
                                                     Macros.plusOnePlusOne)
       , Activated (Compound [Mana [Macros.generic 2],
                              Do (RemoveCounters (Lit 1) Macros.plusOnePlusOne
                                   (Macros.a (And [Macros.creature, ControlledBy You])))])
                   Macros.drawACard ]
       (Just (2, 2))

public export
metallicMimic : Card
metallicMimic =
  Macros.card "Metallic Mimic" (Just [Macros.generic 2]) []
       (MkTypeLine [Shapeshifter] [Artifact, Creature])
       [ Static (EntersChoice Macros.thisCreature CreatureType)
       , Static (AddsChosenQuality Macros.thisCreature (OfChosen CreatureType))
       , Static (Macros.entersWithAdditionalCounters (Each (And [Macros.creature, ControlledBy You,
                                                                 OfChosen CreatureType,
                                                                 OtherThan Macros.thisCreature]))
                                                     (Lit 1)
                                                     Macros.plusOnePlusOne) ]
       (Just (2, 1))

public export
faceDownFlyingCounter : StaticEffect []
faceDownFlyingCounter =
  EntersWithCounters (AllOf (And [Macros.creature, ControlledBy You,
                                  HasStatus FaceDown]))
                     (Lit 1) (KeywordCounter Flying)

public export
curatorBeastieLine : StaticEffect []
curatorBeastieLine =
  Macros.entersWithAdditionalCounters (AllOf (And [Macros.creature, ControlledBy You, IsColorless]))
                                      (Lit 2)
                                      Macros.plusOnePlusOne

public export
renataLine : StaticEffect []
renataLine =
  Macros.entersWithAdditionalCounters (Each (And [Macros.creature, ControlledBy You,
                                                  OtherThan Macros.thisCreature]))
                                      (Lit 1)
                                      Macros.plusOnePlusOne

public export
tayamLine : StaticEffect []
tayamLine =
  Macros.entersWithAdditionalCounters (Each (And [Macros.creature, ControlledBy You,
                                                  OtherThan Macros.thisCreature]))
                                      (Lit 1)
                                      (KeywordCounter Vigilance)

public export
phyrexianRevoker : Card
phyrexianRevoker =
  Macros.card "Phyrexian Revoker" (Just [Macros.generic 2]) []
       (MkTypeLine [Horror] [Artifact, Creature])
       [ Static (Macros.entersChoosingFrom Macros.thisCreature
                                           CardName
                                           (NameOfCard (Not (HasType Land))))
       , Static (ObjectCant Activated
                   (AllOf (And [ AbilityHead AnyActivated
                               , AbilityOf (AllOf (And [Macros.source,
                                                        Named ChosenName])) ]))) ]
       (Just (2, 1))

public export
voidstoneGargoyle : Card
voidstoneGargoyle =
  Macros.card "Voidstone Gargoyle"
       (Just [Macros.generic 3, Macros.pip White, Macros.pip White]) []
       (MkTypeLine [Gargoyle] [Creature])
       [ KeywordAbility Flying
       , Static (Macros.entersChoosingFrom Macros.thisCreature
                                           CardName
                                           (NameOfCard (Not (HasType Land))))
       , Static (ObjectCant Cast
                   (AllOf (And [Macros.spell, Named ChosenName])))
       , Static (ObjectCant Activated
                   (AllOf (And [ AbilityHead AnyActivated
                               , AbilityOf (AllOf (And [Macros.source,
                                                        Named ChosenName])) ]))) ]
       (Just (3, 3))

public export
pithingNeedle : Card
pithingNeedle =
  Macros.card "Pithing Needle" (Just [Macros.generic 1]) []
       (MkTypeLine [] [Artifact])
       [ Static (EntersChoice Macros.thisArtifact CardName)
       , Static (ObjectCant Activated
                   (AllOf (And [ AbilityHead AnyActivated
                               , AbilityOf (AllOf (And [Macros.source,
                                                        Named ChosenName]))
                               , Not IsManaAbility ]))) ]
       Nothing

public export
cursedTotem : Card
cursedTotem =
  Macros.card "Cursed Totem" (Just [Macros.generic 2]) []
       (MkTypeLine [] [Artifact])
       [ Static (ObjectCant Activated
                   (AllOf (And [ AbilityHead AnyActivated
                               , AbilityOf (AllOf Macros.creature) ]))) ]
       Nothing

public export
nullRod : Card
nullRod =
  Macros.card "Null Rod" (Just [Macros.generic 2]) []
       (MkTypeLine [] [Artifact])
       [ Static (ObjectCant Activated
                   (AllOf (And [ AbilityHead AnyActivated
                               , AbilityOf (AllOf Macros.artifact) ]))) ]
       Nothing

public export
stonySilence : Card
stonySilence =
  Macros.card "Stony Silence" (Just [Macros.generic 1, Macros.pip White]) []
       (MkTypeLine [] [Enchantment])
       [ Static (ObjectCant Activated
                   (AllOf (And [ AbilityHead AnyActivated
                               , AbilityOf (AllOf Macros.artifact) ]))) ]
       Nothing

public export
clarionConqueror : Card
clarionConqueror =
  Macros.card "Clarion Conqueror" (Just [Macros.generic 2, Macros.pip White]) []
       (MkTypeLine [Dragon] [Creature])
       [ KeywordAbility Flying
       , Static (ObjectCant Activated
                   (AllOf (And [ AbilityHead AnyActivated
                               , AbilityOf (AllOf (Or [Macros.artifact,
                                                       Macros.creature,
                                                       HasType Planeswalker])) ]))) ]
       (Just (3, 3))

public export
dampingMatrix : Card
dampingMatrix =
  Macros.card "Damping Matrix" (Just [Macros.generic 3]) []
       (MkTypeLine [] [Artifact])
       [ Static (ObjectCant Activated
                   (AllOf (And [ AbilityHead AnyActivated
                               , AbilityOf (AllOf (Or [Macros.artifact,
                                                       Macros.creature]))
                               , Not IsManaAbility ]))) ]
       Nothing

public export
suppressionField : Card
suppressionField =
  Macros.card "Suppression Field" (Just [Macros.generic 1, Macros.pip White]) []
       (MkTypeLine [] [Enchantment])
       [ Static (CostsToCast
                   (AllOf (And [AbilityHead AnyActivated, Not IsManaAbility]))
                   (CostMore (Lit 2))) ]
       Nothing

public export
gloom : Card
gloom =
  Macros.card "Gloom" (Just [Macros.generic 2, Macros.pip Black]) []
       (MkTypeLine [] [Enchantment])
       [ Static (CostsToCast (AllOf (And [Macros.spell, ColorIs White]))
                             (CostMore (Lit 3)))
       , Static (CostsToCast
                   (AllOf (And [ AbilityHead AnyActivated
                               , AbilityOf (AllOf (And [Macros.enchantment,
                                                        ColorIs White])) ]))
                   (CostMore (Lit 3))) ]
       Nothing

public export
bureauHeadmaster : Card
bureauHeadmaster =
  Macros.card "Bureau Headmaster" (Just [Macros.pip Red, Macros.pip White]) []
       (MkTypeLine [Human, Assassin] [Creature])
       [ Static (CostsToCast
                   (AllOf (And [Macros.spell, HasSubtype Equipment, CastBy You]))
                   (CostLess (Lit 1)))
       , Static (CostsToCast
                   (AllOf (And [AbilityHead (KeywordClass Equip), ActivatedBy You]))
                   (CostLess (Lit 1))) ]
       (Just (2, 2))

public export
demotion : Card
demotion =
  Macros.card "Demotion" (Just [Macros.pip White]) []
       (MkTypeLine [Aura] [Enchantment])
       [ Macros.keywordSubject Enchant Macros.creature
       , Static (AndAlso
           [ Deontic (AttachHost Enchanted (TypeW Creature)) Forbid Block Agent NoDeonticPatient
           , ObjectCant Activated
               (AllOf (And [AbilityHead AnyActivated, AbilityOf It])) ]) ]
       Nothing

public export
vipersKiss : Card
vipersKiss =
  Macros.card "Viper's Kiss" (Just [Macros.pip Black]) []
       (MkTypeLine [Aura] [Enchantment])
       [ Macros.keywordSubject Enchant Macros.creature
       , Static (AndAlso
           [ Gets (AttachHost Enchanted (TypeW Creature)) (PtDown (Lit 1)) (PtDown (Lit 1))
           , ObjectCant Activated
               (AllOf (And [AbilityHead AnyActivated, AbilityOf It])) ]) ]
       Nothing

public export
stupefyingTouch : Card
stupefyingTouch =
  Macros.card "Stupefying Touch" (Just [Macros.generic 1, Macros.pip Blue]) []
       (MkTypeLine [Aura] [Enchantment])
       [ Macros.keywordSubject Enchant Macros.creature
       , Triggered When (Enters Macros.thisAura) (Draw You (Lit 1))
       , Static (ObjectCant Activated
                   (AllOf (And [ AbilityHead AnyActivated
                               , AbilityOf (AttachHost Enchanted (TypeW Creature)) ]))) ]
       Nothing

public export
oppressiveRaysLine : StaticEffect []
oppressiveRaysLine =
  CostsToCast
    (AllOf (And [ AbilityHead AnyActivated
                , AbilityOf (AttachHost Enchanted (TypeW Creature)) ]))
    (CostMore (Lit 3))

public export
linvalaKeeperOfSilence : Card
linvalaKeeperOfSilence =
  Macros.card "Linvala, Keeper of Silence"
       (Just [Macros.generic 2, Macros.pip White, Macros.pip White]) [Legendary]
       (MkTypeLine [Angel] [Creature])
       [ KeywordAbility Flying
       , Static (ObjectCant Activated
                   (AllOf (And [ AbilityHead AnyActivated
                               , AbilityOf (AllOf (And [Macros.creature,
                                                        ControlledBy (PlayerGroup YourOpponents)])) ]))) ]
       (Just (3, 4))

public export
eidolonOfObstruction : Card
eidolonOfObstruction =
  Macros.card "Eidolon of Obstruction" (Just [Macros.generic 1, Macros.pip White]) []
       (MkTypeLine [Spirit] [Enchantment, Creature])
       [ KeywordAbility FirstStrike
       , Static (CostsToCast
                   (AllOf (And [ AbilityHead LoyaltyClass
                               , AbilityOf (AllOf (And [HasType Planeswalker,
                                                        ControlledBy (PlayerGroup YourOpponents)])) ]))
                   (CostMore (Lit 1))) ]
       (Just (2, 1))

public export
forceOfWill : Card
forceOfWill =
  Macros.card "Force of Will"
       (Just [Macros.generic 3, Macros.pip Blue, Macros.pip Blue]) []
       (MkTypeLine [] [Instant])
       [ Static (AltCost (Just (Compound
                   [ Do (ChangeLife You (Down (Lit 1)))
                   , Do (Macros.exile (Macros.a (And [ColorIs Blue,
                                                      InZone (Macros.handOf You)]))) ])))
       , Spell (Macros.counterSpell (Macros.target Macros.spell)) ]
       Nothing

public export
crash : Card
crash =
  Macros.card "Crash" (Just [Macros.generic 2, Macros.pip Red]) []
       (MkTypeLine [] [Instant])
       [ Static (AltCost (Just (Do (Macros.sacrifice You
                   (Macros.a (And [Macros.land, HasSubtype Mountain]))))))
       , Spell (Macros.destroy (Macros.target Macros.artifact)) ]
       Nothing

public export
moggSalvage : Card
moggSalvage =
  Macros.card "Mogg Salvage" (Just [Macros.generic 2, Macros.pip Red]) []
       (MkTypeLine [] [Instant])
       [ Static (Conditionally
                   (AndCond
                      [ Exists (And [Macros.land, HasSubtype Island,
                                     ControlledBy Macros.anOpponent])
                      , Exists (And [Macros.land, HasSubtype Mountain,
                                     ControlledBy You]) ])
                   (AltCost Nothing))
       , Spell (Macros.destroy (Macros.target Macros.artifact)) ]
       Nothing

public export
abolish : Card
abolish =
  Macros.card "Abolish" (Just [Macros.generic 1, Macros.pip White, Macros.pip White]) []
       (MkTypeLine [] [Instant])
       [ Static (AltCost (Just (Do (Macros.discards You
                   (Macros.a (And [HasSubtype Plains, InZone Macros.handZ]))))))
       , Spell (Macros.destroy (Macros.target (Or [Macros.artifact,
                                                   Macros.enchantment]))) ]
       Nothing

public export
gush : Card
gush =
  Macros.card "Gush" (Just [Macros.generic 4, Macros.pip Blue]) []
       (MkTypeLine [] [Instant])
       [ Static (AltCost (Just (Do (Move
                   (CountedGroup (Macros.exactly 2)
                                 (And [Macros.land, HasSubtype Island,
                                       ControlledBy You]))
                   Macros.handZ))))
       , Spell (Macros.drawCards 2) ]
       Nothing

public export
sunscour : Card
sunscour =
  Macros.card "Sunscour"
       (Just [Macros.generic 5, Macros.pip White, Macros.pip White]) []
       (MkTypeLine [] [Sorcery])
       [ Static (AltCost (Just (Do (Macros.exile
                   (CountedGroup (Macros.exactly 2)
                                 (And [ColorIs White,
                                       InZone (Macros.handOf You)]))))))
       , Spell (Macros.destroy (AllOf Macros.creature)) ]
       Nothing

public export
admiralsOrder : Card
admiralsOrder =
  Macros.card "Admiral's Order"
       (Just [Macros.generic 1, Macros.pip Blue, Macros.pip Blue]) []
       (MkTypeLine [] [Instant])
       [ Static (Conditionally
                   (Happened AttackDeclaration You Lookback.ThisTurn)
                   (AltCost (Just (Mana [Macros.pip Blue]))))
       , Spell (Macros.counterSpell (Macros.target Macros.spell)) ]
       Nothing

public export
massacre : Card
massacre =
  Macros.card "Massacre"
       (Just [Macros.generic 2, Macros.pip Black, Macros.pip Black]) []
       (MkTypeLine [] [Sorcery])
       [ Static (Conditionally
                   (AndCond
                      [ Exists (And [Macros.land, HasSubtype Plains,
                                     ControlledBy Macros.anOpponent])
                      , Exists (And [Macros.land, HasSubtype Swamp,
                                     ControlledBy You]) ])
                   (AltCost Nothing))
       , Spell (Continuously
                  (Gets (AllOf Macros.creature) (PtDown (Lit 2)) (PtDown (Lit 2)))
                  (Just Macros.untilEndOfTurn)) ]
       Nothing

public export
kyrenLegate : Card
kyrenLegate =
  Macros.card "Kyren Legate" (Just [Macros.generic 1, Macros.pip Red]) []
       (MkTypeLine [Goblin] [Creature])
       [ Static (Conditionally
                   (AndCond
                      [ Exists (And [Macros.land, HasSubtype Plains,
                                     ControlledBy Macros.anOpponent])
                      , Exists (And [Macros.land, HasSubtype Mountain,
                                     ControlledBy You]) ])
                   (AltCost Nothing))
       , KeywordAbility Haste ]
       (Just (1, 1))

public export
rouse : Card
rouse =
  Macros.card "Rouse" (Just [Macros.generic 1, Macros.pip Black]) []
       (MkTypeLine [] [Instant])
       [ Static (Conditionally
                   (Exists (And [Macros.land, HasSubtype Swamp, ControlledBy You]))
                   (AltCost (Just (Do (ChangeLife You (Down (Lit 2)))))))
       , Spell (Continuously
                  (Gets (Macros.target Macros.creature) (PtUp (Lit 2)) (PtUp (Lit 0)))
                  (Just Macros.untilEndOfTurn)) ]
       Nothing

public export
demonOfDeathsGate : Card
demonOfDeathsGate =
  Macros.card "Demon of Death's Gate"
       (Just [Macros.generic 6, Macros.pip Black, Macros.pip Black, Macros.pip Black]) []
       (MkTypeLine [Demon] [Creature])
       [ Static (AltCost (Just (Compound
                   [ Do (ChangeLife You (Down (Lit 6)))
                   , Do (Macros.sacrifice You
                           (CountedGroup (Macros.exactly 3)
                                         (And [Macros.creature, ColorIs Black]))) ])))
       , KeywordAbility Flying
       , KeywordAbility Trample ]
       (Just (9, 9))

public export
thwart : Card
thwart =
  Macros.card "Thwart"
       (Just [Macros.generic 2, Macros.pip Blue, Macros.pip Blue]) []
       (MkTypeLine [] [Instant])
       [ Static (AltCost (Just (Do (Move
                   (CountedGroup (Macros.exactly 3)
                                 (And [Macros.land, HasSubtype Island,
                                       ControlledBy You]))
                   Macros.handZ))))
       , Spell (Macros.counterSpell (Macros.target Macros.spell)) ]
       Nothing

public export
theLadyOfOtariaLine : StaticEffect []
theLadyOfOtariaLine =
  AltCost (Just (Do (SetStatus Tapped
            (CountedGroup (Macros.exactly 3)
                          (And [Macros.creature, HasSubtype Dwarf,
                                ControlledBy You, HasStatus Untapped])))))

public export
shiningShoal : Card
shiningShoal =
  Macros.card "Shining Shoal"
       (Just [Variable, Macros.pip White, Macros.pip White]) []
       (MkTypeLine [Arcane] [Instant])
       [ Static (AltCost (Just (Do (Macros.exile
                   (Macros.a (And [ColorIs White,
                                   Compare ManaValue Eq XVal,
                                   InZone (Macros.handOf You)]))))))
       , Spell (Continuously
                  (Redirects AnyDamage (TheNext XVal)
                             (Macros.shieldingIt
                                (YouAnd (AllOf (And [HasType Creature,
                                                     ControlledBy You]))))
                             (Just (Macros.aYourChoice Macros.source))
                             (Macros.target AnyTarget))
                  (Just Macros.thisTurn)) ]
       Nothing

public export
disruptingShoal : Card
disruptingShoal =
  Macros.card "Disrupting Shoal"
       (Just [Variable, Macros.pip Blue, Macros.pip Blue]) []
       (MkTypeLine [Arcane] [Instant])
       [ Static (AltCost (Just (Do (Macros.exile
                   (Macros.a (And [ColorIs Blue,
                                   Compare ManaValue Eq XVal,
                                   InZone (Macros.handOf You)]))))))
       , Spell (If (Macros.counterSpell (Macros.target Macros.spell))
                   (CompareAmt (Macros.manaValueOf It) Eq XVal) Nothing) ]
       Nothing

public export
blazingShoal : Card
blazingShoal =
  Macros.card "Blazing Shoal"
       (Just [Variable, Macros.pip Red, Macros.pip Red]) []
       (MkTypeLine [Arcane] [Instant])
       [ Static (AltCost (Just (Do (Macros.exile
                   (Macros.a (And [ColorIs Red,
                                   Compare ManaValue Eq XVal,
                                   InZone (Macros.handOf You)]))))))
       , Spell (Macros.gets (Macros.target Macros.creature) (PtUp XVal)
                            (PtUp (Lit 0)) (Just Macros.untilEndOfTurn)) ]
       Nothing

public export
sickeningShoal : Card
sickeningShoal =
  Macros.card "Sickening Shoal"
       (Just [Variable, Macros.pip Black, Macros.pip Black]) []
       (MkTypeLine [Arcane] [Instant])
       [ Static (AltCost (Just (Do (Macros.exile
                   (Macros.a (And [ColorIs Black,
                                   Compare ManaValue Eq XVal,
                                   InZone (Macros.handOf You)]))))))
       , Spell (Macros.gets (Macros.target Macros.creature) (PtDown XVal)
                            (PtDown XVal) (Just Macros.untilEndOfTurn)) ]
       Nothing

public export
nourishingShoal : Card
nourishingShoal =
  Macros.card "Nourishing Shoal"
       (Just [Variable, Macros.pip Green, Macros.pip Green]) []
       (MkTypeLine [Arcane] [Instant])
       [ Static (AltCost (Just (Do (Macros.exile
                   (Macros.a (And [ColorIs Green,
                                   Compare ManaValue Eq XVal,
                                   InZone (Macros.handOf You)]))))))
       , Spell (ChangeLife You (Up XVal)) ]
       Nothing

public export
spellSnare : Card
spellSnare =
  Macros.card "Spell Snare" (Just [Macros.pip Blue]) []
       (MkTypeLine [] [Instant])
       [ Spell (Macros.counterSpell
                  (Macros.target (And [Macros.spell,
                                       Compare ManaValue Eq (Lit 2)]))) ]
       Nothing

public export
isolate : Card
isolate =
  Macros.card "Isolate" (Just [Macros.pip White]) []
       (MkTypeLine [] [Instant])
       [ Spell (Macros.exile
                  (Macros.target (And [Permanent,
                                       Compare ManaValue Eq (Lit 1)]))) ]
       Nothing

public export
disembowel : Card
disembowel =
  Macros.card "Disembowel" (Just [Variable, Macros.pip Black]) []
       (MkTypeLine [] [Instant])
       [ Spell (Macros.destroy
                  (Macros.target (And [Macros.creature,
                                       Compare ManaValue Eq XVal]))) ]
       Nothing

public export
repeal : Card
repeal =
  Macros.card "Repeal" (Just [Variable, Macros.pip Blue]) []
       (MkTypeLine [] [Instant])
       [ Spell (Sequentially
                  [ Move (Macros.target (And [Not Macros.land, Permanent,
                                              Compare ManaValue Eq XVal]))
                         Macros.handZ
                  , Draw You (Lit 1) ]) ]
       Nothing

public export
entrancingMelody : Card
entrancingMelody =
  Macros.card "Entrancing Melody"
       (Just [Variable, Macros.pip Blue, Macros.pip Blue]) []
       (MkTypeLine [] [Sorcery])
       [ Spell (Continuously
                  (GainsControl You
                     (Macros.target (And [Macros.creature,
                                          Compare ManaValue Eq XVal])))
                  Nothing) ]
       Nothing

public export
ratchetBomb : Card
ratchetBomb =
  Macros.card "Ratchet Bomb" (Just [Macros.generic 2]) []
       (MkTypeLine [] [Artifact])
       [ Activated TapSymbol
           (PutCounters (Lit 1) Charge Macros.thisArtifact)
       , Activated (Compound [TapSymbol,
                              Do (Macros.sacrifice You Macros.thisArtifact)])
           (Macros.destroy (Each (And [Not Macros.land, Permanent,
                                       Compare ManaValue Eq
                                         (CountersOn Charge Macros.thisArtifact)]))) ]
       Nothing

public export
entDraughtBasinAbility : Ability
entDraughtBasinAbility =
  Activated (Compound [Mana [Variable], TapSymbol])
            (PutCounters (Lit 1) Macros.plusOnePlusOne
               (Macros.target (And [Macros.creatureYouControl,
                                    Compare Power Eq XVal])))

public export
sarkhansUnsealingLine : Ability
sarkhansUnsealingLine =
  Triggered Whenever
    (Casts You (Macros.a (And [Macros.creature, Macros.spell,
                               Or [Compare Power Eq (Lit 4),
                                   Compare Power Eq (Lit 5),
                                   Compare Power Eq (Lit 6)]])))
    (DealDamage Macros.thisEnchantment (Lit 4) (Macros.target AnyTarget))

public export
savageSwipeLine : Effect []
savageSwipeLine =
  If (Macros.gets (Macros.target Macros.creatureYouControl) (PtUp (Lit 2))
                  (PtUp (Lit 2)) (Just Macros.untilEndOfTurn))
     (CompareAmt (Macros.powerOf It) Eq (Lit 2)) Nothing

public export
drudgeSkeletons : Card
drudgeSkeletons =
  Macros.card "Drudge Skeletons" (Just [Macros.generic 1, Macros.pip Black]) []
       (MkTypeLine [Skeleton] [Creature])
       [ Activated (Mana [Macros.pip Black]) (Regenerate Macros.thisCreature) ]
       (Just (1, 1))

public export
asphodelWanderer : Card
asphodelWanderer =
  Macros.card "Asphodel Wanderer" (Just [Macros.pip Black]) []
       (MkTypeLine [Skeleton, Soldier] [Creature])
       [ Activated (Mana [Macros.generic 2, Macros.pip Black])
                   (Regenerate Macros.thisCreature) ]
       (Just (1, 1))

public export
deathWard : Card
deathWard =
  Macros.card "Death Ward" (Just [Macros.pip White]) []
       (MkTypeLine [] [Instant])
       [ Spell (Regenerate (Macros.target Macros.creature)) ]
       Nothing

public export
terror : Card
terror =
  Macros.card "Terror" (Just [Macros.generic 1, Macros.pip Black]) []
       (MkTypeLine [] [Instant])
       [ Spell (CantBe (Macros.destroy (Macros.target
                  (And [Macros.creature, Not Macros.artifact,
                        Not (ColorIs Black)])))
                Regenerated It) ]
       Nothing

public export
snuffOut : Card
snuffOut =
  Macros.card "Snuff Out" (Just [Macros.generic 3, Macros.pip Black]) []
       (MkTypeLine [] [Instant])
       [ Static (Conditionally
                   (Exists (And [Macros.land, HasSubtype Swamp,
                                 ControlledBy You]))
                   (AltCost (Just (Do (ChangeLife You (Down (Lit 4)))))))
       , Spell (CantBe (Macros.destroy (Macros.target
                  (And [Macros.creature, Not (ColorIs Black)])))
                Regenerated It) ]
       Nothing

public export
wrathOfGod : Card
wrathOfGod =
  Macros.card "Wrath of God"
       (Just [Macros.generic 2, Macros.pip White, Macros.pip White]) []
       (MkTypeLine [] [Sorcery])
       [ Spell (CantBe (Macros.destroy (AllOf Macros.creature))
                       Regenerated Them) ]
       Nothing

public export
damnation : Card
damnation =
  Macros.card "Damnation"
       (Just [Macros.generic 2, Macros.pip Black, Macros.pip Black]) []
       (MkTypeLine [] [Sorcery])
       [ Spell (CantBe (Macros.destroy (AllOf Macros.creature))
                       Regenerated Them) ]
       Nothing

public export
hurrJackalAbility : Ability
hurrJackalAbility =
  Activated TapSymbol
    (Continuously (ObjectCant Regenerated (Macros.target Macros.creature))
                  (Just Macros.thisTurn))

public export
solGrail : Card
solGrail =
  Macros.card "Sol Grail" (Just [Macros.generic 3]) []
       (MkTypeLine [] [Artifact])
       [ Static (EntersChoice Macros.thisArtifact Color)
       , Activated TapSymbol
           (AddMana You (Lit 1) (OfChosenColor Nothing) []) ]
       Nothing

public export
unchartedHaven : Card
unchartedHaven =
  Macros.card "Uncharted Haven" Nothing []
       (MkTypeLine [] [Land])
       [ Static (Macros.entersTapped Macros.thisLand)
       , Static (EntersChoice Macros.thisLand Color)
       , Activated TapSymbol
           (AddMana You (Lit 1) (OfChosenColor Nothing) []) ]
       Nothing

public export
mirageMesa : Card
mirageMesa =
  Macros.card "Mirage Mesa" Nothing []
       (MkTypeLine [Desert] [Land])
       [ Static (Macros.entersTapped Macros.thisLand)
       , Static (EntersChoice Macros.thisLand Color)
       , Activated TapSymbol
           (AddMana You (Lit 1) (OfChosenColor Nothing) []) ]
       Nothing

public export
crossroadsVillage : Card
crossroadsVillage =
  Macros.card "Crossroads Village" Nothing []
       (MkTypeLine [Town] [Land])
       [ Static (Macros.entersTapped Macros.thisLand)
       , Static (EntersChoice Macros.thisLand Color)
       , Activated TapSymbol
           (AddMana You (Lit 1) (OfChosenColor Nothing) []) ]
       Nothing

public export
thrivingBluff : Card
thrivingBluff =
  Macros.card "Thriving Bluff" Nothing []
       (MkTypeLine [] [Land])
       [ Static (Macros.entersTapped Macros.thisLand)
       , Static (Macros.entersChoosingFrom Macros.thisLand Color (ColorOtherThan Red))
       , Activated TapSymbol
           (AddMana You (Lit 1) (OfChosenColor (Just [OfColor Red])) []) ]
       Nothing

public export
thrivingGrove : Card
thrivingGrove =
  Macros.card "Thriving Grove" Nothing []
       (MkTypeLine [] [Land])
       [ Static (Macros.entersTapped Macros.thisLand)
       , Static (Macros.entersChoosingFrom Macros.thisLand Color (ColorOtherThan Green))
       , Activated TapSymbol
           (AddMana You (Lit 1) (OfChosenColor (Just [OfColor Green])) []) ]
       Nothing

public export
thrivingHeath : Card
thrivingHeath =
  Macros.card "Thriving Heath" Nothing []
       (MkTypeLine [] [Land])
       [ Static (Macros.entersTapped Macros.thisLand)
       , Static (Macros.entersChoosingFrom Macros.thisLand Color (ColorOtherThan White))
       , Activated TapSymbol
           (AddMana You (Lit 1) (OfChosenColor (Just [OfColor White])) []) ]
       Nothing

public export
thrivingIsle : Card
thrivingIsle =
  Macros.card "Thriving Isle" Nothing []
       (MkTypeLine [] [Land])
       [ Static (Macros.entersTapped Macros.thisLand)
       , Static (Macros.entersChoosingFrom Macros.thisLand Color (ColorOtherThan Blue))
       , Activated TapSymbol
           (AddMana You (Lit 1) (OfChosenColor (Just [OfColor Blue])) []) ]
       Nothing

public export
thrivingMoor : Card
thrivingMoor =
  Macros.card "Thriving Moor" Nothing []
       (MkTypeLine [] [Land])
       [ Static (Macros.entersTapped Macros.thisLand)
       , Static (Macros.entersChoosingFrom Macros.thisLand Color (ColorOtherThan Black))
       , Activated TapSymbol
           (AddMana You (Lit 1) (OfChosenColor (Just [OfColor Black])) []) ]
       Nothing



public export
bramblewoodParagon : Card
bramblewoodParagon =
  Macros.card "Bramblewood Paragon"
       (Just [Macros.generic 1, Macros.pip Green]) []
       (MkTypeLine [Elf, Warrior] [Creature])
       [ Static (Macros.entersWithAdditionalCounters (Each (And [Macros.creature, HasSubtype Warrior, ControlledBy You,
                                                                 OtherThan Macros.thisCreature]))
                                                     (Lit 1)
                                                     Macros.plusOnePlusOne)
       , Static (Gains (Each (And [Macros.creature, ControlledBy You,
                                   HasCounters (Just Macros.plusOnePlusOne)]))
                       (KeywordAbility Trample)) ]
       (Just (2, 2))

public export
oonasBlackguard : Card
oonasBlackguard =
  Macros.card "Oona's Blackguard"
       (Just [Macros.generic 1, Macros.pip Black]) []
       (MkTypeLine [Faerie, Rogue] [Creature])
       [ KeywordAbility Flying
       , Static (Macros.entersWithAdditionalCounters (Each (And [Macros.creature, HasSubtype Rogue, ControlledBy You,
                                                                 OtherThan Macros.thisCreature]))
                                                     (Lit 1)
                                                     Macros.plusOnePlusOne)
       , Triggered Whenever
                   (DealsCombatDamage
                      (Each (And [Macros.creature, ControlledBy You,
                                  HasCounters (Just Macros.plusOnePlusOne)]))
                      (Macros.a AnyPlayer))
                   (Macros.discardsACard (That PlayerW)) ]
       (Just (1, 1))

public export
crumblingAshes : Card
crumblingAshes =
  Macros.card "Crumbling Ashes"
       (Just [Macros.generic 1, Macros.pip Black]) []
       (MkTypeLine [] [Enchantment])
       [ Triggered At (BeginningOf Upkeep (ByWord Yours))
                   (Macros.destroy
                      (Macros.target
                         (And [Macros.creature,
                               HasCounters (Just Macros.minusOneMinusOne)]))) ]
       Nothing

public export
damningVerdict : Card
damningVerdict =
  Macros.card "Damning Verdict"
       (Just [Macros.generic 3, Macros.pip White, Macros.pip White]) []
       (MkTypeLine [] [Sorcery])
       [ Spell (Macros.destroy
                  (AllOf (And [Macros.creature, Not (HasCounters Nothing)]))) ]
       Nothing

public export
hazardousConditions : Card
hazardousConditions =
  Macros.card "Hazardous Conditions"
       (Just [Macros.generic 2, Macros.pip Black, Macros.pip Green]) []
       (MkTypeLine [] [Sorcery])
       [ Spell (Macros.gets (AllOf (And [Macros.creature, Not (HasCounters Nothing)]))
                            (PtDown (Lit 2)) (PtDown (Lit 2))
                            (Just Macros.untilEndOfTurn)) ]
       Nothing

public export
hunterOfEyeblights : Card
hunterOfEyeblights =
  Macros.card "Hunter of Eyeblights"
       (Just [Macros.generic 3, Macros.pip Black, Macros.pip Black]) []
       (MkTypeLine [Elf, Assassin] [Creature])
       [ Triggered When (Enters Macros.thisCreature)
                   (PutCounters (Lit 1) Macros.plusOnePlusOne
                      (Macros.target Macros.creatureYouDontControl))
       , Activated (Compound [Mana [Macros.generic 2, Macros.pip Black], TapSymbol])
                   (Macros.destroy
                      (Macros.target (And [Macros.creature, HasCounters Nothing]))) ]
       (Just (3, 3))

public export
pridemalkin : Card
pridemalkin =
  Macros.card "Pridemalkin"
       (Just [Macros.generic 2, Macros.pip Green]) []
       (MkTypeLine [Cat] [Creature])
       [ Triggered When (Enters Macros.thisCreature)
                   (PutCounters (Lit 1) Macros.plusOnePlusOne
                      (Macros.target Macros.creatureYouControl))
       , Static (Gains (Each (And [Macros.creature, ControlledBy You,
                                   HasCounters (Just Macros.plusOnePlusOne)]))
                       (KeywordAbility Trample)) ]
       (Just (2, 1))



public export
glacialChasm : Card
glacialChasm =
  Macros.card "Glacial Chasm" Nothing []
       (MkTypeLine [] [Land])
       [ Macros.keywordCosting CumulativeUpkeep (Macros.payLife You 2)
       , Triggered When (Enters Macros.thisLand)
                   (Macros.sacrifice You (Macros.a Macros.land))
       , Static (Deontic (AllOf Macros.creatureYouControl) Forbid Attack Agent NoDeonticPatient)
       , Static (Prevents AnyDamage AllOfIt (Macros.shieldingIt You) Nothing Nothing) ]
       Nothing

public export
aboroth : Card
aboroth =
  Macros.card "Aboroth"
       (Just [Macros.generic 4, Macros.pip Green, Macros.pip Green]) []
       (MkTypeLine [Elemental] [Creature])
       [ Macros.keywordCosting CumulativeUpkeep
                               (Do (PutCounters (Lit 1) Macros.minusOneMinusOne
                                      Macros.thisCreature)) ]
       (Just (9, 9))

public export
shelteringAncient : Card
shelteringAncient =
  Macros.card "Sheltering Ancient"
       (Just [Macros.generic 1, Macros.pip Green]) []
       (MkTypeLine [Treefolk] [Creature])
       [ KeywordAbility Trample
       , Macros.keywordCosting CumulativeUpkeep
                               (Do (PutCounters (Lit 1) Macros.plusOnePlusOne
                                      (Macros.a (And [Macros.creature,
                                                      ControlledBy Macros.anOpponent])))) ]
       (Just (5, 5))

public export
polarKraken : Card
polarKraken =
  Macros.card "Polar Kraken"
       (Just [Macros.generic 8, Macros.pip Blue, Macros.pip Blue, Macros.pip Blue]) []
       (MkTypeLine [Kraken] [Creature])
       [ KeywordAbility Trample
       , Static (Macros.entersTapped Macros.thisCreature)
       , Macros.keywordCosting CumulativeUpkeep (Do (Macros.sacrifice You (Macros.a Macros.land))) ]
       (Just (11, 11))

public export
yavimayaAnts : Card
yavimayaAnts =
  Macros.card "Yavimaya Ants"
       (Just [Macros.generic 2, Macros.pip Green, Macros.pip Green]) []
       (MkTypeLine [Insect] [Creature])
       [ KeywordAbility Trample
       , KeywordAbility Haste
       , Macros.keywordCosting CumulativeUpkeep (Mana [Macros.pip Green, Macros.pip Green]) ]
       (Just (5, 1))

public export
illusionaryForces : Card
illusionaryForces =
  Macros.card "Illusionary Forces"
       (Just [Macros.generic 3, Macros.pip Blue]) []
       (MkTypeLine [Illusion] [Creature])
       [ KeywordAbility Flying
       , Macros.keywordCosting CumulativeUpkeep (Mana [Macros.pip Blue]) ]
       (Just (4, 4))

public export
vexingSphinx : Card
vexingSphinx =
  Macros.card "Vexing Sphinx"
       (Just [Macros.generic 1, Macros.pip Blue, Macros.pip Blue]) []
       (MkTypeLine [Sphinx] [Creature])
       [ KeywordAbility Flying
       , Macros.keywordCosting CumulativeUpkeep (Do (Macros.discardsACard You))
       , Triggered When (Dies Macros.thisCreature)
                   (Draw You (CountersOn Age It)) ]
       (Just (4, 4))

public export
manaChains : Card
manaChains =
  Macros.card "Mana Chains" (Just [Macros.pip Blue]) []
       (MkTypeLine [Aura] [Enchantment])
       [ Macros.keywordSubject Enchant Macros.creature
       , Static (Gains (AttachHost Enchanted (TypeW Creature))
                       (Macros.keywordCosting CumulativeUpkeep (Mana [Macros.generic 1]))) ]
       Nothing

public export
coverOfWinterPut : Ability
coverOfWinterPut =
  Activated (Mana [SnowMana])
            (PutCounters (Lit 1) Age Macros.thisEnchantment)



public export
odricLunarchMarshal : Card
odricLunarchMarshal =
  Macros.card "Odric, Lunarch Marshal"
       (Just [Macros.generic 3, Macros.pip White]) [Legendary]
       (MkTypeLine [Human, Soldier] [Creature])
       [ AlsoForKeywords
           (Macros.triggeredIf At
                               (BeginningOf Combat (ByWord EachPlayers))
                               (Exists (And [Macros.creature, ControlledBy You,
                                             HasKeyword FirstStrike]))
                               (Continuously
                                  (Gains (AllOf Macros.creatureYouControl)
                                         (KeywordAbility FirstStrike))
                                  (Just Macros.untilEndOfTurn)))
           [Flying, Deathtouch, DoubleStrike, Haste, Hexproof, Indestructible,
            Lifelink, Menace, Reach, Skulk, Trample, Vigilance] ]
       (Just (3, 3))

public export
bleedingEffect : Card
bleedingEffect =
  Macros.card "Bleeding Effect"
       (Just [Macros.generic 2, Macros.pip White, Macros.pip Black]) []
       (MkTypeLine [] [Enchantment])
       [ AlsoForKeywords
           (Macros.triggeredIf At
                               (BeginningOf Combat (ByWord Yours))
                               (Exists (And [Macros.creature,
                                             InZone (Macros.graveyardOf You),
                                             HasKeyword Flying]))
                               (Continuously
                                  (Gains (AllOf Macros.creatureYouControl)
                                         (KeywordAbility Flying))
                                  (Just Macros.untilEndOfTurn)))
           [FirstStrike, DoubleStrike, Deathtouch, Hexproof, Indestructible,
            Lifelink, Menace, Reach, Trample, Vigilance] ]
       Nothing

public export
urborgScavengers : Card
urborgScavengers =
  Macros.card "Urborg Scavengers"
       (Just [Macros.generic 2, Macros.pip Black]) []
       (MkTypeLine [Spirit] [Creature])
       [ Macros.triggeredOr Whenever
                            (Enters Macros.thisCreature)
                            (Attacks Macros.thisCreature)
                            (Sequentially
                               [ Macros.exile (Macros.target (InZone Macros.graveyardZ))
                               , PutCounters (Lit 1) Macros.plusOnePlusOne Macros.thisCreature ])
       , AlsoForKeywords
           (Static (Conditionally
                      (Exists (And [ExiledWith Macros.thisCreature, HasKeyword Flying]))
                      (Gains Macros.thisCreature (KeywordAbility Flying))))
           [FirstStrike, DoubleStrike, Deathtouch, Haste, Hexproof, Indestructible,
            Lifelink, Menace, Reach, Trample, Vigilance] ]
       (Just (2, 2))


public export
sengirVampire : Card
sengirVampire =
  Macros.card "Sengir Vampire"
       (Just [Macros.generic 3, Macros.pip Black, Macros.pip Black]) []
       (MkTypeLine [Vampire] [Creature])
       [ KeywordAbility Flying
       , Triggered Whenever
           (Dies (Macros.a (And [Macros.creature,
                                 Macros.happenedToInvolving DamageTaken
                                                            Lookback.ThisTurn
                                                            Macros.thisCreature])))
           (PutCounters (Lit 1) Macros.plusOnePlusOne Macros.thisCreature) ]
       (Just (4, 4))

public export
wickedAkuba : Card
wickedAkuba =
  Macros.card "Wicked Akuba" (Just [Macros.pip Black, Macros.pip Black]) []
       (MkTypeLine [Spirit] [Creature])
       [ Activated (Mana [Macros.pip Black])
           (Macros.losesLife
              (Macros.target (And [AnyPlayer,
                                   Macros.happenedToInvolving DamageTaken
                                                              Lookback.ThisTurn
                                                              Macros.thisCreature]))
              (Lit 1)) ]
       (Just (2, 2))

public export
fyndhornDruid : Card
fyndhornDruid =
  Macros.card "Fyndhorn Druid" (Just [Macros.generic 2, Macros.pip Green]) []
       (MkTypeLine [Elf, Druid] [Creature])
       [ Macros.triggeredIf When
                            (Dies Macros.thisCreature)
                            (Happened BlockedDeclaration It Lookback.ThisTurn)
                            (Macros.gainsLife You (Lit 4)) ]
       (Just (2, 2))

public export
dreamThief : Card
dreamThief =
  Macros.card "Dream Thief" (Just [Macros.generic 2, Macros.pip Blue]) []
       (MkTypeLine [Faerie, Rogue] [Creature])
       [ KeywordAbility Flying
       , Triggered When (Enters Macros.thisCreature)
           (If Macros.drawACard
               (Macros.happenedInvolving SpellCast You Lookback.ThisTurn
                                         (Macros.a (And [Macros.spell, ColorIs Blue,
                                                         OtherThan This])))
               Nothing) ]
       (Just (2, 1))

public export
brightspearZealot : Card
brightspearZealot =
  Macros.card "Brightspear Zealot" (Just [Macros.generic 2, Macros.pip White]) []
       (MkTypeLine [Human, Soldier] [Creature])
       [ KeywordAbility Vigilance
       , Static (Conditionally
           (CompareAmt (Macros.eventCountInvolving SpellCast
                                                   You
                                                   Lookback.ThisTurn
                                                   (Macros.a Macros.spell))
                       AtLeast (Lit 2))
           (Gets Macros.thisCreature (PtUp (Lit 2)) (PtUp (Lit 0)))) ]
       (Just (2, 4))

public export
deepwayNavigator : Card
deepwayNavigator =
  Macros.card "Deepway Navigator" (Just [Macros.pip White, Macros.pip Blue]) []
       (MkTypeLine [Merfolk, Wizard] [Creature])
       [ KeywordAbility Flash
       , Triggered When (Enters Macros.thisCreature)
           (SetStatus Untapped
              (Each (And [HasSubtype Merfolk, ControlledBy You,
                          OtherThan Macros.thisCreature])))
       , Static (Conditionally
           (Macros.happenedInvolving AttackDeclaration
                                     You
                                     Lookback.ThisTurn
                                     (CountedGroup (Macros.atLeast 3)
                                        (HasSubtype Merfolk)))
           (Gets (AllOf (And [HasSubtype Merfolk, ControlledBy You]))
                 (PtUp (Lit 1)) (PtUp (Lit 0)))) ]
       (Just (2, 2))

public export
idolOfOblivion : Card
idolOfOblivion =
  Macros.card "Idol of Oblivion" (Just [Macros.generic 2]) []
       (MkTypeLine [] [Artifact])
       [ Macros.activatedOnlyIf TapSymbol
                                Macros.drawACard
                                (Macros.happenedInvolving TokenCreation
                                                          You
                                                          Lookback.ThisTurn
                                                          (Macros.a IsToken))
       , Activated (Compound [Mana [Macros.generic 8], TapSymbol,
                              Do (Macros.sacrifice You Macros.thisArtifact)])
           (Macros.create (Lit 1) (Macros.creatureTok 10 10 [] [Eldrazi])) ]
       Nothing

public export
mildManneredLibrarian : Card
mildManneredLibrarian =
  Macros.card "Mild-Mannered Librarian" (Just [Macros.pip Green]) []
       (MkTypeLine [Human] [Creature])
       [ Macros.activatedOnlyOnce (Mana [Macros.generic 3, Macros.pip Green])
                                  (Sequentially
                                     [ Continuously (SetsType Macros.thisCreature
                                         (MkToken Nothing [] (Macros.subtypesOnly [Werewolf]) [] Nothing)
                                         Nothing) Nothing
                                     , PutCounters (Lit 2) Macros.plusOnePlusOne It
                                     , Macros.drawACard ])
                                  OncePerGame ]
       (Just (1, 1))


public export
secretsOfTheDead : Card
secretsOfTheDead =
  Macros.card "Secrets of the Dead"
       (Just [Macros.generic 2, Macros.pip Blue]) []
       (MkTypeLine [] [Enchantment])
       [ Triggered Whenever
           (Casts You (Macros.a (And [Macros.spell,
                                      CastFrom (Macros.graveyardOf You)])))
           Macros.drawACard ]
       Nothing

public export
ashZealot : Card
ashZealot =
  Macros.card "Ash Zealot" (Just [Macros.pip Red, Macros.pip Red]) []
       (MkTypeLine [Human, Warrior] [Creature])
       [ KeywordAbility FirstStrike
       , KeywordAbility Haste
       , Triggered Whenever
           (Casts (Macros.a AnyPlayer)
                  (Macros.a (And [Macros.spell, CastFrom Macros.graveyardZ])))
           (DealDamage Macros.thisCreature (Lit 3) (That PlayerW)) ]
       (Just (2, 2))

public export
laquatussDisdain : Card
laquatussDisdain =
  Macros.card "Laquatus's Disdain" (Just [Macros.generic 1, Macros.pip Blue]) []
       (MkTypeLine [] [Instant])
       [ Spell (Sequentially
           [ Macros.counterSpell
               (Macros.target (And [Macros.spell, CastFrom Macros.graveyardZ]))
           , Macros.drawACard ]) ]
       Nothing

public export
coalStoker : Card
coalStoker =
  Macros.card "Coal Stoker" (Just [Macros.generic 3, Macros.pip Red]) []
       (MkTypeLine [Elemental] [Creature])
       [ Macros.triggeredIf When
                            (Enters Macros.thisCreature)
                            (Matches It (CastFrom (Macros.handOf You)))
                            (AddMana You (Lit 3) (Runs [[OfColor Red, OfColor Red, OfColor Red]]) []) ]
       (Just (3, 3))

public export
vegaTheWatcher : Card
vegaTheWatcher =
  Macros.card "Vega, the Watcher"
       (Just [Macros.generic 1, Macros.pip White, Macros.pip Blue]) [Legendary]
       (MkTypeLine [Bird, Spirit] [Creature])
       [ KeywordAbility Flying
       , Triggered Whenever
           (Casts You (Macros.a (And [Macros.spell,
                                      Not (CastFrom (Macros.handOf You))])))
           Macros.drawACard ]
       (Just (2, 2))

public export
patricianGeist : Card
patricianGeist =
  Macros.card "Patrician Geist" (Just [Macros.generic 2, Macros.pip Blue]) []
       (MkTypeLine [Spirit, Knight] [Creature])
       [ KeywordAbility Flying
       , Static (Gets (AllOf (And [HasSubtype Spirit, ControlledBy You,
                                   OtherThan Macros.thisCreature]))
                      (PtUp (Lit 1)) (PtUp (Lit 1)))
       , Static (CostsToCast
                   (AllOf (And [Macros.spell, CastBy You,
                                CastFrom (Macros.graveyardOf You)]))
                   (CostLess (Lit 1))) ]
       (Just (2, 2))


public export
oust : Card
oust =
  Macros.card "Oust" (Just [Macros.pip White]) [] (MkTypeLine [] [Sorcery])
       [ Spell (Sequentially
                  [ Move (Macros.target Macros.creature) (Macros.nthFromTop Second)
                  , Macros.gainsLife (ControllerOf It) (Lit 3) ]) ]
       Nothing

public export
chronostutter : Card
chronostutter =
  Macros.card "Chronostutter" (Just [Macros.generic 5, Macros.pip Blue]) []
       (MkTypeLine [] [Instant])
       [ Spell (Move (Macros.target Macros.creature) (Macros.nthFromTop Second)) ]
       Nothing

public export
shatteredEgo : Card
shatteredEgo =
  Macros.card "Shattered Ego" (Just [Macros.pip Blue]) []
       (MkTypeLine [Aura] [Enchantment])
       [ Macros.keywordSubject Enchant Macros.creature
       , Static (Gets (AttachHost Enchanted (TypeW Creature))
                      (PtDown (Lit 3)) (PtDown (Lit 0)))
       , Activated (Mana [Macros.generic 3, Macros.pip Blue, Macros.pip Blue])
                   (Move (AttachHost Enchanted (TypeW Creature))
                         (Macros.nthFromTop Third)) ]
       Nothing

public export
gandalfWhiteRider : Card
gandalfWhiteRider =
  Macros.card "Gandalf, White Rider" (Just [Macros.generic 3, Macros.pip White])
       [Legendary] (MkTypeLine [Avatar, Wizard] [Creature])
       [ KeywordAbility Vigilance
       , Triggered Whenever (Casts You (Macros.a Macros.spell))
                   (Sequentially
                      [ Macros.gets (Each Macros.creatureYouControl)
                                    (PtUp (Lit 1)) (PtUp (Lit 0))
                                    (Just Macros.untilEndOfTurn)
                      , Does You Scry (Macros.lookAt (Macros.topCards 1)) ])
       , Triggered When (Dies Macros.thisCreature)
                   (Macros.may You (Move It (Macros.nthFromTop Fifth))) ]
       (Just (3, 3))


public export
approachOfTheSecondSun : Card
approachOfTheSecondSun =
  Macros.card "Approach of the Second Sun"
       (Just [Macros.generic 6, Macros.pip White]) [] (MkTypeLine [] [Sorcery])
       [ Spell (If (Concludes WinGame You)
                   (AndCond
                      [ Matches This (CastFrom (Macros.handOf You))
                      , Macros.happenedInvolving SpellCast
                                                 You
                                                 ThisGame
                                                 (Macros.a (And [Macros.spell, OtherThan This,
                                                                 Named (PrintedName
                                                                          "Approach of the Second Sun")])) ])
                   (Just (Sequentially
                            [ Move This (Macros.nthFromTop Seventh)
                            , Macros.gainsLife You (Lit 7) ]))) ]
       Nothing

public export
kamiOfTerribleSecrets : Card
kamiOfTerribleSecrets =
  Macros.card "Kami of Terrible Secrets" (Just [Macros.generic 3, Macros.pip Black]) []
       (MkTypeLine [Spirit] [Creature])
       [ Macros.triggeredIf When
                            (Enters Macros.thisCreature)
                            (AndCond [ Exists (And [Macros.artifact, ControlledBy You])
                                     , Exists (And [Macros.enchantment, ControlledBy You]) ])
                            (Sequentially [ Draw You (Lit 1), Macros.gainsLife You (Lit 1) ]) ]
       (Just (3, 4))

public export
bloodfireEnforcers : Card
bloodfireEnforcers =
  Macros.card "Bloodfire Enforcers" (Just [Macros.generic 3, Macros.pip Red]) []
       (MkTypeLine [Human, Monk] [Creature])
       [ Static (Conditionally
                   (AndCond [ Exists (And [Macros.instant, InZone (Macros.graveyardOf You)])
                            , Exists (And [Macros.sorcery, InZone (Macros.graveyardOf You)]) ])
                   (AndAlso [ Gains Macros.thisCreature (KeywordAbility FirstStrike)
                            , Gains It (KeywordAbility Trample) ])) ]
       (Just (5, 2))

public export
helmOfPossession : Card
helmOfPossession =
  Macros.card "Helm of Possession" (Just [Macros.generic 4]) []
       (MkTypeLine [] [Artifact])
       [ Static (MayDeclineUntap Macros.thisArtifact)
       , Activated (Compound [Mana [Macros.generic 2], TapSymbol,
                              Do (Macros.sacrifice You (Macros.a Macros.creature))])
                   (Macros.gainControl (Macros.target Macros.creature)
                      (Just (ForAsLongAs
                               (AndCond [ Matches Macros.thisArtifact (ControlledBy You)
                                        , Matches Macros.thisArtifact Macros.tapped ])))) ]
       Nothing


public export
override : Card
override =
  Macros.card "Override" (Just [Macros.generic 2, Macros.pip Blue]) []
       (MkTypeLine [] [Instant])
       [ Spell (Macros.mayElse (ControllerOf (Macros.target Macros.spell))
                               (Pay They (ScaledMana (Macros.forEach
                                            (And [Macros.artifact, ControlledBy You]))))
                               (Macros.counterSpell It)) ]
       Nothing

public export
rakshasasDisdain : Card
rakshasasDisdain =
  Macros.card "Rakshasa's Disdain" (Just [Macros.generic 2, Macros.pip Blue]) []
       (MkTypeLine [] [Instant])
       [ Spell (Macros.mayElse (ControllerOf (Macros.target Macros.spell))
                               (Pay They (ScaledMana (Macros.forEach
                                            (InZone (Macros.graveyardOf You)))))
                               (Macros.counterSpell It)) ]
       Nothing

public export
fettergeist : Card
fettergeist =
  Macros.card "Fettergeist" (Just [Macros.generic 2, Macros.pip Blue]) []
       (MkTypeLine [Spirit] [Creature])
       [ KeywordAbility Flying
       , Triggered At (BeginningOf Upkeep (ByWord Yours))
           (Macros.mayElse You
              (Pay You (ScaledMana (Macros.forEach
                          (And [Macros.creature, ControlledBy You,
                                OtherThan Macros.thisCreature]))))
              (Macros.sacrifice You Macros.thisCreature)) ]
       (Just (3, 4))

public export
megatherium : Card
megatherium =
  Macros.card "Megatherium" (Just [Macros.generic 2, Macros.pip Green]) []
       (MkTypeLine [Beast] [Creature])
       [ KeywordAbility Trample
       , Triggered When (Enters Macros.thisCreature)
           (Macros.mayElse You
              (Pay You (ScaledMana (Macros.forEach (InZone (Macros.handOf You)))))
              (Macros.sacrifice You Macros.thisCreature)) ]
       (Just (4, 4))


public export
riseFromTheGrave : Effect []
riseFromTheGrave =
  Sequentially [Macros.putOntoBattlefieldUnderYourControl (Macros.target (And [Macros.creature, InZone (ZoneAt Graveyard Bare)])),
                Macros.becomesAs (That (TypeW Creature))
                                 (MkToken Nothing [Black] (MkTypeLine [Zombie] []) [] Nothing)
                                 Nothing]

public export
dreadSlaver : Card
dreadSlaver =
  Macros.card "Dread Slaver" (Just [Macros.generic 3, Macros.pip Black, Macros.pip Black]) []
       (MkTypeLine [Zombie, Horror] [Creature])
       [ Triggered When (Dies (Macros.a (And [Macros.creature,
                                              Macros.happenedToInvolving DamageTaken
                                                                         Lookback.ThisTurn
                                                                         Macros.thisCreature])))
           (Sequentially [Macros.putOntoBattlefieldUnderYourControl It,
                          Macros.becomesAs (That (TypeW Creature))
                                           (MkToken Nothing [Black] (MkTypeLine [Zombie] []) [] Nothing)
                                           Nothing]) ]
       (Just (3, 5))

public export
stormOfSouls : Effect []
stormOfSouls =
  Sequentially [Move (AllOf (And [Macros.creature, InZone (Macros.graveyardOf You)]))
                     Macros.battlefieldZ,
                Macros.becomesAs Them
                                 (MkToken (Just (Lit 1, Lit 1)) [] (MkTypeLine [Spirit] [])
                                          [KeywordAbility Flying] Nothing)
                                 Nothing,
                Macros.exile This]

public export
everAfter : Effect []
everAfter =
  Sequentially [Move (TargetGroup (Macros.upTo 2) (And [Macros.creature, InZone (Macros.graveyardOf You)]))
                     Macros.battlefieldZ,
                Macros.becomesAs (Those (TypeW Creature))
                                 (MkToken Nothing [Black] (MkTypeLine [Zombie] []) [] Nothing)
                                 Nothing,
                Move This (LibraryAt (OneEnd OnBottom) Nothing Bare)]

public export
infernalVessel : Card
infernalVessel =
  Macros.card "Infernal Vessel" (Just [Macros.generic 2, Macros.pip Black]) []
       (MkTypeLine [Human, Cleric] [Creature])
       [ Macros.triggeredIf When
                            (Dies Macros.thisCreature)
                            (Macros.itIsntA (HasSubtype Demon))
                            (Sequentially [Macros.returnToBattlefieldWithCounters It
                                                                                  (OwnerOf It)
                                                                                  (Lit 2)
                                                                                  Macros.plusOnePlusOne,
                                           Macros.becomesAs It
                                                            (MkToken Nothing [] (MkTypeLine [Demon] []) [] Nothing)
                                                            Nothing]) ]
       (Just (2, 1))

public export
answeredPrayers : Card
answeredPrayers =
  Macros.card "Answered Prayers" (Just [Macros.generic 1, Macros.pip White, Macros.pip White]) []
       (MkTypeLine [] [Enchantment])
       [ Triggered When (Enters (Macros.a Macros.creatureYouControl))
           (Sequentially [Macros.gainsLife You (Lit 1),
                          If (Macros.becomesAs Macros.thisEnchantment
                                               (MkToken (Just (Lit 3, Lit 3)) []
                                                        (MkTypeLine [Angel] [Creature])
                                                        [KeywordAbility Flying] Nothing)
                                               (Just Macros.untilEndOfTurn))
                             (NotCond (Matches Macros.thisEnchantment Macros.creature))
                             Nothing]) ]
       Nothing




public export
killingWave : Effect []
killingWave =
  ForEachOf (Each Macros.creature)
            (Macros.mayElse (ControllerOf It)
                            (Pay They (Do (ChangeLife They (Down XVal))))
                            (Macros.sacrifice They It))

public export
fadeAway : Effect []
fadeAway =
  ForEachOf (Each Macros.creature)
            (Macros.mayElse (ControllerOf It)
                            (Pay They (Mana [Macros.generic 1]))
                            (Macros.sacrifice They (Macros.a Permanent)))

public export
martyrsCry : Effect []
martyrsCry =
  Sequentially [Macros.exile (AllOf (And [Macros.creature, ColorIs White])),
                ForEachOf (ThoseVerbed Exile (TypeW Creature))
                          (Draw (ControllerOf It) (Lit 1))]

public export
descentOfTheDragons : Effect []
descentOfTheDragons =
  Sequentially [Macros.destroy (TargetGroup Macros.anyNumber Macros.creature),
                ForEachOf (Macros.thoseVerbedThisWay Destroy (TypeW Creature))
                          (Create (ControllerOf It) (Lit 1)
                                  (TokenWritten (MkToken (Just (Lit 4, Lit 4)) [Red]
                                                         (MkTypeLine [Dragon] [Creature])
                                                         [KeywordAbility Flying] Nothing))
                                  [])]

public export
tidalFlats : Card
tidalFlats =
  Macros.card "Tidal Flats" (Just [Macros.pip Blue]) []
       (MkTypeLine [] [Enchantment])
       [ Activated (Mana [Macros.pip Blue, Macros.pip Blue])
           (ForEachOf (Each (And [Macros.creature, Attacking,
                                  Not (HasKeyword Flying)]))
                      (Macros.mayElse (ControllerOf It)
                                      (Pay They (Mana [Macros.generic 1]))
                                      (Macros.gains
                                         (AllOf (And [Macros.creature, ControlledBy You,
                                                      BlockerOf (That (TypeW Creature))]))
                                         (KeywordAbility FirstStrike)
                                         (Just Macros.untilEndOfTurn)))) ]
       Nothing


public export
adNauseam : Card
adNauseam =
  Macros.card "Ad Nauseam"
       (Just [Macros.generic 3, Macros.pip Black, Macros.pip Black]) []
       (MkTypeLine [] [Instant])
       [ Spell (Sequentially
              [ Macros.revealCards Macros.topCard
              , Move (That CardW) Macros.handZ
              , Macros.losesLife You (Macros.manaValueOf It)
              , Macros.may You (Repeat AnyNumber) ]) ]
       Nothing

public export
primalSurge : Card
primalSurge =
  Macros.card "Primal Surge"
       (Just [Macros.generic 8, Macros.pip Green, Macros.pip Green]) []
       (MkTypeLine [] [Sorcery])
       [ Spell (Sequentially
              [ Macros.exile Macros.topCard
              , If (Macros.mayThen You (Macros.putOntoBattlefield It)
                                       (Repeat Again))
                   (Macros.itsA Permanent)
                   Nothing ]) ]
       Nothing

public export
cultivatorColossus : Card
cultivatorColossus =
  Macros.cardOf "Cultivator Colossus"
       (Just [Macros.generic 4, Macros.pip Green, Macros.pip Green,
              Macros.pip Green]) []
       (MkTypeLine [Plant, Beast] [Creature])
       [ KeywordAbility Trample
       , Static (DefinesPt Macros.thisCreature BothEach
                           (CountOf (And [Macros.land, ControlledBy You])))
       , Triggered When (Enters Macros.thisCreature)
           (Macros.mayThen You
              (Macros.putOntoBattlefieldTapped
                 (Macros.a (And [Macros.land, InZone (Macros.handOf You)])))
              (Sequentially [Macros.drawACard, Repeat Again])) ]
       (Just (PrintedStar, PrintedStar))

public export
zimoneAndDina : Ability
zimoneAndDina =
  Activated (Compound [TapSymbol,
                       Do (Macros.sacrifice You
                             (Macros.a (Macros.otherCreature Macros.thisCreature)))])
    (Sequentially
       [ Macros.drawACard
       , Macros.may You (Macros.putOntoBattlefieldTapped
                           (Macros.a (And [Macros.land,
                                           InZone (Macros.handOf You)])))
       , If (Repeat (MoreTimes (Lit 1)))
            (CompareAmt (CountOf (And [Macros.land, ControlledBy You]))
                        AtLeast (Lit 8))
            Nothing ])

public export
anotherRound : Effect []
anotherRound =
  Sequentially [ Macros.exile (CountedGroup Macros.anyNumber
                                 Macros.creatureYouControl)
               , Macros.putOntoBattlefield Them
               , Repeat (MoreTimes XVal) ]

||| "{T}: You gain 1 life. Activate only during your turn, before attackers are declared."
public export
shuFarmer : Card
shuFarmer =
  Macros.card "Shu Farmer" (Just [Macros.generic 1, Macros.pip White]) []
       (MkTypeLine [Human] [Creature])
       [ Macros.activatedOnlyDuring TapSymbol
                                    (Macros.gainsLife You (Lit 1))
                                    (BeforePoint AttackersDeclared (Just Yours)) ]
       (Just (1, 1))

||| The loyalty ability a player activates, as the event's complement:
||| "a loyalty ability of enchanted planeswalker".
public export
loyaltyAbilityOfEnchanted : Noun bs Ability
loyaltyAbilityOfEnchanted =
  Macros.a (And [ AbilityHead LoyaltyClass
                , AbilityOf (AttachHost Enchanted (TypeW Planeswalker)) ])

public export
elspethsTalent : Card
elspethsTalent =
  Macros.card "Elspeth's Talent"
       (Just [Macros.generic 2, Macros.pip White, Macros.pip White]) []
       (MkTypeLine [Aura] [Enchantment])
       [ Macros.keywordSubject Enchant (HasType Planeswalker)
       , Static (Gains (AttachHost Enchanted (TypeW Planeswalker))
                       (Activated (LoyaltySymbol (LoyaltyUp 1))
                                  (Macros.create (Lit 3)
                                     (Macros.creatureTok 1 1 [White] [Soldier]))))
       , Triggered Whenever
                   (Activates You loyaltyAbilityOfEnchanted)
                   (Continuously
                      (AndAlso [ Gets (AllOf Macros.creatureYouControl)
                                      (PtUp (Lit 2)) (PtUp (Lit 2))
                               , Gains Them (KeywordAbility Vigilance) ])
                      (Just Macros.untilEndOfTurn)) ]
       Nothing

||| The Chain Veil's first line: the activation read back as a negative lookback.
public export
chainVeilEndStep : Ability
chainVeilEndStep =
  Macros.triggeredIf At
                     (BeginningOf EndStep (ByWord Yours))
                     (Macros.notSo
                        (Macros.happenedInvolving AbilityActivation You
                           Lookback.ThisTurn
                           (Macros.a (And [ AbilityHead LoyaltyClass
                                          , AbilityOf (Macros.a (HasType Planeswalker)) ]))))
                     (Macros.losesLife You (Lit 2))

public export
curseOfTheBloodyTome : Card
curseOfTheBloodyTome =
  Macros.card "Curse of the Bloody Tome" (Just [Macros.generic 2, Macros.pip Blue]) []
       (MkTypeLine [Aura, Curse] [Enchantment])
       [ Macros.keywordSubject Enchant AnyPlayer
       , Triggered At
                   (Macros.beginningOfPossessed Upkeep (AttachHost Enchanted PlayerW))
                   (Macros.mills (That PlayerW) (Lit 2) They) ]
       Nothing

public export
shriekingAffliction : Card
shriekingAffliction =
  Macros.card "Shrieking Affliction" (Just [Macros.pip Black]) []
       (MkTypeLine [] [Enchantment])
       [ Macros.triggeredIf At
                            (BeginningOf Upkeep (ByWord EachOpponents))
                            (CompareAmt (CountOf (InZone (Macros.handOf (That PlayerW))))
                                        AtMost (Lit 1))
                            (Macros.losesLife They (Lit 3)) ]
       Nothing

||| Galvanic Blast's replacement clause. English elides the recipient
||| ("deals 4 damage instead"); the clause reads the announced target back.
public export
galvanicBlastLine : Effect []
galvanicBlastLine =
  InsteadOf (DealDamage This (Lit 2) (Macros.target AnyTarget))
            (If (DealDamage This (Lit 4) (That JoinW))
                (CompareAmt (CountOf (And [Macros.artifact, ControlledBy You]))
                            AtLeast (Lit 3))
                Nothing)

public export
cryptLurker : Card
cryptLurker =
  Macros.card "Crypt Lurker" (Just [Macros.generic 3, Macros.pip Black]) []
       (MkTypeLine [Horror] [Creature])
       [ Triggered When (Enters Macros.thisCreature)
           (Macros.mayThen You
              (Macros.chooseOne
                 [ Macros.sacrifice You (Macros.a Macros.creature)
                 , Macros.discards You (Macros.a (And [Macros.creature,
                                                       InZone Macros.handZ])) ])
              Macros.drawACard) ]
       (Just (3, 4))

||| Peacekeeper's second line: the bare generic plural as a deontic subject.
public export
peacekeeperCant : Ability
peacekeeperCant = Static (Deontic (AllOf Macros.creature) Forbid Attack Agent NoDeonticPatient)


||| Memoricide's search clause: "Search target player's graveyard, hand,
||| and library for … cards with that name. Then that player shuffles."
||| The name choice heads the card; the sweep is what was missing.
||| ("… and exile them" wants a plural search result, which is not the
||| sweep's gap.)
public export
memoricideSearch : Effect []
memoricideSearch =
  Sequentially [ Choose (Macros.a (Macros.qualityFrom CardName
                                     (NameOfCard (Not (HasType Land)))))
               , Macros.searchZonesOf (Macros.target AnyPlayer) (Named ChosenName)
               , Shuffle (That PlayerW) ]

||| Eradicate's search clause: "Exile target nonblack creature. Search its
||| controller's graveyard, hand, and library for all cards with the same
||| name as that creature. Then that player shuffles." The same sweep,
||| anchored on a co-referential possessor instead of a target. The card
||| writes "that creature"; the exile has already rebound it as a card in
||| exile [CR#400.7], so the type word finds no antecedent and `CardW` is
||| what the binding offers.
public export
eradicateSearch : Effect []
eradicateSearch =
  Sequentially [ Macros.exile (Macros.target (And [Macros.creature,
                                                   Not (ColorIs Black)]))
               , Macros.searchZonesOf (ControllerOf (That CardW))
                                      (Named (SameNameAs (That CardW)))
               , Shuffle (That PlayerW) ]

||| Deem Inferior: "The owner of target nonland permanent puts it into
||| their library second from the top or on the bottom." The agentive
||| placement clause carrying the offset spelling of the disjunction.
public export
deemInferior : Effect []
deemInferior =
  Macros.puts (OwnerOf (Macros.target (And [Permanent, Not (HasType Land)])))
              It
              (Macros.nthFromTopOrBottomZ Second)

||| Lost Hours' third line: "That player puts that card into their library
||| third from the top." The agentive clause over a plain ordinal.
public export
lostHoursPlacement : Effect []
lostHoursPlacement =
  Sequentially [ Macros.revealsTheirHand (Macros.target AnyPlayer)
               , Choose (Macros.a (And [Not (HasType Land),
                                        InZone (Macros.handOf They)]))
               , Macros.puts (That PlayerW) It (Macros.nthFromTop Third) ]

||| Aether Gust's second line: "Its owner puts it on their choice of the
||| top or bottom of their library." The agentive clause with the chooser
||| slot filled by the subject.
public export
aetherGustPlacement : Effect []
aetherGustPlacement =
  Sequentially [ Choose (Macros.target (And [Permanent, ColorIs Red]))
               , Macros.puts (OwnerOf It) It (Macros.choiceOfTopOrBottom They) ]

||| Not Forgotten's first line: "Put target card from a graveyard on your
||| choice of the top or bottom of its owner's library." The same
||| disjunction under an imperative, with the chooser slot spelling "your".
public export
notForgottenPlacement : Effect []
notForgottenPlacement =
  Move (Macros.target (InZone Macros.graveyardZ)) (Macros.choiceOfTopOrBottom You)

||| Write into Being's placement half: "put the other on the top or bottom
||| of your library" — the bare disjunction, no chooser named. (Manifest is
||| not in the vocabulary; the exile stands in for the card it consumes.)
public export
writeIntoBeingPlacement : Effect []
writeIntoBeingPlacement =
  Sequentially [ Macros.lookAt (Macros.topCards 2)
               , Macros.exile (Macros.oneOf Them)
               , Move TheRest Macros.topOrBottomZ ]

||| Culling Mark: "Target creature blocks this turn if able." The block
||| requirement with its patient left out.
public export
cullingMark : Card
cullingMark =
  Macros.card "Culling Mark" (Just [Macros.generic 2, Macros.pip Green]) []
       (MkTypeLine [] [Sorcery])
       [ Spell (Continuously (Deontic (Macros.target Macros.creature)
                                      Require Block Agent NoDeonticPatient)
                             (Just Macros.thisTurn)) ]
       Nothing

||| Blazing Archon's second line: "Creatures can't attack you." The
||| restriction naming the player the attack is aimed at [CR#506.3].
public export
blazingArchonCant : Ability
blazingArchonCant =
  Static (Deontic (AllOf Macros.creature) Forbid Attack Agent (DefendingPlayer You))

||| Clergy of the Holy Nimbus: "If this creature would be destroyed,
||| regenerate it." A replacement effect over a destruction. The
||| self-mention stands in for the printed "it": a replacement's event
||| mints no subject binding for its own noun.
public export
clergyOfTheHolyNimbus : Ability
clergyOfTheHolyNimbus =
  Static (Intercepts (IsDestroyed Macros.thisCreature)
                     (Regenerate Macros.thisCreature) Repeatedly)

||| Rampant Frogantua's second line: "This creature gets +10/+10 for each
||| player who has lost the game." The game-loss look-back [CR#603.10f].
public export
rampantFrogantuaPump : Ability
rampantFrogantuaPump =
  Static (Gets Macros.thisCreature
               (PtUp (Macros.nForEach 10 (And [AnyPlayer, HappenedTo GameLoss ThisGame])))
               (PtUp (Macros.nForEach 10 (And [AnyPlayer, HappenedTo GameLoss ThisGame]))))

||| Goad's own reminder text, on the creature the goad names: "Until your
||| next turn, that creature … attacks a player other than you if able."
||| [CR#701.15b] The attack requirement naming its defender.
public export
goadedAttacksOther : Effect []
goadedAttacksOther =
  Continuously (Deontic (Macros.target Macros.creature) Require Attack Agent
                        (DefendingPlayer (Macros.a (And [AnyPlayer, OtherThan You]))))
               (Just Macros.untilYourNextTurn)


||| Fastbond's land line: "You may play any number of lands on each of your
||| turns." [CR#305.2] lets a continuous effect raise the number a player may
||| play, and this one takes the ceiling off. Its damage trigger needs an
||| ordinal read of the lands already played this turn.
public export
fastbondLands : Ability
fastbondLands = Static (MayPlayAdditionalLands You Macros.anyNumber)

||| Furious Reprisal: "Furious Reprisal deals 2 damage to each of two
||| targets." The counted group over the class word, a form [CR#115.4]
||| names itself. Pinnacle of Rage writes the same line at 3 damage.
public export
furiousReprisal : Effect []
furiousReprisal =
  DealDamage This (Lit 2) (EachOf (TargetGroup (Macros.exactly 2) AnyTarget))

||| Maskwood Nexus's first line: "Creatures you control are every creature
||| type. The same is true for creature spells you control and creature
||| cards you own that aren't on the battlefield." The off-battlefield
||| extension over a type addition, [CR#205.3m]'s shared subtype list.
public export
maskwoodNexusTypes : Ability
maskwoodNexusTypes =
  Static (AlsoOffBattlefield
            (AddsEveryType (AllOf Macros.creatureYouControl) CreatureSpace))

||| Mystical Tutor: "Search your library for an instant or sorcery card,
||| reveal it, then shuffle and put that card on top." [CR#701.24b] keeps
||| the found card out of the shuffle, so the discourse still holds it.
public export
mysticalTutor : Effect []
mysticalTutor =
  Sequentially [ Macros.searchLibraryFor Macros.instantOrSorcery
               , Macros.revealCards It
               , Macros.shuffle
               , Move It (LibraryAt (OneEnd OnTop) Nothing Bare) ]

||| Demonic Tutor: "Search your library for a card, put that card into your
||| hand, then shuffle." The bare description a single-zone search names.
public export
demonicTutor : Effect []
demonicTutor =
  Sequentially [ Macros.searchLibraryFor (And [])
               , Move It Macros.handZ
               , Macros.shuffle ]

||| Thalia's Lancers' search line: "search your library for a legendary
||| card, reveal it, put it into your hand, then shuffle." A supertype word
||| heads nothing and still describes a set. Its printed trigger frame
||| ("When this creature enters, you may …") is left off: the entering
||| permanent is a second Object mention, and "it" then has two antecedents.
public export
thaliasLancersSearch : Effect []
thaliasLancersSearch =
  Sequentially [ Macros.searchLibraryFor (And [HasSupertype Legendary])
               , Macros.revealCards It
               , Move It Macros.handZ
               , Macros.shuffle ]

||| Brimaz, King of Oreskos' attack trigger: "Whenever Brimaz attacks,
||| create a 1/1 white Cat Soldier creature token with vigilance that's
||| attacking." [CR#508.4] designates the token attacking and taps
||| nothing, so the attacking rider stands alone.
public export
brimazAttackToken : Ability
brimazAttackToken =
  Triggered Whenever (Attacks Macros.thisCreature)
    (Create You (Lit 1)
            (TokenWritten (MkToken (Just (Lit 1, Lit 1)) [White]
                                   (MkTypeLine [Cat, Soldier] [Creature])
                                   [KeywordAbility Vigilance] Nothing))
            [EntersAttacking])

||| Luxior, Giada's Gift's second line: "Equipped permanent … is a
||| creature in addition to its other types." [CR#301.5f] lets the word
||| "equipped" name whatever the permanent is attached to, so the host
||| word is the host's own.
public export
luxiorEquippedPermanent : Ability
luxiorEquippedPermanent =
  Static (BecomesAlso (AttachHost Equipped PermanentW)
                      (MkToken Nothing [] (MkTypeLine [] [Creature]) [] Nothing))

||| Nahiri, the Unforgiving's [0] read: "creature card with mana value
||| less than Nahiri's loyalty from your graveyard". [CR#109.3] lists
||| loyalty among an object's characteristics and [CR#306.5] gives it to
||| planeswalkers alone. The printed head is "creature or Equipment
||| card"; the disjunction is trimmed here and waits on the union
||| redesign.
public export
nahiriLoyaltyRead : Predicate [] Object
nahiriLoyaltyRead =
  And [ Macros.creature, InZone (Macros.graveyardOf You)
      , Compare ManaValue Less (StatOf Loyalty This) ]

||| Vivien's Talent: "Whenever a nontoken creature you control enters, put
||| a loyalty counter on enchanted planeswalker." [CR#122.1] makes a
||| loyalty counter a marker like any other.
public export
viviensTalentTrigger : Ability
viviensTalentTrigger =
  Triggered Whenever (Enters (Macros.a (And [Macros.nontoken,
                                             Macros.creatureYouControl])))
    (PutCounters (Lit 1) LoyaltyCounter
                 (AttachHost Enchanted (TypeW Planeswalker)))

||| "Put a +1/+1 counter on target creature card in your graveyard."
||| [CR#122.1a] counts a +X/+Y counter on a creature card in a zone other
||| than the battlefield.
public export
counterOnGraveyardCard : Effect []
counterOnGraveyardCard =
  PutCounters (Lit 1) Macros.plusOnePlusOne
              (Macros.target (And [Macros.creature,
                                   InZone (Macros.graveyardOf You)]))

||| Ascend's own reminder text: "you get the city's blessing for the rest
||| of the game" [CR#702.131a].
public export
ascendConferral : Effect []
ascendConferral = Macros.getsCitysBlessing

||| Saddle's expansion body: "This permanent becomes saddled until end of
||| turn" [CR#702.171a].
public export
saddleConferral : Effect []
saddleConferral = Macros.becomesSaddled

||| Bioessence Hydra's second line: "Whenever one or more loyalty counters
||| are put on planeswalkers you control, put that many +1/+1 counters on
||| this creature." The loyalty counter read as the marker [CR#122.1] it
||| is, on the plural subject the line names.
public export
bioessenceHydraTrigger : Ability
bioessenceHydraTrigger =
  Triggered Whenever
    (CounterEvent CounterPut LoyaltyCounter
                  (AllOf (And [HasType Planeswalker, ControlledBy You])))
    (PutCounters ThatMuch Macros.plusOnePlusOne Macros.thisCreature)

||| Void Maw's activated ability: "Put a card exiled with this creature
||| into its owner's graveyard: This creature gets +2/+2 until end of
||| turn." A placement paid as a cost [CR#118.1].
public export
voidMawPutCost : Ability
voidMawPutCost =
  Activated (Do (Macros.puts You (Macros.a (ExiledWith Macros.thisCreature))
                             Macros.graveyardZ))
            (Macros.gets Macros.thisCreature (PtUp (Lit 2)) (PtUp (Lit 2))
                         (Just Macros.untilEndOfTurn))

||| Bullseye, Death Dealer's activated ability: "{3}, {T}, Sacrifice an
||| artifact or discard a nonland card: Bullseye deals 2 damage to any
||| target." The cost the player chooses between [CR#118.1].
public export
bullseyeModalCost : Ability
bullseyeModalCost =
  Activated (Compound [Mana [Macros.generic 3], TapSymbol,
                       Do (Macros.chooseOne
                             [Macros.sacrifice You (Macros.a Macros.artifact),
                              Macros.discards You
                                (Macros.a (And [Not Macros.land,
                                                InZone Macros.handZ]))])])
            (DealDamage This (Lit 2) (Macros.target AnyTarget))

||| Sedris, the Traitor King: "Each creature card in your graveyard has
||| unearth {2}{B}." The grant reaches a subject in a graveyard because
||| unearth's own ability functions there [CR#702.84a].
public export
sedrisTheTraitorKing : Ability
sedrisTheTraitorKing =
  Static (Gains (Each (And [Macros.creature, InZone (Macros.graveyardOf You)]))
                (Macros.keywordCosting Unearth
                   (Mana [Macros.generic 2, Macros.pip Black])))

||| Grixis: "Blue, black, and/or red creature cards in your graveyard have
||| unearth. The unearth cost is equal to the card's mana cost." The
||| second sentence fixes the parameter [CR#202.1a].
public export
grixis : Ability
grixis =
  Static (Gains (AllOf (And [Or [ColorIs Blue, ColorIs Black, ColorIs Red],
                             Macros.creature,
                             InZone (Macros.graveyardOf You)]))
                (Macros.keywordCosting Unearth ItsManaCost))

||| Dralnu, Lich Lord: "Target instant or sorcery card in your graveyard
||| gains flashback until end of turn. The flashback cost is equal to its
||| mana cost." Flashback's graveyard static is named by [CR#702.34a].
public export
dralnuLichLord : Effect []
dralnuLichLord =
  Macros.gains (Macros.target (And [Macros.instantOrSorcery,
                                    InZone (Macros.graveyardOf You)]))
               (Macros.keywordCosting Flashback ItsManaCost)
               (Just Macros.untilEndOfTurn)

||| Dregscape Zombie: "Unearth {B}" [CR#702.84a].
public export
dregscapeZombie : Ability
dregscapeZombie = Macros.keywordCosting Unearth (Mana [Macros.pip Black])

||| Think Twice: "Flashback {2}{U}" [CR#702.34a].
public export
thinkTwice : Ability
thinkTwice =
  Macros.keywordCosting Flashback (Mana [Macros.generic 2, Macros.pip Blue])

||| Greater Mossdog: "Dredge 3" [CR#702.52a].
public export
greaterMossdog : Ability
greaterMossdog = Macros.keywordNumber Dredge (Lit 3)

||| Raven's Crime: "Retrace" [CR#702.81a].
public export
ravensCrime : Ability
ravensCrime = KeywordAbility Retrace

||| Barkhide Mauler: "Cycling {2}" [CR#702.29a].
public export
barkhideMauler : Ability
barkhideMauler = Macros.keywordCosting Cycling (Mana [Macros.generic 2])

||| Ninja of the New Moon: "Ninjutsu {3}{B}" [CR#702.49a].
public export
ninjaOfTheNewMoon : Ability
ninjaOfTheNewMoon =
  Macros.keywordCosting Ninjutsu (Mana [Macros.generic 3, Macros.pip Black])

||| Thunderous Wrath: "Miracle {R}" [CR#702.94a].
public export
thunderousWrath : Ability
thunderousWrath = Macros.keywordCosting Miracle (Mana [Macros.pip Red])

||| Bygone Colossus: "Warp {3}" [CR#702.185a].
public export
bygoneColossus : Ability
bygoneColossus = Macros.keywordCosting Warp (Mana [Macros.generic 3])
