//! Abilities and their parts: instructions, static specs, costs, tokens, and
//! the characteristics an object carries. Mirrors
//! `lean/Semantics/Abilities.lean`.

use macro_ron::Expand;
use macro_ron::SupportsMacros;
use serde::Deserialize;
use serde::Serialize;

use crate::events::CondMarking;
use crate::events::DamageKind;
use crate::events::Deeds;
use crate::events::ReplUse;
use crate::phrase::Amount;
use crate::phrase::Ballot;
use crate::phrase::CaptureInput;
use crate::phrase::ChoiceDomain;
use crate::phrase::ColorTerm;
use crate::phrase::Condition;
use crate::phrase::Door;
use crate::phrase::Duration;
use crate::phrase::Exposed;
use crate::phrase::FlipScope;
use crate::phrase::GameEvent;
use crate::phrase::IgnoredOutcomes;
use crate::phrase::NounPhrase;
use crate::phrase::Pile;
use crate::phrase::Predicate;
use crate::phrase::Quantity;
use crate::phrase::SearchScope;
use crate::phrase::Timing;
use crate::phrase::VisibleThing;
use crate::phrase::ZoneExpr;
use crate::triggers::CountBound;
use crate::triggers::JoinedHeader;
use crate::triggers::UsageLimit;
use crate::words::AbilityClass;
use crate::words::Arrangement;
use crate::words::AttachMove;
use crate::words::CardType;
use crate::words::ChoiceOccasion;
use crate::words::ChoiceRef;
use crate::words::ChoiceSort;
use crate::words::Color;
use crate::words::ColorFreedom;
use crate::words::ColorOrColorless;
use crate::words::ColorSpec;
use crate::words::CopySort;
use crate::words::CostNamed;
use crate::words::CounterKind;
use crate::words::Deed;
use crate::words::DefinedSlots;
use crate::words::Delta;
use crate::words::DesignationLabel;
use crate::words::DieSides;
use crate::words::Disclosure;
use crate::words::EntryCounterMark;
use crate::words::ExposeVerb;
use crate::words::HiddenSort;
use crate::words::ItalicWord;
use crate::words::KeywordLabel;
use crate::words::KeywordTerm;
use crate::words::KindAxis;
use crate::words::Letter;
use crate::words::LoyaltyCost;
use crate::words::ManaCost;
use crate::words::ManaMatch;
use crate::words::MergePosition;
use crate::words::Ordinal;
use crate::words::OutcomeVerb;
use crate::words::Parity;
use crate::words::PayTimes;
use crate::words::PileFace;
use crate::words::Procedure;
use crate::words::ProducedRun;
use crate::words::QualitySort;
use crate::words::RelationLabel;
use crate::words::Role;
use crate::words::RoundMode;
use crate::words::ScaleFactor;
use crate::words::ShiftDir;
use crate::words::Stat;
use crate::words::StateBasedCause;
use crate::words::Status;
use crate::words::Subtype;
use crate::words::SubtypeSpace;
use crate::words::Supertype;
use crate::words::TurnPart;

/// What mana may be spent on.
#[derive(Debug, Clone, PartialEq, Eq, Hash, Deserialize, Expand, Serialize)]
pub enum SpendPurpose {
    ToCast { predicate: Predicate },
    ToActivate { source: Option<Predicate> },
    ToPay { cost: CostNamed },
}

#[derive(Debug, Clone, PartialEq, Eq, Hash, Deserialize, Expand, Serialize)]
pub enum ManaHeld {
    ThisMana,
    Unspent { r#type: Option<ColorOrColorless> },
}

#[derive(Debug, Clone, PartialEq, Eq, Hash, Deserialize, Expand, Serialize)]
pub enum AsThough {
    Of {
        predicate: Predicate,
    },
    Mana {
        what: Option<ColorOrColorless>,
        r#as: ManaMatch,
        purpose: Option<SpendPurpose>,
    },
    Greater {
        stat: Stat,
        amount: Amount,
    },
}

#[derive(Debug, Clone, PartialEq, Eq, Hash, Deserialize, Expand, Serialize)]
pub enum Exchanged {
    LifeTotals {
        parties: NounPhrase,
    },
    ControlOf {
        left: NounPhrase,
        right: NounPhrase,
    },
    CardsAcross {
        left: NounPhrase,
        right: NounPhrase,
    },
    Zones {
        left: ZoneExpr,
        right: ZoneExpr,
    },
    /// Two numerical values [CR#701.12g]: life totals, powers or toughnesses,
    /// rolled results.
    Values {
        left: Amount,
        right: Amount,
    },
    /// Two permanents' text boxes.
    TextBoxes {
        left: NounPhrase,
        right: NounPhrase,
    },
}

#[derive(Debug, Clone, PartialEq, Eq, Hash, Deserialize, Expand, Serialize)]
pub enum TokenQuality {
    WithEveryType { space: SubtypeSpace },
    WithQuality { quality: Predicate },
}

/// Where a counter's kind comes from.
#[derive(Debug, Clone, PartialEq, Eq, Hash, SupportsMacros)]
pub enum CounterKindSource {
    /// Carries only the printed kind, so a bare `p1p1Counter` or
    /// `Named(label: "charge")` reads here (§11.1; ruling 2026-10-03).
    #[macro_ron(embed)]
    Printed {
        kind: CounterKind,
    },
    Chosen {
        menu: Vec<CounterKind>,
    },
    DistinctChosen {
        menu: Vec<CounterKind>,
    },
    Bound,
    Those,
    Own,
    SameAs {
        source: NounPhrase,
    },
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, Deserialize, Expand, Serialize)]
pub enum QualityOp {
    Adds,
    Sets,
    Loses,
}

