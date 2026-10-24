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

## Decisions

- HIGH-1 takes the second option. Under the first, the Clause would need a
  copy of every post-verbal Adjunct the Verb Phrase coordination already
  hosts (*Draw a card and discard a card this turn* would lose its wide
  scope). A privative Clause feature `ClauseType: Imperative`, unshared by the
  mixed-mood coordinations, keeps CoordinatedClauseAdjunct off chains of bare
  imperatives.
- The *and only* duplicate occurs in finite clauses too (*the player can
  activate mana abilities only if … and only if …*), so a Finite requirement
  would not fix it. A new engine declaration, `unadjoined`, keeps
  AddedElement off a Clause ending in a coordinable Predicate Adjunct (CGEL,
  Ch. 15 §4.5, p. 1346: a new element, not a coordinate).
- *only once each turn* is one analysis. The judgments already endorse the
  separate Adjunct at the end of a Verb Phrase (`once_frequency.txt`,
  `preposed_frequency.txt`), so EndAdverbial no longer takes a
  FrequencyInterval phrase; the fronted *Once each turn, …* keeps it.

Census delta (klmtvnok): faces read 32,769 of 32,828 before and after;
Readings 344,348 → 338,440, on 741 units; lost faces: none.
Judgment set (klmtvnok): all hold, 3,219 enforced and 317 pending.
