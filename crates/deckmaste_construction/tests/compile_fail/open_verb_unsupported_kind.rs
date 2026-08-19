use deckmaste_construction::constructions;

constructions! {
    construction unsupported: VerbPhrase {
        element Unsupported {}
        form unsupported = open_verb(Subtype, "Elf");
    }
    root VerbPhrase { punctuation = "."; eoi = true; standalone_render = true; }
}

fn main() {}
