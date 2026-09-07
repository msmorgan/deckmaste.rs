//! The phrase grammar: zones, predicates, noun phrases, amounts, quantities,
//! and conditions. Mirrors `lean/Semantics/Phrase.lean`; its `mutual` block
//! becomes one recursive family here, indirected with `Box` where Rust needs a
//! known size.

use crate::events::ChoiceMode;
use crate::events::CounterBatch;
use crate::events::CounterMove;
use crate::events::DamageKind;
use crate::events::DiceBatch;
use crate::events::LifeMove;
use crate::events::PaymentOutcome;
use crate::events::TallyOp;
use crate::words::AbilityClass;
use crate::words::AggregateOp;
use crate::words::AmountShape;
use crate::words::ArithOp;
use crate::words::Arrangement;
use crate::words::AttachMove;
use crate::words::AttachWord;
use crate::words::AttachmentSide;
use crate::words::CardFaceSide;
use crate::words::CardType;
use crate::words::ChapterNumber;
use crate::words::ChoiceRef;
use crate::words::ChoiceSort;
use crate::words::CoinFace;
use crate::words::Color;
use crate::words::CombatRelation;
use crate::words::CombatRole;
use crate::words::Comparator;
use crate::words::CounterKind;
use crate::words::Deed;
use crate::words::DesignationLabel;
use crate::words::FlipCall;
use crate::words::KeywordLabel;
use crate::words::KeywordTerm;
use crate::words::Kind;
use crate::words::KindAxis;
use crate::words::Letter;
use crate::words::LibraryEnd;
use crate::words::LockState;
use crate::words::Lookback;
use crate::words::ManaSymbol;
use crate::words::MarkerWord;
use crate::words::NameAgreement;
use crate::words::NounWord;
use crate::words::Ordinal;
use crate::words::OutcomeSort;
use crate::words::PaidFacet;
use crate::words::PartQuant;
use crate::words::PlayerGroupWord;
use crate::words::Plurality;
use crate::words::PossessorAxis;
use crate::words::ProjAxis;
use crate::words::QualitySort;
use crate::words::RankPeriod;
use crate::words::Reach;
use crate::words::RollExtreme;
use crate::words::RoundMode;
use crate::words::Stat;
use crate::words::Status;
use crate::words::Subtype;
use crate::words::Supertype;
use crate::words::TargetExtent;
use crate::words::TurnPart;
use crate::words::VoteLabel;
use crate::words::Window;
use crate::words::Zone;
use macro_ron::Expand;
use macro_ron::SupportsMacros;
use serde::Deserialize;
use serde::Serialize;

/// A color, written or "the chosen color".
#[derive(Debug, Clone, PartialEq, Eq, Hash, SupportsMacros)]
pub enum ColorTerm {
    /// Carries only the written colour, so a bare `Green` reads here
    /// [CR#105.1].
    #[macro_ron(embed)]
    Lit {
        color: Color,
    },
    Chosen {
        r#ref: ChoiceRef,
    },
}

/// The mana type a "tapped for mana of …" trigger specifies [CR#106.12a]: colorless, or a
/// color written or chosen; the six types of [CR#106.1b].
#[derive(Debug, Clone, PartialEq, Eq, Hash, Deserialize, Expand, Serialize)]
pub enum ManaTypeTerm {
    Colorless,
    OfColor { color: ColorTerm },
}

/// Which version of a zone-changing object supplies the event pattern's characteristics
/// [CR#603.10,603.10a].
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, Deserialize, Expand, Serialize)]
pub enum ObservationPoint {
    Before,
    After,
}

/// Ordered inputs to a semantic binding scope; each input is read once.
#[derive(Debug, Clone, PartialEq, Eq, Hash, Deserialize, Expand, Serialize)]
pub enum CaptureInput {
    Subject { expression: Box<NounPhrase> },
    Value { expression: Box<Amount> },
}

/// Whose zone: "your graveyard", or a bare zone name.
#[derive(Debug, Clone, PartialEq, Eq, Hash, Deserialize, Expand, Serialize)]
pub enum ZoneScope {
    Bare,
    PossessedBy { possessor: Box<NounPhrase> },
}

