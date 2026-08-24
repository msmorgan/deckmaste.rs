use deckmaste_construction::constructions;

constructions! {
    construction item: Item {
        element ItemValue {}
        derive agreement = Values::Bare;
        derive number = Values::Singular;
        form item = "item";
    }
    construction invalid: Root {
        element MixedSequence { members: seq Item separated by " ", }
        require len(members) >= 2;
        derive agreement = members.agreement;
        derive number = members.number;
        form invalid = members;
    }
    root Root { punctuation = "."; eoi = true; standalone_render = true; }
}

fn main() {}
