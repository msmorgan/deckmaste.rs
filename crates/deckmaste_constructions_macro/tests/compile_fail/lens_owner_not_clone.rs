#[derive(Clone)]
pub struct Token;

pub struct Owner {
    pub prefix: Vec<Token>,
    pub head: Token,
}

deckmaste_constructions_macro::constructions! {
    group lens_owner_not_clone;

    lens owner bind Owner {
        prefix: vec Token,
        head: value Token,
    }

    construction extend: Owner {
        bind Owner {
            owner: hole Owner,
            member: hole Token,
        }
        lens owner from owner {
            prepend prefix with member;
        }
        form only @ 0 = member owner;
        selection unique;
    }
}

fn main() {}
