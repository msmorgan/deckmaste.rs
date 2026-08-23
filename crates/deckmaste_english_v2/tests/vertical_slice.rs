use std::path::Path;

use deckmaste_english_v2::ast::*;
use deckmaste_english_v2::context::ParseContext;
use deckmaste_english_v2::environment::CatalogProviderRow;
use deckmaste_english_v2::environment::CatalogProviderRows;
use deckmaste_english_v2::environment::DeclarationId;
use deckmaste_english_v2::environment::ParserEnvironment;
use deckmaste_english_v2::parser::LexicalProvenanceKind;
use deckmaste_english_v2::parser::Parser;
use deckmaste_english_v2::parser::TraceLimits;
use deckmaste_english_v2::render::Render;
use deckmaste_english_v2::visit::Visitor;
use deckmaste_english_v2::visit::walk_amount;
use deckmaste_english_v2::visit::walk_cardinal_quantity;
use deckmaste_english_v2::visit::walk_scalar_number;
use macro_ron::v2::DeclarationKind;
use macro_ron::v2::Onset;
use macro_ron::v2::SubtypeCategory;
use macro_ron::v2::SurfaceFeature;

#[derive(Default)]
struct RecordingVisitor {
    amounts: Vec<Amount>,
    declaration_nouns: Vec<(DeclarationKind, String)>,
    variables: Vec<Variable>,
    scalar_numbers: Vec<u32>,
    self_reference_spellings: Vec<SelfReferenceSpelling>,
    trigger_words: Vec<TriggerWord>,
    nouns: Vec<NounLexeme>,
    verbs: Vec<VerbLexeme>,
    declarations: Vec<(macro_ron::v2::DeclarationKind, String)>,
    catalog_providers: Vec<CatalogProvider>,
    card_names: Vec<String>,
}

impl Visitor for RecordingVisitor {
    fn visit_amount(&mut self, amount: &Amount) {
        self.amounts.push(amount.clone());
        walk_amount(self, amount);
    }

    fn visit_declaration_noun(&mut self, noun: &DeclarationNoun) {
        self.declaration_nouns
            .push((noun.id().kind(), noun.id().name().to_owned()));
    }

    fn visit_variable(&mut self, variable: Variable) {
        self.variables.push(variable);
    }

    fn visit_scalar_number(&mut self, number: &ScalarNumber) {
        self.scalar_numbers.push(number.magnitude);
    }

    fn visit_self_reference_spelling(&mut self, spelling: SelfReferenceSpelling) {
        self.self_reference_spellings.push(spelling);
    }

    fn visit_trigger_word(&mut self, word: TriggerWord) {
        self.trigger_words.push(word);
    }

    fn visit_noun_lexeme(&mut self, noun: NounLexeme) {
        self.nouns.push(noun);
    }

    fn visit_verb_lexeme(&mut self, verb: VerbLexeme) {
        self.verbs.push(verb);
    }

    fn visit_declaration(&mut self, declaration: &macro_ron::v2::DeclarationIdentity) {
        if matches!(declaration.kind(), DeclarationKind::KeywordAction) {
            self.declarations
                .push((declaration.kind(), declaration.name().to_owned()));
        }
    }

    fn visit_catalog_provider(&mut self, provider: CatalogProvider) {
        self.catalog_providers.push(provider);
    }

    fn visit_card_name(&mut self, card_name: &CardName) {
        self.card_names
            .push(card_name.canonical_identity().to_owned());
    }
}

fn environment() -> ParserEnvironment {
    let declarations = macro_ron::v2::read_builtin_v2(
        Path::new(env!("CARGO_MANIFEST_DIR")).join("../../plugins/builtin_v2"),
    )
    .expect("integrated builtin-v2 declarations load");
    ParserEnvironment::try_from_parts(
        declarations,
        [CatalogProviderRows::new(
            CatalogProvider::CardNames,
            [CatalogProviderRow::new(
                "seven-dwarves",
                "Seven Dwarves",
                Onset::Consonant,
            )],
        )],
    )
    .expect("builtin-v2 declaration and catalog environment freezes")
}

