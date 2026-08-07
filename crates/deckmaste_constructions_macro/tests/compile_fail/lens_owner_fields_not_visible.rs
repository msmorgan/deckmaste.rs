#[derive(Clone)]
pub struct Token;

mod model {
    use super::Token;

    #[derive(Clone)]
    pub struct Owner {
        head: Token,
    }
}

deckmaste_constructions_macro::constructions! {
    group lens_owner_fields_not_visible;

    lens owner bind model::Owner {
        head: value Token,
    }

    construction focus: OwnerCategory {
        bind model::Owner {
            head: hole Token,
        }
        lens owner {
            focus head with head;
        }
        form only @ 0 = head;
        selection unique;
    }
}

fn main() {}
