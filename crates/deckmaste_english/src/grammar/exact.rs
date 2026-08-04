//! The set-valued exact-parse law harness: budgeted enumeration of packed
//! alternatives into deduplicated `ExactParse` sets, with `linearize` as the
//! generated render entry. Strict exact mode throughout — `OpacityMode::Exact`,
//! no opaque-noun retry — so a law can never pass through an escape hatch.
//! Chart-level admission only: form guards are not consulted (Milestone-3
//! status quo), and dominance-losing derivations stay in the set (dominance
//! is selection preference, not admission).

use deckmaste_construction_compiler::runtime::AtomData;

use super::Catalogs;
use super::EnglishGrammar;
use super::EnglishSurfaceWitness;
use super::GrammarError;
use super::MeaningKey;
use super::Nonterminal;
use super::OpacityMode;
use super::generated::GeneratedActivation;
use super::lowering::lower;
use super::lowering::selected_rule_children;
use super::parse_chart;
use super::parse_support::EnglishForest;
use super::rules::RegistrationOrder;
use super::rules::RuleImpl;
use crate::features::ExactParse;
use crate::forest::AlternativeSelection;
use crate::forest::ChoiceMap;
use crate::forest::NodeId;
use crate::forest::SelectionEnumerationError;
use crate::identity::SelfReference;
use crate::surface::collapse_full_names;
use crate::surface::lex;

