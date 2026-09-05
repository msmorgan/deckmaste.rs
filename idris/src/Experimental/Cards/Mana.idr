module Experimental.Cards.Mana

import Experimental
import Experimental.Macros
import Experimental.Cards.Anaphora
import Experimental.Cards.Keyword

%default total


boshIronGolem : Ability
boshIronGolem = Macros.activated (Compound [Mana [Macros.generic 3, Macros.pip Red],
                                     Do (Macros.sacrifice You (Macros.a Macros.artifact))])
                                 (DealDamage This
                                      (StatOf ManaValue (Macros.TheVerbed "Sacrifice" (TypeW Artifact) Attributive OneOf))
                                      (Macros.target Macros.anyTarget))

pyromancy : Ability
pyromancy = Macros.activated (Compound [Mana [Macros.generic 3],
                                 Do (Macros.discard You (Macros.aAtRandom (InZone Macros.handZ)))])
                             (DealDamage Macros.thisEnchantment
                                  (StatOf ManaValue (Macros.TheVerbed "Discard" CardW Attributive OneOf))
                                  (Macros.target Macros.anyTarget))

luckyOffering : Instruction []
luckyOffering =
  Sequentially [Macros.destroy (Macros.target (And [Macros.artifact,
                                      Compare [StatAxis ManaValue] AtMost (Lit 3)])),
                Macros.gainsLife You (Lit 3)]

overload : Instruction []
overload = OnlyIf (Macros.destroy (Macros.target Macros.artifact))
                  (CompareAmt (StatOf ManaValue ((Macros.It))) AtMost (Lit 2))
                  Nothing

austereCommand : Instruction []
austereCommand =
  Macros.chooseModes (Macros.exactly 2) [Macros.destroy (Macros.allOf Macros.artifact),
             Macros.destroy (Macros.allOf Macros.enchantment),
             Macros.destroy (Macros.allOf (And [Macros.creature, Compare [StatAxis ManaValue] AtMost (Lit 3)])),
             Macros.destroy (Macros.allOf (And [Macros.creature, Compare [StatAxis ManaValue] AtLeast (Lit 4)]))]

bondersEnclave : Card
bondersEnclave =
  Macros.card "Bonders' Enclave" Nothing [] (MkTypeLine [] [Land])
       [ Macros.activated TapSymbol (AddMana You (Lit 1) (Runs [[Colorless]]) [])
       , Macros.activatedOnlyIf (Compound [Mana [Macros.generic 3], TapSymbol])
                                (Draw You (Lit 1))
                                (Macros.exists (And [Macros.creature, HasPossessor ControllerAx You,
                                              Compare [StatAxis Power] AtLeast (Lit 4)])) ]
       Nothing

workhorse : Card
workhorse =
  Macros.card "Workhorse" (Just [Macros.generic 6]) []
       (MkTypeLine [creatureType "Horse"] [Artifact, Creature])
       [ Static (Macros.entersWithCounters Macros.thisCreature (Lit 4) Macros.plusOnePlusOne)
       , Macros.activated (Do (RemoveCounters (Just (Macros.exactly 1)) (Just (PrintedKind Macros.plusOnePlusOne)) Macros.thisCreature))
                          (AddMana You (Lit 1) (Runs [[Colorless]]) []) ]
       (Just (0, 0))

labyrinthOfSkophos : Card
labyrinthOfSkophos =
  Macros.card "Labyrinth of Skophos" Nothing [] (MkTypeLine [] [Land])
       [ Macros.activated TapSymbol (AddMana You (Lit 1) (Runs [[Colorless]]) [])
       , Macros.activated (Compound [Mana [Macros.generic 4], TapSymbol])
                          (RemoveFromCombat
                      (Macros.target (And [Macros.creature, Or [Attacking, Blocking]]))) ]
       Nothing

acceleratedMutation : Card
acceleratedMutation =
  Macros.card "Accelerated Mutation"
       (Just [Macros.generic 3, Macros.pip Green, Macros.pip Green]) []
       (MkTypeLine [] [Instant])
       [ Spell Nothing (Sequentially
                  [ Macros.gets (Macros.target Macros.creature)
                                (Up (LetterVal X)) (Up (LetterVal X))
                                (Just Macros.untilEndOfTurn)
                  , Define X (Macros.aggregate MaxOf (StatAxis ManaValue)
                                 (And [Permanent, HasPossessor ControllerAx You])) ]) ]
       Nothing

cullingScales : Card
cullingScales =
  Macros.card "Culling Scales" (Just [Macros.generic 3]) []
       (MkTypeLine [] [Artifact])
       [ Macros.triggered At (BeginningOf ThePart Upkeep (ByPlayer You))
                          (Macros.destroy
                      (Macros.target
                         (And [Permanent, Not Macros.land,
                               Superlative MinOf (StatAxis ManaValue)
                                           (And [Permanent, Not Macros.land])]))) ]
       Nothing

