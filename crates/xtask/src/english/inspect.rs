use std::collections::BTreeSet;
use std::io::Write;
use std::io::{self};

use anyhow::Result;
use anyhow::bail;
use clap::Args;
use deckmaste_english::Catalogs;
use deckmaste_english::ConstructionBackend;
use deckmaste_english::ConstructionDecision;
use deckmaste_english::ConstructionEvidenceKind;
use deckmaste_english::ConstructionOwner;
use deckmaste_english::ParseCost;
use deckmaste_english::ParseCostDimension;
use deckmaste_english::ParseReport;
use deckmaste_english::SelectionReason;
use deckmaste_english::parse_with_identity;
use deckmaste_english::syntax::AbilityKind;
use deckmaste_english::syntax::IndependentClause;
use deckmaste_english::syntax::Predicate;
use deckmaste_english::syntax::PredicateAdjunct;
use deckmaste_english::syntax::PredicateComplement;
use deckmaste_english::syntax::PredicateElement;
use deckmaste_english::syntax::PredicateExpression;
use deckmaste_english::syntax::SentenceBody;

use super::data::CardFace;
use super::data::OracleDataArgs;

#[derive(Debug, Args)]
pub(super) struct InspectArgs {
    /// Exact card or face name (matched case-insensitively).
    card: String,

    #[command(flatten)]
    data: OracleDataArgs,

    #[command(flatten)]
    output_config: OutputConfig,
}

#[derive(Debug, Args)]
struct OutputConfig {
    /// Include diagnostics and parse provenance with their byte spans.
    #[arg(short, long)]
    verbose: bool,

    /// Print only the parsed abilities and any diagnostics.
    #[arg(short, long)]
    abilities_only: bool,
}

pub(super) fn run(args: &InspectArgs) -> Result<()> {
    let data = args.data.load()?;
    let cards = find_cards(&data.faces, &args.card);

    if cards.is_empty() {
        bail!(
            "no exact card or face named {:?} in {}",
            args.card,
            data.data_path.display()
        );
    }

    write_cards(
        io::stdout().lock(),
        &cards,
        &data.catalogs,
        &args.output_config,
    )
}

pub(super) fn find_cards(faces: &[CardFace], query: &str) -> Vec<CardFace> {
    let mut standalone = Vec::new();
    let mut whole_card = Vec::new();
    let mut face = Vec::new();

    for card in faces {
        match &card.face_name {
            None if card.card_name.eq_ignore_ascii_case(query) => standalone.push(card.clone()),
            Some(_) if card.card_name.eq_ignore_ascii_case(query) => whole_card.push(card.clone()),
            Some(face_name) if face_name.eq_ignore_ascii_case(query) => face.push(card.clone()),
            _ => {}
        }
    }

    if !standalone.is_empty() {
        standalone
    } else if !whole_card.is_empty() {
        whole_card
    } else {
        face
    }
}

fn write_cards(
    mut writer: impl Write,
    cards: &[CardFace],
    catalogs: &Catalogs,
    output_config: &OutputConfig,
) -> Result<()> {
    for (index, card) in cards.iter().enumerate() {
        if index != 0 {
            writeln!(writer)?;
        }

        match &card.face_name {
            Some(face_name) => writeln!(writer, "{} — {face_name}", card.card_name)?,
            None => writeln!(writer, "{}", card.card_name)?,
        }
        writeln!(writer, "\nOracle text:\n{}", card.oracle_text)?;
        let report = parse_with_identity(
            &card.oracle_text,
            catalogs,
            card.printed_name(),
            card.is_legendary,
        );
        let ast = report.ast();
        if output_config.abilities_only {
            writeln!(writer, "\nAbilities:\n{:#?}", ast.abilities)?;
        } else {
            writeln!(writer, "\nAST:\n{}", structural_debug(ast))?;
        }
        write_diagnostics(
            &mut writer,
            &report,
            &card.oracle_text,
            output_config.verbose,
        )?;
        if output_config.verbose && !report.provenance().selections().is_empty() {
            write_provenance(&mut writer, &report)?;
        }
    }

    Ok(())
}

fn structural_debug(value: &impl std::fmt::Debug) -> String {
    format!("{value:#?}")
}

fn write_provenance(mut writer: impl Write, report: &ParseReport) -> Result<()> {
    writeln!(writer, "\nProvenance:")?;
    for selection in report.provenance().selections() {
        for decision in selection.constructions() {
            writeln!(writer, "{}", construction_decision_text(decision))?;
            for alternative in decision.alternatives() {
                writeln!(
                    writer,
                    "  alternative {}#{} dominated={}",
                    alternative.id(),
                    alternative.production_ordinal(),
                    alternative.is_dominated(),
                )?;
            }
        }
    }
    write_p02_attachment_evidence(&mut writer, report)?;
    Ok(())
}

fn write_p02_attachment_evidence(mut writer: impl Write, report: &ParseReport) -> Result<()> {
    let has_p02 = report.provenance().selections().iter().any(|selection| {
        selection
            .constructions()
            .iter()
            .any(|decision| decision.selected().as_str() == "prepositional_phrase")
    });
    if !has_p02 {
        return Ok(());
    }

    let mut roles = BTreeSet::new();
    if report.provenance().selections().iter().any(|selection| {
        selection
            .constructions()
            .iter()
            .any(|decision| decision.selected().as_str() == "nominal_prepositional")
    }) {
        roles.insert("nominal");
    }
    for ability in &report.ast().abilities {
        if let AbilityKind::Paragraph(paragraph) = &ability.kind {
            for sentence in &paragraph.sentences {
                if let SentenceBody::Independent(clause) = sentence.body() {
                    collect_predicate_pp_roles(clause, &mut roles);
                }
            }
        }
    }
    if !roles.is_empty() {
        writeln!(writer, "P02 consuming attachment roles:")?;
        for role in roles {
            writeln!(writer, "  role={role}")?;
        }
    }
    Ok(())
}

