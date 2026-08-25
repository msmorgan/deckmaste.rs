use deckmaste_construction::constructions;

constructions! {
    vocab Words { Act = "act", }
    construction projected: Cat {
        element Projected { word: lex Words, }
        form projected = verb(word);
    }
    root Cat { punctuation = "."; eoi = true; standalone_render = true; }
}

fn main() {}
