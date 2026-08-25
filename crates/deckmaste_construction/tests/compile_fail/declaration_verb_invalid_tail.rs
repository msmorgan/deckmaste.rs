use deckmaste_construction::constructions;

constructions! {
    codec Verb {
        generate declaration_verb {
            position = Verb;
            kinds = [KeywordAction];
            tail = [Clause];
            feature = Agreement;
        }
    }
}

fn main() {}
