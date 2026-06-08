//! Casting (CR 601) — payment this task; the announce flow in Task 8.

use deckmaste_core::{ColorOrColorless, ManaCost, ManaSymbol, SimpleManaSymbol, Uint};

use crate::player::ManaPool;

/// How the pool's mana is spent on a cost's generic part: one entry per unit
/// of generic mana owed (colored pips are forced by color, so they need no
/// choice). Empty when the cost is all-colored.
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct Payment {
    pub generic: Vec<ColorOrColorless>,
}

/// The colored requirement (per color) and total generic of a cost, or `None`
/// if the cost uses an out-of-scope symbol (X, hybrid, phyrexian, snow).
fn requirement(cost: &ManaCost) -> Option<(Vec<(ColorOrColorless, Uint)>, Uint)> {
    let mut colored: Vec<(ColorOrColorless, Uint)> = Vec::new();
    let mut generic: Uint = 0;
    for symbol in cost.iter() {
        match symbol {
            ManaSymbol::Simple(SimpleManaSymbol::Generic(n)) => generic += n,
            ManaSymbol::Simple(SimpleManaSymbol::Specific(c)) => {
                match colored.iter_mut().find(|(k, _)| k == c) {
                    Some((_, v)) => *v += 1,
                    None => colored.push((*c, 1)),
                }
            }
            // X/Hybrid/Phyrexian/Snow are out of scope this stage.
            _ => return None,
        }
    }
    Some((colored, generic))
}

/// Whether `pool` can pay `cost` (CR 601.2g). Colored pips must be covered by
/// their color; the remaining mana must cover the generic.
///
/// Returns `false` for costs containing out-of-scope symbols (X, hybrid,
/// phyrexian, snow).
#[must_use]
pub fn can_pay(pool: &ManaPool, cost: &ManaCost) -> bool {
    let Some((colored, generic)) = requirement(cost) else { return false };
    let mut leftover: Uint = 0;
    // Sum leftover across all six kinds after colored deductions.
    for (kind, have) in pool_kinds(pool) {
        let need = colored
            .iter()
            .find(|(k, _)| *k == kind)
            .map_or(0, |(_, n)| *n);
        if have < need {
            return false;
        }
        leftover += have - need;
    }
    leftover >= generic
}

/// The unique allocation if there is exactly one; `None` if the player has a
/// real choice (so a `PayMana` decision must surface) or it can't be paid.
///
/// Returns `None` for costs containing out-of-scope symbols.
#[must_use]
pub fn forced_payment(pool: &ManaPool, cost: &ManaCost) -> Option<Payment> {
    let (colored, generic) = requirement(cost)?;
    if !can_pay(pool, cost) {
        return None;
    }
    // Pool remaining after the (forced) colored pips, as a flat multiset.
    let mut leftover: Vec<ColorOrColorless> = Vec::new();
    for (kind, have) in pool_kinds(pool) {
        let need = colored
            .iter()
            .find(|(k, _)| *k == kind)
            .map_or(0, |(_, n)| *n);
        for _ in 0..(have - need) {
            leftover.push(kind);
        }
    }
    if generic == 0 {
        return Some(Payment { generic: vec![] });
    }
    // Forced iff the generic units can be drawn only one way: the leftover is
    // a single color, or you must spend all of it.
    let distinct: std::collections::HashSet<_> = leftover.iter().copied().collect();
    let forced = distinct.len() == 1 || leftover.len() == generic as usize;
    forced.then(|| Payment {
        generic: leftover.into_iter().take(generic as usize).collect(),
    })
}

/// Whether `payment` legally covers `cost` from `pool`.
#[must_use]
pub fn validate_payment(pool: &ManaPool, cost: &ManaCost, payment: &Payment) -> bool {
    let Some((colored, generic)) = requirement(cost) else { return false };
    if payment.generic.len() != generic as usize {
        return false;
    }
    // Total spend per kind = colored need + generic chosen of that kind.
    for (kind, have) in pool_kinds(pool) {
        let colored_need = colored
            .iter()
            .find(|(k, _)| *k == kind)
            .map_or(0, |(_, n)| *n);
        let generic_need =
            u32::try_from(payment.generic.iter().filter(|&&c| c == kind).count()).unwrap();
        if have < colored_need + generic_need {
            return false;
        }
    }
    true
}

