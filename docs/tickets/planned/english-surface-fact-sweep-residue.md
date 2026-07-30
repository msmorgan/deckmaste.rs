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
