//! Parsing and rendering *fragments* — one grammatical category standing on
//! its own, rather than a whole card's oracle text.
//!
//! [`parse`](crate::parse) and
//! [`OracleText::render`](crate::syntax::OracleText) are whole-card entries:
//! they take a card's complete text and give back a complete tree. A
//! bidirectional English *template* needs less than that. A template is
//! authored at one category — a nominal phrase, a sentence, an activation cost
//! — and must both match oracle text and render back. This module is that door,
//! and nothing more: every entry here wraps an internal function the whole-card
//! path already uses, at the same seam, with the same flags. No new parsing, no
//! new rendering, no new AST.
//!
//! # Reminder text is deliberately not stripped
//!
//! [`strip_reminder_text`](crate::strip_reminder_text) removes every balanced
//! `(…)` group from a string. It is applied at the *corpus* boundary
//! (`crates/xtask/src/english/data.rs`), never inside [`parse`](crate::parse),
//! and [`parse_fragment`] does not apply it either. Two reasons:
//!
//! 1. A template is authored text, not printed card text. Parenthesized
//!    material in a template is part of the template — stripping it would
//!    silently rewrite what the author wrote. The hazard is concrete: a hole
//!    sigil spelled `<Param(0)>` reads as `<Param>` followed by a reminder
//!    group, so `strip_reminder_text` turns it into `<Param>` and destroys the
//!    hole's index.
//! 2. It would be redundant anyway. A frame compiler substitutes every hole
//!    sigil for a reserved witness token *before* calling [`parse_fragment`],
//!    so by the time text reaches here there is no sigil left to protect — and
//!    a template that genuinely wants a parenthesized run wants it preserved.
//!
//! A caller that does want printed-card normalization applies
//! [`strip_reminder_text`](crate::strip_reminder_text) itself, exactly as the
//! corpus loader does.

use crate::Diagnostic;
use crate::DiagnosticKind;
use crate::Span;
use crate::catalog::Catalogs;
use crate::grammar::GeneratedActivation;
use crate::grammar::Nonterminal;
use crate::grammar::ability::AbilityDiagnostic;
use crate::grammar::ability::AbilityDiagnosticKind;
use crate::grammar::ability::parse_ability_fragment;
use crate::grammar::ability::parse_ability_fragment_with_activation;
use crate::grammar::ability::parse_cost_fragment;
use crate::grammar::ability::parse_cost_fragment_with_activation;
use crate::grammar::ability::parse_keyword_line_fragment;
use crate::grammar::ability::parse_keyword_line_fragment_with_activation;
use crate::grammar::parse_nonterminal_with_self_reference_and_activation;
use crate::identity::SelfReference;
use crate::renderer::RenderError;
use crate::surface::collapse_full_names;
use crate::surface::lex;
use crate::syntax::Ability;
use crate::syntax::Cost;
use crate::syntax::KeywordAbilityList;
use crate::syntax::NounPhrase;
use crate::syntax::Sentence;

/// The grammatical category a fragment is parsed and rendered at.
///
/// The five are not an arbitrary selection: they are the categories a template
/// can be authored at today, split by which parser layer owns them.
/// [`Self::Nominal`] and [`Self::Sentence`] are chart nonterminals;
/// [`Self::Cost`], [`Self::KeywordLine`] and [`Self::Ability`] have **no**
/// chart rules at all and live entirely in the hand-written ability layer.
// Serialization remains available to diagnostic shape tooling. Spelling uses
// the typed construction projection and makes no contract from this layout.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, serde::Serialize)]
pub enum FragmentKind {
    /// A noun phrase — `creature you control`, `a 1/1 white Soldier creature
    /// token`. Parsed at `Nonterminal::NounPhrase`, whose rule set already
    /// covers the bare `Nominal` (`NounPhrase → Nominal`), pronouns,
    /// self-references, quantities and coordination, so this one category
    /// reaches the whole nominal space rather than a slice of it.
    Nominal,
    /// One sentence — `Draw a card.` — with or without its terminal period.
    /// Parsed at `Nonterminal::Sentence`, whose two rules (`Sentence → Clause
    /// '.'`, `Sentence → Clause`) accept and discard the period; the renderer
    /// re-derives it.
    Sentence,
    /// An activation-cost line — `{2}{W}, Sacrifice this artifact` — the run
    /// left of an activated ability's colon.
    Cost,
    /// A keyword-ability line — `Flying`, `Flying, first strike`, `Ward {2}`.
    KeywordLine,
    /// One whole ability: a single oracle-text line in any of its frames
    /// (activated, triggered, keyword, plain paragraph). Multi-line bodies —
    /// a modal ability's bulleted modes, a level band's stat rows — are out of
    /// scope, exactly as they are for a quoted ability's interior.
    Ability,
}

/// A parsed fragment: one typed subtree, tagged with the category it was
/// parsed at.
///
/// Every payload is an ordinary `crate::syntax` type — the same node the
/// whole-card tree holds in that position — so a fragment can be spliced into
/// a card tree, or have a card tree's subtree spliced into it, with no
/// conversion.
#[allow(
    clippy::large_enum_variant,
    reason = "each payload is the concrete-syntax node the whole-card tree holds in that \
              position, boxed nowhere else; adding indirection solely to equalize variant \
              sizes would make a fragment a different value from the subtree it splices into"
)]
// Serialization remains diagnostic only; every wrapped syntax node already
// derives it, but spelling does not inspect the resulting layout.
#[derive(Debug, Clone, PartialEq, Eq, serde::Serialize)]
pub enum Fragment {
    /// See [`FragmentKind::Nominal`].
    Nominal(NounPhrase),
    /// See [`FragmentKind::Sentence`].
    Sentence(Sentence),
    /// See [`FragmentKind::Cost`].
    Cost(Cost),
    /// See [`FragmentKind::KeywordLine`].
    KeywordLine(KeywordAbilityList),
    /// See [`FragmentKind::Ability`].
    Ability(Ability),
}

