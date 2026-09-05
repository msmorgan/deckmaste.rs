---
needs: [english-v2-adjunct-licence-removal]
---
# One scope device for right-peripheral and shared constituents

**B7 — the scope device.** DESIGN TICKET; the design is written and reviewed
before an implementer starts. Authority: rewrite ADR "Ruling: adjunct licences
removed; attachment misselection is a recorded class (2026-09-03)" and
"Amendment: one scope device, principles before packing (2026-09-04)" (Q1-Q4).
Design: `docs/memory/scratch/plan09-postmortem/b7-scope-device-design.md`
(gitignored).

## Scope (Q1)

One device covers right-peripheral Adjunct attachment **and** Coordination
bracketing. Two candidates that differ only in where a right-peripheral
Constituent attaches, or in how far a Constituent realized once beside a
Coordination scopes over it, are one ambiguity class: *scope of a
right-peripheral or shared Constituent*. Adjunct height, Postmodifier scope over
a Coordination, a shared Determiner, a shared Modifier, a shared head and a
shared preposition are that one class. Was: right-peripheral Adjunct attachment
only (2026-09-03 letter, superseded 2026-09-04 by coordinator ruling Q1).

## Ordered principles, then packing (Q2)

Declared English principles decide first; whatever survives is packed. Each is
one general rule over derivation shapes and declared features, stated once —
never a corpus count, never a word, never a dominance edge between named
constructions. In order:

1. A qualification Complement (scalar comparison, degree measure, power and
   toughness value, granted keyword line, quoted ability) predicates a property
   of a Nominal and has no verb-Adjunct site. Landed: the complement-kind site
   capability.
2. A frame-selected role preempts the Postmodifier derivation of the same
   preposition, at every preposition the Verb Frame declares and at every depth
   of the complement it governs. `english-v2-role-preemption-depth` (B6).
3. An identity claim outranks a lexeme claim over the same bytes. Landed: the
   Identity specificity tier.
4. A distributive measure attaches to the Predicate as its multiplier and has no
   Nominal-Postmodifier derivation. **Lands inside this ticket.**

Not admitted, therefore packed: the scope of a shared Determiner over a
Coordination. English is ambiguous there and Semantics chooses.

## Representation (Q3)

The ambiguous Constituent is hoisted to its highest admissible host and carries
the set of admissible lower hosts as a derived value on that host, like the zero
Determiner: a declared field kind, no new AST category, no bytes in any
Realization. Sites are named by role path, never by candidate ordinal or rule
index. Render is unaffected — every host renders the same bytes — leaf traversal
is unchanged, and the canonical construction path is the hoisted one. Consumers
(semantics workbench, xtask diagnostic display) read the slot. A first-class
packed AST node holding alternative subtrees is deferred to the re-layering
wayfinder.

## Census (Q4)

A packed candidate is one candidate: not a tie, never a STOP. `packed_units` is
a REPORT figure emitted beside the resolution counts, with its identities
listed, and it does not enter the `selected == unique + specificity_resolved +
exception_resolved` partition identity. A tie that is not a scope tie remains a
STOP.

## Sequencing (Q6)

B7 lands before `english-v2-attachment-class-declared` (R1) and
`english-v2-locative-coordination-arms` (R7) re-measure and land; both are
parked pending it. `english-v2-role-preemption-depth` (B6) lands as part of this
ticket or immediately after it. `english-v2-frame-complement-coordination` mints
the missing coherent Coordination for a Verb Frame's complement cluster, which
principle 2 makes a hard prerequisite of R1's re-measure. Nothing partial.

## Routed residues this ticket absorbs

2026-09-04, from the `english-v2-with-preposition` landing review: fourteen
identities where a free comitative Adjunct has no Adjunct site and attaches to
the object Nominal (suspend template, attack instrument), plus Vicious Rivalry
and seventeen siblings where Postmodifier scope over a Coordination is the same
choice one level down. Admissibility is no longer the defect once R1's declared
classes land; the survivors are this class.

2026-09-04, from the `english-v2-possessive-nominal-form-collapse` (R10) landing
STOP: the possessive Coordination arm. `possessed_reference` has no Coordination
arm, so `your creature and artifact` has no derivation; its sibling
`genitive_determiner_coordination_reference` is the shape the arm copies. The
arm admits `Destroy your creature and artifact.` and breaks Aquatic Alchemist //
Bubble Up (`eabbc2128de743c6555fa404e3102827062681275d373db6f3adbed32980abbb`),
whose two bracketings differ only in how far a shared Modifier and a shared head
scope over the Coordination. Both are English; the device packs them.

2026-09-04, from the `english-v2-attachment-class-declared` (R1) landing review:
153 distributive-measure misselections (principle 4 decides them), ~79 object
characteristics pulled into relative Predicates (packed), 2 prohibition-scope
inversions (packed), and 62 gains bought by an incoherent bracketing (principle
2 eliminates it; the coherent reading needs
`english-v2-frame-complement-coordination`).

