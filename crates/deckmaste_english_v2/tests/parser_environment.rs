use std::path::Path;
use std::sync::Arc;

use deckmaste_construction_core::macro_def::DeclarationKind;
use deckmaste_construction_core::macro_def::GrammarRecipe;
use deckmaste_construction_core::macro_def::NormalizedDeclaration;
use deckmaste_construction_core::macro_def::Onset;
use deckmaste_construction_core::macro_def::SubtypeCategory;
use deckmaste_construction_core::macro_def::SurfaceFeature;
use deckmaste_construction_core::macro_def::VerbFrameSet;
use deckmaste_construction_core::macro_def::read_str;
use deckmaste_english_v2::ast::CatalogProvider;
use deckmaste_english_v2::context::ParseContext;
use deckmaste_english_v2::environment::CatalogProviderRow;
use deckmaste_english_v2::environment::CatalogProviderRows;
use deckmaste_english_v2::environment::DeclarationId;
use deckmaste_english_v2::environment::GrammarPosition;
use deckmaste_english_v2::environment::ParserEnvironment;
use deckmaste_english_v2::environment::ParserEnvironmentError;
use deckmaste_english_v2::parser::Parser;
use deckmaste_english_v2::parser::ParserBuildError;

fn declaration(path: &str, source: &str) -> NormalizedDeclaration {
    read_str(path, source).expect("synthetic v2 declaration is valid")
}

