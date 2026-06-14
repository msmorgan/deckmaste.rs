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
    pub(crate) fn as_enters_status(&self, source: ObjectSource) -> EnterStatus {
        let mut status = EnterStatus::default();
        for ability in crate::derive::abilities_of_source(self, source) {
            if let Ability::Static(s) = &ability {
                for eff in &s.effects {
                    if let StaticEffect::Replacement(replacement) = eff
                        && let Replacement::Also { would, also } = look_through(replacement)
                        && would_is_self_enter(would)
                    {
                        self.apply_as_enters(also, &mut status);
                    }
                }
            }
        }
        status
    }

    /// Fold one `also` effect into the entering status. `Tap(This)` → tapped;
    /// `Attach(This, to)` → enters attached, the host resolved from the `to`
    /// selection (§4). Counters/face-down are Stage-4 seams.
    fn apply_as_enters(&self, effect: &Effect, status: &mut EnterStatus) {
        match effect {
            // `Tap` is a `PlayerAction`, so the `AsEnters` sugar expands to
            // `Act(By(You, Tap(This)))` (the agent is irrelevant here).
            Effect::Act(Action::By(_, PlayerAction::Tap(Selection::Ref(Reference::This)))) => {
                status.tapped = true;
            }
            // [CR#303.4]: enters attached. `what` is always this object (the
            // self-replacement watcher); the host is whatever `to` resolves to.
            Effect::Act(Action::Attach { what, to }) if is_self_selection(what) => {
                status.attach_to = self.enters_attached_host(to);
            }
            Effect::Expanded(e) => self.apply_as_enters(&e.value, status),
            other => todo!("stage 3 does not interpret enters-replacement effect {other:?}"),
        }
    }

    /// Resolve the host an `AsEnters(Attach(This, to))` attaches to (§4). The
    /// `to` selection is a *filter* over legal hosts (the enchant/equip
    /// `Param(0)` quality, or a target-position filter looked through here):
    /// pick the first matching battlefield permanent in deterministic id order.
    ///
    /// Host resolution by entry context (spec §4) — entering from the stack
    /// attaches to the resolving spell's chosen target, entering otherwise
    /// surfaces a controller choice — is the Stage-4 cast-path wiring; Stage 1
    /// resolves the candidate set so the mechanism is exercised end-to-end.
    ///
    /// TODO(engine-attach Stage 2): fold in `Cant(Attach)` legality — restrict
    /// the candidate set to hosts where `attachment_legal(state, _, host)`
    /// holds, not just the `to` filter ([CR#303.4f]).
    fn enters_attached_host(&self, to: &Selection) -> Option<crate::object::ObjectId> {
        let filter = host_filter(to)?;
        crate::target::candidates(self, filter)
            .into_iter()
            .find(|&id| self.objects.obj(id).zone == Some(Zone::Battlefield))
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

/// Look through a remembered `Replacement` macro invocation (`AsEnters`, …) to
/// the form it expanded to.
fn look_through(replacement: &Replacement) -> &Replacement {
    match replacement {
        Replacement::Expanded(e) => look_through(&e.value),
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
    use deckmaste_core::CharacteristicFilter;
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
                condition: None,
                effects: vec![StaticEffect::Replacement(Replacement::Also {
                    would: Event::ZoneMove {
                        what: Filter::Ref(Reference::This),
                        from: None,
                        to: Some(Zone::Battlefield),
                        face: None,
                        cause: None,
                    },
                    also: Effect::Act(Action::Attach {
                        what: Selection::Ref(Reference::This),
                        to: Selection::Filter(Filter::AllOf(vec![
                            Filter::State(deckmaste_core::StateFilter::InZone(Zone::Battlefield)),
                            Filter::Characteristic(CharacteristicFilter::Type(Type::Creature)),
                        ])),
                    }),
                })],
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
}
