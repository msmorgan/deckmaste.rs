//! The generated grammar tables the elaborator consumes — emitted by
//! `idris/src/EmitTables.idr` (regenerate with `idris/scripts/emit-tables`)
//! into `crates/deckmaste_cards/tables/*.ron` and compiled in here. The
//! checker never restates a row in code: rule VALUES (event caps, cost
//! eligibility, scopes, the kind join, binding-context transitions) come from
//! these tables; the walker contributes only tree shape.

use std::collections::HashMap;
use std::sync::LazyLock;

use serde::Deserialize;

/// A reference/filter kind — the Idris `RefKind` ([CR#109.1]): does an
/// expression denote an object, a player, either (`Any`, the "any target"
/// top, [CR#115.4]), or nothing (`Empty`, the vacuous bottom).
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, Deserialize)]
pub enum Kind {
    Empty,
    Object,
    Player,
    Any,
}

impl Kind {
    /// Whether a value of this kind can occupy a slot expecting `expected` —
    /// the lattice compatibility read (`≤` up to the `Any` top): only a
    /// definite `Object` in a definite `Player` slot (or vice versa)
    /// conflicts.
    #[must_use]
    pub fn compatible_with(self, expected: Kind) -> bool {
        !matches!(
            (self, expected),
            (Kind::Object, Kind::Player) | (Kind::Player, Kind::Object)
        )
    }
}

/// What an event supplies its body's anaphora — the Idris `EventCaps`
/// ([CR#603.2e,608.2k]): a distinguished object, a responsible player, a
/// numeric amount, a patient of a FIXED kind, a combat defender.
#[expect(
    clippy::struct_excessive_bools,
    reason = "each flag is an independent capability the emitted row carries"
)]
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub struct Caps {
    pub object: bool,
    pub actor: bool,
    pub amount: bool,
    pub patient: Option<Kind>,
    pub defender: bool,
}

/// No capabilities — the caps outside any event body, and of any event
/// pattern with no table row (the safe direction: an unknown event supplies
/// nothing, so a body anaphor under it is rejected).
pub const NO_CAPS: Caps = Caps {
    object: false,
    actor: false,
    amount: false,
    patient: None,
    defender: false,
};

impl Caps {
    /// The intersection — a multi-pattern event guarantees only what EVERY
    /// disjunct supplies (the Idris `andCaps`). A patient guaranteed by both
    /// sides survives at the WIDENED sort ([CR#115.4] — object|player is
    /// still a patient, kind-poly): never `sameKind`-dropped.
    #[must_use]
    pub fn meet(self, other: Caps) -> Caps {
        Caps {
            object: self.object && other.object,
            actor: self.actor && other.actor,
            amount: self.amount && other.amount,
            patient: widen_kind(self.patient, other.patient),
            defender: self.defender && other.defender,
        }
    }

    /// The union — a conjunction/composite binds what ANY side binds (the
    /// Idris `orCaps`), with patient-sort REFINEMENT: a side that fixes a
    /// definite patient sort refines a polymorphic (`Any`) or absent one;
    /// two incomparable definite sorts refine to nothing (conservative).
    #[must_use]
    pub fn join(self, other: Caps) -> Caps {
        Caps {
            object: self.object || other.object,
            actor: self.actor || other.actor,
            amount: self.amount || other.amount,
            patient: refine_kind(self.patient, other.patient),
            defender: self.defender || other.defender,
        }
    }
}

/// The disjunctive patient combination: both sides must supply one, and the
/// sort is their kind-lattice join (equal sorts keep it, object|player
/// widens to `Any` — mirroring the emitted `kind-lattice.ron` square, in
/// which patient sorts never involve `Empty`).
fn widen_kind(a: Option<Kind>, b: Option<Kind>) -> Option<Kind> {
    match (a, b) {
        (Some(x), Some(y)) if x == y => Some(x),
        (Some(_), Some(_)) => Some(Kind::Any),
        _ => None,
    }
}

/// The conjunctive patient combination: either side alone supplies it, a
/// definite sort refines `Any`, and two incomparable definite sorts cancel
/// (the conjunction is vacuous there — the kind pass flags it separately).
fn refine_kind(a: Option<Kind>, b: Option<Kind>) -> Option<Kind> {
    match (a, b) {
        (Some(x), Some(y)) if x == y => Some(x),
        (Some(Kind::Any), Some(definite)) | (Some(definite), Some(Kind::Any)) => Some(definite),
        (Some(_), Some(_)) => None,
        (one, None) | (None, one) => one,
    }
}

