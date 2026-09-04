module Experimental.Cards.Deontic

import Experimental
import Experimental.Macros
import Experimental.Cards.Description

%default total


infiltrate : Instruction []
infiltrate = Macros.cantBeBlocked (Macros.target Macros.creature) (Just ThisTurn)

changeOfHeart : Instruction []
changeOfHeart = Macros.cantAttack (Macros.target Macros.creature) (Just ThisTurn)

blindblast : Instruction []
blindblast = Sequentially [DealDamage This (Lit 1) (Macros.target Macros.creature),
                           Macros.cantBlock (Macros.That (TypeW Creature) OneOf) (Just ThisTurn)]

blindingFlare : Instruction []
blindingFlare = Macros.cantBlock (Described (TargetDet Macros.anyNumber) Macros.creature) (Just ThisTurn)

cowardKiller : Instruction []
cowardKiller = Sequentially [Macros.cantBlock (Macros.target Macros.creature) (Just ThisTurn),
                             Macros.becomes (Macros.That (TypeW Creature) OneOf) (Macros.subtypesOnly [creatureType "Coward"])
                                     (Just Macros.untilEndOfTurn)]

||| Auriok Siege Sled
auriokSiegeSledDenial : Ability
auriokSiegeSledDenial =
  Macros.activated (Mana [Macros.generic 1])
                   (Continuously (Macros.cantDoTo "Block"
                                    (Macros.target (And [Macros.artifact, Macros.creature]))
                                    Macros.thisCreature)
                                 (Just ThisTurn))

||| Blindblast
blindblastWhole : Instruction []
blindblastWhole = Sequentially [DealDamage This (Lit 1) (Macros.target Macros.creature),
                                Macros.cantBlock (Macros.That (TypeW Creature) OneOf) (Just ThisTurn),
                                (Draw You (Lit 1))]

sparkmagesGambit : Instruction []
sparkmagesGambit =
  Sequentially [DealDamage This (Lit 1) (EachOf (Described (TargetDet (Macros.upTo 2)) Macros.creature)),
                Macros.cantBlock (Macros.That (TypeW Creature) ManyOf) (Just ThisTurn)]

||| Glacial Chasm
glacialChasmCant : Ability
glacialChasmCant = Static (Macros.deontic (Macros.allOf Macros.creatureYouControl) Forbid ["Attack"] Agent NoDeonticPatient)

desperateCastaways : Ability
desperateCastaways =
  Static (Macros.onlyUnless (Macros.deontic Macros.thisCreature Forbid ["Attack"] Agent NoDeonticPatient)
                   (Macros.exists (And [Macros.artifact, HasPossessor ControllerAx You])))

munghaWurm : Ability
munghaWurm = Static (Macros.cantMoreThan You "Untap" 1 Macros.land)

dampingField : Ability
dampingField = Static (Macros.cantMoreThan (PlayerGroup AllPlayers) "Untap" 1 Macros.artifact)

smoke : Ability
smoke = Static (Macros.cantMoreThan (PlayerGroup AllPlayers) "Untap" 1 Macros.creature)

winterOrb : Ability
winterOrb =
  Static (Macros.onlyWhile (Macros.cantMoreThan (PlayerGroup AllPlayers) "Untap" 1 Macros.land)
                          (Matches Macros.thisArtifact Macros.untapped))

staticOrb : Ability
staticOrb =
  Static (Macros.onlyWhile (Macros.cantMoreThan (PlayerGroup AllPlayers) "Untap" 2 Permanent)
                          (Matches Macros.thisArtifact Macros.untapped))

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
       [ Static (Macros.cantMoreThan (PlayerGroup AllPlayers) "Draw" 1 IsCard) ]
       (Just (3, 1))

winterMoon : Ability
winterMoon =
  Static (Macros.cantMoreThan (PlayerGroup AllPlayers) "Untap" 1
                         (And [Macros.land, Not (HasSupertype Basic)]))

