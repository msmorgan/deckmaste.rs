use anyhow::Context;
use anyhow::Result;
use deckmaste_english::Catalogs;
use deckmaste_english::DiagnosticLimits;
use deckmaste_english::FailureCategory;
use deckmaste_english::FailureClusterKey;
use deckmaste_english::FailureFingerprint;
use deckmaste_english::FailureStage;
use deckmaste_english::FeatureState;
use deckmaste_english::FingerprintStatus;
use deckmaste_english::diagnose_nonterminal_failure_with_identity;
use deckmaste_english::syntax::RecoveryRole;
use serde::Serialize;

use crate::english::recovery_worklist::group_id;

const SCHEMA_VERSION: u32 = 1;
const NOTICE: &str = "Runtime-derived recovery fingerprints; diagnostic only, not causal dispositions. Do not commit this corpus artifact.";

#[derive(Debug, Clone, PartialEq, Eq)]
pub(super) struct RecoveryFingerprintInput {
    pub(super) role: RecoveryRole,
    pub(super) text: String,
    pub(super) occurrences: usize,
    pub(super) source_tokens: usize,
    pub(super) exemplar_face_name: String,
    pub(super) exemplar_is_legendary: bool,
}

#[derive(Serialize)]
struct FingerprintExport {
    schema_version: u32,
    notice: &'static str,
    limits: LimitsOutput,
    groups: Vec<GroupOutput>,
}

#[derive(Clone, Copy, Serialize)]
struct LimitsOutput {
    #[serde(rename = "max_events")]
    events: usize,
    #[serde(rename = "max_frontier")]
    frontier: usize,
    #[serde(rename = "max_lattice_states")]
    lattice_states: usize,
}

impl From<DiagnosticLimits> for LimitsOutput {
    fn from(limits: DiagnosticLimits) -> Self {
        Self {
            events: limits.max_events,
            frontier: limits.max_frontier,
            lattice_states: limits.max_lattice_states,
        }
    }
}

#[derive(Serialize)]
struct GroupOutput {
    group_id: String,
    role: &'static str,
    text: String,
    occurrences: usize,
    source_tokens: usize,
    exemplar: ExemplarOutput,
    #[serde(flatten)]
    diagnostics: GroupDiagnostics,
}

#[derive(Serialize)]
struct ExemplarOutput {
    face_name: String,
    is_legendary: bool,
}

