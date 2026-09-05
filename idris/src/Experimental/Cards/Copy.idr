module Experimental.Cards.Copy

import Experimental
import Experimental.Macros

%default total


||| Display of Power
public export
displayOfPowerCopyLock : StaticSpec []
displayOfPowerCopyLock = Macros.objectCant "Copy" This

||| Repeated Reverberation
public export
repeatedReverberation : Card
repeatedReverberation =
  Macros.card "Repeated Reverberation"
       (Just [Macros.generic 2, Macros.pip Red, Macros.pip Red]) []
       (MkTypeLine [] [Instant])
       [ Spell Nothing (Delayed
                  (Casts You (Macros.a (And [Macros.instant, Macros.spell])) Nothing)
                  [ Casts You (Macros.a (And [Macros.sorcery, Macros.spell])) Nothing
                  , Activates You (Macros.a (AbilityHead LoyaltyClass)) ]
                  (Just ThisTurn)
                  (Sequentially
                     [ Copy FromStack You (Macros.That StackW OneOf) (Lit 2) []
                     , Macros.may You (ChooseNewTargets (Macros.That CopyW ManyOf)) ])) ]
       Nothing

||| Frontline Heroism
public export
frontlineHeroismCopy : Ability
frontlineHeroismCopy =
  Macros.triggered Whenever
    (Casts You
       (Macros.a (And [ Macros.spell
                      , Targets (Macros.a (And [Macros.creature, HasPossessor ControllerAx You]))
                                SoleTarget ]))
       Nothing)
    (Sequentially
       [ Macros.create (Lit 1)
           (MkToken (Just (Lit 1 ** Lit 1)) [Red]
                    (MkTypeLine [creatureType "Soldier"] [Creature])
                    [Macros.keyword "Haste"] Nothing)
       , Copy FromStack You (Macros.That SpellW OneOf) (Lit 1) []
       , CopyTargets (Macros.That CopyW OneOf) (Macros.That TokenW OneOf) ])

||| Flawless Forgery
public export
flawlessForgeryLine : Instruction []
flawlessForgeryLine =
  Sequentially
    [ Macros.exile (Macros.target (And [ Macros.instantOrSorcery
                                       , InZone (Macros.graveyardOf
                                                   (Macros.a Opponent)) ]))
    , Copy FromCardZone You (Macros.That CardW OneOf) (Lit 1) []
    , Continuously (Macros.mayPlayDeed "Cast" You (Macros.That CopyW OneOf) Nothing
                       (PlayRider Nothing Nothing Nothing False WithoutPaying))
                   Nothing ]

public export
twincast : Card
twincast =
  Macros.card "Twincast" (Just [Macros.pip Blue, Macros.pip Blue]) []
       (MkTypeLine [] [Instant])
       [ Spell Nothing (Sequentially
                  [ Copy FromStack You
                      (Macros.target (And [Macros.instantOrSorcery, Macros.spell]))
                      (Lit 1) []
                  , Macros.may You (ChooseNewTargets (Macros.That CopyW OneOf)) ]) ]
       Nothing

public export
fork : Card
fork =
  Macros.card "Fork" (Just [Macros.pip Red, Macros.pip Red]) []
       (MkTypeLine [] [Instant])
       [ Spell Nothing (Sequentially
                  [ Copy FromStack You
                      (Macros.target (And [Macros.instantOrSorcery, Macros.spell]))
                      (Lit 1) [ExceptColor Red]
                  , Macros.may You (ChooseNewTargets (Macros.That CopyW OneOf)) ]) ]
       Nothing

public export
meletisCharlatan : Card
meletisCharlatan =
  Macros.card "Meletis Charlatan"
       (Just [Macros.generic 2, Macros.pip Blue]) []
       (MkTypeLine [creatureType "Human", creatureType "Wizard"] [Creature])
       [ Macros.activated (Compound [Mana [Macros.generic 2, Macros.pip Blue], TapSymbol])
                          (Sequentially
                      [ Copy FromStack
                          (Macros.controllerOf (Macros.target
                                           (And [Macros.instantOrSorcery, Macros.spell])))
                          ((Macros.It OneOf)) (Lit 1) []
                      , Macros.may (Macros.That PlayerW OneOf) (ChooseNewTargets (Macros.That CopyW OneOf)) ]) ]
       (Just (2, 3))

public export
echoMagesFourthLevel : Ability
echoMagesFourthLevel =
  Macros.activated (Compound [Mana [Macros.pip Blue, Macros.pip Blue], TapSymbol])
                   (Sequentially
               [ Copy FromStack You
                   (Macros.target (And [Macros.instantOrSorcery, Macros.spell]))
                   (Lit 2) []
               , Macros.may You (ChooseNewTargets (Macros.That CopyW ManyOf)) ])

||| Strionic Resonator
public export
strionicResonator : Card
strionicResonator =
  Macros.card "Strionic Resonator" (Just [Macros.generic 2]) []
       (MkTypeLine [] [Artifact])
       [ Macros.activated (Compound [Mana [Macros.generic 2], TapSymbol])
           (Sequentially
              [ Copy FromStack You
                  (Macros.target (And [AbilityHead AnyTriggered, HasPossessor ControllerAx You]))
                  (Lit 1) []
              , Macros.may You (ChooseNewTargets (Macros.That AbilityCopyW OneOf)) ]) ]
       Nothing

||| Mister Fantastic
public export
misterFantasticCopy : Ability
misterFantasticCopy =
  Macros.activated (Compound [ Mana [ Macros.pip Red, Macros.pip Green
                                    , Macros.pip White, Macros.pip Blue ]
                             , TapSymbol ])
    (Sequentially
       [ Copy FromStack You
           (Macros.target (And [AbilityHead AnyTriggered, HasPossessor ControllerAx You]))
           (Lit 2) []
       , Macros.may You (ChooseNewTargets (Macros.That AbilityCopyW ManyOf)) ])

