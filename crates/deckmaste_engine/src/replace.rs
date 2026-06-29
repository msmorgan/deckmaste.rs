//! Replacement effects ([CR#614]) — the `ZoneWillChange` replace stage. Stage 3
//! wires self-replacement on enter (`Also(would: Enters(This), …)`) into the
//! entering object's `EnterStatus`; other replacement kinds are Stage-4 seams
//! (§7.2).

use deckmaste_core::Ability;
use deckmaste_core::Action;
use deckmaste_core::Effect;
use deckmaste_core::Event;
use deckmaste_core::Filter;
use deckmaste_core::PlayerAction;
use deckmaste_core::Reference;
use deckmaste_core::Replacement;
use deckmaste_core::Selection;
use deckmaste_core::StaticEffect;
use deckmaste_core::Zone;

use crate::event::EnterStatus;
use crate::object::ObjectSource;
use crate::state::GameState;

impl GameState {
    /// [CR#614.1c,614.12]: the entering status a permanent's own augment
    /// replacements (`Also(would: Enters(This), also: …)`) impose. Recognises
    /// `Also(would: Enters(This), also: Tap(This))` → enters tapped, and
    /// `Also(would: Enters(This), also: Attach(This, to))` → enters attached
    /// ([CR#303.4], §4) — the host resolved from `to`; other `also` effects are
    /// a `todo!` seam.
    pub(crate) fn as_enters_status(
        &self,
        source: ObjectSource,
        entering: crate::object::ObjectId,
    ) -> EnterStatus {
        let mut status = EnterStatus::default();
        for ability in crate::derive::abilities_of_source(self, source) {
            if let Ability::Static(s) = &ability {
                for eff in &s.effects {
                    if let StaticEffect::Replacement(replacement) = eff
                        && let Replacement::Also { would, also } =
                            look_through_replacement(replacement)
                        && would_is_self_enter(would)
                    {
                        self.apply_as_enters(also, entering, &mut status);
                    }
                }
            }
        }
        status
    }

    /// Whether `source` carries an enters-attached self-replacement
    /// (`Also(would: Enters(This), also: Attach(This, …))`) — i.e. it attaches
    /// itself on entry ([CR#303.4], the Enchant `AsEnters`). The cast-path host
    /// resolution (spec §4) keys on this: a permanent SPELL with this
    /// replacement attaches to its resolving spell's chosen target, not an
    /// arbitrary candidate.
    pub(crate) fn enters_attached_self(&self, source: ObjectSource) -> bool {
        crate::derive::abilities_of_source(self, source)
            .iter()
            .any(|ability| {
                let Ability::Static(s) = ability else { return false };
                s.effects.iter().any(|eff| {
                    matches!(eff, StaticEffect::Replacement(r)
                        if matches!(look_through_replacement(r), Replacement::Also { would, also }
                            if would_is_self_enter(would) && also_is_self_attach(also)))
                })
            })
    }

    /// Fold one `also` effect into the entering status. `Tap(This)` → tapped;
    /// `Attach(This, to)` → enters attached, the host resolved from the `to`
    /// selection (§4). Counters/face-down are Stage-4 seams.
    fn apply_as_enters(
        &self,
        effect: &Effect,
        entering: crate::object::ObjectId,
        status: &mut EnterStatus,
    ) {
        match effect {
            // `Tap` is a `PlayerAction`, so the `AsEnters` sugar expands to
            // `Act(By(You, Tap(This)))` (the agent is irrelevant here).
            Effect::Act(Action::By(_, PlayerAction::Tap(Selection::Ref(Reference::This)))) => {
                status.tapped = true;
            }
            // [CR#303.4]: enters attached. `what` is always this object (the
            // self-replacement watcher); the host is whatever `to` resolves to.
            Effect::Act(Action::Attach { what, to }) if is_self_selection(what) => {
                status.attach_to = self.enters_attached_host(entering, to);
            }
            // [CR#122.6a,614.1c]: enters with counters. `PutCounters(This, kind,
            // n)` self-replacement → fold `(kind, n)` into the entering status.
            // `n` is evaluated against a `This`-anchored frame so a count that
            // scales ("a +1/+1 counter for each …") resolves at entry.
            Effect::Act(Action::By(_, PlayerAction::PutCounters(what, kind, count)))
                if is_self_selection(what) =>
            {
                let frame = self.enters_frame(entering);
                let n = self.eval_count(count, &frame);
                if n > 0 {
                    status.counters.push((kind.0, n));
                }
            }
            // [CR#614.1d,608.2c]: a conditional enters-replacement — the
            // dual-land "enters tapped unless you control …" gate, modeled as
            // `AsEnters(If(condition: <gate>, then: <fold>, otherwise: <fold>))`.
            // Evaluate the gate against a `This`-anchored entry frame (the "you"
            // is the entering object's controller) and fold the chosen branch.
            Effect::If(if_effect) => {
                let frame = self.enters_frame(entering);
                if self.condition_holds(&if_effect.condition, &frame) {
                    self.apply_as_enters(&if_effect.then, entering, status);
                } else if let Some(otherwise) = &if_effect.otherwise {
                    self.apply_as_enters(otherwise, entering, status);
                }
            }
            Effect::Expanded(e) => self.apply_as_enters(&e.value, entering, status),
            other => todo!("stage 3 does not interpret enters-replacement effect {other:?}"),
        }
    }

