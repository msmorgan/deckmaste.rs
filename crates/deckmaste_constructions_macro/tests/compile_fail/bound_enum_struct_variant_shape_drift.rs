pub enum Choice {
    Named(u8, u8),
}

deckmaste_constructions_macro::constructions! {
    group struct_shape_drift;
    element choice bind Choice {
        variant Named { left: lex u8, right: lex u8 },
    }
}

fn main() {}
