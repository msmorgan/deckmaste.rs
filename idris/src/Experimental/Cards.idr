||| The workbench's evidence bench: typechecking positives and pinned negatives over real cards.
|||
||| Selection principle: an entry is added when a construction, keyword, or
||| interaction needs a witness for some round's work. The bench is not a
||| random sample of the corpus, not a curated-for-coverage set, and not a
||| representative one; it grows by proof, card by card, as rounds need
||| evidence.
|||
||| A card's absence from this file is therefore not a claim about that card.
||| It says only that no round has yet needed it as a witness. Absence is a
||| normal, un-alarming state, and the fraction of the corpus present here is
||| not a quantity this workbench tracks: the bench exists to show that the
||| rules text is self-consistent and that every card *can* be represented,
||| not to census what has been printed.
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

||| Bond of Revival -- "Return target creature card from your graveyard to
||| the battlefield. It gains haste until your next turn." The move is
||| LABELED, so the pronoun the second sentence writes is read at the
||| clause that produced its referent rather than across every singular
||| object mention.
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
kindredDominance = Sequentially [Macros.choose (Macros.a (Macros.quality (SubtypeQ Creature))),
                                 Macros.destroy (AllOf (And [Macros.creature, Not (OfChosen (SubtypeQ Creature))]))]

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
  Sequentially [ Macros.tap (CountedGroup Macros.anyNumber Nothing
                              (And [Macros.untapped, Macros.creature, ControlledBy You]))
               , ForEachOf (Macros.thoseVerbedThisWay "Tap" (TypeW Creature))
                           (Macros.gainsLife You (Lit 4)) ]

ratsOfRath : Effect []
ratsOfRath = Macros.destroy (Macros.target (And [Or [Macros.artifact, Macros.creature, Macros.land], ControlledBy You]))

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
blindingFlare = Macros.cantBlock (TargetGroup Macros.anyNumber Macros.creature) (Just Macros.thisTurn)


defeat : Effect []
defeat = Macros.destroy (Macros.target (And [Macros.creature, Compare [CharAxis Power] AtMost (Lit 2)]))

terashisVerdict : Effect []
terashisVerdict =
  Macros.destroy (Macros.target (And [Macros.creature, Attacking, Compare [CharAxis Power] AtMost (Lit 3)]))

pillarOfLight : Effect []
pillarOfLight =
  Macros.exile (Macros.target (And [Macros.creature, Compare [CharAxis Toughness] AtLeast (Lit 4)]))

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
  Sequentially [DealDamage This (Lit 4) (Macros.target (And [Macros.creature, ControlledBy Macros.anOpponent])),
                OnlyIf (DealDamage This (Lit 2)
                                   (Each (And [Macros.creature, Other, ControlledBy (That PlayerW)])))
                       (Exists (And [Macros.creature, ControlledBy You,
                                     Compare [CharAxis Power] AtLeast (Lit 4)]))
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
                  (PutCounters (Lit 1) (PrintedKind Macros.plusOnePlusOne) Macros.thisCreature)
                  (RemoveCounters (Macros.exactly 1) (Just Macros.plusOnePlusOne) Macros.thisCreature)

yawgmothDemon : Effect []
yawgmothDemon =
  Macros.mayElse You (Macros.sacrifice You (Macros.a Macros.artifact))
              (Sequentially [SetStatus Tapped Macros.thisCreature, DealDamage This (Lit 2) You])


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
  Sequentially [If (Macros.notSo (Exists (And [HasSubtype (creatureType "Army"), Macros.creature, ControlledBy You])))
                   (Macros.create (Lit 1) (Macros.creatureTok 0 0 [Black] [creatureType "Zombie", creatureType "Army"]))
                   Nothing,
                Macros.choose (Macros.a (And [HasSubtype (creatureType "Army"), Macros.creature, ControlledBy You])),
                PutCounters (Lit 2) (PrintedKind Macros.plusOnePlusOne) (That (TypeW Creature)),
                If (Macros.itIsntA (HasSubtype (creatureType "Zombie")))
                   (Macros.becomes It (Macros.subtypesOnly [creatureType "Zombie"]) Nothing)
                   Nothing]

battlegrowth : Effect []
battlegrowth = PutCounters (Lit 1) (PrintedKind Macros.plusOnePlusOne) (Macros.target Macros.creature)

chainbreaker : Effect []
chainbreaker = RemoveCounters (Macros.exactly 1) (Just Macros.minusOneMinusOne) (Macros.target Macros.creature)

kaitoBaneOfNightmares : Effect []
kaitoBaneOfNightmares = Sequentially [SetStatus Tapped (Macros.target Macros.creature),
                                      PutCounters (Lit 2) (PrintedKind Stun) It]

jhoiraOfTheGhitu : Ability
jhoiraOfTheGhitu =
  Macros.activated (Compound [Mana [Macros.generic 2],
                       Do (Macros.exile (Macros.a (And [Not Macros.land, InZone (Macros.handOf You)])))])
                   (PutCounters (Lit 4) (PrintedKind Time) (Macros.theVerbed "Exile" CardW))

alaundoTheSeer : Effect []
alaundoTheSeer = RemoveCounters (Macros.exactly 1) (Just Time) (Each (InZone Macros.exileZ))

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
                                      (CostLess (Lit 1) Nothing))
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
             Macros.destroy (AllOf (And [Macros.creature, Compare [CharAxis ManaValue] AtMost (Lit 3)])),
             Macros.destroy (AllOf (And [Macros.creature, Compare [CharAxis ManaValue] AtLeast (Lit 4)]))]

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
  PutCounters (Lit 1) (PrintedKind Macros.plusOnePlusOne) (Each (Macros.otherCreatureYouControl Macros.thisCreature))

carnifexDemon : Effect []
carnifexDemon =
  PutCounters (Lit 1) (PrintedKind Macros.minusOneMinusOne) (Each (Macros.otherCreature Macros.thisCreature))

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
  PutCounters (Lit 1) (PrintedKind Macros.plusOnePlusOne) (EachOf (TargetGroup (Macros.upTo 2) Macros.creature))

fallOfTheTitans : Effect []
fallOfTheTitans = DealDamage This (LetterVal X) (EachOf (TargetGroup (Macros.upTo 2) Macros.anyTarget))

naturesPanoply : Effect []
naturesPanoply =
  Sequentially [Macros.choose (TargetGroup Macros.anyNumber Macros.creature),
                PutCounters (Lit 1) (PrintedKind Macros.plusOnePlusOne) (EachOf Them)]

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
                            (Macros.leavesBattlefield Macros.thisCreature)

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
                                Macros.drawACard
                                (Exists (And [Macros.creature, ControlledBy You,
                                              Compare [CharAxis Power] AtLeast (Lit 4)])) ]
       Nothing

woeleecher : Ability
woeleecher =
  Macros.activated (Compound [Mana [Macros.pip White], TapSymbol])
                   (Macros.doThen (RemoveCounters (Macros.exactly 1) (Just Macros.minusOneMinusOne) (Macros.target Macros.creature))
                    (Macros.gainsLife You (Lit 2)))

moltingHarpy : Effect []
moltingHarpy = Macros.mayElse You (Pay You (Mana [Macros.generic 2]) PaidOnce) (Macros.sacrifice You Macros.thisCreature)

carnophage : Effect []
carnophage = Macros.mayElse You (Pay You (Macros.payLife You 1) PaidOnce) (SetStatus Tapped Macros.thisCreature)

solitaryConfinement : Effect []
solitaryConfinement = Macros.mayElse You (Macros.discardsACard You) (Macros.sacrifice You Macros.thisEnchantment)


securityDetail : Ability
securityDetail =
  Macros.activatedOnlyOnceIf (Mana [Macros.pip White, Macros.pip White])
                             (Macros.create (Lit 1) (Macros.creatureTok 1 1 [White] [creatureType "Soldier"]))
                             OncePerTurn
                             (Macros.notSo (Exists (And [Macros.creature, ControlledBy You])))




cloudkinSeer : Ability
cloudkinSeer = Macros.triggered When (Enters Macros.thisCreature Nothing) Macros.drawACard

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
                     (Enters Macros.thisCreature Nothing)
                     (Exists (And [Macros.artifact, ControlledBy You]))
                     Macros.drawACard

||| Glacial Chasm
glacialChasmCant : Ability
glacialChasmCant = Static (Macros.deontic (AllOf Macros.creatureYouControl) Forbid ["Attack"] Agent NoDeonticPatient)

||| Glacial Chasm
glacialChasmShield : Ability
glacialChasmShield =
  Static (Prevents AnyDamage AllOfIt (Macros.shieldingIt You) Nothing Nothing)

miserysShadow : Ability
miserysShadow =
  Static (Intercepts (Dies (Macros.a (And [Macros.creature, ControlledBy (Macros.a Opponent)]))) [] Nothing
                     (Macros.exile It) Repeatedly Nothing)

thoughtReflection : Ability
thoughtReflection =
  Static (Intercepts (Draws You) [] Nothing (Draw You (Lit 2)) Repeatedly Nothing)

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
  Macros.triggered When (Enters Macros.thisCreature Nothing)
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
  Static (Macros.unlessSo (Exists (And [Macros.artifact, ControlledBy You]))
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
       , Macros.activated (Do (RemoveCounters (Macros.exactly 1) (Just Macros.plusOnePlusOne) Macros.thisCreature))
                          (AddMana You (Lit 1) (Runs [[Colorless]]) []) ]
       (Just (0, 0))


counterspell : Effect []
counterspell = Macros.counterSpell (Macros.target Macros.spell)

beastWhisperer : Ability
beastWhisperer =
  Macros.triggered Whenever (Casts You (Macros.a (And [Macros.creature, Macros.spell])) Nothing) Macros.drawACard

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
              (Each (And [Macros.creature, ControlledBy You, Not (HasKeyword (TheKeyword "Flying"))]))


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
                   (PutCounters (Lit 1) (PrintedKind Macros.plusOnePlusOne) Macros.thisCreature)

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
                   (Macros.destroy (AllOf (And [Macros.creature,
                                         Or [BlockerOf Macros.thisCreature,
                                             BlockedBy Macros.thisCreature]])))

vertigoSpawn : Ability
vertigoSpawn =
  Macros.triggered Whenever (Blocks Macros.thisCreature (Just (Macros.a Macros.creature)))
                   (Sequentially [SetStatus Tapped (That (TypeW Creature)),
                           DoesntUntapNext (That (TypeW Creature)) (Lit 1)])


munghaWurm : Ability
munghaWurm = Static (CantMoreThan You "Untap" 1 Macros.land)

dampingField : Ability
dampingField = Static (CantMoreThan (PlayerGroup AllPlayers) "Untap" 1 Macros.artifact)

smoke : Ability
smoke = Static (CantMoreThan (PlayerGroup AllPlayers) "Untap" 1 Macros.creature)

winterOrb : Ability
winterOrb =
  Static (Macros.asLongAs (Matches Macros.thisArtifact Macros.untapped)
                          (CantMoreThan (PlayerGroup AllPlayers) "Untap" 1 Macros.land))

staticOrb : Ability
staticOrb =
  Static (Macros.asLongAs (Matches Macros.thisArtifact Macros.untapped)
                          (CantMoreThan (PlayerGroup AllPlayers) "Untap" 2 Permanent))

||| Rule of Law, whole -- "Each player can't cast more than one spell
||| each turn." The count cap at a SECOND deed, which is what generalised
||| the untap cap into one row: 12 supported lines cap casting, 9 cap
||| untapping and 3 cap drawing, and the three differ only in the label.
||| The period is not written here: [CR#500.1] gives the turn to a deed
||| with no step of its own, and the printed "each turn" spells that.
public export
ruleOfLaw : Card
ruleOfLaw =
  Macros.card "Rule of Law" (Just [Macros.generic 2, Macros.pip White]) []
       (MkTypeLine [] [Enchantment])
       [ Static (CantMoreThan (PlayerGroup AllPlayers) "Cast" 1 Macros.spell) ]
       Nothing

||| Spirit of the Labyrinth, whole -- "Each player can't draw more than
||| one card each turn." The count cap's third deed, and the reason
||| `deedFacts "DrawCard"` gained a patient: [CR#121.1] has the drawn
||| card come off the top of a library, so the thing counted has a role
||| and a zone even though no printed line writes the object voice.
||| `IsCard` is the bare word and seeds no zone [CR#109.2].
public export
spiritOfTheLabyrinth : Card
spiritOfTheLabyrinth =
  Macros.card "Spirit of the Labyrinth"
       (Just [Macros.generic 1, Macros.pip White]) []
       (MkTypeLine [creatureType "Spirit"] [Enchantment, Creature])
       [ Static (CantMoreThan (PlayerGroup AllPlayers) "DrawCard" 1 IsCard) ]
       (Just (3, 1))


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
                   (PutCounters (CountersOn Experience You) (PrintedKind Macros.plusOnePlusOne)
                         (Macros.target Macros.creature))


orneryDilophosaur : Ability
orneryDilophosaur =
  Macros.triggeredIf Whenever
                     (Macros.attacks Macros.thisCreature)
                     (Exists (And [Macros.creature, ControlledBy You,
                                   Compare [CharAxis Power] AtLeast (Lit 4)]))
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
                     (Enters Macros.thisCreature Nothing)
                     (Macros.happened AttackDeclaration You Lookback.ThisTurn)
                     Macros.drawACard

vashtaNerada : Ability
vashtaNerada =
  Macros.triggeredIf At
                     (BeginningOf EndStep (ByWord EachPlayers))
                     (Macros.happened Death (Macros.a Macros.creature)
                                      Lookback.ThisTurn)
                     (PutCounters (Lit 1) (PrintedKind Macros.plusOnePlusOne) Macros.thisCreature)

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
                     (Enters Macros.thisCreature Nothing)
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
                         (PrintedKind Macros.plusOnePlusOne) Macros.thisCreature)


winterMoon : Ability
winterMoon =
  Static (CantMoreThan (PlayerGroup AllPlayers) "Untap" 1
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

||| Pure // Simple
simpleHalf : Effect []
simpleHalf = Macros.destroy (Macros.target (And [Permanent, Multicolored]))

leitmotifComposer : Ability
leitmotifComposer =
  Macros.activated (Mana [Macros.generic 2, Macros.pip Blue])
                   (Continuously (Macros.deontic (AllOf (And [Macros.creature,
                                                Named (PrintedName "Leitmotif Composer")]))
                                   Forbid ["Block"] Patient NoDeonticPatient)
                          (Just Macros.thisTurn))


paladinOfAtonement : Ability
paladinOfAtonement =
  Macros.triggeredIf At
                     (BeginningOf Upkeep (ByWord EachPlayers))
                     (Macros.happened LifeLoss You Lookback.LastTurn)
                     (PutCounters (Lit 1) (PrintedKind Macros.plusOnePlusOne) Macros.thisCreature)

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
berserkersOfBloodRidge = Static (Macros.deontic Macros.thisCreature Require ["Attack"] Agent NoDeonticPatient)

trumpetingArmodon : Ability
trumpetingArmodon =
  Macros.activated (Mana [Macros.generic 1, Macros.pip Green])
                   (Continuously (Macros.deontic (Macros.target Macros.creature) Require ["Block"] Agent
                                   (DeonticCounterpart Macros.thisCreature))
                          (Just Macros.thisTurn))

loathsomeCatoblepas : Ability
loathsomeCatoblepas =
  Macros.activated (Mana [Macros.generic 2, Macros.pip Green])
                   (Continuously (Macros.deontic Macros.thisCreature Require ["Block"] Patient NoDeonticPatient)
                          (Just Macros.thisTurn))

ashnodsBattleGear : Ability
ashnodsBattleGear = Static (MayDeclineUntap Macros.thisArtifact)


throneWarden : Ability
throneWarden =
  Macros.triggeredIf At
                     (BeginningOf EndStep (ByWord Yours))
                     (Matches You (HasDesignation Monarch))
                     (PutCounters (Lit 1) (PrintedKind Macros.plusOnePlusOne) Macros.thisCreature)

aragornKingOfGondor : Ability
aragornKingOfGondor =
  Macros.triggered When (Enters Macros.thisCreature Nothing) (Macros.gainsDesignation You Monarch Instructed)

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
  Static (Macros.deontic Macros.thisCreature (GatedBy (Mana [Macros.generic 1]))
                  ["Block"] Agent
                  (DeonticCounterpart (AllOf (And [Macros.creature,
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

||| Bristlepack Sentry, whole card -- "Defender / As long as you control
||| a creature with power 4 or greater, this creature can attack as
||| though it didn't have defender." The permission's largest cell: 52
||| supported lines write "can attack as though it didn't have defender"
||| (measured 2026-08-28), 45 of them this bare form under a condition or
||| an activated cost. `Permit` is what makes the line writable and the
||| [CR#609.4] premise rides it; before the row existed the sentence was
||| unwritable before any counterfactual was reached.
public export
bristlepackSentry : Card
bristlepackSentry =
  Macros.card "Bristlepack Sentry" (Just [Macros.generic 1, Macros.pip Green]) []
       (MkTypeLine [creatureType "Plant", creatureType "Wolf"] [Creature])
       [ Macros.keyword "Defender"
       , Static (Macros.asLongAs
                   (Exists (And [Macros.creature, ControlledBy You,
                                 Compare [CharAxis Power] AtLeast (Lit 4)]))
                   (Macros.canDoAsThough Macros.thisCreature "Attack"
                                         (Not (HasKeyword (TheKeyword "Defender"))))) ]
       (Just (3, 3))

||| Pacifism, whole card -- "Enchant creature / Enchanted creature can't
||| attack or block." ONE subject, ONE modality, TWO deeds: the carrier's
||| deed list, which is the coordination designed once rather than per
||| family. 109 supported lines write "can't attack or block" (measured
||| 2026-08-28).
public export
pacifism : Card
pacifism =
  Macros.card "Pacifism" (Just [Macros.generic 1, Macros.pip White]) []
       (MkTypeLine [enchantmentType "Aura"] [Enchantment])
       [ Macros.keywordSubject "Enchant" Macros.creature
       , Static (Macros.deontic (AttachHost Enchanted (TypeW Creature))
                                Forbid ["Attack", "Block"] Agent NoDeonticPatient) ]
       Nothing

||| Everybody Lives!'s third conjunct -- "players can't lose the game or
||| win the game this turn". TWO gate kinds under ONE subject, which the
||| one-clause-one-gate row deliberately could not say; it elaborates
||| through the SAME coordination Pacifism's two deeds do, because the
||| outcome gates are deed labels of the one carrier. 1 supported card
||| writes it (measured 2026-08-28; Abyssal Persecutor, Platinum Angel
||| and their family write two SUBJECTS and are plain conjunctions).
public export
everybodyLivesGateLine : Effect []
everybodyLivesGateLine =
  Continuously (Macros.deontic (PlayerGroup AllPlayers) Forbid ["LoseGame", "WinGame"]
                               Agent NoDeonticPatient)
               (Just Macros.thisTurn)

||| Gaea's Revenge, whole card -- "This spell can't be countered. / Haste
||| / This creature can't be the target of nongreen spells or abilities
||| from nongreen sources." The targeting deed's canonical carrier: 30
||| supported sentences write the prohibition (measured 2026-08-28; a
||| naive sweep returns 216, of which 186 sit inside the reminder text
||| printed under hexproof and shroud and are no card's own line).
||| The by-spell/by-source distinction needs no slot: [CR#115.1a]
||| describes a targeting spell by the stack object itself, while
||| [CR#115.1c,115.1d] reach an ability through the object it came from,
||| so the colour is written twice because the rules make it two
||| descriptions of two different objects, and `AbilityOf` is already the
||| predicate that names the second.
public export
gaeasRevenge : Card
gaeasRevenge =
  Macros.card "Gaea's Revenge"
       (Just [Macros.generic 5, Macros.pip Green, Macros.pip Green]) []
       (MkTypeLine [creatureType "Elemental"] [Creature])
       [ Static (Macros.objectCant "Counter" This)
       , Macros.keyword "Haste"
       , Static (Macros.cantBeTargetedBy Macros.thisCreature
                   (AllOf (Joined (And [Macros.spell, Not (ColorIs Green)])
                                  (AbilityOf (AllOf (And [IsSource,
                                                          Not (ColorIs Green)])))))) ]
       (Just (8, 5))

||| Nowhere to Run's first line -- "Creatures your opponents control can
||| be the targets of spells and abilities as though they didn't have
||| hexproof." The targeting deed's PERMISSION, with [CR#609.4]'s premise
||| riding it: one of the five supported lines that let an object be
||| targeted despite hexproof or shroud, and the cell that composes the
||| permission row, the premise slot and the targeting deed at once.
public export
nowhereToRunTargetLine : StaticEffect []
nowhereToRunTargetLine =
  Macros.canBeTargetedAsThough (AllOf Macros.creatureYourOpponentsControl)
    (AllOf (Joined Macros.spell (AbilityHead AnyOnStack)))
    (Not (HasKeyword (TheKeyword "Hexproof")))

||| Nowhere to Run's second sentence -- "Ward abilities of those
||| creatures don't trigger." The TRIGGERED-ABILITY subject, and the deed
||| is `"Trigger"` rather than a reading of "Activate": [CR#603.2a] says
||| outright that triggered abilities "aren't cast or activated" and that
||| "effects that preclude abilities from being activated don't affect
||| them", so the activation prohibition reaches this sentence at no
||| point. [CR#603.2] gives the deed its one participant -- the ability
||| triggers, and nothing is done to a second thing -- so the ability is
||| the deed's AGENT and the carrier refuses it at the patient, which is
||| the gate saying what the rule says.
||| One real supported line: a naive sweep for "don't trigger" returns
||| 13, of which 11 are the reminder text printed under read ahead
||| [CR#702.155a] and one is a different construction (measured
||| 2026-08-28).
public export
nowhereToRunWardLine : StaticEffect []
nowhereToRunWardLine =
  Macros.deontic
    (AllOf (And [ AbilityHead (KeywordClass "Ward")
                , AbilityOf (AllOf Macros.creatureYourOpponentsControl) ]))
    Forbid ["Trigger"] Agent NoDeonticPatient

||| Hithlain Rope's first line -- "This artifact can't be sacrificed."
||| The sacrifice deed's one standalone sentence: [CR#701.21a] moves the
||| permanent from the battlefield to its owner's graveyard, so the row
||| is that sentence and nothing else. Seven of the eight remaining
||| "can't be sacrificed" lines are conjuncts of a wider coordination or
||| sit inside a quoted token ability (measured 2026-08-28).
public export
hithlainRopeSacrificeLock : StaticEffect []
hithlainRopeSacrificeLock = Macros.objectCant "Sacrifice" This

||| Display of Power's first line -- "This spell can't be copied", which
||| [CR#113.6g] functions on the stack beside the can't-be-countered
||| sentence it is printed next to. Three supported lines write it.
public export
displayOfPowerCopyLock : StaticEffect []
displayOfPowerCopyLock = Macros.objectCant "Copy" This

||| Mornsong Aria -- "Players can't draw cards or gain life." The
||| carrier's deed LIST at a third pair, after "can't attack or block"
||| and "can't lose the game or win the game": one subject, one
||| modality, two labels, and no coordination machinery of its own.
public export
mornsongAriaLock : StaticEffect []
mornsongAriaLock =
  Macros.deontic (PlayerGroup AllPlayers) Forbid ["DrawCard", "GainLife"]
                 Agent NoDeonticPatient

||| "Your opponents can't gain life" -- the 9-line subfamily of the
||| life-gain suppression mass, at the carrier's other player subject.
||| Recorded as a witness because the measurement that scheduled a
||| suppression ROW is what retired it: all 23 supported can't-gain-life
||| sentences (10 "Players can't", 9 "Your opponents can't", 3 singular
||| subjects, and Mornsong Aria's conjoined line; re-measured 2026-08-28)
||| are the deontic carrier at the "GainLife" label, and the carrier's
||| deed LIST is the shared suppressed-event row the ticket asked whether
||| to mint. The sibling "can't" statics ride it already -- "DrawCard"
||| (6 lines), "SearchLibrary" (4), "WinGame"/"LoseGame" (8) -- so no
||| suppression subsystem is minted here.
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

||| When Lich's Mastery
lichsMasteryLoss : Ability
lichsMasteryLoss =
  Macros.triggered When (Macros.leavesBattlefield Macros.thisEnchantment) (Concludes LoseGame You)

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
       , Static (Gets (AllOf (And [Macros.creature, HasKeyword (TheKeyword "Flying"),
                                   ControlledBy (PlayerGroup YourOpponents)]))
                      (PtDown (Lit 1)) (PtDown (Lit 1))) ]
       (Just (3, 3))

anglerTurtle : Ability
anglerTurtle =
  Static (Macros.deontic (AllOf Macros.creatureYourOpponentsControl) Require ["Attack"] Agent NoDeonticPatient)


bloodTyrant : Ability
bloodTyrant =
  Macros.triggered Whenever (LosesGame (Macros.a AnyPlayer))
                   (PutCounters (Lit 5) (PrintedKind Macros.plusOnePlusOne) Macros.thisCreature)

theGoldenThrone : Ability
theGoldenThrone =
  Static (Intercepts (LosesGame You) [] Nothing
                     (Sequentially [Macros.exile Macros.thisArtifact,
                                    ChangeLife You (Set (Lit 1))])
                     Repeatedly Nothing)

stunningReversal : Ability
stunningReversal =
  Spell (Continuously (Intercepts (LosesGame You) [] Nothing
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
       , Static (Intercepts (LosesGame You) [] Nothing
                            (Sequentially [Macros.exile Macros.thisCreature,
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
       [ Static (Macros.deontic (AllOf (And [Macros.creature,
                                      Compare [CharAxis Power] Greater
                                              (CountOf (InZone (Macros.handOf You)))]))
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
                     (Enters Macros.thisCreature Nothing)
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
       , Macros.triggered When (Enters Macros.thisCreature Nothing)
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
       [ Static (AndAlso [ CostsToCast This (CostLess (LetterVal X) Nothing)
                         , Define X
                             (Aggregate SumOf (CharAxis Power)
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
                             (CostLess (Lit 2) Nothing)) ]
       Nothing


emeraldMedallion : Card
emeraldMedallion =
  Macros.card "Emerald Medallion" (Just [Macros.generic 2]) []
       (MkTypeLine [] [Artifact])
       [ Static (CostsToCast (AllOf (And [ColorIs Green, Macros.spell, CastBy You]))
                             (CostLess (Lit 1) Nothing)) ]
       Nothing

foundryInspector : Card
foundryInspector =
  Macros.card "Foundry Inspector" (Just [Macros.generic 3]) []
       (MkTypeLine [creatureType "Construct"] [Artifact, Creature])
       [ Static (CostsToCast (AllOf (And [Macros.artifact, Macros.spell, CastBy You]))
                             (CostLess (Lit 1) Nothing)) ]
       (Just (3, 2))

daruWarchief : Card
daruWarchief =
  Macros.card "Daru Warchief"
       (Just [Macros.generic 2, Macros.pip White, Macros.pip White]) []
       (MkTypeLine [creatureType "Human", creatureType "Soldier"] [Creature])
       [ Static (CostsToCast (AllOf (And [HasSubtype (creatureType "Soldier"), Macros.spell, CastBy You]))
                             (CostLess (Lit 1) Nothing))
       , Static (Gets (AllOf (And [HasSubtype (creatureType "Soldier"), Macros.creature, ControlledBy You]))
                      (PtUp (Lit 1)) (PtUp (Lit 2))) ]
       (Just (1, 1))

grandArbiter : Card
grandArbiter =
  Macros.card "Grand Arbiter Augustin IV"
       (Just [Macros.generic 2, Macros.pip White, Macros.pip Blue]) [Legendary]
       (MkTypeLine [creatureType "Human", creatureType "Advisor"] [Creature])
       [ Static (CostsToCast (AllOf (And [ColorIs White, Macros.spell, CastBy You]))
                             (CostLess (Lit 1) Nothing))
       , Static (CostsToCast (AllOf (And [ColorIs Blue, Macros.spell, CastBy You]))
                             (CostLess (Lit 1) Nothing))
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
                             (CostLess (Lit 1) Nothing)) ]
       (Just (2, 2))

arcaneMelee : Card
arcaneMelee =
  Macros.card "Arcane Melee" (Just [Macros.generic 4, Macros.pip Blue]) []
       (MkTypeLine [] [Enchantment])
       [ Static (CostsToCast (AllOf (And [Macros.instantOrSorcery, Macros.spell]))
                             (CostLess (Lit 2) Nothing)) ]
       Nothing

manaMatrix : Card
manaMatrix =
  Macros.card "Mana Matrix" (Just [Macros.generic 6]) []
       (MkTypeLine [] [Artifact])
       [ Static (CostsToCast (AllOf (And [Or [Macros.instant, Macros.enchantment],
                                          Macros.spell, CastBy You]))
                             (CostLess (Lit 2) Nothing)) ]
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
                          (Casts You (Macros.a (And [Macros.instantOrSorcery, Macros.spell])) Nothing)
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

||| Purging Scythe, whole -- "At the beginning of your upkeep, this
||| artifact deals 2 damage to the creature with the least toughness. If
||| two or more creatures are tied for least toughness, you choose one of
||| them." The TIE SENTENCE, and the condition-first conditional it needs:
||| the count is taken over a uniquifying description, so the condition
||| announces the set it counted, and the consequent partitions it. Drop
||| of Honey, Porphyry Nodes and Topple print the same sentence; seven
||| supported lines read "one of them" back off a tie.
public export
purgingScythe : Ability
purgingScythe =
  Macros.triggered At (BeginningOf Upkeep (ByWord Yours))
    (Sequentially
       [ DealDamage Macros.thisArtifact (Lit 2)
                    (Definite (And [Macros.creature,
                                    Superlative MinOf (CharAxis Toughness)
                                                Macros.creature]))
       , If (CompareAmt (CountOf (And [Macros.creature,
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
                          (PutCounters (Lit 1) (PrintedKind Charge) (Macros.target Macros.artifact))
       , Macros.activated (Do (Macros.sacrifice You Macros.thisCreature))
                          (PutCounters (Lit 2) (PrintedKind Charge) (Macros.target Macros.artifact)) ]
       (Just (1, 1))

divineIntervention : Card
divineIntervention =
  Macros.card "Divine Intervention"
       (Just [Macros.generic 6, Macros.pip White, Macros.pip White]) []
       (MkTypeLine [] [Enchantment])
       [ Static (Macros.entersWithCounters Macros.thisEnchantment (Lit 2) Intervention)
       , Macros.triggered At (BeginningOf Upkeep (ByWord Yours))
                          (RemoveCounters (Macros.exactly 1) (Just Intervention) Macros.thisEnchantment)
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
              [ RemoveCounters (Macros.exactly 1) (Just Omen) Macros.thisEnchantment
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
                          (PutCounters (Lit 1) (PrintedKind Spite) Macros.thisAura)
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
       , Macros.triggered When (Enters Macros.thisCreature Nothing)
                          (Macros.gainsDesignation You TheInitiative Instructed)
       , Macros.triggeredIf At
                            (BeginningOf EndStep (ByWord Yours))
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
       [ Macros.triggered When (Enters Macros.thisEquipment Nothing)
                          (DealDamage It (Lit 3)
                               (TargetGroup (Macros.upTo 1) Macros.creature))
       , Macros.triggered Whenever
                          (Dies (CountedGroup (Macros.atLeast 1) Nothing Macros.creature))
                          (PutCounters (Lit 1) (PrintedKind Rev) Macros.thisEquipment)
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

||| Phyrexian Ingester, whole. The imprint trigger exiles and the static
||| line reads the card it exiled: [CR#607.2a]'s exile linkage, not
||| [CR#607.2d]'s chosen-value linkage. The read needs no cross-ability
||| mention because [CR#607.2a] equates "the exiled cards" with cards
||| "exiled with [this object]" -- the same linked pair under either
||| spelling -- so `ExiledWith This` writes it deictically.
phyrexianIngester : Card
phyrexianIngester =
  Macros.card "Phyrexian Ingester"
       (Just [Macros.generic 6, Macros.pip Blue]) []
       (MkTypeLine [creatureType "Phyrexian", creatureType "Beast"] [Creature])
       [ AbilityWord Imprint
           (Macros.triggered When (Enters Macros.thisCreature Nothing)
                             (Macros.may You
                                (Macros.exile
                                   (Macros.target
                                      (And [Macros.creature, Macros.nontoken])))))
       , phyrexianIngesterPump ]
       (Just (3, 3))

||| Drach'Nyen, the same shape on an Equipment: the enters trigger exiles
||| up to one creature and the equipment's static line reads "the exiled
||| card's power" ([CR#607.2a] again). The card prints no linkage word at
||| all, so the self-word is the bench's to pick, and it picks the one
||| this card's own enters trigger already writes -- "this Equipment".
||| The card-type spelling stood in only while the subtype linkage cell
||| was shut.
drachNyen : Card
drachNyen =
  Macros.card "Drach'Nyen"
       (Just [Macros.generic 4, Macros.pip Black, Macros.pip Red]) [Legendary]
       (MkTypeLine [artifactType "Equipment"] [Artifact])
       [ Macros.triggered When (Enters Macros.thisEquipment Nothing)
                          (Macros.exile (TargetGroup (Macros.upTo 1) Macros.creature))
       , Static (AndAlso [ Gains (AttachHost Equipped (TypeW Creature))
                                 (KeywordAbility "Menace" Nothing)
                         , Gets (AttachHost Equipped (TypeW Creature))
                                (PtUp (LetterVal X)) (PtUp (Lit 0))
                         , Define X
                             (StatOf Power
                                (Macros.a (ExiledWith Macros.thisEquipment))) ])
       , Macros.keywordCosting "Equip" (Mana [Macros.generic 2]) ]
       Nothing

||| Soul's Might: "Put X +1/+1 counters on target creature, where X is
||| that creature's power." The definition reads the mention its own
||| clause introduced, which is why it must be written after it.
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
                          (Macros.attacks (CountedGroup (Macros.atLeast 1) Nothing Macros.creature))
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
                          (Enters (CountedGroup (Macros.atLeast 1) Nothing
                                         (And [IsToken, ControlledBy You])) Nothing)
                          (May Nothing Macros.drawACard Nothing Nothing) ]
       (Just (2, 4))


woodlandChampion : Card
woodlandChampion =
  Macros.card "Woodland Champion" (Just [Macros.generic 1, Macros.pip Green]) []
       (MkTypeLine [creatureType "Elf", creatureType "Scout"] [Creature])
       [ Macros.triggered Whenever
                          (Enters (CountedGroup (Macros.atLeast 1) Nothing
                                         (And [IsToken, ControlledBy You])) Nothing)
                          (PutCounters GroupSize (PrintedKind Macros.plusOnePlusOne)
                                Macros.thisCreature) ]
       (Just (2, 2))

ingeniousArtillerist : Card
ingeniousArtillerist =
  Macros.card "Ingenious Artillerist"
       (Just [Macros.generic 2, Macros.pip Red]) []
       (MkTypeLine [creatureType "Human", creatureType "Artificer"] [Creature])
       [ Macros.triggered Whenever
                          (Enters (CountedGroup (Macros.atLeast 1) Nothing
                                         (And [Macros.artifact, ControlledBy You])) Nothing)
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
                          (Macros.attacksPlayer (CountedGroup (Macros.atLeast 1) Nothing Macros.creature) You)
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
                          (Continuously (Macros.deontic It Forbid ["Block"] Patient NoDeonticPatient)
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
                          (Macros.putIntoFrom (CountedGroup (Macros.atLeast 1) Nothing Macros.creature)
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
                          (Macros.putIntoFrom (CountedGroup (Macros.atLeast 1) Nothing
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
                                  (Enters (CountedGroup (Macros.atLeast 1) Nothing
                                                        (And [Macros.creature, ControlledBy You,
                                                              Compare [CharAxis ManaValue] AtMost (Lit 3)])) Nothing)
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
                                    (Enters (Macros.a Macros.land) Nothing)
                                    (DuringWindow Turn (Just Yours))
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
                          (Macros.putIntoFrom (CountedGroup (Macros.atLeast 1) Nothing
                                                     (And [Macros.creature,
                                                           InZone Macros.yourLibrary]))
                                       (Macros.graveyardOf You)
                                       (FromZone [Macros.yourLibrary]))
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
                            [BecomesBlocked Macros.thisCreature Nothing]
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
                            [BecomesBlocked Macros.thisCreature
                                             (Just (Macros.a Macros.creature))]
                            (DealDamage Macros.thisCreature (Lit 3)
                                        (That (TypeW Creature))) ]
       (Just (4, 4))

||| Giggling Skitterspike's header -- "Whenever this creature attacks,
||| blocks, or becomes the target of a spell, …": the three-armed
||| coordination the n-ary seat is for, gated arm by arm. Its BODY does
||| not write: "it deals damage equal to its power to each opponent"
||| reads back a mention the header never makes. Every arm announces the
||| self, but the targeting arm announces its targeter beside it, so
||| whole agreement fails and the tail is handed the outer discourse
||| bare. Pinned as `badThreeArmHeaderReadback`. The card's blocker, not
||| the seat's.
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

||| Corpsejack Menace — "If one or more +1/+1 counters would be put on a
||| creature you control, twice that many +1/+1 counters are put on it
||| instead." The family's second named whole; the card itself waits on a
||| `Fungus` creature-subtype row, so the ability alone is benched.
corpsejackMenace : Ability
corpsejackMenace =
  Static (Intercepts
            (Macros.manyCounterEvent CounterPut Macros.plusOnePlusOne
                                     (Macros.a Macros.creatureYouControl)) [] Nothing
            (PutCounters (Times 2 ThatMuch) (PrintedKind Macros.plusOnePlusOne) It)
            Repeatedly Nothing)

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
                      (CountedGroup (Macros.atLeast 1) Nothing IsToken) You) [] Nothing
                   (Create You (Times 2 GroupSize) TokenAsThose [])
                   Repeatedly Nothing)
       , Static (Intercepts
                   (Macros.manyCountersPutByEffect
                      (Macros.a (And [Permanent, ControlledBy You]))) [] Nothing
                   (PutCountersOfThoseKinds (Times 2 ThatMuch) (That PermanentW))
                   Repeatedly Nothing) ]
       Nothing

||| Doc Samson, Super Psychiatrist — "If you would put one or more counters
||| on a permanent you control, put that many plus one of each of those
||| kinds of counters on that permanent instead": the per-kind spelling
||| with its agent voiced. Pir, Imaginative Rascal writes the same clause
||| over "a permanent your team controls"; the team form landed
||| ([CR#102.4], `PlayerGroup YourTeam`) and Pir's own clause is benched
||| at `pirDistributive`, and the card is whole at `pirImaginativeRascal`
||| now that [CR#702.124j]'s "Partner with [name]" has a row.
docSamsonDistributive : Ability
docSamsonDistributive =
  Static (Intercepts
            (Macros.manyBareCountersPutBy You
               (Macros.a (And [Permanent, ControlledBy You]))) [] Nothing
            (PutCountersOfThoseKinds (Plus ThatMuch (Lit 1)) (That PermanentW))
            Repeatedly Nothing)

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
                                      ControlledBy You]))) [] Nothing
                   (PutCountersOfThoseKinds (Plus ThatMuch (Lit 1))
                                            (That PermanentW))
                   Repeatedly Nothing)
       , Static (Intercepts
                   (Macros.manyBareCounterEvent CounterPut You) [] Nothing
                   (GetsCountersOfThoseKinds You (Plus ThatMuch (Lit 1)))
                   Repeatedly Nothing) ]
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
    (Enters (Macros.a (And [Macros.nontoken, Macros.creatureYouControl])) Nothing)
    (Matches Macros.thisCreature (HasCounters Nothing))
    (PutSameCounters Macros.thisCreature (That (TypeW Creature)))

||| Denry Klin, Editor in Chief -- whole. The entry line fills the kind
||| slot with a printed menu, the choice made before it enters
||| [CR#614.12a] by the "you" [CR#109.5] its own text names; the trigger
||| is `denryKlinSameKinds`.
denryKlin : Card
denryKlin =
  Macros.card "Denry Klin, Editor in Chief"
       (Just [Macros.generic 2, Macros.pip White, Macros.pip Blue])
       [Legendary]
       (MkTypeLine [creatureType "Cat", creatureType "Advisor"] [Creature])
       [ Static (EntersWithCounters Macros.thisCreature (Lit 1)
                   (ChosenKind [ Macros.plusOnePlusOne
                               , KeywordCounter "FirstStrike"
                               , KeywordCounter "Vigilance" ])
                   Fresh)
       , denryKlinSameKinds ]
       (Just (2, 2))

||| Helica Glider -- whole, and the menu's other determiner spelling:
||| "a flying counter or a first strike counter", one determiner per arm
||| against Denry Klin's one over the list. Same node, since each picks
||| one counter of one kind from a written range.
helicaGlider : Card
helicaGlider =
  Macros.card "Helica Glider"
       (Just [Macros.generic 2, Macros.pip White]) []
       (MkTypeLine [creatureType "Nightmare", creatureType "Squirrel"] [Creature])
       [ Static (EntersWithCounters Macros.thisCreature (Lit 1)
                   (ChosenKind [ KeywordCounter "Flying"
                               , KeywordCounter "FirstStrike" ])
                   Fresh) ]
       (Just (2, 2))

||| Grimdancer -- whole: "This creature enters with your choice of two
||| different counters on it from among menace, deathtouch, and lifelink."
||| The menu picked TWICE with distinctness written, against Denry Klin's
||| and Helica Glider's single pick from the same slot.
public export
grimdancer : Card
grimdancer =
  Macros.card "Grimdancer"
       (Just [Macros.generic 1, Macros.pip Black, Macros.pip Black]) []
       (MkTypeLine [creatureType "Nightmare"] [Creature])
       [ Static (EntersWithCounters Macros.thisCreature (Lit 2)
                   (DistinctChosenKinds [ KeywordCounter "Menace"
                                        , KeywordCounter "Deathtouch"
                                        , KeywordCounter "Lifelink" ])
                   Fresh) ]
       (Just (3, 3))

||| Me, the Immortal's combat trigger -- "put your choice of a +1/+1,
||| first strike, vigilance, or menace counter on Me": the menu at the
||| put seat, four arms, and the same slot as the entry side.
meTheImmortalCounterMenu : Ability
meTheImmortalCounterMenu =
  Macros.triggered At (BeginningOf Combat (ByWord Yours))
    (PutCounters (Lit 1)
       (ChosenKind [ Macros.plusOnePlusOne
                   , KeywordCounter "FirstStrike"
                   , KeywordCounter "Vigilance"
                   , KeywordCounter "Menace" ])
       Macros.thisCreature)

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

||| Bloom Hulk — whole. "When this creature enters, proliferate." The
||| cheapest whole-card carrier the proliferate corpus offers: a vanilla
||| body and one entry trigger, so the card pays for the label and the
||| self-reading counter row and for nothing else.
bloomHulk : Card
bloomHulk =
  Macros.card "Bloom Hulk" (Just [Macros.generic 3, Macros.pip Green]) []
       (MkTypeLine [creatureType "Plant", creatureType "Elemental"] [Creature])
       [ Macros.triggered When (Enters Macros.thisCreature Nothing) Macros.proliferate ]
       (Just (4, 4))

||| Tromell, Seymour's Butler — whole. The entry rider is the additional
||| counter mark; the activated line is a counted iteration of the whole
||| keyword action -- [CR#701.34a] fixes the per-kind amount at one, so
||| a written count can only iterate the whole action -- over a letter
||| the same clause defines.
tromell : Card
tromell =
  Macros.card "Tromell, Seymour's Butler"
       (Just [Macros.generic 2, Macros.pip Green]) [Legendary]
       (MkTypeLine [creatureType "Elf", creatureType "Advisor"] [Creature])
       [ Static (Macros.entersWithAdditionalCounters
                   (Each (And [Macros.creature, Macros.nontoken, ControlledBy You,
                               OtherThan Macros.thisCreature]))
                   (Lit 1) Macros.plusOnePlusOne)
       , Macros.activated (Compound [Mana [Macros.generic 1], TapSymbol])
           (Sequentially
              [ Repeated (LetterVal X) Macros.proliferate
              , Define X (CountOf (And [Macros.nontoken, Macros.creature,
                                        ControlledBy You,
                                        HappenedTo Entry Lookback.ThisTurn Nothing])) ]) ]
       (Just (2, 3))

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
boonOfSafetyPut = PutCounters (Lit 1) (PrintedKind Shield) (Macros.target Macros.creature)

||| Vivien's Talent and Teferi's Talent — "put a loyalty counter on
||| enchanted planeswalker": the loyalty kind's one-shot put, both cards
||| writing the phrase identically.
talentLoyaltyPut : Effect []
talentLoyaltyPut =
  PutCounters (Lit 1) (PrintedKind LoyaltyCounter)
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
                   (Macros.tokensCreatedUnder (CountedGroup (Macros.atLeast 1) Nothing
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
                   (Macros.tokensCreatedByEffectUnder (CountedGroup (Macros.atLeast 1) Nothing IsToken)
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
                   (Macros.tokensCreated (CountedGroup (Macros.atLeast 1) Nothing IsToken)) [] Nothing
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
                   (Macros.tokensCreatedUnder (CountedGroup (Macros.atLeast 1) Nothing IsToken) You) [] Nothing
                   (Create You (Times 2 GroupSize) TokenAsThose [])
                   Repeatedly Nothing) ]
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
                   (Continuously (LosesAllAbilities Macros.thisCreature Nothing)
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
                  (AndAlso [ LosesAllAbilities (Macros.target Macros.creature) Nothing
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
       [ Static (AndAlso [ LosesAllAbilities (AllOf Macros.creature) Nothing
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
                         , LosesAllAbilities It Nothing ]) ]
       Nothing

kenrithsTransformation : Card
kenrithsTransformation =
  Macros.card "Kenrith's Transformation" (Just [Macros.generic 1, Macros.pip Green]) []
       (MkTypeLine [enchantmentType "Aura"] [Enchantment])
       [ Macros.keywordSubject "Enchant" Macros.creature
       , Macros.triggered When (Enters Macros.thisAura Nothing) (Draw You (Lit 1))
       , Static (AndAlso [ LosesAllAbilities (AttachHost Enchanted (TypeW Creature)) Nothing
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
       , Static (AndAlso [ LosesAllAbilities (AttachHost Enchanted (TypeW Creature)) Nothing
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
                         , LosesAllAbilities It Nothing ]) ]
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
            [ Macros.triggered When (Enters Macros.thisSiege Nothing)
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
                                                                  (PrintedKind Macros.plusOnePlusOne) Macros.thisCreature) ]
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
                          (CostLess (Lit 1) Nothing)) ]


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
                                                Compare [CharAxis Power] AtLeast (Lit 4)])))
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
       [ Spell (Sequentially [ Macros.scryOne
                             , Macros.drawACard ]) ]
       Nothing

serumVisions : Card
serumVisions =
  Macros.card "Serum Visions" (Just [Macros.pip Blue]) [] (MkTypeLine [] [Sorcery])
       [ Spell (Sequentially [ Macros.drawACard
                             , Macros.scry (Lit 2) ]) ]
       Nothing

consider : Card
consider =
  Macros.card "Consider" (Just [Macros.pip Blue]) [] (MkTypeLine [] [Instant])
       [ Spell (Sequentially [ Macros.surveilOne
                             , Macros.drawACard ]) ]
       Nothing

crystalBall : Card
crystalBall =
  Macros.card "Crystal Ball" (Just [Macros.generic 3]) [] (MkTypeLine [] [Artifact])
       [ Macros.activated (Compound [Mana [Macros.generic 1], TapSymbol])
                          (Macros.scry (Lit 2)) ]
       Nothing

nefariousImp : Card
nefariousImp =
  Macros.card "Nefarious Imp" (Just [Macros.generic 2, Macros.pip Black]) []
       (MkTypeLine [creatureType "Imp"] [Creature])
       [ Macros.keyword "Flying"
       , Macros.triggered Whenever
                          (Macros.leavesBattlefield
                             (CountedGroup (Macros.atLeast 1) Nothing
                                     (And [Permanent, ControlledBy You])))
                          (Macros.scryOne) ]
       (Just (2, 1))

saheeliFiligreeMaster : Card
saheeliFiligreeMaster =
  Macros.cardOf "Saheeli, Filigree Master"
       (Just [Macros.generic 2, Macros.pip Blue, Macros.pip Red])
       [Legendary] (MkTypeLine [planeswalkerType "Saheeli"] [Planeswalker])
       [ Macros.activated (LoyaltySymbol (LoyaltyUp 1))
                          (Sequentially
                      [ Macros.scryOne
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

||| Rosheen Meanderer, whole card -- "{T}: Add {C}{C}{C}{C}. Spend this
||| mana only on costs that contain {X}." The spend purpose that names a
||| COST by a symbol its text writes, where the two older cells name an
||| object.
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

||| Adarkar Unicorn, whole card -- "{T}: Add {U} or {C}{U}. Spend this
||| mana only to pay cumulative upkeep costs." The keyword-named cost,
||| and the card the landed `Keyword.CumulativeUpkeep` row did NOT
||| unblock: that row is what a permanent prints, this names the cost it
||| charges.
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

||| Overgrown Zealot, whole card -- its second ability is "Spend this
||| mana only to turn permanents face up", the purpose that names a
||| SPECIAL ACTION [CR#116.2b] rather than a cost's contents or a
||| keyword.
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

||| Rootcoil Creeper's second ability -- "{T}: Add two mana of any one
||| color. Spend this mana only to cast spells from your graveyard."
||| The ZONE-QUALIFIED spend purpose, which needed no cell of its own:
||| the landed cast-provenance predicate describes the spell, and
||| [CR#601.2a] has the card on the stack before [CR#601.2h] takes the
||| payment. Its two negative kin (Karolina Dean, Vhal) write the same
||| cell under `Not`.
public export
rootcoilCreeperGraveyardMana : Ability
rootcoilCreeperGraveyardMana =
  Macros.activated TapSymbol
                   (AddMana You (Lit 2) (AnyColor SameColor)
                     [SpendOnly [ToCast (And [Macros.spell,
                                              CastFrom (Macros.graveyardOf You)])]])

||| Black Mana Battery, whole card -- the storage-counter family's shape
||| written twice over. Its second ability is the round's two purchases
||| at once: an ANY-NUMBER removal in the activation cost, and the
||| production reading how many that removal took.
||| "An additional" gets no cell of its own: all 16 supported lines that
||| write it (measured 2026-08-28) are the second add of a chain or a
||| production off another one, so the word marks discourse and the
||| structure is the sequence.
public export
blackManaBattery : Card
blackManaBattery =
  Macros.card "Black Mana Battery" (Just [Macros.generic 4]) []
       (MkTypeLine [] [Artifact])
       [ Macros.activated (Compound [Mana [Macros.generic 2], TapSymbol])
                          (PutCounters (Lit 1) (PrintedKind Charge) Macros.thisArtifact)
       , Macros.activated (Compound [TapSymbol,
                                     Do (RemoveCounters Macros.anyNumber (Just Charge)
                                                        Macros.thisArtifact)])
                          (Sequentially
                             [ AddMana You (Lit 1) (Runs [[OfColor Black]]) []
                             , AddMana You RemovedThisWay (Runs [[OfColor Black]]) [] ]) ]
       Nothing

||| Cyclone's upkeep trigger, first sentence -- "put a wind counter on
||| this enchantment, then sacrifice this enchantment unless you pay {G}
||| for each wind counter on it". The COLOURED scaled payment: the count
||| composed before, the unit did not. Its second sentence reads the
||| payment back and is not this witness.
public export
cycloneUpkeepPayment : Ability
cycloneUpkeepPayment =
  Macros.triggered At (BeginningOf Upkeep (ByWord Yours))
    (Sequentially
       [ PutCounters (Lit 1) (PrintedKind Wind) Macros.thisEnchantment
       , Macros.mayElse You
           (Pay You (ScaledMana (RunUnit [Macros.pip Green])
                                (Times 1 (CountersOn Wind Macros.thisEnchantment)))
                PaidOnce)
           (Macros.sacrifice You Macros.thisEnchantment) ])

||| War Tax's payment -- "pays {X} for each attacking creature". The
||| product whose PER-UNIT is read rather than printed, announced by the
||| ability's own {X} [CR#107.3a]. The fragment and not the card: all
||| four lines writing this product are attack/block gates, and the
||| payer noun those gates derive is a separate gap.
public export
warTaxScaledPayment : Cost [letterB X]
warTaxScaledPayment =
  ScaledMana GenericUnit
             (TimesOf (LetterVal X) (CountOf (And [Macros.creature, Attacking])))

||| Rune Snag, whole card -- "Counter target spell unless its controller
||| pays {2} plus an additional {2} for each card named Rune Snag in each
||| graveyard." A fixed base beside a scaled one needs no new cost cell:
||| the compound already joins them, and "plus an additional" is the
||| coordinator's spelling. Spell Stutter and Concerted Defense are the
||| other two lines.
public export
runeSnag : Card
runeSnag =
  Macros.card "Rune Snag" (Just [Macros.generic 1, Macros.pip Blue]) []
       (MkTypeLine [] [Instant])
       [ Spell (Macros.mayElse (ControllerOf (Macros.target Macros.spell))
                  (Pay They (Compound [Mana [Macros.generic 2],
                                       ScaledMana GenericUnit
                                         (Times 2 (CountOf
                                            (And [Named (PrintedName "Rune Snag"),
                                                  InZone (ZoneAt Graveyard Bare)])))])
                       PaidOnce)
                  (Macros.counterSpell It)) ]
       Nothing

||| Elemental Resonance, whole card -- "add mana equal to enchanted
||| permanent's mana cost". The production a card names by a PRINTED
||| COST, leaving [CR#106.8..106.11] to say what each symbol adds.
public export
elementalResonance : Card
elementalResonance =
  Macros.card "Elemental Resonance"
       (Just [Macros.generic 2, Macros.pip Green, Macros.pip Green]) []
       (MkTypeLine [enchantmentType "Aura"] [Enchantment])
       [ Macros.keywordSubject "Enchant" Permanent
       , Macros.triggered At (BeginningOf FirstMain (ByWord Yours))
           (AddMana You (Lit 1)
                    (AsPrintedCost (AttachHost Enchanted PermanentW)) []) ]
       Nothing



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
       [ Static (CantPrevent AnyDamage (DamageDescribed Everywhere (Just Macros.thisCreature))
                                 NoPreventionOnly) ]
       (Just (7, 7))

public export
flaringPain : Card
flaringPain =
  Macros.card "Flaring Pain" (Just [Macros.generic 1, Macros.pip Red]) []
       (MkTypeLine [] [Instant])
       [ Spell (Continuously (CantPrevent AnyDamage (DamageDescribed Everywhere Nothing) NoPreventionOnly)
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
                       (Macros.deontic (AllOf (And [Macros.creature,
                                             ControlledBy (That PlayerW)]))
                                Require ["Attack"] Agent
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
                                                 (PrintedKind Macros.plusOnePlusOne)
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
                                               (PrintedKind Macros.plusOnePlusOne)
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
                                               (PrintedKind Macros.plusOnePlusOne)
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

||| "Pinpoint Avalanche deals 4 damage to target creature. The damage
||| can't be prevented." -- [CR#615.12]'s rider with an ANAPHORIC subject:
||| "the damage" names the damage event this card's own previous clause
||| described, which neither a recipient scope nor a by-phrase says. 9
||| supported sentences write it (measured 2026-08-28).
public export
pinpointAvalanche : Card
pinpointAvalanche =
  Macros.card "Pinpoint Avalanche"
       (Just [Macros.generic 3, Macros.pip Red, Macros.pip Red]) []
       (MkTypeLine [] [Instant])
       [ Spell (Sequentially
                  [ DealDamage This (Lit 4) (Macros.target Macros.creature)
                  , Continuously
                      (CantPrevent AnyDamage ThatDamage NoPreventionOnly) Nothing ]) ]
       Nothing

||| "Damage that would be dealt to that creature this turn can't be
||| prevented or dealt instead to another permanent or player" --
||| Whippoorwill's middle statement, the CONJOINED ban over [CR#615.12]'s
||| prevention and [CR#614.9]'s redirection. Benched as a fragment because
||| the ability's other two statements (a regeneration prohibition and a
||| delayed exile) are not this round's; the subject is written as a
||| description rather than the printed anaphor for the same reason.
||| Lava Burst writes the same ban under an if-would antecedent, which no
||| statement row takes -- recorded as the family's remaining spelling gap.
public export
whippoorwillImmunity : Effect []
whippoorwillImmunity =
  Continuously
    (CantPrevent AnyDamage
                 (DamageDescribed (Macros.shieldingIt (Macros.a Macros.creature)) Nothing)
                 NoRedirectEither)
    (Just Macros.thisTurn)

||| "If damage would be dealt to this creature, put that many +1/+1
||| counters on it instead." The replacement side of the recipient's
||| damage event, whose body reads the MAGNITUDE of the damage that would
||| have been dealt: [CR#614.6] keeps the event from happening, but
||| [CR#614.1] has the replacement watch an event that WOULD happen and
||| [CR#120.8] makes that event one of a stated size, so `eventIntro`
||| leaves the amount for "that many" -- `RollsDice`' announcement at the
||| damage seat. 10 supported bodies read it (measured 2026-08-28).
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

||| "If you would gain life, draw that many cards instead" -- Nefarious
||| Lich's middle statement. The life change as a REPLACEABLE event, whose
||| announced amount the body reads: [CR#119.9] makes a 0-life gain no
||| life gain event at all, so the event a replacement reaches is one of a
||| stated size. Benched as a fragment; the card's other two statements
||| are not this round's.
public export
nefariousLichGain : AbilityAt []
nefariousLichGain =
  Static (Intercepts (LifeChanges You LifeGoesUp) [] Nothing
                     (Draw You ThatMuch) Repeatedly Nothing)

||| "Whenever this creature is dealt combat damage, you gain that much
||| life." The damage KIND written at an event position: `IsDealtDamage`'s
||| new adjective, the same vocabulary the shield rows spell. 9 supported
||| sentences carry it (measured 2026-08-28).
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

||| "Whenever an opponent is dealt noncombat damage, this creature gets
||| +3/+0 until end of turn." The other pole of the same adjective.
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

||| "Whenever you gain life, target opponent loses that much life."
||| [CR#119.9] writes this header in the rules' own words; the row
||| announces the amount [CR#119.3] moved the total by, which "that much
||| life" reads.
public export
sanguineBond : Card
sanguineBond =
  Macros.card "Sanguine Bond"
       (Just [Macros.generic 3, Macros.pip Black, Macros.pip Black]) []
       (MkTypeLine [] [Enchantment])
       [ Macros.triggered Whenever (LifeChanges You LifeGoesUp)
                          (ChangeLife (Macros.target Opponent) (Down ThatMuch)) ]
       Nothing

||| "Whenever an opponent loses life, you gain that much life." The other
||| direction of the same row.
public export
exquisiteBlood : Card
exquisiteBlood =
  Macros.card "Exquisite Blood"
       (Just [Macros.generic 4, Macros.pip Black]) []
       (MkTypeLine [] [Enchantment])
       [ Macros.triggered Whenever (LifeChanges (Macros.anOpponent) LifeGoesDown)
                          (Macros.gainsLife You ThatMuch) ]
       Nothing

||| "Whenever you gain life, put that many +1/+1 counters on this
||| creature."
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

||| "If a source would deal damage to another Dinosaur you control,
||| prevent all but 1 of that damage." [CR#615.10]'s per-event cut written
||| as the damage it LEAVES. All 4 supported "prevent all but" sentences
||| sit at this seat; the shield determiner's twin is a measured zero.
public export
templeAltisaur : Card
templeAltisaur =
  Macros.card "Temple Altisaur"
       (Just [Macros.generic 4, Macros.pip White]) []
       (MkTypeLine [creatureType "Dinosaur"] [Creature])
       [ Static (PreventsFrom AnyDamage (DealtBy (Macros.a Macros.source))
                              (Macros.shieldingIt
                                 (Macros.a (And [HasSubtype (creatureType "Dinosaur"),
                                                 ControlledBy You])))
                              (CutAllBut (Lit 1)) Repeatedly Nothing) ]
       (Just (3, 4))

||| "{T}, Sacrifice this artifact: The next time a source of your choice
||| would deal damage to you this turn, prevent half that damage, rounded
||| down." The cut written as a FRACTION; [CR#107.1a] is why the rounding
||| word is obligatory. Gisela, Blade of Goldnight writes the same arm at
||| `RoundUp`, and the two cards are the whole of the family.
public export
darkSphere : Card
darkSphere =
  Macros.card "Dark Sphere" (Just []) []
       (MkTypeLine [] [Artifact])
       [ Macros.activated
           (Compound [TapSymbol, Do (Macros.sacrifice You Macros.thisArtifact)])
                          (Continuously
                             (PreventsFrom AnyDamage
                                           (DealtBy (Macros.aYourChoice Macros.source))
                                           (Macros.shieldingIt You)
                                           (CutHalf RoundDown) NextTimeOnly Nothing)
                             (Just Macros.thisTurn)) ]
       Nothing

||| "The next time a source of your choice would deal damage to any target
||| this turn, prevent that damage. If damage from a red source is
||| prevented this way, Honorable Passage deals that much damage to the
||| source's controller." [CR#615.5]'s consequence rider with its
||| container clause narrowed by the damage's SOURCE -- 6 supported
||| sentences over 5 cards write that narrowing (measured 2026-08-28).
||| The source read is the generic pronoun's, as Deflecting Palm's is.
||| "The next time a source of your choice would deal damage to you
||| and/or creatures you control this turn, prevent that damage. If damage
||| from a black source is prevented this way, you gain that much life."
||| [CR#615.5]'s consequence rider with its container clause narrowed by
||| the damage's SOURCE -- [CR#615.2] is why a prevention narrows there at
||| all, and [CR#120.1] is what a source is. 6 supported sentences over 5
||| cards write the narrowing (measured 2026-08-28).
||| The recipient is written with "and": the printed "and/or" is a
||| coordination surface this grammar does not spell, recorded as a
||| spelling gap rather than paid here.
public export
shadowbane : Card
shadowbane =
  Macros.card "Shadowbane" (Just [Macros.generic 1, Macros.pip White]) []
       (MkTypeLine [] [Instant])
       [ Spell (Continuously
                  (PreventsFrom AnyDamage
                                (DealtBy (Macros.aYourChoice Macros.source))
                                (Macros.shieldingIt
                                   (Macros.youAnd (AllOf Macros.creatureYouControl)))
                                CutAll NextTimeOnly
                                (Just (If (PreventedFromSource
                                             (And [Macros.source, ColorIs Black]))
                                          (Macros.gainsLife You PreventedThisWay)
                                          Nothing)))
                  (Just Macros.thisTurn)) ]
       Nothing

||| Honorable Passage's rider is the one member of the family this round
||| does NOT bench, and its blocker is recorded rather than paid: "If
||| damage from a red source is prevented this way, Honorable Passage
||| deals that much damage to THE SOURCE'S CONTROLLER" over a shield whose
||| recipient is "any target". The generic pronoun the other source reads
||| use (Deflecting Palm's `ControllerOf It`) needs exactly one object
||| mention to resolve against, and this card leaves two -- the chosen
||| source and the joined-kind target. So Honorable Passage is a card that
||| needs the demonstrative re-sort the "source" word would buy, which
||| corrects the ledger entry saying none of the 13 source reads does.

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
       [ Macros.triggered When (Enters Macros.thisEnchantment Nothing) Macros.drawACard
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
           , Macros.scry (Lit 3)
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
                          (Enters (Macros.a (And [Macros.creature, OtherThan This])) Nothing)
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
                          (Macros.gainsLife (Each Opponent) ThatMuch) ]
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
                               (ControllerOf (That (TypeW Creature)))) ]
       Nothing

public export
spitefulShadows : Card
spitefulShadows =
  Macros.card "Spiteful Shadows" (Just [Macros.generic 1, Macros.pip Black]) []
       (MkTypeLine [enchantmentType "Aura"] [Enchantment])
       [ Macros.keywordSubject "Enchant" Macros.creature
       , Macros.triggered Whenever
                          (IsDealtDamage AnyDamage (AttachHost Enchanted (TypeW Creature)))
                          (DealDamage It ThatMuch (ControllerOf It)) ]
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
                               (ControllerOf (That (TypeW Creature)))) ]
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
                          (IsDealtDamage AnyDamage Macros.thisCreature)
                          (Sequentially
                             [ DealDamage It ThatMuch
                                 (Macros.target (And [Macros.anyTarget,
                                                      OtherThan This]))
                             , If (DealtThisWay AnyPlayer)
                                  (Continuously (Macros.playerCant "GainLife" They)
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
                          (Enters Macros.thisCreature Nothing)
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
                      (Macros.playerCant "GainLife" (PlayerGroup AllPlayers))
                      (Just Macros.thisTurn)
                  , Continuously
                      (CantPrevent AnyDamage (DamageDescribed Everywhere Nothing) NoPreventionOnly)
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
       [ Static (Macros.playerCant "SearchLibrary" (PlayerGroup AllPlayers)) ]
       Nothing

public export
silence : Card
silence =
  Macros.card "Silence" (Just [Macros.pip White]) []
       (MkTypeLine [] [Instant])
       [ Spell (Continuously
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
                  [ Continuously
                      (Macros.playerCant "SearchLibrary" (PlayerGroup AllPlayers))
                      (Just Macros.thisTurn)
                  , Macros.drawACard ]) ]
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
       , Static (Macros.playerCant "GainLife" (AttachHost Enchanted PlayerW)) ]
       Nothing

public export
solfataraLandLock : Effect []
solfataraLandLock =
  Continuously (Macros.playerCant "Play" (Macros.target AnyPlayer))
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
                          (Continuously
                             (SetsChosenQuality (Macros.target Macros.land)
                                (OfYourChoice (SubtypeQ Land) (Just BasicTypesOnly)))
                             (Just Macros.untilEndOfTurn)) ]
       (Just (0, 2))

public export
grixisIllusionist : Card
grixisIllusionist =
  Macros.card "Grixis Illusionist" (Just [Macros.pip Blue]) []
       (MkTypeLine [creatureType "Human", creatureType "Wizard"] [Creature])
       [ Macros.activated TapSymbol
                          (Continuously
                      (SetsChosenQuality
                         (Macros.target (And [Macros.land, ControlledBy You]))
                         (OfYourChoice (SubtypeQ Land) (Just BasicTypesOnly)))
                      (Just Macros.untilEndOfTurn)) ]
       (Just (1, 1))

public export
unstableFrontier : Card
unstableFrontier =
  Macros.card "Unstable Frontier" Nothing [] (MkTypeLine [] [Land])
       [ Macros.activated TapSymbol (AddMana You (Lit 1) (Runs [[Colorless]]) [])
       , Macros.activated TapSymbol
                          (Continuously
                      (SetsChosenQuality
                         (Macros.target (And [Macros.land, ControlledBy You]))
                         (OfYourChoice (SubtypeQ Land) (Just BasicTypesOnly)))
                      (Just Macros.untilEndOfTurn)) ]
       Nothing


public export
extinction : Card
extinction =
  Macros.card "Extinction" (Just [Macros.generic 4, Macros.pip Black]) []
       (MkTypeLine [] [Sorcery])
       [ Spell (Macros.destroy (AllOf (And [Macros.creature,
                                            OfYourChoice (SubtypeQ Creature) Nothing]))) ]
       Nothing

public export
defensiveManeuvers : Card
defensiveManeuvers =
  Macros.card "Defensive Maneuvers"
       (Just [Macros.generic 3, Macros.pip White]) []
       (MkTypeLine [] [Instant])
       [ Spell (Macros.gets (AllOf (And [Macros.creature,
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
       [ Spell (Macros.gets (AllOf (And [Macros.creature,
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
                          (Macros.destroy (AllOf (And [HasType Enchantment,
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
                      (AllOf (And [Macros.creature,
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
                                (And [Permanent, ControlledBy You,
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
                  , Macros.gets (AllOf (And [Macros.creature,
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
       , Static (Gets (AllOf (And [Macros.creature, ControlledBy You,
                                   OfChosen (SubtypeQ Creature)]))
                      (PtUp (Lit 1)) (PtUp (Lit 1))) ]
       Nothing

public export
sharedTriumph : Card
sharedTriumph =
  Macros.card "Shared Triumph" (Just [Macros.generic 1, Macros.pip White]) []
       (MkTypeLine [] [Enchantment])
       [ Static (Macros.entersChoosing Macros.thisEnchantment (SubtypeQ Creature))
       , Static (Gets (AllOf (And [Macros.creature, OfChosen (SubtypeQ Creature)]))
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
       [ Static (Macros.entersChoosing Macros.thisEnchantment (SubtypeQ Creature))
       , Static (Gets (AllOf (And [Macros.creature, OfChosen (SubtypeQ Creature)]))
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
       , Static (CostsToCast (AllOf (And [Macros.creature, Macros.spell,
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
       , Static (Gets (AllOf (And [Macros.creature, ControlledBy You,
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
                          (Just (AllOf (And [Macros.source, OfLastChosen Color])))
                          Nothing) ]
       Nothing

||| Sanctuary Blade -- "As this Equipment becomes attached to a creature,
||| choose a color. / Equipped creature gets +2/+0 and has protection from
||| the last chosen color. / Equip {3}". The ATTACH-triggered chooser's
||| witness: a replacement effect watching [CR#701.3a]'s attachment rather
||| than [CR#614.1c]'s entering, and the marked read it feeds
||| [CR#607.2d] -- marked because re-equipping re-fires the chooser.
||| Beckoning Will-o'-Wisp's chooser -- "At the beginning of combat on
||| your turn, choose an opponent." The COMBAT-trigger chooser position;
||| Triarch Stalker writes the same ability. Neither card lands and the
||| chooser is not why. Two further blockers apiece: the flavor word
||| ("Lure the Unwary", "Targeting Relay") is not one of [CR#207.2c]'s
||| ability words and no row spells it; and both spend the read inside
||| "creatures attacking the last chosen player", an attributive
||| attacking-DEFENDER phrase that `Attacking` carries no slot for -- 94
||| supported lines write one, so that is its own cell and not this
||| round's.
public export
beckoningWillOWispChooser : Ability
beckoningWillOWispChooser =
  Macros.triggered At (BeginningOf Combat (ByWord Yours))
                   (Macros.choose (Macros.a Opponent))

||| Koh, the Face Stealer's third line -- "Pay 1 life: Choose a creature
||| card exiled with Koh." The chooser at an ACTIVATED ability and at an
||| OBJECT: the exile linkage [CR#607.2a] narrows the choice and the clause
||| leaves a mention, not a chosen value. The card's fourth line, "Koh has
||| all activated and triggered abilities of the last chosen card", wanted
||| two things: a grant of ANOTHER object's described ability set, which
||| `GainsAbilitiesOf` now writes at exactly this pair of classes, and the
||| object-sorted marked read, which is still the line's blocker. Idris,
||| Soul of the TARDIS writes the same class pair and is blocked the same
||| way -- its source is "the exiled card", a participle definite its own
||| earlier ability stamped, and no static line reads across an ability
||| boundary. So the class slot's LIST arm has no bench entry: both
||| supported lines that write two classes are held up by their SOURCE.
public export
kohChooser : Ability
kohChooser =
  Macros.activated (Macros.payLife You 1)
    (Macros.choose (Macros.a (And [Macros.creature, ExiledWith This])))

||| Volrath's Laboratory's first line -- "As this artifact enters, choose a
||| color and a creature type." The COMPOUND chooser at the as-enters
||| position: two choices in ONE sentence, spelled as the coordination it
||| is, which is all the family was missing -- the two choosers already
||| composed and both reads already elaborated. Riptide Replicator writes
||| the same line. Neither card lands whole and the chooser is not why:
||| both spend the reads on a TOKEN SPEC ("a 2/2 creature token of the
||| chosen color and type"), and `TokenChars` carries a literal colour
||| list and a literal type line with nowhere for a read to sit.
public export
volrathsLaboratoryChoice : StaticEffect []
volrathsLaboratoryChoice =
  AndAlso [ Macros.entersChoosing Macros.thisArtifact Color
          , Macros.entersChoosing Macros.thisArtifact (SubtypeQ Creature) ]

||| Call to Arms' first line -- "As this enchantment enters, choose a color
||| and an opponent." The same coordination ACROSS SORTS, a quality beside
||| a player, which the container needed no extra row for. The card's
||| second and third lines read both choices inside a most-common-colour
||| comparison and are not taken here.
public export
callToArmsChoice : StaticEffect []
callToArmsChoice =
  AndAlso [ Macros.entersChoosing Macros.thisEnchantment Color
          , Macros.entersChoosingPlayer Macros.thisEnchantment
                                        (Just OpponentsOnly) ]

||| Forgotten Lore's first sentence -- "Target opponent chooses a card in
||| your graveyard." The chooser at an OBJECT, with the chooser written:
||| what it leaves is a MENTION, which the ordinary anaphora reads, and no
||| chosen VALUE for a linked ability to name [CR#607.2d]. Shrouded Lore
||| writes the same sentence. Neither card lands: the tail repeats the
||| process with an exclusion memory ("that opponent can't choose a card
||| already chosen for Forgotten Lore") and then reads "the last chosen
||| card", the object-sorted marked read no row writes.
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
                                    (OfLastChosen Color)) ])
       , Macros.keywordCosting "Equip" (Mana [Macros.generic 3]) ]
       Nothing

||| Psychic Paper minus its three-way coordination -- "As this Equipment
||| becomes attached to a creature, choose a creature card name and a
||| creature type." with the setting half of "its name and creature type
||| are the last chosen name and creature type". Two witnesses in one
||| fragment: the COMPOUND chooser (two choices in one sentence, spelled
||| as the coordination it is) at the attach position, and the marked
||| read at the two sorts colour is not. The ward and can't-be-blocked
||| conjuncts of the printed second line are the coordination cell's,
||| not this one's.
public export
psychicPaperChoiceAndReads : AbilitySeq []
psychicPaperChoiceAndReads =
  [ Static (AndAlso [ Macros.attachChoosing Macros.thisEquipment CardName
                    , Macros.attachChoosing Macros.thisEquipment
                                            (SubtypeQ Creature) ])
  , Static (AndAlso
      [ SetsChosenQuality (AttachHost Equipped (TypeW Creature))
                          (OfLastChosen CardName)
      , SetsChosenQuality (AttachHost Equipped (TypeW Creature))
                          (OfLastChosen (SubtypeQ Creature)) ]) ]

public export
xenograft : Card
xenograft =
  Macros.card "Xenograft" (Just [Macros.generic 4, Macros.pip Blue]) []
       (MkTypeLine [] [Enchantment])
       [ Static (Macros.entersChoosing Macros.thisEnchantment (SubtypeQ Creature))
       , Static (AddsChosenQuality (AllOf (And [Macros.creature, ControlledBy You]))
                                (OfChosen (SubtypeQ Creature))) ]
       Nothing

||| Convincing Mirage -- "Enchant land / As this Aura enters, choose a
||| basic land type. / Enchanted land is the chosen type." The chosen
||| basic land type SET rather than added, on the attach host, and it
||| needed nothing the chosen-quality setting row did not already have
||| once the land host carried the sort. Phantasmal Terrain is the same
||| two rows at a different cost.
public export
convincingMirage : Card
convincingMirage =
  Macros.card "Convincing Mirage" (Just [Macros.generic 1, Macros.pip Blue]) []
       (MkTypeLine [enchantmentType "Aura"] [Enchantment])
       [ Macros.keywordSubject "Enchant" Macros.land
       , Static (Macros.entersChoosingFrom Macros.thisAura (SubtypeQ Land)
                                           BasicTypesOnly)
       , Static (SetsChosenQuality (AttachHost Enchanted (TypeW Land))
                                   (OfChosen (SubtypeQ Land))) ]
       Nothing

||| Realmwright -- "As this creature enters, choose a basic land type.
||| Lands you control are the chosen type in addition to their other
||| types." Xenograft's sentence at the land host, which is the whole of
||| what the subtype sort's host parameter buys: the same two rows, with
||| [CR#305.6]'s basic-only narrowing written as the choice's domain.
public export
realmwright : Card
realmwright =
  Macros.card "Realmwright" (Just [Macros.pip Blue]) []
       (MkTypeLine [creatureType "Vedalken", creatureType "Wizard"] [Creature])
       [ Static (Macros.entersChoosingFrom Macros.thisCreature (SubtypeQ Land)
                                           BasicTypesOnly)
       , Static (AddsChosenQuality (AllOf (And [Macros.land, ControlledBy You]))
                                (OfChosen (SubtypeQ Land))) ]
       (Just (1, 1))

public export
adaptiveAutomaton : Card
adaptiveAutomaton =
  Macros.card "Adaptive Automaton" (Just [Macros.generic 3]) []
       (MkTypeLine [creatureType "Construct"] [Artifact, Creature])
       [ Static (Macros.entersChoosing Macros.thisCreature (SubtypeQ Creature))
       , Static (AddsChosenQuality Macros.thisCreature (OfChosen (SubtypeQ Creature)))
       , Static (Gets (AllOf (And [Macros.creature, ControlledBy You,
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
                          (Continuously
                      (SetsChosenQuality Macros.thisCreature
                                      (OfYourChoice (SubtypeQ Creature) Nothing))
                      (Just Macros.untilEndOfTurn)) ]
       (Just (2, 1))

public export
arcaneAdaptation : Card
arcaneAdaptation =
  Macros.card "Arcane Adaptation" (Just [Macros.generic 2, Macros.pip Blue]) []
       (MkTypeLine [] [Enchantment])
       [ Static (Macros.entersChoosing Macros.thisEnchantment (SubtypeQ Creature))
       , Static (AlsoOffBattlefield
                   (AddsChosenQuality
                      (AllOf (And [Macros.creature, ControlledBy You]))
                      (OfChosen (SubtypeQ Creature)))) ]
       Nothing

public export
conspiracy : Card
conspiracy =
  Macros.card "Conspiracy"
       (Just [Macros.generic 3, Macros.pip Black, Macros.pip Black]) []
       (MkTypeLine [] [Enchantment])
       [ Static (Macros.entersChoosing Macros.thisEnchantment (SubtypeQ Creature))
       , Static (AlsoOffBattlefield
                   (SetsChosenQuality
                      (AllOf (And [Macros.creature, ControlledBy You]))
                      (OfChosen (SubtypeQ Creature)))) ]
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

||| Aisling Leprechaun -- "Whenever this creature blocks or becomes blocked
||| by a creature, that creature becomes green." The literal colour change
||| at the plainest read subject there is: Inferno Elemental's coordinated
||| header, whose two arms announce the one creature the block pairs this
||| creature with, and the tail sets that creature's colour [CR#613.1e].
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

||| Darkest Hour -- "All creatures are black." The whole family in one
||| sentence: a universal subject, one colour, no duration.
public export
darkestHour : Card
darkestHour =
  Macros.card "Darkest Hour" (Just [Macros.pip Black]) []
       (MkTypeLine [] [Enchantment])
       [ Static (SetsColor (AllOf Macros.creature) (SomeColors [Black])) ]
       Nothing

||| Thran Lens -- "All permanents are colorless." The empty colour list is
||| the word "colorless" [CR#105.2c], exactly as it is on a written token.
public export
thranLens : Card
thranLens =
  Macros.card "Thran Lens" (Just [Macros.generic 2]) []
       (MkTypeLine [] [Artifact])
       [ Static (SetsColor (AllOf Permanent) (SomeColors [])) ]
       Nothing

||| Ghostflame Sliver -- "All Slivers are colorless." The same setting at a
||| subtype-named subject.
public export
ghostflameSliver : Card
ghostflameSliver =
  Macros.card "Ghostflame Sliver" (Just [Macros.pip Black, Macros.pip Red]) []
       (MkTypeLine [creatureType "Sliver"] [Creature])
       [ Static (SetsColor (AllOf (HasSubtype (creatureType "Sliver"))) (SomeColors [])) ]
       (Just (2, 2))

||| Sinister Strength -- "Enchanted creature gets +3/+1 and is black." The
||| copular spelling of the same statement, coordinated with a pump on the
||| attach host.
public export
sinisterStrength : Card
sinisterStrength =
  Macros.card "Sinister Strength" (Just [Macros.generic 1, Macros.pip Black]) []
       (MkTypeLine [enchantmentType "Aura"] [Enchantment])
       [ Macros.keywordSubject "Enchant" Macros.creature
       , Static (AndAlso [ Gets (AttachHost Enchanted (TypeW Creature))
                                (PtUp (Lit 3)) (PtUp (Lit 1))
                         , SetsColor It (SomeColors [Black]) ]) ]
       Nothing

||| Crimson Wisps -- "Target creature becomes red and gains haste until end
||| of turn. / Draw a card." The inchoative spelling with a duration, and
||| the coordination with a keyword grant on one target.
public export
crimsonWisps : Card
crimsonWisps =
  Macros.card "Crimson Wisps" (Just [Macros.pip Red]) []
       (MkTypeLine [] [Instant])
       [ Spell (Sequentially
                  [ Continuously
                      (AndAlso [ SetsColor (Macros.target Macros.creature) (SomeColors [Red])
                               , Gains It (Macros.keyword "Haste") ])
                      (Just Macros.untilEndOfTurn)
                  , Macros.drawACard ]) ]
       Nothing

||| Nightcreep -- "Until end of turn, all creatures become black and all
||| lands become Swamps." The colour setting coordinated with a type
||| setting under one duration: two layers [CR#613.1d,613.1e] in one
||| sentence, which is why the colour is its own row and not an empty type
||| line on the second.
public export
nightcreep : Card
nightcreep =
  Macros.card "Nightcreep" (Just [Macros.pip Black, Macros.pip Black]) []
       (MkTypeLine [] [Instant])
       [ Spell (Continuously
                  (AndAlso [ SetsColor (AllOf Macros.creature) (SomeColors [Black])
                           , SetsType (AllOf Macros.land)
                                      (MkToken Nothing []
                                               (Macros.basicLandLine [landType "Swamp"])
                                               [] Nothing)
                                      Nothing ])
                  (Just Macros.untilEndOfTurn)) ]
       Nothing

||| Celestial Dawn's two ascription lines -- "Lands you control are Plains.
||| / Nonland permanents you control are white. The same is true for spells
||| you control and nonland cards you own that aren't on the battlefield."
||| The literal colour setting under the off-battlefield extension, which
||| is what the extension was waiting for. The whole card additionally
||| wants the two mana-spending permissions, which `PlayAsThough` does not
||| carry.
public export
celestialDawnAscriptions : AbilitySeq []
celestialDawnAscriptions =
  [ Static (SetsType (AllOf (And [Macros.land, ControlledBy You]))
                     (MkToken Nothing [] (Macros.basicLandLine [landType "Plains"])
                              [] Nothing)
                     Nothing)
  , Static (AlsoOffBattlefield
              (SetsColor (AllOf (And [Permanent, Not (HasType Land), ControlledBy You]))
                         (SomeColors [White]))) ]

||| Ghoulflesh -- "Enchant creature / Enchanted creature gets -1/-1 and is
||| a black Zombie in addition to its other colors and types." The whole
||| card, and the plainest of the 19 lines that coordinate a P/T statement
||| with the addition in one sentence.
public export
ghoulflesh : Card
ghoulflesh =
  Macros.card "Ghoulflesh" (Just [Macros.pip Black]) []
       (MkTypeLine [enchantmentType "Aura"] [Enchantment])
       [ Macros.keywordSubject "Enchant" Macros.creature
       , Static (AndAlso [ Gets (AttachHost Enchanted (TypeW Creature))
                                (PtDown (Lit 1)) (PtDown (Lit 1))
                         , BecomesAlso It
                                       (MkToken Nothing [Black]
                                                (MkTypeLine [creatureType "Zombie"] [])
                                                [] Nothing) ]) ]
       Nothing

||| Blade of the Oni's static line -- "Equipped creature has base power and
||| toughness 5/5, has menace, and is a black Demon in addition to its
||| other colors and types." The BASE P/T rider coordinated with the
||| addition in one sentence, which the ledger carried as unspelled and
||| which `AndAlso` already writes: the same shape Darksteel Mutation
||| writes at the setting. No new construction. The whole card wants
||| reconfigure.
public export
bladeOfTheOniStatic : StaticEffect []
bladeOfTheOniStatic =
  AndAlso [ HasBasePt (AttachHost Equipped (TypeW Creature)) (Lit 5) (Lit 5)
          , Gains It (Macros.keyword "Menace")
          , BecomesAlso It
                        (MkToken Nothing [Black]
                                 (MkTypeLine [creatureType "Demon"] []) [] Nothing) ]

||| Ersatz Gnomes -- "{T}: Target spell becomes colorless. / {T}: Target
||| permanent becomes colorless until end of turn." The whole card, and
||| the reason the colour row carries no subject-zone demand: its first
||| ability sets the colour of an object on the STACK.
public export
ersatzGnomes : Card
ersatzGnomes =
  Macros.card "Ersatz Gnomes" (Just [Macros.generic 3]) []
       (MkTypeLine [creatureType "Gnome"] [Artifact, Creature])
       [ Macros.activated TapSymbol
           (Continuously (SetsColor (Macros.target Macros.spell) (SomeColors []))
                         Nothing)
       , Macros.activated TapSymbol
           (Macros.becomesColor (Macros.target Permanent) (SomeColors [])
                                (Just Macros.untilEndOfTurn)) ]
       (Just (1, 1))

||| Missy's first trigger -- "Whenever another nonartifact creature dies,
||| return it to the battlefield under your control face down and tapped.
||| It's a 2/2 Cyberman artifact creature." TWO arrival riders in one
||| sentence, one of them the face-down word, which is the whole of what
||| indexing the rider vocabulary over `StatusVal` buys. The `Cyberman`
||| word costs nothing: subtypes are labels. The whole card wants the
||| villainous choice.
public export
missyFaceDownReturn : Ability
missyFaceDownReturn =
  Macros.triggered Whenever
    (Dies (Macros.a (And [Macros.creature, Not (HasType Artifact),
                          OtherThan Macros.thisCreature])))
    (Sequentially
       [ Move It Macros.battlefieldZ
              (MkMoveRiders [EntersAs FaceDown, EntersTapped] (Just You) Nothing)
       , Continuously
           (SetsType It
                     (MkToken (Just (Lit 2 ** Lit 2)) []
                              (MkTypeLine [creatureType "Cyberman"] [Artifact, Creature])
                              [] Nothing)
                     Nothing)
           Nothing ])

||| Transguild Courier -- "Transguild Courier is all colors." The colour
||| SPACE's quantifier at the same position the enumerated list sits, and
||| the whole card. The word is not a five-way list for `AddsEveryType`'s
||| own reason: "all colors" is one printed quantifier [CR#105.1].
public export
transguildCourier : Card
transguildCourier =
  Macros.card "Transguild Courier" (Just [Macros.generic 4]) []
       (MkTypeLine [creatureType "Golem"] [Artifact, Creature])
       [ Static (SetsColor Macros.thisCreature EveryColor) ]
       (Just (3, 3))

||| Scrapbasket -- "{1}: This creature becomes all colors until end of
||| turn." The same quantifier with a duration, and the whole card.
public export
scrapbasket : Card
scrapbasket =
  Macros.card "Scrapbasket" (Just [Macros.generic 4]) []
       (MkTypeLine [creatureType "Scarecrow"] [Artifact, Creature])
       [ Macros.activated (Mana [Macros.generic 1])
                          (Macros.becomesColor Macros.thisCreature EveryColor
                                               (Just Macros.untilEndOfTurn)) ]
       (Just (3, 2))

||| Indigo Faerie -- "{U}: Target permanent becomes blue in addition to its
||| other colors until end of turn." The colour-only ADDITION, which is one
||| printed line and so buys no row of its own: `BecomesAlso`'s bundle
||| carries the colour and the bundle-level gate is what lets its type line
||| be absent. The other line of the shape is Painter's Servant's chosen
||| colour, which `AddsChosenQuality` already writes.
public export
indigoFaerie : Card
indigoFaerie =
  Macros.card "Indigo Faerie" (Just [Macros.generic 1, Macros.pip Blue]) []
       (MkTypeLine [creatureType "Faerie", creatureType "Wizard"] [Creature])
       [ Macros.keyword "Flying"
       , Macros.activated (Mana [Macros.pip Blue])
                          (Macros.becomesAs (Macros.target Permanent)
                                            (MkToken Nothing [Blue] (MkTypeLine [] [])
                                                     [] Nothing)
                                            (Just Macros.untilEndOfTurn)) ]
       (Just (1, 1))

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
              , Continuously (SetsChosenQuality (Macros.target Macros.creature)
                                                (OfChosen (SubtypeQ Creature)))
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
              , Continuously (SetsChosenQuality (Macros.target Macros.creature)
                                                (OfChosen (SubtypeQ Creature)))
                             (Just Macros.untilEndOfTurn) ]) ]
       Nothing

public export
standardize : Card
standardize =
  Macros.card "Standardize" (Just [Macros.pip Blue, Macros.pip Blue]) []
       (MkTypeLine [] [Instant])
       [ Spell (Sequentially
              [ Macros.choose (Macros.a (Macros.qualityFrom (SubtypeQ Creature) (TypeOtherThan (creatureType "Wall"))))
              , Continuously (SetsChosenQuality (Each Macros.creature)
                                                (OfChosen (SubtypeQ Creature)))
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
           (Casts (Macros.a AnyPlayer) (Macros.a Macros.spell) Nothing)
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
       , Static (Macros.objectCant "Cast"
                   (AllOf (And [Macros.spell, Named ChosenName]))) ]
       (Just (2, 2))

||| Sanctum Prelate -- "As this creature enters, choose a number. /
||| Noncreature spells with mana value equal to the chosen number can't
||| be cast." The chosen-number amount's witness: the number sort had a
||| chooser and no read at all until the amount seat opened, and this is
||| the read at its plainest -- one chooser, one comparison bound, no
||| domain written.
public export
sanctumPrelate : Card
sanctumPrelate =
  Macros.card "Sanctum Prelate"
       (Just [Macros.generic 1, Macros.pip White, Macros.pip White]) []
       (MkTypeLine [creatureType "Human", creatureType "Cleric"] [Creature])
       [ Static (Macros.entersChoosing Macros.thisCreature Number)
       , Static (Macros.objectCant "Cast"
                   (AllOf (And [Macros.spell, Not (HasType Creature),
                                Compare [CharAxis ManaValue] Eq ChosenNumber]))) ]
       (Just (2, 2))

||| Nyxathid -- "As this creature enters, choose an opponent. / This
||| creature gets -1/-1 for each card in the chosen player's hand."
||| The player sort's chooser and read in one card: the as-enters row
||| carried the choice all along, and what was missing was a sort that
||| reached the player kind and a phrase to read it back with.
public export
nyxathid : Card
nyxathid =
  Macros.card "Nyxathid"
       (Just [Macros.generic 1, Macros.pip Black, Macros.pip Black]) []
       (MkTypeLine [creatureType "Elemental"] [Creature])
       [ Static (Macros.entersChoosingPlayer Macros.thisCreature
                                             (Just OpponentsOnly))
       , Static (Gets Macros.thisCreature
                      (PtDown (CountOf (InZone (Macros.handOf
                                 (Definite ChosenPlayer)))))
                      (PtDown (CountOf (InZone (Macros.handOf
                                 (Definite ChosenPlayer)))))) ]
       (Just (7, 7))

||| Stuffy Doll -- the bare "choose a player" beside Nyxathid's narrowed
||| "choose an opponent", with Spitemare's damage-back trigger aimed at
||| the chosen player instead of at a target.
public export
stuffyDoll : Card
stuffyDoll =
  Macros.card "Stuffy Doll" (Just [Macros.generic 5]) []
       (MkTypeLine [creatureType "Construct"] [Artifact, Creature])
       [ Macros.keyword "Indestructible"
       , Static (Macros.entersChoosingPlayer Macros.thisCreature Nothing)
       , Macros.triggered Whenever
                          (IsDealtDamage AnyDamage Macros.thisCreature)
                          (DealDamage It ThatMuch (Definite ChosenPlayer))
       , Macros.activated TapSymbol
                          (DealDamage Macros.thisCreature (Lit 1)
                                      Macros.thisCreature) ]
       (Just (0, 1))

||| Expel the Interlopers -- "Choose a number between 0 and 10. Destroy
||| all creatures with power greater than or equal to the chosen
||| number." The chosen number read as a comparison BOUND, and the
||| printed range at the number sort's second domain, in one spell.
public export
expelTheInterlopers : Card
expelTheInterlopers =
  Macros.card "Expel the Interlopers"
       (Just [Macros.generic 3, Macros.pip White, Macros.pip White]) []
       (MkTypeLine [] [Sorcery])
       [ Spell (Sequentially
           [ Macros.choose (Macros.a (Macros.qualityFrom Number
                                        (NumberBetween 0 10)))
           , Macros.destroy (AllOf (And [Macros.creature,
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
                   (AllOf (And [Macros.spell, Named ChosenName]))) ]
       Nothing

||| Council of the Absolute, whole -- "As this creature enters, choose a
||| noncreature, nonland card name. / Your opponents can't cast spells
||| with the chosen name. / Spells with the chosen name you cast cost {2}
||| less to cast." The QUALIFIED cast prohibition in the PLAYER voice, one
||| card away from Meddling Mage's object voice of the same sentence. The
||| complement is `DeonticCounterpart`, read through `counterRole` --
||| [CR#601.2] seats a cast spell on the stack, which is where the deed's
||| own row already puts a `Cast` patient, so the description the printed
||| line carries needed no slot of its own.
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
                   (AllOf (And [Macros.spell, Named ChosenName])))
       , Static (CostsToCast (AllOf (And [Macros.spell, Named ChosenName,
                                          CastBy You]))
                             (CostLess (Lit 2) Nothing)) ]
       (Just (2, 3))

||| Failure // Comply's second face -- "Choose a card name. Until your
||| next turn, your opponents can't cast spells with the chosen name."
||| Conjurer's Ban's sentence in the other voice, and the pair is the
||| argument for one deed at two roles: the choice announces the name
||| once and each voice reads it from the same binding.
public export
complyNameLock : Effect []
complyNameLock =
  Sequentially
    [ Macros.choose (Macros.a (Macros.quality CardName))
    , Continuously
        (Macros.cantDoTo "Cast" (PlayerGroup YourOpponents)
           (AllOf (And [Macros.spell, Named ChosenName])))
        (Just Macros.untilYourNextTurn) ]

||| Gideon's Intervention, whole -- the qualified cast prohibition with
||| the chosen name read a second time by a damage prevention, which is
||| what makes the card one card and not two lines that happen to share a
||| word: [CR#109.2c] reads "sources with the chosen name" as the objects
||| themselves, so both readers take the same binding.
public export
gideonsIntervention : Card
gideonsIntervention =
  Macros.card "Gideon's Intervention"
       (Just [Macros.generic 2, Macros.pip White, Macros.pip White]) []
       (MkTypeLine [] [Enchantment])
       [ Static (Macros.entersChoosing Macros.thisEnchantment CardName)
       , Static (Macros.cantDoTo "Cast" (PlayerGroup YourOpponents)
                   (AllOf (And [Macros.spell, Named ChosenName])))
       , Static (Prevents AnyDamage AllOfIt
                   (ToRecipient (Macros.youAnd
                      (AllOf (And [Permanent, ControlledBy You]))))
                   (Just (AllOf (And [IsSource, Named ChosenName])))
                   Nothing) ]
       Nothing

||| Academic Probation's first mode -- "Choose a nonland card name.
||| Opponents can't cast spells with the chosen name until your next
||| turn." The same sentence as Comply's with the domain written out.
public export
academicProbationNameMode : Effect []
academicProbationNameMode =
  Sequentially
    [ Macros.choose (Macros.a (Macros.qualityFrom CardName
                                 (NameOfCard (Not (HasType Land)))))
    , Continuously
        (Macros.cantDoTo "Cast" (PlayerGroup YourOpponents)
           (AllOf (And [Macros.spell, Named ChosenName])))
        (Just Macros.untilYourNextTurn) ]

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
                     (CountedGroup Macros.anyNumber Nothing
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
                     (CountedGroup (Macros.exactly 2) Nothing
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
       [ Macros.triggeredIf When (Enters Macros.thisCreature Nothing)
           (ExistsGroup (Macros.withTheSameName
                           (CountedGroup (Macros.atLeast 2) Nothing
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
                                   (CountedGroup (Macros.atLeast 3) Nothing
                                                 (And [Macros.land,
                                                       ControlledBy You])))) ]
       Nothing

||| Saheeli Rai's +1. The -2 is `saheelisCopy`; the card stays off the
||| bench for its last one, whose counted search has no mention to carry a
||| count.
public export
saheeliRaiPlusOne : Effect []
saheeliRaiPlusOne =
  Sequentially [ Macros.scryOne
               , DealDamage This (Lit 1) (Each Opponent) ]

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
           , Continuously
               (AndAlso [ Macros.objectCant "Cast"
                            (AllOf (And [Macros.spell, Named ChosenName]))
                        , Macros.objectCant "Play"
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
                                      Compare [CharAxis Power] AtLeast (Lit 4)])))
       , Macros.triggered When (ChapterMark [ChapterII, ChapterIII])
           (PutCounters (Lit 1) (PrintedKind Macros.plusOnePlusOne)
                        (Each Macros.creatureYouControl)) ]
       Nothing

public export
keldonWarcaller : Card
keldonWarcaller =
  Macros.card "Keldon Warcaller"
       (Just [Macros.generic 1, Macros.pip Red]) []
       (MkTypeLine [creatureType "Human", creatureType "Warrior"] [Creature])
       [ Macros.triggered Whenever (Macros.attacks Macros.thisCreature)
           (PutCounters (Lit 1) (PrintedKind Lore)
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
       [ Static (Macros.mayCastAsThough You (AllOf Macros.spell)) ]
       Nothing

public export
shimmerMyr : Card
shimmerMyr =
  Macros.card "Shimmer Myr" (Just [Macros.generic 3]) []
       (MkTypeLine [creatureType "Myr"] [Artifact, Creature])
       [ Macros.keyword "Flash"
       , Static (Macros.mayCastAsThough You
                                        (AllOf (And [Macros.spell, HasType Artifact]))) ]
       (Just (2, 2))

public export
quickSliver : Card
quickSliver =
  Macros.card "Quick Sliver" (Just [Macros.generic 1, Macros.pip Green]) []
       (MkTypeLine [creatureType "Sliver"] [Creature])
       [ Macros.keyword "Flash"
       , Static (Macros.mayCastAsThough (Macros.a AnyPlayer)
                                        (AllOf (And [Macros.spell, HasSubtype (creatureType "Sliver")]))) ]
       (Just (1, 1))

public export
vernalEquinox : Card
vernalEquinox =
  Macros.card "Vernal Equinox"
       (Just [Macros.generic 3, Macros.pip Green]) []
       (MkTypeLine [] [Enchantment])
       [ Static (Macros.mayCastAsThough (Macros.a AnyPlayer)
                                        (AllOf (And [Macros.spell,
                                                     Or [HasType Creature, HasType Enchantment]]))) ]
       Nothing

public export
borneUponAWind : Card
borneUponAWind =
  Macros.card "Borne Upon a Wind" (Just [Macros.generic 1, Macros.pip Blue]) []
       (MkTypeLine [] [Instant])
       [ Spell (Continuously
                  (Macros.mayCastAsThough You (AllOf Macros.spell))
                  (Just Macros.thisTurn))
       , Spell Macros.drawACard ]
       Nothing

public export
gisaAndGeralf : Card
gisaAndGeralf =
  Macros.card "Gisa and Geralf"
       (Just [Macros.generic 2, Macros.pip Blue, Macros.pip Black]) [Legendary]
       (MkTypeLine [creatureType "Human", creatureType "Wizard"] [Creature])
       [ Macros.triggered When (Enters Macros.thisCreature Nothing)
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
                                            Compare [CharAxis Power] AtMost (Lit 2)]))
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
           (Enters (Macros.a (And [Macros.land, ControlledBy You])) Nothing)
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
       , Macros.triggered When (Enters Macros.thisAura Nothing) Macros.drawACard
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


||| Amoeboid Changeling's two type abilities -- "{T}: Target creature gains
||| all creature types until end of turn. / {T}: Target creature loses all
||| creature types until end of turn." The quantifier's two poles printed
||| on one card, which is the minimal pair `LosesEveryType` is paid for by.
||| The whole card wants the Changeling keyword, which the catalog does not
||| carry.
public export
amoeboidChangelingTypeAbilities : AbilitySeq []
amoeboidChangelingTypeAbilities =
  [ Macros.activated TapSymbol
      (Continuously (AddsEveryType (Macros.target Macros.creature) CreatureSpace)
                    (Just Macros.untilEndOfTurn))
  , Macros.activated TapSymbol
      (Continuously (LosesEveryType (Macros.target Macros.creature) CreatureSpace)
                    (Just Macros.untilEndOfTurn)) ]

||| Nameless Inversion's body -- "Target creature gets +3/-3 and loses all
||| creature types until end of turn." The loss coordinated with a pump on
||| one target. Its whole card wants the Changeling keyword.
public export
namelessInversionBody : Effect []
namelessInversionBody =
  Continuously (AndAlso [ Gets (Macros.target Macros.creature)
                               (PtUp (Lit 3)) (PtDown (Lit 3))
                        , LosesEveryType It CreatureSpace ])
               (Just Macros.untilEndOfTurn)

||| Ego Erasure's body -- "Creatures target player controls get -2/-0 and
||| lose all creature types until end of turn." The same coordination at a
||| controlled group. Its whole card wants the Changeling keyword.
public export
egoErasureBody : Effect []
egoErasureBody =
  Continuously (AndAlso [ Gets (AllOf (And [Macros.creature,
                                            ControlledBy (Macros.target AnyPlayer)]))
                               (PtDown (Lit 2)) (PtUp (Lit 0))
                        , LosesEveryType Them CreatureSpace ])
               (Just Macros.untilEndOfTurn)

||| Curse of Conformity -- "Enchant player / Nonlegendary creatures
||| enchanted player controls have base power and toughness 3/3 and lose
||| all creature types." The standing (undurated) loss, at the enchanted
||| player's creatures.
public export
curseOfConformity : Card
curseOfConformity =
  Macros.card "Curse of Conformity" (Just [Macros.generic 4, Macros.pip White]) []
       (MkTypeLine [enchantmentType "Aura", enchantmentType "Curse"] [Enchantment])
       [ Macros.keywordSubject "Enchant" AnyPlayer
       , Static (AndAlso
           [ HasBasePt (AllOf (And [Macros.creature, Not (HasSupertype Legendary),
                                    ControlledBy (AttachHost Enchanted PlayerW)]))
                       (Lit 3) (Lit 3)
           , LosesEveryType Them CreatureSpace ]) ]
       Nothing

||| Lithoform Blight's third line, in part -- "Enchanted land loses all land
||| types and abilities …". The type-loss/ability-loss coordination decided
||| ONE way: two statements under `AndAlso`, not one row with an ability
||| rider, because [CR#613.1d] applies the type loss at layer 4 and
||| [CR#613.1f] the ability loss at layer 6. All three printed land-type
||| loss lines coordinate this way; none writes the type loss alone. The
||| rest of the line grants two mana abilities and is not this cell's.
public export
lithoformBlightLoss : StaticEffect []
lithoformBlightLoss =
  AndAlso [ LosesEveryType (AttachHost Enchanted (TypeW Land)) LandSpace
          , LosesAllAbilities It Nothing ]

||| Energybending -- "Lands you control gain all basic land types until end
||| of turn. / Draw a card." The basic-land grant cell, which needed only
||| the Lesson spell type [CR#205.3k]; subtypes are labels, so the word cost
||| nothing to write.
public export
energybending : Card
energybending =
  Macros.card "Energybending" (Just [Macros.generic 2]) []
       (MkTypeLine [spellType "Lesson"] [Instant])
       [ Spell (Sequentially
                  [ Continuously
                      (AddsEveryType (AllOf (And [Macros.land, ControlledBy You]))
                                     BasicLandSpace)
                      (Just Macros.untilEndOfTurn)
                  , Macros.drawACard ]) ]
       Nothing

||| Ashes of the Fallen -- "As this artifact enters, choose a creature type.
||| / Each creature card in your graveyard has the chosen creature type in
||| addition to its other types." The chosen-quality ascription at a
||| GRAVEYARD subject, which is why the two chosen-quality rows carry no
||| subject-zone demand: the layers apply to an object's characteristics
||| [CR#613.1] and a card in a graveyard is an object [CR#109.1].
public export
ashesOfTheFallen : Card
ashesOfTheFallen =
  Macros.card "Ashes of the Fallen" (Just [Macros.generic 2]) []
       (MkTypeLine [] [Artifact])
       [ Static (Macros.entersChoosing Macros.thisArtifact (SubtypeQ Creature))
       , Static (AddsChosenQuality
                   (Each (And [Macros.creature, InZone (Macros.graveyardOf You)]))
                   (OfChosen (SubtypeQ Creature))) ]
       Nothing

||| Yedora, Grave Gardener -- "Whenever another nontoken creature you
||| control dies, you may return it to the battlefield face down under its
||| owner's control. It's a Forest land." The face-down ARRIVAL rider, the
||| cheapest of the seven lines that place their object face down before
||| ascribing to it. [CR#708.3] turns the object face down before it
||| enters, so the word is a rider on the arrival; [CR#708.2a]'s "unless
||| otherwise specified" is what licenses the type line the next sentence
||| writes.
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
                        (MkMoveRiders [EntersAs FaceDown] (Just (OwnerOf It)) Nothing)
                 , Continuously
                     (SetsType It
                               (MkToken Nothing []
                                        (MkTypeLine [landType "Forest"] [Land])
                                        [] Nothing)
                               Nothing)
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
                                   (CountedGroup (Macros.exactly 5) Nothing Macros.artifact))])
                         (ExtraTurn You (Lit 1))] Nothing

public export
magistratesScepter : Card
magistratesScepter =
  Macros.card "Magistrate's Scepter" (Just [Macros.generic 3]) []
       (MkTypeLine [] [Artifact])
       [ Macros.activated (Compound [Mana [Macros.generic 4], TapSymbol])
                          (PutCounters (Lit 1) (PrintedKind Charge) Macros.thisArtifact)
       , Macros.activated (Compound [TapSymbol,
                              Do (RemoveCounters (Macros.exactly 3) (Just Charge) Macros.thisArtifact)])
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
           (Macros.mayElse You (Pay You (Mana [Macros.pip Blue]) PaidOnce)
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
              (Pay You (Mana [Macros.generic 5, Macros.pip Red, Macros.pip Red]) PaidOnce)
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
                              Do (RemoveCounters (Macros.exactly 1) (Just Macros.plusOnePlusOne)
                                   (Macros.a (And [Macros.creature, ControlledBy You])))])
                          Macros.drawACard ]
       (Just (2, 2))

public export
metallicMimic : Card
metallicMimic =
  Macros.card "Metallic Mimic" (Just [Macros.generic 2]) []
       (MkTypeLine [creatureType "Shapeshifter"] [Artifact, Creature])
       [ Static (Macros.entersChoosing Macros.thisCreature (SubtypeQ Creature))
       , Static (AddsChosenQuality Macros.thisCreature (OfChosen (SubtypeQ Creature)))
       , Static (Macros.entersWithAdditionalCounters (Each (And [Macros.creature, ControlledBy You,
                                                                 OfChosen (SubtypeQ Creature),
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
       , Static (Macros.objectCant "Activate"
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
       , Static (Macros.objectCant "Cast"
                   (AllOf (And [Macros.spell, Named ChosenName])))
       , Static (Macros.objectCant "Activate"
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
       , Static (Macros.objectCant "Activate"
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
       [ Static (Macros.objectCant "Activate"
                   (AllOf (And [ AbilityHead AnyActivated
                               , AbilityOf (AllOf Macros.creature) ]))) ]
       Nothing

public export
nullRod : Card
nullRod =
  Macros.card "Null Rod" (Just [Macros.generic 2]) []
       (MkTypeLine [] [Artifact])
       [ Static (Macros.objectCant "Activate"
                   (AllOf (And [ AbilityHead AnyActivated
                               , AbilityOf (AllOf Macros.artifact) ]))) ]
       Nothing

public export
stonySilence : Card
stonySilence =
  Macros.card "Stony Silence" (Just [Macros.generic 1, Macros.pip White]) []
       (MkTypeLine [] [Enchantment])
       [ Static (Macros.objectCant "Activate"
                   (AllOf (And [ AbilityHead AnyActivated
                               , AbilityOf (AllOf Macros.artifact) ]))) ]
       Nothing

public export
clarionConqueror : Card
clarionConqueror =
  Macros.card "Clarion Conqueror" (Just [Macros.generic 2, Macros.pip White]) []
       (MkTypeLine [creatureType "Dragon"] [Creature])
       [ Macros.keyword "Flying"
       , Static (Macros.objectCant "Activate"
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
       [ Static (Macros.objectCant "Activate"
                   (AllOf (And [ AbilityHead AnyActivated
                               , AbilityOf (AllOf (Or [Macros.artifact,
                                                       Macros.creature]))
                               , Not IsManaAbility ]))) ]
       Nothing

||| Fluctuator, whole -- "Cycling abilities you activate cost {2} less to
||| activate." The ability-CLASS subject named by a KEYWORD, which is the
||| cell the five catalog rows were owed for: `KeywordClass` reads the
||| word and the row is what makes the word known. [CR#702.29a] makes
||| cycling an activated ability, so the class word names a class of
||| abilities and not a class of objects.
public export
fluctuator : Card
fluctuator =
  Macros.card "Fluctuator" (Just [Macros.generic 2]) []
       (MkTypeLine [] [Artifact])
       [ Static (CostsToCast
                   (AllOf (And [AbilityHead (KeywordClass "Cycling"),
                                ActivatedBy You]))
                   (CostLess (Lit 2) Nothing)) ]
       Nothing

||| Boom Scholar's first line -- "Exhaust abilities of other permanents
||| you control cost {2} less to activate." The same cell at a word whose
||| row this round added: [CR#702.177a] makes exhaust a keyword adding
||| rules to the activated ability that follows it, so what the line
||| narrows is that ability's class and the possessor rides `AbilityOf`.
public export
boomScholarExhaustDiscount : Ability
boomScholarExhaustDiscount =
  Static (CostsToCast
            (AllOf (And [ AbilityHead (KeywordClass "Exhaust")
                        , AbilityOf (AllOf (And [Permanent,
                                                 OtherThan Macros.thisCreature,
                                                 ControlledBy You])) ]))
            (CostLess (Lit 2) Nothing))

||| Hulk, Gamma Goliath's first line -- "Power-up abilities of other
||| creatures you control cost {3} less to activate" [CR#702.193a].
public export
hulkPowerUpDiscount : Ability
hulkPowerUpDiscount =
  Static (CostsToCast
            (AllOf (And [ AbilityHead (KeywordClass "PowerUp")
                        , AbilityOf (AllOf (And [Macros.creature,
                                                 OtherThan Macros.thisCreature,
                                                 ControlledBy You])) ]))
            (CostLess (Lit 3) Nothing))

||| Kang the Conqueror's power-up rider -- "power-up abilities can't be
||| activated." The ability-class PROHIBITION at a keyword-named class,
||| which is the same row Pithing Needle writes at `AnyActivated`.
public export
kangPowerUpLock : StaticEffect []
kangPowerUpLock =
  Macros.objectCant "Activate"
    (AllOf (AbilityHead (KeywordClass "PowerUp")))

||| Grand Abolisher, whole -- "During your turn, your opponents can't
||| cast spells or activate abilities of artifacts, creatures, or
||| enchantments." The WINDOW and the qualified complement in one line,
||| and the reason the complement is kind-general: casting takes an
||| object [CR#601.2] and activating takes an ability [CR#602.2,109.1],
||| so a coordination of the two names participants at two kinds and the
||| slot cannot be `Object`-only. Two statements under one window rather
||| than one statement over two labels, because each deed carries its own
||| complement and the carrier holds one.
public export
grandAbolisher : Card
grandAbolisher =
  Macros.card "Grand Abolisher" (Just [Macros.pip White, Macros.pip White]) []
       (MkTypeLine [creatureType "Human", creatureType "Cleric"] [Creature])
       [ Static (OnlyDuring Turn (Just Yours)
                   (AndAlso
                      [ Macros.cantDoTo "Cast" (PlayerGroup YourOpponents)
                          (AllOf Macros.spell)
                      , Macros.cantDoTo "Activate" (PlayerGroup YourOpponents)
                          (AllOf (And [ AbilityHead AnyActivated
                                      , AbilityOf (AllOf (Or [Macros.artifact,
                                                              Macros.creature,
                                                              HasType Enchantment])) ])) ])) ]
       (Just (2, 2))

||| Festival, whole -- "Cast this spell only during an opponent's upkeep.
||| / Creatures can't attack this turn." The CAST WINDOW on a spell card,
||| which [CR#113.6e] functions from the zones the spell could be cast
||| from and from the stack; 47 supported lines write one. The routed
||| gap read it as a missing `Timing` seat, and the answer was the
||| window a static statement is confined to rather than a second
||| activation restriction.
public export
festival : Card
festival =
  Macros.card "Festival" (Just [Macros.pip White]) []
       (MkTypeLine [] [Instant])
       [ Static (OnlyDuring Upkeep (Just AnOpponents)
                   (Macros.deontic This Permit ["Cast"] Patient NoDeonticPatient))
       , Spell (Macros.cantAttack (AllOf Macros.creature)
                                  (Just Macros.thisTurn)) ]
       Nothing

||| Kopala, Warden of Waves, whole -- "Spells your opponents cast that
||| target a Merfolk you control cost {2} more to cast. / Abilities your
||| opponents activate that target a Merfolk you control cost {2} more to
||| activate." The "that target ..." RESTRICTOR on an ability class, and
||| the second line is the first one's shape at the other kind:
||| [CR#115.9b] states the relation in the rules' own words for a spell
||| and an ability alike, so one predicate answers both and the class
||| word is what changes.
public export
kopalaWardenOfWaves : Card
kopalaWardenOfWaves =
  Macros.card "Kopala, Warden of Waves"
       (Just [Macros.generic 1, Macros.pip Blue, Macros.pip Blue])
       [Legendary]
       (MkTypeLine [creatureType "Merfolk", creatureType "Wizard"] [Creature])
       [ Static (CostsToCast
                   (AllOf (And [ Macros.spell
                               , CastBy (PlayerGroup YourOpponents)
                               , Targets (Macros.a (And [Macros.creature,
                                                         HasSubtype (creatureType "Merfolk"),
                                                         ControlledBy You]))
                                         SomeTarget ]))
                   (CostMore (Lit 2)))
       , Static (CostsToCast
                   (AllOf (And [ AbilityHead AnyActivated
                               , ActivatedBy (PlayerGroup YourOpponents)
                               , Targets (Macros.a (And [Macros.creature,
                                                         HasSubtype (creatureType "Merfolk"),
                                                         ControlledBy You]))
                                         SomeTarget ]))
                   (CostMore (Lit 2))) ]
       (Just (2, 2))

||| Tithe Taker, whole -- "During your turn, spells your opponents cast
||| cost {1} more to cast and abilities your opponents activate cost {1}
||| more to activate unless they're mana abilities. / Afterlife 1." The
||| WINDOW over a coordination of two cost statements at two sorts, which
||| is the card's own difficulty: the window is not a duration and not a
||| condition, and each conjunct describes a different kind of object.
public export
titheTaker : Card
titheTaker =
  Macros.card "Tithe Taker" (Just [Macros.generic 1, Macros.pip White]) []
       (MkTypeLine [creatureType "Human", creatureType "Soldier"] [Creature])
       [ Static (OnlyDuring Turn (Just Yours)
                   (AndAlso
                      [ CostsToCast
                          (AllOf (And [Macros.spell,
                                       CastBy (PlayerGroup YourOpponents)]))
                          (CostMore (Lit 1))
                      , CostsToCast
                          (AllOf (And [ AbilityHead AnyActivated
                                      , ActivatedBy (PlayerGroup YourOpponents)
                                      , Not IsManaAbility ]))
                          (CostMore (Lit 1)) ]))
       , Macros.keywordNumber "Afterlife" (Lit 1) ]
       (Just (2, 1))

||| Gaddock Teeg, whole -- "Noncreature spells with mana value 4 or
||| greater can't be cast. / Noncreature spells with {X} in their mana
||| costs can't be cast." Two qualified cast prohibitions in the object
||| voice whose complements differ in KIND of question: the first reads a
||| derived characteristic [CR#202.3] and the second reads a printed
||| symbol [CR#202.1], which is why the second wanted a predicate of its
||| own rather than another comparison.
public export
gaddockTeeg : Card
gaddockTeeg =
  Macros.card "Gaddock Teeg" (Just [Macros.pip Green, Macros.pip White])
       [Legendary]
       (MkTypeLine [creatureType "Kithkin", creatureType "Advisor"] [Creature])
       [ Static (Macros.objectCant "Cast"
                   (AllOf (And [Macros.spell, Not (HasType Creature),
                                Compare [CharAxis ManaValue] AtLeast (Lit 4)])))
       , Static (Macros.objectCant "Cast"
                   (AllOf (And [Macros.spell, Not (HasType Creature),
                                ManaCostHasX]))) ]
       (Just (2, 2))

||| Vexing Shusher, whole -- "This spell can't be countered. / {R/G}:
||| Target spell can't be countered." The SPANLESS prohibition, and the
||| finding is that nothing refuses it: `SpanOk` admits an unstated
||| duration at every `StaticKind` ([CR#611.2a] gives an unstated
||| duration the end of the game), so the second line wanted only the
||| targeted spell's own noun. The first line elaborated already; the
||| pair is one card because [CR#113.6g] functions both on the stack.
public export
vexingShusher : Card
vexingShusher =
  Macros.card "Vexing Shusher"
       (Just [Macros.hybridPip Red Green, Macros.hybridPip Red Green]) []
       (MkTypeLine [creatureType "Goblin", creatureType "Shaman"] [Creature])
       [ Static (Macros.objectCant "Counter" This)
       , Macros.activated (Mana [Macros.hybridPip Red Green])
           (Continuously
              (Macros.objectCant "Counter" (Macros.target Macros.spell))
              Nothing) ]
       (Just (2, 2))

||| Training Grounds, whole -- "Activated abilities of creatures you
||| control cost {2} less to activate. This effect can't reduce the mana
||| in that cost to less than one mana." The cost reduction's printed
||| FLOOR, a slot on the reduction rather than a prohibition: the second
||| sentence forbids no agent anything, it bounds the first sentence's
||| own amount, and it states a bound [CR#601.2f] does not -- the rules
||| stop a total cost at {0} and this line stops it at one mana.
public export
trainingGrounds : Card
trainingGrounds =
  Macros.card "Training Grounds" (Just [Macros.pip Blue]) []
       (MkTypeLine [] [Enchantment])
       [ Static (CostsToCast
                   (AllOf (And [ AbilityHead AnyActivated
                               , AbilityOf (AllOf Macros.creatureYouControl) ]))
                   (CostLess (Lit 2) (Just (Lit 1)))) ]
       Nothing

||| Power Artifact, whole -- the same floor over the Aura's host.
public export
powerArtifact : Card
powerArtifact =
  Macros.card "Power Artifact" (Just [Macros.pip Blue, Macros.pip Blue]) []
       (MkTypeLine [enchantmentType "Aura"] [Enchantment])
       [ Macros.keywordSubject "Enchant" Macros.artifact
       , Static (CostsToCast
                   (AllOf (And [ AbilityHead AnyActivated
                               , AbilityOf (AttachHost Enchanted (TypeW Artifact)) ]))
                   (CostLess (Lit 2) (Just (Lit 1)))) ]
       Nothing

||| Fervent Champion's third line -- "Equip abilities you activate that
||| target this creature cost {3} less to activate." The "that target
||| ..." RESTRICTOR at a keyword-named ability class, which is where five
||| of the seven supported lines write it (Bladegraft Aspirant, Cloud,
||| Dwarven Mauler, Helitrooper, Strong Back are the others; Kopala's is
||| the sixth and seventh at the bare class). [CR#702.6a] makes equip an
||| activated ability that targets, so the class word and the restrictor
||| answer to the same rule.
public export
ferventChampionEquipDiscount : Ability
ferventChampionEquipDiscount =
  Static (CostsToCast
            (AllOf (And [ AbilityHead (KeywordClass "Equip")
                        , ActivatedBy You
                        , Targets Macros.thisCreature SomeTarget ]))
            (CostLess (Lit 3) Nothing))

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
                   (CostLess (Lit 1) Nothing))
       , Static (CostsToCast
                   (AllOf (And [AbilityHead (KeywordClass "Equip"), ActivatedBy You]))
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
           , Macros.objectCant "Activate"
               (AllOf (And [AbilityHead AnyActivated, AbilityOf It])) ]) ]
       Nothing

public export
stupefyingTouch : Card
stupefyingTouch =
  Macros.card "Stupefying Touch" (Just [Macros.generic 1, Macros.pip Blue]) []
       (MkTypeLine [enchantmentType "Aura"] [Enchantment])
       [ Macros.keywordSubject "Enchant" Macros.creature
       , Macros.triggered When (Enters Macros.thisAura Nothing) (Draw You (Lit 1))
       , Static (Macros.objectCant "Activate"
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
       , Static (Macros.objectCant "Activate"
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
                   (CountedGroup (Macros.exactly 2) Nothing
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
                   (CountedGroup (Macros.exactly 2) Nothing
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
                           (CountedGroup (Macros.exactly 3) Nothing
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
                   (CountedGroup (Macros.exactly 3) Nothing
                                 (And [Macros.land, HasSubtype (landType "Island"),
                                       ControlledBy You]))
                   Macros.handZ))))
       , Spell (Macros.counterSpell (Macros.target Macros.spell)) ]
       Nothing

public export
theLadyOfOtariaLine : StaticEffect []
theLadyOfOtariaLine =
  AltCost (Just (Do (SetStatus Tapped
            (CountedGroup (Macros.exactly 3) Nothing
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
                                   Compare [CharAxis ManaValue] Eq (LetterVal X),
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
                                   Compare [CharAxis ManaValue] Eq (LetterVal X),
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
           (Casts Macros.anOpponent (Macros.a Macros.spell) Nothing)
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
       [ Static (AltCost (Just (Do (Macros.exile
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
       [ Static (AltCost (Just (Do (Macros.exile
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
       [ Spell (Macros.exile
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
       [ Spell (Continuously
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
           (PutCounters (Lit 1) (PrintedKind Charge) Macros.thisArtifact)
       , Macros.activated (Compound [TapSymbol,
                              Do (Macros.sacrifice You Macros.thisArtifact)])
           (Macros.destroy (Each (And [Not Macros.land, Permanent,
                                       Compare [CharAxis ManaValue] Eq
                                         (CountersOn Charge Macros.thisArtifact)]))) ]
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

||| Savage Swipe, both sentences -- "Target creature you control gets
||| +2/+2 until end of turn if its power is 2. Then it fights target
||| creature you don't control." The conditioned clause announces its own
||| target [CR#601.2c] whether or not the condition holds, so the second
||| sentence names it. Five supported lines write this shape.
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
                            (PutCounters (Lit 1) (PrintedKind Macros.minusOneMinusOne) It)) ]
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
                   (Exists (And [Macros.land, HasSubtype (landType "Swamp"),
                                 ControlledBy You]))
                   (AltCost (Just (Do (ChangeLife You (Down (Lit 4)))))))
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
       [ Spell (CantBe (Macros.destroy (AllOf Macros.creature))
                       "Regenerate" Them) ]
       Nothing

public export
damnation : Card
damnation =
  Macros.card "Damnation"
       (Just [Macros.generic 2, Macros.pip Black, Macros.pip Black]) []
       (MkTypeLine [] [Sorcery])
       [ Spell (CantBe (Macros.destroy (AllOf Macros.creature))
                       "Regenerate" Them) ]
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
    (Continuously (Macros.objectCant "Regenerate" (Macros.target Macros.creature))
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
       , Macros.triggered When (Enters Macros.thisLand Nothing)
                          (Macros.sacrifice You (Macros.a Macros.land))
       , Static (Macros.deontic (AllOf Macros.creatureYouControl) Forbid ["Attack"] Agent NoDeonticPatient)
       , Static (Prevents AnyDamage AllOfIt (Macros.shieldingIt You) Nothing Nothing) ]
       Nothing

public export
aboroth : Card
aboroth =
  Macros.card "Aboroth"
       (Just [Macros.generic 4, Macros.pip Green, Macros.pip Green]) []
       (MkTypeLine [creatureType "Elemental"] [Creature])
       [ Macros.keywordCosting "CumulativeUpkeep"
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
       , Macros.keywordCosting "CumulativeUpkeep"
                               (Do (PutCounters (Lit 1) (PrintedKind Macros.plusOnePlusOne)
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
                   (PutCounters (Lit 1) (PrintedKind Age) Macros.thisEnchantment)



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
                                             HasKeyword (TheKeyword "FirstStrike")]))
                               (Continuously
                                  (Gains (AllOf Macros.creatureYouControl)
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
                               (BeginningOf Combat (ByWord Yours))
                               (Exists (And [Macros.creature,
                                             InZone (Macros.graveyardOf You),
                                             HasKeyword (TheKeyword "Flying")]))
                               (Continuously
                                  (Gains (AllOf Macros.creatureYouControl)
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
                               [ Macros.exile (Macros.target (InZone Macros.graveyardZ))
                               , PutCounters (Lit 1) (PrintedKind Macros.plusOnePlusOne) Macros.thisCreature ])
       , AlsoForKeywords
           (Static (Macros.asLongAs
                      (Exists (And [ExiledWith Macros.thisCreature, HasKeyword (TheKeyword "Flying")]))
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
       , Macros.triggered When (Enters Macros.thisCreature Nothing)
           (SetStatus Untapped
              (Each (And [HasSubtype (creatureType "Merfolk"), ControlledBy You,
                          OtherThan Macros.thisCreature])))
       , Static (Macros.asLongAs
           (Macros.happenedInvolving AttackDeclaration
                                     You
                                     Lookback.ThisTurn
                                     (CountedGroup (Macros.atLeast 3) Nothing
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
                                     , PutCounters (Lit 2) (PrintedKind Macros.plusOnePlusOne) It
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
                                      CastFrom (Macros.graveyardOf You)])) Nothing)
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
           , Macros.drawACard ]) ]
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
                   (CostLess (Lit 1) Nothing)) ]
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
       , Macros.triggered Whenever (Casts You (Macros.a Macros.spell) Nothing)
                          (Sequentially
                      [ Macros.gets (Each Macros.creatureYouControl)
                                    (PtUp (Lit 1)) (PtUp (Lit 0))
                                    (Just Macros.untilEndOfTurn)
                      , Macros.scryOne ])
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
                               (Pay They (ScaledMana GenericUnit (Macros.forEach
                                            (And [Macros.artifact, ControlledBy You]))) PaidOnce)
                               (Macros.counterSpell It)) ]
       Nothing

public export
rakshasasDisdain : Card
rakshasasDisdain =
  Macros.card "Rakshasa's Disdain" (Just [Macros.generic 2, Macros.pip Blue]) []
       (MkTypeLine [] [Instant])
       [ Spell (Macros.mayElse (ControllerOf (Macros.target Macros.spell))
                               (Pay They (ScaledMana GenericUnit (Macros.forEach
                                            (InZone (Macros.graveyardOf You)))) PaidOnce)
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
              (Pay You (ScaledMana GenericUnit (Macros.forEach
                          (And [Macros.creature, ControlledBy You,
                                OtherThan Macros.thisCreature]))) PaidOnce)
              (Macros.sacrifice You Macros.thisCreature)) ]
       (Just (3, 4))

public export
megatherium : Card
megatherium =
  Macros.card "Megatherium" (Just [Macros.generic 2, Macros.pip Green]) []
       (MkTypeLine [creatureType "Beast"] [Creature])
       [ Macros.keyword "Trample"
       , Macros.triggered When (Enters Macros.thisCreature Nothing)
           (Macros.mayElse You
              (Pay You (ScaledMana GenericUnit (Macros.forEach (InZone (Macros.handOf You)))) PaidOnce)
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
  ForEachOf (Each Macros.creature)
            (Macros.mayElse (ControllerOf It)
                            (Pay They (Do (ChangeLife They (Down (LetterVal X)))) PaidOnce)
                            (Macros.sacrifice They It))

public export
fadeAway : Effect []
fadeAway =
  ForEachOf (Each Macros.creature)
            (Macros.mayElse (ControllerOf It)
                            (Pay They (Mana [Macros.generic 1]) PaidOnce)
                            (Macros.sacrifice They (Macros.a Permanent)))

public export
martyrsCry : Effect []
martyrsCry =
  Sequentially [Macros.exile (AllOf (And [Macros.creature, ColorIs White])),
                ForEachOf (Macros.thoseVerbed "Exile" (TypeW Creature))
                          (Draw (ControllerOf It) (Lit 1))]

||| Hate Mirage's middle two sentences -- "For each of those creatures,
||| create a token that's a copy of that creature. Those tokens gain
||| haste." The loop's union export at work: `ForEachOf` summarises what
||| its body introduced ONE pass at a time into one plural mention, so
||| the sentence after the loop names the whole batch of tokens. Twinflame
||| ("Exile those tokens at the beginning of the next end step") and Smoke
||| Spirits' Aid ("Those tokens have enchant creature and ...") read the
||| same export at the same seam.
||| The printed FOURTH sentence, "Exile them", is not written here and
||| cannot be: the targeted creatures the loop ran over are still a plural
||| object mention, so the bare plural pronoun has two candidates where
||| "those tokens" has one.
public export
hateMirageTokens : Effect []
hateMirageTokens =
  Sequentially
    [ ForEachOf (TargetGroup (Macros.upTo 2) Macros.creatureYouDontControl)
                (Create You (Lit 1) (TokenCopyOf It []) [])
    , Macros.gains (Those TokenW) (Macros.keyword "Haste") Nothing ]

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
                                  Not (HasKeyword (TheKeyword "Flying"))]))
                      (Macros.mayElse (ControllerOf It)
                                      (Pay They (Mana [Macros.generic 1]) PaidOnce)
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
       , Macros.triggered When (Enters Macros.thisCreature Nothing)
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
  Sequentially [ Macros.exile (CountedGroup Macros.anyNumber Nothing
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
       [ Macros.triggered When (Enters Macros.thisCreature Nothing)
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
peacekeeperCant = Static (Macros.deontic (AllOf Macros.creature) Forbid ["Attack"] Agent NoDeonticPatient)


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
       [ Spell (Continuously (Macros.deontic (Macros.target Macros.creature)
                                      Require ["Block"] Agent NoDeonticPatient)
                             (Just Macros.thisTurn)) ]
       Nothing

||| Blazing Archon's second line: "Creatures can't attack you." The
||| restriction naming the player the attack is aimed at [CR#506.3].
public export
blazingArchonCant : Ability
blazingArchonCant =
  Static (Macros.deontic (AllOf Macros.creature) Forbid ["Attack"] Agent (DefendingPlayer You))

||| Clergy of the Holy Nimbus: "If this creature would be destroyed,
||| regenerate it." A replacement effect over a destruction, written as
||| the verbed event's passive [CR#701.8a] now that `IsDestroyed` has
||| retired into it. [CR#701.8b] destroys with no destroyer named -- the
||| lethal-damage state-based action [CR#704.5g] -- which is exactly what
||| the actorless voice says. The replacement arm writes the PRINTED
||| "it": `eventIntro` announces the replaced event's own subject, and a
||| deictic subject announces itself, so the pronoun has its antecedent.
public export
clergyOfTheHolyNimbus : Ability
clergyOfTheHolyNimbus =
  Static (Intercepts (VerbedEvent Nothing "Destroy"
                                  (Just Macros.thisCreature) False) [] Nothing
                     (Regenerate It) Repeatedly Nothing)

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
  Continuously (Macros.deontic (Macros.target Macros.creature) Require ["Attack"] Agent
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
      , Compare [CharAxis ManaValue] Less (StatOf Loyalty This) ]

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
                                             Macros.creatureYouControl])) Nothing)
    (PutCounters (Lit 1) (PrintedKind LoyaltyCounter)
                 (AttachHost Enchanted (TypeW Planeswalker)))

||| "Put a +1/+1 counter on target creature card in your graveyard."
||| [CR#122.1a] counts a +X/+Y counter on a creature card in a zone other
||| than the battlefield.
public export
counterOnGraveyardCard : Effect []
counterOnGraveyardCard =
  PutCounters (Lit 1) (PrintedKind Macros.plusOnePlusOne)
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
    (PutCounters ThatMuch (PrintedKind Macros.plusOnePlusOne) Macros.thisCreature)

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
    (Macros.triggered Whenever (Enters (Macros.a (And [Macros.land, ControlledBy You])) Nothing)
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
                      -- the second verb phrase's subject is ELIDED, not
                      -- pronominalised: "gets +4/+4 and gains trample" is one
                      -- statement of one subject, so the coordination writes
                      -- that subject once and nothing here reads anything.
                      -- The carrier-scoped pronoun this line used to write
                      -- (`itAsPermanent`, to keep the card the discard cost
                      -- put in the graveyard out of the count) was working
                      -- around the missing form.
                      (Macros.sharedSubject
                         (Macros.target (And [Macros.creature, Attacking]))
                         [ VPGets (PtUp (Lit 4)) (PtUp (Lit 4)) Nothing
                         , VPGains (Macros.keyword "Trample") Nothing ]
                         (Just Macros.untilEndOfTurn)))


||| Twinshot Sniper, whole card -- "Reach / When this creature enters, it
||| deals 2 damage to any target. / Channel — {1}{R}, Discard this card:
||| It deals 2 damage to any target." The Channel line's pronoun reads
||| what the COST announced: a bare `This` moved to a zone now mints its
||| own mention, on the ascribed self's model, so the discarded card is in
||| the discourse the ability body reads.
public export
twinshotSniper : Card
twinshotSniper =
  Macros.card "Twinshot Sniper"
       (Just [Macros.generic 3, Macros.pip Red]) []
       (MkTypeLine [creatureType "Goblin", creatureType "Archer"] [Artifact, Creature])
       [ Macros.keyword "Reach"
       , Macros.triggered When (Enters Macros.thisCreature Nothing)
           (DealDamage It (Lit 2) (Macros.target Macros.anyTarget))
       , AbilityWord Channel
           (Macros.activated (Compound [Mana [Macros.generic 1, Macros.pip Red],
                                        Do (Macros.discards You This)])
                             (DealDamage It (Lit 2) (Macros.target Macros.anyTarget))) ]
       (Just (2, 3))

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

||| Dragonlord Ojutai's static: "Dragonlord Ojutai has hexproof as long as
||| it's untapped." The postposed orientation: the condition is written
||| after the statement and pronominalises the statement's own subject,
||| which only that orientation can read. Iymrith, Desert Doom writes the
||| same shape with a PARAMETER ("has ward {4} as long as it's untapped")
||| and is now benched whole at `iymrithDesertDoom`, its draw line with it.
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
  Static (Macros.onlyUnless (Macros.deontic Macros.thisCreature Forbid ["Attack"] Agent
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


||| Quakebringer's damage trigger: "At the beginning of your upkeep,
||| Quakebringer deals 2 damage to each opponent. This ability triggers
||| only if Quakebringer is on the battlefield or if Quakebringer is in
||| your graveyard and you control a Giant." The condition disjunction
||| whole, with a conjunction inside its second arm: the reduplicated "if"
||| is what scopes the "and", so the conjunction binds inside the disjunct
||| its own "if" opened.
public export
quakebringerDamage : Ability
quakebringerDamage =
  Macros.triggeredIf At (BeginningOf Upkeep (ByWord Yours))
    (OrCond [ Matches This (InZone Macros.battlefieldZ)
            , AndCond [ Matches This (InZone (Macros.graveyardOf You))
                      , Exists (And [Macros.creature,
                                     HasSubtype (creatureType "Giant"),
                                     ControlledBy You]) ] ])
    (DealDamage This (Lit 2) (Each Opponent))

||| Dark Fortress's mana ability: "{T}: Add {B} or {R}. Activate only if
||| this land entered this turn or if you control a basic land." The
||| doubly-marked disjunction on the ACTIVATION GUARD, which is this
||| family's biggest carrier; Gathering Place, Gleaming Bastion, Hidden
||| Lair and Training Compound are the same line.
public export
darkFortressMana : Ability
darkFortressMana =
  Macros.activatedOnlyIf TapSymbol
    (AddMana You (Lit 1) (Runs [[OfColor Black], [OfColor Red]]) [])
    (OrCond [ Macros.happened Entry Macros.thisLand Lookback.ThisTurn
            , Exists (And [Macros.land, HasSupertype Basic, ControlledBy You]) ])

||| Sand Strangler's trigger: "When this creature enters, if you control a
||| Desert or there is a Desert card in your graveyard, you may have this
||| creature deal 3 damage to target creature." The SINGLY marked
||| disjunction, whose two arms are independent clauses; Desert's Hold,
||| Gilded Cerodon, Unquenchable Thirst, Wall of Forgotten Pharaohs and
||| Wretched Camel write the same condition.
public export
sandStranglerDamage : Ability
sandStranglerDamage =
  Macros.triggeredIf When (Enters Macros.thisCreature Nothing)
    (OrCond [ Exists (And [Macros.land, HasSubtype (landType "Desert"),
                           ControlledBy You])
            , Exists (And [Macros.land, HasSubtype (landType "Desert"),
                           InZone (Macros.graveyardOf You)]) ])
    (Macros.may You (DealDamage This (Lit 3) (Macros.target Macros.creature)))

||| Skyblade's Boon's return ability: "{2}{W}: Return Skyblade's Boon to
||| its owner's hand. Activate only if Skyblade's Boon is on the
||| battlefield or in your graveyard." The singly-marked ZONE disjunction
||| on an activation guard -- one subject, two zones, no reduplicated
||| marking -- which Arahbo, Edgar Markov, Firemane Angel, Inalla and
||| Sidar Jabari of Zhalfir all write.
public export
skybladesBoonReturn : Ability
skybladesBoonReturn =
  Macros.activatedOnlyIf (Mana [Macros.generic 2, Macros.pip White])
    (Macros.move This Macros.handZ)
    (OrCond [ Matches This (InZone Macros.battlefieldZ)
            , Matches This (InZone (Macros.graveyardOf You)) ])


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
                          (PutCounters (Lit 1) (PrintedKind Luck) Macros.thisEnchantment)
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
                   (PutCounters Macros.theResult (PrintedKind Charge) Macros.thisArtifact)

||| The Space Family Goblinson, first line: "Whenever you roll a die, put
||| a +1/+1 counter on The Space Family Goblinson." The singular
||| determiner beside Brazen Dwarf's plural one.
public export
spaceFamilyGoblinsonRoll : Ability
spaceFamilyGoblinsonRoll =
  Macros.triggered Whenever Macros.youRollADie
                   (PutCounters (Lit 1) (PrintedKind Macros.plusOnePlusOne)
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
  Sequentially [Macros.rollDice 2 20, IgnoreOutcomes (IgnoreExtreme LowestRoll)]

||| Iron Mastiff's ignore, benched over a bare roll: "…and ignore all but
||| the highest roll." The card rolls "a d20 for each player being
||| attacked", a count over the players an attack names, which no player
||| description carries.
public export
ironMastiffIgnore : Effect []
ironMastiffIgnore =
  Sequentially [Macros.rollADie 20, IgnoreOutcomes (IgnoreAllBut HighestRoll)]

||| Xenosquirrels' modifier, benched over a roll of its own: "…increase
||| or decrease the result by 1." [CR#706.2]'s modifier "from other
||| sources". The card writes it under an "After you roll a die" header,
||| and [CR#603.1] writes a triggered ability's word as
||| "[When/Whenever/At]", which is the whole of `TriggerWord`.
public export
xenosquirrelsShift : Effect []
xenosquirrelsShift =
  Sequentially [Macros.rollADie 6, ShiftResult (Lit 1)]

||| Wyll, Blade of Frontiers, first line -- "If you would roll one or more
||| dice, instead roll that many dice plus one and ignore the lowest
||| roll." Barbarian Class's level-1 line is the same sentence, and Pixie
||| Guide's is it under an ability word. The replacement side of the
||| ignore instruction: the would-event announces the dice it called for
||| [CR#706.1], never a result, since [CR#614.6] keeps the replaced roll
||| from happening; "that many" reads that count and the bare "dice"
||| reads its kind.
public export
wyllExtraDie : Effect []
wyllExtraDie =
  Macros.ifWouldInstead Macros.youRollDice
    (Sequentially [ RollDice You (Plus ThatMuch (Lit 1)) ThoseDice
                  , IgnoreOutcomes (IgnoreExtreme LowestRoll) ])
    Nothing

||| Atomwheel Acrobats, first line: "Whenever you roll a 1 or 2, put that
||| many +1/+1 counters on this creature." The two-ended result test
||| [CR#706.3a], and the body reading the result back as "that many".
public export
atomwheelAcrobatsRoll : Ability
atomwheelAcrobatsRoll =
  Macros.triggered Whenever (Macros.youRollResultIn (Range (Just 1) (Just 2)))
                   (PutCounters ThatMuch (PrintedKind Macros.plusOnePlusOne)
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

||| Netherese Puzzle-Ward, second line: "Perfect Illumination — Whenever
||| you roll a die's highest natural result, draw a card." The test no
||| literal range spells: [CR#706.2] takes the natural result before any
||| modifier and [CR#706.1a] numbers each die to its own N, so the
||| header's number depends on the die it watches. The ability word is
||| not written.
public export
netheresePuzzleWardIllumination : Ability
netheresePuzzleWardIllumination =
  Macros.triggered Whenever Macros.youRollHighestNatural Macros.drawACard

||| Resolute Veggiesaur, second line: "Whenever you roll your third die
||| each turn, put a +1/+1 counter on this creature." No vocabulary of its
||| own: the ordinal occurrence word over the roll event, with the period
||| that resets its count. It was benched as an every-player WINDOW, which
||| said the wrong thing: a window restricts when the header may trigger,
||| and this header may trigger on any turn -- what "each turn" bounds is
||| how many rolls have to have happened first.
public export
resoluteVeggiesaurThirdDie : Ability
resoluteVeggiesaurThirdDie =
  Macros.triggered Whenever
    (NthOccurrence (Nth 3) (Just Turn) Macros.youRollADie)
    (PutCounters (Lit 1) (PrintedKind Macros.plusOnePlusOne)
                 Macros.thisCreature)

||| Fractured Powerstone, second line: "{T}: Roll the planar die.
||| Activate only as a sorcery." The planar die's instruction row
||| [CR#901.3a]; it announces no number, since [CR#706.7] has every
||| numerical read ignore the planar roll.
public export
fracturedPowerstonePlanarRoll : Ability
fracturedPowerstonePlanarRoll =
  Macros.activatedOnlyDuring TapSymbol Macros.rollThePlanarDie AsSorcery

||| Ichor Elixir, first line: "If you would roll one or more planar dice,
||| instead roll that many planar dice plus one and ignore one." The roll
||| header narrowed to the planar die, the planar instruction taking a
||| count, and the chosen ignore, in one sentence. The replaced event
||| announces the DICE it called for and no result [CR#706.7], which is
||| exactly what "that many planar dice" reads.
public export
ichorElixirPlanarDice : Effect []
ichorElixirPlanarDice =
  Macros.ifWouldInstead Macros.youRollPlanarDice
    (Sequentially [ RollPlanarDie You (Plus ThatMuch (Lit 1))
                  , IgnoreOutcomes (IgnoreChosen Nothing (Lit 1)) ])
    Nothing

||| Vedalken Squirrel-Whacker, second line, benched to the clause the
||| exchange blocks: "If you would roll one or more six-sided dice,
||| instead roll them." The numbered die named on the header [CR#706.1],
||| and the wholly anaphoric body -- "them" is `ThatMuch` over
||| `ThoseDice`, both read off the roll the replacement announced. The
||| line's remaining clause exchanges a result with a base characteristic,
||| which is not written.
public export
vedalkenSquirrelWhackerReroll : Effect []
vedalkenSquirrelWhackerReroll =
  Macros.ifWouldInstead (RollsDice You ManyDice (SidedDie 6) AnyResult)
    (RollDice You ThatMuch ThoseDice)
    Nothing

||| Krark's Thumb: "If you would flip a coin, instead flip two coins and
||| ignore one." The flipping ACT as the replaced event [CR#705.1] --
||| which neither arm of the call names [CR#705.2] -- and the ignore
||| instruction over coins. The body writes its own count, since
||| [CR#614.6] leaves no flip to read back.
public export
krarksThumbExtraFlip : Effect []
krarksThumbExtraFlip =
  Macros.ifWouldInstead Macros.youFlipACoin
    (Sequentially [ Macros.flipCoins 2
                  , IgnoreOutcomes (IgnoreChosen Nothing (Lit 1)) ])
    Nothing

||| Bamboozling Beeble, second line: "{1}, {T}: The next time target
||| player would roll one or more dice this turn, instead they roll that
||| many dice plus one and you choose one of those rolls to ignore." The
||| one printed line that WRITES the ignore's chooser, and it writes a
||| different player from the roller -- which is why the chooser is a
||| slot and not [CR#706.6]'s tie-break rule.
public export
bamboozlingBeebleIgnore : Ability
bamboozlingBeebleIgnore =
  Macros.activated (Compound [Mana [Macros.generic 1], TapSymbol])
    (Macros.nextTimeWouldInstead
       (RollsDice (Macros.target AnyPlayer) ManyDice AnyDie AnyResult)
       (Sequentially [ RollDice They (Plus ThatMuch (Lit 1)) ThoseDice
                     , IgnoreOutcomes (IgnoreChosen (Just You) (Lit 1)) ])
       (Just Macros.thisTurn))

||| Missy's end-step line, benched as the branch it writes: "you draw a
||| card and chaos ensues." [CR#311.7] admits the instruction beside the
||| die face — a chaos ability triggers "if a resolving spell or ability
||| says that chaos ensues" — so the sentence needs no planar roll. The
||| villainous choice that frames the two branches is its own family and
||| is not written.
public export
missyChaosBranch : Effect []
missyChaosBranch = Sequentially [Macros.drawACard, ChaosEnsues]

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
  Sequentially [ FlipCoins (Each AnyPlayer) (FlipCount (Lit 1))
               , Macros.sacrifice (Each (And [AnyPlayer, CoinCameUp Tails]))
                                  (Macros.a Macros.creature) ]

||| Rakdos, the Showstopper's trigger body: "flip a coin for each creature
||| that isn't a Demon, Devil, or Imp. Destroy each creature whose coin
||| comes up tails." The per-member flip: [CR#705.1] leaves the coin owned
||| by no referent and [CR#705.2] gives the flip to whoever flips it, so
||| the described set is a second slot on the instruction and the subject
||| stays the flipper.
public export
rakdosShowstopperFlips : Effect []
rakdosShowstopperFlips =
  Sequentially
    [ Macros.flipACoinFor
        (Each (And [ Macros.creature
                   , Not (Or [ HasSubtype (creatureType "Demon")
                             , HasSubtype (creatureType "Devil")
                             , HasSubtype (creatureType "Imp") ]) ]))
    , Macros.destroy (Each (And [Macros.creature, CoinCameUp Tails])) ]

||| Warp Vortex's first sentence, benched alone: "flip a coin for each
||| opponent you have." The per-member flip over the player kind beside
||| Rakdos's over the object kind [CR#705.1]. The line's remaining
||| sentences count the flips you WON and the ones you lost;
||| `CoinsShowing` counts coins by the face they came up, which is
||| [CR#705.2]'s other reading, and the called one has no count word.
public export
warpVortexFlips : Effect []
warpVortexFlips = Macros.flipACoinFor (Each Opponent)

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
       [ Macros.triggered When (Enters Macros.thisCreature Nothing)
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
||| costless face answers. The probe stands on its own terms; the Arlinn card
||| it stood in for is benched whole as `arlinnKord`, and Garruk Relentless
||| waits on a state trigger rather than on the verb (see `predatoryWurm`).
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

||| Nykthos, Shrine to Nyx -- "{T}: Add {C}. / {2}, {T}: Choose a color.
||| Add an amount of mana of that color equal to your devotion to that
||| color." The devotion read at a CHOSEN colour rather than a printed
||| one: the produced-mana side already had its chosen-colour arm, and
||| the count's own colour slot is what had none. Nyx Lotus writes the
||| same ability without the {2}.
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

||| Karametra's Acolyte -- "{T}: Add an amount of {G} equal to your devotion
||| to green."
public export
karametrasAcolyte : Ability
karametrasAcolyte =
  Macros.activated TapSymbol
    (AddMana You (Devotion You (LitColor Green) Nothing) (Runs [[OfColor Green]]) [])

||| Anax, Hardened in the Forge -- "Anax's power is equal to your devotion
||| to red." The devotion read at the definition frame; the box is */3.
public export
anaxPowerDefinition : Ability
anaxPowerDefinition =
  Static (DefinesPt Macros.thisCreature PowerAlone (Devotion You (LitColor Red) Nothing))

||| Gray Merchant of Asphodel, first clause -- "each opponent loses X life,
||| where X is your devotion to black." (The second sentence, "You gain life
||| equal to the life lost this way.", is not taken here.)
public export
grayMerchantDrain : Effect []
grayMerchantDrain =
  Sequentially [ Macros.losesLife (Each Opponent) (LetterVal X)
               , Define X (Devotion You (LitColor Black) Nothing) ]

||| Erebos, God of the Dead -- "As long as your devotion to black is less
||| than five, Erebos isn't a creature." The devotion read at the comparison
||| frame.
public export
devotionCondition : Condition []
devotionCondition = CompareAmt (Devotion You (LitColor Black) Nothing) Less (Lit 5)

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
  Macros.triggered Whenever (Enters (Macros.a Macros.creatureYouControl) Nothing)
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
    (Dies (CountedGroup (Macros.atLeast 1) Nothing
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
                              Do (RemoveCounters (Macros.exactly 1) (Just Suspect)
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

||| Shapeshifter -- "As this creature enters, choose a number between 0 and
||| 7. / At the beginning of your upkeep, you may choose a number between 0
||| and 7. / Shapeshifter's power is equal to the last chosen number and its
||| toughness is equal to 7 minus that number." The NON-ENTRY chooser's
||| whole-card witness: the second chooser sits in an UPKEEP TRIGGER and the
||| static two lines later reads it, so what the card wanted was the
||| container carrying a choice across the ability boundary, not a second
||| choice clause. The read is marked for the reason [CR#607.2d] marks it --
||| two choosers of one sort stand, so no unmarked read could name either.
public export
shapeshifter : Card
shapeshifter =
  Macros.cardOf "Shapeshifter" (Just [Macros.generic 6]) []
       (MkTypeLine [creatureType "Shapeshifter"] [Artifact, Creature])
       [ Static (Macros.entersChoosingFrom Macros.thisCreature Number
                                           (NumberBetween 0 7))
       , Macros.triggered At (BeginningOf Upkeep (ByWord Yours))
           (Macros.may You
              (Macros.choose (Macros.a (Macros.qualityFrom Number
                                          (NumberBetween 0 7)))))
       , Static (AndAlso
           [ DefinesPt Macros.thisCreature PowerAlone ChosenNumber
           , DefinesPt Macros.thisCreature ToughnessAlone
               (Minus (Lit 7) ChosenNumber) ]) ]
       (Just shapeshifterBox)

||| Multiple Choice, first arm -- "If X is 1, scry 1, then draw a card." The
||| announced letter on a comparison's left, at the equality.
public export
multipleChoiceFirstArm : Effect []
multipleChoiceFirstArm =
  If (CompareAmt (LetterVal X) Eq (Lit 1))
     (Sequentially [ Macros.scryOne
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
                 Compare [CharAxis Power] Greater
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
             Compare [CharAxis ManaValue] Eq
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
    (NthOccurrence (Nth 1) Nothing (Casts You (Macros.a Macros.spell) Nothing))
    (DuringWindow Turn (Just EachOpponents))
    Macros.drawACard

||| Midnight Clock's header -- "When the twelfth hour counter is put on this
||| artifact, …" (the body shuffles hand and graveyard into the library, which
||| is unbuilt, so the header is the witness).
public export
midnightClockHeader : GameEvent []
midnightClockHeader =
  NthOccurrence (Nth 12) Nothing
    (Macros.singleCounterEvent CounterPut Hour Macros.thisArtifact)

||| Political Triumph -- "When the fourth plan counter is put on this
||| enchantment, sacrifice it, draw a card, and put a +1/+1 counter on each
||| creature you control." The plan-counter Saga family's header.
public export
politicalTriumphHeader : GameEvent []
politicalTriumphHeader =
  NthOccurrence (Nth 4) Nothing
    (Macros.singleCounterEvent CounterPut Plan Macros.thisEnchantment)

||| Run the Play (Striding Shotcaller's other half), first clause -- "Put a
||| +1/+1 counter on each of up to X target creatures." The amount ceiling on
||| a target group's quantity.
public export
runThePlayCounters : Effect []
runThePlayCounters =
  PutCounters (Lit 1) (PrintedKind Macros.plusOnePlusOne)
              (EachOf (TargetGroup (UpToOf (LetterVal X)) Macros.creature))

||| Berserker's Frenzy, the 1—14 striation -- "Choose any number of creatures.
||| They block this turn if able." The counted choice the ticket lists as
||| over-refused; `choosable (CountedGroup _ _ _)` already admits it.
public export
berserkersFrenzyLowRoll : Effect []
berserkersFrenzyLowRoll =
  Sequentially
    [ Macros.choose (CountedGroup Macros.anyNumber Nothing Macros.creature)
    , Continuously (Macros.deontic Them Require ["Block"] Agent NoDeonticPatient)
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
  {auto 0 ck : countUnionHalf (TypeW Planeswalker) bs = 1} ->
  {auto 0 pk : countUnionHalf PlayerW bs = 1} ->
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
    (Macros.exiles (That PlayerW) (Each (InZone (Macros.libraryOf They))))

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

||| Heart of Bogardan, whole card. Its body writes now that the split
||| read is gated on the union mention its two arms share rather than on
||| each arm's word being unique in the whole prefix -- the header
||| announces the non-payer, so "that player" had two singular player
||| mentions to choose between and exactly one union to name a half of.
public export
heartOfBogardan : Card
heartOfBogardan =
  Macros.card "Heart of Bogardan"
       (Just [Macros.generic 2, Macros.pip Red, Macros.pip Red]) []
       (MkTypeLine [] [Enchantment])
       [ Macros.keywordCosting "CumulativeUpkeep" (Mana [Macros.generic 2])
       , Macros.triggered When Cards.heartOfBogardanHeader
           (Sequentially
              [ Simultaneously
                  [ DealDamage This (LetterVal X) Cards.targetPlayerOrPlaneswalker
                  , DealDamage This (LetterVal X) Cards.eachCreatureThatSplitControls ]
              , Define X (Minus (Times 2 (CountersOn Age Macros.thisEnchantment)) (Lit 2)) ]) ]
       Nothing

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

||| Shah of Naar Isle -- "Trample / Echo {0} / When this creature's echo
||| cost is paid, each opponent may draw up to three cards." The passive
||| payment header over the second keyword whose parameter is a cost
||| [CR#702.30a], and the ceiling draw: the `may` offers the action, the
||| `UpTo` offers the number, and both belong to the opponent.
public export
shahOfNaarIsle : Card
shahOfNaarIsle =
  Macros.card "Shah of Naar Isle"
       (Just [Macros.generic 3, Macros.pip Red]) []
       (MkTypeLine [creatureType "Efreet"] [Creature])
       [ Macros.keyword "Trample"
       , Macros.keywordCosting "Echo" (Mana [Macros.generic 0])
       , Macros.triggered When (PaysCost Nothing Paid Macros.thisCreature "Echo")
                          (Macros.may (Each Opponent)
                                      (Draw (Those PlayerW) (UpTo (Lit 3)))) ]
       (Just (6, 6))

||| Font of Agonies -- "Whenever you pay life, put that many blood
||| counters on this enchantment." The paid thing is a resource and not a
||| named cost, and the payment carries the number [CR#119.4] that "that
||| many" reads back.
public export
fontOfAgoniesTrigger : Ability
fontOfAgoniesTrigger =
  Macros.triggered Whenever (PaysLife You)
    (PutCounters ThatMuch (PrintedKind Blood) Macros.thisEnchantment)

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
                    Compare [CharAxis ManaValue] Eq (CountersOn Age Macros.thisEnchantment)])
          , Macros.putOntoBattlefield (That CardW)
          , Macros.shuffle ]))

-- A paid optional cost read back later: the payment is state the object
-- carries [CR#707.2], the reading ability is linked to the offering one
-- [CR#607.2i], and the read names WHICH cost -- never a tag some clause
-- minted.

||| Krosan Druid -- "Kicker {4}{G} / When this creature enters, if it was
||| kicked, you gain 10 life." The cheapest whole card in the family: the
||| keyword offers the cost [CR#702.33a] and the intervening-if reads
||| back the declaration that made the spell kicked [CR#702.33d].
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

||| Lightkeeper of Emeria -- "Multikicker {W} / Flying / When this
||| creature enters, you gain 2 life for each time it was kicked." The
||| count read: [CR#702.33c] makes a multikicker cost a kicker cost, so
||| the declaration writes the printed word and the read names the cost
||| that was paid.
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

||| Merfolk Falconer -- "Flying / Whenever you cast a kicked spell, scry
||| 2." The same read inside a DESCRIPTION rather than on the source:
||| "kicked" describes the spell that was cast, and nothing about the
||| read changes when the object it is anchored to is someone else's.
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
           (Macros.scry (Lit 2)) ]
       (Just (4, 4))

||| Ertai's Trickery -- "Counter target spell if it was kicked." The
||| whole card in one clause, and the read at its plainest: the object it
||| describes is the spell the clause just targeted, not the source, so
||| nothing about the read is tied to the text that carries it.
public export
ertaisTrickery : Card
ertaisTrickery =
  Macros.card "Ertai's Trickery" (Just [Macros.pip Blue]) []
       (MkTypeLine [] [Instant])
       [ Spell (OnlyIf (Macros.counterSpell (Macros.target Macros.spell))
                       (Matches It (PaidCost (ByKeyword "Kicker") Nothing)) Nothing) ]
       Nothing

||| Baleful Mastery's paid read -- "If the {1}{B} cost was paid, an
||| opponent draws a card." The unnamed arm: the card writes its
||| alternative cost out [CR#118.9] instead of naming a keyword, so the
||| read has nothing but "the card's own alternative cost" to sort by and
||| the printed symbols are spelling. The rest of the card does not
||| write -- "Exile target creature or planeswalker" needs a joined
||| target the round did not open.
public export
balefulMasteryPaidRead : Ability
balefulMasteryPaidRead =
  Spell (If (Matches This (PaidCost TheAlternative Nothing))
            (Draw (Macros.a Opponent) (Lit 1)) Nothing)

||| Stormscape Battlemage's first kicker trigger -- "When this creature
||| enters, if it was kicked with its {W} kicker, you gain 3 life."
||| [CR#607.2i]'s own worked example, and the ordinal arm: the card
||| declares "Kicker {W} and/or {2}{B}", which is two kicker abilities
||| [CR#702.33b], and [CR#702.33f] fixes the printed cost in the read as
||| naming the FIRST one listed. The rest of the card does not write --
||| its second trigger destroys a creature that "can't be regenerated",
||| whose regeneration ban is not this round's.
public export
stormscapeBattlemageFirstKicker : Ability
stormscapeBattlemageFirstKicker =
  Macros.triggeredIf When (Enters Macros.thisCreature Nothing)
    (Matches Macros.thisCreature (PaidCost (ByNthKeyword (Nth 1) "Kicker") Nothing))
    (Macros.gainsLife You (Lit 3))

||| Karai, Future of the Foot's payment window -- "if her sneak cost was
||| paid this turn". The corpus's ONE turn-scoped payment read, and the
||| whole reason `PaidCost` has a window slot: every other read is
||| timeless because the payment is casting state the object keeps
||| [CR#707.2], and this one asks whether the casting was this turn.
||| The rest of the card is not this row's -- its trigger returns a
||| creature card from a graveyard to hand and this line replaces the
||| destination.
public export
karaiSneakPaidThisTurn : Predicate [] Object
karaiSneakPaidThisTurn = PaidCost (ByKeyword "Sneak") (Just ThisTurn)

-- TWO PAYMENT READS THAT STILL DO NOT WRITE, with their exact blockers,
-- so no later round re-derives them:
--
-- * VERRAK, WARPED SENGIR -- "Whenever you activate an ability that
--   isn't a mana ability, IF LIFE WAS PAID to activate it, you may pay
--   that much life again." The corpus's one true life-payment readback.
--   Three things are wrong for `PaidCost`, not one: the read sorts by no
--   cost NAME (there is no keyword and no written alternative, so no
--   `PaidCostName` arm fits), it is anchored to an ABILITY where
--   `PaidCost` is a `Predicate bs Object`, and it reads back an AMOUNT
--   ("that much life") where `TimesPaid` reads a count of payments.
--   `PaysLife` is an event header and not a state read, so it does not
--   reach either.
--
-- * YIDARO, WANDERING MONSTER -- "If you've cycled a card named Yidaro,
--   Wandering Monster four or more times this game". Cycling is not a
--   keyword ACTION: [CR#702.29c] defines "when you cycle this card" as
--   discarding it "to pay an activation cost of a cycling ability", so
--   the count is a count of cost PAYMENTS. What refuses it is that the
--   count runs over payment EVENTS across every copy of a named card in
--   the game, where `TimesPaid` reads the state of ONE object as it was
--   cast [CR#118.10] and `EventCount`'s complement has no way to name a
--   keyword's cost. Adding a window to `TimesPaid` would not close it.

-- THE MODAL COST WORDS. Entwine [CR#702.42a] and escalate [CR#702.120a]
-- are additional costs a modal spell declares; both write ZERO readbacks
-- in the corpus, so the keyword line itself is the whole surface.

||| Borrowed Malevolence, whole -- "Escalate {2} / Choose one or both —
||| • Target creature gets +1/+1 until end of turn. • Target creature
||| gets -1/-1 until end of turn." The escalate line beside the modal
||| clause it prices; the LINKAGE between them -- and the per-mode
||| multiplier [CR#702.120a] writes -- is still unspelled, and no card
||| reads the payment back.
public export
borrowedMalevolence : Card
borrowedMalevolence =
  Macros.card "Borrowed Malevolence" (Just [Macros.pip Black]) []
       (MkTypeLine [] [Instant])
       [ Macros.keywordCosting "Escalate" (Mana [Macros.generic 2])
       , Spell (Modal (Range (Just 1) (Just 2))
                  [ Continuously
                      (Gets (Macros.target Macros.creature)
                            (PtUp (Lit 1)) (PtUp (Lit 1)))
                      (Just Macros.untilEndOfTurn)
                  , Continuously
                      (Gets (Macros.target Macros.creature)
                            (PtDown (Lit 1)) (PtDown (Lit 1)))
                      (Just Macros.untilEndOfTurn) ]) ]
       Nothing

||| Korlash's grandeur ability -- "Grandeur — Discard another card named
||| Korlash, Heir to Blackblade: Search your library for up to two Swamp
||| cards, put them onto the battlefield tapped, then shuffle." The
||| grandeur discard cost's shape, 7 supported lines: the cost names a
||| card by its own PRINTED NAME [CR#201.1] and excludes the object
||| itself, which is `Named (PrintedName ...)` beside `Other` and needs
||| nothing minted. Benched as the cost alone -- the body's
||| search-and-put is not this row's.
public export
grandeurDiscardCost : Cost []
grandeurDiscardCost =
  Do (Macros.discards You
        (Macros.a (And [Named (PrintedName "Korlash, Heir to Blackblade"),
                        OtherThan This, InZone Macros.handZ])))

||| Invigorate, whole -- "If you control a Forest, rather than pay this
||| spell's mana cost, you may have an opponent gain 3 life. / Target
||| creature gets +4/+4 until end of turn." The one card whose declined
||| cost is a life GAIN. It was recorded as blocked on `costActionOk`
||| admitting only the loss; re-probed 2026-08-28, the cell reads
||| `costActionOk (ChangeLife _ _) = True` and the card writes with
||| nothing minted, so that record was stale.
public export
invigorate : Card
invigorate =
  Macros.card "Invigorate"
       (Just [Macros.generic 2, Macros.pip Green]) []
       (MkTypeLine [] [Instant])
       [ Static (Macros.asLongAs
                   (Exists (And [Macros.land, HasSubtype (landType "Forest"),
                                 ControlledBy You]))
                   (AltCost (Just (Do (ChangeLife Macros.anOpponent
                                                  (Up (Lit 3)))))))
       , Spell (Continuously
                  (Gets (Macros.target Macros.creature)
                        (PtUp (Lit 4)) (PtUp (Lit 4)))
                  (Just Macros.untilEndOfTurn)) ]
       Nothing

||| Deflecting Swat's first line -- "If you control a commander, you may
||| cast this spell without paying its mana cost." One of the five
||| commander-gated free spells the alternative-cost round left refused
||| (with Fierce Guardianship, Deadly Rollick, Flawless Maneuver and
||| Obscuring Haze). The blocker was never this row: it was that
||| `CommanderD`'s scope is `HeldByCard` and no predicate read that
||| scope. `HasCardDesignation` [CR#903.3] is that predicate, and with it
||| the five write with nothing minted here. Their SECOND lines are
||| separate business apiece.
public export
deflectingSwatCommanderAltCost : Ability
deflectingSwatCommanderAltCost =
  Static (Macros.asLongAs
            (Exists (And [HasCardDesignation CommanderD, ControlledBy You]))
            (AltCost Nothing))

-- THE FREE CAST OF ANOTHER CARD, 296 supported lines. [CR#118.9] lets an
-- effect license a cast "without paying its mana cost"; where the 16
-- self lines say it of the object the line is printed on -- which is
-- `AltCost Nothing`, landed -- these say it of a card the clause has
-- picked out, so the licence and the spell being cast are two different
-- objects and the rider belongs on the permission.

||| Memory Plunder, whole -- "You may cast target instant or sorcery card
||| from an opponent's graveyard without paying its mana cost."
public export
memoryPlunder : Card
memoryPlunder =
  Macros.card "Memory Plunder"
       (Just [Macros.generic 1, Macros.pip Blue, Macros.pip Black]) []
       (MkTypeLine [] [Instant])
       [ Spell (Continuously
                  (Macros.mayCastFromFree You
                     (Macros.target (And [Macros.instantOrSorcery, IsCard]))
                     (Macros.graveyardOf Macros.anOpponent))
                  Nothing) ]
       Nothing

||| Omniscience, whole -- "You may cast spells from your hand without
||| paying their mana costs." The same rider on a STANDING permission
||| rather than a resolving one, at the plural the card writes.
public export
omniscience : Card
omniscience =
  Macros.card "Omniscience"
       (Just [Macros.generic 7, Macros.pip Blue, Macros.pip Blue]) []
       (MkTypeLine [] [Enchantment])
       [ Static (Macros.mayCastFromFree You (AllOf Macros.spell)
                                        (Macros.handOf You)) ]
       Nothing

||| Tranquil Frillback's offer -- "you may pay {G} up to three times."
||| The capped half of `PayTimes`, and the family's one card that writes
||| a bound; the reflexive trigger it seats chooses up to that many modes
||| and is not this row's.
public export
tranquilFrillbackOffer : Effect []
tranquilFrillbackOffer =
  Macros.may You (Pay You (Mana [Macros.pip Green]) (UpToTimes 3))

-- THE WHERE-CLAUSE ON A KEYWORD'S NUMBER PARAMETER: the grant opens the
-- letter, the where-clause closes it. 4 supported lines write one, and
-- monstrosity's "{X}{X}{G}: Monstrosity X" is NOT among them --
-- [CR#701.37c] makes the value X had as the permanent became monstrous
-- a LINKED value the other abilities read, so nothing there is defined
-- afresh and no card writes "where X is" beside it.

||| Fumiko the Lowblood's first line -- "Fumiko has bushido X, where X is
||| the number of attacking creatures." [CR#702.45a] writes "Bushido N";
||| the card puts a variable in the slot and defines it in the same
||| statement, which is [CR#107.3f]'s "X appears in the text ... and the
||| value is defined by the text".
public export
fumikoBushidoX : Ability
fumikoBushidoX =
  Static (AndAlso [ Gains Macros.thisCreature
                          (KeywordAbility "Bushido"
                             (Just (ParamNumber (LetterVal X))))
                  , Define X (CountOf Attacking) ])

-- THE REPEATED-PAYMENT OFFER: one payment offered a number of times
-- inside one resolution [CR#702.56a], and the reflexive trigger it seats
-- [CR#603.12a]. 6 supported cards -- the five Adversaries at "any number
-- of times" and Tranquil Frillback at "up to three times".

||| Tainted Adversary's offer and its reflexive trigger -- "When this
||| creature enters, you may pay {2}{B} any number of times. When you pay
||| this cost one or more times, put that many +1/+1 counters on this
||| creature." The count the repeated payment leaves is what "that many"
||| reads, and [CR#603.12a] is why the trigger fires ONCE however many
||| payments were made: the reflexive seat restates the offer rather than
||| naming an iteration. (The card's second clause creates twice that
||| many Zombie tokens with decayed and is not this row's.)
public export
taintedAdversaryOffer : Ability
taintedAdversaryOffer =
  Macros.triggered When (Enters Macros.thisCreature Nothing)
    (Reflexively
       (Macros.may You (Pay You (Mana [Macros.generic 2, Macros.pip Black])
                            AnyNumberOfTimes))
       (PutCounters ThatMuch (PrintedKind Macros.plusOnePlusOne)
                    Macros.thisCreature))

-- THE UN-KEYWORDED ADDITIONAL COST [CR#118.8], the row the 12 measured
-- "if this spell's additional cost was paid" lines were waiting on. 315
-- supported lines write the frame; 308 write it of THIS spell (260
-- mandatory, 48 under "you may"), and the other 7 name a class of
-- spells, which is the subject slot this row has none of.

||| Voltage Surge's declaration -- "As an additional cost to cast this
||| spell, you may sacrifice an artifact." The OFFERED half of the row
||| [CR#118.8b], and the plainest cost in the family: 260 of the 308 self
||| lines are mandatory and this is one of the 48 written with "you may".
public export
voltageSurgeAddedCost : Ability
voltageSurgeAddedCost =
  Static (AddedCost (Do (Macros.sacrifice You (Macros.a Macros.artifact))) True)

||| Requiting Hex's read -- "If this spell's additional cost was paid,
||| you gain 2 life." The fourth `PaidCostName` arm's witness: the read
||| has no word to name, so it sorts by "the additional cost" exactly as
||| Baleful Mastery's sorts by "the alternative" one. 12 supported lines
||| write it. (The card's own additional cost is "you may blight 1", a
||| keyword action this grammar has no word for; the two halves are
||| benched apart for that reason and Voltage Surge above supplies the
||| declaration.)
public export
requitingHexAdditionalRead : Ability
requitingHexAdditionalRead =
  Spell (If (Matches This (PaidCost TheAdditional Nothing))
            (Macros.gainsLife You (Lit 2)) Nothing)

||| Burn at the Stake's additional cost -- "As an additional cost to cast
||| this spell, tap any number of untapped creatures you control." The
||| mandatory half of the row, at the any-number quantity. What the card
||| still waits on is its damage line: "three times the number of
||| creatures tapped this way" reads the stamp the COST's action left,
||| and `staticChoiceDelta` exports a chooser across the ability
||| boundary and not a whole delta.
public export
burnAtTheStakeAddedCost : Ability
burnAtTheStakeAddedCost =
  Static (AddedCost (Do (SetStatus Tapped
                           (CountedGroup Macros.anyNumber Nothing
                              (And [Macros.creature, ControlledBy You,
                                    HasStatus Untapped])))) False)

||| Caller of the Hunt, whole -- "As an additional cost to cast this
||| spell, choose a creature type. / Caller of the Hunt's power and
||| toughness are each equal to the number of creatures of the chosen
||| type on the battlefield." THE additional-cost chooser position: the
||| choice is announced as the cost is paid [CR#118.8a,601.2b] and the
||| defining ability reads it as [CR#607.2d]'s linked ability.
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
                   (CountOf (And [Macros.creature,
                                  OfChosen (SubtypeQ Creature)]))) ]
       (Just (PtBox PrintedStar PrintedStar))

-- The ALTERNATIVE-cost words read back the same way the additional-cost
-- ones do: [CR#607.2i] links the offering ability to the reading one and
-- the read names WHICH cost, so nothing about `PaidCost` changes when
-- the word offers a cost instead of adding one. What the four cards
-- below buy is the catalog rows -- prowl [CR#702.76a], surge
-- [CR#702.117a], spectacle [CR#702.137a] -- each a static ability on the
-- stack reading "You may pay [cost] rather than pay this spell's mana
-- cost if [something happened this turn]".

||| Latchkey Faerie -- "Flying / Prowl {2}{U} / When this creature
||| enters, if its prowl cost was paid, draw a card." The whole card, and
||| the plainest alternative-cost readback there is: the keyword line
||| offers the cost and the intervening-if reads back the declaration.
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
           Macros.drawACard ]
       (Just (3, 1))

||| Tyrant of Valakut -- "Surge {3}{R}{R} / Flying / When this creature
||| enters, if its surge cost was paid, it deals 3 damage to any target."
||| One of Fall of the Titans' ten surge siblings, whole.
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

||| Rafter Demon -- "Spectacle {3}{B}{R} / When this creature enters, if
||| its spectacle cost was paid, each opponent discards a card." Whole.
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
           (Macros.discardsACard (Each Opponent)) ]
       (Just (4, 2))

||| Fall of the Titans -- "Surge {X}{R} / Fall of the Titans deals X
||| damage to each of up to two targets." The payment test the round was
||| scoped against, whole. Its X is the CARD's: the printed mana cost is
||| {X}{X}{R}, so `costLetters` opens the letter before any line is read,
||| and [CR#107.3a] gives the surge cost's X and the mana cost's X the
||| one announced value ([CR#107.3i]: all instances of X on an object
||| have the same value). So the keyword line's cost parameter reads a
||| letter it does not have to bind, and no letter flows out of a keyword
||| line -- `abIntro (KeywordAbility _ _) = bs` stands.
||| The damage clause alone was already benched as `fallOfTheTitans`
||| above; this is the whole card the surge row completes.
public export
fallOfTheTitansCard : Card
fallOfTheTitansCard =
  Macros.card "Fall of the Titans"
       (Just [Variable, Variable, Macros.pip Red]) []
       (MkTypeLine [] [Instant])
       [ Macros.keywordCosting "Surge" (Mana [Variable, Macros.pip Red])
       , Spell (DealDamage This (LetterVal X)
                  (EachOf (TargetGroup (Macros.upTo 2) Macros.anyTarget))) ]
       Nothing

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
  Sequentially [ Macros.choose (CountedGroup (Macros.upTo 1) Nothing Macros.creature)
               , Macros.destroy TheRest ]

public export
duneblastCard : Card
duneblastCard =
  Macros.card "Duneblast"
       (Just [Macros.generic 4, Macros.pip White, Macros.pip Black,
              Macros.pip Green])
       [] (MkTypeLine [] [Sorcery]) [Spell Cards.duneblast] Nothing

||| Celebrate the Harvest, whole -- "Search your library for up to X
||| basic land cards, where X is the number of different powers among
||| creatures you control. Put those cards onto the battlefield tapped,
||| then shuffle." The COUNTED search: the count is the search clause's
||| own slot, and what it announces is plural, so the destination
||| sentence names the batch as "those cards". [CR#701.23e] keeps that
||| sentence separate, which is why the search row carries no destination
||| of its own.
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
                                (AllOf Macros.creatureYouControl))
                  , Macros.putOntoBattlefieldTapped (Those CardW)
                  , Macros.shuffle ]) ]
       Nothing

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
||| lands than you".
public export
boreasChargerChoice : Effect []
boreasChargerChoice = Macros.choose (Macros.a Cards.opponentWithMoreLands)

||| ...and the margin that choice leaves readable, which is what the
||| search clause spends as its count.
public export
boreasChargerDifference : Amount (effIntro Cards.boreasChargerChoice)
boreasChargerDifference = TheDifference

||| Boreas Charger's spell text, whole -- "choose an opponent who
||| controls more lands than you. Search your library for a number of
||| Plains cards equal to the difference, reveal those cards, put one of
||| them onto the battlefield tapped and the rest into your hand, then
||| shuffle." The counted search at its hardest: the count is an AMOUNT
||| the earlier clause left readable, and the plural mention the search
||| announces is what the three sentences after it partition -- "those
||| cards", "one of them", "the rest".
||| The printed trigger frame ("When this creature leaves the
||| battlefield, …") is elided: `opponentWithMoreLands` reads its own
||| domain member back as "they", which pins it to the empty prefix.
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
       , Macros.triggered When (Enters Macros.thisCreature Nothing)
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
  Macros.triggered When (Macros.leavesBattlefield Macros.thisCreature)
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
    (Macros.triggered When (Enters Macros.thisCreature Nothing)
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
       [ PutCounters (LetterVal X) (PrintedKind Macros.plusOnePlusOne) Macros.thisCreature
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

||| Bloom Tender's mana ability -- "Vivid — {T}: For each color among
||| permanents you control, add one mana of that color." The distributive
||| pass at the colour axis, and the reason it is not a counted
||| iteration: "of that color" reads the value the pass bound, which a
||| count discards. Faeburrow Elder's second line is the same ability
||| without the ability word.
public export
bloomTenderMana : Ability
bloomTenderMana =
  AbilityWord Vivid
    (Macros.activated TapSymbol
       (ForEachKindOf ColorAxis
          (Just (AllOf (And [Permanent, ControlledBy You]))) Color
          (AddMana You (Lit 1) (OfChosenColor Nothing) [])))

||| Faeburrow Elder's pump -- "This creature gets +1/+1 for each color
||| among permanents you control." The SCALING reading of the same words
||| as `bloomTenderMana`, on the same card: no pass and no bound value,
||| the distinct count under `Times`.
public export
faeburrowElderPump : Ability
faeburrowElderPump =
  Static (Gets Macros.thisCreature
               (PtUp (Times 1 (DistinctCount ColorAxis
                                 (AllOf (And [Permanent, ControlledBy You])))))
               (PtUp (Times 1 (DistinctCount ColorAxis
                                 (AllOf (And [Permanent, ControlledBy You]))))))

||| Tarnation Vista's second mana ability -- "{1}, {T}: For each color
||| among monocolored permanents you control, add one mana of that
||| color." The pass's domain narrowed by a predicate [CR#105.2a], which
||| is where every restriction on these lines lives.
public export
tarnationVistaMana : Ability
tarnationVistaMana =
  Macros.activated
    (Compound [Mana [Macros.generic 1], TapSymbol])
    (ForEachKindOf ColorAxis
       (Just (AllOf (And [Permanent, Monocolored, ControlledBy You]))) Color
       (AddMana You (Lit 1) (OfChosenColor Nothing) []))

||| Rogues' Gallery -- "For each color, return up to one target creature
||| card of that color from your graveyard to your hand." The DOMAINLESS
||| pass, whole: no group supplies the values, so it runs over
||| [CR#105.1]'s five colours themselves. All Suns' Dawn writes the same
||| sentence over cards rather than creature cards.
public export
roguesGallery : Card
roguesGallery =
  Macros.card "Rogues' Gallery" (Just [Macros.generic 2, Macros.pip Black]) []
       (MkTypeLine [] [Sorcery])
       [ Spell (ForEachKindOf ColorAxis Nothing Color
                  (Macros.move (TargetGroup (Macros.upTo 1)
                                  (And [Macros.creature, OfChosen Color,
                                        InZone (Macros.graveyardOf You)]))
                               Macros.handZ)) ]
       Nothing

||| Celestial Judgment's pass -- "For each different power among creatures
||| on the battlefield, choose a creature with that power." The value axis
||| binding a NUMBER and the body reading it back: "with that power" is a
||| comparison against the bound value, which is the amount seat, not
||| [CR#109.3]'s object-side match `chosenQualityReadOk Number = False`
||| still refuses. The sentence that follows it -- "Destroy each creature
||| not chosen this way" -- wants a negated verb-stamped read and is not
||| this cell.
public export
celestialJudgmentPass : Effect []
celestialJudgmentPass =
  ForEachKindOf (ValueAxis Power) (Just (AllOf Macros.creature)) Number
    (Macros.choose (Macros.a (And [Macros.creature,
                                   Compare [CharAxis Power] Eq ChosenNumber])))

||| World Queller's upkeep trigger -- "you may choose a card type. If you
||| do, each player sacrifices a permanent of their choice of that type."
||| The card-type sort's read: "of that type" is [CR#109.3]'s
||| characteristic on the permanent, which is why `OfChosen` takes it.
public export
worldQuellerChoice : Effect []
worldQuellerChoice =
  Macros.mayThen You (Macros.choose (Macros.a (Macros.quality CardTypeQ)))
              (Macros.sacrifice (Each AnyPlayer)
                 (Macros.aTheirChoice (And [Permanent, OfChosen CardTypeQ])))

||| Niv-Mizzet, Guildpact's combat trigger -- "Whenever Niv-Mizzet deals
||| combat damage to a player, it deals X damage to any target, target
||| player draws X cards, and you gain X life, where X is the number of
||| different color pairs among permanents you control that are exactly
||| two colors." The pair axis [CR#105.5] with the restriction the card
||| writes on its own domain.
public export
nivMizzetGuildpactTrigger : Ability
nivMizzetGuildpactTrigger =
  Macros.triggered Whenever
    (DealsCombatDamage Macros.thisCreature (Macros.a AnyPlayer))
    (Sequentially
       [ DealDamage Macros.thisCreature (LetterVal X)
                    (Macros.target Macros.anyTarget)
       , Draw (Macros.target AnyPlayer) (LetterVal X)
       , ChangeLife You (Up (LetterVal X))
       , Define X (DistinctCount ColorPairAxis
                     (AllOf (And [Permanent, ControlledBy You,
                                  ExactlyColors 2]))) ])

||| Tourach, Dread Cantor's discard trigger -- "Whenever an opponent
||| discards a card, put a +1/+1 counter on Tourach." The verbed event's
||| active voice, and the family's cheapest whole line: the act is named
||| by the verb [CR#701.9a] and the header announces its actor.
public export
tourachDiscardTrigger : Ability
tourachDiscardTrigger =
  Macros.triggered Whenever
    (VerbedEvent (Just Macros.anOpponent) "Discard"
                 (Just (Macros.a (InZone Macros.handZ))) False)
    (PutCounters (Lit 1) (PrintedKind Macros.plusOnePlusOne) Macros.thisCreature)

||| All-Seeing Arbiter's header -- "Whenever you discard a card, …", the
||| routed line that motivated the row. Its BODY does not write: "target
||| creature an opponent controls gets -X/-0 until your next turn, where X
||| is the number of different mana values among cards in your graveyard"
||| needs the distinct-kind count over mana values, which is ledgered on
||| `workbench-distinct-kind-count`. The card's blocker, not the row's.
public export
allSeeingArbiterHeader : GameEvent []
allSeeingArbiterHeader =
  VerbedEvent (Just You) "Discard" (Just (Macros.a (InZone Macros.handZ))) False

||| Mirelurk Queen's mill trigger -- "Whenever one or more nonland cards
||| are milled, draw a card, then put a +1/+1 counter on this creature.
||| This ability triggers only once each turn." The verbed event's passive
||| voice: [CR#701.17a] makes milling a player's act, and the printed line
||| names no actor at all, so the patient is the surface subject.
public export
mirelurkQueenTrigger : Ability
mirelurkQueenTrigger =
  Macros.triggeredOnlyOnce Whenever
    (VerbedEvent Nothing "Mill"
                 (Just (CountedGroup (Macros.atLeast 1) Nothing
                                     (And [Not Macros.land,
                                           InZone (ZoneAt Library Bare)])))
                 False)
    OncePerTurn
    (Sequentially [ Macros.drawCards 1
                  , PutCounters (Lit 1) (PrintedKind Macros.plusOnePlusOne)
                                Macros.thisCreature ])

||| Liliana's Caress -- "Whenever an opponent discards a card, that player
||| loses 2 life." The round's whole card: the header is the verbed event
||| entire and the body reads back the actor it announced.
public export
lilianasCaress : Card
lilianasCaress =
  Macros.card "Liliana's Caress"
       (Just [Macros.generic 1, Macros.pip Black]) []
       (MkTypeLine [] [Enchantment])
       [ Macros.triggered Whenever
           (VerbedEvent (Just Macros.anOpponent) "Discard"
                        (Just (Macros.a (InZone Macros.handZ))) False)
           (ChangeLife They (Down (Lit 2))) ]
       Nothing

||| Scheming Aspirant -- "Whenever you proliferate, each opponent loses 2
||| life and you gain 2 life." The verbed event over a PATIENTLESS act,
||| and a whole card: [CR#701.34a] has proliferating choose its own
||| permanents and players rather than take a patient from the clause
||| that instructed it, so the header is the actor alone.
public export
schemingAspirant : Card
schemingAspirant =
  Macros.card "Scheming Aspirant"
       (Just [Macros.generic 1, Macros.pip Black]) []
       (MkTypeLine [creatureType "Phyrexian", creatureType "Advisor"]
                   [Creature])
       [ Macros.triggered Whenever
           (VerbedEvent (Just You) "Proliferate" Nothing False)
           (Sequentially [ Macros.losesLife (Each Opponent) (Lit 2)
                         , Macros.gainsLife You (Lit 2) ]) ]
       (Just (1, 3))

||| Reciprocate -- "Exile target creature that dealt damage to you this
||| turn." The dealer-side damage read at its plainest, and the whole
||| card: the described creature is what dealt the damage, "you" is what
||| took it, and [CR#120.1] gives the dealing to the object alone.
public export
reciprocate : Card
reciprocate =
  Macros.card "Reciprocate" (Just [Macros.pip White]) []
       (MkTypeLine [] [Instant])
       [ Spell (Macros.exile
                  (Macros.target
                     (And [Macros.creature,
                           Macros.happenedToInvolving DamageDealing
                                                      Lookback.ThisTurn
                                                      You]))) ]
       Nothing

||| Whirling Dervish -- "Protection from black / At the beginning of each
||| end step, if this creature dealt damage to an opponent this turn, put
||| a +1/+1 counter on it." The same event in the CONDITION frame, and
||| the other complement kind. The body writes the PRINTED "it": the
||| intervening condition announces its own subject [CR#603.4], and the
||| opponent it names is a player, so the pronoun has one object
||| candidate. Dunerider Outlaw prints the same line word for word.
public export
whirlingDervish : Card
whirlingDervish =
  Macros.card "Whirling Dervish" (Just [Macros.pip Green, Macros.pip Green]) []
       (MkTypeLine [creatureType "Human", creatureType "Monk"] [Creature])
       [ Macros.keywordQuality "Protection" (ColorIs Black)
       , Macros.triggeredIf At
           (BeginningOf EndStep (ByWord EachPlayers))
           (Macros.happenedInvolving DamageDealing Macros.thisCreature
                                     Lookback.ThisTurn Macros.anOpponent)
           (PutCounters (Lit 1) (PrintedKind Macros.plusOnePlusOne) It) ]
       (Just (1, 1))

||| Military Intelligence -- "Whenever you attack with two or more
||| creatures, draw a card." The player-subject attack's whole card, and
||| the family's plainest member: the subject is the attacking player and
||| the attackers are the complement [CR#508.3c].
public export
militaryIntelligence : Card
militaryIntelligence =
  Macros.card "Military Intelligence"
       (Just [Macros.generic 1, Macros.pip Blue]) []
       (MkTypeLine [] [Enchantment])
       [ Macros.triggered Whenever
           (AttacksWith You NoDefender
                        (CountedGroup (Macros.atLeast 2) Nothing Macros.creature))
           Macros.drawACard ]
       Nothing

||| Aurelia, the Law Above -- "Flying, vigilance, haste / Whenever a
||| player attacks with three or more creatures, you draw a card. /
||| Whenever a player attacks with five or more creatures, Aurelia deals
||| 3 damage to each of your opponents and you gain 3 life." The subject
||| written as a described player rather than "you", which is what makes
||| the seat a player one and not a second spelling of `Attacks`.
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
                        (CountedGroup (Macros.atLeast 3) Nothing Macros.creature))
           Macros.drawACard
       , Macros.triggered Whenever
           (AttacksWith (Macros.a AnyPlayer) NoDefender
                        (CountedGroup (Macros.atLeast 5) Nothing Macros.creature))
           (Sequentially [ DealDamage This (Lit 3) (Each Opponent)
                         , Macros.gainsLife You (Lit 3) ]) ]
       (Just (4, 4))

||| Tahngarth, First Mate's header alone -- "Whenever an opponent attacks
||| with one or more creatures". The routed line that motivated the row.
||| Its TAIL does not write: "choose a player or planeswalker that
||| opponent is attacking" needs a description of a defender by what
||| attacks it, and "Tahngarth is attacking that player or planeswalker"
||| an assignment of an attacker to one, both of which stayed on
||| `workbench-combat-assignment-and-forced-attack`. The card's blockers,
||| not the row's.
public export
tahngarthHeader : GameEvent []
tahngarthHeader =
  AttacksWith Macros.anOpponent NoDefender
              (CountedGroup (Macros.atLeast 1) Nothing Macros.creature)

||| Myth Unbound's header -- "Whenever your commander is put into the
||| command zone from anywhere, draw a card." The command-zone row's
||| witness: [CR#903.9a] and [CR#903.9b] both write the move as putting
||| the card INTO the command zone, so `placementDestOk` has a value to
||| admit where before it had none. The card's OTHER line, "your commander
||| costs {1} less to cast for each time it's been cast from the command
||| zone this game", does not write: it needs a cost-reduction static
||| effect, of which the tree has none, and its passive names no caster
||| where `EventCount` requires a subject noun. Commander's Insignia is
||| the same family's active voice and does write.
public export
mythUnboundTrigger : Ability
mythUnboundTrigger =
  Macros.triggered Whenever
    (Macros.putIntoFrom Macros.yourCommander Macros.commandZ FromAnywhere)
    Macros.drawACard

||| Commander's Insignia -- "Creatures you control get +1/+1 for each time
||| you've cast your commander from the command zone this game." The
||| origin rider's whole card, and the largest family that reads one back:
||| [CR#903.8]'s commander tax counts casts "from the command zone that
||| game", and 21 supported lines write the count.
public export
commandersInsignia : Card
commandersInsignia =
  Macros.card "Commander's Insignia"
       (Just [Macros.generic 2, Macros.pip White, Macros.pip White]) []
       (MkTypeLine [] [Enchantment])
       [ Static (Gets (AllOf Macros.creatureYouControl)
                      (PtUp (Macros.eventCountFrom SpellCast You
                               Lookback.ThisGame Macros.yourCommander
                               (FromZone [Macros.commandZ])))
                      (PtUp (Macros.eventCountFrom SpellCast You
                               Lookback.ThisGame Macros.yourCommander
                               (FromZone [Macros.commandZ])))) ]
       Nothing

||| Jem Lightfoote, Sky Explorer -- "Flying, vigilance. At the beginning of
||| your end step, if you haven't cast a spell from your hand this turn,
||| draw a card." The origin rider away from the command zone: the same
||| payload with the hand in it, and the cheapest of the seven supported
||| lines that read a cast's origin without needing the new zone row.
public export
jemLightfooteSkyExplorer : Card
jemLightfooteSkyExplorer =
  Macros.card "Jem Lightfoote, Sky Explorer"
       (Just [Macros.generic 2, Macros.pip White, Macros.pip Blue]) [Legendary]
       (MkTypeLine [creatureType "Human", creatureType "Scout"] [Creature])
       [ Macros.keyword "Flying"
       , Macros.keyword "Vigilance"
       , Macros.triggeredIf At (BeginningOf EndStep (ByWord Yours))
           (NotCond (Macros.happenedFrom SpellCast You Lookback.ThisTurn
                       (Macros.a Macros.spell)
                       (FromZone [Macros.handOf You])))
           Macros.drawACard ]
       (Just (3, 3))

-- ---------------------------------------------------------------------------
-- The placement's zone complement, and the negated origin
-- ---------------------------------------------------------------------------

||| Faith's Reward, whole card -- "Return to the battlefield all permanent
||| cards in your graveyard that were put there from the battlefield this
||| turn." The placement complement's RELATIVE-CLAUSE reading, the
||| family's largest: of the 50 supported instances of a placement read
||| back over a window, 28 write the arrival zone as "there" and name it
||| by the described noun's own zone. A complement that writes the origin
||| alone is that reading, so the zone the clause does write is the one
||| the row carries.
public export
faithsReward : Card
faithsReward =
  Macros.card "Faith's Reward"
       (Just [Macros.generic 3, Macros.pip White]) []
       (MkTypeLine [] [Instant])
       [ Spell (Macros.move
                  (AllOf (And [Permanent,
                               InZone (Macros.graveyardOf You),
                               HappenedTo Placement Lookback.ThisTurn
                                 (Just (FromZones
                                          (FromZone [Macros.battlefieldZ])
                                          Nothing))]))
                  Macros.battlefieldZ) ]
       Nothing

||| Ichor Shade, whole card -- "At the beginning of your end step, if an
||| artifact or creature was put into a graveyard from the battlefield
||| this turn, put a +1/+1 counter on this creature." The CONDITION
||| reading, with both ends of the move written: the arrival zone outside
||| and the origin nested in it, which is the order English writes them.
public export
ichorShade : Card
ichorShade =
  Macros.card "Ichor Shade"
       (Just [Macros.generic 2, Macros.pip Black]) []
       (MkTypeLine [creatureType "Phyrexian", creatureType "Shade"] [Creature])
       [ Macros.triggeredIf At (BeginningOf EndStep (ByWord Yours))
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

||| Asmira, Holy Avenger, whole card -- "Flying. At the beginning of each
||| end step, put a +1/+1 counter on Asmira for each creature put into
||| your graveyard from the battlefield this turn." The COUNT reading of
||| the same complement.
public export
asmiraHolyAvenger : Card
asmiraHolyAvenger =
  Macros.card "Asmira, Holy Avenger"
       (Just [Macros.generic 2, Macros.pip Green, Macros.pip White]) [Legendary]
       (MkTypeLine [creatureType "Human", creatureType "Cleric"] [Creature])
       [ Macros.keyword "Flying"
       , Macros.triggered At (BeginningOf EndStep (ByWord EachPlayers))
           (PutCounters (EventCount Placement (Macros.a Macros.creature)
                           Lookback.ThisTurn
                           (Just (IntoZone (Macros.graveyardOf You)
                                    (Just (FromZones
                                             (FromZone [Macros.battlefieldZ])
                                             Nothing)))))
                        (PrintedKind Macros.plusOnePlusOne)
                        Macros.thisCreature) ]
       (Just (2, 3))

||| Syr Konrad, the Grim, whole card -- "Whenever another creature dies,
||| or a creature card is put into a graveyard from anywhere other than
||| the battlefield, or a creature card leaves your graveyard, Syr Konrad
||| deals 1 damage to each opponent. {1}{B}: Each player mills a card."
||| The corpus's only three-armed header whose tail reads nothing back,
||| and the negated origin's whole-card witness: arm 2 is the phrase that
||| names every zone but one, which the same `EventSource` carries at this
||| prospective seat and at the retrospective one.
||| The head of arm 2 is the type word where the printing writes "creature
||| card": the vocabulary has no bare card word, and a head with no zone
||| predicate is battlefield-sorted, which the unfixed origin lets pass.
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
           (DealDamage This (Lit 1) (Each Opponent))
       , Macros.activated (Mana [Macros.generic 1, Macros.pip Black])
           (Macros.mills (Each AnyPlayer) (Lit 1) (Each AnyPlayer)) ]
       (Just (5, 4))

||| Oscorp Industries' second line -- "When this land enters from a
||| graveyard, you lose 2 life." The entry origin's prospective seat: the
||| "enters from" family is 13 supported lines, and this is its cheapest.
||| The card is not whole -- its mana ability and its Mayhem keyword are
||| both unbuilt -- so the line alone benches.
public export
oscorpIndustriesReturn : Ability
oscorpIndustriesReturn =
  Macros.triggered When
    (Enters Macros.thisLand (Just (FromZone [Macros.graveyardZ])))
    (Macros.losesLife You (Lit 2))

||| The Lost and the Damned's first header arm -- "Whenever a land you
||| control enters from anywhere other than your hand". The negated origin
||| at the ENTRY seat, the same `EventSource` Syr Konrad writes at the
||| placement seat. The card is not whole: its second arm coordinates a
||| cast with an entry, and its body creates a token.
public export
theLostAndTheDamnedEntryArm : GameEvent []
theLostAndTheDamnedEntryArm =
  Enters (Macros.a (And [Macros.land, ControlledBy You]))
         (Just (FromAnywhereBut [Macros.handOf You]))

-- ---------------------------------------------------------------------------
-- The targeting relation, in both voices
-- ---------------------------------------------------------------------------

||| Gnarlback Rhino, whole card -- "Whenever you cast a spell that targets
||| this creature, draw a card." The targeting relative clause described
||| side is the spell, and the card writes no ability word to get in the
||| way of it.
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
           Macros.drawACard ]
       (Just (4, 4))

||| Forsaken Wastes's third ability -- "Whenever this enchantment becomes
||| the target of a spell, that spell's controller loses 5 life." The
||| event voice, with the targeter read back by its demonstrative.
public export
forsakenWastesTargeted : Ability
forsakenWastesTargeted =
  Macros.triggered Whenever
    (BecomesTarget Macros.thisEnchantment (Macros.a Macros.spell))
    (Macros.losesLife (ControllerOf (That SpellW)) (Lit 5))

||| Fblthp, the Lost, whole -- "When Fblthp enters, draw a card. If it
||| entered from your library or was cast from your library, draw two
||| cards instead. / When Fblthp becomes the target of a spell, shuffle
||| Fblthp into its owner's library."
||| The two disjuncts are two different readings of one arrival and
||| neither needs a new word now: "it entered from your library" is the
||| windowless lookback `Triggering` [CR#603.2c], scoped to the entry the
||| ability triggered on, and "was cast from your library" is the
||| object's own casting history, agentless -- which `CastFrom` has
||| always been, since [CR#601.2a] names the zone the card left without
||| naming who moved it.
||| The tail is a move whose destination names no position because the
||| act that puts the card there randomizes the pile
||| [CR#701.24a,701.24c]; the library is owner-rooted [CR#400.3], so the
||| bare scope IS "its owner's".
public export
fblthp : Card
fblthp =
  Macros.card "Fblthp, the Lost" (Just [Macros.generic 1, Macros.pip Blue])
       [Legendary]
       (MkTypeLine [creatureType "Homunculus"] [Creature])
       [ Macros.triggered When (Enters This Nothing)
           (InsteadOf Macros.drawACard
              (If (OrCond
                     [ Happened Entry It Triggering
                         (Just (FromZones (FromZone [Macros.yourLibrary]) Nothing))
                     , Matches It (CastFrom Macros.yourLibrary) ])
                  (Draw You (Lit 2))
                  Nothing))
       , Macros.triggered When (BecomesTarget This (Macros.a Macros.spell))
           (Macros.shuffleInto This) ]
       (Just (1, 1))

||| Squelch, whole card -- "Counter target activated ability. Draw a
||| card." The ability TARGET: [CR#115.2] admits an object that can't
||| exist on the battlefield, "such as a spell or ability", and the head
||| picks WHICH abilities, since [CR#115.1c] gives the word "target" to
||| an activated ability. The reminder "(Mana abilities can't be
||| targeted.)" restates [CR#605.3b] -- a mana ability never reaches the
||| stack -- and is not text the card writes.
public export
squelch : Card
squelch =
  Macros.card "Squelch" (Just [Macros.generic 1, Macros.pip Blue]) []
       (MkTypeLine [] [Instant])
       [ Spell (Sequentially
                  [ Macros.counterSpell
                      (Macros.target (AbilityHead AnyActivated))
                  , Macros.drawACard ]) ]
       Nothing

||| Diplomatic Escort's line -- "{U}, {T}, Discard a card: Counter target
||| spell or ability that targets a creature." The joined targeter head
||| with both halves: the object half is the spell [CR#112.1], the
||| ability half the bare word [CR#115.2], and the relative clause
||| describes the pair by what it targets -- [CR#115.9b]'s own "[spell or
||| ability] that targets [something]", whose `Targeter` gate the join
||| passes on [CR#115.1a] and [CR#115.1c,115.1d] together.
public export
diplomaticEscortLine : Ability
diplomaticEscortLine =
  Macros.activated (Compound [ Mana [Macros.pip Blue]
                             , TapSymbol
                             , Do (Macros.discardsACard You) ])
    (Macros.counterSpell
       (Macros.target
          (And [ Joined Macros.spell (AbilityHead AnyOnStack)
               , Targets (Macros.a Macros.creature) SomeTarget ])))

||| Shimmering Glasskite, whole card -- "Whenever this creature becomes
||| the target of a spell or ability for the first time each turn,
||| counter that spell or ability." The coordinated anaphor: the header
||| announces its targeter as ONE union mention [CR#115.1], and the body
||| reads that mention back whole with the joined demonstrative rather
||| than naming a half. Countering the read is [CR#701.6a] over both of
||| the stack's inhabitants at once.
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

||| Frost Walker, whole card -- "When this creature becomes the target
||| of a spell or ability, sacrifice it." The bare pronoun read at the
||| carrier its verb demands: the header announces the targeting spell as
||| well as the creature [CR#115.1], and [CR#701.21a] lets a player
||| sacrifice only a permanent, so one of the two candidates is in the
||| slot's carrier and the count is 1. 19 supported lines write this
||| sentence; the other 18 differ only in the permanent word and in what
||| rides beside it.
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

||| Destructive Revelry, whole card -- "Destroy target artifact or
||| enchantment. Destructive Revelry deals 2 damage to that permanent's
||| controller." The permanent word after the zone change its own clause
||| caused: [CR#110.1] stops the object being a permanent as it leaves the
||| battlefield, and [CR#608.2h] is why the later clause still reads it --
||| by last known information. 15 supported lines write this shape.
public export
destructiveRevelry : Card
destructiveRevelry =
  Macros.card "Destructive Revelry" (Just [Macros.pip Red, Macros.pip Green]) []
       (MkTypeLine [] [Instant])
       [ Spell (Sequentially
                  [ Macros.destroy (Macros.target (Or [Macros.artifact, Macros.enchantment]))
                  , DealDamage This (Lit 2) (ControllerOf (That PermanentW)) ]) ]
       Nothing

||| Bioplasm's prefix at its second sentence -- "Whenever this creature
||| attacks, exile the top card of your library. If it's a creature card,
||| …". Two singular object mentions stand there.
public export
bioplasmAfterExile : Bindings
bioplasmAfterExile =
  effIntro {bs = eventAfter {bs = []} (Attacks Macros.thisCreature NoDefender)}
           (Macros.exile Macros.topCard)

||| ...which is why the bare pronoun is refused there and the card-carrier
||| read is not: [CR#109.2] takes a description including the word "card"
||| off the battlefield, and the attacking creature is on the battlefield.
public export
bioplasmTwoCandidates : countOnes Object Cards.bioplasmAfterExile = 2
bioplasmTwoCandidates = Refl

public export
bioplasmCardTest : Condition Cards.bioplasmAfterExile
bioplasmCardTest = Macros.itsACard Macros.creature

||| Fuming Effigy, whole card -- "Whenever one or more cards leave your
||| graveyard, this creature deals 1 damage to each opponent." The
||| leave-event off the battlefield, which [CR#603.10a] names beside the
||| leaves-the-battlefield ability.
public export
fumingEffigy : Card
fumingEffigy =
  Macros.card "Fuming Effigy"
       (Just [Macros.generic 3, Macros.pip Red]) []
       (MkTypeLine [creatureType "Spirit"] [Creature])
       [ Macros.triggered Whenever
           (Macros.leavesZone
              (CountedGroup (Macros.atLeast 1) Nothing (InZone (Macros.graveyardOf You)))
              (Macros.graveyardOf You))
           (DealDamage This (Lit 1) (Each Opponent)) ]
       (Just (4, 3))

||| The Fallen's whole line -- "At the beginning of your upkeep, this
||| creature deals 1 damage to each opponent and planeswalker it has
||| dealt damage to this game." The victim-side lookback described at a
||| JOINED kind: [CR#120.1] gives the damage to a player or a
||| planeswalker alike, and one clause names both halves at once.
public export
theFallenUpkeep : Ability
theFallenUpkeep =
  Macros.triggered At (BeginningOf Upkeep (ByWord Yours))
    (DealDamage This (Lit 1)
       (Each (And [ Joined (HasType Planeswalker) Opponent
                  , HappenedTo DamageTaken ThisGame (Just (Involving This)) ])))


-- ---------------------------------------------------------------------------
-- The shuffle-into-library move
-- ---------------------------------------------------------------------------

||| Loaming Shaman, whole card -- "When this creature enters, target
||| player shuffles any number of target cards from their graveyard into
||| their library." [CR#701.24d]'s own example: the set may turn out
||| empty and the library is shuffled anyway, which is the rule's
||| business and not the clause's, so the clause is the move and nothing
||| more.
public export
loamingShaman : Card
loamingShaman =
  Macros.card "Loaming Shaman"
       (Just [Macros.generic 2, Macros.pip Green]) []
       (MkTypeLine [creatureType "Centaur", creatureType "Shaman"] [Creature])
       [ Macros.triggered When (Enters Macros.thisCreature Nothing)
           (Macros.shufflesInto (Macros.target AnyPlayer)
              (TargetGroup Macros.anyNumber
                 (InZone (Macros.graveyardOf They)))) ]
       (Just (3, 2))

||| Blessed Respite's first line -- "Target player shuffles their
||| graveyard into their library." The MASS form: [CR#400.12] reads an
||| instruction given to a zone as the same instruction given to all the
||| cards in it, so the zone word is spelling and the moved thing is the
||| cards.
public export
blessedRespiteShuffle : Effect []
blessedRespiteShuffle =
  Macros.shufflesInto (Macros.target AnyPlayer)
    (AllOf (InZone (Macros.graveyardOf They)))


-- ---------------------------------------------------------------------------
-- The replacement side of a create clause
-- ---------------------------------------------------------------------------

||| Conqueror's Pledge, whole -- "Kicker {6} / Create six 1/1 white Kor
||| Soldier creature tokens. If this spell was kicked, create twelve of
||| those tokens instead." The DEFINITION CHANNEL at its plainest:
||| [CR#614.6] makes the replaced creation never happen, so the tokens
||| the first clause would have made are not objects the second clause
||| can name -- but the characteristics it wrote are still written, and
||| that definition [CR#111.3] is what "those tokens" reads. `replacedCtx`
||| threads it: `deedDelta (Create ...)` mints the token-origin binding
||| `countTokenSpecs` counts.
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

||| Prismari Pianist, whole -- "Whenever you cast an instant or sorcery
||| spell, create a 1/1 blue and red Elemental creature token. If that
||| spell's mana value is 5 or greater, create three of those tokens
||| instead." The SINGULAR antecedent: one token was written, and one
||| token defines characteristics exactly as a batch does, so the anaphor
||| finds its definition. `countTokenSpecs` used to demand a plural
||| antecedent and refused this line; Mr. House writes the other half of
||| the same fact, the singular ANAPHOR ("instead create that token"),
||| and waits on the coordination of two specifications.
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

||| Moonlit Meditation, whole -- "Enchant artifact or creature you control
||| / The first time you would create one or more tokens each turn, you
||| may instead create that many tokens that are copies of enchanted
||| permanent." The CAP on a standing replacement: [CR#614.3] says how
||| long the effect lasts -- here as long as the Aura is on the
||| battlefield -- and the rider says how OFTEN it may apply while it
||| stands, which is the trigger rider's own word (`UsageLimit`) rather
||| than a third `ReplUse` ending. Esix, Fractal Bloom and Mirrormind
||| Crown write the same clause behind a chooser and an attachment
||| condition.
public export
moonlitMeditation : Card
moonlitMeditation =
  Macros.card "Moonlit Meditation" (Just [Macros.generic 2, Macros.pip Blue]) []
       (MkTypeLine [enchantmentType "Aura"] [Enchantment])
       [ Macros.keywordSubject "Enchant"
           (And [Or [Macros.artifact, Macros.creature], ControlledBy You])
       , Static (Intercepts
                   (TokensCreated (CountedGroup (Macros.atLeast 1) Nothing IsToken)
                                  Nothing (Just You) Nothing)
                   [] Nothing
                   (May (Just You)
                        (Create You GroupSize
                                (TokenCopyOf (AttachHost Enchanted PermanentW) []) [])
                        Nothing Nothing)
                   Repeatedly (Just OncePerTurn)) ]
       Nothing

||| Damn's first line -- "Destroy target creature. A creature destroyed
||| this way can't be regenerated." The PARTICIPLE subject of the rider,
||| which needs the destruction's own stamp where the rider's subject is
||| read: [CR#608.2c] reads the two sentences as one statement and takes
||| this very shape as its worked example, so `riderIntro` writes the
||| clause's label in place and the participle finds it. The rest of the
||| card is elided: its overload cost has no word in this vocabulary.
public export
damnDestroyLine : Effect []
damnDestroyLine =
  CantBe (Macros.destroy (Macros.target Macros.creature))
         "Regenerate" (Macros.theVerbedThisWay "Destroy" (TypeW Creature))

||| Nekrataal's trigger body reads its rider's subject here -- "When this
||| creature enters, destroy target nonartifact, nonblack creature. That
||| creature can't be regenerated."
public export
nekrataalRider : Bindings
nekrataalRider =
  riderIntro {bs = eventAfter {bs = []} (Enters Macros.thisCreature Nothing)}
    (Macros.destroy (Macros.target
       (And [Macros.creature, Not Macros.artifact, Not (ColorIs Black)])))

||| Two battlefield creatures stand there -- the one that entered and the
||| one the trigger destroyed -- but only ONE of them is a creature the
||| demonstrative can name: the entering creature is announced as the
||| SOURCE, under `SelfD`, and the demonstrative noun words do not read a
||| self-mention. So Nekrataal's printed "that creature" resolves, while
||| the bare pronoun below still has two candidates -- which is why the
||| printed rider spells the demonstrative and not "it".
public export
nekrataalOneCreatureWord : countWord (TypeW Creature) Cards.nekrataalRider = 1
nekrataalOneCreatureWord = Refl

public export
nekrataalTwoObjects : countOnes Object Cards.nekrataalRider = 2
nekrataalTwoObjects = Refl

||| ...and exactly one of them was DESTROYED, which is what the rider
||| means and what neither of the other two reads can ask.
public export
nekrataalOneDestroyed : countVerbedIt "Destroy" Cards.nekrataalRider = 1
nekrataalOneDestroyed = Refl

||| ...so Nekrataal's printed rider writes as printed -- "When this
||| creature enters, destroy target nonartifact, nonblack creature. That
||| creature can't be regenerated."
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

||| "Sacrifice this artifact: Exile target creature. At the beginning of
||| the next end step, return that card to the battlefield." The line
||| `Proofs.badBareCardRead` refuses once the sacrificed half is a
||| DESCRIBED creature: written with the source, the sacrificed card is
||| announced under `SelfD`, which no demonstrative noun word reads, so
||| "that card" has the exiled creature and nothing else.
public export
selfSacrificeThenExile : Ability
selfSacrificeThenExile =
  Macros.activated (Do (Macros.sacrifice You Macros.thisArtifact))
    (Sequentially
       [ Macros.exile (Macros.target Macros.creature)
       , Delayed (BeginningOf EndStep NoPossessor) [] Nothing
                 (Move (That CardW) Macros.battlefieldZ
                       (MkMoveRiders [] Nothing Nothing)) ])

||| The wrapper seam a rider is read at, in the smallest shape the
||| printed family shares: a sentence of two clauses, the rider naming
||| what the LAST one did. Harsh Mercy ("Each player chooses a creature
||| type. Destroy all creatures that aren't of a type chosen this way.
||| They can't be regenerated.") and Tsabo's Decree write it out;
||| `riderIntro` recurses into the sequence rather than falling through
||| to `preIntro`, which dropped the label at every wrapper.
public export
sequencedRider : Bindings
sequencedRider =
  riderIntro {bs = []}
    (Sequentially [ Macros.exile (Macros.target Macros.artifact)
                  , Macros.destroy (Macros.target Macros.creature) ])

||| Exactly one mention carries the destroying label -- the sequence's
||| last clause -- and the exiled artifact carries its own.
public export
sequencedRiderOneDestroyed : countVerbedIt "Destroy" Cards.sequencedRider = 1
sequencedRiderOneDestroyed = Refl

public export
sequencedRiderOneExiled : countVerbedIt "Exile" Cards.sequencedRider = 1
sequencedRiderOneExiled = Refl

||| Bonds of Faith's pump line -- "Enchanted creature gets +2/+2 as long
||| as it's a Human." The POSTPOSED static conditional reading its own
||| statement's subject: `staticIntro` announces the attachment's host, so
||| the trailing condition says "it". The card's second sentence
||| ("Otherwise, it can't attack or block.") is a second statement and is
||| not this row's.
public export
bondsOfFaithPump : Ability
bondsOfFaithPump =
  Static (Macros.onlyWhile
            (Gets (AttachHost Enchanted (TypeW Creature)) (PtUp (Lit 2)) (PtUp (Lit 2)))
            (Matches It (HasSubtype (creatureType "Human"))))

||| "Discard up to two cards, then draw that many cards" -- 21 supported
||| occurrences, the commonest shape of the announced-magnitude family.
||| The iterated-singular discard exports its passes' batch, and the draw
||| reads that batch's size; the ceiling stays on the `UpTo` amount, where
||| a bare-number ceiling belongs.
public export
discardUpToTwoThenDrawThatMany : Effect []
discardUpToTwoThenDrawThatMany =
  Sequentially [ Macros.discardN (UpTo (Lit 2))
               , Draw You GroupSize ]

||| Soul of Emancipation, whole -- "When this creature enters, destroy up
||| to three other target nonland permanents. For each of those
||| permanents, its controller creates a 3/3 white Angel creature token
||| with flying." The loop MEMBER carries the group's stamp, so the body
||| names it by the label that destroyed it; the trigger's own subject is
||| announced under `SelfD` and is no candidate for that read.
public export
soulOfEmancipation : Ability
soulOfEmancipation =
  Macros.triggered When (Enters Macros.thisCreature Nothing)
    (Sequentially
       [ Macros.destroy (TargetGroup (Macros.upTo 3)
                           (And [Permanent, Not Macros.land, OtherThan This]))
       , ForEachOf (Macros.thoseVerbedThisWay "Destroy" PermanentW)
                   (Create (ControllerOf (ItVerbed "Destroy")) (Lit 1)
                           (TokenWritten
                              (MkToken (Just (Lit 3 ** Lit 3)) [White]
                                       (MkTypeLine [creatureType "Angel"] [Creature])
                                       [Macros.keyword "Flying"] Nothing))
                           []) ])

||| Engulfing Flames' rider reads here -- "Engulfing Flames deals 1
||| damage to target creature. It can't be regenerated this turn."
public export
engulfingFlamesRider : Bindings
engulfingFlamesRider =
  riderIntro {bs = []} (DealDamage This (Lit 1) (Macros.target Macros.creature))

||| A damage clause destroys nothing itself -- [CR#704.5g] destroys the
||| lethally damaged creature as a state-based action, and regeneration
||| replaces THAT event -- so no destroy stamp stands where the rider is
||| read and the verb-scoped pronoun finds nothing.
public export
engulfingFlamesNoDestroyStamp :
  countVerbedIt "Destroy" Cards.engulfingFlamesRider = 0
engulfingFlamesNoDestroyStamp = Refl

||| The bare pronoun still resolves there, which is the overgeneration
||| recorded at chapter 125 and re-measured here at 9 occurrences over 9
||| cards, all of them carrying "this turn". It is NOT pinned:
||| [CR#704.5g] says regeneration can replace the destruction lethal
||| damage causes, and [CR#701.19c] makes the denial a shield-application
||| denial, so a rider after a damage clause is rules-meaningful. What
||| the round buys is that the distinction is now WRITABLE -- the 138
||| attached riders name their own destroying label -- not that the
||| unnamed form is refused.
public export
engulfingFlamesBareReadStands :
  countOnes Object Cards.engulfingFlamesRider = 1
engulfingFlamesBareReadStands = Refl

||| Bioplasm's exiled card, read back by the verb that exiled it. The
||| CARD word resolves and so does the verb-scoped pronoun; what does not
||| is the card's own spelling, "the exiled creature card".
public export
bioplasmExiledCard : Noun Cards.bioplasmAfterExile Object
bioplasmExiledCard = Macros.theVerbed "Exile" CardW

public export
bioplasmExiledPronoun : Noun Cards.bioplasmAfterExile Object
bioplasmExiledPronoun = Macros.itVerbed "Exile"

||| ...and the type word finds nothing, for TWO reasons and not the one
||| sub-round A's close named. `verbedWordOk (TypeW t)` asks the stamp
||| for `wasField`, which a card taken off a library never carries; and
||| the mention records NO card type at all, because "the top card of
||| your library" names none and the "if it's a creature card" test that
||| follows does not re-mark the binding it tested. The second is an
||| announcement question, not a provenance one.
public export
bioplasmNoTypedRead : countVerbed "Exile" (TypeW Creature) Cards.bioplasmAfterExile = 0
bioplasmNoTypedRead = Refl

public export
bioplasmExiledCardHasNoType : tyOfVerbedIt "Exile" Cards.bioplasmAfterExile = Nothing
bioplasmExiledCardHasNoType = Refl

||| ...until the test itself is read. "If it's a creature card" is a fact
||| about the mention the clause before it exiled, and the consequent is
||| typed at the prefix the test left MARKED, so the type the card had no
||| word for stands there [CR#608.2c].
public export
bioplasmAfterTest : Bindings
bioplasmAfterTest = condIntro Cards.bioplasmCardTest

public export
bioplasmTestRemarksType :
  tyOfVerbedIt "Exile" Cards.bioplasmAfterTest = Just Creature
bioplasmTestRemarksType = Refl

||| ...and it re-marks and does not announce: the count the test's own
||| subject read is the count the consequent reads.
public export
bioplasmTestMintsNothing : countOnes Object Cards.bioplasmAfterTest = 2
bioplasmTestMintsNothing = Refl

public export
bioplasmTestKeepsCardSlot : countOnesAt CardSlot Cards.bioplasmAfterTest = 1
bioplasmTestKeepsCardSlot = Refl

||| The participle read at a TYPE word is still refused, and the re-mark
||| is not what refuses it: `verbedWordOk (TypeW t)` asks the stamp for
||| `wasField`, and a card taken off a library carries none. That gate is
||| RIGHT -- [CR#109.2] gives a bare type description the battlefield --
||| so what was missing was never a looser `TypeW` but the other rule's
||| word.
public export
bioplasmTypedReadStillRefused :
  countVerbed "Exile" (TypeW Creature) Cards.bioplasmAfterTest = 0
bioplasmTypedReadStillRefused = Refl

||| ...and "the exiled creature card" is that word: [CR#109.2a] reads a
||| description carrying a card type AND the word "card" as a card
||| matching it, which is what the exiled mention is. The re-mark is what
||| supplies the type; the zone was always the exile the clause named.
public export
bioplasmTypedCardReadWrites :
  countVerbed "Exile" (TypedCardW Creature) Cards.bioplasmAfterTest = 1
bioplasmTypedCardReadWrites = Refl

||| Scapeshift's prefix after its first sentence -- "Sacrifice any number
||| of lands." One group mention stands there, which is what lets the
||| next sentence write "that many".
public export
scapeshiftSacrificed : Bindings
scapeshiftSacrificed =
  effIntro {bs = []}
           (Macros.sacrifice You (CountedGroup Macros.anyNumber Nothing Macros.land))

||| ...and after the search clause, where the pronoun is written. TWO
||| group mentions stand: the lands the first sentence sacrificed and the
||| cards the search found.
public export
scapeshiftAfterSearch : Bindings
scapeshiftAfterSearch =
  effIntro {bs = Cards.scapeshiftSacrificed}
           (Macros.searchLibraryForCount (UpToOf GroupSize) Macros.land)

||| ...so the bare plural pronoun is refused there.
public export
scapeshiftTwoGroups : countManys Object Cards.scapeshiftAfterSearch = 2
scapeshiftTwoGroups = Refl

||| ...and the label the search left is what picks one of them: the found
||| cards carry the "Search" stamp [CR#701.23a] and the sacrificed lands
||| carry "Sacrifice", so the verb-scoped group pronoun resolves where
||| the bare one cannot.
public export
scapeshiftOneSearchedGroup : countVerbedThem "Search" Cards.scapeshiftAfterSearch = 1
scapeshiftOneSearchedGroup = Refl

public export
scapeshiftOneSacrificedGroup :
  countVerbedThem "Sacrifice" Cards.scapeshiftAfterSearch = 1
scapeshiftOneSacrificedGroup = Refl

||| Scapeshift, whole -- "Sacrifice any number of lands. Search your
||| library for up to that many land cards, put them onto the battlefield
||| tapped, then shuffle." The plural twin of the verb-scoped pronoun,
||| written where `Them` counts two groups. The search's own stamp is
||| also what keeps the found cards readable across the shuffle that
||| follows [CR#701.24b].
public export
scapeshift : Card
scapeshift =
  Macros.card "Scapeshift"
       (Just [Macros.generic 2, Macros.pip Green, Macros.pip Green]) []
       (MkTypeLine [] [Sorcery])
       [ Spell (Sequentially
                  [ Macros.sacrifice You (CountedGroup Macros.anyNumber Nothing
                                                       Macros.land)
                  , Macros.searchLibraryForCount (UpToOf GroupSize) Macros.land
                  , Macros.putOntoBattlefieldTapped (Macros.themVerbed "Search")
                  , Macros.shuffle ]) ]
       Nothing

--------------------------------------------------------------------------------
-- The partitive's second surface, the plural at-random determiner, and the
-- slice over a distributive possessor.
--------------------------------------------------------------------------------

||| Collected Company -- "Look at the top six cards of your library. Put
||| up to two creature cards with mana value 3 or less from among them
||| onto the battlefield. Put the rest on the bottom of your library in
||| any order." The described partitive whole: a count, a description of
||| the SLICE, and the group an earlier clause named. The preposition is
||| the only thing the description changes -- "of them" bare, "from among
||| them" once a head noun stands between the count and the pronoun.
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

||| Commune with the Gods -- "Reveal the top five cards of your library.
||| You may put a creature or enchantment card from among them into your
||| hand. Put the rest into your graveyard." The reveal arm of the same
||| family, with a disjunctive description on the slice.
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

||| Bind to Life, Vastlands Scavenger's adventure -- "Mill seven cards.
||| Then put a creature card from among them onto the battlefield." The
||| mill arm: the batch is the keyword action's own, and the bare group
||| pronoun reads it. Benched as the line, not the card: the whole is an
||| adventure face pair.
public export
bindToLife : Effect []
bindToLife =
  Sequentially [ Macros.mills You (Lit 7) You
               , Macros.move (Macros.oneFromAmong Macros.creature Them)
                             Macros.battlefieldZ ]

||| The same partitive over the PARTICIPLE spelling of the same batch --
||| "You may put an artifact card from among the cards milled this way
||| into your hand" (Tomakul Scrapsmith). One group mention, three
||| spellings: "them", "the milled cards", "the cards milled this way".
public export
millThenPutFromAmongMilled : Effect []
millThenPutFromAmongMilled =
  Sequentially [ Macros.mills You (Lit 3) You
               , Macros.may You
                   (Macros.move (Macros.oneFromAmong Macros.artifact
                                   (Macros.thoseVerbedThisWay "Mill" CardW))
                                Macros.handZ) ]

||| Hymn to Tourach -- "Target player discards two cards at random."
||| The plural at-random determiner: the count is the counted mention's
||| and the mode rides beside it [CR#701.9b]. A plural `Indefinite` is
||| not the alternative -- that constructor is singular by definition.
||| Sort-only in the same way every discard line on this bench is: an
||| owned-hand expansion needs a subject-read noun the vocabulary does
||| not have yet.
public export
hymnToTourach : Card
hymnToTourach =
  Macros.card "Hymn to Tourach" (Just [Macros.pip Black, Macros.pip Black]) []
       (MkTypeLine [] [Sorcery])
       [ Spell (Macros.discards (Macros.target AnyPlayer)
                                (Macros.countedAtRandom (Macros.exactly 2)
                                                        (InZone Macros.handZ))) ]
       Nothing

||| Tourach, Dread Cantor, whole -- "Kicker {B}{B} / Protection from
||| white / Whenever an opponent discards a card, put a +1/+1 counter on
||| Tourach. / When Tourach enters, if it was kicked, target opponent
||| discards two cards at random." The card the plural at-random
||| determiner was blocking: every other row it needs was already here.
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
           (Macros.discards (Macros.target Opponent)
                            (Macros.countedAtRandom (Macros.exactly 2)
                                                    (InZone Macros.handZ))) ]
       (Just (2, 1))

||| The slice over a distributive group possessor: [CR#400.1] gives each
||| player their own library, so the phrase names one card in each and
||| the ZONE word is what pluralises.
||| It is NOT what Field of Dreams, Lantern of Insight and Wizened
||| Snitches write, which this note used to say: "play with the top card
||| of their libraries revealed" is the visibility RIDER, whose
||| complement names the position and takes whose from the subject, and
||| all three cards bench that way (`fieldOfDreams` below). The slice is
||| the phrase an instruction names when it MOVES those cards.
public export
playersTopCardSlice : Noun [] Object
playersTopCardSlice = LibrarySlice OnTop (Lit 1) (PlayerGroup AllPlayers)

||| ...and the mention it writes is PLURAL, which is the whole of what
||| the singular-possessor spelling could not say.
public export
playersTopCardIsPlural : nounPlur Cards.playersTopCardSlice = ManyOf
playersTopCardIsPlural = Refl

||| Breeches, Brazen Plunderer's slice -- "exile the top card of each of
||| those opponents' libraries", the distributive partitive possessor
||| over a group the header named.
public export
eachOfThoseOpponentsTopCard : Effect []
eachOfThoseOpponentsTopCard =
  Sequentially [ DealDamage This (Lit 1) (Each Opponent)
               , Macros.exile (LibrarySlice OnTop (Lit 1) (EachOf (Those PlayerW))) ]

||| The context Collected Company's second clause reads.
public export
companyContext : Bindings
companyContext = Macros.lookedTop [] (Lit 6)

||| The described slice, and the bare one beside it.
public export
companyDescribedSlice : Noun Cards.companyContext Object
companyDescribedSlice =
  Macros.fromAmong (Macros.upTo 2)
                   (And [Macros.creature, Compare [CharAxis ManaValue] AtMost (Lit 3)])
                   Them

public export
companyBareSlice : Noun Cards.companyContext Object
companyBareSlice = Macros.someOf 2 Them

||| What the description buys: the slice reads back as a CREATURE card.
||| "The top six cards of your library" names no card type -- a library
||| is hidden and a slice projects none -- so the bare partitive over it
||| carries none either, and every later mention of the chosen cards
||| would find an untyped batch.
public export
describedSliceReadsAsCreature : nounTy Cards.companyDescribedSlice = Just Creature
describedSliceReadsAsCreature = Refl

public export
bareSliceReadsUntyped : nounTy Cards.companyBareSlice = Nothing
bareSliceReadsUntyped = Refl

||| The slice stays where the GROUP is, whatever the description says:
||| "from among them" fills the slot where [CR#109.2a] would otherwise
||| read a zone name, and the description only tests the members.
public export
describedSliceKeepsGroupZone :
  nounZone Cards.companyDescribedSlice = nounZone Cards.companyBareSlice
describedSliceKeepsGroupZone = Refl

||| The at-random mode is orthogonal to the announcement: the counted
||| mention binds the same way with the mode written and without, which
||| is why the slot rides `CountedGroup` rather than replacing its
||| determiner.
public export
atRandomModeIsAnnouncementNeutral :
  nounDelta (Macros.countedAtRandom {bs = []} (Macros.exactly 2) (InZone Macros.handZ))
    = nounDelta (CountedGroup {bs = []} (Macros.exactly 2) Nothing (InZone Macros.handZ))
atRandomModeIsAnnouncementNeutral = Refl

||| ...and it stays plural, which is what a mode-carrying `Indefinite`
||| could not be.
public export
countedAtRandomIsPlural :
  nounPlur (Macros.countedAtRandom {bs = []} (Macros.exactly 2) (InZone Macros.handZ))
    = ManyOf
countedAtRandomIsPlural = Refl

||| Lord of the Void's body -- "exile the top seven cards of that
||| player's library, then put a creature card from among them onto the
||| battlefield under your control". The exile arm of the same family.
||| Written here over your own library: the card's possessor read and its
||| "under your control" rider are separate asks, and neither is what the
||| partitive was blocking.
public export
exileTopThenPutFromAmong : Effect []
exileTopThenPutFromAmong =
  Sequentially [ Macros.exile (Macros.topCards 7)
               , Macros.move (Macros.oneFromAmong Macros.creature Them)
                             Macros.battlefieldZ ]

--------------------------------------------------------------------------------
-- The self-reading counter row's multiplicative twin.
--------------------------------------------------------------------------------

||| Vorel of the Hull Clade, whole -- "{G}{U}, {T}: Double the number of
||| each kind of counter on target artifact, creature, or land." The
||| MULTIPLICATIVE self-reading distributive at a single recipient: the
||| permanent's own counters are both the kinds and the counts, and the
||| clause names neither.
public export
vorelOfTheHullClade : Card
vorelOfTheHullClade =
  Macros.card "Vorel of the Hull Clade"
       (Just [Macros.generic 1, Macros.pip Green, Macros.pip Blue]) [Legendary]
       (MkTypeLine [creatureType "Human", creatureType "Merfolk"] [Creature])
       [ Macros.activated
           (Compound [Mana [Macros.pip Green, Macros.pip Blue], TapSymbol])
           (DoubleCountersOfOwnKinds
              (Macros.target (Or [Macros.artifact, Macros.creature, Macros.land]))) ]
       (Just (1, 4))

||| Deepglow Skate's recipient -- "any number of target permanents" --
||| is refused here, and not by anything this row decided: `PerMember` is
||| every counter row's gate, and a bare plural target group fails it
||| exactly as it fails `PutCounters`'. The printed lines that distribute
||| a counter operation over a group write the word ("on each of up to
||| four target creatures"), and this one distributes without it. The
||| measurement, recorded rather than worked around: of the eleven
||| supported doubling lines, ten name a single recipient and this is
||| the one that does not.
public export
deepglowSkateRecipientRefused :
  perMemberOk (TargetGroup {bs = []} Macros.anyNumber Permanent) = False
deepglowSkateRecipientRefused = Refl

||| Aetheric Amplifier's second mode -- "Double the number of each kind of
||| counter you have". The player seat of the same row: [CR#122.1] places
||| a counter on an object OR a player, so the doubling has kinds to
||| range over at either seat and the union index is not a courtesy.
public export
doubleYourOwnCounters : Effect []
doubleYourOwnCounters = DoubleCountersOfOwnKinds You

--------------------------------------------------------------------------------
-- The per-member event count inside `AggregateOver`'s binder body.
--------------------------------------------------------------------------------

||| Thought Sponge's entry count -- "the greatest number of cards an
||| opponent has drawn this turn". The element binder over a described
||| domain, with an EVENT read in the body: the domain binds one opponent
||| (`TheD`, `OneOf`) and the body names that member as `They`, which is
||| exactly what `EventCount`'s subject slot takes. [CR#121.2] makes each
||| draw its own event, so the number of cards a player drew is the
||| number of drawing events with that player as subject.
public export
greatestCardsAnOpponentDrew : Amount []
greatestCardsAnOpponentDrew =
  AggregateOver MaxOf Opponent (Macros.eventCount CardDrawn They ThisTurn)

||| Thought Sponge, whole -- "Flash / This creature enters with a number
||| of +1/+1 counters on it equal to the greatest number of cards an
||| opponent has drawn this turn. / When this creature dies, draw cards
||| equal to its power."
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

||| The Windfall / Jace's Archivist cycle's read -- "the greatest number of
||| cards a player discarded this way". The same binder body at a
||| VERBED-ACT event and the "this way" window: what a labelled act did in
||| this very resolution, per member. [CR#603.2c] makes a multi-card
||| discard one event with several occurrences, which is why the count is
||| written over the cards the act named rather than over the acts.
public export
greatestCardsAPlayerDiscardedThisWay : Amount []
greatestCardsAPlayerDiscardedThisWay =
  AggregateOver MaxOf AnyPlayer
    (Macros.eventCountInvolving (VerbedAct "Discard") They ThisWay
       (AllOf {k = Object} (And [])))

--------------------------------------------------------------------------------
-- The plural read-back mention, measured.
--------------------------------------------------------------------------------

||| "Each player may scry 1" (Eager Construct) is still unwritable, and the
||| measurement says why: `Each` mints its member set at `EachD ManyOf`, so
||| the prefix a distributed body reads holds NO singular player mention,
||| and every keyword action whose rule reads one player's own library
||| gates on exactly that count ([CR#701.22a]'s scry, [CR#701.25a]'s
||| surveil). The binder shape that answers it is the one
||| `greatestCardsAnOpponentDrew` above uses -- a member bound at `TheD
||| OneOf` for the body to read back -- lifted from the AMOUNT sort, where
||| it works today, to the EFFECT sort, where a distributive agent's body
||| is typed.
public export
eachPlayerBindsNoSingular : countOnes Player (nomIntro (Each {bs = []} AnyPlayer)) = 0
eachPlayerBindsNoSingular = Refl

public export
eachPlayerBindsAGroup : countManys Player (nomIntro (Each {bs = []} AnyPlayer)) = 1
eachPlayerBindsAGroup = Refl

--------------------------------------------------------------------------------
-- The card's own cost letters, in the telescope its text elaborates against.
--------------------------------------------------------------------------------

||| Prosperity -- "{X}{U} Sorcery: Each player draws X cards." The printed
||| cost writes the variable symbol, so the face hands its text one letter
||| already bound.
public export
prosperityCostLetters :
  costLetters (Just [Variable, Macros.pip Blue]) = [letterB X]
prosperityCostLetters = Refl

||| ...and the text's "X" mints nothing of its own there: it READS the
||| cost's letter, which is the whole content of [CR#107.3i] at this seat.
public export
prosperityTextReadsCostLetter :
  amtDelta (LetterVal X {bs = costLetters (Just [Variable, Macros.pip Blue])}) = []
prosperityTextReadsCostLetter = Refl

||| ...where the same "X" against the empty telescope the face used to hand
||| it minted its OWN binding. Two letters where the card prints one; that
||| is the gap the index closes.
public export
textAloneOnceMintedItsOwnLetter :
  amtDelta (LetterVal X {bs = []}) = [letterB X]
textAloneOnceMintedItsOwnLetter = Refl

||| A cost that writes no variable symbol hands its text the empty
||| telescope, so every card that was writable before is writable
||| unchanged.
public export
noVariableSymbolNoLetter :
  costLetters (Just [Macros.generic 1, Macros.pip Blue]) = []
noVariableSymbolNoLetter = Refl

||| ...and so does a face with no printed cost at all [CR#202.3a].
public export
noCostNoLetter : costLetters Nothing = []
noCostNoLetter = Refl



--------------------------------------------------------------------------------
-- The ascription's last five subtype words, its self-antecedent pronoun,
-- and the linkage read written under a subtype word.
--------------------------------------------------------------------------------

||| Debris Beetle's enters trigger -- "When this Vehicle enters, each
||| opponent loses 3 life and you gain 3 life." The `Vehicle` word's
||| witness: [CR#205.3c] hangs the word on its own card type and
||| `ascriptionOk` reads it back, so naming it is writing it. The whole
||| card waits on crew.
public export
debrisBeetleTrigger : Ability
debrisBeetleTrigger =
  Macros.triggered When (Enters Macros.thisVehicle Nothing)
    (Sequentially [ Macros.losesLife (Each Opponent) (Lit 3)
                  , Macros.gainsLife You (Lit 3) ])

||| Nautiloid Ship's damage trigger -- "Whenever this Vehicle deals combat
||| damage to a player, you may put a creature card exiled with this
||| Vehicle onto the battlefield under your control." The linkage note
||| [CR#406.6] read at a SUBTYPE word: the note hangs on the object, and
||| which of its own type words named it is spelling. The whole card waits
||| on crew.
public export
nautiloidShipTrigger : Ability
nautiloidShipTrigger =
  Macros.triggered Whenever
    (DealsCombatDamage Macros.thisVehicle (Macros.a AnyPlayer))
    (Macros.may You
       (Macros.putOntoBattlefieldUnderYourControl
          (Macros.a (And [Macros.creature, ExiledWith Macros.thisVehicle]))))

||| Summon: Esper Valigarmanda's II/III/IV body, in part -- "You may cast
||| an instant or sorcery card exiled with this Saga, and mana of any
||| type can be spent to cast that spell." The Saga word was already
||| rowed; only the linkage cell was shut, and this is the first of the
||| six supported lines it was shutting. The clause beside it now writes
||| too, at the SPEND deed: [CR#118.14] makes "mana of any type can be
||| spent" the same permission as "you may spend mana as though it were",
||| so the passive is spelling and this is the same row Rogue Class
||| writes actively. What the card still waits on is the possessed-hand
||| destination Roads Go Ever, Ever On's chapter wants: `DestOk` admits
||| bare zones only, which is a move-destination gap and not a linkage
||| one.
public export
summonEsperValigarmandaCast : StaticEffect []
summonEsperValigarmandaCast =
  AndAlso [ MayPlay You (Macros.a (And [Macros.instantOrSorcery,
                                        ExiledWith Macros.thisSaga]))
                    Cast Nothing Nothing Nothing Nothing False ItsOwnCost
          , Macros.maySpendAsThough You Nothing MatchAnyType
              (Just (ToCast (And [Macros.instantOrSorcery,
                                  ExiledWith Macros.thisSaga]))) ]

||| Rogue Class's level-3 body -- "You may play cards exiled with this
||| Class, and you may spend mana as though it were mana of any color to
||| cast those spells." Both conjuncts now: the play permission and
||| [CR#609.4b]'s payment permission beside it. The level machinery is
||| what the whole card still waits on.
|||
||| The purpose is written as the description rather than as "those
||| spells": `SpendPurpose.ToCast` takes a `Predicate`, which describes
||| and points at nothing, so an anaphoric purpose has no spelling here.
||| It names the same set, and the anaphor is this round's recorded gap
||| on the slot -- most of the 83 sentences write one.
public export
rogueClassLevelThree : StaticEffect []
rogueClassLevelThree =
  AndAlso [ Macros.mayPlay You (AllOf (ExiledWith Macros.thisClass))
          , Macros.maySpendAsThough You Nothing MatchAnyColor
              (Just (ToCast (ExiledWith Macros.thisClass))) ]

||| Wurmwall Sweeper's enters trigger -- "When this Spacecraft enters,
||| surveil 2." The `Spacecraft` word's witness; the whole card waits on
||| station.
public export
wurmwallSweeperTrigger : Ability
wurmwallSweeperTrigger =
  Macros.triggered When (Enters Macros.thisSpacecraft Nothing) (Macros.surveil (Lit 2))

||| Case of the Crimson Pulse's enters trigger -- "When this Case enters,
||| discard a card, then draw two cards." The `Case` word's witness; the
||| whole card waits on the to-solve/solved clauses.
public export
caseOfTheCrimsonPulseTrigger : Ability
caseOfTheCrimsonPulseTrigger =
  Macros.triggered When (Enters Macros.thisCase Nothing)
    (Sequentially [Macros.discardsACard You, Macros.drawCards 2])

||| Glassworks' end-step trigger -- "At the beginning of your end step,
||| this Room deals 1 damage to each opponent." The `Room` word's witness;
||| the whole card waits on the door machinery.
public export
glassworksTrigger : Ability
glassworksTrigger =
  Macros.triggered At (BeginningOf EndStep (ByWord Yours))
    (DealDamage Macros.thisRoom (Lit 1) (Each Opponent))

||| Ferocious Pup, whole -- "When this creature enters, create a 2/2 green
||| Wolf creature token." The `Wolf` word, on the type line and on the
||| token, for the row Arlinn, Voice of the Pack asked for.
public export
ferociousPup : Card
ferociousPup =
  Macros.card "Ferocious Pup" (Just [Macros.generic 2, Macros.pip Green]) []
       (MkTypeLine [creatureType "Wolf"] [Creature])
       [ Macros.triggered When (Enters Macros.thisCreature Nothing)
           (Macros.create (Lit 1) (Macros.creatureTok 2 2 [Green] [creatureType "Wolf"])) ]
       (Just (0, 1))

||| Talrand's Invocation -- "Create two 2/2 blue Drake creature tokens
||| with flying." The `Drake` word, for the row Flailing Drake asked for.
public export
talrandsInvocation : Effect []
talrandsInvocation =
  Macros.create (Lit 2)
    (MkToken (Just (Lit 2 ** Lit 2)) [Blue] (MkTypeLine [creatureType "Drake"] [Creature])
             [Macros.keyword "Flying"] Nothing)

||| Predatory Wurm, whole -- "Vigilance / This creature gets +2/+2 as long
||| as you control a Garruk planeswalker." The `Garruk` word, read as a
||| DESCRIPTION rather than off a type line: [CR#109.2] reads a
||| description naming a card type or subtype onto the battlefield, so a
||| card may name the planeswalker set without being in it. Garruk
||| Relentless's own card is blocked on a STATE TRIGGER, not on the
||| transform verb: "When Garruk has two or fewer loyalty counters on
||| him, transform him" is [CR#603.8]'s shape, whose header is a
||| CONDITION rather than an event, and `Triggered` takes a `GameEvent`.
||| Its two loyalty abilities and its whole back face write today, and
||| the verb it waited on is landed. 1 supported face writes a
||| loyalty-counter state trigger and it is this one (measured
||| 2026-08-28), so the machinery waits on a carrier count this card
||| cannot supply alone.
public export
predatoryWurm : Card
predatoryWurm =
  Macros.card "Predatory Wurm" (Just [Macros.generic 3, Macros.pip Green]) []
       (MkTypeLine [creatureType "Wurm"] [Creature])
       [ Macros.keyword "Vigilance"
       , Static (Macros.asLongAs
                   (Exists (And [HasSubtype (planeswalkerType "Garruk"), ControlledBy You]))
                   (Gets Macros.thisCreature (PtUp (Lit 2)) (PtUp (Lit 2)))) ]
       (Just (4, 4))

||| Soul Ransom's ransom clause -- "This Aura's controller sacrifices it,
||| then draws two cards." The SELF-ANTECEDENT pronoun. A possessive
||| already announces whatever its base announces, so "target creature's
||| controller … it" has always written; a deictic base announced nothing
||| only because deixis has no `nounDelta`. The possessive now threads
||| `selfSubjDelta`, which mints at `SelfD` -- the determiner "it" reads
||| and the demonstrative words do not -- so the ascription is the
||| sacrifice slot's one candidate at the permanent carrier
||| [CR#109.2,701.21a] and the clause writes. The whole card is
||| `soulRansom` below, since the activation restriction it waited on
||| now has its seat.
public export
soulRansomRansom : Effect []
soulRansomRansom =
  Sequentially [ Macros.sacrificeIt (ControllerOf Macros.thisAura)
               , Draw They (Lit 2) ]

||| Soul Ransom, whole -- the ransom clause with the activation
||| restriction that was holding it. "Only your opponents may activate
||| this ability" is [CR#602.2]'s own exception written out: the rule
||| gives an activated ability to its object's controller alone "unless
||| the object specifically says otherwise", so the restriction is a slot
||| on the ability beside its window and its usage limit, and not a
||| second statement about it.
public export
soulRansom : Card
soulRansom =
  Macros.card "Soul Ransom"
       (Just [Macros.generic 2, Macros.pip Blue, Macros.pip Black]) []
       (MkTypeLine [enchantmentType "Aura"] [Enchantment])
       [ Macros.keywordSubject "Enchant" Macros.creature
       , Static (GainsControl You (AttachHost Enchanted (TypeW Creature)))
       , Macros.activatedBy (Do (Macros.discardN (Lit 2)))
           (Sequentially [ Macros.sacrificeIt (ControllerOf Macros.thisAura)
                         , Draw They (Lit 2) ])
           (PlayerGroup YourOpponents) ]
       Nothing

||| ...and the announcement it rests on, measured. The ascription under
||| the possessive is one candidate at the sacrifice slot's carrier.
public export
possessiveDeicticIsReadableByIt :
  countOnesAt PermanentSlot (nomIntro (ControllerOf (Macros.thisAura {bs = []}))) = 1
possessiveDeicticIsReadableByIt = Refl

||| ...while the demonstrative noun words still find nothing there:
||| `SelfD` is what "it" reads and what "that enchantment" does not, so
||| opening the possessive costs no demonstrative its resolution.
public export
possessiveDeicticIsNotADemonstrative :
  countWord (TypeW Enchantment) (nomIntro (ControllerOf (Macros.thisAura {bs = []}))) = 0
possessiveDeicticIsNotADemonstrative = Refl

||| ...and a DESCRIBED base is unchanged: it announced its referent
||| before this round and announces exactly one now.
public export
possessiveDescribedBaseUnchanged :
  countOnesAt PermanentSlot
    (nomIntro (ControllerOf (Macros.target Macros.creature {bs = []}))) = 1
possessiveDescribedBaseUnchanged = Refl

||| Contractual Safeguard's second paragraph -- "Choose a kind of counter on
||| a creature you control. Put a counter of that kind on each other creature
||| you control." The BOARD-READ counter-kind chooser and the first benched
||| carrier of `BoundKind`, which landed with a measured zero of them. The
||| chooser's description is what makes the second sentence's "other"
||| readable: it names the creature the kind came off, and "each OTHER
||| creature you control" is other than that one. The card's Addendum
||| paragraph is a cast-timing rider and is not taken here.
public export
contractualSafeguardPass : Effect []
contractualSafeguardPass =
  Sequentially
    [ Macros.choose (Macros.a (CounterKindOn (Macros.a Macros.creatureYouControl)))
    , PutCounters (Lit 1) BoundKind
        (Each (Macros.otherCreatureYouControl It)) ]

-- ---------------------------------------------------------------------------
-- The noun coordinations
-- ---------------------------------------------------------------------------

||| Bile Blight, whole card -- "Target creature and all other creatures with
||| the same name as that creature get -3/-3 until end of turn." The
||| COORDINATED SUBJECT: one statement over two mentions at one kind, which
||| `Both` could not write because a join of a kind with itself is not that
||| kind. The co-referential name half was already spelled; the pair is what
||| the cycle waited on. Echoing Decay writes the same sentence at -2/-2, and
||| Echoing Truth, Echoing Calm, Echoing Return, Declaration in Stone,
||| Deputy of Detention and Banishment write it under other verbs.
public export
bileBlight : Card
bileBlight =
  Macros.card "Bile Blight" (Just [Macros.pip Black, Macros.pip Black]) []
       (MkTypeLine [] [Instant])
       [ Spell (Macros.gets
                  (BothOf (Macros.target Macros.creature)
                          (AllOf (And [ Macros.creature
                                      , Named (SameNameAs (That (TypeW Creature)))
                                      , OtherThan (That (TypeW Creature)) ])))
                  (PtDown (Lit 3)) (PtDown (Lit 3))
                  (Just Macros.untilEndOfTurn)) ]
       Nothing

||| Echoing Ruin, whole card -- "Destroy target artifact and all other
||| artifacts with the same name as that artifact." The same coordination
||| under a second verb, which is what makes it the phrase's row and not the
||| statement's.
public export
echoingRuin : Card
echoingRuin =
  Macros.card "Echoing Ruin" (Just [Macros.generic 1, Macros.pip Red]) []
       (MkTypeLine [] [Sorcery])
       [ Spell (Macros.destroy
                  (BothOf (Macros.target Macros.artifact)
                          (AllOf (And [ Macros.artifact
                                      , Named (SameNameAs (That (TypeW Artifact)))
                                      , OtherThan (That (TypeW Artifact)) ])))) ]
       Nothing

||| Stomp and Howl, whole card -- "Destroy target artifact and target
||| enchantment." The HETEROGENEOUS DOUBLE TARGET, and the reason it is the
||| noun coordination rather than a union head or a coordinating
||| description: [CR#601.2c] lets a spell choose the same object once for
||| each instance of the word "target", and gives "Destroy target artifact
||| and target land" as its own example of a spell that may target one
||| artifact land twice. The right arm is read in the left arm's discourse,
||| so each writes the word once and the pair writes it twice.
public export
stompAndHowl : Card
stompAndHowl =
  Macros.card "Stomp and Howl" (Just [Macros.generic 2, Macros.pip Green]) []
       (MkTypeLine [] [Sorcery])
       [ Spell (Macros.destroy (BothOf (Macros.target Macros.artifact)
                                       (Macros.target Macros.enchantment))) ]
       Nothing

||| Churning Eddy, whole card -- "Return target creature and target land to
||| their owners' hands." The double target under a MOVE, where the two
||| mentions share one destination.
public export
churningEddy : Card
churningEddy =
  Macros.card "Churning Eddy" (Just [Macros.generic 4, Macros.pip Blue]) []
       (MkTypeLine [] [Instant])
       [ Spell (Macros.move (BothOf (Macros.target Macros.creature)
                                    (Macros.target Macros.land))
                            Macros.handZ) ]
       Nothing

||| Secret Rendezvous, whole card -- "You and target opponent each draw
||| three cards." The PLAYER-PLUS-PLAYER DISTRIBUTIVE: the trailing "each"
||| is the construction, not decoration, and every one of the family's
||| printed lines writes it (the unmarked joint pair is a measured zero).
||| Three cards apiece, never three between them.
public export
secretRendezvous : Card
secretRendezvous =
  Macros.card "Secret Rendezvous"
       (Just [Macros.generic 1, Macros.pip White, Macros.pip White]) []
       (MkTypeLine [] [Sorcery])
       [ Spell (Draw (EachOfBoth (BothOf You (Macros.target Opponent))) (Lit 3)) ]
       Nothing

||| Mana Clash's first sentence -- "You and target opponent each flip a
||| coin." The same distributive with a per-referent verb: the "each" is
||| what makes it two coins rather than one, since [CR#705.2] gives a flip
||| to the player who flipped it and each arm flips its own. The card's
||| repeat-until clause is not taken here.
public export
manaClashFlip : Effect []
manaClashFlip =
  FlipCoins (EachOfBoth (BothOf You (Macros.target Opponent))) (FlipCount (Lit 1))

||| Weftwalking's body -- "shuffle your hand and graveyard into your library,
||| then draw seven cards." The COORDINATED MASS OBJECT: two whole-zone
||| mentions under one move. The pair's own zone is neither arm's, so it
||| projects none [CR#109.2a] and the destination is what places it.
||| Midnight Clock and Trenzalore Clocktower write the same clause; the
||| dominant surface of the family ("each player shuffles THEIR hand and
||| graveyard") waits on a distributive possessive and not on this.
public export
weftwalkingShuffle : Effect []
weftwalkingShuffle =
  Sequentially
    [ Macros.shuffleInto (BothOf (AllOf (InZone (Macros.handOf You)))
                                 (AllOf (InZone (Macros.graveyardOf You))))
    , Macros.drawCards 7 ]

||| Sugar Coat, WHOLE CARD -- the kind-crossing disjunction, "Enchant
||| creature or Food". One card TYPE crossed with an artifact SUBTYPE at one
||| kind, which is a noun coordination and no union head: [CR#205.3c]
||| correlates a subtype to its own card type, so a disjunct writing its own
||| head presupposes that head and nothing shared. The rest of the card was
||| already writable -- the flash line is a keyword row, the quoted payload
||| rides `TokenChars`' abilities, and "loses all other card types and
||| abilities" is the retention slot plus `LosesAllAbilities`.
public export
sugarCoat : Card
sugarCoat =
  Macros.card "Sugar Coat" (Just [Macros.generic 2, Macros.pip White]) []
       (MkTypeLine [enchantmentType "Aura"] [Enchantment])
       [ Macros.keyword "Flash"
       , Macros.keywordSubject "Enchant"
           (Or [HasType Creature, HasSubtype (artifactType "Food")])
       , Static (AndAlso
           [ SetsType (AttachHost Enchanted PermanentW)
                      (MkToken Nothing []
                               (MkTypeLine [artifactType "Food"] [Artifact])
                               [ Macros.activated
                                   (Compound [ Mana [Macros.generic 2]
                                             , TapSymbol
                                             , Do (Macros.sacrifice You Macros.thisArtifact) ])
                                   (Macros.gainsLife You (Lit 3)) ]
                               Nothing)
                      Nothing
           , LosesAllAbilities It Nothing ]) ]
       Nothing

||| Doc Aurlock, Grizzled Genius's first line -- "Spells you cast from your
||| graveyard or from exile cost {2} less to cast." The CAST-ORIGIN
||| disjunction, which needed nothing: [CR#601.2a] moves the card OUT of the
||| zone it was in as it is cast, so an origin is history rather than a place
||| the object is, `CastFrom` seeds no zone, and the arms are parallel
||| already. The cross-zone refusal is `InZone`'s alone, where the two arms
||| would project two places for one phrase. The card's plot line is not
||| taken.
public export
docAurlockCost : StaticEffect []
docAurlockCost =
  CostsToCast (AllOf (And [Macros.spell, CastBy You,
                           Or [ CastFrom (Macros.graveyardOf You)
                              , CastFrom Macros.exileZ ]]))
              (CostLess (Lit 2) Nothing)

-- ---------------------------------------------------------------------------
-- "and/or": the zone coordination it earns, and the description it does not
-- ---------------------------------------------------------------------------

||| Agency Outfitter's search -- "search your graveyard, hand and/or library
||| for a card named Magnifying Glass and/or a card named Thinking Cap". The
||| word twice in one clause, at its two positions, and the round's verdict
||| in one line: over ZONES it is the coordination `SomeZones` writes, and
||| over the DESCRIPTION it is `Or`, which the grammar already had. The card
||| is not whole -- "If you search your library this way, shuffle" reads
||| back WHICH zone was searched, and no such reader exists.
public export
agencyOutfitterSearch : Effect []
agencyOutfitterSearch =
  Macros.searchZonesOf You
    (Or [ Named (PrintedName "Magnifying Glass")
        , Named (PrintedName "Thinking Cap") ])

||| Delivery Moogle's search -- "search your library and/or graveyard for an
||| artifact card with mana value 2 or less, reveal it, and put it into your
||| hand." The family's commonest arity; Ajani's Aid, Finale of Devastation
||| and the whole planeswalker-fetch cycle write the same two zones. Blocked
||| whole by the same which-zone reader.
public export
deliveryMoogleSearch : Effect []
deliveryMoogleSearch =
  Sequentially
    [ Macros.searchLibraryOrGraveyard
        (And [Macros.artifact, Compare [CharAxis ManaValue] AtMost (Lit 2)])
    , Macros.revealCards It
    , Macros.move It Macros.handZ ]

||| Concussive Bolt, both paragraphs -- "deals 4 damage to target player or
||| planeswalker. / Metalcraft — If you control three or more artifacts,
||| creatures controlled by that player or by that planeswalker's controller
||| can't block this turn." The repeated preposition is SPELLING and nothing
||| more: "by X or by Y" and "X or Y" denote the same pair, so the split read
||| the union round landed writes this line unchanged. The ability word is
||| not taken here.
public export
concussiveBolt : Effect []
concussiveBolt =
  Sequentially
    [ DealDamage This (Lit 4) Cards.targetPlayerOrPlaneswalker
    , If (CompareAmt (CountOf (And [Macros.artifact, ControlledBy You]))
                     AtLeast (Lit 3))
         (Continuously
            (Macros.deontic (AllOf (And [Macros.creature,
                                  ControlledBy Macros.splitOverPlaneswalker]))
                     Forbid ["Block"] Agent NoDeonticPatient)
            (Just Macros.thisTurn))
         Nothing ]

||| Trouble in Pairs' whole header -- "Whenever an opponent attacks you
||| with two or more creatures, draws their second card each turn, or casts
||| their second spell each turn, you draw a card." All three arms now: the
||| first wanted the defender the attacking player's header may name
||| [CR#508.1b], and the other two wanted the period their ordinals are
||| counted over. The arms SHARE a subject in print and write their own in
||| the semantics: English elides the repeated noun, and the elision is the
||| spelling, not a mechanism. Nothing is lost here because the body reads
||| no arm ("you draw a card"); a body naming "that player" would be reading
||| one of three existentials and is what the seat's own whole-agreement
||| rule already refuses.
||| The card's OTHER line -- "If an opponent would begin an extra turn,
||| that player skips that turn instead" -- is a replacement over a turn's
||| beginning, which no event row spells, so the header is the witness and
||| the card is not whole.
public export
troubleInPairsArms : AbilityAt []
troubleInPairsArms =
  Triggered Whenever
    (AttacksWith Macros.anOpponent (OneDefender You)
                 (CountedGroup (Macros.atLeast 2) Nothing Macros.creature))
    [ NthOccurrence (Nth 2) (Just Turn) (Draws (Macros.a Opponent))
    , NthOccurrence (Nth 2) (Just Turn)
        (Casts (Macros.a Opponent) (Macros.a Macros.spell) Nothing) ]
    Nothing [] Nothing Nothing Nothing
    (Draw You (Lit 1))

||| Gaea's Revenge's protection-shaped phrase -- "nongreen spells or
||| abilities from nongreen sources", the described side alone. The colour
||| is NOT one modifier distributed over two conjuncts: the spell arm tests
||| the SPELL's own colour and the ability arm tests its SOURCE's, because
||| an ability on the stack has no colour of its own and [CR#113.7] gives it
||| a source instead. The card's own ruling states the pair in exactly those
||| words. So the phrase is the landed cross-kind head with a different
||| description per arm, and it writes today.
||| What the eight printed sentences wait on is the TARGETING RESTRICTION
||| and nothing else: no act row anywhere says "can't be the target of"
||| ([CR#115.1] is where the relation lives, and the grammar's act
||| vocabularies cover countering, casting, playing, copying, activating,
||| regenerating, attacking and blocking).
public export
nongreenSpellsOrAbilities : Predicate [] (Object \/ Ability)
nongreenSpellsOrAbilities =
  Joined (And [Macros.spell, Not (ColorIs Green)])
         (And [ AbilityHead AnyOnStack
              , AbilityOf (Macros.a (And [Macros.source, Not (ColorIs Green)])) ])


-- ---------------------------------------------------------------------
-- Counted anaphora narrowings, round 1: the de-pronominalization
-- templates and the producer-label extensions.
-- ---------------------------------------------------------------------

||| Aim High, whole card -- "Untap target creature. It gets +2/+2 and gains
||| reach until end of turn." Two of the round's rows on one card: the
||| untap is a LABELED action, so the pronoun that follows it is read at
||| the label rather than across every singular object mention
||| [CR#701.26b], and the sentence it heads is a shared-subject
||| coordination whose second verb phrase writes no subject at all.
public export
aimHigh : Card
aimHigh =
  Macros.card "Aim High" (Just [Macros.generic 1, Macros.pip Green]) []
       (MkTypeLine [] [Instant])
       [ Spell (Sequentially
                  [ Macros.untap (Macros.target Macros.creature)
                  , Macros.sharedSubject (ItVerbed "Untap")
                      [ VPGets (PtUp (Lit 2)) (PtUp (Lit 2)) Nothing
                      , VPGains (Macros.keyword "Reach") Nothing ]
                      (Just Macros.untilEndOfTurn) ]) ]
       Nothing

||| Hijack, whole card -- "Gain control of target artifact or creature
||| until end of turn. Untap it. It gains haste until end of turn." The
||| control change stamps its own patient, so "untap it" is read at the
||| clause that gained control of it; the untap then re-stamps the same
||| mention, and the third sentence reads it at THAT label. 19 supported
||| faces write the pair (re-measured 2026-08-27).
public export
hijack : Card
hijack =
  Macros.card "Hijack" (Just [Macros.generic 1, Macros.pip Red, Macros.pip Red]) []
       (MkTypeLine [] [Sorcery])
       [ Spell (Sequentially
                  [ Macros.gainControl (Macros.target (Or [Macros.artifact, Macros.creature]))
                                       (Just Macros.untilEndOfTurn)
                  , Macros.untap (ItVerbed "GainControl")
                  , Macros.gainsHaste (ItVerbed "Untap") (Just Macros.untilEndOfTurn) ]) ]
       Nothing

||| Aggressive Instinct, whole card -- "Target creature you control deals
||| damage equal to its power to target creature you don't control." The
||| "its" is the DAMAGE SOURCE [CR#120.1], which is the clause's own
||| subject, so the row supplies it from its own earlier argument and no
||| pronoun is read. 275 supported faces write the family, all of them
||| source-bound.
public export
aggressiveInstinct : Card
aggressiveInstinct =
  Macros.card "Aggressive Instinct" (Just [Macros.generic 1, Macros.pip Green]) []
       (MkTypeLine [] [Sorcery])
       [ Spell (DealDamageOwn (Macros.target Macros.creatureYouControl) Power
                              (Macros.target Macros.creatureYouDontControl)) ]
       Nothing

||| Arcum Dagsson's sacrifice sentence -- "Target artifact creature's
||| controller sacrifices it." The possessive subject and the pronoun are
||| ONE referent by [CR#701.21a]'s own definition of the act, so the
||| template writes the permanent once and neither is a read. The
||| ability's second sentence ("That player may search their library for a
||| noncreature artifact card, put it onto the battlefield, then shuffle")
||| wants a library search whose searcher is a mention rather than "you",
||| which `searchLibraryFor` does not spell.
public export
arcumDagssonSacrifice : Effect []
arcumDagssonSacrifice =
  ControllerSacrifices (Macros.target (And [Macros.artifact, Macros.creature]))

||| Harried Dronesmith's token line -- "create a 1/1 colorless Thopter
||| artifact creature token with flying. It gains haste until end of
||| turn." The pronoun is read at the ORIGIN the create clause wrote onto
||| its own mention [CR#111.1], not across every singular object. The
||| card's third sentence schedules a sacrifice at a named future step.
public export
harriedDronesmithToken : Effect []
harriedDronesmithToken =
  Sequentially [ Macros.create (Lit 1)
                   (MkToken (Just (Lit 1 ** Lit 1)) []
                            (MkTypeLine [creatureType "Thopter"] [Artifact, Creature])
                            [Macros.keyword "Flying"] Nothing)
               , Macros.gainsHaste Macros.itAsToken (Just Macros.untilEndOfTurn) ]

||| Feral Contest, whole card -- "Put a +1/+1 counter on target creature
||| you control. Another target creature blocks IT this turn if able."
||| The pronoun is read over the prefix the BLOCKER did not announce:
||| [CR#509.1a] and [CR#508.1a] put the blocker and the creature it is
||| made to block under different players' control, so the second
||| sentence's own subject is not among the candidates for its object.
||| The bare `It` here counts two battlefield creatures and refuses --
||| correctly, since one of the two is the blocker itself. 6 supported
||| faces write the forced block with a pronoun (re-measured 2026-08-27).
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

||| Thranduil's Company, whole card -- "Whenever a land you control
||| enters, put two +1/+1 counters on target creature you control. It
||| gains vigilance until end of turn."
||| Both candidates are on the battlefield, so neither the bare `It` nor
||| the carrier narrowing resolves this, and the counter clause leaves no
||| label and no origin for the producer narrowings to read. What names
||| the referent is the coordination itself: the third clause reads the
||| mentions its immediate neighbour made [CR#608.2c], and that segment
||| holds exactly one singular object.
public export
thranduilsCompany : Card
thranduilsCompany =
  Macros.card "Thranduil's Company"
       (Just [Macros.generic 2, Macros.pip Green, Macros.pip Blue]) []
       (MkTypeLine [creatureType "Elf", creatureType "Soldier"] [Creature])
       [ Macros.triggered Whenever
           (Enters (Macros.a (And [Macros.land, ControlledBy You])) Nothing)
           (Sequentially
              [ PutCounters (Lit 2) (PrintedKind Macros.plusOnePlusOne)
                            (Macros.target Macros.creatureYouControl)
              , Macros.gains
                  (Macros.itPrior
                     (PutCounters (Lit 2) (PrintedKind Macros.plusOnePlusOne)
                                  (Macros.target Macros.creatureYouControl)))
                  (Macros.keyword "Vigilance") (Just Macros.untilEndOfTurn) ]) ]
       (Just (3, 4))

||| Inquisitor's Flail, whole card -- "If equipped creature would deal
||| combat damage, it deals double that damage instead. / If another
||| creature would deal combat damage to equipped creature, it deals
||| double that damage to equipped creature instead. / Equip {2}"
||| The damage-replacement family's "it" is CONSTRUCTOR SPELLING and not
||| a read at all: `Scales` writes the source once and the replacement
||| restates it, exactly as [CR#614.6] restates the replaced event
||| ("A modified event occurs instead"). Both of this card's rows are the
||| family at its two shapes -- an unattributed recipient and a named one
||| -- and neither writes a pronoun the grammar has to resolve.
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

||| Stunning Shot, whole card -- "Put two +1/+1 counters on up to one
||| target creature you control. Tap up to one target creature an
||| opponent controls and put a stun counter on IT."
||| The ticket's named witness for the previous-sibling read, and the
||| within-sentence shape of it: the two conjuncts of one "and" [CR#608.2c],
||| where the first sentence has already announced a second battlefield
||| creature and the bare `It` therefore counts two. 38 supported faces
||| write the tap-then-stun pair (re-measured 2026-08-27). The label
||| narrowing `ItVerbed "Tap"` admits this line too -- what the segment
||| read adds is the LABEL-FREE case, which Thranduil's Company benches.
public export
stunningShot : Card
stunningShot =
  Macros.card "Stunning Shot" (Just [Macros.generic 1, Macros.pip White]) []
       (MkTypeLine [] [Sorcery])
       [ Spell (Sequentially
                  [ PutCounters (Lit 2) (PrintedKind Macros.plusOnePlusOne)
                                (TargetGroup (Macros.upTo 1) Macros.creatureYouControl)
                  , Macros.tap (TargetGroup (Macros.upTo 1)
                                  (And [Macros.creature, ControlledBy Macros.anOpponent]))
                  , PutCounters (Lit 1) (PrintedKind Stun)
                                (Macros.itPrior
                                   (Macros.tap (TargetGroup (Macros.upTo 1)
                                      (And [Macros.creature,
                                            ControlledBy Macros.anOpponent])))) ]) ]
       Nothing

||| Stonebinder's Familiar, whole -- "Whenever one or more cards are put
||| into exile during your turn, put a +1/+1 counter on this creature.
||| This ability triggers only once each turn." The subject is the bare
||| card head: the line writes the word with no zone beside it, and the
||| exile it names is the move's DESTINATION, not the phrase's zone.
public export
stonebindersFamiliar : Card
stonebindersFamiliar =
  Macros.card "Stonebinder's Familiar"
       (Just [Macros.pip White]) []
       (MkTypeLine [creatureType "Spirit", creatureType "Dog"] [Creature])
       [ Triggered Whenever
                   (PutInto (CountedGroup (Macros.atLeast 1) Nothing IsCard)
                            Macros.exileZ Nothing)
                   []
                   Nothing
                   []
                   (Just (DuringWindow Turn (Just Yours)))
                   (Just OncePerTurn)
                   Nothing
                   (PutCounters (Lit 1) (PrintedKind Macros.plusOnePlusOne)
                                Macros.thisCreature) ]
       (Just (1, 1))

||| Bioplasm, whole -- "Whenever this creature attacks, exile the top card
||| of your library. If it's a creature card, this creature gets +X/+Y
||| until end of turn, where X is the exiled creature card's power and Y
||| is its toughness." The typed CARD word is what the last clause wanted:
||| the type comes from the test's re-mark and the zone from the exile the
||| clause named, which is [CR#109.2a]'s reading and never [CR#109.2]'s.
public export
bioplasm : Card
bioplasm =
  Macros.card "Bioplasm"
       (Just [Macros.generic 3, Macros.pip Green, Macros.pip Green]) []
       (MkTypeLine [creatureType "Ooze"] [Creature])
       [ Macros.triggered Whenever
           (Macros.attacks Macros.thisCreature)
           (Sequentially
              [ Macros.exile Macros.topCard
              , If Cards.bioplasmCardTest
                   (Macros.gets Macros.thisCreature
                        (PtUp (Macros.powerOf
                                 (Macros.theVerbed "Exile" (TypedCardW Creature))))
                        (PtUp (Macros.toughnessOf (Macros.itVerbed "Exile")))
                        (Just Macros.thisTurn))
                   Nothing ]) ]
       (Just (4, 4))

||| Oath of Kaya, whole -- "When Oath of Kaya enters, it deals 3 damage to
||| any target and you gain 3 life. / Whenever an opponent attacks a
||| planeswalker you control with one or more creatures, Oath of Kaya
||| deals 2 damage to that player and you gain 2 life." The corpus's one
||| PLANESWALKER defender, and it is the player-side header that writes
||| it: [CR#508.1b] has the attacking player announce which player,
||| planeswalker or battle each creature attacks, so the defender is
||| nameable from that side and the deed table already admits the type.
||| The body reads the attacking player back as "that player".
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
                                                     ControlledBy You])))
                        (CountedGroup (Macros.atLeast 1) Nothing Macros.creature))
           (Sequentially [ DealDamage This (Lit 2) (That PlayerW)
                         , Macros.gainsLife You (Lit 2) ]) ]
       Nothing

||| Stifle, whole -- "Counter target activated or triggered ability."
||| The TRIGGERED half of the pair on the stack, named beside the
||| activated one: [CR#113.3c] puts it there, [CR#113.9] lets an
||| ability-countering effect counter it, and [CR#115.1d] gives it the
||| word "target". Its reminder text ("Mana abilities can't be targeted")
||| is [CR#605.3b]'s rule restated and is not part of the ability.
public export
stifle : Card
stifle =
  Macros.card "Stifle" (Just [Macros.pip Blue]) []
       (MkTypeLine [] [Instant])
       [ Spell (Macros.counterSpell
                  (Macros.target (Or [AbilityHead AnyActivated,
                                      AbilityHead AnyTriggered]))) ]
       Nothing

||| Disallow, whole -- "Counter target spell, activated ability, or
||| triggered ability." The same pair with the spell arm beside it, which
||| is the cross-kind head over an ability disjunction.
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

||| Nazgûl's third line -- "Whenever the Ring tempts you, put a +1/+1
||| counter on each Wraith you control." The Ring's temptation as a thing
||| that happens: [CR#701.54] makes it a keyword action and [CR#701.54d]
||| states the trigger on it in the rules' own words, so it needs no event
||| row of its own -- the labelled act reading is what the vocabulary
||| already has for a watched keyword action.
||| The card is not whole: its second line INSTRUCTS the temptation, whose
||| body is [CR#701.54a]'s Ring-bearer choice and [CR#701.54c]'s emblem,
||| and its last line is a deck-construction rule with no seat here.
public export
nazgulRingTrigger : Ability
nazgulRingTrigger =
  Macros.triggered Whenever
    (VerbedEvent (Just You) "The Ring Tempts You" Nothing False)
    (PutCounters (Lit 1) (PrintedKind Macros.plusOnePlusOne)
                 (Each (And [HasSubtype (creatureType "Wraith"),
                             ControlledBy You])))

||| Captain Marvel, Apex Avenger, whole -- "Flying, double strike,
||| indestructible / Whenever you put one or more counters on another
||| creature, if it's not a Kree, you may put the same number and kind of
||| counters on Captain Marvel." The negated-SUBTYPE intervening "if" was
||| routed here as having no writable form; it writes, and always did.
||| [CR#603.4] gives the intervening slot a condition and puts no shape on
||| it, and the description side negates a subtype like any other
||| predicate, so the clause is `Matches` over `Not (HasSubtype …)` and
||| wants nothing new. The body's "same number and kind" is the landed
||| kind-blind distributive over the batch the header announced.
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
           (Macros.may You (PutCountersOfThoseKinds ThatMuch This)) ]
       (Just (4, 4))

||| Verity Circle, whole -- "Whenever a creature an opponent controls
||| becomes tapped, if it isn't being declared as an attacker, you may
||| draw a card. / {4}{U}: Tap target creature without flying." The
||| attack declaration IN PROGRESS as an intervening condition: the tap
||| [CR#508.1f] performs during the declaration is the one the card
||| excludes, and [CR#508.1k] is why "isn't attacking" would not say it.
public export
verityCircle : Card
verityCircle =
  Macros.card "Verity Circle"
       (Just [Macros.generic 2, Macros.pip Blue]) []
       (MkTypeLine [] [Enchantment])
       [ Macros.triggeredIf Whenever
           (StatusEvent (Macros.a (And [Macros.creature,
                                        ControlledBy Macros.anOpponent])) Tapped)
           (Matches It (Not BeingDeclaredAttacker))
           (Macros.may You Macros.drawACard)
       , Macros.activated (Mana [Macros.generic 4, Macros.pip Blue])
           (Macros.tap (Macros.target
                          (And [Macros.creature, Not (HasKeyword (TheKeyword "Flying"))]))) ]
       Nothing

||| Archfiend's Vessel, whole -- "Lifelink / When this creature enters, if
||| it entered from your graveyard or you cast it from your graveyard,
||| exile it. If you do, create a 5/5 black Demon creature token with
||| flying." The WINDOWLESS lookback: "it entered from your graveyard"
||| scopes to the entry this ability triggered on and writes no window
||| word, which is `Triggering` [CR#603.2,603.2c]. Its other disjunct
||| needs no window at all -- "you cast it from your graveyard" is the
||| object's own casting history, which the description side already
||| reads [CR#601.2a].
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
           (Reflexively (Macros.exile It)
              (Macros.create (Lit 1)
                 (MkToken (Just (Lit 5 ** Lit 5)) [Black]
                          (MkTypeLine [creatureType "Demon"] [Creature])
                          [Macros.keyword "Flying"] Nothing))) ]
       (Just (1, 1))

||| Hostile Investigator's header -- "Whenever one or more players discard
||| one or more cards, …". Two counted groups in one event, the first a
||| PLAYER: the act row's subject and patient are kind-general and each
||| takes its own determiner, so the composition needs nothing. The
||| patient is the bare card head, which the line writes with no zone
||| beside it; [CR#701.9a] supplies the hand the act finds it in.
||| The card's block is its BODY, `investigate` -- a keyword action that
||| creates a named token, whose label is unwritten here.
public export
hostileInvestigatorHeader : GameEvent []
hostileInvestigatorHeader =
  VerbedEvent (Just (CountedGroup (Macros.atLeast 1) Nothing AnyPlayer))
              "Discard"
              (Just (CountedGroup (Macros.atLeast 1) Nothing IsCard)) False

||| Hallowed Moonlight, whole -- "Until end of turn, if a creature would
||| enter and it wasn't cast, exile it instead. / Draw a card." The
||| agentless cast history: the conjunct denies that ANY casting
||| happened, which `CastBy` cannot say (it denies one named player's)
||| and `CastFrom` cannot say (it denies one origin's), so `WasCast`
||| under `Not` is the whole of it [CR#601.2,111.1].
public export
hallowedMoonlight : Card
hallowedMoonlight =
  Macros.card "Hallowed Moonlight"
       (Just [Macros.generic 1, Macros.pip White]) []
       (MkTypeLine [] [Instant])
       [ Spell (Sequentially
                  [ Macros.ifWouldInstead
                      (Enters (Macros.a (And [Macros.creature, Not WasCast])) Nothing)
                      (Macros.exile It)
                      (Just Macros.untilEndOfTurn)
                  , Macros.drawACard ]) ]
       Nothing

||| Containment Priest, whole -- "Flash / If a nontoken creature would
||| enter and it wasn't cast, exile it instead." The same conjunct with
||| no duration: a permanent's own standing replacement.
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
                   [] Nothing (Macros.exile It) Repeatedly Nothing) ]
       (Just (2, 2))


testMox1 : Card
testMox1 =
  Macros.card "Mox Diamond" Nothing [] (MkTypeLine [] [Artifact])
       [ Macros.activated TapSymbol
                          (AddMana You (Lit 1) (AnyColor SameColor) []) ]
       Nothing

||| Heart of Yavimaya, whole -- "If this land would enter, sacrifice a
||| Forest instead. If you do, put this land onto the battlefield. If you
||| don't, put it into its owner's graveyard. / {T}: Add {G}. / {T}:
||| Target creature gets +1/+1 until end of turn."
||| The replacement's body is ONE instruction with two conditional
||| continuations over the same antecedent, and that pair is `May`'s own
||| two arms: [CR#608.2c] reads the sentences in the order written, and
||| the second arm is reachable without an offer because [CR#609.3] does
||| only as much as possible -- a player who controls no Forest leaves
||| the instructed sacrifice undone, and the third sentence says what
||| happens then. So the offer slot stays empty here and Mox Diamond
||| writes it; nothing else separates the cycle's members.
||| The third sentence's printed pronoun is written as the self: the
||| replacement's body is PROSPECTIVE, so the entry it intercepts has
||| introduced no mention to read, and the sentence's "it" is the same
||| permanent its neighbour calls "this land".
public export
heartOfYavimaya : Card
heartOfYavimaya =
  Macros.card "Heart of Yavimaya" Nothing [] (MkTypeLine [] [Land])
       [ Static (Intercepts (Enters This Nothing) [] Nothing
                   (May Nothing
                        (Macros.sacrifice You
                           (Macros.a (And [Macros.land,
                                           HasSubtype (landType "Forest")])))
                        (Just (Macros.putOntoBattlefield This))
                        (Just (Macros.move This Macros.graveyardZ)))
                   Repeatedly Nothing)
       , Macros.activated TapSymbol
                          (AddMana You (Lit 1) (Runs [[OfColor Green]]) [])
       , Macros.activated TapSymbol
                          (Macros.gets (Macros.target Macros.creature)
                             (PtUp (Lit 1)) (PtUp (Lit 1))
                             (Just Macros.untilEndOfTurn)) ]
       Nothing

||| Mox Diamond, whole -- "If this artifact would enter, you may discard a
||| land card instead. If you do, put this artifact onto the battlefield.
||| If you don't, put it into its owner's graveyard. / {T}: Add one mana
||| of any color." The same pair with the offer WRITTEN, which is what
||| `May`'s first slot spells. The discarded card's hand is unwritten on
||| the card and supplied by [CR#701.9a], as it is wherever this
||| vocabulary writes a discard.
public export
moxDiamond : Card
moxDiamond =
  Macros.card "Mox Diamond" Nothing [] (MkTypeLine [] [Artifact])
       [ Static (Intercepts (Enters This Nothing) [] Nothing
                   (May (Just You)
                        (Macros.discard
                           (Macros.a (And [Macros.land, InZone Macros.handZ])))
                        (Just (Macros.putOntoBattlefield This))
                        (Just (Macros.move This Macros.graveyardZ)))
                   Repeatedly Nothing)
       , Macros.activated TapSymbol
                          (AddMana You (Lit 1) (AnyColor SameColor) []) ]
       Nothing

||| Gather Specimens, whole -- "If a creature would enter the battlefield
||| under an opponent's control this turn, it enters under your control
||| instead." The entry as a replacement BODY, which is a fixed-body row
||| and not an instruction: `EntersUnderInstead` changes one parameter of
||| the entry [CR#614.1d,614.12] where an `Effect` in `Intercepts`' body
||| would instruct a second act. The antecedent's "under an opponent's
||| control" rides the subject, where [CR#614.12] checks it.
public export
gatherSpecimens : Card
gatherSpecimens =
  Macros.card "Gather Specimens"
       (Just [Macros.generic 3, Macros.pip Blue, Macros.pip Blue, Macros.pip Blue])
       [] (MkTypeLine [] [Instant])
       [ Spell (Continuously
                  (EntersUnderInstead
                     (Macros.a (And [Macros.creature,
                                     ControlledBy Macros.anOpponent]))
                     You)
                  (Just Macros.thisTurn)) ]
       Nothing

||| Don't Blink's replacement, without its written agent -- "Until end of
||| turn, if one or more creatures would enter from exile or after being
||| cast from exile, their owners shuffle them into their libraries
||| instead."
||| Both halves the entry originally recorded as missing are seated: the
||| entry event carries the zone it arrived FROM, and the disjunction
||| over that origin is `Intercepts`' own arm list -- the second arm
||| reads the origin off the casting instead ([CR#601.2a] moves the card
||| to the stack, so a permanent spell cast from exile enters from there).
||| What the whole line still wants is the PLURAL possessor: "their
||| owners" distributes over the counted group, and `OwnerOf` is gated to
||| a singular subject, so the agent is dropped here and the act is
||| written agentless ([CR#701.24a] shuffles the library either way).
public export
dontBlinkReplacement : StaticEffect []
dontBlinkReplacement =
  Intercepts (Enters (CountedGroup (Macros.atLeast 1) Nothing Macros.creature)
                     (Just (FromZone [Macros.exileZ])))
             [ Enters (CountedGroup (Macros.atLeast 1) Nothing
                         (And [Macros.creature, CastFrom Macros.exileZ]))
                      Nothing ] Nothing
             (Macros.shuffleInto Them)
             Repeatedly Nothing

||| Seasoned Warrenguard, whole -- "Whenever this creature attacks while
||| you control a token, this creature gets +2/+0 until end of turn."
||| The header's concurrent clause in its commonest shape: a state named
||| INSIDE the trigger condition [CR#603.1,603.2], with no comma marking
||| it off, so it is what triggered rather than [CR#603.4]'s intervening
||| "if" -- which is checked a second time on resolution and this is not.
public export
seasonedWarrenguard : Card
seasonedWarrenguard =
  Macros.card "Seasoned Warrenguard" (Just [Macros.pip White]) []
       (MkTypeLine [creatureType "Rabbit", creatureType "Warrior"] [Creature])
       [ Macros.triggeredWhile Whenever
           (Macros.attacks Macros.thisCreature)
           (Macros.whileState (Exists (And [IsToken, ControlledBy You])))
           (Macros.gets Macros.thisCreature (PtUp (Lit 2)) (PtUp (Lit 0))
                        (Just Macros.untilEndOfTurn)) ]
       (Just (1, 2))

||| Brazen Blademaster, whole -- "Whenever this creature attacks while you
||| control two or more artifacts, it gets +2/+1 until end of turn." The
||| same slot over a counted condition, and the tail's "it" reads the
||| header's own subject: the concurrent clause announces nothing, so what
||| the effect sees is what the event left.
public export
brazenBlademaster : Card
brazenBlademaster =
  Macros.card "Brazen Blademaster"
       (Just [Macros.generic 2, Macros.pip Red]) []
       (MkTypeLine [creatureType "Orc", creatureType "Pirate"] [Creature])
       [ Macros.triggeredWhile Whenever
           (Macros.attacks Macros.thisCreature)
           (Macros.whileState
              (CompareAmt (CountOf (And [Macros.artifact, ControlledBy You]))
                          AtLeast (Lit 2)))
           (Macros.gets It (PtUp (Lit 2)) (PtUp (Lit 1))
                        (Just Macros.untilEndOfTurn)) ]
       (Just (2, 3))

||| Up the Beanstalk, whole -- "When this enchantment enters and whenever
||| you cast a spell with mana value 5 or greater, draw a card." The
||| two-header join: two trigger WORDS over one effect, told apart from
||| the coordination by that second word.
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
           Macros.drawACard ]
       Nothing

||| Autarch Mammoth's printed line -- "When this creature enters and
||| whenever it attacks while saddled, create a 3/3 green Elephant
||| creature token." The join and the concurrent clause in one header:
||| the "while saddled" qualifies the ATTACK alone, which is why the
||| joined half is a whole header and not one more `AltEvent` arm.
||| The card is whole at `autarchMammoth` now that [CR#702.171a]'s
||| "Saddle N" has a row; this name stays because the header is what the
||| trigger round bought.
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

||| Veiling Oddity's second line -- "When the last time counter is removed
||| from this card while it's exiled, creatures can't be blocked this
||| turn." The last-removal event's first reachable bench line, and what
||| reached it is the concurrent clause: the zone is named INSIDE the
||| trigger condition, unmarked by a comma, so it is part of what
||| triggered [CR#603.1,603.2] rather than [CR#603.4]'s intervening "if".
||| The card is still not whole, and its blocker moved rather than
||| cleared: "Suspend 4—{1}{U}" has no `keywordFacts` row because
||| [CR#702.62a]'s "Suspend N—[cost]" writes a COMPOUND parameter -- a
||| count of time counters beside a cost, and a counter count is no
||| component of one -- where craft's two printed slots are two
||| components of the one activation cost [CR#702.167a] writes out.
||| `KeywordParamShape` has no compound arm, and minting one is the
||| restricted equip line's decision [CR#702.6c], not this row's.
public export
veilingOddityLine : Ability
veilingOddityLine =
  Macros.triggeredWhile When
    (LastCounterRemoved Time This Nothing)
    (Macros.whileState (Matches This (InZone Macros.exileZ)))
    (Continuously (Macros.deontic (AllOf Macros.creature) Forbid ["Block"] Patient
                           NoDeonticPatient)
                  (Just Macros.thisTurn))

||| The concurrent clause's ACT arm, at the one printed shape the event
||| vocabulary already names: "while scrying" (The Temporal Anchor).
||| [CR#701.22a] looks at the top N cards and THEN puts them, so the act
||| has a moment inside it, which is what `eventUnderwayOk` reads off
||| `actStepwise`. The other two printed act shapes have landed
||| with the keyword rows they wanted: "while casting a spell with
||| emerge" (Foul Emissary, `foulEmissaryLine`) reads an "Emerge" row
||| through `HasKeyword`, and "while you're activating a craft ability"
||| (Market Gnome, whole) reads a "Craft" row through `AbilityClass`'
||| `KeywordClass`, which already spelled an ability described by a
||| keyword.
||| The Temporal Anchor is not on the bench for its own reason: its
||| trigger event is "you choose to put one or more cards on the bottom
||| of your library", a step inside the scry that no event row names.
public export
whileScrying : Concurrent []
whileScrying = WhileDoing (VerbedEvent (Just You) "Scry" Nothing False)

||| Akki Lavarunner // Tok-Tok, Volcano Born, a flip card [CR#710.1],
||| whole -- "Haste / Whenever this creature deals damage to an opponent,
||| flip it." // "Protection from red / If a red source would deal damage
||| to a player, it deals that much damage plus 1 to that player instead."
||| The flip verb's SOURCE-SIDE event: the header watches damage this
||| creature deals outside combat as well as in it, which `IsDealtDamage`
||| reads from the wrong end and `DealsCombatDamage` narrows to
||| [CR#510.1]'s assignment.
public export
akkiLavarunner : Card
akkiLavarunner =
  FlipCard
    (MkFace "Akki Lavarunner" (Just [Macros.generic 3, Macros.pip Red]) []
            (MkTypeLine [creatureType "Goblin", creatureType "Warrior"] [Creature])
            [ Macros.keyword "Haste"
            , Macros.triggered Whenever
                (DealsDamage Macros.thisCreature (OnePatient Macros.anOpponent))
                (SetStatus Flipped Macros.thisCreature) ]
            (Macros.printedBox (Just (1, 1))))
    (MkAltFace "Tok-Tok, Volcano Born" [Legendary]
               (MkTypeLine [creatureType "Goblin", creatureType "Shaman"] [Creature])
               [ Macros.keywordQuality "Protection" (ColorIs Red)
               , Static (Scales AnyDamage
                           (Macros.a (And [Macros.source, ColorIs Red]))
                           (Macros.shieldingIt (Macros.a AnyPlayer))
                           (Shifted ShiftUp (Lit 1)) Repeatedly) ]
               (Macros.printedBox (Just (2, 2))))

||| Bushi Tenderfoot // Kenzo the Hardhearted, a flip card [CR#710.1],
||| whole -- "When a creature dealt damage by this creature this turn
||| dies, flip this creature." // "Double strike; bushido 2."
||| The flip verb's BY-SOURCE lookback: the header's subject is described
||| by what happened TO it and by whom, which is the participial lookback
||| with its agent in the complement -- the umbrella recorded this as a
||| blocker and it writes as it stands.
public export
bushiTenderfoot : Card
bushiTenderfoot =
  FlipCard
    (MkFace "Bushi Tenderfoot" (Just [Macros.pip White]) []
            (MkTypeLine [creatureType "Human", creatureType "Soldier"] [Creature])
            [ Macros.triggered When
                (Dies (Macros.a (And [ Macros.creature
                                     , HappenedTo DamageTaken ThisTurn
                                         (Just (Involving Macros.thisCreature)) ])))
                (SetStatus Flipped Macros.thisCreature) ]
            (Macros.printedBox (Just (1, 1))))
    (MkAltFace "Kenzo the Hardhearted" [Legendary]
               (MkTypeLine [creatureType "Human", creatureType "Samurai"] [Creature])
               [ Macros.keyword "DoubleStrike"
               , Macros.keywordNumber "Bushido" (Lit 2) ]
               (Macros.printedBox (Just (3, 4))))

||| Frostwielder, whole -- "If a creature dealt damage by this creature
||| this turn would die, exile it instead. / {T}: This creature deals 1
||| damage to any target." The same participial subject on the
||| INTERCEPTION side, which is why it rides here: one phrase, both
||| frames.
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
                   [] Nothing (Macros.exile It) Repeatedly Nothing)
       , Macros.activated TapSymbol
           (DealDamage Macros.thisCreature (Lit 1) (Macros.target Macros.anyTarget)) ]
       (Just (1, 2))

||| Kitsune Mystic's flip trigger -- "At the beginning of the end step, if
||| this creature is enchanted by two or more Auras, flip it." The COUNTED
||| attachment: `IsAttached` asks only whether the permanent is attached,
||| and this line asks by how many, which `AttachedBy` puts to the
||| attachers' own determiner.
||| The card is not whole: its alternative half is "{1}: Attach target
||| Aura attached to a creature to another creature", whose target is
||| described by what it is attached TO -- the reverse of every
||| attachment phrase this vocabulary writes, which reads from the
||| attachment's own side (`AttachHost`, `IsAttached`, `AttachedBy`).
public export
kitsuneMysticFlip : Ability
kitsuneMysticFlip =
  Macros.triggeredIf At
    (BeginningOf EndStep NoPossessor)
    (Matches Macros.thisCreature
       (AttachedBy Enchanted
          (CountedGroup (Macros.atLeast 2) Nothing
             (HasSubtype (enchantmentType "Aura")))))
    (SetStatus Flipped Macros.thisCreature)


||| Blood Spatter Analysis, whole -- "When this enchantment enters, it
||| deals 3 damage to target creature an opponent controls. / Whenever one
||| or more creatures die, mill a card and put a bloodstain counter on
||| this enchantment. Then sacrifice it if it has five or more bloodstain
||| counters on it. When you do, return target creature card from your
||| graveyard to your hand."
||| Two things: the bloodstain counter is a FLAT kind -- [CR#122.1]'s
||| ordinary marker, whose whole meaning is the ability that counts it,
||| exactly as Font of Agonies' blood counter is -- and the reflexive
||| trigger hangs off a conditioned sacrifice, which is still one action
||| for [CR#603.12]'s pro-verb to abbreviate.
public export
bloodSpatterAnalysis : Card
bloodSpatterAnalysis =
  Macros.card "Blood Spatter Analysis"
       (Just [Macros.pip Black, Macros.pip Red]) []
       (MkTypeLine [] [Enchantment])
       [ Macros.triggered When (Enters Macros.thisEnchantment Nothing)
           (DealDamage Macros.thisEnchantment (Lit 3)
              (Macros.target (And [Macros.creature,
                                   ControlledBy Macros.anOpponent])))
       , Macros.triggered Whenever
           (Dies (CountedGroup (Macros.atLeast 1) Nothing Macros.creature))
           (Sequentially
              [ Macros.mills You (Lit 1) You
              , PutCounters (Lit 1) (PrintedKind Bloodstain) Macros.thisEnchantment
              , Reflexively
                  (OnlyIf (Macros.sacrifice You Macros.thisEnchantment)
                          (CompareAmt (CountersOn Bloodstain Macros.thisEnchantment)
                                      AtLeast (Lit 5))
                          Nothing)
                  (Macros.move
                     (Macros.target (And [Macros.creature, IsCard,
                                          InZone (Macros.graveyardOf You)]))
                     Macros.handZ) ]) ]
       Nothing

||| Bewitching Leechcraft, whole -- "Enchant creature / When this Aura
||| enters, tap enchanted creature. / Enchanted creature has 'If this
||| creature would untap during your untap step, remove a +1/+1 counter
||| from it instead. If you do, untap it.'"
||| The granted QUOTED replacement, and the two things it wanted: the
||| status change is admitted as a replacement's antecedent -- [CR#614.1]
||| replaces an event that would happen and [CR#701.26b] makes untapping
||| one, so nothing about the cell was trigger-only -- and "during your
||| untap step" sits in `Intercepts`' own window slot rather than in a
||| second copy of the header's window words. The reminder "(Otherwise, it
||| doesn't untap.)" restates [CR#614.1a] and is not carried.
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
                              (Just (DuringWindow UntapStep (Just Yours)))
                              (Macros.doThen
                                 (RemoveCounters (Macros.exactly 1)
                                    (Just Macros.plusOnePlusOne)
                                    Macros.thisCreature)
                                 (Macros.untap Macros.thisCreature))
                              Repeatedly Nothing))) ]
       Nothing

||| Bonus Round, whole -- "Until end of turn, whenever a player casts an
||| instant or sorcery spell, that player copies it and may choose new
||| targets for the copy."
||| The STANDING triggered ability: [CR#603.7b] fires a delayed trigger
||| once "unless it has a stated duration", so the duration slot the
||| delayed clause already carries is what makes the ability stand and
||| fire repeatedly while it lasts. No second ability shape is needed;
||| the re-measurement is 29 supported lines and this is the shape all of
||| them write.
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

||| Sheltered Valley, whole -- "If this land would enter, instead
||| sacrifice each other permanent named Sheltered Valley you control,
||| then put this land onto the battlefield. / At the beginning of your
||| upkeep, if you control three or fewer lands, you gain 1 life. / {T}:
||| Add {C}." The cycle's exception, with no conditional pair at all.
||| Both blockers the umbrella recorded are gone: the sacrifice is
||| `Does`' labelled act [CR#701.21a], which this vocabulary has always
||| written, and the name match is `Named` over a printed name, which
||| [CR#201.2a] is what makes true. What the printed "each other" needed
||| was an ANCHOR: the
||| replacement's body is prospective, so nothing has been announced for
||| a bare "other" to be other than, and the phrase names the land
||| itself.
public export
shelteredValley : Card
shelteredValley =
  Macros.card "Sheltered Valley" Nothing [] (MkTypeLine [] [Land])
       [ Static (Intercepts (Enters This Nothing) [] Nothing
                   (Sequentially
                      [ Macros.sacrifice You
                          (Each (And [Permanent, OtherThan Macros.thisLand,
                                      Named (PrintedName "Sheltered Valley"),
                                      ControlledBy You]))
                      , Macros.putOntoBattlefield This ])
                   Repeatedly Nothing)
       , Macros.triggeredIf At (BeginningOf Upkeep (ByWord Yours))
           (CompareAmt (CountOf (And [Macros.land, ControlledBy You]))
                       AtMost (Lit 3))
           (Macros.gainsLife You (Lit 1))
       , Macros.activated TapSymbol
                          (AddMana You (Lit 1) (Runs [[Colorless]]) []) ]
       Nothing


-- ==== The play permission's remaining subjects, riders and complements,
-- ==== and the game-outcome gates (prohibition-3)

||| Field of Dreams -- "Players play with the top card of their libraries
||| revealed." The GROUP possessor the family owed, and it is answered by
||| the rider's own complement rather than by a slice: [CR#400.1] gives
||| each player their own library, and `VisibleThing.TopOfLibrary` names
||| the position while the subject supplies whose, so a plural subject
||| pluralises the zone word with nothing added. The positioned SLICE
||| (`playersTopCardSlice` below) is a different phrase that no line of
||| this family writes.
public export
fieldOfDreams : Card
fieldOfDreams =
  Macros.card "Field of Dreams" (Just [Macros.pip Blue]) [World]
       (MkTypeLine [] [Enchantment])
       [ Static (Visibility Reveal (PlayerGroup AllPlayers) TopOfLibrary) ]
       Nothing

||| Lantern of Insight's first line, the same sentence at an artifact.
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

||| Revelation -- the same group subject at the hand surface, and the
||| second World enchantment.
public export
revelation : Card
revelation =
  Macros.card "Revelation" (Just [Macros.pip Green]) [World]
       (MkTypeLine [] [Enchantment])
       [ Static (Visibility Reveal (PlayerGroup AllPlayers) WholeHand) ]
       Nothing

||| Keeper of the Lens, whole -- the face-down look-at rider.
||| [CR#708.5] hides a face-down permanent by what it IS rather than by
||| where it is ("you can't look at ... face-down spells or permanents
||| controlled by another player"), so the complement is an object
||| DESCRIPTION and the predicate is the status the grammar already
||| carries.
public export
keeperOfTheLens : Card
keeperOfTheLens =
  Macros.card "Keeper of the Lens" (Just [Macros.generic 1]) []
       (MkTypeLine [creatureType "Golem"] [Artifact, Creature])
       [ Static (Visibility LookAt You
                   (VisibleObjects
                      (AllOf (And [Macros.creature, HasStatus FaceDown,
                                   Not (ControlledBy You)])))) ]
       (Just (1, 2))

||| Lens of Clarity, whole -- the two surfaces of one rider coordinated
||| in one sentence, which is what the shared row makes writable.
public export
lensOfClarity : Card
lensOfClarity =
  Macros.card "Lens of Clarity" (Just [Macros.generic 1]) []
       (MkTypeLine [] [Artifact])
       [ Static (AndAlso
           [ Visibility LookAt You TopOfLibrary
           , Visibility LookAt You
               (VisibleObjects
                  (AllOf (And [Macros.creature, HasStatus FaceDown,
                               Not (ControlledBy You)]))) ]) ]
       Nothing

||| Danitha, New Benalia's Light, whole -- the subtype-narrowed spell
||| complement. `And [spell, Or [Aura, Equipment]]` is coherent: a
||| subtype word presupposes a card TYPE and no zone (`seedZone` is
||| silent for it), so the permanent-only reading never reached the
||| conjunction's zone question at all and `ZoneCoherent` has nothing to
||| refuse. The permission's own `complementLocates` answered the same
||| question one level up for the same reason -- a word for what playing
||| the object will MAKE it locates nothing.
public export
danithaNewBenaliasLight : Card
danithaNewBenaliasLight =
  Macros.card "Danitha, New Benalia's Light"
       (Just [Macros.generic 1, Macros.pip Green, Macros.pip White]) [Legendary]
       (MkTypeLine [creatureType "Human", creatureType "Knight"] [Creature])
       [ Macros.keyword "Vigilance"
       , Macros.keyword "Trample"
       , Macros.keyword "Lifelink"
       , Static (Macros.mayCastFromLimited You
                   (Macros.a (And [Macros.spell,
                                   Or [HasSubtype (enchantmentType "Aura"),
                                       HasSubtype (artifactType "Equipment")]]))
                   (Macros.graveyardOf You) OnceEachYourTurn) ]
       (Just (2, 2))

||| Muldrotha, the Gravetide's window, first conjunct: "During each of
||| your turns, you may play a land ... from your graveyard." The BARE
||| window, permitting repeatedly inside a stretch of time where
||| `PlayLimit` permits once. Its second conjunct wants "a permanent
||| spell of each permanent type", a distributive over card types.
public export
muldrothaLandWindow : StaticEffect []
muldrothaLandWindow =
  Macros.mayPlayFromEachYourTurn You (Macros.a Macros.land) (Macros.graveyardOf You)

||| Nahiri's Lithoforming's third sentence, "you may play X additional
||| lands this turn" -- the one printed land allowance the literal bound
||| refused. The letter is READ from the cost [CR#107.3i], so the
||| quantity announces nothing and the widened gate admits it.
public export
nahiriExtraLands : StaticEffect (costLetters (Just [Variable]))
nahiriExtraLands = MayPlayAdditionalLands You (ExactlyOf (LetterVal X))

||| Phyrexian Unlife's first line -- the partial-cause outcome immunity.
||| It carves out [CR#104.3b] and leaves [CR#104.3e] standing, which is
||| what keeps it off the deontic carrier.
public export
phyrexianUnlifeImmunity : Ability
phyrexianUnlifeImmunity = Static (NoLossFrom You ZeroOrLessLife)

||| Haakon, Stromgald Scourge, whole -- the self-permission's EXCLUSION
||| clause, "but not from anywhere else", which nothing else in the
||| corpus writes. [CR#601.3] makes casting depend on a rule or effect
||| allowing it, so the line grants one permission and revokes the
||| default in one sentence.
public export
haakonStromgaldScourge : Card
haakonStromgaldScourge =
  Macros.card "Haakon, Stromgald Scourge"
       (Just [Macros.generic 1, Macros.pip Black, Macros.pip Black]) [Legendary]
       (MkTypeLine [creatureType "Zombie", creatureType "Knight"] [Creature])
       [ Static (Macros.mayCastFromOnly You This (Macros.graveyardOf You))
       , Static (Conditionally (Matches This (InZone Macros.battlefieldZ))
                   (Macros.mayCastFrom You
                      (AllOf (And [Macros.spell,
                                   HasSubtype (creatureType "Knight")]))
                      (Macros.graveyardOf You))
                   AsLongAs)
       , Macros.triggered When (Dies Macros.thisCreature)
           (Macros.losesLife You (Lit 2)) ]
       (Just (3, 3))

||| Melek, Izzet Paragon, whole -- the cast watch's SOURCE phrase.
||| "Whenever you cast an instant or sorcery spell FROM YOUR LIBRARY" was
||| the card's only blocker; the phrase is a fact about the casting, not
||| about the spell, which is on the stack [CR#112.1] however it got
||| there.
public export
melekIzzetParagon : Card
melekIzzetParagon =
  Macros.card "Melek, Izzet Paragon"
       (Just [Macros.generic 4, Macros.pip Blue, Macros.pip Red]) [Legendary]
       (MkTypeLine [creatureType "Weird", creatureType "Wizard"] [Creature])
       [ Static (Visibility Reveal You TopOfLibrary)
       , Static (Macros.mayCastFrom You
                   (AllOf (And [Macros.spell, Macros.instantOrSorcery]))
                   Macros.onTopZ)
       , Macros.triggered Whenever
           (Casts You (Macros.a (And [Macros.instantOrSorcery, Macros.spell]))
                  (Just Macros.yourLibrary))
           (Sequentially
              [ CopyStack You It (Lit 1) []
              , Macros.may You (ChooseNewTargets (That CopyW)) ]) ]
       (Just (2, 4))

||| Hot Pursuit's intervening "if two or more players have lost the
||| game" -- the player-set COUNT. The reader it wants was already
||| there: `HappenedTo GameLoss ThisGame` at the player kind, which
||| Rampant Frogantua's `+10/+10 for each player who has lost the game`
||| already benches. The cell wanted no new predicate, only a witness
||| that the second line reads the same one through `CountOf`.
public export
twoOrMorePlayersHaveLost : Condition []
twoOrMorePlayersHaveLost =
  CompareAmt (CountOf (And [AnyPlayer, Macros.happenedTo GameLoss ThisGame]))
             AtLeast (Lit 2)

||| Apex of Power's first line -- "Exile the top seven cards of your
||| library. Until end of turn, you may cast spells from among them."
||| The play permission's "from among" SOURCE, and the measurement's
||| correction: the phrase is the COMPLEMENT's own partitive `SomeOf`,
||| not a source-zone phrase at all. [CR#109.2a] locates a card-worded
||| description by the zone the phrase states, and a partitive states a
||| GROUP in that slot -- so "spells from among them" carries the
||| exiled cards' zone on the mention and `MayPlay`'s `from` stays
||| unwritten. The permission needed no mention-valued source slot.
public export
apexOfPowerCast : Effect []
apexOfPowerCast =
  Sequentially
    [ Macros.exile (LibrarySlice OnTop (Lit 7) You)
    , Continuously
        (MayPlay You (Macros.fromAmong Macros.anyNumber Macros.spell Them)
                 Cast Nothing Nothing Nothing Nothing False ItsOwnCost)
        (Just ThisTurn) ]

||| Umbris, Fear Manifest's first line -- "Umbris gets +1/+1 for each card
||| your opponents own in exile." The ownership PREDICATE's description
||| witness, and the reason the axis is not `ControlledBy`'s: [CR#109.4]
||| leaves an exiled card controlled by nobody, so the owner [CR#108.3] is
||| the only possessor the phrase can describe it by.
public export
umbrisPump : Ability
umbrisPump =
  Static (Gets Macros.thisCreature
               (PtUp (Macros.nForEach 1
                        (And [IsCard, OwnedBy (PlayerGroup YourOpponents),
                              InZone Macros.exileZ])))
               (PtUp (Macros.nForEach 1
                        (And [IsCard, OwnedBy (PlayerGroup YourOpponents),
                              InZone Macros.exileZ]))))

||| Obelisk of Undoing -- "{6}, {T}: Return target permanent you both own
||| and control to your hand." The non-melding carrier of "you both own
||| and control": the conjunction row reaches the phrase as two
||| descriptions the moment ownership has one. The seven cards writing
||| the same words as a CONDITION still do not land, and the meld
||| vocabulary is no longer why -- `meldInto` and its `Meld` label are
||| landed. What is left is the CONDITION's own shape: each of the seven
||| writes "you both own and control [X] and [a Y named Z]", one clause
||| over TWO named objects, and then reads them back as a single "them".
||| `AndCond` writes the two existence claims, and `Them` admits one
||| `ManyOf` mention where they leave two `OneOf`s -- so the plural
||| anaphor over two singular antecedents is the whole of the remaining
||| blocker, shared by all seven.
||| -- spelling: the destination is the BARE hand zone, as every other
||| benched return writes it. The printed "your hand" adds nothing the
||| description has not already said: the returned permanent is one YOU
||| own [CR#108.3], and a return puts a card into its owner's hand.
public export
obeliskOfUndoing : Ability
obeliskOfUndoing =
  Macros.activated (Compound [Mana [Macros.generic 6], TapSymbol])
                   (Macros.returnTo
                      (Macros.target (And [Permanent, OwnedBy You, ControlledBy You]))
                      Macros.handZ)

||| Blight Herder's cast trigger -- "you may put two cards your opponents
||| own from exile into their owners' graveyards." The ownership
||| PREDICATE over cards in exile, where [CR#109.4] leaves no controller
||| to describe them by. Ulamog's Despoiler and Ulamog's Nullifier write
||| the same sentence.
||| -- spelling: the destination is the BARE graveyard zone. "Their
||| owners' graveyards" needs no possessor here: `DestOk` already rules
||| that a moved card is routed to its owner's zone regardless of the
||| sentence [CR#400.3], so the bare zone IS the owner-rooted
||| destination, plural possessor and all.
public export
blightHerderCast : Effect []
blightHerderCast =
  Macros.may You
    (Macros.move (CountedGroup (Macros.exactly 2) Nothing
                    (And [IsCard, OwnedBy (PlayerGroup YourOpponents),
                          InZone Macros.exileZ]))
                 Macros.graveyardZ)

||| Open the Vaults -- "Return all artifact and enchantment cards from all
||| graveyards to the battlefield under their owners' control." Two
||| routed questions in one line.
||| The ALL-GRAVEYARDS possessor wanted no new row: the possessive-zone
||| reader already takes a plural player noun, so "all graveyards" is
||| `PossessedBy (PlayerGroup AllPlayers)` and the bare zone is a
||| separate spelling rather than the only one.
||| The controller rider is the member-wise possessor's witness. Each
||| returned card gets the one controller [CR#110.2] demands; the phrase
||| is plural because the GROUP is, which is exactly what
||| `PerMemberController` admits and what the singular-controller
||| refusal never meant to catch.
public export
openTheVaults : Effect []
openTheVaults =
  Enact "Return"
    (Move (AllOf (And [IsCard,
                       Or [HasType Artifact, HasType Enchantment],
                       InZone (ZoneAt Graveyard
                                 (PossessedBy (PlayerGroup AllPlayers)))]))
          Macros.battlefieldZ
          (MkMoveRiders [] (Just (PossessorsOf OwnerAx Them)) Nothing))

||| Crackling Doom, whole -- "Crackling Doom deals 2 damage to each
||| opponent. Each opponent sacrifices a creature with the greatest power
||| among creatures that player controls." The demonstrative reading a
||| DISTRIBUTED mention: the pass's member is what `agentIntro` binds, so
||| "that player" names the opponent the pass is on. Consume's singular
||| twin (`consume` above) writes the same superlative under a targeted
||| player and is unchanged.
public export
cracklingDoom : Effect []
cracklingDoom =
  Sequentially
    [ DealDamage This (Lit 2) (Each Opponent)
    , Macros.sacrifice (Each Opponent)
        (Macros.a (And [Macros.creature,
                        Superlative MaxOf (CharAxis Power)
                          (And [Macros.creature,
                                ControlledBy (That PlayerW)])])) ]

||| Altar of the Brood's trigger -- "Whenever another permanent you
||| control enters, each opponent mills a card." The DISTRIBUTIVE MILL,
||| counted before it was written: 62 supported lines write "each
||| opponent/player mills [n]" (measured 2026-08-28), all of them wanting
||| the same thing -- a library the pass's own member owns. `They` finds
||| it now that the agent seat binds one.
public export
altarOfTheBrood : Ability
altarOfTheBrood =
  Macros.triggered Whenever
    (Enters (Macros.a (And [Permanent, ControlledBy You, OtherThan This])) Nothing)
    (Macros.mills (Each Opponent) (Lit 1) They)

||| Soul Shatter -- "Each opponent sacrifices a creature or planeswalker
||| with the greatest mana value among creatures and planeswalkers they
||| control." The same distributed subject read back by the PRONOUN
||| rather than the demonstrative; one binder answers both spellings.
public export
soulShatter : Effect []
soulShatter =
  Macros.sacrifice (Each Opponent)
    (Macros.a (And [Or [Macros.creature, HasType Planeswalker],
                    Superlative MaxOf (CharAxis ManaValue)
                      (And [Or [Macros.creature, HasType Planeswalker],
                            ControlledBy They])]))

||| Padeem, Consul of Innovation's upkeep trigger -- "At the beginning of
||| your upkeep, if you control the artifact with the greatest mana value
||| or tied for the greatest mana value, draw a card." A control test over
||| a DEFINITE description: the phrase names THE artifact with the
||| greatest mana value and then asks who controls it, which is not what
||| `Exists` says. The definite's mention is announced into the governed
||| clause rather than dropped at the condition's door.
||| Six supported lines write this (re-measured 2026-08-28): Abzan
||| Beastmaster and Thickest in the Thicket on toughness and power, this
||| card on mana value, Summon: Fenrir, Triumph of Cruelty and Triumph of
||| Ferocity on power.
||| -- spelling: "or tied for the greatest [axis]" is the superlative's own
||| bound said twice; the extremal fold admits every referent that reaches
||| the maximum, so the printed disjunct adds no second test.
public export
padeemConsulOfInnovation : Ability
padeemConsulOfInnovation =
  Macros.triggeredIf At (BeginningOf Upkeep (ByWord Yours))
    (Matches (Definite (And [Macros.artifact,
                             Superlative MaxOf (CharAxis ManaValue)
                               (And [Macros.artifact,
                                     InZone Macros.battlefieldZ])]))
             (ControlledBy You))
    Macros.drawACard

||| Turbulent Fen -- "This land enters tapped unless your opponents
||| control eight or more lands." The counted threshold over a POSSESSOR
||| SET, written as the umbrella asked: an independent counted clause over
||| a described set, never a possessor relation on a mention. The other
||| four Turbulent lands print the same sentence and Lashwhip Predator
||| writes the same clause at three creatures -- six supported lines,
||| re-measured 2026-08-28 against 259 lines writing "your opponents
||| control" at all.
||| No row was needed: `CountOf` already takes a description and
||| `ControlledBy (PlayerGroup YourOpponents)` already describes one.
public export
turbulentFen : Ability
turbulentFen =
  Static (OnlyWhile (Macros.entersTapped Macros.thisLand)
                    (NotCond (CompareAmt
                                (CountOf (And [Macros.land,
                                               ControlledBy (PlayerGroup YourOpponents)]))
                                AtLeast (Lit 8)))
                    Unless)

||| Darkblade Agent's first grant -- "As long as you've surveilled this
||| turn, this creature has deathtouch". The KEYWORD-ACTION event name,
||| and the round's answer to the pair the umbrella routed together: the
||| name already exists. `VerbedAct` carries the label [CR#701.1] leaves
||| open, so the per-turn state read is `Happened (VerbedAct "Surveil")`
||| and wanted nothing new -- `bareLookbackOk` already admits a
||| subjectless act ([CR#701.25a]'s surveil names no patient), and Eye of
||| Duskmantle's "cards in your graveyard you've surveilled this turn"
||| reads the same event.
||| Yidaro's cycling count is the SAME answer arriving negative, and that
||| is why the two are one item: [CR#702.29c] defines "when you cycle this
||| card" as "when you discard this card to pay an activation cost of a
||| cycling ability", so cycling is no keyword action and a `VerbedAct
||| "Cycle"` label would be a fiction. What Yidaro counts is a labelled
||| COST PAYMENT, which is `PaysCost`'s neighbourhood; the card stays
||| blocked there and not here.
public export
darkbladeAgentDeathtouch : Ability
darkbladeAgentDeathtouch =
  Static (Conditionally (Macros.happened (VerbedAct "Surveil") You ThisTurn)
                        (Gains Macros.thisCreature (Macros.keyword "Deathtouch"))
                        AsLongAs)

||| The Fallen, whole -- "At the beginning of your upkeep, this creature
||| deals 1 damage to each opponent and planeswalker it has dealt damage
||| to this game." The umbrella priced this as a THIRD reader shape --
||| a relative clause with both an overt subject and its head in the
||| complement position, being neither `Happened` nor `HappenedTo`. The
||| re-derivation says otherwise and the round takes the smaller answer:
||| `HappenedTo` at the VICTIM's voice already puts the dealer in the
||| complement. [CR#120.1]'s two sides are two event names here, and
||| reading the same clause as `DamageTaken` rather than `DamageDealing`
||| moves the head to the subject seat and "it" to the complement, which
||| is the shape the row has always had.
||| The head is a JOINED kind and is read as one head over it, per
||| [The kind index joins; union marking is spelling]: `lookbackSubjectOk`
||| and `lookbackComplementOk` both distribute `DamageTaken` over the
||| join, so the join costs nothing here either.
||| No row was minted. The per-game lookback keeps its first benched
||| carrier past Approach of the Second Sun.
public export
theFallen : Ability
theFallen =
  Macros.triggered At (BeginningOf Upkeep (ByWord Yours))
    (DealDamage Macros.thisCreature (Lit 1)
       (Each (And [Joined Opponent (HasType Planeswalker),
                   Macros.happenedToInvolving DamageTaken ThisGame
                     Macros.thisCreature])))

||| Codecracker Hound's second line -- "Look at the top two cards of your
||| library. Put one into your hand and the other into your graveyard."
||| The SUBSET COMPLEMENT, and the cardinality that makes it writable: the
||| library slice records that it is two cards, the partitive records that
||| it took one, and "the other" is the row that can ask. `TheRest` would
||| have written the same partition in the plural and mis-spelled the
||| sentence.
||| A Little Chat, Akal Pakal, Chrome Courier, Ashiok, Wicked Manipulator
||| and Atris, Oracle of Half-Truths write the same shape into different
||| destinations; 104 occurrences over 102 supported cards write the word
||| at all (re-measured 2026-08-28).
public export
codecrackerHoundLook : Effect []
codecrackerHoundLook =
  Sequentially
    [ Macros.lookAt (Macros.topSlice (Lit 2))
    , Macros.move (Macros.oneOf Them) Macros.handZ
    , Macros.move TheOther Macros.graveyardZ ]

||| The subset complement's gate, measured from both sides. A group the
||| text COUNTED leaves a singleton once all but one member is taken, and
||| "the other" names it; a group whose size is read at resolution leaves
||| a remainder of unknown size, and the sentence writes "the rest".
||| This is what the cardinality on `ObjectP` buys, and the only thing it
||| buys: nothing else reads a mention's size.
public export
theOtherAfterATwoCardLook :
  theOtherOk (nomIntro (SomeOf {bs = []} (Macros.exactly 1) Nothing
                               (LibrarySlice OnTop (Lit 2) You)
                               {gm = Oh} {nz = MaxAtLeastOne} {wf = Oh})) = True
theOtherAfterATwoCardLook = Refl

public export
theOtherNeedsAStatedCount :
  theOtherOk (nomIntro (SomeOf {bs = []} (Macros.exactly 1) Nothing
                               (LibrarySlice OnTop (CountOf Macros.creature) You)
                               {gm = Oh} {nz = MaxAtLeastOne} {wf = Oh})) = False
theOtherNeedsAStatedCount = Refl

public export
theRestStandsWhereTheOtherRefuses :
  theRestOk (nomIntro (SomeOf {bs = []} (Macros.exactly 1) Nothing
                              (LibrarySlice OnTop (CountOf Macros.creature) You)
                              {gm = Oh} {nz = MaxAtLeastOne} {wf = Oh})) = True
theRestStandsWhereTheOtherRefuses = Refl

||| Talion, the Kindly Lord's trigger -- "Whenever an opponent casts a
||| spell with mana value, power, or toughness equal to the chosen
||| number, that player loses 2 life and you draw a card." The
||| CHARACTERISTIC LIST on `Compare`, and why the list rather than an
||| `Or`: `seedsUniform`'s refusal of three arms presupposing different
||| card types was correct, so the fix is to stop writing three arms.
||| One referent, one bound, three places to look.
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
                         , Macros.drawACard ]) ]
       (Just (3, 4))

||| Cavalcade of Calamity -- "Whenever a creature you control with power 1
||| or less attacks, this enchantment deals 1 damage to the player or
||| planeswalker that creature is attacking." The ATTACKED-BY predicate,
||| routed here from the combat round: no `Predicate` described a player
||| or planeswalker by what was attacking it, and this one does it at the
||| joined kind [CR#506.3] rather than as two marked rows.
||| Raid Bombardment prints the same sentence at power 2 or less.
public export
cavalcadeOfCalamity : Ability
cavalcadeOfCalamity =
  Macros.triggered Whenever
    (Macros.attacks (Macros.a (And [Macros.creature, ControlledBy You,
                                    Compare [CharAxis Power] AtMost (Lit 1)])))
    (DealDamage Macros.thisEnchantment (Lit 1)
       (Definite (And [Joined AnyPlayer (HasType Planeswalker),
                       AttackedBy (That (TypeW Creature))])))

||| Bastion Protector's first clause -- "Commander creatures you control
||| get +2/+2". The COMMANDER description, routed here with the ownership
||| row it shares a phrase with. 37 supported lines head a phrase with the
||| word (measured 2026-08-28), split between "you control" and "you own".
public export
bastionProtectorPump : Ability
bastionProtectorPump =
  Static (Gets (AllOf (And [Macros.creature, HasCardDesignation CommanderD,
                            ControlledBy You]))
               (PtUp (Lit 2)) (PtUp (Lit 2)))

||| "Commander creatures you own" -- the owned half of the same phrase,
||| which needed BOTH of this round's description rows: the card-scoped
||| designation [CR#903.3] and the ownership predicate [CR#108.3]. Master
||| Chef, Acolyte of Bahamut, Candlekeep Sage and their kin head their
||| granted abilities with it; what those cards still wait on is the
||| QUOTED ability they grant, not the description.
public export
commanderCreaturesYouOwn : Predicate [] Object
commanderCreaturesYouOwn =
  And [Macros.creature, HasCardDesignation CommanderD, OwnedBy You]

||| "a creature spell from among cards exiled with this artifact" (Idol
||| of Endurance) -- the OBJECT PARTITIVE over a DESCRIBED group, and the
||| decision this round owed. The base of a partitive is not the base of
||| "each of", so the two stopped sharing a gate: "each of" fills a
||| determiner position and its base must leave one open, while "from
||| among" names the set the pick comes out of. 86 supported lines write
||| "from among [a description]" (measured 2026-08-28) -- exiled-with
||| sets, "the nonland permanents they control", "creatures you control".
||| The three pins that refuse an indefinite base, a counted untargeted
||| group and a partitive of a partitive are untouched, "each of all
||| creatures" still refuses, and `groupMention (PlayerGroup _)` is
||| unchanged.
public export
creatureSpellFromAmongExiled : Noun [] Object
creatureSpellFromAmongExiled =
  Macros.oneFromAmong (And [Macros.creature, Macros.spell])
                      (AllOf (And [IsCard, Macros.exiledWithThisArtifact]))

||| "the Ring has tempted you two or more times this game" (Frodo,
||| Adventurous Hobbit; Frodo, Sauron's Bane writes four). The umbrella
||| recorded these as out of reach for want of Ring machinery; the
||| trigger round's `verbFacts` row for the temptation put the COUNT in
||| reach, and this is the check on that. [CR#701.54d] makes the tempted
||| player the act's own subject, which is why the count takes `You` and
||| writes no complement.
||| What the two cards still wait on is the Ring-BEARER possessive ("if
||| Frodo is your Ring-bearer"): the designation is checked and
||| object-scoped, so `HasDesignation RingBearer` says "is a
||| Ring-bearer", and the possessive that names WHOSE has no row.
public export
ringHasTemptedYouTwiceThisGame : Condition []
ringHasTemptedYouTwiceThisGame =
  CompareAmt (Macros.eventCount (VerbedAct "The Ring Tempts You") You ThisGame)
             AtLeast (Lit 2)

||| "a creature card" with no zone written -- the bare CARD word routed
||| here from the placement round. It needed no row either: `IsCard` is
||| the head [CR#109.2] names among the four words that take a
||| description out of the battlefield default, and conjoining it with a
||| type word is what spells the printed phrase. Syr Konrad's second arm,
||| Disa the Restless and the two Ultrons spell "a creature" today and
||| can spell the card word now.
public export
creatureCardAnywhere : Predicate [] Object
creatureCardAnywhere = And [Macros.creature, IsCard]

||| "a permanent card" -- the class the anaphora round measured at 4
||| blocked cards, and the second thing the bare CARD word closes. The
||| card word takes the head OUT of the battlefield default [CR#109.2]
||| and `headIsPlaceless` reads it, so the conjunction is placeless: "a
||| card of a permanent type, wherever it is", which is the printed
||| class. Measured beside the phrase, because the whole question was
||| where the phrase lands.
public export
permanentCardAnywhere : Predicate [] Object
permanentCardAnywhere = And [Permanent, IsCard]

public export
permanentCardIsPlaceless :
  phraseZone (And {bs = []} [Permanent, IsCard]) = Nothing
permanentCardIsPlaceless = Refl

||| "target opponent who has more life than you do" (Keeper of the Flame,
||| Keeper of the Light) -- the player-headed comparison, and the PLAYER
||| CELL of `Compare`. The row was not duplicated to reach it: the axis
||| slot is a `ProjAxis`, so the same constructor that reads an object's
||| power reads a player's life total, and `AxesAt` scopes the list to
||| the kind. What the condition frame already said with `CompareAmt`
||| over `PlayerStatOf` the description frame now says of a referent it
||| does not have to name.
||| 18 supported lines write "has more life than" (measured 2026-08-28);
||| 4 of them put the comparison on a DESCRIPTION -- the two Keepers,
||| Oath of Mages and Namor, Atlantean King.
||| Neither Keeper benches whole: both restrict the choice to the moment
||| the ability is activated ("as you activate this ability"), for which
||| this vocabulary has nothing.
public export
opponentWithMoreLifeThanYou : Predicate [] Player
opponentWithMoreLifeThanYou =
  And [Opponent, Compare [PlayerStatAxis LifeTotal] Greater
                         (PlayerStatOf LifeTotal You)]

||| Namor, Atlantean King's trigger head -- "a player who has more life
||| than you". The same cell over the unnarrowed player noun, which is
||| what says the narrowing rides the head and not the comparison.
||| Namor does not bench whole: its body describes creatures by the
||| defender they are attacking ("other creatures you control attacking
||| that player"), the ATTACKER's voice of the row sub-round 1 built at
||| the defender's (`AttackedBy`), and no predicate writes it -- 4 more
||| supported lines want the same voice under "one of your opponents"
||| (Martial Impetus, Oviya, Scriv, Seifer; measured 2026-08-28).
public export
playerWithMoreLifeThanYou : Predicate [] Player
playerWithMoreLifeThanYou =
  And [AnyPlayer, Compare [PlayerStatAxis LifeTotal] Greater
                          (PlayerStatOf LifeTotal You)]

||| The ability the Commander-grant family quotes -- "Whenever this
||| creature attacks a player, if no opponent has more life than that
||| player, put a +1/+1 counter on this creature" (Agent of the Shadow
||| Thieves). The NEGATED EXISTENTIAL over players, 5 supported lines
||| (Agent of the Shadow Thieves, Guild Artisan, Hardy Outlander, Sword
||| Coast Sailor, Veteran Soldier -- re-measured 2026-08-28).
||| The umbrella's premise is corrected rather than built on: the shape
||| IS `NotCond`'s negation of a whole condition, because the existential
||| is the whole condition. "No opponent has more life than that player"
||| denies that any opponent answers the description, and `Exists` is
||| what asks that question; putting `NotCond` outside it scopes the
||| negation over exactly the quantifier.
||| It needed the player cell of `Compare` and could not have used
||| `CompareOver`: that row binds a member of its domain and reads it
||| back as `They`, and this clause has a second singular player in scope
||| (the attacked one), so the pronoun would resolve to neither.
||| A FRAGMENT: all five carriers grant the ability with "Commander
||| creatures you own have [...]", and the quoted grant has no
||| vocabulary. The description they head is benched at
||| `commanderCreaturesYouOwn`.
public export
agentOfTheShadowThievesGrantedTrigger : AbilityAt []
agentOfTheShadowThievesGrantedTrigger =
  Triggered Whenever
    (Attacks Macros.thisCreature (OneDefender (Macros.a AnyPlayer)))
    [] Nothing [] Nothing Nothing
    (Just (NotCond (Exists (And [Opponent,
             Compare [PlayerStatAxis LifeTotal] Greater
                     (PlayerStatOf LifeTotal (That PlayerW))]))))
    (PutCounters (Lit 1) (PrintedKind Macros.plusOnePlusOne)
                 Macros.thisCreature)

||| The three-way distinction the negated existential has to keep,
||| measured at the bare context so the shapes stand side by side. This
||| one is the negated existential itself: "no opponent has more life
||| than you".
public export
noOpponentHasMoreLifeThanYou : Condition []
noOpponentHasMoreLifeThanYou =
  NotCond (Exists (And [Opponent, Compare [PlayerStatAxis LifeTotal] Greater
                                          (PlayerStatOf LifeTotal You)]))

||| ...beside `Exists (Not ...)`, which negates the DESCRIPTION and not
||| the quantifier: "an opponent doesn't have more life than you" is true
||| whenever any one opponent is at or below you and says nothing about
||| the rest. A different condition, and the grammar keeps them apart by
||| where the negation sits.
public export
someOpponentLacksMoreLifeThanYou : Condition []
someOpponentLacksMoreLifeThanYou =
  Exists (And [Opponent,
               Not (Compare [PlayerStatAxis LifeTotal] Greater
                            (PlayerStatOf LifeTotal You))])

||| ...and beside the PLURAL player read, which tests the group rather
||| than quantifying over its members: "your opponents have more life
||| than you". `groupMention (PlayerGroup _)` is still False, so nothing
||| partitive is reachable through this mention.
public export
yourOpponentsHaveMoreLifeThanYou : Condition []
yourOpponentsHaveMoreLifeThanYou =
  Matches (PlayerGroup YourOpponents)
          (Compare [PlayerStatAxis LifeTotal] Greater
                   (PlayerStatOf LifeTotal You))

||| Mirror Universe, WHOLE CARD -- "{T}, Sacrifice this artifact:
||| Exchange life totals with target opponent. Activate only during your
||| upkeep." The life exchange's first witness, and the coordination
||| spelling of the parties: the printed clause writes only the second
||| party, and the first is the unwritten "you".
||| Magus of the Mirror prints the same ability on a creature.
public export
mirrorUniverse : Card
mirrorUniverse =
  Macros.card "Mirror Universe" (Just [Macros.generic 6]) []
       (MkTypeLine [] [Artifact])
       [ Macros.activatedOnlyDuring
           (Compound [TapSymbol, Do (Macros.sacrifice You Macros.thisArtifact)])
           (ExchangeLife (BothOf You (Macros.target Opponent)))
           (DuringPart Upkeep (Just Yours)) ]
       Nothing

||| Soul Conduit's ability -- "{6}, {T}: Two target players exchange life
||| totals." The exchange's OTHER spelling: one mention counted at two,
||| where Mirror Universe coordinates two mentions. Both fill the one
||| party slot, which is what says the row asks for two players and not
||| for a way of naming them. Axis of Mortality and Profane Transfusion
||| write the same counted mention.
public export
soulConduitExchange : Ability
soulConduitExchange =
  Macros.activated (Compound [Mana [Macros.generic 6], TapSymbol])
                   (ExchangeLife (TargetGroup (Macros.exactly 2) AnyPlayer))

||| Frenzied Gorespawn's second line -- "Whenever one or more creatures
||| attack one of your opponents, those creatures gain menace until end
||| of turn." The witness for "one of your opponents": the phrase is the
||| ordinary indefinite over the opponent head, and the spelling is the
||| renderer's (`Macros.anOpponent`). Menace needed no data row -- its
||| `keywordFacts` entry already stands.
||| A FRAGMENT, and the umbrella's "one gap from whole" is corrected: the
||| card's FIRST line is "for each opponent, goad target creature that
||| player controls", and `ForEachOf` takes an OBJECT group, so a loop
||| over a player group has no row here.
public export
frenziedGorespawnMenaceTrigger : AbilityAt []
frenziedGorespawnMenaceTrigger =
  Triggered Whenever
    (Attacks (CountedGroup (Macros.atLeast 1) Nothing Macros.creature)
             (OneDefender Macros.anOpponent))
    [] Nothing [] Nothing Nothing Nothing
    (Macros.gains (Those (TypeW Creature)) (Macros.keyword "Menace")
                  (Just Macros.untilEndOfTurn))

||| "Each player shuffles their hand and graveyard into their library" --
||| the DISTRIBUTIVE POSSESSIVE, routed here from the coordination round
||| as the dominant surface of Weftwalking's family. Re-measured
||| 2026-08-28: 38 supported lines write a distributive possessive under
||| "each player", 29 of them the shuffle family ("each player shuffles
||| their hand and graveyard into their library", Commit // Memory,
||| Day's Undoing, Diminishing Returns, Echo of Eons and their kin).
||| It needed NO row: sub-round 1's `agentIntro` lift binds one member of
||| an `Each` agent in place of the group mention, so the possessive
||| inside the clause is the ordinary `They` reading that member, and the
||| coordinated mass object is the one `weftwalkingShuffle` already
||| wrote. The "your" spelling stays exactly as it was.
public export
eachPlayerShufflesTheirHandAndGraveyard : Effect []
eachPlayerShufflesTheirHandAndGraveyard =
  Macros.shufflesInto (Each AnyPlayer)
    (BothOf (AllOf (InZone (Macros.handOf They)))
            (AllOf (InZone (Macros.graveyardOf They))))

||| Pir, Imaginative Rascal's replacement -- "If one or more counters
||| would be put on a permanent your team controls, that many plus one of
||| each of those kinds of counters are put on that permanent instead."
||| The TEAM form, decided on [CR#102.4]: "your team" is a player-group
||| VALUE and not a spelling of `You`, because the rule makes it
||| shorthand for "you and/or your teammates" -- more than one player
||| wherever the game has teams [CR#102.3] -- and collapses it to "you"
||| only in a game that is not between teams. 15 supported lines write
||| the phrase (measured 2026-08-28).
||| Doc Samson's note that Pir waited on the team form ALONE was
||| corrected here; the second blocker was "Partner with Toothy,
||| Imaginary Friend", and with [CR#702.124j]'s row the card is whole at
||| `pirImaginativeRascal`.
public export
pirDistributive : Ability
pirDistributive =
  Static (Intercepts
            (Macros.manyBareCountersPutBy You
               (Macros.a (And [Permanent, ControlledBy (PlayerGroup YourTeam)])))
            [] Nothing
            (PutCountersOfThoseKinds (Plus ThatMuch (Lit 1)) (That PermanentW))
            Repeatedly Nothing)


-- ===================================================================
-- The keyword row's parameters, its class term and the catalog rows
-- ===================================================================

||| Frogmite, whole -- "Affinity for artifacts". AFFINITY's witness: the
||| parameter is a described CLASS [CR#702.41a] ("this spell costs {1}
||| less to cast for each [text] you control"), which is the quality
||| payload and not a number, so the row needed no new shape.
||| 30 distinct printed spellings over 74 supported cards write the line;
||| Frogmite is the shortest of them.
public export
frogmite : Card
frogmite =
  Macros.card "Frogmite" (Just [Macros.generic 4]) []
       (MkTypeLine [creatureType "Frog"] [Artifact, Creature])
       [ Macros.keywordQuality "Affinity" Macros.artifact ]
       (Just (2, 2))

||| Ulamog's Crusher, whole -- "Annihilator 2" and "This creature attacks
||| each combat if able." ANNIHILATOR's cheapest consumer: 21 supported
||| lines write the word, 13 of them as a printed keyword line, and this
||| is the only one whose other line was already written
||| (`berserkersOfBloodRidge`'s deontic, verbatim).
public export
ulamogsCrusher : Card
ulamogsCrusher =
  Macros.card "Ulamog's Crusher" (Just [Macros.generic 8]) []
       (MkTypeLine [creatureType "Eldrazi"] [Creature])
       [ Macros.keywordNumber "Annihilator" (Lit 2)
       , Static (Macros.deontic Macros.thisCreature Require ["Attack"] Agent
                                NoDeonticPatient) ]
       (Just (8, 8))

||| Iymrith, Desert Doom, whole -- the PARAMETERISED ward line the row
||| was minted for and had no witness at: "Iymrith has ward {4} as long
||| as it's untapped" [CR#702.21a]. Benched at Dragonlord Ojutai's
||| cost-free spelling of the same postposed static, whose fragment
||| `dragonlordOjutaiHexproof` is this one with the parameter dropped.
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
                          (DealsCombatDamage Macros.thisCreature (Macros.a AnyPlayer))
                          (Sequentially
                             [ Macros.drawACard
                             , If (CompareAmt (CountOf (InZone (Macros.handOf You)))
                                              Less (Lit 3))
                                  (Draw You TheDifference)
                                  Nothing ]) ]
       (Just (5, 5))

||| True-Name Nemesis, whole -- "As this creature enters, choose a
||| player." and "This creature has protection from the chosen player."
||| The PLAYER payload: [CR#702.16k] makes "protection from [a player]"
||| a variant of the ability whose slot names a player outright, and
||| [CR#109.3] makes a player no characteristic of anything, so the
||| phrase names its referent instead of matching one -- `ChosenPlayer`,
||| the head-noun read, at the quality payload's kind-indexed slot.
||| Two supported cards write the phrase; Guardian Archon is the other
||| and stays off the bench on its own line ("You and target permanent
||| you control each gain ...", a mixed-group subject).
public export
trueNameNemesis : Card
trueNameNemesis =
  Macros.card "True-Name Nemesis"
       (Just [Macros.generic 1, Macros.pip Blue, Macros.pip Blue]) []
       (MkTypeLine [creatureType "Merfolk", creatureType "Rogue"] [Creature])
       [ Static (Macros.entersChoosingPlayer Macros.thisCreature Nothing)
       , Macros.keywordQuality "Protection" ChosenPlayer ]
       (Just (3, 1))

||| The two keyword CLASSES the seven blocked extension lists name, as
||| terms: [CR#702.16a] writes every protection ability as "Protection
||| from [quality]", so "protection" with nothing after it is that word
||| with its quality left open; [CR#702.14a] makes landwalk "a generic
||| term that appears within an object's rules text as '[type]walk'",
||| which is that word with its land type left open.
public export
protectionAbilities : KeywordTerm
protectionAbilities = AnyKeywordIn (MkKeywordFamily "Protection" Nothing)

public export
landwalkAbilities : KeywordTerm
landwalkAbilities = AnyKeywordIn (MkKeywordFamily "Landwalk" Nothing)

||| "protection from any color": the same word with its quality NARROWED
||| to a sort rather than left open -- [CR#105.1]'s five colors, not
||| [CR#702.16a]'s whole "any characteristic value or information".
||| Escaped Shapeshifter is its only supported carrier.
public export
protectionFromAnyColor : KeywordTerm
protectionFromAnyColor = AnyKeywordIn (MkKeywordFamily "Protection" (Just Color))

||| Cairn Wanderer, whole -- "Changeling" and "As long as a creature card
||| with flying is in a graveyard, this creature has flying. The same is
||| true for fear, first strike, double strike, deathtouch, haste,
||| landwalk, lifelink, protection, reach, trample, shroud, and
||| vigilance."
||| The keyword-list extension's longest list and the one that buys the
||| most: both class terms, both paramless words whose only carriers were
||| the class-blocked seven (`fear`, 3 lines; `shroud`, 1), and the
||| changeling row. Seven of the family's fifteen lists name a class;
||| this is the first of them to write.
public export
cairnWanderer : Card
cairnWanderer =
  Macros.card "Cairn Wanderer" (Just [Macros.generic 4, Macros.pip Black]) []
       (MkTypeLine [creatureType "Shapeshifter"] [Creature])
       [ Macros.keyword "Changeling"
       , AlsoForKeywords
           (Static (Macros.asLongAs
                      (Exists (And [Macros.creature, InZone Macros.graveyardZ,
                                    HasKeyword (TheKeyword "Flying")]))
                      (Gains Macros.thisCreature (Macros.keyword "Flying"))))
           [ TheKeyword "Fear", TheKeyword "FirstStrike"
           , TheKeyword "DoubleStrike", TheKeyword "Deathtouch"
           , TheKeyword "Haste", landwalkAbilities, TheKeyword "Lifelink"
           , protectionAbilities, TheKeyword "Reach", TheKeyword "Trample"
           , TheKeyword "Shroud", TheKeyword "Vigilance" ] ]
       (Just (4, 4))

||| Concerted Effort, whole -- "At the beginning of each upkeep,
||| creatures you control gain flying until end of turn if a creature you
||| control has flying. The same is true for fear, first strike, double
||| strike, landwalk, protection, trample, and vigilance." Odric's
||| triggered shape at the upkeep, with two class terms in the list.
public export
concertedEffort : Card
concertedEffort =
  Macros.card "Concerted Effort"
       (Just [Macros.generic 2, Macros.pip White, Macros.pip White]) []
       (MkTypeLine [] [Enchantment])
       [ AlsoForKeywords
           (Macros.triggeredIf At
                               (BeginningOf Upkeep (ByWord EachPlayers))
                               (Exists (And [Macros.creature, ControlledBy You,
                                             HasKeyword (TheKeyword "Flying")]))
                               (Continuously
                                  (Gains (AllOf Macros.creatureYouControl)
                                         (Macros.keyword "Flying"))
                                  (Just Macros.untilEndOfTurn)))
           [ TheKeyword "Fear", TheKeyword "FirstStrike"
           , TheKeyword "DoubleStrike", landwalkAbilities, protectionAbilities
           , TheKeyword "Trample", TheKeyword "Vigilance" ] ]
       Nothing

||| Death-Mask Duplicant, whole -- "Imprint — {1}: Exile target creature
||| card from your graveyard." and "As long as a card exiled with this
||| creature has flying, this creature has flying. The same is true for
||| fear, first strike, double strike, haste, landwalk, protection, and
||| trample." Urborg Scavengers' base sentence under an ability word,
||| with the two class terms in the trailer.
public export
deathMaskDuplicant : Card
deathMaskDuplicant =
  Macros.card "Death-Mask Duplicant" (Just [Macros.generic 7]) []
       (MkTypeLine [creatureType "Shapeshifter"] [Artifact, Creature])
       [ AbilityWord Imprint
           (Macros.activated (Mana [Macros.generic 1])
                             (Macros.exile
                                (Macros.target
                                   (And [Macros.creature,
                                         InZone (Macros.graveyardOf You)]))))
       , AlsoForKeywords
           (Static (Macros.asLongAs
                      (Exists (And [ExiledWith Macros.thisCreature,
                                    HasKeyword (TheKeyword "Flying")]))
                      (Gains Macros.thisCreature (Macros.keyword "Flying"))))
           [ TheKeyword "Fear", TheKeyword "FirstStrike"
           , TheKeyword "DoubleStrike", TheKeyword "Haste"
           , landwalkAbilities, protectionAbilities, TheKeyword "Trample" ] ]
       (Just (5, 5))

||| Autarch Mammoth, whole -- its header line plus "Saddle 5". The
||| DESIGNATION half was already there (`Saddled`, `Macros.becomesSaddled`);
||| what was missing was the word, and [CR#702.171a]'s "Saddle N" writes a
||| number after it exactly as [CR#702.122a]'s "Crew N" does.
public export
autarchMammoth : Card
autarchMammoth =
  Macros.card "Autarch Mammoth"
       (Just [Macros.generic 4, Macros.pip Green, Macros.pip Green]) []
       (MkTypeLine [creatureType "Elephant", creatureType "Mount"] [Creature])
       [ autarchMammothLine
       , Macros.keywordNumber "Saddle" (Lit 5) ]
       (Just (5, 5))

||| Debris Beetle, whole -- "Trample", the enters drain, and "Crew 2".
||| CREW-THE-WORD, which is all this round buys of crew: [CR#702.122a]
||| writes "Crew N" with a number after the word, and the crewing itself
||| -- the tap-a-set-of-creatures-by-total-power cost -- is the keyword's
||| own expansion and not a construction this grammar writes.
public export
debrisBeetle : Card
debrisBeetle =
  Macros.card "Debris Beetle"
       (Just [Macros.generic 2, Macros.pip Black, Macros.pip Green]) []
       (MkTypeLine [artifactType "Vehicle"] [Artifact])
       [ Macros.keyword "Trample"
       , Macros.triggered When (Enters Macros.thisVehicle Nothing)
           (Sequentially [ Macros.losesLife (Each Opponent) (Lit 3)
                         , Macros.gainsLife You (Lit 3) ])
       , Macros.keywordNumber "Crew" (Lit 2) ]
       (Just (6, 6))

||| Pir, Imaginative Rascal, whole -- "Partner with Toothy, Imaginary
||| Friend" and the team distributive `pirDistributive` already carried.
||| [CR#702.124j]'s slot is a card NAME; [CR#109.3] lists name among an
||| object's characteristics and [CR#702.16a] takes a quality to be "any
||| characteristic value or information", so the payload is `Named`'s
||| printed-name predicate at the quality slot. The reminder text is the
||| second ability [CR#702.124j] gives the word and not a printed line.
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

||| Bloodline Pretender, whole -- "Changeling", "As this creature enters,
||| choose a creature type." and "Whenever another creature you control of
||| the chosen type enters, put a +1/+1 counter on this creature." Prism
||| Ring's chooser-then-reader shape on a creature, with the changeling
||| row as its last blocker.
public export
bloodlinePretender : Card
bloodlinePretender =
  Macros.card "Bloodline Pretender" (Just [Macros.generic 3]) []
       (MkTypeLine [creatureType "Shapeshifter"] [Artifact, Creature])
       [ Macros.keyword "Changeling"
       , Static (Macros.entersChoosing Macros.thisCreature (SubtypeQ Creature))
       , Macros.triggered Whenever
           (Enters (Macros.a (And [Macros.creature, ControlledBy You,
                                   OtherThan Macros.thisCreature,
                                   OfChosen (SubtypeQ Creature)]))
                   Nothing)
           (PutCounters (Lit 1) (PrintedKind Macros.plusOnePlusOne)
                        Macros.thisCreature) ]
       (Just (2, 2))

||| Foul Emissary's second line -- "When you sacrifice this creature
||| while casting a spell with emerge, create a 3/2 colorless Eldrazi
||| Horror creature token." The concurrent clause's ACT arm at its
||| second printed shape, and what it wanted was the keyword: [CR#702.119a]
||| writes "Emerge [cost]" as an alternative cost, so the row is a cost
||| row, and `HasKeyword` reads it off the spell being cast.
||| The card is not whole: its other line looks at the top four cards,
||| reveals one from among them and bottoms the rest.
public export
foulEmissaryLine : Ability
foulEmissaryLine =
  Macros.triggeredWhile When
    (VerbedEvent (Just You) "Sacrifice" (Just Macros.thisCreature) False)
    (WhileDoing (Casts You
                   (Macros.a (And [Macros.spell,
                                   HasKeyword (TheKeyword "Emerge")]))
                   Nothing))
    (Macros.create (Lit 1)
       (Macros.creatureTok 3 2 []
          [creatureType "Eldrazi", creatureType "Horror"]))

||| Market Gnome, whole -- "When this creature dies, you gain 1 life and
||| draw a card." and "When this creature is exiled from the battlefield
||| while you're activating a craft ability, you gain 1 life and draw a
||| card." The concurrent clause's ACT arm at its third and last printed
||| shape. It needed a "Craft" row and an ability DESCRIBED by a keyword;
||| the description was already there (`AbilityClass`' `KeywordClass`),
||| and [CR#702.167a]'s "Craft with [materials] [cost]" is one activation
||| cost written in two printed pieces, so the row needed no compound
||| parameter shape.
public export
marketGnome : Card
marketGnome =
  Macros.card "Market Gnome" (Just [Macros.pip White]) []
       (MkTypeLine [creatureType "Gnome"] [Artifact, Creature])
       [ Macros.triggered When (Dies Macros.thisCreature)
           (Sequentially [ Macros.gainsLife You (Lit 1), Macros.drawACard ])
       , Macros.triggeredWhile When
           (VerbedEvent Nothing "Exile" (Just Macros.thisCreature) False)
           (WhileDoing (Activates You
                          (Macros.a (AbilityHead (KeywordClass "Craft")))))
           (Sequentially [ Macros.gainsLife You (Lit 1), Macros.drawACard ]) ]
       (Just (0, 3))

||| Escaped Shapeshifter, whole -- "As long as an opponent controls a
||| creature with flying not named Escaped Shapeshifter, this creature
||| has flying. The same is true for first strike, trample, and
||| protection from any color." The NARROWED class term's only supported
||| carrier: [CR#105.1]'s five colors, not [CR#702.16a]'s whole quality
||| range.
public export
escapedShapeshifter : Card
escapedShapeshifter =
  Macros.card "Escaped Shapeshifter"
       (Just [Macros.generic 3, Macros.pip Blue, Macros.pip Blue]) []
       (MkTypeLine [creatureType "Shapeshifter"] [Creature])
       [ AlsoForKeywords
           (Static (Macros.asLongAs
                      (Exists (And [Macros.creature,
                                    ControlledBy Macros.anOpponent,
                                    HasKeyword (TheKeyword "Flying"),
                                    Not (Named (PrintedName "Escaped Shapeshifter"))]))
                      (Gains Macros.thisCreature (Macros.keyword "Flying"))))
           [ TheKeyword "FirstStrike", TheKeyword "Trample"
           , protectionFromAnyColor ] ]
       (Just (3, 4))

||| Ancestral Blade, whole -- "When this Equipment enters, create a 1/1
||| white Soldier creature token, then attach this Equipment to it. /
||| Equipped creature gets +1/+1. / Equip {1}". THE ATTACH EFFECT's first
||| witness, and the one that spends `ItToken`: the host pronoun names the
||| token the same clause minted [CR#111.1], which is the origin narrowing
||| and not a count over the whole prefix.
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

||| Disarm, whole -- "Unattach all Equipment from target creature."
||| [CR#701.3d]'s act with its "from" phrase, which is the described
||| object's own predicate rather than a second slot: what the phrase
||| does here is pick out WHICH Equipment, against every Equipment on the
||| battlefield, and `AttachedTo` is the phrase that says so.
public export
disarm : Card
disarm =
  Macros.card "Disarm" (Just [Macros.pip Blue]) [] (MkTypeLine [] [Instant])
       [ Spell (Unattach
                  (AllOf (And [ HasSubtype (artifactType "Equipment")
                              , AttachedTo (Macros.target Macros.creature) ]))) ]
       Nothing

||| Embercleave, whole -- "Flash / This spell costs {1} less to cast for
||| each attacking creature you control. / When Embercleave enters, attach
||| it to target creature you control. / Equipped creature gets +1/+1 and
||| has double strike and trample. / Equip {3}". The attach clause at its
||| commonest shape (29 of the 233 supported faces write exactly this
||| sentence), with the ATTACHED object as the pronoun and the host
||| described.
public export
embercleave : Card
embercleave =
  Macros.card "Embercleave"
       (Just [Macros.generic 4, Macros.pip Red, Macros.pip Red]) [Legendary]
       (MkTypeLine [artifactType "Equipment"] [Artifact])
       [ Macros.keyword "Flash"
       , Static (CostsToCast This
                   (CostLess (Macros.forEach
                                (And [Macros.creature, Attacking, ControlledBy You]))
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

||| Stone Haven Outfitter, whole -- "Equipped creatures you control get
||| +1/+1. / Whenever an equipped creature you control dies, draw a card."
||| THE PRENOMINAL PARTICIPLE, and it needed no row. Finding 369 recorded
||| that the corpus writes the attachment participle "predicatively after
||| the copula and never prenominally"; the second half of that is wrong
||| and is corrected here and at `IsAttached`. Both writings are
||| `IsAttached` inside a described noun, and the position is spelling:
||| "creatures you control that are equipped" (12 supported faces) and
||| "equipped creatures you control" (29 occurrences over 28 faces,
||| re-measured 2026-08-28) take the same word with the same absent
||| slots, so nothing covaries with the position.
||| What the participle is NOT here is `AttachHost`: that names the ONE
||| host of this permanent's own attachment [CR#301.5f], where these
||| lines describe every equipped creature their controller has.
public export
stoneHavenOutfitter : Card
stoneHavenOutfitter =
  Macros.card "Stone Haven Outfitter" (Just [Macros.generic 1, Macros.pip White]) []
       (MkTypeLine [creatureType "Kor", creatureType "Artificer",
                    creatureType "Ally"] [Creature])
       [ Static (Gets (AllOf (And [Macros.creatureYouControl, IsAttached Equipped]))
                      (PtUp (Lit 1)) (PtUp (Lit 1)))
       , Macros.triggered Whenever
           (Dies (Macros.a (And [Macros.creatureYouControl, IsAttached Equipped])))
           (Macros.drawsACard You) ]
       (Just (2, 2))

||| Cloud, Ex-SOLDIER's entry trigger -- "When Cloud enters, attach up to
||| one target Equipment you control to it." THE CO-ARGUMENT NARROWING's
||| witness on the family it was built for: the host pronoun is read over
||| the prefix the ATTACHED object did not mint, and the bare `It` here
||| counts the target Equipment as well as Cloud and refuses. The
||| exclusion is the act's own rule -- [CR#301.5c] says an Equipment
||| "can't equip itself" -- and not a preference among candidates.
||| A FRAGMENT: the card's second line reads "draw a card for each
||| equipped attacking creature you control", whose amount is a count
||| over a described class rather than the per-member `forEach` this
||| grammar spells at a draw.
public export
cloudExSoldierAttach : Ability
cloudExSoldierAttach =
  Macros.triggered When (Enters Macros.thisCreature Nothing)
    (Macros.attachToIt
       (TargetGroup (Macros.upTo 1)
          (And [HasSubtype (artifactType "Equipment"), ControlledBy You])))

||| Kitsune Mystic // Autumn-Tail, Kitsune Sage, a flip card [CR#710.1],
||| WHOLE -- "At the beginning of the end step, if this creature is
||| enchanted by two or more Auras, flip it." // "{1}: Attach target Aura
||| attached to a creature to another creature."
||| The reverse attachment phrase's witness. The alternative face's target
||| is described by what it is attached TO, which is the relation read
||| from the attachment's own side [CR#303.4b] -- the direction the
||| umbrella recorded as this card's last blocker -- and the ability's
||| body is [CR#701.3a]'s act with a written host.
||| `kitsuneMysticFlip` above stays as the normal face's own witness.
public export
kitsuneMystic : Card
kitsuneMystic =
  FlipCard
    (MkFace "Kitsune Mystic" (Just [Macros.generic 3, Macros.pip White]) []
            (MkTypeLine [creatureType "Fox", creatureType "Wizard"] [Creature])
            [ kitsuneMysticFlip ]
            (Macros.printedBox (Just (2, 3))))
    (MkAltFace "Autumn-Tail, Kitsune Sage" [Legendary]
               (MkTypeLine [creatureType "Fox", creatureType "Wizard"] [Creature])
               [ Macros.activated (Mana [Macros.generic 1])
                   (AttachTo
                      (Macros.target
                         (And [ HasSubtype (enchantmentType "Aura")
                              , AttachedTo (Macros.a Macros.creature) ]))
                      (Macros.a (And [Macros.creature, OtherThan This]))) ]
               (Macros.printedBox (Just (4, 5))))

||| Akiri, Fearless Voyager's two lines -- "Whenever you attack a player
||| with one or more equipped creatures, draw a card. / {W}: You may
||| unattach an Equipment from a creature you control. If you do, tap that
||| creature and it gains indestructible until end of turn."
||| [CR#701.3d]'s act at its offered form, with the "from" phrase carried
||| by the object's own `AttachedTo` description -- and the readback that
||| description leaves is what "that creature" then reads. The header
||| beside it is the prenominal participle inside an attack event.
||| A FRAGMENT by one pronoun: the printed line ends "and it gains
||| indestructible until end of turn", whose "it" names the creature the
||| TAP clause just spoke of. `ItPrior` is the reading for that, and its
||| segment is the preceding member's own delta -- which here is empty,
||| the tap clause's subject being itself a readback rather than a fresh
||| mention. A residue of the anaphora family and not of the attachment;
||| nothing in this round's rows moves it.
public export
akiriEquippedAttackers : Ability
akiriEquippedAttackers =
  Macros.triggered Whenever
    (AttacksWith You (OneDefender (Macros.a AnyPlayer))
       (CountedGroup (Macros.atLeast 1) Nothing
          (And [Macros.creatureYouControl, IsAttached Equipped])))
    (Macros.drawsACard You)

public export
akiriUnattachOffer : Ability
akiriUnattachOffer =
  Macros.activated (Mana [Macros.pip White])
    (May (Just You)
       (Unattach
          (Macros.a (And [ HasSubtype (artifactType "Equipment")
                         , AttachedTo (Macros.a Macros.creatureYouControl) ])))
       (Just (SetStatus Tapped (That (TypeW Creature))))
       Nothing)

||| Black Ward, whole -- "Enchant creature / Enchanted creature has
||| protection from black. This effect doesn't remove this Aura."
||| THE AURA CARVE-OUT, at the simplest of its four spellings. What the
||| rider suspends is a state-based action: [CR#702.16c] puts an Aura of
||| the stated quality attached to a protected permanent into its owner's
||| graveyard, which is [CR#704.5m], and this Aura is black.
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

||| Cho-Manno's Blessing, whole -- "Flash / Enchant creature / As this
||| Aura enters, choose a color. / Enchanted creature has protection from
||| the chosen color. This effect doesn't remove this Aura."
||| The carve-out over a CHOSEN quality [CR#607.2d], which is what eleven
||| of its twelve siblings write.
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

||| Tattoo Ward, whole -- "Enchant creature / Enchanted creature gets
||| +1/+1 and has protection from enchantments. This effect doesn't remove
||| this Aura. / Sacrifice this Aura: Destroy target enchantment."
||| The carve-out over a CARD-TYPE quality, which [CR#702.16a] admits
||| outright ("can be any characteristic value"), and over a coordinated
||| verb phrase rather than a bare grant -- the rider names the whole
||| statement either way.
public export
tattooWard : Card
tattooWard =
  Macros.card "Tattoo Ward" (Just [Macros.generic 2, Macros.pip White]) []
       (MkTypeLine [enchantmentType "Aura"] [Enchantment])
       [ Macros.keywordSubject "Enchant" Macros.creature
       , Static (DoesntRemove
                   (OfSubject (AttachHost Enchanted (TypeW Creature))
                      [ VPGets (PtUp (Lit 1)) (PtUp (Lit 1)) Nothing
                      , VPGains (Macros.keywordQuality "Protection"
                                   (HasType Enchantment)) Nothing ])
                   Macros.thisAura)
       , Macros.activated (Do (Macros.sacrifice You Macros.thisAura))
           (Macros.destroy (Macros.target Macros.enchantment)) ]
       Nothing

||| Pentarch Ward, whole -- the same carve-out with an entry trigger
||| beside the entry choice.
public export
pentarchWard : Card
pentarchWard =
  Macros.card "Pentarch Ward" (Just [Macros.generic 2, Macros.pip White]) []
       (MkTypeLine [enchantmentType "Aura"] [Enchantment])
       [ Macros.keywordSubject "Enchant" Macros.creature
       , Static (Macros.entersChoosing Macros.thisAura Color)
       , Macros.triggered When (Enters Macros.thisAura Nothing) (Macros.drawsACard You)
       , Static (DoesntRemove
                   (Gains (AttachHost Enchanted (TypeW Creature))
                          (Macros.keywordQuality "Protection" (OfChosen Color)))
                   Macros.thisAura) ]
       Nothing

||| Benevolent Blessing, whole -- "Flash / Enchant creature / As this Aura
||| enters, choose a color. / Enchanted creature has protection from the
||| chosen color. This effect doesn't remove Auras and Equipment you
||| control that are already attached to it."
||| The carve-out with its object DESCRIBED rather than named, which is
||| what the other three spellings do -- and the description is the
||| reverse attachment phrase, reading back the host the statement itself
||| announced. "Already" is spelling: [CR#702.16c] and [CR#702.16d] act
||| on attachments that are there when the protection applies, and the
||| word says no more.
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
                   (AllOf (And [ Or [ HasSubtype (enchantmentType "Aura")
                                    , HasSubtype (artifactType "Equipment") ]
                               , ControlledBy You
                               , AttachedTo It ]))) ]
       Nothing

||| Floating Shield, whole -- the chosen-colour carve-out plus "Sacrifice
||| this Aura: Target creature gains protection from the chosen color
||| until end of turn." The second ability reads the SAME chosen value the
||| entry choice wrote [CR#607.2d], which is what makes this card the
||| family's sixth whole one rather than a fifth twin.
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

||| Summoning Materia's second line -- "As long as this Equipment is
||| attached to a creature, you may cast creature spells from the top of
||| your library." THE ATTACHMENT'S OWN DIRECTION in its printed
||| position: the same `AttachedTo` Kitsune Mystic writes attributively,
||| after a copula this time, which is the position finding 275's test
||| calls spelling.
||| A FRAGMENT: the card's third line grants the equipped creature a
||| quoted mana ability, which the umbrella records as an unbuilt hole.
public export
summoningMateriaTopCast : Ability
summoningMateriaTopCast =
  Static (Macros.asLongAs
            (Matches Macros.thisEquipment (AttachedTo (Macros.a Macros.creature)))
            (Macros.mayCastFrom You
               (AllOf (And [Macros.spell, HasType Creature])) Macros.onTopZ))

||| Ghostfire Blade, whole -- "Equipped creature gets +2/+2. / Equip {3} /
||| This Equipment's equip ability costs {2} less to activate if it
||| targets a colorless creature."
||| THE EQUIP COST REDUCTION, which is the cost-modification axis and not
||| a variant of the equip row: [CR#702.6a] makes equip an activated
||| ability, [CR#115.9b] gives the reading that describes one by what it
||| targets, and `CostsToCast`'s ability seat was already open
||| (`AbilityCostSubject`). Re-measured 2026-08-28: 10 supported lines
||| write "equip abilities you activate … cost {N} less to activate", 6
||| of them with a target restrictor; 5 more append the general
||| activated-ability reduction to a bare equip line, which is a family of
||| 46 faces and no part of the equip row either.
||| The printed "if it targets a colorless creature" is written here as
||| the described ability's own predicate, which is the same statement:
||| the condition decides WHICH equip abilities the reduction applies to,
||| and that is what a description of the subject says.
public export
ghostfireBlade : Card
ghostfireBlade =
  Macros.card "Ghostfire Blade" (Just [Macros.generic 1]) []
       (MkTypeLine [artifactType "Equipment"] [Artifact])
       [ Static (Gets (AttachHost Equipped (TypeW Creature))
                      (PtUp (Lit 2)) (PtUp (Lit 2)))
       , Macros.keywordCosting "Equip" (Mana [Macros.generic 3])
       , Static (CostsToCast
                   (AllOf (And [ AbilityHead (KeywordClass "Equip")
                               , AbilityOf This
                               , Targets (Macros.a (And [Macros.creature, IsColorless]))
                                         SomeTarget ]))
                   (CostLess (Lit 2) Nothing)) ]
       Nothing

||| Luxior, Giada's Gift's second line -- "Equipped permanent isn't a
||| planeswalker and is a creature in addition to its other types."
||| THE NEGATED TYPE SETTING, and the line that spends
||| `attachHeadOk Equipped PermanentW`. The two halves are why the card
||| writes "permanent" and not "creature": [CR#301.5a] names the equipped
||| CREATURE and [CR#702.6e]'s "equip planeswalker" attaches an Equipment
||| to a planeswalker "as though that planeswalker were a creature", so
||| the host word has to reach further than [CR#301.5] otherwise lets it.
||| Written as two coordinated statements with the second reading the
||| host back, on Darksteel Mutation's ground: `SubjectVPs` spells a
||| shared subject only for a P/T shift and a grant, and neither half
||| here is either.
||| A FRAGMENT: the card's "Equip planeswalker {1}" wants the compound
||| equip parameter, which this round did not mint.
public export
luxiorTypeSetting : Ability
luxiorTypeSetting =
  Static (AndAlso
            [ LosesType (AttachHost Equipped PermanentW) Planeswalker
            , BecomesAlso It (MkToken Nothing []
                                      (MkTypeLine [] [Creature]) [] Nothing) ])

||| Vesuvan Shapeshifter's copy clause -- "until this creature is turned
||| face down, it becomes a copy of that creature, except it has 'At the
||| beginning of your upkeep, you may turn this creature face down.'"
||| THE STATUS EVENT'S SECOND READER. This is the only supported line
||| anywhere that names a face-down transition, and it names it as a
||| DURATION ENDPOINT. `statusEventOk` answers it -- the transition
||| exists -- while `statusHeaderOk` keeps the trigger header at its
||| measured zero (`badTurnedFaceDownHeader`). Two tables, one cell of
||| disagreement, and neither reader widened.
||| `spanEventOk`'s `StatusChange` cell is answered by the same line: it
||| was admitted by the catch-all and is now attested, at one line, by
||| this one.
||| A FRAGMENT: the sentence before it is an as-clause over two
||| alternative events ("As this creature enters or is turned face up")
||| whose body is an optional choice, and morph's own line waits on the
||| keyword row.
public export
vesuvanShapeshifterCopySpan :
  Effect [MkBinding AD Object OneOf
            (ObjectP (Just Creature) (Just Battlefield) Nothing Nothing Nothing)]
vesuvanShapeshifterCopySpan =
  Continuously
    (BecomesCopy Macros.thisCreature (That (TypeW Creature))
       [ExceptAbility
          (Macros.triggered At (BeginningOf Upkeep (ByWord Yours))
             (Macros.may You (SetStatus FaceDown Macros.thisCreature)))])
    (Just (UntilEvent (StatusEvent Macros.thisCreature FaceDown)))

||| Academy Journeymage, whole -- "This spell costs {1} less to cast if
||| you control a Wizard. / When this creature enters, return target
||| creature an opponent controls to its owner's hand."
||| THE CONDITIONAL COST STATEMENT at the "if" marking, `CondMarking`'s
||| third word. It is the same construction the closed pair was, settled
||| on [CR#601.2f] and [CR#611.3a] at `CondMarking`'s own declaration:
||| the lock-in [CR#601.2f] describes belongs to the TOTAL COST, and
||| [CR#611.3a] denies this static's continuous effect any of its own, so
||| the condition is read once at the determination step whichever word
||| marks it. 145 supported lines write it over a cost modification.
public export
academyJourneymage : Card
academyJourneymage =
  Macros.card "Academy Journeymage" (Just [Macros.generic 4, Macros.pip Blue]) []
       (MkTypeLine [creatureType "Human", creatureType "Wizard"] [Creature])
       [ Static (Macros.onlyIfSo (CostsToCast This (CostLess (Lit 1) Nothing))
                   (Exists (And [HasSubtype (creatureType "Wizard"), ControlledBy You])))
       , Macros.triggered When (Enters Macros.thisCreature Nothing)
           (Macros.move (Macros.target (And [Macros.creature,
                                             ControlledBy Macros.anOpponent]))
                        Macros.handZ) ]
       (Just (3, 2))

||| Alabaster Leech, whole -- "White spells you cast cost {W} more to
||| cast." THE COLOURED PAYLOAD, `CostShiftRun` beside the `Amount` arms
||| rather than in place of them: Ghalta and Cavern-Hoard Dragon still
||| write a letter at the same slot. Derelor and Jade Leech print the
||| same line at {B} and {G}.
public export
alabasterLeech : Card
alabasterLeech =
  Macros.card "Alabaster Leech" (Just [Macros.pip White]) []
       (MkTypeLine [creatureType "Leech"] [Creature])
       [ Static (CostsToCast (AllOf (And [Macros.spell, ColorIs White, CastBy You]))
                             (CostShiftRun [Macros.pip White] True)) ]
       (Just (1, 3))

||| Cavern-Hoard Dragon's cost rider, written -- "This spell costs {X}
||| less to cast, where X is the greatest number of artifacts an opponent
||| controls." Ghalta's telescope over the narrowed domain the amount
||| beside it already spelled.
||| A FRAGMENT: the card's combat-damage trigger creates a Treasure for
||| each artifact the damaged player controls, which is the for-each over
||| a damaged player's permanents.
public export
cavernHoardDragonRider : Ability
cavernHoardDragonRider =
  Static (AndAlso [ CostsToCast This (CostLess (LetterVal X) Nothing)
                  , Define X (AggregateOver MaxOf Opponent
                                (CountOf (And [Macros.artifact, ControlledBy They]))) ])

||| Propaganda, whole -- "Creatures can't attack you unless their
||| controller pays {2} for each creature they control that's attacking
||| you." THE GATE READING ITS OWN SUBJECT AND ITS DERIVED PAYER: the
||| carrier types a `Compulsion` at `selfSubjIntro n` and `GatedBy` adds
||| the payer [CR#508.1h] derives, so "they control" is a read rather
||| than an unwritable pronoun, and `AttackerOf` spells "that's
||| attacking you". Ghostly Prison prints the same sentence.
public export
propaganda : Card
propaganda =
  Macros.card "Propaganda" (Just [Macros.generic 2, Macros.pip Blue]) []
       (MkTypeLine [] [Enchantment])
       [ Static (Macros.deontic (AllOf Macros.creature)
                   (GatedBy (ScaledMana GenericUnit (Times 2
                      (CountOf (And [Macros.creature, ControlledBy They,
                                     AttackerOf You])))))
                   ["Attack"] Agent (DefendingPlayer You)) ]
       Nothing

||| Ghostly Prison, whole -- Propaganda's sentence at the other colour.
||| Windborn Muse and Koskun Falls print it too; Elephant Grass narrows
||| the subject to nonblack creatures and Onakke Oathkeeper reads the
||| defender at a planeswalker, which is the kind index `AttackerOf`
||| carries.
public export
ghostlyPrisonWhole : Card
ghostlyPrisonWhole =
  Macros.card "Ghostly Prison" (Just [Macros.generic 2, Macros.pip White]) []
       (MkTypeLine [] [Enchantment])
       [ Static (Macros.deontic (AllOf Macros.creature)
                   (GatedBy (ScaledMana GenericUnit (Times 2
                      (CountOf (And [Macros.creature, ControlledBy They,
                                     AttackerOf You])))))
                   ["Attack"] Agent (DefendingPlayer You)) ]
       Nothing

||| Archangel of Tithes' block line -- "As long as this creature is
||| attacking, creatures can't block unless their controller pays {1} for
||| each of those creatures." THE ANAPHORIC COUNT: "those creatures" is
||| the subject's own plural mention and `GroupSize` is the read of it,
||| which the gate's cost could not reach while the cost sat before the
||| subject. 9 supported lines write the phrase; the other eight name
||| "you or planeswalkers you control" as the defender, which is a
||| DISJOINED patient this line does not need and no noun in the grammar
||| yet spells.
public export
archangelOfTithesBlockToll : Ability
archangelOfTithesBlockToll =
  Static (Macros.asLongAs (Matches Macros.thisCreature Attacking)
            (Macros.deontic (AllOf Macros.creature)
               (GatedBy (ScaledMana GenericUnit (Times 1 GroupSize)))
               ["Block"] Agent NoDeonticPatient))

||| Myr Prototype, whole -- "At the beginning of your upkeep, put a
||| +1/+1 counter on this creature. / This creature can't attack or block
||| unless you pay {1} for each +1/+1 counter on it."
||| THE COORDINATED DEED under a gate, which the carrier's deed LIST
||| already was, plus the gate cost reading the subject's own counters.
||| Cowed by Wisdom and Whipgrass Entangler are the family's other two.
||| Phyrexian Marauder writes the same counter-scaled toll at one deed.
public export
myrPrototype : Card
myrPrototype =
  Macros.card "Myr Prototype" (Just [Macros.generic 5]) []
       (MkTypeLine [creatureType "Myr"] [Artifact, Creature])
       [ Macros.triggered At (BeginningOf Upkeep (ByWord Yours))
           (PutCounters (Lit 1) (PrintedKind Macros.plusOnePlusOne) Macros.thisCreature)
       , Static (Macros.deontic Macros.thisCreature
                   (GatedBy (ScaledMana GenericUnit (Times 1
                      (CountersOn Macros.plusOnePlusOne It))))
                   ["Attack", "Block"] Agent NoDeonticPatient) ]
       (Just (3, 3))

||| Heat Wave, whole -- "Cumulative upkeep {R} / Blue creatures can't
||| block creatures you control. / Nonblue creatures can't block
||| creatures you control unless their controller pays 1 life for each
||| blocking creature they control."
||| THE PAYER NOUN INSIDE AN ACTION COST: a life payment is a clause that
||| writes its own payer, and the payer the gate derives [CR#509.1d] is
||| the noun it writes. Sivitri, Dragon Master's "pays 2 life for each of
||| those creatures" is the family's other line.
public export
heatWave : Card
heatWave =
  Macros.card "Heat Wave" (Just [Macros.generic 2, Macros.pip Red]) []
       (MkTypeLine [] [Enchantment])
       [ Macros.keywordCosting "CumulativeUpkeep" (Mana [Macros.pip Red])
       , Static (Macros.deontic (AllOf (And [Macros.creature, ColorIs Blue]))
                   Forbid ["Block"] Agent
                   (DeonticCounterpart (AllOf Macros.creatureYouControl)))
       , Static (Macros.deontic (AllOf (And [Macros.creature, Not (ColorIs Blue)]))
                   (GatedBy (Do (Macros.losesLife They
                                   (Times 1 (CountOf (And [Macros.creature, Blocking,
                                                           ControlledBy They]))))))
                   ["Block"] Agent
                   (DeonticCounterpart (AllOf Macros.creatureYouControl))) ]
       Nothing

||| Braid of Fire, whole -- "Cumulative upkeep-Add {R}."
||| The first of the four cumulative upkeeps whose cost ACTION the
||| keyword's cell was recorded as disagreeing with. It needed nothing
||| minted: [CR#118.1] makes a cost an action a player carries out and
||| `costActionOk` already admits the mana ability, the draw, the token
||| creation and the life gain.
public export
braidOfFire : Card
braidOfFire =
  Macros.card "Braid of Fire" (Just [Macros.generic 1, Macros.pip Red]) []
       (MkTypeLine [] [Enchantment])
       [ Macros.keywordCosting "CumulativeUpkeep"
           (Do (AddMana You (Lit 1) (Runs [[OfColor Red]]) [])) ]
       Nothing

||| Psychic Vortex's upkeep line -- "Cumulative upkeep-Draw a card."
||| A FRAGMENT: the card's end-step trigger discards a whole hand, which
||| no clause spells.
public export
psychicVortexUpkeep : Ability
psychicVortexUpkeep =
  Macros.keywordCosting "CumulativeUpkeep" (Do Macros.drawACard)

||| Varchild's War-Riders' upkeep line -- "Cumulative upkeep-Have an
||| opponent create a 1/1 red Survivor creature token."
||| -- spelling: the causative "have [who] [verb]" is this row's own
||| agent slot, as Grismold's "each player creates" is.
||| A FRAGMENT: the card's second line is "Trample; rampage 1", and
||| rampage is not a catalog row.
public export
varchildsWarRidersUpkeep : Ability
varchildsWarRidersUpkeep =
  Macros.keywordCosting "CumulativeUpkeep"
    (Do (Create Macros.anOpponent (Lit 1)
           (TokenWritten (Macros.creatureTok 1 1 [Red] [creatureType "Survivor"])) []))

||| Wall of Shards, whole -- "Defender, flying / Cumulative upkeep-An
||| opponent gains 1 life."
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

||| Earthen Goo, whole -- "Trample / Cumulative upkeep {R} or {G} / This
||| creature gets +1/+1 for each age counter on it."
||| THE MANA-OR-MANA COST, `EitherCost` on `Cost` and not a widened
||| `ManaCost`. Arctic Nishoba ({G} or {W}), Jotun Owl Keeper ({W} or
||| {U}) and Krovikan Whispers ({U} or {B}) are the other three, and
||| [CR#702.24a] is what says the choice is answered once per age
||| counter.
public export
earthenGoo : Card
earthenGoo =
  Macros.card "Earthen Goo" (Just [Macros.generic 2, Macros.pip Red]) []
       (MkTypeLine [creatureType "Ooze"] [Creature])
       [ Macros.keyword "Trample"
       , Macros.keywordCosting "CumulativeUpkeep"
           (EitherCost (Mana [Macros.pip Red]) (Mana [Macros.pip Green]))
       , Static (Gets Macros.thisCreature
                   (PtUp (Times 1 (CountersOn Age It)))
                   (PtUp (Times 1 (CountersOn Age It)))) ]
       (Just (2, 2))

||| Chamber Sentry's damage ability -- "{X}, {T}, Remove X +1/+1 counters
||| from this creature: It deals X damage to any target."
||| THE [CR#107.3k] BOUNDARY, benched: the card's printed cost is {X} and
||| this ability's is another, so the ability's telescope drops the
||| object's letter and its own cost opens the one the body reads.
||| Defenders of Humanity is the other supported card writing both at
||| once; Riptide Replicator writes an X in a body whose cost has none
||| and defines it with a where-clause of its own.
||| A FRAGMENT: the card's entry rider counts the colours of mana spent
||| to cast it, which no phrase reads.
public export
chamberSentryDamage : Ability
chamberSentryDamage =
  Macros.activated (Compound [Mana [Variable], TapSymbol,
                       Do (RemoveCounters (ExactlyOf (LetterVal X))
                             (Just Macros.plusOnePlusOne) Macros.thisCreature)])
                   (DealDamage This (LetterVal X) (Macros.target Macros.anyTarget))

||| Awesome Presence, whole -- "Enchant creature / Enchanted creature
||| can't be blocked unless defending player pays {3} for each creature
||| they control that's blocking it."
||| THE AURA CARRIER of the gate family, and the printed "defending
||| player" read at the BLOCK role: the payer [CR#509.1d] derives is the
||| one the cost writes, and "that's blocking it" is `BlockerOf` over
||| the enchanted creature the statement's own subject named. Brainwash,
||| Oppressive Rays and Cowed by Wisdom are the family's other three.
public export
awesomePresence : Card
awesomePresence =
  Macros.card "Awesome Presence" (Just [Macros.pip Blue]) []
       (MkTypeLine [enchantmentType "Aura"] [Enchantment])
       [ Macros.keywordSubject "Enchant" Macros.creature
       , Static (Macros.deontic (AttachHost Enchanted (TypeW Creature))
                   (GatedBy (ScaledMana GenericUnit (Times 3
                      (CountOf (And [Macros.creature, ControlledBy They,
                                     BlockerOf It])))))
                   ["Block"] Patient NoDeonticPatient) ]
       Nothing

||| Oppressive Rays, whole -- "Enchant creature / Enchanted creature
||| can't attack or block unless its controller pays {3}. / Activated
||| abilities of enchanted creature cost {3} more to activate."
||| The coordinated deed under a flat gate beside the activation
||| variant, on one card.
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
                   (AllOf (And [ AbilityHead AnyActivated
                               , AbilityOf (AttachHost Enchanted (TypeW Creature)) ]))
                   (CostMore (Lit 3))) ]
       Nothing

-- ==== The mana region: the mention, the riders, the productions ====

||| Delighted Halfling's second ability -- "{T}: Add one mana of any
||| color. Spend this mana only to cast a legendary spell, and that spell
||| can't be countered." The ONE-SENTENCE form, where [CR#106.6]'s
||| restriction and its additional effect share a single mention of the
||| spell. Cavern of Souls is its twin and writes the chosen creature
||| type where this writes a supertype.
public export
delightedHalflingMana : Ability
delightedHalflingMana =
  Macros.activated TapSymbol
    (AddMana You (Lit 1) (AnyColor SameColor)
      [ OnSpent AffectsIt True
                (Macros.a (And [HasSupertype Legendary, Macros.spell]))
                (Continuously (Macros.objectCant "Counter" (That SpellW)) Nothing) ])

||| Boseiju, Who Shelters All's mana ability -- "{T}, Pay 2 life: Add
||| {C}. If that mana is spent on an instant or sorcery spell, that spell
||| can't be countered." [CR#106.6]'s ADDITIONAL EFFECT in the
||| CONDITIONAL spelling, against Delighted Halfling's restrictive one.
||| The paid-for spell is bound by the rider's own mention and read back
||| as "that spell".
|||
||| It is the cell whose body speaks of the SPELL. The other 9 of the 11
||| speak of the permanent the spell becomes -- "it gains haste until end
||| of turn" (Arena of Glory, Generator Servant, Hall of the Bandit Lord),
||| "that creature enters with an additional +1/+1 counter on it" (Animal
||| Attendant, Biophagus, Guildmages' Forum) -- and that read is this
||| round's remaining gap, not this row's: the mention is a spell on the
||| stack [CR#601.2a] and the grant lands on what it resolves into, a
||| transition no noun here spells. `OnSpent` carries all 11 either way;
||| what the 9 wait on is a spell-to-permanent read.
public export
boseijuMana : Ability
boseijuMana =
  Macros.activated (Compound [TapSymbol, Do (Macros.losesLife You (Lit 2))])
    (AddMana You (Lit 1) (Runs [[Colorless]])
      [ OnSpent AffectsIt False
                (Macros.a (And [Macros.instantOrSorcery, Macros.spell]))
                (Continuously (Macros.objectCant "Counter" (That SpellW))
                              Nothing) ])

||| Pyromancer's Goggles' mana ability -- "{T}: Add {R}. When that mana
||| is spent to cast a red instant or sorcery spell, copy that spell and
||| you may choose new targets for the copy." [CR#106.6]'s third rider,
||| the DELAYED TRIGGER [CR#603.7a]. One of three; Path of Ancestry and
||| Primal Amulet are the others.
public export
pyromancersGogglesMana : Ability
pyromancersGogglesMana =
  Macros.activated TapSymbol
    (AddMana You (Lit 1) (Runs [[OfColor Red]])
      [ OnSpent TriggersThen False
                (Macros.a (And [ColorIs Red, Macros.instantOrSorcery, Macros.spell]))
                (CopyStack You (That SpellW) (Lit 1) []) ])

||| Thran Turbine's upkeep trigger -- "At the beginning of your upkeep,
||| you may add {C}{C}. This mana can't be spent to cast spells." The
||| restriction stated by what it EXCLUDES, 1 of 9 real lines. The other
||| 31 that a naive sweep returns are the Powerstone token's reminder
||| text on cards that make one.
public export
thranTurbineMana : Effect []
thranTurbineMana =
  AddMana You (Lit 1) (Runs [[Colorless, Colorless]])
          [SpendNotOn [ToCast Macros.spell]]

||| Su-Chi Cave Guard's death trigger -- "When this creature dies, add
||| eight {C}. Until end of turn, you don't lose this mana as steps and
||| phases end." The MENTION at work: the add leaves a `ManaAdded`
||| outcome and the next sentence reads it as "this mana". 25 supported
||| lines write this shape.
public export
suChiCaveGuardDies : Ability
suChiCaveGuardDies =
  Macros.triggered When (Dies Macros.thisCreature)
    (Sequentially
       [ AddMana You (Lit 1)
                 (Runs [[Colorless, Colorless, Colorless, Colorless,
                         Colorless, Colorless, Colorless, Colorless]]) []
       , Continuously (KeepsUnspentMana You ThisMana)
                      (Just Macros.untilEndOfTurn) ])

||| Omnath, Locus of Mana's first line -- "You don't lose unspent green
||| mana as steps and phases end." The persistence sentence with no add
||| anywhere on the card and no span written, which is why the family is
||| not a rider on the production: 6 of the 7 description lines are
||| static abilities of permanents that add no mana at all.
public export
omnathLocusOfManaPersistence : StaticEffect []
omnathLocusOfManaPersistence =
  KeepsUnspentMana You (UnspentMana (Just (OfColor Green)))

||| Upwelling, whole card -- "Players don't lose unspent mana as steps
||| and phases end." The same row with an untyped read and a plural
||| subject.
public export
upwelling : Card
upwelling =
  Macros.card "Upwelling" (Just [Macros.generic 4, Macros.pip Green]) []
       (MkTypeLine [] [Enchantment])
       [ Static (KeepsUnspentMana (Each AnyPlayer) (UnspentMana Nothing)) ]
       Nothing

||| Mana Flare, whole card -- "Whenever a player taps a land for mana,
||| that player adds one mana of any type that land produced." The
||| tapped-for-mana header in the ACTIVE voice, and `ProducedByEvent`
||| under it. All 17 sentences that read a production sit inside one of
||| these headers.
public export
manaFlare : Card
manaFlare =
  Macros.card "Mana Flare" (Just [Macros.generic 2, Macros.pip Red]) []
       (MkTypeLine [] [Enchantment])
       [ Macros.triggered Whenever
           (VerbedEvent (Just (Macros.a AnyPlayer)) "Tap"
                        (Just (Macros.a Macros.land)) True)
           (AddMana (That PlayerW) (Lit 1)
                    (ProducedByEvent (That (TypeW Land))) []) ]
       Nothing

||| Shimmerwilds Growth, whole card -- "Enchant land / As this Aura
||| enters, choose a color. / Enchanted land is the chosen color. /
||| Whenever enchanted land is tapped for mana, its controller adds an
||| additional one mana of the chosen color." The tapped-for-mana header
||| in the PASSIVE voice, against Mana Flare's active -- one row and one
||| header, with `VerbedVoice` making the voice a spelling. It is also
||| one of the four OTHER-OBJECT readers of the chosen colour: the adder
||| is the host's controller, not the ability's own "you".
public export
shimmerwildsGrowth : Card
shimmerwildsGrowth =
  Macros.card "Shimmerwilds Growth" (Just [Macros.pip Green]) []
       (MkTypeLine [enchantmentType "Aura"] [Enchantment])
       [ Macros.keywordSubject "Enchant" Macros.land
       , Static (Macros.entersChoosing Macros.thisAura Color)
       , Static (SetsChosenQuality (AttachHost Enchanted (TypeW Land))
                                   (OfChosen Color))
       , Macros.triggered Whenever
           (VerbedEvent Nothing "Tap"
                        (Just (AttachHost Enchanted (TypeW Land))) True)
           (AddMana (ControllerOf (That (TypeW Land))) (Lit 1)
                    (OfChosenColor Nothing) []) ]
       Nothing

||| Chrome Mox's mana ability -- "{T}: Add one mana of any of the exiled
||| card's colors." The colour SET read off a mentioned object. Pit of
||| Offerings writes it plural and Omnath, Locus of All writes it as a
||| combination over the same set.
public export
chromeMoxMana : Ability
chromeMoxMana =
  Macros.activated TapSymbol
    (AddMana You (Lit 1)
             (AmongColorsOf (Macros.a (ExiledWith Macros.thisArtifact))) [])

||| Fellwar Stone, whole card -- "{T}: Add one mana of any color that a
||| land an opponent controls could produce." [CR#106.7]'s HYPOTHETICAL
||| read, measured apart from `ProducedByEvent`: 18 lines over 18 cards,
||| 15 of them add payloads. Exotic Orchard writes the same sentence and
||| Reflecting Pool writes it of a land its own controller has.
public export
fellwarStone : Card
fellwarStone =
  Macros.card "Fellwar Stone" (Just [Macros.generic 2]) []
       (MkTypeLine [] [Artifact])
       [ Macros.activated TapSymbol
           (AddMana You (Lit 1)
                    (CouldProduce (Macros.a (And [Macros.land,
                                                  ControlledBy Macros.anOpponent])))
                    []) ]
       Nothing

||| Ice Cauldron's second ability -- "{T}, Remove a charge counter from
||| this artifact: Add this artifact's last noted type and amount of
||| mana. Spend this mana only to cast the last card exiled with this
||| artifact." The NOTE read, anchored to its holder and ungated, on
||| `GreatestStoredMatch`'s model. Its first ability -- which does the
||| noting -- is the remaining gap: no effect here records state on a
||| permanent, and [CR#607.2e] is the link that would make the pair one
||| statement once one exists.
public export
iceCauldronNotedMana : Ability
iceCauldronNotedMana =
  Macros.activated
    (Compound [TapSymbol,
               Do (RemoveCounters (Macros.exactly 1) (Just Charge)
                                  Macros.thisArtifact)])
    (AddMana You (Lit 1) (LastNotedMana Macros.thisArtifact)
             [SpendOnly [ToCast (ExiledWith Macros.thisArtifact)]])

||| Firemind Vessel's mana ability -- "{T}: Add two mana of different
||| colors." The third value on the colour-freedom axis: each unit free
||| of the rest, as `EachColor` has it, but no repeat. 4 supported lines
||| (Component Pouch, Guild Globe, Interplanar Beacon are the others).
public export
firemindVesselMana : Ability
firemindVesselMana =
  Macros.activated TapSymbol
    (AddMana You (Lit 2) (AnyColor DistinctColors) [])

||| Goblin Clearcutter's mana ability -- "{T}, Sacrifice a Forest: Add
||| three mana in any combination of {R} and/or {G}." The combination
||| over a WRITTEN colour set, 12 supported sentences, against the 34
||| that leave the set at all five.
public export
goblinClearcutterMana : Ability
goblinClearcutterMana =
  Macros.activated
    (Compound [TapSymbol,
               Do (Macros.sacrifice You (Macros.a (HasSubtype (landType "Forest"))))])
    (AddMana You (Lit 3) (AmongWritten [Red, Green]) [])

||| Vizier of the Menagerie's third line -- "You can spend mana of any
||| type to cast creature spells." [CR#118.14]'s permission with the
||| matcher in SUBJECT position, which is the third way the corpus writes
||| one sentence: the rule glosses all of them as spending mana as though
||| it were mana of any type. Its first two lines already wrote.
public export
vizierOfTheMenagerieSpend : StaticEffect []
vizierOfTheMenagerieSpend =
  Macros.maySpendAsThough You Nothing MatchAnyType
    (Just (ToCast (And [Macros.creature, Macros.spell])))

||| Vexing Bauble's second ability -- "Whenever a player casts a spell,
||| if no mana was spent to cast it, counter that spell." The
||| MANA-SPENT-TO-CAST test, negated by the ordinary `NotCond`. Boromir,
||| Lavinia and Roiling Vortex write the same intervening condition;
||| Nix writes it as a trailing one.
public export
vexingBaubleTrigger : Ability
vexingBaubleTrigger =
  Macros.triggeredIf Whenever
    (Casts (Macros.a AnyPlayer) (Macros.a Macros.spell) Nothing)
    (NotCond (ManaSpentToCast It Nothing))
    (CounterSpell (That SpellW))

||| Void Mirror, whole card -- "Whenever a player casts a spell, if no
||| colored mana was spent to cast it, counter that spell." The one line
||| that narrows which mana counts, at [CR#106.1a]'s five colours against
||| [CR#106.1b]'s six types.
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


-- ---------------------------------------------------------------------------
-- The transform verb, the transformed arrival, the meld pair and fuse.
-- ---------------------------------------------------------------------------

||| Arlinn Kord // Arlinn, Embraced by the Moon, whole -- the transform
||| verb's named carrier, and a nonmodal double-faced card [CR#712.2]
||| whose back writes no mana cost [CR#202.3a] and no loyalty number
||| [CR#209.1,712.8a]. `planeswalkerBackWithoutLoyaltyOk` probed that box
||| law on this very face; the card it was standing in for is now here.
||| Both faces write the verb, at the [0] and the [-1], which is what a
||| nonmodal double-faced card's abilities are for [CR#712.2].
public export
arlinnKord : Card
arlinnKord =
  Transforming
    (MkFace "Arlinn Kord" (Just [Macros.generic 2, Macros.pip Red, Macros.pip Green])
            [Legendary] (MkTypeLine [planeswalkerType "Arlinn"] [Planeswalker])
            [ Macros.activated (LoyaltySymbol (LoyaltyUp 1))
                (Continuously
                   (AndAlso [ Gets (TargetGroup (Macros.upTo 1) Macros.creature)
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
    (MkAltFace "Arlinn, Embraced by the Moon" [Legendary]
               (MkTypeLine [planeswalkerType "Arlinn"] [Planeswalker])
               [ Macros.activated (LoyaltySymbol (LoyaltyUp 1))
                   (Continuously
                      (AndAlso [ Gets (AllOf Macros.creatureYouControl)
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
                          [ Gains (AllOf Macros.creatureYouControl) (Macros.keyword "Haste")
                          , Gains Them (Macros.activated TapSymbol
                              (DealDamageOwn Macros.thisCreature Power (Macros.target Macros.anyTarget))) ]) ]) ]
               Nothing)

||| Neglected Heirloom // Ashmouth Blade, whole -- the transform verb and
||| the TRIGGER on the act in one card. "When equipped creature
||| transforms, transform this Equipment" is the only supported line that
||| writes the bare act as a header ([CR#701.27e] names the family; 39
||| supported faces write a trigger on transforming and 37 of them narrow
||| it with "into [name]", which this one does not). The event is
||| `VerbedEvent` under the `Transform` label, in the voice that names no
||| actor -- [CR#701.27a] turns the permanent over and the permanent is
||| what the header announces.
public export
neglectedHeirloom : Card
neglectedHeirloom =
  Transforming
    (MkFace "Neglected Heirloom" (Just [Macros.generic 1]) []
            (MkTypeLine [artifactType "Equipment"] [Artifact])
            [ Static (Gets (AttachHost Equipped (TypeW Creature))
                           (PtUp (Lit 1)) (PtUp (Lit 1)))
            , Macros.triggered When
                (VerbedEvent Nothing "Transform"
                             (Just (AttachHost Equipped (TypeW Creature))) False)
                (Macros.transform Macros.thisEquipment)
            , Macros.keywordCosting "Equip" (Mana [Macros.generic 1]) ]
            Nothing)
    (MkAltFace "Ashmouth Blade" []
               (MkTypeLine [artifactType "Equipment"] [Artifact])
               [ Static (AndAlso [ Gets (AttachHost Equipped (TypeW Creature))
                                        (PtUp (Lit 3)) (PtUp (Lit 3))
                                 , Gains It (Macros.keyword "FirstStrike") ])
               , Macros.keywordCosting "Equip" (Mana [Macros.generic 3]) ]
               Nothing)

||| Harvest Hand // Scrounged Scythe, whole -- the TRANSFORMED ARRIVAL,
||| which is a different thing from the verb: [CR#712.14a] has the card
||| enter with its back face up, and nothing is turned over. The rider
||| carries the controller override the printed line writes beside it.
public export
harvestHand : Card
harvestHand =
  Transforming
    (MkFace "Harvest Hand" (Just [Macros.generic 3]) []
            (MkTypeLine [creatureType "Scarecrow"] [Artifact, Creature])
            [ Macros.triggered When (Dies Macros.thisCreature)
                (Macros.returnToBattlefieldTransformed It (Just You)) ]
            (Macros.printedBox (Just (2, 2))))
    (MkAltFace "Scrounged Scythe" []
               (MkTypeLine [artifactType "Equipment"] [Artifact])
               [ Static (Gets (AttachHost Equipped (TypeW Creature))
                              (PtUp (Lit 1)) (PtUp (Lit 1)))
               , Static (Macros.asLongAs
                           (Matches (AttachHost Equipped (TypeW Creature))
                                    (HasSubtype (creatureType "Human")))
                           (Gains It (Macros.keyword "Menace")))
               , Macros.keywordCosting "Equip" (Mana [Macros.generic 2]) ]
               Nothing)

-- --- The meld pair, and the identity lint the ruling asks for --------------

||| Chittering Host as the MIDNIGHT SCAVENGERS card carries it: the
||| combined back face of the Graf Rats / Midnight Scavengers meld pair
||| [CR#712.4]. Under the 2026-08-27 ruling the reverse face is
||| DUPLICATED on each card of the pair rather than shared by reference,
||| so this is one of two copies and `chitteringHostOnGrafRats` is the
||| other. See `Card`'s docstring for what the duplication throws away.
public export
chitteringHostOnScavengers : AltFace
chitteringHostOnScavengers =
  MkAltFace "Chittering Host" []
            (MkTypeLine [creatureType "Eldrazi", creatureType "Horror"] [Creature])
            [ Macros.keyword "Haste"
            , Macros.keyword "Menace"
            , Macros.triggered When (Enters Macros.thisCreature Nothing)
                (Continuously
                   (AndAlso [ Gets (AllOf (Macros.otherCreatureYouControl Macros.thisCreature))
                                   (PtUp (Lit 1)) (PtUp (Lit 0))
                            , Gains Them (Macros.keyword "Menace") ])
                   (Just Macros.untilEndOfTurn)) ]
            (Macros.printedBox (Just (5, 6)))

||| Chittering Host as the GRAF RATS card carries it -- the second copy,
||| written out rather than aliased so that the lint below has two terms
||| to compare. Graf Rats' own card is not benched: its whole printed
||| text is the meld trigger, whose "you both own and control this
||| creature and a creature named Midnight Scavengers" wants a single
||| clause over two named objects and a plural "them" reading them both
||| back, and `Them` admits one `ManyOf` mention where that condition
||| leaves two `OneOf`s. That is the blocker all 7 supported melders
||| share, and it is not the verb.
public export
chitteringHostOnGrafRats : AltFace
chitteringHostOnGrafRats =
  MkAltFace "Chittering Host" []
            (MkTypeLine [creatureType "Eldrazi", creatureType "Horror"] [Creature])
            [ Macros.keyword "Haste"
            , Macros.keyword "Menace"
            , Macros.triggered When (Enters Macros.thisCreature Nothing)
                (Continuously
                   (AndAlso [ Gets (AllOf (Macros.otherCreatureYouControl Macros.thisCreature))
                                   (PtUp (Lit 1)) (PtUp (Lit 0))
                            , Gains Them (Macros.keyword "Menace") ])
                   (Just Macros.untilEndOfTurn)) ]
            (Macros.printedBox (Just (5, 6)))

||| THE MELD IDENTITY LINT, and the whole of what holds the 2026-08-27
||| ruling's acknowledged debt in place. [CR#712.4b] makes the two back
||| faces of a meld pair ONE face, used together to determine the
||| characteristics of one permanent; the duplication writes it twice and
||| states no relation between the copies. This law is that relation,
||| kept by the typechecker: the two copies reduce to the same `AltFace`
||| or the build fails. Cheap on purpose -- the ruling names the lint as
||| the cheap option beside a real cross-card reference, and this costs
||| one `Refl`.
public export
meldBackFacesAgree : Cards.chitteringHostOnScavengers = Cards.chitteringHostOnGrafRats
meldBackFacesAgree = Refl

||| Midnight Scavengers // Chittering Host, whole -- the benchable half of
||| a meld pair, and the first meld card in this bench. `Transforming` is
||| the shape by the ruling and not by [CR#712.2]: a meld card is no
||| nonmodal double-faced card and [CR#712.4c] refuses to transform it.
||| The front's own text says nothing about melding -- "(Melds with Graf
||| Rats.)" is reminder text -- so this face writes exactly what the
||| grammar already had, and the pair's meld ability sits on the other
||| card.
public export
midnightScavengers : Card
midnightScavengers =
  Transforming
    (MkFace "Midnight Scavengers" (Just [Macros.generic 4, Macros.pip Black]) []
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

||| The meld EFFECT vocabulary's own witness, written at the effect
||| rather than at a card: "exile them, then meld them into Chittering
||| Host", the second half of every one of the 7 supported meld lines.
||| [CR#701.42a]'s act is the meld alone -- the exile is the instructing
||| clause's own step, which is why the two are `Sequentially` and not one
||| row -- and the meld reads the exiled pair back through the stamp the
||| exile left. The clause that would introduce that pair is the
||| ownership condition named above; this term supplies it as a plain
||| plural description so that the verb's own shape is on the bench while
||| the condition is not.
public export
meldThemInto : Effect []
meldThemInto =
  Sequentially
    [ Macros.exile (AllOf (And [Macros.creature, ControlledBy You]))
    , Macros.meldInto (Macros.themVerbed "Exile") "Chittering Host" ]

||| Profit // Loss, whole -- the FUSE witness, and what the ticket's
||| earlier round avoided by choosing Wax // Wane over Wear // Tear.
||| [CR#702.102a] makes fuse a static ability of the split CARD that
||| applies in its owner's hand, so the word is printed once per half and
||| written here as a keyword line on each: 17 supported cards carry it,
||| all 34 halves instants or sorceries (measured 2026-08-28).
||| What the row does not buy is the fused spell [CR#702.102b,702.102d];
||| see `keywordFacts`.
public export
profitLoss : Card
profitLoss =
  SplitCard
    (MkFace "Profit" (Just [Macros.generic 1, Macros.pip White]) []
            (MkTypeLine [] [Instant])
            [ Spell (Macros.gets (AllOf Macros.creatureYouControl)
                                 (PtUp (Lit 1)) (PtUp (Lit 1))
                                 (Just Macros.untilEndOfTurn))
            , Macros.keyword "Fuse" ]
            Nothing)
    (MkFace "Loss" (Just [Macros.generic 2, Macros.pip Black]) []
            (MkTypeLine [] [Instant])
            [ Spell (Macros.gets
                       (AllOf Macros.creatureYourOpponentsControl)
                       (PtDown (Lit 1)) (PtDown (Lit 1))
                       (Just Macros.untilEndOfTurn))
            , Macros.keyword "Fuse" ]
            Nothing)

||| Kasmina, Enigma Sage's first line -- "Each other planeswalker you
||| control has the loyalty abilities of Kasmina." The DESCRIBED ability
||| payload at the loyalty class, with the source named by self-name --
||| [CR#201.5]'s reference, which `This` already spells: text on a card
||| that names that card means that particular object. The other half of
||| the ledgered pair is Nicol Bolas,
||| Dragon-God below; neither was writable while `Gains` was the only
||| grant, since neither card quotes an ability.
public export
kasminaLoyaltySharing : StaticEffect []
kasminaLoyaltySharing =
  GainsAbilitiesOf (AllOf (And [HasType Planeswalker, ControlledBy You,
                                OtherThan This]))
                   [LoyaltyClass] This Nothing

||| Nicol Bolas, Dragon-God's first line -- "Nicol Bolas has all loyalty
||| abilities of all other planeswalkers on the battlefield." The same
||| row read from the other side: the source is the described GROUP and
||| the subject is the card itself.
public export
nicolBolasDragonGodSharing : StaticEffect []
nicolBolasDragonGodSharing =
  GainsAbilitiesOf This [LoyaltyClass]
                   (AllOf (And [HasType Planeswalker, OtherThan This,
                                InZone Macros.battlefieldZ]))
                   Nothing

||| Myr Welder's second line -- "This creature has all activated
||| abilities of all cards exiled with it." The commonest spelling of the
||| described payload (26 of the 29 supported lines write "all activated
||| abilities of"), over the exile linkage [CR#607.2a] its own first line
||| makes. Dark Impostor, Patchwork Crawler and Rex, Cyber-Hound write
||| the same sentence.
public export
myrWelderBorrowedAbilities : StaticEffect []
myrWelderBorrowedAbilities =
  GainsAbilitiesOf Macros.thisCreature [AnyActivated]
                   (AllOf (ExiledWith This)) Nothing

||| Sharkey, Tyrant of the Shire's third line -- "Sharkey has all
||| activated abilities of lands your opponents control except mana
||| abilities." The EXCEPTION slot at [CR#605.1a]'s derived property,
||| which `IsManaAbility` already spelled for the ability-on-the-stack
||| noun. Scheming Fence writes the other supported exception ("except
||| for loyalty abilities") and does not bench: its source is "the chosen
||| permanent", the object-sorted marked read.
public export
sharkeyBorrowedLandAbilities : StaticEffect []
sharkeyBorrowedLandAbilities =
  GainsAbilitiesOf This [AnyActivated]
                   (AllOf (And [Macros.land,
                                ControlledBy (PlayerGroup YourOpponents)]))
                   (Just IsManaAbility)


||| Conspicuous Snoop, whole card -- "Play with the top card of your
||| library revealed. / You may cast Goblin spells from the top of your
||| library. / As long as the top card of your library is a Goblin card,
||| this creature has all activated abilities of that card." The first
||| two lines were the visibility rider and the top-of-library
||| permission; the third was the ability-borrowing gap, and it is the
||| described payload over a condition's own mention.
public export
conspicuousSnoop : Card
conspicuousSnoop =
  Macros.card "Conspicuous Snoop" (Just [Macros.pip Red, Macros.pip Red]) []
       (MkTypeLine [creatureType "Goblin", creatureType "Rogue"] [Creature])
       [ Static (Visibility Reveal You TopOfLibrary)
       , Static (Macros.mayPlayFrom You
                   (AllOf (And [Macros.spell,
                                HasSubtype (creatureType "Goblin")]))
                   Macros.onTopZ)
       , Static (Macros.asLongAs
                   (Matches Macros.topCard (HasSubtype (creatureType "Goblin")))
                   (GainsAbilitiesOf Macros.thisCreature [AnyActivated]
                                     (That CardW) Nothing)) ]
       (Just (2, 2))

||| Skill Borrower, whole card -- the same two lines at an artifact
||| creature, the condition widened to "an artifact or creature card".
||| Its reminder text ("If any of the abilities use that card's name,
||| use this creature's name instead") is [CR#201.5b] restated and is not
||| part of the ability.
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

||| Blind Fury, whole card -- "All creatures lose trample until end of
||| turn. / If a creature would deal combat damage to a creature this
||| turn, it deals double that damage to that creature instead." The
||| named-ability loss was the card's last blocker; its second sentence
||| is the ordinary combat-damage doubling.
public export
blindFury : Card
blindFury =
  Macros.card "Blind Fury"
       (Just [Macros.generic 2, Macros.pip Red, Macros.pip Red]) []
       (MkTypeLine [] [Instant])
       [ Spell (Sequentially
                  [ Continuously
                      (LosesAbilities (AllOf Macros.creature)
                                      [KeywordAbility "Trample" Nothing])
                      (Just Macros.untilEndOfTurn)
                  , Continuously
                      (Scales CombatOnly (Macros.a Macros.creature)
                              (Macros.shieldingIt (Macros.a Macros.creature))
                              (Multiplied Doubled) Repeatedly)
                      (Just Macros.thisTurn) ]) ]
       Nothing

||| Shadowspear's second line -- "{1}: Permanents your opponents control
||| lose hexproof and indestructible until end of turn." The payload's
||| LIST arm: one subject, one loss, two abilities. Bonds of Mortality
||| and The Fire Nation Drill write the same sentence, and Shay Cormac
||| writes it at five words.
public export
shadowspearStrip : Ability
shadowspearStrip =
  Macros.activated (Mana [Macros.generic 1])
    (Continuously
       (LosesAbilities
          (AllOf (And [Permanent, ControlledBy (PlayerGroup YourOpponents)]))
          [ KeywordAbility "Hexproof" Nothing
          , KeywordAbility "Indestructible" Nothing ])
       (Just Macros.untilEndOfTurn))

||| Blood Sun, whole card -- "When this enchantment enters, draw a card. /
||| All lands lose all abilities except mana abilities." The exception on
||| the ability LOSS, and the corpus's only line that writes one there.
public export
bloodSun : Card
bloodSun =
  Macros.card "Blood Sun" (Just [Macros.generic 2, Macros.pip Red]) []
       (MkTypeLine [] [Enchantment])
       [ Macros.triggered When (Enters Macros.thisEnchantment Nothing)
                          Macros.drawACard
       , Static (LosesAllAbilities (AllOf Macros.land) (Just IsManaAbility)) ]
       Nothing

-- ---------------------------------------------------------------------------
-- The static statement's TURN WINDOW, re-measured.
-- ---------------------------------------------------------------------------

||| Ahn-Crop Invader, whole card -- "During your turn, this creature has
||| first strike. / {1}, Sacrifice another creature: This creature gets
||| +2/+0 until end of turn."
|||
||| The window is `OnlyDuring`, which landed with the prohibition round
||| and is the THIRD reader of the timing vocabulary the statement frame
||| was said to lack: `windowOk` gates it exactly as it gates an
||| activation restriction's `Timing` and a trigger's `TriggerWindow`,
||| and nothing here is a copy of either grid. 96 supported lines over 94
||| cards write a window-confined static statement (measured 2026-09-02),
||| 91 of them "during your turn"; the ledger that called this cell two
||| lines predates the carrier. Restless Spire writes this same sentence
||| inside a quoted payload and At Knifepoint at a described group --
||| that one is blocked on "outlaws", the cover word for five creature
||| types, and not on the window.
public export
ahnCropInvader : Card
ahnCropInvader =
  Macros.card "Ahn-Crop Invader" (Just [Macros.generic 2, Macros.pip Red]) []
       (MkTypeLine [creatureType "Zombie", creatureType "Minotaur",
                    creatureType "Warrior"] [Creature])
       [ Static (OnlyDuring Turn (Just Yours)
                   (Gains Macros.thisCreature
                          (KeywordAbility "FirstStrike" Nothing)))
       , Macros.activated
           (Compound [ Mana [Macros.generic 1]
                     , Do (Macros.sacrifice You
                             (Macros.a (And [Macros.creature, OtherThan This]))) ])
           (Macros.gets Macros.thisCreature (PtUp (Lit 2)) (PtUp (Lit 0))
                        (Just Macros.untilEndOfTurn)) ]
       (Just (2, 2))

||| Bedrock Tortoise's second line -- "During your turn, creatures you
||| control have hexproof." The same window over a DESCRIBED GROUP rather
||| than the source, which is how most of the 96 write it.
public export
bedrockTortoiseWindow : StaticEffect []
bedrockTortoiseWindow =
  OnlyDuring Turn (Just Yours)
    (Gains (AllOf Macros.creatureYouControl)
           (KeywordAbility "Hexproof" Nothing))

-- ---------------------------------------------------------------------------
-- The marker object's self-ascription and the grantor named from inside
-- the quotation.
-- ---------------------------------------------------------------------------

||| Nesting Dragon's inner token payload -- "{R}: This token gets +1/+0
||| until end of turn." The marker word at the seat that wanted it: 102
||| of the 206 distinct quoted token-creation payloads name their bearer
||| "this token" (measured 2026-09-02), and `TokenChars.abilities`
||| already held whole abilities.
|||
||| The word ascribes NO type, which is the point of the third axis and
||| also its limit: "This token can't block" (Harried Spearguard, Anax,
||| Hardened in the Forge) still does not write, because the deed table
||| gives "Block" a Creature-typed agent and refuses a bare subject
||| ([CR#509.1a] chooses blockers from among creatures). That is the deed
||| vocabulary's cell, not this word's -- a payload whose verb demands a
||| typed subject needs the type word, and the marker word is not one.
public export
nestingDragonInnerToken : AbilityAt []
nestingDragonInnerToken =
  Macros.activated (Mana [Macros.pip Red])
    (Macros.gets (AsMarker TokenMarker This) (PtUp (Lit 1)) (PtUp (Lit 0))
                 (Just Macros.untilEndOfTurn))

||| Chandra, Awakened Inferno's emblem -- "You get an emblem with 'At the
||| beginning of your upkeep, this emblem deals 1 damage to you.'" The
||| same word at the OTHER marker object, 9 of the 90 distinct emblem
||| payloads. [CR#114.3] leaves an emblem no types, so the word is the
||| only self-reference such a payload has, and [CR#114.2] is what puts
||| it in the command zone.
public export
chandraAwakenedInfernoEmblem : Effect []
chandraAwakenedInfernoEmblem =
  GetsEmblem You
    [ Macros.triggered At (BeginningOf Upkeep (ByWord Yours))
        (DealDamage (AsMarker EmblemMarker This) (Lit 1) You) ]

||| Leonin Bola, whole card -- "Equipped creature has '{T}, Unattach
||| Leonin Bola: Tap target creature.' / Equip {1}". [CR#201.5a]'s
||| reference: the granted ability names the Equipment, and the name
||| denotes THAT object rather than a class of objects with the name.
||| Heartseeker, Blazing Torch, Razor Boomerang and Shuriken all write
||| their own name at the same seat.
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

-- ---------------------------------------------------------------------------
-- The PER-PART span on the shared-subject coordination.
-- ---------------------------------------------------------------------------

||| Distortion Strike, whole card -- "Target creature gets +1/+0 until
||| end of turn and can't be blocked this turn. / Rebound". One subject,
||| two verb phrases, and TWO SPANS: the grant is written "until end of
||| turn" and the restriction "this turn", which are different `Duration`
||| values, so the single envelope `Continuously` puts over the whole
||| coordination could not write the line. The envelope stays as the
||| ELIDED form -- "gets +2/+2 and gains trample until end of turn" still
||| writes one span at the end and no arm span at all.
||| Taigam's Strike writes the same line at +2/+0; Teleportal, Marchesa's
||| Smuggler, Veil of Secrecy and Cephalid Inkshrouder are the same
||| family at other verbs.
||| The card does not bench whole: its second line is the keyword
||| Rebound, which the keyword catalog does not carry -- that word's
||| cell, not this span's.
public export
distortionStrikeLine : Effect []
distortionStrikeLine =
  Macros.sharedSubject
    (Macros.target Macros.creature)
    [ VPGets (PtUp (Lit 1)) (PtUp (Lit 0)) (Just Macros.untilEndOfTurn)
    , VPDeontic Forbid ["Block"] Patient (Just Macros.thisTurn) ]
    Nothing

||| Battlegate Mimic, whole card -- "Whenever you cast a spell that's
||| both red and white, this creature has base power and toughness 4/2
||| until end of turn and gains first strike until end of turn."
|||
||| THE SAME-WORD-TWICE VERDICT, and it is a spelling variant of the
||| shared envelope. Re-measured 2026-09-02: four lines write the
||| identical current-turn word twice on one shared-subject coordination
||| (the Mimic cycle's flying, first strike, trample and wither members;
||| the fifth, Riverfall Mimic, writes the can't-be-blocked variant and
||| belongs to the DISAGREEING family), plus Sylvan Awakening's
||| land-copy line. Because both written spans are the same `Duration`
||| value, the envelope already says what the line says, and the family
||| that actually needed a per-part span shrinks to the 51 grant-plus-
||| restriction lines `VPDeontic` buys. This card benches on the
||| envelope, unchanged -- `HasBasePt` has no verb-phrase arm, so the
||| coordination is `AndAlso`'s and the second statement writes the
||| deictic again rather than reading it back.
public export
battlegateMimic : Card
battlegateMimic =
  Macros.card "Battlegate Mimic"
       (Just [Macros.generic 1, Macros.hybridPip Red White]) []
       (MkTypeLine [creatureType "Shapeshifter"] [Creature])
       [ Macros.triggered Whenever
           (Casts You (Macros.a (And [Macros.spell, ColorIs Red,
                                      ColorIs White])) Nothing)
           (Continuously
              (AndAlso [ HasBasePt Macros.thisCreature (Lit 4) (Lit 2)
                       , Gains Macros.thisCreature
                               (Macros.keyword "FirstStrike") ])
              (Just Macros.untilEndOfTurn)) ]
       (Just (2, 1))


-- ---------------------------------------------------------------------------
-- The MULTI-SENTENCE static line, and why it wanted no carrier.
-- ---------------------------------------------------------------------------

||| Retro-Mutation, whole card -- "Enchant creature / Enchanted creature
||| is a Turtle with base power and toughness 0/1. It can't attack and
||| loses all abilities."
|||
||| The line prints TWO SENTENCES and the second reads the first's
||| subject back, which was ledgered as a carrier gap: `Static` holds one
||| `StaticEffect`, so the second sentence was said to have nowhere to
||| stand. It does not need one. `AndAlso` coordinates whole STATEMENTS,
||| each naming its own subject and the later ones free to read an
||| earlier one back -- which is exactly what this line does -- so the
||| full stop is SPELLING and carries no rules content the "and" does
||| not. Lithoform Blight's bench entry decided the same question the
||| same way one round earlier.
|||
||| Re-measured 2026-09-02: 11 supported static ability lines write two
||| sentences with the second reading the first's subject back. They are
||| NOT one class, and the split is what a second carrier would have
||| hidden: 6 are a plain further statement (this card, Spider-Man No
||| More, Heliod's Punishment, Intercessor's Arrest, Bride's Gown,
||| Groom's Finery), 3 write a REPLACING second statement (Mind Carver,
||| Precipitous Drop and So Tiny's "It gets +3/+1 INSTEAD as long as …")
||| and 2 give the subject's controller leave to ignore the effect (Lost
||| in Thought, Volrath's Curse). The plain six write today; the other
||| five want the "instead"/"additional" marking and the
||| ignore-this-effect permission, which are their own cells and are
||| recorded rather than minted -- 3 lines and 2 lines respectively.
||| The 12 Zendikon-style "It's still a land" lines are not in this
||| count: `SetsType`'s retention slot already spells them.
public export
retroMutation : Card
retroMutation =
  Macros.card "Retro-Mutation" (Just [Macros.generic 2, Macros.pip Blue]) []
       (MkTypeLine [enchantmentType "Aura"] [Enchantment])
       [ Macros.keywordSubject "Enchant" Macros.creature
       , Static (AndAlso
           [ SetsType (AttachHost Enchanted (TypeW Creature))
                      (MkToken Nothing []
                               (MkTypeLine [creatureType "Turtle"] []) [] Nothing)
                      Nothing
           , HasBasePt It (Lit 0) (Lit 1)
           , Deontic It Forbid ["Attack"] Agent NoDeonticPatient Nothing
           , LosesAllAbilities It Nothing ]) ]
       Nothing

||| Alluring Suitor's activated ability -- "{R}{R}: This creature and
||| another target creature each get +1/+0 until end of turn." The
||| COMPOUND SUBJECT, recorded on the static-frame ledger as a noun
||| question and answered by the noun vocabulary that already holds it:
||| the "and" is inside the phrase and the distributive "each" is
||| `EachOfBoth` over `BothOf`'s pair. Re-measured 2026-09-02 at 14
||| supported lines, not the three the ledger carried -- Alandra, Sky
||| Dreamer; Eidolon of Countless Battles; Fated Clash; Nighthowler;
||| Razorgrass Invoker; Gogo, Mysterious Mime and the rest. Nothing was
||| owed here; the entry is the witness that says so at a STATIC subject.
public export
alluringSuitorPump : Ability
alluringSuitorPump =
  Macros.activated (Mana [Macros.pip Red, Macros.pip Red])
    (Macros.gets
       (EachOfBoth
          (BothOf Macros.thisCreature
                  (Macros.target (And [Macros.creature, OtherThan This]))))
       (PtUp (Lit 1)) (PtUp (Lit 0)) (Just Macros.untilEndOfTurn))

-- THE ABILITY ON THE STACK, re-measured 2026-09-02: 40 supported lines
-- name a TARGETED ability there, not the 60 the ledger carried -- 23 at
-- "counter" (Squelch, Stifle, Disallow and Counterspell's kin, all
-- benched above) and 15 at "copy" (Strionic Resonator, Rings of
-- Brighthearth, Lithoform Engine, Illusionist's Bracers), plus two
-- oblique mentions. The NOUN is landed: `AbilityHead`'s class word with
-- `AbilityOf`/`ActivatedBy` for the possessor and the source
-- restriction, and `Counterable`'s ability row is what lets the counter
-- verb take it.
--
-- The COPY verb does not, and the ledger's "both verbs' complement
-- slots are already the ordinary stack noun" is wrong about it:
-- `CopyStack` takes a `Noun bs Object` under `OnStack`, where
-- `CounterSpell` is kind-indexed under `Counterable`. Widening it wants
-- `Counterable`'s twin AND a second move at the rider -- every one of
-- the 15 lines goes on to say "you may choose new targets for the
-- copy", and `ChooseNewTargets` is Object-kinded too, with
-- `wordReaches CopyW AbilityP = False` behind it. Two widenings in the
-- COPY verb's own family; recorded here at its count rather than taken
-- by this round.

-- ---------------------------------------------------------------------------
-- Measured, and deliberately not built.
-- ---------------------------------------------------------------------------
--
-- THE GRANTED ABILITY'S SUBJECT IN A NON-BATTLEFIELD ZONE: FOUR supported
-- spans, re-measured 2026-09-02 and unchanged. Case of the Uneaten Feast,
-- Kethis, the Hidden Hand and The Grim Captain's Locker grant to cards in a
-- GRAVEYARD; Lukka, Coppercoat Outcast to cards EXILED this way. Hand and
-- library are ZERO. Three of the four payloads are play permissions
-- `MayPlay` already spells -- it is the SUBJECT that refuses -- and
-- [CR#113.6b] is what those payloads say about themselves. Under the bar;
-- no row minted.
--
-- THE ONE-TIME BOON: 27 cards write "you get a one-time boon with '…'" and
-- NONE of them is supported (re-measured 2026-09-02). Every one is Alchemy,
-- which is the supported flag doing its job. Its shapes, for whoever
-- inherits a changed flag: one-time, two-time, three-time and bare (Merfolk
-- Tunnel-Guide, Tasteful Offering, Swiftspear's Teachings, Flaming Fist
-- Duskguard).
--
-- THE ATTACHMENT HEAD `Equipped` × `PermanentW`: the ledger called this a
-- live False discrepancy; it is neither. `attachHeadOk Equipped PermanentW`
-- is True and the table's own docstring names Luxior, Giada's Gift as the
-- printing that opened it. Re-measured 2026-09-02: "equipped permanent" 1
-- line, "equipped planeswalker" 0, "enchanted permanent" 105.
