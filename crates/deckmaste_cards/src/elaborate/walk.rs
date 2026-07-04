//! The elaborator's tree walk: one pass over a card/token with a running
//! binding context ([`Ctx`]), pushing an [`ElabError`] for every rule
//! violation. Context transitions are applied through the emitted bind-rule
//! rows ([`tables::BindRule`]); event/cost capabilities, carrier scopes, and
//! the kind lattice come from the emitted tables too — the walker contributes
//! tree shape, never rule values.

use std::collections::BTreeSet;

use deckmaste_core::Ability;
use deckmaste_core::Action;
use deckmaste_core::AlternativeCost;
use deckmaste_core::Anchor;
use deckmaste_core::AsThough;
use deckmaste_core::Binder;
use deckmaste_core::Card;
use deckmaste_core::CardFace;
use deckmaste_core::CharacteristicFilter;
use deckmaste_core::Condition;
use deckmaste_core::Cost;
use deckmaste_core::CostChange;
use deckmaste_core::CostComponent;
use deckmaste_core::Count;
use deckmaste_core::CountBound;
use deckmaste_core::CounterRef;
use deckmaste_core::CounterScope;
use deckmaste_core::CounterSpec;
use deckmaste_core::DeedAgent;
use deckmaste_core::Deontic;
use deckmaste_core::DeonticAction;
use deckmaste_core::DesignationDecl;
use deckmaste_core::DesignationDef;
use deckmaste_core::DesignationScope;
use deckmaste_core::Destination;
use deckmaste_core::Duration;
use deckmaste_core::Effect;
use deckmaste_core::EventFilter;
use deckmaste_core::Filter;
use deckmaste_core::Ident;
use deckmaste_core::KeywordAbility;
use deckmaste_core::ManaProduction;
use deckmaste_core::ManaRider;
use deckmaste_core::ManaSymbol;
use deckmaste_core::Modification;
use deckmaste_core::Normalize;
use deckmaste_core::NotedKind;
use deckmaste_core::ParamShape;
use deckmaste_core::PlayerAction;
use deckmaste_core::PlayerMod;
use deckmaste_core::Prevention;
use deckmaste_core::Property;
use deckmaste_core::Quantity;
use deckmaste_core::Reference;
use deckmaste_core::RelationFilter;
use deckmaste_core::Replacement;
use deckmaste_core::Scope;
use deckmaste_core::Selection;
use deckmaste_core::Sort;
use deckmaste_core::StateChange;
use deckmaste_core::StateFilter;
use deckmaste_core::StaticAbility;
use deckmaste_core::StaticEffect;
use deckmaste_core::TargetSpec;
use deckmaste_core::Token;
use deckmaste_core::TokenSpec;
use deckmaste_core::TriggeredAbility;
use deckmaste_core::Type;

use super::Code;
use super::ElabError;
use super::Registries;
use super::tables;
use super::tables::Caps;
use super::tables::CapsFrom;
use super::tables::Cardinality;
use super::tables::Kind;
use super::tables::NO_CAPS;
use super::tables::Tables;

/// Which consumer position an event pattern sits in — the key into the
/// emitted lane table (`event-lanes.ron`, the plan's §3.2): each lane
/// admits a different slice of the event algebra.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
enum Lane {
    /// `Triggered.event` / `Reflexive` ([CR#603.2,603.12a]).
    Trigger,
    /// `Delayed.event` ([CR#603.7c]).
    Delayed,
    /// `Replacement.would` — Instead/Also ([CR#614.1]).
    Replacement,
    /// `StaticEffect::CantHappen` ([CR#614.17c]).
    CantHappen,
    /// `Duration::UntilEvent` ([CR#610.3]).
    UntilEvent,
    /// `TriggerMultiplier.cause` ([CR#603.2d]).
    TriggerMultiplier,
    /// History counting — `Happened`/`EventCount`/`EventSum` ([CR#608.2i]).
    History,
}

impl Lane {
    /// The emitted lane row's key.
    fn key(self) -> &'static str {
        match self {
            Lane::Trigger => "Triggered.event",
            Lane::Delayed => "Delayed.event",
            Lane::Replacement => "Replacement.would",
            Lane::CantHappen => "CantHappen",
            Lane::UntilEvent => "UntilEvent",
            Lane::TriggerMultiplier => "TriggerMultiplier.cause",
            Lane::History => "Happened",
        }
    }

    /// The lane a triggered-ability construct's event sits in.
    fn for_trigger_construct(construct: &str) -> Lane {
        if construct == "Delayed" { Lane::Delayed } else { Lane::Trigger }
    }
}

/// The event-caps table key of a MASTER form (`None` for the algebra tail) —
/// also the kind name the residual kind pass compares.
fn form_key(event: &EventFilter) -> Option<&'static str> {
    Some(match event {
        EventFilter::ZoneChange { .. } => "ZoneChange",
        EventFilter::Damage { .. } => "Damage",
        EventFilter::LifeGained { .. } => "LifeGained",
        EventFilter::LifeLost { .. } => "LifeLost",
        EventFilter::Drawn { .. } => "Drawn",
        EventFilter::CounterPlaced { .. } => "CounterPlaced",
        EventFilter::CounterRemoved { .. } => "CounterRemoved",
        EventFilter::Cast { .. } => "Cast",
        EventFilter::Played { .. } => "Played",
        EventFilter::ActivatedAb { .. } => "ActivatedAb",
        EventFilter::AttackDeclared { .. } => "AttackDeclared",
        EventFilter::BlockDeclared { .. } => "BlockDeclared",
        EventFilter::Attached { .. } => "Attached",
        EventFilter::StateBecame { becomes, .. } => match becomes {
            StateChange::Tapped => "StateBecame:Tapped",
            StateChange::Untapped => "StateBecame:Untapped",
            StateChange::Phased(_) => "StateBecame:Phased",
            StateChange::TurnedFace(_) => "StateBecame:TurnedFace",
        },
        EventFilter::BecomesTarget { .. } => "BecomesTarget",
        EventFilter::StepBegins { .. } => "StepBegins",
        EventFilter::ControlChanged { .. } => "ControlChanged",
        EventFilter::DesignationChanged { .. } => "DesignationChanged",
        EventFilter::TokenCreated { .. } => "TokenCreated",
        EventFilter::Used { .. } => "Used",
        EventFilter::CoinFlipped { .. } => "CoinFlipped",
        EventFilter::DiceRolled { .. } => "DiceRolled",
        EventFilter::BecameDay => "BecameDay",
        EventFilter::BecameNight => "BecameNight",
        EventFilter::AllOf(_)
        | EventFilter::OneOf(_)
        | EventFilter::Not(_)
        | EventFilter::OneOrMore(_)
        | EventFilter::Nth { .. }
        | EventFilter::When(..)
        | EventFilter::Within(..)
        | EventFilter::Expanded(_) => return None,
    })
}

/// The bridge-caps atom of a history window ([`deckmaste_core::Lookback`]) —
/// the log is turn-tagged, so sub-turn windows need combat/step markers it
/// doesn't record ([CR#608.2i]).
fn lookback_atom(within: deckmaste_core::Lookback) -> &'static str {
    use deckmaste_core::Lookback;
    match within {
        Lookback::ThisTurn => "Lookback:ThisTurn",
        Lookback::ThisGame => "Lookback:ThisGame",
        Lookback::LastTurn => "Lookback:LastTurn",
        Lookback::ThisCombat => "Lookback:ThisCombat",
        Lookback::ThisStep => "Lookback:ThisStep",
        Lookback::SinceYour(_) => "Lookback:SinceYour",
    }
}

/// Whether `Filter::Where` sits on the SPINE a snapshot-evaluated slot walks
/// (`ZoneChange.what` / `Played.what` — the moved object is gone,
/// [CR#603.10a]). Only the spine is snapshot-bound: the snapshot matcher
/// recurses through the logical combinators itself but resolves
/// relation-nested filters against LIVE objects (a controller's proxy, an
/// owner's proxy), where `Where` evaluates.
fn where_in_spine(filter: &Filter) -> bool {
    match filter {
        Filter::Where(_) => true,
        Filter::AllOf(fs) | Filter::OneOf(fs) => fs.iter().any(where_in_spine),
        Filter::Not(f) => where_in_spine(f),
        Filter::Expanded(e) => where_in_spine(&e.value),
        _ => false,
    }
}

/// Whether a filter is the match-anything default, looking through
/// remembered macros — the engine's `TokenCreated` arm makes the same read.
fn is_any_filter(filter: &Filter) -> bool {
    match filter {
        Filter::Any => true,
        Filter::Expanded(e) => is_any_filter(&e.value),
        _ => false,
    }
}

/// Looks through remembered `Reference` macros to the structural reference.
fn deref_reference(r: &Reference) -> &Reference {
    match r {
        Reference::Expanded(e) => deref_reference(&e.value),
        other => other,
    }
}

/// Whether a pattern is kind-ANCHORED — bottoms out in master forms
/// ([CR#603.2]; no freeze-everything `CantHappen`): a master form anchors;
/// a conjunction anchors if ANY conjunct does; a disjunction only if EVERY
/// disjunct does; refinement wrappers inherit; `Not` never anchors.
fn anchored(event: &EventFilter) -> bool {
    match event {
        EventFilter::AllOf(events) => events.iter().any(anchored),
        EventFilter::OneOf(events) => events.iter().all(anchored),
        EventFilter::Not(_) => false,
        EventFilter::OneOrMore(inner)
        | EventFilter::Nth { of: inner, .. }
        | EventFilter::When(inner, _)
        | EventFilter::Within(inner, _) => anchored(inner),
        EventFilter::Expanded(e) => anchored(&e.value),
        _master => true,
    }
}

/// The exchange-family macro names ([CR#701.12]) whose bodies may carry the
/// batch primitives `Effect::Simultaneous` and `Action::GainControl` — the
/// engine's batch wiring is exchange-only until it generalizes
/// (`E-POS-SIMULTANEOUS`; the load cap mirrors the E-BRIDGE-CAP discipline:
/// reject at load, never silently misexecute).
const EXCHANGE_FAMILY: &[&str] = &["ExchangeControl"];

/// Whether the pattern GUARANTEES a before/after totals channel — every
/// anchor is a counter-placement fact, the only kind carrying them
/// ([CR#714.2b,122.1]): a conjunction needs one such conjunct, a
/// disjunction needs every disjunct, refinement wrappers inherit, `Not`
/// guarantees nothing (mirrors `anchored`/`event_caps`).
fn fixes_crossing(event: &EventFilter) -> bool {
    match event {
        EventFilter::CounterPlaced { .. } => true,
        EventFilter::AllOf(events) => events.iter().any(fixes_crossing),
        EventFilter::OneOf(events) => !events.is_empty() && events.iter().all(fixes_crossing),
        EventFilter::OneOrMore(inner)
        | EventFilter::Nth { of: inner, .. }
        | EventFilter::When(inner, _)
        | EventFilter::Within(inner, _) => fixes_crossing(inner),
        EventFilter::Expanded(e) => fixes_crossing(&e.value),
        _ => false,
    }
}

/// Where an antecedent came from — the plan's §2.1 `Site`. Sites drive the
/// survival rules ([CR#603.7c] delayed bodies drop `TargetSlot`s, keep
/// `Product`s) and the runtime mapping (a `TargetSlot`-resolved anaphor is
/// the announced target; a `Product` is fact-backed).
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
enum Site {
    /// Pushed by an announced target slot ([CR#115.3,601.2c]).
    TargetSlot(usize),
    /// Pushed by a producing verb — backed by enacted facts at runtime
    /// ([CR#400.7j]).
    Product,
    /// A trigger/replacement/payment event role ([CR#603.2e,608.2k]).
    EventRole(Role),
    /// An `Each`/`DivideAmong`/`Where`/`Pick` element ([CR#608.2]) — `It`
    /// binds to the innermost of these deterministically.
    Loop,
    /// A `DivideAmong` share ([CR#601.2d]) — read only as
    /// `Count::Allotment`, never by the amount anaphors.
    Allot,
    /// A resolution-time choice — a `With` binder, `A(filter)`, a chosen
    /// pile ([CR#608.2d]).
    Chosen,
}

/// The event role an [`Site::EventRole`] antecedent carries.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
enum Role {
    Object,
    Patient,
    Actor,
    Amount,
    Defender,
}

/// One antecedent on the ordered stack — the plan's §2.1 `Ante`.
#[derive(Debug, Clone)]
struct Ante {
    /// The English noun ([`Sort`]) an anaphor must be compatible with (R1).
    sort: Sort,
    /// The reference kind the antecedent denotes ([CR#109.1]).
    kind: Kind,
    /// One object or a group ([CR#608.2d]).
    card: Cardinality,
    site: Site,
    /// The zone the producing clause put the object in — the [CR#603.7c]
    /// expected-zone stamp delayed bodies consume.
    expected_zone: Option<deckmaste_core::Zone>,
    /// The `Label { as, .. }` / pile-label name, when the antecedent is
    /// labeled — read explicitly as `The`/`TheGroup` (the R2 fallback).
    label: Option<Ident>,
    /// A BINDER FRAME: a `With` binder's choice or a chosen pile — the
    /// legacy deterministic scope. An anaphor inside the frame binds to it
    /// directly (innermost frame wins, never R2-gated), preserving the
    /// pre-stack `With`/`That` semantics; the telescope surface doesn't
    /// spell `With` at all.
    binder: bool,
}

impl Ante {
    /// A short human description for traces and error messages.
    fn describe(&self) -> String {
        use std::fmt::Write as _;
        let mut s = format!("{:?} {:?} at {:?}", self.card, self.sort, self.site);
        if let Some(zone) = self.expected_zone {
            let _ = write!(s, ", expected zone {zone:?}");
        }
        if let Some(label) = self.label {
            let _ = write!(s, ", label {:?}", label.as_str());
        }
        s
    }
}

/// The running binding context: the ordered ANTECEDENT STACK (nearest =
/// last), plus the positional discipline the plan's `Ctx` adds
/// (`may_target`, `has_x`, event caps for the explicit role reads, noted
/// keys). Subsumes the old role-named fields (`targets`/`that`/`it`/
/// `allotment`/`amount`) — those are now sites on the one stack.
#[expect(
    clippy::struct_excessive_bools,
    reason = "each flag is an independent binding-context dimension, not a state machine"
)]
#[derive(Debug, Clone)]
struct Ctx {
    /// The ordered antecedent stack, nearest LAST.
    stack: Vec<Ante>,
    /// The surrounding event's caps; [`NO_CAPS`] outside an event body.
    /// Gates the EXPLICIT event-role reads (`EventObject`/…); the stack's
    /// `EventRole` antecedents serve the anaphors.
    caps: Caps,
    /// Whether ANY event body encloses this position — splits `E-BIND-EVENT`
    /// (no event at all) from `E-CAPS-*` (an event that doesn't supply the
    /// role).
    in_event: bool,
    /// The surrounding event fixes a before/after totals channel — every
    /// anchor is a counter-placement fact ([CR#714.2b,122.1]); the
    /// `Condition::Crossed` read is legal only here.
    crossing: bool,
    /// Inside an exchange-family macro's body ([CR#701.12]) — the only
    /// position where `Simultaneous`/`GainControl` load
    /// (`E-POS-SIMULTANEOUS`).
    in_exchange_macro: bool,
    /// `{X}` is declared by the carrying cost ([CR#107.3]).
    has_x: bool,
    /// `Targeted` is legal here ([CR#115.1a..115.1e,601.2c]).
    may_target: bool,
    /// Note keys in scope, each with the domain its writer declared
    /// ([CR#607.2] — a linked reader refers only to the kind of
    /// information the writer noted).
    notes: Vec<(Ident, NotedKind)>,
}

impl Ctx {
    fn base(has_x: bool) -> Ctx {
        Ctx {
            stack: Vec::new(),
            caps: NO_CAPS,
            in_event: false,
            crossing: false,
            in_exchange_macro: false,
            has_x,
            may_target: false,
            notes: Vec::new(),
        }
    }

    /// The innermost BINDER — a loop element or a `With`/pile choice
    /// frame — `It`'s deterministic antecedent ([CR#608.2]; the pre-stack
    /// semantics, generalized: inside a binder, `It` is ALWAYS the bound
    /// thing, never gated by R2).
    fn innermost_binder(&self) -> Option<(usize, &Ante)> {
        self.stack
            .iter()
            .rev()
            .enumerate()
            .find(|(_, a)| a.site == Site::Loop || (a.binder && a.site == Site::Chosen))
    }

    /// The innermost `With`/pile CHOICE frame — the legacy deterministic
    /// scope the sorted anaphors bind inside ([CR#608.2d]; never R2-gated).
    fn innermost_frame(&self) -> Option<(usize, &Ante)> {
        self.stack
            .iter()
            .rev()
            .enumerate()
            .find(|(_, a)| a.binder && a.site == Site::Chosen)
    }
}

/// What a walked effect introduces for its FOLLOWING siblings (the
/// `Sequence` telescope, the plan's §2.1 `intro`): antecedent pushes and
/// noted keys.
#[derive(Debug, Default)]
struct Intro {
    pushes: Vec<Ante>,
    notes: Vec<(Ident, NotedKind)>,
}

impl Intro {
    fn absorb(&mut self, other: Intro) {
        self.pushes.extend(other.pushes);
        self.notes.extend(other.notes);
    }

    /// Applies this intro to a running context (the telescope step).
    fn apply_to(&self, ctx: &mut Ctx) {
        ctx.stack.extend(self.pushes.iter().cloned());
        ctx.notes.extend(self.notes.iter().copied());
    }
}

pub(super) fn card(card: &Card, registries: &Registries) -> Vec<ElabError> {
    walk_card(card, registries, false).0
}

/// Like [`card`], additionally collecting every resolved binding
/// ([`super::Resolution`]) — `cargo xtask elaborate --dump`.
pub(super) fn card_traced(
    card: &Card,
    registries: &Registries,
) -> (Vec<ElabError>, Vec<super::Resolution>) {
    walk_card(card, registries, true)
}

fn walk_card(
    card: &Card,
    registries: &Registries,
    trace: bool,
) -> (Vec<ElabError>, Vec<super::Resolution>) {
    let mut w = Walker::new(registries, trace);
    match card {
        Card::Normal(face) => w.scoped("face", |w| w.face(face)),
        Card::TwoFaced { front, back, .. } => {
            w.scoped("front", |w| w.face(front));
            w.scoped("back", |w| w.face(back));
        }
    }
    (w.errors, w.resolutions)
}

pub(super) fn token(token: &Token, registries: &Registries) -> Vec<ElabError> {
    walk_token(token, registries, false).0
}

/// Like [`token`], additionally collecting every resolved binding
/// ([`super::Resolution`]) — `cargo xtask elaborate --dump`.
pub(super) fn token_traced(
    token: &Token,
    registries: &Registries,
) -> (Vec<ElabError>, Vec<super::Resolution>) {
    walk_token(token, registries, true)
}

fn walk_token(
    token: &Token,
    registries: &Registries,
    trace: bool,
) -> (Vec<ElabError>, Vec<super::Resolution>) {
    let mut w = Walker::new(registries, trace);
    w.scoped("token", |w| w.token(token));
    (w.errors, w.resolutions)
}

