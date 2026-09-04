//! Bounded, opt-in diagnostics for failed chart nonterminal parses.
//!
//! Production parsing does not construct these values. The recovery audit
//! calls this entry point only after a normal parse has selected recovery.

use super::Catalogs;
use super::EnglishGrammar;
use super::EnglishLexicalSlot;
use super::Features;
use super::ForestSymbol;
use super::Nonterminal;
use super::OpacityMode;
use super::RegistrationOrder;
use super::SelfReference;
use super::collapse_full_names;
use super::lex;
use super::reduction::GeneratedRejection;
use crate::chart::ChartRejection;
use crate::chart::DiagnosticLimits as ChartDiagnosticLimits;
use crate::chart::GrammarError;
use crate::chart::parse_chart_diagnostic;
use crate::chart::parse_chart_lattice;
use crate::surface::Punctuation;
use crate::surface::Token;
use crate::surface::TokenKind;

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, Ord, PartialOrd)]
pub enum FailureCategory {
    Clause,
    Sentence,
}

impl FailureCategory {
    const fn nonterminal(self) -> Nonterminal {
        match self {
            Self::Clause => Nonterminal::Clause,
            Self::Sentence => Nonterminal::Sentence,
        }
    }
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub struct DiagnosticLimits {
    /// Maximum distinct chart rejection events retained for causal ranking.
    pub max_events: usize,
    /// Maximum maximal exact constituents retained in the reported frontier.
    pub max_frontier: usize,
    /// Maximum chart states processed by the all-position recognition lattice.
    pub max_lattice_states: usize,
}

impl Default for DiagnosticLimits {
    fn default() -> Self {
        Self {
            max_events: 256,
            max_frontier: 64,
            max_lattice_states: 250_000,
        }
    }
}

#[derive(Debug, Clone, PartialEq, Eq, Hash, Ord, PartialOrd)]
pub enum FailureStage {
    Scan,
    Admission,
    Reduction,
    NoCandidate,
}

#[derive(Debug, Clone, PartialEq, Eq, Hash, Ord, PartialOrd)]
pub struct FeatureState {
    pub kind: &'static str,
    pub state: String,
}

#[derive(Debug, Clone, PartialEq, Eq, Hash, Ord, PartialOrd)]
pub struct FrontierConstituent {
    pub category: String,
    pub start_token: usize,
    pub end_token: usize,
    pub features: FeatureState,
    pub producers: Vec<(String, u16)>,
}

#[derive(Debug, Clone, PartialEq, Eq, Hash, Ord, PartialOrd)]
pub struct UncoveredBoundary {
    pub start_token: usize,
    pub end_token: usize,
}

#[derive(Debug, Clone, PartialEq, Eq, Hash, Ord, PartialOrd)]
pub struct NearDeclaration {
    pub stage: FailureStage,
    pub construction: String,
    pub form_ordinal: u16,
    pub category: String,
    pub origin: usize,
    pub position: usize,
    pub progress: usize,
    pub expected: Option<String>,
    pub prefix_features: Vec<FeatureState>,
    pub child_features: Option<FeatureState>,
    pub reason: String,
}

#[derive(Debug, Clone, PartialEq, Eq, Hash, Ord, PartialOrd)]
pub struct FailureClusterKey {
    pub requested_category: FailureCategory,
    pub stage: FailureStage,
    pub construction: Option<String>,
    pub form_ordinal: Option<u16>,
    pub category: Option<String>,
    pub expected: Option<String>,
    pub feature_shape: Vec<FeatureState>,
    pub reason: String,
    pub boundary_shape: Vec<String>,
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct FailureFingerprint {
    pub requested_category: FailureCategory,
    pub token_count: usize,
    pub frontier: Vec<FrontierConstituent>,
    pub uncovered: Vec<UncoveredBoundary>,
    pub near_declarations: Vec<NearDeclaration>,
    cluster_key: FailureClusterKey,
}

impl FailureFingerprint {
    #[must_use]
    pub const fn cluster_key(&self) -> &FailureClusterKey {
        &self.cluster_key
    }
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub enum FingerprintStatus {
    /// The trace was complete and exposed either an uncovered token boundary
    /// or a rejection aligned with an exact constituent span.
    Complete(FailureFingerprint),
    /// The diagnostic completed without truncation, but did not expose a
    /// mechanically defensible single causal seam. This includes failures
    /// below a complete chart root, full roots assembled only by independently
    /// seeded lattice states, and traces with no aligned rejection event.
    Declined {
        partial: FailureFingerprint,
        reason: &'static str,
    },
    /// One or more requested bounds truncated the evidence. A capped result
    /// is never eligible for clustering.
    Capped {
        partial: FailureFingerprint,
        dropped_events: usize,
        dropped_frontier: usize,
    },
}

impl FingerprintStatus {
    /// A capped trace is intentionally not clusterable: its omitted best
    /// candidate might change the causal key.
    #[must_use]
    pub const fn cluster_key(&self) -> Option<&FailureClusterKey> {
        match self {
            Self::Complete(fingerprint) => Some(fingerprint.cluster_key()),
            Self::Declined { .. } | Self::Capped { .. } => None,
        }
    }
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct FailureDiagnosticError {
    pub message: String,
}

/// Diagnoses one exact Clause or Sentence chart parse. This never retries
/// under opaque-noun mode and never changes the production parser's path.
///
/// # Errors
///
/// Returns an error when the generated grammar is invalid or chart parsing
/// fails for a reason other than the explicitly reported diagnostic caps.
pub fn diagnose_nonterminal_failure(
    source: &str,
    catalogs: &Catalogs,
    category: FailureCategory,
    limits: DiagnosticLimits,
) -> Result<FingerprintStatus, FailureDiagnosticError> {
    diagnose_with_identity_and_order(
        source,
        catalogs,
        category,
        limits,
        SelfReference::default(),
        RegistrationOrder::Normal,
    )
}

/// Identity-aware form of [`diagnose_nonterminal_failure`].
///
/// # Errors
///
/// Returns an error when the generated grammar is invalid or chart parsing
/// fails for a reason other than the explicitly reported diagnostic caps.
pub fn diagnose_nonterminal_failure_with_identity(
    source: &str,
    catalogs: &Catalogs,
    category: FailureCategory,
    limits: DiagnosticLimits,
    name: &str,
    is_legendary: bool,
) -> Result<FingerprintStatus, FailureDiagnosticError> {
    diagnose_with_identity_and_order(
        source,
        catalogs,
        category,
        limits,
        SelfReference::new(name, is_legendary),
        RegistrationOrder::Normal,
    )
}

#[allow(
    clippy::too_many_lines,
    reason = "the bounded audit entry assembles one inspectable fingerprint in causal order"
)]
fn diagnose_with_identity_and_order(
    source: &str,
    catalogs: &Catalogs,
    category: FailureCategory,
    limits: DiagnosticLimits,
    self_reference: SelfReference,
    order: RegistrationOrder,
) -> Result<FingerprintStatus, FailureDiagnosticError> {
    let surface = lex(source);
    let tokens = collapse_full_names(source, surface.tokens, self_reference.full_name());
    let grammar = EnglishGrammar::with_opacity_mode_and_registration_order(
        source,
        catalogs,
        category.nonterminal(),
        OpacityMode::Exact,
        self_reference,
        order,
        super::generated::GeneratedActivation::Production,
    );
    let traced = parse_chart_diagnostic(
        &grammar,
        &tokens,
        ChartDiagnosticLimits {
            max_events: limits.max_events,
        },
    )
    .map_err(|error| FailureDiagnosticError {
        message: format!("{error:?}"),
    })?;
    let lattice = match parse_chart_lattice(&grammar, &tokens, limits.max_lattice_states) {
        Ok(lattice) => lattice,
        Err(GrammarError::StateLimitExceeded { .. }) => {
            return Ok(FingerprintStatus::Capped {
                partial: empty_fingerprint(category, tokens.len(), "lattice_state_limit"),
                dropped_events: 0,
                dropped_frontier: 1,
            });
        }
        Err(error) => {
            return Err(FailureDiagnosticError {
                message: format!("{error:?}"),
            });
        }
    };

    let mut frontier = lattice
        .forest
        .nodes()
        .filter_map(|node| {
            let ForestSymbol::Nonterminal(nonterminal) = node.key.symbol else {
                return None;
            };
            let features = node.key.constituent_features()?;
            let mut producers = node
                .alternatives
                .iter()
                .filter_map(|alternative| alternative.production)
                .map(|production| {
                    (
                        production.construction.as_str().to_owned(),
                        production.ordinal,
                    )
                })
                .collect::<Vec<_>>();
            producers.sort();
            producers.dedup();
            Some(FrontierConstituent {
                category: nonterminal_name(nonterminal),
                start_token: node.key.start,
                end_token: node.key.end,
                features: feature_state(features),
                producers,
            })
        })
        .collect::<Vec<_>>();
    frontier.sort();
    frontier.dedup();
    let all_frontier = frontier.clone();
    frontier.retain(|candidate| {
        !all_frontier.iter().any(|other| {
            other.start_token <= candidate.start_token
                && candidate.end_token <= other.end_token
                && (other.start_token < candidate.start_token
                    || candidate.end_token < other.end_token)
        })
    });
    frontier.sort_by_key(|item| {
        (
            item.start_token,
            usize::MAX - item.end_token,
            item.category.clone(),
            item.features.clone(),
        )
    });
    let lattice_has_synthetic_full_root = frontier.iter().any(|constituent| {
        constituent.category
            == match category {
                FailureCategory::Clause => "Clause",
                FailureCategory::Sentence => "Sentence",
            }
            && constituent.start_token == 0
            && constituent.end_token == tokens.len()
    }) && !traced.has_complete_root;
    let dropped_frontier = frontier
        .len()
        .saturating_sub(limits.max_frontier)
        .max(usize::from(limits.max_frontier == 0 && !tokens.is_empty()));
    frontier.truncate(limits.max_frontier);

    let uncovered = uncovered_boundaries(&tokens, &frontier);
    let boundary_shape = uncovered_token_shape(&tokens, &uncovered);
    let dropped_events = traced.dropped_rejections;
    let mut near_declarations = traced
        .rejections
        .into_iter()
        .map(near_declaration)
        .collect::<Vec<_>>();
    near_declarations.sort_by(|left, right| {
        right
            .position
            .cmp(&left.position)
            .then_with(|| right.progress.cmp(&left.progress))
            .then_with(|| left.cmp(right))
    });
    near_declarations.dedup();
    near_declarations.truncate(limits.max_events);
    let causal_candidate = near_declarations.iter().find(|candidate| {
        near_declaration_aligns_with_frontier(Some(candidate), &frontier, tokens.len())
    });
    let has_causal_candidate = causal_candidate.is_some();

    let cluster_key = causal_candidate.map_or_else(
        || FailureClusterKey {
            requested_category: category,
            stage: FailureStage::NoCandidate,
            construction: None,
            form_ordinal: None,
            category: None,
            expected: None,
            feature_shape: frontier
                .iter()
                .map(|constituent| constituent.features.clone())
                .collect(),
            reason: "no_registered_candidate".to_owned(),
            boundary_shape: boundary_shape.clone(),
        },
        |candidate| FailureClusterKey {
            requested_category: category,
            stage: candidate.stage.clone(),
            construction: Some(candidate.construction.clone()),
            form_ordinal: Some(candidate.form_ordinal),
            category: Some(candidate.category.clone()),
            expected: candidate.expected.clone(),
            feature_shape: candidate
                .prefix_features
                .iter()
                .chain(candidate.child_features.iter())
                .map(cluster_feature_state)
                .collect(),
            reason: candidate.reason.clone(),
            boundary_shape: boundary_shape.clone(),
        },
    );
    let cluster_key = if boundary_shape.is_empty() {
        cluster_key
    } else {
        let word_boundary = boundary_shape.iter().any(|token| token == "word_run");
        FailureClusterKey {
            requested_category: category,
            stage: FailureStage::NoCandidate,
            construction: None,
            form_ordinal: None,
            category: (!word_boundary)
                .then(|| composition_category_shape(&frontier, &uncovered, &boundary_shape))
                .flatten(),
            expected: None,
            feature_shape: Vec::new(),
            reason: "uncovered_boundary".to_owned(),
            boundary_shape,
        }
    };
    let has_uncovered = !uncovered.is_empty();
    let fingerprint = FailureFingerprint {
        requested_category: category,
        token_count: tokens.len(),
        frontier,
        uncovered,
        near_declarations,
        cluster_key,
    };
    if dropped_events == 0 && dropped_frontier == 0 {
        if traced.has_complete_root {
            Ok(FingerprintStatus::Declined {
                partial: fingerprint,
                reason: "lowering_not_traced",
            })
        } else if lattice_has_synthetic_full_root {
            Ok(FingerprintStatus::Declined {
                partial: fingerprint,
                reason: "lattice_cross_seed_dependency",
            })
        } else if !has_uncovered && !has_causal_candidate {
            Ok(FingerprintStatus::Declined {
                partial: fingerprint,
                reason: "unaligned_near_declaration",
            })
        } else {
            Ok(FingerprintStatus::Complete(fingerprint))
        }
    } else {
        Ok(FingerprintStatus::Capped {
            partial: fingerprint,
            dropped_events,
            dropped_frontier,
        })
    }
}

fn empty_fingerprint(
    category: FailureCategory,
    token_count: usize,
    reason: &str,
) -> FailureFingerprint {
    FailureFingerprint {
        requested_category: category,
        token_count,
        frontier: Vec::new(),
        uncovered: Vec::new(),
        near_declarations: Vec::new(),
        cluster_key: FailureClusterKey {
            requested_category: category,
            stage: FailureStage::NoCandidate,
            construction: None,
            form_ordinal: None,
            category: None,
            expected: None,
            feature_shape: Vec::new(),
            reason: reason.to_owned(),
            boundary_shape: Vec::new(),
        },
    }
}

fn near_declaration_aligns_with_frontier(
    candidate: Option<&NearDeclaration>,
    frontier: &[FrontierConstituent],
    token_count: usize,
) -> bool {
    let Some(candidate) = candidate else {
        return false;
    };
    if candidate.stage == FailureStage::Scan
        && candidate
            .expected
            .as_deref()
            .is_some_and(|expected| expected.starts_with("lexical:Punctuation("))
        && frontier.iter().any(|constituent| {
            constituent.category == candidate.category
                && constituent.start_token == candidate.origin
                && constituent.end_token == candidate.position
        })
    {
        // A completed constituent does not fail merely because one of its
        // optional continuation productions cannot scan another delimiter.
        return false;
    }
    if candidate.stage == FailureStage::Scan
        && candidate
            .expected
            .as_deref()
            .is_some_and(|expected| expected.starts_with("lexical:Punctuation("))
        && frontier.iter().any(|constituent| {
            constituent.start_token < candidate.position
                && candidate.position < constituent.end_token
        })
    {
        // A failed extension inside a larger exact constituent is not a
        // boundary of the best available cover and therefore cannot explain
        // the whole-phrase failure.
        return false;
    }
    if candidate.stage == FailureStage::Scan
        && candidate.progress > 0
        && frontier.iter().any(|constituent| {
            constituent.start_token == candidate.origin
                && constituent.end_token == candidate.position
        })
    {
        // A construction may try to extend an already complete constituent.
        // Failure to scan that optional continuation does not explain why the
        // containing phrase failed.
        return false;
    }
    candidate.origin == 0 && candidate.position == token_count
        || frontier.iter().any(|constituent| {
            constituent.start_token == candidate.origin
                && constituent.end_token == candidate.position
        })
}

fn near_declaration(
    rejection: ChartRejection<Nonterminal, EnglishLexicalSlot, Features, GeneratedRejection>,
) -> NearDeclaration {
    match rejection {
        ChartRejection::ScanMiss {
            production,
            lhs,
            origin,
            position,
            dot,
            slot,
            prefix_features,
        } => NearDeclaration {
            stage: FailureStage::Scan,
            construction: production.construction.as_str().to_owned(),
            form_ordinal: production.ordinal,
            category: nonterminal_name(lhs),
            origin,
            position,
            progress: dot,
            expected: Some(format!("lexical:{slot:?}")),
            prefix_features: prefix_features.iter().map(feature_state).collect(),
            child_features: None,
            reason: "scan_miss".to_owned(),
        },
        ChartRejection::Prefix {
            production,
            lhs,
            origin,
            child_start: _,
            position,
            dot,
            prefix_features,
            child_features,
            reason,
        } => NearDeclaration {
            stage: FailureStage::Admission,
            construction: production.construction.as_str().to_owned(),
            form_ordinal: production.ordinal,
            category: nonterminal_name(lhs),
            origin,
            position,
            progress: dot,
            expected: None,
            prefix_features: prefix_features.iter().map(feature_state).collect(),
            child_features: Some(feature_state(&child_features)),
            reason: rejection_name(reason),
        },
        ChartRejection::Reduction {
            production,
            lhs,
            origin,
            position,
            prefix_features,
            reason,
        } => NearDeclaration {
            stage: FailureStage::Reduction,
            construction: production.construction.as_str().to_owned(),
            form_ordinal: production.ordinal,
            category: nonterminal_name(lhs),
            origin,
            position,
            progress: prefix_features.len(),
            expected: None,
            prefix_features: prefix_features.iter().map(feature_state).collect(),
            child_features: None,
            reason: rejection_name(reason),
        },
    }
}

fn rejection_name(rejection: GeneratedRejection) -> String {
    match rejection {
        GeneratedRejection::MissingDeclaration => "missing_declaration".to_owned(),
        GeneratedRejection::ChildAssembly => "child_assembly".to_owned(),
        GeneratedRejection::Requirement { index } => format!("requirement:{index}"),
        GeneratedRejection::SurfaceSequence => "surface_sequence".to_owned(),
        GeneratedRejection::FeatureProjection => "feature_projection".to_owned(),
        GeneratedRejection::FeatureCombination => "feature_combination".to_owned(),
        GeneratedRejection::ContextProjection => "context_projection".to_owned(),
        GeneratedRejection::PrefixRequirement { index } => {
            format!("prefix_requirement:{index}")
        }
        GeneratedRejection::PrefixAdmission => "prefix_admission".to_owned(),
        GeneratedRejection::AuxiliaryReduction => "auxiliary_reduction".to_owned(),
    }
}

fn uncovered_boundaries(
    tokens: &[Token],
    frontier: &[FrontierConstituent],
) -> Vec<UncoveredBoundary> {
    let token_count = tokens.len();
    let mut covered = vec![false; token_count];
    for constituent in frontier {
        for covered in covered
            .iter_mut()
            .take(constituent.end_token.min(token_count))
            .skip(constituent.start_token)
        {
            *covered = true;
        }
    }
    let mut uncovered = Vec::new();
    let mut start = None;
    for (index, covered) in covered.into_iter().chain([true]).enumerate() {
        match (start, covered) {
            (None, false) => start = Some(index),
            (Some(begin), true) => {
                uncovered.push(UncoveredBoundary {
                    start_token: begin,
                    end_token: index,
                });
                start = None;
            }
            _ => {}
        }
    }
    uncovered.retain(|boundary| {
        !(boundary.end_token == token_count
            && boundary.start_token + 1 == boundary.end_token
            && matches!(
                tokens[boundary.start_token].kind,
                TokenKind::Punctuation(Punctuation::Period)
            ))
    });
    uncovered
}

fn uncovered_token_shape(tokens: &[Token], uncovered: &[UncoveredBoundary]) -> Vec<String> {
    let raw = uncovered
        .iter()
        .flat_map(|boundary| &tokens[boundary.start_token..boundary.end_token])
        .map(|token| match token.kind {
            TokenKind::Punctuation(punctuation) => punctuation_name(punctuation),
            TokenKind::OracleSymbol => "oracle_symbol".to_owned(),
            TokenKind::SymbolSequence => "symbol_sequence".to_owned(),
            TokenKind::PowerToughness => "power_toughness".to_owned(),
            TokenKind::Bullet => "bullet".to_owned(),
            TokenKind::Newline => "newline".to_owned(),
            TokenKind::Integer => "integer".to_owned(),
            TokenKind::Word | TokenKind::FullSelfReference => "word".to_owned(),
        })
        .collect::<Vec<_>>();
    if raw.iter().any(|token| token == "open_bracket")
        && raw.iter().any(|token| token == "close_bracket")
    {
        return vec!["open_bracket".to_owned(), "close_bracket".to_owned()];
    }
    let mut collapsed = Vec::new();
    for token in raw {
        if token == "word" && collapsed.last().is_some_and(|last| last == "word_run") {
            continue;
        }
        collapsed.push(if token == "word" { "word_run".to_owned() } else { token });
    }
    collapsed
}

fn punctuation_name(punctuation: Punctuation) -> String {
    match punctuation {
        Punctuation::Comma => "comma",
        Punctuation::Colon => "colon",
        Punctuation::Semicolon => "semicolon",
        Punctuation::Period => "period",
        Punctuation::Exclamation => "exclamation",
        Punctuation::Question => "question",
        Punctuation::Apostrophe => "apostrophe",
        Punctuation::DoubleQuote => "double_quote",
        Punctuation::OpenBracket => "open_bracket",
        Punctuation::CloseBracket => "close_bracket",
        Punctuation::OpenParenthesis => "open_parenthesis",
        Punctuation::CloseParenthesis => "close_parenthesis",
        Punctuation::Plus => "plus",
        Punctuation::Minus => "minus",
        Punctuation::Slash => "slash",
        Punctuation::Hyphen => "hyphen",
        Punctuation::EnDash => "en_dash",
        Punctuation::EmDash => "em_dash",
        Punctuation::Other(_) => "other_punctuation",
    }
    .to_owned()
}

fn composition_category_shape(
    frontier: &[FrontierConstituent],
    uncovered: &[UncoveredBoundary],
    boundary_shape: &[String],
) -> Option<String> {
    if boundary_shape == ["open_bracket", "close_bracket"]
        && frontier.iter().any(|item| item.category == "Sentence")
    {
        return Some("Sentence".to_owned());
    }
    let boundary = uncovered.first()?;
    let mut categories = frontier
        .iter()
        .filter(|constituent| {
            constituent.category != "Generated"
                && (constituent.end_token == boundary.start_token
                    || constituent.start_token == boundary.end_token)
        })
        .map(|constituent| constituent.category.as_str())
        .collect::<Vec<_>>();
    categories.sort_unstable();
    categories.dedup();
    (!categories.is_empty()).then(|| categories.join("+"))
}

fn cluster_feature_state(state: &FeatureState) -> FeatureState {
    let relevant = match state.kind {
        "Nominal" => state
            .state
            .split(';')
            .find(|field| field.starts_with("attachment="))
            .unwrap_or_default()
            .to_owned(),
        "RelativeClause" => state
            .state
            .split(';')
            .filter(|field| field.starts_with("gap=") || field.starts_with("marker="))
            .collect::<Vec<_>>()
            .join(";"),
        "VerbPhrase" => state
            .state
            .split(';')
            .filter(|field| {
                field.starts_with("form=")
                    || field.starts_with("object=")
                    || field.starts_with("phase=")
            })
            .collect::<Vec<_>>()
            .join(";"),
        _ => state.state.clone(),
    };
    FeatureState {
        kind: state.kind,
        state: relevant,
    }
}

fn nonterminal_name(nonterminal: Nonterminal) -> String {
    match nonterminal {
        // Internal generated-category indices are assembly-local. They are
        // retained in the full frontier for human diagnosis, but erased from
        // the canonical cluster identity below.
        Nonterminal::Generated(_) => "Generated".to_owned(),
        other => format!("{other:?}"),
    }
}

fn feature_state(features: &Features) -> FeatureState {
    let (kind, state) = match features {
        Features::Nominal {
            form,
            determined,
            modified,
            leading_opacity,
            attachment,
            opaque_head,
            ..
        } => (
            "Nominal",
            format!(
                "form={};det={determined};mod={modified};leading_opaque={leading_opacity};attachment={};opaque_head={opaque_head}",
                noun_form_name(*form),
                nominal_attachment_name(*attachment),
            ),
        ),
        Features::NounPhrase {
            agreement,
            coordination_domain,
            pronoun_case,
            set_exception,
            coordination,
            recipient_passive_theme,
            rules_object_followup,
            ..
        } => (
            "NounPhrase",
            format!(
                "agreement={};domain={};case={};exception={};coord={};recipient_theme={recipient_passive_theme};rules_followup={rules_object_followup}",
                agreement.map_or("none".to_owned(), agreement_name),
                coordination_domain.map_or("none", coordination_domain_name),
                pronoun_case.map_or("none", pronoun_case_name),
                set_exception_name(*set_exception),
                coordination_name(*coordination),
            ),
        ),
        Features::VerbPhrase {
            form,
            passive,
            dependent_count,
            object,
            indirect_object,
            selected_preposition,
            phase,
            frame: _,
            bare,
            head_is_copular,
            object_gap_requires_rules_object,
            subjunctive,
        } => (
            "VerbPhrase",
            format!(
                "form={};passive={passive};deps={dependent_count};object={};indirect={indirect_object};selected_pp={selected_preposition};phase={};bare={bare};copular={head_is_copular};rules_gap={object_gap_requires_rules_object};subj={subjunctive}",
                predicate_form_name(*form),
                predicate_object_name(*object),
                predicate_phase_name(*phase),
            ),
        ),
        Features::SimpleClause {
            agreement,
            has_subject,
            standalone,
            has_direct_object,
            host_addressee_subject,
            host_modal,
            subjunctive,
        } => (
            "SimpleClause",
            format!(
                "agreement={};subject={has_subject};standalone={standalone};object={has_direct_object};addressee={host_addressee_subject};modal={host_modal};subj={subjunctive}",
                agreement.map_or("none".to_owned(), agreement_name),
            ),
        ),
        Features::Clause {
            agreement,
            standalone,
            finite,
            host_addressee_subject,
            host_modal,
            subjunctive,
        } => (
            "Clause",
            format!(
                "agreement={};standalone={standalone};finite={finite};addressee={host_addressee_subject};modal={host_modal};subj={subjunctive}",
                agreement.map_or("none".to_owned(), agreement_name),
            ),
        ),
        Features::RelativeClause {
            gap,
            marker,
            antecedent_agreement,
            distributive_each,
            copular,
            object_gap_requires_rules_object,
            bare_copular_tail,
            ..
        } => (
            "RelativeClause",
            format!(
                "gap={};marker={};agreement={};each={distributive_each};copular={};rules_gap={object_gap_requires_rules_object};bare_copula={bare_copular_tail}",
                gap_name(*gap),
                relative_marker_name(*marker),
                antecedent_agreement.map_or("none".to_owned(), agreement_name),
                relative_copular_name(*copular),
            ),
        ),
        Features::PrepositionalPhrase {
            preposition,
            nominal_attachment,
            role_members,
            shared_determiner_object,
            nearer_relative_host,
        } => (
            "PrepositionalPhrase",
            format!(
                "prep={};nominal={nominal_attachment};roles={};shared_det={shared_determiner_object};nearer_relative={nearer_relative_host}",
                preposition.spelling(),
                role_members.len()
            ),
        ),
        Features::GeneratedSequence { .. } => ("GeneratedSequence", String::new()),
        other => (feature_kind(other), String::new()),
    };
    FeatureState { kind, state }
}

fn feature_kind(features: &Features) -> &'static str {
    match features {
        Features::None => "None",
        Features::WithAttributeMember { .. } => "WithAttributeMember",
        Features::WithAttributeList { .. } => "WithAttributeList",
        Features::Number { .. } => "Number",
        Features::Quantity(_) => "Quantity",
        Features::PowerToughness { .. } => "PowerToughness",
        Features::Determiner { .. } => "Determiner",
        Features::Adjective { .. } => "Adjective",
        Features::Noun { .. } => "Noun",
        Features::Nominal { .. } => "Nominal",
        Features::NounPhrase { .. } => "NounPhrase",
        Features::PossessiveThisCard { .. } => "PossessiveThisCard",
        Features::PossessiveNounPhrase { .. } => "PossessiveNounPhrase",
        Features::Verb { .. } => "Verb",
        Features::VerbPhrase { .. } => "VerbPhrase",
        Features::InfinitiveClause => "InfinitiveClause",
        Features::GerundClause => "GerundClause",
        Features::SimpleClause { .. } => "SimpleClause",
        Features::Clause { .. } => "Clause",
        Features::Sentence => "Sentence",
        Features::Preposition(_) => "Preposition",
        Features::Subordinator(_) => "Subordinator",
        Features::PrepositionalObject { .. } => "PrepositionalObject",
        Features::PrepositionalPhrase { .. } => "PrepositionalPhrase",
        Features::VerbParticle(_) => "VerbParticle",
        Features::CoinResult(_) => "CoinResult",
        Features::RelativeClause { .. } => "RelativeClause",
        Features::RelativeMarker(_) => "RelativeMarker",
        Features::Auxiliary(_) => "Auxiliary",
        Features::Conjunction(_) => "Conjunction",
        Features::Existential { .. } => "Existential",
        Features::Copula(_) => "Copula",
        Features::SubjectAuxiliary { .. } => "SubjectAuxiliary",
        Features::ExceptionRider => "ExceptionRider",
        Features::RestrictionMember => "RestrictionMember",
        Features::PredicatedQuality => "PredicatedQuality",
        Features::PredicatedArgument => "PredicatedArgument",
        Features::CoordinatedModifier { .. } => "CoordinatedModifier",
        Features::GeneratedElement { .. } => "GeneratedElement",
        Features::GeneratedSequence { .. } => "GeneratedSequence",
    }
}

const fn noun_form_name(value: super::NounForm) -> &'static str {
    match value {
        super::NounForm::Singular => "singular",
        super::NounForm::Plural => "plural",
        super::NounForm::Mass => "mass",
    }
}

