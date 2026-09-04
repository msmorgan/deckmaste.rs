module Experimental.Cards.Choice

import Experimental
import Experimental.Macros
import Experimental.Cards.Anaphora

%default total


carefulStudy : Instruction []
carefulStudy = Sequentially [(Draw You (Lit 2)), (Repeated (Lit 2) (Sequentially [Macros.choose (Macros.a (InZone Macros.handZ)), Macros.discard You (Macros.That CardW OneOf)]))]

zombieInfestation : Ability
zombieInfestation =
  Macros.activated (Do ((Repeated (Lit 2) (Sequentially [Macros.choose (Macros.a (InZone Macros.handZ)), Macros.discard You (Macros.That CardW OneOf)]))))
                   (Macros.create (Lit 1)
                      (MkToken (Just (Lit 2 ** Lit 2)) [Black]
                               (MkTypeLine [creatureType "Zombie"] [Creature]) [] Nothing))

fulgentDistraction : Instruction []
fulgentDistraction = Sequentially [Macros.choose (Described (TargetDet (Macros.exactly 2)) Macros.creature),
                                   SetStatus Tapped (Macros.That (TypeW Creature) ManyOf)]

continueSpell : Instruction []
continueSpell = Sequentially [Macros.choose (Described (TargetDet (Macros.upTo 4)) (And [Macros.creature, InZone (Macros.graveyardOf You)])),
                              Macros.move ((Macros.It ManyOf)) Macros.battlefieldZ]

kindredDominance : Instruction []
kindredDominance = Sequentially [Macros.choose (Macros.a (Macros.quality (SubtypeQ Creature))),
                                 Macros.destroy (Macros.allOf (And [Macros.creature, Not (Macros.ofChosen (SubtypeQ Creature))]))]

phantomBlade : Instruction []
phantomBlade = Sequentially [Macros.choose (Described (TargetDet (Macros.upTo 1)) (And [Macros.creature, HasPossessor ControllerAx You])),
                             Macros.destroy (Described (TargetDet (Macros.upTo 1)) (And [Macros.creature, Other]))]

||| Braids's Frightful Return
braidsFrightfulReturn : Instruction []
braidsFrightfulReturn =
  (May You (Macros.sacrifice You (Macros.a Macros.creature)) (Just ((Macros.discard (Macros.each Opponent) (Macros.a (InZone Macros.handZ))))) Nothing)

||| Daretti, Ingenious Iconoclast
darettisMinusOne : Instruction []
darettisMinusOne =
  (May You (Macros.sacrifice You (Macros.a Macros.artifact)) (Just (Macros.destroy (Macros.target (Or [Macros.artifact, Macros.creature])))) Nothing)

public export
cheeringFanatic : Card
cheeringFanatic =
  Macros.card "Cheering Fanatic" (Just [Macros.generic 1, Macros.pip Red]) []
       (MkTypeLine [creatureType "Goblin"] [Creature])
       [ Macros.triggered Whenever (Macros.attacks Macros.thisCreature)
                          (Sequentially
                     [ Macros.choose (Macros.a (Macros.quality CardName))
                     , Continuously
                         (CostsToCast (Macros.allOf (And [Macros.spell, Macros.ofChosen CardName]))
                                      (CostLess (Lit 1) Nothing))
                         (Just ThisTurn) ]) ]
       (Just (2, 2))

rainOfThorns : Instruction []
rainOfThorns = Macros.chooseModes (Macros.atLeast 1) [Macros.destroy (Macros.target Macros.artifact),
                                Macros.destroy (Macros.target Macros.enchantment),
                                Macros.destroy (Macros.target Macros.land)]

rankleMasterOfPranks : Instruction []
rankleMasterOfPranks =
  Macros.chooseModes Macros.anyNumber [(Macros.discard (Macros.each AnyPlayer) (Macros.a (InZone Macros.handZ))),
                   Macros.sacrifice (Macros.each AnyPlayer) (Macros.aTheirChoice Macros.creature)]

myrkulsEdict : Instruction []
myrkulsEdict = Sequentially [Macros.choose (Macros.a Opponent),
                             Macros.sacrifice (Macros.That PlayerW OneOf) (Macros.aTheirChoice Macros.creature)]

lookAtTopThenBin : Instruction []
lookAtTopThenBin =
  Sequentially [Macros.lookAt (Macros.topSlice (Lit 1)), Macros.may You (Macros.move (Macros.That CardW OneOf) Macros.graveyardZ)]

moltingHarpy : Instruction []
moltingHarpy = (May You (Pay You (Mana [Macros.generic 2]) PaidOnce) Nothing (Just (Macros.sacrifice You Macros.thisCreature)))

carnophage : Instruction []
carnophage = (May You (Pay You (Macros.payLife You 1) PaidOnce) Nothing (Just (SetStatus Tapped Macros.thisCreature)))

solitaryConfinement : Instruction []
solitaryConfinement = (May You ((Macros.discard You (Macros.a (InZone Macros.handZ)))) Nothing (Just (Macros.sacrifice You Macros.thisEnchantment)))

yasminKhan : Ability
yasminKhan =
  Macros.activated TapSymbol
                   (Sequentially [Macros.exile You (Macros.topSlice (Lit 1)),
                           Continuously ((Macros.mayPlayDeed "Play" You ((Macros.It OneOf)) Nothing (PlayRider Nothing Nothing Nothing False ItsOwnCost))) (Just Macros.untilYourNextEndStep)])

||| Brazen Cannonade
brazenCannonadePermission : Instruction []
brazenCannonadePermission =
  Sequentially [Macros.exile You (Macros.topSlice (Lit 1)),
                Continuously ((Macros.mayPlayDeed "Play" You ((Macros.It OneOf)) Nothing (PlayRider Nothing Nothing Nothing False ItsOwnCost))) (Just (Until (EndOf Combat (Just You))))]

thousandMoonsCrackshot : Ability
thousandMoonsCrackshot =
  Macros.triggered Whenever (Macros.attacks Macros.thisCreature)
    (Macros.mayWhen You (Pay You (Mana [Macros.generic 2, Macros.pip White]) PaidOnce)
                 (SetStatus Tapped (Macros.target Macros.creature)))

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

skaabRuinator : Ability
skaabRuinator =
  Static ((Macros.mayPlayDeed "Cast" You This Nothing (PlayRider (Just (Macros.graveyardOf You)) Nothing Nothing False ItsOwnCost)))

||| Raffine's Guidance
public export
raffinesGuidance : Card
raffinesGuidance =
  Macros.card "Raffine's Guidance" (Just [Macros.pip White]) []
       (MkTypeLine [enchantmentType "Aura"] [Enchantment])
       [ Macros.keywordSubject "Enchant" Macros.creature
       , Static (Gets Adds (AttachHost Enchanted (TypeW Creature))
                      (PtUp (Lit 1)) (PtUp (Lit 1)))
       , Static ((Macros.mayPlayDeed "Cast" You This Nothing (PlayRider (Just (Macros.graveyardOf You)) Nothing Nothing False (PayingInstead (Mana [Macros.generic 2, Macros.pip White]))))) ]
       Nothing

||| Scourge of Nel Toth
public export
scourgeOfNelToth : Card
scourgeOfNelToth =
  Macros.card "Scourge of Nel Toth"
       (Just [Macros.generic 5, Macros.pip Black, Macros.pip Black]) []
       (MkTypeLine [creatureType "Zombie", creatureType "Dragon"] [Creature])
       [ Macros.keyword "Flying"
       , Static ((Macros.mayPlayDeed "Cast" You This Nothing (PlayRider (Just (Macros.graveyardOf You)) Nothing Nothing False (PayingInstead (Compound [ Mana [Macros.pip Black, Macros.pip Black]
                             , Do (Macros.sacrifice You
                                     (Macros.counted (Macros.exactly 2)
                                                   Macros.creature)) ]))))) ]
       (Just (6, 6))

escapeToTheWilds : Instruction []
escapeToTheWilds =
  Sequentially [Macros.exile You ((Macros.topSlice (Lit 5))),
                Continuously
                  ((Macros.mayPlayDeed "Play" You (Macros.TheVerbed "Exile" CardW ThisWay ManyOf) Nothing (PlayRider Nothing Nothing Nothing False ItsOwnCost)))
                  (Just (Until (EndOf Turn (Just You))))]

