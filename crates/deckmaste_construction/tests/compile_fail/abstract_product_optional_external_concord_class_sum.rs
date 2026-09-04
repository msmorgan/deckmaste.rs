use deckmaste_construction::constructions;

constructions! {
    morphology EnglishVerb { feature = ConcordClass; recipe = english_verb; }
    lexeme Verbs using EnglishVerb { Act = "act", }
    construction contextual: Child {
        element ContextualChild {}
        derive concord_class = verb.concord_class;
        form contextual = verb(Verbs::Act);
    }
    abstract sum Choice { Child, }
    abstract product Holder { optional: opt Choice, }
    construction anchor: Root { element Anchor {} form anchor = "anchor"; }
    root Root { punctuation = "."; eoi = true; standalone_render = true; }
}

fn main() {}
