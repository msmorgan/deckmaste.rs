mod extra_operand {
    deckmaste_construction::constructions! {
        construction invalid: Root {
            element Invalid { value: Root, }
            form invalid = circumfix("[", value, "]", "extra");
        }
    }
}

mod nested_bound {
    deckmaste_construction::constructions! {
        construction invalid: Root {
            element Invalid { value: Root, }
            form invalid = circumfix("[", prefix("non", value), "]");
        }
    }
}

mod nested_circumfix {
    deckmaste_construction::constructions! {
        construction invalid: Root {
            element Invalid { value: Root, }
            form invalid = circumfix("[", circumfix("{", value, "}"), "]");
        }
    }
}

mod circumfix_inside_bound {
    deckmaste_construction::constructions! {
        construction invalid: Root {
            element Invalid { value: Root, }
            form invalid = prefix("non", circumfix("[", value, "]"));
        }
    }
}

fn main() {}
