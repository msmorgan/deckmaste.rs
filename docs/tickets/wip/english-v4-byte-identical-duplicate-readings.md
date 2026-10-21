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
