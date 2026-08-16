#![allow(
    dead_code,
    reason = "the consumer compiles every generated phase while executing the build boundary"
)]

use deckmaste_construction::constructions;

mod fixture {
    use super::constructions;
    use RulePosition::Lexical as L;
    use RulePosition::Nonterminal as N;

    #[derive(Debug, Clone, Copy, PartialEq, Eq)]
    enum Agreement {
        Bare,
        ThirdPersonSingular,
    }

    #[derive(Debug, Clone, Copy, PartialEq, Eq)]
    enum Number {
        Singular,
        Plural,
    }

    #[derive(Debug, Clone, Copy, PartialEq, Eq)]
    enum NounNumber {
        Singular,
        Plural,
        Either,
    }

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

    #[derive(Debug, Clone, PartialEq, Eq)]
    enum Leaf {
        Literal(&'static str),
        EndOfInput,
        Mode(Mode),
        Head { head: Head, number: NounNumber },
    }

    #[derive(Debug, Clone, PartialEq, Eq)]
    enum Lexical {
        Literal(&'static str),
        EndOfInput,
        Mode,
        Head(NounNumber),
    }

    #[derive(Debug, Clone, PartialEq, Eq)]
    enum BuildValue {
        Source(Source, NounNumber),
        Phrase(Phrase),
        PairPhrase(PairPhrase),
        Child(Child, Agreement),
        Parent(Parent),
        Collision(Collision),
        Leaf(Leaf),
    }

    constructions! {
        vocab Mode { One = "one", Many = "many", }

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
            derive mode.agreement = match mode {
                One => Values::ThirdPersonSingular,
                Many => Values::Bare,
            };
            derive child.agreement = mode.agreement;
            form refined = lex(mode) child;
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

        root Phrase { punctuation = "."; eoi = true; standalone_render = true; }
    }

    pub(super) fn run() {
        let context = ParseContext::default();
        let source = build(
            RuleId::SourceSource,
            &[BuildValue::Leaf(Leaf::Literal("source"))],
            &context,
        )
        .expect("source builds with its parser-domain number payload");
        assert!(matches!(
            source,
            BuildValue::Source(Source::Source(SourceNode), NounNumber::Singular)
        ));

        let one_children = |number| {
            vec![
                source.clone(),
                BuildValue::Leaf(Leaf::Head {
                    head: Head(1),
                    number,
                }),
                BuildValue::Leaf(Leaf::Literal(".")),
                BuildValue::Leaf(Leaf::EndOfInput),
            ]
        };
        assert!(
            build(
                RuleId::PhraseOne,
                &one_children(NounNumber::Singular),
                &context
            )
            .is_some()
        );
        assert!(
            build(
                RuleId::PhraseOne,
                &one_children(NounNumber::Plural),
                &context
            )
            .is_none()
        );

        let two_children = |left, right| {
            vec![
                source.clone(),
                BuildValue::Leaf(Leaf::Head {
                    head: Head(2),
                    number: left,
                }),
                BuildValue::Leaf(Leaf::Head {
                    head: Head(3),
                    number: right,
                }),
            ]
        };
        assert!(
            build(
                RuleId::PairPhraseTwo,
                &two_children(NounNumber::Singular, NounNumber::Singular),
                &context,
            )
            .is_some()
        );
        for children in [
            two_children(NounNumber::Plural, NounNumber::Singular),
            two_children(NounNumber::Singular, NounNumber::Plural),
        ] {
            assert!(build(RuleId::PairPhraseTwo, &children, &context).is_none());
        }

        let matching_child =
            BuildValue::Child(Child::Third(ThirdChild), Agreement::ThirdPersonSingular);
        let mismatching_child = BuildValue::Child(Child::Third(ThirdChild), Agreement::Bare);
        let refined_children = |child| vec![BuildValue::Leaf(Leaf::Mode(Mode::One)), child];
        assert!(
            build(
                RuleId::ParentRefined,
                &refined_children(matching_child),
                &context,
            )
            .is_some()
        );
        assert!(
            build(
                RuleId::ParentRefined,
                &refined_children(mismatching_child),
                &context,
            )
            .is_none()
        );

        let collision = build(
            RuleId::CollisionCollision,
            &[
                BuildValue::Leaf(Leaf::Head {
                    head: Head(4),
                    number: NounNumber::Singular,
                }),
                BuildValue::Leaf(Leaf::Head {
                    head: Head(5),
                    number: NounNumber::Singular,
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
    }
}

#[test]
fn generated_build_is_type_correct_and_executes_every_boundary_case() {
    fixture::run();
}
