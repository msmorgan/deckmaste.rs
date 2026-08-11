pub enum Choice {
    Pair { left: u8, right: u8 },
}

deckmaste_constructions_macro::constructions! {
    group tuple_shape_drift;
    element choice bind Choice {
        variant Pair(left: lex u8, right: lex u8),
    }
}

fn main() {}
