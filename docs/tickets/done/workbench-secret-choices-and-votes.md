# workbench-secret-choices-and-votes

The secret-choice and voting bundle, declined whole by the turn-structure
round (2026-09-02) rather than sliced. Inventory at its close: 17 "secretly"
lines (Menacing Ogre's secret number the marquee) + 39 vote lines. The
choice machinery (`EntersChoice`/`ChoiceDomain`/`NumberBetween`) covers the
value side; what the bundle owes is the SIMULTANEOUS/HIDDEN protocol and the
vote tally reads. Named blockers recorded in
done/workbench-turn-structure-and-procedures.md. Re-measure at claim.

## As landed (round of 2026-09-02)

Every count below was re-measured this round over `jq 'select(.supported)'`.

### The supported-vote collapse check: it does NOT collapse

40 supported cards write a vote against 3 unsupported ones (a Plane, an
Alchemy split card, a land). The Conspiracy-set worry was misplaced: the
*sets* are unsupported only in their conspiracy-typed cards, and the vote
cards printed alongside them (Council's Judgment, Custodi Squire,
Expropriate, Capital Punishment, …) are vintage-playable. 33 of the 40 cast
a vote — 27 openly, 6 secretly — and the other 7 are the extra-vote statics
(4), the finish-voting triggers (3) and Illusion of Choice. 17 supported
cards write "secretly", 11 at a chooser and 6 at a vote; the two families
overlap in the six "secret council" cards.

### The protocol

ONE marking, `Disclosure = Openly | Secretly`, shared by the chooser and the
vote. [CR#101.4] runs simultaneous decisions in APNAP order and [CR#101.4b]
states the default a printed "secretly" turns off ("a player knows the
choices made by the previous players"); Wheel of Misfortune's ruling states
what replaces it. [CR#101.4a]'s face-down card is explicitly NOT the marking.

The reveal splits by host on the corpus's own evidence: the CHOOSER's reveal
is a separate sentence in 4 of 6 cards, so it is a row (`ChoicesRevealed`);
the VOTE's reveal is comma-joined inside the vote sentence in 6 of 6, so it
is spelled off the vote's own `Secretly` and is no second row.

`Vote` is NOT a `Choose` and the two may never collapse — [CR#701.38c] says
so outright, and Prisoner's Dilemma is the card that proves the line earns
its keep (a ballot's exact shape written in the chooser's words).

### Rows minted

- **`Disclosure`** on `Effect.Choose` and on `StaticEffect.EntersChoice` —
  a slot, not a second row.
- **`Effect.ChoicesRevealed`** (`HiddenSort` = numbers / choices).
- **`Effect.Vote`** + **`Ballot`** (`ByLabel` 26 cards, `ByCandidate` 6).
  The STARTING PLAYER IS DERIVED from the disclosure: all 27 open votes
  write "Starting with you," and none of the 6 secret ones writes one,
  because [CR#701.38a]'s specified player is what a simultaneous ballot has
  no use for.
- **`Condition.VoteLead`** — "if [l] gets more votes[ or the vote is
  tied]", 14 cards (13 with the tie). The tie is a SLOT: it covaries with
  the arm's position, not the row.
- **`Amount.VotesFor`** — "for each [l] vote" / "the number of [l] votes",
  12 cards.
- **`Predicate.WithMostVotes`** — "with the most votes or tied for most
  votes", 4 cards, kind-indexed. NOT `Superlative`: a vote count is no
  `ProjAxis`, and widening `ProjAxis` would need a kind-polymorphic axis.
- **`Predicate.ChoseExtreme`** — "with the highest number", introducing
  `NamedNumber` so `ThatMuch` reads it [CR#608.2c].
- **`Exposed.ExposedChoice`** — "Reveal the [q] you chose", 5 lines.
- **`StaticEffect.MayVoteAdditional`** + **`StaticKind.VoteAllowance`** —
  [CR#701.38d], the land and block allowances' third sibling, window
  derived. 4 cards.

### Pins

`badRepeatedBallotOption` ([CR#701.38b]: a ballot's words are "each
connected to a different effect") and `badSingletonBallot` ([CR#701.38a]:
"one choice from a list of options"). Both are rules-impossibility, not
counts.

### Benched whole (9)

MENACING OGRE (the marquee — the secret choose, the reveal, the extremal
read and the number it names), TYRANT'S CHOICE, COERCIVE PORTAL, PLEA FOR
POWER, COUNCIL'S JUDGMENT, CUSTODI SQUIRE, ORCHARD ELEMENTAL, LIEUTENANTS OF
THE GUARD, BALLOT BROKER.

Fragments: Truth or Consequences' first two sentences (the secret vote),
Emissary of Grudges' entry line and its reveal-cost ability.

Menacing Ogre's one unreproduced surface is named in its bench docstring:
the fourth sentence spells its predicate anaphorically ("one of those
players") where the bench writes it out. Exactly one supported card writes
the membership condition, so no witness pays for a cell.

### Not taken

The pile-partition, draft, bidding and end-the-turn bundles were not opened.

### Remainders

The full residue list with counts and each blocker named is in
`idris/src/Experimental/Cards.idr` under "The secret-choice and voting
bundle's residues": the finish-voting trigger and agreement reads (3), "the
voter" (2), votes received by a player (2), the per-candidate count (1),
vote control (1), the colour ballot (1), the optional ballot (1), "you get
an additional vote" (1), the chooser that binds its own noun (2), the
printed value list at a chooser (2), the label chooser (1), Wheel of
Misfortune (1), The Toymaker's Trap (1), the membership condition (1), the
reveal-cost pair's other payloads (2), and the 18 open votes blocked on
their own consequents.

Gate: `idris/scripts/build` PASS, 23/23, 0 errors, 0 warnings. Citations:
`cite check --list-noncompliant` empty, `cite check` 0 stale ([CR#101.4a] and
[CR#701.38c] newly blessed), `cite audit --diff` 60 sites read.
