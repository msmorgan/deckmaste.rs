use std::fmt;
use std::str::FromStr;
use std::sync::Arc;

use serde::Deserialize;
use serde::Serialize;

use crate::Color;
use crate::Expand;
use crate::Expansion;
use crate::Reference;
use crate::SupportsMacros;
use crate::color::ColorOrColorless;

/// The (Planechase) planar die's face ([CR#901.3a] — one Planeswalker
/// symbol, one chaos symbol, four blanks, collapsed here to one `Blank` — a
/// no-op roll, [CR#901.9a]). `Chaos` triggers the chaos ability
/// ([CR#901.9b,311.7]), `Planeswalker` the planeswalking ability
/// ([CR#901.8,901.9c]). Placed here (a small companion enum) rather than in
/// `event.rs`, mirroring `ColorOrColorless`/`SimpleManaSymbol`'s home; unlike
/// those, `PlanarFace` isn't itself a mana concept — it rides
/// [`EventFilter::RollPlanarDie`](crate::EventFilter::RollPlanarDie), whose
/// own home is `event.rs`.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, Deserialize, Expand, Serialize)]
pub enum PlanarFace {
    Blank,
    Chaos,
    Planeswalker,
}

/// Produced-mana spec ([CR#106]): what colors or types a mana-adding effect
/// may produce. Variants accrete — `AnyType`, riders later.
///
/// The `#[macro_ron(embed)]` Specific variant serializes transparently, so the
/// RON stays flat: `AddMana(Literal(1), White)`, not `…Specific(White)`. Tagged
/// variants (`AnyColor`, `OneOf`, `OneOfRuns`, future `AnyType`, riders) read
/// by name; anything that isn't one of them falls through to the embedded
/// `ColorOrColorless`.
///
/// Not `Copy`: `OneOf`/`OneOfRuns` carry a `Vec`. Nothing `Copy` holds a
/// `ManaSpec` (`Action`/`Token` are `Clone`), so the spec stays `Clone`.
#[derive(Debug, Clone, PartialEq, Eq, Hash, SupportsMacros)]
pub enum ManaSpec {
    AnyColor,
    /// One mana of a color the controller chooses from a fixed set on
    /// resolution ("{W} or {U}", [CR#106.1b]) — a single mana ability, not a
    /// [CR#700.2] modal choice. Members keep their printed order.
    OneOf(Arc<[ColorOrColorless]>),
    /// A choice among multi-symbol RUNS the controller makes on resolution
    /// ("{W}{W}, {W}{U}, or {U}{U}", the filterland cycle, [CR#106.1b]): one
    /// mana ability offering several fixed sequences, the chosen run's whole
    /// sequence of mana produced at once. Each run is a non-empty sequence of
    /// colored/colorless symbols; runs keep their printed order. The
    /// general multi-mana form of `OneOf` (whose "runs" are each one mana);
    /// still a single mana ability, not a [CR#700.2] modal choice.
    OneOfRuns(Arc<[Vec<ColorOrColorless>]>),
    /// One mana of any of a referenced object's colors ([CR#105.2]) — the
    /// producer picks AMONG that object's colors, not a fixed authored set
    /// (distinct from [`OneOf`](ManaSpec::OneOf)): Chrome Mox's imprint,
    /// "add one mana of any of the exiled card's colors". Mirrors the Idris
    /// `ProducedMana.AmongColorsOf`.
    AmongColorsOf(Reference),
    /// The type the ambient mana-producing event actually produced
    /// ([CR#106.1b,106.12a]) — Dictate of Karametra/Vorinclex's "add one
    /// mana of any type that land produced". Idris gates this behind
    /// `producesMana (eventCaps b) = True` (sound only inside a
    /// [`TapForMana`](crate::EventFilter::TapForMana)-triggered body, the
    /// `EventObject`/`EventAmount` pattern) via an ERASED auto-proof; Rust
    /// has no dependent types to carry that obligation, so this mirrors as
    /// a plain nullary variant — an authoring mistake that reaches for it
    /// outside a `TapForMana` body simply fails `idris-check` (the
    /// soundness gate), and the engine fizzles gracefully if ever evaluated
    /// without a live `TapForMana` context (see
    /// `deckmaste_engine::resolve`). Mirrors the Idris
    /// `ProducedMana.ProducedByEvent`.
    ProducedByEvent,
    #[macro_ron(embed)]
    Specific(ColorOrColorless),
}