#[expect(
    clippy::struct_excessive_bools,
    reason = "the fields mirror the emitted RON row shape one-to-one"
)]
#[derive(Debug, Deserialize)]
struct EventCapsRow {
    key: String,
    object: bool,
    actor: bool,
    amount: bool,
    patient: Option<Kind>,
    defender: bool,
    /// Where the form's OBJECT antecedent takes its SORT from — a named
    /// participant slot (`"source"`/`"what"`/`"by"`/`"of"`/`"on"`), the
    /// destination zone (`"to_zone"`), a fixed sort (`"spell"`/
    /// `"stack_object"`/`"token"`), or `"none"`.
    object_sort: String,
    #[expect(
        dead_code,
        reason = "cites ride the rows for `cite audit`, not the checker"
    )]
    cite: String,
}

#[derive(Debug, Deserialize)]
struct EventCapsFile {
    rows: Vec<EventCapsRow>,
}

#[expect(
    clippy::struct_excessive_bools,
    reason = "the fields mirror the emitted RON row shape one-to-one"
)]
#[derive(Debug, Deserialize)]
struct CostActionRow {
    action: String,
    object: bool,
    actor: bool,
    amount: bool,
    patient: Option<Kind>,
    defender: bool,
    #[expect(
        dead_code,
        reason = "cites ride the rows for `cite audit`, not the checker"
    )]
    cite: String,
}

#[derive(Debug, Deserialize)]
struct CostActionFile {
    rows: Vec<CostActionRow>,
}

#[derive(Debug, Deserialize)]
struct ScopeRow {
    name: String,
    scope: Kind,
    #[expect(
        dead_code,
        reason = "cites ride the rows for `cite audit`, not the checker"
    )]
    cite: String,
}

#[derive(Debug, Deserialize)]
struct AgentRow {
    relation: String,
    scope: Kind,
    #[expect(
        dead_code,
        reason = "cites ride the rows for `cite audit`, not the checker"
    )]
    cite: String,
}

#[derive(Debug, Deserialize)]
struct ScopesFile {
    counter_default: Kind,
    counters: Vec<ScopeRow>,
    designations: Vec<ScopeRow>,
    agents: Vec<AgentRow>,
    patients: Vec<AgentRow>,
}

#[derive(Debug, Deserialize)]
struct JoinRow {
    left: Kind,
    right: Kind,
    join: Kind,
}

#[derive(Debug, Deserialize)]
struct LatticeFile {
    joins: Vec<JoinRow>,
}

/// A binder's cardinality — one object (`Reference::That`) vs a group
/// (`Selection::That`).
#[derive(Debug, Clone, Copy, PartialEq, Eq, Deserialize)]
pub enum Cardinality {
    One,
    Many,
}

/// Where a construct's body caps come from: `Keep` the surrounding event
/// context, rebind from an event `Query`, or from a `Cost`'s payment.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Deserialize)]
pub enum CapsFrom {
    Keep,
    Query,
    Cost,
}

/// One binding-context transition row — computed in the emitter by applying
/// the Idris `bind*` Endophora transforms to a fully-bound probe.
#[expect(
    clippy::struct_excessive_bools,
    reason = "the fields mirror the emitted RON row shape one-to-one"
)]
#[derive(Debug, Deserialize)]
pub struct BindRule {
    pub construct: String,
    pub drops_targets: bool,
    pub keeps_that: bool,
    pub binds_targets: bool,
    pub binds_that: Option<Cardinality>,
    pub binds_it: bool,
    pub binds_allotment: bool,
    pub clears_allotment: bool,
    pub caps_from: CapsFrom,
    pub may_target: Option<bool>,
    /// Rides the row for `cite audit`, not the checker.
    pub cite: String,
}

#[derive(Debug, Deserialize)]
struct BindRulesFile {
    rows: Vec<BindRule>,
}

/// One cause-verb ENTAILMENT row ([CR#701] keyword actions): the fact form a
/// [`deckmaste_core::CauseVerb`] normalizes to — its master-form kind and
/// the zone coordinates it fixes — plus the caps guarantees a cause-narrowed
/// pattern inherits from the verb (`Sacrifice` supplies the sacrificing
/// player as actor, [CR#701.21a]). Consumed two ways: caps augmentation
/// under `ZoneChange { cause }`, and the kind pass's entailment-consistency
/// check (a pattern whose fixed coordinates contradict its verb's entailed
/// ones can never match).
#[derive(Debug, Deserialize)]
pub struct EntailmentRow {
    pub verb: String,
    /// The entailed master-form KIND (an `event-caps.ron` key).
    pub kind: String,
    pub from: Option<deckmaste_core::Zone>,
    pub to: Option<deckmaste_core::Zone>,
    pub object: bool,
    pub actor: bool,
    pub amount: bool,
    /// Rides the row for `cite audit`, not the checker.
    pub cite: String,
}

