# Guided tour

deckmaste.rs is easiest to read along the same path a card takes: Oracle text is
compiled into a typed intermediate form, while the card's printed
characteristics arrive as structured metadata. That information-complete form
can then be rendered back to English or lowered into a shared rules engine. This
tour follows one example through that path.

## Oracle text becomes typed data

[Elesh Norn, Grand
Cenobite](../plugins/canon/cards/Elesh%20Norn,%20Grand%20Cenobite.ron) is the
current persisted intermediate form of the card: its characteristics and three
composed abilities — vigilance, a static boost for other creatures you control,
and a static penalty for creatures your opponents control. There is no Elesh
Norn class or resolution script. The two statics use the same predicates and
modifications available to every other card.

The canonical form is deliberately minimal as well as complete. It preserves
everything rendering and execution need without retaining a parser trace or an
expanded machine dump, so a mistranslation can be investigated by comparing
this short RON term with the card itself. The built-in macro library provides
the compact, rules-shaped names; its construction frames connect those names to
the Oracle phrases they recover and spell. Those same frames drive ingestion:
the compiled frame lexicon matches the parsed Oracle tree, each match identifies
a macro and fills its typed arguments, and nested matches assemble the persisted
RON card.

Oracle text is the authoritative input for existing cards; after compilation,
the RON term is their canonical semantic representation, joined by the
structured characteristics. The curated corpus is still written and verified
by hand while structural recovery grows, but its intended producer is the
English parser and spelling relation. From here the term macro-expands and
lowers into the smaller core vocabulary the engine executes. The crate map in
[Architecture](../README.md#architecture) shows those projections.

The English-side projection checks losslessness: `cargo xtask english roundtrip`
renders every supported face back to the same normalized Oracle bytes. The
recovery census separately records how much of that text became structure rather
than a preserved unknown span. Recovery coverage and executable-card coverage
turn “reading the card explains the card” into two card-by-card measurements.

## Rules live in shared systems

Continuous effects meet in
[`layer.rs`](../crates/deckmaste_engine/src/layer.rs). Start with
[`run_layer_pass`](../crates/deckmaste_engine/src/layer.rs#L1929): it applies the
layer sequence, asks
[`depends_on`](../crates/deckmaste_engine/src/layer.rs#L1837) whether one effect
changes another's affected set, and re-evaluates the remaining order after each
application. The code cites the Comprehensive Rules at the point where each
ordering constraint enters the algorithm.

The focused tests live beside the implementation. In particular,
[`same_layer_change_enables_conditional_static`](../crates/deckmaste_engine/src/layer.rs#L3340)
proves that an earlier effect can enable a later conditional in the same layer,
while
[`granted_static_is_gathered_to_fixpoint`](../crates/deckmaste_engine/src/layer.rs#L3260)
proves that an ability granted in layer 6 becomes an effect source on the next
pass. These are engine behaviors, so every card using those shapes gets them
without card-specific code.

## Invalid descriptions fail at the boundary

[`Card.check`](../lean/Semantics/Check/Card.lean) checks a card-language term
and returns its complete refusal list. A `Spelled` card carries a proof that
this list is empty. The checker reads structural and declared features for
bindings, kinds, zones and card frames; the Rust engine executes game behavior.

The [pin suites](../lean/Semantics/Proofs/) give exact positive and negative
witnesses. For example, `okChoosePlayerOrPlaneswalker` accepts a joined-kind
choice, while `badChooseYou` proves the single `choiceClause` refusal for
“Choose you.” Both live in `Proofs/Anaphora.lean` and use kernel `decide`.
Run `lean/scripts/build` to check the workbench and its card bench.

The [Rust-to-Lean card gate](tickets/planned/lean-card-soundness-gate.md) remains
pending. The current workbench build proves the terms supplied to Lean; it does
not yet certify every loaded Rust card. The [succession decision](decisions/lean-is-the-workbench.md)
records that boundary and the frozen Idris reference.

## Run the whole path

`cargo run` bootstraps the current cached demo corpus on first use and launches
the terminal client shown in the README. The current scope and the work still
in motion are recorded plainly in
[Present state and roadmap](../README.md#present-state-and-roadmap).
