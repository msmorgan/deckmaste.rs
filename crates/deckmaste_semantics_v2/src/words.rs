//! The vocabulary: every leaf type a phrase, trigger, effect, or card is
//! written from. Mirrors `lean/Semantics/Words.lean` constructor-for-constructor
//! and field-for-field; Lean is the specification (see
//! `docs/decisions/semantics-v2.md` §10).

use macro_ron::Expand;
use macro_ron::SupportsMacros;
use serde::Deserialize;
use serde::Serialize;

/// [CR#205.2a]
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, Deserialize, Expand, Serialize)]
pub enum CardType {
    Creature,
    Artifact,
    Land,
    Enchantment,
    Instant,
    Sorcery,
    Planeswalker,
    Battle,
    Kindred,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, Deserialize, Expand, Serialize)]
pub enum Stat {
    Power,
    Toughness,
    ManaValue,
    Loyalty,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, Deserialize, Expand, Serialize)]
pub enum PlayerStat {
    LifeTotal,
    StartingLifeTotal,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, Deserialize, Expand, Serialize)]
pub enum Comparator {
    AtLeast,
    AtMost,
    Greater,
    Less,
    Eq,
}

/// The sorts a quality noun ("a color", "a creature type", "a card name") ranges over.
#[derive(Debug, Clone, PartialEq, Eq, Hash, Deserialize, Expand, Serialize)]
pub enum QualitySort {
    Color,
    Subtype {
        host: CardType,
    },
    CardName,
    Number,
    CardType,
    CounterKind,
    /// "an ability" chosen among keywords ("choose flying or trample").
    Ability,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, Deserialize, Expand, Serialize)]
pub enum Letter {
    X,
    Y,
}

/// The sort of thing a phrase denotes.
#[derive(Debug, Clone, PartialEq, Eq, Hash, Deserialize, Expand, Serialize)]
pub enum Kind {
    Object,
    Player,
    Quality {
        sort: QualitySort,
    },
    Outcome,
    Gap,
    Letter {
        letter: Letter,
    },
    TurnRef,
    /// A pile of objects; the pile itself is not an object [CR#700.3b].
    Pile,
    /// Idris `(\/)`: a phrase denoting either sort ("target creature or player").
    Join {
        left: Box<Kind>,
        right: Box<Kind>,
    },
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, Deserialize, Expand, Serialize)]
pub enum AggregateOp {
    Sum,
    Min,
    Max,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, Deserialize, Expand, Serialize)]
pub enum SubtypeScope {
    Any,
    BasicOnly,
    NonbasicOnly,
}

/// An axis a "for each kind of …" ranges over.
#[derive(Debug, Clone, PartialEq, Eq, Hash, Deserialize, Expand, Serialize)]
pub enum KindAxis {
    CardType,
    PermanentType,
    Color,
    Subtype { host: CardType, scope: SubtypeScope },
    Value { stat: Stat },
    CounterKind,
    ColorPair,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, Deserialize, Expand, Serialize)]
pub enum OutcomeSort {
    DamageDealt,
    LifeGained,
    LifeLost,
    CountersPut,
    DamagePrevented,
    RollResult,
    CoinFlipped,
    DiceRolled,
    NamedNumber,
    RepeatCount,
    CountersRemoved,
    ManaAdded,
    ManaProduced,
    CeilingShortfall,
    VoteHeld,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, Deserialize, Expand, Serialize)]
pub enum FlipCall {
    Wins,
    Loses,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, Deserialize, Expand, Serialize)]
pub enum CoinFace {
    Heads,
    Tails,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, Deserialize, Expand, Serialize)]
pub enum RollExtreme {
    Lowest,
    Highest,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, Deserialize, Expand, Serialize)]
pub enum Plurality {
    One,
    Many,
}

/// Static number facts carried by a typed parameter read and checked against its binding.
#[derive(Debug, Clone, PartialEq, Eq, Hash, Deserialize, Expand, Serialize)]
pub struct AmountShape {
    pub exact: Option<u32>,
    pub non_zero: bool,
    pub plurality: Plurality,
    pub read: bool,
    pub literal: bool,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, Deserialize, Expand, Serialize)]