    /// The minimal resolution `Frame` for an enters-replacement fold: source =
    /// the entering object, controller = its controller (the "you" a gate
    /// condition reads), no targets/bindings/chosen/x/subject. Shared by the
    /// counter-count and conditional-gate folds.
    fn enters_frame(&self, entering: crate::object::ObjectId) -> crate::stack::Frame {
        crate::stack::Frame {
            source: entering,
            controller: self.objects.obj(entering).controller,
            targets: Vec::new(),
            bindings: None,
            chosen: None,
            x: None,
            subject: None,
            those: None,
        }
    }

    /// Resolve the host an `AsEnters(Attach(This, to))` attaches to (§4). The
    /// `to` selection is a *filter* over legal hosts (the enchant/equip
    /// `Param(0)` quality, or a target-position filter looked through here):
    /// pick the first battlefield permanent that both matches the `to` filter
    /// AND is a LEGAL host for `entering` per `attachment_legal` ([CR#303.4f] —
    /// the controller chooses among *legal* hosts; here, deterministic id
    /// order). `attachment_legal` folds in host-side protection ([CR#702.16d])
    /// on top of the `to` quality bound, generically — no subtype branch.
    ///
    /// Host resolution by entry context (spec §4) — entering from the stack
    /// attaches to the resolving spell's chosen target, entering otherwise
    /// surfaces a controller choice — is the Stage-4 cast-path wiring; Stage 1
    /// resolves the candidate set so the mechanism is exercised end-to-end.
    fn enters_attached_host(
        &self,
        entering: crate::object::ObjectId,
        to: &Selection,
    ) -> Option<crate::object::ObjectId> {
        let filter = host_filter(to)?;
        crate::target::candidates(self, filter)
            .into_iter()
            .find(|&id| {
                self.objects.obj(id).zone == Some(Zone::Battlefield)
                    && crate::legal::attachment_legal(self, entering, id)
            })
    }
}

/// The legal-host `Filter` carried by an `Attach`'s `to` selection, if it is a
/// filter-position selection (`Filter`/`Each`, or an `Expanded` wrapper). A
/// bare `Ref`/`Target`/`Choose` `to` is the cast-path host (Stage-4 wiring),
/// so it yields `None` here.
fn host_filter(to: &Selection) -> Option<&Filter> {
    match to {
        Selection::Filter(f) | Selection::Each(f) => Some(f),
        Selection::Expanded(e) => host_filter(&e.value),
        _ => None,
    }
}

/// Whether a `Selection` is this object itself (`Ref(This)`/`Ref(Ref(This))`),
/// looked through any remembered macro invocation.
fn is_self_selection(sel: &Selection) -> bool {
    match sel {
        Selection::Ref(Reference::This) => true,
        Selection::Ref(Reference::Expanded(e)) => {
            is_self_selection(&Selection::Ref((*e.value).clone()))
        }
        Selection::Expanded(e) => is_self_selection(&e.value),
        _ => false,
    }
}

/// Whether an `also` effect is this object attaching itself (`Attach(what:
/// This, to: …)`), looked through `Expanded` — the enters-attached shape.
fn also_is_self_attach(effect: &Effect) -> bool {
    match effect {
        Effect::Act(Action::Attach { what, .. }) => is_self_selection(what),
        Effect::Expanded(e) => also_is_self_attach(&e.value),
        _ => false,
    }
}

/// Look through a remembered `Replacement` macro invocation (`AsEnters`, …) to
/// the form it expanded to.
pub(crate) fn look_through_replacement(replacement: &Replacement) -> &Replacement {
    match replacement {
        Replacement::Expanded(e) => look_through_replacement(&e.value),
        other => other,
    }
}

