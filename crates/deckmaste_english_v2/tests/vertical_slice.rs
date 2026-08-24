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
    trigger_markers: Vec<TriggerMarker>,
    nouns: Vec<CommonNoun>,
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

    fn visit_verb_lexeme(&mut self, verb: VerbLexeme) {
        self.verbs.push(verb);
    }

    fn visit_declaration(&mut self, declaration: &macro_ron::v2::DeclarationIdentity) {
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

enum Noun {
    Lexeme(CommonNoun),
    Type(TypeNoun),
    ArtifactSubtype(ArtifactSubtypeNoun),
}

fn card_type(spelling: &str) -> Noun {
    declaration_noun(DeclarationKind::Type, spelling, SurfaceFeature::Singular)
}

fn declaration_noun(kind: DeclarationKind, name: &str, _feature: SurfaceFeature) -> Noun {
    let environment = environment();
    let id = DeclarationId::new(kind, name);
    match kind {
        DeclarationKind::Type => Noun::Type(TypeNoun::Declaration(
            DeclarationTypeNoun::new(&environment, id)
                .expect("normalized Type noun declaration is present"),
        )),
        DeclarationKind::Subtype(SubtypeCategory::Artifact) => {
            Noun::ArtifactSubtype(ArtifactSubtypeNoun::Declaration(
                DeclarationArtifactSubtypeNoun::new(&environment, id)
                    .expect("normalized Subtype noun declaration is present"),
            ))
        }
        DeclarationKind::Subtype(_) => {
            panic!("this vertical-slice builder uses only its Artifact subtype fixture")
        }
        _ => panic!("test helper accepts only Type/Subtype noun identities"),
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
    ParseContext::new(card_name, false, macro_ron::v2::Onset::Consonant)
        .expect("test card name is a valid parse context")
}

fn legendary_context(card_name: &str) -> ParseContext<'_> {
    ParseContext::new(card_name, true, macro_ron::v2::Onset::Consonant)
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

fn singular_nominal(noun: Noun) -> SingularNominal {
    SingularNominal::BareSingularNominal(BareSingularNominal {
        head: match noun {
            Noun::Lexeme(noun) => SingularHead::CommonSingularHead(CommonSingularHead { noun }),
            Noun::Type(noun) => SingularHead::TypeSingularHead(TypeSingularHead { noun }),
            Noun::ArtifactSubtype(noun) => {
                SingularHead::ArtifactSubtypeSingularHead(ArtifactSubtypeSingularHead { noun })
            }
        },
    })
}

fn plural_nominal(noun: Noun) -> PluralNominal {
    PluralNominal::BarePluralNominal(BarePluralNominal {
        head: plural_head(noun),
    })
}

fn plural_head(noun: Noun) -> PluralHead {
    match noun {
        Noun::Lexeme(noun) => PluralHead::CommonPluralHead(CommonPluralHead { noun }),
        Noun::Type(noun) => PluralHead::TypePluralHead(TypePluralHead { noun }),
        Noun::ArtifactSubtype(noun) => {
            PluralHead::ArtifactSubtypePluralHead(ArtifactSubtypePluralHead { noun })
        }
    }
}

fn noun_phrase(reference: UnqualifiedReference) -> NounPhrase {
    NounPhrase::QualifiedNounPhrase(QualifiedNounPhrase {
        reference: NumericStage::UnqualifiedNumericStage(UnqualifiedNumericStage {
            reference: ZoneStage::UnqualifiedZoneStage(UnqualifiedZoneStage {
                reference: ControllerStage::UnqualifiedControllerStage(
                    UnqualifiedControllerStage { reference },
                ),
            }),
        }),
    })
}

fn indefinite(noun: Noun) -> NounPhrase {
    noun_phrase(UnqualifiedReference::IndefiniteReference(
        IndefiniteReference {
            nominal: singular_nominal(noun),
        },
    ))
}

fn target_noun(noun: Noun) -> NounPhrase {
    noun_phrase(UnqualifiedReference::OrdinarySingularReference(
        OrdinarySingularReference::new(SingularSelector::TargetSingularSelector(
            TargetSingularSelector {
                nominal: singular_nominal(noun),
            },
        ))
        .expect("a target selector is an ordinary singular reference"),
    ))
}

fn creatures_you_control_with_power_at_most_two() -> NounPhrase {
    NounPhrase::QualifiedNounPhrase(QualifiedNounPhrase {
        reference: NumericStage::ScalarQualifiedReference(ScalarQualifiedReference {
            reference: ZoneStage::UnqualifiedZoneStage(UnqualifiedZoneStage {
                reference: ControllerStage::ControllerQualifiedReference(
                    ControllerQualifiedReference {
                        reference: UnqualifiedReference::OrdinaryPluralReference(
                            OrdinaryPluralReference {
                                selector: PluralSelector::UnmarkedPluralSelector(
                                    UnmarkedPluralSelector {
                                        nominal: plural_nominal(creatures()),
                                    },
                                ),
                            },
                        ),
                        controller_owner: ControllerOwnerQualification::YouControl(
                            YouControl::new(SubjectPronoun::You)
                                .expect("You is a valid controller"),
                        ),
                    },
                ),
            }),
            scalar: ScalarQualification::ScalarQualification(ScalarQualificationValue {
                measure: ScalarMeasure::CharacteristicScalar(CharacteristicScalar {
                    characteristic: ScalarCharacteristic::Power,
                }),
                comparison: ScalarComparison::ScalarOrLess(ScalarOrLess {
                    threshold: ScalarThreshold::FixedScalarThreshold(FixedScalarThreshold {
                        value: ScalarNumber { magnitude: 2 },
                    }),
                }),
            }),
        }),
    })
}

fn that_noun(noun: Noun) -> NounPhrase {
    noun_phrase(UnqualifiedReference::ThatReference(ThatReference {
        nominal: singular_nominal(noun),
    }))
}

fn those_noun(noun: Noun) -> NounPhrase {
    noun_phrase(UnqualifiedReference::ThoseReference(ThoseReference {
        nominal: plural_nominal(noun),
    }))
}

fn nominal_subject(value: NounPhrase) -> Subject {
    Subject::SubjectNominal(NominalSubject { value })
}

fn nominal_object(value: NounPhrase) -> Object {
    Object::ObjectNominal(NominalObject { value })
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
    VerbPhrase::DealDamage(DealDamage { to: it(), amount })
}

fn atomic(predicate: VerbPhrase) -> Predicate {
    Predicate::Atomic(predicate)
}

fn imperative(predicate: VerbPhrase) -> Sentence {
    Sentence::Imperative(Imperative {
        predicate: atomic(predicate),
    })
}

fn finite_clause(subject: Subject, predicate: VerbPhrase) -> FiniteClause {
    FiniteClause::PlainFiniteClause(PlainFiniteClause {
        subject,
        predicate: atomic(predicate),
    })
}

fn declarative(subject: Subject, predicate: VerbPhrase) -> Sentence {
    Sentence::Declarative(Declarative {
        clause: Clause::Finite(finite_clause(subject, predicate)),
    })
}

fn body(values: Vec<Sentence>) -> AbilityBody {
    AbilityBody::Sentences(Sentences::new(values).expect("one or more sentences construct a body"))
}

fn plain(values: Vec<Sentence>) -> Ability {
    Ability::Plain(Plain { body: body(values) })
}

fn triggered(trigger_clause: FiniteClause, consequences: Vec<Sentence>) -> Triggered {
    Triggered {
        trigger: TriggerPrefix::Finite(Finite {
            marker: TriggerMarker::Whenever,
            clause: trigger_clause,
        }),
        intervening_if: None,
        body: body(consequences),
    }
}

fn triggered_damage() -> Ability {
    let event = finite_clause(
        nominal_subject(indefinite(Noun::Lexeme(CommonNoun::Player))),
        VerbPhrase::Connive(Connive),
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
    let ability = plain(vec![imperative(VerbPhrase::GainLife(GainLife {
        amount: Amount::Number(NumberAmount { number }),
    }))]);
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
    let ability = plain(vec![imperative(VerbPhrase::GainLife(GainLife {
        amount: Amount::Number(NumberAmount {
            number: scalar.clone(),
        }),
    }))]);
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

    let y_ability = plain(vec![imperative(VerbPhrase::GainLife(GainLife {
        amount: Amount::Variable(VariableAmount {
            variable: Variable::Y,
        }),
    }))]);
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
            VerbPhrase::GainLife(GainLife {
                amount: Amount::Variable(VariableAmount {
                    variable: Variable::X,
                }),
            }),
        )),
        clause: WhereClauseCategory::Where(WhereClause {
            variable: Variable::X,
            value: nominal_object(creatures_you_control_with_power_at_most_two()),
        }),
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
                Sentence::Imperative(Imperative {
                    predicate: Predicate::Atomic(VerbPhrase::Destroy(_)),
                }) => "destroy",
                Sentence::Imperative(Imperative {
                    predicate: Predicate::Atomic(VerbPhrase::Connive(_)),
                }) => "connive",
                Sentence::Declarative(_) => "gain",
                Sentence::WithWhere(_) => "where",
                Sentence::Imperative(_)
                | Sentence::PreposedIf(_)
                | Sentence::PostposedIf(_)
                | Sentence::PostposedUnless(_)
                | Sentence::PreposedAsLongAs(_)
                | Sentence::PreposedWhile(_)
                | Sentence::PreposedDuring(_)
                | Sentence::PreposedUntil(_)
                | Sentence::ThenSequence(_)
                | Sentence::ReflexiveSubordinate(_) => "other",
            };
            self.0.push(Event::Sentence(label));
            deckmaste_english_v2::visit::walk_sentence(self, sentence);
        }
    }

    let destroy = imperative(VerbPhrase::Destroy(Destroy {
        object: target_creature(),
    }));
    let gain = declarative(
        subject_you(),
        VerbPhrase::GainLife(GainLife {
            amount: Amount::Number(NumberAmount {
                number: ScalarNumber { magnitude: 2 },
            }),
        }),
    );
    let connive = imperative(VerbPhrase::Connive(Connive));

    assert!(Sentences::new(vec![]).is_none());
    let paragraph_sentences = vec![destroy, gain.clone()];
    let paragraph = Sentences::new(paragraph_sentences.clone())
        .expect("a paragraph accepts one or more sentences");
    assert_eq!(paragraph.sentences(), paragraph_sentences.as_slice());

    let event = finite_clause(subject_you(), VerbPhrase::Connive(Connive));
    assert!(
        Sentences::new(vec![]).is_none(),
        "an ability body is nonempty"
    );
    let triggered_effects = vec![connive, gain];
    let triggered = triggered(event, triggered_effects.clone());
    let AbilityBody::Sentences(triggered_body) = &triggered.body;
    assert_eq!(triggered_body.sentences(), triggered_effects.as_slice());

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
        DocumentBlock::Ability(Ability::Plain(Plain {
            body: AbilityBody::Sentences(paragraph),
        })),
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
        DeclarationTypeNoun::new(
            &environment,
            DeclarationId::new(DeclarationKind::Type, "Creature"),
        )
        .is_some()
    );
    assert!(
        DeclarationTypeNoun::new(
            &environment,
            DeclarationId::new(DeclarationKind::Type, "Definitely Not A Type"),
        )
        .is_none()
    );
    assert!(
        DeclarationTypeNoun::new(
            &environment,
            DeclarationId::new(DeclarationKind::KeywordAbility, "Flying"),
        )
        .is_none(),
        "a present but disallowed declaration kind cannot become a noun"
    );
}

