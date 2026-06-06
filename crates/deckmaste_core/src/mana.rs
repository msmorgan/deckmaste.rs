use std::sync::LazyLock;

use regex::Regex;
use serde::Serialize;

use crate::Color;

/// The component symbols hybrid/phyrexian symbols are built from: a generic
/// amount, one of the five colors, or colorless ({C}, which is not a color).
///
/// The untagged Color variant serializes transparently, so the RON stays
/// flat: `White`, not `Color(White)`.
#[derive(Debug, PartialEq, Serialize)]
pub enum SimpleManaSymbol {
    Generic(u16),
    Colorless,
    #[serde(untagged)]
    Color(Color),
}

/// The untagged Simple variant serializes transparently, so the RON stays
/// flat: `Generic(2)`, not `Simple(Generic(2))`.
#[derive(Debug, PartialEq, Serialize)]
pub enum ManaSymbol {
    Variable,
    Snow,
    Hybrid(SimpleManaSymbol, SimpleManaSymbol),
    Phyrexian(SimpleManaSymbol),
    PhyrexianHybrid(SimpleManaSymbol, SimpleManaSymbol),
    #[serde(untagged)]
    Simple(SimpleManaSymbol),
}

fn simple_symbol(code: &str) -> SimpleManaSymbol {
    if code == "C" {
        return SimpleManaSymbol::Colorless;
    }
    Color::from_code(code)
        .map(SimpleManaSymbol::Color)
        .expect("codes are restricted by the symbol regex")
}

/// Parses one `{...}` mana symbol.
pub fn parse_symbol(symbol: &str) -> anyhow::Result<ManaSymbol> {
    static SYMBOL: LazyLock<Regex> = LazyLock::new(|| {
        Regex::new(
            r"(?x)^[{](?:
                (?P<variable>X)
            |
                (?P<snow>S)
            |
                (?:
                    (?P<generic>[0-9]|[1-9][0-9]+)
                |
                    (?P<color>[WUBRGC])
                )
                (?:/(?P<hybrid>[WUBRG]))?
                (?:/(?P<phyrexian>P))?
            )[}]$",
        )
        .unwrap()
    });

    let captures = SYMBOL
        .captures(symbol)
        .ok_or_else(|| anyhow::anyhow!("unrecognized mana symbol: {symbol:?}"))?;

    if captures.name("variable").is_some() {
        return Ok(ManaSymbol::Variable);
    }
    if captures.name("snow").is_some() {
        return Ok(ManaSymbol::Snow);
    }

    let simple = if let Some(generic) = captures.name("generic") {
        SimpleManaSymbol::Generic(generic.as_str().parse()?)
    } else {
        simple_symbol(&captures["color"])
    };
    let hybrid = captures.name("hybrid").map(|h| simple_symbol(h.as_str()));
    let phyrexian = captures.name("phyrexian").is_some();
    Ok(match (hybrid, phyrexian) {
        (Some(hybrid), true) => ManaSymbol::PhyrexianHybrid(simple, hybrid),
        (Some(hybrid), false) => ManaSymbol::Hybrid(simple, hybrid),
        (None, true) => ManaSymbol::Phyrexian(simple),
        (None, false) => ManaSymbol::Simple(simple),
    })
}

/// Parses every `{...}` symbol in a mana cost string like "{2}{W/U}{X}".
pub fn parse_cost(mana_cost: &str) -> anyhow::Result<Vec<ManaSymbol>> {
    static SYMBOLS: LazyLock<Regex> = LazyLock::new(|| Regex::new(r"[{][^}]+[}]").unwrap());

    SYMBOLS
        .find_iter(mana_cost)
        .map(|symbol| parse_symbol(symbol.as_str()))
        .collect()
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn mana_costs() {
        use ManaSymbol::*;
        use SimpleManaSymbol::{Colorless, Generic};
        let color = |c| SimpleManaSymbol::Color(c);

        assert_eq!(
            parse_cost("{1}{G}").unwrap(),
            vec![Simple(Generic(1)), Simple(color(Color::Green))]
        );
        assert_eq!(parse_cost("{X}{S}").unwrap(), vec![Variable, Snow]);
        assert_eq!(
            parse_cost("{2/W}{C/B}").unwrap(),
            vec![
                Hybrid(Generic(2), color(Color::White)),
                Hybrid(Colorless, color(Color::Black)),
            ]
        );
        assert_eq!(
            parse_cost("{G/U/P}{W/P}").unwrap(),
            vec![
                PhyrexianHybrid(color(Color::Green), color(Color::Blue)),
                Phyrexian(color(Color::White)),
            ]
        );
        assert!(parse_cost("{HW}").is_err());
    }
}
