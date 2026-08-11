enum Choice {
    Item(String),
}

deckmaste_constructions_macro::constructions! {
    group bad_product;
    element choice bind Choice {
        variant Item: lex String,
    }
    construction root: Root {
        own RootNode { value: product choice, }
        form only @ 0 = value;
    }
}

fn main() {}