fn card_type(spelling: &str) -> Noun {
    declaration_noun(DeclarationKind::Type, spelling, SurfaceFeature::Singular)
}

fn declaration_noun(kind: DeclarationKind, name: &str, _feature: SurfaceFeature) -> Noun {
    let environment = environment();
    Noun::Declaration(
        DeclarationNoun::new(&environment, DeclarationId::new(kind, name))
            .expect("normalized noun declaration is present"),
    )
}

fn creature() -> Noun {
    card_type("Creature")
}

fn creatures() -> Noun {
    declaration_noun(DeclarationKind::Type, "Creature", SurfaceFeature::Plural)
}

fn artifact() -> Noun {
    card_type("Artifact")
}

fn equipment() -> Noun {
    declaration_noun(
        DeclarationKind::Subtype(SubtypeCategory::Artifact),
        "Equipment",
        SurfaceFeature::Singular,
    )
}

fn context(card_name: &str) -> ParseContext<'_> {
    ParseContext::new(card_name).expect("test card name is a valid parse context")
}

fn self_reference(spelling: SelfReferenceSpelling, card_name: &str) -> SelfReferenceNp {
    let context = context(card_name);
    SelfReferenceNp::new(spelling, &context).expect("test spelling is valid for its context")
}

fn target_creature() -> NounPhrase {
    NounPhrase::Target(TargetNp { head: creature() })
}

fn it() -> NounPhrase {
    NounPhrase::Pronoun(PronounNp { word: Pronoun::It })
}

fn damage(amount: Amount) -> VerbPhrase {
    VerbPhrase::DealDamage(DealDamage { to: it(), amount })
}

fn triggered_damage() -> Ability {
    let event = Clause::Event(EventClause {
        subject: NounPhrase::Common(Common {
            head: Noun::Lexeme(NounLexeme::Player),
        }),
        predicate: VerbPhrase::Connive(Connive),
    });
    let effect = Sentence::Declarative(Declarative {
        subject: NounPhrase::Demonstrative(DemonstrativeNp {
            word: Demonstrative::That,
            head: creature(),
        }),
        predicate: damage(Amount::Variable(VariableAmount {
            variable: Variable::X,
        })),
    });
    Ability::Triggered(Triggered::new(TriggerWord::Whenever, event, vec![effect]).unwrap())
}

#[test]
fn parser_analysis_preserves_the_vertical_slice_triggered_ability() {
    let environment = environment();
    let parser = Parser::new(environment.clone()).expect("required declarations are present");
    let context = context("Context Card");
    let expected = triggered_damage();
    let text = expected.render(&context, &environment);

    assert_eq!(parser.parse(&text, &context), Ok(expected));
    assert_eq!(
        parser.parse(&text, &context),
        parser.analyze(&text, &context).into_parse_result(),
    );
    for limit in [0, 1, usize::MAX] {
        assert_eq!(
            parser.parse(&text, &context),
            parser
                .trace(&text, &context, TraceLimits::new(limit))
                .into_parse_result(),
        );
    }
}

#[test]
fn unsigned_decimal_zero_constructs_renders_scans_and_visits() {
    let environment = environment();
    let parser = Parser::new(environment.clone()).expect("required declarations are present");
    let context = context("Context Card");
    let number = ScalarNumber { magnitude: 0 };
    let ability = Ability::Paragraph(
        Paragraph::new(vec![Sentence::Imperative(Imperative {
            predicate: VerbPhrase::GainLife(GainLife {
                amount: Amount::Number(NumberAmount { number }),
            }),
        })])
        .expect("one sentence constructs a paragraph"),
    );
    assert_eq!(ability.render(&context, &environment), "Gain 0 life.");
    assert_eq!(parser.parse("Gain 0 life.", &context), Ok(ability));
    assert!(parser.parse("Gain -0 life.", &context).is_err());
}

