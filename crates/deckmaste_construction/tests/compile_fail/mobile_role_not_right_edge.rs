use deckmaste_construction::constructions;

constructions! {
    construction child: Child {
        element ChildNode {}
        form child = "child";
    }
    construction invalid: Root {
        element Invalid { mobile_tail: mobile Child, suffix: Child, }
        form invalid = mobile_tail suffix;
    }
    root Root { punctuation = "."; eoi = true; standalone_render = true; }
}

fn main() {}
