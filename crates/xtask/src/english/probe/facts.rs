//! A parse reduced to plain data.
//!
//! `ParseReport` cannot be constructed outside `deckmaste_english`, so an
//! invariant taking `&ParseReport` could only be tested against real
//! pathological text — which is what this instrument exists to *find*.
//! `ParseFacts` is data xtask owns, so every law gets a hand-built passing case
//! and a hand-built violating case.

use deckmaste_english::Catalogs;
use deckmaste_english::Span;
use deckmaste_english::parse_with_identity;

use super::super::shape::Shape;
use super::super::shape::{self};

#[derive(Debug, Clone, PartialEq, Eq)]
pub(super) struct TieFact {
    pub(super) span: Span,
    pub(super) rule: Option<usize>,
    pub(super) alternatives: usize,
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub(super) struct AbilityFact {
    pub(super) span: Span,
    pub(super) text: String,
    pub(super) fingerprint: String,
}

#[derive(Debug, Clone, Default, PartialEq, Eq)]
pub(super) struct ParseFacts {
    pub(super) text: String,
    pub(super) source_tokens: usize,
    pub(super) ties: Vec<TieFact>,
    pub(super) max_alternatives: usize,
    pub(super) constituent_nodes: usize,
    pub(super) chart_items: usize,
    pub(super) abilities: Vec<AbilityFact>,
    pub(super) recovered: Vec<String>,
}

/// Parse `text` as anonymous, non-legendary card text and reduce it to facts.
///
/// The identity arguments are deliberately fixed: a synthesized composition has
/// no printed name, and varying legendary-ness would make two runs of the same
/// text incomparable.
pub(super) fn facts_for(text: &str, catalogs: &Catalogs) -> ParseFacts {
    let report = parse_with_identity(text, catalogs, "", false);

    let mut ties = Vec::new();
    let mut max_alternatives = 0;
    let mut constituent_nodes = 0;
    let mut chart_items = 0;
    for selection in report.provenance().selections() {
        if !selection.tied_alternatives().is_empty() {
            ties.push(TieFact {
                span: selection.span(),
                rule: selection.rule(),
                alternatives: selection.tied_alternatives().len(),
            });
        }
        max_alternatives = max_alternatives.max(selection.forest_stats().max_alternatives());
        constituent_nodes = constituent_nodes.max(selection.forest_stats().constituent_nodes());
        chart_items = chart_items.max(selection.chart_stats().unique_items());
    }

    let whole = shape::of(report.ast());
    let recovered = recovered_sites(&whole);

    // Iterate the AST's abilities, not the spans: a span/node count mismatch
    // must not silently truncate abilities out of the facts, which is what
    // zipping the two would do.
    let spans = report.ability_spans();
    let abilities = report
        .ast()
        .abilities
        .iter()
        .enumerate()
        .map(|(index, ability)| AbilityFact {
            span: spans.get(index).copied().unwrap_or_default(),
            text: spans
                .get(index)
                .and_then(|span| span.text(text))
                .unwrap_or_default()
                .to_string(),
            // IN CONTEXT: the subtree this ability got inside the joint parse.
            // Invariant 2 compares it against a solo re-parse, so this side must
            // NOT re-parse — see the module docs.
            fingerprint: shape::of(ability).fingerprint(),
        })
        .collect();

    ParseFacts {
        text: text.to_string(),
        source_tokens: report.source_tokens(),
        ties,
        max_alternatives,
        constituent_nodes,
        chart_items,
        abilities,
        recovered,
    }
}

/// Every `"Recovered"`-labeled node in `shape`, counting each recovery
/// *site* once.
///
/// Every recovery variant in the AST — `SentenceBody::Recovered`,
/// `CostComponent::Recovered`, `KeywordArgument::Recovered`,
/// `Phrase::Recovered` — wraps the same leaf, `syntax::RecoveredText`,
/// directly as its own immediate payload (none of them ever wraps another
/// `Recovered`-named variant). [`Shape::walk`]'s visitor takes no return
/// value, so it cannot be told to stop descending once it finds a match;
/// recording every matching label with it therefore sees each site twice —
/// once for the wrapper (e.g. `"SentenceBody::Recovered"`) and once for the
/// `"RecoveredText"` leaf one level inside it.
///
/// This walks by hand instead, mirroring [`Shape::walk`]'s traversal, with
/// one difference: once a node's own label matches, it is recorded and its
/// children are not descended into, so the payload a matched wrapper holds
/// is never visited. The wrapper's label is what's kept, and deliberately
/// so, not an arbitrary pick between the two: it says *where* in the
/// grammar recovery happened (a sentence, a cost component, a keyword
/// argument, ...), where the leaf only ever says "some text failed to
/// parse" — true at every site and distinguishing none of them.
///
/// Two independent recovery sites are unaffected: pruning only stops
/// descent *into* a matched node, so sibling matches elsewhere in the tree
/// (e.g. two separately-unparsed cost alternatives either side of an `or`)
/// are still both found.
fn recovered_sites(shape: &Shape) -> Vec<String> {
    let mut found = Vec::new();
    collect_recovered(shape, &mut found);
    found
}

fn collect_recovered(shape: &Shape, found: &mut Vec<String>) {
    let label = shape.label();
    if label.contains("Recovered") {
        found.push(label);
        return;
    }
    match shape {
        Shape::Newtype { inner, .. } => collect_recovered(inner, found),
        Shape::Node { fields, .. } => {
            for (_, child) in fields {
                collect_recovered(child, found);
            }
        }
        Shape::Seq(items) => {
            for item in items {
                collect_recovered(item, found);
            }
        }
        Shape::Map(entries) => {
            for (key, value) in entries {
                collect_recovered(key, found);
                collect_recovered(value, found);
            }
        }
        Shape::Scalar(_) | Shape::Unit { .. } | Shape::Absent => {}
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::english::data::OracleDataArgs;

    /// Catalogs are required for a faithful parse; build them from the snapshot
    /// the same way every other english subcommand does.
    fn catalogs() -> deckmaste_english::Catalogs {
        OracleDataArgs::default()
            .load()
            .expect("derived card data must be generated")
            .catalogs
    }

    #[test]
    fn facts_expose_one_ability_per_sentence() {
        let facts = facts_for("Draw a card.", &catalogs());
        assert_eq!(facts.abilities.len(), 1);
        assert_eq!(facts.abilities[0].text, "Draw a card.");
        assert!(!facts.abilities[0].fingerprint.is_empty());
        assert_eq!(facts.text, "Draw a card.");
        assert!(facts.source_tokens > 0);
    }

    #[test]
    fn facts_separate_two_abilities() {
        let facts = facts_for("Draw a card.\nYou gain 1 life.", &catalogs());
        assert_eq!(facts.abilities.len(), 2, "each ability gets its own fact");
        assert_ne!(
            facts.abilities[0].fingerprint, facts.abilities[1].fingerprint,
            "different abilities must not share a fingerprint"
        );
    }

    /// The comparison invariant 2 actually performs, on text that should pass.
    ///
    /// This is the test that catches a fingerprint accidentally computed from a
    /// solo re-parse: if it were, this assertion would hold vacuously, so it is
    /// paired with the one below.
    #[test]
    fn benign_abilities_parse_the_same_in_context_as_alone() {
        let catalogs = catalogs();
        let joint = facts_for("Draw a card.\nYou gain 1 life.", &catalogs);
        for ability in &joint.abilities {
            let alone = facts_for(&ability.text, &catalogs);
            assert_eq!(
                ability.fingerprint, alone.abilities[0].fingerprint,
                "`{}` must parse the same beside its neighbor as alone",
                ability.text
            );
        }
    }

    /// Proof the fingerprint is context-sensitive rather than a solo re-parse.
    ///
    /// A single ability's in-context fingerprint must equal the fingerprint of
    /// the AST node itself — not of the whole `OracleText` wrapper. If
    /// `facts_for` were re-parsing the slice and fingerprinting the wrapper,
    /// the label would be `OracleText{…}` and this fails.
    #[test]
    fn a_fingerprint_is_the_ability_node_not_the_wrapper() {
        let facts = facts_for("Draw a card.", &catalogs());
        assert!(
            !facts.abilities[0].fingerprint.starts_with("OracleText"),
            "fingerprint must be the ability subtree, got {}",
            facts.abilities[0].fingerprint
        );
    }

    /// F2: `SentenceBody::Recovered(RecoveredText)` is one recovery site, not
    /// two. A naive walk records every node whose label contains
    /// `"Recovered"`, and the wrapper and the leaf it directly holds both
    /// qualify, so one unparsed sentence used to surface as two entries in
    /// `facts.recovered`. This exact text is confirmed (via a real corpus
    /// sweep) to fall back to whole-sentence recovery under the parser's
    /// anonymous, non-legendary identity — the same identity `facts_for`
    /// always parses under.
    #[test]
    fn a_recovery_site_is_not_double_counted_by_its_wrapper_and_payload() {
        let facts = facts_for("Aang, Aang enters, and La attack", &catalogs());
        assert_eq!(
            facts.recovered.len(),
            1,
            "one unparsed sentence is one recovery site, not two: {:?}",
            facts.recovered
        );
        assert_eq!(
            facts.recovered[0], "SentenceBody::Recovered",
            "the wrapper's label is kept: it says which grammar role recovered \
             (a sentence, here); the `RecoveredText` leaf inside it says only \
             that something failed to parse, which is true at every site and \
             distinguishes none of them"
        );
    }
}