impl Fragment {
    /// The category this fragment was parsed at.
    #[must_use]
    pub const fn kind(&self) -> FragmentKind {
        match self {
            Self::Nominal(_) => FragmentKind::Nominal,
            Self::Sentence(_) => FragmentKind::Sentence,
            Self::Cost(_) => FragmentKind::Cost,
            Self::KeywordLine(_) => FragmentKind::KeywordLine,
            Self::Ability(_) => FragmentKind::Ability,
        }
    }
}

/// The result of parsing one fragment.
///
/// The whole-card [`ParseReport`](crate::ParseReport) has no failure mode —
/// parsing is total and a failure surfaces as recovery. A fragment report
/// keeps that property for the ability-layer categories and adds an explicit
/// `None` for the chart categories, which genuinely can decline. Callers that
/// need a fragment to be *right*, not merely present, should gate on
/// [`Self::clean`].
#[derive(Debug)]
pub struct FragmentReport {
    kind: FragmentKind,
    fragment: Option<Fragment>,
    diagnostics: Vec<Diagnostic>,
    construction_decisions: Vec<crate::ConstructionDecision>,
}

impl FragmentReport {
    /// The category this fragment was requested at, whether or not one was
    /// produced.
    #[must_use]
    pub const fn kind(&self) -> FragmentKind {
        self.kind
    }

    /// The parsed subtree, or `None` when the category declined the text
    /// outright.
    #[must_use]
    pub const fn fragment(&self) -> Option<&Fragment> {
        self.fragment.as_ref()
    }

    /// Takes ownership of the parsed subtree, dropping the diagnostics.
    ///
    /// The counterpart to
    /// [`ParseReport::into_ast`](crate::ParseReport::into_ast), and needed for
    /// the same reason: a frame compiler *stores* the compiled tree past the
    /// report and past the source string, so borrowing from
    /// [`Self::fragment`] would force a clone of the whole subtree.
    #[must_use]
    pub fn into_fragment(self) -> Option<Fragment> {
        self.fragment
    }

    /// Surface and parse diagnostics, in the same shape
    /// [`ParseReport::diagnostics`](crate::ParseReport::diagnostics) reports
    /// them.
    #[must_use]
    pub fn diagnostics(&self) -> &[Diagnostic] {
        &self.diagnostics
    }

    /// Generated construction selections made inside the fragment, including
    /// chart subtrees nested by the ability layer.
    #[must_use]
    pub fn construction_decisions(&self) -> &[crate::ConstructionDecision] {
        &self.construction_decisions
    }

    /// Every recovered span inside the parsed subtree, or empty when there is
    /// no subtree.
    ///
    /// This is the only channel that reports *what* went wrong when
    /// [`Self::clean`] is false but [`Self::diagnostics`] is empty — a quoted
    /// ability's interior is parsed by a nested parser whose diagnostics never
    /// reach this report, so the recovered spelling is all a caller has to name
    /// in an error message.
    #[must_use]
    pub fn recoveries(&self) -> Vec<crate::syntax::RecoveryRef<'_>> {
        self.fragment
            .as_ref()
            .map(Fragment::recoveries)
            .unwrap_or_default()
    }

    /// Whether this text parsed *completely* at its category: a subtree was
    /// produced, no diagnostic was raised, and nothing anywhere inside it was
    /// recovered verbatim.
    ///
    /// All three conditions are load-bearing and none implies the others. The
    /// ability layer is total, so it answers an unparseable run with a
    /// `RecoveredText`-bearing node rather than a failure — a fragment can be
    /// `Some` and still be rubbish. And a quoted ability's interior is parsed
    /// by a nested `Parser` whose diagnostics never reach this report, so the
    /// recovery walk catches what the diagnostic list cannot.
    ///
    /// Lexical *opacity* — an unknown noun kept explicit rather than guessed —
    /// is deliberately **not** a defect here: it is how the crate represents
    /// vocabulary it does not know, and the supported corpus is full of it. A
    /// fragment whose noun is not in the catalogs is therefore clean, and
    /// round-trips byte-exactly. No opacity reader is exposed for a fragment
    /// until a caller needs one; the whole-card
    /// [`OracleText::lexical_opacity`](crate::syntax::OracleText) is the
    /// existing precedent to follow if one does.
    #[must_use]
    pub fn clean(&self) -> bool {
        self.diagnostics.is_empty()
            && self
                .fragment
                .as_ref()
                .is_some_and(|fragment| fragment.recoveries().is_empty())
    }
}

