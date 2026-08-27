||| The workbench's evidence bench: typechecking positives and pinned negatives over real cards.
module Experimental.Cards

import Experimental
import Experimental.Macros

%default total


||| Lightning Bolt
bolt : Effect []
bolt = DealDamage This (Lit 3) (Macros.target Macros.anyTarget)

barrageOfBoulders : Effect []
barrageOfBoulders = DealDamage This (Lit 1) (Each Macros.creatureYouDontControl)

rabidBite : Effect []
rabidBite = DealDamage (Macros.target Macros.creatureYouControl)
                       (Macros.powerOf It)
                       (Macros.target Macros.creatureYouDontControl)

preyUpon : Effect []
preyUpon = Fights (Macros.target Macros.creatureYouControl) (Macros.target Macros.creatureYouDontControl)

arcTrail : Effect []
arcTrail = Sequentially [DealDamage This (Lit 2) (Macros.target Macros.anyTarget),
                         DealDamage This (Lit 1) (Macros.target Macros.anyOtherTarget)]

cloudshift : Effect []
cloudshift = Sequentially [Macros.exile (Macros.target Macros.creatureYouControl),
                           Macros.putOntoBattlefieldUnderYourControl (That CardW)]

||| Through the Breach Splice
throughTheBreach : Effect []
throughTheBreach = Sequentially [Macros.may You (Macros.move (Macros.a (And [Macros.creature, InZone (Macros.handOf You)])) Macros.battlefieldZ),
                                 Macros.gainsHaste (That (TypeW Creature)) Nothing,
                                 Macros.delayed (BeginningOf EndStep NoPossessor) (Macros.sacrifice You (That (TypeW Creature)))]

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
pyriteSpellbomb = Macros.activated (Compound [Mana [Macros.pip Red], Do (Macros.sacrifice You Macros.thisArtifact)])
                                   (DealDamage It (Lit 2) (Macros.target Macros.anyTarget))

diabolicEdict : Effect []
diabolicEdict = Macros.sacrifice (Macros.target AnyPlayer) (Macros.aTheirChoice Macros.creature)

innocentBlood : Effect []
innocentBlood = Macros.sacrifice (Each AnyPlayer) (Macros.aTheirChoice Macros.creature)

cryOfContrition : Effect []
cryOfContrition = Macros.discardsACard (Macros.target AnyPlayer)

-- "Draw two cards, then discard two cards." (Careful Study) -- the
-- counted discard in its canonical iterated-singular expansion.
carefulStudy : Effect []
carefulStudy = Sequentially [Macros.drawCards 2, Macros.discardN (Lit 2)]

-- "Discard two cards: Create a 2/2 black Zombie creature token."
-- (Zombie Infestation) -- the same counted discard as a printed COST: a
-- repetition spells one instruction, not a coordination.
zombieInfestation : Ability
zombieInfestation =
  Macros.activated (Do (Macros.discardN (Lit 2)))
                   (Macros.create (Lit 1)
                      (MkToken (Just (Lit 2 ** Lit 2)) [Black]
                               (MkTypeLine [creatureType "Zombie"] [Creature]) [] Nothing))

suspendedSentence : Effect []
suspendedSentence = Sequentially [Macros.destroy (Macros.target (And [Macros.creature, ControlledBy Macros.anOpponent])),
                                  Macros.losesLife (That PlayerW) (Lit 3)]

flickeringSpirit : Effect []
flickeringSpirit = Sequentially [Macros.exile Macros.thisCreature,
                                 Macros.move It Macros.battlefieldZ]

turnToMist : Effect []
turnToMist = Sequentially [Macros.exile (Macros.target Macros.creature),
                           Macros.delayed (BeginningOf EndStep NoPossessor) (Macros.move (That CardW) Macros.battlefieldZ)]

jump : Effect []
jump = Macros.gains (Macros.target Macros.creature) (Macros.keyword "Flying") (Just Macros.untilEndOfTurn)

giantGrowth : Effect []
giantGrowth = Macros.gets (Macros.target Macros.creature) (PtUp (Lit 3)) (PtUp (Lit 3)) (Just Macros.untilEndOfTurn)

glyphOfDestruction : Effect []
glyphOfDestruction =
  Macros.gets (Macros.target (And [Blocking, Macros.creature, ControlledBy You])) (PtUp (Lit 10)) (PtUp (Lit 0)) (Just Macros.untilEndOfCombat)

gabrielAngelfire : Effect []
gabrielAngelfire =
  Macros.gains Macros.thisCreature (Macros.keyword "Flying") (Just Macros.untilYourNextUpkeep)

bondOfRevival : Effect []
bondOfRevival = Sequentially [Macros.move (Macros.target (And [Macros.creature, InZone (Macros.graveyardOf You)])) Macros.battlefieldZ,
                              Macros.gainsHaste It (Just Macros.untilYourNextTurn)]

gracefulReprieve : Effect []
gracefulReprieve = Macros.delayedWithin (Dies (Macros.target Macros.creature))
                                        ThisTurn
                                        (Macros.move (That CardW) Macros.battlefieldZ)

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
fulgentDistraction = Sequentially [Macros.choose (TargetGroup (Macros.exactly 2) Macros.creature),
                                   SetStatus Tapped (Those (TypeW Creature))]

||| Continue?
continueSpell : Effect []
continueSpell = Sequentially [Macros.choose (TargetGroup (Macros.upTo 4) (And [Macros.creature, InZone (Macros.graveyardOf You)])),
                              Macros.move Them Macros.battlefieldZ]

suddenDemise : Effect []
suddenDemise = Sequentially [Macros.choose (Macros.a (Macros.quality Color)),
                             DealDamage This (LetterVal X) (Each (And [Macros.creature, OfChosen Color]))]

kindredDominance : Effect []
kindredDominance = Sequentially [Macros.choose (Macros.a (Macros.quality CreatureType)),
                                 Macros.destroy (AllOf (And [Macros.creature, Not (OfChosen CreatureType)]))]

voyagerStaff : Ability
voyagerStaff = Macros.activated (Compound [Mana [Macros.generic 2], Do (Macros.sacrifice You Macros.thisArtifact)])
                                (Sequentially [Macros.exile (Macros.target Macros.creature),
                                          Macros.delayed (BeginningOf EndStep NoPossessor) (Macros.move (Macros.theVerbed "Exile" CardW) Macros.battlefieldZ)])

boshIronGolem : Ability
boshIronGolem = Macros.activated (Compound [Mana [Macros.generic 3, Macros.pip Red],
                                     Do (Macros.sacrifice You (Macros.a (HasType Artifact)))])
                                 (DealDamage This
                                      (Macros.manaValueOf (Macros.theVerbed "Sacrifice" (TypeW Artifact)))
                                      (Macros.target Macros.anyTarget))

pyromancy : Ability
pyromancy = Macros.activated (Compound [Mana [Macros.generic 3],
                                 Do (Macros.discards You (Macros.aAtRandom (InZone Macros.handZ)))])
                             (DealDamage Macros.thisEnchantment
                                  (Macros.manaValueOf (Macros.theVerbed "Discard" CardW))
                                  (Macros.target Macros.anyTarget))

foulTongueShriek : Effect []
foulTongueShriek = Sequentially [Macros.losesLife (Macros.target Opponent)
                                           (Macros.forEach (And [Attacking, Macros.creature, ControlledBy You])),
                                 Macros.gainsLife You ThatMuch]

unsummon : Effect []
unsummon = Macros.move (Macros.target Macros.creature) Macros.handZ

phantomBlade : Effect []
phantomBlade = Sequentially [Macros.choose (TargetGroup (Macros.upTo 1) (And [Macros.creature, ControlledBy You])),
                             Macros.destroy (TargetGroup (Macros.upTo 1) (And [Macros.creature, Other]))]