impl From<ColorOrColorless> for ManaSpec {
    fn from(color_or_colorless: ColorOrColorless) -> Self {
        Self::Specific(color_or_colorless)
    }
}

impl From<Color> for ManaSpec {
    fn from(color: Color) -> Self {
        Self::Specific(color.into())
    }
}

/// The component symbols hybrid/phyrexian symbols are built from: a generic
/// amount, one of the five colors, or colorless ({C}, which is not a color).
///
/// The `#[macro_ron(embed)]` Specific variant serializes transparently, so the
/// RON stays flat: `White`, not `Specific(White)`; a bare colored/colorless
/// ident falls through to the embedded [`ColorOrColorless`].
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, SupportsMacros)]
pub enum SimpleManaSymbol {
    Generic(crate::Uint),
    #[macro_ron(embed)]
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

    /// The amount of mana this component represents ([CR#202.3]): a generic
    /// symbol its number, a specific symbol 1.
    #[must_use]
    pub fn mana_value(&self) -> crate::Uint {
        match self {
            Self::Generic(n) => *n,
            Self::Specific(_) => 1,
        }
    }
}

impl From<Color> for SimpleManaSymbol {
    fn from(color: Color) -> Self {
        Self::Specific(color.into())
    }
}

impl From<ColorOrColorless> for SimpleManaSymbol {
    fn from(color: ColorOrColorless) -> Self {
        Self::Specific(color)
    }
}

impl From<crate::Uint> for SimpleManaSymbol {
    fn from(amount: crate::Uint) -> Self {
        Self::Generic(amount)
    }
}

/// A printed mana symbol. Colored/colorless/generic/{X}/{S} plus the hybrid
/// families, built compositionally: `Hybrid(SimpleManaSymbol, Color)` covers
/// `{W/U}` (`Hybrid(White, Blue)`), the monocolored `{2/W}`
/// (`Hybrid(Generic(2), White)`) and colorless `{C/W}` (`Hybrid(Colorless,
/// White)`) families in one shape ([CR#107.4,107.4e]); `Phyrexian(Color,
/// Option<Color>)` covers the five `{W/P}` (`Phyrexian(White, None)`) and the
/// hybrid `{G/U/P}` (`Phyrexian(Green, Some(Blue))`) symbols ([CR#107.4f]).
/// Deliberately a hair more permissive than the printed [CR#107.4] set —
/// unprinted forms like `{5/W}` (`Hybrid(Generic(5), White)`) are
/// representable on purpose (variant/design headroom); well-formedness beyond
/// the type is a load-time/proof concern, not a structural one. Printed
/// symbols and PRODUCED mana ([`ManaSpec`]) stay separate types.
///
/// The `#[macro_ron(embed)]` Simple variant serializes transparently, so the
/// RON stays flat: `Generic(2)`, not `Simple(Generic(2))`; a bare
/// `SimpleManaSymbol` spelling falls through to the embedded type.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, SupportsMacros)]
pub enum ManaSymbol {
    Variable,
    Snow,
    Hybrid(SimpleManaSymbol, Color), // Slightly more permissive than [CR#107.4].
    Phyrexian(Color, Option<Color>),
    #[macro_ron(embed)]
    Simple(SimpleManaSymbol),
}

impl From<Color> for ManaSymbol {
    fn from(color: Color) -> Self {
        Self::Simple(color.into())
    }
}

impl From<ColorOrColorless> for ManaSymbol {
    fn from(color: ColorOrColorless) -> Self {
        Self::Simple(color.into())
    }
}

impl From<crate::Uint> for ManaSymbol {
    fn from(amount: crate::Uint) -> Self {
        Self::Simple(amount.into())
    }
}