/// Where in a library.
#[derive(Debug, Clone, PartialEq, Eq, Hash, Deserialize, Expand, Serialize)]
pub enum LibraryPlace {
    OneEnd { end: LibraryEnd },
    EitherEnd { chooser: Option<Box<NounPhrase>> },
    Shuffled,
}

#[derive(Debug, Clone, PartialEq, Eq, Hash, SupportsMacros)]
pub enum ZoneExpr {
    WithBindings {
        scope: u32,
        inputs: Vec<CaptureInput>,
        body: Box<ZoneExpr>,
    },
    InCaller {
        scope: u32,
        body: Box<ZoneExpr>,
    },
    Zone {
        zone: Zone,
        scope: Box<ZoneScope>,
    },
    Library {
        place: Box<LibraryPlace>,
        order: Option<Arrangement>,
        offset: Option<Ordinal>,
        scope: Box<ZoneScope>,
    },
}

/// Where a name comes from: printed, chosen, or "with the same name as …".
#[derive(Debug, Clone, PartialEq, Eq, Hash, Deserialize, Expand, Serialize)]
pub enum NameSource {
    Printed { name: String },
    Chosen,
    SameAs { subject: Box<NounPhrase> },
}

/// What a choice ranges over.
#[derive(Debug, Clone, PartialEq, Eq, Hash, Deserialize, Expand, Serialize)]
pub enum ChoiceDomain {
    NameOfCard {
        predicate: Box<Predicate>,
    },
    ColorOtherThan {
        color: Color,
    },
    TypeOtherThan {
        subtype: Subtype,
    },
    BasicTypesOnly,
    NonbasicTypesOnly,
    Number {
        quantity: Box<Quantity>,
    },
    Players {
        predicate: Box<Predicate>,
    },
    /// "choose flying or trample": an ability chosen among named keywords.
    AbilitiesAmong {
        keywords: Vec<KeywordTerm>,
    },
}

#[derive(Debug, Clone, PartialEq, Eq, Hash, Deserialize, Expand, Serialize)]
pub enum EventSource {
    Anywhere,
    Zones { zones: Vec<ZoneExpr> },
    AnywhereBut { zones: Vec<ZoneExpr> },
}

/// A historical event whose gap denotes the entity being described.
#[derive(Debug, Clone, PartialEq, Eq, Hash, Deserialize, Expand, Serialize)]
pub enum LookbackClause {
    Mk {
        event: Box<GameEvent>,
        lookback: Lookback,
    },
}

