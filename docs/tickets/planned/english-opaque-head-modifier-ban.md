---
needs: []
---
**Measure, then ban, the opaque-nominal-head-with-known-modifier shape.**
The degenerate reading where an `Opaque` nominal HEAD carries a non-opaque
`NominalModifier::Noun` (a known noun demoted to modifier under an unknown
head) has produced wrong-but-round-tripping trees in two unrelated rounds:
the declarestep `cleanup`-split defect (2026-07-25) and the pluralposs
`their owners' hands except` / `all nonland permanents not` defects
(2026-07-25, neutralized for `except`/`not` specifically by the
`has_known_word` literal repair). The conjecture: that shape is never a right
tree in this corpus — real English noun-noun compounds put the UNKNOWN word in
modifier position, not head position, when the head is known. Work: a
corpus-wide categorical measurement in the declarestep-C style (thread an
`opaque_head`-style flag through `Features::Noun` → `Features::Nominal` and
enumerate every face where the shape occurs today), then — if the measurement
confirms zero legitimate occurrences — a features-level gate rejecting the
shape, so it becomes unconstructible rather than out-competed. No cost
changes. Note the standing instrument caveat: the noun-opacity census counts
opaque HEADS only, so this gate may shift census bookkeeping
(head↔modifier); require per-row attribution.
