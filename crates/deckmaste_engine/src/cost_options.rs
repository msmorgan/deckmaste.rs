//! Announce-time concretization of multi-way mana symbols ([CR#601.2b]).
//!
//! Before the total cost locks ([CR#601.2f]), the player announces how to read
//! each hybrid or Phyrexian symbol in the cost: a hybrid symbol's "nonhybrid
//! equivalent" — one of its two halves ([CR#107.4e]) — and, for a Phyrexian
//! symbol, whether to pay the colored mana or 2 life ([CR#107.4f]). These pure
//! helpers enumerate the legal readings ([`choosable`]) and apply a player's
//! picks to produce a concrete [`ManaCost`] of `Simple` symbols plus any
//! Phyrexian "lose 2 life" verb costs ([`concretize`]). Wiring this into the
//! announce flow is a later task; these are the seam-free building blocks.

use deckmaste_core::CostComponent;
use deckmaste_core::Count;
use deckmaste_core::ManaCost;
use deckmaste_core::ManaSymbol;
use deckmaste_core::PlayerAction;
use deckmaste_core::SimpleManaSymbol;

/// One concrete reading of a multi-way mana symbol ([CR#601.2b]).
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum SymbolChoice {
    /// Pay this concrete simple symbol — a chosen hybrid half ([CR#107.4e]) or
    /// a Phyrexian color ([CR#107.4f]).
    Mana(SimpleManaSymbol),
    /// Pay 2 life instead (Phyrexian only) ([CR#107.4f]).
    Life,
}

/// The legal readings for ONE choosable symbol, in the order the symbol appears
/// in the cost. A hybrid offers its two halves; a Phyrexian offers its color(s)
/// then `Life`.
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct SymbolOptions {
    pub choices: Vec<SymbolChoice>,
}

/// Every choosable symbol in a cost, in cost order. Empty when the cost has no
/// hybrid or Phyrexian symbols.
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct ChoosableOptions {
    pub options: Vec<SymbolOptions>,
}

/// A player's announce ([CR#601.2b]): one chosen reading per choosable symbol,
/// in the same order [`choosable`] lists them.
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct CostOptionChoices {
    pub picks: Vec<SymbolChoice>,
}

/// Why a set of announced picks can't concretize a cost.
#[derive(Debug, Clone, PartialEq, Eq)]
pub enum ConcretizeError {
    /// The number of picks doesn't match the number of choosable symbols.
    WrongPickCount { expected: usize, found: usize },
    /// The pick at this choosable-symbol index isn't one of that symbol's legal
    /// readings ([CR#601.2b]).
    IllegalPick { index: usize },
}

/// The legal nonhybrid/Phyrexian readings of `cost`'s multi-way symbols, in
/// cost order ([CR#601.2b]).
///
/// A hybrid `Hybrid(left, right)` offers `[Mana(left), Mana(right)]`
/// ([CR#107.4e]); a Phyrexian `Phyrexian(c1, c2)` offers its color(s) then
/// `Life` ([CR#107.4f]). `Simple`/`Snow`/`Variable` symbols aren't choosable
/// and contribute nothing.
#[must_use]
pub fn choosable(cost: &ManaCost) -> ChoosableOptions {
    let options = cost.iter().filter_map(symbol_options).collect();
    ChoosableOptions { options }
}

/// The legal readings of one symbol, or `None` if it isn't choosable.
fn symbol_options(symbol: &ManaSymbol) -> Option<SymbolOptions> {
    let choices = match *symbol {
        // [CR#107.4e]: the two halves — the chosen left component, or one mana
        // of the right color. `left` is already a `SimpleManaSymbol` (covers
        // both generic-amount and color halves); `right` is a `Color` and
        // needs `.into()` to become `SimpleManaSymbol::Specific`.
        ManaSymbol::Hybrid(left, right) => {
            vec![SymbolChoice::Mana(left), SymbolChoice::Mana(right.into())]
        }
        // [CR#107.4f]: one mana of each color, then 2 life.
        ManaSymbol::Phyrexian(c1, c2) => {
            let mut choices = vec![SymbolChoice::Mana(c1.into())];
            if let Some(c2) = c2 {
                choices.push(SymbolChoice::Mana(c2.into()));
            }
            choices.push(SymbolChoice::Life);
            choices
        }
        ManaSymbol::Simple(_) | ManaSymbol::Snow | ManaSymbol::Variable => return None,
    };
    Some(SymbolOptions { choices })
}

