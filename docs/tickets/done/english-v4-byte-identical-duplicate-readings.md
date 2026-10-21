---
needs: []
---
# Find and remove the multiplier behind byte-identical duplicate Readings

During [`english-v4-spurious-ambiguity-multipliers`](../done/english-v4-spurious-ambiguity-multipliers.md)
the old and new Readings of the 3,446 changed units were compared as canonical
trees (category, span, function, head, parent). 225 of those units still hold
1,181 groups of Readings that print identically, even with tree structure
compared. Animar is one example: 2 derivations, with byte-identical printed
Readings. That change did not introduce any of these, since the old tree had at
least as many copies of each. Only the changed units were compared, so the
corpus-wide count is unknown.

Two derivations that no Reading distinguishes mean the census overcounts and
generation has two sources for the same text. The likely cause is duplicate
lexical matches: the same span matched by two Lexemes or two entries of one
Lexeme with identical facts. That is not yet confirmed.

## Change

- Measure the whole corpus: count units whose Readings include two or more that
  print identically, and the total of surplus copies.
- Find each cause (duplicate lexical entries, duplicate frames, two productions
  with identical output) and remove it at the source. A dedup pass over finished
  Readings does not count as a fix.

## Done when

- No unit has two Readings that print identically.
- No face is lost, and every Reading removed is a copy of one that remains.
- Animar has one Reading for each distinct analysis, covered by a `+` Judgment
  on its real text.

Standard constraints apply.

## Record

- Corpus-wide, before: 2,100 units held 24,990 groups of Readings that print
  identically under (category, span, function, head, parent), with 27,274
  surplus copies. The Packed Forest never yields one tree twice: every unit's
  Derivation count equals its count of distinct trees.
- Copies that a defect caused, now removed at the source:
  - Subject *who* took no Number from its clause, so both of
    its declared Numbers read every subject relative. `SubjectRelative` now
    agrees the marker with the clause (1,597 copies).
  - *more* in a measured comparative (*one more spell*, *four more creatures*)
    read as both the count and the mass Lexeme. `MeasuredComparativeDeterminer`
    and `MeasuredComparativeDetermined` now agree the quantifier's
    Countability (13).
  - Irrealis *were* projected no `VerbForm`, so either participial
    postmodifier could value it (*cards that share a color were milled this
    way*). It is now `Finite` (`deckmaste_syntax`, `leaf_facts`).
  - A genitive Noun Phrase (*that spell's*) could be a Predicative or a
    Subject, which copied the fused-head independent genitive.
    `NominalPredicative` and `SubjectPredicate` now exclude `Case` `Genitive`.
  - `MannerAdjunct` and `TemporalQuantity` gave a Noun Phrase with no
    `NounKind` a manner or temporal kind (*put … attached to it any number of
    Aura cards*). They now `Carry` it.
  The last three removed only wrong analyses, in 16 units. In each, *were* or a
  genitive took a role it cannot fill, or a Noun Phrase with no kind became an
  Adjunct. Every other Reading removed is a copy of one that remains.
- What remains is not copies. *cast*, *put*, *cost* and *bid* are each a
  present and a preterite Word Form, and *Equipment* or *Merfolk* is a
  singular and a plural. That is a grammatical distinction a Reading keeps
  (`english-lexical-analysis.md` §2). *reduced by {2}* reads by-agent and
  `PP[by]` complement under different Selected Frames. The parse dump now
  prints each head's Word Form, and no two Readings of any unit print
  identically (0 groups; before, about 27,165 surplus copies printed alike in
  the dump). Animar's two Readings of *Whenever you cast …* are its present
  and preterite *cast*.
- The zero-plural Readings are wrong where *target* or *all* should fix the
  Number. That is ticketed: `english-v4-zero-plural-determiner-number`. So
  are the preterite in present-only clauses
  (`english-v4-clause-tense-selection`) and the agent Reading of an amount
  (`english-v4-passive-by-agent-vs-by-extent`).
- Judgments are in `judgments/syncretic_word_forms.txt`.

Census delta (measured on soxswlps): faces read 28,709 → 28,709; total Readings 346,725 → 344,736; lost faces: none.
Judgment set: 2,914 enforced at `COMPLETED_STRATUM` 6, all hold; 192 pending.
