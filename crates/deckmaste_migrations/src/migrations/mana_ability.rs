//! The shared `{T}: Add …` mana-ability grammar and rendering: the `_007`
//! lands, `_009` rocks, and `_010` dorks all tap for mana the same way. Reads
//! `_004`'s `~`-normalized text, so enters-tapped is `"~ enters tapped."`.

use deckmaste_core::ColorOrColorless;

use crate::ron_output::to_string_pretty;

/// A parsed `Add …` production ([CR#106]).
pub(super) enum Production {
    /// "one mana of any color".
    AnyColor,
    /// "{W} or {U}" — one mana, color chosen from a set on resolution
    /// ([CR#106.1b]). Members keep printed order.
    OneOf(Vec<ColorOrColorless>),
    /// A run of mana symbols, consecutive identical specs run-length encoded:
    /// `{C}{C}` -> `[(2, Colorless)]`, `{W}{U}` -> `[(1, White), (1, Blue)]`.
    Fixed(Vec<(u32, ColorOrColorless)>),
}

/// One simple ability on a tap-for-mana permanent.
pub(super) enum TapAbility {
    /// `~ enters tapped.` (lands and rocks; creatures never enter tapped).
    EntersTapped,
    /// `{T}: Add <production>` ([CR#605]).
    Mana(Production),
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
    if text == "one mana of any color" {
        return Some(Production::AnyColor);
    }
    if text.contains(" or ") {
        let colors = text
            .split(" or ")
            .map(symbol_color)
            .collect::<Option<Vec<_>>>()?;
        return Some(Production::OneOf(colors));
    }
    Some(Production::Fixed(run_length_encode(&parse_symbol_run(
        text,
    )?)))
}

/// Parses one normalized oracle line as a tap ability, or `None`.
pub(super) fn parse_tap_ability(line: &str) -> Option<TapAbility> {
    if line == "~ enters tapped." {
        return Some(TapAbility::EntersTapped);
    }
    let production = line.strip_prefix("{T}: Add ")?.strip_suffix('.')?;
    Some(TapAbility::Mana(parse_production(production)?))
}

/// The produced-mana effect: a single `AddMana` for a one-run production, a
/// `Sequence` of them for a heterogeneous run.
fn render_effect(production: &Production) -> anyhow::Result<String> {
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
    })
}

/// One ability block at the `abilities:` items indent (8 spaces), with its
/// trailing comma + newline.
pub(super) fn render_tap_ability(ability: &TapAbility) -> anyhow::Result<String> {
    Ok(match ability {
        TapAbility::EntersTapped =>
            "        Static(\n            effects: [Replacement(AsEnters(effect: Tap(This)))],\n        ),\n"
                .to_owned(),
        TapAbility::Mana(production) => format!(
            "        Activated(\n            cost: [Tap],\n            effect: {effect},\n        ),\n",
            effect = render_effect(production)?
        ),
    })
}

#[cfg(test)]
mod tests {
    use super::*;

    fn effect(text: &str) -> String {
        let TapAbility::Mana(production) =
            parse_tap_ability(&format!("{{T}}: Add {text}.")).unwrap()
        else {
            panic!("not a mana ability");
        };
        render_effect(&production).unwrap()
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

    #[test]
    fn declines_non_mana_and_garbage() {
        assert!(parse_tap_ability("Flying").is_none());
        // Bare braces / unknown symbols don't parse.
        assert!(parse_production("{Q}").is_none());
        assert!(parse_production("").is_none());
    }

    #[test]
    fn enters_tapped_renders_the_static() {
        let block = render_tap_ability(&parse_tap_ability("~ enters tapped.").unwrap()).unwrap();
        assert_eq!(
            block,
            "        Static(\n            effects: [Replacement(AsEnters(effect: Tap(This)))],\n        ),\n"
        );
    }
}
