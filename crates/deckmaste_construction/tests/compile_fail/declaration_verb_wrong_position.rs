use deckmaste_construction::constructions;

constructions! {
    codec Verb {
        generate declaration_verb {
            position = Noun;
            kinds = [KeywordAction];
            tail = [];
            feature = Agreement;
        }
    }
}

fn main() {}