/// Applies the announced `choices` to `cost`, yielding the concrete nonhybrid
/// [`ManaCost`] plus the verb costs the Phyrexian-life picks incur
/// ([CR#601.2b]).
///
/// Walks the symbols in order, consuming one pick per choosable (hybrid or
/// Phyrexian) symbol. `Simple`/`Snow` pass through unchanged; `Mana` picks
/// become `Simple` symbols in the output mana; each `Life` pick contributes a
/// `Do(LoseLife(2))` cost ([CR#107.4f]) for a later task to pay.
///
/// # Errors
///
/// [`ConcretizeError::WrongPickCount`] if `choices.picks` isn't exactly one per
/// choosable symbol; [`ConcretizeError::IllegalPick`] if a pick isn't one of
/// that symbol's legal readings ([CR#601.2b]) — an illegal announce, surfaced
/// to the player by the submission handler in a later task.
pub fn concretize(
    cost: &ManaCost,
    choices: &CostOptionChoices,
) -> Result<(ManaCost, Vec<CostComponent>), ConcretizeError> {
    let expected = cost
        .iter()
        .filter(|sym| symbol_options(sym).is_some())
        .count();
    if choices.picks.len() != expected {
        return Err(ConcretizeError::WrongPickCount {
            expected,
            found: choices.picks.len(),
        });
    }

    let mut mana = Vec::new();
    let mut verbs = Vec::new();
    // The next pick to consume; advances once per choosable symbol. The count
    // check above guarantees `index < picks.len()` whenever we index here.
    let mut index = 0;

    for symbol in cost.iter() {
        match symbol_options(symbol) {
            // Choosable: consume the next pick and validate it.
            Some(legal) => {
                let pick = choices.picks[index];
                index += 1;
                if !legal.choices.contains(&pick) {
                    return Err(ConcretizeError::IllegalPick { index: index - 1 });
                }
                match pick {
                    SymbolChoice::Mana(sym) => mana.push(ManaSymbol::Simple(sym)),
                    // [CR#107.4f]: 2 life in place of the colored mana.
                    SymbolChoice::Life => {
                        verbs.push(CostComponent::Do(PlayerAction::LoseLife(Count::Literal(2))));
                    }
                }
            }
            // Not choosable: pass through unchanged. `Variable` is announced at
            // [CR#601.2b] too, but X is out of scope here (engine-x-costs).
            None => mana.push(*symbol),
        }
    }

    Ok((ManaCost::from(mana), verbs))
}

#[cfg(test)]
mod tests {
    use deckmaste_core::Color;
    use deckmaste_core::Color::*;
    use deckmaste_core::SimpleManaSymbol::Generic;

    use super::*;

    /// A `Mana` pick of one mana of a color.
    fn color(c: Color) -> SymbolChoice { SymbolChoice::Mana(c.into()) }

    fn parse_cost(s: &str) -> ManaCost { s.parse().expect("test cost parses") }

