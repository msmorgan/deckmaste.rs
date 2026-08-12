use std::collections::BTreeMap;
use std::fs;
use std::path::Path;

use anyhow::Context;
use anyhow::Result;
use anyhow::bail;
use serde::Deserialize;
use serde::Serialize;
use sha2::Digest;
use sha2::Sha256;

const SCHEMA_VERSION: u32 = 1;
const NOTICE: &str = "Runtime-derived recovery audit; do not commit this corpus worklist.";

#[derive(Debug, Clone, PartialEq, Eq)]
pub(super) struct RuntimeRecoveryGroup {
    pub(super) role: String,
    pub(super) text: String,
    pub(super) occurrences: usize,
    pub(super) source_tokens: usize,
    pub(super) face_names: Vec<String>,
}

#[derive(Debug, Clone, PartialEq, Eq, Deserialize, Serialize)]
#[serde(deny_unknown_fields)]
struct RecoveryWorklist {
    schema_version: u32,
    notice: String,
    groups: Vec<RecoveryWorklistGroup>,
}

#[derive(Debug, Clone, PartialEq, Eq, Deserialize, Serialize)]
#[serde(deny_unknown_fields)]
struct RecoveryWorklistGroup {
    group_id: String,
    role: String,
    text: String,
    structural_signature: String,
    lexical_signature: String,
    occurrences: usize,
    source_tokens: usize,
    face_names: Vec<String>,
    disposition: Option<Disposition>,
}

#[derive(Debug, Clone, PartialEq, Eq, Deserialize, Serialize)]
#[serde(deny_unknown_fields)]
struct Disposition {
    cause: CausalCause,
    resolution: Resolution,
    rationale: String,
    #[serde(default, skip_serializing_if = "Vec::is_empty")]
    evidence: Vec<String>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    follow_up_ticket: Option<String>,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Deserialize, Serialize)]
#[serde(rename_all = "snake_case")]
enum CausalCause {
    Composition,
    FeatureTransition,
    Admission,
    Lowering,
    UnsupportedGrammar,
    UnsupportedLexicon,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Deserialize, Serialize)]
