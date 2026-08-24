mod empty_prefix {
    deckmaste_construction::constructions! {
        construction item: Item { element ItemValue {} form item = "item"; }
        construction invalid: Root {
            element Invalid { value: Item, }
            form invalid = circumfix("", value, "]");
        }
    }
}

mod empty_suffix {
    deckmaste_construction::constructions! {
        construction item: Item { element ItemValue {} form item = "item"; }
        construction invalid: Root {
            element Invalid { value: Item, }
            form invalid = circumfix("[", value, "");
        }
    }
}

mod whitespace_prefix {
    deckmaste_construction::constructions! {
        construction item: Item { element ItemValue {} form item = "item"; }
        construction invalid: Root {
            element Invalid { value: Item, }
            form invalid = circumfix("[ ", value, "]");
        }
    }
}

mod whitespace_suffix {
    deckmaste_construction::constructions! {
        construction item: Item { element ItemValue {} form item = "item"; }
        construction invalid: Root {
            element Invalid { value: Item, }
            form invalid = circumfix("[", value, " ]");
        }
    }
}

fn main() {}