deadeyeBrawler : Card
deadeyeBrawler =
  Macros.card "Deadeye Brawler"
       (Just [Macros.generic 2, Macros.pip Blue, Macros.pip Black]) []
       (MkTypeLine [creatureType "Human", creatureType "Pirate"] [Creature])
       [ Macros.keyword "Deathtouch"
       , Macros.keyword "Ascend"
       , Macros.triggeredIf Whenever
                            (Macros.dealsCombatDamage Macros.thisCreature (Macros.a AnyPlayer))
                            (Matches You (HasDesignation CitysBlessing Nothing))
                            (Draw You (Lit 1)) ]
       (Just (2, 4))

femerefEnchantress : Card
femerefEnchantress =
  Macros.card "Femeref Enchantress"
       (Just [Macros.pip Green, Macros.pip White]) []
       (MkTypeLine [creatureType "Human", creatureType "Druid"] [Creature])
       [ Macros.triggered Whenever
                          (Macros.putIntoFrom (Macros.a Macros.enchantment)
                                       Macros.graveyardZ
                                       (FromZone [Macros.battlefieldZ]))
                          (Draw You (Lit 1)) ]
       (Just (1, 2))

tocasiasWelcome : Card
tocasiasWelcome =
  Macros.card "Tocasia's Welcome"
       (Just [Macros.generic 2, Macros.pip White]) []
       (MkTypeLine [] [Enchantment])
       [ Macros.triggeredOnlyOnce Whenever
                                  (Enters (Macros.counted (Macros.atLeast 1)
                                                        (And [Macros.creature, HasPossessor ControllerAx You,
                                                              Compare [StatAxis ManaValue] AtMost (Lit 3)])) Nothing)
                                  OncePerTurn
                                  (Draw You (Lit 1)) ]
       Nothing

duskLegionDuelist : Card
duskLegionDuelist =
  Macros.card "Dusk Legion Duelist"
       (Just [Macros.generic 1, Macros.pip White]) []
       (MkTypeLine [creatureType "Vampire", creatureType "Soldier"] [Creature])
       [ Macros.keyword "Vigilance"
       , Macros.triggeredOnlyOnce Whenever
                                  (Macros.counterEvent CounterPut Macros.plusOnePlusOne ManyCounters
                                                           Macros.thisCreature)
                                  OncePerTurn
                                  (Draw You (Lit 1)) ]
       (Just (2, 2))

mishrasFactory : Card
mishrasFactory =
  Macros.card "Mishra's Factory" Nothing [] (MkTypeLine [] [Land])
       [ Macros.activated TapSymbol (AddMana You (Lit 1) (Runs [[Colorless]]) [])
       , Macros.activated (Mana [Macros.generic 1])
                          (Continuously
                      (Becomes Macros.thisLand Sets (Bundle (MkToken (Just (Lit 2 ** Lit 2)) []
                                         (MkTypeLine [creatureType "AssemblyWorker"] [Artifact, Creature])
                                         [] Nothing) (Just Land)))
                      (Just Macros.untilEndOfTurn))
       , Macros.activated TapSymbol
                          (Macros.gets (Macros.target (And [Macros.creature, HasSubtype (creatureType "AssemblyWorker")]))
                                (Up (Lit 1)) (Up (Lit 1)) (Just Macros.untilEndOfTurn)) ]
       Nothing

