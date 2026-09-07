use std::path::Path;

use deckmaste_construction_core::macro_def::DeclarationKind;
use deckmaste_construction_core::macro_def::Onset;
use deckmaste_construction_core::macro_def::SubtypeCategory;
use deckmaste_construction_core::macro_def::SurfaceFeature;
use deckmaste_english_v2::ast::*;
use deckmaste_english_v2::context::ParseContext;
use deckmaste_english_v2::environment::CatalogProviderRow;
use deckmaste_english_v2::environment::CatalogProviderRows;
use deckmaste_english_v2::environment::CoreVerbIdentity;
use deckmaste_english_v2::environment::DeclarationId;
use deckmaste_english_v2::environment::ParserEnvironment;
use deckmaste_english_v2::environment::VerbInventoryRef;
use deckmaste_english_v2::parser::LexicalProvenanceKind;
use deckmaste_english_v2::parser::Parser;
use deckmaste_english_v2::parser::TraceLimits;
use deckmaste_english_v2::render::Render;
use deckmaste_english_v2::visit::Visitor;
use deckmaste_english_v2::visit::walk_amount;
use deckmaste_english_v2::visit::walk_cardinal_quantity;
use deckmaste_english_v2::visit::walk_scalar_number;

#[derive(Default)]
struct RecordingVisitor {
    amounts: Vec<Amount>,
    declaration_nouns: Vec<(DeclarationKind, String)>,
    variables: Vec<Variable>,
    scalar_numbers: Vec<u32>,
    self_reference_spellings: Vec<SelfReferenceSpelling>,
    trigger_markers: Vec<TriggerMarker>,
    nouns: Vec<CommonNoun>,
    verbs: Vec<VerbInventoryRef>,
    finite_copulas: Vec<FiniteCopula>,
    declarations: Vec<(
        deckmaste_construction_core::macro_def::DeclarationKind,
        String,
    )>,
    catalog_providers: Vec<CatalogProvider>,
    card_names: Vec<String>,
}

