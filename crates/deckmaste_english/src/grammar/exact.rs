//! The set-valued exact-parse law harness: budgeted enumeration of packed
//! alternatives into deduplicated `ExactParse` sets, with `linearize` as the
//! generated render entry. Strict exact mode throughout — `OpacityMode::Exact`,
//! no opaque-noun retry — so a law can never pass through an escape hatch.
//! Chart-level admission only: form guards are not consulted (Milestone-3
//! status quo), and dominance-losing derivations stay in the set (dominance
//! is selection preference, not admission).

use super::Catalogs;
use super::EnglishGrammar;
use super::Nonterminal;
use super::OpacityMode;
use super::lowering::lower;
use super::parse_chart;
use crate::forest::ChoiceMap;
use crate::identity::SelfReference;
use crate::surface::collapse_full_names;
use crate::surface::lex;

#[cfg(test)]
mod tests {
    use super::*;
    use crate::catalog::CatalogKind;

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
}
