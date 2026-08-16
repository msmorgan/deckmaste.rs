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