    /// `{2/W}` is choosable as its two halves; each half concretizes alone.
    #[test]
    fn monohybrid_picks_a_half() {
        let cost = parse_cost("{2/W}");
        assert_eq!(
            choosable(&cost),
            ChoosableOptions {
                options: vec![SymbolOptions {
                    choices: vec![SymbolChoice::Mana(Generic(2)), color(White)],
                }],
            }
        );

        // Pick the white half -> {W}, no verbs.
        let (mana, verbs) = concretize(
            &cost,
            &CostOptionChoices {
                picks: vec![color(White)],
            },
        )
        .expect("white half is legal");
        assert_eq!(mana, parse_cost("{W}"));
        assert!(verbs.is_empty());

        // Pick the generic half -> {2}.
        let (mana, verbs) = concretize(
            &cost,
            &CostOptionChoices {
                picks: vec![SymbolChoice::Mana(Generic(2))],
            },
        )
        .expect("generic half is legal");
        assert_eq!(mana, parse_cost("{2}"));
        assert!(verbs.is_empty());
    }

    /// Two-color hybrid `{W/U}`: picking the blue half -> {U}.
    #[test]
    fn two_color_hybrid_picks_a_color() {
        let (mana, verbs) = concretize(
            &parse_cost("{W/U}"),
            &CostOptionChoices {
                picks: vec![color(Blue)],
            },
        )
        .expect("blue half is legal");
        assert_eq!(mana, parse_cost("{U}"));
        assert!(verbs.is_empty());
    }

    /// `{W/P}` is choosable as `[Mana(W), Life]`; Life -> empty mana + lose-2;
    /// the color -> {W}.
    #[test]
    fn phyrexian_picks_color_or_life() {
        let cost = parse_cost("{W/P}");
        assert_eq!(
            choosable(&cost),
            ChoosableOptions {
                options: vec![SymbolOptions {
                    choices: vec![color(White), SymbolChoice::Life],
                }],
            }
        );

        // Pay 2 life: no mana, one lose-life verb cost.
        let (mana, verbs) = concretize(
            &cost,
            &CostOptionChoices {
                picks: vec![SymbolChoice::Life],
            },
        )
        .expect("life is legal");
        assert!(mana.is_empty());
        assert_eq!(
            verbs,
            vec![CostComponent::Do(PlayerAction::LoseLife(Count::Literal(2)))]
        );

        // Pay the color: {W}, no verbs.
        let (mana, verbs) = concretize(
            &cost,
            &CostOptionChoices {
                picks: vec![color(White)],
            },
        )
        .expect("color is legal");
        assert_eq!(mana, parse_cost("{W}"));
        assert!(verbs.is_empty());
    }

    /// Hybrid-Phyrexian `{W/U/P}` offers either color or life.
    #[test]
    fn hybrid_phyrexian_offers_both_colors_and_life() {
        assert_eq!(
            choosable(&parse_cost("{W/U/P}")),
            ChoosableOptions {
                options: vec![SymbolOptions {
                    choices: vec![color(White), color(Blue), SymbolChoice::Life],
                }],
            }
        );
    }

    /// A plain symbol next to a Phyrexian one passes through; the Phyrexian
    /// life pick still incurs lose-2.
    #[test]
    fn simple_symbol_passes_through() {
        let (mana, verbs) = concretize(
            &parse_cost("{1}{W/P}"),
            &CostOptionChoices {
                picks: vec![SymbolChoice::Life],
            },
        )
        .expect("life is legal");
        assert_eq!(mana, parse_cost("{1}"));
        assert_eq!(
            verbs,
            vec![CostComponent::Do(PlayerAction::LoseLife(Count::Literal(2)))]
        );
    }

    /// A cost with no multi-way symbols: nothing is choosable, and an empty
    /// announce concretizes it unchanged.
    #[test]
    fn no_choice_cost_round_trips() {
        let cost = parse_cost("{1}{G}");
        assert_eq!(choosable(&cost), ChoosableOptions { options: vec![] });
        let (mana, verbs) =
            concretize(&cost, &CostOptionChoices { picks: vec![] }).expect("no picks needed");
        assert_eq!(mana, cost);
        assert!(verbs.is_empty());
    }

    /// An illegal reading (a color the symbol doesn't offer) is rejected.
    #[test]
    fn illegal_pick_is_rejected() {
        assert_eq!(
            concretize(
                &parse_cost("{W/U}"),
                &CostOptionChoices {
                    picks: vec![color(Black)]
                },
            ),
            Err(ConcretizeError::IllegalPick { index: 0 })
        );
    }