#[serde(rename_all = "snake_case")]
enum Resolution {
    Implemented,
    FollowUp,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub(super) struct VerificationSummary {
    pub(super) audited: usize,
    pub(super) implemented: usize,
    pub(super) follow_up: usize,
    pub(super) current: usize,
}

pub(super) fn export(groups: &[RuntimeRecoveryGroup]) -> Result<String> {
    let worklist = RecoveryWorklist {
        schema_version: SCHEMA_VERSION,
        notice: NOTICE.to_owned(),
        groups: groups
            .iter()
            .map(|group| RecoveryWorklistGroup {
                group_id: group_id(&group.role, &group.text),
                role: group.role.clone(),
                text: group.text.clone(),
                structural_signature: structural_signature(&group.role, &group.text),
                lexical_signature: lexical_signature(&group.text),
                occurrences: group.occurrences,
                source_tokens: group.source_tokens,
                face_names: group.face_names.clone(),
                disposition: None,
            })
            .collect(),
    };
    serde_json::to_string_pretty(&worklist).context("could not serialize recovery worklist")
}

pub(super) fn verify(
    path: &Path,
    current_groups: &[RuntimeRecoveryGroup],
) -> Result<VerificationSummary> {
    let bytes = fs::read(path)
        .with_context(|| format!("could not read recovery worklist {}", path.display()))?;
    let worklist: RecoveryWorklist = serde_json::from_slice(&bytes)
        .with_context(|| format!("could not parse recovery worklist {}", path.display()))?;
    if worklist.schema_version != SCHEMA_VERSION {
        bail!(
            "recovery worklist {} uses schema version {}; expected {SCHEMA_VERSION}",
            path.display(),
            worklist.schema_version
        );
    }

    let mut errors = Vec::new();
    let mut audited = BTreeMap::new();
    let mut implemented_groups = 0;
    let mut follow_up_groups = 0;
    for group in &worklist.groups {
        let expected_id = group_id(&group.role, &group.text);
        if group.group_id != expected_id {
            errors.push(format!(
                "{} has stale group_id {:?}; expected {:?}",
                group_label(group),
                group.group_id,
                expected_id
            ));
        }
        let expected_structural = structural_signature(&group.role, &group.text);
        if group.structural_signature != expected_structural {
            errors.push(format!(
                "{} has stale structural_signature {:?}; expected {:?}",
                group_label(group),
                group.structural_signature,
                expected_structural
            ));
        }
        let expected_lexical = lexical_signature(&group.text);
        if group.lexical_signature != expected_lexical {
            errors.push(format!(
                "{} has stale lexical_signature {:?}; expected {:?}",
                group_label(group),
                group.lexical_signature,
                expected_lexical
            ));
        }
        if audited.insert(group.group_id.as_str(), group).is_some() {
            errors.push(format!("duplicate audited group_id {:?}", group.group_id));
        }

        match group.disposition.as_ref() {
            None => errors.push(format!("{} has no causal disposition", group_label(group))),
            Some(disposition) => {
                validate_disposition(group, disposition, &mut errors);
                match disposition.resolution {
                    Resolution::Implemented => implemented_groups += 1,
                    Resolution::FollowUp => follow_up_groups += 1,
                }
            }
        }
    }

    let current = current_groups
        .iter()
        .map(|group| (group_id(&group.role, &group.text), group))
        .collect::<BTreeMap<_, _>>();
    for (id, group) in &current {
        let Some(audited_group) = audited.get(id.as_str()) else {
            errors.push(format!(
                "current recovery group {} {:?} is absent from the audit",
                group.role, group.text
            ));
            continue;
        };
        if audited_group.role != group.role || audited_group.text != group.text {
            errors.push(format!(
                "current recovery group {} {:?} collides with a different audited group",
                group.role, group.text
            ));
        }
    }

    for group in &worklist.groups {
        let is_current = current.contains_key(&group.group_id);
        let Some(disposition) = group.disposition.as_ref() else {
            continue;
        };
        match (disposition.resolution, is_current) {
            (Resolution::Implemented, true) => errors.push(format!(
                "{} is marked implemented but still recovers",
                group_label(group)
            )),
            (Resolution::FollowUp, false) => errors.push(format!(
                "{} is assigned to a follow-up but no longer recovers",
                group_label(group)
            )),
            (Resolution::Implemented, false) | (Resolution::FollowUp, true) => {}
        }
    }

    if !errors.is_empty() {
        bail!(
            "recovery worklist verification failed with {} error(s):\n  - {}",
            errors.len(),
            errors.join("\n  - ")
        );
    }

    Ok(VerificationSummary {
        audited: worklist.groups.len(),
        implemented: implemented_groups,
        follow_up: follow_up_groups,
        current: current.len(),
    })
}

fn validate_disposition(
    group: &RecoveryWorklistGroup,
    disposition: &Disposition,
    errors: &mut Vec<String>,
) {
    if disposition.rationale.trim().is_empty() {
        errors.push(format!(
            "{} has an empty causal rationale",
            group_label(group)
        ));
    }
    match (disposition.cause, disposition.resolution) {
        (
            CausalCause::Composition
            | CausalCause::FeatureTransition
            | CausalCause::Admission
            | CausalCause::Lowering,
            Resolution::Implemented,
        ) => {
            if disposition
                .evidence
                .iter()
                .all(|item| item.trim().is_empty())
            {
                errors.push(format!(
                    "{} is implemented without direct typed/render/registration evidence",
                    group_label(group)
                ));
            }
            if disposition.follow_up_ticket.is_some() {
                errors.push(format!(
                    "{} is implemented but also names a follow-up ticket",
                    group_label(group)
                ));
            }
        }
        (
            CausalCause::UnsupportedGrammar | CausalCause::UnsupportedLexicon,
            Resolution::FollowUp,
        ) => {
            if disposition
                .follow_up_ticket
                .as_deref()
                .is_none_or(|ticket| ticket.trim().is_empty())
            {
                errors.push(format!(
                    "{} retains recovery without a focused follow-up ticket",
                    group_label(group)
                ));
            }
        }
        (cause, resolution) => errors.push(format!(
            "{} has incompatible cause {cause:?} and resolution {resolution:?}",
            group_label(group)
        )),
    }
}

fn group_label(group: &RecoveryWorklistGroup) -> String {
    format!("{} {:?}", group.role, group.text)
}

fn group_id(role: &str, text: &str) -> String {
    let mut hasher = Sha256::new();
    hasher.update(role.as_bytes());
    hasher.update([0]);
    hasher.update(text.as_bytes());
    format!("{role}:{}", hex(&hasher.finalize()))
}

fn hex(bytes: &[u8]) -> String {
    const DIGITS: &[u8; 16] = b"0123456789abcdef";
    let mut result = String::with_capacity(bytes.len() * 2);
    for byte in bytes {
        result.push(char::from(DIGITS[usize::from(byte >> 4)]));
        result.push(char::from(DIGITS[usize::from(byte & 0x0f)]));
    }
    result
}

fn lexical_signature(text: &str) -> String {
    signature_tokens(text)
        .into_iter()
        .map(|token| match token {
            SignatureToken::Word(word) => word.to_lowercase(),
            SignatureToken::Number => "<number>".to_owned(),
            SignatureToken::Symbol => "<symbol>".to_owned(),
            SignatureToken::Punctuation(mark) => mark.to_string(),
        })
        .collect::<Vec<_>>()
        .join(" ")
}

fn structural_signature(role: &str, text: &str) -> String {
    let shape = signature_tokens(text)
        .into_iter()
        .map(|token| match token {
            SignatureToken::Word(_) => "word".to_owned(),
            SignatureToken::Number => "number".to_owned(),
            SignatureToken::Symbol => "symbol".to_owned(),
            SignatureToken::Punctuation(mark) => format!("punct({mark})"),
        })
        .collect::<Vec<_>>()
        .join(" ");
    format!("{role}: {shape}")
}

#[derive(Debug, Clone, PartialEq, Eq)]
enum SignatureToken {
    Word(String),
    Number,
    Symbol,
    Punctuation(char),
}

fn signature_tokens(text: &str) -> Vec<SignatureToken> {
    let chars = text.chars().collect::<Vec<_>>();
    let mut tokens = Vec::new();
    let mut index = 0;
    while index < chars.len() {
        let current = chars[index];
        if current.is_whitespace() {
            index += 1;
        } else if current == '{' {
            index += 1;
            while index < chars.len() && chars[index] != '}' {
                index += 1;
            }
            if index < chars.len() {
                index += 1;
            }
            tokens.push(SignatureToken::Symbol);
        } else if current.is_numeric() {
            index += 1;
            while index < chars.len() && chars[index].is_numeric() {
                index += 1;
            }
            tokens.push(SignatureToken::Number);
        } else if current.is_alphabetic() {
            let start = index;
            index += 1;
            while index < chars.len()
                && (chars[index].is_alphabetic() || matches!(chars[index], '\'' | '\u{2019}'))
            {
                index += 1;
            }
            tokens.push(SignatureToken::Word(chars[start..index].iter().collect()));
        } else {
            tokens.push(SignatureToken::Punctuation(current));
            index += 1;
        }
    }
    tokens
}

#[cfg(test)]
mod tests {
    use tempfile::NamedTempFile;

