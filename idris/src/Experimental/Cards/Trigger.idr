module Experimental.Cards.Trigger

import Experimental
import Experimental.Macros
import Experimental.Cards.Description
import Experimental.Cards.Anaphora

%default total


gracefulReprieve : Instruction []
gracefulReprieve = Macros.delayedWithin (Dies (Macros.target Macros.creature))
                                        ThisTurn
                                        (Macros.move (Macros.That CardW OneOf) Macros.battlefieldZ)

cloudkinSeer : Ability
cloudkinSeer = Macros.triggered When (Enters Macros.thisCreature Nothing) (Draw You (Lit 1))

promiseOfTomorrow : Ability
promiseOfTomorrow = Macros.triggered Whenever (Dies (Macros.a Macros.creatureYouControl)) (Macros.exile ((Macros.It OneOf)))

libraryLarcenist : Ability
libraryLarcenist = Macros.triggered Whenever (Macros.attacks Macros.thisCreature) (Draw You (Lit 1))

jhessianThief : Ability
jhessianThief =
  Macros.triggered Whenever (Macros.dealsCombatDamage Macros.thisCreature (Macros.a AnyPlayer)) (Draw You (Lit 1))

scholarOfStars : Ability
scholarOfStars =
  Macros.triggeredIf When
                     (Enters Macros.thisCreature Nothing)
                     (Macros.exists (And [Macros.artifact, HasPossessor ControllerAx You]))
                     (Draw You (Lit 1))

miserysShadow : Ability
miserysShadow =
  Static (Intercepts (Dies (Macros.a (And [Macros.creature, HasPossessor ControllerAx (Macros.a Opponent)]))) [] Nothing
                     (Macros.exile ((Macros.It OneOf))) Repeatedly Nothing)

beastWhisperer : Ability
beastWhisperer =
  Macros.triggered Whenever (Casts You (Macros.a (And [Macros.creature, Macros.spell])) Nothing) (Draw You (Lit 1))

mesmericOrb : Ability
mesmericOrb =
  Macros.triggered Whenever (StatusEvent (Macros.a Permanent) Untapped)
                   (Macros.mills (Macros.controllerOf (Macros.That PermanentW OneOf)) (Lit 1) They)

secretPlans : Ability
secretPlans =
  Macros.triggered Whenever
                   (StatusEvent (Macros.a (And [Permanent, HasPossessor ControllerAx You])) FaceUp)
                   (Draw You (Lit 1))

||| Teferi's Imp
teferisImpPhasesOut : Ability
teferisImpPhasesOut =
  Macros.triggered Whenever (StatusEvent Macros.thisCreature PhasedOut)
                   ((Macros.discard You (Macros.a (InZone Macros.handZ))))

||| Teferi's Imp
teferisImpPhasesIn : Ability
teferisImpPhasesIn =
  Macros.triggered Whenever (StatusEvent Macros.thisCreature PhasedIn)
                   (Draw You (Lit 1))

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

hollowhengeSpirit : Ability
hollowhengeSpirit =
  Macros.triggered When (Enters Macros.thisCreature Nothing)
                   (RemoveFromCombat (Macros.target (And [Macros.creature, Or [Attacking, Blocking]])))

netcasterSpider : Ability
netcasterSpider =
  Macros.triggered Whenever
                   (Blocks Macros.thisCreature
                    (Just (Macros.a (And [Macros.creature, HasKeyword (TheKeyword "Flying")]))))
                   (Macros.gets Macros.thisCreature (Up (Lit 2)) (Up (Lit 0)) (Just Macros.untilEndOfTurn))

viashinoWeaponsmith : Ability
viashinoWeaponsmith =
  Macros.triggered Whenever
                   (BecomesBlocked Macros.thisCreature (Just (Macros.a Macros.creature)))
                   (Macros.gets Macros.thisCreature (Up (Lit 2)) (Up (Lit 2)) (Just Macros.untilEndOfTurn))

somberwaldAlpha : Ability
somberwaldAlpha =
  Macros.triggered Whenever (BecomesBlocked (Macros.a Macros.creatureYouControl) Nothing)
                   (Macros.gets ((Macros.It OneOf)) (Up (Lit 1)) (Up (Lit 1)) (Just Macros.untilEndOfTurn))

vertigoSpawn : Ability
vertigoSpawn =
  Macros.triggered Whenever (Blocks Macros.thisCreature (Just (Macros.a Macros.creature)))
                   (Sequentially [SetStatus Tapped (Macros.That (TypeW Creature) OneOf),
                           DoesntUntapNext (Macros.That (TypeW Creature) OneOf) (Lit 1)])

orneryDilophosaur : Ability
orneryDilophosaur =
  Macros.triggeredIf Whenever
                     (Macros.attacks Macros.thisCreature)
                     (Macros.exists (And [Macros.creature, HasPossessor ControllerAx You,
                                   Compare [StatAxis Power] AtLeast (Lit 4)]))
                     (Macros.gets Macros.thisCreature (Up (Lit 2)) (Up (Lit 2)) (Just Macros.untilEndOfTurn))

