use std::fmt;
use std::str::FromStr;

use serde::{Deserialize, Serialize};

use crate::Color;
use crate::color::ColorOrColorless;

/// Produced-mana spec (CR 106): what colors or types a mana-adding effect
/// may produce. Variants accrete — `AnyType`, riders later.
///
/// The untagged Specific variant serializes transparently, so the RON stays
/// flat: `AddMana(Literal(1), White)`, not `…Specific(White)`. Tagged
/// variants (`AnyColor`, `OneOf`, future `AnyType`, riders) must stay above
/// the `#[serde(untagged)]` line — the untagged arm is tried last.
///
/// Not `Copy`: `OneOf` carries a `Vec`. Nothing `Copy` holds a `ManaSpec`
/// (`Action`/`Token` are `Clone`), so the spec stays `Clone`.
#[derive(Debug, Clone, PartialEq, Eq, Hash, Deserialize, Serialize)]
pub enum ManaSpec {
    AnyColor,
    /// One mana of a color the controller chooses from a fixed set on
    /// resolution ("{W} or {U}", CR 106.1b) — a single mana ability, not a
    /// CR 700.2 modal choice. Members keep their printed order.
    OneOf(Vec<ColorOrColorless>),
    #[serde(untagged)]
    Specific(ColorOrColorless),
}

impl From<ColorOrColorless> for ManaSpec {
    fn from(color_or_colorless: ColorOrColorless) -> Self { Self::Specific(color_or_colorless) }
}

impl From<Color> for ManaSpec {
    fn from(color: Color) -> Self { Self::Specific(color.into()) }
}

/// The component symbols hybrid/phyrexian symbols are built from: a generic
/// amount, one of the five colors, or colorless ({C}, which is not a color).
///
/// The untagged Color variant serializes transparently, so the RON stays
/// flat: `White`, not `Color(White)`.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, Deserialize, Serialize)]
pub enum SimpleManaSymbol {
    Generic(crate::Uint),
    #[serde(untagged)]
    Specific(ColorOrColorless),
}

impl SimpleManaSymbol {
    #[must_use]
    pub fn color(&self) -> Option<Color> {
        match self {
            &Self::Specific(c) => c.color(),
            _ => None,
        }
    }
}

impl From<Color> for SimpleManaSymbol {
    fn from(color: Color) -> Self { Self::Specific(color.into()) }
}

impl From<ColorOrColorless> for SimpleManaSymbol {
    fn from(color: ColorOrColorless) -> Self { Self::Specific(color) }
}

impl From<crate::Uint> for SimpleManaSymbol {
    fn from(amount: crate::Uint) -> Self { Self::Generic(amount) }
}

/// The untagged Simple variant serializes transparently, so the RON stays
/// flat: `Generic(2)`, not `Simple(Generic(2))`.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, Deserialize, Serialize)]
pub enum ManaSymbol {
    Variable,
    Snow,
    Hybrid(SimpleManaSymbol, Color), // Slightly more permissive than CR107.4.
    Phyrexian(Color, Option<Color>),
    #[serde(untagged)]
    Simple(SimpleManaSymbol),
}

impl From<Color> for ManaSymbol {
    fn from(color: Color) -> Self { Self::Simple(color.into()) }
}

impl From<ColorOrColorless> for ManaSymbol {
    fn from(color: ColorOrColorless) -> Self { Self::Simple(color.into()) }
}

impl From<crate::Uint> for ManaSymbol {
    fn from(amount: crate::Uint) -> Self { Self::Simple(amount.into()) }
}

#[derive(Debug, Clone, Default, PartialEq, Eq, Hash, Deserialize, Serialize)]
#[serde(transparent)]
pub struct ManaCost(Vec<ManaSymbol>);

impl ManaCost {
    #[must_use]
    pub fn is_empty(&self) -> bool { self.0.is_empty() }
}

impl From<Vec<ManaSymbol>> for ManaCost {
    fn from(symbols: Vec<ManaSymbol>) -> Self { Self(symbols) }
}

impl From<ManaCost> for Vec<ManaSymbol> {
    fn from(cost: ManaCost) -> Self { cost.0 }
}

impl std::ops::Deref for ManaCost {
    type Target = [ManaSymbol];

    fn deref(&self) -> &Self::Target { &self.0 }
}

