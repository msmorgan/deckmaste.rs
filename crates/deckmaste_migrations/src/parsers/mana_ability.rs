//! The shared `{T}: Add …` mana-ability grammar and rendering. Reads
//! `~`-normalized oracle text, so enters-tapped is `"~ enters tapped."`.

use deckmaste_core::ColorOrColorless;

use crate::parsers::count;
use crate::resolve::CardKind;
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
    /// "{G} for each <filter>" — a dynamic count of one symbol ([CR#107.3]).
    /// Only a single symbol scales; the `String` is `CountOf(<filter>)` RON.
    Scaled(String, ColorOrColorless),
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
        Production::Scaled(count, spec) => {
            format!("AddMana({count}, {})", to_string_pretty(spec)?)
        }
    })
}

/// A registry parser: a `{T}: Add …` / `~ enters tapped.` line -> the bare RON
/// of one ability, or `None`.
pub(crate) fn resolve_line(line: &str, _kind: CardKind) -> anyhow::Result<Option<String>> {
    let Some(ability) = parse_tap_ability(line) else {
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
        TapAbility::Mana(production) => {
            format!(
                "Activated(cost: [Tap], effect: {})",
                render_effect(production)?
            )
        }
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
    fn scaled_for_each_single_symbol() {
        // Elvish Archdruid.
        assert_eq!(
            effect("{G} for each Elf you control"),
            "AddMana(CountOf(AllOf([Subtype(\"Elf\"), ControlledBy(Ref(You))])), Green)"
        );
        // Priest of Titania (battlefield scope).
        assert_eq!(
            effect("{G} for each Elf on the battlefield"),
            "AddMana(CountOf(Subtype(\"Elf\")), Green)"
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
        assert!(parse_tap_ability("Flying").is_none());
        // Bare braces / unknown symbols don't parse.
        assert!(parse_production("{Q}").is_none());
        assert!(parse_production("").is_none());
    }

    #[test]
    fn resolve_line_bare() {
        use crate::resolve::CardKind;
        assert_eq!(
            resolve_line("{T}: Add {G}.", CardKind::Permanent)
                .unwrap()
                .as_deref(),
            Some("Activated(cost: [Tap], effect: AddMana(1, Green))")
        );
        assert_eq!(
            resolve_line("~ enters tapped.", CardKind::Permanent)
                .unwrap()
                .as_deref(),
            Some("Static(effects: [Replacement(AsEnters(Tap(This)))])")
        );
        assert!(
            resolve_line("Draw a card.", CardKind::Permanent)
                .unwrap()
                .is_none()
        );
        assert_eq!(
            resolve_line("{T}: Add {W}{U}.", CardKind::Permanent)
                .unwrap()
                .as_deref(),
            Some("Activated(cost: [Tap], effect: Sequence([AddMana(1, White), AddMana(1, Blue)]))")
        );
    }
}
