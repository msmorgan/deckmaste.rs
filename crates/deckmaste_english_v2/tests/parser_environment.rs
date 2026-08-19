use std::path::Path;

use deckmaste_english_v2::context::ParseContext;
use deckmaste_english_v2::environment::DeclarationId;
use deckmaste_english_v2::environment::GrammarPosition;
use deckmaste_english_v2::environment::ParserEnvironment;
use deckmaste_english_v2::environment::ParserEnvironmentError;
use deckmaste_english_v2::parser::Parser;
use macro_ron::v2::DeclarationKind;
use macro_ron::v2::GrammarRecipe;
use macro_ron::v2::NormalizedDeclaration;
use macro_ron::v2::SubtypeCategory;
use macro_ron::v2::SurfaceFeature;
use macro_ron::v2::VerbValence;
use macro_ron::v2::read_str;

fn declaration(path: &str, source: &str) -> NormalizedDeclaration {
    read_str(path, source).expect("synthetic v2 declaration is valid")
}

fn synthetic_declarations() -> Vec<NormalizedDeclaration> {
    vec![
        declaration(
            "/synthetic/actions/Scry.ron",
            r#"KeywordAction(name:"Scry",spelling:"scry",grammar:Verb(bare:"scry",third_person:"scries",valence:Numerative))"#,
        ),
        declaration(
            "/synthetic/abilities/Echo.ron",
            r#"KeywordAbility(name:"Echo",spelling:"echo",grammar:FixedTerm(surface:"echo"))"#,
        ),
        declaration(
            "/synthetic/designations/Echo.ron",
            r#"Designation(name:"Echo",spelling:"echo",grammar:FixedTerm(surface:"echo"))"#,
        ),
        declaration(
            "/synthetic/types/Relic.ron",
            r#"Type(name:"Relic",spelling:"relic",grammar:Noun(singular:"relic"))"#,
        ),
        declaration(
            "/synthetic/subtypes/creature/Sprite.ron",
            r#"Subtype(category:Creature,name:"Sprite",spelling:"sprite",grammar:Noun(singular:"sprite"))"#,
        ),
        declaration(
            "/synthetic/counters/Charge.ron",
            r#"CounterKind(name:"Charge",spelling:"charge",grammar:FixedTerm(surface:"charge"))"#,
        ),
        declaration(
            "/synthetic/abilities/Ward.ron",
            r#"KeywordAbility(name:"Ward",spelling:"ward")"#,
        ),
    ]
}

fn reading_projection(
    environment: &ParserEnvironment,
    position: GrammarPosition,
    surface: &str,
) -> Vec<(DeclarationKind, String, SurfaceFeature, String)> {
    environment
        .readings(position, surface)
        .iter()
        .map(|reading| {
            (
                reading.id().kind(),
                reading.id().name().to_owned(),
                reading.feature(),
                reading.surface().to_owned(),
            )
        })
        .collect()
}

#[test]
fn parser_environment_indexes_open_categories_in_both_directions() {
    let environment = ParserEnvironment::try_from_declarations(synthetic_declarations())
        .expect("synthetic declarations compile");

    let scry = environment
        .declaration(DeclarationKind::KeywordAction, "Scry")
        .expect("action identity is indexed");
    assert_eq!(scry.id().kind(), DeclarationKind::KeywordAction);
    assert_eq!(scry.id().name(), "Scry");
    assert!(matches!(scry.recipe(), Some(GrammarRecipe::Verb { .. })));
    assert_eq!(scry.valence(), Some(&VerbValence::Numerative));
    assert_eq!(scry.provenance(), Path::new("/synthetic/actions/Scry.ron"));
    assert_eq!(
        environment.surface(scry.id(), SurfaceFeature::Bare),
        Some("scry")
    );
    assert_eq!(
        environment.surface(scry.id(), SurfaceFeature::ThirdPersonSingular),
        Some("scries")
    );
    assert_eq!(
        reading_projection(&environment, GrammarPosition::Verb, "scries"),
        [(
            DeclarationKind::KeywordAction,
            "Scry".to_owned(),
            SurfaceFeature::ThirdPersonSingular,
            "scries".to_owned(),
        )]
    );

    for (kind, name) in [
        (DeclarationKind::KeywordAbility, "Echo"),
        (DeclarationKind::Designation, "Echo"),
        (DeclarationKind::Type, "Relic"),
        (
            DeclarationKind::Subtype(SubtypeCategory::Creature),
            "Sprite",
        ),
        (DeclarationKind::CounterKind, "Charge"),
    ] {
        assert!(
            environment.declaration(kind, name).is_some(),
            "missing {kind:?} {name}"
        );
    }

    let ward = environment
        .declaration(DeclarationKind::KeywordAbility, "Ward")
        .expect("grammar-free declaration remains addressable");
    assert_eq!(ward.recipe(), None);
    assert!(
        environment
            .readings(GrammarPosition::FixedKeyword, "ward")
            .is_empty()
    );
}

