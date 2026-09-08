use deckmaste_construction_core::macro_def::CustomTailAtom;
use deckmaste_construction_core::macro_def::DeclarationKind;
use deckmaste_construction_core::macro_def::GrammarPosition;
use deckmaste_construction_core::macro_def::NormalizedDeclaration;
use deckmaste_construction_core::macro_def::Onset;
use deckmaste_construction_core::macro_def::SurfaceFeature;
use deckmaste_construction_core::macro_def::read_str;
use deckmaste_english_v2::ast::CatalogProvider;
use deckmaste_english_v2::ast::CommonNoun;
use deckmaste_english_v2::ast::DeclarationTransitiveVerb;
use deckmaste_english_v2::context::ParseContext;
use deckmaste_english_v2::environment::CatalogProviderRow;
use deckmaste_english_v2::environment::CatalogProviderRows;
use deckmaste_english_v2::environment::DeclarationId;
use deckmaste_english_v2::environment::ParserEnvironment;
use deckmaste_english_v2::environment::VerbInventoryRef;
use deckmaste_english_v2::parser::Parser;
use deckmaste_english_v2::render::Render;
use deckmaste_english_v2::visit::Visitor;
use deckmaste_english_v2::visit::walk_ability;

fn declaration(path: &str, source: &str) -> NormalizedDeclaration {
    read_str(path, source).expect("synthetic normalized declaration is valid")
}

fn synthetic_verb_rows() -> Vec<NormalizedDeclaration> {
    vec![
        declaration(
            "/synthetic/actions/Destroy.ron",
            r#"KeywordAction(name:"Destroy",spelling:"frindle",grammar:Verb(bare:"frindle",third_person:"frondles",preterite:"frindled",frame_set:Transitive))"#,
        ),
        declaration(
            "/synthetic/actions/Connive.ron",
            r#"KeywordAction(name:"Connive",spelling:"zorble",grammar:Verb(bare:"zorble",third_person:"zurbles",preterite:"zorbled",frame_set:Intransitive))"#,
        ),
    ]
}

fn synthetic_environment() -> ParserEnvironment {
    environment_from(synthetic_verb_rows()).expect("synthetic verb declarations freeze")
}

fn environment_from(
    declarations: impl IntoIterator<Item = NormalizedDeclaration>,
) -> Result<ParserEnvironment, deckmaste_english_v2::environment::ParserEnvironmentError> {
    ParserEnvironment::try_from_parts(
        declarations,
        [CatalogProviderRows::new(
            CatalogProvider::CardNames,
            [CatalogProviderRow::new(
                "context-card",
                "Context Card",
                Onset::Consonant,
            )],
        )],
    )
}

fn parser() -> Parser {
    Parser::new(synthetic_environment()).expect("required declarations are present")
}

fn context() -> ParseContext<'static> {
    ParseContext::new(
        "Context Card",
        false,
        deckmaste_construction_core::macro_def::Onset::Consonant,
    )
    .expect("context is valid")
}

#[test]
fn environment_declaration_verb_readings_use_literal_exact_frame_membership() {
    let environment = environment_from([
        declaration(
            "/synthetic/actions/FirstAct.ron",
            r#"KeywordAction(name:"FirstAct",spelling:"act",grammar:Verb(bare:"act",frame_set:Transitive))"#,
        ),
        declaration(
            "/synthetic/actions/SecondAct.ron",
            r#"KeywordAction(name:"SecondAct",spelling:"act",grammar:Verb(bare:"act",frame_set:Custom(frames:[[ObjectNounPhrase]])))"#,
        ),
        declaration(
            "/synthetic/actions/Rest.ron",
            r#"KeywordAction(name:"Rest",spelling:"rest",grammar:Verb(bare:"rest",frame_set:Intransitive))"#,
        ),
        declaration(
            "/synthetic/actions/Count.ron",
            r#"KeywordAction(name:"Count",spelling:"count",grammar:Verb(bare:"count",frame_set:MeasureComplement))"#,
        ),
        declaration(
            "/synthetic/actions/Shape.ron",
            r#"KeywordAction(name:"Shape",spelling:"shape",grammar:Verb(bare:"shape",frame_set:Custom(frames:[[],[Amount]])))"#,
        ),
        declaration(
            "/synthetic/actions/Cross.ron",
            r#"KeywordAction(name:"Cross",spelling:"cross",grammar:Verb(bare:"cross",third_person:"crosses",frame_set:Custom(frames:[[ObjectNounPhrase,Amount]])))"#,
        ),
        declaration(
            "/synthetic/abilities/WrongKind.ron",
            r#"KeywordAbility(name:"WrongKind",spelling:"act",grammar:Verb(bare:"act",frame_set:Transitive))"#,
        ),
        declaration(
            "/synthetic/actions/WrongPosition.ron",
            r#"KeywordAction(name:"WrongPosition",spelling:"act",grammar:FixedTerm(surface:"act"))"#,
        ),
    ])
    .expect("synthetic declaration frame matrix freezes");
    let names = |surface, feature, frame: &[CustomTailAtom]| {
        environment
            .declaration_verb_readings(surface, feature, frame)
            .into_iter()
            .map(|reading| reading.id().name().to_owned())
            .collect::<Vec<_>>()
    };

    assert_eq!(
        names(
            "act",
            SurfaceFeature::PLAIN,
            &[CustomTailAtom::ObjectNounPhrase]
        ),
        ["FirstAct", "SecondAct", "WrongKind"]
    );
    assert_eq!(names("rest", SurfaceFeature::PLAIN, &[]), ["Rest"]);
    assert_eq!(
        names("count", SurfaceFeature::PLAIN, &[CustomTailAtom::Amount]),
        ["Count"]
    );
    assert_eq!(names("shape", SurfaceFeature::PLAIN, &[]), ["Shape"]);
    assert_eq!(
        names("shape", SurfaceFeature::PLAIN, &[CustomTailAtom::Amount]),
        ["Shape"]
    );
    assert!(
        names(
            "shape",
            SurfaceFeature::PLAIN,
            &[CustomTailAtom::ObjectNounPhrase]
        )
        .is_empty()
    );
    assert!(
        names(
            "cross",
            SurfaceFeature::PLAIN,
            &[CustomTailAtom::ObjectNounPhrase]
        )
        .is_empty()
    );
}

