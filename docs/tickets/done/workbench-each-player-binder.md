# workbench-each-player-binder

Two binder reads over players, from tails-b's close (2026-09-02):

- **The each-player singular binder** — 35 supported "each player may
  [verb]" lines, mostly member-reading. The one-line fix is `mayCtx` handing
  `agentIntro` (as `Does` took), but it buys no witness until the
  keyword-action macros (`Macros.scry` and kin) are re-seated on a WRITTEN
  agent instead of hardcoding `Does You` — that re-seat is this round's real
  work.
- **The ordinal player read** — 6 supported cards (Cruel Entertainment + the
  five Oaths). Design recorded in done/workbench-element-binder-remainders.md
  (`NthPlayer` over `countOnes Player bs`); all six reach it only through the
  Oath cycle's unbuilt chooser clause, so land the chooser clause with it.

Re-measure at claim.

- **Routed from tails-a (close, 2026-09-02):** the DISTRIBUTIVE PASS's member
  read in the SENTENCE AFTER the pass — Truce/Temporary Truce's "that player"
  (2 cards) and the description-side "each player who searched their/a
  library this way" (9 carriers, 8 also wanting the self-possessive). Both
  wait on ONE decision at this ticket's seam (`agentIntro`'s member scope),
  not two.

## As landed

Corpus authority: `data/derived/cards.jsonl` filtered `jq 'select(.supported)'`,
re-measured 2026-09-02 (reminder text stripped before counting). CR text via
`data/rules/`.

### The counts moved, and the ticket's "35" was short

**79 supported sentences** over **77 cards** write "each player may [verb]" or
"each opponent may [verb]" (57 + 22 occurrences; 55 + 22 cards). **46 of the 79
read the member back** — "their", "they", "that player". The ticket's 35 is not
reproducible by any filter tried; the number to carry forward is 79/46.

Sub-families, re-measured: discard-their-hand-and-draw **6 cards**; search
their library **9 cards**; shuffle hand and graveyard (offered) **2**; scry 1
**3** (Eager Construct, Kaya Intangible Slayer, Myr Custodian).

### 1. The each-player singular binder — BUILT and benched

