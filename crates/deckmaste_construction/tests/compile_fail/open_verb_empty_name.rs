use deckmaste_construction::constructions;

constructions! {
    construction unnamed: VerbPhrase {
        element Unnamed {}
        form unnamed = open_verb(KeywordAction, "");
    }
    root VerbPhrase { punctuation = "."; eoi = true; standalone_render = true; }
}

fn main() {}