||| Muse Vessel
museVesselPlay : Ability
museVesselPlay =
  Macros.activated (Mana [Macros.generic 1])
                   (Sequentially [Macros.choose (Macros.a Macros.exiledWithThisArtifact),
                           Continuously ((Macros.mayPlayDeed "Play" You (Macros.That CardW OneOf) Nothing (PlayRider Nothing Nothing Nothing False ItsOwnCost))) (Just ThisTurn)])

demonicConsultationChoice : Instruction []
demonicConsultationChoice = Macros.choose (Macros.a (Macros.quality CardName))

voidChoice : Instruction []
voidChoice = Macros.choose (Macros.a (Macros.quality Number))

ashnodsBattleGear : Ability
ashnodsBattleGear = Static (Macros.mayDeclineUntap Macros.thisArtifact (Just You))

||| Phyrexian Ingester
phyrexianIngesterPump : Ability
phyrexianIngesterPump =
  Static (AndAlso Nothing [ Gets Adds Macros.thisCreature (PtUp (LetterVal X)) (PtUp (LetterVal Y))
                  , DefinesLetter X
                      (StatOf Power
                         (Macros.a (And [Macros.creature,
                                         ExiledWith Macros.thisCreature])))
                  , DefinesLetter Y (StatOf Toughness (Macros.That CardW OneOf)) ])

||| Phyrexian Ingester
phyrexianIngester : Card
phyrexianIngester =
  Macros.card "Phyrexian Ingester"
       (Just [Macros.generic 6, Macros.pip Blue]) []
       (MkTypeLine [creatureType "Phyrexian", creatureType "Beast"] [Creature])
       [ Macros.abilityWord "imprint"
           (Macros.triggered When (Enters Macros.thisCreature Nothing)
                             (Macros.may You
                                (Macros.exile You
                                   (Macros.target
                                      (And [Macros.creature, Macros.nontoken])))))
       , phyrexianIngesterPump ]
       (Just (3, 3))

misterFantastic : Card
misterFantastic =
  Macros.card "Mister Fantastic, Reed Richards"
       (Just [Macros.generic 3, Macros.pip Blue]) [Legendary]
       (MkTypeLine [creatureType "Human", creatureType "Scientist", creatureType "Hero"] [Creature])
       [ Macros.keyword "Reach"
       , Macros.triggered Whenever
                          (Enters (Macros.counted (Macros.atLeast 1)
                                         (And [IsToken, HasPossessor ControllerAx You])) Nothing)
                          (Macros.may You (Draw You (Lit 1))) ]
       (Just (2, 4))

murmursFromBeyond : Card
murmursFromBeyond =
  Macros.card "Murmurs from Beyond"
       (Just [Macros.generic 2, Macros.pip Blue]) []
       (MkTypeLine [spellType "Arcane"] [Instant])
       [ Spell (Sequentially
                  [ Macros.revealCards ((Macros.topSlice (Lit 3)))
                  , Macros.chooses (Macros.a Opponent) (Macros.someOf (Macros.exactly 1) ((Macros.It ManyOf)))
                  , Macros.move (Macros.That CardW OneOf) Macros.graveyardZ
                  , Macros.move (Macros.theRest Object) Macros.handZ ]) ]
       Nothing

||| Helica Glider
helicaGlider : Card
helicaGlider =
  Macros.card "Helica Glider"
       (Just [Macros.generic 2, Macros.pip White]) []
       (MkTypeLine [creatureType "Nightmare", creatureType "Squirrel"] [Creature])
       [ Static (EntersRider Macros.thisCreature (WithCounters (Lit 1)
                   (ChosenKind [ Macros.flyingCounter
                               , KeywordCounter "FirstStrike" ])
                   Fresh)) ]
       (Just (2, 2))

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

||| Rune Snag
public export
runeSnag : Card
runeSnag =
  Macros.card "Rune Snag" (Just [Macros.generic 1, Macros.pip Blue]) []
       (MkTypeLine [] [Instant])
       [ Spell ((May (Macros.controllerOf (Macros.target Macros.spell)) (Pay They (Compound [Mana [Macros.generic 2],
                                       Macros.scaledMana GenericUnit
                                         (Macros.times 2 (Macros.countOf
                                            (And [Named (PrintedName "Rune Snag"),
                                                  InZone Macros.graveyardZ])))])
                       PaidOnce) Nothing (Just (CounterSpell ((Macros.It OneOf)))))) ]
       Nothing

||| Tahngarth
public export
tahngarthChoosesDefender : Instruction []
tahngarthChoosesDefender =
  Choose Nothing (Macros.a (Macros.kindJoin AnyPlayer (HasType Planeswalker))) Openly

public export
reefShaman : Card
reefShaman =
  Macros.card "Reef Shaman" (Just [Macros.pip Blue]) []
       (MkTypeLine [creatureType "Merfolk", creatureType "Shaman"] [Creature])
       [ Macros.activated TapSymbol
                          (Continuously
                             (Becomes (Macros.target Macros.land) Sets (ChosenQuality (OfYourChoice (SubtypeQ Land) (Just BasicTypesOnly))))
                             (Just Macros.untilEndOfTurn)) ]
       (Just (0, 2))

public export
grixisIllusionist : Card
grixisIllusionist =
  Macros.card "Grixis Illusionist" (Just [Macros.pip Blue]) []
       (MkTypeLine [creatureType "Human", creatureType "Wizard"] [Creature])
       [ Macros.activated TapSymbol
                          (Continuously
                      (Becomes (Macros.target (And [Macros.land, HasPossessor ControllerAx You])) Sets (ChosenQuality (OfYourChoice (SubtypeQ Land) (Just BasicTypesOnly))))
                      (Just Macros.untilEndOfTurn)) ]
       (Just (1, 1))

public export
distantMelody : Card
distantMelody =
  Macros.card "Distant Melody" (Just [Macros.generic 3, Macros.pip Blue]) []
       (MkTypeLine [] [Sorcery])
       [ Spell (Sequentially
                  [ Macros.choose (Macros.a (Macros.quality (SubtypeQ Creature)))
                  , Draw You (Macros.forEach 1
                                (And [Permanent, HasPossessor ControllerAx You,
                                      Macros.ofChosen (SubtypeQ Creature)])) ]) ]
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
                                             Not (Macros.ofChosen (SubtypeQ Creature))]))
                                (PtDown (Lit 3)) (PtDown (Lit 3))
                                (Just Macros.untilEndOfTurn) ]) ]
       Nothing

||| Koh, the Face Stealer
public export
kohChooser : Ability
kohChooser =
  Macros.activated (Macros.payLife You 1)
    (Macros.choose (Macros.a (And [Macros.creature, ExiledWith This])))

||| Forgotten Lore
public export
forgottenLoreChoice : Instruction []
forgottenLoreChoice =
  Macros.chooses (Macros.target Opponent)
                 (Macros.a (InZone (Macros.graveyardOf You)))

||| Psychic Paper minus its three-way coordination
public export
psychicPaperChoiceAndReads : AbilitySeq []
psychicPaperChoiceAndReads =
  [ Static (AndAlso Nothing [ Macros.attachChoosing Macros.thisEquipment CardName
                    , Macros.attachChoosing Macros.thisEquipment
                                            (SubtypeQ Creature) ])
  , Static (AndAlso Nothing
      [ Becomes (AttachHost Equipped (TypeW Creature)) Sets (ChosenQuality (Macros.ofTheLastChosen CardName))
      , Becomes (AttachHost Equipped (TypeW Creature)) Sets (ChosenQuality (Macros.ofTheLastChosen (SubtypeQ Creature))) ]) ]

public export
xenograft : Card
xenograft =
  Macros.card "Xenograft" (Just [Macros.generic 4, Macros.pip Blue]) []
       (MkTypeLine [] [Enchantment])
       [ Static (Macros.entersChoosing Macros.thisEnchantment (SubtypeQ Creature))
       , Static (Becomes (Macros.allOf (And [Macros.creature, HasPossessor ControllerAx You])) Adds (ChosenQuality (Macros.ofChosen (SubtypeQ Creature)))) ]
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
       , Static (Becomes (AttachHost Enchanted (TypeW Land)) Sets (ChosenQuality (Macros.ofChosen (SubtypeQ Land)))) ]
       Nothing