incisorGlider : Ability
incisorGlider =
  Macros.triggeredIf Whenever
                     (Macros.attacks Macros.thisCreature)
                     (CompareAmt (CountersOn (NamedCounter "Poison") (Macros.a Opponent))
                                 AtLeast (Lit 3))
                     (Macros.gets (Macros.allOf Macros.creatureYouControl)
                                  (Up (Lit 1)) (Up (Lit 1))
                                  (Just Macros.untilEndOfTurn))

stormFleetSpy : Ability
stormFleetSpy =
  Macros.triggeredIf When
                     (Enters Macros.thisCreature Nothing)
                     (Macros.happened AttackDeclaration You Lookback.ThisTurn)
                     (Draw You (Lit 1))

loanShark : Ability
loanShark =
  Macros.triggeredIf When
                     (Enters Macros.thisCreature Nothing)
                     (CompareAmt (Macros.eventCount SpellCast You Lookback.ThisTurn)
                                 AtLeast (Lit 2))
                     (Draw You (Lit 1))

forceOfDespair : Instruction []
forceOfDespair =
  Macros.destroy (Macros.allOf (And [Macros.creature,
                              Macros.happenedTo Entry Lookback.ThisTurn]))

cradleToGrave : Instruction []
cradleToGrave =
  Macros.destroy (Macros.target (And [Macros.creature, Not (ColorIs Black),
                                      Macros.happenedTo Entry Lookback.ThisTurn]))

aragornKingOfGondor : Ability
aragornKingOfGondor =
  Macros.triggered When (Enters Macros.thisCreature Nothing) (Macros.gainsDesignation You Monarch Instructed)

firmamentSage : Ability
firmamentSage = Macros.triggered Whenever DayNightShift (Draw You (Lit 1))

deeprootWarrior : Ability
deeprootWarrior =
  Macros.triggered Whenever (BecomesBlocked Macros.thisCreature Nothing)
                   (Macros.gets ((Macros.It OneOf)) (Up (Lit 1)) (Up (Lit 1)) (Just Macros.untilEndOfTurn))

borderlandMarauder : Ability
borderlandMarauder =
  Macros.triggered Whenever (Macros.attacks Macros.thisCreature)
                   (Macros.gets ((Macros.It OneOf)) (Up (Lit 2)) (Up (Lit 0)) (Just Macros.untilEndOfTurn))

lichsMasteryLoss : Ability
lichsMasteryLoss =
  Macros.triggered When (Macros.leavesBattlefield Macros.thisEnchantment) (Concludes LoseGame You)

phageTheUntouchable : Ability
phageTheUntouchable =
  Macros.triggered Whenever
                   (Macros.dealsCombatDamage Macros.thisCreature (Macros.a AnyPlayer))
                   (Concludes LoseGame (Macros.That PlayerW OneOf))

elderscaleWurm : Ability
elderscaleWurm =
  Macros.triggeredIf When
                     (Enters Macros.thisCreature Nothing)
                     (CompareAmt (PlayerStatOf LifeTotal You) Less (Lit 7))
                     (Macros.lifeBecomes You (Lit 7))

||| Krang, Master Mind
krang : Ability
krang =
  Macros.triggeredIf When
                     (Enters Macros.thisCreature Nothing)
                     (CompareAmt (Macros.countOf (InZone (Macros.handOf You)))
                                 Less (Lit 4))
                     (Draw You TheDifference)

carrionGrub : Card
carrionGrub =
  Macros.card "Carrion Grub" (Just [Macros.generic 3, Macros.pip Black]) []
       (MkTypeLine [creatureType "Insect"] [Creature])
       [ Static (AndAlso Nothing [ Modify Macros.thisCreature Power (Up (LetterVal X))
                                 , Modify Macros.thisCreature Toughness (Up (Lit 0))
                         , DefinesLetter X
                             (Macros.aggregate MaxOf (StatAxis Power)
                                        (And [Macros.creature,
                                              InZone (Macros.graveyardOf You)])) ])
       , Macros.triggered When (Enters Macros.thisCreature Nothing)
                        (Macros.mills You (Lit 4) You) ]
       (Just (0, 5))

||| Once Upon a Time
public export
onceUponATimeFirstCast : Predicate [] Object
onceUponATimeFirstCast = Macros.nthCastBy (Nth 1) You (RankWithin Lookback.ThisGame)

youngPyromancer : Card
youngPyromancer =
  Macros.card "Young Pyromancer" (Just [Macros.generic 1, Macros.pip Red]) []
       (MkTypeLine [creatureType "Human", creatureType "Shaman"] [Creature])
       [ Macros.triggered Whenever
                          (Casts You (Macros.a (And [Macros.instantOrSorcery, Macros.spell])) Nothing)
                          (Create You (Lit 1)
                            (TokenWritten (Macros.creatureTok 1 1 [Red] [creatureType "Elemental"])) []) ]
       (Just (2, 1))

