use deckmaste_construction::constructions;

constructions! {
    construction item: Item {
        element ItemValue {}
        derive number = Values::Singular;
        form item = "item";
    }
    construction invalid: Root {
        element UnsupportedSequence { members: seq Item separated by " ", }
        require len(members) >= 2;
        derive number = members.number;
        form invalid = members;
    }
    root Root { punctuation = "."; eoi = true; standalone_render = true; }
}

fn main() {}
