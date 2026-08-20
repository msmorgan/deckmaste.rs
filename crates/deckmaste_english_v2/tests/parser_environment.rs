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
            "/synthetic/actions/Quuxify.ron",
            r#"KeywordAction(name:"Quuxify",spelling:"quuxify",grammar:Verb(bare:"quuxify",third_person:"quuxifies",valence:Numerative))"#,
        ),
        declaration(
            "/synthetic/abilities/Zorblance.ron",
            r#"KeywordAbility(name:"Zorblance",spelling:"zorblance",grammar:FixedTerm(surface:"zorblance"))"#,
        ),
        declaration(
            "/synthetic/designations/Zorblance.ron",
            r#"Designation(name:"Zorblance",spelling:"zorblance",grammar:FixedTerm(surface:"zorblance"))"#,
        ),
        declaration(
            "/synthetic/types/Glimmerhedron.ron",
            r#"Type(name:"Glimmerhedron",spelling:"glimmerhedron",grammar:Noun(singular:"glimmerhedron"))"#,
        ),
        declaration(
            "/synthetic/subtypes/creature/Nivellin.ron",
            r#"Subtype(category:Creature,name:"Nivellin",spelling:"nivellin",grammar:Noun(singular:"nivellin"))"#,
        ),
        declaration(
            "/synthetic/counters/Fluxion.ron",
            r#"CounterKind(name:"Fluxion",spelling:"fluxion",grammar:FixedTerm(surface:"fluxion"))"#,
        ),
        declaration(
            "/synthetic/abilities/Quorbling.ron",
            r#"KeywordAbility(name:"Quorbling",spelling:"quorbling")"#,
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

    let quuxify = environment
        .declaration(DeclarationKind::KeywordAction, "Quuxify")
        .expect("action identity is indexed");
    assert_eq!(quuxify.id().kind(), DeclarationKind::KeywordAction);
    assert_eq!(quuxify.id().name(), "Quuxify");
    assert!(matches!(quuxify.recipe(), Some(GrammarRecipe::Verb { .. })));
    assert_eq!(quuxify.valence(), Some(&VerbValence::Numerative));
    assert_eq!(
        quuxify.provenance(),
        Path::new("/synthetic/actions/Quuxify.ron")
    );
    assert_eq!(
        environment.surface(quuxify.id(), SurfaceFeature::Bare),
        Some("quuxify")
    );
    assert_eq!(
        environment.surface(quuxify.id(), SurfaceFeature::ThirdPersonSingular),
        Some("quuxifies")
    );
    assert_eq!(
        reading_projection(&environment, GrammarPosition::Verb, "quuxifies"),
        [(
            DeclarationKind::KeywordAction,
            "Quuxify".to_owned(),
            SurfaceFeature::ThirdPersonSingular,
            "quuxifies".to_owned(),
        )]
    );

    for (kind, name) in [
        (DeclarationKind::KeywordAbility, "Zorblance"),
        (DeclarationKind::Designation, "Zorblance"),
        (DeclarationKind::Type, "Glimmerhedron"),
    ] {
        assert!(
            environment.declaration(kind, name).is_some(),
            "missing {kind:?} {name}"
        );
    }

    for (kind, name, position, feature, surface) in [
        (
            DeclarationKind::Subtype(SubtypeCategory::Creature),
            "Nivellin",
            GrammarPosition::Noun,
            SurfaceFeature::Singular,
            "nivellin",
        ),
        (
            DeclarationKind::Subtype(SubtypeCategory::Creature),
            "Nivellin",
            GrammarPosition::Noun,
            SurfaceFeature::Plural,
            "nivellins",
        ),
        (
            DeclarationKind::CounterKind,
            "Fluxion",
            GrammarPosition::FixedTerm,
            SurfaceFeature::Fixed,
            "fluxion",
        ),
    ] {
        let declaration = environment
            .declaration(kind, name)
            .expect("synthetic declaration is indexed");
        assert_eq!(
            environment.surface(declaration.id(), feature),
            Some(surface)
        );
        assert_eq!(
            reading_projection(&environment, position, surface),
            [(kind, name.to_owned(), feature, surface.to_owned())],
        );
    }

    let quorbling = environment
        .declaration(DeclarationKind::KeywordAbility, "Quorbling")
        .expect("grammar-free declaration remains addressable");
    assert_eq!(quorbling.recipe(), None);
    assert!(
        environment
            .readings(GrammarPosition::FixedKeyword, "quorbling")
            .is_empty()
    );
}

