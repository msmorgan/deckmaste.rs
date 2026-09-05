---
needs: [english-v2-closed-class-single-owner]
---
**Rehome the closed-class vocabularies whose names still record the
construction they were born in.** Single ownership (2026-09-04) pointed
general constructions at vocabularies named for one earlier consumer, so the
declared name now misdescribes the word it spells:

- `ObjectOrder { Any, Random }` spells the determiner of `in any order`; the
  quantifying determiner `any number of` now consumes `ObjectOrder::Any`, and
  its element field is called `order`. Note the `Determiner` lexicon already
  carries a member realizing `any`, so the right owner may be the lexeme
  rather than a vocabulary at all.
- `CostComparisonDirection { More, Less }` is consumed by four scalar
  comparisons that are not cost comparisons.
- `DistributionReplacement { Instead }` is a distributed-measure verb tail
  slot; the general replacement predicate now consumes it.

Decide each word's real owner (vocabulary vs closed lexicon), rename the
vocabulary or move the member, and re-spell the element fields to name the
role rather than the vocabulary. Ownership provenance strings
(`vocab:<Vocabulary>/<Member>`) and their tests move with it; zero coverage
change and no new form literals. Standard constraints apply.
