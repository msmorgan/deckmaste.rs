#![allow(
    dead_code,
    reason = "the consumer compiles every generated phase while executing build, render, and visitor boundaries"
)]

use deckmaste_construction::constructions;

mod fixture {
    use RulePosition::Lexical as L;
    use RulePosition::Nonterminal as N;

    use super::constructions;

    #[derive(Debug, Clone, Copy, PartialEq, Eq)]
    enum RulePosition<Category, Lexical> {
        Nonterminal(Category),
        Lexical(Lexical),
    }

    #[derive(Debug, Clone, Copy, PartialEq, Eq)]
    struct Rule<Category: 'static, Lexical: 'static, RuleId> {
        id: RuleId,
        lhs: Category,
        rhs: &'static [RulePosition<Category, Lexical>],
    }

    #[derive(Default)]
    struct ParseContext<'a> {
        sentinel: u8,
        marker: std::marker::PhantomData<&'a ()>,
    }

    trait Render {
        fn render(&self, context: &ParseContext<'_>) -> String;
    }

    struct Writer(String);

    impl Writer {
        fn new() -> Self {
            Self(String::new())
        }

        fn word(&mut self, word: &str) {
            if !self.0.is_empty() {
                self.0.push(' ');
            }
            self.0.push_str(word);
        }

        fn punctuation(&mut self, punctuation: char) {
            self.0.push(punctuation);
        }

        fn identity(&mut self, identity: &str) {
            self.word(identity);
        }

        fn finish(self) -> String {
            self.0
        }
    }

    #[derive(Debug, Clone, PartialEq, Eq)]
    struct Head(u8);

    fn render_head(writer: &mut Writer, head: &Head, number: Number) {
        let suffix = match number {
            Number::Singular => "s",
            Number::Plural => "p",
        };
        writer.word(&format!("{}{}", head.0, suffix));
    }

    fn agreement_for_mode(mode: Mode) -> Agreement {
        match mode {
            Mode::One => Agreement::ThirdPersonSingular,
            Mode::Many => Agreement::Bare,
        }
    }

    fn inflect(lexeme: Verbs, agreement: Agreement) -> &'static str {
        match (lexeme, agreement) {
            (Verbs::Act, Agreement::Bare) => "act",
            (Verbs::Act, Agreement::ThirdPersonSingular) => "acts",
        }
    }

    impl ParseContext<'_> {
        fn card_name(&self) -> &'static str {
            debug_assert_ne!(self.sentinel, 0);
            "card"
        }
    }

    #[derive(Debug, Clone, Copy, PartialEq, Eq)]
    pub(crate) enum SelfRef {
        Full,
    }

    #[derive(Debug, Clone, Copy, PartialEq, Eq)]
    pub(crate) struct Token(u8);

    #[derive(Debug, Clone, Copy, PartialEq, Eq)]
    enum RawBranchToken {
        Marker(Marker),
    }

    #[allow(
        clippy::trivially_copy_pass_by_ref,
        reason = "the declared borrowed codec ABI is the behavior under test"
    )]
    fn render_token(writer: &mut Writer, token: &Token) {
        writer.word(&format!("token{}", token.0));
    }

    #[derive(Debug, Clone, PartialEq, Eq)]
    pub(crate) struct Pair {
        context_value: u8,
        child_slot: u8,
        rule_code: u8,
    }

    impl Pair {
        #[allow(
            clippy::trivially_copy_pass_by_ref,
            reason = "the fixture preserves parser-slice references through the declared build call"
        )]
        fn new(context: &u8, children: &u8, rule: &u8) -> Self {
            Self {
                context_value: *context,
                child_slot: *children,
                rule_code: *rule,
            }
        }
    }

    fn render_pair(writer: &mut Writer, pair: &Pair) {
        writer.word(&format!(
            "{}:{}:{}",
            pair.context_value, pair.child_slot, pair.rule_code
        ));
    }

    #[derive(Debug, Clone, PartialEq, Eq)]
    enum BoundLeaf {
        Pair(u8, u8, u8),
    }

    #[derive(Debug, Clone, PartialEq, Eq)]
    enum BuildValue {
        Source(Source, Number),
        Phrase(Phrase),
        PairPhrase(PairPhrase),
        Child(Child, Agreement),
        Parent(Parent, Agreement),
        Action(Action, Agreement),
        Predicate(Predicate, Agreement),
        Container(Container),
        CheckedContext(CheckedContext),
        FeatureChecked(FeatureChecked, Agreement),
        Collision(Collision),
        RenderChild(RenderChild),
        RawPayload(RawPayload),
        Keyword(Keyword, Agreement),
        HygieneRoot(HygieneRoot),
        ContextEnvelope(ContextEnvelope),
        VisitCategory(VisitCategory),
        CheckedMarker(CheckedMarker),
        RawCategory(RawCategory),
        Leaf(Leaf),
    }

    constructions! {
        vocab Mode { One = "one", Many = "many", }
        vocab r#Marker { One = "marker", }
        vocab WriterWord { One = "writer", }
        lexeme Verbs { Act, }
        lexeme r#VisitorLexeme { Act, }

        identity r#SelfRef {
            value_type = SelfRef;
            lexical = Lexical::SelfRef;
            render context_identity { Full => card_name, }
            build { pattern = BuildValue::SelfRef(value); construct = value; }
            traversal {
                callback = copy;
                argument = value;
                variant Full;
            }
        }

        codec r#Token {
            atom = lex;
            value_type = Token;
            lexical = Lexical::Token;
            render = render_token;
            build { pattern = BuildValue::Token(token); construct = token; }
            traversal {
                callback = borrowed;
                argument = visitor;
                call visitor::visit_token(borrowed(visitor));
            }
        }

        codec RawBranchToken {
            value_type = RawBranchToken;
            traversal {
                callback = borrowed;
                argument = r#payload;
                variant Marker;
                match payload {
                    RawBranchToken::Marker(r#walk_marker: Marker) =>
                        r#walk_marker(copy(walk_marker)),
                }
            }
        }

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

        codec Pair {
            atom = lex;
            value_type = Pair;
            lexical = Lexical::Pair;
            render = render_pair;
            build {
                pattern = BuildValue::Pair(r#context, children, rule);
                construct = Pair::new(context, children, rule);
            }
            traversal {
                callback = borrowed;
                argument = pair;
                call visitor::visit_pair(borrowed(pair));
            }
        }

        construction source: Source {
            element SourceNode {}
            derive number = Values::Singular;
            form source = "source";
        }
        construction one: Phrase {
            element OnePhrase { source: Source, head: lex Head, }
            derive number = source.number;
            form one = source noun(head);
        }
        construction two: PairPhrase {
            element TwoPhrase { source: Source, left: lex Head, right: lex Head, }
            derive number = source.number;
            form two = source noun(left) noun(right);
        }

        construction bare: Child {
            element BareChild {}
            derive agreement = Values::Bare;
            form bare = "bare";
        }
        construction third: Child {
            element ThirdChild {}
            derive agreement = Values::ThirdPersonSingular;
            form third = "third";
        }
        construction refined: Parent {
            element RefinedParent { mode: lex Mode, child: Child, }
            require mode is One;
            derive agreement = child.agreement;
            derive child.agreement = mode.agreement;
            derive mode.agreement = match mode {
                One => Values::ThirdPersonSingular,
                Many => Values::Bare,
            };
            form refined = lex(mode) child;
        }
        construction category_chain: Parent {
            element CategoryChain { child: Child, }
            derive agreement = child.agreement;
            derive child.agreement = Values::Bare;
            form category_chain = child;
        }

        construction action: Action {
            element ActionNode {}
            derive agreement = verb.agreement;
            derive verb.agreement = Values::Bare;
            form action = verb(Verbs::Act);
        }

        construction contextual: Predicate {
            element ContextualPredicate {}
            derive agreement = verb.agreement;
            form contextual = verb(Verbs::Act);
        }
        construction constant_container: Container {
            element ConstantContainer { predicate: Predicate, }
            derive predicate.agreement = Values::Bare;
            form constant_container = predicate;
        }
        construction from_role_container: Container {
            element FromRoleContainer { source: Child, predicate: Predicate, }
            derive predicate.agreement = source.agreement;
            form from_role_container = source predicate;
        }
        construction agreement_relay: Container {
            element RelayContainer { first: Predicate, second: Predicate, }
            derive second.agreement = first.agreement;
            derive first.agreement = Values::Bare;
            form agreement_relay = first second;
        }

        construction checked_context: CheckedContext {
            element CheckedContextNode { pair: lex Pair, }
            checked {
                visibility pair = pub(crate);
                constructor = CheckedContextNode::new(pair, context);
            }
            form checked_context = lex(pair);
        }

        construction agreement: FeatureChecked {
            element AgreementNode { marker: lex Marker, }
            checked {
                visibility marker = pub(crate);
                constructor = AgreementNode::new(marker);
            }
            derive agreement = verb.agreement;
            form agreement = lex(marker) verb(Verbs::Act);
        }

        construction collision: Collision {
            element CollisionNode {
                mode: lex Mode,
                number: lex Head,
                right_number: lex Head,
            }
            derive number = match mode {
                One => Values::Singular,
                Many => Values::Plural,
            };
            form collision = noun(number) noun(right_number) lex(mode);
        }

        construction contextual_named: Predicate {
            element ContextualNamed { agreement: lex Marker, }
            derive agreement = verb.agreement;
            form contextual_named = lex(agreement) verb(Verbs::Act);
        }

        construction wrapper: RenderChild {
            element RenderChildNode { child: Child, }
            form wrapper = child;
        }

        construction raw_payload: RawPayload {
            element RawPayloadNode { r#payload: lex Marker, }
            form raw_payload = lex(payload);
        }

        construction where: Keyword {
            element WhereNode { r#payload: lex Marker, }
            checked {
                visibility payload = private;
                access payload = payload;
                constructor = WhereNode::new(payload);
            }
            derive agreement = Values::Bare;
            form where = lex(payload);
        }

        construction hygiene_root: HygieneRoot {
            element HygieneRootNode {
                r#writer: lex Marker,
                r#context: identity SelfRef,
                predicate: Predicate,
                render_child: RenderChild,
                writer_word: lex WriterWord,
                raw_payload: RawPayload,
                keyword: Keyword,
            }
            derive predicate.agreement = Values::Bare;
            form hygiene_root = lex(r#writer) identity(r#context) predicate render_child
                lex(writer_word) raw_payload keyword;
        }

        construction context_envelope: ContextEnvelope {
            element ContextEnvelopeNode { inner: HygieneRoot, }
            form context_envelope = inner;
        }

        construction visit_node: VisitCategory {
            element VisitNode { visitor: lex Marker, }
            form visit_node = lex(visitor);
        }

        construction checked_marker: CheckedMarker {
            element WalkMarker { marker: lex Marker, }
            checked {
                visibility marker = private;
                access marker = marker;
                constructor = WalkMarker::new(marker);
            }
            form checked_marker = lex(marker);
        }

        construction r#raw_leaf: r#RawCategory {
            element r#RawNode {}
            form raw_leaf = "raw";
        }

        root Phrase { punctuation = "."; eoi = true; standalone_render = true; }
        root RenderChild { punctuation = "."; eoi = false; standalone_render = true; }
        root HygieneRoot { punctuation = "!"; eoi = true; standalone_render = true; }
        root r#RawCategory { punctuation = "?"; eoi = true; standalone_render = true; }
    }

    impl CheckedContextNode {
        #[allow(
            clippy::unnecessary_wraps,
            reason = "checked-construction callbacks deliberately expose the fallible ABI"
        )]
        fn new(mut pair: Pair, context: &ParseContext<'_>) -> Option<Self> {
            pair.context_value = context.sentinel;
            Some(Self { pair })
        }
    }

    impl AgreementNode {
        #[allow(
            clippy::unnecessary_wraps,
            reason = "checked-construction callbacks deliberately expose the fallible ABI"
        )]
        fn new(marker: Marker) -> Option<Self> {
            Some(Self { marker })
        }
    }

    impl WhereNode {
        #[allow(
            clippy::unnecessary_wraps,
            reason = "checked-construction callbacks deliberately expose the fallible ABI"
        )]
        fn new(payload: Marker) -> Option<Self> {
            Some(Self { payload })
        }

        fn payload(&self) -> Marker {
            self.r#payload
        }
    }

    impl WalkMarker {
        #[allow(
            clippy::unnecessary_wraps,
            reason = "checked-construction callbacks deliberately expose the fallible ABI"
        )]
        fn new(marker: Marker) -> Option<Self> {
            Some(Self { marker })
        }

        fn marker(&self) -> Marker {
            self.marker
        }
    }

    #[derive(Debug, PartialEq, Eq)]
    enum VisitEvent {
        Marker(Marker),
        VisitorLexeme(VisitorLexeme),
        Token(u8),
    }

    #[derive(Default)]
    struct RecordingVisitor(Vec<VisitEvent>);

    impl Visitor for RecordingVisitor {
        fn visit_marker(&mut self, marker: Marker) {
            self.0.push(VisitEvent::Marker(marker));
        }

        fn visit_visitor_lexeme(&mut self, lexeme: VisitorLexeme) {
            self.0.push(VisitEvent::VisitorLexeme(lexeme));
        }

        fn visit_token(&mut self, token: &Token) {
            self.0.push(VisitEvent::Token(token.0));
        }
    }

    fn assert_generated_runtime_abi() {
        let owner = |template: LexicalOwnerTemplate, leaf: &Leaf| {
            template
                .instantiate(leaf)
                .expect("every non-EOI terminal has an owner")
        };

        let vocab = owner(
            LexicalOwnerTemplate::Vocab {
                declaration: "Mode",
            },
            &Leaf::Mode(Mode::One),
        );
        assert_eq!(vocab.kind(), LexicalProvenanceKind::Vocab);
        assert_eq!(vocab.stable_id(), "vocab:Mode/One");

        for (template, leaf, kind, stable_id) in [
            (
                LexicalOwnerTemplate::Static {
                    kind: LexicalProvenanceKind::FormLiteral,
                    stable_id: "form:fixture",
                },
                Leaf::Literal("fixture"),
                LexicalProvenanceKind::FormLiteral,
                "form:fixture",
            ),
            (
                LexicalOwnerTemplate::Lexeme {
                    declaration: "Verbs",
                    member: "Act",
                },
                Leaf::Verb {
                    lexeme: Verbs::Act,
                    agreement: Agreement::Bare,
                },
                LexicalProvenanceKind::Lexeme,
                "lexeme:Verbs/Act",
            ),
            (
                LexicalOwnerTemplate::Static {
                    kind: LexicalProvenanceKind::Codec,
                    stable_id: "codec:Token",
                },
                Leaf::Token(Token(1)),
                LexicalProvenanceKind::Codec,
                "codec:Token",
            ),
            (
                LexicalOwnerTemplate::Static {
                    kind: LexicalProvenanceKind::Identity,
                    stable_id: "identity:SelfRef",
                },
                Leaf::SelfRef(SelfRef::Full),
                LexicalProvenanceKind::Identity,
                "identity:SelfRef",
            ),
        ] {
            let owner = owner(template, &leaf);
            assert_eq!(owner.kind(), kind);
            assert_eq!(owner.stable_id(), stable_id);
        }

        assert_eq!(
            LexicalOwnerTemplate::None.instantiate(&Leaf::EndOfInput),
            None
        );

        let kind = macro_ron::v2::DeclarationKind::KeywordAction;
        let position = macro_ron::v2::GrammarPosition::Verb;
        let declaration = owner(
            LexicalOwnerTemplate::Declaration {
                kind,
                name: "Destroy",
            },
            &Leaf::Declaration(DeclarationLeaf {
                id: macro_ron::v2::DeclarationIdentity::new(kind, "Destroy"),
                feature: macro_ron::v2::SurfaceFeature::Bare,
            }),
        );
        assert_eq!(declaration.kind(), LexicalProvenanceKind::Declaration);
        assert_eq!(
            declaration.stable_id(),
            "declaration:keyword action/Destroy"
        );

        let terminal = LexicalTerminal {
            matcher: Lexical::Declaration(DeclarationMatcher {
                kind,
                name: "Destroy",
                position,
                feature: FeatureConstraint::Exact(macro_ron::v2::SurfaceFeature::Bare),
            }),
            owner: LexicalOwnerTemplate::Declaration {
                kind,
                name: "Destroy",
            },
        };
        let TerminalClass::Declaration(class) = terminal.class() else {
            panic!("open declaration matcher retains its category-safe class")
        };
        assert_eq!(class.kind(), kind);
        assert_eq!(class.position(), position);
    }

    #[allow(
        clippy::too_many_lines,
        reason = "one authentic compiled consumer executes the complete boundary matrix"
    )]
    pub(super) fn run() {
        assert_generated_runtime_abi();
        let context = ParseContext {
            sentinel: 99,
            ..ParseContext::default()
        };
        let source = build(
            RuleId::SourceSource,
            &[BuildValue::Leaf(Leaf::Literal("source"))],
            &context,
        )
        .expect("source builds with its parser-domain number payload");
        assert!(matches!(
            source,
            BuildValue::Source(Source::Source(SourceNode), Number::Singular)
        ));

        let one_children = |number| {
            vec![
                source.clone(),
                BuildValue::Leaf(Leaf::Noun {
                    noun: Head(1),
                    number,
                }),
                BuildValue::Leaf(Leaf::Literal(".")),
                BuildValue::Leaf(Leaf::EndOfInput),
            ]
        };
        assert!(build(RuleId::PhraseOne, &one_children(Number::Singular), &context).is_some());
        assert!(build(RuleId::PhraseOne, &one_children(Number::Plural), &context).is_none());

        let two_children = |left, right| {
            vec![
                source.clone(),
                BuildValue::Leaf(Leaf::Noun {
                    noun: Head(2),
                    number: left,
                }),
                BuildValue::Leaf(Leaf::Noun {
                    noun: Head(3),
                    number: right,
                }),
            ]
        };
        assert!(
            build(
                RuleId::PairPhraseTwo,
                &two_children(Number::Singular, Number::Singular),
                &context,
            )
            .is_some()
        );
        for children in [
            two_children(Number::Plural, Number::Singular),
            two_children(Number::Singular, Number::Plural),
        ] {
            assert!(build(RuleId::PairPhraseTwo, &children, &context).is_none());
        }

        let bare_child = BuildValue::Child(Child::Bare(BareChild), Agreement::Bare);
        let third_child =
            BuildValue::Child(Child::Third(ThirdChild), Agreement::ThirdPersonSingular);
        let parent = build(
            RuleId::ParentCategoryChain,
            std::slice::from_ref(&bare_child),
            &context,
        )
        .expect("a constant category writer flows into construction output");
        assert!(matches!(parent, BuildValue::Parent(_, Agreement::Bare)));
        assert!(
            build(
                RuleId::ParentCategoryChain,
                std::slice::from_ref(&third_child),
                &context,
            )
            .is_none()
        );

        let matching_child =
            BuildValue::Child(Child::Third(ThirdChild), Agreement::ThirdPersonSingular);
        let mismatching_child = BuildValue::Child(Child::Third(ThirdChild), Agreement::Bare);
        let refined_children = |child| vec![BuildValue::Leaf(Leaf::Mode(Mode::One)), child];
        let refined = build(
            RuleId::ParentRefined,
            &refined_children(matching_child),
            &context,
        )
        .expect("a refined writer flows through its category into construction output");
        assert!(matches!(
            refined,
            BuildValue::Parent(_, Agreement::ThirdPersonSingular)
        ));
        assert!(
            build(
                RuleId::ParentRefined,
                &refined_children(mismatching_child),
                &context,
            )
            .is_none()
        );

        let action = build(
            RuleId::ActionAction,
            &[BuildValue::Leaf(Leaf::Verb {
                lexeme: Verbs::Act,
                agreement: Agreement::Bare,
            })],
            &context,
        )
        .expect("an implicit-verb constant flows into construction output");
        assert!(matches!(action, BuildValue::Action(_, Agreement::Bare)));
        assert!(
            build(
                RuleId::ActionAction,
                &[BuildValue::Leaf(Leaf::Verb {
                    lexeme: Verbs::Act,
                    agreement: Agreement::ThirdPersonSingular,
                })],
                &context,
            )
            .is_none()
        );

        let contextual = |agreement| {
            BuildValue::Predicate(Predicate::Contextual(ContextualPredicate), agreement)
        };
        assert!(
            build(
                RuleId::ContainerConstantContainer,
                &[contextual(Agreement::Bare)],
                &context,
            )
            .is_some()
        );
        assert!(
            build(
                RuleId::ContainerConstantContainer,
                &[contextual(Agreement::ThirdPersonSingular)],
                &context,
            )
            .is_none()
        );
        assert!(
            build(
                RuleId::ContainerFromRoleContainer,
                &[bare_child.clone(), contextual(Agreement::Bare),],
                &context,
            )
            .is_some()
        );
        assert!(
            build(
                RuleId::ContainerFromRoleContainer,
                &[bare_child, contextual(Agreement::ThirdPersonSingular)],
                &context,
            )
            .is_none()
        );
        assert!(
            build(
                RuleId::ContainerAgreementRelay,
                &[contextual(Agreement::Bare), contextual(Agreement::Bare),],
                &context,
            )
            .is_some()
        );
        assert!(
            build(
                RuleId::ContainerAgreementRelay,
                &[
                    contextual(Agreement::Bare),
                    contextual(Agreement::ThirdPersonSingular),
                ],
                &context,
            )
            .is_none()
        );

        let checked_context = build(
            RuleId::CheckedContextCheckedContext,
            &[BuildValue::Leaf(Leaf::Pair(BoundLeaf::Pair(1, 2, 3)))],
            &context,
        )
        .expect("terminal slots cannot shadow the parser context ABI local");
        assert!(matches!(
            checked_context,
            BuildValue::CheckedContext(CheckedContext::CheckedContext(CheckedContextNode {
                pair: Pair {
                    context_value: 99,
                    child_slot: 2,
                    rule_code: 3,
                },
            }))
        ));

        let feature_checked = build(
            RuleId::FeatureCheckedAgreement,
            &[
                BuildValue::Leaf(Leaf::Marker(Marker::One)),
                BuildValue::Leaf(Leaf::Verb {
                    lexeme: Verbs::Act,
                    agreement: Agreement::ThirdPersonSingular,
                }),
            ],
            &context,
        )
        .expect("a checked map local cannot shadow its carried agreement");
        assert!(matches!(
            feature_checked,
            BuildValue::FeatureChecked(_, Agreement::ThirdPersonSingular)
        ));

        let collision = build(
            RuleId::CollisionCollision,
            &[
                BuildValue::Leaf(Leaf::Noun {
                    noun: Head(4),
                    number: Number::Singular,
                }),
                BuildValue::Leaf(Leaf::Noun {
                    noun: Head(5),
                    number: Number::Singular,
                }),
                BuildValue::Leaf(Leaf::Mode(Mode::One)),
            ],
            &context,
        )
        .expect("adversarial preferred binders compile and build");
        assert!(matches!(
            collision,
            BuildValue::Collision(Collision::Collision(CollisionNode {
                number: Head(4),
                right_number: Head(5),
                ..
            }))
        ));

        let keyword = build(
            RuleId::KeywordWhere,
            &[BuildValue::Leaf(Leaf::Marker(Marker::One))],
            &context,
        )
        .expect("a keyword-named checked construction builds with its carried feature");
        let BuildValue::Keyword(keyword, Agreement::Bare) = keyword else {
            panic!("checked `where` preserves its category value and known feature")
        };

        let hygiene_root = HygieneRoot::HygieneRoot(HygieneRootNode {
            r#writer: Marker::One,
            r#context: SelfRef::Full,
            predicate: Predicate::ContextualNamed(ContextualNamed {
                agreement: Marker::One,
            }),
            render_child: RenderChild::Wrapper(RenderChildNode {
                child: Child::Bare(BareChild),
            }),
            writer_word: WriterWord::One,
            raw_payload: RawPayload::RawPayload(RawPayloadNode {
                r#payload: Marker::One,
            }),
            keyword,
        });
        assert_eq!(
            Render::render(&hygiene_root, &context),
            "marker card marker act bare writer marker marker!",
            "allocated render locals preserve ABI values and generated helper calls",
        );
        let context_free_nested_root = RenderChild::Wrapper(RenderChildNode {
            child: Child::Bare(BareChild),
        });
        assert_eq!(
            Render::render(&context_free_nested_root, &context),
            "bare.",
            "a context-free nested category and its standalone root share one helper arity",
        );

        let mut recording = RecordingVisitor::default();
        walk_visit_node(
            &mut recording,
            &VisitNode {
                visitor: Marker::One,
            },
        );
        walk_visitor_lexeme(&mut recording, VisitorLexeme::Act);
        walk_token(&mut recording, &Token(7));
        assert_eq!(
            recording.0.last(),
            Some(&VisitEvent::Token(7)),
            "an ordinary declared visitor callback remains executable",
        );
        walk_raw_branch_token(&mut recording, &RawBranchToken::Marker(Marker::One));
        assert_eq!(
            recording.0.last(),
            Some(&VisitEvent::Marker(Marker::One)),
            "an ordinary generated walker callback in a branch remains executable",
        );
        let walk_marker = WalkMarker::new(Marker::One).expect("checked walker fixture");
        walk_walk_marker(&mut recording, &walk_marker);
        assert_eq!(
            recording.0,
            [
                VisitEvent::Marker(Marker::One),
                VisitEvent::VisitorLexeme(VisitorLexeme::Act),
                VisitEvent::Token(7),
                VisitEvent::Marker(Marker::One),
                VisitEvent::Marker(Marker::One),
            ],
            "generated walkers retain callback order and the allocated operand values",
        );

        let raw_category = build(
            RuleId::RawCategoryRawLeaf,
            &[
                BuildValue::Leaf(Leaf::Literal("raw")),
                BuildValue::Leaf(Leaf::Literal("?")),
                BuildValue::Leaf(Leaf::EndOfInput),
            ],
            &context,
        )
        .expect("raw nonkeyword declaration spellings build through canonical generated names");
        let BuildValue::RawCategory(raw_category) = raw_category else {
            panic!("raw nonkeyword root preserves its generated category value")
        };
        assert_eq!(Render::render(&raw_category, &context), "raw?");
        walk_raw_category(&mut recording, &raw_category);
    }
}

#[test]
fn generated_output_is_type_correct_and_executes_every_boundary_case() {
    fixture::run();
}
