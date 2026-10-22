---
needs: [english-v4-gate-review]
---
# FreeChoice: the Requires become `Word("Any")` (R5.4)

From `english-v4-child-name-control-and-category-fission-audit`, Finding G
(the T list). The owner ruled R5.4 in round 9.

## Owner's ruling (round 9, R5.4)

"Approve: FreeChoice Requires → Word("Any") (verify `any turn` on Teferi,
Master of Time first); its Excludes under the R1 test."

## Evidence

DeterminativeDistribution `FreeChoice` has one holder, *any*. Its Requires
select *any* by value:
- FreeChoiceDetermined (`english.ron:1107`);
- FreeChoiceQuantity (`:1109`);
- TransparentQuantity (`:1111`);
- TemporalQuantity (`:1125`).

Its Excludes (Determined `:1077`, DeterminedModifiedQuantity `:798`) are R1
gates, which `english-v4-gate-review` decides. This ticket follows it.

The audit warned that the fallback distribution would lose *any turn* (CGEL,
Ch. 5 §5, p. 361).

## Change

1. First, check *any turn* on Teferi, Master of Time. Record its Readings
   before the change, and confirm the change keeps them.
2. Replace each `Require(.., "FreeChoice")` with a direct reference to the
   Closed Word: `Word(.., "Any")` in the determiner slot
   (`docs/decisions/english-lexical-analysis.md` §3). Where the slot is a
   whole DeterminerPhrase that *any* heads, keep what the Production reads
   from it.
3. If `english-v4-gate-review` retired the FreeChoice Excludes, delete the
   `FreeChoice` value once nothing reads it. If it kept them, the value stays
   with those readers, and the landing says so.

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

## Resolution (owner ruling, round 10)

Closed without a grammar change of its own. The owner kept both FreeChoice
Excludes (`docs/decisions/english-v4.md` §4.4, item 10, closed list), so
`DeterminativeDistribution` `FreeChoice` keeps its readers and is a justified
single-holder value on *any*, as `NounDistribution` `Serial` is on *time*: it
routes *any* to the Productions that carry number transparency (CGEL, Ch. 5
§3.3, pp. 349–351) and keeps a second tree of one analysis out
(`docs/decisions/english-lexical-analysis.md` §2). Change 2 is not made: a
`Word("Any")` in place of the value would not let Determined and
DeterminedModifiedQuantity tell *any* apart. The justification is documented
on the value in `english.ron` and in `deckmaste_lexical_source_v4`'s
`schema.rs`, landed with `english-v4-gate-review` in xmzmyptl.

Census delta (measured on xmzmyptl): none of its own; resolved by the keep ruling, with the grammar unchanged; lost faces: none.
Judgment set: unchanged from `english-v4-gate-review`'s landing (xmzmyptl), all holding.