fn collect_predicate_pp_roles(clause: &IndependentClause, roles: &mut BTreeSet<&'static str>) {
    match clause {
        IndependentClause::Transitive(_, predicate) => {
            collect_pp_element_roles(predicate.elements(), roles);
        }
        IndependentClause::Intransitive(_, predicate) => {
            collect_pp_element_roles(predicate.elements(), roles);
        }
        IndependentClause::Passive(_, predicate) => {
            collect_pp_element_roles(predicate.elements(), roles);
        }
        IndependentClause::Imperative(predicate) => collect_predicate_roles(predicate, roles),
        IndependentClause::Predicated(_, expression) => {
            collect_predicate_expression_roles(expression, roles);
        }
        IndependentClause::Deontic(_, _, Some(predicate)) => {
            collect_predicate_roles(predicate, roles);
        }
        IndependentClause::Copular(_, predicate) => {
            if matches!(
                predicate.complement,
                deckmaste_english::syntax::CopularComplement::Prepositional(_)
            ) {
                roles.insert("selected-complement");
            }
            collect_pp_adjunct_roles(&predicate.adjuncts, roles);
        }
        IndependentClause::Deontic(_, _, None)
        | IndependentClause::Existential(_)
        | IndependentClause::Proform(_, _)
        | IndependentClause::Complex(_)
        | IndependentClause::Coordinated(_) => {}
    }
}

fn collect_predicate_expression_roles(
    expression: &PredicateExpression,
    roles: &mut BTreeSet<&'static str>,
) {
    match expression {
        PredicateExpression::Simple(predicate) => collect_predicate_roles(predicate, roles),
        PredicateExpression::Coordinated(coordination) => {
            for conjunct in coordination.conjuncts() {
                collect_predicate_expression_roles(conjunct, roles);
            }
        }
    }
}

fn collect_predicate_roles(predicate: &Predicate, roles: &mut BTreeSet<&'static str>) {
    match predicate {
        Predicate::Transitive(predicate) => collect_pp_element_roles(predicate.elements(), roles),
        Predicate::Intransitive(predicate) => collect_pp_element_roles(predicate.elements(), roles),
        Predicate::Passive(predicate) => collect_pp_element_roles(predicate.elements(), roles),
        Predicate::Copular(predicate) => {
            if matches!(
                predicate.complement,
                deckmaste_english::syntax::CopularComplement::Prepositional(_)
            ) {
                roles.insert("selected-complement");
            }
            collect_pp_adjunct_roles(&predicate.adjuncts, roles);
        }
        Predicate::Deontic(predicate) => {
            if let Some(inner) = &predicate.inner {
                collect_predicate_roles(inner, roles);
            }
        }
        Predicate::Attached(predicate) => collect_predicate_roles(&predicate.predicate, roles),
        Predicate::Proform(_) => {}
    }
}

fn collect_pp_element_roles(elements: &[PredicateElement], roles: &mut BTreeSet<&'static str>) {
    for element in elements {
        match element {
            PredicateElement::Complement(PredicateComplement::Prepositional(_)) => {
                roles.insert("selected-complement");
            }
            PredicateElement::Adjunct(
                PredicateAdjunct::Prepositional(_) | PredicateAdjunct::Exception(_),
            ) => {
                roles.insert("adjunct");
            }
            PredicateElement::Complement(_)
            | PredicateElement::Adjunct(_)
            | PredicateElement::Particle(_)
            | PredicateElement::CoinResult(_) => {}
        }
    }
}

fn collect_pp_adjunct_roles(adjuncts: &[PredicateAdjunct], roles: &mut BTreeSet<&'static str>) {
    if adjuncts.iter().any(|adjunct| {
        matches!(
            adjunct,
            PredicateAdjunct::Prepositional(_) | PredicateAdjunct::Exception(_)
        )
    }) {
        roles.insert("adjunct");
    }
}

/// One construction row in verbose inspect. Keeping this formatter generic
/// over the public decision value lets generated families use the ordinary
/// English inspect path without adding a predicate-specific formatter.
fn construction_decision_text(decision: &ConstructionDecision) -> String {
    let span = decision.span();
    let evidence = decision.evidence();
    let evidence_value = decision
        .evidence_value()
        .map_or_else(String::new, |value| format!(" value={value}"));
    format!(
        "bytes {}..{} {} owner={} backend={} form={} evidence={}:{}{} reason={} cost={}",
        span.start,
        span.end,
        decision.selected(),
        owner_name(decision.owner()),
        backend_name(decision.backend()),
        decision.selected_production_ordinal(),
        evidence_kind_name(evidence.kind()),
        evidence.label(),
        evidence_value,
        reason_name(decision.reason()),
        cost_text(decision.cost()),
    )
}

const fn owner_name(owner: ConstructionOwner) -> &'static str {
    match owner {
        ConstructionOwner::Handwritten => "handwritten",
        ConstructionOwner::Generated => "generated",
    }
}

const fn backend_name(backend: ConstructionBackend) -> &'static str {
    match backend {
        ConstructionBackend::Chart => "chart",
        ConstructionBackend::Ability => "ability",
    }
}

