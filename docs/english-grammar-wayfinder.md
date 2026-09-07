# English v3 grammar

Current authority: [independent lexical analysis and retained readings](decisions/english-lexical-analysis.md).
English v3 is a fresh bidirectional grammar over independent declared lexical
analyses. It retains every grammatical Reading and shares no code or model with
Semantics. The `needs:` graph is the scheduling authority.

[Lexical analysis is implemented](tickets/done/english-v2-lexical-analysis.md).
The current [`english-v2-feature-chart-integration`](tickets/wip/english-v2-feature-chart-integration.md)
claim is producing the disposable v3 chart prototype that anchors this new
line. `english-v2-keyword-action-verb-inflections` remains with its owner; the
lexical-inventory ticket consumes any useful declarations only after it lands.

## Production chain

| Order | Ticket | Deliverable |
|---:|---|---|
| 1 | [english-v3-lean-grammar-model](tickets/planned/english-v3-lean-grammar-model.md) | Rebuild the independent English workbench around the complete v3 grammar, lexical relation, admitted Readings and relational roundtripping. |
| 2 | [english-v3-lean-proof-audit](tickets/planned/english-v3-lean-proof-audit.md) | Make the model's central inhabitants, exclusions and correlation laws nonvacuous before Rust follows them. |
| 3 | [english-v3-lexical-model](tickets/planned/english-v3-lexical-model.md) | Extract the small data-only model shared by lexical analysis, generated constructions and English v3. |
| 4a | [english-v3-packed-chart](tickets/planned/english-v3-packed-chart.md) | Pack incomplete and complete derivations, define future-admissibility summaries and measure actual growth. |
| 4b | [english-v3-lexical-inventory](tickets/planned/english-v3-lexical-inventory.md) | Consolidate declared morphology, catalogs, numeral codecs and supported-corpus lexical remainders. |
| 5 | [english-v3-construction-compiler](tickets/planned/english-v3-construction-compiler.md) | Build the independent compiler core and thin proc macro that generate every bidirectional projection. |
| 6 | [english-v3-generated-roundtrip-slice](tickets/planned/english-v3-generated-roundtrip-slice.md) | Prove the generated pipeline and both roundtrip laws on one bounded interacting slice. |
| 7 | [english-v3-whole-grammar-activation](tickets/planned/english-v3-whole-grammar-activation.md) | Declare every planned family together, turn the complete machine on, and establish the first supported-corpus baseline. |
| 8 | [english-v3-systemic-residuals](tickets/planned/english-v3-systemic-residuals.md) | Repair failures by shared cause until the remaining work is a productive long tail. |
| 9 | [english-v3-production-cutover](tickets/planned/english-v3-production-cutover.md) | Move every production consumer to v3 and retire the replaced v2 parser/compiler paths. |
| 10 | [english-v3-corpus-long-tail](tickets/planned/english-v3-corpus-long-tail.md) | Complete the remaining supported-corpus work in coherent residual batches. |

The chart and lexical-inventory tickets can run in parallel after the shared
model lands. Everything before the whole-grammar activation establishes a
sound machine; none of those slices becomes the grammar-migration work unit.
The activation translates the complete connected grammar informed by Lean and
only then measures what the corpus accepts.

## Evidence and salvage

The [whole-grammar source map](english-grammar-design.md#whole-intended-grammar-and-source-map)
and [obligation register](english-grammar-migration-obligations.md) preserve the
requirements formerly distributed across family tickets. V2 declarations,
tests, catalogs and algorithms are quarries. V2 scanner APIs, eager AST
admission, stored ownership, destructive selection and compatibility shapes do
not constrain v3.

The old predicted family tickets are retired rather than carried into the
execution graph. Their evidence is owned first by whole-grammar activation and
then by cause-grouped residual work. New residual tickets are minted from the
machine-on report, rather than forecast from individual failed cards.

Old v2 instrumentation tickets that depend on a selected tree or the v2 AST are
also retired. The v3 activation and residual tickets own reproducible corpus
inspection, structural validation and performance evidence against the new
all-Readings interface.

WIP tickets and claims remain untouched. Done tickets remain historical landing
evidence; their old owner names describe the work graph that existed when they
landed.
