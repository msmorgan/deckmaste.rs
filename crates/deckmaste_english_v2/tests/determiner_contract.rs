use std::path::Path;

use deckmaste_construction_core::macro_def::Onset;
use deckmaste_english_v2::ast::CatalogProvider;
use deckmaste_english_v2::ast::Determinative;
use deckmaste_english_v2::ast::DeterminativeHead;
use deckmaste_english_v2::ast::DeterminativeHeadLemma;
use deckmaste_english_v2::ast::DeterminedNominal;
use deckmaste_english_v2::ast::Determiner;
use deckmaste_english_v2::ast::Nominal;
use deckmaste_english_v2::context::ParseContext;
use deckmaste_english_v2::environment::CatalogProviderRow;
use deckmaste_english_v2::environment::CatalogProviderRows;
use deckmaste_english_v2::environment::ParserEnvironment;
use deckmaste_english_v2::parser::Parser;
use deckmaste_english_v2::render::Render;
use deckmaste_english_v2::visit::Visitor;

fn parser() -> Parser {
    let declarations = deckmaste_construction_core::macro_def::read_builtin_v2(
        Path::new(env!("CARGO_MANIFEST_DIR")).join("../../plugins/builtin_v2"),
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
fn one_reference_frame_accepts_zero_simple_and_quantity_determinatives() {
    let parser = parser();
    let context = context();
    for text in [
        "Destroy creatures.",
        "Destroy target creature.",
        "Destroy any target.",
        "Destroy an attacking creature.",
        "Destroy these creatures.",
        "Destroy up to two target creatures.",
        "Destroy one or more target creatures.",
    ] {
        let analysis = parser.analyze(text, &context);
        let parsed = analysis
            .selected()
            .unwrap_or_else(|| panic!("{text:?} must select: {analysis:?}"));
        assert_eq!(parsed.render(&context, parser.environment()), text);
        let ownership = analysis
            .ownership()
            .expect("a selected reference document owns its bytes");
        assert_eq!(ownership.rendered_text(), text);
        assert!(ownership.failures().is_empty(), "{text:?}: {ownership:?}");
        let summary = ownership.summary();
        assert!(summary.covered(), "{text:?}: {ownership:?}");
        assert_eq!(summary.gap_spans(), 0, "{text:?}");
        assert_eq!(summary.overlap_spans(), 0, "{text:?}");
        assert_eq!(summary.synthetic_claims(), 0, "{text:?}");
        assert_eq!(summary.provenance_plan_mismatches(), 0, "{text:?}");
    }
}

#[test]
fn zero_and_quantity_number_are_selected_from_the_complete_determinative() {
    let parser = parser();
    let context = context();
    for text in [
        "Destroy creature.",
        "Destroy these creature.",
        "Destroy a attacking creature.",
        "Destroy an target creature.",
    ] {
        assert!(
            parser.parse(text, &context).is_err(),
            "{text:?} must be rejected"
        );
    }

    // These were rescued by a duration reading of the mis-agreeing object
    // until the fixed duration phrase required a temporal endpoint.
    for rescued in [
        "Destroy each creatures.",
        "Destroy one or more target creature.",
        "Destroy up to one target creatures.",
        "Context Card deals 2 damage to each creatures.",
    ] {
        let analysis = parser.analyze(rescued, &context);
        assert!(
            analysis.selected().is_none(),
            "the invalid object concord class has no duration rescue: {analysis:#?}",
        );
        assert!(
            parser.parse(rescued, &context).is_err(),
            "{rescued:?} must be rejected"
        );
    }
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
enum MassDeterminer {
    Zero,
    All,
}

#[derive(Default)]
struct MassDeterminerVisitor(Vec<MassDeterminer>);

fn is_all_determinative(value: &Determinative) -> bool {
    match value {
        Determinative::SingularSimpleDeterminative(det) => matches!(
            det.head,
            DeterminativeHead::Closed(DeterminativeHeadLemma::All)
        ),
        Determinative::PluralSimpleDeterminative(det) => matches!(
            det.head,
            DeterminativeHead::Closed(DeterminativeHeadLemma::All)
        ),
        _ => false,
    }
}

impl Visitor for MassDeterminerVisitor {
    fn visit_determined_nominal(&mut self, value: &DeterminedNominal) {
        if matches!(value.nominal, Nominal::MassNominal(_)) {
            let determiner = match value.det() {
                Determiner::Zero => MassDeterminer::Zero,
                Determiner::Headed(det) if is_all_determinative(det) => MassDeterminer::All,
                other @ Determiner::Headed(_) => {
                    panic!("unexpected mass determiner: {other:?}")
                }
            };
            self.0.push(determiner);
        }
        deckmaste_english_v2::visit::walk_determined_nominal(self, value);
    }
}

#[test]
fn zero_and_all_mass_nominals_share_the_determined_nominal_path() {
    let parser = parser();
    let context = context();

    for (text, expected) in [
        ("Destroy damage.", MassDeterminer::Zero),
        ("Destroy all damage.", MassDeterminer::All),
    ] {
        let analysis = parser.analyze(text, &context);
        let ability = analysis
            .selected()
            .unwrap_or_else(|| panic!("{text:?} must select: {analysis:?}"));
        assert_eq!(ability.render(&context, parser.environment()), text);
        let ownership = analysis
            .ownership()
            .unwrap_or_else(|| panic!("{text:?} has selected ownership"));
        assert!(ownership.failures().is_empty(), "{text:?}: {ownership:?}");
        assert!(ownership.summary().covered(), "{text:?}: {ownership:?}");

        let mut visitor = MassDeterminerVisitor::default();
        visitor.visit_ability(ability);
        assert_eq!(visitor.0, [expected], "{text:?}");
    }

    assert!(
        parser.parse("Destroy creature.", &context).is_err(),
        "the zero determiner still refuses a bare singular count nominal",
    );
}

#[test]
fn any_and_all_keep_their_declared_fusedness_distributions() {
    let parser = parser();
    let context = context();

    for (text, construction) in [
        ("Destroy any of them.", "DeterminativePartitive"),
        (
            "Exile all the cards from your hand.",
            "AllPredeterminedNominal",
        ),
    ] {
        let analysis = parser.analyze(text, &context);
        let selected = analysis
            .selected()
            .unwrap_or_else(|| panic!("{text:?} must select: {analysis:#?}"));
        assert_eq!(selected.render(&context, parser.environment()), text);
        let decision = analysis.decision().expect("selected parse has a decision");
        let selected_ordinal = decision
            .selected()
            .expect("decision has a selected candidate");
        assert!(
            decision.candidates()[selected_ordinal]
                .construction_path()
                .iter()
                .any(|step| step.contains(construction)),
            "{text:?} keeps its declared determinative route",
        );
    }

    for text in ["Destroy any.", "Destroy all."] {
        assert!(
            parser.parse(text, &context).is_err(),
            "{text:?} must not admit a bare nominal object",
        );
    }
}
