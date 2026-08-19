use deckmaste_construction::constructions;

constructions! {
    lexeme NounLexeme { Player, }
    codec Noun {
        generate declaration_noun {
            closed = NounLexeme;
            position = Noun;
            kinds = [Type, KeywordAction];
            feature = Number;
        }
    }
}

fn main() {}