pub enum NameAgreement {
    DifferentNames,
    SameName,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, Deserialize, Expand, Serialize)]
pub enum Determiner {
    Target,
    A,
    Each,
    All,
    The,
    Part,
    Count,
    #[serde(rename = "Self")]
    Self_,
    Bare,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, Deserialize, Expand, Serialize)]
pub enum PossessorAxis {
    Owner,
    Controller,
}

/// How an object stands in combat, optionally toward a counterpart: "attacking" is
/// `attackerOf` with none, "creature blocking it" is `blockerOf` with one. `declaredAttacker` is
/// the declare-attackers moment [CR#508.1].
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, Deserialize, Expand, Serialize)]
pub enum CombatRelation {
    AttackerOf,
    DeclaredAttacker,
    BlockerOf,
    BlockedBy,
    AttackedBy,
    CouldBlock,
    CouldBeBlockedBy,
}

/// The two players of a combat [CR#506.2].
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, Deserialize, Expand, Serialize)]
pub enum CombatRole {
    Attacking,
    Defending,
}

/// An attachment's coming or going.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, Deserialize, Expand, Serialize)]
pub enum AttachMove {
    Attached,
    Unattached,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, Deserialize, Expand, Serialize)]
pub enum ArithOp {
    Plus,
    Minus,
    Times,
    DifferenceBetween,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, Deserialize, Expand, Serialize)]
pub enum Zone {
    Battlefield,
    Graveyard,
    Exile,
    Hand,
    Library,
    Stack,
    Command,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, Deserialize, Expand, Serialize)]
pub enum LibraryEnd {
    Top,
    Bottom,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, Deserialize, Expand, Serialize)]
pub enum Arrangement {
    AnyOrder,
    RandomOrder,
}

/// The N of "Nth" [CR#401.7]; the Idris carries `IsSucc n`.
#[derive(Debug, Clone, PartialEq, Eq, Hash, Deserialize, Expand, Serialize)]
pub enum Ordinal {
    Nth { n: u32 },
}

/// A designation's name [CR#701.15b,725.1,731.1]. Which designations exist, and what each
/// holds and does, is the designation facts table, generated like the keyword table; the grammar
/// carries only the mechanism.
pub type DesignationLabel = String;

pub type KeywordLabel = String;

pub type AbilityWordLabel = String;

pub type FlavorWordLabel = String;

pub type VoteLabel = String;

/// A keyword action's name [CR#701.1]. The keyword actions are an open set the registry
/// declares, so a deed of that source is keyed by its written label, as a designation is.
pub type KeywordActionLabel = String;

/// Deeds the core rules define outside the keyword actions of [CR#701.1]: turn-based actions
/// [CR#508.1,509.1] and the other actions the rules define in their own sections. Closed, so a law
/// matches them structurally instead of naming a lexeme.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, Deserialize, Expand, Serialize)]
pub enum CoreDeed {
    Attack,
    Block,
    Target,
    Copy,
    Draw,
    GainLife,
    LoseGame,
    WinGame,
    Spend,
    Trigger,
    Put,
    Return,
    GainControl,
    Unlock,
    FullyUnlock,
}

/// A condition that causes a state-based action [CR#704.1].
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, Deserialize, Expand, Serialize)]
pub enum StateBasedCause {
    /// A player has 0 or less life [CR#704.5a].
    NonpositiveLife,
}

/// A deed: something a card's text says is done. Three sources define one, and only the middle
/// one is open: the core rules [CR#508.1,509.1,709.5f], the keyword actions the registry declares
/// [CR#701.1], and the verb a keyword ability defines for itself [CR#702.122b,702.171a,702.26a].
#[derive(Debug, Clone, PartialEq, Eq, Hash, Deserialize, Expand, Serialize)]
pub enum Deed {
    Core { deed: CoreDeed },
    Action { label: KeywordActionLabel },
    OfAbility { keyword: KeywordLabel },
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, Deserialize, Expand, Serialize)]
pub enum PileFace {
    FaceDown,
    FaceUp,
}

