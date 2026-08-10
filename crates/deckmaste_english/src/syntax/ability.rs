use super::clause::Clause;
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
use crate::features::Conjunction;

#[derive(Debug, Clone, Default, PartialEq, Eq, serde::Serialize)]
pub struct OracleText {
    pub abilities: Vec<Ability>,
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct Ability {
    repr: AbilityRepr,
}

#[derive(Debug, Clone, PartialEq, Eq)]
struct AbilityRepr {
    /// A Scryfall ability word ([`CatalogKind::AbilityWord`]) peeled before the
    /// ability frame and reproduced as `<word> — `. An ability word is a
    /// rules-relevant grouping label, so it is a licensed structural header,
    /// not lexical opacity.
    ///
    /// [`CatalogKind::AbilityWord`]: crate::CatalogKind::AbilityWord
    word: Option<CatalogAtom>,
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
    flavor: Option<FlavorHeader>,
    kind: AbilityKind,
}

impl Ability {
    pub(crate) const fn from_parts(
        _owner: &crate::constructions::ability::AbilityOwner,
        ability_word: Option<CatalogAtom>,
        flavor_header: Option<FlavorHeader>,
        kind: AbilityKind,
    ) -> Self {
        Self {
            repr: AbilityRepr {
                word: ability_word,
                flavor: flavor_header,
                kind,
            },
        }
    }

    /// Returns the ability-word header, if present.
    #[must_use]
    pub const fn ability_word(&self) -> Option<&CatalogAtom> {
        self.repr.word.as_ref()
    }

    /// Returns the flavor-word header, if present.
    #[must_use]
    pub const fn flavor_header(&self) -> Option<&FlavorHeader> {
        self.repr.flavor.as_ref()
    }

    /// Returns the ability's validated frame.
    #[must_use]
    pub const fn kind(&self) -> &AbilityKind {
        &self.repr.kind
    }
}

impl serde::Serialize for Ability {
    fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error>
    where
        S: serde::Serializer,
    {
        use serde::ser::SerializeStruct;

        let mut state = serializer.serialize_struct("Ability", 3)?;
        state.serialize_field("ability_word", &self.repr.word)?;
        state.serialize_field("flavor_header", &self.repr.flavor)?;
        state.serialize_field("kind", &self.repr.kind)?;
        state.end()
    }
}

#[derive(Debug, Clone, PartialEq, Eq, serde::Serialize)]
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

#[derive(Debug, Clone, PartialEq, Eq, serde::Serialize)]
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
#[derive(Debug, Clone, PartialEq, Eq, serde::Serialize)]
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
#[derive(Debug, Clone, PartialEq, Eq, serde::Serialize)]
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
#[derive(Debug, Clone, Copy, PartialEq, Eq, serde::Serialize)]
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
#[derive(Debug, Clone, PartialEq, Eq, serde::Serialize)]
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
#[derive(Debug, Clone, Copy, PartialEq, Eq, serde::Serialize)]
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
#[derive(Debug, Clone, PartialEq, Eq, serde::Serialize)]
pub struct StationThresholdAbility {
    pub threshold: super::phrase::NumberLiteral,
    /// Boxed so the variant stays small, mirroring `QuotedAbility::ability`.
    pub ability: Box<Ability>,
}

#[derive(Debug, Clone, PartialEq, Eq, serde::Serialize)]
pub struct ActivatedAbility {
    pub cost: Cost,
    pub effect: Paragraph,
}

/// An activation cost: the comma-separated list of components paid before the
/// colon. Every cost is a list of typed [`CostComponent`]s; the earlier raw
/// `SymbolList(String)` and untyped `Components(Vec<Phrase>)` shapes are gone.
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct Cost {
    flavor_header: Option<FlavorHeader>,
    components: Vec<CostComponent>,
}

impl Cost {
    pub(crate) const fn from_parts(
        flavor_header: Option<FlavorHeader>,
        components: Vec<CostComponent>,
    ) -> Self {
        Self {
            flavor_header,
            components,
        }
    }