#[test]
fn generated_invariant_triggered_compile_surface_stores_and_accepts_nonempty_effects() {
    let event = finite_clause(subject_you(), VerbPhrase::Connive(Connive));
    let effect = imperative(VerbPhrase::Connive(Connive));
    let value = triggered(event, vec![effect.clone()]);
    let AbilityBody::Sentences(body) = value.body;
    assert_eq!(body.sentences(), [effect]);
}

#[test]
fn renders_destroy_target_creature_exactly() {
    let value = plain(vec![imperative(VerbPhrase::Destroy(Destroy {
        object: target_creature(),
    }))]);
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
        VerbPhrase::GainLife(GainLife {
            amount: Amount::Variable(VariableAmount {
                variable: Variable::X,
            }),
        }),
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
        VerbPhrase::DealDamage(DealDamage {
            amount: Amount::Number(NumberAmount {
                number: ScalarNumber { magnitude: 3 },
            }),
            to: target_creature(),
        }),
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
        VerbPhrase::DealDamage(DealDamage {
            amount: Amount::Number(NumberAmount {
                number: ScalarNumber { magnitude: 3 },
            }),
            to: target_creature(),
        }),
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
fn demonstrative_references_visit_their_distinct_typed_ast_nodes() {
    #[derive(Debug, PartialEq, Eq)]
    enum Event {
        ThatReference,
        ThoseReference,
        Head(DeclarationKind, String),
    }

    #[derive(Default)]
    struct DemonstrativeVisitor(Vec<Event>);

    impl Visitor for DemonstrativeVisitor {
        fn visit_that_reference(&mut self, reference: &ThatReference) {
            self.0.push(Event::ThatReference);
            deckmaste_english_v2::visit::walk_that_reference(self, reference);
        }

        fn visit_those_reference(&mut self, reference: &ThoseReference) {
            self.0.push(Event::ThoseReference);
            deckmaste_english_v2::visit::walk_those_reference(self, reference);
        }

        fn visit_declaration_type_noun(&mut self, noun: &DeclarationTypeNoun) {
            self.0
                .push(Event::Head(noun.id().kind(), noun.id().name().to_owned()));
        }
    }

    for (value, expected, expected_event) in [
        (
            that_noun(creature()),
            "That creature deals 3 damage to it.",
            Event::ThatReference,
        ),
        (
            those_noun(creatures()),
            "Those creatures deal 3 damage to it.",
            Event::ThoseReference,
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
                expected_event,
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
    let destroy = plain(vec![imperative(VerbPhrase::Destroy(Destroy {
        object: target_creature(),
    }))]);
    let self_reference = declarative(
        nominal_subject(noun_phrase(UnqualifiedReference::SelfReference(
            self_reference(
                SelfReferenceSpelling::Abbreviated,
                "Zacama, Primal Calamity",
            ),
        ))),
        VerbPhrase::DealDamage(DealDamage {
            amount: Amount::Number(NumberAmount {
                number: ScalarNumber { magnitude: 3 },
            }),
            to: target_creature(),
        }),
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
    assert_eq!(visitor.nouns, vec![CommonNoun::Player]);
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
