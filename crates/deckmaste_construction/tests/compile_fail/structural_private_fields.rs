#![allow(
    dead_code,
    reason = "the compiled consumer supplies the complete generated-code environment"
)]

#[path = "../compiled_consumer.rs"]
mod compiled_consumer;

pub use compiled_consumer::environment;

fn main() {
    let _ = compiled_consumer::fixture::Holder {
        maybe: None,
        items: Vec::new(),
    };
}
