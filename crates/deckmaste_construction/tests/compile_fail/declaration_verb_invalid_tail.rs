use deckmaste_construction::constructions;

constructions! {
    codec Verb {
        generate declaration_verb {
            position = Verb;
            tail = ["if"?];
            feature = ConcordClass;
        }
    }
}

fn main() {}
