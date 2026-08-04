pub enum BoundPayload {
    Present,
}

pub struct OptionalBoundSequence {
    pub payloads: Option<Vec<BoundPayload>>,
}

deckmaste_constructions_macro::constructions! {
    group optional_empty_without_proof;

    element empty_payload bind BoundPayload {}

    construction optional_bound_sequence: FixturePair {
        bind OptionalBoundSequence {
            payloads: opt seq empty_payload,
        }
        form only @ 0 = payloads;
    }
}

fn main() {}
