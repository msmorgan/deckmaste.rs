//! Historical decklists as data and an exact-list builder.

use std::sync::Arc;

use deckmaste_card::Card;

use crate::source::CardSource;

/// A 60-card list: one basic-land entry plus the nonbasic "rest" (spells AND
/// utility lands — anything that may need graduation).
pub struct DeckSpec {
    pub name: &'static str,
    pub basics: (&'static str, u8),
    pub rest: &'static [(&'static str, u8)],
}

impl DeckSpec {
    #[must_use]
    pub fn size(&self) -> usize {
        usize::from(self.basics.1)
            + self
                .rest
                .iter()
                .map(|&(_, c)| usize::from(c))
                .sum::<usize>()
    }
}

/// Builds the listed 60 exactly.
#[must_use]
pub fn build_full(spec: &DeckSpec, src: &CardSource) -> Vec<Arc<Card>> {
    let mut deck = Vec::with_capacity(spec.size());
    push_copies(&mut deck, src, spec.basics.0, usize::from(spec.basics.1));
    for &(name, count) in spec.rest {
        push_copies(&mut deck, src, name, usize::from(count));
    }
    deck
}

fn push_copies(deck: &mut Vec<Arc<Card>>, src: &CardSource, name: &str, count: usize) {
    let card = src.card(name);
    for _ in 0..count {
        deck.push(Arc::clone(&card));
    }
}

#[cfg(test)]
mod tests {
    use deckmaste_card::Card;

    use super::*;
    use crate::wc99;

    fn names(deck: &[Arc<Card>]) -> Vec<String> {
        deck.iter()
            .map(|c| c.primary_face().characteristics.name.to_string())
            .collect()
    }

    #[test]
    fn specs_are_sixty_cards() {
        assert_eq!(wc99::SPED_RED.size(), 60);
        assert_eq!(wc99::STOMPY.size(), 60);
        assert_eq!(wc99::RDW2K.size(), 60);
    }

    #[test]
    #[cfg_attr(not(wizards_corpus), ignore = "needs generated plugins/wizards corpus")]
    fn historical_matchup_full_lists_resolve() {
        let src = CardSource::load();
        for spec in [&wc99::SPED_RED, &wc99::STOMPY] {
            let deck = build_full(spec, &src);
            assert_eq!(deck.len(), 60);
            let names = names(&deck);
            assert_eq!(
                names.iter().filter(|n| n.as_str() == spec.basics.0).count(),
                usize::from(spec.basics.1)
            );
            for &(name, count) in spec.rest {
                assert_eq!(
                    names
                        .iter()
                        .filter(|actual| actual.as_str() == name)
                        .count(),
                    usize::from(count),
                    "{}: {name} count drifted",
                    spec.name
                );
            }
        }
    }
}
