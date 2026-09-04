use deckmaste_construction::constructions;

constructions! {
    morphology EnglishVerb { feature = ConcordClass; recipe = english_verb; }
    lexeme DeclarationVerb using EnglishVerb { Intruder = "intrude", }
    codec Verb {
        generate declaration_verb {
            position = Verb;
            tail = [];
            feature = ConcordClass;
        }
    }
}

fn main() {}
