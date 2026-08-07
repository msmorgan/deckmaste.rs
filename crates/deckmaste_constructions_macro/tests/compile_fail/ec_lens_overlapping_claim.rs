pub struct Token;

pub struct Owner {
    pub members: Vec<Token>,
}

deckmaste_constructions_macro::constructions! {
    group lens_overlapping_claim;

    lens owner bind Owner {
        members: vec Token,
    }

    construction bad: Owner {
        bind Owner {
            owner: hole Owner,
            first: hole Token,
            second: hole Token,
        }
        lens owner from owner {
            prepend members with first;
            append members with second;
        }
        form only @ 0 = first owner second;
    }
}

fn main() {}
