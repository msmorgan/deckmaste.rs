# English grammar rewrite

Current authority: [independent lexical analysis and retained readings](decisions/english-lexical-analysis.md).
Keep bidirectional construction declarations; separate declared morphology from
construction-directed scanning and grammatical admission from full AST products.
The primary parse result retains all grammatical readings. English and Semantics
remain independent projects. The `needs:` graph is the scheduling authority.

## Upcoming work

| Ticket | Deliverable |
|---|---|
| [english-v2-lexical-analysis](tickets/planned/english-v2-lexical-analysis.md) | Independent analyses from declared vocabulary/default morphology/explicit replacing overrides; token/source contract and early supported-corpus word inventory. |
| [english-lean-lexical-ambiguity](tickets/planned/english-lean-lexical-ambiguity.md) | Connect lexical forms to admission, retain readings across classes, and challenge correlated alternatives and relational roundtripping. Runs alongside lexical/Rust work. |
| [english-v2-feature-chart-integration](tickets/planned/english-v2-feature-chart-integration.md) | Adapt the existing engine/compiler to grammatical summaries, packed alternatives and on-demand AST extraction; exercise both roundtrip laws and actual all-readings consumers. |
| [english-v2-grammar-family-breadth](tickets/planned/english-v2-grammar-family-breadth.md) | Consumed positive/exclusion and cross-family witnesses for every planned family through the new interfaces. |
| [english-v2-grammar-migration-close](tickets/planned/english-v2-grammar-migration-close.md) | Consolidate the replaced paths, reconcile evidence and owners, and make the revisable long-tail handoff judgment. |
| [english-v2-stage-5-grammar-buildout-14-10](tickets/planned/english-v2-stage-5-grammar-buildout-14-10.md) | Eventual supported-corpus completion through coherent residual work. The handoff is not completion of this umbrella. |

[english-v2-grammar-lexical-source](tickets/planned/english-v2-grammar-lexical-source.md)
completes the vocabulary/source inventory after lexical analysis, independently
of the grammar-family schedule. [english-lean-proof-gaps](tickets/planned/english-lean-proof-gaps.md)
retains the remaining model repairs after accounting for the focused Lean work.
Neither inventory exhaustion nor every formal residual blocks the breadth
experiment. Concrete missing interfaces are repaired where they are needed.

## Family completion

These tickets consume breadth. They retain linguistic requirements, authentic
witnesses and consolidation work; they do not prescribe the old scanner,
hoisted-AST representation or uniqueness policy.

| Ticket | Responsibility |
|---|---|
| `english-v2-lexeme-owned-verb-frames` | Lexical predicates, shared relations, agreement, copular/passive frames and remaining codec replacement. |
| `english-v2-adjective-inventory` | Nominal/countability features and adjective/participle distribution. |
| `english-v2-subordinate-clause` | Shared dependent clauses and selected forms. |
| `english-v2-remaining-prepositions` | PP distributions, grammatical Complement selection and attachment alternatives. |
| `english-v2-relative-clause` | Relative forms, extraction, category/relation gaps and boundaries. |
| `english-v2-scope-device-cross-host-gates` | Coordination and exact correlated readings across hosts; optional preference explanations. |
| `english-v2-grammar-measure-phrases` | Measures, arithmetic, comparison, degree and distribution. |
| `english-v2-grammar-document` | Textual/cost/keyword collections, reminders, quotations and templates. |
| `english-v2-grammar-context-ellipsis` | Recoverability and shared/omitted dependents in real document contexts. |
| `english-v2-type-line-construction` | Open Type ordering, Type Line declarations and consumer. |
| `english-v2-target-verb-subject-selection` | Target Verb/marker/Noun distinctions and Infectious Curse; retain multiple readings where grammatical. |

## Salvage and ticket disposition

The reviewed [family/source map](english-grammar-design.md#whole-intended-grammar-and-source-map)
and [obligation register](english-grammar-migration-obligations.md) remain the
starting evidence. Existing frame-compiler landings are reusable capability,
not proof that the new lexical/admission interface exists. Archived Rust work
is unfinished and does not establish production breadth. Restored Lean witnesses
retain their stated limits, including the still-admitted crossed word forms.

Three new tickets introduce lexical analysis, chart integration and focused
Lean changes. Existing breadth, frame/scope/source/Target Verb, model residual
and closure tickets are refocused. The register records eighteen retired
legacy tickets, their retained witnesses and replacement owners. Retirements
are cancellations/merges, not falsely completed implementations. Other legacy
instrumentation and semantic-consumer tickets remain outside this parser's
readiness graph; they do not authorize game-semantic admission constraints.

WIP tickets and claims are untouched. In particular,
`english-v2-keyword-action-verb-inflections` remains with its owner. Assess its
integrated morphology data against the shared lexical interface; do not create
a second implementation of its active work or edit its claim. Done-ticket
bodies remain historical landing evidence.

## The former re-layering wayfinder

The [current reading contract](decisions/english-lexical-analysis.md#chart-admission-and-ambiguity)
supersedes the former Group R/hoisted-representative schedule. Local scope
classes may organize readings; all valid classes coexist in the primary result.
