use deckmaste_construction::constructions;

constructions! {
    morphology EnglishNoun { feature = Number; recipe = english_noun; }
    lexeme NounLexeme using EnglishNoun { Player = "player", }
    codec Noun {
        value_type = Noun;
        generate declaration_noun {
            closed = NounLexeme;
            position = Noun;
            kinds = [Type, Subtype];
            feature = Number;
        }
    }
}

fn main() {}
