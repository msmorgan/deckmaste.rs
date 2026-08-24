use deckmaste_construction::constructions;

constructions! {
    abstract sum Choice { Left: LeftNode, }
    construction left: Choice {
        element LeftNode {}
        form left = "left";
    }
    construction right: Choice {
        element RightNode {}
        form right = "right";
    }
    root Choice { punctuation = "."; eoi = true; standalone_render = true; }
}

fn main() {}