||| Realmwright
public export
realmwright : Card
realmwright =
  Macros.card "Realmwright" (Just [Macros.pip Blue]) []
       (MkTypeLine [creatureType "Vedalken", creatureType "Wizard"] [Creature])
       [ Static (Macros.entersChoosingFrom Macros.thisCreature (SubtypeQ Land)
                                           BasicTypesOnly)
       , Static (Becomes (Macros.allOf (And [Macros.land, HasPossessor ControllerAx You])) Adds (ChosenQuality (Macros.ofChosen (SubtypeQ Land)))) ]
       (Just (1, 1))

public export
adaptiveAutomaton : Card
adaptiveAutomaton =
  Macros.card "Adaptive Automaton" (Just [Macros.generic 3]) []
       (MkTypeLine [creatureType "Construct"] [Artifact, Creature])
       [ Static (Macros.entersChoosing Macros.thisCreature (SubtypeQ Creature))
       , Static (Becomes Macros.thisCreature Adds (ChosenQuality (Macros.ofChosen (SubtypeQ Creature))))
       , Static (Gets Adds (Macros.allOf (And [Macros.creature, HasPossessor ControllerAx You,
                                   OtherThan Macros.thisCreature,
                                   Macros.ofChosen (SubtypeQ Creature)]))
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
                   (Becomes (Macros.allOf (And [Macros.creature, HasPossessor ControllerAx You])) Adds (ChosenQuality (Macros.ofChosen (SubtypeQ Creature))))) ]
       Nothing

public export
conspiracy : Card
conspiracy =
  Macros.card "Conspiracy"
       (Just [Macros.generic 3, Macros.pip Black, Macros.pip Black]) []
       (MkTypeLine [] [Enchantment])
       [ Static (Macros.entersChoosing Macros.thisEnchantment (SubtypeQ Creature))
       , Static (AlsoOffBattlefield
                   (Becomes (Macros.allOf (And [Macros.creature, HasPossessor ControllerAx You])) Sets (ChosenQuality (Macros.ofChosen (SubtypeQ Creature))))) ]
       Nothing

public export
declarationOfNaught : Card
declarationOfNaught =
  Macros.card "Declaration of Naught" (Just [Macros.pip Blue, Macros.pip Blue]) []
       (MkTypeLine [] [Enchantment])
       [ Static (Macros.entersChoosing Macros.thisEnchantment CardName)
       , Macros.activated (Mana [Macros.pip Blue])
                          (CounterSpell
                      (Macros.target (And [Macros.spell, Named ChosenName]))) ]
       Nothing

public export
imagecrafter : Card
imagecrafter =
  Macros.card "Imagecrafter" (Just [Macros.pip Blue]) []
       (MkTypeLine [creatureType "Human", creatureType "Wizard"] [Creature])
       [ Macros.activated TapSymbol
           (Sequentially
              [ Macros.choose (Macros.a (Macros.qualityFrom (SubtypeQ Creature) (TypeOtherThan (creatureType "Wall"))))
              , Continuously (Becomes (Macros.target Macros.creature) Sets (ChosenQuality (Macros.ofChosen (SubtypeQ Creature))))
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
              , Continuously (Becomes (Macros.target Macros.creature) Sets (ChosenQuality (Macros.ofChosen (SubtypeQ Creature))))
                             (Just Macros.untilEndOfTurn) ]) ]
       Nothing

public export
standardize : Card
standardize =
  Macros.card "Standardize" (Just [Macros.pip Blue, Macros.pip Blue]) []
       (MkTypeLine [] [Instant])
       [ Spell (Sequentially
              [ Macros.choose (Macros.a (Macros.qualityFrom (SubtypeQ Creature) (TypeOtherThan (creatureType "Wall"))))
              , Continuously (Becomes (Macros.each Macros.creature) Sets (ChosenQuality (Macros.ofChosen (SubtypeQ Creature))))
                             (Just Macros.untilEndOfTurn) ]) ]
       Nothing

public export
silverquillSilencer : Card
silverquillSilencer =
  Macros.card "Silverquill Silencer" (Just [Macros.pip White, Macros.pip Black]) []
       (MkTypeLine [creatureType "Human", creatureType "Cleric"] [Creature])
       [ Static (Macros.entersChoosingFrom Macros.thisCreature
                                           CardName
                                           (NameOfCard (Not Macros.land)))
       , Macros.triggered Whenever
           (Casts Macros.anOpponent
                  (Macros.a (And [Macros.spell, Named ChosenName])) Nothing)
           (Sequentially [ Macros.losesLife They (Lit 3), (Draw You (Lit 1)) ]) ]
       (Just (3, 2))

public export
meddlingMage : Card
meddlingMage =
  Macros.card "Meddling Mage" (Just [Macros.pip White, Macros.pip Blue]) []
       (MkTypeLine [creatureType "Human", creatureType "Wizard"] [Creature])
       [ Static (Macros.entersChoosingFrom Macros.thisCreature
                                           CardName
                                           (NameOfCard (Not Macros.land)))
       , Static (Macros.objectCant "Cast"
                   (Macros.allOf (And [Macros.spell, Named ChosenName]))) ]
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
       , Static (Gets Adds Macros.thisCreature
                      (PtDown (Macros.countOf (InZone (Macros.handOf
                                 (Macros.the Macros.chosenPlayer)))))
                      (PtDown (Macros.countOf (InZone (Macros.handOf
                                 (Macros.the Macros.chosenPlayer)))))) ]
       (Just (7, 7))

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
                                         Compare [StatAxis Power] AtLeast Macros.chosenNumber])) ]) ]
       Nothing

public export
nevermore : Card
nevermore =
  Macros.card "Nevermore"
       (Just [Macros.generic 1, Macros.pip White, Macros.pip White]) []
       (MkTypeLine [] [Enchantment])
       [ Static (Macros.entersChoosingFrom Macros.thisEnchantment
                                           CardName
                                           (NameOfCard (Not Macros.land)))
       , Static (Macros.objectCant "Cast"
                   (Macros.allOf (And [Macros.spell, Named ChosenName]))) ]
       Nothing

public export
necromentiaChoice : Instruction []
necromentiaChoice =
  Macros.choose (Macros.a (Macros.qualityFrom CardName
                   (NameOfCard (Not (And [HasSupertype Basic, Macros.land])))))

public export
conjurersBan : Card
conjurersBan =
  Macros.card "Conjurer's Ban" (Just [Macros.pip White, Macros.pip Black]) []
       (MkTypeLine [] [Sorcery])
       [ Spell (Sequentially
           [ Macros.choose (Macros.a (Macros.quality CardName))
           , Continuously
               (AndAlso Nothing [ Macros.objectCant "Cast"
                            (Macros.allOf (And [Macros.spell, Named ChosenName]))
                        , Macros.objectCant "Play"
                            (Macros.allOf (And [Macros.land, Named ChosenName])) ])
               (Just Macros.untilYourNextTurn) ])
       , Spell (Draw You (Lit 1)) ]
       Nothing

public export
foundingOfOmashu : Card
foundingOfOmashu =
  Macros.card "Founding of Omashu"
       (Just [Macros.generic 2, Macros.pip Red]) []
       (MkTypeLine [enchantmentType "Saga"] [Enchantment])
       [ Macros.triggered When (ChapterMark [ChapterI])
           (Macros.create (Lit 2) (Macros.creatureTok 1 1 [White] [creatureType "Ally"]))
       , Macros.triggered When (ChapterMark [ChapterII])
           ((May You ((Macros.discard You (Macros.a (InZone Macros.handZ)))) (Just (Draw You (Lit 1))) Nothing))
       , Macros.triggered When (ChapterMark [ChapterIII])
           (Macros.gets (Macros.allOf Macros.creatureYouControl)
                        (PtUp (Lit 1)) (PtUp (Lit 0))
                        (Just Macros.untilEndOfTurn)) ]
       Nothing

