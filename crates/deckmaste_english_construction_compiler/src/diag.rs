//! Validation diagnostics: stable machine-readable codes, human messages,
//! and a deterministic sort so reported order never depends on traversal.

use proc_macro2::Span;

#[derive(Debug, Clone)]
pub struct Diagnostic {
    pub code: DiagCode,
    pub construction: String,
    pub message: String,
    pub span: Span,
}

impl Diagnostic {
    pub fn new(code: DiagCode, construction: &str, message: impl Into<String>) -> Self {
        Self {
            code,
            construction: construction.to_owned(),
            message: message.into(),
            span: Span::call_site(),
        }
    }

    pub fn with_span(mut self, span: Span) -> Self {
        self.span = span;
        self
    }
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum DiagCode {
    DuplicateConstructionId,
    DuplicateOrdinal,
    UnknownElement,
    UnknownFieldPath,
    WitnessFieldCollision,
    StoredWitnessPathUnknown,
    EmptyProduction,
    FieldNeverProduced,
    HoleConsumedTwice,
    UncoveredValueSpace,
    AmbiguousLinearization,
    ContradictoryConstraints,
    UnknownCombinator,
    SelfDominance,
    DominanceCycle,
}

impl DiagCode {
    pub fn as_str(self) -> &'static str {
        match self {
            Self::DuplicateConstructionId => "EC001",
            Self::DuplicateOrdinal => "EC002",
            Self::UnknownElement => "EC003",
            Self::UnknownFieldPath => "EC010",
            Self::WitnessFieldCollision => "EC012",
            Self::StoredWitnessPathUnknown => "EC013",
            Self::EmptyProduction => "EC020",
            Self::FieldNeverProduced => "EC021",
            Self::HoleConsumedTwice => "EC022",
            Self::UncoveredValueSpace => "EC023",
            Self::AmbiguousLinearization => "EC024",
            Self::ContradictoryConstraints => "EC030",
            Self::UnknownCombinator => "EC031",
            Self::SelfDominance => "EC040",
            Self::DominanceCycle => "EC041",
        }
    }
}

pub fn sort_key(diag: &Diagnostic) -> (String, &'static str, String) {
    (
        diag.construction.clone(),
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
}