#[derive(Serialize)]
#[serde(tag = "diagnostic_support", rename_all = "snake_case")]
enum GroupDiagnostics {
    Supported { attempts: Vec<AttemptOutput> },
    Unsupported { reason: &'static str },
}

#[derive(Serialize)]
struct AttemptOutput {
    category: &'static str,
    #[serde(flatten)]
    outcome: AttemptOutcome,
}

#[derive(Serialize)]
#[serde(tag = "status", rename_all = "snake_case")]
enum AttemptOutcome {
    Complete {
        trace: TraceSummary,
        cluster_key: ClusterKeyOutput,
    },
    Declined {
        trace: TraceSummary,
        reason: &'static str,
    },
    Capped {
        trace: TraceSummary,
        dropped_events: usize,
        dropped_frontier: usize,
    },
}

#[derive(Serialize)]
struct TraceSummary {
    token_count: usize,
    frontier_constituents: usize,
    uncovered_boundaries: usize,
    near_declarations: usize,
}

#[derive(Serialize)]
struct ClusterKeyOutput {
    requested_category: &'static str,
    stage: &'static str,
    construction: Option<String>,
    form_ordinal: Option<u16>,
    category: Option<String>,
    expected: Option<String>,
    feature_shape: Vec<FeatureStateOutput>,
    reason: String,
    boundary_shape: Vec<String>,
}

#[derive(Serialize)]
struct FeatureStateOutput {
    kind: &'static str,
    state: String,
}

pub(super) fn export(
    groups: &[RecoveryFingerprintInput],
    catalogs: &Catalogs,
    limits: DiagnosticLimits,
) -> Result<String> {
    let groups = groups
        .iter()
        .map(|group| diagnose_group(group, catalogs, limits))
        .collect::<Result<Vec<_>>>()?;
    serde_json::to_string_pretty(&FingerprintExport {
        schema_version: SCHEMA_VERSION,
        notice: NOTICE,
        limits: limits.into(),
        groups,
    })
    .context("could not serialize recovery fingerprints")
}

fn diagnose_group(
    group: &RecoveryFingerprintInput,
    catalogs: &Catalogs,
    limits: DiagnosticLimits,
) -> Result<GroupOutput> {
    let diagnostics = match group.role {
        RecoveryRole::Clause => GroupDiagnostics::Supported {
            attempts: [FailureCategory::Sentence, FailureCategory::Clause]
                .into_iter()
                .map(|category| diagnose_attempt(group, catalogs, category, limits))
                .collect::<Result<Vec<_>>>()?,
        },
        RecoveryRole::NominalComplement
        | RecoveryRole::ActivationCost
        | RecoveryRole::KeywordArgument
        | RecoveryRole::ModalHeader
        | RecoveryRole::EmbeddedRules => GroupDiagnostics::Unsupported {
            reason: "no_role_faithful_diagnostic_entry_point",
        },
    };
    let role = role_name(group.role);
    Ok(GroupOutput {
        group_id: group_id(role, &group.text),
        role,
        text: group.text.clone(),
        occurrences: group.occurrences,
        source_tokens: group.source_tokens,
        exemplar: ExemplarOutput {
            face_name: group.exemplar_face_name.clone(),
            is_legendary: group.exemplar_is_legendary,
        },
        diagnostics,
    })
}

fn diagnose_attempt(
    group: &RecoveryFingerprintInput,
    catalogs: &Catalogs,
    category: FailureCategory,
    limits: DiagnosticLimits,
) -> Result<AttemptOutput> {
    let status = diagnose_nonterminal_failure_with_identity(
        &group.text,
        catalogs,
        category,
        limits,
        &group.exemplar_face_name,
        group.exemplar_is_legendary,
    )
    .map_err(|error| anyhow::anyhow!(error.message))
    .with_context(|| {
        format!(
            "could not diagnose {} recovery {:?} as {} for exemplar {:?}",
            role_name(group.role),
            group.text,
            category_name(category),
            group.exemplar_face_name,
        )
    })?;
    let outcome = match status {
        FingerprintStatus::Complete(fingerprint) => AttemptOutcome::Complete {
            trace: trace_summary(&fingerprint),
            cluster_key: cluster_key_output(fingerprint.cluster_key()),
        },
        FingerprintStatus::Declined { partial, reason } => AttemptOutcome::Declined {
            trace: trace_summary(&partial),
            reason,
        },
        FingerprintStatus::Capped {
            partial,
            dropped_events,
            dropped_frontier,
        } => AttemptOutcome::Capped {
            trace: trace_summary(&partial),
            dropped_events,
            dropped_frontier,
        },
    };
    Ok(AttemptOutput {
        category: category_name(category),
        outcome,
    })
}

fn trace_summary(fingerprint: &FailureFingerprint) -> TraceSummary {
    TraceSummary {
        token_count: fingerprint.token_count,
        frontier_constituents: fingerprint.frontier.len(),
        uncovered_boundaries: fingerprint.uncovered.len(),
        near_declarations: fingerprint.near_declarations.len(),
    }
}

fn cluster_key_output(key: &FailureClusterKey) -> ClusterKeyOutput {
    ClusterKeyOutput {
        requested_category: category_name(key.requested_category),
        stage: stage_name(&key.stage),
        construction: key.construction.clone(),
        form_ordinal: key.form_ordinal,
        category: key.category.clone(),
        expected: key.expected.clone(),
        feature_shape: key.feature_shape.iter().map(feature_state_output).collect(),
        reason: key.reason.clone(),
        boundary_shape: key.boundary_shape.clone(),
    }
}

fn feature_state_output(feature: &FeatureState) -> FeatureStateOutput {
    FeatureStateOutput {
        kind: feature.kind,
        state: feature.state.clone(),
    }
}

const fn category_name(category: FailureCategory) -> &'static str {
    match category {
        FailureCategory::Clause => "clause",
        FailureCategory::Sentence => "sentence",
    }
}

