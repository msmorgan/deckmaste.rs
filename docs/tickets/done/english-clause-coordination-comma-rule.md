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

## Completion — field KEPT, refutation documented

Measured on the corpus (re-measuring the baseline first, as the `block`
vocabulary fix had moved it from 281):

| rule | mismatches |
|---|---|
| unconditional `true` | 1016 |
| count rule (exact for five sibling structs) | 303 |
| `Then`-aware refinement | 127 |
| conjunct length / clause-variant split | refuted, see below |

Only ~12% of two-member clause coordinations take the comma, so the field is not
a constant. The `Then` connective itself is **180/180 exact** — a clean sub-rule
even though the whole field is not derivable.

Length and complexity are refuted by minimal pairs with identical continuations
and no AST-visible difference. For `… and its activated abilities can't be
activated`: Gelid Shackles (`Enchanted creature can't block,`) takes the comma
while Edifice of Authority (`target creature can't attack or block`) does not,
though the latter's first clause is *longer* — the counterexample runs in the
wrong direction for any length threshold. Same story for `… may spend mana as
though it were mana of any color`: Share the Spoils, Gale's Redirection,
Covetous Urge, Cunning Rhetoric and Mezzio Mugger take the comma; Daxos of
Meletis, Grenzo Havoc Raiser, Hurl Through Hell, Robber of the Rich and The
Ruinous Powers do not.

This is Oracle house-style drift, not grammar. No wrong trees were found — the
residue is genuine source inconsistency, confirmed by direct oracle-text
comparison rather than inferred. The field stays, and the refutation with these
counts and witnesses is recorded on `ClauseCoordination` in `syntax/clause.rs`.

Doc-only change; round-trip 31685/31685 clean, 2942 tests, recovery census
byte-identical, fidelity 0 failing.
