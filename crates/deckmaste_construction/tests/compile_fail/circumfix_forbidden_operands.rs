mod literal {
    deckmaste_construction::constructions! {
        construction invalid: Root {
            element Invalid {}
            form invalid = circumfix("[", "value", "]");
        }
    }
}

mod lexical_atom {
    deckmaste_construction::constructions! {
        construction invalid: Root {
            element Invalid { value: Root, }
            form invalid = circumfix("[", lex(value), "]");
        }
    }
}

mod fixed_verb {
    deckmaste_construction::constructions! {
        construction invalid: Root {
            element Invalid {}
            form invalid = circumfix("[", verb(Verbs::Act), "]");
        }
    }
}

mod callback {
    deckmaste_construction::constructions! {
        construction invalid: Root {
            element Invalid { value: Root, }
            form invalid = circumfix("[", callback(value), "]");
        }
    }
}

mod open_declaration {
    deckmaste_construction::constructions! {
        construction invalid: Root {
            element Invalid {}
            form invalid = circumfix("[", open_verb(KeywordAction, "act"), "]");
        }
    }
}

fn main() {}