#[test]
fn environment_declaration_verb_readings_filter_position_surface_and_concord_class() {
    let environment = environment_from([
        declaration(
            "/synthetic/actions/Right.ron",
            r#"KeywordAction(name:"Right",spelling:"echo",grammar:Verb(bare:"echo",third_person:"echoes",frame_set:Transitive))"#,
        ),
        declaration(
            "/synthetic/abilities/WrongKind.ron",
            r#"KeywordAbility(name:"WrongKind",spelling:"echo",grammar:Verb(bare:"echo",third_person:"echoes",frame_set:Transitive))"#,
        ),
        declaration(
            "/synthetic/actions/WrongPosition.ron",
            r#"KeywordAction(name:"WrongPosition",spelling:"echo",grammar:FixedTerm(surface:"echo"))"#,
        ),
        declaration(
            "/synthetic/actions/MissingThird.ron",
            r#"KeywordAction(name:"MissingThird",spelling:"wane",grammar:Verb(bare:"wane",third_person:Unavailable,frame_set:Transitive))"#,
        ),
    ])
    .expect("synthetic declaration filters freeze");
    let names = |surface, feature| {
        environment
            .declaration_verb_readings(surface, feature, &[CustomTailAtom::ObjectNounPhrase])
            .into_iter()
            .map(|reading| reading.id().name().to_owned())
            .collect::<Vec<_>>()
    };

    assert_eq!(names("echo", SurfaceFeature::PLAIN), ["Right", "WrongKind"]);
    assert_eq!(
        names("echoes", SurfaceFeature::THIRD_PERSON_SINGULAR_PRESENT),
        ["Right", "WrongKind"]
    );
    assert!(names("echo", SurfaceFeature::THIRD_PERSON_SINGULAR_PRESENT).is_empty());
    assert!(names("wane", SurfaceFeature::THIRD_PERSON_SINGULAR_PRESENT).is_empty());
    assert!(names("missing", SurfaceFeature::PLAIN).is_empty());
}

#[test]
fn parser_build_has_no_fixed_keyword_requirements_and_constructors_fail_closed() {
    Parser::new(environment_from([]).unwrap())
        .expect("the shared frames have no fixed keyword-name requirements");

    let wrong_recipe = environment_from([
        declaration(
            "/synthetic/actions/Destroy.ron",
            r#"KeywordAction(name:"Destroy",spelling:"destroy",grammar:FixedTerm(surface:"destroy"))"#,
        ),
        declaration(
            "/synthetic/actions/Connive.ron",
            r#"KeywordAction(name:"Connive",spelling:"connive",grammar:Verb(bare:"connive",frame_set:Intransitive))"#,
        ),
    ])
    .unwrap();
    let wrong_id = DeclarationId::new(DeclarationKind::KeywordAction, "Destroy");
    assert!(
        DeclarationTransitiveVerb::new(&wrong_recipe, VerbInventoryRef::Declaration(wrong_id))
            .is_none()
    );
    Parser::new(wrong_recipe).expect("wrong recipes do not become fixed parser requirements");

    let missing_feature = environment_from([
        declaration(
            "/synthetic/actions/Destroy.ron",
            r#"KeywordAction(name:"Destroy",spelling:"destroy",grammar:Verb(bare:"destroy",third_person:Unavailable,frame_set:Transitive))"#,
        ),
        declaration(
            "/synthetic/actions/Connive.ron",
            r#"KeywordAction(name:"Connive",spelling:"connive",grammar:Verb(bare:"connive",frame_set:Intransitive))"#,
        ),
    ])
    .unwrap();
    let missing_id = DeclarationId::new(DeclarationKind::KeywordAction, "Destroy");
    assert!(
        DeclarationTransitiveVerb::new(
            &missing_feature,
            VerbInventoryRef::Declaration(missing_id),
        )
        .is_none()
    );
    Parser::new(missing_feature)
        .expect("missing concord_class surfaces do not become fixed parser requirements");
}

