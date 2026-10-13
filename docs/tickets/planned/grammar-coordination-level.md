---
needs: []
---
# Coordination: same-coordinator grouping and bare-imperative level

Stop producing coordination trees that differ only in two approved ways: how a chain joined by the same coordinator is grouped, and whether a chain of bare imperatives is coordinated as clauses or as verb phrases. "Draw a card and gain 1 life." has 2 trees, with "and create a token." 6, with "and discard a card." 21; each is one instruction list. A stratified sample puts the removable trees at about 24,000 (10,000 to 45,000; the heavy tail makes the estimate loose), almost all of them imperative-level twins. Measurements, the 30 trial texts and declaration sketches are in `docs/evidence/grammar-review-2026-10-11/coordination-level-measurement.md` (untracked).

## The gate, built first

A restricted normal form in the census tool, in the style of the nominal one, with no grammar annotations. It erases exactly two things and nothing else:

- the grouping of a chain whose links are the same coordinator (exact coordinator identity, not `CoordinationKind`, which is too coarse), keeping member order, each link, punctuation and every member subtree;
- for a chain of bare imperatives only, the difference between coordinated imperative clauses and one imperative over coordinated verb phrases.

It keeps distinct: subjects, modals, quantifiers, any dependent and the head it attaches to, gaps, correlatives, mixed coordinators, punctuation. These guarantees are tested on hand-built trees fed to the normaliser. The census records each unit's set of normal forms under a new version; the check fails on a lost face, an incomplete enumeration, or a unit whose set differs. Landing the check alone changes no Reading.

## The change

- A chain of bare imperatives is coordinated as imperative clauses.
- A chain joined by the same coordinator has one grouping: the existing serial forms where they apply, otherwise right-associated binary coordination.
- Verb-phrase coordination is not disabled in general, and no global level convention is introduced: a shared subject, modal or auxiliary still coordinates below it.
- Use the `coordinable` blocks, templates and existing summaries; no new compiler capability is expected. If one turns out to be needed, stop.

## Not in this ticket

Which head a trailing phrase attaches to, and whether a modifier or determiner before a coordination covers one conjunct or all ("red creatures and planeswalkers", "all creatures and lands", *target*, cardinals). Those are attachment and scope questions for later work. Also left alone: the finite-plus-imperative hybrid, and extending the grouping rule to *then* or adversative chains.

## Tests

Under `CLAUDE.md`'s Assurance section: a test asserts the intended analysis, and a count only when the count is the claim. Tests whose expected trees change are rewritten to the single intended analysis for the same witness; a test that pins a grouping or level variant as a separate reading is retired, named with its reason.

## Gate

No face lost; per unit, the set of normal forms unchanged while tree count falls; full tests. Note: census delta (faces read and Readings before and after, lost faces none); the judgment-set line does not apply yet.
