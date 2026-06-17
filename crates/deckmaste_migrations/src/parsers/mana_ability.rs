//! The shared `<cost>: Add …` mana-ability grammar and rendering. Reads
//! `~`-normalized oracle text, so enters-tapped is `"~ enters tapped."`.
//!
//! The activation cost is the full pre-colon clause ([CR#602.1a]), parsed by
//! the shared [`crate::parsers::cost`] grammar — so `{1}, {T}`, `{T}, Sacrifice
//! ~`, `{T}, Pay 1 life` all front a mana ability, not just a bare `{T}`. The
//! production after `Add ` is a fixed run, a color choice ([CR#106.1b]), "one
//! mana of any color", or a `for each` scaler. A painland tail (`. ~ deals N
//! damage to you.`) rides as a second effect in the same resolution.

use deckmaste_core::ColorOrColorless;

use crate::parsers::cost::VariableMana;
use crate::parsers::cost::{self};
use crate::parsers::count;
use crate::resolve::ResolveCtx;
use crate::ron_output::to_string_pretty;

/// A parsed `Add …` production ([CR#106]).
pub(super) enum Production {
    /// "one mana of any color".
    AnyColor,
    /// "{W} or {U}" / "{U}, {B}, or {R}" — one mana, color chosen from a set
    /// on resolution ([CR#106.1b]). Members keep printed order. Each member is
    /// a SINGLE symbol; a multi-symbol choice ("{W}{W}, {W}{U}, or {U}{U}") has
    /// no engine spec and is declined.
    OneOf(Vec<ColorOrColorless>),
    /// A run of mana symbols, consecutive identical specs run-length encoded:
    /// `{C}{C}` -> `[(2, Colorless)]`, `{W}{U}` -> `[(1, White), (1, Blue)]`.
    Fixed(Vec<(u32, ColorOrColorless)>),
    /// "{G} for each <filter>" — a dynamic count of one symbol ([CR#107.3]).
    /// Only a single symbol scales; the `String` is `CountOf(<filter>)` RON.
    Scaled(String, ColorOrColorless),
}

/// One simple ability on a tap-for-mana permanent.
pub(super) enum TapAbility {
    /// `~ enters tapped.` (lands and rocks; creatures never enter tapped).
    EntersTapped,
    /// `<cost>: Add <production>[. <rider>]` ([CR#602.1a,605]). The cost is the
    /// pre-parsed list of `CostComponent` RON strings.
    Mana {
        cost: Vec<String>,
        production: Production,
        rider: Option<Rider>,
    },
}

/// An extra clause a mana ability resolves alongside its `Add` ([CR#605.1a]).
/// Painlands tax the controller a fixed amount of damage.
pub(super) enum Rider {
    /// "~ deals N damage to you." — the source deals damage to its controller
    /// ([CR#120.1]).
    DamageToYou(u32),
}

/// "{W}" -> White, "{C}" -> Colorless. Only single colored/colorless symbols.
fn symbol_color(symbol: &str) -> Option<ColorOrColorless> {
    ColorOrColorless::from_code(symbol.strip_prefix('{')?.strip_suffix('}')?)
}

/// Splits a contiguous run of `{X}` symbols into colors, or `None` if any
/// token isn't a single colored/colorless symbol.
fn parse_symbol_run(text: &str) -> Option<Vec<ColorOrColorless>> {
    if text.is_empty() {
        return None;
    }
    let mut colors = Vec::new();
    let mut rest = text;
    while !rest.is_empty() {
        let close = rest.find('}')? + 1;
        let (symbol, tail) = rest.split_at(close);
        colors.push(symbol_color(symbol)?);
        rest = tail;
    }
    Some(colors)
}

/// Run-length encodes consecutive identical specs.
fn run_length_encode(colors: &[ColorOrColorless]) -> Vec<(u32, ColorOrColorless)> {
    let mut runs: Vec<(u32, ColorOrColorless)> = Vec::new();
    for &color in colors {
        match runs.last_mut() {
            Some((count, spec)) if *spec == color => *count += 1,
            _ => runs.push((1, color)),
        }
    }
    runs
}

/// Parses the text after `Add ` (trailing `.` already stripped).
pub(super) fn parse_production(text: &str) -> Option<Production> {
    // "<symbol> for each <filter>": only a single fixed symbol scales.
    if let Some(clause) = count::strip(text) {
        if !matches!(clause.binder, count::Binder::ForEach) {
            return None;
        }
        let colors = parse_symbol_run(clause.head.trim())?;
        let [color] = colors.as_slice() else {
            return None;
        };
        return Some(Production::Scaled(clause.count, *color));
    }
    if text == "one mana of any color" {
        return Some(Production::AnyColor);
    }
    if text.contains(" or ") {
        return parse_one_of(text);
    }
    Some(Production::Fixed(run_length_encode(&parse_symbol_run(
        text,
    )?)))
}