||| Mutavault
public export
mutavault : Card
mutavault =
  Macros.card "Mutavault" Nothing [] (MkTypeLine [] [Land])
       [ Macros.activated TapSymbol (AddMana You (Lit 1) (Runs [[Colorless]]) [])
       , Macros.activated (Mana [Macros.generic 1])
                          (Continuously
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
                          (Continuously
                      (Becomes Macros.thisLand Sets (Bundle (MkTokenChars (Just (Lit 3 ** Lit 3)) [] []
                                              (MkTypeLine [] [Creature])
                                              [Macros.keyword "Vigilance"] Nothing
                                              [WithEveryType CreatureSpace]) (Just Land)))
                      Nothing) ]
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
                      (Becomes Macros.thisLand Sets (Bundle (MkToken (Just (Lit 3 ** Lit 3)) [Red, Green]
                                         (MkTypeLine [creatureType "Elemental"] [Creature])
                                         [ Macros.triggered Whenever
                                                            (Macros.attacks Macros.thisCreature)
                                                            (PutCounters (Lit 1)
                                                                  (PrintedKind Macros.plusOnePlusOne) Macros.thisCreature) ]
                                         Nothing) (Just Land)))
                      (Just Macros.untilEndOfTurn)) ]
       Nothing

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
                                             HasPossessor ControllerAx You]))) (Just (Draw You (Lit 1))) Nothing) ])
       , Macros.activated (LoyaltySymbol (LoyaltyDown 2))
                          (Sequentially
                      [ Macros.create (Lit 2)
                          (MkToken (Just (Lit 1 ** Lit 1)) []
                                   (MkTypeLine [creatureType "Thopter"] [Artifact, Creature])
                                   [Macros.keyword "Flying"] Nothing)
                      , Macros.gainsHaste ((Macros.Them)) (Just Macros.untilEndOfTurn) ])
       , Macros.activated (LoyaltySymbol (LoyaltyDown 4))
                          (GetsEmblem You
                      [ Static (Macros.getsPt (Macros.allOf (And [Macros.artifact, Macros.creature,
                                                  HasPossessor ControllerAx You]))
                                     (Up (Lit 1)) (Up (Lit 1)))
                      , Static (Costs (Macros.allOf (And [Macros.artifact, Macros.spell,
                                                         Macros.castBy You]))
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
       [ Spell Nothing (AddMana You (Lit 1)
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

||| Qarsi Deceiver
public export
qarsiDeceiverMorphSpend : Ability
qarsiDeceiverMorphSpend =
  Macros.activated TapSymbol
                   (AddMana You (Lit 1) (Runs [[Colorless]])
                     [SpendOnly [ ToCast (And [Macros.creature, Macros.faceDown])
                                , ToPay (OfSpecialAction TurnFaceUp)
                                , ToPay (OfKeyword "Morph") ]])

||| Mercadian Bazaar
public export
mercadianBazaar : Card
mercadianBazaar =
  Macros.card "Mercadian Bazaar" Nothing [] (MkTypeLine [] [Land])
       [ Static (Macros.entersTapped Macros.thisLand)
       , Macros.activated TapSymbol
                          (PutCounters (Lit 1) (PrintedKind (NamedCounter "Storage")) Macros.thisLand)
       , Macros.activated (Compound [TapSymbol,
                                     Do (RemoveCounters (Just Macros.anyNumber) (Just (PrintedKind (NamedCounter "Storage")))
                                                        Macros.thisLand)])
                          (AddMana You Macros.removedThisWay (Runs [[OfColor Red]]) []) ]
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
                          (PutCounters (Lit 1) (PrintedKind (NamedCounter "Charge")) Macros.thisArtifact)
       , Macros.activated (Compound [TapSymbol,
                                     Do (RemoveCounters (Just Macros.anyNumber) (Just (PrintedKind (NamedCounter "Charge")))
                                                        Macros.thisArtifact)])
                          (Sequentially
                             [ AddMana You (Lit 1) (Runs [[OfColor Black]]) []
                             , AddMana You Macros.removedThisWay (Runs [[OfColor Black]]) [] ]) ]
       Nothing

||| Elemental Resonance
public export
elementalResonance : Card
elementalResonance =
  Macros.card "Elemental Resonance"
       (Just [Macros.generic 2, Macros.pip Green, Macros.pip Green]) []
       (MkTypeLine [enchantmentType "Aura"] [Enchantment])
       [ Macros.keywordSubject "Enchant" Permanent
       , Macros.triggered At (BeginningOf ThePart FirstMain (ByPlayer You))
           (AddMana You (Lit 1)
                    (AsPrintedCost (AttachHost Enchanted PermanentW)) []) ]
       Nothing

||| Steelswarm Operator
steelswarmOperatorMana : Instruction []
steelswarmOperatorMana =
  AddMana You (Lit 1) (Runs [[OfColor Blue, OfColor Blue]])
          [SpendOnly [ToActivate (Just (And [Macros.source, Macros.artifact]))]]

public export
blastOfGenius : Card
blastOfGenius =
  Macros.card "Blast of Genius"
       (Just [Macros.generic 4, Macros.pip Blue, Macros.pip Red]) []
       (MkTypeLine [] [Sorcery])
       [ Spell Nothing (Sequentially
           [ Macros.choose (Macros.target Macros.anyTarget)
           , Draw You (Lit 3)
           , Macros.discard You (Macros.a (InZone Macros.handZ))
           , DealDamage This (StatOf ManaValue (Macros.TheVerbed "Discard" CardW Attributive OneOf))
                        (Macros.thatJoin) ]) ]
       Nothing

public export
riddleOfLightning : Card
riddleOfLightning =
  Macros.card "Riddle of Lightning"
       (Just [Macros.generic 3, Macros.pip Red, Macros.pip Red]) []
       (MkTypeLine [] [Instant])
       [ Spell Nothing (Sequentially
           [ Macros.choose (Macros.target Macros.anyTarget)
           , Macros.scry You (Lit 3)
           , Macros.revealCards (Macros.topSlice (Lit 1))
           , DealDamage This (StatOf ManaValue (Macros.That CardW)) (Macros.thatJoin) ]) ]
       Nothing

public export
unstableFrontier : Card
unstableFrontier =
  Macros.card "Unstable Frontier" Nothing [] (MkTypeLine [] [Land])
       [ Macros.activated TapSymbol (AddMana You (Lit 1) (Runs [[Colorless]]) [])
       , Macros.activated TapSymbol
                          (Continuously
                      (Becomes (Macros.target (And [Macros.land, HasPossessor ControllerAx You])) Sets (ChosenQuality (OfYourChoice (SubtypeQ Land) (Just BasicTypesOnly))))
                      (Just Macros.untilEndOfTurn)) ]
       Nothing

||| Sanctum Prelate
public export
sanctumPrelate : Card
sanctumPrelate =
  Macros.card "Sanctum Prelate"
       (Just [Macros.generic 1, Macros.pip White, Macros.pip White]) []
       (MkTypeLine [creatureType "Human", creatureType "Cleric"] [Creature])
       [ Static (Macros.entersChoosing Macros.thisCreature Number)
       , Static (Macros.objectCant "Cast"
                   (Macros.allOf (And [Macros.spell, Not Macros.creature,
                                Compare [StatAxis ManaValue] Eq Macros.chosenNumber]))) ]
       (Just (2, 2))

public export
abruptDecay : Card
abruptDecay =
  Macros.card "Abrupt Decay" (Just [Macros.pip Black, Macros.pip Green]) []
       (MkTypeLine [] [Instant])
       [ Static (Macros.objectCant "Counter" This)
       , Spell Nothing (Macros.destroy
                  (Macros.target (And [Permanent, Not Macros.land,
                                       Compare [StatAxis ManaValue] AtMost (Lit 3)]))) ]
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
              [ Macros.exile (Macros.topSlice (Lit 1))
              , Continuously ((Macros.mayPlayDeed "Play" You (Macros.That CardW) Nothing (PlayRider Nothing Nothing Nothing False ItsOwnCost)))
                             (Just ThisTurn) ])
       , Macros.triggered When (ChapterMark [ChapterVI])
           (AddMana You (Lit 6) (Runs [[OfColor Red]]) []) ]
       Nothing

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
                          (Draw You (Lit 1)) ]
       (Just (2, 2))

||| Gaddock Teeg
public export
gaddockTeeg : Card
gaddockTeeg =
  Macros.card "Gaddock Teeg" (Just [Macros.pip Green, Macros.pip White])
       [Legendary]
       (MkTypeLine [creatureType "Kithkin", creatureType "Advisor"] [Creature])
       [ Static (Macros.objectCant "Cast"
                   (Macros.allOf (And [Macros.spell, Not Macros.creature,
                                Compare [StatAxis ManaValue] AtLeast (Lit 4)])))
       , Static (Macros.objectCant "Cast"
                   (Macros.allOf (And [Macros.spell, Not Macros.creature,
                                ManaCostHas Variable]))) ]
       (Just (2, 2))

public export
shiningShoal : Card
shiningShoal =
  Macros.card "Shining Shoal"
       (Just [Variable, Macros.pip White, Macros.pip White]) []
       (MkTypeLine [spellType "Arcane"] [Instant])
       [ Static (AltCost This (Just (Do (Macros.exiles You
                   (Macros.a (And [ColorIs White,
                                   Compare [StatAxis ManaValue] Eq (LetterVal X),
                                   InZone (Macros.handOf You)]))))))
       , Spell Nothing (Continuously
                  (DamageRule AnyDamage (DealtBy (Macros.aYourChoice Macros.source)) (ToRecipient
                                (Macros.youAnd (Macros.allOf (And [Macros.creature,
                                                     HasPossessor ControllerAx You])))) (Redirect (Shield (LetterVal X)) (Macros.target Macros.anyTarget)) Repeatedly)
                  (Just ThisTurn)) ]
       Nothing

public export
disruptingShoal : Card
disruptingShoal =
  Macros.card "Disrupting Shoal"
       (Just [Variable, Macros.pip Blue, Macros.pip Blue]) []
       (MkTypeLine [spellType "Arcane"] [Instant])
       [ Static (AltCost This (Just (Do (Macros.exiles You
                   (Macros.a (And [ColorIs Blue,
                                   Compare [StatAxis ManaValue] Eq (LetterVal X),
                                   InZone (Macros.handOf You)]))))))
       , Spell Nothing (OnlyIf (CounterSpell (Macros.target Macros.spell))
                       (CompareAmt (StatOf ManaValue ((Macros.It))) Eq (LetterVal X)) Nothing) ]
       Nothing

public export
blazingShoal : Card
blazingShoal =
  Macros.card "Blazing Shoal"
       (Just [Variable, Macros.pip Red, Macros.pip Red]) []
       (MkTypeLine [spellType "Arcane"] [Instant])
       [ Static (AltCost This (Just (Do (Macros.exiles You
                   (Macros.a (And [ColorIs Red,
                                   Compare [StatAxis ManaValue] Eq (LetterVal X),
                                   InZone (Macros.handOf You)]))))))
       , Spell Nothing (Macros.gets (Macros.target Macros.creature) (Up (LetterVal X))
                            (Up (Lit 0)) (Just Macros.untilEndOfTurn)) ]
       Nothing

public export
sickeningShoal : Card
sickeningShoal =
  Macros.card "Sickening Shoal"
       (Just [Variable, Macros.pip Black, Macros.pip Black]) []
       (MkTypeLine [spellType "Arcane"] [Instant])
       [ Static (AltCost This (Just (Do (Macros.exiles You
                   (Macros.a (And [ColorIs Black,
                                   Compare [StatAxis ManaValue] Eq (LetterVal X),
                                   InZone (Macros.handOf You)]))))))
       , Spell Nothing (Macros.gets (Macros.target Macros.creature) (Down (LetterVal X))
                            (Down (LetterVal X)) (Just Macros.untilEndOfTurn)) ]
       Nothing

public export
nourishingShoal : Card
nourishingShoal =
  Macros.card "Nourishing Shoal"
       (Just [Variable, Macros.pip Green, Macros.pip Green]) []
       (MkTypeLine [spellType "Arcane"] [Instant])
       [ Static (AltCost This (Just (Do (Macros.exiles You
                   (Macros.a (And [ColorIs Green,
                                   Compare [StatAxis ManaValue] Eq (LetterVal X),
                                   InZone (Macros.handOf You)]))))))
       , Spell Nothing (Macros.gainsLife You (LetterVal X)) ]
       Nothing

public export
spellSnare : Card
spellSnare =
  Macros.card "Spell Snare" (Just [Macros.pip Blue]) []
       (MkTypeLine [] [Instant])
       [ Spell Nothing (CounterSpell
                  (Macros.target (And [Macros.spell,
                                       Compare [StatAxis ManaValue] Eq (Lit 2)]))) ]
       Nothing

public export
isolate : Card
isolate =
  Macros.card "Isolate" (Just [Macros.pip White]) []
       (MkTypeLine [] [Instant])
       [ Spell Nothing (Macros.exile
                  (Macros.target (And [Permanent,
                                       Compare [StatAxis ManaValue] Eq (Lit 1)]))) ]
       Nothing

public export
disembowel : Card
disembowel =
  Macros.card "Disembowel" (Just [Variable, Macros.pip Black]) []
       (MkTypeLine [] [Instant])
       [ Spell Nothing (Macros.destroy
                  (Macros.target (And [Macros.creature,
                                       Compare [StatAxis ManaValue] Eq (LetterVal X)]))) ]
       Nothing

public export
repeal : Card
repeal =
  Macros.card "Repeal" (Just [Variable, Macros.pip Blue]) []
       (MkTypeLine [] [Instant])
       [ Spell Nothing (Sequentially
                  [ Macros.move (Macros.target (And [Not Macros.land, Permanent,
                                              Compare [StatAxis ManaValue] Eq (LetterVal X)]))
                                Macros.handZ
                  , Draw You (Lit 1) ]) ]
       Nothing

public export
entrancingMelody : Card
entrancingMelody =
  Macros.card "Entrancing Melody"
       (Just [Variable, Macros.pip Blue, Macros.pip Blue]) []
       (MkTypeLine [] [Sorcery])
       [ Spell Nothing (Continuously
                  (GainsControl You
                     (Macros.target (And [Macros.creature,
                                          Compare [StatAxis ManaValue] Eq (LetterVal X)])))
                  Nothing) ]
       Nothing

public export
ratchetBomb : Card
ratchetBomb =
  Macros.card "Ratchet Bomb" (Just [Macros.generic 2]) []
       (MkTypeLine [] [Artifact])
       [ Macros.activated TapSymbol
           (PutCounters (Lit 1) (PrintedKind (NamedCounter "Charge")) Macros.thisArtifact)
       , Macros.activated (Compound [TapSymbol,
                              Do (Macros.sacrifice You Macros.thisArtifact)])
           (Macros.destroy (Macros.each (And [Not Macros.land, Permanent,
                                       Compare [StatAxis ManaValue] Eq
                                         (CountersOn (NamedCounter "Charge") Macros.thisArtifact)]))) ]
       Nothing

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
secretsOfTheDead : Card
secretsOfTheDead =
  Macros.card "Secrets of the Dead"
       (Just [Macros.generic 2, Macros.pip Blue]) []
       (MkTypeLine [] [Enchantment])
       [ Macros.triggered Whenever
           (Casts You (Macros.a (And [Macros.spell,
                                      CastFrom (Macros.graveyardOf You)])) Nothing)
           (Draw You (Lit 1)) ]
       Nothing

public export
coalStoker : Card
coalStoker =
  Macros.card "Coal Stoker" (Just [Macros.generic 3, Macros.pip Red]) []
       (MkTypeLine [creatureType "Elemental"] [Creature])
       [ Macros.triggeredIf When
                            (Enters Macros.thisCreature Nothing)
                            (Matches ((Macros.It)) (CastFrom (Macros.handOf You)))
                            (AddMana You (Lit 1) (Runs [[OfColor Red, OfColor Red, OfColor Red]]) []) ]
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
           (Draw You (Lit 1)) ]
       (Just (2, 2))

