use deckmaste_construction::constructions;

constructions! {
    construction numbered: Item {
        element NumberedItem {}
        derive number = Values::Singular;
        form numbered = "numbered";
    }
    abstract sum NumberedChoice { Item, }
    construction unsupported: Root {
        element UnsupportedSumSequence {
            members: seq NumberedChoice separated by " ",
        }
        require len(members) >= 2;
        derive number = members.number;
        form unsupported = members;
    }
    root Root { punctuation = "."; eoi = true; standalone_render = true; }
}

fn main() {}