2026-09-04: `english-v2-granted-ability-coordination` takes over the granted-line
residue for this item only.

Standard constraints apply. Gate scope: `cargo test --workspace` (the device is
a declaration-compiler field kind).

2026-09-04 (OPEN items ruled, user-verified): Move S verification = identity
erasure inside the affected region, tested in both failure directions; the
distributive axis lives on the Determiner vocabulary; joint realizability of two
mobiles is proven by a phase-1 test (fallback: pack per mobile, outermost first,
as an amendment); the collapse runs at the root first and moves per-node only on
a measured >~10% ns/B cost; the `AdmissibleSites` slot is always present; the
shared-operator case is packed, not a fifth principle. Design:
docs/memory/scratch/plan09-postmortem/b7-scope-device-design.md (gitignored);
phases per its §G.2 — phases 1 and 3 never share a round.

## Landing record

### Phase 1

Measured after `kata refresh` on change `uqztnnqkvzkr`, with coverage-lock
`covered` count 17,601.

**PROVE.** The declaration compiler now accepts `mobile` on a construction
role, retains that declaration through its semantic plan, and validates that
the role is at the right edge or adjacent to a `seq` role in every form. A
declaration outside that topology fails at load time with a diagnostic naming
the role. Each carrying element has one always-present, sealed
`AdmissibleSites`; `AttachmentSitePath` is an ordered sequence of `Role` and
`Conjunct` steps relative to that element. Constructors derive the empty slot
without accepting a build value.

Emitter and compiled-consumer witnesses prove that this derived field adds no
`RulePosition` or specificity tier, is not visited as a leaf, and renders no
bytes. The compiled-consumer roundtrip renders and parses the same byte-exact
one-word span with one lexical claim and one leaf visit. Its nested fixture has
two mobile roles over that span and admits the candidate with both Constituents
at their highest hosts, settling OPEN-3's phase-1 obligation without invoking
the fallback.

No silent loss: coverage is 17,601 -> 17,601, with no newly covered or no-longer
covered identities. `DECKMASTE_COVERAGE_LOCK=report` emitted no delta rows
(lock +0/-0), and `english-v2-coverage.lock` remained byte-identical at SHA-256
`766eca211bace1feebdd8a1c2c53c35655c36159c3f4df7dbe18a0e6d29fdcac`.
The structural laws are exact: roundtrip 17,601/17,601 clean with 0 mismatches;
ownership has 0 failures, gaps, overlaps, synthetic claims, or provenance-plan
mismatches; construction traversal is 750,925 nodes / 750,925 visits with 0
failure units; leaf traversal is 263,397 expected / 263,397 visited with 0
failure units; unresolved ties and internal failures are both 0. Forbidden
word-naming licensing checkers are 0, and the environment loaded without an
error.

**DISCLOSE.** The production grammar is unchanged: `constructions.rs` and
`core_verbs.ron` are byte-identical to the refreshed parent, and construction
count is 387 -> 387. Parent and tree `ambiguity --json` reports are byte-identical
at SHA-256
`25960e93f78e4e572baa82c04e721c4674c33b3c7993d8a961249e0723ea2f85`
across all 32,641 units. The selection census is therefore unchanged: 13,759
unique, 3,842 specificity-resolved, 0 exception-resolved, 15,040 parse failures,
and 0 unresolved ties. Newly covered identities: none. Permitted licensing
checkers: 20.

Assurance counts: restored 0; re-spelled 0; ignored 0; added 8 (seven Rust test
entries and one compile-fail fixture); removed 0. Deviations and additions:
none beyond the ticket's required positive, negative, structural-law,
roundtrip, and joint-realizability witnesses. STOP: none. Citations changed:
none. Glossary gaps carried from the design: Scope, Attachment, Head,
Premodifier, Peripheral, Bracketing, Mobility.

**REPORT.** Lock `covered`: 17,601. Construction count: 387. Licensed
vocabulary/lexicon homographs (2): `AttributiveAdjective::Untap` beside the
declaration keyword action `Untap`; `TargetingMarker::Target` beside
`CommonNoun::Target`. Form-literal/vocabulary overlaps (9): `additional` in
`additional_cost`; `to` in `up_to_quantifying_determiner`; `the` and `next` in
`definite_next_mass_quantity_reference`; `to` in
`scalar_less_than_or_equal_to`; `the` in `number_of_scalar_value`; `the` in
`greatest_scalar_value`; `other` in `other_than_qualified_reference`; and `the`
in `positional_partitive`.

Coverage performance advisory: 178 s against the 16.26 s quiet-host ceiling,
186,843 ns/B thread CPU, 8 workers, and 1/5/15-minute host load 33/40/32. The
sandbox cannot observe sibling-process count; the elevated load is reported
for reviewer contention stamping and is not fitted to. Wall clock: start
2026-09-04 20:13:04 PDT; end 2026-09-04 21:07:10 PDT.
