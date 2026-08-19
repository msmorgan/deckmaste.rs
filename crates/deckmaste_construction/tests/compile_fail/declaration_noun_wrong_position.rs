use deckmaste_construction::constructions;

constructions! {
    lexeme NounLexeme { Player, }
    codec Noun {
        generate declaration_noun {
            closed = NounLexeme;
            position = Verb;
            kinds = [Type, Subtype];
            feature = Number;
        }
    }
}

fn main() {}