/// A color-choice production: `{W} or {U}` or the Oxford-comma list `{U}, {B},
/// or {R}` ([CR#106.1b]). Splits on `, ` then `or `, so the last item's `, or`
/// yields the same single-symbol tokens as the two-item ` or `. Each token
/// must be ONE colored/colorless symbol; a multi-symbol member (`{W}{W}`)
/// declines — `ManaSpec::OneOf` holds single colors, not runs.
fn parse_one_of(text: &str) -> Option<Production> {
    let colors = text
        .replace(", or ", ", ")
        .split(", ")
        .flat_map(|tok| tok.split(" or "))
        .map(symbol_color)
        .collect::<Option<Vec<_>>>()?;
    // A bare " or " between two symbols is the two-item form; the comma form
    // needs at least three. Either way two or more members is required.
    if colors.len() < 2 {
        return None;
    }
    Some(Production::OneOf(colors))
}

/// Splits off a painland rider: the `Add` body and an optional trailing clause
/// after the production's `.`. `"{W} or {B}. ~ deals 1 damage to you"` ->
/// `("{W} or {B}", Some(DamageToYou(1)))`. A `.`-and-clause the rider grammar
/// can't read leaves the rider `None` and the `.` inside the body, so the
/// production parse declines the whole line. No trailing clause -> `(body,
/// None)`.
fn split_rider(body: &str) -> (&str, Option<Rider>) {
    if let Some((head, tail)) = body.split_once(". ")
        && let Some(rider) = parse_rider(tail)
    {
        return (head, Some(rider));
    }
    (body, None)
}

/// `~ deals N damage to you` -> [`Rider::DamageToYou`]; anything else declines.
fn parse_rider(text: &str) -> Option<Rider> {
    let amount = text
        .strip_prefix("~ deals ")?
        .strip_suffix(" damage to you")?;
    Some(Rider::DamageToYou(amount.parse().ok()?))
}

/// Parses one normalized oracle line as a tap ability, or `None`. The cost is
/// any pre-colon clause the shared cost grammar accepts ([CR#602.1a]).
pub(super) fn parse_tap_ability(line: &str) -> anyhow::Result<Option<TapAbility>> {
    if line == "~ enters tapped." {
        return Ok(Some(TapAbility::EntersTapped));
    }
    let Some((cost_clause, add_clause)) = line.split_once(": ") else {
        return Ok(None);
    };
    let Some(add_body) = add_clause
        .strip_prefix("Add ")
        .and_then(|s| s.strip_suffix('.'))
    else {
        return Ok(None);
    };
    // No variable mana in a mana ability's own production cost: {X} costs front
    // spells/X-abilities, not "{T}: Add" rocks.
    let Some(cost) = cost::parse_cost(cost_clause, VariableMana::Decline)? else {
        return Ok(None);
    };
    let (production_text, rider) = split_rider(add_body);
    let Some(production) = parse_production(production_text) else {
        return Ok(None);
    };
    Ok(Some(TapAbility::Mana {
        cost,
        production,
        rider,
    }))
}

/// The `AddMana` instruction(s) for a production: one `AddMana` for a one-run
/// production, a `Sequence` for a heterogeneous run.
fn render_production(production: &Production) -> anyhow::Result<String> {
    Ok(match production {
        Production::AnyColor => "AddMana(1, AnyColor)".to_owned(),
        Production::OneOf(colors) => {
            let inner = colors
                .iter()
                .map(to_string_pretty)
                .collect::<Result<Vec<_>, _>>()?
                .join(", ");
            format!("AddMana(1, OneOf([{inner}]))")
        }
        Production::Fixed(runs) => {
            let adds = runs
                .iter()
                .map(|(count, spec)| Ok(format!("AddMana({count}, {})", to_string_pretty(spec)?)))
                .collect::<anyhow::Result<Vec<_>>>()?;
            if adds.len() == 1 {
                adds.into_iter().next().unwrap()
            } else {
                format!("Sequence([{}])", adds.join(", "))
            }
        }
        Production::Scaled(count, spec) => {
            format!("AddMana({count}, {})", to_string_pretty(spec)?)
        }
    })
}

