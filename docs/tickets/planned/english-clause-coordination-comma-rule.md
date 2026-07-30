---
needs: []
---
**Find the comma rule for full-clause coordination, or prove there isn't one.**
Split out of `english-derived-serial-comma` (round `sfdiet`, 2026-07-30):
`ClauseCoordination.comma` (`syntax/clause.rs`) is the one coordination field
where the phrase-level count rule is not merely inexact but **inverted**, so it
was left stored rather than guessed at.

Measured: deriving `comma == conjunction.is_none() || rest.len() >= 2` — exact
for five phrase-level structs — mismatches **281 faces**. At the full-clause
level a two-member coordination almost always *keeps* the comma, regardless of
connective: Accumulated Knowledge `Draw a card, then draw cards equal to …`,
Ageless Sentinels `it becomes a Bird Giant, and it loses defender`. The
phrase-level Oxford model is simply the wrong model for clauses joining two
complete clauses that each carry their own subject.

Candidates to measure, in order:

1. **Unconditional `true`** — if every clause coordination in the corpus takes
   the comma, the field is a constant and deletes outright. Measure first; it is
   one line and it either succeeds completely or names its exceptions.
2. If exceptions exist, check whether they correlate with conjunct length or with
   the connective (`and` / `or` / `then`), the way `CoordinationJunction`'s
   residue does — see the `Then`-aware refinement recorded on that field's doc,
   which reached 8 residual faces on the shared-subject predicate coordination.
3. If neither is exact, **keep the field and document the refutation** with its
   witnesses, as this round did for `CoordinationJunction`,
   `SetExceptionNounPhrase`, `Attachment<T>`, and `ComparativeWord`.

Method (the one that worked all through round `sfdiet`): do **not** delete the
field to measure it. Change the renderer's read to the candidate derivation with
the field still in place and populated, then run `cargo xtask english roundtrip
--list`; every face where derived and stored disagree mismatches, and `--list`
names it. A wrong stored bit round-trips clean, so deriving is the only way to
see a disagreement.

Watch for the same class of finding as `english-coordination-comma-defects`: a
disagreement can mean the *tree* is wrong rather than the derivation. Read the
witnesses before concluding the rule is free.

Standard constraints apply.