public export
adNauseam : Card
adNauseam =
  Macros.card "Ad Nauseam"
       (Just [Macros.generic 3, Macros.pip Black, Macros.pip Black]) []
       (MkTypeLine [] [Instant])
       [ Spell Nothing (Sequentially
              [ Macros.revealCards (Macros.topSlice (Lit 1))
              , Macros.move (Macros.That CardW) Macros.handZ
              , Macros.losesLife You (StatOf ManaValue ((Macros.It)))
              , Macros.may You (Repeat AnyNumber) ]) ]
       Nothing

public export
nahiriLoyaltyRead : Predicate [] Object
nahiriLoyaltyRead =
  And [ Macros.creature, InZone (Macros.graveyardOf You)
      , Compare [StatAxis ManaValue] Less (StatOf Loyalty This) ]

||| Caustic Bronco
public export
causticBroncoLoss : Instruction []
causticBroncoLoss =
  Sequentially [ Macros.revealCards (Macros.topSlice (Lit 1))
               , Macros.move (Macros.That CardW) Macros.handZ
               , OnlyIf (Macros.losesLife You (StatOf ManaValue ((Macros.It))))
                        (NotCond (Matches Macros.thisCreature (HasDesignation Saddled Nothing)))
                        (Just (Macros.losesLife (Macros.each Opponent) ThatMuch)) ]

