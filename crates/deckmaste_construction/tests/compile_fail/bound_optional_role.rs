use deckmaste_construction::constructions;

constructions! {
    construction item: Item {
        element ItemValue {}
        form item = "item";
    }
    construction invalid: Root {
        element Invalid { maybe: opt Item, }
        form invalid = prefix("non", maybe);
    }
    root Root { punctuation = "."; eoi = true; standalone_render = true; }
}

fn main() {}
