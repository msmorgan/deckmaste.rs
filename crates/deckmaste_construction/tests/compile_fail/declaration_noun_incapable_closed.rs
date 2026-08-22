use deckmaste_construction::constructions;

constructions! {
    vocab Word { A = "a", }
    codec Noun {
        generate declaration_noun {
            closed = Word;
            position = Noun;
            kinds = [Type, Subtype];
            feature = Number;
        }
    }
}

fn main() {}
