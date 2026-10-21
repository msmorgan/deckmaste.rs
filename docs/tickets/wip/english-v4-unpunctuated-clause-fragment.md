---
needs: []
---
# Unpunctuated `ClauseFragment` reading *Equip {N}* as an imperative clause

In the Oracle Style Guide (`docs/oracle-style-guide.md` §3, "Periods"), every
complete rules sentence, including an imperative, ends with a period; standalone
unpunctuated lines are keyword ability lines (*Equip {2}*), headers, or type
lines. Because `ClauseFragment` (`Document -> Clause`, `english.ron`) admits an
unpunctuated `Imperative` at the `Document` root, and *equip* is also a
monotransitive verb (`verbs.ron`) while `{2}` is a `SymbolNominalPhrase`
(`NounPhrase`), every standalone *Equip {N}* keyword line across 812 Equipment
cards (813 Corpus Units, 815 surplus Readings) is read a second time as an
unpunctuated imperative clause commanding the player to "equip" the mana symbol
`{N}`.

## Change

- Retire `ClauseFragment` (and `PrepositionFragment` if no Corpus Unit needs an
  unpunctuated root `Clause` or `PrepositionPhrase`), updating any unpunctuated
  clause/PP fragment entries in `judgments/*.txt` to name their root category
  explicitly (`+Clause`, `*Clause`, `+PrepositionPhrase`), or otherwise restrict
  `Document` root / `SymbolNominalPhrase` so standalone keyword lines cannot be
  read as unpunctuated imperative clauses.

## Done when

- *Equip {2}* has a single Reading (`KeywordAbilityList`).
- The 815 surplus Readings across 813 Corpus Units are gone.
- No face is lost, and all judgment files pass.

Standard constraints apply.
