module Experimental.Cards.Damage

import Experimental
import Experimental.Macros
import Experimental.Cards.Description
import Experimental.Cards.Anaphora
import Experimental.Cards.Trigger

%default total


||| Lightning Bolt
bolt : Instruction []
bolt = DealDamage This (Lit 3) (Macros.target Macros.anyTarget)

barrageOfBoulders : Instruction []
barrageOfBoulders = DealDamage This (Lit 1) (Macros.each Macros.creatureYouDontControl)

rabidBite : Instruction []
rabidBite = DealDamage (Macros.target Macros.creatureYouControl)
                       (StatOf Power ((Macros.It OneOf)))
                       (Macros.target Macros.creatureYouDontControl)

preyUpon : Instruction []
preyUpon = Fights (Macros.target Macros.creatureYouControl) (Macros.target Macros.creatureYouDontControl)

arcTrail : Instruction []
arcTrail = Sequentially [DealDamage This (Lit 2) (Macros.target Macros.anyTarget),
                         DealDamage This (Lit 1) (Macros.target Macros.anyOtherTarget)]

deadshot : Instruction []
deadshot = Sequentially [SetStatus Tapped (Macros.target Macros.creature),
                         DealDamage ((Macros.It OneOf)) (StatOf Power ((Macros.It OneOf))) (Macros.target (And [Macros.creature, Other]))]

immersturmSkullcairn : Ability
immersturmSkullcairn =
  Macros.activatedOnlyDuring (Compound [Mana [Macros.generic 1, Macros.pip Black, Macros.pip Red, Macros.pip Red], TapSymbol,
                                        Do (Macros.sacrifice You Macros.thisLand)])
                             (Sequentially [DealDamage ((Macros.It OneOf)) (Lit 3) (Macros.target AnyPlayer),
                                            (Macros.discard (Macros.That PlayerW OneOf) (Macros.a (InZone Macros.handZ)))])
                             AsSorcery

pyriteSpellbomb : Ability
pyriteSpellbomb = Macros.activated (Compound [Mana [Macros.pip Red], Do (Macros.sacrifice You Macros.thisArtifact)])
                                   (DealDamage ((Macros.It OneOf)) (Lit 2) (Macros.target Macros.anyTarget))

karplusanYeti : Instruction []
karplusanYeti = Sequentially [DealDamage Macros.thisCreature (StatOf Power Macros.thisCreature) (Macros.target Macros.creature),
                              DealDamage (Macros.That (TypeW Creature) OneOf) (StatOf Power ((Macros.It OneOf))) Macros.thisCreature]

suddenDemise : Instruction []
suddenDemise = Sequentially [Macros.choose (Macros.a (Macros.quality Color)),
                             DealDamage This (LetterVal X) (Macros.each (And [Macros.creature, Macros.ofChosen Color]))]

caseOfTheGatewayExpress : Instruction []
caseOfTheGatewayExpress = Sequentially [Macros.choose (Macros.target Macros.creatureYouDontControl),
                                        DealDamage (Macros.each Macros.creatureYouControl) (Lit 1)
                                                   (Macros.That (TypeW Creature) OneOf)]

arrowsOfJustice : Instruction []
arrowsOfJustice = DealDamage This (Lit 4)
                             (Macros.target (And [Macros.creature, Or [Attacking, Blocking]]))

flamesOfTheRazeBoar : Instruction []
flamesOfTheRazeBoar =
  Sequentially [DealDamage This (Lit 4) (Macros.target (And [Macros.creature, HasPossessor ControllerAx Macros.anOpponent])),
                OnlyIf (DealDamage This (Lit 2)
                                   (Macros.each (And [Macros.creature, Other, HasPossessor ControllerAx (Macros.That PlayerW OneOf)])))
                       (Macros.exists (And [Macros.creature, HasPossessor ControllerAx You,
                                     Compare [StatAxis Power] AtLeast (Lit 4)]))
                       Nothing]

yawgmothDemon : Instruction []
yawgmothDemon =
  (May You (Macros.sacrifice You (Macros.a Macros.artifact)) Nothing (Just (Sequentially [SetStatus Tapped Macros.thisCreature, DealDamage This (Lit 2) You])))

arcBlade : Instruction []
arcBlade = Sequentially [DealDamage This (Lit 2) (Macros.target Macros.anyTarget),
                         Macros.exileWithCounters This (Lit 3) (NamedCounter "Time")]

abrade : Instruction []
abrade = Macros.chooseModes (Macros.exactly 1) [DealDamage This (Lit 3) (Macros.target Macros.creature),
                    Macros.destroy (Macros.target Macros.artifact)]

nibelheimAflame : Instruction []
nibelheimAflame =
  Sequentially [Macros.choose (Macros.target Macros.creatureYouControl),
                DealDamage ((Macros.It OneOf)) (StatOf Power ((Macros.It OneOf))) (Macros.each (Macros.otherCreature ((Macros.It OneOf))))]

brashTaunter : Instruction []
brashTaunter = Fights Macros.thisCreature (Macros.target (Macros.otherCreature Macros.thisCreature))

ulvenwaldTracker : Instruction []
ulvenwaldTracker = Fights (Macros.target Macros.creatureYouControl) (Macros.target (And [Macros.creature, Other]))


botBashingTime : Instruction []
botBashingTime =
  Sequentially [ DealDamage This (Lit 6) (Macros.target Macros.creature)
               , Macros.ifWouldInstead (Dies (Macros.That (TypeW Creature) OneOf)) (Macros.exile You ((Macros.It OneOf))) (Just ThisTurn)
               ]

wordsOfWar : Instruction []
wordsOfWar = Macros.nextTimeWouldInstead (Draws You)
                                  (DealDamage This (Lit 2) (Macros.target Macros.anyTarget))
                                  (Just ThisTurn)

||| Thunderwave
public export
thunderwave : Card
thunderwave =
  Macros.card "Thunderwave"
       (Just [Macros.generic 2, Macros.pip Red, Macros.pip Red]) []
       (MkTypeLine [] [Sorcery])
       [ Spell Nothing (Sequentially
                  [ Macros.rollDice You 1 20
                  , Macros.resultsTable
                      [ Macros.rollRow (Macros.fromTo 1 9)
                          (DealDamage This (Lit 3) (Macros.each Macros.creature))
                      , Macros.rollRow (Macros.fromTo 10 19)
                          (Sequentially
                             [ Macros.may You
                                 (Macros.chooses You (Macros.a Macros.creature))
                             , DealDamage This (Lit 3)
                                 (Macros.each (And [Macros.creature, NotChosen])) ])
                      , Macros.rollRow (Macros.fromTo 20 20)
                          (DealDamage This (Lit 6)
                             (Macros.each
                                (And [Macros.creature,
                                      HasPossessor ControllerAx
                                        (PlayerGroup YourOpponents)]))) ] ]) ]
       Nothing

||| Fog, Holy Day, Darkness, Root Snare
fog : Instruction []
fog = Macros.preventAll CombatOnly Everywhere (Just ThisTurn)

||| Indestructible Aura, Shielded Passage
indestructibleAura : Instruction []
indestructibleAura = Macros.preventAll AnyDamage (ToRecipient (Macros.target Macros.creature)) (Just ThisTurn)

shieldmatesBlessing : Instruction []
shieldmatesBlessing =
  Macros.preventNext AnyDamage (ToRecipient (Macros.target Macros.anyTarget)) (Lit 3) (Just ThisTurn)

moonlitWake : Ability
moonlitWake = Macros.triggered Whenever (Dies (Macros.a Macros.creature)) (Macros.gainsLife You (Lit 1))

eliteJavelineer : Ability
eliteJavelineer =
  Macros.triggered Whenever (Blocks Macros.thisCreature Nothing)
                   (DealDamage This (Lit 1) (Macros.target (And [Macros.creature, Attacking])))

||| Glacial Chasm
glacialChasmShield : Ability
glacialChasmShield =
  Static (DamageRule AnyDamage Unattributed (ToRecipient You) (Prevent CutAll Nothing) Repeatedly)

corneredCrook : Ability
corneredCrook =
  Macros.triggered When (Enters Macros.thisCreature Nothing)
    (Macros.mayWhen You (Macros.sacrifice You (Macros.a Macros.artifact))
                 (DealDamage This (Lit 3) (Macros.target Macros.anyTarget)))