const fn nominal_attachment_name(value: super::NominalAttachmentPhase) -> &'static str {
    match value {
        super::NominalAttachmentPhase::Open => "open",
        super::NominalAttachmentPhase::Prepositional {
            nearer_relative_host: false,
        } => "prepositional",
        super::NominalAttachmentPhase::Prepositional {
            nearer_relative_host: true,
        } => "prepositional_nearer_relative",
        super::NominalAttachmentPhase::Relative => "relative",
        super::NominalAttachmentPhase::RulesObjectRelative => "rules_object_relative",
        super::NominalAttachmentPhase::RelativeBareCopula => "relative_bare_copula",
        super::NominalAttachmentPhase::ReducedRecipientPassive => "reduced_recipient_passive",
        super::NominalAttachmentPhase::PostpositiveAdjective => "postpositive_adjective",
        super::NominalAttachmentPhase::Comparison => "comparison",
    }
}

fn agreement_name(value: super::PersonNumber) -> String {
    format!(
        "{}_{}",
        match value.person {
            crate::features::Person::Second => "second",
            crate::features::Person::Third => "third",
        },
        match value.number {
            crate::features::Number::Singular => "singular",
            crate::features::Number::Plural => "plural",
        }
    )
}

const fn coordination_domain_name(value: super::CoordinationDomain) -> &'static str {
    match value {
        super::CoordinationDomain::Entity => "entity",
        super::CoordinationDomain::NonEntity => "non_entity",
        super::CoordinationDomain::Damage => "damage",
        super::CoordinationDomain::Power => "power",
        super::CoordinationDomain::Toughness => "toughness",
        super::CoordinationDomain::PowerToughness => "power_toughness",
        super::CoordinationDomain::SelectionHost => "selection_host",
        super::CoordinationDomain::SelectionContinuation => "selection_continuation",
    }
}

