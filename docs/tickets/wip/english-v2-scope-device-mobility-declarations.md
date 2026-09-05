---
needs: [english-v2-underspecified-adjunct-attachment]
---
# The scope device, phase 2: declare which roles are scope-mobile

**B7 phase 2.** Phase 1 taught the declaration compiler the `mobile` role
annotation and the derived `AdmissibleSites` slot. This ticket uses it: the
seven role families of the design's §A.2 get their `mobile` annotations in
`crates/deckmaste_english_v2/src/constructions.rs`. Nothing is packed yet —
every slot still renders empty — so the corpus must not move at all.

Design: `docs/memory/scratch/plan09-postmortem/b7-scope-device-design.md`
(gitignored), §A.2 (the families and the not-mobile list) and §G.2 phase 2.
The Q1-Q6 rulings, the OPEN rulings and the routed residues live on
`english-v2-underspecified-adjunct-attachment` (phase 1) as inherited context.

## Letter

Annotate as `mobile`, by linguistic function and never by a list of named
constructions:

- the Adjunct role of a Predicate-Adjunct host, and of a relative clause whose
  Predicate takes one;
- the Postmodifier role of a postmodified-reference production, prepositional
  and relational alike;
- the Determiner, Predeterminer and possessor role of a Nominal that dominates
  a Coordination;
- the attributive Modifier (premodifier) role of a Nominal that dominates a
  Coordination;
- the head role of a Nominal that dominates a Coordination;
- the preposition role of a Prepositional Phrase whose Complement is or
  contains a Coordination;
- the modal or negation role of a Clause whose Predicate is a Coordination.

Leave un-annotated, per §A.2: any Complement; any role a Verb Frame declares
(principle (ii) removes the competing derivation before packing runs); every
literal, Coordinator and vocabulary position — words do not move, Constituents
do.

≈40 annotations, ~60 lines (§G.2).

## Known obstacle, carried from the phase-1 landing review (2026-09-04)

Phase 1 implements §A.2's placement rule verbatim: a mobile role must be at its
form's right edge **or linearly adjacent to a `seq` role**. That rule rejects
part of §A.2's own family list. In `your first instant or sorcery spell` the
Determiner is not linearly adjacent to the Coordination — the attributive
Modifier intervenes — so the shared-Determiner family fails to load as written.
The same forms put two or three mobile roles on one element (Determiner,
Modifier, head), and phase 1 emits **one** `admissible_sites` slot per element,
which cannot say which Constituent a path belongs to.

Both are ruling questions, not implementer calls: widen §A.2's adjacency rule
(separated from the `seq` role only by other mobile roles?), and decide whether
the slot is per element or per mobile role (§C.3's "each mobile's slot" reads
per-role). **STOP and report** on reaching either; do not pick one.

## Acceptance

- Every annotation answers "which English fact makes this Constituent's host
  underdetermined?", never a witness count (§G.4, attestation is provenance).
- The corpus does not move: coverage, the selection census, the ambiguity JSON
  and the roundtrip are byte-identical to the claimed baseline, because no slot
  is populated until phase 3.
- Zero mobile roles land on a Verb-Frame-declared role (§C.4's invariant, in
  its declaration-time form).

## Fences (§G.4)

- No `require`, `checked by`, comment or literal naming a lexeme, Construction,
  verb, noun, preposition or card. A permitted guard reads a declared feature or
  a declared role property.
- No `SELECTION_EXCEPTIONS` entry, no dominance edge between named
  Constructions, no new specificity weight or tier.
- No new AST Category, and no field holding an alternative subtree.
- `unresolved_ties` stays 0 and keeps its meaning.
- No `run_in_background` on a gate; report positive artifacts.

Standard constraints apply. Gate scope: `cargo test --workspace`.

## Baseline

Measured on `lpvvplmyynul`, phase 1's base — re-measure at claim. Lock
`covered` 17,601; constructions 387; 32,641 units; census 13,759 unique /
3,842 specificity-resolved / 0 exception-resolved / 15,040 parse failures / 0
unresolved ties.

## Glossary gaps

Terms the design needs that `docs/contexts/oracle-english/CONTEXT.md` does not
define: Scope, Attachment, Head, Premodifier, Peripheral, Bracketing, Mobility.
Listed as gaps; none is coined into the tracked glossary by this ticket.

## Landing record

Measured on refreshed change `ptllomznnwyr`, based on claim change
`smpnwwvuvuzk`, with coverage-lock `covered` count 17,601. `kata refresh`
first reconciled the concurrent role-preemption semantic-plan changes with the
scope-sibling metadata and then reported a no-op on the resolved tree. All
gates and figures below are from that refreshed tree.

The 2026-09-04 coordinator ruling resolved the earlier STOP: a mobile role may
be at a form edge, immediately left of its declared scope sibling, or adjacent
to a sequence role. `AllPredeterminedNominal.det` therefore names `nominal` as
its scope sibling and remains in the existing `predeterminer det nominal` form;
no Construction was reshaped. The declaration syntax accepts
`mobile(<scope-sibling>)`, retains that role in the semantic plan, and rejects
an unknown sibling. Placement validation accepts both edges, immediate
left-adjacency to the named scope sibling, and either-side sequence adjacency;
a separated role fails with a diagnostic naming the role and form.