aladdinsRing : Card
aladdinsRing =
  Macros.card "Aladdin's Ring" (Just [Macros.generic 8]) [] (MkTypeLine [] [Artifact])
       [Macros.activated (Compound [Mana [Macros.generic 8], TapSymbol])
                         (DealDamage Macros.thisArtifact (Lit 4) (Macros.target Macros.anyTarget))]
       Nothing

chandrasRevolution : Instruction []
chandrasRevolution = Sequentially [DealDamage This (Lit 4) (Macros.target Macros.creature),
                                   SetStatus Tapped (Macros.target Macros.land),
                                   DoesntUntapNext (Macros.That (TypeW Land) OneOf) (Lit 1)]

arbalestElite : Ability
arbalestElite =
  Macros.activated (Compound [Mana [Macros.generic 2, Macros.pip White], TapSymbol])
                   (Sequentially [DealDamage Macros.thisCreature (Lit 3)
                                      (Macros.target (And [Macros.creature, Or [Attacking, Blocking]])),
                           DoesntUntapNext Macros.thisCreature (Lit 1)])

sizzlingBarrage : Instruction []
sizzlingBarrage =
  DealDamage This (Lit 4)
             (Macros.target (And [Macros.creature,
                                  Macros.happenedTo BlockDeclaration Lookback.ThisTurn]))

goadedAttackTrigger : Ability
goadedAttackTrigger =
  Macros.triggered Whenever (Macros.attacks (Macros.a (And [Macros.creature, HasDesignation Goaded Nothing])))
                   (DealDamage ((Macros.It OneOf)) (Lit 1) (Macros.controllerOf ((Macros.It OneOf))))

extraArms : Card
extraArms =
  Macros.card "Extra Arms" (Just [Macros.generic 4, Macros.pip Red]) []
       (MkTypeLine [enchantmentType "Aura"] [Enchantment])
       [ Macros.keywordSubject "Enchant" Macros.creature
       , Macros.triggered Whenever (Macros.attacks (AttachHost Enchanted (TypeW Creature)))
                          (DealDamage ((Macros.It OneOf)) (Lit 2) (Macros.target Macros.anyTarget)) ]
       Nothing

chainReaction : Card
chainReaction =
  Macros.card "Chain Reaction"
       (Just [Macros.generic 2, Macros.pip Red, Macros.pip Red]) []
       (MkTypeLine [] [Sorcery])
       [ Spell Nothing (Sequentially
                  [ DealDamage This (LetterVal X) (Macros.each Macros.creature)
                  , Define X (Macros.countOf Macros.creature) ]) ]
       Nothing

||| Black Vise
public export
blackVise : Card
blackVise =
  Macros.card "Black Vise" (Just [Macros.generic 1]) []
       (MkTypeLine [] [Artifact])
       [ Static (Macros.entersChoosingPlayer Macros.thisArtifact (Just OpponentsOnly))
       , Macros.triggered At
           (Macros.beginningOfPossessed ThePart Upkeep (Macros.the Macros.chosenPlayer))
           (Sequentially
              [ DealDamage Macros.thisArtifact (LetterVal X) (Macros.That PlayerW OneOf)
              , Define X (Minus (Macros.countOf (InZone (Macros.handOf Macros.They)))
                          (Lit 4)) ]) ]
       Nothing

harshSustenance : Card
harshSustenance =
  Macros.card "Harsh Sustenance"
       (Just [Macros.generic 1, Macros.pip White, Macros.pip Black]) []
       (MkTypeLine [] [Instant])
       [ Spell Nothing (Sequentially
                  [ DealDamage This (LetterVal X) (Macros.target Macros.anyTarget)
                  , Macros.gainsLife You (LetterVal X)
                  , Define X (Macros.countOf Macros.creatureYouControl) ]) ]
       Nothing

||| Purging Scythe
public export
purgingScythe : Ability
purgingScythe =
  Macros.triggered At (BeginningOf ThePart Upkeep (ByPlayer You))
    (Sequentially
       [ DealDamage Macros.thisArtifact (Lit 2)
                    (Macros.the (And [Macros.creature,
                                    Superlative MinOf (StatAxis Toughness)
                                                Macros.creature]))
       , If (CompareAmt (Macros.countOf (And [Macros.creature,
                                       Superlative MinOf (StatAxis Toughness)
                                                   Macros.creature]))
                        AtLeast (Lit 2))
            (Macros.chooses You (Macros.someOf (Macros.exactly 1) ((Macros.It ManyOf))))
            Nothing ])

lionHeart : Card
lionHeart =
  Macros.card "Lion Heart" (Just [Macros.generic 4]) []
       (MkTypeLine [artifactType "Equipment"] [Artifact])
       [ Macros.triggered When (Enters Macros.thisEquipment Nothing)
                          (DealDamage ((Macros.It OneOf)) (Lit 2) (Macros.target Macros.anyTarget))
       , Static (Gets Adds (AttachHost Equipped (TypeW Creature))
                      (PtUp (Lit 2)) (PtUp (Lit 1)))
       , Macros.keywordCosting "Equip" (Mana [Macros.generic 2]) ]
       Nothing

lightmineField : Card
lightmineField =
  Macros.card "Lightmine Field"
       (Just [Macros.generic 2, Macros.pip White, Macros.pip White]) []
       (MkTypeLine [] [Enchantment])
       [ Macros.triggered Whenever
                          (Macros.attacks (Macros.counted (Macros.atLeast 1) Macros.creature))
                          (DealDamage Macros.thisEnchantment
                               (CountOf (Macros.That (TypeW Creature) ManyOf))
                               (EachOf (Macros.That (TypeW Creature) ManyOf))) ]
       Nothing

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
                                        (Macros.That (TypeW Creature) OneOf)) ]
       (Just (4, 4))

||| Psychic Purge
public export
psychicPurge : Card
psychicPurge =
  Macros.card "Psychic Purge" (Just [Macros.pip Blue]) []
       (MkTypeLine [] [Sorcery])
       [ Spell Nothing (DealDamage This (Lit 1) (Macros.target Macros.anyTarget))
       , Macros.triggered When
           (Causes
              (CausedBySource
                 (Macros.a (And [ Or [Macros.spell, AbilityHead AnyOnStack]
                                , HasPossessor ControllerAx (Macros.a Opponent) ])))
              (VerbedEvent (Just You) "Discard" (Just This) Nothing))
           (Macros.losesLife (Macros.That PlayerW OneOf) (Lit 5)) ]
       Nothing

||| Chandra Nalaar
chandraNalaarsX : Ability
chandraNalaarsX =
  Macros.activated (LoyaltySymbol LoyaltyDownX)
                   (DealDamage This (LetterVal X) (Macros.target Macros.creature))

public export
etherealHaze : Card
etherealHaze =
  Macros.card "Ethereal Haze" (Just [Macros.pip White]) []
       (MkTypeLine [spellType "Arcane"] [Instant])
       [ Spell Nothing (Macros.preventAllBy AnyDamage (Macros.allOf Macros.creature)
                                    Everywhere (Just ThisTurn)) ]
       Nothing

public export
defang : Card
defang =
  Macros.card "Defang" (Just [Macros.generic 1, Macros.pip White]) []
       (MkTypeLine [enchantmentType "Aura"] [Enchantment])
       [ Macros.keywordSubject "Enchant" Macros.creature
       , Static (DamageRule AnyDamage (DealtBy (AttachHost Enchanted (TypeW Creature))) Everywhere (Prevent CutAll Nothing) Repeatedly) ]
       Nothing

public export
sphereOfPurity : Card
sphereOfPurity =
  Macros.card "Sphere of Purity" (Just [Macros.generic 3, Macros.pip White]) []
       (MkTypeLine [] [Enchantment])
       [ Static (DamageRule AnyDamage (DealtBy (Macros.a Macros.artifact)) (ToRecipient You) (Prevent (CutSome (Lit 1)) Nothing) Repeatedly) ]
       Nothing

