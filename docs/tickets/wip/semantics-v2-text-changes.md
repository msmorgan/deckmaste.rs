---
needs: []
---
**Text-changing effects as specified rewrites over bracketed text.** Ruling
2026-10-09 ("A text-changing effect is a specified term rewrite over
bracket markers written on the card", `docs/decisions/semantics-v2.md` §7).
Bodies cleave, overload and splice. Standard constraints apply.

## Shape

- Markers `Predicate.bracketed (p : Predicate)` and `Instruction.bracketed
  (i : Instruction)`; in Rust one generic `Bracketed<T>`. Transparent to the
  checker (it checks the inner term), printed by spelling with its brackets.
  A card is written once, brackets in place.
- `StaticSpec.textChange (subject : NounPhrase) (change : TextChange)`,
  applied at layer 3 [CR#612.1,613.1c]. `TextChange` rows, each a specified
  term rewrite:
  - `removeBracketed`: every `bracketed` term is deleted (cleave
    [CR#702.148a]);
  - `targetToEach`: every `target(…)` determiner becomes `each(…)`
    (overload [CR#702.96a]); `target` is the constructor the rewrite finds;
  - `appendTextOf (source : NounPhrase)`: the source's rules text follows
    the spell's own (splice [CR#702.47a,612.10]);
  - `replaceWord (sort : QualitySort) (from : Option value) (to : Option
    value)`: a color word or a basic land type word replaced
    [CR#612.2] (Magical Hack, Mind Bend, Alter Reality; 13 cards).
- Checker: kinds only; the rewrite's result is not re-checked by the static.
- Spelling learns to print `bracketed` (the English side reads nothing new).

## Known limit

The alternative of storing a card's alternative readings was rejected
because it loses bracket positions. A rewrite must therefore be defined on
the term, and spelling prints the brackets. Record any card whose rewrite
cannot be stated over its terms as a STOP in its file.

## Re-spells (never deleted)

`Exchanged.textBoxes` (`lean/Semantics/Abilities.lean`,
`Check/AbilityRules.lean`, `Check/Abilities.lean`, `Cards/Keyword.lean`,
`Macros/Declared.lean`; Exchange of Words [CR#612.5]) and its probe are
re-spelled against `textChange` where the exchange is a text change,
keeping their asserted outcome.

## Bodies

- cleave: an alternative cost plus `textChange(thisSpell, removeBracketed)`
  under `keywordCostPaid(cleave, this)` [CR#702.148a] (12 cards).
- overload: an alternative cost plus `textChange(thisSpell, targetToEach)`
  [CR#702.96a] (27 cards).
- splice: reveal this card from hand as you cast a [quality] spell,
  `addedCost(Param(1), …)` on that spell, `textChange(thatSpell,
  appendTextOf(this))` [CR#702.47a,612.10] (30 cards).

## Witnesses (canon)

A cleave card with its brackets, an overload card, a splice card, and Mind
Bend (`replaceWord`). Pins: `bracketed` is transparent (a bracketed
mismatched kind is still refused); `removeBracketed` on a card with no
brackets is a no-op that still checks.

## Siblings

None by name. Refresh before integrate; expect conflicts in
`Words`/`Phrase`/`Abilities`.