impl EntailmentRow {
    /// The caps guarantees the verb adds to a cause-narrowed pattern.
    #[must_use]
    pub fn caps(&self) -> Caps {
        Caps {
            object: self.object,
            actor: self.actor,
            amount: self.amount,
            patient: None,
            defender: false,
        }
    }
}

#[derive(Debug, Deserialize)]
struct EntailmentFile {
    rows: Vec<EntailmentRow>,
}

/// Which ENGINE MATCHER a lane's patterns run on until the one-evaluator
/// rebase (the `engine-eventfilter-bridge` compile-down): the live trigger
/// matcher (`event_matches`), the replacement would-matcher (abstract
/// intents), or the history scan (the live matcher over recorded facts).
/// The bridge-caps table is keyed per matcher.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Deserialize)]
pub enum Matcher {
    Live,
    Would,
    History,
}

/// One event-LANE row (the plan's §3.2 lane table): which algebra
/// refinements each consumer position admits. `within` — history windows
/// are vacuous against a live fact ([CR#603.2]); `one_or_more` — batch
/// matching only makes sense where one occurrence fires once ([CR#603.2c]);
/// `nth` — ordinal refinement; `anchored` — every disjunct must bottom out
/// in a master form (kind-anchored; no freeze-everything `CantHappen`);
/// `matcher` — the bridge matcher the lane evaluates on.
#[expect(
    clippy::struct_excessive_bools,
    reason = "the fields mirror the emitted RON row shape one-to-one"
)]
#[derive(Debug, Deserialize)]
pub struct LaneRow {
    pub lane: String,
    pub within: bool,
    pub nth: bool,
    pub one_or_more: bool,
    pub anchored: bool,
    pub matcher: Matcher,
    /// Rides the row for `cite audit`, not the checker.
    pub cite: String,
}

#[derive(Debug, Deserialize)]
struct LanesFile {
    rows: Vec<LaneRow>,
}

/// One bridge-caps row (`engine-eventfilter-bridge`): an `EventFilter` ATOM
/// (a master-form key, a `Form:field` refinement, an algebra node, or a
/// `Lookback:*` history window) and whether each bridge [`Matcher`]
/// evaluates it FAITHFULLY today. An atom a lane's matcher does not support
/// is load-rejected (`E-BRIDGE-CAP`), never silently mis-matched; the
/// one-evaluator rebase lifts the caps.
#[derive(Debug, Deserialize)]
pub struct BridgeRow {
    pub atom: String,
    pub live: bool,
    pub would: bool,
    pub history: bool,
    /// Rides the row for `cite audit`, not the checker.
    pub cite: String,
}

impl BridgeRow {
    /// Whether `matcher` evaluates this atom faithfully.
    #[must_use]
    pub fn supports(&self, matcher: Matcher) -> bool {
        match matcher {
            Matcher::Live => self.live,
            Matcher::Would => self.would,
            Matcher::History => self.history,
        }
    }
}

#[derive(Debug, Deserialize)]
struct BridgeFile {
    rows: Vec<BridgeRow>,
}

/// One sort-compat row (the anaphor surface's R1 table): an anaphor's
/// wanted sort KEY reaching an antecedent's have sort KEY. `widened` marks
/// a non-exact (widening) match — the R2 gate's exact-vs-widened
/// distinction. A pair with no row is incompatible. `OfType`/`OfType`
/// additionally requires the same card type (applied by the walker).
#[derive(Debug, Deserialize)]
struct CompatRow {
    want: String,
    have: String,
    widened: bool,
    #[expect(
        dead_code,
        reason = "cites ride the rows for `cite audit`, not the checker"
    )]
    cite: String,
}

#[derive(Debug, Deserialize)]
struct CompatFile {
    /// The R2 gate's ONE pre-approved loosening (a nearer exact-sort match
    /// beats farther non-exact candidates without erroring) — calibrated and
    /// FROZEN STRICT (off) by [[cards-corpus-dry-run]]: 0 gate fires over
    /// 5785 encodable faces, 0 mis-bindings in the 200-face hand audit.
    exact_sort_precedence: bool,
    rows: Vec<CompatRow>,
}

/// One zone-sort row: the noun an object answers to in each zone
/// ([CR#110.1,112.1,108.2]).
#[derive(Debug, Deserialize)]
struct ZoneSortRow {
    zone: deckmaste_core::Zone,
    sort: String,
    #[expect(
        dead_code,
        reason = "cites ride the rows for `cite audit`, not the checker"
    )]
    cite: String,
}

