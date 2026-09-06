---
needs: [english-lean-grammar-design-review]
---
# Plan the constructions! migration from the formal grammar

Design the production migration from the reviewed Lean `English` model into
`english_v2`, following the [Lean design decision](../../decisions/english-lean-design-workbench.md).
Keep declaration-owned types, parsing, rendering, and traversal. Assess reuse
of the chart engine, declaration compiler, lexical environment, scanner,
materialization, selection, and scope machinery against concrete requirements.
Compiler changes are allowed when needed; neither a separate `english_v3` nor
automatic consumption of Lean is required.

Write the correspondence in `docs/english-grammar-design.md`: which formal
categories, judgments, and surface relations map to which declarations or
compiler facilities, and where tests rather than proofs will establish Rust's
behavior. Identify coherent replacement boundaries after considering mutually
dependent categories; temporary compatibility must earn its complexity.

Reconcile the unclaimed implementation tickets listed in the wayfinder. Keep
their examples, negative cases, lost-coverage identities, and independently
justified outcomes; replace obsolete mechanisms and scheduling. Some inherited
prescriptions may conflict with the glossary or formal design: name and resolve
the conflict explicitly. Make each resulting implementation ticket executable
with pinned scope, dependencies, replacement/deletion targets, and structural
acceptance. Retire merged tickets only after moving all obligations and edges.
Do not edit WIP tickets or claims; assess their integrated results when available.

Give every fog family and re-coverage obligation a production owner or an
explicit deferred destination. Update `english-v2-grammar-migration-close`'s
`needs:` to exactly the resulting migration tasks, including tasks minted for
requirements not represented in today's backlog. Update the wayfinder and
validate the graph before closing this design ticket. The old Group R order
does not constrain the new dependency graph.

Acceptance: the implementation sequence accounts for the reviewed model and
all inherited obligations, identifies the surrounding machinery's fate, and
can be executed without rediscovering the grammatical design. No production
code changes or production coverage measurement are required for this ticket.
Standard constraints apply.

## Reviewed entry point and elaborations

Use `English.Analysis.Reading` as the formal candidate contract, including
`Features.Conforms` and `Dependencies.Safe`; the composition-only `Admissible`
relation is insufficient. Preserve declaration-backed local preference and
packing over complete, checked alternatives. Termination measures do not
license comparisons, and equal surface is not an equivalence key.

The review resolved the feature/context/carrier choices with bounded checked
fragments. Pin the elaborations listed in the design for each production
slice: arbitrary declared role sequences and their context lifting; additional
scope/sharing moves; qualification/distributive distributions; remaining
wh/free-relative, omission and intra-sentence context forms; and lexical,
notation, template and source-byte contracts. Add the applicable positive and
negative formal witnesses before relying on a new extension. These are
migration prerequisites for those slices, not post-migration tail work.

Retain the inherited frame-pair alternate-anchoring obligation. Both readings
must survive; no leftmost boundary or specificity preference is authorized.
The prototype's lexical-leaf and anchor-count invariants are necessary checks,
not a replacement for the full production topology contract.

## Landing record

Migration design completed 2026-09-05 on change `zyvukoyk`; no production code,
Lean definitions, declaration data or coverage lock changed. The source
inspection at claim `luwqpxyt` covered the generic chart, scanner, environment,
generated builders/AST/render/visit, materialization and selection. Lean LSP
outlines checked the actual `Analysis.Reading`/`package_exact` and
`Dependencies.Admitted` interfaces. The resulting correspondence and machinery
decisions are in `docs/english-grammar-design.md`.

PROVE: the obligation transfer retains all twenty full identity keys from the
former fog/relative/Target records, all forty-one historical frontier rows,
and the scope/overlap/frame-boundary witnesses. The ten merged unclaimed
nodes were removed only after assigning their obligations and transferring
incoming live edges. No protected WIP/done node had an incoming dependency
on a retired node. Closure has exactly the thirteen resulting implementation
tasks as direct dependencies. The graph has no duplicates, cycles or dangling
needs. Changed-document links resolve. No production identity or analysis was
added, removed or measured by this documentation ticket.

DISCLOSE: eight inherited implementation tickets were repinned; five were
added for the frame compiler, measures, documents, recoverability and lexical/
source correspondence. Ten smaller tickets were merged, with the old-to-new
routing recorded in `docs/english-grammar-migration-obligations.md`. The
unclaimed `template-verb-conjugation` dependency and prose now point to the
combined frame/agreement owner. WIP tickets belonging to other workspaces and
done-ticket records were untouched. The refreshed tree includes another
session's new claim; no work from that claim was edited.

STOP/reconciliation: the old Target Verb ticket prescribed a game-defined
Spell/Ability Subject whitelist. That prescription was not carried into the
new plan: the user's explicit NLP-only/no-Semantics clarification supplies the
authority to remove it. Its marker/verb ambiguity and Infectious Curse remain
an explicit integration gate, not a solved case. The old specificity and
meaning-bearing trigger-envelope prescriptions conflict with the reviewed
candidate/document contracts; their scheduled replacement is recorded in the
two decision documents under the accepted top-down migration authorization.
No production rule was changed or narrowed to conceal those conflicts.
The old Type Line no-consumer/no-schema-change STOP is resolved by a single
schema/declaration/consumer ticket within the already-authorized compiler scope.

Deviations and additions: the five missing implementation owners, the obligation
register, three glossary entries (Case, Tense, Subordinator), and the tracked
decision deltas make the authorized migration executable. No new grammar
constructions or formal proof obligations were implemented here. Runtime frame
preparation and complete scope-assignment size are explicitly uncertain costs
with compiled/structural witnesses in their owners. The completed grammar
review was not reopened; no global uniqueness or completeness gate was added.
Assurance counts: zero tests restored, re-spelled, ignored, added or removed.
The retired files are planning tickets, not tests or coverage evidence.

REPORT: production coverage, selection, construction and performance figures
were not measured; this ticket explicitly requires no production code or
coverage ceremony. Historical counts retain their original provenance in the
obligation register. After a changed `kata refresh`, the documentation audit
and graph checks passed again; `cargo xtask cite check` checked 14,929 citations
with zero stale references. The source changes are documentation only, so no
Rust test closure or new Lean build was required.
