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

||| "Reveal the top five cards of your library. An opponent separates those
||| cards into two piles. Put those piles into your hand."
public export
okPileWordAfterPartition : Card
okPileWordAfterPartition =
  Macros.card "" Nothing (MkTypeLine [] [Instant] [])
       [ Spell Nothing (Sequentially
                  [ Macros.revealCards (Macros.topSlice (Lit 5))
                  , SeparateIntoPiles Macros.anOpponent ((Macros.Them)) 2 []
                  , Macros.move (Macros.Those PileW) Macros.handZ ]) ]
       Nothing

||| "Reveal the top five cards of your library. An opponent separates those
||| cards into two piles. Put those cards into your hand.": each object in a
||| pile is still an individual object [CR#700.3b], so the card word still
||| reads them.
public export
okCardWordReadsPiles : Card
okCardWordReadsPiles =
  Macros.card "" Nothing (MkTypeLine [] [Instant] [])
       [ Spell Nothing (Sequentially
                  [ Macros.revealCards (Macros.topSlice (Lit 5))
                  , SeparateIntoPiles Macros.anOpponent ((Macros.Them)) 2 []
                  , Macros.move (Macros.Those CardW) Macros.handZ ]) ]
       Nothing

||| "Reveal the top five cards of your library. Put those piles into your hand."
public export
badPileWordWithoutAPartition : Unspellable Card (\ok =>
  Macros.card "" Nothing (MkTypeLine [] [Instant] [])
       [ Spell Nothing (Sequentially
                  [ Macros.revealCards (Macros.topSlice (Lit 5))
                  , Macros.move (Macros.Those PileW {ok = ok}) Macros.handZ ]) ]
       Nothing)
badPileWordWithoutAPartition Refl impossible

||| "... into two piles. Put one pile into your hand."
public export
okOnePileAfterPartition : Instruction []
okOnePileAfterPartition =
  Sequentially [ Macros.revealCards (Macros.topSlice (Lit 5))
               , SeparateIntoPiles Macros.anOpponent (Macros.Them) 2 []
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
  Macros.card "" Nothing (MkTypeLine [] [Instant] [])
       [ Spell Nothing (Sequentially
                  [ Macros.revealCards (Macros.topSlice (Lit 5))
                  , SeparateIntoPiles Macros.anOpponent (Macros.Them) 2 []
                  , Macros.move (Macros.allOf (And [IsCard,
                                    InPile (Macros.pileOfChoice You)]))
                                Macros.handZ ]) ]
       Nothing

||| "Reveal the top five cards of your library. Put each card in those
||| piles into your hand." -- no effect grouped them into piles [CR#700.3].
public export
badMembershipWithoutAPartition : Unspellable Card (\ok =>
  Macros.card "" Nothing (MkTypeLine [] [Instant] [])
       [ Spell Nothing (Sequentially
                  [ Macros.revealCards (Macros.topSlice (Lit 5))
                  , Macros.move (Macros.allOf (And [IsCard,
                                    InPile (Macros.Those PileW {ok = ok})]))
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
||| "An opponent separates all creatures into two piles. Turn each creature
||| in the pile of your choice face down."
public export
okStatusOnPermanentAfterPartition : Card
okStatusOnPermanentAfterPartition =
  Macros.card "" Nothing (MkTypeLine [] [Instant] [])
       [ Spell Nothing (Sequentially
                  [ SeparateIntoPiles Macros.anOpponent (Macros.allOf Macros.creature) 2 []
                  , SetStatus FaceDown
                      (Macros.allOf (And [Macros.creature,
                                          InPile (Macros.pileOfChoice You)])) ]) ]
       Nothing

public export
badPileFaceAsAStatus : Unspellable Card (\ok =>
  Macros.card "" Nothing (MkTypeLine [] [Instant] [])
       [ Spell Nothing (Sequentially
                  [ SeparateIntoPiles Macros.anOpponent (Macros.allOf Macros.creature) 2 []
                  , SetStatus FaceDown (Macros.Those PileW) {sh = ok} ]) ]
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

||| "Starting with you, target player votes for alpha or beta." -- one player
||| votes, so there is no order for the vote to proceed in [CR#701.38a]
||| (`voteStartingWithSpecifiedPlayer` is the same order over every player).
public export
badOrderedSingularVoter : Unspellable (Instruction []) (\ok =>
  Vote (Just You) (Macros.target AnyPlayer) Openly (ByLabel ["alpha", "beta"])
       {od = ok})
badOrderedSingularVoter Oh impossible

||| "Each player votes for alpha or beta. You draw a card for each alpha vote.
||| If beta gets more votes or the vote is tied, exile each permanent with the
||| most votes."
public export
okVoteReadsAfterVote : Instruction []
okVoteReadsAfterVote =
  Sequentially
    [ Macros.vote (Macros.each AnyPlayer) Openly (ByLabel ["alpha", "beta"])
    , Draw You (VotesFor "alpha")
    , Macros.ifThen (VoteLead "beta" True)
        (Macros.exile (Macros.allOf (And [Permanent, WithMostVotes]))) ]

||| "You draw a card for each alpha vote." -- no spell or ability instructed
||| the players to vote, so there are no votes to count [CR#701.38a]
||| (`okVoteReadsAfterVote` is the same read after a vote).
public export
badVotesForWithoutVote : Unspellable (Instruction []) (\ok =>
  Draw You (VotesFor "alpha" {vt = ok}))
badVotesForWithoutVote Refl impossible

||| "If beta gets more votes, draw a card." -- the same missing vote
||| [CR#701.38a] (`okVoteReadsAfterVote` is the same condition after a vote).
public export
badVoteLeadWithoutVote : Unspellable (Instruction []) (\ok =>
  Macros.ifThen (VoteLead "beta" True {vt = ok}) (Draw You (Lit 1)))
badVoteLeadWithoutVote Refl impossible

||| "Exile each permanent with the most votes." -- the same missing vote
||| [CR#701.38a] (`okVoteReadsAfterVote` exiles by the same predicate after a
||| vote).
public export
badWithMostVotesWithoutVote : Unspellable (Instruction []) (\ok =>
  Macros.exile (Macros.allOf (And [Permanent, WithMostVotes {vt = ok}])))
badWithMostVotesWithoutVote Refl impossible

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

||| "When you unlock this door, draw a card." -- a door header belongs to a
||| Room's shared line.
public export
okDoorHeaderOnSharedLine : Card
okDoorHeaderOnSharedLine =
  SharedLineSplit (MkTypeLine [] [Enchantment] [enchantmentType "Room"]) Nothing
    (MkSharedHalf "" (Just [Macros.generic 1, Macros.pip Red])
       [ Macros.triggered When (UnlocksDoor You ThisDoor) (Draw You (Lit 1)) ])
    (MkSharedHalf "" (Just [Macros.generic 3, Macros.pip Red])
       [ Macros.triggered When (UnlocksDoor You ThisDoor) (Draw You (Lit 2)) ])

public export
badDelayedDoorDeixis : Unspellable Card (\ok =>
  Macros.card "" Nothing (MkTypeLine [] [Instant] [])
    [Spell Nothing (Delayed (UnlocksDoor You ThisDoor) [] Nothing (Draw You (Lit 1))
                    {so = Absent})]
    Nothing {fl = ok})
badDelayedDoorDeixis MkCharacteristicsLaws impossible
