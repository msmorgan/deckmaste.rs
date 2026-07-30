---
needs: []
---
**Finish the surface-fact enumeration where round `sfdiet` stopped.** The
systematic sweep in `english-surface-fact-diet` covered
`syntax/{ability,clause,phrase}.rs` exhaustively — every `pub struct` and
`pub enum` — and left three things unmeasured. They are recorded here rather
than buried in that ticket's done-record.

1. **`word.rs`, `word/*.rs`, `numeral.rs` were never enumerated.** They sit
   outside `syntax/` and were only sampled. Enumerate them the same way: every
   field populated from a surface observation rather than from meaning.
2. **`Numeral` (digit vs word)** — population mechanism traced, no derivation
   attempted. Oracle text writes some counts as digits and some as words; find
   whether the choice follows the value, the head, or the construction.
3. **`RelativeMarker`** — the `that` / `which` / `who` choice. An animacy
   hypothesis was raised and never taken to a corpus count. Start by counting.

Use the measurement technique the round settled on, which is *not* the one the
parent ticket described: do **not** delete a field to test it. Swap the
candidate derivation into the renderer with the field still in place and
populated, then run `cargo xtask english roundtrip --list`. A stored bit
round-trips clean whatever it holds, because the renderer replays it — deriving
is the only way to make a disagreement visible.

Two rules the round paid for, worth reusing:

- A disagreement can mean the **tree** is wrong, not the derivation. Round
  `sfdiet` found the Arrest/Pacifism misparse that way (see
  `english-coordination-comma-defects`). Read the witnesses before concluding a
  rule is free.
- **Corpus-clean is not correct.** A per-keyword rule for
  `KeywordArgument::Named.separator` reached zero corpus mismatches and was
  still rejected, because a test asserts that shape is deliberately
  keyword-independent and its fixture would break. Run the test suite before
  believing a derivation, not after.

Keeping a field is a legitimate outcome — document the refutation on the field
with its witnesses, as that round did for eleven of them. Standard constraints
apply.

## Two refutations from that round that do not hold up

Both were recorded as settled and are not. Re-test before trusting either.

**`ComparativeWord` — the cited minimal pair is not minimal.** The round kept the
field on `total power 2 or more` vs `total power 8 or greater`, described as the
same head class and direction. The two differ in **syntactic frame**:
`Tap any number of other creatures you control with total power 2 or more:` is a
nominal qualifier (a saddle cost), while `if creatures you control have total
power 8 or greater` is a predicate complement. Corpus checks that motivate a
frame-sensitive retest: `with power N or more` has **zero** occurrences, while
`with power N or greater` is well attested (`Regenerate target creature with
power 5 or greater`, `Target creature with power 4 or greater`, `a creature with
mana value 6 or greater`). So a rule keyed on frame — qualifier vs predicate —
or on the statistic head is untested, not refuted. Direction stays semantic
either way.

**`KeywordListSeparator` — the "reminder text" explanation is unverified.** The
round recorded its 34 mismatches against an always-`Comma` rule as "genuine WotC
semicolon style, likely tied to now-stripped reminder text." That causal story
was never checked, and a search for semicolon-separated keyword lists does not
turn up the shape it predicts. Treat the 34 as **unclassified**: enumerate them
and look at the actual faces before concluding the variation is language rather
than a parse or a stripping artifact. Note that reminder text is *stripped*, not
parsed — `strip_reminder_text` removes parenthesized groups at the input
boundary before tokenizing — so any theory involving reminder text has to
explain an effect that survives its removal.