archivistOfGondor : Ability
archivistOfGondor =
  Macros.triggeredIf When
                     (Macros.dealsCombatDamage Macros.yourCommander (Macros.a AnyPlayer))
                     (Macros.thereIsNo Monarch)
                     (Macros.gainsDesignation You Monarch Instructed)

hissingMiasma : Card
hissingMiasma =
  Macros.card "Hissing Miasma"
       (Just [Macros.generic 1, Macros.pip Black, Macros.pip Black]) []
       (MkTypeLine [] [Enchantment])
       [ Macros.triggered Whenever
                          (Macros.attacksPlayer (Macros.a Macros.creature) You)
                          (Macros.losesLife (Macros.controllerOf ((Macros.It OneOf))) (Lit 1)) ]
       Nothing

orimsPrayer : Card
orimsPrayer =
  Macros.card "Orim's Prayer"
       (Just [Macros.generic 1, Macros.pip White, Macros.pip White]) []
       (MkTypeLine [] [Enchantment])
       [ Macros.triggered Whenever
                          (Macros.attacksPlayer (Macros.counted (Macros.atLeast 1) Macros.creature) You)
                          (Macros.gainsLife You
                             (Macros.forEach 1 (And [Attacking, Macros.creature]))) ]
       Nothing

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

||| Chub Toad
chubToad : Card
chubToad =
  Macros.card "Chub Toad"
       (Just [Macros.generic 2, Macros.pip Green]) []
       (MkTypeLine [creatureType "Frog"] [Creature])
       [ Macros.triggeredOr Whenever
                            (Blocks Macros.thisCreature Nothing)
                            [BecomesBlocked Macros.thisCreature Nothing]
                            (Macros.gets ((Macros.It OneOf)) (Up (Lit 2)) (Up (Lit 2))
                                         (Just Macros.untilEndOfTurn)) ]
       (Just (1, 1))

||| Giggling Skitterspike
public export
gigglingSkitterspikeTriggers : List (GameEvent [])
gigglingSkitterspikeTriggers =
  [ Blocks Macros.thisCreature Nothing
  , BecomesTarget Macros.thisCreature (Macros.a Macros.spell) ]

public export
gigglingSkitterspikeArms : AltEvent Whenever Trigger.gigglingSkitterspikeTriggers
gigglingSkitterspikeArms = MoreAlt

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

||| Bloom Hulk
bloomHulk : Card
bloomHulk =
  Macros.card "Bloom Hulk" (Just [Macros.generic 3, Macros.pip Green]) []
       (MkTypeLine [creatureType "Plant", creatureType "Elemental"] [Creature])
       [ Macros.triggered When (Enters Macros.thisCreature Nothing) Macros.proliferate ]
       (Just (4, 4))

||| Stalwart Successor
stalwartSuccessorHeader : GameEvent []
stalwartSuccessorHeader =
  Macros.bareCounterEvent CounterPut ManyCounters (Macros.a Macros.creatureYouControl)

||| Hollowmurk Siege
hollowmurkSiegeSultai : Ability
hollowmurkSiegeSultai =
  Macros.triggeredOnlyOnce Whenever
    (Macros.bareCounterEvent CounterPut OneCounter (Macros.a Macros.creatureYouControl))
    OncePerTurn
    (Draw You (Lit 1))

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

||| Chameleon, Master of Disguise
public export
chameleonCopy : Ability
chameleonCopy =
  Static (EntersRider Macros.thisCreature (AsCopyOf True
                       (Macros.a (And [Macros.creature, HasPossessor ControllerAx You]))
                       [ExceptName "Chameleon, Master of Disguise"]))

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
sanguineBond : Card
sanguineBond =
  Macros.card "Sanguine Bond"
       (Just [Macros.generic 3, Macros.pip Black, Macros.pip Black]) []
       (MkTypeLine [] [Enchantment])
       [ Macros.triggered Whenever (LifeChanges You LifeGoesUp)
                          (Macros.losesLife (Macros.target Opponent) ThatMuch) ]
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

||| Agate-Blade Assassin
public export
agateBladeAssassin : Card
agateBladeAssassin =
  Macros.card "Agate-Blade Assassin"
       (Just [Macros.generic 1, Macros.pip Black]) []
       (MkTypeLine [creatureType "Lizard", creatureType "Assassin"] [Creature])
       [ Macros.triggered Whenever (Attacks Macros.thisCreature NoDefender)
           (Sequentially [ Macros.losesLife TheDefendingPlayer (Lit 1)
                         , Macros.gainsLife You (Lit 1) ]) ]
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
                   (Macros.losesLife TheAttackingPlayer ThatMuch)

public export
unstableShapeshifter : Card
unstableShapeshifter =
  Macros.card "Unstable Shapeshifter"
       (Just [Macros.generic 3, Macros.pip Blue]) []
       (MkTypeLine [creatureType "Shapeshifter"] [Creature])
       [ Macros.triggered Whenever
                          (Enters (Macros.a (And [Macros.creature, OtherThan This])) Nothing)
                          (Continuously
                      (BecomesCopy Macros.thisCreature (Macros.That (TypeW Creature) OneOf)
                                   [ExceptThisAbility])
                      Nothing) ]
       (Just (0, 1))