leitmotifComposer : Ability
leitmotifComposer =
  Macros.activated (Mana [Macros.generic 2, Macros.pip Blue])
                   (Continuously (Macros.deontic (Macros.allOf (And [Macros.creature,
                                                Named (PrintedName "Leitmotif Composer")]))
                                   Forbid ["Block"] Patient NoDeonticPatient)
                          (Just ThisTurn))

berserkersOfBloodRidge : Ability
berserkersOfBloodRidge = Static (Macros.deontic Macros.thisCreature Require ["Attack"] Agent NoDeonticPatient)

trumpetingArmodon : Ability
trumpetingArmodon =
  Macros.activated (Mana [Macros.generic 1, Macros.pip Green])
                   (Continuously (Macros.deontic (Macros.target Macros.creature) Require ["Block"] Agent
                                   (DeonticCounterpart Macros.thisCreature))
                          (Just ThisTurn))

loathsomeCatoblepas : Ability
loathsomeCatoblepas =
  Macros.activated (Mana [Macros.generic 2, Macros.pip Green])
                   (Continuously (Macros.deontic Macros.thisCreature Require ["Block"] Patient NoDeonticPatient)
                          (Just ThisTurn))

hipparion : Ability
hipparion =
  Static (Macros.deontic Macros.thisCreature (GatedBy (Mana [Macros.generic 1]))
                  ["Block"] Agent
                  (DeonticCounterpart (Macros.allOf (And [Macros.creature,
                                                   Compare [StatAxis Power] AtLeast (Lit 3)]))))

frodoBaggins : Ability
frodoBaggins =
  Static (Macros.onlyWhile (Macros.deontic Macros.thisCreature Require ["Block"] Patient NoDeonticPatient)
                          (Matches Macros.thisCreature (HasDesignation RingBearer (Just You))))

bloodshedFever : Card
bloodshedFever =
  Macros.card "Bloodshed Fever" (Just [Macros.pip Red]) []
       (MkTypeLine [enchantmentType "Aura"] [Enchantment])
       [ Macros.keywordSubject "Enchant" Macros.creature
       , Static (Macros.deontic (AttachHost Enchanted (TypeW Creature))
                         Require ["Attack"] Agent NoDeonticPatient) ]
       Nothing

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
everybodyLivesGateLine : Instruction []
everybodyLivesGateLine =
  Continuously (Macros.deontic (PlayerGroup AllPlayers) Forbid ["LoseGame", "WinGame"]
                               Agent NoDeonticPatient)
               (Just ThisTurn)

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
                                  (AbilityOf (Macros.allOf (And [Macros.source,
                                                          Not (ColorIs Green)])))))) ]
       (Just (8, 5))

||| Nowhere to Run
public export
nowhereToRunWardLine : StaticSpec []
nowhereToRunWardLine =
  AndAlso Nothing
    [ Macros.canBeTargetedAsThough (Macros.allOf Macros.creatureYourOpponentsControl)
        (Macros.allOf (Or [Macros.spell, AbilityHead AnyOnStack]))
        (Not (HasKeyword (TheKeyword "Hexproof")))
    , Macros.deontic
        (Macros.allOf (And [ AbilityHead (KeywordClass "Ward")
                    , AbilityOf (Macros.That (TypeW Creature) ManyOf) ]))
        Forbid ["Trigger"] Agent NoDeonticPatient ]

||| Mornsong Aria
public export
mornsongAriaLock : StaticSpec []
mornsongAriaLock =
  Macros.deontic (PlayerGroup AllPlayers) Forbid ["Draw", "GainLife"]
                 Agent NoDeonticPatient

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
  Static (Macros.onlyWhile (Macros.deontic Macros.thisCreature Require ["Block"] Patient NoDeonticPatient)
                          (Matches Macros.thisCreature (IsAttached Equipped)))

anglerTurtle : Ability
anglerTurtle =
  Static (Macros.deontic (Macros.allOf Macros.creatureYourOpponentsControl) Require ["Attack"] Agent NoDeonticPatient)

