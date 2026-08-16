use quote::quote;

pub(crate) fn representative_expansion() -> crate::Expansion {
    crate::generate(quote! {
        vocab Words { First = "first", Second = "second", }
        lexeme Nouns { Person, }
        lexeme Verbs { Act, }

        construction leaf: Node {
            element Leaf { word: lex Words, }
            form leaf = lex(word);
        }
        construction chain: Node {
            element Chain { next: Node, word: lex Words, }
            checked {
                visibility next = private;
                access next = next;
                visibility word = pub(crate);
                constructor = Chain::new(next, word);
            }
            form chain = next lex(word);
        }
        construction action: Action {
            element ActionElement { node: Node, }
            derive agreement = verb.agreement;
            form action = verb(Verbs::Act) node;
        }

        root Action { punctuation = "."; eoi = true; standalone_render = true; }
    })
    .expect("representative declarations generate")
}

pub(crate) fn access_modes_expansion() -> crate::Expansion {
    crate::generate(quote! {
        identity Flag {
            value_type = Flag;
            lexical = Lexical::Flag;
            render context_identity {
                Full => card_name,
                Short => abbreviated_card_name,
            }
            build { pattern = BuildValue::Flag(flag); construct = flag; }
            traversal {
                callback = copy;
                argument = flag;
                variant Full;
                variant Short;
            }
        }

        construction child: Child {
            element ChildElement {}
            form child = "child";
        }
        construction public: Root {
            element PublicIdentity { flag: identity Flag, }
            form public = identity(flag);
        }
        construction mixed: Root {
            element MixedAccess { hidden: identity Flag, child: Child, }
            checked {
                visibility hidden = private;
                access hidden = hidden;
                visibility child = pub(crate);
                constructor = MixedAccess::new(hidden, child);
            }
            form mixed = identity(hidden) child;
        }
        root Root { punctuation = "."; eoi = true; standalone_render = true; }
    })
    .expect("copy identity and mixed checked access declarations generate")
}
