#![allow(
    dead_code,
    reason = "the compiled consumer supplies the complete generated-code environment"
)]

#[path = "../compiled_consumer.rs"]
mod compiled_consumer;

pub use compiled_consumer::environment;

fn main() {
    let _ = compiled_consumer::fixture::BareDirectOuterMixedRelayEnvelope {
        choice: compiled_consumer::fixture::OuterRelayedMixedChoice::RelayedMixedChoiceSequence(
            compiled_consumer::fixture::RelayedMixedChoiceSequence::IntrinsicThirdMixedChoice(
                compiled_consumer::fixture::IntrinsicThirdMixedChoice,
            ),
        ),
    };
}
