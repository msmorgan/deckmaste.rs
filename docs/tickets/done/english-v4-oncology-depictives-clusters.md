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

## Outcome

- **F1: no defect.** *equal to its power* in *deals damage to target
  creature equal to its power* reads through PostposedAdjective; the
  depictive beside it is a second analysis, so the proposed exclusion would
  lose no face. It is still not adopted: a graded Adjective Phrase is a
  predicative adjunct like any other (CGEL, Ch. 6 §2.1, p. 529: every
  predicative-adjunct adjective is also a predicative complement, and
  *less than or equal to N*, *equal to N* are predicative complements;
  Ch. 13 §5, p. 1154, [47iv]: a comparison phrase as predicative adjunct).
  The witnesses' depictives (*a spell with mana value [less than or equal
  to …]*, *deals damage … [equal to the difference]*) are grammatical and
  semantically anomalous, which is semantics' to reject. The cost change
  would be ranking work only (principle 4).
- **F3: fixed.** A phrase that ends in a comma-set post-dependent is
  followed by a mark, which closes the comma, or ends its Production
  (`Summary::open_comma`; CGEL, Ch. 20 §3.2.5, p. 1745 [25]). Dominaria's
  Judgment 88 → 32; Aspect of Wolf unchanged. A dash closes it too
  (*choose up to X, where X is … —*, Riku of Many Paths, Bumi). The copy
  exceptions halve (Phantasmal Image 48 → 24, Mockingbird 2,168 → 1,084):
  *[…, except it's an Illusion …] and [it has …]* is gone, since it would
  need a comma before *and*; *except* keeps the whole coordination. The
  Phantasmal Image and Machine God's Effigy judgments
  (`enter_as_copy_except_clause.txt`) are rewritten to drop that second
  group.
- **F6: fixed in part.** A bare Cluster is licensed only beside Clusters of
  necessity, each holding an Adjunct no Nominal hosts (*at random*,
  `Summary::nominal_unhosted`); otherwise the Objects coordinate in
  parallel and the bare Cluster is the breach CGEL stars (Ch. 15 §4.3,
  p. 1341, [17iia]). Sphere of Annihilation 2,227 → 1,288; Crabomination
  keeps its judgment. The `MIXED` short-circuit is no defect: `MIXED`
  conditions a bare Cluster's licence only, and Clusters ending in
  different numbers of Adjuncts are parallel in their Slots.

## Done when

- Each finding is fixed structurally, or reported as no defect with the
  reason.
- No face is lost; every removed Reading is a duplicate or non-English.

Standard constraints apply.

1. **Census delta** (`mxpyosxm`): faces read 32,825 → 32,825 of 32,828; Readings 340,496 → 336,755; 76 units fell, none rose (Sphere of Annihilation 2,227 → 1,288, Dominaria's Judgment 88 → 32, Mockingbird 2,168 → 1,084); lost faces: none.
2. **Judgment set**: all 3,346 hold (0 pending)