/// Only present axes are written. A present empty list explicitly clears that
/// axis.
#[derive(Debug, Clone, PartialEq, Eq, Hash, Deserialize, Expand, Serialize)]
pub struct TypeLineChanges {
    #[serde(default)]
    pub supertypes: Option<Vec<Supertype>>,
    #[serde(default)]
    pub types: Option<Vec<CardType>>,
    #[serde(default)]
    pub subtypes: Option<Vec<Subtype>>,
    #[serde(default)]
    pub retained: Option<CardType>,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, Deserialize, Expand, Serialize)]
pub enum CharacteristicStat {
    Power,
    Toughness,
    Loyalty,
    Defense,
}

/// A mana cost a cost shift raises or lowers by: written ("{W}{B}"), or an
/// object's mana cost ("the sacrificed permanent's mana cost" [CR#702.48a]).
#[derive(Debug, Clone, PartialEq, Eq, Hash, SupportsMacros)]
pub enum ManaCostTerm {
    Lit { cost: ManaCost },
    ManaCostOf { object: NounPhrase },
}

#[derive(Debug, Clone, PartialEq, Eq, Hash, Deserialize, Expand, Serialize)]
pub enum CostShift {
    Less {
        amount: Amount,
        floor: Option<Amount>,
    },
    More {
        amount: Amount,
    },
    /// Raise (`rises`) or lower the cost by the symbols of `cost`.
    /// `colored_only` is "this effect reduces only the amount of colored mana
    /// you pay" (Edgewalker); without it, colored mana beyond what the total
    /// cost has reduces generic mana [CR#702.48c,118.7].
    BySymbols {
        cost: ManaCostTerm,
        rises: bool,
        colored_only: bool,
    },
}

/// The mana one act of a Mana Substitute pays for: one generic mana, or one
/// mana of a color [CR#702.51a,702.66a,702.126a,701.67a].
#[derive(Debug, Clone, PartialEq, Eq, Hash, SupportsMacros)]
pub enum PipPaid {
    Generic,
    OfColor { color: ColorTerm },
}

#[derive(Debug, Clone, PartialEq, Eq, Hash, Deserialize, Expand, Serialize)]
pub struct DeedComplement {
    pub deed: Deed,
    pub counterpart: NounPhrase,
}

/// Whom a deontic rule's deed is done to.
#[derive(Debug, Clone, PartialEq, Eq, Hash, Deserialize, Expand, Serialize)]
pub enum DeonticPatient {
    NoPatient,
    DefendingPlayer { patient: NounPhrase },
    Counterpart { patient: NounPhrase },
    TargetedBy { patient: NounPhrase },
    CounterpartsAt { complements: Vec<DeedComplement> },
}

#[derive(Debug, Clone, PartialEq, Eq, Hash, Deserialize, Expand, Serialize)]
pub enum DamageScope {
    Everywhere,
    ToRecipient { recipient: NounPhrase },
}

#[derive(Debug, Clone, PartialEq, Eq, Hash, Deserialize, Expand, Serialize)]
pub enum DamageAgent {
    Unattributed,
    DealtBy { source: NounPhrase },
}

#[derive(Debug, Clone, PartialEq, Eq, Hash, Deserialize, Expand, Serialize)]
pub enum Unpreventable {
    Described {
        source: DamageAgent,
        scope: DamageScope,
    },
    ThatDamage,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, Deserialize, Expand, Serialize)]
pub enum PreventionBan {
    NoPreventionOnly,
    NoRedirectEither,
}

#[derive(Debug, Clone, PartialEq, Eq, Hash, Deserialize, Expand, Serialize)]
pub enum PreventCut {
    All,
    Some { amount: Amount },
    Shield { amount: Amount },
    AllBut { amount: Amount },
    Half { rounding: RoundMode },
}

#[derive(Debug, Clone, PartialEq, Eq, Hash, Deserialize, Expand, Serialize)]
pub enum DamageScale {
    Multiplied { factor: ScaleFactor },
    Halved { rounding: RoundMode },
    Shifted { direction: ShiftDir, amount: Amount },
}

#[derive(Debug, Clone, PartialEq, Eq, Hash, Deserialize, Expand, Serialize)]
pub enum DividedVerb {
    Damage { source: NounPhrase },
    Counters { kind: CounterKind },
}

#[derive(Debug, Clone, PartialEq, Eq, Hash, Deserialize, Expand, Serialize)]
pub enum ProducedMana {
    Runs { runs: Vec<ProducedRun> },
    AnyColor { freedom: ColorFreedom },
    OfChosenColor { alternative: Option<ProducedRun> },
    AsPrintedCost { subject: NounPhrase },
    ProducedByEvent { subject: NounPhrase },
    CouldProduce { subject: NounPhrase },
    AmongColorsOf { subject: NounPhrase },
    AmongWritten { colors: Vec<Color> },
    LastNoted { subject: NounPhrase },
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, Deserialize, Expand, Serialize)]
pub enum SpentMode {
    AffectsIt,
    TriggersThen,
}