||| Dark Fortress
public export
darkFortressMana : Ability
darkFortressMana =
  Macros.activatedOnlyIf TapSymbol
    (AddMana You (Lit 1) (Runs [[OfColor Black], [OfColor Red]]) [])
    (OrCond [ Macros.happened Entry Macros.thisLand Lookback.ThisTurn
            , Macros.exists (And [Macros.land, HasSupertype Basic, HasPossessor ControllerAx You]) ])

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
              , AddMana You (Devotion You Macros.thatColor Nothing)
                        (OfChosenColor Nothing) [] ]) ]
       Nothing

||| Karametra's Acolyte
public export
karametrasAcolyte : Ability
karametrasAcolyte =
  Macros.activated TapSymbol
    (AddMana You (Devotion You (LitColor Green) Nothing) (Runs [[OfColor Green]]) [])

||| Investigator's Journal
investigatorsJournal : Card
investigatorsJournal =
  Macros.card "Investigator's Journal" (Just [Macros.generic 2]) []
       (MkTypeLine [artifactType "Book", artifactType "Clue"] [Artifact])
       [ Static (Macros.entersWithCounters Macros.thisArtifact
                   greatestCreaturesAPlayerControls (NamedCounter "Suspect"))
       , Macros.activated (Compound [Mana [Macros.generic 2], TapSymbol,
                              Do (RemoveCounters (Just (Macros.exactly 1)) (Just (PrintedKind (NamedCounter "Suspect")))
                                    Macros.thisArtifact)])
                          (Draw You (Lit 1))
       , Macros.activated (Compound [Mana [Macros.generic 2],
                              Do (Macros.sacrifice You Macros.thisArtifact)])
                          (Draw You (Lit 1)) ]
       Nothing