struct Walker<'a> {
    tables: &'static Tables,
    registries: &'a Registries<'a>,
    errors: Vec<ElabError>,
    path: Vec<String>,
    /// Bridge-cap findings collected during one [`Self::event`] walk
    /// (`engine-eventfilter-bridge`): the cap speaks LAST — a pattern the
    /// grammar's own rules (lane, anchor, floor, kind) already refused
    /// never reaches the engine, so its caps are not reported.
    bridge_pending: Vec<(String, Lane)>,
    /// Whether to collect [`super::Resolution`]s (`--dump`) — the load
    /// gate's plain walk leaves this off so it never pays for the
    /// bookkeeping.
    trace: bool,
    resolutions: Vec<super::Resolution>,
    /// Antecedents pushed by REFERENCES inside the clause being walked
    /// (`A(filter)`'s Chosen push) — drained into the clause's [`Intro`] by
    /// the `Act` arm, dropped by non-clause positions.
    pending: Vec<Ante>,
}

impl<'a> Walker<'a> {
    fn new(registries: &'a Registries<'a>, trace: bool) -> Walker<'a> {
        Walker {
            tables: tables::tables(),
            registries,
            errors: Vec::new(),
            path: Vec::new(),
            bridge_pending: Vec::new(),
            trace,
            resolutions: Vec::new(),
            pending: Vec::new(),
        }
    }

    fn scoped(&mut self, segment: impl Into<String>, f: impl FnOnce(&mut Self)) {
        self.path.push(segment.into());
        f(self);
        self.path.pop();
    }

    fn err(&mut self, code: Code, message: impl Into<String>) {
        self.errors.push(ElabError {
            code,
            path: self.path.join("."),
            message: message.into(),
        });
    }

    /// Records a successfully-resolved binding when tracing (`--dump`); a
    /// no-op (not even formatting `f`'s message) otherwise.
    fn resolve(&mut self, f: impl FnOnce() -> String) {
        if self.trace {
            self.resolutions.push(super::Resolution {
                path: self.path.join("."),
                description: f(),
            });
        }
    }

    /// Applies a construct's emitted bind-rule row to the context, as STACK
    /// transitions. `targets` replaces the slot antecedents when the row
    /// binds targets; `bind` is the binder's antecedent (pushed at `Chosen`
    /// per `binds_that`, `Loop` per `binds_it`); `caps`+`roles` are consumed
    /// per the row's `caps_from` (the event/payment role antecedents).
    fn descend(
        &self,
        ctx: &Ctx,
        construct: &str,
        targets: Option<Vec<Ante>>,
        bind: Option<Ante>,
        caps: Caps,
        roles: Vec<Ante>,
    ) -> Ctx {
        let rule = self.tables.bind_rule(construct);
        let mut next = ctx.clone();
        if rule.drops_targets {
            // [CR#603.7c]: delayed bodies drop the announced targets;
            // Product antecedents (with their expected zones) survive.
            next.stack
                .retain(|a| !matches!(a.site, Site::TargetSlot(_)));
        }
        if rule.binds_targets
            && let Some(slots) = targets
        {
            next.stack
                .retain(|a| !matches!(a.site, Site::TargetSlot(_)));
            next.stack.extend(slots);
        }
        if rule.clears_allotment {
            // Loop bodies clear `Allot` on nesting (the survival rules).
            next.stack.retain(|a| a.site != Site::Allot);
        }
        if let Some(mut ante) = bind {
            if let Some(card) = rule.binds_that {
                // The binder's own site stands: a choice binds a Chosen
                // FRAME; a search/produce binder pushes a whiffable
                // Product ([CR#701.23]).
                ante.card = card;
                next.stack.push(ante);
            } else if rule.binds_it {
                ante.site = Site::Loop;
                ante.card = Cardinality::One;
                next.stack.push(ante);
            }
        }
        if rule.binds_allotment {
            // The `DivideAmong` share ([CR#601.2d]) — an Amount antecedent
            // at the `Allot` site, read only as `Count::Allotment`.
            next.stack.push(Ante {
                sort: Sort::Amount,
                kind: Kind::Any,
                card: Cardinality::One,
                site: Site::Allot,
                expected_zone: None,
                label: None,
                binder: false,
            });
        }
        match rule.caps_from {
            CapsFrom::Keep => {}
            CapsFrom::Query | CapsFrom::Cost => {
                next.caps = caps;
                next.in_event = true;
                // A fresh event context: the crossing channel is per-event
                // ([CR#714.2b]); `triggered_ability` re-derives it from the
                // new pattern. Event-role antecedents SHADOW an outer
                // event's ("one antecedent per caps guarantee" — per body).
                next.crossing = false;
                next.stack.retain(|a| !matches!(a.site, Site::EventRole(_)));
                next.stack.extend(roles);
            }
        }
        if let Some(may_target) = rule.may_target {
            next.may_target = may_target;
        }
        next
    }

    // ------------------------------------------------------------------
    // the anaphor surface: R1 nearest-compatible + the R2 uniqueness gate
    // ------------------------------------------------------------------

    /// R1 sort compatibility: `Some(widened)` when an anaphor wanting `want`
    /// (`None` = the `It`/`They` wildcard) reaches an antecedent of sort
    /// `have`; `widened = true` marks a non-exact reach (the wildcard, or a
    /// widening table row).
    fn sort_reaches(&self, want: Option<Sort>, have: Sort) -> Option<bool> {
        match want {
            // The wildcard reaches every object/player/pile noun — but
            // never an Amount antecedent (value anaphora is `ThatMany`).
            None => (have != Sort::Amount).then_some(true),
            Some(w) => {
                if let (Sort::OfType(a), Sort::OfType(b)) = (w, have)
                    && a != b
                {
                    return None;
                }
                self.tables.sort_compat(w.key(), have.key())
            }
        }
    }

    /// The R1+R2 stack resolution shared by every anaphor spelling:
    /// nearest compatible antecedent ([R1]), refusing when a SECOND
    /// same-kind compatible antecedent is in scope ([R2] — calibrated and
    /// FROZEN STRICT by [[cards-corpus-dry-run]]; the emitted
    /// `exact_sort_precedence` flag is the one pre-approved loosening,
    /// off). `Allot`-sited antecedents never participate (they are
    /// `Count::Allotment`'s).
    ///
    /// On failure pushes `unbound` (nothing compatible) or
    /// `Code::BindAmbiguous` and returns `None`.
    fn resolve_anaphor(
        &mut self,
        ctx: &Ctx,
        spelling: &str,
        want: Option<Sort>,
        card: Cardinality,
        expected: Kind,
        unbound: Code,
    ) -> Option<Ante> {
        let candidates: Vec<(usize, &Ante, bool)> = ctx
            .stack
            .iter()
            .rev()
            .enumerate()
            .filter(|(_, a)| a.site != Site::Allot)
            .filter(|(_, a)| a.card == card)
            .filter(|(_, a)| a.kind.compatible_with(expected))
            .filter_map(|(depth, a)| {
                self.sort_reaches(want, a.sort)
                    .map(|widened| (depth, a, widened))
            })
            .collect();
        let Some((depth, nearest, nearest_widened)) = candidates.first().copied() else {
            self.err(
                unbound,
                format!("{spelling} read with no compatible antecedent in scope"),
            );
            return None;
        };
        let farther = &candidates[1..];
        if !farther.is_empty() {
            // R2: a second same-kind, compatible antecedent exists. The one
            // pre-approved loosening: a nearer EXACT-sort match beats
            // farther non-exact candidates ([[cards-corpus-dry-run]] froze
            // the emitted flag STRICT=off; the path stays for Idris v2's
            // mirror and any reviewed re-calibration).
            let loosened = self.tables.exact_sort_precedence()
                && !nearest_widened
                && farther.iter().all(|(_, _, widened)| *widened);
            if !loosened {
                self.err(
                    Code::BindAmbiguous,
                    format!(
                        "ambiguous reference — add `Label`/`The`, or name the announce slot \
                         with `As`: {spelling} reaches {} antecedents (nearest: {})",
                        candidates.len(),
                        nearest.describe(),
                    ),
                );
                return None;
            }
        }
        let resolved = nearest.clone();
        self.resolve(|| format!("{spelling} -> #{depth} {}", resolved.describe()));
        Some(resolved)
    }

    /// The labeled-antecedent read (`The`/`TheGroup`, the R2 fallback):
    /// nearest antecedent carrying `label`, of the wanted cardinality.
    fn resolve_label(
        &mut self,
        ctx: &Ctx,
        spelling: &str,
        label: Ident,
        card: Cardinality,
    ) -> Option<Ante> {
        let found = ctx
            .stack
            .iter()
            .rev()
            .enumerate()
            .find(|(_, a)| a.label == Some(label));
        match found {
            Some((depth, ante)) if ante.card == card => {
                let resolved = ante.clone();
                self.resolve(|| format!("{spelling} -> #{depth} {}", resolved.describe()));
                Some(resolved)
            }
            Some((_, ante)) => {
                self.err(
                    Code::BindLabel,
                    format!(
                        "{spelling} names label {:?}, but that antecedent is {:?} \
                         where {card:?} is read",
                        label.as_str(),
                        ante.card,
                    ),
                );
                None
            }
            None => {
                self.err(
                    Code::BindLabel,
                    format!("{spelling} names no label in scope: {:?}", label.as_str()),
                );
                None
            }
        }
    }

    /// The shared plural-anaphor read (`They`/`Them`): the innermost
    /// choice frame when one encloses (deterministic), else the stack.
    fn plural_anaphor(
        &mut self,
        ctx: &Ctx,
        spelling: &str,
        want: Option<Sort>,
        expected: Kind,
    ) -> Option<Kind> {
        if let Some((depth, ante)) = ctx.innermost_frame() {
            if ante.card != Cardinality::Many {
                self.err(
                    Code::BindThatGroup,
                    format!(
                        "{spelling} read where the enclosing binder binds a single object \
                         (read it as That(Sort))"
                    ),
                );
                return None;
            }
            if self.sort_reaches(want, ante.sort).is_none() {
                self.err(
                    Code::BindThatGroup,
                    format!(
                        "{spelling} is incompatible with the enclosing binder's {:?} choice",
                        ante.sort
                    ),
                );
                return None;
            }
            let kind = ante.kind;
            let description = ante.describe();
            self.resolve(|| format!("{spelling} -> #{depth} {description} (binder frame)"));
            return Some(kind);
        }
        self.resolve_anaphor(
            ctx,
            spelling,
            want,
            Cardinality::Many,
            expected,
            Code::BindThatGroup,
        )
        .map(|ante| ante.kind)
    }

    // ------------------------------------------------------------------
    // sort derivation: filters, zones, destinations, binders
    // ------------------------------------------------------------------

    /// The [`Sort`] a zone gives its objects (the emitted `zone-sorts.ron`,
    /// [CR#110.1,112.1,108.2]), refined on the battlefield by a pinned card
    /// type / token kind.
    fn zone_sort(
        &self,
        zone: deckmaste_core::Zone,
        type_pin: Option<Type>,
        token_pin: bool,
    ) -> Sort {
        match self.tables.zone_sort(zone) {
            "Spell" => Sort::Spell,
            "Permanent" => {
                if token_pin {
                    Sort::Token
                } else if let Some(t) = type_pin {
                    Sort::OfType(t)
                } else {
                    Sort::Permanent
                }
            }
            _ => Sort::Card,
        }
    }

    /// The [`Sort`] a filter's objects answer to: a pinned zone (else the
    /// battlefield default) through [`Self::zone_sort`], refined by
    /// conjunctive type/kind pins.
    fn filter_sort(&self, filter: &Filter) -> Sort {
        let pins = filter_pins(filter);
        if pins.player {
            return Sort::Player;
        }
        match pins.object_kind {
            Some(deckmaste_core::ObjectKind::Spell) => return Sort::Spell,
            Some(deckmaste_core::ObjectKind::Ability) => return Sort::StackObject,
            _ => {}
        }
        let zone = pins.zone.unwrap_or(deckmaste_core::Zone::Battlefield);
        self.zone_sort(
            zone,
            pins.card_type,
            pins.object_kind == Some(deckmaste_core::ObjectKind::Token),
        )
    }

    /// The antecedent a binder introduces for its body (site filled in by
    /// the bind-rule row): sort/kind/cardinality from the binder's content,
    /// per the emitted intro rows.
    fn binder_ante(&self, binder: &Binder, cardinality: Cardinality, kind: Kind) -> Ante {
        let (sort, site, frame) = match binder {
            // Choice binders are legacy deterministic FRAMES (the emitted
            // `With.ChooseOne`/`With.Choose`/`With.TheRef`/`With.Existing`
            // intro rows, site Chosen).
            Binder::ChooseOne { filter, .. } | Binder::Choose { filter, .. } => {
                (self.filter_sort(filter), Site::Chosen, true)
            }
            Binder::TheRef(_) => (kind_sort(kind), Site::Chosen, true),
            Binder::Existing(selection) => (self.selection_sort(selection), Site::Chosen, true),
            // Search/produce binders push whiffable PRODUCTS (the
            // `With.SearchOne`/`With.Search`/`With.Produce` rows,
            // [CR#701.23,400.7j]).
            Binder::SearchOne { .. } | Binder::Search { .. } => (Sort::Card, Site::Product, false),
            Binder::Produce(action) => (
                match &**action {
                    Action::Move(_, destination, _) => {
                        self.zone_sort(destination_zone(destination), None, false)
                    }
                    _ => Sort::Permanent,
                },
                Site::Product,
                false,
            ),
            Binder::Expanded(e) => return self.binder_ante(&e.value, cardinality, kind),
        };
        Ante {
            sort,
            kind,
            card: cardinality,
            site,
            expected_zone: None,
            label: None,
            binder: frame,
        }
    }

    /// A best-effort sort for a selection (an `Existing` binder's group).
    fn selection_sort(&self, selection: &Selection) -> Sort {
        match selection {
            Selection::Filter(f) | Selection::Random(_, f) => self.filter_sort(f),
            // Library windows and noted sets are non-battlefield cards.
            Selection::TopOfLibrary { .. }
            | Selection::BottomOfLibrary { .. }
            | Selection::AmongNoted(..) => Sort::Card,
            Selection::Pick { of, .. } => self.filter_sort(of),
            Selection::Union(members) => {
                let sorts: Vec<Sort> = members.iter().map(|m| self.selection_sort(m)).collect();
                match sorts.split_first() {
                    Some((first, rest)) if rest.iter().all(|s| s == first) => *first,
                    _ => Sort::Permanent,
                }
            }
            // The plural anaphors re-read an existing antecedent; their
            // group keeps that antecedent's noun where it is unambiguous —
            // conservative `Permanent` otherwise (the walk of the selection
            // itself already validated the read).
            Selection::They
            | Selection::Them(_)
            | Selection::TheGroup(_)
            | Selection::PilesOf { .. } => Sort::Permanent,
            Selection::Expanded(e) => self.selection_sort(&e.value),
        }
    }

    /// Builds a producing clause's [`Intro`] from its emitted intro row:
    /// the object push (site/cardinality/sort/expected-zone per the row's
    /// rules) plus the Amount companion ("that much", [CR#608.2i]).
    fn intro_from_row(
        &self,
        clause: &str,
        one: bool,
        filter: Option<&Filter>,
        destination: Option<&Destination>,
    ) -> Intro {
        let row = self.tables.intro(clause);
        let mut intro = Intro::default();
        if row.object {
            let card = match row.card {
                tables::IntroCard::One => Cardinality::One,
                tables::IntroCard::Many => Cardinality::Many,
                tables::IntroCard::FromQuantity | tables::IntroCard::FromCount => {
                    if one {
                        Cardinality::One
                    } else {
                        Cardinality::Many
                    }
                }
            };
            let site = match row.site {
                tables::IntroSite::Product => Site::Product,
                tables::IntroSite::Chosen => Site::Chosen,
                tables::IntroSite::TargetSlot | tables::IntroSite::Loop => {
                    unreachable!("slot/loop intro rows are applied by their own constructs")
                }
            };
            let expected_zone = match row.zone.as_str() {
                "none" => None,
                "from_destination" => destination.map(destination_zone),
                "Battlefield" => Some(deckmaste_core::Zone::Battlefield),
                "Hand" => Some(deckmaste_core::Zone::Hand),
                "Graveyard" => Some(deckmaste_core::Zone::Graveyard),
                other => panic!("unknown intro zone rule {other:?}"),
            };
            let sort = match row.sort.as_str() {
                "Card" => Sort::Card,
                "Token" => Sort::Token,
                "Pile" => Sort::Pile,
                "from_filter" => filter.map_or(Sort::Permanent, |f| self.filter_sort(f)),
                "from_destination" => match destination {
                    Some(d) => self.zone_sort(destination_zone(d), None, false),
                    None => Sort::Card,
                },
                other => panic!("unknown intro sort rule {other:?}"),
            };
            let kind = if sort == Sort::Player { Kind::Player } else { Kind::Object };
            intro.pushes.push(Ante {
                sort,
                kind,
                card,
                site,
                expected_zone,
                label: None,
                binder: false,
            });
        }
        if row.amount {
            intro.pushes.push(Ante {
                sort: Sort::Amount,
                kind: Kind::Any,
                card: Cardinality::One,
                site: Site::Product,
                expected_zone: None,
                label: None,
                binder: false,
            });
        }
        intro
    }

    /// The role antecedents an event/payment body pushes — ONE per caps
    /// guarantee ([CR#603.2e,608.2k]; the caps table and the anaphora
    /// mechanism are the same machine).
    fn role_antes(caps: Caps, object_sort: Sort, patient_sort: Option<Sort>) -> Vec<Ante> {
        let ante = |sort, kind, role| Ante {
            sort,
            kind,
            card: Cardinality::One,
            site: Site::EventRole(role),
            expected_zone: None,
            label: None,
            binder: false,
        };
        let mut roles = Vec::new();
        if caps.object {
            roles.push(ante(object_sort, Kind::Object, Role::Object));
        }
        if let Some(kind) = caps.patient {
            let sort = patient_sort.unwrap_or(match kind {
                Kind::Player => Sort::Player,
                _ => Sort::Permanent,
            });
            roles.push(ante(sort, kind, Role::Patient));
        }
        if caps.actor {
            roles.push(ante(Sort::Player, Kind::Player, Role::Actor));
        }
        if caps.amount {
            roles.push(ante(Sort::Amount, Kind::Any, Role::Amount));
        }
        if caps.defender {
            roles.push(ante(Sort::Player, Kind::Player, Role::Defender));
        }
        roles
    }

    /// The role antecedents an EVENT body pushes, with the object/patient
    /// sorts derived from the pattern (the `object_sort` column +
    /// per-form patient slots).
    fn event_role_antes(&self, event: &EventFilter) -> Vec<Ante> {
        let caps = self.event_caps(event);
        Self::role_antes(
            caps,
            self.event_object_sort_of(event),
            self.event_patient_sort_of(event),
        )
    }

    /// The role antecedents a COST payment pushes ([CR#601.2f]) — payment
    /// objects leave for another zone, so the object noun is `Card`.
    fn cost_role_antes(caps: Caps) -> Vec<Ante> {
        Self::role_antes(caps, Sort::Card, None)
    }

    /// The SORT of an event pattern's object antecedent — the emitted
    /// `object_sort` column names the deriving slot; the walker extracts it
    /// ([CR#400.7e] — the noun follows the zone the object moved to).
    fn event_object_sort_of(&self, event: &EventFilter) -> Sort {
        match event {
            EventFilter::Expanded(e) => self.event_object_sort_of(&e.value),
            EventFilter::OneOrMore(inner)
            | EventFilter::Nth { of: inner, .. }
            | EventFilter::When(inner, _)
            | EventFilter::Within(inner, _) => self.event_object_sort_of(inner),
            EventFilter::AllOf(members) | EventFilter::OneOf(members) => {
                let sorts: Vec<Sort> = members
                    .iter()
                    .map(|m| self.event_object_sort_of(m))
                    .collect();
                match sorts.split_first() {
                    Some((first, rest)) if rest.iter().all(|s| s == first) => *first,
                    _ => Sort::Permanent,
                }
            }
            master => {
                let Some(key) = form_key(master) else {
                    return Sort::Permanent;
                };
                match self.tables.event_object_sort(key) {
                    "to_zone" => {
                        let EventFilter::ZoneChange {
                            what,
                            to,
                            cause,
                            from: _,
                        } = master
                        else {
                            return Sort::Permanent;
                        };
                        // An unfixed `to` falls back to the cause verb's
                        // entailed destination ([CR#701] entailment rows).
                        let to = to.or_else(|| {
                            if let Some(deckmaste_core::Cause::Cause(pattern)) = cause
                                && let Some(verb) = pattern.verb
                                && let Some(row) = self.tables.entailment(verb.as_str())
                            {
                                row.to
                            } else {
                                None
                            }
                        });
                        match to {
                            Some(zone) => {
                                let pins = filter_pins(what);
                                self.zone_sort(
                                    zone,
                                    pins.card_type,
                                    pins.object_kind == Some(deckmaste_core::ObjectKind::Token),
                                )
                            }
                            // The object left its zone; where it went is
                            // unfixed — the conservative noun is "card".
                            None => Sort::Card,
                        }
                    }
                    "spell" => Sort::Spell,
                    "stack_object" => Sort::StackObject,
                    "token" => Sort::Token,
                    slot @ ("source" | "what" | "by" | "of" | "on") => {
                        match event_slot_filter(master, slot) {
                            Some(filter) => self.filter_sort(filter),
                            None => Sort::Permanent,
                        }
                    }
                    _ => Sort::Permanent,
                }
            }
        }
    }

    /// The SORT of an event pattern's patient antecedent, where a form
    /// fixes one (`Damage.to`, [CR#120.3]).
    fn event_patient_sort_of(&self, event: &EventFilter) -> Option<Sort> {
        match event {
            EventFilter::Expanded(e) => self.event_patient_sort_of(&e.value),
            EventFilter::OneOrMore(inner)
            | EventFilter::Nth { of: inner, .. }
            | EventFilter::When(inner, _)
            | EventFilter::Within(inner, _) => self.event_patient_sort_of(inner),
            EventFilter::Damage { to, .. } => Some(self.filter_sort(to)),
            _ => None,
        }
    }

    // ------------------------------------------------------------------
    // faces and tokens: the well-formedness floors
    // ------------------------------------------------------------------

    fn face(&mut self, face: &CardFace) {
        if face.types.is_empty() {
            self.err(Code::FloorTypes, "a face must have at least one card type");
        }
        self.subtype_floor(&face.subtypes, &face.types);
        if face.loyalty.is_some() && !face.types.contains(&Type::Planeswalker) {
            self.err(
                Code::FloorLoyalty,
                "printed loyalty on a face without the Planeswalker type",
            );
        }
        if face.defense.is_some() && !face.types.contains(&Type::Battle) {
            self.err(
                Code::FloorDefense,
                "printed defense on a face without the Battle type",
            );
        }
        let has_x = face
            .mana_cost
            .iter()
            .any(|s| matches!(s, ManaSymbol::Variable));
        self.abilities(&face.abilities, has_x);
        self.subtype_confers(&face.subtypes);
    }

    fn token(&mut self, token: &Token) {
        if token.types.is_empty() {
            self.err(Code::FloorTypes, "a token must have at least one card type");
        }
        if token
            .types
            .iter()
            .any(|t| matches!(t, Type::Instant | Type::Sorcery))
        {
            self.err(
                Code::FloorTokenTypes,
                "a token's types must be permanent types",
            );
        }
        self.subtype_floor(&token.subtypes, &token.types);
        self.abilities(&token.abilities, false);
        self.subtype_confers(&token.subtypes);
    }

    /// [CR#205.3d]: a face may carry a subtype only if it has one of the
    /// subtype's governing card types (the declared list is a disjunction —
    /// creature types govern Creature AND Kindred). The governing types come
    /// from the loaded registry when the name is declared (the declaration is
    /// the authority; the existing `bare-subtype-in-card` lint pins value ==
    /// declaration), else from the inline value.
    fn subtype_floor(&mut self, subtypes: &[deckmaste_core::Subtype], types: &[Type]) {
        for subtype in subtypes {
            // [CR#205.3]: the CATEGORY column of the loaded registry is the
            // authority — a hand-inlined value whose governing types
            // disagree with the declaration of the same name is a forgery,
            // not a new subtype.
            if let Some(declared) = self.registries.subtypes.get(&subtype.name) {
                if declared.types == subtype.types {
                    self.resolve(|| {
                        format!(
                            "subtype {:?} -> category {:?}",
                            subtype.name.as_str(),
                            declared.types
                        )
                    });
                } else {
                    self.err(
                        Code::KindSubtypeCategory,
                        format!(
                            "subtype {:?} is declared with category {:?}, but this value \
                             carries {:?} — the declaration is the authority",
                            subtype.name.as_str(),
                            declared.types,
                            subtype.types
                        ),
                    );
                }
            }
            let governing_types = self
                .registries
                .subtypes
                .get(&subtype.name)
                .map_or(&subtype.types, |declared| &declared.types);
            if !governing_types.is_empty()
                && !governing_types
                    .iter()
                    .any(|governing| types.contains(governing))
            {
                self.err(
                    Code::FloorSubtype,
                    format!(
                        "subtype {:?} governs {governing_types:?}, but the face has none of \
                         those types",
                        subtype.name.as_str()
                    ),
                );
            }
        }
    }

    /// What the face's subtypes confer rides along as abilities/effects of
    /// the bearer — walked like printed text (`This` = the bearer).
    fn subtype_confers(&mut self, subtypes: &[deckmaste_core::Subtype]) {
        for subtype in subtypes {
            self.scoped(format!("subtype[{}]", subtype.name.as_str()), |w| {
                for (i, property) in subtype.confers.iter().enumerate() {
                    w.scoped(format!("confers[{i}]"), |w| w.property(property));
                }
            });
        }
    }

    fn property(&mut self, property: &Property) {
        let ctx = Ctx::base(false);
        match property {
            Property::Ability(ability) => self.ability(ability, false),
            Property::Continuous { of, changes } => {
                self.modify_scope(of, changes, &ctx);
            }
            Property::StateBased { condition, effect } => {
                self.condition(condition, &ctx);
                let inner = self.descend(&ctx, "Sba", None, None, NO_CAPS, vec![]);
                self.effect(effect, &inner);
            }
            Property::TurnBased { at: _, effect } => {
                // A turn-based action never targets ([CR#703]); the Sba row
                // carries the same never-a-targeting-position discipline.
                let inner = self.descend(&ctx, "Sba", None, None, NO_CAPS, vec![]);
                self.effect(effect, &inner);
            }
        }
    }

    // ------------------------------------------------------------------
    // abilities
    // ------------------------------------------------------------------

    fn abilities(&mut self, abilities: &[Ability], has_x: bool) {
        for (i, ability) in abilities.iter().enumerate() {
            self.scoped(format!("abilities[{i}]"), |w| w.ability(ability, has_x));
        }
    }

    fn ability(&mut self, ability: &Ability, face_x: bool) {
        match ability {
            Ability::Spell(spell) => {
                let mut ctx = Ctx::base(face_x);
                ctx.may_target = true;
                self.scoped("effect", |w| {
                    w.effect(&spell.effect, &ctx);
                });
            }
            Ability::Activated(activated) => {
                let has_x = cost_declares_x(&activated.cost);
                let ctx = Ctx::base(has_x);
                self.scoped("cost", |w| w.cost(&activated.cost, &ctx));
                if let Some(condition) = &activated.condition {
                    self.scoped("condition", |w| w.condition(condition, &ctx));
                }
                let mut body = ctx;
                body.may_target = true;
                self.scoped("effect", |w| {
                    w.effect(&activated.effect, &body);
                });
            }
            Ability::Triggered(triggered) => {
                self.triggered_ability(triggered, &Ctx::base(false), "Triggered");
            }
            Ability::Static(static_ability) => self.static_ability(static_ability),
            Ability::Keyword(keyword) => self.keyword(keyword, face_x),
            Ability::Innate(inner) => self.ability(inner, face_x),
            Ability::Expanded(e) => self.ability(&e.value, face_x),
        }
    }

    fn keyword(&mut self, keyword: &KeywordAbility, face_x: bool) {
        self.keyword_use(keyword, face_x, false);
    }

    /// A keyword USE against the loaded keyword registry ([CR#702] —
    /// registry row + typed args): a `Composite` reached WITHOUT its macro
    /// invocation (`via_invocation`) has lost its args, so a declared
    /// non-`None` `ParamShape` refuses it — a bare parameterized keyword is
    /// a load error. Invoked uses were arity/type-checked by the macro
    /// layer; intrinsics and undeclared names (an open vocabulary) pass.
    fn keyword_use(&mut self, keyword: &KeywordAbility, face_x: bool, via_invocation: bool) {
        match keyword {
            KeywordAbility::FirstStrike
            | KeywordAbility::DoubleStrike
            | KeywordAbility::Deathtouch
            | KeywordAbility::Trample
            | KeywordAbility::Vigilance => {}
            KeywordAbility::Composite { name, abilities } => {
                match self.registries.keywords.get(name) {
                    Some(decl) if !via_invocation && decl.shape != ParamShape::None => {
                        self.err(
                            Code::KindKeywordShape,
                            format!(
                                "keyword {:?} is declared {:?} — a bare Composite spelling loses its args; invoke the keyword macro",
                                name.as_str(),
                                decl.shape
                            ),
                        );
                    }
                    Some(decl) => {
                        self.resolve(|| {
                            format!("keyword {:?} -> shape {:?}", name.as_str(), decl.shape)
                        });
                    }
                    None => {}
                }
                self.abilities(abilities, face_x);
            }
            KeywordAbility::Expanded(e) => self.keyword_use(&e.value, face_x, true),
        }
    }

    fn static_ability(&mut self, ability: &StaticAbility) {
        let ctx = Ctx::base(false);
        if let Some(condition) = &ability.condition {
            self.scoped("condition", |w| w.condition(condition, &ctx));
        }
        for (i, effect) in ability.effects.iter().enumerate() {
            self.scoped(format!("effects[{i}]"), |w| w.static_effect(effect, &ctx));
        }
    }

    /// A triggered ability, wherever it appears: the card-level kind, a
    /// `Delayed` schedule, or a `Reflexive` — the `construct` names the
    /// bind-rule row that shapes the body context.
    fn triggered_ability(&mut self, triggered: &TriggeredAbility, ctx: &Ctx, construct: &str) {
        // The event pattern reads the context BEFORE the event binds — but
        // with the row's drops already applied (a delayed trigger's pattern
        // can't see the spell's targets either, [CR#603.7c]).
        let pattern_ctx = self.descend(ctx, construct, None, None, NO_CAPS, vec![]);
        let lane = Lane::for_trigger_construct(construct);
        self.scoped("event", |w| w.event(&triggered.event, &pattern_ctx, lane));
        let caps = self.event_caps(&triggered.event);
        let roles = self.event_role_antes(&triggered.event);
        let mut body = self.descend(ctx, construct, None, None, caps, roles);
        body.crossing = fixes_crossing(&triggered.event);
        if let Some(condition) = &triggered.condition {
            // Intervening-if ([CR#603.4]) is elaborated in the event-extended
            // context ("if that creature's power…").
            self.scoped("condition", |w| w.condition(condition, &body));
        }
        self.scoped("effect", |w| {
            w.effect(&triggered.effect, &body);
        });
    }

    // ------------------------------------------------------------------
    // effects
    // ------------------------------------------------------------------

    fn effect(&mut self, effect: &Effect, ctx: &Ctx) -> Intro {
        let pending_floor = self.pending.len();
        let intro = self.effect_inner(effect, ctx, pending_floor);
        // Drop reference-pushes no clause consumed (an `A()` in a
        // condition/cost position introduces nothing for siblings).
        self.pending.truncate(pending_floor);
        intro
    }

    #[expect(
        clippy::too_many_lines,
        reason = "one arm per Effect variant; the flat dispatch reads better whole"
    )]
    fn effect_inner(&mut self, effect: &Effect, ctx: &Ctx, pending_floor: usize) -> Intro {
        match effect {
            Effect::Act(action) => {
                let mut intro = self.action(action, ctx);
                // An `A(filter)` read inside the clause pushes a Chosen
                // antecedent for the clauses to its right ([CR#608.2d]) —
                // ahead of the verb's own products (the choice precedes the
                // act).
                let chosen: Vec<Ante> = self.pending.split_off(pending_floor);
                intro.pushes.splice(0..0, chosen);
                intro
            }
            // [CR#603.3b,616.1]: the simultaneous batch — every member reads
            // ONE pre-application snapshot, so no member's Intro reaches a
            // sibling (unlike `Sequence`'s telescope). Load-capped to the
            // exchange-family macros' bodies until the engine wiring
            // generalizes.
            Effect::Simultaneous(effects) => {
                if !ctx.in_exchange_macro {
                    self.err(
                        Code::PosSimultaneous,
                        "Simultaneous outside an exchange-family macro body — the engine's \
                         batch wiring is exchange-only ([CR#701.12]) until it generalizes",
                    );
                }
                let mut intro = Intro::default();
                for (i, element) in effects.iter().enumerate() {
                    self.scoped(format!("Simultaneous[{i}]"), |w| {
                        intro.absorb(w.effect(element, ctx));
                    });
                }
                intro
            }
            Effect::Sequence(effects) => {
                // The TELESCOPE: clause i+1 elaborates in the context
                // extended by clause i's introductions — sentence order IS
                // binder order (the plan's §2.2 `Seq`).
                let mut running = ctx.clone();
                let mut intro = Intro::default();
                for (i, element) in effects.iter().enumerate() {
                    let mut introduced = Intro::default();
                    self.scoped(format!("Sequence[{i}]"), |w| {
                        introduced = w.effect(element, &running);
                    });
                    introduced.apply_to(&mut running);
                    intro.absorb(introduced);
                }
                intro
            }
            Effect::Continuously(continuously) => {
                self.scoped("Continuously", |w| {
                    // The single-part spelling of `Until` — the same
                    // per-part class stamp ([CR#611.2c]).
                    let class = w
                        .tables
                        .static_class(static_effect_key(&continuously.effect));
                    w.resolve(|| format!("Continuously part -> {class:?} ([CR#611.2c])"));
                    w.static_effect(&continuously.effect, ctx);
                    w.duration(&continuously.duration, ctx);
                });
                Intro::default()
            }
            Effect::May(may) => {
                let mut intro = Intro::default();
                self.scoped("May", |w| {
                    intro = w.effect(&may.effect, ctx);
                    // "if you do" elaborates in intro(effect)'s context.
                    let mut did_ctx = ctx.clone();
                    intro.apply_to(&mut did_ctx);
                    if let Some(if_did) = &may.if_did {
                        w.scoped("if_did", |w| {
                            w.effect(if_did, &did_ctx);
                        });
                    }
                    if let Some(if_not) = &may.if_not {
                        w.scoped("if_not", |w| {
                            w.effect(if_not, ctx);
                        });
                    }
                });
                // A May's introductions FLOW to its right siblings (the
                // Through-the-Breach shape: "You may put a creature card …
                // That creature gains haste"); a DECLINED May's products are
                // runtime-skipped — defined, not grace ([CR#701.23b]).
                intro
            }
            Effect::If(branch) => {
                self.scoped("If", |w| {
                    w.condition(&branch.condition, ctx);
                    w.effect(&branch.then, ctx);
                    if let Some(otherwise) = &branch.otherwise {
                        w.effect(otherwise, ctx);
                    }
                });
                Intro::default()
            }
            Effect::Unless(unless) => {
                self.scoped("Unless", |w| {
                    w.reference(&unless.who, ctx, Kind::Player);
                    for (i, component) in unless.unless.iter().enumerate() {
                        w.scoped(format!("unless[{i}]"), |w| w.cost_component(component, ctx));
                    }
                    w.effect(&unless.effect, ctx);
                });
                Intro::default()
            }
            Effect::MayPay(may_pay) => {
                self.scoped("MayPay", |w| {
                    w.reference(&may_pay.actor, ctx, Kind::Player);
                    w.cost(&may_pay.cost, ctx);
                    // "if they do" reads the payment ([CR#608.2d]): the
                    // cost's caps push role antecedents, like an
                    // AdditionalCost body. The "or else" branch runs when
                    // nothing was paid and reads the plain context.
                    let caps = w.cost_caps(&may_pay.cost);
                    let roles = Walker::cost_role_antes(caps);
                    let body = w.descend(ctx, "MayPay", None, None, caps, roles);
                    w.effect(&may_pay.and_then, &body);
                    if let Some(or_else) = &may_pay.or_else {
                        w.effect(or_else, ctx);
                    }
                });
                Intro::default()
            }
            Effect::MustPay(must_pay) => {
                self.scoped("MustPay", |w| {
                    w.reference(&must_pay.actor, ctx, Kind::Player);
                    w.cost(&must_pay.cost, ctx);
                    w.effect(&must_pay.or_else, ctx);
                });
                Intro::default()
            }
            Effect::AdditionalCost(additional) => {
                self.scoped("AdditionalCost", |w| {
                    w.cost(&additional.pay, ctx);
                    // The payment is an event: the body reads the paid object
                    // through the event roles ([CR#601.2f,118.8]).
                    let caps = w.cost_caps(&additional.pay);
                    let roles = Walker::cost_role_antes(caps);
                    let body = w.descend(ctx, "AdditionalCost", None, None, caps, roles);
                    w.effect(&additional.body, &body);
                });
                Intro::default()
            }
            Effect::Each(each) => {
                let mut intro = Intro::default();
                self.scoped("Each", |w| {
                    let (_, kind) = w.binder(&each.binder, ctx);
                    let ante = w.binder_ante(&each.binder, Cardinality::One, kind);
                    let body = w.descend(ctx, "Each", None, Some(ante), NO_CAPS, vec![]);
                    let inner = w.effect(&each.effect, &body);
                    // Loop antecedents pop with the body; NOTES are
                    // game-state memory ([CR#607.2]) and escape — the
                    // Whims-of-the-Fates two-loop shape reads loop-1's
                    // noted piles in loop 2.
                    intro.notes.extend(inner.notes);
                });
                intro
            }
            Effect::With(with) => {
                let mut intro = Intro::default();
                self.scoped("With", |w| {
                    let (cardinality, kind) = w.binder(&with.binder, ctx);
                    let construct = match cardinality {
                        Cardinality::One => "With.One",
                        Cardinality::Many => "With.Many",
                    };
                    let ante = w.binder_ante(&with.binder, cardinality, kind);
                    let body = w.descend(ctx, construct, None, Some(ante), NO_CAPS, vec![]);
                    intro = w.effect(&with.body, &body);
                });
                intro
            }
            Effect::DivideAmong(divide) => {
                self.scoped("DivideAmong", |w| {
                    w.count(&divide.amount, ctx);
                    let (_, kind) = w.binder(&divide.binder, ctx);
                    w.divide_floor(&divide.amount, &divide.binder, ctx);
                    let ante = w.binder_ante(&divide.binder, Cardinality::One, kind);
                    let body = w.descend(ctx, "DivideAmong", None, Some(ante), NO_CAPS, vec![]);
                    w.effect(&divide.body, &body);
                });
                Intro::default()
            }
            Effect::Until(duration, parts) => {
                self.scoped("Until", |w| {
                    w.duration(duration, ctx);
                    for (i, part) in parts.iter().enumerate() {
                        w.scoped(format!("parts[{i}]"), |w| {
                            // [CR#611.2c]: fixed-vs-live affected sets are
                            // PER PART — the emitted static-classes column
                            // stamps each one.
                            let class = w.tables.static_class(static_effect_key(part));
                            w.resolve(|| format!("Until part -> {class:?} ([CR#611.2c])"));
                            w.static_effect(part, ctx);
                        });
                    }
                });
                Intro::default()
            }
            Effect::Label(label) => {
                let mut intro = Intro::default();
                self.scoped("Label", |w| {
                    intro = w.effect(&label.effect, ctx);
                });
                // Re-site the inner introductions at the label — the R2
                // gate's escape hatch (`The`/`TheGroup` read them,
                // [CR#608.2d]).
                for ante in &mut intro.pushes {
                    ante.label = Some(label.r#as);
                }
                intro
            }
            Effect::SeparatePiles(piles) => {
                let mut intro = Intro::default();
                self.scoped("SeparatePiles", |w| {
                    w.selection(&piles.group, ctx, Kind::Object);
                    w.reference(&piles.by, ctx, Kind::Player);
                    // Pile floors ([CR#700.3]): at least one label, no
                    // duplicates. (Piles themselves may be EMPTY,
                    // [CR#700.3d] — the floor is about the label list.)
                    if piles.into.is_empty() {
                        w.err(Code::FloorPiles, "SeparatePiles with no pile labels");
                    }
                    let mut seen = BTreeSet::new();
                    for label in &piles.into {
                        if !seen.insert(label.as_str()) {
                            w.err(
                                Code::FloorPiles,
                                format!("duplicate pile label {:?}", label.as_str()),
                            );
                        }
                    }
                    // One labeled Many antecedent per pile (the emitted
                    // `SeparatePiles.pile` intro row); `note:` persists
                    // them as noted pile groups ([CR#700.3], read back via
                    // `PilesOf`).
                    let row = w.tables.intro("SeparatePiles.pile");
                    debug_assert!(row.object && row.site == tables::IntroSite::Product);
                    for label in &piles.into {
                        intro.pushes.push(Ante {
                            sort: Sort::Pile,
                            kind: Kind::Object,
                            card: Cardinality::Many,
                            site: Site::Product,
                            expected_zone: None,
                            label: Some(*label),
                            binder: false,
                        });
                    }
                    if let Some(note) = piles.note {
                        intro.notes.push((note, NotedKind::Piles));
                    }
                    if let Some(then) = &piles.then {
                        let mut then_ctx = ctx.clone();
                        intro.apply_to(&mut then_ctx);
                        w.scoped("then", |w| {
                            w.effect(then, &then_ctx);
                        });
                    }
                });
                intro
            }
            Effect::ChoosePile(choose) => {
                self.scoped("ChoosePile", |w| {
                    match &choose.from {
                        deckmaste_core::PileSource::Labels(labels) => {
                            if labels.is_empty() {
                                w.err(Code::FloorPiles, "ChoosePile with no pile labels");
                            }
                            for label in labels {
                                let _ = w.resolve_label(
                                    ctx,
                                    "ChoosePile label",
                                    *label,
                                    Cardinality::Many,
                                );
                            }
                        }
                        deckmaste_core::PileSource::Noted { note, of } => {
                            w.reference(of, ctx, Kind::Player);
                            match noted_kind(ctx, note) {
                                Some(NotedKind::Piles) => {
                                    w.resolve(|| {
                                        format!(
                                            "ChoosePile -> piles noted under {:?}",
                                            note.as_str()
                                        )
                                    });
                                }
                                Some(kind) => {
                                    w.err(
                                        Code::KindNoteDomain,
                                        format!(
                                            "ChoosePile reads piles, but key {:?} stores \
                                             {kind:?} ([CR#607.2])",
                                            note.as_str()
                                        ),
                                    );
                                }
                                None => {
                                    w.err(
                                        Code::BindNote,
                                        format!(
                                            "ChoosePile reads key {:?} but nothing noted it",
                                            note.as_str()
                                        ),
                                    );
                                }
                            }
                        }
                    }
                    w.reference(&choose.by, ctx, Kind::Player);
                    // The chosen pile binds for `then` as a Many Pile
                    // antecedent (the emitted `ChoosePile` intro row),
                    // read as `Them(Pile)`.
                    let row = w.tables.intro("ChoosePile");
                    debug_assert!(row.object && row.site == tables::IntroSite::Chosen);
                    let mut body = ctx.clone();
                    body.stack.push(Ante {
                        sort: Sort::Pile,
                        kind: Kind::Object,
                        card: Cardinality::Many,
                        site: Site::Chosen,
                        expected_zone: None,
                        label: None,
                        binder: true,
                    });
                    w.scoped("then", |w| {
                        w.effect(&choose.then, &body);
                    });
                });
                Intro::default()
            }
            Effect::Noting(noting) => {
                // A `Noting` stores the OBJECT SET the inner effect touched
                // ([CR#607.2a] "exiled with" linkage) — its slot is
                // `NotedKind::Objects`.
                let mut inner_ctx = ctx.clone();
                inner_ctx.notes.push((noting.key, NotedKind::Objects));
                let mut intro = Intro::default();
                self.scoped("Noting", |w| {
                    intro = w.effect(&noting.effect, &inner_ctx);
                });
                intro.notes.push((noting.key, NotedKind::Objects));
                intro
            }
            Effect::Delayed(triggered) => {
                self.scoped("Delayed", |w| {
                    w.triggered_ability(triggered, ctx, "Delayed");
                });
                Intro::default()
            }
            Effect::Reflexive(triggered) => {
                self.scoped("Reflexive", |w| {
                    w.triggered_ability(triggered, ctx, "Reflexive");
                });
                Intro::default()
            }
            Effect::Modal(modal) => {
                self.modal(modal, ctx);
                Intro::default()
            }
            Effect::Targeted(targeted) => self.targeted(targeted, ctx),
            Effect::Expanded(e) => {
                // The invocation's name is the macro-body provenance hook:
                // an exchange-family expansion admits the batch primitives
                // inside its own body ([CR#701.12], `E-POS-SIMULTANEOUS`).
                if EXCHANGE_FAMILY.contains(&e.name.as_str()) {
                    let mut inner = ctx.clone();
                    inner.in_exchange_macro = true;
                    self.effect(&e.value, &inner)
                } else {
                    self.effect(&e.value, ctx)
                }
            }
        }
    }

    fn targeted(&mut self, targeted: &deckmaste_core::Targeted, ctx: &Ctx) -> Intro {
        if !ctx.may_target {
            self.err(
                Code::PosTargeted,
                "Targeted is legal only at spell/mode/triggered/activated/reflexive/delayed \
                 roots — never in replacement/static/loop positions",
            );
        }
        let mut slots = Vec::with_capacity(targeted.targets.len());
        let mut intro = Intro::default();
        self.scoped("Targeted", |w| {
            // Each announced slot pushes an antecedent (the emitted
            // `Targeted.slot` intro row): sort from the slot's filter,
            // cardinality from its quantity ([CR#115.3,601.2c]).
            let row = w.tables.intro("Targeted.slot");
            debug_assert!(row.site == tables::IntroSite::TargetSlot);
            let mut seen_labels: Vec<Ident> = Vec::new();
            for (i, spec) in targeted.targets.iter().enumerate() {
                let mut slot = (Kind::Any, Sort::Permanent, Cardinality::One);
                w.scoped(format!("targets[{i}]"), |w| {
                    slot = w.target_spec(spec, ctx, targeted.targets.len());
                });
                let (kind, sort, card) = slot;
                // An `As`-named slot carries its label — read explicitly as
                // `The`/`TheGroup` ([CR#608.2d]); it still participates in
                // R1/R2 like an unlabeled slot. Duplicate names would make
                // the labeled read a guess, so they are refused.
                let label = slot_label(spec);
                if let Some(label) = label {
                    if seen_labels.contains(&label) {
                        w.err(
                            Code::BindLabel,
                            format!(
                                "duplicate announce label {:?} — every As-named slot needs \
                                 a distinct name",
                                label.as_str()
                            ),
                        );
                    }
                    seen_labels.push(label);
                }
                slots.push(Ante {
                    sort,
                    kind,
                    card,
                    site: Site::TargetSlot(i),
                    expected_zone: None,
                    label,
                    binder: false,
                });
            }
            let body = w.descend(ctx, "Targeted", Some(slots), None, NO_CAPS, vec![]);
            intro = w.effect(&targeted.effect, &body);
        });
        intro
    }

    /// One announced target spec: its kind, its SORT (from the filter), and
    /// its cardinality (a literal-1 quantity announces One, anything wider
    /// a Many group — a range slot is read as `They`, [CR#601.2d]).
    fn target_spec(
        &mut self,
        spec: &TargetSpec,
        ctx: &Ctx,
        sibling_count: usize,
    ) -> (Kind, Sort, Cardinality) {
        match spec {
            TargetSpec::Target(quantity, filter) => {
                self.quantity(quantity, ctx, true);
                let kind = self.filter(filter, ctx, Kind::Any);
                let card =
                    if quantity_is_one(quantity) { Cardinality::One } else { Cardinality::Many };
                (kind, self.filter_sort(filter), card)
            }
            // The name is the enclosing announce walk's business
            // ([`slot_label`]); the slot's shape is the inner spec's.
            TargetSpec::As(_, inner) => self.target_spec(inner, ctx, sibling_count),
            TargetSpec::Distinct(siblings, inner) => {
                for &index in siblings {
                    if index >= sibling_count {
                        self.err(
                            Code::BindTarget,
                            format!(
                                "Distinct names sibling target spec {index}, but only \
                                 {sibling_count} specs are announced"
                            ),
                        );
                    }
                }
                self.target_spec(inner, ctx, sibling_count)
            }
            TargetSpec::Expanded(e) => self.target_spec(&e.value, ctx, sibling_count),
        }
    }

    fn modal(&mut self, modal: &deckmaste_core::Modal, ctx: &Ctx) {
        self.scoped("Modal", |w| {
            if modal.modes.is_empty() {
                w.err(Code::FloorModalEmpty, "a modal effect must offer a mode");
            }
            w.quantity(&modal.choose.count, ctx, false);
            w.reference(&modal.choose.chooser, ctx, Kind::Player);
            if let Some(rider) = &modal.choose.rider {
                // The entwine/escalate rider cost ([CR#702.42a,702.120a]) is
                // announced with the mode choice ([CR#601.2b]).
                let (deckmaste_core::ModalCostRider::Entwine(cost)
                | deckmaste_core::ModalCostRider::Escalate(cost)) = rider;
                w.scoped("rider", |w| w.cost(cost, ctx));
            }
            // [CR#700.2d]: the REQUIRED minimum must be satisfiable by the
            // printed modes (repeats lifts the ceiling; up_to lowers the
            // floor to zero).
            if !modal.choose.repeats
                && !modal.choose.up_to
                && let (Some(lo), _) = modal.choose.count.bounds()
                && let Some(lo) = lo.literal_value()
                && lo as usize > modal.modes.len()
            {
                w.err(
                    Code::FloorModalCount,
                    format!("choose {lo} of {} modes ([CR#700.2d])", modal.modes.len()),
                );
            }
            for (i, mode) in modal.modes.iter().enumerate() {
                w.scoped(format!("modes[{i}]"), |w| {
                    if let Some(cost) = &mode.cost {
                        for (j, component) in cost.iter().enumerate() {
                            w.scoped(format!("cost[{j}]"), |w| w.cost_component(component, ctx));
                        }
                    }
                    w.effect(&mode.effect, ctx);
                });
            }
        });
    }

    /// [CR#601.2d]: a statically-literal divided amount must cover the
    /// statically-known minimum group size (each element receives ≥1).
    fn divide_floor(&mut self, amount: &Count, binder: &Binder, _ctx: &Ctx) {
        let Some(amount) = amount.literal_value() else {
            return;
        };
        let minimum = match binder {
            Binder::Choose { quantity, .. } | Binder::Search { quantity, .. } => {
                quantity.bounds().0.and_then(Count::literal_value)
            }
            _ => None,
        };
        if let Some(minimum) = minimum
            && amount < minimum
        {
            self.err(
                Code::FloorDivide,
                format!("dividing {amount} among at least {minimum} recipients ([CR#601.2d])"),
            );
        }
    }

    // ------------------------------------------------------------------
    // actions
    // ------------------------------------------------------------------

    fn action(&mut self, action: &Action, ctx: &Ctx) -> Intro {
        match action {
            Action::DealDamage(patient, amount, source) => {
                self.reference(patient, ctx, Kind::Any);
                self.count(amount, ctx);
                self.reference(source, ctx, Kind::Object);
                // An amount-bearing verb pushes "that much" ([CR#608.2i])
                // — the emitted DealDamage intro row.
                self.intro_from_row("DealDamage", true, None, None)
            }
            Action::Destroy(r)
            | Action::ReturnToHand(r)
            | Action::Counter(r)
            | Action::Unattach(r) => {
                self.reference(r, ctx, Kind::Object);
                Intro::default()
            }
            Action::Attach { what, to } => {
                self.reference(what, ctx, Kind::Object);
                self.reference(to, ctx, Kind::Object);
                Intro::default()
            }
            Action::Move(r, destination, riders) => {
                self.reference(r, ctx, Kind::Object);
                self.destination(destination, ctx);
                self.enter_riders(riders, Some(destination), ctx);
                // The moved object is a product the effect can find
                // ([CR#400.7j]); its noun follows the destination.
                self.intro_from_row("Move", true, None, Some(destination))
            }
            Action::MoveGroup {
                group,
                arrangement,
                to,
                riders,
            } => {
                self.selection(group, ctx, Kind::Object);
                if let deckmaste_core::Arrangement::ChosenOrder(chooser) = arrangement {
                    self.reference(chooser, ctx, Kind::Player);
                }
                self.destination(to, ctx);
                self.enter_riders(riders, Some(to), ctx);
                self.intro_from_row("MoveGroup", false, None, Some(to))
            }
            Action::Fight(a, b) => {
                self.reference(a, ctx, Kind::Object);
                self.reference(b, ctx, Kind::Object);
                Intro::default()
            }
            // The exchange-family control transition ([CR#701.12b]) — legal
            // only inside an exchange macro's body, like `Simultaneous`.
            Action::GainControl(what, to) => {
                if !ctx.in_exchange_macro {
                    self.err(
                        Code::PosSimultaneous,
                        "GainControl outside an exchange-family macro body — one-shot \
                         control transitions are exchange-only ([CR#701.12b]); duration-bounded \
                         control changes are the continuous layer-2 form",
                    );
                }
                self.reference(what, ctx, Kind::Object);
                self.reference(to, ctx, Kind::Player);
                Intro::default()
            }
            Action::ExtraPhase(_, player) | Action::TheRingTempts(player) => {
                self.reference(player, ctx, Kind::Player);
                Intro::default()
            }
            Action::BecomeDay | Action::BecomeNight => Intro::default(),
            Action::MoveCounters(spec, from, to) => {
                match spec {
                    CounterSpec::Named(counter, count) => {
                        self.counter_ref(counter, Kind::Object);
                        self.count(count, ctx);
                    }
                    CounterSpec::AllKinds => {}
                }
                self.reference(from, ctx, Kind::Object);
                self.reference(to, ctx, Kind::Object);
                Intro::default()
            }
            Action::CreateReplacement {
                replacement,
                subject,
                duration,
                one_shot: _,
            } => {
                self.replacement(replacement, ctx);
                self.reference(subject, ctx, Kind::Object);
                self.duration(duration, ctx);
                Intro::default()
            }
            Action::By(agent, player_action) => {
                self.reference(agent, ctx, Kind::Player);
                self.player_action(player_action, ctx)
            }
        }
    }

    fn player_action(&mut self, action: &PlayerAction, ctx: &Ctx) -> Intro {
        match action {
            PlayerAction::SetLife(count)
            | PlayerAction::FlipCoins(count)
            | PlayerAction::RollDice(count, _) => {
                self.count(count, ctx);
                Intro::default()
            }
            PlayerAction::Draw(count) => {
                self.count(count, ctx);
                // Drawn cards land in hand ([CR#121.1]) and push an amount
                // antecedent — the emitted Draw intro row.
                self.intro_from_row("Draw", count.literal_value() == Some(1), None, None)
            }
            PlayerAction::Mill(count) => {
                self.count(count, ctx);
                // Milled cards land in the graveyard ([CR#701.17a]).
                self.intro_from_row("Mill", count.literal_value() == Some(1), None, None)
            }
            PlayerAction::GainLife(count) => {
                self.count(count, ctx);
                // The amount antecedent ([CR#119.3]) — the GainLife row.
                self.intro_from_row("GainLife", true, None, None)
            }
            PlayerAction::LoseLife(count) => {
                self.count(count, ctx);
                self.intro_from_row("LoseLife", true, None, None)
            }
            PlayerAction::Discard {
                count,
                what,
                random: _,
            } => {
                self.count(count, ctx);
                if let Some(what) = what {
                    self.reference(what, ctx, Kind::Object);
                }
                // Discarded cards land in the graveyard ([CR#701.9a]) and
                // fix the card count ("…, then draws that many cards").
                self.intro_from_row("Discard", count.literal_value() == Some(1), None, None)
            }
            PlayerAction::AddMana(count, production) => {
                self.count(count, ctx);
                self.mana_production(production, ctx);
                Intro::default()
            }
            PlayerAction::Create(count, spec, riders) => {
                self.count(count, ctx);
                self.token_spec(spec);
                // A created token always enters the battlefield ([CR#111.2]),
                // so every rider is legal here — no destination gate. The
                // product arity is DERIVED from the count (the Create row).
                self.enter_riders(riders, None, ctx);
                self.intro_from_row("Create", count.literal_value() == Some(1), None, None)
            }
            PlayerAction::Sacrifice(r)
            | PlayerAction::Tap(r)
            | PlayerAction::Untap(r)
            | PlayerAction::CopySpell(r)
            | PlayerAction::RemoveDamage(r) => {
                self.reference(r, ctx, Kind::Object);
                Intro::default()
            }
            PlayerAction::Move(r, destination, riders) => {
                self.reference(r, ctx, Kind::Object);
                self.destination(destination, ctx);
                self.enter_riders(riders, Some(destination), ctx);
                // The player-verb move produces like `Action::Move`
                // ([CR#400.7j]).
                self.intro_from_row("Move", true, None, Some(destination))
            }
            PlayerAction::GetEmblem(abilities) => {
                self.abilities(abilities, false);
                Intro::default()
            }
            PlayerAction::GetDesignation(name) => {
                self.designation(name, Kind::Player);
                Intro::default()
            }
            PlayerAction::ChooseAndNote(key, kind) => Intro {
                pushes: Vec::new(),
                notes: vec![(*key, *kind)],
            },
            PlayerAction::PutCounters(r, counter, count)
            | PlayerAction::RemoveCounters(r, counter, count) => {
                let carrier = self.reference(r, ctx, Kind::Any);
                self.counter_ref(counter, carrier);
                self.count(count, ctx);
                Intro::default()
            }
            PlayerAction::Distribute { group, .. } => {
                self.selection(group, ctx, Kind::Object);
                Intro::default()
            }
            PlayerAction::WinGame
            | PlayerAction::LoseGame
            | PlayerAction::RestartGame
            | PlayerAction::Shuffle
            | PlayerAction::VentureIntoDungeon => Intro::default(),
            PlayerAction::Reveal { what, to } => {
                self.reference(what, ctx, Kind::Object);
                if let Some(to) = to {
                    self.reference(to, ctx, Kind::Player);
                }
                Intro::default()
            }
            PlayerAction::Expanded(e) => self.player_action(&e.value, ctx),
        }
    }

    fn destination(&mut self, destination: &Destination, ctx: &Ctx) {
        match destination {
            // An ordered-zone position exists only via `Library(Anchor)` — a
            // bare `Library` names no position ([CR#401.4,401.7]); the stack
            // is never a destination (objects reach it only by casting/
            // activating/triggering, [CR#405.1]). The bare-`Library` spelling
            // is unrepresentable in RON by construction (the `Library` name
            // dispatches to `Destination::Library(Anchor)`), so this arm
            // guards only programmatic construction; the Stack arm has the
            // reject fixture.
            Destination::Zone(deckmaste_core::Zone::Library) => {
                self.err(
                    Code::FloorDestination,
                    "a bare Library destination names no position — use Library(FromTop(n)) / \
                     Library(FromBottom(n)) ([CR#401.4,401.7])",
                );
            }
            Destination::Zone(deckmaste_core::Zone::Stack) => {
                self.err(
                    Code::FloorDestination,
                    "the stack is not a Move destination — objects reach it only by \
                     casting/activating/triggering ([CR#405.1])",
                );
            }
            Destination::Zone(_) => {}
            Destination::Library(Anchor::FromTop(count) | Anchor::FromBottom(count)) => {
                self.count(count, ctx);
            }
        }
    }

    /// Walk a verb's entry riders ([CR#614.12]) and enforce the
    /// battlefield-only gate: `destination` is `Some` for a `Move`/`MoveGroup`
    /// (riders demand `Zone(Battlefield)`), `None` for `Create` (a token
    /// always enters the battlefield, so every rider is legal).
    fn enter_riders(
        &mut self,
        riders: &[deckmaste_core::EnterRider],
        destination: Option<&Destination>,
        ctx: &Ctx,
    ) {
        use deckmaste_core::EnterRider;
        if riders.is_empty() {
            return;
        }
        if let Some(destination) = destination
            && !matches!(
                destination,
                Destination::Zone(deckmaste_core::Zone::Battlefield)
            )
        {
            self.err(
                Code::PosRider,
                "enter riders modify how a permanent enters the BATTLEFIELD ([CR#614.12]); \
                 this destination is not the battlefield",
            );
        }
        for (i, rider) in riders.iter().enumerate() {
            self.scoped(format!("riders[{i}]"), |w| match rider {
                EnterRider::Tapped | EnterRider::FaceDown | EnterRider::UnderOwnersControl => {}
                EnterRider::UnderControlOf(player) => {
                    w.reference(player, ctx, Kind::Player);
                }
                EnterRider::Attacking(whom) => {
                    if let Some(whom) = whom {
                        w.reference(whom, ctx, Kind::Player);
                    }
                }
                EnterRider::WithCounters(counter, count) => {
                    // The arriving object is battlefield-bound — an object
                    // carrier ([CR#122.1]).
                    w.counter_ref(counter, Kind::Object);
                    w.count(count, ctx);
                }
            });
        }
    }

    fn mana_production(&mut self, production: &ManaProduction, ctx: &Ctx) {
        match production {
            ManaProduction::Bare(_) => {}
            ManaProduction::WithRiders { mana: _, riders } => {
                for (i, rider) in riders.iter().enumerate() {
                    self.scoped(format!("riders[{i}]"), |w| w.mana_rider(rider, ctx));
                }
            }
        }
    }

    fn mana_rider(&mut self, rider: &ManaRider, ctx: &Ctx) {
        match rider {
            ManaRider::SpendOnly(filter) => {
                self.filter(filter, ctx, Kind::Object);
            }
            ManaRider::GrantOnSpend(effect) | ManaRider::TriggerOnSpend(effect) => {
                // The paid-for object binds as `It` ([CR#106.6]) — the same
                // element-binding shape as a loop body.
                let element = Ante {
                    sort: Sort::Permanent,
                    kind: Kind::Object,
                    card: Cardinality::One,
                    site: Site::Loop,
                    expected_zone: None,
                    label: None,
                    binder: false,
                };
                let body = self.descend(ctx, "Each", None, Some(element), NO_CAPS, vec![]);
                self.effect(effect, &body);
            }
            ManaRider::Persistent(_) | ManaRider::Snow => {}
            ManaRider::Expanded(e) => self.mana_rider(&e.value, ctx),
        }
    }

    fn token_spec(&mut self, spec: &TokenSpec) {
        match spec {
            TokenSpec::Token(token) => self.scoped("token", |w| w.token(token)),
            // A predefined name resolves against the rules-defined registry;
            // its body is builtin data, not this card's to validate.
            TokenSpec::Named(_) => {}
        }
    }

    // ------------------------------------------------------------------
    // costs
    // ------------------------------------------------------------------

    fn cost(&mut self, cost: &Cost, ctx: &Ctx) {
        // Normalize first so macro-spliced nested costs are still checked.
        let normalized = cost.clone().normalize();
        for (i, component) in normalized.iter().enumerate() {
            self.scoped(format!("cost[{i}]"), |w| w.cost_component(component, ctx));
        }
    }

    fn cost_component(&mut self, component: &CostComponent, ctx: &Ctx) {
        match component {
            CostComponent::Mana(_) | CostComponent::Tap | CostComponent::Untap => {}
            CostComponent::ManaCostOf(r) => {
                self.reference(r, ctx, Kind::Object);
            }
            CostComponent::Do(action) => {
                let verb = player_action_key(action);
                if self.tables.cost_action(verb).is_none() {
                    self.err(
                        Code::CostIneligible,
                        format!("{verb} is not a cost-eligible action ([CR#118.3])"),
                    );
                }
                self.player_action(action, ctx);
            }
            CostComponent::Cost(inner) => self.cost(inner, ctx),
            CostComponent::TapTotal { count, filter, .. } => {
                self.count(count, ctx);
                self.filter(filter, ctx, Kind::Object);
            }
            CostComponent::With { binder, body } => {
                let (cardinality, kind) = self.binder(binder, ctx);
                let construct = match cardinality {
                    Cardinality::One => "With.One",
                    Cardinality::Many => "With.Many",
                };
                let ante = self.binder_ante(binder, cardinality, kind);
                let inner = self.descend(ctx, construct, None, Some(ante), NO_CAPS, vec![]);
                self.cost(body, &inner);
            }
            CostComponent::Expanded(e) => self.cost_component(&e.value, ctx),
        }
    }

    /// The caps a cost's payment supplies an `AdditionalCost` body — the
    /// Idris `costCaps`: the union over the components' payment events.
    fn cost_caps(&self, cost: &Cost) -> Caps {
        let normalized = cost.clone().normalize();
        normalized.iter().fold(NO_CAPS, |acc, component| {
            acc.join(self.component_caps(component))
        })
    }

    fn component_caps(&self, component: &CostComponent) -> Caps {
        match component {
            CostComponent::Do(action) => self
                .tables
                .cost_action(player_action_key(action))
                .unwrap_or(NO_CAPS),
            CostComponent::Cost(inner) => self.cost_caps(inner),
            CostComponent::With { body, .. } => self.cost_caps(body),
            CostComponent::Expanded(e) => self.component_caps(&e.value),
            CostComponent::Mana(_)
            | CostComponent::ManaCostOf(_)
            | CostComponent::Tap
            | CostComponent::Untap
            | CostComponent::TapTotal { .. } => NO_CAPS,
        }
    }

    // ------------------------------------------------------------------
    // binders and selections
    // ------------------------------------------------------------------

    fn binder(&mut self, binder: &Binder, ctx: &Ctx) -> (Cardinality, Kind) {
        match binder {
            Binder::TheRef(r) => {
                let kind = self.reference(r, ctx, Kind::Any);
                (Cardinality::One, kind)
            }
            Binder::ChooseOne { filter, by } => {
                self.reference(by, ctx, Kind::Player);
                let kind = self.filter(filter, ctx, Kind::Any);
                (Cardinality::One, kind)
            }
            Binder::Produce(action) => {
                self.action(action, ctx);
                (Cardinality::One, Kind::Object)
            }
            Binder::SearchOne {
                filter,
                by,
                whose,
                if_none,
                ..
            } => {
                self.reference(by, ctx, Kind::Player);
                self.reference(whose, ctx, Kind::Player);
                let kind = self.filter(filter, ctx, Kind::Object);
                // The whiff branch elaborates in the PRE-binding context
                // ([CR#701.23b]) — nothing was found, so the search's product
                // is NOT in scope; reading it there is a load error.
                if let Some(if_none) = if_none {
                    self.scoped("if_none", |w| {
                        w.effect(if_none, ctx);
                    });
                }
                (Cardinality::One, kind)
            }
            Binder::Choose {
                quantity,
                filter,
                by,
            } => {
                self.quantity(quantity, ctx, false);
                self.reference(by, ctx, Kind::Player);
                let kind = self.filter(filter, ctx, Kind::Any);
                (Cardinality::Many, kind)
            }
            Binder::Existing(selection) => {
                let kind = self.selection(selection, ctx, Kind::Any);
                (Cardinality::Many, kind)
            }
            Binder::Search {
                quantity,
                filter,
                by,
                whose,
                if_none,
                ..
            } => {
                self.quantity(quantity, ctx, false);
                self.reference(by, ctx, Kind::Player);
                self.reference(whose, ctx, Kind::Player);
                let kind = self.filter(filter, ctx, Kind::Object);
                // Pre-binding context, as on `SearchOne` ([CR#701.23b]).
                if let Some(if_none) = if_none {
                    self.scoped("if_none", |w| {
                        w.effect(if_none, ctx);
                    });
                }
                (Cardinality::Many, kind)
            }
            Binder::Expanded(e) => self.binder(&e.value, ctx),
        }
    }

    fn selection(&mut self, selection: &Selection, ctx: &Ctx, expected: Kind) -> Kind {
        let kind = match selection {
            Selection::Filter(filter) => self.filter(filter, ctx, Kind::Any),
            Selection::Union(members) => members.iter().fold(Kind::Empty, |acc, member| {
                let kind = self.selection(member, ctx, Kind::Any);
                self.tables.join(acc, kind)
            }),
            Selection::Random(quantity, filter) => {
                self.quantity(quantity, ctx, false);
                self.filter(filter, ctx, Kind::Any)
            }
            Selection::AmongNoted(key, quantity) => {
                match noted_kind(ctx, key) {
                    Some(NotedKind::Objects) => {
                        self.resolve(|| {
                            format!(
                                "AmongNoted({:?}) -> bound to a prior Objects noting",
                                key.as_str()
                            )
                        });
                    }
                    Some(kind) => {
                        self.err(
                            Code::KindNoteDomain,
                            format!(
                                "AmongNoted({:?}) reads an object set, but the key stores \
                                 {kind:?} ([CR#607.2])",
                                key.as_str()
                            ),
                        );
                    }
                    None => {
                        self.err(
                            Code::BindNote,
                            format!(
                                "AmongNoted reads key {:?} but nothing noted it",
                                key.as_str()
                            ),
                        );
                    }
                }
                self.quantity(quantity, ctx, false);
                Kind::Object
            }
            Selection::TopOfLibrary { count, of } | Selection::BottomOfLibrary { count, of } => {
                self.count(count, ctx);
                self.reference(of, ctx, Kind::Player);
                Kind::Object
            }
            // The plural anaphors: inside a `With`/pile choice frame they
            // bind the frame deterministically (legacy semantics);
            // otherwise R1 nearest Many antecedent (wildcard / sorted) +
            // the R2 uniqueness gate ([CR#608.2d]).
            Selection::They => match self.plural_anaphor(ctx, "They", None, expected) {
                Some(kind) => kind,
                None => Kind::Any,
            },
            Selection::Them(sort) => {
                match self.plural_anaphor(ctx, &format!("Them({sort:?})"), Some(*sort), expected) {
                    Some(kind) => kind,
                    None => Kind::Any,
                }
            }
            // The labeled plural fallback — piles and `Label`ed groups
            // ([CR#700.3,608.2d]); explicit, never gated.
            Selection::TheGroup(label) => {
                match self.resolve_label(ctx, "TheGroup", *label, Cardinality::Many) {
                    Some(ante) => ante.kind,
                    None => Kind::Any,
                }
            }
            // Piles noted earlier under a key, per divider ([CR#700.3]).
            Selection::PilesOf { note, of } => {
                self.reference(of, ctx, Kind::Player);
                match noted_kind(ctx, note) {
                    Some(NotedKind::Piles) => {
                        self.resolve(|| {
                            format!(
                                "PilesOf({:?}) -> bound to a prior pile noting",
                                note.as_str()
                            )
                        });
                    }
                    Some(kind) => {
                        self.err(
                            Code::KindNoteDomain,
                            format!(
                                "PilesOf({:?}) reads piles, but the key stores {kind:?} \
                                 ([CR#607.2])",
                                note.as_str()
                            ),
                        );
                    }
                    None => {
                        self.err(
                            Code::BindNote,
                            format!("PilesOf reads key {:?} but nothing noted it", note.as_str()),
                        );
                    }
                }
                Kind::Object
            }
            Selection::Pick { op: _, of, by } => {
                self.filter(of, ctx, Kind::Object);
                let candidate = Ante {
                    sort: self.filter_sort(of),
                    kind: Kind::Object,
                    card: Cardinality::One,
                    site: Site::Loop,
                    expected_zone: None,
                    label: None,
                    binder: false,
                };
                let body = self.descend(ctx, "Pick", None, Some(candidate), NO_CAPS, vec![]);
                self.count(by, &body);
                Kind::Object
            }
            // The recursion applies the slot expectation itself; returning
            // here avoids double-reporting.
            Selection::Expanded(e) => return self.selection(&e.value, ctx, expected),
        };
        self.expect_kind(kind, expected, "selection");
        kind
    }

    // ------------------------------------------------------------------
    // references, filters, kinds
    // ------------------------------------------------------------------

    fn expect_kind(&mut self, kind: Kind, expected: Kind, what: &str) {
        if !kind.compatible_with(expected) {
            self.err(
                Code::KindFilter,
                format!("{what} is {kind:?}-kinded where {expected:?} is required"),
            );
        }
    }

    #[expect(
        clippy::too_many_lines,
        reason = "one arm per Reference variant; the flat dispatch reads better whole"
    )]
    fn reference(&mut self, reference: &Reference, ctx: &Ctx, expected: Kind) -> Kind {
        let kind = match reference {
            Reference::This => Kind::Object,
            Reference::You | Reference::Opponent => Kind::Player,
            Reference::It => {
                // A binder body's `It` is ALWAYS the innermost bound
                // thing — a loop element or a `With`/pile choice —
                // deterministic, never gated ([CR#608.2]; the pre-stack
                // semantics, kept). Outside every binder, `It` is the
                // wildcard singular anaphor over the stack (R1 nearest +
                // the R2 gate).
                if let Some((depth, ante)) = ctx.innermost_binder() {
                    let kind = ante.kind;
                    let description = ante.describe();
                    self.resolve(|| format!("It -> #{depth} {description} (loop element)"));
                    kind
                } else if ctx.stack.is_empty() {
                    self.err(
                        Code::BindIt,
                        "It read with no antecedent in scope (no loop binder, empty stack)",
                    );
                    Kind::Any
                } else {
                    match self.resolve_anaphor(
                        ctx,
                        "It",
                        None,
                        Cardinality::One,
                        expected,
                        Code::BindIt,
                    ) {
                        Some(ante) => ante.kind,
                        None => Kind::Any,
                    }
                }
            }
            // The sorted singular anaphor: inside a `With`/pile choice
            // frame it binds the frame deterministically (the legacy
            // semantics — never R2-gated); otherwise R1 nearest of this
            // sort + the R2 uniqueness gate ([CR#608.2d]).
            Reference::That(sort) => {
                if let Some((depth, ante)) = ctx.innermost_frame() {
                    if ante.card != Cardinality::One {
                        self.err(
                            Code::BindThat,
                            "singular That read where the enclosing binder binds a group \
                             (iterate it with Each, or read They)",
                        );
                        Kind::Any
                    } else if self.sort_reaches(Some(*sort), ante.sort).is_none() {
                        self.err(
                            Code::BindThat,
                            format!(
                                "That({sort:?}) is incompatible with the enclosing binder's \
                                 {:?} choice",
                                ante.sort
                            ),
                        );
                        Kind::Any
                    } else {
                        let kind = ante.kind;
                        let description = ante.describe();
                        self.resolve(|| {
                            format!("That({sort:?}) -> #{depth} {description} (binder frame)")
                        });
                        kind
                    }
                } else {
                    match self.resolve_anaphor(
                        ctx,
                        &format!("That({sort:?})"),
                        Some(*sort),
                        Cardinality::One,
                        expected,
                        Code::BindThat,
                    ) {
                        Some(ante) => ante.kind,
                        None => Kind::Any,
                    }
                }
            }
            // The labeled fallback ([CR#608.2d]) — explicit, never gated.
            Reference::The(label) => {
                match self.resolve_label(ctx, "The", *label, Cardinality::One) {
                    Some(ante) => ante.kind,
                    None => Kind::Any,
                }
            }
            // The indefinite determiner ([CR#608.2d]): the actor chooses a
            // matching object at resolution; the choice pushes a Chosen
            // antecedent for the clauses to the right.
            Reference::A { filter, by } => {
                self.reference(by, ctx, Kind::Player);
                let kind = self.filter(filter, ctx, Kind::Object);
                let sort = self.filter_sort(filter);
                self.resolve(|| format!("A(…) -> chooses a {sort:?} ({kind:?})"));
                self.pending.push(Ante {
                    sort,
                    kind,
                    card: Cardinality::One,
                    site: Site::Chosen,
                    expected_zone: None,
                    label: None,
                    binder: false,
                });
                kind
            }
            Reference::EventObject => {
                if ctx.caps.object {
                    self.resolve(|| "EventObject -> Object (the enclosing event's object)".into());
                } else if ctx.in_event {
                    self.err(
                        Code::CapsObject,
                        "EventObject read where the event supplies no object",
                    );
                } else {
                    self.err(Code::BindEvent, "EventObject read outside any event body");
                }
                Kind::Object
            }
            Reference::EventPatient => {
                if let Some(kind) = ctx.caps.patient {
                    self.resolve(|| {
                        format!(
                            "EventPatient -> {kind:?} (the enclosing event's fixed patient kind)"
                        )
                    });
                    kind
                } else {
                    if ctx.in_event {
                        self.err(
                            Code::CapsPatient,
                            "EventPatient read where the event fixes no patient kind",
                        );
                    } else {
                        self.err(Code::BindEvent, "EventPatient read outside any event body");
                    }
                    Kind::Any
                }
            }
            Reference::EventActor => {
                if ctx.caps.actor {
                    self.resolve(|| "EventActor -> Player (the enclosing event's actor)".into());
                } else if ctx.in_event {
                    self.err(
                        Code::CapsActor,
                        "EventActor read where the event supplies no actor",
                    );
                } else {
                    self.err(Code::BindEvent, "EventActor read outside any event body");
                }
                Kind::Player
            }
            Reference::DefendingPlayer => {
                if ctx.caps.defender {
                    self.resolve(|| {
                        "DefendingPlayer -> Player (the enclosing combat onset's defender)".into()
                    });
                } else if ctx.in_event {
                    self.err(
                        Code::CapsDefender,
                        "DefendingPlayer read where no combat onset supplies one",
                    );
                } else {
                    self.err(
                        Code::BindEvent,
                        "DefendingPlayer read outside any event body",
                    );
                }
                Kind::Player
            }
            // Engine-seam references: named-role and linked-value bindings
            // are resolved by engine machinery this walk doesn't model.
            Reference::Bound(name) => {
                self.resolve(|| {
                    format!("Bound({name:?}) -> resolved by engine machinery at runtime")
                });
                Kind::Any
            }
            Reference::Linked(name) => {
                self.resolve(|| {
                    format!("Linked({name:?}) -> resolved by engine machinery at runtime")
                });
                Kind::Any
            }
            Reference::ControllerOf(inner) | Reference::OwnerOf(inner) => {
                self.reference(inner, ctx, Kind::Object);
                Kind::Player
            }
            Reference::AttachHostOf(inner) | Reference::AttachedTo(inner) => {
                self.reference(inner, ctx, Kind::Object);
                Kind::Object
            }
            // The recursion applies the slot expectation itself; returning
            // here avoids double-reporting.
            Reference::Expanded(e) => return self.reference(&e.value, ctx, expected),
        };
        self.expect_kind(kind, expected, "reference");
        kind
    }

    fn filter(&mut self, filter: &Filter, ctx: &Ctx, expected: Kind) -> Kind {
        let kind = self.filter_kind(filter, ctx);
        self.expect_kind(kind, expected, "filter");
        kind
    }

    fn filter_kind(&mut self, filter: &Filter, ctx: &Ctx) -> Kind {
        match filter {
            Filter::Kind(deckmaste_core::ObjectKind::Player) => Kind::Player,
            Filter::Kind(_) => Kind::Object,
            Filter::Characteristic(atom) => {
                if let CharacteristicFilter::Stat(_, _, count) = atom {
                    self.count(count, ctx);
                }
                Kind::Object
            }
            Filter::State(atom) => self.state_filter_kind(atom, ctx),
            Filter::Relation(relation) => self.relation_filter_kind(relation, ctx),
            Filter::Ref(reference) => self.reference(reference, ctx, Kind::Any),
            Filter::AllOf(members) => {
                // Conjuncts share one candidate ([CR#109.1]): a definite
                // object atom and a definite player atom can't both hold.
                let mut kind = Kind::Any;
                for member in members {
                    let member_kind = self.filter_kind(member, ctx);
                    kind = match (kind, member_kind) {
                        (Kind::Any, k) | (k, Kind::Any) => k,
                        (a, b) if a == b => a,
                        (a, b) => {
                            self.err(
                                Code::KindFilter,
                                format!("AllOf mixes {a:?}- and {b:?}-kinded atoms"),
                            );
                            Kind::Any
                        }
                    };
                }
                kind
            }
            Filter::OneOf(members) => members.iter().fold(Kind::Empty, |acc, member| {
                let kind = self.filter_kind(member, ctx);
                self.tables.join(acc, kind)
            }),
            Filter::Not(inner) => self.filter_kind(inner, ctx),
            Filter::FromSource(inner) => {
                // [CR#702.11d]: a stack ABILITY whose source matches — the
                // inner filter judges the source OBJECT ([CR#113.7]), and
                // the atom itself denotes an object (the ability).
                self.filter(inner, ctx, Kind::Object);
                Kind::Object
            }
            Filter::Where(condition) => {
                // The candidate binds as `It` for the condition ([CR#603.4]).
                let candidate = Ante {
                    sort: Sort::Permanent,
                    kind: Kind::Any,
                    card: Cardinality::One,
                    site: Site::Loop,
                    expected_zone: None,
                    label: None,
                    binder: false,
                };
                let body = self.descend(ctx, "Where", None, Some(candidate), NO_CAPS, vec![]);
                self.condition(condition, &body);
                Kind::Any
            }
            Filter::Any => Kind::Any,
            Filter::Expanded(e) => self.filter_kind(&e.value, ctx),
        }
    }

    fn state_filter_kind(&mut self, atom: &StateFilter, ctx: &Ctx) -> Kind {
        match atom {
            StateFilter::InZone(_)
            | StateFilter::Status(_)
            | StateFilter::Attacking
            | StateFilter::Blocking
            | StateFilter::Unblocked
            // The paid-cost linkage tests an OBJECT's own optional cost
            // ([CR#702.33d..702.33e]).
            | StateFilter::WasPaidWith(_) => Kind::Object,
            StateFilter::HasCounter(counter) => {
                // The atom's kind IS the counter's carrier scope.
                self.counter_declared(counter);
                self.counter_scope_of(counter)
            }
            StateFilter::Designated(name) => self.designation_scope_of(name).unwrap_or(Kind::Any),
            StateFilter::RelatedBy(_, inner) | StateFilter::Targets(inner) => {
                self.filter(inner, ctx, Kind::Any);
                Kind::Object
            }
            StateFilter::TargetCount(bound) => {
                self.count_bound(bound, ctx);
                Kind::Object
            }
        }
    }

    fn relation_filter_kind(&mut self, relation: &RelationFilter, ctx: &Ctx) -> Kind {
        match relation {
            RelationFilter::ControlledBy(player) | RelationFilter::Owner(player) => {
                self.filter(player, ctx, Kind::Player);
                Kind::Object
            }
            RelationFilter::Controls(object) => {
                self.filter(object, ctx, Kind::Object);
                Kind::Player
            }
            RelationFilter::OpponentOf(player) | RelationFilter::TeammateOf(player) => {
                self.filter(player, ctx, Kind::Player);
                Kind::Player
            }
            RelationFilter::AttachedTo(object) | RelationFilter::Attachment(object) => {
                self.filter(object, ctx, Kind::Object);
                Kind::Object
            }
        }
    }

    fn counter_declared(&mut self, counter: &CounterRef) {
        if !self.registries.counters.contains_key(&counter.0) {
            self.err(
                Code::KindCounterUndeclared,
                format!(
                    "counter reference {:?} names no declared counter kind",
                    counter.as_str()
                ),
            );
        }
    }

    /// A counter kind's carrier scope ([CR#122.1]): the LOADED registry
    /// row's `scope` column is the authority; an undeclared kind (already an
    /// `E-KIND-COUNTER-UNDECLARED` finding) falls back to the emitted
    /// curated table and its object default.
    fn counter_scope_of(&self, counter: &CounterRef) -> Kind {
        match self.registries.counters.get(&counter.0) {
            Some(decl) => match decl.scope {
                CounterScope::Object => Kind::Object,
                CounterScope::Player => Kind::Player,
            },
            None => self.tables.counter_scope(counter.as_str()),
        }
    }

    fn counter_ref(&mut self, counter: &CounterRef, carrier: Kind) {
        self.counter_declared(counter);
        let scope = self.counter_scope_of(counter);
        if carrier.compatible_with(scope) {
            self.resolve(|| format!("counter {:?} -> {scope:?}-borne", counter.as_str()));
        } else {
            self.err(
                Code::KindCounterScope,
                format!(
                    "{:?} counters are {scope:?}-borne but the carrier is {carrier:?} \
                     ([CR#122.1])",
                    counter.as_str()
                ),
            );
        }
    }

    /// A designation's carrier scope: the LOADED registry row first (its
    /// `Stored` scope column; derived designations judge objects; a
    /// game-scoped designation constrains no carrier), then the emitted
    /// curated table — unknown names are an open vocabulary and impose no
    /// constraint.
    fn designation_scope_of(&self, name: &Ident) -> Option<Kind> {
        match self.registries.designations.get(name) {
            Some(decl) => designation_decl_scope(decl),
            None => self.tables.designation_scope(name.as_str()),
        }
    }

    fn designation(&mut self, name: &Ident, carrier: Kind) {
        if let Some(scope) = self.designation_scope_of(name) {
            if carrier.compatible_with(scope) {
                self.resolve(|| format!("designation {:?} -> {scope:?}-borne", name.as_str()));
            } else {
                self.err(
                    Code::KindDesignationScope,
                    format!(
                        "designation {:?} is {scope:?}-borne but the carrier is {carrier:?}",
                        name.as_str()
                    ),
                );
            }
        }
    }

    // ------------------------------------------------------------------
    // counts, quantities, conditions
    // ------------------------------------------------------------------

    fn count(&mut self, count: &Count, ctx: &Ctx) {
        match count {
            Count::X => {
                if ctx.has_x {
                    self.resolve(|| "X -> bound to the carrying cost's {X}".into());
                } else {
                    self.err(
                        Code::CostX,
                        "X read where the carrying cost declares no {X}",
                    );
                }
            }
            Count::CountOf(filter) | Count::CountDistinct(_, filter) => {
                self.filter(filter, ctx, Kind::Any);
            }
            Count::StatOf(r, _) | Count::Damage(r) => {
                self.reference(r, ctx, Kind::Object);
            }
            Count::CounterCount(r, counter) => {
                let carrier = self.reference(r, ctx, Kind::Any);
                self.counter_ref(counter, carrier);
            }
            Count::Min(a, b)
            | Count::Max(a, b)
            | Count::Plus(a, b)
            | Count::Minus(a, b)
            | Count::Times(a, b) => {
                self.count(a, ctx);
                self.count(b, ctx);
            }
            Count::Half(_, inner) => self.count(inner, ctx),
            // The ONE value anaphor, two spellings ([CR#107.3,608.2i]):
            // "that many" (countable) / "that much" (uncountable) — the
            // nearest Amount antecedent, R2-gated like the object anaphors.
            Count::ThatMany | Count::ThatMuch => {
                let spelling = match count {
                    Count::ThatMany => "ThatMany",
                    _ => "ThatMuch",
                };
                self.resolve_anaphor(
                    ctx,
                    spelling,
                    Some(Sort::Amount),
                    Cardinality::One,
                    Kind::Any,
                    Code::CapsAmount,
                );
            }
            Count::Allotment => {
                // The `DivideAmong` share — the nearest `Allot`-sited
                // antecedent, read only by this spelling ([CR#601.2d]).
                let found = ctx
                    .stack
                    .iter()
                    .rev()
                    .enumerate()
                    .find(|(_, a)| a.site == Site::Allot);
                if let Some((depth, _)) = found {
                    self.resolve(|| {
                        format!("Allotment -> #{depth} the enclosing DivideAmong share")
                    });
                } else {
                    self.err(
                        Code::BindAllotment,
                        "Allotment read outside a DivideAmong body",
                    );
                }
            }
            Count::EventCount(event, within) => {
                self.event(event, ctx, Lane::History);
                self.bridge_gate_lookback(*within, Lane::History);
            }
            Count::EventSum(event, within) => {
                self.event(event, ctx, Lane::History);
                self.bridge_gate_lookback(*within, Lane::History);
                if !self.event_caps(event).amount {
                    self.err(
                        Code::CapsAmount,
                        "EventSum over an event that guarantees no amount",
                    );
                }
            }
            Count::Noted(key) => match noted_kind(ctx, key) {
                Some(NotedKind::Number) => {
                    self.resolve(|| {
                        format!(
                            "Noted({:?}) -> bound to a prior Number noting",
                            key.as_str()
                        )
                    });
                }
                Some(kind) => {
                    self.err(
                        Code::KindNoteDomain,
                        format!(
                            "Noted({:?}) reads a number, but the key stores {kind:?} \
                             ([CR#607.2])",
                            key.as_str()
                        ),
                    );
                }
                None => {
                    self.err(
                        Code::BindNote,
                        format!("Noted reads key {:?} but nothing noted it", key.as_str()),
                    );
                }
            },
            // The paid-cost count reads the object's own announce record
            // ([CR#702.33c..702.33d]) — a leaf, like `X`.
            Count::TimesPaid(_) | Count::Literal(_) => {}
            Count::Expanded(e) => self.count(&e.value, ctx),
        }
    }

    fn count_bound(&mut self, bound: &CountBound, ctx: &Ctx) {
        match bound {
            CountBound::Eq(count)
            | CountBound::AtLeast(count)
            | CountBound::AtMost(count)
            | CountBound::Greater(count)
            | CountBound::Less(count) => self.count(count, ctx),
        }
    }

    /// Quantity floors: literal bounds must be ordered ([CR#115.6]); a
    /// target slot's upper bound must permit ≥1 target ([CR#115.1]).
    fn quantity(&mut self, quantity: &Quantity, ctx: &Ctx, target_slot: bool) {
        let (lo, hi) = quantity.bounds();
        if let Some(lo) = lo {
            self.count(lo, ctx);
        }
        if let Some(hi) = hi {
            self.count(hi, ctx);
        }
        if let (Some(lo), Some(hi)) = (
            lo.and_then(Count::literal_value),
            hi.and_then(Count::literal_value),
        ) && lo > hi
        {
            self.err(
                Code::FloorRange,
                format!("inverted literal range: {lo} to {hi}"),
            );
        }
        if target_slot && hi.and_then(Count::literal_value) == Some(0) {
            self.err(
                Code::FloorTargetQty,
                "a target slot cannot target zero things ([CR#115.1])",
            );
        }
    }

    /// [CR#509.1]: a declared block involves at least one blocker (an
    /// arrangement unable to comply is an illegal declaration), so a
    /// block-arrangement bound whose statically-known upper limit is zero
    /// (`Eq(0)`/`AtMost(0)`/`Less(1)`) can never match a legal block — the
    /// deontic row is dead. The set-level twin of `quantity`'s zero-target
    /// floor ([CR#115.1]); menace's `Less(2)` ([CR#702.111b]) has upper
    /// limit 1 and passes.
    fn block_qty_floor(&mut self, bound: &CountBound) {
        let upper = match bound {
            CountBound::Eq(c) | CountBound::AtMost(c) => c.literal_value(),
            CountBound::Less(c) => c.literal_value().map(|n| n.saturating_sub(1)),
            CountBound::AtLeast(_) | CountBound::Greater(_) => None,
        };
        if upper == Some(0) {
            self.err(
                Code::FloorBlockQty,
                "a block-arrangement bound satisfiable only by zero blockers ([CR#509.1])",
            );
        }
    }

    fn condition(&mut self, condition: &Condition, ctx: &Ctx) {
        match condition {
            Condition::Compare(a, _, b) => {
                self.count(a, ctx);
                self.count(b, ctx);
            }
            Condition::Exists(filter) => {
                self.filter(filter, ctx, Kind::Any);
            }
            Condition::Is(r, filter) => {
                let kind = self.reference(r, ctx, Kind::Any);
                self.filter(filter, ctx, kind);
            }
            Condition::LegallyAttached(r) | Condition::DamagedByDeathtouch(r) => {
                self.reference(r, ctx, Kind::Object);
            }
            Condition::Happened { event, within } => {
                self.event(event, ctx, Lane::History);
                self.bridge_gate_lookback(*within, Lane::History);
            }
            // [CR#714.2b]: reads the firing fact's before/after totals —
            // only a counter-placement event fixes that channel.
            Condition::Crossed { value, threshold } => {
                if !ctx.in_event {
                    self.err(
                        Code::BindEvent,
                        "Crossed outside any event body — it reads the firing fact's \
                         before/after totals ([CR#714.2b])",
                    );
                } else if !ctx.crossing {
                    self.err(
                        Code::CapsAmount,
                        "Crossed under an event that fixes no before/after totals — only a \
                         counter placement does ([CR#714.2b,122.1])",
                    );
                } else {
                    self.resolve(|| {
                        "Crossed -> bound to the counter event's before/after channel".into()
                    });
                }
                self.count(value, ctx);
                let (_, bound) = threshold.split();
                self.count(bound, ctx);
            }
            // The paid-cost flag reads the object's own announce record
            // ([CR#702.33d]) — a leaf.
            Condition::YourTurn | Condition::DuringPhase(_) | Condition::PaidCost(_) => {}
            Condition::TurnOf(filter) => {
                self.filter(filter, ctx, Kind::Player);
            }
            Condition::AllOf(members) | Condition::OneOf(members) => {
                for member in members {
                    self.condition(member, ctx);
                }
            }
            Condition::Not(inner) => self.condition(inner, ctx),
            Condition::Expanded(e) => self.condition(&e.value, ctx),
        }
    }

    // ------------------------------------------------------------------
    // events
    // ------------------------------------------------------------------

    /// Walks an event PATTERN's embedded filters/references (in the context
    /// BEFORE the event binds — a pattern is not its own body), enforcing
    /// the LANE gates (the emitted `event-lanes.ron` rows: `Within` only in
    /// history lanes, `OneOrMore`/`Nth` per row, kind-anchoring where the
    /// lane requires it) and the residual kind-consistency pass.
    fn event(&mut self, event: &EventFilter, ctx: &Ctx, lane: Lane) {
        let before = self.errors.len();
        let outer_pending = std::mem::take(&mut self.bridge_pending);
        if self.tables.lane(lane.key()).anchored && !anchored(event) {
            self.err(
                Code::CapsAnchor,
                format!(
                    "the {} lane is kind-anchored: every disjunct must bottom out \
                     in a master form (a bare Not never anchors)",
                    lane.key()
                ),
            );
        }
        self.event_node(event, ctx, lane);
        self.event_kinds(event);
        // The bridge cap speaks LAST (`engine-eventfilter-bridge`): a
        // pattern the grammar's own rules already refused never reaches the
        // engine's matchers, so only an otherwise-admissible pattern reports
        // its caps — reject fixtures stay one-code-minimal.
        let pending = std::mem::replace(&mut self.bridge_pending, outer_pending);
        if self.errors.len() == before {
            for (atom, lane) in pending {
                self.bridge_cap_err(&atom, lane);
            }
        }
    }

    /// The `engine-eventfilter-bridge` cap gate (`E-BRIDGE-CAP`): until the
    /// one-evaluator rebase, each lane's patterns run on one of the engine's
    /// three bridge matchers, and an atom that matcher cannot faithfully
    /// evaluate is refused at load — never silently mis-matched
    /// ([CR#603.2]). Support is per-atom emitted data (`bridge-caps.ron`);
    /// the lane→matcher mapping rides the lane table. Findings are DEFERRED
    /// to the enclosing [`Self::event`] so the cap reports only
    /// otherwise-admissible patterns.
    fn bridge_gate(&mut self, atom: &str, lane: Lane) {
        let matcher = self.tables.lane(lane.key()).matcher;
        if !self.tables.bridge_supports(atom, matcher) {
            self.bridge_pending.push((atom.to_owned(), lane));
        }
    }

    /// Emits one bridge-cap finding (the flush half of [`Self::bridge_gate`];
    /// also called directly for the history-window positions outside an
    /// event pattern).
    fn bridge_cap_err(&mut self, atom: &str, lane: Lane) {
        self.err(
            Code::BridgeCap,
            format!(
                "{atom} is beyond the {} lane's bridge matcher — \
                 load-capped until engine-one-evaluator",
                lane.key()
            ),
        );
    }

    /// The bridge gate for a HISTORY WINDOW position (`Happened` /
    /// `EventCount` / `EventSum` lookbacks) — immediate, since it sits
    /// outside the pattern walk's deferral scope.
    fn bridge_gate_lookback(&mut self, within: deckmaste_core::Lookback, lane: Lane) {
        let atom = lookback_atom(within);
        let matcher = self.tables.lane(lane.key()).matcher;
        if !self.tables.bridge_supports(atom, matcher) {
            self.bridge_cap_err(atom, lane);
        }
    }

    /// One node of the pattern walk: master forms walk their filter slots
    /// with the slot's expected kind; algebra nodes apply their lane gates
    /// and recurse. Every node passes the [`Self::bridge_gate`] — master
    /// forms by their key here, refinement atoms per arm below.
    #[expect(
        clippy::too_many_lines,
        reason = "one arm per master form; splitting would scatter the form list"
    )]
    fn event_node(&mut self, event: &EventFilter, ctx: &Ctx, lane: Lane) {
        if let Some(key) = form_key(event) {
            self.bridge_gate(key, lane);
        }
        match event {
            EventFilter::ZoneChange {
                what,
                from,
                to,
                cause,
            } => {
                self.filter(what, ctx, Kind::Object);
                // The moved object is matched by SNAPSHOT ([CR#603.10a]).
                if where_in_spine(what) {
                    self.bridge_gate("Where-in-snapshot", lane);
                }
                if let Some(deckmaste_core::Cause::Cause(pattern)) = cause {
                    if let Some(agent) = &pattern.agent {
                        self.filter(agent, ctx, Kind::Any);
                        self.bridge_gate("Cause:agent", lane);
                    }
                    // Entailment consistency ([CR#603.2g] — a pattern whose
                    // fixed coordinates contradict its verb's entailed fact
                    // form can never match).
                    if let Some(verb) = pattern.verb
                        && let Some(row) = self.tables.entailment(verb.as_str())
                    {
                        if row.kind != "ZoneChange" {
                            self.err(
                                Code::CapsContradiction,
                                format!(
                                    "cause verb {} entails a {} fact — a ZoneChange \
                                     pattern narrowed by it can never match",
                                    verb.as_str(),
                                    row.kind
                                ),
                            );
                        }
                        for (slot, fixed, entailed) in
                            [("from", *from, row.from), ("to", *to, row.to)]
                        {
                            if let (Some(p), Some(e)) = (fixed, entailed)
                                && p != e
                            {
                                self.err(
                                    Code::CapsContradiction,
                                    format!(
                                        "{slot}: {p:?} contradicts cause verb {}'s \
                                         entailed {slot}: {e:?} ({})",
                                        verb.as_str(),
                                        row.cite
                                    ),
                                );
                            }
                        }
                    }
                }
            }
            EventFilter::Damage {
                source,
                to,
                combat,
                amount,
            } => {
                self.filter(source, ctx, Kind::Object);
                self.filter(to, ctx, Kind::Any);
                if combat.is_some() {
                    self.bridge_gate("Damage:combat", lane);
                }
                if amount.is_some() {
                    self.bridge_gate("Damage:amount", lane);
                }
            }
            EventFilter::LifeGained { who, amount }
            | EventFilter::LifeLost { who, amount }
            | EventFilter::Drawn { who, amount } => {
                self.filter(who, ctx, Kind::Player);
                if amount.is_some() {
                    // `form_key` is total over master forms, so the
                    // refinement atom is the form's own key.
                    let atom = format!("{}:amount", form_key(event).expect("a master form"));
                    self.bridge_gate(&atom, lane);
                }
            }
            EventFilter::CounterPlaced { kind, on, amount }
            | EventFilter::CounterRemoved { kind, on, amount } => {
                let carrier = self.filter(on, ctx, Kind::Any);
                if let Some(counter) = kind {
                    self.counter_ref(counter, carrier);
                }
                if amount.is_some() {
                    let atom = format!("{}:amount", form_key(event).expect("a master form"));
                    self.bridge_gate(&atom, lane);
                }
            }
            EventFilter::Cast { who, what } | EventFilter::ActivatedAb { who, what } => {
                self.filter(who, ctx, Kind::Player);
                self.filter(what, ctx, Kind::Object);
            }
            EventFilter::Played { who, what } => {
                self.filter(who, ctx, Kind::Player);
                self.filter(what, ctx, Kind::Object);
                // The played card is matched by SNAPSHOT ([CR#603.10a]).
                if where_in_spine(what) {
                    self.bridge_gate("Where-in-snapshot", lane);
                }
            }
            EventFilter::AttackDeclared { by, against } => {
                self.filter(by, ctx, Kind::Object);
                self.filter(against, ctx, Kind::Player);
            }
            EventFilter::BlockDeclared { by, of } => {
                self.filter(by, ctx, Kind::Object);
                self.filter(of, ctx, Kind::Object);
            }
            EventFilter::Attached { what, to } => {
                self.filter(what, ctx, Kind::Object);
                // An attachment host may be an object or a player
                // ([CR#701.3a] "to an object or player").
                self.filter(to, ctx, Kind::Any);
            }
            EventFilter::StateBecame { of, becomes: _ } => {
                self.filter(of, ctx, Kind::Object);
            }
            EventFilter::BecomesTarget { what, by, source } => {
                self.filter(what, ctx, Kind::Any);
                self.filter(by, ctx, Kind::Object);
                if let Some(source) = source {
                    self.filter(source, ctx, Kind::Object);
                    // The hexproof-from arm has no fact-side data yet
                    // ([CR#702.11d,702.16b]).
                    self.bridge_gate("BecomesTarget:source", lane);
                }
            }
            EventFilter::StepBegins { .. } | EventFilter::BecameDay | EventFilter::BecameNight => {}
            EventFilter::ControlChanged { of, to } => {
                self.filter(of, ctx, Kind::Object);
                self.filter(to, ctx, Kind::Player);
            }
            EventFilter::DesignationChanged { name, of } => {
                let expected = self.designation_scope_of(name).unwrap_or(Kind::Any);
                self.filter(of, ctx, expected);
            }
            EventFilter::TokenCreated { what, by } => {
                self.filter(what, ctx, Kind::Object);
                self.filter(by, ctx, Kind::Player);
                // The fact carries the token SPEC — no minted object for a
                // `what` filter to run against ([CR#701.7a]).
                if !is_any_filter(what) {
                    self.bridge_gate("TokenCreated:what", lane);
                }
            }
            EventFilter::Used { of } => {
                self.reference(of, ctx, Kind::Object);
                // The bridge resolves `of` through the watching object —
                // only the self-scoped `This` is resolvable outside a frame
                // ([CR#400.7]).
                if !matches!(deref_reference(of), Reference::This) {
                    self.bridge_gate("Used:of", lane);
                }
            }
            EventFilter::CoinFlipped { by, won } => {
                self.filter(by, ctx, Kind::Player);
                // Flip-WIN is call-relative ([CR#705.2]); the fact records
                // only the physical outcome.
                if won.is_some() {
                    self.bridge_gate("CoinFlipped:won", lane);
                }
            }
            EventFilter::DiceRolled { by } => {
                self.filter(by, ctx, Kind::Player);
            }
            EventFilter::AllOf(events) => {
                self.bridge_gate("AllOf", lane);
                for event in events {
                    self.event_node(event, ctx, lane);
                }
            }
            EventFilter::OneOf(events) => {
                self.bridge_gate("OneOf", lane);
                for event in events {
                    self.event_node(event, ctx, lane);
                }
            }
            EventFilter::Not(inner) => {
                self.bridge_gate("Not", lane);
                // Kind pass rule (a): the operand itself must bottom out in
                // master forms ([CR#603.2]).
                if !anchored(inner) {
                    self.err(
                        Code::CapsAnchor,
                        "Not's operand must bottom out in master forms",
                    );
                }
                self.event_node(inner, ctx, lane);
            }
            EventFilter::OneOrMore(inner) => {
                self.bridge_gate("OneOrMore", lane);
                if !self.tables.lane(lane.key()).one_or_more {
                    self.err(
                        Code::LaneBatch,
                        format!(
                            "OneOrMore (batch matching, [CR#603.2c]) is not admitted \
                             in the {} lane",
                            lane.key()
                        ),
                    );
                }
                self.event_node(inner, ctx, lane);
            }
            EventFilter::Nth { n, of, within: _ } => {
                self.bridge_gate("Nth", lane);
                if !self.tables.lane(lane.key()).nth {
                    self.err(
                        Code::LaneBatch,
                        format!("Nth is not admitted in the {} lane", lane.key()),
                    );
                }
                if *n < 1 {
                    self.err(
                        Code::FloorNth,
                        "Nth is 1-based — a 0th occurrence never occurs ([CR#603.2g])",
                    );
                }
                self.event_node(of, ctx, lane);
            }
            EventFilter::When(inner, condition) => {
                self.bridge_gate("When", lane);
                self.event_node(inner, ctx, lane);
                self.scoped("When", |w| w.condition(condition, ctx));
            }
            EventFilter::Within(inner, _) => {
                self.bridge_gate("Within", lane);
                if !self.tables.lane(lane.key()).within {
                    self.err(
                        Code::LaneWithin,
                        format!(
                            "Within is a history refinement — vacuous, and refused, \
                             in the {} lane",
                            lane.key()
                        ),
                    );
                }
                self.event_node(inner, ctx, lane);
            }
            EventFilter::Expanded(e) => self.event_node(&e.value, ctx, lane),
        }
    }

    /// The residual kind-consistency pass: the set of master-form kinds a
    /// pattern can match (`None` = unconstrained — a bare `Not`), flagging
    /// an `AllOf` whose conjuncts fix incompatible kinds
    /// (`E-CAPS-CONTRADICTION`, rule (c)); `OneOf` is checked per disjunct
    /// (rule (d)) by recursion; `When`/`Nth`/`OneOrMore`/`Within` inherit
    /// the operand's kinds (rule (b)).
    fn event_kinds(&mut self, event: &EventFilter) -> Option<BTreeSet<String>> {
        match event {
            EventFilter::AllOf(events) => {
                let mut acc: Option<BTreeSet<String>> = None;
                for event in events {
                    let Some(kinds) = self.event_kinds(event) else {
                        continue;
                    };
                    acc = Some(match acc {
                        None => kinds,
                        Some(prev) => {
                            let both: BTreeSet<String> =
                                prev.intersection(&kinds).cloned().collect();
                            if both.is_empty() {
                                self.err(
                                    Code::CapsContradiction,
                                    format!(
                                        "AllOf conjoins different master forms \
                                         ({prev:?} vs {kinds:?}) — the pattern can \
                                         never match"
                                    ),
                                );
                                return None;
                            }
                            both
                        }
                    });
                }
                acc
            }
            EventFilter::OneOf(events) => {
                let mut acc: Option<BTreeSet<String>> = None;
                for event in events {
                    if let Some(kinds) = self.event_kinds(event) {
                        acc.get_or_insert_with(BTreeSet::new).extend(kinds);
                    }
                }
                acc
            }
            EventFilter::Not(_) => None,
            EventFilter::OneOrMore(inner)
            | EventFilter::Nth { of: inner, .. }
            | EventFilter::When(inner, _)
            | EventFilter::Within(inner, _) => self.event_kinds(inner),
            EventFilter::Expanded(e) => self.event_kinds(&e.value),
            master => Some(BTreeSet::from([form_key(master)
                .expect("non-algebra variants are master forms")
                .to_owned()])),
        }
    }

    /// The caps an event pattern guarantees its body — table rows keyed by
    /// the master-form name; a cause-narrowed `ZoneChange` inherits its
    /// verb's entailed guarantees ([CR#701] entailment rows); a disjunction
    /// guarantees only the meet ([CR#603.2c]); a conjunction the union with
    /// patient refinement; `When`/`Nth`/`OneOrMore`/`Within` inherit the
    /// operand's caps (kind pass rule (b)); `Not` guarantees nothing.
    fn event_caps(&self, event: &EventFilter) -> Caps {
        match event {
            EventFilter::ZoneChange { cause, .. } => {
                let base = self.tables.event_caps("ZoneChange");
                if let Some(deckmaste_core::Cause::Cause(pattern)) = cause
                    && let Some(verb) = pattern.verb
                    && let Some(row) = self.tables.entailment(verb.as_str())
                    && row.kind == "ZoneChange"
                {
                    base.join(row.caps())
                } else {
                    base
                }
            }
            EventFilter::AllOf(events) => events
                .iter()
                .fold(NO_CAPS, |acc, e| acc.join(self.event_caps(e))),
            EventFilter::OneOf(events) => match events.split_first() {
                None => NO_CAPS,
                Some((first, rest)) => rest.iter().fold(self.event_caps(first), |acc, e| {
                    acc.meet(self.event_caps(e))
                }),
            },
            EventFilter::Not(_) => NO_CAPS,
            EventFilter::OneOrMore(inner)
            | EventFilter::Nth { of: inner, .. }
            | EventFilter::When(inner, _)
            | EventFilter::Within(inner, _) => self.event_caps(inner),
            EventFilter::Expanded(e) => self.event_caps(&e.value),
            master => self
                .tables
                .event_caps(form_key(master).expect("non-algebra variants are master forms")),
        }
    }

    // ------------------------------------------------------------------
    // static effects, replacements, deontics, modifications
    // ------------------------------------------------------------------

    fn static_effect(&mut self, effect: &StaticEffect, ctx: &Ctx) {
        match effect {
            StaticEffect::Modify { of, changes } => self.modify_scope(of, changes, ctx),
            StaticEffect::Deontic(deontic) => self.deontic(deontic, ctx),
            StaticEffect::CostModifier { of, change } => {
                self.filter(of, ctx, Kind::Object);
                self.cost_change(change, ctx);
            }
            StaticEffect::CostOption(optional) => {
                // The declared optional cost's components are cost positions
                // ([CR#118.8b,601.2b]).
                for (i, component) in optional.components.iter().enumerate() {
                    self.scoped(format!("option[{i}]"), |w| w.cost_component(component, ctx));
                }
            }
            StaticEffect::TriggerMultiplier {
                cause,
                extra,
                affected,
            } => {
                self.event(cause, ctx, Lane::TriggerMultiplier);
                self.count(extra, ctx);
                self.filter(affected, ctx, Kind::Object);
            }
            StaticEffect::ModifyPlayer(r, player_mod) => {
                self.reference(r, ctx, Kind::Player);
                match player_mod {
                    PlayerMod::SetTo(_, count)
                    | PlayerMod::Raise(_, count)
                    | PlayerMod::Lower(_, count) => self.count(count, ctx),
                    PlayerMod::NoMax(_) => {}
                }
            }
            StaticEffect::Replacement(replacement) => self.replacement(replacement, ctx),
            StaticEffect::Prevention(prevention) => match &**prevention {
                Prevention::PreventNext {
                    n,
                    from,
                    to,
                    duration,
                } => {
                    self.count(n, ctx);
                    self.filter(from, ctx, Kind::Object);
                    self.filter(to, ctx, Kind::Any);
                    if let Some(duration) = duration {
                        self.duration(duration, ctx);
                    }
                }
                Prevention::PreventNextInstance { from, to } => {
                    self.filter(from, ctx, Kind::Object);
                    self.filter(to, ctx, Kind::Any);
                }
                Prevention::PreventAll { from, to, duration } => {
                    self.filter(from, ctx, Kind::Object);
                    self.filter(to, ctx, Kind::Any);
                    if let Some(duration) = duration {
                        self.duration(duration, ctx);
                    }
                }
            },
            StaticEffect::AsThough(as_though) => match as_though {
                AsThough::Expanded(_) => {}
            },
            // [CR#615.12]: the gate on the Prevention class — both slots
            // judge objects (a damage source / recipient may also be a
            // player, hence `Any` on `to`).
            StaticEffect::CantPrevent { from, to } => {
                self.filter(from, ctx, Kind::Object);
                self.filter(to, ctx, Kind::Any);
            }
            // [CR#609.4b] payment freedom: `mana_from` judges the mana's
            // PRODUCER (an object); the `as_` reading carries no bindings.
            StaticEffect::SpendAsThough { mana_from, as_: _ } => {
                self.filter(mana_from, ctx, Kind::Object);
            }
            StaticEffect::Sba { when, then } => {
                self.condition(when, ctx);
                let body = self.descend(ctx, "Sba", None, None, NO_CAPS, vec![]);
                self.scoped("Sba", |w| {
                    w.effect(then, &body);
                });
            }
            StaticEffect::OutcomeGate { who, gate: _ } => {
                self.filter(who, ctx, Kind::Player);
            }
            StaticEffect::CantHappen(event) => self.event(event, ctx, Lane::CantHappen),
            StaticEffect::PayPips(_, act) => match act {
                deckmaste_core::PayAct::TapToPay(filter)
                | deckmaste_core::PayAct::ExileToPay(filter) => {
                    self.filter(filter, ctx, Kind::Object);
                }
            },
            StaticEffect::Expanded(e) => self.static_effect(&e.value, ctx),
        }
    }

    /// A `Modify`'s scope + changes: the per-subject candidate is readable
    /// as `It` inside the changes (an anthem's Coat-of-Arms shape).
    fn modify_scope(&mut self, of: &Scope, changes: &[Modification], ctx: &Ctx) {
        match of {
            Scope::Of(r) => {
                self.reference(r, ctx, Kind::Object);
            }
            Scope::These(refs) => {
                for r in refs {
                    self.reference(r, ctx, Kind::Object);
                }
            }
            Scope::Matching(filter) => {
                self.filter(filter, ctx, Kind::Object);
            }
        }
        let subject = Ante {
            sort: Sort::Permanent,
            kind: Kind::Object,
            card: Cardinality::One,
            site: Site::Loop,
            expected_zone: None,
            label: None,
            binder: false,
        };
        let body = self.descend(ctx, "Where", None, Some(subject), NO_CAPS, vec![]);
        for (i, change) in changes.iter().enumerate() {
            self.scoped(format!("changes[{i}]"), |w| w.modification(change, &body));
        }
    }

    fn modification(&mut self, modification: &Modification, ctx: &Ctx) {
        match modification {
            Modification::Power(op)
            | Modification::Toughness(op)
            | Modification::BaseLoyalty(op)
            | Modification::BaseDefense(op) => match op {
                deckmaste_core::NumericOp::Set(count)
                | deckmaste_core::NumericOp::Up(count)
                | deckmaste_core::NumericOp::Down(count) => self.count(count, ctx),
            },
            Modification::SwitchPowerToughness
            | Modification::Colors(_)
            | Modification::CardTypes(_)
            | Modification::Subtypes(_)
            | Modification::Supertypes(_)
            | Modification::LoseAbility(_)
            | Modification::LoseAllAbilities
            | Modification::CantHaveAbility(_)
            | Modification::SetText(_)
            | Modification::AllCreatureTypes
            | Modification::BecomeBasicLandType(_) => {}
            Modification::GainAbility(ability) => {
                // A granted ability is a rule of its new bearer: it
                // elaborates as a fresh root (`This` rebinds).
                self.scoped("GainAbility", |w| w.ability(ability, false));
            }
            Modification::SetController(r) => {
                self.reference(r, ctx, Kind::Player);
            }
            Modification::Several(members) => {
                for member in members {
                    self.modification(member, ctx);
                }
            }
            Modification::Expanded(e) => self.modification(&e.value, ctx),
        }
    }

    fn cost_change(&mut self, change: &CostChange, ctx: &Ctx) {
        match change {
            CostChange::Increase(components)
            | CostChange::Reduce(components)
            | CostChange::Additional { components, .. } => {
                for (i, component) in components.iter().enumerate() {
                    self.scoped(format!("change[{i}]"), |w| w.cost_component(component, ctx));
                }
            }
            CostChange::Scaled { change, times } => {
                self.cost_change(change, ctx);
                self.count(times, ctx);
            }
        }
    }

    fn replacement(&mut self, replacement: &Replacement, ctx: &Ctx) {
        match replacement {
            Replacement::Instead { would, instead } => {
                self.scoped("Instead", |w| {
                    // [CR#615.1,615.1a]: replacing a damage event with
                    // NOTHING is a prevention effect in disguise — it must
                    // be spelled through the marked `Prevention` class, or
                    // the `CantPrevent` gate ([CR#615.12]) can't see it.
                    if is_damage_form(would) && is_noop_effect(instead) {
                        w.err(
                            Code::PosPrevention,
                            "a damage event replaced by nothing is a prevention effect - spell it Prevention(...), not a generic Instead",
                        );
                    }
                    w.event(would, ctx, Lane::Replacement);
                    let caps = w.event_caps(would);
                    let roles = w.event_role_antes(would);
                    let body =
                        w.descend(ctx, "Replacement.Instead", None, None, caps, roles);
                    w.effect(instead, &body);
                });
            }
            Replacement::Skip { what: _ } => {}
            Replacement::Also { would, also } => {
                self.scoped("Also", |w| {
                    w.event(would, ctx, Lane::Replacement);
                    let caps = w.event_caps(would);
                    let roles = w.event_role_antes(would);
                    let body = w.descend(ctx, "Replacement.Also", None, None, caps, roles);
                    w.effect(also, &body);
                });
            }
            Replacement::Expanded(e) => self.replacement(&e.value, ctx),
        }
    }

    fn deontic(&mut self, deontic: &Deontic, ctx: &Ctx) {
        match deontic {
            Deontic::May(action) | Deontic::Cant(action) | Deontic::Must(action) => {
                self.deontic_action(action, ctx);
            }
            Deontic::Gate(action, components) => {
                self.deontic_action(action, ctx);
                for (i, component) in components.iter().enumerate() {
                    self.scoped(format!("gate[{i}]"), |w| w.cost_component(component, ctx));
                }
            }
            Deontic::Expanded(e) => self.deontic(&e.value, ctx),
        }
    }

    /// A deed's agent slot is kind-gated by the relation (the Idris
    /// `agentScope` rows); its PATIENT slot by the relation's emitted
    /// `patientScope` row ([CR#508.1b,509.1a,115.4]).
    fn deontic_action(&mut self, action: &DeonticAction, ctx: &Ctx) {
        let agent = |w: &mut Self, relation: &str, by: &Filter, ctx: &Ctx| {
            let expected = w.tables.agent_scope(relation).unwrap_or(Kind::Any);
            w.filter(by, ctx, expected);
        };
        match action {
            DeonticAction::Attack { by, on } => {
                agent(self, "Attack", by, ctx);
                self.deed_patient("Attack", on, ctx);
            }
            DeonticAction::Block { by, on, count } => {
                agent(self, "Block", by, ctx);
                self.deed_patient("Block", on, ctx);
                if let Some(bound) = count {
                    self.count_bound(bound, ctx);
                    self.block_qty_floor(bound);
                }
            }
            DeonticAction::Target { by, on } => {
                self.deed_agent("Target", by, ctx);
                self.deed_patient("Target", on, ctx);
            }
            DeonticAction::Attach { what, to } => {
                agent(self, "Attach", what, ctx);
                self.deed_patient("Attach", to, ctx);
            }
            DeonticAction::Cast { what, by, cost, .. } => {
                self.deed_patient("Cast", what, ctx);
                agent(self, "Cast", by, ctx);
                if let Some(AlternativeCost::Components(components)) = cost {
                    for (i, component) in components.iter().enumerate() {
                        self.scoped(format!("cost[{i}]"), |w| w.cost_component(component, ctx));
                    }
                }
            }
            DeonticAction::Play { what, by, .. } => {
                self.deed_patient("Play", what, ctx);
                agent(self, "Play", by, ctx);
            }
            DeonticAction::Activate { what, by } => {
                self.deed_patient("Activate", what, ctx);
                agent(self, "Activate", by, ctx);
            }
            DeonticAction::Expanded(e) => self.deontic_action(&e.value, ctx),
        }
    }

    /// The two-slot deed agent ([CR#702.11d,702.16b]): at least one arm must
    /// be present, the `stack_object` arm is kind-gated by the relation's
    /// `agentScope` row, and the `source` arm judges the acting object's
    /// SOURCE — always an object ([CR#113.7,609.7a]).
    fn deed_agent(&mut self, relation: &str, agent: &DeedAgent, ctx: &Ctx) {
        if agent.is_empty() {
            self.err(
                Code::FloorDeedAgent,
                format!(
                    "the {relation} deed's agent has neither arm — a DeedAgent constrains through stack_object and/or source ([CR#702.11d,702.16b])"
                ),
            );
        }
        if let Some(f) = &agent.stack_object {
            let expected = self.tables.agent_scope(relation).unwrap_or(Kind::Any);
            self.scoped("stack_object", |w| {
                w.filter(f, ctx, expected);
            });
        }
        if let Some(f) = &agent.source {
            self.scoped("source", |w| {
                w.filter(f, ctx, Kind::Object);
            });
        }
    }

    /// A deed's PATIENT slot, kind-gated per relation by the emitted
    /// `patientScope` row: a blocked thing is an attacking creature
    /// ([CR#509.1a]), an attacked thing a player, planeswalker, or battle
    /// ([CR#508.1b]), a target anything targetable ([CR#115.4]).
    fn deed_patient(&mut self, relation: &str, on: &Filter, ctx: &Ctx) {
        let expected = self.tables.patient_scope(relation).unwrap_or(Kind::Any);
        let kind = self.filter_kind(on, ctx);
        if kind.compatible_with(expected) {
            self.resolve(|| format!("{relation} patient -> {expected:?}-scoped"));
        } else {
            self.err(
                Code::KindPatientScope,
                format!(
                    "the {relation} deed acts on a {expected:?}-kinded patient, but the filter is {kind:?}-kinded"
                ),
            );
        }
    }

    fn duration(&mut self, duration: &Duration, ctx: &Ctx) {
        match duration {
            Duration::FixedUntil(_) | Duration::ForThisEvent | Duration::EndOfGame => {}
            Duration::UntilEvent(event) => self.event(event, ctx, Lane::UntilEvent),
            Duration::ForAsLongAs(condition) => self.condition(condition, ctx),
        }
    }
}

