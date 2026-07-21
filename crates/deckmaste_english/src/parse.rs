use crate::Span;
use crate::catalog::Catalogs;
use crate::grammar::ability::AbilityDiagnosticKind;
use crate::grammar::ability::parse_oracle_text;
use crate::surface::SurfaceDiagnosticKind;
use crate::surface::lex;
use crate::syntax::OracleText;

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub(crate) enum DiagnosticKind {
    Surface(SurfaceDiagnosticKind),
    OrphanMode,
    EmptyActivationEffect,
    NoCompleteParse,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub(crate) struct Diagnostic {
    pub(crate) kind: DiagnosticKind,
    pub(crate) span: Span,
}

#[derive(Debug, Clone, Default, PartialEq, Eq)]
pub(crate) struct ParseProvenance {
    pub(crate) selections: Vec<ParseSelection>,
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub(crate) struct ParseSelection {
    pub(crate) span: Span,
    pub(crate) rule: Option<usize>,
    pub(crate) tied_alternatives: Vec<usize>,
    pub(crate) cost: crate::forest::ParseCost,
}

#[derive(Debug)]
pub(crate) struct ParseReport {
    pub(crate) ast: OracleText,
    pub(crate) diagnostics: Vec<Diagnostic>,
    pub(crate) provenance: ParseProvenance,
}

impl ParseReport {
    pub(crate) fn into_ast(self) -> OracleText {
        self.ast
    }
}

pub(crate) fn parse(source: &str) -> ParseReport {
    parse_with_catalogs(source, &Catalogs::default())
}

pub(crate) fn parse_with_catalogs(source: &str, catalogs: &Catalogs) -> ParseReport {
    let surface = lex(source);
    let parsed = parse_oracle_text(source, catalogs, &surface.tokens);
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
        diagnostics,
        provenance: ParseProvenance {
            selections: parsed
                .selections
                .into_iter()
                .map(|selection| ParseSelection {
                    span: selection.span,
                    rule: selection.rule,
                    tied_alternatives: selection.tied_alternatives,
                    cost: selection.cost,
                })
                .collect(),
        },
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
                && !selection.tied_alternatives.is_empty()
                && selection.cost == crate::forest::ParseCost::default()
        }));
    }
}
