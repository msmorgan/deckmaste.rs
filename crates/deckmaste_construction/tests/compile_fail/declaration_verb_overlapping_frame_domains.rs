use deckmaste_construction::constructions;

constructions! {
    codec FirstVerb {
        generate declaration_verb {
            position = Verb;
            kinds = [KeywordAction];
            tail = [ObjectNounPhrase];
            feature = Agreement;
        }
    }
    codec LaterVerb {
        generate declaration_verb {
            position = Verb;
            kinds = [KeywordAction];
            tail = [ObjectNounPhrase];
            feature = Agreement;
        }
    }
}

fn main() {}
