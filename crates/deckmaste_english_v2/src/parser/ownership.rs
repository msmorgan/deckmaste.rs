use super::TextSpan;
use crate::constructions::Leaf;
use crate::constructions::LexicalOwner;
use crate::constructions::LexicalProvenanceKind;

#[cfg(any(test, feature = "test-support"))]
thread_local! {
    static PROJECTION_RUNS: std::cell::Cell<usize> = const { std::cell::Cell::new(0) };
    static FORCE_INSPECTION_CORRUPTION: std::cell::Cell<bool> = const { std::cell::Cell::new(false) };
}

#[cfg(test)]
pub(super) fn reset_projection_runs() {
    PROJECTION_RUNS.with(|runs| runs.set(0));
}

#[cfg(test)]
pub(super) fn projection_runs() -> usize {
    PROJECTION_RUNS.with(std::cell::Cell::get)
}

#[cfg(any(test, feature = "test-support"))]
pub(super) fn force_inspection_corruption(value: bool) {
    FORCE_INSPECTION_CORRUPTION.with(|forced| forced.set(value));
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub(crate) struct RawLexicalClaim {
    pub(crate) span: TextSpan,
    pub(crate) value: Leaf,
    pub(crate) owner: LexicalOwner,
}

#[derive(Debug, Clone, PartialEq, Eq)]
#[allow(dead_code, reason = "render claim collection lands in this task")]
pub(crate) struct RawRenderedClaim {
    pub(crate) span: TextSpan,
    pub(crate) owner: LexicalOwner,
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct LexicalClaim {
    span: TextSpan,
    kind: LexicalProvenanceKind,
    stable_owner_id: String,
    semantic_summary: String,
}

impl LexicalClaim {
    #[must_use]
    pub const fn span(&self) -> TextSpan {
        self.span
    }

    #[must_use]
    pub const fn kind(&self) -> LexicalProvenanceKind {
        self.kind
    }

    #[must_use]
    pub fn stable_owner_id(&self) -> &str {
        &self.stable_owner_id
    }

    #[must_use]
    pub fn semantic_summary(&self) -> &str {
        &self.semantic_summary
    }
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum InvalidSpanKind {
    OutOfBounds,
    NonUtf8Boundary,
}

/// Identifies which rendered-byte comparison produced an ownership mismatch.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum ByteMismatchScope {
    /// The complete rendered document differs from the parsed source bytes.
    WholeRender,
    /// Corresponding ownership claims cover different byte slices.
    ClaimSlice { index: usize },
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub enum OwnershipFailure {
    Gap {
        span: TextSpan,
    },
    Overlap {
        left: TextSpan,
        right: TextSpan,
        overlap: TextSpan,
    },
    InvalidSpan {
        span: TextSpan,
        kind: InvalidSpanKind,
    },
    Synthetic {
        span: TextSpan,
    },
    ProvenancePlanMismatch {
        index: usize,
        parsed: String,
        rendered: String,
    },
    ByteMismatch {
        scope: ByteMismatchScope,
        expected: String,
        actual: String,
    },
}

#[derive(Debug, Clone, Default, PartialEq, Eq)]
pub struct OwnershipSummary {
    covered: bool,
    claims: usize,
    claimed_bytes: usize,
    form_literal_claims: usize,
    form_literal_bytes: usize,
    vocab_claims: usize,
    vocab_bytes: usize,
    lexeme_claims: usize,
    lexeme_bytes: usize,
    codec_claims: usize,
    codec_bytes: usize,
    identity_claims: usize,
    identity_bytes: usize,
    gap_spans: usize,
    gap_bytes: usize,
    overlap_spans: usize,
    overlap_bytes: usize,
    synthetic_claims: usize,
    provenance_plan_mismatches: usize,
}

macro_rules! summary_accessors {
    ($($name:ident),* $(,)?) => {
        impl OwnershipSummary {
            $(
                #[must_use]
                pub const fn $name(&self) -> usize {
                    self.$name
                }
            )*
        }
    };
}

impl OwnershipSummary {
    #[must_use]
    pub const fn covered(&self) -> bool {
        self.covered
    }
}

summary_accessors!(
    claims,
    claimed_bytes,
    form_literal_claims,
    form_literal_bytes,
    vocab_claims,
    vocab_bytes,
    lexeme_claims,
    lexeme_bytes,
    codec_claims,
    codec_bytes,
    identity_claims,
    identity_bytes,
    gap_spans,
    gap_bytes,
    overlap_spans,
    overlap_bytes,
    synthetic_claims,
    provenance_plan_mismatches,
);

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct SelectedOwnership {
    parsed_claims: Vec<LexicalClaim>,
    rendered_claims: Vec<LexicalClaim>,
    rendered_text: String,
    failures: Vec<OwnershipFailure>,
    summary: OwnershipSummary,
}

impl SelectedOwnership {
    #[must_use]
    pub fn parsed_claims(&self) -> &[LexicalClaim] {
        &self.parsed_claims
    }

    #[must_use]
    pub fn rendered_claims(&self) -> &[LexicalClaim] {
        &self.rendered_claims
    }

    #[must_use]
    pub fn rendered_text(&self) -> &str {
        &self.rendered_text
    }

    #[must_use]
    pub fn failures(&self) -> &[OwnershipFailure] {
        &self.failures
    }

    #[must_use]
    pub const fn summary(&self) -> &OwnershipSummary {
        &self.summary
    }
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub(crate) struct OwnershipInspectionError;

pub(crate) fn validate_ownership(
    text: &str,
    parsed: &[RawLexicalClaim],
    synthetic_spans: &[TextSpan],
    rendered_text: String,
    rendered: &[RawRenderedClaim],
) -> Result<SelectedOwnership, OwnershipInspectionError> {
    #[cfg(any(test, feature = "test-support"))]
    if FORCE_INSPECTION_CORRUPTION.with(std::cell::Cell::get) {
        return Err(OwnershipInspectionError);
    }
    #[cfg(test)]
    PROJECTION_RUNS.with(|runs| runs.set(runs.get() + 1));
    let parsed_claims = parsed
        .iter()
        .map(|claim| LexicalClaim {
            span: claim.span,
            kind: claim.owner.kind(),
            stable_owner_id: claim.owner.stable_id_owned(),
            semantic_summary: format!("{:?}", claim.value),
        })
        .collect::<Vec<_>>();
    let rendered_claims = rendered
        .iter()
        .map(|claim| LexicalClaim {
            span: claim.span,
            kind: claim.owner.kind(),
            stable_owner_id: claim.owner.stable_id_owned(),
            semantic_summary: rendered_text
                .get(claim.span.start..claim.span.end)
                .unwrap_or("<invalid span>")
                .to_owned(),
        })
        .collect::<Vec<_>>();
    let mut failures = Vec::new();
    failures.extend(
        synthetic_spans
            .iter()
            .copied()
            .map(|span| OwnershipFailure::Synthetic { span }),
    );
    validate_partition(text, &parsed_claims, &mut failures);
    validate_partition(&rendered_text, &rendered_claims, &mut failures);
    if rendered_text != text {
        failures.push(OwnershipFailure::ByteMismatch {
            scope: ByteMismatchScope::WholeRender,
            expected: text.to_owned(),
            actual: rendered_text.clone(),
        });
    }
    let count = parsed_claims.len().max(rendered_claims.len());
    for index in 0..count {
        let parsed = parsed_claims.get(index);
        let rendered = rendered_claims.get(index);
        if parsed.map(|claim| (claim.kind, claim.stable_owner_id.as_str()))
            != rendered.map(|claim| (claim.kind, claim.stable_owner_id.as_str()))
        {
            failures.push(OwnershipFailure::ProvenancePlanMismatch {
                index,
                parsed: plan_label(parsed),
                rendered: plan_label(rendered),
            });
            continue;
        }
        if let (Some(parsed), Some(rendered)) = (parsed, rendered)
            && let (Some(expected), Some(actual)) = (
                text.get(parsed.span.start..parsed.span.end),
                rendered_text.get(rendered.span.start..rendered.span.end),
            )
            && expected != actual
        {
            failures.push(OwnershipFailure::ByteMismatch {
                scope: ByteMismatchScope::ClaimSlice { index },
                expected: expected.to_owned(),
                actual: actual.to_owned(),
            });
        }
    }
    let summary = summarize(&parsed_claims, &failures);
    Ok(SelectedOwnership {
        parsed_claims,
        rendered_claims,
        rendered_text,
        failures,
        summary,
    })
}

fn plan_label(claim: Option<&LexicalClaim>) -> String {
    claim.map_or_else(
        || "<missing>".to_owned(),
        |claim| format!("{:?}:{}", claim.kind, claim.stable_owner_id),
    )
}

fn validate_partition(text: &str, claims: &[LexicalClaim], failures: &mut Vec<OwnershipFailure>) {
    let mut cursor = 0;
    let mut previous = None;
    for claim in claims {
        let span = claim.span;
        if span.start > span.end || span.end > text.len() {
            failures.push(OwnershipFailure::InvalidSpan {
                span,
                kind: InvalidSpanKind::OutOfBounds,
            });
            continue;
        }
        if !text.is_char_boundary(span.start) || !text.is_char_boundary(span.end) {
            failures.push(OwnershipFailure::InvalidSpan {
                span,
                kind: InvalidSpanKind::NonUtf8Boundary,
            });
            continue;
        }
        if span.start == span.end {
            failures.push(OwnershipFailure::Synthetic { span });
            continue;
        }
        if span.start > cursor {
            failures.push(OwnershipFailure::Gap {
                span: TextSpan {
                    start: cursor,
                    end: span.start,
                },
            });
        } else if span.start < cursor {
            let left = previous.unwrap_or(TextSpan {
                start: 0,
                end: cursor,
            });
            failures.push(OwnershipFailure::Overlap {
                left,
                right: span,
                overlap: TextSpan {
                    start: span.start,
                    end: cursor.min(span.end),
                },
            });
        }
        cursor = cursor.max(span.end);
        previous = Some(span);
    }
    if cursor < text.len() {
        failures.push(OwnershipFailure::Gap {
            span: TextSpan {
                start: cursor,
                end: text.len(),
            },
        });
    }
}

fn summarize(claims: &[LexicalClaim], failures: &[OwnershipFailure]) -> OwnershipSummary {
    let mut summary = OwnershipSummary {
        claims: claims.len(),
        ..OwnershipSummary::default()
    };
    for claim in claims {
        let bytes = claim.span.end.saturating_sub(claim.span.start);
        summary.claimed_bytes += bytes;
        match claim.kind {
            LexicalProvenanceKind::FormLiteral => {
                summary.form_literal_claims += 1;
                summary.form_literal_bytes += bytes;
            }
            LexicalProvenanceKind::Vocab => {
                summary.vocab_claims += 1;
                summary.vocab_bytes += bytes;
            }
            LexicalProvenanceKind::Lexeme => {
                summary.lexeme_claims += 1;
                summary.lexeme_bytes += bytes;
            }
            LexicalProvenanceKind::Codec => {
                summary.codec_claims += 1;
                summary.codec_bytes += bytes;
            }
            LexicalProvenanceKind::Identity => {
                summary.identity_claims += 1;
                summary.identity_bytes += bytes;
            }
        }
    }
    for failure in failures {
        match failure {
            OwnershipFailure::Gap { span } => {
                summary.gap_spans += 1;
                summary.gap_bytes += span.end.saturating_sub(span.start);
            }
            OwnershipFailure::Overlap { overlap, .. } => {
                summary.overlap_spans += 1;
                summary.overlap_bytes += overlap.end.saturating_sub(overlap.start);
            }
            OwnershipFailure::Synthetic { .. } => summary.synthetic_claims += 1,
            OwnershipFailure::ProvenancePlanMismatch { .. } => {
                summary.provenance_plan_mismatches += 1;
            }
            OwnershipFailure::InvalidSpan { .. } | OwnershipFailure::ByteMismatch { .. } => {}
        }
    }
    summary.covered = failures.is_empty();
    summary
}

#[cfg(test)]
mod tests {
    use super::*;

    fn owner(id: &'static str) -> LexicalOwner {
        LexicalOwner::static_owner(LexicalProvenanceKind::FormLiteral, id)
    }

    fn parsed(start: usize, end: usize, id: &'static str) -> RawLexicalClaim {
        RawLexicalClaim {
            span: TextSpan { start, end },
            value: Leaf::Literal("token"),
            owner: owner(id),
        }
    }

    fn rendered(start: usize, end: usize, id: &'static str) -> RawRenderedClaim {
        RawRenderedClaim {
            span: TextSpan { start, end },
            owner: owner(id),
        }
    }

    fn failures(
        text: &str,
        parsed: &[RawLexicalClaim],
        synthetic: &[TextSpan],
        rendered_text: &str,
        rendered: &[RawRenderedClaim],
    ) -> Vec<OwnershipFailure> {
        validate_ownership(text, parsed, synthetic, rendered_text.to_owned(), rendered)
            .expect("the fixture has inspectable ownership")
            .failures
    }

    #[test]
    fn lexical_ownership_authenticates_partition_and_plan_failures() {
        assert!(matches!(
            failures(
                "abc",
                &[parsed(1, 3, "a")],
                &[],
                "abc",
                &[rendered(0, 3, "a")],
            )[0],
            OwnershipFailure::Gap { .. }
        ));
        assert!(
            failures(
                "abc",
                &[parsed(0, 2, "a"), parsed(1, 3, "b")],
                &[],
                "abc",
                &[rendered(0, 2, "a"), rendered(1, 3, "b")],
            )
            .iter()
            .any(|failure| matches!(failure, OwnershipFailure::Overlap { .. }))
        );
        assert!(
            failures(
                "abc",
                &[parsed(0, 4, "a")],
                &[],
                "abc",
                &[rendered(0, 3, "a")],
            )
            .iter()
            .any(|failure| matches!(
                failure,
                OwnershipFailure::InvalidSpan {
                    kind: InvalidSpanKind::OutOfBounds,
                    ..
                }
            ))
        );
        assert!(
            failures("é", &[parsed(1, 2, "a")], &[], "é", &[rendered(0, 2, "a")],)
                .iter()
                .any(|failure| matches!(
                    failure,
                    OwnershipFailure::InvalidSpan {
                        kind: InvalidSpanKind::NonUtf8Boundary,
                        ..
                    }
                ))
        );
        assert!(
            failures(
                "abc",
                &[parsed(0, 3, "a")],
                &[TextSpan { start: 1, end: 1 }],
                "abc",
                &[rendered(0, 3, "a")],
            )
            .iter()
            .any(|failure| matches!(failure, OwnershipFailure::Synthetic { .. }))
        );
        assert!(
            failures(
                "abc",
                &[parsed(0, 3, "a")],
                &[],
                "abc",
                &[rendered(0, 3, "b")],
            )
            .iter()
            .any(|failure| matches!(failure, OwnershipFailure::ProvenancePlanMismatch { .. }))
        );
    }

    #[test]
    fn lexical_ownership_isolates_claimed_slice_mismatch_from_whole_render_bytes() {
        let failures = failures(
            "abcd",
            &[parsed(0, 1, "a"), parsed(1, 4, "b")],
            &[],
            "abcd",
            &[rendered(0, 2, "a"), rendered(2, 4, "b")],
        );
        assert_eq!(
            failures,
            vec![
                OwnershipFailure::ByteMismatch {
                    scope: ByteMismatchScope::ClaimSlice { index: 0 },
                    expected: "a".to_owned(),
                    actual: "ab".to_owned(),
                },
                OwnershipFailure::ByteMismatch {
                    scope: ByteMismatchScope::ClaimSlice { index: 1 },
                    expected: "bcd".to_owned(),
                    actual: "cd".to_owned(),
                },
            ]
        );
    }

    #[test]
    fn lexical_ownership_pins_whole_render_mismatch_separately() {
        let failures = failures(
            "abc",
            &[parsed(0, 3, "a")],
            &[],
            "abd",
            &[rendered(0, 3, "a")],
        );
        assert_eq!(
            failures,
            vec![
                OwnershipFailure::ByteMismatch {
                    scope: ByteMismatchScope::WholeRender,
                    expected: "abc".to_owned(),
                    actual: "abd".to_owned(),
                },
                OwnershipFailure::ByteMismatch {
                    scope: ByteMismatchScope::ClaimSlice { index: 0 },
                    expected: "abc".to_owned(),
                    actual: "abd".to_owned(),
                },
            ]
        );
    }
}