#[derive(Debug, Clone, PartialEq, Eq, Hash, SupportsMacros)]
pub enum Predicate {
    WithBindings {
        scope: u32,
        inputs: Vec<CaptureInput>,
        body: Box<Predicate>,
    },
    InCaller {
        scope: u32,
        body: Box<Predicate>,
    },
    HasType {
        r#type: CardType,
    },
    HasSubtype {
        subtype: Subtype,
    },
    HasSupertype {
        supertype: Supertype,
    },
    AnyPlayer,
    Opponent,
    ChosenPlayer {
        r#ref: ChoiceRef,
    },
    QualityNoun {
        sort: QualitySort,
        domain: Option<Box<ChoiceDomain>>,
    },
    CounterKindOn {
        subject: Box<NounPhrase>,
    },
    OfChosen {
        r#ref: ChoiceRef,
        sort: QualitySort,
    },
    OfYourChoice {
        sort: QualitySort,
        domain: Option<Box<ChoiceDomain>>,
    },
    HasKeyword {
        keyword: KeywordTerm,
    },
    HasPossessor {
        axis: PossessorAxis,
        possessor: Box<NounPhrase>,
    },
    CastBy {
        caster: Box<NounPhrase>,
        rank: Option<(Ordinal, RankPeriod)>,
    },
    CastFrom {
        zone: Box<ZoneExpr>,
    },
    WasCast,
    InCombat {
        relation: CombatRelation,
        counterpart: Option<Box<NounPhrase>>,
    },
    HappenedTo {
        lookback: Box<LookbackClause>,
    },
    ColorIs {
        color: Color,
    },
    ColorCount {
        comparator: Comparator,
        count: u32,
    },
    Named {
        source: Box<NameSource>,
    },
    HasDesignation {
        designation: DesignationLabel,
        holder: Option<Box<NounPhrase>>,
    },
    Attachment {
        side: AttachmentSide,
        word: Option<AttachWord>,
        counterpart: Option<Box<NounPhrase>>,
    },
    IsCard,
    IsToken,
    IsEmblem,
    IsCopyOfACard,
    /// The whole single double-faced permanent has this side up [CR#701.27g].
    CurrentFace {
        side: CardFaceSide,
    },
    HasStatus {
        status: Status,
    },
    HasCounters {
        kind: Option<CounterKind>,
    },
    Compare {
        axes: Vec<ProjAxis>,
        comparator: Comparator,
        bound: Box<Amount>,
    },
    Superlative {
        op: AggregateOp,
        axis: ProjAxis,
        domain: Box<Predicate>,
    },
    /// A vote stands only where a spell or ability instructed players to vote [CR#701.38a].
    WithMostVotes,
    ChoseExtreme {
        op: AggregateOp,
    },
    CompareOver {
        domain: Box<Predicate>,
        measure: Box<Amount>,
        comparator: Comparator,
        bound: Box<Amount>,
    },
    InZone {
        zone: Box<ZoneExpr>,
    },
    InPile {
        pile: Box<NounPhrase>,
    },
    ExiledWith {
        source: Box<NounPhrase>,
    },
    And {
        conjuncts: Vec<Predicate>,
    },
    /// Disjuncts of different kinds join: "creature or player" denotes either.
    Or {
        disjuncts: Vec<Predicate>,
    },
    Not {
        predicate: Box<Predicate>,
    },
    Other,
    NotChosen,
    OtherThan {
        anchor: Box<NounPhrase>,
    },
    CoinCameUp {
        face: CoinFace,
    },
    IsSource,
    ManaCostHas {
        symbol: ManaSymbol,
    },
    AbilityHead {
        class: AbilityClass,
    },
    AbilityOf {
        source: Box<NounPhrase>,
    },
    ActivatedBy {
        activator: Box<NounPhrase>,
    },
    IsManaAbility,
    Targets {
        subject: Box<NounPhrase>,
        extent: TargetExtent,
    },
}

#[derive(Debug, Clone, PartialEq, Eq, Hash, Deserialize, Expand, Serialize)]
pub enum DetPhrase {
    Target {
        quantity: Box<Quantity>,
    },
    A {
        mode: ChoiceMode,
    },
    /// "each …": a group, resolution-time [CR#608.2]
    Each,
    All,
    The,
    Count {
        quantity: Box<Quantity>,
        mode: Option<ChoiceMode>,
    },
    /// the bare plural: a description, no determiner [CR#109.2]
    Bare,
}

#[derive(Debug, Clone, PartialEq, Eq, Hash, SupportsMacros)]
pub enum NounPhrase {
    WithBindings {
        scope: u32,
        inputs: Vec<CaptureInput>,
        body: Box<NounPhrase>,
    },
    InCaller {
        scope: u32,
        body: Box<NounPhrase>,
    },
    /// The participant bound by the nearest lookback, viewed one entity at a time.
    Gap {
        kind: Kind,
    },
    /// the source, by self-name or "this spell" [CR#113.7]
    This,
    AsType {
        r#type: CardType,
        subject: Box<NounPhrase>,
        subtype: Option<Subtype>,
    },
    AsMarker {
        marker: MarkerWord,
        subject: Box<NounPhrase>,
    },
    ResolvedPermanent {
        spell: Box<NounPhrase>,
    },
    TheGrantor {
        marker: MarkerWord,
    },
    You,
    CombatPlayer {
        role: CombatRole,
    },
    PlayerGroup {
        group: PlayerGroupWord,
    },
    Described {
        determiner: Box<DetPhrase>,
        predicate: Box<Predicate>,
    },
    EachOf {
        group: Box<NounPhrase>,
    },
    And {
        phrases: Vec<NounPhrase>,
    },
    Or {
        phrases: Vec<NounPhrase>,
    },
    LibrarySlice {
        end: LibraryEnd,
        amount: Box<Amount>,
        whose: Box<NounPhrase>,
    },
    SomeOf {
        count: Box<SliceCount>,
        description: Option<Box<Predicate>>,
        group: Box<NounPhrase>,
    },
    NamesAgree {
        agreement: NameAgreement,
        group: Box<NounPhrase>,
    },
    TheRest {
        kind: Kind,
        plurality: Plurality,
    },
    PileOf {
        count: Box<SliceCount>,
        by: Option<Box<NounPhrase>>,
    },
    /// A pronoun: what it reaches for, its number, and the window it resolves in.
    Pro {
        reach: Reach,
        plurality: Plurality,
        window: Window,
    },
    AttachHost {
        word: AttachWord,
        head: NounWord,
    },
    PossessorOf {
        axis: PossessorAxis,
        subject: Box<NounPhrase>,
    },
    Designated {
        designation: DesignationLabel,
        whose: Box<NounPhrase>,
    },
    OneEachOf {
        roles: Vec<Predicate>,
        pool: Box<NounPhrase>,
    },
}