/// Conjunctive pins a filter fixes, for SORT derivation: a definite player
/// atom, a card-type atom, a zone atom, an object-kind atom. `AllOf` merges
/// (first pin wins); `OneOf` pins only what EVERY disjunct pins; `Not` pins
/// nothing.
#[derive(Debug, Default, Clone, Copy, PartialEq, Eq)]
struct FilterPins {
    player: bool,
    card_type: Option<Type>,
    zone: Option<deckmaste_core::Zone>,
    object_kind: Option<deckmaste_core::ObjectKind>,
}

fn filter_pins(filter: &Filter) -> FilterPins {
    match filter {
        Filter::Kind(deckmaste_core::ObjectKind::Player) => FilterPins {
            player: true,
            ..FilterPins::default()
        },
        Filter::Kind(k) => FilterPins {
            object_kind: Some(*k),
            ..FilterPins::default()
        },
        Filter::Characteristic(CharacteristicFilter::Type(t)) => FilterPins {
            card_type: Some(*t),
            ..FilterPins::default()
        },
        Filter::State(StateFilter::InZone(z)) => FilterPins {
            zone: Some(*z),
            ..FilterPins::default()
        },
        Filter::AllOf(members) => {
            let mut pins = FilterPins::default();
            for member in members {
                let m = filter_pins(member);
                pins.player |= m.player;
                pins.card_type = pins.card_type.or(m.card_type);
                pins.zone = pins.zone.or(m.zone);
                pins.object_kind = pins.object_kind.or(m.object_kind);
            }
            pins
        }
        Filter::OneOf(members) => {
            let all: Vec<FilterPins> = members.iter().map(filter_pins).collect();
            match all.split_first() {
                Some((first, rest)) if rest.iter().all(|p| p == first) => *first,
                _ => FilterPins::default(),
            }
        }
        Filter::Expanded(e) => filter_pins(&e.value),
        _ => FilterPins::default(),
    }
}

