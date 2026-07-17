//! Replacement effects ([CR#614]) — the future-form `ZoneChange` replace stage.
//! Stage 3 wires self-replacement on enter (`Also(would: Enters(This), …)`)
//! into the entering object's `EnterStatus`; other replacement kinds are
//! Stage-4 seams (§7.2).

use deckmaste_core::Ability;
use deckmaste_core::Action;
use deckmaste_core::EventFilter;
use deckmaste_core::OneShotEffect;
use deckmaste_core::PlayerAction;
use deckmaste_core::Predicate;
use deckmaste_core::Reference;
use deckmaste_core::Replacement;
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
        for ability in self.enters_fold_abilities(source, entering) {
            if let Ability::Static(s) = ability.peel_innate()
                && let StaticEffect::Replacement(replacement) = s.as_ref()
                && let Replacement::Also { would, also } = look_through_replacement(replacement)
                && would_is_self_enter(would)
            {
                self.apply_as_enters(also, entering, &mut status);
            }
        }
        status
    }

    /// The abilities the enters-status fold scans ([Task 2]): `source`'s
    /// printed abilities plus any `ConferralRule` conferrals whose `scope`
    /// matches `entering` (`crate::matches`, the same predicate-scope matcher
    /// `global_sba_rules` uses) — a predicate-scoped augment-on-enter
    /// replacement (`Also { would: ThisEnters, .. }`) applies exactly like a
    /// printed one. A conferred ability arrives `Ability::Innate`-wrapped
    /// ([`Property::conferred_ability`]), so `as_enters_status` peels
    /// `Innate` on every ability from this list before matching the
    /// `Static(Replacement(Also {..}))` shape — a no-op for the printed ones,
    /// which are never wrapped.
    fn enters_fold_abilities(
        &self,
        source: ObjectSource,
        entering: crate::object::ObjectId,
    ) -> Vec<Ability> {
        let mut abilities = crate::derive::abilities_of_source(self, source);
        abilities.extend(
            self.conferral_rules
                .iter()
                .filter(|rule| crate::matches(self, entering, &rule.scope))
                .filter_map(|rule| rule.confer.conferred_ability()),
        );
        abilities
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
                matches!(s.as_ref(), StaticEffect::Replacement(r)
                    if matches!(look_through_replacement(r), Replacement::Also { would, also }
                        if would_is_self_enter(would) && also_is_self_attach(also)))
            })
    }

    /// Fold one `also` effect into the entering status. `Tap(This)` → tapped;
    /// `Attach(This, to)` → enters attached, the host resolved from the `to`
    /// selection (§4). Counters/face-down are Stage-4 seams.
    fn apply_as_enters(
        &self,
        effect: &OneShotEffect,
        entering: crate::object::ObjectId,
        status: &mut EnterStatus,
    ) {
        match effect {
            // `Tap` is a `PlayerAction`, so the `AsEnters` sugar expands to
            // `Act(By(You, Tap(This)))` (the agent is irrelevant here).
            OneShotEffect::Act(Action::By(_, PlayerAction::Tap(Reference::This))) => {
                status.tapped = true;
            }
            // [CR#303.4,303.4f]: enters attached. The enters-attached shape is
            // `With(ChooseOne(quality), Attach(This, That))` — as it enters, the
            // controller chooses a legal host matching the Aura's enchant
            // quality (choosing BEFORE the attach, never inside the verb). v1
            // picks the first legal candidate; the cast-path choice is Stage-4.
            OneShotEffect::With(_) if enters_attached_quality(effect).is_some() => {
                let filter = enters_attached_quality(effect).expect("guarded just above");
                status.attach_to = self.enters_attached_host(entering, filter);
            }
            // [CR#122.6a,614.1c]: enters with counters. `PutCounters(This, kind,
            // n)` self-replacement → fold `(kind, n)` into the entering status.
            // `n` is evaluated against a `This`-anchored frame so a count that
            // scales ("a +1/+1 counter for each …") resolves at entry.
            OneShotEffect::Act(Action::By(_, PlayerAction::PutCounters(what, kind, count)))
                if is_self_reference(what) =>
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
            OneShotEffect::If(if_effect) => {
                let frame = self.enters_frame(entering);
                if self.condition_holds(&if_effect.condition, &frame) {
                    self.apply_as_enters(&if_effect.then, entering, status);
                } else if let Some(otherwise) = &if_effect.otherwise {
                    self.apply_as_enters(otherwise, entering, status);
                }
            }
            OneShotEffect::Expanded(e) => self.apply_as_enters(&e.value, entering, status),
            other => todo!("stage 3 does not interpret enters-replacement effect {other:?}"),
        }
    }

    /// The minimal resolution `Frame` for an enters-replacement fold: source =
    /// the entering object, controller = its controller (the "you" a gate
    /// condition reads), no targets/bindings/chosen/x/subject. Shared by the
    /// counter-count and conditional-gate folds.
    fn enters_frame(&self, entering: crate::object::ObjectId) -> crate::stack::Frame {
        crate::stack::Frame::bare(entering, self.objects.obj(entering).controller)
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
        filter: &Predicate,
    ) -> Option<crate::object::ObjectId> {
        crate::target::candidates(self, filter)
            .into_iter()
            .find(|&id| {
                self.objects.obj(id).zone == Some(Zone::Battlefield)
                    && crate::legal::attachment_legal(self, entering, id)
            })
    }
}

