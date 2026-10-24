---
needs: []
---
# Duplicate trees found by the post-S7 re-scan (group A)

Source: the post-S7 re-scan (Argon, 2026-10-24), group A, findings HIGH-1 and
HIGH-3. They are hypotheses; confirm each with its witness parse first.
Principle: one tree per analysis (english-lexical-analysis §2).

## HIGH-1: trailing adjunct on a coordinated imperative

*Draw a card and discard a card at the beginning of the next end step.*
reportedly gives 3 Readings for 2 analyses: `CoordinatedClauseAdjunct`
duplicates `Imperative(Adjunct: VerbPhrase(Coordination, PP))` because the
`uncoordinated` check on `Imperative` cannot see a coordination wrapped by a
post-adjunct. Fix by the most general means: either propagate coordinated
status through transparent VP post-adjuncts, or restrict
`CoordinatedClauseAdjunct` to coordinations with a Finite conjunct. Keep the
mixed-mood witnesses in `trailing_adjunct_height.txt`.

## HIGH-3: *and only*, and *only once each turn*

- `AddedElement` / `QualifyingFragment` duplicate `CoordinatedAdjunct` on
  imperatives *Activate only X and only Y*. Remove the duplicate; the
  declarative *…, but only …* and *Only your opponents may … and only …* keep
  their analysis.
- *only once each turn* reportedly gets two trees (`FrequencyInterval` versus a
  clause-level `TemporalAdjunct`). If they are one analysis, canonicalize.
- Witness: *Activate only as a sorcery and only once each turn.* (4 Readings).

## Acceptance

Each witness has exactly its distinct analyses; census `--check` passes with
no face lost; judgments hold.

Census delta:
Judgment set:
