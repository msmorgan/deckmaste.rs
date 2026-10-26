import Semantics.Definitions
import Semantics.Check.Card

/-!
# Semantics.Proofs.Piles

Port of `idris/src/Experimental/Proofs/Piles.idr`: the pins of the Piles family, in theorem
form, each closed by `decide`. The names and the sentences are the Idris ones.

Not ported: `objectScopedChaos`, a Planechase sentence ("chaos ensues"), which the accord
leaves out of the grammar.
-/

open Semantics Semantics.Templates Semantics.Definitions

namespace Semantics.Proofs.Piles

private def nounSelected (drs : DRS) (noun : NounPhrase) : List Referent :=
  let result := stepNounPhrase 64 drs .other false noun
  result.2.1.filterMap result.1.findById?

/-- An instant whose whole text is one spell ability. -/
def instantSaying (action : Action) : Card :=
  .singleFaced
    { characteristics :=
      { name := some "", types := [.instant], text := [.spell none action] } }

/-- "Starting with you, each player votes for death or torture." -/
theorem okDistinctBallotOptions : Ballot.check (DRS.ofList []) (.byLabel ["death", "torture"]) = [] := by decide

/-- "Starting with you, each player votes for death or death." -/
theorem badRepeatedBallotOption :
    Ballot.check (DRS.ofList []) (.byLabel ["death", "death"]) = [.ballotLabelsOk] := by decide

/-- "Starting with you, each player votes for death." -/
theorem badSingletonBallot : Ballot.check (DRS.ofList []) (.byLabel ["death"]) = [.ballotLabelsOk] := by decide

