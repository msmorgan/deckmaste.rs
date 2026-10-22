---
needs: []
---
# Merge PartitiveNounPhrase into PartitiveHead (R4)

From `english-v4-child-name-control-and-category-fission-audit`, Finding C.
The owner ruled R4 in round 9.

## Owner's ruling (round 9, R4)

"Approve merging PartitiveNounPhrase into PartitiveHead, measured (0 faces
lost)."

## Evidence

PartitiveNounPhrase (`english.ron:140`) keeps PP postmodifiers off *each/any/all
of X*. PartitiveHead (`:143`) allows them on *two of X*: the Adjunct schema at
`:1399` hosts them. That is inconsistent, and English admits both. The
Productions on PartitiveNounPhrase are Partitive (`:822`), DistributivePartitive
(`:824`), SynesisDistributivePartitive (`:830`), AnyPartitive (`:833`),
AnyMassPartitive (`:835`) and MaximumPartitive (`:839`).
DeterminativePartitiveHead (`:1034`) parallels PartitiveFusedHead (`:1033`).

No `!` or count witness involves a PP. There is a risk, low to medium:
PartitiveHead's DeterminerPhrase heads would start agreeing on
DeterminativeDistribution. PartitiveHead itself probably exists to leave that
feature out of implicit head agreement (Finding C).

## Change

1. Move the PartitiveNounPhrase Productions to lhs PartitiveHead. Delete
   PartitiveNounPhrase and DeterminativePartitiveHead. That is −1 Category
   and −1 Production.
2. Resolve the DeterminativeDistribution agreement. The merged Category must
   carry what the former PartitiveNounPhrase consumers read, and must not
   force the feature onto the old PartitiveHead Productions. A new feature,
   value or Production added only to keep a Reading out is frozen (§4.4,
   item 2). If the merge needs one, STOP.
3. The new PP-postmodifier Readings on *each/any/all of X* are well-formed
   English and stay (§4.4, item 6). Where one ranks ahead of the endorsed
   Reading, that is reported, not fixed.
4. Measure: 0 faces lost.

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

Census delta (measured on `<change>` → this landing): faces read, total Readings and Surplus before and after; lost faces by name, or "none".
Judgment set: the result of running it.