/// The static-classes table key of a static part ([CR#611.2c]) — its
/// variant name, looked through remembered macros.
fn static_effect_key(effect: &StaticEffect) -> &'static str {
    match effect {
        StaticEffect::Modify { .. } => "Modify",
        StaticEffect::ModifyPlayer(..) => "ModifyPlayer",
        StaticEffect::Deontic(_) => "Deontic",
        StaticEffect::CostModifier { .. } => "CostModifier",
        StaticEffect::CostOption(_) => "CostOption",
        StaticEffect::TriggerMultiplier { .. } => "TriggerMultiplier",
        StaticEffect::Replacement(_) => "Replacement",
        StaticEffect::Prevention(_) => "Prevention",
        StaticEffect::CantPrevent { .. } => "CantPrevent",
        StaticEffect::SpendAsThough { .. } => "SpendAsThough",
        StaticEffect::Sba { .. } => "Sba",
        StaticEffect::OutcomeGate { .. } => "OutcomeGate",
        StaticEffect::CantHappen(_) => "CantHappen",
        StaticEffect::PayPips(..) => "PayPips",
        StaticEffect::AsThough(_) => "AsThough",
        StaticEffect::Expanded(e) => static_effect_key(&e.value),
    }
}

/// The zone a `Move`/`MoveGroup` destination lands in.
fn destination_zone(destination: &Destination) -> deckmaste_core::Zone {
    match destination {
        Destination::Zone(z) => *z,
        Destination::Library(_) => deckmaste_core::Zone::Library,
    }
}