/// Optional payment is decided by its performer, the actor in context; required
/// payment tests whether it started [CR#118.12].
#[derive(Debug, Clone, PartialEq, Eq, Hash, Deserialize, Expand, Serialize)]
pub enum ContinuationPolicy {
    Optional,
    Required,
}

#[derive(Debug, Clone, PartialEq, Eq, Hash, Deserialize, Expand, Serialize)]
pub enum DeckTrait {
    ACharacteristic {
        predicate: Predicate,
    },
    /// "cards with even mana values" [CR#202.3]
    ManaValueParity {
        parity: Parity,
    },
    /// "more than one of the same mana symbol in its mana cost"
    RepeatedManaSymbol,
    /// "has an activated ability"
    HasAbilityOf {
        class: AbilityClass,
    },
    /// "... and land cards"
    AnyOf {
        traits: Vec<DeckTrait>,
    },
}

#[derive(Debug, Clone, PartialEq, Eq, Hash, Deserialize, Expand, Serialize)]
pub enum DeckCondition {
    /// "Each permanent card in your starting deck has mana value 2 or less."
    EveryCardIs {
        scope: Predicate,
        r#trait: DeckTrait,
    },
    /// "No card in your starting deck has more than one of the same mana symbol
    /// in its mana cost."
    NoCardIs {
        scope: Predicate,
        r#trait: DeckTrait,
    },
    /// "Each nonland card in your starting deck shares a card type." and "…
    /// has a different name.": an Agreement (`NounPhrase::Agree`) over the
    /// starting deck's cards `group` describes.
    GroupAgrees { group: NounPhrase },
    /// "at least twenty cards more than the minimum deck size", a minimum the
    /// format sets [CR#100.2a,100.2b].
    DeckSizeOverMinimum { extra: u32 },
}

/// Combat participation is independent of the set of blocking relations.
#[derive(Debug, Clone, PartialEq, Eq, Hash, Deserialize, Expand, Serialize)]
pub enum CombatParticipation {
    Attacking { defender: Option<NounPhrase> },
    Blocked { value: bool },
    OutsideCombat,
}

#[derive(Debug, Clone, PartialEq, Eq, Hash, Deserialize, Expand, Serialize)]
pub enum CombatUpdate {
    Participation {
        state: CombatParticipation,
    },
    /// Adding a blocker makes its attacker blocked; removing the relation
    /// leaves that blockedness unchanged.
    Blocking {
        r#move: AttachMove,
        attacker: NounPhrase,
    },
}

#[derive(Debug, Clone, PartialEq, Eq, Hash, Deserialize, Expand, Serialize)]
pub enum Repetition {
    Again,
    MoreTimes {
        times: Amount,
    },
    AnyNumber,
    UntilCond {
        condition: Condition,
    },
    AgainExcludingChosen,
    Fixed {
        times: Amount,
        body: Box<Instruction>,
    },
}

/// An object's characteristics: name, mana cost, color and color indicator,
/// card type, subtype, supertype, rules text and abilities, power, toughness,
/// loyalty, and defense [CR#109.3]. Every field defaults to absent.
#[derive(Debug, Clone, PartialEq, Eq, Hash, Deserialize, Expand, Serialize)]
pub struct Characteristics {
    #[serde(default)]
    pub name: Option<String>,
    #[serde(default)]
    pub cost: Option<ManaCost>,
    #[serde(default)]
    pub colors: Vec<Color>,
    #[serde(default)]
    pub supertypes: Vec<Supertype>,
    #[serde(default)]
    pub types: Vec<CardType>,
    #[serde(default)]
    pub subtypes: Vec<Subtype>,
    #[serde(default)]
    pub text: Vec<Ability>,
    #[serde(default)]
    pub power: Option<Amount>,
    #[serde(default)]
    pub toughness: Option<Amount>,
    #[serde(default)]
    pub loyalty: Option<Amount>,
    #[serde(default)]
    pub defense: Option<Amount>,
}

/// A characteristics set as an effect writes it, with the token qualities
/// ("with every creature type") an effect can add [CR#111.3].
#[derive(Debug, Clone, PartialEq, Eq, Hash, Deserialize, Expand, Serialize)]
pub struct CharacteristicBundle {
    pub characteristics: Box<Characteristics>,
    #[serde(default)]
    pub qualities: Vec<TokenQuality>,
}

/// The one face-down spec. `None` is a card exiled face down, which has no
/// characteristics and no status [CR#406.3a,110.5d]; `Some` is a face-down
/// spell or permanent, its listed characteristics and so its copiable values
/// [CR#708.2,613.2b]. Turning it face up is `SetStatus(FaceUp, …)`, and the
/// values revert [CR#708.8]. Nothing face down is a token.
#[derive(Debug, Clone, PartialEq, Eq, Hash, Deserialize, Expand, Serialize)]
pub struct FaceDown {
    pub characteristics: Option<Box<Characteristics>>,
}

