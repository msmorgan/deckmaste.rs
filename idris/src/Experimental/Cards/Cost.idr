module Experimental.Cards.Cost

import Experimental
import Experimental.Macros
import Experimental.Cards.Description
import Experimental.Cards.Keyword
import Experimental.Cards.Static

%default total


masterDecoy : Ability
masterDecoy =
  Macros.activated (Compound [Mana [Macros.pip White], TapSymbol])
                   (Macros.tap (Macros.target Macros.creature))

cycling : Ability
cycling = Macros.activated (Compound [Mana [Macros.generic 2], Do (Macros.discard You This)]) (Draw You (Lit 1))

merrowGrimeblotter : Ability
merrowGrimeblotter =
  Macros.activated (Compound [Mana [Macros.generic 1, Macros.hybridPip Blue Black], UntapSymbol])
                   (Macros.gets (Macros.target Macros.creature) (Down (Lit 2)) (Down (Lit 0)) (Just Macros.untilEndOfTurn))

phyrexianSnowcrusher : Ability
phyrexianSnowcrusher =
  Macros.activated (Mana [Macros.generic 1, SnowMana])
                   (Macros.gets Macros.thisCreature (Up (Lit 1)) (Up (Lit 0)) (Just Macros.untilEndOfTurn))

havocSower : Ability
havocSower =
  Macros.activated (Mana [Macros.generic 1, Macros.colorlessPip])
                   (Macros.gets Macros.thisCreature (Up (Lit 2)) (Up (Lit 1)) (Just Macros.untilEndOfTurn))

||| Erebos, God of the Dead
erebos : Ability
erebos = Macros.activated (Compound [Mana [Macros.generic 1, Macros.pip Black], Macros.payLife You 2]) (Draw You (Lit 1))

baskingRootwalla : Ability
baskingRootwalla =
  Macros.activatedOnlyOnce (Mana [Macros.generic 1, Macros.pip Green])
                           (Macros.gets Macros.thisCreature (Up (Lit 2)) (Up (Lit 2)) (Just Macros.untilEndOfTurn))
                           OncePerTurn

securityDetail : Ability
securityDetail =
  Macros.activatedOnlyOnceIf (Mana [Macros.pip White, Macros.pip White])
                             (Macros.create (Lit 1) (Macros.creatureTok 1 1 [White] [creatureType "Soldier"]))
                             OncePerTurn
                             (NotCond (Macros.exists (And [Macros.creature, HasPossessor ControllerAx You])))

aphettoAlchemist : Ability
aphettoAlchemist = Macros.activated TapSymbol
                                    (SetStatus Untapped (Macros.target (Or [Macros.artifact, Macros.creature])))

charRumbler : Card
charRumbler =
  Macros.card "Char-Rumbler" (Just [Macros.generic 2, Macros.pip Red, Macros.pip Red]) []
       (MkTypeLine [creatureType "Elemental"] [Creature])
       [Macros.keyword "DoubleStrike",
        Macros.activated (Mana [Macros.pip Red]) (Macros.gets Macros.thisCreature (Up (Lit 1)) (Up (Lit 0)) (Just Macros.untilEndOfTurn))]
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
       [Macros.activated (Mana [Macros.generic 3]) (Macros.exile (Macros.target Macros.creatureYouControl)),
        Macros.activated (Do (Macros.sacrifice You Macros.thisArtifact))
                                (Macros.putOntoBattlefieldUnderYourControl
                     (Macros.each (And [Macros.creature, Macros.exiledWithThisArtifact])))]
       Nothing

barlsCage : Ability
barlsCage = Macros.activated (Mana [Macros.generic 3])
                             (DoesntUntapNext (Macros.target Macros.creature) (Lit 1))

vodalianIllusionist : Ability
vodalianIllusionist =
  Macros.activated (Compound [Mana [Macros.pip Blue, Macros.pip Blue], TapSymbol])
                   (SetStatus PhasedOut (Macros.target Macros.creature))

witchsMist : Ability
witchsMist =
  Macros.activated (Compound [Mana [Macros.generic 2, Macros.pip Black], TapSymbol])
                   (Macros.destroy (Macros.target (And [Macros.creature,
                                                 Macros.happenedTo DamageTaken Lookback.ThisTurn])))

goadTargetCreature : Ability
goadTargetCreature =
  Macros.activated (Compound [Mana [Macros.generic 3], TapSymbol])
                   (Macros.gainsDesignation (Macros.target Macros.creature) Goaded Instructed)

||| Krenko, Mob Boss
krenko : Ability
krenko =
  Macros.activated TapSymbol
       (Sequentially
          [ Create You (LetterVal X)
                   (TokenWritten (Macros.creatureTok 1 1 [Red] [creatureType "Goblin"])) []
          , Define X (Macros.countOf (And [HasSubtype (creatureType "Goblin"), HasPossessor ControllerAx You])) ])

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

ghalta : Card
ghalta =
  Macros.card "Ghalta, Primal Hunger"
       (Just [Macros.generic 10, Macros.pip Green, Macros.pip Green]) [Legendary]
       (MkTypeLine [creatureType "Elder", creatureType "Dinosaur"] [Creature])
       [ Static (AndAlso Nothing [ Costs This (CostLess (LetterVal X) Nothing)
                         , DefinesLetter X
                             (Macros.aggregate SumOf (StatAxis Power)
                                        Macros.creatureYouControl) ])
       , Macros.keyword "Trample" ]
       (Just (12, 12))

ancientStoneIdol : Ability
ancientStoneIdol =
  Static (Costs This
            (CostLess (Macros.forEach 1 (And [Macros.creature, Attacking])) Nothing))

thornOfAmethyst : Card
thornOfAmethyst =
  Macros.card "Thorn of Amethyst" (Just [Macros.generic 2]) []
       (MkTypeLine [] [Artifact])
       [ Static (Costs (Macros.allOf (And [Not Macros.creature, Macros.spell]))
                             (CostMore (Lit 1))) ]
       Nothing

ferozsBan : Card
ferozsBan =
  Macros.card "Feroz's Ban" (Just [Macros.generic 6]) []
       (MkTypeLine [] [Artifact])
       [ Static (Costs (Macros.allOf (And [Macros.creature, Macros.spell]))
                             (CostMore (Lit 2))) ]
       Nothing