public export
dazzlingReflection : Card
dazzlingReflection =
  Macros.card "Dazzling Reflection" (Just [Macros.generic 1, Macros.pip White]) []
       (MkTypeLine [] [Instant])
       [ Spell Nothing (Sequentially
                  [ Macros.gainsLife You (StatOf Power (Macros.target Macros.creature))
                  , Continuously
                      (DamageRule AnyDamage (DealtBy (Macros.That (TypeW Creature) OneOf)) Everywhere (Prevent CutAll Nothing) NextTimeOnly)
                      (Just ThisTurn) ]) ]
       Nothing

public export
thunderstaff : Card
thunderstaff =
  Macros.card "Thunderstaff" (Just [Macros.generic 3]) []
       (MkTypeLine [] [Artifact])
       [ Static (Macros.onlyWhile (DamageRule CombatOnly (DealtBy (Macros.a Macros.creature)) (ToRecipient You) (Prevent (CutSome (Lit 1)) Nothing) Repeatedly)
                                 (Matches Macros.thisArtifact Macros.untapped))
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
                      [ Continuously
                          (Becomes Macros.thisPlaneswalker Sets (Bundle (MkToken (Just (Lit 5 ** Lit 5)) []
                                             (MkTypeLine [creatureType "Human", creatureType "Soldier", creatureType "Ally"] [Creature])
                                             [Macros.keyword "Indestructible"] Nothing) (Just Planeswalker)))
                          (Just Macros.untilEndOfTurn)
                      , Macros.preventAll AnyDamage (ToRecipient ((Macros.It OneOf)))
                                          (Just ThisTurn) ])
       , Macros.activated (LoyaltySymbol LoyaltyZero)
                          (Macros.create (Lit 1) (Macros.creatureTok 2 2 [White] [creatureType "Knight", creatureType "Ally"]))
       , Macros.activated (LoyaltySymbol (LoyaltyDown 4))
                          (GetsEmblem You [Static (Gets Adds (Macros.allOf Macros.creatureYouControl)
                                                 (PtUp (Lit 1)) (PtUp (Lit 1)))]) ]
       (Macros.loyaltyBox 4)

public export
turnTheTables : Card
turnTheTables =
  Macros.card "Turn the Tables"
       (Just [Macros.generic 3, Macros.pip White, Macros.pip White]) []
       (MkTypeLine [] [Instant])
       [ Spell Nothing (Continuously
                  (DamageRule CombatOnly Unattributed (ToRecipient You) (Redirect CutAll (Macros.target (And [Macros.creature, Attacking]))) Repeatedly)
                  (Just ThisTurn)) ]
       Nothing

public export
pariah : Card
pariah =
  Macros.card "Pariah" (Just [Macros.generic 2, Macros.pip White]) []
       (MkTypeLine [enchantmentType "Aura"] [Enchantment])
       [ Macros.keywordSubject "Enchant" Macros.creature
       , Static (DamageRule AnyDamage Unattributed (ToRecipient You) (Redirect CutAll (AttachHost Enchanted (TypeW Creature))) Repeatedly) ]
       Nothing

public export
martyrsOfKorlis : Card
martyrsOfKorlis =
  Macros.card "Martyrs of Korlis"
       (Just [Macros.generic 3, Macros.pip White, Macros.pip White]) []
       (MkTypeLine [creatureType "Human"] [Creature])
       [ Static (Macros.onlyWhile (DamageRule AnyDamage (DealtBy (Macros.allOf Macros.artifact)) (ToRecipient You) (Redirect CutAll Macros.thisCreature) Repeatedly)
                                 (Matches Macros.thisCreature Macros.untapped)) ]
       (Just (1, 6))

public export
wardOfPiety : Card
wardOfPiety =
  Macros.card "Ward of Piety" (Just [Macros.generic 1, Macros.pip White]) []
       (MkTypeLine [enchantmentType "Aura"] [Enchantment])
       [ Macros.keywordSubject "Enchant" Macros.creature
       , Macros.activated (Mana [Macros.generic 1, Macros.pip White])
                          (Continuously
                      (DamageRule AnyDamage Unattributed (ToRecipient (AttachHost Enchanted (TypeW Creature))) (Redirect (Shield (Lit 1)) (Macros.target Macros.anyTarget)) Repeatedly)
                      (Just ThisTurn)) ]
       Nothing

public export
mirrorwoodTreefolk : Card
mirrorwoodTreefolk =
  Macros.card "Mirrorwood Treefolk" (Just [Macros.generic 3, Macros.pip Green]) []
       (MkTypeLine [creatureType "Treefolk"] [Creature])
       [ Macros.activated (Mana [Macros.generic 2, Macros.pip Red, Macros.pip White])
                          (Continuously
                      (DamageRule AnyDamage Unattributed (ToRecipient Macros.thisCreature) (Redirect CutAll (Macros.target Macros.anyTarget)) NextTimeOnly)
                      (Just ThisTurn)) ]
       (Just (2, 4))

public export
carom : Card
carom =
  Macros.card "Carom" (Just [Macros.generic 1, Macros.pip White]) []
       (MkTypeLine [] [Instant])
       [ Spell Nothing (Sequentially
                  [ Continuously
                      (DamageRule AnyDamage Unattributed (ToRecipient (Macros.target Macros.creature)) (Redirect (Shield (Lit 1)) (Macros.target (And [Macros.creature, Other]))) Repeatedly)
                      (Just ThisTurn)
                  , (Draw You (Lit 1)) ]) ]
       Nothing

public export
daughterOfAutumn : Card
daughterOfAutumn =
  Macros.card "Daughter of Autumn"
       (Just [Macros.generic 2, Macros.pip Green, Macros.pip Green]) [Legendary]
       (MkTypeLine [creatureType "Avatar"] [Creature])
       [ Macros.activated (Mana [Macros.pip White])
                          (Continuously
                      (DamageRule AnyDamage Unattributed (ToRecipient
                                    (Macros.target (And [Macros.creature, ColorIs White]))) (Redirect (Shield (Lit 1)) Macros.thisCreature) Repeatedly)
                      (Just ThisTurn)) ]
       (Just (2, 4))

public export
aegisOfHonor : Card
aegisOfHonor =
  Macros.card "Aegis of Honor" (Just [Macros.pip White]) []
       (MkTypeLine [] [Enchantment])
       [ Macros.activated (Mana [Macros.generic 1])
                          (Continuously
                      (DamageRule AnyDamage (DealtBy (Macros.a Macros.instantOrSorcery)) (ToRecipient You) (Redirect CutAll (Macros.controllerOf ((Macros.It OneOf)))) NextTimeOnly)
                      (Just ThisTurn)) ]
       Nothing

public export
candlesGlow : Card
candlesGlow =
  Macros.card "Candles' Glow" (Just [Macros.generic 1, Macros.pip White]) []
       (MkTypeLine [spellType "Arcane"] [Instant])
       [ Spell Nothing (Continuously
                  (DamageRule AnyDamage Unattributed (ToRecipient (Macros.target Macros.anyTarget)) (Prevent (Shield (Lit 3)) (Just (Macros.gainsLife You Macros.preventedThisWay))) Repeatedly)
                  (Just ThisTurn)) ]
       Nothing

||| Inkshield
inkshieldRider : Instruction []
inkshieldRider =
  Continuously
    (DamageRule AnyDamage Unattributed (ToRecipient You) (Prevent CutAll (Just (Macros.create Macros.preventedThisWay
                                   (Macros.creatureTok 2 1 [White, Black] [])))) Repeatedly)
    (Just ThisTurn)

public export
urzasArmor : Card
urzasArmor =
  Macros.card "Urza's Armor" (Just [Macros.generic 6]) []
       (MkTypeLine [] [Artifact])
       [ Static (DamageRule AnyDamage (DealtBy (Macros.a Macros.source)) (ToRecipient You) (Prevent (CutSome (Lit 1)) Nothing) Repeatedly) ]
       Nothing

public export
circleOfProtectionRed : Card
circleOfProtectionRed =
  Macros.card "Circle of Protection: Red"
       (Just [Macros.generic 1, Macros.pip White]) []
       (MkTypeLine [] [Enchantment])
       [ Macros.activated (Mana [Macros.generic 1])
                          (Continuously
                      (DamageRule AnyDamage (DealtBy (Macros.aYourChoice
                                                (And [Macros.source, ColorIs Red]))) (ToRecipient You) (Prevent CutAll Nothing) NextTimeOnly)
                      (Just ThisTurn)) ]
       Nothing

