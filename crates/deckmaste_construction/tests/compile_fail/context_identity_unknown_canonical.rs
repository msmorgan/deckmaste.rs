use deckmaste_construction::constructions;

constructions! {
    identity SelfReferenceSpelling {
        generate context {
            Full => card_name,
            Abbreviated => abbreviated_card_name,
            canonical_on_collision = Short;
        }
    }
}

fn main() {}