/// Parses `source` as a single fragment of the given category.
///
/// `name` and `is_legendary` are the face identity, exactly as
/// [`parse_with_identity`](crate::parse_with_identity) takes them: they decide
/// which spellings count as a self-reference. Pass `("", false)` for text
/// known to contain none, which is what [`parse`](crate::parse) does.
///
/// Which internal function each category wraps:
///
/// | category | wraps |
/// |---|---|
/// | [`FragmentKind::Nominal`] | `grammar::parse_nonterminal_with_self_reference(.., Nonterminal::NounPhrase)` → `ParsedNonterminal::noun_phrase` |
/// | [`FragmentKind::Sentence`] | `grammar::parse_nonterminal_with_self_reference(.., Nonterminal::Sentence)` → `ParsedNonterminal::sentence` |
/// | [`FragmentKind::Cost`] | `grammar::ability::parse_cost_fragment` → `Parser::parse_cost` |
/// | [`FragmentKind::KeywordLine`] | generated `keyword_line` ability family |
/// | [`FragmentKind::Ability`] | `grammar::ability::parse_ability_fragment` → `Parser::parse_ability` |
///
/// The two chart categories go through the *same* entry the whole-card path
/// reaches them by (`Parser::parse_exact` calls it too), so they inherit its
/// `OpacityMode::Exact`-then-`OpaqueNouns` retry unchanged rather than picking
/// a fragment-only opacity policy.
///
/// Reminder text is **not** stripped — see the [module
/// documentation](self#reminder-text-is-deliberately-not-stripped).
#[must_use]
pub fn parse_fragment(
    source: &str,
    catalogs: &Catalogs,
    kind: FragmentKind,
    name: &str,
    is_legendary: bool,
) -> FragmentReport {
    parse_fragment_with_generated_activation(
        source,
        catalogs,
        kind,
        name,
        is_legendary,
        GeneratedActivation::Production,
    )
}

#[cfg(test)]
pub(crate) fn parse_fragment_with_activation(
    source: &str,
    catalogs: &Catalogs,
    kind: FragmentKind,
    name: &str,
    is_legendary: bool,
    activation: GeneratedActivation,
) -> FragmentReport {
    parse_fragment_with_generated_activation(source, catalogs, kind, name, is_legendary, activation)
}

fn parse_fragment_with_generated_activation(
    source: &str,
    catalogs: &Catalogs,
    kind: FragmentKind,
    name: &str,
    is_legendary: bool,
    activation: GeneratedActivation,
) -> FragmentReport {
    let self_reference = SelfReference::new(name, is_legendary);
    // Lexed here for two reasons: the ability-layer seams take prepared
    // tokens, and the chart entry lexes privately and drops the surface
    // diagnostics, which a template author wants to see. The chart categories
    // therefore lex twice — a fragment is a few dozen tokens, and the
    // alternative is widening a chart internal for no behavioural gain.
    let surface = lex(source);
    let mut diagnostics = surface
        .diagnostics
        .iter()
        .map(|diagnostic| Diagnostic {
            kind: DiagnosticKind::Surface(diagnostic.kind),
            span: diagnostic.span,
        })
        .collect::<Vec<_>>();
    let tokens = collapse_full_names(source, surface.tokens, self_reference.full_name());
    let mut construction_decisions = Vec::new();

    let mut fragment = match kind {
        FragmentKind::Nominal => chart_fragment(
            source,
            catalogs,
            Nonterminal::NounPhrase,
            &self_reference,
            activation,
            |parsed| parsed.noun_phrase().cloned().map(Fragment::Nominal),
        )
        .and_then(|(fragment, decisions)| {
            construction_decisions = decisions;
            fragment
        }),
        FragmentKind::Sentence => chart_fragment(
            source,
            catalogs,
            Nonterminal::Sentence,
            &self_reference,
            activation,
            |parsed| parsed.sentence().cloned().map(Fragment::Sentence),
        )
        .and_then(|(fragment, decisions)| {
            construction_decisions = decisions;
            fragment
        }),
        FragmentKind::Cost => {
            let parsed = if activation.is_production() {
                parse_cost_fragment(source, catalogs, &tokens, &self_reference)
            } else {
                parse_cost_fragment_with_activation(
                    source,
                    catalogs,
                    &tokens,
                    &self_reference,
                    activation,
                )
            };
            diagnostics.extend(parsed.diagnostics.iter().map(ability_diagnostic));
            construction_decisions = parsed.constructions;
            Some(Fragment::Cost(parsed.value))
        }
        FragmentKind::KeywordLine => {
            let parsed = if activation.is_production() {
                parse_keyword_line_fragment(source, catalogs, &tokens, &self_reference)
            } else {
                parse_keyword_line_fragment_with_activation(
                    source,
                    catalogs,
                    &tokens,
                    &self_reference,
                    activation,
                )
            };
            diagnostics.extend(parsed.diagnostics.iter().map(ability_diagnostic));
            construction_decisions = parsed.constructions;
            parsed.value.map(Fragment::KeywordLine)
        }
        FragmentKind::Ability => {
            let parsed = if activation.is_production() {
                parse_ability_fragment(source, catalogs, &tokens, &self_reference)
            } else {
                parse_ability_fragment_with_activation(
                    source,
                    catalogs,
                    &tokens,
                    &self_reference,
                    activation,
                )
            };
            diagnostics.extend(parsed.diagnostics.iter().map(ability_diagnostic));
            construction_decisions = parsed.constructions;
            Some(Fragment::Ability(parsed.value))
        }
    };

    if let Some(Fragment::Sentence(sentence)) = &fragment
        && !crate::renderer::sentence_form_is_admitted(
            sentence,
            source.ends_with('.'),
            name,
            is_legendary,
            false,
        )
    {
        fragment = None;
    }

    // A category that declines outright raises no diagnostic of its own — the
    // chart returns an error and the keyword-line frame returns `None`. Mint
    // one so an unclean fragment is unclean for one uniform, inspectable
    // reason rather than by the absence of a subtree.
    if fragment.is_none() {
        diagnostics.push(Diagnostic {
            kind: DiagnosticKind::NoCompleteParse,
            span: Span::new(0, source.len()),
        });
    }

    FragmentReport {
        kind,
        fragment,
        diagnostics,
        construction_decisions,
    }
}

