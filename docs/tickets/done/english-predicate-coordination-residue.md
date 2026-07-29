---
needs: []
---
**Additive-type subject-shared copular predicate coordination.** Admit the
narrow `... and is <NounPhrase> in <PrepositionalPhrase>` continuation, plus
its comma and asyndetic variants. The finite subject scopes over the complete
copular predicate; the final `in addition to its other types` phrase is a typed
copular adjunct [CR#205.1b]. Generic copular remainders, `or`, imperative
hosts, agreement mismatches, and `every/all creature type` remain excluded.

## Completion

- Added three append-last productions with a categorical finite-clause dot-1
  gate and structural reduce/lower checks for `and`, agreeing indicative `be`,
  a noun-phrase complement, and an `in` PP.
- Reused the existing n-ary `PredicateExpression::Coordinated` AST; no first
  predicate is structurally privileged and no spelling or card identity enters
  the grammar.
- Removed 33 clause recoveries across 31 supported faces and 761 source tokens,
  with no added unresolved row and no movement in any other recovery or lexical
  opacity cell.
- The rejected host/quantification stages remain owned by
  `english-predicate-coordination-redesign`.