const fn evidence_kind_name(kind: ConstructionEvidenceKind) -> &'static str {
    match kind {
        ConstructionEvidenceKind::Structural => "structural",
        ConstructionEvidenceKind::Guard => "guard",
        ConstructionEvidenceKind::Feature => "feature",
        ConstructionEvidenceKind::Role => "role",
    }
}

fn reason_name(reason: SelectionReason) -> String {
    match reason {
        SelectionReason::Unique => "unique".to_owned(),
        SelectionReason::Cost(dimension) => format!("cost:{}", cost_dimension_name(dimension)),
        SelectionReason::Dominance => "dominance".to_owned(),
        SelectionReason::StableIdentity => "stable_identity".to_owned(),
    }
}

const fn cost_dimension_name(dimension: ParseCostDimension) -> &'static str {
    match dimension {
        ParseCostDimension::OpaqueWords => "opaque_words",
        ParseCostDimension::OpaqueLexemes => "opaque_lexemes",
        ParseCostDimension::GenericRules => "generic_rules",
        ParseCostDimension::ReadingDispreference => "reading_dispreference",
        ParseCostDimension::AttachmentCount => "attachment_count",
        ParseCostDimension::AttachmentDistance => "attachment_distance",
        ParseCostDimension::AttachmentExtent => "attachment_extent",
        ParseCostDimension::Precedence => "precedence",
    }
}

fn cost_text(cost: ParseCost) -> String {
    format!(
        "{{opaque_words:{},opaque_lexemes:{},generic_rules:{},reading_dispreference:{},\
         attachment_count:{},attachment_distance:{},attachment_extent:{},precedence:{}}}",
        cost.opaque_words(),
        cost.opaque_lexemes(),
        cost.generic_rules(),
        cost.reading_dispreference(),
        cost.attachment_count(),
        cost.attachment_distance(),
        cost.attachment_extent(),
        cost.precedence(),
    )
}

fn write_diagnostics(
    mut writer: impl Write,
    report: &ParseReport,
    source: &str,
    verbose: bool,
) -> Result<()> {
    if report.diagnostics().is_empty() {
        return Ok(());
    }
    if verbose {
        writeln!(writer, "\nDiagnostics:\n{:#?}", report.diagnostics())?;
        return Ok(());
    }

    writeln!(writer, "\nDiagnostics:")?;
    for diagnostic in report.diagnostics() {
        let text = diagnostic
            .span()
            .text(source)
            .unwrap_or("<invalid source span>");
        writeln!(writer, "  {:?} at {text:?}", diagnostic.kind())?;
    }
    Ok(())
}

#[cfg(test)]
mod tests {
    use std::io::Cursor;
    use std::path::Path;
    use std::time::Duration;

    use deckmaste_english::normalize_roll_row_dashes;
    use deckmaste_english::normalize_sentence_case;
    use deckmaste_english::normalize_typographic_quotes;
    use deckmaste_english::strip_reminder_text;

    use super::*;
    use crate::english::data::map_supported_faces;
    use crate::english::data::read_card_faces;

    const DATA_PATH: &str = "test-cards.jsonl";

    struct RoundTripOutcome {
        index: usize,
        printed_name: String,
        elapsed: Duration,
        failure: Option<String>,
    }

    fn cards(data: &str) -> Vec<CardFace> {
        read_card_faces(Cursor::new(data), Path::new(DATA_PATH)).unwrap()
    }

    #[test]
    fn standalone_name_wins_over_a_face_with_the_same_name() {
        let data = concat!(
            r#"{"name":"Borrow","face":null,"text":"Draw a card."}"#,
            "\n",
            r#"{"name":"Borrow // Return","face":"Borrow","text":"Return target creature."}"#,
            "\n",
        );

        let cards = find_cards(&cards(data), "borrow");

        assert_eq!(
            cards,
            [CardFace {
                card_name: "Borrow".to_owned(),
                face_name: None,
                is_legendary: false,
                supported: false,
                source_text: "Draw a card.".to_owned(),
                oracle_text: "Draw a card.".to_owned(),
            }]
        );
    }

    #[test]
    fn combined_name_returns_every_face() {
        let data = concat!(
            r#"{"name":"Fire // Ice","face":"Fire","text":"Deal damage."}"#,
            "\n",
            r#"{"name":"Fire // Ice","face":"Ice","text":"Tap a permanent."}"#,
            "\n",
        );

        let cards = find_cards(&cards(data), "FIRE // ICE");

        assert_eq!(cards.len(), 2);
        assert_eq!(cards[0].face_name.as_deref(), Some("Fire"));
        assert_eq!(cards[1].face_name.as_deref(), Some("Ice"));
    }

    #[test]
    fn face_name_returns_only_that_face() {
        let data = concat!(
            r#"{"name":"Fire // Ice","face":"Fire","text":"Deal damage."}"#,
            "\n",
            r#"{"name":"Fire // Ice","face":"Ice","text":"Tap a permanent."}"#,
            "\n",
        );

        let cards = find_cards(&cards(data), "ice");

        assert_eq!(cards.len(), 1);
        assert_eq!(cards[0].face_name.as_deref(), Some("Ice"));
    }

    #[test]
    fn incoming_legendary_names_are_kept_verbatim_in_the_name_bearing_domain() {
        let data = concat!(
            r#"{"name":"Aang, A Lot to Learn","face":null,"supertypes":["Legendary"],"text":"Aang attacks. Aang, A Lot to Learn's power is 3."}"#,
            "\n",
        );

        let cards = find_cards(&cards(data), "Aang, A Lot to Learn");

        // The self-reference is recognized during the parse, not rewritten on
        // the way in, so the Oracle text keeps the face's own name verbatim.
        assert_eq!(
            cards[0].oracle_text,
            "Aang attacks. Aang, A Lot to Learn's power is 3."
        );
    }

