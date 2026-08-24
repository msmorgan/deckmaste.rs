use deckmaste_construction::constructions;

constructions! {
    construction item: Item {
        element ItemValue {}
        derive agreement = Values::Bare;
        form item = "item";
    }
    construction invalid: Root {
        element EmptySequence { members: seq Item separated by " ", }
        derive agreement = members.agreement;
        form invalid = members;
    }
    root Root { punctuation = "."; eoi = true; standalone_render = true; }
}

fn main() {}