public export
prismRing : Card
prismRing =
  Macros.card "Prism Ring" (Just [Macros.generic 1]) []
       (MkTypeLine [] [Artifact])
       [ Static (Macros.entersChoosing Macros.thisArtifact Color)
       , Macros.triggered Whenever
                          (Casts You (Macros.a (And [Macros.spell, Macros.ofChosen Color])) Nothing)
                          (Macros.gainsLife You (Lit 1)) ]
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
                            (Macros.becomesColor (Macros.That (TypeW Creature) OneOf) (SomeColors [Green]) Nothing) ]
       (Just (1, 1))

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
                                 Named (SameNameAs (Macros.That SpellW OneOf))])) ]) ]
       Nothing

public export
chromeReplicator : Card
chromeReplicator =
  Macros.card "Chrome Replicator" (Just [Macros.generic 5]) []
       (MkTypeLine [creatureType "Construct"] [Artifact, Creature])
       [ Macros.triggeredIf When (Enters Macros.thisCreature Nothing)
           (Exists (Macros.withTheSameName
                           (Macros.counted (Macros.atLeast 2)
                                         (And [Permanent, Not Macros.land,
                                               Macros.nontoken, HasPossessor ControllerAx You]))))
           (Macros.create (Lit 1)
              (MkToken (Just (Lit 4 ** Lit 4)) []
                       (MkTypeLine [creatureType "Construct"] [Artifact, Creature])
                       [] Nothing)) ]
       (Just (4, 4))

public export
admiralsOrder : Card
admiralsOrder =
  Macros.card "Admiral's Order"
       (Just [Macros.generic 1, Macros.pip Blue, Macros.pip Blue]) []
       (MkTypeLine [] [Instant])
       [ Static (Macros.onlyWhile
                   (AltCost This (Just (Mana [Macros.pip Blue])))
                   (Macros.happened AttackDeclaration You Lookback.ThisTurn))
       , Spell Nothing (CounterSpell (Macros.target Macros.spell)) ]
       Nothing

public export
fyndhornDruid : Card
fyndhornDruid =
  Macros.card "Fyndhorn Druid" (Just [Macros.generic 2, Macros.pip Green]) []
       (MkTypeLine [creatureType "Elf", creatureType "Druid"] [Creature])
       [ Macros.triggeredIf When
                            (Dies Macros.thisCreature)
                            (Macros.happened BlockedDeclaration ((Macros.It OneOf)) Lookback.ThisTurn)
                            (Macros.gainsLife You (Lit 4)) ]
       (Just (2, 2))

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
dreadSlaver : Card
dreadSlaver =
  Macros.card "Dread Slaver" (Just [Macros.generic 3, Macros.pip Black, Macros.pip Black]) []
       (MkTypeLine [creatureType "Zombie", creatureType "Horror"] [Creature])
       [ Macros.triggered Whenever (Dies (Macros.a (And [Macros.creature,
                                              Macros.happenedToInvolving DamageTaken
                                                                         Lookback.ThisTurn
                                                                         Macros.thisCreature])))
           (Sequentially [Macros.putOntoBattlefieldUnderYourControl ((Macros.It OneOf)),
                          Macros.becomesAs (Macros.That (TypeW Creature) OneOf)
                                           (MkToken Nothing [Black] (MkTypeLine [creatureType "Zombie"] []) [] Nothing)
                                           Nothing]) ]
       (Just (3, 5))

||| Steppe Lynx
public export
steppeLynx : Ability
steppeLynx =
  Macros.abilityWord "landfall"
    (Macros.triggered Whenever (Enters (Macros.a (And [Macros.land, HasPossessor ControllerAx You])) Nothing)
                      (Macros.gets Macros.thisCreature (Up (Lit 2)) (Up (Lit 2))
                            (Just Macros.untilEndOfTurn)))

||| Owlbear
public export
owlbear : Ability
owlbear =
  Macros.flavorWord "Keen Senses"
    (Macros.triggered When (Enters Macros.thisCreature Nothing) (Draw You (Lit 1)))

||| Netherese Puzzle-Ward
public export
netheresePuzzleWardIllumination : Ability
netheresePuzzleWardIllumination =
  Macros.triggered Whenever Macros.youRollHighestNatural (Draw You (Lit 1))

||| Farideh, Devil's Chosen
public export
faridehResultRead : Ability
faridehResultRead =
  Macros.triggered Whenever (RollsDice You ManyDice AnyDie AnyResult)
    (Sequentially
       [ Macros.gains Macros.thisCreature (Macros.keyword "Flying")
                      (Just Macros.untilEndOfTurn)
       , Macros.gains Macros.thisCreature (Macros.keyword "Menace")
                      (Just Macros.untilEndOfTurn)
       , Macros.ifThen (AnyResultIs AtLeast (Lit 10)) (Draw You (Lit 1)) ])

