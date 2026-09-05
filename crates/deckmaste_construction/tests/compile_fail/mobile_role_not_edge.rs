use deckmaste_construction::constructions;

constructions! {
    construction child: Child {
        element ChildNode {}
        form child = "child";
    }
    construction invalid: Root {
        element Invalid { prefix: Child, mobile_middle: mobile Child, suffix: Child, }
        form invalid = prefix mobile_middle suffix;
    }
    root Root { punctuation = "."; eoi = true; standalone_render = true; }
}

fn main() {}
