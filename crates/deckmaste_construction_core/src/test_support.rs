use quote::quote;

pub(crate) fn representative_tokens() -> proc_macro2::TokenStream {
    quote! {
        vocab Words { First = "first", Second = "second", }
        lexeme Nouns { Person, }
        lexeme Verbs { Act, }
        codec SignedNumber {
            generate signed_decimal {
                magnitude = u32;
                sign_type = Sign { Positive = none, Negative = "-", };
            }
        }

        construction leaf: Node {
            element WordLeaf { word: lex Words, }
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
            derive verb.agreement = Values::Bare;
            form action = verb(Verbs::Act) node;
        }

        root Action { punctuation = "."; eoi = true; standalone_render = true; }
    }
}

pub(crate) fn representative_expansion() -> crate::Expansion {
    crate::generate(representative_tokens()).expect("representative declarations generate")
}

pub(crate) fn representative_semantic_plan() -> crate::semantic::SemanticPlan {
    crate::validate_declarations(
        crate::parse_declarations(quote! {
            construction first: Node {
                element First {}
                form first = "first";
            }
            vocab Words { First = "first", }
            root Node { punctuation = "."; eoi = true; standalone_render = true; }
        })
        .expect("representative semantic-plan declarations parse"),
    )
    .expect("representative semantic-plan declarations validate")
    .into_semantic()
}

