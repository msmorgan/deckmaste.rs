use deckmaste_construction::constructions;

constructions! {
    identity Spelling {
        generate context {
            Full => card_name,
            Abbreviated => abbreviated_card_name,
            canonical_on_collision = Full;
        }
    }
    construction sample: Root {
        element Sample { spelling: identity Spelling, }
        form sample = identity(spelling);
    }
    root Root { punctuation = "."; eoi = true; standalone_render = true; }
}

fn main() {}
