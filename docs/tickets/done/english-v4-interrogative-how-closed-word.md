---
needs: []
---
# Interrogative *how* is a Closed Word (R5.2)

From `english-v4-child-name-control-and-category-fission-audit`, Finding G
(the T list). The owner ruled R5.2 in round 9.

## Owner's ruling (round 9, R5.2)

"Approve: fold InterrogativeAdverb into a Closed Word `How` (CGEL Ch. 10 §7,
p. 902)."

## Evidence

The typed class `InterrogativeAdverb` (`english.ron:356`) has one member,
*how*. The load report lists it as a single-member class. Interrogative *how*
belongs to CGEL's closed interrogative class (Ch. 10 §7, p. 902), so
`docs/decisions/english-lexical-analysis.md` §3 requires a direct reference.
Two Productions read it: DegreeInterrogativeDeterminer (`:1349`) and
AdverbialInterrogative (`:1355`). `deckmaste_lexical_source_v4`'s `lib.rs`
inserts the class.

## Change

1. Declare *how* as a Closed Word `How` in the core lexicon, and drop the
   `InterrogativeAdverb` class from the lexicon and from `typed_classes`.
2. The two Productions take `Word(.., "How")` in place of
   `Class(.., "InterrogativeAdverb")`.
3. Readings are unchanged (the same analyses, the same head lexeme), and the
   single-member finding is gone.

## Acceptance (round 9: `docs/decisions/english-v4.md` §4.4, item 9)

- No face is lost.
- Every judgment's positive assertions hold.
- Nothing ungrammatical is admitted.
- `cargo xtask english-v4 strata --check` passes.
- No pathological forest explosion on real cards.

The landing reports the cheapest-tier figures and the judgment-set S/A/P/F,
but they are not gates (item 7). No Attachment Class, Production Cost tweak or
residual ticket is added to win a rank (item 8).

Standard constraints apply.

Census delta (measured on `unwpsuqt` → `orvowoml`): faces read 28,800 → 28,800 (of 32,828), total Readings 237,320 → 237,320, Surplus 101,963 → 101,963 (the `strata.json` report is byte-identical before and after); lost faces: none.
Judgment set: every judgment holds (`cargo test -p deckmaste_english_v4`, `judgments.rs`, all tests pass). `strata --check` passes; the cheapest tier is unchanged (singleton in 15,824 of 17,634 ambiguous units, summed 20,053, largest 8). The S/A/P/F tally has no tool in this tree, and the census report it would be computed from is byte-identical.
