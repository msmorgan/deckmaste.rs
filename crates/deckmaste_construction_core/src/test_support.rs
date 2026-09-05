use quote::quote;

pub(crate) fn representative_tokens() -> proc_macro2::TokenStream {
    quote! {
        vocab Words { First = "first", Second = "second", }
        lexeme Nouns using EnglishNoun { Person = "person", }
        lexeme Verbs using EnglishVerb { Act = "act", }
        codec SignedNumber {
            generate signed_decimal {
                magnitude = u32;
                sign_type = Sign { Positive = none, Negative = "-", };
            }
        }
        morphology EnglishNoun { feature = Number; recipe = english_noun; }
        morphology EnglishVerb { feature = ConcordClass; recipe = english_verb; }

        construction leaf: Node {
            element WordLeaf { word: lex Words, }
            form leaf = lex(word);
        }
        construction chain: Node {
            element Chain { next: Node, word: lex Words, }
            form chain = next lex(word);
        }
        construction action: Action {
            element ActionElement { node: Node, }
            derive concord_class = verb.concord_class;
            derive verb.concord_class = Values::Other;
            form action = verb(Verbs::Act) node;
        }

        root Action { punctuation = "."; eoi = true; standalone_render = true; }
    }
}

pub(crate) fn representative_expansion() -> crate::Expansion {
    crate::generate(representative_tokens()).expect("representative declarations generate")
}

pub(crate) fn generated_morphology_tokens() -> proc_macro2::TokenStream {
    quote! {
        vocab Pronoun { It = "it", You = "you", }
        morphology EnglishVerb { feature = ConcordClass; recipe = english_verb; }
        morphology EnglishNoun { feature = Number; recipe = english_noun; }
        lexeme VerbLexeme using EnglishVerb {
            InventedLemma = "deal",
            Be = "be" { Other = "are", ThirdPersonSingular = "is", },
        }
        lexeme NounLexeme using EnglishNoun { TwoWords = "object", }
        codec Noun {
            generate declaration_noun {
                closed = NounLexeme;
                position = Noun;
                kinds = [Type, Subtype];
                feature = Number;
            }
        }

        construction statement: Sentence {
            element Statement { pronoun: lex Pronoun, noun: lex Noun, }
            derive pronoun.concord_class = match pronoun {
                It => Values::ThirdPersonSingular,
                You => Values::Other,
            };
            derive verb.concord_class = pronoun.concord_class;
            derive number = Values::Plural;
            form statement = lex(pronoun) verb(VerbLexeme::InventedLemma) noun(noun);
        }
        construction question: Sentence {
            element Question { pronoun: lex Pronoun, }
            derive pronoun.concord_class = match pronoun {
                It => Values::ThirdPersonSingular,
                You => Values::Other,
            };
            derive verb.concord_class = pronoun.concord_class;
            derive number = Values::Singular;
            form question = verb(VerbLexeme::Be) lex(pronoun);
        }
        root Sentence { punctuation = "."; eoi = true; standalone_render = true; }
    }
}

pub(crate) fn generated_morphology_expansion() -> crate::Expansion {
    crate::generate(generated_morphology_tokens()).expect("generated morphology fixture generates")
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
            derive concord_class = verb.concord_class;
            form destroy = open_verb(KeywordAction, "Destroy") object;
        }
        construction object: NounPhrase {
            element Object {}
            form object = "object";
        }
        construction imperative: Ability {
            element Imperative { predicate: VerbPhrase, }
            derive predicate.concord_class = Values::Other;
            form imperative = predicate;
        }
        root Ability { punctuation = "."; eoi = true; standalone_render = true; }
    }
}

pub(crate) fn invariant_access_semantic_plan() -> crate::semantic::SemanticPlan {
    crate::validate_declarations(
        crate::parse_declarations(quote! {
            vocab Plain { Open = "open", }
            vocab Mode { One = "one", Two = "two", }
            vocab Marker { Marked = "marked", }
            identity SelfReferenceSpelling {
                generate context {
                    Full => card_name,
                    Abbreviated => abbreviated_card_name,
                    canonical_on_collision = Full;
                }
            }

            construction leaf: Node {
                element LeafNode {}
                derive concord_class = Values::Other;
                derive number = Values::Singular;
                form leaf = "leaf";
            }
            construction writer: Node {
                element WalkMode {
                    plain: lex Plain,
                    mode: lex Mode,
                    child: Node,
                    spelling: identity SelfReferenceSpelling,
                    visitor: lex Marker,
                }
                require mode is One;
                require child is Leaf;
                derive mode.concord_class = match mode {
                    One => Values::Other,
                    Two => Values::ThirdPersonSingular,
                };
                derive concord_class = mode.concord_class;
                derive number = child.number;
                form writer = lex(plain) lex(mode) child identity(spelling) lex(visitor);
            }
            construction root: Root {
                element RootNode { node: Node, }
                derive concord_class = node.concord_class;
                derive number = node.number;
                form root = node;
            }
            root Root { punctuation = "."; eoi = true; standalone_render = true; }
        })
        .expect("mixed invariant access fixture parses"),
    )
    .expect("mixed invariant access fixture validates")
    .into_semantic()
}