/// Typed characteristic writes; the enclosing static or copy form retains its
/// context.
#[derive(Debug, Clone, PartialEq, Eq, Hash, Deserialize, Expand, Serialize)]
pub enum CharacteristicEdit {
    TypeLine {
        op: QualityOp,
        changes: TypeLineChanges,
    },
    Name {
        op: QualityOp,
        name: String,
    },
    ManaCost {
        cost: Option<ManaCost>,
    },
    Colors {
        op: QualityOp,
        colors: ColorSpec,
    },
    Stat {
        op: QualityOp,
        axis: CharacteristicStat,
        amount: Amount,
    },
    EveryTypeOf {
        op: QualityOp,
        space: SubtypeSpace,
    },
    ChosenQuality {
        op: QualityOp,
        quality: Predicate,
    },
    AddedAbilities {
        abilities: Vec<Ability>,
    },
    RemovedAbilities {
        selection: Box<AbilitySelection>,
    },
}

#[derive(Debug, Clone, PartialEq, Eq, Hash, Deserialize, Expand, Serialize)]
pub enum AbilitySelection {
    Specified { abilities: Vec<AbilityLost> },
    Family { class: AbilityClass },
    AllExcept { except: Option<Predicate> },
}

#[derive(Debug, Clone, PartialEq, Eq, Hash, SupportsMacros)]
pub enum TokenSpec {
    /// The creating spell or ability defines the token's characteristic values
    /// [CR#111.3] and sets its name and subtypes [CR#111.4].
    Written {
        characteristics: Box<CharacteristicBundle>,
    },
    AsThose,
    CopyOf {
        source: NounPhrase,
        exceptions: Vec<CopyExcept>,
    },
    /// A double-faced token: its front face and its back face
    /// [CR#111.10i,701.53b].
    DoubleFaced {
        front: Box<CharacteristicBundle>,
        back: Box<CharacteristicBundle>,
    },
}

#[derive(Debug, Clone, PartialEq, Eq, Hash, SupportsMacros)]
pub enum StaticSpec {
    WithBindings {
        scope: u32,
        inputs: Vec<CaptureInput>,
        body: Box<StaticSpec>,
    },
    InCaller {
        scope: u32,
        body: Box<StaticSpec>,
    },
    Modification {
        subject: NounPhrase,
        stat: Stat,
        delta: Delta<Amount>,
    },
    /// "<subject> gets +P/+T" and "<subject> has base power and toughness P/T":
    /// one change to power and toughness together, its subject written
    /// once. Each delta is the one a `Modification` of that stat would
    /// carry: a set applies in layer 7b, a change by an amount in layer 7c
    /// [CR#613.4b,613.4c].
    PtModification {
        subject: NounPhrase,
        power: Delta<Amount>,
        toughness: Delta<Amount>,
    },
    PtDefinition {
        subject: NounPhrase,
        slots: DefinedSlots,
        amount: Amount,
    },
    PtSwitch {
        subject: NounPhrase,
    },
    CostShift {
        subject: NounPhrase,
        shift: CostShift,
    },
    AltCost {
        subject: NounPhrase,
        cost: Option<Box<Cost>>,
    },
    AddedCost {
        cost: Box<Cost>,
        offered: bool,
        times: PayTimes,
    },
    LetterDefinition {
        letter: Letter,
        amount: Amount,
    },
    AbilityGrant {
        subject: NounPhrase,
        ability: Box<Ability>,
    },
    AbilityGrantFrom {
        subject: NounPhrase,
        classes: Vec<AbilityClass>,
        source: NounPhrase,
        except: Option<Predicate>,
    },
    DeonticRule {
        subject: NounPhrase,
        compulsion: Box<Compulsion>,
        deeds: Deeds,
        role: Role,
        bound: Option<CountBound>,
        patient: DeonticPatient,
        as_though: Option<AsThough>,
        rider: Box<DeonticRider>,
    },
    ManaRetention {
        player: NounPhrase,
        mana: ManaHeld,
    },
    PartSkip {
        player: NounPhrase,
        part: TurnPart,
    },
    CharacteristicChange {
        subject: NounPhrase,
        edits: Vec<CharacteristicEdit>,
    },
    Retention {
        spec: Box<StaticSpec>,
        subject: NounPhrase,
    },
    CopyChange {
        subject: NounPhrase,
        source: NounPhrase,
        exceptions: Vec<CopyExcept>,
    },
    ControlGrant {
        player: NounPhrase,
        subject: NounPhrase,
    },
    Replacement {
        event: GameEvent,
        alternatives: Vec<GameEvent>,
        timing: Option<Timing>,
        replacement: Box<Instruction>,
        r#use: ReplUse,
        limit: Option<UsageLimit>,
    },
    DamageRule {
        kind: DamageKind,
        source: DamageAgent,
        scope: DamageScope,
        op: Box<DamageOp>,
        r#use: ReplUse,
    },
    PreventionBan {
        kind: DamageKind,
        damage: Unpreventable,
        ban: PreventionBan,
    },
    Conditional {
        spec: Box<StaticSpec>,
        condition: Condition,
        marking: CondMarking,
    },
    Visibility {
        verb: ExposeVerb,
        player: NounPhrase,
        what: VisibleThing,
    },
    AdditionalTriggers {
        event: GameEvent,
        quantity: Quantity,
    },
    EntryRider {
        subject: NounPhrase,
        rider: Box<EntryRider>,
    },
    Choice {
        occasion: ChoiceOccasion,
        subject: NounPhrase,
        sort: ChoiceSort,
        domain: Option<ChoiceDomain>,
        disclosure: Disclosure,
    },
    Conjunction {
        subject: Option<NounPhrase>,
        parts: Vec<StaticSpec>,
    },
    /// A Mana Substitute on a spell (convoke, delve, improvise): neither an
    /// additional nor an alternative cost, applied once the spell's total cost
    /// is locked [CR#702.51b,702.66b,702.126b].
    PaySubstitute {
        subject: NounPhrase,
        substitute: ManaSubstitute,
    },
    /// A special action: any time you have priority, within `when`, you may
    /// pay `cost` and do `does`, without using the stack [CR#116.1,116.2c].
    /// Morph's turning face up, foretell's and plot's exiling are each one
    /// [CR#116.2b,116.2h,116.2k].
    SpecialAction {
        when: Option<Timing>,
        cost: Box<Cost>,
        does: Box<Instruction>,
    },
    /// "<subject> is a Combatant": the creature-like combat rules apply to it
    /// [CR#120.3e,302.6,508.1a,509.1a,701.14a]. A quality of a permanent, not
    /// an ability [CR#113.12]; the creature type definition confers it.
    Combatant {
        subject: NounPhrase,
    },
    /// A Mode Change: modifies how the modes of the spell or ability it is
    /// printed with are chosen [CR#700.2,702.42a,702.120a].
    ModeChoice {
        change: ModeChange,
    },
    /// The spell gains a target, `description` saying what it may target,
    /// when its alternative cost is chosen: mutate's "targets a non-Human
    /// creature with the same owner as this spell" [CR#702.140a,601.2c].
    AddsTarget {
        subject: NounPhrase,
        description: Predicate,
    },
}