#[derive(Debug, Clone, PartialEq, Eq, Hash, Deserialize, Expand, Serialize)]
pub enum ChoiceSort {
    Quality { sort: QualitySort },
    Player,
}

/// Which die a roll instruction names.
#[derive(Debug, Clone, PartialEq, Eq, Hash, Deserialize, Expand, Serialize)]
pub enum DieSides {
    Sides { sides: u32 },
    ThoseDice,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, Deserialize, Expand, Serialize)]
pub enum ChoiceRef {
    TheChoice,
    TheLatestChoice,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, Deserialize, Expand, Serialize)]
pub enum Disclosure {
    Openly,
    Secretly,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, Deserialize, Expand, Serialize)]
pub enum HiddenSort {
    Numbers,
    Choices,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, Deserialize, Expand, Serialize)]
pub enum ExposeVerb {
    LookAt,
    Reveal,
}

/// The noun a pronoun ("that player", "that card") is written with.
#[derive(Debug, Clone, PartialEq, Eq, Hash, Deserialize, Expand, Serialize)]
pub enum NounWord {
    Type {
        r#type: CardType,
    },
    Card,
    Spell,
    Player,
    Permanent,
    Token,
    Copy,
    Join,
    /// "Counter target spell or ability. … THAT SPELL OR ABILITY"
    Stack,
    /// "Whenever you activate an ability, … copy THAT ABILITY"
    Ability,
    /// A written type narrows the word without replacing its carrier requirement.
    OfType {
        word: Box<NounWord>,
        r#type: CardType,
    },
    /// Copy origin narrows the word; an ability copy still has the ability head [CR#707.10].
    Copied {
        word: Box<NounWord>,
    },
    /// "Put THAT PILE into your hand and the other into your graveyard."
    Pile,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, Deserialize, Expand, Serialize)]
pub enum VerbedMarking {
    Attributive,
    ThisWay,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, Deserialize, Expand, Serialize)]
pub enum SlotCarrier {
    Permanent,
    Card,
    Spell,
}

/// Structural facts retained by a captured subject, independent of its current properties.
#[allow(
    clippy::struct_excessive_bools,
    reason = "the field set mirrors `NounShape` in lean/Semantics/Words.lean; Lean is the spec"
)]
#[derive(Debug, Clone, PartialEq, Eq, Hash, Deserialize, Expand, Serialize)]
pub struct NounShape {
    pub kind: Kind,
    pub ability: bool,
    pub is_you: bool,
    pub self_defined: bool,
    pub ascribable: bool,
    pub two_parties: bool,
    pub opponent_only: bool,
    pub bare_this: bool,
}

/// How a pronoun is written: what it reaches back for.
#[derive(Debug, Clone, PartialEq, Eq, Hash, Deserialize, Expand, Serialize)]
pub enum Reach {
    Parameter {
        shape: NounShape,
    },
    Bare,
    AtSlot {
        slot: SlotCarrier,
    },
    Stamped {
        verb: Deed,
    },
    TokenBorn,
    Word {
        word: NounWord,
    },
    UnionHalf {
        word: NounWord,
    },
    Verbed {
        verb: Deed,
        word: NounWord,
        marking: VerbedMarking,
    },
    ThatTurn,
}

/// The stretch of the antecedent stack a pronoun resolves in.
#[derive(Debug, Clone, PartialEq, Eq, Hash, SupportsMacros)]
pub enum Window {
    Whole,
    /// A generated subject read in its lexical macro scope.
    Parameter {
        scope: u32,
        index: u32,
    },
    Top {
        depth: u32,
    },
    Below {
        depth: u32,
    },
    Introduced {
        pattern: Vec<Kind>,
    },
    OutsideIntroduced {
        pattern: Vec<Kind>,
    },
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, Deserialize, Expand, Serialize)]
pub enum EntryCounterMark {
    Fresh,
    Additional,
    Fewer,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, Deserialize, Expand, Serialize)]
