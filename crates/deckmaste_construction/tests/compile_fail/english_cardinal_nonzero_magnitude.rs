use deckmaste_construction::constructions;

constructions! {
    codec CardinalNumber {
        generate english_cardinal {
            magnitude = NonZeroU32;
        }
    }
    construction only: Cat { element Only {} form only = "only"; }
    root Cat { punctuation = "."; eoi = true; standalone_render = true; }
}

fn main() {}
