use deckmaste_construction::constructions;

constructions! {
    construction item: Item {
        element ItemValue {}
        form item = "item";
    }
    construction invalid: Root {
        element Invalid { items: seq Item, }
        form invalid = suffix(items, "'s");
    }
    root Root { punctuation = "."; eoi = true; standalone_render = true; }
}

fn main() {}
