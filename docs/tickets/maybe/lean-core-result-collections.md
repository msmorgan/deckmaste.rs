---
needs: []
---
# Consolidate roll, coin-flip, vote, and choice result readbacks

`Semantics/Abilities.lean` currently scatters single-mechanic result and tally
readbacks across `Predicate`, `Condition`, `Amount`, and `Action`:

- **`Predicate` (`Abilities.lean:240-241, 259`):**
  - `Predicate.withMostVotes` (`"each permanent with the most votes"`,
    *Council's Judgment* / *Custodi Squire* in `Cards/Piles.lean:71,130`)
  - `Predicate.choseExtreme (op : AggregateOp)` (`"each player who secretly chose
    the highest number"`, *Menacing Ogre* in `Cards/Counters.lean:1135-1136`)
  - `Predicate.coinCameUp (face : CoinFace)` (`"each player whose coin came up
    tails"` / `"each creature whose coin came up tails"`, *Goblin Assassin* and
    *Rakdos, the Showstopper* in `Cards/Faces.lean:134,144`)
- **`Condition` (`Abilities.lean:429-431`):**
  - `Condition.voteLead (label : VoteLabel) (orTied : Bool)` (`"if death gets
    more votes"` / `"if torture gets more votes or the vote is tied"`,
    *Capital Punishment* / *Plea for Power* in `Cards/Piles.lean:55,57,97,98`)
  - `Condition.anyResultIs (comparator : Comparator) (bound : Amount)` (`"if any
    of those results is 10 or more"`, *Farideh, Devil's Chosen* in
    `Cards/Trigger.lean:484`)
  - `Condition.rolledDoubles`
- **`Amount` & `Action` (`Abilities.lean:363, 365-366, 1096, 1101-1102`):**
  - `Amount.votesFor (label : VoteLabel)`
  - `Amount.coinsShowing (face : CoinFace)` (*Ral Zarek*, `Cards/Faces.lean:121`)
  - `Amount.greatestStoredMatch (subject : NounPhrase)`, `Action.storeResults (on : NounPhrase)`,
    and `Action.rerollStored (quantity : Quantity) (whose : NounPhrase)`
    (*Centaur of Attention*, `Cards/Turn.lean:360-364`, `[CR#706.8a..706.8c]`)
  - `Action.applyResultsTable (rows : List RollRow)`

## Design Candidates

- **`voteLead` and `withMostVotes` (`[CR#701.38a]`):** Express `voteLead` and
  `withMostVotes` via `votesFor` / `superlative` / `compareOver` instead of
  bespoke `Condition` and `Predicate` constructors (preserving strict `>` vs
  tied `≥` modes).
- **`choseExtreme`:** Express `"each player who chose the highest number"` via
  `superlative` / `compareOver` over `chosenNumber` rather than a dedicated
  `Predicate.choseExtreme`.
- **Die-roll and coin-flip result queries (`[CR#705.2,706.2]`):** Unify
  `anyResultIs`, `rolledDoubles`, `coinsShowing`, `coinCameUp`, and
  `applyResultsTable` over a small typed result/outcome read rather than five
  unrelated constructors, while keeping `ignoreOutcomes` (`[CR#706.6]`) and
  `shiftResult` (`[CR#706.2]`) as semantic operations on rolls.