caseOfTheGatewayExpress : Effect []
caseOfTheGatewayExpress = Sequentially [Macros.choose (Macros.target Macros.creatureYouDontControl),
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

-- "{W}, {T}: Tap target creature." (Master Decoy) -- the same body
-- `icyManipulator` writes bare, here NAMED as the keyword action
-- [CR#701.26a]. Tap joined the vocabulary as a macro and a label row and
-- nothing else: no constructor, no table, no coverage re-decide.
masterDecoy : Ability
masterDecoy =
  Macros.activated (Compound [Mana [Macros.pip White], TapSymbol])
                   (Macros.tap (Macros.target Macros.creature))

-- "Tap any number of untapped creatures you control. You gain 4 life for
-- each creature tapped this way." (Harmony of Nature) -- the participle
-- read a labeled STATUS change leaves behind, exactly as `martyrsCry`
-- reads an exile's.
harmonyOfNature : Effect []
harmonyOfNature =
  Sequentially [ Macros.tap (CountedGroup Macros.anyNumber
                              (And [Macros.untapped, Macros.creature, ControlledBy You]))
               , ForEachOf (Macros.thoseVerbedThisWay "Tap" (TypeW Creature))
                           (Macros.gainsLife You (Lit 4)) ]

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
overload = OnlyIf (Macros.destroy (Macros.target Macros.artifact))
                  (CompareAmt (Macros.manaValueOf It) AtMost (Lit 2))
                  Nothing

builtToSmash : Effect []
builtToSmash =
  Sequentially [Macros.gets (Macros.target (And [Macros.creature, Attacking])) (PtUp (Lit 3)) (PtUp (Lit 3)) (Just Macros.untilEndOfTurn),
                If (Macros.itsA (And [Macros.artifact, Macros.creature]))
                   (Macros.gains It (Macros.keyword "Trample") (Just Macros.untilEndOfTurn))
                   Nothing]

flamesOfTheRazeBoar : Effect []
flamesOfTheRazeBoar =
  Sequentially [DealDamage This (Lit 4) (Macros.target (And [Macros.creature, ControlledBy Macros.anOpponent])),
                OnlyIf (DealDamage This (Lit 2)
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
                  (RemoveCounters (Lit 1) (Just Macros.plusOnePlusOne) Macros.thisCreature)

yawgmothDemon : Effect []
yawgmothDemon =
  Macros.mayElse You (Macros.sacrifice You (Macros.a Macros.artifact))
              (Sequentially [SetStatus Tapped Macros.thisCreature, DealDamage This (Lit 2) You])


raiseTheAlarm : Effect []
raiseTheAlarm = Macros.create (Lit 2) (Macros.creatureTok 1 1 [White] [creatureType "Soldier"])

additiveEvolution : Effect []
additiveEvolution = Sequentially [Macros.create (Lit 1) (Macros.creatureTok 0 0 [Green, Blue] [creatureType "Fractal"]),
                                  PutCounters (Lit 3) Macros.plusOnePlusOne It]

aviationPioneer : Effect []
aviationPioneer =
  Macros.create (Lit 1) (MkToken (Just (Lit 1 ** Lit 1)) [] (MkTypeLine [creatureType "Thopter"] [Artifact, Creature])
                          [Macros.keyword "Flying"] Nothing)

fireNavyTrebuchet : Effect []
fireNavyTrebuchet =
  Macros.createTappedAttacking (Lit 1)
    (MkToken (Just (Lit 2 ** Lit 1)) [] (MkTypeLine [creatureType "Construct"] [Artifact, Creature])
             [Macros.keyword "Flying"] (Just "Ballistic Boulder"))

amassZombiesTwo : Effect []
amassZombiesTwo =
  Sequentially [If (Macros.notSo (Exists (And [HasSubtype (creatureType "Army"), Macros.creature, ControlledBy You])))
                   (Macros.create (Lit 1) (Macros.creatureTok 0 0 [Black] [creatureType "Zombie", creatureType "Army"]))
                   Nothing,
                Macros.choose (Macros.a (And [HasSubtype (creatureType "Army"), Macros.creature, ControlledBy You])),
                PutCounters (Lit 2) Macros.plusOnePlusOne (That (TypeW Creature)),
                If (Macros.itIsntA (HasSubtype (creatureType "Zombie")))
                   (Macros.becomes It (Macros.subtypesOnly [creatureType "Zombie"]) Nothing)
                   Nothing]

battlegrowth : Effect []
battlegrowth = PutCounters (Lit 1) Macros.plusOnePlusOne (Macros.target Macros.creature)

chainbreaker : Effect []
chainbreaker = RemoveCounters (Lit 1) (Just Macros.minusOneMinusOne) (Macros.target Macros.creature)

kaitoBaneOfNightmares : Effect []
kaitoBaneOfNightmares = Sequentially [SetStatus Tapped (Macros.target Macros.creature),
                                      PutCounters (Lit 2) Stun It]

jhoiraOfTheGhitu : Ability
jhoiraOfTheGhitu =
  Macros.activated (Compound [Mana [Macros.generic 2],
                       Do (Macros.exile (Macros.a (And [Not Macros.land, InZone (Macros.handOf You)])))])
                   (PutCounters (Lit 4) Time (Macros.theVerbed "Exile" CardW))

alaundoTheSeer : Effect []
alaundoTheSeer = RemoveCounters (Lit 1) (Just Time) (Each (InZone Macros.exileZ))

arcBlade : Effect []
arcBlade = Sequentially [DealDamage This (Lit 2) (Macros.target Macros.anyTarget),
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
  Macros.activated (Compound [TapSymbol, Do (Macros.sacrifice You Macros.thisArtifact)])
                   (Sequentially [PutCounters (Lit 1) Macros.plusOnePlusOne
                                         (Macros.target (And [Macros.creature, Not Macros.artifact])),
                             Macros.becomes (That (TypeW Creature)) (Macros.typesOnly [Artifact]) Nothing])

neurokTransmuter : Effect []
neurokTransmuter = Macros.becomes (Macros.target Macros.creature) (Macros.typesOnly [Artifact]) (Just Macros.untilEndOfTurn)

cowardKiller : Effect []
cowardKiller = Sequentially [Macros.cantBlock (Macros.target Macros.creature) (Just Macros.thisTurn),
                             Macros.becomes (That (TypeW Creature)) (Macros.subtypesOnly [creatureType "Coward"])
                                     (Just Macros.untilEndOfTurn)]

clavilenoPhrase : Predicate [] Object
clavilenoPhrase = And [Macros.creature, Attacking, Not (HasSubtype (creatureType "Demon"))]

unholyAnnex : Effect []
unholyAnnex =
  Sequentially [Macros.drawACard,
                If (Exists (And [HasSubtype (creatureType "Demon"), ControlledBy You]))
                   (Sequentially [Macros.losesLife (Each Opponent) (Lit 2),
                                  Macros.gainsLife You (Lit 2)])
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
       (MkTypeLine [creatureType "Goblin"] [Creature])
       [ Macros.triggered Whenever (Macros.attacks Macros.thisCreature)
                          (Sequentially
                     [ Macros.choose (Macros.a (Macros.quality CardName))
                     , Continuously
                         (CostsToCast (AllOf (And [Macros.spell, OfChosen CardName]))
                                      (CostLess (Lit 1)))
                         (Just Macros.thisTurn) ]) ]
       (Just (2, 2))

public export
prosperity : Card
prosperity =
  Macros.card "Prosperity" (Just [Variable, Macros.pip Blue]) []
       (MkTypeLine [] [Sorcery]) [Spell (Draw (Each AnyPlayer) (LetterVal X))] Nothing

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
cycling = Macros.activated (Compound [Mana [Macros.generic 2], Do (Macros.discards You This)]) Macros.drawACard

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
myrkulsEdict = Sequentially [Macros.choose (Macros.a Opponent),
                             Macros.sacrifice (That PlayerW) (Macros.aTheirChoice Macros.creature)]



nibelheimAflame : Effect []
nibelheimAflame =
  Sequentially [Macros.choose (Macros.target Macros.creatureYouControl),
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
                  (TokenWritten (Macros.creatureTok 1 1 [Green] [creatureType "Plant"])) []

sparkmagesGambit : Effect []
sparkmagesGambit =
  Sequentially [DealDamage This (Lit 1) (EachOf (TargetGroup (Macros.upTo 2) Macros.creature)),
                Macros.cantBlock (Those (TypeW Creature)) (Just Macros.thisTurn)]

ajaniAdversaryOfTyrants : Effect []
ajaniAdversaryOfTyrants =
  PutCounters (Lit 1) Macros.plusOnePlusOne (EachOf (TargetGroup (Macros.upTo 2) Macros.creature))

fallOfTheTitans : Effect []
fallOfTheTitans = DealDamage This (LetterVal X) (EachOf (TargetGroup (Macros.upTo 2) Macros.anyTarget))

naturesPanoply : Effect []
naturesPanoply =
  Sequentially [Macros.choose (TargetGroup Macros.anyNumber Macros.creature),
                PutCounters (Lit 1) Macros.plusOnePlusOne (EachOf Them)]

arcLightning : Effect []
arcLightning = Macros.dealsDivided This (Lit 3) (TargetGroup (Macros.oneThrough 3) Macros.anyTarget)

forkedBolt : Effect []
forkedBolt = Macros.dealsDivided This (Lit 2) (TargetGroup (Macros.oneThrough 2) Macros.anyTarget)

boulderfall : Effect []
boulderfall = Macros.dealsDivided This (Lit 5) (TargetGroup Macros.anyNumber Macros.anyTarget)

armamentCorps : Effect []
armamentCorps =
  Macros.distributeCounters (Lit 2) Macros.plusOnePlusOne
                     (TargetGroup (Macros.oneThrough 2) Macros.creatureYouControl)



peek : Effect []
peek = Sequentially [Macros.lookAtHandOf (Macros.target AnyPlayer), Macros.drawACard]

impulse : Effect []
impulse = Sequentially [ Macros.lookAt (Macros.topCards 4)
                       , Macros.move (Macros.oneOf Them) Macros.handZ
                       , Macros.move TheRest (Macros.onBottomIn AnyOrder)
                       ]

anticipate : Effect []
anticipate = Sequentially [ Macros.lookAt (Macros.topCards 3)
                          , Macros.move (Macros.oneOf Them) Macros.handZ
                          , Macros.move TheRest (Macros.onBottomIn AnyOrder)
                          ]

revealFourPartition : Effect []
revealFourPartition = Sequentially [ Macros.revealCards (Macros.topCards 4)
                                   , Macros.move (Macros.oneOf (Those CardW)) Macros.handZ
                                   , Macros.move TheRest Macros.graveyardZ
                                   ]

exileFourOfThem : Effect []
exileFourOfThem = Sequentially [ Macros.lookAt (Macros.topCards 8)
                               , Macros.exile (Macros.someOf 4 Them)
                               , Macros.move TheRest (Macros.onTopIn AnyOrder)
                               ]

||| Grenzo, Dungeon Warden
grenzoBottomCard : Effect []
grenzoBottomCard = Macros.move Macros.bottomCard Macros.graveyardZ

sylvanScrying : Effect []
sylvanScrying = Sequentially [ Macros.searchLibraryFor Macros.land
                             , Macros.revealCards It
                             , Macros.move It Macros.handZ
                             , Macros.shuffle
                             ]

searchToBattlefield : Effect []
searchToBattlefield = Sequentially [ Macros.searchLibraryFor Macros.creature
                                   , Macros.move It Macros.battlefieldZ
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
  Sequentially [Macros.lookAt Macros.topCard, Macros.may You (Macros.move (That CardW) Macros.graveyardZ)]



botBashingTime : Effect []
botBashingTime =
  Sequentially [ DealDamage This (Lit 6) (Macros.target Macros.creature)
               , Macros.ifWouldInstead (Dies (That (TypeW Creature))) (Macros.exile It) (Just Macros.thisTurn)
               ]

wordsOfWar : Effect []
wordsOfWar = Macros.nextTimeWouldInstead (Draws You)
                                  (DealDamage This (Lit 2) (Macros.target Macros.anyTarget))
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
  Macros.preventNext AnyDamage (Lit 3) (Macros.shieldingIt (Macros.target Macros.anyTarget)) (Just Macros.thisTurn)

banisherPriest : Effect []
banisherPriest = Macros.exileUntil (Macros.target (And [Macros.creature, ControlledBy Macros.anOpponent]))
                            (Leaves Macros.thisCreature)

||| Tezzeret, Artifice Master
tezzeretDrawTwo : Effect []
tezzeretDrawTwo =
  Macros.insteadOf Macros.drawACard
            (If (CompareAmt (CountOf (And [Macros.artifact, ControlledBy You]))
                            AtLeast (Lit 3))
                (Macros.drawCards 2)
                Nothing)

||| Zimone, Quandrix Prodigy
zimoneDrawTwo : Effect []
zimoneDrawTwo =
  Macros.insteadOf Macros.drawACard
            (If (CompareAmt (CountOf (And [Macros.land, ControlledBy You]))
                            AtLeast (Lit 8))
                (Macros.drawCards 2)
                Nothing)


merrowGrimeblotter : Ability
merrowGrimeblotter =
  Macros.activated (Compound [Mana [Macros.generic 1, Macros.hybridPip Blue Black], UntapSymbol])
                   (Macros.gets (Macros.target Macros.creature) (PtDown (Lit 2)) (PtDown (Lit 0)) (Just Macros.untilEndOfTurn))

phyrexianSnowcrusher : Ability
phyrexianSnowcrusher =
  Macros.activated (Mana [Macros.generic 1, SnowMana])
                   (Macros.gets Macros.thisCreature (PtUp (Lit 1)) (PtUp (Lit 0)) (Just Macros.untilEndOfTurn))

havocSower : Ability
havocSower =
  Macros.activated (Mana [Macros.generic 1, Macros.colorlessPip])
                   (Macros.gets Macros.thisCreature (PtUp (Lit 2)) (PtUp (Lit 1)) (Just Macros.untilEndOfTurn))

||| Erebos, God of the Dead
erebos : Ability
erebos = Macros.activated (Compound [Mana [Macros.generic 1, Macros.pip Black], Macros.payLife You 2]) Macros.drawACard

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
       [ Macros.activated TapSymbol (AddMana You (Lit 1) (Runs [[Colorless]]) [])
       , Macros.activatedOnlyIf (Compound [Mana [Macros.generic 3], TapSymbol])
                                Macros.drawACard
                                (Exists (And [Macros.creature, ControlledBy You,
                                              Compare Power AtLeast (Lit 4)])) ]
       Nothing

woeleecher : Ability
woeleecher =
  Macros.activated (Compound [Mana [Macros.pip White], TapSymbol])
                   (Macros.doThen (RemoveCounters (Lit 1) (Just Macros.minusOneMinusOne) (Macros.target Macros.creature))
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
                             (Macros.create (Lit 1) (Macros.creatureTok 1 1 [White] [creatureType "Soldier"]))
                             OncePerTurn
                             (Macros.notSo (Exists (And [Macros.creature, ControlledBy You])))




cloudkinSeer : Ability
cloudkinSeer = Macros.triggered When (Enters Macros.thisCreature) Macros.drawACard

moonlitWake : Ability
moonlitWake = Macros.triggered Whenever (Dies (Macros.a Macros.creature)) (Macros.gainsLife You (Lit 1))

promiseOfTomorrow : Ability
promiseOfTomorrow = Macros.triggered Whenever (Dies (Macros.a Macros.creatureYouControl)) (Macros.exile It)

staffOfNin : Ability
staffOfNin = Macros.triggered At (BeginningOf Upkeep (ByWord Yours)) Macros.drawACard

libraryLarcenist : Ability
libraryLarcenist = Macros.triggered Whenever (Macros.attacks Macros.thisCreature) Macros.drawACard

eliteJavelineer : Ability
eliteJavelineer =
  Macros.triggered Whenever (Blocks Macros.thisCreature Nothing)
                   (DealDamage This (Lit 1) (Macros.target (And [Macros.creature, Attacking])))

jhessianThief : Ability
jhessianThief =
  Macros.triggered Whenever (DealsCombatDamage Macros.thisCreature (Macros.a AnyPlayer)) Macros.drawACard

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
abandonedOutpost = Static (Macros.entersTapped Macros.thisLand)

aerialAssault : Effect []
aerialAssault = Macros.destroy (Macros.target (And [Macros.creature, Macros.tapped]))

asphyxiate : Effect []
asphyxiate = Macros.destroy (Macros.target (And [Macros.creature, Macros.untapped]))

aphettoAlchemist : Ability
aphettoAlchemist = Macros.activated TapSymbol
                                    (SetStatus Untapped (Macros.target (Or [Macros.artifact, Macros.creature])))

||| Time Vault
timeVaultLock : Ability
timeVaultLock = Static (DoesntUntap Macros.thisArtifact)

vindicate : Effect []
vindicate = Macros.destroy (Macros.target Permanent)

yasminKhan : Ability
yasminKhan =
  Macros.activated TapSymbol
                   (Sequentially [Macros.exile Macros.topCard,
                           Continuously (Macros.mayPlay You It) (Just Macros.untilYourNextEndStep)])

||| Brazen Cannonade
brazenCannonadePermission : Effect []
brazenCannonadePermission =
  Sequentially [Macros.exile Macros.topCard,
                Continuously (Macros.mayPlay You It) (Just (Until (EndOf Combat (Just Yours))))]


corneredCrook : Ability
corneredCrook =
  Macros.triggered When (Enters Macros.thisCreature)
    (Macros.mayWhen You (Macros.sacrifice You (Macros.a Macros.artifact))
                 (DealDamage This (Lit 3) (Macros.target Macros.anyTarget)))

||| The Last Ronin
theLastRoninII : Effect []
theLastRoninII =
  Reflexively (Macros.mills You (Lit 4) You)
              (Macros.move (Macros.target (And [Macros.creature, InZone (Macros.graveyardOf You)])) Macros.handZ)

thousandMoonsCrackshot : Ability
thousandMoonsCrackshot =
  Macros.triggered Whenever (Macros.attacks Macros.thisCreature)
    (Macros.mayWhen You (Pay You (Mana [Macros.generic 2, Macros.pip White]))
                 (SetStatus Tapped (Macros.target Macros.creature)))

anointerOfValor : Ability
anointerOfValor =
  Macros.triggered Whenever (Macros.attacks (Macros.a Macros.creature))
    (Macros.mayWhen You (Pay You (Mana [Macros.generic 3]))
                 (PutCounters (Lit 1) Macros.plusOnePlusOne (That (TypeW Creature))))


zimoneQuandrixProdigy : Ability
zimoneQuandrixProdigy =
  Macros.activated (Compound [Mana [Macros.generic 1], TapSymbol])
                   (Macros.may You (Macros.putOntoBattlefieldTapped
                        (Macros.a (And [Macros.land, InZone (Macros.handOf You)]))))

preeminentCaptain : Ability
preeminentCaptain =
  Macros.triggered Whenever (Macros.attacks Macros.thisCreature)
    (Macros.may You (Macros.putOntoBattlefieldTappedAttacking
                (Macros.a (And [HasSubtype (creatureType "Soldier"), Macros.creature, InZone (Macros.handOf You)]))))

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
  Macros.activated (Mana [Macros.generic 3, Macros.pip Black])
                   (Macros.delayed (BeginningOf EndOfCombat NoPossessor)
                            (Macros.destroy (Macros.target (And [Blocking, Macros.creature]))))

workhorse : Card
workhorse =
  Macros.card "Workhorse" (Just [Macros.generic 6]) []
       (MkTypeLine [creatureType "Horse"] [Artifact, Creature])
       [ Static (Macros.entersWithCounters Macros.thisCreature (Lit 4) Macros.plusOnePlusOne)
       , Macros.activated (Do (RemoveCounters (Lit 1) (Just Macros.plusOnePlusOne) Macros.thisCreature))
                          (AddMana You (Lit 1) (Runs [[Colorless]]) []) ]
       (Just (0, 0))


counterspell : Effect []
counterspell = Macros.counterSpell (Macros.target Macros.spell)

beastWhisperer : Ability
beastWhisperer =
  Macros.triggered Whenever (Casts You (Macros.a (And [Macros.creature, Macros.spell]))) Macros.drawACard

skaabRuinator : Ability
skaabRuinator =
  Static (Macros.mayCastFrom You This (Macros.graveyardOf You))


escapeToTheWilds : Effect []
escapeToTheWilds =
  Sequentially [Macros.exile (Macros.topCards 5),
                Continuously
                  (Macros.mayPlay You (Macros.thoseVerbedThisWay "Exile" CardW))
                  (Just (Until (EndOf Turn (Just Yours))))]


scatheZombies : Card
scatheZombies = Macros.card "Scathe Zombies" (Just [Macros.generic 2, Macros.pip Black]) []
                     (MkTypeLine [creatureType "Zombie"] [Creature]) [] (Just (2, 2))

rorixBladewing : Card
rorixBladewing =
  Macros.card "Rorix Bladewing" (Just [Macros.generic 3, Macros.pip Red, Macros.pip Red, Macros.pip Red])
       [Legendary] (MkTypeLine [creatureType "Dragon"] [Creature])
       [Macros.keyword "Flying", Macros.keyword "Haste"] (Just (6, 5))

aladdinsRing : Card
aladdinsRing =
  Macros.card "Aladdin's Ring" (Just [Macros.generic 8]) [] (MkTypeLine [] [Artifact])
       [Macros.activated (Compound [Mana [Macros.generic 8], TapSymbol])
                         (DealDamage Macros.thisArtifact (Lit 4) (Macros.target Macros.anyTarget))]
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
       [Macros.activated (Mana [Macros.pip Red]) (Macros.gets Macros.thisCreature (PtUp (Lit 1)) (PtUp (Lit 0)) (Just Macros.untilEndOfTurn))]
       (Just (-1, 3))

||| Sisters of Stone Death
sistersOfStoneDeathRecall : Ability
sistersOfStoneDeathRecall =
  Macros.activated (Mana [Macros.generic 2, Macros.pip Black])
                   (Macros.putOntoBattlefieldUnderYourControl
               (Macros.a (And [Macros.creature, ExiledWith Macros.thisCreature])))

||| Synod Sanctum
synodSanctumReturn : Ability
synodSanctumReturn =
  Macros.activated (Compound [Mana [Macros.generic 2], Do (Macros.sacrifice You Macros.thisArtifact)])
                   (Macros.putOntoBattlefieldUnderYourControl (AllOf Macros.exiledWithThisArtifact))

coldStorage : Card
coldStorage =
  Macros.card "Cold Storage" (Just [Macros.generic 4]) [] (MkTypeLine [] [Artifact])
       [Macros.activated (Mana [Macros.generic 3]) (Macros.exile (Macros.target Macros.creatureYouControl)),
        Macros.activated (Do (Macros.sacrifice You Macros.thisArtifact))
                                (Macros.putOntoBattlefieldUnderYourControl
                     (Each (And [Macros.creature, Macros.exiledWithThisArtifact])))]
       Nothing

||| Muse Vessel
museVesselPlay : Ability
museVesselPlay =
  Macros.activated (Mana [Macros.generic 1])
                   (Sequentially [Macros.choose (Macros.a Macros.exiledWithThisArtifact),
                           Continuously (Macros.mayPlay You (That CardW)) (Just Macros.thisTurn)])

aerialVolley : Card
aerialVolley =
  Macros.card "Aerial Volley" (Just [Macros.pip Green]) [] (MkTypeLine [] [Instant])
       [Spell (Macros.dealsDivided This (Lit 3)
                            (TargetGroup (Macros.oneThrough 3)
                              (And [Macros.creature, HasKeyword "Flying"])))] Nothing

yotianSoldier : Card
yotianSoldier =
  Macros.card "Yotian Soldier" (Just [Macros.generic 3]) []
       (MkTypeLine [creatureType "Soldier"] [Artifact, Creature])
       [Macros.keyword "Vigilance"] (Just (1, 4))

||| Pym Particles
pymParticlesVigilanceGrant : Effect []
pymParticlesVigilanceGrant =
  Macros.gains (Macros.target Macros.creature) (Macros.keyword "Vigilance") (Just Macros.untilEndOfTurn)

demonicConsultationChoice : Effect []
demonicConsultationChoice = Macros.choose (Macros.a (Macros.quality CardName))

voidChoice : Effect []
voidChoice = Macros.choose (Macros.a (Macros.quality Number))

basicLandCards : List Card
basicLandCards =
  [ Macros.card "Plains" Nothing [Basic] (MkTypeLine [landType "Plains"] [Land]) [] Nothing
  , Macros.card "Island" Nothing [Basic] (MkTypeLine [landType "Island"] [Land]) [] Nothing
  , Macros.card "Swamp" Nothing [Basic] (MkTypeLine [landType "Swamp"] [Land]) [] Nothing
  , Macros.card "Mountain" Nothing [Basic] (MkTypeLine [landType "Mountain"] [Land]) [] Nothing
  , Macros.card "Forest" Nothing [Basic] (MkTypeLine [landType "Forest"] [Land]) [] Nothing
  ]

snowCoveredForest : Card
snowCoveredForest =
  Macros.card "Snow-Covered Forest" Nothing [Basic, Snow]
       (MkTypeLine [landType "Forest"] [Land]) [] Nothing

bladebrand : Effect []
bladebrand =
  Macros.gains (Macros.target Macros.creature) (Macros.keyword "Deathtouch") (Just Macros.untilEndOfTurn)

criticalHit : Effect []
criticalHit =
  Macros.gains (Macros.target Macros.creature) (Macros.keyword "DoubleStrike") (Just Macros.untilEndOfTurn)

lightningBlow : Effect []
lightningBlow =
  Macros.gains (Macros.target Macros.creature) (Macros.keyword "FirstStrike") (Just Macros.untilEndOfTurn)

avianOddity : Effect []
avianOddity = PutCounters (Lit 1) Macros.flyingCounter (Macros.target Macros.creatureYouControl)

||| Song of Eärendil
songOfEarendil : Effect []
songOfEarendil =
  PutCounters (Lit 1) Macros.flyingCounter
              (Each (And [Macros.creature, ControlledBy You, Not (HasKeyword "Flying")]))


deathByDragons : Effect []
deathByDragons =
  Create (Each (And [AnyPlayer, OtherThan (Macros.target AnyPlayer)])) (Lit 1)
         (TokenWritten (MkToken (Just (Lit 5 ** Lit 5)) [Red] (MkTypeLine [creatureType "Dragon"] [Creature])
                                [Macros.keyword "Flying"] Nothing)) []

||| Terrifying Presence
terrifyingPresenceAnchor : Predicate [] Object
terrifyingPresenceAnchor = And [Macros.creature, OtherThan (Macros.target Macros.creature)]

lifeTotalBecomesOne : Effect []
lifeTotalBecomesOne = Macros.lifeTotalBecomes (Macros.target AnyPlayer) (Lit 1)



gideonsAvenger : Ability
gideonsAvenger =
  Macros.triggered Whenever
                   (StatusEvent (Macros.a (And [Macros.creature, ControlledBy Macros.anOpponent])) Tapped)
                   (PutCounters (Lit 1) Macros.plusOnePlusOne Macros.thisCreature)

mesmericOrb : Ability
mesmericOrb =
  Macros.triggered Whenever (StatusEvent (Macros.a Permanent) Untapped)
                   (Macros.mills (ControllerOf (That PermanentW)) (Lit 1) They)


barlsCage : Ability
barlsCage = Macros.activated (Mana [Macros.generic 3])
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
  Macros.activated (Compound [Mana [Macros.generic 2, Macros.pip White], TapSymbol])
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
  Macros.triggered Whenever
                   (StatusEvent (Macros.a (And [Permanent, ControlledBy You])) FaceUp)
                   Macros.drawACard

vodalianIllusionist : Ability
vodalianIllusionist =
  Macros.activated (Compound [Mana [Macros.pip Blue, Macros.pip Blue], TapSymbol])
                   (SetStatus PhasedOut (Macros.target Macros.creature))

||| Teferi's Imp
teferisImpPhasesOut : Ability
teferisImpPhasesOut =
  Macros.triggered Whenever (StatusEvent Macros.thisCreature PhasedOut)
                   (Macros.discardsACard You)

||| Teferi's Imp
teferisImpPhasesIn : Ability
teferisImpPhasesIn =
  Macros.triggered Whenever (StatusEvent Macros.thisCreature PhasedIn)
                   Macros.drawACard

||| Oubliette — [CR#610.4]'s "until" rider on a permanent phasing out,
||| the second one-shot that takes the rider.
oubliette : Ability
oubliette =
  Macros.triggered When (Enters Macros.thisEnchantment)
                   (Macros.phasesOutUntil (Macros.target Macros.creature)
                                   (Leaves Macros.thisEnchantment))

shimmeringEfreet : Ability
shimmeringEfreet =
  Macros.triggered Whenever (StatusEvent Macros.thisCreature PhasedIn)
                   (SetStatus PhasedOut (Macros.target Macros.creature))


labyrinthOfSkophos : Card
labyrinthOfSkophos =
  Macros.card "Labyrinth of Skophos" Nothing [] (MkTypeLine [] [Land])
       [ Macros.activated TapSymbol (AddMana You (Lit 1) (Runs [[Colorless]]) [])
       , Macros.activated (Compound [Mana [Macros.generic 4], TapSymbol])
                          (RemoveFromCombat
                      (Macros.target (And [Macros.creature, Or [Attacking, Blocking]]))) ]
       Nothing

hollowhengeSpirit : Ability
hollowhengeSpirit =
  Macros.triggered When (Enters Macros.thisCreature)
                   (RemoveFromCombat (Macros.target (And [Macros.creature, Or [Attacking, Blocking]])))

netcasterSpider : Ability
netcasterSpider =
  Macros.triggered Whenever
                   (Blocks Macros.thisCreature
                    (Just (Macros.a (And [Macros.creature, HasKeyword "Flying"]))))
                   (Macros.gets Macros.thisCreature (PtUp (Lit 2)) (PtUp (Lit 0)) (Just Macros.untilEndOfTurn))

viashinoWeaponsmith : Ability
viashinoWeaponsmith =
  Macros.triggered Whenever
                   (BecomesBlocked Macros.thisCreature (Just (Macros.a Macros.creature)))
                   (Macros.gets Macros.thisCreature (PtUp (Lit 2)) (PtUp (Lit 2)) (Just Macros.untilEndOfTurn))

somberwaldAlpha : Ability
somberwaldAlpha =
  Macros.triggered Whenever (BecomesBlocked (Macros.a Macros.creatureYouControl) Nothing)
                   (Macros.gets It (PtUp (Lit 1)) (PtUp (Lit 1)) (Just Macros.untilEndOfTurn))

kjeldoranFrostbeast : Ability
kjeldoranFrostbeast =
  Macros.triggered At (BeginningOf EndOfCombat NoPossessor)
                   (Macros.destroy (AllOf (And [Macros.creature,
                                         Or [BlockerOf Macros.thisCreature,
                                             BlockedBy Macros.thisCreature]])))

vertigoSpawn : Ability
vertigoSpawn =
  Macros.triggered Whenever (Blocks Macros.thisCreature (Just (Macros.a Macros.creature)))
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
  Static (Macros.asLongAs (Matches Macros.thisArtifact Macros.untapped)
                          (CantUntapMoreThan (PlayerGroup AllPlayers) 1 Macros.land))

staticOrb : Ability
staticOrb =
  Static (Macros.asLongAs (Matches Macros.thisArtifact Macros.untapped)
                          (CantUntapMoreThan (PlayerGroup AllPlayers) 2 Permanent))


prologueToPhyresis : Effect []
prologueToPhyresis = GetsCounters (Each Opponent) (Lit 1) Poison

screechingScorchbeast : Ability
screechingScorchbeast =
  Macros.triggered Whenever (Macros.attacks Macros.thisCreature)
                   (GetsCounters (Each AnyPlayer) (Lit 2) Rad)

merenOfClanNelToth : Ability
merenOfClanNelToth =
  Macros.triggered Whenever (Dies (Macros.a (Macros.otherCreatureYouControl Macros.thisCreature)))
                   (GetsCounters You (Lit 1) Experience)

||| Bumi, King of Three Trials — "Target player scries 3": the looker is
||| a player other than you [CR#701.22a].
bumiScryMode : Effect []
bumiScryMode = Macros.playerScries (Macros.target AnyPlayer) (Lit 3)

||| Final Act
finalActCounterMode : Effect []
finalActCounterMode = Macros.losesAllCounters (Each Opponent) Nothing

leeches : Effect []
leeches = Macros.losesAllCounters (Macros.target AnyPlayer) (Just Poison)

kratosStoicFather : Ability
kratosStoicFather =
  Macros.triggered At (BeginningOf EndStep (ByWord Yours))
                   (PutCounters (CountersOn Experience You) Macros.plusOnePlusOne
                         (Macros.target Macros.creature))


orneryDilophosaur : Ability
orneryDilophosaur =
  Macros.triggeredIf Whenever
                     (Macros.attacks Macros.thisCreature)
                     (Exists (And [Macros.creature, ControlledBy You,
                                   Compare Power AtLeast (Lit 4)]))
                     (Macros.gets Macros.thisCreature (PtUp (Lit 2)) (PtUp (Lit 2)) (Just Macros.untilEndOfTurn))

incisorGlider : Ability
incisorGlider =
  Macros.triggeredIf Whenever
                     (Macros.attacks Macros.thisCreature)
                     (CompareAmt (CountersOn Poison (Macros.a Opponent))
                                 AtLeast (Lit 3))
                     (Continuously (Gets (AllOf Macros.creatureYouControl) (PtUp (Lit 1)) (PtUp (Lit 1)))
                                   (Just Macros.untilEndOfTurn))


stormFleetSpy : Ability
stormFleetSpy =
  Macros.triggeredIf When
                     (Enters Macros.thisCreature)
                     (Macros.happened AttackDeclaration You Lookback.ThisTurn)
                     Macros.drawACard

vashtaNerada : Ability
vashtaNerada =
  Macros.triggeredIf At
                     (BeginningOf EndStep (ByWord EachPlayers))
                     (Macros.happened Death (Macros.a Macros.creature)
                                      Lookback.ThisTurn)
                     (PutCounters (Lit 1) Macros.plusOnePlusOne Macros.thisCreature)

||| Tippy-Toe, Terrific Partner
tippyToe : Ability
tippyToe =
  Macros.triggeredIf At
                     (BeginningOf EndStep (ByWord Yours))
                     (Macros.happened LifeGain You Lookback.ThisTurn)
                     Macros.drawACard

loanShark : Ability
loanShark =
  Macros.triggeredIf When
                     (Enters Macros.thisCreature)
                     (CompareAmt (Macros.eventCount SpellCast You Lookback.ThisTurn)
                                 AtLeast (Lit 2))
                     Macros.drawACard


sizzlingBarrage : Effect []
sizzlingBarrage =
  DealDamage This (Lit 4)
             (Macros.target (And [Macros.creature,
                                  Macros.happenedTo BlockDeclaration Lookback.ThisTurn]))

witchsMist : Ability
witchsMist =
  Macros.activated (Compound [Mana [Macros.generic 2, Macros.pip Black], TapSymbol])
                   (Macros.destroy (Macros.target (And [Macros.creature,
                                                 Macros.happenedTo DamageTaken Lookback.ThisTurn])))

forceOfDespair : Effect []
forceOfDespair =
  Macros.destroy (AllOf (And [Macros.creature,
                              Macros.happenedTo Entry Lookback.ThisTurn]))

furiousSpinesplitter : Ability
furiousSpinesplitter =
  Macros.triggered At (BeginningOf EndStep (ByWord Yours))
                   (PutCounters (Macros.forEach (And [Opponent,
                                               Macros.happenedTo DamageTaken Lookback.ThisTurn]))
                         Macros.plusOnePlusOne Macros.thisCreature)


winterMoon : Ability
winterMoon =
  Static (CantUntapMoreThan (PlayerGroup AllPlayers) 1
                            (And [Macros.land, Not (HasSupertype Basic)]))

cradleToGrave : Effect []
cradleToGrave =
  Macros.destroy (Macros.target (And [Macros.creature, Not (ColorIs Black),
                                      Macros.happenedTo Entry Lookback.ThisTurn]))

herosDemise : Effect []
herosDemise =
  Macros.destroy (Macros.target (And [Macros.creature, HasSupertype Legendary]))

||| Concordant Crossroads — [CR#205.4a]'s World supertype, printed.
concordantCrossroads : Card
concordantCrossroads =
  Macros.card "Concordant Crossroads" (Just [Macros.pip Green]) [World]
       (MkTypeLine [] [Enchantment])
       [ Static (Gains (AllOf Macros.creature) (Macros.keyword "Haste")) ] Nothing

||| Selenia — a vanguard card [CR#313.1], whose whole printed text is one
||| static. [CR#205.2a]'s vanguard row reaches a real type line, and
||| `cardClassOf` reads it as a `CommandZoneCard`: [CR#313.4] is what
||| licenses the static from the command zone, not the permanent frame.
selenia : Card
selenia =
  Macros.card "Selenia" Nothing [] (MkTypeLine [] [Vanguard])
       [ Static (Gains (AllOf Macros.creatureYouControl)
                       (Macros.keyword "Vigilance")) ] Nothing

||| Pure // Simple
simpleHalf : Effect []
simpleHalf = Macros.destroy (Macros.target (And [Permanent, Multicolored]))

leitmotifComposer : Ability
leitmotifComposer =
  Macros.activated (Mana [Macros.generic 2, Macros.pip Blue])
                   (Continuously (Deontic (AllOf (And [Macros.creature,
                                                Named (PrintedName "Leitmotif Composer")]))
                                   Forbid Block Patient NoDeonticPatient)
                          (Just Macros.thisTurn))


paladinOfAtonement : Ability
paladinOfAtonement =
  Macros.triggeredIf At
                     (BeginningOf Upkeep (ByWord EachPlayers))
                     (Macros.happened LifeLoss You Lookback.LastTurn)
                     (PutCounters (Lit 1) Macros.plusOnePlusOne Macros.thisCreature)

brazenCannonade : Ability
brazenCannonade =
  Macros.triggeredIf At
                     (BeginningOf PostcombatMain (ByWord EachYours))
                     (Macros.happened AttackDeclaration You Lookback.ThisTurn)
                     (Macros.exile Macros.topCard)

fourKnocks : Ability
fourKnocks =
  Macros.triggered At (BeginningOf FirstMain (ByWord Yours)) Macros.drawACard

hammerOfBogardan : Ability
hammerOfBogardan =
  Macros.activatedOnlyDuring (Mana [Macros.generic 2, Macros.pip Red, Macros.pip Red, Macros.pip Red])
                             (Macros.move This Macros.handZ)
                             (DuringPart Upkeep (Just Yours))


berserkersOfBloodRidge : Ability
berserkersOfBloodRidge = Static (Deontic Macros.thisCreature Require Attack Agent NoDeonticPatient)

trumpetingArmodon : Ability
trumpetingArmodon =
  Macros.activated (Mana [Macros.generic 1, Macros.pip Green])
                   (Continuously (Deontic (Macros.target Macros.creature) Require Block Agent
                                   (DeonticCounterpart Macros.thisCreature))
                          (Just Macros.thisTurn))

loathsomeCatoblepas : Ability
loathsomeCatoblepas =
  Macros.activated (Mana [Macros.generic 2, Macros.pip Green])
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
  Macros.triggered When (Enters Macros.thisCreature) (Macros.gainsDesignation You Monarch Instructed)

goadTargetCreature : Ability
goadTargetCreature =
  Macros.activated (Compound [Mana [Macros.generic 3], TapSymbol])
                   (Macros.gainsDesignation (Macros.target Macros.creature) Goaded Instructed)

goadedAttackTrigger : Ability
goadedAttackTrigger =
  Macros.triggered Whenever (Macros.attacks (Macros.a (And [Macros.creature, HasDesignation Goaded])))
                   (DealDamage It (Lit 1) (ControllerOf It))

firmamentSage : Ability
firmamentSage = Macros.triggered Whenever DayNightShift Macros.drawACard


deeprootWarrior : Ability
deeprootWarrior =
  Macros.triggered Whenever (BecomesBlocked Macros.thisCreature Nothing)
                   (Macros.gets It (PtUp (Lit 1)) (PtUp (Lit 1)) (Just Macros.untilEndOfTurn))

borderlandMarauder : Ability
borderlandMarauder =
  Macros.triggered Whenever (Macros.attacks Macros.thisCreature)
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
       (MkTypeLine [enchantmentType "Aura"] [Enchantment])
       [ Macros.keywordSubject "Enchant" Macros.creature
       , Static (Deontic (AttachHost Enchanted (TypeW Creature))
                         Require Attack Agent NoDeonticPatient) ]
       Nothing

brainwash : Card
brainwash =
  Macros.card "Brainwash" (Just [Macros.pip White]) []
       (MkTypeLine [enchantmentType "Aura"] [Enchantment])
       [ Macros.keywordSubject "Enchant" Macros.creature
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
       (MkTypeLine [enchantmentType "Aura"] [Enchantment])
       [ Macros.keywordSubject "Enchant" Macros.creature
       , Macros.triggered Whenever (Macros.attacks (AttachHost Enchanted (TypeW Creature)))
                          (DealDamage It (Lit 2) (Macros.target Macros.anyTarget)) ]
       Nothing


||| Lich's Mastery
lichsMasteryGate : Ability
lichsMasteryGate = Static (OutcomeGate CantLose You)

||| When Lich's Mastery
lichsMasteryLoss : Ability
lichsMasteryLoss =
  Macros.triggered When (Leaves Macros.thisEnchantment) (Concludes LoseGame You)

phageTheUntouchable : Ability
phageTheUntouchable =
  Macros.triggered Whenever
                   (DealsCombatDamage Macros.thisCreature (Macros.a AnyPlayer))
                   (Concludes LoseGame (That PlayerW))


platinumAngel : Card
platinumAngel =
  Macros.card "Platinum Angel" (Just [Macros.generic 7]) []
       (MkTypeLine [creatureType "Angel"] [Artifact, Creature])
       [ Macros.keyword "Flying"
       , Static (OutcomeGate CantLose You)
       , Static (OutcomeGate CantWin (PlayerGroup YourOpponents)) ]
       (Just (4, 4))

abyssalPersecutor : Card
abyssalPersecutor =
  Macros.card "Abyssal Persecutor"
       (Just [Macros.generic 2, Macros.pip Black, Macros.pip Black]) []
       (MkTypeLine [creatureType "Demon"] [Creature])
       [ Macros.keyword "Flying"
       , Macros.keyword "Trample"
       , Static (OutcomeGate CantWin You)
       , Static (OutcomeGate CantLose (PlayerGroup YourOpponents)) ]
       (Just (6, 6))


smogElemental : Card
smogElemental =
  Macros.card "Smog Elemental"
       (Just [Macros.generic 4, Macros.pip Black, Macros.pip Black]) []
       (MkTypeLine [creatureType "Elemental"] [Creature])
       [ Macros.keyword "Flying"
       , Static (Gets (AllOf (And [Macros.creature, HasKeyword "Flying",
                                   ControlledBy (PlayerGroup YourOpponents)]))
                      (PtDown (Lit 1)) (PtDown (Lit 1))) ]
       (Just (3, 3))

anglerTurtle : Ability
anglerTurtle =
  Static (Deontic (AllOf Macros.creatureYourOpponentsControl) Require Attack Agent NoDeonticPatient)


bloodTyrant : Ability
bloodTyrant =
  Macros.triggered Whenever (LosesGame (Macros.a AnyPlayer))
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
       (MkTypeLine [creatureType "Cat", creatureType "Beast"] [Creature])
       [ Macros.keyword "Vigilance"
       , Macros.keyword "Lifelink"
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
       (MkTypeLine [creatureType "Angel"] [Creature])
       [ Macros.keyword "Flying"
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
                  [ If (CompareAmt (PlayerStatOf LifeTotal You) Less
                                   (PlayerStatOf LifeTotal Macros.anOpponent))
                       (Macros.gainsLife You (Lit 6))
                       Nothing
                  , If (CompareAmt (CountOf Macros.creatureYouControl) Less
                                   (CountOf (And [Macros.creature,
                                                  ControlledBy Macros.anOpponent])))
                       (Macros.create (Lit 3) (Macros.creatureTok 1 1 [White] [creatureType "Soldier"]))
                       Nothing ]) ]
       Nothing

survivalCache : Effect []
survivalCache =
  Sequentially [ Macros.gainsLife You (Lit 2)
               , If (CompareAmt (PlayerStatOf LifeTotal You) Greater
                                (PlayerStatOf LifeTotal Macros.anOpponent))
                    Macros.drawACard
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
       (MkTypeLine [creatureType "Angel"] [Creature])
       [ Macros.keyword "Flying"
       , Macros.keyword "Lifelink"
       , Macros.triggeredIf At
                            (BeginningOf Combat (ByWord EachPlayers))
                            (CompareAmt (PlayerStatOf LifeTotal You) Greater
                                        (PlayerStatOf LifeTotal Macros.anOpponent))
                            (Macros.gains Macros.thisCreature (Macros.keyword "DoubleStrike")
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
  Macros.activated TapSymbol
       (Sequentially
          [ Create You (LetterVal X)
                   (TokenWritten (Macros.creatureTok 1 1 [Red] [creatureType "Goblin"])) []
          , Define X (CountOf (And [HasSubtype (creatureType "Goblin"), ControlledBy You])) ])

chainReaction : Card
chainReaction =
  Macros.card "Chain Reaction"
       (Just [Macros.generic 2, Macros.pip Red, Macros.pip Red]) []
       (MkTypeLine [] [Sorcery])
       [ Spell (Sequentially
                  [ DealDamage This (LetterVal X) (Each Macros.creature)
                  , Define X (CountOf Macros.creature) ]) ]
       Nothing

ivoryTower : Card
ivoryTower =
  Macros.card "Ivory Tower" (Just [Macros.generic 1]) []
       (MkTypeLine [] [Artifact])
       [ Macros.triggered At (BeginningOf Upkeep (ByWord Yours))
                  (Sequentially
                     [ Macros.gainsLife You (LetterVal X)
                     , Define X (Minus (CountOf (InZone (Macros.handOf You)))
                                (Lit 4)) ]) ]
       Nothing

harshSustenance : Card
harshSustenance =
  Macros.card "Harsh Sustenance"
       (Just [Macros.generic 1, Macros.pip White, Macros.pip Black]) []
       (MkTypeLine [] [Sorcery])
       [ Spell (Sequentially
                  [ DealDamage This (LetterVal X) (Macros.target Macros.anyTarget)
                  , Macros.gainsLife You (LetterVal X)
                  , Define X (CountOf Macros.creatureYouControl) ]) ]
       Nothing


nightmarishEnd : Card
nightmarishEnd =
  Macros.card "Nightmarish End"
       (Just [Macros.generic 2, Macros.pip Black]) []
       (MkTypeLine [] [Instant])
       [ Spell (Sequentially
                  [ Macros.gets (Macros.target Macros.creature)
                                (PtDown (LetterVal X)) (PtDown (LetterVal X))
                                (Just Macros.untilEndOfTurn)
                  , Define X (CountOf (InZone (Macros.handOf You))) ]) ]
       Nothing

adelbertSteiner : Card
adelbertSteiner =
  Macros.card "Adelbert Steiner" (Just [Macros.generic 1, Macros.pip White])
       [Legendary] (MkTypeLine [creatureType "Human", creatureType "Knight"] [Creature])
       [ Macros.keyword "Lifelink"
       , Static (Gets Macros.thisCreature
               (PtUp (Macros.forEach (And [HasSubtype (artifactType "Equipment"), ControlledBy You])))
               (PtUp (Macros.forEach (And [HasSubtype (artifactType "Equipment"), ControlledBy You])))) ]
       (Just (2, 1))

||| Dokai, Weaver of Life
dokai : Ability
dokai =
  Macros.activated (Compound [Mana [Macros.generic 4, Macros.pip Green, Macros.pip Green],
                       TapSymbol])
       (Sequentially
          [ Macros.create (Lit 1)
              (Macros.creatureTokOf (LetterVal X) (LetterVal X)
                                    [Green] [creatureType "Elemental"])
          , Define X (CountOf (And [Macros.land, ControlledBy You])) ])


deathsShadow : Card
deathsShadow =
  Macros.card "Death's Shadow" (Just [Macros.pip Black]) []
       (MkTypeLine [creatureType "Avatar"] [Creature])
       [ Static (AndAlso [ Gets Macros.thisCreature
                                (PtDown (LetterVal X)) (PtDown (LetterVal X))
                         , Define X (PlayerStatOf LifeTotal You) ]) ]
       (Just (13, 13))

spontaneousMutation : Ability
spontaneousMutation =
  Static (AndAlso [ Gets (AttachHost Enchanted (TypeW Creature))
                         (PtDown (LetterVal X)) (PtDown (Lit 0))
                  , Define X (CountOf (InZone (Macros.graveyardOf You))) ])

stagBeetle : Card
stagBeetle =
  Macros.card "Stag Beetle"
       (Just [Macros.generic 3, Macros.pip Green, Macros.pip Green]) []
       (MkTypeLine [creatureType "Insect"] [Creature])
       [ Static (AndAlso [ Macros.entersWithCounters Macros.thisCreature
                                                     (LetterVal X)
                                                     Macros.plusOnePlusOne
                         , Define X
                             (CountOf (Macros.otherCreature Macros.thisCreature)) ]) ]
       (Just (0, 0))


acceleratedMutation : Card
acceleratedMutation =
  Macros.card "Accelerated Mutation"
       (Just [Macros.generic 3, Macros.pip Green, Macros.pip Green]) []
       (MkTypeLine [] [Instant])
       [ Spell (Sequentially
                  [ Continuously (Gets (Macros.target Macros.creature)
                                       (PtUp (LetterVal X)) (PtUp (LetterVal X)))
                                 (Just Macros.untilEndOfTurn)
                  , Define X (Aggregate MaxOf (CharAxis ManaValue)
                                 (And [Permanent, ControlledBy You])) ]) ]
       Nothing

carrionGrub : Card
carrionGrub =
  Macros.card "Carrion Grub" (Just [Macros.generic 3, Macros.pip Black]) []
       (MkTypeLine [creatureType "Insect"] [Creature])
       [ Static (AndAlso [ Gets Macros.thisCreature
                                (PtUp (LetterVal X)) (PtUp (Lit 0))
                         , Define X
                             (Aggregate MaxOf (CharAxis Power)
                                        (And [Macros.creature,
                                              InZone (Macros.graveyardOf You)])) ])
       , Macros.triggered When (Enters Macros.thisCreature)
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
  Static (AndAlso [ Macros.entersWithCounters Macros.thisCreature
                                              (LetterVal X)
                                              Macros.plusOnePlusOne
                  , Define X
                      (Aggregate SumOf (CharAxis Toughness)
                         (Macros.otherCreatureYouControl Macros.thisCreature)) ])


ghalta : Card
ghalta =
  Macros.card "Ghalta, Primal Hunger"
       (Just [Macros.generic 10, Macros.pip Green, Macros.pip Green]) [Legendary]
       (MkTypeLine [creatureType "Elder", creatureType "Dinosaur"] [Creature])
       [ Static (AndAlso [ CostsToCast This (CostLess (LetterVal X))
                         , Define X
                             (Aggregate SumOf (CharAxis Power)
                                        Macros.creatureYouControl) ])
       , Macros.keyword "Trample" ]
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
       (MkTypeLine [creatureType "Construct"] [Artifact, Creature])
       [ Static (CostsToCast (AllOf (And [Macros.artifact, Macros.spell, CastBy You]))
                             (CostLess (Lit 1))) ]
       (Just (3, 2))

daruWarchief : Card
daruWarchief =
  Macros.card "Daru Warchief"
       (Just [Macros.generic 2, Macros.pip White, Macros.pip White]) []
       (MkTypeLine [creatureType "Human", creatureType "Soldier"] [Creature])
       [ Static (CostsToCast (AllOf (And [HasSubtype (creatureType "Soldier"), Macros.spell, CastBy You]))
                             (CostLess (Lit 1)))
       , Static (Gets (AllOf (And [HasSubtype (creatureType "Soldier"), Macros.creature, ControlledBy You]))
                      (PtUp (Lit 1)) (PtUp (Lit 2))) ]
       (Just (1, 1))

grandArbiter : Card
grandArbiter =
  Macros.card "Grand Arbiter Augustin IV"
       (Just [Macros.generic 2, Macros.pip White, Macros.pip Blue]) [Legendary]
       (MkTypeLine [creatureType "Human", creatureType "Advisor"] [Creature])
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
       (MkTypeLine [creatureType "Goblin", creatureType "Wizard"] [Creature])
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
       , Macros.activated (Do (Macros.sacrifice You Macros.thisEnchantment))
                          (Macros.destroy (Macros.target (Or [Macros.artifact,
                                                       Macros.enchantment]))) ]
       Nothing

youngPyromancer : Card
youngPyromancer =
  Macros.card "Young Pyromancer" (Just [Macros.generic 1, Macros.pip Red]) []
       (MkTypeLine [creatureType "Human", creatureType "Shaman"] [Creature])
       [ Macros.triggered Whenever
                          (Casts You (Macros.a (And [Macros.instantOrSorcery, Macros.spell])))
                          (Create You (Lit 1)
                            (TokenWritten (Macros.creatureTok 1 1 [Red] [creatureType "Elemental"])) []) ]
       (Just (2, 1))


inspiringStatuary : Card
inspiringStatuary =
  Macros.card "Inspiring Statuary" (Just [Macros.generic 3]) []
       (MkTypeLine [] [Artifact])
       [ Static (Gains (AllOf (And [Not Macros.artifact, Macros.spell, CastBy You]))
                       (Macros.keyword "Improvise")) ]
       Nothing

chiefEngineer : Card
chiefEngineer =
  Macros.card "Chief Engineer" (Just [Macros.generic 1, Macros.pip Blue]) []
       (MkTypeLine [creatureType "Vedalken", creatureType "Artificer"] [Creature])
       [ Static (Gains (AllOf (And [Macros.artifact, Macros.spell, CastBy You]))
                       (Macros.keyword "Convoke")) ]
       (Just (1, 3))

firesongAndSunspeaker : Ability
firesongAndSunspeaker =
  Static (Gains (AllOf (And [ColorIs Red, Macros.instantOrSorcery, Macros.spell,
                             ControlledBy You]))
                (Macros.keyword "Lifelink"))

prismariTheInspiration : Card
prismariTheInspiration =
  Macros.card "Prismari, the Inspiration"
       (Just [Macros.generic 5, Macros.pip Blue, Macros.pip Red]) [Legendary]
       (MkTypeLine [creatureType "Elder", creatureType "Dragon"] [Creature])
       [ Macros.keyword "Flying"
       , Macros.keywordCosting "Ward" (Macros.payLife You 5)
       , Static (Gains (AllOf (And [Macros.instantOrSorcery, Macros.spell, CastBy You]))
                       (Macros.keyword "Storm")) ]
       (Just (7, 7))


tomakulHonorGuard : Card
tomakulHonorGuard =
  Macros.card "Tomakul Honor Guard" (Just [Macros.generic 1, Macros.pip Green]) []
       (MkTypeLine [creatureType "Human", creatureType "Soldier"] [Creature])
       [ Macros.keywordCosting "Ward" (Mana [Macros.generic 2]) ]
       (Just (3, 1))

repentantBlacksmith : Card
repentantBlacksmith =
  Macros.card "Repentant Blacksmith" (Just [Macros.generic 1, Macros.pip White]) []
       (MkTypeLine [creatureType "Human"] [Creature])
       [ Macros.keywordQuality "Protection" (ColorIs Red) ]
       (Just (1, 2))


maro : Card
maro =
  Macros.cardOf "Maro" (Just [Macros.generic 2, Macros.pip Green, Macros.pip Green]) []
       (MkTypeLine [creatureType "Elemental"] [Creature])
       [ Static (DefinesPt Macros.thisCreature BothEach
                           (CountOf (InZone (Macros.handOf You)))) ]
       (Just (PtBox PrintedStar PrintedStar))

battleSquadron : Card
battleSquadron =
  Macros.cardOf "Battle Squadron" (Just [Macros.generic 3, Macros.pip Red, Macros.pip Red]) []
       (MkTypeLine [creatureType "Goblin"] [Creature])
       [ Macros.keyword "Flying"
       , Static (DefinesPt Macros.thisCreature BothEach
                           (CountOf Macros.creatureYouControl)) ]
       (Just (PtBox PrintedStar PrintedStar))

peopleOfTheWoods : Card
peopleOfTheWoods =
  Macros.cardOf "People of the Woods" (Just [Macros.pip Green, Macros.pip Green]) []
       (MkTypeLine [creatureType "Human"] [Creature])
       [ Static (DefinesPt Macros.thisCreature ToughnessAlone
                           (CountOf (And [HasSubtype (landType "Forest"), ControlledBy You]))) ]
       (Just (PtBox (PrintedNum 1) PrintedStar))

scourgeOfTheSkyclaves : Ability
scourgeOfTheSkyclaves =
  Static (DefinesPt Macros.thisCreature BothEach
                    (Minus (Lit 20) (Aggregate MaxOf (PlayerStatAxis LifeTotal) AnyPlayer)))

aettirAndPriwen : Ability
aettirAndPriwen =
  Static (AndAlso [ HasBasePt (AttachHost Equipped (TypeW Creature))
                              (LetterVal X) (LetterVal X)
                  , Define X (PlayerStatOf LifeTotal You) ])

diminish : Card
diminish =
  Macros.card "Diminish" (Just [Macros.pip Blue]) []
       (MkTypeLine [] [Instant])
       [ Spell (Continuously (HasBasePt (Macros.target Macros.creature) (Lit 1) (Lit 1))
                             (Just Macros.untilEndOfTurn)) ]
       Nothing

||| Cycle of Life
||| "Target creature you cast this turn has base power and toughness 0/1
||| until your next upkeep." — the subject is a battlefield permanent, so
||| `CastBy` reads as history rather than as a stack seed.
cycleOfLife : Effect []
cycleOfLife =
  Continuously (HasBasePt (Macros.target (And [Macros.creature, CastBy You]))
                          (Lit 0) (Lit 1))
               (Just Macros.untilYourNextUpkeep)

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
       [ Macros.triggered At (BeginningOf Upkeep (ByWord Yours))
                          (Macros.destroy
                      (Macros.target
                         (And [Permanent, Not Macros.land,
                               Superlative MinOf (CharAxis ManaValue)
                                           (And [Permanent, Not Macros.land])]))) ]
       Nothing

purgingScythe : Ability
purgingScythe =
  Macros.triggered At (BeginningOf Upkeep (ByWord Yours))
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
       (MkTypeLine [creatureType "Human", creatureType "Knight"] [Creature])
       [ Macros.keyword "Haste"
       , Macros.triggeredIf Whenever
                            (Macros.attacks Macros.thisCreature)
                            (Exists (And [Macros.creature, ControlledBy You,
                                          Superlative MaxOf (CharAxis Power)
                                            (And [Macros.creature,
                                                  InZone Macros.battlefieldZ])]))
                            (Macros.create (Lit 1) (Macros.creatureTok 1 1 [White] [creatureType "Human", creatureType "Soldier"])) ]
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
       (MkTypeLine [creatureType "Myr"] [Artifact, Creature])
       [ Macros.activated TapSymbol
                          (PutCounters (Lit 1) Charge (Macros.target Macros.artifact))
       , Macros.activated (Do (Macros.sacrifice You Macros.thisCreature))
                          (PutCounters (Lit 2) Charge (Macros.target Macros.artifact)) ]
       (Just (1, 1))

divineIntervention : Card
divineIntervention =
  Macros.card "Divine Intervention"
       (Just [Macros.generic 6, Macros.pip White, Macros.pip White]) []
       (MkTypeLine [] [Enchantment])
       [ Static (Macros.entersWithCounters Macros.thisEnchantment (Lit 2) Intervention)
       , Macros.triggered At (BeginningOf Upkeep (ByWord Yours))
                          (RemoveCounters (Lit 1) (Just Intervention) Macros.thisEnchantment)
       , Macros.triggered When (Macros.lastCounterRemovedBy Intervention Macros.thisEnchantment You)
                          GameDrawn ]
       Nothing

celestialConvergence : Card
celestialConvergence =
  Macros.card "Celestial Convergence"
       (Just [Macros.generic 2, Macros.pip White, Macros.pip White]) []
       (MkTypeLine [] [Enchantment])
       [ Static (Macros.entersWithCounters Macros.thisEnchantment (Lit 7) Omen)
       , Macros.triggered At (BeginningOf Upkeep (ByWord Yours))
           (Sequentially
              [ RemoveCounters (Lit 1) (Just Omen) Macros.thisEnchantment
              , If (CompareAmt (CountersOn Omen Macros.thisEnchantment)
                               AtMost (Lit 0))
                          (Concludes WinGame
                      (Definite (And [AnyPlayer,
                                      Superlative MaxOf (PlayerStatAxis LifeTotal)
                                                  AnyPlayer])))
                          Nothing
              , If (CompareAmt (CountOf (And [AnyPlayer,
                                              Superlative MaxOf
                                                (PlayerStatAxis LifeTotal)
                                                AnyPlayer]))
                               AtLeast (Lit 2))
                          GameDrawn
                          Nothing ]) ]
       Nothing


angelicGift : Card
angelicGift =
  Macros.card "Angelic Gift" (Just [Macros.generic 1, Macros.pip White]) []
       (MkTypeLine [enchantmentType "Aura"] [Enchantment])
       [ Macros.keywordSubject "Enchant" Macros.creature
       , Macros.triggered When (Enters Macros.thisAura) (Draw You (Lit 1))
       , Static (Gains (AttachHost Enchanted (TypeW Creature))
                       (Macros.keyword "Flying")) ]
       Nothing

lionHeart : Card
lionHeart =
  Macros.card "Lion Heart" (Just [Macros.generic 4]) []
       (MkTypeLine [artifactType "Equipment"] [Artifact])
       [ Macros.triggered When (Enters Macros.thisEquipment)
                          (DealDamage It (Lit 2) (Macros.target Macros.anyTarget))
       , Static (Gets (AttachHost Equipped (TypeW Creature))
                      (PtUp (Lit 2)) (PtUp (Lit 1)))
       , Macros.keywordCosting "Equip" (Mana [Macros.generic 2]) ]
       Nothing

curseOfVengeance : Card
curseOfVengeance =
  Macros.card "Curse of Vengeance" (Just [Macros.pip Black]) []
       (MkTypeLine [enchantmentType "Aura", enchantmentType "Curse"] [Enchantment])
       [ Macros.keywordSubject "Enchant" AnyPlayer
       , Macros.triggered Whenever
                          (Casts (AttachHost Enchanted PlayerW) (Macros.a Macros.spell))
                          (PutCounters (Lit 1) Spite Macros.thisAura)
       , Macros.triggered When (LosesGame (AttachHost Enchanted PlayerW))
                          (Sequentially
                             [ ChangeLife You (Up (LetterVal X))
                             , Draw You (LetterVal X)
                             , Define X (CountersOn Spite Macros.thisAura) ]) ]
       Nothing


consulsLieutenant : Card
consulsLieutenant =
  Macros.card "Consul's Lieutenant" (Just [Macros.pip White, Macros.pip White]) []
       (MkTypeLine [creatureType "Human", creatureType "Soldier"] [Creature])
       [ Macros.keyword "FirstStrike"
       , Macros.keywordNumber "Renown" (Lit 1)
       , Macros.triggeredIf Whenever
                            (Macros.attacks Macros.thisCreature)
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
       [ Macros.keyword "Ascend"
       , Spell (InsteadOf (Draw You (Lit 2))
                          (If (Matches You (HasDesignation CitysBlessing))
                              (Draw You (Lit 3))
                              Nothing)) ]
       Nothing

bombur : Card
bombur =
  Macros.card "Bombur, Gentle Dreamer"
       (Just [Macros.generic 2, Macros.pip Red]) [Legendary]
       (MkTypeLine [creatureType "Dwarf", creatureType "Bard"] [Creature])
       [ Macros.keyword "Storied"
       , Static (Macros.unlessSo (Matches You (HasDesignation EnduringStory))
                                 (DoesntUntap Macros.thisCreature)) ]
       (Just (5, 3))


passagewaySeer : Card
passagewaySeer =
  Macros.card "Passageway Seer" (Just [Macros.generic 3, Macros.pip Black]) []
       (MkTypeLine [creatureType "Tiefling", creatureType "Warlock"] [Creature])
       [ Macros.keyword "Lifelink"
       , Macros.triggered When (Enters Macros.thisCreature)
                          (Macros.gainsDesignation You TheInitiative Instructed)
       , Macros.triggeredIf At
                            (BeginningOf EndStep (ByWord Yours))
                            (Matches You (HasDesignation TheInitiative))
                            (PutCounters (Lit 1) Macros.plusOnePlusOne Macros.thisCreature) ]
       (Just (2, 2))

deadeyeBrawler : Card
deadeyeBrawler =
  Macros.card "Deadeye Brawler"
       (Just [Macros.generic 2, Macros.pip Blue, Macros.pip Black]) []
       (MkTypeLine [creatureType "Human", creatureType "Pirate"] [Creature])
       [ Macros.keyword "Deathtouch"
       , Macros.keyword "Ascend"
       , Macros.triggeredIf Whenever
                            (DealsCombatDamage Macros.thisCreature (Macros.a AnyPlayer))
                            (Matches You (HasDesignation CitysBlessing))
                            Macros.drawACard ]
       (Just (2, 4))

chillerpillar : Card
chillerpillar =
  Macros.card "Chillerpillar" (Just [Macros.generic 3, Macros.pip Blue]) [Snow]
       (MkTypeLine [creatureType "Insect"] [Creature])
       [ Macros.activated (Mana [Macros.generic 4, SnowMana, SnowMana])
                          (Macros.monstrosity (Lit 2))
       , Static (Macros.asLongAs (Matches Macros.thisCreature (HasDesignation Monstrous))
                                 (Gains It (Macros.keyword "Flying"))) ]
       (Just (3, 3))

archivistOfGondor : Ability
archivistOfGondor =
  Macros.triggeredIf When
                     (DealsCombatDamage Macros.yourCommander (Macros.a AnyPlayer))
                     (Macros.thereIsNo Monarch)
                     (Macros.gainsDesignation You Monarch Instructed)


jushiApprentice : Ability
jushiApprentice =
  Macros.activated (Compound [Mana [Macros.generic 2, Macros.pip Blue], TapSymbol])
                   (Sequentially
               [ Macros.drawACard
               , If (CompareAmt (CountOf (InZone (Macros.handOf You)))
                                AtLeast (Lit 9))
                    (SetStatus Flipped Macros.thisCreature)
                    Nothing ])


chainsaw : Card
chainsaw =
  Macros.card "Chainsaw" (Just [Macros.generic 3]) []
       (MkTypeLine [artifactType "Equipment"] [Artifact])
       [ Macros.triggered When (Enters Macros.thisEquipment)
                          (DealDamage It (Lit 3)
                               (TargetGroup (Macros.upTo 1) Macros.creature))
       , Macros.triggered Whenever
                          (Dies (CountedGroup (Macros.atLeast 1) Macros.creature))
                          (PutCounters (Lit 1) Rev Macros.thisEquipment)
       , Static (AndAlso [ Gets (AttachHost Equipped (TypeW Creature))
                                (PtUp (LetterVal X)) (PtUp (Lit 0))
                         , Define X (CountersOn Rev Macros.thisEquipment) ])
       , Macros.keywordCosting "Equip" (Mana [Macros.generic 3]) ]
       Nothing

||| Phyrexian Ingester's second ability: "This creature gets +X/+Y, where
||| X is the exiled creature card's power and Y is its toughness." Two
||| letters in one statement, each with its own definition [CR#107.3p];
||| the "its" is written `That CardW`, the exiled card the first
||| definition named.
phyrexianIngesterPump : Ability
phyrexianIngesterPump =
  Static (AndAlso [ Gets Macros.thisCreature (PtUp (LetterVal X)) (PtUp (LetterVal Y))
                  , Define X
                      (StatOf Power
                         (Macros.a (And [Macros.creature,
                                         ExiledWith Macros.thisCreature])))
                  , Define Y (StatOf Toughness (That CardW)) ])

||| Soul's Might: "Put X +1/+1 counters on target creature, where X is
||| that creature's power." The definition reads the mention its own
||| clause introduced, which is why it must be written after it.
soulsMight : Card
soulsMight =
  Macros.card "Soul's Might" (Just [Macros.generic 4, Macros.pip Green]) []
       (MkTypeLine [] [Sorcery])
       [ Spell (Sequentially
                  [ PutCounters (LetterVal X) Macros.plusOnePlusOne
                                (Macros.target Macros.creature)
                  , Define X (StatOf Power (That (TypeW Creature))) ]) ]
       Nothing


lightmineField : Card
lightmineField =
  Macros.card "Lightmine Field"
       (Just [Macros.generic 2, Macros.pip White, Macros.pip White]) []
       (MkTypeLine [] [Enchantment])
       [ Macros.triggered Whenever
                          (Macros.attacks (CountedGroup (Macros.atLeast 1) Macros.creature))
                          (DealDamage Macros.thisEnchantment
                               (CountOf (And [Attacking, Macros.creature]))
                               (EachOf (Those (TypeW Creature)))) ]
       Nothing

misterFantastic : Card
misterFantastic =
  Macros.card "Mister Fantastic, Reed Richards"
       (Just [Macros.generic 1, Macros.pip White, Macros.pip Blue]) [Legendary]
       (MkTypeLine [creatureType "Human", creatureType "Hero"] [Creature])
       [ Macros.keyword "Reach"
       , Macros.triggered Whenever
                          (Enters (CountedGroup (Macros.atLeast 1)
                                         (And [IsToken, ControlledBy You])))
                          (May Nothing Macros.drawACard Nothing Nothing) ]
       (Just (2, 4))


woodlandChampion : Card
woodlandChampion =
  Macros.card "Woodland Champion" (Just [Macros.generic 1, Macros.pip Green]) []
       (MkTypeLine [creatureType "Elf", creatureType "Scout"] [Creature])
       [ Macros.triggered Whenever
                          (Enters (CountedGroup (Macros.atLeast 1)
                                         (And [IsToken, ControlledBy You])))
                          (PutCounters GroupSize Macros.plusOnePlusOne
                                Macros.thisCreature) ]
       (Just (2, 2))

ingeniousArtillerist : Card
ingeniousArtillerist =
  Macros.card "Ingenious Artillerist"
       (Just [Macros.generic 2, Macros.pip Red]) []
       (MkTypeLine [creatureType "Human", creatureType "Artificer"] [Creature])
       [ Macros.triggered Whenever
                          (Enters (CountedGroup (Macros.atLeast 1)
                                         (And [Macros.artifact, ControlledBy You])))
                          (DealDamage Macros.thisCreature GroupSize (Each Opponent)) ]
       (Just (3, 1))

hissingMiasma : Card
hissingMiasma =
  Macros.card "Hissing Miasma"
       (Just [Macros.generic 1, Macros.pip Black, Macros.pip Black]) []
       (MkTypeLine [] [Enchantment])
       [ Macros.triggered Whenever
                          (Macros.attacksPlayer (Macros.a Macros.creature) You)
                          (Macros.losesLife (ControllerOf It) (Lit 1)) ]
       Nothing

orimsPrayer : Card
orimsPrayer =
  Macros.card "Orim's Prayer"
       (Just [Macros.generic 1, Macros.pip White, Macros.pip White]) []
       (MkTypeLine [] [Enchantment])
       [ Macros.triggered Whenever
                          (Macros.attacksPlayer (CountedGroup (Macros.atLeast 1) Macros.creature) You)
                          (Macros.gainsLife You
                      (Macros.forEach (And [Attacking, Macros.creature]))) ]
       Nothing


undercoverButler : Card
undercoverButler =
  Macros.card "Undercover Butler"
       (Just [Macros.generic 2, Macros.hybridPip Blue Black]) []
       (MkTypeLine [creatureType "Human", creatureType "Rogue"] [Creature])
       [ Macros.triggered Whenever
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
       (MkTypeLine [spellType "Arcane"] [Instant])
       [ Spell (Sequentially
                  [ Macros.revealCards (Macros.topCards 3)
                  , Macros.chooses (Macros.a Opponent) (Macros.oneOf Them)
                  , Macros.move (That CardW) Macros.graveyardZ
                  , Macros.move TheRest Macros.handZ ]) ]
       Nothing


femerefEnchantress : Card
femerefEnchantress =
  Macros.card "Femeref Enchantress"
       (Just [Macros.pip Green, Macros.pip White]) []
       (MkTypeLine [creatureType "Human", creatureType "Druid"] [Creature])
       [ Macros.triggered Whenever
                          (Macros.putIntoFrom (Macros.a Macros.enchantment)
                                       Macros.graveyardZ
                                       (FromZone Macros.battlefieldZ))
                          Macros.drawACard ]
       (Just (1, 2))

voraciousBrood : Card
voraciousBrood =
  Macros.card "Voracious Brood"
       (Just [Macros.generic 2, Macros.pip Green]) []
       (MkTypeLine [creatureType "Alien", creatureType "Insect"] [Creature])
       [ Static (Macros.entersWithCounters Macros.thisCreature
                   (Macros.forEach (And [Macros.creature,
                                         InZone (Macros.graveyardOf You)]))
                   Macros.plusOnePlusOne)
       , Macros.triggered Whenever
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
       (MkTypeLine [creatureType "Beast"] [Creature])
       [ Macros.triggered Whenever
                          (Macros.manyCounterEvent CounterPut Macros.plusOnePlusOne
                                            Macros.thisCreature)
                          (Macros.may You
                      (Macros.create (Lit 1)
                                     (Macros.creatureTok 4 4 [Green] [creatureType "Beast"]))) ]
       (Just (4, 4))

flourishingDefenses : Card
flourishingDefenses =
  Macros.card "Flourishing Defenses"
       (Just [Macros.generic 4, Macros.pip Green]) []
       (MkTypeLine [] [Enchantment])
       [ Macros.triggered Whenever
                          (Macros.singleCounterEvent CounterPut
                                              Macros.minusOneMinusOne
                                              (Macros.a Macros.creature))
                          (Macros.may You
                      (Macros.create (Lit 1)
                                     (Macros.creatureTok 1 1 [Green]
                                                         [creatureType "Elf", creatureType "Warrior"]))) ]
       Nothing

rakshasaVizier : Card
rakshasaVizier =
  Macros.card "Rakshasa Vizier"
       (Just [Macros.generic 2, Macros.pip Black, Macros.pip Green,
              Macros.pip Blue]) []
       (MkTypeLine [creatureType "Demon"] [Creature])
       [ Macros.triggered Whenever
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
       (MkTypeLine [creatureType "Vampire", creatureType "Soldier"] [Creature])
       [ Macros.keyword "Vigilance"
       , Macros.triggeredOnlyOnce Whenever
                                  (Macros.manyCounterEvent CounterPut Macros.plusOnePlusOne
                                                           Macros.thisCreature)
                                  OncePerTurn
                                  Macros.drawACard ]
       (Just (2, 2))

foeLiage : Card
foeLiage =
  Macros.card "Foe-liage"
       (Just [Macros.generic 3, Macros.pip Green]) []
       (MkTypeLine [creatureType "Plant", creatureType "Mutant"] [Creature])
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
       (MkTypeLine [creatureType "Beast"] [Creature])
       [ Macros.triggeredOr Whenever
                            (Macros.attacks Macros.thisCreature)
                            (Blocks Macros.thisCreature Nothing)
                            (Macros.sacrifice You (Macros.a Macros.land)) ]
       (Just (6, 4))

colossalGraveReaver : Card
colossalGraveReaver =
  Macros.card "Colossal Grave-Reaver"
       (Just [Macros.generic 6, Macros.pip Black, Macros.pip Green]) []
       (MkTypeLine [creatureType "Dragon"] [Creature])
       [ Macros.keyword "Flying"
       , Macros.triggeredOr Whenever
                            (Enters Macros.thisCreature)
                            (Macros.attacks Macros.thisCreature)
                            (Macros.mills You (Lit 3) You)
       , Macros.triggered Whenever
                          (Macros.putIntoFrom (CountedGroup (Macros.atLeast 1)
                                                     (And [Macros.creature,
                                                           InZone Macros.yourLibrary]))
                                       (Macros.graveyardOf You)
                                       (FromZone Macros.yourLibrary))
                          (Macros.putOntoBattlefield (Macros.oneOf Them)) ]
       (Just (7, 6))

||| Chub Toad — the coordinated header whose two arms announce the same
||| thing. Both bare-partner arms announce the self (`selfSubjIntro`), so
||| `headerCtx` hands that common announcement on and the tail's "it"
||| finds its referent.
chubToad : Card
chubToad =
  Macros.card "Chub Toad"
       (Just [Macros.generic 2, Macros.pip Green]) []
       (MkTypeLine [creatureType "Frog"] [Creature])
       [ Macros.triggeredOr Whenever
                            (Blocks Macros.thisCreature Nothing)
                            (BecomesBlocked Macros.thisCreature Nothing)
                            (Macros.gets It (PtUp (Lit 2)) (PtUp (Lit 2))
                                         (Just Macros.untilEndOfTurn)) ]
       (Just (1, 1))

||| Inferno Elemental — the coordinated header's common announcement in
||| its partner-phrase form. Both arms announce the one creature the
||| block pairs this creature with, so `headerCtx` hands that common
||| announcement on and the tail's "that creature" reads it back. The
||| whole card: its printed text is this trigger and nothing else.
infernoElemental : Card
infernoElemental =
  Macros.card "Inferno Elemental"
       (Just [Macros.generic 4, Macros.pip Red, Macros.pip Red]) []
       (MkTypeLine [creatureType "Elemental"] [Creature])
       [ Macros.triggeredOr Whenever
                            (Blocks Macros.thisCreature (Just (Macros.a Macros.creature)))
                            (BecomesBlocked Macros.thisCreature
                                            (Just (Macros.a Macros.creature)))
                            (DealDamage Macros.thisCreature (Lit 3)
                                        (That (TypeW Creature))) ]
       (Just (4, 4))

hardenedScales : Card
hardenedScales =
  Macros.card "Hardened Scales" (Just [Macros.pip Green]) []
       (MkTypeLine [] [Enchantment])
       [ Static (Intercepts
                   (Macros.manyCounterEvent CounterPut Macros.plusOnePlusOne
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
                   (Macros.manyCounterEvent CounterPut Macros.plusOnePlusOne
                                            (Macros.a Macros.creatureYouControl))
                   (PutCounters (Times 2 ThatMuch)
                                Macros.plusOnePlusOne (That (TypeW Creature)))
                   Repeatedly) ]
       Nothing

||| Corpsejack Menace — "If one or more +1/+1 counters would be put on a
||| creature you control, twice that many +1/+1 counters are put on it
||| instead." The family's second named whole; the card itself waits on a
||| `Fungus` creature-subtype row, so the ability alone is benched.
corpsejackMenace : Ability
corpsejackMenace =
  Static (Intercepts
            (Macros.manyCounterEvent CounterPut Macros.plusOnePlusOne
                                     (Macros.a Macros.creatureYouControl))
            (PutCounters (Times 2 ThatMuch) Macros.plusOnePlusOne It)
            Repeatedly)

||| Doubling Season, whole: "If an effect would create one or more tokens
||| under your control, it creates twice that many of those tokens
||| instead." and "If an effect would put one or more counters on a
||| permanent you control, it puts twice that many of those counters on
||| that permanent instead."
doublingSeason : Card
doublingSeason =
  Macros.card "Doubling Season" (Just [Macros.generic 4, Macros.pip Green]) []
       (MkTypeLine [] [Enchantment])
       [ Static (Intercepts
                   (Macros.tokensCreatedByEffectUnder
                      (CountedGroup (Macros.atLeast 1) IsToken) You)
                   (Create You (Times 2 GroupSize) TokenAsThose [])
                   Repeatedly)
       , Static (Intercepts
                   (Macros.manyCountersPutByEffect
                      (Macros.a (And [Permanent, ControlledBy You])))
                   (PutCountersOfThoseKinds (Times 2 ThatMuch) (That PermanentW))
                   Repeatedly) ]
       Nothing

||| Doc Samson, Super Psychiatrist — "If you would put one or more counters
||| on a permanent you control, put that many plus one of each of those
||| kinds of counters on that permanent instead": the per-kind spelling
||| with its agent voiced. Pir, Imaginative Rascal writes the same clause
||| over "a permanent your team controls" and waits on the team form
||| [CR#102.3] alone.
docSamsonDistributive : Ability
docSamsonDistributive =
  Static (Intercepts
            (Macros.manyBareCountersPutBy You
               (Macros.a (And [Permanent, ControlledBy You])))
            (PutCountersOfThoseKinds (Plus ThatMuch (Lit 1)) (That PermanentW))
            Repeatedly)

||| Winding Constrictor, whole: the object-side distributive over a
||| disjoined holder, and its player-side twin — "If you would get one or
||| more counters, you get that many plus one of each of those kinds of
||| counters instead." [CR#122.1]'s second holder, written with the
||| player's own verb.
windingConstrictor : Card
windingConstrictor =
  Macros.card "Winding Constrictor" (Just [Macros.pip Black, Macros.pip Green]) []
       (MkTypeLine [creatureType "Snake"] [Creature])
       [ Static (Intercepts
                   (Macros.manyBareCounterEvent CounterPut
                      (Macros.a (And [Or [Macros.artifact, Macros.creature],
                                      ControlledBy You])))
                   (PutCountersOfThoseKinds (Plus ThatMuch (Lit 1))
                                            (That PermanentW))
                   Repeatedly)
       , Static (Intercepts
                   (Macros.manyBareCounterEvent CounterPut You)
                   (GetsCountersOfThoseKinds You (Plus ThatMuch (Lit 1)))
                   Repeatedly) ]
       (Just (2, 3))

||| Aragorn, Company Leader (second line) — "Whenever you put one or more
||| counters on Aragorn, put one of each of those kinds of counters on up
||| to one other target creature": the distributive kind anaphor in a
||| trigger body rather than under an interception -- the header's own
||| batch is what it ranges over either way. The card waits on the
||| Ring-tempts header its first line writes.
aragornDistributive : Ability
aragornDistributive =
  Macros.triggered Whenever
    (Macros.manyBareCountersPutBy You Macros.thisCreature)
    (PutCountersOfThoseKinds (Lit 1)
       (TargetGroup (Macros.upTo 1)
          (Macros.otherCreature Macros.thisCreature)))

||| Captain Marvel, Apex Avenger (trigger) — "Whenever you put one or more
||| counters on another creature, … you may put the same number and kind
||| of counters on Captain Marvel": "the same number and kind" is the
||| distributive anaphor at the batch's own size. The card's intervening
||| "if it's not a Kree" has no row. Bold Plagiarist writes the same body
||| over an opponent's put.
captainMarvelSameKinds : Ability
captainMarvelSameKinds =
  Macros.triggered Whenever
    (Macros.manyBareCountersPutBy You
       (Macros.a (Macros.otherCreature Macros.thisCreature)))
    (Macros.may You (PutCountersOfThoseKinds ThatMuch Macros.thisCreature))

||| Denry Klin, Editor in Chief (trigger) — "Whenever a nontoken creature
||| you control enters, if Denry Klin has counters on it, put the same
||| number of each kind of counter on that creature": the copy-read whose
||| source the effect clause leaves unwritten. [CR#201.5] fixes a
||| self-name to the object the text is on, so the source is a referent
||| the spelling elides rather than an anaphor over the intervening "if",
||| whose own job [CR#603.4] gives it: a condition checked at trigger and
||| again at resolution. The card's entry line, "your choice of a +1/+1, first
||| strike, or vigilance counter", has no row: no entry mark chooses its
||| kind.
denryKlinSameKinds : Ability
denryKlinSameKinds =
  Macros.triggeredIf Whenever
    (Enters (Macros.a (And [Macros.nontoken, Macros.creatureYouControl])))
    (Matches Macros.thisCreature (HasCounters Nothing))
    (PutSameCounters Macros.thisCreature (That (TypeW Creature)))

||| Star Pupil — "When this creature dies, put its counters on target
||| creature you control": the same row with its source written out, and
||| the family's dominant spelling. `It` names a creature that has left
||| the battlefield, the case [CR#122.8] is written for.
starPupil : Card
starPupil =
  Macros.card "Star Pupil" (Just [Macros.pip White]) []
       (MkTypeLine [creatureType "Human", creatureType "Wizard"] [Creature])
       [ Static (Macros.entersWithCounters Macros.thisCreature (Lit 1)
                                           Macros.plusOnePlusOne)
       , Macros.triggered When (Dies Macros.thisCreature)
                          (PutSameCounters It
                             (Macros.target Macros.creatureYouControl)) ]
       (Just (0, 0))

||| Stalwart Successor's header — "Whenever one or more counters are put on
||| a creature you control": the kind-blind batch on the trigger side,
||| benched at the event level; the
||| intervening-if clause the line goes on to write has no row.
stalwartSuccessorHeader : GameEvent []
stalwartSuccessorHeader =
  Macros.manyBareCounterEvent CounterPut (Macros.a Macros.creatureYouControl)

||| Runadi, Behemoth Caller — "Creatures you control with three or more
||| +1/+1 counters on them have haste": the Object-scope bound read.
runadiBehemothCaller : Ability
runadiBehemothCaller =
  Static (Gains (AllOf (And [Macros.creature, ControlledBy You,
                             CounterCompare (Just Macros.plusOnePlusOne)
                                            AtLeast (Lit 3)]))
                (Macros.keyword "Haste"))

||| The corrupted keyword's reading — "each opponent who has three or more
||| poison counters" (Feed the Infection, Geth's Summons, Ixhel, Phyrexian
||| Atlas; Glissa's Retriever and Wurmquake write the same bound as
||| "opponents who have" and "opponent with"): the player-scope bound read
||| at the kind index [CR#122.1f].
corruptedOpponents : Noun [] Player
corruptedOpponents =
  Each (And [Opponent, CounterCompare (Just Poison) AtLeast (Lit 3)])

||| Boon of Safety — "Put a shield counter on target creature." The line
||| that pays for the `Shield` row.
boonOfSafetyPut : Effect []
boonOfSafetyPut = PutCounters (Lit 1) Shield (Macros.target Macros.creature)

||| Vivien's Talent and Teferi's Talent — "put a loyalty counter on
||| enchanted planeswalker": the loyalty kind's one-shot put, both cards
||| writing the phrase identically.
talentLoyaltyPut : Effect []
talentLoyaltyPut =
  PutCounters (Lit 1) LoyaltyCounter
              (AttachHost Enchanted (TypeW Planeswalker))

||| Simic Fluxmage — "Move a +1/+1 counter from this creature onto target
||| creature": the transfer verb with its kind named [CR#122.5].
simicFluxmageMove : Effect []
simicFluxmageMove =
  MoveCounters (Lit 1) (Just Macros.plusOnePlusOne) Macros.thisCreature
               (Macros.target Macros.creature)

||| Rikku, Resourceful Guardian — "Move a counter from target creature an
||| opponent controls onto target creature you control": the same verb
||| kind-blind.
rikkuStealMove : Effect []
rikkuStealMove =
  MoveCounters (Lit 1) Nothing
               (Macros.target (And [Macros.creature,
                                    ControlledBy Macros.anOpponent]))
               (Macros.target Macros.creatureYouControl)

||| Littjara Mirrorlake — "Create a token that's a copy of target creature
||| you control, except it enters with an additional +1/+1 counter on it":
||| the entry-counter clause on the copy's own carrier.
littjaraMirrorlakeCopy : Effect []
littjaraMirrorlakeCopy =
  Create You (Lit 1)
         (TokenCopyOf (Macros.target (And [Macros.creature, ControlledBy You]))
                      [ExceptEntersWithCounters (Lit 1) Macros.plusOnePlusOne
                                                Additional])
         []

||| Master Chef's granted quotation — "This creature enters with an
||| additional +1/+1 counter on it", the entry-counter clause carried by a
||| grant rather than printed on the entering object.
masterChefGrantedAbility : Ability
masterChefGrantedAbility =
  Static (Macros.entersWithAdditionalCounters Macros.thisCreature (Lit 1)
                                              Macros.plusOnePlusOne)

||| …and the grant carrier takes it over a described class, which is what
||| `GrantSubject` has to agree to. The subject is a stand-in: Master
||| Chef's own "Commander creatures you own" has no row, so the card stays
||| ledgered while the shape it needs is shown to compose.
grantedEntryCounterShape : StaticEffect []
grantedEntryCounterShape =
  Gains (AllOf Macros.creatureYouControl) masterChefGrantedAbility

||| Hollowmurk Siege's Sultai mode — "Whenever a counter is put on a
||| creature you control, draw a card": the kind-blind SINGLE counter on
||| the trigger side. The mode's "This ability triggers only once each
||| turn" rider has no row yet.
hollowmurkSiegeSultai : Ability
hollowmurkSiegeSultai =
  Macros.triggered Whenever
    (Macros.bareCounterEvent CounterPut (Macros.a Macros.creatureYouControl))
    Macros.drawACard


shalaiAndHallar : Card
shalaiAndHallar =
  Macros.card "Shalai and Hallar"
       (Just [Macros.generic 1, Macros.pip Red, Macros.pip Green, Macros.pip White])
       [Legendary]
       (MkTypeLine [creatureType "Angel", creatureType "Elf"] [Creature])
       [ Macros.keyword "Flying"
       , Macros.keyword "Vigilance"
       , Macros.triggered Whenever
                          (Macros.manyCounterEvent CounterPut Macros.plusOnePlusOne
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
                                  (MkToken (Just (Lit 4 ** Lit 4)) [White]
                                           (MkTypeLine [creatureType "Angel"] [Creature])
                                           [Macros.keyword "Flying", Macros.keyword "Vigilance"]
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
                   (Macros.tokensCreated (CountedGroup (Macros.atLeast 1) IsToken))
                   (Create You (Times 2 GroupSize) TokenAsThose [])
                   Repeatedly)
       , Static (Intercepts
                   (Macros.manyCounterEvent CounterPut Macros.plusOnePlusOne
                                            (Macros.a Macros.creature))
                   (PutCounters (Times 2 ThatMuch)
                                Macros.plusOnePlusOne (That (TypeW Creature)))
                   Repeatedly) ]
       Nothing

adrixAndNev : Card
adrixAndNev =
  Macros.card "Adrix and Nev, Twincasters"
       (Just [Macros.generic 2, Macros.pip Green, Macros.pip Blue]) [Legendary]
       (MkTypeLine [creatureType "Merfolk", creatureType "Wizard"] [Creature])
       [ Macros.keywordCosting "Ward" (Mana [Macros.generic 2])
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
                            (MkToken (Just (Lit 2 ** Lit 2)) []
                                     (MkTypeLine [] [Creature]) [] Nothing)
                            (Just Land))
                  (Just Macros.untilEndOfTurn)) ]
       Nothing

mishrasFactory : Card
mishrasFactory =
  Macros.card "Mishra's Factory" Nothing [] (MkTypeLine [] [Land])
       [ Macros.activated TapSymbol (AddMana You (Lit 1) (Runs [[Colorless]]) [])
       , Macros.activated (Mana [Macros.generic 1])
                          (Continuously
                      (SetsType Macros.thisLand
                                (MkToken (Just (Lit 2 ** Lit 2)) []
                                         (MkTypeLine [creatureType "AssemblyWorker"] [Artifact, Creature])
                                         [] Nothing)
                                (Just Land))
                      (Just Macros.untilEndOfTurn))
       , Macros.activated TapSymbol
                          (Macros.gets (Macros.target (And [Macros.creature, HasSubtype (creatureType "AssemblyWorker")]))
                                (PtUp (Lit 1)) (PtUp (Lit 1)) (Just Macros.untilEndOfTurn)) ]
       Nothing

windZendikon : Card
windZendikon =
  Macros.card "Wind Zendikon" (Just [Macros.pip Blue]) []
       (MkTypeLine [enchantmentType "Aura"] [Enchantment])
       [ Macros.keywordSubject "Enchant" Macros.land
       , Static (SetsType (AttachHost Enchanted (TypeW Land))
                          (MkToken (Just (Lit 2 ** Lit 2)) [Blue]
                                   (MkTypeLine [creatureType "Elemental"] [Creature])
                                   [Macros.keyword "Flying"] Nothing)
                          (Just Land)) ]
       Nothing

nullhideFerox : Ability
nullhideFerox =
  Macros.activated (Mana [Macros.generic 2])
                   (Continuously (LosesAllAbilities Macros.thisCreature)
                          (Just Macros.untilEndOfTurn))


awakenTheBear : Card
awakenTheBear =
  Macros.card "Awaken the Bear" (Just [Macros.generic 2, Macros.pip Green]) []
       (MkTypeLine [] [Instant])
       [ Spell (Continuously
                  (AndAlso [ Gets (Macros.target Macros.creature)
                                  (PtUp (Lit 3)) (PtUp (Lit 3))
                           , Gains It (Macros.keyword "Trample") ])
                  (Just Macros.untilEndOfTurn)) ]
       Nothing

spidersilkArmor : Card
spidersilkArmor =
  Macros.card "Spidersilk Armor" (Just [Macros.generic 2, Macros.pip Green]) []
       (MkTypeLine [] [Enchantment])
       [ Static (AndAlso [ Gets (AllOf Macros.creatureYouControl)
                                (PtUp (Lit 0)) (PtUp (Lit 1))
                         , Gains Them (Macros.keyword "Reach") ]) ]
       Nothing

turnToFrog : Card
turnToFrog =
  Macros.card "Turn to Frog" (Just [Macros.generic 1, Macros.pip Blue]) []
       (MkTypeLine [] [Instant])
       [ Spell (Continuously
                  (AndAlso [ LosesAllAbilities (Macros.target Macros.creature)
                           , SetsType It (MkToken Nothing [Blue]
                                                  (MkTypeLine [creatureType "Frog"] []) [] Nothing)
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
       (MkTypeLine [enchantmentType "Aura"] [Enchantment])
       [ Macros.keywordSubject "Enchant" Macros.creature
       , Static (AndAlso [ Gets (AttachHost Enchanted (TypeW Creature))
                                (PtUp (Lit 1)) (PtUp (Lit 1))
                         , Gains It (Macros.keyword "Flying") ]) ]
       Nothing

bootsOfSpeed : Card
bootsOfSpeed =
  Macros.card "Boots of Speed" (Just [Macros.pip Red]) []
       (MkTypeLine [artifactType "Equipment"] [Artifact])
       [ Static (AndAlso [ Gets (AttachHost Equipped (TypeW Creature))
                                (PtUp (Lit 1)) (PtUp (Lit 0))
                         , Gains It (Macros.keyword "Haste") ])
       , Macros.keywordCosting "Equip" (Mana [Macros.generic 1]) ]
       Nothing

aetherTunnel : Card
aetherTunnel =
  Macros.card "Aether Tunnel" (Just [Macros.generic 1, Macros.pip Blue]) []
       (MkTypeLine [enchantmentType "Aura"] [Enchantment])
       [ Macros.keywordSubject "Enchant" Macros.creature
       , Static (AndAlso [ Gets (AttachHost Enchanted (TypeW Creature))
                                (PtUp (Lit 1)) (PtUp (Lit 0))
                         , Deontic It Forbid Block Patient NoDeonticPatient ]) ]
       Nothing

frogify : Card
frogify =
  Macros.card "Frogify" (Just [Macros.generic 1, Macros.pip Blue]) []
       (MkTypeLine [enchantmentType "Aura"] [Enchantment])
       [ Macros.keywordSubject "Enchant" Macros.creature
       , Static (AndAlso [ LosesAllAbilities (AttachHost Enchanted (TypeW Creature))
                         , SetsType It (MkToken Nothing [Blue]
                                                (MkTypeLine [creatureType "Frog"] [Creature]) [] Nothing)
                                    Nothing
                         , HasBasePt It (Lit 1) (Lit 1) ]) ]
       Nothing


darksteelMutation : Card
darksteelMutation =
  Macros.card "Darksteel Mutation" (Just [Macros.generic 1, Macros.pip White]) []
       (MkTypeLine [enchantmentType "Aura"] [Enchantment])
       [ Macros.keywordSubject "Enchant" Macros.creature
       , Static (AndAlso [ SetsType (AttachHost Enchanted (TypeW Creature))
                                    (MkToken Nothing []
                                             (MkTypeLine [creatureType "Insect"] [Artifact, Creature])
                                             [] Nothing)
                                    Nothing
                         , HasBasePt It (Lit 0) (Lit 1)
                         , Gains It (Macros.keyword "Indestructible")
                         , LosesAllAbilities It ]) ]
       Nothing

kenrithsTransformation : Card
kenrithsTransformation =
  Macros.card "Kenrith's Transformation" (Just [Macros.generic 1, Macros.pip Green]) []
       (MkTypeLine [enchantmentType "Aura"] [Enchantment])
       [ Macros.keywordSubject "Enchant" Macros.creature
       , Macros.triggered When (Enters Macros.thisAura) (Draw You (Lit 1))
       , Static (AndAlso [ LosesAllAbilities (AttachHost Enchanted (TypeW Creature))
                         , SetsType It (MkToken Nothing [Green]
                                                (MkTypeLine [creatureType "Elk"] [Creature]) [] Nothing)
                                    Nothing
                         , HasBasePt It (Lit 3) (Lit 3) ]) ]
       Nothing

amphibianDownpour : Card
amphibianDownpour =
  Macros.card "Amphibian Downpour" (Just [Macros.generic 2, Macros.pip Blue]) []
       (MkTypeLine [enchantmentType "Aura"] [Enchantment])
       [ Macros.keyword "Flash"
       , Macros.keyword "Storm"
       , Macros.keywordSubject "Enchant" Macros.creature
       , Static (AndAlso [ LosesAllAbilities (AttachHost Enchanted (TypeW Creature))
                         , SetsType It (MkToken Nothing [Blue]
                                                (MkTypeLine [creatureType "Frog"] [Creature]) [] Nothing)
                                    Nothing
                         , HasBasePt It (Lit 1) (Lit 1) ]) ]
       Nothing


lignify : Card
lignify =
  Macros.card "Lignify" (Just [Macros.generic 1, Macros.pip Green]) []
       (MkTypeLine [creatureType "Treefolk", enchantmentType "Aura"] [Kindred, Enchantment])
       [ Macros.keywordSubject "Enchant" Macros.creature
       , Static (AndAlso [ SetsType (AttachHost Enchanted (TypeW Creature))
                                    (MkToken Nothing [] (MkTypeLine [creatureType "Treefolk"] [])
                                             [] Nothing)
                                    Nothing
                         , HasBasePt It (Lit 0) (Lit 4)
                         , LosesAllAbilities It ]) ]
       Nothing

||| Invasion of Dominaria // Serra Faithkeeper, a nonmodal double-faced card
||| [CR#712.2]. Nothing here spells the turning over: [CR#310.12b] gives every
||| Siege the intrinsic ability that exiles it and casts it transformed, so the
||| printed text is only what each face says for itself. The back face writes
||| no mana cost of its own [CR#202.3a].
invasionOfDominaria : Card
invasionOfDominaria =
  Transforming
    (MkFace "Invasion of Dominaria" (Just [Macros.generic 2, Macros.pip White]) []
            (MkTypeLine [battleType "Siege"] [Battle])
            [ Macros.triggered When (Enters Macros.thisSiege)
                               (Sequentially [ ChangeLife You (Up (Lit 4))
                                             , Draw You (Lit 1) ]) ]
            (Macros.defenseBox 5))
    (MkAltFace "Serra Faithkeeper" [] (MkTypeLine [creatureType "Angel"] [Creature])
               [ Macros.keyword "Flying", Macros.keyword "Vigilance" ]
               (Macros.printedBox (Just (4, 4))))


causticTar : Card
causticTar =
  Macros.card "Caustic Tar" (Just [Macros.generic 4, Macros.pip Black, Macros.pip Black]) []
       (MkTypeLine [enchantmentType "Aura"] [Enchantment])
       [ Macros.keywordSubject "Enchant" Macros.land
       , Static (Gains (AttachHost Enchanted (TypeW Land))
                       (Macros.activated TapSymbol
                                         (ChangeLife (Macros.target AnyPlayer) (Down (Lit 3))))) ]
       Nothing

ragingRavine : Card
ragingRavine =
  Macros.card "Raging Ravine" Nothing [] (MkTypeLine [] [Land])
       [ Static (Macros.entersTapped Macros.thisLand)
       , Macros.activated TapSymbol
                          (AddMana You (Lit 1)
                            (Runs [[OfColor Red], [OfColor Green]]) [])
       , Macros.activated (Mana [Macros.generic 2, Macros.pip Red, Macros.pip Green])
                          (Continuously
                      (SetsType Macros.thisLand
                                (MkToken (Just (Lit 3 ** Lit 3)) [Red, Green]
                                         (MkTypeLine [creatureType "Elemental"] [Creature])
                                         [ Macros.triggered Whenever
                                                            (Macros.attacks Macros.thisCreature)
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
    [ Macros.triggered At (BeginningOf EndStep (ByWord Yours))
                       (Macros.create (Lit 3)
                   (MkToken (Just (Lit 1 ** Lit 1)) [White] (MkTypeLine [creatureType "Cat"] [Creature])
                            [Macros.keyword "Lifelink"] Nothing)) ]

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
  Macros.cardOf "Jace Beleren" (Just [Macros.generic 1, Macros.pip Blue, Macros.pip Blue])
       [Legendary] (MkTypeLine [planeswalkerType "Jace"] [Planeswalker])
       [ Macros.activated (LoyaltySymbol (LoyaltyUp 2)) (Draw (Each AnyPlayer) (Lit 1))
       , Macros.activated (LoyaltySymbol (LoyaltyDown 1))
                          (Macros.drawsACard (Macros.target AnyPlayer))
       , Macros.activated (LoyaltySymbol (LoyaltyDown 10))
                          (Macros.mills (Macros.target AnyPlayer) (Lit 20) They) ]
       (Macros.loyaltyBox 3)

elspethSunsChampion : Card
elspethSunsChampion =
  Macros.cardOf "Elspeth, Sun's Champion"
       (Just [Macros.generic 4, Macros.pip White, Macros.pip White])
       [Legendary] (MkTypeLine [planeswalkerType "Elspeth"] [Planeswalker])
       [ Macros.activated (LoyaltySymbol (LoyaltyUp 1))
                          (Macros.create (Lit 3) (Macros.creatureTok 1 1 [White] [creatureType "Soldier"]))
       , Macros.activated (LoyaltySymbol (LoyaltyDown 3))
                          (Macros.destroy (AllOf (And [Macros.creature,
                                                Compare Power AtLeast (Lit 4)])))
       , Macros.activated (LoyaltySymbol (LoyaltyDown 7))
                          (GetsEmblem You
                      [ Static (AndAlso [ Gets (AllOf Macros.creatureYouControl)
                                               (PtUp (Lit 2)) (PtUp (Lit 2))
                                        , Gains Them (Macros.keyword "Flying") ]) ]) ]
       (Macros.loyaltyBox 4)

||| Chandra Nalaar
chandraNalaarsX : Ability
chandraNalaarsX =
  Macros.activated (LoyaltySymbol LoyaltyDownX)
                   (DealDamage This (LetterVal X) (Macros.target Macros.creature))

||| Elspeth's Talent
elspethsTalentGrant : StaticEffect []
elspethsTalentGrant =
  Gains (AttachHost Enchanted (TypeW Planeswalker))
        (Macros.activated (LoyaltySymbol (LoyaltyUp 1))
                          (Macros.create (Lit 3) (Macros.creatureTok 1 1 [White] [creatureType "Soldier"])))


opt : Card
opt =
  Macros.card "Opt" (Just [Macros.pip Blue]) [] (MkTypeLine [] [Instant])
       [ Spell (Sequentially [ Does You "Scry" (Macros.lookAt (Macros.topCards 1))
                             , Macros.drawACard ]) ]
       Nothing

serumVisions : Card
serumVisions =
  Macros.card "Serum Visions" (Just [Macros.pip Blue]) [] (MkTypeLine [] [Sorcery])
       [ Spell (Sequentially [ Macros.drawACard
                             , Does You "Scry" (Macros.lookAt (Macros.topCards 2)) ]) ]
       Nothing

consider : Card
consider =
  Macros.card "Consider" (Just [Macros.pip Blue]) [] (MkTypeLine [] [Instant])
       [ Spell (Sequentially [ Does You "Surveil" (Macros.lookAt (Macros.topCards 1))
                             , Macros.drawACard ]) ]
       Nothing

crystalBall : Card
crystalBall =
  Macros.card "Crystal Ball" (Just [Macros.generic 3]) [] (MkTypeLine [] [Artifact])
       [ Macros.activated (Compound [Mana [Macros.generic 1], TapSymbol])
                          (Does You "Scry" (Macros.lookAt (Macros.topCards 2))) ]
       Nothing

nefariousImp : Card
nefariousImp =
  Macros.card "Nefarious Imp" (Just [Macros.generic 2, Macros.pip Black]) []
       (MkTypeLine [creatureType "Imp"] [Creature])
       [ Macros.keyword "Flying"
       , Macros.triggered Whenever
                          (Leaves (CountedGroup (Macros.atLeast 1)
                                         (And [Permanent, ControlledBy You])))
                          (Does You "Scry" (Macros.lookAt (Macros.topCards 1))) ]
       (Just (2, 1))

saheeliFiligreeMaster : Card
saheeliFiligreeMaster =
  Macros.cardOf "Saheeli, Filigree Master"
       (Just [Macros.generic 2, Macros.pip Blue, Macros.pip Red])
       [Legendary] (MkTypeLine [planeswalkerType "Saheeli"] [Planeswalker])
       [ Macros.activated (LoyaltySymbol (LoyaltyUp 1))
                          (Sequentially
                      [ Does You "Scry" (Macros.lookAt (Macros.topCards 1))
                      , Macros.mayThen You
                          (SetStatus Tapped
                             (Macros.a (And [Macros.artifact, Macros.untapped,
                                             ControlledBy You])))
                          Macros.drawACard ])
       , Macros.activated (LoyaltySymbol (LoyaltyDown 2))
                          (Sequentially
                      [ Macros.create (Lit 2)
                          (MkToken (Just (Lit 1 ** Lit 1)) []
                                   (MkTypeLine [creatureType "Thopter"] [Artifact, Creature])
                                   [Macros.keyword "Flying"] Nothing)
                      , Macros.gainsHaste Them (Just Macros.untilEndOfTurn) ])
       , Macros.activated (LoyaltySymbol (LoyaltyDown 4))
                          (GetsEmblem You
                      [ Static (Gets (AllOf (And [Macros.artifact, Macros.creature,
                                                  ControlledBy You]))
                                     (PtUp (Lit 1)) (PtUp (Lit 1)))
                      , Static (CostsToCast (AllOf (And [Macros.artifact, Macros.spell,
                                                         CastBy You]))
                                            (CostLess (Lit 1))) ]) ]
       (Macros.loyaltyBox 3)



manalith : Card
manalith =
  Macros.card "Manalith" (Just [Macros.generic 3]) [] (MkTypeLine [] [Artifact])
       [ Macros.activated TapSymbol (AddMana You (Lit 1) (AnyColor SameColor) []) ]
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
       [ Macros.activated TapSymbol
                          (AddMana You (Lit 1) (AnyColor SameColor)
                            [SpendOnly [ToCast (And [Macros.creature, Macros.spell])]]) ]
       Nothing

mishrasWorkshop : Card
mishrasWorkshop =
  Macros.card "Mishra's Workshop" Nothing [] (MkTypeLine [] [Land])
       [ Macros.activated TapSymbol
                          (AddMana You (Lit 1)
                            (Runs [[Colorless, Colorless, Colorless]])
                            [SpendOnly [ToCast (And [Macros.artifact, Macros.spell])]]) ]
       Nothing

boommobile : Ability
boommobile =
  Macros.triggered When (Enters Macros.thisArtifact)
                   (AddMana You (Lit 4) (AnyColor SameColor)
                     [SpendOnly [ToActivate Nothing]])



mizziumTransreliquat : Card
mizziumTransreliquat =
  Macros.card "Mizzium Transreliquat" (Just [Macros.generic 3]) []
       (MkTypeLine [] [Artifact])
       [ Macros.activated (Mana [Macros.generic 3])
                          (Continuously
                      (BecomesCopy Macros.thisArtifact
                                   (Macros.target Macros.artifact) [])
                      (Just Macros.untilEndOfTurn))
       , Macros.activated (Mana [Macros.generic 1, Macros.pip Blue, Macros.pip Red])
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
                               [ExceptAbility (Macros.keyword "Flying"),
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
               , Macros.delayed (BeginningOf EndStep NoPossessor) (Macros.exile It) ]

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
       (MkTypeLine [creatureType "Human", creatureType "Wizard"] [Creature])
       [ Macros.activated (Compound [Mana [Macros.generic 2, Macros.pip Blue], TapSymbol])
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
  Macros.activated (Compound [Mana [Macros.pip Blue, Macros.pip Blue], TapSymbol])
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
       (MkTypeLine [creatureType "Human", creatureType "Artificer"] [Creature])
       [ Macros.triggered Whenever
                          (Casts You (Macros.a (And [Or [HasSubtype (creatureType "Beast"), HasSubtype (creatureType "Bird")],
                                              Macros.creature, Macros.spell])))
                          (Macros.may You
                      (CopyStack You It (Lit 1)
                                 [ExceptTypes (MkTypeLine [] [Artifact])])) ]
       (Just (3, 5))

public export
etherealHaze : Card
etherealHaze =
  Macros.card "Ethereal Haze" (Just [Macros.pip White]) []
       (MkTypeLine [spellType "Arcane"] [Instant])
       [ Spell (Macros.preventAllBy AnyDamage Everywhere
                                    (AllOf Macros.creature) (Just Macros.thisTurn)) ]
       Nothing

public export
defang : Card
defang =
  Macros.card "Defang" (Just [Macros.generic 1, Macros.pip White]) []
       (MkTypeLine [enchantmentType "Aura"] [Enchantment])
       [ Macros.keywordSubject "Enchant" Macros.creature
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
       (MkTypeLine [creatureType "Avatar"] [Creature])
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
       [ Static (Macros.asLongAs (Matches Macros.thisArtifact (HasStatus Untapped))
                                 (PreventsFrom CombatOnly
                                             (DealtBy (Macros.a Macros.creature))
                                             (Macros.shieldingIt You)
                                             (CutSome (Lit 1)) Repeatedly Nothing))
       , Macros.activated (Compound [Mana [Macros.generic 2], TapSymbol])
                          (Macros.gets (AllOf (And [Macros.creature, Attacking]))
                                (PtUp (Lit 1)) (PtUp (Lit 0))
                                (Just Macros.untilEndOfTurn)) ]
       Nothing

public export
gideonAllyOfZendikar : Card
gideonAllyOfZendikar =
  Macros.cardOf "Gideon, Ally of Zendikar"
       (Just [Macros.generic 2, Macros.pip White, Macros.pip White]) [Legendary]
       (MkTypeLine [planeswalkerType "Gideon"] [Planeswalker])
       [ Macros.activated (LoyaltySymbol (LoyaltyUp 1))
                          (Sequentially
                      [ Continuously
                          (SetsType Macros.thisPlaneswalker
                                    (MkToken (Just (Lit 5 ** Lit 5)) []
                                             (MkTypeLine [creatureType "Human", creatureType "Soldier", creatureType "Ally"] [Creature])
                                             [Macros.keyword "Indestructible"] Nothing)
                                    (Just Planeswalker))
                          (Just Macros.untilEndOfTurn)
                      , Macros.preventAll AnyDamage (Macros.shieldingIt It)
                                          (Just Macros.thisTurn) ])
       , Macros.activated (LoyaltySymbol LoyaltyZero)
                          (Macros.create (Lit 1) (Macros.creatureTok 2 2 [White] [creatureType "Knight", creatureType "Ally"]))
       , Macros.activated (LoyaltySymbol (LoyaltyDown 4))
                          (GetsEmblem You [Static (Gets (AllOf Macros.creatureYouControl)
                                                 (PtUp (Lit 1)) (PtUp (Lit 1)))]) ]
       (Macros.loyaltyBox 4)

||| Gideon Jura. The [+2] is the fronted span: "During target opponent's
||| next turn" announces the target and "creatures that player controls"
||| reads it back, which is why the span leads the statement rather than
||| trailing it. Its patient is the source named by its own type -- an
||| attack aimed at a named permanent [CR#506.3].
public export
gideonJura : Card
gideonJura =
  Macros.cardOf "Gideon Jura"
       (Just [Macros.generic 3, Macros.pip White, Macros.pip White]) [Legendary]
       (MkTypeLine [planeswalkerType "Gideon"] [Planeswalker])
       [ Macros.activated (LoyaltySymbol (LoyaltyUp 2))
           (Throughout (DuringNextTurnOf (Macros.target Opponent))
                       (Deontic (AllOf (And [Macros.creature,
                                             ControlledBy (That PlayerW)]))
                                Require Attack Agent
                                (DefendingPlayer Macros.thisPlaneswalker)))
       , Macros.activated (LoyaltySymbol (LoyaltyDown 2))
           (Macros.destroy (Macros.target (And [Macros.creature, Macros.tapped])))
       , Macros.activated (LoyaltySymbol LoyaltyZero)
           (Sequentially
              [ Continuously
                  (SetsType Macros.thisPlaneswalker
                            (MkToken (Just (Lit 6 ** Lit 6)) []
                                     (MkTypeLine [creatureType "Human",
                                                  creatureType "Soldier"] [Creature])
                                     [] Nothing)
                            (Just Planeswalker))
                  (Just Macros.untilEndOfTurn)
              , Macros.preventAll AnyDamage (Macros.shieldingIt It)
                                  (Just Macros.thisTurn) ]) ]
       (Macros.loyaltyBox 6)

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
       (MkTypeLine [enchantmentType "Aura"] [Enchantment])
       [ Macros.keywordSubject "Enchant" Macros.creature
       , Static (Redirects AnyDamage AllOfIt (Macros.shieldingIt You) Nothing
                           (AttachHost Enchanted (TypeW Creature))) ]
       Nothing

public export
martyrsOfKorlis : Card
martyrsOfKorlis =
  Macros.card "Martyrs of Korlis"
       (Just [Macros.generic 3, Macros.pip White, Macros.pip White]) []
       (MkTypeLine [creatureType "Human"] [Creature])
       [ Static (Macros.asLongAs (Matches Macros.thisCreature (HasStatus Untapped))
                                 (Redirects AnyDamage AllOfIt (Macros.shieldingIt You)
                                          (Just (AllOf Macros.artifact))
                                          Macros.thisCreature)) ]
       (Just (1, 6))

public export
wardOfPiety : Card
wardOfPiety =
  Macros.card "Ward of Piety" (Just [Macros.generic 1, Macros.pip White]) []
       (MkTypeLine [enchantmentType "Aura"] [Enchantment])
       [ Macros.keywordSubject "Enchant" Macros.creature
       , Macros.activated (Mana [Macros.generic 1, Macros.pip White])
                          (Continuously
                      (Redirects AnyDamage (TheNext (Lit 1))
                                 (Macros.shieldingIt (AttachHost Enchanted (TypeW Creature)))
                                 Nothing (Macros.target Macros.anyTarget))
                      (Just Macros.thisTurn)) ]
       Nothing

public export
mirrorwoodTreefolk : Card
mirrorwoodTreefolk =
  Macros.card "Mirrorwood Treefolk" (Just [Macros.generic 3, Macros.pip Green]) []
       (MkTypeLine [creatureType "Treefolk"] [Creature])
       [ Macros.activated (Mana [Macros.generic 2, Macros.pip Red, Macros.pip White])
                          (Continuously
                      (RedirectsFrom AnyDamage Unattributed
                                     (Macros.shieldingIt Macros.thisCreature)
                                     (Macros.target Macros.anyTarget) NextTimeOnly)
                      (Just Macros.thisTurn)) ]
       (Just (2, 4))

public export
stormwildCapridor : Card
stormwildCapridor =
  Macros.card "Stormwild Capridor" (Just [Macros.generic 2, Macros.pip White]) []
       (MkTypeLine [creatureType "Bird", creatureType "Goat"] [Creature])
       [ Macros.keyword "Flying"
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
       (MkTypeLine [creatureType "Avatar"] [Creature])
       [ Macros.activated (Mana [Macros.pip White])
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
       [ Macros.activated (Mana [Macros.generic 1])
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
                  (Prevents AnyDamage (TheNext (LetterVal X))
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
       (MkTypeLine [spellType "Arcane"] [Instant])
       [ Spell (Continuously
                  (Prevents AnyDamage (TheNext (Lit 3))
                            (Macros.shieldingIt (Macros.target Macros.anyTarget))
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
       [ Macros.activated (Mana [Macros.generic 1])
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
                                (Macros.shieldingIt (Macros.target Macros.anyTarget))
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
                           (Macros.target (Macros.kindJoin AnyPlayer (HasType Planeswalker)))) ]
       Nothing

public export
searingFlesh : Card
searingFlesh =
  Macros.card "Searing Flesh" (Just [Macros.generic 6, Macros.pip Red]) []
       (MkTypeLine [] [Sorcery])
       [ Spell (DealDamage This (Lit 7)
                           (Macros.target (Macros.kindJoin Opponent (HasType Planeswalker)))) ]
       Nothing

public export
onakkeJavelineerBolt : Ability
onakkeJavelineerBolt =
  Macros.activated TapSymbol
                   (DealDamage Macros.thisCreature (Lit 2)
                        (Macros.target (Macros.kindJoin AnyPlayer (HasType Battle))))

||| Firesong and Sunspeaker's joined head -- "deals 3 damage to target
||| creature or player", the one current-oracle black-border phrasing that
||| still writes the pair out instead of "any target" -- with the echo that
||| reads it back. No printed card spells that echo, so the second clause is
||| the workbench's; it is what makes the pair `JoinP` carries observable,
||| since the joined head's creature half is the Object half's card type.
public export
firesongJoinEcho : Effect []
firesongJoinEcho =
  Sequentially [ DealDamage This (Lit 3)
                   (Macros.target (Macros.kindJoin AnyPlayer Macros.creature))
               , DealDamage This (Lit 1) (Macros.thatJoin) ]

||| Sorrow's Path's could-block test -- "if each of those creatures could
||| block all creatures that the other is blocking". [CR#509.1a] and
||| [CR#509.1b] are the whole of the reading: untapped, and no blocking
||| restriction disobeyed. "The other" has no shape, so the relatum is
||| written as the attacking creatures at large.
public export
sorrowsPathCouldBlock : Predicate [] Object
sorrowsPathCouldBlock = CouldBlock (AllOf (And [Macros.creature, Attacking]))

||| General Jarkeld's -- "if each of those creatures could be blocked by
||| all creatures that the other is blocked by": the same test in the other
||| voice, as `BlockedBy` is to `BlockerOf`. Same relatum drop.
public export
generalJarkeldCouldBeBlocked : Predicate [] Object
generalJarkeldCouldBeBlocked =
  CouldBeBlockedBy (AllOf (And [Macros.creature, Blocking]))

||| Sorrow's Path's reassignment -- "remove both of them from combat. Each
||| one then blocks all creatures the other was blocking." The removal and
||| the write back in, which [CR#509.3a] distinguishes from the reading
||| below: the creature was not a blocking creature between the two
||| clauses, so a "whenever [it] blocks" ability triggers again. The card's
||| own subject ("two target blocking creatures controlled by the same
||| opponent") is written as one, and "the other" is dropped.
public export
sorrowsPathReassign : Effect []
sorrowsPathReassign =
  Sequentially
    [ RemoveFromCombat (Macros.target (And [Macros.creature, Blocking]))
    , BecomesBlocking (That (TypeW Creature))
                      (Macros.a (And [Macros.creature, Attacking])) ]

||| General Jarkeld's -- "each creature that's blocking exactly one of
||| those attacking creatures stops blocking it and is blocking the other
||| attacking creature." The same reassignment written the other way: no
||| removal, so [CR#506.4] leaves the creature a blocking creature
||| throughout and [CR#509.3a] fires nothing. That the two terms differ at
||| all is the point; collapsing them would lose the card's ruling. Its
||| subject is written as the source for want of "each creature that's
||| blocking exactly one of those attacking creatures".
public export
generalJarkeldReassign : Effect []
generalJarkeldReassign =
  Sequentially
    [ StopsBlocking Macros.thisCreature
                    (Macros.target (And [Macros.creature, Attacking]))
    , BecomesBlocking Macros.thisCreature
                      (Macros.a (And [Macros.creature, Attacking])) ]

||| Tahngarth, First Mate's last two clauses -- "choose a player or
||| planeswalker that opponent is attacking. Tahngarth is attacking that
||| player or planeswalker." The choose mints the joined binding; the
||| attack declaration below reads it back in the DEFENDER slot, which is
||| what the joined slot buys: the card never says which half it names.
||| The restriction "that opponent is attacking" is dropped -- no predicate
||| describes a player by what is attacking it.
public export
tahngarthChoosesDefender : Effect []
tahngarthChoosesDefender =
  Choose (Macros.a (Macros.kindJoin AnyPlayer (HasType Planeswalker))) Nothing

||| "Tahngarth is attacking that player or planeswalker", in the context
||| the choose above leaves behind. [CR#506.3] closes what the slot may
||| name and the joined head is inside that set; the same slot refuses a
||| creature (`badCreatureAttackDefender`). The clause asserts the
||| assignment rather than declaring it, so it is `BecomesAttacking` and
||| not the declaration event: nothing is declared as an attacker by a
||| resolving effect [CR#508.3a].
public export
tahngarthAttacksThatJoin : Effect (effIntro Cards.tahngarthChoosesDefender)
tahngarthAttacksThatJoin =
  BecomesAttacking Macros.thisCreature (OneDefender Macros.thatJoin)

public export
endure : Card
endure =
  Macros.card "Endure"
       (Just [Macros.generic 3, Macros.pip White, Macros.pip White]) []
       (MkTypeLine [] [Instant])
       [ Spell (Macros.preventAll AnyDamage
                                  (Macros.shieldingIt
                                     (Macros.youAnd (AllOf (And [Permanent,
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
                                (Macros.youAnd (AllOf (And [Permanent,
                                                     ControlledBy You]))))
                             (Just (Macros.aYourChoice Macros.source))
                             (Macros.target Macros.anyTarget))
                  (Just Macros.thisTurn)) ]
       Nothing

public export
divineDeflection : Card
divineDeflection =
  Macros.card "Divine Deflection" (Just [Variable, Macros.pip White]) []
       (MkTypeLine [] [Instant])
       [ Spell (Continuously
                  (Prevents AnyDamage (TheNext (LetterVal X))
                            (Macros.shieldingIt
                               (Macros.youAnd (AllOf (And [Permanent,
                                                    ControlledBy You]))))
                            Nothing
                            (Just (DealDamage This ThatMuch
                                              (Macros.target Macros.anyTarget))))
                  (Just Macros.thisTurn)) ]
       Nothing

public export
glarecasterShield : Ability
glarecasterShield =
  Macros.activated (Mana [Macros.generic 5, Macros.pip White])
                   (Continuously
               (RedirectsFrom AnyDamage Unattributed
                              (Macros.shieldingIt (Macros.youAnd Macros.thisCreature))
                              (Macros.target Macros.anyTarget)
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
                           (Macros.a (Macros.kindJoin AnyPlayer Permanent)))
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
                           (Macros.a (Macros.kindJoin AnyPlayer Permanent)))
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
                           (Macros.a (Macros.kindJoin AnyPlayer Permanent)))
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
                           (Macros.a (Macros.kindJoin AnyPlayer Permanent)))
                        (Shifted ShiftUp (Lit 1)) Repeatedly) ]
       Nothing

public export
lashknifeBarrier : Card
lashknifeBarrier =
  Macros.card "Lashknife Barrier"
       (Just [Macros.generic 2, Macros.pip White]) []
       (MkTypeLine [] [Enchantment])
       [ Macros.triggered When (Enters Macros.thisEnchantment) Macros.drawACard
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
       (MkTypeLine [creatureType "Spirit"] [Creature])
       [ Static (Scales AnyDamage (Macros.a Macros.source)
                        (Macros.shieldingIt
                           (Macros.a (Macros.kindJoin AnyPlayer Permanent)))
                        (Halved RoundDown) Repeatedly) ]
       (Just (4, 5))

public export
fireServant : Card
fireServant =
  Macros.card "Fire Servant"
       (Just [Macros.generic 3, Macros.pip Red, Macros.pip Red]) []
       (MkTypeLine [creatureType "Elemental"] [Creature])
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
           [ Macros.choose (Macros.target Macros.anyTarget)
           , Draw You (Lit 3)
           , Macros.discards You (Macros.a (InZone Macros.handZ))
           , DealDamage This (Macros.manaValueOf (Macros.theVerbed "Discard" CardW))
                        (Macros.thatJoin) ]) ]
       Nothing

public export
riddleOfLightning : Card
riddleOfLightning =
  Macros.card "Riddle of Lightning"
       (Just [Macros.generic 3, Macros.pip Red, Macros.pip Red]) []
       (MkTypeLine [] [Instant])
       [ Spell (Sequentially
           [ Macros.choose (Macros.target Macros.anyTarget)
           , Does You "Scry" (Macros.lookAt (Macros.topCards 3))
           , Macros.revealCards Macros.topCard
           , DealDamage This (Macros.manaValueOf (That CardW)) (Macros.thatJoin) ]) ]
       Nothing

public export
platedPegasus : Card
platedPegasus =
  Macros.card "Plated Pegasus" (Just [Macros.generic 2, Macros.pip White]) []
       (MkTypeLine [creatureType "Pegasus"] [Creature])
       [ Macros.keyword "Flash"
       , Macros.keyword "Flying"
       , Static (PreventsFrom AnyDamage
                              (DealtBy (Macros.a Macros.spell))
                              (ToRecipient
                                 (Macros.a (Macros.kindJoin AnyPlayer Permanent)))
                              (CutSome (Lit 1))
                              Repeatedly
                              Nothing) ]
       (Just (1, 2))

public export
unstableShapeshifter : Card
unstableShapeshifter =
  Macros.card "Unstable Shapeshifter"
       (Just [Macros.generic 3, Macros.pip Blue]) []
       (MkTypeLine [creatureType "Shapeshifter"] [Creature])
       [ Macros.triggered Whenever
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
       [ Macros.activated (Mana [Macros.generic 1, Macros.pip Green, Macros.pip Green])
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
       (MkTypeLine [creatureType "Elemental"] [Creature])
       [ Macros.triggered Whenever
                          (IsDealtDamage Macros.thisCreature)
                          (DealDamage It ThatMuch (Macros.target Macros.anyTarget)) ]
       (Just (3, 3))

public export
grollub : Card
grollub =
  Macros.card "Grollub" (Just [Macros.generic 2, Macros.pip Black]) []
       (MkTypeLine [creatureType "Beast"] [Creature])
       [ Macros.triggered Whenever
                          (IsDealtDamage Macros.thisCreature)
                          (Macros.gainsLife (Each Opponent) ThatMuch) ]
       (Just (3, 3))

public export
moggManiac : Card
moggManiac =
  Macros.card "Mogg Maniac" (Just [Macros.generic 1, Macros.pip Red]) []
       (MkTypeLine [creatureType "Goblin"] [Creature])
       [ Macros.triggered Whenever
                          (IsDealtDamage Macros.thisCreature)
                          (DealDamage It ThatMuch
                               (Macros.target (Macros.kindJoin Opponent (HasType Planeswalker)))) ]
       (Just (1, 1))

public export
repercussion : Card
repercussion =
  Macros.card "Repercussion"
       (Just [Macros.generic 1, Macros.pip Red, Macros.pip Red]) []
       (MkTypeLine [] [Enchantment])
       [ Macros.triggered Whenever
                          (IsDealtDamage (Macros.a Macros.creature))
                          (DealDamage Macros.thisEnchantment ThatMuch
                               (ControllerOf (That (TypeW Creature)))) ]
       Nothing

public export
spitefulShadows : Card
spitefulShadows =
  Macros.card "Spiteful Shadows" (Just [Macros.generic 1, Macros.pip Black]) []
       (MkTypeLine [enchantmentType "Aura"] [Enchantment])
       [ Macros.keywordSubject "Enchant" Macros.creature
       , Macros.triggered Whenever
                          (IsDealtDamage (AttachHost Enchanted (TypeW Creature)))
                          (DealDamage It ThatMuch (ControllerOf It)) ]
       Nothing

public export
bindingAgony : Card
bindingAgony =
  Macros.card "Binding Agony" (Just [Macros.generic 1, Macros.pip Black]) []
       (MkTypeLine [enchantmentType "Aura"] [Enchantment])
       [ Macros.keywordSubject "Enchant" Macros.creature
       , Macros.triggered Whenever
                          (IsDealtDamage (AttachHost Enchanted (TypeW Creature)))
                          (DealDamage Macros.thisAura ThatMuch
                               (ControllerOf (That (TypeW Creature)))) ]
       Nothing

public export
darienKingOfKjeldor : Card
darienKingOfKjeldor =
  Macros.card "Darien, King of Kjeldor"
       (Just [Macros.generic 4, Macros.pip White, Macros.pip White]) [Legendary]
       (MkTypeLine [creatureType "Human", creatureType "Soldier"] [Creature])
       [ Macros.triggered Whenever
                          (IsDealtDamage You)
                          (Macros.may You
                      (Macros.create ThatMuch
                                     (Macros.creatureTok 1 1 [White] [creatureType "Soldier"]))) ]
       (Just (3, 3))

public export
livingHive : Card
livingHive =
  Macros.card "Living Hive"
       (Just [Macros.generic 6, Macros.pip Green, Macros.pip Green]) []
       (MkTypeLine [creatureType "Elemental", creatureType "Insect"] [Creature])
       [ Macros.keyword "Trample"
       , Macros.triggered Whenever
                          (DealsCombatDamage Macros.thisCreature (Macros.a AnyPlayer))
                          (Macros.create ThatMuch
                                  (Macros.creatureTok 1 1 [Green] [creatureType "Insect"])) ]
       (Just (6, 6))

||| "Whenever this creature is dealt damage, it deals that much damage to
||| any other target. If a player is dealt damage this way, they can't gain
||| life for the rest of the game." -- the union-narrowing container. The
||| damage went to a joined-kind phrase; `DealtThisWay` picks out the player
||| case, and "they" reads the joined mention back at its player half
||| ([CR#115.1] makes both readings real), which the condition has just
||| established was the one that happened.
public export
screamingNemesis : Card
screamingNemesis =
  Macros.card "Screaming Nemesis"
       (Just [Macros.generic 2, Macros.pip Red]) []
       (MkTypeLine [creatureType "Spirit"] [Creature])
       [ Macros.keyword "Haste"
       , Macros.triggered Whenever
                          (IsDealtDamage Macros.thisCreature)
                          (Sequentially
                             [ DealDamage It ThatMuch
                                 (Macros.target (And [Macros.anyTarget,
                                                      OtherThan This]))
                             , If (DealtThisWay AnyPlayer)
                                  (Continuously (PlayerCant GainsLife They)
                                                (Just RestOfGame))
                                  Nothing ]) ]
       (Just (3, 3))


||| "When this creature enters, it deals 2 damage to any target and you gain
||| 2 life. If a player is dealt damage this way, they discard a card." --
||| the second union-narrowing witness, and the one that shows the licence
||| has to be loose: the life-gain clause stands between the damage and the
||| conditional, so "this way" reads past it.
public export
sonicShrieker : Card
sonicShrieker =
  Macros.card "Sonic Shrieker"
       (Just [Macros.generic 2, Macros.pip Red, Macros.pip White,
              Macros.pip Black]) []
       (MkTypeLine [creatureType "Dragon"] [Creature])
       [ Macros.keyword "Flying"
       , Macros.triggered When
                          (Enters Macros.thisCreature)
                          (Sequentially
                             [ DealDamage It (Lit 2)
                                 (Macros.target Macros.anyTarget)
                             , Macros.gainsLife You (Lit 2)
                             , If (DealtThisWay AnyPlayer)
                                  (Macros.discardsACard They)
                                  Nothing ]) ]
       (Just (4, 4))


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
                  , DealDamage This (Lit 3) (Macros.target Macros.anyTarget) ]) ]
       Nothing

public export
giantCindermaw : Card
giantCindermaw =
  Macros.card "Giant Cindermaw" (Just [Macros.generic 2, Macros.pip Red]) []
       (MkTypeLine [creatureType "Dinosaur", creatureType "Beast"] [Creature])
       [ Macros.keyword "Trample"
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
       (MkTypeLine [enchantmentType "Aura"] [Enchantment])
       [ Macros.keywordSubject "Enchant" AnyPlayer
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
                          (MkToken Nothing [] (Macros.basicLandLine [landType "Mountain"])
                                   [] Nothing)
                          Nothing) ]
       Nothing

public export
magusOfTheMoon : Card
magusOfTheMoon =
  Macros.card "Magus of the Moon" (Just [Macros.generic 2, Macros.pip Red]) []
       (MkTypeLine [creatureType "Human", creatureType "Wizard"] [Creature])
       [ Static (SetsType (AllOf (And [Macros.land, Not (HasSupertype Basic)]))
                          (MkToken Nothing [] (Macros.basicLandLine [landType "Mountain"])
                                   [] Nothing)
                          Nothing) ]
       (Just (2, 2))

public export
harbingerOfTheSeas : Card
harbingerOfTheSeas =
  Macros.card "Harbinger of the Seas"
       (Just [Macros.generic 1, Macros.pip Blue, Macros.pip Blue]) []
       (MkTypeLine [creatureType "Merfolk", creatureType "Wizard"] [Creature])
       [ Static (SetsType (AllOf (And [Macros.land, Not (HasSupertype Basic)]))
                          (MkToken Nothing [] (Macros.basicLandLine [landType "Island"])
                                   [] Nothing)
                          Nothing) ]
       (Just (2, 2))

public export
lushGrowth : Card
lushGrowth =
  Macros.card "Lush Growth" (Just [Macros.pip Green]) []
       (MkTypeLine [enchantmentType "Aura"] [Enchantment])
       [ Macros.keywordSubject "Enchant" Macros.land
       , Static (SetsType (AttachHost Enchanted (TypeW Land))
                          (MkToken Nothing []
                                   (Macros.basicLandLine [landType "Mountain", landType "Forest", landType "Plains"])
                                   [] Nothing)
                          Nothing) ]
       Nothing

public export
yavimayaCradleOfGrowth : Card
yavimayaCradleOfGrowth =
  Macros.card "Yavimaya, Cradle of Growth" Nothing [Legendary]
       (MkTypeLine [] [Land])
       [ Static (BecomesAlso (AllOf Macros.land)
                             (MkToken Nothing [] (Macros.basicLandLine [landType "Forest"]) [] Nothing)) ]
       Nothing

public export
reefShaman : Card
reefShaman =
  Macros.card "Reef Shaman" (Just [Macros.pip Blue]) []
       (MkTypeLine [creatureType "Merfolk", creatureType "Shaman"] [Creature])
       [ Macros.activated TapSymbol
                          (Continuously (SetsChosenBasicType (Macros.target Macros.land))
                                 (Just Macros.untilEndOfTurn)) ]
       (Just (0, 2))

public export
grixisIllusionist : Card
grixisIllusionist =
  Macros.card "Grixis Illusionist" (Just [Macros.pip Blue]) []
       (MkTypeLine [creatureType "Human", creatureType "Wizard"] [Creature])
       [ Macros.activated TapSymbol
                          (Continuously
                      (SetsChosenBasicType
                         (Macros.target (And [Macros.land, ControlledBy You])))
                      (Just Macros.untilEndOfTurn)) ]
       (Just (1, 1))

public export
unstableFrontier : Card
unstableFrontier =
  Macros.card "Unstable Frontier" Nothing [] (MkTypeLine [] [Land])
       [ Macros.activated TapSymbol (AddMana You (Lit 1) (Runs [[Colorless]]) [])
       , Macros.activated TapSymbol
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
       (MkTypeLine [creatureType "Beast"] [Creature])
       [ Macros.activated (Compound [Mana [Macros.generic 2, Macros.pip Green],
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
       (MkTypeLine [creatureType "Human", creatureType "Wizard"] [Creature])
       [ Macros.activated (Compound [Mana [Macros.pip Blue],
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
                  [ Macros.choose (Macros.a (Macros.quality CreatureType))
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
                  [ Macros.choose (Macros.a (Macros.quality CreatureType))
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
       [ Static (Macros.entersChoosing Macros.thisEnchantment CreatureType)
       , Static (Gets (AllOf (And [Macros.creature, ControlledBy You,
                                   OfChosen CreatureType]))
                      (PtUp (Lit 1)) (PtUp (Lit 1))) ]
       Nothing

public export
sharedTriumph : Card
sharedTriumph =
  Macros.card "Shared Triumph" (Just [Macros.generic 1, Macros.pip White]) []
       (MkTypeLine [] [Enchantment])
       [ Static (Macros.entersChoosing Macros.thisEnchantment CreatureType)
       , Static (Gets (AllOf (And [Macros.creature, OfChosen CreatureType]))
                      (PtUp (Lit 1)) (PtUp (Lit 1))) ]
       Nothing

public export
hallOfTriumph : Card
hallOfTriumph =
  Macros.card "Hall of Triumph" (Just [Macros.generic 3]) [Legendary]
       (MkTypeLine [] [Artifact])
       [ Static (Macros.entersChoosing Macros.thisArtifact Color)
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
       [ Static (Macros.entersChoosing Macros.thisEnchantment CreatureType)
       , Static (Gets (AllOf (And [Macros.creature, OfChosen CreatureType]))
                      (PtDown (Lit 1)) (PtDown (Lit 1))) ]
       Nothing

public export
prismRing : Card
prismRing =
  Macros.card "Prism Ring" (Just [Macros.generic 1]) []
       (MkTypeLine [] [Artifact])
       [ Static (Macros.entersChoosing Macros.thisArtifact Color)
       , Macros.triggered Whenever
                          (Casts You (Macros.a (And [Macros.spell, OfChosen Color])))
                          (Macros.gainsLife You (Lit 1)) ]
       Nothing

public export
urzasIncubator : Card
urzasIncubator =
  Macros.card "Urza's Incubator" (Just [Macros.generic 3]) []
       (MkTypeLine [] [Artifact])
       [ Static (Macros.entersChoosing Macros.thisArtifact CreatureType)
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
       [ Static (Macros.entersChoosing Macros.thisEnchantment CreatureType)
       , Static (Gets (AllOf (And [Macros.creature, ControlledBy You,
                                   OfChosen CreatureType]))
                      (PtUp (Lit 1)) (PtUp (Lit 1)))
       , Macros.activated (Compound [Mana [Macros.generic 1],
                              Do (Macros.sacrifice You
                                    (Macros.a (And [Macros.creature,
                                                    OfChosen CreatureType])))])
                          (Macros.gains (Macros.target Macros.creatureYouControl)
                                 (Macros.keyword "Indestructible")
                                 (Just Macros.untilEndOfTurn)) ]
       Nothing

public export
voiceOfAll : Card
voiceOfAll =
  Macros.card "Voice of All"
       (Just [Macros.generic 2, Macros.pip White, Macros.pip White]) []
       (MkTypeLine [creatureType "Angel"] [Creature])
       [ Macros.keyword "Flying"
       , Static (Macros.entersChoosing Macros.thisCreature Color)
       , Static (Gains Macros.thisCreature
                       (Macros.keywordQuality "Protection" (OfChosen Color))) ]
       (Just (2, 2))

public export
wardSliver : Card
wardSliver =
  Macros.card "Ward Sliver" (Just [Macros.generic 4, Macros.pip White]) []
       (MkTypeLine [creatureType "Sliver"] [Creature])
       [ Static (Macros.entersChoosing Macros.thisCreature Color)
       , Static (Gains (AllOf (HasSubtype (creatureType "Sliver")))
                       (Macros.keywordQuality "Protection" (OfChosen Color))) ]
       (Just (2, 2))

public export
chromaticArmor : Card
chromaticArmor =
  Macros.card "Chromatic Armor"
       (Just [Macros.generic 1, Macros.pip White, Macros.pip Blue]) []
       (MkTypeLine [enchantmentType "Aura"] [Enchantment])
       [ Macros.keywordSubject "Enchant" Macros.creature
       , Static (Macros.entersChoosing Macros.thisAura Color)
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
       [ Static (Macros.entersChoosing Macros.thisEnchantment CreatureType)
       , Static (AddsChosenQuality (AllOf (And [Macros.creature, ControlledBy You]))
                                (OfChosen CreatureType)) ]
       Nothing

public export
adaptiveAutomaton : Card
adaptiveAutomaton =
  Macros.card "Adaptive Automaton" (Just [Macros.generic 3]) []
       (MkTypeLine [creatureType "Construct"] [Artifact, Creature])
       [ Static (Macros.entersChoosing Macros.thisCreature CreatureType)
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
       (MkTypeLine [creatureType "Illusion"] [Creature])
       [ Macros.keyword "Flying"
       , Macros.activated (Mana [Macros.generic 1])
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
       [ Static (Macros.entersChoosing Macros.thisEnchantment CreatureType)
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
       [ Static (Macros.entersChoosing Macros.thisEnchantment CreatureType)
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
       [ Static (Macros.entersChoosing Macros.thisEnchantment CardName)
       , Macros.activated (Mana [Macros.pip Blue])
                          (Macros.counterSpell
                      (Macros.target (And [Macros.spell, Named ChosenName]))) ]
       Nothing

public export
cursedScroll : Card
cursedScroll =
  Macros.card "Cursed Scroll" (Just [Macros.generic 1]) []
       (MkTypeLine [] [Artifact])
       [ Macros.activated (Compound [Mana [Macros.generic 3], TapSymbol])
                          (Sequentially
                      [ Macros.choose (Macros.a (Macros.quality CardName))
                      , Macros.revealCards
                          (Macros.aAtRandom (InZone (Macros.handOf You)))
                      , If (Matches (That CardW) (Named ChosenName))
                           (DealDamage Macros.thisArtifact (Lit 2)
                              (Macros.target Macros.anyTarget))
                           Nothing ]) ]
       Nothing

public export
magusOfTheScroll : Card
magusOfTheScroll =
  Macros.card "Magus of the Scroll" (Just [Macros.pip Red]) []
       (MkTypeLine [creatureType "Human", creatureType "Wizard"] [Creature])
       [ Macros.activated (Compound [Mana [Macros.generic 3], TapSymbol])
                          (Sequentially
                      [ Macros.choose (Macros.a (Macros.quality CardName))
                      , Macros.revealCards
                          (Macros.aAtRandom (InZone (Macros.handOf You)))
                      , If (Matches (That CardW) (Named ChosenName))
                           (DealDamage Macros.thisCreature (Lit 2)
                              (Macros.target Macros.anyTarget))
                           Nothing ]) ]
       (Just (1, 1))

public export
imagecrafter : Card
imagecrafter =
  Macros.card "Imagecrafter" (Just [Macros.pip Blue]) []
       (MkTypeLine [creatureType "Human", creatureType "Wizard"] [Creature])
       [ Macros.activated TapSymbol
           (Sequentially
              [ Macros.choose (Macros.a (Macros.qualityFrom CreatureType (TypeOtherThan (creatureType "Wall"))))
              , Continuously (SetsChosenQuality (Macros.target Macros.creature)
                                                (OfChosen CreatureType))
                             (Just Macros.untilEndOfTurn) ]) ]
       (Just (1, 1))

public export
unnaturalSelection : Card
unnaturalSelection =
  Macros.card "Unnatural Selection" (Just [Macros.generic 1, Macros.pip Blue]) []
       (MkTypeLine [] [Enchantment])
       [ Macros.activated (Mana [Macros.generic 1])
           (Sequentially
              [ Macros.choose (Macros.a (Macros.qualityFrom CreatureType (TypeOtherThan (creatureType "Wall"))))
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
              [ Macros.choose (Macros.a (Macros.qualityFrom CreatureType (TypeOtherThan (creatureType "Wall"))))
              , Continuously (SetsChosenQuality (Each Macros.creature)
                                                (OfChosen CreatureType))
                             (Just Macros.untilEndOfTurn) ]) ]
       Nothing

public export
silverquillSilencer : Card
silverquillSilencer =
  Macros.card "Silverquill Silencer" (Just [Macros.pip White, Macros.pip Black]) []
       (MkTypeLine [creatureType "Human", creatureType "Cleric"] [Creature])
       [ Static (Macros.entersChoosingFrom Macros.thisCreature
                                           CardName
                                           (NameOfCard (Not (HasType Land))))
       , Macros.triggered Whenever
           (Casts Macros.anOpponent
                  (Macros.a (And [Macros.spell, Named ChosenName])))
           (Sequentially [ Macros.losesLife They (Lit 3), Macros.drawACard ]) ]
       (Just (3, 2))

public export
candlesOfLeng : Card
candlesOfLeng =
  Macros.card "Candles of Leng" (Just [Macros.generic 2]) []
       (MkTypeLine [] [Artifact])
       [ Macros.activated (Compound [Mana [Macros.generic 4], TapSymbol])
           (Sequentially
              [ Macros.revealCards Macros.topCard
              , If (Matches It (Named (SameNameAs
                                  (Macros.a (InZone (Macros.graveyardOf You))))))
                          (Macros.move It Macros.graveyardZ)
                          (Just Macros.drawACard) ]) ]
       Nothing

public export
avenShrine : Card
avenShrine =
  Macros.card "Aven Shrine"
       (Just [Macros.generic 1, Macros.pip White, Macros.pip White]) []
       (MkTypeLine [] [Enchantment])
       [ Macros.triggered Whenever
           (Casts (Macros.a AnyPlayer) (Macros.a Macros.spell))
           (Sequentially
              [ Macros.gainsLife They (LetterVal X)
              , Define X (CountOf (And [InZone Macros.graveyardZ,
                                 Named (SameNameAs (That SpellW))])) ]) ]
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
       (MkTypeLine [creatureType "Human", creatureType "Wizard"] [Creature])
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

||| "Choose a card name other than a basic land card name." The postnominal
||| exception, written as the negated conjunction the phrase states.
public export
necromentiaChoice : Effect []
necromentiaChoice =
  Macros.choose (Macros.a (Macros.qualityFrom CardName
                   (NameOfCard (Not (And [HasSupertype Basic, HasType Land])))))

||| Booby Trap's name half: "As this artifact enters, choose … a card name
||| other than a basic land card name." The same exception in the as-enters
||| slot; the opponent chosen alongside it wants a second choice inside one
||| clause, which the as-enters chooser does not carry.
public export
boobyTrapNameChoice : StaticEffect []
boobyTrapNameChoice =
  Macros.entersChoosingFrom Macros.thisArtifact
                            CardName
                            (NameOfCard (Not (And [HasSupertype Basic,
                                                   HasType Land])))

||| "Return any number of permanent cards with different names from your
||| graveyard to the battlefield."
public export
eerieUltimatum : Card
eerieUltimatum =
  Macros.card "Eerie Ultimatum"
       (Just [Macros.pip White, Macros.pip White, Macros.pip Black,
              Macros.pip Black, Macros.pip Black, Macros.pip Green,
              Macros.pip Green]) []
       (MkTypeLine [] [Sorcery])
       [ Spell (Macros.move
                  (Macros.withDifferentNames
                     (CountedGroup Macros.anyNumber
                                   (And [Permanent,
                                         InZone (Macros.graveyardOf You)])))
                  Macros.battlefieldZ) ]
       Nothing

||| "Flying / Discard two nonland cards with the same name: Draw four cards."
||| The positive pole with no relatum written, in a cost.
public export
sphinxOfTheChimes : Card
sphinxOfTheChimes =
  Macros.card "Sphinx of the Chimes"
       (Just [Macros.generic 4, Macros.pip Blue, Macros.pip Blue]) []
       (MkTypeLine [creatureType "Sphinx"] [Creature])
       [ Macros.keyword "Flying"
       , Macros.activated
           (Do (Macros.discards You
                  (Macros.withTheSameName
                     (CountedGroup (Macros.exactly 2)
                                   (And [Not Macros.land,
                                         InZone Macros.handZ])))))
           (Macros.drawCards 4) ]
       (Just (5, 6))

||| "When this creature enters, if you control two or more nonland, nontoken
||| permanents with the same name as one another, create a 4/4 colorless
||| Construct artifact creature token."
public export
chromeReplicator : Card
chromeReplicator =
  Macros.card "Chrome Replicator" (Just [Macros.generic 5]) []
       (MkTypeLine [creatureType "Construct"] [Artifact, Creature])
       [ Macros.triggeredIf When (Enters Macros.thisCreature)
           (ExistsGroup (Macros.withTheSameName
                           (CountedGroup (Macros.atLeast 2)
                                         (And [Permanent, Not Macros.land,
                                               Not IsToken, ControlledBy You]))))
           (Macros.create (Lit 1)
              (MkToken (Just (Lit 4 ** Lit 4)) []
                       (MkTypeLine [creatureType "Construct"] [Artifact, Creature])
                       [] Nothing)) ]
       (Just (4, 4))

||| "{2}, {T}: Draw a card. Activate only if you control three or more lands
||| with the same name." The elliptical positive pole in a guard.
public export
endlessAtlas : Card
endlessAtlas =
  Macros.card "Endless Atlas" (Just [Macros.generic 2]) []
       (MkTypeLine [] [Artifact])
       [ Macros.activatedOnlyIf (Compound [Mana [Macros.generic 2], TapSymbol])
                                Macros.drawACard
                                (ExistsGroup (Macros.withTheSameName
                                   (CountedGroup (Macros.atLeast 3)
                                                 (And [Macros.land,
                                                       ControlledBy You])))) ]
       Nothing

||| Saheeli Rai's +1. The -2 is `saheelisCopy`; the card stays off the
||| bench for its last one, whose counted search has no mention to carry a
||| count.
public export
saheeliRaiPlusOne : Effect []
saheeliRaiPlusOne =
  Sequentially [ Does You "Scry" (Macros.lookAt (Macros.topCards 1))
               , DealDamage This (Lit 1) (Each Opponent) ]

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
           [ Macros.choose (Macros.a (Macros.quality CardName))
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
       (MkTypeLine [enchantmentType "Saga"] [Enchantment])
       [ Macros.triggered When (ChapterMark [ChapterI, ChapterII])
           (Macros.create (Lit 1)
              (MkToken (Just (Lit 2 ** Lit 2)) [White]
                       (MkTypeLine [creatureType "Knight"] [Creature])
                       [Macros.keyword "Vigilance"] Nothing))
       , Macros.triggered When (ChapterMark [ChapterIII])
           (Macros.gets (AllOf (And [HasSubtype (creatureType "Knight"), ControlledBy You]))
                        (PtUp (Lit 2)) (PtUp (Lit 1))
                        (Just Macros.untilEndOfTurn)) ]
       Nothing

public export
vault75MiddleSchool : Card
vault75MiddleSchool =
  Macros.card "Vault 75: Middle School"
       (Just [Macros.generic 2, Macros.pip White, Macros.pip White]) []
       (MkTypeLine [enchantmentType "Saga"] [Enchantment])
       [ Macros.triggered When (ChapterMark [ChapterI])
           (Macros.exile (AllOf (And [Macros.creature,
                                      Compare Power AtLeast (Lit 4)])))
       , Macros.triggered When (ChapterMark [ChapterII, ChapterIII])
           (PutCounters (Lit 1) Macros.plusOnePlusOne
                        (Each Macros.creatureYouControl)) ]
       Nothing

public export
keldonWarcaller : Card
keldonWarcaller =
  Macros.card "Keldon Warcaller"
       (Just [Macros.generic 1, Macros.pip Red]) []
       (MkTypeLine [creatureType "Human", creatureType "Warrior"] [Creature])
       [ Macros.triggered Whenever (Macros.attacks Macros.thisCreature)
           (PutCounters (Lit 1) Lore
                        (Macros.target (And [HasSubtype (enchantmentType "Saga"), ControlledBy You]))) ]
       (Just (2, 2))

public export
foundingOfOmashu : Card
foundingOfOmashu =
  Macros.card "Founding of Omashu"
       (Just [Macros.generic 2, Macros.pip Red]) []
       (MkTypeLine [enchantmentType "Saga"] [Enchantment])
       [ Macros.triggered When (ChapterMark [ChapterI])
           (Macros.create (Lit 2) (Macros.creatureTok 1 1 [White] [creatureType "Ally"]))
       , Macros.triggered When (ChapterMark [ChapterII])
           (Macros.mayThen You (Macros.discardsACard You) Macros.drawACard)
       , Macros.triggered When (ChapterMark [ChapterIII])
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
       (MkTypeLine [creatureType "Myr"] [Artifact, Creature])
       [ Macros.keyword "Flash"
       , Static (Macros.mayCastAsThough You
                                        (AllOf (And [Macros.spell, HasType Artifact]))
                                        HadFlash) ]
       (Just (2, 2))

public export
quickSliver : Card
quickSliver =
  Macros.card "Quick Sliver" (Just [Macros.generic 1, Macros.pip Green]) []
       (MkTypeLine [creatureType "Sliver"] [Creature])
       [ Macros.keyword "Flash"
       , Static (Macros.mayCastAsThough (Macros.a AnyPlayer)
                                        (AllOf (And [Macros.spell, HasSubtype (creatureType "Sliver")]))
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
       (MkTypeLine [creatureType "Human", creatureType "Wizard"] [Creature])
       [ Macros.triggered When (Enters Macros.thisCreature)
           (Macros.mills You (Lit 4) You)
       , Static (Macros.mayCastFromLimited You
                                           (Macros.a (And [Macros.spell, HasType Creature,
                                                           HasSubtype (creatureType "Zombie")]))
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
       (MkTypeLine [creatureType "Goblin", creatureType "Rogue"] [Creature])
       [ Static (Visibility Reveal You TopOfLibrary) ]
       (Just (1, 1))

public export
garruksHorde : Card
garruksHorde =
  Macros.card "Garruk's Horde"
       (Just [Macros.generic 5, Macros.pip Green, Macros.pip Green]) []
       (MkTypeLine [creatureType "Beast"] [Creature])
       [ Macros.keyword "Trample"
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
       (MkTypeLine [creatureType "Human", creatureType "Wizard"] [Creature])
       [ Static (Visibility Reveal You TopOfLibrary)
       , Static playLandsAndCastSpellsFromTop ]
       (Just (2, 3))

public export
courserOfKruphix : Card
courserOfKruphix =
  Macros.card "Courser of Kruphix"
       (Just [Macros.generic 1, Macros.pip Green, Macros.pip Green]) []
       (MkTypeLine [creatureType "Centaur"] [Enchantment, Creature])
       [ Static (Visibility Reveal You TopOfLibrary)
       , Static (Macros.mayPlayFrom You (AllOf Macros.land) Macros.onTopZ)
       , Macros.triggered Whenever
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
       (MkTypeLine [creatureType "Illusion"] [Creature])
       [ Macros.keyword "Flying"
       , Static (Visibility Reveal (PlayerGroup AllPlayers) WholeHand) ]
       (Just (1, 3))

public export
korlessaScaleSinger : Card
korlessaScaleSinger =
  Macros.card "Korlessa, Scale Singer"
       (Just [Macros.pip Green, Macros.pip Blue]) [Legendary]
       (MkTypeLine [creatureType "Dragon", creatureType "Bard"] [Creature])
       [ Static (Visibility LookAt You TopOfLibrary)
       , Static (Macros.mayCastFrom You
                                    (AllOf (And [Macros.spell, HasSubtype (creatureType "Dragon")]))
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
       , Macros.activated (Mana [Macros.generic 3])
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
       , Macros.activated (Compound [TapSymbol, Do (Macros.losesLife You (Lit 1))])
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
       (MkTypeLine [creatureType "Elf", creatureType "Shaman"] [Creature])
       [ Static (MayPlayAdditionalLands You (Macros.exactly 1))
       , Static (Visibility Reveal You TopOfLibrary)
       , Static (Macros.mayPlayFrom You (AllOf Macros.land) Macros.onTopZ) ]
       (Just (2, 2))

public export
azusaLostButSeeking : Card
azusaLostButSeeking =
  Macros.card "Azusa, Lost but Seeking"
       (Just [Macros.generic 2, Macros.pip Green]) [Legendary]
       (MkTypeLine [creatureType "Human", creatureType "Monk"] [Creature])
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
       (MkTypeLine [creatureType "Nymph", creatureType "Dryad"] [Enchantment, Creature])
       [ Static (MayPlayAdditionalLands You (Macros.exactly 1))
       , Static (AddsEveryType (AllOf (And [Macros.land, ControlledBy You]))
                               BasicLandSpace) ]
       (Just (2, 4))

public export
mistformUltimus : Card
mistformUltimus =
  Macros.card "Mistform Ultimus"
       (Just [Macros.generic 3, Macros.pip Blue]) [Legendary]
       (MkTypeLine [creatureType "Illusion"] [Creature])
       [ Static (AddsEveryType Macros.thisCreature CreatureSpace) ]
       (Just (3, 3))

public export
amorphousAxe : Card
amorphousAxe =
  Macros.card "Amorphous Axe" (Just [Macros.generic 2]) []
       (MkTypeLine [artifactType "Equipment"] [Artifact])
       [ Static (AndAlso
           [ Gets (AttachHost Equipped (TypeW Creature)) (PtUp (Lit 3))
                  (PtUp (Lit 0))
           , AddsEveryType (AttachHost Equipped (TypeW Creature))
                           CreatureSpace ])
       , Macros.keywordCosting "Equip" (Mana [Macros.generic 3]) ]
       Nothing

public export
runedStalactite : Card
runedStalactite =
  Macros.card "Runed Stalactite" (Just [Macros.generic 1]) []
       (MkTypeLine [artifactType "Equipment"] [Artifact])
       [ Static (AndAlso
           [ Gets (AttachHost Equipped (TypeW Creature)) (PtUp (Lit 1))
                  (PtUp (Lit 1))
           , AddsEveryType (AttachHost Equipped (TypeW Creature))
                           CreatureSpace ])
       , Macros.keywordCosting "Equip" (Mana [Macros.generic 2]) ]
       Nothing

public export
arachnoform : Card
arachnoform =
  Macros.card "Arachnoform" (Just [Macros.generic 1, Macros.pip Green]) []
       (MkTypeLine [enchantmentType "Aura"] [Enchantment])
       [ Macros.keywordSubject "Enchant" Macros.creature
       , Static (AndAlso
           [ Gets (AttachHost Enchanted (TypeW Creature)) (PtUp (Lit 2))
                  (PtUp (Lit 2))
           , Gains (AttachHost Enchanted (TypeW Creature)) (Macros.keyword "Reach")
           , AddsEveryType (AttachHost Enchanted (TypeW Creature))
                           CreatureSpace ]) ]
       Nothing

public export
nyleasPresence : Card
nyleasPresence =
  Macros.card "Nylea's Presence"
       (Just [Macros.generic 1, Macros.pip Green]) []
       (MkTypeLine [enchantmentType "Aura"] [Enchantment])
       [ Macros.keywordSubject "Enchant" Macros.land
       , Macros.triggered When (Enters Macros.thisAura) Macros.drawACard
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
       [Macros.activated (Compound [TapSymbol,
                             Do (Macros.sacrifice You
                                   (CountedGroup (Macros.exactly 5) Macros.artifact))])
                         (ExtraTurn You (Lit 1))] Nothing

public export
magistratesScepter : Card
magistratesScepter =
  Macros.card "Magistrate's Scepter" (Just [Macros.generic 3]) []
       (MkTypeLine [] [Artifact])
       [ Macros.activated (Compound [Mana [Macros.generic 4], TapSymbol])
                          (PutCounters (Lit 1) Charge Macros.thisArtifact)
       , Macros.activated (Compound [TapSymbol,
                              Do (RemoveCounters (Lit 3) (Just Charge) Macros.thisArtifact)])
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
       (MkTypeLine [creatureType "Angel"] [Creature])
       [ Macros.keyword "Flying"
       , Macros.triggered Whenever
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
       , Macros.triggered At (BeginningOf Upkeep (ByWord Yours))
           (Macros.mayElse You (Pay You (Mana [Macros.pip Blue]))
                           (Macros.sacrifice You Macros.thisEnchantment)) ] Nothing

public export
yawgmothsBargain : Card
yawgmothsBargain =
  Macros.card "Yawgmoth's Bargain"
       (Just [Macros.generic 4, Macros.pip Black, Macros.pip Black]) []
       (MkTypeLine [] [Enchantment])
       [ Static (Skips You DrawStep)
       , Macros.activated (Macros.payLife You 1) Macros.drawACard ] Nothing

public export
sandsOfTimeSkip : StaticEffect []
sandsOfTimeSkip = Skips (Each AnyPlayer) UntapStep

public export
wormfangManta : Card
wormfangManta =
  Macros.card "Wormfang Manta"
       (Just [Macros.generic 5, Macros.pip Blue, Macros.pip Blue]) []
       (MkTypeLine [creatureType "Nightmare", creatureType "Fish", creatureType "Beast"] [Creature])
       [ Macros.keyword "Flying"
       , Macros.triggered When (Enters Macros.thisCreature) (SkipsNext You Turn (Lit 1))
       , Macros.triggered When (Leaves Macros.thisCreature) (ExtraTurn You (Lit 1)) ]
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
                             Macros.delayed (BeginningOf EndStep (ByWord ThatTurns))
                                            (Concludes LoseGame You)])] Nothing

public export
lastChance : Card
lastChance =
  Macros.card "Last Chance" (Just [Macros.pip Red, Macros.pip Red]) []
       (MkTypeLine [] [Sorcery])
       [Spell (Sequentially [ExtraTurn You (Lit 1),
                             Macros.delayed (BeginningOf EndStep (ByWord ThatTurns))
                                            (Concludes LoseGame You)])] Nothing

public export
chanceForGlory : Card
chanceForGlory =
  Macros.card "Chance for Glory"
       (Just [Macros.generic 1, Macros.pip Red, Macros.pip White]) []
       (MkTypeLine [] [Instant])
       [Spell (Sequentially
         [ Continuously (Gains (AllOf (And [Macros.creature, ControlledBy You]))
                               (Macros.keyword "Indestructible")) Nothing
         , ExtraTurn You (Lit 1)
         , Macros.delayed (BeginningOf EndStep (ByWord ThatTurns))
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
                          Macros.happenedTo AttackDeclaration Lookback.ThisTurn]))
         , Macros.additionalPartThen Combat (Just MainPhase) (Lit 1) MainPhase])] Nothing

public export
hellkiteCharger : Card
hellkiteCharger =
  Macros.card "Hellkite Charger"
       (Just [Macros.generic 4, Macros.pip Red, Macros.pip Red]) []
       (MkTypeLine [creatureType "Dragon"] [Creature])
       [ Macros.keyword "Flying"
       , Macros.keyword "Haste"
       , Macros.triggered Whenever (Macros.attacks Macros.thisCreature)
           (Macros.mayThen You
              (Pay You (Mana [Macros.generic 5, Macros.pip Red, Macros.pip Red]))
              (Sequentially
                [ SetStatus Untapped (AllOf (And [Macros.creature, Attacking]))
                , Macros.additionalPart Combat Nothing (Lit 1)])) ] (Just (5, 5))

public export
fullThrottleFirstLine : Effect []
fullThrottleFirstLine = Macros.additionalPart Combat (Just MainPhase) (Lit 2)

public export
raphaelAdditionalCombat : Effect []
raphaelAdditionalCombat = Macros.additionalPart Combat (Just Combat) (Lit 1)

public export
yshtolaAdditionalEndStep : Effect []
yshtolaAdditionalEndStep = Macros.additionalPart EndStep Nothing (Lit 1)



public export
grumgully : Card
grumgully =
  Macros.card "Grumgully, the Generous"
       (Just [Macros.generic 1, Macros.pip Red, Macros.pip Green]) [Legendary]
       (MkTypeLine [creatureType "Goblin", creatureType "Shaman"] [Creature])
       [Static (Macros.entersWithAdditionalCounters (Each (And [Macros.creature, ControlledBy You,
                                                                Not (HasSubtype (creatureType "Human")),
                                                                OtherThan Macros.thisCreature]))
                                                    (Lit 1)
                                                    Macros.plusOnePlusOne)]
       (Just (3, 3))

public export
dragonstormGlobe : Card
dragonstormGlobe =
  Macros.card "Dragonstorm Globe" (Just [Macros.generic 3]) []
       (MkTypeLine [] [Artifact])
       [ Static (Macros.entersWithAdditionalCounters (Each (And [HasSubtype (creatureType "Dragon"), ControlledBy You]))
                                                     (Lit 1)
                                                     Macros.plusOnePlusOne)
       , Macros.activated TapSymbol (AddMana You (Lit 1) (AnyColor SameColor) []) ]
       Nothing

public export
sageOfFables : Card
sageOfFables =
  Macros.card "Sage of Fables" (Just [Macros.generic 2, Macros.pip Blue]) []
       (MkTypeLine [creatureType "Merfolk", creatureType "Wizard"] [Creature])
       [ Static (Macros.entersWithAdditionalCounters (Each (And [Macros.creature, HasSubtype (creatureType "Wizard"), ControlledBy You,
                                                                 OtherThan Macros.thisCreature]))
                                                     (Lit 1)
                                                     Macros.plusOnePlusOne)
       , Macros.activated (Compound [Mana [Macros.generic 2],
                              Do (RemoveCounters (Lit 1) (Just Macros.plusOnePlusOne)
                                   (Macros.a (And [Macros.creature, ControlledBy You])))])
                          Macros.drawACard ]
       (Just (2, 2))

public export
metallicMimic : Card
metallicMimic =
  Macros.card "Metallic Mimic" (Just [Macros.generic 2]) []
       (MkTypeLine [creatureType "Shapeshifter"] [Artifact, Creature])
       [ Static (Macros.entersChoosing Macros.thisCreature CreatureType)
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
  Macros.entersWithCounters (AllOf (And [Macros.creature, ControlledBy You,
                                  HasStatus FaceDown]))
                            (Lit 1) (KeywordCounter "Flying")

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
                                      (KeywordCounter "Vigilance")

public export
phyrexianRevoker : Card
phyrexianRevoker =
  Macros.card "Phyrexian Revoker" (Just [Macros.generic 2]) []
       (MkTypeLine [creatureType "Horror"] [Artifact, Creature])
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
       (MkTypeLine [creatureType "Gargoyle"] [Creature])
       [ Macros.keyword "Flying"
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
       [ Static (Macros.entersChoosing Macros.thisArtifact CardName)
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
       (MkTypeLine [creatureType "Dragon"] [Creature])
       [ Macros.keyword "Flying"
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
       (MkTypeLine [creatureType "Human", creatureType "Assassin"] [Creature])
       [ Static (CostsToCast
                   (AllOf (And [Macros.spell, HasSubtype (artifactType "Equipment"), CastBy You]))
                   (CostLess (Lit 1)))
       , Static (CostsToCast
                   (AllOf (And [AbilityHead (KeywordClass "Equip"), ActivatedBy You]))
                   (CostLess (Lit 1))) ]
       (Just (2, 2))

public export
demotion : Card
demotion =
  Macros.card "Demotion" (Just [Macros.pip White]) []
       (MkTypeLine [enchantmentType "Aura"] [Enchantment])
       [ Macros.keywordSubject "Enchant" Macros.creature
       , Static (AndAlso
           [ Deontic (AttachHost Enchanted (TypeW Creature)) Forbid Block Agent NoDeonticPatient
           , ObjectCant Activated
               (AllOf (And [AbilityHead AnyActivated, AbilityOf It])) ]) ]
       Nothing

public export
vipersKiss : Card
vipersKiss =
  Macros.card "Viper's Kiss" (Just [Macros.pip Black]) []
       (MkTypeLine [enchantmentType "Aura"] [Enchantment])
       [ Macros.keywordSubject "Enchant" Macros.creature
       , Static (AndAlso
           [ Gets (AttachHost Enchanted (TypeW Creature)) (PtDown (Lit 1)) (PtDown (Lit 1))
           , ObjectCant Activated
               (AllOf (And [AbilityHead AnyActivated, AbilityOf It])) ]) ]
       Nothing

public export
stupefyingTouch : Card
stupefyingTouch =
  Macros.card "Stupefying Touch" (Just [Macros.generic 1, Macros.pip Blue]) []
       (MkTypeLine [enchantmentType "Aura"] [Enchantment])
       [ Macros.keywordSubject "Enchant" Macros.creature
       , Macros.triggered When (Enters Macros.thisAura) (Draw You (Lit 1))
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
       (MkTypeLine [creatureType "Angel"] [Creature])
       [ Macros.keyword "Flying"
       , Static (ObjectCant Activated
                   (AllOf (And [ AbilityHead AnyActivated
                               , AbilityOf (AllOf (And [Macros.creature,
                                                        ControlledBy (PlayerGroup YourOpponents)])) ]))) ]
       (Just (3, 4))

public export
eidolonOfObstruction : Card
eidolonOfObstruction =
  Macros.card "Eidolon of Obstruction" (Just [Macros.generic 1, Macros.pip White]) []
       (MkTypeLine [creatureType "Spirit"] [Enchantment, Creature])
       [ Macros.keyword "FirstStrike"
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
                   (Macros.a (And [Macros.land, HasSubtype (landType "Mountain")]))))))
       , Spell (Macros.destroy (Macros.target Macros.artifact)) ]
       Nothing

public export
moggSalvage : Card
moggSalvage =
  Macros.card "Mogg Salvage" (Just [Macros.generic 2, Macros.pip Red]) []
       (MkTypeLine [] [Instant])
       [ Static (Macros.asLongAs
                   (AndCond
                      [ Exists (And [Macros.land, HasSubtype (landType "Island"),
                                     ControlledBy Macros.anOpponent])
                      , Exists (And [Macros.land, HasSubtype (landType "Mountain"),
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
                   (Macros.a (And [HasSubtype (landType "Plains"), InZone Macros.handZ]))))))
       , Spell (Macros.destroy (Macros.target (Or [Macros.artifact,
                                                   Macros.enchantment]))) ]
       Nothing

public export
gush : Card
gush =
  Macros.card "Gush" (Just [Macros.generic 4, Macros.pip Blue]) []
       (MkTypeLine [] [Instant])
       [ Static (AltCost (Just (Do (Macros.move
                   (CountedGroup (Macros.exactly 2)
                                 (And [Macros.land, HasSubtype (landType "Island"),
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
       [ Static (Macros.asLongAs
                   (Macros.happened AttackDeclaration You Lookback.ThisTurn)
                   (AltCost (Just (Mana [Macros.pip Blue]))))
       , Spell (Macros.counterSpell (Macros.target Macros.spell)) ]
       Nothing

public export
massacre : Card
massacre =
  Macros.card "Massacre"
       (Just [Macros.generic 2, Macros.pip Black, Macros.pip Black]) []
       (MkTypeLine [] [Sorcery])
       [ Static (Macros.asLongAs
                   (AndCond
                      [ Exists (And [Macros.land, HasSubtype (landType "Plains"),
                                     ControlledBy Macros.anOpponent])
                      , Exists (And [Macros.land, HasSubtype (landType "Swamp"),
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
       (MkTypeLine [creatureType "Goblin"] [Creature])
       [ Static (Macros.asLongAs
                   (AndCond
                      [ Exists (And [Macros.land, HasSubtype (landType "Plains"),
                                     ControlledBy Macros.anOpponent])
                      , Exists (And [Macros.land, HasSubtype (landType "Mountain"),
                                     ControlledBy You]) ])
                   (AltCost Nothing))
       , Macros.keyword "Haste" ]
       (Just (1, 1))

public export
rouse : Card
rouse =
  Macros.card "Rouse" (Just [Macros.generic 1, Macros.pip Black]) []
       (MkTypeLine [] [Instant])
       [ Static (Macros.asLongAs
                   (Exists (And [Macros.land, HasSubtype (landType "Swamp"), ControlledBy You]))
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
       (MkTypeLine [creatureType "Demon"] [Creature])
       [ Static (AltCost (Just (Compound
                   [ Do (ChangeLife You (Down (Lit 6)))
                   , Do (Macros.sacrifice You
                           (CountedGroup (Macros.exactly 3)
                                         (And [Macros.creature, ColorIs Black]))) ])))
       , Macros.keyword "Flying"
       , Macros.keyword "Trample" ]
       (Just (9, 9))

public export
thwart : Card
thwart =
  Macros.card "Thwart"
       (Just [Macros.generic 2, Macros.pip Blue, Macros.pip Blue]) []
       (MkTypeLine [] [Instant])
       [ Static (AltCost (Just (Do (Macros.move
                   (CountedGroup (Macros.exactly 3)
                                 (And [Macros.land, HasSubtype (landType "Island"),
                                       ControlledBy You]))
                   Macros.handZ))))
       , Spell (Macros.counterSpell (Macros.target Macros.spell)) ]
       Nothing

public export
theLadyOfOtariaLine : StaticEffect []
theLadyOfOtariaLine =
  AltCost (Just (Do (SetStatus Tapped
            (CountedGroup (Macros.exactly 3)
                          (And [Macros.creature, HasSubtype (creatureType "Dwarf"),
                                ControlledBy You, HasStatus Untapped])))))

public export
shiningShoal : Card
shiningShoal =
  Macros.card "Shining Shoal"
       (Just [Variable, Macros.pip White, Macros.pip White]) []
       (MkTypeLine [spellType "Arcane"] [Instant])
       [ Static (AltCost (Just (Do (Macros.exile
                   (Macros.a (And [ColorIs White,
                                   Compare ManaValue Eq (LetterVal X),
                                   InZone (Macros.handOf You)]))))))
       , Spell (Continuously
                  (Redirects AnyDamage (TheNext (LetterVal X))
                             (Macros.shieldingIt
                                (Macros.youAnd (AllOf (And [HasType Creature,
                                                     ControlledBy You]))))
                             (Just (Macros.aYourChoice Macros.source))
                             (Macros.target Macros.anyTarget))
                  (Just Macros.thisTurn)) ]
       Nothing

public export
disruptingShoal : Card
disruptingShoal =
  Macros.card "Disrupting Shoal"
       (Just [Variable, Macros.pip Blue, Macros.pip Blue]) []
       (MkTypeLine [spellType "Arcane"] [Instant])
       [ Static (AltCost (Just (Do (Macros.exile
                   (Macros.a (And [ColorIs Blue,
                                   Compare ManaValue Eq (LetterVal X),
                                   InZone (Macros.handOf You)]))))))
       , Spell (OnlyIf (Macros.counterSpell (Macros.target Macros.spell))
                       (CompareAmt (Macros.manaValueOf It) Eq (LetterVal X)) Nothing) ]
       Nothing

||| Mana Leak: "Counter target spell unless its controller pays {3}."
||| The unless-arm reads the spell the clause just named [CR#118.12a].
public export
manaLeak : Card
manaLeak =
  Macros.card "Mana Leak" (Just [Macros.generic 1, Macros.pip Blue]) []
       (MkTypeLine [] [Instant])
       [ Spell (Unless (Macros.counterSpell (Macros.target Macros.spell))
                       (ControllerOf It)
                       (Mana [Macros.generic 3])) ]
       Nothing

||| Rhystic Study: "Whenever an opponent casts a spell, you may draw a card
||| unless that player pays {1}." [CR#118.12a] over an offered body.
public export
rhysticStudy : Card
rhysticStudy =
  Macros.card "Rhystic Study"
       (Just [Macros.generic 2, Macros.pip Blue]) []
       (MkTypeLine [] [Enchantment])
       [ Macros.triggered Whenever
           (Casts Macros.anOpponent (Macros.a Macros.spell))
           (Unless (Macros.may You Macros.drawACard)
                          (That PlayerW)
                          (Mana [Macros.generic 1])) ]
       Nothing

public export
blazingShoal : Card
blazingShoal =
  Macros.card "Blazing Shoal"
       (Just [Variable, Macros.pip Red, Macros.pip Red]) []
       (MkTypeLine [spellType "Arcane"] [Instant])
       [ Static (AltCost (Just (Do (Macros.exile
                   (Macros.a (And [ColorIs Red,
                                   Compare ManaValue Eq (LetterVal X),
                                   InZone (Macros.handOf You)]))))))
       , Spell (Macros.gets (Macros.target Macros.creature) (PtUp (LetterVal X))
                            (PtUp (Lit 0)) (Just Macros.untilEndOfTurn)) ]
       Nothing

public export
sickeningShoal : Card
sickeningShoal =
  Macros.card "Sickening Shoal"
       (Just [Variable, Macros.pip Black, Macros.pip Black]) []
       (MkTypeLine [spellType "Arcane"] [Instant])
       [ Static (AltCost (Just (Do (Macros.exile
                   (Macros.a (And [ColorIs Black,
                                   Compare ManaValue Eq (LetterVal X),
                                   InZone (Macros.handOf You)]))))))
       , Spell (Macros.gets (Macros.target Macros.creature) (PtDown (LetterVal X))
                            (PtDown (LetterVal X)) (Just Macros.untilEndOfTurn)) ]
       Nothing

public export
nourishingShoal : Card
nourishingShoal =
  Macros.card "Nourishing Shoal"
       (Just [Variable, Macros.pip Green, Macros.pip Green]) []
       (MkTypeLine [spellType "Arcane"] [Instant])
       [ Static (AltCost (Just (Do (Macros.exile
                   (Macros.a (And [ColorIs Green,
                                   Compare ManaValue Eq (LetterVal X),
                                   InZone (Macros.handOf You)]))))))
       , Spell (ChangeLife You (Up (LetterVal X))) ]
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
                                       Compare ManaValue Eq (LetterVal X)]))) ]
       Nothing

public export
repeal : Card
repeal =
  Macros.card "Repeal" (Just [Variable, Macros.pip Blue]) []
       (MkTypeLine [] [Instant])
       [ Spell (Sequentially
                  [ Macros.move (Macros.target (And [Not Macros.land, Permanent,
                                              Compare ManaValue Eq (LetterVal X)]))
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
                                          Compare ManaValue Eq (LetterVal X)])))
                  Nothing) ]
       Nothing

public export
ratchetBomb : Card
ratchetBomb =
  Macros.card "Ratchet Bomb" (Just [Macros.generic 2]) []
       (MkTypeLine [] [Artifact])
       [ Macros.activated TapSymbol
           (PutCounters (Lit 1) Charge Macros.thisArtifact)
       , Macros.activated (Compound [TapSymbol,
                              Do (Macros.sacrifice You Macros.thisArtifact)])
           (Macros.destroy (Each (And [Not Macros.land, Permanent,
                                       Compare ManaValue Eq
                                         (CountersOn Charge Macros.thisArtifact)]))) ]
       Nothing

public export
entDraughtBasinAbility : Ability
entDraughtBasinAbility =
  Macros.activated (Compound [Mana [Variable], TapSymbol])
                   (PutCounters (Lit 1) Macros.plusOnePlusOne
               (Macros.target (And [Macros.creatureYouControl,
                                    Compare Power Eq (LetterVal X)])))

public export
sarkhansUnsealingLine : Ability
sarkhansUnsealingLine =
  Macros.triggered Whenever
    (Casts You (Macros.a (And [Macros.creature, Macros.spell,
                               Or [Compare Power Eq (Lit 4),
                                   Compare Power Eq (Lit 5),
                                   Compare Power Eq (Lit 6)]])))
    (DealDamage Macros.thisEnchantment (Lit 4) (Macros.target Macros.anyTarget))

public export
savageSwipeLine : Effect []
savageSwipeLine =
  OnlyIf (Macros.gets (Macros.target Macros.creatureYouControl) (PtUp (Lit 2))
                      (PtUp (Lit 2)) (Just Macros.untilEndOfTurn))
         (CompareAmt (Macros.powerOf It) Eq (Lit 2)) Nothing

public export
drudgeSkeletons : Card
drudgeSkeletons =
  Macros.card "Drudge Skeletons" (Just [Macros.generic 1, Macros.pip Black]) []
       (MkTypeLine [creatureType "Skeleton"] [Creature])
       [ Macros.activated (Mana [Macros.pip Black]) (Regenerate Macros.thisCreature) ]
       (Just (1, 1))

public export
asphodelWanderer : Card
asphodelWanderer =
  Macros.card "Asphodel Wanderer" (Just [Macros.pip Black]) []
       (MkTypeLine [creatureType "Skeleton", creatureType "Soldier"] [Creature])
       [ Macros.activated (Mana [Macros.generic 2, Macros.pip Black])
                          (Regenerate Macros.thisCreature) ]
       (Just (1, 1))

public export
deathWard : Card
deathWard =
  Macros.card "Death Ward" (Just [Macros.pip White]) []
       (MkTypeLine [] [Instant])
       [ Spell (Regenerate (Macros.target Macros.creature)) ]
       Nothing

||| "{1}: Regenerate this creature. When it regenerates this way, put a
||| -1/-1 counter on it." — "this way" binds the trigger to the shield
||| this resolution created [CR#701.19a], so it waits for that shield to
||| apply rather than being checked at once [CR#603.7,603.12].
public export
matopiGolem : Card
matopiGolem =
  Macros.card "Matopi Golem" (Just [Macros.generic 5]) []
       (MkTypeLine [creatureType "Golem"] [Artifact, Creature])
       [ Macros.activated (Mana [Macros.generic 1])
                          (ThisWay (Regenerate Macros.thisCreature)
                            (Regenerates Macros.thisCreature)
                            (PutCounters (Lit 1) Macros.minusOneMinusOne It)) ]
       (Just (3, 3))

||| "{R}: This creature gets +1/+0 until end of turn. When its power
||| becomes 20 this way, it deals 20 damage to any target." — the
||| enclosure is a continuous effect with no agent to inflect, so only
||| [CR#603.12]'s outcome-bound template reaches it.
public export
infernoOfTheStarMounts : Card
infernoOfTheStarMounts =
  Macros.card "Inferno of the Star Mounts"
       (Just [Macros.generic 4, Macros.pip Red, Macros.pip Red]) [Legendary]
       (MkTypeLine [creatureType "Dragon"] [Creature])
       [ Static (ObjectCant Countered This)
       , Macros.keyword "Flying"
       , Macros.keyword "Haste"
       , Macros.activated (Mana [Macros.pip Red])
                          (ThisWay (Macros.gets Macros.thisCreature (PtUp (Lit 1))
                                         (PtUp (Lit 0)) (Just Macros.untilEndOfTurn))
                            (StatBecomes It Power (Lit 20))
                            (DealDamage It (Lit 20) (Macros.target Macros.anyTarget))) ]
       (Just (6, 6))

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
       [ Static (Macros.asLongAs
                   (Exists (And [Macros.land, HasSubtype (landType "Swamp"),
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

||| "Destroy all creatures, then create an X/X colorless Phyrexian Horror
||| artifact creature token, where X is the number of creatures destroyed
||| this way." The letter reads the group the destruction left, through
||| `CountOfGroup`.
public export
phyrexianRebirth : Card
phyrexianRebirth =
  Macros.card "Phyrexian Rebirth"
       (Just [Macros.generic 4, Macros.pip White, Macros.pip White]) []
       (MkTypeLine [] [Sorcery])
       [ Spell (Sequentially
                  [ Macros.destroy (AllOf Macros.creature)
                  , Create You (Lit 1)
                           (TokenWritten
                              (MkToken (Just (LetterVal X ** LetterVal X)) []
                                       (MkTypeLine [creatureType "Phyrexian", creatureType "Horror"]
                                                   [Artifact, Creature])
                                       [] Nothing))
                           []
                  , Define X (CountOfGroup
                                (Macros.thoseVerbedThisWay "Destroy"
                                                           (TypeW Creature))) ]) ]
       Nothing

||| "Incinerate deals 3 damage to any target. A creature dealt damage this
||| way can't be regenerated this turn." The bare participle is the
||| lookback reader with `ThisWay` in the window's place.
public export
incinerate : Card
incinerate =
  Macros.card "Incinerate" (Just [Macros.generic 1, Macros.pip Red]) []
       (MkTypeLine [] [Instant])
       [ Spell (Sequentially
                  [ DealDamage This (Lit 3) (Macros.target Macros.anyTarget)
                  , Continuously
                      (ObjectCant Regenerated
                         (Macros.a (And [Macros.creature,
                                         HappenedTo DamageTaken Lookback.ThisWay
                                                    Nothing])))
                      (Just Macros.thisTurn) ]) ]
       Nothing

public export
hurrJackalAbility : Ability
hurrJackalAbility =
  Macros.activated TapSymbol
    (Continuously (ObjectCant Regenerated (Macros.target Macros.creature))
                  (Just Macros.thisTurn))

public export
solGrail : Card
solGrail =
  Macros.card "Sol Grail" (Just [Macros.generic 3]) []
       (MkTypeLine [] [Artifact])
       [ Static (Macros.entersChoosing Macros.thisArtifact Color)
       , Macros.activated TapSymbol
           (AddMana You (Lit 1) (OfChosenColor Nothing) []) ]
       Nothing

public export
unchartedHaven : Card
unchartedHaven =
  Macros.card "Uncharted Haven" Nothing []
       (MkTypeLine [] [Land])
       [ Static (Macros.entersTapped Macros.thisLand)
       , Static (Macros.entersChoosing Macros.thisLand Color)
       , Macros.activated TapSymbol
           (AddMana You (Lit 1) (OfChosenColor Nothing) []) ]
       Nothing

public export
mirageMesa : Card
mirageMesa =
  Macros.card "Mirage Mesa" Nothing []
       (MkTypeLine [landType "Desert"] [Land])
       [ Static (Macros.entersTapped Macros.thisLand)
       , Static (Macros.entersChoosing Macros.thisLand Color)
       , Macros.activated TapSymbol
           (AddMana You (Lit 1) (OfChosenColor Nothing) []) ]
       Nothing

public export
crossroadsVillage : Card
crossroadsVillage =
  Macros.card "Crossroads Village" Nothing []
       (MkTypeLine [landType "Town"] [Land])
       [ Static (Macros.entersTapped Macros.thisLand)
       , Static (Macros.entersChoosing Macros.thisLand Color)
       , Macros.activated TapSymbol
           (AddMana You (Lit 1) (OfChosenColor Nothing) []) ]
       Nothing

public export
thrivingBluff : Card
thrivingBluff =
  Macros.card "Thriving Bluff" Nothing []
       (MkTypeLine [] [Land])
       [ Static (Macros.entersTapped Macros.thisLand)
       , Static (Macros.entersChoosingFrom Macros.thisLand Color (ColorOtherThan Red))
       , Macros.activated TapSymbol
           (AddMana You (Lit 1) (OfChosenColor (Just [OfColor Red])) []) ]
       Nothing

public export
thrivingGrove : Card
thrivingGrove =
  Macros.card "Thriving Grove" Nothing []
       (MkTypeLine [] [Land])
       [ Static (Macros.entersTapped Macros.thisLand)
       , Static (Macros.entersChoosingFrom Macros.thisLand Color (ColorOtherThan Green))
       , Macros.activated TapSymbol
           (AddMana You (Lit 1) (OfChosenColor (Just [OfColor Green])) []) ]
       Nothing

public export
thrivingHeath : Card
thrivingHeath =
  Macros.card "Thriving Heath" Nothing []
       (MkTypeLine [] [Land])
       [ Static (Macros.entersTapped Macros.thisLand)
       , Static (Macros.entersChoosingFrom Macros.thisLand Color (ColorOtherThan White))
       , Macros.activated TapSymbol
           (AddMana You (Lit 1) (OfChosenColor (Just [OfColor White])) []) ]
       Nothing

public export
thrivingIsle : Card
thrivingIsle =
  Macros.card "Thriving Isle" Nothing []
       (MkTypeLine [] [Land])
       [ Static (Macros.entersTapped Macros.thisLand)
       , Static (Macros.entersChoosingFrom Macros.thisLand Color (ColorOtherThan Blue))
       , Macros.activated TapSymbol
           (AddMana You (Lit 1) (OfChosenColor (Just [OfColor Blue])) []) ]
       Nothing

public export
thrivingMoor : Card
thrivingMoor =
  Macros.card "Thriving Moor" Nothing []
       (MkTypeLine [] [Land])
       [ Static (Macros.entersTapped Macros.thisLand)
       , Static (Macros.entersChoosingFrom Macros.thisLand Color (ColorOtherThan Black))
       , Macros.activated TapSymbol
           (AddMana You (Lit 1) (OfChosenColor (Just [OfColor Black])) []) ]
       Nothing



public export
bramblewoodParagon : Card
bramblewoodParagon =
  Macros.card "Bramblewood Paragon"
       (Just [Macros.generic 1, Macros.pip Green]) []
       (MkTypeLine [creatureType "Elf", creatureType "Warrior"] [Creature])
       [ Static (Macros.entersWithAdditionalCounters (Each (And [Macros.creature, HasSubtype (creatureType "Warrior"), ControlledBy You,
                                                                 OtherThan Macros.thisCreature]))
                                                     (Lit 1)
                                                     Macros.plusOnePlusOne)
       , Static (Gains (Each (And [Macros.creature, ControlledBy You,
                                   HasCounters (Just Macros.plusOnePlusOne)]))
                       (Macros.keyword "Trample")) ]
       (Just (2, 2))

public export
oonasBlackguard : Card
oonasBlackguard =
  Macros.card "Oona's Blackguard"
       (Just [Macros.generic 1, Macros.pip Black]) []
       (MkTypeLine [creatureType "Faerie", creatureType "Rogue"] [Creature])
       [ Macros.keyword "Flying"
       , Static (Macros.entersWithAdditionalCounters (Each (And [Macros.creature, HasSubtype (creatureType "Rogue"), ControlledBy You,
                                                                 OtherThan Macros.thisCreature]))
                                                     (Lit 1)
                                                     Macros.plusOnePlusOne)
       , Macros.triggered Whenever
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
       [ Macros.triggered At (BeginningOf Upkeep (ByWord Yours))
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
       (MkTypeLine [creatureType "Elf", creatureType "Assassin"] [Creature])
       [ Macros.triggered When (Enters Macros.thisCreature)
                          (PutCounters (Lit 1) Macros.plusOnePlusOne
                      (Macros.target Macros.creatureYouDontControl))
       , Macros.activated (Compound [Mana [Macros.generic 2, Macros.pip Black], TapSymbol])
                          (Macros.destroy
                      (Macros.target (And [Macros.creature, HasCounters Nothing]))) ]
       (Just (3, 3))

public export
pridemalkin : Card
pridemalkin =
  Macros.card "Pridemalkin"
       (Just [Macros.generic 2, Macros.pip Green]) []
       (MkTypeLine [creatureType "Cat"] [Creature])
       [ Macros.triggered When (Enters Macros.thisCreature)
                          (PutCounters (Lit 1) Macros.plusOnePlusOne
                      (Macros.target Macros.creatureYouControl))
       , Static (Gains (Each (And [Macros.creature, ControlledBy You,
                                   HasCounters (Just Macros.plusOnePlusOne)]))
                       (Macros.keyword "Trample")) ]
       (Just (2, 1))



public export
glacialChasm : Card
glacialChasm =
  Macros.card "Glacial Chasm" Nothing []
       (MkTypeLine [] [Land])
       [ Macros.keywordCosting "CumulativeUpkeep" (Macros.payLife You 2)
       , Macros.triggered When (Enters Macros.thisLand)
                          (Macros.sacrifice You (Macros.a Macros.land))
       , Static (Deontic (AllOf Macros.creatureYouControl) Forbid Attack Agent NoDeonticPatient)
       , Static (Prevents AnyDamage AllOfIt (Macros.shieldingIt You) Nothing Nothing) ]
       Nothing

public export
aboroth : Card
aboroth =
  Macros.card "Aboroth"
       (Just [Macros.generic 4, Macros.pip Green, Macros.pip Green]) []
       (MkTypeLine [creatureType "Elemental"] [Creature])
       [ Macros.keywordCosting "CumulativeUpkeep"
                               (Do (PutCounters (Lit 1) Macros.minusOneMinusOne
                                      Macros.thisCreature)) ]
       (Just (9, 9))

public export
shelteringAncient : Card
shelteringAncient =
  Macros.card "Sheltering Ancient"
       (Just [Macros.generic 1, Macros.pip Green]) []
       (MkTypeLine [creatureType "Treefolk"] [Creature])
       [ Macros.keyword "Trample"
       , Macros.keywordCosting "CumulativeUpkeep"
                               (Do (PutCounters (Lit 1) Macros.plusOnePlusOne
                                      (Macros.a (And [Macros.creature,
                                                      ControlledBy Macros.anOpponent])))) ]
       (Just (5, 5))

public export
polarKraken : Card
polarKraken =
  Macros.card "Polar Kraken"
       (Just [Macros.generic 8, Macros.pip Blue, Macros.pip Blue, Macros.pip Blue]) []
       (MkTypeLine [creatureType "Kraken"] [Creature])
       [ Macros.keyword "Trample"
       , Static (Macros.entersTapped Macros.thisCreature)
       , Macros.keywordCosting "CumulativeUpkeep" (Do (Macros.sacrifice You (Macros.a Macros.land))) ]
       (Just (11, 11))

public export
yavimayaAnts : Card
yavimayaAnts =
  Macros.card "Yavimaya Ants"
       (Just [Macros.generic 2, Macros.pip Green, Macros.pip Green]) []
       (MkTypeLine [creatureType "Insect"] [Creature])
       [ Macros.keyword "Trample"
       , Macros.keyword "Haste"
       , Macros.keywordCosting "CumulativeUpkeep" (Mana [Macros.pip Green, Macros.pip Green]) ]
       (Just (5, 1))

public export
illusionaryForces : Card
illusionaryForces =
  Macros.card "Illusionary Forces"
       (Just [Macros.generic 3, Macros.pip Blue]) []
       (MkTypeLine [creatureType "Illusion"] [Creature])
       [ Macros.keyword "Flying"
       , Macros.keywordCosting "CumulativeUpkeep" (Mana [Macros.pip Blue]) ]
       (Just (4, 4))

public export
vexingSphinx : Card
vexingSphinx =
  Macros.card "Vexing Sphinx"
       (Just [Macros.generic 1, Macros.pip Blue, Macros.pip Blue]) []
       (MkTypeLine [creatureType "Sphinx"] [Creature])
       [ Macros.keyword "Flying"
       , Macros.keywordCosting "CumulativeUpkeep" (Do (Macros.discardsACard You))
       , Macros.triggered When (Dies Macros.thisCreature)
                          (Draw You (CountersOn Age It)) ]
       (Just (4, 4))

public export
manaChains : Card
manaChains =
  Macros.card "Mana Chains" (Just [Macros.pip Blue]) []
       (MkTypeLine [enchantmentType "Aura"] [Enchantment])
       [ Macros.keywordSubject "Enchant" Macros.creature
       , Static (Gains (AttachHost Enchanted (TypeW Creature))
                       (Macros.keywordCosting "CumulativeUpkeep" (Mana [Macros.generic 1]))) ]
       Nothing

public export
coverOfWinterPut : Ability
coverOfWinterPut =
  Macros.activated (Mana [SnowMana])
                   (PutCounters (Lit 1) Age Macros.thisEnchantment)



public export
odricLunarchMarshal : Card
odricLunarchMarshal =
  Macros.card "Odric, Lunarch Marshal"
       (Just [Macros.generic 3, Macros.pip White]) [Legendary]
       (MkTypeLine [creatureType "Human", creatureType "Soldier"] [Creature])
       [ AlsoForKeywords
           (Macros.triggeredIf At
                               (BeginningOf Combat (ByWord EachPlayers))
                               (Exists (And [Macros.creature, ControlledBy You,
                                             HasKeyword "FirstStrike"]))
                               (Continuously
                                  (Gains (AllOf Macros.creatureYouControl)
                                         (Macros.keyword "FirstStrike"))
                                  (Just Macros.untilEndOfTurn)))
           ["Flying", "Deathtouch", "DoubleStrike", "Haste", "Hexproof", "Indestructible",
            "Lifelink", "Menace", "Reach", "Skulk", "Trample", "Vigilance"] ]
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
                                             HasKeyword "Flying"]))
                               (Continuously
                                  (Gains (AllOf Macros.creatureYouControl)
                                         (Macros.keyword "Flying"))
                                  (Just Macros.untilEndOfTurn)))
           ["FirstStrike", "DoubleStrike", "Deathtouch", "Hexproof", "Indestructible",
            "Lifelink", "Menace", "Reach", "Trample", "Vigilance"] ]
       Nothing

public export
urborgScavengers : Card
urborgScavengers =
  Macros.card "Urborg Scavengers"
       (Just [Macros.generic 2, Macros.pip Black]) []
       (MkTypeLine [creatureType "Spirit"] [Creature])
       [ Macros.triggeredOr Whenever
                            (Enters Macros.thisCreature)
                            (Macros.attacks Macros.thisCreature)
                            (Sequentially
                               [ Macros.exile (Macros.target (InZone Macros.graveyardZ))
                               , PutCounters (Lit 1) Macros.plusOnePlusOne Macros.thisCreature ])
       , AlsoForKeywords
           (Static (Macros.asLongAs
                      (Exists (And [ExiledWith Macros.thisCreature, HasKeyword "Flying"]))
                      (Gains Macros.thisCreature (Macros.keyword "Flying"))))
           ["FirstStrike", "DoubleStrike", "Deathtouch", "Haste", "Hexproof", "Indestructible",
            "Lifelink", "Menace", "Reach", "Trample", "Vigilance"] ]
       (Just (2, 2))


public export
sengirVampire : Card
sengirVampire =
  Macros.card "Sengir Vampire"
       (Just [Macros.generic 3, Macros.pip Black, Macros.pip Black]) []
       (MkTypeLine [creatureType "Vampire"] [Creature])
       [ Macros.keyword "Flying"
       , Macros.triggered Whenever
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
       (MkTypeLine [creatureType "Spirit"] [Creature])
       [ Macros.activated (Mana [Macros.pip Black])
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
       (MkTypeLine [creatureType "Elf", creatureType "Druid"] [Creature])
       [ Macros.triggeredIf When
                            (Dies Macros.thisCreature)
                            (Macros.happened BlockedDeclaration It Lookback.ThisTurn)
                            (Macros.gainsLife You (Lit 4)) ]
       (Just (2, 2))

public export
dreamThief : Card
dreamThief =
  Macros.card "Dream Thief" (Just [Macros.generic 2, Macros.pip Blue]) []
       (MkTypeLine [creatureType "Faerie", creatureType "Rogue"] [Creature])
       [ Macros.keyword "Flying"
       , Macros.triggered When (Enters Macros.thisCreature)
           (OnlyIf Macros.drawACard
                          (Macros.happenedInvolving SpellCast You Lookback.ThisTurn
                                             (Macros.a (And [Macros.spell, ColorIs Blue,
                                                             OtherThan This])))
                          Nothing) ]
       (Just (2, 1))

public export
brightspearZealot : Card
brightspearZealot =
  Macros.card "Brightspear Zealot" (Just [Macros.generic 2, Macros.pip White]) []
       (MkTypeLine [creatureType "Human", creatureType "Soldier"] [Creature])
       [ Macros.keyword "Vigilance"
       , Static (Macros.asLongAs
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
       (MkTypeLine [creatureType "Merfolk", creatureType "Wizard"] [Creature])
       [ Macros.keyword "Flash"
       , Macros.triggered When (Enters Macros.thisCreature)
           (SetStatus Untapped
              (Each (And [HasSubtype (creatureType "Merfolk"), ControlledBy You,
                          OtherThan Macros.thisCreature])))
       , Static (Macros.asLongAs
           (Macros.happenedInvolving AttackDeclaration
                                     You
                                     Lookback.ThisTurn
                                     (CountedGroup (Macros.atLeast 3)
                                        (HasSubtype (creatureType "Merfolk"))))
           (Gets (AllOf (And [HasSubtype (creatureType "Merfolk"), ControlledBy You]))
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
       , Macros.activated (Compound [Mana [Macros.generic 8], TapSymbol,
                              Do (Macros.sacrifice You Macros.thisArtifact)])
           (Macros.create (Lit 1) (Macros.creatureTok 10 10 [] [creatureType "Eldrazi"])) ]
       Nothing

public export
mildManneredLibrarian : Card
mildManneredLibrarian =
  Macros.card "Mild-Mannered Librarian" (Just [Macros.pip Green]) []
       (MkTypeLine [creatureType "Human"] [Creature])
       [ Macros.activatedOnlyOnce (Mana [Macros.generic 3, Macros.pip Green])
                                  (Sequentially
                                     [ Continuously (SetsType Macros.thisCreature
                                         (MkToken Nothing [] (Macros.subtypesOnly [creatureType "Werewolf"]) [] Nothing)
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
       [ Macros.triggered Whenever
           (Casts You (Macros.a (And [Macros.spell,
                                      CastFrom (Macros.graveyardOf You)])))
           Macros.drawACard ]
       Nothing

public export
ashZealot : Card
ashZealot =
  Macros.card "Ash Zealot" (Just [Macros.pip Red, Macros.pip Red]) []
       (MkTypeLine [creatureType "Human", creatureType "Warrior"] [Creature])
       [ Macros.keyword "FirstStrike"
       , Macros.keyword "Haste"
       , Macros.triggered Whenever
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
       (MkTypeLine [creatureType "Elemental"] [Creature])
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
       (MkTypeLine [creatureType "Bird", creatureType "Spirit"] [Creature])
       [ Macros.keyword "Flying"
       , Macros.triggered Whenever
           (Casts You (Macros.a (And [Macros.spell,
                                      Not (CastFrom (Macros.handOf You))])))
           Macros.drawACard ]
       (Just (2, 2))

public export
patricianGeist : Card
patricianGeist =
  Macros.card "Patrician Geist" (Just [Macros.generic 2, Macros.pip Blue]) []
       (MkTypeLine [creatureType "Spirit", creatureType "Knight"] [Creature])
       [ Macros.keyword "Flying"
       , Static (Gets (AllOf (And [HasSubtype (creatureType "Spirit"), ControlledBy You,
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
                  [ Macros.move (Macros.target Macros.creature) (Macros.nthFromTop (Nth 2))
                  , Macros.gainsLife (ControllerOf It) (Lit 3) ]) ]
       Nothing

public export
chronostutter : Card
chronostutter =
  Macros.card "Chronostutter" (Just [Macros.generic 5, Macros.pip Blue]) []
       (MkTypeLine [] [Instant])
       [ Spell (Macros.move (Macros.target Macros.creature) (Macros.nthFromTop (Nth 2))) ]
       Nothing

public export
shatteredEgo : Card
shatteredEgo =
  Macros.card "Shattered Ego" (Just [Macros.pip Blue]) []
       (MkTypeLine [enchantmentType "Aura"] [Enchantment])
       [ Macros.keywordSubject "Enchant" Macros.creature
       , Static (Gets (AttachHost Enchanted (TypeW Creature))
                      (PtDown (Lit 3)) (PtDown (Lit 0)))
       , Macros.activated (Mana [Macros.generic 3, Macros.pip Blue, Macros.pip Blue])
                          (Macros.move (AttachHost Enchanted (TypeW Creature))
                                (Macros.nthFromTop (Nth 3))) ]
       Nothing

public export
gandalfWhiteRider : Card
gandalfWhiteRider =
  Macros.card "Gandalf, White Rider" (Just [Macros.generic 3, Macros.pip White])
       [Legendary] (MkTypeLine [creatureType "Avatar", creatureType "Wizard"] [Creature])
       [ Macros.keyword "Vigilance"
       , Macros.triggered Whenever (Casts You (Macros.a Macros.spell))
                          (Sequentially
                      [ Macros.gets (Each Macros.creatureYouControl)
                                    (PtUp (Lit 1)) (PtUp (Lit 0))
                                    (Just Macros.untilEndOfTurn)
                      , Does You "Scry" (Macros.lookAt (Macros.topCards 1)) ])
       , Macros.triggered When (Dies Macros.thisCreature)
                          (Macros.may You (Macros.move It (Macros.nthFromTop (Nth 5)))) ]
       (Just (3, 3))


public export
approachOfTheSecondSun : Card
approachOfTheSecondSun =
  Macros.card "Approach of the Second Sun"
       (Just [Macros.generic 6, Macros.pip White]) [] (MkTypeLine [] [Sorcery])
       [ Spell (If (AndCond
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
kamiOfTerribleSecrets : Card
kamiOfTerribleSecrets =
  Macros.card "Kami of Terrible Secrets" (Just [Macros.generic 3, Macros.pip Black]) []
       (MkTypeLine [creatureType "Spirit"] [Creature])
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
       (MkTypeLine [creatureType "Human", creatureType "Monk"] [Creature])
       [ Static (Macros.asLongAs
                   (AndCond [ Exists (And [Macros.instant, InZone (Macros.graveyardOf You)])
                            , Exists (And [Macros.sorcery, InZone (Macros.graveyardOf You)]) ])
                   (AndAlso [ Gains Macros.thisCreature (Macros.keyword "FirstStrike")
                            , Gains It (Macros.keyword "Trample") ])) ]
       (Just (5, 2))

public export
helmOfPossession : Card
helmOfPossession =
  Macros.card "Helm of Possession" (Just [Macros.generic 4]) []
       (MkTypeLine [] [Artifact])
       [ Static (MayDeclineUntap Macros.thisArtifact)
       , Macros.activated (Compound [Mana [Macros.generic 2], TapSymbol,
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
       (MkTypeLine [creatureType "Spirit"] [Creature])
       [ Macros.keyword "Flying"
       , Macros.triggered At (BeginningOf Upkeep (ByWord Yours))
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
       (MkTypeLine [creatureType "Beast"] [Creature])
       [ Macros.keyword "Trample"
       , Macros.triggered When (Enters Macros.thisCreature)
           (Macros.mayElse You
              (Pay You (ScaledMana (Macros.forEach (InZone (Macros.handOf You)))))
              (Macros.sacrifice You Macros.thisCreature)) ]
       (Just (4, 4))


public export
riseFromTheGrave : Effect []
riseFromTheGrave =
  Sequentially [Macros.putOntoBattlefieldUnderYourControl (Macros.target (And [Macros.creature, InZone (ZoneAt Graveyard Bare)])),
                Macros.becomesAs (That (TypeW Creature))
                                 (MkToken Nothing [Black] (MkTypeLine [creatureType "Zombie"] []) [] Nothing)
                                 Nothing]

public export
dreadSlaver : Card
dreadSlaver =
  Macros.card "Dread Slaver" (Just [Macros.generic 3, Macros.pip Black, Macros.pip Black]) []
       (MkTypeLine [creatureType "Zombie", creatureType "Horror"] [Creature])
       [ Macros.triggered When (Dies (Macros.a (And [Macros.creature,
                                              Macros.happenedToInvolving DamageTaken
                                                                         Lookback.ThisTurn
                                                                         Macros.thisCreature])))
           (Sequentially [Macros.putOntoBattlefieldUnderYourControl It,
                          Macros.becomesAs (That (TypeW Creature))
                                           (MkToken Nothing [Black] (MkTypeLine [creatureType "Zombie"] []) [] Nothing)
                                           Nothing]) ]
       (Just (3, 5))

public export
stormOfSouls : Effect []
stormOfSouls =
  Sequentially [Macros.move (AllOf (And [Macros.creature, InZone (Macros.graveyardOf You)]))
                            Macros.battlefieldZ,
                Macros.becomesAs Them
                                 (MkToken (Just (Lit 1 ** Lit 1)) [] (MkTypeLine [creatureType "Spirit"] [])
                                          [Macros.keyword "Flying"] Nothing)
                                 Nothing,
                Macros.exile This]

public export
everAfter : Effect []
everAfter =
  Sequentially [Macros.move (TargetGroup (Macros.upTo 2) (And [Macros.creature, InZone (Macros.graveyardOf You)]))
                            Macros.battlefieldZ,
                Macros.becomesAs (Those (TypeW Creature))
                                 (MkToken Nothing [Black] (MkTypeLine [creatureType "Zombie"] []) [] Nothing)
                                 Nothing,
                Macros.move This Macros.onBottomZ]

public export
infernalVessel : Card
infernalVessel =
  Macros.card "Infernal Vessel" (Just [Macros.generic 2, Macros.pip Black]) []
       (MkTypeLine [creatureType "Human", creatureType "Cleric"] [Creature])
       [ Macros.triggeredIf When
                            (Dies Macros.thisCreature)
                            (Macros.itIsntA (HasSubtype (creatureType "Demon")))
                            (Sequentially [Macros.returnToBattlefieldWithCounters It
                                                                                  (OwnerOf It)
                                                                                  (Lit 2)
                                                                                  Macros.plusOnePlusOne,
                                           Macros.becomesAs It
                                                            (MkToken Nothing [] (MkTypeLine [creatureType "Demon"] []) [] Nothing)
                                                            Nothing]) ]
       (Just (2, 1))

public export
answeredPrayers : Card
answeredPrayers =
  Macros.card "Answered Prayers" (Just [Macros.generic 1, Macros.pip White, Macros.pip White]) []
       (MkTypeLine [] [Enchantment])
       [ Macros.triggered When (Enters (Macros.a Macros.creatureYouControl))
           (Sequentially [Macros.gainsLife You (Lit 1),
                          If (NotCond (Matches Macros.thisEnchantment Macros.creature))
                             (Macros.becomesAs Macros.thisEnchantment
                                               (MkToken (Just (Lit 3 ** Lit 3)) []
                                                        (MkTypeLine [creatureType "Angel"] [Creature])
                                                        [Macros.keyword "Flying"] Nothing)
                                               (Just Macros.untilEndOfTurn))
                             Nothing]) ]
       Nothing




public export
killingWave : Effect []
killingWave =
  ForEachOf (Each Macros.creature)
            (Macros.mayElse (ControllerOf It)
                            (Pay They (Do (ChangeLife They (Down (LetterVal X)))))
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
                ForEachOf (Macros.thoseVerbed "Exile" (TypeW Creature))
                          (Draw (ControllerOf It) (Lit 1))]

public export
descentOfTheDragons : Effect []
descentOfTheDragons =
  Sequentially [Macros.destroy (TargetGroup Macros.anyNumber Macros.creature),
                ForEachOf (Macros.thoseVerbedThisWay "Destroy" (TypeW Creature))
                          (Create (ControllerOf It) (Lit 1)
                                  (TokenWritten (MkToken (Just (Lit 4 ** Lit 4)) [Red]
                                                         (MkTypeLine [creatureType "Dragon"] [Creature])
                                                         [Macros.keyword "Flying"] Nothing))
                                  [])]

public export
tidalFlats : Card
tidalFlats =
  Macros.card "Tidal Flats" (Just [Macros.pip Blue]) []
       (MkTypeLine [] [Enchantment])
       [ Macros.activated (Mana [Macros.pip Blue, Macros.pip Blue])
           (ForEachOf (Each (And [Macros.creature, Attacking,
                                  Not (HasKeyword "Flying")]))
                      (Macros.mayElse (ControllerOf It)
                                      (Pay They (Mana [Macros.generic 1]))
                                      (Macros.gains
                                         (AllOf (And [Macros.creature, ControlledBy You,
                                                      BlockerOf (That (TypeW Creature))]))
                                         (Macros.keyword "FirstStrike")
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
              , Macros.move (That CardW) Macros.handZ
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
              , If (Macros.itsA Permanent)
                   (Macros.mayThen You (Macros.putOntoBattlefield It)
                                       (Repeat Again))
                   Nothing ]) ]
       Nothing

public export
cultivatorColossus : Card
cultivatorColossus =
  Macros.cardOf "Cultivator Colossus"
       (Just [Macros.generic 4, Macros.pip Green, Macros.pip Green,
              Macros.pip Green]) []
       (MkTypeLine [creatureType "Plant", creatureType "Beast"] [Creature])
       [ Macros.keyword "Trample"
       , Static (DefinesPt Macros.thisCreature BothEach
                           (CountOf (And [Macros.land, ControlledBy You])))
       , Macros.triggered When (Enters Macros.thisCreature)
           (Macros.mayThen You
              (Macros.putOntoBattlefieldTapped
                 (Macros.a (And [Macros.land, InZone (Macros.handOf You)])))
              (Sequentially [Macros.drawACard, Repeat Again])) ]
       (Just (PtBox PrintedStar PrintedStar))

public export
zimoneAndDina : Ability
zimoneAndDina =
  Macros.activated (Compound [TapSymbol,
                       Do (Macros.sacrifice You
                             (Macros.a (Macros.otherCreature Macros.thisCreature)))])
    (Sequentially
       [ Macros.drawACard
       , Macros.may You (Macros.putOntoBattlefieldTapped
                           (Macros.a (And [Macros.land,
                                           InZone (Macros.handOf You)])))
       , If (CompareAmt (CountOf (And [Macros.land, ControlledBy You]))
                        AtLeast (Lit 8))
                   (Repeat (MoreTimes (Lit 1)))
                   Nothing ])

public export
anotherRound : Effect []
anotherRound =
  Sequentially [ Macros.exile (CountedGroup Macros.anyNumber
                                 Macros.creatureYouControl)
               , Macros.putOntoBattlefield Them
               , Repeat (MoreTimes (LetterVal X)) ]

||| "{T}: You gain 1 life. Activate only during your turn, before attackers are declared."
public export
shuFarmer : Card
shuFarmer =
  Macros.card "Shu Farmer" (Just [Macros.generic 1, Macros.pip White]) []
       (MkTypeLine [creatureType "Human"] [Creature])
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
       (MkTypeLine [enchantmentType "Aura"] [Enchantment])
       [ Macros.keywordSubject "Enchant" (HasType Planeswalker)
       , Static (Gains (AttachHost Enchanted (TypeW Planeswalker))
                       (Macros.activated (LoyaltySymbol (LoyaltyUp 1))
                                         (Macros.create (Lit 3)
                                     (Macros.creatureTok 1 1 [White] [creatureType "Soldier"]))))
       , Macros.triggered Whenever
                          (Activates You loyaltyAbilityOfEnchanted)
                          (Continuously
                      (AndAlso [ Gets (AllOf Macros.creatureYouControl)
                                      (PtUp (Lit 2)) (PtUp (Lit 2))
                               , Gains Them (Macros.keyword "Vigilance") ])
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
       (MkTypeLine [enchantmentType "Aura", enchantmentType "Curse"] [Enchantment])
       [ Macros.keywordSubject "Enchant" AnyPlayer
       , Macros.triggered At
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
  InsteadOf (DealDamage This (Lit 2) (Macros.target Macros.anyTarget))
            (OnlyIf (DealDamage This (Lit 4) (Macros.thatJoin))
                    (CompareAmt (CountOf (And [Macros.artifact, ControlledBy You]))
                                AtLeast (Lit 3))
                    Nothing)

public export
cryptLurker : Card
cryptLurker =
  Macros.card "Crypt Lurker" (Just [Macros.generic 3, Macros.pip Black]) []
       (MkTypeLine [creatureType "Horror"] [Creature])
       [ Macros.triggered When (Enters Macros.thisCreature)
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
  Sequentially [ Macros.choose (Macros.a (Macros.qualityFrom CardName
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
              (Macros.nthFromTopOrBottomZ (Nth 2))

||| Lost Hours' third line: "That player puts that card into their library
||| third from the top." The agentive clause over a plain ordinal.
public export
lostHoursPlacement : Effect []
lostHoursPlacement =
  Sequentially [ Macros.revealsTheirHand (Macros.target AnyPlayer)
               , Macros.choose (Macros.a (And [Not (HasType Land),
                                        InZone (Macros.handOf They)]))
               , Macros.puts (That PlayerW) It (Macros.nthFromTop (Nth 3)) ]

||| Aether Gust's second line: "Its owner puts it on their choice of the
||| top or bottom of their library." The agentive clause with the chooser
||| slot filled by the subject.
public export
aetherGustPlacement : Effect []
aetherGustPlacement =
  Sequentially [ Macros.choose (Macros.target (And [Permanent, ColorIs Red]))
               , Macros.puts (OwnerOf It) It (Macros.choiceOfTopOrBottom They) ]

||| Not Forgotten's first line: "Put target card from a graveyard on your
||| choice of the top or bottom of its owner's library." The same
||| disjunction under an imperative, with the chooser slot spelling "your".
public export
notForgottenPlacement : Effect []
notForgottenPlacement =
  Macros.move (Macros.target (InZone Macros.graveyardZ)) (Macros.choiceOfTopOrBottom You)

||| Write into Being's placement half: "put the other on the top or bottom
||| of your library" — the bare disjunction, no chooser named. (Manifest is
||| not in the vocabulary; the exile stands in for the card it consumes.)
public export
writeIntoBeingPlacement : Effect []
writeIntoBeingPlacement =
  Sequentially [ Macros.lookAt (Macros.topCards 2)
               , Macros.exile (Macros.oneOf Them)
               , Macros.move TheRest Macros.topOrBottomZ ]

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
               (PtUp (Macros.nForEach 10 (And [AnyPlayer, Macros.happenedTo GameLoss ThisGame])))
               (PtUp (Macros.nForEach 10 (And [AnyPlayer, Macros.happenedTo GameLoss ThisGame]))))

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
  DealDamage This (Lit 2) (EachOf (TargetGroup (Macros.exactly 2) Macros.anyTarget))

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
               , Macros.move It Macros.onTopZ ]

||| Demonic Tutor: "Search your library for a card, put that card into your
||| hand, then shuffle." The bare description a single-zone search names.
public export
demonicTutor : Effect []
demonicTutor =
  Sequentially [ Macros.searchLibraryFor (And [])
               , Macros.move It Macros.handZ
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
               , Macros.move It Macros.handZ
               , Macros.shuffle ]

||| Brimaz, King of Oreskos' attack trigger: "Whenever Brimaz attacks,
||| create a 1/1 white Cat Soldier creature token with vigilance that's
||| attacking." [CR#508.4] designates the token attacking and taps
||| nothing, so the attacking rider stands alone.
public export
brimazAttackToken : Ability
brimazAttackToken =
  Macros.triggered Whenever (Macros.attacks Macros.thisCreature)
    (Create You (Lit 1)
                   (TokenWritten (MkToken (Just (Lit 1 ** Lit 1)) [White]
                                   (MkTypeLine [creatureType "Cat", creatureType "Soldier"] [Creature])
                                   [Macros.keyword "Vigilance"] Nothing))
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

||| Nahiri, the Unforgiving's compleated reminder -- "this planeswalker
||| enters with two fewer loyalty counters": the entry mark's subtracting
||| arm, over [CR#306.5b]'s printed loyalty count. The reminder's leading
||| "If life was paid" reads back a payment made while casting and has no
||| condition row.
public export
nahiriCompleatedEntry : Ability
nahiriCompleatedEntry =
  Static (Macros.entersWithFewerCounters Macros.thisPlaneswalker (Lit 2)
                                         LoyaltyCounter)

||| Vivien's Talent: "Whenever a nontoken creature you control enters, put
||| a loyalty counter on enchanted planeswalker." [CR#122.1] makes a
||| loyalty counter a marker like any other.
public export
viviensTalentTrigger : Ability
viviensTalentTrigger =
  Macros.triggered Whenever (Enters (Macros.a (And [Macros.nontoken,
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
  Macros.triggered Whenever
    (Macros.manyCounterEvent CounterPut LoyaltyCounter
                             (AllOf (And [HasType Planeswalker, ControlledBy You])))
    (PutCounters ThatMuch Macros.plusOnePlusOne Macros.thisCreature)

||| Void Maw's activated ability: "Put a card exiled with this creature
||| into its owner's graveyard: This creature gets +2/+2 until end of
||| turn." A placement paid as a cost [CR#118.1].
public export
voidMawPutCost : Ability
voidMawPutCost =
  Macros.activated (Do (Macros.puts You (Macros.a (ExiledWith Macros.thisCreature))
                             Macros.graveyardZ))
                   (Macros.gets Macros.thisCreature (PtUp (Lit 2)) (PtUp (Lit 2))
                         (Just Macros.untilEndOfTurn))

||| Bullseye, Death Dealer's activated ability: "{3}, {T}, Sacrifice an
||| artifact or discard a nonland card: Bullseye deals 2 damage to any
||| target." The cost the player chooses between [CR#118.1].
public export
bullseyeModalCost : Ability
bullseyeModalCost =
  Macros.activated (Compound [Mana [Macros.generic 3], TapSymbol,
                       Do (Macros.chooseOne
                             [Macros.sacrifice You (Macros.a Macros.artifact),
                              Macros.discards You
                                (Macros.a (And [Not Macros.land,
                                                InZone Macros.handZ]))])])
                   (DealDamage This (Lit 2) (Macros.target Macros.anyTarget))

||| Sedris, the Traitor King: "Each creature card in your graveyard has
||| unearth {2}{B}." The grant reaches a subject in a graveyard because
||| unearth's own ability functions there [CR#702.84a].
public export
sedrisTheTraitorKing : Ability
sedrisTheTraitorKing =
  Static (Gains (Each (And [Macros.creature, InZone (Macros.graveyardOf You)]))
                (Macros.keywordCosting "Unearth"
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
                (Macros.keywordCosting "Unearth" ItsManaCost))

||| Dralnu, Lich Lord: "Target instant or sorcery card in your graveyard
||| gains flashback until end of turn. The flashback cost is equal to its
||| mana cost." Flashback's graveyard static is named by [CR#702.34a].
public export
dralnuLichLord : Effect []
dralnuLichLord =
  Macros.gains (Macros.target (And [Macros.instantOrSorcery,
                                    InZone (Macros.graveyardOf You)]))
               (Macros.keywordCosting "Flashback" ItsManaCost)
               (Just Macros.untilEndOfTurn)

||| Dregscape Zombie: "Unearth {B}" [CR#702.84a].
public export
dregscapeZombie : Ability
dregscapeZombie = Macros.keywordCosting "Unearth" (Mana [Macros.pip Black])

||| Think Twice: "Flashback {2}{U}" [CR#702.34a].
public export
thinkTwice : Ability
thinkTwice =
  Macros.keywordCosting "Flashback" (Mana [Macros.generic 2, Macros.pip Blue])

||| Greater Mossdog: "Dredge 3" [CR#702.52a].
public export
greaterMossdog : Ability
greaterMossdog = Macros.keywordNumber "Dredge" (Lit 3)

||| Raven's Crime: "Retrace" [CR#702.81a].
public export
ravensCrime : Ability
ravensCrime = Macros.keyword "Retrace"

||| Barkhide Mauler: "Cycling {2}" [CR#702.29a].
public export
barkhideMauler : Ability
barkhideMauler = Macros.keywordCosting "Cycling" (Mana [Macros.generic 2])

||| Ninja of the New Moon: "Ninjutsu {3}{B}" [CR#702.49a].
public export
ninjaOfTheNewMoon : Ability
ninjaOfTheNewMoon =
  Macros.keywordCosting "Ninjutsu" (Mana [Macros.generic 3, Macros.pip Black])

||| Thunderous Wrath: "Miracle {R}" [CR#702.94a].
public export
thunderousWrath : Ability
thunderousWrath = Macros.keywordCosting "Miracle" (Mana [Macros.pip Red])

||| Bygone Colossus: "Warp {3}" [CR#702.185a].
public export
bygoneColossus : Ability
bygoneColossus = Macros.keywordCosting "Warp" (Mana [Macros.generic 3])

||| Nezumi Ronin: "Bushido 1" [CR#702.45a]. The openness witness: bushido
||| joined the vocabulary as one `keywordFacts` row and nothing else -- no
||| constructor, no total-table clause, and no macro either, since
||| `keywordNumber` already writes any word whose rule takes a number.
public export
nezumiRonin : Ability
nezumiRonin = Macros.keywordNumber "Bushido" (Lit 1)

||| Steppe Lynx: "Landfall — Whenever a land you control enters, this
||| creature gets +2/+2 until end of turn." An ability word over a triggered
||| ability [CR#207.2c].
public export
steppeLynx : Ability
steppeLynx =
  AbilityWord Landfall
    (Macros.triggered Whenever (Enters (Macros.a (And [Macros.land, ControlledBy You])))
                      (Macros.gets Macros.thisCreature (PtUp (Lit 2)) (PtUp (Lit 2))
                            (Just Macros.untilEndOfTurn)))

||| Nimble Mongoose: "Threshold — This creature gets +2/+2 as long as there
||| are seven or more cards in your graveyard." An ability word over a
||| static ability [CR#207.2c].
public export
nimbleMongoose : Ability
nimbleMongoose =
  AbilityWord Threshold
    (Static (Macros.onlyWhile (Gets Macros.thisCreature (PtUp (Lit 2)) (PtUp (Lit 2)))
                              (CompareAmt (CountOf (InZone (Macros.graveyardOf You)))
                                          AtLeast (Lit 7))))

||| Ghor-Clan Rampager: "Bloodrush — {R}{G}, Discard this card: Target
||| attacking creature gets +4/+4 and gains trample until end of turn." An
||| ability word over an activated ability [CR#207.2c].
public export
ghorClanRampager : Ability
ghorClanRampager =
  AbilityWord Bloodrush
    (Macros.activated (Compound [Mana [Macros.pip Red, Macros.pip Green],
                          Do (Macros.discards You This)])
                      (Sequentially
                  [Macros.gets (Macros.target (And [Macros.creature, Attacking]))
                               (PtUp (Lit 4)) (PtUp (Lit 4)) (Just Macros.untilEndOfTurn),
                   Macros.gains It (Macros.keyword "Trample") (Just Macros.untilEndOfTurn)]))


||| Balance of Power: "If target opponent has more cards in hand than you,
||| draw cards equal to the difference." The leading condition hands its
||| consequent the margin; the target written inside the condition is
||| announced at casting like any other [CR#601.2c].
public export
balanceOfPower : Effect []
balanceOfPower =
  If (CompareAmt (CountOf (InZone (Macros.handOf (Macros.target Opponent))))
                 Greater
                 (CountOf (InZone (Macros.handOf You))))
     (Draw You TheDifference)
     Nothing

||| Vraska, Betrayal's Sting: "[-9]: If target player has fewer than nine
||| poison counters, they get a number of poison counters equal to the
||| difference." The second leading-condition margin read, with the target
||| inside the condition.
public export
vraskaBetrayalsStingUltimate : Effect []
vraskaBetrayalsStingUltimate =
  If (CompareAmt (CountersOn Poison (Macros.target AnyPlayer)) Less (Lit 9))
     (GetsCounters They TheDifference Poison)
     Nothing

||| Iymrith, Desert Doom's draw line: "Draw a card. Then if you have fewer
||| than three cards in hand, draw cards equal to the difference."
public export
iymrithGapDraw : Effect []
iymrithGapDraw =
  Sequentially [ Macros.drawACard
               , If (CompareAmt (CountOf (InZone (Macros.handOf You)))
                                Less (Lit 3))
                    (Draw You TheDifference)
                    Nothing ]

||| Iymrith, Desert Doom's static: "Iymrith has ward {4} as long as it's
||| untapped." Benched at Dragonlord Ojutai's simpler spelling of the same
||| shape, "has hexproof as long as it's untapped": the condition is written
||| after the statement and pronominalises the statement's own subject, which
||| only the postposed orientation can read.
public export
dragonlordOjutaiHexproof : Ability
dragonlordOjutaiHexproof =
  Static (Macros.onlyWhile (Gains Macros.thisCreature (Macros.keyword "Hexproof"))
                           (Matches It (HasStatus Untapped)))

||| Caustic Bronco's loss line: "You lose life equal to that card's mana
||| value if this creature isn't saddled. Otherwise, each opponent loses that
||| much life." The arm reads the quantity the then-branch wrote, not a deed
||| that happened.
public export
causticBroncoLoss : Effect []
causticBroncoLoss =
  Sequentially [ Macros.revealCards Macros.topCard
               , Macros.move (That CardW) Macros.handZ
               , OnlyIf (Macros.losesLife You (Macros.manaValueOf It))
                        (NotCond (Matches Macros.thisCreature (HasDesignation Saddled)))
                        (Just (Macros.losesLife (Each Opponent) ThatMuch)) ]

||| Gadrak, the Crown-Scourge: "Gadrak can't attack unless you control four
||| or more artifacts." The counted "unless" on the postposed static.
public export
gadrakCantAttack : Ability
gadrakCantAttack =
  Static (Macros.onlyUnless (Deontic Macros.thisCreature Forbid Attack Agent
                                     NoDeonticPatient)
                            (CompareAmt (CountOf (And [Macros.artifact, ControlledBy You]))
                                        AtLeast (Lit 4)))

||| Panglacial Wurm: "While you're searching your library, you may cast this
||| card from your library." A static permission that functions from the
||| library [CR#113.6b], confined to the search action [CR#701.23a].
public export
panglacialWurmCast : Ability
panglacialWurmCast =
  Static (Macros.mayCastFromWhileSearching You This Macros.yourLibrary)


||| Goblin Archaeologist
||| "{R}, {T}: Flip a coin. If you win the flip, destroy target artifact and
||| untap this creature. If you lose the flip, sacrifice this creature."
goblinArchaeologist : Ability
goblinArchaeologist =
  Macros.activated (Compound [Mana [Macros.pip Red], TapSymbol])
    (Sequentially
       [Macros.flipACoin,
        Macros.ifThen Macros.youWinTheFlip
          (Sequentially [Macros.destroy (Macros.target Macros.artifact),
                         SetStatus Untapped Macros.thisCreature]),
        Macros.ifThen Macros.youLoseTheFlip
          (Macros.sacrifice You Macros.thisCreature)])

||| Contraband Livestock
||| "Exile target creature, then roll a d20.
|||  1—9 | Its controller creates a 4/4 green Ox creature token.
|||  10—19 | Its controller creates a 2/2 green Boar creature token.
|||  20 | Its controller creates a 0/1 white Goat creature token."
contrabandLivestock : Effect []
contrabandLivestock =
  Sequentially
    [Macros.exile (Macros.target Macros.creature),
     Macros.rollADie 20,
     Macros.resultsTable
       [Macros.rollRow (Macros.fromTo 1 9)
          (Create (ControllerOf It) (Lit 1)
                  (TokenWritten (Macros.creatureTok 4 4 [Green] [creatureType "Ox"])) []),
        Macros.rollRow (Macros.fromTo 10 19)
          (Create (ControllerOf It) (Lit 1)
                  (TokenWritten (Macros.creatureTok 2 2 [Green] [creatureType "Boar"])) []),
        Macros.rollRow (Macros.exactly 20)
          (Create (ControllerOf It) (Lit 1)
                  (TokenWritten (Macros.creatureTok 0 1 [White] [creatureType "Goat"])) [])]]

||| Hypnotic Specter, second line's body: "…, that player discards a card
||| at random." The header is NOT written: "this creature deals damage to
||| an opponent" is a source-side, non-combat damage event, and `GameEvent`
||| carries only the recipient-side `IsDealtDamage` and the combat-only
||| `DealsCombatDamage`. The at-random half is what this round owed.
hypnoticSpecterDiscard : Effect [MkBinding TheD Player OneOf PlayerP]
hypnoticSpecterDiscard = Macros.discardsACardAtRandom They


||| Chance Encounter
||| "Whenever you win a coin flip, put a luck counter on this enchantment.
|||  At the beginning of your upkeep, if this enchantment has ten or more
|||  luck counters on it, you win the game."
public export
chanceEncounter : Card
chanceEncounter =
  Macros.card "Chance Encounter"
       (Just [Macros.generic 2, Macros.pip Red, Macros.pip Red]) []
       (MkTypeLine [] [Enchantment])
       [ Macros.triggered Whenever Macros.youWinACoinFlip
                          (PutCounters (Lit 1) Luck Macros.thisEnchantment)
       , Macros.triggeredIf At
                            (BeginningOf Upkeep (ByWord Yours))
                            (CompareAmt (CountersOn Luck Macros.thisEnchantment)
                                        AtLeast (Lit 10))
                            (Concludes WinGame You) ]
       Nothing

||| Karplusan Minotaur's win arm: "Whenever you win a coin flip, this
||| creature deals 1 damage to any target." Its lose arm writes "any target
||| of an opponent's choice", which no mention shape carries -- the same
||| chooser gap the at-random round ledgered, and outside the event row.
||| Its cumulative upkeep belongs to the cost-and-payment ticket.
public export
karplusanMinotaurWinFlip : Ability
karplusanMinotaurWinFlip =
  Macros.triggered Whenever Macros.youWinACoinFlip
                   (DealDamage Macros.thisCreature (Lit 1)
                               (Macros.target Macros.anyTarget))

||| Brazen Dwarf
||| "Whenever you roll one or more dice, this creature deals 1 damage to
||| each opponent."
public export
brazenDwarf : Card
brazenDwarf =
  Macros.card "Brazen Dwarf"
       (Just [Macros.generic 1, Macros.pip Red]) []
       (MkTypeLine [creatureType "Dwarf", creatureType "Shaman"] [Creature])
       [ Macros.triggered Whenever Macros.youRollDice
                          (DealDamage Macros.thisCreature (Lit 1)
                                      (Each Opponent)) ]
       (Just (1, 3))

||| Vexing Puzzlebox, first line: "Whenever you roll one or more dice, put a
||| number of charge counters on this artifact equal to the result." The
||| body reads the number the event announced [CR#706.2].
public export
vexingPuzzleboxCounters : Ability
vexingPuzzleboxCounters =
  Macros.triggered Whenever Macros.youRollDice
                   (PutCounters Macros.theResult Charge Macros.thisArtifact)

||| The Space Family Goblinson, first line: "Whenever you roll a die, put
||| a +1/+1 counter on The Space Family Goblinson." The singular
||| determiner beside Brazen Dwarf's plural one.
public export
spaceFamilyGoblinsonRoll : Ability
spaceFamilyGoblinsonRoll =
  Macros.triggered Whenever Macros.youRollADie
                   (PutCounters (Lit 1) Macros.plusOnePlusOne
                                Macros.thisCreature)

||| Ral Zarek's ultimate: "Flip five coins. Take an extra turn after this
||| one for each coin that comes up heads." The count over the flipped
||| coins, read off the flip the clause before it wrote.
public export
ralZarekUltimate : Effect []
ralZarekUltimate =
  Sequentially [Macros.flipCoins 5,
                ExtraTurn You (Macros.coinsThatCameUp Heads)]

||| Spark Fiend's upkeep roll: "roll two six-sided dice. If you rolled 7,
||| sacrifice this creature." The total of the two dice, which no per-roll
||| result gives [CR#706.2]. The line's remaining clauses read a total the
||| card NOTED on itself, which is not written.
public export
sparkFiendUpkeepRoll : Effect []
sparkFiendUpkeepRoll =
  Sequentially [Macros.rollDice 2 6,
                Macros.ifThen (CompareAmt Macros.theTotal Eq (Lit 7))
                              (Macros.sacrifice You Macros.thisCreature)]


||| Berserker's Frenzy's roll: "Roll two d20 and ignore the lower roll."
||| The ignore instruction written as an instruction [CR#706.6], beside
||| the replacement lines that write the same word under a "would".
||| "The lower roll" is the two-die spelling of the lowest.
public export
berserkersFrenzyRoll : Effect []
berserkersFrenzyRoll =
  Sequentially [Macros.rollDice 2 20, IgnoreRolls (IgnoreExtreme LowestRoll)]

||| Iron Mastiff's ignore, benched over a bare roll: "…and ignore all but
||| the highest roll." The card rolls "a d20 for each player being
||| attacked", a count over the players an attack names, which no player
||| description carries.
public export
ironMastiffIgnore : Effect []
ironMastiffIgnore =
  Sequentially [Macros.rollADie 20, IgnoreRolls (IgnoreAllBut HighestRoll)]

||| Xenosquirrels' modifier, benched over a roll of its own: "…increase
||| or decrease the result by 1." [CR#706.2]'s modifier "from other
||| sources". The card writes it under an "After you roll a die" header,
||| and [CR#603.1] writes a triggered ability's word as
||| "[When/Whenever/At]", which is the whole of `TriggerWord`.
public export
xenosquirrelsShift : Effect []
xenosquirrelsShift =
  Sequentially [Macros.rollADie 6, ShiftResult (Lit 1)]

||| Atomwheel Acrobats, first line: "Whenever you roll a 1 or 2, put that
||| many +1/+1 counters on this creature." The two-ended result test
||| [CR#706.3a], and the body reading the result back as "that many".
public export
atomwheelAcrobatsRoll : Ability
atomwheelAcrobatsRoll =
  Macros.triggered Whenever (Macros.youRollResultIn (Range (Just 1) (Just 2)))
                   (PutCounters ThatMuch Macros.plusOnePlusOne
                                Macros.thisCreature)

||| Monoxa, Midway Manager, first line: "Whenever you roll a 3 or higher,
||| Monoxa gains first strike until end of turn. If the roll was 4 or
||| higher, it gains menace until end of turn. If the roll was 5 or
||| higher, it gains lifelink until end of turn." The one-ended test on
||| the header, and "the roll" read back off it as the result [CR#706.2].
public export
monoxaRollTrigger : Ability
monoxaRollTrigger =
  Macros.triggered Whenever (Macros.youRollResultIn (Macros.orHigher 3))
    (Sequentially
      [ Macros.gains Macros.thisCreature (KeywordAbility "FirstStrike" Nothing)
                     (Just Macros.untilEndOfTurn)
      , Macros.ifThen (CompareAmt Macros.theResult AtLeast (Lit 4))
                      (Macros.gains Macros.thisCreature
                                    (KeywordAbility "Menace" Nothing)
                                    (Just Macros.untilEndOfTurn))
      , Macros.ifThen (CompareAmt Macros.theResult AtLeast (Lit 5))
                      (Macros.gains Macros.thisCreature
                                    (KeywordAbility "Lifelink" Nothing)
                                    (Just Macros.untilEndOfTurn)) ])

||| Fractured Powerstone, second line: "{T}: Roll the planar die.
||| Activate only as a sorcery." The planar die's instruction row
||| [CR#901.3a]; it announces no number, since [CR#706.7] has every
||| numerical read ignore the planar roll.
public export
fracturedPowerstonePlanarRoll : Ability
fracturedPowerstonePlanarRoll =
  Macros.activatedOnlyDuring TapSymbol (RollPlanarDie You) AsSorcery

||| Farideh, Devil's Chosen, her roll trigger's second sentence: "If any
||| of those results was 10 or higher, draw a card." The existential over
||| one clause's rolls [CR#706.2]. The line's first sentence grants two
||| keywords at once, which is a coordination of grants and not this
||| ticket's.
public export
faridehResultRead : Ability
faridehResultRead =
  Macros.triggered Whenever Macros.youRollDice
                   (Macros.ifThen (AnyResultIs AtLeast (Lit 10))
                                  (Draw You (Lit 1)))

||| Celebr-8000's doubles clause: "roll two six-sided dice. … If you
||| rolled doubles, it also gains double strike until end of turn."
||| [CR#706.5] defines the phrase for this card by name.
public export
celebr8000Doubles : Effect []
celebr8000Doubles =
  Sequentially [ Macros.rollDice 2 6
               , Macros.ifThen RolledDoubles
                   (Macros.gains Macros.thisCreature
                                 (KeywordAbility "DoubleStrike" Nothing)
                                 (Just Macros.untilEndOfTurn)) ]

||| Goblin Assassin's second sentence: "each player flips a coin. Each
||| player whose coin comes up tails sacrifices a creature." The uncalled
||| face read [CR#705.2] narrowing a described set, where `FlipFace`
||| reads the one coin a clause flipped. The card's "of their choice"
||| is not written here: the sentence mentions players twice -- once
||| flipping, once narrowed by the face -- and `TheirChoice` presupposes
||| a single chooser mention, the pre-existing chooser-mention gap.
public export
goblinAssassinCoinTails : Effect []
goblinAssassinCoinTails =
  Sequentially [ FlipCoins (Each AnyPlayer) (Lit 1)
               , Macros.sacrifice (Each (And [AnyPlayer, CoinCameUp Tails]))
                                  (Macros.a Macros.creature) ]

||| Centaur of Attention
||| "When this creature enters, roll five six-sided dice and store those
|||  results on it.
|||  At the beginning of combat on your turn, you may reroll any number of
|||  this creature's stored results.
|||  This creature gets +X/+X, where X is the greatest number of stored
|||  results on it of the same value."
||| The card [CR#706.8] is written for, whole.
public export
centaurOfAttention : Card
centaurOfAttention =
  Macros.card "Centaur of Attention"
       (Just [Macros.generic 2, Macros.pip Green, Macros.pip Green]) []
       (MkTypeLine [creatureType "Centaur", creatureType "Advisor"] [Creature])
       [ Macros.triggered When (Enters Macros.thisCreature)
                          (Sequentially [ Macros.rollDice 5 6
                                        , StoreResults Macros.thisCreature ])
       , Macros.triggered At (BeginningOf Combat (ByWord Yours))
                          (Macros.may You
                             (RerollStored You Macros.anyNumber
                                           Macros.thisCreature))
       , Static (AndAlso [ Gets Macros.thisCreature
                                (PtUp (LetterVal X)) (PtUp (LetterVal X))
                         , Define X
                             (GreatestStoredMatch Macros.thisCreature) ]) ]
       (Just (0, 0))


||| Wax // Wane, a split card [CR#709.1]: two faces on one card, each with its
||| own mana cost [CR#709.4b] and its own type line and text box [CR#709.4c].
waxWane : Card
waxWane =
  SplitCard
    (MkFace "Wax" (Just [Macros.pip Green]) [] (MkTypeLine [] [Instant])
            [ Spell (Macros.gets (Macros.target Macros.creature)
                                 (PtUp (Lit 2)) (PtUp (Lit 2))
                                 (Just Macros.untilEndOfTurn)) ]
            Nothing)
    (MkFace "Wane" (Just [Macros.pip White]) [] (MkTypeLine [] [Instant])
            [ Spell (Macros.destroy (Macros.target Macros.enchantment)) ]
            Nothing)

||| Branchloft Pathway // Boulderloft Pathway, a modal double-faced card
||| [CR#712.3]: the two faces are independent, and a player playing it as a
||| land chooses which of them enters [CR#712.12].
branchloftPathway : Card
branchloftPathway =
  ModalDfc
    (MkFace "Branchloft Pathway" Nothing [] (MkTypeLine [] [Land])
            [ Macros.activated TapSymbol
                               (AddMana You (Lit 1) (Runs [[OfColor Green]]) []) ]
            Nothing)
    (MkFace "Boulderloft Pathway" Nothing [] (MkTypeLine [] [Land])
            [ Macros.activated TapSymbol
                               (AddMana You (Lit 1) (Runs [[OfColor White]]) []) ]
            Nothing)

||| Merfolk Secretkeeper // Venture Deeper, an adventurer card [CR#715.1]: the
||| normal face, and the inset frame whose alternative characteristics the
||| object has while it's a spell [CR#715.2].
merfolkSecretkeeper : Card
merfolkSecretkeeper =
  Adventurer
    (MkFace "Merfolk Secretkeeper" (Just [Macros.pip Blue]) []
            (MkTypeLine [creatureType "Merfolk", creatureType "Wizard"] [Creature]) []
            (Macros.printedBox (Just (0, 4))))
    (MkFace "Venture Deeper" (Just [Macros.pip Blue]) []
            (MkTypeLine [spellType "Adventure"] [Sorcery])
            [ Spell (Macros.mills (Macros.target AnyPlayer) (Lit 4) They) ]
            Nothing)

||| Orochi Eggwatcher // Shidako, Broodmistress, a flip card [CR#710.1]. The
||| activated ability's own words turn it over, spelled with the landed
||| `Flipped` status; the alternative half writes no mana cost of its own
||| [CR#710.1c].
orochiEggwatcher : Card
orochiEggwatcher =
  FlipCard
    (MkFace "Orochi Eggwatcher" (Just [Macros.generic 2, Macros.pip Green]) []
            (MkTypeLine [creatureType "Snake", creatureType "Shaman"] [Creature])
            [ Macros.activated
                (Compound [Mana [Macros.generic 2, Macros.pip Green], TapSymbol])
                (Sequentially
                   [ Macros.create (Lit 1) (Macros.creatureTok 1 1 [Green] [creatureType "Snake"])
                   , Macros.ifThen (CompareAmt (CountOf Macros.creatureYouControl)
                                               AtLeast (Lit 10))
                                   (SetStatus Flipped Macros.thisCreature) ]) ]
            (Macros.printedBox (Just (1, 1))))
    (MkAltFace "Shidako, Broodmistress" [Legendary]
               (MkTypeLine [creatureType "Snake", creatureType "Shaman"] [Creature])
               [ Macros.activated
                   (Compound [ Mana [Macros.pip Green]
                             , Do (Macros.sacrifice You (Macros.a Macros.creature)) ])
                   (Macros.gets (Macros.target Macros.creature)
                                (PtUp (Lit 3)) (PtUp (Lit 3))
                                (Just Macros.untilEndOfTurn)) ]
               (Macros.printedBox (Just (3, 3))))


||| A planeswalker back face with no loyalty number, probed at the law rather
||| than benched as a card. Arlinn, Embraced by the Moon and Garruk, the
||| Veil-Cursed both print a legendary planeswalker back with the box empty,
||| while Jace, Telepath Unbound prints one on the same kind of face: [CR#209.1]
||| puts the number on each planeswalker *card*, and [CR#712.8a] reads a
||| double-faced card's characteristics off its front face off the battlefield,
||| so the back's box is optional and `AltCardBox` — not `CardBox` — is what a
||| costless face answers. Both whole cards wait on a transform verb (Ledger),
||| so the evidence is the law, not an entry.
public export
planeswalkerBackWithoutLoyalty : AltFace
planeswalkerBackWithoutLoyalty =
  MkAltFace "" [Legendary] (MkTypeLine [planeswalkerType "Arlinn"] [Planeswalker]) [] Nothing

public export
planeswalkerBackWithoutLoyaltyOk : AltFaceLaws Cards.planeswalkerBackWithoutLoyalty
planeswalkerBackWithoutLoyaltyOk = MkAltFaceLaws


-- ---------------------------------------------------------------------------
-- The amount reads, the open comparison left side, and the two folds.
-- ---------------------------------------------------------------------------

||| Karametra's Acolyte -- "{T}: Add an amount of {G} equal to your devotion
||| to green."
public export
karametrasAcolyte : Ability
karametrasAcolyte =
  Macros.activated TapSymbol
    (AddMana You (Devotion You Green Nothing) (Runs [[OfColor Green]]) [])

||| Anax, Hardened in the Forge -- "Anax's power is equal to your devotion
||| to red." The devotion read at the definition frame; the box is */3.
public export
anaxPowerDefinition : Ability
anaxPowerDefinition =
  Static (DefinesPt Macros.thisCreature PowerAlone (Devotion You Red Nothing))

||| Gray Merchant of Asphodel, first clause -- "each opponent loses X life,
||| where X is your devotion to black." (The second sentence, "You gain life
||| equal to the life lost this way.", is not taken here.)
public export
grayMerchantDrain : Effect []
grayMerchantDrain =
  Sequentially [ Macros.losesLife (Each Opponent) (LetterVal X)
               , Define X (Devotion You Black Nothing) ]

||| Erebos, God of the Dead -- "As long as your devotion to black is less
||| than five, Erebos isn't a creature." The devotion read at the comparison
||| frame.
public export
devotionCondition : Condition []
devotionCondition = CompareAmt (Devotion You Black Nothing) Less (Lit 5)

||| Aspect of Wolf -- "Enchanted creature gets +X/+Y, where X is half the
||| number of Forests you control, rounded down, and Y is half the number of
||| Forests you control, rounded up."
public export
aspectOfWolf : Ability
aspectOfWolf =
  Static (AndAlso
    [ Gets (AttachHost Enchanted (TypeW Creature))
           (PtUp (LetterVal X)) (PtUp (LetterVal Y))
    , Define X (Half RoundDown
                 (CountOf (And [HasSubtype (landType "Forest"), ControlledBy You])))
    , Define Y (Half RoundUp
                 (CountOf (And [HasSubtype (landType "Forest"), ControlledBy You]))) ])

||| Jaws of Defeat -- "Whenever a creature you control enters, target
||| opponent loses life equal to the difference between that creature's
||| power and its toughness." The symmetric margin beside the directional
||| `Minus`.
public export
jawsOfDefeat : Ability
jawsOfDefeat =
  Macros.triggered Whenever (Enters (Macros.a Macros.creatureYouControl))
    (Macros.losesLife (Macros.target Opponent)
       (DifferenceBetween (Macros.powerOf (That (TypeW Creature)))
                          (Macros.toughnessOf (That (TypeW Creature)))))

||| Defiling Daemogoth -- "At the beginning of your end step, each opponent
||| loses X life, where X is the amount of life you gained this turn." The
||| summed lookback, `EventCount`'s numeric twin.
public export
defilingDaemogothDrain : Effect []
defilingDaemogothDrain =
  Sequentially [ Macros.losesLife (Each Opponent) (LetterVal X)
               , Define X (EventSum LifeGain You Lookback.ThisTurn Nothing) ]

||| The Skullspore Nexus -- "Whenever one or more nontoken creatures you
||| control die, create a green Fungus Dinosaur creature token with base
||| power and toughness each equal to the total power of those creatures."
||| The fold whose complement is a group MENTION.
||| -- spelling: the card writes "those creatures"; `eventAfter (Dies …)`
||| moves the mention to the graveyard, so the mention this term reads back
||| is the graveyard-side `Those CardW`. The card-side spelling of a death's
||| own mention is a standing gap, not this row's.
public export
skullsporeNexusTrigger : Ability
skullsporeNexusTrigger =
  Macros.triggered Whenever
    (Dies (CountedGroup (Macros.atLeast 1)
                        (And [Macros.nontoken, Macros.creatureYouControl])))
    (Macros.create (Lit 1)
       (Macros.creatureTokOf
          (AggregateOf SumOf (CharAxis Power) (Those CardW))
          (AggregateOf SumOf (CharAxis Power) (Those CardW))
          [Green] [creatureType "Fungus", creatureType "Dinosaur"]))

||| Investigator's Journal's count -- "the greatest number of creatures a
||| player controls". The element-binder fold's measured phrase; the whole
||| card waits on a suspect counter-kind row.
public export
greatestCreaturesAPlayerControls : Amount []
greatestCreaturesAPlayerControls =
  AggregateOver MaxOf AnyPlayer
    (CountOf (And [Macros.creature, ControlledBy They]))

||| Investigator's Journal, whole -- the element-binder fold as an entry
||| count, with the suspect counters it stores.
investigatorsJournal : Card
investigatorsJournal =
  Macros.card "Investigator's Journal" (Just [Macros.generic 2]) []
       (MkTypeLine [artifactType "Book", artifactType "Clue"] [Artifact])
       [ Static (Macros.entersWithCounters Macros.thisArtifact
                   greatestCreaturesAPlayerControls Suspect)
       , Macros.activated (Compound [Mana [Macros.generic 2], TapSymbol,
                              Do (RemoveCounters (Lit 1) (Just Suspect)
                                    Macros.thisArtifact)])
                          Macros.drawACard
       , Macros.activated (Compound [Mana [Macros.generic 2],
                              Do (Macros.sacrifice You Macros.thisArtifact)])
                          Macros.drawACard ]
       Nothing

||| Cavern-Hoard Dragon's cost rider -- "This spell costs {X} less to cast,
||| where X is the greatest number of artifacts an opponent controls." The
||| same binder over a narrowed domain.
public export
greatestArtifactsAnOpponentControls : Amount []
greatestArtifactsAnOpponentControls =
  AggregateOver MaxOf Opponent
    (CountOf (And [Macros.artifact, ControlledBy They]))

||| Lhurgoyf -- "Lhurgoyf's power is equal to the number of creature cards in
||| all graveyards and its toughness is equal to that number plus 1." The
||| asymmetric definition as a telescope: the first slot names a number and
||| the second reads it back.
||| -- spelling: the bare graveyard zone prints "in all graveyards" here; the
||| box is */1+*.
public export
lhurgoyfDefinition : Ability
lhurgoyfDefinition =
  Static (AndAlso
    [ DefinesPt Macros.thisCreature PowerAlone
        (CountOf (And [Macros.creature, InZone Macros.graveyardZ]))
    , DefinesPt Macros.thisCreature ToughnessAlone
        (Plus ThatMuch (Lit 1)) ])

||| Shapeshifter's printed box -- "*/7-*", the subtracted star at the
||| toughness slot.
public export
shapeshifterBox : PrintedBox
shapeshifterBox = PtBox PrintedStar (PrintedMinusStar 7)

||| Multiple Choice, first arm -- "If X is 1, scry 1, then draw a card." The
||| announced letter on a comparison's left, at the equality.
public export
multipleChoiceFirstArm : Effect []
multipleChoiceFirstArm =
  If (CompareAmt (LetterVal X) Eq (Lit 1))
     (Sequentially [ Does You "Scry" (Macros.lookAt (Macros.topCards 1))
                   , Macros.drawACard ])
     Nothing

||| Multiple Choice, fourth arm -- "If X is 4 or more, do all of the above."
||| The same left side at the ranged relation.
public export
multipleChoiceFourthGate : Condition []
multipleChoiceFourthGate = CompareAmt (LetterVal X) AtLeast (Lit 4)

||| Fell the Mighty -- "Destroy all creatures with power greater than target
||| creature's power." The announcing bound in the postnominal frame,
||| threaded by `predDelta`.
public export
fellTheMighty : Effect []
fellTheMighty =
  Macros.destroy
    (AllOf (And [Macros.creature,
                 Compare Power Greater
                         (Macros.powerOf (Macros.target Macros.creature))]))

||| Birthing Pod -- "{1}{G/P}, {T}, Sacrifice a creature: Search your library
||| for a creature card with mana value equal to 1 plus the sacrificed
||| creature's mana value, …" The summed bound at the open bound column.
public export
birthingPodSearch : Ability
birthingPodSearch =
  Macros.activated
    (Compound [Mana [Macros.generic 1, Macros.phyrexianPip Green],
               TapSymbol,
               Do (Macros.sacrifice You (Macros.a Macros.creature))])
    (Macros.searchLibraryFor
       (And [Macros.creature,
             Compare ManaValue Eq
                     (Plus (Lit 1)
                           (Macros.manaValueOf
                              (Macros.theVerbed "Sacrifice"
                                                (TypeW Creature))))]))


-- ---------------------------------------------------------------------------
-- The ordinal occurrence word and the amount-ceilinged quantity.
-- ---------------------------------------------------------------------------

||| Wavebreak Hippocamp -- "Whenever you cast your first spell during each
||| opponent's turn, draw a card." The ordinal at the cast restriction,
||| under the window that landed without it.
public export
wavebreakHippocamp : Ability
wavebreakHippocamp =
  Macros.triggeredOnlyDuring Whenever
    (NthOccurrence (Nth 1) (Casts You (Macros.a Macros.spell)))
    (DuringWindow Turn (Just EachOpponents))
    Macros.drawACard

||| Midnight Clock's header -- "When the twelfth hour counter is put on this
||| artifact, …" (the body shuffles hand and graveyard into the library, which
||| is unbuilt, so the header is the witness).
public export
midnightClockHeader : GameEvent []
midnightClockHeader =
  NthOccurrence (Nth 12)
    (Macros.singleCounterEvent CounterPut Hour Macros.thisArtifact)

||| Political Triumph -- "When the fourth plan counter is put on this
||| enchantment, sacrifice it, draw a card, and put a +1/+1 counter on each
||| creature you control." The plan-counter Saga family's header.
public export
politicalTriumphHeader : GameEvent []
politicalTriumphHeader =
  NthOccurrence (Nth 4)
    (Macros.singleCounterEvent CounterPut Plan Macros.thisEnchantment)

||| Run the Play (Striding Shotcaller's other half), first clause -- "Put a
||| +1/+1 counter on each of up to X target creatures." The amount ceiling on
||| a target group's quantity.
public export
runThePlayCounters : Effect []
runThePlayCounters =
  PutCounters (Lit 1) Macros.plusOnePlusOne
              (EachOf (TargetGroup (UpToOf (LetterVal X)) Macros.creature))

||| Berserker's Frenzy, the 1—14 striation -- "Choose any number of creatures.
||| They block this turn if able." The counted choice the ticket lists as
||| over-refused; `choosable (CountedGroup _ _)` already admits it.
public export
berserkersFrenzyLowRoll : Effect []
berserkersFrenzyLowRoll =
  Sequentially
    [ Macros.choose (CountedGroup Macros.anyNumber Macros.creature)
    , Continuously (Deontic Them Require Block Agent NoDeonticPatient)
                   (Just Macros.thisTurn) ]

-- The split-determiner union read. A union mention is read back either
-- whole, by `thatJoin`, or one half at a time by the split read below; the
-- two spellings share one antecedent, and Searing Blaze writes both.

||| "target player or planeswalker": the union head the split read is
||| written over, and the context its noun witnesses are read in.
public export
targetPlayerOrPlaneswalker : Noun bs (Object \/ Player)
targetPlayerOrPlaneswalker =
  Macros.target (Macros.kindJoin AnyPlayer (HasType Planeswalker))

||| "target opponent or planeswalker": the same head with the player half
||| described, which the split read's player arm does not echo.
public export
targetOpponentOrPlaneswalker : Noun bs (Object \/ Player)
targetOpponentOrPlaneswalker =
  Macros.target (Macros.kindJoin Opponent (HasType Planeswalker))

||| "each creature that player or that planeswalker's controller controls":
||| the split read in the possessor slot of a description.
public export
eachCreatureThatSplitControls :
  {bs : Bindings} ->
  {auto 0 ck : countWord (TypeW Planeswalker) bs = 1} ->
  {auto 0 pk : countWord PlayerW bs = 1} ->
  Noun bs Object
eachCreatureThatSplitControls =
  Each (And [Macros.creature, ControlledBy (Macros.splitOverPlaneswalker {ck} {pk})])

||| Lavalanche -- "deals X damage to target player or planeswalker and each
||| creature that player or that planeswalker's controller controls."
public export
lavalanche : Effect []
lavalanche =
  Simultaneously
    [ DealDamage This (LetterVal X) Cards.targetPlayerOrPlaneswalker
    , DealDamage This (LetterVal X) Cards.eachCreatureThatSplitControls ]

||| Flame Wave -- "deals 4 damage to target player or planeswalker and each
||| creature that player or that planeswalker's controller controls."
public export
flameWave : Effect []
flameWave =
  Simultaneously
    [ DealDamage This (Lit 4) Cards.targetPlayerOrPlaneswalker
    , DealDamage This (Lit 4) Cards.eachCreatureThatSplitControls ]

||| Chandra Nalaar's ultimate -- "deals 10 damage to target player or
||| planeswalker and each creature that player or that planeswalker's
||| controller controls."
public export
chandraNalaarUltimate : Effect []
chandraNalaarUltimate =
  Simultaneously
    [ DealDamage This (Lit 10) Cards.targetPlayerOrPlaneswalker
    , DealDamage This (Lit 10) Cards.eachCreatureThatSplitControls ]

||| Chandra, Pyrogenius's ultimate -- "deals 6 damage to target player or
||| planeswalker and each creature that player or that planeswalker's
||| controller controls."
public export
chandraPyrogeniusUltimate : Effect []
chandraPyrogeniusUltimate =
  Simultaneously
    [ DealDamage This (Lit 6) Cards.targetPlayerOrPlaneswalker
    , DealDamage This (Lit 6) Cards.eachCreatureThatSplitControls ]

||| Bonfire of the Damned -- "deals X damage to target player or
||| planeswalker and each creature that player or that planeswalker's
||| controller controls." The miracle cost is the card's, not this clause's.
public export
bonfireOfTheDamned : Effect []
bonfireOfTheDamned =
  Simultaneously
    [ DealDamage This (LetterVal X) Cards.targetPlayerOrPlaneswalker
    , DealDamage This (LetterVal X) Cards.eachCreatureThatSplitControls ]

||| Chandra's Fury -- "deals 4 damage to target player or planeswalker and 1
||| damage to each creature that player or that planeswalker's controller
||| controls." The two halves of the union take different amounts.
public export
chandrasFury : Effect []
chandrasFury =
  Simultaneously
    [ DealDamage This (Lit 4) Cards.targetPlayerOrPlaneswalker
    , DealDamage This (Lit 1) Cards.eachCreatureThatSplitControls ]

||| Heart of Bogardan's cumulative-upkeep trigger body -- "deals X damage to
||| target player or planeswalker and each creature that player or that
||| planeswalker's controller controls, where X is twice the number of age
||| counters on this enchantment minus 2."
public export
heartOfBogardanBody : Effect []
heartOfBogardanBody =
  Sequentially
    [ Simultaneously
        [ DealDamage This (LetterVal X) Cards.targetPlayerOrPlaneswalker
        , DealDamage This (LetterVal X) Cards.eachCreatureThatSplitControls ]
    , Define X (Minus (Times 2 (CountersOn Age Macros.thisEnchantment)) (Lit 2)) ]

||| Angrath, Minotaur Pirate's plus -- "Angrath deals 1 damage to target
||| opponent or planeswalker and each creature that player or that
||| planeswalker's controller controls." The player arm writes "player" over
||| an antecedent that described the half as an opponent.
public export
angrathMinotaurPirateBolt : Effect []
angrathMinotaurPirateBolt =
  Simultaneously
    [ DealDamage This (Lit 1) Cards.targetOpponentOrPlaneswalker
    , DealDamage This (Lit 1) Cards.eachCreatureThatSplitControls ]

||| Which of You Burns Brightest?'s body -- "this scheme deals X damage to
||| target opponent or planeswalker and each creature that player or that
||| planeswalker's controller controls." The {X} offer is the scheme
||| trigger's, not this clause's.
public export
whichOfYouBurnsBrightestBody : Effect []
whichOfYouBurnsBrightestBody =
  Simultaneously
    [ DealDamage This (LetterVal X) Cards.targetOpponentOrPlaneswalker
    , DealDamage This (LetterVal X) Cards.eachCreatureThatSplitControls ]

||| Chandra, Pyromaster's plus -- "deals 1 damage to target player or
||| planeswalker and 1 damage to up to one target creature that player or
||| that planeswalker's controller controls." The split read describes a
||| SECOND target rather than a group.
public export
chandraPyromasterBolt : Effect []
chandraPyromasterBolt =
  Simultaneously
    [ DealDamage This (Lit 1) Cards.targetPlayerOrPlaneswalker
    , DealDamage This (Lit 1)
        (TargetGroup (Macros.upTo 1)
                     (And [Macros.creature,
                           ControlledBy Macros.splitOverPlaneswalker])) ]

||| Ravager of the Fells's transform trigger -- "deals 2 damage to target
||| opponent or planeswalker and 2 damage to up to one target creature that
||| player or that planeswalker's controller controls."
public export
ravagerOfTheFellsBolt : Effect []
ravagerOfTheFellsBolt =
  Simultaneously
    [ DealDamage Macros.thisCreature (Lit 2) Cards.targetOpponentOrPlaneswalker
    , DealDamage Macros.thisCreature (Lit 2)
        (TargetGroup (Macros.upTo 1)
                     (And [Macros.creature,
                           ControlledBy Macros.splitOverPlaneswalker])) ]

||| Soul of Shandalar's battlefield activation -- "deals 3 damage to target
||| player or planeswalker and 3 damage to up to one target creature that
||| player or that planeswalker's controller controls."
public export
soulOfShandalarBolt : Effect []
soulOfShandalarBolt =
  Simultaneously
    [ DealDamage Macros.thisCreature (Lit 3) Cards.targetPlayerOrPlaneswalker
    , DealDamage Macros.thisCreature (Lit 3)
        (TargetGroup (Macros.upTo 1)
                     (And [Macros.creature,
                           ControlledBy Macros.splitOverPlaneswalker])) ]

||| Soul of Shandalar's graveyard activation -- the same clause a second
||| time, over the same union head; the card's two occurrences differ only
||| in the ability's cost and source.
public export
soulOfShandalarGraveyardBolt : Effect []
soulOfShandalarGraveyardBolt =
  Simultaneously
    [ DealDamage This (Lit 3) Cards.targetPlayerOrPlaneswalker
    , DealDamage This (Lit 3)
        (TargetGroup (Macros.upTo 1)
                     (And [Macros.creature,
                           ControlledBy Macros.splitOverPlaneswalker])) ]

||| Blightning -- "deals 3 damage to target player or planeswalker. That
||| player or that planeswalker's controller discards two cards." The split
||| read in a clause's SUBJECT, where the group-A cards put it in a
||| possessor.
public export
blightning : Effect []
blightning =
  Sequentially
    [ DealDamage This (Lit 3) Cards.targetPlayerOrPlaneswalker
    , Repeated (Lit 2) (Macros.discardsACard Macros.splitOverPlaneswalker) ]

||| Rakdos's Return -- "deals X damage to target opponent or planeswalker.
||| That player or that planeswalker's controller discards X cards." The
||| player arm writes "player" over an antecedent that said "opponent".
public export
rakdossReturn : Effect []
rakdossReturn =
  Sequentially
    [ DealDamage This (LetterVal X) Cards.targetOpponentOrPlaneswalker
    , Repeated (LetterVal X) (Macros.discardsACard Macros.splitOverPlaneswalker) ]

||| Nicol Bolas, Planeswalker's ultimate -- "deals 7 damage to target player
||| or planeswalker. That player or that planeswalker's controller discards
||| seven cards, then sacrifices seven permanents of their choice."
public export
nicolBolasUltimate : Effect []
nicolBolasUltimate =
  Sequentially
    [ DealDamage This (Lit 7) Cards.targetPlayerOrPlaneswalker
    , Repeated (Lit 7) (Macros.discardsACard Macros.splitOverPlaneswalker)
    , Repeated (Lit 7) (Macros.sacrifice Macros.splitOverPlaneswalker
                                         (Macros.a Permanent)) ]

||| Pulse of the Forge -- "deals 4 damage to target player or planeswalker.
||| Then if that player or that planeswalker's controller has more life than
||| you, return Pulse of the Forge to its owner's hand." The split read as a
||| comparison's subject.
public export
pulseOfTheForge : Effect []
pulseOfTheForge =
  Sequentially
    [ DealDamage This (Lit 4) Cards.targetPlayerOrPlaneswalker
    , If (CompareAmt (PlayerStatOf LifeTotal Macros.splitOverPlaneswalker)
                     Greater (PlayerStatOf LifeTotal You))
         (Macros.move This Macros.handZ)
         Nothing ]

||| Goblin Lyre's losing arm -- "this artifact deals damage to you equal to
||| the number of creatures that opponent or that planeswalker's controller
||| controls." The split read inside an amount, and the one occurrence whose
||| player arm writes "opponent" rather than "player".
public export
goblinLyreLoseFlip : Effect []
goblinLyreLoseFlip =
  Sequentially
    [ DealDamage Macros.thisArtifact (CountOf Macros.creatureYouControl)
                 Cards.targetOpponentOrPlaneswalker
    , DealDamage Macros.thisArtifact
                 (CountOf (And [Macros.creature,
                                ControlledBy Macros.splitOverPlaneswalker]))
                 You ]

||| Chain of Plasma's second sentence, subject only -- "Then that player or
||| that permanent's controller may discard a card." The class arm over the
||| class word [CR#115.4], which records no card type, so it writes the
||| generic "permanent" instead of an echo. A `May` body cannot name its own
||| decider once the decider is not `you`, so the offer is unwritten and the
||| noun is witnessed in the context the first sentence leaves.
public export
chainOfPlasmaOfferee : Noun (nomIntro {bs = []} (Macros.target Macros.anyTarget)) Player
chainOfPlasmaOfferee = Macros.splitOverPermanent

||| Chain Lightning's second sentence, subject only -- "Then that player or
||| that permanent's controller may pay {R}{R}." No effect row offers a bare
||| cost payment, so the clause is unwritten; the noun is the same term over
||| the same head as Chain of Plasma's.
public export
chainLightningPayer : Noun (nomIntro {bs = []} (Macros.target Macros.anyTarget)) Player
chainLightningPayer = Macros.splitOverPermanent

||| Flames of the Blood Hand's third sentence, subject only -- "If that
||| player or that planeswalker's controller would gain life this turn, that
||| player gains no life instead." No row replaces a life gain over a span,
||| so the clause is unwritten and the noun is witnessed in the context the
||| card's first sentence leaves.
public export
flamesOfTheBloodHandSubject :
  Noun (nomIntro {bs = []} Cards.targetPlayerOrPlaneswalker) Player
flamesOfTheBloodHandSubject = Macros.splitOverPlaneswalker

||| Flaming Gambit's second sentence, subject only -- "That player or that
||| planeswalker's controller may choose a creature they control and have
||| Flaming Gambit deal that damage to it instead." Neither the redirection
||| nor an offer naming its own decider is written, so the noun is witnessed
||| in the context the first sentence leaves.
public export
flamingGambitOfferee :
  Noun (nomIntro {bs = []} Cards.targetPlayerOrPlaneswalker) Player
flamingGambitOfferee = Macros.splitOverPlaneswalker

||| Quenchable Fire -- "deals 3 damage to target player or planeswalker. It
||| deals an additional 3 damage to that player or planeswalker at the
||| beginning of your next upkeep step unless that player or that
||| planeswalker's controller pays {U} before that step." One union mention
||| read BOTH ways in one sentence: `thatJoin` echoes it whole in the
||| recipient slot and the split read names its halves in the payer slot.
public export
quenchableFire : Effect []
quenchableFire =
  Sequentially
    [ DealDamage This (Lit 3) Cards.targetPlayerOrPlaneswalker
    , Macros.delayed (BeginningOf Upkeep (ByWord Yours))
        (Unless (DealDamage This (Lit 3) Macros.thatJoin)
                Macros.splitOverPlaneswalker
                (Mana [Macros.pip Blue])) ]

||| Searing Blaze, both sentences -- "deals 1 damage to target player or
||| planeswalker and 1 damage to target creature that player or that
||| planeswalker's controller controls. Landfall — If you had a land enter
||| the battlefield under your control this turn, … deals 3 damage to that
||| player or planeswalker and 3 damage to that creature instead." The whole
||| echo and the split read over ONE antecedent, which is what says the two
||| spellings are one construction. The landfall CONDITION is unwritten: a
||| replacement that applies only when a condition holds has no shape here,
||| since an `If` around the replacement drops the clause it replaces.
public export
searingBlaze : Ability
searingBlaze =
  AbilityWord Landfall
    (Spell
      (Macros.insteadOf
        (Simultaneously
           [ DealDamage This (Lit 1) Cards.targetPlayerOrPlaneswalker
           , DealDamage This (Lit 1)
               (Macros.target (And [Macros.creature,
                                    ControlledBy Macros.splitOverPlaneswalker])) ])
        (Simultaneously
           [ DealDamage This (Lit 3) Macros.thatJoin
           , DealDamage This (Lit 3) (That (TypeW Creature)) ])))

||| Thought Lash's trigger -- "When a player doesn't pay this enchantment's
||| cumulative upkeep, that player exiles all cards from their library."
||| The declined arm of `PaysCost`, whose announced payer the tail reads
||| back.
public export
thoughtLashTrigger : Ability
thoughtLashTrigger =
  Macros.triggered When
    (PaysCost (Just (Macros.a AnyPlayer)) Unpaid Macros.thisEnchantment
              "CumulativeUpkeep")
    (Does (That PlayerW) "Exile"
          (Macros.move (Each (InZone (Macros.libraryOf They))) Macros.exileZ))

||| Heart of Bogardan's header -- "When a player doesn't pay this
||| enchantment's cumulative upkeep, …", the second carrier of the declined
||| arm and the same event term Thought Lash writes. Its BODY is what does
||| not write: "deals X damage to target player or planeswalker and each
||| creature that player or that planeswalker's controller controls" needs
||| the split read `splitOverPlaneswalker`, whose demonstrative may find
||| only one singular player mention, and the header has already announced
||| the non-payer. The card's blocker, not the row's.
public export
heartOfBogardanHeader : GameEvent []
heartOfBogardanHeader =
  PaysCost (Just (Macros.a AnyPlayer)) Unpaid Macros.thisEnchantment "CumulativeUpkeep"

||| Balduvian Fallen's header -- "Whenever this creature's cumulative
||| upkeep is paid, …", the passive voice: the cost is the surface subject
||| and no payer is written, though [CR#702.24a] fixes one. Its BODY does
||| not write: "it gets +1/+0 until end of turn for each {B} or {R} spent
||| this way" counts the mana that paid the cost, and no phrase names mana
||| by what it was spent on. The card's blocker, not the row's.
public export
balduvianFallenHeader : GameEvent []
balduvianFallenHeader =
  PaysCost Nothing Paid Macros.thisCreature "CumulativeUpkeep"

||| Shah of Naar Isle's header -- "When this creature's echo cost is paid,
||| …", the passive again, over the second keyword whose parameter is a
||| cost [CR#702.30a]. Its BODY does not write: "each opponent may draw up
||| to three cards" wants a ceiling on the drawn count, and "up to [n]" is
||| a quantity over a described set with no `Amount` twin. The card's
||| blocker, not the row's.
public export
shahOfNaarIsleHeader : GameEvent []
shahOfNaarIsleHeader =
  PaysCost Nothing Paid Macros.thisCreature "Echo"

||| Font of Agonies -- "Whenever you pay life, put that many blood
||| counters on this enchantment." The paid thing is a resource and not a
||| named cost, and the payment carries the number [CR#119.4] that "that
||| many" reads back.
public export
fontOfAgoniesTrigger : Ability
fontOfAgoniesTrigger =
  Macros.triggered Whenever (PaysLife You)
    (PutCounters ThatMuch Blood Macros.thisEnchantment)

||| Hibernation's End's trigger -- "Whenever you pay this enchantment's
||| cumulative upkeep, you may search your library for a creature card with
||| mana value equal to the number of age counters on this enchantment, put
||| it onto the battlefield, then shuffle." The paid arm.
public export
hibernationsEndTrigger : Ability
hibernationsEndTrigger =
  Macros.triggered Whenever
    (PaysCost (Just You) Paid Macros.thisEnchantment "CumulativeUpkeep")
    (Macros.may You
       (Sequentially
          [ Macros.searchLibraryFor
              (And [Macros.creature,
                    Compare ManaValue Eq (CountersOn Age Macros.thisEnchantment)])
          , Macros.putOntoBattlefield (That CardW)
          , Macros.shuffle ]))

-- The choice frame that licenses a later read: a choice announced as the
-- effect applies [CR#608.2d] partitions the described set, and what it
-- leaves behind -- the unchosen members, the margin of a comparison it
-- carried -- is what the next clause names.

||| Duneblast -- "Choose up to one creature. Destroy the rest." The choice
||| is the partition: the creatures it did not pick are what "the rest"
||| names, and no group mention stands between them and the phrase.
public export
duneblast : Effect []
duneblast =
  Sequentially [ Macros.choose (CountedGroup (Macros.upTo 1) Macros.creature)
               , Macros.destroy TheRest ]

public export
duneblastCard : Card
duneblastCard =
  Macros.card "Duneblast"
       (Just [Macros.generic 4, Macros.pip White, Macros.pip Black,
              Macros.pip Green])
       [] (MkTypeLine [] [Sorcery]) [Spell Cards.duneblast] Nothing

||| Boreas Charger's description -- "an opponent who controls more lands
||| than you". The member-relative comparison: the count on the left is
||| taken on the opponent the phrase picks, the one on the right on the
||| reader.
public export
opponentWithMoreLands : Predicate [] Player
opponentWithMoreLands =
  CompareOver Opponent
    (CountOf (And [Macros.land, ControlledBy They]))
    Greater
    (CountOf (And [Macros.land, ControlledBy You]))

||| Boreas Charger's first clause -- "choose an opponent who controls more
||| lands than you". Its SECOND clause does not write: "search your library
||| for a number of Plains cards equal to the difference" needs a counted
||| search, and `Search` carries no count. The card's blocker, not the
||| row's.
public export
boreasChargerChoice : Effect []
boreasChargerChoice = Macros.choose (Macros.a Cards.opponentWithMoreLands)

||| ...and the margin that choice leaves readable, which is what the
||| unwritable clause would have spent.
public export
boreasChargerDifference : Amount (effIntro Cards.boreasChargerChoice)
boreasChargerDifference = TheDifference

||| Sandstone Oracle, whole -- "When this creature enters, choose an
||| opponent. If that player has more cards in hand than you, draw cards
||| equal to the difference." Here the comparison is the condition's, and
||| the choice's part is only what "that player" reads back.
public export
sandstoneOracle : Card
sandstoneOracle =
  Macros.card "Sandstone Oracle" (Just [Macros.generic 7]) []
       (MkTypeLine [creatureType "Sphinx"] [Artifact, Creature])
       [ Macros.keyword "Flying"
       , Macros.triggered When (Enters Macros.thisCreature)
           (Sequentially
              [ Macros.choose Macros.anOpponent
              , If (CompareAmt (CountOf (InZone (Macros.handOf (That PlayerW))))
                               Greater
                               (CountOf (InZone (Macros.handOf You))))
                   (Draw You TheDifference)
                   Nothing ]) ]
       (Just (4, 4))

||| Slithermuse's trigger -- the same sentence off a leave-the-battlefield
||| header. The whole card waits on evoke, which no keyword row writes.
public export
slithermuseTrigger : Ability
slithermuseTrigger =
  Macros.triggered When (Leaves Macros.thisCreature)
    (Sequentially
       [ Macros.choose Macros.anOpponent
       , If (CompareAmt (CountOf (InZone (Macros.handOf (That PlayerW))))
                        Greater
                        (CountOf (InZone (Macros.handOf You))))
            (Draw You TheDifference)
            Nothing ])

-- ---------------------------------------------------------------------------
-- The distinct-kind count
-- ---------------------------------------------------------------------------

||| Tarmogoyf, whole definition line -- "Tarmogoyf's power is equal to the
||| number of card types among cards in all graveyards and its toughness
||| is equal to that number plus 1." The distinct-kind count at the
||| definition's first slot; "all graveyards" is every player's own
||| [CR#404.1], so the possessor is `AllPlayers` and not the bare zone.
||| Barrowgoyf, Polygoyf, Pyrogoyf and Tarmogoyf Nest's token write the
||| same line.
public export
tarmogoyfDefinition : Ability
tarmogoyfDefinition =
  Static (AndAlso
    [ DefinesPt Macros.thisCreature PowerAlone
        (DistinctCount CardTypeAxis
           (AllOf (InZone (Macros.graveyardOf (PlayerGroup AllPlayers)))))
    , DefinesPt Macros.thisCreature ToughnessAlone
        (Plus ThatMuch (Lit 1)) ])

||| Tarmogoyf's printed box -- "*/1+*".
public export
tarmogoyfBox : PrintedBox
tarmogoyfBox = PtBox PrintedStar (PrintedStarPlus 1)

||| Consuming Blob's definition -- "Consuming Blob's power is equal to the
||| number of card types among cards in your graveyard and its toughness is
||| equal to that number plus 1." The same line over ONE graveyard;
||| Nethergoyf writes it too.
public export
consumingBlobDefinition : Ability
consumingBlobDefinition =
  Static (AndAlso
    [ DefinesPt Macros.thisCreature PowerAlone
        (DistinctCount CardTypeAxis
           (AllOf (InZone (Macros.graveyardOf You))))
    , DefinesPt Macros.thisCreature ToughnessAlone
        (Plus ThatMuch (Lit 1)) ])

||| Nighthawk Scavenger's definition -- "Nighthawk Scavenger's power is
||| equal to 1 plus the number of card types among cards in your
||| opponents' graveyards." One slot, an offset count, and the possessor
||| written on the zone.
public export
nighthawkScavengerDefinition : Ability
nighthawkScavengerDefinition =
  Static (DefinesPt Macros.thisCreature PowerAlone
    (Plus (Lit 1)
          (DistinctCount CardTypeAxis
             (AllOf (InZone (Macros.graveyardOf (PlayerGroup YourOpponents)))))))

||| Lucid Dreams, whole -- "Draw X cards, where X is the number of card
||| types among cards in your graveyard."
public export
lucidDreams : Card
lucidDreams =
  Macros.card "Lucid Dreams"
       (Just [Macros.generic 3, Macros.pip Blue, Macros.pip Blue]) []
       (MkTypeLine [] [Sorcery])
       [ Spell (Sequentially
                  [ Draw You (LetterVal X)
                  , Define X (DistinctCount CardTypeAxis
                                (AllOf (InZone (Macros.graveyardOf You)))) ]) ]
       Nothing

||| Tribal Flames, whole -- "Domain — Tribal Flames deals X damage to any
||| target, where X is the number of basic land types among lands you
||| control." The subtype axis under its basic-only scope [CR#305.6].
public export
tribalFlames : Card
tribalFlames =
  Macros.card "Tribal Flames" (Just [Macros.generic 1, Macros.pip Red]) []
       (MkTypeLine [] [Sorcery])
       [ AbilityWord Domain
           (Spell (Sequentially
                     [ DealDamage This (LetterVal X)
                                  (Macros.target Macros.anyTarget)
                     , Define X (DistinctCount (SubtypeAxis Land BasicOnly)
                                   (AllOf (And [Macros.land,
                                                ControlledBy You]))) ])) ]
       Nothing

||| Explosive Prodigy's trigger -- "Vivid — When this creature enters, it
||| deals X damage to target creature an opponent controls, where X is the
||| number of colors among permanents you control." The colour axis
||| [CR#105.1].
public export
explosiveProdigyTrigger : Ability
explosiveProdigyTrigger =
  AbilityWord Vivid
    (Macros.triggered When (Enters Macros.thisCreature)
       (Sequentially
          [ DealDamage It (LetterVal X)
                       (Macros.target (And [Macros.creature,
                                            ControlledBy Macros.anOpponent]))
          , Define X (DistinctCount ColorAxis
                        (AllOf (And [Permanent, ControlledBy You]))) ]))

||| Korvold, Gleeful Glutton's combat trigger -- "Whenever Korvold deals
||| combat damage to a player, put X +1/+1 counters on Korvold and draw X
||| cards, where X is the number of permanent types among cards in your
||| graveyard." The permanent-type axis [CR#110.4], a named six of
||| [CR#205.2a]'s fifteen.
public export
korvoldCombatTrigger : Ability
korvoldCombatTrigger =
  Macros.triggered Whenever
    (DealsCombatDamage Macros.thisCreature (Macros.a AnyPlayer))
    (Sequentially
       [ PutCounters (LetterVal X) Macros.plusOnePlusOne Macros.thisCreature
       , Draw You (LetterVal X)
       , Define X (DistinctCount PermanentTypeAxis
                     (AllOf (InZone (Macros.graveyardOf You)))) ])

||| General Tazri's pump -- "{W}{U}{B}{R}{G}: Ally creatures you control
||| get +X/+X until end of turn, where X is the number of colors among
||| those creatures." The domain is a MENTION here, which the one `Noun`
||| slot takes as it takes a description.
public export
generalTazriPump : Ability
generalTazriPump =
  Macros.activated
    (Mana [Macros.pip White, Macros.pip Blue, Macros.pip Black,
           Macros.pip Red, Macros.pip Green])
    (Sequentially
       [ Macros.gets (Each (And [Macros.creature,
                                 HasSubtype (creatureType "Ally"),
                                 ControlledBy You]))
                     (PtUp (LetterVal X)) (PtUp (LetterVal X))
                     (Just Macros.untilEndOfTurn)
       , Define X (DistinctCount ColorAxis (Those (TypeW Creature))) ])
