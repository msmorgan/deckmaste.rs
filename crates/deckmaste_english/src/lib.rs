//! **PRE-CUTOVER v1 — this crate is deleted at the `english_v2` cutover.**
//!
//! `deckmaste_english_v2` takes this crate's name at cutover. Put new
//! architecture and features there; touch this crate only to keep current
//! users functioning or to enable cutover. Never treat the pair as parallel
//! long-term implementations, and never copy the v2 design back here for
//! parity. Authority: `docs/decisions/english-v2-rewrite.md`.
//!
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

pub mod ability;
pub mod adjective;
pub mod catalog;
pub mod clause;
pub mod coordination;
pub mod cost;
pub mod determiner;
pub mod keyword_line;
pub use deckmaste_features as features;
mod chart;
mod construction;
mod constructions;
mod forest;
mod fragment;
mod grammar;
mod identity;
mod input;
pub mod nominal;
pub mod noun_phrase;
mod numeral;
mod parse;
pub mod predicate;
pub mod prepositional_phrase;
mod projection;
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
pub use deckmaste_construction_compiler::runtime::ConstructionProjection;
pub use deckmaste_construction_compiler::runtime::OwnedProjectionValue;
pub use deckmaste_construction_compiler::runtime::ProjectedAtom;
pub use deckmaste_construction_compiler::runtime::ProjectedMember;
pub use deckmaste_construction_compiler::runtime::ProjectedProduct;
pub use deckmaste_construction_compiler::runtime::ProjectedSequence;
pub use deckmaste_construction_compiler::runtime::ProjectedValue;
pub use deckmaste_construction_compiler::runtime::ProjectedVariant;
pub use deckmaste_construction_compiler::runtime::ProjectedWitness;
pub use deckmaste_construction_compiler::runtime::ProjectionValue;
pub use forest::ForestStats;
pub use forest::ParseCost;
pub use forest::ParseCostDimension;
pub use forest::SelectionReason;
pub use fragment::Fragment;
pub use fragment::FragmentKind;
pub use fragment::FragmentReport;
pub use fragment::parse_fragment;
pub use fragment::render_fragment;
pub use grammar::diagnostic::DiagnosticLimits;
pub use grammar::diagnostic::FailureCategory;
pub use grammar::diagnostic::FailureClusterKey;
pub use grammar::diagnostic::FailureDiagnosticError;
pub use grammar::diagnostic::FailureFingerprint;
pub use grammar::diagnostic::FailureStage;
pub use grammar::diagnostic::FeatureState;
pub use grammar::diagnostic::FingerprintStatus;
pub use grammar::diagnostic::FrontierConstituent;
pub use grammar::diagnostic::NearDeclaration;
pub use grammar::diagnostic::UncoveredBoundary;
pub use grammar::diagnostic::diagnose_nonterminal_failure;
pub use grammar::diagnostic::diagnose_nonterminal_failure_with_identity;
pub use input::normalize_loyalty_minus;
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
pub use projection::ProjectionError;
pub use projection::project_fragment;
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
