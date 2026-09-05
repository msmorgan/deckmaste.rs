use deckmaste_construction::constructions;

constructions! {
    construction child: Child {
        element ChildNode {}
        form child = "child";
    }
    construction invalid: Root {
        element Invalid {
            prefix: Child,
            mobile_left: mobile(scope) Child,
            separator: Child,
            scope: Child,
        }
        form invalid = prefix mobile_left separator scope;
    }
    root Root { punctuation = "."; eoi = true; standalone_render = true; }
}

fn main() {}
