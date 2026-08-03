use crate::Span;
use crate::catalog::Catalogs;
use crate::chart::ChartStats;
use crate::forest::ForestStats;
use crate::grammar::ability::AbilityDiagnosticKind;
use crate::grammar::ability::parse_oracle_text;
use crate::identity::SelfReference;
use crate::surface::SurfaceDiagnosticKind;
use crate::surface::collapse_full_names;
use crate::surface::lex;
use crate::syntax::OracleText;

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum DiagnosticKind {
    Surface(SurfaceDiagnosticKind),
    OrphanMode,
    EmptyActivationEffect,
    NoCompleteParse,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub struct Diagnostic {
    pub(crate) kind: DiagnosticKind,
    pub(crate) span: Span,
}

#[derive(Debug, Clone, Default, PartialEq, Eq)]
pub struct ParseProvenance {
    pub(crate) selections: Vec<ParseSelection>,
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct ParseSelection {
    pub(crate) span: Span,
    pub(crate) constituent_spans: Vec<Span>,
    pub(crate) rule: Option<usize>,
    pub(crate) construction: Option<crate::construction::ConstructionId>,
    pub(crate) constructions: Vec<crate::construction::ConstructionDecision>,
    pub(crate) tied_alternatives: Vec<usize>,
    pub(crate) cost: crate::forest::ParseCost,
    pub(crate) chart_stats: ChartStats,
    pub(crate) forest_stats: ForestStats,
}

#[derive(Debug)]
pub struct ParseReport {
    pub(crate) ast: OracleText,
    pub(crate) ability_spans: Vec<Span>,
    pub(crate) diagnostics: Vec<Diagnostic>,
    pub(crate) provenance: ParseProvenance,
    pub(crate) source_tokens: usize,
}

impl ParseReport {
    #[must_use]
    pub const fn ast(&self) -> &OracleText {
        &self.ast
    }

    /// Source spans of the top-level abilities represented by [`Self::ast`].
    #[must_use]
    pub fn ability_spans(&self) -> &[Span] {
        &self.ability_spans
    }

    #[must_use]
    pub fn diagnostics(&self) -> &[Diagnostic] {
        &self.diagnostics
    }

    #[must_use]
    pub const fn provenance(&self) -> &ParseProvenance {
        &self.provenance
    }

    #[must_use]
    pub const fn source_tokens(&self) -> usize {
        self.source_tokens
    }

    #[must_use]
    pub fn into_ast(self) -> OracleText {
        self.ast
    }
}

impl Diagnostic {
    #[must_use]
    pub const fn kind(self) -> DiagnosticKind {
        self.kind
    }

    #[must_use]
    pub const fn span(self) -> Span {
        self.span
    }
}

impl ParseProvenance {
    #[must_use]
    pub fn selections(&self) -> &[ParseSelection] {
        &self.selections
    }
}

impl ParseSelection {
    #[must_use]
    pub const fn span(&self) -> Span {
        self.span
    }

    /// Source spans of every nonterminal in this selected forest derivation.
    #[must_use]
    pub fn constituent_spans(&self) -> &[Span] {
        &self.constituent_spans
    }

    #[must_use]
    pub const fn rule(&self) -> Option<usize> {
        self.rule
    }

    /// Stable identity of the selected root construction.
    #[must_use]
    pub const fn selected_construction(&self) -> Option<crate::construction::ConstructionId> {
        self.construction
    }

    /// Source-ordered construction decisions in the selected derivation.
    #[must_use]
    pub fn constructions(&self) -> &[crate::construction::ConstructionDecision] {
        &self.constructions
    }

    /// Indices of the undominated alternatives at this selection's root with
    /// the minimum parse cost, including the selected alternative itself.
    ///
    /// A successful forest selection therefore has at least one index here;
    /// only a length greater than one indicates an equal-cost tie. The forest
    /// resolves such a tie deterministically by stable production identity.
    #[must_use]
    pub fn tied_alternatives(&self) -> &[usize] {
        &self.tied_alternatives
    }

    #[must_use]
    pub const fn cost(&self) -> crate::forest::ParseCost {
        self.cost
    }

    #[must_use]
    pub const fn chart_stats(&self) -> ChartStats {
        self.chart_stats
    }

    #[must_use]
    pub const fn forest_stats(&self) -> ForestStats {
        self.forest_stats
    }
}

/// Parses Oracle text with no card identity, so no self-reference is
/// recognized. Suitable for text known to contain no reference to the card's
/// own name.
#[must_use]
pub fn parse(source: &str) -> ParseReport {
    parse_with_catalogs(source, &Catalogs::default())
}

/// Parses Oracle text with catalogs but no card identity. See [`parse`].
#[must_use]
pub fn parse_with_catalogs(source: &str, catalogs: &Catalogs) -> ParseReport {
    parse_internal(source, catalogs, &SelfReference::default())
}

/// Parses Oracle text as a specific face, so the face's own name — written in
/// full or as its shortened self-reference — is recognized as a self-reference
/// rather than left as opaque residue. Render the resulting tree with the same
/// `name` and `is_legendary` to round-trip.
#[must_use]
pub fn parse_with_identity(
    source: &str,
    catalogs: &Catalogs,
    name: &str,
    is_legendary: bool,
) -> ParseReport {
    parse_internal(source, catalogs, &SelfReference::new(name, is_legendary))
}

fn parse_internal(
    source: &str,
    catalogs: &Catalogs,
    self_reference: &SelfReference,
) -> ParseReport {
    let surface = lex(source);
    // The corpus metric counts the name-bearing tokenization; the full-name
    // collapse below is an internal step that keeps the structural splitter
    // from dividing a comma-bearing self-reference.
    let source_tokens = surface.tokens.len();
    let tokens = collapse_full_names(source, surface.tokens, self_reference.full_name());
    let parsed = parse_oracle_text(source, catalogs, &tokens, self_reference);
    let mut diagnostics = surface
        .diagnostics
        .into_iter()
        .map(|diagnostic| Diagnostic {
            kind: DiagnosticKind::Surface(diagnostic.kind),
            span: diagnostic.span,
        })
        .collect::<Vec<_>>();
    diagnostics.extend(parsed.diagnostics.into_iter().map(|diagnostic| Diagnostic {
        kind: match diagnostic.kind {
            AbilityDiagnosticKind::OrphanMode => DiagnosticKind::OrphanMode,
            AbilityDiagnosticKind::EmptyActivationEffect => DiagnosticKind::EmptyActivationEffect,
            AbilityDiagnosticKind::NoCompleteParse => DiagnosticKind::NoCompleteParse,
        },
        span: diagnostic.span,
    }));
    ParseReport {
        ast: parsed.ast,
        ability_spans: parsed.ability_spans,
        diagnostics,
        provenance: ParseProvenance {
            selections: parsed
                .selections
                .into_iter()
                .map(|selection| ParseSelection {
                    span: selection.span,
                    constituent_spans: selection.constituent_spans,
                    rule: selection.rule,
                    construction: selection.construction,
                    constructions: selection.constructions,
                    tied_alternatives: selection.tied_alternatives,
                    cost: selection.cost,
                    chart_stats: selection.chart_stats,
                    forest_stats: selection.forest_stats,
                })
                .collect(),
        },
        source_tokens,
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn input_can_be_dropped_before_the_ast_is_rendered() {
        let source = String::from("Draw a card.");
        let report = parse(&source);
        assert!(!report.provenance.selections.is_empty());

        let ast = report.into_ast();
        drop(source);

        assert_eq!(ast.render("Test Card", false).unwrap(), "Draw a card.");
    }

    #[test]
    fn provenance_identifies_selected_rules_without_copying_source() {
        let report = parse("Draw a card.");
        assert!(report.diagnostics.is_empty());
        assert!(report.provenance.selections.iter().all(|selection| {
            selection.span == Span::new(0, 12)
                && selection.rule.is_some()
                && selection.tied_alternatives.len() == 1
                && selection.cost == crate::forest::ParseCost::default()
        }));
    }

    #[test]
    fn proform_selection_does_not_duplicate_do_on_render() {
        let source = "All creatures with flying able to block this creature do so.";
        let rendered = parse(source)
            .into_ast()
            .render("Test Card", false)
            .expect("selected syntax must render");
        assert_eq!(rendered, source);
    }
}
