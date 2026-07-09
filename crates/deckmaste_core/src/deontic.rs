//! The deontic layer ([CR#101.2,601.3]): clauses gating whether a *proposed
//! action* is legal. Boundary ([CR#614.17]): "doesn't"/"skip"/can't-happen
//! over events are replacement-family, never deontic; outcome "can't"s
//! ([CR#104]) are SBA-override machinery, also not here.

use serde::Deserialize;
use serde::Serialize;

use crate::CostComponent;
use crate::Count;
use crate::Expand;
use crate::Expansion;
use crate::Predicate;
use crate::SupportsMacros;
use crate::Timing;
use crate::Zone;

/// An alternative base cost on a cast-permission row ([CR#118.9]): paid
/// INSTEAD of the mana cost, chosen at announce ([CR#601.2b]); only one
/// alternative may apply per spell ([CR#118.9a] — an engine rule, not
/// encodable here). Additional costs, increases, and reductions apply on
/// top ([CR#118.9d]); an alternative unlocks an unpayable base
/// ([CR#118.6a]).
#[derive(Debug, Clone, PartialEq, Eq, Hash, Deserialize, Expand, Serialize)]
pub enum AlternativeCost {
    /// "Without paying its mana cost" — the limiting case ([CR#118.9]).
    Free,
    /// "You may pay [cost] rather than …" ([CR#118.9b]).
    Components(Vec<CostComponent>),
}

/// A scoped counterfactual premise ([CR#609.4]): "treat the game as if
/// [premise] were true, for purposes of that effect only." Carried by
/// `StaticEffect::AsThough`. Many as-though cards compile to deontic rows
/// instead (cast-as-though-flash = `May(Cast(window: InstantSpeed))`), and
/// the mana counterfactual has its own channel
/// (`StaticEffect::SpendAsThough`, [CR#609.4b]); the variants here are the
/// residue. Premises accrete as cards demand them.
#[derive(Debug, Clone, PartialEq, Eq, Hash, SupportsMacros)]
pub enum AsThough {
    /// A remembered `AsThough` macro invocation. Serialized as the
    /// invocation, not the struct.
    #[macro_ron(expanded)]
    Expanded(Expansion<AsThough>),
}

/// A cardinality bound on a matched set — comparator-headed so it reads as
/// prose (`Less(Literal(2))` = "fewer than two"). A set-level predicate,
/// deliberately NOT a `Predicate` atom: filters judge one object at a time,
/// bounds judge the arrangement ([CR#702.111b] menace; deontics set-level
/// evaluation).
#[derive(Debug, Clone, PartialEq, Eq, Hash, Deserialize, Expand, Serialize)]
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
#[derive(Debug, Clone, PartialEq, Eq, Hash, Deserialize, Expand, Serialize)]
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
/// over. Closed core enum (the engine pattern-matches it); openness comes
/// from macro interception at the `Deontic`/`Ability` positions. Slots
/// default to match-anything; the carrier side is spelled `Ref(This)`.
///
/// Both serde impls are generated by `#[derive(SupportsMacros)]`: struct
/// variants read flat in RON through generated helper structs (carrying the
/// forwarded `#[serde(...)]` field defaults) + `unwrap_variant_newtypes`.
#[derive(Debug, Clone, PartialEq, Eq, Hash, SupportsMacros)]
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
    /// `by` activates abilities of `what` ([CR#602.1]).
    Activate {
        #[serde(default = "Predicate::any")]
        what: Predicate,
        #[serde(default = "Predicate::any")]
        by: Predicate,
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
    /// A remembered `DeonticAction` macro invocation. Serialized as the
    /// invocation, not the struct.
    #[macro_ron(expanded)]
    Expanded(Expansion<DeonticAction>),
}

