use deckmaste_english::Catalogs;
use deckmaste_english::parse_with_catalogs;
use deckmaste_english::syntax::AbilityKind;
use deckmaste_english::syntax::Clause;

#[test]
fn public_parser_returns_a_source_independent_grammar_tree() {
    let source = String::from("Draw a card.");
    let report = parse_with_catalogs(&source, &Catalogs::default());

    assert!(report.diagnostics().is_empty());
    assert!(matches!(
        report.ast().abilities[0].kind,
        AbilityKind::Paragraph(ref paragraph)
            if matches!(paragraph.sentences[0].clause, Clause::Simple(_))
    ));

    let ast = report.into_ast();
    drop(source);

    assert_eq!(ast.render("Test Card", false).unwrap(), "Draw a card.");
}
