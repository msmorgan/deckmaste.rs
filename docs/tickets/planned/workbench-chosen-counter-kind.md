---
needs: []
---
# The chosen counter kind — a closed menu over kinds

Routed from `workbench-counter-kind-holder-source` (close, 2026-08-26).
Measured there: 21 distinct supported lines over 33 cards write a counter
whose KIND is chosen from a printed closed menu — 10 entry-side ("enters
with your choice of a +1/+1, first strike, or vigilance counter on it" —
riot's reminder text, the two-kind creature run, Denry Klin) and the rest
plain puts. Two menu spellings ("your choice of a X, a Y, or a Z counter";
Aragorn's "from among X, Y, Z, and W"), plus Bribe Taker's variant whose
second arm is a kind read off the board.

Neither existing seat reaches it: `EntersWithCounters` takes one
`CounterKind`; `EntersChoice`/`ChoiceDomain` offer only open-ended domains
(`QualitySort` has no counter-kind member), where these lines write a
closed menu. The shape question is whether the menu is a `ChoiceDomain`
extension (a counter-kind quality sort with a list domain) or a dedicated
menu slot on the counter rows — weigh against the chooser-position doctrine
in `workbench-choice-chosen-and-ascription` (which owns chooser POSITIONS;
this ticket owns only the kind menu).

This is the only remaining blocker for Denry Klin whole
(`denryKlinSameKinds` benches its trigger already).

## Consumption boundary

`idris/src/Experimental/Words.idr` (if the menu needs vocabulary),
`Phrase.idr`/`Effect.idr` (the seat), `Cards.idr` bench, `Proofs*.idr`.
No Rust crate.

## Acceptance

- Denry Klin benches whole; the menu is closed by design (a written list),
  not per-card.
- `idris/scripts/build` PASS, no witness lost, no pin silently passing.

Standard constraints apply.