fn synthetic_declarations() -> Vec<NormalizedDeclaration> {
    vec![
        declaration(
            "/synthetic/actions/Quuxify.ron",
            r#"KeywordAction(name:"Quuxify",spelling:"quuxify",grammar:Verb(bare:"quuxify",third_person:"quuxifies",frame_set:MeasureComplement))"#,
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

fn card_name_provider(rows: impl IntoIterator<Item = CatalogProviderRow>) -> CatalogProviderRows {
    CatalogProviderRows::new(CatalogProvider::CardNames, rows)
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
    assert_eq!(quuxify.frame_set(), Some(&VerbFrameSet::MeasureComplement));
    assert_eq!(
        quuxify.provenance(),
        Path::new("/synthetic/actions/Quuxify.ron")
    );
    assert_eq!(
        environment.surface(quuxify.id(), SurfaceFeature::PLAIN),
        Some("quuxify")
    );
    assert_eq!(
        environment.surface(quuxify.id(), SurfaceFeature::THIRD_PERSON_SINGULAR_PRESENT),
        Some("quuxifies")
    );
    assert_eq!(
        reading_projection(&environment, GrammarPosition::Verb, "quuxifies"),
        [(
            DeclarationKind::KeywordAction,
            "Quuxify".to_owned(),
            SurfaceFeature::THIRD_PERSON_SINGULAR_PRESENT,
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
fn parser_environment_freezes_exact_catalog_identity_rows() {
    let provider = card_name_provider([
        CatalogProviderRow::new("alpha", "Alpha", Onset::Vowel),
        CatalogProviderRow::new("alpha-beta", "Alpha Beta", Onset::Vowel),
        CatalogProviderRow::new("urzas-saga", "Urza's Saga", Onset::Vowel),
        CatalogProviderRow::new("seven-dwarves", "Seven Dwarves", Onset::Consonant),
    ]);
    let environment = ParserEnvironment::try_from_parts(synthetic_declarations(), [provider])
        .expect("catalog provider rows freeze");

    assert_eq!(
        environment.catalog_surface(CatalogProvider::CardNames, "seven-dwarves"),
        Some("Seven Dwarves")
    );
    assert_eq!(
        environment.catalog_onset(CatalogProvider::CardNames, "seven-dwarves"),
        Some(Onset::Consonant)
    );
    assert_eq!(
        environment
            .catalog_identity(CatalogProvider::CardNames, "seven-dwarves")
            .as_deref(),
        Some("seven-dwarves")
    );
    assert_eq!(
        environment.catalog_identity(CatalogProvider::CardNames, "Seven Dwarves"),
        None,
        "the value constructor accepts canonical identities, not surface spellings"
    );
}

#[test]
fn parser_environment_catalog_rows_are_deterministic_and_owned() {
    let forward_rows = vec![
        CatalogProviderRow::new("second", "Alpha Beta", Onset::Vowel),
        CatalogProviderRow::new("first", "Alpha", Onset::Vowel),
    ];
    let reverse_rows = forward_rows.iter().cloned().rev().collect::<Vec<_>>();
    let forward = ParserEnvironment::try_from_parts(
        synthetic_declarations(),
        [card_name_provider(forward_rows)],
    )
    .expect("forward rows freeze");
    let reverse = ParserEnvironment::try_from_parts(
        synthetic_declarations().into_iter().rev(),
        [card_name_provider(reverse_rows)],
    )
    .expect("reverse rows freeze");

    assert_eq!(forward, reverse);
}

#[test]
fn parser_environment_catalog_rows_outlive_owned_sources() {
    let environment = {
        let identity_string = String::from("scoped-identity");
        let surface_string = String::from("Scoped Surface");
        let identity: Arc<str> = Arc::from(identity_string.as_str());
        let surface: Arc<str> = Arc::from(surface_string.as_str());
        let environment = ParserEnvironment::try_from_parts(
            [],
            [card_name_provider([CatalogProviderRow::new(
                Arc::clone(&identity),
                Arc::clone(&surface),
                Onset::Consonant,
            )])],
        )
        .expect("scoped owned row freezes");
        drop(identity);
        drop(surface);
        drop(identity_string);
        drop(surface_string);
        environment
    };

    assert_eq!(
        environment.catalog_surface(CatalogProvider::CardNames, "scoped-identity"),
        Some("Scoped Surface")
    );
    assert_eq!(
        environment.catalog_onset(CatalogProvider::CardNames, "scoped-identity"),
        Some(Onset::Consonant)
    );
    assert_eq!(
        environment
            .catalog_identity(CatalogProvider::CardNames, "scoped-identity")
            .as_deref(),
        Some("scoped-identity")
    );
}

#[test]
fn parser_environment_rejects_duplicate_catalog_identity_exactly() {
    let error = ParserEnvironment::try_from_parts(
        [],
        [card_name_provider([
            CatalogProviderRow::new("same", "First Surface", Onset::Consonant),
            CatalogProviderRow::new("same", "Second Surface", Onset::Consonant),
        ])],
    )
    .expect_err("one provider cannot repeat a canonical identity");

    assert_eq!(
        error,
        ParserEnvironmentError::DuplicateCatalogIdentity {
            provider: CatalogProvider::CardNames,
            canonical_identity: "same".to_owned(),
        }
    );
}

#[test]
fn parser_environment_rejects_duplicate_catalog_surface_exactly() {
    let error = ParserEnvironment::try_from_parts(
        [],
        [card_name_provider([
            CatalogProviderRow::new("first", "Same Surface", Onset::Consonant),
            CatalogProviderRow::new("second", "Same Surface", Onset::Vowel),
        ])],
    )
    .expect_err("one provider cannot repeat an exact canonical surface");

    assert_eq!(
        error,
        ParserEnvironmentError::DuplicateCatalogSurface {
            provider: CatalogProvider::CardNames,
            canonical_surface: "Same Surface".to_owned(),
        }
    );
}

#[test]
fn parser_environment_rejects_duplicate_catalog_provider() {
    let error = ParserEnvironment::try_from_parts(
        synthetic_declarations(),
        [
            card_name_provider([CatalogProviderRow::new("first", "First", Onset::Consonant)]),
            card_name_provider([CatalogProviderRow::new(
                "second",
                "Second",
                Onset::Consonant,
            )]),
        ],
    )
    .expect_err("one named provider may be supplied only once");

    assert_eq!(
        error,
        ParserEnvironmentError::DuplicateCatalogProvider {
            provider: CatalogProvider::CardNames,
        }
    );
}

#[test]
fn parser_constructor_rejects_missing_generated_catalog_provider() {
    let declarations = deckmaste_construction_core::macro_def::read_builtin_v2(
        Path::new(env!("CARGO_MANIFEST_DIR")).join("../../plugins_v2/builtin"),
    )
    .expect("integrated builtin-v2 declarations load");
    let environment = ParserEnvironment::try_from_declarations(declarations)
        .expect("declarations freeze without implicit provider discovery");

    assert_eq!(
        Parser::new(environment).expect_err("generated provider metadata is fail-closed"),
        ParserBuildError::MissingCatalogProvider {
            provider: CatalogProvider::CardNames,
        }
    );
}

#[test]
fn parser_environment_rejects_duplicate_category_safe_identity() {
    let first = declaration(
        "/synthetic/first/Quuxify.ron",
        r#"KeywordAction(name:"Quuxify",spelling:"quuxify",grammar:Verb(bare:"quuxify",frame_set:MeasureComplement))"#,
    );
    let duplicate = declaration(
        "/synthetic/duplicate/Quuxify.ron",
        r#"KeywordAction(name:"Quuxify",spelling:"quuxify",grammar:Verb(bare:"quuxify",third_person:"quuxifies",frame_set:MeasureComplement))"#,
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
    let declarations = deckmaste_construction_core::macro_def::read_builtin_v2(
        Path::new(env!("CARGO_MANIFEST_DIR")).join("../../plugins_v2/builtin"),
    )
    .expect("integrated builtin-v2 declarations load");
    let environment = ParserEnvironment::try_from_parts(
        declarations,
        [card_name_provider([CatalogProviderRow::new(
            "seven-dwarves",
            "Seven Dwarves",
            Onset::Consonant,
        )])],
    )
    .expect("builtin-v2 declarations and provider compile");
    let parser = Parser::new(environment).expect("required declarations are present");
    let cloned = parser.clone();
    let context = ParseContext::new(
        "Context Card",
        false,
        deckmaste_construction_core::macro_def::Onset::Consonant,
    )
    .expect("nonempty context");

    assert_eq!(
        parser.parse("You gain 3 life.", &context),
        cloned.parse("You gain 3 life.", &context)
    );
}
