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