/// Whether `would` is an enter-the-battlefield event for the watching object
/// itself — the `Enters(This)`/`Enters(Ref(This))` shape, looked through any
/// remembered macro invocation. Such a `would` on a static replacement is the
/// object's own self-enter (the watcher in `as_enters_status` is always self),
/// so a `Ref(This)`/`Any` `what` both qualify.
fn would_is_self_enter(would: &Event) -> bool {
    match would {
        // Look through `Enters(…)` and any other remembered Event macro.
        Event::Expanded(e) => would_is_self_enter(&e.value),
        // A move *to* the battlefield, of this object (or match-anything).
        Event::ZoneMove { what, to, .. } => {
            *to == Some(Zone::Battlefield)
                && matches!(what, Filter::Ref(Reference::This) | Filter::Any)
        }
        _ => false,
    }
}

#[cfg(test)]
mod tests {
    use std::path::Path;
    use std::sync::Arc;

    use deckmaste_cards::plugin::Plugin;
    use deckmaste_core::Ability;
    use deckmaste_core::Card;
    use deckmaste_core::CardFace;
    use deckmaste_core::StaticAbility;
    use deckmaste_core::Type;

    use super::*;
    use crate::agenda::WorkItem;
    use crate::event::GameEvent;
    use crate::event::Occurrence;
    use crate::object::ObjectId;
    use crate::player::PlayerId;
    use crate::state::GameConfig;
    use crate::state::GameState;
    use crate::state::PlayerConfig;
    use crate::state::StartingPlayer;
    use crate::step::StepOutcome;

    fn canon() -> Plugin {
        Plugin::load_with_sibling_prelude(
            Path::new(env!("CARGO_MANIFEST_DIR")).join("../../plugins/canon"),
        )
        .unwrap()
    }

    fn game() -> GameState {
        GameState::new(GameConfig {
            players: vec![PlayerConfig { deck: vec![] }, PlayerConfig { deck: vec![] }],
            seed: 7,
            starting_life: 20,
            starting_player: StartingPlayer::Fixed(PlayerId(0)),
            sba_rules: vec![],
            counter_decls: std::collections::HashMap::new(),
            subtypes: std::collections::HashMap::new(),
        })
    }

    /// A synthetic enchantment whose sole ability is the enters-attached
    /// self-replacement `AsEnters(Attach(This, to: <a creature on the
    /// battlefield>))` — the `to` is a filter, the Stage-1 candidate-set form.
    fn enchant_aura_card() -> Card {
        Card::Normal(CardFace {
            name: "Test Aura".into(),
            types: vec![Type::Enchantment],
            abilities: vec![Ability::Static(StaticAbility {
                from: None,
                condition: None,
                effects: vec![StaticEffect::Replacement(Box::new(Replacement::Also {
                    would: Event::ZoneMove {
                        what: Filter::Ref(Reference::This),
                        from: None,
                        to: Some(Zone::Battlefield),
                        face: None,
                        cause: None,
                    },
                    also: Effect::Act(Action::Attach {
                        what: Selection::this(),
                        to: Selection::Filter(Filter::AllOf(vec![
                            Filter::State(deckmaste_core::StateFilter::InZone(Zone::Battlefield)),
                            Filter::creature(),
                        ])),
                    }),
                }))],
                characteristic_defining: false,
            })],
            ..CardFace::default()
        })
    }

    /// Put a Grizzly Bears on the battlefield as a host. Returns its id.
    fn host_creature(state: &mut GameState) -> ObjectId {
        let bears = Arc::new(canon().card("Grizzly Bears").unwrap());
        let card = state.cards.push(bears, PlayerId(0));
        let id = state.objects.mint(
            ObjectSource::Card(card),
            PlayerId(0),
            Some(Zone::Battlefield),
        );
        state.zones.battlefield.push(id);
        id
    }

    /// Mint the Aura in its owner's hand and run a hand→battlefield entry to
    /// completion (a non-cast ETB, so the host comes from the candidate set,
    /// §4; the cast path's host-from-target is Stage-4 wiring). Returns the
    /// Aura's reminted battlefield id.
    fn enter_aura(state: &mut GameState) -> ObjectId {
        let card = state.cards.push(Arc::new(enchant_aura_card()), PlayerId(0));
        let hand_id = state
            .objects
            .mint(ObjectSource::Card(card), PlayerId(0), Some(Zone::Hand));
        state.zones.hands[PlayerId(0).index()].push(hand_id);
        state.schedule_front(vec![WorkItem::Emit(Occurrence::single(
            GameEvent::ZoneWillChange {
                object: hand_id,
                from: Some(Zone::Hand),
                to: Zone::Battlefield,
                enters: None,
                position: None,
                face: None,
                cause: None,
            },
        ))]);
        for _ in 0..30 {
            if matches!(state.step(), StepOutcome::NeedsDecision(_)) {
                break;
            }
        }
        *state
            .zones
            .battlefield
            .iter()
            .find(|&&o| state.objects.obj(o).card_id() == Some(card))
            .expect("the Aura entered the battlefield")
    }

