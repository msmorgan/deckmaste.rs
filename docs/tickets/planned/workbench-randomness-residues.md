---
needs: []
---
# Randomness residues: roll replacements, dice follow-up reads, the planar die

Routed from `workbench-randomness-event-side` (close, 2026-08-26), which
landed the flip/roll event rows (`FlipEvent`, `RollsDice`), `TheTotal`,
`CoinsShowing`, and the two rule-backed verdicts. Six measured remainders,
one region:

1. **Roll modifiers, ignored rolls, rerolls** (4/8/2 lines) — the whole
   surface is REPLACEMENT-side ("If you would roll…, instead roll that many
   plus one and ignore the lowest roll"): needs an ignore instruction over
   rolls the same clause made and a superlative selection among them
   [CR#706.2,706.6]. Never pinnable — both rules make the family meaningful.
2. **The planar die** (6 lines) — instruction row, face vocabulary, special
   action [CR#901.3a,116.2i]. Its roll EVENT is already covered by
   `RollsDice` per [CR#706.7]; numerical reads are inapplicable to it by
   that rule's own second sentence.
3. **Result-conditioned roll headers** — "Whenever you roll a 4 or higher"
   (4 supported, 13 all).
4. **Noted/stored results** (3 supported, 5 all) [CR#706.8a..706.8c].
5. **Reads over a roll set** — "If any of those results was 10 or higher"
   (1), "rolled doubles" (1) [CR#706.5].
6. **The uncalled face read distributed over players** — "each player whose
   coin comes up tails" (3 lines) — `CoinsShowing`'s per-player twin.

## Consumption boundary

`idris/src/Experimental/Events.idr`, `Triggers.idr`, `Phrase.idr`,
`Effect.idr` as each item's shape demands; `Cards.idr` bench; `Proofs*.idr`.
No Rust crate. Vocabulary only — no execution semantics, no randomness
engine (the parent round's fence carries over).

## Acceptance

- Each numbered item lands or ends in a written rule-backed verdict; no
  silent drops.
- `idris/scripts/build` PASS, no witness lost, no pin silently passing.

Standard constraints apply.