/// How a Mode Change modifies its own spell's `chooseModes`.
#[derive(Debug, Clone, PartialEq, Eq, Hash, SupportsMacros)]
pub enum ModeChange {
    /// "You may choose all modes instead": for `cost` (entwine
    /// [CR#702.42a]), or when `condition` holds ("if COND, you may choose
    /// both instead").
    AllInstead {
        cost: Option<Cost>,
        condition: Option<Condition>,
    },
    /// "For each mode you choose beyond the first, you pay an additional
    /// [cost]" (escalate [CR#702.120a]).
    ExtraModesCost { cost: Cost },
    /// The number of modes chosen: spree's one or more, tiered's one
    /// [CR#702.172a,702.183a].
    SetQuantity { quantity: Quantity },
    /// "You may choose the same mode more than once" [CR#700.2d].
    Repeatable,
    /// "Choose one that hasn't been chosen [within]".
    NotChosenBefore { within: Duration },
}

#[derive(Debug, Clone, PartialEq, Eq, Hash, Deserialize, Expand, Serialize)]
pub enum Compulsion {
    Forbid,
    Require,
    GatedBy { cost: Box<Cost> },
    Permit,
}

#[derive(Debug, Clone, PartialEq, Eq, Hash, Deserialize, Expand, Serialize)]
pub enum PlayPayment {
    ItsOwnCost,
    WithoutPaying,
    PayingInstead {
        cost: Box<Cost>,
    },
    /// Cast face down as `spec` says, paying `cost` rather than the mana
    /// cost: morph and disguise [CR#702.37a,702.168a,708.4].
    FaceDown {
        spec: FaceDown,
        cost: Box<Cost>,
    },
}

#[derive(Debug, Clone, PartialEq, Eq, Hash, Deserialize, Expand, Serialize)]
pub enum DeonticRider {
    StateBased {
        cause: StateBasedCause,
    },
    NoRider,
    Play {
        from: Option<ZoneExpr>,
        limit: Option<UsageLimit>,
        timing: Option<Timing>,
        exclusive: bool,
        payment: Box<PlayPayment>,
    },
}

#[derive(Debug, Clone, PartialEq, Eq, Hash, Deserialize, Expand, Serialize)]
pub enum DamageOp {
    Prevent {
        cut: PreventCut,
        also: Option<Box<Instruction>>,
    },
    Redirect {
        cut: PreventCut,
        to: NounPhrase,
    },
    Scale {
        scale: DamageScale,
    },
}

/// How a permanent arrives on the battlefield: from a move, a token's
/// creation, or a static entry rider.
#[derive(Debug, Clone, PartialEq, Eq, Hash, SupportsMacros)]
pub enum EntryRider {
    EntersAs {
        status: Status,
    },
    EntersAttacking {
        defender: Option<NounPhrase>,
    },
    EntersTransformed,
    EntersMelded {
        into: String,
    },
    WithCounters {
        amount: Amount,
        kind: CounterKindSource,
        mark: EntryCounterMark,
    },
    Under {
        controller: NounPhrase,
    },
    AsCopyOf {
        optional: bool,
        source: NounPhrase,
        exceptions: Vec<CopyExcept>,
    },
    /// Enters face down as `spec` lists, and with no other characteristics
    /// while it stays face down [CR#708.2,708.3].
    FaceDown {
        spec: FaceDown,
    },
}

