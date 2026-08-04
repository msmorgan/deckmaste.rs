//! Validation diagnostics: stable machine-readable codes, human messages,
//! and a deterministic sort so reported order never depends on traversal.

use proc_macro2::Span;

#[derive(Debug, Clone)]
pub struct Note {
    pub message: String,
    pub span: Span,
}

#[derive(Debug, Clone)]
pub struct Diagnostic {
    pub code: DiagCode,
    /// `None` scopes the diagnostic to the whole group (e.g. a duplicate
    /// element name — elements belong to the group, not a construction).
    pub construction: Option<String>,
    pub message: String,
    pub span: Span,
    pub notes: Vec<Note>,
}

impl Diagnostic {
    pub fn new(code: DiagCode, construction: &str, message: impl Into<String>) -> Self {
        Self {
            code,
            construction: Some(construction.to_owned()),
            message: message.into(),
            span: Span::call_site(),
            notes: Vec::new(),
        }
    }

    pub fn group(code: DiagCode, message: impl Into<String>) -> Self {
        Self {
            code,
            construction: None,
            message: message.into(),
            span: Span::call_site(),
            notes: Vec::new(),
        }
    }

    pub fn with_span(mut self, span: Span) -> Self {
        self.span = span;
        self
    }

    pub fn with_note(mut self, message: impl Into<String>, span: Span) -> Self {
        self.notes.push(Note {
            message: message.into(),
            span,
        });
        self
    }
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum DiagCode {
    DuplicateConstructionId,
    DuplicateOrdinal,
    UnknownElement,
    DuplicateName,
    DeserializeRequiresOwn,
    GeneratedNameCollision,
    UnknownFieldPath,
    PresenceOnNonOptional,
    WitnessFieldCollision,
    StoredWitnessPathUnknown,
    SurfaceKindMismatch,
    PredicateKindMismatch,
    EmptyProduction,
    FieldNeverProduced,
    HoleConsumedTwice,
    UncoveredValueSpace,
    AmbiguousLinearization,
    ContradictoryConstraints,
    UnknownCombinator,
    UnsupportedConstraintPath,
    SelfDominance,
    DominanceCycle,
    FreeWitnessStratum,
    DiscourseFeatureExcluded,
}

impl DiagCode {
    pub fn as_str(self) -> &'static str {
        match self {
            Self::DuplicateConstructionId => "EC001",
            Self::DuplicateOrdinal => "EC002",
            Self::UnknownElement => "EC003",
            Self::DuplicateName => "EC004",
            Self::DeserializeRequiresOwn => "EC005",
            Self::GeneratedNameCollision => "EC006",
            Self::UnknownFieldPath => "EC010",
            Self::PresenceOnNonOptional => "EC011",
            Self::WitnessFieldCollision => "EC012",
            Self::StoredWitnessPathUnknown => "EC013",
            Self::SurfaceKindMismatch => "EC014",
            Self::PredicateKindMismatch => "EC015",
            Self::EmptyProduction => "EC020",
            Self::FieldNeverProduced => "EC021",
            Self::HoleConsumedTwice => "EC022",
            Self::UncoveredValueSpace => "EC023",
            Self::AmbiguousLinearization => "EC024",
            Self::ContradictoryConstraints => "EC030",
            Self::UnknownCombinator => "EC031",
            Self::UnsupportedConstraintPath => "EC032",
            Self::SelfDominance => "EC040",
            Self::DominanceCycle => "EC041",
            Self::FreeWitnessStratum => "EC050",
            Self::DiscourseFeatureExcluded => "EC051",
        }
    }
}

pub fn sort_key(diag: &Diagnostic) -> (String, &'static str, String) {
    (
        diag.construction.clone().unwrap_or_default(),
        diag.code.as_str(),
        diag.message.clone(),
    )
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn codes_render_stable_strings() {
        assert_eq!(DiagCode::DuplicateConstructionId.as_str(), "EC001");
        assert_eq!(DiagCode::DominanceCycle.as_str(), "EC041");
    }

    #[test]
    fn diagnostics_sort_independent_of_emission_order() {
        let a = Diagnostic::new(DiagCode::EmptyProduction, "zeta", "form seed is empty");
        let b = Diagnostic::new(DiagCode::DuplicateOrdinal, "alpha", "ordinal 0 reused");
        let mut forward = vec![a.clone(), b.clone()];
        let mut backward = vec![b, a];
        forward.sort_by_key(sort_key);
        backward.sort_by_key(sort_key);
        let codes = |v: &[Diagnostic]| v.iter().map(|d| d.code.as_str()).collect::<Vec<_>>();
        assert_eq!(codes(&forward), codes(&backward));
        assert_eq!(codes(&forward), vec!["EC002", "EC020"]);
    }

    #[test]
    fn new_codes_render_stable_strings() {
        assert_eq!(DiagCode::DuplicateName.as_str(), "EC004");
        assert_eq!(DiagCode::DeserializeRequiresOwn.as_str(), "EC005");
        assert_eq!(DiagCode::GeneratedNameCollision.as_str(), "EC006");
        assert_eq!(DiagCode::PresenceOnNonOptional.as_str(), "EC011");
        assert_eq!(DiagCode::SurfaceKindMismatch.as_str(), "EC014");
        assert_eq!(DiagCode::PredicateKindMismatch.as_str(), "EC015");
        assert_eq!(DiagCode::UnsupportedConstraintPath.as_str(), "EC032");
        assert_eq!(DiagCode::FreeWitnessStratum.as_str(), "EC050");
        assert_eq!(DiagCode::DiscourseFeatureExcluded.as_str(), "EC051");
    }

    #[test]
    fn group_diagnostics_sort_before_construction_diagnostics() {
        let g = Diagnostic::group(DiagCode::DuplicateName, "element `m` declared twice");
        let c = Diagnostic::new(DiagCode::DuplicateConstructionId, "alpha", "dup");
        let mut v = vec![c, g];
        v.sort_by_key(sort_key);
        assert!(v[0].construction.is_none());
    }

    #[test]
    fn notes_attach_and_survive() {
        let d = Diagnostic::new(DiagCode::DuplicateConstructionId, "alpha", "dup")
            .with_note("first declared here", proc_macro2::Span::call_site());
        assert_eq!(d.notes.len(), 1);
        assert_eq!(d.notes[0].message, "first declared here");
    }
}
