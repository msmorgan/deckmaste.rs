use deckmaste_construction::constructions;

constructions! {
    identity SelfReferenceSpelling {
        generate context {
            _ => card_name,
            Abbreviated => abbreviated_card_name,
            canonical_on_collision = Abbreviated;
        }
    }
}

fn main() {}
