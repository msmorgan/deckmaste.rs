use deckmaste_construction::constructions;

constructions! {
    identity FirstName {
        generate catalog_identity { provider = CardNames; }
    }
    identity SecondName {
        generate catalog_identity { provider = r#CardNames; }
    }
    construction sample: Root {
        element Sample {
            first: identity FirstName,
            second: identity SecondName,
        }
        form sample = identity(first) identity(second);
    }
    root Root { punctuation = "."; eoi = true; standalone_render = true; }
}

fn main() {}
