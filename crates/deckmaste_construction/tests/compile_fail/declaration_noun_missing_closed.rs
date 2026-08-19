use deckmaste_construction::constructions;

constructions! {
    lexeme NounLexeme { Player, }
    codec Noun {
        generate declaration_noun {
            position = Noun;
            kinds = [Type, Subtype];
            feature = Number;
        }
    }
}

fn main() {}
