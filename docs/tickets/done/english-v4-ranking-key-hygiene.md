---
needs: []
---
# Ranking Key hygiene and kept-gate list

Implements owner-rulings-12 findings F4, F5, F6 and F8 (the oncology re-scan).

## Change

- F4: move the tree-side `CompiledGrammar::ranking_key(&Reading)` into test
  code (its only user is the `ranking_keys` cross-check test). Move
  `Segmentation` / `Parse::segments` out of `deckmaste_syntax` into
  `deckmaste_english_v4`'s census. Keep the forest-side RankingKey.
- F5: AdjectivalQuality `cost: 4` becomes `cost: 1`. Keep the pre-head
  AttachmentClass rule.
- F6: add SubjectInitialClause to the ADR §4.4 item 10 closed list (CGEL
  Ch. 12 §3.5.5 p. 1055), with a matching comment at its declaration.
- F8: record S7's four intrinsic coverage needs in the S7 ticket.

## Acceptance

- F4 and F6 leave the census byte-identical. F5 changes only the cheapest-tier
  statistic.
- Tests and clippy pass for changed crates; `strata --check` passes.

Standard constraints apply.

1. **Census delta** (`uuplusuywuny`, full corpus): faces read 28,801 → 28,801; total Readings 245,781 → 245,781; Surplus 106,817 → 106,817; cheapest-tier line unchanged (singleton in 15,624 of 18,344 ambiguous units, summed 21,723, largest 8); `strata` output byte-identical. Lost faces: none.
2. **Judgment set** (`uuplusuywuny`): all enforced judgments hold (`cargo test -p deckmaste_english_v4`); `strata --check` passes.