- `mayCtx (Just d) = agentIntro d`, and `Macros.may` / `mayThen` / `mayElse` /
  `mayThenElse` / `mayWhen` re-typed at `agentIntro decider`. [CR#118.12]
  writes the decider first and [CR#101.4] has each of several players make
  their own choice, so a distributive offer hands its body the one member the
  offer is decided by. Every non-distributive decider is definitionally what
  `nomIntro` gave; the whole tree's other 200+ `may` sites were unaffected.
- **The re-seat.** Four keyword-action macros hardcoded `Does You`
  (`scry`, `surveil`, `scryOne`, `surveilOne`); two of the four already had
  agent-seated twins (`playerScries`, `playerSurveils`), and the missing half
  is now written: `playerScriesOne`, `playerSurveilsOne` (over the new
  `theirTopCard`, `lookedTop`'s twin at the anaphor), plus
  `playerSearchesTheirLibraryFor`. The `You` spellings stay — they are the
  controller's, and `You` binds no mention for `They` to read.
- Pins: `eachPlayerOfferBindsOneMember` (one member) and
  `eachPlayerOfferDropsTheGroup` (no group) beside the unchanged
  `eachPlayerBindsNoSingular` / `eachPlayerBindsAGroup`, which still state what
  `nomIntro (Each …)` mints. The second new pin is what the choice COSTS,
  pinned deliberately.
- **Benched whole:** `eagerConstruct` ("each player may scry 1" — the ticket's
  own witness), `oldGrowthDryads` ("each opponent may search their library for
  a basic land card, put it onto the battlefield tapped, then shuffle"), and
  Shah of Naar Isle re-seated onto the member. Fragments benched:
  `eachPlayerMayShuffleTheirHandAndGraveyard`,
  `eachPlayerMayDiscardTheirHandAndDrawSeven`.

### 2. The ordinal player read — NOT built, and the blockers are now measured

`NthPlayer` stays unbought. All six carriers still reach it only through the
Oath cycle's chooser clause, and that clause has **two** blockers, neither at
this ticket's seam. Both were probed against the compiler, not reasoned:

1. **`CompareOver`'s member is unnameable here.** "That player chooses target
   player who controls more lands than THEY do" sits inside "at the beginning
   of each player's upkeep", whose `possessorB EachPlayers` already binds one
   singular player. The measure is then typed at two singular Player mentions
   and `They` is refused —
   `countOnes Player (bindFor TheD OneOf PhPlayer AnyPlayer :: (predDelta AnyPlayer ++ [MkBinding EachD Player OneOf PlayerP])) = 1`
   has no solution. The bound side is fine (it wants the outer player). Naming
   a member past an outer mention is a recency read, which `ItPrior` refuses on
   purpose, so this is a design question and not a small build. Recorded in
   `CompareOver`'s docstring.
2. **"is their opponent" has no seat.** `Opponent` is of-`You` and carries no
   possessor slot. Re-measured: **10** supported cards want the possessor, not
   5 — the five already recorded plus the five Oaths. Recorded in `Opponent`'s
   docstring. A slot would bump arity across 118 `Opponent` sites in
   `Cards.idr`; a sibling `OpponentOf` row is the cheaper shape, and neither
   was minted because neither alone pays for the Oaths.

Cruel Entertainment additionally needs the player-controls-a-player effect,
unchanged from the earlier record.

### 3. The routed member-scope decision — RE-DECIDED, and it is the member

Written where the old note sits (`agentIntro`'s docstring in `Phrase.idr`).

- The old note's last sentence — "the group mention … stays exactly where it
  was, for the clauses AFTER the pass that read it back as 'those players'" —
  **was never true of the code**: `effIntro (Does s v e) = effIntro e` computes
  in the agent-seated context, so what a pass has always left standing is the
  member row and not the group one.
- The corpus does not want the group there. **0 supported cards read "those
  players" off an agent seat.** All 16 lines writing those words name a group
  some other clause bound: a damage recipient (Skull Rend), a `CopyStack` agent
  (Hive Mind), a targeting clause (Cultural Exchange), a per-member loop's
  pluralized delta (Winds of Abandon) — every one through `nomIntro`, untouched.
- So the member's scope is the pass and the sentences after it, now stated
  rather than left to `effIntro`'s accident. **Truce and Temporary Truce bench
  WHOLE**: `drawUpToTwoThenGainPerShortfall` now writes both printed sentences,
  the gain over `They`.

**The routing claim was wrong: these were two decisions, not one.** The
description-side "each player who searched their/a library this way"
(re-measured: **9 carriers**, 5 of them also writing "each player may search
their library") is untouched. `Each p` elaborates `p` in the OUTER bindings —
it must, since `bindFor` needs `p` to mint the row — so no binding at an agent
seat puts a player inside the description describing them. That gap wants
`CompareOver`'s split shape (domain, then a body typed at the domain's own
member) at the PREDICATE layer. Recorded in `agentIntro`'s note.

### Remainders

- **The ordinal player read (`NthPlayer`) — 6 supported cards**, blocked behind
  (1) `CompareOver`'s member at two mentions and (2) `Opponent`'s missing
  possessor (10 carriers), plus Cruel Entertainment's player-controls-a-player
  effect. Route as a planned ticket; the design stays as recorded in
  done/workbench-element-binder-remainders.md.
- **The self-describing description — 9 carriers** ("each player who searched
  their/a library this way", 8 also wanting the self-possessive). A PREDICATE-
  layer gap, not this seam's. Route as a planned ticket.
- **`Opponent`'s possessor slot — 10 carriers.** Cheapest shape is a sibling
  `OpponentOf` row rather than an arity bump across 118 sites. Travels with the
  ordinal item.
- The five search cards whose tail is the description gap (Boldwyr Heavyweights,
  New Frontiers, Noble Benefactor, Rootweaver Druid, Tempt with Discovery) stay
  fragments; their first sentence writes today.