/// The error type for [`ManaSymbol`] and [`ManaCost`]'s [`FromStr`] impls.
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct ParseManaError {
    symbol: String,
}

impl ParseManaError {
    fn new(symbol: &str) -> Self {
        Self {
            symbol: symbol.to_owned(),
        }
    }
}

impl fmt::Display for ParseManaError {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "unrecognized mana symbol: {:?}", self.symbol)
    }
}

impl std::error::Error for ParseManaError {}

/// Parses a generic amount, insisting on the canonical form: digits only with
/// no leading zeros (unlike `Uint::from_str`, which also accepts "+1").
fn parse_generic(code: &str) -> Option<crate::Uint> {
    let canonical = !code.is_empty()
        && code.bytes().all(|b| b.is_ascii_digit())
        && (code.len() == 1 || !code.starts_with('0'));
    if canonical { code.parse().ok() } else { None }
}

fn parse_simple(code: &str) -> Option<SimpleManaSymbol> {
    ColorOrColorless::from_code(code)
        .map(SimpleManaSymbol::Specific)
        .or_else(|| parse_generic(code).map(SimpleManaSymbol::Generic))
}

/// Parses the body of a `{...}` mana symbol, braces already stripped.
fn parse_symbol_body(body: &str) -> Option<ManaSymbol> {
    Some(match *body.split('/').collect::<Vec<_>>() {
        ["X"] => ManaSymbol::Variable,
        ["S"] => ManaSymbol::Snow,
        [simple] => ManaSymbol::Simple(parse_simple(simple)?),
        // Phyrexian symbols need a colored left half: there is no {2/P} or {C/P}.
        [simple, "P"] => ManaSymbol::Phyrexian(parse_simple(simple)?.color()?, None),
        [simple, hybrid] => ManaSymbol::Hybrid(parse_simple(simple)?, Color::from_code(hybrid)?),
        [simple, hybrid, "P"] => ManaSymbol::Phyrexian(
            parse_simple(simple)?.color()?,
            Some(Color::from_code(hybrid)?),
        ),
        _ => return None,
    })
}

impl FromStr for ManaSymbol {
    type Err = ParseManaError;

    /// Parses one `{...}` mana symbol.
    fn from_str(symbol: &str) -> Result<Self, Self::Err> {
        symbol
            .strip_prefix('{')
            .and_then(|body| body.strip_suffix('}'))
            .and_then(parse_symbol_body)
            .ok_or_else(|| ParseManaError::new(symbol))
    }
}

impl FromStr for ManaCost {
    type Err = ParseManaError;

    /// Parses a mana cost like "{2}{W/U}{X}": a string of symbols and nothing
    /// else. The empty string is the empty cost.
    fn from_str(mana_cost: &str) -> Result<Self, Self::Err> {
        let mut symbols = Vec::new();
        let mut rest = mana_cost;
        while !rest.is_empty() {
            let end = rest.find('}').map_or(rest.len(), |close| close + 1);
            let (symbol, tail) = rest.split_at(end);
            symbols.push(symbol.parse()?);
            rest = tail;
        }
        Ok(Self(symbols))
    }
}

#[cfg(test)]
mod tests {
    use Color::*;
    use ColorOrColorless::Colorless;
    use ManaSymbol::*;
    use SimpleManaSymbol::{Generic, Specific};

    use super::*;

    fn symbol(s: &str) -> Result<ManaSymbol, ParseManaError> { s.parse() }

    #[test]
    fn mana_symbols() {
        assert_eq!(symbol("{W}").unwrap(), White.into());
        assert_eq!(symbol("{C}").unwrap(), Simple(Specific(Colorless)));
        assert_eq!(symbol("{0}").unwrap(), Simple(Generic(0)));
        assert_eq!(symbol("{15}").unwrap(), Simple(Generic(15)));
        assert_eq!(symbol("{1000000}").unwrap(), Simple(Generic(1_000_000))); // Gleemax
        assert_eq!(symbol("{X}").unwrap(), Variable);
        assert_eq!(symbol("{S}").unwrap(), Snow);
        assert_eq!(
            symbol("{G/U}").unwrap(),
            Hybrid(Specific(Green.into()), Blue)
        );
        assert_eq!(symbol("{2/W}").unwrap(), Hybrid(Generic(2), White));
        assert_eq!(symbol("{C/B}").unwrap(), Hybrid(Specific(Colorless), Black));
        assert_eq!(symbol("{R/P}").unwrap(), Phyrexian(Red, None));
        assert_eq!(symbol("{G/U/P}").unwrap(), Phyrexian(Green, Some(Blue)));
    }