public export
vedalkenOrrery : Card
vedalkenOrrery =
  Macros.card "Vedalken Orrery" (Just [Macros.generic 4]) []
       (MkTypeLine [] [Artifact])
       [ Static (Macros.mayPlayDeed "Cast" You (Macros.allOf Macros.spell)
                    (Just (AsThoughOf (HasKeyword (TheKeyword "Flash"))))
                    (PlayRider Nothing Nothing Nothing False ItsOwnCost)) ]
       Nothing

public export
shimmerMyr : Card
shimmerMyr =
  Macros.card "Shimmer Myr" (Just [Macros.generic 3]) []
       (MkTypeLine [creatureType "Myr"] [Artifact, Creature])
       [ Macros.keyword "Flash"
       , Static (Macros.mayPlayDeed "Cast" You
                    (Macros.allOf (And [Macros.spell, Macros.artifact]))
                    (Just (AsThoughOf (HasKeyword (TheKeyword "Flash"))))
                    (PlayRider Nothing Nothing Nothing False ItsOwnCost)) ]
       (Just (2, 2))

public export
quickSliver : Card
quickSliver =
  Macros.card "Quick Sliver" (Just [Macros.generic 1, Macros.pip Green]) []
       (MkTypeLine [creatureType "Sliver"] [Creature])
       [ Macros.keyword "Flash"
       , Static (Macros.mayPlayDeed "Cast" (Macros.a AnyPlayer)
                    (Macros.allOf (And [Macros.spell, HasSubtype (creatureType "Sliver")]))
                    (Just (AsThoughOf (HasKeyword (TheKeyword "Flash"))))
                    (PlayRider Nothing Nothing Nothing False ItsOwnCost)) ]
       (Just (1, 1))

public export
vernalEquinox : Card
vernalEquinox =
  Macros.card "Vernal Equinox"
       (Just [Macros.generic 3, Macros.pip Green]) []
       (MkTypeLine [] [Enchantment])
       [ Static (Macros.mayPlayDeed "Cast" (Macros.a AnyPlayer)
                    (Macros.allOf (And [Macros.spell,
                                       Or [Macros.creature, Macros.enchantment]]))
                    (Just (AsThoughOf (HasKeyword (TheKeyword "Flash"))))
                    (PlayRider Nothing Nothing Nothing False ItsOwnCost)) ]
       Nothing

public export
borneUponAWind : Card
borneUponAWind =
  Macros.card "Borne Upon a Wind" (Just [Macros.generic 1, Macros.pip Blue]) []
       (MkTypeLine [] [Instant])
       [ Spell (Continuously
                  (Macros.mayPlayDeed "Cast" You (Macros.allOf Macros.spell)
                     (Just (AsThoughOf (HasKeyword (TheKeyword "Flash"))))
                     (PlayRider Nothing Nothing Nothing False ItsOwnCost))
                  (Just ThisTurn))
       , Spell (Draw You (Lit 1)) ]
       Nothing

public export
gisaAndGeralf : Card
gisaAndGeralf =
  Macros.card "Gisa and Geralf"
       (Just [Macros.generic 2, Macros.pip Blue, Macros.pip Black]) [Legendary]
       (MkTypeLine [creatureType "Human", creatureType "Wizard"] [Creature])
       [ Macros.triggered When (Enters Macros.thisCreature Nothing)
           (Macros.mills You (Lit 4) You)
       , Static ((Macros.mayPlayDeed "Cast" You (Macros.a (And [Macros.spell, Macros.creature,
                                                           HasSubtype (creatureType "Zombie")])) Nothing (PlayRider (Just (Macros.graveyardOf You)) (Just OnceEachYourTurn) Nothing False ItsOwnCost))) ]
       (Just (4, 4))

public export
playLandsAndCastSpellsFromTop : StaticSpec []
playLandsAndCastSpellsFromTop =
  AndAlso Nothing [ (Macros.mayPlayDeed "Play" You (Macros.allOf Macros.land) Nothing (PlayRider (Just Macros.onTopZ) Nothing Nothing False ItsOwnCost))
          , (Macros.mayPlayDeed "Cast" You (Macros.allOf Macros.spell) Nothing (PlayRider (Just Macros.onTopZ) Nothing Nothing False ItsOwnCost)) ]

public export
playAndCastFromGraveyardThisTurn : Instruction []
playAndCastFromGraveyardThisTurn =
  Continuously
    (AndAlso Nothing [ (Macros.mayPlayDeed "Play" You (Macros.allOf Macros.land) Nothing (PlayRider (Just (Macros.graveyardOf You)) Nothing Nothing False ItsOwnCost))
             , (Macros.mayPlayDeed "Cast" You (Macros.allOf Macros.spell) Nothing (PlayRider (Just (Macros.graveyardOf You)) Nothing Nothing False ItsOwnCost)) ])
    (Just Macros.untilEndOfTurn)

public export
castSmallCreatureFromTopOnceEachTurn : StaticSpec []
castSmallCreatureFromTopOnceEachTurn =
  (Macros.mayPlayDeed "Cast" You (Macros.a (And [Macros.spell, Macros.creature,
                                            Compare [StatAxis Power] AtMost (Lit 2)])) Nothing (PlayRider (Just Macros.onTopZ) (Just OnceEachTurn) Nothing False ItsOwnCost))

public export
courserOfKruphix : Card
courserOfKruphix =
  Macros.card "Courser of Kruphix"
       (Just [Macros.generic 1, Macros.pip Green, Macros.pip Green]) []
       (MkTypeLine [creatureType "Centaur"] [Enchantment, Creature])
       [ Static (Visibility Reveal You TopOfLibrary)
       , Static ((Macros.mayPlayDeed "Play" You (Macros.allOf Macros.land) Nothing (PlayRider (Just Macros.onTopZ) Nothing Nothing False ItsOwnCost)))
       , Macros.triggered Whenever
           (Enters (Macros.a (And [Macros.land, HasPossessor ControllerAx You])) Nothing)
           (Macros.gainsLife You (Lit 1)) ]
       (Just (2, 4))

public export
korlessaScaleSinger : Card
korlessaScaleSinger =
  Macros.card "Korlessa, Scale Singer"
       (Just [Macros.pip Green, Macros.pip Blue]) [Legendary]
       (MkTypeLine [creatureType "Dragon", creatureType "Bard"] [Creature])
       [ Static (Visibility LookAt You TopOfLibrary)
       , Static ((Macros.mayPlayDeed "Cast" You (Macros.allOf (And [Macros.spell, HasSubtype (creatureType "Dragon")])) Nothing (PlayRider (Just Macros.onTopZ) Nothing Nothing False ItsOwnCost))) ]
       (Just (1, 4))

public export
precognitionField : Card
precognitionField =
  Macros.card "Precognition Field"
       (Just [Macros.generic 3, Macros.pip Blue]) []
       (MkTypeLine [] [Enchantment])
       [ Static (Visibility LookAt You TopOfLibrary)
       , Static ((Macros.mayPlayDeed "Cast" You (Macros.allOf (And [Macros.spell,
                                                 Macros.instantOrSorcery])) Nothing (PlayRider (Just Macros.onTopZ) Nothing Nothing False ItsOwnCost)))
       , Macros.activated (Mana [Macros.generic 3])
                          (Macros.exile You (LibrarySlice OnTop (Lit 1) You)) ]
       Nothing

public export
mysticForge : Card
mysticForge =
  Macros.card "Mystic Forge" (Just [Macros.generic 4]) []
       (MkTypeLine [] [Artifact])
       [ Static (Visibility LookAt You TopOfLibrary)
       , Static (AndAlso Nothing
           [ (Macros.mayPlayDeed "Cast" You (Macros.allOf (And [Macros.spell, Macros.artifact])) Nothing (PlayRider (Just Macros.onTopZ) Nothing Nothing False ItsOwnCost))
           , (Macros.mayPlayDeed "Cast" You (Macros.allOf (And [Macros.spell, IsColorless])) Nothing (PlayRider (Just Macros.onTopZ) Nothing Nothing False ItsOwnCost)) ])
       , Macros.activated (Compound [TapSymbol, Macros.payLife You 1])
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
       , Static ((Macros.mayPlayDeed "Play" You (Macros.allOf Macros.land) Nothing (PlayRider (Just Macros.onTopZ) Nothing Nothing False ItsOwnCost))) ]
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
       [ Spell (Continuously (Macros.mayPlayAdditionalLands You (Macros.upTo 3))
                             (Just ThisTurn)) ]
       Nothing

