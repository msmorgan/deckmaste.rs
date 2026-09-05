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

### STOP — a named Determiner role is mid-form

Measured on change `voxolnnllusv` after `kata refresh` moved the claim base to
the landed role-preemption change `slqupuzyyvmp`, with coverage-lock `covered`
count 17,601. All gates and figures below are from that refreshed tree.

**STOP.** Section A.2 names the Determiner role of a Nominal that dominates a
Coordination as mobile. `AllPredeterminedNominal.det` is such a role: its
`nominal: Nominal` admits the coordination-valued Nominal productions, and its
form is `all_predetermined_nominal = predeterminer det nominal`. The `det` role
is the middle of that form and is not adjacent to a `seq` role. It therefore
fails the amended placement rule, which permits a Move S shared Constituent at
the left edge of its own Construction's form or next to a `seq` role. The
ticket says to stop on exactly this condition and forbids reshaping the
Construction to make the annotation fit, so no mobility declarations were
made. A coordinator ruling is needed on how this Determiner role is represented
under the edge rule before phase 2 can resume.

**Safe committed subset.** The independently pinned validation amendment is
complete: a mobile role at either edge now validates, while a mid-form mobile
role still fails with a diagnostic naming the role and form. The validator unit
witness now includes a left-edge role. The trybuild fixture was re-spelled from
the now-valid left-edge case to a mid-form case and pins
`mobile_middle` / `invalid` in the diagnostic. No declaration, emitted-slot,
collapse, principle-(iv), AST-Category, selection, or census code changed.

**PROVE.** Coverage is 17,601 -> 17,601. Report mode printed no lock-delta rows
(lock +0/-0), and `english-v2-coverage.lock` stayed byte-identical at SHA-256
`766eca211bace1feebdd8a1c2c53c35655c36159c3f4df7dbe18a0e6d29fdcac`.
No identity became covered or stopped being covered. The structural laws remain
exact: roundtrip 17,601/17,601 clean with 0 mismatches; ownership has 0 failure
units, gaps, overlaps, synthetic claims, or provenance-plan mismatches;
Construction traversal is 750,925 nodes / 750,925 visits with 0 failure units;
leaf traversal is 263,397 expected / 263,397 visited with 0 failure units.
Unresolved ties and internal failures are both 0. Forbidden word-naming
licensing checkers are 0, permitted licensing checkers are 20, and the
environment loaded without an error.

**DISCLOSE.** The production grammar is unchanged: `constructions.rs` and
`core_verbs.ron` are untouched, and Construction count is 387 -> 387. Parent
and tree `ambiguity --json --require-resolved --workers 8` reports are
byte-identical across all 32,641 units (`cmp` exit 0), both SHA-256
`54399b5258823a8cd26e72f390ba8811478866471ddda73ab35f0f8d3c41ec65`.
The selection census is unchanged: 13,759 unique, 3,842
specificity-resolved, 0 exception-resolved, 15,040 parse failures, 0
unresolved ties, and 0 internal failures. Newly covered identities: none.

Assurance counts: restored 0; re-spelled 2 (the validator edge-topology unit
and the trybuild placement fixture); ignored with blockers 0; added 0; removed
0. Deviations and additions: the ticket is incomplete because its explicit
STOP fired; only the separately required placement-validation widening and its
witnesses are present. Citations changed: none. Glossary gaps: Scope,
Attachment, Head, Premodifier, Peripheral, Bracketing, Mobility.

**REPORT.** Lock `covered`: 17,601. Construction count: 387. Licensed
vocabulary/lexicon homographs (2): `AttributiveAdjective::Untap` beside the
declaration keyword action `Untap`; `TargetingMarker::Target` beside
`CommonNoun::Target`. Form-literal/vocabulary overlaps (9): `additional` in
`additional_cost`; `to` in `up_to_quantifying_determiner`; `the` and `next` in
`definite_next_mass_quantity_reference`; `to` in
`scalar_less_than_or_equal_to`; `the` in `number_of_scalar_value`; `the` in
`greatest_scalar_value`; `other` in `other_than_qualified_reference`; and `the`
in `positional_partitive`.

Performance advisory, all with 8 workers: coverage 112 s at 137,513 ns/B,
host load 15.75/15.81/13.17; ambiguity 121 s at 144,408 ns/B, host load
15.34/15.79/14.00; roundtrip 118 s at 129,218 ns/B, host load
15.07/16.23/14.51. Each exceeds the 16.26 s quiet-host ceiling under load and
is provenance, never a gate target. The sandbox cannot observe sibling-process
count; each measurement used one foreground gate process. Wall clock: start
2026-09-04 22:24:16 PDT; end 2026-09-04 23:13:16 PDT.

Positive gates on the refreshed tree: `cargo fmt --all` exited 0; strict
all-target Clippy for `deckmaste_construction_core` and
`deckmaste_construction` finished without diagnostics; `cargo test
--workspace` exited 0 with every completed `test result: ok` line at 0 failed,
including `compile_fail_fixtures` and
`mobile_roles_add_only_an_empty_derived_slot_and_both_attachment_heights_agree`;
the focused validator witness passed 1/1; coverage reported 17,601 selected and
covered with every failure counter 0; ambiguity required resolution and
reported 0 ties; roundtrip reported 17,601 clean and 0 mismatched. Citation
gates were not required because no citation changed.