/// Deducts a validated `payment` from `pool` (CR 601.2g, 106.4).
///
/// # Panics
///
/// Panics if the cost contains out-of-scope symbols, or if the pool does not
/// cover the payment (callers must validate first).
pub fn apply_payment(pool: &mut ManaPool, cost: &ManaCost, payment: &Payment) {
    let (colored, _) = requirement(cost).expect("validated cost");
    for (kind, n) in colored {
        pool.spend(kind, n);
    }
    for kind in &payment.generic {
        pool.spend(*kind, 1);
    }
}

/// The pool as (kind, amount) pairs over all six kinds.
fn pool_kinds(pool: &ManaPool) -> [(ColorOrColorless, Uint); 6] {
    use ColorOrColorless::{Color, Colorless};
    use deckmaste_core::Color::{Black, Blue, Green, Red, White};
    [
        Colorless,
        Color(White),
        Color(Blue),
        Color(Black),
        Color(Red),
        Color(Green),
    ]
    .map(|k| (k, pool.amount(k)))
}

#[cfg(test)]
mod tests {
    use deckmaste_core::Color;

    use super::*;

    fn pool(pairs: &[(ColorOrColorless, Uint)]) -> ManaPool {
        let mut p = ManaPool::default();
        for &(m, n) in pairs {
            p.add(m, n);
        }
        p
    }
    fn cost(s: &str) -> ManaCost { s.parse().unwrap() }
    fn red() -> ColorOrColorless { Color::Red.into() }
    fn green() -> ColorOrColorless { Color::Green.into() }

    #[test]
    fn colored_pip_needs_its_color() {
        assert!(can_pay(&pool(&[(red(), 1)]), &cost("{R}")));
        assert!(!can_pay(&pool(&[(green(), 1)]), &cost("{R}")));
        assert!(!can_pay(&ManaPool::default(), &cost("{R}")));
    }

    #[test]
    fn generic_pays_from_any_leftover() {
        assert!(can_pay(&pool(&[(green(), 2)]), &cost("{1}{G}"))); // G pays {G}, G pays {1}
        assert!(!can_pay(&pool(&[(green(), 1)]), &cost("{1}{G}"))); // nothing left for {1}
        assert!(can_pay(&pool(&[(green(), 1), (red(), 1)]), &cost("{1}{G}")));
    }

    #[test]
    fn forced_payment_is_unique_or_none() {
        // {1}{G} from G,G: {G}<-one green, {1}<-the other — the only allocation.
        assert_eq!(
            forced_payment(&pool(&[(green(), 2)]), &cost("{1}{G}")),
            Some(Payment {
                generic: vec![green()]
            }),
        );
        // {1}{G} from G,R: {G} must take the lone green, so {1} can only be R —
        // exactly one allocation, so it IS forced (no decision needed).
        assert_eq!(
            forced_payment(&pool(&[(green(), 1), (red(), 1)]), &cost("{1}{G}")),
            Some(Payment {
                generic: vec![red()]
            }),
        );
        // {1}{G} from G,G,R: {G} takes one green; {1} can be the other green OR
        // the red — a real choice, so NOT forced.
        assert_eq!(
            forced_payment(&pool(&[(green(), 2), (red(), 1)]), &cost("{1}{G}")),
            None,
        );
        // {R} has no generic at all — trivially forced (empty allocation).
        assert_eq!(
            forced_payment(&pool(&[(red(), 1)]), &cost("{R}")),
            Some(Payment { generic: vec![] }),
        );
    }

    #[test]
    fn validate_and_apply_round_trip() {
        let mut p = pool(&[(green(), 1), (red(), 1)]);
        let pay = Payment {
            generic: vec![red()],
        }; // {1}<-R, {G}<-G
        assert!(validate_payment(&p, &cost("{1}{G}"), &pay));
        apply_payment(&mut p, &cost("{1}{G}"), &pay);
        assert!(p.is_empty());
        // An allocation that double-spends a color it doesn't have is invalid.
        assert!(!validate_payment(
            &pool(&[(green(), 2)]),
            &cost("{1}{G}"),
            &Payment {
                generic: vec![red()]
            }
        ));
    }
}