pub enum PlayerGroupWord {
    AllPlayers,
    YourOpponents,
    YourTeam,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, Deserialize, Expand, Serialize)]
pub enum RoundMode {
    Up,
    Down,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, Deserialize, Expand, Serialize)]
pub enum ScaleFactor {
    Doubled,
    Tripled,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, Deserialize, Expand, Serialize)]
pub enum ShiftDir {
    Up,
    Down,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, Deserialize, Expand, Serialize)]
pub enum Lookback {
    ThisTurn,
    EarlierThisTurn,
    ThisCombat,
    LastTurn,
    ThisGame,
    ThisWay,
    Triggering,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, Deserialize, Expand, Serialize)]
pub enum TargetExtent {
    SomeTarget,
    SoleTarget,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, Deserialize, Expand, Serialize)]
pub enum CopySort {
    FromStack,
    FromCardZone,
}

/// A family of keywords ("a landwalk ability").
#[derive(Debug, Clone, PartialEq, Eq, Hash, Deserialize, Expand, Serialize)]
pub struct KeywordFamily {
    pub word: KeywordLabel,
    pub sort: Option<QualitySort>,
}

#[derive(Debug, Clone, PartialEq, Eq, Hash, Deserialize, Expand, Serialize)]
pub enum KeywordTerm {
    The {
        keyword: KeywordLabel,
    },
    AnyIn {
        family: KeywordFamily,
    },
    /// A keyword named with its number, as "rampage 3" is.
    TheWith {
        keyword: KeywordLabel,
        number: u32,
    },
}

#[derive(Debug, Clone, PartialEq, Eq, Hash, Deserialize, Expand, Serialize)]
pub enum PaidCostName {
    ByKeyword {
        keyword: KeywordLabel,
    },
    ByNthKeyword {
        ordinal: Ordinal,
        keyword: KeywordLabel,
    },
    TheAlternative,
    TheAdditional,
}

#[derive(Debug, Clone, PartialEq, Eq, Hash, Deserialize, Expand, Serialize)]
pub enum PaidFacet {
    /// colors of mana spent to cast it [CR#702.44a]
    ColorsSpent,
    /// mana spent to pay the total cost [CR#601.2h]
    ManaValueSpent,
    /// [CR#702.33c..702.33d]
    TimesPaid { which: PaidCostName },
    /// "was kicked" [CR#702.33d]
    Readback {
        which: PaidCostName,
        lookback: Option<Lookback>,
    },
}

#[derive(Debug, Clone, PartialEq, Eq, Hash, Deserialize, Expand, Serialize)]
pub enum AbilityClass {
    AnyOnStack,
    AnyActivated,
    AnyTriggered,
    Loyalty,
    Keyword { k: KeywordLabel },
}

#[derive(Debug, Clone, PartialEq, Eq, Hash, Deserialize, Expand, Serialize)]
pub enum ItalicWord {
    AbilityWord { label: AbilityWordLabel },
    FlavorWord { label: FlavorWordLabel },
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, Deserialize, Expand, Serialize)]
pub enum Color {
    White,
    Blue,
    Black,
    Red,
    Green,
}

#[derive(Debug, Clone, PartialEq, Eq, Hash, Deserialize, Expand, Serialize)]
pub enum ColorOrColorless {
    Colorless,
    Of { color: Color },
}

#[derive(Debug, Clone, PartialEq, Eq, Hash, Deserialize, Expand, Serialize)]
pub enum SimpleManaSymbol {
    Generic { amount: u32 },
    Specific { color: ColorOrColorless },
}

#[derive(Debug, Clone, PartialEq, Eq, Hash, Deserialize, Expand, Serialize)]
pub enum ManaSymbol {
    Simple {
        symbol: SimpleManaSymbol,
    },
    Hybrid {
        left: SimpleManaSymbol,
        right: Color,
    },
    Phyrexian {
        color: Color,
        second: Option<Color>,
    },
    Variable,
    Snow,
}

pub type ManaCost = Vec<ManaSymbol>;