||| Jaws of Defeat
public export
jawsOfDefeat : Ability
jawsOfDefeat =
  Macros.triggered Whenever (Enters (Macros.a Macros.creatureYouControl) Nothing)
    (Macros.losesLife (Macros.target Opponent)
       (DifferenceBetween (StatOf Power (Macros.That (TypeW Creature) OneOf))
                          (StatOf Toughness (Macros.That (TypeW Creature) OneOf))))

||| Defiling Daemogoth
public export
defilingDaemogothDrain : Instruction []
defilingDaemogothDrain =
  Sequentially [ Macros.losesLife (Macros.each Opponent) (LetterVal X)
               , Define X (Macros.eventSum LifeGain You Lookback.ThisTurn) ]

public export
skullsporeNexusTrigger : Ability
skullsporeNexusTrigger =
  Macros.triggered Whenever
    (Dies (Macros.counted (Macros.atLeast 1)
                        (And [Macros.nontoken, Macros.creatureYouControl])))
    (Macros.create (Lit 1)
       (Macros.creatureTokOf
          (Aggregate SumOf (StatAxis Power) (Macros.That CardW ManyOf))
          (Aggregate SumOf (StatAxis Power) (Macros.That CardW ManyOf))
          [Green] [creatureType "Fungus", creatureType "Dinosaur"]))

||| Wavebreak Hippocamp
public export
wavebreakHippocamp : Ability
wavebreakHippocamp =
  Macros.triggeredOnlyDuring Whenever
    (NthOccurrence (Nth 1) Nothing (Casts You (Macros.a Macros.spell) Nothing))
    (DuringWindow Turn (Just (Macros.each Opponent)))
    (Draw You (Lit 1))

||| Midnight Clock
public export
midnightClockHeader : GameEvent []
midnightClockHeader =
  NthOccurrence (Nth 12) Nothing
    (Macros.counterEvent CounterPut (NamedCounter "Hour") OneCounter Macros.thisArtifact)

||| Political Triumph
public export
politicalTriumphHeader : GameEvent []
politicalTriumphHeader =
  NthOccurrence (Nth 4) Nothing
    (Macros.counterEvent CounterPut (NamedCounter "Plan") OneCounter Macros.thisEnchantment)

||| Thought Lash
public export
thoughtLashTrigger : Ability
thoughtLashTrigger =
  Macros.triggered When
    (PaysCost (Just (Macros.a AnyPlayer)) Unpaid Macros.thisEnchantment
              "CumulativeUpkeep")
    (Macros.exiles (Macros.That PlayerW OneOf) (Macros.each (InZone (Macros.libraryOf They))))

||| Heart of Bogardan
public export
heartOfBogardanHeader : GameEvent []
heartOfBogardanHeader =
  PaysCost (Just (Macros.a AnyPlayer)) Unpaid Macros.thisEnchantment "CumulativeUpkeep"

||| Balduvian Fallen
public export
balduvianFallenHeader : GameEvent []
balduvianFallenHeader =
  PaysCost Nothing Paid Macros.thisCreature "CumulativeUpkeep"

||| Stormscape Battlemage
public export
stormscapeBattlemageFirstKicker : Ability
stormscapeBattlemageFirstKicker =
  Macros.triggeredIf When (Enters Macros.thisCreature Nothing)
    (Macros.costWasPaid (ByNthKeyword (Nth 1) "Kicker") Nothing Macros.thisCreature)
    (Macros.gainsLife You (Lit 3))

||| All-Seeing Arbiter
public export
allSeeingArbiterHeader : GameEvent []
allSeeingArbiterHeader =
  VerbedEvent (Just You) "Discard" (Just (Macros.a (InZone Macros.handZ)))
              Nothing

||| Liliana's Caress
public export
lilianasCaress : Card
lilianasCaress =
  Macros.card "Liliana's Caress"
       (Just [Macros.generic 1, Macros.pip Black]) []
       (MkTypeLine [] [Enchantment])
       [ Macros.triggered Whenever
           (VerbedEvent (Just Macros.anOpponent) "Discard"
                        (Just (Macros.a (InZone Macros.handZ))) Nothing)
           (Macros.losesLife They (Lit 2)) ]
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
           (VerbedEvent (Just You) "Proliferate" Nothing Nothing)
           (Sequentially [ Macros.losesLife (Macros.each Opponent) (Lit 2)
                         , Macros.gainsLife You (Lit 2) ]) ]
       (Just (1, 3))

||| Reciprocate
public export
reciprocate : Card
reciprocate =
  Macros.card "Reciprocate" (Just [Macros.pip White]) []
       (MkTypeLine [] [Instant])
       [ Spell Nothing (Macros.exile
                  (Macros.target
                     (And [Macros.creature,
                           Macros.happenedToInvolving DamageDealing
                                                      Lookback.ThisTurn
                                                      You]))) ]
       Nothing

||| Tahngarth, First Mate
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
    (Draw You (Lit 1))

