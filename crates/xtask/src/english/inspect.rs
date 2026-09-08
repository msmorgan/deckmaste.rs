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
use deckmaste_english::ParseCost;
use deckmaste_english::ParseCostDimension;
use deckmaste_english::ParseReport;
use deckmaste_english::SelectionReason;
use deckmaste_english::parse_with_identity;
use deckmaste_english::syntax::AbilityKind;
use deckmaste_english::syntax::ClauseAttachment;
use deckmaste_english::syntax::ClauseAttachmentKind;
use deckmaste_english::syntax::CoordinatedClauseMember;
use deckmaste_english::syntax::DependentClause;
use deckmaste_english::syntax::GerundClause;
use deckmaste_english::syntax::GerundClauseKind;
use deckmaste_english::syntax::IndependentClause;
use deckmaste_english::syntax::Predicate;
use deckmaste_english::syntax::PredicateAdjunct;
use deckmaste_english::syntax::PredicateComplement;
use deckmaste_english::syntax::PredicateElement;
use deckmaste_english::syntax::PredicateExpression;
use deckmaste_english::syntax::RelativeBody;
use deckmaste_english::syntax::SentenceBody;
use deckmaste_english::syntax::SubordinateBody;

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
                    "  alternative {}#{} dominated={} cost={}",
                    alternative.id(),
                    alternative.production_ordinal(),
                    alternative.is_dominated(),
                    cost_text(alternative.cost()),
                )?;
            }
        }
    }
    write_attachment_role_evidence(&mut writer, report)?;
    Ok(())
}

fn write_attachment_role_evidence(mut writer: impl Write, report: &ParseReport) -> Result<()> {
    let has_attachment_roles = report.provenance().selections().iter().any(|selection| {
        selection
            .constructions()
            .iter()
            .any(|decision| decision.selected().as_str() == "prepositional_phrase")
    });
    if !has_attachment_roles {
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
        if let AbilityKind::Paragraph(paragraph) = ability.kind() {
            for sentence in &paragraph.sentences {
                if let SentenceBody::Independent(clause) = sentence.body() {
                    collect_predicate_pp_roles(clause, &mut roles);
                }
            }
        }
    }
    if !roles.is_empty() {
        writeln!(writer, "Consuming attachment roles:")?;
        for role in roles {
            writeln!(writer, "  role={role}")?;
        }
    }
    Ok(())
}

fn collect_predicate_pp_roles(clause: &IndependentClause, roles: &mut BTreeSet<&'static str>) {
    match clause {
        IndependentClause::Finite(finite) => {
            collect_predicate_expression_roles(finite.predicate(), roles);
        }
        IndependentClause::Existential(_) => {}
        IndependentClause::Complex(complex) => {
            collect_predicate_pp_roles(complex.host(), roles);
            collect_clause_attachment_pp_roles(complex.attachment(), roles);
        }
        IndependentClause::Coordinated(coordinated) => {
            collect_predicate_pp_roles(coordinated.first(), roles);
            for coordination in coordinated.rest() {
                match coordination.member() {
                    CoordinatedClauseMember::Independent(clause) => {
                        collect_predicate_pp_roles(clause, roles);
                    }
                }
            }
        }
    }
}

fn collect_clause_attachment_pp_roles(
    attachment: &ClauseAttachment,
    roles: &mut BTreeSet<&'static str>,
) {
    match attachment.payload() {
        ClauseAttachmentKind::Dependent(clause) => {
            collect_dependent_clause_pp_roles(clause, roles);
        }
        ClauseAttachmentKind::Adjunct(adjunct) => {
            collect_predicate_adjunct_pp_roles(adjunct, roles);
        }
        ClauseAttachmentKind::Exception(rider) => {
            collect_predicate_pp_roles(rider.first(), roles);
            for conjunct in rider.rest() {
                collect_predicate_pp_roles(conjunct.clause(), roles);
            }
        }
        ClauseAttachmentKind::Restriction(run) => {
            for adjunct in run.first().adjuncts() {
                collect_predicate_adjunct_pp_roles(adjunct, roles);
            }
            for coordination in run.rest() {
                for adjunct in coordination.member().adjuncts() {
                    collect_predicate_adjunct_pp_roles(adjunct, roles);
                }
            }
        }
        ClauseAttachmentKind::Appositive(clause) => collect_predicate_pp_roles(clause, roles),
    }
}

fn collect_dependent_clause_pp_roles(clause: &DependentClause, roles: &mut BTreeSet<&'static str>) {
    match clause {
        DependentClause::Subordinate(_, body) => match body {
            SubordinateBody::Finite(clause) => collect_predicate_pp_roles(clause, roles),
            SubordinateBody::CoordinatedFinite(body) => {
                collect_predicate_pp_roles(body.first(), roles);
                collect_predicate_pp_roles(body.next(), roles);
            }
            SubordinateBody::Infinitive(clause) => {
                collect_predicate_roles(clause.predicate(), roles);
            }
            SubordinateBody::Gerund(clause) => collect_gerund_clause_pp_roles(clause, roles),
            SubordinateBody::Elliptical(_) => {}
        },
        DependentClause::Relative(relative) => match relative.body() {
            RelativeBody::SubjectGap(predicate) => collect_predicate_roles(predicate, roles),
            RelativeBody::ObjectGap { predicate, .. } => {
                collect_pp_element_roles(predicate.elements(), roles);
            }
        },
        DependentClause::Infinitive(clause) => {
            collect_predicate_roles(clause.predicate(), roles);
        }
        DependentClause::Gerund(clause) => collect_gerund_clause_pp_roles(clause, roles),
    }
}

fn collect_gerund_clause_pp_roles(clause: &GerundClause, roles: &mut BTreeSet<&'static str>) {
    match clause.kind() {
        GerundClauseKind::Base { predicate } => collect_predicate_roles(predicate, roles),
        GerundClauseKind::RatherThan {
            matrix,
            alternative,
        } => {
            collect_gerund_clause_pp_roles(matrix, roles);
            collect_gerund_clause_pp_roles(alternative, roles);
        }
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
        Predicate::Transitive(predicate) => {
            collect_pp_element_roles(predicate.pre_object_elements(), roles);
            collect_pp_element_roles(predicate.elements(), roles);
        }
        Predicate::Intransitive(predicate) => collect_pp_element_roles(predicate.elements(), roles),
        Predicate::Passive(predicate) => collect_pp_element_roles(predicate.elements(), roles),
        Predicate::Copular(predicate) => {
            if matches!(
                predicate.complement(),
                deckmaste_english::syntax::CopularComplement::Prepositional(_)
            ) {
                roles.insert("selected-complement");
            }
            collect_pp_adjunct_roles(predicate.adjuncts(), roles);
        }
        Predicate::Deontic(predicate) => {
            if let Some(inner) = predicate.inner() {
                collect_predicate_expression_roles(inner, roles);
            }
        }
        Predicate::Attached(predicate) => {
            collect_predicate_roles(predicate.predicate(), roles);
            collect_clause_attachment_pp_roles(predicate.attachment(), roles);
        }
        Predicate::Proform(_) => {}
    }
}

