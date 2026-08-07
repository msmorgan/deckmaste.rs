pub struct Token;

pub struct Owner {
    pub head: Token,
}

deckmaste_constructions_macro::constructions! {
    group lens_unknown_path;

    lens owner bind Owner {
        head: value Token,
    }

    construction bad: Owner {
        bind Owner {
            head: hole Token,
        }
        lens owner {
            focus missing with head;
        }
        form only @ 0 = head;
    }
}

fn main() {}