public export
healingGrace : Card
healingGrace =
  Macros.card "Healing Grace" (Just [Macros.pip White]) []
       (MkTypeLine [] [Instant])
       [ Spell Nothing (Sequentially
                  [ Continuously
                      (DamageRule AnyDamage (DealtBy (Macros.aYourChoice Macros.source)) (ToRecipient (Macros.target Macros.anyTarget)) (Prevent (Shield (Lit 3)) Nothing) Repeatedly)
                      (Just ThisTurn)
                  , Macros.gainsLife You (Lit 3) ]) ]
       Nothing

public export
reverseDamage : Card
reverseDamage =
  Macros.card "Reverse Damage"
       (Just [Macros.generic 1, Macros.pip White, Macros.pip White]) []
       (MkTypeLine [] [Instant])
       [ Spell Nothing (Continuously
                  (DamageRule AnyDamage (DealtBy (Macros.aYourChoice Macros.source)) (ToRecipient You) (Prevent CutAll (Just (Macros.gainsLife You Macros.preventedThisWay))) NextTimeOnly)
                  (Just ThisTurn)) ]
       Nothing

public export
deflectingPalm : Card
deflectingPalm =
  Macros.card "Deflecting Palm" (Just [Macros.pip Red, Macros.pip White]) []
       (MkTypeLine [] [Instant])
       [ Spell Nothing (Continuously
                  (DamageRule AnyDamage (DealtBy (Macros.aYourChoice Macros.source)) (ToRecipient You) (Prevent CutAll (Just (DealDamage This ThatMuch (Macros.controllerOf ((Macros.It OneOf)))))) NextTimeOnly)
                  (Just ThisTurn)) ]
       Nothing

public export
templeAltisaur : Card
templeAltisaur =
  Macros.card "Temple Altisaur"
       (Just [Macros.generic 4, Macros.pip White]) []
       (MkTypeLine [creatureType "Dinosaur"] [Creature])
       [ Static (DamageRule AnyDamage (DealtBy (Macros.a Macros.source)) (ToRecipient
                                 (Macros.a (And [HasSubtype (creatureType "Dinosaur"),
                                                 HasPossessor ControllerAx You]))) (Prevent (CutAllBut (Lit 1)) Nothing) Repeatedly) ]
       (Just (3, 4))

public export
darkSphere : Card
darkSphere =
  Macros.card "Dark Sphere" (Just []) []
       (MkTypeLine [] [Artifact])
       [ Macros.activated
           (Compound [TapSymbol, Do (Macros.sacrifice You Macros.thisArtifact)])
                          (Continuously
                             (DamageRule AnyDamage (DealtBy (Macros.aYourChoice Macros.source)) (ToRecipient You) (Prevent (CutHalf RoundDown) Nothing) NextTimeOnly)
                             (Just ThisTurn)) ]
       Nothing

public export
shadowbane : Card
shadowbane =
  Macros.card "Shadowbane" (Just [Macros.generic 1, Macros.pip White]) []
       (MkTypeLine [] [Instant])
       [ Spell Nothing (Continuously
                  (DamageRule AnyDamage (DealtBy (Macros.aYourChoice Macros.source)) (ToRecipient
                                   (Macros.youAnd (Macros.allOf Macros.creatureYouControl))) (Prevent CutAll (Just (If (PreventedFromSource
                                             (And [Macros.source, ColorIs Black]))
                                          (Macros.gainsLife You Macros.preventedThisWay)
                                          Nothing))) NextTimeOnly)
                  (Just ThisTurn)) ]
       Nothing

||| Honorable Passage

public export
sphereOfLaw : Card
sphereOfLaw =
  Macros.card "Sphere of Law" (Just [Macros.generic 3, Macros.pip White]) []
       (MkTypeLine [] [Enchantment])
       [ Static (DamageRule AnyDamage (DealtBy (Macros.a (And [Macros.source, ColorIs Red]))) (ToRecipient You) (Prevent (CutSome (Lit 2)) Nothing) Repeatedly) ]
       Nothing

public export
lavaAxe : Card
lavaAxe =
  Macros.card "Lava Axe" (Just [Macros.generic 4, Macros.pip Red]) []
       (MkTypeLine [] [Sorcery])
       [ Spell Nothing (DealDamage This (Lit 5)
                           (Macros.target (Joined (HasType Planeswalker) AnyPlayer))) ]
       Nothing

public export
searingFlesh : Card
searingFlesh =
  Macros.card "Searing Flesh" (Just [Macros.generic 6, Macros.pip Red]) []
       (MkTypeLine [] [Sorcery])
       [ Spell Nothing (DealDamage This (Lit 7)
                           (Macros.target (Joined (HasType Planeswalker) Opponent))) ]
       Nothing

public export
onakkeJavelineerBolt : Ability
onakkeJavelineerBolt =
  Macros.activated TapSymbol
                   (DealDamage Macros.thisCreature (Lit 2)
                        (Macros.target (Joined (HasType Battle) AnyPlayer)))

||| Firesong and Sunspeaker
public export
firesongJoinEcho : Instruction []
firesongJoinEcho =
  Sequentially [ DealDamage This (Lit 3)
                   (Macros.target (Joined Macros.creature AnyPlayer))
               , DealDamage This (Lit 1) (Macros.thatJoin) ]

||| Forcefield
public export
forcefield : Card
forcefield =
  Macros.card "Forcefield" (Just [Macros.generic 3]) []
       (MkTypeLine [] [Artifact])
       [ Macros.activated (Mana [Macros.generic 1])
           (Continuously
              (DamageRule CombatOnly (DealtBy (Macros.aYourChoice
                                        (And [Macros.creature, Macros.unblocked]))) (ToRecipient You) (Prevent (CutAllBut (Lit 1)) Nothing) NextTimeOnly)
              (Just ThisTurn)) ]
       Nothing

public export
endure : Card
endure =
  Macros.card "Endure"
       (Just [Macros.generic 3, Macros.pip White, Macros.pip White]) []
       (MkTypeLine [] [Instant])
       [ Spell Nothing (Macros.preventAll AnyDamage
                                  (ToRecipient
                                     (Macros.youAnd (Macros.allOf (And [Permanent,
                                                          HasPossessor ControllerAx You]))))
                                  (Just ThisTurn)) ]
       Nothing

public export
harmsWay : Card
harmsWay =
  Macros.card "Harm's Way" (Just [Macros.pip White]) []
       (MkTypeLine [] [Instant])
       [ Spell Nothing (Continuously
                  (DamageRule AnyDamage (DealtBy (Macros.aYourChoice Macros.source)) (ToRecipient
                                (Macros.youAnd (Macros.allOf (And [Permanent,
                                                     HasPossessor ControllerAx You])))) (Redirect (Shield (Lit 2)) (Macros.target Macros.anyTarget)) Repeatedly)
                  (Just ThisTurn)) ]
       Nothing

public export
divineDeflection : Card
divineDeflection =
  Macros.card "Divine Deflection" (Just [Variable, Macros.pip White]) []
       (MkTypeLine [] [Instant])
       [ Spell Nothing (Continuously
                  (DamageRule AnyDamage Unattributed (ToRecipient
                               (Macros.youAnd (Macros.allOf (And [Permanent,
                                                    HasPossessor ControllerAx You])))) (Prevent (Shield (LetterVal X)) (Just (DealDamage This ThatMuch
                                              (Macros.target Macros.anyTarget)))) Repeatedly)
                  (Just ThisTurn)) ]
       Nothing

public export
glarecasterShield : Ability
glarecasterShield =
  Macros.activated (Mana [Macros.generic 5, Macros.pip White])
                   (Continuously
               (DamageRule AnyDamage Unattributed (ToRecipient (Macros.youAnd Macros.thisCreature)) (Redirect CutAll (Macros.target Macros.anyTarget)) NextTimeOnly)
               (Just ThisTurn))

public export
furnaceOfRath : Card
furnaceOfRath =
  Macros.card "Furnace of Rath"
       (Just [Macros.generic 1, Macros.pip Red, Macros.pip Red,
              Macros.pip Red]) []
       (MkTypeLine [] [Enchantment])
       [ Static (DamageRule AnyDamage (DealtBy (Macros.a Macros.source)) (ToRecipient
                           (Macros.a (Joined Permanent AnyPlayer))) (Scale (Multiplied Doubled)) Repeatedly) ]
       Nothing