pub(crate) fn open_verb_tokens() -> proc_macro2::TokenStream {
    quote! {
        construction destroy: VerbPhrase {
            element Destroy { object: NounPhrase, }
            derive agreement = verb.agreement;
            form destroy = open_verb(KeywordAction, "Destroy") object;
        }
        construction object: NounPhrase {
            element Object {}
            form object = "object";
        }
        construction imperative: Ability {
            element Imperative { predicate: VerbPhrase, }
            derive predicate.agreement = Values::Bare;
            form imperative = predicate;
        }
        root Ability { punctuation = "."; eoi = true; standalone_render = true; }
    }
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

pub(crate) fn synthetic_projection_tokens() -> proc_macro2::TokenStream {
    quote! {
        vocab Mode { Solo = "solo", Group = "group", }
        lexeme ObjectStem { Widget, }
        lexeme ActionStem { Activate, }

        codec Resource {
            atom = noun;
            value_type = Resource;
            lexical = Lexical::Resource;
            render = render_resource;
            build { pattern = BuildValue::Resource(resource); construct = resource; }
            traversal {
                callback = borrowed;
                argument = resource;
                variant Builtin;
                variant External;
                match resource {
                    Resource::Builtin(stem: ObjectStem) => walk_object_stem(copy(stem)),
                    Resource::External(record: Record) => walk_record(borrowed(record)),
                }
            }
        }
        codec Marker {
            atom = lex;
            value_type = Marker;
            lexical = Lexical::Marker;
            render = render_marker;
            build { pattern = BuildValue::Marker(marker); construct = marker; }
            traversal {
                callback = copy;
                argument = marker;
                variant Open;
                variant Closed;
            }
        }
        identity Handle {
            value_type = Handle;
            lexical = Lexical::Handle;
            render context_identity {
                Primary => primary_name,
                Alias => alias_name,
            }
            build { pattern = BuildValue::Handle(handle); construct = handle; }
            traversal {
                callback = copy;
                argument = handle;
                variant Primary;
                variant Alias;
            }
        }
        codec Pair {
            atom = lex;
            value_type = Pair;
            lexical = Lexical::Pair;
            render = render_pair;
            build {
                pattern = BuildValue::Pair(left, right);
                construct = RuntimePair::new(left, Factory::wrap(right));
            }
            traversal {
                callback = borrowed;
                argument = pair;
                field left: i32;
                field right: i32;
                call visitor::visit_pair(borrowed(pair));
            }
        }
        identity Record {
            value_type = Record;
            traversal {
                callback = borrowed;
                argument = record;
                leaf visit_record_label: str = borrowed;
                field kind: RecordKind;
                field label: str;
                call visitor::visit_record(borrowed(record));
                call visitor::visit_record_label(borrowed(label));
            }
        }

        construction leaf: Expr {
            element LeafNode { mode: lex Mode, resource: lex Resource, }
            derive agreement = match mode {
                Solo => Values::ThirdPersonSingular,
                Group => Values::Bare,
            };
            derive number = match mode {
                Solo => Values::Singular,
                Group => Values::Plural,
            };
            form leaf = lex(mode) noun(resource);
        }
        construction nested: Expr {
            element NestedNode { next: Expr, marker: lex Marker, }
            checked {
                visibility next = private;
                access next = next;
                visibility marker = pub(crate);
                constructor = NestedNode::checked(next, marker);
            }
            derive agreement = next.agreement;
            derive number = next.number;
            form nested = "nest" next lex(marker);
        }
        construction action: Predicate {
            element ActionNode {}
            derive agreement = verb.agreement;
            form action = verb(ActionStem::Activate);
        }
        construction idle: Predicate {
            element IdleNode {}
            derive agreement = Values::Bare;
            form idle = "idle";
        }
        construction solo: Tag {
            element SoloTag { mode: lex Mode, }
            require mode is Solo;
            form solo = lex(mode);
        }
        construction document: Document {
            element DocumentNode {
                subject: Expr,
                predicate: Predicate,
                handle: identity Handle,
                pair: lex Pair,
            }
            checked {
                visibility subject = pub(crate);
                visibility predicate = pub(crate);
                visibility handle = private;
                access handle = handle;
                visibility pair = private;
                access pair = pair;
                constructor = DocumentNode::checked(subject, predicate, handle, vec![pair]);
            }
            require subject is Leaf;
            derive predicate.agreement = subject.agreement;
            form document = subject predicate identity(handle) lex(pair);
        }

        root Document { punctuation = "!"; eoi = true; standalone_render = true; }
    }
}

pub(crate) fn synthetic_projection_expansion() -> crate::Expansion {
    crate::generate(synthetic_projection_tokens()).expect("synthetic projection fixture generates")
}

pub(crate) fn role_derived_noun_tokens() -> proc_macro2::TokenStream {
    quote! {
        codec Head {
            atom = noun;
            value_type = Head;
            lexical = Lexical::Head;
            render = render_head;
            build { pattern = BuildValue::Head(head); construct = head; }
            traversal {
                callback = borrowed;
                argument = head;
                call visitor::visit_head(borrowed(head));
            }
        }

        construction source: Source {
            element SourceNode {}
            derive number = Values::Singular;
            form source = "source";
        }
        construction phrase: Phrase {
            element PhraseNode { source: Source, head: lex Head, }
            derive number = source.number;
            form phrase = source noun(head);
        }

        root Phrase { punctuation = "."; eoi = true; standalone_render = true; }
    }
}

pub(crate) fn vocab_matched_number_without_noun_tokens() -> proc_macro2::TokenStream {
    quote! {
        vocab Count { One = "one", Many = "many", }

        construction source: Source {
            element SourceNode { count: lex Count, }
            derive agreement = match count {
                One => Values::ThirdPersonSingular,
                Many => Values::Bare,
            };
            derive number = match count {
                One => Values::Singular,
                Many => Values::Plural,
            };
            form source = lex(count);
        }
        construction phrase: Phrase {
            element PhraseNode { source: Source, }
            derive agreement = source.agreement;
            derive number = source.number;
            form phrase = source;
        }

        root Phrase { punctuation = "."; eoi = true; standalone_render = true; }
    }
}

pub(crate) fn vocab_matched_number_with_two_nouns_tokens() -> proc_macro2::TokenStream {
    quote! {
        vocab Count { One = "one", Many = "many", }
        codec Head {
            atom = noun;
            value_type = Head;
            lexical = Lexical::Head;
            render = render_head;
            build { pattern = BuildValue::Head(head); construct = head; }
            traversal {
                callback = borrowed;
                argument = head;
                call visitor::visit_head(borrowed(head));
            }
        }

        construction pair: Phrase {
            element PairNode { count: lex Count, left: lex Head, right: lex Head, }
            derive number = match count {
                One => Values::Singular,
                Many => Values::Plural,
            };
            form pair = lex(count) noun(left) noun(right);
        }

        root Phrase { punctuation = "."; eoi = true; standalone_render = true; }
    }
}

#[test]
fn synthetic_projection_fixture_generates() {
    let expansion = synthetic_projection_expansion();
    assert_eq!(expansion.plan().items().len(), 71);
    assert!(expansion.items().iter().any(|item| {
        matches!(
            &item.key,
            crate::ItemKey::Named {
                kind: crate::NamedKind::Function,
                name,
            } if name == "scan_lexical"
        )
    }));
}

#[test]
fn role_derived_noun_fixture_generates_all_phases() {
    let expansion = crate::generate(role_derived_noun_tokens())
        .expect("the accepted role-derived noun composition is backend-complete");
    assert!(
        expansion
            .items()
            .iter()
            .any(|item| item.tokens.to_string().contains("number_for_source"))
    );
}

#[test]
fn vocab_matched_number_without_noun_generates_all_phases() {
    let expansion = crate::generate(vocab_matched_number_without_noun_tokens())
        .expect("a stored vocab match does not require a noun scanner binder");
    let source = expansion
        .items()
        .iter()
        .map(|item| item.tokens.to_string())
        .collect::<Vec<_>>()
        .join("\n");
    assert!(source.contains("number_for_source"), "{source}");
}

#[test]
fn vocab_matched_number_with_two_nouns_generates_all_phases() {
    crate::generate(vocab_matched_number_with_two_nouns_tokens())
        .expect("every dynamic noun number is lowerable");
}
