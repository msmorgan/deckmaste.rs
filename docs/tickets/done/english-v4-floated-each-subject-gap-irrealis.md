---
needs: []
---
# FloatedEachSubjectGap lacks SubjectGap's Irrealis exclusion

A side finding of `english-v4-child-name-control-and-category-fission-audit`.
The owner approved a ticket for it in round 9.

## Owner's ruling (round 9, side findings)

"FloatedEachSubjectGap lacks SubjectGap's Irrealis exclusion: align or state
why."

## The finding, as the audit found it

SubjectGap (`english.ron:1266`) has `Exclude("head.Inflection", "Irrealis")`.
FloatedEachSubjectGap (`:1268`), the same subject-gap clause with a floated
*each*, does not. The audit classified this as drift.

## Change

1. Check that the irrealis form is ungrammatical in a subject-gap relative
   with floated *each*, as it is without *each*. If so, add the Exclude to
   FloatedEachSubjectGap. The Exclude enforces the same agreement rule that
   SubjectGap's does, so it is not new suppression (§4.4, items 2 and 10).
2. If the two rightly differ, state why in a comment on FloatedEachSubjectGap.
3. List any unit whose Readings change.

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

Census delta (measured on `ztmvqxzl` → this landing, `xrnurmzl`): faces read 28,801 → 28,801, total Readings 245,825 → 245,825, Surplus 106,859 → 106,859; no unit's Readings, verdict or cheapest tier changed; lost faces: none. Adding the Exclude changed nothing: irrealis *were* is third singular only, and the Production already requires a plural head.
Judgment set: holds (`cargo test -p deckmaste_english_v4`); all 3,258 non-seed entries give identical outcomes, derivation counts and cheapest tiers before and after.
