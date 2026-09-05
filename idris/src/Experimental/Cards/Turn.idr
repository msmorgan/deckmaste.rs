module Experimental.Cards.Turn

import Experimental
import Experimental.Macros
import Experimental.Cards.Anaphora
import Experimental.Cards.Keyword
import Experimental.Cards.Faces

%default total


||| Through the Breach Splice
throughTheBreach : Instruction []
throughTheBreach = Sequentially [Macros.may You (Macros.move (Macros.a (And [Macros.creature, InZone (Macros.handOf You)])) Macros.battlefieldZ),
                                 Macros.gainsHaste (Macros.That (TypeW Creature)) Nothing,
                                 Macros.delayed (BeginningOf ThePart EndStep NoPossessor) (Macros.sacrifice You (Macros.That (TypeW Creature)))]

turnToMist : Instruction []
turnToMist = Sequentially [Macros.exile (Macros.target Macros.creature),
                           Macros.delayed (BeginningOf ThePart EndStep NoPossessor) (Macros.move (Macros.That CardW) Macros.battlefieldZ)]

voyagerStaff : Ability
voyagerStaff = Macros.activated (Compound [Mana [Macros.generic 2], Do (Macros.sacrifice You Macros.thisArtifact)])
                                (Sequentially [Macros.exile (Macros.target Macros.creature),
                                          Macros.delayed (BeginningOf ThePart EndStep NoPossessor) (Macros.move (Macros.TheVerbed "Exile" CardW Attributive OneOf) Macros.battlefieldZ)])

staffOfNin : Ability
staffOfNin = Macros.triggered At (BeginningOf ThePart Upkeep (ByPlayer You)) (Draw You (Lit 1))

silentAssassin : Ability
silentAssassin =
  Macros.activated (Mana [Macros.generic 3, Macros.pip Black])
                   (Macros.delayed (BeginningOf ThePart EndOfCombat NoPossessor)
                            (Macros.destroy (Macros.target (And [Blocking, Macros.creature]))))

kjeldoranFrostbeast : Ability
kjeldoranFrostbeast =
  Macros.triggered At (BeginningOf ThePart EndOfCombat NoPossessor)
                   (Macros.destroy (Macros.allOf (And [Macros.creature,
                                         Or [CombatRel BlockerOf Macros.thisCreature,
                                             CombatRel BlockedBy Macros.thisCreature]])))

||| Tippy-Toe, Terrific Partner
tippyToe : Ability
tippyToe =
  Macros.triggeredIf At
                     (BeginningOf ThePart EndStep (ByPlayer You))
                     (Macros.happened LifeGain You Lookback.ThisTurn)
                     (Draw You (Lit 1))

brazenCannonade : Ability
brazenCannonade =
  Macros.abilityWord "raid"
   (Macros.triggeredIf At
                     (BeginningOf EachPart PostcombatMain (ByPlayer You))
                     (Macros.happened AttackDeclaration You Lookback.ThisTurn)
                     (Macros.exile (Macros.topSlice (Lit 1))))

fourKnocks : Ability
fourKnocks =
  Macros.triggered At (BeginningOf ThePart FirstMain (ByPlayer You)) (Draw You (Lit 1))

hammerOfBogardan : Ability
hammerOfBogardan =
  Macros.activatedOnlyDuring (Mana [Macros.generic 2, Macros.pip Red, Macros.pip Red, Macros.pip Red])
                             (Macros.move This Macros.handZ)
                             (DuringPart Upkeep (Just You))

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
           (Macros.beginningOfPossessed ThePart Upkeep
              (Macros.controllerOf (AttachHost Enchanted (TypeW Creature))))
           (May (Macros.That PlayerW)
                (Macros.discard (Macros.That PlayerW) (Macros.aAtRandom (InZone Macros.handZ)))
                (Just (Macros.untap (Macros.That (TypeW Creature))))
                Nothing) ]
       Nothing

