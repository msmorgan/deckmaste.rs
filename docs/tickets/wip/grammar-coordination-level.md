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

## Gate implementation — mpxyuoyn

The gate-only commit changes no grammar or lexicon declaration. Census schema 11
records `normal_forms`, `normal_form_version` (8) and
`normal_form_source_sha256`; `diff --check normal-forms` compares the combined
nominal and coordination equivalence. Both reports must match the running
version and compiled source hash. The source lock covers the parser, nominal
adapter and coordination adapter. Complete enumeration, fixed unit boundaries
and surrounding structure, no lost face, equal per-unit sets and nonincreasing
per-unit tree counts are required. Schemas 9 and 10 remain usable for raw digest
comparisons only.

Grouping uses exact And, Or and AndOr lexeme IDs, keeps every ordered link and
its punctuation and full lexical payload, and never reassociates Then or But.
Bare Imperative/BarePredicate/SecondaryVerbPhrase chains normalize to Clause
chains without crossing any other wrapper. Then and But retain grouping while
their bare-imperative level twins normalize. The asyndetic premodifier use of
SerialCoordination is explicitly outside this adapter (Waste Not exercised the
shared schema name); focused adjuncts retain their exceptional helper name.

The raw baseline and combined baseline are in
`target/english-v3/coordination/{raw-baseline,combined-baseline}.json`.
Full raw digest identity passed on all 32,828 faces: 30,794 read and 373,659
Readings before and after, zero issues. The combined baseline records 354,012
normal forms over 75,412 units: 19,647 redundant trees under this equivalence.
This is the instrument's measurement, not a grammar reduction yet.
Bring to Light unit 0 has 10,200 trees / 6,800 normal forms; Slimefoot's Survey
unit 0 has 10,368 / 7,776. The census wall times were 582.24 s and 672.53 s
(four workers, other builds/tests running; not an isolated performance comparison).

Twelve tests were added: association and member order; imperative level plus
grouping; Then level without reassociation; mixed markers/correlatives/categories;
serial punctuation and helper identities; non-distribution of subjects, modals,
quantifiers, dependents and gaps; exact lexical form/variant/capitalization;
attachment and barrier scope during nominal reordering; changed-shape rejection;
real imperative lists with one normal form; attachment during coordination
flattening; and asyndetic premodifier preservation. A thirteenth test injects a
removed grouping twin (pass) and a lost attachment class (failure) into the
actual census diff gate. Existing incompatible-version/hash, incomplete-census,
unit-boundary and surrounding-structure checks now cover the combined fields.

Two older synthetic tests were re-spelled with lexical coordinator Words and
proper series helpers: `layer_adapter_preserves_old_records_and_barrier_scope`
and `synthetic_trees_preserve_attachment_lexeme_form_gap_and_barrier_scope`.
Their equality/inequality assertions remain; no test was retired. Full
`cargo test -p xtask`: 499 passed, one pre-existing ignored live-corpus macro
schema census test. Clippy with `--tests -- -D warnings` passed. Citation audit
had no changed citation sites; citation checks found no stale or noncompliant
citations.

Remaining: impose the bare-imperative Clause convention and canonical grouping
in the grammar; update intended-analysis grammar tests; run the combined gate,
full tests and clippy for each touched crate; record the final census delta.
Attachment licensing, determiner/modifier scope and finite-plus-imperative
hybrids remain outside this ticket.
