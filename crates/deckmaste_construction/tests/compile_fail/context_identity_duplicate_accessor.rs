use deckmaste_construction::constructions;

constructions! {
    identity SelfReferenceSpelling {
        generate context {
            Full => card_name,
            Abbreviated => card_name,
            canonical_on_collision = Full;
        }
    }
}

fn main() {}
