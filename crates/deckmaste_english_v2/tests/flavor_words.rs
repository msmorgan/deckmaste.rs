use std::path::Path;

use deckmaste_construction_core::macro_def::DeclarationIdentity;
use deckmaste_construction_core::macro_def::DeclarationKind;
use deckmaste_construction_core::macro_def::Onset;
use deckmaste_construction_core::macro_def::read_builtin_v2;
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

fn environment() -> ParserEnvironment {
    ParserEnvironment::try_from_parts(
        read_builtin_v2(Path::new(env!("CARGO_MANIFEST_DIR")).join("../../plugins_v2/builtin"))
            .expect("builtin-v2 declarations load"),
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

fn context() -> ParseContext<'static> {
    ParseContext::new("Context Card", false, Onset::Consonant).expect("context is valid")
}

fn assert_exact_document(parser: &Parser, text: &str) -> OracleText {
    let context = context();
    let parsed = parser
        .parse_oracle_text(text, &context)
        .unwrap_or_else(|error| panic!("{text:?} parses: {error:?}"));
    assert_eq!(parsed.render(&context, parser.environment()), text);

    let analysis: ParseAnalysis<OracleText> = parser.analyze_oracle_text(text, &context);
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

#[derive(Default)]
struct DeclarationVisitor(Vec<(DeclarationKind, String)>);

impl Visitor for DeclarationVisitor {
    fn visit_declaration(&mut self, declaration: &DeclarationIdentity) {
        self.0
            .push((declaration.kind(), declaration.name().to_owned()));
    }
}

#[test]
fn flavor_words_cover_plain_chapter_and_mode_label_positions() {
    let parser = Parser::new(environment()).expect("flavor-word grammar initializes");
    let witnesses = [
        "Into the TARDIS — Whenever this creature attacks, draw a card.",
        "II, III — Brimstone — Add {R}{R}{R}{R}.",
        "Choose one —\n• Khans — Draw a card.\n• Dragons — You gain 1 life.",
    ];

    let parsed = witnesses
        .iter()
        .map(|text| assert_exact_document(&parser, text))
        .collect::<Vec<_>>();
    let mut visitor = DeclarationVisitor::default();
    for document in &parsed {
        visitor.visit_oracle_text(document);
    }
    for name in ["intoTheTARDIS", "brimstone", "khans", "dragons"] {
        assert!(
            visitor
                .0
                .contains(&(DeclarationKind::FlavorWord, name.to_owned())),
            "{name} is visited as a flavor-word declaration: {:?}",
            visitor.0,
        );
    }
}

#[test]
fn flavor_word_class_is_open_but_declaration_backed() {
    let parser = Parser::new(environment()).expect("flavor-word grammar initializes");
    assert!(
        parser
            .parse_oracle_text("Undeclared Signal — Draw a card.", &context())
            .is_err(),
        "an undeclared label remains an ordinary parse failure",
    );
}