ensnaringBridge : Card
ensnaringBridge =
  Macros.card "Ensnaring Bridge" (Just [Macros.generic 3]) []
       (MkTypeLine [] [Artifact])
       [ Static (Macros.deontic (Macros.allOf (And [Macros.creature,
                                      Compare [StatAxis Power] Greater
                                              (Macros.countOf (InZone (Macros.handOf You)))]))
                         Forbid ["Attack"] Agent NoDeonticPatient) ]
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
                          (Continuously (Macros.deontic ((Macros.It OneOf)) Forbid ["Block"] Patient NoDeonticPatient)
                                 (Just ThisTurn)) ]
       (Just (2, 3))

aetherTunnel : Card
aetherTunnel =
  Macros.card "Aether Tunnel" (Just [Macros.generic 1, Macros.pip Blue]) []
       (MkTypeLine [enchantmentType "Aura"] [Enchantment])
       [ Macros.keywordSubject "Enchant" Macros.creature
       , Static (AndAlso Nothing [ Gets Adds (AttachHost Enchanted (TypeW Creature))
                                (PtUp (Lit 1)) (PtUp (Lit 0))
                         , Macros.deontic ((Macros.It OneOf)) Forbid ["Block"] Patient NoDeonticPatient ]) ]
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
       [ Spell Nothing (Continuously (CantPrevent AnyDamage (DamageDescribed Unattributed Everywhere) NoPreventionOnly)
                             (Just ThisTurn))
       , Macros.keywordCosting "Flashback" (Mana [Macros.pip Red]) ]
       Nothing

public export
gideonJura : Card
gideonJura =
  Macros.cardOf "Gideon Jura"
       (Just [Macros.generic 3, Macros.pip White, Macros.pip White]) [Legendary]
       (MkTypeLine [planeswalkerType "Gideon"] [Planeswalker])
       [ Macros.activated (LoyaltySymbol (LoyaltyUp 2))
           (Macros.throughout
                       (Macros.deontic (Macros.allOf (And [Macros.creature,
                                             HasPossessor ControllerAx (Macros.target Opponent)]))
                                Require ["Attack"] Agent
                                (DefendingPlayer Macros.thisPlaneswalker))
                       (DuringNextTurnOf (Macros.That PlayerW OneOf)))
       , Macros.activated (LoyaltySymbol (LoyaltyDown 2))
           (Macros.destroy (Macros.target (And [Macros.creature, Macros.tapped])))
       , Macros.activated (LoyaltySymbol LoyaltyZero)
           (Sequentially
              [ Continuously
                  (Becomes Macros.thisPlaneswalker Sets (Bundle (MkToken (Just (Lit 6 ** Lit 6)) []
                                     (MkTypeLine [creatureType "Human",
                                                  creatureType "Soldier"] [Creature])
                                     [] Nothing) (Just Planeswalker)))
                  (Just Macros.untilEndOfTurn)
              , Macros.preventAll AnyDamage (ToRecipient ((Macros.It OneOf)))
                                  (Just ThisTurn) ]) ]
       (Macros.loyaltyBox 6)

public export
pinpointAvalanche : Card
pinpointAvalanche =
  Macros.card "Pinpoint Avalanche"
       (Just [Macros.generic 3, Macros.pip Red, Macros.pip Red]) []
       (MkTypeLine [] [Instant])
       [ Spell Nothing (Sequentially
                  [ DealDamage This (Lit 4) (Macros.target Macros.creature)
                  , Continuously
                      (CantPrevent AnyDamage ThatDamage NoPreventionOnly) Nothing ]) ]
       Nothing

public export
whippoorwillImmunity : Instruction []
whippoorwillImmunity =
  Sequentially
    [ Continuously (Macros.objectCant "Regenerate" (Macros.target Macros.creature))
                   (Just ThisTurn)
    , Continuously
        (CantPrevent AnyDamage
                     (DamageDescribed Unattributed
                        (ToRecipient (Macros.That (TypeW Creature) OneOf)))
                     NoRedirectEither)
        (Just ThisTurn) ]