/// A predicate over mana symbols — both the SPEND-time payment-freedom slot of
/// `StaticEffect::SpendAsThough` ([CR#609.4b]) and the devotion/pip matcher
/// under `Countable::ManaSymbols` ([CR#700.5]). Grown from the spend-time
/// subset (`AnyColor`/`AnyType`) into the Idris `SymbolPred` matcher algebra;
/// Rust stays a permissive superset (the spend-time spellings have no Idris
/// twin).
#[derive(Debug, Clone, PartialEq, Eq, Hash, Deserialize, Expand, Serialize)]
pub enum SymbolPred {
    /// "as though it were mana of any color" — matches any COLORED pip
    /// (colorless pips excluded, [CR#107.4c]).
    AnyColor,
    /// "as though it were mana of any type" — matches any pip, colorless
    /// included ([CR#106.1b]).
    AnyType,
    /// A pip that counts as `[color]` ([CR#700.5]): a colored pip of that
    /// color, a hybrid pip with that color as one half, or a Phyrexian pip of
    /// that color.
    CountsAs(Color),
    /// A generic pip (`{2}`, `{X}`), i.e. one with no color.
    IsGeneric,
    /// All of the given filters (non-empty by convention; empty fizzles).
    And(Arc<[SymbolPred]>),
    /// Any of the given filters (non-empty by convention; empty = "no colors").
    Or(Arc<[SymbolPred]>),
    /// The negation of a filter.
    Not(Arc<SymbolPred>),
}

impl SymbolPred {
    /// Whether `sym` matches this predicate ([CR#700.5] devotion counting; the
    /// spend-time slot uses the same matcher). Hybrid counts as each of its
    /// colors; Phyrexian as its color.
    #[must_use]
    pub fn matches(&self, sym: &ManaSymbol) -> bool {
        match self {
            SymbolPred::AnyColor => symbol_colors(sym).next().is_some(),
            SymbolPred::AnyType => !matches!(sym, ManaSymbol::Snow),
            SymbolPred::CountsAs(c) => symbol_colors(sym).any(|k| k == *c),
            SymbolPred::IsGeneric => {
                matches!(
                    sym,
                    ManaSymbol::Simple(SimpleManaSymbol::Generic(_)) | ManaSymbol::Variable
                )
            }
            SymbolPred::And(ps) => !ps.is_empty() && ps.iter().all(|p| p.matches(sym)),
            SymbolPred::Or(ps) => ps.iter().any(|p| p.matches(sym)),
            SymbolPred::Not(p) => !p.matches(sym),
        }
    }
}

/// The colors a mana symbol counts as ([CR#107.4a,107.4e,107.4f]): a colored
/// pip is its color, a hybrid pip is both halves' colors (its
/// `SimpleManaSymbol` half contributes a color only when it is a colored
/// `Specific`), a Phyrexian pip its color(s). Generic/colorless/{X}/{S}
/// contribute none.
fn symbol_colors(sym: &ManaSymbol) -> impl Iterator<Item = Color> + '_ {
    let mut out: Vec<Color> = Vec::new();
    match sym {
        ManaSymbol::Simple(s) => out.extend(s.color()),
        ManaSymbol::Hybrid(s, c) => {
            out.extend(s.color());
            out.push(*c);
        }
        ManaSymbol::Phyrexian(c, extra) => {
            out.push(*c);
            out.extend(*extra);
        }
        ManaSymbol::Variable | ManaSymbol::Snow => {}
    }
    out.into_iter()
}