/// One enumerated generated derivation: the construction, the DECLARED form
/// ordinal it parsed through (the form-choice witness — comma presence and
/// its like are facts of which form matched), and parts mirroring that
/// form's surface atoms in declaration order.
#[derive(Debug, Clone, PartialEq, Eq)]
pub(crate) struct GeneratedParse {
    pub(crate) construction: &'static str,
    pub(crate) ordinal: u16,
    pub(crate) parts: Vec<GeneratedPart>,
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub(crate) enum GeneratedPart {
    Literal(&'static str),
    Scalar {
        field: &'static str,
        value: GeneratedScalar,
    },
    Subtree {
        field: &'static str,
        parse: GeneratedParse,
    },
}

/// Captured scalar-lexeme values, mirroring `generated::codec_slot`'s closed
/// codec table exactly — a codec admitted there must be renderable here.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub(crate) enum GeneratedScalar {
    Conjunction(crate::features::Conjunction),
    Comma,
}

fn generated_parse<S: AlternativeSelection>(
    grammar: &EnglishGrammar<'_, '_>,
    forest: &EnglishForest,
    node: NodeId,
    selection: &S,
) -> Option<GeneratedParse> {
    let forest_node = forest.node(node);
    let alternative = forest_node.alternatives.get(selection.alternative(node)?)?;
    let rule = alternative.rule?;
    let RuleImpl::Generated(generated) = grammar.impls.get(rule.index()).copied()? else {
        return None;
    };
    let construction = generated.group.constructions.get(generated.construction)?;
    let form = construction.forms.get(generated.form)?;
    let [intermediate] = alternative.children.as_slice() else {
        return None;
    };
    let mut children = Vec::new();
    selected_rule_children(forest, *intermediate, selection, &mut children)?;
    if children.len() != form.atoms.len() {
        return None;
    }
    let mut parts = Vec::with_capacity(form.atoms.len());
    for (&atom, &child) in form.atoms.iter().zip(&children) {
        parts.push(match atom {
            AtomData::Literal(literal) => GeneratedPart::Literal(literal),
            AtomData::Lexeme(field) => GeneratedPart::Scalar {
                field,
                value: scalar_value(forest, child)?,
            },
            AtomData::Hole(field) => GeneratedPart::Subtree {
                field,
                parse: generated_parse(grammar, forest, child, selection)?,
            },
        });
    }
    Some(GeneratedParse {
        construction: construction.id,
        ordinal: form.ordinal,
        parts,
    })
}

fn scalar_value(forest: &EnglishForest, node: NodeId) -> Option<GeneratedScalar> {
    match forest.node(node).key.lexical_value()? {
        MeaningKey::Conjunction(conjunction) => Some(GeneratedScalar::Conjunction(*conjunction)),
        // `MeaningKey::Punctuation` is a unit variant — it lost which glyph
        // matched. Reading it as a comma relies on `generated.rs`'s closed
        // tables (`codec_slot`'s codec map, `atom_expected`'s
        // `UnsupportedLiteral` rejection of every literal but `","`) to keep
        // any other punctuation out of a generated atom position.
        MeaningKey::Punctuation => Some(GeneratedScalar::Comma),
        _ => None,
    }
}

/// The generated render entry: parts back to bytes. Tokens join with a
/// single space, except nothing precedes a comma — the fixture surface
/// convention (`"and, or"`), and the rule the byte-equality laws hold
/// linearization to.
pub(crate) fn linearize(parse: &GeneratedParse) -> String {
    let mut tokens = Vec::new();
    collect_tokens(parse, &mut tokens);
    let mut rendered = String::new();
    for token in tokens {
        if !(rendered.is_empty() || token == ",") {
            rendered.push(' ');
        }
        rendered.push_str(token);
    }
    rendered
}

fn collect_tokens(parse: &GeneratedParse, tokens: &mut Vec<&'static str>) {
    for part in &parse.parts {
        match part {
            GeneratedPart::Literal(literal) => tokens.push(literal),
            GeneratedPart::Scalar {
                value: GeneratedScalar::Conjunction(conjunction),
                ..
            } => tokens.push(conjunction.spelling()),
            GeneratedPart::Scalar {
                value: GeneratedScalar::Comma,
                ..
            } => tokens.push(","),
            GeneratedPart::Subtree { parse, .. } => collect_tokens(parse, tokens),
        }
    }
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub(crate) enum ExactParseError {
    Grammar(GrammarError),
    /// The forest reaches a cycle; enumeration refuses rather than risking
    /// unbounded derivations (see `SelectionEnumerationError::Cycle`).
    Cycle,
    /// More complete selections exist than the caller's budget admits. A
    /// hard, loud ceiling — never silent truncation.
    TooManyAlternatives {
        budget: usize,
    },
}

/// Strict-exact, set-valued parse: every per-node-consistent derivation of
/// `source` as `nonterminal`, extracted as [`GeneratedParse`] and paired
/// with the root alternative's surface payload, deduplicated. Empty set =
/// nothing admitted. Dominance-losing derivations are included (dominance
/// is selection preference, not admission), and no opaque-noun retry ever
/// runs.
pub(crate) fn parse_as(
    source: &str,
    catalogs: &Catalogs,
    nonterminal: Nonterminal,
    activation: GeneratedActivation,
    budget: usize,
) -> Result<Vec<ExactParse<GeneratedParse, EnglishSurfaceWitness>>, ExactParseError> {
    parse_as_with_registration_order(
        source,
        catalogs,
        nonterminal,
        activation,
        budget,
        RegistrationOrder::Normal,
    )
}

fn parse_as_with_registration_order(
    source: &str,
    catalogs: &Catalogs,
    nonterminal: Nonterminal,
    activation: GeneratedActivation,
    budget: usize,
    order: RegistrationOrder,
) -> Result<Vec<ExactParse<GeneratedParse, EnglishSurfaceWitness>>, ExactParseError> {
    let self_reference = SelfReference::default();
    let surface = lex(source);
    let tokens = collapse_full_names(source, surface.tokens, self_reference.full_name());
    let grammar = EnglishGrammar::with_opacity_mode_and_registration_order(
        source,
        catalogs,
        nonterminal,
        OpacityMode::Exact,
        self_reference,
        order,
        activation,
    );
    let chart = parse_chart(&grammar, &tokens).map_err(ExactParseError::Grammar)?;
    let mut remaining = budget;
    let mut results = Vec::new();
    for &root in &chart.roots {
        let selections = chart
            .forest
            .enumerate_selections(root, &mut remaining)
            .map_err(|error| match error {
                SelectionEnumerationError::Cycle(_) => ExactParseError::Cycle,
                SelectionEnumerationError::BudgetExhausted => {
                    ExactParseError::TooManyAlternatives { budget }
                }
            })?;
        for selection in selections {
            let Some(ast) = generated_parse(&grammar, &chart.forest, root, &selection) else {
                continue;
            };
            let Some(alternative) = selection.alternative(root) else {
                continue;
            };
            let Some(exact) = chart.forest.exact_result(root, alternative, ast) else {
                continue;
            };
            if !results.contains(&exact) {
                results.push(exact);
            }
        }
    }
    Ok(results)
}

#[cfg(test)]
mod tests {
    use super::super::generated::GeneratedActivation;
    use super::*;
    use crate::catalog::CatalogKind;
    use crate::constructions::law;
    use crate::constructions::probe;

    fn probe_category(name: &str) -> Nonterminal {
        let cats = super::super::generated::internal_categories(probe::GROUPS);
        Nonterminal::Generated(cats[name])
    }

    fn law_category(name: &str) -> Nonterminal {
        let cats = super::super::generated::internal_categories(law::GROUPS);
        Nonterminal::Generated(cats[name])
    }

    /// Order-insensitive set equality — permutations may reorder discovery.
    fn same_exact_set(
        left: &[ExactParse<GeneratedParse, EnglishSurfaceWitness>],
        right: &[ExactParse<GeneratedParse, EnglishSurfaceWitness>],
    ) -> bool {
        left.len() == right.len() && left.iter().all(|member| right.contains(member))
    }

    fn fixture_catalogs() -> Catalogs {
        Catalogs::default()
            .with_catalog(CatalogKind::KeywordAbility, ["Protection"])
            .with_catalog(
                CatalogKind::CardType,
                [
                    "Artifact",
                    "Battle",
                    "Creature",
                    "Land",
                    "Planeswalker",
                    "Sorcery",
                ],
            )
    }

    #[test]
    fn choice_map_lowering_matches_best_parse_lowering() {
        let catalogs = fixture_catalogs();
        for &(nonterminal, source) in &[
            (Nonterminal::Clause, "copy that spell"),
            (Nonterminal::NounPhrase, "the declare attackers step"),
            (
                Nonterminal::Sentence,
                "Exile target artifact, creature, or planeswalker and target land or battle.",
            ),
        ] {
            let self_reference = SelfReference::default();
            let surface = lex(source);
            let tokens = collapse_full_names(source, surface.tokens, self_reference.full_name());
            let grammar = EnglishGrammar::with_opacity_mode(
                source,
                &catalogs,
                nonterminal,
                OpacityMode::Exact,
                self_reference,
            );
            let chart = parse_chart(&grammar, &tokens)
                .unwrap_or_else(|error| panic!("chart failed for {source:?}: {error:?}"));
            let mut best_syntax = None;
            let (root, best) = chart
                .forest
                .best_root_matching(
                    chart.roots.iter().copied(),
                    super::super::construction::registry(),
                    |root, best| {
                        best_syntax = lower(&grammar, &chart.forest, root, best);
                        best_syntax.is_some()
                    },
                )
                .expect("acyclic forest")
                .unwrap_or_else(|| panic!("no lowerable root for {source:?}"));
            let map = ChoiceMap::from_best(&best);
            let via_map = lower(&grammar, &chart.forest, root, &map)
                .unwrap_or_else(|| panic!("map lowering declined for {source:?}"));
            assert_eq!(
                format!("{via_map:?}"),
                format!("{:?}", best_syntax.expect("matching root lowered")),
                "ChoiceMap lowering diverged from BestParse lowering for {source:?}",
            );
        }
    }

    #[test]
    fn linearize_joins_tokens_with_no_space_before_comma() {
        use crate::features::Conjunction;
        let padded = GeneratedParse {
            construction: "probe_word",
            ordinal: 7,
            parts: vec![
                GeneratedPart::Literal(","),
                GeneratedPart::Scalar {
                    field: "word",
                    value: GeneratedScalar::Conjunction(Conjunction::And),
                },
            ],
        };
        assert_eq!(linearize(&padded), ", and");
        let pair = GeneratedParse {
            construction: "probe_pair",
            ordinal: 0,
            parts: vec![
                GeneratedPart::Subtree {
                    field: "first",
                    parse: GeneratedParse {
                        construction: "probe_word",
                        ordinal: 0,
                        parts: vec![GeneratedPart::Scalar {
                            field: "word",
                            value: GeneratedScalar::Conjunction(Conjunction::And),
                        }],
                    },
                },
                GeneratedPart::Scalar {
                    field: "tail",
                    value: GeneratedScalar::Comma,
                },
                GeneratedPart::Subtree {
                    field: "second",
                    parse: GeneratedParse {
                        construction: "probe_word",
                        ordinal: 0,
                        parts: vec![GeneratedPart::Scalar {
                            field: "word",
                            value: GeneratedScalar::Conjunction(Conjunction::Or),
                        }],
                    },
                },
            ],
        };
        assert_eq!(linearize(&pair), "and, or");
    }

    #[test]
    fn parse_as_carries_the_declared_form_ordinal_as_the_form_witness() {
        let bare = parse_as(
            "or",
            &Catalogs::default(),
            probe_category("ProbeItem"),
            GeneratedActivation::Groups(probe::GROUPS),
            100,
        )
        .expect("the bare probe word parses");
        assert_eq!(bare.len(), 1);
        assert_eq!(bare[0].ast().construction, "probe_word");
        assert_eq!(bare[0].ast().ordinal, 0);
        assert_eq!(linearize(bare[0].ast()), "or");

        let padded = parse_as(
            ", and",
            &Catalogs::default(),
            probe_category("ProbeItem"),
            GeneratedActivation::Groups(probe::GROUPS),
            100,
        )
        .expect("the padded probe word parses");
        assert_eq!(padded.len(), 1);
        assert_eq!(padded[0].ast().construction, "probe_word");
        assert_eq!(
            padded[0].ast().ordinal,
            7,
            "the declared `@ 7` ordinal is the form witness",
        );
        assert_eq!(linearize(padded[0].ast()), ", and");
    }

    #[test]
    fn parse_as_returns_the_empty_set_when_nothing_is_admitted() {
        // Contrast case first: the comma-bearing pair parses under these
        // same groups, so an empty set below is admission refusal, not a
        // wiring failure.
        let admitted = parse_as(
            "and, or",
            &Catalogs::default(),
            probe_category("ProbePairRoot"),
            GeneratedActivation::Groups(probe::GROUPS),
            100,
        )
        .expect("the comma-bearing pair parses");
        assert!(!admitted.is_empty());
        let refused = parse_as(
            "and or",
            &Catalogs::default(),
            probe_category("ProbePairRoot"),
            GeneratedActivation::Groups(probe::GROUPS),
            100,
        )
        .expect("no complete parse is the EMPTY SET, not an error");
        assert_eq!(refused, Vec::new());
    }

    #[test]
    fn exceeding_the_budget_is_too_many_alternatives() {
        // "and" as ProbeRoot packs probe_pick and probe_pick_shadow: two
        // selections against a budget of one.
        let error = parse_as(
            "and",
            &Catalogs::default(),
            probe_category("ProbeRoot"),
            GeneratedActivation::Groups(probe::GROUPS),
            1,
        )
        .expect_err("two derivations cannot fit a budget of one");
        assert!(
            matches!(error, ExactParseError::TooManyAlternatives { budget: 1 }),
            "unexpected error: {error:?}",
        );
    }

    #[test]
    fn one_byte_string_two_asts() {
        let set = parse_as(
            "or",
            &Catalogs::default(),
            law_category("LawRoot"),
            GeneratedActivation::Groups(law::GROUPS),
            100,
        )
        .expect("the ambiguous root parses");
        assert_eq!(set.len(), 2, "law_first and law_second both admit: {set:?}");
        let mut ids = set
            .iter()
            .map(|member| member.ast().construction)
            .collect::<Vec<_>>();
        ids.sort_unstable();
        assert_eq!(ids, ["law_first", "law_second"]);
        for member in &set {
            assert_eq!(linearize(member.ast()), "or");
        }
    }

    #[test]
    fn one_ast_two_surfaces_distinguished_by_form_witness() {
        // probe_word admits the same scalar value through two forms: bare
        // (`@ 0`) and comma-padded (`@ 7`). Same construction, same field
        // values, different form witness, different bytes — and linearize
        // maps each exact parse back to ITS OWN surface.
        let activation = GeneratedActivation::Groups(probe::GROUPS);
        let bare = parse_as(
            "or",
            &Catalogs::default(),
            probe_category("ProbeItem"),
            activation,
            100,
        )
        .expect("bare parses");
        let padded = parse_as(
            ", or",
            &Catalogs::default(),
            probe_category("ProbeItem"),
            activation,
            100,
        )
        .expect("padded parses");
        assert_eq!(bare.len(), 1, "one exact parse for the bare surface");
        assert_eq!(padded.len(), 1, "one exact parse for the padded surface");
        let (bare, padded) = (&bare[0], &padded[0]);
        assert_eq!(bare.ast().construction, padded.ast().construction);
        let scalar_fields = |parse: &GeneratedParse| {
            parse
                .parts
                .iter()
                .filter_map(|part| match part {
                    GeneratedPart::Scalar { field, value } => Some((*field, *value)),
                    _ => None,
                })
                .collect::<Vec<_>>()
        };
        assert_eq!(
            scalar_fields(bare.ast()),
            scalar_fields(padded.ast()),
            "the two surfaces carry the same field values",
        );
        assert_ne!(
            bare.ast().ordinal,
            padded.ast().ordinal,
            "the witness differs"
        );
        assert_eq!(linearize(bare.ast()), "or");
        assert_eq!(linearize(padded.ast()), ", or");
    }

    #[test]
    fn dominance_losing_derivations_stay_in_the_set() {
        // probe_pick dominates probe_pick_shadow; the SELECTED parse drops
        // the shadow, but the exact-parse SET keeps it — dominance is
        // selection preference, not admission.
        let set = parse_as(
            "and",
            &Catalogs::default(),
            probe_category("ProbeRoot"),
            GeneratedActivation::Groups(probe::GROUPS),
            100,
        )
        .expect("the probe root parses");
        assert_eq!(set.len(), 2);
        let mut ids = set
            .iter()
            .map(|member| member.ast().construction)
            .collect::<Vec<_>>();
        ids.sort_unstable();
        assert_eq!(ids, ["probe_pick", "probe_pick_shadow"]);
        for member in &set {
            assert_eq!(linearize(member.ast()), "and");
        }
    }

    #[test]
    fn every_exact_parse_linearizes_to_its_source_and_reparses_to_itself() {
        // The two round-trip laws over the nested pair: probe_pair's product
        // space ("and, or" = pick/shadow choices in each hole = 4 exact
        // parses). Law 1: linearize(ep) == source, for every member. Law 2:
        // parse_as(linearize(ep)) contains ep.
        let activation = GeneratedActivation::Groups(probe::GROUPS);
        let source = "and, or";
        let set = parse_as(
            source,
            &Catalogs::default(),
            probe_category("ProbePairRoot"),
            activation,
            100,
        )
        .expect("the pair parses");
        assert_eq!(set.len(), 4, "2 root choices x 2 per hole: {set:?}");
        for member in &set {
            assert_eq!(linearize(member.ast()), source, "law 1 for {member:?}");
        }
        for member in &set {
            let reparsed = parse_as(
                &linearize(member.ast()),
                &Catalogs::default(),
                probe_category("ProbePairRoot"),
                activation,
                100,
            )
            .expect("linearized bytes reparse");
            assert!(reparsed.contains(member), "law 2 for {member:?}");
        }
    }

    #[test]
    fn law_outputs_survive_registration_and_group_permutations() {
        let normal = parse_as(
            "and, or",
            &Catalogs::default(),
            probe_category("ProbePairRoot"),
            GeneratedActivation::Groups(probe::GROUPS),
            100,
        )
        .expect("normal order parses");
        for order in [RegistrationOrder::Reversed, RegistrationOrder::FixedShuffle] {
            let permuted = parse_as_with_registration_order(
                "and, or",
                &Catalogs::default(),
                probe_category("ProbePairRoot"),
                GeneratedActivation::Groups(probe::GROUPS),
                100,
                order,
            )
            .expect("permuted order parses");
            assert!(
                same_exact_set(&normal, &permuted),
                "law outputs changed under {order:?}: {normal:?} vs {permuted:?}",
            );
        }
        let reversed_groups = parse_as(
            "and, or",
            &Catalogs::default(),
            probe_category("ProbePairRoot"),
            GeneratedActivation::Groups(probe::GROUPS_REVERSED),
            100,
        )
        .expect("reversed group list parses");
        assert!(
            same_exact_set(&normal, &reversed_groups),
            "law outputs changed under group-list reversal",
        );
    }
}