/// Renders a fragment back to Oracle text.
///
/// Dispatches to `renderer::render_fragment`, which selects the private
/// `Renderer` method the whole-card path uses for that node kind and passes
/// the flags a card passes when the node heads its ability. No render logic is
/// duplicated: the byte-for-byte behaviour of every existing path is the
/// behaviour a fragment gets.
///
/// `name` and `is_legendary` are the face identity, exactly as
/// [`OracleText::render`](crate::syntax::OracleText) takes them; they are
/// consulted only by a self-reference node.
///
/// A fragment renders as if it headed its own ability — see
/// `renderer::render_fragment`'s *Position* section for the one card-internal
/// position (a triggered ability's effect) whose capitalization differs, and
/// which therefore has to be rendered as part of its
/// [`FragmentKind::Ability`].
///
/// # Errors
///
/// Returns an error when the fragment requests a grammatical form its
/// vocabulary identity does not define — the same conditions
/// [`OracleText::render`](crate::syntax::OracleText) reports.
pub fn render_fragment(
    fragment: &Fragment,
    name: &str,
    is_legendary: bool,
) -> Result<String, RenderError> {
    crate::renderer::render_fragment(fragment, name, is_legendary)
}

/// Runs a chart category and lowers its result to a [`Fragment`], mapping both
/// a parse failure and a category mismatch to `None`.
fn chart_fragment(
    source: &str,
    catalogs: &Catalogs,
    nonterminal: Nonterminal,
    self_reference: &SelfReference,
    activation: GeneratedActivation,
    lower: impl FnOnce(&crate::grammar::ParsedNonterminal) -> Option<Fragment>,
) -> Option<(Option<Fragment>, Vec<crate::ConstructionDecision>)> {
    let parsed = parse_nonterminal_with_self_reference_and_activation(
        source,
        catalogs,
        nonterminal,
        self_reference,
        activation,
    )
    .ok()?;
    let decisions = parsed.construction_decisions().to_vec();
    Some((lower(&parsed), decisions))
}

