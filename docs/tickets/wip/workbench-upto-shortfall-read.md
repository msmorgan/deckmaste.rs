# workbench-upto-shortfall-read

Truce / Temporary Truce's tail — "for each card less than two a player draws
this way" reads the SHORTFALL against the ceiling. Choice-B (done 2026-08-27)
verdict: this is not chosen-value vocabulary; it wants an announcement on the
ceilinged amount, i.e. the `UpTo` row announcing what was NOT taken. 2
supported cards. Re-verify with `jq 'select(.supported)'` at claim.

## As landed

Corpus authority: `data/derived/cards.jsonl` filtered
`jq 'select(.supported)'`, re-measured 2026-09-02. CR text via
`data/rules/`.

### The count holds at 2, and they are the same two

Re-measured: **2** supported cards write the shortfall read, Truce and
Temporary Truce, and they write it identically — "Each player may draw up
to two cards. For each card less than two a player draws this way, that
player gains 2 life." Two other supported lines match a loose "for each …
less than" and neither is a shortfall: both compare a standing value
against a threshold (Anya's half-starting-life opponents, Wasp's
power-less-than-0 creatures).

### The premise held: the ceiling announces, and the read is its own row

- `OutcomeSort.CeilingShortfall`. [CR#608.2d] has the acting player
  announce an effect's own choices while applying the effect, so the
  ceiling's number is fixed there and the difference from the printed
  bound is fixed at the same moment. Its own sort beside `RepeatCount`,
  the other announced count: that one is how many times the TEXT said to
  go round, this is how many of a bound the ANNOUNCER declined, and one
  clause can leave both.
- `Amount.UpTo` now announces it (`amtDelta`/`amtIntro`), unconditionally
  and at every ceiling. No gate was minted for "only a draw announces":
  [CR#608.2d] states the announcement of every ceiling alike, and an
  announcement no clause reads costs nothing — the same tolerance
  `Shuffle`'s unread `actLoci` cell already records.
- `Amount.ShortOfCeiling`, gated `countOutcomes CeilingShortfall bs = 1`.
  Slotless, as the "this way" family is: the unit ("card"), the bound
  ("two"), the actor ("a player") and the verb ("draws") are all the
  announcing clause's own words and the read names none of them again.
  `readAmount` is False — the cards left undrawn are still in a library
  and no game state tells them from the rest, which is `UpTo`'s own
  answer one step on.
- **`outcomeIsQuantity CeilingShortfall = False`, and this was the round's
  one real choice.** True was written first and reverted: "that much"
  names a quantity a clause WROTE, and a ceiling wrote the number the
  player TOOK, not the one they declined. Making the sort quantity-
  carrying would have let `ThatMuch` resolve to the shortfall after a bare
  ceiling — a reading 0 supported lines write, where the nine printed
  "discard up to [n] cards, then draw that many cards" lines all name the
  TAKEN batch, which `GroupSize` already reads
  (`discardUpToTwoThenDrawThatMany`). `ManaAdded`'s precedent: an outcome
  may stand in scope and carry no number for "that much" to name.
- Pin `badShortOfCeilingUnannounced` (ProofsF): "You gain 2 life for each
  card less than two you draw this way" with no ceiling announced.

### Bench — a FRAGMENT, and the whole card is the remainder

`drawUpToTwoThenGainPerShortfall : Effect []` writes Truce's two clauses
with the shortfall read live: `may (Each AnyPlayer)` over
`Draw (Those PlayerW) (UpTo (Lit 2))`, then
`gainsLife (Those PlayerW) (Times 2 ShortOfCeiling)`.

What it cannot spell is the printed **"that player"**. That word asks a
distributive pass's MEMBER to be readable in the sentence AFTER the pass,
and `agentIntro`'s own note settles the other way on purpose — the member
binding belongs to the AGENT SEAT (`Does`, and the macros typed through
it), and what stands for the clauses after a pass is the group mention
"those players". `Draw` takes `nomIntro who`, not `agentIntro`, so
neither sentence of Truce is inside a pass to begin with: what
`Each AnyPlayer` leaves is the group mention alone, which is what the
bench's own `eachPlayerBindsNoSingular` states
(`countOnes Player (nomIntro (Each AnyPlayer)) = 0`), and `That PlayerW`
needs a singular mention to count.

So **Truce and Temporary Truce are not benched whole**, and the blocker is
not this row: it is the distributive member escaping into a following
sentence. That is the same seam workbench-search-locus-tail's item 2
declined ("each player who searched their library this way") from the
description side, and it wants one decision, not two.

### Remainder

- Truce / Temporary Truce whole — blocked on the post-pass singular read
  of a distributive subject ("that player"), above. Not a ceiling gap.
- No second carrier for `ShortOfCeiling` exists to widen it against; the
  row is written at its measured two cards.
