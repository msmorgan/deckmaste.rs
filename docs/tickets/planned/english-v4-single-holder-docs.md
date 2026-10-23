---
needs: []
---
# Document the remaining single-holder values

Implements owner-rulings-12b.md finding F. Single-holder values the load report
still lists have no written justification: `ClauseMarking::To`,
`ComplementKind`/`ObliqueKind` MeasureNP and QuotedAbility,
`DeterminativeDistribution::Targeting`, `NounDistribution::Quantity`,
`NounKind`/`ObliqueNounKind` Manner, `PronounKind::Existential`, and the
one-member classes FusedHeadAdjective, RangeMarker and Pawprint.

## Change

Comments only, in `english.ron` and (where the value is declared there)
`crates/deckmaste_lexical_source_v4/src/schema.rs`, following the model of
Serial, FreeChoice, IndefiniteArticle and SingularOrQuantified: what
construction the value marks and why one lexeme carries it, cited to CGEL/OSG
(or the CR where it enumerates). A value with no genuine justification is
reported to the owner, not invented.

## Acceptance

Census byte-identical; `strata --check` passes.

Standard constraints apply.

1. **Census delta** (`<change>`): TBD.
2. **Judgment set** (`<change>`): TBD.
