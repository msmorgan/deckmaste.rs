//! A grammatical syntax tree for Magic card text.
//!
//! This crate parses Oracle text into English structure before deckmaste's
//! executable rules grammar. Parts of speech are selected by grammar slots,
//! not assigned by the lexer, and current card vocabularies can be supplied
//! through [`Catalogs`]. Unknown nouns remain explicit lexical opacity, while
//! text the grammar cannot yet classify is retained as role-specific
//! [`syntax::RecoveredText`].
//!
//! The returned [`syntax::OracleText`] owns every spelling needed to render
//! itself. It does not retain the input string; byte [`Span`] values exist only
//! in diagnostics and optional parse provenance.

pub mod catalog;
pub use deckmaste_features as features;
mod chart;
mod construction;
mod constructions;
mod forest;
mod fragment;
mod grammar;
mod identity;
mod input;
mod numeral;
mod parse;
pub mod renderer;
mod span;
mod surface;
pub mod syntax;
pub mod word;

pub use catalog::CatalogKind;
pub use catalog::Catalogs;
pub use chart::ChartStats;
pub use construction::ConstructionAlternative;
pub use construction::ConstructionBackend;
pub use construction::ConstructionDecision;
pub use construction::ConstructionEvidence;
pub use construction::ConstructionEvidenceKind;
pub use construction::ConstructionFamily;
pub use construction::ConstructionId;
pub use construction::ConstructionOwner;
pub use forest::ForestStats;
pub use forest::ParseCost;
pub use forest::ParseCostDimension;
pub use forest::SelectionReason;
pub use fragment::Fragment;
pub use fragment::FragmentKind;
pub use fragment::FragmentReport;
pub use fragment::parse_fragment;
pub use fragment::render_fragment;
pub use input::normalize_roll_row_dashes;
pub use input::normalize_sentence_case;
pub use input::normalize_typographic_quotes;
pub use input::strip_reminder_text;
pub use numeral::Numeral;
pub use numeral::ParseNumeralError;
pub use parse::Diagnostic;
pub use parse::DiagnosticKind;
pub use parse::ParseProvenance;
pub use parse::ParseReport;
pub use parse::ParseSelection;
pub use parse::ParseWork;
pub use parse::parse;
pub use parse::parse_with_catalogs;
pub use parse::parse_with_identity;
pub use renderer::RenderError;
pub use span::Span;
pub use surface::SurfaceDiagnosticKind;

/// The active English construction-family inventory.
#[must_use]
pub fn construction_families() -> &'static [ConstructionFamily] {
    grammar::construction::families()
}

/// Looks up one active construction family by stable identity.
#[must_use]
pub fn construction_family(id: ConstructionId) -> Option<ConstructionFamily> {
    grammar::construction::family_by_id(id)
}
