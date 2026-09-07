# Independent lexical analysis and retained grammatical readings

Accepted 2026-09-07 after the rewrite planning dialogue. Replace the current
`english_v2` grammar in place, salvaging useful implementation and evidence.
English remains an independent NLP project with no interaction with Semantics.
The [wayfinder](../english-grammar-wayfinder.md) names the implementation work.

## Authority and retained work

This decision supersedes conflicting production prescriptions in
[the original rewrite](english-v2-rewrite.md),
[the Lean migration](english-lean-design-workbench.md), and
[the grammar design](../english-grammar-design.md). Their source hierarchy,
Vintage support scope, linguistic requirements and authentic regression
witnesses survive. Old coverage figures describe their measured trees only.

Keep bidirectional construction declarations: one declaration drives generated
parsing rules, checked construction, rendering and traversal. Per-construction
handwritten renderers are not an alternative implementation. The proposed
Rust-data-first replacement of the compiler was withdrawn; compiler changes
serve the new lexical/admission interfaces, rather than starting another DSL
project. Existing generated types may change where that interface requires it.

Salvage catalogs, lexical identities, declared forms and frames, immutable
indexes, source/normalization evidence, corpus subset/report/inspection tools,
and independent Lean witnesses. The generic Earley scheduler and packed forest
are reuse candidates. The archived verb-frame/breadth Rust stack is unfinished;
extract useful declarations and tests individually, rather than transplanting
it or counting its grammar families as current production.

## Lexical analysis

Analyze input independently of construction-directed scanning. Each alternative
identifies a declared Lexeme, its lexical Category, Word Form, correlated
grammatical features, and declaration/catalog provenance. Inflectional Form,
Person, Number, Tense and Finiteness remain distinct concepts; absence or
inapplicability of a feature is not permission to invent its values.

Analyses come only from declared vocabulary and its default or overridden
morphology. There is one default rule per applicable inflection; an explicit
irregular override replaces that default for the inflection, and multiple
valid alternatives are explicitly declared. Rules apply within a declared
lexical category. Unknown words and unlicensed POS/derivational readings are
reported as vocabulary gaps, never guessed. Initial overgeneration means
retaining all these licensed alternatives until grammar constrains their use.

Lexemes also declare grammatical properties, including count/mass uses and
selected Complement frames. Grammar applies general composition constraints
to these properties; morphology does not parse the surrounding phrase. Core
vocabulary, plugin declarations and catalogs feed one analysis interface even
when their authoring sources differ. Lexical inflection generation used for
analysis and realization must share the same defaults and overrides.

The lexical ticket pins the remaining representation choices against actual
inputs: atomic token boundaries, bound and overlapping multiword forms,
capitalization, feature applicability, and the exact default spelling rules.
The decision to use one default does not yet prescribe literal suffix append
versus an algorithm with ordinary orthographic cases. Record the chosen
algorithm and its overrides; do not silently introduce guessed paradigm classes.

## Source and roundtripping

Prefer a lossless token sequence with positions derived from occurrences and
surface lengths. Exact token/separator treatment is refined in the lexical
ticket. Position, lexical identity and declaration provenance are separate;
discarding redundant span storage does not discard the latter two. Reuse
existing ownership checks for their useful invariants, not to preserve their
entire representation. No source-buffer echo or opaque catch-all leaf counts
as grammatical realization.

With the same parse context and lexical environment, both laws are required:

```
v is an admitted reading of s  =>  render(v) = s
v is independently constructed and grammatically valid
                              =>  v is a reading of parse(render(v))
```

Equality in the second law preserves grammatical structure, lexical identities,
features and explicitly retained spelling variants. Computed source positions
are not grammatical identity. Every non-derivable surface distinction admitted
by the grammar must be recoverable by the declaration-driven renderer; the
lexical integration pins how it is represented. Keep raw source and any
preexisting normalization distinct. Tests starting from independently
constructed values remain required alongside corpus parse/render checks.

## Chart admission and ambiguity

Retain Earley-family chart parsing. The lexical layer supplies alternatives;
the grammar composes Categories, lexical frame requirements and feature/
dependency constraints. Chart completion must operate on sufficient grammatical
summaries or explicit constraints, without constructing Cartesian products of
full AST child values. Generated declarations supply those checks. Full AST
construction happens when a reading is requested from the forest.

Packing preserves every distinction that can affect a parent's admissibility
and keeps alternatives correlated. A feature-sensitive key is a candidate
implementation, not a proof that an arbitrary bundle is sufficient. Open
dependencies must remain represented until their context is available. Any
deferred validity check must participate in the definition of an admitted
reading: an unchecked forest path is not a successful grammatical analysis.

The primary result retains all admitted readings. Multiple grammatical readings
are successful parsing, including unrelated homographs and different scope
classes. Preference is an optional, non-destructive view. A representative may
aid display but cannot remove alternatives. Equal text does not equate readings;
distinct derivations of the same intended structure are not automatically
distinct linguistic readings. Test that distinction explicitly.

This replaces the hard-error tie rule, global structural-specificity arbitration
as admission, and the requirement to fit all retained ambiguity into one
hoisted representative with derived site/anchor rows. Existing local scope
classes and their invariants remain useful relationships within the complete
reading set. Neither a specific alternative-subtree AST nor a general new
forest implementation is mandated; exact recoverability is the contract.

## Iteration and evidence

The first production integration exercises ambiguous output and both roundtrip
laws, before broad grammar migration. Measure lexical alternatives, packed
nodes and completion/materialization work on interacting fixtures. If the
existing compiler cannot express the needed grammatical summaries, report the
concrete missing operation and bounded extension; do not silently restore the
AST product or undertake another compiler rewrite.

Run tokenization/analysis and the word inventory on the supported corpus early.
Account separately for catalog, keyword and notation material so a numeric
coverage count cannot conceal lexical gaps. Use the existing whole-grammar
family map, establish consumed capability across families, then deepen their
inventories. New general omissions may reopen design. The long-tail handoff is
an adaptable judgment from measured residuals, without a fixed coverage target
or a claim that no general grammar gap can remain.

The landing contract changes only where the accepted ambiguity/position design
requires it: report no-reading failures, unique readings and multiple readings
separately; retain exact-source, lexical identity, structural traversal and
zero-internal-failure checks. The old zero-unresolved-ties requirement is
superseded. A newly admitted wrong reading is still a defect, even if a correct
reading also survives. Preserve the no-silent-loss identity accounting and
classify removed wrong analyses separately from regressions. Optional preference
has its own disclosed effects and does not determine parsing coverage.

## Lean and remaining uncertainty

Make the lexical-analysis relation constrain actual admission, preserving the
restored word-form counterexamples as evidence until their defect is repaired.
Model admitted readings across classes independently of preference; demonstrate
lost-correlation exclusions using inhabited grammatical examples. State the
relational roundtrip obligations without making them true merely by definition.
An executable Lean parser or proof of the Rust engine is not scheduled.

The new lexical/ambiguity ticket handles those interfaces; the remaining
`english-lean-proof-gaps` work retains its substantive obligations, including
flat coordination, tense and countability. Neither an external review nor a
desire to defer work authorizes declaring Oracle English tense-neutral. The
amount of explicit forest structure Lean needs depends on the concrete
counterexamples; a blanket SPPF formalization is not a prerequisite.

The main engineering uncertainty is how much compiler/admission adaptation and
feature-sensitive packing cost. Reviews identified the existing eager AST
product in source, but did not measure its runtime contribution. The first
integration measures the change before committing to broader migration.
