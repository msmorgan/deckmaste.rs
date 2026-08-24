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

mod identity_role {
    deckmaste_construction::constructions! {
        identity CircumfixIdentity {
            generate context {
                Full => card_name,
                Abbreviated => abbreviated_card_name,
                canonical_on_collision = Full;
            }
        }
        construction invalid: Root {
            element Invalid { value: identity CircumfixIdentity, }
            form invalid = circumfix("[", value, "]");
        }
    }
}

mod noun_role {
    deckmaste_construction::constructions! {
        codec NounValue {
            atom = noun;
            value_type = NounValue;
            lexical = Lexical::NounValue;
            render = render_noun_value;
            build { pattern = BuildValue::NounValue(value); construct = value; }
            traversal {
                callback = borrowed;
                argument = value;
                call visitor::visit_noun_value(borrowed(value));
            }
        }
        construction invalid: Root {
            element Invalid { value: lex NounValue, }
            form invalid = circumfix("[", value, "]");
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
