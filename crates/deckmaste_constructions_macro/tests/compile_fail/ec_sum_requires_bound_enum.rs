deckmaste_constructions_macro::constructions! {
    group bad_sum;
    element record { value: lex u8, }
    construction root: Root {
        own RootNode { value: sum record, }
        form only @ 0 = value;
    }
}

fn main() {}