public export
explore : Card
explore =
  Macros.card "Explore" (Just [Macros.generic 1, Macros.pip Green]) []
       (MkTypeLine [] [Sorcery])
       [ Spell (Sequentially
           [ Continuously (Macros.mayPlayAdditionalLands You (Macros.exactly 1))
                          (Just ThisTurn)
           , (Draw You (Lit 1)) ]) ]
       Nothing

public export
urbanEvolution : Card
urbanEvolution =
  Macros.card "Urban Evolution"
       (Just [Macros.generic 3, Macros.pip Green, Macros.pip Blue]) []
       (MkTypeLine [] [Sorcery])
       [ Spell (Sequentially
           [ Draw You (Lit 3)
           , Continuously (Macros.mayPlayAdditionalLands You (Macros.exactly 1))
                          (Just ThisTurn) ]) ]
       Nothing

public export
eachPlayerPlaysAdditionalLand : StaticSpec []
eachPlayerPlaysAdditionalLand =
  Macros.mayPlayAdditionalLands (Macros.each AnyPlayer) (Macros.exactly 1)

public export
dryadOfTheIlysianGrove : Card
dryadOfTheIlysianGrove =
  Macros.card "Dryad of the Ilysian Grove"
       (Just [Macros.generic 2, Macros.pip Green]) []
       (MkTypeLine [creatureType "Nymph", creatureType "Dryad"] [Enchantment, Creature])
       [ Static (Macros.mayPlayAdditionalLands You (Macros.exactly 1))
       , Static (Becomes (Macros.allOf (And [Macros.land, HasPossessor ControllerAx You])) Adds (EveryTypeOf BasicLandSpace)) ]
       (Just (2, 4))

||| Ashes of the Fallen
public export
ashesOfTheFallen : Card
ashesOfTheFallen =
  Macros.card "Ashes of the Fallen" (Just [Macros.generic 2]) []
       (MkTypeLine [] [Artifact])
       [ Static (Macros.entersChoosing Macros.thisArtifact (SubtypeQ Creature))
       , Static (Becomes (Macros.each (And [Macros.creature, InZone (Macros.graveyardOf You)])) Adds (ChosenQuality (Macros.ofChosen (SubtypeQ Creature)))) ]
       Nothing

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

||| Forgotten Lore
public export
forgottenLoreRepeat : Instruction []
forgottenLoreRepeat =
  Sequentially
    [ Choose (Just (Macros.target Opponent))
             (Macros.a (InZone (Macros.graveyardOf You))) Openly
    , (May You (Pay You (Mana [Macros.pip Green]) PaidOnce) (Just (Repeat AgainExcludingChosen)) Nothing) ]

||| Leyline of the Meek
public export
leylineOfTheMeek : Card
leylineOfTheMeek =
  Macros.card "Leyline of the Meek"
       (Just [Macros.generic 2, Macros.pip White, Macros.pip White]) []
       (MkTypeLine [] [Enchantment])
       [ MayBeginOnBattlefield
       , Static (Gets Adds (Macros.allOf (And [Macros.creature, IsToken]))
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
       , Static (Gets Adds (Macros.allOf Macros.creatureYouControl)
                      (PtUp (Lit 0)) (PtUp (Lit 1)))
       , Macros.triggered Whenever
           (Enters (Macros.a Macros.creatureYouControl) Nothing)
           (Macros.may You (Macros.gainsLife You (Lit 1))) ]
       Nothing

public export
phyrexianRevoker : Card
phyrexianRevoker =
  Macros.card "Phyrexian Revoker" (Just [Macros.generic 2]) []
       (MkTypeLine [creatureType "Phyrexian", creatureType "Horror"] [Artifact, Creature])
       [ Static (Macros.entersChoosingFrom Macros.thisCreature
                                           CardName
                                           (NameOfCard (Not Macros.land)))
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
                                           (NameOfCard (Not Macros.land)))
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

||| Rhystic Study
public export
rhysticStudy : Card
rhysticStudy =
  Macros.card "Rhystic Study"
       (Just [Macros.generic 2, Macros.pip Blue]) []
       (MkTypeLine [] [Enchantment])
       [ Macros.triggered Whenever
           (Casts Macros.anOpponent (Macros.a Macros.spell) Nothing)
           (Unless (Macros.may You (Draw You (Lit 1)))
                          (Macros.That PlayerW OneOf)
                          (Mana [Macros.generic 1])) ]
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
                          (Macros.may You (Macros.move ((Macros.It OneOf)) (Macros.nthFromTop (Nth 5)))) ]
       (Just (3, 3))

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
       [ Spell ((May (Macros.controllerOf (Macros.target Macros.spell)) (Pay They (Macros.scaledMana GenericUnit (Macros.forEach 1
                                            (And [Macros.artifact, HasPossessor ControllerAx You]))) PaidOnce) Nothing (Just (CounterSpell ((Macros.It OneOf)))))) ]
       Nothing

public export
rakshasasDisdain : Card
rakshasasDisdain =
  Macros.card "Rakshasa's Disdain" (Just [Macros.generic 2, Macros.pip Blue]) []
       (MkTypeLine [] [Instant])
       [ Spell ((May (Macros.controllerOf (Macros.target Macros.spell)) (Pay They (Macros.scaledMana GenericUnit (Macros.forEach 1
                                            (InZone (Macros.graveyardOf You)))) PaidOnce) Nothing (Just (CounterSpell ((Macros.It OneOf)))))) ]
       Nothing

public export
megatherium : Card
megatherium =
  Macros.card "Megatherium" (Just [Macros.generic 2, Macros.pip Green]) []
       (MkTypeLine [creatureType "Beast"] [Creature])
       [ Macros.keyword "Trample"
       , Macros.triggered When (Enters Macros.thisCreature Nothing)
           ((May You (Pay You (Macros.scaledMana GenericUnit (Macros.forEach 1 (InZone (Macros.handOf You)))) PaidOnce) Nothing (Just (Macros.sacrifice You Macros.thisCreature)))) ]
       (Just (4, 4))

public export
killingWave : Instruction []
killingWave =
  ForEachOf (Macros.each Macros.creature)
            ((May (Macros.controllerOf ((Macros.It OneOf))) (Pay They (Do (Macros.losesLife They (LetterVal X))) PaidOnce) Nothing (Just (Macros.sacrifice They ((Macros.It OneOf))))))

public export
fadeAway : Instruction []
fadeAway =
  ForEachOf (Macros.each Macros.creature)
            ((May (Macros.controllerOf ((Macros.It OneOf))) (Pay They (Mana [Macros.generic 1]) PaidOnce) Nothing (Just (Macros.sacrifice They (Macros.a Permanent)))))

public export
tidalFlats : Card
tidalFlats =
  Macros.card "Tidal Flats" (Just [Macros.pip Blue]) []
       (MkTypeLine [] [Enchantment])
       [ Macros.activated (Mana [Macros.pip Blue, Macros.pip Blue])
           (ForEachOf (Macros.each (And [Macros.creature, Attacking,
                                  Not (HasKeyword (TheKeyword "Flying"))]))
                      ((May (Macros.controllerOf ((Macros.It OneOf))) (Pay They (Mana [Macros.generic 1]) PaidOnce) Nothing (Just (Macros.gains
                                         (Macros.allOf (And [Macros.creature, HasPossessor ControllerAx You,
                                                      CombatRel BlockerOf (Macros.That (TypeW Creature) OneOf)]))
                                         (Macros.keyword "FirstStrike")
                                         (Just Macros.untilEndOfTurn)))))) ]
       Nothing

public export
primalSurge : Card
primalSurge =
  Macros.card "Primal Surge"
       (Just [Macros.generic 8, Macros.pip Green, Macros.pip Green]) []
       (MkTypeLine [] [Sorcery])
       [ Spell (Sequentially
              [ Macros.exile You (Macros.topSlice (Lit 1))
              , If (Macros.itsA Permanent)
                   ((May You (Macros.putOntoBattlefield ((Macros.It OneOf))) (Just (Repeat Again)) Nothing))
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
                 (Macros.a (And [Macros.land, InZone (Macros.handOf You)]))) (Just (Sequentially [(Draw You (Lit 1)), Repeat Again])) Nothing)) ]
       (Just (PtBox PrintedStar PrintedStar))