#[test]
fn open_declaration_synthetic_verbs_parse_and_render_both_concord_classes_exactly() {
    let parser = parser();
    let context = context();
    for text in [
        "Frindle target player.",
        "That player frondles target player.",
        "Zorble.",
        "It zurbles.",
    ] {
        let ability = parser.parse(text, &context).unwrap_or_else(|error| {
            panic!("synthetic declaration surface must parse `{text}`: {error}")
        });
        assert_eq!(ability.render(&context, parser.environment()), text);
    }

    for text in ["It frindle target player.", "That player zorble."] {
        assert!(
            parser.parse(text, &context).is_err(),
            "wrong concord_class parsed: {text}"
        );
    }
}

#[test]
fn declared_preterites_parse_and_render_through_both_concord_classes() {
    let parser = parser();
    let context = context();
    for text in [
        "You frindled target player.",
        "That player frindled target player.",
        "It zorbled.",
        "They zorbled.",
    ] {
        let ability = parser
            .parse(text, &context)
            .unwrap_or_else(|error| panic!("declared preterite must parse `{text}`: {error}"));
        assert_eq!(ability.render(&context, parser.environment()), text);
    }
}

#[test]
fn category_homonyms_remain_distinct_declaration_identities() {
    let mut rows = synthetic_verb_rows();
    rows.push(declaration(
        "/synthetic/abilities/Destroy.ron",
        r#"KeywordAbility(name:"Destroy",spelling:"frindle",grammar:Verb(bare:"frindle",third_person:"frondles",frame_set:Transitive))"#,
    ));
    let environment = environment_from(rows).unwrap();
    assert_eq!(
        environment.readings(GrammarPosition::Verb, "frindle").len(),
        2,
        "the environment must retain both category-safe identities"
    );
    let parser = Parser::new(environment).unwrap();
    assert!(
        parser.parse("Frindle target player.", &context()).is_err(),
        "equal-surface, equal-frame declarations in distinct categories remain ambiguous"
    );

    let ability_only = environment_from([declaration(
        "/synthetic/abilities/Destroy.ron",
        r#"KeywordAbility(name:"Destroy",spelling:"frindle",grammar:Verb(bare:"frindle",third_person:"frondles",frame_set:Transitive))"#,
    )])
    .unwrap();
    let parser = Parser::new(ability_only).expect("declaration categories share the open frame");
    let ability = parser
        .parse("Frindle target player.", &context())
        .expect("a non-action verb declaration reaches the shared frame");
    let mut visitor = IdentityVisitor::default();
    visitor.visit_ability(&ability);
    assert_eq!(
        visitor.events.first().map(String::as_str),
        Some("declaration:KeywordAbility/Destroy")
    );
}

#[derive(Default)]
struct IdentityVisitor {
    events: Vec<String>,
}

impl Visitor for IdentityVisitor {
    fn visit_declaration(&mut self, id: &DeclarationId) {
        self.events
            .push(format!("declaration:{:?}/{}", id.kind(), id.name()));
    }

    fn visit_common_noun(&mut self, noun: CommonNoun) {
        self.events.push(format!("noun:{noun:?}"));
    }
}

#[test]
fn visitor_observes_owned_declaration_identity_in_form_order() {
    let parser = parser();
    let ability = parser
        .parse("Frindle target player.", &context())
        .expect("open declaration form parses");
    let mut visitor = IdentityVisitor::default();
    visitor.visit_ability(&ability);
    assert_eq!(
        visitor.events,
        ["declaration:KeywordAction/Destroy", "noun:Player"]
    );

    let mut explicit = IdentityVisitor::default();
    walk_ability(&mut explicit, &ability);
    assert_eq!(explicit.events, visitor.events);
}