/// The kind-directed fallback sort for a bound reference: a player is
/// `Player`, anything else the battlefield default `Permanent`.
fn kind_sort(kind: Kind) -> Sort {
    match kind {
        Kind::Player => Sort::Player,
        _ => Sort::Permanent,
    }
}

/// Whether a quantity is literally "exactly one" — the One/Many
/// cardinality derivation for announced slots ([CR#115.3]).
/// The `As` name on an announce slot, seen through `Distinct` and macro
/// provenance ([CR#608.2d] labeled antecedents).
fn slot_label(spec: &TargetSpec) -> Option<Ident> {
    match spec {
        TargetSpec::As(label, _) => Some(*label),
        TargetSpec::Distinct(_, inner) => slot_label(inner),
        TargetSpec::Expanded(e) => slot_label(&e.value),
        TargetSpec::Target(..) => None,
    }
}

fn quantity_is_one(quantity: &Quantity) -> bool {
    let (lo, hi) = quantity.bounds();
    lo.and_then(Count::literal_value) == Some(1) && hi.and_then(Count::literal_value) == Some(1)
}

/// The named participant slot of a master form (the `object_sort` column's
/// slot tags) — tree shape; the tag itself is emitted data.
fn event_slot_filter<'e>(event: &'e EventFilter, slot: &str) -> Option<&'e Filter> {
    match (event, slot) {
        (EventFilter::Damage { source, .. }, "source") => Some(source),
        (
            EventFilter::Played { what, .. }
            | EventFilter::Attached { what, .. }
            | EventFilter::BecomesTarget { what, .. },
            "what",
        ) => Some(what),
        (EventFilter::AttackDeclared { by, .. } | EventFilter::BlockDeclared { by, .. }, "by") => {
            Some(by)
        }
        (EventFilter::StateBecame { of, .. } | EventFilter::ControlChanged { of, .. }, "of") => {
            Some(of)
        }
        (EventFilter::CounterPlaced { on, .. } | EventFilter::CounterRemoved { on, .. }, "on") => {
            Some(on)
        }
        _ => None,
    }
}

