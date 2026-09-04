use deckmaste_construction::constructions;

constructions! {
    construction item: Item {
        element ItemValue {}
        derive concord_class = Values::Other;
        derive number = Values::Singular;
        form item = "item";
    }
    construction invalid: Root {
        element DuplicateNumberSequence {
            source: Item,
            members: seq Item separated by " ",
        }
        require len(members) >= 2;
        derive number = members.number;
        derive source.number = members.number;
        form invalid = source members;
    }
    root Root { punctuation = "."; eoi = true; standalone_render = true; }
}

fn main() {}