/// How a moved card is placed in a zone other than the battlefield: its order
/// and depth in a pile, its face, and the counters it is exiled with.
#[derive(Debug, Clone, PartialEq, Eq, Hash, Deserialize, Expand, Serialize)]
pub enum PilePlacement {
    /// "In any order", "in a random order": how the objects moved to one
    /// place are ordered there [CR#401.4].
    InOrder { arrangement: Arrangement },
    /// "Nth from the top": that many objects in from the place's end
    /// [CR#401.7].
    NthFromEnd { ordinal: Ordinal },
    /// "On the top or bottom": the performer may choose the place's other end
    /// instead.
    OrOtherEnd,
    /// "Exile it face down": `None`, a card with no characteristics
    /// [CR#406.3,406.3a].
    FaceDown { spec: FaceDown },
    /// "Exile it with N time counters on it" [CR#702.62a].
    WithCounters {
        amount: Amount,
        kind: CounterKindSource,
        mark: EntryCounterMark,
    },
}

/// A move's rider, of the kind its destination allows: an entry rider onto
/// the battlefield, a placement anywhere else (owner ruling 2026-10-08).
#[derive(Debug, Clone, PartialEq, Eq, Hash, SupportsMacros)]
pub enum MoveRider {
    /// Carries only the entry rider, so a bare `EntersAs(Tapped)` reads at a
    /// move rider's place.
    #[macro_ron(embed)]
    Entry {
        rider: EntryRider,
    },
    Placement {
        placement: PilePlacement,
    },
}

#[derive(Debug, Clone, PartialEq, Eq, Hash, SupportsMacros)]
pub enum Cost {
    WithBindings {
        scope: u32,
        inputs: Vec<CaptureInput>,
        body: Box<Cost>,
    },
    InCaller {
        scope: u32,
        body: Box<Cost>,
    },
    Mana {
        cost: ManaCost,
    },
    Scaled {
        cost: Box<Cost>,
        amount: Amount,
    },
    TapSymbol,
    UntapSymbol,
    LoyaltySymbol {
        loyalty: LoyaltyCost,
    },
    /// An instruction performed as a cost ("Sacrifice a creature:").
    Perform {
        instruction: Box<Instruction>,
    },
    Compound {
        costs: Vec<Cost>,
    },
    Or {
        costs: Vec<Cost>,
    },
    ItsManaCost,
    /// Pay `cost`, `substitute` standing in only for that cost's own generic
    /// mana (waterbend [CR#701.67b]).
    WithSubstitute {
        cost: Box<Cost>,
        substitute: Box<ManaSubstitute>,
    },
}

/// "For each [generic | colored] mana in the cost, you may <action> rather
/// than pay that mana" [CR#702.51a,702.66a,702.126a,701.67a]: the action
/// first, then the mana each act pays, read back from the acted-on object by
/// anaphora ("that creature's color").
#[derive(Debug, Clone, PartialEq, Eq, Hash, Deserialize, Expand, Serialize)]
pub struct ManaSubstitute {
    pub action: Box<Instruction>,
    pub pays: Vec<PipPaid>,
}

#[derive(Debug, Clone, PartialEq, Eq, Hash, Deserialize, Expand, Serialize)]
pub enum ManaRider {
    SpendOnly {
        purposes: Vec<SpendPurpose>,
    },
    SpendNotOn {
        purposes: Vec<SpendPurpose>,
    },
    OnSpent {
        mode: SpentMode,
        only: bool,
        spell: NounPhrase,
        says: Box<Instruction>,
    },
}

#[derive(Debug, Clone, PartialEq, Eq, Hash, Deserialize, Expand, Serialize)]
pub enum CopyExcept {
    Edits {
        edits: Vec<CharacteristicEdit>,
    },
    Ability {
        ability: Box<Ability>,
    },
    ThisAbility,
    EntersWithCounters {
        amount: Amount,
        kind: CounterKind,
        mark: EntryCounterMark,
    },
}

#[derive(Debug, Clone, PartialEq, Eq, Hash, Deserialize, Expand, Serialize)]
pub struct RollRow {
    pub results: Quantity,
    pub instruction: Box<Instruction>,
}

#[derive(Debug, Clone, PartialEq, Eq, Hash, Deserialize, Expand, Serialize)]
pub enum CreationSpec {
    Token {
        spec: Box<TokenSpec>,
        riders: Vec<EntryRider>,
    },
    /// An emblem, named where the rules name it ("an emblem named The Ring"
    /// [CR#701.54c]).
    Emblem {
        name: Option<String>,
        abilities: Vec<Ability>,
    },
}