    #[test]
    fn incoming_double_faced_cards_keep_the_face_name_verbatim() {
        let data = concat!(
            r#"{"name":"Aang, Swift Savior // Aang and La, Ocean's Fury","face":"Aang, Swift Savior","supertypes":["Legendary"],"text":"Aang transforms. Aang, Swift Savior has flying."}"#,
            "\n",
        );

        let cards = find_cards(&cards(data), "Aang, Swift Savior");

        assert_eq!(
            cards[0].oracle_text,
            "Aang transforms. Aang, Swift Savior has flying."
        );
    }

    #[test]
    fn incoming_typographic_quotes_normalize_before_parsing() {
        let data = concat!(
            r#"{"name":"A Realm Reborn","face":null,"text":"Other permanents you control have “{T}: Add one mana of any color.”"}"#,
            "\n",
        );

        let cards = cards(data);

        assert_eq!(
            cards[0].oracle_text,
            "Other permanents you control have \"{T}: Add one mana of any color.\""
        );
    }

    #[test]
    fn adjective_structural_debug_keeps_face_degree_marker_and_standard_categories() {
        use deckmaste_english::Fragment;
        use deckmaste_english::Numeral;
        use deckmaste_english::adjective as adjective_api;
        use deckmaste_english::syntax::Clause;
        use deckmaste_english::syntax::NumberLiteral;
        use deckmaste_english::syntax::SentenceBody;
        use deckmaste_english::syntax::ThisCardForm;
        use deckmaste_english::word::Adjective;
        use deckmaste_english::word::Vocab;

        let face = adjective_api::build_adjective_phrase_face_down().unwrap();
        let measured = adjective_api::build_adjective_phrase_degree_measure(
            NumberLiteral {
                value: 2,
                numeral: Numeral::Arabic(false),
            },
            Adjective::Word(Vocab::Greater),
        )
        .unwrap();
        let target = adjective_api::build_adjective_phrase(Adjective::Word(Vocab::Target)).unwrap();
        let parsed = deckmaste_english::parse_fragment(
            "Draw a card.",
            &Catalogs::default(),
            deckmaste_english::FragmentKind::Sentence,
            "Test Card",
            false,
        )
        .into_fragment()
        .unwrap();
        let Fragment::Sentence(sentence) = parsed else {
            panic!("the clause fixture is a sentence")
        };
        let SentenceBody::Independent(clause) = sentence.body() else {
            panic!("the clause fixture is independent")
        };

        let standards = [
            adjective_api::build_comparison_standard(
                Some(
                    deckmaste_english::noun_phrase::build_noun_phrase_this_card(
                        ThisCardForm::AbbreviatedName,
                    )
                    .unwrap(),
                ),
                None,
                None,
            )
            .unwrap(),
            adjective_api::build_comparison_standard(None, Some(target), None).unwrap(),
            adjective_api::build_comparison_standard(
                None,
                None,
                Some(Clause::Independent(clause.clone())),
            )
            .unwrap(),
        ];

        let face = structural_debug(&face);
        let measured = structural_debug(&measured);
        assert!(
            face.contains("CardOrientation(\n        FaceDown"),
            "{face}"
        );
        assert!(measured.contains("degree: Some"), "{measured}");
        assert!(measured.contains("Arabic"), "{measured}");
        for (standard, category) in
            standards
                .iter()
                .zip(["NounPhrase", "AdjectivePhrase", "Clause"])
        {
            let comparison = adjective_api::build_comparison_than(standard.clone()).unwrap();
            let debug = structural_debug(&comparison);
            assert!(debug.contains("marker: Than"), "{debug}");
            assert!(debug.contains(category), "{debug}");
        }
    }

    #[test]
    fn production_j01_inspect_reports_generated_owners() {
        let verbose = verbose_m01("Its power is greater than a card.");
        for construction in [
            "adjective",
            "adjective_phrase",
            "comparison_standard",
            "comparison_than",
            "adjective_phrase_comparison",
        ] {
            assert!(
                verbose.contains(&format!(" {construction} owner=generated backend=chart ")),
                "missing production-generated J01 owner {construction}:\n{verbose}"
            );
        }
    }

    #[test]
    fn production_p01_inspect_reports_generated_owners_and_decisive_constraints() {
        for (source, construction, evidence) in [
            (
                "They draw a card.",
                "noun_phrase_subject_pronoun",
                "feature:pronoun case",
            ),
            (
                "Destroy them.",
                "noun_phrase_object_pronoun",
                "feature:pronoun case",
            ),
            (
                "Destroy all creatures except artifacts.",
                "noun_phrase_set_exception_bare",
                "feature:set-exception host eligibility",
            ),
            (
                "Any number of target players draw a card.",
                "noun_phrase_any_number_of",
                "feature:notional plural agreement",
            ),
            (
                "This card deals damage to you and creatures you control that are tapped.",
                "rules_object_noun_phrase",
                "role:rules-object attachment role",
            ),
        ] {
            let verbose = verbose_m01(source);
            assert!(
                verbose.contains(&format!(
                    " {construction} owner=generated backend=chart form=0 evidence={evidence}"
                )),
                "missing generated P01 owner/evidence {construction}:\n{verbose}"
            );
        }
    }

