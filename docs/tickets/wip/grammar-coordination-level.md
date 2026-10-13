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
- Same-coordinator grouping remains in the gate equivalence. The owner subsequently limited grammar canonicalisation to an existing structural restriction without another policing feature; the measured 442 remaining twins are deliberately retained (see closure below).
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

The combined baseline is in
`target/english-v3/coordination/combined-baseline.json`. The raw predecessor
census was deleted after the raw gate passed; `raw-gate.log` retains its result.
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

## Bare imperative level — wllxrvoo

OrdinaryPredicateCoordination defaults to No. It is local on SecondaryVerbPhrase,
head-propagated on BarePredicate, and set to Yes only for binary and serial
SecondaryVerbPhrase coordination. Imperative requires No. Other verb-phrase
constructions reset it locally, so coordination inside a modal, auxiliary or
adjunct remains available; correlatives remain unchanged. This is six authored
grammar lines, with no lexicon, compiler capability, constructor, field or form
change. SupplementAnchorImperative is outside this change.

The full `diff --check normal-forms` gate passed: 32,828 faces checked, 30,794
read before and after, no lost face or changed unit set. Readings fell from
373,659 to 354,454 (19,205 removed); 1,981 faces decreased and none increased.
The 75,412 units retain exactly their 354,012 normal forms. The retained census
pair is `combined-baseline.json` and `after.json`; `imperative-gate.log` records
the result. The remaining 442 redundant trees are grouping twins, deliberately retained under the owner's subsequent size constraint.

| Unit (zero-based index) | Before | After | Normal forms, unchanged |
| --- | ---: | ---: | ---: |
| Slimefoot's Survey 0 | 10,368 | 7,776 | 7,776 |
| Bring to Light 0 | 10,200 | 6,800 | 6,800 |
| Beneath the Sands 0 | 12 | 6 | 6 |
| Deliberate 0 | 2 | 1 | 1 |
| Deadly Dispute 1 | 2 | 1 | 1 |

Added `bare_instruction_pairs_are_independently_constructible_coordinated_clauses`:
And and Then pairs parse as exactly the independently built coordinated Clauses,
which admit and realize; the old imperative-over-coordinated-predicates value
rejects admission and realization, while standalone predicate coordination remains.
Four tests were rewritten for their existing intended witnesses:

- `beneath_the_sands_retains_the_final_link_in_a_flat_serial_list`: the final-link
  constituent is ClauseSeries rather than SecondaryPredicateSeries; spans and
  member assertions remain.
- `cryptic_annelid_retains_marker_readings_without_an_unmarked_adverb_conjunct`:
  remove the incidental count 11 (now 5); require readings and retain every
  per-Reading coordinator/adverb distinction.
- `selected_and_comparative_controls_gain_no_as_adjunct`: remove incidental
  control counts (Counterbore 22 to 11; Aven Shrine 9 and Clone 5 unchanged);
  require readings and retain all unwanted-as-adjunct exclusions.
- `open_the_gates_preserves_coordination_without_card_as_a_coordinated_premodifier`:
  remove the incidental count 20 (now 10); require readings and retain every
  exclusion of Card as a noun premodifier.

No whole test was retired. Full English tests: 1,166 passed, one pre-existing
ignored Mana-symbol subject test with its named blocker. Full xtask tests:
499 passed, one pre-existing ignored live-corpus macro census test. Clippy for
both crates with `--tests -- -D warnings` passed. README and glossary describe
the gate and local predicate-coordination property.

## Closure and structural comparison — qvumruql

The owner narrowed completion after the prefix: remove the remaining 442 grouping
twins only through existing structure without another threaded policing feature;
compare structural alternatives to the imperative flag and retain it if smaller.
Both checks are complete. No further grammar or normaliser change was made.

**Grouping: deliberately retained.** Coordination's schema binds both `left` and
`right` to the same member Category; the generator's `of` chooses that member
Category for both sides, not a left-only subset. A serial tail changes `right`
to a helper, but its members are still the same Category. The current summaries
have no exact root-coordinator identity. `ClauseCoordinationHost` distinguishes
simple/coordinated, and `HeadCoordination` distinguishes no/yes; neither tells
And from Or, AndOr, Then or But. `CoordinationKind` is also too coarse.