public export
gratuitousViolence : Card
gratuitousViolence =
  Macros.card "Gratuitous Violence"
       (Just [Macros.generic 2, Macros.pip Red, Macros.pip Red,
              Macros.pip Red]) []
       (MkTypeLine [] [Enchantment])
       [ Static (DamageRule AnyDamage (DealtBy (Macros.a Macros.creatureYouControl)) (ToRecipient
                           (Macros.a (Joined Permanent AnyPlayer))) (Scale (Multiplied Doubled)) Repeatedly) ]
       Nothing

public export
fieryEmancipation : Card
fieryEmancipation =
  Macros.card "Fiery Emancipation"
       (Just [Macros.generic 3, Macros.pip Red, Macros.pip Red,
              Macros.pip Red]) []
       (MkTypeLine [] [Enchantment])
       [ Static (DamageRule AnyDamage (DealtBy (Macros.a (And [Macros.source, HasPossessor ControllerAx You]))) (ToRecipient
                           (Macros.a (Joined Permanent AnyPlayer))) (Scale (Multiplied Tripled)) Repeatedly) ]
       Nothing

public export
sulfuricVapors : Card
sulfuricVapors =
  Macros.card "Sulfuric Vapors"
       (Just [Macros.generic 3, Macros.pip Red]) []
       (MkTypeLine [] [Enchantment])
       [ Static (DamageRule AnyDamage (DealtBy (Macros.a (And [Macros.spell, ColorIs Red]))) (ToRecipient
                           (Macros.a (Joined Permanent AnyPlayer))) (Scale (Shifted ShiftUp (Lit 1))) Repeatedly) ]
       Nothing

public export
lashknifeBarrier : Card
lashknifeBarrier =
  Macros.card "Lashknife Barrier"
       (Just [Macros.generic 2, Macros.pip White]) []
       (MkTypeLine [] [Enchantment])
       [ Macros.triggered When (Enters Macros.thisEnchantment Nothing) (Draw You (Lit 1))
       , Static (DamageRule AnyDamage (DealtBy (Macros.a Macros.source)) (ToRecipient
                           (Macros.a Macros.creatureYouControl)) (Scale (Shifted ShiftDown (Lit 1))) Repeatedly) ]
       Nothing

public export
ghostsOfTheInnocent : Card
ghostsOfTheInnocent =
  Macros.card "Ghosts of the Innocent"
       (Just [Macros.generic 5, Macros.pip White, Macros.pip White]) []
       (MkTypeLine [creatureType "Spirit"] [Creature])
       [ Static (DamageRule AnyDamage (DealtBy (Macros.a Macros.source)) (ToRecipient
                           (Macros.a (Joined Permanent AnyPlayer))) (Scale (Halved RoundDown)) Repeatedly) ]
       (Just (4, 5))

public export
fireServant : Card
fireServant =
  Macros.card "Fire Servant"
       (Just [Macros.generic 3, Macros.pip Red, Macros.pip Red]) []
       (MkTypeLine [creatureType "Elemental"] [Creature])
       [ Static (DamageRule AnyDamage (DealtBy (Macros.a (And [Macros.instantOrSorcery, ColorIs Red,
                                        HasPossessor ControllerAx You]))) Everywhere (Scale (Multiplied Doubled)) Repeatedly) ]
       (Just (4, 3))

public export
platedPegasus : Card
platedPegasus =
  Macros.card "Plated Pegasus" (Just [Macros.generic 2, Macros.pip White]) []
       (MkTypeLine [creatureType "Pegasus"] [Creature])
       [ Macros.keyword "Flash"
       , Macros.keyword "Flying"
       , Static (DamageRule AnyDamage (DealtBy (Macros.a Macros.spell)) (ToRecipient
                                 (Macros.a (Joined Permanent AnyPlayer))) (Prevent (CutSome (Lit 1)) Nothing) Repeatedly) ]
       (Just (1, 2))

public export
spitemare : Card
spitemare =
  Macros.card "Spitemare"
       (Just [Macros.generic 2, Macros.hybridPip Red White,
              Macros.hybridPip Red White]) []
       (MkTypeLine [creatureType "Elemental"] [Creature])
       [ Macros.triggered Whenever
                          (IsDealtDamage AnyDamage Macros.thisCreature)
                          (DealDamage ((Macros.It OneOf)) ThatMuch (Macros.target Macros.anyTarget)) ]
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
                          (DealDamage ((Macros.It OneOf)) ThatMuch
                               (Macros.target (Joined (HasType Planeswalker) Opponent))) ]
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
                               (Macros.controllerOf (Macros.That (TypeW Creature) OneOf))) ]
       Nothing

public export
spitefulShadows : Card
spitefulShadows =
  Macros.card "Spiteful Shadows" (Just [Macros.generic 1, Macros.pip Black]) []
       (MkTypeLine [enchantmentType "Aura"] [Enchantment])
       [ Macros.keywordSubject "Enchant" Macros.creature
       , Macros.triggered Whenever
                          (IsDealtDamage AnyDamage (AttachHost Enchanted (TypeW Creature)))
                          (DealDamage ((Macros.It OneOf)) ThatMuch (Macros.controllerOf ((Macros.It OneOf)))) ]
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
                               (Macros.controllerOf (Macros.That (TypeW Creature) OneOf))) ]
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
screamingNemesis : Card
screamingNemesis =
  Macros.card "Screaming Nemesis"
       (Just [Macros.generic 2, Macros.pip Red]) []
       (MkTypeLine [creatureType "Spirit"] [Creature])
       [ Macros.keyword "Haste"
       , Macros.triggered Whenever
                          (IsDealtDamage AnyDamage Macros.thisCreature)
                          (Sequentially
                             [ DealDamage ((Macros.It OneOf)) ThatMuch
                                 (Macros.target (And [Macros.anyTarget,
                                                      OtherThan This]))
                             , If (DealtThisWay AnyPlayer)
                                  (Continuously (Macros.playerCant "GainLife" They)
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
                             [ DealDamage ((Macros.It OneOf)) (Lit 2)
                                 (Macros.target Macros.anyTarget)
                             , Macros.gainsLife You (Lit 2)
                             , If (DealtThisWay AnyPlayer)
                                  ((Macros.discard They (Macros.a (InZone Macros.handZ))))
                                  Nothing ]) ]
       (Just (4, 4))

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
cursedScroll : Card
cursedScroll =
  Macros.card "Cursed Scroll" (Just [Macros.generic 1]) []
       (MkTypeLine [] [Artifact])
       [ Macros.activated (Compound [Mana [Macros.generic 3], TapSymbol])
                          (Sequentially
                      [ Macros.choose (Macros.a (Macros.quality CardName))
                      , Macros.revealCards
                          (Macros.aAtRandom (InZone (Macros.handOf You)))
                      , If (Matches (Macros.That CardW OneOf) (Named ChosenName))
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
                      , If (Matches (Macros.That CardW OneOf) (Named ChosenName))
                           (DealDamage Macros.thisCreature (Lit 2)
                              (Macros.target Macros.anyTarget))
                           Nothing ]) ]
       (Just (1, 1))

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
                          (DealDamage ((Macros.It OneOf)) ThatMuch (Macros.the Macros.chosenPlayer))
       , Macros.activated TapSymbol
                          (DealDamage Macros.thisCreature (Lit 1)
                                      Macros.thisCreature) ]
       (Just (0, 1))

public export
saheeliRaiPlusOne : Instruction []
saheeliRaiPlusOne =
  Sequentially [ Macros.scry You (Lit 1)
               , DealDamage This (Lit 1) (Macros.each Opponent) ]

public export
sarkhansUnsealingLine : Ability
sarkhansUnsealingLine =
  Macros.triggered Whenever
    (Casts You (Macros.a (And [Macros.creature, Macros.spell,
                               Or [Compare [StatAxis Power] Eq (Lit 4),
                                   Compare [StatAxis Power] Eq (Lit 5),
                                   Compare [StatAxis Power] Eq (Lit 6)]])) Nothing)
    (DealDamage Macros.thisEnchantment (Lit 4) (Macros.target Macros.anyTarget))

||| Savage Swipe, both sentences
public export
savageSwipeLine : Instruction []
savageSwipeLine =
  Sequentially
    [ OnlyIf (Macros.gets (Macros.target Macros.creatureYouControl) (PtUp (Lit 2))
                          (PtUp (Lit 2)) (Just Macros.untilEndOfTurn))
             (CompareAmt (StatOf Power ((Macros.It OneOf))) Eq (Lit 2)) Nothing
    , Fights ((Macros.It OneOf)) (Macros.target Macros.creatureYouDontControl) ]

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
                            (StatBecomes ((Macros.It OneOf)) Power (Lit 20))
                            (DealDamage ((Macros.It OneOf)) (Lit 20) (Macros.target Macros.anyTarget))) ]
       (Just (6, 6))

