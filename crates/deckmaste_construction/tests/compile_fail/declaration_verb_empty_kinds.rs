use deckmaste_construction::constructions;

constructions! {
    codec Verb {
        generate declaration_verb {
            position = Verb;
            kinds = [];
            tail = [];
            feature = Agreement;
        }
    }
}

fn main() {}