All 39 roles in §A.2's seven families are declared mobile: 12 Adjunct roles, 5
Postmodifier roles, 7 Determiner/Predeterminer/possessor roles, 2 attributive
Modifier roles, 9 nominal head roles, 1 preposition role, and 3 modal/negation
roles. Complements, Verb-Frame-declared roles, form literals, Coordinators, and
vocabulary positions remain unannotated. Nothing reads or populates an
`admissible_sites` slot in this phase.

**PROVE.** Coverage is 17,601 -> 17,601. Report mode printed no lock-delta rows
(lock +0/-0), and `english-v2-coverage.lock` stayed byte-identical at SHA-256
`766eca211bace1feebdd8a1c2c53c35655c36159c3f4df7dbe18a0e6d29fdcac`.
No identity became covered or stopped being covered. Parent and feature
`ambiguity --json --require-resolved --workers 8` reports are byte-identical
across every one of the 32,641 units (`cmp` exit 0), including every candidate
Construction path and selected ordinal; both have SHA-256
`5f996af6cb688bc7d8a866672f20cca343c74a2f831ae55fbbc89867f710a3ce`.
Thus selection and its per-unit traversal inputs are unchanged.

The structural laws remain exact: roundtrip is 17,601/17,601 clean with 0
mismatches; ownership has 0 failure units, gaps, overlaps, synthetic claims, or
provenance-plan mismatches; Construction traversal is 750,914 nodes / 750,914
visits with 0 failure units; leaf traversal is 263,397 expected / 263,397
visited with 0 failure units. Unresolved ties and internal failures are both 0.
Forbidden word-naming licensing checkers are 0, permitted licensing checkers
are 20, and the environment loaded without an error.

**DISCLOSE.** Construction count is 387 -> 387. The selection census is
byte-identical to the refreshed parent: 13,784 unique, 3,817
specificity-resolved, 0 exception-resolved, 15,040 parse failures, 0 unresolved
ties, and 0 internal failures. Newly covered identities: none. Stopped-covered
identities: none. No specificity share changed.

Assurance counts: restored 0; re-spelled 3 (the parser annotation witness, the
validator placement-topology witness, and the trybuild placement fixture);
ignored with blockers 0; added 0; removed 0. Existing direct-AST test fixtures
were routed through generated checked constructors, or supplied the still-empty
derived slot internally, solely to keep them compiling after their elements
became mobile.

Deviations and additions: none beyond the ticket and coordinator ruling. The
earlier `AllPredeterminedNominal.det` STOP is resolved by that ruling; no fresh
STOP fired. The per-mobile admissible-site slot shape and joint-realizability
witness remain routed to phase 3, as amended in the design. No collapse code,
principle-(iv) work, AST Category, census gate, named-Construction dominance,
selection exception, word-naming guard, or per-preposition/per-Construction
switch was added. Citations changed: none. Glossary gaps: Scope, Attachment,
Head, Premodifier, Peripheral, Bracketing, Mobility.

**REPORT.** Lock `covered`: 17,601. Construction count: 387. Licensed
vocabulary/lexicon homographs (2): `AttributiveAdjective::Untap` beside the
declaration keyword action `Untap`; `TargetingMarker::Target` beside
`CommonNoun::Target`. Form-literal/vocabulary overlaps (9): `additional` in
`additional_cost`; `to` in `up_to_quantifying_determiner`; `the` and `next` in
`definite_next_mass_quantity_reference`; `to` in
`scalar_less_than_or_equal_to`; `the` in `number_of_scalar_value`; `the` in
`greatest_scalar_value`; `other` in `other_than_qualified_reference`; and `the`
in `positional_partitive`.

Performance advisory, all with 8 workers: coverage 100 s at 116,526 ns/B,
host load 7.09/8.68/8.83; ambiguity 107 s at 118,362 ns/B, host load
4.62/7.11/8.20; roundtrip 95 s at 112,985 ns/B, host load 4.13/5.95/7.49.
Each exceeds the 16.26 s quiet-host ceiling under load and is provenance, never
a gate target. The sandbox cannot observe sibling-process count; each
measurement used one foreground gate process. Wall clock: start 2026-09-04
23:23:25 PDT; end 2026-09-04 23:58:04 PDT.

Positive gates on the refreshed tree: `cargo fmt --all` exited 0; strict
all-target Clippy for `deckmaste_construction_core`, `deckmaste_construction`,
and `deckmaste_english_v2` finished without diagnostics; `cargo test
--workspace` exited 0 with every completed `test result: ok` line at 0 failed,
including `mobile_role_not_edge.rs ... ok`, `compile_fail_fixtures`, and
`mobile_roles_add_only_an_empty_derived_slot_and_both_attachment_heights_agree`;
coverage reported 17,601 selected and covered with every failure counter 0;
ambiguity required resolution and reported 0 ties; roundtrip reported 17,601
clean and 0 mismatched. Citation gates were not required because no citation
changed.