    #[test]
    fn production_p02_inspect_reports_generated_object_and_attachment_evidence() {
        let selected = verbose_m01("Look at the top card of your library.");
        assert!(
            selected.contains(
                " prepositional_object owner=generated backend=chart form=0 evidence=feature:prepositional object category value=object_category=NounPhrase"
            ),
            "{selected}",
        );
        assert!(
            selected.contains(" verb_phrase_prepositional owner=generated backend=chart "),
            "selected-complement consumer missing:\n{selected}",
        );
        assert!(
            selected
                .split_whitespace()
                .collect::<Vec<_>>()
                .join(" ")
                .contains("Complement( Prepositional("),
            "selected PP lost its typed complement role:\n{selected}",
        );
        assert!(
            selected.contains("P02 consuming attachment roles:")
                && selected.contains("  role=selected-complement"),
            "selected PP lacks decisive consuming-role evidence:\n{selected}",
        );

        for (source, form, category) in [
            ("Attack from among them.", 1, "PrepositionalPhrase"),
            ("Attack by paying 1 life.", 2, "GerundClause"),
            ("Attack from anywhere.", 3, "Adverb"),
        ] {
            let rendered = verbose_m01(source);
            assert!(
                rendered.contains(&format!(
                    " prepositional_object owner=generated backend=chart form={form} evidence=feature:prepositional object category value=object_category={category}"
                )),
                "missing typed {category} object evidence:\n{rendered}",
            );
        }

        let adjunct = verbose_m01("Attack during your turn.");
        assert!(
            adjunct.contains(" verb_phrase_prepositional owner=generated backend=chart "),
            "adjunct consumer missing:\n{adjunct}",
        );
        assert!(
            adjunct
                .split_whitespace()
                .collect::<Vec<_>>()
                .join(" ")
                .contains("Adjunct( Prepositional("),
            "adjunct PP lost its typed consumer role:\n{adjunct}",
        );
        assert!(
            adjunct.contains("P02 consuming attachment roles:\n  role=adjunct"),
            "adjunct PP lacks decisive consuming-role evidence:\n{adjunct}",
        );

        let nominal = verbose_m01("Destroy target creature with flying.");
        assert!(
            nominal.contains(
                " nominal_prepositional owner=generated backend=chart form=0 evidence=feature:nominal attachment phase"
            ),
            "nominal-attachment consumer missing:\n{nominal}",
        );
        assert!(
            nominal.contains("P02 consuming attachment roles:\n  role=nominal"),
            "nominal PP lacks decisive consuming-role evidence:\n{nominal}",
        );
        for rendered in [&selected, &adjunct, &nominal] {
            assert!(
                rendered.contains(" prepositional_phrase owner=generated backend=chart "),
                "P02 phrase owner missing:\n{rendered}",
            );
        }
    }

    #[test]
    fn production_d01_inspect_reports_generated_owners_and_constraint_evidence() {
        let target = verbose_m01("Target creature gets +1/+1 until end of turn.");
        assert!(
            target.contains(
                " determiner_target owner=generated backend=chart form=0 evidence=feature:singular target cardinality"
            ),
            "{target}",
        );

        let possessive = verbose_m01("The creature's controller draws two cards.");
        for (construction, evidence) in [
            ("possessive_noun_base", "noun possessor number and onset"),
            (
                "possessive_noun_determined",
                "possessor determination and agreement",
            ),
            ("determiner_possessive_noun", "noun possessor determiner"),
            ("determiner_quantity", "quantity cardinality"),
        ] {
            assert!(
                possessive.contains(&format!(
                    " {construction} owner=generated backend=chart form=0 evidence=feature:{evidence}"
                )) || possessive.contains(&format!(
                    " {construction} owner=generated backend=chart form=0 evidence=role:{evidence}"
                )),
                "missing generated D01 evidence for {construction}:\n{possessive}",
            );
        }
    }

    #[test]
    fn production_f01_inspect_reports_generated_forms_and_constraints() {
        let infinitive = verbose_m01("You may choose not to untap this creature.");
        assert!(
            infinitive.contains(
                " infinitive_not_to owner=generated backend=chart form=0 evidence=feature:complete infinitive predicate form and valency"
            ),
            "{infinitive}",
        );

        let gerund = verbose_m01(
            "You may cast that card by paying life equal to the spell's mana value rather than paying its mana cost.",
        );
        for (construction, evidence) in [
            (
                "gerund_clause_base",
                "complete present-participle predicate form and valency",
            ),
            (
                "gerund_clause_subordinate_after",
                "typed trailing rather-than gerund attachment",
            ),
        ] {
            assert!(
                gerund.contains(&format!(
                    " {construction} owner=generated backend=chart form=0 evidence=feature:{evidence}"
                )),
                "missing generated F01 evidence for {construction}:\n{gerund}",
            );
        }
    }