#[derive(Debug, Deserialize)]
struct ZoneSortsFile {
    rows: Vec<ZoneSortRow>,
}

/// Which SITE an intro row's antecedent lands at.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Deserialize)]
pub enum IntroSite {
    TargetSlot,
    Product,
    Chosen,
    Loop,
}

/// An intro row's cardinality rule.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Deserialize)]
pub enum IntroCard {
    One,
    Many,
    /// Derived from the clause's quantity (literal 1 = One, else Many).
    FromQuantity,
    /// Derived from the clause's count (literal 1 = One, else Many) — the
    /// fact-signature product-arity column.
    FromCount,
}

/// One intro row: a producing clause's antecedent pushes (the plan's §2.1
/// intro table). `sort`/`zone` are derivation tags the walker interprets
/// (`"from_filter"`, `"from_destination"`, `"from_binder"`, a fixed sort /
/// zone name, `"none"`).
#[derive(Debug, Deserialize)]
pub struct IntroRow {
    pub clause: String,
    /// Whether the clause pushes an OBJECT antecedent at all.
    pub object: bool,
    pub site: IntroSite,
    pub card: IntroCard,
    pub sort: String,
    pub zone: String,
    /// Whether the clause ADDITIONALLY pushes an Amount antecedent
    /// ("that many/much").
    pub amount: bool,
    /// Rides the row for `cite audit`, not the checker.
    pub cite: String,
}

#[derive(Debug, Deserialize)]
struct IntroFile {
    rows: Vec<IntroRow>,
}

/// A static part's affected-set class under `Until` ([CR#611.2c]):
/// characteristic-/controller-modifying parts gather once at start;
/// rules-modifying parts stay live.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Deserialize)]
pub enum SetClass {
    GatherOnce,
    GatherLive,
}

#[derive(Debug, Deserialize)]
struct ClassRow {
    kind: String,
    class: SetClass,
    #[expect(
        dead_code,
        reason = "cites ride the rows for `cite audit`, not the checker"
    )]
    cite: String,
}

#[derive(Debug, Deserialize)]
struct ClassesFile {
    rows: Vec<ClassRow>,
}

/// One checker-rule manifest row: an error code the elaborator may emit,
/// with its CR citation. The drift pin: a test asserts the elaborator's code
/// list equals this manifest (no checker rule without a table row).
#[derive(Debug, Deserialize)]
pub struct RuleRow {
    pub code: String,
    /// Rides the row for `cite audit`, not the checker.
    pub cite: String,
    /// Human-facing one-line meaning.
    pub summary: String,
}

#[derive(Debug, Deserialize)]
struct RulesFile {
    rows: Vec<RuleRow>,
}

/// The loaded tables.
pub struct Tables {
    event_caps: HashMap<String, Caps>,
    event_object_sorts: HashMap<String, String>,
    cost_actions: HashMap<String, Caps>,
    counter_default: Kind,
    counter_scopes: HashMap<String, Kind>,
    designation_scopes: HashMap<String, Kind>,
    agent_scopes: HashMap<String, Kind>,
    patient_scopes: HashMap<String, Kind>,
    joins: HashMap<(Kind, Kind), Kind>,
    bind_rules: HashMap<String, BindRule>,
    entailments: HashMap<String, EntailmentRow>,
    lanes: HashMap<String, LaneRow>,
    bridge: HashMap<String, BridgeRow>,
    exact_sort_precedence: bool,
    compat: HashMap<(String, String), bool>,
    zone_sorts: HashMap<deckmaste_core::Zone, String>,
    intro: HashMap<String, IntroRow>,
    static_classes: HashMap<String, SetClass>,
    pub rules: Vec<RuleRow>,
}

impl Tables {
    /// The caps an event-pattern KEY guarantees; a key with no row supplies
    /// nothing (never a silent grant).
    #[must_use]
    pub fn event_caps(&self, key: &str) -> Caps {
        self.event_caps.get(key).copied().unwrap_or(NO_CAPS)
    }

    /// A cost-eligible player verb's row ([CR#118.3]) — `None` means the verb
    /// is NOT cost-eligible; there is no catch-all row.
    #[must_use]
    pub fn cost_action(&self, action: &str) -> Option<Caps> {
        self.cost_actions.get(action).copied()
    }

    /// A counter kind's carrier scope ([CR#122.1]): its row, or the emitted
    /// object default (which is model content — `counterScope`'s total
    /// fallback).
    #[must_use]
    pub fn counter_scope(&self, name: &str) -> Kind {
        self.counter_scopes
            .get(name)
            .copied()
            .unwrap_or(self.counter_default)
    }

