use deckmaste_construction::constructions;

constructions! {
    codec Verb {
        generate declaration_verb {
            position = Verb;
            kinds = [KeywordAction];
            tail = [];
            feature = Number;
        }
    }
}

fn main() {}
