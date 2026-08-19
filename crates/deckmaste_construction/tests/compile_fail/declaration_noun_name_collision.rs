use deckmaste_construction::constructions;

constructions! {
    lexeme NounLexeme { Player, }
    lexeme DeclarationNoun { Intruder, }
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
