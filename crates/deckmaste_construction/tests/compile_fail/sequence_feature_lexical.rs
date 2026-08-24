use deckmaste_construction::constructions;

constructions! {
    vocab Word { One = "one", }
    construction invalid: Root {
        element LexicalSequence { members: seq lex Word separated by " ", }
        require len(members) >= 2;
        derive agreement = members.agreement;
        form invalid = lex(members);
    }
    root Root { punctuation = "."; eoi = true; standalone_render = true; }
}

fn main() {}
