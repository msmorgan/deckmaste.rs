use deckmaste_construction::constructions;

constructions! {
    morphology EnglishVerb {
        feature = Agreement;
        recipe = english_verb;
    }
    lexeme VerbLexeme using EnglishVerb {
        Deal = "deal" {
            Bare = "deal",
            Bare = "deals",
        },
    }
}

fn main() {}
