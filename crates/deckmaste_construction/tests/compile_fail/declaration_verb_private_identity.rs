#![allow(
    dead_code,
    reason = "the compiled consumer supplies the complete generated-code environment"
)]

#[path = "../compiled_consumer.rs"]
mod compiled_consumer;

pub use compiled_consumer::environment;

fn forge(id: macro_ron::v2::DeclarationIdentity) {
    let _ = compiled_consumer::declaration_verb_fixture::DeclarationTransitiveVerb { id };
}

fn main() {}
