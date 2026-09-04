module Experimental.Cards.Counters

import Experimental
import Experimental.Macros
import Experimental.Cards.Anaphora
import Experimental.Cards.Keyword

%default total


||| Yawgmoth Demon
crovaxTheCursed : Instruction []
crovaxTheCursed =
  (May You (Macros.sacrifice You (Macros.a Macros.creature)) (Just (PutCounters (Lit 1) (PrintedKind Macros.plusOnePlusOne) Macros.thisCreature)) (Just (RemoveCounters (Just (Macros.exactly 1)) (Just (PrintedKind Macros.plusOnePlusOne)) Macros.thisCreature)))

additiveEvolution : Instruction []
additiveEvolution = Sequentially [Macros.create (Lit 1) (Macros.creatureTok 0 0 [Green, Blue] [creatureType "Fractal"]),
                                  PutCounters (Lit 3) (PrintedKind Macros.plusOnePlusOne) ((Macros.It OneOf))]

battlegrowth : Instruction []
battlegrowth = PutCounters (Lit 1) (PrintedKind Macros.plusOnePlusOne) (Macros.target Macros.creature)

chainbreaker : Instruction []
chainbreaker = RemoveCounters (Just (Macros.exactly 1)) (Just (PrintedKind Macros.minusOneMinusOne)) (Macros.target Macros.creature)

kaitoBaneOfNightmares : Instruction []
kaitoBaneOfNightmares = Sequentially [SetStatus Tapped (Macros.target Macros.creature),
                                      PutCounters (Lit 2) (PrintedKind (NamedCounter "Stun")) ((Macros.It OneOf))]

jhoiraOfTheGhitu : Ability
jhoiraOfTheGhitu =
  Macros.activated (Compound [Mana [Macros.generic 2],
                       Do (Macros.exile You (Macros.a (And [Not Macros.land, InZone (Macros.handOf You)])))])
                   (PutCounters (Lit 4) (PrintedKind (NamedCounter "Time")) (Macros.TheVerbed "Exile" CardW Attributive OneOf))

alaundoTheSeer : Instruction []
alaundoTheSeer = RemoveCounters (Just (Macros.exactly 1)) (Just (PrintedKind (NamedCounter "Time"))) (Macros.each (InZone Macros.exileZ))

daydream : Card
daydream =
  Macros.card "Daydream" (Just [Macros.pip White]) [] (MkTypeLine [] [Sorcery])
       [Spell Nothing (Sequentially [Macros.exile You (Macros.target Macros.creatureYouControl),
                             Macros.returnToBattlefieldWithCounters
                               (Macros.That CardW OneOf) (Macros.ownerOf (Macros.That CardW OneOf))
                               (Lit 1) Macros.plusOnePlusOne])
       , Macros.keywordCosting "Flashback" (Mana [Macros.generic 2, Macros.pip White])]
       Nothing

ashnodsTransmogrant : Ability
ashnodsTransmogrant =
  Macros.activated (Compound [TapSymbol, Do (Macros.sacrifice You Macros.thisArtifact)])
                   (Sequentially [PutCounters (Lit 1) (PrintedKind Macros.plusOnePlusOne)
                                         (Macros.target (And [Macros.creature, Not Macros.artifact])),
                             Macros.becomes (Macros.That (TypeW Creature) OneOf) (Macros.typesOnly [Artifact]) Nothing])

azulaAlwaysLies : Instruction []
azulaAlwaysLies =
  Macros.chooseModes (Macros.oneThrough 2) [Macros.gets (Macros.target Macros.creature) (PtDown (Lit 1)) (PtDown (Lit 1)) (Just Macros.untilEndOfTurn),
                   PutCounters (Lit 1) (PrintedKind Macros.plusOnePlusOne) (Macros.target Macros.creature)]

bellowingAegisaur : Instruction []
bellowingAegisaur =
  PutCounters (Lit 1) (PrintedKind Macros.plusOnePlusOne) (Macros.each (Macros.otherCreatureYouControl Macros.thisCreature))

carnifexDemon : Instruction []
carnifexDemon =
  PutCounters (Lit 1) (PrintedKind Macros.minusOneMinusOne) (Macros.each (Macros.otherCreature Macros.thisCreature))

ajaniAdversaryOfTyrants : Instruction []
ajaniAdversaryOfTyrants =
  PutCounters (Lit 1) (PrintedKind Macros.plusOnePlusOne) (EachOf (Described (TargetDet (Macros.upTo 2)) Macros.creature))

naturesPanoply : Instruction []
naturesPanoply =
  Sequentially [Macros.choose (Described (TargetDet Macros.anyNumber) Macros.creature),
                PutCounters (Lit 1) (PrintedKind Macros.plusOnePlusOne) (EachOf ((Macros.It ManyOf)))]

armamentCorps : Instruction []
armamentCorps =
  Macros.distributeCounters (Lit 2) Macros.plusOnePlusOne
                     (Described (TargetDet (Macros.oneThrough 2)) Macros.creatureYouControl)

savagebornHydra : Ability
savagebornHydra =
  Macros.activatedOnlyDuring (Mana [Macros.generic 1, Macros.hybridPip Red Green])
                             (PutCounters (Lit 1) (PrintedKind Macros.plusOnePlusOne) Macros.thisCreature)
                             AsSorcery

woeleecher : Ability
woeleecher =
  Macros.activated (Compound [Mana [Macros.pip White], TapSymbol])
                   ((IfDone (RemoveCounters (Just (Macros.exactly 1)) (Just (PrintedKind Macros.minusOneMinusOne)) (Macros.target Macros.creature)) (Just (Macros.gainsLife You (Lit 2))) Nothing))

anointerOfValor : Ability
anointerOfValor =
  Macros.triggered Whenever (Macros.attacks (Macros.a Macros.creature))
    (Macros.mayWhen You (Pay You (Mana [Macros.generic 3]) PaidOnce)
                 (PutCounters (Lit 1) (PrintedKind Macros.plusOnePlusOne) (Macros.That (TypeW Creature) OneOf)))

avianOddity : Instruction []
avianOddity = PutCounters (Lit 1) (PrintedKind Macros.flyingCounter) (Macros.target Macros.creatureYouControl)

||| Song of Eärendil
songOfEarendil : Instruction []
songOfEarendil =
  PutCounters (Lit 1) (PrintedKind Macros.flyingCounter)
              (Macros.each (And [Macros.creature, HasPossessor ControllerAx You, Not (HasKeyword (TheKeyword "Flying"))]))

gideonsAvenger : Ability
gideonsAvenger =
  Macros.triggered Whenever
                   (StatusEvent (Macros.a (And [Macros.creature, HasPossessor ControllerAx Macros.anOpponent])) Tapped)
                   (PutCounters (Lit 1) (PrintedKind Macros.plusOnePlusOne) Macros.thisCreature)