    /// A designation's carrier scope, when the curated table names it —
    /// unknown designations are an open vocabulary and impose no constraint.
    #[must_use]
    pub fn designation_scope(&self, name: &str) -> Option<Kind> {
        self.designation_scopes.get(name).copied()
    }

    /// A deed relation's agent kind (the Idris `agentScope`).
    #[must_use]
    pub fn agent_scope(&self, relation: &str) -> Option<Kind> {
        self.agent_scopes.get(relation).copied()
    }

    /// A deed relation's PATIENT kind (the Idris `patientScope`): what kind
    /// of participant the relation acts upon — a blocked thing is an
    /// attacking creature ([CR#509.1a]), an attacked thing is a player,
    /// planeswalker, or battle ([CR#508.1b]), a target is anything
    /// targetable ([CR#115.4]).
    #[must_use]
    pub fn patient_scope(&self, relation: &str) -> Option<Kind> {
        self.patient_scopes.get(relation).copied()
    }

    /// The kind JOIN (the Idris `\/`): what a disjunction of the two kinds
    /// denotes ([CR#115.4] — object|player widens to `Any`).
    ///
    /// # Panics
    /// On a pair the emitted lattice doesn't cover — impossible while the
    /// emitter enumerates the full `Kind × Kind` square.
    #[must_use]
    pub fn join(&self, a: Kind, b: Kind) -> Kind {
        *self
            .joins
            .get(&(a, b))
            .expect("the emitted lattice covers all kind pairs")
    }

    /// A construct's binding-context transition row.
    ///
    /// # Panics
    /// On a missing row: the walker's construct names and the emitted rows
    /// are fixed together (both compiled in), so a miss is a bug, not data.
    #[must_use]
    pub fn bind_rule(&self, construct: &str) -> &BindRule {
        self.bind_rules
            .get(construct)
            .unwrap_or_else(|| panic!("no bind rule emitted for construct {construct:?}"))
    }

    /// A cause verb's entailment row ([CR#701] keyword actions) — `None`
    /// only for a verb outside the emitted closed set (impossible while the
    /// emitter covers every `CauseVerb`; the caller treats a miss as
    /// no-augmentation, the safe direction).
    #[must_use]
    pub fn entailment(&self, verb: &str) -> Option<&EntailmentRow> {
        self.entailments.get(verb)
    }

    /// An event consumer position's lane row (the §3.2 lane table).
    ///
    /// # Panics
    /// On a missing row: the walker's lane keys and the emitted rows are
    /// fixed together (both compiled in), so a miss is a bug, not data.
    #[must_use]
    pub fn lane(&self, key: &str) -> &LaneRow {
        self.lanes
            .get(key)
            .unwrap_or_else(|| panic!("no lane row emitted for {key:?}"))
    }

    /// Whether `matcher` supports the bridge-caps `atom`
    /// (`engine-eventfilter-bridge`). An atom with no row supplies nothing —
    /// unsupported is the safe direction (a new grammar atom must mint its
    /// row before it loads).
    #[must_use]
    pub fn bridge_supports(&self, atom: &str, matcher: Matcher) -> bool {
        self.bridge.get(atom).is_some_and(|r| r.supports(matcher))
    }

    /// Every bridge-caps row — the engine's bridge-agreement test iterates
    /// these to pin admission against matcher capability.
    pub fn bridge_rows(&self) -> impl Iterator<Item = &BridgeRow> {
        self.bridge.values()
    }

    /// The R2 gate's one pre-approved loosening flag (`sort-compat.ron`):
    /// a nearer exact-sort match beats farther non-exact candidates without
    /// erroring. Calibrated and FROZEN STRICT (off) by
    /// [[cards-corpus-dry-run]] — flipping it is a reviewed event now.
    #[must_use]
    pub fn exact_sort_precedence(&self) -> bool {
        self.exact_sort_precedence
    }

    /// R1 sort compatibility by table KEYS: `Some(widened)` when the wanted
    /// sort reaches the had sort (`widened = false` marks an exact-shape
    /// match); `None` = incompatible. `OfType`/`OfType` same-type refinement
    /// is the caller's (the keys collapse the payload).
    #[must_use]
    pub fn sort_compat(&self, want: &str, have: &str) -> Option<bool> {
        self.compat
            .get(&(want.to_owned(), have.to_owned()))
            .copied()
    }

    /// The noun an object answers to in `zone` ([CR#110.1,112.1,108.2]) —
    /// a fixed sort key (`"Permanent"`/`"Spell"`/`"Card"`).
    ///
    /// # Panics
    /// On a missing row: the emitter enumerates every zone.
    #[must_use]
    pub fn zone_sort(&self, zone: deckmaste_core::Zone) -> &str {
        self.zone_sorts
            .get(&zone)
            .unwrap_or_else(|| panic!("no zone-sort row emitted for {zone:?}"))
    }