fn collect_pp_element_roles(elements: &[PredicateElement], roles: &mut BTreeSet<&'static str>) {
    for element in elements {
        match element {
            PredicateElement::Complement(PredicateComplement::Prepositional(_)) => {
                roles.insert("selected-complement");
            }
            PredicateElement::Complement(PredicateComplement::Infinitive(clause)) => {
                collect_predicate_roles(clause.predicate(), roles);
            }
            PredicateElement::Adjunct(adjunct) => {
                collect_predicate_adjunct_pp_roles(adjunct, roles);
            }
            PredicateElement::Complement(_)
            | PredicateElement::Particle(_)
            | PredicateElement::CoinResult(_) => {}
        }
    }
}

fn collect_pp_adjunct_roles(adjuncts: &[PredicateAdjunct], roles: &mut BTreeSet<&'static str>) {
    for adjunct in adjuncts {
        collect_predicate_adjunct_pp_roles(adjunct, roles);
    }
}

fn collect_predicate_adjunct_pp_roles(
    adjunct: &PredicateAdjunct,
    roles: &mut BTreeSet<&'static str>,
) {
    match adjunct {
        PredicateAdjunct::Prepositional(_) | PredicateAdjunct::Exception(_) => {
            roles.insert("adjunct");
        }
        PredicateAdjunct::Dependent(clause) => collect_dependent_clause_pp_roles(clause, roles),
        PredicateAdjunct::Adverb(_)
        | PredicateAdjunct::Frequency(_)
        | PredicateAdjunct::Temporal(_)
        | PredicateAdjunct::Manner(_)
        | PredicateAdjunct::AbilityPostmodifier(_) => {}
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
        "bytes {}..{} {} backend={} form={} evidence={}:{}{} reason={} cost={}",
        span.start,
        span.end,
        decision.selected(),
        backend_name(decision.backend()),
        decision.selected_production_ordinal(),
        evidence_kind_name(evidence.kind()),
        evidence.label(),
        evidence_value,
        reason_name(decision.reason()),
        cost_text(decision.cost()),
    )
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

    use deckmaste_english::normalize_loyalty_minus;
    use deckmaste_english::normalize_roll_row_dashes;
    use deckmaste_english::normalize_sentence_case;
    use deckmaste_english::normalize_typographic_quotes;
    use deckmaste_english::strip_reminder_text;
    use deckmaste_english::syntax::NounPhraseKind;
    use deckmaste_english::word::Vocab;

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

    fn attachment_topology(attachment: &ClauseAttachment) -> String {
        let payload = match attachment.payload() {
            ClauseAttachmentKind::Dependent(DependentClause::Subordinate(subordinator, _)) => {
                format!("Subordinate({subordinator:?})")
            }
            ClauseAttachmentKind::Dependent(_) => "Dependent".to_owned(),
            ClauseAttachmentKind::Adjunct(_) => "Adjunct".to_owned(),
            ClauseAttachmentKind::Exception(_) => "Exception".to_owned(),
            ClauseAttachmentKind::Appositive(_) => "Appositive".to_owned(),
            ClauseAttachmentKind::Restriction(_) => "Restriction".to_owned(),
        };
        format!("{:?}:{payload}", attachment.position())
    }

    fn predicate_topology(predicate: &Predicate) -> String {
        match predicate {
            Predicate::Transitive(_) => "Transitive".to_owned(),
            Predicate::Intransitive(_) => "Intransitive".to_owned(),
            Predicate::Copular(_) => "Copular".to_owned(),
            Predicate::Passive(_) => "Passive".to_owned(),
            Predicate::Proform(_) => "Proform".to_owned(),
            Predicate::Deontic(deontic) => deontic.inner().map_or_else(
                || "Deontic(elided)".to_owned(),
                |inner| format!("Deontic({})", expression_topology(inner)),
            ),
            Predicate::Attached(attached) => format!(
                "Attached({}, {})",
                attachment_topology(attached.attachment()),
                predicate_topology(attached.predicate())
            ),
        }
    }

    fn expression_topology(expression: &PredicateExpression) -> String {
        match expression {
            PredicateExpression::Simple(predicate) => predicate_topology(predicate),
            PredicateExpression::Coordinated(coordination) => format!(
                "PredCoord[{}]",
                coordination
                    .conjuncts()
                    .iter()
                    .map(expression_topology)
                    .collect::<Vec<_>>()
                    .join(", ")
            ),
        }
    }

    fn clause_topology(clause: &IndependentClause) -> String {
        match clause {
            IndependentClause::Finite(finite) => format!(
                "Finite(subject={}, {})",
                finite.subject().is_some(),
                expression_topology(finite.predicate())
            ),
            IndependentClause::Existential(_) => "Existential".to_owned(),
            IndependentClause::Complex(complex) => format!(
                "Complex({}, {})",
                attachment_topology(complex.attachment()),
                clause_topology(complex.host())
            ),
            IndependentClause::Coordinated(coordination) => {
                let mut members = vec![clause_topology(coordination.first())];
                members.extend(
                    coordination
                        .rest()
                        .iter()
                        .map(|member| match member.member() {
                            CoordinatedClauseMember::Independent(clause) => clause_topology(clause),
                        }),
                );
                format!("ClauseCoord[{}]", members.join(", "))
            }
        }
    }

    fn clause_topologies(report: &ParseReport) -> String {
        let topologies = report
            .ast()
            .abilities
            .iter()
            .flat_map(|ability| match ability.kind() {
                AbilityKind::Paragraph(paragraph) => paragraph
                    .sentences
                    .iter()
                    .filter_map(|sentence| match sentence.body() {
                        SentenceBody::Independent(clause) => Some(clause_topology(clause)),
                        _ => None,
                    })
                    .collect::<Vec<_>>(),
                _ => Vec::new(),
            })
            .collect::<Vec<_>>();
        if topologies.is_empty() {
            "no independent clause".to_owned()
        } else {
            topologies.join("; ")
        }
    }

    fn assert_chaos_matrix(report: &ParseReport, oracle_source: &str, source: &str) {
        assert!(oracle_source.contains(&source[1..]));
        let [ability] = report.ast().abilities.as_slice() else {
            panic!("Chaos Mutation's matrix must produce one ability")
        };
        let AbilityKind::Paragraph(paragraph) = ability.kind() else {
            panic!("Chaos Mutation's matrix must produce a paragraph")
        };
        let [sentence] = paragraph.sentences.as_slice() else {
            panic!("Chaos Mutation's matrix must produce one sentence")
        };
        let SentenceBody::Independent(IndependentClause::Finite(finite)) = sentence.body() else {
            panic!(
                "Chaos Mutation's matrix must be one finite clause: {:#?}",
                sentence.body()
            )
        };
        let PredicateExpression::Coordinated(coordination) = finite.predicate() else {
            panic!("Chaos Mutation's matrix must retain its predicate coordination")
        };
        assert_eq!(coordination.conjuncts().len(), 3);
        assert!(matches!(
            coordination.conjuncts().first(),
            Some(PredicateExpression::Simple(Predicate::Attached(predicate)))
                if matches!(
                    predicate.attachment().payload(),
                    ClauseAttachmentKind::Dependent(DependentClause::Subordinate(
                        deckmaste_english::syntax::Subordinator::Until,
                        _
                    ))
                )
        ));
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
    fn production_comparison_inspect_reports_chart_constructions() {
        let verbose = verbose_parse("Its power is greater than a card.");
        for construction in [
            "adjective",
            "adjective_phrase",
            "comparison_standard",
            "comparison_than",
            "adjective_phrase_comparison",
        ] {
            assert!(
                verbose.contains(&format!(" {construction} backend=chart ")),
                "missing comparison construction {construction}:\n{verbose}"
            );
        }
    }

    #[test]
    fn production_cost_inspect_reports_nested_ability_backend() {
        let verbose =
            verbose_parse("{2}: Choose one —\n• Draw a card.\n• Create a Treasure token.");
        let cost = verbose
            .lines()
            .find(|line| line.contains(" cost backend=ability "))
            .unwrap_or_else(|| panic!("missing generated cost decision:\n{verbose}"));
        assert!(
            cost.contains("evidence=role:activation-cost root"),
            "{cost}"
        );
    }

    #[test]
    fn production_ability_inspect_reports_generated_semantic_root() {
        let verbose = verbose_parse("Choose one —\n• Draw a card.\n• Create a Treasure token.");
        let ability = verbose
            .lines()
            .find(|line| line.contains(" ability backend=ability "))
            .unwrap_or_else(|| panic!("missing generated ability decision:\n{verbose}"));
        assert!(
            ability.contains("evidence=guard:decisive ability frame guard"),
            "{ability}"
        );
    }

    #[test]
    fn ability_collision_inspect_reports_ranked_alternatives_and_decisive_cost() {
        let verbose = verbose_parse("Ward—Discard a card: Draw a card.");
        let ability = verbose
            .lines()
            .find(|line| line.contains(" ability backend=ability "))
            .unwrap_or_else(|| panic!("missing generated ability decision:\n{verbose}"));
        assert!(
            ability.contains("evidence=guard:decisive ability frame guard")
                && ability.contains("reason=cost:precedence")
                && ability.ends_with("precedence:0}"),
            "{ability}"
        );
        let alternatives = verbose
            .lines()
            .filter(|line| line.trim_start().starts_with("alternative ability#"))
            .collect::<Vec<_>>();
        assert_eq!(alternatives.len(), 2, "{verbose}");
        assert!(
            alternatives
                .iter()
                .all(|line| line.contains("dominated=false")),
            "{alternatives:#?}"
        );
        assert!(
            alternatives
                .iter()
                .any(|line| line.ends_with("precedence:4}"))
                && alternatives
                    .iter()
                    .any(|line| line.ends_with("precedence:0}")),
            "{alternatives:#?}"
        );
    }

    #[test]
    fn production_noun_phrase_inspect_reports_constructions_and_decisive_constraints() {
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
            let verbose = verbose_parse(source);
            assert!(
                verbose.contains(&format!(
                    " {construction} backend=chart form=0 evidence={evidence}"
                )),
                "missing noun-phrase construction/evidence {construction}:\n{verbose}"
            );
        }
    }

    #[test]
    fn production_inspect_reports_common_head_head_list_and_mixed_with_constructions() {
        for (source, construction) in [
            (
                "Tap an Elf, Orc, or enchantment creature you control.",
                "nominal_coordinated_modifier",
            ),
            (
                "Search your library for a basic land card or Gate card, reveal it, put it into your hand, then shuffle.",
                "shared_determiner_nominal",
            ),
            (
                "Create a 1/1 red Alien creature token with haste and \"This token attacks each combat if able.\"",
                "nominal_with_attributes",
            ),
        ] {
            let verbose = verbose_parse(source);
            assert!(
                verbose.contains(&format!(" {construction} backend=chart ")),
                "missing coordination construction {construction} for {source:?}:\n{verbose}"
            );
        }

        let mixed = verbose_parse(
            "Create a 1/1 red Alien creature token with haste and \"This token attacks each combat if able.\"",
        );
        assert!(
            mixed.contains(" with_attribute_list_conjoined backend=chart "),
            "mixed `with` list did not expose its list construction:\n{mixed}"
        );
        assert!(
            mixed.contains("with_attribute_member_quoted backend=chart "),
            "mixed `with` list did not expose its quoted-member construction:\n{mixed}"
        );
    }

    #[test]
    #[cfg_attr(
        not(all(derived_cards, gen_catalogs)),
        ignore = "needs data/derived/cards.jsonl and data/gen/catalogs-legacy; run `cargo xtask catalogs text`"
    )]
    fn supported_mixed_with_cards_expose_the_dedicated_construction() {
        let data = OracleDataArgs::default()
            .load()
            .expect("release corpus data must be available for mixed `with` fixtures");
        for name in ["Alien Invasion", "Basilica Shepherd", "Blink"] {
            let cards = find_cards(&data.faces, name)
                .into_iter()
                .filter(|card| card.supported)
                .collect::<Vec<_>>();
            assert_eq!(cards.len(), 1, "expected one supported fixture for {name}");
            let mut rendered = Vec::new();
            write_cards(
                &mut rendered,
                &cards,
                &data.catalogs,
                &OutputConfig {
                    verbose: true,
                    abilities_only: false,
                },
            )
            .expect("mixed `with` fixture must inspect");
            let rendered = String::from_utf8(rendered).unwrap();
            assert!(
                rendered.contains(" nominal_with_attributes backend=chart "),
                "{name} did not select the dedicated nominal `with` construction:\n{rendered}"
            );
            assert!(
                rendered.contains(" with_attribute_member_quoted backend=chart "),
                "{name} did not expose the quoted-member construction:\n{rendered}"
            );
        }
    }

    #[test]
    #[cfg_attr(
        not(all(derived_cards, gen_catalogs)),
        ignore = "needs data/derived/cards.jsonl and data/gen/catalogs-legacy; run `cargo xtask catalogs text`"
    )]
    fn supported_phrase_coordination_cards_round_trip_with_generated_grouping() {
        let data = OracleDataArgs::default()
            .load()
            .expect("release corpus data must be available for coordination fixtures");
        for (name, grouping, construction, ability_line) in [
            (
                "Abzan Monument",
                "a basic Plains, Swamp, or Forest card",
                Some("shared_determiner_nominal"),
                None,
            ),
            (
                "Open the Gates",
                "a basic land card or Gate card",
                Some("shared_determiner_nominal"),
                None,
            ),
            (
                "Banishing Slash",
                "artifact, enchantment, or tapped creature",
                Some("nominal_coordinated_modifier"),
                None,
            ),
            (
                "Cowabunga!",
                "Mutant, Ninja, Turtle, or land card",
                Some("nominal_coordinated_modifier"),
                None,
            ),
            (
                "Monument to Perfection",
                "basic, Sphere, or Locus land",
                None,
                Some(0),
            ),
            ("Grassland Crusader", "Elf or Soldier creature", None, None),
        ] {
            let cards = find_cards(&data.faces, name);
            let [card] = cards.as_slice() else {
                panic!("expected one supported snapshot face for {name}, got {cards:#?}")
            };
            assert!(card.supported, "{name} must remain in the supported corpus");
            let (oracle_text, source_text) = if let Some(line) = ability_line {
                (
                    card.oracle_text
                        .lines()
                        .nth(line)
                        .unwrap_or_else(|| panic!("{name} has no Oracle ability line {line}")),
                    card.source_text
                        .lines()
                        .nth(line)
                        .unwrap_or_else(|| panic!("{name} has no source ability line {line}")),
                )
            } else {
                (card.oracle_text.as_str(), card.source_text.as_str())
            };
            assert_eq!(
                oracle_text, source_text,
                "{name} must not need lossy input normalization"
            );

            let report = parse_with_identity(
                oracle_text,
                &data.catalogs,
                card.printed_name(),
                card.is_legendary,
            );
            assert!(
                report.ast().recoveries().is_empty(),
                "{name} introduced recovery: {:#?}",
                report.diagnostics()
            );
            let rebuilt = report
                .ast()
                .render(card.printed_name(), card.is_legendary)
                .unwrap_or_else(|error| panic!("{name} failed to render: {error}"));
            assert_eq!(rebuilt, source_text, "{name} did not round-trip exactly");

            if let Some(construction) = construction {
                let selected = report
                    .provenance()
                    .selections()
                    .iter()
                    .find_map(|selection| {
                        let span = selection.span().text(oracle_text)?;
                        if !span.contains(grouping) {
                            return None;
                        }
                        selection
                            .constructions()
                            .iter()
                            .find(|decision| decision.selected().as_str() == construction)
                            .map(|decision| (span, decision))
                    });
                let Some((span, decision)) = selected else {
                    panic!(
                        "{name} did not select {construction} over grouping {grouping:?}: {:#?}",
                        report.provenance()
                    )
                };
                assert!(span.contains(grouping), "{name}: selected span {span:?}");
                assert_eq!(decision.backend(), ConstructionBackend::Chart, "{name}");
            }
        }
    }

    #[test]
    #[cfg_attr(
        not(all(derived_cards, gen_catalogs)),
        ignore = "needs data/derived/cards.jsonl and data/gen/catalogs-legacy; run `cargo xtask catalogs text`"
    )]
    fn recovery_composition_witnesses_remain_structural_in_corpus_inspect() {
        let data = OracleDataArgs::default()
            .load()
            .expect("release corpus data must be available for recovery-composition witnesses");

        let sage_owl = find_cards(&data.faces, "Sage Owl");
        let [sage_owl] = sage_owl.as_slice() else {
            panic!("expected one Sage Owl snapshot face")
        };
        assert!(sage_owl.supported, "Sage Owl must remain supported");
        let sage_source = "When this creature enters, look at the top four cards of your library, then put them back in any order.";
        assert_eq!(sage_owl.oracle_text.lines().nth(1), Some(sage_source));
        assert_eq!(sage_owl.source_text.lines().nth(1), Some(sage_source));
        let sage_report = parse_with_identity(
            sage_source,
            &data.catalogs,
            sage_owl.printed_name(),
            sage_owl.is_legendary,
        );
        assert!(sage_report.ast().recoveries().is_empty(), "Sage Owl");
        assert!(
            sage_report.ast().lexical_opacity().is_empty(),
            "Sage Owl must not trade recovery for lexical opacity"
        );
        assert_eq!(
            sage_report
                .ast()
                .render(sage_owl.printed_name(), sage_owl.is_legendary)
                .expect("Sage Owl sentence must render"),
            sage_source
        );
        let [ability] = sage_report.ast().abilities.as_slice() else {
            panic!("Sage Owl sentence must produce one ability")
        };
        let AbilityKind::Triggered(triggered) = ability.kind() else {
            panic!("Sage Owl sentence must retain its trigger frame")
        };
        let [sentence] = triggered.effect.sentences.as_slice() else {
            panic!("Sage Owl trigger must have one effect sentence")
        };
        let SentenceBody::Independent(IndependentClause::Finite(finite)) = sentence.body() else {
            panic!("Sage Owl effect must be a finite clause")
        };
        let PredicateExpression::Coordinated(coordination) = finite.predicate() else {
            panic!("Sage Owl effect must retain its predicate coordination")
        };
        let Some(PredicateExpression::Simple(Predicate::Transitive(put))) =
            coordination.conjuncts().last()
        else {
            panic!("Sage Owl put-back clause must remain transitive")
        };
        assert!(put.elements().iter().any(|element| matches!(
            element,
            PredicateElement::Adjunct(PredicateAdjunct::Adverb(Vocab::Back))
        )));

        let launch = find_cards(&data.faces, "Launch the Fleet");
        let [launch] = launch.as_slice() else {
            panic!("expected one Launch the Fleet snapshot face")
        };
        assert!(launch.supported, "Launch the Fleet must remain supported");
        let launch_source =
            "Strive — This spell costs {1} more to cast for each target beyond the first.";
        assert_eq!(launch.oracle_text.lines().next(), Some(launch_source));
        assert_eq!(launch.source_text.lines().next(), Some(launch_source));
        let launch_report = parse_with_identity(
            launch_source,
            &data.catalogs,
            launch.printed_name(),
            launch.is_legendary,
        );
        assert!(
            launch_report.ast().recoveries().is_empty(),
            "Launch the Fleet"
        );
        assert!(
            launch_report.ast().lexical_opacity().is_empty(),
            "Launch the Fleet must not trade recovery for lexical opacity"
        );
        assert_eq!(
            launch_report
                .ast()
                .render(launch.printed_name(), launch.is_legendary)
                .expect("Launch the Fleet rider must render"),
            launch_source
        );
        assert!(
            launch_report
                .ast()
                .noun_phrases()
                .iter()
                .any(|phrase| matches!(phrase.kind(), NounPhraseKind::TargetsBeyondFirst))
        );
        assert!(
            launch_report
                .provenance()
                .selections()
                .iter()
                .flat_map(deckmaste_english::ParseSelection::constructions)
                .any(|decision| decision.selected().as_str() == "noun_phrase_targets_beyond_first")
        );

        let marvel = find_cards(&data.faces, "Aetherworks Marvel");
        let [marvel] = marvel.as_slice() else {
            panic!("expected one Aetherworks Marvel snapshot face")
        };
        assert!(marvel.supported, "Aetherworks Marvel must remain supported");
        let marvel_source = "{T}, Pay six {E}: Look at the top six cards of your library. You may cast a spell from among them without paying its mana cost. Put the rest on the bottom of your library in a random order.";
        assert_eq!(marvel.oracle_text.lines().nth(1), Some(marvel_source));
        assert_eq!(marvel.source_text.lines().nth(1), Some(marvel_source));
        let marvel_report = parse_with_identity(
            marvel_source,
            &data.catalogs,
            marvel.printed_name(),
            marvel.is_legendary,
        );
        assert!(
            marvel_report.ast().recoveries().is_empty(),
            "Aetherworks Marvel"
        );
        assert!(
            marvel_report.ast().lexical_opacity().is_empty(),
            "Aetherworks Marvel must not trade recovery for lexical opacity"
        );
        assert_eq!(
            marvel_report
                .ast()
                .render(marvel.printed_name(), marvel.is_legendary)
                .expect("Aetherworks Marvel activation must render"),
            marvel_source
        );
        assert!(
            marvel_report
                .provenance()
                .selections()
                .iter()
                .flat_map(deckmaste_english::ParseSelection::constructions)
                .any(|decision| decision.selected().as_str() == "verb_phrase_counted_energy")
        );
    }

    #[test]
    #[cfg_attr(
        not(all(derived_cards, gen_catalogs)),
        ignore = "needs data/derived/cards.jsonl and data/gen/catalogs-legacy; run `cargo xtask catalogs text`"
    )]
    fn supported_clause_coordination_cards_round_trip_through_chart_constructions() {
        let data = OracleDataArgs::default()
            .load()
            .expect("release corpus data must be available for clause-coordination fixtures");
        for (name, ability_line, oracle_source, matrix_source, expected_constructions) in [
            (
                "Tek",
                0,
                "This creature gets +0/+2 as long as you control a Plains, has flying as long as you control an Island, gets +2/+0 as long as you control a Swamp, has first strike as long as you control a Mountain, and has trample as long as you control a Forest.",
                None,
                &["clause_coordination_comma", "clause_coordination_asyndetic"][..],
            ),
            (
                "Tribal Golem",
                0,
                r#"This creature has trample as long as you control a Beast, haste as long as you control a Goblin, first strike as long as you control a Soldier, flying as long as you control a Wizard, and "{B}: Regenerate this creature" as long as you control a Zombie."#,
                None,
                &["clause_coordination_shared_grant_comma"][..],
            ),
            (
                "Backwoods Survivalists",
                0,
                "Delirium — This creature gets +1/+1 and has trample as long as there are four or more card types among cards in your graveyard.",
                None,
                &["clause_coordination"][..],
            ),
            (
                "Tuinvale Guide",
                1,
                "Celebration — This creature gets +1/+0 and has lifelink as long as two or more nonland permanents entered the battlefield under your control this turn.",
                None,
                &["clause_coordination"][..],
            ),
            (
                "Dragon's Rage Channeler",
                1,
                "Delirium — As long as there are four or more card types among cards in your graveyard, this creature gets +2/+2, has flying, and attacks each combat if able.",
                None,
                &["clause_coordination_comma", "clause_coordination_asyndetic"][..],
            ),
            (
                "Chaos Mutation",
                0,
                "Exile any number of target creatures controlled by different players. For each creature exiled this way, its controller reveals cards from the top of their library until they reveal a creature card, puts that card onto the battlefield, then puts the rest on the bottom of their library in a random order.",
                Some(
                    "Its controller reveals cards from the top of their library until they reveal a creature card, puts that card onto the battlefield, then puts the rest on the bottom of their library in a random order.",
                ),
                &["clause_coordination_comma", "clause_coordination_asyndetic"][..],
            ),
            (
                "Sycorax Commander",
                1,
                "Sanctified Rules of Combat — When this creature enters, each opponent faces a villainous choice — That opponent discards all the cards in their hand, then draws that many cards minus one, or this creature deals damage to that player equal to the number of cards in their hand.",
                None,
                &["clause_coordination_comma"][..],
            ),
            (
                "Giant's Amulet",
                1,
                r#"Equipped creature gets +0/+1 and has "This creature has hexproof as long as it's untapped.""#,
                None,
                &["clause_coordination"][..],
            ),
        ] {
            let cards = find_cards(&data.faces, name);
            let [card] = cards.as_slice() else {
                panic!("expected one supported snapshot face for {name}, got {cards:#?}")
            };
            assert!(card.supported, "{name} must remain in the supported corpus");
            assert_eq!(
                card.oracle_text.lines().nth(ability_line),
                Some(oracle_source),
                "{name}'s verified Oracle fixture drifted"
            );

            let source = matrix_source.unwrap_or(oracle_source);

            let report = parse_with_identity(source, &data.catalogs, name, card.is_legendary);
            assert!(
                report.ast().recoveries().is_empty(),
                "{name} introduced recovery: {:#?}",
                report.diagnostics()
            );
            assert_eq!(
                report
                    .ast()
                    .render(name, card.is_legendary)
                    .unwrap_or_else(|error| panic!("{name} failed to render: {error}")),
                source,
                "{name} did not round-trip its normalized Oracle ability exactly"
            );

            if name == "Chaos Mutation" {
                assert_chaos_matrix(&report, oracle_source, source);
            }

            let decisions = report
                .provenance()
                .selections()
                .iter()
                .flat_map(deckmaste_english::ParseSelection::constructions)
                .filter(|decision| {
                    decision
                        .selected()
                        .as_str()
                        .starts_with("clause_coordination")
                })
                .collect::<Vec<_>>();
            assert!(
                !decisions.is_empty(),
                "{name} exposed no clause coordination construction"
            );
            assert!(
                decisions
                    .iter()
                    .all(|decision| decision.backend() == ConstructionBackend::Chart),
                "{name} retained a non-chart clause coordination construction: {decisions:#?}"
            );
            let mut inspected = Vec::new();
            write_provenance(&mut inspected, &report)
                .unwrap_or_else(|error| panic!("{name} failed verbose inspection: {error}"));
            let inspected = String::from_utf8(inspected).expect("inspect output is UTF-8");
            for expected in expected_constructions {
                assert!(
                    decisions
                        .iter()
                        .any(|decision| decision.selected().as_str() == *expected),
                    "{name} did not expose construction {expected}: {decisions:#?}"
                );
                assert!(
                    inspected.contains(&format!(" {expected} backend=chart ")),
                    "{name} omitted {expected} from verbose inspection:\n{inspected}"
                );
            }
        }
    }

    #[test]
    fn conditioned_quoted_predicate_is_complete_without_outer_coordination() {
        let source =
            r#"This creature has "{B}: Regenerate this creature" as long as you control a Zombie."#;
        let report = parse_with_identity(source, &inspect_fixture_catalogs(), "Test Card", false);
        assert!(
            report.ast().recoveries().is_empty(),
            "isolated quoted predicate recovered: {:#?}",
            report.diagnostics()
        );
        let [ability] = report.ast().abilities.as_slice() else {
            panic!("isolated predicate must produce one ability")
        };
        let AbilityKind::Paragraph(paragraph) = ability.kind() else {
            panic!("isolated predicate must produce a paragraph")
        };
        let [sentence] = paragraph.sentences.as_slice() else {
            panic!("isolated predicate must produce one sentence")
        };
        let SentenceBody::Independent(IndependentClause::Complex(complex)) = sentence.body() else {
            panic!("postpositive condition must produce a complex clause")
        };
        assert!(matches!(
            complex.attachment().payload(),
            ClauseAttachmentKind::Dependent(DependentClause::Subordinate(
                deckmaste_english::syntax::Subordinator::AsLongAs,
                _
            ))
        ));
        assert!(matches!(complex.host(), IndependentClause::Finite(_)));
        assert!(
            report
                .provenance()
                .selections()
                .iter()
                .flat_map(deckmaste_english::ParseSelection::constructions)
                .any(|decision| decision.selected().as_str() == "clause_subordinate_after")
        );
        assert_eq!(report.ast().render("Test Card", false).unwrap(), source);
    }

    #[test]
    #[cfg_attr(
        not(all(derived_cards, gen_catalogs)),
        ignore = "needs data/derived/cards.jsonl and data/gen/catalogs-legacy; run `cargo xtask catalogs text`"
    )]
    fn fronted_condition_scopes_over_the_complete_predicate_coordination() {
        let source = "As long as there are four or more card types among cards in your graveyard, this creature gets +2/+2, has flying, and attacks each combat if able.";
        let data = OracleDataArgs::default()
            .load()
            .expect("release corpus data must be available for the fronted-condition fixture");
        let report = parse_with_identity(source, &data.catalogs, "Dragon's Rage Channeler", false);
        let selected_clause_constructions = report
            .provenance()
            .selections()
            .iter()
            .flat_map(deckmaste_english::ParseSelection::constructions)
            .map(|decision| decision.selected().as_str())
            .filter(|construction| construction.starts_with("clause_"))
            .collect::<Vec<_>>();
        let [ability] = report.ast().abilities.as_slice() else {
            panic!("fronted condition must produce one ability")
        };
        let AbilityKind::Paragraph(paragraph) = ability.kind() else {
            panic!("fronted condition must produce a paragraph")
        };
        let [sentence] = paragraph.sentences.as_slice() else {
            panic!("fronted condition must produce one sentence")
        };
        let SentenceBody::Independent(IndependentClause::Complex(outer)) = sentence.body() else {
            panic!(
                "fronted condition must be the outer clause edge: {:#?}",
                sentence.body()
            )
        };
        assert_eq!(
            outer.attachment().position(),
            deckmaste_english::syntax::AttachmentPosition::BeforeMatrix,
            "the fronted condition must outscope the coordinated matrix; selected={selected_clause_constructions:#?}: {:#?}",
            sentence.body()
        );
        let IndependentClause::Finite(finite) = outer.host() else {
            panic!(
                "the outer condition must directly host the finite matrix: {:#?}",
                sentence.body()
            )
        };
        let PredicateExpression::Coordinated(coordination) = finite.predicate() else {
            panic!("the conditioned matrix must retain one predicate coordination")
        };
        assert_eq!(coordination.conjuncts().len(), 3);
        let Some(PredicateExpression::Simple(Predicate::Attached(final_predicate))) =
            coordination.conjuncts().last()
        else {
            panic!(
                "if able must remain local to the final predicate: {:#?}",
                sentence.body()
            )
        };
        assert!(matches!(
            final_predicate.attachment().payload(),
            ClauseAttachmentKind::Dependent(DependentClause::Subordinate(
                deckmaste_english::syntax::Subordinator::If,
                SubordinateBody::Elliptical(_)
            ))
        ));
        assert_eq!(
            report
                .ast()
                .render("Dragon's Rage Channeler", false)
                .unwrap(),
            source
        );
    }

    #[test]
    #[cfg_attr(
        not(all(derived_cards, gen_catalogs)),
        ignore = "needs data/derived/cards.jsonl and data/gen/catalogs-legacy; run `cargo xtask catalogs text`"
    )]
    fn binary_shared_condition_remains_group_wide() {
        let source = "Delirium — This creature gets +1/+1 and has trample as long as there are four or more card types among cards in your graveyard.";
        let data = OracleDataArgs::default()
            .load()
            .expect("release corpus data must be available for the shared-condition fixture");
        let report = parse_with_identity(source, &data.catalogs, "Backwoods Survivalists", false);
        let [ability] = report.ast().abilities.as_slice() else {
            panic!("shared condition must produce one ability")
        };
        let AbilityKind::Paragraph(paragraph) = ability.kind() else {
            panic!("shared condition must produce a paragraph")
        };
        let [sentence] = paragraph.sentences.as_slice() else {
            panic!("shared condition must produce one sentence")
        };
        let SentenceBody::Independent(IndependentClause::Complex(complex)) = sentence.body() else {
            panic!(
                "shared condition must remain a clause attachment: {:#?}",
                sentence.body()
            )
        };
        assert_eq!(
            complex.attachment().position(),
            deckmaste_english::syntax::AttachmentPosition::AfterMatrix
        );
        assert!(matches!(
            complex.attachment().payload(),
            ClauseAttachmentKind::Dependent(DependentClause::Subordinate(
                deckmaste_english::syntax::Subordinator::AsLongAs,
                _
            ))
        ));
        let IndependentClause::Finite(finite) = complex.host() else {
            panic!("shared condition must directly host the binary matrix")
        };
        let PredicateExpression::Coordinated(coordination) = finite.predicate() else {
            panic!("shared condition must host a predicate coordination")
        };
        assert_eq!(coordination.conjuncts().len(), 2);
        assert!(coordination.conjuncts().iter().all(|member| {
            !matches!(member, PredicateExpression::Simple(Predicate::Attached(_)))
        }));
        assert_eq!(
            report
                .ast()
                .render("Backwoods Survivalists", false)
                .unwrap(),
            source
        );
    }

    #[test]
    #[cfg_attr(
        not(all(derived_cards, gen_catalogs)),
        ignore = "needs data/derived/cards.jsonl and data/gen/catalogs-legacy; run `cargo xtask catalogs text`"
    )]
    fn dash_appositive_scopes_over_the_complete_predicate_matrix() {
        let source = "Target creature's owner shuffles it into their library, then faces a villainous choice — They lose 5 life, or they shuffle another creature they own into their library.";
        let data = OracleDataArgs::default()
            .load()
            .expect("release corpus data must be available for the dash-appositive fixture");
        let report = parse_with_identity(source, &data.catalogs, "This Is How It Ends", false);
        assert!(
            report.ast().recoveries().is_empty(),
            "dash appositive introduced recovery: {:#?}",
            report.diagnostics()
        );
        let [ability] = report.ast().abilities.as_slice() else {
            panic!("dash appositive must produce one ability")
        };
        let AbilityKind::Paragraph(paragraph) = ability.kind() else {
            panic!("dash appositive must produce a paragraph")
        };
        let [sentence] = paragraph.sentences.as_slice() else {
            panic!("dash appositive must produce one sentence")
        };
        let SentenceBody::Independent(IndependentClause::Complex(complex)) = sentence.body() else {
            panic!(
                "dash appositive must be the outer clause edge: {:#?}",
                sentence.body()
            )
        };
        assert!(matches!(
            complex.attachment().payload(),
            ClauseAttachmentKind::Appositive(body)
                if matches!(body.as_ref(), IndependentClause::Coordinated(_))
        ));
        let IndependentClause::Finite(finite) = complex.host() else {
            panic!(
                "the appositive must directly host its complete finite matrix: {:#?}",
                sentence.body()
            )
        };
        let PredicateExpression::Coordinated(coordination) = finite.predicate() else {
            panic!("the appositive matrix must retain the shuffles-then-faces coordination")
        };
        assert_eq!(coordination.conjuncts().len(), 2);
        assert_eq!(
            report.ast().render("This Is How It Ends", false).unwrap(),
            source
        );
    }

    #[test]
    #[cfg_attr(
        not(all(derived_cards, gen_catalogs)),
        ignore = "needs data/derived/cards.jsonl and data/gen/catalogs-legacy; run `cargo xtask catalogs text`"
    )]
    fn supported_nested_clause_coordination_neighbors_round_trip() {
        let data = OracleDataArgs::default()
            .load()
            .expect("release corpus data must be available for coordination neighbors");
        let mut failures = Vec::new();
        for (name, source) in [
            (
                "Charitable Levy",
                "Then if there are three or more collection counters on it, sacrifice it. If you do, draw a card, then you may search your library for a Plains card, put it onto the battlefield tapped, then shuffle.",
            ),
            (
                "Everybody Lives!",
                "Players can't lose life this turn and players can't lose the game or win the game this turn.",
            ),
            (
                "Grand Master of Flowers",
                "Target creature without first strike, double strike, or vigilance can't attack or block until your next turn.",
            ),
            (
                "Hidden Strings",
                "You may tap or untap target permanent, then you may tap or untap another target permanent.",
            ),
            (
                "Shared Fate",
                "Each player may look at cards they exiled with this enchantment, and they may play lands and cast spells from among those cards.",
            ),
            (
                "The Belligerent",
                "Until end of turn, you may look at the top card of your library any time, and you may play lands and cast spells from the top of your library.",
            ),
            (
                "Toils of Night and Day",
                "You may tap or untap target permanent, then you may tap or untap another target permanent.",
            ),
        ] {
            let cards = find_cards(&data.faces, name);
            let [card] = cards.as_slice() else {
                panic!("expected one supported snapshot face for {name}")
            };
            assert!(card.supported && card.oracle_text.contains(source));
            let report = parse_with_identity(source, &data.catalogs, name, card.is_legendary);
            assert!(
                report.ast().recoveries().is_empty(),
                "{name} introduced recovery: {:#?}",
                report.diagnostics()
            );
            let topology = clause_topologies(&report);
            match report.ast().render(name, card.is_legendary) {
                Ok(rendered) if rendered == source => {}
                Ok(rendered) => {
                    failures.push(format!("{name} rendered {rendered:?}, expected {source:?}"));
                }
                Err(error) => failures.push(format!(
                    "{name} failed to render: {error}; topology={topology}"
                )),
            }
        }
        assert!(failures.is_empty(), "{}", failures.join("\n"));
    }

    #[test]
    fn production_prepositional_inspect_reports_generated_object_and_attachment_evidence() {
        let selected = verbose_parse("Look at the top card of your library.");
        assert!(
            selected.contains(
                " prepositional_object backend=chart form=0 evidence=feature:prepositional object category value=object_category=NounPhrase"
            ),
            "{selected}",
        );
        assert!(
            selected.contains(" verb_phrase_prepositional backend=chart "),
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
            selected.contains("Consuming attachment roles:")
                && selected.contains("  role=selected-complement"),
            "selected PP lacks decisive consuming-role evidence:\n{selected}",
        );

        for (source, form, category) in [
            ("Attack from among them.", 1, "PrepositionalPhrase"),
            ("Attack by paying 1 life.", 2, "GerundClause"),
            ("Attack from anywhere.", 3, "Adverb"),
        ] {
            let rendered = verbose_parse(source);
            assert!(
                rendered.contains(&format!(
                    " prepositional_object backend=chart form={form} evidence=feature:prepositional object category value=object_category={category}"
                )),
                "missing typed {category} object evidence:\n{rendered}",
            );
        }

        let adjunct = verbose_parse("Attack during your turn.");
        assert!(
            adjunct.contains(" verb_phrase_prepositional backend=chart "),
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
            adjunct.contains("Consuming attachment roles:\n  role=adjunct"),
            "adjunct PP lacks decisive consuming-role evidence:\n{adjunct}",
        );

        let nominal = verbose_parse("Destroy target creature with flying.");
        assert!(
            nominal.contains(
                " nominal_prepositional backend=chart form=0 evidence=feature:nominal attachment phase"
            ),
            "nominal-attachment consumer missing:\n{nominal}",
        );
        assert!(
            nominal.contains("Consuming attachment roles:\n  role=nominal"),
            "nominal PP lacks decisive consuming-role evidence:\n{nominal}",
        );
        for rendered in [&selected, &adjunct, &nominal] {
            assert!(
                rendered.contains(" prepositional_phrase backend=chart "),
                "prepositional phrase construction missing:\n{rendered}",
            );
        }
    }

    #[test]
    fn production_determiner_inspect_reports_constructions_and_constraint_evidence() {
        let target = verbose_parse("Target creature gets +1/+1 until end of turn.");
        assert!(
            target.contains(
                " determiner_target backend=chart form=0 evidence=feature:singular target cardinality"
            ),
            "{target}",
        );

        let possessive = verbose_parse("The creature's controller draws two cards.");
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
                    " {construction} backend=chart form=0 evidence=feature:{evidence}"
                )) || possessive.contains(&format!(
                    " {construction} backend=chart form=0 evidence=role:{evidence}"
                )),
                "missing generated determiner evidence for {construction}:\n{possessive}",
            );
        }
    }

    #[test]
    fn production_nonfinite_inspect_reports_generated_forms_and_constraints() {
        let infinitive = verbose_parse("You may choose not to untap this creature.");
        assert!(
            infinitive.contains(
                " infinitive_not_to backend=chart form=0 evidence=feature:complete infinitive predicate form and valency"
            ),
            "{infinitive}",
        );

        let gerund = verbose_parse(
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
                    " {construction} backend=chart form=0 evidence=feature:{evidence}"
                )),
                "missing generated nonfinite evidence for {construction}:\n{gerund}",
            );
        }
    }

    #[test]
    fn production_relative_inspect_reports_decisive_generated_evidence() {
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
                "Destroy target permanent that opponent controls.",
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
                "A creature who attacks gets +1/+1.",
                "relative_subject",
                "Subject",
                "Who",
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
            let verbose = verbose_parse(source);
            let evidence = format!(
                " {construction} backend=chart form=0 evidence=feature:decisive relative form signature value=gap={gap};marker={marker};agreement="
            );
            let evidence_line = verbose
                .lines()
                .find(|line| line.contains(&evidence))
                .unwrap_or_else(|| {
                    panic!("missing generated relative evidence line for {source:?}:\n{verbose}")
                });
            assert!(
                evidence_line.contains(&format!("contraction={contraction}"))
                    && evidence_line.contains(&format!("distributive_each={distributive_each}"))
                    && evidence_line.contains(&format!("copular={copular}"))
                    && evidence_line.contains("rules_object=")
                    && evidence_line.contains("bare_copular_tail="),
                "missing decisive typed relative evidence for {source:?}:\n{verbose}",
            );
        }
    }

    #[test]
    fn production_attachment_inspect_reports_generated_scope() {
        let fronted = verbose_parse("Otherwise, draw a card.");
        assert!(
            fronted.contains(" clause_sentence_adverbial_before backend=chart form=0"),
            "{fronted}",
        );

        let subordinate =
            verbose_parse("If you control a Plains, creatures you control get +1/+1.");
        assert!(
            subordinate.contains(" clause_subordinate_before backend=chart form=0 evidence=feature:finite subordinate selection and host eligibility"),
            "{subordinate}",
        );

        let restriction = verbose_parse("Activate only as a sorcery and only once each turn.");
        for construction in ["clause_restriction_member", "clause_restriction_run"] {
            assert!(
                restriction.contains(&format!(" {construction} backend=chart ")),
                "missing attachment construction {construction}:\n{restriction}",
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
        // Inspect is the public provenance surface and must identify both the
        // root and its noun phrase through their chart constructions.
        assert!(verbose.contains("sentence backend=chart"));
        assert!(verbose.contains("noun backend=chart"));
        assert!(
            !verbose.contains(" owner="),
            "generated-only provenance has no migration ownership field: {verbose}"
        );
        assert!(verbose.contains("cost={opaque_words:"));
        assert!(!verbose.contains("Span {"));
        assert!(!verbose.contains("ChartStats"));
        assert!(!verbose.contains("ForestStats"));
    }

    #[test]
    fn verbose_output_distinguishes_known_and_opaque_noun_constructions() {
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
                "noun backend=chart form=0 evidence=structural:generated production reason=unique cost={opaque_words:0,opaque_lexemes:0"
            ),
            "{known}"
        );
        assert!(!known.contains("noun_opaque backend="));

        let opaque = render("Draw a BlOrPlE.");
        assert!(
            opaque.contains(
                "noun_opaque backend=chart form=0 evidence=structural:generated production reason=unique cost={opaque_words:1,opaque_lexemes:1"
            ),
            "{opaque}"
        );
        assert!(
            !opaque
                .lines()
                .any(|line| line.contains(" noun backend=chart "))
        );
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
            verbose.contains("sentence backend=chart form=1"),
            "{verbose}"
        );
        assert!(verbose.contains("QuotedAbility"));
        assert!(verbose.contains("alternative sentence#1"));
    }

    fn verbose_parse(source: &str) -> String {
        let cards = [CardFace {
            card_name: "Test Card".to_owned(),
            face_name: None,
            is_legendary: false,
            supported: false,
            source_text: source.to_owned(),
            oracle_text: source.to_owned(),
        }];
        let catalogs = inspect_fixture_catalogs();
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

    fn inspect_fixture_catalogs() -> Catalogs {
        Catalogs::default()
            .with_catalog(
                deckmaste_english::CatalogKind::KeywordAbility,
                [
                    "First strike",
                    "Flying",
                    "Haste",
                    "Lifelink",
                    "Protection",
                    "Trample",
                    "Vigilance",
                    "Ward",
                ],
            )
            .with_catalog(
                deckmaste_english::CatalogKind::CreatureType,
                ["Ally", "Alien", "Elf", "Mutant", "Ninja", "Orc", "Turtle"],
            )
            .with_catalog(
                deckmaste_english::CatalogKind::CardType,
                [
                    "Artifact",
                    "Creature",
                    "Enchantment",
                    "Instant",
                    "Land",
                    "Sorcery",
                ],
            )
    }

    fn assert_verbose_dominance(source: &str, winner: &str, loser: &str) {
        let rendered = verbose_parse(source);
        let lines = rendered.lines().collect::<Vec<_>>();
        let selected = lines
            .iter()
            .position(|line| {
                line.contains(&format!(" {winner} backend=chart "))
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
    fn verbose_output_explains_every_dominance_edge() {
        // Each fixture names the competing construction pair. Removing an
        // edge, selecting a different construction, or omitting the defeated
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
        let reduced = verbose_parse(
            "If a creature dealt damage this way would die this turn, exile it instead.",
        );
        assert!(
            reduced.contains(
                "nominal_reduced_recipient_passive backend=chart form=0 evidence=guard:reduced-recipient-passive frame"
            ),
            "{reduced}"
        );
    }

    #[test]
    fn verbose_output_reports_the_production_predicate_construction() {
        // Mutations caught: format construction provenance only for isolated
        // English fixtures, or bypass the production registry in xtask.
        let rendered = verbose_parse("You may have this creature enter.");
        assert!(
            rendered.contains(
                "verb_phrase_causative backend=chart form=0 evidence=role:causative host-causee-complement order"
            ),
            "{rendered}"
        );
    }

    #[test]
    fn verbose_output_formats_a_declaration_evidence_value() {
        let keyword = verbose_parse("This creature has protection from red and from blue.");
        assert!(
            keyword.contains(
                "predicated_argument_from_extend backend=chart form=0 evidence=guard:keyword-grant conjunction gate value=conjunction=And;allowed=[And];matched=true"
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
        ignore = "needs data/derived/cards.jsonl and data/gen/catalogs-legacy; run `cargo xtask catalogs text`"
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
        ignore = "needs data/derived/cards.jsonl and data/gen/catalogs-legacy; run `cargo xtask catalogs text`"
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
        normalize_sentence_case(&normalize_loyalty_minus(&normalize_roll_row_dashes(
            &normalize_typographic_quotes(&strip_reminder_text(text)),
        )))
    }
}