    /// [CR#303.4]: an Aura with `AsEnters(Attach(This, to: Creature))` enters
    /// the battlefield already attached to a legal host — no observable
    /// unattached window, and the `Attached` fact is recorded.
    #[test]
    fn enters_attached_to_a_legal_host() {
        let mut state = game();
        let host = host_creature(&mut state);
        let aura = enter_aura(&mut state);
        assert_eq!(
            state.objects.obj(aura).attached_to,
            Some(host),
            "the Aura entered attached to the host creature"
        );
        assert!(
            state
                .history
                .scan(deckmaste_core::Window::ThisGame, state.turn.turn_number)
                .any(|e| matches!(e, GameEvent::Attached { attachment, host: h }
                    if *attachment == aura && *h == host)),
            "the Attached fact was recorded on entry"
        );
    }

    /// [CR#303.4f] (v1 approximation, §4): with no legal host on the
    /// battlefield, the Aura enters unattached (the §5 SBA then graveyards it —
    /// that sweep is Stage 2). No `Attached` fact.
    #[test]
    fn enters_unattached_when_no_legal_host() {
        let mut state = game();
        let aura = enter_aura(&mut state);
        assert_eq!(
            state.objects.obj(aura).attached_to,
            None,
            "no creature to attach to → enters unattached"
        );
        assert!(
            !state
                .history
                .scan(deckmaste_core::Window::ThisGame, state.turn.turn_number)
                .any(|e| matches!(e, GameEvent::Attached { .. })),
            "no Attached fact when there was no legal host"
        );
    }

    /// [CR#122.6a,614.1c]: a permanent with an `AsEnters(PutCounters(This,
    /// P1P1Counter, 2))` self-replacement enters the battlefield already
    /// carrying two `P1P1Counter` counters — placed atomically at mint, before
    /// the `ZoneChanged` fact.
    #[test]
    fn enters_with_counters() {
        use deckmaste_core::Count;

        let card = Card::Normal(CardFace {
            name: "Test Counterer".into(),
            types: vec![Type::Artifact],
            abilities: vec![Ability::Static(StaticAbility {
                from: None,
                condition: None,
                effects: vec![StaticEffect::Replacement(Box::new(Replacement::Also {
                    would: Event::ZoneMove {
                        what: Filter::Ref(Reference::This),
                        from: None,
                        to: Some(Zone::Battlefield),
                        face: None,
                        cause: None,
                    },
                    also: Effect::Act(Action::By(
                        Reference::You,
                        PlayerAction::PutCounters(
                            Selection::this(),
                            "P1P1Counter".into(),
                            Count::Literal(2),
                        ),
                    )),
                }))],
                characteristic_defining: false,
            })],
            ..CardFace::default()
        });

        let mut state = game();
        let card_id = state.cards.push(Arc::new(card), PlayerId(0));
        let hand_id =
            state
                .objects
                .mint(ObjectSource::Card(card_id), PlayerId(0), Some(Zone::Hand));
        state.zones.hands[PlayerId(0).index()].push(hand_id);
        state.schedule_front(vec![WorkItem::Emit(Occurrence::single(
            GameEvent::ZoneWillChange {
                object: hand_id,
                from: Some(Zone::Hand),
                to: Zone::Battlefield,
                enters: None,
                position: None,
                face: None,
                cause: None,
            },
        ))]);
        for _ in 0..10 {
            if matches!(state.step(), StepOutcome::NeedsDecision(_)) {
                break;
            }
        }
        let entered = *state
            .zones
            .battlefield
            .iter()
            .find(|&&o| state.objects.obj(o).card_id() == Some(card_id))
            .expect("the permanent entered the battlefield");
        assert_eq!(
            state
                .objects
                .obj(entered)
                .counters
                .get(&deckmaste_core::Ident::from("P1P1Counter"))
                .copied(),
            Some(2),
            "enters with two P1P1Counter counters"
        );
    }