    /// Returns the cost's optional flavor header.
    #[must_use]
    pub const fn flavor_header(&self) -> Option<&FlavorHeader> {
        self.flavor_header.as_ref()
    }

    /// Returns the cost components in surface order.
    #[must_use]
    pub fn components(&self) -> &[CostComponent] {
        &self.components
    }
}

impl serde::Serialize for Cost {
    fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error>
    where
        S: serde::Serializer,
    {
        use serde::ser::SerializeStruct;

        let mut state = serializer.serialize_struct("Cost", 2)?;
        state.serialize_field("flavor_header", &self.flavor_header)?;
        state.serialize_field("components", &self.components)?;
        state.end()
    }
}

/// One component of an activation cost's comma-separated list. A closed sum
/// over the cost-component *shapes* the supported corpus attests, never over a
/// cost's meaning: a mana/symbol run, a cost expressed as an independent
/// clause, a bare noun phrase, or a verbatim recovery when no shape parses.
/// Which shape a component takes is decided by its surface alone — the clause's
/// verb is open, the shape is not, so this crate records no per-action semantic
/// facts. An escape variant ([`Recovered`](CostComponent::Recovered)) keeps the
/// type from over-closing, mirroring [`KeywordArgument::Recovered`].
#[derive(Debug, Clone, PartialEq, Eq, serde::Serialize)]
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

#[derive(Debug, Clone, PartialEq, Eq, serde::Serialize)]
pub struct TriggeredAbility {
    pub conditions: TriggerConditionList,
    pub intervening_condition: Option<DependentClause>,
    pub effect: Paragraph,
}

/// The ordered conditions that can trigger one ability. A single-condition
/// ability has an empty [`Self::rest`]; a mixed-introducer ability keeps each
/// introducer paired with its own event instead of folding the later condition
/// into the first event or splitting the effect into multiple abilities.
#[derive(Debug, Clone, PartialEq, Eq, serde::Serialize)]
pub struct TriggerConditionList {
    pub first: TriggerCondition,
    pub rest: Vec<TriggerConditionCoordination>,
}

#[derive(Debug, Clone, PartialEq, Eq, serde::Serialize)]
pub struct TriggerConditionCoordination {
    #[serde(serialize_with = "super::legacy_serde::serialize_predicate_conjunction")]
    pub conjunction: Conjunction,
    pub condition: TriggerCondition,
}

#[derive(Debug, Clone, PartialEq, Eq, serde::Serialize)]
pub struct TriggerCondition {
    pub introducer: TriggerWord,
    pub event: TriggerEvent,
}

/// The common header of a single-event trigger embedded in a larger syntax
/// node. Multi-condition top-level triggered abilities use
/// [`TriggerConditionList`] instead.
#[derive(Debug, Clone, PartialEq, Eq, serde::Serialize)]
pub struct TriggerHeader {
    pub introducer: TriggerWord,
    pub event: TriggerEvent,
    pub intervening_condition: Option<DependentClause>,
}

#[derive(Debug, Clone, PartialEq, Eq, serde::Serialize)]
pub enum TriggerEvent {
    Clause(IndependentClause),
    Temporal(NounPhrase),
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, serde::Serialize)]
pub enum TriggerWord {
    When,
    Whenever,
    At,
}

impl TriggerWord {
    const FORMS: &'static [(Self, &'static str)] = &[
        (Self::When, "when"),
        (Self::Whenever, "whenever"),
        (Self::At, "at"),
    ];

    pub(crate) fn from_spelling(surface: &str) -> Option<Self> {
        Self::FORMS
            .iter()
            .find_map(|(word, spelling)| surface.eq_ignore_ascii_case(spelling).then_some(*word))
    }

    pub(crate) fn spelling(self) -> &'static str {
        Self::FORMS
            .iter()
            .find_map(|(word, spelling)| (*word == self).then_some(*spelling))
            .expect("every trigger word has one spelling")
    }
}