    use super::*;

    fn group(role: &str, text: &str) -> RuntimeRecoveryGroup {
        RuntimeRecoveryGroup {
            role: role.to_owned(),
            text: text.to_owned(),
            occurrences: 1,
            source_tokens: 4,
            face_names: vec!["Witness".to_owned()],
        }
    }

    fn write_worklist(worklist: &RecoveryWorklist) -> NamedTempFile {
        let file = NamedTempFile::new().unwrap();
        fs::write(file.path(), serde_json::to_vec_pretty(worklist).unwrap()).unwrap();
        file
    }

    fn audited_group(
        group: &RuntimeRecoveryGroup,
        disposition: Option<Disposition>,
    ) -> RecoveryWorklistGroup {
        RecoveryWorklistGroup {
            group_id: group_id(&group.role, &group.text),
            role: group.role.clone(),
            text: group.text.clone(),
            structural_signature: structural_signature(&group.role, &group.text),
            lexical_signature: lexical_signature(&group.text),
            occurrences: group.occurrences,
            source_tokens: group.source_tokens,
            face_names: group.face_names.clone(),
            disposition,
        }
    }

    #[test]
    fn signatures_cluster_inflection_slots_without_erasing_lexical_words() {
        let first = "Pay 2 {E}: Draw a card.";
        let second = "PAY 12 {R}: draw a card.";

        assert_eq!(lexical_signature(first), lexical_signature(second));
        assert_eq!(
            structural_signature("activation cost", first),
            structural_signature("activation cost", second)
        );
        assert_ne!(
            structural_signature("activation cost", first),
            structural_signature("clause", first)
        );
    }