public export
incinerate : Card
incinerate =
  Macros.card "Incinerate" (Just [Macros.generic 1, Macros.pip Red]) []
       (MkTypeLine [] [Instant])
       [ Spell Nothing (Sequentially
                  [ DealDamage This (Lit 3) (Macros.target Macros.anyTarget)
                  , Continuously
                      (Macros.objectCant "Regenerate"
                         (Macros.a (And [Macros.creature,
                                         HappenedTo (MkLookback DamageTaken Lookback.ThisWay Nothing)])))
                      (Just ThisTurn) ]) ]
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
           (DealDamage Macros.thisCreature (Lit 3) (Macros.That PlayerW OneOf)) ]
       (Just (2, 2))

public export
galvanicBlastLine : Instruction []
galvanicBlastLine =
  InsteadOf (DealDamage This (Lit 2) (Macros.target Macros.anyTarget))
            (OnlyIf (DealDamage This (Lit 4) (Macros.thatJoin))
                    (CompareAmt (Macros.countOf (And [Macros.artifact, HasPossessor ControllerAx You]))
                                AtLeast (Lit 3))
                    Nothing)

||| Furious Reprisal
public export
furiousReprisal : Instruction []
furiousReprisal =
  DealDamage This (Lit 2) (EachOf (Described (TargetDet (Macros.exactly 2)) Macros.anyTarget))

||| Bullseye, Death Dealer
public export
bullseyeModalCost : Ability
bullseyeModalCost =
  Macros.activated (Compound [Mana [Macros.generic 3], TapSymbol,
                       Do (Macros.chooseModes (Macros.exactly 1)
                             [Macros.sacrifice You (Macros.a Macros.artifact),
                              Macros.discard You
                                (Macros.a (And [Not Macros.land,
                                                InZone Macros.handZ]))])])
                   (DealDamage This (Lit 2) (Macros.target Macros.anyTarget))

||| Twinshot Sniper
public export
twinshotSniper : Card
twinshotSniper =
  Macros.card "Twinshot Sniper"
       (Just [Macros.generic 3, Macros.pip Red]) []
       (MkTypeLine [creatureType "Goblin", creatureType "Archer"] [Artifact, Creature])
       [ Macros.keyword "Reach"
       , Macros.triggered When (Enters Macros.thisCreature Nothing)
           (DealDamage ((Macros.It OneOf)) (Lit 2) (Macros.target Macros.anyTarget))
       , Macros.abilityWord "channel"
           (Macros.activated (Compound [Mana [Macros.generic 1, Macros.pip Red],
                                        Do (Macros.discard You This)])
                             (DealDamage ((Macros.It OneOf)) (Lit 2) (Macros.target Macros.anyTarget))) ]
       (Just (2, 3))

||| Quakebringer
public export
quakebringerDamage : Ability
quakebringerDamage =
  Macros.triggeredIf At (BeginningOf ThePart Upkeep (ByPlayer You))
    (OrCond [ Matches This (InZone Macros.battlefieldZ)
            , AndCond [ Matches This (InZone (Macros.graveyardOf You))
                      , Macros.exists (And [Macros.creature,
                                     HasSubtype (creatureType "Giant"),
                                     HasPossessor ControllerAx You]) ] ])
    (DealDamage This (Lit 2) (Macros.each Opponent))

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

||| Lavalanche
public export
lavalanche : Instruction []
lavalanche =
  Simultaneously
    [ DealDamage This (LetterVal X) Description.targetPlayerOrPlaneswalker
    , DealDamage This (LetterVal X) Anaphora.eachCreatureThatSplitControls ]

||| Flame Wave
public export
flameWave : Instruction []
flameWave =
  Simultaneously
    [ DealDamage This (Lit 4) Description.targetPlayerOrPlaneswalker
    , DealDamage This (Lit 4) Anaphora.eachCreatureThatSplitControls ]

||| Chandra Nalaar's ultimate
public export
chandraNalaarUltimate : Instruction []
chandraNalaarUltimate =
  Simultaneously
    [ DealDamage This (Lit 10) Description.targetPlayerOrPlaneswalker
    , DealDamage This (Lit 10) Anaphora.eachCreatureThatSplitControls ]

||| Chandra, Pyrogenius's ultimate
public export
chandraPyrogeniusUltimate : Instruction []
chandraPyrogeniusUltimate =
  Simultaneously
    [ DealDamage This (Lit 6) Description.targetPlayerOrPlaneswalker
    , DealDamage This (Lit 6) Anaphora.eachCreatureThatSplitControls ]

||| Bonfire of the Damned
public export
bonfireOfTheDamned : Instruction []
bonfireOfTheDamned =
  Simultaneously
    [ DealDamage This (LetterVal X) Description.targetPlayerOrPlaneswalker
    , DealDamage This (LetterVal X) Anaphora.eachCreatureThatSplitControls ]

||| Chandra's Fury
public export
chandrasFury : Instruction []
chandrasFury =
  Simultaneously
    [ DealDamage This (Lit 4) Description.targetPlayerOrPlaneswalker
    , DealDamage This (Lit 1) Anaphora.eachCreatureThatSplitControls ]


||| Angrath, Minotaur Pirate's plus
public export
angrathMinotaurPirateBolt : Instruction []
angrathMinotaurPirateBolt =
  Simultaneously
    [ DealDamage This (Lit 1) Description.targetOpponentOrPlaneswalker
    , DealDamage This (Lit 1) Anaphora.eachCreatureThatSplitControls ]

public export
whichOfYouBurnsBrightestBody : Instruction []
whichOfYouBurnsBrightestBody =
  Simultaneously
    [ DealDamage This (LetterVal X) Description.targetOpponentOrPlaneswalker
    , DealDamage This (LetterVal X) Anaphora.eachCreatureThatSplitControls ]

||| Chandra, Pyromaster's plus
public export
chandraPyromasterBolt : Instruction []
chandraPyromasterBolt =
  Simultaneously
    [ DealDamage This (Lit 1) Description.targetPlayerOrPlaneswalker
    , DealDamage This (Lit 1)
        (Described (TargetDet (Macros.upTo 1)) (And [Macros.creature,
                           HasPossessor ControllerAx Macros.splitOverPlaneswalker])) ]

||| Ravager of the Fells
public export
ravagerOfTheFellsBolt : Instruction []
ravagerOfTheFellsBolt =
  Simultaneously
    [ DealDamage Macros.thisCreature (Lit 2) Description.targetOpponentOrPlaneswalker
    , DealDamage Macros.thisCreature (Lit 2)
        (Described (TargetDet (Macros.upTo 1)) (And [Macros.creature,
                           HasPossessor ControllerAx Macros.splitOverPlaneswalker])) ]

||| Soul of Shandalar's battlefield activation
public export
soulOfShandalarBolt : Instruction []
soulOfShandalarBolt =
  Simultaneously
    [ DealDamage Macros.thisCreature (Lit 3) Description.targetPlayerOrPlaneswalker
    , DealDamage Macros.thisCreature (Lit 3)
        (Described (TargetDet (Macros.upTo 1)) (And [Macros.creature,
                           HasPossessor ControllerAx Macros.splitOverPlaneswalker])) ]