/// A deontic clause over a proposed action ([CR#101.2,601.3]).
#[derive(Debug, Clone, PartialEq, Eq, Hash, SupportsMacros)]
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
    Gate(DeonticAction, Vec<CostComponent>),
    /// A remembered `Deontic` macro invocation (evasion keywords, …).
    #[macro_ron(expanded)]
    Expanded(Expansion<Deontic>),
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::Reference;

    fn read(source: &str) -> Deontic {
        crate::ron::options().from_str(source).unwrap()
    }

    /// Omitted slots default to `Any` (load-bearing serde defaults, like
    /// the `EventFilter` master forms').
    #[test]
    fn slots_default_to_any() {
        assert_eq!(
            read("Cant(Attack(by: Ref(This)))"),
            Deontic::Cant(DeonticAction::Attack {
                by: Predicate::Ref(Reference::This),
                on: Predicate::Any,
            }),
        );
    }

    /// `satisfied_by` routes the matched-set cardinality and the bound's
    /// evaluated `Count` through the right comparator — and feeds the closure
    /// the BOUND, not the actual. Menace (`Less(2)`) forbids a non-empty set
    /// of fewer than two: one blocker is too few, two is enough.
    #[test]
    fn count_bound_satisfied_by_routes_through_comparator() {
        let menace = CountBound::Less(Count::Literal(2));
        // The closure must receive the bound (Literal(2)), returning 2.
        let eval = |c: &Count| match c {
            Count::Literal(n) => *n,
            other => panic!("unexpected count {other:?}"),
        };
        assert!(
            menace.satisfied_by(1, eval),
            "1 < 2 → bound holds (too few)"
        );
        assert!(!menace.satisfied_by(2, eval), "2 < 2 is false");
        // Every comparator variant maps correctly against a fixed bound of 2.
        assert!(CountBound::Eq(Count::Literal(2)).satisfied_by(2, eval));
        assert!(CountBound::AtLeast(Count::Literal(2)).satisfied_by(2, eval));
        assert!(CountBound::AtMost(Count::Literal(2)).satisfied_by(2, eval));
        assert!(CountBound::Greater(Count::Literal(2)).satisfied_by(3, eval));
        assert!(!CountBound::Greater(Count::Literal(2)).satisfied_by(2, eval));
    }

    /// Menace's shape: a Cant over too-small blocker sets.
    #[test]
    fn block_count_bound_reads() {
        assert_eq!(
            read("Cant(Block(on: Ref(This), count: Less(Literal(2))))"),
            Deontic::Cant(DeonticAction::Block {
                by: Predicate::Any,
                on: Predicate::Ref(Reference::This),
                count: Some(CountBound::Less(Count::Literal(2))),
            }),
        );
    }

    /// The Flash keyword's expansion target.
    #[test]
    fn flash_permission_row_reads() {
        assert_eq!(
            read("May(Cast(what: Ref(This), window: InstantSpeed))"),
            Deontic::May(DeonticAction::Cast {
                what: Predicate::Ref(Reference::This),
                by: Predicate::Any,
                from: None,
                window: Some(Timing::InstantSpeed),
                cost: None,
                tag: None,
            }),
        );
    }

    /// The old MayCastFrom(Zone) row in decomposed spelling.
    #[test]
    fn cast_from_zone_reads() {
        assert_eq!(
            read("May(Cast(what: Ref(This), from: Graveyard))"),
            Deontic::May(DeonticAction::Cast {
                what: Predicate::Ref(Reference::This),
                by: Predicate::Any,
                from: Some(Zone::Graveyard),
                window: None,
                cost: None,
                tag: None,
            }),
        );
    }

    /// `Cast(… tag: Flashback)` names the alt cost the permission carries
    /// ([CR#702.34a,702.74a]) — reads with a bare-ident tag, round-trips, and
    /// an omitted tag stays absent (like the sibling `cost`/`from` fields).
    #[test]
    fn cast_tag_reads_and_round_trips() {
        let flashback = read("May(Cast(what: Ref(This), tag: Flashback))");
        assert_eq!(
            flashback,
            Deontic::May(DeonticAction::Cast {
                what: Predicate::Ref(Reference::This),
                by: Predicate::Any,
                from: None,
                window: None,
                cost: None,
                tag: Some(crate::CostTag::from("Flashback")),
            }),
        );
        let written = crate::ron::options().to_string(&flashback).unwrap();
        assert!(
            written.contains("tag:Flashback"),
            "tag writes as a bare ident: {written}"
        );
        assert_eq!(read(&written), flashback);

        // Omitted tag stays absent, like the sibling `cost`/`from` fields.
        let untagged = read("May(Cast(what: Ref(This)))");
        let written = crate::ron::options().to_string(&untagged).unwrap();
        assert!(!written.contains("tag"), "default tag omitted: {written}");
        assert_eq!(read(&written), untagged);
    }

    /// The two-slot deed agent ([CR#702.11d,702.16b]): the omitted slot is
    /// the shroud default (any spell or ability, [CR#702.18a]) and is
    /// omitted on write; the hexproof-from shape sets both arms; the EMPTY
    /// agent is spellable (the Idris re-emit gate rejects it, not serde).
    #[test]
    fn deed_agent_two_slots_read_and_round_trip() {
        use crate::CharacteristicPredicate;
        use crate::Color;

        // Shroud: `by` omitted → the any-stack-object default, not written.
        let shroud = read("Cant(Target(on: Ref(This)))");
        assert_eq!(
            shroud,
            Deontic::Cant(DeonticAction::Target {
                by: DeedAgent::default(),
                on: Predicate::Ref(Reference::This),
            }),
        );
        let written = crate::ron::options().to_string(&shroud).unwrap();
        assert!(!written.contains("by"), "default agent omitted: {written}");
        assert_eq!(read(&written), shroud);

        // Hexproof-from-red: opponent clause on the stack object, the
        // quality on the source ([CR#702.11d]).
        let hexproof = read(
            "Cant(Target(on: Ref(This), by: (stack_object: ControlledBy(OpponentOf(Ref(You))), source: ColorIs(Red))))",
        );
        let Deontic::Cant(DeonticAction::Target { by, .. }) = &hexproof else {
            panic!("expected Cant(Target), got {hexproof:?}");
        };
        assert!(by.stack_object.is_some() && by.source.is_some());
        assert_eq!(
            by.source,
            Some(Predicate::Characteristic(CharacteristicPredicate::ColorIs(
                Color::Red
            ))),
        );
        let written = crate::ron::options().to_string(&hexproof).unwrap();
        assert_eq!(read(&written), hexproof);

        // The empty agent parses (both arms absent) — refusing it is the
        // Idris re-emit gate's job, not serde's.
        let empty = read("Cant(Target(on: Ref(This), by: ()))");
        let Deontic::Cant(DeonticAction::Target { by, .. }) = &empty else {
            panic!("expected Cant(Target), got {empty:?}");
        };
        assert!(by.is_empty());
    }

    /// "This spell can't be countered" ([CR#701.6a]): a `Cant(Counter)` over
    /// the self-referential patient, the agent left at the any-source default.
    #[test]
    fn cant_be_countered_reads() {
        let clause = read("Cant(Counter(on: Ref(This)))");
        assert_eq!(
            clause,
            Deontic::Cant(DeonticAction::Counter {
                by: Predicate::Any,
                on: Predicate::Ref(Reference::This),
            }),
        );
        // Round-trips through serialize → read.
        let written = crate::ron::options().to_string(&clause).unwrap();
        assert_eq!(read(&written), clause);
    }

    /// A declaration Gate (Propaganda-shaped, cost simplified to {T}).
    #[test]
    fn declaration_gate_reads() {
        assert_eq!(
            read("Gate(Attack(on: Ref(You)), [Tap])"),
            Deontic::Gate(
                DeonticAction::Attack {
                    by: Predicate::Any,
                    on: Predicate::Ref(Reference::You),
                },
                vec![CostComponent::Tap],
            ),
        );
    }

    /// An unknown name at a `Deontic` position is a hard error, not a
    /// silent fallthrough (the macro layer's type-safety seam).
    #[test]
    fn unknown_names_error() {
        assert!(
            crate::ron::options()
                .from_str::<Deontic>("Bogus(1)")
                .is_err()
        );
    }

    /// The block-restriction shapes the static-ability parser emits read with
    /// BARE-numeral count bounds (`Greater(1)`, `AtMost(2)`) through the
    /// literal-aware core reader — no `Literal(…)` wrapper.
    #[test]
    fn block_restriction_bare_count_bounds_read() {
        assert_eq!(
            read("Cant(Block(on: Ref(This), count: Greater(1)))"),
            Deontic::Cant(DeonticAction::Block {
                by: Predicate::Any,
                on: Predicate::Ref(Reference::This),
                count: Some(CountBound::Greater(Count::Literal(1))),
            }),
        );
        assert_eq!(
            read("May(Block(by: Ref(This), count: AtMost(2)))"),
            Deontic::May(DeonticAction::Block {
                by: Predicate::Ref(Reference::This),
                on: Predicate::Any,
                count: Some(CountBound::AtMost(Count::Literal(2))),
            }),
        );
    }

    /// Serialize → read returns the same value for each polarity shape.
    #[test]
    fn deontic_round_trips() {
        let cases = [
            "Cant(Attack(by: Ref(This)))",
            "Cant(Block(on: Ref(This), count: Less(Literal(2))))",
            "Cant(Block(on: Ref(This), count: Greater(1)))",
            "May(Block(by: Ref(This), count: AtMost(2)))",
            "May(Cast(what: Ref(This), window: InstantSpeed))",
            "Gate(Attack(on: Ref(You)), [Tap])",
        ];
        for source in cases {
            let parsed = read(source);
            let written = crate::ron::options().to_string(&parsed).unwrap();
            assert_eq!(read(&written), parsed, "round-trip failed: {source}");
        }
    }
}