/// One run of produced mana, e.g. `{G}{G}` or `{C}`.
pub type ProducedRun = Vec<ColorOrColorless>;

/// The type space a "with every … type" quality ranges over.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, Deserialize, Expand, Serialize)]
pub enum SubtypeSpace {
    BasicLand,
    Land,
    Creature,
}

/// Which side of a deed a deontic rule speaks about.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, Deserialize, Expand, Serialize)]
pub enum Role {
    Agent,
    Patient,
}

#[derive(Debug, Clone, PartialEq, Eq, Hash, Deserialize, Expand, Serialize)]
pub enum PayTimes {
    Once,
    AnyNumberOfTimes,
    UpTo { times: u32 },
}

#[derive(Debug, Clone, PartialEq, Eq, Hash, Deserialize, Expand, Serialize)]
pub enum ManaUnit {
    Generic,
    Run { cost: ManaCost },
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, Deserialize, Expand, Serialize)]
pub enum SpecialAction {
    TurnFaceUp,
    PutCompanionIntoHand,
    Foretell,
    UnlockDoor,
}

#[derive(Debug, Clone, PartialEq, Eq, Hash, Deserialize, Expand, Serialize)]
pub enum CostNamed {
    Containing { symbol: ManaSymbol },
    OfKeyword { keyword: KeywordLabel },
    OfSpecialAction { action: SpecialAction },
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, Deserialize, Expand, Serialize)]
pub enum ColorFreedom {
    SameColor,
    EachColor,
    DistinctColors,
}

#[derive(Debug, Clone, PartialEq, Eq, Hash, Deserialize, Expand, Serialize)]
pub enum ManaMatch {
    AnyColor,
    AnyType,
    Of { color: ColorOrColorless },
}

#[derive(Debug, Clone, PartialEq, Eq, Hash, Deserialize, Expand, Serialize)]
pub enum LoyaltyCost {
    Up { amount: u32 },
    Down { amount: u32 },
    DownX,
    Zero,
}

/// A card subtype. Instants and sorceries share their spell types [CR#205.3k].
#[derive(Debug, Clone, PartialEq, Eq, Hash, Deserialize, Expand, Serialize)]
pub enum Subtype {
    Of { host: CardType, label: String },
    Spell { label: String },
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, Deserialize, Expand, Serialize)]
pub enum MarkerWord {
    Token,
    Emblem,
    Spell,
    Permanent,
    Ability,
}

/// A change of a quantity: up by, down by, or set to.
#[derive(Debug, Clone, PartialEq, Eq, Hash, Deserialize, Serialize)]
pub enum Delta<T> {
    Up { amount: T },
    Down { amount: T },
    Set { amount: T },
}

/// Where a number below zero reads as zero and where it stands [CR#107.1b]: an effect's result
/// is clamped unless it sets, doubles, or triples a life total or a power and toughness.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, Deserialize, Expand, Serialize)]
pub enum NumberRegime {
    Clamped,
    Signed,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, Deserialize, Expand, Serialize)]
pub enum Parity {
    Even,
    Odd,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, Deserialize, Expand, Serialize)]
pub enum Supertype {
    Legendary,
    Basic,
    Snow,
    World,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, Deserialize, Expand, Serialize)]
pub enum RoomHalf {
    Left,
    Right,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, Deserialize, Expand, Serialize)]
pub enum LockState {
    Locked,
    Unlocked,
}

/// Which endpoint of an attachment relation the predicate describes.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, Deserialize, Expand, Serialize)]
pub enum AttachmentSide {
    Host,
    Attachment,
}

/// The current side of a single double-faced permanent, independently of its Status.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, Deserialize, Expand, Serialize)]
pub enum CardFaceSide {
    Front,
    Back,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, Deserialize, Expand, Serialize)]
pub enum AttachWord {
    Enchanted,
    Equipped,
    Fortified,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, Deserialize, Expand, Serialize)]
pub enum OutcomeVerb {
    WinGame,
    LoseGame,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, Deserialize, Expand, Serialize)]
pub enum DefinedSlots {
    PowerAlone,
    ToughnessAlone,
    BothEach,
}