#[derive(Debug, Clone, PartialEq, Eq, Hash, SupportsMacros)]
pub enum Amount {
    WithBindings {
        scope: u32,
        inputs: Vec<CaptureInput>,
        body: Box<Amount>,
    },
    InCaller {
        scope: u32,
        body: Box<Amount>,
    },
    Parameter {
        scope: u32,
        index: u32,
        shape: AmountShape,
    },
    Lit {
        value: i32,
    },
    StatOf {
        axis: ProjAxis,
        subject: Box<NounPhrase>,
    },
    CountOf {
        group: Box<NounPhrase>,
    },
    Aggregate {
        op: AggregateOp,
        axis: ProjAxis,
        group: Box<NounPhrase>,
    },
    Paid {
        facet: PaidFacet,
        subject: Box<NounPhrase>,
    },
    EventTally {
        op: TallyOp,
        subject: Box<NounPhrase>,
        lookback: Box<LookbackClause>,
    },
    ThatMuch,
    ChosenNumber {
        r#ref: ChoiceRef,
    },
    VotesFor {
        label: VoteLabel,
    },
    TheOutcome {
        sort: OutcomeSort,
    },
    CoinsShowing {
        face: CoinFace,
    },
    GreatestStoredMatch {
        subject: Box<NounPhrase>,
    },
    GroupSize,
    TheDifference,
    Letter {
        letter: Letter,
    },
    Arith {
        op: ArithOp,
        left: Box<Amount>,
        right: Box<Amount>,
    },
    Devotion {
        player: Box<NounPhrase>,
        color: ColorTerm,
        second: Option<ColorTerm>,
    },
    Half {
        rounding: RoundMode,
        amount: Box<Amount>,
    },
    AggregateOver {
        op: AggregateOp,
        domain: Box<Predicate>,
        body: Box<Amount>,
    },
    DistinctCount {
        axis: KindAxis,
        domain: Box<NounPhrase>,
    },
    UpTo {
        bound: Box<Amount>,
    },
}

#[derive(Debug, Clone, PartialEq, Eq, Hash, SupportsMacros)]
pub enum Quantity {
    WithBindings {
        scope: u32,
        inputs: Vec<CaptureInput>,
        body: Box<Quantity>,
    },
    InCaller {
        scope: u32,
        body: Box<Quantity>,
    },
    Range {
        low: Option<u32>,
        high: Option<u32>,
    },
    UpToOf {
        amount: Box<Amount>,
    },
    ExactlyOf {
        amount: Box<Amount>,
    },
}

#[derive(Debug, Clone, PartialEq, Eq, Hash, Deserialize, Expand, Serialize)]
pub enum SliceCount {
    Counted { quantity: Box<Quantity> },
    Whole,
}

#[derive(Debug, Clone, PartialEq, Eq, Hash, Deserialize, Expand, Serialize)]
pub enum Door {
    ThisDoor,
    DoorOf {
        state: Option<LockState>,
        room: Box<NounPhrase>,
    },
}

/// Which roll results a trigger watches.
#[derive(Debug, Clone, PartialEq, Eq, Hash, Deserialize, Expand, Serialize)]
pub enum RollWatch {
    AnyResult,
    ResultIn { quantity: Box<Quantity> },
    HighestNatural,
}

/// Whose turn part a "beginning of" header names.
#[derive(Debug, Clone, PartialEq, Eq, Hash, Deserialize, Expand, Serialize)]
pub enum HeaderPossessor {
    NoPossessor,
    ByPlayer { player: Box<NounPhrase> },
    ByTurn { turn: Box<NounPhrase> },
}

