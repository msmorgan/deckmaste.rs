---
needs: []
---
# The randomness family's event side: flip and roll as things that happen

`docs/tickets/done/workbench-randomness-vocabulary.md` wrote the random ACTS —
the coin flip, the die roll, the results table, the at-random selection mode —
and drew a hard boundary at the event side, ledgering seven items. They are one
region (an `EventName`/`GameEvent` row per random act, plus the reads over
several outcomes) and nothing owns them.

## The event rows

From that round's ledger, each with its own line there:

- **Chance Encounter** — "the event-side reader: an `EventName` and `GameEvent`
  row for winning/losing a coin flip (8 lines), which the ticket made optional."
- **"Whenever you roll one or more dice"** — "the same event-side axis, roll
  side."
- **Karplusan Minotaur** — its cost-side blocker was removed (`costActionOk
  (FlipCoins who _) = costNounOk who`), so "Cumulative upkeep—Flip a coin"
  composes as a cost; what is left is "the coin-flip trigger event (above) and
  cumulative upkeep". The cumulative-upkeep half is
  [workbench-cost-and-payment-residues](workbench-cost-and-payment-residues.md)'s;
  the event is this ticket's.

## The reads over more than one outcome

- **"Flip five coins. … for each coin that comes up heads"** — "a count over
  flips — the `EventCount` axis, not this round's."
- **"roll two six-sided dice … If you rolled 7"** — writable as a roll; the
  total is not. "`RollResult` mints one singular result, per [CR#706.2]'s
  per-roll definition", so summing several rolls is a read the vocabulary does
  not have.

## Deliberately out, and to be re-decided rather than re-discovered

- **Roll modifiers and ignored rolls** — "roll that many dice plus one", "ignore
  the lowest roll"; [CR#706.2b,706.6]. Marked "deliberately out" by that round.
- **The planar die** — 6 lines, one of them the imperative "{T}: Roll the planar
  die." (Fractured Powerstone). "the die is not an N-sided die [CR#706.1a] and
  has its own faces — a Planechase axis, not this round's." Note the closure
  round's command-zone type widening has already landed the `Plane` type, so the
  Planechase argument is weaker than it was; re-read it.

Each of these last two is fine to close as "still out" — but with the rule
written down, not by silence.

## Not this ticket's

Execution semantics. That round landed none: "Nothing resolves a flip, chooses a
number, or models a randomness source; no decider, no visibility, no value kind
was imported." Keep it that way — this is vocabulary for what a text may *watch*
and *read*, not a randomness engine.

## Consumption boundary

`idris/src/Experimental/Events.idr` (`EventName`, `GameEvent`, `eventUse`, the
complement tables, `EventCount`), `idris/src/Experimental.idr` (`FlipCoins`,
`RollDice`, `TheResult`, `countOutcomes`/`countQuantOutcomes` and the trigger
header's readers), the pin modules `idris/src/Experimental/Proofs*.idr`, and the
evidence bench `idris/src/Experimental/Cards.idr`. No Rust crate.

## Acceptance

- Chance Encounter and Karplusan Minotaur's trigger each bench, or reduce to a
  named blocker outside the event rows.
- The multi-outcome reads land or are named at their measured size.
- The two "deliberately out" families end with a written rule-backed verdict.
- `idris/scripts/build` PASS, no witness lost, no pin silently passing.

Standard constraints apply.
