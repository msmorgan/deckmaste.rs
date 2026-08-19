use deckmaste_construction::constructions;

constructions! {
    identity SelfReferenceSpelling {
        generate context {
            Full => card_name,
            Abbreviated => display_name,
            canonical_on_collision = Full;
        }
    }
}

fn main() {}
