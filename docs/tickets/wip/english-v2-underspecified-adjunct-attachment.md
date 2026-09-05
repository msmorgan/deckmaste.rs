---
needs: [english-v2-adjunct-licence-removal]
---
# The scope device, phase 1: the `mobile` field kind

**B7 phase 1.** The declaration compiler learns a `mobile` role annotation and
emits the derived attachment slot the later phases populate. Authority: rewrite
ADR "Ruling: adjunct licences removed; attachment misselection is a recorded
class (2026-09-03)" and "Amendment: one scope device, principles before packing
(2026-09-04)" (Q1-Q4). Design:
`docs/memory/scratch/plan09-postmortem/b7-scope-device-design.md` (gitignored),
§A.2, §C.1, §C.2 and §G.2 phase 1; fences §G.4.

Was one ticket for the whole device. Split 2026-09-04 by coordinator ruling:
`kata integrate` completes a ticket, and §G.2 forbids phases 1 and 3 sharing a
round, so each phase is its own ticket. The Q1-Q6 rulings, the OPEN rulings and
the routed residues below stay here as the context every later phase inherits;
they are not this ticket's letter.

## This ticket's letter

- `mobile` parses as a role annotation that leaves the role's value kind alone,
  survives into the semantic plan, and reaches the emitter.
- A construction that declares a mobile role gains one always-present, sealed
  `AdmissibleSites` field, derived by the constructor and never a build value,
  plus `AttachmentSitePath` — an ordered sequence of `Role` and `Conjunct` steps
  relative to the node carrying the slot. No candidate ordinal, no rule index,
  no `RulePosition` index (§C.2).
- The compiler rejects a mobile role that is neither at its form's right edge
  nor linearly adjacent to a `seq` role, with a diagnostic naming the role
  (§A.2).
- Proof by test that the slot contributes no `RulePosition`, no specificity
  tier, no leaf visit and no rendered byte, and that the roundtrip is
  unaffected.
- No mobility is declared in `constructions.rs` and no collapse code is
  written: those are phases 2 and 3.

Size: ~500-700 lines with tests (§G.2). Gate scope: `cargo test --workspace`
(the diff reaches `construction_core/src/emit/`).

## The remaining phases

| ticket | phase | design |
|---|---|---|
| `english-v2-scope-device-mobility-declarations` | 2 — the ~40 declarations | §A.2 |
| `english-v2-scope-device-collapse` | 3 — bucket-and-verify, representative, slot population | §A.3, §A.4, §C.3-§C.5 |
| `english-v2-scope-device-distributive-measure` | 4 — principle (iv) | §B (iv) |
| `english-v2-scope-device-census` | 5 — `packed_units`, probe and JSON | §E |

`english-v2-attachment-class-declared` (R1) and
`english-v2-locative-coordination-arms` (R7) carry a `needs:` edge on *this*
slug. Both sit in parked workspaces, so the edge is not rewritten here: their
real blocker is `english-v2-scope-device-census`, the last phase, and their
claimants re-point the edge when they resume. Recorded here, in the design doc
and in the phase-5 ticket.

## Inherited context: Scope (Q1)

One device covers right-peripheral Adjunct attachment **and** Coordination
bracketing. Two candidates that differ only in where a right-peripheral
Constituent attaches, or in how far a Constituent realized once beside a
Coordination scopes over it, are one ambiguity class: *scope of a
right-peripheral or shared Constituent*. Adjunct height, Postmodifier scope over
a Coordination, a shared Determiner, a shared Modifier, a shared head and a
shared preposition are that one class. Was: right-peripheral Adjunct attachment
only (2026-09-03 letter, superseded 2026-09-04 by coordinator ruling Q1).

## Inherited context: ordered principles, then packing (Q2)

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
   Nominal-Postmodifier derivation. `english-v2-scope-device-distributive-measure`
   (phase 4).

Not admitted, therefore packed: the scope of a shared Determiner over a
Coordination. English is ambiguous there and Semantics chooses.

## Inherited context: representation (Q3)

The ambiguous Constituent is hoisted to its highest admissible host and carries
the set of admissible lower hosts as a derived value on that host, like the zero
Determiner: a declared field kind, no new AST category, no bytes in any
Realization. Sites are named by role path, never by candidate ordinal or rule
index. Render is unaffected — every host renders the same bytes — leaf traversal
is unchanged, and the canonical construction path is the hoisted one. Consumers
(semantics workbench, xtask diagnostic display) read the slot. A first-class
packed AST node holding alternative subtrees is deferred to the re-layering
wayfinder.

## Inherited context: census (Q4)

A packed candidate is one candidate: not a tie, never a STOP. `packed_units` is
a REPORT figure emitted beside the resolution counts, with its identities
listed, and it does not enter the `selected == unique + specificity_resolved +
exception_resolved` partition identity. A tie that is not a scope tie remains a
STOP.

## Inherited context: sequencing (Q6)

The device — every phase through `english-v2-scope-device-census` — lands before
`english-v2-attachment-class-declared` (R1) and
`english-v2-locative-coordination-arms` (R7) re-measure and land; both are
parked pending it. `english-v2-role-preemption-depth` (B6) lands as part of this
ticket or immediately after it. `english-v2-frame-complement-coordination` mints
the missing coherent Coordination for a Verb Frame's complement cluster, which
principle 2 makes a hard prerequisite of R1's re-measure. Nothing partial.

