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
def instantSaying (instruction : Action) : Card :=
  .singleFaced
    { characteristics :=
      { name := some "", types := [.instant], text := [.spell none instruction] } }

/-- "Starting with you, each player votes for death or torture." -/
theorem okDistinctBallotOptions : Ballot.check (DRS.ofList []) (.byLabel ["death", "torture"]) = [] := by decide

/-- "Starting with you, each player votes for death or death." -/
theorem badRepeatedBallotOption :
    Ballot.check (DRS.ofList []) (.byLabel ["death", "death"]) = [.ballotLabelsOk] := by decide

/-- "Starting with you, each player votes for death." -/
theorem badSingletonBallot : Ballot.check (DRS.ofList []) (.byLabel ["death"]) = [.ballotLabelsOk] := by decide

/-- "Put target creature card ... onto the battlefield transformed." -/
theorem okTransformedArrivalOnField :
    Action.check (DRS.ofList [])
      (.move (target (.and [isCreature, .in_ (graveyardOf .you)])) .anyLocation (some battlefield) .arrival none [.entersTransformed]) = [] := by
  decide

/-- "Reveal the top five cards of your library. An opponent separates those cards into two
piles." -/
def revealAndSplit : List Action :=
  [reveal (topSlice (.lit 5)), instruct anOpponent (.separateIntoPiles them 2 [])]

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
      (instantSaying (.sequentially [reveal (topSlice (.lit 5)), putInto (those .pile) hand]))
      = [.anaphor (.word .pile) .many 0] := by
  decide

/-- "... into two piles. Put one pile into your hand." -/
theorem okOnePileAfterPartition :
    Action.check (DRS.ofList []) (.sequentially (revealAndSplit ++ [putInto onePile hand])) = [] := by decide