    #[test]
    fn production_r01_inspect_reports_decisive_generated_relative_evidence() {
        for (source, construction, gap, marker, contraction, distributive_each, copular) in [
            (
                "Each spell you cast costs {1} less to cast.",
                "relative_object",
                "Object",
                "Zero",
                "Uncontracted",
                false,
                "NonCopular",
            ),
            (
                "Each spell you've cast costs {1} less to cast.",
                "relative_object_contracted_subject",
                "Object",
                "Zero",
                "SubjectAuxiliary",
                false,
                "NonCopular",
            ),
            (
                "A creature that attacks gets +1/+1.",
                "relative_subject",
                "Subject",
                "That",
                "Uncontracted",
                false,
                "NonCopular",
            ),
            (
                "Creature cards that each have a different mana value get +1/+1.",
                "relative_subject_distributive_each",
                "Subject",
                "That",
                "Uncontracted",
                true,
                "NonCopular",
            ),
            (
                "A card that's a creature is colorless.",
                "relative_contracted_copular_noun",
                "Subject",
                "That",
                "Copular",
                false,
                "Noun",
            ),
            (
                "A card that's red is colorless.",
                "relative_contracted_copular_adjective",
                "Subject",
                "That",
                "Copular",
                false,
                "Adjective",
            ),
            (
                "A card that's in exile is colorless.",
                "relative_contracted_copular_prepositional",
                "Subject",
                "That",
                "Copular",
                false,
                "Prepositional",
            ),
        ] {
            let verbose = verbose_m01(source);
            let evidence = format!(
                " {construction} owner=generated backend=chart form=0 evidence=feature:decisive relative form signature value=gap={gap};marker={marker};agreement="
            );
            let evidence_line = verbose
                .lines()
                .find(|line| line.contains(&evidence))
                .unwrap_or_else(|| {
                    panic!("missing generated R01 evidence line for {source:?}:\n{verbose}")
                });
            assert!(
                evidence_line.contains(&format!("contraction={contraction}"))
                    && evidence_line.contains(&format!("distributive_each={distributive_each}"))
                    && evidence_line.contains(&format!("copular={copular}"))
                    && evidence_line.contains("rules_object=")
                    && evidence_line.contains("bare_copular_tail="),
                "missing decisive typed R01 evidence for {source:?}:\n{verbose}",
            );
            assert!(
                !verbose.contains(&format!(" {construction} owner=handwritten ")),
                "R01 retained a handwritten owner for {source:?}:\n{verbose}",
            );
        }
    }

    #[test]
    fn production_f03_inspect_reports_generated_attachment_scope() {
        let fronted = verbose_m01("Otherwise, draw a card.");
        assert!(
            fronted
                .contains(" clause_sentence_adverbial_before owner=generated backend=chart form=0"),
            "{fronted}",
        );

        let subordinate = verbose_m01("If you control a Plains, creatures you control get +1/+1.");
        assert!(
            subordinate.contains(" clause_subordinate_before owner=generated backend=chart form=0 evidence=feature:finite subordinate selection and host eligibility"),
            "{subordinate}",
        );

        let restriction = verbose_m01("Activate only as a sorcery and only once each turn.");
        for construction in ["clause_restriction_member", "clause_restriction_run"] {
            assert!(
                restriction.contains(&format!(" {construction} owner=generated backend=chart ")),
                "missing generated F03 owner {construction}:\n{restriction}",
            );
        }
    }

    #[test]
    fn normal_output_resolves_spans_while_verbose_output_keeps_them() {
        let cards = [CardFace {
            card_name: "Test Card".to_owned(),
            face_name: None,
            is_legendary: false,
            supported: false,
            source_text: "Draw a card.".to_owned(),
            oracle_text: "Draw a card.".to_owned(),
        }];
        let mut normal = Vec::new();
        let mut verbose = Vec::new();
        let catalogs = Catalogs::default();
        let normal_config = OutputConfig {
            verbose: false,
            abilities_only: false,
        };
        let verbose_config = OutputConfig {
            verbose: true,
            abilities_only: false,
        };

        write_cards(&mut normal, &cards, &catalogs, &normal_config).unwrap();
        write_cards(&mut verbose, &cards, &catalogs, &verbose_config).unwrap();
        let normal = String::from_utf8(normal).unwrap();
        let verbose = String::from_utf8(verbose).unwrap();

        assert!(normal.contains("verb: VerbInstance {"));
        assert!(normal.contains("Word(\n"));
        assert!(normal.contains("Draw,"));
        assert!(!normal.contains("Span"));
        assert!(!normal.contains("ChartStats"));
        assert!(!normal.contains("ForestStats"));
        assert!(verbose.contains("Provenance:"));
        assert!(verbose.contains("bytes 0.."));
        // Inspect is the public provenance surface and must attribute both
        // the root and its noun phrase to their generated families.
        assert!(verbose.contains("sentence owner=generated backend=chart"));
        assert!(verbose.contains("noun owner=generated backend=chart"));
        assert!(!verbose.contains("owner=handwritten"));
        assert!(verbose.contains("cost={opaque_words:"));
        assert!(!verbose.contains("Span {"));
        assert!(!verbose.contains("ChartStats"));
        assert!(!verbose.contains("ForestStats"));
    }

    #[test]
    fn verbose_output_distinguishes_known_and_opaque_generated_noun_owners() {
        // Mutations caught: attribute noun_opaque to the known family, admit
        // it in the zero-cost exact result, or omit opaque generated
        // provenance from inspect.
        let render = |source: &str| {
            let cards = [CardFace {
                card_name: "Test Card".to_owned(),
                face_name: None,
                is_legendary: false,
                supported: false,
                source_text: source.to_owned(),
                oracle_text: source.to_owned(),
            }];
            let mut verbose = Vec::new();
            write_cards(
                &mut verbose,
                &cards,
                &Catalogs::default(),
                &OutputConfig {
                    verbose: true,
                    abilities_only: false,
                },
            )
            .unwrap();
            String::from_utf8(verbose).unwrap()
        };

        let known = render("Draw a card.");
        assert!(
            known.contains(
                "noun owner=generated backend=chart form=0 evidence=structural:generated production reason=unique cost={opaque_words:0,opaque_lexemes:0"
            ),
            "{known}"
        );
        assert!(!known.contains("noun_opaque owner="));

        let opaque = render("Draw a BlOrPlE.");
        assert!(
            opaque.contains(
                "noun_opaque owner=generated backend=chart form=0 evidence=structural:generated production reason=unique cost={opaque_words:1,opaque_lexemes:1"
            ),
            "{opaque}"
        );
        assert!(!opaque.contains(" noun owner=generated"));
    }

