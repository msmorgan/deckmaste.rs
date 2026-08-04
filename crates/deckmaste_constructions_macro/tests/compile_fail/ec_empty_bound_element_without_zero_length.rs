pub enum BoundPayload {
    Present,
}

pub struct EmptyBoundSequence {
    pub payloads: Vec<BoundPayload>,
}

deckmaste_constructions_macro::constructions! {
    group empty_without_proof;

    element empty_payload bind BoundPayload {}

    construction empty_bound_sequence: FixturePair {
        bind EmptyBoundSequence {
            payloads: seq empty_payload,
        }
        form only @ 0 = payloads;
    }
}

fn main() {}
