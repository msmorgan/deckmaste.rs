use std::path::Path;

use deckmaste_construction_core::macro_def::Onset;
use deckmaste_english_v2::ast::CatalogProvider;
use deckmaste_english_v2::context::ParseContext;
use deckmaste_english_v2::environment::CatalogProviderRow;
use deckmaste_english_v2::environment::CatalogProviderRows;
use deckmaste_english_v2::environment::ParserEnvironment;
use deckmaste_english_v2::parser::Parser;

fn parser() -> Parser {
    let declarations = deckmaste_construction_core::macro_def::read_builtin_v2(
        Path::new(env!("CARGO_MANIFEST_DIR")).join("../../plugins_v2/builtin"),
    )
    .expect("builtin declarations load");
    let cards = CatalogProviderRows::new(
        CatalogProvider::CardNames,
        [CatalogProviderRow::new(
            "context-card",
            "Context Card",
            Onset::Consonant,
        )],
    );
    let environment = ParserEnvironment::try_from_parts(declarations, [cards])
        .expect("parser environment freezes");
    Parser::new(environment).expect("required declarations are present")
}

fn context() -> ParseContext<'static> {
    ParseContext::new("Context Card", false, Onset::Consonant).expect("context is valid")
}

#[test]
fn zeroable_determiner_branches_have_no_synthetic_ownership_claims() {
    let parser = parser();
    let context = context();
    for text in [
        "Destroy creatures.",
        "Destroy target creature.",
        "Destroy an artifact.",
    ] {
        let analysis = parser.analyze(text, &context);
        assert!(
            analysis.selected().is_some(),
            "{text:?} selects a zeroable determiner branch: {analysis:?}"
        );
        let ownership = analysis
            .ownership()
            .expect("a selected reference document owns its bytes");
        assert!(ownership.failures().is_empty(), "{text:?}: {ownership:?}");
        let summary = ownership.summary();
        assert!(summary.covered(), "{text:?}: {ownership:?}");
        assert_eq!(summary.synthetic_claims(), 0, "{text:?}: {ownership:?}");
    }
}