    /// A producing clause's intro row (the plan's §2.1 intro table).
    ///
    /// # Panics
    /// On a missing row: the walker's clause keys and the emitted rows are
    /// fixed together (both compiled in), so a miss is a bug, not data.
    #[must_use]
    pub fn intro(&self, clause: &str) -> &IntroRow {
        self.intro
            .get(clause)
            .unwrap_or_else(|| panic!("no intro row emitted for clause {clause:?}"))
    }

    /// Where an event form's OBJECT antecedent takes its sort from
    /// (`event-caps.ron`'s `object_sort` column); `"none"` for unknown keys.
    #[must_use]
    pub fn event_object_sort(&self, key: &str) -> &str {
        self.event_object_sorts
            .get(key)
            .map_or("none", String::as_str)
    }

    /// A static part's affected-set class under `Until` ([CR#611.2c]).
    ///
    /// # Panics
    /// On a missing row: the walker's kind keys and the emitted rows are
    /// fixed together.
    #[must_use]
    pub fn static_class(&self, kind: &str) -> SetClass {
        *self
            .static_classes
            .get(kind)
            .unwrap_or_else(|| panic!("no static-class row emitted for {kind:?}"))
    }
}

fn parse<T: serde::de::DeserializeOwned>(what: &str, source: &str) -> T {
    ron::from_str(source).unwrap_or_else(|e| panic!("generated table {what} failed to parse: {e}"))
}

/// The compiled-in tables, parsed once.
#[must_use]
pub fn tables() -> &'static Tables {
    static TABLES: LazyLock<Tables> = LazyLock::new(|| {
        let caps: EventCapsFile = parse("event-caps", include_str!("../../tables/event-caps.ron"));
        let cost: CostActionFile = parse(
            "cost-actions",
            include_str!("../../tables/cost-actions.ron"),
        );
        let scopes: ScopesFile = parse("scopes", include_str!("../../tables/scopes.ron"));
        let lattice: LatticeFile = parse(
            "kind-lattice",
            include_str!("../../tables/kind-lattice.ron"),
        );
        let binds: BindRulesFile = parse("bind-rules", include_str!("../../tables/bind-rules.ron"));
        let entailments: EntailmentFile =
            parse("entailments", include_str!("../../tables/entailments.ron"));
        let lanes: LanesFile = parse("event-lanes", include_str!("../../tables/event-lanes.ron"));
        let bridge: BridgeFile = parse("bridge-caps", include_str!("../../tables/bridge-caps.ron"));
        let rules: RulesFile = parse(
            "checker-rules",
            include_str!("../../tables/checker-rules.ron"),
        );
        let compat: CompatFile = parse("sort-compat", include_str!("../../tables/sort-compat.ron"));
        let zone_sorts: ZoneSortsFile =
            parse("zone-sorts", include_str!("../../tables/zone-sorts.ron"));
        let intro: IntroFile = parse("intro", include_str!("../../tables/intro.ron"));
        let classes: ClassesFile = parse(
            "static-classes",
            include_str!("../../tables/static-classes.ron"),
        );
        Tables {
            event_object_sorts: caps
                .rows
                .iter()
                .map(|r| (r.key.clone(), r.object_sort.clone()))
                .collect(),
            event_caps: caps
                .rows
                .into_iter()
                .map(|r| {
                    (
                        r.key,
                        Caps {
                            object: r.object,
                            actor: r.actor,
                            amount: r.amount,
                            patient: r.patient,
                            defender: r.defender,
                        },
                    )
                })
                .collect(),
            cost_actions: cost
                .rows
                .into_iter()
                .map(|r| {
                    (
                        r.action,
                        Caps {
                            object: r.object,
                            actor: r.actor,
                            amount: r.amount,
                            patient: r.patient,
                            defender: r.defender,
                        },
                    )
                })
                .collect(),
            counter_default: scopes.counter_default,
            counter_scopes: scopes
                .counters
                .into_iter()
                .map(|r| (r.name, r.scope))
                .collect(),
            designation_scopes: scopes
                .designations
                .into_iter()
                .map(|r| (r.name, r.scope))
                .collect(),
            agent_scopes: scopes
                .agents
                .into_iter()
                .map(|r| (r.relation, r.scope))
                .collect(),
            patient_scopes: scopes
                .patients
                .into_iter()
                .map(|r| (r.relation, r.scope))
                .collect(),
            joins: lattice
                .joins
                .into_iter()
                .map(|r| ((r.left, r.right), r.join))
                .collect(),
            bind_rules: binds
                .rows
                .into_iter()
                .map(|r| (r.construct.clone(), r))
                .collect(),
            entailments: entailments
                .rows
                .into_iter()
                .map(|r| (r.verb.clone(), r))
                .collect(),
            lanes: lanes
                .rows
                .into_iter()
                .map(|r| (r.lane.clone(), r))
                .collect(),
            bridge: bridge
                .rows
                .into_iter()
                .map(|r| (r.atom.clone(), r))
                .collect(),
            exact_sort_precedence: compat.exact_sort_precedence,
            compat: compat
                .rows
                .into_iter()
                .map(|r| ((r.want, r.have), r.widened))
                .collect(),
            zone_sorts: zone_sorts
                .rows
                .into_iter()
                .map(|r| (r.zone, r.sort))
                .collect(),
            intro: intro
                .rows
                .into_iter()
                .map(|r| (r.clause.clone(), r))
                .collect(),
            static_classes: classes
                .rows
                .into_iter()
                .map(|r| (r.kind, r.class))
                .collect(),
            rules: rules.rows,
        }
    });
    &TABLES
}

