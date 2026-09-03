module Experimental.Cards

import Experimental
import Experimental.Macros

%default total


||| Lightning Bolt
bolt : Effect []
bolt = DealDamage This (Lit 3) (Macros.target Macros.anyTarget)

barrageOfBoulders : Effect []
barrageOfBoulders = DealDamage This (Lit 1) (Macros.each Macros.creatureYouDontControl)

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
cloudshift = Sequentially [Macros.exile You (Macros.target Macros.creatureYouControl),
                           Macros.putOntoBattlefieldUnderYourControl (That CardW)]

||| Through the Breach Splice
throughTheBreach : Effect []
throughTheBreach = Sequentially [Macros.may You (Macros.move (Macros.a (And [Macros.creature, InZone (Macros.handOf You)])) Macros.battlefieldZ),
                                 Macros.gainsHaste (That (TypeW Creature)) Nothing,
                                 Macros.delayed (BeginningOf EndStep NoPossessor) (Macros.sacrifice You (That (TypeW Creature)))]

bitterDownfall : Effect []
bitterDownfall = Sequentially [Macros.destroy (Macros.target Macros.creature),
                               Macros.losesLife (Macros.controllerOf It) (Lit 2)]

deadshot : Effect []
deadshot = Sequentially [SetStatus Tapped (Macros.target Macros.creature),
                         DealDamage It (Macros.powerOf It) (Macros.target (And [Macros.creature, Other]))]

immersturmSkullcairn : Ability
immersturmSkullcairn =
  Macros.activatedOnlyDuring (Compound [Mana [Macros.generic 1, Macros.pip Black, Macros.pip Red, Macros.pip Red], TapSymbol,
                                        Do (Macros.sacrifice You Macros.thisLand)])
                             (Sequentially [DealDamage It (Lit 3) (Macros.target AnyPlayer),
                                            (Macros.discard (That PlayerW) (Macros.a (InZone Macros.handZ)))])
                             AsSorcery

pyriteSpellbomb : Ability
pyriteSpellbomb = Macros.activated (Compound [Mana [Macros.pip Red], Do (Macros.sacrifice You Macros.thisArtifact)])
                                   (DealDamage It (Lit 2) (Macros.target Macros.anyTarget))

diabolicEdict : Effect []
diabolicEdict = Macros.sacrifice (Macros.target AnyPlayer) (Macros.aTheirChoice Macros.creature)

innocentBlood : Effect []
innocentBlood = Macros.sacrifice (Macros.each AnyPlayer) (Macros.aTheirChoice Macros.creature)

cryOfContrition : Effect []
cryOfContrition = (Macros.discard (Macros.target AnyPlayer) (Macros.a (InZone Macros.handZ)))

carefulStudy : Effect []
carefulStudy = Sequentially [(Macros.draw You (Lit 2)), (Repeated (Lit 2) (Sequentially [Macros.choose (Macros.a (InZone Macros.handZ)), Macros.discard You (That CardW)]))]

zombieInfestation : Ability
zombieInfestation =
  Macros.activated (Do ((Repeated (Lit 2) (Sequentially [Macros.choose (Macros.a (InZone Macros.handZ)), Macros.discard You (That CardW)]))))
                   (Macros.create (Lit 1)
                      (MkToken (Just (Lit 2 ** Lit 2)) [Black]
                               (MkTypeLine [creatureType "Zombie"] [Creature]) [] Nothing))

suspendedSentence : Effect []
suspendedSentence = Sequentially [Macros.destroy (Macros.target (And [Macros.creature, HasPossessor ControllerAx Macros.anOpponent])),
                                  Macros.losesLife (That PlayerW) (Lit 3)]

flickeringSpirit : Effect []
flickeringSpirit = Sequentially [Macros.exile You Macros.thisCreature,
                                 Macros.move It Macros.battlefieldZ]

turnToMist : Effect []
turnToMist = Sequentially [Macros.exile You (Macros.target Macros.creature),
                           Macros.delayed (BeginningOf EndStep NoPossessor) (Macros.move (That CardW) Macros.battlefieldZ)]

jump : Effect []
jump = Macros.gains (Macros.target Macros.creature) (Macros.keyword "Flying") (Just Macros.untilEndOfTurn)

giantGrowth : Effect []
giantGrowth = Macros.gets (Macros.target Macros.creature) (PtUp (Lit 3)) (PtUp (Lit 3)) (Just Macros.untilEndOfTurn)

glyphOfDestruction : Effect []
glyphOfDestruction =
  Macros.gets (Macros.target (And [Blocking, Macros.creature, HasPossessor ControllerAx You])) (PtUp (Lit 10)) (PtUp (Lit 0)) (Just Macros.untilEndOfCombat)

gabrielAngelfire : Effect []
gabrielAngelfire =
  Macros.gains Macros.thisCreature (Macros.keyword "Flying") (Just Macros.untilYourNextUpkeep)

||| Bond of Revival
bondOfRevival : Effect []
bondOfRevival = Sequentially [Macros.returnToBattlefield (Macros.target (And [Macros.creature, InZone (Macros.graveyardOf You)])),
                              Macros.gainsHaste (ItVerbed "Return") (Just Macros.untilYourNextTurn)]

gracefulReprieve : Effect []
gracefulReprieve = Macros.delayedWithin (Dies (Macros.target Macros.creature))
                                        ThisTurn
                                        (Macros.move (That CardW) Macros.battlefieldZ)

vraskasStoneglare : Effect []
vraskasStoneglare = Sequentially [Macros.destroy (Macros.target Macros.creature),
                                  Macros.gainsLife You (Macros.toughnessOf It)]

phthisis : Effect []
phthisis = Sequentially [Macros.destroy (Macros.target Macros.creature),
                         Macros.losesLife (Macros.controllerOf It) (Plus (Macros.powerOf It) (Macros.toughnessOf It))]

karplusanYeti : Effect []
karplusanYeti = Sequentially [DealDamage Macros.thisCreature (Macros.powerOf Macros.thisCreature) (Macros.target Macros.creature),
                              DealDamage (That (TypeW Creature)) (Macros.powerOf It) Macros.thisCreature]

fulgentDistraction : Effect []
fulgentDistraction = Sequentially [Macros.choose (Macros.targets (Macros.exactly 2) Macros.creature),
                                   SetStatus Tapped (Those (TypeW Creature))]

continueSpell : Effect []
continueSpell = Sequentially [Macros.choose (Macros.targets (Macros.upTo 4) (And [Macros.creature, InZone (Macros.graveyardOf You)])),
                              Macros.move Them Macros.battlefieldZ]

suddenDemise : Effect []
suddenDemise = Sequentially [Macros.choose (Macros.a (Macros.quality Color)),
                             DealDamage This (LetterVal X) (Macros.each (And [Macros.creature, OfChosen Color]))]

kindredDominance : Effect []
kindredDominance = Sequentially [Macros.choose (Macros.a (Macros.quality (SubtypeQ Creature))),
                                 Macros.destroy (Macros.allOf (And [Macros.creature, Not (OfChosen (SubtypeQ Creature))]))]

voyagerStaff : Ability
voyagerStaff = Macros.activated (Compound [Mana [Macros.generic 2], Do (Macros.sacrifice You Macros.thisArtifact)])
                                (Sequentially [Macros.exile You (Macros.target Macros.creature),
                                          Macros.delayed (BeginningOf EndStep NoPossessor) (Macros.move (Macros.theVerbed "Exile" CardW) Macros.battlefieldZ)])

boshIronGolem : Ability
boshIronGolem = Macros.activated (Compound [Mana [Macros.generic 3, Macros.pip Red],
                                     Do (Macros.sacrifice You (Macros.a (HasType Artifact)))])
                                 (DealDamage This
                                      (Macros.manaValueOf (Macros.theVerbed "Sacrifice" (TypeW Artifact)))
                                      (Macros.target Macros.anyTarget))

pyromancy : Ability
pyromancy = Macros.activated (Compound [Mana [Macros.generic 3],
                                 Do (Macros.discard You (Macros.aAtRandom (InZone Macros.handZ)))])
                             (DealDamage Macros.thisEnchantment
                                  (Macros.manaValueOf (Macros.theVerbed "Discard" CardW))
                                  (Macros.target Macros.anyTarget))

foulTongueShriek : Effect []
foulTongueShriek = Sequentially [Macros.losesLife (Macros.target Opponent)
                                           (Macros.forEach (And [Attacking, Macros.creature, HasPossessor ControllerAx You])),
                                 Macros.gainsLife You ThatMuch]

unsummon : Effect []
unsummon = Macros.move (Macros.target Macros.creature) Macros.handZ

phantomBlade : Effect []
phantomBlade = Sequentially [Macros.choose (Macros.targets (Macros.upTo 1) (And [Macros.creature, HasPossessor ControllerAx You])),
                             Macros.destroy (Macros.targets (Macros.upTo 1) (And [Macros.creature, Other]))]

caseOfTheGatewayExpress : Effect []
caseOfTheGatewayExpress = Sequentially [Macros.choose (Macros.target Macros.creatureYouDontControl),
                                        DealDamage (Macros.each Macros.creatureYouControl) (Lit 1)
                                                   (That (TypeW Creature))]

cyclingCost : Effect []
cyclingCost = Macros.discard You This

rawNonattacking : Predicate [] Object
rawNonattacking = And [Macros.creature, Not Attacking, Not Blocking]

rawExile : Effect []
rawExile = Macros.exile You (Macros.target Macros.creature)


disenchant : Effect []
disenchant = Macros.destroy (Macros.target (Or [Macros.artifact, Macros.enchantment]))

icyManipulator : Effect []
icyManipulator = SetStatus Tapped (Macros.target (Or [Macros.artifact, Macros.creature, Macros.land]))

masterDecoy : Ability
masterDecoy =
  Macros.activated (Compound [Mana [Macros.pip White], TapSymbol])
                   (Macros.tap (Macros.target Macros.creature))

harmonyOfNature : Effect []
harmonyOfNature =
  Sequentially [ Macros.tap (Macros.counted Macros.anyNumber
                              (And [Macros.untapped, Macros.creature, HasPossessor ControllerAx You]))
               , ForEachOf (Macros.thoseVerbedThisWay "Tap" (TypeW Creature))
                           (Macros.gainsLife You (Lit 4)) ]

ratsOfRath : Effect []
ratsOfRath = Macros.destroy (Macros.target (And [Or [Macros.artifact, Macros.creature, Macros.land], HasPossessor ControllerAx You]))

arrowsOfJustice : Effect []
arrowsOfJustice = DealDamage This (Lit 4)
                             (Macros.target (And [Macros.creature, Or [Attacking, Blocking]]))

anotherDisjunctPhrase : Predicate [MkBinding TargetD Object OneOf
                                             (ObjectP Nothing (Just Battlefield) Nothing Nothing Nothing)] Object
anotherDisjunctPhrase = And [Or [Macros.creature, Macros.land], Other]


infiltrate : Effect []
infiltrate = Macros.cantBeBlocked (Macros.target Macros.creature) (Just Macros.thisTurn)

changeOfHeart : Effect []
changeOfHeart = Macros.cantAttack (Macros.target Macros.creature) (Just Macros.thisTurn)

blindblast : Effect []
blindblast = Sequentially [DealDamage This (Lit 1) (Macros.target Macros.creature),
                           Macros.cantBlock (That (TypeW Creature)) (Just Macros.thisTurn)]

blindingFlare : Effect []
blindingFlare = Macros.cantBlock (Macros.targets Macros.anyNumber Macros.creature) (Just Macros.thisTurn)


defeat : Effect []
defeat = Macros.destroy (Macros.target (And [Macros.creature, Compare [CharAxis Power] AtMost (Lit 2)]))

terashisVerdict : Effect []
terashisVerdict =
  Macros.destroy (Macros.target (And [Macros.creature, Attacking, Compare [CharAxis Power] AtMost (Lit 3)]))

pillarOfLight : Effect []
pillarOfLight =
  Macros.exile You (Macros.target (And [Macros.creature, Compare [CharAxis Toughness] AtLeast (Lit 4)]))

luckyOffering : Effect []
luckyOffering =
  Sequentially [Macros.destroy (Macros.target (And [Macros.artifact,
                                      Compare [CharAxis ManaValue] AtMost (Lit 3)])),
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
  Sequentially [DealDamage This (Lit 4) (Macros.target (And [Macros.creature, HasPossessor ControllerAx Macros.anOpponent])),
                OnlyIf (DealDamage This (Lit 2)
                                   (Macros.each (And [Macros.creature, Other, HasPossessor ControllerAx (That PlayerW)])))
                       (Macros.exists (And [Macros.creature, HasPossessor ControllerAx You,
                                     Compare [CharAxis Power] AtLeast (Lit 4)]))
                       Nothing]

||| Braids's Frightful Return
braidsFrightfulReturn : Effect []
braidsFrightfulReturn =
  (May You (Macros.sacrifice You (Macros.a Macros.creature)) (Just ((Macros.discard (Macros.each Opponent) (Macros.a (InZone Macros.handZ))))) Nothing)

||| Daretti, Ingenious Iconoclast
darettisMinusOne : Effect []
darettisMinusOne =
  (May You (Macros.sacrifice You (Macros.a Macros.artifact)) (Just (Macros.destroy (Macros.target (Or [Macros.artifact, Macros.creature])))) Nothing)

||| Yawgmoth Demon
crovaxTheCursed : Effect []
crovaxTheCursed =
  (May You (Macros.sacrifice You (Macros.a Macros.creature)) (Just (PutCounters (Lit 1) (PrintedKind Macros.plusOnePlusOne) Macros.thisCreature)) (Just (RemoveCounters (Just (Macros.exactly 1)) (Just (PrintedKind Macros.plusOnePlusOne)) Macros.thisCreature)))

yawgmothDemon : Effect []
yawgmothDemon =
  (May You (Macros.sacrifice You (Macros.a Macros.artifact)) Nothing (Just (Sequentially [SetStatus Tapped Macros.thisCreature, DealDamage This (Lit 2) You])))


raiseTheAlarm : Effect []
raiseTheAlarm = Macros.create (Lit 2) (Macros.creatureTok 1 1 [White] [creatureType "Soldier"])

additiveEvolution : Effect []
additiveEvolution = Sequentially [Macros.create (Lit 1) (Macros.creatureTok 0 0 [Green, Blue] [creatureType "Fractal"]),
                                  PutCounters (Lit 3) (PrintedKind Macros.plusOnePlusOne) It]

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
  Sequentially [If (Macros.notSo (Macros.exists (And [HasSubtype (creatureType "Army"), Macros.creature, HasPossessor ControllerAx You])))
                   (Macros.create (Lit 1) (Macros.creatureTok 0 0 [Black] [creatureType "Zombie", creatureType "Army"]))
                   Nothing,
                Macros.choose (Macros.a (And [HasSubtype (creatureType "Army"), Macros.creature, HasPossessor ControllerAx You])),
                PutCounters (Lit 2) (PrintedKind Macros.plusOnePlusOne) (That (TypeW Creature)),
                If (Macros.itIsntA (HasSubtype (creatureType "Zombie")))
                   (Macros.becomes It (Macros.subtypesOnly [creatureType "Zombie"]) Nothing)
                   Nothing]

battlegrowth : Effect []
battlegrowth = PutCounters (Lit 1) (PrintedKind Macros.plusOnePlusOne) (Macros.target Macros.creature)

chainbreaker : Effect []
chainbreaker = RemoveCounters (Just (Macros.exactly 1)) (Just (PrintedKind Macros.minusOneMinusOne)) (Macros.target Macros.creature)

kaitoBaneOfNightmares : Effect []
kaitoBaneOfNightmares = Sequentially [SetStatus Tapped (Macros.target Macros.creature),
                                      PutCounters (Lit 2) (PrintedKind (Named "Stun")) It]

jhoiraOfTheGhitu : Ability
jhoiraOfTheGhitu =
  Macros.activated (Compound [Mana [Macros.generic 2],
                       Do (Macros.exile You (Macros.a (And [Not Macros.land, InZone (Macros.handOf You)])))])
                   (PutCounters (Lit 4) (PrintedKind (Named "Time")) (Macros.theVerbed "Exile" CardW))

alaundoTheSeer : Effect []
alaundoTheSeer = RemoveCounters (Just (Macros.exactly 1)) (Just (PrintedKind (Named "Time"))) (Macros.each (InZone Macros.exileZ))

arcBlade : Effect []
arcBlade = Sequentially [DealDamage This (Lit 2) (Macros.target Macros.anyTarget),
                         Macros.exileWithCounters This (Lit 3) (Named "Time")]

daydream : Card
daydream =
  Macros.card "Daydream" (Just [Macros.pip White]) [] (MkTypeLine [] [Sorcery])
       [Spell (Sequentially [Macros.exile You (Macros.target Macros.creatureYouControl),
                             Macros.returnToBattlefieldWithCounters
                               (That CardW) (Macros.ownerOf (That CardW))
                               (Lit 1) Macros.plusOnePlusOne])]
       Nothing

ashnodsTransmogrant : Ability
ashnodsTransmogrant =
  Macros.activated (Compound [TapSymbol, Do (Macros.sacrifice You Macros.thisArtifact)])
                   (Sequentially [PutCounters (Lit 1) (PrintedKind Macros.plusOnePlusOne)
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
  Sequentially [(Macros.draw You (Lit 1)),
                If (Macros.exists (And [HasSubtype (creatureType "Demon"), HasPossessor ControllerAx You]))
                   (Sequentially [Macros.losesLife (Macros.each Opponent) (Lit 2),
                                  Macros.gainsLife You (Lit 2)])
                   (Just (Macros.losesLife You (Lit 2)))]


divination : Effect []
divination = (Macros.draw You (Lit 2))

ancestralRecall : Effect []
ancestralRecall = Draw (Macros.target AnyPlayer) (Lit 3)

public export
cheeringFanatic : Card
cheeringFanatic =
  Macros.card "Cheering Fanatic" (Just [Macros.generic 1, Macros.pip Red]) []
       (MkTypeLine [creatureType "Goblin"] [Creature])
       [ Macros.triggered Whenever (Macros.attacks Macros.thisCreature)
                          (Sequentially
                     [ Macros.choose (Macros.a (Macros.quality CardName))
                     , Continuously {ts = StaticFirstDone}
                         (CostsToCast (Macros.allOf (And [Macros.spell, OfChosen CardName]))
                                      (CostLess (Lit 1) Nothing))
                         (Just Macros.thisTurn) ]) ]
       (Just (2, 2))

public export
prosperity : Card
prosperity =
  Macros.card "Prosperity" (Just [Variable, Macros.pip Blue]) []
       (MkTypeLine [] [Sorcery]) [Spell (Draw (Macros.each AnyPlayer) (LetterVal X))] Nothing

collectiveUnconscious : Effect []
collectiveUnconscious = Draw You (Macros.forEach Macros.creatureYouControl)

killiansConfidence : Effect []
killiansConfidence = Sequentially [Macros.gets (Macros.target Macros.creature) (PtUp (Lit 1)) (PtUp (Lit 1)) (Just Macros.untilEndOfTurn),
                                   (Macros.draw You (Lit 1))]

||| Blindblast
blindblastWhole : Effect []
blindblastWhole = Sequentially [DealDamage This (Lit 1) (Macros.target Macros.creature),
                                Macros.cantBlock (That (TypeW Creature)) (Just Macros.thisTurn),
                                (Macros.draw You (Lit 1))]

cycling : Ability
cycling = Macros.activated (Compound [Mana [Macros.generic 2], Do (Macros.discard You This)]) (Macros.draw You (Lit 1))

abrade : Effect []
abrade = Macros.chooseOne [DealDamage This (Lit 3) (Macros.target Macros.creature),
                    Macros.destroy (Macros.target Macros.artifact)]

austereCommand : Effect []
austereCommand =
  Macros.chooseTwo [Macros.destroy (Macros.allOf Macros.artifact),
             Macros.destroy (Macros.allOf Macros.enchantment),
             Macros.destroy (Macros.allOf (And [Macros.creature, Compare [CharAxis ManaValue] AtMost (Lit 3)])),
             Macros.destroy (Macros.allOf (And [Macros.creature, Compare [CharAxis ManaValue] AtLeast (Lit 4)]))]

azulaAlwaysLies : Effect []
azulaAlwaysLies =
  Macros.chooseOneOrBoth [Macros.gets (Macros.target Macros.creature) (PtDown (Lit 1)) (PtDown (Lit 1)) (Just Macros.untilEndOfTurn),
                   PutCounters (Lit 1) (PrintedKind Macros.plusOnePlusOne) (Macros.target Macros.creature)]

rainOfThorns : Effect []
rainOfThorns = Macros.chooseOneOrMore [Macros.destroy (Macros.target Macros.artifact),
                                Macros.destroy (Macros.target Macros.enchantment),
                                Macros.destroy (Macros.target Macros.land)]

rankleMasterOfPranks : Effect []
rankleMasterOfPranks =
  Macros.chooseAnyNumber [(Macros.discard (Macros.each AnyPlayer) (Macros.a (InZone Macros.handZ))),
                   Macros.sacrifice (Macros.each AnyPlayer) (Macros.aTheirChoice Macros.creature)]

myrkulsEdict : Effect []
myrkulsEdict = Sequentially [Macros.choose (Macros.a Opponent),
                             Macros.sacrifice (That PlayerW) (Macros.aTheirChoice Macros.creature)]



nibelheimAflame : Effect []
nibelheimAflame =
  Sequentially [Macros.choose (Macros.target Macros.creatureYouControl),
                DealDamage It (Macros.powerOf It) (Macros.each (Macros.otherCreature It))]

warScreecher : Effect []
warScreecher =
  Macros.gets (Macros.allOf (Macros.otherCreatureYouControl Macros.thisCreature)) (PtUp (Lit 1)) (PtUp (Lit 1)) (Just Macros.untilEndOfTurn)

bellowingAegisaur : Effect []
bellowingAegisaur =
  PutCounters (Lit 1) (PrintedKind Macros.plusOnePlusOne) (Macros.each (Macros.otherCreatureYouControl Macros.thisCreature))

carnifexDemon : Effect []
carnifexDemon =
  PutCounters (Lit 1) (PrintedKind Macros.minusOneMinusOne) (Macros.each (Macros.otherCreature Macros.thisCreature))

syphonMind : Effect []
syphonMind =
  Sequentially
    [ Macros.discard (Macros.each Macros.otherPlayer) (Macros.a (InZone Macros.handZ))
    , ForEachOf (Macros.thoseVerbedThisWay "Discard" CardW) (Draw You (Lit 1)) ]

brashTaunter : Effect []
brashTaunter = Fights Macros.thisCreature (Macros.target (Macros.otherCreature Macros.thisCreature))

ulvenwaldTracker : Effect []
ulvenwaldTracker = Fights (Macros.target Macros.creatureYouControl) (Macros.target (And [Macros.creature, Other]))

actOfTreason : Effect []
actOfTreason = Macros.gainControl You (Macros.target Macros.creature) (Just Macros.untilEndOfTurn)

mindFlayer : Effect []
mindFlayer =
  Macros.gainControl You (Macros.target Macros.creature) (Just (ForAsLongAs (Matches Macros.thisCreature (HasPossessor ControllerAx You))))

phyrexianInfiltrator : Effect []
phyrexianInfiltrator =
  Simultaneously
    [Macros.gainControl (Macros.controllerOf (Macros.target Macros.creature)) Macros.thisCreature Nothing,
     Macros.gainControl (Macros.controllerOf Macros.thisCreature) (That (TypeW Creature)) Nothing]

grismold : Effect []
grismold = Create (Macros.each AnyPlayer) (Lit 1)
                  (TokenWritten (Macros.creatureTok 1 1 [Green] [creatureType "Plant"])) []

sparkmagesGambit : Effect []
sparkmagesGambit =
  Sequentially [DealDamage This (Lit 1) (EachOf (Macros.targets (Macros.upTo 2) Macros.creature)),
                Macros.cantBlock (Those (TypeW Creature)) (Just Macros.thisTurn)]

ajaniAdversaryOfTyrants : Effect []
ajaniAdversaryOfTyrants =
  PutCounters (Lit 1) (PrintedKind Macros.plusOnePlusOne) (EachOf (Macros.targets (Macros.upTo 2) Macros.creature))

fallOfTheTitans : Effect []
fallOfTheTitans = DealDamage This (LetterVal X) (EachOf (Macros.targets (Macros.upTo 2) Macros.anyTarget))

naturesPanoply : Effect []
naturesPanoply =
  Sequentially [Macros.choose (Macros.targets Macros.anyNumber Macros.creature),
                PutCounters (Lit 1) (PrintedKind Macros.plusOnePlusOne) (EachOf Them)]

arcLightning : Effect []
arcLightning = Macros.dealsDivided This (Lit 3) (Macros.targets (Macros.oneThrough 3) Macros.anyTarget)

forkedBolt : Effect []
forkedBolt = Macros.dealsDivided This (Lit 2) (Macros.targets (Macros.oneThrough 2) Macros.anyTarget)

boulderfall : Effect []
boulderfall = Macros.dealsDivided This (Lit 5) (Macros.targets Macros.anyNumber Macros.anyTarget)

armamentCorps : Effect []
armamentCorps =
  Macros.distributeCounters (Lit 2) Macros.plusOnePlusOne
                     (Macros.targets (Macros.oneThrough 2) Macros.creatureYouControl)



peek : Effect []
peek = Sequentially [Macros.lookAtHandOf (Macros.target AnyPlayer), (Macros.draw You (Lit 1))]

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
                               , Macros.exile You (Macros.someOf 4 Them)
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
               , Macros.exile You (Those CardW) ]

lookAtTopThenBin : Effect []
lookAtTopThenBin =
  Sequentially [Macros.lookAt Macros.topCard, Macros.may You (Macros.move (That CardW) Macros.graveyardZ)]



botBashingTime : Effect []
botBashingTime =
  Sequentially [ DealDamage This (Lit 6) (Macros.target Macros.creature)
               , Macros.ifWouldInstead (Dies (That (TypeW Creature))) (Macros.exile You It) (Just Macros.thisTurn)
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
  Macros.preventNext AnyDamage (Macros.shieldingIt (Macros.target Macros.anyTarget)) (Lit 3) (Just Macros.thisTurn)

banisherPriest : Effect []
banisherPriest = Macros.exileUntil (Macros.target (And [Macros.creature, HasPossessor ControllerAx Macros.anOpponent]))
                            (Macros.leavesBattlefield Macros.thisCreature)

||| Tezzeret, Artifice Master
tezzeretDrawTwo : Effect []
tezzeretDrawTwo =
  Macros.insteadOf (Macros.draw You (Lit 1))
            (If (CompareAmt (Macros.countOf (And [Macros.artifact, HasPossessor ControllerAx You]))
                            AtLeast (Lit 3))
                ((Macros.draw You (Lit 2)))
                Nothing)

||| Zimone, Quandrix Prodigy
zimoneDrawTwo : Effect []
zimoneDrawTwo =
  Macros.insteadOf (Macros.draw You (Lit 1))
            (If (CompareAmt (Macros.countOf (And [Macros.land, HasPossessor ControllerAx You]))
                            AtLeast (Lit 8))
                ((Macros.draw You (Lit 2)))
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
erebos = Macros.activated (Compound [Mana [Macros.generic 1, Macros.pip Black], Macros.payLife You 2]) (Macros.draw You (Lit 1))

savagebornHydra : Ability
savagebornHydra =
  Macros.activatedOnlyDuring (Mana [Macros.generic 1, Macros.hybridPip Red Green])
                             (PutCounters (Lit 1) (PrintedKind Macros.plusOnePlusOne) Macros.thisCreature)
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
                                (Macros.draw You (Lit 1))
                                (Macros.exists (And [Macros.creature, HasPossessor ControllerAx You,
                                              Compare [CharAxis Power] AtLeast (Lit 4)])) ]
       Nothing

woeleecher : Ability
woeleecher =
  Macros.activated (Compound [Mana [Macros.pip White], TapSymbol])
                   ((IfDone (RemoveCounters (Just (Macros.exactly 1)) (Just (PrintedKind Macros.minusOneMinusOne)) (Macros.target Macros.creature)) (Just (Macros.gainsLife You (Lit 2))) Nothing))

moltingHarpy : Effect []
moltingHarpy = (May You (Pay You (Mana [Macros.generic 2]) PaidOnce) Nothing (Just (Macros.sacrifice You Macros.thisCreature)))

carnophage : Effect []
carnophage = (May You (Pay You (Macros.payLife You 1) PaidOnce) Nothing (Just (SetStatus Tapped Macros.thisCreature)))

solitaryConfinement : Effect []
solitaryConfinement = (May You ((Macros.discard You (Macros.a (InZone Macros.handZ)))) Nothing (Just (Macros.sacrifice You Macros.thisEnchantment)))


securityDetail : Ability
securityDetail =
  Macros.activatedOnlyOnceIf (Mana [Macros.pip White, Macros.pip White])
                             (Macros.create (Lit 1) (Macros.creatureTok 1 1 [White] [creatureType "Soldier"]))
                             OncePerTurn
                             (Macros.notSo (Macros.exists (And [Macros.creature, HasPossessor ControllerAx You])))




cloudkinSeer : Ability
cloudkinSeer = Macros.triggered When (Enters Macros.thisCreature Nothing) (Macros.draw You (Lit 1))

moonlitWake : Ability
moonlitWake = Macros.triggered Whenever (Dies (Macros.a Macros.creature)) (Macros.gainsLife You (Lit 1))

promiseOfTomorrow : Ability
promiseOfTomorrow = Macros.triggered Whenever (Dies (Macros.a Macros.creatureYouControl)) (Macros.exile You It)

staffOfNin : Ability
staffOfNin = Macros.triggered At (BeginningOf Upkeep (Macros.yours)) (Macros.draw You (Lit 1))

libraryLarcenist : Ability
libraryLarcenist = Macros.triggered Whenever (Macros.attacks Macros.thisCreature) (Macros.draw You (Lit 1))

eliteJavelineer : Ability
eliteJavelineer =
  Macros.triggered Whenever (Blocks Macros.thisCreature Nothing)
                   (DealDamage This (Lit 1) (Macros.target (And [Macros.creature, Attacking])))

jhessianThief : Ability
jhessianThief =
  Macros.triggered Whenever (Macros.dealsCombatDamage Macros.thisCreature (Macros.a AnyPlayer)) (Macros.draw You (Lit 1))

scholarOfStars : Ability
scholarOfStars =
  Macros.triggeredIf When
                     (Enters Macros.thisCreature Nothing)
                     (Macros.exists (And [Macros.artifact, HasPossessor ControllerAx You]))
                     (Macros.draw You (Lit 1))

||| Glacial Chasm
glacialChasmCant : Ability
glacialChasmCant = Static (Macros.deontic (Macros.allOf Macros.creatureYouControl) Forbid ["Attack"] Agent NoDeonticPatient)

||| Glacial Chasm
glacialChasmShield : Ability
glacialChasmShield =
  Static (Prevents AnyDamage Unattributed (Macros.shieldingIt You) CutAll Repeatedly Nothing)

miserysShadow : Ability
miserysShadow =
  Static (Intercepts (Dies (Macros.a (And [Macros.creature, HasPossessor ControllerAx (Macros.a Opponent)]))) [] Nothing
                     (Macros.exile You It) Repeatedly Nothing)

thoughtReflection : Ability
thoughtReflection =
  Static (Intercepts (Draws You) [] Nothing (Draw You (Lit 2)) Repeatedly Nothing)

jorKadeen : Ability
jorKadeen =
  Static (Macros.asLongAs (CompareAmt (Macros.countOf (And [Macros.artifact, HasPossessor ControllerAx You]))
                               AtLeast (Lit 3))
                   (Gets (Macros.allOf Macros.creatureYouControl) (PtUp (Lit 3)) (PtUp (Lit 0))))

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
timeVaultLock = Static (Macros.doesntUntap Macros.thisArtifact (Just You))

vindicate : Effect []
vindicate = Macros.destroy (Macros.target Permanent)

yasminKhan : Ability
yasminKhan =
  Macros.activated TapSymbol
                   (Sequentially [Macros.exile You Macros.topCard,
                           Continuously {ts = StaticFirstDone} ((Macros.mayPlayDeed "Play" You It (PlayRider Nothing Nothing Nothing False ItsOwnCost))) (Just Macros.untilYourNextEndStep)])

||| Brazen Cannonade
brazenCannonadePermission : Effect []
brazenCannonadePermission =
  Sequentially [Macros.exile You Macros.topCard,
                Continuously {ts = StaticFirstDone} ((Macros.mayPlayDeed "Play" You It (PlayRider Nothing Nothing Nothing False ItsOwnCost))) (Just (Until (EndOf Combat (Just You))))]


corneredCrook : Ability
corneredCrook =
  Macros.triggered When (Enters Macros.thisCreature Nothing)
    (Macros.mayWhen You (Macros.sacrifice You (Macros.a Macros.artifact))
                 (DealDamage This (Lit 3) (Macros.target Macros.anyTarget)))

theLastRoninII : Effect []
theLastRoninII =
  Reflexively (Macros.mills You (Lit 4) You)
              (Macros.move (Macros.target (And [Macros.creature, InZone (Macros.graveyardOf You)])) Macros.handZ)

thousandMoonsCrackshot : Ability
thousandMoonsCrackshot =
  Macros.triggered Whenever (Macros.attacks Macros.thisCreature)
    (Macros.mayWhen You (Pay You (Mana [Macros.generic 2, Macros.pip White]) PaidOnce)
                 (SetStatus Tapped (Macros.target Macros.creature)))

anointerOfValor : Ability
anointerOfValor =
  Macros.triggered Whenever (Macros.attacks (Macros.a Macros.creature))
    (Macros.mayWhen You (Pay You (Mana [Macros.generic 3]) PaidOnce)
                 (PutCounters (Lit 1) (PrintedKind Macros.plusOnePlusOne) (That (TypeW Creature))))


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
  Static (Macros.unlessSo (Macros.exists (And [Macros.artifact, HasPossessor ControllerAx You]))
                   (Macros.deontic Macros.thisCreature Forbid ["Attack"] Agent NoDeonticPatient))

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
       , Macros.activated (Do (RemoveCounters (Just (Macros.exactly 1)) (Just (PrintedKind Macros.plusOnePlusOne)) Macros.thisCreature))
                          (AddMana You (Lit 1) (Runs [[Colorless]]) []) ]
       (Just (0, 0))


counterspell : Effect []
counterspell = Macros.counterSpell (Macros.target Macros.spell)

beastWhisperer : Ability
beastWhisperer =
  Macros.triggered Whenever (Casts You (Macros.a (And [Macros.creature, Macros.spell])) Nothing) (Macros.draw You (Lit 1))

skaabRuinator : Ability
skaabRuinator =
  Static ((Macros.mayPlayDeed "Cast" You This (PlayRider (Macros.fromZ (Macros.graveyardOf You)) Nothing Nothing False ItsOwnCost)))

||| Raffine's Guidance
public export
raffinesGuidance : Card
raffinesGuidance =
  Macros.card "Raffine's Guidance" (Just [Macros.pip White]) []
       (MkTypeLine [enchantmentType "Aura"] [Enchantment])
       [ Macros.keywordSubject "Enchant" Macros.creature
       , Static (Gets (AttachHost Enchanted (TypeW Creature))
                      (PtUp (Lit 1)) (PtUp (Lit 1)))
       , Static ((Macros.mayPlayDeed "Cast" You This (PlayRider (Macros.fromZ (Macros.graveyardOf You)) Nothing Nothing False (Macros.paying (Mana [Macros.generic 2, Macros.pip White]))))) ]
       Nothing

||| Scourge of Nel Toth
public export
scourgeOfNelToth : Card
scourgeOfNelToth =
  Macros.card "Scourge of Nel Toth"
       (Just [Macros.generic 5, Macros.pip Black, Macros.pip Black]) []
       (MkTypeLine [creatureType "Zombie", creatureType "Dragon"] [Creature])
       [ Macros.keyword "Flying"
       , Static ((Macros.mayPlayDeed "Cast" You This (PlayRider (Macros.fromZ (Macros.graveyardOf You)) Nothing Nothing False (Macros.paying (Compound [ Mana [Macros.pip Black, Macros.pip Black]
                             , Do (Macros.sacrifice You
                                     (Macros.counted (Macros.exactly 2)
                                                   Macros.creature)) ]))))) ]
       (Just (6, 6))


escapeToTheWilds : Effect []
escapeToTheWilds =
  Sequentially [Macros.exile You (Macros.topCards 5),
                Continuously {ts = StaticFirstDone}
                  ((Macros.mayPlayDeed "Play" You (Macros.thoseVerbedThisWay "Exile" CardW) (PlayRider Nothing Nothing Nothing False ItsOwnCost)))
                  (Just (Until (EndOf Turn (Just You))))]


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
       [Static (Gets (Macros.allOf Macros.creatureYouControl) (PtUp (Lit 1)) (PtUp (Lit 1)))] Nothing

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
                   (Macros.putOntoBattlefieldUnderYourControl (Macros.allOf Macros.exiledWithThisArtifact))

coldStorage : Card
coldStorage =
  Macros.card "Cold Storage" (Just [Macros.generic 4]) [] (MkTypeLine [] [Artifact])
       [Macros.activated (Mana [Macros.generic 3]) (Macros.exile You (Macros.target Macros.creatureYouControl)),
        Macros.activated (Do (Macros.sacrifice You Macros.thisArtifact))
                                (Macros.putOntoBattlefieldUnderYourControl
                     (Macros.each (And [Macros.creature, Macros.exiledWithThisArtifact])))]
       Nothing

||| Muse Vessel
museVesselPlay : Ability
museVesselPlay =
  Macros.activated (Mana [Macros.generic 1])
                   (Sequentially [Macros.choose (Macros.a Macros.exiledWithThisArtifact),
                           Continuously {ts = StaticFirstDone} ((Macros.mayPlayDeed "Play" You (That CardW) (PlayRider Nothing Nothing Nothing False ItsOwnCost))) (Just Macros.thisTurn)])

aerialVolley : Card
aerialVolley =
  Macros.card "Aerial Volley" (Just [Macros.pip Green]) [] (MkTypeLine [] [Instant])
       [Spell (Macros.dealsDivided This (Lit 3)
                            (Macros.targets (Macros.oneThrough 3)
                              (And [Macros.creature, HasKeyword (TheKeyword "Flying")])))] Nothing

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
avianOddity = PutCounters (Lit 1) (PrintedKind Macros.flyingCounter) (Macros.target Macros.creatureYouControl)

||| Song of Eärendil
songOfEarendil : Effect []
songOfEarendil =
  PutCounters (Lit 1) (PrintedKind Macros.flyingCounter)
              (Macros.each (And [Macros.creature, HasPossessor ControllerAx You, Not (HasKeyword (TheKeyword "Flying"))]))


deathByDragons : Effect []
deathByDragons =
  Create (Macros.each (And [AnyPlayer, OtherThan (Macros.target AnyPlayer)])) (Lit 1)
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
                   (StatusEvent (Macros.a (And [Macros.creature, HasPossessor ControllerAx Macros.anOpponent])) Tapped)
                   (PutCounters (Lit 1) (PrintedKind Macros.plusOnePlusOne) Macros.thisCreature)

mesmericOrb : Ability
mesmericOrb =
  Macros.triggered Whenever (StatusEvent (Macros.a Permanent) Untapped)
                   (Macros.mills (Macros.controllerOf (That PermanentW)) (Lit 1) They)


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
                                               HasPossessor ControllerAx Macros.anOpponent]))

secretPlans : Ability
secretPlans =
  Macros.triggered Whenever
                   (StatusEvent (Macros.a (And [Permanent, HasPossessor ControllerAx You])) FaceUp)
                   (Macros.draw You (Lit 1))

vodalianIllusionist : Ability
vodalianIllusionist =
  Macros.activated (Compound [Mana [Macros.pip Blue, Macros.pip Blue], TapSymbol])
                   (SetStatus PhasedOut (Macros.target Macros.creature))

||| Teferi's Imp
teferisImpPhasesOut : Ability
teferisImpPhasesOut =
  Macros.triggered Whenever (StatusEvent Macros.thisCreature PhasedOut)
                   ((Macros.discard You (Macros.a (InZone Macros.handZ))))

||| Teferi's Imp
teferisImpPhasesIn : Ability
teferisImpPhasesIn =
  Macros.triggered Whenever (StatusEvent Macros.thisCreature PhasedIn)
                   (Macros.draw You (Lit 1))

||| Oubliette
oubliette : Ability
oubliette =
  Macros.triggered When (Enters Macros.thisEnchantment Nothing)
                   (Macros.phasesOutUntil (Macros.target Macros.creature)
                                   (Macros.leavesBattlefield Macros.thisEnchantment))

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
  Macros.triggered When (Enters Macros.thisCreature Nothing)
                   (RemoveFromCombat (Macros.target (And [Macros.creature, Or [Attacking, Blocking]])))

netcasterSpider : Ability
netcasterSpider =
  Macros.triggered Whenever
                   (Blocks Macros.thisCreature
                    (Just (Macros.a (And [Macros.creature, HasKeyword (TheKeyword "Flying")]))))
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
                   (Macros.destroy (Macros.allOf (And [Macros.creature,
                                         Or [CombatRel BlockerOf Macros.thisCreature,
                                             CombatRel BlockedBy Macros.thisCreature]])))

vertigoSpawn : Ability
vertigoSpawn =
  Macros.triggered Whenever (Blocks Macros.thisCreature (Just (Macros.a Macros.creature)))
                   (Sequentially [SetStatus Tapped (That (TypeW Creature)),
                           DoesntUntapNext (That (TypeW Creature)) (Lit 1)])


munghaWurm : Ability
munghaWurm = Static (Macros.cantMoreThan You "Untap" 1 Macros.land)

dampingField : Ability
dampingField = Static (Macros.cantMoreThan (PlayerGroup AllPlayers) "Untap" 1 Macros.artifact)

smoke : Ability
smoke = Static (Macros.cantMoreThan (PlayerGroup AllPlayers) "Untap" 1 Macros.creature)

winterOrb : Ability
winterOrb =
  Static (Macros.asLongAs (Matches Macros.thisArtifact Macros.untapped)
                          (Macros.cantMoreThan (PlayerGroup AllPlayers) "Untap" 1 Macros.land))

staticOrb : Ability
staticOrb =
  Static (Macros.asLongAs (Matches Macros.thisArtifact Macros.untapped)
                          (Macros.cantMoreThan (PlayerGroup AllPlayers) "Untap" 2 Permanent))

||| Rule of Law
public export
ruleOfLaw : Card
ruleOfLaw =
  Macros.card "Rule of Law" (Just [Macros.generic 2, Macros.pip White]) []
       (MkTypeLine [] [Enchantment])
       [ Static (Macros.cantMoreThan (PlayerGroup AllPlayers) "Cast" 1 Macros.spell) ]
       Nothing

||| Spirit of the Labyrinth
public export
spiritOfTheLabyrinth : Card
spiritOfTheLabyrinth =
  Macros.card "Spirit of the Labyrinth"
       (Just [Macros.generic 1, Macros.pip White]) []
       (MkTypeLine [creatureType "Spirit"] [Enchantment, Creature])
       [ Static (Macros.cantMoreThan (PlayerGroup AllPlayers) "DrawCard" 1 IsCard) ]
       (Just (3, 1))


prologueToPhyresis : Effect []
prologueToPhyresis = PutCounters (Lit 1) (PrintedKind (Named "Poison")) (Macros.each Opponent)

screechingScorchbeast : Ability
screechingScorchbeast =
  Macros.triggered Whenever (Macros.attacks Macros.thisCreature)
                   (PutCounters (Lit 2) (PrintedKind (Named "Rad")) (Macros.each AnyPlayer))

merenOfClanNelToth : Ability
merenOfClanNelToth =
  Macros.triggered Whenever (Dies (Macros.a (Macros.otherCreatureYouControl Macros.thisCreature)))
                   (PutCounters (Lit 1) (PrintedKind (Named "Experience")) You)

||| Bumi, King of Three Trials
bumiScryMode : Effect []
bumiScryMode =
  Macros.scry (Macros.target AnyPlayer) (Lit 3)

||| Final Act
finalActCounterMode : Effect []
finalActCounterMode = Macros.losesAllCounters (Macros.each Opponent) Nothing

leeches : Effect []
leeches = Macros.losesAllCounters (Macros.target AnyPlayer) (Just (PrintedKind (Named "Poison")))

kratosStoicFather : Ability
kratosStoicFather =
  Macros.triggered At (BeginningOf EndStep (Macros.yours))
                   (PutCounters (CountersOn (Named "Experience") You) (PrintedKind Macros.plusOnePlusOne)
                         (Macros.target Macros.creature))


orneryDilophosaur : Ability
orneryDilophosaur =
  Macros.triggeredIf Whenever
                     (Macros.attacks Macros.thisCreature)
                     (Macros.exists (And [Macros.creature, HasPossessor ControllerAx You,
                                   Compare [CharAxis Power] AtLeast (Lit 4)]))
                     (Macros.gets Macros.thisCreature (PtUp (Lit 2)) (PtUp (Lit 2)) (Just Macros.untilEndOfTurn))

incisorGlider : Ability
incisorGlider =
  Macros.triggeredIf Whenever
                     (Macros.attacks Macros.thisCreature)
                     (CompareAmt (CountersOn (Named "Poison") (Macros.a Opponent))
                                 AtLeast (Lit 3))
                     (Continuously {ts = StaticFirstDone} (Gets (Macros.allOf Macros.creatureYouControl) (PtUp (Lit 1)) (PtUp (Lit 1)))
                                   (Just Macros.untilEndOfTurn))


stormFleetSpy : Ability
stormFleetSpy =
  Macros.triggeredIf When
                     (Enters Macros.thisCreature Nothing)
                     (Macros.happened AttackDeclaration You Lookback.ThisTurn)
                     (Macros.draw You (Lit 1))

vashtaNerada : Ability
vashtaNerada =
  Macros.triggeredIf At
                     (BeginningOf EndStep (Macros.eachPlayers))
                     (Macros.happened Death (Macros.a Macros.creature)
                                      Lookback.ThisTurn)
                     (PutCounters (Lit 1) (PrintedKind Macros.plusOnePlusOne) Macros.thisCreature)

||| Tippy-Toe, Terrific Partner
tippyToe : Ability
tippyToe =
  Macros.triggeredIf At
                     (BeginningOf EndStep (Macros.yours))
                     (Macros.happened LifeGain You Lookback.ThisTurn)
                     (Macros.draw You (Lit 1))

loanShark : Ability
loanShark =
  Macros.triggeredIf When
                     (Enters Macros.thisCreature Nothing)
                     (CompareAmt (Macros.eventCount SpellCast You Lookback.ThisTurn)
                                 AtLeast (Lit 2))
                     (Macros.draw You (Lit 1))


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
  Macros.destroy (Macros.allOf (And [Macros.creature,
                              Macros.happenedTo Entry Lookback.ThisTurn]))

furiousSpinesplitter : Ability
furiousSpinesplitter =
  Macros.triggered At (BeginningOf EndStep (Macros.yours))
                   (PutCounters (Macros.forEach (And [Opponent,
                                               Macros.happenedTo DamageTaken Lookback.ThisTurn]))
                         (PrintedKind Macros.plusOnePlusOne) Macros.thisCreature)


winterMoon : Ability
winterMoon =
  Static (Macros.cantMoreThan (PlayerGroup AllPlayers) "Untap" 1
                         (And [Macros.land, Not (HasSupertype Basic)]))

cradleToGrave : Effect []
cradleToGrave =
  Macros.destroy (Macros.target (And [Macros.creature, Not (ColorIs Black),
                                      Macros.happenedTo Entry Lookback.ThisTurn]))

herosDemise : Effect []
herosDemise =
  Macros.destroy (Macros.target (And [Macros.creature, HasSupertype Legendary]))

||| Concordant Crossroads
concordantCrossroads : Card
concordantCrossroads =
  Macros.card "Concordant Crossroads" (Just [Macros.pip Green]) [World]
       (MkTypeLine [] [Enchantment])
       [ Static (Gains (Macros.allOf Macros.creature) (Macros.keyword "Haste")) ] Nothing

||| Pure // Simple
simpleHalf : Effect []
simpleHalf = Macros.destroy (Macros.target (And [Permanent, Multicolored]))

leitmotifComposer : Ability
leitmotifComposer =
  Macros.activated (Mana [Macros.generic 2, Macros.pip Blue])
                   (Continuously {ts = StaticFirstDone} (Macros.deontic (Macros.allOf (And [Macros.creature,
                                                Named (PrintedName "Leitmotif Composer")]))
                                   Forbid ["Block"] Patient NoDeonticPatient)
                          (Just Macros.thisTurn))


paladinOfAtonement : Ability
paladinOfAtonement =
  Macros.triggeredIf At
                     (BeginningOf Upkeep (Macros.eachPlayers))
                     (Macros.happened LifeLoss You Lookback.LastTurn)
                     (PutCounters (Lit 1) (PrintedKind Macros.plusOnePlusOne) Macros.thisCreature)

brazenCannonade : Ability
brazenCannonade =
  Macros.triggeredIf At
                     (BeginningOf PostcombatMain (Macros.yours))
                     (Macros.happened AttackDeclaration You Lookback.ThisTurn)
                     (Macros.exile You Macros.topCard)

fourKnocks : Ability
fourKnocks =
  Macros.triggered At (BeginningOf FirstMain (Macros.yours)) (Macros.draw You (Lit 1))

hammerOfBogardan : Ability
hammerOfBogardan =
  Macros.activatedOnlyDuring (Mana [Macros.generic 2, Macros.pip Red, Macros.pip Red, Macros.pip Red])
                             (Macros.move This Macros.handZ)
                             (DuringPart Upkeep (Just You))


berserkersOfBloodRidge : Ability
berserkersOfBloodRidge = Static (Macros.deontic Macros.thisCreature Require ["Attack"] Agent NoDeonticPatient)

trumpetingArmodon : Ability
trumpetingArmodon =
  Macros.activated (Mana [Macros.generic 1, Macros.pip Green])
                   (Continuously {ts = StaticFirstDone} (Macros.deontic (Macros.target Macros.creature) Require ["Block"] Agent
                                   (DeonticCounterpart Macros.thisCreature))
                          (Just Macros.thisTurn))

loathsomeCatoblepas : Ability
loathsomeCatoblepas =
  Macros.activated (Mana [Macros.generic 2, Macros.pip Green])
                   (Continuously {ts = StaticFirstDone} (Macros.deontic Macros.thisCreature Require ["Block"] Patient NoDeonticPatient)
                          (Just Macros.thisTurn))

ashnodsBattleGear : Ability
ashnodsBattleGear = Static (Macros.mayDeclineUntap Macros.thisArtifact (Just You))


throneWarden : Ability
throneWarden =
  Macros.triggeredIf At
                     (BeginningOf EndStep (Macros.yours))
                     (Matches You (HasDesignation Monarch))
                     (PutCounters (Lit 1) (PrintedKind Macros.plusOnePlusOne) Macros.thisCreature)

aragornKingOfGondor : Ability
aragornKingOfGondor =
  Macros.triggered When (Enters Macros.thisCreature Nothing) (Macros.gainsDesignation You Monarch Instructed)

goadTargetCreature : Ability
goadTargetCreature =
  Macros.activated (Compound [Mana [Macros.generic 3], TapSymbol])
                   (Macros.gainsDesignation (Macros.target Macros.creature) Goaded Instructed)

||| Frenzied Gorespawn
public export
frenziedGorespawnGoad : Effect []
frenziedGorespawnGoad =
  ForEachOf (Macros.each Opponent)
    (Macros.gainsDesignation
       (Macros.target (And [Macros.creature, HasPossessor ControllerAx (That PlayerW)]))
       Goaded Instructed)

goadedAttackTrigger : Ability
goadedAttackTrigger =
  Macros.triggered Whenever (Macros.attacks (Macros.a (And [Macros.creature, HasDesignation Goaded])))
                   (DealDamage It (Lit 1) (Macros.controllerOf It))

firmamentSage : Ability
firmamentSage = Macros.triggered Whenever DayNightShift (Macros.draw You (Lit 1))


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
  Static (Macros.deontic Macros.thisCreature (GatedBy (Mana [Macros.generic 1]))
                  ["Block"] Agent
                  (DeonticCounterpart (Macros.allOf (And [Macros.creature,
                                                   Compare [CharAxis Power] AtLeast (Lit 3)]))))


frodoBaggins : Ability
frodoBaggins =
  Static (Macros.asLongAs (Matches Macros.thisCreature (HasDesignation RingBearer))
                          (Macros.deontic It Require ["Block"] Patient NoDeonticPatient))

adantoVanguard : Ability
adantoVanguard =
  Static (Macros.asLongAs (Matches Macros.thisCreature Attacking)
                          (Gets It (PtUp (Lit 2)) (PtUp (Lit 0))))


bloodshedFever : Card
bloodshedFever =
  Macros.card "Bloodshed Fever" (Just [Macros.pip Red]) []
       (MkTypeLine [enchantmentType "Aura"] [Enchantment])
       [ Macros.keywordSubject "Enchant" Macros.creature
       , Static (Macros.deontic (AttachHost Enchanted (TypeW Creature))
                         Require ["Attack"] Agent NoDeonticPatient) ]
       Nothing

||| Apathy
public export
apathy : Card
apathy =
  Macros.card "Apathy" (Just [Macros.pip Blue]) []
       (MkTypeLine [enchantmentType "Aura"] [Enchantment])
       [ Macros.keywordSubject "Enchant" Macros.creature
       , Static (Macros.doesntUntap (AttachHost Enchanted (TypeW Creature))
                   (Just (Macros.controllerOf (AttachHost Enchanted (TypeW Creature)))))
       , Macros.triggered At
           (Macros.beginningOfPossessed Upkeep
              (Macros.controllerOf (AttachHost Enchanted (TypeW Creature))))
           (May (That PlayerW)
                (Macros.discard (That PlayerW) (Macros.aAtRandom (InZone Macros.handZ)))
                (Just (Macros.untap (That (TypeW Creature))))
                Nothing) ]
       Nothing

||| Bristlepack Sentry
public export
bristlepackSentry : Card
bristlepackSentry =
  Macros.card "Bristlepack Sentry" (Just [Macros.generic 1, Macros.pip Green]) []
       (MkTypeLine [creatureType "Plant", creatureType "Wolf"] [Creature])
       [ Macros.keyword "Defender"
       , Static (Macros.asLongAs
                   (Macros.exists (And [Macros.creature, HasPossessor ControllerAx You,
                                 Compare [CharAxis Power] AtLeast (Lit 4)]))
                   (Macros.canDoAsThough Macros.thisCreature "Attack"
                                         (Not (HasKeyword (TheKeyword "Defender"))))) ]
       (Just (3, 3))

||| Pacifism
public export
pacifism : Card
pacifism =
  Macros.card "Pacifism" (Just [Macros.generic 1, Macros.pip White]) []
       (MkTypeLine [enchantmentType "Aura"] [Enchantment])
       [ Macros.keywordSubject "Enchant" Macros.creature
       , Static (Macros.deontic (AttachHost Enchanted (TypeW Creature))
                                Forbid ["Attack", "Block"] Agent NoDeonticPatient) ]
       Nothing

||| Everybody Lives!
public export
everybodyLivesGateLine : Effect []
everybodyLivesGateLine =
  Continuously {ts = StaticFirstDone} (Macros.deontic (PlayerGroup AllPlayers) Forbid ["LoseGame", "WinGame"]
                               Agent NoDeonticPatient)
               (Just Macros.thisTurn)

||| Gaea's Revenge
public export
gaeasRevenge : Card
gaeasRevenge =
  Macros.card "Gaea's Revenge"
       (Just [Macros.generic 5, Macros.pip Green, Macros.pip Green]) []
       (MkTypeLine [creatureType "Elemental"] [Creature])
       [ Static (Macros.objectCant "Counter" This)
       , Macros.keyword "Haste"
       , Static (Macros.cantBeTargetedBy Macros.thisCreature
                   (Macros.allOf (Joined (And [Macros.spell, Not (ColorIs Green)])
                                  (AbilityOf (Macros.allOf (And [IsSource,
                                                          Not (ColorIs Green)])))))) ]
       (Just (8, 5))

||| Nowhere to Run
public export
nowhereToRunTargetLine : StaticEffect []
nowhereToRunTargetLine =
  Macros.canBeTargetedAsThough (Macros.allOf Macros.creatureYourOpponentsControl)
    (Macros.allOf (Joined Macros.spell (AbilityHead AnyOnStack)))
    (Not (HasKeyword (TheKeyword "Hexproof")))

||| Nowhere to Run
public export
nowhereToRunWardLine : StaticEffect []
nowhereToRunWardLine =
  Macros.deontic
    (Macros.allOf (And [ AbilityHead (KeywordClass "Ward")
                , AbilityOf (Macros.allOf Macros.creatureYourOpponentsControl) ]))
    Forbid ["Trigger"] Agent NoDeonticPatient

||| Hithlain Rope
public export
hithlainRopeSacrificeLock : StaticEffect []
hithlainRopeSacrificeLock = Macros.objectCant "Sacrifice" This

||| Display of Power
public export
displayOfPowerCopyLock : StaticEffect []
displayOfPowerCopyLock = Macros.objectCant "Copy" This

||| Mornsong Aria
public export
mornsongAriaLock : StaticEffect []
mornsongAriaLock =
  Macros.deontic (PlayerGroup AllPlayers) Forbid ["DrawCard", "GainLife"]
                 Agent NoDeonticPatient

public export
opponentsCantGainLife : StaticEffect []
opponentsCantGainLife = Macros.playerCant "GainLife" (PlayerGroup YourOpponents)

brainwash : Card
brainwash =
  Macros.card "Brainwash" (Just [Macros.pip White]) []
       (MkTypeLine [enchantmentType "Aura"] [Enchantment])
       [ Macros.keywordSubject "Enchant" Macros.creature
       , Static (Macros.deontic (AttachHost Enchanted (TypeW Creature))
                         (GatedBy (Mana [Macros.generic 3])) ["Attack"] Agent NoDeonticPatient) ]
       Nothing

enkiraHostileScavenger : Ability
enkiraHostileScavenger =
  Static (Macros.asLongAs (Matches Macros.thisCreature (IsAttached Equipped))
                          (Macros.deontic It Require ["Block"] Patient NoDeonticPatient))

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
lichsMasteryGate = Static (Macros.playerCant "LoseGame" You)

lichsMasteryLoss : Ability
lichsMasteryLoss =
  Macros.triggered When (Macros.leavesBattlefield Macros.thisEnchantment) (Concludes LoseGame You)

phageTheUntouchable : Ability
phageTheUntouchable =
  Macros.triggered Whenever
                   (Macros.dealsCombatDamage Macros.thisCreature (Macros.a AnyPlayer))
                   (Concludes LoseGame (That PlayerW))


platinumAngel : Card
platinumAngel =
  Macros.card "Platinum Angel" (Just [Macros.generic 7]) []
       (MkTypeLine [creatureType "Angel"] [Artifact, Creature])
       [ Macros.keyword "Flying"
       , Static (Macros.playerCant "LoseGame" You)
       , Static (Macros.playerCant "WinGame" (PlayerGroup YourOpponents)) ]
       (Just (4, 4))

abyssalPersecutor : Card
abyssalPersecutor =
  Macros.card "Abyssal Persecutor"
       (Just [Macros.generic 2, Macros.pip Black, Macros.pip Black]) []
       (MkTypeLine [creatureType "Demon"] [Creature])
       [ Macros.keyword "Flying"
       , Macros.keyword "Trample"
       , Static (Macros.playerCant "WinGame" You)
       , Static (Macros.playerCant "LoseGame" (PlayerGroup YourOpponents)) ]
       (Just (6, 6))


smogElemental : Card
smogElemental =
  Macros.card "Smog Elemental"
       (Just [Macros.generic 4, Macros.pip Black, Macros.pip Black]) []
       (MkTypeLine [creatureType "Elemental"] [Creature])
       [ Macros.keyword "Flying"
       , Static (Gets (Macros.allOf (And [Macros.creature, HasKeyword (TheKeyword "Flying"),
                                   HasPossessor ControllerAx (PlayerGroup YourOpponents)]))
                      (PtDown (Lit 1)) (PtDown (Lit 1))) ]
       (Just (3, 3))

anglerTurtle : Ability
anglerTurtle =
  Static (Macros.deontic (Macros.allOf Macros.creatureYourOpponentsControl) Require ["Attack"] Agent NoDeonticPatient)


bloodTyrant : Ability
bloodTyrant =
  Macros.triggered Whenever (LosesGame (Macros.a AnyPlayer))
                   (PutCounters (Lit 5) (PrintedKind Macros.plusOnePlusOne) Macros.thisCreature)

theGoldenThrone : Ability
theGoldenThrone =
  Static (Intercepts (LosesGame You) [] Nothing
                     (Sequentially [Macros.exile You Macros.thisArtifact,
                                    ChangeLife You (Set (Lit 1))])
                     Repeatedly Nothing)

stunningReversal : Ability
stunningReversal =
  Spell (Continuously {ts = StaticFirstDone} (Intercepts (LosesGame You) [] Nothing
                                  (Sequentially [Draw You (Lit 7),
                                                 ChangeLife You (Set (Lit 1))])
                                  NextTimeOnly Nothing)
                      (Just Macros.thisTurn))


felidarSovereign : Card
felidarSovereign =
  Macros.card "Felidar Sovereign"
       (Just [Macros.generic 4, Macros.pip White, Macros.pip White]) []
       (MkTypeLine [creatureType "Cat", creatureType "Beast"] [Creature])
       [ Macros.keyword "Vigilance"
       , Macros.keyword "Lifelink"
       , Macros.triggeredIf At
                            (BeginningOf Upkeep (Macros.yours))
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
       , Static (Intercepts (LosesGame You) [] Nothing
                            (Sequentially [Macros.exile You Macros.thisCreature,
                                           ChangeLife You (Set (PlayerStatOf StartingLifeTotal You))])
                            Repeatedly Nothing) ]
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
                  , If (CompareAmt (Macros.countOf Macros.creatureYouControl) Less
                                   (Macros.countOf (And [Macros.creature,
                                                  HasPossessor ControllerAx Macros.anOpponent])))
                       (Macros.create (Lit 3) (Macros.creatureTok 1 1 [White] [creatureType "Soldier"]))
                       Nothing ]) ]
       Nothing

survivalCache : Effect []
survivalCache =
  Sequentially [ Macros.gainsLife You (Lit 2)
               , If (CompareAmt (PlayerStatOf LifeTotal You) Greater
                                (PlayerStatOf LifeTotal Macros.anOpponent))
                    (Macros.draw You (Lit 1))
                    Nothing ]

pathOfBravery : Ability
pathOfBravery =
  Static (Macros.asLongAs (CompareAmt (PlayerStatOf LifeTotal You) AtLeast
                                      (PlayerStatOf StartingLifeTotal You))
                          (Gets (Macros.allOf Macros.creatureYouControl) (PtUp (Lit 1)) (PtUp (Lit 1))))

ensnaringBridge : Card
ensnaringBridge =
  Macros.card "Ensnaring Bridge" (Just [Macros.generic 3]) []
       (MkTypeLine [] [Artifact])
       [ Static (Macros.deontic (Macros.allOf (And [Macros.creature,
                                      Compare [CharAxis Power] Greater
                                              (Macros.countOf (InZone (Macros.handOf You)))]))
                         Forbid ["Attack"] Agent NoDeonticPatient) ]
       Nothing

elderscaleWurm : Ability
elderscaleWurm =
  Macros.triggeredIf When
                     (Enters Macros.thisCreature Nothing)
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
                            (BeginningOf Combat (Macros.eachPlayers))
                            (CompareAmt (PlayerStatOf LifeTotal You) Greater
                                        (PlayerStatOf LifeTotal Macros.anOpponent))
                            (Macros.gains Macros.thisCreature (Macros.keyword "DoubleStrike")
                                          (Just Macros.untilEndOfTurn)) ]
       (Just (5, 5))


||| Damia, Sage of Stone
damia : Ability
damia =
  Macros.triggeredIf At
                     (BeginningOf Upkeep (Macros.yours))
                     (CompareAmt (Macros.countOf (InZone (Macros.handOf You)))
                                 Less (Lit 7))
                     (Draw You TheDifference)

||| Krang, Master Mind
krang : Ability
krang =
  Macros.triggeredIf When
                     (Enters Macros.thisCreature Nothing)
                     (CompareAmt (Macros.countOf (InZone (Macros.handOf You)))
                                 Less (Lit 4))
                     (Draw You TheDifference)


||| Krenko, Mob Boss
krenko : Ability
krenko =
  Macros.activated TapSymbol
       (Sequentially
          [ Create You (LetterVal X)
                   (TokenWritten (Macros.creatureTok 1 1 [Red] [creatureType "Goblin"])) []
          , Define X (Macros.countOf (And [HasSubtype (creatureType "Goblin"), HasPossessor ControllerAx You])) ])

chainReaction : Card
chainReaction =
  Macros.card "Chain Reaction"
       (Just [Macros.generic 2, Macros.pip Red, Macros.pip Red]) []
       (MkTypeLine [] [Sorcery])
       [ Spell (Sequentially
                  [ DealDamage This (LetterVal X) (Macros.each Macros.creature)
                  , Define X (Macros.countOf Macros.creature) ]) ]
       Nothing

ivoryTower : Card
ivoryTower =
  Macros.card "Ivory Tower" (Just [Macros.generic 1]) []
       (MkTypeLine [] [Artifact])
       [ Macros.triggered At (BeginningOf Upkeep (Macros.yours))
                  (Sequentially
                     [ Macros.gainsLife You (LetterVal X)
                     , Define X (Minus (Macros.countOf (InZone (Macros.handOf You)))
                                (Lit 4)) ]) ]
       Nothing

||| Black Vise
public export
blackVise : Card
blackVise =
  Macros.card "Black Vise" (Just [Macros.generic 1]) []
       (MkTypeLine [] [Artifact])
       [ Static (Macros.entersChoosingPlayer Macros.thisArtifact (Just OpponentsOnly))
       , Macros.triggered At
           (Macros.beginningOfPossessed Upkeep (Macros.the ChosenPlayer))
           (Sequentially
              [ DealDamage Macros.thisArtifact (LetterVal X) (Macros.the ChosenPlayer)
              , Define X (Minus (Macros.countOf (InZone (Macros.handOf (Macros.the ChosenPlayer))))
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
                  , Define X (Macros.countOf Macros.creatureYouControl) ]) ]
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
                  , Define X (Macros.countOf (InZone (Macros.handOf You))) ]) ]
       Nothing

adelbertSteiner : Card
adelbertSteiner =
  Macros.card "Adelbert Steiner" (Just [Macros.generic 1, Macros.pip White])
       [Legendary] (MkTypeLine [creatureType "Human", creatureType "Knight"] [Creature])
       [ Macros.keyword "Lifelink"
       , Static (Gets Macros.thisCreature
               (PtUp (Macros.forEach (And [HasSubtype (artifactType "Equipment"), HasPossessor ControllerAx You])))
               (PtUp (Macros.forEach (And [HasSubtype (artifactType "Equipment"), HasPossessor ControllerAx You])))) ]
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
          , Define X (Macros.countOf (And [Macros.land, HasPossessor ControllerAx You])) ])


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
                  , Define X (Macros.countOf (InZone (Macros.graveyardOf You))) ])

stagBeetle : Card
stagBeetle =
  Macros.card "Stag Beetle"
       (Just [Macros.generic 3, Macros.pip Green, Macros.pip Green]) []
       (MkTypeLine [creatureType "Insect"] [Creature])
       [ Static (AndAlso [ Macros.entersWithCounters Macros.thisCreature
                                                     (LetterVal X)
                                                     Macros.plusOnePlusOne
                         , Define X
                             (Macros.countOf (Macros.otherCreature Macros.thisCreature)) ]) ]
       (Just (0, 0))


acceleratedMutation : Card
acceleratedMutation =
  Macros.card "Accelerated Mutation"
       (Just [Macros.generic 3, Macros.pip Green, Macros.pip Green]) []
       (MkTypeLine [] [Instant])
       [ Spell (Sequentially
                  [ Continuously {ts = StaticFirstDone} (Gets (Macros.target Macros.creature)
                                       (PtUp (LetterVal X)) (PtUp (LetterVal X)))
                                 (Just Macros.untilEndOfTurn)
                  , Define X (Macros.aggregate MaxOf (CharAxis ManaValue)
                                 (And [Permanent, HasPossessor ControllerAx You])) ]) ]
       Nothing

carrionGrub : Card
carrionGrub =
  Macros.card "Carrion Grub" (Just [Macros.generic 3, Macros.pip Black]) []
       (MkTypeLine [creatureType "Insect"] [Creature])
       [ Static (AndAlso [ Gets Macros.thisCreature
                                (PtUp (LetterVal X)) (PtUp (Lit 0))
                         , Define X
                             (Macros.aggregate MaxOf (CharAxis Power)
                                        (And [Macros.creature,
                                              InZone (Macros.graveyardOf You)])) ])
       , Macros.triggered When (Enters Macros.thisCreature Nothing)
                        (Macros.mills You (Lit 4) You) ]
       (Just (0, 5))

repayInKind : Card
repayInKind =
  Macros.card "Repay in Kind"
       (Just [Macros.generic 5, Macros.pip Black, Macros.pip Black]) []
       (MkTypeLine [] [Sorcery])
       [ Spell (Macros.lifeTotalBecomes (Macros.each AnyPlayer)
                  (Macros.aggregate MinOf (PlayerStatAxis LifeTotal) AnyPlayer)) ]
       Nothing

toweringTitan : Ability
toweringTitan =
  Static (AndAlso [ Macros.entersWithCounters Macros.thisCreature
                                              (LetterVal X)
                                              Macros.plusOnePlusOne
                  , Define X
                      (Macros.aggregate SumOf (CharAxis Toughness)
                         (Macros.otherCreatureYouControl Macros.thisCreature)) ])


ghalta : Card
ghalta =
  Macros.card "Ghalta, Primal Hunger"
       (Just [Macros.generic 10, Macros.pip Green, Macros.pip Green]) [Legendary]
       (MkTypeLine [creatureType "Elder", creatureType "Dinosaur"] [Creature])
       [ Static (AndAlso [ CostsToCast This (CostLess (LetterVal X) Nothing)
                         , Define X
                             (Macros.aggregate SumOf (CharAxis Power)
                                        Macros.creatureYouControl) ])
       , Macros.keyword "Trample" ]
       (Just (12, 12))

ancientStoneIdol : Ability
ancientStoneIdol =
  Static (CostsToCast This
            (CostLess (Macros.forEach (And [Macros.creature, Attacking])) Nothing))

thornOfAmethyst : Card
thornOfAmethyst =
  Macros.card "Thorn of Amethyst" (Just [Macros.generic 2]) []
       (MkTypeLine [] [Artifact])
       [ Static (CostsToCast (Macros.allOf (And [Not Macros.creature, Macros.spell]))
                             (CostMore (Lit 1))) ]
       Nothing

ferozsBan : Card
ferozsBan =
  Macros.card "Feroz's Ban" (Just [Macros.generic 6]) []
       (MkTypeLine [] [Artifact])
       [ Static (CostsToCast (Macros.allOf (And [Macros.creature, Macros.spell]))
                             (CostMore (Lit 2))) ]
       Nothing

urzasFilter : Card
urzasFilter =
  Macros.card "Urza's Filter" (Just [Macros.generic 4]) []
       (MkTypeLine [] [Artifact])
       [ Static (CostsToCast (Macros.allOf (And [Multicolored, Macros.spell]))
                             (CostLess (Lit 2) Nothing)) ]
       Nothing


emeraldMedallion : Card
emeraldMedallion =
  Macros.card "Emerald Medallion" (Just [Macros.generic 2]) []
       (MkTypeLine [] [Artifact])
       [ Static (CostsToCast (Macros.allOf (And [ColorIs Green, Macros.spell, CastBy You]))
                             (CostLess (Lit 1) Nothing)) ]
       Nothing

||| Highspire Bell-Ringer
public export
highspireBellRinger : Card
highspireBellRinger =
  Macros.card "Highspire Bell-Ringer"
       (Just [Macros.generic 2, Macros.pip Blue]) []
       (MkTypeLine [creatureType "Djinn", creatureType "Monk"] [Creature])
       [ Macros.keyword "Flying"
       , Static (CostsToCast
                   (Macros.the (And [Macros.spell,
                                   NthCastBy (Nth 2) You (RankEach Turn)]))
                   (CostLess (Lit 1) Nothing)) ]
       (Just (1, 4))

||| Once Upon a Time
public export
onceUponATimeFirstCast : Predicate [] Object
onceUponATimeFirstCast = NthCastBy (Nth 1) You (RankWithin Lookback.ThisGame)

foundryInspector : Card
foundryInspector =
  Macros.card "Foundry Inspector" (Just [Macros.generic 3]) []
       (MkTypeLine [creatureType "Construct"] [Artifact, Creature])
       [ Static (CostsToCast (Macros.allOf (And [Macros.artifact, Macros.spell, CastBy You]))
                             (CostLess (Lit 1) Nothing)) ]
       (Just (3, 2))

daruWarchief : Card
daruWarchief =
  Macros.card "Daru Warchief"
       (Just [Macros.generic 2, Macros.pip White, Macros.pip White]) []
       (MkTypeLine [creatureType "Human", creatureType "Soldier"] [Creature])
       [ Static (CostsToCast (Macros.allOf (And [HasSubtype (creatureType "Soldier"), Macros.spell, CastBy You]))
                             (CostLess (Lit 1) Nothing))
       , Static (Gets (Macros.allOf (And [HasSubtype (creatureType "Soldier"), Macros.creature, HasPossessor ControllerAx You]))
                      (PtUp (Lit 1)) (PtUp (Lit 2))) ]
       (Just (1, 1))

grandArbiter : Card
grandArbiter =
  Macros.card "Grand Arbiter Augustin IV"
       (Just [Macros.generic 2, Macros.pip White, Macros.pip Blue]) [Legendary]
       (MkTypeLine [creatureType "Human", creatureType "Advisor"] [Creature])
       [ Static (CostsToCast (Macros.allOf (And [ColorIs White, Macros.spell, CastBy You]))
                             (CostLess (Lit 1) Nothing))
       , Static (CostsToCast (Macros.allOf (And [ColorIs Blue, Macros.spell, CastBy You]))
                             (CostLess (Lit 1) Nothing))
       , Static (CostsToCast (Macros.allOf (And [Macros.spell,
                                          CastBy (PlayerGroup YourOpponents)]))
                             (CostMore (Lit 1))) ]
       (Just (2, 3))


goblinElectromancer : Card
goblinElectromancer =
  Macros.card "Goblin Electromancer"
       (Just [Macros.pip Blue, Macros.pip Red]) []
       (MkTypeLine [creatureType "Goblin", creatureType "Wizard"] [Creature])
       [ Static (CostsToCast (Macros.allOf (And [Macros.instantOrSorcery, Macros.spell,
                                          CastBy You]))
                             (CostLess (Lit 1) Nothing)) ]
       (Just (2, 2))

arcaneMelee : Card
arcaneMelee =
  Macros.card "Arcane Melee" (Just [Macros.generic 4, Macros.pip Blue]) []
       (MkTypeLine [] [Enchantment])
       [ Static (CostsToCast (Macros.allOf (And [Macros.instantOrSorcery, Macros.spell]))
                             (CostLess (Lit 2) Nothing)) ]
       Nothing

manaMatrix : Card
manaMatrix =
  Macros.card "Mana Matrix" (Just [Macros.generic 6]) []
       (MkTypeLine [] [Artifact])
       [ Static (CostsToCast (Macros.allOf (And [Or [Macros.instant, Macros.enchantment],
                                          Macros.spell, CastBy You]))
                             (CostLess (Lit 2) Nothing)) ]
       Nothing

auraOfSilence : Card
auraOfSilence =
  Macros.card "Aura of Silence"
       (Just [Macros.generic 1, Macros.pip White, Macros.pip White]) []
       (MkTypeLine [] [Enchantment])
       [ Static (CostsToCast (Macros.allOf (And [Or [Macros.artifact, Macros.enchantment],
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
                          (Casts You (Macros.a (And [Macros.instantOrSorcery, Macros.spell])) Nothing)
                          (Create You (Lit 1)
                            (TokenWritten (Macros.creatureTok 1 1 [Red] [creatureType "Elemental"])) []) ]
       (Just (2, 1))


inspiringStatuary : Card
inspiringStatuary =
  Macros.card "Inspiring Statuary" (Just [Macros.generic 3]) []
       (MkTypeLine [] [Artifact])
       [ Static (Gains (Macros.allOf (And [Not Macros.artifact, Macros.spell, CastBy You]))
                       (Macros.keyword "Improvise")) ]
       Nothing

chiefEngineer : Card
chiefEngineer =
  Macros.card "Chief Engineer" (Just [Macros.generic 1, Macros.pip Blue]) []
       (MkTypeLine [creatureType "Vedalken", creatureType "Artificer"] [Creature])
       [ Static (Gains (Macros.allOf (And [Macros.artifact, Macros.spell, CastBy You]))
                       (Macros.keyword "Convoke")) ]
       (Just (1, 3))

firesongAndSunspeaker : Ability
firesongAndSunspeaker =
  Static (Gains (Macros.allOf (And [ColorIs Red, Macros.instantOrSorcery, Macros.spell,
                             HasPossessor ControllerAx You]))
                (Macros.keyword "Lifelink"))

prismariTheInspiration : Card
prismariTheInspiration =
  Macros.card "Prismari, the Inspiration"
       (Just [Macros.generic 5, Macros.pip Blue, Macros.pip Red]) [Legendary]
       (MkTypeLine [creatureType "Elder", creatureType "Dragon"] [Creature])
       [ Macros.keyword "Flying"
       , Macros.keywordCosting "Ward" (Macros.payLife You 5)
       , Static (Gains (Macros.allOf (And [Macros.instantOrSorcery, Macros.spell, CastBy You]))
                       Macros.storm) ]
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
                           (Macros.countOf (InZone (Macros.handOf You)))) ]
       (Just (PtBox PrintedStar PrintedStar))

battleSquadron : Card
battleSquadron =
  Macros.cardOf "Battle Squadron" (Just [Macros.generic 3, Macros.pip Red, Macros.pip Red]) []
       (MkTypeLine [creatureType "Goblin"] [Creature])
       [ Macros.keyword "Flying"
       , Static (DefinesPt Macros.thisCreature BothEach
                           (Macros.countOf Macros.creatureYouControl)) ]
       (Just (PtBox PrintedStar PrintedStar))

peopleOfTheWoods : Card
peopleOfTheWoods =
  Macros.cardOf "People of the Woods" (Just [Macros.pip Green, Macros.pip Green]) []
       (MkTypeLine [creatureType "Human"] [Creature])
       [ Static (DefinesPt Macros.thisCreature ToughnessAlone
                           (Macros.countOf (And [HasSubtype (landType "Forest"), HasPossessor ControllerAx You]))) ]
       (Just (PtBox (PrintedNum 1) PrintedStar))

scourgeOfTheSkyclaves : Ability
scourgeOfTheSkyclaves =
  Static (DefinesPt Macros.thisCreature BothEach
                    (Minus (Lit 20) (Macros.aggregate MaxOf (PlayerStatAxis LifeTotal) AnyPlayer)))

aettirAndPriwen : Ability
aettirAndPriwen =
  Static (AndAlso [ HasBasePt (AttachHost Equipped (TypeW Creature))
                              (LetterVal X) (LetterVal X)
                  , Define X (PlayerStatOf LifeTotal You) ])

diminish : Card
diminish =
  Macros.card "Diminish" (Just [Macros.pip Blue]) []
       (MkTypeLine [] [Instant])
       [ Spell (Continuously {ts = StaticFirstDone} (HasBasePt (Macros.target Macros.creature) (Lit 1) (Lit 1))
                             (Just Macros.untilEndOfTurn)) ]
       Nothing

cycleOfLife : Effect []
cycleOfLife =
  Continuously {ts = StaticFirstDone} (HasBasePt (Macros.target (And [Macros.creature, CastBy You]))
                          (Lit 0) (Lit 1))
               (Just Macros.untilYourNextUpkeep)

aboutFace : Card
aboutFace =
  Macros.card "About Face" (Just [Macros.pip Red]) []
       (MkTypeLine [] [Instant])
       [ Spell (Continuously {ts = StaticFirstDone} (SwitchesPt (Macros.target Macros.creature))
                             (Just Macros.untilEndOfTurn)) ]
       Nothing


topple : Card
topple =
  Macros.card "Topple" (Just [Macros.generic 2, Macros.pip White]) []
       (MkTypeLine [] [Sorcery])
       [ Spell (Macros.exile You
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
       [ Macros.triggered At (BeginningOf Upkeep (Macros.yours))
                          (Macros.destroy
                      (Macros.target
                         (And [Permanent, Not Macros.land,
                               Superlative MinOf (CharAxis ManaValue)
                                           (And [Permanent, Not Macros.land])]))) ]
       Nothing

||| Purging Scythe
public export
purgingScythe : Ability
purgingScythe =
  Macros.triggered At (BeginningOf Upkeep (Macros.yours))
    (Sequentially
       [ DealDamage Macros.thisArtifact (Lit 2)
                    (Macros.the (And [Macros.creature,
                                    Superlative MinOf (CharAxis Toughness)
                                                Macros.creature]))
       , If (CompareAmt (Macros.countOf (And [Macros.creature,
                                       Superlative MinOf (CharAxis Toughness)
                                                   Macros.creature]))
                        AtLeast (Lit 2))
            (Macros.chooses You (Macros.oneOf Them))
            Nothing ])

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
                            (Macros.exists (And [Macros.creature, HasPossessor ControllerAx You,
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
                                 (And [Macros.creature, HasPossessor ControllerAx They])]))
           , Macros.gainsLife You (StatOf Power It) ])

coretapper : Card
coretapper =
  Macros.card "Coretapper" (Just [Macros.generic 2]) []
       (MkTypeLine [creatureType "Myr"] [Artifact, Creature])
       [ Macros.activated TapSymbol
                          (PutCounters (Lit 1) (PrintedKind (Named "Charge")) (Macros.target Macros.artifact))
       , Macros.activated (Do (Macros.sacrifice You Macros.thisCreature))
                          (PutCounters (Lit 2) (PrintedKind (Named "Charge")) (Macros.target Macros.artifact)) ]
       (Just (1, 1))

divineIntervention : Card
divineIntervention =
  Macros.card "Divine Intervention"
       (Just [Macros.generic 6, Macros.pip White, Macros.pip White]) []
       (MkTypeLine [] [Enchantment])
       [ Static (Macros.entersWithCounters Macros.thisEnchantment (Lit 2) (Named "Intervention"))
       , Macros.triggered At (BeginningOf Upkeep (Macros.yours))
                          (RemoveCounters (Just (Macros.exactly 1)) (Just (PrintedKind (Named "Intervention"))) Macros.thisEnchantment)
       , Macros.triggered When (Macros.lastCounterRemovedBy (Named "Intervention") Macros.thisEnchantment You)
                          GameDrawn ]
       Nothing

celestialConvergence : Card
celestialConvergence =
  Macros.card "Celestial Convergence"
       (Just [Macros.generic 2, Macros.pip White, Macros.pip White]) []
       (MkTypeLine [] [Enchantment])
       [ Static (Macros.entersWithCounters Macros.thisEnchantment (Lit 7) (Named "Omen"))
       , Macros.triggered At (BeginningOf Upkeep (Macros.yours))
           (Sequentially
              [ RemoveCounters (Just (Macros.exactly 1)) (Just (PrintedKind (Named "Omen"))) Macros.thisEnchantment
              , If (CompareAmt (CountersOn (Named "Omen") Macros.thisEnchantment)
                               AtMost (Lit 0))
                          (Concludes WinGame
                      (Macros.the (And [AnyPlayer,
                                      Superlative MaxOf (PlayerStatAxis LifeTotal)
                                                  AnyPlayer])))
                          Nothing
              , If (CompareAmt (Macros.countOf (And [AnyPlayer,
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
       , Macros.triggered When (Enters Macros.thisAura Nothing) (Draw You (Lit 1))
       , Static (Gains (AttachHost Enchanted (TypeW Creature))
                       (Macros.keyword "Flying")) ]
       Nothing

lionHeart : Card
lionHeart =
  Macros.card "Lion Heart" (Just [Macros.generic 4]) []
       (MkTypeLine [artifactType "Equipment"] [Artifact])
       [ Macros.triggered When (Enters Macros.thisEquipment Nothing)
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
                          (Casts (AttachHost Enchanted PlayerW) (Macros.a Macros.spell) Nothing)
                          (PutCounters (Lit 1) (PrintedKind (Named "Spite")) Macros.thisAura)
       , Macros.triggered When (LosesGame (AttachHost Enchanted PlayerW))
                          (Sequentially
                             [ ChangeLife You (Up (LetterVal X))
                             , Draw You (LetterVal X)
                             , Define X (CountersOn (Named "Spite") Macros.thisAura) ]) ]
       Nothing


consulsLieutenant : Card
consulsLieutenant =
  Macros.card "Consul's Lieutenant" (Just [Macros.pip White, Macros.pip White]) []
       (MkTypeLine [creatureType "Human", creatureType "Soldier"] [Creature])
       [ Macros.keyword "FirstStrike"
       , Macros.renown 1
       , Macros.triggeredIf Whenever
                            (Macros.attacks Macros.thisCreature)
                            (Matches Macros.thisCreature
                                     (HasDesignation Renowned))
                            (Macros.gets (Macros.allOf (And [Macros.creature, Attacking,
                                                      HasPossessor ControllerAx You,
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
                                 (Macros.doesntUntap Macros.thisCreature (Just You))) ]
       (Just (5, 3))


passagewaySeer : Card
passagewaySeer =
  Macros.card "Passageway Seer" (Just [Macros.generic 3, Macros.pip Black]) []
       (MkTypeLine [creatureType "Tiefling", creatureType "Warlock"] [Creature])
       [ Macros.keyword "Lifelink"
       , Macros.triggered When (Enters Macros.thisCreature Nothing)
                          (Macros.gainsDesignation You TheInitiative Instructed)
       , Macros.triggeredIf At
                            (BeginningOf EndStep (Macros.yours))
                            (Matches You (HasDesignation TheInitiative))
                            (PutCounters (Lit 1) (PrintedKind Macros.plusOnePlusOne) Macros.thisCreature) ]
       (Just (2, 2))

deadeyeBrawler : Card
deadeyeBrawler =
  Macros.card "Deadeye Brawler"
       (Just [Macros.generic 2, Macros.pip Blue, Macros.pip Black]) []
       (MkTypeLine [creatureType "Human", creatureType "Pirate"] [Creature])
       [ Macros.keyword "Deathtouch"
       , Macros.keyword "Ascend"
       , Macros.triggeredIf Whenever
                            (Macros.dealsCombatDamage Macros.thisCreature (Macros.a AnyPlayer))
                            (Matches You (HasDesignation CitysBlessing))
                            (Sequentially
              [ Macros.searchLibraryOrGraveyard
                  (And [Macros.artifact,
                        Compare [CharAxis ManaValue] AtMost (Lit 2)]) ]) ]
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
                     (Macros.dealsCombatDamage Macros.yourCommander (Macros.a AnyPlayer))
                     (Macros.thereIsNo Monarch)
                     (Macros.gainsDesignation You Monarch Instructed)


jushiApprentice : Ability
jushiApprentice =
  Macros.activated (Compound [Mana [Macros.generic 2, Macros.pip Blue], TapSymbol])
                   (Sequentially
               [ (Macros.draw You (Lit 1))
               , If (CompareAmt (Macros.countOf (InZone (Macros.handOf You)))
                                AtLeast (Lit 9))
                    (SetStatus Flipped Macros.thisCreature)
                    Nothing ])


chainsaw : Card
chainsaw =
  Macros.card "Chainsaw" (Just [Macros.generic 3]) []
       (MkTypeLine [artifactType "Equipment"] [Artifact])
       [ Macros.triggered When (Enters Macros.thisEquipment Nothing)
                          (DealDamage It (Lit 3)
                               (Macros.targets (Macros.upTo 1) Macros.creature))
       , Macros.triggered Whenever
                          (Dies (Macros.counted (Macros.atLeast 1) Macros.creature))
                          (PutCounters (Lit 1) (PrintedKind (Named "Rev")) Macros.thisEquipment)
       , Static (AndAlso [ Gets (AttachHost Equipped (TypeW Creature))
                                (PtUp (LetterVal X)) (PtUp (Lit 0))
                         , Define X (CountersOn (Named "Rev") Macros.thisEquipment) ])
       , Macros.keywordCosting "Equip" (Mana [Macros.generic 3]) ]
       Nothing

||| Phyrexian Ingester
phyrexianIngesterPump : Ability
phyrexianIngesterPump =
  Static (AndAlso [ Gets Macros.thisCreature (PtUp (LetterVal X)) (PtUp (LetterVal Y))
                  , Define X
                      (StatOf Power
                         (Macros.a (And [Macros.creature,
                                         ExiledWith Macros.thisCreature])))
                  , Define Y (StatOf Toughness (That CardW)) ])

||| Phyrexian Ingester
phyrexianIngester : Card
phyrexianIngester =
  Macros.card "Phyrexian Ingester"
       (Just [Macros.generic 6, Macros.pip Blue]) []
       (MkTypeLine [creatureType "Phyrexian", creatureType "Beast"] [Creature])
       [ Macros.abilityWord Imprint
           (Macros.triggered When (Enters Macros.thisCreature Nothing)
                             (Macros.may You
                                (Macros.exile You
                                   (Macros.target
                                      (And [Macros.creature, Macros.nontoken])))))
       , phyrexianIngesterPump ]
       (Just (3, 3))

drachNyen : Card
drachNyen =
  Macros.card "Drach'Nyen"
       (Just [Macros.generic 4, Macros.pip Black, Macros.pip Red]) [Legendary]
       (MkTypeLine [artifactType "Equipment"] [Artifact])
       [ Macros.triggered When (Enters Macros.thisEquipment Nothing)
                          (Macros.exile You (Macros.targets (Macros.upTo 1) Macros.creature))
       , Static (AndAlso [ Gains (AttachHost Equipped (TypeW Creature))
                                 (KeywordAbility "Menace" Nothing Nothing)
                         , Gets (AttachHost Equipped (TypeW Creature))
                                (PtUp (LetterVal X)) (PtUp (Lit 0))
                         , Define X
                             (StatOf Power
                                (Macros.a (ExiledWith Macros.thisEquipment))) ])
       , Macros.keywordCosting "Equip" (Mana [Macros.generic 2]) ]
       Nothing

||| Soul's Might
soulsMight : Card
soulsMight =
  Macros.card "Soul's Might" (Just [Macros.generic 4, Macros.pip Green]) []
       (MkTypeLine [] [Sorcery])
       [ Spell (Sequentially
                  [ PutCounters (LetterVal X) (PrintedKind Macros.plusOnePlusOne)
                                (Macros.target Macros.creature)
                  , Define X (StatOf Power (That (TypeW Creature))) ]) ]
       Nothing


lightmineField : Card
lightmineField =
  Macros.card "Lightmine Field"
       (Just [Macros.generic 2, Macros.pip White, Macros.pip White]) []
       (MkTypeLine [] [Enchantment])
       [ Macros.triggered Whenever
                          (Macros.attacks (Macros.counted (Macros.atLeast 1) Macros.creature))
                          (DealDamage Macros.thisEnchantment
                               (CountOf (Those (TypeW Creature)))
                               (EachOf (Those (TypeW Creature)))) ]
       Nothing

misterFantastic : Card
misterFantastic =
  Macros.card "Mister Fantastic, Reed Richards"
       (Just [Macros.generic 1, Macros.pip White, Macros.pip Blue]) [Legendary]
       (MkTypeLine [creatureType "Human", creatureType "Hero"] [Creature])
       [ Macros.keyword "Reach"
       , Macros.triggered Whenever
                          (Enters (Macros.counted (Macros.atLeast 1)
                                         (And [IsToken, HasPossessor ControllerAx You])) Nothing)
                          (Macros.may You (Macros.draw You (Lit 1))) ]
       (Just (2, 4))


woodlandChampion : Card
woodlandChampion =
  Macros.card "Woodland Champion" (Just [Macros.generic 1, Macros.pip Green]) []
       (MkTypeLine [creatureType "Elf", creatureType "Scout"] [Creature])
       [ Macros.triggered Whenever
                          (Enters (Macros.counted (Macros.atLeast 1)
                                         (And [IsToken, HasPossessor ControllerAx You])) Nothing)
                          (PutCounters GroupSize (PrintedKind Macros.plusOnePlusOne)
                                Macros.thisCreature) ]
       (Just (2, 2))

ingeniousArtillerist : Card
ingeniousArtillerist =
  Macros.card "Ingenious Artillerist"
       (Just [Macros.generic 2, Macros.pip Red]) []
       (MkTypeLine [creatureType "Human", creatureType "Artificer"] [Creature])
       [ Macros.triggered Whenever
                          (Enters (Macros.counted (Macros.atLeast 1)
                                         (And [Macros.artifact, HasPossessor ControllerAx You])) Nothing)
                          (DealDamage Macros.thisCreature GroupSize (Macros.each Opponent)) ]
       (Just (3, 1))

hissingMiasma : Card
hissingMiasma =
  Macros.card "Hissing Miasma"
       (Just [Macros.generic 1, Macros.pip Black, Macros.pip Black]) []
       (MkTypeLine [] [Enchantment])
       [ Macros.triggered Whenever
                          (Macros.attacksPlayer (Macros.a Macros.creature) You)
                          (Macros.losesLife (Macros.controllerOf It) (Lit 1)) ]
       Nothing

orimsPrayer : Card
orimsPrayer =
  Macros.card "Orim's Prayer"
       (Just [Macros.generic 1, Macros.pip White, Macros.pip White]) []
       (MkTypeLine [] [Enchantment])
       [ Macros.triggered Whenever
                          (Macros.attacksPlayer (Macros.counted (Macros.atLeast 1) Macros.creature) You)
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
                                         (Macros.the (And [AnyPlayer,
                                             Superlative MaxOf (PlayerStatAxis LifeTotal)
                                                         AnyPlayer])))
                          (Continuously {ts = StaticFirstDone} (Macros.deontic It Forbid ["Block"] Patient NoDeonticPatient)
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
                                       (FromZone [Macros.battlefieldZ]))
                          (Sequentially
              [ Macros.searchLibraryOrGraveyard
                  (And [Macros.artifact,
                        Compare [CharAxis ManaValue] AtMost (Lit 2)]) ]) ]
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
                          (Macros.putIntoFrom (Macros.counted (Macros.atLeast 1) Macros.creature)
                                       (Macros.graveyardOf You)
                                       FromAnywhere)
                          (PutCounters GroupSize (PrintedKind Macros.plusOnePlusOne)
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
                          (Macros.putIntoFrom (Macros.counted (Macros.atLeast 1)
                                                     (InZone (Macros.graveyardOf You)))
                                       Macros.exileZ
                                       (FromZone [Macros.graveyardOf You]))
                          (PutCounters GroupSize (PrintedKind Macros.plusOnePlusOne)
                                Macros.thisCreature) ]
       (Just (4, 4))


tocasiasWelcome : Card
tocasiasWelcome =
  Macros.card "Tocasia's Welcome"
       (Just [Macros.generic 2, Macros.pip White]) []
       (MkTypeLine [] [Enchantment])
       [ Macros.triggeredOnlyOnce Whenever
                                  (Enters (Macros.counted (Macros.atLeast 1)
                                                        (And [Macros.creature, HasPossessor ControllerAx You,
                                                              Compare [CharAxis ManaValue] AtMost (Lit 3)])) Nothing)
                                  OncePerTurn
                                  (Sequentially
              [ Macros.searchLibraryOrGraveyard
                  (And [Macros.artifact,
                        Compare [CharAxis ManaValue] AtMost (Lit 2)]) ]) ]
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
                                  (Sequentially
              [ Macros.searchLibraryOrGraveyard
                  (And [Macros.artifact,
                        Compare [CharAxis ManaValue] AtMost (Lit 2)]) ]) ]
       (Just (2, 2))

foeLiage : Card
foeLiage =
  Macros.card "Foe-liage"
       (Just [Macros.generic 3, Macros.pip Green]) []
       (MkTypeLine [creatureType "Plant", creatureType "Mutant"] [Creature])
       [ Macros.triggeredOnlyDuring Whenever
                                    (Enters (Macros.a Macros.land) Nothing)
                                    (DuringWindow Turn (Just You))
                                    (PutCounters (Lit 1) (PrintedKind Macros.plusOnePlusOne)
                                                 Macros.thisCreature) ]
       (Just (3, 3))

lesserGargadon : Card
lesserGargadon =
  Macros.card "Lesser Gargadon"
       (Just [Macros.generic 2, Macros.pip Red, Macros.pip Red]) []
       (MkTypeLine [creatureType "Beast"] [Creature])
       [ Macros.triggeredOr Whenever
                            (Macros.attacks Macros.thisCreature)
                            [Blocks Macros.thisCreature Nothing]
                            (Macros.sacrifice You (Macros.a Macros.land)) ]
       (Just (6, 4))

colossalGraveReaver : Card
colossalGraveReaver =
  Macros.card "Colossal Grave-Reaver"
       (Just [Macros.generic 6, Macros.pip Black, Macros.pip Green]) []
       (MkTypeLine [creatureType "Dragon"] [Creature])
       [ Macros.keyword "Flying"
       , Macros.triggeredOr Whenever
                            (Enters Macros.thisCreature Nothing)
                            [Macros.attacks Macros.thisCreature]
                            (Macros.mills You (Lit 3) You)
       , Macros.triggered Whenever
                          (Macros.putIntoFrom (Macros.counted (Macros.atLeast 1)
                                                     (And [Macros.creature,
                                                           InZone Macros.yourLibrary]))
                                       (Macros.graveyardOf You)
                                       (FromZone [Macros.yourLibrary]))
                          (Macros.putOntoBattlefield (Macros.oneOf Them)) ]
       (Just (7, 6))

||| Chub Toad
chubToad : Card
chubToad =
  Macros.card "Chub Toad"
       (Just [Macros.generic 2, Macros.pip Green]) []
       (MkTypeLine [creatureType "Frog"] [Creature])
       [ Macros.triggeredOr Whenever
                            (Blocks Macros.thisCreature Nothing)
                            [BecomesBlocked Macros.thisCreature Nothing]
                            (Macros.gets It (PtUp (Lit 2)) (PtUp (Lit 2))
                                         (Just Macros.untilEndOfTurn)) ]
       (Just (1, 1))

||| Inferno Elemental
infernoElemental : Card
infernoElemental =
  Macros.card "Inferno Elemental"
       (Just [Macros.generic 4, Macros.pip Red, Macros.pip Red]) []
       (MkTypeLine [creatureType "Elemental"] [Creature])
       [ Macros.triggeredOr Whenever
                            (Blocks Macros.thisCreature (Just (Macros.a Macros.creature)))
                            [BecomesBlocked Macros.thisCreature
                                             (Just (Macros.a Macros.creature))]
                            (DealDamage Macros.thisCreature (Lit 3)
                                        (That (TypeW Creature))) ]
       (Just (4, 4))

||| Giggling Skitterspike
public export
gigglingSkitterspikeArms : AltEvent {bs = []} Whenever
                             [ Blocks Macros.thisCreature Nothing
                             , BecomesTarget Macros.thisCreature
                                             (Macros.a Macros.spell) ]
gigglingSkitterspikeArms = MoreAlt

hardenedScales : Card
hardenedScales =
  Macros.card "Hardened Scales" (Just [Macros.pip Green]) []
       (MkTypeLine [] [Enchantment])
       [ Static (Intercepts
                   (Macros.manyCounterEvent CounterPut Macros.plusOnePlusOne
                                            (Macros.a Macros.creatureYouControl)) [] Nothing
                   (PutCounters (Plus ThatMuch (Lit 1))
                                (PrintedKind Macros.plusOnePlusOne) It)
                   Repeatedly Nothing) ]
       Nothing

branchingEvolution : Card
branchingEvolution =
  Macros.card "Branching Evolution"
       (Just [Macros.generic 2, Macros.pip Green]) []
       (MkTypeLine [] [Enchantment])
       [ Static (Intercepts
                   (Macros.manyCounterEvent CounterPut Macros.plusOnePlusOne
                                            (Macros.a Macros.creatureYouControl)) [] Nothing
                   (PutCounters (Times 2 ThatMuch)
                                (PrintedKind Macros.plusOnePlusOne) (That (TypeW Creature)))
                   Repeatedly Nothing) ]
       Nothing

||| Corpsejack Menace
corpsejackMenace : Ability
corpsejackMenace =
  Static (Intercepts
            (Macros.manyCounterEvent CounterPut Macros.plusOnePlusOne
                                     (Macros.a Macros.creatureYouControl)) [] Nothing
            (PutCounters (Times 2 ThatMuch) (PrintedKind Macros.plusOnePlusOne) It)
            Repeatedly Nothing)

||| Doubling Season
doublingSeason : Card
doublingSeason =
  Macros.card "Doubling Season" (Just [Macros.generic 4, Macros.pip Green]) []
       (MkTypeLine [] [Enchantment])
       [ Static (Intercepts
                   (Macros.tokensCreatedByEffectUnder
                      (Macros.counted (Macros.atLeast 1) IsToken) You) [] Nothing
                   (Create You (Times 2 GroupSize) TokenAsThose [])
                   Repeatedly Nothing)
       , Static (Intercepts
                   (Macros.manyCountersPutByEffect
                      (Macros.a (And [Permanent, HasPossessor ControllerAx You]))) [] Nothing
                   (PutCounters (Times 2 ThatMuch) ThoseKinds (That PermanentW))
                   Repeatedly Nothing) ]
       Nothing

||| Katara, the Fearless
public export
kataraTheFearless : Card
kataraTheFearless =
  Macros.card "Katara, the Fearless"
       (Just [Macros.pip Green, Macros.pip White, Macros.pip Blue]) [Legendary]
       (MkTypeLine [ creatureType "Human", creatureType "Warrior"
                   , creatureType "Ally" ] [Creature])
       [ Static (TriggersAdditionally
                   (Triggers
                      (Macros.a (And [ AbilityHead AnyTriggered
                                     , AbilityOf (Macros.a
                                         (And [ HasSubtype (creatureType "Ally")
                                              , HasPossessor ControllerAx You ])) ])))
                   (Macros.exactly 1)) ]
       (Just (3, 3))

||| Naban, Dean of Iteration
public export
nabanDeanOfIteration : Card
nabanDeanOfIteration =
  Macros.card "Naban, Dean of Iteration"
       (Just [Macros.generic 1, Macros.pip Blue]) [Legendary]
       (MkTypeLine [creatureType "Human", creatureType "Wizard"] [Creature])
       [ Static (TriggersAdditionally
                   (Causes
                      (CausedByEvent
                         (Enters (Macros.a (And [ HasSubtype (creatureType "Wizard")
                                                , HasPossessor ControllerAx You ])) Nothing))
                      (Triggers
                         (Macros.a (And [ AbilityHead AnyTriggered
                                        , AbilityOf (Macros.a
                                            (And [Permanent, HasPossessor ControllerAx You])) ]))))
                   (Macros.exactly 1)) ]
       (Just (2, 1))

||| Psychic Purge
public export
psychicPurge : Card
psychicPurge =
  Macros.card "Psychic Purge" (Just [Macros.pip Blue]) []
       (MkTypeLine [] [Sorcery])
       [ Spell (DealDamage This (Lit 1) (Macros.target Macros.anyTarget))
       , Macros.triggered When
           (Causes
              (CausedBySource
                 (Macros.a (And [ Joined Macros.spell (AbilityHead AnyOnStack)
                                , HasPossessor ControllerAx (Macros.a Opponent) ])))
              (VerbedEvent (Just You) "Discard" (Just This) Nothing False))
           (Macros.losesLife (That PlayerW) (Lit 5)) ]
       Nothing

||| Rain of Gore
public export
rainOfGore : Card
rainOfGore =
  Macros.card "Rain of Gore" (Just [Macros.pip Black, Macros.pip Red]) []
       (MkTypeLine [] [Enchantment])
       [ Static (Intercepts
                   (Causes
                      (CausedBySource
                         (Macros.a (Joined Macros.spell (AbilityHead AnyOnStack))))
                      (LifeChanges (Macros.controllerOf It) LifeGoesUp))
                   [] Nothing
                   (Macros.losesLife (That PlayerW) ThatMuch)
                   Repeatedly Nothing) ]
       Nothing


||| Doc Samson, Super Psychiatrist
docSamsonDistributive : Ability
docSamsonDistributive =
  Static (Intercepts
            (Macros.manyBareCountersPutBy You
               (Macros.a (And [Permanent, HasPossessor ControllerAx You]))) [] Nothing
            (PutCounters (Plus ThatMuch (Lit 1)) ThoseKinds (That PermanentW))
            Repeatedly Nothing)

||| Winding Constrictor
windingConstrictor : Card
windingConstrictor =
  Macros.card "Winding Constrictor" (Just [Macros.pip Black, Macros.pip Green]) []
       (MkTypeLine [creatureType "Snake"] [Creature])
       [ Static (Intercepts
                   (Macros.manyBareCounterEvent CounterPut
                      (Macros.a (And [Or [Macros.artifact, Macros.creature],
                                      HasPossessor ControllerAx You]))) [] Nothing
                   (PutCounters (Plus ThatMuch (Lit 1))
                                ThoseKinds (That PermanentW))
                   Repeatedly Nothing)
       , Static (Intercepts
                   (Macros.manyBareCounterEvent CounterPut You) [] Nothing
                   (PutCounters (Plus ThatMuch (Lit 1)) ThoseKinds You)
                   Repeatedly Nothing) ]
       (Just (2, 3))

||| Aragorn, Company Leader
aragornDistributive : Ability
aragornDistributive =
  Macros.triggered Whenever
    (Macros.manyBareCountersPutBy You Macros.thisCreature)
    (PutCounters (Lit 1) ThoseKinds
       (Macros.targets (Macros.upTo 1)
          (Macros.otherCreature Macros.thisCreature)))

||| Captain Marvel, Apex Avenger
captainMarvelSameKinds : Ability
captainMarvelSameKinds =
  Macros.triggered Whenever
    (Macros.manyBareCountersPutBy You
       (Macros.a (Macros.otherCreature Macros.thisCreature)))
    (Macros.may You (PutCounters ThatMuch ThoseKinds Macros.thisCreature))

||| Denry Klin, Editor in Chief
denryKlinSameKinds : Ability
denryKlinSameKinds =
  Macros.triggeredIf Whenever
    (Enters (Macros.a (And [Macros.nontoken, Macros.creatureYouControl])) Nothing)
    (Matches Macros.thisCreature (HasCounters Nothing))
    (PutCounters (Lit 1) (SameAs Macros.thisCreature) (That (TypeW Creature)))

||| Denry Klin, Editor in Chief
denryKlin : Card
denryKlin =
  Macros.card "Denry Klin, Editor in Chief"
       (Just [Macros.generic 2, Macros.pip White, Macros.pip Blue])
       [Legendary]
       (MkTypeLine [creatureType "Cat", creatureType "Advisor"] [Creature])
       [ Static (EntersRider Macros.thisCreature (WithCounters (Lit 1)
                   (ChosenKind [ Macros.plusOnePlusOne
                               , KeywordCounter "FirstStrike"
                               , KeywordCounter "Vigilance" ])
                   Fresh))
       , denryKlinSameKinds ]
       (Just (2, 2))

||| Helica Glider
helicaGlider : Card
helicaGlider =
  Macros.card "Helica Glider"
       (Just [Macros.generic 2, Macros.pip White]) []
       (MkTypeLine [creatureType "Nightmare", creatureType "Squirrel"] [Creature])
       [ Static (EntersRider Macros.thisCreature (WithCounters (Lit 1)
                   (ChosenKind [ KeywordCounter "Flying"
                               , KeywordCounter "FirstStrike" ])
                   Fresh)) ]
       (Just (2, 2))

||| Grimdancer
public export
grimdancer : Card
grimdancer =
  Macros.card "Grimdancer"
       (Just [Macros.generic 1, Macros.pip Black, Macros.pip Black]) []
       (MkTypeLine [creatureType "Nightmare"] [Creature])
       [ Static (EntersRider Macros.thisCreature (WithCounters (Lit 2)
                   (DistinctChosenKinds [ KeywordCounter "Menace"
                                        , KeywordCounter "Deathtouch"
                                        , KeywordCounter "Lifelink" ])
                   Fresh)) ]
       (Just (3, 3))

||| Me, the Immortal
meTheImmortalCounterMenu : Ability
meTheImmortalCounterMenu =
  Macros.triggered At (BeginningOf Combat (Macros.yours))
    (PutCounters (Lit 1)
       (ChosenKind [ Macros.plusOnePlusOne
                   , KeywordCounter "FirstStrike"
                   , KeywordCounter "Vigilance"
                   , KeywordCounter "Menace" ])
       Macros.thisCreature)

||| Star Pupil
starPupil : Card
starPupil =
  Macros.card "Star Pupil" (Just [Macros.pip White]) []
       (MkTypeLine [creatureType "Human", creatureType "Wizard"] [Creature])
       [ Static (Macros.entersWithCounters Macros.thisCreature (Lit 1)
                                           Macros.plusOnePlusOne)
       , Macros.triggered When (Dies Macros.thisCreature)
                          (PutCounters (Lit 1) (SameAs It)
                             (Macros.target Macros.creatureYouControl)) ]
       (Just (0, 0))

||| Bloom Hulk
bloomHulk : Card
bloomHulk =
  Macros.card "Bloom Hulk" (Just [Macros.generic 3, Macros.pip Green]) []
       (MkTypeLine [creatureType "Plant", creatureType "Elemental"] [Creature])
       [ Macros.triggered When (Enters Macros.thisCreature Nothing) Macros.proliferate ]
       (Just (4, 4))

||| Tromell, Seymour's Butler
tromell : Card
tromell =
  Macros.card "Tromell, Seymour's Butler"
       (Just [Macros.generic 2, Macros.pip Green]) [Legendary]
       (MkTypeLine [creatureType "Elf", creatureType "Advisor"] [Creature])
       [ Static (Macros.entersWithAdditionalCounters
                   (Macros.each (And [Macros.creature, Macros.nontoken, HasPossessor ControllerAx You,
                               OtherThan Macros.thisCreature]))
                   (Lit 1) Macros.plusOnePlusOne)
       , Macros.activated (Compound [Mana [Macros.generic 1], TapSymbol])
           (Sequentially
              [ Repeated (LetterVal X) Macros.proliferate
              , Define X (Macros.countOf (And [Macros.nontoken, Macros.creature,
                                        HasPossessor ControllerAx You,
                                        HappenedTo Entry Lookback.ThisTurn Nothing])) ]) ]
       (Just (2, 3))

||| Stalwart Successor
stalwartSuccessorHeader : GameEvent []
stalwartSuccessorHeader =
  Macros.manyBareCounterEvent CounterPut (Macros.a Macros.creatureYouControl)

||| Runadi, Behemoth Caller
runadiBehemothCaller : Ability
runadiBehemothCaller =
  Static (Gains (Macros.allOf (And [Macros.creature, HasPossessor ControllerAx You,
                             Compare [CounterAxis Macros.plusOnePlusOne]
                                            AtLeast (Lit 3)]))
                (Macros.keyword "Haste"))

corruptedOpponents : Noun [] Player
corruptedOpponents =
  Macros.each (And [Opponent, Compare [CounterAxis (Named "Poison")] AtLeast (Lit 3)])

||| Boon of Safety
boonOfSafetyPut : Effect []
boonOfSafetyPut = PutCounters (Lit 1) (PrintedKind (Named "Shield")) (Macros.target Macros.creature)

||| Vivien's Talent and Teferi's Talent
talentLoyaltyPut : Effect []
talentLoyaltyPut =
  PutCounters (Lit 1) (PrintedKind (Named "Loyalty"))
              (AttachHost Enchanted (TypeW Planeswalker))

||| Simic Fluxmage
simicFluxmageMove : Effect []
simicFluxmageMove =
  MoveCounters (Lit 1) (Just (PrintedKind Macros.plusOnePlusOne)) Macros.thisCreature
               (Macros.target Macros.creature)

||| Rikku, Resourceful Guardian
rikkuStealMove : Effect []
rikkuStealMove =
  MoveCounters (Lit 1) Nothing
               (Macros.target (And [Macros.creature,
                                    HasPossessor ControllerAx Macros.anOpponent]))
               (Macros.target Macros.creatureYouControl)

||| Littjara Mirrorlake
littjaraMirrorlakeCopy : Effect []
littjaraMirrorlakeCopy =
  Create You (Lit 1)
         (TokenCopyOf (Macros.target (And [Macros.creature, HasPossessor ControllerAx You]))
                      [ExceptEntersWithCounters (Lit 1) Macros.plusOnePlusOne
                                                Additional])
         []

||| Master Chef
masterChefGrantedAbility : Ability
masterChefGrantedAbility =
  Static (Macros.entersWithAdditionalCounters Macros.thisCreature (Lit 1)
                                              Macros.plusOnePlusOne)

grantedEntryCounterShape : StaticEffect []
grantedEntryCounterShape =
  Gains (Macros.allOf Macros.creatureYouControl) masterChefGrantedAbility

||| Hollowmurk Siege
hollowmurkSiegeSultai : Ability
hollowmurkSiegeSultai =
  Macros.triggered Whenever
    (Macros.bareCounterEvent CounterPut (Macros.a Macros.creatureYouControl))
    (Macros.draw You (Lit 1))


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
                   (Macros.tokensCreatedUnder (Macros.counted (Macros.atLeast 1)
                                                            (And [Macros.creature, IsToken]))
                                              You) [] Nothing
                   (Macros.create GroupSize
                                  (MkToken (Just (Lit 4 ** Lit 4)) [White]
                                           (MkTypeLine [creatureType "Angel"] [Creature])
                                           [Macros.keyword "Flying", Macros.keyword "Vigilance"]
                                             Nothing))
                   Repeatedly Nothing) ]
       Nothing

anointedProcession : Card
anointedProcession =
  Macros.card "Anointed Procession"
       (Just [Macros.generic 3, Macros.pip White]) []
       (MkTypeLine [] [Enchantment])
       [ Static (Intercepts
                   (Macros.tokensCreatedByEffectUnder (Macros.counted (Macros.atLeast 1) IsToken)
                                                      You) [] Nothing
                   (Create You (Times 2 GroupSize) TokenAsThose [])
                   Repeatedly Nothing) ]
       Nothing

primalVigor : Card
primalVigor =
  Macros.card "Primal Vigor"
       (Just [Macros.generic 4, Macros.pip Green]) []
       (MkTypeLine [] [Enchantment])
       [ Static (Intercepts
                   (Macros.tokensCreated (Macros.counted (Macros.atLeast 1) IsToken)) [] Nothing
                   (Create You (Times 2 GroupSize) TokenAsThose [])
                   Repeatedly Nothing)
       , Static (Intercepts
                   (Macros.manyCounterEvent CounterPut Macros.plusOnePlusOne
                                            (Macros.a Macros.creature)) [] Nothing
                   (PutCounters (Times 2 ThatMuch)
                                (PrintedKind Macros.plusOnePlusOne) (That (TypeW Creature)))
                   Repeatedly Nothing) ]
       Nothing

adrixAndNev : Card
adrixAndNev =
  Macros.card "Adrix and Nev, Twincasters"
       (Just [Macros.generic 2, Macros.pip Green, Macros.pip Blue]) [Legendary]
       (MkTypeLine [creatureType "Merfolk", creatureType "Wizard"] [Creature])
       [ Macros.keywordCosting "Ward" (Mana [Macros.generic 2])
       , Static (Intercepts
                   (Macros.tokensCreatedUnder (Macros.counted (Macros.atLeast 1) IsToken) You) [] Nothing
                   (Create You (Times 2 GroupSize) TokenAsThose [])
                   Repeatedly Nothing) ]
       (Just (2, 2))


naturalAffinity : Card
naturalAffinity =
  Macros.card "Natural Affinity" (Just [Macros.generic 2, Macros.pip Green]) []
       (MkTypeLine [] [Instant])
       [ Spell (Continuously {ts = StaticFirstDone}
                  (Becomes (Macros.allOf Macros.land) Sets (Bundle (MkToken (Just (Lit 2 ** Lit 2)) []
                                     (MkTypeLine [] [Creature]) [] Nothing) (Just Land)))
                  (Just Macros.untilEndOfTurn)) ]
       Nothing

mishrasFactory : Card
mishrasFactory =
  Macros.card "Mishra's Factory" Nothing [] (MkTypeLine [] [Land])
       [ Macros.activated TapSymbol (AddMana You (Lit 1) (Runs [[Colorless]]) [])
       , Macros.activated (Mana [Macros.generic 1])
                          (Continuously {ts = StaticFirstDone}
                      (Becomes Macros.thisLand Sets (Bundle (MkToken (Just (Lit 2 ** Lit 2)) []
                                         (MkTypeLine [creatureType "AssemblyWorker"] [Artifact, Creature])
                                         [] Nothing) (Just Land)))
                      (Just Macros.untilEndOfTurn))
       , Macros.activated TapSymbol
                          (Macros.gets (Macros.target (And [Macros.creature, HasSubtype (creatureType "AssemblyWorker")]))
                                (PtUp (Lit 1)) (PtUp (Lit 1)) (Just Macros.untilEndOfTurn)) ]
       Nothing

||| Mutavault
public export
mutavault : Card
mutavault =
  Macros.card "Mutavault" Nothing [] (MkTypeLine [] [Land])
       [ Macros.activated TapSymbol (AddMana You (Lit 1) (Runs [[Colorless]]) [])
       , Macros.activated (Mana [Macros.generic 1])
                          (Continuously {ts = StaticFirstDone}
                      (Becomes Macros.thisLand Sets (Bundle (MkTokenChars (Just (Lit 2 ** Lit 2)) [] []
                                              (MkTypeLine [] [Creature])
                                              [] Nothing
                                              [WithEveryType CreatureSpace]) (Just Land)))
                      (Just Macros.untilEndOfTurn)) ]
       Nothing

||| Soulstone Sanctuary
public export
soulstoneSanctuary : Card
soulstoneSanctuary =
  Macros.card "Soulstone Sanctuary" Nothing [] (MkTypeLine [] [Land])
       [ Macros.activated TapSymbol (AddMana You (Lit 1) (Runs [[Colorless]]) [])
       , Macros.activated (Mana [Macros.generic 4])
                          (Continuously {ts = StaticFirstDone}
                      (Becomes Macros.thisLand Sets (Bundle (MkTokenChars (Just (Lit 3 ** Lit 3)) [] []
                                              (MkTypeLine [] [Creature])
                                              [Macros.keyword "Vigilance"] Nothing
                                              [WithEveryType CreatureSpace]) (Just Land)))
                      Nothing) ]
       Nothing

windZendikon : Card
windZendikon =
  Macros.card "Wind Zendikon" (Just [Macros.pip Blue]) []
       (MkTypeLine [enchantmentType "Aura"] [Enchantment])
       [ Macros.keywordSubject "Enchant" Macros.land
       , Static (Becomes (AttachHost Enchanted (TypeW Land)) Sets (Bundle (MkToken (Just (Lit 2 ** Lit 2)) [Blue]
                                   (MkTypeLine [creatureType "Elemental"] [Creature])
                                   [Macros.keyword "Flying"] Nothing) (Just Land))) ]
       Nothing

nullhideFerox : Ability
nullhideFerox =
  Macros.activated (Mana [Macros.generic 2])
                   (Continuously {ts = StaticFirstDone} (LosesAllAbilities Macros.thisCreature Nothing)
                          (Just Macros.untilEndOfTurn))


awakenTheBear : Card
awakenTheBear =
  Macros.card "Awaken the Bear" (Just [Macros.generic 2, Macros.pip Green]) []
       (MkTypeLine [] [Instant])
       [ Spell (Continuously {ts = StaticFirstDone}
                  (AndAlso [ Gets (Macros.target Macros.creature)
                                  (PtUp (Lit 3)) (PtUp (Lit 3))
                           , Gains It (Macros.keyword "Trample") ])
                  (Just Macros.untilEndOfTurn)) ]
       Nothing

spidersilkArmor : Card
spidersilkArmor =
  Macros.card "Spidersilk Armor" (Just [Macros.generic 2, Macros.pip Green]) []
       (MkTypeLine [] [Enchantment])
       [ Static (AndAlso [ Gets (Macros.allOf Macros.creatureYouControl)
                                (PtUp (Lit 0)) (PtUp (Lit 1))
                         , Gains Them (Macros.keyword "Reach") ]) ]
       Nothing

turnToFrog : Card
turnToFrog =
  Macros.card "Turn to Frog" (Just [Macros.generic 1, Macros.pip Blue]) []
       (MkTypeLine [] [Instant])
       [ Spell (Continuously {ts = StaticFirstDone}
                  (AndAlso [ LosesAllAbilities (Macros.target Macros.creature) Nothing
                           , Becomes It Sets (Bundle (MkToken Nothing [Blue]
                                                  (MkTypeLine [creatureType "Frog"] []) [] Nothing) Nothing)
                           , HasBasePt It (Lit 1) (Lit 1) ])
                  (Just Macros.untilEndOfTurn)) ]
       Nothing

humility : Card
humility =
  Macros.card "Humility" (Just [Macros.generic 2, Macros.pip White, Macros.pip White]) []
       (MkTypeLine [] [Enchantment])
       [ Static (AndAlso [ LosesAllAbilities (Macros.allOf Macros.creature) Nothing
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
                         , Macros.deontic It Forbid ["Block"] Patient NoDeonticPatient ]) ]
       Nothing

frogify : Card
frogify =
  Macros.card "Frogify" (Just [Macros.generic 1, Macros.pip Blue]) []
       (MkTypeLine [enchantmentType "Aura"] [Enchantment])
       [ Macros.keywordSubject "Enchant" Macros.creature
       , Static (AndAlso [ LosesAllAbilities (AttachHost Enchanted (TypeW Creature)) Nothing
                         , Becomes It Sets (Bundle (MkToken Nothing [Blue]
                                                (MkTypeLine [creatureType "Frog"] [Creature]) [] Nothing) Nothing)
                         , HasBasePt It (Lit 1) (Lit 1) ]) ]
       Nothing


darksteelMutation : Card
darksteelMutation =
  Macros.card "Darksteel Mutation" (Just [Macros.generic 1, Macros.pip White]) []
       (MkTypeLine [enchantmentType "Aura"] [Enchantment])
       [ Macros.keywordSubject "Enchant" Macros.creature
       , Static (AndAlso [ Becomes (AttachHost Enchanted (TypeW Creature)) Sets (Bundle (MkToken Nothing []
                                             (MkTypeLine [creatureType "Insect"] [Artifact, Creature])
                                             [] Nothing) Nothing)
                         , HasBasePt It (Lit 0) (Lit 1)
                         , Gains It (Macros.keyword "Indestructible")
                         , LosesAllAbilities It Nothing ]) ]
       Nothing

kenrithsTransformation : Card
kenrithsTransformation =
  Macros.card "Kenrith's Transformation" (Just [Macros.generic 1, Macros.pip Green]) []
       (MkTypeLine [enchantmentType "Aura"] [Enchantment])
       [ Macros.keywordSubject "Enchant" Macros.creature
       , Macros.triggered When (Enters Macros.thisAura Nothing) (Draw You (Lit 1))
       , Static (AndAlso [ LosesAllAbilities (AttachHost Enchanted (TypeW Creature)) Nothing
                         , Becomes It Sets (Bundle (MkToken Nothing [Green]
                                                (MkTypeLine [creatureType "Elk"] [Creature]) [] Nothing) Nothing)
                         , HasBasePt It (Lit 3) (Lit 3) ]) ]
       Nothing

amphibianDownpour : Card
amphibianDownpour =
  Macros.card "Amphibian Downpour" (Just [Macros.generic 2, Macros.pip Blue]) []
       (MkTypeLine [enchantmentType "Aura"] [Enchantment])
       [ Macros.keyword "Flash"
       , Macros.storm
       , Macros.keywordSubject "Enchant" Macros.creature
       , Static (AndAlso [ LosesAllAbilities (AttachHost Enchanted (TypeW Creature)) Nothing
                         , Becomes It Sets (Bundle (MkToken Nothing [Blue]
                                                (MkTypeLine [creatureType "Frog"] [Creature]) [] Nothing) Nothing)
                         , HasBasePt It (Lit 1) (Lit 1) ]) ]
       Nothing


lignify : Card
lignify =
  Macros.card "Lignify" (Just [Macros.generic 1, Macros.pip Green]) []
       (MkTypeLine [creatureType "Treefolk", enchantmentType "Aura"] [Kindred, Enchantment])
       [ Macros.keywordSubject "Enchant" Macros.creature
       , Static (AndAlso [ Becomes (AttachHost Enchanted (TypeW Creature)) Sets (Bundle (MkToken Nothing [] (MkTypeLine [creatureType "Treefolk"] [])
                                             [] Nothing) Nothing)
                         , HasBasePt It (Lit 0) (Lit 4)
                         , LosesAllAbilities It Nothing ]) ]
       Nothing

invasionOfDominaria : Card
invasionOfDominaria =
  Transforming
    (Macros.frontFace "Invasion of Dominaria" (Just [Macros.generic 2, Macros.pip White]) []
            (MkTypeLine [battleType "Siege"] [Battle])
            [ Macros.triggered When (Enters Macros.thisSiege Nothing)
                               (Sequentially [ ChangeLife You (Up (Lit 4))
                                             , Draw You (Lit 1) ]) ]
            (Macros.defenseBox 5))
    (Macros.backFace "Serra Faithkeeper" [] (MkTypeLine [creatureType "Angel"] [Creature])
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
                          (Continuously {ts = StaticFirstDone}
                      (Becomes Macros.thisLand Sets (Bundle (MkToken (Just (Lit 3 ** Lit 3)) [Red, Green]
                                         (MkTypeLine [creatureType "Elemental"] [Creature])
                                         [ Macros.triggered Whenever
                                                            (Macros.attacks Macros.thisCreature)
                                                            (PutCounters (Lit 1)
                                                                  (PrintedKind Macros.plusOnePlusOne) Macros.thisCreature) ]
                                         Nothing) (Just Land)))
                      (Just Macros.untilEndOfTurn)) ]
       Nothing



||| Ajani, Adversary of Tyrants
ajanisEmblem : Effect []
ajanisEmblem =
  GetsEmblem You
    [ Macros.triggered At (BeginningOf EndStep (Macros.yours))
                       (Macros.create (Lit 3)
                   (MkToken (Just (Lit 1 ** Lit 1)) [White] (MkTypeLine [creatureType "Cat"] [Creature])
                            [Macros.keyword "Lifelink"] Nothing)) ]

||| Saheeli, Filigree Master
saheelisEmblem : Effect []
saheelisEmblem =
  GetsEmblem You
    [ Static (Gets (Macros.allOf (And [Macros.artifact, Macros.creature, HasPossessor ControllerAx You]))
                   (PtUp (Lit 1)) (PtUp (Lit 1)))
    , Static (CostsToCast (Macros.allOf (And [Macros.artifact, Macros.spell, CastBy You]))
                          (CostLess (Lit 1) Nothing)) ]


jaceBeleren : Card
jaceBeleren =
  Macros.cardOf "Jace Beleren" (Just [Macros.generic 1, Macros.pip Blue, Macros.pip Blue])
       [Legendary] (MkTypeLine [planeswalkerType "Jace"] [Planeswalker])
       [ Macros.activated (LoyaltySymbol (LoyaltyUp 2)) (Draw (Macros.each AnyPlayer) (Lit 1))
       , Macros.activated (LoyaltySymbol (LoyaltyDown 1))
                          ((Macros.draw (Macros.target AnyPlayer) (Lit 1)))
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
                          (Macros.destroy (Macros.allOf (And [Macros.creature,
                                                Compare [CharAxis Power] AtLeast (Lit 4)])))
       , Macros.activated (LoyaltySymbol (LoyaltyDown 7))
                          (GetsEmblem You
                      [ Static (AndAlso [ Gets (Macros.allOf Macros.creatureYouControl)
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
       [ Spell (Sequentially [ Macros.scry You (Lit 1)
                             , (Macros.draw You (Lit 1)) ]) ]
       Nothing

serumVisions : Card
serumVisions =
  Macros.card "Serum Visions" (Just [Macros.pip Blue]) [] (MkTypeLine [] [Sorcery])
       [ Spell (Sequentially [ (Macros.draw You (Lit 1))
                             , Macros.scry You (Lit 2) ]) ]
       Nothing

consider : Card
consider =
  Macros.card "Consider" (Just [Macros.pip Blue]) [] (MkTypeLine [] [Instant])
       [ Spell (Sequentially [ Macros.surveil You (Lit 1)
                             , (Macros.draw You (Lit 1)) ]) ]
       Nothing

crystalBall : Card
crystalBall =
  Macros.card "Crystal Ball" (Just [Macros.generic 3]) [] (MkTypeLine [] [Artifact])
       [ Macros.activated (Compound [Mana [Macros.generic 1], TapSymbol])
                          (Macros.scry You (Lit 2)) ]
       Nothing

nefariousImp : Card
nefariousImp =
  Macros.card "Nefarious Imp" (Just [Macros.generic 2, Macros.pip Black]) []
       (MkTypeLine [creatureType "Imp"] [Creature])
       [ Macros.keyword "Flying"
       , Macros.triggered Whenever
                          (Macros.leavesBattlefield
                             (Macros.counted (Macros.atLeast 1)
                                     (And [Permanent, HasPossessor ControllerAx You])))
                          (Macros.scry You (Lit 1)) ]
       (Just (2, 1))

||| Eager Construct
public export
eagerConstruct : Card
eagerConstruct =
  Macros.card "Eager Construct" (Just [Macros.generic 2]) []
       (MkTypeLine [creatureType "Construct"] [Artifact, Creature])
       [ Macros.triggered When (Enters Macros.thisCreature Nothing)
                          (Macros.may (Macros.each AnyPlayer)
                                      ((Macros.scry They (Lit 1)))) ]
       (Just (2, 2))

saheeliFiligreeMaster : Card
saheeliFiligreeMaster =
  Macros.cardOf "Saheeli, Filigree Master"
       (Just [Macros.generic 2, Macros.pip Blue, Macros.pip Red])
       [Legendary] (MkTypeLine [planeswalkerType "Saheeli"] [Planeswalker])
       [ Macros.activated (LoyaltySymbol (LoyaltyUp 1))
                          (Sequentially
                      [ Macros.scry You (Lit 1)
                      , (May You (SetStatus Tapped
                             (Macros.a (And [Macros.artifact, Macros.untapped,
                                             HasPossessor ControllerAx You]))) (Just (Sequentially
              [ Macros.searchLibraryOrGraveyard
                  (And [Macros.artifact,
                        Compare [CharAxis ManaValue] AtMost (Lit 2)]) ])) Nothing) ])
       , Macros.activated (LoyaltySymbol (LoyaltyDown 2))
                          (Sequentially
                      [ Macros.create (Lit 2)
                          (MkToken (Just (Lit 1 ** Lit 1)) []
                                   (MkTypeLine [creatureType "Thopter"] [Artifact, Creature])
                                   [Macros.keyword "Flying"] Nothing)
                      , Macros.gainsHaste Them (Just Macros.untilEndOfTurn) ])
       , Macros.activated (LoyaltySymbol (LoyaltyDown 4))
                          (GetsEmblem You
                      [ Static (Gets (Macros.allOf (And [Macros.artifact, Macros.creature,
                                                  HasPossessor ControllerAx You]))
                                     (PtUp (Lit 1)) (PtUp (Lit 1)))
                      , Static (CostsToCast (Macros.allOf (And [Macros.artifact, Macros.spell,
                                                         CastBy You]))
                                            (CostLess (Lit 1) Nothing)) ]) ]
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
  Macros.triggered When (Enters Macros.thisArtifact Nothing)
                   (AddMana You (Lit 4) (AnyColor SameColor)
                     [SpendOnly [ToActivate Nothing]])

||| Rosheen Meanderer
public export
rosheenMeanderer : Card
rosheenMeanderer =
  Macros.card "Rosheen Meanderer"
       (Just [Macros.generic 3, Macros.hybridPip Red Green]) [Legendary]
       (MkTypeLine [creatureType "Giant", creatureType "Shaman"] [Creature])
       [ Macros.activated TapSymbol
                          (AddMana You (Lit 1)
                            (Runs [[Colorless, Colorless, Colorless, Colorless]])
                            [SpendOnly [ToPay (Containing Variable)]]) ]
       (Just (4, 4))

||| Adarkar Unicorn
public export
adarkarUnicorn : Card
adarkarUnicorn =
  Macros.card "Adarkar Unicorn"
       (Just [Macros.generic 1, Macros.pip White, Macros.pip White]) []
       (MkTypeLine [creatureType "Unicorn"] [Creature])
       [ Macros.activated TapSymbol
                          (AddMana You (Lit 1)
                            (Runs [[OfColor Blue], [Colorless, OfColor Blue]])
                            [SpendOnly [ToPay (OfKeyword "CumulativeUpkeep")]]) ]
       (Just (2, 2))

||| Overgrown Zealot
public export
overgrownZealot : Card
overgrownZealot =
  Macros.card "Overgrown Zealot"
       (Just [Macros.generic 1, Macros.pip Green]) []
       (MkTypeLine [creatureType "Elf", creatureType "Druid"] [Creature])
       [ Macros.activated TapSymbol (AddMana You (Lit 1) (AnyColor SameColor) [])
       , Macros.activated TapSymbol
                          (AddMana You (Lit 2) (AnyColor SameColor)
                            [SpendOnly [ToPay (OfSpecialAction TurnFaceUp)]]) ]
       (Just (0, 4))

||| Unblinking Observer
public export
unblinkingObserver : Card
unblinkingObserver =
  Macros.card "Unblinking Observer"
       (Just [Macros.generic 1, Macros.pip Blue]) []
       (MkTypeLine [creatureType "Homunculus"] [Creature])
       [ Macros.activated TapSymbol
                          (AddMana You (Lit 1) (Runs [[OfColor Blue]])
                            [SpendOnly [ ToPay (OfKeyword "Disturb")
                                       , ToCast Macros.instantOrSorcery ]]) ]
       (Just (2, 1))

||| Ainok Tracker
public export
ainokTracker : Card
ainokTracker =
  Macros.card "Ainok Tracker"
       (Just [Macros.generic 5, Macros.pip Red]) []
       (MkTypeLine [creatureType "Dog", creatureType "Scout"] [Creature])
       [ Macros.keyword "FirstStrike"
       , Macros.keywordCosting "Morph"
                               (Mana [Macros.generic 4, Macros.pip Red]) ]
       (Just (3, 3))

||| Qarsi Deceiver
public export
qarsiDeceiverMorphSpend : Ability
qarsiDeceiverMorphSpend =
  Macros.activated TapSymbol
                   (AddMana You (Lit 1) (Runs [[Colorless]])
                     [SpendOnly [ ToPay (OfSpecialAction TurnFaceUp)
                                , ToPay (OfKeyword "Morph") ]])

||| Mercadian Bazaar
public export
mercadianBazaar : Card
mercadianBazaar =
  Macros.card "Mercadian Bazaar" Nothing [] (MkTypeLine [] [Land])
       [ Static (Macros.entersTapped Macros.thisLand)
       , Macros.activated TapSymbol
                          (PutCounters (Lit 1) (PrintedKind (Named "Storage")) Macros.thisLand)
       , Macros.activated (Compound [TapSymbol,
                                     Do (RemoveCounters (Just Macros.anyNumber) (Just (PrintedKind (Named "Storage")))
                                                        Macros.thisLand)])
                          (AddMana You RemovedThisWay (Runs [[OfColor Red]]) []) ]
       Nothing

||| Rootcoil Creeper
public export
rootcoilCreeperGraveyardMana : Ability
rootcoilCreeperGraveyardMana =
  Macros.activated TapSymbol
                   (AddMana You (Lit 2) (AnyColor SameColor)
                     [SpendOnly [ToCast (And [Macros.spell,
                                              CastFrom (Macros.graveyardOf You)])]])

||| Black Mana Battery
public export
blackManaBattery : Card
blackManaBattery =
  Macros.card "Black Mana Battery" (Just [Macros.generic 4]) []
       (MkTypeLine [] [Artifact])
       [ Macros.activated (Compound [Mana [Macros.generic 2], TapSymbol])
                          (PutCounters (Lit 1) (PrintedKind (Named "Charge")) Macros.thisArtifact)
       , Macros.activated (Compound [TapSymbol,
                                     Do (RemoveCounters (Just Macros.anyNumber) (Just (PrintedKind (Named "Charge")))
                                                        Macros.thisArtifact)])
                          (Sequentially
                             [ AddMana You (Lit 1) (Runs [[OfColor Black]]) []
                             , AddMana You RemovedThisWay (Runs [[OfColor Black]]) [] ]) ]
       Nothing

||| Galloping Lizrog
public export
gallopingLizrog : Card
gallopingLizrog =
  Macros.card "Galloping Lizrog"
       (Just [Macros.generic 3, Macros.pip Green, Macros.pip Blue]) []
       (MkTypeLine [creatureType "Frog", creatureType "Lizard"] [Creature])
       [ Macros.keyword "Trample"
       , Macros.triggered When (Enters Macros.thisCreature Nothing)
           ((May You (RemoveCountersAmong Macros.anyNumber (Just (PrintedKind Macros.plusOnePlusOne))
                                   (Macros.allOf Macros.creatureYouControl)) (Just (PutCounters (Times 2 RemovedThisWay)
                           (PrintedKind Macros.plusOnePlusOne)
                           Macros.thisCreature)) Nothing)) ]
       (Just (3, 3))

||| Vampire Hexmage
public export
vampireHexmage : Card
vampireHexmage =
  Macros.card "Vampire Hexmage" (Just [Macros.pip Black, Macros.pip Black]) []
       (MkTypeLine [creatureType "Vampire", creatureType "Shaman"] [Creature])
       [ Macros.keyword "FirstStrike"
       , Macros.activated (Do (Macros.sacrifice You Macros.thisCreature))
                          (Macros.removeAllCounters Nothing
                             (Macros.target Permanent)) ]
       (Just (2, 1))

||| Novijen Sages
public export
novijenSagesDraw : Ability
novijenSagesDraw =
  Macros.activated (Compound [Mana [Macros.generic 1],
                       Do (RemoveCountersAmong (Macros.exactly 2)
                             (Just (PrintedKind Macros.plusOnePlusOne))
                             (Macros.allOf Macros.creatureYouControl))])
                   ((Macros.draw You (Lit 1)))

||| Cyclone
public export
cycloneUpkeepPayment : Ability
cycloneUpkeepPayment =
  Macros.triggered At (BeginningOf Upkeep (Macros.yours))
    (Sequentially
       [ PutCounters (Lit 1) (PrintedKind (Named "Wind")) Macros.thisEnchantment
       , (May You (Pay You (ScaledMana (RunUnit [Macros.pip Green])
                                (Times 1 (CountersOn (Named "Wind") Macros.thisEnchantment)))
                PaidOnce) Nothing (Just (Macros.sacrifice You Macros.thisEnchantment))) ])

||| War Tax
public export
warTaxScaledPayment : Cost [letterB X]
warTaxScaledPayment =
  ScaledMana GenericUnit
             (TimesOf (LetterVal X) (Macros.countOf (And [Macros.creature, Attacking])))

||| Rune Snag
public export
runeSnag : Card
runeSnag =
  Macros.card "Rune Snag" (Just [Macros.generic 1, Macros.pip Blue]) []
       (MkTypeLine [] [Instant])
       [ Spell ((May (Macros.controllerOf (Macros.target Macros.spell)) (Pay They (Compound [Mana [Macros.generic 2],
                                       ScaledMana GenericUnit
                                         (Times 2 (Macros.countOf
                                            (And [Named (PrintedName "Rune Snag"),
                                                  InZone (ZoneAt Graveyard Bare)])))])
                       PaidOnce) Nothing (Just (Macros.counterSpell It)))) ]
       Nothing

||| Elemental Resonance
public export
elementalResonance : Card
elementalResonance =
  Macros.card "Elemental Resonance"
       (Just [Macros.generic 2, Macros.pip Green, Macros.pip Green]) []
       (MkTypeLine [enchantmentType "Aura"] [Enchantment])
       [ Macros.keywordSubject "Enchant" Permanent
       , Macros.triggered At (BeginningOf FirstMain (Macros.yours))
           (AddMana You (Lit 1)
                    (AsPrintedCost (AttachHost Enchanted PermanentW)) []) ]
       Nothing



mizziumTransreliquat : Card
mizziumTransreliquat =
  Macros.card "Mizzium Transreliquat" (Just [Macros.generic 3]) []
       (MkTypeLine [] [Artifact])
       [ Macros.activated (Mana [Macros.generic 3])
                          (Continuously {ts = StaticFirstDone}
                      (BecomesCopy Macros.thisArtifact
                                   (Macros.target Macros.artifact) [])
                      (Just Macros.untilEndOfTurn))
       , Macros.activated (Mana [Macros.generic 1, Macros.pip Blue, Macros.pip Red])
                          (Continuously {ts = StaticFirstDone}
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
                  (TokenCopyOf (Macros.target (And [Macros.creature, HasPossessor ControllerAx You]))
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
                  (TokenCopyOf (Macros.target (And [Macros.creature, HasPossessor ControllerAx You])) [])
                  []) ]
       Nothing

||| Saheeli Rai
saheelisCopy : Effect []
saheelisCopy =
  Sequentially [ Create You (Lit 1)
                   (TokenCopyOf (Macros.target (And [Or [Macros.artifact, Macros.creature],
                                                     HasPossessor ControllerAx You]))
                                [ExceptTypes (MkTypeLine [] [Artifact])])
                   []
               , Macros.gainsHaste (That TokenW) Nothing
               , Macros.delayed (BeginningOf EndStep NoPossessor) (Macros.exile You It) ]

||| Clone
public export
clone : Card
clone =
  Macros.card "Clone" (Just [Macros.generic 3, Macros.pip Blue]) []
       (MkTypeLine [creatureType "Shapeshifter"] [Creature])
       [ Static (EntersRider Macros.thisCreature (AsCopyOf True
                              (Macros.a Macros.creature) [])) ]
       (Just (0, 0))

||| Quicksilver Gargantuan
public export
quicksilverGargantuan : Card
quicksilverGargantuan =
  Macros.card "Quicksilver Gargantuan"
       (Just [Macros.generic 5, Macros.pip Blue, Macros.pip Blue]) []
       (MkTypeLine [creatureType "Shapeshifter"] [Creature])
       [ Static (EntersRider Macros.thisCreature (AsCopyOf True
                              (Macros.a Macros.creature)
                              [ExceptPt (Lit 7) (Lit 7)])) ]
       (Just (7, 7))

||| Sculpting Steel
public export
sculptingSteel : Card
sculptingSteel =
  Macros.card "Sculpting Steel" (Just [Macros.generic 3]) []
       (MkTypeLine [] [Artifact])
       [ Static (EntersRider Macros.thisArtifact (AsCopyOf True
                              (Macros.a Macros.artifact) [])) ]
       Nothing

||| Sakashima's Student
public export
sakashimasStudentCopy : Ability
sakashimasStudentCopy =
  Static (EntersRider Macros.thisCreature (AsCopyOf True
                       (Macros.a Macros.creature)
                       [ExceptTypes (MkTypeLine [creatureType "Ninja"] [])]))

||| Croaking Counterpart
public export
croakingCounterpartCopy : Effect []
croakingCounterpartCopy =
  Create You (Lit 1)
    (TokenCopyOf (Macros.target (And [ Macros.creature
                                     , Not (HasSubtype (creatureType "Frog")) ]))
                 [ExceptChars (MkToken (Just (Lit 1 ** Lit 1)) [Green]
                                       (MkTypeLine [creatureType "Frog"] [])
                                       [] Nothing)
                              False])
    []

||| Chameleon, Master of Disguise
public export
chameleonCopy : Ability
chameleonCopy =
  Static (EntersRider Macros.thisCreature (AsCopyOf True
                       (Macros.a (And [Macros.creature, HasPossessor ControllerAx You]))
                       [ExceptName "Chameleon, Master of Disguise"]))

||| Repeated Reverberation
public export
repeatedReverberation : Card
repeatedReverberation =
  Macros.card "Repeated Reverberation"
       (Just [Macros.generic 3, Macros.pip Red, Macros.pip Red]) []
       (MkTypeLine [] [Sorcery])
       [ Spell (Delayed
                  (Casts You (Macros.a (And [Macros.instant, Macros.spell])) Nothing)
                  [ Casts You (Macros.a (And [Macros.sorcery, Macros.spell])) Nothing
                  , Activates You (Macros.a (AbilityHead LoyaltyClass)) ]
                  (Just Macros.thisTurn)
                  (Sequentially
                     [ CopyStack You (That AbilityJoinW) (Lit 2) []
                     , Macros.may You (ChooseNewTargets (Those CopyJoinW)) ])) ]
       Nothing

||| Frontline Heroism
public export
frontlineHeroismCopy : Ability
frontlineHeroismCopy =
  Macros.triggered Whenever
    (Casts You
       (Macros.a (And [ Macros.spell
                      , Targets (Macros.a (And [Macros.creature, HasPossessor ControllerAx You]))
                                SoleTarget ]))
       Nothing)
    (Sequentially
       [ Macros.create (Lit 1)
           (MkToken (Just (Lit 1 ** Lit 1)) [Red]
                    (MkTypeLine [creatureType "Soldier"] [Creature])
                    [Macros.keyword "Haste"] Nothing)
       , CopyStack You (That SpellW) (Lit 1) []
       , CopyTargets (That CopyW) (That TokenW) ])

||| Flawless Forgery
public export
flawlessForgeryLine : Effect []
flawlessForgeryLine =
  Sequentially
    [ Macros.exile You (Macros.target (And [ Macros.instantOrSorcery
                                       , InZone (Macros.graveyardOf
                                                   (Macros.a Opponent)) ]))
    , CopyCard You (That CardW) (Lit 1)
    , Continuously {ts = StaticFirstDone} (Macros.mayPlayDeed "Cast" You (That CopyW)
                       (PlayRider Nothing Nothing Nothing False Macros.free))
                   Nothing ]

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
                          (Macros.controllerOf (Macros.target
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

||| Strionic Resonator
public export
strionicResonator : Card
strionicResonator =
  Macros.card "Strionic Resonator" (Just [Macros.generic 2]) []
       (MkTypeLine [] [Artifact])
       [ Macros.activated (Compound [Mana [Macros.generic 2], TapSymbol])
           (Sequentially
              [ CopyStack You
                  (Macros.target (And [AbilityHead AnyTriggered, HasPossessor ControllerAx You]))
                  (Lit 1) []
              , Macros.may You (ChooseNewTargets (That AbilityCopyW)) ]) ]
       Nothing

||| Mister Fantastic
public export
misterFantasticCopy : Ability
misterFantasticCopy =
  Macros.activated (Compound [ Mana [ Macros.pip Red, Macros.pip Green
                                    , Macros.pip White, Macros.pip Blue ]
                             , TapSymbol ])
    (Sequentially
       [ CopyStack You
           (Macros.target (And [AbilityHead AnyTriggered, HasPossessor ControllerAx You]))
           (Lit 2) []
       , Macros.may You (ChooseNewTargets (Those AbilityCopyW)) ])

||| Rowan's Talent
public export
rowansTalentCopy : Ability
rowansTalentCopy =
  Macros.triggered Whenever
    (Activates You
       (Macros.a (And [ AbilityHead LoyaltyClass
                      , AbilityOf (AttachHost Enchanted (TypeW Planeswalker)) ])))
    (Sequentially
       [ CopyStack You (That AbilityW) (Lit 1) []
       , Macros.may You (ChooseNewTargets (That AbilityCopyW)) ])

||| Rings of Brighthearth
public export
ringsOfBrighthearth : Card
ringsOfBrighthearth =
  Macros.card "Rings of Brighthearth" (Just [Macros.generic 3]) []
       (MkTypeLine [] [Artifact])
       [ Macros.triggeredIf Whenever
           (Activates You (Macros.a (AbilityHead AnyActivated)))
           (Macros.itIsntAnAbility IsManaAbility)
           ((May You (Pay You (Mana [Macros.generic 2]) PaidOnce) (Just (Sequentially
                 [ CopyStack You (That AbilityW) (Lit 1) []
                 , Macros.may You (ChooseNewTargets (That AbilityCopyW)) ])) Nothing)) ]
       Nothing

||| Iron Man, Bleeding Edge
public export
ironManBleedingEdge : Card
ironManBleedingEdge =
  Macros.card "Iron Man, Bleeding Edge"
       (Just [Macros.generic 3, Macros.pip Blue, Macros.pip Blue]) [Legendary]
       (MkTypeLine [creatureType "Human", creatureType "Hero"]
                   [Artifact, Creature])
       [ Macros.keyword "Flying"
       , Macros.triggeredOnlyOnce Whenever
           (Casts You (Macros.a (And [Macros.artifact, Macros.spell])) Nothing)
           ActionOncePerTurn
           (Macros.may You (CopyStack You It (Lit 1) [ExceptNonlegendary])) ]
       (Just (3, 5))

||| Donal, Herald of Wings
public export
donalHeraldOfWings : Card
donalHeraldOfWings =
  Macros.card "Donal, Herald of Wings"
       (Just [Macros.generic 2, Macros.pip Blue, Macros.pip Blue]) [Legendary]
       (MkTypeLine [creatureType "Human", creatureType "Wizard"] [Creature])
       [ Macros.triggeredOnlyOnce Whenever
           (Casts You
              (Macros.a (And [ Macros.creature, Macros.spell
                             , Not (HasSupertype Legendary)
                             , HasKeyword (TheKeyword "Flying") ]))
              Nothing)
           ActionOncePerTurn
           (Macros.may You
              (CopyStack You It (Lit 1)
                 [ExceptChars (MkToken (Just (Lit 1 ** Lit 1)) []
                                       (MkTypeLine [creatureType "Spirit"] [])
                                       [] Nothing)
                              True])) ]
       (Just (3, 3))

public export
tawnosTheToymaker : Card
tawnosTheToymaker =
  Macros.card "Tawnos, the Toymaker"
       (Just [Macros.generic 3, Macros.pip Green, Macros.pip Blue]) [Legendary]
       (MkTypeLine [creatureType "Human", creatureType "Artificer"] [Creature])
       [ Macros.triggered Whenever
                          (Casts You (Macros.a (And [Or [HasSubtype (creatureType "Beast"), HasSubtype (creatureType "Bird")],
                                              Macros.creature, Macros.spell])) Nothing)
                          (Macros.may You
                      (CopyStack You It (Lit 1)
                                 [ExceptTypes (MkTypeLine [] [Artifact])])) ]
       (Just (3, 5))

public export
etherealHaze : Card
etherealHaze =
  Macros.card "Ethereal Haze" (Just [Macros.pip White]) []
       (MkTypeLine [spellType "Arcane"] [Instant])
       [ Spell (Macros.preventAllBy AnyDamage (Macros.allOf Macros.creature)
                                    Everywhere (Just Macros.thisTurn)) ]
       Nothing

public export
defang : Card
defang =
  Macros.card "Defang" (Just [Macros.generic 1, Macros.pip White]) []
       (MkTypeLine [enchantmentType "Aura"] [Enchantment])
       [ Macros.keywordSubject "Enchant" Macros.creature
       , Static (Prevents AnyDamage (DealtBy (AttachHost Enchanted (TypeW Creature))) Everywhere
                          CutAll Repeatedly Nothing) ]
       Nothing

public export
sphereOfPurity : Card
sphereOfPurity =
  Macros.card "Sphere of Purity" (Just [Macros.generic 3, Macros.pip White]) []
       (MkTypeLine [] [Enchantment])
       [ Static (Prevents AnyDamage (DealtBy (Macros.a Macros.artifact))
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
                  , Continuously {ts = StaticFirstDone}
                      (Prevents AnyDamage (DealtBy (That (TypeW Creature)))
                                    Everywhere CutAll NextTimeOnly Nothing)
                      (Just Macros.thisTurn) ]) ]
       Nothing

public export
excruciator : Card
excruciator =
  Macros.card "Excruciator"
       (Just [Macros.generic 6, Macros.pip Red, Macros.pip Red]) []
       (MkTypeLine [creatureType "Avatar"] [Creature])
       [ Static (CantPrevent AnyDamage (DamageDescribed (DealtBy Macros.thisCreature) Everywhere)
                                 NoPreventionOnly) ]
       (Just (7, 7))

public export
flaringPain : Card
flaringPain =
  Macros.card "Flaring Pain" (Just [Macros.generic 1, Macros.pip Red]) []
       (MkTypeLine [] [Instant])
       [ Spell (Continuously {ts = StaticFirstDone} (CantPrevent AnyDamage (DamageDescribed Unattributed Everywhere) NoPreventionOnly)
                             (Just Macros.thisTurn)) ]
       Nothing

public export
thunderstaff : Card
thunderstaff =
  Macros.card "Thunderstaff" (Just [Macros.generic 3]) []
       (MkTypeLine [] [Artifact])
       [ Static (Macros.asLongAs (Matches Macros.thisArtifact (HasStatus Untapped))
                                 (Prevents CombatOnly
                                             (DealtBy (Macros.a Macros.creature))
                                             (Macros.shieldingIt You)
                                             (CutSome (Lit 1)) Repeatedly Nothing))
       , Macros.activated (Compound [Mana [Macros.generic 2], TapSymbol])
                          (Macros.gets (Macros.allOf (And [Macros.creature, Attacking]))
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
                      [ Continuously {ts = StaticFirstDone}
                          (Becomes Macros.thisPlaneswalker Sets (Bundle (MkToken (Just (Lit 5 ** Lit 5)) []
                                             (MkTypeLine [creatureType "Human", creatureType "Soldier", creatureType "Ally"] [Creature])
                                             [Macros.keyword "Indestructible"] Nothing) (Just Planeswalker)))
                          (Just Macros.untilEndOfTurn)
                      , Macros.preventAll AnyDamage (Macros.shieldingIt It)
                                          (Just Macros.thisTurn) ])
       , Macros.activated (LoyaltySymbol LoyaltyZero)
                          (Macros.create (Lit 1) (Macros.creatureTok 2 2 [White] [creatureType "Knight", creatureType "Ally"]))
       , Macros.activated (LoyaltySymbol (LoyaltyDown 4))
                          (GetsEmblem You [Static (Gets (Macros.allOf Macros.creatureYouControl)
                                                 (PtUp (Lit 1)) (PtUp (Lit 1)))]) ]
       (Macros.loyaltyBox 4)

public export
gideonJura : Card
gideonJura =
  Macros.cardOf "Gideon Jura"
       (Just [Macros.generic 3, Macros.pip White, Macros.pip White]) [Legendary]
       (MkTypeLine [planeswalkerType "Gideon"] [Planeswalker])
       [ Macros.activated (LoyaltySymbol (LoyaltyUp 2))
           (Macros.throughout (DuringNextTurnOf (Macros.target Opponent))
                       (Macros.deontic (Macros.allOf (And [Macros.creature,
                                             HasPossessor ControllerAx (That PlayerW)]))
                                Require ["Attack"] Agent
                                (DefendingPlayer Macros.thisPlaneswalker)))
       , Macros.activated (LoyaltySymbol (LoyaltyDown 2))
           (Macros.destroy (Macros.target (And [Macros.creature, Macros.tapped])))
       , Macros.activated (LoyaltySymbol LoyaltyZero)
           (Sequentially
              [ Continuously {ts = StaticFirstDone}
                  (Becomes Macros.thisPlaneswalker Sets (Bundle (MkToken (Just (Lit 6 ** Lit 6)) []
                                     (MkTypeLine [creatureType "Human",
                                                  creatureType "Soldier"] [Creature])
                                     [] Nothing) (Just Planeswalker)))
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
       [ Spell (Continuously {ts = StaticFirstDone}
                  (Redirects CombatOnly Unattributed (Macros.shieldingIt You) CutAll
                             (Macros.target (And [Macros.creature, Attacking])) Repeatedly)
                  (Just Macros.thisTurn)) ]
       Nothing

public export
pariah : Card
pariah =
  Macros.card "Pariah" (Just [Macros.generic 2, Macros.pip White]) []
       (MkTypeLine [enchantmentType "Aura"] [Enchantment])
       [ Macros.keywordSubject "Enchant" Macros.creature
       , Static (Redirects AnyDamage Unattributed (Macros.shieldingIt You) CutAll
                           (AttachHost Enchanted (TypeW Creature)) Repeatedly) ]
       Nothing

public export
martyrsOfKorlis : Card
martyrsOfKorlis =
  Macros.card "Martyrs of Korlis"
       (Just [Macros.generic 3, Macros.pip White, Macros.pip White]) []
       (MkTypeLine [creatureType "Human"] [Creature])
       [ Static (Macros.asLongAs (Matches Macros.thisCreature (HasStatus Untapped))
                                 (Redirects AnyDamage (DealtBy (Macros.allOf Macros.artifact))
                                          (Macros.shieldingIt You)
                                          CutAll Macros.thisCreature Repeatedly)) ]
       (Just (1, 6))

public export
wardOfPiety : Card
wardOfPiety =
  Macros.card "Ward of Piety" (Just [Macros.generic 1, Macros.pip White]) []
       (MkTypeLine [enchantmentType "Aura"] [Enchantment])
       [ Macros.keywordSubject "Enchant" Macros.creature
       , Macros.activated (Mana [Macros.generic 1, Macros.pip White])
                          (Continuously {ts = StaticFirstDone}
                      (Redirects AnyDamage Unattributed
                                 (Macros.shieldingIt (AttachHost Enchanted (TypeW Creature)))
                                 (Shield (Lit 1)) (Macros.target Macros.anyTarget) Repeatedly)
                      (Just Macros.thisTurn)) ]
       Nothing

public export
mirrorwoodTreefolk : Card
mirrorwoodTreefolk =
  Macros.card "Mirrorwood Treefolk" (Just [Macros.generic 3, Macros.pip Green]) []
       (MkTypeLine [creatureType "Treefolk"] [Creature])
       [ Macros.activated (Mana [Macros.generic 2, Macros.pip Red, Macros.pip White])
                          (Continuously {ts = StaticFirstDone}
                      (Redirects AnyDamage Unattributed
                                     (Macros.shieldingIt Macros.thisCreature)
                                     CutAll (Macros.target Macros.anyTarget) NextTimeOnly)
                      (Just Macros.thisTurn)) ]
       (Just (2, 4))

public export
stormwildCapridor : Card
stormwildCapridor =
  Macros.card "Stormwild Capridor" (Just [Macros.generic 2, Macros.pip White]) []
       (MkTypeLine [creatureType "Bird", creatureType "Goat"] [Creature])
       [ Macros.keyword "Flying"
       , Static (Prevents NoncombatOnly Unattributed
                              (Macros.shieldingIt Macros.thisCreature)
                              CutAll Repeatedly
                              (Just (PutCounters PreventedThisWay
                                                 (PrintedKind Macros.plusOnePlusOne)
                                                 Macros.thisCreature))) ]
       (Just (1, 3))

public export
carom : Card
carom =
  Macros.card "Carom" (Just [Macros.generic 1, Macros.pip White]) []
       (MkTypeLine [] [Instant])
       [ Spell (Sequentially
                  [ Continuously {ts = StaticFirstDone}
                      (Redirects AnyDamage Unattributed
                                 (Macros.shieldingIt (Macros.target Macros.creature))
                                 (Shield (Lit 1))
                                 (Macros.target (And [Macros.creature, Other])) Repeatedly)
                      (Just Macros.thisTurn)
                  , (Macros.draw You (Lit 1)) ]) ]
       Nothing

public export
daughterOfAutumn : Card
daughterOfAutumn =
  Macros.card "Daughter of Autumn"
       (Just [Macros.generic 2, Macros.pip Green, Macros.pip Green]) [Legendary]
       (MkTypeLine [creatureType "Avatar"] [Creature])
       [ Macros.activated (Mana [Macros.pip White])
                          (Continuously {ts = StaticFirstDone}
                      (Redirects AnyDamage Unattributed
                                 (Macros.shieldingIt
                                    (Macros.target (And [Macros.creature, ColorIs White])))
                                 (Shield (Lit 1)) Macros.thisCreature Repeatedly)
                      (Just Macros.thisTurn)) ]
       (Just (2, 4))

public export
aegisOfHonor : Card
aegisOfHonor =
  Macros.card "Aegis of Honor" (Just [Macros.pip White]) []
       (MkTypeLine [] [Enchantment])
       [ Macros.activated (Mana [Macros.generic 1])
                          (Continuously {ts = StaticFirstDone}
                      (Redirects AnyDamage
                                     (DealtBy (Macros.a Macros.instantOrSorcery))
                                     (Macros.shieldingIt You) CutAll (Macros.controllerOf It)
                                     NextTimeOnly)
                      (Just Macros.thisTurn)) ]
       Nothing

public export
testOfFaith : Card
testOfFaith =
  Macros.card "Test of Faith" (Just [Macros.generic 1, Macros.pip White]) []
       (MkTypeLine [] [Instant])
       [ Spell (Continuously {ts = StaticFirstDone}
                  (Prevents AnyDamage Unattributed
                            (Macros.shieldingIt (Macros.target Macros.creature))
                            (Shield (Lit 3)) Repeatedly
                            (Just (PutCounters PreventedThisWay
                                               (PrintedKind Macros.plusOnePlusOne)
                                               (That (TypeW Creature)))))
                  (Just Macros.thisTurn)) ]
       Nothing

public export
temper : Card
temper =
  Macros.card "Temper" (Just [Variable, Macros.generic 1, Macros.pip White]) []
       (MkTypeLine [] [Instant])
       [ Spell (Continuously {ts = StaticFirstDone}
                  (Prevents AnyDamage Unattributed
                            (Macros.shieldingIt (Macros.target Macros.creature))
                            (Shield (LetterVal X)) Repeatedly
                            (Just (PutCounters PreventedThisWay
                                               (PrintedKind Macros.plusOnePlusOne)
                                               (That (TypeW Creature)))))
                  (Just Macros.thisTurn)) ]
       Nothing

public export
candlesGlow : Card
candlesGlow =
  Macros.card "Candles' Glow" (Just [Macros.generic 1, Macros.pip White]) []
       (MkTypeLine [spellType "Arcane"] [Instant])
       [ Spell (Continuously {ts = StaticFirstDone}
                  (Prevents AnyDamage Unattributed
                            (Macros.shieldingIt (Macros.target Macros.anyTarget))
                            (Shield (Lit 3)) Repeatedly
                            (Just (ChangeLife You (Up PreventedThisWay))))
                  (Just Macros.thisTurn)) ]
       Nothing

||| Inkshield
inkshieldRider : Effect []
inkshieldRider =
  Continuously {ts = StaticFirstDone}
    (Prevents AnyDamage Unattributed (Macros.shieldingIt You) CutAll Repeatedly
              (Just (Macros.create PreventedThisWay
                                   (Macros.creatureTok 2 1 [White, Black] []))))
    (Just Macros.thisTurn)

public export
urzasArmor : Card
urzasArmor =
  Macros.card "Urza's Armor" (Just [Macros.generic 6]) []
       (MkTypeLine [] [Artifact])
       [ Static (Prevents AnyDamage (DealtBy (Macros.a Macros.source))
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
                          (Continuously {ts = StaticFirstDone}
                      (Prevents AnyDamage
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
                  [ Continuously {ts = StaticFirstDone}
                      (Prevents AnyDamage (DealtBy (Macros.aYourChoice Macros.source))
                                (Macros.shieldingIt (Macros.target Macros.anyTarget))
                                (Shield (Lit 3)) Repeatedly Nothing)
                      (Just Macros.thisTurn)
                  , ChangeLife You (Up (Lit 3)) ]) ]
       Nothing

public export
reverseDamage : Card
reverseDamage =
  Macros.card "Reverse Damage"
       (Just [Macros.generic 1, Macros.pip White, Macros.pip White]) []
       (MkTypeLine [] [Instant])
       [ Spell (Continuously {ts = StaticFirstDone}
                  (Prevents AnyDamage
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
       [ Spell (Continuously {ts = StaticFirstDone}
                  (Prevents AnyDamage
                                (DealtBy (Macros.aYourChoice Macros.source))
                                (Macros.shieldingIt You) CutAll NextTimeOnly
                                (Just (DealDamage This ThatMuch (Macros.controllerOf It))))
                  (Just Macros.thisTurn)) ]
       Nothing

public export
pinpointAvalanche : Card
pinpointAvalanche =
  Macros.card "Pinpoint Avalanche"
       (Just [Macros.generic 3, Macros.pip Red, Macros.pip Red]) []
       (MkTypeLine [] [Instant])
       [ Spell (Sequentially
                  [ DealDamage This (Lit 4) (Macros.target Macros.creature)
                  , Continuously {ts = StaticFirstDone}
                      (CantPrevent AnyDamage ThatDamage NoPreventionOnly) Nothing ]) ]
       Nothing

public export
whippoorwillImmunity : Effect []
whippoorwillImmunity =
  Continuously {ts = StaticFirstDone}
    (CantPrevent AnyDamage
                 (DamageDescribed Unattributed (Macros.shieldingIt (Macros.a Macros.creature)))
                 NoRedirectEither)
    (Just Macros.thisTurn)

public export
phytohydra : Card
phytohydra =
  Macros.card "Phytohydra"
       (Just [Macros.generic 2, Macros.pip Green, Macros.pip White,
              Macros.pip White]) []
       (MkTypeLine [creatureType "Plant", creatureType "Hydra"] [Creature])
       [ Static (Intercepts (IsDealtDamage AnyDamage Macros.thisCreature) [] Nothing
                            (PutCounters ThatMuch
                                         (PrintedKind Macros.plusOnePlusOne) It)
                            Repeatedly Nothing) ]
       (Just (1, 1))

public export
nefariousLichGain : AbilityAt []
nefariousLichGain =
  Static (Intercepts (LifeChanges You LifeGoesUp) [] Nothing
                     (Draw You ThatMuch) Repeatedly Nothing)

public export
piousWarrior : Card
piousWarrior =
  Macros.card "Pious Warrior"
       (Just [Macros.generic 3, Macros.pip White]) []
       (MkTypeLine [creatureType "Human", creatureType "Rebel",
                    creatureType "Warrior"] [Creature])
       [ Macros.triggered Whenever
                          (IsDealtDamage CombatOnly Macros.thisCreature)
                          (Macros.gainsLife You ThatMuch) ]
       (Just (2, 3))

public export
chandrasSpitfire : Card
chandrasSpitfire =
  Macros.card "Chandra's Spitfire"
       (Just [Macros.generic 2, Macros.pip Red]) []
       (MkTypeLine [creatureType "Elemental"] [Creature])
       [ Macros.keyword "Flying"
       , Macros.triggered Whenever
                          (IsDealtDamage NoncombatOnly (Macros.a Opponent))
                          (Macros.gets Macros.thisCreature (PtUp (Lit 3)) (PtUp (Lit 0))
                                       (Just Macros.untilEndOfTurn)) ]
       (Just (1, 3))

public export
sanguineBond : Card
sanguineBond =
  Macros.card "Sanguine Bond"
       (Just [Macros.generic 3, Macros.pip Black, Macros.pip Black]) []
       (MkTypeLine [] [Enchantment])
       [ Macros.triggered Whenever (LifeChanges You LifeGoesUp)
                          (ChangeLife (Macros.target Opponent) (Down ThatMuch)) ]
       Nothing

public export
exquisiteBlood : Card
exquisiteBlood =
  Macros.card "Exquisite Blood"
       (Just [Macros.generic 4, Macros.pip Black]) []
       (MkTypeLine [] [Enchantment])
       [ Macros.triggered Whenever (LifeChanges (Macros.anOpponent) LifeGoesDown)
                          (Macros.gainsLife You ThatMuch) ]
       Nothing

public export
agelessEntity : Card
agelessEntity =
  Macros.card "Ageless Entity"
       (Just [Macros.generic 3, Macros.pip Green, Macros.pip Green]) []
       (MkTypeLine [creatureType "Elemental"] [Creature])
       [ Macros.triggered Whenever (LifeChanges You LifeGoesUp)
                          (PutCounters ThatMuch (PrintedKind Macros.plusOnePlusOne)
                                       Macros.thisCreature) ]
       (Just (4, 4))

public export
templeAltisaur : Card
templeAltisaur =
  Macros.card "Temple Altisaur"
       (Just [Macros.generic 4, Macros.pip White]) []
       (MkTypeLine [creatureType "Dinosaur"] [Creature])
       [ Static (Prevents AnyDamage (DealtBy (Macros.a Macros.source))
                              (Macros.shieldingIt
                                 (Macros.a (And [HasSubtype (creatureType "Dinosaur"),
                                                 HasPossessor ControllerAx You])))
                              (CutAllBut (Lit 1)) Repeatedly Nothing) ]
       (Just (3, 4))

public export
darkSphere : Card
darkSphere =
  Macros.card "Dark Sphere" (Just []) []
       (MkTypeLine [] [Artifact])
       [ Macros.activated
           (Compound [TapSymbol, Do (Macros.sacrifice You Macros.thisArtifact)])
                          (Continuously {ts = StaticFirstDone}
                             (Prevents AnyDamage
                                           (DealtBy (Macros.aYourChoice Macros.source))
                                           (Macros.shieldingIt You)
                                           (CutHalf RoundDown) NextTimeOnly Nothing)
                             (Just Macros.thisTurn)) ]
       Nothing

public export
shadowbane : Card
shadowbane =
  Macros.card "Shadowbane" (Just [Macros.generic 1, Macros.pip White]) []
       (MkTypeLine [] [Instant])
       [ Spell (Continuously {ts = StaticFirstDone}
                  (Prevents AnyDamage
                                (DealtBy (Macros.aYourChoice Macros.source))
                                (Macros.shieldingIt
                                   (Macros.youAnd (Macros.allOf Macros.creatureYouControl)))
                                CutAll NextTimeOnly
                                (Just (If (PreventedFromSource
                                             (And [Macros.source, ColorIs Black]))
                                          (Macros.gainsLife You PreventedThisWay)
                                          Nothing)))
                  (Just Macros.thisTurn)) ]
       Nothing

||| Honorable Passage

public export
sphereOfLaw : Card
sphereOfLaw =
  Macros.card "Sphere of Law" (Just [Macros.generic 3, Macros.pip White]) []
       (MkTypeLine [] [Enchantment])
       [ Static (Prevents AnyDamage
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

||| Firesong and Sunspeaker
public export
firesongJoinEcho : Effect []
firesongJoinEcho =
  Sequentially [ DealDamage This (Lit 3)
                   (Macros.target (Macros.kindJoin AnyPlayer Macros.creature))
               , DealDamage This (Lit 1) (Macros.thatJoin) ]

||| Sorrow's Path
public export
sorrowsPathCouldBlock : Predicate [] Object
sorrowsPathCouldBlock = CombatRel CouldBlock (Macros.allOf (And [Macros.creature, Attacking]))

||| General Jarkeld's
public export
generalJarkeldCouldBeBlocked : Predicate [] Object
generalJarkeldCouldBeBlocked =
  CombatRel CouldBeBlockedBy (Macros.allOf (And [Macros.creature, Blocking]))

||| Sorrow's Path
public export
sorrowsPathReassign : Effect []
sorrowsPathReassign =
  Sequentially
    [ RemoveFromCombat (Macros.target (And [Macros.creature, Blocking]))
    , BecomesBlocking (That (TypeW Creature))
                      (Macros.a (And [Macros.creature, Attacking])) ]

||| General Jarkeld's
public export
generalJarkeldReassign : Effect []
generalJarkeldReassign =
  Sequentially
    [ StopsBlocking Macros.thisCreature
                    (Macros.target (And [Macros.creature, Attacking]))
    , BecomesBlocking Macros.thisCreature
                      (Macros.a (And [Macros.creature, Attacking])) ]

||| Tahngarth
public export
tahngarthChoosesDefender : Effect []
tahngarthChoosesDefender =
  Choose (Macros.a (Macros.kindJoin AnyPlayer (HasType Planeswalker))) Nothing Openly

public export
tahngarthAttacksThatJoin : Effect (effIntro Cards.tahngarthChoosesDefender)
tahngarthAttacksThatJoin =
  BecomesAttacking Macros.thisCreature (OneDefender Macros.thatJoin)

||| Smite
public export
smite : Card
smite =
  Macros.card "Smite" (Just [Macros.pip White]) []
       (MkTypeLine [] [Instant])
       [ Spell (Macros.destroy (Macros.target (And [Macros.creature, Blocked]))) ]
       Nothing

||| Forcefield
public export
forcefield : Card
forcefield =
  Macros.card "Forcefield" (Just [Macros.generic 3]) []
       (MkTypeLine [] [Artifact])
       [ Macros.activated (Mana [Macros.generic 1])
           (Continuously {ts = StaticFirstDone}
              (Prevents CombatOnly
                            (DealtBy (Macros.aYourChoice
                                        (And [Macros.creature, Unblocked])))
                            (Macros.shieldingIt You)
                            (CutAllBut (Lit 1)) NextTimeOnly Nothing)
              (Just Macros.thisTurn)) ]
       Nothing

||| Agate-Blade Assassin
public export
agateBladeAssassin : Card
agateBladeAssassin =
  Macros.card "Agate-Blade Assassin"
       (Just [Macros.generic 1, Macros.pip Black]) []
       (MkTypeLine [creatureType "Lizard", creatureType "Assassin"] [Creature])
       [ Macros.triggered Whenever (Attacks Macros.thisCreature NoDefender)
           (Sequentially [ ChangeLife TheDefendingPlayer (Down (Lit 1))
                         , ChangeLife You (Up (Lit 1)) ]) ]
       (Just (1, 3))

||| Fiend Binder
public export
fiendBinder : Card
fiendBinder =
  Macros.card "Fiend Binder"
       (Just [Macros.generic 3, Macros.pip White]) []
       (MkTypeLine [creatureType "Human", creatureType "Soldier"] [Creature])
       [ Macros.triggered Whenever (Attacks Macros.thisCreature NoDefender)
           (SetStatus Tapped
              (Macros.target (And [Macros.creature,
                                   HasPossessor ControllerAx TheDefendingPlayer]))) ]
       (Just (3, 2))

||| Souls of the Faultless
public export
soulsOfTheFaultlessDrain : Ability
soulsOfTheFaultlessDrain =
  Macros.triggered Whenever (IsDealtDamage CombatOnly Macros.thisCreature)
                   (ChangeLife TheAttackingPlayer (Down ThatMuch))

||| Blessed Reversal
public export
blessedReversal : Card
blessedReversal =
  Macros.card "Blessed Reversal"
       (Just [Macros.generic 1, Macros.pip White]) []
       (MkTypeLine [] [Instant])
       [ Spell (ChangeLife You
                  (Up (Times 3 (Macros.countOf (And [Macros.creature, CombatRel AttackerOf You]))))) ]
       Nothing

public export
endure : Card
endure =
  Macros.card "Endure"
       (Just [Macros.generic 3, Macros.pip White, Macros.pip White]) []
       (MkTypeLine [] [Instant])
       [ Spell (Macros.preventAll AnyDamage
                                  (Macros.shieldingIt
                                     (Macros.youAnd (Macros.allOf (And [Permanent,
                                                          HasPossessor ControllerAx You]))))
                                  (Just Macros.thisTurn)) ]
       Nothing

public export
harmsWay : Card
harmsWay =
  Macros.card "Harm's Way" (Just [Macros.pip White]) []
       (MkTypeLine [] [Instant])
       [ Spell (Continuously {ts = StaticFirstDone}
                  (Redirects AnyDamage (DealtBy (Macros.aYourChoice Macros.source))
                             (Macros.shieldingIt
                                (Macros.youAnd (Macros.allOf (And [Permanent,
                                                     HasPossessor ControllerAx You]))))
                             (Shield (Lit 2))
                             (Macros.target Macros.anyTarget) Repeatedly)
                  (Just Macros.thisTurn)) ]
       Nothing

public export
divineDeflection : Card
divineDeflection =
  Macros.card "Divine Deflection" (Just [Variable, Macros.pip White]) []
       (MkTypeLine [] [Instant])
       [ Spell (Continuously {ts = StaticFirstDone}
                  (Prevents AnyDamage Unattributed
                            (Macros.shieldingIt
                               (Macros.youAnd (Macros.allOf (And [Permanent,
                                                    HasPossessor ControllerAx You]))))
                            (Shield (LetterVal X)) Repeatedly
                            (Just (DealDamage This ThatMuch
                                              (Macros.target Macros.anyTarget))))
                  (Just Macros.thisTurn)) ]
       Nothing

public export
glarecasterShield : Ability
glarecasterShield =
  Macros.activated (Mana [Macros.generic 5, Macros.pip White])
                   (Continuously {ts = StaticFirstDone}
               (Redirects AnyDamage Unattributed
                              (Macros.shieldingIt (Macros.youAnd Macros.thisCreature))
                              CutAll (Macros.target Macros.anyTarget)
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
                        (Macros.a (And [Macros.source, HasPossessor ControllerAx You]))
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
       [ Macros.triggered When (Enters Macros.thisEnchantment Nothing) (Macros.draw You (Lit 1))
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
                                        HasPossessor ControllerAx You]))
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
           , Macros.discard You (Macros.a (InZone Macros.handZ))
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
           , Macros.scry You (Lit 3)
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
       , Static (Prevents AnyDamage
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
                          (Enters (Macros.a (And [Macros.creature, OtherThan This])) Nothing)
                          (Continuously {ts = StaticFirstDone}
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
                      (Macros.allOf (And [Macros.enchantment, OtherThan This]))) ]
       Nothing

public export
wordsOfWisdom : Card
wordsOfWisdom =
  Macros.card "Words of Wisdom"
       (Just [Macros.generic 1, Macros.pip Blue]) []
       (MkTypeLine [] [Instant])
       [ Spell (Sequentially
                  [ Draw You (Lit 2)
                  , Draw (Macros.each (And [AnyPlayer, OtherThan You])) (Lit 1) ]) ]
       Nothing


public export
spitemare : Card
spitemare =
  Macros.card "Spitemare"
       (Just [Macros.generic 2, Macros.hybridPip Red White,
              Macros.hybridPip Red White]) []
       (MkTypeLine [creatureType "Elemental"] [Creature])
       [ Macros.triggered Whenever
                          (IsDealtDamage AnyDamage Macros.thisCreature)
                          (DealDamage It ThatMuch (Macros.target Macros.anyTarget)) ]
       (Just (3, 3))

public export
grollub : Card
grollub =
  Macros.card "Grollub" (Just [Macros.generic 2, Macros.pip Black]) []
       (MkTypeLine [creatureType "Beast"] [Creature])
       [ Macros.triggered Whenever
                          (IsDealtDamage AnyDamage Macros.thisCreature)
                          (Macros.gainsLife (Macros.each Opponent) ThatMuch) ]
       (Just (3, 3))

public export
moggManiac : Card
moggManiac =
  Macros.card "Mogg Maniac" (Just [Macros.generic 1, Macros.pip Red]) []
       (MkTypeLine [creatureType "Goblin"] [Creature])
       [ Macros.triggered Whenever
                          (IsDealtDamage AnyDamage Macros.thisCreature)
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
                          (IsDealtDamage AnyDamage (Macros.a Macros.creature))
                          (DealDamage Macros.thisEnchantment ThatMuch
                               (Macros.controllerOf (That (TypeW Creature)))) ]
       Nothing

public export
spitefulShadows : Card
spitefulShadows =
  Macros.card "Spiteful Shadows" (Just [Macros.generic 1, Macros.pip Black]) []
       (MkTypeLine [enchantmentType "Aura"] [Enchantment])
       [ Macros.keywordSubject "Enchant" Macros.creature
       , Macros.triggered Whenever
                          (IsDealtDamage AnyDamage (AttachHost Enchanted (TypeW Creature)))
                          (DealDamage It ThatMuch (Macros.controllerOf It)) ]
       Nothing

public export
bindingAgony : Card
bindingAgony =
  Macros.card "Binding Agony" (Just [Macros.generic 1, Macros.pip Black]) []
       (MkTypeLine [enchantmentType "Aura"] [Enchantment])
       [ Macros.keywordSubject "Enchant" Macros.creature
       , Macros.triggered Whenever
                          (IsDealtDamage AnyDamage (AttachHost Enchanted (TypeW Creature)))
                          (DealDamage Macros.thisAura ThatMuch
                               (Macros.controllerOf (That (TypeW Creature)))) ]
       Nothing

public export
darienKingOfKjeldor : Card
darienKingOfKjeldor =
  Macros.card "Darien, King of Kjeldor"
       (Just [Macros.generic 4, Macros.pip White, Macros.pip White]) [Legendary]
       (MkTypeLine [creatureType "Human", creatureType "Soldier"] [Creature])
       [ Macros.triggered Whenever
                          (IsDealtDamage AnyDamage You)
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
                          (Macros.dealsCombatDamage Macros.thisCreature (Macros.a AnyPlayer))
                          (Macros.create ThatMuch
                                  (Macros.creatureTok 1 1 [Green] [creatureType "Insect"])) ]
       (Just (6, 6))

public export
screamingNemesis : Card
screamingNemesis =
  Macros.card "Screaming Nemesis"
       (Just [Macros.generic 2, Macros.pip Red]) []
       (MkTypeLine [creatureType "Spirit"] [Creature])
       [ Macros.keyword "Haste"
       , Macros.triggered Whenever
                          (IsDealtDamage AnyDamage Macros.thisCreature)
                          (Sequentially
                             [ DealDamage It ThatMuch
                                 (Macros.target (And [Macros.anyTarget,
                                                      OtherThan This]))
                             , If (DealtThisWay AnyPlayer)
                                  (Continuously {ts = StaticFirstDone} (Macros.playerCant "GainLife" They)
                                                (Just RestOfGame))
                                  Nothing ]) ]
       (Just (3, 3))


public export
sonicShrieker : Card
sonicShrieker =
  Macros.card "Sonic Shrieker"
       (Just [Macros.generic 2, Macros.pip Red, Macros.pip White,
              Macros.pip Black]) []
       (MkTypeLine [creatureType "Dragon"] [Creature])
       [ Macros.keyword "Flying"
       , Macros.triggered When
                          (Enters Macros.thisCreature Nothing)
                          (Sequentially
                             [ DealDamage It (Lit 2)
                                 (Macros.target Macros.anyTarget)
                             , Macros.gainsLife You (Lit 2)
                             , If (DealtThisWay AnyPlayer)
                                  ((Macros.discard They (Macros.a (InZone Macros.handZ))))
                                  Nothing ]) ]
       (Just (4, 4))


public export
callInAProfessional : Card
callInAProfessional =
  Macros.card "Call In a Professional"
       (Just [Macros.generic 2, Macros.pip Red]) []
       (MkTypeLine [] [Instant])
       [ Spell (Sequentially
                  [ Continuously {ts = StaticFirstDone}
                      (Macros.playerCant "GainLife" (PlayerGroup AllPlayers))
                      (Just Macros.thisTurn)
                  , Continuously {ts = StaticFirstDone}
                      (CantPrevent AnyDamage (DamageDescribed Unattributed Everywhere) NoPreventionOnly)
                      (Just Macros.thisTurn)
                  , DealDamage This (Lit 3) (Macros.target Macros.anyTarget) ]) ]
       Nothing

public export
giantCindermaw : Card
giantCindermaw =
  Macros.card "Giant Cindermaw" (Just [Macros.generic 2, Macros.pip Red]) []
       (MkTypeLine [creatureType "Dinosaur", creatureType "Beast"] [Creature])
       [ Macros.keyword "Trample"
       , Static (Macros.playerCant "GainLife" (PlayerGroup AllPlayers)) ]
       (Just (4, 3))

public export
mindlockOrb : Card
mindlockOrb =
  Macros.card "Mindlock Orb" (Just [Macros.generic 3, Macros.pip Blue]) []
       (MkTypeLine [] [Artifact])
       [ Static (Macros.playerCant "Search" (PlayerGroup AllPlayers)) ]
       Nothing

public export
silence : Card
silence =
  Macros.card "Silence" (Just [Macros.pip White]) []
       (MkTypeLine [] [Instant])
       [ Spell (Continuously {ts = StaticFirstDone}
                  (Macros.playerCant "Cast" (PlayerGroup YourOpponents))
                  (Just Macros.thisTurn)) ]
       Nothing

public export
shadowOfDoubt : Card
shadowOfDoubt =
  Macros.card "Shadow of Doubt"
       (Just [Macros.hybridPip Blue Black, Macros.hybridPip Blue Black]) []
       (MkTypeLine [] [Instant])
       [ Spell (Sequentially
                  [ Continuously {ts = StaticFirstDone}
                      (Macros.playerCant "Search" (PlayerGroup AllPlayers))
                      (Just Macros.thisTurn)
                  , (Macros.draw You (Lit 1)) ]) ]
       Nothing

public export
aggressiveMining : Card
aggressiveMining =
  Macros.card "Aggressive Mining"
       (Just [Macros.generic 3, Macros.pip Red]) []
       (MkTypeLine [] [Enchantment])
       [ Static (Macros.playerCant "Play" You)
       , Macros.activatedOnlyOnce (Do (Macros.sacrifice You (Macros.a Macros.land)))
                                  (Draw You (Lit 2))
                                  OncePerTurn ]
       Nothing

public export
omenMachineDraw : Ability
omenMachineDraw = Static (Macros.playerCant "DrawCard" (PlayerGroup AllPlayers))

public export
grievousWoundLifeLock : Card
grievousWoundLifeLock =
  Macros.card "Grievous Wound"
       (Just [Macros.generic 3, Macros.pip Black, Macros.pip Black]) []
       (MkTypeLine [enchantmentType "Aura"] [Enchantment])
       [ Macros.keywordSubject "Enchant" AnyPlayer
       , Static (Macros.playerCant "GainLife" (AttachHost Enchanted PlayerW))
       , Macros.triggered Whenever
                          (IsDealtDamage AnyDamage (AttachHost Enchanted PlayerW))
                          (Macros.losesLife They (Half RoundUp (PlayerStatOf LifeTotal They))) ]
       Nothing

public export
solfataraLandLock : Effect []
solfataraLandLock =
  Continuously {ts = StaticFirstDone} (Macros.playerCant "Play" (Macros.target AnyPlayer))
               (Just Macros.thisTurn)


public export
bloodMoon : Card
bloodMoon =
  Macros.card "Blood Moon" (Just [Macros.generic 2, Macros.pip Red]) []
       (MkTypeLine [] [Enchantment])
       [ Static (Becomes (Macros.allOf (And [Macros.land, Not (HasSupertype Basic)])) Sets (Bundle (MkToken Nothing [] (Macros.basicLandLine [landType "Mountain"])
                                   [] Nothing) Nothing)) ]
       Nothing

public export
magusOfTheMoon : Card
magusOfTheMoon =
  Macros.card "Magus of the Moon" (Just [Macros.generic 2, Macros.pip Red]) []
       (MkTypeLine [creatureType "Human", creatureType "Wizard"] [Creature])
       [ Static (Becomes (Macros.allOf (And [Macros.land, Not (HasSupertype Basic)])) Sets (Bundle (MkToken Nothing [] (Macros.basicLandLine [landType "Mountain"])
                                   [] Nothing) Nothing)) ]
       (Just (2, 2))

public export
harbingerOfTheSeas : Card
harbingerOfTheSeas =
  Macros.card "Harbinger of the Seas"
       (Just [Macros.generic 1, Macros.pip Blue, Macros.pip Blue]) []
       (MkTypeLine [creatureType "Merfolk", creatureType "Wizard"] [Creature])
       [ Static (Becomes (Macros.allOf (And [Macros.land, Not (HasSupertype Basic)])) Sets (Bundle (MkToken Nothing [] (Macros.basicLandLine [landType "Island"])
                                   [] Nothing) Nothing)) ]
       (Just (2, 2))

public export
lushGrowth : Card
lushGrowth =
  Macros.card "Lush Growth" (Just [Macros.pip Green]) []
       (MkTypeLine [enchantmentType "Aura"] [Enchantment])
       [ Macros.keywordSubject "Enchant" Macros.land
       , Static (Becomes (AttachHost Enchanted (TypeW Land)) Sets (Bundle (MkToken Nothing []
                                   (Macros.basicLandLine [landType "Mountain", landType "Forest", landType "Plains"])
                                   [] Nothing) Nothing)) ]
       Nothing

public export
yavimayaCradleOfGrowth : Card
yavimayaCradleOfGrowth =
  Macros.card "Yavimaya, Cradle of Growth" Nothing [Legendary]
       (MkTypeLine [] [Land])
       [ Static (Becomes (Macros.allOf Macros.land) Adds (Bundle (MkToken Nothing [] (Macros.basicLandLine [landType "Forest"]) [] Nothing) Nothing)) ]
       Nothing

public export
reefShaman : Card
reefShaman =
  Macros.card "Reef Shaman" (Just [Macros.pip Blue]) []
       (MkTypeLine [creatureType "Merfolk", creatureType "Shaman"] [Creature])
       [ Macros.activated TapSymbol
                          (Continuously {ts = StaticFirstDone}
                             (Becomes (Macros.target Macros.land) Sets (ChosenQuality (OfYourChoice (SubtypeQ Land) (Just BasicTypesOnly))))
                             (Just Macros.untilEndOfTurn)) ]
       (Just (0, 2))

public export
grixisIllusionist : Card
grixisIllusionist =
  Macros.card "Grixis Illusionist" (Just [Macros.pip Blue]) []
       (MkTypeLine [creatureType "Human", creatureType "Wizard"] [Creature])
       [ Macros.activated TapSymbol
                          (Continuously {ts = StaticFirstDone}
                      (Becomes (Macros.target (And [Macros.land, HasPossessor ControllerAx You])) Sets (ChosenQuality (OfYourChoice (SubtypeQ Land) (Just BasicTypesOnly))))
                      (Just Macros.untilEndOfTurn)) ]
       (Just (1, 1))

public export
unstableFrontier : Card
unstableFrontier =
  Macros.card "Unstable Frontier" Nothing [] (MkTypeLine [] [Land])
       [ Macros.activated TapSymbol (AddMana You (Lit 1) (Runs [[Colorless]]) [])
       , Macros.activated TapSymbol
                          (Continuously {ts = StaticFirstDone}
                      (Becomes (Macros.target (And [Macros.land, HasPossessor ControllerAx You])) Sets (ChosenQuality (OfYourChoice (SubtypeQ Land) (Just BasicTypesOnly))))
                      (Just Macros.untilEndOfTurn)) ]
       Nothing


public export
extinction : Card
extinction =
  Macros.card "Extinction" (Just [Macros.generic 4, Macros.pip Black]) []
       (MkTypeLine [] [Sorcery])
       [ Spell (Macros.destroy (Macros.allOf (And [Macros.creature,
                                            OfYourChoice (SubtypeQ Creature) Nothing]))) ]
       Nothing

public export
defensiveManeuvers : Card
defensiveManeuvers =
  Macros.card "Defensive Maneuvers"
       (Just [Macros.generic 3, Macros.pip White]) []
       (MkTypeLine [] [Instant])
       [ Spell (Macros.gets (Macros.allOf (And [Macros.creature,
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
       [ Spell (Macros.gets (Macros.allOf (And [Macros.creature,
                                         OfYourChoice (SubtypeQ Creature) Nothing]))
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
                          (Macros.destroy (Macros.allOf (And [HasType Enchantment,
                                                OfYourChoice Color Nothing]))) ]
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
                      (Macros.allOf (And [Macros.creature,
                                   OfYourChoice (SubtypeQ Creature) Nothing]))) ]
       (Just (1, 3))

public export
distantMelody : Card
distantMelody =
  Macros.card "Distant Melody" (Just [Macros.generic 3, Macros.pip Blue]) []
       (MkTypeLine [] [Sorcery])
       [ Spell (Sequentially
                  [ Macros.choose (Macros.a (Macros.quality (SubtypeQ Creature)))
                  , Draw You (Macros.forEach
                                (And [Permanent, HasPossessor ControllerAx You,
                                      OfChosen (SubtypeQ Creature)])) ]) ]
       Nothing

public export
cripplingFear : Card
cripplingFear =
  Macros.card "Crippling Fear"
       (Just [Macros.generic 2, Macros.pip Black, Macros.pip Black]) []
       (MkTypeLine [] [Sorcery])
       [ Spell (Sequentially
                  [ Macros.choose (Macros.a (Macros.quality (SubtypeQ Creature)))
                  , Macros.gets (Macros.allOf (And [Macros.creature,
                                             Not (OfChosen (SubtypeQ Creature))]))
                                (PtDown (Lit 3)) (PtDown (Lit 3))
                                (Just Macros.untilEndOfTurn) ]) ]
       Nothing


public export
rallyTheRanks : Card
rallyTheRanks =
  Macros.card "Rally the Ranks" (Just [Macros.generic 1, Macros.pip White]) []
       (MkTypeLine [] [Enchantment])
       [ Static (Macros.entersChoosing Macros.thisEnchantment (SubtypeQ Creature))
       , Static (Gets (Macros.allOf (And [Macros.creature, HasPossessor ControllerAx You,
                                   OfChosen (SubtypeQ Creature)]))
                      (PtUp (Lit 1)) (PtUp (Lit 1))) ]
       Nothing

public export
sharedTriumph : Card
sharedTriumph =
  Macros.card "Shared Triumph" (Just [Macros.generic 1, Macros.pip White]) []
       (MkTypeLine [] [Enchantment])
       [ Static (Macros.entersChoosing Macros.thisEnchantment (SubtypeQ Creature))
       , Static (Gets (Macros.allOf (And [Macros.creature, OfChosen (SubtypeQ Creature)]))
                      (PtUp (Lit 1)) (PtUp (Lit 1))) ]
       Nothing

public export
hallOfTriumph : Card
hallOfTriumph =
  Macros.card "Hall of Triumph" (Just [Macros.generic 3]) [Legendary]
       (MkTypeLine [] [Artifact])
       [ Static (Macros.entersChoosing Macros.thisArtifact Color)
       , Static (Gets (Macros.allOf (And [Macros.creature, HasPossessor ControllerAx You,
                                   OfChosen Color]))
                      (PtUp (Lit 1)) (PtUp (Lit 1))) ]
       Nothing

public export
engineeredPlague : Card
engineeredPlague =
  Macros.card "Engineered Plague"
       (Just [Macros.generic 2, Macros.pip Black]) []
       (MkTypeLine [] [Enchantment])
       [ Static (Macros.entersChoosing Macros.thisEnchantment (SubtypeQ Creature))
       , Static (Gets (Macros.allOf (And [Macros.creature, OfChosen (SubtypeQ Creature)]))
                      (PtDown (Lit 1)) (PtDown (Lit 1))) ]
       Nothing

public export
prismRing : Card
prismRing =
  Macros.card "Prism Ring" (Just [Macros.generic 1]) []
       (MkTypeLine [] [Artifact])
       [ Static (Macros.entersChoosing Macros.thisArtifact Color)
       , Macros.triggered Whenever
                          (Casts You (Macros.a (And [Macros.spell, OfChosen Color])) Nothing)
                          (Macros.gainsLife You (Lit 1)) ]
       Nothing

public export
urzasIncubator : Card
urzasIncubator =
  Macros.card "Urza's Incubator" (Just [Macros.generic 3]) []
       (MkTypeLine [] [Artifact])
       [ Static (Macros.entersChoosing Macros.thisArtifact (SubtypeQ Creature))
       , Static (CostsToCast (Macros.allOf (And [Macros.creature, Macros.spell,
                                          OfChosen (SubtypeQ Creature)]))
                             (CostLess (Lit 2) Nothing)) ]
       Nothing

public export
etchingsOfTheChosen : Card
etchingsOfTheChosen =
  Macros.card "Etchings of the Chosen"
       (Just [Macros.generic 1, Macros.pip White, Macros.pip Black]) []
       (MkTypeLine [] [Enchantment])
       [ Static (Macros.entersChoosing Macros.thisEnchantment (SubtypeQ Creature))
       , Static (Gets (Macros.allOf (And [Macros.creature, HasPossessor ControllerAx You,
                                   OfChosen (SubtypeQ Creature)]))
                      (PtUp (Lit 1)) (PtUp (Lit 1)))
       , Macros.activated (Compound [Mana [Macros.generic 1],
                              Do (Macros.sacrifice You
                                    (Macros.a (And [Macros.creature,
                                                    OfChosen (SubtypeQ Creature)])))])
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
       , Static (Gains (Macros.allOf (HasSubtype (creatureType "Sliver")))
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
       , Static (Macros.entersWithCounters Macros.thisAura (Lit 1) (Named "Sleight"))
       , Static (Prevents AnyDamage (DealtBy (Macros.allOf (And [Macros.source, OfTheLastChosen Color])))
                          (Macros.shieldingIt (AttachHost Enchanted (TypeW Creature)))
                          CutAll Repeatedly
                          Nothing)
       , Macros.activated (Mana [Variable])
           (Sequentially
              [ PutCounters (Lit 1) (PrintedKind (Named "Sleight")) Macros.thisAura
              , Macros.choose (Macros.a (Macros.quality Color))
              , Define X (CountersOn (Named "Sleight") Macros.thisAura) ]) ]
       Nothing

||| Sanctuary Blade
public export
beckoningWillOWispChooser : Ability
beckoningWillOWispChooser =
  Macros.triggered At (BeginningOf Combat (Macros.yours))
                   (Macros.choose (Macros.a Opponent))

||| Koh, the Face Stealer
public export
kohChooser : Ability
kohChooser =
  Macros.activated (Macros.payLife You 1)
    (Macros.choose (Macros.a (And [Macros.creature, ExiledWith This])))

||| Volrath's Laboratory
public export
volrathsLaboratoryChoice : StaticEffect []
volrathsLaboratoryChoice =
  AndAlso [ Macros.entersChoosing Macros.thisArtifact Color
          , Macros.entersChoosing Macros.thisArtifact (SubtypeQ Creature) ]

||| Volrath's Laboratory
public export
volrathsLaboratory : Card
volrathsLaboratory =
  Macros.card "Volrath's Laboratory" (Just [Macros.generic 5]) []
       (MkTypeLine [] [Artifact])
       [ Static volrathsLaboratoryChoice
       , Macros.activated (Compound [Mana [Macros.generic 5], TapSymbol])
           (Macros.create (Lit 1)
              (MkTokenChars (Just (Lit 2 ** Lit 2)) [] []
                            (MkTypeLine [] [Creature]) [] Nothing
                            [ WithQuality (OfChosen Color)
                            , WithQuality (OfChosen (SubtypeQ Creature)) ])) ]
       Nothing

||| Call to Arms
public export
callToArmsChoice : StaticEffect []
callToArmsChoice =
  AndAlso [ Macros.entersChoosing Macros.thisEnchantment Color
          , Macros.entersChoosingPlayer Macros.thisEnchantment
                                        (Just OpponentsOnly) ]

||| Forgotten Lore
public export
forgottenLoreChoice : Effect []
forgottenLoreChoice =
  Macros.chooses (Macros.target Opponent)
                 (Macros.a (InZone (Macros.graveyardOf You)))

public export
sanctuaryBlade : Card
sanctuaryBlade =
  Macros.card "Sanctuary Blade" (Just [Macros.generic 2]) []
       (MkTypeLine [artifactType "Equipment"] [Artifact])
       [ Static (Macros.attachChoosing Macros.thisEquipment Color)
       , Static (AndAlso [ Gets (AttachHost Equipped (TypeW Creature))
                                (PtUp (Lit 2)) (PtUp (Lit 0))
                         , Gains (AttachHost Equipped (TypeW Creature))
                                 (Macros.keywordQuality "Protection"
                                    (OfTheLastChosen Color)) ])
       , Macros.keywordCosting "Equip" (Mana [Macros.generic 3]) ]
       Nothing

||| Psychic Paper minus its three-way coordination
public export
psychicPaperChoiceAndReads : AbilitySeq []
psychicPaperChoiceAndReads =
  [ Static (AndAlso [ Macros.attachChoosing Macros.thisEquipment CardName
                    , Macros.attachChoosing Macros.thisEquipment
                                            (SubtypeQ Creature) ])
  , Static (AndAlso
      [ Becomes (AttachHost Equipped (TypeW Creature)) Sets (ChosenQuality (OfTheLastChosen CardName))
      , Becomes (AttachHost Equipped (TypeW Creature)) Sets (ChosenQuality (OfTheLastChosen (SubtypeQ Creature))) ]) ]

public export
xenograft : Card
xenograft =
  Macros.card "Xenograft" (Just [Macros.generic 4, Macros.pip Blue]) []
       (MkTypeLine [] [Enchantment])
       [ Static (Macros.entersChoosing Macros.thisEnchantment (SubtypeQ Creature))
       , Static (Becomes (Macros.allOf (And [Macros.creature, HasPossessor ControllerAx You])) Adds (ChosenQuality (OfChosen (SubtypeQ Creature)))) ]
       Nothing

||| Convincing Mirage
public export
convincingMirage : Card
convincingMirage =
  Macros.card "Convincing Mirage" (Just [Macros.generic 1, Macros.pip Blue]) []
       (MkTypeLine [enchantmentType "Aura"] [Enchantment])
       [ Macros.keywordSubject "Enchant" Macros.land
       , Static (Macros.entersChoosingFrom Macros.thisAura (SubtypeQ Land)
                                           BasicTypesOnly)
       , Static (Becomes (AttachHost Enchanted (TypeW Land)) Sets (ChosenQuality (OfChosen (SubtypeQ Land)))) ]
       Nothing

||| Realmwright
public export
realmwright : Card
realmwright =
  Macros.card "Realmwright" (Just [Macros.pip Blue]) []
       (MkTypeLine [creatureType "Vedalken", creatureType "Wizard"] [Creature])
       [ Static (Macros.entersChoosingFrom Macros.thisCreature (SubtypeQ Land)
                                           BasicTypesOnly)
       , Static (Becomes (Macros.allOf (And [Macros.land, HasPossessor ControllerAx You])) Adds (ChosenQuality (OfChosen (SubtypeQ Land)))) ]
       (Just (1, 1))

public export
adaptiveAutomaton : Card
adaptiveAutomaton =
  Macros.card "Adaptive Automaton" (Just [Macros.generic 3]) []
       (MkTypeLine [creatureType "Construct"] [Artifact, Creature])
       [ Static (Macros.entersChoosing Macros.thisCreature (SubtypeQ Creature))
       , Static (Becomes Macros.thisCreature Adds (ChosenQuality (OfChosen (SubtypeQ Creature))))
       , Static (Gets (Macros.allOf (And [Macros.creature, HasPossessor ControllerAx You,
                                   OtherThan Macros.thisCreature,
                                   OfChosen (SubtypeQ Creature)]))
                      (PtUp (Lit 1)) (PtUp (Lit 1))) ]
       (Just (2, 2))

public export
mistformDreamer : Card
mistformDreamer =
  Macros.card "Mistform Dreamer" (Just [Macros.generic 2, Macros.pip Blue]) []
       (MkTypeLine [creatureType "Illusion"] [Creature])
       [ Macros.keyword "Flying"
       , Macros.activated (Mana [Macros.generic 1])
                          (Continuously {ts = StaticFirstDone}
                      (Becomes Macros.thisCreature Sets (ChosenQuality (OfYourChoice (SubtypeQ Creature) Nothing)))
                      (Just Macros.untilEndOfTurn)) ]
       (Just (2, 1))

public export
arcaneAdaptation : Card
arcaneAdaptation =
  Macros.card "Arcane Adaptation" (Just [Macros.generic 2, Macros.pip Blue]) []
       (MkTypeLine [] [Enchantment])
       [ Static (Macros.entersChoosing Macros.thisEnchantment (SubtypeQ Creature))
       , Static (AlsoOffBattlefield
                   (Becomes (Macros.allOf (And [Macros.creature, HasPossessor ControllerAx You])) Adds (ChosenQuality (OfChosen (SubtypeQ Creature))))) ]
       Nothing

public export
conspiracy : Card
conspiracy =
  Macros.card "Conspiracy"
       (Just [Macros.generic 3, Macros.pip Black, Macros.pip Black]) []
       (MkTypeLine [] [Enchantment])
       [ Static (Macros.entersChoosing Macros.thisEnchantment (SubtypeQ Creature))
       , Static (AlsoOffBattlefield
                   (Becomes (Macros.allOf (And [Macros.creature, HasPossessor ControllerAx You])) Sets (ChosenQuality (OfChosen (SubtypeQ Creature))))) ]
       Nothing

public export
encroachingMycosynth : Card
encroachingMycosynth =
  Macros.card "Encroaching Mycosynth" (Just [Macros.generic 3, Macros.pip Blue]) []
       (MkTypeLine [] [Artifact])
       [ Static (AlsoOffBattlefield
                   (Becomes (Macros.allOf (And [Permanent, Not (HasType Land),
                                             HasPossessor ControllerAx You])) Adds (Bundle (MkToken Nothing [] (MkTypeLine [] [Artifact]) [] Nothing) Nothing))) ]
       Nothing

||| Aisling Leprechaun
public export
aislingLeprechaun : Card
aislingLeprechaun =
  Macros.card "Aisling Leprechaun" (Just [Macros.pip Green]) []
       (MkTypeLine [creatureType "Faerie"] [Creature])
       [ Macros.triggeredOr Whenever
                            (Blocks Macros.thisCreature (Just (Macros.a Macros.creature)))
                            [BecomesBlocked Macros.thisCreature
                                            (Just (Macros.a Macros.creature))]
                            (Macros.becomesColor (That (TypeW Creature)) (SomeColors [Green]) Nothing) ]
       (Just (1, 1))

||| Darkest Hour
public export
darkestHour : Card
darkestHour =
  Macros.card "Darkest Hour" (Just [Macros.pip Black]) []
       (MkTypeLine [] [Enchantment])
       [ Static (Becomes (Macros.allOf Macros.creature) Sets (Colored (SomeColors [Black]))) ]
       Nothing

||| Thran Lens
public export
thranLens : Card
thranLens =
  Macros.card "Thran Lens" (Just [Macros.generic 2]) []
       (MkTypeLine [] [Artifact])
       [ Static (Becomes (Macros.allOf Permanent) Sets (Colored (SomeColors []))) ]
       Nothing

||| Ghostflame Sliver
public export
ghostflameSliver : Card
ghostflameSliver =
  Macros.card "Ghostflame Sliver" (Just [Macros.pip Black, Macros.pip Red]) []
       (MkTypeLine [creatureType "Sliver"] [Creature])
       [ Static (Becomes (Macros.allOf (HasSubtype (creatureType "Sliver"))) Sets (Colored (SomeColors []))) ]
       (Just (2, 2))

||| Sinister Strength
public export
sinisterStrength : Card
sinisterStrength =
  Macros.card "Sinister Strength" (Just [Macros.generic 1, Macros.pip Black]) []
       (MkTypeLine [enchantmentType "Aura"] [Enchantment])
       [ Macros.keywordSubject "Enchant" Macros.creature
       , Static (AndAlso [ Gets (AttachHost Enchanted (TypeW Creature))
                                (PtUp (Lit 3)) (PtUp (Lit 1))
                         , Becomes It Sets (Colored (SomeColors [Black])) ]) ]
       Nothing

||| Crimson Wisps
public export
crimsonWisps : Card
crimsonWisps =
  Macros.card "Crimson Wisps" (Just [Macros.pip Red]) []
       (MkTypeLine [] [Instant])
       [ Spell (Sequentially
                  [ Continuously {ts = StaticFirstDone}
                      (AndAlso [ Becomes (Macros.target Macros.creature) Sets (Colored (SomeColors [Red]))
                               , Gains It (Macros.keyword "Haste") ])
                      (Just Macros.untilEndOfTurn)
                  , (Macros.draw You (Lit 1)) ]) ]
       Nothing

||| Nightcreep
public export
nightcreep : Card
nightcreep =
  Macros.card "Nightcreep" (Just [Macros.pip Black, Macros.pip Black]) []
       (MkTypeLine [] [Instant])
       [ Spell (Continuously {ts = StaticFirstDone}
                  (AndAlso [ Becomes (Macros.allOf Macros.creature) Sets (Colored (SomeColors [Black]))
                           , Becomes (Macros.allOf Macros.land) Sets (Bundle (MkToken Nothing []
                                               (Macros.basicLandLine [landType "Swamp"])
                                               [] Nothing) Nothing) ])
                  (Just Macros.untilEndOfTurn)) ]
       Nothing

||| Celestial Dawn
public export
celestialDawnAscriptions : AbilitySeq []
celestialDawnAscriptions =
  [ Static (Becomes (Macros.allOf (And [Macros.land, HasPossessor ControllerAx You])) Sets (Bundle (MkToken Nothing [] (Macros.basicLandLine [landType "Plains"])
                              [] Nothing) Nothing))
  , Static (AlsoOffBattlefield
              (Becomes (Macros.allOf (And [Permanent, Not (HasType Land), HasPossessor ControllerAx You])) Sets (Colored (SomeColors [White])))) ]

||| Ghoulflesh
public export
ghoulflesh : Card
ghoulflesh =
  Macros.card "Ghoulflesh" (Just [Macros.pip Black]) []
       (MkTypeLine [enchantmentType "Aura"] [Enchantment])
       [ Macros.keywordSubject "Enchant" Macros.creature
       , Static (AndAlso [ Gets (AttachHost Enchanted (TypeW Creature))
                                (PtDown (Lit 1)) (PtDown (Lit 1))
                         , Becomes It Adds (Bundle (MkToken Nothing [Black]
                                                (MkTypeLine [creatureType "Zombie"] [])
                                                [] Nothing) Nothing) ]) ]
       Nothing

||| Blade of the Oni
public export
bladeOfTheOniStatic : StaticEffect []
bladeOfTheOniStatic =
  AndAlso [ HasBasePt (AttachHost Equipped (TypeW Creature)) (Lit 5) (Lit 5)
          , Gains It (Macros.keyword "Menace")
          , Becomes It Adds (Bundle (MkToken Nothing [Black]
                                 (MkTypeLine [creatureType "Demon"] []) [] Nothing) Nothing) ]

||| Ersatz Gnomes
public export
ersatzGnomes : Card
ersatzGnomes =
  Macros.card "Ersatz Gnomes" (Just [Macros.generic 3]) []
       (MkTypeLine [creatureType "Gnome"] [Artifact, Creature])
       [ Macros.activated TapSymbol
           (Continuously {ts = StaticFirstDone} (Becomes (Macros.target Macros.spell) Sets (Colored (SomeColors [])))
                         Nothing)
       , Macros.activated TapSymbol
           (Macros.becomesColor (Macros.target Permanent) (SomeColors [])
                                (Just Macros.untilEndOfTurn)) ]
       (Just (1, 1))

||| Missy
public export
missyFaceDownReturn : Ability
missyFaceDownReturn =
  Macros.triggered Whenever
    (Dies (Macros.a (And [Macros.creature, Not (HasType Artifact),
                          OtherThan Macros.thisCreature])))
    (Sequentially
       [ Move It Macros.battlefieldZ
              [EntersAs FaceDown, EntersTapped, Under You]
       , Continuously {ts = StaticFirstDone}
           (Becomes It Sets (Bundle (MkToken (Just (Lit 2 ** Lit 2)) []
                              (MkTypeLine [creatureType "Cyberman"] [Artifact, Creature])
                              [] Nothing) Nothing))
           Nothing ])

||| Transguild Courier
public export
transguildCourier : Card
transguildCourier =
  Macros.card "Transguild Courier" (Just [Macros.generic 4]) []
       (MkTypeLine [creatureType "Golem"] [Artifact, Creature])
       [ Static (Becomes Macros.thisCreature Sets (Colored EveryColor)) ]
       (Just (3, 3))

||| Scrapbasket
public export
scrapbasket : Card
scrapbasket =
  Macros.card "Scrapbasket" (Just [Macros.generic 4]) []
       (MkTypeLine [creatureType "Scarecrow"] [Artifact, Creature])
       [ Macros.activated (Mana [Macros.generic 1])
                          (Macros.becomesColor Macros.thisCreature EveryColor
                                               (Just Macros.untilEndOfTurn)) ]
       (Just (3, 2))

||| Indigo Faerie
public export
indigoFaerie : Card
indigoFaerie =
  Macros.card "Indigo Faerie" (Just [Macros.generic 1, Macros.pip Blue]) []
       (MkTypeLine [creatureType "Faerie", creatureType "Wizard"] [Creature])
       [ Macros.keyword "Flying"
       , Macros.activated (Mana [Macros.pip Blue])
                          (Continuously {ts = StaticFirstDone} (Becomes (Macros.target Permanent) Adds
                                                 (Colored (SomeColors [Blue])))
                                        (Just Macros.untilEndOfTurn)) ]
       (Just (1, 1))

||| Arcum's Weathervane
public export
arcumsWeathervane : Card
arcumsWeathervane =
  Macros.card "Arcum's Weathervane" (Just [Macros.generic 2]) [] (MkTypeLine [] [Artifact])
       [ Macros.activated (Compound [Mana [Macros.generic 2], TapSymbol])
           (Continuously {ts = StaticFirstDone}
              (Becomes (Macros.target (And [Macros.land, HasSupertype Snow])) Loses
                       (Bundle (MkTokenChars Nothing [] [Snow] (MkTypeLine [] []) [] Nothing [])
                               Nothing))
              Nothing)
       , Macros.activated (Compound [Mana [Macros.generic 2], TapSymbol])
           (Continuously {ts = StaticFirstDone}
              (Becomes (Macros.target (And [Macros.land, HasSupertype Basic,
                                            Not (HasSupertype Snow)])) Adds
                       (Bundle (MkTokenChars Nothing [] [Snow] (MkTypeLine [] []) [] Nothing [])
                               Nothing))
              Nothing) ]
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
              [ Macros.choose (Macros.a (Macros.qualityFrom (SubtypeQ Creature) (TypeOtherThan (creatureType "Wall"))))
              , Continuously {ts = StaticFirstDone} (Becomes (Macros.target Macros.creature) Sets (ChosenQuality (OfChosen (SubtypeQ Creature))))
                             (Just Macros.untilEndOfTurn) ]) ]
       (Just (1, 1))

public export
unnaturalSelection : Card
unnaturalSelection =
  Macros.card "Unnatural Selection" (Just [Macros.generic 1, Macros.pip Blue]) []
       (MkTypeLine [] [Enchantment])
       [ Macros.activated (Mana [Macros.generic 1])
           (Sequentially
              [ Macros.choose (Macros.a (Macros.qualityFrom (SubtypeQ Creature) (TypeOtherThan (creatureType "Wall"))))
              , Continuously {ts = StaticFirstDone} (Becomes (Macros.target Macros.creature) Sets (ChosenQuality (OfChosen (SubtypeQ Creature))))
                             (Just Macros.untilEndOfTurn) ]) ]
       Nothing

public export
standardize : Card
standardize =
  Macros.card "Standardize" (Just [Macros.pip Blue, Macros.pip Blue]) []
       (MkTypeLine [] [Instant])
       [ Spell (Sequentially
              [ Macros.choose (Macros.a (Macros.qualityFrom (SubtypeQ Creature) (TypeOtherThan (creatureType "Wall"))))
              , Continuously {ts = StaticFirstDone} (Becomes (Macros.each Macros.creature) Sets (ChosenQuality (OfChosen (SubtypeQ Creature))))
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
                  (Macros.a (And [Macros.spell, Named ChosenName])) Nothing)
           (Sequentially [ Macros.losesLife They (Lit 3), (Macros.draw You (Lit 1)) ]) ]
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
                          (Just (Macros.draw You (Lit 1))) ]) ]
       Nothing

public export
avenShrine : Card
avenShrine =
  Macros.card "Aven Shrine"
       (Just [Macros.generic 1, Macros.pip White, Macros.pip White]) []
       (MkTypeLine [] [Enchantment])
       [ Macros.triggered Whenever
           (Casts (Macros.a AnyPlayer) (Macros.a Macros.spell) Nothing)
           (Sequentially
              [ Macros.gainsLife They (LetterVal X)
              , Define X (Macros.countOf (And [InZone Macros.graveyardZ,
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
       , Static (Macros.objectCant "Cast"
                   (Macros.allOf (And [Macros.spell, Named ChosenName]))) ]
       (Just (2, 2))

||| Sanctum Prelate
public export
sanctumPrelate : Card
sanctumPrelate =
  Macros.card "Sanctum Prelate"
       (Just [Macros.generic 1, Macros.pip White, Macros.pip White]) []
       (MkTypeLine [creatureType "Human", creatureType "Cleric"] [Creature])
       [ Static (Macros.entersChoosing Macros.thisCreature Number)
       , Static (Macros.objectCant "Cast"
                   (Macros.allOf (And [Macros.spell, Not (HasType Creature),
                                Compare [CharAxis ManaValue] Eq ChosenNumber]))) ]
       (Just (2, 2))

||| Nyxathid
public export
nyxathid : Card
nyxathid =
  Macros.card "Nyxathid"
       (Just [Macros.generic 1, Macros.pip Black, Macros.pip Black]) []
       (MkTypeLine [creatureType "Elemental"] [Creature])
       [ Static (Macros.entersChoosingPlayer Macros.thisCreature
                                             (Just OpponentsOnly))
       , Static (Gets Macros.thisCreature
                      (PtDown (Macros.countOf (InZone (Macros.handOf
                                 (Macros.the ChosenPlayer)))))
                      (PtDown (Macros.countOf (InZone (Macros.handOf
                                 (Macros.the ChosenPlayer)))))) ]
       (Just (7, 7))

||| Stuffy Doll
public export
stuffyDoll : Card
stuffyDoll =
  Macros.card "Stuffy Doll" (Just [Macros.generic 5]) []
       (MkTypeLine [creatureType "Construct"] [Artifact, Creature])
       [ Macros.keyword "Indestructible"
       , Static (Macros.entersChoosingPlayer Macros.thisCreature Nothing)
       , Macros.triggered Whenever
                          (IsDealtDamage AnyDamage Macros.thisCreature)
                          (DealDamage It ThatMuch (Macros.the ChosenPlayer))
       , Macros.activated TapSymbol
                          (DealDamage Macros.thisCreature (Lit 1)
                                      Macros.thisCreature) ]
       (Just (0, 1))

||| Expel the Interlopers
public export
expelTheInterlopers : Card
expelTheInterlopers =
  Macros.card "Expel the Interlopers"
       (Just [Macros.generic 3, Macros.pip White, Macros.pip White]) []
       (MkTypeLine [] [Sorcery])
       [ Spell (Sequentially
           [ Macros.choose (Macros.a (Macros.qualityFrom Number
                                        (NumberBetween 0 10)))
           , Macros.destroy (Macros.allOf (And [Macros.creature,
                                         Compare [CharAxis Power] AtLeast ChosenNumber])) ]) ]
       Nothing

public export
nevermore : Card
nevermore =
  Macros.card "Nevermore"
       (Just [Macros.generic 1, Macros.pip White, Macros.pip White]) []
       (MkTypeLine [] [Enchantment])
       [ Static (Macros.entersChoosingFrom Macros.thisEnchantment
                                           CardName
                                           (NameOfCard (Not (HasType Land))))
       , Static (Macros.objectCant "Cast"
                   (Macros.allOf (And [Macros.spell, Named ChosenName]))) ]
       Nothing

||| Council of the Absolute
public export
councilOfTheAbsolute : Card
councilOfTheAbsolute =
  Macros.card "Council of the Absolute"
       (Just [Macros.generic 2, Macros.pip White, Macros.pip Blue]) []
       (MkTypeLine [creatureType "Human", creatureType "Advisor"] [Creature])
       [ Static (Macros.entersChoosingFrom Macros.thisCreature
                                           CardName
                                           (NameOfCard (Not (Or [HasType Creature,
                                                                 HasType Land]))))
       , Static (Macros.cantDoTo "Cast" (PlayerGroup YourOpponents)
                   (Macros.allOf (And [Macros.spell, Named ChosenName])))
       , Static (CostsToCast (Macros.allOf (And [Macros.spell, Named ChosenName,
                                          CastBy You]))
                             (CostLess (Lit 2) Nothing)) ]
       (Just (2, 3))

||| Failure // Comply
public export
complyNameLock : Effect []
complyNameLock =
  Sequentially
    [ Macros.choose (Macros.a (Macros.quality CardName))
    , Continuously {ts = StaticFirstDone}
        (Macros.cantDoTo "Cast" (PlayerGroup YourOpponents)
           (Macros.allOf (And [Macros.spell, Named ChosenName])))
        (Just Macros.untilYourNextTurn) ]

||| Gideon's Intervention
public export
gideonsIntervention : Card
gideonsIntervention =
  Macros.card "Gideon's Intervention"
       (Just [Macros.generic 2, Macros.pip White, Macros.pip White]) []
       (MkTypeLine [] [Enchantment])
       [ Static (Macros.entersChoosing Macros.thisEnchantment CardName)
       , Static (Macros.cantDoTo "Cast" (PlayerGroup YourOpponents)
                   (Macros.allOf (And [Macros.spell, Named ChosenName])))
       , Static (Prevents AnyDamage (DealtBy (Macros.allOf (And [IsSource, Named ChosenName])))
                   (ToRecipient (Macros.youAnd
                      (Macros.allOf (And [Permanent, HasPossessor ControllerAx You]))))
                   CutAll Repeatedly
                   Nothing) ]
       Nothing

||| Academic Probation
public export
academicProbationNameMode : Effect []
academicProbationNameMode =
  Sequentially
    [ Macros.choose (Macros.a (Macros.qualityFrom CardName
                                 (NameOfCard (Not (HasType Land)))))
    , Continuously {ts = StaticFirstDone}
        (Macros.cantDoTo "Cast" (PlayerGroup YourOpponents)
           (Macros.allOf (And [Macros.spell, Named ChosenName])))
        (Just Macros.untilYourNextTurn) ]

public export
necromentiaChoice : Effect []
necromentiaChoice =
  Macros.choose (Macros.a (Macros.qualityFrom CardName
                   (NameOfCard (Not (And [HasSupertype Basic, HasType Land])))))

||| Booby Trap
public export
boobyTrapNameChoice : StaticEffect []
boobyTrapNameChoice =
  Macros.entersChoosingFrom Macros.thisArtifact
                            CardName
                            (NameOfCard (Not (And [HasSupertype Basic,
                                                   HasType Land])))

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
                     (Macros.counted Macros.anyNumber
                                   (And [Permanent,
                                         InZone (Macros.graveyardOf You)])))
                  Macros.battlefieldZ) ]
       Nothing

public export
sphinxOfTheChimes : Card
sphinxOfTheChimes =
  Macros.card "Sphinx of the Chimes"
       (Just [Macros.generic 4, Macros.pip Blue, Macros.pip Blue]) []
       (MkTypeLine [creatureType "Sphinx"] [Creature])
       [ Macros.keyword "Flying"
       , Macros.activated
           (Do (Macros.discard You
                  (Macros.withTheSameName
                     (Macros.counted (Macros.exactly 2)
                                   (And [Not Macros.land,
                                         InZone Macros.handZ])))))
           ((Macros.draw You (Lit 4))) ]
       (Just (5, 6))

public export
chromeReplicator : Card
chromeReplicator =
  Macros.card "Chrome Replicator" (Just [Macros.generic 5]) []
       (MkTypeLine [creatureType "Construct"] [Artifact, Creature])
       [ Macros.triggeredIf When (Enters Macros.thisCreature Nothing)
           (Exists (Macros.withTheSameName
                           (Macros.counted (Macros.atLeast 2)
                                         (And [Permanent, Not Macros.land,
                                               Not IsToken, HasPossessor ControllerAx You]))))
           (Macros.create (Lit 1)
              (MkToken (Just (Lit 4 ** Lit 4)) []
                       (MkTypeLine [creatureType "Construct"] [Artifact, Creature])
                       [] Nothing)) ]
       (Just (4, 4))

public export
endlessAtlas : Card
endlessAtlas =
  Macros.card "Endless Atlas" (Just [Macros.generic 2]) []
       (MkTypeLine [] [Artifact])
       [ Macros.activatedOnlyIf (Compound [Mana [Macros.generic 2], TapSymbol])
                                (Macros.draw You (Lit 1))
                                (Exists (Macros.withTheSameName
                                   (Macros.counted (Macros.atLeast 3)
                                                 (And [Macros.land,
                                                       HasPossessor ControllerAx You])))) ]
       Nothing

public export
saheeliRaiPlusOne : Effect []
saheeliRaiPlusOne =
  Sequentially [ Macros.scry You (Lit 1)
               , DealDamage This (Lit 1) (Macros.each Opponent) ]

public export
abruptDecay : Card
abruptDecay =
  Macros.card "Abrupt Decay" (Just [Macros.pip Black, Macros.pip Green]) []
       (MkTypeLine [] [Instant])
       [ Static (Macros.objectCant "Counter" This)
       , Spell (Macros.destroy
                  (Macros.target (And [Permanent, Not (HasType Land),
                                       Compare [CharAxis ManaValue] AtMost (Lit 3)]))) ]
       Nothing

public export
dovinsVeto : Card
dovinsVeto =
  Macros.card "Dovin's Veto" (Just [Macros.pip White, Macros.pip Blue]) []
       (MkTypeLine [] [Instant])
       [ Static (Macros.objectCant "Counter" This)
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
           , Continuously {ts = StaticFirstDone}
               (AndAlso [ Macros.objectCant "Cast"
                            (Macros.allOf (And [Macros.spell, Named ChosenName]))
                        , Macros.objectCant "Play"
                            (Macros.allOf (And [Macros.land, Named ChosenName])) ])
               (Just Macros.untilYourNextTurn) ])
       , Spell (Macros.draw You (Lit 1)) ]
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
           (Macros.gets (Macros.allOf (And [HasSubtype (creatureType "Knight"), HasPossessor ControllerAx You]))
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
           (Macros.exile You (Macros.allOf (And [Macros.creature,
                                      Compare [CharAxis Power] AtLeast (Lit 4)])))
       , Macros.triggered When (ChapterMark [ChapterII, ChapterIII])
           (PutCounters (Lit 1) (PrintedKind Macros.plusOnePlusOne)
                        (Macros.each Macros.creatureYouControl)) ]
       Nothing

public export
keldonWarcaller : Card
keldonWarcaller =
  Macros.card "Keldon Warcaller"
       (Just [Macros.generic 1, Macros.pip Red]) []
       (MkTypeLine [creatureType "Human", creatureType "Warrior"] [Creature])
       [ Macros.triggered Whenever (Macros.attacks Macros.thisCreature)
           (PutCounters (Lit 1) (PrintedKind (Named "Lore"))
                        (Macros.target (And [HasSubtype (enchantmentType "Saga"), HasPossessor ControllerAx You]))) ]
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
           ((May You ((Macros.discard You (Macros.a (InZone Macros.handZ)))) (Just (Macros.draw You (Lit 1))) Nothing))
       , Macros.triggered When (ChapterMark [ChapterIII])
           (Macros.gets (Macros.allOf Macros.creatureYouControl)
                        (PtUp (Lit 1)) (PtUp (Lit 0))
                        (Just Macros.untilEndOfTurn)) ]
       Nothing

||| Burn, Burn, Tree and Fern
public export
burnBurnTreeAndFern : Card
burnBurnTreeAndFern =
  Macros.card "Burn, Burn, Tree and Fern"
       (Just [Macros.generic 3, Macros.pip Red]) []
       (MkTypeLine [enchantmentType "Saga"] [Enchantment])
       [ Macros.triggered When (ChapterMark [ChapterI])
           (DealDamage This (Lit 6)
              (Macros.target (And [Macros.creature,
                                   HasPossessor ControllerAx (Macros.a Opponent)])))
       , Macros.triggered When (ChapterMark [ChapterII])
           (Macros.destroy (Macros.target (And [Macros.artifact,
                                                HasPossessor ControllerAx (Macros.a Opponent)])))
       , Macros.triggered When (ChapterMark [ChapterIII, ChapterIV])
           (AddMana You (Lit 1) (Runs [[OfColor Red]]) []) ]
       Nothing

public export
theFlux : Card
theFlux =
  Macros.card "The Flux"
       (Just [Macros.generic 2, Macros.pip Red, Macros.pip Red]) []
       (MkTypeLine [enchantmentType "Saga"] [Enchantment])
       [ Macros.triggered When (ChapterMark [ChapterI])
           (DealDamage This (Lit 4)
              (Macros.target (And [Macros.creature,
                                   HasPossessor ControllerAx (Macros.a Opponent)])))
       , Macros.triggered When
           (ChapterMark [ChapterII, ChapterIII, ChapterIV, ChapterV])
           (Sequentially
              [ Macros.exile You Macros.topCard
              , Continuously {ts = StaticFirstDone} ((Macros.mayPlayDeed "Play" You (That CardW) (PlayRider Nothing Nothing Nothing False ItsOwnCost)))
                             (Just Macros.thisTurn) ])
       , Macros.triggered When (ChapterMark [ChapterVI])
           (AddMana You (Lit 6) (Runs [[OfColor Red]]) []) ]
       Nothing

||| Akroan Sergeant
public export
akroanSergeant : Card
akroanSergeant =
  Macros.card "Akroan Sergeant" (Just [Macros.generic 2, Macros.pip Red]) []
       (MkTypeLine [creatureType "Human", creatureType "Soldier"] [Creature])
       [ Macros.keyword "FirstStrike"
       , Macros.renown 1 ]
       (Just (2, 2))

public export
storiedEnduringStory : Effect []
storiedEnduringStory =
  If (AndCond
        [ CompareAmt (Macros.countOf (And [Permanent,
                                    Or [Macros.artifact,
                                        HasSubtype (enchantmentType "Saga"),
                                        HasSupertype Legendary],
                                    HasPossessor ControllerAx You]))
                     AtLeast (Lit 3)
        , Macros.notSo (Matches You (HasDesignation EnduringStory)) ])
     Macros.getsEnduringStory
     Nothing

public export
vedalkenOrrery : Card
vedalkenOrrery =
  Macros.card "Vedalken Orrery" (Just [Macros.generic 4]) []
       (MkTypeLine [] [Artifact])
       [ Static ((Deontic You Permit ["Cast"] Agent Nothing (DeonticCounterpart (Macros.allOf Macros.spell)) (Just (AsThoughOf (HasKeyword (TheKeyword "Flash")))) (PlayRider Nothing Nothing Nothing False ItsOwnCost))) ]
       Nothing

public export
shimmerMyr : Card
shimmerMyr =
  Macros.card "Shimmer Myr" (Just [Macros.generic 3]) []
       (MkTypeLine [creatureType "Myr"] [Artifact, Creature])
       [ Macros.keyword "Flash"
       , Static ((Deontic You Permit ["Cast"] Agent Nothing (DeonticCounterpart (Macros.allOf (And [Macros.spell, HasType Artifact]))) (Just (AsThoughOf (HasKeyword (TheKeyword "Flash")))) (PlayRider Nothing Nothing Nothing False ItsOwnCost))) ]
       (Just (2, 2))

public export
quickSliver : Card
quickSliver =
  Macros.card "Quick Sliver" (Just [Macros.generic 1, Macros.pip Green]) []
       (MkTypeLine [creatureType "Sliver"] [Creature])
       [ Macros.keyword "Flash"
       , Static ((Deontic (Macros.a AnyPlayer) Permit ["Cast"] Agent Nothing (DeonticCounterpart (Macros.allOf (And [Macros.spell, HasSubtype (creatureType "Sliver")]))) (Just (AsThoughOf (HasKeyword (TheKeyword "Flash")))) (PlayRider Nothing Nothing Nothing False ItsOwnCost))) ]
       (Just (1, 1))

public export
vernalEquinox : Card
vernalEquinox =
  Macros.card "Vernal Equinox"
       (Just [Macros.generic 3, Macros.pip Green]) []
       (MkTypeLine [] [Enchantment])
       [ Static ((Deontic (Macros.a AnyPlayer) Permit ["Cast"] Agent Nothing (DeonticCounterpart (Macros.allOf (And [Macros.spell,
                                                     Or [HasType Creature, HasType Enchantment]]))) (Just (AsThoughOf (HasKeyword (TheKeyword "Flash")))) (PlayRider Nothing Nothing Nothing False ItsOwnCost))) ]
       Nothing

public export
borneUponAWind : Card
borneUponAWind =
  Macros.card "Borne Upon a Wind" (Just [Macros.generic 1, Macros.pip Blue]) []
       (MkTypeLine [] [Instant])
       [ Spell (Continuously {ts = StaticFirstDone}
                  ((Deontic You Permit ["Cast"] Agent Nothing (DeonticCounterpart (Macros.allOf Macros.spell)) (Just (AsThoughOf (HasKeyword (TheKeyword "Flash")))) (PlayRider Nothing Nothing Nothing False ItsOwnCost)))
                  (Just Macros.thisTurn))
       , Spell (Macros.draw You (Lit 1)) ]
       Nothing

public export
gisaAndGeralf : Card
gisaAndGeralf =
  Macros.card "Gisa and Geralf"
       (Just [Macros.generic 2, Macros.pip Blue, Macros.pip Black]) [Legendary]
       (MkTypeLine [creatureType "Human", creatureType "Wizard"] [Creature])
       [ Macros.triggered When (Enters Macros.thisCreature Nothing)
           (Macros.mills You (Lit 4) You)
       , Static ((Macros.mayPlayDeed "Cast" You (Macros.a (And [Macros.spell, HasType Creature,
                                                           HasSubtype (creatureType "Zombie")])) (PlayRider (Macros.fromZ (Macros.graveyardOf You)) Macros.onceEachYourTurn Nothing False ItsOwnCost))) ]
       (Just (4, 4))

public export
castCreatureSpellsFromTop : StaticEffect []
castCreatureSpellsFromTop =
  (Macros.mayPlayDeed "Cast" You (Macros.allOf (And [Macros.spell, HasType Creature])) (PlayRider (Macros.fromZ Macros.onTopZ) Nothing Nothing False ItsOwnCost))

public export
playLandsAndCastSpellsFromTop : StaticEffect []
playLandsAndCastSpellsFromTop =
  AndAlso [ (Macros.mayPlayDeed "Play" You (Macros.allOf Macros.land) (PlayRider (Macros.fromZ Macros.onTopZ) Nothing Nothing False ItsOwnCost))
          , (Macros.mayPlayDeed "Cast" You (Macros.allOf Macros.spell) (PlayRider (Macros.fromZ Macros.onTopZ) Nothing Nothing False ItsOwnCost)) ]

public export
playAndCastFromGraveyardThisTurn : Effect []
playAndCastFromGraveyardThisTurn =
  Continuously {ts = StaticFirstDone}
    (AndAlso [ (Macros.mayPlayDeed "Play" You (Macros.allOf Macros.land) (PlayRider (Macros.fromZ (Macros.graveyardOf You)) Nothing Nothing False ItsOwnCost))
             , (Macros.mayPlayDeed "Cast" You (Macros.allOf Macros.spell) (PlayRider (Macros.fromZ (Macros.graveyardOf You)) Nothing Nothing False ItsOwnCost)) ])
    (Just Macros.untilEndOfTurn)

public export
castSmallCreatureFromTopOnceEachTurn : StaticEffect []
castSmallCreatureFromTopOnceEachTurn =
  (Macros.mayPlayDeed "Cast" You (Macros.a (And [Macros.spell, HasType Creature,
                                            Compare [CharAxis Power] AtMost (Lit 2)])) (PlayRider (Macros.fromZ Macros.onTopZ) (Just OnceEachTurn) Nothing False ItsOwnCost))

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
       , Static ((Macros.mayPlayDeed "Play" You (Macros.allOf Macros.land) (PlayRider (Macros.fromZ Macros.onTopZ) Nothing Nothing False ItsOwnCost)))
       , Macros.triggered Whenever
           (Enters (Macros.a (And [Macros.land, HasPossessor ControllerAx You])) Nothing)
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
       , Static ((Macros.mayPlayDeed "Cast" You (Macros.allOf (And [Macros.spell, HasSubtype (creatureType "Dragon")])) (PlayRider (Macros.fromZ Macros.onTopZ) Nothing Nothing False ItsOwnCost))) ]
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
       , Static ((Macros.mayPlayDeed "Cast" You (Macros.allOf (And [Macros.spell,
                                                 Macros.instantOrSorcery])) (PlayRider (Macros.fromZ Macros.onTopZ) Nothing Nothing False ItsOwnCost)))
       , Macros.activated (Mana [Macros.generic 3])
                          (Macros.exile You (LibrarySlice OnTop (Lit 1) You)) ]
       Nothing

public export
mysticForge : Card
mysticForge =
  Macros.card "Mystic Forge" (Just [Macros.generic 4]) []
       (MkTypeLine [] [Artifact])
       [ Static (Visibility LookAt You TopOfLibrary)
       , Static (AndAlso
           [ (Macros.mayPlayDeed "Cast" You (Macros.allOf (And [Macros.spell, Macros.artifact])) (PlayRider (Macros.fromZ Macros.onTopZ) Nothing Nothing False ItsOwnCost))
           , (Macros.mayPlayDeed "Cast" You (Macros.allOf (And [Macros.spell, IsColorless])) (PlayRider (Macros.fromZ Macros.onTopZ) Nothing Nothing False ItsOwnCost)) ])
       , Macros.activated (Compound [TapSymbol, Do (Macros.losesLife You (Lit 1))])
                          (Macros.exile You (LibrarySlice OnTop (Lit 1) You)) ]
       Nothing

public export
exploration : Card
exploration =
  Macros.card "Exploration" (Just [Macros.pip Green]) []
       (MkTypeLine [] [Enchantment])
       [ Static (Macros.mayPlayAdditionalLands You (Macros.exactly 1)) ]
       Nothing

public export
oracleOfMulDaya : Card
oracleOfMulDaya =
  Macros.card "Oracle of Mul Daya"
       (Just [Macros.generic 3, Macros.pip Green]) []
       (MkTypeLine [creatureType "Elf", creatureType "Shaman"] [Creature])
       [ Static (Macros.mayPlayAdditionalLands You (Macros.exactly 1))
       , Static (Visibility Reveal You TopOfLibrary)
       , Static ((Macros.mayPlayDeed "Play" You (Macros.allOf Macros.land) (PlayRider (Macros.fromZ Macros.onTopZ) Nothing Nothing False ItsOwnCost))) ]
       (Just (2, 2))

public export
azusaLostButSeeking : Card
azusaLostButSeeking =
  Macros.card "Azusa, Lost but Seeking"
       (Just [Macros.generic 2, Macros.pip Green]) [Legendary]
       (MkTypeLine [creatureType "Human", creatureType "Monk"] [Creature])
       [ Static (Macros.mayPlayAdditionalLands You (Macros.exactly 2)) ]
       (Just (1, 2))

public export
summerBloom : Card
summerBloom =
  Macros.card "Summer Bloom"
       (Just [Macros.generic 1, Macros.pip Green]) []
       (MkTypeLine [] [Sorcery])
       [ Spell (Continuously {ts = StaticFirstDone} (Macros.mayPlayAdditionalLands You (Macros.upTo 3))
                             (Just ThisTurn)) ]
       Nothing

public export
explore : Card
explore =
  Macros.card "Explore" (Just [Macros.generic 1, Macros.pip Green]) []
       (MkTypeLine [] [Sorcery])
       [ Spell (Sequentially
           [ Continuously {ts = StaticFirstDone} (Macros.mayPlayAdditionalLands You (Macros.exactly 1))
                          (Just ThisTurn)
           , (Macros.draw You (Lit 1)) ]) ]
       Nothing

public export
urbanEvolution : Card
urbanEvolution =
  Macros.card "Urban Evolution"
       (Just [Macros.generic 3, Macros.pip Green, Macros.pip Blue]) []
       (MkTypeLine [] [Sorcery])
       [ Spell (Sequentially
           [ Draw You (Lit 3)
           , Continuously {ts = StaticFirstDone} (Macros.mayPlayAdditionalLands You (Macros.exactly 1))
                          (Just ThisTurn) ]) ]
       Nothing

public export
eachPlayerPlaysAdditionalLand : StaticEffect []
eachPlayerPlaysAdditionalLand =
  Macros.mayPlayAdditionalLands (Macros.each AnyPlayer) (Macros.exactly 1)

public export
prismaticOmen : Card
prismaticOmen =
  Macros.card "Prismatic Omen" (Just [Macros.generic 1, Macros.pip Green]) []
       (MkTypeLine [] [Enchantment])
       [ Static (Becomes (Macros.allOf (And [Macros.land, HasPossessor ControllerAx You])) Adds (EveryTypeOf BasicLandSpace)) ]
       Nothing

public export
dryadOfTheIlysianGrove : Card
dryadOfTheIlysianGrove =
  Macros.card "Dryad of the Ilysian Grove"
       (Just [Macros.generic 2, Macros.pip Green]) []
       (MkTypeLine [creatureType "Nymph", creatureType "Dryad"] [Enchantment, Creature])
       [ Static (Macros.mayPlayAdditionalLands You (Macros.exactly 1))
       , Static (Becomes (Macros.allOf (And [Macros.land, HasPossessor ControllerAx You])) Adds (EveryTypeOf BasicLandSpace)) ]
       (Just (2, 4))

public export
mistformUltimus : Card
mistformUltimus =
  Macros.card "Mistform Ultimus"
       (Just [Macros.generic 3, Macros.pip Blue]) [Legendary]
       (MkTypeLine [creatureType "Illusion"] [Creature])
       [ Static (Becomes Macros.thisCreature Adds (EveryTypeOf CreatureSpace)) ]
       (Just (3, 3))

public export
amorphousAxe : Card
amorphousAxe =
  Macros.card "Amorphous Axe" (Just [Macros.generic 2]) []
       (MkTypeLine [artifactType "Equipment"] [Artifact])
       [ Static (AndAlso
           [ Gets (AttachHost Equipped (TypeW Creature)) (PtUp (Lit 3))
                  (PtUp (Lit 0))
           , Becomes (AttachHost Equipped (TypeW Creature)) Adds (EveryTypeOf CreatureSpace) ])
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
           , Becomes (AttachHost Equipped (TypeW Creature)) Adds (EveryTypeOf CreatureSpace) ])
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
           , Becomes (AttachHost Enchanted (TypeW Creature)) Adds (EveryTypeOf CreatureSpace) ]) ]
       Nothing

public export
nyleasPresence : Card
nyleasPresence =
  Macros.card "Nylea's Presence"
       (Just [Macros.generic 1, Macros.pip Green]) []
       (MkTypeLine [enchantmentType "Aura"] [Enchantment])
       [ Macros.keywordSubject "Enchant" Macros.land
       , Macros.triggered When (Enters Macros.thisAura Nothing) (Macros.draw You (Lit 1))
       , Static (Becomes (AttachHost Enchanted (TypeW Land)) Adds (EveryTypeOf BasicLandSpace)) ]
       Nothing

public export
volatileClaws : Card
volatileClaws =
  Macros.card "Volatile Claws"
       (Just [Macros.generic 2, Macros.pip Red]) []
       (MkTypeLine [] [Instant])
       [ Spell (Continuously {ts = StaticFirstDone}
           (AndAlso
             [ Gets (Macros.allOf (And [Macros.creature, HasPossessor ControllerAx You]))
                    (PtUp (Lit 2)) (PtUp (Lit 0))
             , Becomes (Macros.allOf (And [Macros.creature, HasPossessor ControllerAx You])) Adds (EveryTypeOf CreatureSpace) ])
           (Just Macros.untilEndOfTurn)) ]
       Nothing


||| Amoeboid Changeling
public export
amoeboidChangelingTypeAbilities : AbilitySeq []
amoeboidChangelingTypeAbilities =
  [ Macros.activated TapSymbol
      (Continuously {ts = StaticFirstDone} (Becomes (Macros.target Macros.creature) Adds (EveryTypeOf CreatureSpace))
                    (Just Macros.untilEndOfTurn))
  , Macros.activated TapSymbol
      (Continuously {ts = StaticFirstDone} (Becomes (Macros.target Macros.creature) Loses (EveryTypeOf CreatureSpace))
                    (Just Macros.untilEndOfTurn)) ]

||| Nameless Inversion
public export
namelessInversionBody : Effect []
namelessInversionBody =
  Continuously {ts = StaticFirstDone} (AndAlso [ Gets (Macros.target Macros.creature)
                               (PtUp (Lit 3)) (PtDown (Lit 3))
                        , Becomes It Loses (EveryTypeOf CreatureSpace) ])
               (Just Macros.untilEndOfTurn)

||| Ego Erasure
public export
egoErasureBody : Effect []
egoErasureBody =
  Continuously {ts = StaticFirstDone} (AndAlso [ Gets (Macros.allOf (And [Macros.creature,
                                            HasPossessor ControllerAx (Macros.target AnyPlayer)]))
                               (PtDown (Lit 2)) (PtUp (Lit 0))
                        , Becomes Them Loses (EveryTypeOf CreatureSpace) ])
               (Just Macros.untilEndOfTurn)

||| Curse of Conformity
public export
curseOfConformity : Card
curseOfConformity =
  Macros.card "Curse of Conformity" (Just [Macros.generic 4, Macros.pip White]) []
       (MkTypeLine [enchantmentType "Aura", enchantmentType "Curse"] [Enchantment])
       [ Macros.keywordSubject "Enchant" AnyPlayer
       , Static (AndAlso
           [ HasBasePt (Macros.allOf (And [Macros.creature, Not (HasSupertype Legendary),
                                    HasPossessor ControllerAx (AttachHost Enchanted PlayerW)]))
                       (Lit 3) (Lit 3)
           , Becomes Them Loses (EveryTypeOf CreatureSpace) ]) ]
       Nothing

||| Lithoform Blight
public export
lithoformBlightLoss : StaticEffect []
lithoformBlightLoss =
  AndAlso [ Becomes (AttachHost Enchanted (TypeW Land)) Loses (EveryTypeOf LandSpace)
          , LosesAllAbilities It Nothing ]

||| Energybending
public export
energybending : Card
energybending =
  Macros.card "Energybending" (Just [Macros.generic 2]) []
       (MkTypeLine [spellType "Lesson"] [Instant])
       [ Spell (Sequentially
                  [ Continuously {ts = StaticFirstDone}
                      (Becomes (Macros.allOf (And [Macros.land, HasPossessor ControllerAx You])) Adds (EveryTypeOf BasicLandSpace))
                      (Just Macros.untilEndOfTurn)
                  , (Macros.draw You (Lit 1)) ]) ]
       Nothing

||| Ashes of the Fallen
public export
ashesOfTheFallen : Card
ashesOfTheFallen =
  Macros.card "Ashes of the Fallen" (Just [Macros.generic 2]) []
       (MkTypeLine [] [Artifact])
       [ Static (Macros.entersChoosing Macros.thisArtifact (SubtypeQ Creature))
       , Static (Becomes (Macros.each (And [Macros.creature, InZone (Macros.graveyardOf You)])) Adds (ChosenQuality (OfChosen (SubtypeQ Creature)))) ]
       Nothing

||| Yedora, Grave Gardener
public export
yedoraGraveGardener : Card
yedoraGraveGardener =
  Macros.card "Yedora, Grave Gardener"
       (Just [Macros.generic 4, Macros.pip Green]) [Legendary]
       (MkTypeLine [creatureType "Treefolk", creatureType "Druid"] [Creature])
       [ Macros.triggered Whenever
           (Dies (Macros.a (And [Macros.nontoken, Macros.creatureYouControl,
                                 OtherThan Macros.thisCreature])))
           (Macros.may You
              (Sequentially
                 [ Move It Macros.battlefieldZ
                        [EntersAs FaceDown, Under (Macros.ownerOf It)]
                 , Continuously {ts = StaticFirstDone}
                     (Becomes It Sets (Bundle (MkToken Nothing []
                                        (MkTypeLine [landType "Forest"] [Land])
                                        [] Nothing) Nothing))
                     Nothing ])) ]
       (Just (5, 5))


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
                                   (Macros.counted (Macros.exactly 5) Macros.artifact))])
                         (ExtraTurn You (Lit 1))] Nothing

public export
magistratesScepter : Card
magistratesScepter =
  Macros.card "Magistrate's Scepter" (Just [Macros.generic 3]) []
       (MkTypeLine [] [Artifact])
       [ Macros.activated (Compound [Mana [Macros.generic 4], TapSymbol])
                          (PutCounters (Lit 1) (PrintedKind (Named "Charge")) Macros.thisArtifact)
       , Macros.activated (Compound [TapSymbol,
                              Do (RemoveCounters (Just (Macros.exactly 3)) (Just (PrintedKind (Named "Charge"))) Macros.thisArtifact)])
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
           (Macros.dealsCombatDamage Macros.thisCreature (Macros.a AnyPlayer))
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
       , Macros.triggered At (BeginningOf Upkeep (Macros.yours))
           ((May You (Pay You (Mana [Macros.pip Blue]) PaidOnce) Nothing (Just (Macros.sacrifice You Macros.thisEnchantment)))) ] Nothing

public export
yawgmothsBargain : Card
yawgmothsBargain =
  Macros.card "Yawgmoth's Bargain"
       (Just [Macros.generic 4, Macros.pip Black, Macros.pip Black]) []
       (MkTypeLine [] [Enchantment])
       [ Static (Skips You DrawStep)
       , Macros.activated (Macros.payLife You 1) (Macros.draw You (Lit 1)) ] Nothing

public export
sandsOfTimeSkip : StaticEffect []
sandsOfTimeSkip = Skips (Macros.each AnyPlayer) UntapStep

public export
wormfangManta : Card
wormfangManta =
  Macros.card "Wormfang Manta"
       (Just [Macros.generic 5, Macros.pip Blue, Macros.pip Blue]) []
       (MkTypeLine [creatureType "Nightmare", creatureType "Fish", creatureType "Beast"] [Creature])
       [ Macros.keyword "Flying"
       , Macros.triggered When (Enters Macros.thisCreature Nothing) (SkipsNext You Turn (Lit 1))
       , Macros.triggered When (Macros.leavesBattlefield Macros.thisCreature) (ExtraTurn You (Lit 1)) ]
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
                             Macros.delayed (BeginningOf EndStep (Macros.thatTurns))
                                            (Concludes LoseGame You)])] Nothing

public export
finalFortuneThatTurn : Noun (effIntro {bs = []} (ExtraTurn You (Lit 1))) TurnRef
finalFortuneThatTurn = Macros.thatTurn

public export
lastChance : Card
lastChance =
  Macros.card "Last Chance" (Just [Macros.pip Red, Macros.pip Red]) []
       (MkTypeLine [] [Sorcery])
       [Spell (Sequentially [ExtraTurn You (Lit 1),
                             Macros.delayed (BeginningOf EndStep (Macros.thatTurns))
                                            (Concludes LoseGame You)])] Nothing

public export
chanceForGlory : Card
chanceForGlory =
  Macros.card "Chance for Glory"
       (Just [Macros.generic 1, Macros.pip Red, Macros.pip White]) []
       (MkTypeLine [] [Instant])
       [Spell (Sequentially
         [ Continuously {ts = StaticFirstDone} (Gains (Macros.allOf (And [Macros.creature, HasPossessor ControllerAx You]))
                               (Macros.keyword "Indestructible")) Nothing
         , ExtraTurn You (Lit 1)
         , Macros.delayed (BeginningOf EndStep (Macros.thatTurns))
                          (Concludes LoseGame You)])] Nothing

public export
aggravatedAssault : Card
aggravatedAssault =
  Macros.card "Aggravated Assault" (Just [Macros.generic 2, Macros.pip Red]) []
       (MkTypeLine [] [Enchantment])
       [Macros.activatedOnlyDuring (Mana [Macros.generic 3, Macros.pip Red, Macros.pip Red])
                                   (Sequentially
                                     [ SetStatus Untapped
                                         (Macros.allOf (And [Macros.creature, HasPossessor ControllerAx You]))
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
             (Macros.allOf (And [Macros.creature,
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
           ((May You (Pay You (Mana [Macros.generic 5, Macros.pip Red, Macros.pip Red]) PaidOnce) (Just (Sequentially
                [ SetStatus Untapped (Macros.allOf (And [Macros.creature, Attacking]))
                , Macros.additionalPart Combat Nothing (Lit 1)])) Nothing)) ] (Just (5, 5))

public export
fullThrottleFirstLine : Effect []
fullThrottleFirstLine = Macros.additionalPart Combat (Just MainPhase) (Lit 2)

public export
raphaelAdditionalCombat : Effect []
raphaelAdditionalCombat = Macros.additionalPart Combat (Just Combat) (Lit 1)

public export
yshtolaAdditionalEndStep : Effect []
yshtolaAdditionalEndStep = Macros.additionalPart EndStep Nothing (Lit 1)

||| Sphinx of the Second Sun
public export
sphinxOfTheSecondSun : Card
sphinxOfTheSecondSun =
  Macros.card "Sphinx of the Second Sun"
       (Just [Macros.generic 6, Macros.pip Blue, Macros.pip Blue]) []
       (MkTypeLine [creatureType "Sphinx"] [Creature])
       [ Macros.keyword "Flying"
       , Macros.triggered At (BeginningOf PostcombatMain (Macros.yours))
           (Macros.additionalPart BeginningPhase (Just PostcombatMain) (Lit 1)) ]
       (Just (6, 6))

||| Obeka, Splitter of Seconds
public export
obekaSplitterOfSeconds : Card
obekaSplitterOfSeconds =
  Macros.card "Obeka, Splitter of Seconds"
       (Just [Macros.generic 1, Macros.pip Blue, Macros.pip Black,
              Macros.pip Red]) [Legendary]
       (MkTypeLine [creatureType "Ogre", creatureType "Warlock"] [Creature])
       [ Macros.keyword "Menace"
       , Macros.triggered Whenever
           (Macros.dealsCombatDamage Macros.thisCreature (Macros.a AnyPlayer))
           (Macros.getsAdditionalPart You Upkeep ThatMuch) ]
       (Just (2, 5))

||| Paradox Haze
public export
paradoxHaze : Card
paradoxHaze =
  Macros.card "Paradox Haze" (Just [Macros.generic 2, Macros.pip Blue]) []
       (MkTypeLine [enchantmentType "Aura"] [Enchantment])
       [ Macros.keywordSubject "Enchant" AnyPlayer
       , Macros.triggered At
           (NthOccurrence (Nth 1) (Just Turn)
              (Macros.beginningOfPossessed Upkeep (AttachHost Enchanted PlayerW)))
           (Macros.getsAdditionalPart (That PlayerW) Upkeep (Lit 1)) ]
       Nothing

public export
ninthDoctorAdditionalUpkeep : Effect []
ninthDoctorAdditionalUpkeep = Macros.getsAdditionalPart You Upkeep (Lit 1)

||| Empty City Ruse
public export
emptyCityRuse : Card
emptyCityRuse =
  Macros.card "Empty City Ruse" (Just [Macros.pip White]) []
       (MkTypeLine [] [Sorcery])
       [Spell (SkipsAllOf (Macros.target Opponent) Combat)] Nothing

||| False Peace
public export
falsePeace : Card
falsePeace =
  Macros.card "False Peace" (Just [Macros.pip White]) []
       (MkTypeLine [] [Sorcery])
       [Spell (SkipsAllOf (Macros.target AnyPlayer) Combat)] Nothing

||| Seedborn Muse
public export
seedbornMuse : Card
seedbornMuse =
  Macros.card "Seedborn Muse"
       (Just [Macros.generic 3, Macros.pip Green, Macros.pip Green]) []
       (MkTypeLine [creatureType "Spirit"] [Creature])
       [ Static (Macros.untapsDuring (Macros.allOf (HasPossessor ControllerAx You)) (Just (Macros.each Macros.otherPlayer))) ]
       (Just (2, 4))

||| Unwinding Clock
public export
unwindingClock : Card
unwindingClock =
  Macros.card "Unwinding Clock" (Just [Macros.generic 4]) []
       (MkTypeLine [] [Artifact])
       [ Static (Macros.untapsDuring (Macros.allOf (And [Macros.artifact, HasPossessor ControllerAx You]))
                                     (Just (Macros.each Macros.otherPlayer))) ]
       Nothing

||| Drumbellower
public export
drumbellower : Card
drumbellower =
  Macros.card "Drumbellower" (Just [Macros.generic 2, Macros.pip White]) []
       (MkTypeLine [creatureType "Spirit"] [Creature])
       [ Macros.keyword "Flying"
       , Static (Macros.untapsDuring (Macros.allOf (And [Macros.creature, HasPossessor ControllerAx You]))
                                     (Just (Macros.each Macros.otherPlayer))) ]
       (Just (2, 1))

||| Thousand Moons Infantry
public export
thousandMoonsInfantry : Card
thousandMoonsInfantry =
  Macros.card "Thousand Moons Infantry"
       (Just [Macros.generic 2, Macros.pip White]) []
       (MkTypeLine [creatureType "Human", creatureType "Soldier"] [Creature])
       [ Static (Macros.untapsDuring Macros.thisCreature (Just (Macros.each Macros.otherPlayer))) ]
       (Just (2, 4))

||| Battlefront Krushok
public export
battlefrontKrushokEvasion : Ability
battlefrontKrushokEvasion =
  Static (Deontic Macros.thisCreature Forbid ["Block"] Patient (Just (MoreThan (Lit 1)))
                  (DeonticCounterpart (Macros.allOf Macros.creature)) Nothing NoDeonticRider)

||| Foriysian Brigade
public export
foriysianBrigade : Card
foriysianBrigade =
  Macros.card "Foriysian Brigade" (Just [Macros.generic 3, Macros.pip White]) []
       (MkTypeLine [creatureType "Human", creatureType "Soldier"] [Creature])
       [Static (Macros.mayBlockAdditional Macros.thisCreature (Macros.exactly 1))]
       (Just (2, 4))

||| Two-Headed Giant of Foriys
public export
twoHeadedGiantOfForiys : Card
twoHeadedGiantOfForiys =
  Macros.card "Two-Headed Giant of Foriys"
       (Just [Macros.generic 4, Macros.pip Red]) []
       (MkTypeLine [creatureType "Giant"] [Creature])
       [ Macros.keyword "Trample"
       , Static (Macros.mayBlockAdditional Macros.thisCreature (Macros.exactly 1)) ]
       (Just (4, 4))

public export
highGround : Card
highGround =
  Macros.card "High Ground" (Just [Macros.pip White]) []
       (MkTypeLine [] [Enchantment])
       [Static (Macros.mayBlockAdditional (Macros.each Macros.creatureYouControl)
                                   (Macros.exactly 1))]
       Nothing

||| Watcher in the Web
public export
watcherInTheWeb : Card
watcherInTheWeb =
  Macros.card "Watcher in the Web" (Just [Macros.generic 4, Macros.pip Green]) []
       (MkTypeLine [creatureType "Spider"] [Creature])
       [ Macros.keyword "Reach"
       , Static (Macros.mayBlockAdditional Macros.thisCreature (Macros.exactly 7)) ]
       (Just (2, 5))

||| Rites of Flourishing
public export
ritesOfFlourishing : Card
ritesOfFlourishing =
  Macros.card "Rites of Flourishing"
       (Just [Macros.generic 2, Macros.pip Green]) []
       (MkTypeLine [] [Enchantment])
       [ Macros.triggered At (BeginningOf DrawStep (Macros.eachPlayers))
           (Draw (That PlayerW) (Lit 1))
       , Static (Macros.mayPlayAdditionalLands (Macros.each AnyPlayer) (Macros.exactly 1)) ]
       Nothing

||| Karn Liberated
public export
karnRestart : Effect []
karnRestart = RestartsGame

||| Forgotten Lore
public export
forgottenLoreRepeat : Effect []
forgottenLoreRepeat =
  Sequentially
    [ Choose (Macros.a (InZone (Macros.graveyardOf You)))
             (Just (Macros.target Opponent)) Openly
    , (May You (Pay You (Mana [Macros.pip Green]) PaidOnce) (Just (Repeat AgainExcludingChosen)) Nothing) ]

||| Leyline of the Meek
public export
leylineOfTheMeek : Card
leylineOfTheMeek =
  Macros.card "Leyline of the Meek"
       (Just [Macros.generic 2, Macros.pip White, Macros.pip White]) []
       (MkTypeLine [] [Enchantment])
       [ MayBeginOnBattlefield
       , Static (Gets (Macros.allOf (And [Macros.creature, IsToken]))
                      (PtUp (Lit 1)) (PtUp (Lit 1))) ]
       Nothing

||| Leyline of Vitality
public export
leylineOfVitality : Card
leylineOfVitality =
  Macros.card "Leyline of Vitality"
       (Just [Macros.generic 2, Macros.pip Green, Macros.pip Green]) []
       (MkTypeLine [] [Enchantment])
       [ MayBeginOnBattlefield
       , Static (Gets (Macros.allOf Macros.creatureYouControl)
                      (PtUp (Lit 0)) (PtUp (Lit 1)))
       , Macros.triggered Whenever
           (Enters (Macros.a Macros.creatureYouControl) Nothing)
           (Macros.may You (Macros.gainsLife You (Lit 1))) ]
       Nothing

||| Blatant Thievery
public export
blatantThievery : Card
blatantThievery =
  Macros.card "Blatant Thievery"
       (Just [Macros.generic 4, Macros.pip Blue, Macros.pip Blue,
              Macros.pip Blue]) []
       (MkTypeLine [] [Sorcery])
       [ Spell (ForEachOf (Macros.each Opponent)
                  (Continuously {ts = StaticFirstDone}
                     (GainsControl You (Macros.target (HasPossessor ControllerAx (That PlayerW))))
                     Nothing)) ]
       Nothing

||| Ral Zarek, Guest Lecturer's ultimate
public export
ralZarekGuestLecturerUltimate : Effect []
ralZarekGuestLecturerUltimate =
  Sequentially [ FlipCoins You (FlipCount (Lit 5))
               , SkipsNext (Macros.target Opponent) Turn (LetterVal X)
               , Define X (CoinsShowing Heads) ]



public export
grumgully : Card
grumgully =
  Macros.card "Grumgully, the Generous"
       (Just [Macros.generic 1, Macros.pip Red, Macros.pip Green]) [Legendary]
       (MkTypeLine [creatureType "Goblin", creatureType "Shaman"] [Creature])
       [Static (Macros.entersWithAdditionalCounters (Macros.each (And [Macros.creature, HasPossessor ControllerAx You,
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
       [ Static (Macros.entersWithAdditionalCounters (Macros.each (And [HasSubtype (creatureType "Dragon"), HasPossessor ControllerAx You]))
                                                     (Lit 1)
                                                     Macros.plusOnePlusOne)
       , Macros.activated TapSymbol (AddMana You (Lit 1) (AnyColor SameColor) []) ]
       Nothing

public export
sageOfFables : Card
sageOfFables =
  Macros.card "Sage of Fables" (Just [Macros.generic 2, Macros.pip Blue]) []
       (MkTypeLine [creatureType "Merfolk", creatureType "Wizard"] [Creature])
       [ Static (Macros.entersWithAdditionalCounters (Macros.each (And [Macros.creature, HasSubtype (creatureType "Wizard"), HasPossessor ControllerAx You,
                                                                 OtherThan Macros.thisCreature]))
                                                     (Lit 1)
                                                     Macros.plusOnePlusOne)
       , Macros.activated (Compound [Mana [Macros.generic 2],
                              Do (RemoveCounters (Just (Macros.exactly 1)) (Just (PrintedKind Macros.plusOnePlusOne))
                                   (Macros.a (And [Macros.creature, HasPossessor ControllerAx You])))])
                          (Sequentially
              [ Macros.searchLibraryOrGraveyard
                  (And [Macros.artifact,
                        Compare [CharAxis ManaValue] AtMost (Lit 2)]) ]) ]
       (Just (2, 2))

public export
metallicMimic : Card
metallicMimic =
  Macros.card "Metallic Mimic" (Just [Macros.generic 2]) []
       (MkTypeLine [creatureType "Shapeshifter"] [Artifact, Creature])
       [ Static (Macros.entersChoosing Macros.thisCreature (SubtypeQ Creature))
       , Static (Becomes Macros.thisCreature Adds (ChosenQuality (OfChosen (SubtypeQ Creature))))
       , Static (Macros.entersWithAdditionalCounters (Macros.each (And [Macros.creature, HasPossessor ControllerAx You,
                                                                 OfChosen (SubtypeQ Creature),
                                                                 OtherThan Macros.thisCreature]))
                                                     (Lit 1)
                                                     Macros.plusOnePlusOne) ]
       (Just (2, 1))

public export
faceDownFlyingCounter : StaticEffect []
faceDownFlyingCounter =
  Macros.entersWithCounters (Macros.allOf (And [Macros.creature, HasPossessor ControllerAx You,
                                  HasStatus FaceDown]))
                            (Lit 1) (KeywordCounter "Flying")

public export
curatorBeastieLine : StaticEffect []
curatorBeastieLine =
  Macros.entersWithAdditionalCounters (Macros.allOf (And [Macros.creature, HasPossessor ControllerAx You, IsColorless]))
                                      (Lit 2)
                                      Macros.plusOnePlusOne

public export
renataLine : StaticEffect []
renataLine =
  Macros.entersWithAdditionalCounters (Macros.each (And [Macros.creature, HasPossessor ControllerAx You,
                                                  OtherThan Macros.thisCreature]))
                                      (Lit 1)
                                      Macros.plusOnePlusOne

public export
tayamLine : StaticEffect []
tayamLine =
  Macros.entersWithAdditionalCounters (Macros.each (And [Macros.creature, HasPossessor ControllerAx You,
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
       , Static (Macros.objectCant "Activate"
                   (Macros.allOf (And [ AbilityHead AnyActivated
                               , AbilityOf (Macros.allOf (And [Macros.source,
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
       , Static (Macros.objectCant "Cast"
                   (Macros.allOf (And [Macros.spell, Named ChosenName])))
       , Static (Macros.objectCant "Activate"
                   (Macros.allOf (And [ AbilityHead AnyActivated
                               , AbilityOf (Macros.allOf (And [Macros.source,
                                                        Named ChosenName])) ]))) ]
       (Just (3, 3))

public export
pithingNeedle : Card
pithingNeedle =
  Macros.card "Pithing Needle" (Just [Macros.generic 1]) []
       (MkTypeLine [] [Artifact])
       [ Static (Macros.entersChoosing Macros.thisArtifact CardName)
       , Static (Macros.objectCant "Activate"
                   (Macros.allOf (And [ AbilityHead AnyActivated
                               , AbilityOf (Macros.allOf (And [Macros.source,
                                                        Named ChosenName]))
                               , Not IsManaAbility ]))) ]
       Nothing

public export
cursedTotem : Card
cursedTotem =
  Macros.card "Cursed Totem" (Just [Macros.generic 2]) []
       (MkTypeLine [] [Artifact])
       [ Static (Macros.objectCant "Activate"
                   (Macros.allOf (And [ AbilityHead AnyActivated
                               , AbilityOf (Macros.allOf Macros.creature) ]))) ]
       Nothing

public export
nullRod : Card
nullRod =
  Macros.card "Null Rod" (Just [Macros.generic 2]) []
       (MkTypeLine [] [Artifact])
       [ Static (Macros.objectCant "Activate"
                   (Macros.allOf (And [ AbilityHead AnyActivated
                               , AbilityOf (Macros.allOf Macros.artifact) ]))) ]
       Nothing

public export
stonySilence : Card
stonySilence =
  Macros.card "Stony Silence" (Just [Macros.generic 1, Macros.pip White]) []
       (MkTypeLine [] [Enchantment])
       [ Static (Macros.objectCant "Activate"
                   (Macros.allOf (And [ AbilityHead AnyActivated
                               , AbilityOf (Macros.allOf Macros.artifact) ]))) ]
       Nothing

public export
clarionConqueror : Card
clarionConqueror =
  Macros.card "Clarion Conqueror" (Just [Macros.generic 2, Macros.pip White]) []
       (MkTypeLine [creatureType "Dragon"] [Creature])
       [ Macros.keyword "Flying"
       , Static (Macros.objectCant "Activate"
                   (Macros.allOf (And [ AbilityHead AnyActivated
                               , AbilityOf (Macros.allOf (Or [Macros.artifact,
                                                       Macros.creature,
                                                       HasType Planeswalker])) ]))) ]
       (Just (3, 3))

public export
dampingMatrix : Card
dampingMatrix =
  Macros.card "Damping Matrix" (Just [Macros.generic 3]) []
       (MkTypeLine [] [Artifact])
       [ Static (Macros.objectCant "Activate"
                   (Macros.allOf (And [ AbilityHead AnyActivated
                               , AbilityOf (Macros.allOf (Or [Macros.artifact,
                                                       Macros.creature]))
                               , Not IsManaAbility ]))) ]
       Nothing

||| Fluctuator
public export
fluctuator : Card
fluctuator =
  Macros.card "Fluctuator" (Just [Macros.generic 2]) []
       (MkTypeLine [] [Artifact])
       [ Static (CostsToCast
                   (Macros.allOf (And [AbilityHead (KeywordClass "Cycling"),
                                ActivatedBy You]))
                   (CostLess (Lit 2) Nothing)) ]
       Nothing

||| Boom Scholar
public export
boomScholarExhaustDiscount : Ability
boomScholarExhaustDiscount =
  Static (CostsToCast
            (Macros.allOf (And [ AbilityHead (KeywordClass "Exhaust")
                        , AbilityOf (Macros.allOf (And [Permanent,
                                                 OtherThan Macros.thisCreature,
                                                 HasPossessor ControllerAx You])) ]))
            (CostLess (Lit 2) Nothing))

||| Hulk, Gamma Goliath
public export
hulkPowerUpDiscount : Ability
hulkPowerUpDiscount =
  Static (CostsToCast
            (Macros.allOf (And [ AbilityHead (KeywordClass "PowerUp")
                        , AbilityOf (Macros.allOf (And [Macros.creature,
                                                 OtherThan Macros.thisCreature,
                                                 HasPossessor ControllerAx You])) ]))
            (CostLess (Lit 3) Nothing))

||| Kang the Conqueror
public export
kangPowerUpLock : StaticEffect []
kangPowerUpLock =
  Macros.objectCant "Activate"
    (Macros.allOf (AbilityHead (KeywordClass "PowerUp")))

||| Grand Abolisher
public export
grandAbolisher : Card
grandAbolisher =
  Macros.card "Grand Abolisher" (Just [Macros.pip White, Macros.pip White]) []
       (MkTypeLine [creatureType "Human", creatureType "Cleric"] [Creature])
       [ Static (OnlyDuring Turn (Just You)
                   (AndAlso
                      [ Macros.cantDoTo "Cast" (PlayerGroup YourOpponents)
                          (Macros.allOf Macros.spell)
                      , Macros.cantDoTo "Activate" (PlayerGroup YourOpponents)
                          (Macros.allOf (And [ AbilityHead AnyActivated
                                      , AbilityOf (Macros.allOf (Or [Macros.artifact,
                                                              Macros.creature,
                                                              HasType Enchantment])) ])) ])) ]
       (Just (2, 2))

||| Festival
public export
festival : Card
festival =
  Macros.card "Festival" (Just [Macros.pip White]) []
       (MkTypeLine [] [Instant])
       [ Static (OnlyDuring Upkeep (Just Macros.anOpponent)
                   (Macros.deontic This Permit ["Cast"] Patient NoDeonticPatient))
       , Spell (Macros.cantAttack (Macros.allOf Macros.creature)
                                  (Just Macros.thisTurn)) ]
       Nothing

||| Kopala, Warden of Waves
public export
kopalaWardenOfWaves : Card
kopalaWardenOfWaves =
  Macros.card "Kopala, Warden of Waves"
       (Just [Macros.generic 1, Macros.pip Blue, Macros.pip Blue])
       [Legendary]
       (MkTypeLine [creatureType "Merfolk", creatureType "Wizard"] [Creature])
       [ Static (CostsToCast
                   (Macros.allOf (And [ Macros.spell
                               , CastBy (PlayerGroup YourOpponents)
                               , Targets (Macros.a (And [Macros.creature,
                                                         HasSubtype (creatureType "Merfolk"),
                                                         HasPossessor ControllerAx You]))
                                         SomeTarget ]))
                   (CostMore (Lit 2)))
       , Static (CostsToCast
                   (Macros.allOf (And [ AbilityHead AnyActivated
                               , ActivatedBy (PlayerGroup YourOpponents)
                               , Targets (Macros.a (And [Macros.creature,
                                                         HasSubtype (creatureType "Merfolk"),
                                                         HasPossessor ControllerAx You]))
                                         SomeTarget ]))
                   (CostMore (Lit 2))) ]
       (Just (2, 2))

||| Tithe Taker
public export
titheTaker : Card
titheTaker =
  Macros.card "Tithe Taker" (Just [Macros.generic 1, Macros.pip White]) []
       (MkTypeLine [creatureType "Human", creatureType "Soldier"] [Creature])
       [ Static (OnlyDuring Turn (Just You)
                   (AndAlso
                      [ CostsToCast
                          (Macros.allOf (And [Macros.spell,
                                       CastBy (PlayerGroup YourOpponents)]))
                          (CostMore (Lit 1))
                      , CostsToCast
                          (Macros.allOf (And [ AbilityHead AnyActivated
                                      , ActivatedBy (PlayerGroup YourOpponents)
                                      , Not IsManaAbility ]))
                          (CostMore (Lit 1)) ]))
       , Macros.keywordNumber "Afterlife" (Lit 1) ]
       (Just (2, 1))

||| Gaddock Teeg
public export
gaddockTeeg : Card
gaddockTeeg =
  Macros.card "Gaddock Teeg" (Just [Macros.pip Green, Macros.pip White])
       [Legendary]
       (MkTypeLine [creatureType "Kithkin", creatureType "Advisor"] [Creature])
       [ Static (Macros.objectCant "Cast"
                   (Macros.allOf (And [Macros.spell, Not (HasType Creature),
                                Compare [CharAxis ManaValue] AtLeast (Lit 4)])))
       , Static (Macros.objectCant "Cast"
                   (Macros.allOf (And [Macros.spell, Not (HasType Creature),
                                ManaCostHas Variable]))) ]
       (Just (2, 2))

||| Vexing Shusher
public export
vexingShusher : Card
vexingShusher =
  Macros.card "Vexing Shusher"
       (Just [Macros.hybridPip Red Green, Macros.hybridPip Red Green]) []
       (MkTypeLine [creatureType "Goblin", creatureType "Shaman"] [Creature])
       [ Static (Macros.objectCant "Counter" This)
       , Macros.activated (Mana [Macros.hybridPip Red Green])
           (Continuously {ts = StaticFirstDone}
              (Macros.objectCant "Counter" (Macros.target Macros.spell))
              Nothing) ]
       (Just (2, 2))

public export
trainingGrounds : Card
trainingGrounds =
  Macros.card "Training Grounds" (Just [Macros.pip Blue]) []
       (MkTypeLine [] [Enchantment])
       [ Static (CostsToCast
                   (Macros.allOf (And [ AbilityHead AnyActivated
                               , AbilityOf (Macros.allOf Macros.creatureYouControl) ]))
                   (CostLess (Lit 2) (Just (Lit 1)))) ]
       Nothing

||| Power Artifact
public export
powerArtifact : Card
powerArtifact =
  Macros.card "Power Artifact" (Just [Macros.pip Blue, Macros.pip Blue]) []
       (MkTypeLine [enchantmentType "Aura"] [Enchantment])
       [ Macros.keywordSubject "Enchant" Macros.artifact
       , Static (CostsToCast
                   (Macros.allOf (And [ AbilityHead AnyActivated
                               , AbilityOf (AttachHost Enchanted (TypeW Artifact)) ]))
                   (CostLess (Lit 2) (Just (Lit 1)))) ]
       Nothing

||| Fervent Champion
public export
ferventChampionEquipDiscount : Ability
ferventChampionEquipDiscount =
  Static (CostsToCast
            (Macros.allOf (And [ AbilityHead (KeywordClass "Equip")
                        , ActivatedBy You
                        , Targets Macros.thisCreature SomeTarget ]))
            (CostLess (Lit 3) Nothing))

public export
suppressionField : Card
suppressionField =
  Macros.card "Suppression Field" (Just [Macros.generic 1, Macros.pip White]) []
       (MkTypeLine [] [Enchantment])
       [ Static (CostsToCast
                   (Macros.allOf (And [AbilityHead AnyActivated, Not IsManaAbility]))
                   (CostMore (Lit 2))) ]
       Nothing

public export
gloom : Card
gloom =
  Macros.card "Gloom" (Just [Macros.generic 2, Macros.pip Black]) []
       (MkTypeLine [] [Enchantment])
       [ Static (CostsToCast (Macros.allOf (And [Macros.spell, ColorIs White]))
                             (CostMore (Lit 3)))
       , Static (CostsToCast
                   (Macros.allOf (And [ AbilityHead AnyActivated
                               , AbilityOf (Macros.allOf (And [Macros.enchantment,
                                                        ColorIs White])) ]))
                   (CostMore (Lit 3))) ]
       Nothing

public export
bureauHeadmaster : Card
bureauHeadmaster =
  Macros.card "Bureau Headmaster" (Just [Macros.pip Red, Macros.pip White]) []
       (MkTypeLine [creatureType "Human", creatureType "Assassin"] [Creature])
       [ Static (CostsToCast
                   (Macros.allOf (And [Macros.spell, HasSubtype (artifactType "Equipment"), CastBy You]))
                   (CostLess (Lit 1) Nothing))
       , Static (CostsToCast
                   (Macros.allOf (And [AbilityHead (KeywordClass "Equip"), ActivatedBy You]))
                   (CostLess (Lit 1) Nothing)) ]
       (Just (2, 2))

public export
demotion : Card
demotion =
  Macros.card "Demotion" (Just [Macros.pip White]) []
       (MkTypeLine [enchantmentType "Aura"] [Enchantment])
       [ Macros.keywordSubject "Enchant" Macros.creature
       , Static (AndAlso
           [ Macros.deontic (AttachHost Enchanted (TypeW Creature)) Forbid ["Block"] Agent NoDeonticPatient
           , Macros.objectCant "Activate"
               (Macros.allOf (And [AbilityHead AnyActivated, AbilityOf It])) ]) ]
       Nothing

public export
vipersKiss : Card
vipersKiss =
  Macros.card "Viper's Kiss" (Just [Macros.pip Black]) []
       (MkTypeLine [enchantmentType "Aura"] [Enchantment])
       [ Macros.keywordSubject "Enchant" Macros.creature
       , Static (AndAlso
           [ Gets (AttachHost Enchanted (TypeW Creature)) (PtDown (Lit 1)) (PtDown (Lit 1))
           , Macros.objectCant "Activate"
               (Macros.allOf (And [AbilityHead AnyActivated, AbilityOf It])) ]) ]
       Nothing

public export
stupefyingTouch : Card
stupefyingTouch =
  Macros.card "Stupefying Touch" (Just [Macros.generic 1, Macros.pip Blue]) []
       (MkTypeLine [enchantmentType "Aura"] [Enchantment])
       [ Macros.keywordSubject "Enchant" Macros.creature
       , Macros.triggered When (Enters Macros.thisAura Nothing) (Draw You (Lit 1))
       , Static (Macros.objectCant "Activate"
                   (Macros.allOf (And [ AbilityHead AnyActivated
                               , AbilityOf (AttachHost Enchanted (TypeW Creature)) ]))) ]
       Nothing

public export
oppressiveRaysLine : StaticEffect []
oppressiveRaysLine =
  CostsToCast
    (Macros.allOf (And [ AbilityHead AnyActivated
                , AbilityOf (AttachHost Enchanted (TypeW Creature)) ]))
    (CostMore (Lit 3))

public export
linvalaKeeperOfSilence : Card
linvalaKeeperOfSilence =
  Macros.card "Linvala, Keeper of Silence"
       (Just [Macros.generic 2, Macros.pip White, Macros.pip White]) [Legendary]
       (MkTypeLine [creatureType "Angel"] [Creature])
       [ Macros.keyword "Flying"
       , Static (Macros.objectCant "Activate"
                   (Macros.allOf (And [ AbilityHead AnyActivated
                               , AbilityOf (Macros.allOf (And [Macros.creature,
                                                        HasPossessor ControllerAx (PlayerGroup YourOpponents)])) ]))) ]
       (Just (3, 4))

public export
eidolonOfObstruction : Card
eidolonOfObstruction =
  Macros.card "Eidolon of Obstruction" (Just [Macros.generic 1, Macros.pip White]) []
       (MkTypeLine [creatureType "Spirit"] [Enchantment, Creature])
       [ Macros.keyword "FirstStrike"
       , Static (CostsToCast
                   (Macros.allOf (And [ AbilityHead LoyaltyClass
                               , AbilityOf (Macros.allOf (And [HasType Planeswalker,
                                                        HasPossessor ControllerAx (PlayerGroup YourOpponents)])) ]))
                   (CostMore (Lit 1))) ]
       (Just (2, 1))

public export
forceOfWill : Card
forceOfWill =
  Macros.card "Force of Will"
       (Just [Macros.generic 3, Macros.pip Blue, Macros.pip Blue]) []
       (MkTypeLine [] [Instant])
       [ Static (AltCost This (Just (Compound
                   [ Do (ChangeLife You (Down (Lit 1)))
                   , Do (Macros.exile You (Macros.a (And [ColorIs Blue,
                                                      InZone (Macros.handOf You)]))) ])))
       , Spell (Macros.counterSpell (Macros.target Macros.spell)) ]
       Nothing

public export
crash : Card
crash =
  Macros.card "Crash" (Just [Macros.generic 2, Macros.pip Red]) []
       (MkTypeLine [] [Instant])
       [ Static (AltCost This (Just (Do (Macros.sacrifice You
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
                      [ Macros.exists (And [Macros.land, HasSubtype (landType "Island"),
                                     HasPossessor ControllerAx Macros.anOpponent])
                      , Macros.exists (And [Macros.land, HasSubtype (landType "Mountain"),
                                     HasPossessor ControllerAx You]) ])
                   (AltCost This Nothing))
       , Spell (Macros.destroy (Macros.target Macros.artifact)) ]
       Nothing

public export
abolish : Card
abolish =
  Macros.card "Abolish" (Just [Macros.generic 1, Macros.pip White, Macros.pip White]) []
       (MkTypeLine [] [Instant])
       [ Static (AltCost This (Just (Do (Macros.discard You
                   (Macros.a (And [HasSubtype (landType "Plains"), InZone Macros.handZ]))))))
       , Spell (Macros.destroy (Macros.target (Or [Macros.artifact,
                                                   Macros.enchantment]))) ]
       Nothing

public export
gush : Card
gush =
  Macros.card "Gush" (Just [Macros.generic 4, Macros.pip Blue]) []
       (MkTypeLine [] [Instant])
       [ Static (AltCost This (Just (Do (Macros.move
                   (Macros.counted (Macros.exactly 2)
                                 (And [Macros.land, HasSubtype (landType "Island"),
                                       HasPossessor ControllerAx You]))
                   Macros.handZ))))
       , Spell ((Macros.draw You (Lit 2))) ]
       Nothing

public export
sunscour : Card
sunscour =
  Macros.card "Sunscour"
       (Just [Macros.generic 5, Macros.pip White, Macros.pip White]) []
       (MkTypeLine [] [Sorcery])
       [ Static (AltCost This (Just (Do (Macros.exile You
                   (Macros.counted (Macros.exactly 2)
                                 (And [ColorIs White,
                                       InZone (Macros.handOf You)]))))))
       , Spell (Macros.destroy (Macros.allOf Macros.creature)) ]
       Nothing

public export
admiralsOrder : Card
admiralsOrder =
  Macros.card "Admiral's Order"
       (Just [Macros.generic 1, Macros.pip Blue, Macros.pip Blue]) []
       (MkTypeLine [] [Instant])
       [ Static (Macros.asLongAs
                   (Macros.happened AttackDeclaration You Lookback.ThisTurn)
                   (AltCost This (Just (Mana [Macros.pip Blue]))))
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
                      [ Macros.exists (And [Macros.land, HasSubtype (landType "Plains"),
                                     HasPossessor ControllerAx Macros.anOpponent])
                      , Macros.exists (And [Macros.land, HasSubtype (landType "Swamp"),
                                     HasPossessor ControllerAx You]) ])
                   (AltCost This Nothing))
       , Spell (Continuously {ts = StaticFirstDone}
                  (Gets (Macros.allOf Macros.creature) (PtDown (Lit 2)) (PtDown (Lit 2)))
                  (Just Macros.untilEndOfTurn)) ]
       Nothing

public export
kyrenLegate : Card
kyrenLegate =
  Macros.card "Kyren Legate" (Just [Macros.generic 1, Macros.pip Red]) []
       (MkTypeLine [creatureType "Goblin"] [Creature])
       [ Static (Macros.asLongAs
                   (AndCond
                      [ Macros.exists (And [Macros.land, HasSubtype (landType "Plains"),
                                     HasPossessor ControllerAx Macros.anOpponent])
                      , Macros.exists (And [Macros.land, HasSubtype (landType "Mountain"),
                                     HasPossessor ControllerAx You]) ])
                   (AltCost This Nothing))
       , Macros.keyword "Haste" ]
       (Just (1, 1))

public export
rouse : Card
rouse =
  Macros.card "Rouse" (Just [Macros.generic 1, Macros.pip Black]) []
       (MkTypeLine [] [Instant])
       [ Static (Macros.asLongAs
                   (Macros.exists (And [Macros.land, HasSubtype (landType "Swamp"), HasPossessor ControllerAx You]))
                   (AltCost This (Just (Do (ChangeLife You (Down (Lit 2)))))))
       , Spell (Continuously {ts = StaticFirstDone}
                  (Gets (Macros.target Macros.creature) (PtUp (Lit 2)) (PtUp (Lit 0)))
                  (Just Macros.untilEndOfTurn)) ]
       Nothing

public export
demonOfDeathsGate : Card
demonOfDeathsGate =
  Macros.card "Demon of Death's Gate"
       (Just [Macros.generic 6, Macros.pip Black, Macros.pip Black, Macros.pip Black]) []
       (MkTypeLine [creatureType "Demon"] [Creature])
       [ Static (AltCost This (Just (Compound
                   [ Do (ChangeLife You (Down (Lit 6)))
                   , Do (Macros.sacrifice You
                           (Macros.counted (Macros.exactly 3)
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
       [ Static (AltCost This (Just (Do (Macros.move
                   (Macros.counted (Macros.exactly 3)
                                 (And [Macros.land, HasSubtype (landType "Island"),
                                       HasPossessor ControllerAx You]))
                   Macros.handZ))))
       , Spell (Macros.counterSpell (Macros.target Macros.spell)) ]
       Nothing

public export
theLadyOfOtariaLine : StaticEffect []
theLadyOfOtariaLine =
  AltCost This (Just (Do (SetStatus Tapped
            (Macros.counted (Macros.exactly 3)
                          (And [Macros.creature, HasSubtype (creatureType "Dwarf"),
                                HasPossessor ControllerAx You, HasStatus Untapped])))))

public export
shiningShoal : Card
shiningShoal =
  Macros.card "Shining Shoal"
       (Just [Variable, Macros.pip White, Macros.pip White]) []
       (MkTypeLine [spellType "Arcane"] [Instant])
       [ Static (AltCost This (Just (Do (Macros.exile You
                   (Macros.a (And [ColorIs White,
                                   Compare [CharAxis ManaValue] Eq (LetterVal X),
                                   InZone (Macros.handOf You)]))))))
       , Spell (Continuously {ts = StaticFirstDone}
                  (Redirects AnyDamage (DealtBy (Macros.aYourChoice Macros.source))
                             (Macros.shieldingIt
                                (Macros.youAnd (Macros.allOf (And [HasType Creature,
                                                     HasPossessor ControllerAx You]))))
                             (Shield (LetterVal X))
                             (Macros.target Macros.anyTarget) Repeatedly)
                  (Just Macros.thisTurn)) ]
       Nothing

public export
disruptingShoal : Card
disruptingShoal =
  Macros.card "Disrupting Shoal"
       (Just [Variable, Macros.pip Blue, Macros.pip Blue]) []
       (MkTypeLine [spellType "Arcane"] [Instant])
       [ Static (AltCost This (Just (Do (Macros.exile You
                   (Macros.a (And [ColorIs Blue,
                                   Compare [CharAxis ManaValue] Eq (LetterVal X),
                                   InZone (Macros.handOf You)]))))))
       , Spell (OnlyIf (Macros.counterSpell (Macros.target Macros.spell))
                       (CompareAmt (Macros.manaValueOf It) Eq (LetterVal X)) Nothing) ]
       Nothing

||| Mana Leak
public export
manaLeak : Card
manaLeak =
  Macros.card "Mana Leak" (Just [Macros.generic 1, Macros.pip Blue]) []
       (MkTypeLine [] [Instant])
       [ Spell (Unless (Macros.counterSpell (Macros.target Macros.spell))
                       (Macros.controllerOf It)
                       (Mana [Macros.generic 3])) ]
       Nothing

||| Rhystic Study
public export
rhysticStudy : Card
rhysticStudy =
  Macros.card "Rhystic Study"
       (Just [Macros.generic 2, Macros.pip Blue]) []
       (MkTypeLine [] [Enchantment])
       [ Macros.triggered Whenever
           (Casts Macros.anOpponent (Macros.a Macros.spell) Nothing)
           (Unless (Macros.may You (Macros.draw You (Lit 1)))
                          (That PlayerW)
                          (Mana [Macros.generic 1])) ]
       Nothing

public export
blazingShoal : Card
blazingShoal =
  Macros.card "Blazing Shoal"
       (Just [Variable, Macros.pip Red, Macros.pip Red]) []
       (MkTypeLine [spellType "Arcane"] [Instant])
       [ Static (AltCost This (Just (Do (Macros.exile You
                   (Macros.a (And [ColorIs Red,
                                   Compare [CharAxis ManaValue] Eq (LetterVal X),
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
       [ Static (AltCost This (Just (Do (Macros.exile You
                   (Macros.a (And [ColorIs Black,
                                   Compare [CharAxis ManaValue] Eq (LetterVal X),
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
       [ Static (AltCost This (Just (Do (Macros.exile You
                   (Macros.a (And [ColorIs Green,
                                   Compare [CharAxis ManaValue] Eq (LetterVal X),
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
                                       Compare [CharAxis ManaValue] Eq (Lit 2)]))) ]
       Nothing

public export
isolate : Card
isolate =
  Macros.card "Isolate" (Just [Macros.pip White]) []
       (MkTypeLine [] [Instant])
       [ Spell (Macros.exile You
                  (Macros.target (And [Permanent,
                                       Compare [CharAxis ManaValue] Eq (Lit 1)]))) ]
       Nothing

public export
disembowel : Card
disembowel =
  Macros.card "Disembowel" (Just [Variable, Macros.pip Black]) []
       (MkTypeLine [] [Instant])
       [ Spell (Macros.destroy
                  (Macros.target (And [Macros.creature,
                                       Compare [CharAxis ManaValue] Eq (LetterVal X)]))) ]
       Nothing

public export
repeal : Card
repeal =
  Macros.card "Repeal" (Just [Variable, Macros.pip Blue]) []
       (MkTypeLine [] [Instant])
       [ Spell (Sequentially
                  [ Macros.move (Macros.target (And [Not Macros.land, Permanent,
                                              Compare [CharAxis ManaValue] Eq (LetterVal X)]))
                                Macros.handZ
                  , Draw You (Lit 1) ]) ]
       Nothing

public export
entrancingMelody : Card
entrancingMelody =
  Macros.card "Entrancing Melody"
       (Just [Variable, Macros.pip Blue, Macros.pip Blue]) []
       (MkTypeLine [] [Sorcery])
       [ Spell (Continuously {ts = StaticFirstDone}
                  (GainsControl You
                     (Macros.target (And [Macros.creature,
                                          Compare [CharAxis ManaValue] Eq (LetterVal X)])))
                  Nothing) ]
       Nothing

public export
ratchetBomb : Card
ratchetBomb =
  Macros.card "Ratchet Bomb" (Just [Macros.generic 2]) []
       (MkTypeLine [] [Artifact])
       [ Macros.activated TapSymbol
           (PutCounters (Lit 1) (PrintedKind (Named "Charge")) Macros.thisArtifact)
       , Macros.activated (Compound [TapSymbol,
                              Do (Macros.sacrifice You Macros.thisArtifact)])
           (Macros.destroy (Macros.each (And [Not Macros.land, Permanent,
                                       Compare [CharAxis ManaValue] Eq
                                         (CountersOn (Named "Charge") Macros.thisArtifact)]))) ]
       Nothing

public export
entDraughtBasinAbility : Ability
entDraughtBasinAbility =
  Macros.activated (Compound [Mana [Variable], TapSymbol])
                   (PutCounters (Lit 1) (PrintedKind Macros.plusOnePlusOne)
               (Macros.target (And [Macros.creatureYouControl,
                                    Compare [CharAxis Power] Eq (LetterVal X)])))

public export
sarkhansUnsealingLine : Ability
sarkhansUnsealingLine =
  Macros.triggered Whenever
    (Casts You (Macros.a (And [Macros.creature, Macros.spell,
                               Or [Compare [CharAxis Power] Eq (Lit 4),
                                   Compare [CharAxis Power] Eq (Lit 5),
                                   Compare [CharAxis Power] Eq (Lit 6)]])) Nothing)
    (DealDamage Macros.thisEnchantment (Lit 4) (Macros.target Macros.anyTarget))

||| Savage Swipe, both sentences
public export
savageSwipeLine : Effect []
savageSwipeLine =
  Sequentially
    [ OnlyIf (Macros.gets (Macros.target Macros.creatureYouControl) (PtUp (Lit 2))
                          (PtUp (Lit 2)) (Just Macros.untilEndOfTurn))
             (CompareAmt (Macros.powerOf It) Eq (Lit 2)) Nothing
    , Fights It (Macros.target Macros.creatureYouDontControl) ]

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

public export
matopiGolem : Card
matopiGolem =
  Macros.card "Matopi Golem" (Just [Macros.generic 5]) []
       (MkTypeLine [creatureType "Golem"] [Artifact, Creature])
       [ Macros.activated (Mana [Macros.generic 1])
                          (ThisWay (Regenerate Macros.thisCreature)
                            (Regenerates Macros.thisCreature)
                            (PutCounters (Lit 1) (PrintedKind Macros.minusOneMinusOne) It)) ]
       (Just (3, 3))

public export
infernoOfTheStarMounts : Card
infernoOfTheStarMounts =
  Macros.card "Inferno of the Star Mounts"
       (Just [Macros.generic 4, Macros.pip Red, Macros.pip Red]) [Legendary]
       (MkTypeLine [creatureType "Dragon"] [Creature])
       [ Static (Macros.objectCant "Counter" This)
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
                "Regenerate" (Macros.itVerbed "Destroy")) ]
       Nothing

public export
snuffOut : Card
snuffOut =
  Macros.card "Snuff Out" (Just [Macros.generic 3, Macros.pip Black]) []
       (MkTypeLine [] [Instant])
       [ Static (Macros.asLongAs
                   (Macros.exists (And [Macros.land, HasSubtype (landType "Swamp"),
                                 HasPossessor ControllerAx You]))
                   (AltCost This (Just (Do (ChangeLife You (Down (Lit 4)))))))
       , Spell (CantBe (Macros.destroy (Macros.target
                  (And [Macros.creature, Not (ColorIs Black)])))
                "Regenerate" (Macros.itVerbed "Destroy")) ]
       Nothing

public export
wrathOfGod : Card
wrathOfGod =
  Macros.card "Wrath of God"
       (Just [Macros.generic 2, Macros.pip White, Macros.pip White]) []
       (MkTypeLine [] [Sorcery])
       [ Spell (CantBe (Macros.destroy (Macros.allOf Macros.creature))
                       "Regenerate" Them) ]
       Nothing

public export
damnation : Card
damnation =
  Macros.card "Damnation"
       (Just [Macros.generic 2, Macros.pip Black, Macros.pip Black]) []
       (MkTypeLine [] [Sorcery])
       [ Spell (CantBe (Macros.destroy (Macros.allOf Macros.creature))
                       "Regenerate" Them) ]
       Nothing

public export
phyrexianRebirth : Card
phyrexianRebirth =
  Macros.card "Phyrexian Rebirth"
       (Just [Macros.generic 4, Macros.pip White, Macros.pip White]) []
       (MkTypeLine [] [Sorcery])
       [ Spell (Sequentially
                  [ Macros.destroy (Macros.allOf Macros.creature)
                  , Create You (Lit 1)
                           (TokenWritten
                              (MkToken (Just (LetterVal X ** LetterVal X)) []
                                       (MkTypeLine [creatureType "Phyrexian", creatureType "Horror"]
                                                   [Artifact, Creature])
                                       [] Nothing))
                           []
                  , Define X (CountOf
                                (Macros.thoseVerbedThisWay "Destroy"
                                                           (TypeW Creature))) ]) ]
       Nothing

public export
incinerate : Card
incinerate =
  Macros.card "Incinerate" (Just [Macros.generic 1, Macros.pip Red]) []
       (MkTypeLine [] [Instant])
       [ Spell (Sequentially
                  [ DealDamage This (Lit 3) (Macros.target Macros.anyTarget)
                  , Continuously {ts = StaticFirstDone}
                      (Macros.objectCant "Regenerate"
                         (Macros.a (And [Macros.creature,
                                         HappenedTo DamageTaken Lookback.ThisWay
                                                    Nothing])))
                      (Just Macros.thisTurn) ]) ]
       Nothing

public export
hurrJackalAbility : Ability
hurrJackalAbility =
  Macros.activated TapSymbol
    (Continuously {ts = StaticFirstDone} (Macros.objectCant "Regenerate" (Macros.target Macros.creature))
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
       [ Static (Macros.entersWithAdditionalCounters (Macros.each (And [Macros.creature, HasSubtype (creatureType "Warrior"), HasPossessor ControllerAx You,
                                                                 OtherThan Macros.thisCreature]))
                                                     (Lit 1)
                                                     Macros.plusOnePlusOne)
       , Static (Gains (Macros.each (And [Macros.creature, HasPossessor ControllerAx You,
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
       , Static (Macros.entersWithAdditionalCounters (Macros.each (And [Macros.creature, HasSubtype (creatureType "Rogue"), HasPossessor ControllerAx You,
                                                                 OtherThan Macros.thisCreature]))
                                                     (Lit 1)
                                                     Macros.plusOnePlusOne)
       , Macros.triggered Whenever
                          (Macros.dealsCombatDamage
                      (Macros.each (And [Macros.creature, HasPossessor ControllerAx You,
                                  HasCounters (Just Macros.plusOnePlusOne)]))
                      (Macros.a AnyPlayer))
                          ((Macros.discard (That PlayerW) (Macros.a (InZone Macros.handZ)))) ]
       (Just (1, 1))

public export
crumblingAshes : Card
crumblingAshes =
  Macros.card "Crumbling Ashes"
       (Just [Macros.generic 1, Macros.pip Black]) []
       (MkTypeLine [] [Enchantment])
       [ Macros.triggered At (BeginningOf Upkeep (Macros.yours))
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
                  (Macros.allOf (And [Macros.creature, Not (HasCounters Nothing)]))) ]
       Nothing

public export
hazardousConditions : Card
hazardousConditions =
  Macros.card "Hazardous Conditions"
       (Just [Macros.generic 2, Macros.pip Black, Macros.pip Green]) []
       (MkTypeLine [] [Sorcery])
       [ Spell (Macros.gets (Macros.allOf (And [Macros.creature, Not (HasCounters Nothing)]))
                            (PtDown (Lit 2)) (PtDown (Lit 2))
                            (Just Macros.untilEndOfTurn)) ]
       Nothing

public export
hunterOfEyeblights : Card
hunterOfEyeblights =
  Macros.card "Hunter of Eyeblights"
       (Just [Macros.generic 3, Macros.pip Black, Macros.pip Black]) []
       (MkTypeLine [creatureType "Elf", creatureType "Assassin"] [Creature])
       [ Macros.triggered When (Enters Macros.thisCreature Nothing)
                          (PutCounters (Lit 1) (PrintedKind Macros.plusOnePlusOne)
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
       [ Macros.triggered When (Enters Macros.thisCreature Nothing)
                          (PutCounters (Lit 1) (PrintedKind Macros.plusOnePlusOne)
                      (Macros.target Macros.creatureYouControl))
       , Static (Gains (Macros.each (And [Macros.creature, HasPossessor ControllerAx You,
                                   HasCounters (Just Macros.plusOnePlusOne)]))
                       (Macros.keyword "Trample")) ]
       (Just (2, 1))



public export
glacialChasm : Card
glacialChasm =
  Macros.card "Glacial Chasm" Nothing []
       (MkTypeLine [] [Land])
       [ Macros.cumulativeUpkeep (Macros.payLife You 2)
       , Macros.triggered When (Enters Macros.thisLand Nothing)
                          (Macros.sacrifice You (Macros.a Macros.land))
       , Static (Macros.deontic (Macros.allOf Macros.creatureYouControl) Forbid ["Attack"] Agent NoDeonticPatient)
       , Static (Prevents AnyDamage Unattributed (Macros.shieldingIt You) CutAll Repeatedly Nothing) ]
       Nothing

public export
aboroth : Card
aboroth =
  Macros.card "Aboroth"
       (Just [Macros.generic 4, Macros.pip Green, Macros.pip Green]) []
       (MkTypeLine [creatureType "Elemental"] [Creature])
       [ Macros.cumulativeUpkeep
                               (Do (PutCounters (Lit 1) (PrintedKind Macros.minusOneMinusOne)
                                      Macros.thisCreature)) ]
       (Just (9, 9))

public export
shelteringAncient : Card
shelteringAncient =
  Macros.card "Sheltering Ancient"
       (Just [Macros.generic 1, Macros.pip Green]) []
       (MkTypeLine [creatureType "Treefolk"] [Creature])
       [ Macros.keyword "Trample"
       , Macros.cumulativeUpkeep
                               (Do (PutCounters (Lit 1) (PrintedKind Macros.plusOnePlusOne)
                                      (Macros.a (And [Macros.creature,
                                                      HasPossessor ControllerAx Macros.anOpponent])))) ]
       (Just (5, 5))

public export
polarKraken : Card
polarKraken =
  Macros.card "Polar Kraken"
       (Just [Macros.generic 8, Macros.pip Blue, Macros.pip Blue, Macros.pip Blue]) []
       (MkTypeLine [creatureType "Kraken"] [Creature])
       [ Macros.keyword "Trample"
       , Static (Macros.entersTapped Macros.thisCreature)
       , Macros.cumulativeUpkeep (Do (Macros.sacrifice You (Macros.a Macros.land))) ]
       (Just (11, 11))

public export
yavimayaAnts : Card
yavimayaAnts =
  Macros.card "Yavimaya Ants"
       (Just [Macros.generic 2, Macros.pip Green, Macros.pip Green]) []
       (MkTypeLine [creatureType "Insect"] [Creature])
       [ Macros.keyword "Trample"
       , Macros.keyword "Haste"
       , Macros.cumulativeUpkeep (Mana [Macros.pip Green, Macros.pip Green]) ]
       (Just (5, 1))

public export
illusionaryForces : Card
illusionaryForces =
  Macros.card "Illusionary Forces"
       (Just [Macros.generic 3, Macros.pip Blue]) []
       (MkTypeLine [creatureType "Illusion"] [Creature])
       [ Macros.keyword "Flying"
       , Macros.cumulativeUpkeep (Mana [Macros.pip Blue]) ]
       (Just (4, 4))

public export
vexingSphinx : Card
vexingSphinx =
  Macros.card "Vexing Sphinx"
       (Just [Macros.generic 1, Macros.pip Blue, Macros.pip Blue]) []
       (MkTypeLine [creatureType "Sphinx"] [Creature])
       [ Macros.keyword "Flying"
       , Macros.cumulativeUpkeep (Do ((Macros.discard You (Macros.a (InZone Macros.handZ)))))
       , Macros.triggered When (Dies Macros.thisCreature)
                          (Draw You (CountersOn (Named "Age") It)) ]
       (Just (4, 4))

public export
manaChains : Card
manaChains =
  Macros.card "Mana Chains" (Just [Macros.pip Blue]) []
       (MkTypeLine [enchantmentType "Aura"] [Enchantment])
       [ Macros.keywordSubject "Enchant" Macros.creature
       , Static (Gains (AttachHost Enchanted (TypeW Creature))
                       (Macros.cumulativeUpkeep (Mana [Macros.generic 1]))) ]
       Nothing

public export
coverOfWinterPut : Ability
coverOfWinterPut =
  Macros.activated (Mana [SnowMana])
                   (PutCounters (Lit 1) (PrintedKind (Named "Age")) Macros.thisEnchantment)



public export
odricLunarchMarshal : Card
odricLunarchMarshal =
  Macros.card "Odric, Lunarch Marshal"
       (Just [Macros.generic 3, Macros.pip White]) [Legendary]
       (MkTypeLine [creatureType "Human", creatureType "Soldier"] [Creature])
       [ AlsoForKeywords
           (Macros.triggeredIf At
                               (BeginningOf Combat (Macros.eachPlayers))
                               (Macros.exists (And [Macros.creature, HasPossessor ControllerAx You,
                                             HasKeyword (TheKeyword "FirstStrike")]))
                               (Continuously {ts = StaticFirstDone}
                                  (Gains (Macros.allOf Macros.creatureYouControl)
                                         (Macros.keyword "FirstStrike"))
                                  (Just Macros.untilEndOfTurn)))
           (map TheKeyword
              ["Flying", "Deathtouch", "DoubleStrike", "Haste", "Hexproof", "Indestructible",
               "Lifelink", "Menace", "Reach", "Skulk", "Trample", "Vigilance"]) ]
       (Just (3, 3))

public export
bleedingEffect : Card
bleedingEffect =
  Macros.card "Bleeding Effect"
       (Just [Macros.generic 2, Macros.pip White, Macros.pip Black]) []
       (MkTypeLine [] [Enchantment])
       [ AlsoForKeywords
           (Macros.triggeredIf At
                               (BeginningOf Combat (Macros.yours))
                               (Macros.exists (And [Macros.creature,
                                             InZone (Macros.graveyardOf You),
                                             HasKeyword (TheKeyword "Flying")]))
                               (Continuously {ts = StaticFirstDone}
                                  (Gains (Macros.allOf Macros.creatureYouControl)
                                         (Macros.keyword "Flying"))
                                  (Just Macros.untilEndOfTurn)))
           (map TheKeyword
              ["FirstStrike", "DoubleStrike", "Deathtouch", "Hexproof", "Indestructible",
               "Lifelink", "Menace", "Reach", "Trample", "Vigilance"]) ]
       Nothing

public export
urborgScavengers : Card
urborgScavengers =
  Macros.card "Urborg Scavengers"
       (Just [Macros.generic 2, Macros.pip Black]) []
       (MkTypeLine [creatureType "Spirit"] [Creature])
       [ Macros.triggeredOr Whenever
                            (Enters Macros.thisCreature Nothing)
                            [Macros.attacks Macros.thisCreature]
                            (Sequentially
                               [ Macros.exile You (Macros.target (InZone Macros.graveyardZ))
                               , PutCounters (Lit 1) (PrintedKind Macros.plusOnePlusOne) Macros.thisCreature ])
       , AlsoForKeywords
           (Static (Macros.asLongAs
                      (Macros.exists (And [ExiledWith Macros.thisCreature, HasKeyword (TheKeyword "Flying")]))
                      (Gains Macros.thisCreature (Macros.keyword "Flying"))))
           (map TheKeyword
              ["FirstStrike", "DoubleStrike", "Deathtouch", "Haste", "Hexproof", "Indestructible",
               "Lifelink", "Menace", "Reach", "Trample", "Vigilance"]) ]
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
           (PutCounters (Lit 1) (PrintedKind Macros.plusOnePlusOne) Macros.thisCreature) ]
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
       , Macros.triggered When (Enters Macros.thisCreature Nothing)
           (OnlyIf (Macros.draw You (Lit 1))
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
       , Macros.triggered When (Enters Macros.thisCreature Nothing)
           (SetStatus Untapped
              (Macros.each (And [HasSubtype (creatureType "Merfolk"), HasPossessor ControllerAx You,
                          OtherThan Macros.thisCreature])))
       , Static (Macros.asLongAs
           (Macros.happenedInvolving AttackDeclaration
                                     You
                                     Lookback.ThisTurn
                                     (Macros.counted (Macros.atLeast 3)
                                        (HasSubtype (creatureType "Merfolk"))))
           (Gets (Macros.allOf (And [HasSubtype (creatureType "Merfolk"), HasPossessor ControllerAx You]))
                 (PtUp (Lit 1)) (PtUp (Lit 0)))) ]
       (Just (2, 2))

public export
idolOfOblivion : Card
idolOfOblivion =
  Macros.card "Idol of Oblivion" (Just [Macros.generic 2]) []
       (MkTypeLine [] [Artifact])
       [ Macros.activatedOnlyIf TapSymbol
                                (Macros.draw You (Lit 1))
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
                                     [ Continuously {ts = StaticFirstDone} (Becomes Macros.thisCreature Sets (Bundle (MkToken Nothing [] (Macros.subtypesOnly [creatureType "Werewolf"]) [] Nothing) Nothing)) Nothing
                                     , PutCounters (Lit 2) (PrintedKind Macros.plusOnePlusOne) It
                                     , (Macros.draw You (Lit 1)) ])
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
                                      CastFrom (Macros.graveyardOf You)])) Nothing)
           (Sequentially
              [ Macros.searchLibraryOrGraveyard
                  (And [Macros.artifact,
                        Compare [CharAxis ManaValue] AtMost (Lit 2)]) ]) ]
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
                  (Macros.a (And [Macros.spell, CastFrom Macros.graveyardZ])) Nothing)
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
           , (Macros.draw You (Lit 1)) ]) ]
       Nothing

public export
coalStoker : Card
coalStoker =
  Macros.card "Coal Stoker" (Just [Macros.generic 3, Macros.pip Red]) []
       (MkTypeLine [creatureType "Elemental"] [Creature])
       [ Macros.triggeredIf When
                            (Enters Macros.thisCreature Nothing)
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
                                      Not (CastFrom (Macros.handOf You))])) Nothing)
           (Sequentially
              [ Macros.searchLibraryOrGraveyard
                  (And [Macros.artifact,
                        Compare [CharAxis ManaValue] AtMost (Lit 2)]) ]) ]
       (Just (2, 2))

public export
patricianGeist : Card
patricianGeist =
  Macros.card "Patrician Geist" (Just [Macros.generic 2, Macros.pip Blue]) []
       (MkTypeLine [creatureType "Spirit", creatureType "Knight"] [Creature])
       [ Macros.keyword "Flying"
       , Static (Gets (Macros.allOf (And [HasSubtype (creatureType "Spirit"), HasPossessor ControllerAx You,
                                   OtherThan Macros.thisCreature]))
                      (PtUp (Lit 1)) (PtUp (Lit 1)))
       , Static (CostsToCast
                   (Macros.allOf (And [Macros.spell, CastBy You,
                                CastFrom (Macros.graveyardOf You)]))
                   (CostLess (Lit 1) Nothing)) ]
       (Just (2, 2))


public export
oust : Card
oust =
  Macros.card "Oust" (Just [Macros.pip White]) [] (MkTypeLine [] [Sorcery])
       [ Spell (Sequentially
                  [ Macros.move (Macros.target Macros.creature) (Macros.nthFromTop (Nth 2))
                  , Macros.gainsLife (Macros.controllerOf It) (Lit 3) ]) ]
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
       , Macros.triggered Whenever (Casts You (Macros.a Macros.spell) Nothing)
                          (Sequentially
                      [ Macros.gets (Macros.each Macros.creatureYouControl)
                                    (PtUp (Lit 1)) (PtUp (Lit 0))
                                    (Just Macros.untilEndOfTurn)
                      , Macros.scry You (Lit 1) ])
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
                            (Enters Macros.thisCreature Nothing)
                            (AndCond [ Macros.exists (And [Macros.artifact, HasPossessor ControllerAx You])
                                     , Macros.exists (And [Macros.enchantment, HasPossessor ControllerAx You]) ])
                            (Sequentially [ Draw You (Lit 1), Macros.gainsLife You (Lit 1) ]) ]
       (Just (3, 4))

public export
bloodfireEnforcers : Card
bloodfireEnforcers =
  Macros.card "Bloodfire Enforcers" (Just [Macros.generic 3, Macros.pip Red]) []
       (MkTypeLine [creatureType "Human", creatureType "Monk"] [Creature])
       [ Static (Macros.asLongAs
                   (AndCond [ Macros.exists (And [Macros.instant, InZone (Macros.graveyardOf You)])
                            , Macros.exists (And [Macros.sorcery, InZone (Macros.graveyardOf You)]) ])
                   (AndAlso [ Gains Macros.thisCreature (Macros.keyword "FirstStrike")
                            , Gains It (Macros.keyword "Trample") ])) ]
       (Just (5, 2))

public export
helmOfPossession : Card
helmOfPossession =
  Macros.card "Helm of Possession" (Just [Macros.generic 4]) []
       (MkTypeLine [] [Artifact])
       [ Static (Macros.mayDeclineUntap Macros.thisArtifact (Just You))
       , Macros.activated (Compound [Mana [Macros.generic 2], TapSymbol,
                              Do (Macros.sacrifice You (Macros.a Macros.creature))])
                          (Macros.gainControl You (Macros.target Macros.creature)
                      (Just (ForAsLongAs
                               (AndCond [ Matches Macros.thisArtifact (HasPossessor ControllerAx You)
                                        , Matches Macros.thisArtifact Macros.tapped ])))) ]
       Nothing


public export
override : Card
override =
  Macros.card "Override" (Just [Macros.generic 2, Macros.pip Blue]) []
       (MkTypeLine [] [Instant])
       [ Spell ((May (Macros.controllerOf (Macros.target Macros.spell)) (Pay They (ScaledMana GenericUnit (Macros.forEach
                                            (And [Macros.artifact, HasPossessor ControllerAx You]))) PaidOnce) Nothing (Just (Macros.counterSpell It)))) ]
       Nothing

public export
rakshasasDisdain : Card
rakshasasDisdain =
  Macros.card "Rakshasa's Disdain" (Just [Macros.generic 2, Macros.pip Blue]) []
       (MkTypeLine [] [Instant])
       [ Spell ((May (Macros.controllerOf (Macros.target Macros.spell)) (Pay They (ScaledMana GenericUnit (Macros.forEach
                                            (InZone (Macros.graveyardOf You)))) PaidOnce) Nothing (Just (Macros.counterSpell It)))) ]
       Nothing

public export
fettergeist : Card
fettergeist =
  Macros.card "Fettergeist" (Just [Macros.generic 2, Macros.pip Blue]) []
       (MkTypeLine [creatureType "Spirit"] [Creature])
       [ Macros.keyword "Flying"
       , Macros.triggered At (BeginningOf Upkeep (Macros.yours))
           ((May You (Pay You (ScaledMana GenericUnit (Macros.forEach
                          (And [Macros.creature, HasPossessor ControllerAx You,
                                OtherThan Macros.thisCreature]))) PaidOnce) Nothing (Just (Macros.sacrifice You Macros.thisCreature)))) ]
       (Just (3, 4))

public export
megatherium : Card
megatherium =
  Macros.card "Megatherium" (Just [Macros.generic 2, Macros.pip Green]) []
       (MkTypeLine [creatureType "Beast"] [Creature])
       [ Macros.keyword "Trample"
       , Macros.triggered When (Enters Macros.thisCreature Nothing)
           ((May You (Pay You (ScaledMana GenericUnit (Macros.forEach (InZone (Macros.handOf You)))) PaidOnce) Nothing (Just (Macros.sacrifice You Macros.thisCreature)))) ]
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
  Sequentially [Macros.move (Macros.allOf (And [Macros.creature, InZone (Macros.graveyardOf You)]))
                            Macros.battlefieldZ,
                Macros.becomesAs Them
                                 (MkToken (Just (Lit 1 ** Lit 1)) [] (MkTypeLine [creatureType "Spirit"] [])
                                          [Macros.keyword "Flying"] Nothing)
                                 Nothing,
                Macros.exile You This]

public export
everAfter : Effect []
everAfter =
  Sequentially [Macros.move (Macros.targets (Macros.upTo 2) (And [Macros.creature, InZone (Macros.graveyardOf You)]))
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
                                                                                  (Macros.ownerOf It)
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
       [ Macros.triggered When (Enters (Macros.a Macros.creatureYouControl) Nothing)
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
  ForEachOf (Macros.each Macros.creature)
            ((May (Macros.controllerOf It) (Pay They (Do (ChangeLife They (Down (LetterVal X)))) PaidOnce) Nothing (Just (Macros.sacrifice They It))))

public export
fadeAway : Effect []
fadeAway =
  ForEachOf (Macros.each Macros.creature)
            ((May (Macros.controllerOf It) (Pay They (Mana [Macros.generic 1]) PaidOnce) Nothing (Just (Macros.sacrifice They (Macros.a Permanent)))))

public export
martyrsCry : Effect []
martyrsCry =
  Sequentially [Macros.exile You (Macros.allOf (And [Macros.creature, ColorIs White])),
                ForEachOf (Macros.thoseVerbed "Exile" (TypeW Creature))
                          (Draw (Macros.controllerOf It) (Lit 1))]

||| Hate Mirage's middle two sentences
public export
hateMirageTokens : Effect []
hateMirageTokens =
  Sequentially
    [ ForEachOf (Macros.targets (Macros.upTo 2) Macros.creatureYouDontControl)
                (Create You (Lit 1) (TokenCopyOf It []) [])
    , Macros.gains (Those TokenW) (Macros.keyword "Haste") Nothing ]

public export
descentOfTheDragons : Effect []
descentOfTheDragons =
  Sequentially [Macros.destroy (Macros.targets Macros.anyNumber Macros.creature),
                ForEachOf (Macros.thoseVerbedThisWay "Destroy" (TypeW Creature))
                          (Create (Macros.controllerOf It) (Lit 1)
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
           (ForEachOf (Macros.each (And [Macros.creature, Attacking,
                                  Not (HasKeyword (TheKeyword "Flying"))]))
                      ((May (Macros.controllerOf It) (Pay They (Mana [Macros.generic 1]) PaidOnce) Nothing (Just (Macros.gains
                                         (Macros.allOf (And [Macros.creature, HasPossessor ControllerAx You,
                                                      CombatRel BlockerOf (That (TypeW Creature))]))
                                         (Macros.keyword "FirstStrike")
                                         (Just Macros.untilEndOfTurn)))))) ]
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
              [ Macros.exile You Macros.topCard
              , If (Macros.itsA Permanent)
                   ((May You (Macros.putOntoBattlefield It) (Just (Repeat Again)) Nothing))
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
                           (Macros.countOf (And [Macros.land, HasPossessor ControllerAx You])))
       , Macros.triggered When (Enters Macros.thisCreature Nothing)
           ((May You (Macros.putOntoBattlefieldTapped
                 (Macros.a (And [Macros.land, InZone (Macros.handOf You)]))) (Just (Sequentially [(Macros.draw You (Lit 1)), Repeat Again])) Nothing)) ]
       (Just (PtBox PrintedStar PrintedStar))

public export
zimoneAndDina : Ability
zimoneAndDina =
  Macros.activated (Compound [TapSymbol,
                       Do (Macros.sacrifice You
                             (Macros.a (Macros.otherCreature Macros.thisCreature)))])
    (Sequentially
       [ (Macros.draw You (Lit 1))
       , Macros.may You (Macros.putOntoBattlefieldTapped
                           (Macros.a (And [Macros.land,
                                           InZone (Macros.handOf You)])))
       , If (CompareAmt (Macros.countOf (And [Macros.land, HasPossessor ControllerAx You]))
                        AtLeast (Lit 8))
                   (Repeat (MoreTimes (Lit 1)))
                   Nothing ])

public export
anotherRound : Effect []
anotherRound =
  Sequentially [ Macros.exile You (Macros.counted Macros.anyNumber
                                 Macros.creatureYouControl)
               , Macros.putOntoBattlefield Them
               , Repeat (MoreTimes (LetterVal X)) ]

public export
shuFarmer : Card
shuFarmer =
  Macros.card "Shu Farmer" (Just [Macros.generic 1, Macros.pip White]) []
       (MkTypeLine [creatureType "Human"] [Creature])
       [ Macros.activatedOnlyDuring TapSymbol
                                    (Macros.gainsLife You (Lit 1))
                                    (BeforePoint AttackersDeclared (Just You)) ]
       (Just (1, 1))

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
                          (Continuously {ts = StaticFirstDone}
                      (AndAlso [ Gets (Macros.allOf Macros.creatureYouControl)
                                      (PtUp (Lit 2)) (PtUp (Lit 2))
                               , Gains Them (Macros.keyword "Vigilance") ])
                      (Just Macros.untilEndOfTurn)) ]
       Nothing

public export
chainVeilEndStep : Ability
chainVeilEndStep =
  Macros.triggeredIf At
                     (BeginningOf EndStep (Macros.yours))
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
                            (BeginningOf Upkeep (Macros.eachOpponents))
                            (CompareAmt (Macros.countOf (InZone (Macros.handOf (That PlayerW))))
                                        AtMost (Lit 1))
                            (Macros.losesLife They (Lit 3)) ]
       Nothing

public export
galvanicBlastLine : Effect []
galvanicBlastLine =
  InsteadOf (DealDamage This (Lit 2) (Macros.target Macros.anyTarget))
            (OnlyIf (DealDamage This (Lit 4) (Macros.thatJoin))
                    (CompareAmt (Macros.countOf (And [Macros.artifact, HasPossessor ControllerAx You]))
                                AtLeast (Lit 3))
                    Nothing)

public export
cryptLurker : Card
cryptLurker =
  Macros.card "Crypt Lurker" (Just [Macros.generic 3, Macros.pip Black]) []
       (MkTypeLine [creatureType "Horror"] [Creature])
       [ Macros.triggered When (Enters Macros.thisCreature Nothing)
           ((May You (Macros.chooseOne
                 [ Macros.sacrifice You (Macros.a Macros.creature)
                 , Macros.discard You (Macros.a (And [Macros.creature,
                                                       InZone Macros.handZ])) ]) (Just (Macros.draw You (Lit 1))) Nothing)) ]
       (Just (3, 4))

||| Peacekeeper
public export
peacekeeperCant : Ability
peacekeeperCant = Static (Macros.deontic (Macros.allOf Macros.creature) Forbid ["Attack"] Agent NoDeonticPatient)


||| Memoricide
public export
memoricideSearch : Effect []
memoricideSearch =
  Sequentially [ Macros.choose (Macros.a (Macros.qualityFrom CardName
                                     (NameOfCard (Not (HasType Land)))))
               , Macros.searchZonesOf (Macros.target AnyPlayer) (Named ChosenName)
               , Shuffle (That PlayerW) ]

||| Eradicate
public export
eradicateSearch : Effect []
eradicateSearch =
  Sequentially [ Macros.exile You (Macros.target (And [Macros.creature,
                                                   Not (ColorIs Black)]))
               , Macros.searchZonesOf (Macros.controllerOf (That CardW))
                                      (Named (SameNameAs (That CardW)))
               , Shuffle (That PlayerW) ]

||| Deem Inferior
public export
deemInferior : Effect []
deemInferior =
  Macros.puts (Macros.ownerOf (Macros.target (And [Permanent, Not (HasType Land)])))
              It
              (Macros.nthFromTopOrBottomZ (Nth 2))

||| Lost Hours
public export
lostHoursPlacement : Effect []
lostHoursPlacement =
  Sequentially [ Macros.revealsTheirHand (Macros.target AnyPlayer)
               , Macros.choose (Macros.a (And [Not (HasType Land),
                                        InZone (Macros.handOf They)]))
               , Macros.puts (That PlayerW) It (Macros.nthFromTop (Nth 3)) ]

||| Aether Gust
public export
aetherGustPlacement : Effect []
aetherGustPlacement =
  Sequentially [ Macros.choose (Macros.target (And [Permanent, ColorIs Red]))
               , Macros.puts (Macros.ownerOf It) It (Macros.choiceOfTopOrBottom They) ]

||| Not Forgotten
public export
notForgottenPlacement : Effect []
notForgottenPlacement =
  Macros.move (Macros.target (InZone Macros.graveyardZ)) (Macros.choiceOfTopOrBottom You)

||| Write into Being
public export
writeIntoBeingPlacement : Effect []
writeIntoBeingPlacement =
  Sequentially [ Macros.lookAt (Macros.topCards 2)
               , Macros.exile You (Macros.oneOf Them)
               , Macros.move TheRest Macros.topOrBottomZ ]

||| Culling Mark
public export
cullingMark : Card
cullingMark =
  Macros.card "Culling Mark" (Just [Macros.generic 2, Macros.pip Green]) []
       (MkTypeLine [] [Sorcery])
       [ Spell (Continuously {ts = StaticFirstDone} (Macros.deontic (Macros.target Macros.creature)
                                      Require ["Block"] Agent NoDeonticPatient)
                             (Just Macros.thisTurn)) ]
       Nothing

||| Blazing Archon
public export
blazingArchonCant : Ability
blazingArchonCant =
  Static (Macros.deontic (Macros.allOf Macros.creature) Forbid ["Attack"] Agent (DefendingPlayer You))

||| Clergy of the Holy Nimbus
public export
clergyOfTheHolyNimbus : Ability
clergyOfTheHolyNimbus =
  Static (Intercepts (VerbedEvent Nothing "Destroy"
                                  (Just Macros.thisCreature) Nothing False)
                     [] Nothing
                     (Regenerate It) Repeatedly Nothing)

||| Rampant Frogantua
public export
rampantFrogantuaPump : Ability
rampantFrogantuaPump =
  Static (Gets Macros.thisCreature
               (PtUp (Macros.nForEach 10 (And [AnyPlayer, Macros.happenedTo GameLoss ThisGame])))
               (PtUp (Macros.nForEach 10 (And [AnyPlayer, Macros.happenedTo GameLoss ThisGame]))))

public export
goadedAttacksOther : Effect []
goadedAttacksOther =
  Continuously {ts = StaticFirstDone} (Macros.deontic (Macros.target Macros.creature) Require ["Attack"] Agent
                        (DefendingPlayer (Macros.a (And [AnyPlayer, OtherThan You]))))
               (Just Macros.untilYourNextTurn)


||| Fastbond
public export
fastbondLands : Ability
fastbondLands = Static (Macros.mayPlayAdditionalLands You Macros.anyNumber)

||| Furious Reprisal
public export
furiousReprisal : Effect []
furiousReprisal =
  DealDamage This (Lit 2) (EachOf (Macros.targets (Macros.exactly 2) Macros.anyTarget))

||| Maskwood Nexus
public export
maskwoodNexusTypes : Ability
maskwoodNexusTypes =
  Static (AlsoOffBattlefield
            (Becomes (Macros.allOf Macros.creatureYouControl) Adds (EveryTypeOf CreatureSpace)))

||| Mystical Tutor
public export
mysticalTutor : Effect []
mysticalTutor =
  Sequentially [ Macros.searchLibraryFor Macros.instantOrSorcery
               , Macros.revealCards It
               , Macros.shuffle
               , Macros.move It Macros.onTopZ ]

||| Demonic Tutor
public export
demonicTutor : Effect []
demonicTutor =
  Sequentially [ Macros.searchLibraryFor (And [])
               , Macros.move It Macros.handZ
               , Macros.shuffle ]

||| Thalia's Lancers
public export
thaliasLancersSearch : Effect []
thaliasLancersSearch =
  Sequentially [ Macros.searchLibraryFor (And [HasSupertype Legendary])
               , Macros.revealCards It
               , Macros.move It Macros.handZ
               , Macros.shuffle ]

||| Brimaz, King of Oreskos
public export
brimazAttackToken : Ability
brimazAttackToken =
  Macros.triggered Whenever (Macros.attacks Macros.thisCreature)
    (Create You (Lit 1)
                   (TokenWritten (MkToken (Just (Lit 1 ** Lit 1)) [White]
                                   (MkTypeLine [creatureType "Cat", creatureType "Soldier"] [Creature])
                                   [Macros.keyword "Vigilance"] Nothing))
                   [EntersAttacking NoDefender])

||| Luxior, Giada's Gift
public export
luxiorEquippedPermanent : Ability
luxiorEquippedPermanent =
  Static (Becomes (AttachHost Equipped PermanentW) Adds (Bundle (MkToken Nothing [] (MkTypeLine [] [Creature]) [] Nothing) Nothing))

public export
nahiriLoyaltyRead : Predicate [] Object
nahiriLoyaltyRead =
  And [ Macros.creature, InZone (Macros.graveyardOf You)
      , Compare [CharAxis ManaValue] Less (StatOf Loyalty This) ]

||| Nahiri, the Unforgiving's compleated reminder
public export
nahiriCompleatedEntry : Ability
nahiriCompleatedEntry =
  Static (Macros.entersWithFewerCounters Macros.thisPlaneswalker (Lit 2)
                                         (Named "Loyalty"))

||| Vivien's Talent
public export
viviensTalentTrigger : Ability
viviensTalentTrigger =
  Macros.triggered Whenever (Enters (Macros.a (And [Macros.nontoken,
                                             Macros.creatureYouControl])) Nothing)
    (PutCounters (Lit 1) (PrintedKind (Named "Loyalty"))
                 (AttachHost Enchanted (TypeW Planeswalker)))

public export
counterOnGraveyardCard : Effect []
counterOnGraveyardCard =
  PutCounters (Lit 1) (PrintedKind Macros.plusOnePlusOne)
              (Macros.target (And [Macros.creature,
                                   InZone (Macros.graveyardOf You)]))

||| Ascend's own reminder text
public export
ascendConferral : Effect []
ascendConferral = Macros.getsCitysBlessing

||| Saddle's expansion body
public export
saddleConferral : Effect []
saddleConferral = Macros.becomesSaddled

||| Bioessence Hydra
public export
bioessenceHydraTrigger : Ability
bioessenceHydraTrigger =
  Macros.triggered Whenever
    (Macros.manyCounterEvent CounterPut (Named "Loyalty")
                             (Macros.allOf (And [HasType Planeswalker, HasPossessor ControllerAx You])))
    (PutCounters ThatMuch (PrintedKind Macros.plusOnePlusOne) Macros.thisCreature)

||| Void Maw
public export
voidMawPutCost : Ability
voidMawPutCost =
  Macros.activated (Do (Macros.puts You (Macros.a (ExiledWith Macros.thisCreature))
                             Macros.graveyardZ))
                   (Macros.gets Macros.thisCreature (PtUp (Lit 2)) (PtUp (Lit 2))
                         (Just Macros.untilEndOfTurn))

||| Bullseye, Death Dealer
public export
bullseyeModalCost : Ability
bullseyeModalCost =
  Macros.activated (Compound [Mana [Macros.generic 3], TapSymbol,
                       Do (Macros.chooseOne
                             [Macros.sacrifice You (Macros.a Macros.artifact),
                              Macros.discard You
                                (Macros.a (And [Not Macros.land,
                                                InZone Macros.handZ]))])])
                   (DealDamage This (Lit 2) (Macros.target Macros.anyTarget))

||| Sedris, the Traitor King
public export
sedrisTheTraitorKing : Ability
sedrisTheTraitorKing =
  Static (Gains (Macros.each (And [Macros.creature, InZone (Macros.graveyardOf You)]))
                (Macros.keywordCosting "Unearth"
                   (Mana [Macros.generic 2, Macros.pip Black])))

||| Grixis
public export
grixis : Ability
grixis =
  Static (Gains (Macros.allOf (And [Or [ColorIs Blue, ColorIs Black, ColorIs Red],
                             Macros.creature,
                             InZone (Macros.graveyardOf You)]))
                (Macros.keywordCosting "Unearth" ItsManaCost))

||| Dralnu, Lich Lord
public export
dralnuLichLord : Effect []
dralnuLichLord =
  Macros.gains (Macros.target (And [Macros.instantOrSorcery,
                                    InZone (Macros.graveyardOf You)]))
               (Macros.keywordCosting "Flashback" ItsManaCost)
               (Just Macros.untilEndOfTurn)

||| Dregscape Zombie
public export
dregscapeZombie : Ability
dregscapeZombie = Macros.keywordCosting "Unearth" (Mana [Macros.pip Black])

||| Think Twice
public export
thinkTwice : Ability
thinkTwice =
  Macros.keywordCosting "Flashback" (Mana [Macros.generic 2, Macros.pip Blue])

||| Greater Mossdog
public export
greaterMossdog : Ability
greaterMossdog = Macros.keywordNumber "Dredge" (Lit 3)

||| Raven's Crime
public export
ravensCrime : Ability
ravensCrime = Macros.keyword "Retrace"

||| Barkhide Mauler
public export
barkhideMauler : Ability
barkhideMauler = Macros.keywordCosting "Cycling" (Mana [Macros.generic 2])

||| Ninja of the New Moon
public export
ninjaOfTheNewMoon : Ability
ninjaOfTheNewMoon =
  Macros.keywordCosting "Ninjutsu" (Mana [Macros.generic 3, Macros.pip Black])

||| Thunderous Wrath
public export
thunderousWrath : Ability
thunderousWrath = Macros.keywordCosting "Miracle" (Mana [Macros.pip Red])

||| Bygone Colossus
public export
bygoneColossus : Ability
bygoneColossus = Macros.keywordCosting "Warp" (Mana [Macros.generic 3])

||| Nezumi Ronin
public export
nezumiRonin : Ability
nezumiRonin = Macros.keywordNumber "Bushido" (Lit 1)

||| Steppe Lynx
public export
steppeLynx : Ability
steppeLynx =
  Macros.abilityWord Landfall
    (Macros.triggered Whenever (Enters (Macros.a (And [Macros.land, HasPossessor ControllerAx You])) Nothing)
                      (Macros.gets Macros.thisCreature (PtUp (Lit 2)) (PtUp (Lit 2))
                            (Just Macros.untilEndOfTurn)))

||| Nimble Mongoose
public export
nimbleMongoose : Ability
nimbleMongoose =
  Macros.abilityWord Threshold
    (Static (Macros.onlyWhile (Gets Macros.thisCreature (PtUp (Lit 2)) (PtUp (Lit 2)))
                              (CompareAmt (Macros.countOf (InZone (Macros.graveyardOf You)))
                                          AtLeast (Lit 7))))

||| Ghor-Clan Rampager
public export
ghorClanRampager : Ability
ghorClanRampager =
  Macros.abilityWord Bloodrush
    (Macros.activated (Compound [Mana [Macros.pip Red, Macros.pip Green],
                          Do (Macros.discard You This)])
                      (Macros.sharedSubject
                         (Macros.target (And [Macros.creature, Attacking]))
                         [ Gets (Macros.ownSubject (Macros.target (And [Macros.creature, Attacking])))
                                (PtUp (Lit 4)) (PtUp (Lit 4))
                         , Gains (Macros.ownSubject (Macros.target (And [Macros.creature, Attacking])))
                                 (Macros.keyword "Trample") ]
                         (Just Macros.untilEndOfTurn)))

||| Glaring Spotlight
public export
glaringSpotlight : Card
glaringSpotlight =
  Macros.card "Glaring Spotlight" (Just [Macros.generic 1]) []
       (MkTypeLine [] [Artifact])
       [ Static (Macros.canBeTargetedAsThough
                   (Macros.allOf (And [Macros.creature, HasPossessor ControllerAx (PlayerGroup YourOpponents),
                                HasKeyword (TheKeyword "Hexproof")]))
                   (Macros.allOf (And [Joined Macros.spell (AbilityHead AnyOnStack), HasPossessor ControllerAx You]))
                   (Not (HasKeyword (TheKeyword "Hexproof"))))
       , Macros.activated
           (Compound [Mana [Macros.generic 3], Do (Macros.sacrifice You Macros.thisArtifact)])
           (Macros.sharedSubject (Macros.allOf Macros.creatureYouControl)
              [ Gains (Macros.ownSubject (Macros.allOf Macros.creatureYouControl))
                      (Macros.keyword "Hexproof")
              , Deontic (Macros.ownSubject (Macros.allOf Macros.creatureYouControl)) Forbid ["Block"]
                        Patient Nothing NoDeonticPatient Nothing NoDeonticRider ]
              (Just Macros.untilEndOfTurn)) ]
       Nothing

||| Owlbear
public export
owlbear : Ability
owlbear =
  Macros.flavorWord "Keen Senses"
    (Macros.triggered When (Enters Macros.thisCreature Nothing) (Macros.draw You (Lit 1)))

||| Canoptek Wraith
public export
canoptekWraith : Ability
canoptekWraith =
  Macros.flavorWord "Wraith Form"
    (Static (Deontic Macros.thisCreature Forbid ["Block"] Patient Nothing
                     NoDeonticPatient Nothing NoDeonticRider))

||| Tymora's Invoker
public export
tymorasInvoker : Ability
tymorasInvoker =
  Macros.flavorWord "Sleight of Hand"
    (Macros.activated (Mana [Macros.generic 8]) ((Macros.draw You (Lit 2))))


||| Twinshot Sniper
public export
twinshotSniper : Card
twinshotSniper =
  Macros.card "Twinshot Sniper"
       (Just [Macros.generic 3, Macros.pip Red]) []
       (MkTypeLine [creatureType "Goblin", creatureType "Archer"] [Artifact, Creature])
       [ Macros.keyword "Reach"
       , Macros.triggered When (Enters Macros.thisCreature Nothing)
           (DealDamage It (Lit 2) (Macros.target Macros.anyTarget))
       , Macros.abilityWord Channel
           (Macros.activated (Compound [Mana [Macros.generic 1, Macros.pip Red],
                                        Do (Macros.discard You This)])
                             (DealDamage It (Lit 2) (Macros.target Macros.anyTarget))) ]
       (Just (2, 3))

||| Balance of Power
public export
balanceOfPower : Effect []
balanceOfPower =
  If (CompareAmt (Macros.countOf (InZone (Macros.handOf (Macros.target Opponent))))
                 Greater
                 (Macros.countOf (InZone (Macros.handOf You))))
     (Draw You TheDifference)
     Nothing

||| Vraska, Betrayal's Sting
public export
vraskaBetrayalsStingUltimate : Effect []
vraskaBetrayalsStingUltimate =
  If (CompareAmt (CountersOn (Named "Poison") (Macros.target AnyPlayer)) Less (Lit 9))
     (PutCounters TheDifference (PrintedKind (Named "Poison")) They)
     Nothing

||| Dragonlord Ojutai
public export
dragonlordOjutaiHexproof : Ability
dragonlordOjutaiHexproof =
  Static (Macros.onlyWhile (Gains Macros.thisCreature (Macros.keyword "Hexproof"))
                           (Matches It (HasStatus Untapped)))

||| Caustic Bronco
public export
causticBroncoLoss : Effect []
causticBroncoLoss =
  Sequentially [ Macros.revealCards Macros.topCard
               , Macros.move (That CardW) Macros.handZ
               , OnlyIf (Macros.losesLife You (Macros.manaValueOf It))
                        (NotCond (Matches Macros.thisCreature (HasDesignation Saddled)))
                        (Just (Macros.losesLife (Macros.each Opponent) ThatMuch)) ]

||| Gadrak, the Crown-Scourge
public export
gadrakCantAttack : Ability
gadrakCantAttack =
  Static (Macros.onlyUnless (Macros.deontic Macros.thisCreature Forbid ["Attack"] Agent
                                     NoDeonticPatient)
                            (CompareAmt (Macros.countOf (And [Macros.artifact, HasPossessor ControllerAx You]))
                                        AtLeast (Lit 4)))

||| Panglacial Wurm
public export
panglacialWurmCast : Ability
panglacialWurmCast =
  Static ((Macros.mayPlayDeed "Cast" You This (PlayRider (Macros.fromZ Macros.yourLibrary) Nothing Macros.whileSearching False ItsOwnCost)))


||| Quakebringer
public export
quakebringerDamage : Ability
quakebringerDamage =
  Macros.triggeredIf At (BeginningOf Upkeep (Macros.yours))
    (OrCond [ Matches This (InZone Macros.battlefieldZ)
            , AndCond [ Matches This (InZone (Macros.graveyardOf You))
                      , Macros.exists (And [Macros.creature,
                                     HasSubtype (creatureType "Giant"),
                                     HasPossessor ControllerAx You]) ] ])
    (DealDamage This (Lit 2) (Macros.each Opponent))

||| Dark Fortress
public export
darkFortressMana : Ability
darkFortressMana =
  Macros.activatedOnlyIf TapSymbol
    (AddMana You (Lit 1) (Runs [[OfColor Black], [OfColor Red]]) [])
    (OrCond [ Macros.happened Entry Macros.thisLand Lookback.ThisTurn
            , Macros.exists (And [Macros.land, HasSupertype Basic, HasPossessor ControllerAx You]) ])

||| Sand Strangler
public export
sandStranglerDamage : Ability
sandStranglerDamage =
  Macros.triggeredIf When (Enters Macros.thisCreature Nothing)
    (OrCond [ Macros.exists (And [Macros.land, HasSubtype (landType "Desert"),
                           HasPossessor ControllerAx You])
            , Macros.exists (And [Macros.land, HasSubtype (landType "Desert"),
                           InZone (Macros.graveyardOf You)]) ])
    (Macros.may You (DealDamage This (Lit 3) (Macros.target Macros.creature)))

||| Skyblade's Boon
public export
skybladesBoonReturn : Ability
skybladesBoonReturn =
  Macros.activatedOnlyIf (Mana [Macros.generic 2, Macros.pip White])
    (Macros.move This Macros.handZ)
    (OrCond [ Matches This (InZone Macros.battlefieldZ)
            , Matches This (InZone (Macros.graveyardOf You)) ])


goblinArchaeologist : Ability
goblinArchaeologist =
  Macros.activated (Compound [Mana [Macros.pip Red], TapSymbol])
    (Sequentially
       [(Macros.flipCoins You 1),
        Macros.ifThen (FlipCalled You WinsFlip)
          (Sequentially [Macros.destroy (Macros.target Macros.artifact),
                         SetStatus Untapped Macros.thisCreature]),
        Macros.ifThen (FlipCalled You LosesFlip)
          (Macros.sacrifice You Macros.thisCreature)])

contrabandLivestock : Effect []
contrabandLivestock =
  Sequentially
    [Macros.exile You (Macros.target Macros.creature),
     (Macros.rollDice You 1 20),
     Macros.resultsTable
       [Macros.rollRow (Macros.fromTo 1 9)
          (Create (Macros.controllerOf It) (Lit 1)
                  (TokenWritten (Macros.creatureTok 4 4 [Green] [creatureType "Ox"])) []),
        Macros.rollRow (Macros.fromTo 10 19)
          (Create (Macros.controllerOf It) (Lit 1)
                  (TokenWritten (Macros.creatureTok 2 2 [Green] [creatureType "Boar"])) []),
        Macros.rollRow (Macros.exactly 20)
          (Create (Macros.controllerOf It) (Lit 1)
                  (TokenWritten (Macros.creatureTok 0 1 [White] [creatureType "Goat"])) [])]]

||| Hypnotic Specter
hypnoticSpecterDiscard : Effect [MkBinding TheD Player OneOf PlayerP]
hypnoticSpecterDiscard = (Macros.discard They (Macros.aAtRandom (InZone Macros.handZ)))


public export
chanceEncounter : Card
chanceEncounter =
  Macros.card "Chance Encounter"
       (Just [Macros.generic 2, Macros.pip Red, Macros.pip Red]) []
       (MkTypeLine [] [Enchantment])
       [ Macros.triggered Whenever (FlipsCoin You (Just WinsFlip))
                          (PutCounters (Lit 1) (PrintedKind (Named "Luck")) Macros.thisEnchantment)
       , Macros.triggeredIf At
                            (BeginningOf Upkeep (Macros.yours))
                            (CompareAmt (CountersOn (Named "Luck") Macros.thisEnchantment)
                                        AtLeast (Lit 10))
                            (Concludes WinGame You) ]
       Nothing

||| Karplusan Minotaur's win arm
public export
karplusanMinotaurWinFlip : Ability
karplusanMinotaurWinFlip =
  Macros.triggered Whenever (FlipsCoin You (Just WinsFlip))
                   (DealDamage Macros.thisCreature (Lit 1)
                               (Macros.target Macros.anyTarget))

public export
brazenDwarf : Card
brazenDwarf =
  Macros.card "Brazen Dwarf"
       (Just [Macros.generic 1, Macros.pip Red]) []
       (MkTypeLine [creatureType "Dwarf", creatureType "Shaman"] [Creature])
       [ Macros.triggered Whenever (RollsDice You ManyDice AnyDie AnyResult)
                          (DealDamage Macros.thisCreature (Lit 1)
                                      (Macros.each Opponent)) ]
       (Just (1, 3))

||| Vexing Puzzlebox
public export
vexingPuzzleboxCounters : Ability
vexingPuzzleboxCounters =
  Macros.triggered Whenever (RollsDice You ManyDice AnyDie AnyResult)
                   (PutCounters Macros.theResult (PrintedKind (Named "Charge")) Macros.thisArtifact)

public export
spaceFamilyGoblinsonRoll : Ability
spaceFamilyGoblinsonRoll =
  Macros.triggered Whenever (RollsDice You OneDie AnyDie AnyResult)
                   (PutCounters (Lit 1) (PrintedKind Macros.plusOnePlusOne)
                                Macros.thisCreature)

||| Ral Zarek's ultimate
public export
ralZarekUltimate : Effect []
ralZarekUltimate =
  Sequentially [(Macros.flipCoins You 5),
                ExtraTurn You (Macros.coinsThatCameUp Heads)]

||| Spark Fiend
public export
sparkFiendUpkeepRoll : Effect []
sparkFiendUpkeepRoll =
  Sequentially [(Macros.rollDice You 2 6),
                Macros.ifThen (CompareAmt Macros.theTotal Eq (Lit 7))
                              (Macros.sacrifice You Macros.thisCreature)]


||| Berserker's Frenzy's roll
public export
berserkersFrenzyRoll : Effect []
berserkersFrenzyRoll =
  Sequentially [(Macros.rollDice You 2 20), IgnoreOutcomes (IgnoreExtreme LowestRoll)]

public export
ironMastiffIgnore : Effect []
ironMastiffIgnore =
  Sequentially [(Macros.rollDice You 1 20), IgnoreOutcomes (IgnoreAllBut HighestRoll)]

public export
xenosquirrelsShift : Effect []
xenosquirrelsShift =
  Sequentially [(Macros.rollDice You 1 6), Macros.shiftResult (Lit 1)]

||| Wyll, Blade of Frontiers
public export
wyllExtraDie : Effect []
wyllExtraDie =
  Macros.ifWouldInstead (RollsDice You ManyDice AnyDie AnyResult)
    (Sequentially [ RollDice You (Plus ThatMuch (Lit 1)) ThoseDice
                  , IgnoreOutcomes (IgnoreExtreme LowestRoll) ])
    Nothing

||| Atomwheel Acrobats
public export
atomwheelAcrobatsRoll : Ability
atomwheelAcrobatsRoll =
  Macros.triggered Whenever (Macros.youRollResultIn (Range (Just 1) (Just 2)))
                   (PutCounters ThatMuch (PrintedKind Macros.plusOnePlusOne)
                                Macros.thisCreature)

||| Monoxa, Midway Manager
public export
monoxaRollTrigger : Ability
monoxaRollTrigger =
  Macros.triggered Whenever (Macros.youRollResultIn (Macros.orHigher 3))
    (Sequentially
      [ Macros.gains Macros.thisCreature (KeywordAbility "FirstStrike" Nothing Nothing)
                     (Just Macros.untilEndOfTurn)
      , Macros.ifThen (CompareAmt Macros.theResult AtLeast (Lit 4))
                      (Macros.gains Macros.thisCreature
                                    (KeywordAbility "Menace" Nothing Nothing)
                                    (Just Macros.untilEndOfTurn))
      , Macros.ifThen (CompareAmt Macros.theResult AtLeast (Lit 5))
                      (Macros.gains Macros.thisCreature
                                    (KeywordAbility "Lifelink" Nothing Nothing)
                                    (Just Macros.untilEndOfTurn)) ])

||| Netherese Puzzle-Ward
public export
netheresePuzzleWardIllumination : Ability
netheresePuzzleWardIllumination =
  Macros.triggered Whenever Macros.youRollHighestNatural (Macros.draw You (Lit 1))

||| Resolute Veggiesaur
public export
resoluteVeggiesaurThirdDie : Ability
resoluteVeggiesaurThirdDie =
  Macros.triggered Whenever
    (NthOccurrence (Nth 3) (Just Turn) (RollsDice You OneDie AnyDie AnyResult))
    (PutCounters (Lit 1) (PrintedKind Macros.plusOnePlusOne)
                 Macros.thisCreature)

||| Fractured Powerstone
public export
fracturedPowerstonePlanarRoll : Ability
fracturedPowerstonePlanarRoll =
  Macros.activatedOnlyDuring TapSymbol Macros.rollThePlanarDie AsSorcery

||| Ichor Elixir
public export
ichorElixirPlanarDice : Effect []
ichorElixirPlanarDice =
  Macros.ifWouldInstead Macros.youRollPlanarDice
    (Sequentially [ RollPlanarDie You (Plus ThatMuch (Lit 1))
                  , IgnoreOutcomes (IgnoreChosen Nothing (Lit 1)) ])
    Nothing

||| Vedalken Squirrel-Whacker
public export
vedalkenSquirrelWhackerReroll : Effect []
vedalkenSquirrelWhackerReroll =
  Macros.ifWouldInstead (RollsDice You ManyDice (SidedDie 6) AnyResult)
    (RollDice You ThatMuch ThoseDice)
    Nothing

||| Krark's Thumb
public export
krarksThumbExtraFlip : Effect []
krarksThumbExtraFlip =
  Macros.ifWouldInstead (Macros.flipsCoin You)
    (Sequentially [ (Macros.flipCoins You 2)
                  , IgnoreOutcomes (IgnoreChosen Nothing (Lit 1)) ])
    Nothing

||| Bamboozling Beeble
public export
bamboozlingBeebleIgnore : Ability
bamboozlingBeebleIgnore =
  Macros.activated (Compound [Mana [Macros.generic 1], TapSymbol])
    (Macros.nextTimeWouldInstead
       (RollsDice (Macros.target AnyPlayer) ManyDice AnyDie AnyResult)
       (Sequentially [ RollDice They (Plus ThatMuch (Lit 1)) ThoseDice
                     , IgnoreOutcomes (IgnoreChosen (Just You) (Lit 1)) ])
       (Just Macros.thisTurn))

public export
missyChaosBranch : Effect []
missyChaosBranch = Sequentially [(Macros.draw You (Lit 1)), Macros.chaosEnsues]

||| Farideh, Devil's Chosen
public export
faridehResultRead : Ability
faridehResultRead =
  Macros.triggered Whenever (RollsDice You ManyDice AnyDie AnyResult)
                   (Macros.ifThen (AnyResultIs AtLeast (Lit 10))
                                  (Draw You (Lit 1)))

||| Celebr-8000
public export
celebr8000Doubles : Effect []
celebr8000Doubles =
  Sequentially [ (Macros.rollDice You 2 6)
               , Macros.ifThen RolledDoubles
                   (Macros.gains Macros.thisCreature
                                 (KeywordAbility "DoubleStrike" Nothing Nothing)
                                 (Just Macros.untilEndOfTurn)) ]

||| Goblin Assassin
public export
goblinAssassinCoinTails : Effect []
goblinAssassinCoinTails =
  Sequentially [ FlipCoins (Macros.each AnyPlayer) (FlipCount (Lit 1))
               , Macros.sacrifice (Macros.each (And [AnyPlayer, CoinCameUp Tails]))
                                  (Macros.a Macros.creature) ]

||| Rakdos, the Showstopper
public export
rakdosShowstopperFlips : Effect []
rakdosShowstopperFlips =
  Sequentially
    [ (FlipCoins You (FlipPer (Macros.each (And [ Macros.creature
                   , Not (Or [ HasSubtype (creatureType "Demon")
                             , HasSubtype (creatureType "Devil")
                             , HasSubtype (creatureType "Imp") ]) ]))))
    , Macros.destroy (Macros.each (And [Macros.creature, CoinCameUp Tails])) ]

||| Warp Vortex
public export
warpVortexFlips : Effect []
warpVortexFlips = (FlipCoins You (FlipPer (Macros.each Opponent)))

public export
centaurOfAttention : Card
centaurOfAttention =
  Macros.card "Centaur of Attention"
       (Just [Macros.generic 2, Macros.pip Green, Macros.pip Green]) []
       (MkTypeLine [creatureType "Centaur", creatureType "Advisor"] [Creature])
       [ Macros.triggered When (Enters Macros.thisCreature Nothing)
                          (Sequentially [ (Macros.rollDice You 5 6)
                                        , StoreResults Macros.thisCreature ])
       , Macros.triggered At (BeginningOf Combat (Macros.yours))
                          (Macros.may You
                             (RerollStored You Macros.anyNumber
                                           Macros.thisCreature))
       , Static (AndAlso [ Gets Macros.thisCreature
                                (PtUp (LetterVal X)) (PtUp (LetterVal X))
                         , Define X
                             (GreatestStoredMatch Macros.thisCreature) ]) ]
       (Just (0, 0))


waxWane : Card
waxWane =
  SplitCard
    (Macros.frontFace "Wax" (Just [Macros.pip Green]) [] (MkTypeLine [] [Instant])
            [ Spell (Macros.gets (Macros.target Macros.creature)
                                 (PtUp (Lit 2)) (PtUp (Lit 2))
                                 (Just Macros.untilEndOfTurn)) ]
            Nothing)
    (Macros.frontFace "Wane" (Just [Macros.pip White]) [] (MkTypeLine [] [Instant])
            [ Spell (Macros.destroy (Macros.target Macros.enchantment)) ]
            Nothing)

branchloftPathway : Card
branchloftPathway =
  ModalDfc
    (Macros.frontFace "Branchloft Pathway" Nothing [] (MkTypeLine [] [Land])
            [ Macros.activated TapSymbol
                               (AddMana You (Lit 1) (Runs [[OfColor Green]]) []) ]
            Nothing)
    (Macros.frontFace "Boulderloft Pathway" Nothing [] (MkTypeLine [] [Land])
            [ Macros.activated TapSymbol
                               (AddMana You (Lit 1) (Runs [[OfColor White]]) []) ]
            Nothing)

merfolkSecretkeeper : Card
merfolkSecretkeeper =
  Adventurer
    (Macros.frontFace "Merfolk Secretkeeper" (Just [Macros.pip Blue]) []
            (MkTypeLine [creatureType "Merfolk", creatureType "Wizard"] [Creature]) []
            (Macros.printedBox (Just (0, 4))))
    (Macros.frontFace "Venture Deeper" (Just [Macros.pip Blue]) []
            (MkTypeLine [spellType "Adventure"] [Sorcery])
            [ Spell (Macros.mills (Macros.target AnyPlayer) (Lit 4) They) ]
            Nothing)

orochiEggwatcher : Card
orochiEggwatcher =
  FlipCard
    (Macros.frontFace "Orochi Eggwatcher" (Just [Macros.generic 2, Macros.pip Green]) []
            (MkTypeLine [creatureType "Snake", creatureType "Shaman"] [Creature])
            [ Macros.activated
                (Compound [Mana [Macros.generic 2, Macros.pip Green], TapSymbol])
                (Sequentially
                   [ Macros.create (Lit 1) (Macros.creatureTok 1 1 [Green] [creatureType "Snake"])
                   , Macros.ifThen (CompareAmt (Macros.countOf Macros.creatureYouControl)
                                               AtLeast (Lit 10))
                                   (SetStatus Flipped Macros.thisCreature) ]) ]
            (Macros.printedBox (Just (1, 1))))
    (Macros.backFace "Shidako, Broodmistress" [Legendary]
               (MkTypeLine [creatureType "Snake", creatureType "Shaman"] [Creature])
               [ Macros.activated
                   (Compound [ Mana [Macros.pip Green]
                             , Do (Macros.sacrifice You (Macros.a Macros.creature)) ])
                   (Macros.gets (Macros.target Macros.creature)
                                (PtUp (Lit 3)) (PtUp (Lit 3))
                                (Just Macros.untilEndOfTurn)) ]
               (Macros.printedBox (Just (3, 3))))


public export
planeswalkerBackWithoutLoyalty : CardFace
planeswalkerBackWithoutLoyalty =
  Macros.backFace "" [Legendary] (MkTypeLine [planeswalkerType "Arlinn"] [Planeswalker]) [] Nothing

public export
planeswalkerBackWithoutLoyaltyOk : FaceLaws Back Cards.planeswalkerBackWithoutLoyalty
planeswalkerBackWithoutLoyaltyOk = MkFaceLaws



||| Nykthos, Shrine to Nyx
public export
nykthosShrineToNyx : Card
nykthosShrineToNyx =
  Macros.card "Nykthos, Shrine to Nyx" Nothing [Legendary]
       (MkTypeLine [] [Land])
       [ Macros.activated TapSymbol (AddMana You (Lit 1) (Runs [[Colorless]]) [])
       , Macros.activated (Compound [Mana [Macros.generic 2], TapSymbol])
           (Sequentially
              [ Macros.choose (Macros.a (Macros.quality Color))
              , AddMana You (Devotion You ThatColor Nothing)
                        (OfChosenColor Nothing) [] ]) ]
       Nothing

||| Karametra's Acolyte
public export
karametrasAcolyte : Ability
karametrasAcolyte =
  Macros.activated TapSymbol
    (AddMana You (Devotion You (LitColor Green) Nothing) (Runs [[OfColor Green]]) [])

||| Anax, Hardened in the Forge
public export
anaxPowerDefinition : Ability
anaxPowerDefinition =
  Static (DefinesPt Macros.thisCreature PowerAlone (Devotion You (LitColor Red) Nothing))

||| Gray Merchant of Asphodel
public export
grayMerchantDrain : Effect []
grayMerchantDrain =
  Sequentially [ Macros.losesLife (Macros.each Opponent) (LetterVal X)
               , Define X (Devotion You (LitColor Black) Nothing) ]

||| Erebos, God of the Dead
public export
devotionCondition : Condition []
devotionCondition = CompareAmt (Devotion You (LitColor Black) Nothing) Less (Lit 5)

||| Aspect of Wolf
public export
aspectOfWolf : Ability
aspectOfWolf =
  Static (AndAlso
    [ Gets (AttachHost Enchanted (TypeW Creature))
           (PtUp (LetterVal X)) (PtUp (LetterVal Y))
    , Define X (Half RoundDown
                 (Macros.countOf (And [HasSubtype (landType "Forest"), HasPossessor ControllerAx You])))
    , Define Y (Half RoundUp
                 (Macros.countOf (And [HasSubtype (landType "Forest"), HasPossessor ControllerAx You]))) ])

||| Jaws of Defeat
public export
jawsOfDefeat : Ability
jawsOfDefeat =
  Macros.triggered Whenever (Enters (Macros.a Macros.creatureYouControl) Nothing)
    (Macros.losesLife (Macros.target Opponent)
       (DifferenceBetween (Macros.powerOf (That (TypeW Creature)))
                          (Macros.toughnessOf (That (TypeW Creature)))))

||| Defiling Daemogoth
public export
defilingDaemogothDrain : Effect []
defilingDaemogothDrain =
  Sequentially [ Macros.losesLife (Macros.each Opponent) (LetterVal X)
               , Define X (EventSum LifeGain You Lookback.ThisTurn Nothing) ]

public export
skullsporeNexusTrigger : Ability
skullsporeNexusTrigger =
  Macros.triggered Whenever
    (Dies (Macros.counted (Macros.atLeast 1)
                        (And [Macros.nontoken, Macros.creatureYouControl])))
    (Macros.create (Lit 1)
       (Macros.creatureTokOf
          (Aggregate SumOf (CharAxis Power) (Those CardW))
          (Aggregate SumOf (CharAxis Power) (Those CardW))
          [Green] [creatureType "Fungus", creatureType "Dinosaur"]))

||| Investigator's Journal's count
public export
greatestCreaturesAPlayerControls : Amount []
greatestCreaturesAPlayerControls =
  AggregateOver MaxOf AnyPlayer
    (Macros.countOf (And [Macros.creature, HasPossessor ControllerAx They]))

||| Investigator's Journal
investigatorsJournal : Card
investigatorsJournal =
  Macros.card "Investigator's Journal" (Just [Macros.generic 2]) []
       (MkTypeLine [artifactType "Book", artifactType "Clue"] [Artifact])
       [ Static (Macros.entersWithCounters Macros.thisArtifact
                   greatestCreaturesAPlayerControls (Named "Suspect"))
       , Macros.activated (Compound [Mana [Macros.generic 2], TapSymbol,
                              Do (RemoveCounters (Just (Macros.exactly 1)) (Just (PrintedKind (Named "Suspect")))
                                    Macros.thisArtifact)])
                          (Macros.draw You (Lit 1))
       , Macros.activated (Compound [Mana [Macros.generic 2],
                              Do (Macros.sacrifice You Macros.thisArtifact)])
                          (Sequentially
              [ Macros.searchLibraryOrGraveyard
                  (And [Macros.artifact,
                        Compare [CharAxis ManaValue] AtMost (Lit 2)]) ]) ]
       Nothing

||| Cavern-Hoard Dragon
public export
greatestArtifactsAnOpponentControls : Amount []
greatestArtifactsAnOpponentControls =
  AggregateOver MaxOf Opponent
    (Macros.countOf (And [Macros.artifact, HasPossessor ControllerAx They]))

||| Lhurgoyf
public export
lhurgoyfDefinition : Ability
lhurgoyfDefinition =
  Static (AndAlso
    [ DefinesPt Macros.thisCreature PowerAlone
        (Macros.countOf (And [Macros.creature, InZone Macros.graveyardZ]))
    , DefinesPt Macros.thisCreature ToughnessAlone
        (Plus ThatMuch (Lit 1)) ])

||| Shapeshifter's printed box
public export
shapeshifterBox : PrintedBox
shapeshifterBox = PtBox PrintedStar (PrintedMinusStar 7)

||| Shapeshifter
public export
shapeshifter : Card
shapeshifter =
  Macros.cardOf "Shapeshifter" (Just [Macros.generic 6]) []
       (MkTypeLine [creatureType "Shapeshifter"] [Artifact, Creature])
       [ Static (Macros.entersChoosingFrom Macros.thisCreature Number
                                           (NumberBetween 0 7))
       , Macros.triggered At (BeginningOf Upkeep (Macros.yours))
           (Macros.may You
              (Macros.choose (Macros.a (Macros.qualityFrom Number
                                          (NumberBetween 0 7)))))
       , Static (AndAlso
           [ DefinesPt Macros.thisCreature PowerAlone TheLastChosenNumber
           , DefinesPt Macros.thisCreature ToughnessAlone
               (Minus (Lit 7) TheLastChosenNumber) ]) ]
       (Just shapeshifterBox)

||| Multiple Choice
public export
multipleChoiceFirstArm : Effect []
multipleChoiceFirstArm =
  If (CompareAmt (LetterVal X) Eq (Lit 1))
     (Sequentially [ Macros.scry You (Lit 1)
                   , (Macros.draw You (Lit 1)) ])
     Nothing

||| Multiple Choice, fourth arm
public export
multipleChoiceFourthGate : Condition []
multipleChoiceFourthGate = CompareAmt (LetterVal X) AtLeast (Lit 4)

||| Fell the Mighty
public export
fellTheMighty : Effect []
fellTheMighty =
  Macros.destroy
    (Macros.allOf (And [Macros.creature,
                 Compare [CharAxis Power] Greater
                         (Macros.powerOf (Macros.target Macros.creature))]))

||| Birthing Pod
public export
birthingPodSearch : Ability
birthingPodSearch =
  Macros.activated
    (Compound [Mana [Macros.generic 1, Macros.phyrexianPip Green],
               TapSymbol,
               Do (Macros.sacrifice You (Macros.a Macros.creature))])
    (Macros.searchLibraryFor
       (And [Macros.creature,
             Compare [CharAxis ManaValue] Eq
                     (Plus (Lit 1)
                           (Macros.manaValueOf
                              (Macros.theVerbed "Sacrifice"
                                                (TypeW Creature))))]))



||| Wavebreak Hippocamp
public export
wavebreakHippocamp : Ability
wavebreakHippocamp =
  Macros.triggeredOnlyDuring Whenever
    (NthOccurrence (Nth 1) Nothing (Casts You (Macros.a Macros.spell) Nothing))
    (DuringWindow Turn (Just (Macros.each Opponent)))
    (Macros.draw You (Lit 1))

||| Midnight Clock
public export
midnightClockHeader : GameEvent []
midnightClockHeader =
  NthOccurrence (Nth 12) Nothing
    (Macros.singleCounterEvent CounterPut (Named "Hour") Macros.thisArtifact)

||| Political Triumph
public export
politicalTriumphHeader : GameEvent []
politicalTriumphHeader =
  NthOccurrence (Nth 4) Nothing
    (Macros.singleCounterEvent CounterPut (Named "Plan") Macros.thisEnchantment)

||| Run the Play (Striding Shotcaller's other half)
public export
runThePlayCounters : Effect []
runThePlayCounters =
  PutCounters (Lit 1) (PrintedKind Macros.plusOnePlusOne)
              (EachOf (Macros.targets (UpToOf (LetterVal X)) Macros.creature))

||| Berserker's Frenzy, the 1—14 striation
public export
berserkersFrenzyLowRoll : Effect []
berserkersFrenzyLowRoll =
  Sequentially
    [ Macros.choose (Macros.counted Macros.anyNumber Macros.creature)
    , Continuously {ts = StaticFirstDone} (Macros.deontic Them Require ["Block"] Agent NoDeonticPatient)
                   (Just Macros.thisTurn) ]


public export
targetPlayerOrPlaneswalker : Noun bs (Object \/ Player)
targetPlayerOrPlaneswalker =
  Macros.target (Macros.kindJoin AnyPlayer (HasType Planeswalker))

public export
targetOpponentOrPlaneswalker : Noun bs (Object \/ Player)
targetOpponentOrPlaneswalker =
  Macros.target (Macros.kindJoin Opponent (HasType Planeswalker))

public export
eachCreatureThatSplitControls :
  {bs : Bindings} ->
  {auto 0 ck : countReach (UnionHalf (TypeW Planeswalker)) OneOf bs = 1} ->
  {auto 0 pk : countReach (UnionHalf PlayerW) OneOf bs = 1} ->
  Noun bs Object
eachCreatureThatSplitControls =
  Macros.each (And [Macros.creature, HasPossessor ControllerAx (Macros.splitOverPlaneswalker {ck} {pk})])

||| Lavalanche
public export
lavalanche : Effect []
lavalanche =
  Simultaneously
    [ DealDamage This (LetterVal X) Cards.targetPlayerOrPlaneswalker
    , DealDamage This (LetterVal X) Cards.eachCreatureThatSplitControls ]

||| Flame Wave
public export
flameWave : Effect []
flameWave =
  Simultaneously
    [ DealDamage This (Lit 4) Cards.targetPlayerOrPlaneswalker
    , DealDamage This (Lit 4) Cards.eachCreatureThatSplitControls ]

||| Chandra Nalaar's ultimate
public export
chandraNalaarUltimate : Effect []
chandraNalaarUltimate =
  Simultaneously
    [ DealDamage This (Lit 10) Cards.targetPlayerOrPlaneswalker
    , DealDamage This (Lit 10) Cards.eachCreatureThatSplitControls ]

||| Chandra, Pyrogenius's ultimate
public export
chandraPyrogeniusUltimate : Effect []
chandraPyrogeniusUltimate =
  Simultaneously
    [ DealDamage This (Lit 6) Cards.targetPlayerOrPlaneswalker
    , DealDamage This (Lit 6) Cards.eachCreatureThatSplitControls ]

||| Bonfire of the Damned
public export
bonfireOfTheDamned : Effect []
bonfireOfTheDamned =
  Simultaneously
    [ DealDamage This (LetterVal X) Cards.targetPlayerOrPlaneswalker
    , DealDamage This (LetterVal X) Cards.eachCreatureThatSplitControls ]

||| Chandra's Fury
public export
chandrasFury : Effect []
chandrasFury =
  Simultaneously
    [ DealDamage This (Lit 4) Cards.targetPlayerOrPlaneswalker
    , DealDamage This (Lit 1) Cards.eachCreatureThatSplitControls ]

||| Heart of Bogardan
public export
heartOfBogardanBody : Effect []
heartOfBogardanBody =
  Sequentially
    [ Simultaneously
        [ DealDamage This (LetterVal X) Cards.targetPlayerOrPlaneswalker
        , DealDamage This (LetterVal X) Cards.eachCreatureThatSplitControls ]
    , Define X (Minus (Times 2 (CountersOn (Named "Age") Macros.thisEnchantment)) (Lit 2)) ]

||| Angrath, Minotaur Pirate's plus
public export
angrathMinotaurPirateBolt : Effect []
angrathMinotaurPirateBolt =
  Simultaneously
    [ DealDamage This (Lit 1) Cards.targetOpponentOrPlaneswalker
    , DealDamage This (Lit 1) Cards.eachCreatureThatSplitControls ]

public export
whichOfYouBurnsBrightestBody : Effect []
whichOfYouBurnsBrightestBody =
  Simultaneously
    [ DealDamage This (LetterVal X) Cards.targetOpponentOrPlaneswalker
    , DealDamage This (LetterVal X) Cards.eachCreatureThatSplitControls ]

||| Chandra, Pyromaster's plus
public export
chandraPyromasterBolt : Effect []
chandraPyromasterBolt =
  Simultaneously
    [ DealDamage This (Lit 1) Cards.targetPlayerOrPlaneswalker
    , DealDamage This (Lit 1)
        (Macros.targets (Macros.upTo 1)
                     (And [Macros.creature,
                           HasPossessor ControllerAx Macros.splitOverPlaneswalker])) ]

||| Ravager of the Fells
public export
ravagerOfTheFellsBolt : Effect []
ravagerOfTheFellsBolt =
  Simultaneously
    [ DealDamage Macros.thisCreature (Lit 2) Cards.targetOpponentOrPlaneswalker
    , DealDamage Macros.thisCreature (Lit 2)
        (Macros.targets (Macros.upTo 1)
                     (And [Macros.creature,
                           HasPossessor ControllerAx Macros.splitOverPlaneswalker])) ]

||| Soul of Shandalar's battlefield activation
public export
soulOfShandalarBolt : Effect []
soulOfShandalarBolt =
  Simultaneously
    [ DealDamage Macros.thisCreature (Lit 3) Cards.targetPlayerOrPlaneswalker
    , DealDamage Macros.thisCreature (Lit 3)
        (Macros.targets (Macros.upTo 1)
                     (And [Macros.creature,
                           HasPossessor ControllerAx Macros.splitOverPlaneswalker])) ]

||| Soul of Shandalar's graveyard activation
public export
soulOfShandalarGraveyardBolt : Effect []
soulOfShandalarGraveyardBolt =
  Simultaneously
    [ DealDamage This (Lit 3) Cards.targetPlayerOrPlaneswalker
    , DealDamage This (Lit 3)
        (Macros.targets (Macros.upTo 1)
                     (And [Macros.creature,
                           HasPossessor ControllerAx Macros.splitOverPlaneswalker])) ]

||| Blightning
public export
blightning : Effect []
blightning =
  Sequentially
    [ DealDamage This (Lit 3) Cards.targetPlayerOrPlaneswalker
    , Repeated (Lit 2) ((Macros.discard Macros.splitOverPlaneswalker (Macros.a (InZone Macros.handZ)))) ]

||| Rakdos's Return
public export
rakdossReturn : Effect []
rakdossReturn =
  Sequentially
    [ DealDamage This (LetterVal X) Cards.targetOpponentOrPlaneswalker
    , Repeated (LetterVal X) ((Macros.discard Macros.splitOverPlaneswalker (Macros.a (InZone Macros.handZ)))) ]

||| Nicol Bolas, Planeswalker's ultimate
public export
nicolBolasUltimate : Effect []
nicolBolasUltimate =
  Sequentially
    [ DealDamage This (Lit 7) Cards.targetPlayerOrPlaneswalker
    , Repeated (Lit 7) ((Macros.discard Macros.splitOverPlaneswalker (Macros.a (InZone Macros.handZ))))
    , Repeated (Lit 7) (Macros.sacrifice Macros.splitOverPlaneswalker
                                         (Macros.a Permanent)) ]

||| Pulse of the Forge
public export
pulseOfTheForge : Effect []
pulseOfTheForge =
  Sequentially
    [ DealDamage This (Lit 4) Cards.targetPlayerOrPlaneswalker
    , If (CompareAmt (PlayerStatOf LifeTotal Macros.splitOverPlaneswalker)
                     Greater (PlayerStatOf LifeTotal You))
         (Macros.move This Macros.handZ)
         Nothing ]

||| Goblin Lyre's losing arm
public export
goblinLyreLoseFlip : Effect []
goblinLyreLoseFlip =
  Sequentially
    [ DealDamage Macros.thisArtifact (Macros.countOf Macros.creatureYouControl)
                 Cards.targetOpponentOrPlaneswalker
    , DealDamage Macros.thisArtifact
                 (Macros.countOf (And [Macros.creature,
                                HasPossessor ControllerAx Macros.splitOverPlaneswalker]))
                 You ]

||| Chain of Plasma
public export
chainOfPlasmaOfferee : Noun (nomIntro {bs = []} (Macros.target Macros.anyTarget)) Player
chainOfPlasmaOfferee = Macros.splitOverPermanent

||| Chain Lightning
public export
chainLightningPayer : Noun (nomIntro {bs = []} (Macros.target Macros.anyTarget)) Player
chainLightningPayer = Macros.splitOverPermanent

||| Flames of the Blood Hand
public export
flamesOfTheBloodHandSubject :
  Noun (nomIntro {bs = []} Cards.targetPlayerOrPlaneswalker) Player
flamesOfTheBloodHandSubject = Macros.splitOverPlaneswalker

||| Flaming Gambit
public export
flamingGambitOfferee :
  Noun (nomIntro {bs = []} Cards.targetPlayerOrPlaneswalker) Player
flamingGambitOfferee = Macros.splitOverPlaneswalker

||| Quenchable Fire
public export
quenchableFire : Effect []
quenchableFire =
  Sequentially
    [ DealDamage This (Lit 3) Cards.targetPlayerOrPlaneswalker
    , Macros.delayed (BeginningOf Upkeep (Macros.yours))
        (Unless (DealDamage This (Lit 3) Macros.thatJoin)
                Macros.splitOverPlaneswalker
                (Mana [Macros.pip Blue])) ]

||| Searing Blaze, both sentences
public export
searingBlaze : Ability
searingBlaze =
  Macros.abilityWord Landfall
    (Spell
      (Macros.insteadOf
        (Simultaneously
           [ DealDamage This (Lit 1) Cards.targetPlayerOrPlaneswalker
           , DealDamage This (Lit 1)
               (Macros.target (And [Macros.creature,
                                    HasPossessor ControllerAx Macros.splitOverPlaneswalker])) ])
        (Simultaneously
           [ DealDamage This (Lit 3) Macros.thatJoin
           , DealDamage This (Lit 3) (That (TypeW Creature)) ])))

||| Thought Lash
public export
thoughtLashTrigger : Ability
thoughtLashTrigger =
  Macros.triggered When
    (PaysCost (Just (Macros.a AnyPlayer)) Unpaid Macros.thisEnchantment
              "CumulativeUpkeep")
    (Macros.exile (That PlayerW) (Macros.each (InZone (Macros.libraryOf They))))

||| Heart of Bogardan
public export
heartOfBogardanHeader : GameEvent []
heartOfBogardanHeader =
  PaysCost (Just (Macros.a AnyPlayer)) Unpaid Macros.thisEnchantment "CumulativeUpkeep"

||| Heart of Bogardan
public export
heartOfBogardan : Card
heartOfBogardan =
  Macros.card "Heart of Bogardan"
       (Just [Macros.generic 2, Macros.pip Red, Macros.pip Red]) []
       (MkTypeLine [] [Enchantment])
       [ Macros.cumulativeUpkeep (Mana [Macros.generic 2])
       , Macros.triggered When Cards.heartOfBogardanHeader
           (Sequentially
              [ Simultaneously
                  [ DealDamage This (LetterVal X) Cards.targetPlayerOrPlaneswalker
                  , DealDamage This (LetterVal X) Cards.eachCreatureThatSplitControls ]
              , Define X (Minus (Times 2 (CountersOn (Named "Age") Macros.thisEnchantment)) (Lit 2)) ]) ]
       Nothing

||| Balduvian Fallen
public export
balduvianFallenHeader : GameEvent []
balduvianFallenHeader =
  PaysCost Nothing Paid Macros.thisCreature "CumulativeUpkeep"

||| Shah of Naar Isle
public export
shahOfNaarIsle : Card
shahOfNaarIsle =
  Macros.card "Shah of Naar Isle"
       (Just [Macros.generic 3, Macros.pip Red]) []
       (MkTypeLine [creatureType "Efreet"] [Creature])
       [ Macros.keyword "Trample"
       , Macros.keywordCosting "Echo" (Mana [Macros.generic 0])
       , Macros.triggered When (PaysCost Nothing Paid Macros.thisCreature "Echo")
                          (Macros.may (Macros.each Opponent)
                                      (Draw They (UpTo (Lit 3)))) ]
       (Just (6, 6))

||| Font of Agonies
public export
fontOfAgoniesTrigger : Ability
fontOfAgoniesTrigger =
  Macros.triggered Whenever (PaysLife You)
    (PutCounters ThatMuch (PrintedKind (Named "Blood")) Macros.thisEnchantment)

||| Hibernation's End
public export
hibernationsEndTrigger : Ability
hibernationsEndTrigger =
  Macros.triggered Whenever
    (PaysCost (Just You) Paid Macros.thisEnchantment "CumulativeUpkeep")
    (Macros.may You
       (Sequentially
          [ Macros.searchLibraryFor
              (And [Macros.creature,
                    Compare [CharAxis ManaValue] Eq (CountersOn (Named "Age") Macros.thisEnchantment)])
          , Macros.putOntoBattlefield (That CardW)
          , Macros.shuffle ]))


||| Krosan Druid
public export
krosanDruid : Card
krosanDruid =
  Macros.card "Krosan Druid"
       (Just [Macros.generic 2, Macros.pip Green]) []
       (MkTypeLine [creatureType "Centaur", creatureType "Druid"] [Creature])
       [ Macros.keywordCosting "Kicker"
           (Mana [Macros.generic 4, Macros.pip Green])
       , Macros.triggeredIf When (Enters Macros.thisCreature Nothing)
           (Matches Macros.thisCreature (PaidCost (ByKeyword "Kicker") Nothing))
           (Macros.gainsLife You (Lit 10)) ]
       (Just (2, 3))

||| Lightkeeper of Emeria
public export
lightkeeperOfEmeria : Card
lightkeeperOfEmeria =
  Macros.card "Lightkeeper of Emeria"
       (Just [Macros.generic 3, Macros.pip White]) []
       (MkTypeLine [creatureType "Angel"] [Creature])
       [ Macros.keywordCosting "Multikicker" (Mana [Macros.pip White])
       , Macros.keyword "Flying"
       , Macros.triggered When (Enters Macros.thisCreature Nothing)
           (Macros.gainsLife You
              (Times 2 (TimesPaid (ByKeyword "Kicker") Macros.thisCreature))) ]
       (Just (2, 4))

||| Merfolk Falconer
public export
merfolkFalconer : Card
merfolkFalconer =
  Macros.card "Merfolk Falconer"
       (Just [Macros.generic 3, Macros.pip Blue, Macros.pip Blue]) []
       (MkTypeLine [creatureType "Merfolk", creatureType "Wizard"] [Creature])
       [ Macros.keyword "Flying"
       , Macros.triggered Whenever
           (Casts You (Macros.a (And [Macros.spell,
                                      PaidCost (ByKeyword "Kicker") Nothing])) Nothing)
           (Macros.scry You (Lit 2)) ]
       (Just (4, 4))

||| Ertai's Trickery
public export
ertaisTrickery : Card
ertaisTrickery =
  Macros.card "Ertai's Trickery" (Just [Macros.pip Blue]) []
       (MkTypeLine [] [Instant])
       [ Spell (OnlyIf (Macros.counterSpell (Macros.target Macros.spell))
                       (Matches It (PaidCost (ByKeyword "Kicker") Nothing)) Nothing) ]
       Nothing

||| Baleful Mastery's paid read
public export
balefulMasteryPaidRead : Ability
balefulMasteryPaidRead =
  Spell (If (Matches This (PaidCost TheAlternative Nothing))
            (Draw (Macros.a Opponent) (Lit 1)) Nothing)

||| Stormscape Battlemage
public export
stormscapeBattlemageFirstKicker : Ability
stormscapeBattlemageFirstKicker =
  Macros.triggeredIf When (Enters Macros.thisCreature Nothing)
    (Matches Macros.thisCreature (PaidCost (ByNthKeyword (Nth 1) "Kicker") Nothing))
    (Macros.gainsLife You (Lit 3))

||| Karai, Future of the Foot
public export
karaiSneakPaidThisTurn : Predicate [] Object
karaiSneakPaidThisTurn = PaidCost (ByKeyword "Sneak") (Just ThisTurn)



||| Borrowed Malevolence
public export
borrowedMalevolence : Card
borrowedMalevolence =
  Macros.card "Borrowed Malevolence" (Just [Macros.pip Black]) []
       (MkTypeLine [] [Instant])
       [ Macros.keywordCosting "Escalate" (Mana [Macros.generic 2])
       , Spell (Modal (Range (Just 1) (Just 2))
                  [ Continuously {ts = StaticFirstDone}
                      (Gets (Macros.target Macros.creature)
                            (PtUp (Lit 1)) (PtUp (Lit 1)))
                      (Just Macros.untilEndOfTurn)
                  , Continuously {ts = StaticFirstDone}
                      (Gets (Macros.target Macros.creature)
                            (PtDown (Lit 1)) (PtDown (Lit 1)))
                      (Just Macros.untilEndOfTurn) ]) ]
       Nothing

||| Korlash
public export
grandeurDiscardCost : Cost []
grandeurDiscardCost =
  Do (Macros.discard You
        (Macros.a (And [Named (PrintedName "Korlash, Heir to Blackblade"),
                        OtherThan This, InZone Macros.handZ])))

||| Invigorate
public export
invigorate : Card
invigorate =
  Macros.card "Invigorate"
       (Just [Macros.generic 2, Macros.pip Green]) []
       (MkTypeLine [] [Instant])
       [ Static (Macros.asLongAs
                   (Macros.exists (And [Macros.land, HasSubtype (landType "Forest"),
                                 HasPossessor ControllerAx You]))
                   (AltCost This (Just (Do (ChangeLife Macros.anOpponent
                                                  (Up (Lit 3)))))))
       , Spell (Continuously {ts = StaticFirstDone}
                  (Gets (Macros.target Macros.creature)
                        (PtUp (Lit 4)) (PtUp (Lit 4)))
                  (Just Macros.untilEndOfTurn)) ]
       Nothing

||| Deflecting Swat
public export
deflectingSwatCommanderAltCost : Ability
deflectingSwatCommanderAltCost =
  Static (Macros.asLongAs
            (Macros.exists (And [HasCardDesignation CommanderD, HasPossessor ControllerAx You]))
            (AltCost This Nothing))

||| Fist of Suns
public export
fistOfSuns : Card
fistOfSuns =
  Macros.card "Fist of Suns" (Just [Macros.generic 3]) []
       (MkTypeLine [] [Artifact])
       [ Static (AltCost (Macros.allOf (And [Macros.spell, CastBy You]))
                   (Just (Mana [Macros.pip White, Macros.pip Blue,
                                Macros.pip Black, Macros.pip Red,
                                Macros.pip Green]))) ]
       Nothing

||| Rooftop Storm
public export
rooftopStorm : Card
rooftopStorm =
  Macros.card "Rooftop Storm"
       (Just [Macros.generic 5, Macros.pip Blue]) []
       (MkTypeLine [] [Enchantment])
       [ Static (AltCost (Macros.allOf (And [ HasSubtype (creatureType "Zombie")
                                     , Macros.creature, Macros.spell
                                     , CastBy You ]))
                   (Just (Mana [Macros.generic 0]))) ]
       Nothing

||| Heart of Kiran
public export
heartOfKiranCrewAltCost : Ability
heartOfKiranCrewAltCost =
  Static (AltCost (Macros.allOf (And [AbilityHead (KeywordClass "Crew"),
                               AbilityOf This]))
            (Just (Do (RemoveCounters (Just (Macros.exactly 1))
                                      (Just (PrintedKind (Named "Loyalty")))
                                      (Macros.a (And [HasType Planeswalker,
                                                      HasPossessor ControllerAx You]))))))

||| New Perspectives
public export
newPerspectivesCyclingAltCost : Ability
newPerspectivesCyclingAltCost =
  Static (AltCost (Macros.allOf (AbilityHead (KeywordClass "Cycling")))
            (Just (Mana [Macros.generic 0])))

||| Thick-Skinned Goblin
public export
thickSkinnedGoblinEchoAltCost : Ability
thickSkinnedGoblinEchoAltCost =
  Static (AltCost (Macros.allOf (And [ AbilityHead (KeywordClass "Echo")
                              , AbilityOf (Macros.allOf (And [Permanent,
                                                       HasPossessor ControllerAx You])) ]))
            (Just (Mana [Macros.generic 0])))


||| Memory Plunder
public export
memoryPlunder : Card
memoryPlunder =
  Macros.card "Memory Plunder"
       (Just [Macros.generic 1, Macros.pip Blue, Macros.pip Black]) []
       (MkTypeLine [] [Instant])
       [ Spell (Continuously {ts = StaticFirstDone}
                  ((Macros.mayPlayDeed "Cast" You (Macros.target (And [Macros.instantOrSorcery, IsCard])) (PlayRider (Macros.fromZ (Macros.graveyardOf Macros.anOpponent)) Nothing Nothing False Macros.free)))
                  Nothing) ]
       Nothing

||| Omniscience
public export
omniscience : Card
omniscience =
  Macros.card "Omniscience"
       (Just [Macros.generic 7, Macros.pip Blue, Macros.pip Blue]) []
       (MkTypeLine [] [Enchantment])
       [ Static ((Macros.mayPlayDeed "Cast" You (Macros.allOf Macros.spell) (PlayRider (Macros.fromZ (Macros.handOf You)) Nothing Nothing False Macros.free))) ]
       Nothing

||| Tranquil Frillback
public export
tranquilFrillbackOffer : Effect []
tranquilFrillbackOffer =
  Macros.may You (Pay You (Mana [Macros.pip Green]) (UpToTimes 3))


||| Fumiko the Lowblood
public export
fumikoBushidoX : Ability
fumikoBushidoX =
  Static (AndAlso [ Gains Macros.thisCreature
                          (KeywordAbility "Bushido"
                             (Just (ParamNumber (LetterVal X))) Nothing)
                  , Define X (Macros.countOf Attacking) ])


||| Tainted Adversary
public export
taintedAdversaryOffer : Ability
taintedAdversaryOffer =
  Macros.triggered When (Enters Macros.thisCreature Nothing)
    (Reflexively
       (Macros.may You (Pay You (Mana [Macros.generic 2, Macros.pip Black])
                            AnyNumberOfTimes))
       (PutCounters ThatMuch (PrintedKind Macros.plusOnePlusOne)
                    Macros.thisCreature))


||| Voltage Surge's declaration
public export
voltageSurgeAddedCost : Ability
voltageSurgeAddedCost =
  Static (AddedCost (Do (Macros.sacrifice You (Macros.a Macros.artifact))) True)

||| Requiting Hex's read
public export
requitingHexAdditionalRead : Ability
requitingHexAdditionalRead =
  Spell (If (Matches This (PaidCost TheAdditional Nothing))
            (Macros.gainsLife You (Lit 2)) Nothing)

||| Burn at the Stake
public export
burnAtTheStake : Card
burnAtTheStake =
  Macros.card "Burn at the Stake"
       (Just [Macros.generic 2, Macros.pip Red, Macros.pip Red, Macros.pip Red]) []
       (MkTypeLine [] [Sorcery])
       [ Static (AddedCost (Do (Macros.tap
                                  (Macros.counted Macros.anyNumber
                                     (And [Macros.creature, HasPossessor ControllerAx You,
                                           HasStatus Untapped])))) False)
       , Spell (DealDamage This (Times 3 GroupSize)
                           (Macros.target Macros.anyTarget)) ]
       Nothing

||| Explosive Singularity
public export
explosiveSingularity : Card
explosiveSingularity =
  Macros.card "Explosive Singularity"
       (Just [Macros.generic 8, Macros.pip Red, Macros.pip Red]) []
       (MkTypeLine [] [Sorcery])
       [ Static (AddedCost (Do (Macros.tap
                                  (Macros.counted Macros.anyNumber
                                     (And [Macros.creature, HasPossessor ControllerAx You,
                                           HasStatus Untapped])))) True)
       , Static (CostsToCast This (CostLess (Times 1 GroupSize) Nothing))
       , Spell (DealDamage This (Lit 10) (Macros.target Macros.anyTarget)) ]
       Nothing

||| Caller of the Hunt
public export
callerOfTheHunt : Card
callerOfTheHunt =
  Macros.cardOf "Caller of the Hunt"
       (Just [Macros.generic 2, Macros.pip Green]) []
       (MkTypeLine [creatureType "Human"] [Creature])
       [ Static (AddedCost
                   (Do (Macros.choose (Macros.a (Macros.quality (SubtypeQ Creature)))))
                   False)
       , Static (DefinesPt Macros.thisCreature BothEach
                   (Macros.countOf (And [Macros.creature,
                                  OfChosen (SubtypeQ Creature)]))) ]
       (Just (PtBox PrintedStar PrintedStar))


||| Latchkey Faerie
public export
latchkeyFaerie : Card
latchkeyFaerie =
  Macros.card "Latchkey Faerie"
       (Just [Macros.generic 3, Macros.pip Blue]) []
       (MkTypeLine [creatureType "Faerie", creatureType "Rogue"] [Creature])
       [ Macros.keyword "Flying"
       , Macros.keywordCosting "Prowl"
           (Mana [Macros.generic 2, Macros.pip Blue])
       , Macros.triggeredIf When (Enters Macros.thisCreature Nothing)
           (Matches Macros.thisCreature (PaidCost (ByKeyword "Prowl") Nothing))
           (Sequentially
              [ Macros.searchLibraryOrGraveyard
                  (And [Macros.artifact,
                        Compare [CharAxis ManaValue] AtMost (Lit 2)]) ]) ]
       (Just (3, 1))

||| Tyrant of Valakut
public export
tyrantOfValakut : Card
tyrantOfValakut =
  Macros.card "Tyrant of Valakut"
       (Just [Macros.generic 5, Macros.pip Red, Macros.pip Red]) []
       (MkTypeLine [creatureType "Dragon"] [Creature])
       [ Macros.keywordCosting "Surge"
           (Mana [Macros.generic 3, Macros.pip Red, Macros.pip Red])
       , Macros.keyword "Flying"
       , Macros.triggeredIf When (Enters Macros.thisCreature Nothing)
           (Matches Macros.thisCreature (PaidCost (ByKeyword "Surge") Nothing))
           (DealDamage Macros.thisCreature (Lit 3)
                       (Macros.target Macros.anyTarget)) ]
       (Just (5, 4))

||| Rafter Demon
public export
rafterDemon : Card
rafterDemon =
  Macros.card "Rafter Demon"
       (Just [Macros.generic 2, Macros.pip Black, Macros.pip Red]) []
       (MkTypeLine [creatureType "Demon"] [Creature])
       [ Macros.keywordCosting "Spectacle"
           (Mana [Macros.generic 3, Macros.pip Black, Macros.pip Red])
       , Macros.triggeredIf When (Enters Macros.thisCreature Nothing)
           (Matches Macros.thisCreature (PaidCost (ByKeyword "Spectacle") Nothing))
           ((Macros.discard (Macros.each Opponent) (Macros.a (InZone Macros.handZ)))) ]
       (Just (4, 2))

||| Fall of the Titans
public export
fallOfTheTitansCard : Card
fallOfTheTitansCard =
  Macros.card "Fall of the Titans"
       (Just [Variable, Variable, Macros.pip Red]) []
       (MkTypeLine [] [Instant])
       [ Macros.keywordCosting "Surge" (Mana [Variable, Macros.pip Red])
       , Spell (DealDamage This (LetterVal X)
                  (EachOf (Macros.targets (Macros.upTo 2) Macros.anyTarget))) ]
       Nothing


||| Duneblast
public export
duneblast : Effect []
duneblast =
  Sequentially [ Macros.choose (Macros.counted (Macros.upTo 1) Macros.creature)
               , Macros.destroy TheRest ]

public export
duneblastCard : Card
duneblastCard =
  Macros.card "Duneblast"
       (Just [Macros.generic 4, Macros.pip White, Macros.pip Black,
              Macros.pip Green])
       [] (MkTypeLine [] [Sorcery]) [Spell Cards.duneblast] Nothing

||| Celebrate the Harvest
public export
celebrateTheHarvest : Card
celebrateTheHarvest =
  Macros.card "Celebrate the Harvest"
       (Just [Macros.generic 3, Macros.pip Green]) []
       (MkTypeLine [] [Sorcery])
       [ Spell (Sequentially
                  [ Macros.searchLibraryForCount (UpToOf (LetterVal X))
                      (And [Macros.land, HasSupertype Basic])
                  , Define X (DistinctCount (ValueAxis Power)
                                (Macros.allOf Macros.creatureYouControl))
                  , Macros.putOntoBattlefieldTapped (Those CardW)
                  , Macros.shuffle ]) ]
       Nothing

||| Boreas Charger's description
public export
opponentWithMoreLands : Predicate [] Player
opponentWithMoreLands =
  CompareOver Opponent
    (Macros.countOf (And [Macros.land, HasPossessor ControllerAx They]))
    Greater
    (Macros.countOf (And [Macros.land, HasPossessor ControllerAx You]))

||| Boreas Charger
public export
boreasChargerChoice : Effect []
boreasChargerChoice = Macros.choose (Macros.a Cards.opponentWithMoreLands)

public export
boreasChargerDifference : Amount (effIntro Cards.boreasChargerChoice)
boreasChargerDifference = TheDifference

||| Boreas Charger's spell text
public export
boreasChargerSpell : Effect []
boreasChargerSpell =
  Sequentially
    [ Macros.choose (Macros.a Cards.opponentWithMoreLands)
    , Macros.searchLibraryForCount (ExactlyOf TheDifference)
                                   (HasSubtype (landType "Plains"))
    , Macros.revealCards (Those CardW)
    , Macros.putOntoBattlefieldTapped (Macros.oneOf (Those CardW))
    , Macros.move TheRest Macros.handZ ]

||| Sandstone Oracle
public export
sandstoneOracle : Card
sandstoneOracle =
  Macros.card "Sandstone Oracle" (Just [Macros.generic 7]) []
       (MkTypeLine [creatureType "Sphinx"] [Artifact, Creature])
       [ Macros.keyword "Flying"
       , Macros.triggered When (Enters Macros.thisCreature Nothing)
           (Sequentially
              [ Macros.choose Macros.anOpponent
              , If (CompareAmt (Macros.countOf (InZone (Macros.handOf (That PlayerW))))
                               Greater
                               (Macros.countOf (InZone (Macros.handOf You))))
                   (Draw You TheDifference)
                   Nothing ]) ]
       (Just (4, 4))

||| Slithermuse
public export
slithermuseTrigger : Ability
slithermuseTrigger =
  Macros.triggered When (Macros.leavesBattlefield Macros.thisCreature)
    (Sequentially
       [ Macros.choose Macros.anOpponent
       , If (CompareAmt (Macros.countOf (InZone (Macros.handOf (That PlayerW))))
                        Greater
                        (Macros.countOf (InZone (Macros.handOf You))))
            (Draw You TheDifference)
            Nothing ])


||| Tarmogoyf
public export
tarmogoyfDefinition : Ability
tarmogoyfDefinition =
  Static (AndAlso
    [ DefinesPt Macros.thisCreature PowerAlone
        (DistinctCount CardTypeAxis
           (Macros.allOf (InZone (Macros.graveyardOf (PlayerGroup AllPlayers)))))
    , DefinesPt Macros.thisCreature ToughnessAlone
        (Plus ThatMuch (Lit 1)) ])

||| Tarmogoyf's printed box
public export
tarmogoyfBox : PrintedBox
tarmogoyfBox = PtBox PrintedStar (PrintedStarPlus 1)

||| Consuming Blob's definition
public export
consumingBlobDefinition : Ability
consumingBlobDefinition =
  Static (AndAlso
    [ DefinesPt Macros.thisCreature PowerAlone
        (DistinctCount CardTypeAxis
           (Macros.allOf (InZone (Macros.graveyardOf You))))
    , DefinesPt Macros.thisCreature ToughnessAlone
        (Plus ThatMuch (Lit 1)) ])

||| Nighthawk Scavenger's definition
public export
nighthawkScavengerDefinition : Ability
nighthawkScavengerDefinition =
  Static (DefinesPt Macros.thisCreature PowerAlone
    (Plus (Lit 1)
          (DistinctCount CardTypeAxis
             (Macros.allOf (InZone (Macros.graveyardOf (PlayerGroup YourOpponents)))))))

||| Lucid Dreams
public export
lucidDreams : Card
lucidDreams =
  Macros.card "Lucid Dreams"
       (Just [Macros.generic 3, Macros.pip Blue, Macros.pip Blue]) []
       (MkTypeLine [] [Sorcery])
       [ Spell (Sequentially
                  [ Draw You (LetterVal X)
                  , Define X (DistinctCount CardTypeAxis
                                (Macros.allOf (InZone (Macros.graveyardOf You)))) ]) ]
       Nothing

||| Tribal Flames
public export
tribalFlames : Card
tribalFlames =
  Macros.card "Tribal Flames" (Just [Macros.generic 1, Macros.pip Red]) []
       (MkTypeLine [] [Sorcery])
       [ Macros.abilityWord Domain
           (Spell (Sequentially
                     [ DealDamage This (LetterVal X)
                                  (Macros.target Macros.anyTarget)
                     , Define X (DistinctCount (SubtypeAxis Land BasicOnly)
                                   (Macros.allOf (And [Macros.land,
                                                HasPossessor ControllerAx You]))) ])) ]
       Nothing

||| Explosive Prodigy
public export
explosiveProdigyTrigger : Ability
explosiveProdigyTrigger =
  Macros.abilityWord Vivid
    (Macros.triggered When (Enters Macros.thisCreature Nothing)
       (Sequentially
          [ DealDamage It (LetterVal X)
                       (Macros.target (And [Macros.creature,
                                            HasPossessor ControllerAx Macros.anOpponent]))
          , Define X (DistinctCount ColorAxis
                        (Macros.allOf (And [Permanent, HasPossessor ControllerAx You]))) ]))

||| Korvold, Gleeful Glutton
public export
korvoldCombatTrigger : Ability
korvoldCombatTrigger =
  Macros.triggered Whenever
    (Macros.dealsCombatDamage Macros.thisCreature (Macros.a AnyPlayer))
    (Sequentially
       [ PutCounters (LetterVal X) (PrintedKind Macros.plusOnePlusOne) Macros.thisCreature
       , Draw You (LetterVal X)
       , Define X (DistinctCount PermanentTypeAxis
                     (Macros.allOf (InZone (Macros.graveyardOf You)))) ])

||| General Tazri's pump
public export
generalTazriPump : Ability
generalTazriPump =
  Macros.activated
    (Mana [Macros.pip White, Macros.pip Blue, Macros.pip Black,
           Macros.pip Red, Macros.pip Green])
    (Sequentially
       [ Macros.gets (Macros.each (And [Macros.creature,
                                 HasSubtype (creatureType "Ally"),
                                 HasPossessor ControllerAx You]))
                     (PtUp (LetterVal X)) (PtUp (LetterVal X))
                     (Just Macros.untilEndOfTurn)
       , Define X (DistinctCount ColorAxis (Those (TypeW Creature))) ])

||| Bloom Tender
public export
bloomTenderMana : Ability
bloomTenderMana =
  Macros.abilityWord Vivid
    (Macros.activated TapSymbol
       (ForEachKindOf ColorAxis
          (Just (Macros.allOf (And [Permanent, HasPossessor ControllerAx You]))) Color
          (AddMana You (Lit 1) (OfChosenColor Nothing) [])))

||| Faeburrow Elder's pump
public export
faeburrowElderPump : Ability
faeburrowElderPump =
  Static (Gets Macros.thisCreature
               (PtUp (Times 1 (DistinctCount ColorAxis
                                 (Macros.allOf (And [Permanent, HasPossessor ControllerAx You])))))
               (PtUp (Times 1 (DistinctCount ColorAxis
                                 (Macros.allOf (And [Permanent, HasPossessor ControllerAx You]))))))

||| Tarnation Vista
public export
tarnationVistaMana : Ability
tarnationVistaMana =
  Macros.activated
    (Compound [Mana [Macros.generic 1], TapSymbol])
    (ForEachKindOf ColorAxis
       (Just (Macros.allOf (And [Permanent, Monocolored, HasPossessor ControllerAx You]))) Color
       (AddMana You (Lit 1) (OfChosenColor Nothing) []))

||| Rogues' Gallery
public export
roguesGallery : Card
roguesGallery =
  Macros.card "Rogues' Gallery" (Just [Macros.generic 2, Macros.pip Black]) []
       (MkTypeLine [] [Sorcery])
       [ Spell (ForEachKindOf ColorAxis Nothing Color
                  (Macros.move (Macros.targets (Macros.upTo 1)
                                  (And [Macros.creature, OfChosen Color,
                                        InZone (Macros.graveyardOf You)]))
                               Macros.handZ)) ]
       Nothing

||| Celestial Judgment's pass
public export
celestialJudgmentPass : Effect []
celestialJudgmentPass =
  ForEachKindOf (ValueAxis Power) (Just (Macros.allOf Macros.creature)) Number
    (Macros.choose (Macros.a (And [Macros.creature,
                                   Compare [CharAxis Power] Eq ChosenNumber])))

||| World Queller
public export
worldQuellerChoice : Effect []
worldQuellerChoice =
  (May You (Macros.choose (Macros.a (Macros.quality CardTypeQ))) (Just (Macros.sacrifice (Macros.each AnyPlayer)
                 (Macros.aTheirChoice (And [Permanent, OfChosen CardTypeQ])))) Nothing)

||| Niv-Mizzet, Guildpact
public export
nivMizzetGuildpactTrigger : Ability
nivMizzetGuildpactTrigger =
  Macros.triggered Whenever
    (Macros.dealsCombatDamage Macros.thisCreature (Macros.a AnyPlayer))
    (Sequentially
       [ DealDamage Macros.thisCreature (LetterVal X)
                    (Macros.target Macros.anyTarget)
       , Draw (Macros.target AnyPlayer) (LetterVal X)
       , ChangeLife You (Up (LetterVal X))
       , Define X (DistinctCount ColorPairAxis
                     (Macros.allOf (And [Permanent, HasPossessor ControllerAx You,
                                  ExactlyColors 2]))) ])

||| Tourach, Dread Cantor
public export
tourachDiscardTrigger : Ability
tourachDiscardTrigger =
  Macros.triggered Whenever
    (VerbedEvent (Just Macros.anOpponent) "Discard"
                 (Just (Macros.a (InZone Macros.handZ))) Nothing False)
    (PutCounters (Lit 1) (PrintedKind Macros.plusOnePlusOne) Macros.thisCreature)

||| All-Seeing Arbiter
public export
allSeeingArbiterHeader : GameEvent []
allSeeingArbiterHeader =
  VerbedEvent (Just You) "Discard" (Just (Macros.a (InZone Macros.handZ)))
              Nothing False

||| Mirelurk Queen
public export
mirelurkQueenTrigger : Ability
mirelurkQueenTrigger =
  Macros.triggeredOnlyOnce Whenever
    (VerbedEvent Nothing "Mill"
                 (Just (Macros.counted (Macros.atLeast 1)
                                     (And [Not Macros.land,
                                           InZone (ZoneAt Library Bare)])))
                 Nothing False)
    OncePerTurn
    (Sequentially [ (Macros.draw You (Lit 1))
                  , PutCounters (Lit 1) (PrintedKind Macros.plusOnePlusOne)
                                Macros.thisCreature ])

||| Liliana's Caress
public export
lilianasCaress : Card
lilianasCaress =
  Macros.card "Liliana's Caress"
       (Just [Macros.generic 1, Macros.pip Black]) []
       (MkTypeLine [] [Enchantment])
       [ Macros.triggered Whenever
           (VerbedEvent (Just Macros.anOpponent) "Discard"
                        (Just (Macros.a (InZone Macros.handZ))) Nothing False)
           (ChangeLife They (Down (Lit 2))) ]
       Nothing

||| Scheming Aspirant
public export
schemingAspirant : Card
schemingAspirant =
  Macros.card "Scheming Aspirant"
       (Just [Macros.generic 1, Macros.pip Black]) []
       (MkTypeLine [creatureType "Phyrexian", creatureType "Advisor"]
                   [Creature])
       [ Macros.triggered Whenever
           (VerbedEvent (Just You) "Proliferate" Nothing Nothing False)
           (Sequentially [ Macros.losesLife (Macros.each Opponent) (Lit 2)
                         , Macros.gainsLife You (Lit 2) ]) ]
       (Just (1, 3))

||| Reciprocate
public export
reciprocate : Card
reciprocate =
  Macros.card "Reciprocate" (Just [Macros.pip White]) []
       (MkTypeLine [] [Instant])
       [ Spell (Macros.exile You
                  (Macros.target
                     (And [Macros.creature,
                           Macros.happenedToInvolving DamageDealing
                                                      Lookback.ThisTurn
                                                      You]))) ]
       Nothing

||| Whirling Dervish
public export
whirlingDervish : Card
whirlingDervish =
  Macros.card "Whirling Dervish" (Just [Macros.pip Green, Macros.pip Green]) []
       (MkTypeLine [creatureType "Human", creatureType "Monk"] [Creature])
       [ Macros.keywordQuality "Protection" (ColorIs Black)
       , Macros.triggeredIf At
           (BeginningOf EndStep (Macros.eachPlayers))
           (Macros.happenedInvolving DamageDealing Macros.thisCreature
                                     Lookback.ThisTurn Macros.anOpponent)
           (PutCounters (Lit 1) (PrintedKind Macros.plusOnePlusOne) It) ]
       (Just (1, 1))

||| Military Intelligence
public export
militaryIntelligence : Card
militaryIntelligence =
  Macros.card "Military Intelligence"
       (Just [Macros.generic 1, Macros.pip Blue]) []
       (MkTypeLine [] [Enchantment])
       [ Macros.triggered Whenever
           (AttacksWith You NoDefender
                        (Macros.counted (Macros.atLeast 2) Macros.creature))
           (Sequentially
              [ Macros.searchLibraryOrGraveyard
                  (And [Macros.artifact,
                        Compare [CharAxis ManaValue] AtMost (Lit 2)]) ]) ]
       Nothing

||| Aurelia, the Law Above
public export
aureliaTheLawAbove : Card
aureliaTheLawAbove =
  Macros.card "Aurelia, the Law Above"
       (Just [Macros.generic 3, Macros.pip Red, Macros.pip White]) [Legendary]
       (MkTypeLine [creatureType "Angel"] [Creature])
       [ Macros.keyword "Flying"
       , Macros.keyword "Vigilance"
       , Macros.keyword "Haste"
       , Macros.triggered Whenever
           (AttacksWith (Macros.a AnyPlayer) NoDefender
                        (Macros.counted (Macros.atLeast 3) Macros.creature))
           (Macros.draw You (Lit 1))
       , Macros.triggered Whenever
           (AttacksWith (Macros.a AnyPlayer) NoDefender
                        (Macros.counted (Macros.atLeast 5) Macros.creature))
           (Sequentially [ DealDamage This (Lit 3) (Macros.each Opponent)
                         , Macros.gainsLife You (Lit 3) ]) ]
       (Just (4, 4))

||| Tahngarth
public export
tahngarthHeader : GameEvent []
tahngarthHeader =
  AttacksWith Macros.anOpponent NoDefender
              (Macros.counted (Macros.atLeast 1) Macros.creature)

||| Myth Unbound
public export
mythUnboundTrigger : Ability
mythUnboundTrigger =
  Macros.triggered Whenever
    (Macros.putIntoFrom Macros.yourCommander Macros.commandZ FromAnywhere)
    (Macros.draw You (Lit 1))

||| Commander's Insignia
public export
commandersInsignia : Card
commandersInsignia =
  Macros.card "Commander's Insignia"
       (Just [Macros.generic 2, Macros.pip White, Macros.pip White]) []
       (MkTypeLine [] [Enchantment])
       [ Static (Gets (Macros.allOf Macros.creatureYouControl)
                      (PtUp (Macros.eventCountFrom SpellCast You
                               Lookback.ThisGame Macros.yourCommander
                               (FromZone [Macros.commandZ])))
                      (PtUp (Macros.eventCountFrom SpellCast You
                               Lookback.ThisGame Macros.yourCommander
                               (FromZone [Macros.commandZ])))) ]
       Nothing

||| Jem Lightfoote, Sky Explorer
public export
jemLightfooteSkyExplorer : Card
jemLightfooteSkyExplorer =
  Macros.card "Jem Lightfoote, Sky Explorer"
       (Just [Macros.generic 2, Macros.pip White, Macros.pip Blue]) [Legendary]
       (MkTypeLine [creatureType "Human", creatureType "Scout"] [Creature])
       [ Macros.keyword "Flying"
       , Macros.keyword "Vigilance"
       , Macros.triggeredIf At (BeginningOf EndStep (Macros.yours))
           (NotCond (Macros.happenedFrom SpellCast You Lookback.ThisTurn
                       (Macros.a Macros.spell)
                       (FromZone [Macros.handOf You])))
           (Sequentially
              [ Macros.searchLibraryOrGraveyard
                  (And [Macros.artifact,
                        Compare [CharAxis ManaValue] AtMost (Lit 2)]) ]) ]
       (Just (3, 3))


||| Faith's Reward
public export
faithsReward : Card
faithsReward =
  Macros.card "Faith's Reward"
       (Just [Macros.generic 3, Macros.pip White]) []
       (MkTypeLine [] [Instant])
       [ Spell (Macros.move
                  (Macros.allOf (And [Permanent,
                               InZone (Macros.graveyardOf You),
                               HappenedTo Placement Lookback.ThisTurn
                                 (Just (FromZones
                                          (FromZone [Macros.battlefieldZ])
                                          Nothing))]))
                  Macros.battlefieldZ) ]
       Nothing

||| Ichor Shade
public export
ichorShade : Card
ichorShade =
  Macros.card "Ichor Shade"
       (Just [Macros.generic 2, Macros.pip Black]) []
       (MkTypeLine [creatureType "Phyrexian", creatureType "Shade"] [Creature])
       [ Macros.triggeredIf At (BeginningOf EndStep (Macros.yours))
           (Happened Placement
                     (Macros.a (Or [Macros.artifact, Macros.creature]))
                     Lookback.ThisTurn
                     (Just (IntoZone Macros.graveyardZ
                              (Just (FromZones
                                       (FromZone [Macros.battlefieldZ])
                                       Nothing)))))
           (PutCounters (Lit 1) (PrintedKind Macros.plusOnePlusOne)
                        Macros.thisCreature) ]
       (Just (2, 3))

||| Asmira, Holy Avenger
public export
asmiraHolyAvenger : Card
asmiraHolyAvenger =
  Macros.card "Asmira, Holy Avenger"
       (Just [Macros.generic 2, Macros.pip Green, Macros.pip White]) [Legendary]
       (MkTypeLine [creatureType "Human", creatureType "Cleric"] [Creature])
       [ Macros.keyword "Flying"
       , Macros.triggered At (BeginningOf EndStep (Macros.eachPlayers))
           (PutCounters (EventCount Placement (Macros.a Macros.creature)
                           Lookback.ThisTurn
                           (Just (IntoZone (Macros.graveyardOf You)
                                    (Just (FromZones
                                             (FromZone [Macros.battlefieldZ])
                                             Nothing)))))
                        (PrintedKind Macros.plusOnePlusOne)
                        Macros.thisCreature) ]
       (Just (2, 3))

||| Syr Konrad, the Grim
public export
syrKonradTheGrim : Card
syrKonradTheGrim =
  Macros.card "Syr Konrad, the Grim"
       (Just [Macros.generic 3, Macros.pip Black, Macros.pip Black]) [Legendary]
       (MkTypeLine [creatureType "Human", creatureType "Knight"] [Creature])
       [ Macros.triggeredOr Whenever
           (Dies (Macros.a (And [Macros.creature, OtherThan This])))
           [ Macros.putIntoFrom (Macros.a Macros.creature) Macros.graveyardZ
               (FromAnywhereBut [Macros.battlefieldZ])
           , Macros.leavesZone
               (Macros.a (And [Macros.creature,
                               InZone (Macros.graveyardOf You)]))
               (Macros.graveyardOf You) ]
           (DealDamage This (Lit 1) (Macros.each Opponent))
       , Macros.activated (Mana [Macros.generic 1, Macros.pip Black])
           (Macros.mills (Macros.each AnyPlayer) (Lit 1) (Macros.each AnyPlayer)) ]
       (Just (5, 4))

||| Oscorp Industries
public export
oscorpIndustriesReturn : Ability
oscorpIndustriesReturn =
  Macros.triggered When
    (Enters Macros.thisLand (Just (FromZone [Macros.graveyardZ])))
    (Macros.losesLife You (Lit 2))

public export
theLostAndTheDamnedEntryArm : GameEvent []
theLostAndTheDamnedEntryArm =
  Enters (Macros.a (And [Macros.land, HasPossessor ControllerAx You]))
         (Just (FromAnywhereBut [Macros.handOf You]))


||| Gnarlback Rhino
public export
gnarlbackRhino : Card
gnarlbackRhino =
  Macros.card "Gnarlback Rhino"
       (Just [Macros.generic 2, Macros.pip Green, Macros.pip Green]) []
       (MkTypeLine [creatureType "Rhino"] [Creature])
       [ Macros.keyword "Trample"
       , Macros.triggered Whenever
           (Casts You (Macros.a (And [Macros.spell,
                                      Targets Macros.thisCreature SomeTarget])) Nothing)
           (Sequentially
              [ Macros.searchLibraryOrGraveyard
                  (And [Macros.artifact,
                        Compare [CharAxis ManaValue] AtMost (Lit 2)]) ]) ]
       (Just (4, 4))

||| Forsaken Wastes
public export
forsakenWastesTargeted : Ability
forsakenWastesTargeted =
  Macros.triggered Whenever
    (BecomesTarget Macros.thisEnchantment (Macros.a Macros.spell))
    (Macros.losesLife (Macros.controllerOf (That SpellW)) (Lit 5))

||| Fblthp, the Lost
public export
fblthp : Card
fblthp =
  Macros.card "Fblthp, the Lost" (Just [Macros.generic 1, Macros.pip Blue])
       [Legendary]
       (MkTypeLine [creatureType "Homunculus"] [Creature])
       [ Macros.triggered When (Enters This Nothing)
           (InsteadOf (Macros.draw You (Lit 1))
              (If (OrCond
                     [ Happened Entry It Triggering
                         (Just (FromZones (FromZone [Macros.yourLibrary]) Nothing))
                     , Matches It (CastFrom Macros.yourLibrary) ])
                  (Draw You (Lit 2))
                  Nothing))
       , Macros.triggered When (BecomesTarget This (Macros.a Macros.spell))
           (Macros.shuffleInto You This) ]
       (Just (1, 1))

||| Squelch
public export
squelch : Card
squelch =
  Macros.card "Squelch" (Just [Macros.generic 1, Macros.pip Blue]) []
       (MkTypeLine [] [Instant])
       [ Spell (Sequentially
                  [ Macros.counterSpell
                      (Macros.target (AbilityHead AnyActivated))
                  , (Macros.draw You (Lit 1)) ]) ]
       Nothing

||| Diplomatic Escort
public export
diplomaticEscortLine : Ability
diplomaticEscortLine =
  Macros.activated (Compound [ Mana [Macros.pip Blue]
                             , TapSymbol
                             , Do ((Macros.discard You (Macros.a (InZone Macros.handZ)))) ])
    (Macros.counterSpell
       (Macros.target
          (And [ Joined Macros.spell (AbilityHead AnyOnStack)
               , Targets (Macros.a Macros.creature) SomeTarget ])))

||| Shimmering Glasskite
public export
shimmeringGlasskite : Card
shimmeringGlasskite =
  Macros.card "Shimmering Glasskite"
       (Just [Macros.generic 3, Macros.pip Blue]) []
       (MkTypeLine [creatureType "Spirit"] [Creature])
       [ Macros.keyword "Flying"
       , Macros.triggeredOnlyOnce Whenever
           (BecomesTarget Macros.thisCreature
              (Macros.a (Joined Macros.spell (AbilityHead AnyOnStack))))
           OncePerTurn
           (Macros.counterSpell (That AbilityJoinW)) ]
       (Just (2, 3))

||| Frost Walker
public export
frostWalker : Card
frostWalker =
  Macros.card "Frost Walker"
       (Just [Macros.generic 1, Macros.pip Blue]) []
       (MkTypeLine [creatureType "Elemental"] [Creature])
       [ Macros.triggered When
           (BecomesTarget Macros.thisCreature
              (Macros.a (Joined Macros.spell (AbilityHead AnyOnStack))))
           (Macros.sacrificeIt You) ]
       (Just (4, 1))

||| Destructive Revelry
public export
destructiveRevelry : Card
destructiveRevelry =
  Macros.card "Destructive Revelry" (Just [Macros.pip Red, Macros.pip Green]) []
       (MkTypeLine [] [Instant])
       [ Spell (Sequentially
                  [ Macros.destroy (Macros.target (Or [Macros.artifact, Macros.enchantment]))
                  , DealDamage This (Lit 2) (Macros.controllerOf (That PermanentW)) ]) ]
       Nothing

||| Bioplasm
public export
bioplasmAfterExile : Bindings
bioplasmAfterExile =
  effIntro {bs = eventAfter {bs = []} (Attacks Macros.thisCreature NoDefender)}
           (Macros.exile You Macros.topCard)

public export
bioplasmTwoCandidates : countOnes Object Cards.bioplasmAfterExile = 2
bioplasmTwoCandidates = Refl

public export
bioplasmCardTest : Condition Cards.bioplasmAfterExile
bioplasmCardTest = Macros.itsACard Macros.creature

||| Fuming Effigy
public export
fumingEffigy : Card
fumingEffigy =
  Macros.card "Fuming Effigy"
       (Just [Macros.generic 3, Macros.pip Red]) []
       (MkTypeLine [creatureType "Spirit"] [Creature])
       [ Macros.triggered Whenever
           (Macros.leavesZone
              (Macros.counted (Macros.atLeast 1) (InZone (Macros.graveyardOf You)))
              (Macros.graveyardOf You))
           (DealDamage This (Lit 1) (Macros.each Opponent)) ]
       (Just (4, 3))

public export
theFallenUpkeep : Ability
theFallenUpkeep =
  Macros.triggered At (BeginningOf Upkeep (Macros.yours))
    (DealDamage This (Lit 1)
       (Macros.each (And [ Joined (HasType Planeswalker) Opponent
                  , HappenedTo DamageTaken ThisGame (Just (Involving This)) ])))



||| Loaming Shaman
public export
loamingShaman : Card
loamingShaman =
  Macros.card "Loaming Shaman"
       (Just [Macros.generic 2, Macros.pip Green]) []
       (MkTypeLine [creatureType "Centaur", creatureType "Shaman"] [Creature])
       [ Macros.triggered When (Enters Macros.thisCreature Nothing)
           (Macros.shuffleInto (Macros.target AnyPlayer)
              (Macros.targets Macros.anyNumber
                 (InZone (Macros.graveyardOf They)))) ]
       (Just (3, 2))

||| Blessed Respite
public export
blessedRespiteShuffle : Effect []
blessedRespiteShuffle =
  Macros.shuffleInto (Macros.target AnyPlayer)
    (Macros.allOf (InZone (Macros.graveyardOf They)))



||| Conqueror's Pledge
public export
conquerorsPledge : Card
conquerorsPledge =
  Macros.card "Conqueror's Pledge"
       (Just [Macros.generic 2, Macros.pip White, Macros.pip White, Macros.pip White]) []
       (MkTypeLine [] [Sorcery])
       [ Macros.keywordCosting "Kicker" (Mana [Macros.generic 6])
       , Spell (InsteadOf
                  (Macros.create (Lit 6)
                     (Macros.creatureTok 1 1 [White]
                        [creatureType "Kor", creatureType "Soldier"]))
                  (If (Matches This (PaidCost (ByKeyword "Kicker") Nothing))
                      (Create You (Lit 12) TokenAsThose [])
                      Nothing)) ]
       Nothing

||| Prismari Pianist
public export
prismariPianist : Card
prismariPianist =
  Macros.card "Prismari Pianist"
       (Just [Macros.generic 1, Macros.pip Red, Macros.pip Red]) []
       (MkTypeLine [creatureType "Djinn", creatureType "Bard"] [Creature])
       [ Macros.triggered Whenever
           (Casts You (Macros.a (And [Macros.instantOrSorcery, Macros.spell])) Nothing)
           (InsteadOf
              (Create You (Lit 1)
                 (TokenWritten (Macros.creatureTok 1 1 [Blue, Red]
                                  [creatureType "Elemental"])) [])
              (If (CompareAmt (StatOf ManaValue (That SpellW)) AtLeast (Lit 5))
                  (Create You (Lit 3) TokenAsThose [])
                  Nothing)) ]
       (Just (2, 1))

||| Moonlit Meditation
public export
moonlitMeditation : Card
moonlitMeditation =
  Macros.card "Moonlit Meditation" (Just [Macros.generic 2, Macros.pip Blue]) []
       (MkTypeLine [enchantmentType "Aura"] [Enchantment])
       [ Macros.keywordSubject "Enchant"
           (And [Or [Macros.artifact, Macros.creature], HasPossessor ControllerAx You])
       , Static (Intercepts
                   (TokensCreated (Macros.counted (Macros.atLeast 1) IsToken)
                                  Nothing (Just You) Nothing)
                   [] Nothing
                   (Macros.may You
                        (Create You GroupSize
                                (TokenCopyOf (AttachHost Enchanted PermanentW) []) []))
                   Repeatedly (Just OncePerTurn)) ]
       Nothing

||| Damn
public export
damnDestroyLine : Effect []
damnDestroyLine =
  CantBe (Macros.destroy (Macros.target Macros.creature))
         "Regenerate" (Macros.theVerbedThisWay "Destroy" (TypeW Creature))

||| Nekrataal
public export
nekrataalRider : Bindings
nekrataalRider =
  riderIntro {bs = eventAfter {bs = []} (Enters Macros.thisCreature Nothing)}
    (Macros.destroy (Macros.target
       (And [Macros.creature, Not Macros.artifact, Not (ColorIs Black)])))

public export
nekrataalOneCreatureWord : countReach (Word (TypeW Creature)) OneOf Cards.nekrataalRider = 1
nekrataalOneCreatureWord = Refl

public export
nekrataalTwoObjects : countOnes Object Cards.nekrataalRider = 2
nekrataalTwoObjects = Refl

public export
nekrataalOneDestroyed : countReach (Stamped "Destroy") OneOf Cards.nekrataalRider = 1
nekrataalOneDestroyed = Refl

public export
nekrataalWhole : Card
nekrataalWhole =
  Macros.card "Nekrataal" (Just [Macros.generic 2, Macros.pip Black, Macros.pip Black]) []
       (MkTypeLine [creatureType "Human", creatureType "Assassin"] [Creature])
       [ Macros.keyword "FirstStrike"
       , Macros.triggered When (Enters Macros.thisCreature Nothing)
           (CantBe (Macros.destroy (Macros.target
                      (And [Macros.creature, Not Macros.artifact,
                            Not (ColorIs Black)])))
                   "Regenerate" (That (TypeW Creature))) ]
       (Just (2, 1))

public export
selfSacrificeThenExile : Ability
selfSacrificeThenExile =
  Macros.activated (Do (Macros.sacrifice You Macros.thisArtifact))
    (Sequentially
       [ Macros.exile You (Macros.target Macros.creature)
       , Delayed (BeginningOf EndStep NoPossessor) [] Nothing
                 (Move (That CardW) Macros.battlefieldZ
                       []) ])

public export
sequencedRider : Bindings
sequencedRider =
  riderIntro {bs = []}
    (Sequentially [ Macros.exile You (Macros.target Macros.artifact)
                  , Macros.destroy (Macros.target Macros.creature) ])

public export
sequencedRiderOneDestroyed : countReach (Stamped "Destroy") OneOf Cards.sequencedRider = 1
sequencedRiderOneDestroyed = Refl

public export
sequencedRiderOneExiled : countReach (Stamped "Exile") OneOf Cards.sequencedRider = 1
sequencedRiderOneExiled = Refl

||| Bonds of Faith
public export
bondsOfFaithPump : Ability
bondsOfFaithPump =
  Static (Macros.onlyWhile
            (Gets (AttachHost Enchanted (TypeW Creature)) (PtUp (Lit 2)) (PtUp (Lit 2)))
            (Matches It (HasSubtype (creatureType "Human"))))

public export
discardUpToTwoThenDrawThatMany : Effect []
discardUpToTwoThenDrawThatMany =
  Sequentially [ (Repeated (UpTo (Lit 2)) (Sequentially [Macros.choose (Macros.a (InZone Macros.handZ)), Macros.discard You (That CardW)]))
               , Draw You GroupSize ]

||| Truce and Temporary Truce
public export
drawUpToTwoThenGainPerShortfall : Effect []
drawUpToTwoThenGainPerShortfall =
  Sequentially [ Macros.may (Macros.each AnyPlayer) (Draw They (UpTo (Lit 2)))
               , Macros.gainsLife They (Times 2 ShortOfCeiling) ]

||| Soul of Emancipation
public export
soulOfEmancipation : Ability
soulOfEmancipation =
  Macros.triggered When (Enters Macros.thisCreature Nothing)
    (Sequentially
       [ Macros.destroy (Macros.targets (Macros.upTo 3)
                           (And [Permanent, Not Macros.land, OtherThan This]))
       , ForEachOf (Macros.thoseVerbedThisWay "Destroy" PermanentW)
                   (Create (Macros.controllerOf (ItVerbed "Destroy")) (Lit 1)
                           (TokenWritten
                              (MkToken (Just (Lit 3 ** Lit 3)) [White]
                                       (MkTypeLine [creatureType "Angel"] [Creature])
                                       [Macros.keyword "Flying"] Nothing))
                           []) ])

||| Engulfing Flames
public export
engulfingFlamesRider : Bindings
engulfingFlamesRider =
  riderIntro {bs = []} (DealDamage This (Lit 1) (Macros.target Macros.creature))

public export
engulfingFlamesNoDestroyStamp :
  countReach (Stamped "Destroy") OneOf Cards.engulfingFlamesRider = 0
engulfingFlamesNoDestroyStamp = Refl

public export
engulfingFlamesBareReadStands :
  countReach Bare OneOf Cards.engulfingFlamesRider = 1
engulfingFlamesBareReadStands = Refl

public export
bioplasmExiledCard : Noun Cards.bioplasmAfterExile Object
bioplasmExiledCard = Macros.theVerbed "Exile" CardW

public export
bioplasmExiledPronoun : Noun Cards.bioplasmAfterExile Object
bioplasmExiledPronoun = Macros.itVerbed "Exile"

public export
bioplasmNoTypedRead :
  countReach (Verbed "Exile" (TypeW Creature) Attributive) OneOf
    Cards.bioplasmAfterExile = 0
bioplasmNoTypedRead = Refl

public export
bioplasmExiledCardHasNoType :
  tyOfReach (Stamped "Exile") OneOf Cards.bioplasmAfterExile = Nothing
bioplasmExiledCardHasNoType = Refl

public export
bioplasmAfterTest : Bindings
bioplasmAfterTest = condIntro Cards.bioplasmCardTest

public export
bioplasmTestRemarksType :
  tyOfReach (Stamped "Exile") OneOf Cards.bioplasmAfterTest = Just Creature
bioplasmTestRemarksType = Refl

public export
bioplasmTestMintsNothing : countOnes Object Cards.bioplasmAfterTest = 2
bioplasmTestMintsNothing = Refl

public export
bioplasmTestKeepsCardSlot :
  countReach (AtSlot CardSlot) OneOf Cards.bioplasmAfterTest = 1
bioplasmTestKeepsCardSlot = Refl

public export
bioplasmTypedReadStillRefused :
  countReach (Verbed "Exile" (TypeW Creature) Attributive) OneOf
    Cards.bioplasmAfterTest = 0
bioplasmTypedReadStillRefused = Refl

public export
bioplasmTypedCardReadWrites :
  countReach (Verbed "Exile" (TypedCardW Creature) Attributive) OneOf
    Cards.bioplasmAfterTest = 1
bioplasmTypedCardReadWrites = Refl

||| Scapeshift
public export
scapeshiftSacrificed : Bindings
scapeshiftSacrificed =
  effIntro {bs = []}
           (Macros.sacrifice You (Macros.counted Macros.anyNumber Macros.land))

public export
scapeshiftAfterSearch : Bindings
scapeshiftAfterSearch =
  effIntro {bs = Cards.scapeshiftSacrificed}
           (Macros.searchLibraryForCount (UpToOf GroupSize) Macros.land)

public export
scapeshiftTwoGroups : countReach Bare ManyOf Cards.scapeshiftAfterSearch = 2
scapeshiftTwoGroups = Refl

public export
scapeshiftOneSearchedGroup :
  countReach (Stamped "Search") ManyOf Cards.scapeshiftAfterSearch = 1
scapeshiftOneSearchedGroup = Refl

public export
scapeshiftOneSacrificedGroup :
  countReach (Stamped "Sacrifice") ManyOf Cards.scapeshiftAfterSearch = 1
scapeshiftOneSacrificedGroup = Refl

||| Scapeshift
public export
scapeshift : Card
scapeshift =
  Macros.card "Scapeshift"
       (Just [Macros.generic 2, Macros.pip Green, Macros.pip Green]) []
       (MkTypeLine [] [Sorcery])
       [ Spell (Sequentially
                  [ Macros.sacrifice You (Macros.counted Macros.anyNumber
                                                       Macros.land)
                  , Macros.searchLibraryForCount (UpToOf GroupSize) Macros.land
                  , Macros.putOntoBattlefieldTapped (Macros.themVerbed "Search")
                  , Macros.shuffle ]) ]
       Nothing


||| Collected Company
public export
collectedCompany : Card
collectedCompany =
  Macros.card "Collected Company" (Just [Macros.generic 3, Macros.pip Green]) []
       (MkTypeLine [] [Instant])
       [ Spell (Sequentially
           [ Macros.lookAt (Macros.topCards 6)
           , Macros.move (Macros.fromAmong (Macros.upTo 2)
                                           (And [Macros.creature,
                                                 Compare [CharAxis ManaValue] AtMost (Lit 3)])
                                           Them)
                         Macros.battlefieldZ
           , Macros.move TheRest (Macros.onBottomIn AnyOrder) ]) ]
       Nothing

||| Commune with the Gods
public export
communeWithTheGods : Card
communeWithTheGods =
  Macros.card "Commune with the Gods" (Just [Macros.generic 1, Macros.pip Green]) []
       (MkTypeLine [] [Sorcery])
       [ Spell (Sequentially
           [ Macros.revealCards (Macros.topCards 5)
           , Macros.may You
               (Macros.move (Macros.oneFromAmong (Or [Macros.creature, Macros.enchantment])
                                                 Them)
                            Macros.handZ)
           , Macros.move TheRest Macros.graveyardZ ]) ]
       Nothing

||| Tezzeret's Gatebreaker
public export
tezzeretsGatebreaker : Card
tezzeretsGatebreaker =
  Macros.card "Tezzeret's Gatebreaker" (Just [Macros.generic 4]) []
       (MkTypeLine [] [Artifact])
       [ Macros.triggered When (Enters Macros.thisArtifact Nothing)
           (Sequentially
              [ Macros.lookAt (Macros.topCards 5)
              , Macros.may You
                  (Sequentially
                     [ Macros.revealCards
                         (Macros.oneFromAmong (Or [ColorIs Blue, Macros.artifact]) Them)
                     , Macros.move (That CardW) Macros.handZ ])
              , Macros.move TheRest (Macros.onBottomIn RandomOrder) ])
       , Macros.activated
           (Compound [Mana [Macros.generic 5, Macros.pip Blue], TapSymbol,
                      Do (Macros.sacrifice You Macros.thisArtifact)])
           (Macros.cantBeBlocked (Macros.allOf Macros.creatureYouControl)
                                 (Just Macros.thisTurn)) ]
       Nothing

||| Soldevi Adnate
public export
soldeviAdnate : Card
soldeviAdnate =
  Macros.card "Soldevi Adnate" (Just [Macros.generic 1, Macros.pip Black]) []
       (MkTypeLine [creatureType "Human", creatureType "Cleric"] [Creature])
       [ Macros.activated
           (Compound [TapSymbol,
                      Do (Macros.sacrifice You
                            (Macros.a (And [Macros.creature,
                                            Or [ColorIs Black, Macros.artifact]])))])
           (AddMana You (Macros.manaValueOf (Macros.itVerbed "Sacrifice"))
                    (Runs [[OfColor Black]]) []) ]
       (Just (1, 1))

||| Bind to Life, Vastlands Scavenger's adventure
public export
bindToLife : Effect []
bindToLife =
  Sequentially [ Macros.mills You (Lit 7) You
               , Macros.move (Macros.oneFromAmong Macros.creature Them)
                             Macros.battlefieldZ ]

||| Glamdring, Foe-hammer's Gleam of Death
public export
gleamOfDeath : Effect []
gleamOfDeath =
  Sequentially [ Macros.mills You (Lit 6) You
               , Macros.move (Macros.allFromAmong
                                (Or [Macros.instant, Macros.sorcery]) Them)
                             Macros.handZ ]

||| Tezzeret, Master of the Bridge
public export
tezzeretAllArtifacts : Effect []
tezzeretAllArtifacts =
  Sequentially [ Macros.mills You (Lit 6) You
               , Macros.move (Macros.allFromAmong Macros.artifact Them)
                             Macros.battlefieldZ ]

public export
wholeSliceIsPluralAndUncounted :
  (nounPlur (SomeOf {bs = []} WholeSlice (Just Macros.artifact)
                    (LibrarySlice OnTop (Lit 5) You) {gm = Oh}),
   sliceExact (WholeSlice {bs = []})) = (ManyOf, Nothing)
wholeSliceIsPluralAndUncounted = Refl

public export
millThenPutFromAmongMilled : Effect []
millThenPutFromAmongMilled =
  Sequentially [ Macros.mills You (Lit 3) You
               , Macros.may You
                   (Macros.move (Macros.oneFromAmong Macros.artifact
                                   (Macros.thoseVerbedThisWay "Mill" CardW))
                                Macros.handZ) ]

||| Hymn to Tourach
public export
hymnToTourach : Card
hymnToTourach =
  Macros.card "Hymn to Tourach" (Just [Macros.pip Black, Macros.pip Black]) []
       (MkTypeLine [] [Sorcery])
       [ Spell (Macros.discard (Macros.target AnyPlayer)
                                (Macros.countedAtRandom (Macros.exactly 2)
                                                        (InZone Macros.handZ))) ]
       Nothing

||| Tourach, Dread Cantor
public export
tourachDreadCantor : Card
tourachDreadCantor =
  Macros.card "Tourach, Dread Cantor"
       (Just [Macros.generic 1, Macros.pip Black]) [Legendary]
       (MkTypeLine [creatureType "Human", creatureType "Cleric"] [Creature])
       [ Macros.keywordCosting "Kicker" (Mana [Macros.pip Black, Macros.pip Black])
       , Macros.keywordQuality "Protection" (ColorIs White)
       , Cards.tourachDiscardTrigger
       , Macros.triggeredIf When (Enters Macros.thisCreature Nothing)
           (Matches Macros.thisCreature (PaidCost (ByKeyword "Kicker") Nothing))
           (Macros.discard (Macros.target Opponent)
                            (Macros.countedAtRandom (Macros.exactly 2)
                                                    (InZone Macros.handZ))) ]
       (Just (2, 1))

public export
playersTopCardSlice : Noun [] Object
playersTopCardSlice = LibrarySlice OnTop (Lit 1) (PlayerGroup AllPlayers)

public export
playersTopCardIsPlural : nounPlur Cards.playersTopCardSlice = ManyOf
playersTopCardIsPlural = Refl

||| Breeches, Brazen Plunderer's slice
public export
eachOfThoseOpponentsTopCard : Effect []
eachOfThoseOpponentsTopCard =
  Sequentially [ DealDamage This (Lit 1) (Macros.each Opponent)
               , Macros.exile You (LibrarySlice OnTop (Lit 1) (EachOf (Those PlayerW))) ]

public export
companyContext : Bindings
companyContext = Macros.lookedTop [] (Lit 6)

public export
companyDescribedSlice : Noun Cards.companyContext Object
companyDescribedSlice =
  Macros.fromAmong (Macros.upTo 2)
                   (And [Macros.creature, Compare [CharAxis ManaValue] AtMost (Lit 3)])
                   Them

public export
companyBareSlice : Noun Cards.companyContext Object
companyBareSlice = Macros.someOf 2 Them

public export
describedSliceReadsAsCreature : nounTy Cards.companyDescribedSlice = Just Creature
describedSliceReadsAsCreature = Refl

public export
bareSliceReadsUntyped : nounTy Cards.companyBareSlice = Nothing
bareSliceReadsUntyped = Refl

public export
describedSliceKeepsGroupZone :
  nounZone Cards.companyDescribedSlice = nounZone Cards.companyBareSlice
describedSliceKeepsGroupZone = Refl

public export
atRandomModeIsAnnouncementNeutral :
  nounDelta (Macros.countedAtRandom {bs = []} (Macros.exactly 2) (InZone Macros.handZ))
    = nounDelta (Macros.counted {bs = []} ((Macros.exactly 2)) (InZone Macros.handZ))
atRandomModeIsAnnouncementNeutral = Refl

public export
countedAtRandomIsPlural :
  nounPlur (Macros.countedAtRandom {bs = []} (Macros.exactly 2) (InZone Macros.handZ))
    = ManyOf
countedAtRandomIsPlural = Refl

||| Lord of the Void
public export
exileTopThenPutFromAmong : Effect []
exileTopThenPutFromAmong =
  Sequentially [ Macros.exile You (Macros.topCards 7)
               , Macros.move (Macros.oneFromAmong Macros.creature Them)
                             Macros.battlefieldZ ]


||| Vorel of the Hull Clade
public export
vorelOfTheHullClade : Card
vorelOfTheHullClade =
  Macros.card "Vorel of the Hull Clade"
       (Just [Macros.generic 1, Macros.pip Green, Macros.pip Blue]) [Legendary]
       (MkTypeLine [creatureType "Human", creatureType "Merfolk"] [Creature])
       [ Macros.activated
           (Compound [Mana [Macros.pip Green, Macros.pip Blue], TapSymbol])
           (DoubleCounters
              (Macros.target (Or [Macros.artifact, Macros.creature, Macros.land]))) ]
       (Just (1, 4))

||| Deepglow Skate's recipient
public export
deepglowSkateRecipientRefused :
  perMemberOk (Macros.targets {bs = []} Macros.anyNumber Permanent) = False
deepglowSkateRecipientRefused = Refl

||| Aetheric Amplifier
public export
doubleYourOwnCounters : Effect []
doubleYourOwnCounters = DoubleCounters You


||| Thought Sponge
public export
greatestCardsAnOpponentDrew : Amount []
greatestCardsAnOpponentDrew =
  AggregateOver MaxOf Opponent (Macros.eventCount CardDrawn They ThisTurn)

||| Thought Sponge
public export
thoughtSponge : Card
thoughtSponge =
  Macros.card "Thought Sponge" (Just [Macros.generic 3, Macros.pip Blue]) []
       (MkTypeLine [creatureType "Sponge"] [Creature])
       [ Macros.keyword "Flash"
       , Static (Macros.entersWithCounters Macros.thisCreature
                   Cards.greatestCardsAnOpponentDrew Macros.plusOnePlusOne)
       , Macros.triggered When (Dies Macros.thisCreature)
                          (Draw You (StatOf Power Macros.thisCreature)) ]
       (Just (1, 1))

public export
greatestCardsAPlayerDiscardedThisWay : Amount []
greatestCardsAPlayerDiscardedThisWay =
  AggregateOver MaxOf AnyPlayer
    (Macros.eventCountInvolving (VerbedAct "Discard") They ThisWay
       (Macros.allOf {k = Object} (And [])))


public export
eachPlayerBindsNoSingular : countOnes Player (nomIntro (Macros.each {bs = []} AnyPlayer)) = 0
eachPlayerBindsNoSingular = Refl

public export
eachPlayerBindsAGroup : countManys Player (nomIntro (Macros.each {bs = []} AnyPlayer)) = 1
eachPlayerBindsAGroup = Refl

public export
eachPlayerOfferBindsOneMember :
  countOnes Player (mayCtx (Macros.each {bs = []} AnyPlayer)) = 1
eachPlayerOfferBindsOneMember = Refl

public export
eachPlayerOfferDropsTheGroup :
  countManys Player (mayCtx (Macros.each {bs = []} AnyPlayer)) = 0
eachPlayerOfferDropsTheGroup = Refl


||| Prosperity
public export
prosperityCostLetters :
  costLetters (Just [Variable, Macros.pip Blue]) = [letterB X]
prosperityCostLetters = Refl

public export
prosperityTextReadsCostLetter :
  amtDelta (LetterVal X {bs = costLetters (Just [Variable, Macros.pip Blue])}) = []
prosperityTextReadsCostLetter = Refl

public export
textAloneOnceMintedItsOwnLetter :
  amtDelta (LetterVal X {bs = []}) = [letterB X]
textAloneOnceMintedItsOwnLetter = Refl

public export
noVariableSymbolNoLetter :
  costLetters (Just [Macros.generic 1, Macros.pip Blue]) = []
noVariableSymbolNoLetter = Refl

public export
noCostNoLetter : costLetters Nothing = []
noCostNoLetter = Refl




||| Debris Beetle
public export
debrisBeetleTrigger : Ability
debrisBeetleTrigger =
  Macros.triggered When (Enters Macros.thisVehicle Nothing)
    (Sequentially [ Macros.losesLife (Macros.each Opponent) (Lit 3)
                  , Macros.gainsLife You (Lit 3) ])

||| Nautiloid Ship
public export
nautiloidShipTrigger : Ability
nautiloidShipTrigger =
  Macros.triggered Whenever
    (Macros.dealsCombatDamage Macros.thisVehicle (Macros.a AnyPlayer))
    (Macros.may You
       (Macros.putOntoBattlefieldUnderYourControl
          (Macros.a (And [Macros.creature, ExiledWith Macros.thisVehicle]))))

||| Summon: Esper Valigarmanda's II/III/IV body, in part
public export
summonEsperValigarmandaCast : StaticEffect []
summonEsperValigarmandaCast =
  AndAlso [ Macros.mayPlayDeed "Cast" You
                      (Macros.a (And [Macros.instantOrSorcery,
                                      ExiledWith Macros.thisSaga]))
                      (PlayRider Nothing Nothing Nothing False ItsOwnCost)
          , Macros.maySpendAsThough You Nothing MatchAnyType
              (Just (ToCast (And [Macros.instantOrSorcery,
                                  ExiledWith Macros.thisSaga]))) ]

||| Roads Go Ever, Ever On's chapters II and III
public export
roadsGoEverEverOnChapters : Ability
roadsGoEverEverOnChapters =
  Macros.triggered When (ChapterMark [ChapterII, ChapterIII])
    (Macros.move (Macros.a (And [IsCard, ExiledWith Macros.thisSaga]))
                 Macros.handZ)

||| Rogue Class's level-3 body
public export
rogueClassLevelThree : StaticEffect []
rogueClassLevelThree =
  AndAlso [ (Macros.mayPlayDeed "Play" You (Macros.allOf (ExiledWith Macros.thisClass)) (PlayRider Nothing Nothing Nothing False ItsOwnCost))
          , Macros.maySpendAsThough You Nothing MatchAnyColor
              (Just (ToCast (ExiledWith Macros.thisClass))) ]

||| Wurmwall Sweeper
public export
wurmwallSweeperTrigger : Ability
wurmwallSweeperTrigger =
  Macros.triggered When (Enters Macros.thisSpacecraft Nothing) (Macros.surveil You (Lit 2))

||| Case of the Crimson Pulse
public export
caseOfTheCrimsonPulseTrigger : Ability
caseOfTheCrimsonPulseTrigger =
  Macros.triggered When (Enters Macros.thisCase Nothing)
    (Sequentially [(Macros.discard You (Macros.a (InZone Macros.handZ))), (Macros.draw You (Lit 2))])

||| Ferocious Pup
public export
ferociousPup : Card
ferociousPup =
  Macros.card "Ferocious Pup" (Just [Macros.generic 2, Macros.pip Green]) []
       (MkTypeLine [creatureType "Wolf"] [Creature])
       [ Macros.triggered When (Enters Macros.thisCreature Nothing)
           (Macros.create (Lit 1) (Macros.creatureTok 2 2 [Green] [creatureType "Wolf"])) ]
       (Just (0, 1))

||| Talrand's Invocation
public export
talrandsInvocation : Effect []
talrandsInvocation =
  Macros.create (Lit 2)
    (MkToken (Just (Lit 2 ** Lit 2)) [Blue] (MkTypeLine [creatureType "Drake"] [Creature])
             [Macros.keyword "Flying"] Nothing)

||| Predatory Wurm
public export
predatoryWurm : Card
predatoryWurm =
  Macros.card "Predatory Wurm" (Just [Macros.generic 3, Macros.pip Green]) []
       (MkTypeLine [creatureType "Wurm"] [Creature])
       [ Macros.keyword "Vigilance"
       , Static (Macros.asLongAs
                   (Macros.exists (And [HasSubtype (planeswalkerType "Garruk"), HasPossessor ControllerAx You]))
                   (Gets Macros.thisCreature (PtUp (Lit 2)) (PtUp (Lit 2)))) ]
       (Just (4, 4))

||| Skeleton Ship
public export
skeletonShip : Card
skeletonShip =
  Macros.card "Skeleton Ship"
       (Just [Macros.generic 3, Macros.pip Blue, Macros.pip Black]) [Legendary]
       (MkTypeLine [creatureType "Skeleton"] [Creature])
       [ Macros.triggered When
           (Macros.whenState
              (NotCond (Macros.exists (And [Macros.land,
                                     HasSubtype (landType "Island"),
                                     HasPossessor ControllerAx You]))))
           (Macros.sacrifice You Macros.thisCreature)
       , Macros.activated TapSymbol
           (PutCounters (Lit 1) (PrintedKind Macros.minusOneMinusOne)
                        (Macros.target Macros.creature)) ]
       (Just (0, 3))

||| Garruk Relentless
public export
garrukRelentlessFlip : Ability
garrukRelentlessFlip =
  Macros.triggered When
    (Macros.whenState
       (Matches Macros.thisPlaneswalker
                (Compare [CounterAxis (Named "Loyalty")] AtMost (Lit 2))))
    (Macros.transform Macros.thisPlaneswalker)

||| Soul Ransom
public export
soulRansomRansom : Effect []
soulRansomRansom =
  Sequentially [ Macros.sacrificeIt (Macros.controllerOf Macros.thisAura)
               , Draw They (Lit 2) ]

||| Soul Ransom
public export
soulRansom : Card
soulRansom =
  Macros.card "Soul Ransom"
       (Just [Macros.generic 2, Macros.pip Blue, Macros.pip Black]) []
       (MkTypeLine [enchantmentType "Aura"] [Enchantment])
       [ Macros.keywordSubject "Enchant" Macros.creature
       , Static (GainsControl You (AttachHost Enchanted (TypeW Creature)))
       , Macros.activatedBy (Do ((Repeated (Lit 2) (Sequentially [Macros.choose (Macros.a (InZone Macros.handZ)), Macros.discard You (That CardW)]))))
           (Sequentially [ Macros.sacrificeIt (Macros.controllerOf Macros.thisAura)
                         , Draw They (Lit 2) ])
           (PlayerGroup YourOpponents) ]
       Nothing

public export
possessiveDeicticIsReadableByIt :
  countReach (AtSlot PermanentSlot) OneOf
    (nomIntro (Macros.controllerOf (Macros.thisAura {bs = []}))) = 1
possessiveDeicticIsReadableByIt = Refl

public export
possessiveDeicticIsNotADemonstrative :
  countReach (Word (TypeW Enchantment)) OneOf
    (nomIntro (Macros.controllerOf (Macros.thisAura {bs = []}))) = 0
possessiveDeicticIsNotADemonstrative = Refl

public export
possessiveDescribedBaseUnchanged :
  countReach (AtSlot PermanentSlot) OneOf
    (nomIntro (Macros.controllerOf (Macros.target Macros.creature {bs = []}))) = 1
possessiveDescribedBaseUnchanged = Refl

||| Contractual Safeguard
public export
contractualSafeguardPass : Effect []
contractualSafeguardPass =
  Sequentially
    [ Macros.choose (Macros.a (CounterKindOn (Macros.a Macros.creatureYouControl)))
    , PutCounters (Lit 1) BoundKind
        (Macros.each (Macros.otherCreatureYouControl It)) ]


||| Bile Blight
public export
bileBlight : Card
bileBlight =
  Macros.card "Bile Blight" (Just [Macros.pip Black, Macros.pip Black]) []
       (MkTypeLine [] [Instant])
       [ Spell (Macros.gets
                  (Both (Macros.target Macros.creature)
                          (Macros.allOf (And [ Macros.creature
                                      , Named (SameNameAs (That (TypeW Creature)))
                                      , OtherThan (That (TypeW Creature)) ])))
                  (PtDown (Lit 3)) (PtDown (Lit 3))
                  (Just Macros.untilEndOfTurn)) ]
       Nothing

||| Echoing Ruin
public export
echoingRuin : Card
echoingRuin =
  Macros.card "Echoing Ruin" (Just [Macros.generic 1, Macros.pip Red]) []
       (MkTypeLine [] [Sorcery])
       [ Spell (Macros.destroy
                  (Both (Macros.target Macros.artifact)
                          (Macros.allOf (And [ Macros.artifact
                                      , Named (SameNameAs (That (TypeW Artifact)))
                                      , OtherThan (That (TypeW Artifact)) ])))) ]
       Nothing

||| Stomp and Howl
public export
stompAndHowl : Card
stompAndHowl =
  Macros.card "Stomp and Howl" (Just [Macros.generic 2, Macros.pip Green]) []
       (MkTypeLine [] [Sorcery])
       [ Spell (Macros.destroy (Both (Macros.target Macros.artifact)
                                       (Macros.target Macros.enchantment))) ]
       Nothing

||| Churning Eddy
public export
churningEddy : Card
churningEddy =
  Macros.card "Churning Eddy" (Just [Macros.generic 4, Macros.pip Blue]) []
       (MkTypeLine [] [Instant])
       [ Spell (Macros.move (Both (Macros.target Macros.creature)
                                    (Macros.target Macros.land))
                            Macros.handZ) ]
       Nothing

||| Secret Rendezvous
public export
secretRendezvous : Card
secretRendezvous =
  Macros.card "Secret Rendezvous"
       (Just [Macros.generic 1, Macros.pip White, Macros.pip White]) []
       (MkTypeLine [] [Sorcery])
       [ Spell (Draw (EachOf (Both You (Macros.target Opponent))) (Lit 3)) ]
       Nothing

||| Mana Clash
public export
manaClashFlip : Effect []
manaClashFlip =
  FlipCoins (EachOf (Both You (Macros.target Opponent))) (FlipCount (Lit 1))

||| Weftwalking
public export
weftwalkingShuffle : Effect []
weftwalkingShuffle =
  Sequentially
    [ Macros.shuffleInto You (Both (Macros.allOf (InZone (Macros.handOf You)))
                                 (Macros.allOf (InZone (Macros.graveyardOf You))))
    , (Macros.draw You (Lit 7)) ]

||| Sugar Coat
public export
sugarCoat : Card
sugarCoat =
  Macros.card "Sugar Coat" (Just [Macros.generic 2, Macros.pip White]) []
       (MkTypeLine [enchantmentType "Aura"] [Enchantment])
       [ Macros.keyword "Flash"
       , Macros.keywordSubject "Enchant"
           (Or [HasType Creature, HasSubtype (artifactType "Food")])
       , Static (AndAlso
           [ Becomes (AttachHost Enchanted PermanentW) Sets (Bundle (MkToken Nothing []
                               (MkTypeLine [artifactType "Food"] [Artifact])
                               [ Macros.activated
                                   (Compound [ Mana [Macros.generic 2]
                                             , TapSymbol
                                             , Do (Macros.sacrifice You Macros.thisArtifact) ])
                                   (Macros.gainsLife You (Lit 3)) ]
                               Nothing) Nothing)
           , LosesAllAbilities It Nothing ]) ]
       Nothing

||| Doc Aurlock, Grizzled Genius
public export
docAurlockCost : StaticEffect []
docAurlockCost =
  CostsToCast (Macros.allOf (And [Macros.spell, CastBy You,
                           Or [ CastFrom (Macros.graveyardOf You)
                              , CastFrom Macros.exileZ ]]))
              (CostLess (Lit 2) Nothing)


||| Agency Outfitter's search
public export
agencyOutfitterSearch : Effect []
agencyOutfitterSearch =
  Sequentially
    [ Macros.searchZonesOf You
        (Or [ Named (PrintedName "Magnifying Glass")
            , Named (PrintedName "Thinking Cap") ])
    , If (Macros.happenedAt (VerbedAct "Search") You Lookback.ThisWay
                            Macros.yourLibrary)
         Macros.shuffle Nothing ]

||| Delivery Moogle
public export
deliveryMoogle : Card
deliveryMoogle =
  Macros.card "Delivery Moogle"
       (Just [Macros.generic 3, Macros.pip White]) []
       (MkTypeLine [creatureType "Moogle"] [Creature])
       [ Macros.keyword "Flying"
       , Macros.triggered When (Enters Macros.thisCreature Nothing)
           (Sequentially
              [ Macros.searchLibraryOrGraveyard
                  (And [Macros.artifact,
                        Compare [CharAxis ManaValue] AtMost (Lit 2)])
              , Macros.revealsIt
              , Macros.move Macros.foundCard Macros.handZ
              , If (Macros.happenedAt (VerbedAct "Search") You Lookback.ThisWay
                                      Macros.yourLibrary)
                   Macros.shuffle Nothing ]) ]
       (Just (3, 2))

||| Tower Winder
public export
towerWinder : Card
towerWinder =
  Macros.card "Tower Winder" (Just [Macros.generic 1, Macros.pip Green]) []
       (MkTypeLine [creatureType "Snake"] [Creature])
       [ Macros.keyword "Reach"
       , Macros.keyword "Deathtouch"
       , Macros.triggered When (Enters Macros.thisCreature Nothing)
           (Sequentially
              [ Macros.searchLibraryOrGraveyard
                  (Named (PrintedName "Command Tower"))
              , Macros.revealsIt
              , Macros.move Macros.foundCard Macros.handZ
              , If (Macros.happenedAt (VerbedAct "Search") You Lookback.ThisWay
                                      Macros.yourLibrary)
                   Macros.shuffle Nothing ]) ]
       (Just (1, 1))

||| Vraska's Scorn
public export
vraskasScorn : Card
vraskasScorn =
  Macros.card "Vraska's Scorn"
       (Just [Macros.generic 2, Macros.pip Black, Macros.pip Black]) []
       (MkTypeLine [] [Sorcery])
       [ Spell (Sequentially
           [ Macros.losesLife (Macros.target Opponent) (Lit 4)
           , Macros.may You
               (Sequentially
                  [ Macros.searchLibraryOrGraveyard
                      (Named (PrintedName "Vraska, Scheming Gorgon"))
                  , Macros.revealCards It
                  , Macros.move It Macros.handZ ])
           , If (Macros.happenedAt (VerbedAct "Search") You Lookback.ThisWay
                                   Macros.yourLibrary)
                Macros.shuffle Nothing ]) ]
       Nothing

||| Old-Growth Dryads
public export
oldGrowthDryads : Card
oldGrowthDryads =
  Macros.card "Old-Growth Dryads" (Just [Macros.pip Green]) []
       (MkTypeLine [creatureType "Dryad"] [Creature])
       [ Macros.triggered When (Enters Macros.thisCreature Nothing)
           (Macros.may (Macros.each Opponent)
              (Sequentially
                 [ Macros.playerSearchesTheirLibraryFor They
                     (And [Macros.land, HasSupertype Basic])
                 , Macros.putOntoBattlefieldTapped Macros.foundCard
                 , Shuffle They ])) ]
       (Just (3, 3))

||| Concussive Bolt, both paragraphs
public export
concussiveBolt : Effect []
concussiveBolt =
  Sequentially
    [ DealDamage This (Lit 4) Cards.targetPlayerOrPlaneswalker
    , If (CompareAmt (Macros.countOf (And [Macros.artifact, HasPossessor ControllerAx You]))
                     AtLeast (Lit 3))
         (Continuously {ts = StaticFirstDone}
            (Macros.deontic (Macros.allOf (And [Macros.creature,
                                  HasPossessor ControllerAx Macros.splitOverPlaneswalker]))
                     Forbid ["Block"] Agent NoDeonticPatient)
            (Just Macros.thisTurn))
         Nothing ]

||| Trouble in Pairs'
public export
troubleInPairsArms : AbilityAt []
troubleInPairsArms =
  Triggered Whenever
    (AttacksWith Macros.anOpponent (OneDefender You)
                 (Macros.counted (Macros.atLeast 2) Macros.creature))
    [ NthOccurrence (Nth 2) (Just Turn) (Draws (Macros.a Opponent))
    , NthOccurrence (Nth 2) (Just Turn)
        (Casts (Macros.a Opponent) (Macros.a Macros.spell) Nothing) ]
    Nothing [] Nothing Nothing Nothing
    (Draw You (Lit 1))

||| Gaea's Revenge's protection-shaped phrase
public export
nongreenSpellsOrAbilities : Predicate [] (Object \/ Ability)
nongreenSpellsOrAbilities =
  Joined (And [Macros.spell, Not (ColorIs Green)])
         (And [ AbilityHead AnyOnStack
              , AbilityOf (Macros.a (And [Macros.source, Not (ColorIs Green)])) ])



||| Aim High
public export
aimHigh : Card
aimHigh =
  Macros.card "Aim High" (Just [Macros.generic 1, Macros.pip Green]) []
       (MkTypeLine [] [Instant])
       [ Spell (Sequentially
                  [ Macros.untap (Macros.target Macros.creature)
                  , Continuously {ts = StaticFirstDone}
                      (AndAlso [ Gets (ItVerbed "Untap") (PtUp (Lit 2)) (PtUp (Lit 2))
                               , Gains (ItVerbed "Untap") (Macros.keyword "Reach") ])
                      (Just Macros.untilEndOfTurn) ]) ]
       Nothing

||| Hijack
public export
hijack : Card
hijack =
  Macros.card "Hijack" (Just [Macros.generic 1, Macros.pip Red, Macros.pip Red]) []
       (MkTypeLine [] [Sorcery])
       [ Spell (Sequentially
                  [ Macros.gainControl You (Macros.target (Or [Macros.artifact, Macros.creature]))
                                       (Just Macros.untilEndOfTurn)
                  , Macros.untap (ItVerbed "GainControl")
                  , Macros.gainsHaste (ItVerbed "Untap") (Just Macros.untilEndOfTurn) ]) ]
       Nothing

||| Aggressive Instinct
public export
aggressiveInstinct : Card
aggressiveInstinct =
  Macros.card "Aggressive Instinct" (Just [Macros.generic 1, Macros.pip Green]) []
       (MkTypeLine [] [Sorcery])
       [ Spell (Macros.dealsDamageOwnPower (Macros.target Macros.creatureYouControl)
                                           (Macros.target Macros.creatureYouDontControl)) ]
       Nothing

||| Arcum Dagsson
public export
arcumDagssonSacrifice : Effect []
arcumDagssonSacrifice =
  ControllerSacrifices (Macros.target (And [Macros.artifact, Macros.creature]))

||| Harried Dronesmith
public export
harriedDronesmithToken : Effect []
harriedDronesmithToken =
  Sequentially [ Macros.create (Lit 1)
                   (MkToken (Just (Lit 1 ** Lit 1)) []
                            (MkTypeLine [creatureType "Thopter"] [Artifact, Creature])
                            [Macros.keyword "Flying"] Nothing)
               , Macros.gainsHaste Macros.itAsToken (Just Macros.untilEndOfTurn) ]

||| Feral Contest
public export
feralContest : Card
feralContest =
  Macros.card "Feral Contest" (Just [Macros.generic 3, Macros.pip Green]) []
       (MkTypeLine [] [Sorcery])
       [ Spell (Sequentially
                  [ PutCounters (Lit 1) (PrintedKind Macros.plusOnePlusOne)
                                (Macros.target Macros.creatureYouControl)
                  , Macros.mustBlockIt
                      (Macros.target (And [Macros.creature, Other]))
                      (Just Macros.thisTurn) ]) ]
       Nothing

||| Thranduil's Company
public export
thranduilsCompany : Card
thranduilsCompany =
  Macros.card "Thranduil's Company"
       (Just [Macros.generic 2, Macros.pip Green, Macros.pip Blue]) []
       (MkTypeLine [creatureType "Elf", creatureType "Soldier"] [Creature])
       [ Macros.triggered Whenever
           (Enters (Macros.a (And [Macros.land, HasPossessor ControllerAx You])) Nothing)
           (Sequentially
              [ PutCounters (Lit 2) (PrintedKind Macros.plusOnePlusOne)
                            (Macros.target Macros.creatureYouControl)
              , Macros.gains
                  (Macros.itPrior
                     (PutCounters (Lit 2) (PrintedKind Macros.plusOnePlusOne)
                                  (Macros.target Macros.creatureYouControl)))
                  (Macros.keyword "Vigilance") (Just Macros.untilEndOfTurn) ]) ]
       (Just (3, 4))

||| Inquisitor's Flail
public export
inquisitorsFlail : Card
inquisitorsFlail =
  Macros.card "Inquisitor's Flail" (Just [Macros.generic 2]) []
       (MkTypeLine [artifactType "Equipment"] [Artifact])
       [ Static (Scales CombatOnly (AttachHost Equipped (TypeW Creature))
                        Everywhere (Multiplied Doubled) Repeatedly)
       , Static (Scales CombatOnly
                        (Macros.a (Macros.otherCreature
                                     (AttachHost Equipped (TypeW Creature))))
                        (Macros.shieldingIt (AttachHost Equipped (TypeW Creature)))
                        (Multiplied Doubled) Repeatedly)
       , Macros.keywordCosting "Equip" (Mana [Macros.generic 2]) ]
       Nothing

||| Stunning Shot
public export
stunningShot : Card
stunningShot =
  Macros.card "Stunning Shot" (Just [Macros.generic 1, Macros.pip White]) []
       (MkTypeLine [] [Sorcery])
       [ Spell (Sequentially
                  [ PutCounters (Lit 2) (PrintedKind Macros.plusOnePlusOne)
                                (Macros.targets (Macros.upTo 1) Macros.creatureYouControl)
                  , Macros.tap (Macros.targets (Macros.upTo 1)
                                  (And [Macros.creature, HasPossessor ControllerAx Macros.anOpponent]))
                  , PutCounters (Lit 1) (PrintedKind (Named "Stun"))
                                (Macros.itPrior
                                   (Macros.tap (Macros.targets (Macros.upTo 1)
                                      (And [Macros.creature,
                                            HasPossessor ControllerAx Macros.anOpponent])))) ]) ]
       Nothing

||| Stonebinder's Familiar
public export
stonebindersFamiliar : Card
stonebindersFamiliar =
  Macros.card "Stonebinder's Familiar"
       (Just [Macros.pip White]) []
       (MkTypeLine [creatureType "Spirit", creatureType "Dog"] [Creature])
       [ Triggered Whenever
                   (PutInto (Macros.counted (Macros.atLeast 1) IsCard)
                            Macros.exileZ Nothing)
                   []
                   Nothing
                   []
                   (Just (DuringWindow Turn (Just You)))
                   (Just OncePerTurn)
                   Nothing
                   (PutCounters (Lit 1) (PrintedKind Macros.plusOnePlusOne)
                                Macros.thisCreature) ]
       (Just (1, 1))

||| Bioplasm
public export
bioplasm : Card
bioplasm =
  Macros.card "Bioplasm"
       (Just [Macros.generic 3, Macros.pip Green, Macros.pip Green]) []
       (MkTypeLine [creatureType "Ooze"] [Creature])
       [ Macros.triggered Whenever
           (Macros.attacks Macros.thisCreature)
           (Sequentially
              [ Macros.exile You Macros.topCard
              , If Cards.bioplasmCardTest
                   (Macros.gets Macros.thisCreature
                        (PtUp (Macros.powerOf
                                 (Macros.theVerbed "Exile" (TypedCardW Creature))))
                        (PtUp (Macros.toughnessOf (Macros.itVerbed "Exile")))
                        (Just Macros.thisTurn))
                   Nothing ]) ]
       (Just (4, 4))

||| Oath of Kaya
public export
oathOfKaya : Card
oathOfKaya =
  Macros.card "Oath of Kaya"
       (Just [Macros.generic 1, Macros.pip White, Macros.pip Black]) [Legendary]
       (MkTypeLine [] [Enchantment])
       [ Macros.triggered When
           (Enters This Nothing)
           (Sequentially [ DealDamage This (Lit 3) (Macros.target Macros.anyTarget)
                         , Macros.gainsLife You (Lit 3) ])
       , Macros.triggered Whenever
           (AttacksWith Macros.anOpponent
                        (OneDefender (Macros.a (And [HasType Planeswalker,
                                                     HasPossessor ControllerAx You])))
                        (Macros.counted (Macros.atLeast 1) Macros.creature))
           (Sequentially [ DealDamage This (Lit 2) (That PlayerW)
                         , Macros.gainsLife You (Lit 2) ]) ]
       Nothing

||| Stifle
public export
stifle : Card
stifle =
  Macros.card "Stifle" (Just [Macros.pip Blue]) []
       (MkTypeLine [] [Instant])
       [ Spell (Macros.counterSpell
                  (Macros.target (Or [AbilityHead AnyActivated,
                                      AbilityHead AnyTriggered]))) ]
       Nothing

||| Disallow
public export
disallow : Card
disallow =
  Macros.card "Disallow"
       (Just [Macros.generic 1, Macros.pip Blue, Macros.pip Blue]) []
       (MkTypeLine [] [Instant])
       [ Spell (Macros.counterSpell
                  (Macros.target
                     (Joined Macros.spell
                             (Or [AbilityHead AnyActivated,
                                  AbilityHead AnyTriggered])))) ]
       Nothing

||| Nazgûl
public export
nazgulRingTrigger : Ability
nazgulRingTrigger =
  Macros.triggered Whenever
    (VerbedEvent (Just You) "The Ring Tempts You" Nothing Nothing False)
    (PutCounters (Lit 1) (PrintedKind Macros.plusOnePlusOne)
                 (Macros.each (And [HasSubtype (creatureType "Wraith"),
                             HasPossessor ControllerAx You])))

||| Captain Marvel, Apex Avenger
public export
captainMarvelApexAvenger : Card
captainMarvelApexAvenger =
  Macros.card "Captain Marvel, Apex Avenger"
       (Just [Macros.generic 5, Macros.pip Red, Macros.pip White]) [Legendary]
       (MkTypeLine [creatureType "Human", creatureType "Kree",
                    creatureType "Hero"] [Creature])
       [ Macros.keyword "Flying"
       , Macros.keyword "DoubleStrike"
       , Macros.keyword "Indestructible"
       , Macros.triggeredIf Whenever
           (CounterEvent CounterPut Nothing
                         (Macros.a (And [Macros.creature, OtherThan This]))
                         ManyCounters (Just You) Nothing)
           (Matches It (Not (HasSubtype (creatureType "Kree"))))
           (Macros.may You (PutCounters ThatMuch ThoseKinds This)) ]
       (Just (4, 4))

||| Verity Circle
public export
verityCircle : Card
verityCircle =
  Macros.card "Verity Circle"
       (Just [Macros.generic 2, Macros.pip Blue]) []
       (MkTypeLine [] [Enchantment])
       [ Macros.triggeredIf Whenever
           (StatusEvent (Macros.a (And [Macros.creature,
                                        HasPossessor ControllerAx Macros.anOpponent])) Tapped)
           (Matches It (Not BeingDeclaredAttacker))
           (Macros.may You (Macros.draw You (Lit 1)))
       , Macros.activated (Mana [Macros.generic 4, Macros.pip Blue])
           (Macros.tap (Macros.target
                          (And [Macros.creature, Not (HasKeyword (TheKeyword "Flying"))]))) ]
       Nothing

||| Archfiend's Vessel
public export
archfiendsVessel : Card
archfiendsVessel =
  Macros.card "Archfiend's Vessel" (Just [Macros.pip Black]) []
       (MkTypeLine [creatureType "Human", creatureType "Cleric"] [Creature])
       [ Macros.keyword "Lifelink"
       , Macros.triggeredIf When
           (Enters This Nothing)
           (OrCond [ Happened Entry It Triggering
                       (Just (FromZones (FromZone [Macros.graveyardOf You]) Nothing))
                   , Matches It (And [CastBy You,
                                      CastFrom (Macros.graveyardOf You)]) ])
           (Reflexively (Macros.exile You It)
              (Macros.create (Lit 1)
                 (MkToken (Just (Lit 5 ** Lit 5)) [Black]
                          (MkTypeLine [creatureType "Demon"] [Creature])
                          [Macros.keyword "Flying"] Nothing))) ]
       (Just (1, 1))

||| Hostile Investigator
public export
hostileInvestigatorHeader : GameEvent []
hostileInvestigatorHeader =
  VerbedEvent (Just (Macros.counted (Macros.atLeast 1) AnyPlayer))
              "Discard"
              (Just (Macros.counted (Macros.atLeast 1) IsCard)) Nothing
              False

||| Hallowed Moonlight
public export
hallowedMoonlight : Card
hallowedMoonlight =
  Macros.card "Hallowed Moonlight"
       (Just [Macros.generic 1, Macros.pip White]) []
       (MkTypeLine [] [Instant])
       [ Spell (Sequentially
                  [ Macros.ifWouldInstead
                      (Enters (Macros.a (And [Macros.creature, Not WasCast])) Nothing)
                      (Macros.exile You It)
                      (Just Macros.untilEndOfTurn)
                  , (Macros.draw You (Lit 1)) ]) ]
       Nothing

||| Containment Priest
public export
containmentPriest : Card
containmentPriest =
  Macros.card "Containment Priest"
       (Just [Macros.generic 1, Macros.pip White]) []
       (MkTypeLine [creatureType "Human", creatureType "Cleric"] [Creature])
       [ Macros.keyword "Flash"
       , Static (Intercepts
                   (Enters (Macros.a (And [Macros.creature, Not IsToken,
                                           Not WasCast])) Nothing)
                   [] Nothing (Macros.exile You It) Repeatedly Nothing) ]
       (Just (2, 2))


testMox1 : Card
testMox1 =
  Macros.card "Mox Diamond" Nothing [] (MkTypeLine [] [Artifact])
       [ Macros.activated TapSymbol
                          (AddMana You (Lit 1) (AnyColor SameColor) []) ]
       Nothing

||| Heart of Yavimaya
public export
heartOfYavimaya : Card
heartOfYavimaya =
  Macros.card "Heart of Yavimaya" Nothing [] (MkTypeLine [] [Land])
       [ Static (Intercepts (Enters This Nothing) [] Nothing
                   ((IfDone (Macros.sacrifice You
                           (Macros.a (And [Macros.land,
                                           HasSubtype (landType "Forest")]))) (Just (Macros.putOntoBattlefield This)) (Just (Macros.move This Macros.graveyardZ))))
                   Repeatedly Nothing)
       , Macros.activated TapSymbol
                          (AddMana You (Lit 1) (Runs [[OfColor Green]]) [])
       , Macros.activated TapSymbol
                          (Macros.gets (Macros.target Macros.creature)
                             (PtUp (Lit 1)) (PtUp (Lit 1))
                             (Just Macros.untilEndOfTurn)) ]
       Nothing

||| Mox Diamond
public export
moxDiamond : Card
moxDiamond =
  Macros.card "Mox Diamond" Nothing [] (MkTypeLine [] [Artifact])
       [ Static (Intercepts (Enters This Nothing) [] Nothing
                   ((May You (Macros.discard You
                           (Macros.a (And [Macros.land, InZone Macros.handZ]))) (Just (Macros.putOntoBattlefield This)) (Just (Macros.move This Macros.graveyardZ))))
                   Repeatedly Nothing)
       , Macros.activated TapSymbol
                          (AddMana You (Lit 1) (AnyColor SameColor) []) ]
       Nothing

||| Gather Specimens
public export
gatherSpecimens : Card
gatherSpecimens =
  Macros.card "Gather Specimens"
       (Just [Macros.generic 3, Macros.pip Blue, Macros.pip Blue, Macros.pip Blue])
       [] (MkTypeLine [] [Instant])
       [ Spell (Continuously {ts = StaticFirstDone}
                  (EntersRider
                     (Macros.a (And [Macros.creature,
                                     HasPossessor ControllerAx Macros.anOpponent]))
                     (Under You))
                  (Just Macros.thisTurn)) ]
       Nothing

||| Don't Blink's replacement, without its written agent
public export
dontBlinkReplacement : StaticEffect []
dontBlinkReplacement =
  Intercepts (Enters (Macros.counted (Macros.atLeast 1) Macros.creature)
                     (Just (FromZone [Macros.exileZ])))
             [ Enters (Macros.counted (Macros.atLeast 1)
                         (And [Macros.creature, CastFrom Macros.exileZ]))
                      Nothing ] Nothing
             (Macros.shuffleInto You Them)
             Repeatedly Nothing

||| Seasoned Warrenguard
public export
seasonedWarrenguard : Card
seasonedWarrenguard =
  Macros.card "Seasoned Warrenguard" (Just [Macros.pip White]) []
       (MkTypeLine [creatureType "Rabbit", creatureType "Warrior"] [Creature])
       [ Macros.triggeredWhile Whenever
           (Macros.attacks Macros.thisCreature)
           (Macros.whileState (Macros.exists (And [IsToken, HasPossessor ControllerAx You])))
           (Macros.gets Macros.thisCreature (PtUp (Lit 2)) (PtUp (Lit 0))
                        (Just Macros.untilEndOfTurn)) ]
       (Just (1, 2))

||| Brazen Blademaster
public export
brazenBlademaster : Card
brazenBlademaster =
  Macros.card "Brazen Blademaster"
       (Just [Macros.generic 2, Macros.pip Red]) []
       (MkTypeLine [creatureType "Orc", creatureType "Pirate"] [Creature])
       [ Macros.triggeredWhile Whenever
           (Macros.attacks Macros.thisCreature)
           (Macros.whileState
              (CompareAmt (Macros.countOf (And [Macros.artifact, HasPossessor ControllerAx You]))
                          AtLeast (Lit 2)))
           (Macros.gets It (PtUp (Lit 2)) (PtUp (Lit 1))
                        (Just Macros.untilEndOfTurn)) ]
       (Just (2, 3))

||| Up the Beanstalk
public export
upTheBeanstalk : Card
upTheBeanstalk =
  Macros.card "Up the Beanstalk" (Just [Macros.generic 1, Macros.pip Green]) []
       (MkTypeLine [] [Enchantment])
       [ Macros.triggeredJoined When
           (Enters This Nothing)
           [ Macros.joinedHead Whenever
               (Casts You (Macros.a (And [Macros.spell,
                                          Compare [CharAxis ManaValue] AtLeast (Lit 5)])) Nothing) ]
           (Sequentially
              [ Macros.searchLibraryOrGraveyard
                  (And [Macros.artifact,
                        Compare [CharAxis ManaValue] AtMost (Lit 2)]) ]) ]
       Nothing

||| Autarch Mammoth
public export
autarchMammothLine : Ability
autarchMammothLine =
  Macros.triggeredJoined When
    (Enters Macros.thisCreature Nothing)
    [ Macros.joinedHeadWhile Whenever
        (Macros.attacks Macros.thisCreature)
        (Macros.whileState
           (Matches Macros.thisCreature (HasDesignation Saddled))) ]
    (Macros.create (Lit 1)
       (Macros.creatureTok 3 3 [Green] [creatureType "Elephant"]))

||| Veiling Oddity
public export
veilingOddityLine : Ability
veilingOddityLine =
  Macros.triggeredWhile When
    (LastCounterRemoved (Named "Time") This Nothing)
    (Macros.whileState (Matches This (InZone Macros.exileZ)))
    (Continuously {ts = StaticFirstDone} (Macros.deontic (Macros.allOf Macros.creature) Forbid ["Block"] Patient
                           NoDeonticPatient)
                  (Just Macros.thisTurn))

public export
whileScrying : Concurrent []
whileScrying = WhileDoing (VerbedEvent (Just You) "Scry" Nothing Nothing False)

public export
akkiLavarunner : Card
akkiLavarunner =
  FlipCard
    (Macros.frontFace "Akki Lavarunner" (Just [Macros.generic 3, Macros.pip Red]) []
            (MkTypeLine [creatureType "Goblin", creatureType "Warrior"] [Creature])
            [ Macros.keyword "Haste"
            , Macros.triggered Whenever
                (DealsDamage AnyDamage Macros.thisCreature (OnePatient Macros.anOpponent))
                (SetStatus Flipped Macros.thisCreature) ]
            (Macros.printedBox (Just (1, 1))))
    (Macros.backFace "Tok-Tok, Volcano Born" [Legendary]
               (MkTypeLine [creatureType "Goblin", creatureType "Shaman"] [Creature])
               [ Macros.keywordQuality "Protection" (ColorIs Red)
               , Static (Scales AnyDamage
                           (Macros.a (And [Macros.source, ColorIs Red]))
                           (Macros.shieldingIt (Macros.a AnyPlayer))
                           (Shifted ShiftUp (Lit 1)) Repeatedly) ]
               (Macros.printedBox (Just (2, 2))))

public export
bushiTenderfoot : Card
bushiTenderfoot =
  FlipCard
    (Macros.frontFace "Bushi Tenderfoot" (Just [Macros.pip White]) []
            (MkTypeLine [creatureType "Human", creatureType "Soldier"] [Creature])
            [ Macros.triggered When
                (Dies (Macros.a (And [ Macros.creature
                                     , HappenedTo DamageTaken ThisTurn
                                         (Just (Involving Macros.thisCreature)) ])))
                (SetStatus Flipped Macros.thisCreature) ]
            (Macros.printedBox (Just (1, 1))))
    (Macros.backFace "Kenzo the Hardhearted" [Legendary]
               (MkTypeLine [creatureType "Human", creatureType "Samurai"] [Creature])
               [ Macros.keyword "DoubleStrike"
               , Macros.keywordNumber "Bushido" (Lit 2) ]
               (Macros.printedBox (Just (3, 4))))

||| Frostwielder
public export
frostwielder : Card
frostwielder =
  Macros.card "Frostwielder"
       (Just [Macros.generic 2, Macros.pip Red, Macros.pip Red]) []
       (MkTypeLine [creatureType "Human", creatureType "Shaman"] [Creature])
       [ Static (Intercepts
                   (Dies (Macros.a (And [ Macros.creature
                                        , HappenedTo DamageTaken ThisTurn
                                            (Just (Involving Macros.thisCreature)) ])))
                   [] Nothing (Macros.exile You It) Repeatedly Nothing)
       , Macros.activated TapSymbol
           (DealDamage Macros.thisCreature (Lit 1) (Macros.target Macros.anyTarget)) ]
       (Just (1, 2))

||| Kitsune Mystic
public export
kitsuneMysticFlip : Ability
kitsuneMysticFlip =
  Macros.triggeredIf At
    (BeginningOf EndStep NoPossessor)
    (Matches Macros.thisCreature
       (AttachedBy Enchanted
          (Macros.counted (Macros.atLeast 2)
             (HasSubtype (enchantmentType "Aura")))))
    (SetStatus Flipped Macros.thisCreature)


||| Blood Spatter Analysis
public export
bloodSpatterAnalysis : Card
bloodSpatterAnalysis =
  Macros.card "Blood Spatter Analysis"
       (Just [Macros.pip Black, Macros.pip Red]) []
       (MkTypeLine [] [Enchantment])
       [ Macros.triggered When (Enters Macros.thisEnchantment Nothing)
           (DealDamage Macros.thisEnchantment (Lit 3)
              (Macros.target (And [Macros.creature,
                                   HasPossessor ControllerAx Macros.anOpponent])))
       , Macros.triggered Whenever
           (Dies (Macros.counted (Macros.atLeast 1) Macros.creature))
           (Sequentially
              [ Macros.mills You (Lit 1) You
              , PutCounters (Lit 1) (PrintedKind (Named "Bloodstain")) Macros.thisEnchantment
              , Reflexively
                  (OnlyIf (Macros.sacrifice You Macros.thisEnchantment)
                          (CompareAmt (CountersOn (Named "Bloodstain") Macros.thisEnchantment)
                                      AtLeast (Lit 5))
                          Nothing)
                  (Macros.move
                     (Macros.target (And [Macros.creature, IsCard,
                                          InZone (Macros.graveyardOf You)]))
                     Macros.handZ) ]) ]
       Nothing

||| Bewitching Leechcraft
public export
bewitchingLeechcraft : Card
bewitchingLeechcraft =
  Macros.card "Bewitching Leechcraft"
       (Just [Macros.generic 1, Macros.pip Blue]) []
       (MkTypeLine [enchantmentType "Aura"] [Enchantment])
       [ Macros.keywordSubject "Enchant" Macros.creature
       , Macros.triggered When (Enters Macros.thisAura Nothing)
           (Macros.tap (AttachHost Enchanted (TypeW Creature)))
       , Static (Gains (AttachHost Enchanted (TypeW Creature))
                   (Static (Intercepts
                              (StatusEvent Macros.thisCreature Untapped) []
                              (Just (DuringWindow UntapStep (Just You)))
                              ((IfDone (RemoveCounters (Just (Macros.exactly 1))
                                    (Just (PrintedKind Macros.plusOnePlusOne))
                                    Macros.thisCreature) (Just (Macros.untap Macros.thisCreature)) Nothing))
                              Repeatedly Nothing))) ]
       Nothing

public export
bonusRound : Card
bonusRound =
  Macros.card "Bonus Round"
       (Just [Macros.generic 1, Macros.pip Red, Macros.pip Red]) []
       (MkTypeLine [] [Sorcery])
       [ Spell (Delayed (Casts (Macros.a AnyPlayer)
                               (Macros.a (And [Macros.instantOrSorcery,
                                               Macros.spell])) Nothing)
                        [] (Just Macros.untilEndOfTurn)
                        (Sequentially
                           [ CopyStack (That PlayerW) (That SpellW) (Lit 1) []
                           , Macros.may (That PlayerW)
                               (ChooseNewTargets (That CopyW)) ])) ]
       Nothing

||| Sheltered Valley
public export
shelteredValley : Card
shelteredValley =
  Macros.card "Sheltered Valley" Nothing [] (MkTypeLine [] [Land])
       [ Static (Intercepts (Enters This Nothing) [] Nothing
                   (Sequentially
                      [ Macros.sacrifice You
                          (Macros.each (And [Permanent, OtherThan Macros.thisLand,
                                      Named (PrintedName "Sheltered Valley"),
                                      HasPossessor ControllerAx You]))
                      , Macros.putOntoBattlefield This ])
                   Repeatedly Nothing)
       , Macros.triggeredIf At (BeginningOf Upkeep (Macros.yours))
           (CompareAmt (Macros.countOf (And [Macros.land, HasPossessor ControllerAx You]))
                       AtMost (Lit 3))
           (Macros.gainsLife You (Lit 1))
       , Macros.activated TapSymbol
                          (AddMana You (Lit 1) (Runs [[Colorless]]) []) ]
       Nothing



||| Field of Dreams
public export
fieldOfDreams : Card
fieldOfDreams =
  Macros.card "Field of Dreams" (Just [Macros.pip Blue]) [World]
       (MkTypeLine [] [Enchantment])
       [ Static (Visibility Reveal (PlayerGroup AllPlayers) TopOfLibrary) ]
       Nothing

||| Lantern of Insight
public export
lanternOfInsightRider : Ability
lanternOfInsightRider =
  Static (Visibility Reveal (PlayerGroup AllPlayers) TopOfLibrary)

public export
wizenedSnitches : Card
wizenedSnitches =
  Macros.card "Wizened Snitches"
       (Just [Macros.generic 3, Macros.pip Blue]) []
       (MkTypeLine [creatureType "Faerie", creatureType "Rogue"] [Creature])
       [ Macros.keyword "Flying"
       , Static (Visibility Reveal (PlayerGroup AllPlayers) TopOfLibrary) ]
       (Just (1, 3))

||| Revelation
public export
revelation : Card
revelation =
  Macros.card "Revelation" (Just [Macros.pip Green]) [World]
       (MkTypeLine [] [Enchantment])
       [ Static (Visibility Reveal (PlayerGroup AllPlayers) WholeHand) ]
       Nothing

||| Keeper of the Lens
public export
keeperOfTheLens : Card
keeperOfTheLens =
  Macros.card "Keeper of the Lens" (Just [Macros.generic 1]) []
       (MkTypeLine [creatureType "Golem"] [Artifact, Creature])
       [ Static (Visibility LookAt You
                   (VisibleObjects
                      (Macros.allOf (And [Macros.creature, HasStatus FaceDown,
                                   Not (HasPossessor ControllerAx You)])))) ]
       (Just (1, 2))

||| Lens of Clarity
public export
lensOfClarity : Card
lensOfClarity =
  Macros.card "Lens of Clarity" (Just [Macros.generic 1]) []
       (MkTypeLine [] [Artifact])
       [ Static (AndAlso
           [ Visibility LookAt You TopOfLibrary
           , Visibility LookAt You
               (VisibleObjects
                  (Macros.allOf (And [Macros.creature, HasStatus FaceDown,
                               Not (HasPossessor ControllerAx You)]))) ]) ]
       Nothing

||| Danitha, New Benalia's Light
public export
danithaNewBenaliasLight : Card
danithaNewBenaliasLight =
  Macros.card "Danitha, New Benalia's Light"
       (Just [Macros.generic 1, Macros.pip Green, Macros.pip White]) [Legendary]
       (MkTypeLine [creatureType "Human", creatureType "Knight"] [Creature])
       [ Macros.keyword "Vigilance"
       , Macros.keyword "Trample"
       , Macros.keyword "Lifelink"
       , Static ((Macros.mayPlayDeed "Cast" You (Macros.a (And [Macros.spell,
                                   Or [HasSubtype (enchantmentType "Aura"),
                                       HasSubtype (artifactType "Equipment")]])) (PlayRider (Macros.fromZ (Macros.graveyardOf You)) Macros.onceEachYourTurn Nothing False ItsOwnCost))) ]
       (Just (2, 2))

||| Muldrotha, the Gravetide
public export
muldrothaLandWindow : StaticEffect []
muldrothaLandWindow =
  (Macros.mayPlayDeed "Play" You (Macros.a Macros.land) (PlayRider (Macros.fromZ (Macros.graveyardOf You)) Nothing Macros.duringEachYourTurn False ItsOwnCost))

||| Nahiri's Lithoforming
public export
nahiriExtraLands : StaticEffect (costLetters (Just [Variable]))
nahiriExtraLands = Macros.mayPlayAdditionalLands You (ExactlyOf (LetterVal X))

||| Phyrexian Unlife
public export
phyrexianUnlifeImmunity : Ability
phyrexianUnlifeImmunity = Static (NoLossFrom You ZeroOrLessLife)

||| Haakon, Stromgald Scourge
public export
haakonStromgaldScourge : Card
haakonStromgaldScourge =
  Macros.card "Haakon, Stromgald Scourge"
       (Just [Macros.generic 1, Macros.pip Black, Macros.pip Black]) [Legendary]
       (MkTypeLine [creatureType "Zombie", creatureType "Knight"] [Creature])
       [ Static ((Macros.mayPlayDeed "Cast" You This (PlayRider (Macros.fromZ (Macros.graveyardOf You)) Nothing Nothing True ItsOwnCost)))
       , Static (Conditionally (Matches This (InZone Macros.battlefieldZ))
                   ((Macros.mayPlayDeed "Cast" You (Macros.allOf (And [Macros.spell,
                                   HasSubtype (creatureType "Knight")])) (PlayRider (Macros.fromZ (Macros.graveyardOf You)) Nothing Nothing False ItsOwnCost)))
                   AsLongAs {st = Static.CondFirstDone})
       , Macros.triggered When (Dies Macros.thisCreature)
           (Macros.losesLife You (Lit 2)) ]
       (Just (3, 3))

||| Melek, Izzet Paragon
public export
melekIzzetParagon : Card
melekIzzetParagon =
  Macros.card "Melek, Izzet Paragon"
       (Just [Macros.generic 4, Macros.pip Blue, Macros.pip Red]) [Legendary]
       (MkTypeLine [creatureType "Weird", creatureType "Wizard"] [Creature])
       [ Static (Visibility Reveal You TopOfLibrary)
       , Static ((Macros.mayPlayDeed "Cast" You (Macros.allOf (And [Macros.spell, Macros.instantOrSorcery])) (PlayRider (Macros.fromZ Macros.onTopZ) Nothing Nothing False ItsOwnCost)))
       , Macros.triggered Whenever
           (Casts You (Macros.a (And [Macros.instantOrSorcery, Macros.spell]))
                  (Just Macros.yourLibrary))
           (Sequentially
              [ CopyStack You It (Lit 1) []
              , Macros.may You (ChooseNewTargets (That CopyW)) ]) ]
       (Just (2, 4))

||| Hot Pursuit's intervening "if two or more players have lost the game"
public export
twoOrMorePlayersHaveLost : Condition []
twoOrMorePlayersHaveLost =
  CompareAmt (Macros.countOf (And [AnyPlayer, Macros.happenedTo GameLoss ThisGame]))
             AtLeast (Lit 2)

||| Apex of Power
public export
apexOfPowerCast : Effect []
apexOfPowerCast =
  Sequentially
    [ Macros.exile You (LibrarySlice OnTop (Lit 7) You)
    , Continuously {ts = StaticFirstDone}
        (Macros.mayPlayDeed "Cast" You
             (Macros.fromAmong Macros.anyNumber Macros.spell Them)
             (PlayRider Nothing Nothing Nothing False ItsOwnCost))
        (Just ThisTurn) ]

||| Umbris, Fear Manifest
public export
umbrisPump : Ability
umbrisPump =
  Static (Gets Macros.thisCreature
               (PtUp (Macros.nForEach 1
                        (And [IsCard, HasPossessor OwnerAx (PlayerGroup YourOpponents),
                              InZone Macros.exileZ])))
               (PtUp (Macros.nForEach 1
                        (And [IsCard, HasPossessor OwnerAx (PlayerGroup YourOpponents),
                              InZone Macros.exileZ]))))

||| Obelisk of Undoing
public export
obeliskOfUndoing : Ability
obeliskOfUndoing =
  Macros.activated (Compound [Mana [Macros.generic 6], TapSymbol])
                   (Macros.returnTo
                      (Macros.target (And [Permanent, HasPossessor OwnerAx You, HasPossessor ControllerAx You]))
                      Macros.handZ)

||| Flickering Ward
public export
flickeringWardBounce : Ability
flickeringWardBounce =
  Macros.activated (Mana [Macros.pip White])
                   (Macros.returnTo Macros.thisAura Macros.handZ)

||| Blight Herder
public export
blightHerderCast : Effect []
blightHerderCast =
  Macros.may You
    (Macros.move (Macros.counted (Macros.exactly 2)
                    (And [IsCard, HasPossessor OwnerAx (PlayerGroup YourOpponents),
                          InZone Macros.exileZ]))
                 Macros.graveyardZ)

||| Open the Vaults
public export
openTheVaults : Effect []
openTheVaults =
  Enact "Return"
    (Move (Macros.allOf (And [IsCard,
                       Or [HasType Artifact, HasType Enchantment],
                       InZone (ZoneAt Graveyard
                                 (PossessedBy (PlayerGroup AllPlayers)))]))
          Macros.battlefieldZ
          [Under (PossessorsOf OwnerAx Them)])

||| Crackling Doom
public export
cracklingDoom : Effect []
cracklingDoom =
  Sequentially
    [ DealDamage This (Lit 2) (Macros.each Opponent)
    , Macros.sacrifice (Macros.each Opponent)
        (Macros.a (And [Macros.creature,
                        Superlative MaxOf (CharAxis Power)
                          (And [Macros.creature,
                                HasPossessor ControllerAx (That PlayerW)])])) ]

||| Altar of the Brood
public export
altarOfTheBrood : Ability
altarOfTheBrood =
  Macros.triggered Whenever
    (Enters (Macros.a (And [Permanent, HasPossessor ControllerAx You, OtherThan This])) Nothing)
    (Macros.mills (Macros.each Opponent) (Lit 1) They)

||| Soul Shatter
public export
soulShatter : Effect []
soulShatter =
  Macros.sacrifice (Macros.each Opponent)
    (Macros.a (And [Or [Macros.creature, HasType Planeswalker],
                    Superlative MaxOf (CharAxis ManaValue)
                      (And [Or [Macros.creature, HasType Planeswalker],
                            HasPossessor ControllerAx They])]))

||| Padeem, Consul of Innovation
public export
padeemConsulOfInnovation : Ability
padeemConsulOfInnovation =
  Macros.triggeredIf At (BeginningOf Upkeep (Macros.yours))
    (Matches (Macros.the (And [Macros.artifact,
                             Superlative MaxOf (CharAxis ManaValue)
                               (And [Macros.artifact,
                                     InZone Macros.battlefieldZ])]))
             (HasPossessor ControllerAx You))
    (Macros.draw You (Lit 1))

||| Turbulent Fen
public export
turbulentFen : Ability
turbulentFen =
  Static (Macros.onlyUnless (Macros.entersTapped Macros.thisLand)
                    (CompareAmt
                       (Macros.countOf (And [Macros.land,
                                      HasPossessor ControllerAx (PlayerGroup YourOpponents)]))
                       AtLeast (Lit 8)))

||| Darkblade Agent
public export
darkbladeAgentDeathtouch : Ability
darkbladeAgentDeathtouch =
  Static (Conditionally (Macros.happened (VerbedAct "Surveil") You ThisTurn)
                        (Gains Macros.thisCreature (Macros.keyword "Deathtouch"))
                        AsLongAs {st = Static.CondFirstDone})

public export
theFallen : Ability
theFallen =
  Macros.triggered At (BeginningOf Upkeep (Macros.yours))
    (DealDamage Macros.thisCreature (Lit 1)
       (Macros.each (And [Joined Opponent (HasType Planeswalker),
                   Macros.happenedToInvolving DamageTaken ThisGame
                     Macros.thisCreature])))

||| Codecracker Hound
public export
codecrackerHoundLook : Effect []
codecrackerHoundLook =
  Sequentially
    [ Macros.lookAt (Macros.topSlice (Lit 2))
    , Macros.move (Macros.oneOf Them) Macros.handZ
    , Macros.move TheOther Macros.graveyardZ ]

public export
theOtherAfterATwoCardLook :
  theOtherOk (nomIntro (SomeOf {bs = []}
                        (CountedSlice (Macros.exactly 1) {nz = MaxAtLeastOne} {wf = Oh})
                        Nothing
                               (LibrarySlice OnTop (Lit 2) You)
                               {gm = Oh})) = True
theOtherAfterATwoCardLook = Refl

public export
theOtherNeedsAStatedCount :
  theOtherOk (nomIntro (SomeOf {bs = []}
                        (CountedSlice (Macros.exactly 1) {nz = MaxAtLeastOne} {wf = Oh})
                        Nothing
                               (LibrarySlice OnTop (Macros.countOf Macros.creature) You)
                               {gm = Oh})) = False
theOtherNeedsAStatedCount = Refl

public export
theRestStandsWhereTheOtherRefuses :
  theRestOk (nomIntro (SomeOf {bs = []}
                        (CountedSlice (Macros.exactly 1) {nz = MaxAtLeastOne} {wf = Oh})
                        Nothing
                              (LibrarySlice OnTop (Macros.countOf Macros.creature) You)
                              {gm = Oh})) = True
theRestStandsWhereTheOtherRefuses = Refl

public export
theOtherRefusesTheUniversalSlice :
  (theOtherOk (nomIntro (SomeOf {bs = []} WholeSlice (Just Macros.artifact)
                                (LibrarySlice OnTop (Lit 5) You) {gm = Oh})),
   theRestOk (nomIntro (SomeOf {bs = []} WholeSlice (Just Macros.artifact)
                               (LibrarySlice OnTop (Lit 5) You) {gm = Oh})))
    = (False, True)
theOtherRefusesTheUniversalSlice = Refl

||| Talion, the Kindly Lord
public export
talionTheKindlyLord : Card
talionTheKindlyLord =
  Macros.card "Talion, the Kindly Lord"
       (Just [Macros.generic 2, Macros.pip Blue, Macros.pip Black]) [Legendary]
       (MkTypeLine [creatureType "Faerie", creatureType "Noble"] [Creature])
       [ Macros.keyword "Flying"
       , Static (Macros.entersChoosing Macros.thisCreature Number)
       , Macros.triggered Whenever
           (Casts (Macros.a Opponent)
                  (Macros.a (And [Macros.spell,
                                  Compare [CharAxis ManaValue, CharAxis Power, CharAxis Toughness]
                                          Eq ChosenNumber]))
                  Nothing)
           (Sequentially [ Macros.losesLife (That PlayerW) (Lit 2)
                         , (Macros.draw You (Lit 1)) ]) ]
       (Just (3, 4))

||| Cavalcade of Calamity
public export
cavalcadeOfCalamity : Ability
cavalcadeOfCalamity =
  Macros.triggered Whenever
    (Macros.attacks (Macros.a (And [Macros.creature, HasPossessor ControllerAx You,
                                    Compare [CharAxis Power] AtMost (Lit 1)])))
    (DealDamage Macros.thisEnchantment (Lit 1)
       (Macros.the (And [Joined AnyPlayer (HasType Planeswalker),
                       CombatRel AttackedBy (That (TypeW Creature))])))

||| Bastion Protector
public export
bastionProtectorPump : Ability
bastionProtectorPump =
  Static (Gets (Macros.allOf (And [Macros.creature, HasCardDesignation CommanderD,
                            HasPossessor ControllerAx You]))
               (PtUp (Lit 2)) (PtUp (Lit 2)))

public export
commanderCreaturesYouOwn : Predicate [] Object
commanderCreaturesYouOwn =
  And [Macros.creature, HasCardDesignation CommanderD, HasPossessor OwnerAx You]

||| Idol of Endurance
public export
creatureSpellFromAmongExiled : Noun [] Object
creatureSpellFromAmongExiled =
  Macros.oneFromAmong (And [Macros.creature, Macros.spell])
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

public export
permanentCardIsPlaceless :
  phraseZone (And {bs = []} [Permanent, IsCard]) = Nothing
permanentCardIsPlaceless = Refl

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
agentOfTheShadowThievesGrantedTrigger : AbilityAt []
agentOfTheShadowThievesGrantedTrigger =
  Triggered Whenever
    (Attacks Macros.thisCreature (OneDefender (Macros.a AnyPlayer)))
    [] Nothing [] Nothing Nothing
    (Just (NotCond (Macros.exists (And [Opponent,
             Compare [PlayerStatAxis LifeTotal] Greater
                     (PlayerStatOf LifeTotal (That PlayerW))]))))
    (PutCounters (Lit 1) (PrintedKind Macros.plusOnePlusOne)
                 Macros.thisCreature)

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

||| Mirror Universe
public export
mirrorUniverse : Card
mirrorUniverse =
  Macros.card "Mirror Universe" (Just [Macros.generic 6]) []
       (MkTypeLine [] [Artifact])
       [ Macros.activatedOnlyDuring
           (Compound [TapSymbol, Do (Macros.sacrifice You Macros.thisArtifact)])
           (ExchangeLife (Both You (Macros.target Opponent)))
           (DuringPart Upkeep (Just You)) ]
       Nothing

||| Soul Conduit
public export
soulConduitExchange : Ability
soulConduitExchange =
  Macros.activated (Compound [Mana [Macros.generic 6], TapSymbol])
                   (ExchangeLife (Macros.targets (Macros.exactly 2) AnyPlayer))

||| Frenzied Gorespawn
public export
frenziedGorespawnMenaceTrigger : AbilityAt []
frenziedGorespawnMenaceTrigger =
  Triggered Whenever
    (Attacks (Macros.counted (Macros.atLeast 1) Macros.creature)
             (OneDefender Macros.anOpponent))
    [] Nothing [] Nothing Nothing Nothing
    (Macros.gains (Those (TypeW Creature)) (Macros.keyword "Menace")
                  (Just Macros.untilEndOfTurn))

public export
eachPlayerShufflesTheirHandAndGraveyard : Effect []
eachPlayerShufflesTheirHandAndGraveyard =
  Macros.shuffleInto (Macros.each AnyPlayer)
    (Both (Macros.allOf (InZone (Macros.handOf They)))
            (Macros.allOf (InZone (Macros.graveyardOf They))))

public export
eachPlayerMayShuffleTheirHandAndGraveyard : Effect []
eachPlayerMayShuffleTheirHandAndGraveyard =
  Macros.may (Macros.each AnyPlayer)
    (Macros.shuffleInto They
       (Both (Macros.allOf (InZone (Macros.handOf They)))
               (Macros.allOf (InZone (Macros.graveyardOf They)))))

public export
eachPlayerMayDiscardTheirHandAndDrawSeven : Effect []
eachPlayerMayDiscardTheirHandAndDrawSeven =
  Macros.may (Macros.each AnyPlayer)
    (Sequentially [ Macros.discard They (Macros.allOf (InZone (Macros.handOf They)))
                  , Draw They (Lit 7) ])

||| Pir, Imaginative Rascal's replacement
public export
pirDistributive : Ability
pirDistributive =
  Static (Intercepts
            (Macros.manyBareCountersPutBy You
               (Macros.a (And [Permanent, HasPossessor ControllerAx (PlayerGroup YourTeam)])))
            [] Nothing
            (PutCounters (Plus ThatMuch (Lit 1)) ThoseKinds (That PermanentW))
            Repeatedly Nothing)



||| Frogmite
public export
frogmite : Card
frogmite =
  Macros.card "Frogmite" (Just [Macros.generic 4]) []
       (MkTypeLine [creatureType "Frog"] [Artifact, Creature])
       [ Macros.keywordQuality "Affinity" Macros.artifact ]
       (Just (2, 2))

||| Ulamog's Crusher
public export
ulamogsCrusher : Card
ulamogsCrusher =
  Macros.card "Ulamog's Crusher" (Just [Macros.generic 8]) []
       (MkTypeLine [creatureType "Eldrazi"] [Creature])
       [ Macros.keywordNumber "Annihilator" (Lit 2)
       , Static (Macros.deontic Macros.thisCreature Require ["Attack"] Agent
                                NoDeonticPatient) ]
       (Just (8, 8))

||| Iymrith, Desert Doom
public export
iymrithDesertDoom : Card
iymrithDesertDoom =
  Macros.card "Iymrith, Desert Doom"
       (Just [Macros.generic 3, Macros.pip Blue, Macros.pip Blue]) [Legendary]
       (MkTypeLine [creatureType "Dragon"] [Creature])
       [ Macros.keyword "Flying"
       , Static (Macros.onlyWhile
                   (Gains Macros.thisCreature
                          (Macros.keywordCosting "Ward" (Mana [Macros.generic 4])))
                   (Matches It (HasStatus Untapped)))
       , Macros.triggered Whenever
                          (Macros.dealsCombatDamage Macros.thisCreature (Macros.a AnyPlayer))
                          (Sequentially
                             [ (Macros.draw You (Lit 1))
                             , If (CompareAmt (Macros.countOf (InZone (Macros.handOf You)))
                                              Less (Lit 3))
                                  (Draw You TheDifference)
                                  Nothing ]) ]
       (Just (5, 5))

||| True-Name Nemesis
public export
trueNameNemesis : Card
trueNameNemesis =
  Macros.card "True-Name Nemesis"
       (Just [Macros.generic 1, Macros.pip Blue, Macros.pip Blue]) []
       (MkTypeLine [creatureType "Merfolk", creatureType "Rogue"] [Creature])
       [ Static (Macros.entersChoosingPlayer Macros.thisCreature Nothing)
       , Macros.keywordQuality "Protection" ChosenPlayer ]
       (Just (3, 1))

public export
protectionAbilities : KeywordTerm
protectionAbilities = AnyKeywordIn (MkKeywordFamily "Protection" Nothing)

public export
landwalkAbilities : KeywordTerm
landwalkAbilities = AnyKeywordIn (MkKeywordFamily "Landwalk" Nothing)

||| Shay Cormac
public export
wardAbilities : KeywordTerm
wardAbilities = AnyKeywordIn (MkKeywordFamily "Ward" Nothing)

||| Tolaria, Shelkin Brownie
public export
bandsWithOtherAbilities : KeywordTerm
bandsWithOtherAbilities = AnyKeywordIn (MkKeywordFamily "BandsWithOther" Nothing)

public export
protectionFromAnyColor : KeywordTerm
protectionFromAnyColor = AnyKeywordIn (MkKeywordFamily "Protection" (Just Color))

||| Cairn Wanderer
public export
cairnWanderer : Card
cairnWanderer =
  Macros.card "Cairn Wanderer" (Just [Macros.generic 4, Macros.pip Black]) []
       (MkTypeLine [creatureType "Shapeshifter"] [Creature])
       [ Macros.keyword "Changeling"
       , AlsoForKeywords
           (Static (Macros.asLongAs
                      (Macros.exists (And [Macros.creature, InZone Macros.graveyardZ,
                                    HasKeyword (TheKeyword "Flying")]))
                      (Gains Macros.thisCreature (Macros.keyword "Flying"))))
           [ TheKeyword "Fear", TheKeyword "FirstStrike"
           , TheKeyword "DoubleStrike", TheKeyword "Deathtouch"
           , TheKeyword "Haste", landwalkAbilities, TheKeyword "Lifelink"
           , protectionAbilities, TheKeyword "Reach", TheKeyword "Trample"
           , TheKeyword "Shroud", TheKeyword "Vigilance" ] ]
       (Just (4, 4))

||| Concerted Effort
public export
concertedEffort : Card
concertedEffort =
  Macros.card "Concerted Effort"
       (Just [Macros.generic 2, Macros.pip White, Macros.pip White]) []
       (MkTypeLine [] [Enchantment])
       [ AlsoForKeywords
           (Macros.triggeredIf At
                               (BeginningOf Upkeep (Macros.eachPlayers))
                               (Macros.exists (And [Macros.creature, HasPossessor ControllerAx You,
                                             HasKeyword (TheKeyword "Flying")]))
                               (Continuously {ts = StaticFirstDone}
                                  (Gains (Macros.allOf Macros.creatureYouControl)
                                         (Macros.keyword "Flying"))
                                  (Just Macros.untilEndOfTurn)))
           [ TheKeyword "Fear", TheKeyword "FirstStrike"
           , TheKeyword "DoubleStrike", landwalkAbilities, protectionAbilities
           , TheKeyword "Trample", TheKeyword "Vigilance" ] ]
       Nothing

||| Death-Mask Duplicant
public export
deathMaskDuplicant : Card
deathMaskDuplicant =
  Macros.card "Death-Mask Duplicant" (Just [Macros.generic 7]) []
       (MkTypeLine [creatureType "Shapeshifter"] [Artifact, Creature])
       [ Macros.abilityWord Imprint
           (Macros.activated (Mana [Macros.generic 1])
                             (Macros.exile You
                                (Macros.target
                                   (And [Macros.creature,
                                         InZone (Macros.graveyardOf You)]))))
       , AlsoForKeywords
           (Static (Macros.asLongAs
                      (Macros.exists (And [ExiledWith Macros.thisCreature,
                                    HasKeyword (TheKeyword "Flying")]))
                      (Gains Macros.thisCreature (Macros.keyword "Flying"))))
           [ TheKeyword "Fear", TheKeyword "FirstStrike"
           , TheKeyword "DoubleStrike", TheKeyword "Haste"
           , landwalkAbilities, protectionAbilities, TheKeyword "Trample" ] ]
       (Just (5, 5))

||| Autarch Mammoth
public export
autarchMammoth : Card
autarchMammoth =
  Macros.card "Autarch Mammoth"
       (Just [Macros.generic 4, Macros.pip Green, Macros.pip Green]) []
       (MkTypeLine [creatureType "Elephant", creatureType "Mount"] [Creature])
       [ autarchMammothLine
       , Macros.keywordNumber "Saddle" (Lit 5) ]
       (Just (5, 5))

||| Debris Beetle
public export
debrisBeetle : Card
debrisBeetle =
  Macros.card "Debris Beetle"
       (Just [Macros.generic 2, Macros.pip Black, Macros.pip Green]) []
       (MkTypeLine [artifactType "Vehicle"] [Artifact])
       [ Macros.keyword "Trample"
       , Macros.triggered When (Enters Macros.thisVehicle Nothing)
           (Sequentially [ Macros.losesLife (Macros.each Opponent) (Lit 3)
                         , Macros.gainsLife You (Lit 3) ])
       , Macros.keywordNumber "Crew" (Lit 2) ]
       (Just (6, 6))

||| Pir, Imaginative Rascal
public export
pirImaginativeRascal : Card
pirImaginativeRascal =
  Macros.card "Pir, Imaginative Rascal"
       (Just [Macros.generic 2, Macros.pip Green]) [Legendary]
       (MkTypeLine [creatureType "Human"] [Creature])
       [ Macros.keywordQuality "PartnerWith"
                               (Named (PrintedName "Toothy, Imaginary Friend"))
       , pirDistributive ]
       (Just (1, 1))

||| Bloodline Pretender
public export
bloodlinePretender : Card
bloodlinePretender =
  Macros.card "Bloodline Pretender" (Just [Macros.generic 3]) []
       (MkTypeLine [creatureType "Shapeshifter"] [Artifact, Creature])
       [ Macros.keyword "Changeling"
       , Static (Macros.entersChoosing Macros.thisCreature (SubtypeQ Creature))
       , Macros.triggered Whenever
           (Enters (Macros.a (And [Macros.creature, HasPossessor ControllerAx You,
                                   OtherThan Macros.thisCreature,
                                   OfChosen (SubtypeQ Creature)]))
                   Nothing)
           (PutCounters (Lit 1) (PrintedKind Macros.plusOnePlusOne)
                        Macros.thisCreature) ]
       (Just (2, 2))

||| Foul Emissary
public export
foulEmissaryLine : Ability
foulEmissaryLine =
  Macros.triggeredWhile When
    (VerbedEvent (Just You) "Sacrifice" (Just Macros.thisCreature) Nothing False)
    (WhileDoing (Casts You
                   (Macros.a (And [Macros.spell,
                                   HasKeyword (TheKeyword "Emerge")]))
                   Nothing))
    (Macros.create (Lit 1)
       (Macros.creatureTok 3 2 []
          [creatureType "Eldrazi", creatureType "Horror"]))

||| Market Gnome
public export
marketGnome : Card
marketGnome =
  Macros.card "Market Gnome" (Just [Macros.pip White]) []
       (MkTypeLine [creatureType "Gnome"] [Artifact, Creature])
       [ Macros.triggered When (Dies Macros.thisCreature)
           (Sequentially [ Macros.gainsLife You (Lit 1), (Macros.draw You (Lit 1)) ])
       , Macros.triggeredWhile When
           (VerbedEvent Nothing "Exile" (Just Macros.thisCreature) Nothing False)
           (WhileDoing (Activates You
                          (Macros.a (AbilityHead (KeywordClass "Craft")))))
           (Sequentially [ Macros.gainsLife You (Lit 1), (Macros.draw You (Lit 1)) ]) ]
       (Just (0, 3))

||| Escaped Shapeshifter
public export
escapedShapeshifter : Card
escapedShapeshifter =
  Macros.card "Escaped Shapeshifter"
       (Just [Macros.generic 3, Macros.pip Blue, Macros.pip Blue]) []
       (MkTypeLine [creatureType "Shapeshifter"] [Creature])
       [ AlsoForKeywords
           (Static (Macros.asLongAs
                      (Macros.exists (And [Macros.creature,
                                    HasPossessor ControllerAx Macros.anOpponent,
                                    HasKeyword (TheKeyword "Flying"),
                                    Not (Named (PrintedName "Escaped Shapeshifter"))]))
                      (Gains Macros.thisCreature (Macros.keyword "Flying"))))
           [ TheKeyword "FirstStrike", TheKeyword "Trample"
           , protectionFromAnyColor ] ]
       (Just (3, 4))

||| Ancestral Blade
public export
ancestralBlade : Card
ancestralBlade =
  Macros.card "Ancestral Blade" (Just [Macros.generic 1, Macros.pip White]) []
       (MkTypeLine [artifactType "Equipment"] [Artifact])
       [ Macros.triggered When (Enters Macros.thisEquipment Nothing)
           (Sequentially
              [ Macros.create (Lit 1)
                  (MkToken (Just (Lit 1 ** Lit 1)) [White]
                           (MkTypeLine [creatureType "Soldier"] [Creature])
                           [] Nothing)
              , AttachTo Macros.thisEquipment Macros.itAsToken ])
       , Static (Gets (AttachHost Equipped (TypeW Creature))
                      (PtUp (Lit 1)) (PtUp (Lit 1)))
       , Macros.keywordCosting "Equip" (Mana [Macros.generic 1]) ]
       Nothing

||| Disarm
public export
disarm : Card
disarm =
  Macros.card "Disarm" (Just [Macros.pip Blue]) [] (MkTypeLine [] [Instant])
       [ Spell (Unattach
                  (Macros.allOf (And [ HasSubtype (artifactType "Equipment")
                              , AttachedTo (Macros.target Macros.creature) ]))) ]
       Nothing

||| Embercleave
public export
embercleave : Card
embercleave =
  Macros.card "Embercleave"
       (Just [Macros.generic 4, Macros.pip Red, Macros.pip Red]) [Legendary]
       (MkTypeLine [artifactType "Equipment"] [Artifact])
       [ Macros.keyword "Flash"
       , Static (CostsToCast This
                   (CostLess (Macros.forEach
                                (And [Macros.creature, Attacking, HasPossessor ControllerAx You]))
                             Nothing))
       , Macros.triggered When (Enters Macros.thisEquipment Nothing)
           (AttachTo It (Macros.target Macros.creatureYouControl))
       , Static (AndAlso [ Gets (AttachHost Equipped (TypeW Creature))
                                (PtUp (Lit 1)) (PtUp (Lit 1))
                         , Gains (AttachHost Equipped (TypeW Creature))
                                 (Macros.keyword "DoubleStrike")
                         , Gains (AttachHost Equipped (TypeW Creature))
                                 (Macros.keyword "Trample") ])
       , Macros.keywordCosting "Equip" (Mana [Macros.generic 3]) ]
       Nothing

||| Steelclaw Lance
public export
steelclawLance : Card
steelclawLance =
  Macros.card "Steelclaw Lance" (Just [Macros.pip Black, Macros.pip Red]) []
       (MkTypeLine [artifactType "Equipment"] [Artifact])
       [ Static (Gets (AttachHost Equipped (TypeW Creature))
                      (PtUp (Lit 2)) (PtUp (Lit 2)))
       , Macros.keywordQualityCosting "Equip"
           (HasSubtype (creatureType "Knight")) (Mana [Macros.generic 1])
       , Macros.keywordCosting "Equip" (Mana [Macros.generic 3]) ]
       Nothing

||| Commander's Plate's equip lines
public export
commandersPlateEquip : List Ability
commandersPlateEquip =
  [ Macros.keywordQualityCosting "Equip"
      (HasCardDesignation CommanderD) (Mana [Macros.generic 3])
  , Macros.keywordCosting "Equip" (Mana [Macros.generic 5]) ]

||| Luxior, Giada's Gift's equip lines
public export
luxiorEquipLines : List Ability
luxiorEquipLines =
  [ Macros.keywordQualityCosting "Equip"
      (HasType Planeswalker) (Mana [Macros.generic 1])
  , Macros.keywordCosting "Equip" (Mana [Macros.generic 3]) ]

||| Veiling Oddity
public export
veilingOddity : Card
veilingOddity =
  Macros.card "Veiling Oddity"
       (Just [Macros.generic 3, Macros.pip Blue]) []
       (MkTypeLine [creatureType "Illusion"] [Creature])
       [ Macros.keywordNumberCosting "Suspend" (Lit 4)
           (Mana [Macros.generic 1, Macros.pip Blue])
       , veilingOddityLine ]
       (Just (2, 3))

||| Enormous Energy Blade
public export
enormousEnergyBlade : Card
enormousEnergyBlade =
  Macros.card "Enormous Energy Blade"
       (Just [Macros.generic 2, Macros.pip Black]) []
       (MkTypeLine [artifactType "Equipment"] [Artifact])
       [ Static (Gets (AttachHost Equipped (TypeW Creature))
                      (PtUp (Lit 4)) (PtUp (Lit 0)))
       , Macros.triggered Whenever
           (BecomesAttached Macros.thisEquipment (Macros.a Macros.creature))
           (Macros.tap (That (TypeW Creature)))
       , Macros.keywordCosting "Equip" (Mana [Macros.generic 2]) ]
       Nothing

||| Bramble Elemental
public export
brambleElemental : Card
brambleElemental =
  Macros.card "Bramble Elemental"
       (Just [Macros.generic 3, Macros.pip Green, Macros.pip Green]) []
       (MkTypeLine [creatureType "Elemental"] [Creature])
       [ Macros.triggered Whenever
           (BecomesAttached (Macros.a (HasSubtype (enchantmentType "Aura")))
                            Macros.thisCreature)
           (Macros.create (Lit 2)
              (Macros.creatureTok 1 1 [Green] [creatureType "Saproling"])) ]
       (Just (4, 4))

||| Grafted Wargear
public export
graftedWargear : Card
graftedWargear =
  Macros.card "Grafted Wargear" (Just [Macros.generic 3]) []
       (MkTypeLine [artifactType "Equipment"] [Artifact])
       [ Static (Gets (AttachHost Equipped (TypeW Creature))
                      (PtUp (Lit 3)) (PtUp (Lit 2)))
       , Macros.triggered Whenever
           (BecomesUnattached Macros.thisEquipment (Macros.a Permanent))
           (Macros.sacrifice You (That PermanentW))
       , Macros.keywordCosting "Equip" (Mana [Macros.generic 0]) ]
       Nothing

||| Stone Haven Outfitter
public export
stoneHavenOutfitter : Card
stoneHavenOutfitter =
  Macros.card "Stone Haven Outfitter" (Just [Macros.generic 1, Macros.pip White]) []
       (MkTypeLine [creatureType "Kor", creatureType "Artificer",
                    creatureType "Ally"] [Creature])
       [ Static (Gets (Macros.allOf (And [Macros.creatureYouControl, IsAttached Equipped]))
                      (PtUp (Lit 1)) (PtUp (Lit 1)))
       , Macros.triggered Whenever
           (Dies (Macros.a (And [Macros.creatureYouControl, IsAttached Equipped])))
           ((Macros.draw You (Lit 1))) ]
       (Just (2, 2))

||| Cloud, Ex-SOLDIER
public export
cloudExSoldierAttach : Ability
cloudExSoldierAttach =
  Macros.triggered When (Enters Macros.thisCreature Nothing)
    (Macros.attachToIt
       (Macros.targets (Macros.upTo 1)
          (And [HasSubtype (artifactType "Equipment"), HasPossessor ControllerAx You])))

public export
kitsuneMystic : Card
kitsuneMystic =
  FlipCard
    (Macros.frontFace "Kitsune Mystic" (Just [Macros.generic 3, Macros.pip White]) []
            (MkTypeLine [creatureType "Fox", creatureType "Wizard"] [Creature])
            [ kitsuneMysticFlip ]
            (Macros.printedBox (Just (2, 3))))
    (Macros.backFace "Autumn-Tail, Kitsune Sage" [Legendary]
               (MkTypeLine [creatureType "Fox", creatureType "Wizard"] [Creature])
               [ Macros.activated (Mana [Macros.generic 1])
                   (AttachTo
                      (Macros.target
                         (And [ HasSubtype (enchantmentType "Aura")
                              , AttachedTo (Macros.a Macros.creature) ]))
                      (Macros.a (And [Macros.creature, OtherThan This]))) ]
               (Macros.printedBox (Just (4, 5))))

||| Akiri, Fearless Voyager
public export
akiriEquippedAttackers : Ability
akiriEquippedAttackers =
  Macros.triggered Whenever
    (AttacksWith You (OneDefender (Macros.a AnyPlayer))
       (Macros.counted (Macros.atLeast 1)
          (And [Macros.creatureYouControl, IsAttached Equipped])))
    ((Macros.draw You (Lit 1)))

public export
akiriUnattachOffer : Ability
akiriUnattachOffer =
  Macros.activated (Mana [Macros.pip White])
    ((May You (Unattach
          (Macros.a (And [ HasSubtype (artifactType "Equipment")
                         , AttachedTo (Macros.a Macros.creatureYouControl) ]))) (Just (SetStatus Tapped (That (TypeW Creature)))) Nothing))

||| Black Ward
public export
blackWard : Card
blackWard =
  Macros.card "Black Ward" (Just [Macros.pip White]) []
       (MkTypeLine [enchantmentType "Aura"] [Enchantment])
       [ Macros.keywordSubject "Enchant" Macros.creature
       , Static (DoesntRemove
                   (Gains (AttachHost Enchanted (TypeW Creature))
                          (Macros.keywordQuality "Protection" (ColorIs Black)))
                   Macros.thisAura) ]
       Nothing

||| Cho-Manno's Blessing
public export
choMannosBlessing : Card
choMannosBlessing =
  Macros.card "Cho-Manno's Blessing" (Just [Macros.pip White, Macros.pip White]) []
       (MkTypeLine [enchantmentType "Aura"] [Enchantment])
       [ Macros.keyword "Flash"
       , Macros.keywordSubject "Enchant" Macros.creature
       , Static (Macros.entersChoosing Macros.thisAura Color)
       , Static (DoesntRemove
                   (Gains (AttachHost Enchanted (TypeW Creature))
                          (Macros.keywordQuality "Protection" (OfChosen Color)))
                   Macros.thisAura) ]
       Nothing

||| Tattoo Ward
public export
tattooWard : Card
tattooWard =
  Macros.card "Tattoo Ward" (Just [Macros.generic 2, Macros.pip White]) []
       (MkTypeLine [enchantmentType "Aura"] [Enchantment])
       [ Macros.keywordSubject "Enchant" Macros.creature
       , Static (DoesntRemove
                   (SharedSubject (AttachHost Enchanted (TypeW Creature))
                      [ Gets (Macros.ownSubject (AttachHost Enchanted (TypeW Creature)))
                             (PtUp (Lit 1)) (PtUp (Lit 1))
                      , Gains (Macros.ownSubject (AttachHost Enchanted (TypeW Creature)))
                              (Macros.keywordQuality "Protection" (HasType Enchantment)) ])
                   Macros.thisAura)
       , Macros.activated (Do (Macros.sacrifice You Macros.thisAura))
           (Macros.destroy (Macros.target Macros.enchantment)) ]
       Nothing

||| Pentarch Ward
public export
pentarchWard : Card
pentarchWard =
  Macros.card "Pentarch Ward" (Just [Macros.generic 2, Macros.pip White]) []
       (MkTypeLine [enchantmentType "Aura"] [Enchantment])
       [ Macros.keywordSubject "Enchant" Macros.creature
       , Static (Macros.entersChoosing Macros.thisAura Color)
       , Macros.triggered When (Enters Macros.thisAura Nothing) ((Macros.draw You (Lit 1)))
       , Static (DoesntRemove
                   (Gains (AttachHost Enchanted (TypeW Creature))
                          (Macros.keywordQuality "Protection" (OfChosen Color)))
                   Macros.thisAura) ]
       Nothing

||| Benevolent Blessing
public export
benevolentBlessing : Card
benevolentBlessing =
  Macros.card "Benevolent Blessing" (Just [Macros.generic 1, Macros.pip White]) []
       (MkTypeLine [enchantmentType "Aura"] [Enchantment])
       [ Macros.keyword "Flash"
       , Macros.keywordSubject "Enchant" Macros.creature
       , Static (Macros.entersChoosing Macros.thisAura Color)
       , Static (DoesntRemove
                   (Gains (AttachHost Enchanted (TypeW Creature))
                          (Macros.keywordQuality "Protection" (OfChosen Color)))
                   (Macros.allOf (And [ Or [ HasSubtype (enchantmentType "Aura")
                                    , HasSubtype (artifactType "Equipment") ]
                               , HasPossessor ControllerAx You
                               , AttachedTo It ]))) ]
       Nothing

||| Floating Shield
public export
floatingShield : Card
floatingShield =
  Macros.card "Floating Shield" (Just [Macros.generic 2, Macros.pip White]) []
       (MkTypeLine [enchantmentType "Aura"] [Enchantment])
       [ Macros.keywordSubject "Enchant" Macros.creature
       , Static (Macros.entersChoosing Macros.thisAura Color)
       , Static (DoesntRemove
                   (Gains (AttachHost Enchanted (TypeW Creature))
                          (Macros.keywordQuality "Protection" (OfChosen Color)))
                   Macros.thisAura)
       , Macros.activated (Do (Macros.sacrifice You Macros.thisAura))
           (Macros.gains (Macros.target Macros.creature)
                         (Macros.keywordQuality "Protection" (OfChosen Color))
                         (Just Macros.untilEndOfTurn)) ]
       Nothing

||| Summoning Materia
public export
summoningMateriaTopCast : Ability
summoningMateriaTopCast =
  Static (Macros.asLongAs
            (Matches Macros.thisEquipment (AttachedTo (Macros.a Macros.creature)))
            ((Macros.mayPlayDeed "Cast" You (Macros.allOf (And [Macros.spell, HasType Creature])) (PlayRider (Macros.fromZ Macros.onTopZ) Nothing Nothing False ItsOwnCost))))

||| Ghostfire Blade
public export
ghostfireBlade : Card
ghostfireBlade =
  Macros.card "Ghostfire Blade" (Just [Macros.generic 1]) []
       (MkTypeLine [artifactType "Equipment"] [Artifact])
       [ Static (Gets (AttachHost Equipped (TypeW Creature))
                      (PtUp (Lit 2)) (PtUp (Lit 2)))
       , Macros.keywordCosting "Equip" (Mana [Macros.generic 3])
       , Static (CostsToCast
                   (Macros.allOf (And [ AbilityHead (KeywordClass "Equip")
                               , AbilityOf This
                               , Targets (Macros.a (And [Macros.creature, IsColorless]))
                                         SomeTarget ]))
                   (CostLess (Lit 2) Nothing)) ]
       Nothing

||| Luxior, Giada's Gift
public export
luxiorTypeSetting : Ability
luxiorTypeSetting =
  Static (AndAlso
            [ Becomes (AttachHost Equipped PermanentW) Loses
                      (Bundle (MkToken Nothing [] (Macros.typesOnly [Planeswalker]) [] Nothing)
                              Nothing)
            , Becomes It Adds
                      (Bundle (MkToken Nothing [] (MkTypeLine [] [Creature]) [] Nothing)
                              Nothing) ])

||| Vesuvan Shapeshifter
public export
vesuvanShapeshifterCopySpan :
  Effect [MkBinding AD Object OneOf
            (ObjectP (Just Creature) (Just Battlefield) Nothing Nothing Nothing)]
vesuvanShapeshifterCopySpan =
  Continuously {ts = StaticFirstDone}
    (BecomesCopy Macros.thisCreature (That (TypeW Creature))
       [ExceptAbility
          (Macros.triggered At (BeginningOf Upkeep (Macros.yours))
             (Macros.may You (SetStatus FaceDown Macros.thisCreature)))])
    (Just (UntilEvent (StatusEvent Macros.thisCreature FaceDown)))

||| Academy Journeymage
public export
academyJourneymage : Card
academyJourneymage =
  Macros.card "Academy Journeymage" (Just [Macros.generic 4, Macros.pip Blue]) []
       (MkTypeLine [creatureType "Human", creatureType "Wizard"] [Creature])
       [ Static (Macros.onlyIfSo (CostsToCast This (CostLess (Lit 1) Nothing))
                   (Macros.exists (And [HasSubtype (creatureType "Wizard"), HasPossessor ControllerAx You])))
       , Macros.triggered When (Enters Macros.thisCreature Nothing)
           (Macros.move (Macros.target (And [Macros.creature,
                                             HasPossessor ControllerAx Macros.anOpponent]))
                        Macros.handZ) ]
       (Just (3, 2))

||| Alabaster Leech
public export
alabasterLeech : Card
alabasterLeech =
  Macros.card "Alabaster Leech" (Just [Macros.pip White]) []
       (MkTypeLine [creatureType "Leech"] [Creature])
       [ Static (CostsToCast (Macros.allOf (And [Macros.spell, ColorIs White, CastBy You]))
                             (CostShiftRun [Macros.pip White] True False)) ]
       (Just (1, 3))

||| Edgewalker
public export
edgewalker : Card
edgewalker =
  Macros.card "Edgewalker"
       (Just [Macros.generic 1, Macros.pip White, Macros.pip Black]) []
       (MkTypeLine [creatureType "Human", creatureType "Cleric"] [Creature])
       [ Static (CostsToCast (Macros.allOf (And [HasSubtype (creatureType "Cleric"),
                                          Macros.spell, CastBy You]))
                             (CostShiftRun [Macros.pip White, Macros.pip Black]
                                           False True)) ]
       (Just (2, 2))

||| Cavern-Hoard Dragon
public export
cavernHoardDragonRider : Ability
cavernHoardDragonRider =
  Static (AndAlso [ CostsToCast This (CostLess (LetterVal X) Nothing)
                  , Define X (AggregateOver MaxOf Opponent
                                (Macros.countOf (And [Macros.artifact, HasPossessor ControllerAx They]))) ])

||| Propaganda
public export
propaganda : Card
propaganda =
  Macros.card "Propaganda" (Just [Macros.generic 2, Macros.pip Blue]) []
       (MkTypeLine [] [Enchantment])
       [ Static (Macros.deontic (Macros.allOf Macros.creature)
                   (GatedBy (ScaledMana GenericUnit (Times 2
                      (Macros.countOf (And [Macros.creature, HasPossessor ControllerAx They,
                                     CombatRel AttackerOf You])))))
                   ["Attack"] Agent (DefendingPlayer You)) ]
       Nothing

||| Ghostly Prison
public export
ghostlyPrisonWhole : Card
ghostlyPrisonWhole =
  Macros.card "Ghostly Prison" (Just [Macros.generic 2, Macros.pip White]) []
       (MkTypeLine [] [Enchantment])
       [ Static (Macros.deontic (Macros.allOf Macros.creature)
                   (GatedBy (ScaledMana GenericUnit (Times 2
                      (Macros.countOf (And [Macros.creature, HasPossessor ControllerAx They,
                                     CombatRel AttackerOf You])))))
                   ["Attack"] Agent (DefendingPlayer You)) ]
       Nothing

||| Archangel of Tithes
public export
archangelOfTithesBlockToll : Ability
archangelOfTithesBlockToll =
  Static (Macros.asLongAs (Matches Macros.thisCreature Attacking)
            (Macros.deontic (Macros.allOf Macros.creature)
               (GatedBy (ScaledMana GenericUnit (Times 1 GroupSize)))
               ["Block"] Agent NoDeonticPatient))

||| Archangel of Tithes
public export
archangelOfTithesAttackToll : Ability
archangelOfTithesAttackToll =
  Static (Macros.asLongAs (Matches Macros.thisCreature (HasStatus Untapped))
            (Macros.deontic (Macros.allOf Macros.creature)
               (GatedBy (ScaledMana GenericUnit (Times 1 GroupSize)))
               ["Attack"] Agent
               (DefendingPlayer (Macros.youOr
                                   (Macros.allOf (And [HasType Planeswalker,
                                                HasPossessor ControllerAx You]))))))

||| Archon of Absolution
public export
archonOfAbsolution : Card
archonOfAbsolution =
  Macros.card "Archon of Absolution"
       (Just [Macros.generic 3, Macros.pip White]) []
       (MkTypeLine [creatureType "Archon"] [Creature])
       [ Macros.keyword "Flying"
       , Macros.keywordQuality "Protection" (ColorIs White)
       , Static (Macros.deontic (Macros.allOf Macros.creature)
                   (GatedBy (ScaledMana GenericUnit (Times 1 GroupSize)))
                   ["Attack"] Agent
                   (DefendingPlayer (Macros.youOr
                                       (Macros.allOf (And [HasType Planeswalker,
                                                    HasPossessor ControllerAx You]))))) ]
       (Just (3, 2))

||| Myr Prototype
public export
myrPrototype : Card
myrPrototype =
  Macros.card "Myr Prototype" (Just [Macros.generic 5]) []
       (MkTypeLine [creatureType "Myr"] [Artifact, Creature])
       [ Macros.triggered At (BeginningOf Upkeep (Macros.yours))
           (PutCounters (Lit 1) (PrintedKind Macros.plusOnePlusOne) Macros.thisCreature)
       , Static (Macros.deontic Macros.thisCreature
                   (GatedBy (ScaledMana GenericUnit (Times 1
                      (CountersOn Macros.plusOnePlusOne It))))
                   ["Attack", "Block"] Agent NoDeonticPatient) ]
       (Just (3, 3))

||| Heat Wave
public export
heatWave : Card
heatWave =
  Macros.card "Heat Wave" (Just [Macros.generic 2, Macros.pip Red]) []
       (MkTypeLine [] [Enchantment])
       [ Macros.cumulativeUpkeep (Mana [Macros.pip Red])
       , Static (Macros.deontic (Macros.allOf (And [Macros.creature, ColorIs Blue]))
                   Forbid ["Block"] Agent
                   (DeonticCounterpart (Macros.allOf Macros.creatureYouControl)))
       , Static (Macros.deontic (Macros.allOf (And [Macros.creature, Not (ColorIs Blue)]))
                   (GatedBy (Do (Macros.losesLife They
                                   (Times 1 (Macros.countOf (And [Macros.creature, Blocking,
                                                           HasPossessor ControllerAx They]))))))
                   ["Block"] Agent
                   (DeonticCounterpart (Macros.allOf Macros.creatureYouControl))) ]
       Nothing

||| Braid of Fire
public export
braidOfFire : Card
braidOfFire =
  Macros.card "Braid of Fire" (Just [Macros.generic 1, Macros.pip Red]) []
       (MkTypeLine [] [Enchantment])
       [ Macros.cumulativeUpkeep
           (Do (AddMana You (Lit 1) (Runs [[OfColor Red]]) [])) ]
       Nothing

||| Psychic Vortex
public export
psychicVortexUpkeep : Ability
psychicVortexUpkeep =
  Macros.cumulativeUpkeep (Do (Macros.draw You (Lit 1)))

||| Varchild's War-Riders
public export
varchildsWarRidersUpkeep : Ability
varchildsWarRidersUpkeep =
  Macros.cumulativeUpkeep
    (Do (Create Macros.anOpponent (Lit 1)
           (TokenWritten (Macros.creatureTok 1 1 [Red] [creatureType "Survivor"])) []))

||| Wall of Shards
public export
wallOfShards : Card
wallOfShards =
  Macros.card "Wall of Shards" (Just [Macros.generic 1, Macros.pip White]) []
       (MkTypeLine [creatureType "Wall"] [Creature])
       [ Macros.keyword "Defender"
       , Macros.keyword "Flying"
       , Macros.keywordCosting "CumulativeUpkeep"
           (Do (Macros.gainsLife Macros.anOpponent (Lit 1))) ]
       (Just (3, 6))

||| Earthen Goo
public export
earthenGoo : Card
earthenGoo =
  Macros.card "Earthen Goo" (Just [Macros.generic 2, Macros.pip Red]) []
       (MkTypeLine [creatureType "Ooze"] [Creature])
       [ Macros.keyword "Trample"
       , Macros.cumulativeUpkeep
           (EitherCost (Mana [Macros.pip Red]) (Mana [Macros.pip Green]))
       , Static (Gets Macros.thisCreature
                   (PtUp (Times 1 (CountersOn (Named "Age") It)))
                   (PtUp (Times 1 (CountersOn (Named "Age") It)))) ]
       (Just (2, 2))

||| Chamber Sentry
public export
chamberSentryDamage : Ability
chamberSentryDamage =
  Macros.activated (Compound [Mana [Variable], TapSymbol,
                       Do (RemoveCounters (Just (ExactlyOf (LetterVal X)))
                             (Just (PrintedKind Macros.plusOnePlusOne)) Macros.thisCreature)])
                   (DealDamage This (LetterVal X) (Macros.target Macros.anyTarget))

||| Awesome Presence
public export
awesomePresence : Card
awesomePresence =
  Macros.card "Awesome Presence" (Just [Macros.pip Blue]) []
       (MkTypeLine [enchantmentType "Aura"] [Enchantment])
       [ Macros.keywordSubject "Enchant" Macros.creature
       , Static (Macros.deontic (AttachHost Enchanted (TypeW Creature))
                   (GatedBy (ScaledMana GenericUnit (Times 3
                      (Macros.countOf (And [Macros.creature, HasPossessor ControllerAx They,
                                     CombatRel BlockerOf It])))))
                   ["Block"] Patient NoDeonticPatient) ]
       Nothing

||| Oppressive Rays
public export
oppressiveRays : Card
oppressiveRays =
  Macros.card "Oppressive Rays" (Just [Macros.pip White]) []
       (MkTypeLine [enchantmentType "Aura"] [Enchantment])
       [ Macros.keywordSubject "Enchant" Macros.creature
       , Static (Macros.deontic (AttachHost Enchanted (TypeW Creature))
                   (GatedBy (Mana [Macros.generic 3]))
                   ["Attack", "Block"] Agent NoDeonticPatient)
       , Static (CostsToCast
                   (Macros.allOf (And [ AbilityHead AnyActivated
                               , AbilityOf (AttachHost Enchanted (TypeW Creature)) ]))
                   (CostMore (Lit 3))) ]
       Nothing


||| Delighted Halfling
public export
delightedHalflingMana : Ability
delightedHalflingMana =
  Macros.activated TapSymbol
    (AddMana You (Lit 1) (AnyColor SameColor)
      [ OnSpent AffectsIt True
                (Macros.a (And [HasSupertype Legendary, Macros.spell]))
                (Continuously {ts = StaticFirstDone} (Macros.objectCant "Counter" (That SpellW)) Nothing) ])

||| Boseiju, Who Shelters All
public export
boseijuMana : Ability
boseijuMana =
  Macros.activated (Compound [TapSymbol, Do (Macros.losesLife You (Lit 2))])
    (AddMana You (Lit 1) (Runs [[Colorless]])
      [ OnSpent AffectsIt False
                (Macros.a (And [Macros.instantOrSorcery, Macros.spell]))
                (Continuously {ts = StaticFirstDone} (Macros.objectCant "Counter" (That SpellW))
                              Nothing) ])

||| Generator Servant
public export
generatorServant : Card
generatorServant =
  Macros.card "Generator Servant"
       (Just [Macros.generic 1, Macros.pip Red]) []
       (MkTypeLine [creatureType "Elemental"] [Creature])
       [ Macros.activated
           (Compound [TapSymbol, Do (Macros.sacrifice You Macros.thisCreature)])
           (AddMana You (Lit 1) (Runs [[Colorless, Colorless]])
             [ OnSpent AffectsIt False
                       (Macros.a (And [Macros.creature, Macros.spell]))
                       (Macros.gainsHaste (ResolvedPermanent (That SpellW))
                                          (Just Macros.untilEndOfTurn)) ]) ]
       (Just (2, 1))

||| Animal Attendant
public export
animalAttendant : Card
animalAttendant =
  Macros.card "Animal Attendant"
       (Just [Macros.generic 1, Macros.pip Green]) []
       (MkTypeLine [creatureType "Human", creatureType "Citizen"] [Creature])
       [ Macros.activated TapSymbol
           (AddMana You (Lit 1) (AnyColor SameColor)
             [ OnSpent AffectsIt False
                       (Macros.a (And [Not (HasSubtype (creatureType "Human")),
                                       Macros.creature, Macros.spell]))
                       (Continuously {ts = StaticFirstDone}
                          (EntersRider (ResolvedPermanent (That SpellW))
                                       (WithCounters (Lit 1)
                                              (PrintedKind Macros.plusOnePlusOne)
                                              Additional))
                          Nothing) ]) ]
       (Just (2, 2))

||| Pyromancer's Goggles
public export
pyromancersGogglesMana : Ability
pyromancersGogglesMana =
  Macros.activated TapSymbol
    (AddMana You (Lit 1) (Runs [[OfColor Red]])
      [ OnSpent TriggersThen False
                (Macros.a (And [ColorIs Red, Macros.instantOrSorcery, Macros.spell]))
                (CopyStack You (That SpellW) (Lit 1) []) ])

||| Thran Turbine
public export
thranTurbineMana : Effect []
thranTurbineMana =
  AddMana You (Lit 1) (Runs [[Colorless, Colorless]])
          [SpendNotOn [ToCast Macros.spell]]

||| Su-Chi Cave Guard
public export
suChiCaveGuardDies : Ability
suChiCaveGuardDies =
  Macros.triggered When (Dies Macros.thisCreature)
    (Sequentially
       [ AddMana You (Lit 1)
                 (Runs [[Colorless, Colorless, Colorless, Colorless,
                         Colorless, Colorless, Colorless, Colorless]]) []
       , Continuously {ts = StaticFirstDone} (KeepsUnspentMana You ThisMana)
                      (Just Macros.untilEndOfTurn) ])

||| Omnath, Locus of Mana
public export
omnathLocusOfManaPersistence : StaticEffect []
omnathLocusOfManaPersistence =
  KeepsUnspentMana You (UnspentMana (Just (OfColor Green)))

||| Upwelling
public export
upwelling : Card
upwelling =
  Macros.card "Upwelling" (Just [Macros.generic 4, Macros.pip Green]) []
       (MkTypeLine [] [Enchantment])
       [ Static (KeepsUnspentMana (Macros.each AnyPlayer) (UnspentMana Nothing)) ]
       Nothing

||| Mana Flare
public export
manaFlare : Card
manaFlare =
  Macros.card "Mana Flare" (Just [Macros.generic 2, Macros.pip Red]) []
       (MkTypeLine [] [Enchantment])
       [ Macros.triggered Whenever
           (VerbedEvent (Just (Macros.a AnyPlayer)) "Tap"
                        (Just (Macros.a Macros.land)) Nothing True)
           (AddMana (That PlayerW) (Lit 1)
                    (ProducedByEvent (That (TypeW Land))) []) ]
       Nothing

||| Shimmerwilds Growth
public export
shimmerwildsGrowth : Card
shimmerwildsGrowth =
  Macros.card "Shimmerwilds Growth" (Just [Macros.pip Green]) []
       (MkTypeLine [enchantmentType "Aura"] [Enchantment])
       [ Macros.keywordSubject "Enchant" Macros.land
       , Static (Macros.entersChoosing Macros.thisAura Color)
       , Static (Becomes (AttachHost Enchanted (TypeW Land)) Sets (ChosenQuality (OfChosen Color)))
       , Macros.triggered Whenever
           (VerbedEvent Nothing "Tap"
                        (Just (AttachHost Enchanted (TypeW Land))) Nothing True)
           (AddMana (Macros.controllerOf (That (TypeW Land))) (Lit 1)
                    (OfChosenColor Nothing) []) ]
       Nothing

||| Chrome Mox
public export
chromeMoxMana : Ability
chromeMoxMana =
  Macros.activated TapSymbol
    (AddMana You (Lit 1)
             (AmongColorsOf (Macros.a (ExiledWith Macros.thisArtifact))) [])

||| Fellwar Stone
public export
fellwarStone : Card
fellwarStone =
  Macros.card "Fellwar Stone" (Just [Macros.generic 2]) []
       (MkTypeLine [] [Artifact])
       [ Macros.activated TapSymbol
           (AddMana You (Lit 1)
                    (CouldProduce (Macros.a (And [Macros.land,
                                                  HasPossessor ControllerAx Macros.anOpponent])))
                    []) ]
       Nothing

||| Ice Cauldron
public export
iceCauldronNotedMana : Ability
iceCauldronNotedMana =
  Macros.activated
    (Compound [TapSymbol,
               Do (RemoveCounters (Just (Macros.exactly 1)) (Just (PrintedKind (Named "Charge")))
                                  Macros.thisArtifact)])
    (AddMana You (Lit 1) (LastNotedMana Macros.thisArtifact)
             [SpendOnly [ToCast (ExiledWith Macros.thisArtifact)]])

||| Firemind Vessel
public export
firemindVesselMana : Ability
firemindVesselMana =
  Macros.activated TapSymbol
    (AddMana You (Lit 2) (AnyColor DistinctColors) [])

||| Goblin Clearcutter
public export
goblinClearcutterMana : Ability
goblinClearcutterMana =
  Macros.activated
    (Compound [TapSymbol,
               Do (Macros.sacrifice You (Macros.a (HasSubtype (landType "Forest"))))])
    (AddMana You (Lit 3) (AmongWritten [Red, Green]) [])

||| Vizier of the Menagerie
public export
vizierOfTheMenagerieSpend : StaticEffect []
vizierOfTheMenagerieSpend =
  Macros.maySpendAsThough You Nothing MatchAnyType
    (Just (ToCast (And [Macros.creature, Macros.spell])))

||| Vexing Bauble
public export
vexingBaubleTrigger : Ability
vexingBaubleTrigger =
  Macros.triggeredIf Whenever
    (Casts (Macros.a AnyPlayer) (Macros.a Macros.spell) Nothing)
    (NotCond (ManaSpentToCast It Nothing))
    (CounterSpell (That SpellW))

||| Void Mirror
public export
voidMirror : Card
voidMirror =
  Macros.card "Void Mirror" (Just [Macros.generic 2]) []
       (MkTypeLine [] [Artifact])
       [ Macros.triggeredIf Whenever
           (Casts (Macros.a AnyPlayer) (Macros.a Macros.spell) Nothing)
           (NotCond (ManaSpentToCast It (Just MatchAnyColor)))
           (CounterSpell (That SpellW)) ]
       Nothing



||| Arlinn Kord // Arlinn, Embraced by the Moon
public export
arlinnKord : Card
arlinnKord =
  Transforming
    (Macros.frontFace "Arlinn Kord" (Just [Macros.generic 2, Macros.pip Red, Macros.pip Green])
            [Legendary] (MkTypeLine [planeswalkerType "Arlinn"] [Planeswalker])
            [ Macros.activated (LoyaltySymbol (LoyaltyUp 1))
                (Continuously {ts = StaticFirstDone}
                   (AndAlso [ Gets (Macros.targets (Macros.upTo 1) Macros.creature)
                                   (PtUp (Lit 2)) (PtUp (Lit 2))
                            , Gains It (Macros.keyword "Vigilance")
                            , Gains It (Macros.keyword "Haste") ])
                   (Just Macros.untilEndOfTurn))
            , Macros.activated (LoyaltySymbol LoyaltyZero)
                (Sequentially
                   [ Macros.create (Lit 1)
                       (Macros.creatureTok 2 2 [Green] [creatureType "Wolf"])
                   , Macros.transform Macros.thisPlaneswalker ]) ]
            (Macros.loyaltyBox 3))
    (Macros.backFace "Arlinn, Embraced by the Moon" [Legendary]
               (MkTypeLine [planeswalkerType "Arlinn"] [Planeswalker])
               [ Macros.activated (LoyaltySymbol (LoyaltyUp 1))
                   (Continuously {ts = StaticFirstDone}
                      (AndAlso [ Gets (Macros.allOf Macros.creatureYouControl)
                                      (PtUp (Lit 1)) (PtUp (Lit 1))
                               , Gains Them (Macros.keyword "Trample") ])
                      (Just Macros.untilEndOfTurn))
               , Macros.activated (LoyaltySymbol (LoyaltyDown 1))
                   (Sequentially
                      [ DealDamage This (Lit 3) (Macros.target Macros.anyTarget)
                      , Macros.transform Macros.thisPlaneswalker ])
               , Macros.activated (LoyaltySymbol (LoyaltyDown 6))
                   (GetsEmblem You
                      [ Static (AndAlso
                          [ Gains (Macros.allOf Macros.creatureYouControl) (Macros.keyword "Haste")
                          , Gains Them (Macros.activated TapSymbol
                              (DealDamage Macros.thisCreature (StatOf Power Macros.thisCreature)
                                          (Macros.target Macros.anyTarget))) ]) ]) ]
               Nothing)

||| Neglected Heirloom // Ashmouth Blade
public export
neglectedHeirloom : Card
neglectedHeirloom =
  Transforming
    (Macros.frontFace "Neglected Heirloom" (Just [Macros.generic 1]) []
            (MkTypeLine [artifactType "Equipment"] [Artifact])
            [ Static (Gets (AttachHost Equipped (TypeW Creature))
                           (PtUp (Lit 1)) (PtUp (Lit 1)))
            , Macros.triggered When
                (VerbedEvent Nothing "Transform"
                  (Just (AttachHost Equipped (TypeW Creature))) Nothing False)
                (Macros.transform Macros.thisEquipment)
            , Macros.keywordCosting "Equip" (Mana [Macros.generic 1]) ]
            Nothing)
    (Macros.backFace "Ashmouth Blade" []
               (MkTypeLine [artifactType "Equipment"] [Artifact])
               [ Static (AndAlso [ Gets (AttachHost Equipped (TypeW Creature))
                                        (PtUp (Lit 3)) (PtUp (Lit 3))
                                 , Gains It (Macros.keyword "FirstStrike") ])
               , Macros.keywordCosting "Equip" (Mana [Macros.generic 3]) ]
               Nothing)

||| Harvest Hand // Scrounged Scythe
public export
harvestHand : Card
harvestHand =
  Transforming
    (Macros.frontFace "Harvest Hand" (Just [Macros.generic 3]) []
            (MkTypeLine [creatureType "Scarecrow"] [Artifact, Creature])
            [ Macros.triggered When (Dies Macros.thisCreature)
                (Macros.returnToBattlefieldTransformed It You) ]
            (Macros.printedBox (Just (2, 2))))
    (Macros.backFace "Scrounged Scythe" []
               (MkTypeLine [artifactType "Equipment"] [Artifact])
               [ Static (Gets (AttachHost Equipped (TypeW Creature))
                              (PtUp (Lit 1)) (PtUp (Lit 1)))
               , Static (Macros.asLongAs
                           (Matches (AttachHost Equipped (TypeW Creature))
                                    (HasSubtype (creatureType "Human")))
                           (Gains It (Macros.keyword "Menace")))
               , Macros.keywordCosting "Equip" (Mana [Macros.generic 2]) ]
               Nothing)

||| Cult of the Waxing Moon
public export
cultOfTheWaxingMoon : Card
cultOfTheWaxingMoon =
  Macros.card "Cult of the Waxing Moon"
       (Just [Macros.generic 4, Macros.pip Green]) []
       (MkTypeLine [creatureType "Human", creatureType "Shaman"] [Creature])
       [ Macros.triggered Whenever
           (VerbedEvent Nothing "Transform"
              (Just (Macros.a (And [Permanent, HasPossessor ControllerAx You])))
              (Just (And [Macros.creature,
                          Not (HasSubtype (creatureType "Human"))])) False)
           (Macros.create (Lit 1)
              (Macros.creatureTok 2 2 [Green] [creatureType "Wolf"])) ]
       (Just (5, 4))

||| Mutagen Connoisseur
public export
mutagenConnoisseur : Card
mutagenConnoisseur =
  Macros.card "Mutagen Connoisseur"
       (Just [Macros.generic 1, Macros.pip Green, Macros.pip Blue]) []
       (MkTypeLine [creatureType "Vedalken", creatureType "Mutant"] [Creature])
       [ Macros.keyword "Flying"
       , Macros.keyword "Vigilance"
       , Static (Gets Macros.thisCreature
                      (PtUp (Macros.nForEach 1
                               (And [IsTransformed, HasPossessor ControllerAx You])))
                      (PtUp (Lit 0))) ]
       (Just (0, 5))


public export
chitteringHostOnScavengers : CardFace
chitteringHostOnScavengers =
  Macros.backFace "Chittering Host" []
            (MkTypeLine [creatureType "Eldrazi", creatureType "Horror"] [Creature])
            [ Macros.keyword "Haste"
            , Macros.keyword "Menace"
            , Macros.triggered When (Enters Macros.thisCreature Nothing)
                (Continuously {ts = StaticFirstDone}
                   (AndAlso [ Gets (Macros.allOf (Macros.otherCreatureYouControl Macros.thisCreature))
                                   (PtUp (Lit 1)) (PtUp (Lit 0))
                            , Gains Them (Macros.keyword "Menace") ])
                   (Just Macros.untilEndOfTurn)) ]
            (Macros.printedBox (Just (5, 6)))

||| Chittering Host as the GRAF RATS card carries it
public export
chitteringHostOnGrafRats : CardFace
chitteringHostOnGrafRats =
  Macros.backFace "Chittering Host" []
            (MkTypeLine [creatureType "Eldrazi", creatureType "Horror"] [Creature])
            [ Macros.keyword "Haste"
            , Macros.keyword "Menace"
            , Macros.triggered When (Enters Macros.thisCreature Nothing)
                (Continuously {ts = StaticFirstDone}
                   (AndAlso [ Gets (Macros.allOf (Macros.otherCreatureYouControl Macros.thisCreature))
                                   (PtUp (Lit 1)) (PtUp (Lit 0))
                            , Gains Them (Macros.keyword "Menace") ])
                   (Just Macros.untilEndOfTurn)) ]
            (Macros.printedBox (Just (5, 6)))

public export
meldBackFacesAgree : Cards.chitteringHostOnScavengers = Cards.chitteringHostOnGrafRats
meldBackFacesAgree = Refl

||| Midnight Scavengers // Chittering Host
public export
midnightScavengers : Card
midnightScavengers =
  Transforming
    (Macros.frontFace "Midnight Scavengers" (Just [Macros.generic 4, Macros.pip Black]) []
            (MkTypeLine [creatureType "Human", creatureType "Rogue"] [Creature])
            [ Macros.triggered When (Enters Macros.thisCreature Nothing)
                (Macros.may You
                   (Macros.returnTo
                      (Macros.target (And [ Macros.creature
                                          , InZone (Macros.graveyardOf You)
                                          , Compare [CharAxis ManaValue] AtMost (Lit 3) ]))
                      Macros.handZ)) ]
            (Macros.printedBox (Just (3, 3))))
    Cards.chitteringHostOnScavengers

public export
meldThemInto : Effect []
meldThemInto =
  Sequentially
    [ Macros.exile You (Macros.allOf (And [Macros.creature, HasPossessor ControllerAx You]))
    , Macros.meldInto (Macros.themVerbed "Exile") "Chittering Host" ]

||| Profit // Loss
public export
profitLoss : Card
profitLoss =
  SplitCard
    (Macros.frontFace "Profit" (Just [Macros.generic 1, Macros.pip White]) []
            (MkTypeLine [] [Instant])
            [ Spell (Macros.gets (Macros.allOf Macros.creatureYouControl)
                                 (PtUp (Lit 1)) (PtUp (Lit 1))
                                 (Just Macros.untilEndOfTurn))
            , Macros.keyword "Fuse" ]
            Nothing)
    (Macros.frontFace "Loss" (Just [Macros.generic 2, Macros.pip Black]) []
            (MkTypeLine [] [Instant])
            [ Spell (Macros.gets
                       (Macros.allOf Macros.creatureYourOpponentsControl)
                       (PtDown (Lit 1)) (PtDown (Lit 1))
                       (Just Macros.untilEndOfTurn))
            , Macros.keyword "Fuse" ]
            Nothing)

||| Kasmina, Enigma Sage
public export
kasminaLoyaltySharing : StaticEffect []
kasminaLoyaltySharing =
  GainsAbilitiesOf (Macros.allOf (And [HasType Planeswalker, HasPossessor ControllerAx You,
                                OtherThan This]))
                   [LoyaltyClass] This Nothing

||| Nicol Bolas, Dragon-God
public export
nicolBolasDragonGodSharing : StaticEffect []
nicolBolasDragonGodSharing =
  GainsAbilitiesOf This [LoyaltyClass]
                   (Macros.allOf (And [HasType Planeswalker, OtherThan This,
                                InZone Macros.battlefieldZ]))
                   Nothing

||| Myr Welder
public export
myrWelderBorrowedAbilities : StaticEffect []
myrWelderBorrowedAbilities =
  GainsAbilitiesOf Macros.thisCreature [AnyActivated]
                   (Macros.allOf (ExiledWith This)) Nothing

||| Sharkey, Tyrant of the Shire
public export
sharkeyBorrowedLandAbilities : StaticEffect []
sharkeyBorrowedLandAbilities =
  GainsAbilitiesOf This [AnyActivated]
                   (Macros.allOf (And [Macros.land,
                                HasPossessor ControllerAx (PlayerGroup YourOpponents)]))
                   (Just IsManaAbility)


||| Conspicuous Snoop
public export
conspicuousSnoop : Card
conspicuousSnoop =
  Macros.card "Conspicuous Snoop" (Just [Macros.pip Red, Macros.pip Red]) []
       (MkTypeLine [creatureType "Goblin", creatureType "Rogue"] [Creature])
       [ Static (Visibility Reveal You TopOfLibrary)
       , Static ((Macros.mayPlayDeed "Play" You (Macros.allOf (And [Macros.spell,
                                HasSubtype (creatureType "Goblin")])) (PlayRider (Macros.fromZ Macros.onTopZ) Nothing Nothing False ItsOwnCost)))
       , Static (Macros.asLongAs
                   (Matches Macros.topCard (HasSubtype (creatureType "Goblin")))
                   (GainsAbilitiesOf Macros.thisCreature [AnyActivated]
                                     (That CardW) Nothing)) ]
       (Just (2, 2))

||| Skill Borrower
public export
skillBorrower : Card
skillBorrower =
  Macros.card "Skill Borrower"
       (Just [Macros.generic 2, Macros.pip Blue]) []
       (MkTypeLine [creatureType "Human", creatureType "Wizard"]
                   [Artifact, Creature])
       [ Static (Visibility Reveal You TopOfLibrary)
       , Static (Macros.asLongAs
                   (Matches Macros.topCard (Or [Macros.artifact, Macros.creature]))
                   (GainsAbilitiesOf Macros.thisCreature [AnyActivated]
                                     (That CardW) Nothing)) ]
       (Just (1, 3))

||| Blind Fury
public export
blindFury : Card
blindFury =
  Macros.card "Blind Fury"
       (Just [Macros.generic 2, Macros.pip Red, Macros.pip Red]) []
       (MkTypeLine [] [Instant])
       [ Spell (Sequentially
                  [ Continuously {ts = StaticFirstDone}
                      (LosesAbilities (Macros.allOf Macros.creature)
                                      [LostWritten (KeywordAbility "Trample" Nothing Nothing)])
                      (Just Macros.untilEndOfTurn)
                  , Continuously {ts = StaticFirstDone}
                      (Scales CombatOnly (Macros.a Macros.creature)
                              (Macros.shieldingIt (Macros.a Macros.creature))
                              (Multiplied Doubled) Repeatedly)
                      (Just Macros.thisTurn) ]) ]
       Nothing

||| Shadowspear
public export
shadowspearStrip : Ability
shadowspearStrip =
  Macros.activated (Mana [Macros.generic 1])
    (Continuously {ts = StaticFirstDone}
       (LosesAbilities
          (Macros.allOf (And [Permanent, HasPossessor ControllerAx (PlayerGroup YourOpponents)]))
          [ LostWritten (KeywordAbility "Hexproof" Nothing Nothing)
          , LostWritten (KeywordAbility "Indestructible" Nothing Nothing) ])
       (Just Macros.untilEndOfTurn))

||| Shay Cormac
public export
shayCormacStrip : Ability
shayCormacStrip =
  Macros.activated (Mana [Macros.generic 1])
    (Continuously {ts = StaticFirstDone}
       (LosesAbilities
          (Macros.allOf (And [Permanent, HasPossessor ControllerAx (PlayerGroup YourOpponents)]))
          [ LostWritten (KeywordAbility "Hexproof" Nothing Nothing)
          , LostWritten (KeywordAbility "Indestructible" Nothing Nothing)
          , LostTerm protectionAbilities
          , LostWritten (KeywordAbility "Shroud" Nothing Nothing)
          , LostTerm wardAbilities ])
       (Just Macros.untilEndOfTurn))

||| Shelkin Brownie
public export
shelkinBrownie : Card
shelkinBrownie =
  Macros.card "Shelkin Brownie"
       (Just [Macros.generic 1, Macros.pip Green]) []
       (MkTypeLine [creatureType "Ouphe"] [Creature])
       [ Macros.activated (Compound [TapSymbol])
           (Continuously {ts = StaticFirstDone}
              (LosesAbilities (Macros.target Macros.creature)
                              [LostTerm bandsWithOtherAbilities])
              (Just Macros.untilEndOfTurn)) ]
       (Just (1, 1))

||| Tolaria
public export
tolaria : Card
tolaria =
  Macros.card "Tolaria" Nothing [Legendary] (MkTypeLine [] [Land])
       [ Macros.activated TapSymbol (AddMana You (Lit 1) (Runs [[OfColor Blue]]) [])
       , Activated (Compound [TapSymbol])
           (Continuously {ts = StaticFirstDone}
              (LosesAbilities (Macros.target Macros.creature)
                              [ LostWritten (KeywordAbility "Banding" Nothing Nothing)
                              , LostTerm bandsWithOtherAbilities ])
              (Just Macros.untilEndOfTurn))
           (Just (DuringPart Upkeep Nothing)) Nothing Nothing Nothing ]
       Nothing

||| Blood Sun
public export
bloodSun : Card
bloodSun =
  Macros.card "Blood Sun" (Just [Macros.generic 2, Macros.pip Red]) []
       (MkTypeLine [] [Enchantment])
       [ Macros.triggered When (Enters Macros.thisEnchantment Nothing)
                          (Macros.draw You (Lit 1))
       , Static (LosesAllAbilities (Macros.allOf Macros.land) (Just IsManaAbility)) ]
       Nothing


||| Ahn-Crop Invader
public export
ahnCropInvader : Card
ahnCropInvader =
  Macros.card "Ahn-Crop Invader" (Just [Macros.generic 2, Macros.pip Red]) []
       (MkTypeLine [creatureType "Zombie", creatureType "Minotaur",
                    creatureType "Warrior"] [Creature])
       [ Static (OnlyDuring Turn (Just You)
                   (Gains Macros.thisCreature
                          (KeywordAbility "FirstStrike" Nothing Nothing)))
       , Macros.activated
           (Compound [ Mana [Macros.generic 1]
                     , Do (Macros.sacrifice You
                             (Macros.a (And [Macros.creature, OtherThan This]))) ])
           (Macros.gets Macros.thisCreature (PtUp (Lit 2)) (PtUp (Lit 0))
                        (Just Macros.untilEndOfTurn)) ]
       (Just (2, 2))

||| Bedrock Tortoise
public export
bedrockTortoiseWindow : StaticEffect []
bedrockTortoiseWindow =
  OnlyDuring Turn (Just You)
    (Gains (Macros.allOf Macros.creatureYouControl)
           (KeywordAbility "Hexproof" Nothing Nothing))


||| Nesting Dragon
public export
nestingDragonInnerToken : AbilityAt []
nestingDragonInnerToken =
  Macros.activated (Mana [Macros.pip Red])
    (Macros.gets (AsMarker TokenMarker This) (PtUp (Lit 1)) (PtUp (Lit 0))
                 (Just Macros.untilEndOfTurn))

||| Chandra, Awakened Inferno's emblem
public export
chandraAwakenedInfernoEmblem : Effect []
chandraAwakenedInfernoEmblem =
  GetsEmblem You
    [ Macros.triggered At (BeginningOf Upkeep (Macros.yours))
        (DealDamage (AsMarker EmblemMarker This) (Lit 1) You) ]

||| Leonin Bola
public export
leoninBola : Card
leoninBola =
  Macros.card "Leonin Bola" (Just [Macros.generic 1]) []
       (MkTypeLine [artifactType "Equipment"] [Artifact])
       [ Static (Gains (AttachHost Equipped (TypeW Creature))
                   (Macros.activated
                      (Compound [TapSymbol, Do (Unattach TheGrantor)])
                      (SetStatus Tapped (Macros.target Macros.creature))))
       , Macros.keywordCosting "Equip" (Mana [Macros.generic 1]) ]
       Nothing


||| Distortion Strike
public export
distortionStrikeLine : Effect []
distortionStrikeLine =
  Macros.sharedSubject (Macros.target Macros.creature)
    [ Gets (Macros.ownSubject (Macros.target Macros.creature)) (PtUp (Lit 1)) (PtUp (Lit 0))
    , Deontic (Macros.ownSubject (Macros.target Macros.creature)) Forbid ["Block"] Patient Nothing
              NoDeonticPatient Nothing NoDeonticRider ]
    (Just Macros.untilEndOfTurn)

||| Battlegate Mimic
public export
battlegateMimic : Card
battlegateMimic =
  Macros.card "Battlegate Mimic"
       (Just [Macros.generic 1, Macros.hybridPip Red White]) []
       (MkTypeLine [creatureType "Shapeshifter"] [Creature])
       [ Macros.triggered Whenever
           (Casts You (Macros.a (And [Macros.spell, ColorIs Red,
                                      ColorIs White])) Nothing)
           (Continuously {ts = StaticFirstDone}
              (AndAlso [ HasBasePt Macros.thisCreature (Lit 4) (Lit 2)
                       , Gains Macros.thisCreature
                               (Macros.keyword "FirstStrike") ])
              (Just Macros.untilEndOfTurn)) ]
       (Just (2, 1))



||| Retro-Mutation
public export
retroMutation : Card
retroMutation =
  Macros.card "Retro-Mutation" (Just [Macros.generic 2, Macros.pip Blue]) []
       (MkTypeLine [enchantmentType "Aura"] [Enchantment])
       [ Macros.keywordSubject "Enchant" Macros.creature
       , Static (AndAlso
           [ Becomes (AttachHost Enchanted (TypeW Creature)) Sets (Bundle (MkToken Nothing []
                               (MkTypeLine [creatureType "Turtle"] []) [] Nothing) Nothing)
           , HasBasePt It (Lit 0) (Lit 1)
           , Deontic It Forbid ["Attack"] Agent Nothing NoDeonticPatient Nothing
                      NoDeonticRider
           , LosesAllAbilities It Nothing ]) ]
       Nothing

||| Alluring Suitor
public export
alluringSuitorPump : Ability
alluringSuitorPump =
  Macros.activated (Mana [Macros.pip Red, Macros.pip Red])
    (Macros.gets
       (EachOf
          (Both Macros.thisCreature
                  (Macros.target (And [Macros.creature, OtherThan This]))))
       (PtUp (Lit 1)) (PtUp (Lit 0)) (Just Macros.untilEndOfTurn))




||| Hotshot Mechanic
public export
hotshotMechanic : Card
hotshotMechanic =
  Macros.card "Hotshot Mechanic" (Just [Macros.pip White]) []
       (MkTypeLine [creatureType "Fox", creatureType "Pilot"] [Artifact, Creature])
       [ Static (Deontic Macros.thisCreature Permit ["Crew"] Agent Nothing
                   (DeonticCounterpart (Macros.allOf (HasSubtype (artifactType "Vehicle"))))
                   (Just (AsThoughGreater Power (Lit 2)))
                   NoDeonticRider) ]
       (Just (2, 1))

||| Cloudspire Captain
public export
cloudspireCaptainCrewLine : StaticEffect []
cloudspireCaptainCrewLine =
  Deontic Macros.thisCreature Permit ["Saddle", "Crew"] Agent Nothing
          (CounterpartsAt
             [ MkDeedComplement "Saddle" (Macros.allOf (HasSubtype (creatureType "Mount")))
             , MkDeedComplement "Crew" (Macros.allOf (HasSubtype (artifactType "Vehicle"))) ])
          (Just (AsThoughGreater Power (Lit 2)))
          NoDeonticRider

||| Revoke Privileges
public export
revokePrivileges : Card
revokePrivileges =
  Macros.card "Revoke Privileges" (Just [Macros.generic 2, Macros.pip White]) []
       (MkTypeLine [enchantmentType "Aura"] [Enchantment])
       [ Macros.keywordSubject "Enchant" Macros.creature
       , Static (Deontic (AttachHost Enchanted (TypeW Creature))
                   Forbid ["Attack", "Block", "Crew"] Agent Nothing
                   (CounterpartsAt
                      [ MkDeedComplement "Crew"
                          (Macros.allOf (HasSubtype (artifactType "Vehicle"))) ])
                   Nothing NoDeonticRider) ]
       Nothing

||| Harried Spearguard
public export
harriedSpearguard : Card
harriedSpearguard =
  Macros.card "Harried Spearguard" (Just [Macros.pip Red]) []
       (MkTypeLine [creatureType "Human", creatureType "Soldier"] [Creature])
       [ Macros.keyword "Haste"
       , Macros.triggered When (Dies Macros.thisCreature)
           (Macros.create (Lit 1)
              (MkToken (Just (Lit 1 ** Lit 1)) [Black]
                       (MkTypeLine [creatureType "Rat"] [Creature])
                       [ Static (Deontic (AsMarker TokenMarker This)
                                   Forbid ["Block"] Agent Nothing NoDeonticPatient
                                   Nothing NoDeonticRider) ]
                       Nothing)) ]
       (Just (1, 1))


||| Glassworks // Shattered Yard
public export
glassworksShatteredYard : Card
glassworksShatteredYard =
  SharedLineSplit (MkTypeLine [enchantmentType "Room"] [Enchantment]) [] Nothing
    (MkSharedHalf "Glassworks" (Just [Macros.generic 2, Macros.pip Red])
       [ Macros.triggered When (UnlocksDoor You ThisDoor)
           (DealDamage Macros.thisRoom (Lit 4)
              (Macros.target (And [Macros.creature,
                                   HasPossessor ControllerAx Macros.anOpponent]))) ])
    (MkSharedHalf "Shattered Yard" (Just [Macros.generic 4, Macros.pip Red])
       [ Macros.triggered At (BeginningOf EndStep (Macros.yours))
           (DealDamage Macros.thisRoom (Lit 1) (Macros.each Opponent)) ])

||| Balemurk Leech
public export
balemurkLeech : Card
balemurkLeech =
  Macros.card "Balemurk Leech" (Just [Macros.generic 1, Macros.pip Black]) []
       (MkTypeLine [creatureType "Leech"] [Creature])
       [ Macros.abilityWord Eerie
           (Macros.triggeredJoined Whenever
              (Enters (Macros.a (And [Macros.enchantment, HasPossessor ControllerAx You]))
                      Nothing)
              [ Macros.joinedHead Whenever
                  (VerbedEvent (Just You) "Fully Unlock"
                     (Just (Macros.a (HasSubtype (enchantmentType "Room"))))
                     Nothing False) ]
              (Macros.losesLife (Macros.each Opponent) (Lit 1))) ]
       (Just (2, 2))

||| Ghostly Keybearer
public export
ghostlyKeybearer : Card
ghostlyKeybearer =
  Macros.card "Ghostly Keybearer" (Just [Macros.generic 3, Macros.pip Blue]) []
       (MkTypeLine [creatureType "Spirit"] [Creature])
       [ Macros.keyword "Flying"
       , Macros.triggered Whenever
           (Macros.dealsCombatDamage Macros.thisCreature (Macros.a AnyPlayer))
           (Unlock (DoorOf (Just Locked)
                      (Macros.targets (Macros.upTo 1)
                         (And [HasSubtype (enchantmentType "Room"),
                               HasPossessor ControllerAx You])))) ]
       (Just (3, 3))



||| Arrest
public export
arrest : Card
arrest =
  Macros.card "Arrest" (Just [Macros.generic 2, Macros.pip White]) []
       (MkTypeLine [enchantmentType "Aura"] [Enchantment])
       [ Macros.keywordSubject "Enchant" Macros.creature
       , Static (AndAlso
                   [ Macros.deontic (AttachHost Enchanted (TypeW Creature))
                       Forbid ["Attack", "Block"] Agent NoDeonticPatient
                   , Macros.deontic (Macros.allOf (And [AbilityHead AnyActivated,
                                                 AbilityOf It]))
                       Forbid ["Activate"] Patient NoDeonticPatient ]) ]
       Nothing

||| Conqueror's Flail
public export
conquerorsFlailProhibition : StaticEffect []
conquerorsFlailProhibition =
  Macros.asLongAs (Matches This (AttachedTo (Macros.a Macros.creature)))
    (OnlyDuring Turn (Just You)
       (Macros.cantDoTo "Cast" (PlayerGroup YourOpponents) (Macros.allOf Macros.spell)))


||| Keeper of the Flame
public export
keeperOfTheFlame : Card
keeperOfTheFlame =
  Macros.card "Keeper of the Flame" (Just [Macros.generic 1, Macros.pip Red]) []
       (MkTypeLine [creatureType "Human", creatureType "Cleric"] [Creature])
       [ Macros.activated (Compound [Mana [Macros.pip Red], TapSymbol])
           (Sequentially
              [ Macros.choose
                  (Macros.target
                     (And [ Opponent
                          , Compare [PlayerStatAxis LifeTotal] Greater
                                    (PlayerStatOf LifeTotal You) ]))
              , DealDamage Macros.thisCreature (Lit 2) (That PlayerW) ]) ]
       (Just (1, 1))


||| Menacing Ogre
public export
menacingOgre : Card
menacingOgre =
  Macros.card "Menacing Ogre"
       (Just [Macros.generic 3, Macros.pip Red, Macros.pip Red]) []
       (MkTypeLine [creatureType "Ogre"] [Creature])
       [ Macros.keyword "Trample"
       , Macros.keyword "Haste"
       , Macros.triggered When (Enters Macros.thisCreature Nothing)
           (Sequentially
              [ Macros.secretlyChooses (Macros.each AnyPlayer)
                                       (Macros.a (Macros.quality Number))
              , ChoicesRevealed HiddenNumbers
              , Macros.losesLife (Macros.each (And [AnyPlayer, ChoseExtreme MaxOf]))
                                 ThatMuch
              , Macros.ifThen (Matches You (ChoseExtreme MaxOf))
                              (PutCounters (Lit 2)
                                           (PrintedKind Macros.plusOnePlusOne)
                                           Macros.thisCreature) ]) ]
       (Just (3, 3))

||| Tyrant's Choice
public export
tyrantsChoice : Card
tyrantsChoice =
  Macros.card "Tyrant's Choice"
       (Just [Macros.generic 1, Macros.pip Black]) []
       (MkTypeLine [] [Sorcery])
       [ Macros.abilityWord WillOfTheCouncil
           (Spell (Sequentially
              [ Macros.vote (Macros.each AnyPlayer) Openly (ByLabel ["death", "torture"])
              , Macros.ifThen (VoteLead "death" False)
                  (Macros.sacrifice (Macros.each Opponent)
                                    (Macros.aTheirChoice Macros.creature))
              , Macros.ifThen (VoteLead "torture" True)
                  (Macros.losesLife (Macros.each Opponent) (Lit 4)) ])) ]
       Nothing

||| Ballot Broker
public export
ballotBroker : Card
ballotBroker =
  Macros.card "Ballot Broker"
       (Just [Macros.generic 2, Macros.pip White]) []
       (MkTypeLine [creatureType "Human", creatureType "Advisor"] [Creature])
       [ Static (Macros.mayVoteAdditional You (Macros.exactly 1)) ]
       (Just (2, 3))

||| Council's Judgment
public export
councilsJudgment : Card
councilsJudgment =
  Macros.card "Council's Judgment"
       (Just [Macros.generic 1, Macros.pip White, Macros.pip White]) []
       (MkTypeLine [] [Sorcery])
       [ Macros.abilityWord WillOfTheCouncil
           (Spell (Sequentially
              [ Macros.vote (Macros.each AnyPlayer) Openly
                     (ByCandidate (Macros.a (And [ Permanent
                                                 , Not Macros.land
                                                 , Not (HasPossessor ControllerAx You) ])))
              , Macros.exile You (Macros.allOf (And [Permanent, WithMostVotes])) ])) ]
       Nothing

||| Orchard Elemental
public export
orchardElemental : Card
orchardElemental =
  Macros.card "Orchard Elemental"
       (Just [Macros.generic 5, Macros.pip Green]) []
       (MkTypeLine [creatureType "Elemental"] [Creature])
       [ Macros.abilityWord CouncilsDilemma
           (Macros.triggered When (Enters Macros.thisCreature Nothing)
              (Sequentially
                 [ Macros.vote (Macros.each AnyPlayer) Openly (ByLabel ["sprout", "harvest"])
                 , PutCounters (Times 2 (VotesFor "sprout"))
                               (PrintedKind Macros.plusOnePlusOne)
                               Macros.thisCreature
                 , Macros.gainsLife You (Times 3 (VotesFor "harvest")) ])) ]
       (Just (2, 2))

||| Plea for Power
public export
pleaForPower : Card
pleaForPower =
  Macros.card "Plea for Power"
       (Just [Macros.generic 3, Macros.pip Blue]) []
       (MkTypeLine [] [Sorcery])
       [ Macros.abilityWord WillOfTheCouncil
           (Spell (Sequentially
              [ Macros.vote (Macros.each AnyPlayer) Openly (ByLabel ["time", "knowledge"])
              , Macros.ifThen (VoteLead "time" False) (ExtraTurn You (Lit 1))
              , Macros.ifThen (VoteLead "knowledge" True)
                              (Draw You (Lit 3)) ])) ]
       Nothing

||| Coercive Portal
public export
coercivePortal : Card
coercivePortal =
  Macros.card "Coercive Portal" (Just [Macros.generic 4]) []
       (MkTypeLine [] [Artifact])
       [ Macros.abilityWord WillOfTheCouncil
           (Macros.triggered At (BeginningOf Upkeep (Macros.yours))
              (Sequentially
                 [ Macros.vote (Macros.each AnyPlayer) Openly (ByLabel ["carnage", "homage"])
                 , Macros.ifThen (VoteLead "carnage" False)
                     (Sequentially
                        [ Macros.sacrifice You Macros.thisArtifact
                        , Macros.destroy (Macros.allOf (And [Permanent,
                                                      Not Macros.land])) ])
                 , Macros.ifThen (VoteLead "homage" True)
                                 (Draw You (Lit 1)) ])) ]
       Nothing

||| Custodi Squire
public export
custodiSquire : Card
custodiSquire =
  Macros.card "Custodi Squire"
       (Just [Macros.generic 4, Macros.pip White]) []
       (MkTypeLine [creatureType "Spirit", creatureType "Cleric"] [Creature])
       [ Macros.keyword "Flying"
       , Macros.abilityWord WillOfTheCouncil
           (Macros.triggered When (Enters Macros.thisCreature Nothing)
              (Sequentially
                 [ Macros.vote (Macros.each AnyPlayer) Openly
                        (ByCandidate
                           (Macros.a (And [ Or [ HasType Artifact
                                               , HasType Creature
                                               , HasType Enchantment ]
                                          , InZone (Macros.graveyardOf You) ])))
                 , Macros.returnTo (Macros.allOf (And [IsCard, WithMostVotes]))
                                   Macros.handZ ])) ]
       (Just (3, 3))

||| Lieutenants of the Guard
public export
lieutenantsOfTheGuard : Card
lieutenantsOfTheGuard =
  Macros.card "Lieutenants of the Guard"
       (Just [Macros.generic 4, Macros.pip White]) []
       (MkTypeLine [creatureType "Human", creatureType "Soldier"] [Creature])
       [ Macros.abilityWord CouncilsDilemma
           (Macros.triggered When (Enters Macros.thisCreature Nothing)
              (Sequentially
                 [ Macros.vote (Macros.each AnyPlayer) Openly
                        (ByLabel ["strength", "numbers"])
                 , PutCounters (VotesFor "strength")
                               (PrintedKind Macros.plusOnePlusOne)
                               Macros.thisCreature
                 , Macros.create (VotesFor "numbers")
                                 (Macros.creatureTok 1 1 [White]
                                                     [creatureType "Soldier"]) ])) ]
       (Just (2, 2))

||| Truth or Consequences
public export
truthOrConsequencesVote : Ability
truthOrConsequencesVote =
  Macros.abilityWord SecretCouncil
    (Spell (Sequentially
       [ Macros.vote (Macros.each AnyPlayer) Secretly (ByLabel ["truth", "consequences"])
       , Draw You (VotesFor "truth") ]))

||| Emissary of Grudges
public export
emissaryOfGrudgesEntry : StaticEffect []
emissaryOfGrudgesEntry =
  Macros.entersChoosingPlayerSecretly Macros.thisCreature (Just OpponentsOnly)

||| Emissary of Grudges
public export
emissaryOfGrudgesReveal : AbilityAt [choiceB PlayerC]
emissaryOfGrudgesReveal =
  Macros.activatedOnlyOnce
    (Do (Expose Reveal You (ExposedChoice PlayerC)))
    (OnlyIf (ChooseNewTargets
               (Macros.target (Joined Macros.spell (AbilityHead AnyOnStack))))
            (Matches It (HasPossessor ControllerAx (Macros.the ChosenPlayer)))
            Nothing)
    OncePerGame



||| Fact or Fiction
public export
factOrFiction : Card
factOrFiction =
  Macros.card "Fact or Fiction" (Just [Macros.generic 3, Macros.pip Blue]) []
       (MkTypeLine [] [Instant])
       [ Spell (Sequentially
                  [ Macros.revealCards (Macros.topSlice (Lit 5))
                  , SeparateIntoPiles Macros.anOpponent Them 2 []
                  , Macros.move Macros.onePile Macros.handZ
                  , Macros.move TheOther Macros.graveyardZ ]) ]
       Nothing

||| Death or Glory
public export
deathOrGlory : Card
deathOrGlory =
  Macros.card "Death or Glory"
       (Just [Macros.generic 3, Macros.pip White, Macros.pip White]) []
       (MkTypeLine [] [Sorcery])
       [ Spell (Sequentially
                  [ SeparateIntoPiles You
                      (Macros.allOf (And [Macros.creature,
                                   InZone (Macros.graveyardOf You)])) 2 []
                  , Macros.exile You (Macros.pileOfChoice Macros.anOpponent)
                  , Macros.move TheOther Macros.battlefieldZ ]) ]
       Nothing

||| Steam Augury
public export
steamAugury : Card
steamAugury =
  Macros.card "Steam Augury"
       (Just [Macros.generic 2, Macros.pip Blue, Macros.pip Red]) []
       (MkTypeLine [] [Instant])
       [ Spell (Sequentially
                  [ Macros.revealCards (Macros.topSlice (Lit 5))
                  , SeparateIntoPiles You Them 2 []
                  , Macros.chooses Macros.anOpponent Macros.onePile
                  , Macros.move (That PileW) Macros.handZ
                  , Macros.move TheOther Macros.graveyardZ ]) ]
       Nothing

||| Sphinx of Uthuun
public export
sphinxOfUthuun : Card
sphinxOfUthuun =
  Macros.card "Sphinx of Uthuun"
       (Just [Macros.generic 5, Macros.pip Blue, Macros.pip Blue]) []
       (MkTypeLine [creatureType "Sphinx"] [Creature])
       [ Macros.keyword "Flying"
       , Macros.triggered When (Enters Macros.thisCreature Nothing)
           (Sequentially
              [ Macros.revealCards (Macros.topSlice (Lit 5))
              , SeparateIntoPiles Macros.anOpponent Them 2 []
              , Macros.move Macros.onePile Macros.handZ
              , Macros.move TheOther Macros.graveyardZ ]) ]
       (Just (5, 6))


||| Riddles in the Dark
public export
riddlesInTheDark : Card
riddlesInTheDark =
  Macros.card "Riddles in the Dark"
       (Just [Macros.generic 2, Macros.pip Blue]) []
       (MkTypeLine [] [Instant])
       [ Spell (Sequentially
                  [ Macros.lookAt (Macros.topSlice (Lit 4))
                  , SeparateIntoPiles You Them 2 [FaceDownPile, FaceUpPile]
                  , Macros.chooses Macros.anOpponent Macros.onePile
                  , Macros.move (That PileW) Macros.handZ
                  , Macros.move TheOther Macros.graveyardZ ]) ]
       Nothing

||| Fortune's Favor
public export
fortunesFavor : Card
fortunesFavor =
  Macros.card "Fortune's Favor"
       (Just [Macros.generic 3, Macros.pip Blue]) []
       (MkTypeLine [] [Instant])
       [ Spell (Sequentially
                  [ Expose LookAt (Macros.target Opponent)
                           (ExposedCards (Macros.topSlice (Lit 4)))
                  , SeparateIntoPiles They Them 2 [FaceDownPile, FaceUpPile]
                  , Macros.move Macros.onePile Macros.handZ
                  , Macros.move TheOther Macros.graveyardZ ]) ]
       Nothing

||| Curator of Destinies
public export
curatorOfDestinies : Card
curatorOfDestinies =
  Macros.card "Curator of Destinies"
       (Just [Macros.generic 4, Macros.pip Blue, Macros.pip Blue]) []
       (MkTypeLine [creatureType "Sphinx"] [Creature])
       [ Static (Macros.objectCant "Counter" This)
       , Macros.keyword "Flying"
       , Macros.triggered When (Enters Macros.thisCreature Nothing)
           (Sequentially
              [ Macros.lookAt (Macros.topSlice (Lit 5))
              , SeparateIntoPiles You Them 2 [FaceDownPile, FaceUpPile]
              , Macros.chooses Macros.anOpponent Macros.onePile
              , Macros.move (That PileW) Macros.handZ
              , Macros.move TheOther Macros.graveyardZ ]) ]
       (Just (5, 5))

||| Atris, Oracle of Half-Truths
public export
atrisOracleOfHalfTruths : Card
atrisOracleOfHalfTruths =
  Macros.cardOf "Atris, Oracle of Half-Truths"
       (Just [Macros.generic 2, Macros.pip Blue, Macros.pip Black]) [Legendary]
       (MkTypeLine [creatureType "Human", creatureType "Advisor"] [Creature])
       [ Macros.keyword "Menace"
       , Macros.triggered When (Enters Macros.thisCreature Nothing)
           (Sequentially
              [ Expose LookAt (Macros.target Opponent)
                       (ExposedCards (Macros.topSlice (Lit 3)))
              , SeparateIntoPiles They Them 2 [FaceDownPile, FaceUpPile]
              , Macros.move Macros.onePile Macros.handZ
              , Macros.move TheOther Macros.graveyardZ ]) ]
       (Macros.printedBox (Just (3, 2)))

||| Do or Die
public export
doOrDie : Card
doOrDie =
  Macros.card "Do or Die" (Just [Macros.generic 1, Macros.pip Black]) []
       (MkTypeLine [] [Sorcery])
       [ Spell (Sequentially
                  [ SeparateIntoPiles You
                      (Macros.allOf (And [Macros.creature,
                                   HasPossessor ControllerAx (Macros.target AnyPlayer)])) 2 []
                  , CantBe (Macros.destroy
                              (Macros.allOf (And [Macros.creature,
                                           InPile (Macros.pileOfChoice They)])))
                           "Regenerate" (ThemVerbed "Destroy") ]) ]
       Nothing

||| Liliana of the Veil
public export
lilianaOfTheVeil : Card
lilianaOfTheVeil =
  Macros.cardOf "Liliana of the Veil"
       (Just [Macros.generic 1, Macros.pip Black, Macros.pip Black])
       [Legendary] (MkTypeLine [planeswalkerType "Liliana"] [Planeswalker])
       [ Macros.activated (LoyaltySymbol (LoyaltyUp 1))
                          ((Macros.discard (Macros.each AnyPlayer) (Macros.a (InZone Macros.handZ))))
       , Macros.activated (LoyaltySymbol (LoyaltyDown 2))
                          (Macros.sacrifice (Macros.target AnyPlayer)
                                            (Macros.aTheirChoice Macros.creature))
       , Macros.activated (LoyaltySymbol (LoyaltyDown 6))
           (Sequentially
              [ SeparateIntoPiles You
                  (Macros.allOf (And [Permanent,
                               HasPossessor ControllerAx (Macros.target AnyPlayer)])) 2 []
              , Macros.sacrifice They
                  (Macros.allOf (And [Permanent,
                               InPile (Macros.pileOfChoice They)])) ]) ]
       (Macros.loyaltyBox 3)



||| Charnel Troll
public export
charnelTroll : Card
charnelTroll =
  Macros.card "Charnel Troll"
       (Just [Macros.generic 1, Macros.pip Black, Macros.pip Green]) []
       (MkTypeLine [creatureType "Troll"] [Creature])
       [ Macros.keyword "Trample"
       , Macros.triggered At (BeginningOf Upkeep (Macros.yours))
           ((IfDone (Macros.exile You (Macros.a (And [Macros.creature,
                                            InZone (Macros.graveyardOf You)]))) (Just (PutCounters (Lit 1) (PrintedKind Macros.plusOnePlusOne)
                           Macros.thisCreature)) (Just (Macros.sacrifice You Macros.thisCreature))))
       , Macros.activated
           (Compound [ Mana [Macros.pip Black, Macros.pip Green]
                     , Do (Macros.discard You
                             (Macros.a (And [Macros.creature,
                                             InZone Macros.handZ]))) ])
           (PutCounters (Lit 1) (PrintedKind Macros.plusOnePlusOne)
                        Macros.thisCreature) ]
       (Just (4, 4))

||| Promise of Bunrei
public export
promiseOfBunrei : Card
promiseOfBunrei =
  Macros.card "Promise of Bunrei"
       (Just [Macros.generic 2, Macros.pip White]) []
       (MkTypeLine [] [Enchantment])
       [ Macros.triggered When (Dies (Macros.a Macros.creatureYouControl))
           ((IfDone (Macros.sacrifice You Macros.thisEnchantment) (Just (Macros.create (Lit 4)
                 (Macros.creatureTok 1 1 [] [creatureType "Spirit"]))) Nothing)) ]
       Nothing

||| Grave Peril
public export
gravePeril : Card
gravePeril =
  Macros.card "Grave Peril" (Just [Macros.generic 1, Macros.pip Black]) []
       (MkTypeLine [] [Enchantment])
       [ Macros.triggered When
           (Enters (Macros.a (And [Macros.creature, Not (ColorIs Black)])) Nothing)
           ((IfDone (Macros.sacrifice You Macros.thisEnchantment) (Just (Macros.destroy (That (TypeW Creature)))) Nothing)) ]
       Nothing

||| Mistbreath Elder
public export
mistbreathElder : Card
mistbreathElder =
  Macros.card "Mistbreath Elder" (Just [Macros.pip Green]) []
       (MkTypeLine [creatureType "Frog", creatureType "Warrior"] [Creature])
       [ Macros.triggered At (BeginningOf Upkeep (Macros.yours))
           ((IfDone (Macros.returnTo
                 (Macros.a (Macros.otherCreatureYouControl Macros.thisCreature))
                 Macros.handZ) (Just (PutCounters (Lit 1) (PrintedKind Macros.plusOnePlusOne)
                           Macros.thisCreature)) (Just (Macros.may You (Macros.returnTo Macros.thisCreature Macros.handZ))))) ]
       (Just (2, 2))

||| Woeleecher
public export
woeleecherWhole : Card
woeleecherWhole =
  Macros.card "Woeleecher" (Just [Macros.generic 5, Macros.pip White]) []
       (MkTypeLine [creatureType "Elemental"] [Creature])
       [ Cards.woeleecher ]
       (Just (3, 5))

||| Garruk Relentless // Garruk, the Veil-Cursed
public export
garrukRelentless : Card
garrukRelentless =
  Transforming
    (Macros.frontFace "Garruk Relentless" (Just [Macros.generic 3, Macros.pip Green])
            [Legendary] (MkTypeLine [planeswalkerType "Garruk"] [Planeswalker])
            [ Cards.garrukRelentlessFlip
            , Macros.activated (LoyaltySymbol LoyaltyZero)
                (Sequentially
                   [ DealDamage Macros.thisPlaneswalker (Lit 3)
                                (Macros.target Macros.creature)
                   , DealDamage (That (TypeW Creature)) (StatOf Power (That (TypeW Creature)))
                                Macros.thisPlaneswalker ])
            , Macros.activated (LoyaltySymbol LoyaltyZero)
                (Macros.create (Lit 1)
                   (Macros.creatureTok 2 2 [Green] [creatureType "Wolf"])) ]
            (Macros.loyaltyBox 3))
    (Macros.backFace "Garruk, the Veil-Cursed" [Legendary]
               (MkTypeLine [planeswalkerType "Garruk"] [Planeswalker])
               [ Macros.activated (LoyaltySymbol (LoyaltyUp 1))
                   (Macros.create (Lit 1)
                      (MkToken (Just (Lit 1 ** Lit 1)) [Black]
                               (MkTypeLine [creatureType "Wolf"] [Creature])
                               [Macros.keyword "Deathtouch"] Nothing))
               , Macros.activated (LoyaltySymbol (LoyaltyDown 1))
                   ((IfDone (Macros.sacrifice You (Macros.a Macros.creature)) (Just (Sequentially
                         [ Macros.searchLibraryFor Macros.creature
                         , Macros.revealsIt
                         , Macros.move Macros.foundCard Macros.handZ
                         , Macros.shuffle ])) Nothing))
               , Macros.activated (LoyaltySymbol (LoyaltyDown 3))
                   (Sequentially
                      [ Continuously {ts = StaticFirstDone}
                          (AndAlso
                             [ Gains (Macros.allOf Macros.creatureYouControl)
                                     (Macros.keyword "Trample")
                             , Gets Them (PtUp (LetterVal X)) (PtUp (LetterVal X)) ])
                          (Just Macros.untilEndOfTurn)
                      , Define X (Macros.countOf (And [Macros.creature,
                                                InZone (Macros.graveyardOf You)])) ]) ]
               Nothing)


||| Marit Lage
public export
maritLage : TokenChars bs
maritLage =
  MkSupertypedToken (Just (Lit 20 ** Lit 20)) [Black] [Legendary]
    (MkTypeLine [creatureType "Avatar"] [Creature])
    [Macros.keyword "Flying", Macros.keyword "Indestructible"]
    (Just "Marit Lage")

||| Dark Depths
public export
darkDepths : Card
darkDepths =
  Macros.card "Dark Depths" Nothing [Legendary, Snow]
       (MkTypeLine [] [Land])
       [ Static (Macros.entersWithCounters Macros.thisLand (Lit 10) (Named "Ice"))
       , Macros.activated (Mana [Macros.generic 3])
           (Macros.removeCounters (Macros.exactly 1) (Just (PrintedKind (Named "Ice"))) Macros.thisLand)
       , Macros.triggered When
           (Macros.whenState
              (Macros.notSo (Matches Macros.thisLand (HasCounters (Just (Named "Ice"))))))
           ((IfDone (Macros.sacrifice You Macros.thisLand) (Just (Macros.create (Lit 1) Cards.maritLage)) Nothing)) ]
       Nothing

||| Tuktuk the Explorer
public export
tuktukTheExplorer : Card
tuktukTheExplorer =
  Macros.card "Tuktuk the Explorer"
       (Just [Macros.generic 2, Macros.pip Red]) [Legendary]
       (MkTypeLine [creatureType "Goblin"] [Creature])
       [ Macros.keyword "Haste"
       , Macros.triggered When (Dies Macros.thisCreature)
           (Macros.create (Lit 1)
              (MkSupertypedToken (Just (Lit 5 ** Lit 5)) [] [Legendary]
                 (MkTypeLine [creatureType "Goblin", creatureType "Golem"]
                             [Artifact, Creature])
                 [] (Just "Tuktuk the Returned"))) ]
       (Just (1, 1))


||| Blood Reckoning
public export
bloodReckoning : Card
bloodReckoning =
  Macros.card "Blood Reckoning"
       (Just [Macros.generic 3, Macros.pip Black]) []
       (MkTypeLine [] [Enchantment])
       [ Macros.triggered Whenever
           (Macros.attacksPlayer (Macros.a Macros.creature)
              (Macros.youOr
                 (Macros.a (And [HasType Planeswalker, HasPossessor ControllerAx You]))))
           (Macros.losesLife (Macros.controllerOf (That (TypeW Creature))) (Lit 1)) ]
       Nothing

||| Land Tax
public export
landTax : Card
landTax =
  Macros.card "Land Tax" (Just [Macros.pip White]) []
       (MkTypeLine [] [Enchantment])
       [ Macros.triggeredIf At (BeginningOf Upkeep (Macros.yours))
           (Macros.exists Cards.opponentWithMoreLands)
           (Macros.may You
              (Sequentially
                 [ Macros.searchLibraryForCount (Macros.upTo 3)
                     (And [Macros.land, HasSupertype Basic])
                 , Macros.revealCards (Those CardW)
                 , Macros.move (Those CardW) Macros.handZ
                 , Macros.shuffle ])) ]
       Nothing


||| Artificer's Assistant
public export
artificersAssistant : Card
artificersAssistant =
  Macros.card "Artificer's Assistant" (Just [Macros.pip Blue]) []
       (MkTypeLine [creatureType "Bird"] [Creature])
       [ Macros.keyword "Flying"
       , Macros.triggered Whenever
           (Casts You (Macros.a (And [IsHistoric, Macros.spell])) Nothing)
           (Macros.scry You (Lit 1)) ]
       (Just (1, 1))

||| Aya of Alexandria
public export
ayaOfAlexandria : Card
ayaOfAlexandria =
  Macros.card "Aya of Alexandria"
       (Just [Macros.generic 2, Macros.pip Red, Macros.pip White]) [Legendary]
       (MkTypeLine [creatureType "Human", creatureType "Assassin"] [Creature])
       [ Macros.keyword "Menace"
       , Macros.keyword "Lifelink"
       , Macros.triggered Whenever
           (Macros.dealsCombatDamage
              (Macros.a (And [IsHistoric, Macros.creature, HasPossessor ControllerAx You]))
              (Macros.a AnyPlayer))
           (Macros.create (Lit 1)
              (MkToken (Just (Lit 1 ** Lit 1)) [Black]
                       (MkTypeLine [creatureType "Assassin"] [Creature])
                       [Macros.keyword "Menace"] Nothing)) ]
       (Just (4, 3))


||| Indestructibility
public export
indestructibility : Card
indestructibility =
  Macros.card "Indestructibility" (Just [Macros.generic 3]) []
       (MkTypeLine [enchantmentType "Aura"] [Enchantment])
       [ Macros.keywordSubject "Enchant" Permanent
       , Static (Gains (AttachHost Enchanted PermanentW)
                       (Macros.keyword "Indestructible")) ]
       Nothing


||| Beckoning Will-o'-Wisp
public export
beckoningWillOWisp : Card
beckoningWillOWisp =
  Macros.card "Beckoning Will-o'-Wisp"
       (Just [Macros.generic 2, Macros.pip White]) []
       (MkTypeLine [creatureType "Spirit"] [Creature])
       [ Macros.keyword "Flying"
       , Macros.flavorWord "Lure the Unwary"
           (Macros.triggered At (BeginningOf Combat (Macros.yours))
              (Macros.choose (Macros.a Opponent)))
       , Static (Gets (Macros.allOf (And [Macros.creature,
                                   CombatRel AttackerOf (Macros.the ChosenPlayer)]))
                      (PtUp (Lit 1)) (PtUp (Lit 0))) ]
       (Just (1, 3))

||| Triarch Stalker
public export
triarchStalker : Card
triarchStalker =
  Macros.card "Triarch Stalker"
       (Just [Macros.generic 3, Macros.pip Black, Macros.pip Black]) []
       (MkTypeLine [creatureType "Necron"] [Artifact, Creature])
       [ Macros.flavorWord "Targeting Relay"
           (Macros.triggered At (BeginningOf Combat (Macros.yours))
              (Macros.choose (Macros.a Opponent)))
       , Static (Gains (Macros.allOf (And [Macros.creature,
                                    CombatRel AttackerOf (Macros.the ChosenPlayer)]))
                       (Macros.keyword "Menace")) ]
       (Just (4, 5))


||| Seraphic Greatsword
public export
seraphicGreatsword : Card
seraphicGreatsword =
  Macros.card "Seraphic Greatsword"
       (Just [Macros.generic 1, Macros.pip White]) []
       (MkTypeLine [artifactType "Equipment"] [Artifact])
       [ Static (Gets (AttachHost Equipped (TypeW Creature))
                      (PtUp (Lit 2)) (PtUp (Lit 2)))
       , Macros.triggered Whenever
           (Macros.attacksPlayer (AttachHost Equipped (TypeW Creature))
              (Macros.the (And [AnyPlayer,
                              Superlative MaxOf (PlayerStatAxis LifeTotal)
                                          AnyPlayer])))
           (Create You (Lit 1)
              (TokenWritten (MkToken (Just (Lit 4 ** Lit 4)) [White]
                               (MkTypeLine [creatureType "Angel"] [Creature])
                               [Macros.keyword "Flying"] Nothing))
              [EntersTapped, EntersAttacking (OneDefender (That PlayerW))])
       , Macros.keywordCosting "Equip" (Mana [Macros.generic 4]) ]
       Nothing

||| Sphinx of Clear Skies

||| Unesh, Criosphinx Sovereign
public export
uneshCriosphinxSovereign : Card
uneshCriosphinxSovereign =
  Macros.card "Unesh, Criosphinx Sovereign"
       (Just [Macros.generic 4, Macros.pip Blue, Macros.pip Blue]) [Legendary]
       (MkTypeLine [creatureType "Sphinx"] [Creature])
       [ Macros.keyword "Flying"
       , Static (CostsToCast
                   (Macros.allOf (And [HasSubtype (creatureType "Sphinx"),
                                Macros.spell, CastBy You]))
                   (CostLess (Lit 2) Nothing))
       , Macros.triggered Whenever
           (Enters (EitherOf Macros.thisCreature
                      (Macros.a (And [HasSubtype (creatureType "Sphinx"),
                                      HasPossessor ControllerAx You,
                                      OtherThan Macros.thisCreature])))
                   Nothing)
           (Sequentially
              [ Macros.revealCards (Macros.topSlice (Lit 4))
              , SeparateIntoPiles Macros.anOpponent Them 2 []
              , Macros.move Macros.onePile Macros.handZ
              , Macros.move TheOther Macros.graveyardZ ]) ]
       (Just (4, 4))

||| Jeskai Ascendancy's first trigger
public export
jeskaiAscendancyPump : Ability
jeskaiAscendancyPump =
  Macros.triggered Whenever
    (Casts You (Macros.a (And [Macros.spell, Not Macros.creature])) Nothing)
    (Sequentially
       [ Macros.gets (Macros.bare Macros.creatureYouControl)
                     (PtUp (Lit 1)) (PtUp (Lit 1)) (Just Macros.untilEndOfTurn)
       , Macros.untap (Those (TypeW Creature)) ])

||| Thallid
public export
thallid : Card
thallid =
  Macros.card "Thallid" (Just [Macros.pip Green]) []
       (MkTypeLine [creatureType "Fungus"] [Creature])
       [ Macros.triggered At (BeginningOf Upkeep (Macros.yours))
           (PutCounters (Lit 1) (PrintedKind (Named "Spore")) Macros.thisCreature)
       , Macros.activated
           (Do (RemoveCounters (Just (Macros.exactly 3))
                  (Just (PrintedKind (Named "Spore"))) Macros.thisCreature))
           (Macros.create (Lit 1)
              (Macros.creatureTok 1 1 [Green] [creatureType "Saproling"])) ]
       (Just (1, 1))

||| Aether Chaser
public export
aetherChaserEnergy : Ability
aetherChaserEnergy =
  Macros.triggered When (Enters Macros.thisCreature Nothing)
                   (PutCounters (Lit 2) (PrintedKind (Named "Energy")) You)

||| Blastoderm
public export
blastodermEntersFading : StaticEffect []
blastodermEntersFading =
  Macros.entersWithCounters Macros.thisCreature (Lit 3) (Named "Fade")

||| Blastoderm
public export
blastodermFadeUpkeep : Ability
blastodermFadeUpkeep =
  Macros.triggered At (BeginningOf Upkeep (Macros.yours))
    (RemoveCounters (Just (Macros.exactly 1))
       (Just (PrintedKind (Named "Fade"))) Macros.thisCreature)

||| Archfiend of the Dross
public export
archfiendOfTheDrossEntersOiled : StaticEffect []
archfiendOfTheDrossEntersOiled =
  Macros.entersWithCounters Macros.thisCreature (Lit 4) (Named "Oil")

||| Archfiend of the Dross
public export
archfiendOfTheDrossOilUpkeep : Ability
archfiendOfTheDrossOilUpkeep =
  Macros.triggered At (BeginningOf Upkeep (Macros.yours))
    (RemoveCounters (Just (Macros.exactly 1))
       (Just (PrintedKind (Named "Oil"))) Macros.thisCreature)
