//! The elaborator's tree walk: one pass over a card/token with a running
//! binding context ([`Ctx`]), pushing an [`ElabError`] for every rule
//! violation. Context transitions are applied through the emitted bind-rule
//! rows ([`tables::BindRule`]); event/cost capabilities, carrier scopes, and
//! the kind lattice come from the emitted tables too — the walker contributes
//! tree shape, never rule values.

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
use deckmaste_core::CounterSpec;
use deckmaste_core::Deontic;
use deckmaste_core::DeonticAction;
use deckmaste_core::Destination;
use deckmaste_core::Duration;
use deckmaste_core::Effect;
use deckmaste_core::Event;
use deckmaste_core::Filter;
use deckmaste_core::Ident;
use deckmaste_core::KeywordAbility;
use deckmaste_core::ManaProduction;
use deckmaste_core::ManaRider;
use deckmaste_core::ManaSymbol;
use deckmaste_core::Modification;
use deckmaste_core::Normalize;
use deckmaste_core::NotedKind;
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
use deckmaste_core::StateFilter;
use deckmaste_core::StateFilterEvent;
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

/// The running binding context — the antecedent state a position sees
/// (the Idris `Endophora`, plus the positional discipline the plan's `Ctx`
/// adds: `may_target`, `has_x`, the amount antecedent, noted keys).
#[expect(
    clippy::struct_excessive_bools,
    reason = "each flag is an independent binding-context dimension, not a state machine"
)]
#[derive(Debug, Clone)]
struct Ctx {
    /// One kind per announced target slot ([CR#115.3,601.2c]).
    targets: Vec<Kind>,
    /// The enclosing `With` binding ([CR#608.2d]).
    that: Option<(Cardinality, Kind)>,
    /// The enclosing loop/candidate element ([CR#608.2]).
    it: Option<Kind>,
    /// A `DivideAmong` share in scope ([CR#601.2d]).
    allotment: bool,
    /// The surrounding event's caps; [`NO_CAPS`] outside an event body.
    caps: Caps,
    /// Whether ANY event body encloses this position — splits `E-BIND-EVENT`
    /// (no event at all) from `E-CAPS-*` (an event that doesn't supply the
    /// role).
    in_event: bool,
    /// An amount antecedent is in scope ("that much", [CR#107.3]): the
    /// event's amount, or an amount-guaranteeing earlier instruction.
    amount: bool,
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
            targets: Vec::new(),
            that: None,
            it: None,
            allotment: false,
            caps: NO_CAPS,
            in_event: false,
            amount: false,
            has_x,
            may_target: false,
            notes: Vec::new(),
        }
    }
}

/// What a walked effect introduces for its FOLLOWING siblings (the
/// `Sequence` telescope): an amount antecedent, and noted keys.
#[derive(Debug, Default)]
struct Intro {
    amount: bool,
    notes: Vec<(Ident, NotedKind)>,
}