    /// A land whose sole ability is the dual-land conditional
    /// enters-replacement `AsEnters(If(condition: Not(Compare(CountOf(other
    /// lands you control), AtLeast, 1)), then: Tap(This)))` — "~ enters
    /// tapped unless you control one or more other lands" ([CR#614.1d]).
    fn tapped_unless_land() -> Card {
        use deckmaste_core::CharacteristicFilter;
        use deckmaste_core::Cmp;
        use deckmaste_core::Condition;
        use deckmaste_core::Count;
        use deckmaste_core::If;
        use deckmaste_core::RelationFilter;

        let other_lands_you_control = Filter::AllOf(vec![
            Filter::Characteristic(CharacteristicFilter::Type(Type::Land)),
            Filter::Not(Box::new(Filter::Ref(Reference::This))),
            Filter::Relation(RelationFilter::ControlledBy(Box::new(Filter::Ref(
                Reference::You,
            )))),
        ]);
        let gate = Condition::Compare(
            Count::CountOf(Box::new(other_lands_you_control)),
            Cmp::AtLeast,
            Count::Literal(1),
        );
        Card::Normal(CardFace {
            name: "Test Tapland".into(),
            types: vec![Type::Land],
            abilities: vec![Ability::Static(StaticAbility {
                from: None,
                condition: None,
                effects: vec![StaticEffect::Replacement(Box::new(Replacement::Also {
                    would: Event::ZoneMove {
                        what: Filter::Ref(Reference::This),
                        from: None,
                        to: Some(Zone::Battlefield),
                        face: None,
                        cause: None,
                    },
                    also: Effect::If(If {
                        condition: Condition::Not(Box::new(gate)),
                        then: Box::new(Effect::Act(Action::By(
                            Reference::You,
                            PlayerAction::Tap(Selection::this()),
                        ))),
                        otherwise: None,
                    }),
                }))],
                characteristic_defining: false,
            })],
            ..CardFace::default()
        })
    }

    /// Put a vanilla land on the battlefield under P0. Returns its id — the
    /// "other land" the gate counts.
    fn other_land(state: &mut GameState) -> ObjectId {
        let land = Card::Normal(CardFace {
            name: "Test Land".into(),
            types: vec![Type::Land],
            ..CardFace::default()
        });
        let card = state.cards.push(Arc::new(land), PlayerId(0));
        let id = state.objects.mint(
            ObjectSource::Card(card),
            PlayerId(0),
            Some(Zone::Battlefield),
        );
        state.zones.battlefield.push(id);
        id
    }

    /// Run the tapland from hand to battlefield, returning its reminted id.
    fn enter_tapland(state: &mut GameState) -> ObjectId {
        let card = state
            .cards
            .push(Arc::new(tapped_unless_land()), PlayerId(0));
        let hand_id = state
            .objects
            .mint(ObjectSource::Card(card), PlayerId(0), Some(Zone::Hand));
        state.zones.hands[PlayerId(0).index()].push(hand_id);
        state.schedule_front(vec![WorkItem::Emit(Occurrence::single(
            GameEvent::ZoneWillChange {
                object: hand_id,
                from: Some(Zone::Hand),
                to: Zone::Battlefield,
                enters: None,
                position: None,
                face: None,
                cause: None,
            },
        ))]);
        // Step only until the land appears on the battlefield, then STOP — a
        // further `step()` would advance into the untap step and clear the
        // tapped status this test reads ([CR#502.3]).
        for _ in 0..10 {
            if let Some(&id) = state
                .zones
                .battlefield
                .iter()
                .find(|&&o| state.objects.obj(o).card_id() == Some(card))
            {
                return id;
            }
            if matches!(state.step(), StepOutcome::NeedsDecision(_)) {
                break;
            }
        }
        panic!("the tapland never entered the battlefield");
    }

    /// [CR#614.1d]: with NO other land you control, the unless-condition is
    /// false, so the conditional fold taps the land on entry.
    #[test]
    fn conditional_enters_tapped_when_gate_unmet() {
        let mut state = game();
        let land = enter_tapland(&mut state);
        assert!(
            state.objects.obj(land).tapped,
            "no other land → unless-condition false → enters tapped"
        );
    }

    /// [CR#614.1d]: with an other land already in play, the unless-condition
    /// holds, so the fold does nothing and the land enters untapped.
    #[test]
    fn conditional_enters_untapped_when_gate_met() {
        let mut state = game();
        other_land(&mut state);
        let land = enter_tapland(&mut state);
        assert!(
            !state.objects.obj(land).tapped,
            "an other land is present → unless-condition true → enters untapped"
        );
    }
}
