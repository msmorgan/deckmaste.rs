fn main() {
    let catalogs = deckmaste_english::Catalogs::default();
    let report = deckmaste_english::parse_fragment(
        "Draw a card.",
        &catalogs,
        deckmaste_english::FragmentKind::Ability,
        "Test Card",
        false,
    );
    let deckmaste_english::Fragment::Ability(mut ability) = report.into_fragment().unwrap() else {
        unreachable!()
    };
    ability.kind = deckmaste_english::syntax::AbilityKind::Paragraph(
        deckmaste_english::syntax::Paragraph::default(),
    );
}