#[cfg(test)]
mod tests {
    use super::*;

    /// The tables parse and carry the load-bearing rows.
    #[test]
    fn tables_load() {
        let t = tables();
        // The caps rows the corpus leans on.
        assert!(
            t.event_caps("ZoneChange").object,
            "a zone change binds its object"
        );
        assert!(
            !t.event_caps("ZoneChange").actor,
            "a bare zone change has no responsible player"
        );
        let damage = t.event_caps("Damage");
        assert!(damage.object && damage.actor && damage.amount);
        assert_eq!(
            damage.patient,
            Some(Kind::Any),
            "a damage recipient is a kind-poly patient ([CR#120.3])"
        );
        assert_eq!(
            t.event_caps("Bogus"),
            NO_CAPS,
            "unknown keys supply nothing"
        );
        // Entailments: the cause-verb fact forms ([CR#701.21a,701.17a]).
        let sac = t.entailment("Sacrifice").expect("Sacrifice row");
        assert_eq!(sac.kind, "ZoneChange");
        assert_eq!(sac.from, Some(deckmaste_core::Zone::Battlefield));
        assert_eq!(sac.to, Some(deckmaste_core::Zone::Graveyard));
        assert!(sac.actor, "the sacrificing player is the actor");
        let mill = t.entailment("Mill").expect("Mill row");
        assert_eq!(mill.from, Some(deckmaste_core::Zone::Library));
        assert!(t.entailment("Bogus").is_none());
        // Lanes: live lanes refuse Within; history refuses OneOrMore.
        let trig = t.lane("Triggered.event");
        assert!(!trig.within && trig.one_or_more && trig.nth && trig.anchored);
        assert_eq!(trig.matcher, Matcher::Live);
        let hist = t.lane("Happened");
        assert!(hist.within && !hist.one_or_more && hist.nth && !hist.anchored);
        assert_eq!(hist.matcher, Matcher::History);
        let mult = t.lane("TriggerMultiplier.cause");
        assert!(!mult.nth, "Nth is refused on the multiplier lane");
        assert_eq!(t.lane("Replacement.would").matcher, Matcher::Would);
        assert_eq!(t.lane("CantHappen").matcher, Matcher::Would);
        // Bridge caps: the named engine-eventfilter-bridge caps hold, and an
        // unknown atom supplies nothing (never a silent grant).
        assert!(t.bridge_supports("ZoneChange", Matcher::Live));
        assert!(t.bridge_supports("ZoneChange", Matcher::Would));
        assert!(
            !t.bridge_supports("Not", Matcher::Live)
                && !t.bridge_supports("Nth", Matcher::Live)
                && !t.bridge_supports("When", Matcher::Live)
                && !t.bridge_supports("Within", Matcher::History),
            "the algebra caps hold until engine-one-evaluator"
        );
        assert!(
            t.bridge_supports("OneOrMore", Matcher::Live)
                && t.bridge_supports("OneOrMore", Matcher::Would),
            "batch-once matching is bridged in the live/would lanes"
        );
        assert!(
            !t.bridge_supports("Cast", Matcher::Would),
            "the would-matcher lowers only the intent shapes the engine emits"
        );
        assert!(
            t.bridge_supports("Lookback:ThisTurn", Matcher::History)
                && !t.bridge_supports("Lookback:ThisCombat", Matcher::History),
            "sub-turn history windows are capped (no combat/step markers)"
        );
        assert!(!t.bridge_supports("Bogus", Matcher::Live));
        // Cost rows: an explicit row per eligible verb, no catch-all.
        assert!(t.cost_action("Sacrifice").is_some());
        assert!(t.cost_action("Draw").is_none(), "Draw is not a cost");
        // Scopes.
        assert_eq!(t.counter_scope("Poison"), Kind::Player);
        assert_eq!(t.counter_scope("P1P1Counter"), Kind::Object);
        assert_eq!(t.counter_scope("SomeFutureCounter"), Kind::Object);
        assert_eq!(t.designation_scope("Monarch"), Some(Kind::Player));
        assert_eq!(t.designation_scope("Monstrous"), Some(Kind::Object));
        assert_eq!(t.designation_scope("SomethingElse"), None);
        assert_eq!(t.agent_scope("Cast"), Some(Kind::Player));
        assert_eq!(t.agent_scope("Attack"), Some(Kind::Object));
        // Patient scopes ([CR#508.1b,509.1a,115.4]): attack reaches
        // players/planeswalkers/battles, block reaches attackers, targeting
        // reaches anything targetable.
        assert_eq!(t.patient_scope("Attack"), Some(Kind::Any));
        assert_eq!(t.patient_scope("Block"), Some(Kind::Object));
        assert_eq!(t.patient_scope("Target"), Some(Kind::Any));
        assert_eq!(t.patient_scope("Cast"), Some(Kind::Object));
        // Bind rules: the Delayed row drops targets and keeps That
        // ([CR#603.7c] via `unbindTargets`).
        let delayed = t.bind_rule("Delayed");
        assert!(delayed.drops_targets && delayed.keeps_that);
        assert_eq!(delayed.caps_from, CapsFrom::Query);
    }

