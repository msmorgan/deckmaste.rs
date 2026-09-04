module Experimental.Cards.Piles

import Experimental
import Experimental.Macros

%default total


||| Tezzeret's Gatebreaker
public export
tezzeretsGatebreaker : Card
tezzeretsGatebreaker =
  Macros.card "Tezzeret's Gatebreaker" (Just [Macros.generic 4]) []
       (MkTypeLine [] [Artifact])
       [ Macros.triggered When (Enters Macros.thisArtifact Nothing)
           (Sequentially
              [ Macros.lookAt ((Macros.topSlice (Lit 5)))
              , Macros.may You
                  (Sequentially
                     [ Macros.revealCards
                         (Macros.fromAmong (Macros.exactly 1) (Or [ColorIs Blue, Macros.artifact]) ((Macros.It ManyOf)))
                     , Macros.move (Macros.That CardW OneOf) Macros.handZ ])
              , Macros.move (Macros.theRest Object) (Macros.onBottomIn RandomOrder) ])
       , Macros.activated
           (Compound [Mana [Macros.generic 5, Macros.pip Blue], TapSymbol,
                      Do (Macros.sacrifice You Macros.thisArtifact)])
           (Macros.cantBeBlocked (Macros.allOf Macros.creatureYouControl)
                                 (Just ThisTurn)) ]
       Nothing

public export
twoCardsAtRandom : Noun [] Object
twoCardsAtRandom = Macros.countedAtRandom (Macros.exactly 2) (InZone Macros.handZ)

public export
twoCardsChosen : Noun [] Object
twoCardsChosen = Macros.counted (Macros.exactly 2) (InZone Macros.handZ)

public export
atRandomModeIsAnnouncementNeutral :
  nounDelta Piles.twoCardsAtRandom = nounDelta Piles.twoCardsChosen
atRandomModeIsAnnouncementNeutral = Refl

||| Tyrant's Choice
public export
tyrantsChoice : Card
tyrantsChoice =
  Macros.card "Tyrant's Choice"
       (Just [Macros.generic 1, Macros.pip Black]) []
       (MkTypeLine [] [Sorcery])
       [ Macros.abilityWord "will of the council"
           (Spell Nothing (Sequentially
              [ Macros.vote (Macros.each AnyPlayer) Openly (ByLabel ["death", "torture"])
              , Macros.ifThen (VoteLead "death" False)
                  (Macros.sacrifice (Macros.each Opponent)
                                    (Macros.aTheirChoice Macros.creature))
              , Macros.ifThen (VoteLead "torture" True)
                  (Macros.losesLife (Macros.each Opponent) (Lit 4)) ])) ]
       Nothing

||| Council's Judgment
public export
councilsJudgment : Card
councilsJudgment =
  Macros.card "Council's Judgment"
       (Just [Macros.generic 1, Macros.pip White, Macros.pip White]) []
       (MkTypeLine [] [Sorcery])
       [ Macros.abilityWord "will of the council"
           (Spell Nothing (Sequentially
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
       [ Macros.abilityWord "council's dilemma"
           (Macros.triggered When (Enters Macros.thisCreature Nothing)
              (Sequentially
                 [ Macros.vote (Macros.each AnyPlayer) Openly (ByLabel ["sprout", "harvest"])
                 , PutCounters (Macros.times 2 (VotesFor "sprout"))
                               (PrintedKind Macros.plusOnePlusOne)
                               Macros.thisCreature
                 , Macros.gainsLife You (Macros.times 3 (VotesFor "harvest")) ])) ]
       (Just (2, 2))

||| Plea for Power
public export
pleaForPower : Card
pleaForPower =
  Macros.card "Plea for Power"
       (Just [Macros.generic 3, Macros.pip Blue]) []
       (MkTypeLine [] [Sorcery])
       [ Macros.abilityWord "will of the council"
           (Spell Nothing (Sequentially
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
       [ Macros.abilityWord "will of the council"
           (Macros.triggered At (BeginningOf ThePart Upkeep (ByPlayer You))
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
       , Macros.abilityWord "will of the council"
           (Macros.triggered When (Enters Macros.thisCreature Nothing)
              (Sequentially
                 [ Macros.vote (Macros.each AnyPlayer) Openly
                        (ByCandidate
                           (Macros.a (And [ Or [ Macros.artifact
                                               , Macros.creature
                                               , Macros.enchantment ]
                                          , InZone (Macros.graveyardOf You) ])))
                 , Macros.returnTo (Macros.allOf (And [IsCard, WithMostVotes]))
                                   Macros.handZ [] ])) ]
       (Just (3, 3))

||| Lieutenants of the Guard
public export
lieutenantsOfTheGuard : Card
lieutenantsOfTheGuard =
  Macros.card "Lieutenants of the Guard"
       (Just [Macros.generic 4, Macros.pip White]) []
       (MkTypeLine [creatureType "Human", creatureType "Soldier"] [Creature])
       [ Macros.abilityWord "council's dilemma"
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
  Macros.abilityWord "secret council"
    (Spell Nothing (Sequentially
       [ Macros.vote (Macros.each AnyPlayer) Secretly (ByLabel ["truth", "consequences"])
       , Draw You (VotesFor "truth") ]))

||| Death or Glory
public export
deathOrGlory : Card
deathOrGlory =
  Macros.card "Death or Glory"
       (Just [Macros.generic 4, Macros.pip White]) []
       (MkTypeLine [] [Sorcery])
       [ Spell Nothing (Sequentially
                  [ SeparateIntoPiles You
                      (Macros.allOf (And [Macros.creature,
                                   InZone (Macros.graveyardOf You)])) 2 []
                  , Macros.exile You (Macros.pileOfChoice Macros.anOpponent)
                  , Macros.move (Macros.theOther Pile) Macros.battlefieldZ ]) ]
       Nothing

||| Steam Augury
public export
steamAugury : Card
steamAugury =
  Macros.card "Steam Augury"
       (Just [Macros.generic 2, Macros.pip Blue, Macros.pip Red]) []
       (MkTypeLine [] [Instant])
       [ Spell Nothing (Sequentially
                  [ Macros.revealCards (Macros.topSlice (Lit 5))
                  , SeparateIntoPiles You ((Macros.It ManyOf)) 2 []
                  , Macros.chooses Macros.anOpponent Macros.onePile
                  , Macros.move (Macros.That PileW OneOf) Macros.handZ
                  , Macros.move (Macros.theOther Pile) Macros.graveyardZ ]) ]
       Nothing

||| Do or Die
public export
doOrDie : Card
doOrDie =
  Macros.card "Do or Die" (Just [Macros.generic 1, Macros.pip Black]) []
       (MkTypeLine [] [Sorcery])
       [ Spell Nothing (Sequentially
                  [ SeparateIntoPiles You
                      (Macros.allOf (And [Macros.creature,
                                   HasPossessor ControllerAx (Macros.target AnyPlayer)])) 2 []
                  , CantBe (Macros.destroy
                              (Macros.allOf (And [Macros.creature,
                                           InPile (Macros.pileOfChoice They)])))
                           "Regenerate" (Macros.ItVerbed "Destroy" ManyOf) ]) ]
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

public export
countedAtRandomIsPlural : nounPlur Piles.twoCardsAtRandom = ManyOf
countedAtRandomIsPlural = Refl