#[derive(Debug, Clone, PartialEq, Eq, Hash, SupportsMacros)]
pub enum Condition {
    WithBindings {
        scope: u32,
        inputs: Vec<CaptureInput>,
        body: Box<Condition>,
    },
    InCaller {
        scope: u32,
        body: Box<Condition>,
    },
    DuringPart {
        part: TurnPart,
        whose: Option<Box<NounPhrase>>,
    },
    /// "if there is a …"
    Exists {
        subject: Box<NounPhrase>,
    },
    Happened {
        subject: Box<NounPhrase>,
        lookback: Box<LookbackClause>,
    },
    GameIs {
        designation: DesignationLabel,
    },
    NoHolder {
        designation: DesignationLabel,
    },
    Matches {
        subject: Box<NounPhrase>,
        predicate: Box<Predicate>,
    },
    CompareAmt {
        subject: Box<Amount>,
        comparator: Comparator,
        bound: Box<Amount>,
    },
    DealtThisWay {
        predicate: Box<Predicate>,
    },
    ChoseThisWay {
        chooser: Box<NounPhrase>,
        predicate: Box<Predicate>,
    },
    PreventedFromSource {
        predicate: Box<Predicate>,
    },
    FlipCalled {
        caller: Box<NounPhrase>,
        call: FlipCall,
    },
    FlipFace {
        face: CoinFace,
    },
    VoteLead {
        label: VoteLabel,
        or_tied: bool,
    },
    AnyResultIs {
        comparator: Comparator,
        bound: Box<Amount>,
    },
    RolledDoubles,
    Not {
        condition: Box<Condition>,
    },
    And {
        conjuncts: Vec<Condition>,
    },
    Or {
        disjuncts: Vec<Condition>,
    },
}

#[derive(Debug, Clone, PartialEq, Eq, Hash, SupportsMacros)]
pub enum GameEvent {
    WithBindings {
        scope: u32,
        inputs: Vec<CaptureInput>,
        body: Box<GameEvent>,
    },
    InCaller {
        scope: u32,
        body: Box<GameEvent>,
    },
    ZoneChange {
        subject: Box<NounPhrase>,
        from: Option<Box<EventSource>>,
        to: Option<Box<ZoneExpr>>,
        observation: ObservationPoint,
    },
    Draws {
        player: Box<NounPhrase>,
    },
    LosesGame {
        player: Box<NounPhrase>,
    },
    /// "attacks", "attacks you", "blocks", "becomes blocked by …": an object's combat event,
    /// with the counterpart the sentence names.
    Combat {
        relation: CombatRelation,
        subject: Box<NounPhrase>,
        counterpart: Option<Box<NounPhrase>>,
    },
    /// "you attack with one or more creatures".
    AttacksWith {
        player: Box<NounPhrase>,
        defender: Option<Box<NounPhrase>>,
        attackers: Box<NounPhrase>,
    },
    Attachment {
        r#move: AttachMove,
        subject: Box<NounPhrase>,
        host: Box<NounPhrase>,
    },
    Damage {
        kind: DamageKind,
        source: Option<Box<NounPhrase>>,
        patient: Option<Box<NounPhrase>>,
    },
    BeginningOf {
        quantifier: PartQuant,
        part: TurnPart,
        whose: Box<HeaderPossessor>,
    },
    Casts {
        player: Box<NounPhrase>,
        spell: Option<Box<NounPhrase>>,
        from: Option<Box<EventSource>>,
    },
    BecomesTarget {
        subject: Box<NounPhrase>,
        by: Box<NounPhrase>,
    },
    StatusEvent {
        subject: Box<NounPhrase>,
        status: Status,
    },
    /// The game gains a designation: "it becomes night" [CR#731.1].
    GameBecomes {
        designation: DesignationLabel,
    },
    StateHolds {
        condition: Box<Condition>,
    },
    CounterEvent {
        r#move: CounterMove,
        kind: Option<CounterKind>,
        subject: Box<NounPhrase>,
        batch: CounterBatch,
        by: Option<Box<NounPhrase>>,
        by_effect: bool,
    },
    TokensCreated {
        tokens: Box<NounPhrase>,
        by_effect: bool,
        by: Option<Box<NounPhrase>>,
        under: Option<Box<NounPhrase>>,
    },
    ChapterMark {
        chapters: Vec<ChapterNumber>,
    },
    Activates {
        player: Box<NounPhrase>,
        ability: Box<NounPhrase>,
    },
    StatBecomes {
        subject: Box<NounPhrase>,
        stat: Stat,
        value: Box<Amount>,
    },
    FlipsCoin {
        player: Box<NounPhrase>,
        call: Option<FlipCall>,
    },
    /// `sides = none` is "whenever you roll a die", any die.
    RollsDice {
        player: Box<NounPhrase>,
        batch: DiceBatch,
        sides: Option<u32>,
        watch: Box<RollWatch>,
    },
    PaysCost {
        player: Option<Box<NounPhrase>>,
        outcome: PaymentOutcome,
        whose: Box<NounPhrase>,
        keyword: KeywordLabel,
    },
    PaysLife {
        player: Box<NounPhrase>,
    },
    LifeChanges {
        player: Box<NounPhrase>,
        r#move: LifeMove,
    },
    VerbedEvent {
        agent: Option<Box<NounPhrase>>,
        verb: Deed,
        patient: Option<Box<NounPhrase>>,
        becomes: Option<Box<Predicate>>,
        locus: Option<Box<ZoneExpr>>,
    },
    /// A mana ability with {T} in its cost resolving and producing mana [CR#106.12a].
    TappedForMana {
        player: Option<Box<NounPhrase>>,
        source: Box<NounPhrase>,
        r#type: Option<ManaTypeTerm>,
    },
    UnlocksDoor {
        player: Box<NounPhrase>,
        door: Box<Door>,
    },
    NthOccurrence {
        ordinal: Ordinal,
        per: Option<TurnPart>,
        event: Box<GameEvent>,
    },
    Triggers {
        ability: Box<NounPhrase>,
    },
    CommitsCrime {
        player: Box<NounPhrase>,
    },
    Causes {
        cause: Box<Causing>,
        event: Box<GameEvent>,
    },
}