public export
callInAProfessional : Card
callInAProfessional =
  Macros.card "Call In a Professional"
       (Just [Macros.generic 2, Macros.pip Red]) []
       (MkTypeLine [] [Instant])
       [ Spell Nothing (Sequentially
                  [ Continuously
                      (Macros.playerCant "GainLife" (PlayerGroup AllPlayers))
                      (Just ThisTurn)
                  , Continuously
                      (CantPrevent AnyDamage (DamageDescribed Unattributed Everywhere) NoPreventionOnly)
                      (Just ThisTurn)
                  , DealDamage This (Lit 3) (Macros.target Macros.anyTarget) ]) ]
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
                                           (NameOfCard (Not (Or [Macros.creature,
                                                                 Macros.land]))))
       , Static (Macros.cantDoTo "Cast" (PlayerGroup YourOpponents)
                   (Macros.allOf (And [Macros.spell, Named ChosenName])))
       , Static (Costs (Macros.allOf (And [Macros.spell, Named ChosenName,
                                          Macros.castBy You]))
                             (CostLess (Lit 2) Nothing)) ]
       (Just (2, 4))

||| Failure // Comply
public export
complyNameLock : Instruction []
complyNameLock =
  Sequentially
    [ Macros.choose (Macros.a (Macros.quality CardName))
    , Continuously
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
       , Static (DamageRule AnyDamage (DealtBy (Macros.allOf (And [Macros.source, Named ChosenName]))) (ToRecipient (Macros.youAnd
                      (Macros.allOf (And [Permanent, HasPossessor ControllerAx You])))) (Prevent CutAll Nothing) Repeatedly) ]
       Nothing

||| Academic Probation
public export
academicProbationNameMode : Instruction []
academicProbationNameMode =
  Sequentially
    [ Macros.choose (Macros.a (Macros.qualityFrom CardName
                                 (NameOfCard (Not Macros.land))))
    , Continuously
        (Macros.cantDoTo "Cast" (PlayerGroup YourOpponents)
           (Macros.allOf (And [Macros.spell, Named ChosenName])))
        (Just Macros.untilYourNextTurn) ]

public export
fatigue : Card
fatigue =
  Macros.card "Fatigue" (Just [Macros.generic 1, Macros.pip Blue]) []
       (MkTypeLine [] [Sorcery])
       [Spell Nothing (SkipsNext (Macros.target AnyPlayer) DrawStep (Lit 1))] Nothing

public export
meditate : Card
meditate =
  Macros.card "Meditate" (Just [Macros.generic 2, Macros.pip Blue]) []
       (MkTypeLine [] [Instant])
       [Spell Nothing (Sequentially [Draw You (Lit 4), SkipsNext You Turn (Lit 1)])] Nothing

public export
blindingAngel : Card
blindingAngel =
  Macros.card "Blinding Angel"
       (Just [Macros.generic 3, Macros.pip White, Macros.pip White]) []
       (MkTypeLine [creatureType "Angel"] [Creature])
       [ Macros.keyword "Flying"
       , Macros.triggered Whenever
           (Macros.dealsCombatDamage Macros.thisCreature (Macros.a AnyPlayer))
           (SkipsNext (Macros.That PlayerW OneOf) Combat (Lit 1)) ] (Just (2, 4))

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
       , Macros.triggered At (BeginningOf ThePart Upkeep (ByPlayer You))
           ((May You (Pay You (Mana [Macros.pip Blue]) PaidOnce) Nothing (Just (Macros.sacrifice You Macros.thisEnchantment)))) ] Nothing

public export
yawgmothsBargain : Card
yawgmothsBargain =
  Macros.card "Yawgmoth's Bargain"
       (Just [Macros.generic 4, Macros.pip Black, Macros.pip Black]) []
       (MkTypeLine [] [Enchantment])
       [ Static (Skips You DrawStep)
       , Macros.activated (Macros.payLife You 1) (Draw You (Lit 1)) ] Nothing

public export
sandsOfTimeSkip : StaticSpec []
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
       (Just (6, 1))