public export
zimoneAndDina : Ability
zimoneAndDina =
  Macros.activated (Compound [TapSymbol,
                       Do (Macros.sacrifice You
                             (Macros.a (Macros.otherCreature Macros.thisCreature)))])
    (Sequentially
       [ (Draw You (Lit 1))
       , Macros.may You (Macros.putOntoBattlefieldTapped
                           (Macros.a (And [Macros.land,
                                           InZone (Macros.handOf You)])))
       , If (CompareAmt (Macros.countOf (And [Macros.land, HasPossessor ControllerAx You]))
                        AtLeast (Lit 8))
                   (Repeat (MoreTimes (Lit 1)))
                   Nothing ])

public export
cryptLurker : Card
cryptLurker =
  Macros.card "Crypt Lurker" (Just [Macros.generic 3, Macros.pip Black]) []
       (MkTypeLine [creatureType "Horror"] [Creature])
       [ Macros.triggered When (Enters Macros.thisCreature Nothing)
           ((May You (Macros.chooseModes (Macros.exactly 1)
                 [ Macros.sacrifice You (Macros.a Macros.creature)
                 , Macros.discard You (Macros.a (And [Macros.creature,
                                                       InZone Macros.handZ])) ]) (Just (Draw You (Lit 1))) Nothing)) ]
       (Just (3, 4))

||| Memoricide
public export
memoricideSearch : Instruction []
memoricideSearch =
  Sequentially [ Macros.choose (Macros.a (Macros.qualityFrom CardName
                                     (NameOfCard (Not Macros.land))))
               , Macros.searchZonesOf (Macros.target AnyPlayer) (Named ChosenName)
               , Shuffle (Macros.That PlayerW OneOf) ]

||| Lost Hours
public export
lostHoursPlacement : Instruction []
lostHoursPlacement =
  Sequentially [ Macros.revealsTheirHand (Macros.target AnyPlayer)
               , Macros.choose (Macros.a (And [Not Macros.land,
                                        InZone (Macros.handOf They)]))
               , Macros.puts (Macros.That PlayerW OneOf) ((Macros.It OneOf)) (Macros.nthFromTop (Nth 3)) ]

||| Aether Gust
public export
aetherGustPlacement : Instruction []
aetherGustPlacement =
  Sequentially [ Macros.choose (Macros.target (And [Permanent, ColorIs Red]))
               , Macros.puts (Macros.ownerOf ((Macros.It OneOf))) ((Macros.It OneOf)) (Macros.choiceOfTopOrBottom They) ]

||| Fastbond
public export
fastbondLands : Ability
fastbondLands = Static (Macros.mayPlayAdditionalLands You Macros.anyNumber)

||| Panglacial Wurm
public export
panglacialWurmCast : Ability
panglacialWurmCast =
  Static ((Macros.mayPlayDeed "Cast" You This Nothing (PlayRider (Just Macros.yourLibrary) Nothing (Just WhileSearchingLibrary) False ItsOwnCost)))

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

||| Memory Plunder
public export
memoryPlunder : Card
memoryPlunder =
  Macros.card "Memory Plunder"
       (Just [Macros.hybridPip Blue Black, Macros.hybridPip Blue Black,
              Macros.hybridPip Blue Black, Macros.hybridPip Blue Black]) []
       (MkTypeLine [] [Instant])
       [ Spell (Continuously
                  ((Macros.mayPlayDeed "Cast" You (Macros.target (And [Macros.instantOrSorcery, IsCard])) Nothing (PlayRider (Just (Macros.graveyardOf Macros.anOpponent)) Nothing Nothing False WithoutPaying)))
                  Nothing) ]
       Nothing

||| Omniscience
public export
omniscience : Card
omniscience =
  Macros.card "Omniscience"
       (Just [Macros.generic 7, Macros.pip Blue, Macros.pip Blue, Macros.pip Blue]) []
       (MkTypeLine [] [Enchantment])
       [ Static ((Macros.mayPlayDeed "Cast" You (Macros.allOf Macros.spell) Nothing (PlayRider (Just (Macros.handOf You)) Nothing Nothing False WithoutPaying))) ]
       Nothing

||| Tranquil Frillback
public export
tranquilFrillbackOffer : Instruction []
tranquilFrillbackOffer =
  Macros.may You (Pay You (Mana [Macros.pip Green]) (UpToTimes 3))

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
                                  Macros.ofChosen (SubtypeQ Creature)]))) ]
       (Just (PtBox PrintedStar PrintedStar))

||| Duneblast
public export
duneblast : Instruction []
duneblast =
  Sequentially [ Macros.choose (Macros.counted (Macros.upTo 1) Macros.creature)
               , Macros.destroy (Macros.theRest Object) ]

||| Boreas Charger
public export
boreasChargerChoice : Instruction []
boreasChargerChoice = Macros.choose (Macros.a Anaphora.opponentWithMoreLands)

||| Boreas Charger's spell text
public export
boreasChargerSpell : Instruction []
boreasChargerSpell =
  Sequentially
    [ Macros.choose (Macros.a Anaphora.opponentWithMoreLands)
    , Macros.searchLibraryFor (ExactlyOf TheDifference)
                                   (HasSubtype (landType "Plains"))
    , Macros.revealCards (Macros.That CardW ManyOf)
    , Macros.putOntoBattlefieldTapped (Macros.someOf (Macros.exactly 1) (Macros.That CardW ManyOf))
    , Macros.move (Macros.theRest Object) Macros.handZ ]

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
              , If (CompareAmt (Macros.countOf (InZone (Macros.handOf (Macros.That PlayerW OneOf))))
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
       , If (CompareAmt (Macros.countOf (InZone (Macros.handOf (Macros.That PlayerW OneOf))))
                        Greater
                        (Macros.countOf (InZone (Macros.handOf You))))
            (Draw You TheDifference)
            Nothing ])

||| Celestial Judgment's pass
public export
celestialJudgmentPass : Instruction []
celestialJudgmentPass =
  ForEachKindOf (ValueAxis Power) (Just (Macros.allOf Macros.creature)) Number
    (Macros.choose (Macros.a (And [Macros.creature,
                                   Compare [StatAxis Power] Eq Macros.chosenNumber])))

||| World Queller
public export
worldQuellerChoice : Instruction []
worldQuellerChoice =
  (May You (Macros.choose (Macros.a (Macros.quality CardTypeQ))) (Just (Macros.sacrifice (Macros.each AnyPlayer)
                 (Macros.aTheirChoice (And [Permanent, Macros.ofChosen CardTypeQ])))) Nothing)

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
                                  False (Just You) Nothing)
                   [] Nothing
                   (Macros.may You
                        (Create You GroupSize
                                (TokenCopyOf (AttachHost Enchanted PermanentW) []) []))
                   Repeatedly (Just OncePerTurn)) ]
       Nothing

public export
discardUpToTwoThenDrawThatMany : Instruction []
discardUpToTwoThenDrawThatMany =
  Sequentially [ (Repeated (UpTo (Lit 2)) (Sequentially [Macros.choose (Macros.a (InZone Macros.handZ)), Macros.discard You (Macros.That CardW OneOf)]))
               , Draw You GroupSize ]

||| Truce and Temporary Truce
public export
drawUpToTwoThenGainPerShortfall : Instruction []
drawUpToTwoThenGainPerShortfall =
  Sequentially [ Macros.may (Macros.each AnyPlayer) (Draw They (UpTo (Lit 2)))
               , Macros.gainsLife They (Macros.times 2 Macros.shortOfCeiling) ]