fn ability_diagnostic(diagnostic: &AbilityDiagnostic) -> Diagnostic {
    Diagnostic {
        kind: match diagnostic.kind {
            AbilityDiagnosticKind::OrphanMode => DiagnosticKind::OrphanMode,
            AbilityDiagnosticKind::EmptyActivationEffect => DiagnosticKind::EmptyActivationEffect,
            AbilityDiagnosticKind::NoCompleteParse => DiagnosticKind::NoCompleteParse,
        },
        span: diagnostic.span,
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::catalog::CatalogKind;

    /// Bare `parse`-mode catalogs: the corpus supplies these externally, so a
    /// fragment test that uses corpus vocabulary must supply them too.
    fn catalogs() -> Catalogs {
        Catalogs::default()
            .with_catalog(CatalogKind::CardType, ["Artifact", "Creature", "Land"])
            .with_catalog(CatalogKind::CreatureType, ["Soldier"])
            .with_catalog(
                CatalogKind::KeywordAbility,
                ["Flying", "First strike", "Ward"],
            )
    }

    fn production_fragment(source: &str, kind: FragmentKind) -> FragmentReport {
        parse_fragment_with_activation(
            source,
            &catalogs(),
            kind,
            "",
            false,
            GeneratedActivation::Production,
        )
    }

    fn is_imperative_transitive(clause: &crate::syntax::IndependentClause) -> bool {
        clause.subject().is_none()
            && matches!(
                clause.predicate_expression(),
                Some(crate::syntax::PredicateExpression::Simple(
                    crate::syntax::Predicate::Transitive(_)
                ))
            )
    }

    fn is_deontic_with_inner(clause: &crate::syntax::IndependentClause) -> bool {
        clause.subject().is_some()
            && matches!(
                clause.predicate_expression(),
                Some(crate::syntax::PredicateExpression::Simple(
                    crate::syntax::Predicate::Deontic(predicate)
                )) if predicate.inner().is_some()
            )
    }

    fn assert_generated<'a>(
        report: &'a FragmentReport,
        construction: &str,
    ) -> &'a crate::ConstructionDecision {
        report
            .construction_decisions()
            .iter()
            .find(|decision| decision.selected().as_str() == construction)
            .unwrap_or_else(|| {
                panic!(
                    "missing generated {construction} decision: {:#?}",
                    report.construction_decisions()
                )
            })
    }

    #[test]
    fn cost_is_generated_on_the_ability_backend_and_activation_is_explicit() {
        let report = production_fragment("Exhaust — {2}{R}", FragmentKind::Cost);
        let decision = assert_generated(&report, "cost");
        assert_eq!(decision.backend(), crate::ConstructionBackend::Ability);
        assert_eq!(decision.evidence().label(), "activation-cost root");
        assert_eq!(decision.span(), Span::new(0, "Exhaust — {2}{R}".len()));

        let empty_assembly = parse_fragment_with_activation(
            "{T}",
            &catalogs(),
            FragmentKind::Cost,
            "",
            false,
            GeneratedActivation::Groups(&[]),
        );
        assert!(
            empty_assembly
                .construction_decisions()
                .iter()
                .all(|decision| decision.selected().as_str() != "cost")
        );

        let isolated = parse_fragment_with_activation(
            "{T}",
            &catalogs(),
            FragmentKind::Cost,
            "",
            false,
            GeneratedActivation::Groups(crate::constructions::ability::GROUPS),
        );
        assert_eq!(
            assert_generated(&isolated, "cost").backend(),
            crate::ConstructionBackend::Ability
        );
    }

    #[test]
    fn keyword_line_is_generated_and_registration_order_neutral() {
        let source = "Ward—Discard a card. Draw a card.";
        let activations = [
            GeneratedActivation::Groups(crate::constructions::ability::GROUPS),
            GeneratedActivation::GroupsReversed(crate::constructions::ability::GROUPS),
            GeneratedActivation::GroupsFixedShuffle(crate::constructions::ability::GROUPS),
        ];
        let reports = activations.map(|activation| {
            parse_fragment_with_activation(
                source,
                &catalogs(),
                FragmentKind::KeywordLine,
                "",
                false,
                activation,
            )
        });
        let expected_fragment = reports[0].fragment().cloned();
        let expected_decisions = reports[0].construction_decisions().to_vec();
        let expected_diagnostics = reports[0].diagnostics().to_vec();
        let expected_surface = render_fragment(
            reports[0].fragment().expect("normal keyword line parses"),
            "",
            false,
        )
        .unwrap();
        for report in &reports {
            assert_eq!(report.fragment(), expected_fragment.as_ref());
            assert_eq!(report.construction_decisions(), expected_decisions);
            assert_eq!(report.diagnostics(), expected_diagnostics);
            assert_eq!(
                render_fragment(report.fragment().unwrap(), "", false).unwrap(),
                expected_surface,
            );
            let decision = assert_generated(report, "keyword_line");
            assert_eq!(decision.backend(), crate::ConstructionBackend::Ability);
            assert_eq!(decision.evidence().label(), "keyword-ability list root");
        }

        let empty_assembly = parse_fragment_with_activation(
            "Flying",
            &catalogs(),
            FragmentKind::KeywordLine,
            "",
            false,
            GeneratedActivation::Groups(&[]),
        );
        assert!(
            empty_assembly
                .construction_decisions()
                .iter()
                .all(|decision| decision.selected().as_str() != "keyword_line")
        );
    }

    #[test]
    fn ability_is_generated_with_direct_kinds_and_registration_order_neutral() {
        let fixtures = [
            ("{T}: Draw a card.", "activated"),
            ("{2}: Level 2", "class_level"),
            ("I — Draw a card.", "chapter"),
            ("2–9 | Draw a card.", "roll_row"),
            ("Whenever this creature attacks, draw a card.", "triggered"),
            ("[+1]: Draw a card.", "loyalty"),
            ("Flying", "keyword"),
            ("Draw a card.", "paragraph"),
        ];
        let activations = [
            GeneratedActivation::Groups(crate::constructions::ALL_GROUPS),
            GeneratedActivation::GroupsReversed(crate::constructions::ALL_GROUPS),
            GeneratedActivation::GroupsFixedShuffle(crate::constructions::ALL_GROUPS),
        ];
        for (source, expected_kind) in fixtures {
            let reports = activations.map(|activation| {
                parse_fragment_with_activation(
                    source,
                    &catalogs(),
                    FragmentKind::Ability,
                    "Test Card",
                    false,
                    activation,
                )
            });
            for report in &reports {
                let decision = assert_generated(report, "ability");
                assert_eq!(decision.backend(), crate::ConstructionBackend::Ability);
                assert_eq!(decision.evidence().label(), "decisive ability frame guard");
                let Some(Fragment::Ability(ability)) = report.fragment() else {
                    panic!("ability fixture did not produce an ability: {source}")
                };
                let actual_kind = match ability.kind() {
                    crate::syntax::AbilityKind::Activated(_) => "activated",
                    crate::syntax::AbilityKind::ClassLevel(_) => "class_level",
                    crate::syntax::AbilityKind::Chapter(_) => "chapter",
                    crate::syntax::AbilityKind::RollRow(_) => "roll_row",
                    crate::syntax::AbilityKind::LevelBand(_) => "level_band",
                    crate::syntax::AbilityKind::StationThreshold(_) => "station_threshold",
                    crate::syntax::AbilityKind::Triggered(_) => "triggered",
                    crate::syntax::AbilityKind::Loyalty(_) => "loyalty",
                    crate::syntax::AbilityKind::Modal(_) => "modal",
                    crate::syntax::AbilityKind::Keyword(_) => "keyword",
                    crate::syntax::AbilityKind::Paragraph(_) => "paragraph",
                };
                assert_eq!(actual_kind, expected_kind, "{source}");
            }
            assert_eq!(reports[0].fragment(), reports[1].fragment());
            assert_eq!(reports[0].fragment(), reports[2].fragment());
            assert_eq!(reports[0].diagnostics(), reports[1].diagnostics());
            assert_eq!(reports[0].diagnostics(), reports[2].diagnostics());
            assert_eq!(
                reports[0].construction_decisions(),
                reports[1].construction_decisions()
            );
            assert_eq!(
                reports[0].construction_decisions(),
                reports[2].construction_decisions()
            );
        }
    }

    #[test]
    fn cost_ability_assembly_is_group_order_neutral() {
        fn assembled(
            groups: Vec<&'static deckmaste_construction_compiler::runtime::GroupData>,
        ) -> FragmentReport {
            let groups = Box::leak(groups.into_boxed_slice());
            parse_fragment_with_activation(
                "{2}{R} or {T}",
                &catalogs(),
                FragmentKind::Cost,
                "",
                false,
                GeneratedActivation::Groups(groups),
            )
        }

        let normal = assembled(crate::constructions::ALL_GROUPS.to_vec());
        let mut reversed_groups = crate::constructions::ALL_GROUPS.to_vec();
        reversed_groups.reverse();
        let reversed = assembled(reversed_groups);
        let mut shuffled_groups = crate::constructions::ALL_GROUPS.to_vec();
        shuffled_groups.rotate_left(5);
        shuffled_groups.swap(2, 9);
        let shuffled = assembled(shuffled_groups);

        assert_eq!(normal.fragment(), reversed.fragment());
        assert_eq!(normal.fragment(), shuffled.fragment());
        assert_eq!(
            normal.construction_decisions(),
            reversed.construction_decisions()
        );
        assert_eq!(
            normal.construction_decisions(),
            shuffled.construction_decisions()
        );
    }

    #[test]
    fn cost_facade_builds_and_renders_every_component_shape() {
        for (source, expected, index) in [
            ("{2}{R}", "symbols", 0),
            ("Sacrifice a creature", "clause", 0),
            ("A creature", "noun", 0),
            ("{T} or {W}", "alternative", 0),
            ("{T}, Frobnicate a creature", "recovered", 1),
        ] {
            let report = production_fragment(source, FragmentKind::Cost);
            let Some(Fragment::Cost(cost)) = report.fragment() else {
                panic!("cost fixture did not produce a root: {source}")
            };
            let component = &cost.components()[index];
            let actual = match component {
                crate::syntax::CostComponent::Symbols(_) => "symbols",
                crate::syntax::CostComponent::Clause(_) => "clause",
                crate::syntax::CostComponent::Noun(_) => "noun",
                crate::syntax::CostComponent::Alternative(..) => "alternative",
                crate::syntax::CostComponent::Recovered(_) => "recovered",
            };
            assert_eq!(actual, expected, "{source}");
            let rebuilt =
                crate::cost::build_cost(cost.flavor_header().cloned(), cost.components().to_vec())
                    .expect("every parsed Cost inhabits the declaration");
            assert_eq!(rebuilt.components(), cost.components());
            assert_eq!(crate::cost::render(&rebuilt, "", false).unwrap(), source);
        }
    }

    #[test]
    fn predicate_activation_reaches_every_vertical_fragment_root() {
        // Mutations caught: hardcode production activation in the ability
        // layer; discard nested construction provenance at a fragment seam;
        // or let a recovered predicate masquerade as a semantic
        // Sentence, Cost, KeywordLine, or Ability result.
        let sentence =
            production_fragment("You may have this creature enter.", FragmentKind::Sentence);
        assert!(sentence.clean(), "{:?}", sentence.diagnostics());
        let causative = assert_generated(&sentence, "verb_phrase_causative");
        assert_eq!(
            causative.evidence().kind(),
            crate::ConstructionEvidenceKind::Role
        );
        assert_eq!(
            causative.evidence().label(),
            "causative host-causee-complement order"
        );
        assert_eq!(causative.evidence_value(), Some("category=VerbPhrase"));
        assert!(matches!(
            sentence.fragment(),
            Some(Fragment::Sentence(sentence))
                if matches!(
                    sentence.body(),
                    crate::syntax::SentenceBody::Independent(clause)
                        if is_deontic_with_inner(clause)
                )
        ));

        let cost = production_fragment("Discard a card", FragmentKind::Cost);
        assert!(cost.clean(), "{:?}", cost.diagnostics());
        let direct = assert_generated(&cost, "verb_phrase_direct_object");
        assert_eq!(
            direct.evidence().kind(),
            crate::ConstructionEvidenceKind::Role
        );
        assert_eq!(direct.evidence().label(), "predicate object role");
        assert_eq!(direct.evidence_value(), Some("category=VerbPhrase"));
        assert!(matches!(
            cost.fragment(),
            Some(Fragment::Cost(cost)) if matches!(
                cost.components(),
                [crate::syntax::CostComponent::Clause(clause)]
                    if is_imperative_transitive(clause)
            )
        ));

        let keyword = production_fragment("Ward—Discard a card.", FragmentKind::KeywordLine);
        assert!(keyword.clean(), "{:?}", keyword.diagnostics());
        assert_generated(&keyword, "verb_phrase_direct_object");
        let Some(Fragment::KeywordLine(line)) = keyword.fragment() else {
            panic!("the keyword root returns a semantic keyword line")
        };
        let item = line.separated_abilities().first();
        assert_eq!(line.separated_abilities().len(), 1);
        let crate::syntax::KeywordArgument::Costed(crate::syntax::KeywordCost::Components {
            cost,
            terminal: true,
        }) = &item.argument
        else {
            panic!("the keyword cost retains its typed tight-dash component shape")
        };
        assert!(matches!(
            cost.components(),
            [crate::syntax::CostComponent::Clause(clause)]
                if is_imperative_transitive(clause)
        ));

        let ability = production_fragment(
            "{T}: You may have this creature enter.",
            FragmentKind::Ability,
        );
        assert!(ability.clean(), "{:?}", ability.diagnostics());
        assert_generated(&ability, "verb_phrase_causative");
        let Some(Fragment::Ability(parsed)) = ability.fragment() else {
            panic!("the ability root returns a semantic activated ability")
        };
        let crate::syntax::AbilityKind::Activated(activated) = parsed.kind() else {
            panic!("the ability root returns a semantic activated ability")
        };
        assert!(matches!(
            activated.effect.sentences.as_slice(),
            [sentence]
                if matches!(
                    sentence.body(),
                    crate::syntax::SentenceBody::Independent(clause)
                        if is_deontic_with_inner(clause)
                )
        ));
    }

    #[test]
    fn causative_semantics_remain_visible_to_recovery_traversal() {
        // Mutation caught: lower the generated causative complement as an
        // opaque parallel value, so the ordinary syntax recovery walker can
        // no longer descend into its quoted embedded rules.
        let report = production_fragment(
            "You may have this creature gain \"Zibble quux.\".",
            FragmentKind::Sentence,
        );
        assert_generated(&report, "verb_phrase_causative");
        assert_eq!(
            report
                .recoveries()
                .iter()
                .map(|recovery| (recovery.role, recovery.text))
                .collect::<Vec<_>>(),
            [(crate::syntax::RecoveryRole::EmbeddedRules, "Zibble quux.")]
        );
    }

    #[test]
    fn predicate_constructions_reach_every_vertical_fragment_and_recovery_consumer() {
        // Mutations caught: activate the predicate family only at the Sentence chart
        // root; discard nested provenance at a Cost/KeywordLine/Ability seam;
        // or retain a recovery-only semantic predicate path.
        for (source, kind, construction) in [
            (
                "You may have this creature enter.",
                FragmentKind::Sentence,
                "verb_phrase_causative",
            ),
            (
                "Discard a card",
                FragmentKind::Cost,
                "verb_phrase_direct_object",
            ),
            (
                "Ward—Discard a card.",
                FragmentKind::KeywordLine,
                "verb_phrase_direct_object",
            ),
            (
                "{T}: You may have this creature enter.",
                FragmentKind::Ability,
                "verb_phrase_causative",
            ),
        ] {
            let report = parse_fragment(source, &catalogs(), kind, "", false);
            assert!(report.clean(), "{source:?}: {:?}", report.diagnostics());
            assert_generated(&report, construction);
        }

        let recovered = parse_fragment(
            "You may have this creature gain \"Zibble quux.\".",
            &catalogs(),
            FragmentKind::Sentence,
            "",
            false,
        );
        assert_generated(&recovered, "verb_phrase_causative");
        assert_eq!(
            recovered
                .recoveries()
                .iter()
                .map(|recovery| (recovery.role, recovery.text))
                .collect::<Vec<_>>(),
            [(crate::syntax::RecoveryRole::EmbeddedRules, "Zibble quux.")]
        );
    }

    #[test]
    fn adjective_constructions_reach_sentence_and_nominal_fragments() {
        for (source, kind, required) in [
            (
                "black creature",
                FragmentKind::Nominal,
                &["adjective", "adjective_phrase"][..],
            ),
            (
                "Turn this creature face up.",
                FragmentKind::Sentence,
                &["adjective_phrase_face_up"][..],
            ),
            (
                "Turn this creature face down.",
                FragmentKind::Sentence,
                &["adjective_phrase_face_down"][..],
            ),
            (
                "Its power is 2 greater.",
                FragmentKind::Sentence,
                &["adjective", "adjective_phrase_degree_measure"][..],
            ),
            (
                "Its power is greater than a card.",
                FragmentKind::Sentence,
                &[
                    "adjective",
                    "adjective_phrase",
                    "comparison_standard",
                    "comparison_than",
                    "adjective_phrase_comparison",
                ][..],
            ),
            (
                "Its power is greater than or equal to a card.",
                FragmentKind::Sentence,
                &[
                    "adjective",
                    "adjective_phrase",
                    "comparison_standard",
                    "comparison_than_or_equal_to",
                    "adjective_phrase_comparison",
                ][..],
            ),
        ] {
            let report = production_fragment(source, kind);
            assert!(report.clean(), "{source:?}: {:?}", report.diagnostics());
            assert_eq!(
                render_fragment(report.fragment().unwrap(), "", false).unwrap(),
                source,
            );
            for construction in required {
                assert_generated(&report, construction);
            }
        }
    }

    #[test]
    fn fragments_parse_cleanly_at_their_kind() {
        let catalogs = catalogs();
        for (text, kind) in [
            ("Draw a card.", FragmentKind::Sentence),
            ("creature you control", FragmentKind::Nominal),
            ("{2}{W}, Sacrifice this artifact", FragmentKind::Cost),
            ("Flying, first strike", FragmentKind::KeywordLine),
            ("{T}: Add {G}.", FragmentKind::Ability),
        ] {
            let report = parse_fragment(text, &catalogs, kind, "", false);
            assert!(
                report.clean(),
                "{text:?} at {kind:?} was not clean: {:?} / {:?}",
                report.diagnostics(),
                report.recoveries(),
            );
            assert_eq!(
                report.fragment().map(Fragment::kind),
                Some(kind),
                "{text:?} produced the wrong fragment kind"
            );
        }
    }

    #[test]
    fn fragment_render_round_trips() {
        let catalogs = catalogs();
        for (text, kind) in [
            ("Draw a card.", FragmentKind::Sentence),
            ("creature you control", FragmentKind::Nominal),
            ("a 1/1 white Soldier creature token", FragmentKind::Nominal),
            ("{2}{W}, Sacrifice this artifact", FragmentKind::Cost),
            ("Flying, first strike", FragmentKind::KeywordLine),
            ("{T}: Add {G}.", FragmentKind::Ability),
            (
                "When this creature enters, draw a card.",
                FragmentKind::Ability,
            ),
        ] {
            let report = parse_fragment(text, &catalogs, kind, "", false);
            let fragment = report
                .fragment()
                .unwrap_or_else(|| panic!("{text:?} produced no {kind:?} fragment"));
            assert_eq!(
                render_fragment(fragment, "", false).expect("fragment renders"),
                text,
                "{kind:?} did not round-trip"
            );
        }
    }

    /// Keyword atoms preserve their source spelling, so a lowercase-initial
    /// keyword line is the case that distinguishes "capitalized at ability
    /// head" from "reproduced verbatim". A card capitalizes it —
    /// `AbilityKind::Keyword` is the one kind whose renderer method takes no
    /// `capitalize` flag, because `Renderer::ability` capitalizes the finished
    /// body instead — and the fragment path must agree byte-for-byte.
    #[test]
    fn a_lowercase_initial_keyword_line_capitalizes_exactly_as_a_card_does() {
        let catalogs = catalogs();
        let text = "first strike, flying";

        let whole_card = crate::parse_with_identity(text, &catalogs, "", false)
            .into_ast()
            .render("", false)
            .expect("the whole-card path renders");
        assert_eq!(
            whole_card, "First strike, flying",
            "control: a card capitalizes its keyword line"
        );

        let report = parse_fragment(text, &catalogs, FragmentKind::KeywordLine, "", false);
        assert!(report.clean(), "{:?}", report.diagnostics());
        let fragment = report.fragment().expect("keyword-line fragment");
        assert_eq!(
            render_fragment(fragment, "", false).expect("fragment renders"),
            whole_card
        );
    }

    /// The one thing `clean` must not be is "the parser returned something".
    /// The ability layer is total, so unparseable text still yields a
    /// fragment — with a recovery inside it and a diagnostic beside it.
    ///
    /// Also pins what `recoveries` is *for*: naming the offending run in a
    /// caller's error message.
    #[test]
    fn recovered_text_is_not_clean_even_though_a_fragment_exists() {
        let report = parse_fragment(
            "Zzzz qqqq wwww.",
            &catalogs(),
            FragmentKind::Ability,
            "",
            false,
        );
        assert!(report.fragment().is_some());
        assert!(!report.clean());
        let recoveries = report.recoveries();
        let [recovery] = recoveries.as_slice() else {
            panic!("expected exactly one recovery, got {recoveries:?}");
        };
        assert_eq!(recovery.role, crate::syntax::RecoveryRole::Clause);
        assert_eq!(recovery.text, "Zzzz qqqq wwww.");
    }

    /// A compiled frame outlives the report and the source it was parsed from,
    /// so the subtree has to be takeable rather than borrowed.
    #[test]
    fn a_fragment_can_be_taken_and_outlive_its_source() {
        let source = String::from("creature you control");
        let fragment = parse_fragment(&source, &catalogs(), FragmentKind::Nominal, "", false)
            .into_fragment()
            .expect("nominal fragment");
        drop(source);

        assert_eq!(fragment.kind(), FragmentKind::Nominal);
        assert_eq!(
            render_fragment(&fragment, "", false).expect("renders"),
            "creature you control"
        );
    }

    /// A chart category that declines has no subtree at all, and mints the
    /// diagnostic that says so.
    #[test]
    fn a_declining_chart_category_reports_no_complete_parse() {
        let report = parse_fragment(
            "{2}{W}, Sacrifice this artifact",
            &catalogs(),
            FragmentKind::Sentence,
            "",
            false,
        );
        assert!(report.fragment().is_none());
        assert!(!report.clean());
        assert!(
            report
                .diagnostics()
                .iter()
                .any(|diagnostic| diagnostic.kind() == DiagnosticKind::NoCompleteParse)
        );
    }

    /// The landmine: a template's parenthesized run must survive
    /// `parse_fragment`, because `strip_reminder_text` would eat it.
    #[test]
    fn parenthesized_text_is_not_stripped_from_a_fragment() {
        let text = "Draw a card (this is not reminder text).";
        assert_eq!(
            crate::strip_reminder_text(text),
            "Draw a card.",
            "control: the corpus normalizer does strip it"
        );
        let report = parse_fragment(text, &catalogs(), FragmentKind::Sentence, "", false);
        // The parenthesized run is not grammar, so the sentence category
        // declines rather than silently parsing the stripped text — which is
        // the point: nothing quietly rewrote the input.
        assert!(report.fragment().is_none());

        // And an ability-layer category, which recovers rather than declining,
        // reproduces the run verbatim.
        let report = parse_fragment(text, &catalogs(), FragmentKind::Ability, "", false);
        let fragment = report.fragment().expect("the ability layer is total");
        assert_eq!(render_fragment(fragment, "", false).expect("renders"), text);
    }

    /// Identity threads through both directions, so a self-referring fragment
    /// round-trips as the face that owns it.
    #[test]
    fn self_reference_threads_through_both_directions() {
        let text = "Nissa deals 4 damage to target creature.";
        let report = parse_fragment(
            text,
            &catalogs(),
            FragmentKind::Sentence,
            "Nissa Revane",
            true,
        );
        assert!(report.clean(), "{:?}", report.diagnostics());
        let fragment = report.fragment().expect("sentence fragment");
        assert_eq!(
            render_fragment(fragment, "Nissa Revane", true).expect("renders"),
            text
        );
    }
}
