#![allow(unused_imports, reason = "the macro validates the imported value type")]

use deckmaste_features::Comma;

pub struct Token;

pub struct Owner {
    pub members: Vec<Token>,
}

deckmaste_constructions_macro::constructions! {
    group lens_element_type_mismatch;

    lens owner bind Owner {
        members: vec Token,
    }

    construction bad: Owner {
        bind Owner {
            owner: hole Owner,
            member: lex Comma,
        }
        lens owner from owner {
            prepend members with member;
        }
        form only @ 0 = owner lex(member);
    }
}

fn main() {}