#[derive(Debug, Clone, PartialEq, Eq, Hash, Deserialize, Expand, Serialize)]
pub enum CounterKind {
    Boost {
        power: Delta<u32>,
        toughness: Delta<u32>,
    },
    Keyword {
        keyword: KeywordLabel,
    },
    Named {
        label: String,
    },
}

#[derive(Debug, Clone, PartialEq, Eq, Hash, Deserialize, Expand, Serialize)]
pub enum ProjAxis {
    Stat { stat: Stat },
    PlayerStat { stat: PlayerStat },
    Counter { kind: CounterKind },
    AnyCounter { kind: Kind },
}

/// A Saga's chapter, counted from I [CR#714.2a].
pub type ChapterNumber = u32;

/// Each status category always has exactly one of its two values [CR#110.5].
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, Deserialize, Expand, Serialize)]
pub enum StatusCat {
    Tap,
    Flip,
    Face,
    Phase,
}

/// The Idris `StatusVal : StatusCat → Type`, unindexed; `category` recovers the index.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, Deserialize, Expand, Serialize)]
pub enum Status {
    Tapped,
    Untapped,
    Flipped,
    Unflipped,
    FaceUp,
    FaceDown,
    PhasedIn,
    PhasedOut,
}

#[derive(Debug, Clone, PartialEq, Eq, Hash, Deserialize, Expand, Serialize)]
pub enum ColorSpec {
    Some { colors: Vec<Color> },
    Every,
}

/// Phases and steps [CR#500.1]; `firstStrikeCombatDamage` is the extra step of [CR#510.4].
///
/// Macroable because `plugins_v2`'s turn-part declarations register here: the
/// family's name coincides with this type, so a declaration of kind `TurnPart`
/// stands wherever a card writes one. The derive is what supplies the kind its
/// dispatch set, which the identity-macro exemption in `macro_ron`'s cycle
/// check reads (a declaration named `Upkeep` whose body is `Upkeep` is free
/// vocabulary, not a self-reference).
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, SupportsMacros)]
pub enum TurnPart {
    Turn,
    BeginningPhase,
    UntapStep,
    Upkeep,
    DrawStep,
    MainPhase,
    FirstMain,
    Combat,
    /// The combat phase's own first step [CR#506.1], which the phase around it is not.
    BeginningOfCombat,
    DeclareAttackers,
    DeclareBlockers,
    FirstStrikeCombatDamage,
    CombatDamage,
    EndOfCombat,
    PostcombatMain,
    /// The turn's fifth phase [CR#500.1], the one holding the end and cleanup steps.
    EndingPhase,
    EndStep,
    Cleanup,
}

#[derive(Debug, Clone, PartialEq, Eq, Hash, Deserialize, Expand, Serialize)]
pub enum RankPeriod {
    Within { lookback: Lookback },
    Each { part: TurnPart },
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, Deserialize, Expand, Serialize)]
pub enum PartQuant {
    The,
    Each,
}

/// The event at which a static choice is made.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, Deserialize, Expand, Serialize)]
pub enum ChoiceOccasion {
    Entry,
    Attachment,
}

impl<T: Expand> Expand for Delta<T> {
    fn expand_all(self) -> Self {
        match self {
            Delta::Up { amount } => Delta::Up {
                amount: amount.expand_all(),
            },
            Delta::Down { amount } => Delta::Down {
                amount: amount.expand_all(),
            },
            Delta::Set { amount } => Delta::Set {
                amount: amount.expand_all(),
            },
        }
    }
}

impl Status {
    /// The status category this value belongs to; each category always has
    /// exactly one of its two values [CR#110.5]. Lean `Status.category`.
    #[must_use]
    pub fn category(self) -> StatusCat {
        match self {
            Status::Tapped | Status::Untapped => StatusCat::Tap,
            Status::Flipped | Status::Unflipped => StatusCat::Flip,
            Status::FaceUp | Status::FaceDown => StatusCat::Face,
            Status::PhasedIn | Status::PhasedOut => StatusCat::Phase,
        }
    }
}