urzasFilter : Card
urzasFilter =
  Macros.card "Urza's Filter" (Just [Macros.generic 4]) []
       (MkTypeLine [] [Artifact])
       [ Static (Costs (Macros.allOf (And [Macros.multicolored, Macros.spell]))
                             (CostLess (Lit 2) Nothing)) ]
       Nothing

emeraldMedallion : Card
emeraldMedallion =
  Macros.card "Emerald Medallion" (Just [Macros.generic 2]) []
       (MkTypeLine [] [Artifact])
       [ Static (Costs (Macros.allOf (And [ColorIs Green, Macros.spell, Macros.castBy You]))
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
       , Static (Costs
                   (Macros.the (And [Macros.spell,
                                   Macros.nthCastBy (Nth 2) You (RankEach Turn)]))
                   (CostLess (Lit 1) Nothing)) ]
       (Just (1, 4))

foundryInspector : Card
foundryInspector =
  Macros.card "Foundry Inspector" (Just [Macros.generic 3]) []
       (MkTypeLine [creatureType "Construct"] [Artifact, Creature])
       [ Static (Costs (Macros.allOf (And [Macros.artifact, Macros.spell, Macros.castBy You]))
                             (CostLess (Lit 1) Nothing)) ]
       (Just (3, 2))

daruWarchief : Card
daruWarchief =
  Macros.card "Daru Warchief"
       (Just [Macros.generic 2, Macros.pip White, Macros.pip White]) []
       (MkTypeLine [creatureType "Human", creatureType "Soldier"] [Creature])
       [ Static (Costs (Macros.allOf (And [HasSubtype (creatureType "Soldier"), Macros.spell, Macros.castBy You]))
                             (CostLess (Lit 1) Nothing))
       , Static (Macros.getsPt (Macros.allOf (And [HasSubtype (creatureType "Soldier"), Macros.creature, HasPossessor ControllerAx You]))
                      (Up (Lit 1)) (Up (Lit 2))) ]
       (Just (1, 1))

grandArbiter : Card
grandArbiter =
  Macros.card "Grand Arbiter Augustin IV"
       (Just [Macros.generic 2, Macros.pip White, Macros.pip Blue]) [Legendary]
       (MkTypeLine [creatureType "Human", creatureType "Advisor"] [Creature])
       [ Static (Costs (Macros.allOf (And [ColorIs White, Macros.spell, Macros.castBy You]))
                             (CostLess (Lit 1) Nothing))
       , Static (Costs (Macros.allOf (And [ColorIs Blue, Macros.spell, Macros.castBy You]))
                             (CostLess (Lit 1) Nothing))
       , Static (Costs (Macros.allOf (And [Macros.spell,
                                          Macros.castBy (PlayerGroup YourOpponents)]))
                             (CostMore (Lit 1))) ]
       (Just (2, 3))

goblinElectromancer : Card
goblinElectromancer =
  Macros.card "Goblin Electromancer"
       (Just [Macros.pip Blue, Macros.pip Red]) []
       (MkTypeLine [creatureType "Goblin", creatureType "Wizard"] [Creature])
       [ Static (Costs (Macros.allOf (And [Macros.instantOrSorcery, Macros.spell,
                                          Macros.castBy You]))
                             (CostLess (Lit 1) Nothing)) ]
       (Just (2, 2))

arcaneMelee : Card
arcaneMelee =
  Macros.card "Arcane Melee" (Just [Macros.generic 4, Macros.pip Blue]) []
       (MkTypeLine [] [Enchantment])
       [ Static (Costs (Macros.allOf (And [Macros.instantOrSorcery, Macros.spell]))
                             (CostLess (Lit 2) Nothing)) ]
       Nothing

manaMatrix : Card
manaMatrix =
  Macros.card "Mana Matrix" (Just [Macros.generic 6]) []
       (MkTypeLine [] [Artifact])
       [ Static (Costs (Macros.allOf (And [Or [Macros.instant, Macros.enchantment],
                                          Macros.spell, Macros.castBy You]))
                             (CostLess (Lit 2) Nothing)) ]
       Nothing

auraOfSilence : Card
auraOfSilence =
  Macros.card "Aura of Silence"
       (Just [Macros.generic 1, Macros.pip White, Macros.pip White]) []
       (MkTypeLine [] [Enchantment])
       [ Static (Costs (Macros.allOf (And [Or [Macros.artifact, Macros.enchantment],
                                          Macros.spell,
                                          Macros.castBy (PlayerGroup YourOpponents)]))
                             (CostMore (Lit 2)))
       , Macros.activated (Do (Macros.sacrifice You Macros.thisEnchantment))
                          (Macros.destroy (Macros.target (Or [Macros.artifact,
                                                       Macros.enchantment]))) ]
       Nothing

chillerpillar : Card
chillerpillar =
  Macros.card "Chillerpillar" (Just [Macros.generic 3, Macros.pip Blue]) [Snow]
       (MkTypeLine [creatureType "Insect"] [Creature])
       [ Macros.activated (Mana [Macros.generic 4, SnowMana, SnowMana])
                          (Macros.monstrosity (Lit 2))
       , Static (Macros.onlyWhile (Gains Macros.thisCreature (Macros.keyword "Flying"))
                                 (Matches Macros.thisCreature (HasDesignation Monstrous Nothing))) ]
       (Just (3, 3))

nullhideFerox : Ability
nullhideFerox =
  Macros.activated (Mana [Macros.generic 2])
                   (Continuously (LosesAllAbilities Macros.thisCreature Nothing)
                          (Just Macros.untilEndOfTurn))

causticTar : Card
causticTar =
  Macros.card "Caustic Tar" (Just [Macros.generic 4, Macros.pip Black, Macros.pip Black]) []
       (MkTypeLine [enchantmentType "Aura"] [Enchantment])
       [ Macros.keywordSubject "Enchant" Macros.land
       , Static (Gains (AttachHost Enchanted (TypeW Land))
                       (Macros.activated TapSymbol
                                         (Macros.losesLife (Macros.target AnyPlayer) (Lit 3)))) ]
       Nothing

||| Saheeli, Filigree Master
saheelisEmblem : Instruction []
saheelisEmblem =
  GetsEmblem You
    [ Static (Macros.getsPt (Macros.allOf (And [Macros.artifact, Macros.creature, HasPossessor ControllerAx You]))
                   (Up (Lit 1)) (Up (Lit 1)))
    , Static (Costs (Macros.allOf (And [Macros.artifact, Macros.spell, Macros.castBy You]))
                          (CostLess (Lit 1) Nothing)) ]

jaceBeleren : Card
jaceBeleren =
  Macros.cardOf "Jace Beleren" (Just [Macros.generic 1, Macros.pip Blue, Macros.pip Blue])
       [Legendary] (MkTypeLine [planeswalkerType "Jace"] [Planeswalker])
       [ Macros.activated (LoyaltySymbol (LoyaltyUp 2)) (Draw (Macros.each AnyPlayer) (Lit 1))
       , Macros.activated (LoyaltySymbol (LoyaltyDown 1))
                          ((Draw (Macros.target AnyPlayer) (Lit 1)))
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
                                                Compare [StatAxis Power] AtLeast (Lit 4)])))
       , Macros.activated (LoyaltySymbol (LoyaltyDown 7))
                          (GetsEmblem You
                      [ Static (AndAlso Nothing [ Modify (Macros.allOf Macros.creatureYouControl) Power (Up (Lit 2))
                                                , Modify (Macros.itsOther (Macros.allOf Macros.creatureYouControl) (Up (Lit 2))) Toughness (Up (Lit 2))
                                        , Gains ((Macros.Them)) (Macros.keyword "Flying") ]) ]) ]
       (Macros.loyaltyBox 4)