/// The host-quality `Predicate` of an enters-attached self-replacement —
/// `With(ChooseOne(quality), Attach(This, It))` ([CR#303.4f]: as it enters,
/// the controller chooses a legal host matching the Aura's enchant quality),
/// looked through `Expanded`. `None` for any other shape.
fn enters_attached_quality(effect: &OneShotEffect) -> Option<&Predicate> {
    match effect {
        OneShotEffect::With(with) => {
            let body_is_self_attach = matches!(
                &*with.body,
                OneShotEffect::Act(Action::Attach { what, to })
                    if is_self_reference(what) && matches!(to, Reference::It)
            );
            if body_is_self_attach { host_quality(&with.binder) } else { None }
        }
        OneShotEffect::Expanded(e) => enters_attached_quality(&e.value),
        _ => None,
    }
}

/// The `Predicate` a `ChooseOne` binder chooses among (the host quality),
/// through a remembered macro invocation.
fn host_quality(binder: &deckmaste_core::Binder) -> Option<&Predicate> {
    match binder {
        deckmaste_core::Binder::ChooseOne { filter, .. } => Some(filter),
        deckmaste_core::Binder::Expanded(e) => host_quality(&e.value),
        _ => None,
    }
}

/// Whether a `Reference` is this object itself (`This`), looked through any
/// remembered macro invocation.
fn is_self_reference(r: &Reference) -> bool {
    match r {
        Reference::This => true,
        Reference::Expanded(e) => is_self_reference(&e.value),
        _ => false,
    }
}