#[derive(Debug, Clone, PartialEq, Eq, Hash, SupportsMacros)]
pub enum Instruction {
    /// Bind ordered typed inputs for the body, without an implicit whole-body
    /// precondition.
    WithBindings {
        scope: u32,
        inputs: Vec<CaptureInput>,
        body: Box<Instruction>,
    },
    InCaller {
        scope: u32,
        body: Box<Instruction>,
    },
    /// A keyword action body read in its own Reference Scope: the performer,
    /// its parameters (each `InCaller { scope, .. }`) and what it introduces,
    /// never the calling text's other mentions. Written by the loader inside a
    /// keyword action's `Enact` wrapper (`crate::keywords`).
    OwnScope {
        scope: u32,
        body: Box<Instruction>,
    },
    DealDamage {
        source: NounPhrase,
        amount: Amount,
        recipient: NounPhrase,
    },
    SetStatus {
        status: Status,
        subject: NounPhrase,
    },
    TurnOver {
        subject: NounPhrase,
    },
    Combat {
        subject: NounPhrase,
        update: CombatUpdate,
    },
    Attachment {
        r#move: AttachMove,
        subject: NounPhrase,
        host: Option<NounPhrase>,
    },
    /// Remove all damage marked on the permanent [CR#120.6].
    ClearDamage {
        subject: NounPhrase,
    },
    /// "… can't be regenerated this turn": an instruction plus the deed it
    /// forbids.
    DoAndForbid {
        instruction: Box<Instruction>,
        deed: Deed,
        subject: NounPhrase,
    },
    GainDesignation {
        subject: NounPhrase,
        designation: DesignationLabel,
        duration: Option<Duration>,
    },
    Unlock {
        door: Door,
    },
    SetGameDesignation {
        designation: DesignationLabel,
    },
    Conclude {
        verb: OutcomeVerb,
    },
    DrawGame,
    RestartGame,
    SeparateIntoPiles {
        group: NounPhrase,
        piles: u32,
        faces: Vec<PileFace>,
    },
    /// The `when` rider is the printed "as you activate this ability": the
    /// announcement's timing, not a condition on what may be chosen.
    Choose {
        first: Option<NounPhrase>,
        chosen: NounPhrase,
        disclosure: Disclosure,
        when: Option<Timing>,
    },
    RevealChoices {
        sort: HiddenSort,
    },
    Vote {
        first: Option<NounPhrase>,
        disclosure: Disclosure,
        ballot: Ballot,
    },
    Move {
        subject: NounPhrase,
        from: ZoneExpr,
        to: ZoneExpr,
        riders: Vec<MoveRider>,
    },
    /// Turn `subject` face down as `spec` lists: manifest's first step
    /// [CR#701.40a,708.2].
    TurnFaceDown {
        subject: NounPhrase,
        spec: FaceDown,
    },
    Copy {
        sort: CopySort,
        subject: NounPhrase,
        times: Amount,
        exceptions: Vec<CopyExcept>,
    },
    ChooseNewTargets {
        subject: NounPhrase,
    },
    CopyTargets {
        copy: NounPhrase,
        original: NounPhrase,
    },
    ChangeLife {
        delta: Delta<Amount>,
    },
    Exchange {
        exchanged: Exchanged,
    },
    AddMana {
        amount: Amount,
        produced: ProducedMana,
        riders: Vec<ManaRider>,
    },
    Draw {
        amount: Amount,
    },
    Expose {
        verb: ExposeVerb,
        exposed: Exposed,
    },
    Search {
        scope: SearchScope,
        quantity: Quantity,
        predicate: Predicate,
    },
    /// Randomize the objects in a pile so no player knows their order
    /// [CR#701.24a].
    Shuffle {
        pile: Pile,
    },
    FlipCoins {
        count: FlipScope,
    },
    RollDice {
        count: Amount,
        sides: DieSides,
    },
    ApplyResultsTable {
        rows: Vec<RollRow>,
    },
    IgnoreOutcomes {
        which: IgnoredOutcomes,
    },
    ShiftResult {
        direction: Option<ShiftDir>,
        amount: Amount,
    },
    StoreResults {
        on: NounPhrase,
    },
    RerollStored {
        quantity: Quantity,
        whose: NounPhrase,
    },
    Establish {
        spec: Box<StaticSpec>,
        duration: Option<Duration>,
    },
    CreateObject {
        count: Amount,
        spec: Box<CreationSpec>,
    },
    PutCounters {
        amount: Amount,
        kind: CounterKindSource,
        on: NounPhrase,
    },
    Distribute {
        verb: DividedVerb,
        amount: Amount,
        among: NounPhrase,
    },
    RemoveCounters {
        quantity: Option<Quantity>,
        kind: Option<CounterKindSource>,
        from: NounPhrase,
    },
    MoveCounters {
        amount: Amount,
        kind: Option<CounterKindSource>,
        source: NounPhrase,
        destination: NounPhrase,
    },
    DoubleCounters {
        on: NounPhrase,
    },
    /// A named deed done by the performer in context: the deed's own facts row
    /// gates that performer, the patient and the zones the sentence may
    /// name, whichever of the three sources defines it.
    Enact {
        verb: Deed,
        instruction: Box<Instruction>,
    },
    Pay {
        cost: Box<Cost>,
        times: PayTimes,
    },
    /// The handoff: `performer`, a player or a permanent, performs `body`
    /// ("target player discards a card", "target creature explores"
    /// [CR#701.44a]). Inside the body, `NounPhrase.actor` is that performer;
    /// with a distributive performer ("each opponent") each member performs
    /// the whole body.
    Act {
        performer: NounPhrase,
        body: Box<Instruction>,
    },
    /// Branch on the decision or start of payment, independently of resulting
    /// events.
    WithContinuation {
        policy: ContinuationPolicy,
        body: Box<Instruction>,
        if_did: Option<Box<Instruction>>,
        if_not: Option<Box<Instruction>>,
    },
    DoOnlyIf {
        instruction: Box<Instruction>,
        condition: Condition,
        otherwise: Option<Box<Instruction>>,
    },
    /// "<instruction>. Do this only once each turn.": the instruction is
    /// limited, not the ability that carries it [CR#603.2h].
    Limited {
        instruction: Box<Instruction>,
        limit: UsageLimit,
    },
    DoIf {
        condition: Condition,
        instruction: Box<Instruction>,
        otherwise: Option<Box<Instruction>>,
    },
    DoForEach {
        group: NounPhrase,
        body: Box<Instruction>,
    },
    DoForEachKind {
        axis: KindAxis,
        domain: Option<NounPhrase>,
        sort: QualitySort,
        body: Box<Instruction>,
    },
    Repeat {
        repetition: Box<Repetition>,
    },
    Sequentially {
        steps: Vec<Instruction>,
    },
    Simultaneously {
        steps: Vec<Instruction>,
    },
    ChooseModes {
        quantity: Quantity,
        /// Each mode's cost [CR#700.2h], weight [CR#700.2i] and instruction.
        modes: Vec<(Option<Cost>, Option<u32>, Instruction)>,
    },
    Delay {
        event: GameEvent,
        alternatives: Vec<GameEvent>,
        duration: Option<Duration>,
        body: Box<Instruction>,
    },
    Replace {
        replaced: Box<Instruction>,
        replacement: Box<Instruction>,
    },
    HoldUntil {
        instruction: Box<Instruction>,
        event: GameEvent,
    },
    TriggerReflexively {
        body: Box<Instruction>,
        trigger: Box<Instruction>,
    },
    TriggerThisWay {
        body: Box<Instruction>,
        event: GameEvent,
        trigger: Box<Instruction>,
    },
    SkipUntap {
        subject: NounPhrase,
        steps: Amount,
    },
    SkipPart {
        part: TurnPart,
        count: Amount,
    },
    AddTurnPart {
        part: TurnPart,
        anchor: Option<TurnPart>,
        count: Amount,
        followed_by: Option<TurnPart>,
    },
    /// Carry out a rules procedure on `subject` now, during resolution:
    /// activate it, cast it, or play it [CR#701.2a,701.5a,701.18a], paying as
    /// `payment` says. `resulting` is what the spell the procedure produces
    /// must be ("if the resulting spell's mana value is N or less"
    /// [CR#701.57a]).
    Procedure {
        kind: Procedure,
        subject: NounPhrase,
        payment: PlayPayment,
        resulting: Option<Predicate>,
    },
    /// Make `subject` stand in the Relation `label` with `counterpart`
    /// [CR#702.55b,702.95b,702.99b].
    Relate {
        subject: NounPhrase,
        label: RelationLabel,
        counterpart: NounPhrase,
    },
    /// Merge `subject` with the permanent `onto`, placed at `position`
    /// [CR#730.2].
    Merge {
        subject: NounPhrase,
        onto: NounPhrase,
        position: MergePosition,
    },
}