felidarSovereign : Card
felidarSovereign =
  Macros.card "Felidar Sovereign"
       (Just [Macros.generic 4, Macros.pip White, Macros.pip White]) []
       (MkTypeLine [creatureType "Cat", creatureType "Beast"] [Creature])
       [ Macros.keyword "Vigilance"
       , Macros.keyword "Lifelink"
       , Macros.triggeredIf At
                            (BeginningOf ThePart Upkeep (ByPlayer You))
                            (CompareAmt (PlayerStatOf LifeTotal You)
                                        AtLeast (Lit 40))
                            (Concludes WinGame You) ]
       (Just (4, 6))

gloriousEnforcer : Card
gloriousEnforcer =
  Macros.card "Glorious Enforcer"
       (Just [Macros.generic 5, Macros.pip White, Macros.pip White]) []
       (MkTypeLine [creatureType "Angel"] [Creature])
       [ Macros.keyword "Flying"
       , Macros.keyword "Lifelink"
       , Macros.triggeredIf At
                            (BeginningOf ThePart Combat (ByPlayer (Macros.each AnyPlayer)))
                            (CompareAmt (PlayerStatOf LifeTotal You) Greater
                                        (PlayerStatOf LifeTotal Macros.anOpponent))
                            (Macros.gains Macros.thisCreature (Macros.keyword "DoubleStrike")
                                          (Just Macros.untilEndOfTurn)) ]
       (Just (5, 5))

||| Damia, Sage of Stone
damia : Ability
damia =
  Macros.triggeredIf At
                     (BeginningOf ThePart Upkeep (ByPlayer You))
                     (CompareAmt (Macros.countOf (InZone (Macros.handOf You)))
                                 Less (Lit 7))
                     (Draw You TheDifference)

ivoryTower : Card
ivoryTower =
  Macros.card "Ivory Tower" (Just [Macros.generic 1]) []
       (MkTypeLine [] [Artifact])
       [ Macros.triggered At (BeginningOf ThePart Upkeep (ByPlayer You))
                  (Sequentially
                     [ Macros.gainsLife You (LetterVal X)
                     , Define X (Minus (Macros.countOf (InZone (Macros.handOf You)))
                                (Lit 4)) ]) ]
       Nothing

||| Ajani, Adversary of Tyrants
ajanisEmblem : Instruction []
ajanisEmblem =
  GetsEmblem You
    [ Macros.triggered At (BeginningOf ThePart EndStep (ByPlayer You))
                       (Macros.create (Lit 3)
                   (MkToken (Just (Lit 1 ** Lit 1)) [White] (MkTypeLine [creatureType "Cat"] [Creature])
                            [Macros.keyword "Lifelink"] Nothing)) ]

||| Saheeli Rai
saheelisCopy : Instruction []
saheelisCopy =
  Sequentially [ Create You (Lit 1)
                   (TokenCopyOf (Macros.target (And [Or [Macros.artifact, Macros.creature],
                                                     HasPossessor ControllerAx You]))
                                [ExceptTypes (MkTypeLine [] [Artifact])])
                   []
               , Macros.gainsHaste (Macros.That TokenW) Nothing
               , Macros.delayed (BeginningOf ThePart EndStep NoPossessor) (Macros.exile ((Macros.It))) ]

public export
timeWalk : Card
timeWalk =
  Macros.card "Time Walk" (Just [Macros.generic 1, Macros.pip Blue]) []
       (MkTypeLine [] [Sorcery]) [Spell Nothing (ExtraTurn You (Lit 1))] Nothing

public export
timeStretch : Card
timeStretch =
  Macros.card "Time Stretch"
       (Just [Macros.generic 8, Macros.pip Blue, Macros.pip Blue]) []
       (MkTypeLine [] [Sorcery])
       [Spell Nothing (ExtraTurn (Macros.target AnyPlayer) (Lit 2))] Nothing

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
finalFortune : Card
finalFortune =
  Macros.card "Final Fortune" (Just [Macros.pip Red, Macros.pip Red]) []
       (MkTypeLine [] [Instant])
       [Spell Nothing (Sequentially [ExtraTurn You (Lit 1),
                             Macros.delayed (BeginningOf ThePart EndStep (Macros.thatTurns))
                                            (Concludes LoseGame You)])] Nothing