#[derive(Debug, Clone, PartialEq, Eq, serde::Serialize)]
pub struct LoyaltyAbility {
    pub cost: LoyaltyCost,
    pub effect: Paragraph,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, serde::Serialize)]
pub struct LoyaltyCost {
    pub sign: LoyaltyCostSign,
    pub value: LoyaltyCostValue,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, serde::Serialize)]
pub enum LoyaltyCostSign {
    None,
    Plus,
    Minus,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, serde::Serialize)]
pub enum LoyaltyCostValue {
    Number(u32),
    X,
}

#[derive(Debug, Clone, PartialEq, Eq, serde::Serialize)]
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
#[derive(Debug, Clone, PartialEq, Eq, serde::Serialize)]
pub enum ModalFrame {
    Unframed,
    Activated(Cost),
    Triggered(TriggerHeader),
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

/// **Measured, field KEPT** (surface-fact sweep, 2026-07-30): the best
/// candidate derivation — `SpacedEmDash` iff the modal header's last sentence
/// is a `SentenceBody::Choice` instruction — leaves 8 residual mismatches out
/// of 31685 supported faces: Bumi King of Three Trials, Fatal Lore, Library
/// of Lat-Nam, Lita Little Orphan Amphibian, Misfortune, Riku of Many Paths,
/// Tranquil Frillback, Zuko Conflicted. All 8 share one shape: a
/// **third-person "chooses" clause** standing in for the imperative Choice
/// instruction — "An opponent chooses one —" (Fatal Lore, Misfortune),
/// "choose one that hasn't been chosen and you lose 2 life —" (Zuko,
/// Conflicted). These parse as an ordinary `SentenceBody::Independent`
/// transitive clause, not the dedicated `Choice` node reserved for the bare
/// imperative "Choose X", yet the surface still takes the dash — a real
/// recognition gap in what counts as "a choice header", not corpus noise.
/// Two coarser candidates were also tried and rejected: "non-empty header"
/// (77 mismatches — a trailing restriction sentence after a `Choose one` also
/// keeps a non-empty header without the dash) and "header is exactly one
/// sentence" (24 mismatches, worse than the Choice-instruction rule). Field
/// stays stored.
#[derive(Debug, Clone, Copy, PartialEq, Eq, serde::Serialize)]
pub enum ModalHeaderSuffix {
    None,
    SpacedEmDash,
}

#[derive(Debug, Clone, PartialEq, Eq, serde::Serialize)]
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
#[derive(Debug, Clone, PartialEq, Eq, serde::Serialize)]
pub struct ModeHeading {
    pub label: FlavorHeader,
    pub cost: Cost,
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct KeywordAbilityList {
    abilities: Vec<KeywordAbility>,
    /// Ordinary rules sentences printed on the same physical line after the
    /// terminal carried by the final keyword's cost (`Flashback—{3}{R},
    /// Remove X loyalty counters from among planeswalkers you control. If you
    /// cast this spell this way, X can't be 0.`). Not part of the keyword's
    /// own [`KeywordArgument`], the [`Cost`], or a new top-level [`Ability`],
    /// because all three would lose the surface's line/space boundary.
    trailing: Option<Paragraph>,
}

impl KeywordAbilityList {
    pub(crate) const fn from_parts(
        abilities: Vec<KeywordAbility>,
        trailing: Option<Paragraph>,
    ) -> Self {
        Self {
            abilities,
            trailing,
        }
    }

    /// Returns the keyword abilities in surface order.
    #[must_use]
    pub fn abilities(&self) -> &[KeywordAbility] {
        &self.abilities
    }