prologueToPhyresis : Instruction []
prologueToPhyresis = PutCounters (Lit 1) (PrintedKind (NamedCounter "Poison")) (Macros.each Opponent)

screechingScorchbeast : Ability
screechingScorchbeast =
  Macros.triggered Whenever (Macros.attacks Macros.thisCreature)
                   (PutCounters (Lit 2) (PrintedKind (NamedCounter "Rad")) (Macros.each AnyPlayer))

merenOfClanNelToth : Ability
merenOfClanNelToth =
  Macros.triggered Whenever (Dies (Macros.a (Macros.otherCreatureYouControl Macros.thisCreature)))
                   (PutCounters (Lit 1) (PrintedKind (NamedCounter "Experience")) You)

kratosStoicFather : Ability
kratosStoicFather =
  Macros.triggered At (BeginningOf ThePart EndStep (ByPlayer You))
                   (PutCounters (CountersOn (NamedCounter "Experience") You) (PrintedKind Macros.plusOnePlusOne)
                         (Macros.target Macros.creature))

vashtaNerada : Ability
vashtaNerada =
  Macros.triggeredIf At
                     (BeginningOf ThePart EndStep (ByPlayer (Macros.each AnyPlayer)))
                     (Macros.happened Death (Macros.a Macros.creature)
                                      Lookback.ThisTurn)
                     (PutCounters (Lit 1) (PrintedKind Macros.plusOnePlusOne) Macros.thisCreature)

furiousSpinesplitter : Ability
furiousSpinesplitter =
  Macros.triggered At (BeginningOf ThePart EndStep (ByPlayer You))
                   (PutCounters (Macros.forEach 1 (And [Opponent,
                                               Macros.happenedTo DamageTaken Lookback.ThisTurn]))
                         (PrintedKind Macros.plusOnePlusOne) Macros.thisCreature)

paladinOfAtonement : Ability
paladinOfAtonement =
  Macros.triggeredIf At
                     (BeginningOf ThePart Upkeep (ByPlayer (Macros.each AnyPlayer)))
                     (Macros.happened LifeLoss You Lookback.LastTurn)
                     (PutCounters (Lit 1) (PrintedKind Macros.plusOnePlusOne) Macros.thisCreature)

throneWarden : Ability
throneWarden =
  Macros.triggeredIf At
                     (BeginningOf ThePart EndStep (ByPlayer You))
                     (Matches You (HasDesignation Monarch))
                     (PutCounters (Lit 1) (PrintedKind Macros.plusOnePlusOne) Macros.thisCreature)

bloodTyrant : Ability
bloodTyrant =
  Macros.triggered Whenever (LosesGame (Macros.a AnyPlayer))
                   (PutCounters (Lit 5) (PrintedKind Macros.plusOnePlusOne) Macros.thisCreature)

stagBeetle : Card
stagBeetle =
  Macros.card "Stag Beetle"
       (Just [Macros.generic 3, Macros.pip Green, Macros.pip Green]) []
       (MkTypeLine [creatureType "Insect"] [Creature])
       [ Static (AndAlso Nothing [ Macros.entersWithCounters Macros.thisCreature
                                                     (LetterVal X)
                                                     Macros.plusOnePlusOne
                         , DefinesLetter X
                             (Macros.countOf (Macros.otherCreature Macros.thisCreature)) ]) ]
       (Just (0, 0))

toweringTitan : Ability
toweringTitan =
  Static (AndAlso Nothing [ Macros.entersWithCounters Macros.thisCreature
                                              (LetterVal X)
                                              Macros.plusOnePlusOne
                  , DefinesLetter X
                      (Macros.aggregate SumOf (StatAxis Toughness)
                         (Macros.otherCreatureYouControl Macros.thisCreature)) ])

coretapper : Card
coretapper =
  Macros.card "Coretapper" (Just [Macros.generic 2]) []
       (MkTypeLine [creatureType "Myr"] [Artifact, Creature])
       [ Macros.activated TapSymbol
                          (PutCounters (Lit 1) (PrintedKind (NamedCounter "Charge")) (Macros.target Macros.artifact))
       , Macros.activated (Do (Macros.sacrifice You Macros.thisCreature))
                          (PutCounters (Lit 2) (PrintedKind (NamedCounter "Charge")) (Macros.target Macros.artifact)) ]
       (Just (1, 1))

divineIntervention : Card
divineIntervention =
  Macros.card "Divine Intervention"
       (Just [Macros.generic 6, Macros.pip White, Macros.pip White]) []
       (MkTypeLine [] [Enchantment])
       [ Static (Macros.entersWithCounters Macros.thisEnchantment (Lit 2) (NamedCounter "Intervention"))
       , Macros.triggered At (BeginningOf ThePart Upkeep (ByPlayer You))
                          (RemoveCounters (Just (Macros.exactly 1)) (Just (PrintedKind (NamedCounter "Intervention"))) Macros.thisEnchantment)
       , Macros.triggered When (Macros.lastCounterRemovedBy (NamedCounter "Intervention") Macros.thisEnchantment You)
                          GameDrawn ]
       Nothing

