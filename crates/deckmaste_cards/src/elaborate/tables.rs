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
    /// disjunct supplies (the Idris `andCaps`; the patient kind survives only
    /// when both sides fix the same one).
    #[must_use]
    pub fn meet(self, other: Caps) -> Caps {
        Caps {
            object: self.object && other.object,
            actor: self.actor && other.actor,
            amount: self.amount && other.amount,
            patient: same_kind(self.patient, other.patient),
            defender: self.defender && other.defender,
        }
    }

    /// The union — a composite cost's payment binds what ANY component binds
    /// (the Idris `orCaps`); the patient kind still needs agreement.
    #[must_use]
    pub fn join(self, other: Caps) -> Caps {
        Caps {
            object: self.object || other.object,
            actor: self.actor || other.actor,
            amount: self.amount || other.amount,
            patient: same_kind(self.patient, other.patient),
            defender: self.defender || other.defender,
        }
    }
}

/// The Idris `sameKind`: a patient kind survives combination only when both
/// sides fix the same kind.
fn same_kind(a: Option<Kind>, b: Option<Kind>) -> Option<Kind> {
    match (a, b) {
        (Some(x), Some(y)) if x == y => Some(x),
        _ => None,
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
    cost_actions: HashMap<String, Caps>,
    counter_default: Kind,
    counter_scopes: HashMap<String, Kind>,
    designation_scopes: HashMap<String, Kind>,
    agent_scopes: HashMap<String, Kind>,
    joins: HashMap<(Kind, Kind), Kind>,
    bind_rules: HashMap<String, BindRule>,
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
        let rules: RulesFile = parse(
            "checker-rules",
            include_str!("../../tables/checker-rules.ron"),
        );
        Tables {
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
            t.event_caps("ZoneMove").object,
            "a zone move binds its object"
        );
        assert!(
            !t.event_caps("ZoneMove").actor,
            "a bare zone move has no responsible player"
        );
        let damage = t.event_caps("Performed:DealDamage");
        assert!(damage.object && damage.actor && damage.amount);
        assert_eq!(
            t.event_caps("Bogus"),
            NO_CAPS,
            "unknown keys supply nothing"
        );
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
        // Bind rules: the Delayed row drops targets and keeps That
        // ([CR#603.7c] via `unbindTargets`).
        let delayed = t.bind_rule("Delayed");
        assert!(delayed.drops_targets && delayed.keeps_that);
        assert_eq!(delayed.caps_from, CapsFrom::Query);
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