||| Elspeth's Talent
elspethsTalentGrant : StaticSpec []
elspethsTalentGrant =
  Gains (AttachHost Enchanted (TypeW Planeswalker))
        (Macros.activated (LoyaltySymbol (LoyaltyUp 1))
                          (Macros.create (Lit 3) (Macros.creatureTok 1 1 [White] [creatureType "Soldier"])))

crystalBall : Card
crystalBall =
  Macros.card "Crystal Ball" (Just [Macros.generic 3]) [] (MkTypeLine [] [Artifact])
       [ Macros.activated (Compound [Mana [Macros.generic 1], TapSymbol])
                          (Macros.scry You (Lit 2)) ]
       Nothing

||| Angus Mackenzie
public export
angusMackenzie : Card
angusMackenzie =
  Macros.card "Angus Mackenzie"
       (Just [Macros.pip Green, Macros.pip White, Macros.pip Blue]) [Legendary]
       (MkTypeLine [creatureType "Human", creatureType "Cleric"] [Creature])
       [ Macros.activatedOnlyDuring
           (Compound [Mana [Macros.pip Green, Macros.pip White, Macros.pip Blue],
                      TapSymbol])
           (Macros.preventAll CombatOnly Everywhere (Just ThisTurn))
           (BeforePart CombatDamage Nothing) ]
       (Just (2, 2))

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
aggressiveMining : Card
aggressiveMining =
  Macros.card "Aggressive Mining"
       (Just [Macros.generic 3, Macros.pip Red]) []
       (MkTypeLine [] [Enchantment])
       [ Static (Macros.cantDoTo "Play" You (Macros.allOf Macros.land))
       , Macros.activatedOnlyOnce (Do (Macros.sacrifice You (Macros.a Macros.land)))
                                  (Draw You (Lit 2))
                                  OncePerTurn ]
       Nothing

public export
rootGreevil : Card
rootGreevil =
  Macros.card "Root Greevil" (Just [Macros.generic 3, Macros.pip Green]) []
       (MkTypeLine [creatureType "Beast"] [Creature])
       [ Macros.activated (Compound [Mana [Macros.generic 2, Macros.pip Green],
                              TapSymbol,
                              Do (Macros.sacrifice You Macros.thisCreature)])
                          (Macros.destroy (Macros.allOf (And [Macros.enchantment,
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
urzasIncubator : Card
urzasIncubator =
  Macros.card "Urza's Incubator" (Just [Macros.generic 3]) []
       (MkTypeLine [] [Artifact])
       [ Static (Macros.entersChoosing Macros.thisArtifact (SubtypeQ Creature))
       , Static (Costs (Macros.allOf (And [Macros.creature, Macros.spell,
                                          Macros.ofChosen (SubtypeQ Creature)]))
                             (CostLess (Lit 2) Nothing)) ]
       Nothing

public export
etchingsOfTheChosen : Card
etchingsOfTheChosen =
  Macros.card "Etchings of the Chosen"
       (Just [Macros.generic 1, Macros.pip White, Macros.pip Black]) []
       (MkTypeLine [] [Enchantment])
       [ Static (Macros.entersChoosing Macros.thisEnchantment (SubtypeQ Creature))
       , Static (Macros.getsPt (Macros.allOf (And [Macros.creature, HasPossessor ControllerAx You,
                                   Macros.ofChosen (SubtypeQ Creature)]))
                      (Up (Lit 1)) (Up (Lit 1)))
       , Macros.activated (Compound [Mana [Macros.generic 1],
                              Do (Macros.sacrifice You
                                    (Macros.a (And [Macros.creature,
                                                    Macros.ofChosen (SubtypeQ Creature)])))])
                          (Macros.gains (Macros.target Macros.creatureYouControl)
                                 (Macros.keyword "Indestructible")
                                 (Just Macros.untilEndOfTurn)) ]
       Nothing

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
                            [ WithQuality (Macros.ofChosen Color)
                            , WithQuality (Macros.ofChosen (SubtypeQ Creature)) ])) ]
       Nothing

||| Ersatz Gnomes
public export
ersatzGnomes : Card
ersatzGnomes =
  Macros.card "Ersatz Gnomes" (Just [Macros.generic 3]) []
       (MkTypeLine [creatureType "Gnome"] [Artifact, Creature])
       [ Macros.activated TapSymbol
           (Continuously (Becomes (Macros.target Macros.spell) Sets (Colored (SomeColors [])))
                         Nothing)
       , Macros.activated TapSymbol
           (Macros.becomesColor (Macros.target Permanent) (SomeColors [])
                                (Just Macros.untilEndOfTurn)) ]
       (Just (1, 1))

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
                          (Continuously (Becomes (Macros.target Permanent) Adds
                                                 (Colored (SomeColors [Blue])))
                                        (Just Macros.untilEndOfTurn)) ]
       (Just (1, 1))

||| Arcum's Weathervane
public export
arcumsWeathervane : Card
arcumsWeathervane =
  Macros.card "Arcum's Weathervane" (Just [Macros.generic 2]) [] (MkTypeLine [] [Artifact])
       [ Macros.activated (Compound [Mana [Macros.generic 2], TapSymbol])
           (Continuously
              (Becomes (Macros.target (And [Macros.land, HasSupertype Snow])) Loses
                       (Bundle (MkTokenChars Nothing [] [Snow] (MkTypeLine [] []) [] Nothing [])
                               Nothing))
              Nothing)
       , Macros.activated (Compound [Mana [Macros.generic 2], TapSymbol])
           (Continuously
              (Becomes (Macros.target (And [Macros.land, HasSupertype Basic,
                                            Not (HasSupertype Snow)])) Adds
                       (Bundle (MkTokenChars Nothing [] [Snow] (MkTypeLine [] []) [] Nothing [])
                               Nothing))
              Nothing) ]
       Nothing

public export
candlesOfLeng : Card
candlesOfLeng =
  Macros.card "Candles of Leng" (Just [Macros.generic 2]) []
       (MkTypeLine [] [Artifact])
       [ Macros.activated (Compound [Mana [Macros.generic 4], TapSymbol])
           (Sequentially
              [ Macros.revealCards (Macros.topSlice (Lit 1))
              , If (Matches ((Macros.It)) (Named (SameNameAs
                                  (Macros.a (InZone (Macros.graveyardOf You))))))
                          (Macros.move ((Macros.It)) Macros.graveyardZ)
                          (Just (Draw You (Lit 1))) ]) ]
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
           ((Draw You (Lit 4))) ]
       (Just (5, 6))