## Inherited context: routed residues the device absorbs

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
two mobile roles over that span, both slots sealed and empty, and (after the
review correction below) a second host that dominates the first, so the same
bytes derive with the mobile Constituent at either attachment height, with
identical lexical claims and identical leaf traversal at both. That is Move A's
render-invariance premise (§C.6) demonstrated end to end. It is **not** OPEN-3
discharged: see the review corrections.

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
entries and one compile-fail fixture); removed 0 — verified against the diff.
Two existing `parse.rs` pattern assertions gained a `mobile: false` binding;
that is a mechanical widening, not a weakening. Deviations and additions: the
`mobile` annotation is an orthogonal flag on `Field` rather than a
`FieldKind` variant, so a mobile role keeps its own value kind (§G.2 names the
`zeroable` field kind only as the closest footprint, not as the representation);
otherwise none beyond the ticket's required positive, negative, structural-law,
roundtrip and joint-realizability witnesses. STOP: none raised by the
implementer; the review raises three, recorded below. Citations changed: none.
Glossary gaps carried from the design: Scope, Attachment, Head, Premodifier,
Peripheral, Bracketing, Mobility.

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

### Review corrections (Opus landing review, 2026-09-04)

Contention stamp for the performance advisory above: 3 concurrent executors and
3 concurrent reviews on the host, 1/5-minute load 33-40. The implementer's
sandbox cannot count sibling processes, so its "cannot observe" note stands
with this stamp supplied by the reviewer. The 178 s coverage wall time and the
186,843 ns/B thread-CPU line are contended figures against the 16.26 s
quiet-host ceiling; nothing in this landing is fitted to them.

**HIGH — OPEN-3 was reported as settled and is not.** The record claimed the
nested fixture "admits the candidate with both Constituents at their highest
hosts, settling OPEN-3's phase-1 obligation without invoking the fallback". In
the fixture as delivered each mobile role had exactly one host in the grammar,
so "highest host" was vacuous and the assertion was that the only derivation
parses. OPEN-3's ruling is coordinator-level and explicitly "never an
implementer call", so declaring it discharged on that evidence is a ruling
contradiction resolved without a STOP. **Fixed** two ways: the compiled-consumer
fixture gained `direct_mobile`, a dominating host for the same mobile
Constituent, so the witness now asserts that both attachment heights derive the
same bytes with identical claims and identical leaf traversal (2 accepted roots,
one per host) — the strongest statement phase 1 can make, since phase 1 computes
no admissible sites; and the claim above is corrected. The general question, and
its "pack per mobile, outermost first" fallback, are routed to
`english-v2-scope-device-collapse` (phase 3) as an owned, blocking obligation,
and recorded in the design doc.

**MEDIUM — slot arity contradicts §C.3 when an element has several mobile
roles.** One `admissible_sites` field is emitted per element, so with two or
three mobile roles on one element (§A.2's shared Determiner, attributive
Modifier and head over a Coordination) a path in the slot cannot say which
Constituent it belongs to; §C.3 reads "each mobile's slot". Not fixed here:
nothing reads the slot until phase 3 populates it, so the shape can still change
at zero cost. Routed to `english-v2-scope-device-collapse` as a decide-before-
populating obligation and recorded in the design doc.

**MEDIUM — §A.2's placement rule, implemented verbatim, rejects part of §A.2's
own family list.** `your first instant or sorcery spell` separates the shared
Determiner from the Coordination by the attributive Modifier, so it is neither
at the right edge nor linearly adjacent to the `seq` role and will not load. The
implementation is faithful to the design; the design's rule is the problem.
Routed to `english-v2-scope-device-mobility-declarations` (phase 2) as a STOP it
must raise rather than resolve, and recorded in the design doc.

**LOW — the three attachment metadata type names were reserved conditionally.**
`AdmissibleSites`, `AttachmentSitePath` and `AttachmentSiteStep` were registered
in the declaration namespace only when some construction declared a mobile role,
so the reserved namespace depended on the grammar. `BuildRejection` and
`BuildViolation` — equally conditional in emission — sit in
`FIXED_RUNTIME_TYPE_NAMES` and are reserved always. **Fixed**: the three moved
into `FIXED_RUNTIME_TYPE_NAMES` and the conditional block is gone. The reserved
field name gained a const, `ADMISSIBLE_SITES_FIELD`, so validation no longer
spells it as a bare literal.

Everything else verified clean: no guard names a lexeme, Construction, verb,
noun, preposition or card (the placement diagnostic names the author's declared
role, which §G.4 permits); no `SELECTION_EXCEPTIONS` entry, dominance edge,
specificity weight or tier; no new AST Category and no field holding an
alternative subtree; sites are named by `Role(name)` and `Conjunct(name,
ordinal)` — a `seq`-local linear position, never a candidate ordinal, rule index
or `RulePosition` index; the compile-fail fixture is a real trybuild case
(`tests/compile_fail/*.rs`); no phase-2-or-later content leaked —
`constructions.rs`, `core_verbs.ron` and `materialize.rs` are untouched; no
process language, ticket id or essay comment in the diff.

### Ticket split (coordinator ruling, 2026-09-04)

This ticket's letter was narrowed to phase 1 and the remaining four phases were
minted as `english-v2-scope-device-mobility-declarations`,
`english-v2-scope-device-collapse`,
`english-v2-scope-device-distributive-measure` and
`english-v2-scope-device-census`. The `needs:` edges on
`english-v2-attachment-class-declared` and
`english-v2-locative-coordination-arms` name this slug and belong on
`english-v2-scope-device-census`; both tickets sit in parked workspaces, so the
re-pointing is recorded in the design doc and in the phase-5 ticket for their
claimants to apply rather than edited into files another workspace holds.
