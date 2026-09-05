import Semantics.Macros
import Semantics.Check.Card

/-!
# Semantics.Proofs.Piles

Port of `idris/src/Experimental/Proofs/Piles.idr`: the pins of the Piles family, in theorem
form, each closed by `decide`. The names and the sentences are the Idris ones.

Not ported: `objectScopedChaos`, a Planechase sentence ("chaos ensues"), which the accord
leaves out of the grammar.
-/

open Semantics Semantics.Macros

namespace Semantics.Proofs.Piles

/-- An instant whose whole text is one spell ability. -/
def instantSaying (instruction : Instruction) : Card :=
  .singleFaced { name := some "", types := [.instant], text := [.spell none instruction] }

/-- "Starting with you, each player votes for death or torture." -/
theorem okDistinctBallotOptions : Ballot.check [] (.byLabel ["death", "torture"]) = [] := by decide

/-- "Starting with you, each player votes for death or death." -/
theorem badRepeatedBallotOption :
    Ballot.check [] (.byLabel ["death", "death"]) = [.ballotLabelsOk] := by decide

/-- "Starting with you, each player votes for death." -/
theorem badSingletonBallot : Ballot.check [] (.byLabel ["death"]) = [.ballotLabelsOk] := by decide

/-- "Put target creature card ... onto the battlefield transformed." -/
theorem okTransformedArrivalOnField :
    Instruction.check []
      (.move (target (.and [creature, .inZone (graveyardOf .you)])) battlefield
        [.entersTransformed]) = [] := by
  decide

/-- "Return target creature card from your graveyard to your hand transformed." -/
theorem badTransformedArrivalOffField :
    Instruction.check []
      (.move (target (.and [creature, .inZone (graveyardOf .you)])) hand [.entersTransformed])
      = [.ridersFit] := by
  decide

/-- "Reveal the top five cards of your library. An opponent separates those cards into two
piles." -/
def revealAndSplit : List Instruction :=
  [revealCards (topSlice (.lit 5)), .separateIntoPiles anOpponent them 2 []]

/-- "Reveal the top five cards of your library. An opponent separates those cards into two
piles. Put those piles into your hand." -/
theorem okPileWordAfterPartition :
    Card.check (instantSaying (.sequentially (revealAndSplit ++ [move (those .pile) hand])))
      = [] := by
  decide