#[test]
fn generated_cardinals_and_unsigned_scalars_round_trip_with_codec_ownership() {
    #[derive(Default)]
    struct NumberVisitor {
        cardinals: Vec<u32>,
        scalars: Vec<u32>,
        variables: Vec<Variable>,
    }
    impl Visitor for NumberVisitor {
        fn visit_cardinal_number(&mut self, number: &CardinalNumber) {
            self.cardinals.push(number.magnitude);
        }

        fn visit_scalar_number(&mut self, number: &ScalarNumber) {
            self.scalars.push(number.magnitude);
        }

        fn visit_variable(&mut self, variable: Variable) {
            self.variables.push(variable);
        }
    }

    let environment = environment();
    let parser = Parser::new(environment.clone()).expect("required declarations are present");
    let context = context("Context Card");
    for (surface, magnitude) in [
        ("Zero", 0),
        ("One", 1),
        ("Twenty-one", 21),
        ("One thousand, one", 1_001),
    ] {
        let expected = CardinalQuantity::Cardinal(CardinalQuantityValue {
            number: CardinalNumber { magnitude },
        });
        assert_eq!(
            parser.parse_cardinal_quantity(surface, &context),
            Ok(expected.clone())
        );
        assert_eq!(expected.render(&context, &environment), surface);
        let analysis = parser.analyze_cardinal_quantity(surface, &context);
        let ownership = analysis
            .ownership()
            .expect("selected cardinal owns its bytes");
        assert!(ownership.summary().covered());
        assert_eq!(ownership.parsed_claims().len(), 1);
        assert_eq!(
            ownership.parsed_claims()[0].kind(),
            LexicalProvenanceKind::Codec
        );
    }

    let scalar = ScalarNumber { magnitude: 1_000 };
    let ability = Ability::Paragraph(
        Paragraph::new(vec![Sentence::Imperative(Imperative {
            predicate: VerbPhrase::GainLife(GainLife {
                amount: Amount::Number(NumberAmount {
                    number: scalar.clone(),
                }),
            }),
        })])
        .expect("one sentence constructs a paragraph"),
    );
    assert_eq!(ability.render(&context, &environment), "Gain 1,000 life.");
    assert_eq!(parser.parse("Gain 1,000 life.", &context), Ok(ability));
    let scalar_ownership = parser
        .analyze("Gain 1,000 life.", &context)
        .ownership()
        .expect("selected scalar ability owns its bytes")
        .clone();
    assert!(scalar_ownership.summary().covered());
    assert!(scalar_ownership.parsed_claims().iter().any(|claim| {
        claim.kind() == LexicalProvenanceKind::Codec
            && &"Gain 1,000 life."[claim.span().start..claim.span().end] == " 1,000"
    }));

    let parsed_cardinal = parser
        .parse_cardinal_quantity("One thousand, one", &context)
        .expect("canonical cardinal parses through its generated parent root");

    let y_ability = Ability::Paragraph(
        Paragraph::new(vec![Sentence::Imperative(Imperative {
            predicate: VerbPhrase::GainLife(GainLife {
                amount: Amount::Variable(VariableAmount {
                    variable: Variable::Y,
                }),
            }),
        })])
        .expect("one sentence constructs a paragraph"),
    );
    assert_eq!(y_ability.render(&context, &environment), "Gain Y life.");
    assert_eq!(
        parser.parse("Gain Y life.", &context),
        Ok(y_ability.clone())
    );

    let mut visitor = NumberVisitor::default();
    walk_cardinal_quantity(&mut visitor, &parsed_cardinal);
    walk_scalar_number(&mut visitor, &scalar);
    deckmaste_english_v2::visit::walk_ability(&mut visitor, &y_ability);
    assert_eq!(visitor.cardinals, [1_001]);
    assert_eq!(visitor.scalars, [1_000]);
    assert_eq!(visitor.variables, [Variable::Y]);

    assert!(parser.parse_cardinal_quantity("-1", &context).is_err());
    assert!(parser.parse("Gain -1 life.", &context).is_err());
}

fn gain_life_with_where() -> Sentence {
    Sentence::WithWhere(
        WithWhere::new(
            Box::new(Sentence::Declarative(Declarative {
                subject: NounPhrase::Pronoun(PronounNp { word: Pronoun::You }),
                predicate: VerbPhrase::GainLife(GainLife {
                    amount: Amount::Variable(VariableAmount {
                        variable: Variable::X,
                    }),
                }),
            })),
            Clause::Where(WhereClause {
                variable: Variable::X,
                value: NounPhrase::Count(
                    CountNp::new(creatures(), Pronoun::You, ScalarNumber { magnitude: 2 })
                        .expect("You is a valid count controller"),
                ),
            }),
        )
        .expect("Where is a valid trailing clause"),
    )
}