public export
eaterOfDaysSkip : Instruction []
eaterOfDaysSkip = SkipsNext You Turn (Lit 2)

||| Empty City Ruse
public export
emptyCityRuse : Card
emptyCityRuse =
  Macros.card "Empty City Ruse" (Just [Macros.pip White]) []
       (MkTypeLine [] [Sorcery])
       [Spell Nothing (Macros.throughout (Skips (Macros.target Opponent) Combat)
                                 (DuringNextTurnOf (Macros.That PlayerW OneOf)))] Nothing

||| False Peace
public export
falsePeace : Card
falsePeace =
  Macros.card "False Peace" (Just [Macros.pip White]) []
       (MkTypeLine [] [Sorcery])
       [Spell Nothing (Macros.throughout (Skips (Macros.target AnyPlayer) Combat)
                                 (DuringNextTurnOf (Macros.That PlayerW OneOf)))] Nothing

||| Battlefront Krushok
public export
battlefrontKrushokEvasion : Ability
battlefrontKrushokEvasion =
  Static (Deontic Macros.thisCreature Forbid ["Block"] Patient (Just (MoreThan (Lit 1)))
                  (DeonticCounterpart (Macros.allOf Macros.creature)) Nothing NoDeonticRider)

||| Grand Abolisher
public export
grandAbolisher : Card
grandAbolisher =
  Macros.card "Grand Abolisher" (Just [Macros.pip White, Macros.pip White]) []
       (MkTypeLine [creatureType "Human", creatureType "Cleric"] [Creature])
       [ Static (OnlyDuring Turn (Just You)
                   (AndAlso Nothing
                      [ Macros.cantDoTo "Cast" (PlayerGroup YourOpponents)
                          (Macros.allOf Macros.spell)
                      , Macros.cantDoTo "Activate" (PlayerGroup YourOpponents)
                          (Macros.allOf (And [ AbilityHead AnyActivated
                                      , AbilityOf (Macros.allOf (Or [Macros.artifact,
                                                              Macros.creature,
                                                              Macros.enchantment])) ])) ])) ]
       (Just (2, 2))

||| Festival
public export
festival : Card
festival =
  Macros.card "Festival" (Just [Macros.pip White]) []
       (MkTypeLine [] [Instant])
       [ Spell (Just (DuringPart Upkeep (Just Macros.anOpponent)))
               (Macros.cantAttack (Macros.allOf Macros.creature)
                                  (Just ThisTurn)) ]
       Nothing

public export
demotion : Card
demotion =
  Macros.card "Demotion" (Just [Macros.pip White]) []
       (MkTypeLine [enchantmentType "Aura"] [Enchantment])
       [ Macros.keywordSubject "Enchant" Macros.creature
       , Static (AndAlso Nothing
           [ Macros.deontic (AttachHost Enchanted (TypeW Creature)) Forbid ["Block"] Agent NoDeonticPatient
           , Macros.objectCant "Activate"
               (Macros.allOf (And [AbilityHead AnyActivated, AbilityOf ((Macros.It OneOf))])) ]) ]
       Nothing

public export
terror : Card
terror =
  Macros.card "Terror" (Just [Macros.generic 1, Macros.pip Black]) []
       (MkTypeLine [] [Instant])
       [ Spell Nothing (CantBe (Macros.destroy (Macros.target
                  (And [Macros.creature, Not Macros.artifact,
                        Not (ColorIs Black)])))
                "Regenerate" (Macros.ItVerbed "Destroy" OneOf)) ]
       Nothing

public export
snuffOut : Card
snuffOut =
  Macros.card "Snuff Out" (Just [Macros.generic 3, Macros.pip Black]) []
       (MkTypeLine [] [Instant])
       [ Static (Macros.onlyWhile
                   (AltCost This (Just (Macros.payLife You 4)))
                   (Macros.exists (And [Macros.land, HasSubtype (landType "Swamp"),
                                 HasPossessor ControllerAx You])))
       , Spell Nothing (CantBe (Macros.destroy (Macros.target
                  (And [Macros.creature, Not (ColorIs Black)])))
                "Regenerate" (Macros.ItVerbed "Destroy" OneOf)) ]
       Nothing

