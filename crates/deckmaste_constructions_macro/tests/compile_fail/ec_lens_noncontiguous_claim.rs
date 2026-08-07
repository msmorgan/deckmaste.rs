pub struct Token;

pub struct Owner {
    pub left: Token,
    pub middle: Option<Token>,
    pub right: Token,
}

deckmaste_constructions_macro::constructions! {
    group lens_noncontiguous_claim;

    lens owner bind Owner {
        left: value Token,
        middle: opt Token,
        right: value Token,
    }

    construction bad: Owner {
        bind Owner {
            left: hole Token,
            right: hole Token,
        }
        lens owner {
            focus left with left;
            focus right with right;
        }
        form only @ 0 = left right;
    }
}

fn main() {}