const fn pronoun_case_name(value: crate::features::PronounCase) -> &'static str {
    match value {
        crate::features::PronounCase::Subject => "subject",
        crate::features::PronounCase::Object => "object",
    }
}

const fn set_exception_name(value: super::SetExceptionState) -> &'static str {
    match value {
        super::SetExceptionState::Ineligible => "ineligible",
        super::SetExceptionState::Host => "host",
        super::SetExceptionState::Closed => "closed",
    }
}

fn coordination_name(value: super::NounPhraseCoordinationState) -> &'static str {
    match value {
        super::NounPhraseCoordinationState::None => "none",
        super::NounPhraseCoordinationState::Binary(_) => "binary",
        super::NounPhraseCoordinationState::Oxford(_) => "oxford",
        super::NounPhraseCoordinationState::Shared => "shared",
    }
}

fn predicate_form_name(value: super::PredicateForm) -> String {
    match value {
        super::PredicateForm::Imperative => "imperative".to_owned(),
        super::PredicateForm::Infinitive => "infinitive".to_owned(),
        super::PredicateForm::Finite(None) => "finite_none".to_owned(),
        super::PredicateForm::Finite(Some(agreement)) => {
            format!("finite_{}", agreement_name(agreement))
        }
        super::PredicateForm::PresentParticiple => "present_participle".to_owned(),
        super::PredicateForm::PastParticiple => "past_participle".to_owned(),
    }
}