||| Engineered Explosives (its sunburst [CR#702.44a] written out)
public export
engineeredExplosives : Card
engineeredExplosives =
  Macros.card "Engineered Explosives" (Just [Variable]) []
       (MkTypeLine [] [Artifact])
       [ Static (Macros.entersWithCounters Macros.thisArtifact
                   (Macros.colorsSpentToCast Macros.thisArtifact)
                   (NamedCounter "Charge"))
       , Macros.activated (Compound [Mana [Macros.generic 2],
                              Do (Macros.sacrifice You Macros.thisArtifact)])
                          (Macros.destroy (Macros.each
                             (And [Permanent, Not Macros.land,
                                   Compare [StatAxis ManaValue] Eq
                                     (CountersOn (NamedCounter "Charge")
                                                 Macros.thisArtifact)]))) ]
       Nothing

||| Radiant Flames (converge is an ability word [CR#207.2c])
public export
radiantFlames : Card
radiantFlames =
  Macros.card "Radiant Flames"
       (Just [Macros.generic 2, Macros.pip Red]) []
       (MkTypeLine [] [Sorcery])
       [ Spell Nothing (DealDamage This (Macros.colorsSpentToCast This)
                           (Macros.each Macros.creature)) ]
       Nothing

||| Birthing Pod
public export
birthingPodSearch : Ability
birthingPodSearch =
  Macros.activated (Compound [Mana [Macros.generic 1, Macros.phyrexianPip Green], TapSymbol, Do (Macros.sacrifice You (Macros.a Macros.creature))]) (Macros.searchLibraryFor (Macros.exactly 1) (And [Macros.creature, Compare [StatAxis ManaValue] Eq (Plus (Lit 1) (StatOf ManaValue (Macros.TheVerbed "Sacrifice" (TypeW Creature) Attributive OneOf)))]))

||| Hibernation's End
public export
hibernationsEndTrigger : Ability
hibernationsEndTrigger =
  Macros.triggered Whenever
    (PaysCost (Just You) Paid Macros.thisEnchantment "CumulativeUpkeep")
    (Macros.may You
       (Sequentially
          [ Macros.searchLibraryFor (Macros.exactly 1)
              (And [Macros.creature,
                    Compare [StatAxis ManaValue] Eq (CountersOn (NamedCounter "Age") Macros.thisEnchantment)])
          , Macros.putOntoBattlefield (Macros.That CardW)
          , Macros.shuffle ]))

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
           (Macros.costWasPaid (ByKeyword "Prowl") Nothing Macros.thisCreature)
           (Draw You (Lit 1)) ]
       (Just (3, 1))

||| Bloom Tender
public export
bloomTenderMana : Ability
bloomTenderMana =
  Macros.abilityWord "vivid"
    (Macros.activated TapSymbol
       (ForEachKindOf ColorAxis
          (Just (Macros.allOf (And [Permanent, HasPossessor ControllerAx You]))) Color
          (AddMana You (Lit 1) (OfChosenColor Nothing) [])))

||| Tarnation Vista
public export
tarnationVistaMana : Ability
tarnationVistaMana =
  Macros.activated
    (Compound [Mana [Macros.generic 1], TapSymbol])
    (ForEachKindOf ColorAxis
       (Just (Macros.allOf (And [Permanent, Macros.monocolored, HasPossessor ControllerAx You]))) Color
       (AddMana You (Lit 1) (OfChosenColor Nothing) []))

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
           (Draw You (Lit 1)) ]
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
       , Macros.triggeredIf At (BeginningOf ThePart EndStep (ByPlayer You))
           (NotCond (Macros.happenedFrom SpellCast You Lookback.ThisTurn
                       (Macros.a Macros.spell)
                       (FromZone [Macros.handOf You])))
           (Draw You (Lit 1)) ]
       (Just (3, 3))

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
           (Draw You (Lit 1)) ]
       (Just (4, 4))

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
              (If (CompareAmt (StatOf ManaValue (Macros.That SpellW)) AtLeast (Lit 5))
                  (Create You (Lit 3) TokenAsThose [])
                  Nothing)) ]
       (Just (2, 1))

