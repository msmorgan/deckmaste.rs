//! The deontic layer ([CR#101.2,601.3]): clauses gating whether a *proposed
//! action* is legal. Boundary ([CR#614.17]): "doesn't"/"skip"/can't-happen
//! over events are replacement-family, never deontic; outcome "can't"s
//! ([CR#104]) are SBA-override machinery, also not here.

use std::sync::Arc;

use serde::Deserialize;
use serde::Serialize;

use crate::CostComponent;
use crate::Count;
use crate::Predicate;
use crate::Timing;
use crate::Zone;

/// An alternative base cost on a cast-permission row ([CR#118.9]): paid
/// INSTEAD of the mana cost, chosen at announce ([CR#601.2b]); only one
/// alternative may apply per spell ([CR#118.9a] — an engine rule, not
/// encodable here). Additional costs, increases, and reductions apply on
/// top ([CR#118.9d]); an alternative unlocks an unpayable base
/// ([CR#118.6a]).
#[derive(Debug, Clone, PartialEq, Eq, Hash, Deserialize, Serialize)]
pub enum AlternativeCost {
    /// "Without paying its mana cost" — the limiting case ([CR#118.9]).
    Free,
    /// "You may pay [cost] rather than …" ([CR#118.9b]).
    Components(Arc<[CostComponent]>),
}

/// A predicate over an ability's activation COST, carried by
/// [`DeonticAction::Activate`] to scope a `Cant(Activate)` row. `None` = any
/// activation; the sole variant matches a `{T}`/`{Q}` cost — the summoning-
/// sickness tap gate ([CR#602.5a]).
#[derive(Debug, Clone, PartialEq, Eq, Hash, Deserialize, Serialize)]
pub enum CostPredicate {
    /// The cost includes a tap (`{T}`) or untap (`{Q}`) symbol ([CR#602.5a]).
    IncludesTapSymbol,
}

/// A scoped counterfactual premise ([CR#609.4]): "treat the game as if
/// [premise] were true, for purposes of the named action's legality only."
/// Carried by `StaticSpec::AsThough`. The mana counterfactual has its own
/// channel (`StaticSpec::SpendAsThough`, [CR#609.4b]).
#[derive(Debug, Clone, PartialEq, Eq, Hash, serde::Deserialize, serde::Serialize)]
pub enum AsThough {
    /// A per-checker counterfactual overlay. When the engine checks the
    /// legality of the action named by `then` (a `May` selector: its action +
    /// `by` agent + `on` object-scope), each candidate object is evaluated *as
    /// though it satisfied `premise`* — realized by adding/removing the KEYWORD
    /// named in `premise` on that candidate, for that check only, then running
    /// the UNCHANGED legality check so the keyword's OWN row does the work. No
    /// real characteristic change (unlike a lose-ability layer effect), and the
    /// counterfactual is invisible to every other check.
    ///
    /// * `premise = Not(Has(Hexproof))` — remove the keyword: its
    ///   `Cant(Target)` obstacle vanishes, so the object becomes targetable by
    ///   `then`'s agent (Glaring Spotlight, [CR#702.11d] seen through).
    /// * `premise = Has(Flash)` — add the keyword: *Flash's own*
    ///   `May(Cast(InstantSpeed))` row appears, lifting cast timing (Leyline of
    ///   Anticipation). The keyword is INVOKED, never inlined.
    ///
    /// Mirrors Idris `AsThough : Condition -> StaticSpec -> StaticSpec`:
    /// `AsThough (Matches This premise) then`. `then` is `Arc`-boxed to keep
    /// this variant small (`clippy::large_enum_variant`).
    Counterfactual {
        premise: Predicate,
        then: Arc<Deontic>,
    },
}

/// A cardinality bound on a matched set — comparator-headed so it reads as
/// prose (`Less(Literal(2))` = "fewer than two"). A set-level predicate,
/// deliberately NOT a `Predicate` atom: filters judge one object at a time,
/// bounds judge the arrangement ([CR#702.111b] menace; deontics set-level
/// evaluation).
#[derive(Debug, Clone, PartialEq, Eq, Hash, Deserialize, Serialize)]
pub enum CountBound {
    Eq(Count),
    AtLeast(Count),
    AtMost(Count),
    Greater(Count),
    Less(Count),
}

impl CountBound {
    /// The comparator this bound heads with, and the `Count` it bounds
    /// against — comparator-headed variants map 1:1 onto [`Cmp`].
    #[must_use]
    pub fn split(&self) -> (crate::Cmp, &Count) {
        match self {
            CountBound::Eq(c) => (crate::Cmp::Eq, c),
            CountBound::AtLeast(c) => (crate::Cmp::AtLeast, c),
            CountBound::AtMost(c) => (crate::Cmp::AtMost, c),
            CountBound::Greater(c) => (crate::Cmp::Greater, c),
            CountBound::Less(c) => (crate::Cmp::Less, c),
        }
    }