public export
endlessAtlas : Card
endlessAtlas =
  Macros.card "Endless Atlas" (Just [Macros.generic 2]) []
       (MkTypeLine [] [Artifact])
       [ Macros.activatedOnlyIf (Compound [Mana [Macros.generic 2], TapSymbol])
                                (Draw You (Lit 1))
                                (Exists (Macros.withTheSameName
                                   (Macros.counted (Macros.atLeast 3)
                                                 (And [Macros.land,
                                                       HasPossessor ControllerAx You])))) ]
       Nothing

||| Amoeboid Changeling
public export
amoeboidChangelingTypeAbilities : AbilitySeq []
amoeboidChangelingTypeAbilities =
  [ Macros.activated TapSymbol
      (Continuously (Becomes (Macros.target Macros.creature) Adds (EveryTypeOf CreatureSpace))
                    (Just Macros.untilEndOfTurn))
  , Macros.activated TapSymbol
      (Continuously (Becomes (Macros.target Macros.creature) Loses (EveryTypeOf CreatureSpace))
                    (Just Macros.untilEndOfTurn)) ]

||| Fluctuator
public export
fluctuator : Card
fluctuator =
  Macros.card "Fluctuator" (Just [Macros.generic 2]) []
       (MkTypeLine [] [Artifact])
       [ Static (Costs
                   (Macros.allOf (And [AbilityHead (KeywordClass "Cycling"),
                                ActivatedBy You]))
                   (CostLess (Lit 2) Nothing)) ]
       Nothing

||| Boom Scholar
public export
boomScholarExhaustDiscount : Ability
boomScholarExhaustDiscount =
  Static (Costs
            (Macros.allOf (And [ AbilityHead (KeywordClass "Exhaust")
                        , AbilityOf (Macros.allOf (And [Permanent,
                                                 OtherThan Macros.thisCreature,
                                                 HasPossessor ControllerAx You])) ]))
            (CostLess (Lit 2) Nothing))

||| Hulk, Gamma Goliath
public export
hulkPowerUpDiscount : Ability
hulkPowerUpDiscount =
  Static (Costs
            (Macros.allOf (And [ AbilityHead (KeywordClass "PowerUp")
                        , AbilityOf (Macros.allOf (And [Macros.creature,
                                                 OtherThan Macros.thisCreature,
                                                 HasPossessor ControllerAx You])) ]))
            (CostLess (Lit 3) Nothing))

||| Kopala, Warden of Waves
public export
kopalaWardenOfWaves : Card
kopalaWardenOfWaves =
  Macros.card "Kopala, Warden of Waves"
       (Just [Macros.generic 1, Macros.pip Blue, Macros.pip Blue])
       [Legendary]
       (MkTypeLine [creatureType "Merfolk", creatureType "Wizard"] [Creature])
       [ Static (Costs
                   (Macros.allOf (And [ Macros.spell
                               , Macros.castBy (PlayerGroup YourOpponents)
                               , Targets (Macros.a (And [Macros.creature,
                                                         HasSubtype (creatureType "Merfolk"),
                                                         HasPossessor ControllerAx You]))
                                         SomeTarget ]))
                   (CostMore (Lit 2)))
       , Static (Costs
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
                   (AndAlso Nothing
                      [ Costs
                          (Macros.allOf (And [Macros.spell,
                                       Macros.castBy (PlayerGroup YourOpponents)]))
                          (CostMore (Lit 1))
                      , Costs
                          (Macros.allOf (And [ AbilityHead AnyActivated
                                      , ActivatedBy (PlayerGroup YourOpponents)
                                      , Not IsManaAbility ]))
                          (CostMore (Lit 1)) ]))
       , Macros.keywordNumber "Afterlife" (Lit 1) ]
       (Just (2, 1))

||| Vexing Shusher
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