#[test]
fn parser_environment_preserves_collisions_and_category_isolation() {
    let environment = ParserEnvironment::try_from_declarations(synthetic_declarations())
        .expect("synthetic declarations compile");

    let ability = environment
        .declaration(DeclarationKind::KeywordAbility, "Zorblance")
        .expect("ability homonym");
    let designation = environment
        .declaration(DeclarationKind::Designation, "Zorblance")
        .expect("designation homonym");
    assert_ne!(ability.id(), designation.id());

    assert_eq!(
        reading_projection(&environment, GrammarPosition::FixedTerm, "zorblance"),
        [
            (
                DeclarationKind::KeywordAbility,
                "Zorblance".to_owned(),
                SurfaceFeature::Fixed,
                "zorblance".to_owned(),
            ),
            (
                DeclarationKind::Designation,
                "Zorblance".to_owned(),
                SurfaceFeature::Fixed,
                "zorblance".to_owned(),
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
        (GrammarPosition::Verb, "quuxify"),
        (GrammarPosition::Noun, "glimmerhedrons"),
        (GrammarPosition::FixedTerm, "zorblance"),
    ] {
        assert_eq!(
            reading_projection(&forward, position, surface),
            reading_projection(&reverse, position, surface)
        );
    }

    let owned_name = {
        let source = String::from(
            r#"CounterKind(name:"Motespan",spelling:"motespan",grammar:FixedTerm(surface:"motespan"))"#,
        );
        let environment = ParserEnvironment::try_from_declarations([declaration(
            "/temporary/Motespan.ron",
            &source,
        )])
        .expect("temporary declaration compiles");
        environment
            .declaration(DeclarationKind::CounterKind, "Motespan")
            .expect("temporary identity")
            .id()
            .clone()
    };
    assert_eq!(owned_name.name(), "Motespan");
}

#[test]
fn parser_environment_rejects_duplicate_category_safe_identity() {
    let first = declaration(
        "/synthetic/first/Quuxify.ron",
        r#"KeywordAction(name:"Quuxify",spelling:"quuxify",grammar:Verb(bare:"quuxify",valence:Numerative))"#,
    );
    let duplicate = declaration(
        "/synthetic/duplicate/Quuxify.ron",
        r#"KeywordAction(name:"Quuxify",spelling:"quuxify",grammar:Verb(bare:"quuxify",third_person:"quuxifies",valence:Numerative))"#,
    );

    let error = ParserEnvironment::try_from_declarations([first, duplicate])
        .expect_err("duplicate identity must fail");
    assert!(matches!(
        error,
        ParserEnvironmentError::DuplicateIdentity {
            identity,
            first_path,
            duplicate_path,
        } if identity == DeclarationId::new(DeclarationKind::KeywordAction, "Quuxify")
            && first_path == Path::new("/synthetic/first/Quuxify.ron")
            && duplicate_path == Path::new("/synthetic/duplicate/Quuxify.ron")
    ));
}

#[test]
fn parser_constructor_owns_and_clones_one_immutable_environment() {
    let declarations = macro_ron::v2::read_builtin_v2(
        Path::new(env!("CARGO_MANIFEST_DIR")).join("../../plugins/builtin_v2"),
    )
    .expect("integrated builtin-v2 declarations load");
    let environment = ParserEnvironment::try_from_declarations(declarations)
        .expect("builtin-v2 declarations compile");
    let parser = Parser::new(environment).expect("required declarations are present");
    let cloned = parser.clone();
    let context = ParseContext::new("Context Card").expect("nonempty context");

    assert_eq!(
        parser.parse("You gain 3 life.", &context),
        cloned.parse("You gain 3 life.", &context)
    );
}