/// Whether an effect does NOTHING — the empty `Sequence`, the spelling a
/// prevention-in-disguise `Instead` would use for "instead, no damage".
fn is_noop_effect(effect: &Effect) -> bool {
    matches!(effect, Effect::Sequence(seq) if seq.is_empty())
}

/// Whether a replacement's `would` pattern bottoms out in the `Damage`
/// master form ([CR#615.1] prevention effects "watch for a damage event"):
/// refinement wrappers inherit, a conjunction is damage-anchored if ANY
/// member is, a disjunction if EVERY member is, `Not` never.
fn is_damage_form(event: &EventFilter) -> bool {
    match event {
        EventFilter::AllOf(events) => events.iter().any(is_damage_form),
        EventFilter::OneOf(events) => events.iter().all(is_damage_form),
        EventFilter::Not(_) => false,
        EventFilter::OneOrMore(inner)
        | EventFilter::Nth { of: inner, .. }
        | EventFilter::When(inner, _)
        | EventFilter::Within(inner, _) => is_damage_form(inner),
        EventFilter::Expanded(e) => is_damage_form(&e.value),
        master => form_key(master) == Some("Damage"),
    }
}

/// The carrier scope a loaded designation declaration imposes: a `Stored`
/// decl's scope column ([CR#109.3]; game-scoped designations constrain no
/// carrier); a derived designation re-evaluates an object predicate.
fn designation_decl_scope(decl: &DesignationDecl) -> Option<Kind> {
    match &decl.definition {
        DesignationDef::Stored { scope, .. } => match scope {
            DesignationScope::Object => Some(Kind::Object),
            DesignationScope::Player => Some(Kind::Player),
            DesignationScope::Game => None,
        },
        DesignationDef::Derived(_) | DesignationDef::DerivedIf(_) => Some(Kind::Object),
    }
}