public export
trainingGrounds : Card
trainingGrounds =
  Macros.card "Training Grounds" (Just [Macros.pip Blue]) []
       (MkTypeLine [] [Enchantment])
       [ Static (Costs
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
       , Static (Costs
                   (Macros.allOf (And [ AbilityHead AnyActivated
                               , AbilityOf (AttachHost Enchanted (TypeW Artifact)) ]))
                   (CostLess (Lit 2) (Just (Lit 1)))) ]
       Nothing

||| Fervent Champion
public export
ferventChampionEquipDiscount : Ability
ferventChampionEquipDiscount =
  Static (Costs
            (Macros.allOf (And [ AbilityHead (KeywordClass "Equip")
                        , ActivatedBy You
                        , Targets Macros.thisCreature SomeTarget ]))
            (CostLess (Lit 3) Nothing))

public export
suppressionField : Card
suppressionField =
  Macros.card "Suppression Field" (Just [Macros.generic 1, Macros.pip White]) []
       (MkTypeLine [] [Enchantment])
       [ Static (Costs
                   (Macros.allOf (And [AbilityHead AnyActivated, Not IsManaAbility]))
                   (CostMore (Lit 2))) ]
       Nothing

public export
gloom : Card
gloom =
  Macros.card "Gloom" (Just [Macros.generic 2, Macros.pip Black]) []
       (MkTypeLine [] [Enchantment])
       [ Static (Costs (Macros.allOf (And [Macros.spell, ColorIs White]))
                             (CostMore (Lit 3)))
       , Static (Costs
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
       [ Static (Costs
                   (Macros.allOf (And [Macros.spell, HasSubtype (artifactType "Equipment"), Macros.castBy You]))
                   (CostLess (Lit 1) Nothing))
       , Static (Costs
                   (Macros.allOf (And [AbilityHead (KeywordClass "Equip"), ActivatedBy You]))
                   (CostLess (Lit 1) Nothing)) ]
       (Just (2, 2))

public export
oppressiveRaysLine : StaticSpec []
oppressiveRaysLine =
  Costs
    (Macros.allOf (And [ AbilityHead AnyActivated
                , AbilityOf (AttachHost Enchanted (TypeW Creature)) ]))
    (CostMore (Lit 3))

public export
eidolonOfObstruction : Card
eidolonOfObstruction =
  Macros.card "Eidolon of Obstruction" (Just [Macros.generic 1, Macros.pip White]) []
       (MkTypeLine [creatureType "Spirit"] [Enchantment, Creature])
       [ Macros.keyword "FirstStrike"
       , Static (Costs
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
                   [ Macros.payLife You 1
                   , Do (Macros.exiles You (Macros.a (And [ColorIs Blue,
                                                      InZone (Macros.handOf You)]))) ])))
       , Spell Nothing (CounterSpell (Macros.target Macros.spell)) ]
       Nothing

public export
demonOfDeathsGate : Card
demonOfDeathsGate =
  Macros.card "Demon of Death's Gate"
       (Just [Macros.generic 6, Macros.pip Black, Macros.pip Black, Macros.pip Black]) []
       (MkTypeLine [creatureType "Demon"] [Creature])
       [ Static (AltCost This (Just (Compound
                   [ Macros.payLife You 6
                   , Do (Macros.sacrifice You
                           (Macros.counted (Macros.exactly 3)
                                         (And [Macros.creature, ColorIs Black]))) ])))
       , Macros.keyword "Flying"
       , Macros.keyword "Trample" ]
       (Just (9, 9))

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
hurrJackalAbility : Ability
hurrJackalAbility =
  Macros.activated TapSymbol
    (Continuously (Macros.objectCant "Regenerate" (Macros.target Macros.creature))
                  (Just ThisTurn))

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
idolOfOblivion : Card
idolOfOblivion =
  Macros.card "Idol of Oblivion" (Just [Macros.generic 2]) []
       (MkTypeLine [] [Artifact])
       [ Macros.activatedOnlyIf TapSymbol
                                (Draw You (Lit 1))
                                (Macros.happenedInvolving TokenCreation
                                                          You
                                                          Lookback.ThisTurn
                                                          (Macros.a IsToken))
       , Macros.activated (Compound [Mana [Macros.generic 8], TapSymbol,
                              Do (Macros.sacrifice You Macros.thisArtifact)])
           (Macros.create (Lit 1) (Macros.creatureTok 10 10 [] [creatureType "Eldrazi"])) ]
       Nothing

public export
patricianGeist : Card
patricianGeist =
  Macros.card "Patrician Geist" (Just [Macros.generic 2, Macros.pip Blue]) []
       (MkTypeLine [creatureType "Spirit", creatureType "Knight"] [Creature])
       [ Macros.keyword "Flying"
       , Static (Macros.getsPt (Macros.allOf (And [HasSubtype (creatureType "Spirit"), HasPossessor ControllerAx You,
                                   OtherThan Macros.thisCreature]))
                      (Up (Lit 1)) (Up (Lit 1)))
       , Static (Costs
                   (Macros.allOf (And [Macros.spell, Macros.castBy You,
                                CastFrom (Macros.graveyardOf You)]))
                   (CostLess (Lit 1) Nothing)) ]
       (Just (2, 2))

public export
shatteredEgo : Card
shatteredEgo =
  Macros.card "Shattered Ego" (Just [Macros.pip Blue]) []
       (MkTypeLine [enchantmentType "Aura"] [Enchantment])
       [ Macros.keywordSubject "Enchant" Macros.creature
       , Static (Macros.getsPt (AttachHost Enchanted (TypeW Creature))
                      (Down (Lit 3)) (Down (Lit 0)))
       , Macros.activated (Mana [Macros.generic 3, Macros.pip Blue, Macros.pip Blue])
                          (Macros.move (AttachHost Enchanted (TypeW Creature))
                                (Macros.nthFromTop (Nth 3))) ]
       Nothing

public export
shuFarmer : Card
shuFarmer =
  Macros.card "Shu Farmer" (Just [Macros.generic 1, Macros.pip White]) []
       (MkTypeLine [creatureType "Human"] [Creature])
       [ Macros.activatedOnlyDuring TapSymbol
                                    (Macros.gainsLife You (Lit 1))
                                    (BeforePart DeclareAttackers (Just You)) ]
       (Just (1, 1))

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
                      (AndAlso Nothing [ Modify (Macros.allOf Macros.creatureYouControl) Power (Up (Lit 2))
                                       , Modify (Macros.itsOther (Macros.allOf Macros.creatureYouControl) (Up (Lit 2))) Toughness (Up (Lit 2))
                               , Gains ((Macros.Them)) (Macros.keyword "Vigilance") ])
                      (Just Macros.untilEndOfTurn)) ]
       Nothing

||| Void Maw
public export
voidMawPutCost : Ability
voidMawPutCost =
  Macros.activated (Do (Macros.puts You (Macros.a (ExiledWith Macros.thisCreature))
                             Macros.graveyardZ))
                   (Macros.gets Macros.thisCreature (Up (Lit 2)) (Up (Lit 2))
                         (Just Macros.untilEndOfTurn))