public export
wrathOfGod : Card
wrathOfGod =
  Macros.card "Wrath of God"
       (Just [Macros.generic 2, Macros.pip White, Macros.pip White]) []
       (MkTypeLine [] [Sorcery])
       [ Spell Nothing (CantBe (Macros.destroy (Macros.allOf Macros.creature))
                       "Regenerate" ((Macros.It ManyOf))) ]
       Nothing

public export
damnation : Card
damnation =
  Macros.card "Damnation"
       (Just [Macros.generic 2, Macros.pip Black, Macros.pip Black]) []
       (MkTypeLine [] [Sorcery])
       [ Spell Nothing (CantBe (Macros.destroy (Macros.allOf Macros.creature))
                       "Regenerate" ((Macros.It ManyOf))) ]
       Nothing

public export
glacialChasm : Card
glacialChasm =
  Macros.card "Glacial Chasm" Nothing []
       (MkTypeLine [] [Land])
       [ Macros.cumulativeUpkeep (Macros.payLife You 2)
       , Macros.triggered When (Enters Macros.thisLand Nothing)
                          (Macros.sacrifice You (Macros.a Macros.land))
       , Static (Macros.deontic (Macros.allOf Macros.creatureYouControl) Forbid ["Attack"] Agent NoDeonticPatient)
       , Static (DamageRule AnyDamage Unattributed (ToRecipient You) (Prevent CutAll Nothing) Repeatedly) ]
       Nothing

||| Peacekeeper
public export
peacekeeperCant : Ability
peacekeeperCant = Static (Macros.deontic (Macros.allOf Macros.creature) Forbid ["Attack"] Agent NoDeonticPatient)

||| Culling Mark
public export
cullingMark : Card
cullingMark =
  Macros.card "Culling Mark" (Just [Macros.generic 2, Macros.pip Green]) []
       (MkTypeLine [] [Sorcery])
       [ Spell Nothing (Continuously (Macros.deontic (Macros.target Macros.creature)
                                      Require ["Block"] Agent NoDeonticPatient)
                             (Just ThisTurn)) ]
       Nothing

||| Blazing Archon
public export
blazingArchonCant : Ability
blazingArchonCant =
  Static (Macros.deontic (Macros.allOf Macros.creature) Forbid ["Attack"] Agent (DefendingPlayer You))


||| Glaring Spotlight
public export
glaringSpotlight : Card
glaringSpotlight =
  Macros.card "Glaring Spotlight" (Just [Macros.generic 1]) []
       (MkTypeLine [] [Artifact])
       [ Static (Macros.canBeTargetedAsThough
                   (Macros.allOf (And [Macros.creature, HasPossessor ControllerAx (PlayerGroup YourOpponents),
                                HasKeyword (TheKeyword "Hexproof")]))
                   (Macros.allOf (And [Or [Macros.spell, AbilityHead AnyOnStack], HasPossessor ControllerAx You]))
                   (Not (HasKeyword (TheKeyword "Hexproof"))))
       , Macros.activated
           (Compound [Mana [Macros.generic 3], Do (Macros.sacrifice You Macros.thisArtifact)])
           (Sequentially
              [ Macros.gains (Macros.allOf Macros.creatureYouControl)
                             (Macros.keyword "Hexproof") (Just Macros.untilEndOfTurn)
              , Continuously
                  (Macros.deontic (Macros.allOf Macros.creatureYouControl) Forbid ["Block"]
                                  Patient NoDeonticPatient)
                  (Just ThisTurn) ]) ]
       Nothing

||| Canoptek Wraith
public export
canoptekWraith : Ability
canoptekWraith =
  Macros.flavorWord "Wraith Form"
    (Static (Deontic Macros.thisCreature Forbid ["Block"] Patient Nothing
                     NoDeonticPatient Nothing NoDeonticRider))

