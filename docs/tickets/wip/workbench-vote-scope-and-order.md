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