#[derive(Debug, Clone, PartialEq, Eq, Hash, Deserialize, Expand, Serialize)]
pub enum Causing {
    Source { source: Box<NounPhrase> },
    Event { event: Box<GameEvent> },
    AnEffect,
}

#[derive(Debug, Clone, PartialEq, Eq, Hash, Deserialize, Expand, Serialize)]
pub enum SearchScope {
    OneZone {
        zone: ZoneExpr,
    },
    SomeZones {
        whose: Option<NounPhrase>,
        zones: Vec<Zone>,
    },
}

#[derive(Debug, Clone, PartialEq, Eq, Hash, Deserialize, Expand, Serialize)]
pub enum FlipScope {
    Count { amount: Amount },
    Per { each: NounPhrase },
}

#[derive(Debug, Clone, PartialEq, Eq, Hash, Deserialize, Expand, Serialize)]
pub enum IgnoredOutcomes {
    Extreme {
        extreme: RollExtreme,
    },
    AllBut {
        extreme: RollExtreme,
    },
    Chosen {
        chooser: Option<NounPhrase>,
        amount: Amount,
    },
}

/// Macroable: Vote's `ballot` parameter (`params: [Ballot]`) needs a
/// registered kind to name.
#[derive(Debug, Clone, PartialEq, Eq, Hash, SupportsMacros)]
pub enum Ballot {
    ByLabel { options: Vec<VoteLabel> },
    ByCandidate { candidates: NounPhrase },
}

#[derive(Debug, Clone, PartialEq, Eq, Hash, Deserialize, Expand, Serialize)]
pub enum Exposed {
    Cards { cards: NounPhrase },
    Zone { zone: ZoneExpr },
    Choice { sort: ChoiceSort },
}

#[derive(Debug, Clone, PartialEq, Eq, Hash, Deserialize, Expand, Serialize)]
pub enum VisibleThing {
    TopOfLibrary,
    WholeHand,
    Objects { objects: NounPhrase },
}

#[derive(Debug, Clone, PartialEq, Eq, Hash, Deserialize, Expand, Serialize)]
pub enum DurationEnd {
    StartOf {
        part: TurnPart,
        whose: Option<NounPhrase>,
    },
    EndOf {
        part: TurnPart,
        whose: Option<NounPhrase>,
    },
}