/-- Council's Judgment: "each player votes for a nonland permanent you don't control": a
candidate ballot names one object each voter chooses [CR#701.38a,701.38b]. -/
theorem okCandidateBallot :
    Ballot.check (DRS.ofList []) (byCandidate (a (and [isPermanent, non isLand, youDontControl])))
      = [] := by decide

/-- "each player votes for all nonland permanents you don't control": a vote is for one choice
[CR#701.38a], so a candidate ballot's phrase is an indefinite, a target or a counted mention, never
a set-naming determiner. -/
theorem badSetNamingCandidateBallot :
    Ballot.check (DRS.ofList []) (byCandidate (allOf (and [isPermanent, non isLand, youDontControl])))
      = [.choosable] := by decide

/-- "Whenever a player votes": a vote watched as an event pattern needs no ballot written. -/
theorem okWildcardBallotInAPattern :
    GameEvent.check (DRS.ofList []) (does (a isPlayer) vote) = [] := by decide

/-- "Each player votes.": a performed vote names the choices it is for [CR#701.38a]; only a
pattern may leave its ballot open. -/
theorem badWildcardBallotPerformed :
    Action.check (DRS.ofList []) (instruct eachPlayer vote) = [.concreteBallot] := by decide

/-- "Put target creature card ... onto the battlefield transformed." -/
theorem okTransformedArrivalOnField :
    Action.check (DRS.ofList [])
      (.move (target (.and [isCreature, .in_ (graveyardOf .you)])) .anyLocation (some battlefield) .arrival [.entersTransformed]) = [] := by
  decide

/-- "Reveal the top five cards of your library. An opponent separates those cards into two
piles." -/
def revealAndSplit : List Action :=
  [reveal (topSlice 5), instruct anOpponent (.separateIntoPiles them 2 [])]

/-- "Reveal the top five cards of your library. An opponent separates those cards into two
piles. Put those piles into your hand." -/
theorem okPileWordAfterPartition :
    Card.check (instantSaying (.sequentially (revealAndSplit ++ [putInto (those .pile) hand])))
      = [] := by
  decide

/-- "Reveal the top five cards of your library. An opponent separates those cards into two
piles. Put those cards into your hand.": each object in a pile is still an individual object
[CR#700.3b], so the card word still reads them. -/
theorem okCardWordReadsPiles :
    Card.check (instantSaying (.sequentially (revealAndSplit ++ [putInto (those .card) hand])))
      = [] := by
  decide

/-- "Reveal the top five cards of your library. Put those piles into your hand." -/
theorem badPileWordWithoutAPartition :
    Card.check
      (instantSaying (.sequentially [reveal (topSlice 5), putInto (those .pile) hand]))
      = [.anaphor (.word .pile) .many 0] := by
  decide

/-- "... into two piles. Put one pile into your hand." -/
theorem okOnePileAfterPartition :
    Action.check (DRS.ofList []) (.sequentially (revealAndSplit ++ [putInto onePile hand])) = [] := by decide

/-- "Put target player into your hand.": a zone holds objects [CR#400.1]; a player is not
one. -/
theorem badPlayerMovedToAZone :
    Action.check (DRS.ofList []) (.move (target .isPlayer) .anyLocation (some hand) .arrival [])
      = [.kindLte .player (.join .object .pile)] := by decide

/-- "Put one pile into your hand." -/
theorem badPilePartitiveWithoutAPartition :
    Action.check (DRS.ofList []) (putInto onePile hand) = [.anaphor (.word .pile) .many 0] := by decide

/-- "Put each card in the pile of your choice into your hand." -/
theorem okMembershipInAPile :
    Card.check
      (instantSaying
        (.sequentially
          (revealAndSplit ++ [putInto (allOf (.and [.isCard, .in_ (.pile (pileOfChoice .you))])) hand])))
      = [] := by
  decide

/-- "Reveal the top five cards of your library. Put each card in those piles into your hand.":
no effect grouped them into piles [CR#700.3]. -/
theorem badMembershipWithoutAPartition :
    Card.check
      (instantSaying
        (.sequentially
          [ reveal (topSlice 5),
            putInto (allOf (.and [.isCard, .in_ (.pile (those .pile))])) hand ]))
      = [.anaphor (.word .pile) .many 0] := by
  decide

/-- "Turn target creature face down." -/
theorem okStatusOnBattlefieldNoun :
    Action.check (DRS.ofList []) (.setStatus .faceDown (target isCreature)) = [] := by decide

/-- "An opponent separates all creatures into two piles. Turn each creature in the pile of your
choice face down." Only permanents have status [CR#110.5d]; a pile is not one [CR#700.3b]. -/
theorem okStatusOnPermanentAfterPartition :
    Card.check
      (instantSaying
        (.sequentially
          [ instruct anOpponent (.separateIntoPiles (allOf isCreature) 2 []),
            .setStatus .faceDown (allOf (.and [isCreature, .in_ (.pile (pileOfChoice .you))])) ]))
      = [] := by
  decide

/-- "Separate all creatures into two piles. Turn those piles face down." -/
theorem badPileFaceAsAStatus :
    Card.check
      (instantSaying
        (.sequentially
          [ instruct anOpponent (.separateIntoPiles (allOf isCreature) 2 []),
            .setStatus .faceDown (those .pile) ])) = [.kindMismatch .object .pile] := by
  decide

/-! ## One ordered pile shape

A library, a graveyard, "that pile" and the cards exiled with a source are one `Pile`; a slice
or a run of one is an ordered group of its objects, which stay in their zone [CR#700.3c]. -/

/-- A library is kept face down [CR#401.2] and a graveyard face up [CR#404.2]. The cards exiled
with a source fix no face: exiled cards are face up by default [CR#406.3], and "exiled with" is
whatever the linked ability exiled, face up or face down [CR#406.6,607.2a]. -/
theorem zonePilesFixTheirFaces :
    [Location.face (DRS.ofList []) (.zone .library none), Location.face (DRS.ofList []) (.zone .graveyard none),
      Location.face (DRS.ofList []) (.exiledWith .this)]
      = [some .faceDown, some .faceUp, none] := by decide

/-- "Exile the top card of your graveyard.": a graveyard is ordered too [CR#404.2]. -/
theorem okSliceOfAGraveyard :
    Action.check (DRS.ofList []) (exile (.slice .top 1 (.zone .graveyard (some .you)))) = [] := by
  decide

/-- The slice of a graveyard is a group in that graveyard: grouping moves nothing, and the order
it is read in is the graveyard's [CR#700.3c]. -/
theorem sliceOfAGraveyardStaysInIt :
    (nounSelected (DRS.ofList []) (.slice .top 1 (.zone .graveyard (some .you)))).map
      Referent.zone = [some .graveyard] := by decide

/-- Bare "Shuffle" uses the performer's library in this fixture; shuffling randomizes the
chosen library [CR#701.24a]. -/
theorem okBareShuffle : Action.check (DRS.ofList []) shuffle = [] := by decide

/-- "Shuffle that pile.": no action made a pile. -/
theorem badThatPileWithoutAPile :
    Action.check (DRS.ofList []) (.shuffle (.pile (.pro (.word .pile) .one))) = [.anaphor (.word .pile) .one 0] := by decide

/-- "An opponent separates all creatures into two face-down piles. Choose one pile. Shuffle that
pile." -/
theorem okShuffleTheChosenPile :
    Action.check (DRS.ofList [])
      (.sequentially
        [ instruct anOpponent (.separateIntoPiles (allOf isCreature) 2 [.faceDown, .faceDown]),
          choose onePile, .shuffle (.pile (.pro (.word .pile) .one)) ]) = [] := by
  decide

/-- "An opponent separates all creatures into two face-down piles." -/
theorem okOneFacePerPile :
    Action.check (DRS.ofList [])
      (instruct anOpponent (.separateIntoPiles (allOf isCreature) 2 [.faceDown, .faceDown])) = [] := by
  decide

/-- "An opponent separates all creatures into two piles" with one face written: a written face
list gives each pile its face, so it names as many faces as piles. -/
theorem badFaceCountMismatch :
    Action.check (DRS.ofList [])
      (instruct anOpponent (.separateIntoPiles (allOf isCreature) 2 [.faceDown])) = [.facesFit] := by
  decide

/-- "An opponent separates all creatures into two piles. That player deals 1 damage to target
creature.": a handoff names a player. -/
theorem okHandoffAfterPartition :
    Action.check (DRS.ofList [])
      (.sequentially
        [ instruct anOpponent (separateIntoPiles (allOf isCreature) 2),
          instruct (that .player) (dealDamage 1 (target isCreature)) ]) = [] := by
  decide

/-- "An opponent separates all creatures into two piles. Those piles deal 1 damage to target
creature.": a handoff's performer is a player or an object, and a pile is not an object
[CR#700.3b]. -/
theorem badPileAsHandoffPerformer :
    Action.check (DRS.ofList [])
      (.sequentially
        [ instruct anOpponent (separateIntoPiles (allOf isCreature) 2),
          instruct (those .pile) (dealDamage 1 (target isCreature)) ]) = [.handoffPerformer] := by
  decide

/-- Cascade [CR#702.85a]: "cards exiled this way that weren't cast" reads the exiled run. -/
theorem okCascade : Ability.check (DRS.ofList []) cascade = [] := by decide

/-- Ripple [CR#702.60a]: "you may reveal the top four cards of your library … you may cast any of
those cards with the same name as this spell without paying their mana costs, then put all
revealed cards not cast this way on the bottom of your library in any order." The reveal result
remains accessible through the cast selection and subsequent move. -/
theorem rippleRevealedCardsRemainAccessible :
    Action.check (DRS.ofList [])
      (may (reveal (.slice .top 4 (libraryOf .actor)))
        (some (.sequentially
          [ castWithoutPaying
              (.someOf (.counted anyNumber) (some (.shares .cardName .share .this)) them) none,
            (.move
              (allOf (and [isCard, was (reveal which) thisWay, not (was (cast which) thisWay)]))
              .anyLocation (some onBottom) .arrival [(.inOrder .anyOrder)]) ])))
      = [] := by
  decide

theorem nestedStaticConditionals :
    StaticSpec.check (DRS.ofList [])
      (.conditional
        (.conditional (.manaRetention .you (.unspent none)) (exists_ .isPlayer) .ifSo)
        (exists_ .isPlayer) .ifSo) = [] := by
  decide

theorem nestedTurnPartWindows :
    StaticSpec.check (DRS.ofList [])
      (partScope .combat none
        (partScope .mainPhase none (.manaRetention .you (.unspent none)))) = [] := by
  decide

theorem voteStartingWithSpecifiedPlayer :
    Action.check (DRS.ofList [])
      (instruct (each .isPlayer) (.vote (some anOpponent) .openly (.byLabel ["alpha", "beta"]) none)) =
          [] := by
  decide

/-- "Starting with you, target player votes for alpha or beta.": one player votes, so there is
no order for the vote to proceed in [CR#701.38a] (`voteStartingWithSpecifiedPlayer` is the
same order over every player). -/
theorem badOrderedSingularVoter :
    Action.check (DRS.ofList [])
      (instruct (target .isPlayer) (.vote (some .you) .openly (.byLabel ["alpha", "beta"]) none))
      = [.choiceOrder] := by
  decide

/-- "Each player votes for alpha or beta. You draw a card for each alpha vote. If beta gets more
votes or the vote is tied, exile each permanent with the most votes." -/
theorem okVoteReadsAfterVote :
    Action.check (DRS.ofList [])
      (.sequentially
        [ instruct (each .isPlayer) (vote .openly (.byLabel ["alpha", "beta"])),
          draw (.votesFor "alpha"),
          doIf (.voteLead "beta" true) (exile (allOf (.and [isPermanent, .withMostVotes]))) ])
      = [] := by
  decide

/-- "You draw a card for each alpha vote.": no spell or ability instructed the players to vote,
so there are no votes to count [CR#701.38a] (`okVoteReadsAfterVote` is the same read after a
vote). -/
theorem badVotesForWithoutVote :
    Action.check (DRS.ofList []) (draw (.votesFor "alpha")) = [.outcomeInScope .voteHeld 0]
        := by
  decide

/-- "If beta gets more votes, draw a card.": the same missing vote [CR#701.38a]
(`okVoteReadsAfterVote` is the same condition after a vote). -/
theorem badVoteLeadWithoutVote :
    Action.check (DRS.ofList []) (doIf (.voteLead "beta" true) (draw 1))
      = [.outcomeInScope .voteHeld 0] := by
  decide

/-- "Exile each permanent with the most votes.": the same missing vote [CR#701.38a]
(`okVoteReadsAfterVote` exiles by the same predicate after a vote). -/
theorem badWithMostVotesWithoutVote :
    Action.check (DRS.ofList []) (exile (allOf (.and [isPermanent, .withMostVotes])))
      = [.outcomeInScope .voteHeld 0] := by
  decide

theorem oneWayResultShift :
    Action.check (DRS.ofList []) (.sequentially [rollDice 1 (.sides 6), .shiftResult [.up 1]])
      = [] := by
  decide

/-- "For each ability on the stack, put a counter of each kind on it on it": an ability on the
stack is an object [CR#109.1], so it may hold counters. -/
theorem abilityCounterRecipient :
    Action.check (DRS.ofList [])
      (.doForEach (each (.abilityHead .anyOnStack)) (.putCounters 1 (.sameAs it) it))
      = [] := by
  decide

/-- "Remove a counter of each kind you have from you": a player has counters too [CR#122.1]. -/
theorem removeOwnCounterKinds :
    Action.check (DRS.ofList []) (.removeCounters (some (exactly 1)) (some (.sameAs .you)) .you) = [] := by decide

theorem namedAdditionalPartAnchor :
    Action.check (DRS.ofList []) (Primitives.Action.addTurnPart .upkeep (some .mainPhase) 1 none)
      = [] := by
  decide

/-- "When you unlock this door, draw N cards." -/
def unlockDraw (amount : Nat) : Ability :=
  when (.does (.actor .you) (.unlock .thisDoor)) (draw (.lit amount))

/-- "When you unlock this door, draw a card.": a door header belongs to a Room's shared line. -/
theorem okDoorHeaderOnSharedLine :
    Card.check
      (.sharedLineSplit
        { characteristics :=
          { types := [.enchantment], subtypes := [room] } }
        ⟨"", some [generic 1, pip .red], [unlockDraw 1]⟩
        ⟨"", some [generic 3, pip .red], [unlockDraw 2]⟩) = [] := by
  decide

theorem badDelayedDoorDeixis :
    Card.check (instantSaying (delay (.does (.actor .you) (.unlock .thisDoor)) (draw 1)))
      = [.doorFrame] := by
  decide

end Semantics.Proofs.Piles