/// Whether an `also` effect is this object attaching itself on entry — the
/// enters-attached shape `With(ChooseOne(quality), Attach(This, That))`,
/// looked through `Expanded`.
fn also_is_self_attach(effect: &OneShotEffect) -> bool {
    enters_attached_quality(effect).is_some()
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
fn would_is_self_enter(would: &EventFilter) -> bool {
    match would {
        // Look through `Enters(…)` and any other remembered Event macro.
        EventFilter::Expanded(e) => would_is_self_enter(&e.value),
        // A move *to* the battlefield, of this object (or match-anything).
        EventFilter::ZoneChange { what, to, .. } => {
            *to == Some(Zone::Battlefield)
                && matches!(what, Predicate::Ref(Reference::This) | Predicate::Any)
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
    use deckmaste_core::Deontic;
    use deckmaste_core::DeonticAction;
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
            conferral_rules: vec![],
            damage_result_rules: vec![],
            counter_decls: std::collections::HashMap::new(),
            subtypes: std::collections::HashMap::new(),
            types: std::collections::HashMap::new(),
        })
    }

    /// A synthetic enchantment carrying the enters-attached self-replacement
    /// `AsEnters(With(ChooseOne(<a creature on the battlefield>), Attach(This,
    /// to: That)))` — the host quality is the `ChooseOne` filter; the
    /// controller chooses a legal host as it enters ([CR#303.4f]) — PLUS
    /// the default-deny `May(Attach(what: Ref(This), to: Creature))` grant
    /// (the Enchant keyword's legal-host grant [CR#702.5a]) without which
    /// the attach would be illegal.
    fn enchant_aura_card() -> Card {
        Card::Normal(CardFace {
            name: "Test Aura".into(),
            types: vec![Type::Enchantment.def()],
            abilities: vec![
                Ability::r#static(StaticEffect::Deontic(Deontic::May(DeonticAction::Attach {
                    what: Predicate::Ref(Reference::This),
                    to: Predicate::creature(),
                }))),
                Ability::r#static(StaticEffect::Replacement(Box::new(Replacement::Also {
                    would: EventFilter::ZoneChange {
                        what: Predicate::Ref(Reference::This),
                        from: None,
                        to: Some(Zone::Battlefield),
                        cause: None,
                    },
                    also: OneShotEffect::With(deckmaste_core::With {
                        binder: deckmaste_core::Binder::ChooseOne {
                            filter: Predicate::And(vec![
                                Predicate::State(deckmaste_core::StatePredicate::InZone(
                                    Zone::Battlefield,
                                )),
                                Predicate::creature(),
                            ]),
                            by: Reference::You,
                        },
                        body: Box::new(OneShotEffect::Act(Action::Attach {
                            what: Reference::This,
                            to: Reference::It,
                        })),
                    }),
                }))),
            ],
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
            GameEvent::ZoneChange {
                snapshot: None,
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
                .scan(deckmaste_core::Lookback::ThisGame, state.turn.turn_number)
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
                .scan(deckmaste_core::Lookback::ThisGame, state.turn.turn_number)
                .any(|e| matches!(e, GameEvent::Attached { .. })),
            "no Attached fact when there was no legal host"
        );
    }

    /// [CR#122.6a,614.1c]: a permanent with an `AsEnters(PutCounters(This,
    /// P1P1Counter, 2))` self-replacement enters the battlefield already
    /// carrying two `P1P1Counter` counters — placed atomically at mint, before
    /// the past-form `ZoneChange` fact.
    #[test]
    fn enters_with_counters() {
        use deckmaste_core::Count;

        let card = Card::Normal(CardFace {
            name: "Test Counterer".into(),
            types: vec![Type::Artifact.def()],
            abilities: vec![Ability::r#static(StaticEffect::Replacement(Box::new(
                Replacement::Also {
                    would: EventFilter::ZoneChange {
                        what: Predicate::Ref(Reference::This),
                        from: None,
                        to: Some(Zone::Battlefield),
                        cause: None,
                    },
                    also: OneShotEffect::Act(Action::By(
                        Reference::You,
                        PlayerAction::PutCounters(
                            Reference::This,
                            "P1P1Counter".into(),
                            Count::Literal(2),
                        ),
                    )),
                },
            )))],
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
            GameEvent::ZoneChange {
                snapshot: None,
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
        use deckmaste_core::CharacteristicPredicate;
        use deckmaste_core::Cmp;
        use deckmaste_core::Condition;
        use deckmaste_core::Count;
        use deckmaste_core::If;
        use deckmaste_core::RelationPredicate;

        let other_lands_you_control = Predicate::And(vec![
            Predicate::Characteristic(CharacteristicPredicate::Type(Type::Land.name())),
            Predicate::Not(Box::new(Predicate::Ref(Reference::This))),
            Predicate::Relation(RelationPredicate::ControlledBy(Box::new(Predicate::Ref(
                Reference::You,
            )))),
        ]);
        let gate = Condition::Compare(
            Count::CountOf(deckmaste_core::Countable::Objects(Box::new(
                other_lands_you_control,
            ))),
            Cmp::AtLeast,
            Count::Literal(1),
        );
        Card::Normal(CardFace {
            name: "Test Tapland".into(),
            types: vec![Type::Land.def()],
            abilities: vec![Ability::r#static(StaticEffect::Replacement(Box::new(
                Replacement::Also {
                    would: EventFilter::ZoneChange {
                        what: Predicate::Ref(Reference::This),
                        from: None,
                        to: Some(Zone::Battlefield),
                        cause: None,
                    },
                    also: OneShotEffect::If(If {
                        condition: Condition::Not(Box::new(gate)),
                        then: Box::new(OneShotEffect::Act(Action::By(
                            Reference::You,
                            PlayerAction::Tap(Reference::This),
                        ))),
                        otherwise: None,
                    }),
                },
            )))],
            ..CardFace::default()
        })
    }

    /// Put a vanilla land on the battlefield under P0. Returns its id — the
    /// "other land" the gate counts.
    fn other_land(state: &mut GameState) -> ObjectId {
        let land = Card::Normal(CardFace {
            name: "Test Land".into(),
            types: vec![Type::Land.def()],
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
            GameEvent::ZoneChange {
                snapshot: None,
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

    /// [Task 2]: a predicate-scoped `ConferralRule { scope: Type(Planeswalker),
    /// confer: Property::Ability(Static(Replacement(Also { would: ThisEnters,
    /// also: PutCounters(This, LoyaltyCounter, 3) }))) }` folds into
    /// `as_enters_status` exactly like a PRINTED self-replacement does
    /// ([CR#122.6a,614.1c]) — the card carries NO abilities of its own; the
    /// enters-with-counters behavior comes entirely from the conferral rule
    /// matching the entering object's type. `Literal(3)` here (the
    /// printed-loyalty read is a later task).
    #[test]
    fn conferred_enters_with_counters_by_type() {
        use deckmaste_core::CharacteristicPredicate;
        use deckmaste_core::ConferralRule;
        use deckmaste_core::Count;
        use deckmaste_core::Property;

        let mut state = game();
        state.conferral_rules = vec![ConferralRule {
            scope: Predicate::Characteristic(CharacteristicPredicate::Type(
                Type::Planeswalker.name(),
            )),
            confer: Property::Ability(Box::new(Ability::r#static(StaticEffect::Replacement(
                Box::new(Replacement::Also {
                    would: EventFilter::ZoneChange {
                        what: Predicate::Ref(Reference::This),
                        from: None,
                        to: Some(Zone::Battlefield),
                        cause: None,
                    },
                    also: OneShotEffect::Act(Action::By(
                        Reference::You,
                        PlayerAction::PutCounters(
                            Reference::This,
                            "LoyaltyCounter".into(),
                            Count::Literal(3),
                        ),
                    )),
                }),
            )))),
        }];

        let card = Card::Normal(CardFace {
            name: "Test Walker".into(),
            types: vec![Type::Planeswalker.def()],
            ..CardFace::default()
        });
        let card_id = state.cards.push(Arc::new(card), PlayerId(0));
        let hand_id =
            state
                .objects
                .mint(ObjectSource::Card(card_id), PlayerId(0), Some(Zone::Hand));
        state.zones.hands[PlayerId(0).index()].push(hand_id);
        state.schedule_front(vec![WorkItem::Emit(Occurrence::single(
            GameEvent::ZoneChange {
                snapshot: None,
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
            .expect("the planeswalker entered the battlefield");
        assert_eq!(
            state
                .objects
                .obj(entered)
                .counters
                .get(&deckmaste_core::Ident::from("LoyaltyCounter"))
                .copied(),
            Some(3),
            "the conferred replacement put 3 LoyaltyCounters on entry"
        );
    }

    /// [Task 3, acceptance gate]: the same enters-with-counters behavior as
    /// [`conferred_enters_with_counters_by_type`], but proven through the
    /// REAL data path end to end, not a hand-built `ConferralRule` — a
    /// `rules/grant/*.ron` file in a tempdir plugin, loaded by the real
    /// `Plugin::load` ([Task 1]'s `load_conferral_rules`), plumbed through
    /// `GameConfig.conferral_rules` into `GameState.conferral_rules` ([Task
    /// 2]'s wiring), and folded by `as_enters_status` on entry. A
    /// TEST-SCOPED fixture, not a committed builtin rule: `Literal(3)` here
    /// would confer three loyalty counters on every real planeswalker — the
    /// builtin rule reads `StatOf(This, Loyalty)` and is a later task, once
    /// `Stat::Loyalty` and a printed-loyalty planeswalker card exist.
    #[test]
    fn data_conferred_rule_applies_at_entry_end_to_end() {
        let root = tempfile::tempdir().unwrap();
        let grant_dir = root.path().join("rules").join("grant");
        std::fs::create_dir_all(&grant_dir).unwrap();
        std::fs::write(
            grant_dir.join("planeswalker-loyalty.ron"),
            r#"[
                ConferralRule(
                    scope: Type("Planeswalker"),
                    confer: Ability(Static(Replacement(Also(
                        would: ZoneChange(what: Ref(This), to: Battlefield),
                        also: PutCounters(This, LoyaltyCounter, Literal(3)),
                    )))),
                ),
            ]"#,
        )
        .unwrap();

        // The real loader (Task 1): a plugin with no `macros/`, just the
        // `rules/grant/` fixture.
        let plugin = Plugin::load(root.path()).unwrap();
        assert_eq!(
            plugin.conferral_rules.len(),
            1,
            "the fixture file loaded one conferral rule via load_conferral_rules"
        );

        // The real plumbing (Task 2): loader output flows into GameConfig,
        // never a hand-set `state.conferral_rules`.
        let mut state = GameState::new(GameConfig {
            players: vec![PlayerConfig { deck: vec![] }, PlayerConfig { deck: vec![] }],
            seed: 7,
            starting_life: 20,
            starting_player: StartingPlayer::Fixed(PlayerId(0)),
            sba_rules: vec![],
            conferral_rules: plugin.conferral_rules,
            damage_result_rules: vec![],
            counter_decls: std::collections::HashMap::new(),
            subtypes: std::collections::HashMap::new(),
            types: std::collections::HashMap::new(),
        });

        let card = Card::Normal(CardFace {
            name: "Test Walker".into(),
            types: vec![Type::Planeswalker.def()],
            ..CardFace::default()
        });
        let card_id = state.cards.push(Arc::new(card), PlayerId(0));
        let hand_id =
            state
                .objects
                .mint(ObjectSource::Card(card_id), PlayerId(0), Some(Zone::Hand));
        state.zones.hands[PlayerId(0).index()].push(hand_id);
        state.schedule_front(vec![WorkItem::Emit(Occurrence::single(
            GameEvent::ZoneChange {
                snapshot: None,
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
            .expect("the planeswalker entered the battlefield");
        assert_eq!(
            state
                .objects
                .obj(entered)
                .counters
                .get(&deckmaste_core::Ident::from("LoyaltyCounter"))
                .copied(),
            Some(3),
            "the data-loaded conferral rule put 3 LoyaltyCounters on entry"
        );
    }

    /// [Task 5]: the committed builtin `rules/grant/planeswalker-loyalty.ron`
    /// ([CR#306.5b]'s intrinsic "enters with loyalty counters equal to its
    /// printed loyalty" rule) applies to a REAL planeswalker loaded through
    /// the REAL `plugins/builtin` `Plugin`/`GameConfig`/`GameState` path —
    /// `state.conferral_rules` is never hand-set here, unlike
    /// [`conferred_enters_with_counters_by_type`]'s Task-2 fixture or
    /// [`data_conferred_rule_applies_at_entry_end_to_end`]'s tempdir fixture.
    /// Mirrors that end-to-end harness, but sourcing the builtin plugin so
    /// the committed rule file is what's under test, and checks two printed
    /// loyalties land as that many `LoyaltyCounter`s.
    #[test]
    fn builtin_planeswalker_loyalty_conferral_applies_at_entry() {
        let builtin =
            Plugin::load(Path::new(env!("CARGO_MANIFEST_DIR")).join("../../plugins/builtin"))
                .unwrap();
        assert!(
            builtin.conferral_rules.iter().any(|rule| rule.scope
                == Predicate::Characteristic(deckmaste_core::CharacteristicPredicate::Type(
                    Type::Planeswalker.name()
                ))),
            "the builtin plugin loaded a Type(\"Planeswalker\") conferral rule from \
             rules/grant/planeswalker-loyalty.ron"
        );

        for printed_loyalty in [4, 3] {
            let mut state = GameState::new(GameConfig {
                players: vec![PlayerConfig { deck: vec![] }, PlayerConfig { deck: vec![] }],
                seed: 7,
                starting_life: 20,
                starting_player: StartingPlayer::Fixed(PlayerId(0)),
                sba_rules: vec![],
                conferral_rules: builtin.conferral_rules.clone(),
                damage_result_rules: vec![],
                counter_decls: std::collections::HashMap::new(),
                subtypes: std::collections::HashMap::new(),
                types: std::collections::HashMap::new(),
            });

            let card = Card::Normal(CardFace {
                name: "Test Walker".into(),
                types: vec![Type::Planeswalker.def()],
                loyalty: Some(deckmaste_core::StatValue::Number(printed_loyalty)),
                ..CardFace::default()
            });
            let card_id = state.cards.push(Arc::new(card), PlayerId(0));
            let hand_id =
                state
                    .objects
                    .mint(ObjectSource::Card(card_id), PlayerId(0), Some(Zone::Hand));
            state.zones.hands[PlayerId(0).index()].push(hand_id);
            state.schedule_front(vec![WorkItem::Emit(Occurrence::single(
                GameEvent::ZoneChange {
                    snapshot: None,
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
                .expect("the planeswalker entered the battlefield");
            assert_eq!(
                state
                    .objects
                    .obj(entered)
                    .counters
                    .get(&deckmaste_core::Ident::from("LoyaltyCounter"))
                    .copied(),
                Some(u32::try_from(printed_loyalty).unwrap()),
                "a printed-{printed_loyalty} planeswalker enters with {printed_loyalty} \
                 LoyaltyCounters via the builtin conferral rule"
            );
        }
    }
}