/// A rider a producing effect attaches to the mana itself ([CR#106.6] —
/// riders live on the UNIT: under production doublers each mana gets its
/// own delayed trigger, [CR#106.6a]). None of these change the mana's type.
///
/// Both serde impls are generated by `#[derive(SupportsMacros)]`: `Expanded`
/// writes the invocation back.
#[derive(Debug, Clone, PartialEq, Eq, Hash, SupportsMacros)]
pub enum ManaRider {
    /// Spend restriction — "Spend this mana only to cast creature spells"
    /// ([CR#106.6]); the filter judges the spell/ability being paid.
    SpendOnly(crate::Predicate),
    /// An effect granted to the spell or ability the mana is spent on
    /// ("If that mana is spent on a creature spell, it gains riot").
    GrantOnSpend(Arc<crate::OneShotEffect>),
    /// A delayed trigger ([CR#603.7a]) firing when the mana is spent
    /// ("When that mana is spent to cast …, copy that spell").
    TriggerOnSpend(Arc<crate::OneShotEffect>),
    /// Persistence override ([CR#106.4] emptying does not claim it until
    /// the marker — firebending's "you don't lose this mana",
    /// [CR#702.189a]).
    Persistent(crate::TurnMarker),
    /// Provenance: this mana was produced by a snow source — a snow permanent,
    /// i.e. one with the Snow supertype ([CR#205.4g]) — so the unit may pay the
    /// `{S}` symbol ([CR#107.4h]). Unlike `SpendOnly`, it does NOT restrict
    /// spending; it only ENABLES `{S}`. Set at the production emit site from
    /// the source's derived supertypes, not declared by the producing
    /// ability text.
    Snow,
    /// A remembered `ManaRider` macro invocation. Serialized as the
    /// invocation, not the struct.
    #[macro_ron(expanded)]
    Expanded(Expansion<ManaRider>),
}

/// What a production effect adds: the mana spec, optionally with riders.
/// The `#[macro_ron(embed)]` `Bare` variant keeps existing spellings flat —
/// `AddMana(Literal(1), AnyColor)` — while riders read tagged:
/// `AddMana(Literal(1), WithRiders(mana: Red, riders: [SpendOnly(…)]))`. The
/// embed is the macro-aware replacement for `#[serde(untagged)]`, which buffers
/// through serde's private `Content` and replays past the macro layer — so a
/// macro nested in a rider (`SpendOnly(Not(… Type(Artifact) …))`) would fail to
/// expand; the embed keeps the whole value on the macro-aware path.
#[derive(Debug, Clone, PartialEq, Eq, Hash, SupportsMacros)]
pub enum ManaProduction {
    WithRiders {
        mana: ManaSpec,
        riders: Arc<[ManaRider]>,
    },
    #[macro_ron(embed)]
    Bare(ManaSpec),
}

impl ManaProduction {
    /// The produced-mana spec, riders or not.
    #[must_use]
    pub fn mana(&self) -> &ManaSpec {
        match self {
            ManaProduction::WithRiders { mana, .. } | ManaProduction::Bare(mana) => mana,
        }
    }
}

impl<T: Into<ManaSpec>> From<T> for ManaProduction {
    fn from(spec: T) -> Self {
        ManaProduction::Bare(spec.into())
    }
}

/// A printed mana cost: the symbol list ([CR#202.1a]). Conventions
/// ([CR#118.5..118.6]): an EMPTY list is "no mana cost" — an unpayable
/// base (castable only via an alternative, [CR#118.6a]); the {0} cost is
/// spelled `[Generic(0)]` — payable with nothing, but still a payment.
#[derive(Debug, Clone, Default, PartialEq, Eq, Hash, Deserialize, Expand, Serialize)]
#[serde(transparent)]
pub struct ManaCost(Arc<[ManaSymbol]>);

impl ManaCost {
    #[must_use]
    pub fn is_empty(&self) -> bool {
        self.0.is_empty()
    }

    /// The printed cost's mana value ([CR#202.3]): the total amount of mana,
    /// regardless of color. A hybrid symbol counts its largest component
    /// ([CR#202.3f]); a Phyrexian symbol counts 1 ([CR#202.3g]); {X} counts
    /// 0 — the not-on-the-stack treatment ([CR#202.3e]; while on the stack X
    /// is the announced value, an ENGINE read of the announce slot, not a
    /// property of the printed cost).
    #[must_use]
    pub fn mana_value(&self) -> crate::Uint {
        self.iter()
            .map(|sym| match sym {
                ManaSymbol::Variable => 0,
                ManaSymbol::Snow | ManaSymbol::Phyrexian(..) => 1,
                // Largest component ([CR#202.3f]): the colored half is 1, so a
                // generic-N left half (e.g. {2/W}) dominates, a colored or
                // colorless left half ties at 1.
                ManaSymbol::Hybrid(component, _) => component.mana_value().max(1),
                ManaSymbol::Simple(component) => component.mana_value(),
            })
            .sum()
    }
}