/// The declared domain of `key` in the running context, if any — the most
/// recent noting wins ([CR#607.2]).
fn noted_kind(ctx: &Ctx, key: &Ident) -> Option<NotedKind> {
    ctx.notes
        .iter()
        .rev()
        .find(|(k, _)| k == key)
        .map(|&(_, kind)| kind)
}

/// The table key of a player verb — its variant name, looked through any
/// remembered macro invocation (a keyword composite's spliced `Do` stays
/// checked).
fn player_action_key(action: &PlayerAction) -> &'static str {
    match action {
        PlayerAction::Draw(_) => "Draw",
        PlayerAction::Discard { .. } => "Discard",
        PlayerAction::GainLife(_) => "GainLife",
        PlayerAction::LoseLife(_) => "LoseLife",
        PlayerAction::AddMana(..) => "AddMana",
        PlayerAction::Create(..) => "Create",
        PlayerAction::Sacrifice(_) => "Sacrifice",
        PlayerAction::Move(..) => "Move",
        PlayerAction::Tap(_) => "Tap",
        PlayerAction::Untap(_) => "Untap",
        PlayerAction::GetEmblem(_) => "GetEmblem",
        PlayerAction::GetDesignation(_) => "GetDesignation",
        PlayerAction::ChooseAndNote(..) => "ChooseAndNote",
        PlayerAction::CopySpell(_) => "CopySpell",
        PlayerAction::FlipCoins(_) => "FlipCoins",
        PlayerAction::RollDice(..) => "RollDice",
        PlayerAction::PutCounters(..) => "PutCounters",
        PlayerAction::RemoveCounters(..) => "RemoveCounters",
        PlayerAction::Distribute { .. } => "Distribute",
        PlayerAction::WinGame => "WinGame",
        PlayerAction::LoseGame => "LoseGame",
        PlayerAction::RestartGame => "RestartGame",
        PlayerAction::Shuffle => "Shuffle",
        PlayerAction::SetLife(_) => "SetLife",
        PlayerAction::Reveal { .. } => "Reveal",
        PlayerAction::RemoveDamage(_) => "RemoveDamage",
        PlayerAction::Mill(_) => "Mill",
        PlayerAction::VentureIntoDungeon => "VentureIntoDungeon",
        PlayerAction::Expanded(e) => player_action_key(&e.value),
    }
}

/// Whether a cost declares `{X}` ([CR#107.3]) — a `Mana` component carrying
/// the `Variable` symbol, through nested/spliced cost lists.
fn cost_declares_x(cost: &Cost) -> bool {
    fn component(c: &CostComponent) -> bool {
        match c {
            CostComponent::Mana(mana) => mana.iter().any(|s| matches!(s, ManaSymbol::Variable)),
            CostComponent::Cost(inner) => cost_declares_x(inner),
            CostComponent::With { body, .. } => cost_declares_x(body),
            CostComponent::Expanded(e) => component(&e.value),
            CostComponent::ManaCostOf(_)
            | CostComponent::Tap
            | CostComponent::Untap
            | CostComponent::Do(_)
            | CostComponent::TapTotal { .. } => false,
        }
    }
    cost.iter().any(component)
}
