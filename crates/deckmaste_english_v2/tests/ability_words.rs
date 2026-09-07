use std::path::Path;

use deckmaste_construction_core::macro_def::DeclarationIdentity;
use deckmaste_construction_core::macro_def::DeclarationKind;
use deckmaste_construction_core::macro_def::Onset;
use deckmaste_construction_core::macro_def::read_builtin_v2;
use deckmaste_construction_core::macro_def::read_str;
use deckmaste_english_v2::ast::CatalogProvider;
use deckmaste_english_v2::ast::OracleText;
use deckmaste_english_v2::context::ParseContext;
use deckmaste_english_v2::environment::CatalogProviderRow;
use deckmaste_english_v2::environment::CatalogProviderRows;
use deckmaste_english_v2::environment::ParserEnvironment;
use deckmaste_english_v2::parser::ParseAnalysis;
use deckmaste_english_v2::parser::Parser;
use deckmaste_english_v2::render::Render;
use deckmaste_english_v2::visit::Visitor;

fn declarations() -> Vec<deckmaste_construction_core::macro_def::NormalizedDeclaration> {
    read_builtin_v2(Path::new(env!("CARGO_MANIFEST_DIR")).join("../../plugins_v2/builtin"))
        .expect("builtin-v2 declarations load")
}

fn context(name: &'static str) -> ParseContext<'static> {
    ParseContext::new(name, false, Onset::Consonant).expect("context is valid")
}

fn environment(
    declarations: Vec<deckmaste_construction_core::macro_def::NormalizedDeclaration>,
) -> ParserEnvironment {
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
    .expect("declaration environment freezes")
}

#[derive(Default)]
struct DeclarationVisitor(Vec<(DeclarationKind, String)>);

impl Visitor for DeclarationVisitor {
    fn visit_declaration(&mut self, declaration: &DeclarationIdentity) {
        self.0
            .push((declaration.kind(), declaration.name().to_owned()));
    }
}

fn assert_exact_document(
    parser: &Parser,
    environment: &ParserEnvironment,
    text: &str,
    context: &ParseContext<'_>,
) -> OracleText {
    let parsed = parser
        .parse_oracle_text(text, context)
        .unwrap_or_else(|error| panic!("{text:?} parses: {error:?}"));
    assert_eq!(parsed.render(context, environment), text);

    let analysis: ParseAnalysis<OracleText> = parser.analyze_oracle_text(text, context);
    assert_eq!(
        analysis.selected(),
        Some(&parsed),
        "{text:?} selects exactly"
    );
    let ownership = analysis
        .ownership()
        .unwrap_or_else(|| panic!("{text:?} has selected ownership"));
    assert!(ownership.failures().is_empty(), "{text:?}: {ownership:?}");
    assert!(ownership.summary().covered(), "{text:?}: {ownership:?}");
    parsed
}

#[test]
fn attested_ability_word_lead_in_parses_renders_visits_and_owns_exactly() {
    let environment = environment(declarations());
    let parser = Parser::new(environment.clone()).expect("ability-word grammar initializes");
    let text = "Landfall — Whenever a land you control enters, you gain 1 life.";
    let context = context("Druid Class");
    let parsed = assert_exact_document(&parser, &environment, text, &context);

    let mut visitor = DeclarationVisitor::default();
    visitor.visit_oracle_text(&parsed);
    assert!(
        visitor
            .0
            .contains(&(DeclarationKind::AbilityWord, "Landfall".into()))
    );
    assert!(visitor.0.contains(&(DeclarationKind::Type, "Land".into())));
}

#[test]
fn ability_word_vocabulary_is_open_but_declaration_backed() {
    let mut rows = declarations();
    rows.push(
        read_str(
            "/synthetic/ability_words/Quorblefall.ron",
            r#"AbilityWord(name:"Quorblefall",spelling:"Quorblefall",grammar:FixedTerm(surface:"Quorblefall"))"#,
        )
        .expect("synthetic same-plugin ability word is valid"),
    );
    let environment = environment(rows);
    let parser = Parser::new(environment.clone()).expect("ability-word grammar initializes");
    let context = context("Context Card");

    assert_exact_document(
        &parser,
        &environment,
        "Quorblefall — Draw a card.",
        &context,
    );
    assert!(
        parser
            .parse_oracle_text("Undeclaredfall — Draw a card.", &context)
            .is_err(),
        "an undeclared label remains an ordinary silent parse failure",
    );
}
