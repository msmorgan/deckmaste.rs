use std::str::FromStr;
use std::sync::Arc;

use serde::Deserialize;
use serde::Serialize;

use crate::Color;
use crate::Reference;
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
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, Deserialize, Serialize)]
pub enum PlanarFace {
    Blank,
    Chaos,
    Planeswalker,
}

/// Produced-mana spec ([CR#106]): what colors or types a mana-adding effect
/// may produce. Variants accrete — `AnyType`, riders later.
/// Core RON keeps the `Specific` wrapper explicit; compact mana spellings are
/// resolved by the semantics layer before lowering.
/// Not `Copy`: `OneOf`/`OneOfRuns` carry a `Vec`. Nothing `Copy` holds a
/// `ManaSpec` (`Action`/`Token` are `Clone`), so the spec stays `Clone`.
#[derive(Debug, Clone, PartialEq, Eq, Hash, serde::Deserialize, serde::Serialize)]
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
    /// producer picks AMONG that object's colors, not a fixed semantic set
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
    /// a plain nullary variant — a semantic-input error that reaches for it
    /// outside a `TapForMana` body simply fails `idris-check` (the
    /// soundness gate), and the engine fizzles gracefully if ever evaluated
    /// without a live `TapForMana` context (see
    /// `deckmaste_engine::resolve`). Mirrors the Idris
    /// `ProducedMana.ProducedByEvent`.
    ProducedByEvent,
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
/// Core RON keeps the `Specific` wrapper explicit.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, serde::Deserialize, serde::Serialize)]
pub enum SimpleManaSymbol {
    Generic(crate::Uint),
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
/// Core RON keeps the `Simple` wrapper explicit.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, serde::Deserialize, serde::Serialize)]
pub enum ManaSymbol {
    Variable,
    Snow,
    Hybrid(SimpleManaSymbol, Color), // Slightly more permissive than [CR#107.4].
    Phyrexian(Color, Option<Color>),
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
/// `StaticSpec::SpendAsThough` ([CR#609.4b]) and the devotion/pip matcher
/// under `Countable::ManaSymbols` ([CR#700.5]). Grown from the spend-time
/// subset (`AnyColor`/`AnyType`) into the Idris `SymbolPred` matcher algebra;
/// Rust stays a permissive superset (the spend-time spellings have no Idris
/// twin).
#[derive(Debug, Clone, PartialEq, Eq, Hash, Deserialize, Serialize)]
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
#[derive(Debug, Clone, PartialEq, Eq, Hash, serde::Deserialize, serde::Serialize)]
pub enum ManaRider {
    /// Spend restriction — "Spend this mana only to cast creature spells"
    /// ([CR#106.6]); the filter judges the spell/ability being paid.
    SpendOnly(crate::Predicate),
    /// An effect granted to the spell or ability the mana is spent on
    /// ("If that mana is spent on a creature spell, it gains riot").
    GrantOnSpend(Arc<crate::Instruction>),
    /// A delayed trigger ([CR#603.7a]) firing when the mana is spent
    /// ("When that mana is spent to cast …, copy that spell").
    TriggerOnSpend(Arc<crate::Instruction>),
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
}

/// What a production effect adds: the mana spec, optionally with riders.
/// Core RON distinguishes `Bare(spec)` from `WithRiders { .. }` explicitly.
#[derive(Debug, Clone, PartialEq, Eq, Hash, serde::Deserialize, serde::Serialize)]
pub enum ManaProduction {
    WithRiders {
        mana: ManaSpec,
        riders: Arc<[ManaRider]>,
    },
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
#[derive(Debug, Clone, Default, PartialEq, Eq, Hash, Deserialize, Serialize)]
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
#[derive(Debug, Clone, PartialEq, Eq, thiserror::Error)]
#[error("unrecognized mana symbol: {symbol:?}")]
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
