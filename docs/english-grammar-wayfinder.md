# English grammar workbench and migration

Direction: [English grammar design in Lean](decisions/english-lean-design-workbench.md).
The independent NLP grammar review is complete. Its
[production correspondence](english-grammar-design.md#production-correspondence)
pins migration inside `english_v2`, retaining the Earley engine and declaration
compiler with the named extensions. English and Semantics remain separate
projects with no interaction. Lean execution or production consumption is not
required.

## Sequence

The ticket `needs:` graph is the scheduling authority. This map explains the
work; it is not a second status board. The earlier model, composition, document,
selection and review tickets are complete and remain historical evidence.

| Implementation ticket | Replacement unit | Prerequisite within migration |
|---|---|---|
| `english-v2-grammar-frame-compiler` | Runtime declared frame rules and generated checked sequence, exercised in a compiled consumer | Migration design |
| `english-v2-type-line-construction` | Open Type order metadata, builtin declarations and Type Line consumer together | Migration design |
| `english-v2-lexeme-owned-verb-frames` | Lexical predicates, relation/category separation, agreement, copular and passive frames | Frame compiler |
| `english-v2-adjective-inventory` | Nominal features, adjective ownership, productive polarity/type modifiers | Frames/clauses |
| `english-v2-subordinate-clause` | Shared dependent-clause bodies and declared form selection | Frames/clauses |
| `english-v2-remaining-prepositions` | PP distribution, independent license sets and generalized selected-role preemption | Nominals and subordination |
| `english-v2-relative-clause` | Extraction, relative forms and finite/wh clause Complements | Prepositions |
| `english-v2-scope-device-cross-host-gates` | Coordination ownership, evidenced selection and exact correlated scope alternatives | Extraction |
| `english-v2-grammar-measure-phrases` | Measures, arithmetic, comparison, degree and distribution | Scope/selection |
| `english-v2-grammar-document` | Textual collections, costs, keyword hosts, reminders and notation/templates | Extraction and measures |
| `english-v2-grammar-context-ellipsis` | Paragraph/intra-sentence recoverability, gapping and shared dependents | Documents |
| `english-v2-grammar-lexical-source` | Remaining lexical populations and source-recipe enforcement | Recoverability and Type Line |
| `english-v2-target-verb-subject-selection` | Withheld Target Verb, marker rivalry and Infectious Curse | Completed general grammar and source correspondence |

`english-v2-grammar-migration-close` depends directly on exactly these thirteen
implementation tickets. It validates the agreed design and decides whether to
resume `english-v2-stage-5-grammar-buildout-14-10` from representative residuals.
It does not require a coverage percentage, an exhaustive failure classification
or a global uniqueness theorem. A demonstrated general omission gets a named
migration task and an added closure dependency.

The compiler fixture is the first engineering check of data-defined frame
rules. Its production consumer immediately removes the hand-enumerated tail
codecs. Category recursion is migrated with consumers, not bridged by temporary
old-category aliases. Independent graph branches may still touch the same
`constructions!` source; claimants coordinate through the normal workspace
lifecycle rather than assuming graph independence means file independence.

## Obligations and retired tickets

The [obligation register](english-grammar-migration-obligations.md) preserves
all named re-coverage identities, the eleven passive-temporal misselections,
Class C/D, wrapper-overlap and frame-boundary challenges, and every historical
fog row. Its final table maps the ten merged ticket identities to their live
replacements. The other eight inherited tickets were rewritten around these
replacement units; five new tickets cover previously missing owners.

The cost-family merge has two responsibilities: document cost collections in
`english-v2-grammar-document`, ordinary cost comparisons in
`english-v2-grammar-measure-phrases` (a prerequisite). Keyword-subject modifiers
land with document hosts after the shared nominal/relative grammar. Type-line
schema, declaration and consumer now land together, resolving the prior STOP.
Historical semantic/legacy macro tickets are not prerequisites for NLP forms.

WIP tickets and claims were excluded from the reorganization. In particular,
`english-v2-keyword-action-verb-inflections` remains with its owner. Inspect its
integrated outcome at the frame/morphology slice; do not duplicate or edit its
WIP. Done-ticket bodies retain historical evidence and are not reopened.

## Limits and review discipline

`Analysis.Reading` is the formal entry point: composition, features and
extraction checks before selection. Bounded formal extensions belong to the
slice that needs them. Compiler fixtures and Rust structural tests establish
production correspondence; resemblance to Lean is not a Rust proof.

Keep the hoisted representative and derived metadata. Complete correlated site
assignments and frame anchorings must recover exactly the admitted alternatives
on the finite migration witnesses. A first-class AST node containing alternative
subtrees is not scheduled. This is the disposition of the former re-layering
investigation, whose historical link is retained below.

Two engineering costs need measurement: runtime frame preparation and growth of
complete scope-assignment rows. The Target Verb has a separate known design
risk: its marker/verb rivalry may remain grammatical. A surviving non-scope tie
requires a concrete ruling; a game-defined Subject whitelist is not permitted.
These limits do not reopen the completed whole-grammar review.

## The former re-layering wayfinder

The production decision is recorded in
[admission, selection and retained alternatives](english-grammar-design.md#admission-selection-and-retained-alternatives).
It retains the derived AST strategy while adding the information needed by the
actual interaction witnesses. Historical references to this anchor resolve
here; they do not prescribe the former Group R schedule.