||| Commander's Insignia
public export
commandersInsignia : Card
commandersInsignia =
  Macros.card "Commander's Insignia"
       (Just [Macros.generic 2, Macros.pip White, Macros.pip White]) []
       (MkTypeLine [] [Enchantment])
       [ Static (Macros.getsPt (Macros.allOf Macros.creatureYouControl)
                      (Up (Macros.eventCountFrom SpellCast You
                               Lookback.ThisGame Macros.yourCommander
                               (FromZone [Macros.commandZ])))
                      (Up (Macros.eventCountFrom SpellCast You
                               Lookback.ThisGame Macros.yourCommander
                               (FromZone [Macros.commandZ])))) ]
       Nothing

||| Faith's Reward
public export
faithsReward : Card
faithsReward =
  Macros.card "Faith's Reward"
       (Just [Macros.generic 3, Macros.pip White]) []
       (MkTypeLine [] [Instant])
       [ Spell Nothing (Macros.move
                  (Macros.allOf (And [Permanent,
                               InZone (Macros.graveyardOf You),
                               HappenedTo (MkLookback Placement Lookback.ThisTurn (Just (FromZones
                                          (FromZone [Macros.battlefieldZ])
                                          Nothing)))]))
                  Macros.battlefieldZ) ]
       Nothing

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

||| Forsaken Wastes
public export
forsakenWastesTargeted : Ability
forsakenWastesTargeted =
  Macros.triggered Whenever
    (BecomesTarget Macros.thisEnchantment (Macros.a Macros.spell))
    (Macros.losesLife (Macros.controllerOf (Macros.That SpellW OneOf)) (Lit 5))

||| Fblthp, the Lost
public export
fblthp : Card
fblthp =
  Macros.card "Fblthp, the Lost" (Just [Macros.generic 1, Macros.pip Blue])
       [Legendary]
       (MkTypeLine [creatureType "Homunculus"] [Creature])
       [ Macros.triggered When (Enters This Nothing)
           (InsteadOf (Draw You (Lit 1))
              (If (OrCond
                     [ Happened ((Macros.It OneOf)) (MkLookback Entry Triggering (Just (FromZones (FromZone [Macros.yourLibrary]) Nothing)))
                     , Matches ((Macros.It OneOf)) (CastFrom Macros.yourLibrary) ])
                  (Draw You (Lit 2))
                  Nothing))
       , Macros.triggered When (BecomesTarget This (Macros.a Macros.spell))
           (Macros.shuffleInto You This) ]
       (Just (1, 1))

||| Frost Walker
public export
frostWalker : Card
frostWalker =
  Macros.card "Frost Walker"
       (Just [Macros.generic 1, Macros.pip Blue]) []
       (MkTypeLine [creatureType "Elemental"] [Creature])
       [ Macros.triggered When
           (BecomesTarget Macros.thisCreature
              (Macros.a (Or [Macros.spell, AbilityHead AnyOnStack])))
           (Macros.sacrificeIt You) ]
       (Just (4, 1))

||| Loaming Shaman
public export
loamingShaman : Card
loamingShaman =
  Macros.card "Loaming Shaman"
       (Just [Macros.generic 2, Macros.pip Green]) []
       (MkTypeLine [creatureType "Centaur", creatureType "Shaman"] [Creature])
       [ Macros.triggered When (Enters Macros.thisCreature Nothing)
           (Macros.shuffleInto (Macros.target AnyPlayer)
              (Described (TargetDet Macros.anyNumber) (InZone (Macros.graveyardOf They)))) ]
       (Just (3, 2))

||| Debris Beetle
public export
debrisBeetleTrigger : Ability
debrisBeetleTrigger =
  Macros.triggered When (Enters Macros.thisVehicle Nothing)
    (Sequentially [ Macros.losesLife (Macros.each Opponent) (Lit 3)
                  , Macros.gainsLife You (Lit 3) ])

||| Roads Go Ever, Ever On's chapters II and III
public export
roadsGoEverEverOnChapters : Ability
roadsGoEverEverOnChapters =
  Macros.triggered When (ChapterMark [ChapterII, ChapterIII])
    (Macros.move (Macros.a (And [IsCard, ExiledWith Macros.thisSaga]))
                 Macros.handZ)

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
    (Sequentially [(Macros.discard You (Macros.a (InZone Macros.handZ))), (Draw You (Lit 2))])

||| Ferocious Pup
public export
ferociousPup : Card
ferociousPup =
  Macros.card "Ferocious Pup" (Just [Macros.generic 2, Macros.pip Green]) []
       (MkTypeLine [creatureType "Wolf"] [Creature])
       [ Macros.triggered When (Enters Macros.thisCreature Nothing)
           (Macros.create (Lit 1) (Macros.creatureTok 2 2 [Green] [creatureType "Wolf"])) ]
       (Just (0, 1))