    /// Returns the paragraph printed after the keyword list, if present.
    #[must_use]
    pub const fn trailing(&self) -> Option<&Paragraph> {
        self.trailing.as_ref()
    }
}

impl serde::Serialize for KeywordAbilityList {
    fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error>
    where
        S: serde::Serializer,
    {
        use serde::ser::SerializeStruct;

        let mut state = serializer.serialize_struct("KeywordAbilityList", 2)?;
        state.serialize_field("abilities", &self.abilities)?;
        state.serialize_field("trailing", &self.trailing)?;
        state.end()
    }
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
#[derive(Debug, Clone, PartialEq, Eq, serde::Serialize)]
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
#[derive(Debug, Clone, PartialEq, Eq, serde::Serialize)]
pub enum KeywordArgument {
    /// No argument — flying, first strike, deathtouch.
    Absent,
    /// A count. The shape's exemplar rules give the numeric argument as `N`
    /// [CR#702.86a,702.164a].
    Counted(Quantity),
    /// A cost: a symbol-sequence, a tight em-dash structured cost, or (legacy
    /// surface preservation) a spaced em-dash embedded ability — the
    /// [`KeywordCost`] surfaces. The shape's exemplar rules give the argument
    /// as `[cost]` [CR#702.21a,702.29a].
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
    /// A bare noun-phrase argument: no preposition, no cost, no list — the
    /// shape [CR#702.5a] gives enchant (`Enchant [object or player]`) and
    /// [CR#702.72a] gives champion (`Champion an [object]`). Neither
    /// [`Self::Predicated`] (which needs `from`/`for` or a list) nor
    /// [`Self::RestrictedCost`] (which needs a trailing symbol cost) admits it,
    /// so without this variant such a line leaves the keyword frame for the
    /// general sentence grammar, where `Enchant tapped creature` derives a
    /// transitive clause whose subject is the `Enchant` atom and whose verb is
    /// a past-tense `tap` — a reading that round-trips clean, so no fidelity
    /// gate can see it.
    ///
    /// Holds a whole [`Phrase`] rather than a `NounPhrase` so the slot can
    /// reach the rest of the phrase grammar as the remaining shapes collapse
    /// into it; only the noun-phrase arm is produced today.
    Qualified(Phrase),
    /// A symbol cost paired with power/toughness by a spaced em dash. The
    /// shape's exemplar rules give the argument as `[cost] — [P]/[T]`
    /// [CR#702.160a,718.1]. A shape added beyond the CR's six observed
    /// keyword-parameter shapes for the one argument surface that needs it
    /// (see the round's report).
    Statted {
        symbols: Vec<OracleSymbol>,
        stats: PowerToughness,
    },
    /// A verbatim selector label. Em-dash pairing labels are licensed by their
    /// punctuation shape [CR#702.124i]; space-separated labels require exact
    /// catalog membership. Gift's labels are whole selectors, not noun phrases
    /// [CR#702.174a,702.174d,702.174e,702.174f,702.174g,702.174h,702.174i].
    ///
    /// **Measured, `separator` field KEPT** (surface-fact sweep, 2026-07-30):
    /// on the supported corpus this looked like a clean per-keyword-atom
    /// constant — `Partner`/`Modular` always pair with the tight em dash (19
    /// `Partner—…` occurrences + `Modular—Sunburst`), every other keyword
    /// reaching this shape (`Gift`) always pairs with a plain space (22
    /// `Gift a…` occurrences) — and deriving it that way (threading the
    /// owning `KeywordAbility.ability` into the renderer, a contained change)
    /// gave 0 mismatches across all 31685 supported faces. It was reverted
    /// anyway: `parse_named_keyword_argument`'s own license test,
    /// `named_keyword_argument_label_license_is_exact_and_keyword_independent`,
    /// explicitly asserts and is named for the fact that this shape is
    /// **keyword-independent by design** — `shape_argument("Partner a Food")`
    /// is a real, intentionally-licensed parse (`Partner` with a *space*
    /// separator, not the em dash the corpus-only rule would force), and
    /// forcing the per-keyword derivation makes that exact test fail its own
    /// render round-trip (`"Partner—a Food"` vs `"Partner a Food"`). The
    /// corpus today never exercises the combinations the grammar
    /// deliberately admits, but the field is real, licensed variation, not a
    /// redundant echo — deleting it would silently misrender the very shape
    /// a previous round built and tested this parser to accept.
    Named {
        separator: KeywordArgumentSeparator,
        label: String,
    },
    /// No closed shape parsed the argument tokens; they recover verbatim at the
    /// keyword-argument role — the term-level echo of the model's
    /// "misparameterization has no term."
    ///
    /// No `separator` field: the sole lowering site (the "no shape parsed"
    /// fallback) only ever carries `Space` on the supported corpus —
    /// measured with 0 mismatches across all 31685 supported faces (surface-
    /// fact sweep, 2026-07-30). The renderer emits a literal space instead.
    Recovered { text: RecoveredText },
    /// A quality restriction (optionally introduced by `onto`/`with`) paired
    /// with a cost — `craft with artifact {1}{U}`, `splice onto Arcane
    /// {W}`. One shape, not a sibling per surface family: the restriction
    /// retains the full noun-phrase tree (including the legitimate headless
    /// `NounPhraseKind::Quantity` case, `craft with one or more {5}`), and the
    /// cost is `Symbols` or (composed with the tight em-dash structured cost)
    /// `Components` [CR#702.6c,702.6e,702.47a,702.167a].
    RestrictedCost {
        preposition: Option<Preposition>,
        restriction: Box<NounPhrase>,
        cost: KeywordCost,
    },
}

/// The surfaces a [`KeywordArgument::Costed`] cost takes.
#[derive(Debug, Clone, PartialEq, Eq, serde::Serialize)]
pub enum KeywordCost {
    /// A mana/symbol cost written after a space — `ward {2}`, `equip {3}`. The
    /// run is carried as its structured oracle symbols, reproduced by
    /// concatenation.
    Symbols(Vec<OracleSymbol>),
    /// Legacy surface preservation for the spaced em-dash cost surface
    /// (`SpacedEmDash`) only — `Exhaust — {2}{G}{G}: Put two +1/+1 counters on
    /// this creature.` reads as a designation-headed embedded ability, never
    /// a cost. Not the tight `[cost]` path; see
    /// [`Components`](Self::Components).
    ///
    /// No `separator` field: as the "only" above already says, this variant
    /// is only ever constructed with `SpacedEmDash` — measured with 0
    /// mismatches across all 31685 supported faces (surface-fact sweep,
    /// 2026-07-30). The renderer emits the spaced em dash unconditionally.
    Sentence { ability: Box<Ability> },
    /// A non-mana cost written as a tight em-dash sentence —
    /// `cumulative upkeep—Put a -1/-1 counter on this creature.` The dash
    /// spacing is carried structurally so rendering never inspects the
    /// surface [CR#702.21a,702.138a], though it is always the tight `EmDash`
    /// in practice — measured with 0 mismatches (surface-fact sweep,
    /// 2026-07-30), so it is no longer a field either.
    ///
    /// `terminal` only records *whether* the cost body ended in a sentence
    /// terminal, not *which* punctuation: a former three-way `Period`/
    /// `Exclamation`/`Question` carrier all round-tripped as a bare period
    /// with 0 mismatches (surface-fact sweep, 2026-07-30), so the
    /// punctuation-glyph distinction was removed. `false` prints nothing
    /// after the cost (matching the pre-existing `None` behavior); a present
    /// terminal is never anything but `.` on the supported corpus.
    Components { cost: Cost, terminal: bool },
}

/// A [`KeywordArgument::Predicated`] quality filter: one quality, or several
/// coordinated qualities that the surface joins with ` and ` and the CR treats
/// as separate abilities [CR#702.16g,702.11f].
#[derive(Debug, Clone, PartialEq, Eq, serde::Serialize)]
pub struct PredicatedArgument {
    pub qualities: Vec<PredicatedQuality>,
}

/// One quality of a [`PredicatedArgument`].
#[derive(Debug, Clone, PartialEq, Eq, serde::Serialize)]
pub struct PredicatedQuality {
    /// The preposition introducing this quality on the surface (`from` for
    /// protection/hexproof, `for` for affinity), or `None` when the keyword
    /// atom itself carries it (`Hexproof from`).
    pub preposition: Option<Preposition>,
    pub quality: Phrase,
}

/// **Re-measured, field KEPT** (surface-fact sweep-residue, 2026-07-30
/// measurement round). The prior round's causal story — "genuine `WotC`
/// semicolon style, likely tied to now-stripped reminder text" — was never
/// checked against the actual 34 faces; it turns out to be half right.
///
/// All 34 witnesses (e.g. Longbow Archer's `First strike; reach`, Kjeldoran
/// Skycaptain's `Flying; first strike; banding`) have the *pre-strip* source
/// text confirmed by hand: in every one, the semicolon-joined list's final
/// keyword is immediately followed by that keyword's own inline reminder
/// parenthetical (`reach (This creature can block creatures with flying.)`,
/// `banding (Any creatures with banding, and up to one without, …)`). This
/// is not a stripping artifact — `strip_reminder_text` removes only the
/// parenthesized group, so the semicolon (which precedes it) is a separate
/// token that survives stripping intact and reaches this field unchanged.
///
/// But "reminder-bearing final keyword ⟹ semicolon" is not a rule the corpus
/// supports: `Trample, myriad (Whenever this creature attacks, …)` (Elturel
/// Survivors, Polygoyf), `Vigilance, trample (Attacking doesn't cause…)`
/// (Spider-Man, Miles Morales), `Trample, haste (This creature can deal
/// excess…)` (Spark Elemental), and `Swampwalk, forestwalk (This creature
/// can't be blocked…)` (Stalker Hag) are the identical shape — a final
/// keyword with its own inline reminder — joined with a plain comma. The
/// real split is per-keyword: `banding`, `flanking`, `horsemanship`,
/// `rampage`, `fear`, `bushido`, `reach`, `menace`, `convoke`, and the
/// landwalk family never take a comma when reminder-bearing in this corpus
/// (e.g. zero occurrences of `, banding` anywhere, vs. eleven of `;
/// banding`), while `myriad`, `trample`, `haste`, and `forestwalk` never
/// take a semicolon. Most of the semicolon set are long-retired mechanics
/// (banding, flanking, rampage, horsemanship) whose "keyword + reminder"
/// Oracle-text block looks frozen from an older print era, but `menace` and
/// `convoke` are modern keywords with exactly one semicolon witness each, so
/// "old keyword" is not the full story either. This looks like genuine,
/// idiosyncratic `WotC` Oracle-text curation keyed to specific keyword
/// identities — language, not a parser or stripping defect — but the
/// governing fact (which keyword's canonical reminder block was authored
/// with a leading semicolon) is external to anything this grammar retains
/// post-strip, so it cannot be derived from the current AST. Field stays
/// stored.
#[derive(Debug, Clone, Copy, PartialEq, Eq, serde::Serialize)]
pub enum KeywordListSeparator {
    Comma,
    Semicolon,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, serde::Serialize)]
pub enum KeywordArgumentSeparator {
    Space,
    EmDash,
    SpacedEmDash,
}

#[derive(Debug, Clone, Default, PartialEq, Eq, serde::Serialize)]
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
#[derive(Debug, Clone, PartialEq, Eq, serde::Serialize)]
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
#[derive(Debug, Clone, PartialEq, Eq, serde::Serialize)]
pub struct Sentence {
    pub(crate) body: SentenceBody,
}

impl Sentence {
    /// Builds the generated sentence family through its checked declaration
    /// adapter. Dependent clauses are rejected.
    ///
    /// # Errors
    ///
    /// Returns a declaration violation when `clause` is not an independent,
    /// standalone clause admitted by the generated sentence declaration.
    pub fn try_from_clause(
        clause: Clause,
    ) -> Result<Self, deckmaste_construction_compiler::runtime::DeclarationViolation> {
        crate::constructions::sentence::build_sentence(clause)
    }