||| Ghor-Clan Rampager
public export
ghorClanRampager : Ability
ghorClanRampager =
  Macros.abilityWord "bloodrush"
    (Macros.activated (Compound [Mana [Macros.pip Red, Macros.pip Green],
                          Do (Macros.discard You This)])
                      (Macros.sharedSubject
                         (Macros.target (And [Macros.creature, Attacking]))
                         [ Modify (Macros.ownSubject (Macros.target (And [Macros.creature, Attacking]))) Power (Up (Lit 4))
                         , Modify (Macros.ownSubject (Macros.target (And [Macros.creature, Attacking]))) Toughness (Up (Lit 4))
                         , Gains (Macros.ownSubject (Macros.target (And [Macros.creature, Attacking])))
                                 (Macros.keyword "Trample") ]
                         (Just Macros.untilEndOfTurn)))

||| Tymora's Invoker
public export
tymorasInvoker : Ability
tymorasInvoker =
  Macros.flavorWord "Sleight of Hand"
    (Macros.activated (Mana [Macros.generic 8]) ((Draw You (Lit 2))))

||| Skyblade's Boon
public export
skybladesBoonReturn : Ability
skybladesBoonReturn =
  Macros.activatedOnlyIf (Mana [Macros.generic 2, Macros.pip White])
    (Macros.move This Macros.handZ)
    (OrCond [ Matches This (InZone Macros.battlefieldZ)
            , Matches This (InZone (Macros.graveyardOf You)) ])

||| Bamboozling Beeble
public export
bamboozlingBeebleIgnore : Ability
bamboozlingBeebleIgnore =
  Macros.activated (Compound [Mana [Macros.generic 1], TapSymbol])
    (Macros.nextTimeWouldInstead
       (RollsDice (Macros.target AnyPlayer) ManyDice AnyDie AnyResult)
       (Sequentially [ RollDice They (Plus ThatMuch (Lit 1)) ThoseDice
                     , IgnoreOutcomes (IgnoreChosen (Just You) (Lit 1)) ])
       (Just ThisTurn))

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
                     (Up (LetterVal X)) (Up (LetterVal X))
                     (Just Macros.untilEndOfTurn)
       , Define X (DistinctCount ColorAxis (Macros.Those (TypeW Creature))) ])

||| Diplomatic Escort
public export
diplomaticEscortLine : Ability
diplomaticEscortLine =
  Macros.activated (Compound [ Mana [Macros.pip Blue]
                             , TapSymbol
                             , Do ((Macros.discard You (Macros.a (InZone Macros.handZ)))) ])
    (CounterSpell
       (Macros.target
          (And [ Or [Macros.spell, AbilityHead AnyOnStack]
               , Targets (Macros.a Macros.creature) SomeTarget ])))

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

||| Prosperity
public export
prosperityCostLetters :
  costLetters (Just [Variable, Macros.pip Blue]) = [letterB X]
prosperityCostLetters = Refl

public export
prosperityTextLetter : Amount (costLetters (Just [Variable, Macros.pip Blue]))
prosperityTextLetter = LetterVal X

public export
prosperityTextReadsCostLetter : amtDelta Cost.prosperityTextLetter = []
prosperityTextReadsCostLetter = Refl

public export
noVariableSymbolNoLetter :
  costLetters (Just [Macros.generic 1, Macros.pip Blue]) = []
noVariableSymbolNoLetter = Refl

public export
noCostNoLetter : costLetters Nothing = []
noCostNoLetter = Refl

||| Sugar Coat
public export
sugarCoat : Card
sugarCoat =
  Macros.card "Sugar Coat" (Just [Macros.generic 2, Macros.pip Blue]) []
       (MkTypeLine [enchantmentType "Aura"] [Enchantment])
       [ Macros.keyword "Flash"
       , Macros.keywordSubject "Enchant"
           (Or [Macros.creature, HasSubtype (artifactType "Food")])
       , Static (AndAlso Nothing
           [ Becomes (AttachHost Enchanted PermanentW) Sets (Bundle (MkToken Nothing []
                               (MkTypeLine [artifactType "Food"] [Artifact])
                               [ Macros.activated
                                   (Compound [ Mana [Macros.generic 2]
                                             , TapSymbol
                                             , Do (Macros.sacrifice You Macros.thisArtifact) ])
                                   (Macros.gainsLife You (Lit 3)) ]
                               Nothing) Nothing)
           , LosesAllAbilities ((Macros.It)) Nothing ]) ]
       Nothing

||| Doc Aurlock, Grizzled Genius
public export
docAurlockCost : StaticSpec []
docAurlockCost =
  Costs (Macros.allOf (And [Macros.spell, Macros.castBy You,
                           Or [ CastFrom (Macros.graveyardOf You)
                              , CastFrom Macros.exileZ ]]))
              (CostLess (Lit 2) Nothing)

||| Obelisk of Undoing
public export
obeliskOfUndoing : Ability
obeliskOfUndoing =
  Macros.activated (Compound [Mana [Macros.generic 6], TapSymbol])
                   (Macros.returnTo
                      (Macros.target (And [Permanent, HasPossessor OwnerAx You, HasPossessor ControllerAx You]))
                      Macros.handZ [])

||| Flickering Ward
public export
flickeringWardBounce : Ability
flickeringWardBounce =
  Macros.activated (Mana [Macros.pip White])
                   (Macros.returnTo Macros.thisAura Macros.handZ [])

||| Soul Conduit
public export
soulConduitExchange : Ability
soulConduitExchange =
  Macros.activated (Compound [Mana [Macros.generic 6], TapSymbol])
                   (Exchange (LifeTotals (Described (TargetDet (Macros.exactly 2)) AnyPlayer)))

||| Death-Mask Duplicant
public export
deathMaskDuplicant : Card
deathMaskDuplicant =
  Macros.card "Death-Mask Duplicant" (Just [Macros.generic 7]) []
       (MkTypeLine [creatureType "Shapeshifter"] [Artifact, Creature])
       [ Macros.abilityWord "imprint"
           (Macros.activated (Mana [Macros.generic 1])
                             (Macros.exile
                                (Macros.target
                                   (And [Macros.creature,
                                         InZone (Macros.graveyardOf You)]))))
       , AlsoForKeywords
           (Static (Macros.onlyWhile
                      (Gains Macros.thisCreature (Macros.keyword "Flying"))
                      (Macros.exists (And [ExiledWith Macros.thisCreature,
                                    HasKeyword (TheKeyword "Flying")]))))
           [ TheKeyword "Fear", TheKeyword "FirstStrike"
           , TheKeyword "DoubleStrike", TheKeyword "Haste"
           , landwalkAbilities, protectionAbilities, TheKeyword "Trample" ] ]
       (Just (5, 5))

