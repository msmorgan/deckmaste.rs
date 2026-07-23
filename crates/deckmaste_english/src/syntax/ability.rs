use super::clause::DependentClause;
use super::clause::IndependentClause;
use super::clause::Predicate;
use super::phrase::NounPhrase;
use super::phrase::Phrase;
use super::phrase::RecoveredText;
use crate::catalog::CatalogAtom;

#[derive(Debug, Clone, Default, PartialEq, Eq)]
pub struct OracleText {
    pub abilities: Vec<Ability>,
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct Ability {
    pub ability_word: Option<CatalogAtom>,
    pub kind: AbilityKind,
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub enum AbilityKind {
    Activated(ActivatedAbility),
    ClassLevel(ClassLevelAbility),
    Chapter(ChapterAbility),
    RollRow(RollRowAbility),
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

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct ActivatedAbility {
    pub cost: Cost,
    pub effect: Paragraph,
    pub effect_initial_uppercase: bool,
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub enum Cost {
    Components(Vec<Phrase>),
    SymbolList(String),
}

impl Default for Cost {
    fn default() -> Self {
        Self::Components(Vec::new())
    }
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
    pub body: Paragraph,
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct KeywordAbilityList {
    pub abilities: Vec<KeywordAbility>,
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct KeywordAbility {
    pub preceding_separator: Option<KeywordListSeparator>,
    pub ability: CatalogAtom,
    pub argument_separator: Option<KeywordArgumentSeparator>,
    pub argument: Option<Phrase>,
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
    Recovered(RecoveredText),
}

/// The `Choose one` instruction that heads a modal ability. Every surface
/// distinction the corpus draws is carried structurally so the renderer is an
/// exact inverse, never a guess from spelling:
///
/// - the **choice quantity** (`one`, `two`, `three`, `one or both`, `one or
///   more`, `up to N`, `up to that many`, …) is the [`Predicate`] object of the
///   `choose` imperative, reusing the existing quantity grammar and renderer;
/// - an **`at random`** adverbial is a boolean flag appended after the object;
/// - an optional **trigger prefix** (`When you do, …`, `Whenever ~ enters or
///   attacks, …`) is an ordinary trigger clause parsed by the chart. It is
///   present only when the outer [`ModalFrame`] did not already absorb the
///   ability's trigger (a coordinated event the frame's simple-clause parse
///   rejects, or a reflexive second trigger such as `When you do, …`).
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
/// `When you do,` / `Whenever ~ enters or attacks,` reproduce exactly.
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
}