    /// Does a matched set of cardinality `actual` satisfy this bound? `eval`
    /// is the engine's unified count evaluator, supplied by the caller to turn
    /// the bound's `Count` into a concrete cardinality — keeping this core
    /// predicate independent of the engine while still routing through the one
    /// evaluator (`GameState::eval_count`).
    pub fn satisfied_by(
        &self,
        actual: crate::Uint,
        eval: impl FnOnce(&Count) -> crate::Uint,
    ) -> bool {
        let (cmp, bound) = self.split();
        cmp.apply(actual, eval(bound))
    }
}

/// The TWO-SLOT agent of a stack-object deed ([CR#702.11d,702.16b] — the
/// CR's two-armed "…[quality] spells … or abilities … from [quality]
/// sources" wording as two slots). Both present arms must hold
/// (conjunction):
///
/// * `stack_object` judges the acting stack object itself — shroud's "spells or
///   abilities" is the match-anything default ([CR#702.18a]), hexproof's
///   opponent clause is `ControlledBy(OpponentOf(Ref(You)))` ([CR#702.11b]).
/// * `source` judges the acting object's SOURCE: an ability's source is the
///   object that generated it ([CR#113.7]); a spell is itself a source
///   ([CR#609.7a] lists "a spell on the stack" among sources), so ONE quality
///   on this arm covers both CR arms at once — hexproof-from-red is `source:
///   ColorIs(Red)` ([CR#702.11d]: red spells, or abilities from red sources),
///   protection's targeted clause is `source: Param(quality)` ([CR#702.16b]).
///
/// The empty agent (neither arm) is spellable but meaningless — ill-formed,
/// so the Idris re-emit gate rejects it. Omitting the whole slot
/// yields the shroud default (`stack_object: Any`).
#[derive(Debug, Clone, PartialEq, Eq, Hash, Deserialize, Serialize)]
pub struct DeedAgent {
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub stack_object: Option<Predicate>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub source: Option<Predicate>,
}

impl Default for DeedAgent {
    /// Any spell or ability ([CR#702.18a] shroud's agent): the
    /// `stack_object` arm present and match-anything.
    fn default() -> Self {
        DeedAgent {
            stack_object: Some(Predicate::Any),
            source: None,
        }
    }
}

impl DeedAgent {
    /// Whether this is the omitted-slot default (any spell or ability) — the
    /// write side's `skip_serializing_if`.
    #[must_use]
    pub fn is_default(&self) -> bool {
        *self == DeedAgent::default()
    }

    /// Whether NO arm is present — the ill-formed shape the Idris re-emit
    /// gate rejects (an agent that constrains nothing matches nothing
    /// meaningfully).
    #[must_use]
    pub fn is_empty(&self) -> bool {
        self.stack_object.is_none() && self.source.is_none()
    }
}