||| Darksteel Garrison
public export
darksteelGarrison : Card
darksteelGarrison =
  Macros.card "Darksteel Garrison" (Just [Macros.generic 2]) []
       (MkTypeLine [artifactType "Fortification"] [Artifact])
       [ Static (Gains (AttachHost Fortified (TypeW Land))
                       (Macros.keyword "Indestructible"))
       , Macros.triggered Whenever
           (TappedForMana Nothing (AttachHost Fortified (TypeW Land)) Nothing)
           (Macros.gets (Macros.target Macros.creature)
                        (Up (Lit 1)) (Up (Lit 1)) (Just Macros.untilEndOfTurn))
       , Macros.keywordCosting "Fortify" (Mana [Macros.generic 3]) ]
       Nothing

||| Embercleave
public export
embercleave : Card
embercleave =
  Macros.card "Embercleave"
       (Just [Macros.generic 4, Macros.pip Red, Macros.pip Red]) [Legendary]
       (MkTypeLine [artifactType "Equipment"] [Artifact])
       [ Macros.keyword "Flash"
       , Static (Costs This
                   (CostLess (Macros.forEach 1
                                (And [Macros.creature, Attacking, HasPossessor ControllerAx You]))
                             Nothing))
       , Macros.triggered When (Enters Macros.thisEquipment Nothing)
           (AttachTo ((Macros.It)) (Macros.target Macros.creatureYouControl))
       , Static (AndAlso (Just (AttachHost Equipped (TypeW Creature)))
                   [ Modify (Macros.ownSubject (AttachHost Equipped (TypeW Creature))) Power (Up (Lit 1))
                   , Modify (Macros.ownSubject (AttachHost Equipped (TypeW Creature))) Toughness (Up (Lit 1))
                   , Gains (Macros.ownSubject (AttachHost Equipped (TypeW Creature)))
                           (Macros.keyword "DoubleStrike")
                   , Gains (Macros.ownSubject (AttachHost Equipped (TypeW Creature)))
                           (Macros.keyword "Trample") ])
       , Macros.keywordCosting "Equip" (Mana [Macros.generic 3]) ]
       Nothing

||| Tattoo Ward
public export
tattooWard : Card
tattooWard =
  Macros.card "Tattoo Ward" (Just [Macros.generic 2, Macros.pip White]) []
       (MkTypeLine [enchantmentType "Aura"] [Enchantment])
       [ Macros.keywordSubject "Enchant" Macros.creature
       , Static (DoesntRemove
                   (AndAlso (Just (AttachHost Enchanted (TypeW Creature)))
                      [ Modify (Macros.ownSubject (AttachHost Enchanted (TypeW Creature))) Power (Up (Lit 1))
                      , Modify (Macros.ownSubject (AttachHost Enchanted (TypeW Creature))) Toughness (Up (Lit 1))
                      , Gains (Macros.ownSubject (AttachHost Enchanted (TypeW Creature)))
                              (Macros.keywordQuality "Protection" Macros.enchantment) ])
                   Macros.thisAura)
       , Macros.activated (Do (Macros.sacrifice You Macros.thisAura))
           (Macros.destroy (Macros.target Macros.enchantment)) ]
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
                          (Macros.keywordQuality "Protection" (Macros.ofChosen Color)))
                   Macros.thisAura)
       , Macros.activated (Do (Macros.sacrifice You Macros.thisAura))
           (Macros.gains (Macros.target Macros.creature)
                         (Macros.keywordQuality "Protection" (Macros.ofChosen Color))
                         (Just Macros.untilEndOfTurn)) ]
       Nothing

||| Ghostfire Blade
public export
ghostfireBlade : Card
ghostfireBlade =
  Macros.card "Ghostfire Blade" (Just [Macros.generic 1]) []
       (MkTypeLine [artifactType "Equipment"] [Artifact])
       [ Static (Macros.getsPt (AttachHost Equipped (TypeW Creature))
                      (Up (Lit 2)) (Up (Lit 2)))
       , Macros.keywordCosting "Equip" (Mana [Macros.generic 3])
       , Static (Costs
                   (Macros.allOf (And [ AbilityHead (KeywordClass "Equip")
                               , AbilityOf This
                               , Targets (Macros.a (And [Macros.creature, IsColorless]))
                                         SomeTarget ]))
                   (CostLess (Lit 2) Nothing)) ]
       Nothing

