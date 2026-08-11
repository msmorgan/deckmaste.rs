enum Choice {
    One,
    Two,
}

deckmaste_constructions_macro::constructions! {
    group bad_sum_guard;
    element choice bind Choice {
        variant One {},
        variant Two {},
    }
    construction root: Root {
        own RootNode { value: sum choice, }
        form bad @ 0 when value.variant in [Missing] = value;
        form fallback @ 1 otherwise = value;
        selection unique;
    }
}

fn main() {}
