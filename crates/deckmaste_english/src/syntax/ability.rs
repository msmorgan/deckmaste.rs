use super::clause::DependentClause;
use super::clause::IndependentClause;
use super::clause::Predicate;
use super::phrase::NounPhrase;
use super::phrase::NumberLiteral;
use super::phrase::OracleSymbol;
use super::phrase::Phrase;
use super::phrase::PowerToughness;
use super::phrase::Preposition;
use super::phrase::Quantity;
use super::phrase::RecoveredText;
use crate::catalog::CatalogAtom;

#[derive(Debug, Clone, Default, PartialEq, Eq)]
pub struct OracleText {
    pub abilities: Vec<Ability>,
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct Ability {
    /// A Scryfall ability word ([`CatalogKind::AbilityWord`]) peeled before the
    /// ability frame and reproduced as `<word> — `. An ability word is a
    /// rules-relevant grouping label, so it is a licensed structural header,
    /// not lexical opacity.
    ///
    /// [`CatalogKind::AbilityWord`]: crate::CatalogKind::AbilityWord
    pub ability_word: Option<CatalogAtom>,
    /// A Scryfall flavor word ([`CatalogKind::FlavorWord`]) peeled before the
    /// ability frame and reproduced as `<label> — `. Unlike an ability word a
    /// flavor word carries no rules meaning: it is licensed lexical opacity
    /// (counted as [`LexicalOpacityKind::FlavorHeader`]), the sibling of a
    /// paragraph's or cost's [`Paragraph::flavor_header`], carried one level up
    /// because a flavor word can stand ahead of a trigger or cost frame that a
    /// paragraph header cannot reach. At most one of `ability_word` and
    /// `flavor_header` is set; the two never co-occur on the supported corpus.
    ///
    /// [`CatalogKind::FlavorWord`]: crate::CatalogKind::FlavorWord
    /// [`LexicalOpacityKind::FlavorHeader`]: super::LexicalOpacityKind::FlavorHeader
    pub flavor_header: Option<FlavorHeader>,
    pub kind: AbilityKind,
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub enum AbilityKind {
    Activated(ActivatedAbility),
    ClassLevel(ClassLevelAbility),
    Chapter(ChapterAbility),
    RollRow(RollRowAbility),
    LevelBand(LevelBandAbility),
    StationThreshold(StationThresholdAbility),
    Triggered(TriggeredAbility),
    Loyalty(LoyaltyAbility),
    Modal(ModalAbility),
    Keyword(KeywordAbilityList),
    Paragraph(Paragraph),
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct ClassLevelAbility {
    pub cost: Cost,
    pub level: super::phrase::NumberLiteral,
}

/// A saga chapter ability: one effect that resolves as each listed chapter
/// number is reached. A single chapter (`I — …`) carries a one-element list;
/// a combined header (`I, II — …`) carries the whole comma-separated list. The
/// chapter numbers are structural [`NumberLiteral`]s, so nothing recovers at
/// the header, and the body is a single [`Paragraph`] rendered inline after
/// `HEADER — ` — never as a bulleted mode. The inline layout is carried by the
/// node type itself, which is what distinguishes a chapter from a modal choice
/// ability whose modes render as `• `-prefixed bullet lines.
///
/// [`NumberLiteral`]: super::phrase::NumberLiteral
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct ChapterAbility {
    pub chapters: Vec<super::phrase::NumberLiteral>,
    pub body: Paragraph,
}

/// A die-roll result-table row: one keyed outcome of a `Roll a dN` table,
/// printed as its own paragraph in `RANGE | body` layout (e.g. Treasure
/// Chest's `2—9 | Create five Treasure tokens.`). The face-value key is
/// structural — carried by [`RollRange`] — so nothing recovers at the row
/// prefix, and the body is an ordinary [`Paragraph`] (its own flavor header and
/// sentences) rendered inline after ` | `. This mirrors [`ChapterAbility`]: the
/// range is the row's analogue of the saga chapter header, reproduced exactly
/// by the renderer, and the inline layout is carried by the node type itself.
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct RollRowAbility {
    pub range: RollRange,
    pub body: Paragraph,
}

/// The face-value key of a die-roll result row. Every distinction the surface
/// draws is carried here so the row renders as an exact inverse: a single face
/// (`20`), an inclusive low–high span joined by an *unspaced* en dash (`2–9`),
/// an at-least threshold (`15+`), or an at-most threshold (`9 or less`).
///
/// The dash glyph is not carried: the input boundary normalizes every roll-row
/// range separator (an em dash or an ASCII hyphen) to a single en dash
/// (`–`, U+2013), so the renderer always emits that one glyph. See
/// [`normalize_roll_row_dashes`](crate::normalize_roll_row_dashes).
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum RollRange {
    /// A single face value: `20 | …`.
    Single(super::phrase::NumberLiteral),
    /// An inclusive low–high span, printed with an unspaced en dash: `2–9 | …`.
    Inclusive {
        low: super::phrase::NumberLiteral,
        high: super::phrase::NumberLiteral,
    },
    /// A face value and everything above it: `15+ | …`.
    OrMore(super::phrase::NumberLiteral),
    /// A face value and everything below it: `9 or less | …`.
    OrLess(super::phrase::NumberLiteral),
}

/// A leveler card's level band [CR#711.2]: the level symbol that opens a text
/// box striation, together with the power/toughness box and every ability
/// printed inside that striation. The whole striation is one static ability
/// ([CR#711.2a,711.2b]), so the band is one node rather than a header
/// followed by siblings — the striations are the only thing that says which
/// abilities and which P/T box go with which level symbol [CR#711.3]. The
/// level-up ability itself sits outside every band and is always active
/// [CR#711.4], so it stays an ordinary sibling keyword ability.
///
/// The band renders across lines — `LEVEL <range>`, the stat line, then one
/// line per contained ability — the same way a modal ability renders its
/// bulleted modes. Nothing recovers at the header or the stat line: both are
/// carried structurally.
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct LevelBandAbility {
    pub range: LevelRange,
    pub stats: PowerToughness,
    pub abilities: Vec<Ability>,
}

/// The level symbol's counter range. The two shapes the frame prints are the
/// two [CR#711.2] spells out; unlike [`RollRange`] there is no single-value
/// or at-most shape, and the inclusive band's separator is the **ASCII
/// hyphen** the frame actually prints — a level symbol is not a roll-row key,
/// so `normalize_roll_row_dashes` never rewrites it and the renderer must not
/// emit an en dash.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum LevelRange {
    /// `LEVEL N1-N2` — active while N1 ≤ level counters ≤ N2 [CR#711.2a].
    Band {
        low: super::phrase::NumberLiteral,
        high: super::phrase::NumberLiteral,
    },
    /// `LEVEL N3+` — active while level counters ≥ N3 [CR#711.2b].
    AtLeast(super::phrase::NumberLiteral),
}

/// A station card's threshold striation [CR#721.2]: the station symbol that
/// opens a text-box striation, together with the single ability printed
/// inside it — `8+ | Flying, trample`. The symbol is itself a keyword
/// ability [CR#702.184b] and represents a static ability: "as long as this
/// permanent has N or more charge counters on it, it has [abilities]"
/// [CR#721.2a].
///
/// Unlike a leveler band ([`LevelBandAbility`], [CR#711.2]) the striation is
/// **line-local**: the ` | ` separator delimits it, and any line after it
/// that no station symbol precedes is ordinary always-on text [CR#721.4].
/// Unlike a die-roll row ([`RollRowAbility`]) the body is a whole
/// [`Ability`], not a [`Paragraph`] — a striation routinely holds a keyword
/// list, an activated ability, or a triggered ability, none of which a
/// paragraph can carry. The threshold key is a bare number because
/// [CR#721.2] admits exactly one shape, `N+`; there is no range or at-most
/// form to carry.
///
/// A station row is recognized only on a face that carries the `Station`
/// keyword ability [CR#702.184b], which is what keeps a die-roll table's
/// `15+ | …` row from reaching this frame. [CR#721.1] notes a station card
/// only *usually* prints that keyword; one printed without it would lower as
/// an ordinary die-roll row instead.
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct StationThresholdAbility {
    pub threshold: super::phrase::NumberLiteral,
    /// Boxed so the variant stays small, mirroring `QuotedAbility::ability`.
    pub ability: Box<Ability>,
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct ActivatedAbility {
    pub cost: Cost,
    pub effect: Paragraph,
    pub effect_initial_uppercase: bool,
}

/// An activation cost: the comma-separated list of components paid before the
/// colon. Every cost is a list of typed [`CostComponent`]s; the earlier raw
/// `SymbolList(String)` and untyped `Components(Vec<Phrase>)` shapes are gone.
#[derive(Debug, Clone, Default, PartialEq, Eq)]
pub struct Cost {
    pub flavor_header: Option<FlavorHeader>,
    pub components: Vec<CostComponent>,
}

/// One component of an activation cost's comma-separated list. A closed sum
/// over the cost-component *shapes* the supported corpus attests, never over a
/// cost's meaning: a mana/symbol run, a cost expressed as an independent
/// clause, a bare noun phrase, or a verbatim recovery when no shape parses.
/// Which shape a component takes is decided by its surface alone — the clause's
/// verb is open, the shape is not, so this crate records no per-action semantic
/// facts. An escape variant ([`Recovered`](CostComponent::Recovered)) keeps the
/// type from over-closing, mirroring [`KeywordArgument::Recovered`].
#[derive(Debug, Clone, PartialEq, Eq)]
pub enum CostComponent {
    /// A mana or symbol run paid as a cost — `{2}{R}`, `{T}`, `{Q}`, `{E}{E}`.
    /// One or more oracle symbols; a single symbol is a one-element run,
    /// reproduced by concatenation exactly as [`Phrase::SymbolSequence`] is.
    Symbols(Vec<OracleSymbol>),
    /// A cost expressed as an independent clause: an imperative (`Sacrifice a
    /// creature`, `Pay 3 life`, `Discard a card`, `Remove N counters`, `Tap`),
    /// a coordinated pair (`Exile a creature card from your graveyard and
    /// pay its mana cost`), or a transitive sentence. Boxed so the
    /// component stays small.
    Clause(Box<IndependentClause>),
    /// A bare noun-phrase cost — the comma-split continuation of a preceding
    /// clause's object list (`Sacrifice a red creature, a green creature, and a
    /// white creature` splits each trailing conjunct into its own component).
    Noun(Box<NounPhrase>),
    /// An alternative cost payment joined by `or` where paying either
    /// component satisfies the cost — `{T} or {W}`, `Pay 2 life or {2}`.
    /// Tried only after the whole component fails to parse as a clause or
    /// noun phrase, so an `or` inside an ordinary cost clause (`Sacrifice an
    /// artifact or creature`) never splits.
    Alternative(Box<CostComponent>, Box<CostComponent>),
    /// No cost shape parsed these tokens; they recover verbatim at the
    /// activation-cost role — the term-level echo of "misparameterization has
    /// no term."
    Recovered(RecoveredText),
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct TriggeredAbility {
    pub introducer: TriggerWord,
    pub event: TriggerEvent,
    pub intervening_condition: Option<DependentClause>,
    pub effect: Paragraph,
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub enum TriggerEvent {
    Clause(IndependentClause),
    Temporal(NounPhrase),
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum TriggerWord {
    When,
    Whenever,
    At,
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct LoyaltyAbility {
    pub cost: LoyaltyCost,
    pub effect: Paragraph,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub struct LoyaltyCost {
    pub sign: LoyaltyCostSign,
    pub value: LoyaltyCostValue,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum LoyaltyCostSign {
    None,
    Plus,
    Minus,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum LoyaltyCostValue {
    Number(u32),
    X,
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct ModalAbility {
    pub frame: ModalFrame,
    pub header: Paragraph,
    pub header_suffix: ModalHeaderSuffix,
    pub modes: Vec<Mode>,
}

#[allow(
    clippy::large_enum_variant,
    reason = "larger enum shapes are part of the serialized card-ability model"
)]
#[derive(Debug, Clone, PartialEq, Eq)]
pub enum ModalFrame {
    Unframed,
    Preamble {
        body: Paragraph,
        separator: ModalPreambleSeparator,
    },
    Activated(Cost),
    Triggered {
        introducer: TriggerWord,
        event: TriggerEvent,
        intervening_condition: Option<DependentClause>,
    },
    Loyalty(LoyaltyCost),
    /// A saga chapter heading whose effect is a modal choice, e.g. Life of
    /// Toshiro Umezawa's `I, II — Choose one —`. The chapter numbers are
    /// structural (mirroring [`ChapterAbility`]) and render as an exact inverse
    /// before the choice header.
    Chapter(Vec<super::phrase::NumberLiteral>),
    /// A keyword-ability header standing in for a `Choose …` instruction, e.g.
    /// Final Fantasy's `Tiered`, whose bulleted modes each carry a name and an
    /// additional cost ([`Mode::heading`]). The atom renders verbatim; the
    /// header paragraph is normally empty but carries a shared instruction on
    /// the members that print one before the modes.
    Keyword(CatalogAtom),
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum ModalHeaderSuffix {
    None,
    SpacedEmDash,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum ModalPreambleSeparator {
    None,
    Space,
    CommaSpace,
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct Mode {
    /// A tiered mode's `<name> — <cost> — ` heading (e.g. `Cross-Slash — {0}
    /// —`). Absent on ordinary `Choose …` modes, whose body follows the
    /// bullet directly.
    pub heading: Option<ModeHeading>,
    pub body: Paragraph,
}

/// The name and additional cost heading a [`Tiered`](ModalFrame::Keyword) mode.
/// The label is a verbatim opaque run (licensed lexical opacity); the cost is
/// the structural additional cost paid to choose the mode. Both always
/// co-occur, so a single option carries the whole heading.
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct ModeHeading {
    pub label: FlavorHeader,
    pub cost: Cost,
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct KeywordAbilityList {
    pub abilities: Vec<KeywordAbility>,
}

/// A keyword ability: its open-set name (the catalog atom) and the argument the
/// surface attaches to it. The keyword *name* is an open set (new sets keep
/// minting keywords); what is closed is the argument-shape vocabulary
/// [`KeywordArgument`]. This crate is a surface grammar and records no
/// per-keyword semantic facts: which shape a keyword may take is decided by the
/// argument tokens' surface form alone — any keyword may carry any argument
/// that parses as one of the closed shapes, and an argument that parses as none
/// of them becomes [`KeywordArgument::Recovered`]. Whether a given
/// keyword+shape pairing is legal Magic is the engine's concern, not the
/// grammar's.
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct KeywordAbility {
    pub preceding_separator: Option<KeywordListSeparator>,
    pub ability: CatalogAtom,
    pub argument: KeywordArgument,
}

/// A keyword ability's argument, a closed sum over the parameter *shapes* the
/// Comprehensive Rules give keywords — never over the keywords themselves. Each
/// variant is recognized purely from the argument's surface and carries exactly
/// that shape's payload; surface variants (a symbol cost versus an em-dash
/// sentence cost, a single quality versus a coordinated one) live inside a
/// shape's payload, never as sibling shapes. The per-shape citations name the
/// exemplar CR rules for the shape, not facts about any keyword.
#[derive(Debug, Clone, PartialEq, Eq)]
pub enum KeywordArgument {
    /// No argument — flying, first strike, deathtouch.
    Absent,
    /// A count. The shape's exemplar rules give the numeric argument as `N`
    /// [CR#702.86a,702.164a].
    Counted(Quantity),
    /// A cost, symbol-sequence or em-dash sentence (the [`KeywordCost`]
    /// surfaces). The shape's exemplar rules give the argument as `[cost]`
    /// [CR#702.21a,702.29a].
    Costed(KeywordCost),
    /// A count and a symbol cost joined by an unspaced em dash. The shape's
    /// exemplar rule gives the argument as `N—[cost]` [CR#702.62a].
    CountedCost {
        count: NumberLiteral,
        symbols: Vec<OracleSymbol>,
    },
    /// A quality filter introduced by a preposition, coordinated where the
    /// surface repeats it. The shape's exemplar rules give the argument as
    /// `from [quality]` and `for [text]` [CR#702.16a,702.11d,702.41a], the
    /// coordinated form as `from [A] and from [B]` [CR#702.16g,702.11f].
    Predicated(PredicatedArgument),
    /// A symbol cost paired with power/toughness by a spaced em dash. The
    /// shape's exemplar rules give the argument as `[cost] — [P]/[T]`
    /// [CR#702.160a,718.1]. A shape added beyond the CR's six observed
    /// keyword-parameter shapes for the one argument surface that needs it
    /// (see the round's report).
    Statted {
        symbols: Vec<OracleSymbol>,
        stats: PowerToughness,
    },
    /// A verbatim pairing label after an em dash. The shape's exemplar rule
    /// gives the argument as `[text]` [CR#702.124i]; the label is carried
    /// opaquely, like a card name.
    Named {
        separator: KeywordArgumentSeparator,
        label: String,
    },
    /// No closed shape parsed the argument tokens; they recover verbatim at the
    /// keyword-argument role — the term-level echo of the model's
    /// "misparameterization has no term."
    Recovered {
        separator: KeywordArgumentSeparator,
        text: RecoveredText,
    },
}

/// The two surfaces a [`KeywordArgument::Costed`] cost takes.
#[derive(Debug, Clone, PartialEq, Eq)]
pub enum KeywordCost {
    /// A mana/symbol cost written after a space — `ward {2}`, `equip {3}`. The
    /// run is carried as its structured oracle symbols, reproduced by
    /// concatenation.
    Symbols(Vec<OracleSymbol>),
    /// A non-mana cost written as an em-dash sentence — `cumulative upkeep—Put
    /// a -1/-1 counter on this creature.` The dash spacing is carried
    /// structurally so rendering never inspects the surface.
    Sentence {
        separator: KeywordArgumentSeparator,
        ability: Box<Ability>,
    },
}

/// A [`KeywordArgument::Predicated`] quality filter: one quality, or several
/// coordinated qualities that the surface joins with ` and ` and the CR treats
/// as separate abilities [CR#702.16g,702.11f].
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct PredicatedArgument {
    pub qualities: Vec<PredicatedQuality>,
}

/// One quality of a [`PredicatedArgument`].
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct PredicatedQuality {
    /// The preposition introducing this quality on the surface (`from` for
    /// protection/hexproof, `for` for affinity), or `None` when the keyword
    /// atom itself carries it (`Hexproof from`).
    pub preposition: Option<Preposition>,
    pub quality: Phrase,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum KeywordListSeparator {
    Comma,
    Semicolon,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum KeywordArgumentSeparator {
    Space,
    EmDash,
    SpacedEmDash,
}

#[derive(Debug, Clone, Default, PartialEq, Eq)]
pub struct Paragraph {
    /// Semantically inert flavor text carried before an em dash in header
    /// position (ability start or saga chapter body). Licensed lexical opacity:
    /// the surface is preserved verbatim and reproduced with its em-dash
    /// separator, never parsed structurally.
    pub flavor_header: Option<FlavorHeader>,
    pub sentences: Vec<Sentence>,
}

/// A flavor junk-before-dash header: an arbitrary token run reproduced
/// verbatim.
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct FlavorHeader {
    text: String,
    source_tokens: usize,
}

impl FlavorHeader {
    #[must_use]
    pub fn new(text: impl Into<String>, source_tokens: usize) -> Self {
        Self {
            text: text.into(),
            source_tokens,
        }
    }

    #[must_use]
    pub fn text(&self) -> &str {
        &self.text
    }

    #[must_use]
    pub const fn source_tokens(&self) -> usize {
        self.source_tokens
    }
}

/// A single sentence of an ability's body.
///
/// The terminal period is **not** stored: it is derivable from the sentence's
/// structure. Oracle text ends every sentence with a period except when its
/// final rendered constituent is a closed quoted ability — the period then
/// lives inside the closing quote — or when it is a modal `Choose …` header
/// instruction, which a separator or a bulleted mode list follows. The renderer
/// re-derives the period from the AST tail; the parser strips a trailing period
/// token without recording it.
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct Sentence {
    pub initial_uppercase: bool,
    pub body: SentenceBody,
}

#[allow(
    clippy::large_enum_variant,
    reason = "sentence bodies can legitimately hold a large independent clause payload"
)]
#[derive(Debug, Clone, PartialEq, Eq)]
pub enum SentenceBody {
    Independent(IndependentClause),
    /// A modal ability's `Choose one` header instruction. Structurally distinct
    /// from a bare imperative so the choice quantity, an `at random` adverbial,
    /// and an optional leading trigger clause are all carried and rendered as
    /// an exact inverse. Only produced inside a modal header.
    Choice(ChoiceInstruction),
    /// A verbless power/toughness sentence: the whole sentence is a single
    /// `[P]/[T]` value (`3/2.`). It surfaces as a tiered mode's body, where the
    /// mode's effect is the base power and toughness it sets; the value carries
    /// the same statistic the copular and object positions already model, and
    /// the renderer reproduces it with a derived terminal period.
    PowerToughness(PowerToughness),
    /// A trigger clause heading a sentence that does not open its ability's
    /// effect paragraph — after an activation-cost colon, a loyalty header, an
    /// ability-word or Saga-chapter header, or a preceding sentence. An
    /// ability-initial trigger is absorbed by [`AbilityKind::Triggered`] before
    /// a paragraph is ever parsed, so this variant is reachable only from the
    /// non-initial position. Boxed for the same reason
    /// [`ChoiceInstruction::trigger_prefix`] is: the event clause is large and
    /// the grammar-lowering enum wrapping [`Sentence`] must not grow.
    Triggered(Box<TriggeredSentence>),
    Recovered(RecoveredText),
}

/// A non-initial trigger sentence. Mirrors the fields of [`ChoiceTrigger`] —
/// and of [`ModalFrame::Triggered`] — and renders through the same trigger
/// renderer, plus the effect clause the trigger governs. The effect is a single
/// [`IndependentClause`], not a [`Paragraph`]: a sentence-level trigger governs
/// exactly the remainder of its own sentence.
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct TriggeredSentence {
    pub introducer: TriggerWord,
    pub event: TriggerEvent,
    pub intervening_condition: Option<DependentClause>,
    pub effect: IndependentClause,
}

/// The `Choose one` instruction that heads a modal ability. Every surface
/// distinction the corpus draws is carried structurally so the renderer is an
/// exact inverse, never a guess from spelling:
///
/// - the **choice quantity** (`one`, `two`, `three`, `one or both`, `one or
///   more`, `up to N`, `up to that many`, …) is the [`Predicate`] object of the
///   `choose` imperative, reusing the existing quantity grammar and renderer;
/// - an **`at random`** adverbial is a boolean flag appended after the object;
/// - an optional **trigger prefix** (`When you do, …`) is an ordinary trigger
///   clause parsed by the chart. It is present only when the outer
///   [`ModalFrame`] did not already absorb the ability's trigger — a reflexive
///   second trigger such as `When you do, …` that heads a non-initial header
///   sentence. An ability-initial trigger, coordinated (`Ashcoat enters or
///   attacks`) or not, is instead absorbed by [`ModalFrame::Triggered`].
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct ChoiceInstruction {
    /// Boxed so the choice instruction stays no larger than a bare imperative
    /// header sentence: the trigger clause carries a full event clause, and the
    /// unboxed grammar-lowering enum that wraps [`Sentence`] must not grow.
    pub trigger_prefix: Option<Box<ChoiceTrigger>>,
    pub imperative: Predicate,
    pub at_random: bool,
}

/// A modal choice instruction's leading trigger clause. Mirrors the fields of
/// [`ModalFrame::Triggered`] and renders through the same trigger renderer, so
/// `When you do,` / `Whenever Ashcoat enters or attacks,` reproduce exactly.
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct ChoiceTrigger {
    pub introducer: TriggerWord,
    pub event: TriggerEvent,
    pub intervening_condition: Option<DependentClause>,
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct QuotedAbility {
    pub ability: Box<Ability>,
    pub initial_uppercase: bool,
    pub closed: bool,
    /// Whether the interior's final sentence keeps its terminal period *inside*
    /// the closing quote (`"…."`) or not (`"…"`). A quoted ability that closes
    /// its enclosing sentence absorbs that sentence's period into the quote, so
    /// the surface prints `gains "…."`; the same ability in a non-final slot —
    /// a coordinated conjunct (`has "…" and "…."`) or before a trailing adjunct
    /// (`gains "…" until end of turn.`) — prints without it. The distinction is
    /// not recoverable from the interior alone (the parser strips the terminal
    /// period either way), so it is carried here and the renderer withholds the
    /// interior's final period when this is `false`.
    pub terminal_period: bool,
}