||| Soul of Shandalar's graveyard activation
public export
soulOfShandalarGraveyardBolt : Instruction []
soulOfShandalarGraveyardBolt =
  Simultaneously
    [ DealDamage This (Lit 3) Description.targetPlayerOrPlaneswalker
    , DealDamage This (Lit 3)
        (Described (TargetDet (Macros.upTo 1)) (And [Macros.creature,
                           HasPossessor ControllerAx Macros.splitOverPlaneswalker])) ]

||| Blightning
public export
blightning : Instruction []
blightning =
  Sequentially
    [ DealDamage This (Lit 3) Description.targetPlayerOrPlaneswalker
    , Macros.discard Macros.splitOverPlaneswalker
        (Macros.counted (Macros.exactly 2) (InZone Macros.handZ)) ]

||| Rakdos's Return
public export
rakdossReturn : Instruction []
rakdossReturn =
  Sequentially
    [ DealDamage This (LetterVal X) Description.targetOpponentOrPlaneswalker
    , Macros.discard Macros.splitOverPlaneswalker
        (Macros.counted (ExactlyOf (LetterVal X)) (InZone Macros.handZ)) ]

||| Nicol Bolas, Planeswalker's ultimate
public export
nicolBolasUltimate : Instruction []
nicolBolasUltimate =
  Sequentially
    [ DealDamage This (Lit 7) Description.targetPlayerOrPlaneswalker
    , Macros.discard Macros.splitOverPlaneswalker
        (Macros.counted (Macros.exactly 7) (InZone Macros.handZ))
    , Macros.sacrifice Macros.splitOverPlaneswalker
        (Macros.counted (Macros.exactly 7) Permanent) ]

||| Pulse of the Forge
public export
pulseOfTheForge : Instruction []
pulseOfTheForge =
  Sequentially
    [ DealDamage This (Lit 4) Description.targetPlayerOrPlaneswalker
    , If (CompareAmt (PlayerStatOf LifeTotal Macros.splitOverPlaneswalker)
                     Greater (PlayerStatOf LifeTotal You))
         (Macros.move This Macros.handZ)
         Nothing ]

||| Goblin Lyre's losing arm
public export
goblinLyreLoseFlip : Instruction []
goblinLyreLoseFlip =
  Sequentially
    [ DealDamage Macros.thisArtifact (Macros.countOf Macros.creatureYouControl)
                 Description.targetOpponentOrPlaneswalker
    , DealDamage Macros.thisArtifact
                 (Macros.countOf (And [Macros.creature,
                                HasPossessor ControllerAx Macros.splitOverPlaneswalker]))
                 You ]

||| Quenchable Fire
public export
quenchableFire : Instruction []
quenchableFire =
  Sequentially
    [ DealDamage This (Lit 3) Description.targetPlayerOrPlaneswalker
    , Macros.delayed (BeginningOf ThePart Upkeep (ByPlayer You))
        (Macros.unless Macros.splitOverPlaneswalker
                (DealDamage This (Lit 3) Macros.thatJoin)
                (Mana [Macros.pip Blue])) ]

||| Searing Blaze, both sentences
public export
searingBlaze : Ability
searingBlaze =
  Macros.abilityWord "landfall"
    (Spell Nothing
      (InsteadOf
        (Simultaneously
           [ DealDamage This (Lit 1) Description.targetPlayerOrPlaneswalker
           , DealDamage This (Lit 1)
               (Macros.target (And [Macros.creature,
                                    HasPossessor ControllerAx Macros.splitOverPlaneswalker])) ])
        (Simultaneously
           [ DealDamage This (Lit 3) Macros.thatJoin
           , DealDamage This (Lit 3) (Macros.That (TypeW Creature) OneOf) ])))

||| Heart of Bogardan
public export
heartOfBogardan : Card
heartOfBogardan =
  Macros.card "Heart of Bogardan"
       (Just [Macros.generic 2, Macros.pip Red, Macros.pip Red]) []
       (MkTypeLine [] [Enchantment])
       [ Macros.cumulativeUpkeep (Mana [Macros.generic 2])
       , Macros.triggered When Trigger.heartOfBogardanHeader
           (Sequentially
              [ Simultaneously
                  [ DealDamage This (LetterVal X) Description.targetPlayerOrPlaneswalker
                  , DealDamage This (LetterVal X) Anaphora.eachCreatureThatSplitControls ]
              , Define X (Minus (Macros.times 2 (CountersOn (NamedCounter "Age") Macros.thisEnchantment)) (Lit 2)) ]) ]
       Nothing

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
                                           Macros.untapped])))) False)
       , Spell Nothing (DealDamage This (Macros.times 3 GroupSize)
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
                                           Macros.untapped])))) True)
       , Static (Costs This (CostLess (Macros.times 1 GroupSize) Nothing))
       , Spell Nothing (DealDamage This (Lit 10) (Macros.target Macros.anyTarget)) ]
       Nothing

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
           (Macros.costWasPaid (ByKeyword "Surge") Nothing Macros.thisCreature)
           (DealDamage Macros.thisCreature (Lit 3)
                       (Macros.target Macros.anyTarget)) ]
       (Just (5, 4))

||| Fall of the Titans
public export
fallOfTheTitansCard : Card
fallOfTheTitansCard =
  Macros.card "Fall of the Titans"
       (Just [Variable, Variable, Macros.pip Red]) []
       (MkTypeLine [] [Instant])
       [ Macros.keywordCosting "Surge" (Mana [Variable, Macros.pip Red])
       , Spell Nothing (DealDamage This (LetterVal X)
                  (EachOf (Described (TargetDet (Macros.upTo 2)) Macros.anyTarget))) ]
       Nothing

||| Tribal Flames
public export
tribalFlames : Card
tribalFlames =
  Macros.card "Tribal Flames" (Just [Macros.generic 1, Macros.pip Red]) []
       (MkTypeLine [] [Sorcery])
       [ Macros.abilityWord "domain"
           (Spell Nothing (Sequentially
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
  Macros.abilityWord "vivid"
    (Macros.triggered When (Enters Macros.thisCreature Nothing)
       (Sequentially
          [ DealDamage ((Macros.It OneOf)) (LetterVal X)
                       (Macros.target (And [Macros.creature,
                                            HasPossessor ControllerAx Macros.anOpponent]))
          , Define X (DistinctCount ColorAxis
                        (Macros.allOf (And [Permanent, HasPossessor ControllerAx You]))) ]))

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
       , Macros.gainsLife You (LetterVal X)
       , Define X (DistinctCount ColorPairAxis
                     (Macros.allOf (And [Permanent, HasPossessor ControllerAx You,
                                  ColorCount Eq 2]))) ])

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
           (Draw You (Lit 1))
       , Macros.triggered Whenever
           (AttacksWith (Macros.a AnyPlayer) NoDefender
                        (Macros.counted (Macros.atLeast 5) Macros.creature))
           (Sequentially [ DealDamage This (Lit 3) (Macros.each Opponent)
                         , Macros.gainsLife You (Lit 3) ]) ]
       (Just (4, 4))

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

||| Destructive Revelry
public export
destructiveRevelry : Card
destructiveRevelry =
  Macros.card "Destructive Revelry" (Just [Macros.pip Red, Macros.pip Green]) []
       (MkTypeLine [] [Instant])
       [ Spell Nothing (Sequentially
                  [ Macros.destroy (Macros.target (Or [Macros.artifact, Macros.enchantment]))
                  , DealDamage This (Lit 2) (Macros.controllerOf (Macros.That PermanentW OneOf)) ]) ]
       Nothing

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

||| Breeches, Brazen Plunderer's slice
public export
eachOfThoseOpponentsTopCard : Instruction []
eachOfThoseOpponentsTopCard =
  Sequentially [ DealDamage This (Lit 1) (Macros.each Opponent)
               , Macros.exile You (LibrarySlice OnTop (Lit 1) (EachOf (Macros.That PlayerW ManyOf))) ]

||| Inquisitor's Flail
public export
inquisitorsFlail : Card
inquisitorsFlail =
  Macros.card "Inquisitor's Flail" (Just [Macros.generic 2]) []
       (MkTypeLine [artifactType "Equipment"] [Artifact])
       [ Static (DamageRule CombatOnly (DealtBy (AttachHost Equipped (TypeW Creature))) Everywhere (Scale (Multiplied Doubled)) Repeatedly)
       , Static (DamageRule CombatOnly (DealtBy (Macros.a (Macros.otherCreature
                                     (AttachHost Equipped (TypeW Creature))))) (ToRecipient (AttachHost Equipped (TypeW Creature))) (Scale (Multiplied Doubled)) Repeatedly)
       , Macros.keywordCosting "Equip" (Mana [Macros.generic 2]) ]
       Nothing

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
           (Sequentially [ DealDamage This (Lit 2) (Macros.That PlayerW OneOf)
                         , Macros.gainsLife You (Lit 2) ]) ]
       Nothing