impl From<Arc<[ManaSymbol]>> for ManaCost {
    fn from(symbols: Arc<[ManaSymbol]>) -> Self {
        Self(symbols)
    }
}

impl From<ManaCost> for Arc<[ManaSymbol]> {
    fn from(cost: ManaCost) -> Self {
        cost.0
    }
}

impl std::ops::Deref for ManaCost {
    type Target = [ManaSymbol];

    fn deref(&self) -> &Self::Target {
        &self.0
    }
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

/// Parses the body of a `{...}` mana symbol, braces already stripped. Accepts
/// the compositional hybrid/Phyrexian forms — including deliberately unprinted
/// ones like `{5/W}` — matching the permissive [`ManaSymbol`] shape.
fn parse_symbol_body(body: &str) -> Option<ManaSymbol> {
    Some(match *body.split('/').collect::<Arc<[_]>>() {
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
        let mut symbols: Vec<ManaSymbol> = Vec::new();
        let mut rest = mana_cost;
        while !rest.is_empty() {
            let end = rest.find('}').map_or(rest.len(), |close| close + 1);
            let (symbol, tail) = rest.split_at(end);
            symbols.push(symbol.parse()?);
            rest = tail;
        }
        Ok(Self(symbols.into()))
    }
}

#[cfg(test)]
mod tests {
    use Color::*;
    use ColorOrColorless::Colorless;
    use ManaSymbol::*;
    use SimpleManaSymbol::Generic;
    use SimpleManaSymbol::Specific;

    use super::*;

    fn symbol(s: &str) -> Result<ManaSymbol, ParseManaError> {
        s.parse()
    }

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
            let err = symbol(invalid).unwrap_err();
            assert_eq!(err, ParseManaError::new(invalid));
        }
    }

    #[test]
    fn mana_costs() {
        let cost = |s: &str| s.parse::<ManaCost>();

        assert_eq!(
            cost("{1}{G}").unwrap(),
            ManaCost(vec![Simple(Generic(1)), Green.into()].into())
        );
        assert_eq!(
            cost("{X}{S}").unwrap(),
            ManaCost(vec![Variable, Snow].into())
        );
        assert_eq!(
            cost("{2/W}{C/B}").unwrap(),
            ManaCost(
                vec![
                    Hybrid(Generic(2), White),
                    Hybrid(Specific(Colorless), Black),
                ]
                .into()
            )
        );
        assert_eq!(
            cost("{G/U/P}{W/P}").unwrap(),
            ManaCost(vec![Phyrexian(Green, Some(Blue)), Phyrexian(White, None),].into())
        );
        assert_eq!(cost("").unwrap(), ManaCost::default());

        // ManaCost derefs to its symbols.
        assert_eq!(cost("{1}{G}").unwrap().len(), 2);
        assert_eq!(cost("{X}").unwrap().first(), Some(&Variable));
    }

    /// Each symbol family's RON spelling round-trips — the compositional wire
    /// forms (`Hybrid(White, Blue)`, `Phyrexian(Red, None)`, …) read back to
    /// the same value. Includes a deliberately-permissive `{5/W}`.
    #[test]
    fn symbol_ron_round_trips() {
        for sym in [
            Variable,
            Snow,
            Hybrid(Specific(Green.into()), Blue),
            Hybrid(Generic(2), White),
            Hybrid(Generic(5), White),
            Hybrid(Specific(Colorless), Black),
            Phyrexian(Red, None),
            Phyrexian(Green, Some(Blue)),
            Simple(Generic(3)),
            White.into(),
            Simple(Specific(Colorless)),
        ] {
            let written = crate::ron::options().to_string(&sym).unwrap();
            let read: ManaSymbol = crate::ron::options().from_str(&written).unwrap();
            assert_eq!(read, sym, "round-trip failed for {written}");
        }
    }