||| Rowan's Talent
public export
rowansTalentCopy : Ability
rowansTalentCopy =
  Macros.triggered Whenever
    (Activates You
       (Macros.a (And [ AbilityHead LoyaltyClass
                      , AbilityOf (AttachHost Enchanted (TypeW Planeswalker)) ])))
    (Sequentially
       [ Copy FromStack You (Macros.That AbilityW OneOf) (Lit 1) []
       , Macros.may You (ChooseNewTargets (Macros.That AbilityCopyW OneOf)) ])

||| Rings of Brighthearth
public export
ringsOfBrighthearth : Card
ringsOfBrighthearth =
  Macros.card "Rings of Brighthearth" (Just [Macros.generic 3]) []
       (MkTypeLine [] [Artifact])
       [ Macros.triggeredIf Whenever
           (Activates You (Macros.a (AbilityHead AnyActivated)))
           (Macros.itIsntAnAbility IsManaAbility)
           ((May You (Pay You (Mana [Macros.generic 2]) PaidOnce) (Just (Sequentially
                 [ Copy FromStack You (Macros.That AbilityW OneOf) (Lit 1) []
                 , Macros.may You (ChooseNewTargets (Macros.That AbilityCopyW OneOf)) ])) Nothing)) ]
       Nothing

||| Iron Man, Bleeding Edge
public export
ironManBleedingEdge : Card
ironManBleedingEdge =
  Macros.card "Iron Man, Bleeding Edge"
       (Just [Macros.generic 3, Macros.pip Blue, Macros.pip Blue]) [Legendary]
       (MkTypeLine [creatureType "Human", creatureType "Hero"]
                   [Artifact, Creature])
       [ Macros.keyword "Flying"
       , Macros.triggeredOnlyOnce Whenever
           (Casts You (Macros.a (And [Macros.artifact, Macros.spell])) Nothing)
           ActionOncePerTurn
           (Macros.may You (Copy FromStack You ((Macros.It OneOf)) (Lit 1) [ExceptNonlegendary])) ]
       (Just (3, 5))

||| Donal, Herald of Wings
public export
donalHeraldOfWings : Card
donalHeraldOfWings =
  Macros.card "Donal, Herald of Wings"
       (Just [Macros.generic 2, Macros.pip Blue, Macros.pip Blue]) [Legendary]
       (MkTypeLine [creatureType "Human", creatureType "Wizard"] [Creature])
       [ Macros.triggeredOnlyOnce Whenever
           (Casts You
              (Macros.a (And [ Macros.creature, Macros.spell
                             , Not (HasSupertype Legendary)
                             , HasKeyword (TheKeyword "Flying") ]))
              Nothing)
           ActionOncePerTurn
           (Macros.may You
              (Copy FromStack You ((Macros.It OneOf)) (Lit 1)
                 [ExceptChars (MkToken (Just (Lit 1 ** Lit 1)) []
                                       (MkTypeLine [creatureType "Spirit"] [])
                                       [] Nothing)
                              True])) ]
       (Just (3, 3))

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
                      (Copy FromStack You ((Macros.It OneOf)) (Lit 1)
                                 [ExceptTypes (MkTypeLine [] [Artifact])])) ]
       (Just (3, 5))

public export
bonusRound : Card
bonusRound =
  Macros.card "Bonus Round"
       (Just [Macros.generic 1, Macros.pip Red, Macros.pip Red]) []
       (MkTypeLine [] [Sorcery])
       [ Spell Nothing (Delayed (Casts (Macros.a AnyPlayer)
                               (Macros.a (And [Macros.instantOrSorcery,
                                               Macros.spell])) Nothing)
                        [] (Just Macros.untilEndOfTurn)
                        (Sequentially
                           [ Copy FromStack (Macros.That PlayerW OneOf) (Macros.It OneOf) (Lit 1) []
                           , Macros.may (Macros.That PlayerW OneOf)
                               (ChooseNewTargets (Macros.That CopyW OneOf)) ])) ]
       Nothing

||| Melek, Izzet Paragon
public export
melekIzzetParagon : Card
melekIzzetParagon =
  Macros.card "Melek, Izzet Paragon"
       (Just [Macros.generic 4, Macros.pip Blue, Macros.pip Red]) [Legendary]
       (MkTypeLine [creatureType "Weird", creatureType "Wizard"] [Creature])
       [ Static (Visibility Reveal You TopOfLibrary)
       , Static ((Macros.mayPlayDeed "Cast" You (Macros.allOf (And [Macros.spell, Macros.instantOrSorcery])) Nothing (PlayRider (Just Macros.onTopZ) Nothing Nothing False ItsOwnCost)))
       , Macros.triggered Whenever
           (Casts You (Macros.a (And [Macros.instantOrSorcery, Macros.spell]))
                  (Just Macros.yourLibrary))
           (Sequentially
              [ Copy FromStack You ((Macros.It OneOf)) (Lit 1) []
              , Macros.may You (ChooseNewTargets (Macros.That CopyW OneOf)) ]) ]
       (Just (2, 4))

||| Pyromancer's Goggles
public export
pyromancersGogglesMana : Ability
pyromancersGogglesMana =
  Macros.activated TapSymbol
    (AddMana You (Lit 1) (Runs [[OfColor Red]])
      [ OnSpent TriggersThen False
                (Macros.a (And [ColorIs Red, Macros.instantOrSorcery, Macros.spell]))
                (Sequentially
                   [ Copy FromStack You (Macros.That SpellW OneOf) (Lit 1) []
                   , Macros.may You
                       (ChooseNewTargets (Macros.That CopyW OneOf)) ]) ])