||| Frostwielder
public export
frostwielder : Card
frostwielder =
  Macros.card "Frostwielder"
       (Just [Macros.generic 2, Macros.pip Red, Macros.pip Red]) []
       (MkTypeLine [creatureType "Human", creatureType "Shaman"] [Creature])
       [ Static (Intercepts
                   (Dies (Macros.a (And [ Macros.creature
                                        , HappenedTo (MkLookback DamageTaken ThisTurn (Just (Involving Macros.thisCreature))) ])))
                   [] Nothing (Macros.exile You ((Macros.It OneOf))) Repeatedly Nothing)
       , Macros.activated TapSymbol
           (DealDamage Macros.thisCreature (Lit 1) (Macros.target Macros.anyTarget)) ]
       (Just (1, 2))

||| Crackling Doom
public export
cracklingDoom : Instruction []
cracklingDoom =
  Sequentially
    [ DealDamage This (Lit 2) (Macros.each Opponent)
    , Macros.sacrifice (Macros.each Opponent)
        (Macros.a (And [Macros.creature,
                        Superlative MaxOf (StatAxis Power)
                          (And [Macros.creature,
                                HasPossessor ControllerAx (Macros.That PlayerW OneOf)])])) ]

public export
theFallen : Ability
theFallen =
  Macros.triggered At (BeginningOf ThePart Upkeep (ByPlayer You))
    (DealDamage Macros.thisCreature (Lit 1)
       (Macros.each (And [Joined Opponent (HasType Planeswalker),
                   Macros.happenedToInvolving DamageTaken ThisGame
                     Macros.thisCreature])))

||| Cavalcade of Calamity
public export
cavalcadeOfCalamity : Ability
cavalcadeOfCalamity =
  Macros.triggered Whenever
    (Macros.attacks (Macros.a (And [Macros.creature, HasPossessor ControllerAx You,
                                    Compare [StatAxis Power] AtMost (Lit 1)])))
    (DealDamage Macros.thisEnchantment (Lit 1)
       (Macros.the (And [Joined AnyPlayer (HasType Planeswalker),
                       CombatRel AttackedBy (Macros.That (TypeW Creature) OneOf)])))

||| Blind Fury
public export
blindFury : Card
blindFury =
  Macros.card "Blind Fury"
       (Just [Macros.generic 2, Macros.pip Red, Macros.pip Red]) []
       (MkTypeLine [] [Instant])
       [ Spell Nothing (Sequentially
                  [ Continuously
                      (LosesAbilities (Macros.allOf Macros.creature)
                                      [LostWritten (Macros.keyword "Trample")])
                      (Just Macros.untilEndOfTurn)
                  , Continuously
                      (DamageRule CombatOnly (DealtBy (Macros.a Macros.creature)) (ToRecipient (Macros.a Macros.creature)) (Scale (Multiplied Doubled)) Repeatedly)
                      (Just ThisTurn) ]) ]
       Nothing

||| Chandra, Awakened Inferno's emblem
public export
chandraAwakenedInfernoEmblem : Instruction []
chandraAwakenedInfernoEmblem =
  GetsEmblem You
    [ Macros.triggered At (BeginningOf ThePart Upkeep (ByPlayer You))
        (DealDamage (AsMarker EmblemMarker This) (Lit 1) You) ]

||| Keeper of the Flame
public export
keeperOfTheFlame : Card
keeperOfTheFlame =
  Macros.card "Keeper of the Flame" (Just [Macros.pip Red, Macros.pip Red]) []
       (MkTypeLine [creatureType "Human", creatureType "Wizard"] [Creature])
       [ Macros.activated (Compound [Mana [Macros.pip Red], TapSymbol])
           (Sequentially
              [ Macros.choose
                  (Macros.target
                     (And [ Opponent
                          , Compare [PlayerStatAxis LifeTotal] Greater
                                    (PlayerStatOf LifeTotal You) ]))
              , DealDamage Macros.thisCreature (Lit 2) (Macros.That PlayerW OneOf) ]) ]
       (Just (1, 2))

diabolicEdict : Instruction []
diabolicEdict = Macros.sacrifice (Macros.target AnyPlayer) (Macros.aTheirChoice Macros.creature)

innocentBlood : Instruction []
innocentBlood = Macros.sacrifice (Macros.each AnyPlayer) (Macros.aTheirChoice Macros.creature)

cryOfContrition : Instruction []
cryOfContrition = (Macros.discard (Macros.target AnyPlayer) (Macros.a (InZone Macros.handZ)))

cyclingCost : Instruction []
cyclingCost = Macros.discard You This

raiseTheAlarm : Instruction []
raiseTheAlarm = Macros.create (Lit 2) (Macros.creatureTok 1 1 [White] [creatureType "Soldier"])

actOfTreason : Instruction []
actOfTreason = Macros.gainControl You (Macros.target Macros.creature) (Just Macros.untilEndOfTurn)

wordsOfWorship : Instruction []
wordsOfWorship = Macros.nextTimeWouldInstead (Draws You)
                                      (Macros.gainsLife You (Lit 5))
                                      (Just ThisTurn)

theLastRoninII : Instruction []
theLastRoninII =
  Reflexively (Macros.mills You (Lit 4) You)
              (Macros.move (Macros.target (And [Macros.creature, InZone (Macros.graveyardOf You)])) Macros.handZ)

moonlitWakeCard : Card
moonlitWakeCard =
  Macros.card "Moonlit Wake" (Just [Macros.generic 2, Macros.pip White]) []
       (MkTypeLine [] [Enchantment]) [moonlitWake] Nothing

public export
laquatussDisdain : Card
laquatussDisdain =
  Macros.card "Laquatus's Disdain" (Just [Macros.generic 1, Macros.pip Blue]) []
       (MkTypeLine [] [Instant])
       [ Spell Nothing (Sequentially
           [ CounterSpell
               (Macros.target (And [Macros.spell, CastFrom Macros.graveyardZ]))
           , (Draw You (Lit 1)) ]) ]
       Nothing

||| Stifle
public export
stifle : Card
stifle =
  Macros.card "Stifle" (Just [Macros.pip Blue]) []
       (MkTypeLine [] [Instant])
       [ Spell Nothing (CounterSpell
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
       [ Spell Nothing (CounterSpell
                  (Macros.target
                     (Or [ Macros.spell
                         , AbilityHead AnyActivated
                         , AbilityHead AnyTriggered ]))) ]
       Nothing

||| Fry
public export
fry : Card
fry =
  Macros.card "Fry" (Just [Macros.generic 1, Macros.pip Red]) []
       (MkTypeLine [] [Instant])
       [ Static (Macros.objectCant "Counter" This)
       , Spell Nothing (DealDamage This (Lit 5)
                   (Macros.target (And [Or [Macros.creature, HasType Planeswalker],
                                        Or [ColorIs White, ColorIs Blue]]))) ]
       Nothing

||| Termination Facilitator
public export
terminationFacilitator : Card
terminationFacilitator =
  Macros.card "Termination Facilitator" (Just [Macros.generic 1, Macros.pip Black]) []
       (MkTypeLine [creatureType "Human", creatureType "Assassin"] [Creature])
       [ Macros.activatedOnlyDuring TapSymbol
           (PutCounters (Lit 1) (PrintedKind (NamedCounter "Bounty"))
                        (Macros.target (Or [Macros.creature, HasType Planeswalker])))
           AsSorcery
       , Macros.triggered Whenever
           (IsDealtDamage AnyDamage
              (Macros.a (And [Or [Macros.creature, HasType Planeswalker],
                              HasPossessor ControllerAx Macros.anOpponent,
                              HasCounters (Just (NamedCounter "Bounty"))])))
           (Macros.destroy (Macros.It OneOf)) ]
       (Just (1, 3))
