use deckmaste_construction::constructions;

constructions! {
    construction item: Item {
        element ItemValue {}
        form item = "item";
    }
    construction invalid: Root {
        element MissingProvider { members: seq Item separated by " ", }
        require len(members) >= 2;
        derive agreement = members.agreement;
        form invalid = members;
    }
    root Root { punctuation = "."; eoi = true; standalone_render = true; }
}

fn main() {}
