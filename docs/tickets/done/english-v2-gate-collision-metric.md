Make the gate's collision/structural metric real (dissolution review F1).
`literal_lexicon_collisions` in the coverage summary is never assigned —
a constant zero that reads as a live gate. The 2026-09-02 ruling asked for
census VISIBILITY of the noun-as-literal class. Replace the dead counter
with measured per-unit structure: per-claim-class byte shares are already
computed; add a structural-depth figure (nonterminal nodes per sentence,
longest single-form literal span) and a literal-vs-lexicon surface
collision count that the tripwire's exact-surface check feeds. Report in
the summary line; no ratchet on it yet — visibility first. Also extend
the compile-time tripwire to `vocab` surfaces (review F6: two live
duplicates of CommonNoun surfaces — BareLocativeNoun{exile,hand},
ControllerNoun{opponent,player} — the residue ticket deletes them; the
tripwire must then keep them out). Standard constraints apply.

## Landing record

- Coverage stayed 15,932 -> 15,932 selected-covered units; constructions
  stayed 398 -> 398; the schema-2 lock stayed at the exact 15,932 identities
  and `coverage --check` passed without a lock rewrite.
- Coverage report schema 3 -> 4. The live census measures 760,972 selected
  nonterminal nodes across 15,932 units (47.764 per unit), an 11-byte longest
  single form-literal claim, and 2 exact fixed-surface/lexicon collisions.
  Those two are the category-safe homographs `target` (adjective / noun) and
  `untap` (adjective / verb); the collision metric is visible but is not
  ratcheted.
- Positive gate artifacts: `cargo test -p deckmaste_construction_core` (359
  passed), `cargo test -p deckmaste_english_v2` (all unit, integration, and
  doc tests passed), `cargo test -p xtask english_v2::coverage::tests` (19
  passed), `cargo test -p xtask english_v2::coverage_lock::tests` (11 passed),
  and `cargo xtask english_v2 coverage --check` (zero selected-uncovered,
  unresolved, internal, ownership, or round-trip failures).
- Deviations and additions: no construction or corpus fixture was added or
  deleted. The compiler now emits the requested vocab-surface inventory. The
  tripwire permits adjective readings to remain category-safe homographs but
  rejects noun-shaped vocab/lexicon collisions, including any reintroduction
  of the residue ticket's deleted noun vocabs. Exact generated-item count pins
  were updated for the added inventory type and table. No STOP was taken.