/// The resolution effect: the production, plus any rider folded into a
/// `Sequence` after it (the `Add` happens first, [CR#605.1a]).
fn render_effect(production: &Production, rider: Option<&Rider>) -> anyhow::Result<String> {
    let add = render_production(production)?;
    Ok(match rider {
        None => add,
        Some(Rider::DamageToYou(n)) => format!("Sequence([{add}, DealDamage(You, {n})])"),
    })
}

/// A registry parser: a `<cost>: Add …` / `~ enters tapped.` line -> the bare
/// RON of one ability, or `None`.
pub(crate) fn resolve_line(line: &str, _ctx: &ResolveCtx) -> anyhow::Result<Option<String>> {
    let Some(ability) = parse_tap_ability(line)? else {
        return Ok(None);
    };
    Ok(Some(render_bare(&ability)?))
}

/// The bare ability RON (no indent/comma) for a `TapAbility`.
fn render_bare(ability: &TapAbility) -> anyhow::Result<String> {
    Ok(match ability {
        TapAbility::EntersTapped => {
            "Static(effects: [Replacement(AsEnters(Tap(This)))])".to_owned()
        }
        TapAbility::Mana {
            cost,
            production,
            rider,
        } => {
            format!(
                "Activated(cost: [{}], effect: {})",
                cost.join(", "),
                render_effect(production, rider.as_ref())?
            )
        }
    })
}

#[cfg(test)]
mod tests {
    use super::*;

    fn effect(text: &str) -> String {
        let Some(TapAbility::Mana {
            production, rider, ..
        }) = parse_tap_ability(&format!("{{T}}: Add {text}.")).unwrap()
        else {
            panic!("not a mana ability");
        };
        render_effect(&production, rider.as_ref()).unwrap()
    }

    /// The bare ability RON for a full `<cost>: Add …` line.
    fn ability(line: &str) -> Option<String> {
        resolve_line(
            line,
            &crate::parsers::test_ctx::ctx(crate::resolve::CardKind::Permanent),
        )
        .unwrap()
    }

    #[test]
    fn single_symbol() {
        assert_eq!(effect("{C}"), "AddMana(1, Colorless)");
        assert_eq!(effect("{G}"), "AddMana(1, Green)");
    }

    #[test]
    fn homogeneous_run_uses_a_count() {
        assert_eq!(effect("{C}{C}"), "AddMana(2, Colorless)");
        assert_eq!(effect("{C}{C}{C}"), "AddMana(3, Colorless)");
    }

    #[test]
    fn heterogeneous_run_is_a_sequence() {
        assert_eq!(
            effect("{W}{U}"),
            "Sequence([AddMana(1, White), AddMana(1, Blue)])"
        );
        assert_eq!(
            effect("{G}{G}{W}"),
            "Sequence([AddMana(2, Green), AddMana(1, White)])"
        );
    }

    #[test]
    fn one_of_and_any_color() {
        assert_eq!(effect("{W} or {U}"), "AddMana(1, OneOf([White, Blue]))");
        assert_eq!(effect("one mana of any color"), "AddMana(1, AnyColor)");
    }

    /// The Oxford-comma three-color choice ([CR#106.1b]): one mana, color
    /// chosen from a list. `{C}` may be a member.
    #[test]
    fn one_of_three_colors() {
        assert_eq!(
            effect("{U}, {B}, or {R}"),
            "AddMana(1, OneOf([Blue, Black, Red]))"
        );
        assert_eq!(
            effect("{W}, {U}, {B}, {R}, or {G}"),
            "AddMana(1, OneOf([White, Blue, Black, Red, Green]))"
        );
    }

    /// A multi-symbol color choice ("{W}{W}, {W}{U}, or {U}{U}") has no engine
    /// spec — `ManaSpec::OneOf` holds single colors, not runs — so it declines.
    #[test]
    fn one_of_multi_symbol_declines() {
        assert!(parse_production("{W}{W}, {W}{U}, or {U}{U}").is_none());
        assert!(parse_production("{C}{C}, or {U}{U}").is_none());
    }