const fn stage_name(stage: &FailureStage) -> &'static str {
    match stage {
        FailureStage::Scan => "scan",
        FailureStage::Admission => "admission",
        FailureStage::Reduction => "reduction",
        FailureStage::NoCandidate => "no_candidate",
    }
}

const fn role_name(role: RecoveryRole) -> &'static str {
    match role {
        RecoveryRole::Clause => "clause",
        RecoveryRole::NominalComplement => "nominal",
        RecoveryRole::ActivationCost => "activation cost",
        RecoveryRole::KeywordArgument => "keyword argument",
        RecoveryRole::ModalHeader => "modal header",
        RecoveryRole::EmbeddedRules => "embedded rules",
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    fn input(role: RecoveryRole, text: &str) -> RecoveryFingerprintInput {
        RecoveryFingerprintInput {
            role,
            text: text.to_owned(),
            occurrences: 2,
            source_tokens: 8,
            exemplar_face_name: "Diagnostic Witness".to_owned(),
            exemplar_is_legendary: true,
        }
    }

    #[test]
    fn unsupported_roles_are_explicit_and_never_receive_attempts() {
        let json = export(
            &[input(RecoveryRole::ActivationCost, "Pay a mystery cost")],
            &Catalogs::default(),
            DiagnosticLimits::default(),
        )
        .unwrap();
        let value: serde_json::Value = serde_json::from_str(&json).unwrap();
        let group = &value["groups"][0];

        assert_eq!(group["diagnostic_support"], "unsupported");
        assert_eq!(group["reason"], "no_role_faithful_diagnostic_entry_point");
        assert!(group.get("attempts").is_none());
    }

    #[test]
    fn clause_role_preserves_sentence_and_clause_attempts_independently() {
        let limits = DiagnosticLimits {
            max_events: 0,
            max_frontier: 0,
            max_lattice_states: 0,
        };
        let json = export(
            &[input(RecoveryRole::Clause, "You frobnitz a card.")],
            &Catalogs::default(),
            limits,
        )
        .unwrap();
        let value: serde_json::Value = serde_json::from_str(&json).unwrap();
        let group = &value["groups"][0];
        let attempts = group["attempts"].as_array().unwrap();

        assert_eq!(group["diagnostic_support"], "supported");
        assert_eq!(attempts.len(), 2);
        assert_eq!(attempts[0]["category"], "sentence");
        assert_eq!(attempts[1]["category"], "clause");
        assert_eq!(attempts[0]["status"], "capped");
        assert_eq!(attempts[1]["status"], "capped");
        assert!(attempts[0].get("cluster_key").is_none());
        assert!(attempts[1].get("cluster_key").is_none());
    }

    #[test]
    fn clause_attempts_use_the_exemplar_face_identity() {
        let json = export(
            &[input(
                RecoveryRole::Clause,
                "Diagnostic Witness frobnitzes.",
            )],
            &Catalogs::default(),
            DiagnosticLimits {
                max_events: 0,
                max_frontier: 0,
                max_lattice_states: 0,
            },
        )
        .unwrap();
        let value: serde_json::Value = serde_json::from_str(&json).unwrap();
        let attempts = value["groups"][0]["attempts"].as_array().unwrap();

        for attempt in attempts {
            assert_eq!(attempt["trace"]["token_count"], 3);
        }
    }
}