||| Commune with the Gods
public export
communeWithTheGods : Card
communeWithTheGods =
  Macros.card "Commune with the Gods" (Just [Macros.generic 1, Macros.pip Green]) []
       (MkTypeLine [] [Sorcery])
       [ Spell (Sequentially
           [ Macros.revealCards ((Macros.topSlice (Lit 5)))
           , Macros.may You
               (Macros.move (Macros.fromAmong (Macros.exactly 1) (Or [Macros.creature, Macros.enchantment])
                                                 ((Macros.It ManyOf)))
                            Macros.handZ)
           , Macros.move (Macros.theRest Object) Macros.graveyardZ ]) ]
       Nothing

public export
millThenPutFromAmongMilled : Instruction []
millThenPutFromAmongMilled =
  Sequentially [ Macros.mills You (Lit 3) You
               , Macros.may You
                   (Macros.move (Macros.fromAmong (Macros.exactly 1) Macros.artifact
                                   (Macros.TheVerbed "Mill" CardW ThisWay ManyOf))
                                Macros.handZ) ]

public export
eachPlayerOffered : Noun [] Player
eachPlayerOffered = Macros.each AnyPlayer

public export
eachPlayerOfferBindsOneMember :
  countOnes Player (mayCtx Choice.eachPlayerOffered) = 1
eachPlayerOfferBindsOneMember = Refl

public export
eachPlayerOfferDropsTheGroup :
  countManys Player (mayCtx Choice.eachPlayerOffered) = 0
eachPlayerOfferDropsTheGroup = Refl

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
summonEsperValigarmandaCast : StaticSpec []
summonEsperValigarmandaCast =
  AndAlso Nothing [ Macros.mayPlayDeed "Cast" You
                      (Macros.a (And [Macros.instantOrSorcery,
                                      ExiledWith Macros.thisSaga])) Nothing
                      (PlayRider Nothing Nothing Nothing False ItsOwnCost)
          , Macros.maySpendAsThough You Nothing MatchAnyType
              (Just (ToCast (And [Macros.instantOrSorcery,
                                  ExiledWith Macros.thisSaga]))) ]

||| Rogue Class's level-3 body
public export
rogueClassLevelThree : StaticSpec []
rogueClassLevelThree =
  AndAlso Nothing [ (Macros.mayPlayDeed "Play" You (Macros.allOf (ExiledWith Macros.thisClass)) Nothing (PlayRider Nothing Nothing Nothing False ItsOwnCost))
          , Macros.maySpendAsThough You Nothing MatchAnyColor
              (Just (ToCast (ExiledWith Macros.thisClass))) ]

||| Soul Ransom
public export
soulRansom : Card
soulRansom =
  Macros.card "Soul Ransom"
       (Just [Macros.generic 2, Macros.pip Blue, Macros.pip Black]) []
       (MkTypeLine [enchantmentType "Aura"] [Enchantment])
       [ Macros.keywordSubject "Enchant" Macros.creature
       , Static (GainsControl You (AttachHost Enchanted (TypeW Creature)))
       , Macros.activatedBy (Do ((Repeated (Lit 2) (Sequentially [Macros.choose (Macros.a (InZone Macros.handZ)), Macros.discard You (Macros.That CardW OneOf)]))))
           (Sequentially [ Macros.sacrificeIt (Macros.controllerOf Macros.thisAura)
                         , Draw They (Lit 2) ])
           (PlayerGroup YourOpponents) ]
       Nothing

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
                  , Macros.revealCards ((Macros.It OneOf))
                  , Macros.move ((Macros.It OneOf)) Macros.handZ ])
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
           (Matches ((Macros.It OneOf)) (Not BeingDeclaredAttacker))
           (Macros.may You (Draw You (Lit 1)))
       , Macros.activated (Mana [Macros.generic 4, Macros.pip Blue])
           (Macros.tap (Macros.target
                          (And [Macros.creature, Not (HasKeyword (TheKeyword "Flying"))]))) ]
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
                                       HasSubtype (artifactType "Equipment")]])) Nothing (PlayRider (Just (Macros.graveyardOf You)) (Just OnceEachYourTurn) Nothing False ItsOwnCost))) ]
       (Just (2, 2))

||| Muldrotha, the Gravetide
public export
muldrothaLandWindow : StaticSpec []
muldrothaLandWindow =
  (Macros.mayPlayDeed "Play" You (Macros.a Macros.land) Nothing (PlayRider (Just (Macros.graveyardOf You)) Nothing (Just DuringEachOfYourTurns) False ItsOwnCost))

||| Nahiri's Lithoforming
public export
nahiriExtraLands : StaticSpec (costLetters (Just [Variable]))
nahiriExtraLands = Macros.mayPlayAdditionalLands You (ExactlyOf (LetterVal X))

||| Haakon, Stromgald Scourge
public export
haakonStromgaldScourge : Card
haakonStromgaldScourge =
  Macros.card "Haakon, Stromgald Scourge"
       (Just [Macros.generic 1, Macros.pip Black, Macros.pip Black]) [Legendary]
       (MkTypeLine [creatureType "Zombie", creatureType "Knight"] [Creature])
       [ Static ((Macros.mayPlayDeed "Cast" You This Nothing (PlayRider (Just (Macros.graveyardOf You)) Nothing Nothing True ItsOwnCost)))
       , Static (Conditionally
                   ((Macros.mayPlayDeed "Cast" You (Macros.allOf (And [Macros.spell,
                                   HasSubtype (creatureType "Knight")])) Nothing (PlayRider (Just (Macros.graveyardOf You)) Nothing Nothing False ItsOwnCost)))
                   (Matches This (InZone Macros.battlefieldZ))
                   AsLongAs)
       , Macros.triggered When (Dies Macros.thisCreature)
           (Macros.losesLife You (Lit 2)) ]
       (Just (3, 3))

||| Apex of Power
public export
apexOfPowerCast : Instruction []
apexOfPowerCast =
  Sequentially
    [ Macros.exile You (LibrarySlice OnTop (Lit 7) You)
    , Continuously
        (Macros.mayPlayDeed "Cast" You
             (Macros.fromAmong Macros.anyNumber Macros.spell ((Macros.It ManyOf))) Nothing
             (PlayRider Nothing Nothing Nothing False ItsOwnCost))
        (Just ThisTurn) ]

||| Blight Herder
public export
blightHerderCast : Instruction []
blightHerderCast =
  Macros.may You
    (Macros.move (Macros.counted (Macros.exactly 2)
                    (And [IsCard, HasPossessor OwnerAx (PlayerGroup YourOpponents),
                          InZone Macros.exileZ]))
                 Macros.graveyardZ)

public export
eachPlayerMayShuffleTheirHandAndGraveyard : Instruction []
eachPlayerMayShuffleTheirHandAndGraveyard =
  Macros.may (Macros.each AnyPlayer)
    (Macros.shuffleInto They
       (Both (Macros.allOf (InZone (Macros.handOf They)))
               (Macros.allOf (InZone (Macros.graveyardOf They)))))

public export
eachPlayerMayDiscardTheirHandAndDrawSeven : Instruction []
eachPlayerMayDiscardTheirHandAndDrawSeven =
  Macros.may (Macros.each AnyPlayer)
    (Sequentially [ Macros.discard They (Macros.allOf (InZone (Macros.handOf They)))
                  , Draw They (Lit 7) ])

||| True-Name Nemesis
public export
trueNameNemesis : Card
trueNameNemesis =
  Macros.card "True-Name Nemesis"
       (Just [Macros.generic 1, Macros.pip Blue, Macros.pip Blue]) []
       (MkTypeLine [creatureType "Merfolk", creatureType "Rogue"] [Creature])
       [ Static (Macros.entersChoosingPlayer Macros.thisCreature Nothing)
       , Macros.keywordQuality "Protection" Macros.chosenPlayer ]
       (Just (3, 1))

public export
akiriUnattachOffer : Ability
akiriUnattachOffer =
  Macros.activated (Mana [Macros.pip White])
    ((May You (Unattach
          (Macros.a (And [ HasSubtype (artifactType "Equipment")
                         , AttachedTo (Macros.a Macros.creatureYouControl) ]))) (Just (SetStatus Tapped (Macros.That (TypeW Creature) OneOf))) Nothing))