/-- "Reveal the top five cards of your library. An opponent separates those cards into two
piles. Put those cards into your hand.": each object in a pile is still an individual object
[CR#700.3b], so the card word still reads them. -/
theorem okCardWordReadsPiles :
    Card.check (instantSaying (.sequentially (revealAndSplit ++ [move (those .card) hand])))
      = [] := by
  decide

/-- "Reveal the top five cards of your library. Put those piles into your hand." -/
theorem badPileWordWithoutAPartition :
    Card.check
      (instantSaying (.sequentially [revealCards (topSlice (.lit 5)), move (those .pile) hand]))
      = [.anaphor (.word .pile) .many 0] := by
  decide

/-- "... into two piles. Put one pile into your hand." -/
theorem okOnePileAfterPartition :
    Instruction.check [] (.sequentially (revealAndSplit ++ [move onePile hand])) = [] := by decide

/-- "Put target player into your hand.": a zone holds objects [CR#400.1]; a player is not
one. -/
theorem badPlayerMovedToAZone :
    Instruction.check [] (.move (target .anyPlayer) hand []) = [.movable] := by decide

/-- "Put one pile into your hand." -/
theorem badPilePartitiveWithoutAPartition :
    Instruction.check [] (move onePile hand) = [.anaphor (.word .pile) .many 0] := by decide

/-- "Put each card in the pile of your choice into your hand." -/
theorem okMembershipInAPile :
    Card.check
      (instantSaying
        (.sequentially
          (revealAndSplit ++ [move (allOf (.and [.isCard, .inPile (pileOfChoice .you)])) hand])))
      = [] := by
  decide

/-- "Reveal the top five cards of your library. Put each card in those piles into your hand.":
no effect grouped them into piles [CR#700.3]. -/
theorem badMembershipWithoutAPartition :
    Card.check
      (instantSaying
        (.sequentially
          [ revealCards (topSlice (.lit 5)),
            move (allOf (.and [.isCard, .inPile (those .pile)])) hand ]))
      = [.anaphor (.word .pile) .many 0] := by
  decide

/-- "Turn target creature face down." -/
theorem okStatusOnBattlefieldNoun :
    Instruction.check [] (.setStatus .faceDown (target creature)) = [] := by decide

/-- "An opponent separates all creatures into two piles. Turn each creature in the pile of your
choice face down." Only permanents have status [CR#110.5d]; a pile is not one [CR#700.3b]. -/
theorem okStatusOnPermanentAfterPartition :
    Card.check
      (instantSaying
        (.sequentially
          [ .separateIntoPiles anOpponent (allOf creature) 2 [],
            .setStatus .faceDown (allOf (.and [creature, .inPile (pileOfChoice .you)])) ]))
      = [] := by
  decide

/-- "Separate all creatures into two piles. Turn those piles face down." -/
theorem badPileFaceAsAStatus :
    Card.check
      (instantSaying
        (.sequentially
          [ .separateIntoPiles anOpponent (allOf creature) 2 [],
            .setStatus .faceDown (those .pile) ])) = [.statusHolder] := by
  decide

theorem nestedStaticConditionals :
    StaticSpec.check []
      (.conditionally
        (.conditionally (.keepsUnspentMana .you (.unspent none)) (exists_ .anyPlayer) .ifSo)
        (exists_ .anyPlayer) .ifSo) = [] := by
  decide

theorem nestedTurnPartWindows :
    StaticSpec.check []
      (.onlyDuring .combat none
        (.onlyDuring .mainPhase none (.keepsUnspentMana .you (.unspent none)))) = [] := by
  decide

theorem voteStartingWithSpecifiedPlayer :
    Instruction.check []
      (.vote (some anOpponent) (each .anyPlayer) .openly (.byLabel ["alpha", "beta"])) = [] := by
  decide

/-- "Starting with you, target player votes for alpha or beta.": one player votes, so there is
no order for the vote to proceed in [CR#701.38a] (`voteStartingWithSpecifiedPlayer` is the
same order over every player). -/
theorem badOrderedSingularVoter :
    Instruction.check []
      (.vote (some .you) (target .anyPlayer) .openly (.byLabel ["alpha", "beta"]))
      = [.choiceOrder] := by
  decide

/-- "Each player votes for alpha or beta. You draw a card for each alpha vote. If beta gets more
votes or the vote is tied, exile each permanent with the most votes." -/
theorem okVoteReadsAfterVote :
    Instruction.check []
      (.sequentially
        [ vote (each .anyPlayer) .openly (.byLabel ["alpha", "beta"]),
          draw .you (.votesFor "alpha"),
          if_ (.voteLead "beta" true) (exile (allOf (.and [permanent, .withMostVotes]))) ])
      = [] := by
  decide

/-- "You draw a card for each alpha vote.": no spell or ability instructed the players to vote,
so there are no votes to count [CR#701.38a] (`okVoteReadsAfterVote` is the same read after a
vote). -/
theorem badVotesForWithoutVote :
    Instruction.check [] (draw .you (.votesFor "alpha")) = [.outcomeInScope .voteHeld 0] := by
  decide

/-- "If beta gets more votes, draw a card.": the same missing vote [CR#701.38a]
(`okVoteReadsAfterVote` is the same condition after a vote). -/
theorem badVoteLeadWithoutVote :
    Instruction.check [] (if_ (.voteLead "beta" true) (draw .you (.lit 1)))
      = [.outcomeInScope .voteHeld 0] := by
  decide

/-- "Exile each permanent with the most votes.": the same missing vote [CR#701.38a]
(`okVoteReadsAfterVote` exiles by the same predicate after a vote). -/
theorem badWithMostVotesWithoutVote :
    Instruction.check [] (exile (allOf (.and [permanent, .withMostVotes])))
      = [.outcomeInScope .voteHeld 0] := by
  decide

theorem oneWayResultShift :
    Instruction.check [] (.sequentially [rollDice .you 1 6, .shiftResult (some .up) (.lit 1)])
      = [] := by
  decide

theorem abilityCounterRecipient :
    Instruction.check [] (.putCounters (.lit 1) .own (a (.abilityHead .anyOnStack))) = [] := by
  decide

theorem removeOwnCounterKinds :
    Instruction.check [] (.removeCounters (some (exactly 1)) (some .own) .you) = [] := by decide

theorem namedAdditionalPartAnchor :
    Instruction.check [] (.additionalPart (some .you) .upkeep (some .mainPhase) (.lit 1) none)
      = [] := by
  decide

/-- "When you unlock this door, draw N cards." -/
def unlockDraw (amount : Nat) : Ability :=
  triggered .when (.unlocksDoor .you .thisDoor) (draw .you (.lit amount))

/-- "When you unlock this door, draw a card.": a door header belongs to a Room's shared line. -/
theorem okDoorHeaderOnSharedLine :
    Card.check
      (.sharedLineSplit { types := [.enchantment], subtypes := [enchantmentType "Room"] }
        ⟨"", some [generic 1, pip .red], [unlockDraw 1]⟩
        ⟨"", some [generic 3, pip .red], [unlockDraw 2]⟩) = [] := by
  decide

theorem badDelayedDoorDeixis :
    Card.check (instantSaying (delayed (.unlocksDoor .you .thisDoor) (draw .you (.lit 1))))
      = [.doorFrame] := by
  decide

end Semantics.Proofs.Piles