||| Collected Company
public export
collectedCompany : Card
collectedCompany =
  Macros.card "Collected Company" (Just [Macros.generic 3, Macros.pip Green]) []
       (MkTypeLine [] [Instant])
       [ Spell Nothing (Sequentially
           [ Macros.lookAt ((Macros.topSlice (Lit 6)))
           , Macros.move (Macros.fromAmong (Macros.upTo 2)
                                           (And [Macros.creature,
                                                 Compare [StatAxis ManaValue] AtMost (Lit 3)])
                                           ((Macros.Them)))
                         Macros.battlefieldZ
           , Macros.move (Macros.theRest Object) (Macros.onBottomIn AnyOrder) ]) ]
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
           (AddMana You (StatOf ManaValue (Macros.ItVerbed "Sacrifice"))
                    (Runs [[OfColor Black]]) []) ]
       (Just (1, 2))

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
                        Compare [StatAxis ManaValue] AtMost (Lit 2)])
              , Macros.revealsIt
              , Macros.move Macros.foundCard Macros.handZ
              , If (Macros.happenedAt (VerbedAct "Search") You Lookback.ThisWay
                                      Macros.yourLibrary)
                   Macros.shuffle Nothing ]) ]
       (Just (3, 2))


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
                             (Up (Lit 1)) (Up (Lit 1))
                             (Just Macros.untilEndOfTurn)) ]
       Nothing

||| Mox Diamond
public export
moxDiamond : Card
moxDiamond =
  Macros.card "Mox Diamond" (Just []) [] (MkTypeLine [] [Artifact])
       [ Static (Intercepts (Enters This Nothing) [] Nothing
                   ((May You (Macros.discard You
                           (Macros.a (And [Macros.land, InZone Macros.handZ]))) (Just (Macros.putOntoBattlefield This)) (Just (Macros.move This Macros.graveyardZ))))
                   Repeatedly Nothing)
       , Macros.activated TapSymbol
                          (AddMana You (Lit 1) (AnyColor SameColor) []) ]
       Nothing

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
                                          Compare [StatAxis ManaValue] AtLeast (Lit 5)])) Nothing) ]
           (Draw You (Lit 1)) ]
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
       , Macros.triggeredIf At (BeginningOf ThePart Upkeep (ByPlayer You))
           (CompareAmt (Macros.countOf (And [Macros.land, HasPossessor ControllerAx You]))
                       AtMost (Lit 3))
           (Macros.gainsLife You (Lit 1))
       , Macros.activated TapSymbol
                          (AddMana You (Lit 1) (Runs [[Colorless]]) []) ]
       Nothing

||| Soul Shatter
public export
soulShatter : Instruction []
soulShatter =
  Macros.sacrifice (Macros.each Opponent)
    (Macros.a (And [Or [Macros.creature, HasType Planeswalker],
                    Superlative MaxOf (StatAxis ManaValue)
                      (And [Or [Macros.creature, HasType Planeswalker],
                            HasPossessor ControllerAx They])]))

||| Padeem, Consul of Innovation
public export
padeemConsulOfInnovation : Ability
padeemConsulOfInnovation =
  Macros.triggeredIf At (BeginningOf ThePart Upkeep (ByPlayer You))
    (Matches (Macros.the (And [Macros.artifact,
                             Superlative MaxOf (StatAxis ManaValue)
                               (And [Macros.artifact,
                                     InZone Macros.battlefieldZ])]))
             (HasPossessor ControllerAx You))
    (Draw You (Lit 1))

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
                                  Compare [StatAxis ManaValue, StatAxis Power, StatAxis Toughness]
                                          Eq Macros.chosenNumber]))
                  Nothing)
           (Sequentially [ Macros.losesLife (Macros.That PlayerW) (Lit 2)
                         , (Draw You (Lit 1)) ]) ]
       (Just (3, 4))

||| Braid of Fire
public export
braidOfFire : Card
braidOfFire =
  Macros.card "Braid of Fire" (Just [Macros.generic 1, Macros.pip Red]) []
       (MkTypeLine [] [Enchantment])
       [ Macros.cumulativeUpkeep
           (Do (AddMana You (Lit 1) (Runs [[OfColor Red]]) [])) ]
       Nothing