impl Visitor for RecordingVisitor {
    fn visit_amount(&mut self, amount: &Amount) {
        self.amounts.push(amount.clone());
        walk_amount(self, amount);
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

    fn visit_trigger_marker(&mut self, marker: TriggerMarker) {
        self.trigger_markers.push(marker);
    }

    fn visit_common_noun(&mut self, noun: CommonNoun) {
        self.nouns.push(noun);
    }

    fn visit_verb_inventory(&mut self, verb: &VerbInventoryRef) {
        self.verbs.push(verb.clone());
    }

    fn visit_finite_copula(&mut self, copula: FiniteCopula) {
        self.finite_copulas.push(copula);
    }

    fn visit_declaration(
        &mut self,
        declaration: &deckmaste_construction_core::macro_def::DeclarationIdentity,
    ) {
        match declaration.kind() {
            DeclarationKind::Type | DeclarationKind::Subtype(_) => self
                .declaration_nouns
                .push((declaration.kind(), declaration.name().to_owned())),
            DeclarationKind::KeywordAction => self
                .declarations
                .push((declaration.kind(), declaration.name().to_owned())),
            _ => {}
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
    let declarations = deckmaste_construction_core::macro_def::read_builtin_v2(
        Path::new(env!("CARGO_MANIFEST_DIR")).join("../../plugins_v2/builtin"),
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
    let id = DeclarationId::new(kind, name);
    match kind {
        DeclarationKind::Type | DeclarationKind::Subtype(_) | DeclarationKind::TurnPart => {
            Noun::Declaration(
                DeclarationNoun::new(&environment, id)
                    .expect("normalized noun declaration is present in the aggregate inventory"),
            )
        }
        _ => panic!("test helper accepts only noun-contributing declaration identities"),
    }
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
    ParseContext::new(
        card_name,
        false,
        deckmaste_construction_core::macro_def::Onset::Consonant,
    )
    .expect("test card name is a valid parse context")
}

fn legendary_context(card_name: &str) -> ParseContext<'_> {
    ParseContext::new(
        card_name,
        true,
        deckmaste_construction_core::macro_def::Onset::Consonant,
    )
    .expect("test legendary card name is a valid parse context")
}

fn self_reference(spelling: SelfReferenceSpelling, card_name: &str) -> SourceSelfReference {
    let context = if spelling == SelfReferenceSpelling::Abbreviated {
        legendary_context(card_name)
    } else {
        context(card_name)
    };
    SourceSelfReference::new(spelling, &context).expect("test spelling is valid for its context")
}

fn singular_nominal(noun: Noun) -> Nominal {
    Nominal::BareSingularNominal(
        BareSingularNominal::new(Head::NounSingularHead(
            NounSingularHead::new(noun).expect("test noun is countable"),
        ))
        .expect("the Singular nominal accepts a Singular Head"),
    )
}

fn plural_nominal(noun: Noun) -> Nominal {
    Nominal::BarePluralNominal(
        BarePluralNominal::new(plural_head(noun))
            .expect("the Plural nominal accepts a Plural Head"),
    )
}

fn plural_head(noun: Noun) -> Head {
    Head::NounPluralHead(NounPluralHead::new(noun).expect("test noun is countable"))
}

fn noun_phrase(reference: UnqualifiedReference) -> NounPhrase {
    NounPhrase::QualifiedNounPhrase(QualifiedNounPhrase {
        reference: Box::new(PostmodifiedReference::UnqualifiedPostmodifiedReference(
            UnqualifiedPostmodifiedReference {
                reference: Box::new(reference),
            },
        )),
    })
}

fn indefinite(noun: Noun) -> NounPhrase {
    determined_singular(DeterminativeHeadLemma::IndefiniteArticle, noun)
}

fn target_noun(noun: Noun) -> NounPhrase {
    let determiner = Determinative::TargetingMarkerDeterminative(TargetingMarkerDeterminative {
        marker: TargetingMarker::Target,
    });
    let nominal = singular_nominal(noun);
    noun_phrase(UnqualifiedReference::DeterminedNominal(
        DeterminedNominal::new(Determiner::Headed(determiner), nominal)
            .expect("the targeting marker determinative agrees with its singular nominal"),
    ))
}

fn determined_singular(determiner: DeterminativeHeadLemma, noun: Noun) -> NounPhrase {
    let determiner = Determinative::SingularSimpleDeterminative(SingularSimpleDeterminative {
        head: DeterminativeHead::Closed(determiner),
    });
    let nominal = singular_nominal(noun);
    noun_phrase(UnqualifiedReference::DeterminedNominal(
        DeterminedNominal::new(Determiner::Headed(determiner), nominal)
            .expect("test determiner agrees with its singular nominal"),
    ))
}

fn determined_plural(determiner: Determiner, noun: Noun) -> NounPhrase {
    let nominal = plural_nominal(noun);
    noun_phrase(UnqualifiedReference::DeterminedNominal(
        DeterminedNominal::new(determiner, nominal)
            .expect("test determiner agrees with its plural nominal"),
    ))
}

fn creatures_you_control_with_power_at_most_two() -> NounPhrase {
    NounPhrase::QualifiedNounPhrase(QualifiedNounPhrase {
        reference: Box::new(PostmodifiedReference::PrepositionalQualifiedReference(
            PrepositionalQualifiedReference::new(
                Box::new(PostmodifiedReference::RelativeQualifiedReference(
                    RelativeQualifiedReference::new(
                        Box::new(PostmodifiedReference::UnqualifiedPostmodifiedReference(
                            UnqualifiedPostmodifiedReference {
                                reference: Box::new(UnqualifiedReference::DeterminedNominal(
                                    DeterminedNominal::new(
                                        Determiner::Zero,
                                        plural_nominal(creatures()),
                                    )
                                    .expect("a zero determiner agrees with a plural nominal"),
                                )),
                            },
                        )),
                        Box::new(ObjectGapRelativeClause::Positive(Box::new(
                            PositiveObjectGapRelativeClause::PositiveObjectGapRelative(
                                PositiveObjectGapRelativeClauseValue {
                                    subject: subject_you(),
                                    head: DeclarationTransitiveVerb::new(
                                        &environment(),
                                        VerbInventoryRef::Core(CoreVerbIdentity::Control),
                                    )
                                    .expect("the core inventory declares transitive Control"),
                                },
                            ),
                        ))),
                    )
                    .expect("the relative clause is admissible for the reference"),
                )),
                Box::new(PrepositionalPhrase::PrepositionalPhrase(
                    PrepositionalPhraseValue::new(
                        Preposition::With,
                        Box::new(PrepositionalComplement::ScalarMeasure(
                            ScalarMeasureValue::ScalarMeasureValue(ScalarMeasureValueValue {
                                measure: ScalarMeasure::NominalScalarMeasure(
                                    NominalScalarMeasure::new(singular_nominal(Noun::Lexeme(
                                        CommonNoun::Power,
                                    )))
                                    .expect("power is a Singular nominal scalar measure"),
                                ),
                                value: ScalarMeasureAssignedValue::Comparison(
                                    ScalarComparison::ScalarOrLess(
                                        ScalarOrLess::new(
                                            ScalarThreshold::FixedScalarThreshold(
                                                FixedScalarThreshold {
                                                    value: ScalarNumber { magnitude: 2 },
                                                },
                                            ),
                                            ComparisonDirection::Less,
                                        )
                                        .expect(
                                            "the closed member satisfies the direction requirement",
                                        ),
                                    ),
                                ),
                            }),
                        )),
                    )
                    .expect("the preposition licenses the complement"),
                )),
            )
            .expect("a declared object-attachment licence admits the scalar postmodifier"),
        )),
    })
}

fn number_of(counted: Object) -> NounPhrase {
    let number = UnqualifiedReference::DeterminedNominal(
        DeterminedNominal::new(
            Determiner::Headed(Determinative::SingularSimpleDeterminative(
                SingularSimpleDeterminative {
                    head: DeterminativeHead::Closed(DeterminativeHeadLemma::DefiniteArticle),
                },
            )),
            singular_nominal(Noun::Lexeme(CommonNoun::Number)),
        )
        .expect("the agrees with the singular number nominal"),
    );
    NounPhrase::QualifiedNounPhrase(QualifiedNounPhrase {
        reference: Box::new(PostmodifiedReference::PrepositionalQualifiedReference(
            PrepositionalQualifiedReference::new(
                Box::new(PostmodifiedReference::UnqualifiedPostmodifiedReference(
                    UnqualifiedPostmodifiedReference {
                        reference: Box::new(number),
                    },
                )),
                Box::new(PrepositionalPhrase::PrepositionalPhrase(
                    PrepositionalPhraseValue::new(
                        Preposition::Of,
                        Box::new(PrepositionalComplement::Object(Box::new(counted))),
                    )
                    .expect("the preposition licenses the complement"),
                )),
            )
            .expect("`of` is a licensed nominal postmodifier"),
        )),
    })
}

fn where_number_of(counted: Object) -> WhereClauseCategory {
    WhereClauseCategory::Where(WhereClause {
        clause: FiniteClause::PlainFiniteClause(
            PlainFiniteClause::new(
                Subject::VariableSubject(VariableSubject {
                    variable: Variable::X,
                }),
                Box::new(Predicate::FiniteCopular(Box::new(
                    FiniteCopularPredicate::FiniteCopularPredicate(FiniteCopularPredicateValue {
                        copula: FiniteCopula::Is,
                        complement: Box::new(PredicativeComplement::Nominal(Box::new(
                            PredicativeNominalComplement::PredicativeNominal(
                                PredicativeNominalValue {
                                    value: number_of(counted),
                                },
                            ),
                        ))),
                    }),
                ))),
            )
            .expect("X agrees with a third-person singular finite copular predicate"),
        ),
    })
}

fn that_noun(noun: Noun) -> NounPhrase {
    determined_singular(DeterminativeHeadLemma::DistalDemonstrative, noun)
}

fn those_noun(noun: Noun) -> NounPhrase {
    let determiner = Determinative::PluralSimpleDeterminative(PluralSimpleDeterminative {
        head: DeterminativeHead::Closed(DeterminativeHeadLemma::DistalDemonstrative),
    });
    determined_plural(Determiner::Headed(determiner), noun)
}

fn nominal_subject(value: NounPhrase) -> Subject {
    Subject::SubjectNominal(NominalSubject { value })
}

fn nominal_object(value: NounPhrase) -> Object {
    Object::ObjectNominal(
        NominalObject::new(Box::new(value)).expect("ordinary noun phrase is licensed as an object"),
    )
}

fn subject_you() -> Subject {
    Subject::SubjectPronoun(PersonalSubject {
        word: SubjectPronoun::You,
    })
}

fn target_creature() -> Object {
    nominal_object(target_noun(creature()))
}

fn it() -> Object {
    Object::ObjectPronoun(PersonalObject {
        word: ObjectPronoun::It,
    })
}

fn damage(amount: Amount) -> VerbPhrase {
    damage_to(amount, it())
}

fn quantified_mass_object(amount: Amount, noun: CommonNoun) -> Object {
    let determiner = Determiner::Headed(Determinative::MassQuantityDeterminer(
        MassQuantityDeterminer { amount },
    ));
    let nominal = Nominal::MassNominal(MassNominal {
        noun: MassNoun::MassNoun(
            MassNounValue::new(Noun::Lexeme(noun))
                .expect("the helper is called only with declared mass nouns"),
        ),
    });
    let reference = UnqualifiedReference::DeterminedNominal(
        DeterminedNominal::new(determiner, nominal)
            .expect("a quantity determiner licenses a mass noun"),
    );
    nominal_object(noun_phrase(reference))
}

fn damage_to(amount: Amount, recipient: Object) -> VerbPhrase {
    let environment = environment();
    let head =
        DeclarationToObjectVerb::new(&environment, VerbInventoryRef::Core(CoreVerbIdentity::Deal))
            .expect("the core inventory declares Deal with an object-to-object frame");
    VerbPhrase::DeclaredToObjectPredicate(DeclaredToObjectPredicate {
        head,
        object: quantified_mass_object(amount, CommonNoun::Damage),
        complement: recipient,
    })
}

fn gain_life(amount: Amount) -> VerbPhrase {
    let environment = environment();
    let head = DeclarationTransitiveVerb::new(
        &environment,
        VerbInventoryRef::Core(CoreVerbIdentity::Gain),
    )
    .expect("the core inventory declares Gain with a transitive frame");
    VerbPhrase::BaseVerbPhrase(BaseVerbPhrase {
        frame: Box::new(LexicalVerbPhrase::TransitiveLexicalVerbPhrase(Box::new(
            TransitiveLexicalVerbPhrase::TransitivePredicate(TransitivePredicate {
                head,
                object: quantified_mass_object(amount, CommonNoun::Life),
            }),
        ))),
    })
}

fn atomic(predicate: VerbPhrase) -> Predicate {
    Predicate::Atomic(Box::new(predicate))
}

fn imperative(predicate: VerbPhrase) -> Sentence {
    Sentence::Imperative(
        Imperative::new(Box::new(atomic(predicate)))
            .expect("the helper supplies a bare imperative predicate"),
    )
}

fn finite_clause(subject: Subject, predicate: VerbPhrase) -> FiniteClause {
    FiniteClause::PlainFiniteClause(
        PlainFiniteClause::new(subject, Box::new(atomic(predicate)))
            .expect("the helper supplies matching subject-predicate concord_class"),
    )
}

fn declarative(subject: Subject, predicate: VerbPhrase) -> Sentence {
    Sentence::Declarative(Declarative {
        clause: Box::new(Clause::Finite(Box::new(finite_clause(subject, predicate)))),
    })
}

fn body(values: Vec<Sentence>) -> AbilityBody {
    AbilityBody::Sentences(
        Sentences::new(Box::new(values)).expect("one or more sentences construct a body"),
    )
}

fn plain(values: Vec<Sentence>) -> Ability {
    Ability::Plain(Plain { body: body(values) })
}

fn destroy(object: Object) -> VerbPhrase {
    let environment = environment();
    let head = DeclarationTransitiveVerb::new(
        &environment,
        VerbInventoryRef::Declaration(DeclarationId::new(
            DeclarationKind::KeywordAction,
            "Destroy",
        )),
    )
    .expect("the builtin grammar declares transitive Destroy");
    VerbPhrase::BaseVerbPhrase(BaseVerbPhrase {
        frame: Box::new(LexicalVerbPhrase::TransitiveLexicalVerbPhrase(Box::new(
            TransitiveLexicalVerbPhrase::TransitivePredicate(TransitivePredicate { head, object }),
        ))),
    })
}

fn connive() -> VerbPhrase {
    let environment = environment();
    let head = DeclarationIntransitiveVerb::new(
        &environment,
        VerbInventoryRef::Declaration(DeclarationId::new(
            DeclarationKind::KeywordAction,
            "Connive",
        )),
    )
    .expect("the builtin grammar declares intransitive Connive");
    VerbPhrase::BaseVerbPhrase(BaseVerbPhrase {
        frame: Box::new(LexicalVerbPhrase::IntransitiveLexicalVerbPhrase(
            IntransitiveLexicalVerbPhrase::IntransitivePredicate(IntransitivePredicate { head }),
        )),
    })
}

fn declared_action_name(predicate: &VerbPhrase) -> Option<&str> {
    match predicate {
        VerbPhrase::BaseVerbPhrase(BaseVerbPhrase { frame }) => match frame.as_ref() {
            LexicalVerbPhrase::IntransitiveLexicalVerbPhrase(
                IntransitiveLexicalVerbPhrase::IntransitivePredicate(IntransitivePredicate {
                    head,
                }),
            ) => match head.reference() {
                VerbInventoryRef::Declaration(id) => Some(id.name()),
                VerbInventoryRef::Core(_) => None,
            },
            LexicalVerbPhrase::TransitiveLexicalVerbPhrase(transitive_lexical_verb_phrase) => {
                match transitive_lexical_verb_phrase.as_ref() {
                    TransitiveLexicalVerbPhrase::TransitivePredicate(TransitivePredicate {
                        head,
                        ..
                    }) => match head.reference() {
                        VerbInventoryRef::Declaration(id) => Some(id.name()),
                        VerbInventoryRef::Core(_) => None,
                    },
                }
            }
            _ => None,
        },
        _ => None,
    }
}

fn triggered(trigger_clause: FiniteClause, consequences: Vec<Sentence>) -> Triggered {
    Triggered {
        trigger: TriggerPrefix::Finite(Finite {
            marker: TriggerMarker::Whenever,
            clause: Box::new(Clause::Finite(Box::new(trigger_clause))),
        }),
        intervening_if: Box::new(None),
        body: body(consequences),
    }
}

fn triggered_damage() -> Ability {
    let event = finite_clause(
        nominal_subject(indefinite(Noun::Lexeme(CommonNoun::Player))),
        connive(),
    );
    let effect = declarative(
        nominal_subject(that_noun(creature())),
        damage(Amount::Variable(VariableAmount {
            variable: Variable::X,
        })),
    );
    Ability::Triggered(triggered(event, vec![effect]))
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
    let ability = plain(vec![imperative(gain_life(Amount::Number(NumberAmount {
        number,
    })))]);
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
    let ability = plain(vec![imperative(gain_life(Amount::Number(NumberAmount {
        number: scalar.clone(),
    })))]);
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

    let y_ability = plain(vec![imperative(gain_life(Amount::Variable(
        VariableAmount {
            variable: Variable::Y,
        },
    )))]);
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
    Sentence::WithWhere(WithWhere {
        body: Box::new(declarative(
            subject_you(),
            gain_life(Amount::Variable(VariableAmount {
                variable: Variable::X,
            })),
        )),
        clause: where_number_of(nominal_object(
            creatures_you_control_with_power_at_most_two(),
        )),
    })
}

#[test]
fn paragraph_and_oracle_text_constructors_and_traversal_preserve_structural_order() {
    #[derive(Debug, PartialEq, Eq)]
    enum Event {
        Block,
        Plain,
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

        fn visit_plain(&mut self, plain: &Plain) {
            self.0.push(Event::Plain);
            deckmaste_english_v2::visit::walk_plain(self, plain);
        }

        fn visit_triggered(&mut self, triggered: &Triggered) {
            self.0.push(Event::Triggered);
            deckmaste_english_v2::visit::walk_triggered(self, triggered);
        }

        fn visit_sentence(&mut self, sentence: &Sentence) {
            let label = match sentence {
                Sentence::Imperative(value) if matches!(value.predicate(), Predicate::Atomic(predicate) if declared_action_name(predicate) == Some("Destroy")) => {
                    "destroy"
                }
                Sentence::Imperative(value) if matches!(value.predicate(), Predicate::Atomic(predicate) if declared_action_name(predicate) == Some("Connive")) => {
                    "connive"
                }
                Sentence::Declarative(_) => "gain",
                Sentence::WithWhere(_) => "where",
                Sentence::Imperative(_) | Sentence::Attached(_) => "other",
            };
            self.0.push(Event::Sentence(label));
            deckmaste_english_v2::visit::walk_sentence(self, sentence);
        }
    }

    let destroy = imperative(destroy(target_creature()));
    let gain = declarative(
        subject_you(),
        gain_life(Amount::Number(NumberAmount {
            number: ScalarNumber { magnitude: 2 },
        })),
    );
    let connive_sentence = imperative(connive());

    assert!(Sentences::new(Box::default()).is_none());
    let paragraph_sentences = vec![destroy, gain.clone()];
    let paragraph = Sentences::new(Box::new(paragraph_sentences.clone()))
        .expect("a paragraph accepts one or more sentences");
    assert_eq!(paragraph.sentences(), paragraph_sentences.as_slice());

    let event = finite_clause(subject_you(), connive());
    assert!(
        Sentences::new(Box::default()).is_none(),
        "an ability body is nonempty"
    );
    let triggered_effects = vec![connive_sentence, gain];
    let triggered = triggered(event, triggered_effects.clone());
    let AbilityBody::Sentences(triggered_body) = &triggered.body else {
        panic!("the triggered fixture has an ordinary sentence body")
    };
    assert_eq!(triggered_body.sentences(), triggered_effects.as_slice());

    let empty_oracle_text = OracleText {
        blocks: Box::default(),
    };
    assert_eq!(
        empty_oracle_text.render(&context("Grizzly Bears"), &environment()),
        ""
    );
    assert!(empty_oracle_text.blocks.is_empty());

    let mut empty_visitor = StructuralVisitor::default();
    deckmaste_english_v2::visit::walk_oracle_text(&mut empty_visitor, &empty_oracle_text);
    assert!(empty_visitor.0.is_empty());

    let blocks = vec![
        DocumentBlock::Ability(Box::new(Ability::Plain(Plain {
            body: AbilityBody::Sentences(paragraph),
        }))),
        DocumentBlock::Ability(Box::new(Ability::Triggered(triggered))),
    ];
    let oracle_text = OracleText {
        blocks: Box::new(blocks.clone()),
    };
    assert_eq!(oracle_text.blocks.as_ref(), &blocks);

    let mut visitor = StructuralVisitor::default();
    deckmaste_english_v2::visit::walk_oracle_text(&mut visitor, &oracle_text);
    assert_eq!(
        visitor.0,
        [
            Event::Block,
            Event::Plain,
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
    let event = finite_clause(subject_you(), connive());
    let effect = imperative(connive());
    let value = triggered(event, vec![effect.clone()]);
    let AbilityBody::Sentences(body) = value.body else {
        panic!("the generated invariant fixture has an ordinary sentence body")
    };
    assert_eq!(body.sentences(), [effect]);
}

#[test]
fn renders_destroy_target_creature_exactly() {
    let value = plain(vec![imperative(destroy(target_creature()))]);
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
    let value = declarative(
        nominal_subject(creatures_you_control_with_power_at_most_two()),
        gain_life(Amount::Variable(VariableAmount {
            variable: Variable::X,
        })),
    );

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
    let value = declarative(
        nominal_subject(noun_phrase(UnqualifiedReference::SelfReference(subject))),
        damage_to(
            Amount::Number(NumberAmount {
                number: ScalarNumber { magnitude: 3 },
            }),
            target_creature(),
        ),
    );
    assert_eq!(
        value.render(
            &legendary_context("Zacama, Primal Calamity"),
            &environment()
        ),
        "Zacama deals 3 damage to target creature."
    );
}

#[test]
fn the_same_self_reference_value_renders_from_two_card_contexts() {
    let value = declarative(
        nominal_subject(noun_phrase(UnqualifiedReference::SelfReference(
            self_reference(
                SelfReferenceSpelling::Abbreviated,
                "Zacama, Primal Calamity",
            ),
        ))),
        damage_to(
            Amount::Number(NumberAmount {
                number: ScalarNumber { magnitude: 3 },
            }),
            target_creature(),
        ),
    );

    let zacama = value.render(
        &legendary_context("Zacama, Primal Calamity"),
        &environment(),
    );
    let zoraline = value.render(
        &legendary_context("Zoraline, Cosmos Caller"),
        &environment(),
    );

    assert_eq!(zacama, "Zacama deals 3 damage to target creature.");
    assert_eq!(zoraline, "Zoraline deals 3 damage to target creature.");
    assert_eq!(zacama.replacen("Zacama", "Zoraline", 1), zoraline);
}

#[test]
fn renders_those_with_a_plural_noun_and_bare_verb() {
    let value = declarative(
        nominal_subject(those_noun(creatures())),
        damage(Amount::Number(NumberAmount {
            number: ScalarNumber { magnitude: 3 },
        })),
    );
    assert_eq!(
        value.render(&context("Context Card"), &environment()),
        "Those creatures deal 3 damage to it."
    );
}

#[test]
fn demonstrative_references_visit_their_unified_nominal_ast_nodes() {
    #[derive(Debug, PartialEq, Eq)]
    enum Event {
        DeterminedNominal,
        Determinative,
        Nominal,
        Head(DeclarationKind, String),
    }

    #[derive(Default)]
    struct DemonstrativeVisitor(Vec<Event>);

    impl Visitor for DemonstrativeVisitor {
        fn visit_determined_nominal(&mut self, nominal: &DeterminedNominal) {
            self.0.push(Event::DeterminedNominal);
            deckmaste_english_v2::visit::walk_determined_nominal(self, nominal);
        }

        fn visit_determinative(&mut self, determiner: &Determinative) {
            self.0.push(Event::Determinative);
            deckmaste_english_v2::visit::walk_determinative(self, determiner);
        }

        fn visit_nominal(&mut self, nominal: &Nominal) {
            self.0.push(Event::Nominal);
            deckmaste_english_v2::visit::walk_nominal(self, nominal);
        }

        fn visit_declaration_noun(&mut self, noun: &DeclarationNoun) {
            self.0
                .push(Event::Head(noun.id().kind(), noun.id().name().to_owned()));
        }
    }

    for (value, expected) in [
        (that_noun(creature()), "That creature deals 3 damage to it."),
        (
            those_noun(creatures()),
            "Those creatures deal 3 damage to it.",
        ),
    ] {
        let sentence = declarative(
            nominal_subject(value.clone()),
            damage(Amount::Number(NumberAmount {
                number: ScalarNumber { magnitude: 3 },
            })),
        );
        assert_eq!(
            sentence.render(&context("Context Card"), &environment()),
            expected,
        );

        let mut visitor = DemonstrativeVisitor::default();
        deckmaste_english_v2::visit::walk_noun_phrase(&mut visitor, &value);
        assert_eq!(
            visitor.0,
            [
                Event::DeterminedNominal,
                Event::Determinative,
                Event::Nominal,
                Event::Head(DeclarationKind::Type, "Creature".to_owned(),),
            ],
        );
    }
}

#[test]
fn renders_an_with_a_singular_noun_and_third_person_verb() {
    let value = declarative(
        nominal_subject(indefinite(artifact())),
        damage(Amount::Number(NumberAmount {
            number: ScalarNumber { magnitude: 3 },
        })),
    );
    assert_eq!(
        value.render(&context("Context Card"), &environment()),
        "An artifact deals 3 damage to it."
    );
}

#[test]
fn renders_a_subtype_with_its_printed_case() {
    let value = declarative(
        nominal_subject(indefinite(equipment())),
        damage(Amount::Number(NumberAmount {
            number: ScalarNumber { magnitude: 3 },
        })),
    );
    assert_eq!(
        value.render(&context("Context Card"), &environment()),
        "An Equipment deals 3 damage to it."
    );
}

#[test]
fn visitor_reaches_every_vertical_slice_leaf() {
    let destroy = plain(vec![imperative(destroy(target_creature()))]);
    let self_reference = declarative(
        nominal_subject(noun_phrase(UnqualifiedReference::SelfReference(
            self_reference(
                SelfReferenceSpelling::Abbreviated,
                "Zacama, Primal Calamity",
            ),
        ))),
        damage_to(
            Amount::Number(NumberAmount {
                number: ScalarNumber { magnitude: 3 },
            }),
            target_creature(),
        ),
    );

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
    assert_eq!(visitor.trigger_markers, vec![TriggerMarker::Whenever]);
    assert_eq!(
        visitor.nouns,
        vec![
            CommonNoun::Player,
            CommonNoun::Damage,
            CommonNoun::Life,
            CommonNoun::Number,
            CommonNoun::Power,
            CommonNoun::Damage,
        ]
    );
    assert_eq!(
        visitor.declarations,
        vec![
            (
                deckmaste_construction_core::macro_def::DeclarationKind::KeywordAction,
                "Destroy".to_owned()
            ),
            (
                deckmaste_construction_core::macro_def::DeclarationKind::KeywordAction,
                "Connive".to_owned()
            ),
        ]
    );
    assert_eq!(
        visitor.verbs,
        vec![
            VerbInventoryRef::Declaration(DeclarationId::new(
                DeclarationKind::KeywordAction,
                "Destroy",
            )),
            VerbInventoryRef::Declaration(DeclarationId::new(
                DeclarationKind::KeywordAction,
                "Connive",
            )),
            VerbInventoryRef::Core(CoreVerbIdentity::Deal),
            VerbInventoryRef::Core(CoreVerbIdentity::Gain),
            VerbInventoryRef::Core(CoreVerbIdentity::Control),
            VerbInventoryRef::Core(CoreVerbIdentity::Deal),
        ]
    );
    assert_eq!(visitor.finite_copulas, vec![FiniteCopula::Is]);
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
