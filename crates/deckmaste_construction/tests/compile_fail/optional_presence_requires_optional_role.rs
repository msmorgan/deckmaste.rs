use deckmaste_construction::constructions;

constructions! {
    vocab Word { That = "that", }
    construction required: Cat {
        element RequiredValue { word: lex Word, }
        require word.is_none();
        form required = lex(word);
    }
    root Cat { punctuation = "."; eoi = true; standalone_render = true; }
}

fn main() {}