Dropping binary forms loses two-member joins; dropping serial forms loses their
punctuation. Requiring an uncoordinated left member would also ban mixed-marker
bracketings that this gate protects, for example `(A or B) and C`. Existing
NominalCore is not an uncoordinated subset (coordination itself produces it).
The same issue applies to the other member Categories. Marker-specific category
families could encode this state structurally, but they are new parallel grammar
layers with repeated producer routes/lifts, not a restriction supplied by the
existing structure. The practical small change would need exact-coordinator
state in summaries, hence another policing feature. It is not worth 442 trees
and is forbidden by the revised brief. The gate's grouping normalisation stays;
no tests now claim that its remaining twins are different analyses.

**Imperative flag: retained after comparison.** Counts below are of authored
source; alternative line ranges are design estimates, not implemented changes.
The inventory is seven explicit SecondaryVerbPhrase constructions and 31
schema-instance result rows (38 noncoordination producer sites). Its interface
has nine entries without the new flag; BarePredicate has three without it.

| Design | Categories / lifts added | Authored cost and consequence |
| --- | --- | --- |
| Retained flag | 0 / 0 | Six lines: feature, two interface entries, one context override, one requirement and its comment. Zero handwritten `export` equations. Local on SecondaryVerbPhrase; head on BarePredicate. Generated ordinary/correlative series summaries also carry the feature. |
| Noncoordinated predicate layer, Imperative directly accepts it | 1 / 1 | Roughly 15–25 added lines for a nine-entry interface, lift, split correlative/result declarations and revised imperative form/Plain requirement, plus retargeting 38 producer sites. Every noncoordinated predicate gets a lift when used as SecondaryVerbPhrase. Removes the BarePredicate wrapper under Imperative, requiring a changed Reading shape and gate adapter. |
| Layer preserving the BarePredicate wrapper for imperative use | 2 / 2 | Roughly 25–40 added lines plus the 38 retargeted sites: separate restricted and general bare routes are needed because auxiliary complements must still accept coordinated predicates. One lift remains on every noncoordinated predicate. |
| Separate result Category for binary/serial predicate coordination | 1 / at least 1 | Roughly 15–25 lines for the nine-entry result interface, block split and inclusion route, before changing consumers. Lifting that result back into SecondaryVerbPhrase lets the old Imperative accept it again, so this alone does not impose the restriction. Making SecondaryVerbPhrase the inclusive layer reduces to the preceding designs. |
| Parallel restricted producers, no lifts | At least 1 / 0 | Duplicate the 38 noncoordination producer routes (31 instance rows and seven construction bodies), plus correlative routes/interface and an imperative-specific bare route. Estimated well over 100 authored lines; internal heads must still accept the broad Category so wrapper scope is preserved. This repeats the grammar to avoid one summary bit. |

The layer with a ubiquitous lift is expressly excluded by the owner. The
no-lift alternative is substantially larger and less clear. The existing flag
is a root-construction distinction, reset by wrappers, not accumulated history;
it remains a policing feature, but the smallest honest expression available
without a new compiler capability. No alternative was implemented or presented
as gate-proven.

The README records both decisions; existing glossary entries for Reading Normal
Form and Ordinary Predicate Coordination remain accurate. This closing commit
changes documentation only, so the full raw/normal-form gate results and full
English/xtask tests and clippy above remain applicable. No new tests were added,
rewritten or retired at closure. No lexicon change was made anywhere in the ticket.

The ticket is complete under the revised scope, ready for review, not integrated.
Attachment licensing, shared determiner/modifier/operator scope,
finite-plus-imperative hybrids and Then/adversative reassociation were not changed.
The existing Erode shared-subject tests were left unchanged; their scope questions
belong later. The 442 grouping twins are an explicit cost decision, not an
unfinished implementation obligation.

Census delta (wllxrvoo; unchanged at qvumruql): 30,794 → 30,794 faces read; 373,659 → 354,454 Readings; lost faces none.
Judgment set: does not apply yet.
