---
needs: []
---
**Engine: `Stat::Loyalty` and `Stat::Defense` read counters on the battlefield and
the printed number elsewhere, and the two entry grants read the printed number
explicitly.** Latent today (no implemented card reads either stat on the
battlefield; no battle is implemented), but it turns major as soon as Nahiri-style
"mana value less than its loyalty", Compel Brutality or any battle lands.

Provenance: `docs/evidence/mtg-rules-review-2026-10-21/departures-engine.md` #33
and the Rust half of `departures-lean.md` #20. The Lean half (Nahiri at
`lean/Semantics/Cards/Mana.lean:666-667` and the [CR#306.5b] row at
`lean/Semantics/Proofs/Rules.lean:68` share one `statOf (.stat .loyalty)` term)
belongs with the Lean tickets; `lean-defense-projection` covers the Lean
[CR#310.4b] conferral.

## What is wrong at head

`Stat::Loyalty` is "the PRINTED loyalty characteristic off the card face — never
the live counter count" in every zone; `Stat::Defense` is the `DefenseCounter`
count in every zone. Sites (crates/deckmaste_engine/src):

- `layer.rs:1265-1285` (`eval_stat_of`) and `:915-930` (second arm); the
  `layer.rs:1233` doc "mana value / loyalty / defense are layer-stable base
  state";
- `resolve/count.rs:194-210` (live arm) and `:64-67` (snapshot arm);
- `trigger.rs:1609-1622` (snapshot arm); `activate.rs:504-520`;
  `target.rs:843-858`;
- `plugins/builtin/macros/counters/LoyaltyCounter.ron:1-2` says the opposite ("a
  planeswalker's loyalty IS its number of loyalty counters — read by
  StatOf(_, Loyalty)").

The test `stat_of_loyalty_reads_printed_loyalty` (`resolve/count.rs:1614-1652`)
asserts a battlefield planeswalker printed 4 with one loyalty counter has
`StatOf(Loyalty) = 4`. It asserts the wrong rule and is rewritten (expects 1).

[CR#306.5c]: "The loyalty of a planeswalker on the battlefield is equal to the
number of loyalty counters on it." [CR#306.5a]: the printed number elsewhere.
[CR#310.4c] and [CR#310.4a]: the same for a battle's defense.

## Change

1. Make both stats zone-dependent in every arm above: on the battlefield, the
   counter count (planeswalkers for loyalty, battles for defense); elsewhere, the
   printed number. A snapshot reads the value as it was in the zone it left.
2. The [CR#306.5b] entry grant (`plugins/builtin/rules/grant/planeswalker-loyalty.ron:9`,
   `PutCounters(This, LoyaltyCounter, StatOf(This, Loyalty))`) gets an explicit
   printed-loyalty read, since after step 1 the live read on the battlefield is
   the counter count.
3. Add the missing [CR#310.4b] battle entry grant ("enters with a number of
   defense counters on it equal to its printed defense number") beside it, with a
   printed-defense read. Without it a battle would enter with 0 counters and
   `rules/sba/battle-defense-zero.ron` would put it into the graveyard.
4. Fix the LoyaltyCounter.ron comment and the `layer.rs:1233` doc.

## Tests

- A planeswalker printed 4 with one loyalty counter: `StatOf(Loyalty)` = 1 on the
  battlefield, 4 in hand.
- A planeswalker enters with counters equal to its printed loyalty.
- A battle enters with defense counters equal to its printed defense; off the
  battlefield `StatOf(Defense)` is the printed number.
