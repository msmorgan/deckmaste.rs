module Experimental.ProofsPiles

import Experimental
import Experimental.Macros
import Experimental.Unspellable

%default total

%unbound_implicits off


||| "Starting with you, each player votes for death or torture."
public export
okDistinctBallotOptions : Ballot []
okDistinctBallotOptions = ByLabel ["death", "torture"]

||| "Starting with you, each player votes for death or death."
public export
badRepeatedBallotOption : Unspellable (Ballot []) (\ok =>
  ByLabel ["death", "death"] {ok})
badRepeatedBallotOption Oh impossible

||| "Starting with you, each player votes for death."
public export
badSingletonBallot : Unspellable (Ballot []) (\ok =>
  ByLabel ["death"] {ok})
badSingletonBallot Oh impossible

||| "Put target creature card ... onto the battlefield transformed."
public export
okTransformedArrivalOnField : Instruction []
okTransformedArrivalOnField =
  Move (Macros.target (And [Macros.creature, InZone (Macros.graveyardOf You)]))
       Macros.battlefieldZ [EntersTransformed]

||| "Return target creature card from your graveyard to your hand transformed."
public export
badTransformedArrivalOffField : Unspellable (Instruction []) (\ok =>
  Move (Macros.target (And [Macros.creature, InZone (Macros.graveyardOf You)]))
       Macros.handZ [EntersTransformed] {rf = ok})
badTransformedArrivalOffField Oh impossible

public export
badCardWordReadsPiles : Unspellable Card (\ok =>
  Macros.card "" Nothing [] (MkTypeLine [] [Instant])
       [ Spell (Sequentially
                  [ Macros.revealCards (Macros.topSlice (Lit 5))
                  , SeparateIntoPiles Macros.anOpponent ((Macros.It ManyOf)) 2 []
                  , Macros.move (Macros.That CardW ManyOf {ok = ok}) Macros.handZ ]) ]
       Nothing)
badCardWordReadsPiles Refl impossible

||| "Reveal the top five cards of your library. Put those piles into your hand."
public export
badPileWordWithoutAPartition : Unspellable Card (\ok =>
  Macros.card "" Nothing [] (MkTypeLine [] [Instant])
       [ Spell (Sequentially
                  [ Macros.revealCards (Macros.topSlice (Lit 5))
                  , Macros.move (Macros.That PileW ManyOf {ok = ok}) Macros.handZ ]) ]
       Nothing)
badPileWordWithoutAPartition Refl impossible

||| "... into two piles. Put one pile into your hand."
public export
okOnePileAfterPartition : Instruction []
okOnePileAfterPartition =
  Sequentially [ Macros.revealCards (Macros.topSlice (Lit 5))
               , SeparateIntoPiles Macros.anOpponent (Macros.It ManyOf) 2 []
               , Macros.move Macros.onePile Macros.handZ ]

||| "Put target player into your hand." -- a zone holds objects [CR#400.1];
||| a player is not one.
public export
badPlayerMovedToAZone : Unspellable (Instruction []) (\ok =>
  Move (Macros.target AnyPlayer) Macros.handZ [] {mk = ok})
badPlayerMovedToAZone ObjectMoves impossible
badPlayerMovedToAZone PileMoves impossible

||| "Put one pile into your hand."
public export
badPilePartitiveWithoutAPartition : Unspellable (Instruction []) (\ok =>
  Macros.move (Macros.onePile {ok = ok}) Macros.handZ)
badPilePartitiveWithoutAPartition Refl impossible

||| "Put each card in the pile of your choice into your hand."
public export
okMembershipInAPile : Card
okMembershipInAPile =
  Macros.card "" Nothing [] (MkTypeLine [] [Instant])
       [ Spell (Sequentially
                  [ Macros.revealCards (Macros.topSlice (Lit 5))
                  , SeparateIntoPiles Macros.anOpponent (Macros.It ManyOf) 2 []
                  , Macros.move (Macros.allOf (And [IsCard,
                                    InPile (Macros.pileOfChoice You)]))
                                Macros.handZ ]) ]
       Nothing

||| "Reveal the top five cards of your library. Put each card in those
||| piles into your hand." -- no effect grouped them into piles [CR#700.3].
public export
badMembershipWithoutAPartition : Unspellable Card (\ok =>
  Macros.card "" Nothing [] (MkTypeLine [] [Instant])
       [ Spell (Sequentially
                  [ Macros.revealCards (Macros.topSlice (Lit 5))
                  , Macros.move (Macros.allOf (And [IsCard,
                                    InPile (Macros.That PileW ManyOf {ok = ok})]))
                                Macros.handZ ]) ]
       Nothing)
badMembershipWithoutAPartition Refl impossible

||| "Turn target creature face down."
public export
okStatusOnBattlefieldNoun : Instruction []
okStatusOnBattlefieldNoun =
  SetStatus FaceDown (Macros.target Macros.creature)

||| "Separate all creatures into two piles. Turn those piles face down."
||| -- only permanents have status [CR#110.5d]; a pile is not one [CR#700.3b].
public export
badPileFaceAsAStatus : Unspellable Card (\ok =>
  Macros.card "" Nothing [] (MkTypeLine [] [Instant])
       [ Spell (Sequentially
                  [ SeparateIntoPiles Macros.anOpponent (Macros.allOf Macros.creature) 2 []
                  , SetStatus FaceDown (Macros.That PileW ManyOf) {sh = ok} ]) ]
       Nothing)
badPileFaceAsAStatus ObjectHoldsStatus impossible

public export
nestedStaticConditionals : StaticSpec []
nestedStaticConditionals =
  Conditionally
    (Conditionally (KeepsUnspentMana You (UnspentMana Nothing))
                   (Macros.exists AnyPlayer) IfSo)
    (Macros.exists AnyPlayer) IfSo

public export
nestedTurnPartWindows : StaticSpec []
nestedTurnPartWindows =
  OnlyDuring Combat Nothing
    (OnlyDuring MainPhase Nothing (KeepsUnspentMana You (UnspentMana Nothing)))

public export
voteStartingWithSpecifiedPlayer : Instruction []
voteStartingWithSpecifiedPlayer =
  Vote (Just Macros.anOpponent) (Macros.each AnyPlayer) Openly
       (ByLabel ["alpha", "beta"])

public export
oneWayResultShift : Instruction []
oneWayResultShift =
  Sequentially [(Macros.rollDice You 1 6), ShiftResult (Just ShiftUp) (Lit 1)]

public export
objectScopedChaos : Instruction []
objectScopedChaos = ChaosEnsues (Just Macros.thisRoom)

public export
abilityCounterRecipient : Instruction []
abilityCounterRecipient =
  PutCounters (Lit 1) OwnKinds (Macros.a (AbilityHead AnyOnStack))

public export
removeOwnCounterKinds : Instruction []
removeOwnCounterKinds = RemoveCounters (Just (Macros.exactly 1)) (Just OwnKinds) You

public export
namedAdditionalPartAnchor : Instruction []
namedAdditionalPartAnchor =
  AdditionalPart (Just You) Upkeep (Just MainPhase) (Lit 1) Nothing

public export
badDelayedDoorDeixis : Unspellable Card (\ok =>
  Macros.card "" Nothing [] (MkTypeLine [] [Instant])
    [Spell (Delayed (UnlocksDoor You ThisDoor) [] Nothing (Draw You (Lit 1))
                    {so = Absent})]
    Nothing {fl = ok})
badDelayedDoorDeixis MkCharacteristicsLaws impossible