||| Academy Journeymage
public export
academyJourneymage : Card
academyJourneymage =
  Macros.card "Academy Journeymage" (Just [Macros.generic 4, Macros.pip Blue]) []
       (MkTypeLine [creatureType "Human", creatureType "Wizard"] [Creature])
       [ Static (Macros.onlyIfSo (Costs This (CostLess (Lit 1) Nothing))
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
       [ Static (Costs (Macros.allOf (And [Macros.spell, ColorIs White, Macros.castBy You]))
                             (CostShiftRun [Macros.pip White] True False)) ]
       (Just (1, 3))

||| Edgewalker
public export
edgewalker : Card
edgewalker =
  Macros.card "Edgewalker"
       (Just [Macros.generic 1, Macros.pip White, Macros.pip Black]) []
       (MkTypeLine [creatureType "Human", creatureType "Cleric"] [Creature])
       [ Static (Costs (Macros.allOf (And [HasSubtype (creatureType "Cleric"),
                                          Macros.spell, Macros.castBy You]))
                             (CostShiftRun [Macros.pip White, Macros.pip Black]
                                           False True)) ]
       (Just (2, 2))

||| Cavern-Hoard Dragon
public export
cavernHoardDragonRider : Ability
cavernHoardDragonRider =
  Static (AndAlso Nothing [ Costs This (CostLess (LetterVal X) Nothing)
                  , DefinesLetter X (AggregateOver MaxOf Opponent
                                (Macros.countOf (And [Macros.artifact, HasPossessor ControllerAx They]))) ])

||| Shadowspear
public export
shadowspearStrip : Ability
shadowspearStrip =
  Macros.activated (Mana [Macros.generic 1])
    (Continuously
       (LosesAbilities
          (Macros.allOf (And [Permanent, HasPossessor ControllerAx (PlayerGroup YourOpponents)]))
          [ LostWritten (Macros.keyword "Hexproof")
          , LostWritten (Macros.keyword "Indestructible") ])
       (Just Macros.untilEndOfTurn))

||| Shay Cormac
public export
shayCormacStrip : Ability
shayCormacStrip =
  Macros.activated (Mana [Macros.generic 1])
    (Continuously
       (LosesAbilities
          (Macros.allOf (And [Permanent, HasPossessor ControllerAx (PlayerGroup YourOpponents)]))
          [ LostWritten (Macros.keyword "Hexproof")
          , LostWritten (Macros.keyword "Indestructible")
          , LostTerm protectionAbilities
          , LostWritten (Macros.keyword "Shroud")
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
           (Continuously
              (LosesAbilities (Macros.target Macros.creature)
                              [LostTerm bandsWithOtherAbilities])
              (Just Macros.untilEndOfTurn)) ]
       (Just (1, 1))

||| Ahn-Crop Invader
public export
ahnCropInvader : Card
ahnCropInvader =
  Macros.card "Ahn-Crop Invader" (Just [Macros.generic 2, Macros.pip Red]) []
       (MkTypeLine [creatureType "Zombie", creatureType "Minotaur",
                    creatureType "Warrior"] [Creature])
       [ Static (OnlyDuring Turn (Just You)
                   (Gains Macros.thisCreature
                          (Macros.keyword "FirstStrike")))
       , Macros.activated
           (Compound [ Mana [Macros.generic 1]
                     , Do (Macros.sacrifice You
                             (Macros.a (And [Macros.creature, OtherThan This]))) ])
           (Macros.gets Macros.thisCreature (Up (Lit 2)) (Up (Lit 0))
                        (Just Macros.untilEndOfTurn)) ]
       (Just (2, 2))

||| Nesting Dragon
public export
nestingDragonInnerToken : AbilityAt []
nestingDragonInnerToken =
  Macros.activated (Mana [Macros.pip Red])
    (Continuously (AndAlso Nothing
                     [ Modify (AsMarker TokenMarker This) Power (Up (Lit 1))
                     , Modify (AsMarker TokenMarker This) Toughness (Up (Lit 0)) ])
                  (Just Macros.untilEndOfTurn))

||| Leonin Bola
public export
leoninBola : Card
leoninBola =
  Macros.card "Leonin Bola" (Just [Macros.generic 1]) []
       (MkTypeLine [artifactType "Equipment"] [Artifact])
       [ Static (Gains (AttachHost Equipped (TypeW Creature))
                   (Macros.activated
                      (Compound [TapSymbol, Do (Unattach (TheGrantor PermanentMarker))])
                      (SetStatus Tapped (Macros.target Macros.creature))))
       , Macros.keywordCosting "Equip" (Mana [Macros.generic 1]) ]
       Nothing

||| Alluring Suitor // Deadly Dancer
public export
deadlyDancerPump : Ability
deadlyDancerPump =
  Macros.activated (Mana [Macros.pip Red, Macros.pip Red])
    (Continuously (AndAlso Nothing
                     [ Modify (EachOf
                                 (Both Macros.thisCreature
                                       (Macros.target (And [Macros.creature, OtherThan This]))))
                              Power (Up (Lit 1))
                     , Modify (EachOf (Both Macros.thisCreature ((Macros.It))))
                              Toughness (Up (Lit 0)) ])
                  (Just Macros.untilEndOfTurn))

||| Sphinx of Clear Skies

||| Unesh, Criosphinx Sovereign
public export
uneshCriosphinxSovereign : Card
uneshCriosphinxSovereign =
  Macros.card "Unesh, Criosphinx Sovereign"
       (Just [Macros.generic 4, Macros.pip Blue, Macros.pip Blue]) [Legendary]
       (MkTypeLine [creatureType "Sphinx"] [Creature])
       [ Macros.keyword "Flying"
       , Static (Costs
                   (Macros.allOf (And [HasSubtype (creatureType "Sphinx"),
                                Macros.spell, Macros.castBy You]))
                   (CostLess (Lit 2) Nothing))
       , Macros.triggered Whenever
           (Enters (EitherOf Macros.thisCreature
                      (Macros.a (And [HasSubtype (creatureType "Sphinx"),
                                      HasPossessor ControllerAx You,
                                      OtherThan Macros.thisCreature])))
                   Nothing)
           (Sequentially
              [ Macros.revealCards (Macros.topSlice (Lit 4))
              , SeparateIntoPiles Macros.anOpponent ((Macros.Them)) 2 []
              , Macros.move Macros.onePile Macros.handZ
              , Macros.move (Macros.theOther Pile) Macros.graveyardZ ]) ]
       (Just (4, 4))

opt : Card
opt =
  Macros.card "Opt" (Just [Macros.pip Blue]) [] (MkTypeLine [] [Instant])
       [ Spell Nothing (Sequentially [ Macros.scry You (Lit 1)
                             , (Draw You (Lit 1)) ]) ]
       Nothing

serumVisions : Card
serumVisions =
  Macros.card "Serum Visions" (Just [Macros.pip Blue]) [] (MkTypeLine [] [Sorcery])
       [ Spell Nothing (Sequentially [ (Draw You (Lit 1))
                             , Macros.scry You (Lit 2) ]) ]
       Nothing

consider : Card
consider =
  Macros.card "Consider" (Just [Macros.pip Blue]) [] (MkTypeLine [] [Instant])
       [ Spell Nothing (Sequentially [ Macros.surveil You (Lit 1)
                             , (Draw You (Lit 1)) ]) ]
       Nothing

public export
wordsOfWisdom : Card
wordsOfWisdom =
  Macros.card "Words of Wisdom"
       (Just [Macros.generic 1, Macros.pip Blue]) []
       (MkTypeLine [] [Instant])
       [ Spell Nothing (Sequentially
                  [ Draw You (Lit 2)
                  , Draw (Macros.each (Macros.otherPlayer)) (Lit 1) ]) ]
       Nothing

public export
deathWard : Card
deathWard =
  Macros.card "Death Ward" (Just [Macros.pip White]) []
       (MkTypeLine [] [Instant])
       [ Spell Nothing (Regenerate (Macros.target Macros.creature)) ]
       Nothing

public export
missyChaosBranch : Instruction []
missyChaosBranch = Sequentially [(Draw You (Lit 1)), Macros.chaosEnsues]

public export
bareTextLetter : Amount []
bareTextLetter = LetterVal X

public export
textAloneOnceMintedItsOwnLetter : amtDelta Cost.bareTextLetter = [letterB X]
textAloneOnceMintedItsOwnLetter = Refl