||| Gadrak, the Crown-Scourge
public export
gadrakCantAttack : Ability
gadrakCantAttack =
  Static (Macros.onlyUnless (Macros.deontic Macros.thisCreature Forbid ["Attack"] Agent
                                     NoDeonticPatient)
                            (CompareAmt (Macros.countOf (And [Macros.artifact, HasPossessor ControllerAx You]))
                                        AtLeast (Lit 4)))

||| Berserker's Frenzy, the 1—14 striation
public export
berserkersFrenzyLowRoll : Instruction []
berserkersFrenzyLowRoll =
  Sequentially
    [ Macros.choose (Macros.counted Macros.anyNumber Macros.creature)
    , Continuously (Macros.deontic ((Macros.It ManyOf)) Require ["Block"] Agent NoDeonticPatient)
                   (Just ThisTurn) ]

||| Damn
public export
damnDestroyLine : Instruction []
damnDestroyLine =
  CantBe (Macros.destroy (Macros.target Macros.creature))
         "Regenerate" (Macros.TheVerbed "Destroy" (TypeW Creature) ThisWay OneOf)

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
                   "Regenerate" (Macros.That (TypeW Creature) OneOf)) ]
       (Just (2, 1))

||| Concussive Bolt, both paragraphs
public export
concussiveBolt : Instruction []
concussiveBolt =
  Sequentially
    [ DealDamage This (Lit 4) Description.targetPlayerOrPlaneswalker
    , If (CompareAmt (Macros.countOf (And [Macros.artifact, HasPossessor ControllerAx You]))
                     AtLeast (Lit 3))
         (Continuously
            (Macros.deontic (Macros.allOf (And [Macros.creature,
                                  HasPossessor ControllerAx Macros.splitOverPlaneswalker]))
                     Forbid ["Block"] Agent NoDeonticPatient)
            (Just ThisTurn))
         Nothing ]

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

||| Propaganda
public export
propaganda : Card
propaganda =
  Macros.card "Propaganda" (Just [Macros.generic 2, Macros.pip Blue]) []
       (MkTypeLine [] [Enchantment])
       [ Static (Macros.deontic (Macros.allOf Macros.creature)
                   (GatedBy (Macros.scaledMana GenericUnit (Macros.times 2
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
                   (GatedBy (Macros.scaledMana GenericUnit (Macros.times 2
                      (Macros.countOf (And [Macros.creature, HasPossessor ControllerAx They,
                                     CombatRel AttackerOf You])))))
                   ["Attack"] Agent (DefendingPlayer You)) ]
       Nothing

||| Archangel of Tithes
public export
archangelOfTithesBlockToll : Ability
archangelOfTithesBlockToll =
  Static (Macros.onlyWhile (Macros.deontic (Macros.allOf Macros.creature)
                              (GatedBy (Macros.scaledMana GenericUnit (Macros.times 1 GroupSize)))
                              ["Block"] Agent NoDeonticPatient)
            (Matches Macros.thisCreature Attacking))

||| Archangel of Tithes
public export
archangelOfTithesAttackToll : Ability
archangelOfTithesAttackToll =
  Static (Macros.onlyWhile (Macros.deontic (Macros.allOf Macros.creature)
                              (GatedBy (Macros.scaledMana GenericUnit (Macros.times 1 GroupSize)))
                              ["Attack"] Agent
                              (DefendingPlayer (Macros.youOr
                                                  (Macros.allOf (And [HasType Planeswalker,
                                                               HasPossessor ControllerAx You])))))
            (Matches Macros.thisCreature Macros.untapped))

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
                   (GatedBy (Macros.scaledMana GenericUnit (Macros.times 1 GroupSize)))
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
       [ Macros.triggered At (BeginningOf ThePart Upkeep (ByPlayer You))
           (PutCounters (Lit 1) (PrintedKind Macros.plusOnePlusOne) Macros.thisCreature)
       , Static (Macros.deontic Macros.thisCreature
                   (GatedBy (Macros.scaledMana GenericUnit (Macros.times 1
                      (CountersOn Macros.plusOnePlusOne ((Macros.It OneOf))))))
                   ["Attack", "Block"] Agent NoDeonticPatient) ]
       (Just (2, 2))

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
                                   (Macros.times 1 (Macros.countOf (And [Macros.creature, Blocking,
                                                           HasPossessor ControllerAx They]))))))
                   ["Block"] Agent
                   (DeonticCounterpart (Macros.allOf Macros.creatureYouControl))) ]
       Nothing

