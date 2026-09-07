use std::path::Path;

use deckmaste_construction_core::macro_def::NormalizedDeclaration;
use deckmaste_construction_core::macro_def::Onset;
use deckmaste_english_v2::ast::CatalogProvider;
use deckmaste_english_v2::context::ParseContext;
use deckmaste_english_v2::environment::CatalogProviderRow;
use deckmaste_english_v2::environment::CatalogProviderRows;
use deckmaste_english_v2::environment::ParserEnvironment;
use deckmaste_english_v2::parser::Parser;
use deckmaste_english_v2::parser::TraceLimits;
use deckmaste_english_v2::render::Render;

fn declaration(path: &str, source: &str) -> NormalizedDeclaration {
    deckmaste_construction_core::macro_def::read_str(path, source)
        .expect("synthetic declaration is valid")
}

fn parser_with(extra: impl IntoIterator<Item = NormalizedDeclaration>) -> Parser {
    let mut declarations = deckmaste_construction_core::macro_def::read_builtin_v2(
        Path::new(env!("CARGO_MANIFEST_DIR")).join("../../plugins_v2/builtin"),
    )
    .expect("builtin-v2 declarations load");
    declarations.extend(extra);
    let environment = ParserEnvironment::try_from_parts(
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
    .expect("declarations compile into one environment");
    Parser::new(environment).expect("required static declarations are present")
}

#[test]
fn declaration_noun_admits_dynamic_type_and_subtype_rows_without_catalogs() {
    let parser = parser_with([
        declaration(
            "/synthetic/types/Relic.ron",
            r#"Type(name:"Relic",spelling:"relic",grammar:Noun(singular:"relic"))"#,
        ),
        declaration(
            "/synthetic/subtypes/creature/Wug.ron",
            r#"Subtype(category:Creature,name:"Wug",spelling:"Wug",grammar:Noun(singular:"Wug"))"#,
        ),
    ]);
    let context = ParseContext::new(
        "Context Card",
        false,
        deckmaste_construction_core::macro_def::Onset::Consonant,
    )
    .expect("valid context");
    for text in [
        "Destroy target relic.",
        "Destroy target Wug.",
        "Those relics deal 3 damage to it.",
        "Those Wugs deal 3 damage to it.",
    ] {
        let parsed = parser.parse(text, &context).unwrap_or_else(|error| {
            panic!("dynamic declaration noun did not parse {text:?}: {error}")
        });
        assert_eq!(parsed.render(&context, parser.environment()), text);
    }

    let trace = parser.trace(
        "Destroy target relic.",
        &context,
        TraceLimits::new(usize::MAX),
    );
    assert!(
        trace.scanner_matches().items().iter().any(|scanner_match| {
            scanner_match.start() == 14
                && scanner_match.end() == 20
                && scanner_match
                    .value_label_v1()
                    .contains("DeclarationIdentity { kind: Type, name: \"Relic\" }")
        }),
        "{:#?}",
        trace.scanner_matches().items()
    );
}

#[test]
fn rend_spirit_is_the_reviewed_declaration_noun_corpus_delta() {
    let parser = parser_with([]);
    let context = ParseContext::new(
        "Rend Spirit",
        false,
        deckmaste_construction_core::macro_def::Onset::Consonant,
    )
    .expect("valid card context");
    let text = "Destroy target Spirit.";

    let parsed = parser.parse(text, &context).expect("Rend Spirit parses");
    assert_eq!(parsed.render(&context, parser.environment()), text);

    let trace = parser.trace(text, &context, TraceLimits::new(usize::MAX));
    assert!(trace.scanner_matches().items().iter().any(|scanner_match| {
        scanner_match.start() == 0
            && scanner_match.end() == 7
            && scanner_match
                .value_label_v1()
                .contains("DeclarationIdentity { kind: KeywordAction, name: \"Destroy\" }")
    }));
    assert!(trace.scanner_matches().items().iter().any(|scanner_match| {
        scanner_match.start() == 14
            && scanner_match.end() == 21
            && scanner_match
                .value_label_v1()
                .contains("DeclarationIdentity { kind: Subtype(Creature), name: \"Spirit\" }")
    }));
}