/// A proposed-action pattern — the typed verb the deontic polarities range
/// over. This is a closed core enum that the engine pattern-matches; semantic
/// lowering resolves the open authored forms into it. Slots default to
/// match-anything; the carrier side is spelled `Ref(This)`.
#[derive(Debug, Clone, PartialEq, Eq, Hash, serde::Deserialize, serde::Serialize)]
pub enum DeonticAction {
    /// `by` attacks `on` — a player/planeswalker/battle set
    /// ([CR#508.1a..508.1d]).
    Attack {
        #[serde(default = "Predicate::any")]
        by: Predicate,
        #[serde(default = "Predicate::any")]
        on: Predicate,
    },
    /// `by` blocks `on`. `count` bounds the matched blocking arrangement,
    /// and which slot carries `Ref(This)` decides the reading: anchored on
    /// `on` it bounds the blocker set per blocked creature (menace,
    /// [CR#702.111b]) — also the reading when neither slot is anchored —
    /// while anchored on `by` it bounds how many creatures the blocker
    /// blocks ([CR#509.1a..509.1c]).
    Block {
        #[serde(default = "Predicate::any")]
        by: Predicate,
        #[serde(default = "Predicate::any")]
        on: Predicate,
        #[serde(default, skip_serializing_if = "Option::is_none")]
        count: Option<CountBound>,
    },
    /// `by` (a stack object, through the two-slot [`DeedAgent`]) targets
    /// `on` ([CR#115.1,601.2c]). The agent defaults to any spell or ability
    /// — shroud is `Cant(Target(on: Ref(This)))` ([CR#702.18a]).
    Target {
        #[serde(default, skip_serializing_if = "DeedAgent::is_default")]
        by: DeedAgent,
        #[serde(default = "Predicate::any")]
        on: Predicate,
    },
    /// `what` is attached to `to` ([CR#701.3a] legality — "can't be
    /// attached" rows; protection's can't-be-enchanted/equipped clauses
    /// [CR#702.16c], enchant's quality restriction [CR#702.5a]). An
    /// illegal existing attachment is the SBA's business
    /// ([CR#704.5m..704.5n]), not this row's.
    Attach {
        #[serde(default = "Predicate::any")]
        what: Predicate,
        #[serde(default = "Predicate::any")]
        to: Predicate,
    },
    /// `by` casts `what`, optionally from a zone / in a window
    /// ([CR#601.3,701.5]).
    Cast {
        #[serde(default = "Predicate::any")]
        what: Predicate,
        #[serde(default = "Predicate::any")]
        by: Predicate,
        #[serde(default, skip_serializing_if = "Option::is_none")]
        from: Option<Zone>,
        #[serde(default, skip_serializing_if = "Option::is_none")]
        window: Option<Timing>,
        /// An alternative base cost the permission carries ([CR#118.9]) —
        /// "you may cast … without paying its mana cost".
        #[serde(default, skip_serializing_if = "Option::is_none")]
        cost: Option<AlternativeCost>,
        /// Names the alternative cost this permission carries, so a rider
        /// can ask `CastWith(tag)` — "if its flashback/evoke cost was paid"
        /// ([CR#702.34a,702.74a]). Mirrors Idris `MayCastFor.tag`. The
        /// alt-cost twin of `OptionalCost.tag`.
        #[serde(default, skip_serializing_if = "Option::is_none")]
        tag: Option<crate::CostTag>,
    },
    /// `by` plays `what` — land plays / play-a-card permissions
    /// ([CR#701.18]).
    Play {
        #[serde(default = "Predicate::any")]
        what: Predicate,
        #[serde(default = "Predicate::any")]
        by: Predicate,
        #[serde(default, skip_serializing_if = "Option::is_none")]
        from: Option<Zone>,
    },
    /// `by` activates abilities of `what` ([CR#602.1]), optionally cost-scoped
    /// ([CR#602.5a]).
    Activate {
        #[serde(default = "Predicate::any")]
        what: Predicate,
        #[serde(default = "Predicate::any")]
        by: Predicate,
        /// Restricts the row to activations whose cost matches ([CR#602.5a]);
        /// `None` = any. Additive: existing authorings default `None`.
        #[serde(default, skip_serializing_if = "Option::is_none")]
        cost: Option<CostPredicate>,
    },
    /// `by` regenerates `on` ([CR#701.19]) — the deed behind "can't be
    /// regenerated": a `Cant` row over it causes regeneration shields not
    /// to be APPLIED to `on` ([CR#701.19c] — activating/casting the
    /// shield-creators stays legal). Mirrors the Idris
    /// `Relation.Regenerate`. The `DestroyNoRegen` macro scopes one to its
    /// own destroy event via `Duration::ForThisEvent`; engine enforcement
    /// (the shield-application pass consulting deontic rows) is a seam.
    Regenerate {
        #[serde(default = "Predicate::any")]
        by: Predicate,
        #[serde(default = "Predicate::any")]
        on: Predicate,
    },
    /// `by` (a stack object — a spell or ability, the countering source)
    /// counters `on` (the spell/ability on the stack being countered)
    /// ([CR#701.6a]). The deed behind "can't be countered": a `Cant` row
    /// over it — `Cant(Counter(on: Ref(This)))` — causes the countering
    /// instruction not to affect `on`. Both slots default to match-anything;
    /// the agent needs no restriction for the common self-referential form
    /// (only spells/abilities counter anything). Mirrors the Idris
    /// `Relation.Counter` (agent `AnObject`, patient `AnObject`).
    Counter {
        #[serde(default = "Predicate::any")]
        by: Predicate,
        #[serde(default = "Predicate::any")]
        on: Predicate,
    },
    /// `what` doesn't untap ([CR#502.3] "effects can keep one or more of a
    /// player's permanents from untapping"). A `Cant` row over it —
    /// `Cant(Untap(what: Ref(This)))` ("~ doesn't untap during your untap
    /// step") or `Cant(Untap(what: <enchanted>))` ("enchanted creature
    /// doesn't untap during its controller's untap step") — makes the untap
    /// step's turn-based action ([CR#502.3]) not untap the matching
    /// permanent. ONE patient slot: untapping is a turn-based action, not a
    /// deed some object performs, so there is no agent slot. The untap step
    /// only processes the active player's permanents, so this is inherently
    /// scoped to "its controller's untap step". The one-shot "next untap
    /// step" flavor ([CR#701.43a] exert; the temple/painland mana riders) is
    /// a consumed per-object rider (`GameObject.skip_next_untap`), NOT this
    /// continuous row.
    Untap {
        #[serde(default = "Predicate::any")]
        what: Predicate,
    },
}

/// A deontic clause over a proposed action ([CR#101.2,601.3]).
#[derive(Debug, Clone, PartialEq, Eq, Hash, serde::Deserialize, serde::Serialize)]
pub enum Deontic {
    /// Permission row — the existential floor; granted rows widen the
    /// standing defaults ([CR#117.1a..117.1c,508.1a,509.1a], engine-side).
    May(DeonticAction),
    /// Prohibition; beats May ([CR#101.2]).
    Cant(DeonticAction),
    /// Requirement — if-able, arbitrated by the maximize solver
    /// ([CR#508.1d,509.1c]); never forces a toll payment.
    Must(DeonticAction),
    /// Gate — the declaration-gating price ([CR#508.1d,509.1c]): the
    /// action is legal only in completions paying the cost, once per
    /// matching instance; the actor is never compelled. Resolution Tolls
    /// (ward, [CR#702.21a,118.12a]) are trigger + unless, not a `Deontic`.
    Gate(DeonticAction, Arc<[CostComponent]>),
}