#[test]
fn paragraph_and_oracle_text_constructors_and_traversal_preserve_structural_order() {
    #[derive(Debug, PartialEq, Eq)]
    enum Event {
        Block,
        Paragraph,
        Triggered,
        Sentence(&'static str),
    }

    #[derive(Default)]
    struct StructuralVisitor(Vec<Event>);

    impl Visitor for StructuralVisitor {
        fn visit_document_block(&mut self, block: &DocumentBlock) {
            self.0.push(Event::Block);
            deckmaste_english_v2::visit::walk_document_block(self, block);
        }

        fn visit_paragraph(&mut self, paragraph: &Paragraph) {
            self.0.push(Event::Paragraph);
            deckmaste_english_v2::visit::walk_paragraph(self, paragraph);
        }

        fn visit_triggered(&mut self, triggered: &Triggered) {
            self.0.push(Event::Triggered);
            deckmaste_english_v2::visit::walk_triggered(self, triggered);
        }

        fn visit_sentence(&mut self, sentence: &Sentence) {
            let label = match sentence {
                Sentence::Imperative(Imperative {
                    predicate: VerbPhrase::Destroy(_),
                }) => "destroy",
                Sentence::Imperative(Imperative {
                    predicate: VerbPhrase::Connive(_),
                }) => "connive",
                Sentence::Declarative(_) => "gain",
                Sentence::WithWhere(_) => "where",
                Sentence::Imperative(_) => "other",
            };
            self.0.push(Event::Sentence(label));
            deckmaste_english_v2::visit::walk_sentence(self, sentence);
        }
    }

    let destroy = Sentence::Imperative(Imperative {
        predicate: VerbPhrase::Destroy(Destroy {
            object: target_creature(),
        }),
    });
    let gain = Sentence::Declarative(Declarative {
        subject: NounPhrase::Pronoun(PronounNp { word: Pronoun::You }),
        predicate: VerbPhrase::GainLife(GainLife {
            amount: Amount::Number(NumberAmount {
                number: ScalarNumber { magnitude: 2 },
            }),
        }),
    });
    let connive = Sentence::Imperative(Imperative {
        predicate: VerbPhrase::Connive(Connive),
    });

    assert!(Paragraph::new(vec![]).is_none());
    let paragraph_sentences = vec![destroy, gain.clone()];
    let paragraph = Paragraph::new(paragraph_sentences.clone())
        .expect("a paragraph accepts one or more sentences");
    assert_eq!(paragraph.sentences(), paragraph_sentences.as_slice());

    let event = Clause::Event(EventClause {
        subject: NounPhrase::Pronoun(PronounNp { word: Pronoun::You }),
        predicate: VerbPhrase::Connive(Connive),
    });
    assert!(
        Triggered::new(TriggerWord::Whenever, event.clone(), vec![]).is_none(),
        "the effects length invariant rejects an empty sequence independently",
    );
    assert!(
        Triggered::new(
            TriggerWord::Whenever,
            Clause::Where(WhereClause {
                variable: Variable::X,
                value: NounPhrase::Pronoun(PronounNp { word: Pronoun::You }),
            }),
            vec![gain.clone()],
        )
        .is_none(),
        "the event refinement rejects a non-Event with nonempty effects",
    );
    let triggered_effects = vec![connive, gain];
    let triggered = Triggered::new(TriggerWord::Whenever, event, triggered_effects.clone())
        .expect("an Event and nonempty effects construct Triggered");
    assert_eq!(triggered.effects(), triggered_effects.as_slice());

    let empty_oracle_text = OracleText { blocks: vec![] };
    assert_eq!(
        empty_oracle_text.render(&context("Grizzly Bears"), &environment()),
        ""
    );
    assert!(empty_oracle_text.blocks.is_empty());

    let mut empty_visitor = StructuralVisitor::default();
    deckmaste_english_v2::visit::walk_oracle_text(&mut empty_visitor, &empty_oracle_text);
    assert!(empty_visitor.0.is_empty());

    let blocks = vec![
        DocumentBlock::Ability(Ability::Paragraph(paragraph)),
        DocumentBlock::Ability(Ability::Triggered(triggered)),
    ];
    let oracle_text = OracleText {
        blocks: blocks.clone(),
    };
    assert_eq!(oracle_text.blocks, blocks);

    let mut visitor = StructuralVisitor::default();
    deckmaste_english_v2::visit::walk_oracle_text(&mut visitor, &oracle_text);
    assert_eq!(
        visitor.0,
        [
            Event::Block,
            Event::Paragraph,
            Event::Sentence("destroy"),
            Event::Sentence("gain"),
            Event::Block,
            Event::Triggered,
            Event::Sentence("connive"),
            Event::Sentence("gain"),
        ],
    );
}

#[test]
fn declaration_noun_construction_requires_allowed_environment_membership() {
    let environment = environment();
    assert!(
        DeclarationNoun::new(
            &environment,
            DeclarationId::new(DeclarationKind::Type, "Creature"),
        )
        .is_some()
    );
    assert!(
        DeclarationNoun::new(
            &environment,
            DeclarationId::new(DeclarationKind::Type, "Definitely Not A Type"),
        )
        .is_none()
    );
    assert!(
        DeclarationNoun::new(
            &environment,
            DeclarationId::new(DeclarationKind::KeywordAbility, "Flying"),
        )
        .is_none(),
        "a present but disallowed declaration kind cannot become a noun"
    );
}

#[test]
fn generated_invariant_triggered_compile_surface_stores_and_accepts_nonempty_effects() {
    let constructor: fn(TriggerWord, Clause, Vec<Sentence>) -> Option<Triggered> = Triggered::new;
    let event = Clause::Event(EventClause {
        subject: NounPhrase::Pronoun(PronounNp { word: Pronoun::You }),
        predicate: VerbPhrase::Connive(Connive),
    });
    let effect = Sentence::Imperative(Imperative {
        predicate: VerbPhrase::Connive(Connive),
    });
    let value = constructor(TriggerWord::Whenever, event, vec![effect.clone()])
        .expect("an Event clause and one Sentence construct Triggered");
    let _: &[Sentence] = value.effects();
    assert_eq!(value.effects(), [effect]);
}

#[test]
fn renders_destroy_target_creature_exactly() {
    let value = Ability::Paragraph(
        Paragraph::new(vec![Sentence::Imperative(Imperative {
            predicate: VerbPhrase::Destroy(Destroy {
                object: target_creature(),
            }),
        })])
        .expect("one sentence constructs a paragraph"),
    );
    assert_eq!(
        value.render(&context("Context Card"), &environment()),
        "Destroy target creature."
    );
}

#[test]
fn renders_triggered_damage_exactly_without_capitalizing_after_the_comma() {
    let value = triggered_damage();
    assert_eq!(
        value.render(&context("Context Card"), &environment()),
        "Whenever a player connives, that creature deals X damage to it."
    );
}

#[test]
fn renders_gain_life_with_a_where_binder_exactly() {
    let value = gain_life_with_where();
    assert_eq!(
        value.render(&context("Context Card"), &environment()),
        "You gain X life, where X is the number of creatures you control with power 2 or less."
    );
}

#[test]
fn renders_a_plural_count_subject_with_a_bare_verb() {
    let value = Sentence::Declarative(Declarative {
        subject: NounPhrase::Count(
            CountNp::new(creatures(), Pronoun::You, ScalarNumber { magnitude: 2 })
                .expect("You is a valid count controller"),
        ),
        predicate: VerbPhrase::GainLife(GainLife {
            amount: Amount::Variable(VariableAmount {
                variable: Variable::X,
            }),
        }),
    });

    assert_eq!(
        value.render(&context("Context Card"), &environment()),
        "Creatures you control with power 2 or less gain X life."
    );
}

#[test]
fn renders_real_abbreviated_self_reference_with_a_declaration_noun() {
    let subject = self_reference(
        SelfReferenceSpelling::Abbreviated,
        "Zacama, Primal Calamity",
    );
    let value = Sentence::Declarative(Declarative {
        subject: NounPhrase::SelfReference(subject),
        predicate: VerbPhrase::DealDamage(DealDamage {
            amount: Amount::Number(NumberAmount {
                number: ScalarNumber { magnitude: 3 },
            }),
            to: target_creature(),
        }),
    });
    assert_eq!(
        value.render(&context("Zacama, Primal Calamity"), &environment()),
        "Zacama deals 3 damage to target creature."
    );
}

#[test]
fn the_same_self_reference_value_renders_from_two_card_contexts() {
    let value = Sentence::Declarative(Declarative {
        subject: NounPhrase::SelfReference(self_reference(
            SelfReferenceSpelling::Abbreviated,
            "Zacama, Primal Calamity",
        )),
        predicate: VerbPhrase::DealDamage(DealDamage {
            amount: Amount::Number(NumberAmount {
                number: ScalarNumber { magnitude: 3 },
            }),
            to: target_creature(),
        }),
    });

    let zacama = value.render(&context("Zacama, Primal Calamity"), &environment());
    let zoraline = value.render(&context("Zoraline, Cosmos Caller"), &environment());

    assert_eq!(zacama, "Zacama deals 3 damage to target creature.");
    assert_eq!(zoraline, "Zoraline deals 3 damage to target creature.");
    assert_eq!(zacama.replacen("Zacama", "Zoraline", 1), zoraline);
}

#[test]
fn renders_those_with_a_plural_noun_and_bare_verb() {
    let value = Sentence::Declarative(Declarative {
        subject: NounPhrase::Demonstrative(DemonstrativeNp {
            word: Demonstrative::Those,
            head: creatures(),
        }),
        predicate: damage(Amount::Number(NumberAmount {
            number: ScalarNumber { magnitude: 3 },
        })),
    });
    assert_eq!(
        value.render(&context("Context Card"), &environment()),
        "Those creatures deal 3 damage to it."
    );
}

#[test]
fn demonstrative_form_selection_adds_no_ast_or_visitor_tag() {
    #[derive(Debug, PartialEq, Eq)]
    enum Event {
        DemonstrativeNp,
        Word(Demonstrative),
        Head(DeclarationKind, String),
    }

    #[derive(Default)]
    struct DemonstrativeVisitor(Vec<Event>);

    impl Visitor for DemonstrativeVisitor {
        fn visit_demonstrative_np(&mut self, demonstrative: &DemonstrativeNp) {
            self.0.push(Event::DemonstrativeNp);
            deckmaste_english_v2::visit::walk_demonstrative_np(self, demonstrative);
        }

        fn visit_demonstrative(&mut self, word: Demonstrative) {
            self.0.push(Event::Word(word));
        }

        fn visit_declaration_noun(&mut self, noun: &DeclarationNoun) {
            self.0
                .push(Event::Head(noun.id().kind(), noun.id().name().to_owned()));
        }
    }

    for (word, head, expected) in [
        (
            Demonstrative::That,
            creature(),
            "That creature deals 3 damage to it.",
        ),
        (
            Demonstrative::Those,
            creatures(),
            "Those creatures deal 3 damage to it.",
        ),
    ] {
        let value = NounPhrase::Demonstrative(DemonstrativeNp { word, head });
        let sentence = Sentence::Declarative(Declarative {
            subject: value.clone(),
            predicate: damage(Amount::Number(NumberAmount {
                number: ScalarNumber { magnitude: 3 },
            })),
        });
        assert_eq!(
            sentence.render(&context("Context Card"), &environment()),
            expected,
        );

        let mut visitor = DemonstrativeVisitor::default();
        deckmaste_english_v2::visit::walk_noun_phrase(&mut visitor, &value);
        assert_eq!(
            visitor.0,
            [
                Event::DemonstrativeNp,
                Event::Word(word),
                Event::Head(DeclarationKind::Type, "Creature".to_owned(),),
            ],
        );
    }
}

#[test]
fn renders_an_with_a_singular_noun_and_third_person_verb() {
    let value = Sentence::Declarative(Declarative {
        subject: NounPhrase::Common(Common { head: artifact() }),
        predicate: damage(Amount::Number(NumberAmount {
            number: ScalarNumber { magnitude: 3 },
        })),
    });
    assert_eq!(
        value.render(&context("Context Card"), &environment()),
        "An artifact deals 3 damage to it."
    );
}

#[test]
fn renders_a_subtype_with_its_printed_case() {
    let value = Sentence::Declarative(Declarative {
        subject: NounPhrase::Common(Common { head: equipment() }),
        predicate: damage(Amount::Number(NumberAmount {
            number: ScalarNumber { magnitude: 3 },
        })),
    });
    assert_eq!(
        value.render(&context("Context Card"), &environment()),
        "An Equipment deals 3 damage to it."
    );
}

#[test]
fn visitor_reaches_every_vertical_slice_leaf() {
    let destroy = Ability::Paragraph(
        Paragraph::new(vec![Sentence::Imperative(Imperative {
            predicate: VerbPhrase::Destroy(Destroy {
                object: target_creature(),
            }),
        })])
        .expect("one sentence constructs a paragraph"),
    );
    let self_reference = Sentence::Declarative(Declarative {
        subject: NounPhrase::SelfReference(self_reference(
            SelfReferenceSpelling::Abbreviated,
            "Zacama, Primal Calamity",
        )),
        predicate: VerbPhrase::DealDamage(DealDamage {
            amount: Amount::Number(NumberAmount {
                number: ScalarNumber { magnitude: 3 },
            }),
            to: target_creature(),
        }),
    });

    let mut visitor = RecordingVisitor::default();
    visitor.visit_ability(&destroy);
    visitor.visit_ability(&triggered_damage());
    visitor.visit_sentence(&gain_life_with_where());
    visitor.visit_sentence(&self_reference);

    assert_eq!(
        visitor.amounts,
        vec![
            Amount::Variable(VariableAmount {
                variable: Variable::X,
            }),
            Amount::Variable(VariableAmount {
                variable: Variable::X,
            }),
            Amount::Number(NumberAmount {
                number: ScalarNumber { magnitude: 3 },
            }),
        ]
    );
    assert_eq!(
        visitor.declaration_nouns,
        vec![
            (DeclarationKind::Type, "Creature".to_owned()),
            (DeclarationKind::Type, "Creature".to_owned()),
            (DeclarationKind::Type, "Creature".to_owned()),
            (DeclarationKind::Type, "Creature".to_owned()),
        ]
    );
    assert_eq!(
        visitor.variables,
        vec![Variable::X, Variable::X, Variable::X]
    );
    assert_eq!(visitor.scalar_numbers, vec![2, 3]);
    assert_eq!(
        visitor.self_reference_spellings,
        vec![SelfReferenceSpelling::Abbreviated]
    );
    assert_eq!(visitor.trigger_words, vec![TriggerWord::Whenever]);
    assert_eq!(visitor.nouns, vec![NounLexeme::Player]);
    assert_eq!(
        visitor.declarations,
        vec![
            (
                macro_ron::v2::DeclarationKind::KeywordAction,
                "Destroy".to_owned()
            ),
            (
                macro_ron::v2::DeclarationKind::KeywordAction,
                "Connive".to_owned()
            ),
        ]
    );
    assert_eq!(
        visitor.verbs,
        vec![
            VerbLexeme::Deal,
            VerbLexeme::Gain,
            VerbLexeme::Be,
            VerbLexeme::Control,
            VerbLexeme::Deal,
        ]
    );
}

#[test]
fn visitor_reaches_catalog_provider_and_canonical_identity() {
    let environment = environment();
    let parser = Parser::new(environment).expect("required rows are present");
    let context = context("Context Card");
    let parsed = parser
        .parse("Destroy a card named Seven Dwarves.", &context)
        .expect("named identity parses");

    let mut visitor = RecordingVisitor::default();
    visitor.visit_ability(&parsed);
    assert_eq!(visitor.catalog_providers, [CatalogProvider::CardNames]);
    assert_eq!(visitor.card_names, ["seven-dwarves"]);
}
