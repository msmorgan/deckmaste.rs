use deckmaste_construction::constructions;

constructions! {
    identity SelfReferenceSpelling {
        generate context { canonical_on_collision = Full; }
    }
}

fn main() {}