    #[test]
    fn invalid_mana_symbols() {
        for invalid in [
            "",
            "W",
            "{W",
            "W}",
            "{}",
            "{w}",
            "{ W }",
            "{HW}",
            "{T}",
            "{P}",
            // Non-canonical or overflowing generic amounts.
            "{01}",
            "{+1}",
            "{4294967296}",
            // Phyrexian symbols need a colored left half.
            "{2/P}",
            "{C/P}",
            "{2/W/P}",
            "{X/P}",
            // The right half of a hybrid must be a color.
            "{W/C}",
            "{W/2}",
            "{W/X}",
            "{/W}",
            "{W/}",
            "{W/U/B}",
            "{G/U/P/P}",
        ] {
            assert!(symbol(invalid).is_err(), "{invalid:?} should not parse");
        }
    }

    #[test]
    fn mana_costs() {
        let cost = |s: &str| s.parse::<ManaCost>();

        assert_eq!(
            cost("{1}{G}").unwrap(),
            ManaCost(vec![Simple(Generic(1)), Green.into()])
        );
        assert_eq!(cost("{X}{S}").unwrap(), ManaCost(vec![Variable, Snow]));
        assert_eq!(
            cost("{2/W}{C/B}").unwrap(),
            ManaCost(vec![
                Hybrid(Generic(2), White),
                Hybrid(Specific(Colorless), Black),
            ])
        );
        assert_eq!(
            cost("{G/U/P}{W/P}").unwrap(),
            ManaCost(vec![Phyrexian(Green, Some(Blue)), Phyrexian(White, None),])
        );
        assert_eq!(cost("").unwrap(), ManaCost::default());

        // ManaCost derefs to its symbols.
        assert_eq!(cost("{1}{G}").unwrap().len(), 2);
        assert_eq!(cost("{X}").unwrap().first(), Some(&Variable));
    }

    #[test]
    fn invalid_mana_costs() {
        for invalid in [
            " {W}", "{W} {U}", "{W}junk", "junk{W}", "{W}{", "{1}}", "{X}{HW}",
        ] {
            assert!(
                invalid.parse::<ManaCost>().is_err(),
                "{invalid:?} should not parse"
            );
        }
    }

    #[test]
    fn mana_spec_specific_reads_flat() {
        let read = |s: &str| crate::ron::options().from_str::<ManaSpec>(s).unwrap();
        assert_eq!(read("AnyColor"), ManaSpec::AnyColor);
        assert_eq!(read("White"), ManaSpec::Specific(White.into()));
        assert_eq!(read("Colorless"), ManaSpec::Specific(Colorless));
    }

    #[test]
    fn mana_spec_specific_writes_flat() {
        let write = |m: &ManaSpec| crate::ron::options().to_string(m).unwrap();
        assert_eq!(write(&ManaSpec::AnyColor), "AnyColor");
        assert_eq!(write(&ManaSpec::Specific(White.into())), "White");
        assert_eq!(write(&ManaSpec::Specific(Colorless)), "Colorless");
    }

    /// "{W} or {U}" lands: one mana, color chosen at resolution. The colors
    /// keep their printed order and each spells flat (`White`, not
    /// `Color(White)`).
    #[test]
    fn mana_spec_one_of_round_trips() {
        let read = |s: &str| crate::ron::options().from_str::<ManaSpec>(s).unwrap();
        let spec = read("OneOf([White, Blue])");
        assert_eq!(spec, ManaSpec::OneOf(vec![White.into(), Blue.into()]));
        let write = |m: &ManaSpec| crate::ron::options().to_string(m).unwrap();
        assert_eq!(write(&spec), "OneOf([White,Blue])");
        // Colorless is a valid member too ({C} or {U} appears in the corpus).
        assert_eq!(
            read("OneOf([Colorless, Blue])"),
            ManaSpec::OneOf(vec![Colorless, Blue.into()])
        );
    }
}