celestialConvergence : Card
celestialConvergence =
  Macros.card "Celestial Convergence"
       (Just [Macros.generic 2, Macros.pip White, Macros.pip White]) []
       (MkTypeLine [] [Enchantment])
       [ Static (Macros.entersWithCounters Macros.thisEnchantment (Lit 7) (NamedCounter "Omen"))
       , Macros.triggered At (BeginningOf ThePart Upkeep (ByPlayer You))
           (Sequentially
              [ RemoveCounters (Just (Macros.exactly 1)) (Just (PrintedKind (NamedCounter "Omen"))) Macros.thisEnchantment
              , If (CompareAmt (CountersOn (NamedCounter "Omen") Macros.thisEnchantment)
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

curseOfVengeance : Card
curseOfVengeance =
  Macros.card "Curse of Vengeance" (Just [Macros.pip Black]) []
       (MkTypeLine [enchantmentType "Aura", enchantmentType "Curse"] [Enchantment])
       [ Macros.keywordSubject "Enchant" AnyPlayer
       , Macros.triggered Whenever
                          (Casts (AttachHost Enchanted PlayerW) (Macros.a Macros.spell) Nothing)
                          (PutCounters (Lit 1) (PrintedKind (NamedCounter "Spite")) Macros.thisAura)
       , Macros.triggered When (LosesGame (AttachHost Enchanted PlayerW))
                          (Sequentially
                             [ Macros.gainsLife You (LetterVal X)
                             , Draw You (LetterVal X)
                             , Define X (CountersOn (NamedCounter "Spite") Macros.thisAura) ]) ]
       Nothing

passagewaySeer : Card
passagewaySeer =
  Macros.card "Passageway Seer" (Just [Macros.generic 3, Macros.pip Black]) []
       (MkTypeLine [creatureType "Tiefling", creatureType "Warlock"] [Creature])
       [ Macros.keyword "Lifelink"
       , Macros.triggered When (Enters Macros.thisCreature Nothing)
                          (Macros.gainsDesignation You TheInitiative Instructed)
       , Macros.triggeredIf At
                            (BeginningOf ThePart EndStep (ByPlayer You))
                            (Matches You (HasDesignation TheInitiative))
                            (PutCounters (Lit 1) (PrintedKind Macros.plusOnePlusOne) Macros.thisCreature) ]
       (Just (2, 2))

chainsaw : Card
chainsaw =
  Macros.card "Chainsaw" (Just [Macros.generic 1, Macros.pip Red]) []
       (MkTypeLine [artifactType "Equipment"] [Artifact])
       [ Macros.triggered When (Enters Macros.thisEquipment Nothing)
                          (DealDamage ((Macros.It OneOf)) (Lit 3)
                               (Described (TargetDet (Macros.upTo 1)) Macros.creature))
       , Macros.triggered Whenever
                          (Dies (Macros.counted (Macros.atLeast 1) Macros.creature))
                          (PutCounters (Lit 1) (PrintedKind (NamedCounter "Rev")) Macros.thisEquipment)
       , Static (AndAlso Nothing [ Gets Adds (AttachHost Equipped (TypeW Creature))
                                (PtUp (LetterVal X)) (PtUp (Lit 0))
                         , DefinesLetter X (CountersOn (NamedCounter "Rev") Macros.thisEquipment) ])
       , Macros.keywordCosting "Equip" (Mana [Macros.generic 3]) ]
       Nothing

||| Soul's Might
soulsMight : Card
soulsMight =
  Macros.card "Soul's Might" (Just [Macros.generic 4, Macros.pip Green]) []
       (MkTypeLine [] [Sorcery])
       [ Spell Nothing (Sequentially
                  [ PutCounters (LetterVal X) (PrintedKind Macros.plusOnePlusOne)
                                (Macros.target Macros.creature)
                  , Define X (StatOf Power (Macros.That (TypeW Creature) OneOf)) ]) ]
       Nothing

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

voraciousBrood : Card
voraciousBrood =
  Macros.card "Voracious Brood"
       (Just [Macros.generic 2, Macros.pip Green]) []
       (MkTypeLine [creatureType "Alien", creatureType "Insect"] [Creature])
       [ Static (Macros.entersWithCounters Macros.thisCreature
                   (Macros.forEach 1 (And [Macros.creature,
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
                          (Macros.counterEvent CounterPut Macros.plusOnePlusOne ManyCounters
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
                          (Macros.counterEvent CounterPut
                                              Macros.minusOneMinusOne OneCounter
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

hardenedScales : Card
hardenedScales =
  Macros.card "Hardened Scales" (Just [Macros.pip Green]) []
       (MkTypeLine [] [Enchantment])
       [ Static (Intercepts
                   (Macros.counterEvent CounterPut Macros.plusOnePlusOne ManyCounters
                                            (Macros.a Macros.creatureYouControl)) [] Nothing
                   (PutCounters (Plus ThatMuch (Lit 1))
                                (PrintedKind Macros.plusOnePlusOne) ((Macros.It OneOf)))
                   Repeatedly Nothing) ]
       Nothing

branchingEvolution : Card
branchingEvolution =
  Macros.card "Branching Evolution"
       (Just [Macros.generic 2, Macros.pip Green]) []
       (MkTypeLine [] [Enchantment])
       [ Static (Intercepts
                   (Macros.counterEvent CounterPut Macros.plusOnePlusOne ManyCounters
                                            (Macros.a Macros.creatureYouControl)) [] Nothing
                   (PutCounters (Macros.times 2 ThatMuch)
                                (PrintedKind Macros.plusOnePlusOne) (Macros.That (TypeW Creature) OneOf))
                   Repeatedly Nothing) ]
       Nothing

||| Corpsejack Menace
corpsejackMenace : Ability
corpsejackMenace =
  Static (Intercepts
            (Macros.counterEvent CounterPut Macros.plusOnePlusOne ManyCounters
                                     (Macros.a Macros.creatureYouControl)) [] Nothing
            (PutCounters (Macros.times 2 ThatMuch) (PrintedKind Macros.plusOnePlusOne) ((Macros.It OneOf)))
            Repeatedly Nothing)

||| Doubling Season
doublingSeason : Card
doublingSeason =
  Macros.card "Doubling Season" (Just [Macros.generic 4, Macros.pip Green]) []
       (MkTypeLine [] [Enchantment])
       [ Static (Intercepts
                   (Macros.tokensCreatedByEffectUnder
                      (Macros.counted (Macros.atLeast 1) IsToken) You) [] Nothing
                   (Create You (Macros.times 2 GroupSize) TokenAsThose [])
                   Repeatedly Nothing)
       , Static (Intercepts
                   (Macros.manyCountersPutByEffect
                      (Macros.a (And [Permanent, HasPossessor ControllerAx You]))) [] Nothing
                   (PutCounters (Macros.times 2 ThatMuch) ThoseKinds (Macros.That PermanentW OneOf))
                   Repeatedly Nothing) ]
       Nothing

||| Doc Samson, Super Psychiatrist
docSamsonDistributive : Ability
docSamsonDistributive =
  Static (Intercepts
            (Macros.manyBareCountersPutBy You
               (Macros.a (And [Permanent, HasPossessor ControllerAx You]))) [] Nothing
            (PutCounters (Plus ThatMuch (Lit 1)) ThoseKinds (Macros.That PermanentW OneOf))
            Repeatedly Nothing)

||| Winding Constrictor
windingConstrictor : Card
windingConstrictor =
  Macros.card "Winding Constrictor" (Just [Macros.pip Black, Macros.pip Green]) []
       (MkTypeLine [creatureType "Snake"] [Creature])
       [ Static (Intercepts
                   (Macros.bareCounterEvent CounterPut ManyCounters
                      (Macros.a (And [Or [Macros.artifact, Macros.creature],
                                      HasPossessor ControllerAx You]))) [] Nothing
                   (PutCounters (Plus ThatMuch (Lit 1))
                                ThoseKinds (Macros.That PermanentW OneOf))
                   Repeatedly Nothing)
       , Static (Intercepts
                   (Macros.bareCounterEvent CounterPut ManyCounters You) [] Nothing
                   (PutCounters (Plus ThatMuch (Lit 1)) ThoseKinds You)
                   Repeatedly Nothing) ]
       (Just (2, 3))

||| Aragorn, Company Leader
aragornDistributive : Ability
aragornDistributive =
  Macros.triggered Whenever
    (Macros.manyBareCountersPutBy You Macros.thisCreature)
    (PutCounters (Lit 1) ThoseKinds
       (Described (TargetDet (Macros.upTo 1)) (Macros.otherCreature Macros.thisCreature)))

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
    (PutCounters (Lit 1) (SameAs Macros.thisCreature) (Macros.That (TypeW Creature) OneOf))

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

||| Me, the Immortal
meTheImmortalCounterMenu : Ability
meTheImmortalCounterMenu =
  Macros.triggered At (BeginningOf ThePart Combat (ByPlayer You))
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
                          (PutCounters (Lit 1) (SameAs ((Macros.It OneOf)))
                             (Macros.target Macros.creatureYouControl)) ]
       (Just (0, 0))

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
                                        HappenedTo (MkLookback Entry Lookback.ThisTurn Nothing)])) ]) ]
       (Just (2, 3))

||| Runadi, Behemoth Caller
runadiBehemothCaller : Ability
runadiBehemothCaller =
  Static (Gains (Macros.allOf (And [Macros.creature, HasPossessor ControllerAx You,
                             Compare [CounterAxis Macros.plusOnePlusOne]
                                            AtLeast (Lit 3)]))
                (Macros.keyword "Haste"))

||| Boon of Safety
boonOfSafetyPut : Instruction []
boonOfSafetyPut = PutCounters (Lit 1) (PrintedKind (NamedCounter "Shield")) (Macros.target Macros.creature)

||| Vivien's Talent and Teferi's Talent
talentLoyaltyPut : Instruction []
talentLoyaltyPut =
  PutCounters (Lit 1) (PrintedKind (NamedCounter "Loyalty"))
              (AttachHost Enchanted (TypeW Planeswalker))

||| Simic Fluxmage
simicFluxmageMove : Instruction []
simicFluxmageMove =
  MoveCounters (Lit 1) (Just (PrintedKind Macros.plusOnePlusOne)) Macros.thisCreature
               (Macros.target Macros.creature)

||| Rikku, Resourceful Guardian
rikkuStealMove : Instruction []
rikkuStealMove =
  MoveCounters (Lit 1) Nothing
               (Macros.target (And [Macros.creature,
                                    HasPossessor ControllerAx Macros.anOpponent]))
               (Macros.target Macros.creatureYouControl)

||| Littjara Mirrorlake
littjaraMirrorlakeCopy : Instruction []
littjaraMirrorlakeCopy =
  Create You (Lit 1)
         (TokenCopyOf (Macros.target (And [Macros.creature, HasPossessor ControllerAx You]))
                      [ExceptEntersWithCounters (Lit 1) Macros.plusOnePlusOne
                                                Additional])
         []

shalaiAndHallar : Card
shalaiAndHallar =
  Macros.card "Shalai and Hallar"
       (Just [Macros.generic 1, Macros.pip Red, Macros.pip Green, Macros.pip White])
       [Legendary]
       (MkTypeLine [creatureType "Angel", creatureType "Elf"] [Creature])
       [ Macros.keyword "Flying"
       , Macros.keyword "Vigilance"
       , Macros.triggered Whenever
                          (Macros.counterEvent CounterPut Macros.plusOnePlusOne ManyCounters
                                            (Macros.a Macros.creatureYouControl))
                          (DealDamage Macros.thisCreature ThatMuch
                               (Macros.target Opponent)) ]
       (Just (3, 3))

primalVigor : Card
primalVigor =
  Macros.card "Primal Vigor"
       (Just [Macros.generic 4, Macros.pip Green]) []
       (MkTypeLine [] [Enchantment])
       [ Static (Intercepts
                   (Macros.tokensCreated (Macros.counted (Macros.atLeast 1) IsToken)) [] Nothing
                   (Create You (Macros.times 2 GroupSize) TokenAsThose [])
                   Repeatedly Nothing)
       , Static (Intercepts
                   (Macros.counterEvent CounterPut Macros.plusOnePlusOne ManyCounters
                                            (Macros.a Macros.creature)) [] Nothing
                   (PutCounters (Macros.times 2 ThatMuch)
                                (PrintedKind Macros.plusOnePlusOne) (Macros.That (TypeW Creature) OneOf))
                   Repeatedly Nothing) ]
       Nothing

||| Patrolling Peacemaker
public export
patrollingPeacemaker : Card
patrollingPeacemaker =
  Macros.card "Patrolling Peacemaker"
       (Just [Macros.generic 2, Macros.pip White]) []
       (MkTypeLine [creatureType "Robot", creatureType "Soldier"]
                   [Artifact, Creature])
       [ Static (Macros.entersWithCounters Macros.thisCreature (Lit 2)
                                           Macros.plusOnePlusOne)
       , Macros.triggered Whenever (CommitsCrime Macros.anOpponent)
                          Macros.proliferate ]
       (Just (0, 0))

||| Galloping Lizrog
public export
gallopingLizrog : Card
gallopingLizrog =
  Macros.card "Galloping Lizrog"
       (Just [Macros.generic 3, Macros.pip Green, Macros.pip Blue]) []
       (MkTypeLine [creatureType "Frog", creatureType "Lizard"] [Creature])
       [ Macros.keyword "Trample"
       , Macros.triggered When (Enters Macros.thisCreature Nothing)
           ((May You (RemoveCounters (Just Macros.anyNumber) (Just (PrintedKind Macros.plusOnePlusOne))
                                   (Macros.among (Macros.allOf Macros.creatureYouControl))) (Just (PutCounters (Macros.times 2 Macros.removedThisWay)
                           (PrintedKind Macros.plusOnePlusOne)
                           Macros.thisCreature)) Nothing)) ]
       (Just (3, 3))

||| Novijen Sages
public export
novijenSagesDraw : Ability
novijenSagesDraw =
  Macros.activated (Compound [Mana [Macros.generic 1],
                       Do (RemoveCounters (Just (Macros.exactly 2))
                             (Just (PrintedKind Macros.plusOnePlusOne))
                             (Macros.among (Macros.allOf Macros.creatureYouControl)))])
                   ((Draw You (Lit 1)))

||| Cyclone
public export
cycloneUpkeepPayment : Ability
cycloneUpkeepPayment =
  Macros.triggered At (BeginningOf ThePart Upkeep (ByPlayer You))
    (Sequentially
       [ PutCounters (Lit 1) (PrintedKind (NamedCounter "Wind")) Macros.thisEnchantment
       , (May You (Pay You (Macros.scaledMana (RunUnit [Macros.pip Green])
                                (Macros.times 1 (CountersOn (NamedCounter "Wind") Macros.thisEnchantment)))
                PaidOnce) Nothing (Just (Macros.sacrifice You Macros.thisEnchantment))) ])

public export
stormwildCapridor : Card
stormwildCapridor =
  Macros.card "Stormwild Capridor" (Just [Macros.generic 2, Macros.pip White]) []
       (MkTypeLine [creatureType "Bird", creatureType "Goat"] [Creature])
       [ Macros.keyword "Flying"
       , Static (DamageRule NoncombatOnly Unattributed (ToRecipient Macros.thisCreature) (Prevent CutAll (Just (PutCounters Macros.preventedThisWay
                                                 (PrintedKind Macros.plusOnePlusOne)
                                                 Macros.thisCreature))) Repeatedly) ]
       (Just (1, 3))

public export
testOfFaith : Card
testOfFaith =
  Macros.card "Test of Faith" (Just [Macros.generic 1, Macros.pip White]) []
       (MkTypeLine [] [Instant])
       [ Spell Nothing (Continuously
                  (DamageRule AnyDamage Unattributed (ToRecipient (Macros.target Macros.creature)) (Prevent (Shield (Lit 3)) (Just (PutCounters Macros.preventedThisWay
                                               (PrintedKind Macros.plusOnePlusOne)
                                               (Macros.That (TypeW Creature) OneOf)))) Repeatedly)
                  (Just ThisTurn)) ]
       Nothing

public export
temper : Card
temper =
  Macros.card "Temper" (Just [Variable, Macros.generic 1, Macros.pip White]) []
       (MkTypeLine [] [Instant])
       [ Spell Nothing (Continuously
                  (DamageRule AnyDamage Unattributed (ToRecipient (Macros.target Macros.creature)) (Prevent (Shield (LetterVal X)) (Just (PutCounters Macros.preventedThisWay
                                               (PrintedKind Macros.plusOnePlusOne)
                                               (Macros.That (TypeW Creature) OneOf)))) Repeatedly)
                  (Just ThisTurn)) ]
       Nothing

public export
phytohydra : Card
phytohydra =
  Macros.card "Phytohydra"
       (Just [Macros.generic 2, Macros.pip Green, Macros.pip White,
              Macros.pip White]) []
       (MkTypeLine [creatureType "Plant", creatureType "Hydra"] [Creature])
       [ Static (Intercepts (IsDealtDamage AnyDamage Macros.thisCreature) [] Nothing
                            (PutCounters ThatMuch
                                         (PrintedKind Macros.plusOnePlusOne) ((Macros.It OneOf)))
                            Repeatedly Nothing) ]
       (Just (1, 1))

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

||| Chromatic Armor
public export
chromaticArmor : Card
chromaticArmor =
  Macros.card "Chromatic Armor"
       (Just [Macros.generic 1, Macros.pip White, Macros.pip Blue]) []
       (MkTypeLine [enchantmentType "Aura"] [Enchantment])
       [ Macros.keywordSubject "Enchant" Macros.creature
       , Static (Macros.entersChoosing Macros.thisAura Color)
       , Static (Macros.entersWithCounters Macros.thisAura (Lit 1) (NamedCounter "Sleight"))
       , Static (DamageRule AnyDamage (DealtBy (Macros.allOf (And [Macros.source, Macros.ofTheLastChosen Color]))) (ToRecipient (AttachHost Enchanted (TypeW Creature))) (Prevent CutAll Nothing) Repeatedly)
       , Macros.activated (Mana [Variable])
           (Sequentially
              [ PutCounters (Lit 1) (PrintedKind (NamedCounter "Sleight")) Macros.thisAura
              , Macros.choose (Macros.a (Macros.quality Color))
              , Define X (CountersOn (NamedCounter "Sleight") Macros.thisAura) ]) ]
       Nothing

public export
vault75MiddleSchool : Card
vault75MiddleSchool =
  Macros.card "Vault 75: Middle School"
       (Just [Macros.generic 2, Macros.pip White, Macros.pip White]) []
       (MkTypeLine [enchantmentType "Saga"] [Enchantment])
       [ Macros.triggered When (ChapterMark [ChapterI])
           (Macros.exile You (Macros.allOf (And [Macros.creature,
                                      Compare [StatAxis Power] AtLeast (Lit 4)])))
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
           (PutCounters (Lit 1) (PrintedKind (NamedCounter "Lore"))
                        (Macros.target (And [HasSubtype (enchantmentType "Saga"), HasPossessor ControllerAx You]))) ]
       (Just (2, 2))

public export
magistratesScepter : Card
magistratesScepter =
  Macros.card "Magistrate's Scepter" (Just [Macros.generic 3]) []
       (MkTypeLine [] [Artifact])
       [ Macros.activated (Compound [Mana [Macros.generic 4], TapSymbol])
                          (PutCounters (Lit 1) (PrintedKind (NamedCounter "Charge")) Macros.thisArtifact)
       , Macros.activated (Compound [TapSymbol,
                              Do (RemoveCounters (Just (Macros.exactly 3)) (Just (PrintedKind (NamedCounter "Charge"))) Macros.thisArtifact)])
                          (ExtraTurn You (Lit 1)) ] Nothing

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
metallicMimic : Card
metallicMimic =
  Macros.card "Metallic Mimic" (Just [Macros.generic 2]) []
       (MkTypeLine [creatureType "Shapeshifter"] [Artifact, Creature])
       [ Static (Macros.entersChoosing Macros.thisCreature (SubtypeQ Creature))
       , Static (Becomes Macros.thisCreature Adds (ChosenQuality (Macros.ofChosen (SubtypeQ Creature))))
       , Static (Macros.entersWithAdditionalCounters (Macros.each (And [Macros.creature, HasPossessor ControllerAx You,
                                                                 Macros.ofChosen (SubtypeQ Creature),
                                                                 OtherThan Macros.thisCreature]))
                                                     (Lit 1)
                                                     Macros.plusOnePlusOne) ]
       (Just (2, 1))

public export
curatorBeastieLine : StaticSpec []
curatorBeastieLine =
  Macros.entersWithAdditionalCounters (Macros.allOf (And [Macros.creature, HasPossessor ControllerAx You, IsColorless]))
                                      (Lit 2)
                                      Macros.plusOnePlusOne

public export
renataLine : StaticSpec []
renataLine =
  Macros.entersWithAdditionalCounters (Macros.each (And [Macros.creature, HasPossessor ControllerAx You,
                                                  OtherThan Macros.thisCreature]))
                                      (Lit 1)
                                      Macros.plusOnePlusOne

public export
entDraughtBasinAbility : Ability
entDraughtBasinAbility =
  Macros.activated (Compound [Mana [Variable], TapSymbol])
                   (PutCounters (Lit 1) (PrintedKind Macros.plusOnePlusOne)
               (Macros.target (And [Macros.creatureYouControl,
                                    Compare [StatAxis Power] Eq (LetterVal X)])))

public export
matopiGolem : Card
matopiGolem =
  Macros.card "Matopi Golem" (Just [Macros.generic 5]) []
       (MkTypeLine [creatureType "Golem"] [Artifact, Creature])
       [ Macros.activated (Mana [Macros.generic 1])
                          (ThisWay (Regenerate Macros.thisCreature)
                            (Regenerates Macros.thisCreature)
                            (PutCounters (Lit 1) (PrintedKind Macros.minusOneMinusOne) ((Macros.It OneOf)))) ]
       (Just (3, 3))

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
                          ((Macros.discard (Macros.That PlayerW OneOf) (Macros.a (InZone Macros.handZ)))) ]
       (Just (1, 1))

public export
crumblingAshes : Card
crumblingAshes =
  Macros.card "Crumbling Ashes"
       (Just [Macros.generic 1, Macros.pip Black]) []
       (MkTypeLine [] [Enchantment])
       [ Macros.triggered At (BeginningOf ThePart Upkeep (ByPlayer You))
                          (Macros.destroy
                      (Macros.target
                         (And [Macros.creature,
                               HasCounters (Just Macros.minusOneMinusOne)]))) ]
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
coverOfWinterPut : Ability
coverOfWinterPut =
  Macros.activated (Mana [SnowMana])
                   (PutCounters (Lit 1) (PrintedKind (NamedCounter "Age")) Macros.thisEnchantment)

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
           (Static (Macros.onlyWhile
                      (Gains Macros.thisCreature (Macros.keyword "Flying"))
                      (Macros.exists (And [ExiledWith Macros.thisCreature, HasKeyword (TheKeyword "Flying")]))))
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
mildManneredLibrarian : Card
mildManneredLibrarian =
  Macros.card "Mild-Mannered Librarian" (Just [Macros.pip Green]) []
       (MkTypeLine [creatureType "Human"] [Creature])
       [ Macros.activatedOnlyOnce (Mana [Macros.generic 3, Macros.pip Green])
                                  (Sequentially
                                     [ Continuously (Becomes Macros.thisCreature Sets (Bundle (MkToken Nothing [] (Macros.subtypesOnly [creatureType "Werewolf"]) [] Nothing) Nothing)) Nothing
                                     , PutCounters (Lit 2) (PrintedKind Macros.plusOnePlusOne) ((Macros.It OneOf))
                                     , (Draw You (Lit 1)) ])
                                  OncePerGame ]
       (Just (1, 1))

public export
infernalVessel : Card
infernalVessel =
  Macros.card "Infernal Vessel" (Just [Macros.generic 2, Macros.pip Black]) []
       (MkTypeLine [creatureType "Human", creatureType "Cleric"] [Creature])
       [ Macros.triggeredIf When
                            (Dies Macros.thisCreature)
                            (Macros.itIsntA (HasSubtype (creatureType "Demon")))
                            (Sequentially [Macros.returnToBattlefieldWithCounters ((Macros.It OneOf))
                                                                                  (Macros.ownerOf ((Macros.It OneOf)))
                                                                                  (Lit 2)
                                                                                  Macros.plusOnePlusOne,
                                           Macros.becomesAs ((Macros.It OneOf))
                                                            (MkToken Nothing [] (MkTypeLine [creatureType "Demon"] []) [] Nothing)
                                                            Nothing]) ]
       (Just (2, 1))

||| Vivien's Talent
public export
viviensTalentTrigger : Ability
viviensTalentTrigger =
  Macros.triggered Whenever (Enters (Macros.a (And [Macros.nontoken,
                                             Macros.creatureYouControl])) Nothing)
    (PutCounters (Lit 1) (PrintedKind (NamedCounter "Loyalty"))
                 (AttachHost Enchanted (TypeW Planeswalker)))

public export
counterOnGraveyardCard : Instruction []
counterOnGraveyardCard =
  PutCounters (Lit 1) (PrintedKind Macros.plusOnePlusOne)
              (Macros.target (And [Macros.creature,
                                   InZone (Macros.graveyardOf You)]))

||| Bioessence Hydra
public export
bioessenceHydraTrigger : Ability
bioessenceHydraTrigger =
  Macros.triggered Whenever
    (Macros.counterEvent CounterPut (NamedCounter "Loyalty") ManyCounters
                             (Macros.allOf (And [HasType Planeswalker, HasPossessor ControllerAx You])))
    (PutCounters ThatMuch (PrintedKind Macros.plusOnePlusOne) Macros.thisCreature)

||| Vraska, Betrayal's Sting
public export
vraskaBetrayalsStingUltimate : Instruction []
vraskaBetrayalsStingUltimate =
  If (CompareAmt (CountersOn (NamedCounter "Poison") (Macros.target AnyPlayer)) Less (Lit 9))
     (PutCounters TheDifference (PrintedKind (NamedCounter "Poison")) They)
     Nothing

||| Vexing Puzzlebox
public export
vexingPuzzleboxCounters : Ability
vexingPuzzleboxCounters =
  Macros.triggered Whenever (RollsDice You ManyDice AnyDie AnyResult)
                   (PutCounters (TheOutcome RollResult) (PrintedKind (NamedCounter "Charge")) Macros.thisArtifact)

public export
spaceFamilyGoblinsonRoll : Ability
spaceFamilyGoblinsonRoll =
  Macros.triggered Whenever (RollsDice You OneDie AnyDie AnyResult)
                   (PutCounters (Lit 1) (PrintedKind Macros.plusOnePlusOne)
                                Macros.thisCreature)

||| Atomwheel Acrobats
public export
atomwheelAcrobatsRoll : Ability
atomwheelAcrobatsRoll =
  Macros.triggered Whenever (Macros.youRollResultIn (Range (Just 1) (Just 2)))
                   (PutCounters ThatMuch (PrintedKind Macros.plusOnePlusOne)
                                Macros.thisCreature)

||| Resolute Veggiesaur
public export
resoluteVeggiesaurThirdDie : Ability
resoluteVeggiesaurThirdDie =
  Macros.triggered Whenever
    (NthOccurrence (Nth 3) (Just Turn) (RollsDice You OneDie AnyDie AnyResult))
    (PutCounters (Lit 1) (PrintedKind Macros.plusOnePlusOne)
                 Macros.thisCreature)

||| Run the Play (Striding Shotcaller's other half)
public export
runThePlayCounters : Instruction []
runThePlayCounters =
  PutCounters (Lit 1) (PrintedKind Macros.plusOnePlusOne)
              (EachOf (Described (TargetDet (UpToOf (LetterVal X))) Macros.creature))

||| Font of Agonies
public export
fontOfAgoniesTrigger : Ability
fontOfAgoniesTrigger =
  Macros.triggered Whenever (PaysLife You)
    (PutCounters ThatMuch (PrintedKind (NamedCounter "Blood")) Macros.thisEnchantment)

||| Heart of Kiran
public export
heartOfKiranCrewAltCost : Ability
heartOfKiranCrewAltCost =
  Static (AltCost (Macros.allOf (And [AbilityHead (KeywordClass "Crew"),
                               AbilityOf This]))
            (Just (Do (RemoveCounters (Just (Macros.exactly 1))
                                      (Just (PrintedKind (NamedCounter "Loyalty")))
                                      (Macros.a (And [HasType Planeswalker,
                                                      HasPossessor ControllerAx You]))))))

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

||| Mirelurk Queen
public export
mirelurkQueenTrigger : Ability
mirelurkQueenTrigger =
  Macros.triggeredOnlyOnce Whenever
    (VerbedEvent Nothing "Mill"
                 (Just (Macros.counted (Macros.atLeast 1)
                                     (And [Not Macros.land,
                                           InZone Macros.libraryZ])))
                 Nothing False)
    OncePerTurn
    (Sequentially [ (Draw You (Lit 1))
                  , PutCounters (Lit 1) (PrintedKind Macros.plusOnePlusOne)
                                Macros.thisCreature ])

||| Whirling Dervish
public export
whirlingDervish : Card
whirlingDervish =
  Macros.card "Whirling Dervish" (Just [Macros.pip Green, Macros.pip Green]) []
       (MkTypeLine [creatureType "Human", creatureType "Monk"] [Creature])
       [ Macros.keywordQuality "Protection" (ColorIs Black)
       , Macros.triggeredIf At
           (BeginningOf ThePart EndStep (ByPlayer (Macros.each AnyPlayer)))
           (Macros.happenedInvolving DamageDealing Macros.thisCreature
                                     Lookback.ThisTurn Macros.anOpponent)
           (PutCounters (Lit 1) (PrintedKind Macros.plusOnePlusOne) ((Macros.It OneOf))) ]
       (Just (1, 1))

||| Ichor Shade
public export
ichorShade : Card
ichorShade =
  Macros.card "Ichor Shade"
       (Just [Macros.generic 2, Macros.pip Black]) []
       (MkTypeLine [creatureType "Phyrexian", creatureType "Shade"] [Creature])
       [ Macros.triggeredIf At (BeginningOf ThePart EndStep (ByPlayer You))
           (Happened (Macros.a (Or [Macros.artifact, Macros.creature])) (MkLookback Placement Lookback.ThisTurn (Just (IntoZone Macros.graveyardZ
                              (Just (FromZones
                                       (FromZone [Macros.battlefieldZ])
                                       Nothing))))))
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
       , Macros.triggered At (BeginningOf ThePart EndStep (ByPlayer (Macros.each AnyPlayer)))
           (PutCounters (EventTally TallyCount (Macros.a Macros.creature) (MkLookback Placement Lookback.ThisTurn (Just (IntoZone (Macros.graveyardOf You)
                                    (Just (FromZones
                                             (FromZone [Macros.battlefieldZ])
                                             Nothing))))))
                        (PrintedKind Macros.plusOnePlusOne)
                        Macros.thisCreature) ]
       (Just (2, 3))

||| Thought Sponge
public export
thoughtSponge : Card
thoughtSponge =
  Macros.card "Thought Sponge" (Just [Macros.generic 3, Macros.pip Blue]) []
       (MkTypeLine [creatureType "Sponge"] [Creature])
       [ Macros.keyword "Flash"
       , Static (Macros.entersWithCounters Macros.thisCreature
                   Anaphora.greatestCardsAnOpponentDrew Macros.plusOnePlusOne)
       , Macros.triggered When (Dies Macros.thisCreature)
                          (Draw You (StatOf Power Macros.thisCreature)) ]
       (Just (1, 1))

||| Skeleton Ship
public export
skeletonShip : Card
skeletonShip =
  Macros.card "Skeleton Ship"
       (Just [Macros.generic 3, Macros.pip Blue, Macros.pip Black]) [Legendary]
       (MkTypeLine [creatureType "Skeleton"] [Creature])
       [ Macros.triggered When
           (StateHolds
              (NotCond (Macros.exists (And [Macros.land,
                                     HasSubtype (landType "Island"),
                                     HasPossessor ControllerAx You]))))
           (Macros.sacrifice You Macros.thisCreature)
       , Macros.activated TapSymbol
           (PutCounters (Lit 1) (PrintedKind Macros.minusOneMinusOne)
                        (Macros.target Macros.creature)) ]
       (Just (0, 3))

||| Contractual Safeguard
public export
contractualSafeguardPass : Instruction []
contractualSafeguardPass =
  Sequentially
    [ Macros.choose (Macros.a (CounterKindOn (Macros.a Macros.creatureYouControl)))
    , PutCounters (Lit 1) BoundKind
        (Macros.each (Macros.otherCreatureYouControl ((Macros.It OneOf)))) ]

||| Feral Contest
public export
feralContest : Card
feralContest =
  Macros.card "Feral Contest" (Just [Macros.generic 3, Macros.pip Green]) []
       (MkTypeLine [] [Sorcery])
       [ Spell Nothing (Sequentially
                  [ PutCounters (Lit 1) (PrintedKind Macros.plusOnePlusOne)
                                (Macros.target Macros.creatureYouControl)
                  , Macros.mustBlockIt
                      (Macros.target (And [Macros.creature, Other]))
                      (Just ThisTurn) ]) ]
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

||| Stunning Shot
public export
stunningShot : Card
stunningShot =
  Macros.card "Stunning Shot" (Just [Macros.generic 1, Macros.pip White]) []
       (MkTypeLine [] [Sorcery])
       [ Spell Nothing (Sequentially
                  [ PutCounters (Lit 2) (PrintedKind Macros.plusOnePlusOne)
                                (Described (TargetDet (Macros.upTo 1)) Macros.creatureYouControl)
                  , Macros.tap (Described (TargetDet (Macros.upTo 1)) (And [Macros.creature, HasPossessor ControllerAx Macros.anOpponent]))
                  , PutCounters (Lit 1) (PrintedKind (NamedCounter "Stun"))
                                (Macros.itPrior
                                   (Macros.tap (Described (TargetDet (Macros.upTo 1)) (And [Macros.creature,
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
                         ManyCounters (Just You) False)
           (Matches ((Macros.It OneOf)) (Not (HasSubtype (creatureType "Kree"))))
           (Macros.may You (PutCounters ThatMuch ThoseKinds This)) ]
       (Just (4, 4))

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
              , PutCounters (Lit 1) (PrintedKind (NamedCounter "Bloodstain")) Macros.thisEnchantment
              , Reflexively
                  (OnlyIf (Macros.sacrifice You Macros.thisEnchantment)
                          (CompareAmt (CountersOn (NamedCounter "Bloodstain") Macros.thisEnchantment)
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
agentOfTheShadowThievesGrantedTrigger : AbilityAt []
agentOfTheShadowThievesGrantedTrigger =
  Triggered Whenever
    (Attacks Macros.thisCreature (OneDefender (Macros.a AnyPlayer)))
    [] Nothing [] Nothing Nothing
    (Just (NotCond (Macros.exists (And [Opponent,
             Compare [PlayerStatAxis LifeTotal] Greater
                     (PlayerStatOf LifeTotal (Macros.That PlayerW OneOf))]))))
    (PutCounters (Lit 1) (PrintedKind Macros.plusOnePlusOne)
                 Macros.thisCreature)

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
                                   Macros.ofChosen (SubtypeQ Creature)]))
                   Nothing)
           (PutCounters (Lit 1) (PrintedKind Macros.plusOnePlusOne)
                        Macros.thisCreature) ]
       (Just (2, 2))

||| Chamber Sentry
public export
chamberSentryDamage : Ability
chamberSentryDamage =
  Macros.activated (Compound [Mana [Variable], TapSymbol,
                       Do (RemoveCounters (Just (ExactlyOf (LetterVal X)))
                             (Just (PrintedKind Macros.plusOnePlusOne)) Macros.thisCreature)])
                   (DealDamage This (LetterVal X) (Macros.target Macros.anyTarget))

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

||| Charnel Troll
public export
charnelTroll : Card
charnelTroll =
  Macros.card "Charnel Troll"
       (Just [Macros.generic 1, Macros.pip Black, Macros.pip Green]) []
       (MkTypeLine [creatureType "Troll"] [Creature])
       [ Macros.keyword "Trample"
       , Macros.triggered At (BeginningOf ThePart Upkeep (ByPlayer You))
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

||| Mistbreath Elder
public export
mistbreathElder : Card
mistbreathElder =
  Macros.card "Mistbreath Elder" (Just [Macros.pip Green]) []
       (MkTypeLine [creatureType "Frog", creatureType "Warrior"] [Creature])
       [ Macros.triggered At (BeginningOf ThePart Upkeep (ByPlayer You))
           ((IfDone (Macros.returnTo
                 (Macros.a (Macros.otherCreatureYouControl Macros.thisCreature))
                 Macros.handZ []) (Just (PutCounters (Lit 1) (PrintedKind Macros.plusOnePlusOne)
                           Macros.thisCreature)) (Just (Macros.may You (Macros.returnTo Macros.thisCreature Macros.handZ []))))) ]
       (Just (2, 2))

||| Dark Depths
public export
darkDepths : Card
darkDepths =
  Macros.card "Dark Depths" Nothing [Legendary, Snow]
       (MkTypeLine [] [Land])
       [ Static (Macros.entersWithCounters Macros.thisLand (Lit 10) (NamedCounter "Ice"))
       , Macros.activated (Mana [Macros.generic 3])
           (Macros.removeCounters (Macros.exactly 1) (Just (PrintedKind (NamedCounter "Ice"))) Macros.thisLand)
       , Macros.triggered When
           (StateHolds
              (NotCond (Matches Macros.thisLand (HasCounters (Just (NamedCounter "Ice"))))))
           ((IfDone (Macros.sacrifice You Macros.thisLand) (Just (Macros.create (Lit 1) Keyword.maritLage)) Nothing)) ]
       Nothing

||| Thallid
public export
thallid : Card
thallid =
  Macros.card "Thallid" (Just [Macros.pip Green]) []
       (MkTypeLine [creatureType "Fungus"] [Creature])
       [ Macros.triggered At (BeginningOf ThePart Upkeep (ByPlayer You))
           (PutCounters (Lit 1) (PrintedKind (NamedCounter "Spore")) Macros.thisCreature)
       , Macros.activated
           (Do (RemoveCounters (Just (Macros.exactly 3))
                  (Just (PrintedKind (NamedCounter "Spore"))) Macros.thisCreature))
           (Macros.create (Lit 1)
              (Macros.creatureTok 1 1 [Green] [creatureType "Saproling"])) ]
       (Just (1, 1))

||| Aether Chaser
public export
aetherChaserEnergy : Ability
aetherChaserEnergy =
  Macros.triggered When (Enters Macros.thisCreature Nothing)
                   (PutCounters (Lit 2) (PrintedKind (NamedCounter "Energy")) You)

||| Blastoderm
public export
blastodermEntersFading : StaticSpec []
blastodermEntersFading =
  Macros.entersWithCounters Macros.thisCreature (Lit 3) (NamedCounter "Fade")

||| Blastoderm
public export
blastodermFadeUpkeep : Ability
blastodermFadeUpkeep =
  Macros.triggered At (BeginningOf ThePart Upkeep (ByPlayer You))
    (RemoveCounters (Just (Macros.exactly 1))
       (Just (PrintedKind (NamedCounter "Fade"))) Macros.thisCreature)

||| Archfiend of the Dross
public export
archfiendOfTheDrossEntersOiled : StaticSpec []
archfiendOfTheDrossEntersOiled =
  Macros.entersWithCounters Macros.thisCreature (Lit 4) (NamedCounter "Oil")

||| Archfiend of the Dross
public export
archfiendOfTheDrossOilUpkeep : Ability
archfiendOfTheDrossOilUpkeep =
  Macros.triggered At (BeginningOf ThePart Upkeep (ByPlayer You))
    (RemoveCounters (Just (Macros.exactly 1))
       (Just (PrintedKind (NamedCounter "Oil"))) Macros.thisCreature)

neurokTransmuter : Instruction []
neurokTransmuter = Macros.becomes (Macros.target Macros.creature) (Macros.typesOnly [Artifact]) (Just Macros.untilEndOfTurn)

syphonMind : Instruction []
syphonMind =
  Sequentially
    [ Macros.discard (Macros.each Macros.otherPlayer) (Macros.a (InZone Macros.handZ))
    , ForEachOf (Macros.TheVerbed "Discard" CardW ThisWay ManyOf) (Draw You (Lit 1)) ]

peek : Instruction []
peek = Sequentially [Macros.lookAtHandOf (Macros.target AnyPlayer), (Draw You (Lit 1))]

||| Bumi, King of Three Trials
bumiScryMode : Instruction []
bumiScryMode =
  Macros.scry (Macros.target AnyPlayer) (Lit 3)

||| Final Act
finalActCounterMode : Instruction []
finalActCounterMode = Macros.losesAllCounters (Macros.each Opponent) Nothing

leeches : Instruction []
leeches = Macros.losesAllCounters (Macros.target AnyPlayer) (Just (PrintedKind (NamedCounter "Poison")))

||| Ascend's own reminder text
public export
ascendConferral : Instruction []
ascendConferral = Macros.getsCitysBlessing

||| Saddle's expansion body
public export
saddleConferral : Instruction []
saddleConferral = Macros.becomesSaddled

||| Woeleecher
public export
woeleecherWhole : Card
woeleecherWhole =
  Macros.card "Woeleecher" (Just [Macros.generic 5, Macros.pip White]) []
       (MkTypeLine [creatureType "Elemental"] [Creature])
       [ Counters.woeleecher ]
       (Just (3, 5))