||| Summoning Materia
public export
summoningMateriaTopCast : Ability
summoningMateriaTopCast =
  Static (Macros.onlyWhile
            ((Macros.mayPlayDeed "Cast" You (Macros.allOf (And [Macros.spell, Macros.creature])) Nothing (PlayRider (Just Macros.onTopZ) Nothing Nothing False ItsOwnCost)))
            (Matches Macros.thisEquipment (AttachedTo (Macros.a Macros.creature))))

||| Vizier of the Menagerie
public export
vizierOfTheMenagerieSpend : StaticSpec []
vizierOfTheMenagerieSpend =
  Macros.maySpendAsThough You Nothing MatchAnyType
    (Just (ToCast (And [Macros.creature, Macros.spell])))

||| Conspicuous Snoop
public export
conspicuousSnoop : Card
conspicuousSnoop =
  Macros.card "Conspicuous Snoop" (Just [Macros.pip Red, Macros.pip Red]) []
       (MkTypeLine [creatureType "Goblin", creatureType "Rogue"] [Creature])
       [ Static (Visibility Reveal You TopOfLibrary)
       , Static ((Macros.mayPlayDeed "Play" You (Macros.allOf (And [Macros.spell,
                                HasSubtype (creatureType "Goblin")])) Nothing (PlayRider (Just Macros.onTopZ) Nothing Nothing False ItsOwnCost)))
       , Static (Macros.onlyWhile
                   (GainsAbilitiesOf Macros.thisCreature [AnyActivated]
                                     (Macros.topSlice (Lit 1)) Nothing)
                   (Matches (Macros.topSlice (Lit 1)) (HasSubtype (creatureType "Goblin")))) ]
       (Just (2, 2))

||| Ballot Broker
public export
ballotBroker : Card
ballotBroker =
  Macros.card "Ballot Broker"
       (Just [Macros.generic 2, Macros.pip White]) []
       (MkTypeLine [creatureType "Human", creatureType "Advisor"] [Creature])
       [ Static (Macros.mayVoteAdditional You (Macros.exactly 1)) ]
       (Just (2, 3))

||| Emissary of Grudges
public export
emissaryOfGrudgesReveal : AbilityAt [choiceB PlayerC]
emissaryOfGrudgesReveal =
  Macros.activatedOnlyOnce
    (Do (Expose Reveal You (ExposedChoice PlayerC)))
    (OnlyIf (ChooseNewTargets
               (Macros.target (Or [Macros.spell, AbilityHead AnyOnStack])))
            (Matches ((Macros.It OneOf)) (HasPossessor ControllerAx (Macros.the Macros.chosenPlayer)))
            Nothing)
    OncePerGame

public export
prosperity : Card
prosperity =
  Macros.card "Prosperity" (Just [Variable, Macros.pip Blue]) []
       (MkTypeLine [] [Sorcery]) [Spell (Draw (Macros.each AnyPlayer) (LetterVal X))] Nothing

collectiveUnconscious : Instruction []
collectiveUnconscious = Draw You (Macros.forEach 1 Macros.creatureYouControl)

killiansConfidence : Instruction []
killiansConfidence = Sequentially [Macros.gets (Macros.target Macros.creature) (PtUp (Lit 1)) (PtUp (Lit 1)) (Just Macros.untilEndOfTurn),
                                   (Draw You (Lit 1))]

scatheZombies : Card
scatheZombies = Macros.card "Scathe Zombies" (Just [Macros.generic 2, Macros.pip Black]) []
                     (MkTypeLine [creatureType "Zombie"] [Creature]) [] (Just (2, 2))

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

||| Not Forgotten
public export
notForgottenPlacement : Instruction []
notForgottenPlacement =
  Macros.move (Macros.target (InZone Macros.graveyardZ)) (Macros.choiceOfTopOrBottom You)

public export
duneblastCard : Card
duneblastCard =
  Macros.card "Duneblast"
       (Just [Macros.generic 4, Macros.pip White, Macros.pip Black,
              Macros.pip Green])
       [] (MkTypeLine [] [Sorcery]) [Spell Choice.duneblast] Nothing

public export
boreasChargerDifference : Amount (instrIntro Choice.boreasChargerChoice)
boreasChargerDifference = TheDifference

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

||| Caught in the Crossfire
public export
caughtInTheCrossfire : Card
caughtInTheCrossfire =
  Macros.card "Caught in the Crossfire" (Just [Macros.pip Red, Macros.pip Red]) []
       (MkTypeLine [] [Instant])
       [ Spell (Macros.spree
                  [ (Just (Mana [Macros.generic 1]),
                     DealDamage This (Lit 2)
                       (Macros.each (And [Macros.creature, Macros.outlaw])))
                  , (Just (Mana [Macros.generic 1]),
                     DealDamage This (Lit 2)
                       (Macros.each (And [Macros.creature,
                                          Not Macros.outlaw]))) ]) ]
       Nothing

||| Requisition Raid
public export
requisitionRaid : Card
requisitionRaid =
  Macros.card "Requisition Raid" (Just [Macros.pip White]) []
       (MkTypeLine [] [Sorcery])
       [ Spell (Macros.spree
                  [ (Just (Mana [Macros.generic 1]), Macros.destroy (Macros.target Macros.artifact))
                  , (Just (Mana [Macros.generic 1]), Macros.destroy (Macros.target Macros.enchantment))
                  , (Just (Mana [Macros.generic 1]),
                     PutCounters (Lit 1) (PrintedKind Macros.plusOnePlusOne)
                       (Macros.each (And [Macros.creature,
                                          HasPossessor ControllerAx (Macros.target AnyPlayer)]))) ]) ]
       Nothing

||| Rustler Rampage
public export
rustlerRampage : Card
rustlerRampage =
  Macros.card "Rustler Rampage" (Just [Macros.pip White]) []
       (MkTypeLine [] [Instant])
       [ Spell (Macros.spree
                  [ (Just (Mana [Macros.generic 1]),
                     SetStatus Untapped
                       (Macros.allOf (And [Macros.creature,
                                           HasPossessor ControllerAx (Macros.target AnyPlayer)])))
                  , (Just (Mana [Macros.generic 1]),
                     Macros.gains (Macros.target Macros.creature) (Macros.keyword "DoubleStrike")
                                  (Just Macros.untilEndOfTurn)) ]) ]
       Nothing

||| Consuming Tide
public export
consumingTide : Card
consumingTide =
  Macros.card "Consuming Tide"
       (Just [Macros.generic 2, Macros.pip Blue, Macros.pip Blue]) []
       (MkTypeLine [] [Sorcery])
       [ Spell (Sequentially
                  [ Macros.chooses (Macros.each AnyPlayer)
                      (Macros.a (And [Permanent, Not Macros.land,
                                      HasPossessor ControllerAx Macros.They]))
                  , Macros.returnTo
                      (Macros.allOf (And [Permanent, Not Macros.land, NotChosen]))
                      Macros.handZ []
                  , ForEachOf
                      (Macros.each
                         (CompareOver Opponent
                            (Macros.countOf (InZone (Macros.handOf Macros.They)))
                            Greater
                            (Macros.countOf (InZone (Macros.handOf You)))))
                      (Draw You (Lit 1)) ]) ]
       Nothing

||| Stick Together: a chosen party is four up-to-one choices, not the group the
||| game computes [CR#700.8d].
public export
stickTogether : Instruction []
stickTogether =
  ForEachOf (Macros.each AnyPlayer)
    (Sequentially
       [ Macros.chooses Macros.They
           (Macros.counted (Macros.upTo 1)
              (And [Macros.creature, HasSubtype (creatureType "Cleric"),
                    HasPossessor ControllerAx Macros.They]))
       , Macros.chooses Macros.They
           (Macros.counted (Macros.upTo 1)
              (And [Macros.creature, HasSubtype (creatureType "Rogue"),
                    HasPossessor ControllerAx Macros.They]))
       , Macros.chooses Macros.They
           (Macros.counted (Macros.upTo 1)
              (And [Macros.creature, HasSubtype (creatureType "Warrior"),
                    HasPossessor ControllerAx Macros.They]))
       , Macros.chooses Macros.They
           (Macros.counted (Macros.upTo 1)
              (And [Macros.creature, HasSubtype (creatureType "Wizard"),
                    HasPossessor ControllerAx Macros.They]))
       , Macros.sacrifice Macros.They (Macros.theRest Object) ])
