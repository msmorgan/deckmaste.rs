---
needs: []
---
**Gate the vote family on a vote in scope and give `Effect.Vote` its ordering
slot.** Cleanroom review 3, 2026-09-04, findings U2 and U9.

- **U2 — no scope obligation on the reads.** `Phrase.Amount.VotesFor`,
  `Phrase.Condition.VoteLead`, `Phrase.Predicate.WithMostVotes` and
  `Phrase.Predicate.ChoseExtreme` carry no obligation, and
  `Effect.instrIntro (Vote _ _ _ _) = bs` introduces nothing, so
  `VoteLead "yes" False : Condition []` and `VotesFor "yes" : Amount []` are
  admitted with no vote in the text. A vote exists only where a spell or
  ability instructs players to vote [CR#701.38a]. `Vote` introduces an
  outcome binding (a non-quantity `OutcomeSort`, `VoteHeld`) and the four
  reads take `{auto 0 ok : countOutcomes VoteHeld bs = 1}`, the shape
  `Phrase.CoinsShowing`/`FlipCalled` already use over `coinFlipInScope`.
- **The ballot label.** Decide whether `VotesFor`'s label is checked against
  the ballot (carry `ByLabel opts` in the binding payload) or left open, and
  record the decision in the landing.
- **U9 — `Vote` lacks `Choose`'s ordering obligation.**
  `Effect.Vote : (first : Maybe (Noun bs Player)) -> …` has no
  `{auto 0 od : ChoiceOrder first by}`, so `Vote (Just You) You Openly
  (ByLabel ["Aid","Ruin"])` is admitted where the `Effect.Choose` twin is
  refused. Add the obligation.
- **The printed "starting with you".** `Macros.vote` hard-codes
  `first = Nothing`, so the seven will-of-the-council witnesses in
  `Cards/Piles.idr` drop a clause the cards print; to vote, each player
  chooses starting with a specified player and proceeding in turn order
  [CR#701.38a]. Add `voteStartingWith` and re-spell the seven as printed.

Size: S. Done when: no vote read typechecks without a vote in scope and each
refusal is pinned with a same-module twin; `Vote` refuses a bad order; the
seven council witnesses read "starting with you"; build at its module count.
Standard constraints apply, including the RON-shaped constraint.

## As landed

- **U2, the scope obligation.** `Words.OutcomeSort` gains `VoteHeld`
  (non-quantity, `outcomeSortIx` 15) and `Effect.instrProfile (Vote …)` now
  publishes it as its deed (`sameIntro bs [outcomeB VoteHeld]`), so a vote
  stands in the bindings exactly where one was held. `Phrase.Amount.VotesFor`,
  `Phrase.Condition.VoteLead` and `Phrase.Predicate.WithMostVotes` each take
  `{auto 0 vt : countOutcomes VoteHeld bs = 1}`, the shape `CoinsShowing` /
  `FlipCalled` use over `coinFlipInScope`. `VoteLead "yes" False : Condition []`
  and `VotesFor "yes" : Amount []` are now refused with
  `Can't find an implementation for countOutcomes VoteHeld [] = 1.`
- **`Phrase.Predicate.ChoseExtreme` is NOT gated — see the STOP below.** Its
  only witness is Menacing Ogre ("each player secretly chooses a number …
  each player who chose the highest number"), which holds no vote
  [CR#701.38c]; the obligation the ticket prescribes would refuse a printed
  card. Left as it was, with the correct obligation recorded for a follow-up.
- **The ballot label: left open.** `VotesFor`'s label is not checked against
  the ballot. Reasons in the landing record.
- **U9, the ordering obligation.** `Effect.Vote` takes
  `{auto 0 od : So (choiceOrderOk first (Just voters))}` — the same gate
  `Effect.Choose` carries. `Phrase.choiceOrderOk` is generalised to two
  binding contexts (`{bs}` for `first`, `{cs}` for the voters, which live
  under `first`); its `Choose` use is unchanged.
  `Vote (Just You) (target AnyPlayer) …` is now refused.
- **The printed "starting with you".** New `Macros.voteStartingWith first
  voters disc ballot` over the same row; the seven witnesses in
  `Cards/Piles.idr` that print "Starting with you" (Tyrant's Choice,
  Council's Judgment, Orchard Elemental, Plea for Power, Coercive Portal,
  Custodi Squire, Lieutenants of the Guard) are re-spelled through it.
  Truth or Consequences prints no starting player ("Secret council") and
  keeps `Macros.vote`.
- **Pins** (all in `ProofsPiles`, each beside its twin):
  `badOrderedSingularVoter` (twin `voteStartingWithSpecifiedPlayer`),
  `badVotesForWithoutVote`, `badVoteLeadWithoutVote`,
  `badWithMostVotesWithoutVote` (twin `okVoteReadsAfterVote`, added).

## Landing record

Measured on change `uxrtouwv` (working copy), parent
`zqpkpsqv 31a70bb1` (`workbench-vote-scope-and-order` claim).

Numbers before/after (`idris/src/Experimental/`):

| | before | after |
| --- | --- | --- |
| `OutcomeSort` constructors | 15 | 16 |
| vote reads with a scope obligation | 0 / 4 | 3 / 4 |
| `Effect.Vote` obligations | 0 | 1 |
| `Cards/*.idr` vote sites forced to `first = Nothing` | 8 | 1 |
| `Unspellable` pins, `ProofsPiles.idr` | 9 | 13 |
| `Unspellable` pins, all `Proofs*.idr` | 610 | 614 |
| modules built | 46 | 46 |

Gate lines:

- `cd idris && rm -rf build && ./scripts/build` -> exit 0, last line
  `46/46: Building Cards (src/Cards.idr)`, no `Error` and no `Warning` line;
  1m41.7s clean.
- `cargo xtask cite check --list-noncompliant` -> `0 non-compliant
  citation-looking string(s)`.
- `cargo xtask cite check` -> `checked 14388 citations against cr.txt
  (eff. 2026-08-07); 0 stale`.
- `jj --no-pager diff --git | cargo xtask cite audit --diff` -> `audited 11
  citation site(s) — read each rule text against its claim`. Six are the
  code's, all [CR#701.38a], and each claim is that rule's own content (a vote
  exists only where a spell or ability instructs players to vote; it proceeds
  from a specified player in turn order); the other five are this record's
  ([CR#701.38b] on ballots whose listed choices are objects, [CR#701.38c] on
  "voting" meaning an actual vote).
- `cargo xtask cite bless` registered [CR#701.38c] (newly cited by this
  record) and pruned one entry no included path cites any more (the
  manifest rule, cited only from an excluded path); the pruned file
  is kept.

Non-vacuity probe (each new pin deliberately mis-stated once, obligation
satisfiable, in a scratch module):

- `misVotesFor Refl is not a valid impossible case.`
- `misVoteLead Refl is not a valid impossible case.`
- `misWithMostVotes Refl is not a valid impossible case.`
- `misOrder Oh is not a valid impossible case.`

Assurance counts: restored 0; re-spelled 0; ignored 0; added 5 (4 pins +
`okVoteReadsAfterVote`); removed 0.

The ballot-label decision (open): `VotesFor`'s label is NOT checked against
the ballot. `Payload Outcome` is `OutcomeP (sort : OutcomeSort)` and carries
no room for the ballot's options; giving it one means a second
`Payload Outcome` axis, which `countOutcomes` does not see — i.e. it would
break the very `countOutcomes VoteHeld bs = 1` gate this ticket prescribes for
the other reads, and would need reworking `ProofsAnaphora.countOutcomesIsFold`
and `quantOutcome` with it. [CR#701.38b] also lets the listed choices be
objects rather than words (the `ByCandidate` ballot of Council's Judgment and
Custodi Squire), so a label check is not one predicate but a
ballot-kind-sensitive pair. That is an M, not this S. Recorded as a
follow-up: `VotesFor "sprout"` after `ByLabel ["time","knowledge"]`, and
`VotesFor` of any label after a `ByCandidate` ballot, are still admitted.

Deviations and additions:

- Added `Phrase.choiceOrderOk`'s second binding parameter (`{cs : Bindings}`).
  `Effect.Vote`'s voters live at `Triggers.agentIntro first`, not at `bs`, so
  the existing single-context signature could not be applied to them. The
  `Choose` call site is unchanged (`cs` unifies with `bs`).
- `Macros.vote` keeps its arity and its `first = Nothing`, and
  `Macros.voteStartingWith` is a second macro over the row rather than
  `vote` gaining a positional `first`. Reason: with `first` positional,
  `vote` would be a bare alias of the core row (dead weight the macro
  discipline deletes), and the house already spells this distinction as a
  macro pair — `Macros.choose` (`by = Nothing`) beside `Macros.chooses`
  (`by = Just who`) over the same `Effect.Choose` row.
- `okVoteReadsAfterVote` is a synthetic rules-meaningful sentence, not a
  printed card: it has to exercise all three gated reads at one context, and
  no printed card does.

STOP: **the ticket's fourth read is not a vote read.**
`Phrase.Predicate.ChoseExtreme`'s only use is Menacing Ogre
(`Cards/Counters.idr`), whose printed text is "each player secretly chooses a
number. Then those numbers are revealed. Each player who chose the highest
number loses that much life" — no vote anywhere. [CR#701.38c]: a reference to
voting "refers only to an actual vote, not to any spell or ability that
involves the players making choices or decisions without using the word
'vote.'" Measured at Menacing Ogre's reading context (scratch probe):
`countOutcomes VoteHeld = 0`, so the obligation the ticket prescribes for the
four reads would refuse a printed VINTAGE-legal card in the bench.
Resolution: the three genuine vote reads are gated as the ticket asks;
`ChoseExtreme` is left exactly as it was rather than given a gate nobody has
ruled on. Its real gap (no obligation at all) stands and wants its own
ticket. The obligation it needs is "several players chose a number", for
which the machinery exists but the predicate does not: at the same context
`countManys (Quality Number) = 1` while `countChoice (QSort Number) = 0`
(the choice is pluralised, so the existing singular `countChoice` gate that
`Phrase.ChosenNumber` uses does not apply) — so the shape would be
`{auto 0 nc : So (not (countManys (Quality Number) bs == Z))}`, with the
twin/pin pair in `ProofsChoice` beside `badOrderedSingularChooser`.