    #[test]
    fn verbose_quote_terminal_output_reports_generated_sentence_form_identity() {
        // Mutation caught: inspect only the ordinary period form, or discard
        // the generated form ordinal after lowering a terminal quote.
        let cards = [CardFace {
            card_name: "Test Aura".to_owned(),
            face_name: None,
            is_legendary: false,
            supported: false,
            source_text: "Enchanted creature has \"{T}: Draw a card.\"".to_owned(),
            oracle_text: "Enchanted creature has \"{T}: Draw a card.\"".to_owned(),
        }];
        let catalogs = Catalogs::default()
            .with_catalog(deckmaste_english::CatalogKind::CardType, ["Creature"]);
        let mut verbose = Vec::new();
        write_cards(
            &mut verbose,
            &cards,
            &catalogs,
            &OutputConfig {
                verbose: true,
                abilities_only: false,
            },
        )
        .unwrap();
        let verbose = String::from_utf8(verbose).unwrap();

        assert!(
            verbose.contains("sentence owner=generated backend=chart form=1"),
            "{verbose}"
        );
        assert!(verbose.contains("QuotedAbility"));
        assert!(verbose.contains("alternative sentence#1"));
    }

    fn verbose_m01(source: &str) -> String {
        let cards = [CardFace {
            card_name: "Test Card".to_owned(),
            face_name: None,
            is_legendary: false,
            supported: false,
            source_text: source.to_owned(),
            oracle_text: source.to_owned(),
        }];
        let catalogs = Catalogs::default()
            .with_catalog(
                deckmaste_english::CatalogKind::KeywordAbility,
                ["Protection"],
            )
            .with_catalog(deckmaste_english::CatalogKind::CreatureType, ["Ally"])
            .with_catalog(
                deckmaste_english::CatalogKind::CardType,
                ["Artifact", "Creature", "Instant", "Land", "Sorcery"],
            );
        let mut rendered = Vec::new();
        write_cards(
            &mut rendered,
            &cards,
            &catalogs,
            &OutputConfig {
                verbose: true,
                abilities_only: false,
            },
        )
        .unwrap();
        String::from_utf8(rendered).unwrap()
    }

    fn assert_verbose_dominance(source: &str, winner: &str, loser: &str) {
        let rendered = verbose_m01(source);
        let lines = rendered.lines().collect::<Vec<_>>();
        let selected = lines
            .iter()
            .position(|line| {
                line.contains(&format!(" {winner} owner=generated backend=chart "))
                    && line.contains(" reason=dominance ")
            })
            .unwrap_or_else(|| {
                panic!("{source:?} did not report generated dominance winner {winner}:\n{rendered}")
            });
        assert!(
            lines[selected + 1..]
                .iter()
                .take_while(|line| line.starts_with("  alternative "))
                .any(|line| line.contains(&format!("alternative {loser}#0 dominated=true"))),
            "{source:?} did not report {loser} as the dominated alternative to {winner}:\n{rendered}",
        );
    }

    #[test]
    fn verbose_output_explains_every_m01_dominance_edge() {
        // Each fixture names the competing construction pair. Removing an
        // edge, restoring a handwritten owner, or omitting the defeated
        // alternative makes the corresponding row fail causally.
        for (source, winner, loser) in [
            (
                "Look at the top two cards of your library.",
                "nominal_quantity_modifier",
                "nominal_prepositional",
            ),
            (
                "The top two creatures attacking get +1/+1.",
                "nominal_quantity_modifier",
                "nominal_postpositive_adjective",
            ),
            (
                "The top two greater creatures than a card get +1/+1.",
                "nominal_quantity_modifier",
                "nominal_comparison",
            ),
            (
                "If an opponent controls at least four more creatures than you, this spell costs {6} less to cast.",
                "nominal_determiner",
                "nominal_comparison",
            ),
            (
                "You may spend colorless mana as though it were mana of any color to cast that spell.",
                "nominal_prepositional",
                "nominal_infinitive",
            ),
            (
                "Choose target artifact or land card in your graveyard.",
                "nominal_prepositional",
                "nominal_coordinated_modifier",
            ),
            (
                "You have protection from each of your opponents.",
                "nominal_prepositional",
                "nominal_keyword_predicated_argument",
            ),
            (
                "Return this card from your graveyard to your hand.",
                "nominal_prepositional",
                "nominal_noun",
            ),
            (
                "Create a 1/1 white Ally creature token for each experience counter you have.",
                "nominal_relative",
                "nominal_prepositional",
            ),
        ] {
            assert_verbose_dominance(source, winner, loser);
        }

        // The reduced-passive edge remains declaration-owned and is asserted
        // in the construction registry. Its production predicate now carries
        // an attachment cost before the completed nominal competes with the
        // opaque-noun alternative, so provenance correctly reports the
        // reduced-passive construction as unique instead of attributing the
        // selection to dominance.
        let reduced = verbose_m01(
            "If a creature dealt damage this way would die this turn, exile it instead.",
        );
        assert!(
            reduced.contains(
                "nominal_reduced_recipient_passive owner=generated backend=chart form=0 evidence=guard:reduced-recipient-passive frame"
            ),
            "{reduced}"
        );
    }

