---
needs: [english-v2-lexical-analysis]
---
# Consume lexical analyses with feature-aware admission and retained readings

Implement the first production integration under
[chart admission and ambiguity](../../decisions/english-lexical-analysis.md#chart-admission-and-ambiguity).
Retain the Earley engine and bidirectional declaration/compiler approach.
One declaration still drives checked construction, parsing, rendering and
traversal; no per-construction handwritten renderer or second fallback parser.

Replace construction-directed byte scanning with the independent lexical
interface. Generate sufficient grammatical summaries/constraints from the same
declarations and use them at completion. The completion path must not construct
`BuildValue` products or run full AST constructors to decide admissibility.
Extract full AST readings on demand after chart construction. Audit each
affected `require`/`derive`/frame/dependency check: enforce it on summaries or
retain an explicit deferred constraint until its context is available. Every
exposed reading must satisfy all grammatical checks. Report a missing compiler
operation before substituting a new architecture or restoring eager products.

Define packing identity from the distinctions parents actually inspect. Show
two same-span alternatives with different relevant features remain usable in
their respective contexts, and no mixed feature/child assignment is invented.
Exercise shared child growth as well as initially populated nodes. Measure
node growth and completion work before choosing a more elaborate key or forest.

Change the primary parser result and actual xtask consumers to retain all
admitted readings. Hard-error ties and destructive specificity/scope collapse
are superseded here. Preserve structured no-reading/internal-failure diagnostics;
inspection exposes lexical analyses and distinct complete bracketings without
requiring exhaustive AST enumeration for ordinary parsing. Optional preference
cannot change the primary reading set. Do not delay this interface to a later
selection ticket.

Acceptance uses a bounded set of interacting constructions through emitted
code and the real parser: same-surface noun/verb analyses, lexical agreement
and wrong-case exclusions, overlapping multiword analyses, multiple unrelated
readings, and a correlated alternative where independent choices would admit
a wrong combination. Preserve existing constructed-value reverse-roundtrip
witnesses in `tests/parser.rs::generated_invariant_products_enforce_values_and_round_trip_publicly`.
Check both [roundtrip laws](../../decisions/english-lexical-analysis.md#source-and-roundtripping)
for every finite witness reading, including independently constructed values.

Adapt corpus reporting to no reading / one reading / multiple readings, with
invalid readings still defects. Use focused subsets while developing and the
standard final whole-corpus identity accounting and performance telemetry.
Report compiler/build cost as well as parse work; source inspection is not a
performance measurement. Retire superseded adapters on the replaced path.
Standard constraints apply.
