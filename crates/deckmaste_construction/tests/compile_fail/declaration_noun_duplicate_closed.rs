use deckmaste_construction::constructions;

constructions! {
    lexeme NounLexeme { Player, }
    codec Noun {
        generate declaration_noun {
            closed = NounLexeme;
            closed = NounLexeme;
            position = Noun;
            kinds = [Type, Subtype];
            feature = Number;
        }
    }
}

fn main() {}
