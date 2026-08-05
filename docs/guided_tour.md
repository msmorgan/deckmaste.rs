# Guided tour

deckmaste.rs is easiest to read along the same path a card takes: authored data
enters a shared rules engine, and the representations at that boundary are
checked against explicit invariants. This tour follows one example through that
path.

## A card is data

[Elesh Norn, Grand
Cenobite](../plugins/canon/cards/Elesh%20Norn,%20Grand%20Cenobite.ron) contains
the card's characteristics and three composed abilities: vigilance, a static
boost for other creatures you control, and a static penalty for creatures your
opponents control. There is no Elesh Norn class or resolution script. The two
statics use the same predicates and modifications available to every other
card.

The authored form is intentionally close to printed rules text, but it is not
text. It is typed data that macro-expands and lowers into the smaller core
vocabulary the engine executes. The crate map in the
[Architecture](../README.md#architecture) section shows those projections.

## Rules live in shared systems

Continuous effects meet in
[`layer.rs`](../crates/deckmaste_engine/src/layer.rs). Start with
[`run_layer_pass`](../crates/deckmaste_engine/src/layer.rs#L1920): it applies the
layer sequence, asks
[`depends_on`](../crates/deckmaste_engine/src/layer.rs#L1837) whether one effect
changes another's affected set, and re-evaluates the remaining order after each
application. The code cites the Comprehensive Rules at the point where each
ordering constraint enters the algorithm.

The focused tests live beside the implementation. In particular,
[`same_layer_change_enables_conditional_static`](../crates/deckmaste_engine/src/layer.rs#L3339)
proves that an earlier effect can enable a later conditional in the same layer,
while
[`granted_static_is_gathered_to_fixpoint`](../crates/deckmaste_engine/src/layer.rs#L3260)
proves that an ability granted in layer 6 becomes an effect source on the next
pass. These are engine behaviors, so every card using those shapes gets them
without card-specific code.

## Invalid descriptions fail at the boundary

[`Spec.idr`](../idris/src/Spec.idr) describes authored states that must be
unrepresentable. Its named `failing` blocks pin both rejection and diagnostic:
`tBadTargetOutOfRange` reads a target slot that was never announced,
`tBadEventActorMultiKind` asks a mixed event family for an actor not every event
supplies, and `tBadModalOverCount` chooses more modes than exist.

`cargo xtask idris-check plugins/canon` emits the supported Rust card
definitions into that model and typechecks them. The Idris code is a grammar
oracle, not a second game engine; runtime semantics remain in Rust.

## Run the whole path

`cargo run` bootstraps the demo corpus on first use and launches the terminal
client shown in the README. The current scope and the work still in motion are
recorded plainly in [Present state and roadmap](../README.md#present-state-and-roadmap).