    /// The anaphor-surface tables ride: sort compat (R1), the R2 loosening
    /// flag (calibrated FROZEN STRICT), zone sorts, intro rows,
    /// static-part classes, the event object-sort column.
    #[test]
    fn anaphor_tables_load() {
        let t = tables();
        assert!(
            !t.exact_sort_precedence(),
            "the R2 gate is FROZEN STRICT — calibrated by the corpus dry-run \
             (0 fires / 5785 faces, 0 mis-bindings in the 200-face audit); \
             flipping the flag is a reviewed event"
        );
        assert_eq!(t.sort_compat("Card", "Card"), Some(false));
        assert_eq!(t.sort_compat("Permanent", "OfType"), Some(true));
        assert_eq!(t.sort_compat("OfType", "OfType"), Some(false));
        assert_eq!(t.sort_compat("Card", "Token"), None, "tokens aren't cards");
        assert_eq!(t.sort_compat("StackObject", "Spell"), Some(true));
        assert_eq!(t.zone_sort(deckmaste_core::Zone::Battlefield), "Permanent");
        assert_eq!(t.zone_sort(deckmaste_core::Zone::Exile), "Card");
        assert_eq!(t.zone_sort(deckmaste_core::Zone::Stack), "Spell");
        let mv = t.intro("Move");
        assert!(mv.object && !mv.amount);
        assert_eq!(mv.site, IntroSite::Product);
        assert_eq!(mv.zone, "from_destination", "the [CR#603.7c] zone stamp");
        let draw = t.intro("Draw");
        assert!(
            draw.object && draw.amount,
            "draw pushes cards AND an amount"
        );
        assert_eq!(draw.card, IntroCard::FromCount);
        assert_eq!(t.static_class("Modify"), SetClass::GatherOnce);
        assert_eq!(t.static_class("Deontic"), SetClass::GatherLive);
        assert_eq!(t.event_object_sort("ZoneChange"), "to_zone");
        assert_eq!(t.event_object_sort("Cast"), "spell");
        assert_eq!(t.event_object_sort("Bogus"), "none");
        // MayPay's new bind row: the "if they do" branch reads payment caps.
        assert_eq!(t.bind_rule("MayPay").caps_from, CapsFrom::Cost);
    }

    /// The emitted join table IS the Idris `\/` semilattice: identity on
    /// `Empty`, idempotent, and object|player widens to `Any` ([CR#115.4]).
    #[test]
    fn join_matches_the_lattice_laws() {
        let t = tables();
        for k in [Kind::Empty, Kind::Object, Kind::Player, Kind::Any] {
            assert_eq!(t.join(Kind::Empty, k), k, "Empty is the identity");
            assert_eq!(t.join(k, Kind::Empty), k, "Empty is the identity");
            assert_eq!(t.join(k, k), k, "join is idempotent");
        }
        assert_eq!(t.join(Kind::Object, Kind::Player), Kind::Any);
        assert_eq!(t.join(Kind::Player, Kind::Object), Kind::Any);
        assert_eq!(t.join(Kind::Object, Kind::Any), Kind::Any);
    }
}
