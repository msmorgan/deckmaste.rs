use deckmaste_construction::constructions;

constructions! {
    vocab Word { That = "that", }
    construction optional: Cat {
        element OptionalValue { word: opt lex Word, }
        require word.is_some(That);
        form optional = lex(word);
    }
    root Cat { punctuation = "."; eoi = true; standalone_render = true; }
}

fn main() {}
