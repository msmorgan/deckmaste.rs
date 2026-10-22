---
needs: []
---
# Coordinated bare relatives: a SubjectInitialClause coordination (R3, C-a)

From `english-v4-child-name-control-and-category-fission-audit`, Finding C.
The owner ruled R3 in round 9.

## Owner's ruling (round 9, R3)

"C-a only. Add a SubjectInitialClause coordination so coordinated bare
relatives parse (each creature you control and an opponent owns);
cross-reference-comment the 4 duplicated productions. Reject C-c (it would add
a unary Clause wrapper to every clause's Reading)."

C-b, which keys bare relatives on `Summary::fronted`, needs a new
`ProductionDecl` list and is not taken either.

## Evidence

SubjectInitialClause (`english.ron:121`) keeps a preposed dependent from
coming before a bare relative's Subject: "Bare relatives always have the
subject in initial position" (CGEL, Ch. 12 §3.5.5, p. 1055). To do so, it
restates four Clause Productions:
- RelativeSubjectPredicate restates SubjectPredicate;
- RelativeCliticSubjectPredicate restates CliticSubjectPredicate;
- RelativeFloatedEach restates FloatedEach;
- RelativeFloatedAll restates FloatedAll.

Each differs only in `$.ClauseMarking`. RelativeAdjunctGap is a fifth
SubjectInitialClause Production, restating AdjunctGap.

The Category also shuts out coordinated bare relatives. *each creature you
control and an opponent owns* has 0 Readings, against 1 with *that*.

Probe P3 added `Coordination(category: "SubjectInitialClause", …)`. No face
changed. Six units gained Readings, each wrong, for example Orzhov Charm's
*you control and all Auras you control attached to it*.

## Change

1. Add a Coordination schema over SubjectInitialClause, with the coordinators
   and shared features its conjuncts need, stated generally (§4.4, item 3).
2. Add a `+` judgment for *each creature you control and an opponent owns*.
   It asserts the coordinated relative and the Gap that each conjunct binds
   to *creature*. Look in the corpus for a real coordinated bare relative,
   and add an entry for each one you find.
3. Adjudicate the Readings the coordination adds (P3 found six units). A
   wrong but well-formed English Reading stays (§4.4, item 6). One that no
   English grammar admits is a defect, and you fix it before landing.
4. Comment each restated pair to point at the other: SubjectPredicate and
   RelativeSubjectPredicate, CliticSubjectPredicate and
   RelativeCliticSubjectPredicate, FloatedEach and RelativeFloatedEach,
   FloatedAll and RelativeFloatedAll, and AdjunctGap and RelativeAdjunctGap.
   Say that a change to one must be made to the other.

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