||| Awesome Presence
public export
awesomePresence : Card
awesomePresence =
  Macros.card "Awesome Presence" (Just [Macros.pip Blue]) []
       (MkTypeLine [enchantmentType "Aura"] [Enchantment])
       [ Macros.keywordSubject "Enchant" Macros.creature
       , Static (Macros.deontic (AttachHost Enchanted (TypeW Creature))
                   (GatedBy (Macros.scaledMana GenericUnit (Macros.times 3
                      (Macros.countOf (And [Macros.creature, HasPossessor ControllerAx They,
                                     CombatRel BlockerOf ((Macros.It OneOf))])))))
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
       , Static (Costs
                   (Macros.allOf (And [ AbilityHead AnyActivated
                               , AbilityOf (AttachHost Enchanted (TypeW Creature)) ]))
                   (CostMore (Lit 3))) ]
       Nothing

||| Distortion Strike
public export
distortionStrikeLine : Instruction []
distortionStrikeLine =
  Sequentially
    [ Macros.gets (Macros.target Macros.creature) (PtUp (Lit 1)) (PtUp (Lit 0))
                  (Just Macros.untilEndOfTurn)
    , Continuously
        (Macros.deontic (Macros.That (TypeW Creature) OneOf) Forbid ["Block"]
                        Patient NoDeonticPatient)
        (Just ThisTurn) ]

||| Retro-Mutation
public export
retroMutation : Card
retroMutation =
  Macros.card "Retro-Mutation" (Just [Macros.generic 2, Macros.pip Blue]) []
       (MkTypeLine [enchantmentType "Aura"] [Enchantment])
       [ Macros.keyword "Flash"
       , Macros.keywordSubject "Enchant" Macros.creature
       , Static (AndAlso Nothing
           [ Becomes (AttachHost Enchanted (TypeW Creature)) Sets (Bundle (MkToken Nothing []
                               (MkTypeLine [creatureType "Turtle"] []) [] Nothing) Nothing)
           , Macros.hasBasePt ((Macros.It OneOf)) (Lit 0) (Lit 1)
           , Deontic ((Macros.It OneOf)) Forbid ["Attack"] Agent Nothing NoDeonticPatient Nothing
                      NoDeonticRider
           , LosesAllAbilities ((Macros.It OneOf)) Nothing ]) ]
       Nothing

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
cloudspireCaptainCrewLine : StaticSpec []
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

||| Arrest
public export
arrest : Card
arrest =
  Macros.card "Arrest" (Just [Macros.generic 2, Macros.pip White]) []
       (MkTypeLine [enchantmentType "Aura"] [Enchantment])
       [ Macros.keywordSubject "Enchant" Macros.creature
       , Static (AndAlso Nothing
                   [ Macros.deontic (AttachHost Enchanted (TypeW Creature))
                       Forbid ["Attack", "Block"] Agent NoDeonticPatient
                   , Macros.deontic (Macros.allOf (And [AbilityHead AnyActivated,
                                                 AbilityOf ((Macros.It OneOf))]))
                       Forbid ["Activate"] Patient NoDeonticPatient ]) ]
       Nothing

||| Conqueror's Flail
public export
conquerorsFlailProhibition : StaticSpec []
conquerorsFlailProhibition =
  Macros.onlyWhile (OnlyDuring Turn (Just You)
                      (Macros.cantDoTo "Cast" (PlayerGroup YourOpponents) (Macros.allOf Macros.spell)))
    (Matches This (AttachedTo (Macros.a Macros.creature)))

