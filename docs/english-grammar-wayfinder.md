# English grammar workbench and migration

Direction: [English grammar design in Lean](decisions/english-lean-design-workbench.md).
Design the whole intended grammar in the standalone `english/` Lake project, then
migrate `english_v2`'s `constructions!` implementation. The workbench is a formal
model; executable parsing/rendering and production consumption are optional.

## Sequence

The ticket `needs:` graph is the scheduling authority. This map explains the
work; it is not a second status board.

| Ticket | Deliverable |
|---|---|
| `english-lean-grammar-model` | Whole-grammar source/scope map, formal vocabulary, derivation and realization relations, a checked initial fragment |
| `english-lean-grammar-composition` | General phrase/clause composition, lexical frames and features, cross-capability witnesses |
| `english-lean-document-grammar` | Document structure, notation, keyword surfaces, and type-line realization relations |
| `english-lean-selection-laws` | Explicit ambiguity/preference/packing model, named proofs and counterexamples |
| `english-lean-grammar-design-review` | Whole-model review, interaction challenges, resolved design blockers and stated limits |
| `english-v2-grammar-migration-design` | Rust correspondence, machinery reuse decisions, regrouped and pinned implementation tickets |
| `english-v2-grammar-migration-close` | Migrated-design validation and a revisable decision to resume the tail loop |
| `english-v2-stage-5-grammar-buildout-14-10` | Subsequent corpus long-tail completion |

Document modeling and selection proofs can proceed after the shared composition
model; the review checks their interaction. Production replacement boundaries
are chosen after that review, not prescribed by the old Group R order.

## Reviewed model and migration boundary

The review's final candidate interface is `Analysis.Reading`, combining
composition, lexical-feature constraints and extraction constraints before
selection and packing. The [design](english-grammar-design.md) and
[review](english-grammar-review.md) name its proven fragments and elaboration
obligations. Raw `Admissible` alone is not the reviewed candidate contract.

Migration design must pin general role sequences/context lifting, further
scope moves, qualification/distributive distributions, remaining relative/wh
and omission forms, and lexical/source-boundary enforcement for the slices
that need them. These are explicit grammar/correspondence work, not a return
to card-by-card tail coverage. Existing named regressions keep their owners.

## Existing unclaimed migration work

These ticket identities retain the detailed examples, negative cases, and
re-coverage obligations so existing references keep their destinations. They
wait on the migration design; their earlier implementation recipes are input
for reconciliation, not a competing implementation plan. The design ticket
must rewrite, merge, or replace them with pinned migration tasks and update
the closure dependencies before it completes.

| Concern | Existing tickets |
|---|---|
| Grammatical relations and agreement | `english-v2-grammatical-relations`, `english-v2-person-number-agreement` |
| Lexical frames and clause composition | `english-v2-lexeme-owned-verb-frames`, `english-v2-subordinate-clause`, `english-v2-relative-clause` |
| Lexical inventories and keyword hosts | `english-v2-adjective-inventory`, `english-v2-keyword-subject-modifiers`, `english-v2-target-verb-subject-selection` |
| Preposition/complement licensing | `english-v2-attachment-class-declared`, `english-v2-locative-licence-set`, `english-v2-of-complement-filter-removal`, `english-v2-remaining-prepositions` |
| Coordination and packed ambiguity | `english-v2-locative-coordination-arms`, `english-v2-scope-device-cross-host-gates`, `english-v2-frame-complement-pair-nesting` |
| Document and notation structure | `english-v2-cost-family-lowering`, `english-v2-type-line-construction`, `english-v2-type-line-declaration` |

WIP tickets and claims are excluded from this reorganization. Work already
underway may change the inventory before migration design; inspect integrated
results and preserve their independently justified outcomes. Done tickets are
history, not reopened work. Unversioned legacy English tickets are not a source
of grammatical authority or an instruction to extend the retirement-bound v1
implementation.

## Fog and regression obligations

[fog.md](tickets/fog.md) retains the detailed witnesses and historical census.
The formal model must account for these concerns; the migration design gives
each implementation obligation an owner:

| Fog concern | Formal owner |
|---|---|
| Gerunds, ellipsis, gapped destinations, recipient/retained-object passives | `english-lean-grammar-composition` |
| Copular complement consolidation, polarity, type modifiers | `english-lean-grammar-composition` |
| Arithmetic, fractions, comparisons, degree phrases | `english-lean-grammar-composition` |
| Bare temporal adjuncts swallowed as passive objects | `english-lean-grammar-composition` and `english-lean-selection-laws` |
| Door/half references | `english-lean-grammar-composition`; game-reference resolution is outside English |
| Shared gaps: relative and complement clauses, coordination, anaphoric forms | `english-lean-grammar-model` for scope, then composition |
| Reminder/editorial text and document layouts | `english-lean-document-grammar` |
| Historical tail buckets, including unowned and unbucketed cases | Model scope review, then migration design or the refreshed long-tail register |

In particular, preserve the named obligations for Absorbing Man and Titania
(relative clause), Infectious Curse (relative clause and Target Verb), Cavalier
of Thorns / Animal Magnetism / Genesis Ultimatum (gapped destinations), and the
eleven passive-temporal misselections recorded in fog. Preserve the cross-host,
wrapper-overlap, and conjunct-boundary witnesses in their tickets. These are
regression evidence; no card identity or census may choose the grammar's rules.

## The former re-layering wayfinder

The earlier rewrite and scope records referred to a future re-layering map.
This document is that planning destination. The formal work may investigate a
first-class representation of alternatives and the limitations of the current
derived-site/anchor representation. Investigation does not authorize silently
changing the production scope contract. Record a justified production decision
in the migration design before replacing that representation.

## Completion and evidence

Lean tasks finish on their named definitions, witnesses, and theorem
obligations, with explicit assumptions and no hidden proof placeholders.
Production tasks finish under the existing structural and landing contracts.
The Lean model is not automatically consumed by Rust, and its proofs do not
transfer to Rust by resemblance.

The migration-close ticket owns the return to the tail loop. It requires
implementation of the agreed design and representative residual evidence, not
a proof of grammar completeness or an exhaustive corpus classification.
Further general omissions reopen design. Coverage and construction counts
remain observations, not targets for this design phase.