    /// The cost is the full pre-colon clause: generic mana, sacrifice-self,
    /// pay-life all front a mana ability ([CR#602.1a]).
    #[test]
    fn cost_clause_generalizes_beyond_tap() {
        assert_eq!(
            ability("{1}, {T}: Add {W}{U}."),
            Some(
                "Activated(cost: [Mana([Generic(1)]), Tap], effect: \
                 Sequence([AddMana(1, White), AddMana(1, Blue)]))"
                    .to_owned()
            )
        );
        assert_eq!(
            ability("{1}, {T}: Add one mana of any color."),
            Some(
                "Activated(cost: [Mana([Generic(1)]), Tap], effect: AddMana(1, AnyColor))"
                    .to_owned()
            )
        );
        assert_eq!(
            ability("{T}, Sacrifice ~: Add {B}{B}."),
            Some("Activated(cost: [Tap, SacrificeThis], effect: AddMana(2, Black))".to_owned())
        );
        assert_eq!(
            ability("{T}, Pay 1 life: Add one mana of any color."),
            Some(
                "Activated(cost: [Tap, Do(LoseLife(1))], effect: AddMana(1, AnyColor))".to_owned()
            )
        );
    }

    /// A painland rider taxes the controller damage; the `Add` runs first,
    /// then the damage, in one resolution ([CR#605.1a]).
    #[test]
    fn painland_damage_rider() {
        assert_eq!(
            ability("{T}: Add {W} or {B}. ~ deals 1 damage to you."),
            Some(
                "Activated(cost: [Tap], effect: \
                 Sequence([AddMana(1, OneOf([White, Black])), DealDamage(You, 1)]))"
                    .to_owned()
            )
        );
        // A rider clause the grammar can't read leaves the `.` in the body, so
        // the production parse declines the whole line.
        assert!(
            ability("{T}: Add {W} or {B}. ~ doesn't untap during your next untap step.").is_none()
        );
    }

    /// An {X} mana component in a mana ability's own cost is declined.
    #[test]
    fn variable_cost_declines() {
        assert!(ability("{X}, {T}: Add {C}.").is_none());
    }

    #[test]
    fn scaled_for_each_single_symbol() {
        // Elvish Archdruid — the `Permanent` scope ([CR#109.2]) keeps the
        // count off the ability's own on-stack copy (it would otherwise tap
        // for one extra mana).
        assert_eq!(
            effect("{G} for each Elf you control"),
            "AddMana(CountOf(AllOf([Permanent, Subtype(\"Elf\"), ControlledBy(Ref(You))])), Green)"
        );
        // Priest of Titania (battlefield scope, made explicit on the head).
        assert_eq!(
            effect("{G} for each Elf on the battlefield"),
            "AddMana(CountOf(AllOf([Permanent, Subtype(\"Elf\")])), Green)"
        );
    }

    #[test]
    fn scaled_declines_multi_symbol_and_choice() {
        // Multi-symbol "for each" would need a Count product -> decline.
        assert!(parse_production("{G}{G} for each Elf you control").is_none());
        // A color-choice / any-color base doesn't take a for-each scaler here.
        assert!(parse_production("{W} or {U} for each Elf you control").is_none());
    }

    #[test]
    fn declines_non_mana_and_garbage() {
        assert!(parse_tap_ability("Flying").unwrap().is_none());
        // A cost colon but no `Add` body (e.g. a non-mana activated ability)
        // declines, leaving it to the activated-ability parser.
        assert!(parse_tap_ability("{T}: Draw a card.").unwrap().is_none());
        // An unrecognized cost component declines the whole line.
        assert!(
            parse_tap_ability("{T}, Frobnicate: Add {C}.")
                .unwrap()
                .is_none()
        );
        // Bare braces / unknown symbols don't parse.
        assert!(parse_production("{Q}").is_none());
        assert!(parse_production("").is_none());
    }

    #[test]
    fn resolve_line_bare() {
        use crate::resolve::CardKind;
        assert_eq!(
            resolve_line(
                "{T}: Add {G}.",
                &crate::parsers::test_ctx::ctx(CardKind::Permanent)
            )
            .unwrap()
            .as_deref(),
            Some("Activated(cost: [Tap], effect: AddMana(1, Green))")
        );
        assert_eq!(
            resolve_line(
                "~ enters tapped.",
                &crate::parsers::test_ctx::ctx(CardKind::Permanent)
            )
            .unwrap()
            .as_deref(),
            Some("Static(effects: [Replacement(AsEnters(Tap(This)))])")
        );
        assert!(
            resolve_line(
                "Draw a card.",
                &crate::parsers::test_ctx::ctx(CardKind::Permanent)
            )
            .unwrap()
            .is_none()
        );
        assert_eq!(
            resolve_line(
                "{T}: Add {W}{U}.",
                &crate::parsers::test_ctx::ctx(CardKind::Permanent)
            )
            .unwrap()
            .as_deref(),
            Some("Activated(cost: [Tap], effect: Sequence([AddMana(1, White), AddMana(1, Blue)]))")
        );
    }
}
