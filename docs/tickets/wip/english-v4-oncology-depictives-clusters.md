---
needs: []
---

# english-v4: graded depictives, unclosed comma adjuncts and bare Clusters

Three findings from the post-Stratum-7 oncology re-scan. They interact on
Sphere of Annihilation, Maralen, Fae Ascendant, Dominaria's Judgment and
Spiteful Repossession. Each finding's mechanism and fix is a hypothesis:
verify the mechanism on its witness, then measure the fix and cite CGEL.

## F1: graded adjectives as depictives

`DepictiveAdjunct` and `DepictiveAdjective` exclude only `Grade:
Comparative`; `InterleavedDepictive` and `AdjectivalDepictive` have no
Grade exclusion. Adjective Phrase coordination leaves `Grade` unset, so
*less than or equal to …* passes. Witnesses: Sphere of Annihilation,
Maralen, Spiteful Repossession, Sycorax Commander, *tied for most*.

A depictive denotes a state of the predicand, not a relational comparison
(CGEL Ch. 4 §5.3, pp. 261–263; Ch. 6 §2.1, pp. 528–529). Before excluding
Equative, find which production reads *deals damage to target creature
equal to its power*; no face may be lost.

## F3: `internal_comma` in the syntax engine

A comma that opens a post-dependent closes only at the right edge of its
containing construction or before a coordinating comma, never before an
undelimited sibling (CGEL Ch. 20 §3.2.5, p. 1745). Witness: Dominaria's
Judgment, *if [you control an Island, from black]*. Aspect of Wolf's
*[X is …, rounded down], and [Y …]* must keep reading.

## F6: `Bare::licensed` in the syntax engine

`!self.held` short-circuits the `adjuncts != MIXED` check, and Clusters
peel off NominalAndPredicate PPs beside a bare subject (Sphere of
Annihilation). Crabomination, where *at random* is predicate-only, must
keep reading.

## Done when

- Each finding is fixed structurally, or reported as no defect with the
  reason.
- No face is lost; every removed Reading is a duplicate or non-English.

Standard constraints apply.

1. **Census delta**: pending.
2. **Judgment set**: pending.