    /// The [CR#202.3] examples, symbol family by symbol family: plain total,
    /// hybrid largest-component ([CR#202.3f], both examples), Phyrexian = 1
    /// ([CR#202.3g]), X = 0 off the stack ([CR#202.3e]).
    #[test]
    fn mana_values() {
        let value = |s: &str| s.parse::<ManaCost>().unwrap().mana_value();

        assert_eq!(value("{3}{U}{U}"), 5);
        assert_eq!(value("{1}{W/U}{W/U}"), 3);
        assert_eq!(value("{2/B}{2/B}{2/B}"), 6);
        assert_eq!(value("{1}{W/P}{W/P}"), 3);
        assert_eq!(value("{X}{X}{2}{R}"), 3);
        assert_eq!(value("{S}"), 1);
        assert_eq!(value(""), 0);
    }

    #[test]
    fn invalid_mana_costs() {
        for invalid in [
            " {W}", "{W} {U}", "{W}junk", "junk{W}", "{W}{", "{1}}", "{X}{HW}",
        ] {
            let err = invalid.parse::<ManaCost>().unwrap_err();
            assert!(
                err.to_string().contains("unrecognized"),
                "{invalid:?} => {err}"
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
        assert_eq!(
            spec,
            ManaSpec::OneOf(vec![White.into(), Blue.into()].into())
        );
        let write = |m: &ManaSpec| crate::ron::options().to_string(m).unwrap();
        assert_eq!(write(&spec), "OneOf([White,Blue])");
        // Colorless is a valid member too ({C} or {U} appears in the corpus).
        assert_eq!(
            read("OneOf([Colorless, Blue])"),
            ManaSpec::OneOf(vec![Colorless, Blue.into()].into())
        );
    }

    /// The filterland production "{W}{W}, {W}{U}, or {U}{U}" lands: one mana
    /// ability, a run chosen on resolution. Each run reads as a nested list of
    /// flat colors (`[White, White]`, not `[Color(White), …]`).
    #[test]
    fn mana_spec_one_of_runs_round_trips() {
        let read = |s: &str| crate::ron::options().from_str::<ManaSpec>(s).unwrap();
        let spec = read("OneOfRuns([[White, White], [White, Blue], [Blue, Blue]])");
        assert_eq!(
            spec,
            ManaSpec::OneOfRuns(
                vec![
                    vec![White.into(), White.into()],
                    vec![White.into(), Blue.into()],
                    vec![Blue.into(), Blue.into()],
                ]
                .into()
            )
        );
        let write = |m: &ManaSpec| crate::ron::options().to_string(m).unwrap();
        assert_eq!(
            write(&spec),
            "OneOfRuns([[White,White],[White,Blue],[Blue,Blue]])"
        );
        // Colorless is a valid member of a run too.
        assert_eq!(
            read("OneOfRuns([[Colorless, Colorless], [Colorless, Blue]])"),
            ManaSpec::OneOfRuns(
                vec![vec![Colorless, Colorless], vec![Colorless, Blue.into()],].into()
            )
        );
    }

    #[test]
    fn symbol_pred_round_trips_and_matches() {
        use crate::Color;
        let read = |s: &str| crate::ron::options().from_str::<SymbolPred>(s).unwrap();
        let write = |p: &SymbolPred| crate::ron::options().to_string(p).unwrap();

        // Round-trip the new algebra.
        let g = SymbolPred::CountsAs(Color::Green);
        assert_eq!(read("CountsAs(Green)"), g);
        assert_eq!(read(&write(&g)), g);
        let wb = SymbolPred::Or(
            vec![
                SymbolPred::CountsAs(Color::White),
                SymbolPred::CountsAs(Color::Black),
            ]
            .into(),
        );
        assert_eq!(read(&write(&wb)), wb);
        assert_eq!(read("AnyColor"), SymbolPred::AnyColor); // spend-time spelling preserved

        // matches: hybrid {G/W} counts as each of its colors; Phyrexian {G/P} as its
        // color.
        let hybrid_gw = ManaSymbol::Hybrid(
            SimpleManaSymbol::Specific(Color::Green.into()),
            Color::White,
        );
        assert!(SymbolPred::CountsAs(Color::Green).matches(&hybrid_gw));
        assert!(SymbolPred::CountsAs(Color::White).matches(&hybrid_gw));
        assert!(!SymbolPred::CountsAs(Color::Red).matches(&hybrid_gw));
        let phy_g = ManaSymbol::Phyrexian(Color::Green, None);
        assert!(SymbolPred::CountsAs(Color::Green).matches(&phy_g));
        let generic2 = ManaSymbol::Simple(SimpleManaSymbol::Generic(2));
        assert!(SymbolPred::IsGeneric.matches(&generic2));
        assert!(!SymbolPred::CountsAs(Color::Green).matches(&generic2));
        assert!(SymbolPred::AnyColor.matches(&ManaSymbol::from(Color::Green)));
        assert!(!SymbolPred::AnyColor.matches(&generic2));
        assert!(wb.matches(&hybrid_gw)); // Or: {G/W} is white
        assert!(
            SymbolPred::Not(Arc::new(SymbolPred::IsGeneric))
                .matches(&ManaSymbol::from(Color::Green))
        );
    }

    /// A `ManaProduction` round-trips through the `#[macro_ron(embed)]` `Bare`
    /// variant: a bare spec reads/writes flat (name-erased), while tagged
    /// `WithRiders` keeps its tag.
    #[test]
    fn mana_production_embed_round_trips() {
        let read = |s: &str| crate::ron::options().from_str::<ManaProduction>(s).unwrap();
        let write = |m: &ManaProduction| crate::ron::options().to_string(m).unwrap();
        assert_eq!(read("AnyColor"), ManaProduction::Bare(ManaSpec::AnyColor));
        assert_eq!(
            read("White"),
            ManaProduction::Bare(ManaSpec::Specific(White.into()))
        );
        assert_eq!(write(&ManaProduction::Bare(ManaSpec::AnyColor)), "AnyColor");
        assert_eq!(
            write(&ManaProduction::Bare(ManaSpec::Specific(Colorless))),
            "Colorless"
        );
        let wr = read("WithRiders(mana: Colorless, riders: [Snow])");
        assert_eq!(
            wr,
            ManaProduction::WithRiders {
                mana: ManaSpec::Specific(Colorless),
                riders: vec![ManaRider::Snow].into(),
            }
        );
        assert_eq!(write(&wr), "WithRiders(mana:Colorless,riders:[Snow])");
    }

    /// Regression: a MACRO nested inside a `WithRiders` rider expands. The old
    /// `#[serde(untagged)]` `Bare` variant buffered the whole `ManaProduction`
    /// through serde's private `Content` and replayed it past the macro layer,
    /// so `SpendOnly(<macro>)` failed with "data did not match any variant of
    /// untagged enum `ManaProduction`"; the `#[macro_ron(embed)]` variant keeps
    /// the value on the macro-aware path.
    #[test]
    fn macro_nested_in_rider_expands_through_embed() {
        let def: macro_ron::MacroDef = crate::ron::raw_options()
            .from_str(r#"(name: "CreaturePred", kinds: [Predicate], body: Kind(Spell))"#)
            .expect("macro def parses");
        let mut macros =
            macro_ron::MacroSet::new(crate::ron::kinds()).with_options(crate::ron::raw_options());
        macros.insert(&def).expect("macro inserts");
        let prod: ManaProduction = macros
            .read_str("WithRiders(mana: Colorless, riders: [SpendOnly(CreaturePred)])")
            .expect("rider macro expands through the embed");
        let ManaProduction::WithRiders { riders, mana } = prod else {
            panic!("expected WithRiders");
        };
        assert_eq!(mana, ManaSpec::Specific(Colorless));
        assert!(matches!(riders.as_ref(), [ManaRider::SpendOnly(_)]));
    }
}
