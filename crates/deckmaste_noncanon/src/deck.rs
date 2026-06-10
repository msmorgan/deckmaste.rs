//! Decklists as data, and the two deck builders: the faithful full list and
//! the converging subset (basics kept, unimplemented spell slots filled by
//! cycling the allowlisted cards).

use std::sync::Arc;

use deckmaste_core::Card;

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

/// The faithful list. Panics (via `CardSource::card`) while any card is
/// ungraduated — which is exactly what the red full-matchup test wants.
#[must_use]
pub fn build_full(spec: &DeckSpec, src: &CardSource) -> Vec<Arc<Card>> {
    let mut deck = Vec::with_capacity(spec.size());
    push_copies(&mut deck, src, spec.basics.0, usize::from(spec.basics.1));
    for &(name, count) in spec.rest {
        push_copies(&mut deck, src, name, usize::from(count));
    }
    deck
}

/// The converging subset: basics unchanged; every non-allowlisted "rest" slot
/// is filled by cycling through the allowlisted entries (in spec order,
/// playset by playset). Total stays exactly `spec.size()`, the land ratio is
/// exact, and the deck converges to `build_full` as the allowlist grows.
/// Early waves are degenerate (e.g. 44 Shocks) but fully mechanical.
///
/// # Panics
///
/// Panics if no spell in the allowlist resolves (an empty subset can't play).
#[must_use]
pub fn build_subset(spec: &DeckSpec, allow: &[&str], src: &CardSource) -> Vec<Arc<Card>> {
    let allowed: Vec<(&str, u8)> = spec
        .rest
        .iter()
        .copied()
        .filter(|(name, _)| allow.contains(name))
        .collect();
    assert!(
        !allowed.is_empty(),
        "{}: no spec entry matches allow={allow:?}",
        spec.name
    );

    let mut deck = Vec::with_capacity(spec.size());
    push_copies(&mut deck, src, spec.basics.0, usize::from(spec.basics.1));
    let rest_slots = spec.size() - deck.len();
    let mut cycle = allowed
        .iter()
        .flat_map(|&(name, count)| std::iter::repeat_n(name, usize::from(count)))
        .cycle();
    for _ in 0..rest_slots {
        let name = cycle.next().expect("non-empty cycle");
        deck.push(src.card(name));
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
    use deckmaste_core::Card;

    use super::*;
    use crate::wc99;

    fn names(deck: &[Arc<Card>]) -> Vec<String> {
        deck.iter()
            .map(|c| match c.as_ref() {
                Card::Normal(f) | Card::ModalDfc(f, _) => f.name.clone(),
            })
            .collect()
    }

    #[test]
    fn specs_are_sixty_cards() {
        assert_eq!(wc99::SPED_RED.size(), 60);
        assert_eq!(wc99::STOMPY.size(), 60);
    }

    #[test]
    fn subset_keeps_basics_and_cycles_allowlist() {
        let src = CardSource::load();
        let deck = build_subset(&wc99::SPED_RED, wc99::SPED_RED_ALLOWLIST, &src);
        assert_eq!(deck.len(), 60);
        let names = names(&deck);
        assert_eq!(names.iter().filter(|n| *n == "Mountain").count(), 16);
        for n in names.iter().filter(|n| *n != "Mountain") {
            assert!(
                wc99::SPED_RED_ALLOWLIST.contains(&n.as_str()),
                "{n} not allowlisted"
            );
        }
    }

    #[test]
    fn stompy_subset_resolves_too() {
        let src = CardSource::load();
        let deck = build_subset(&wc99::STOMPY, wc99::STOMPY_ALLOWLIST, &src);
        assert_eq!(deck.len(), 60);
        assert_eq!(names(&deck).iter().filter(|n| *n == "Forest").count(), 14);
    }
}