    #[test]
    fn export_is_a_complete_annotatable_runtime_worklist() {
        let source = group("clause", "You frobnitz 2 cards.");
        let json = export(std::slice::from_ref(&source)).unwrap();
        let worklist: RecoveryWorklist = serde_json::from_str(&json).unwrap();

        assert_eq!(worklist.schema_version, SCHEMA_VERSION);
        assert_eq!(worklist.notice, NOTICE);
        assert_eq!(worklist.groups.len(), 1);
        assert_eq!(worklist.groups[0].text, source.text);
        assert!(worklist.groups[0].disposition.is_none());
    }

    #[test]
    fn verification_covers_removed_implementations_and_retained_follow_ups() {
        let implemented = group("clause", "You frobnitz a card.");
        let retained = group("nominal", "a blorple you control");
        let worklist = RecoveryWorklist {
            schema_version: SCHEMA_VERSION,
            notice: NOTICE.to_owned(),
            groups: vec![
                audited_group(
                    &implemented,
                    Some(Disposition {
                        cause: CausalCause::Composition,
                        resolution: Resolution::Implemented,
                        rationale: "The generated constituents lacked a parent production."
                            .to_owned(),
                        evidence: vec!["typed AST + render + registration test".to_owned()],
                        follow_up_ticket: None,
                    }),
                ),
                audited_group(
                    &retained,
                    Some(Disposition {
                        cause: CausalCause::UnsupportedLexicon,
                        resolution: Resolution::FollowUp,
                        rationale: "The noun domain has no generated owner.".to_owned(),
                        evidence: Vec::new(),
                        follow_up_ticket: Some(
                            "docs/tickets/planned/english-blorple-domain.md".to_owned(),
                        ),
                    }),
                ),
            ],
        };
        let file = write_worklist(&worklist);

        let summary = verify(file.path(), std::slice::from_ref(&retained)).unwrap();

        assert_eq!(summary.audited, 2);
        assert_eq!(summary.implemented, 1);
        assert_eq!(summary.follow_up, 1);
        assert_eq!(summary.current, 1);
    }

    #[test]
    fn verification_rejects_undisposed_and_new_runtime_groups() {
        let audited = group("clause", "You frobnitz a card.");
        let new = group("clause", "You glorp a card.");
        let worklist = RecoveryWorklist {
            schema_version: SCHEMA_VERSION,
            notice: NOTICE.to_owned(),
            groups: vec![audited_group(&audited, None)],
        };
        let file = write_worklist(&worklist);

        let error = verify(file.path(), &[audited, new])
            .unwrap_err()
            .to_string();

        assert!(error.contains("has no causal disposition"), "{error}");
        assert!(error.contains("is absent from the audit"), "{error}");
    }

    #[test]
    fn verification_enforces_cause_resolution_evidence_contracts() {
        let current = group("clause", "You frobnitz a card.");
        let worklist = RecoveryWorklist {
            schema_version: SCHEMA_VERSION,
            notice: NOTICE.to_owned(),
            groups: vec![audited_group(
                &current,
                Some(Disposition {
                    cause: CausalCause::Composition,
                    resolution: Resolution::FollowUp,
                    rationale: String::new(),
                    evidence: Vec::new(),
                    follow_up_ticket: None,
                }),
            )],
        };
        let file = write_worklist(&worklist);

        let error = verify(file.path(), &[current]).unwrap_err().to_string();

        assert!(error.contains("empty causal rationale"), "{error}");
        assert!(error.contains("incompatible cause"), "{error}");
    }
}