    pub(crate) fn from_body(body: SentenceBody) -> Self {
        if let SentenceBody::Independent(clause) = body {
            return crate::constructions::sentence::build_sentence(Clause::Independent(clause))
                .expect("an independent clause satisfies the Sentence construction");
        }
        Self { body }
    }

    #[must_use]
    pub fn body(&self) -> &SentenceBody {
        &self.body
    }
}

#[allow(
    clippy::large_enum_variant,
    reason = "sentence bodies can legitimately hold a large independent clause payload"
)]
#[derive(Debug, Clone, PartialEq, Eq, serde::Serialize)]
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

/// A non-initial trigger sentence. Its common trigger fields live in
/// [`TriggerHeader`], followed by the effect clause the trigger governs. The
/// effect is a single [`IndependentClause`], not a [`Paragraph`]: a
/// sentence-level trigger governs exactly the remainder of its own sentence.
#[derive(Debug, Clone, PartialEq, Eq, serde::Serialize)]
pub struct TriggeredSentence {
    pub trigger: TriggerHeader,
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
#[derive(Debug, Clone, PartialEq, Eq, serde::Serialize)]
pub struct ChoiceInstruction {
    /// Boxed so the choice instruction stays no larger than a bare imperative
    /// header sentence: the trigger clause carries a full event clause, and the
    /// unboxed grammar-lowering enum that wraps [`Sentence`] must not grow.
    pub trigger_prefix: Option<Box<TriggerHeader>>,
    pub imperative: Predicate,
    pub at_random: bool,
}

/// A quoted ability filling an object or coordination slot (`gains "…"`).
///
/// Where the interior's terminal period sits is **not** stored: a quoted
/// ability that closes its enclosing sentence absorbs that sentence's period
/// into the quote (`gains "…."`), while the same ability in a non-final slot —
/// a coordinated conjunct (`has "…" and "…."`) or one before a trailing adjunct
/// (`gains "…" until end of turn.`) — prints without it. That is a fact about
/// the quote's *position*, not about its interior, and the renderer already
/// walks each sentence's AST tail to derive the sentence period; the same walk
/// names the quote the period belongs inside.
///
/// Whether the closing `"` itself prints is likewise **not** stored (a former
/// `closed` field was removed, surface-fact sweep, 2026-07-30): every
/// construction site in the crate set it to `true`, so the renderer now emits
/// the closing quote unconditionally — measured with 0 mismatches across all
/// 31685 supported faces.
#[derive(Debug, Clone, PartialEq, Eq, serde::Serialize)]
pub struct QuotedAbility {
    pub ability: Box<Ability>,
    /// Whether the quoted interior takes an initial capital. NOT derivable,
    /// and the refutation is a witness pair with the same structure and
    /// different bits: both Takklemaggot's `gains "At the beginning of that
    /// player's upkeep, this enchantment deals 1 damage to that player."` and
    /// Master of the Hunt's `It has "bands with other creatures named Wolves
    /// of the Hunt."` quote an interior that parses as an ordinary
    /// `Paragraph` whose first sentence is a parsed `Independent` clause, yet
    /// Oracle prints the first capitalized and the second lowercase. A
    /// derivation from node kind was measured against the supported corpus:
    /// keyword-line and recovered interiors are correctly predicted (a
    /// keyword line's leading text is its `CatalogAtom`'s *observed*
    /// spelling and a recovered span is verbatim, so neither may be
    /// re-cased), but the parsed-sentence interiors split, so the bit stays
    /// stored.
    pub initial_uppercase: bool,
}