||| Agency Outfitter's search
public export
agencyOutfitterSearch : Instruction []
agencyOutfitterSearch =
  Sequentially
    [ Macros.may You
        (Sequentially
           [ Macros.searchZonesOf You
               (Or [ Named (PrintedName "Magnifying Glass")
                   , Named (PrintedName "Thinking Cap") ])
           , Macros.putOntoBattlefield Macros.foundCard ])
    , If (Macros.happenedAt (VerbedAct "Search") You Lookback.ThisWay
                            Macros.yourLibrary)
         Macros.shuffle Nothing ]

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
              [ Macros.exile (Macros.topSlice (Lit 1))
              , If Anaphora.bioplasmCardTest
                   (Macros.gets Macros.thisCreature
                        (Up (StatOf Power
                                 (Macros.TheVerbed "Exile" (TypedCardW Creature) Attributive OneOf)))
                        (Up (StatOf Toughness (Macros.ItVerbed "Exile" OneOf)))
                        (Just ThisTurn))
                   Nothing ]) ]
       (Just (4, 4))

||| Hostile Investigator
public export
hostileInvestigatorHeader : GameEvent []
hostileInvestigatorHeader =
  VerbedEvent (Just (Macros.counted (Macros.atLeast 1) AnyPlayer))
              "Discard"
              (Just (Macros.counted (Macros.atLeast 1) IsCard)) Nothing

||| Hallowed Moonlight
public export
hallowedMoonlight : Card
hallowedMoonlight =
  Macros.card "Hallowed Moonlight"
       (Just [Macros.generic 1, Macros.pip White]) []
       (MkTypeLine [] [Instant])
       [ Spell Nothing (Sequentially
                  [ Macros.ifWouldInstead
                      (Enters (Macros.a (And [Macros.creature, Not WasCast])) Nothing)
                      (Macros.exile ((Macros.It OneOf)))
                      (Just Macros.untilEndOfTurn)
                  , (Draw You (Lit 1)) ]) ]
       Nothing

||| Gather Specimens
public export
gatherSpecimens : Card
gatherSpecimens =
  Macros.card "Gather Specimens"
       (Just [Macros.generic 3, Macros.pip Blue, Macros.pip Blue, Macros.pip Blue])
       [] (MkTypeLine [] [Instant])
       [ Spell Nothing (Continuously
                  (EntersRider
                     (Macros.a (And [Macros.creature,
                                     HasPossessor ControllerAx Macros.anOpponent]))
                     (Under You))
                  (Just ThisTurn)) ]
       Nothing

||| Don't Blink's replacement, without its written agent
public export
dontBlinkReplacement : Instruction []
dontBlinkReplacement =
  Continuously
   (Intercepts (Enters (Macros.counted (Macros.atLeast 1) Macros.creature)
                     (Just (FromZone [Macros.exileZ])))
             [ Enters (Macros.counted (Macros.atLeast 1)
                         (And [Macros.creature, CastFrom Macros.exileZ]))
                      Nothing ] Nothing
             (Macros.shuffleInto You ((Macros.It ManyOf)))
             Repeatedly Nothing)
   (Just Macros.untilEndOfTurn)

