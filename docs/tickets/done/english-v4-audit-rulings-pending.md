---
needs: []
---
# Resolved (owner round 9): the open rulings of the child-name and Category-fission audit

**Resolved.** Left in `planned/` for the coordinator to close.

This ticket carried the rulings that
`english-v4-child-name-control-and-category-fission-audit` left open. The
owner ruled on all of them in round 9 (2026-10-22). Each ruled fix is now its
own ticket, and the policy is recorded in `docs/decisions/english-v4.md`
§4.3–§4.4.

## Policy

The owner did not choose among R1's options (a), (b) and (c). The ruling
replaces the premise that a Reading must rank first:
- The Ranking Key is a heuristic traversal order, not a correctness gate.
- The cheapest-tier figures and the judgment-set S/A/P/F are reported
  statistics.
- No attachment class, cost tweak or residual ticket is added to win a rank.
- Judgments assert grammaticality only, and `ranks first` is retired from the
  harness.
- `english-v4-ranking-residuals` is retired.

This is recorded in `docs/decisions/english-v4.md` §4.4, items 6–10 (and §4.3
reworded), along with the hard landing acceptance list.

## Where each ruling went

| Ruling | Decision | Ticket |
|---|---|---|
| R1. Gates | Keep a gate only if it is a grammatical or register formation rule. Otherwise retire it, unless its retirement makes forests explode. Fill the ADR's closed list of kept gates, and check the AttributeChain Negative* de-duplication. | `english-v4-gate-review` |
| R2. Head renames | B-a: keep the three renames with comments, and document the reserved child names on `ProductionDecl` | `english-v4-reserved-child-names-contract` (edited in place) |
| R3. SubjectInitialClause | C-a only: a SubjectInitialClause coordination, and cross-reference comments. C-c is rejected. | `english-v4-subject-initial-clause-coordination` |
| R4. PartitiveNounPhrase | Merge into PartitiveHead, measured | `english-v4-partitive-merge` |
| R5.1 Privative features | Amend english-lexical-analysis §3 and the check, and delete VerbKind `Lexical` and AdjectivalPassive `Derived` | `english-v4-privative-features` |
| R5.2 Interrogative *how* | A Closed Word `How` | `english-v4-interrogative-how-closed-word` |
| R5.3 *another* class | Unify SingularOrQuantified and IndefiniteArticle, keeping *another day* | `english-v4-another-class-unification` (needs `english-v4-free-choice-any`) |
| R5.4 FreeChoice | The Requires become `Word("Any")`. The Excludes go under R1. | `english-v4-free-choice-any` (needs `english-v4-gate-review`) |
| R5.5 Targeting | Left as is, following R2 B-a | none |
| R5.6 PrepositionMarking | The determinative and its PP as siblings, then delete the feature and the tunnel | `english-v4-preposition-marking-siblings` (needs `english-v4-partitive-merge`) |
| Side finding: ParticipialSupplement | Measure, then drop the transparency or keep it with a reason | `english-v4-participial-supplement-gap` |
| Side finding: FloatedEachSubjectGap | Align it with SubjectGap's Irrealis exclusion, or state why it differs | `english-v4-floated-each-subject-gap-irrealis` |
| Coverage: Acclaimed Contender | Find the rule that keeps the analysis out, and add it as new coverage | `english-v4-acclaimed-contender-coverage` |

The ranking-premised acceptance wording of the open English v4 tickets was
updated to the round-9 list in the same change. That includes
`english-v4-head-final-last-head-line`, `english-v4-equative-multiplier` and
Stratum 7. `english-v4-agency-outfitter-coordination-ranking` is recast as the
grammatical admission fix of its lead 1: Right Node Raising must take the
right edge of a conjunct.

Closed: resolved by owner round 9; each ruled fix is its own ticket, and integration moves this one to done.

Census delta (measured on `okprvpoz` -> this landing): documentation and comment changes only; faces read 28,801, total Readings 245,825 and Surplus 106,859 before and after; lost faces: none.
Judgment set (measured on `okprvpoz`): not affected; `cargo xtask english-v4 strata --check` passes and the crate tests pass.
