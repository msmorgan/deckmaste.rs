---
needs: [english-v4-case-to-solve-label]
---
# Nightbound is its own keyword ability, not a variant of Daybound

In `crates/deckmaste_lexical_source_v4/catalogs/keyword_abilities.ron`, the
Daybound entry (`rule: "[CR#702.145]"`, `name: "Daybound and Nightbound"`)
lists `nightbound` under `variants:`, citing [CR#702.145e]. The glossary's
**Keyword Variant** is "a keyword ability the Comprehensive Rules define as a
variant of another keyword ability". Nightbound is not one, so the catalog
files it under a term that does not describe it.

## Owner's ruling (round 8, 2026-10-22)

Move Nightbound out of Daybound's `variants:` into its own catalog entry, for
the same reason *To solve* got its own entry rather than a variant of Solved
(`english-v4-case-to-solve-label`). The glossary's **Keyword Variant** stays
as it is.

## Evidence

- [CR#702.145] pairs the two under one heading, and [CR#702.145a] says only
  that they "are found on opposite faces of some double-faced cards".
  [CR#702.145b] defines daybound and [CR#702.145e] defines nightbound, each
  with its own meaning.
- The CR never calls nightbound a variant of daybound. Compare the rules that
  do: "Megamorph is a variant of the morph ability" [CR#702.37b] and
  "Multikicker is a variant of the kicker ability" [CR#702.33c].
- Granting daybound doesn't grant nightbound. [CR#702.1c] extends a grant only
  to a keyword's "variants or variables", and the glossary entry rests on that
  ("A grant of a keyword that has variants grants each appropriate variant").

## Change

- Give Nightbound its own entry in `keyword_abilities.ron`, citing
  [CR#702.145e], and remove it from Daybound's `variants:`. The lexeme
  (`keyword:nightbound`) and its parameters should not change.
- `tests/cr_coverage.rs` (`keyword_abilities`) allows one entry per rule number
  and matches each entry's name to the [CR#702] heading. [CR#702.145] already
  belongs to Daybound, so the Nightbound entry is checked the way
  `english-v4-case-to-solve-label` checks *To solve*: against its own rule's
  text. Extend that check from "outside [CR#702]" to "under a heading another
  entry holds", without loosening the heading check for every other entry.
- Decide whether Daybound's `name` stays the CR heading "Daybound and
  Nightbound" or becomes "Daybound". The coverage test pins it to the heading
  today.

## Done when

- Nightbound is its own catalog entry, and no `variants:` entry is something
  the CR does not call a variant (or a special form, as with "bands with
  other" [CR#702.22b]).
- No face is lost, every judgment holds, and
  `cargo xtask english-v4 strata --check` passes. The census should not move.

Standard constraints apply.

Census delta (measured on `kxmoylxk` → this landing): faces read 28,801 → 28,801; total Readings 245,781 → 245,781; Surplus 106,817 → 106,817; changed units none; lost faces none.
Judgment set: `cargo test -p deckmaste_english_v4` passed, including all 6 judgment tests; no judgments added, rewritten or retired.
