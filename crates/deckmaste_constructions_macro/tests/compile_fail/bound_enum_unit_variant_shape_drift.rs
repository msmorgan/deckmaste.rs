pub enum Choice {
    Unit(u8),
}

deckmaste_constructions_macro::constructions! {
    group unit_shape_drift;
    element choice bind Choice {
        variant Unit {},
    }
}

fn main() {}
