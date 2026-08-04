pub enum BoundPayload {
    Present,
}

deckmaste_constructions_macro::constructions! {
    group nested_empty_without_proof;

    element empty_payload bind BoundPayload {}

    element container {
        payloads: seq empty_payload,
    }
}

fn main() {}
