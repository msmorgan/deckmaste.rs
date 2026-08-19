use deckmaste_construction::constructions;

constructions! {
    vocab Article { A = "a", }
    codec Noun {
        generate declaration_noun {
            closed = Article;
            position = Noun;
            kinds = [Type, Subtype];
            feature = Number;
        }
    }
}

fn main() {}