impl Intro {
    fn absorb(&mut self, other: Intro) {
        self.amount |= other.amount;
        self.notes.extend(other.notes);
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
    /// Whether to collect [`super::Resolution`]s (`--dump`) — the load
    /// gate's plain walk leaves this off so it never pays for the
    /// bookkeeping.
    trace: bool,
    resolutions: Vec<super::Resolution>,
}

impl<'a> Walker<'a> {
    fn new(registries: &'a Registries<'a>, trace: bool) -> Walker<'a> {
        Walker {
            tables: tables::tables(),
            registries,
            errors: Vec::new(),
            path: Vec::new(),
            trace,
            resolutions: Vec::new(),
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

    /// Applies a construct's emitted bind-rule row to the context. `targets`
    /// replaces the slot list when the row binds targets; `that_kind` /
    /// `it_kind` supply the bound kinds; `caps` is consumed per the row's
    /// `caps_from`.
    fn descend(
        &self,
        ctx: &Ctx,
        construct: &str,
        targets: Option<Vec<Kind>>,
        bind_kind: Kind,
        caps: Caps,
    ) -> Ctx {
        let rule = self.tables.bind_rule(construct);
        let mut next = ctx.clone();
        if rule.drops_targets {
            next.targets.clear();
        }
        if rule.binds_targets
            && let Some(slots) = targets
        {
            next.targets = slots;
        }
        if !rule.keeps_that {
            next.that = None;
        }
        if let Some(card) = rule.binds_that {
            next.that = Some((card, bind_kind));
        }
        if rule.binds_it {
            next.it = Some(bind_kind);
        }
        if rule.binds_allotment {
            next.allotment = true;
        } else if rule.clears_allotment {
            next.allotment = false;
        }
        match rule.caps_from {
            CapsFrom::Keep => {}
            CapsFrom::Query | CapsFrom::Cost => {
                next.caps = caps;
                next.in_event = true;
                next.amount = caps.amount;
            }
        }
        if let Some(may_target) = rule.may_target {
            next.may_target = may_target;
        }
        next
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
                let inner = self.descend(&ctx, "Sba", None, Kind::Any, NO_CAPS);
                self.effect(effect, &inner);
            }
            Property::TurnBased { at: _, effect } => {
                // A turn-based action never targets ([CR#703]); the Sba row
                // carries the same never-a-targeting-position discipline.
                let inner = self.descend(&ctx, "Sba", None, Kind::Any, NO_CAPS);
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
        match keyword {
            KeywordAbility::FirstStrike
            | KeywordAbility::DoubleStrike
            | KeywordAbility::Deathtouch
            | KeywordAbility::Trample
            | KeywordAbility::Vigilance => {}
            KeywordAbility::Composite { abilities, .. } => self.abilities(abilities, face_x),
            KeywordAbility::Expanded(e) => self.keyword(&e.value, face_x),
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
        let pattern_ctx = self.descend(ctx, construct, None, Kind::Any, NO_CAPS);
        self.scoped("event", |w| w.event(&triggered.event, &pattern_ctx));
        let caps = self.event_caps(&triggered.event);
        let body = self.descend(ctx, construct, None, Kind::Any, caps);
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

    #[expect(
        clippy::too_many_lines,
        reason = "one arm per Effect variant; the flat dispatch reads better whole"
    )]
    fn effect(&mut self, effect: &Effect, ctx: &Ctx) -> Intro {
        match effect {
            Effect::Act(action) => self.action(action, ctx),
            Effect::Sequence(effects) => {
                let mut running = ctx.clone();
                let mut intro = Intro::default();
                for (i, element) in effects.iter().enumerate() {
                    let mut introduced = Intro::default();
                    self.scoped(format!("Sequence[{i}]"), |w| {
                        introduced = w.effect(element, &running);
                    });
                    running.amount |= introduced.amount;
                    running.notes.extend(introduced.notes.iter().copied());
                    intro.absorb(introduced);
                }
                intro
            }
            Effect::Continuously(continuously) => {
                self.scoped("Continuously", |w| {
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
                    did_ctx.amount |= intro.amount;
                    did_ctx.notes.extend(intro.notes.iter().copied());
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
                // A declined May guarantees nothing to later siblings.
                Intro::default()
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
                    w.effect(&may_pay.and_then, ctx);
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
                    let body = w.descend(ctx, "AdditionalCost", None, Kind::Any, caps);
                    w.effect(&additional.body, &body);
                });
                Intro::default()
            }
            Effect::Each(each) => {
                self.scoped("Each", |w| {
                    let (_, kind) = w.binder(&each.binder, ctx);
                    let body = w.descend(ctx, "Each", None, kind, NO_CAPS);
                    w.effect(&each.effect, &body);
                });
                Intro::default()
            }
            Effect::With(with) => {
                let mut intro = Intro::default();
                self.scoped("With", |w| {
                    let (cardinality, kind) = w.binder(&with.binder, ctx);
                    let construct = match cardinality {
                        Cardinality::One => "With.One",
                        Cardinality::Many => "With.Many",
                    };
                    let body = w.descend(ctx, construct, None, kind, NO_CAPS);
                    intro = w.effect(&with.body, &body);
                });
                intro
            }
            Effect::DivideAmong(divide) => {
                self.scoped("DivideAmong", |w| {
                    w.count(&divide.amount, ctx);
                    let (_, kind) = w.binder(&divide.binder, ctx);
                    w.divide_floor(&divide.amount, &divide.binder, ctx);
                    let body = w.descend(ctx, "DivideAmong", None, kind, NO_CAPS);
                    w.effect(&divide.body, &body);
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
            Effect::Expanded(e) => self.effect(&e.value, ctx),
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
            for (i, spec) in targeted.targets.iter().enumerate() {
                let mut kind = Kind::Any;
                w.scoped(format!("targets[{i}]"), |w| {
                    kind = w.target_spec(spec, ctx, targeted.targets.len());
                });
                slots.push(kind);
            }
            let body = w.descend(ctx, "Targeted", Some(slots), Kind::Any, NO_CAPS);
            intro = w.effect(&targeted.effect, &body);
        });
        intro
    }

    fn target_spec(&mut self, spec: &TargetSpec, ctx: &Ctx, sibling_count: usize) -> Kind {
        match spec {
            TargetSpec::Target(quantity, filter) => {
                self.quantity(quantity, ctx, true);
                self.filter(filter, ctx, Kind::Any)
            }
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
            w.count(&modal.choose.count, ctx);
            if !modal.choose.repeats
                && !modal.choose.up_to
                && let Some(count) = modal.choose.count.literal_value()
                && count as usize > modal.modes.len()
            {
                w.err(
                    Code::FloorModalCount,
                    format!(
                        "choose {count} of {} modes ([CR#700.2d])",
                        modal.modes.len()
                    ),
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
                Intro {
                    amount: self.tables.event_caps("Performed:DealDamage").amount,
                    notes: Vec::new(),
                }
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
            Action::Move(r, destination) => {
                self.reference(r, ctx, Kind::Object);
                self.destination(destination, ctx);
                Intro::default()
            }
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
            PlayerAction::Draw(count)
            | PlayerAction::SetLife(count)
            | PlayerAction::FlipCoins(count)
            | PlayerAction::RollDice(count, _) => {
                self.count(count, ctx);
                Intro::default()
            }
            PlayerAction::GainLife(count) => {
                self.count(count, ctx);
                Intro {
                    amount: self.tables.event_caps("Performed:GainLife").amount,
                    notes: Vec::new(),
                }
            }
            PlayerAction::LoseLife(count) => {
                self.count(count, ctx);
                Intro {
                    amount: self.tables.event_caps("Performed:LoseLife").amount,
                    notes: Vec::new(),
                }
            }
            PlayerAction::Discard { count, what } => {
                self.count(count, ctx);
                if let Some(what) = what {
                    self.reference(what, ctx, Kind::Object);
                }
                Intro::default()
            }
            PlayerAction::AddMana(count, production) => {
                self.count(count, ctx);
                self.mana_production(production, ctx);
                Intro::default()
            }
            PlayerAction::Create(count, spec) => {
                self.count(count, ctx);
                self.token_spec(spec);
                Intro::default()
            }
            PlayerAction::Sacrifice(r)
            | PlayerAction::Tap(r)
            | PlayerAction::Untap(r)
            | PlayerAction::CopySpell(r)
            | PlayerAction::RemoveDamage(r) => {
                self.reference(r, ctx, Kind::Object);
                Intro::default()
            }
            PlayerAction::Move(r, destination) => {
                self.reference(r, ctx, Kind::Object);
                self.destination(destination, ctx);
                Intro::default()
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
                amount: false,
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
            | PlayerAction::Shuffle => Intro::default(),
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
            Destination::Zone(_) => {}
            Destination::Library(Anchor::FromTop(count) | Anchor::FromBottom(count)) => {
                self.count(count, ctx);
            }
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
                let body = self.descend(ctx, "Each", None, Kind::Object, NO_CAPS);
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
                let inner = self.descend(ctx, construct, None, kind, NO_CAPS);
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
                filter, by, whose, ..
            } => {
                self.reference(by, ctx, Kind::Player);
                self.reference(whose, ctx, Kind::Player);
                let kind = self.filter(filter, ctx, Kind::Object);
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
                ..
            } => {
                self.quantity(quantity, ctx, false);
                self.reference(by, ctx, Kind::Player);
                self.reference(whose, ctx, Kind::Player);
                let kind = self.filter(filter, ctx, Kind::Object);
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
            Selection::That => match ctx.that {
                Some((Cardinality::Many, kind)) => {
                    self.resolve(|| {
                        format!("group That -> {kind:?} (the enclosing With many-binder)")
                    });
                    kind
                }
                Some((Cardinality::One, _)) => {
                    self.err(
                        Code::BindThatGroup,
                        "group That read where the enclosing With binds a single object \
                         (read it as the singular That)",
                    );
                    Kind::Any
                }
                None => {
                    self.err(
                        Code::BindThatGroup,
                        "group That read outside an enclosing With many-binder",
                    );
                    Kind::Any
                }
            },
            Selection::GetTargets(n) => {
                if *n >= ctx.targets.len() {
                    self.err(
                        Code::BindTarget,
                        format!(
                            "GetTargets({n}) but {} target spec(s) are announced in scope",
                            ctx.targets.len()
                        ),
                    );
                    Kind::Any
                } else {
                    self.resolve(|| {
                        format!(
                            "GetTargets({n}) -> {:?} (target spec {n} of {} announced)",
                            ctx.targets[*n],
                            ctx.targets.len()
                        )
                    });
                    ctx.targets[*n]
                }
            }
            Selection::Pick { op: _, of, by } => {
                self.filter(of, ctx, Kind::Object);
                let body = self.descend(ctx, "Pick", None, Kind::Object, NO_CAPS);
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

    fn reference(&mut self, reference: &Reference, ctx: &Ctx, expected: Kind) -> Kind {
        let kind = match reference {
            Reference::This => Kind::Object,
            Reference::You | Reference::Opponent => Kind::Player,
            Reference::It => {
                if let Some(kind) = ctx.it {
                    self.resolve(|| {
                        format!("It -> {kind:?} (the enclosing loop/candidate binder)")
                    });
                    kind
                } else {
                    self.err(
                        Code::BindIt,
                        "It read outside an Each/DivideAmong/Where/Pick binder",
                    );
                    Kind::Any
                }
            }
            Reference::Target(n) => {
                if *n >= ctx.targets.len() {
                    self.err(
                        Code::BindTarget,
                        format!(
                            "Target({n}) but {} target spec(s) are announced in scope",
                            ctx.targets.len()
                        ),
                    );
                    Kind::Any
                } else {
                    self.resolve(|| {
                        format!(
                            "Target({n}) -> {:?} (target spec {n} of {} announced)",
                            ctx.targets[*n],
                            ctx.targets.len()
                        )
                    });
                    ctx.targets[*n]
                }
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
            Reference::That => match ctx.that {
                Some((Cardinality::One, kind)) => {
                    self.resolve(|| format!("That -> {kind:?} (the enclosing With one-binder)"));
                    kind
                }
                Some((Cardinality::Many, _)) => {
                    self.err(
                        Code::BindThat,
                        "singular That read where the enclosing With binds a group \
                         (iterate it with Each)",
                    );
                    Kind::Any
                }
                None => {
                    self.err(
                        Code::BindThat,
                        "singular That read outside an enclosing With one-binder",
                    );
                    Kind::Any
                }
            },
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
            Filter::Where(condition) => {
                // The candidate binds as `It` for the condition ([CR#603.4]).
                let body = self.descend(ctx, "Where", None, Kind::Any, NO_CAPS);
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
            | StateFilter::Unblocked => Kind::Object,
            StateFilter::HasCounter(counter) => {
                // The atom's kind IS the counter's carrier scope.
                self.counter_declared(counter);
                self.tables.counter_scope(counter.as_str())
            }
            StateFilter::Designated(name) => self
                .tables
                .designation_scope(name.as_str())
                .unwrap_or(Kind::Any),
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

    fn counter_ref(&mut self, counter: &CounterRef, carrier: Kind) {
        self.counter_declared(counter);
        let scope = self.tables.counter_scope(counter.as_str());
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

    fn designation(&mut self, name: &Ident, carrier: Kind) {
        if let Some(scope) = self.tables.designation_scope(name.as_str()) {
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
            Count::ThatMuch => {
                if ctx.amount {
                    self.resolve(|| "ThatMuch -> bound to the in-scope amount antecedent".into());
                } else {
                    self.err(
                        Code::CapsAmount,
                        "ThatMuch read with no amount antecedent in scope",
                    );
                }
            }
            Count::Allotment => {
                if ctx.allotment {
                    self.resolve(|| "Allotment -> bound to the enclosing DivideAmong share".into());
                } else {
                    self.err(
                        Code::BindAllotment,
                        "Allotment read outside a DivideAmong body",
                    );
                }
            }
            Count::EventCount(event, _) => self.event(event, ctx),
            Count::EventSum(event, _) => {
                self.event(event, ctx);
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
            Count::Literal(_) => {}
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
            Condition::Happened { event, within: _ } => self.event(event, ctx),
            Condition::YourTurn | Condition::DuringPhase(_) => {}
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
    /// BEFORE the event binds — a pattern is not its own body).
    fn event(&mut self, event: &Event, ctx: &Ctx) {
        match event {
            Event::Performed { verb: _, by, on } => {
                self.filter(by, ctx, Kind::Any);
                self.filter(on, ctx, Kind::Any);
            }
            Event::ZoneMove { what, cause, .. } => {
                self.filter(what, ctx, Kind::Object);
                if let Some(deckmaste_core::Cause::Cause(pattern)) = cause
                    && let Some(agent) = &pattern.agent
                {
                    self.filter(agent, ctx, Kind::Any);
                }
            }
            Event::BeginningOf(_, _) | Event::DesignationChanged { .. } => {}
            Event::StateBecomes { of, becomes, cause } => {
                let expected = match becomes {
                    StateFilterEvent::Designated(name) => self
                        .tables
                        .designation_scope(name.as_str())
                        .unwrap_or(Kind::Any),
                    StateFilterEvent::ControlledBy(player) => {
                        self.filter(player, ctx, Kind::Player);
                        Kind::Object
                    }
                    StateFilterEvent::Tapped
                    | StateFilterEvent::Untapped
                    | StateFilterEvent::Attacking
                    | StateFilterEvent::Blocking
                    | StateFilterEvent::Blocked
                    | StateFilterEvent::Phased(_)
                    | StateFilterEvent::TurnedFace(_) => Kind::Object,
                };
                self.filter(of, ctx, expected);
                if let Some(deckmaste_core::Cause::Cause(pattern)) = cause
                    && let Some(agent) = &pattern.agent
                {
                    self.filter(agent, ctx, Kind::Any);
                }
            }
            Event::BecomesTarget { what, by } => {
                self.filter(what, ctx, Kind::Any);
                if let Some(by) = by {
                    self.filter(by, ctx, Kind::Object);
                }
            }
            Event::Used { by } => {
                self.reference(by, ctx, Kind::Object);
            }
            Event::OneOf(events) => {
                for event in events {
                    self.event(event, ctx);
                }
            }
            Event::Expanded(e) => self.event(&e.value, ctx),
        }
    }

    /// The caps an event pattern guarantees its body — table rows keyed by
    /// the pattern shape / `Performed` verb; a disjunction guarantees only
    /// the meet ([CR#603.2c] — the Idris `eventQueryCaps`).
    fn event_caps(&self, event: &Event) -> Caps {
        match event {
            Event::Performed { verb, .. } => self
                .tables
                .event_caps(&format!("Performed:{}", verb.as_str())),
            Event::ZoneMove { .. } => self.tables.event_caps("ZoneMove"),
            Event::BeginningOf(_, _) => self.tables.event_caps("BeginningOf"),
            Event::StateBecomes { becomes, .. } => {
                let key = match becomes {
                    StateFilterEvent::Tapped => "StateBecomes:Tapped",
                    StateFilterEvent::Untapped => "StateBecomes:Untapped",
                    StateFilterEvent::Attacking => "StateBecomes:Attacking",
                    StateFilterEvent::Blocking => "StateBecomes:Blocking",
                    StateFilterEvent::Blocked => "StateBecomes:Blocked",
                    StateFilterEvent::Phased(_) => "StateBecomes:Phased",
                    StateFilterEvent::TurnedFace(_) => "StateBecomes:TurnedFace",
                    StateFilterEvent::Designated(_) => "StateBecomes:Designated",
                    StateFilterEvent::ControlledBy(_) => "StateBecomes:ControlledBy",
                };
                self.tables.event_caps(key)
            }
            Event::BecomesTarget { .. } => self.tables.event_caps("BecomesTarget"),
            Event::DesignationChanged { .. } => self.tables.event_caps("DesignationChanged"),
            Event::Used { .. } => self.tables.event_caps("Used"),
            Event::OneOf(events) => match events.split_first() {
                None => NO_CAPS,
                Some((first, rest)) => rest.iter().fold(self.event_caps(first), |acc, e| {
                    acc.meet(self.event_caps(e))
                }),
            },
            Event::Expanded(e) => self.event_caps(&e.value),
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
            StaticEffect::TriggerMultiplier {
                cause,
                extra,
                affected,
            } => {
                self.event(cause, ctx);
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
                AsThough::SpendManaAsAnyColor | AsThough::Expanded(_) => {}
            },
            StaticEffect::Sba { when, then } => {
                self.condition(when, ctx);
                let body = self.descend(ctx, "Sba", None, Kind::Any, NO_CAPS);
                self.scoped("Sba", |w| {
                    w.effect(then, &body);
                });
            }
            StaticEffect::OutcomeGate { who, gate: _ } => {
                self.filter(who, ctx, Kind::Player);
            }
            StaticEffect::CantHappen(event) => self.event(event, ctx),
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
        let body = self.descend(ctx, "Where", None, Kind::Object, NO_CAPS);
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
                    w.event(would, ctx);
                    let caps = w.event_caps(would);
                    let body = w.descend(ctx, "Replacement.Instead", None, Kind::Any, caps);
                    w.effect(instead, &body);
                });
            }
            Replacement::Skip { what: _ } => {}
            Replacement::Also { would, also } => {
                self.scoped("Also", |w| {
                    w.event(would, ctx);
                    let caps = w.event_caps(would);
                    let body = w.descend(ctx, "Replacement.Also", None, Kind::Any, caps);
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
    /// `agentScope` rows); patients stay kind-poly ([CR#508.1]).
    fn deontic_action(&mut self, action: &DeonticAction, ctx: &Ctx) {
        let agent = |w: &mut Self, relation: &str, by: &Filter, ctx: &Ctx| {
            let expected = w.tables.agent_scope(relation).unwrap_or(Kind::Any);
            w.filter(by, ctx, expected);
        };
        match action {
            DeonticAction::Attack { by, on } => {
                agent(self, "Attack", by, ctx);
                self.filter(on, ctx, Kind::Any);
            }
            DeonticAction::Block { by, on, count } => {
                agent(self, "Block", by, ctx);
                self.filter(on, ctx, Kind::Object);
                if let Some(bound) = count {
                    self.count_bound(bound, ctx);
                    self.block_qty_floor(bound);
                }
            }
            DeonticAction::Target { by, on } => {
                agent(self, "Target", by, ctx);
                self.filter(on, ctx, Kind::Any);
            }
            DeonticAction::Attach { what, to } => {
                agent(self, "Attach", what, ctx);
                self.filter(to, ctx, Kind::Any);
            }
            DeonticAction::Cast { what, by, cost, .. } => {
                self.filter(what, ctx, Kind::Object);
                agent(self, "Cast", by, ctx);
                if let Some(AlternativeCost::Components(components)) = cost {
                    for (i, component) in components.iter().enumerate() {
                        self.scoped(format!("cost[{i}]"), |w| w.cost_component(component, ctx));
                    }
                }
            }
            DeonticAction::Play { what, by, .. } => {
                self.filter(what, ctx, Kind::Object);
                agent(self, "Play", by, ctx);
            }
            DeonticAction::Activate { what, by } => {
                self.filter(what, ctx, Kind::Object);
                agent(self, "Activate", by, ctx);
            }
            DeonticAction::Expanded(e) => self.deontic_action(&e.value, ctx),
        }
    }

    fn duration(&mut self, duration: &Duration, ctx: &Ctx) {
        match duration {
            Duration::FixedUntil(_) | Duration::EndOfGame => {}
            Duration::UntilEvent(event) => self.event(event, ctx),
            Duration::ForAsLongAs(condition) => self.condition(condition, ctx),
        }
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
