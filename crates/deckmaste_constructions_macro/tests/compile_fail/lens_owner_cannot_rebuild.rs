#[derive(Clone)]
pub struct Token;

#[derive(Clone)]
pub struct Owner {
    pub head: Token,
    pub required_tail: Token,
}

deckmaste_constructions_macro::constructions! {
    group lens_owner_cannot_rebuild;

    lens owner bind Owner {
        head: value Token,
        required_tail: value Token,
    }

    construction bad: Owner {
        bind Owner {
            head: hole Token,
        }
        lens owner {
            focus head with head;
        }
        form only @ 0 = head;
    }
}

fn main() {}
