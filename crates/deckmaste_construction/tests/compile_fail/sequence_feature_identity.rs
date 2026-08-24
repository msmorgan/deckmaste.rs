use deckmaste_construction::constructions;

constructions! {
    identity HandleSpelling {
        generate context {
            Full => card_name,
            Abbreviated => abbreviated_card_name,
            canonical_on_collision = Full;
        }
    }
    construction invalid: Root {
        element IdentitySequence {
            members: seq identity HandleSpelling separated by " ",
        }
        require len(members) >= 2;
        derive agreement = members.agreement;
        form invalid = identity(members);
    }
    root Root { punctuation = "."; eoi = true; standalone_render = true; }
}

fn main() {}