    #[test]
    fn verbose_output_reports_production_generated_predicate_ownership() {
        // Mutations caught: format generated ownership only for isolated
        // English fixtures, or leave xtask's production parse on RuleTag.
        let rendered = verbose_m01("You may have this creature enter.");
        assert!(
            rendered.contains(
                "verb_phrase_causative owner=generated backend=chart form=0 evidence=role:causative host-causee-complement order"
            ),
            "{rendered}"
        );
    }

    #[test]
    fn verbose_output_formats_a_declaration_evidence_value() {
        let keyword = verbose_m01("This creature has protection from red and from blue.");
        assert!(
            keyword.contains(
                "predicated_argument_from_extend owner=generated backend=chart form=0 evidence=guard:keyword-grant conjunction gate value=conjunction=And;allowed=[And];matched=true"
            ),
            "{keyword}",
        );
    }

    #[test]
    fn abilities_only_omits_the_full_ast_wrapper() {
        let cards = [CardFace {
            card_name: "Test Card".to_owned(),
            face_name: None,
            is_legendary: false,
            supported: false,
            source_text: "Draw a card.".to_owned(),
            oracle_text: "Draw a card.".to_owned(),
        }];
        let output = OutputConfig {
            verbose: false,
            abilities_only: true,
        };
        let mut rendered = Vec::new();

        write_cards(&mut rendered, &cards, &Catalogs::default(), &output).unwrap();

        let rendered = String::from_utf8(rendered).unwrap();
        assert!(rendered.contains("Abilities:"));
        assert!(!rendered.contains("AST:"));
        assert!(!rendered.contains("OracleText {"));
        assert!(!rendered.contains("Span"));
    }

    #[test]
    #[cfg_attr(
        not(all(derived_cards, gen_catalogs)),
        ignore = "needs data/derived/cards.jsonl and data/gen/catalogs"
    )]
    fn local_card_snapshot_structurally_round_trips_without_source_text() {
        use std::time::Instant;

        let data = OracleDataArgs::default()
            .load()
            .expect("release corpus data must be available for the structural round-trip gate");

        let outcomes = map_supported_faces(&data.faces, |index, card| {
            let started = Instant::now();
            let report = parse_with_identity(
                &card.oracle_text,
                &data.catalogs,
                card.printed_name(),
                card.is_legendary,
            );
            let elapsed = started.elapsed();
            let printed_name = card.printed_name().to_owned();
            let failure = match report
                .into_ast()
                .render(card.printed_name(), card.is_legendary)
            {
                Ok(rebuilt)
                    if normalized_rules_text(&rebuilt)
                        == normalized_rules_text(&card.source_text) =>
                {
                    None
                }
                Ok(rebuilt) => Some(format!(
                    "row {} ({}):\n  rendered: {rebuilt:?}\n  expected: {:?}",
                    index + 1,
                    card.printed_name(),
                    normalized_rules_text(&card.source_text)
                )),
                Err(error) => Some(format!(
                    "row {} ({}): render error: {error}",
                    index + 1,
                    card.printed_name()
                )),
            };
            RoundTripOutcome {
                index,
                printed_name,
                elapsed,
                failure,
            }
        });

        let mut failures = Vec::new();
        for outcome in outcomes {
            if outcome.elapsed.as_millis() >= 100 {
                eprintln!(
                    "slow row {} ({}) {:?}",
                    outcome.index + 1,
                    outcome.printed_name,
                    outcome.elapsed
                );
            }
            if outcome.index % 1_000 == 0 {
                eprintln!(
                    "reached row {} ({})",
                    outcome.index + 1,
                    outcome.printed_name
                );
            }
            if let Some(failure) = outcome.failure {
                failures.push(failure);
            }
        }

        assert!(
            failures.is_empty(),
            "{} structural round-trip failures (first 20):\n{}",
            failures.len(),
            failures
                .iter()
                .take(20)
                .cloned()
                .collect::<Vec<_>>()
                .join("\n")
        );
    }

    #[test]
    #[cfg_attr(
        not(all(derived_cards, gen_catalogs)),
        ignore = "needs data/derived/cards.jsonl and data/gen/catalogs"
    )]
    fn quantified_times_clauses_keep_the_determiner_outside_the_special_base() {
        // Mutation caught: make the generated nominal-determiner inverse admit
        // only complement-free and devotion values. `three/five/as many times
        // <clause>` is represented by a determiner structurally outside the
        // specialized `times <clause>` base and must linearize in that order.
        let data = OracleDataArgs::default()
            .load()
            .expect("release corpus data must be available for the times-clause gate");

        for name in [
            "Burn at the Stake",
            "Crackle with Power",
            "Lim-Dûl's Vault",
            "Ojer Taq, Deepest Foundation",
        ] {
            let card = data
                .faces
                .iter()
                .find(|card| card.printed_name() == name)
                .unwrap_or_else(|| panic!("missing times-clause fixture {name:?}"));
            let rebuilt = parse_with_identity(
                &card.oracle_text,
                &data.catalogs,
                card.printed_name(),
                card.is_legendary,
            )
            .into_ast()
            .render(card.printed_name(), card.is_legendary)
            .unwrap_or_else(|error| panic!("{name}: {error}"));
            assert_eq!(
                normalized_rules_text(&rebuilt),
                normalized_rules_text(&card.source_text),
                "{name}"
            );
        }
    }

    fn normalized_rules_text(text: &str) -> String {
        // Sentence case is normalized last, after reminder text is gone, so a
        // stripped reminder can never shift a sentence boundary's position.
        normalize_sentence_case(&normalize_roll_row_dashes(&normalize_typographic_quotes(
            &strip_reminder_text(text),
        )))
    }
}