    /// Too few / too many picks are rejected before any reading is applied.
    #[test]
    fn wrong_pick_count_is_rejected() {
        // {W/U} needs exactly one pick; none given.
        assert_eq!(
            concretize(&parse_cost("{W/U}"), &CostOptionChoices { picks: vec![] }),
            Err(ConcretizeError::WrongPickCount {
                expected: 1,
                found: 0
            })
        );
        // A no-choice cost needs zero picks; one given.
        assert_eq!(
            concretize(
                &parse_cost("{1}{G}"),
                &CostOptionChoices {
                    picks: vec![color(White)]
                },
            ),
            Err(ConcretizeError::WrongPickCount {
                expected: 0,
                found: 1
            })
        );
    }

    /// `{W/U}{2/R}` has TWO choosable symbols; picks must advance the index
    /// correctly across symbol boundaries so each pick applies to the right
    /// symbol — not both to the first.
    #[test]
    fn concretize_two_choosable_symbols() {
        let cost = parse_cost("{W/U}{2/R}");

        // Two choosable symbols: {W/U} offers [W, U], {2/R} offers [{2}, R].
        assert_eq!(
            choosable(&cost),
            ChoosableOptions {
                options: vec![
                    SymbolOptions {
                        choices: vec![color(White), color(Blue)]
                    },
                    SymbolOptions {
                        choices: vec![SymbolChoice::Mana(Generic(2)), color(Red),],
                    },
                ],
            }
        );

        // Pick U for {W/U} and {2} for {2/R}: concrete cost is {U}{2} (cost
        // iteration order), no verb costs.
        let (mana, verbs) = concretize(
            &cost,
            &CostOptionChoices {
                picks: vec![color(Blue), SymbolChoice::Mana(Generic(2))],
            },
        )
        .expect("U + {2} are legal picks");
        assert_eq!(mana, parse_cost("{U}{2}"));
        assert!(verbs.is_empty());

        // Swap: pick W for {W/U} and R for {2/R}: concrete cost is {W}{R}.
        // This differs from the above, confirming each pick maps to its own
        // symbol and not both to the first.
        let (mana, verbs) = concretize(
            &cost,
            &CostOptionChoices {
                picks: vec![color(White), color(Red)],
            },
        )
        .expect("W + R are legal picks");
        assert_eq!(mana, parse_cost("{W}{R}"));
        assert!(verbs.is_empty());
    }

    /// `{W/P}{U/P}`: picking `Life` for the first Phyrexian and a color for
    /// the second yields exactly ONE lose-life verb and ONE concrete mana
    /// symbol, each mapped to the right symbol.
    #[test]
    fn concretize_two_phyrexian_symbols_mixed_picks() {
        let cost = parse_cost("{W/P}{U/P}");

        // Life for {W/P}, color for {U/P}: one verb, one concrete mana.
        let (mana, verbs) = concretize(
            &cost,
            &CostOptionChoices {
                picks: vec![SymbolChoice::Life, color(Blue)],
            },
        )
        .expect("Life + U are legal picks");
        assert_eq!(mana, parse_cost("{U}"));
        assert_eq!(
            verbs,
            vec![CostComponent::Do(PlayerAction::LoseLife(Count::Literal(2)))]
        );

        // Color for {W/P}, Life for {U/P}: one concrete mana, one verb (order
        // reversed compared to above — life-verb follows, not precedes, {U}).
        let (mana, verbs) = concretize(
            &cost,
            &CostOptionChoices {
                picks: vec![color(White), SymbolChoice::Life],
            },
        )
        .expect("W + Life are legal picks");
        assert_eq!(mana, parse_cost("{W}"));
        assert_eq!(
            verbs,
            vec![CostComponent::Do(PlayerAction::LoseLife(Count::Literal(2)))]
        );
    }
}
