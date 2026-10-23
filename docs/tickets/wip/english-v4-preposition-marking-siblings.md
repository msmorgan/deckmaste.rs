---
needs: [english-v4-partitive-merge]
---
# The determinative and its *of*/*than* PP as siblings; delete PrepositionMarking (R5.6)

From `english-v4-child-name-control-and-category-fission-audit`, Finding D.
The owner ruled R5.6 in round 9.

## Owner's ruling (round 9, R5.6)

"Approve the measured restructure: determinative and its Of/Than PP as
siblings (as PairedComparativeNominal / SynesisDistributivePartitive), then
delete the PrepositionMarking feature and tunnel."

## Evidence

PrepositionMarking `["Oblique", "Comparative"]` (`english.ron:21`) records
whether a determinative's PP is *of*, *than* or neither. It is a closed-word
identity carried as a feature, and it reaches the consumers through the
Determinative Valence's `("PrepositionMarking", "PrepositionMarking")` agree
pair (`:1383`). Its consumers are Partitive (`:822`), ExceptivePartitive
(`:850`), ExceptiveMassPartitive (`:852`), ComparativePartitive (`:857`),
ModifiedComparativeDeterminer (`:900`), ComparativeFusedHead (`:958`) and
ComparativeQuantityCountDetermined (`:1100`).

`docs/decisions/english-lexical-analysis.md` §3 prefers a direct reference.
SynesisDistributivePartitive (`:830`) already takes the determinative and
`Selected(.., ["Of"])` as siblings. At this ticket's minting, `english.ron` has
no Production named PairedComparativeNominal. Take SynesisDistributivePartitive
as the model, and STOP if the owner's name points to a different one.

## Change

1. Restructure each consumer so that the determinative and its PP are
   sibling children, with the PP selected by `Selected(.., ["Of"])` or
   `Selected(.., ["Than"])`.
2. Delete the PrepositionMarking feature, its Valence agree pair, and every
   PrepositionMarking entry in Category feature lists.
3. Measure the change. The restructure must keep every analysis, and judgments
   that assert the old constituent structure are rewritten to the sibling
   analysis, naming each.

This ticket follows `english-v4-partitive-merge`, which moves the same
partitive Productions.

## Acceptance (round 9: `docs/decisions/english-v4.md` §4.4, item 9)

- No face is lost.
- Every judgment's positive assertions hold.
- Nothing ungrammatical is admitted.
- `cargo xtask english-v4 strata --check` passes.
- No pathological forest explosion on real cards.

The landing reports the cheapest-tier figures and the judgment-set S/A/P/F,
but they are not gates (item 7). No Attachment Class, Production Cost tweak or
residual ticket is added to win a rank (item 8).

Standard constraints apply.

Census delta (measured on xykuylsq → wxownvmo): faces read 28,801 → 28,801; total Readings 245,825 → 245,781; Surplus 106,859 → 106,817; cheapest tier singleton in 15,633 of 18,353 → 15,624 of 18,344; no verdict changed; no unit grew; lost faces: none.
Judgment set: all enforced entries hold at `COMPLETED_STRATUM` 6; S/A/P/F 704/233/10/92 (a statistic, not a gate).
