# description-2: the player-sorted reads and spellings

Sub-round 2 of [workbench-description-and-player-sorted-reads](workbench-description-and-player-sorted-reads.md)
(the umbrella — authoritative). Runs AFTER sub-round 1. Owns its sections
"'One of your opponents'" (RULED 2026-08-27: single environment — the 80/83
spelling read proceeds on the measured covariance; the three violating lines
recorded as macro-owed, nothing built for them; Frenzied Gorespawn benches
once Menace's data row exists — an open catalog row, cheap) and
"Player-sorted predicates and life reads" (the player-headed comparison —
the player half of `Compare`, same thing the condition frame already says,
no duplicated row; the negated existential over players — distinguishable
from `NotCond` and from a plural read; the point-in-time snapshot rider —
Sengir, closed or recorded at BOTH layers, never a widened present-tense
read; the life exchange — the two-participant clause [CR#701.12c,701.12g],
13 lines), plus routed: the distributive possessive ("each player … their
own"), "their opponents" (`Predicate.Opponent` is of-`You` only — Shared
Fate), and the team form ("your team controls" [CR#102.3] — Pir).

Pins: `groupMention (PlayerGroup _)` stays False; no partitive widening
reintroduced; declined cells carry counts. Standard constraints apply.

## As landed (2026-08-28)

Every count below is this round's own re-measurement over supported cards
(`jq 'select(.supported)'`), with parenthesised reminder text stripped.

### "One of your opponents"

- **Nothing was built and nothing needed to be.** The phrase is the
  ordinary `Indefinite` over the `Opponent` head, and the covariance
  rule is recorded where the spelling side reads it — a docstring on
  **`Macros.anOpponent`**, with a pointer from `Predicate.Opponent`.
- **Re-measured:** 32 supported lines write "one of your opponents" (32
  cards), confirming finding 547's total. The covariance was re-measured
  independently, classifying each occurrence in the attack and
  damage-complement position by the subject of the clause it completes:
  **all 67 you-anchored subjects write "an opponent"**, and of the **27
  free subjects 24 write "one of your opponents"**. The three exceptions
  are exactly Fiendish Duo, Gisela, Blade of Goldnight and Rem Karolus,
  Stalwart Slayer — all replacement statics under a bare "a source" or
  "a spell" — and they are recorded as macro-owed, per the RULED
  paragraph. No determiner row, no second environment, and no partitive
  widening: `groupMention (PlayerGroup _)` is still `False`.
- **Frenzied Gorespawn benches as a FRAGMENT**
  (`frenziedGorespawnMenaceTrigger`), and the umbrella's "one gap from
  whole" is a **wrong premise, corrected**: Menace's `keywordFacts` row
  already existed (no data row was owed), and the card's FIRST line is
  "for each opponent, goad target creature that player controls" —
  `ForEachOf` takes an **object** group, so a loop over a player group
  has no row. The menace trigger itself writes today.
- **A second family named:** 4 supported lines describe a creature by the
  defender it is attacking under this phrase (Martial Impetus, Oviya,
  Scriv, Seifer). That is the ATTACKER's voice of sub-round 1's
  `AttackedBy` and has no predicate.

### The player-headed comparison

- **`Compare` is now kind-INDEXED**, on the ADR's own test rather than by
  minting a marked player twin: its slot moved from `List Characteristic`
  to `List ProjAxis`, which is the axis vocabulary `Superlative` already
  shared between the kinds, and the new `AxesAt k axes` gate scopes the
  list to one kind exactly as `projScope ax = k` scopes `Superlative`'s
  single axis. `AxesAt` has no empty-list constructor, so it replaces the
  old `NonEmpty` gate as well. `comparedTypes`/`sameChars` became
  `axisTypes`/`sameAxes` over axes; 51 call sites moved to `CharAxis`.
  The row is not duplicated: a description takes no subject where
  `CompareAmt` over `PlayerStatOf` names one, so the description frame
  now says at its own frame what the condition frame already said.
- **It is not `CompareOver`**, and the round records why: that row binds
  a member of its domain and reads it back as `They`, which is writable
  only when no other singular player is in scope. Probed and refused in
  exactly the clause the next item needed.
- **Re-measured:** 18 supported lines write "has more life than"; **4**
  put it on a description — Keeper of the Flame, Keeper of the Light,
  Oath of Mages, Namor, Atlantean King.
- **Benched:** `opponentWithMoreLifeThanYou` (both Keepers) and
  `playerWithMoreLifeThanYou` (Namor). No card benches whole, and each
  blocker is named: both Keepers restrict the choice to the moment of
  activation ("as you activate this ability"), for which this vocabulary
  has nothing (**recorded, not chased**); Namor's body wants the
  attacker-voice predicate above; Oath of Mages reads its two players
  back as "the first player"/"the second player".
- **Pinned:** `badMixedAxisComparison` — an axis list crossing an
  object's characteristic with a player's life total describes nothing
  [CR#109.3,119.1].

### The negated existential over players

- **The umbrella's premise is a wrong premise, corrected rather than
  built on.** The shape IS `NotCond`'s negation of a whole condition,
  because the existential is the whole condition: "no opponent has more
  life than that player" denies that any opponent answers a description,
  which is what `Exists` asks, and `NotCond` outside it scopes the
  negation over exactly the quantifier. **No row was minted.**
- What it needed was the player cell of `Compare` above; `CompareOver`
  could not have served, because the clause has a second singular player
  in scope (the attacked one) and the member pronoun resolves to
  neither.
- **Re-measured:** 5 supported lines, 5 cards — Agent of the Shadow
  Thieves, Guild Artisan, Hardy Outlander, Sword Coast Sailor, Veteran
  Soldier.
- **Benched:** `agentOfTheShadowThievesGrantedTrigger` (a FRAGMENT: all
  five grant the ability with "Commander creatures you own have […]",
  and the quoted grant has no vocabulary; the description they head is
  sub-round 1's `commanderCreaturesYouOwn`). The three-way distinction
  the acceptance asks for is measured beside it at the bare context:
  `noOpponentHasMoreLifeThanYou`, `someOpponentLacksMoreLifeThanYou`
  (`Exists (Not …)`) and `yourOpponentsHaveMoreLifeThanYou` (the plural
  player read) are three different conditions.

### The snapshot rider

- **Recorded at BOTH layers with its witnesses, not closed and not
  silently widened.** The record sits on `PlayerStatOf`, which is the
  amount layer, and names the condition layer beside it.
- **Re-measured at 2 lines over 2 cards**, not 1: Sengir, the Dark Baron
  ("that player's life total as the turn began" — the amount layer) and
  **Knights of the Black Rose** ("if you were the monarch as the turn
  began" — the condition layer, a designation `HasDesignation` also
  reads only in the present).
- The rules give the phrase no reading of its own: where they write it,
  it marks a CONTINUITY span ([CR#302.6,508.1a]), never a value read at
  an instant. `Lookback` and `EventSum` scope an EVENT to a window and
  are not this.

### The life exchange

- **Built: `ExchangeLife`,** one Effect row taking ONE party slot gated
  by `twoPartiesOk`. [CR#701.12c] gives the effect exactly two
  participants; the corpus names them two ways, as a coordination
  ("exchange life totals WITH target opponent", whose first party is the
  unwritten "you") and as one mention counted at two ("TWO TARGET
  PLAYERS exchange life totals"), and one slot serves both because the
  row asks for two players and not for a way of naming them. The
  arithmetic was indeed already there — [CR#701.12c] is `LifeOp.Set`
  twice from values read before either is written — and what no row said
  is that ONE clause binds two players symmetrically. It announces both
  life outcomes, which is what Mister Negative's "if you lost life this
  way" reads.
- **Re-measured at 7 lines over 7 cards**, not 13: Axis of Mortality,
  Magus of the Mirror, Mirror Universe, Mister Negative, Profane
  Transfusion, Psychic Transfer, Soul Conduit. (The word "exchange" as a
  whole is 54 lines over 51 cards; the mass of it is the CONTROL
  exchange, which is untouched here.) [CR#701.12g]'s other exchange — a
  life total against a power or toughness — is **3 further lines** (Evra,
  Halcyon Witness; Tree of Perdition; Tree of Redemption) and is a
  different pair of values.
- **Benched: Mirror Universe, WHOLE CARD** (the coordination spelling),
  and `soulConduitExchange` (the counted spelling). Magus of the Mirror
  prints Mirror Universe's ability on a creature.
- **Pinned:** `badExchangeOneParty` and `badExchangePluralParty`
  [CR#701.12a,701.12c].
- **The two "difference between" lines still do not write, and the
  blockers are named:** Psychic Transfer is refused by `readAmount
  (DifferenceBetween _ _) = False` — an arithmetic subject on a
  comparison's left, a separate question — and Profane Transfusion writes
  "the difference between those players' life totals", one plural
  possessive naming both values, which `DifferenceBetween`'s two-amount
  slot cannot say. Neither is unblocked by the exchange.

### Routed items

- **The distributive possessive** — **no row needed**, and the probe was
  what showed it. Sub-round 1's `agentIntro` lift binds one member of an
  `Each` agent in place of the group mention, so "their" inside the
  clause is the ordinary `They` reading that member, and the coordinated
  mass object is the one `weftwalkingShuffle` already wrote. **38
  supported lines** write a distributive possessive under "each player",
  29 of them the shuffle family. Benched as
  `eachPlayerShufflesTheirHandAndGraveyard`; the "your" spelling is
  unchanged.
- **"Their opponents"** — **recorded, not widened.** `Predicate.Opponent`
  stays of-`You`, and its docstring now says what is and is not deferred:
  [CR#102.3] already gives the word its team DENOTATION, so nothing about
  teams is missing there; what has no slot is the POSSESSOR, the player
  an earlier clause bound. **5 supported lines** (Shared Fate,
  Antagonism, Bend or Break, Necrotic Plague, Portal Manipulator).
- **The team form — decided and built.** "Your team" is a player-group
  VALUE (`PlayerGroupWord.YourTeam`), not a spelling of `You`, and
  [CR#102.4] is the deciding text: it makes the term shorthand for "you
  and/or your teammates", which is more than one player wherever the game
  has teams [CR#102.3], and collapses it to "you" only in a game that is
  not between teams — so a spelling note on `You` would say the wrong
  thing in exactly the games the word exists for. **15 supported lines**
  write it. Benched as `pirDistributive`.

## Remainders

- **Pir, Imaginative Rascal does not bench whole** — it also prints
  "Partner with Toothy, Imaginary Friend", and no keyword row carries a
  partner. Doc Samson's note that Pir waited on the team form ALONE is
  corrected in source.
- **The attacker-voice predicate** ("creatures attacking that player",
  "each creature that's attacking one of your opponents") — 4 measured
  lines plus Namor's body. Sub-round 1 built the defender's voice
  (`AttackedBy`); this is the other one, and it is owed.
- **The as-you-activate targeting restriction** — both Keepers. Recorded,
  not chased.
- **Oath of Mages** — reads its two players back as "the first player"
  and "the second player"; an ordinal player read with no row.
- **A loop over a PLAYER group** — `ForEachOf` takes an object group, so
  "for each opponent, goad target creature that player controls"
  (Frenzied Gorespawn's first line) has no row.
- **The granted QUOTED ability** — the five negated-existential carriers
  and the whole Commander-grant family wait on it, unchanged from
  sub-round 1.
- **`readAmount (DifferenceBetween _ _) = False`** — Psychic Transfer's
  blocker; whether a computed amount may stand as a comparison's subject
  is its own question.
- **The plural-possessive difference** — "the difference between those
  players' life totals" (Profane Transfusion).
- **The CONTROL exchange** — 36 lines over 33 cards write "exchange
  control"; untouched, and not this round's.
- **The [CR#701.12g] numeric-value exchange** — 3 lines (Evra, Tree of
  Perdition, Tree of Redemption).
- **The snapshot rider** — recorded at both layers, not built; Sengir and
  Knights of the Black Rose are its witnesses.
