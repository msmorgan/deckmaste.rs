use deckmaste_construction::constructions;

constructions! {
    morphology EnglishVerb { feature = Agreement; recipe = english_verb; }
    lexeme Verbs using EnglishVerb { Act = "act", }
    construction contextual: Child {
        element ContextualChild {}
        derive agreement = verb.agreement;
        form contextual = verb(Verbs::Act);
    }
    abstract sum Choice { Child, }
    abstract product Holder { members: seq Choice separated by " ", }
    construction anchor: Root { element Anchor {} form anchor = "anchor"; }
    root Root { punctuation = "."; eoi = true; standalone_render = true; }
}

fn main() {}
