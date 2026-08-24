use deckmaste_construction::constructions;

constructions! {
    abstract sum Choice { Left: LeftNode, Other: ForeignNode, }
    construction left: Choice {
        element LeftNode {}
        form left = "left";
    }
    root Choice { punctuation = "."; eoi = true; standalone_render = true; }
}

fn main() {}
