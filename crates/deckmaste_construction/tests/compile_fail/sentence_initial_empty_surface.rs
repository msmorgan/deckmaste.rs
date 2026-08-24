use deckmaste_construction::constructions;

constructions! {
    construction invalid: Root {
        element Invalid {}
        form invalid = "item" sentence_initial("");
    }
    root Root { punctuation = "."; eoi = true; standalone_render = true; }
}

fn main() {}