#[derive(Debug, Clone, PartialEq, Eq, Hash, Deserialize, Expand, Serialize)]
pub enum KeywordParam {
    Cost { cost: Box<Cost> },
    Quality { predicate: Predicate },
    Subject { predicate: Predicate },
    Number { amount: Amount },
    DeckCondition { condition: DeckCondition },
}

#[derive(Debug, Clone, PartialEq, Eq, Hash, Deserialize, Expand, Serialize)]
pub enum AbilityLost {
    Written { ability: Box<Ability> },
    Term { keyword: KeywordTerm },
}

#[derive(Debug, Clone, PartialEq, Eq, Hash, SupportsMacros)]
pub enum Ability {
    Keyword {
        keyword: KeywordLabel,
        params: Vec<KeywordParam>,
        body: Vec<Ability>,
    },
    Activated {
        cost: Box<Cost>,
        instruction: Box<Instruction>,
        timing: Option<Timing>,
        limit: Option<UsageLimit>,
        guard: Option<Condition>,
        activator: Option<NounPhrase>,
    },
    Triggered {
        event: GameEvent,
        alternatives: Vec<GameEvent>,
        r#while: Option<Timing>,
        joins: Vec<JoinedHeader>,
        timing: Option<Timing>,
        limit: Option<UsageLimit>,
        intervening: Option<Condition>,
        instruction: Box<Instruction>,
    },
    Static {
        spec: Box<StaticSpec>,
    },
    Spell {
        timing: Option<Timing>,
        instruction: Box<Instruction>,
    },
    /// "You may begin the game with this on the battlefield" (the Leylines).
    MayBeginOnBattlefield,
    /// "The same is true for first strike, double strike, …" (Odric).
    AlsoForKeywords {
        ability: Box<Ability>,
        keywords: Vec<KeywordTerm>,
    },
    /// An italic head before an ability: an ability word or a flavor word.
    ItalicHead {
        word: ItalicWord,
        ability: Box<Ability>,
    },
    /// "that ability", read off an ability chosen earlier in the same text.
    ThatAbility {
        r#ref: ChoiceRef,
    },
}