#[test]
fn parser_environment_preserves_collisions_and_category_isolation() {
    let environment = ParserEnvironment::try_from_declarations(synthetic_declarations())
        .expect("synthetic declarations compile");

    let ability = environment
        .declaration(DeclarationKind::KeywordAbility, "Echo")
        .expect("ability homonym");
    let designation = environment
        .declaration(DeclarationKind::Designation, "Echo")
        .expect("designation homonym");
    assert_ne!(ability.id(), designation.id());

    assert_eq!(
        reading_projection(&environment, GrammarPosition::FixedTerm, "echo"),
        [
            (
                DeclarationKind::KeywordAbility,
                "Echo".to_owned(),
                SurfaceFeature::Fixed,
                "echo".to_owned(),
            ),
            (
                DeclarationKind::Designation,
                "Echo".to_owned(),
                SurfaceFeature::Fixed,
                "echo".to_owned(),
            ),
        ]
    );
}

#[test]
fn parser_environment_is_deterministic_and_owns_source_data() {
    let declarations = synthetic_declarations();
    let forward = ParserEnvironment::try_from_declarations(declarations.clone())
        .expect("forward environment");
    let reverse = ParserEnvironment::try_from_declarations(declarations.into_iter().rev())
        .expect("reverse environment");

    for (position, surface) in [
        (GrammarPosition::Verb, "scry"),
        (GrammarPosition::Noun, "relics"),
        (GrammarPosition::FixedTerm, "echo"),
    ] {
        assert_eq!(
            reading_projection(&forward, position, surface),
            reading_projection(&reverse, position, surface)
        );
    }

    let owned_name = {
        let source = String::from(
            r#"CounterKind(name:"Momentum",spelling:"momentum",grammar:FixedTerm(surface:"momentum"))"#,
        );
        let environment = ParserEnvironment::try_from_declarations([declaration(
            "/temporary/Momentum.ron",
            &source,
        )])
        .expect("temporary declaration compiles");
        environment
            .declaration(DeclarationKind::CounterKind, "Momentum")
            .expect("temporary identity")
            .id()
            .clone()
    };
    assert_eq!(owned_name.name(), "Momentum");
}

#[test]
fn parser_environment_rejects_duplicate_category_safe_identity() {
    let first = declaration(
        "/synthetic/first/Scry.ron",
        r#"KeywordAction(name:"Scry",spelling:"scry",grammar:Verb(bare:"scry",valence:Numerative))"#,
    );
    let duplicate = declaration(
        "/synthetic/duplicate/Scry.ron",
        r#"KeywordAction(name:"Scry",spelling:"scry",grammar:Verb(bare:"scry",third_person:"scries",valence:Numerative))"#,
    );

    let error = ParserEnvironment::try_from_declarations([first, duplicate])
        .expect_err("duplicate identity must fail");
    assert!(matches!(
        error,
        ParserEnvironmentError::DuplicateIdentity {
            identity,
            first_path,
            duplicate_path,
        } if identity == DeclarationId::new(DeclarationKind::KeywordAction, "Scry")
            && first_path == Path::new("/synthetic/first/Scry.ron")
            && duplicate_path == Path::new("/synthetic/duplicate/Scry.ron")
    ));
}

#[test]
fn parser_constructor_owns_and_clones_one_immutable_environment() {
    let environment = ParserEnvironment::try_from_declarations(synthetic_declarations())
        .expect("synthetic declarations compile");
    let parser = Parser::new(environment);
    let cloned = parser.clone();
    let context = ParseContext::new("Context Card").expect("nonempty context");

    assert_eq!(
        parser.parse("You gain 3 life.", &context),
        cloned.parse("You gain 3 life.", &context)
    );
}