const fn predicate_object_name(value: super::PredicateObjectState) -> &'static str {
    match value {
        super::PredicateObjectState::None => "none",
        super::PredicateObjectState::Direct => "direct",
        super::PredicateObjectState::PronominalDirect => "pronominal_direct",
        super::PredicateObjectState::Ability => "ability",
        super::PredicateObjectState::AbilityWithArgument => "ability_with_argument",
    }
}

const fn predicate_phase_name(value: super::PredicateAttachmentPhase) -> &'static str {
    match value {
        super::PredicateAttachmentPhase::Object => "object",
        super::PredicateAttachmentPhase::Tail => "tail",
        super::PredicateAttachmentPhase::PrepositionalTail => "prepositional_tail",
        super::PredicateAttachmentPhase::ExceptionTail => "exception_tail",
    }
}

const fn gap_name(value: crate::features::GapState) -> &'static str {
    match value {
        crate::features::GapState::Subject => "subject",
        crate::features::GapState::Object => "object",
    }
}

const fn relative_marker_name(value: crate::syntax::RelativeMarker) -> &'static str {
    match value {
        crate::syntax::RelativeMarker::That => "that",
        crate::syntax::RelativeMarker::Who => "who",
        crate::syntax::RelativeMarker::Zero => "zero",
    }
}

const fn relative_copular_name(value: super::RelativeCopularClass) -> &'static str {
    match value {
        super::RelativeCopularClass::NonCopular => "non_copular",
        super::RelativeCopularClass::Noun => "noun",
        super::RelativeCopularClass::Adjective => "adjective",
        super::RelativeCopularClass::Prepositional => "prepositional",
        super::RelativeCopularClass::CoordinatedAdjective => "coordinated_adjective",
    }
}