||| Seasoned Warrenguard
public export
seasonedWarrenguard : Card
seasonedWarrenguard =
  Macros.card "Seasoned Warrenguard" (Just [Macros.pip White]) []
       (MkTypeLine [creatureType "Rabbit", creatureType "Warrior"] [Creature])
       [ Macros.triggeredWhile Whenever
           (Macros.attacks Macros.thisCreature)
           (WhileTrue (Macros.exists (And [IsToken, HasPossessor ControllerAx You])))
           (Macros.gets Macros.thisCreature (Up (Lit 2)) (Up (Lit 0))
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
           (WhileTrue
              (CompareAmt (Macros.countOf (And [Macros.artifact, HasPossessor ControllerAx You]))
                          AtLeast (Lit 2)))
           (Macros.gets ((Macros.It OneOf)) (Up (Lit 2)) (Up (Lit 1))
                        (Just Macros.untilEndOfTurn)) ]
       (Just (2, 3))

||| Autarch Mammoth
public export
autarchMammothLine : Ability
autarchMammothLine =
  Macros.triggeredJoined When
    (Enters Macros.thisCreature Nothing)
    [ Macros.joinedHeadWhile Whenever
        (Macros.attacks Macros.thisCreature)
        (WhileTrue
           (Matches Macros.thisCreature (HasDesignation Saddled Nothing))) ]
    (Macros.create (Lit 1)
       (Macros.creatureTok 3 3 [Green] [creatureType "Elephant"]))

||| Altar of the Brood
public export
altarOfTheBrood : Ability
altarOfTheBrood =
  Macros.triggered Whenever
    (Enters (Macros.a (And [Permanent, HasPossessor ControllerAx You, OtherThan This])) Nothing)
    (Macros.mills (Macros.each Opponent) (Lit 1) They)

||| Foul Emissary
public export
foulEmissaryLine : Ability
foulEmissaryLine =
  Macros.triggeredWhile When
    (VerbedEvent (Just You) "Sacrifice" (Just Macros.thisCreature) Nothing)
    (WhileDoing (Casts You
                   (Macros.a (And [Macros.spell,
                                   HasKeyword (TheKeyword "Emerge")]))
                   Nothing))
    (Macros.create (Lit 1)
       (Macros.creatureTok 3 2 []
          [creatureType "Eldrazi", creatureType "Horror"]))

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

||| Stone Haven Outfitter
public export
stoneHavenOutfitter : Card
stoneHavenOutfitter =
  Macros.card "Stone Haven Outfitter" (Just [Macros.generic 1, Macros.pip White]) []
       (MkTypeLine [creatureType "Kor", creatureType "Artificer",
                    creatureType "Ally"] [Creature])
       [ Static (Macros.getsPt (Macros.allOf (And [Macros.creatureYouControl, IsAttached Equipped]))
                      (Up (Lit 1)) (Up (Lit 1)))
       , Macros.triggered Whenever
           (Dies (Macros.a (And [Macros.creatureYouControl, IsAttached Equipped])))
           ((Draw You (Lit 1))) ]
       (Just (2, 2))

||| Cloud, Ex-SOLDIER
public export
cloudExSoldierAttach : Ability
cloudExSoldierAttach =
  Macros.triggered When (Enters Macros.thisCreature Nothing)
    (Macros.attachToIt
       (Described (TargetDet (Macros.upTo 1)) (And [HasSubtype (artifactType "Equipment"), HasPossessor ControllerAx You])))

||| Akiri, Fearless Voyager
public export
akiriEquippedAttackers : Ability
akiriEquippedAttackers =
  Macros.triggered Whenever
    (AttacksWith You (OneDefender (Macros.a AnyPlayer))
       (Macros.counted (Macros.atLeast 1)
          (And [Macros.creatureYouControl, IsAttached Equipped])))
    ((Draw You (Lit 1)))

||| Vexing Bauble
public export
vexingBaubleTrigger : Ability
vexingBaubleTrigger =
  Macros.triggeredIf Whenever
    (Casts (Macros.a AnyPlayer) (Macros.a Macros.spell) Nothing)
    (Macros.noManaSpentToCast (Macros.It OneOf))
    (CounterSpell (Macros.That SpellW OneOf))

||| Void Mirror
public export
voidMirror : Card
voidMirror =
  Macros.card "Void Mirror" (Just [Macros.generic 2]) []
       (MkTypeLine [] [Artifact])
       [ Macros.triggeredIf Whenever
           (Casts (Macros.a AnyPlayer) (Macros.a Macros.spell) Nothing)
           (Macros.noColoredManaSpentToCast (Macros.It OneOf))
           (CounterSpell (Macros.That SpellW OneOf)) ]
       Nothing

||| Blood Sun
public export
bloodSun : Card
bloodSun =
  Macros.card "Blood Sun" (Just [Macros.generic 2, Macros.pip Red]) []
       (MkTypeLine [] [Enchantment])
       [ Macros.triggered When (Enters Macros.thisEnchantment Nothing)
                          (Draw You (Lit 1))
       , Static (LosesAllAbilities (Macros.allOf Macros.land) (Just IsManaAbility)) ]
       Nothing

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
           ((IfDone (Macros.sacrifice You Macros.thisEnchantment) (Just (Macros.destroy (Macros.That (TypeW Creature) OneOf))) Nothing)) ]
       Nothing

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
           (Macros.losesLife (Macros.controllerOf (Macros.That (TypeW Creature) OneOf)) (Lit 1)) ]
       Nothing

||| Jeskai Ascendancy's first trigger
public export
jeskaiAscendancyPump : Ability
jeskaiAscendancyPump =
  Macros.triggered Whenever
    (Casts You (Macros.a (And [Macros.spell, Not Macros.creature])) Nothing)
    (Sequentially
       [ Macros.gets (Macros.bare Macros.creatureYouControl)
                     (Up (Lit 1)) (Up (Lit 1)) (Just Macros.untilEndOfTurn)
       , Macros.untap (Macros.That (TypeW Creature) ManyOf) ])

herosDemise : Instruction []
herosDemise =
  Macros.destroy (Macros.target (And [Macros.creature, HasSupertype Legendary]))

repayInKind : Card
repayInKind =
  Macros.card "Repay in Kind"
       (Just [Macros.generic 5, Macros.pip Black, Macros.pip Black]) []
       (MkTypeLine [] [Sorcery])
       [ Spell Nothing (Macros.lifeBecomes (Macros.each AnyPlayer)
                  (Macros.aggregate MinOf (PlayerStatAxis LifeTotal) AnyPlayer)) ]
       Nothing

||| Squelch
public export
squelch : Card
squelch =
  Macros.card "Squelch" (Just [Macros.generic 1, Macros.pip Blue]) []
       (MkTypeLine [] [Instant])
       [ Spell Nothing (Sequentially
                  [ CounterSpell
                      (Macros.target (AbilityHead AnyActivated))
                  , (Draw You (Lit 1)) ]) ]
       Nothing

public export
bioplasmTwoCandidates : countOnes Object Description.bioplasmAfterExile = 2
bioplasmTwoCandidates = Refl