/-- "Put target player into your hand.": a zone holds objects [CR#400.1]; a player is not
one. -/
theorem badPlayerMovedToAZone :
    Action.check (DRS.ofList []) (.move (target .anyPlayer) .anyLocation (some hand) .arrival none [])
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
          [ reveal (topSlice (.lit 5)),
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

A library, a graveyard, "that pile" and a face-down exile pile are one `Pile`; a slice or a run
of one is an ordered group of its objects, which stay in their zone [CR#700.3c]. -/

/-- A library is kept face down [CR#401.2], a graveyard face up [CR#404.2], and face-down
exiled cards in their own face-down pile [CR#406.4]. -/
theorem zonePilesFixTheirFaces :
    [Location.face (DRS.ofList []) (.zone .library none), Location.face (DRS.ofList []) (.zone .graveyard none),
      Location.face (DRS.ofList []) (.exiledWith .this)]
      = [some .faceDown, some .faceUp, some .faceDown] := by decide

/-- "Exile the top card of your graveyard.": a graveyard is ordered too [CR#404.2]. -/
theorem okSliceOfAGraveyard :
    Action.check (DRS.ofList []) (exile (.slice .top (.lit 1) (.zone .graveyard (some .you)))) = [] := by
  decide

/-- The slice of a graveyard is a group in that graveyard: grouping moves nothing, and the order
it is read in is the graveyard's [CR#700.3c]. -/
theorem sliceOfAGraveyardStaysInIt :
    (nounSelected (DRS.ofList []) (.slice .top (.lit 1) (.zone .graveyard (some .you)))).map
      Referent.zone = [some .graveyard] := by decide

/-- Bare "Shuffle" uses the performer's library in this fixture; shuffling randomizes the
chosen library [CR#701.24a]. -/
theorem okBareShuffle : Action.check (DRS.ofList []) shuffle = [] := by decide

/-- "Shuffle that pile.": no instruction made a pile. -/
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

/-- Cascade [CR#702.85a]: "cards exiled this way that weren't cast" reads the exiled run. -/
theorem okCascade : Ability.check (DRS.ofList []) cascadeExpansion = [] := by decide

/-- Ripple [CR#702.60a]: "you may reveal the top four cards of your library … you may cast any of
those cards with the same name as this spell without paying their mana costs, then put all
revealed cards not cast this way on the bottom of your library in any order." The reveal result
remains accessible through the cast selection and subsequent move. -/
theorem rippleRevealedCardsRemainAccessible :
    Action.check (DRS.ofList [])
      (Primitives.Action.offer (reveal (.slice .top (.lit 4) (libraryOf .actor)))
        (some (.sequentially
          [ castWithoutPaying
              (.someOf (.counted anyNumber) (some (.shares .cardName .share .this)) them) none,
            (.move (allFromAmong (.not wasCast) (theVerbed (.action "reveal") .many)) .anyLocation (some onBottom) .arrival none [(.inOrder .anyOrder)]) ])) none)
      = [] := by
  decide

theorem nestedStaticConditionals :
    StaticSpec.check (DRS.ofList [])
      (.conditional
        (.conditional (.manaRetention .you (.unspent none)) (exists_ .anyPlayer) .ifSo)
        (exists_ .anyPlayer) .ifSo) = [] := by
  decide

theorem nestedTurnPartWindows :
    StaticSpec.check (DRS.ofList [])
      (Primitives.StaticSpec.partScope .combat none
        (Primitives.StaticSpec.partScope .mainPhase none (.manaRetention .you (.unspent none)))) = [] := by
  decide

theorem voteStartingWithSpecifiedPlayer :
    Action.check (DRS.ofList [])
      (instruct (each .anyPlayer) (.vote (some anOpponent) .openly (.byLabel ["alpha", "beta"]) none)) =
          [] := by
  decide

/-- "Starting with you, target player votes for alpha or beta.": one player votes, so there is
no order for the vote to proceed in [CR#701.38a] (`voteStartingWithSpecifiedPlayer` is the
same order over every player). -/
theorem badOrderedSingularVoter :
    Action.check (DRS.ofList [])
      (instruct (target .anyPlayer) (.vote (some .you) .openly (.byLabel ["alpha", "beta"]) none))
      = [.choiceOrder] := by
  decide

/-- "Each player votes for alpha or beta. You draw a card for each alpha vote. If beta gets more
votes or the vote is tied, exile each permanent with the most votes." -/
theorem okVoteReadsAfterVote :
    Action.check (DRS.ofList [])
      (.sequentially
        [ instruct (each .anyPlayer) (vote .openly (.byLabel ["alpha", "beta"])),
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
    Action.check (DRS.ofList []) (doIf (.voteLead "beta" true) (draw (.lit 1)))
      = [.outcomeInScope .voteHeld 0] := by
  decide

/-- "Exile each permanent with the most votes.": the same missing vote [CR#701.38a]
(`okVoteReadsAfterVote` exiles by the same predicate after a vote). -/
theorem badWithMostVotesWithoutVote :
    Action.check (DRS.ofList []) (exile (allOf (.and [isPermanent, .withMostVotes])))
      = [.outcomeInScope .voteHeld 0] := by
  decide

theorem oneWayResultShift :
    Action.check (DRS.ofList []) (.sequentially [rollDice (.lit 1) (.sides 6), .shiftResult (some .up) (.lit
        1)])
      = [] := by
  decide

theorem abilityCounterRecipient :
    Action.check (DRS.ofList []) (.putCounters (.lit 1) .own (a (.abilityHead .anyOnStack)) .one true) = [] := by
  decide

theorem removeOwnCounterKinds :
    Action.check (DRS.ofList []) (.removeCounters (some (exactly 1)) (some .own) .you .one true) = [] := by decide

theorem namedAdditionalPartAnchor :
    Action.check (DRS.ofList []) (Primitives.Action.addTurnPart .upkeep (some .mainPhase) (.lit 1) none)
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
          { types := [.enchantment], subtypes := [enchantmentType "Room"] } }
        ⟨"", some [generic 1, pip .red], [unlockDraw 1]⟩
        ⟨"", some [generic 3, pip .red], [unlockDraw 2]⟩) = [] := by
  decide

theorem badDelayedDoorDeixis :
    Card.check (instantSaying (delay (.does (.actor .you) (.unlock .thisDoor)) (draw (.lit 1))))
      = [.doorFrame] := by
  decide

end Semantics.Proofs.Piles
