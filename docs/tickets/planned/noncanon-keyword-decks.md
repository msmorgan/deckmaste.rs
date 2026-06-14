---
needs: []
---
Grow the noncanon matchup decks with now-live keywords (fliers/reach, menace,
defender, hexproof) so the 50-game gate regression-tests keyword interactions
systemically rather than per-test. Lives in the noncanon workspace/feature.

## Triage note (batch2 worker, 2026-06-14): mis-tiered — NOT claimed, left in planned/

Surfaced rather than guessed. The decisive blocker is mechanism (point 1); a
premise caveat (point 2) needs end-to-end verification before this is worth
deck work.

1. **Wrong mechanism for a default-line batch claim.** The ticket "lives in the
   noncanon workspace/feature" — the long-lived non-mainline `noncanon` branch /
   `../noncanon` workspace — not a fresh default-line claim that `integrate`s
   into trunk. Growing the noncanon matchup decks and running the 50-game gate
   is noncanon-feature work; it should be picked up inside that feature, not via
   this trunk-integrating batch.
2. **Premise needs end-to-end verification.** Updated from an earlier note that
   said combat keywords were entirely unlive: the keyword macros now DO confer
   the deontic rows — `Flying` → `Cant(Block(by: Not(OneOf([Has(Flying),
   Has(Reach)]))))`, `Menace` → `Cant(Block(count: Less(2)))`, `Defender` →
   `Cant(Attack(by: This))` — and `legal.rs` has `Cant(Block)`/`Cant(Attack)`
   enforcement scaffolding ([CR#702.9b] flying-family evasion, [CR#702.111b]
   menace bound, [CR#702.3b] defender). BUT the keyword macro comments still
   say "Engine block-legality enforcement of evasion Cants is a later combat
   task," so whether keyword → derived `Cant` row → `legal.rs` block/attack
   rejection is wired end-to-end (a real flying creature actually un-blockable
   by a groundling in a played game) is unverified. Confirm that with a focused
   engine test before sinking effort into matchup decks, or the 50-game gate
   exercises nothing.

Route to the noncanon feature; verify end-to-end keyword combat enforcement
first.