public export
finalFortuneExtraTurn : Instruction []
finalFortuneExtraTurn = ExtraTurn You (Lit 1)

public export
finalFortuneThatTurn : Noun (instrIntro Turn.finalFortuneExtraTurn) TurnRef
finalFortuneThatTurn = Macros.thatTurn

public export
lastChance : Card
lastChance =
  Macros.card "Last Chance" (Just [Macros.pip Red, Macros.pip Red]) []
       (MkTypeLine [] [Sorcery])
       [Spell Nothing (Sequentially [ExtraTurn You (Lit 1),
                             Macros.delayed (BeginningOf ThePart EndStep (Macros.thatTurns))
                                            (Concludes LoseGame You)])] Nothing

public export
chanceForGlory : Card
chanceForGlory =
  Macros.card "Chance for Glory"
       (Just [Macros.generic 1, Macros.pip Red, Macros.pip White]) []
       (MkTypeLine [] [Instant])
       [Spell Nothing (Sequentially
         [ Continuously (Gains (Macros.allOf (And [Macros.creature, HasPossessor ControllerAx You]))
                               (Macros.keyword "Indestructible")) Nothing
         , ExtraTurn You (Lit 1)
         , Macros.delayed (BeginningOf ThePart EndStep (Macros.thatTurns))
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
       [Spell Nothing (Sequentially
         [ SetStatus Untapped
             (Macros.allOf (And [Macros.creature,
                          Macros.happenedTo AttackDeclaration Lookback.ThisTurn]))
         , Macros.additionalPartThen Combat (Just MainPhase) (Lit 1) MainPhase])] Nothing

public export
fullThrottleFirstLine : Instruction []
fullThrottleFirstLine = Macros.additionalPart Combat (Just MainPhase) (Lit 2)

public export
yshtolaAdditionalEndStep : Instruction []
yshtolaAdditionalEndStep = Macros.additionalPart EndStep Nothing (Lit 1)

||| Sphinx of the Second Sun
public export
sphinxOfTheSecondSun : Card
sphinxOfTheSecondSun =
  Macros.card "Sphinx of the Second Sun"
       (Just [Macros.generic 6, Macros.pip Blue, Macros.pip Blue]) []
       (MkTypeLine [creatureType "Sphinx"] [Creature])
       [ Macros.keyword "Flying"
       , Macros.triggered At (BeginningOf EachPart PostcombatMain (ByPlayer You))
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
              (Macros.beginningOfPossessed ThePart Upkeep (AttachHost Enchanted PlayerW)))
           (Macros.getsAdditionalPart (Macros.That PlayerW) Upkeep (Lit 1)) ]
       Nothing

public export
ninthDoctorAdditionalUpkeep : Instruction []
ninthDoctorAdditionalUpkeep = Macros.getsAdditionalPart You Upkeep (Lit 1)

||| Rites of Flourishing
public export
ritesOfFlourishing : Card
ritesOfFlourishing =
  Macros.card "Rites of Flourishing"
       (Just [Macros.generic 2, Macros.pip Green]) []
       (MkTypeLine [] [Enchantment])
       [ Macros.triggered At (BeginningOf ThePart DrawStep (ByPlayer (Macros.each AnyPlayer)))
           (Draw (Macros.That PlayerW) (Lit 1))
       , Static (Macros.mayPlayAdditionalLands (Macros.each AnyPlayer) (Macros.exactly 1)) ]
       Nothing

public export
odricLunarchMarshal : Card
odricLunarchMarshal =
  Macros.card "Odric, Lunarch Marshal"
       (Just [Macros.generic 3, Macros.pip White]) [Legendary]
       (MkTypeLine [creatureType "Human", creatureType "Soldier"] [Creature])
       [ AlsoForKeywords
           (Macros.triggeredIf At
                               (BeginningOf ThePart Combat (ByPlayer (Macros.each AnyPlayer)))
                               (Macros.exists (And [Macros.creature, HasPossessor ControllerAx You,
                                             HasKeyword (TheKeyword "FirstStrike")]))
                               (Continuously
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
                               (BeginningOf ThePart Combat (ByPlayer You))
                               (Macros.exists (And [Macros.creature,
                                             InZone (Macros.graveyardOf You),
                                             HasKeyword (TheKeyword "Flying")]))
                               (Continuously
                                  (Gains (Macros.allOf Macros.creatureYouControl)
                                         (Macros.keyword "Flying"))
                                  (Just Macros.untilEndOfTurn)))
           (map TheKeyword
              ["FirstStrike", "DoubleStrike", "Deathtouch", "Hexproof", "Indestructible",
               "Lifelink", "Menace", "Reach", "Trample", "Vigilance"]) ]
       Nothing

public export
fettergeist : Card
fettergeist =
  Macros.card "Fettergeist" (Just [Macros.generic 2, Macros.pip Blue]) []
       (MkTypeLine [creatureType "Spirit"] [Creature])
       [ Macros.keyword "Flying"
       , Macros.triggered At (BeginningOf ThePart Upkeep (ByPlayer You))
           ((Macros.unless You (Macros.sacrifice You Macros.thisCreature)
                (Macros.scaledMana GenericUnit (Macros.forEach 1
                          (And [Macros.creature, HasPossessor ControllerAx You,
                                OtherThan Macros.thisCreature]))))) ]
       (Just (3, 4))

public export
chainVeilEndStep : Ability
chainVeilEndStep =
  Macros.triggeredIf At
                     (BeginningOf ThePart EndStep (ByPlayer You))
                     (NotCond
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
                          (Macros.beginningOfPossessed ThePart Upkeep (AttachHost Enchanted PlayerW))
                          (Macros.mills (Macros.That PlayerW) (Lit 2) They) ]
       Nothing

public export
shriekingAffliction : Card
shriekingAffliction =
  Macros.card "Shrieking Affliction" (Just [Macros.pip Black]) []
       (MkTypeLine [] [Enchantment])
       [ Macros.triggeredIf At
                            (BeginningOf ThePart Upkeep (ByPlayer (Macros.each Opponent)))
                            (CompareAmt (Macros.countOf (InZone (Macros.handOf (Macros.That PlayerW))))
                                        AtMost (Lit 1))
                            (Macros.losesLife They (Lit 3)) ]
       Nothing

||| Fractured Powerstone
public export
fracturedPowerstonePlanarRoll : Ability
fracturedPowerstonePlanarRoll =
  Macros.activatedOnlyDuring TapSymbol Macros.rollThePlanarDie AsSorcery

public export
centaurOfAttention : Card
centaurOfAttention =
  Macros.card "Centaur of Attention"
       (Just [Macros.generic 3, Macros.pip Green, Macros.pip Green]) []
       (MkTypeLine [creatureType "Centaur", creatureType "Performer"] [Creature])
       [ Macros.triggered When (Enters Macros.thisCreature Nothing)
                          (Sequentially [ (Macros.rollDice You 5 6)
                                        , StoreResults Macros.thisCreature ])
       , Macros.triggered At (BeginningOf ThePart Combat (ByPlayer You))
                          (Macros.may You
                             (RerollStored You Macros.anyNumber
                                           Macros.thisCreature))
       , Static (AndAlso Nothing [ Modify Macros.thisCreature Power (Up (LetterVal X))
                                 , Modify Macros.thisCreature Toughness (Up (LetterVal X))
                         , DefinesLetter X
                             (GreatestStoredMatch Macros.thisCreature) ]) ]
       (Just (3, 3))

||| Shapeshifter
public export
shapeshifter : Card
shapeshifter =
  Macros.cardOf "Shapeshifter" (Just [Macros.generic 6]) []
       (MkTypeLine [creatureType "Shapeshifter"] [Artifact, Creature])
       [ Static (Macros.entersChoosingFrom Macros.thisCreature Number
                                           (NumberBetween 0 7))
       , Macros.triggered At (BeginningOf ThePart Upkeep (ByPlayer You))
           (Macros.may You
              (Macros.choose (Macros.a (Macros.qualityFrom Number
                                          (NumberBetween 0 7)))))
       , Static (AndAlso Nothing
           [ DefinesPt Macros.thisCreature PowerAlone Macros.theLastChosenNumber
           , DefinesPt Macros.thisCreature ToughnessAlone
               (Minus (Lit 7) Macros.theLastChosenNumber) ]) ]
       (Just shapeshifterBox)

public export
selfSacrificeThenExile : Ability
selfSacrificeThenExile =
  Macros.activated (Do (Macros.sacrifice You Macros.thisArtifact))
    (Sequentially
       [ Macros.exile (Macros.target Macros.creature)
       , Delayed (BeginningOf ThePart EndStep NoPossessor) [] Nothing
                 (Move (Macros.That CardW) Macros.battlefieldZ
                       []) ])

||| Mirror Universe
public export
mirrorUniverse : Card
mirrorUniverse =
  Macros.card "Mirror Universe" (Just [Macros.generic 6]) []
       (MkTypeLine [] [Artifact])
       [ Macros.activatedOnlyDuring
           (Compound [TapSymbol, Do (Macros.sacrifice You Macros.thisArtifact)])
           (Exchange (LifeTotals (Both You (Macros.target Opponent))))
           (DuringPart Upkeep (Just You)) ]
       Nothing

||| Concerted Effort
public export
concertedEffort : Card
concertedEffort =
  Macros.card "Concerted Effort"
       (Just [Macros.generic 2, Macros.pip White, Macros.pip White]) []
       (MkTypeLine [] [Enchantment])
       [ AlsoForKeywords
           (Macros.triggeredIf At
                               (BeginningOf ThePart Upkeep (ByPlayer (Macros.each AnyPlayer)))
                               (Macros.exists (And [Macros.creature, HasPossessor ControllerAx You,
                                             HasKeyword (TheKeyword "Flying")]))
                               (Continuously
                                  (Gains (Macros.allOf Macros.creatureYouControl)
                                         (Macros.keyword "Flying"))
                                  (Just Macros.untilEndOfTurn)))
           [ TheKeyword "Fear", TheKeyword "FirstStrike"
           , TheKeyword "DoubleStrike", landwalkAbilities, protectionAbilities
           , TheKeyword "Trample", TheKeyword "Vigilance" ] ]
       Nothing

||| Land Tax
public export
landTax : Card
landTax =
  Macros.card "Land Tax" (Just [Macros.pip White]) []
       (MkTypeLine [] [Enchantment])
       [ Macros.triggeredIf At (BeginningOf ThePart Upkeep (ByPlayer You))
           (Macros.exists Anaphora.opponentWithMoreLands)
           (Macros.may You
              (Sequentially
                 [ Macros.searchLibraryFor (Macros.upTo 3)
                     (And [Macros.land, HasSupertype Basic])
                 , Macros.revealCards (Macros.Those CardW)
                 , Macros.move (Macros.Those CardW) Macros.handZ
                 , Macros.shuffle ])) ]
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
           (Macros.triggered At (BeginningOf ThePart Combat (ByPlayer You))
              (Macros.choose (Macros.a Opponent)))
       , Static (Macros.getsPt (Macros.allOf (And [Macros.creature,
                                   CombatRel AttackerOf (Macros.the Macros.chosenPlayer)]))
                      (Up (Lit 1)) (Up (Lit 0))) ]
       (Just (1, 3))

||| Triarch Stalker
public export
triarchStalker : Card
triarchStalker =
  Macros.card "Triarch Stalker"
       (Just [Macros.generic 3, Macros.pip Black, Macros.pip Black]) []
       (MkTypeLine [creatureType "Necron"] [Artifact, Creature])
       [ Macros.flavorWord "Targeting Relay"
           (Macros.triggered At (BeginningOf ThePart Combat (ByPlayer You))
              (Macros.choose (Macros.a Opponent)))
       , Static (Gains (Macros.allOf (And [Macros.creature,
                                    CombatRel AttackerOf (Macros.the Macros.chosenPlayer)]))
                       (Macros.keyword "Menace")) ]
       (Just (4, 5))

public export
raphaelAdditionalCombat : Instruction []
raphaelAdditionalCombat = Macros.additionalPart Combat (Just Combat) (Lit 1)

||| Karn Liberated
public export
karnRestart : Instruction []
karnRestart = RestartsGame

waxWane : Card
waxWane =
  SplitCard
    (Macros.frontFace "Wax" (Just [Macros.pip Green]) [] (MkTypeLine [] [Instant])
            [ Spell Nothing (Macros.gets (Macros.target Macros.creature)
                                 (Up (Lit 2)) (Up (Lit 2))
                                 (Just Macros.untilEndOfTurn)) ]
            Nothing)
    (Macros.frontFace "Wane" (Just [Macros.pip White]) [] (MkTypeLine [] [Instant])
            [ Spell Nothing (Macros.destroy (Macros.target Macros.enchantment)) ]
            Nothing)

||| Teleport
public export
teleport : Card
teleport =
  Macros.card "Teleport" (Just [Macros.pip Blue, Macros.pip Blue, Macros.pip Blue]) []
       (MkTypeLine [] [Instant])
       [ Spell (Just (DuringPart DeclareAttackers Nothing))
               (Macros.cantBeBlocked (Macros.target Macros.creature) (Just ThisTurn)) ]
       Nothing

||| Dazzling Beauty's cast window; its targeted effect and delayed
||| "next turn's upkeep" draw need machinery outside this ticket.
public export
dazzlingBeautyCastRestriction : Timing []
dazzlingBeautyCastRestriction = DuringPart DeclareBlockers Nothing

||| Thawing Glaciers
public export
thawingGlaciers : Card
thawingGlaciers =
  Macros.card "Thawing Glaciers" Nothing [] (MkTypeLine [] [Land])
       [ Static (Macros.entersTapped Macros.thisLand)
       , Macros.activated (Compound [Mana [Macros.generic 1], TapSymbol])
           (Sequentially
              [ Macros.searchLibraryFor (Macros.exactly 1)
                  (And [Macros.land, HasSupertype Basic])
              , Macros.putOntoBattlefieldTapped (Macros.That CardW)
              , Macros.shuffle
              , Macros.delayed (BeginningOf ThePart Cleanup NoPossessor)
                  (Macros.move Macros.thisLand Macros.handZ) ]) ]
       Nothing

||| Blood Frenzy
public export
bloodFrenzy : Card
bloodFrenzy =
  Macros.card "Blood Frenzy" (Just [Macros.generic 1, Macros.pip Red]) []
       (MkTypeLine [] [Instant])
       [ Spell (Just (BeforePart CombatDamage Nothing))
           (Sequentially
              [ Macros.gets (Macros.target (And [Macros.creature,
                                                 Or [Attacking, Blocking]]))
                            (Up (Lit 4)) (Up (Lit 0))
                            (Just Macros.untilEndOfTurn)
              , Macros.delayed (BeginningOf ThePart EndStep NoPossessor)
                  (Macros.destroy (Macros.That (TypeW Creature))) ]) ]
       Nothing

||| Berserk
public export
berserk : Card
berserk =
  Macros.card "Berserk" (Just [Macros.pip Green]) []
       (MkTypeLine [] [Instant])
       [ Spell (Just (BeforePart CombatDamage Nothing))
           (Sequentially
              [ Macros.sharedSubject (Macros.target Macros.creature)
                  [ Gains (Macros.ownSubject (Macros.target Macros.creature))
                          (Macros.keyword "Trample")
                  , Modify (Macros.ownSubject (Macros.target Macros.creature)) Power (Up (LetterVal X))
                  , Modify ((Macros.It)) Toughness (Up (Lit 0))
                  , DefinesLetter X (StatOf Power (Macros.It)) ]
                  (Just Macros.untilEndOfTurn)
              , Macros.delayed (BeginningOf ThePart EndStep NoPossessor)
                  (OnlyIf (Macros.destroy (Macros.That (TypeW Creature)))
                          (Macros.happened AttackDeclaration
                                           (Macros.That (TypeW Creature))
                                           Lookback.ThisTurn)
                          Nothing) ]) ]
       Nothing