pub(crate) fn synthetic_projection_tokens() -> proc_macro2::TokenStream {
    quote! {
        vocab Mode { Solo = "solo", Group = "group", }
        lexeme ObjectStem using EnglishNoun { Widget = "widget", }
        lexeme ActionStem using EnglishVerb { Activate = "activate", }

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
        morphology EnglishNoun { feature = Number; recipe = english_noun; }
        morphology EnglishVerb { feature = ConcordClass; recipe = english_verb; }

        construction leaf: Expr {
            element LeafNode { mode: lex Mode, resource: lex Resource, }
            derive concord_class = match mode {
                Solo => Values::ThirdPersonSingular,
                Group => Values::Other,
            };
            derive number = match mode {
                Solo => Values::Singular,
                Group => Values::Plural,
            };
            form leaf = lex(mode) noun(resource);
        }
        construction nested: Expr {
            element NestedNode { next: Expr, marker: lex Marker, }
            derive concord_class = next.concord_class;
            derive number = next.number;
            form nested = "nest" next lex(marker);
        }
        construction action: Predicate {
            element ActionNode {}
            derive concord_class = verb.concord_class;
            form action = verb(ActionStem::Activate);
        }
        construction idle: Predicate {
            element IdleNode {}
            derive concord_class = Values::Other;
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
            require subject is Leaf;
            derive predicate.concord_class = subject.concord_class;
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

pub(crate) fn declaration_verb_tokens(tail: &proc_macro2::TokenStream) -> proc_macro2::TokenStream {
    quote! {
        morphology EnglishVerb { feature = ConcordClass; recipe = english_verb; }
        lexeme CoreVerb using EnglishVerb { Destroy = "destroy", }
        codec TransitiveVerb {
            generate declaration_verb {
                closed = CoreVerb;
                position = Verb;
                tail = [#tail];
                feature = ConcordClass;
            }
        }
        vocab Preposition { For = "for", }
        construction only: Root {
            element Only {}
            form only = "only";
        }
        root Root { punctuation = "."; eoi = true; standalone_render = true; }
    }
}

pub(crate) fn vocab_matched_number_without_noun_tokens() -> proc_macro2::TokenStream {
    quote! {
        vocab Count { One = "one", Many = "many", }

        construction source: Source {
            element SourceNode { count: lex Count, }
            derive concord_class = match count {
                One => Values::ThirdPersonSingular,
                Many => Values::Other,
            };
            derive number = match count {
                One => Values::Singular,
                Many => Values::Plural,
            };
            form source = lex(count);
        }
        construction phrase: Phrase {
            element PhraseNode { source: Source, }
            derive concord_class = source.concord_class;
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
    assert_eq!(expansion.plan().items().len(), 149);
    let concord_class_matches = expansion
        .items()
        .iter()
        .filter_map(|item| match &item.key {
            crate::ItemKey::Named {
                kind: crate::NamedKind::Function,
                name,
            } if name.starts_with("concord_class_matches_for_") => Some((
                name.as_str(),
                item.origins
                    .iter()
                    .map(|origin| (origin.kind(), origin.name()))
                    .collect::<Vec<_>>(),
            )),
            _ => None,
        })
        .collect::<Vec<_>>();
    assert_eq!(
        concord_class_matches,
        [
            (
                "concord_class_matches_for_expr",
                vec![
                    (crate::SourceDeclarationKind::Construction, "leaf"),
                    (crate::SourceDeclarationKind::Construction, "nested"),
                ],
            ),
            (
                "concord_class_matches_for_predicate",
                vec![
                    (crate::SourceDeclarationKind::Construction, "action"),
                    (crate::SourceDeclarationKind::Construction, "idle"),
                ],
            ),
        ],
    );
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
