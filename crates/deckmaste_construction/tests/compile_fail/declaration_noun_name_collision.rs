use deckmaste_construction::constructions;

constructions! {
    morphology EnglishNoun { feature = Number; recipe = english_noun; }
    morphology EnglishVerb { feature = ConcordClass; recipe = english_verb; }
    lexeme NounLexeme using EnglishNoun { Player = "player", }
    lexeme DeclarationNoun using EnglishVerb { Intruder = "intruder", }
    codec Noun {
        generate declaration_noun {
            closed = NounLexeme;
            position = Noun;
            kinds = [Type, Subtype];
            feature = Number;
        }
    }
}

fn main() {}
