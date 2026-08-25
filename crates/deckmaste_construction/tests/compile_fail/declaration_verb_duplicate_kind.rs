use deckmaste_construction::constructions;

constructions! {
    codec Verb {
        generate declaration_verb {
            position = Verb;
            kinds = [KeywordAction, KeywordAction];
            tail = [];
            feature = Agreement;
        }
    }
}

fn main() {}