||| Delighted Halfling
public export
delightedHalflingMana : Ability
delightedHalflingMana =
  Macros.activated TapSymbol
    (AddMana You (Lit 1) (AnyColor SameColor)
      [ OnSpent AffectsIt True
                (Macros.a (And [HasSupertype Legendary, Macros.spell]))
                (Continuously (Macros.objectCant "Counter" (Macros.That SpellW)) Nothing) ])

||| Boseiju, Who Shelters All
public export
boseijuMana : Ability
boseijuMana =
  Macros.activated (Compound [TapSymbol, Macros.payLife You 2])
    (AddMana You (Lit 1) (Runs [[Colorless]])
      [ OnSpent AffectsIt False
                (Macros.a (And [Macros.instantOrSorcery, Macros.spell]))
                (Continuously (Macros.objectCant "Counter" (Macros.That SpellW))
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
                       (Macros.gainsHaste (ResolvedPermanent (Macros.That SpellW))
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
                       (Continuously
                          (EntersRider (ResolvedPermanent (Macros.That SpellW))
                                       (WithCounters (Lit 1)
                                              (PrintedKind Macros.plusOnePlusOne)
                                              Additional))
                          Nothing) ]) ]
       (Just (2, 2))

||| Thran Turbine
public export
thranTurbineMana : Instruction []
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
       , Continuously (KeepsUnspentMana You ThisMana)
                      (Just Macros.untilEndOfTurn) ])

||| Omnath, Locus of Mana
public export
omnathLocusOfManaPersistence : StaticSpec []
omnathLocusOfManaPersistence =
  KeepsUnspentMana You (UnspentMana (Just (OfColor Green)))

||| Upwelling
public export
upwelling : Card
upwelling =
  Macros.card "Upwelling" (Just [Macros.generic 3, Macros.pip Green]) []
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
           (TappedForMana (Just (Macros.a AnyPlayer)) (Macros.a Macros.land) Nothing)
           (AddMana (Macros.That PlayerW) (Lit 1)
                    (ProducedByEvent (Macros.That (TypeW Land))) []) ]
       Nothing

||| Shimmerwilds Growth
public export
shimmerwildsGrowth : Card
shimmerwildsGrowth =
  Macros.card "Shimmerwilds Growth" (Just [Macros.generic 1, Macros.pip Green]) []
       (MkTypeLine [enchantmentType "Aura"] [Enchantment])
       [ Macros.keywordSubject "Enchant" Macros.land
       , Static (Macros.entersChoosing Macros.thisAura Color)
       , Static (Becomes (AttachHost Enchanted (TypeW Land)) Sets (ChosenQuality (Macros.ofChosen Color)))
       , Macros.triggered Whenever
           (TappedForMana Nothing (AttachHost Enchanted (TypeW Land)) Nothing)
           (AddMana (Macros.controllerOf (Macros.That (TypeW Land))) (Lit 1)
                    (OfChosenColor Nothing) []) ]
       Nothing

||| Gauntlet of Power
public export
gauntletOfPower : Card
gauntletOfPower =
  Macros.card "Gauntlet of Power" (Just [Macros.generic 5]) []
       (MkTypeLine [] [Artifact])
       [ Static (Macros.entersChoosing Macros.thisArtifact Color)
       , Static (Macros.getsPt (Macros.allOf (And [Macros.creature, Macros.ofChosen Color]))
                      (Up (Lit 1)) (Up (Lit 1)))
       , Macros.triggered Whenever
           (TappedForMana Nothing (Macros.a (And [Macros.land, HasSupertype Basic]))
                          (Just (ManaOfColor Macros.thatColor)))
           (AddMana (Macros.controllerOf (Macros.That (TypeW Land))) (Lit 1)
                    (OfChosenColor Nothing) []) ]
       Nothing

||| Chrome Mox
public export
chromeMoxMana : Ability
chromeMoxMana =
  Macros.activated TapSymbol
    (AddMana You (Lit 1)
             (AmongColorsOf (Macros.the (ExiledWith Macros.thisArtifact))) [])

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
               Do (RemoveCounters (Just (Macros.exactly 1)) (Just (PrintedKind (NamedCounter "Charge")))
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

||| Tolaria
public export
tolaria : Card
tolaria =
  Macros.card "Tolaria" Nothing [Legendary] (MkTypeLine [] [Land])
       [ Macros.activated TapSymbol (AddMana You (Lit 1) (Runs [[OfColor Blue]]) [])
       , Activated (Compound [TapSymbol])
           (Continuously
              (LosesAbilities (Macros.target Macros.creature)
                              [ LostWritten (Macros.keyword "Banding")
                              , LostTerm bandsWithOtherAbilities ])
              (Just Macros.untilEndOfTurn))
           (Just (DuringPart Upkeep Nothing)) Nothing Nothing Nothing ]
       Nothing

||| Psychic Vortex
public export
psychicVortexUpkeep : Ability
psychicVortexUpkeep =
  Macros.cumulativeUpkeep (Do (Draw You (Lit 1)))

||| Varchild's War-Riders
public export
varchildsWarRidersUpkeep : Ability
varchildsWarRidersUpkeep =
  Macros.cumulativeUpkeep
    (Do (Create Macros.anOpponent (Lit 1)
           (TokenWritten (Macros.creatureTok 1 1 [Red] [creatureType "Survivor"])) []))
