//! State-based actions ([CR#704]). Player losses: zero or less life
//! ([CR#704.5a]), drew from an empty library ([CR#704.5b]), ten or more
//! poison counters ([CR#704.5c]). A permanent with both +1/+1 and -1/-1
//! counters has the smaller count of each removed ([CR#704.5q]). Creatures
//! with lethal marked damage are destroyed ([CR#704.5g]); tokens stranded off
//! the battlefield cease to exist ([CR#704.5d]).

use deckmaste_core::Zone;

use crate::agenda::WorkItem;
use crate::event::GameEvent;
use crate::event::LossReason;
use crate::event::Occurrence;
use crate::object::ObjectId;
use crate::state::GameState;

/// One sweep ([CR#704.3]): the `PlayerLost`, `WillDestroy` (the replaceable
/// destruction intent), and `TokenCeased` events this check would perform. The
/// caller emits them and re-checks until a sweep comes back empty. A destroy's
/// LKI snapshot is captured later, at the will-change apply the `WillDestroy`
/// resolves into (the object is still live then), not here.
#[must_use]
pub fn sweep(state: &GameState) -> Vec<GameEvent> {
    let mut actions = Vec::new();
    let view = state.layers();

    // P0.W6 presence guard: an `OutcomeGate` row in the derived view must
    // suppress matching outcomes at each check — U5 semantics: precedence,
    // not consumption ([CR#101.2,704.3]); concession pierces it
    // ([CR#104.3a]). An unevaluated gate must not let a loss through (or a
    // win past "can't win") silently.
    if crate::legal::statics_present(state, &view, |s| {
        matches!(s, deckmaste_core::StaticEffect::OutcomeGate { .. })
    }) {
        todo!("P0.W6: outcome gates (suppress-per-check, [CR#101.1])");
    }

    let poison: deckmaste_core::Ident = "Poison".into();
    for player in &state.players {
        if player.lost {
            continue;
        }
        if player.life <= 0 {
            actions.push(GameEvent::PlayerLost {
                player: player.id,
                reason: LossReason::LifeZero,
            });
        } else if player.drew_from_empty {
            actions.push(GameEvent::PlayerLost {
                player: player.id,
                reason: LossReason::DrewFromEmpty,
            });
        } else if state
            .objects
            .obj(player.object)
            .counters
            .get(&poison)
            .is_some_and(|&n| n >= 10)
        {
            // [CR#704.5c]: player counters live on the player's PROXY
            // object ([CR#122.1] — counters go on objects and players; one
            // storage, never a parallel map), placed/removed by the
            // PutCounters/RemoveCounters apply arms. Two-Headed Giant swaps in
            // the fifteen-counter TEAM check ([CR#704.6b]) — variant-gated, not
            // built.
            actions.push(GameEvent::PlayerLost {
                player: player.id,
                reason: LossReason::Poison,
            });
        }
    }

    // [CR#704.5q]: the +1/+1 vs -1/-1 annihilation is data-driven now — the
    // `M1M1Counter` decl confers it as a `Property::StateBased` SBA, swept
    // generically here alongside every other counter-conferred state-based
    // action.
    actions.extend(counter_state_based_sbas(state));

    // [CR#704.3]: rules-defined SBAs from `rules/sba/` data (toughness-0,
    // loyalty-0, battle-defense-0). Evaluated after counter SBAs so they join
    // the same simultaneous batch.
    actions.extend(global_sba_rules(state));

    // [CR#704.5d,111.7]: a token in a zone other than the battlefield ceases
    // to exist. The move that stranded it already fired its zone-leave
    // triggers; this sweep just cleans up ([CR#111.7]'s note). Stack objects
    // are exempt: an activated/triggered ability minted from a token source
    // rides the token's `CardId` but is an ability, not the token. (The
    // [CR#111.8] stay-put rule — a token that left the battlefield can't
    // change zones again — is an unwired seam; the window between the move
    // and this sweep is currently unobservable.)
    for obj in state.objects.iter() {
        if matches!(
            obj.zone,
            Some(Zone::Graveyard | Zone::Exile | Zone::Hand | Zone::Library)
        ) && obj.card_id().is_some_and(|c| state.cards.get(c).is_token)
        {
            actions.push(GameEvent::TokenCeased(obj.id));
        }
    }

    // [CR#704.5e] SEAM: a copy of a spell in a zone other than the stack (or a
    // copy of a card outside stack/battlefield) ceases to exist. Unbuilt — no
    // copy representation exists yet (the layer-1 copy seam awaits
    // `core-copy-grammar` / `engine-copy-spells`); there is nothing to observe,
    // so this is intentionally not wired here.

    // Attachment SBAs ([CR#704.5m..704.5p]) — GENERIC, no subtype branch.
    actions.extend(attachment_sbas(state, &view));

    actions
}

/// The attachment state-based actions ([CR#704.5m..704.5p]) — extracted from
/// [`sweep`] but logically part of the same [CR#704.3] check. Two passes,
/// both keyed on conferred data + the `attached_to` relation only; NEVER on
/// the Aura/Equipment/Fortification subtype:
///
/// 1. **Firing `Sba { when, then }` statics.** The Aura graveyard rule
///    ([CR#704.5m]) is `Sba(Not(LegallyAttached(Ref(This))), Move(Ref(This),
///    Graveyard))`, conferred `Innate` by the Aura subtype. For each
///    battlefield object, for each `Sba` it carries (peeling `Innate`),
///    evaluate `when` with `This` = the object; if true, run `then`'s events.
///    Objects a firing `Sba` removes this sweep are tracked so pass 2 doesn't
///    double-handle them.
/// 2. **Generic illegal-attachment cleanup.** Any object attached to an illegal
///    host (per `attachment_legal`) that no firing `Sba` removed → becomes
///    unattached and stays ([CR#704.5n] Equipment/Fortification; [CR#704.5p]
///    creature / battle / other permanent — engine-identical).
fn attachment_sbas(state: &GameState, view: &crate::layer::LayeredView) -> Vec<GameEvent> {
    let mut out = Vec::new();
    let mut removed_by_sba: std::collections::BTreeSet<ObjectId> =
        std::collections::BTreeSet::new();

    // (1) Firing `Sba` statics.
    for &id in &state.zones.battlefield {
        // A `This`-anchored frame: `condition_holds`/`action_items` resolve
        // `Ref(This)` to this object via the frame source ([CR#603.10a]).
        let frame = crate::stack::Frame {
            source: id,
            controller: state.objects.obj(id).controller,
            targets: Vec::new(),
            bindings: None,
            chosen: None,
            x: None,
            it: None,
            that: None,
            allotment: None,
        };
        let mut rows: Vec<(deckmaste_core::Condition, deckmaste_core::Effect)> = Vec::new();
        crate::legal::for_each_static(view, id, |e| {
            if let deckmaste_core::StaticEffect::Sba { when, then } = e {
                rows.push((*when.clone(), (**then).clone()));
            }
        });
        for (when, then) in rows {
            if !state.condition_holds(&when, &frame) {
                continue;
            }
            // Run `then` — `Act(<Action>)` (the Aura's `Move`, the Saga
            // generalization's `Sacrifice`) or a `Sequence` of them.
            out.extend(run_sba_effect(state, &then, &frame));
            // This object is being moved/removed by its own SBA this sweep;
            // pass 2 must not also unattach it.
            removed_by_sba.insert(id);
        }
    }

    // (2) Generic illegal-attachment cleanup ([CR#704.5n,704.5p]).
    for &id in &state.zones.battlefield {
        if removed_by_sba.contains(&id) {
            continue;
        }
        if let Some(host) = state.objects.obj(id).attached_to
            && !crate::legal::attachment_legal(state, id, host)
        {
            // Becomes unattached, stays on the battlefield. The `attached_to`
            // clear happens at the `Unattached` apply (transition-only).
            out.push(GameEvent::Unattached {
                attachment: id,
                former_host: host,
            });
        }
    }

    out
}

/// Counter-conferred state-based actions ([CR#122.1,704.3]): for each
/// battlefield object, for each counter kind it holds, evaluate every
/// `Property::StateBased { condition, effect }` the counter confers (a
/// `This`-anchored frame resolves `Ref(This)` to the object) and run the
/// `effect` of those whose `condition` holds. The +1/+1 vs -1/-1 annihilation
/// ([CR#704.5q]) is the canonical instance — `M1M1Counter` confers it.
fn counter_state_based_sbas(state: &GameState) -> Vec<GameEvent> {
    let mut out = Vec::new();
    for &id in &state.zones.battlefield {
        let obj = state.objects.obj(id);
        if obj.counters.is_empty() {
            continue;
        }
        let frame = crate::stack::Frame {
            source: id,
            controller: obj.controller,
            targets: Vec::new(),
            bindings: None,
            chosen: None,
            x: None,
            it: None,
            that: None,
            allotment: None,
        };
        for kind in obj.counters.keys() {
            let Some(decl) = state.counter_decls.get(kind) else {
                continue;
            };
            for prop in &decl.confers {
                let deckmaste_core::Property::StateBased { condition, effect } = prop else {
                    continue;
                };
                if state.condition_holds(condition, &frame) {
                    out.extend(run_sba_effect(state, effect, &frame));
                }
            }
        }
    }
    out
}

/// Rules-defined state-based actions ([CR#704.3]): for every battlefield object
/// in a rule's `scope`, with `This` bound to that object, run the rule's `then`
/// if its `when` holds. The rules live in data (`rules/sba/`), so the engine
/// never branches on type here — each rule's `scope` is its binding domain and
/// `when` is its firing condition. Checking `scope` first means `when`'s stat
/// reads only run on in-scope objects (a toughness read never runs on a
/// non-creature).
fn global_sba_rules(state: &GameState) -> Vec<GameEvent> {
    let mut out = Vec::new();
    for &id in &state.zones.battlefield {
        let frame = crate::stack::Frame {
            source: id,
            controller: state.objects.obj(id).controller,
            targets: Vec::new(),
            bindings: None,
            chosen: None,
            x: None,
            it: None,
            that: None,
            allotment: None,
        };
        for rule in &state.sba_rules {
            // `scope` binds `This`: only objects in the rule's domain reach
            // `when` (so a toughness read never runs on a non-creature).
            if !crate::matches(state, id, &rule.scope) {
                continue;
            }
            if state.condition_holds(&rule.when, &frame) {
                for mut ev in run_sba_effect(state, &rule.then, &frame) {
                    stamp_sba_cause(&mut ev);
                    out.push(ev);
                }
            }
        }
    }
    out
}

/// Re-attribute a rules-SBA-emitted event to the state-based action itself
/// ([CR#704]): a state-based action has no agent — it is the game performing
/// the action, not an effect or the object. Preserves the cause's verb when
/// one already exists (e.g. the `WillDestroy` "Destroy" verb stays). For
/// events whose `cause` is `None` (plain `Move` actions), upserts a
/// `StateBasedAction` cause so no rules-SBA event goes unattributed.
fn stamp_sba_cause(ev: &mut GameEvent) {
    let cause_slot = match ev {
        GameEvent::WillDestroy { cause, .. } | GameEvent::ZoneWillChange { cause, .. } => {
            Some(cause)
        }
        _ => None,
    };
    let Some(cause_opt) = cause_slot else {
        return;
    };
    match cause_opt {
        Some(cause) => {
            // Preserve the verb; only correct the agency and clear the agent.
            cause.agency = deckmaste_core::Agency::StateBasedAction;
            cause.agent = None;
        }
        None => {
            // No verb yet (plain Move has no cause-verb fact); upsert a
            // generic SBA cause so the event is attributed ([CR#704]).
            *cause_opt = Some(crate::event::Cause {
                verb: "Move".into(),
                agency: deckmaste_core::Agency::StateBasedAction,
                agent: None,
            });
        }
    }
}

/// Run an SBA `then`/`effect` purely (no apply) into the events it produces:
/// `Act(<Action>)`, or a `Sequence` of effects (each evaluated against the SAME
/// pre-sweep state — so the annihilation's two `RemoveCounters` both read the
/// pre-removal counts). Choice-bearing shapes are a documented seam.
fn run_sba_effect(
    state: &GameState,
    effect: &deckmaste_core::Effect,
    frame: &crate::stack::Frame,
) -> Vec<GameEvent> {
    use deckmaste_core::Effect;

    let mut out = Vec::new();
    match effect {
        Effect::Act(action) => {
            for item in state.action_items(action, frame) {
                if let WorkItem::Emit(occ) = item {
                    match occ {
                        Occurrence::Single(ev) => out.push(ev),
                        Occurrence::Batch(evs) => out.extend(evs),
                    }
                }
            }
        }
        Effect::Sequence(children) => {
            for child in children {
                out.extend(run_sba_effect(state, child, frame));
            }
        }
        Effect::Expanded(e) => out.extend(run_sba_effect(state, &e.value, frame)),
        other => todo!("SBA effect is only Act/Sequence in this stage (got {other:?})"),
    }
    out
}

/// [CR#704.5j] legend-rule groups: per controller, battlefield legendary
/// permanents grouped by printed name, keeping groups of size ≥ 2. Ordered
/// active player first ([CR#101.4] APNAP), then by name, for a stable choice
/// order.
pub(crate) fn legend_rule_groups(
    state: &GameState,
) -> Vec<(crate::player::PlayerId, Vec<ObjectId>)> {
    use std::collections::BTreeMap;

    let view = state.layers();
    // controller → name → [ids]
    let mut by_player: BTreeMap<crate::player::PlayerId, BTreeMap<String, Vec<ObjectId>>> =
        BTreeMap::new();
    for &id in &state.zones.battlefield {
        if !view
            .get(id)
            .supertypes
            .contains(&deckmaste_core::Supertype::Legendary)
        {
            continue;
        }
        let controller = state.objects.obj(id).controller;
        let name = crate::derive::face(state.def(id)).name.clone();
        by_player
            .entry(controller)
            .or_default()
            .entry(name)
            .or_default()
            .push(id);
    }
    let mut groups = Vec::new();
    for (player, names) in by_player {
        for (_name, ids) in names {
            if ids.len() >= 2 {
                groups.push((player, ids));
            }
        }
    }
    // Active player's groups first ([CR#101.4] APNAP).
    let active = state.turn.active_player;
    groups.sort_by_key(|(p, _)| *p != active);
    groups
}

#[cfg(test)]
mod tests {
    // `too_many_lines` is exempted for this test module: e2e scenarios read
    // better whole than split into helpers.
    #![allow(clippy::too_many_lines)]

    use std::path::Path;
    use std::sync::Arc;

    use deckmaste_cards::plugin::Plugin;
    use deckmaste_core::Card;
    use deckmaste_core::Filter;
    use deckmaste_core::Type;
    use deckmaste_core::Zone;

    use crate::agenda::WorkItem;
    use crate::event::GameEvent;
    use crate::event::Occurrence;
    use crate::matches as obj_matches;
    use crate::object::ObjectSource;
    use crate::player::PlayerId;
    use crate::sba;
    use crate::state::GameConfig;
    use crate::state::GameOutcome;
    use crate::state::GameState;
    use crate::state::PlayerConfig;
    use crate::state::StartingPlayer;
    use crate::step::StepOutcome;

    fn builtin() -> Plugin {
        Plugin::load(Path::new(env!("CARGO_MANIFEST_DIR")).join("../../plugins/builtin")).unwrap()
    }

    fn canon() -> Plugin {
        Plugin::load_with_sibling_prelude(
            Path::new(env!("CARGO_MANIFEST_DIR")).join("../../plugins/canon"),
        )
        .unwrap()
    }

    /// The graduated WIZARDS corpus, loaded over its `builtin` sibling prelude
    /// (same path real wizards cards load through). Proves the Aura/Equipment/
    /// Fortification `confers:` reach a wizards card: the defs live in builtin,
    /// and the generator emits no confers-less wizards stub to shadow them.
    fn wizards() -> Plugin {
        Plugin::load_with_sibling_prelude(
            Path::new(env!("CARGO_MANIFEST_DIR")).join("../../plugins/wizards"),
        )
        .unwrap()
    }

    fn deck(card: &Arc<Card>, n: usize) -> Vec<Arc<Card>> {
        vec![Arc::clone(card); n]
    }

    /// A two-player game; player 0's deck is Grizzly Bears.
    /// Returns the state plus a creature object forced onto the battlefield.
    fn bear_on_field() -> (GameState, crate::object::ObjectId) {
        let bears = Arc::new(canon().card("Grizzly Bears").unwrap());
        let forest = Arc::new(builtin().card("Forest").unwrap());
        let mut state = GameState::new(GameConfig {
            players: vec![
                PlayerConfig {
                    deck: deck(&bears, 10),
                },
                PlayerConfig {
                    deck: deck(&forest, 10),
                },
            ],
            seed: 1,
            starting_life: 20,
            starting_player: StartingPlayer::Fixed(PlayerId(0)),
            sba_rules: vec![],
            counter_decls: std::collections::HashMap::new(),
            subtypes: std::collections::HashMap::new(),
        });
        // Force a Grizzly Bears from player 0's hand onto the battlefield.
        let bear = *state.zones.hands[0]
            .iter()
            .find(|&&o| {
                obj_matches(
                    &state,
                    o,
                    &Filter::Characteristic(deckmaste_core::CharacteristicFilter::Type(
                        Type::Creature,
                    )),
                )
            })
            .expect("a Grizzly Bears in the opening hand");
        state.zones.hands[PlayerId(0).index()].retain(|&o| o != bear);
        state.objects.obj_mut(bear).zone = Some(Zone::Battlefield);
        state.zones.battlefield.push(bear);
        (state, bear)
    }

    /// Player 0's deck = Darksteel Myr (indestructible 0/1), one on the field.
    fn myr_on_field() -> (GameState, crate::object::ObjectId) {
        let myr = Arc::new(canon().card("Darksteel Myr").unwrap());
        let forest = Arc::new(builtin().card("Forest").unwrap());
        let mut state = GameState::new(GameConfig {
            players: vec![
                PlayerConfig {
                    deck: deck(&myr, 10),
                },
                PlayerConfig {
                    deck: deck(&forest, 10),
                },
            ],
            seed: 1,
            starting_life: 20,
            starting_player: StartingPlayer::Fixed(PlayerId(0)),
            sba_rules: vec![],
            counter_decls: std::collections::HashMap::new(),
            subtypes: std::collections::HashMap::new(),
        });
        let m = *state.zones.hands[0]
            .iter()
            .find(|&&o| {
                obj_matches(
                    &state,
                    o,
                    &Filter::Characteristic(deckmaste_core::CharacteristicFilter::Type(
                        Type::Creature,
                    )),
                )
            })
            .expect("a Darksteel Myr in the opening hand");
        state.zones.hands[PlayerId(0).index()].retain(|&o| o != m);
        state.objects.obj_mut(m).zone = Some(Zone::Battlefield);
        state.zones.battlefield.push(m);
        (state, m)
    }

    /// End-to-end through the WIZARDS load path ([CR#704.5m]): a graduated
    /// wizards Aura (Angelic Gift) carries the Aura subtype's `Innate`
    /// graveyard `Sba` *via the data*, not in-Rust scaffolding. Loaded over the
    /// builtin sibling prelude, put on the battlefield UNATTACHED, the generic
    /// SBA sweep fires its battlefield→graveyard move. This is the regression
    /// the fix targets: the Aura `confers:` lives in builtin and the generator
    /// emits no confers-less wizards stub to shadow it, so a fresh wizards card
    /// inherits the attachment rule.
    #[test]
    fn wizards_aura_carries_innate_graveyard_sba() {
        let gift = Arc::new(wizards().card("Angelic Gift").unwrap());
        // Sanity: the loaded card actually carries the Aura subtype's confer.
        // (`derive::printed_of_face` is what flattens it onto the object.)
        let face = crate::derive::face(&gift);
        assert!(
            face.subtypes
                .iter()
                .any(|s| s.confers.iter().any(|p| matches!(
                    p,
                    deckmaste_core::Property::Ability(a) if a.is_innate()
                ))),
            "the wizards Aura card embeds the Innate confer; subtypes: {:?}",
            face.subtypes
        );

        let mut state = GameState::new(GameConfig {
            players: vec![PlayerConfig { deck: vec![] }, PlayerConfig { deck: vec![] }],
            seed: 1,
            starting_life: 20,
            starting_player: StartingPlayer::Fixed(PlayerId(0)),
            sba_rules: vec![],
            counter_decls: std::collections::HashMap::new(),
            subtypes: std::collections::HashMap::new(),
        });
        let card_id = state.cards.push(gift, PlayerId(0));
        let aura = state.objects.mint(
            ObjectSource::Card(card_id),
            PlayerId(0),
            Some(Zone::Battlefield),
        );
        state.zones.battlefield.push(aura);

        // Unattached → `LegallyAttached` is false → the conferred Innate SBA
        // fires, moving the Aura to its owner's graveyard.
        let actions = sba::sweep(&state);
        assert!(
            actions.iter().any(|e| matches!(e,
                GameEvent::ZoneWillChange { object, to: Zone::Graveyard, .. } if *object == aura)),
            "a graduated wizards Aura's Innate graveyard SBA fires when unattached \
             ([CR#704.5m]); got {actions:?}"
        );
    }

    /// [CR#704.5g,702.12b]: an indestructible creature with lethal damage is
    /// NOT destroyed by the SBA — the sweep emits a `WillDestroy`, and the
    /// event-side cant pass ([CR#614.17]) in `apply_occurrence` suppresses it
    /// before `apply` runs. The Myr stays on the battlefield.
    #[test]
    fn indestructible_survives_lethal_damage() {
        let (mut state, myr) = myr_on_field();
        // Load builtin rules so the lethal-damage SBA fires via the rule path.
        state.sba_rules = builtin().sba_rules;
        state.objects.obj_mut(myr).damage = 1; // toughness 1 → lethal
        let actions = sba::sweep(&state);
        state.schedule_front(vec![WorkItem::Emit(Occurrence::Batch(actions))]);
        let _ = state.step(); // WillDestroy applies → replaced to nothing
        assert!(
            state.objects.get(myr).is_some(),
            "indestructible survives lethal damage"
        );
        assert!(state.zones.battlefield.contains(&myr));
        assert!(state.zones.graveyards[0].is_empty(), "not destroyed");
    }

    #[test]
    fn lethal_damage_destroys_a_creature_in_the_sba_sweep() {
        let (mut state, bear) = bear_on_field();
        // Load builtin rules so the lethal-damage SBA fires via the rule path.
        state.sba_rules = builtin().sba_rules;

        // Grizzly Bears has toughness 2; set lethal damage. The sweep emits
        // the destroy as a replaceable `WillDestroy` intent (its apply commits
        // the battlefield→graveyard move when nothing replaces it), cause-tagged
        // as the SBA destruction verb ([CR#701.8b]).
        state.objects.obj_mut(bear).damage = 2;
        let actions = sba::sweep(&state);
        assert!(
            actions.iter().any(|e| matches!(
                e,
                GameEvent::WillDestroy {
                    object,
                    cause: Some(c),
                } if *object == bear
                    && c.verb == deckmaste_core::Ident::from("Destroy")
                    && c.agency == deckmaste_core::Agency::StateBasedAction
            )),
            "sweep should include a WillDestroy for Grizzly Bears at lethal damage"
        );

        // Sublethal: damage = 1 < toughness 2.
        state.objects.obj_mut(bear).damage = 1;
        let actions = sba::sweep(&state);
        assert!(
            actions
                .iter()
                .all(|e| !matches!(e, GameEvent::WillDestroy { .. })),
            "sweep should NOT include a destroy for Grizzly Bears at sublethal damage"
        );
    }

    /// [CR#704.5q,122.3]: a permanent with both +1/+1 and -1/-1 counters has N
    /// of each removed as a state-based action, where N is the smaller count.
    /// The sweep emits a `CounterRemoved` per kind, cause-tagged as the SBA.
    #[test]
    fn plus_and_minus_counters_annihilate_in_the_sweep() {
        let (mut state, bear) = bear_on_field();
        state.counter_decls = builtin().counters;
        state
            .objects
            .obj_mut(bear)
            .counters
            .insert("P1P1Counter".into(), 3);
        state
            .objects
            .obj_mut(bear)
            .counters
            .insert("M1M1Counter".into(), 2);
        let actions = sba::sweep(&state);
        // [CR#704.5q]: N = min(3, 2) = 2 of EACH kind removed — data-driven via
        // M1M1Counter's conferred `StateBased` annihilation, both legs taking
        // `Min(count P1P1, count M1M1)`.
        for kind in ["P1P1Counter", "M1M1Counter"] {
            assert!(
                actions.iter().any(|e| matches!(e,
                    GameEvent::CounterRemoved { object, kind: k, count: 2, .. }
                    if *object == bear && *k == deckmaste_core::Ident::from(kind))),
                "removes 2 {kind}; got {actions:?}"
            );
        }
    }

    /// [CR#704.5q]: a permanent with only one of the two kinds is untouched —
    /// no annihilation.
    #[test]
    fn one_sided_counters_do_not_annihilate() {
        let (mut state, bear) = bear_on_field();
        state.counter_decls = builtin().counters;
        state
            .objects
            .obj_mut(bear)
            .counters
            .insert("P1P1Counter".into(), 3);
        let actions = sba::sweep(&state);
        assert!(
            actions
                .iter()
                .all(|e| !matches!(e, GameEvent::CounterRemoved { .. })),
            "no annihilation without both kinds; got {actions:?}"
        );
    }

    /// [CR#704.5q] e2e: after the sweep applies, the permanent keeps the
    /// surplus of the larger kind and none of the smaller (3 +1/+1 & 2 -1/-1
    /// → 1 +1/+1, no -1/-1).
    #[test]
    fn annihilation_leaves_the_surplus() {
        let (mut state, bear) = bear_on_field();
        state.counter_decls = builtin().counters;
        state
            .objects
            .obj_mut(bear)
            .counters
            .insert("P1P1Counter".into(), 3);
        state
            .objects
            .obj_mut(bear)
            .counters
            .insert("M1M1Counter".into(), 2);
        let actions = sba::sweep(&state);
        state.schedule_front(vec![WorkItem::Emit(Occurrence::Batch(actions))]);
        let _ = state.step(); // both CounterRemoved apply atomically
        assert_eq!(
            state
                .objects
                .obj(bear)
                .counters
                .get(&deckmaste_core::Ident::from("P1P1Counter"))
                .copied(),
            Some(1),
            "one P1P1Counter survives"
        );
        assert!(
            !state
                .objects
                .obj(bear)
                .counters
                .contains_key(&deckmaste_core::Ident::from("M1M1Counter")),
            "M1M1Counter fully annihilated"
        );
    }

    #[test]
    fn players_not_on_battlefield_do_not_trigger_704_5g() {
        let (state, _) = bear_on_field();
        let proxy = state.players[0].object;
        // Player proxy should never have source Card(...), so def() would
        // panic — the sweep guards against this by only scanning the
        // battlefield (which never contains player proxies).
        // Just confirm: the proxy's source is Player, not on battlefield.
        assert!(matches!(
            state.objects.obj(proxy).source,
            ObjectSource::Player(_)
        ));
        assert!(!state.zones.battlefield.contains(&proxy));
    }

    /// [CR#104.4a,704.3]: two players at ≤0 life in the same sweep → Draw, not
    /// a Win for whoever was checked first.
    #[test]
    fn simultaneous_double_loss_is_a_draw() {
        let forest = Arc::new(builtin().card("Forest").unwrap());
        let mut state = GameState::new(GameConfig {
            players: vec![
                PlayerConfig {
                    deck: vec![Arc::clone(&forest); 10],
                },
                PlayerConfig {
                    deck: vec![Arc::clone(&forest); 10],
                },
            ],
            seed: 1,
            starting_life: 20,
            starting_player: StartingPlayer::Fixed(PlayerId(0)),
            sba_rules: vec![],
            counter_decls: std::collections::HashMap::new(),
            subtypes: std::collections::HashMap::new(),
        });
        state.players[0].life = 0;
        state.players[1].life = 0;
        state.schedule_front(vec![WorkItem::CheckSbas]);
        loop {
            if let StepOutcome::GameOver(o) = state.step() {
                assert_eq!(o, GameOutcome::Draw);
                return;
            }
        }
    }

    /// [CR#704.5d,111.7]: a token put into a graveyard is removed from the
    /// game by the next SBA sweep — the graveyard empties and the object is
    /// gone from the store, with no `ZoneChanged` fact (ceasing to exist is
    /// not a move). A token still on the battlefield never ceases.
    #[test]
    fn dead_token_ceases_to_exist() {
        use deckmaste_core::Action;
        use deckmaste_core::Count;
        use deckmaste_core::Effect;
        use deckmaste_core::PlayerAction;
        use deckmaste_core::Reference;
        use deckmaste_core::Token;

        let (mut state, src) = bear_on_field();
        let frame = crate::stack::Frame {
            source: src,
            controller: PlayerId(0),
            targets: vec![],
            bindings: None,
            chosen: None,
            x: None,
            it: None,
            that: None,
            allotment: None,
        };
        let token = Token {
            color_indicator: vec![],
            supertypes: vec![],
            types: vec![Type::Artifact],
            subtypes: vec![],
            abilities: vec![],
            power: None,
            toughness: None,
        };
        state.run_effect(
            Effect::Act(Action::By(
                Reference::You,
                PlayerAction::Create(Count::Literal(1), token.into()),
            )),
            &frame,
        );
        let _ = state.step(); // TokenCreated applies
        let _ = state.step(); // its ZoneChanged fact
        let &token_obj = state
            .zones
            .battlefield
            .iter()
            .find(|&&id| id != src)
            .expect("the token on the battlefield");

        // On the battlefield the token is exempt.
        assert!(
            sba::sweep(&state)
                .iter()
                .all(|e| !matches!(e, GameEvent::TokenCeased(_))),
            "a battlefield token must not cease"
        );

        // Put it into the graveyard (the generic move: remint + LKI).
        state.schedule_front(vec![WorkItem::Emit(Occurrence::single(
            GameEvent::ZoneWillChange {
                object: token_obj,
                from: Some(Zone::Battlefield),
                to: Zone::Graveyard,
                enters: None,
                position: None,
                face: None,
                cause: None,
            },
        ))]);
        let _ = state.step(); // the move applies
        let _ = state.step(); // its ZoneChanged fact
        let dead = state.zones.graveyards[0][0];

        // The sweep emits exactly one TokenCeased for the reminted object.
        let actions = sba::sweep(&state);
        assert_eq!(actions, vec![GameEvent::TokenCeased(dead)]);

        // Applying it removes the object outright — graveyard empty, id gone.
        state.schedule_front(vec![WorkItem::Emit(Occurrence::Batch(actions))]);
        let _ = state.step();
        assert!(state.zones.graveyards[0].is_empty(), "[CR#704.5d]");
        assert!(
            state.objects.get(dead).is_none(),
            "the ceased token's id must be gone from the store"
        );
    }

    /// [CR#400.7]: when a creature is destroyed, the old `ObjectId` is removed
    /// from the store entirely, and a fresh `ObjectId` is minted in the owner's
    /// graveyard. The `LkiSnapshot` rides the event.
    #[test]
    fn destroy_remints_old_id_gone_new_in_graveyard() {
        let (mut state, bear) = bear_on_field();
        // Load builtin rules so the lethal-damage SBA fires.
        state.sba_rules = builtin().sba_rules;
        // Grizzly Bears has toughness 2; set lethal damage.
        state.objects.obj_mut(bear).damage = 2;
        let actions = sba::sweep(&state);
        state.schedule_front(vec![WorkItem::Emit(Occurrence::Batch(actions))]);
        // WillDestroy applies (nothing replaces it) → ZoneWillChange remints.
        let _ = state.step();
        let _ = state.step();
        assert!(
            state.objects.get(bear).is_none(),
            "old battlefield id must be gone from the object store"
        );
        assert!(
            !state.zones.battlefield.contains(&bear),
            "old id must not remain on the battlefield"
        );
        assert_eq!(
            state.zones.graveyards[0].len(),
            1,
            "owner's graveyard must contain exactly one object"
        );
        let new = state.zones.graveyards[0][0];
        assert_ne!(new, bear, "graveyard object must have a fresh ObjectId");
    }

    // --- Attachment SBAs ([CR#704.5m..704.5p]) ---------------------------------

    use deckmaste_core::Ability;
    use deckmaste_core::CardFace;
    use deckmaste_core::CharacteristicFilter;
    use deckmaste_core::Condition;
    use deckmaste_core::Deontic;
    use deckmaste_core::DeonticAction;
    use deckmaste_core::Effect;
    use deckmaste_core::Reference;
    use deckmaste_core::StaticAbility;
    use deckmaste_core::StaticEffect;

    fn game() -> GameState {
        GameState::new(GameConfig {
            players: vec![PlayerConfig { deck: vec![] }, PlayerConfig { deck: vec![] }],
            seed: 1,
            starting_life: 20,
            starting_player: StartingPlayer::Fixed(PlayerId(0)),
            sba_rules: vec![],
            counter_decls: std::collections::HashMap::new(),
            subtypes: std::collections::HashMap::new(),
        })
    }

    fn on_field(
        state: &mut GameState,
        name: &str,
        types: Vec<Type>,
        abilities: Vec<Ability>,
    ) -> crate::object::ObjectId {
        let card = Card::Normal(CardFace {
            name: name.into(),
            types,
            abilities,
            ..CardFace::default()
        });
        let card_id = state.cards.push(Arc::new(card), PlayerId(0));
        let id = state.objects.mint(
            ObjectSource::Card(card_id),
            PlayerId(0),
            Some(Zone::Battlefield),
        );
        state.zones.battlefield.push(id);
        id
    }

    /// The Aura-subtype shape (scaffolded in-Rust): `Innate(Static([Sba(Not(
    /// LegallyAttached(Ref(This))), Move(Ref(This), Graveyard))]))`.
    fn aura_graveyard_sba() -> Ability {
        Ability::Innate(Box::new(Ability::Static(StaticAbility {
            from: None,
            condition: None,
            effects: vec![StaticEffect::Sba {
                when: Box::new(Condition::Not(Box::new(Condition::LegallyAttached(
                    Reference::This,
                )))),
                then: Box::new(Effect::Act(deckmaste_core::Action::move_to(
                    Reference::This,
                    Zone::Graveyard,
                ))),
            }],
            characteristic_defining: false,
        })))
    }

    /// The Equipment-subtype shape: `Innate(Static([Cant(Attach(what:
    /// Ref(This), to: Not(Creature)))]))`.
    fn equipment_host_rule() -> Ability {
        Ability::Innate(Box::new(Ability::Static(StaticAbility {
            from: None,
            condition: None,
            effects: vec![StaticEffect::Deontic(Deontic::Cant(
                DeonticAction::Attach {
                    what: Filter::Ref(Reference::This),
                    to: Filter::Not(Box::new(Filter::Characteristic(
                        CharacteristicFilter::Type(Type::Creature),
                    ))),
                },
            ))],
            characteristic_defining: false,
        })))
    }

    /// [CR#704.5m]: an Aura (carrying the Innate graveyard `Sba`) that is
    /// UNATTACHED fires the SBA → a `ZoneWillChange(Battlefield → Graveyard)`
    /// for it. Generic — driven by the `Sba` static, not the subtype.
    #[test]
    fn sba_attach_unattached_aura_goes_to_graveyard() {
        let mut state = game();
        let aura = on_field(
            &mut state,
            "Test Aura",
            vec![Type::Enchantment],
            vec![aura_graveyard_sba()],
        );
        // It is unattached → `LegallyAttached` is false → the SBA fires.
        let actions = sba::sweep(&state);
        assert!(
            actions.iter().any(|e| matches!(e,
                GameEvent::ZoneWillChange { object, to: Zone::Graveyard, .. } if *object == aura)),
            "unattached Aura is moved to the graveyard ([CR#704.5m]); got {actions:?}"
        );
    }

    /// Lock a `LoseAllAbilities` (layer-6, end-of-game) continuous effect onto
    /// `id` — strips its normal/granted abilities, but NOT its `Innate` rules.
    fn lose_all_abilities(state: &mut GameState, id: crate::object::ObjectId) {
        use deckmaste_core::Duration;
        use deckmaste_core::Modification;

        use crate::layer::ContinuousEffect;
        use crate::layer::ScopeResolved;

        let timestamp = state.objects.next_timestamp();
        state.continuous.push(ContinuousEffect {
            timestamp,
            controller: PlayerId(0),
            scope: ScopeResolved::Locked(vec![id]),
            changes: vec![Modification::LoseAllAbilities],
            duration: Duration::EndOfGame,
            is_cda: false,
        });
    }

    /// Review #7 e2e — the whole point of making the Aura graveyard rule an
    /// `Innate` SBA ([CR#113.12,704.5m]): an Aura whose normal abilities are
    /// ALL stripped by an active `LoseAllAbilities` and is UNATTACHED still
    /// goes to the graveyard. The `Innate(Sba(...))` survives ability
    /// removal (layer-6 retain), so the sweep still fires it — emitting the
    /// battlefield→graveyard move, and (driven to completion) landing the
    /// reminted object in its owner's graveyard.
    #[test]
    fn ability_less_aura_still_graveyards() {
        let mut state = game();
        let aura = on_field(
            &mut state,
            "Test Aura",
            vec![Type::Enchantment],
            vec![aura_graveyard_sba()],
        );
        // Strip ALL of the Aura's abilities. The Innate SBA must survive.
        lose_all_abilities(&mut state, aura);

        // The sweep STILL emits the graveyard move for the unattached Aura.
        let actions = sba::sweep(&state);
        assert!(
            actions.iter().any(|e| matches!(e,
                GameEvent::ZoneWillChange { object, to: Zone::Graveyard, .. } if *object == aura)),
            "Innate graveyard SBA survives LoseAllAbilities ([CR#113.12,704.5m]); got {actions:?}"
        );

        // Drive it to completion: the Aura ends up in its owner's graveyard.
        state.schedule_front(vec![WorkItem::Emit(Occurrence::Batch(actions))]);
        let _ = state.step(); // the move applies (remint + LKI)
        let _ = state.step(); // its ZoneChanged fact
        assert!(
            state.objects.get(aura).is_none(),
            "old battlefield id is gone after the move"
        );
        assert_eq!(
            state.zones.graveyards[0].len(),
            1,
            "the ability-less Aura landed in its owner's graveyard"
        );
    }

    /// [CR#704.5m]: an Aura legally attached to a creature does NOT fire its
    /// graveyard SBA.
    #[test]
    fn sba_attach_legally_attached_aura_stays() {
        let mut state = game();
        let aura = on_field(
            &mut state,
            "Test Aura",
            vec![Type::Enchantment],
            vec![aura_graveyard_sba()],
        );
        let host = on_field(&mut state, "Bear", vec![Type::Creature], vec![]);
        state.objects.obj_mut(aura).attached_to = Some(host);
        let actions = sba::sweep(&state);
        assert!(
            !actions
                .iter()
                .any(|e| matches!(e, GameEvent::ZoneWillChange { object, .. } if *object == aura)),
            "legally-attached Aura stays put; got {actions:?}"
        );
    }

    /// [CR#704.5n]: an Equipment (no firing `Sba`) attached to an ILLEGAL host
    /// (a non-creature) becomes unattached and stays — the generic
    /// illegal-attachment cleanup, NO subtype branch.
    #[test]
    fn sba_attach_illegal_equipment_unattaches() {
        let mut state = game();
        let equip = on_field(
            &mut state,
            "Test Equipment",
            vec![Type::Artifact],
            vec![equipment_host_rule()],
        );
        // Illegally attached to a non-creature artifact.
        let rock = on_field(&mut state, "Rock", vec![Type::Artifact], vec![]);
        state.objects.obj_mut(equip).attached_to = Some(rock);

        let actions = sba::sweep(&state);
        assert!(
            actions.iter().any(|e| matches!(e,
                GameEvent::Unattached { attachment, former_host }
                if *attachment == equip && *former_host == rock)),
            "illegally-attached Equipment becomes unattached ([CR#704.5n]); got {actions:?}"
        );
        // It does NOT go to the graveyard (no firing Sba).
        assert!(
            !actions
                .iter()
                .any(|e| matches!(e, GameEvent::ZoneWillChange { object, .. } if *object == equip)),
            "Equipment stays on the battlefield, not graveyard"
        );
    }

    /// [CR#704.5p]: a plain permanent with `attached_to` set to an illegal host
    /// and NO `Sba` → becomes unattached (engine-identical to [CR#704.5n]).
    #[test]
    fn sba_attach_plain_permanent_illegal_link_unattaches() {
        let mut state = game();
        // A plain artifact with no attachment rules at all, illegally linked.
        let thing = on_field(&mut state, "Thing", vec![Type::Artifact], vec![]);
        let host = on_field(&mut state, "Bear", vec![Type::Creature], vec![]);
        // Give the host a protection-shaped host-side Cant so the link is
        // illegal even though `thing` itself carries no restriction.
        let protected = on_field(
            &mut state,
            "Protected",
            vec![Type::Creature],
            vec![Ability::Static(StaticAbility {
                from: None,
                condition: None,
                effects: vec![StaticEffect::Deontic(Deontic::Cant(
                    DeonticAction::Attach {
                        what: Filter::Any,
                        to: Filter::Ref(Reference::This),
                    },
                ))],
                characteristic_defining: false,
            })],
        );
        let _ = host;
        state.objects.obj_mut(thing).attached_to = Some(protected);

        let actions = sba::sweep(&state);
        assert!(
            actions.iter().any(|e| matches!(e,
                GameEvent::Unattached { attachment, former_host }
                if *attachment == thing && *former_host == protected)),
            "plain permanent on an illegal host becomes unattached ([CR#704.5p]); got {actions:?}"
        );
    }

    // --- Ascend (permanent form) e2e ([CR#702.131b,702.131c]) ------------------

    /// [CR#702.131b]: the Ascend static grants the city's blessing once the
    /// controller has ten permanents, exactly once (idempotent / no sweep
    /// loop), and not at nine.
    #[test]
    fn ascend_permanent_grants_citys_blessing_at_ten() {
        use deckmaste_core::Action;
        use deckmaste_core::Cmp;
        use deckmaste_core::Count;
        use deckmaste_core::PlayerAction;
        use deckmaste_core::RelationFilter;
        use deckmaste_core::StateFilter;

        let mut state = game();
        let name: deckmaste_core::Ident = "CitysBlessing".into();
        let p0 = PlayerId(0);

        // The Ascend static, built typed (mirrors the builtin macro's expansion).
        let gate = Condition::AllOf(vec![
            Condition::Compare(
                Count::CountOf(Box::new(Filter::AllOf(vec![
                    Filter::State(StateFilter::InZone(Zone::Battlefield)),
                    Filter::Relation(RelationFilter::ControlledBy(Box::new(Filter::Ref(
                        Reference::You,
                    )))),
                ]))),
                Cmp::AtLeast,
                Count::Literal(10),
            ),
            Condition::Not(Box::new(Condition::Is(
                Reference::You,
                Filter::State(StateFilter::Designated(name)),
            ))),
        ]);
        let ascend = Ability::Static(StaticAbility {
            from: None,
            condition: None,
            effects: vec![StaticEffect::Sba {
                when: Box::new(gate),
                then: Box::new(Effect::Act(Action::By(
                    Reference::You,
                    PlayerAction::GetDesignation(name),
                ))),
            }],
            characteristic_defining: false,
        });
        let _ascender = on_field(
            &mut state,
            "Ascender",
            vec![Type::Enchantment],
            vec![ascend],
        );

        // Nine permanents (the ascender + 8 fillers) → no grant.
        for i in 0..8 {
            on_field(
                &mut state,
                &format!("Filler{i}"),
                vec![Type::Artifact],
                vec![],
            );
        }
        assert_eq!(state.zones.battlefield.len(), 9);
        assert!(
            sba::sweep(&state)
                .iter()
                .all(|e| !matches!(e, GameEvent::GotDesignation { .. })),
            "no blessing at nine permanents"
        );

        // Tenth permanent → the sweep emits the grant for p0.
        on_field(&mut state, "Filler8", vec![Type::Artifact], vec![]);
        let actions = sba::sweep(&state);
        assert!(
            actions.iter().any(|e| matches!(e,
                GameEvent::GotDesignation { player, name: n } if *player == p0 && *n == name)),
            "blessing granted at ten permanents; got {actions:?}"
        );

        // Apply it; the store holds it and a re-sweep emits nothing (no loop).
        state.schedule_front(vec![WorkItem::Emit(Occurrence::Batch(actions))]);
        let _ = state.step();
        assert!(state.designations.players.contains_key(&(p0, name)));
        assert!(
            sba::sweep(&state)
                .iter()
                .all(|e| !matches!(e, GameEvent::GotDesignation { .. })),
            "already-held: the Not(Designated) guard stops re-granting"
        );
    }

    /// [CR#702.131c]: the city's blessing is a per-player designation — more
    /// than one player can hold it at once. Two players, each controlling ten
    /// permanents (each with their own Ascend static), both acquire it in a
    /// single sweep.
    #[test]
    fn citys_blessing_is_multi_holder() {
        use deckmaste_core::Action;
        use deckmaste_core::Cmp;
        use deckmaste_core::Count;
        use deckmaste_core::PlayerAction;
        use deckmaste_core::RelationFilter;
        use deckmaste_core::StateFilter;

        let name: deckmaste_core::Ident = "CitysBlessing".into();
        let p0 = PlayerId(0);
        let p1 = PlayerId(1);

        // The Ascend static, built typed (mirrors the builtin macro's
        // expansion). `ControlledBy(Ref(You))` resolves `You` to the carrying
        // object's controller via the Sba frame, so each ascender counts ITS
        // controller's permanents and grants to that controller.
        let ascend = || {
            Ability::Static(StaticAbility {
                from: None,
                condition: None,
                effects: vec![StaticEffect::Sba {
                    when: Box::new(Condition::AllOf(vec![
                        Condition::Compare(
                            Count::CountOf(Box::new(Filter::AllOf(vec![
                                Filter::State(StateFilter::InZone(Zone::Battlefield)),
                                Filter::Relation(RelationFilter::ControlledBy(Box::new(
                                    Filter::Ref(Reference::You),
                                ))),
                            ]))),
                            Cmp::AtLeast,
                            Count::Literal(10),
                        ),
                        Condition::Not(Box::new(Condition::Is(
                            Reference::You,
                            Filter::State(StateFilter::Designated(name)),
                        ))),
                    ])),
                    then: Box::new(Effect::Act(Action::By(
                        Reference::You,
                        PlayerAction::GetDesignation(name),
                    ))),
                }],
                characteristic_defining: false,
            })
        };

        let mut state = game();

        // p0: ascender + 9 fillers, all controlled by p0 (on_field default).
        on_field(
            &mut state,
            "Ascender0",
            vec![Type::Enchantment],
            vec![ascend()],
        );
        for i in 0..9 {
            on_field(
                &mut state,
                &format!("P0Filler{i}"),
                vec![Type::Artifact],
                vec![],
            );
        }

        // p1: mint a second ascender + 9 fillers, then flip the controller of
        // those ten objects to p1 (on_field mints under p0).
        let mut p1_objs = Vec::new();
        p1_objs.push(on_field(
            &mut state,
            "Ascender1",
            vec![Type::Enchantment],
            vec![ascend()],
        ));
        for i in 0..9 {
            p1_objs.push(on_field(
                &mut state,
                &format!("P1Filler{i}"),
                vec![Type::Artifact],
                vec![],
            ));
        }
        for &id in &p1_objs {
            state.objects.obj_mut(id).controller = p1;
        }

        // Sanity: each player controls exactly ten battlefield permanents.
        let controlled = |state: &GameState, who: PlayerId| {
            state
                .zones
                .battlefield
                .iter()
                .filter(|&&id| state.objects.obj(id).controller == who)
                .count()
        };
        assert_eq!(state.zones.battlefield.len(), 20);
        assert_eq!(controlled(&state, p0), 10, "p0 controls ten permanents");
        assert_eq!(controlled(&state, p1), 10, "p1 controls ten permanents");

        // One sweep grants the blessing to BOTH players.
        let actions = sba::sweep(&state);
        assert!(
            actions.iter().any(|e| matches!(e,
                GameEvent::GotDesignation { player, name: n } if *player == p0 && *n == name)),
            "p0 gets the blessing; got {actions:?}"
        );
        assert!(
            actions.iter().any(|e| matches!(e,
                GameEvent::GotDesignation { player, name: n } if *player == p1 && *n == name)),
            "p1 gets the blessing; got {actions:?}"
        );

        // Apply all; both players end up holding the per-player designation.
        state.schedule_front(vec![WorkItem::Emit(Occurrence::Batch(actions))]);
        let _ = state.step();
        assert!(
            state.designations.players.contains_key(&(p0, name)),
            "p0 holds the city's blessing ([CR#702.131c])"
        );
        assert!(
            state.designations.players.contains_key(&(p1, name)),
            "p1 holds the city's blessing ([CR#702.131c])"
        );
    }

    /// [CR#704.5f]: a creature whose toughness drops to 0 (via -1/-1 counters)
    /// is put into its owner's graveyard by the rules-SBA pass — emitted as a
    /// `ZoneWillChange` to `Graveyard`, NOT a `WillDestroy` (so regeneration
    /// and indestructible cannot save it).
    #[test]
    fn toughness_zero_creature_is_put_into_graveyard() {
        let (mut state, bear) = bear_on_field();
        state.sba_rules = builtin().sba_rules;
        state.counter_decls = builtin().counters;
        // Grizzly Bears has toughness 2; two -1/-1 counters bring it to 0.
        state
            .objects
            .obj_mut(bear)
            .counters
            .insert("M1M1Counter".into(), 2);
        let actions = sba::sweep(&state);
        assert!(
            actions.iter().any(|e| matches!(
                e,
                GameEvent::ZoneWillChange { object, to: Zone::Graveyard, .. } if *object == bear
            )),
            "toughness-0 creature should be put into its graveyard (a Move, not a destroy); \
             got {actions:?}"
        );
        // Must NOT be a WillDestroy — regeneration/indestructible must not save it.
        assert!(
            !actions
                .iter()
                .any(|e| matches!(e, GameEvent::WillDestroy { object, .. } if *object == bear)),
            "toughness-0 is a put-into-graveyard, never a destroy; got {actions:?}"
        );
    }

    /// [CR#704.5f,702.12b]: indestructible does NOT save a creature whose
    /// toughness drops to 0. The toughness-0 SBA emits a `ZoneWillChange`
    /// (a Move), not a `WillDestroy`, so the cant-happen guard never fires.
    /// After the sweep applies the creature is gone.
    #[test]
    fn toughness_zero_kills_even_indestructible() {
        let (mut state, myr) = myr_on_field(); // Darksteel Myr: indestructible, toughness 1
        state.sba_rules = builtin().sba_rules;
        state.counter_decls = builtin().counters;
        // One -1/-1 counter: toughness 1 → 0.
        state
            .objects
            .obj_mut(myr)
            .counters
            .insert("M1M1Counter".into(), 1);
        let actions = sba::sweep(&state);
        state.schedule_front(vec![WorkItem::Emit(Occurrence::Batch(actions))]);
        let _ = state.step();
        assert!(
            state.objects.get(myr).is_none() || !state.zones.battlefield.contains(&myr),
            "an indestructible creature with toughness 0 is still put into the graveyard"
        );
        assert!(
            !state.zones.graveyards[0].is_empty(),
            "it went to the graveyard"
        );
    }

    /// [CR#704] rules-SBA events must carry `StateBasedAction` agency and no
    /// agent — the game performs them, not an effect or the object itself.
    /// The toughness-0 rule emits a `Move` (`ZoneWillChange`) with `cause:
    /// None` today; after the stamp it must have `agency ==
    /// StateBasedAction` and `agent.is_none()`.
    #[test]
    fn rules_sba_events_carry_state_based_action_cause() {
        let (mut state, bear) = bear_on_field();
        state.sba_rules = builtin().sba_rules;
        state.counter_decls = builtin().counters;
        // Grizzly Bears toughness 2; two -1/-1 counters drop it to 0.
        state
            .objects
            .obj_mut(bear)
            .counters
            .insert("M1M1Counter".into(), 2);
        let actions = sba::sweep(&state);
        let move_ev = actions
            .iter()
            .find(|e| {
                matches!(e,
                    GameEvent::ZoneWillChange { object, to: Zone::Graveyard, .. }
                    if *object == bear)
            })
            .expect("toughness-0 ZoneWillChange must be present");
        let GameEvent::ZoneWillChange { cause: Some(c), .. } = move_ev else {
            panic!("rules-SBA ZoneWillChange must carry a cause after the stamp; got {move_ev:?}")
        };
        assert_eq!(
            c.agency,
            deckmaste_core::Agency::StateBasedAction,
            "rules-SBA event must have StateBasedAction agency"
        );
        assert!(c.agent.is_none(), "a state-based action has no agent");
    }

    /// [CR#704.5i]: a planeswalker with loyalty 0 is put into its owner's
    /// graveyard by the rules-SBA pass. The pass is generic (B1); this test
    /// characterizes it against the loyalty-zero rule already loaded by
    /// `builtin().sba_rules`. With `LoyaltyCounter` present the walker
    /// survives.
    #[test]
    fn loyalty_zero_planeswalker_is_put_into_graveyard() {
        let (mut state, _bear) = bear_on_field();
        state.sba_rules = builtin().sba_rules;
        state.counter_decls = builtin().counters;
        let pw = on_field(&mut state, "Test Walker", vec![Type::Planeswalker], vec![]);
        // 0 LoyaltyCounters ⇒ loyalty 0 ⇒ SBA fires.
        let actions = sba::sweep(&state);
        assert!(
            actions.iter().any(|e| matches!(
                e,
                GameEvent::ZoneWillChange { object, to: Zone::Graveyard, .. } if *object == pw
            )),
            "a planeswalker with loyalty 0 is put into its graveyard; got {actions:?}"
        );
        // With loyalty counters present it survives.
        state
            .objects
            .obj_mut(pw)
            .counters
            .insert("LoyaltyCounter".into(), 3);
        let actions = sba::sweep(&state);
        assert!(
            !actions
                .iter()
                .any(|e| matches!(e, GameEvent::ZoneWillChange { object, .. } if *object == pw)),
            "loyalty 3 planeswalker survives the sweep; got {actions:?}"
        );
    }

    /// [CR#704.5v]: a battle with defense 0 is put into its owner's graveyard by
    /// the rules-SBA pass. With `DefenseCounter` present the battle survives.
    #[test]
    fn defense_zero_battle_is_put_into_graveyard() {
        let (mut state, _bear) = bear_on_field();
        state.sba_rules = builtin().sba_rules;
        state.counter_decls = builtin().counters;
        let battle = on_field(&mut state, "Test Siege", vec![Type::Battle], vec![]);
        // 0 DefenseCounters ⇒ defense 0 ⇒ SBA fires.
        let actions = sba::sweep(&state);
        assert!(
            actions.iter().any(|e| matches!(
                e,
                GameEvent::ZoneWillChange { object, to: Zone::Graveyard, .. } if *object == battle
            )),
            "a battle with defense 0 is put into its graveyard; got {actions:?}"
        );
        // With defense counters present it survives.
        state
            .objects
            .obj_mut(battle)
            .counters
            .insert("DefenseCounter".into(), 4);
        let actions = sba::sweep(&state);
        assert!(
            !actions.iter().any(
                |e| matches!(e, GameEvent::ZoneWillChange { object, .. } if *object == battle)
            ),
            "defense 4 battle survives the sweep; got {actions:?}"
        );
    }

    /// The global rules-SBA sweep must not panic on permanents outside every
    /// rule's scope (e.g. an enchantment has no toughness/loyalty/defense).
    /// This verifies that `scope` is checked before `when` — the stat read
    /// never runs on an out-of-scope object.
    #[test]
    fn non_matching_permanent_does_not_panic_in_rules_sweep() {
        let (mut state, _bear) = bear_on_field();
        state.sba_rules = builtin().sba_rules;
        state.counter_decls = builtin().counters;
        // A bare enchantment — no toughness, no loyalty, no defense.
        let _ench = on_field(
            &mut state,
            "Test Enchantment",
            vec![Type::Enchantment],
            vec![],
        );
        // Must not panic regardless of how many rules are loaded.
        let _actions = sba::sweep(&state);
    }

    // --- Legend rule [CR#704.5j] -----------------------------------------------

    use deckmaste_core::Supertype;

    /// Mint a legendary creature on the battlefield for the given controller.
    fn legendary_creature(
        state: &mut GameState,
        name: &str,
        controller: PlayerId,
    ) -> crate::object::ObjectId {
        let card = deckmaste_core::Card::Normal(deckmaste_core::CardFace {
            name: name.into(),
            types: vec![Type::Creature],
            supertypes: vec![Supertype::Legendary],
            ..deckmaste_core::CardFace::default()
        });
        let card_id = state.cards.push(Arc::new(card), controller);
        let id = state.objects.mint(
            ObjectSource::Card(card_id),
            controller,
            Some(Zone::Battlefield),
        );
        state.zones.battlefield.push(id);
        id
    }

    /// Mint a non-legendary creature on the battlefield for the given
    /// controller.
    fn nonlegendary_creature(
        state: &mut GameState,
        name: &str,
        controller: PlayerId,
    ) -> crate::object::ObjectId {
        let card = deckmaste_core::Card::Normal(deckmaste_core::CardFace {
            name: name.into(),
            types: vec![Type::Creature],
            supertypes: vec![],
            ..deckmaste_core::CardFace::default()
        });
        let card_id = state.cards.push(Arc::new(card), controller);
        let id = state.objects.mint(
            ObjectSource::Card(card_id),
            controller,
            Some(Zone::Battlefield),
        );
        state.zones.battlefield.push(id);
        id
    }

    #[test]
    fn legend_rule_groups_finds_same_name_legendaries() {
        let (mut state, _bear) = bear_on_field();
        let a = legendary_creature(&mut state, "Bob", PlayerId(0));
        let b = legendary_creature(&mut state, "Bob", PlayerId(0));
        let _other = legendary_creature(&mut state, "Carol", PlayerId(0)); // singleton, no group
        let groups = sba::legend_rule_groups(&state);
        assert_eq!(groups.len(), 1, "one group: the two Bobs");
        let (player, cands) = &groups[0];
        assert_eq!(*player, PlayerId(0));
        assert_eq!(cands.len(), 2);
        assert!(cands.contains(&a) && cands.contains(&b));
    }

    #[test]
    fn legend_rule_groups_ignores_split_controllers_and_nonlegendaries() {
        let (mut state, _bear) = bear_on_field();
        legendary_creature(&mut state, "Bob", PlayerId(0));
        legendary_creature(&mut state, "Bob", PlayerId(1)); // different controller
        nonlegendary_creature(&mut state, "Mox", PlayerId(0));
        nonlegendary_creature(&mut state, "Mox", PlayerId(0)); // not legendary
        assert!(sba::legend_rule_groups(&state).is_empty());
    }

    /// Build a `This`-anchored frame for `id`, mirroring the literal used in
    /// `attachment_sbas` and `global_sba_rules`.
    fn this_frame(state: &GameState, id: crate::object::ObjectId) -> crate::stack::Frame {
        crate::stack::Frame {
            source: id,
            controller: state.objects.obj(id).controller,
            targets: Vec::new(),
            bindings: None,
            chosen: None,
            x: None,
            it: None,
            that: None,
            allotment: None,
        }
    }

    /// [CR#120.3]: `Count::Damage(Reference::This)` reads an object's marked
    /// damage. Grizzly Bears has toughness 2; at 2 damage the lethal-damage
    /// condition holds; at 1 it does not.
    #[test]
    fn damage_count_reads_marked_damage() {
        use deckmaste_core::Cmp;
        use deckmaste_core::Condition;
        use deckmaste_core::Count;
        use deckmaste_core::Reference;
        use deckmaste_core::Stat;

        let (mut state, bear) = bear_on_field(); // Grizzly Bears, toughness 2
        let frame = this_frame(&state, bear);
        let lethal = Condition::Compare(
            Count::Damage(Reference::This),
            Cmp::AtLeast,
            Count::StatOf(Reference::This, Stat::Toughness),
        );
        state.objects.obj_mut(bear).damage = 2;
        assert!(
            state.condition_holds(&lethal, &frame),
            "2 damage >= toughness 2"
        );
        state.objects.obj_mut(bear).damage = 1;
        assert!(
            !state.condition_holds(&lethal, &frame),
            "1 damage < toughness 2"
        );
    }

    /// [CR#704.5h]: `Condition::DamagedByDeathtouch` reads the deal-time
    /// `struck_by_deathtouch` flag on the referenced object. False by default;
    /// true once the flag is set.
    #[test]
    fn damaged_by_deathtouch_reads_the_flag() {
        use deckmaste_core::Condition;
        use deckmaste_core::Reference;

        let (mut state, bear) = bear_on_field();
        let frame = this_frame(&state, bear);
        let cond = Condition::DamagedByDeathtouch(Reference::This);
        assert!(
            !state.condition_holds(&cond, &frame),
            "flag clear by default"
        );
        state.objects.obj_mut(bear).struck_by_deathtouch = true;
        assert!(
            state.condition_holds(&cond, &frame),
            "flag set → condition holds"
        );
    }

    // --- lethal-damage rule [CR#704.5g,704.5h] ----------------------------------

    /// [CR#704.5g]: a creature with lethal marked damage is destroyed via the
    /// rules-SBA rule, with `StateBasedAction` cause and verb "Destroy".
    #[test]
    fn lethal_damage_rule_destroys_via_state_based_action() {
        let (mut state, bear) = bear_on_field(); // toughness 2
        state.sba_rules = builtin().sba_rules;
        state.objects.obj_mut(bear).damage = 2;
        let actions = sba::sweep(&state);
        let n = actions
            .iter()
            .filter(|e| matches!(e, GameEvent::WillDestroy { object, .. } if *object == bear))
            .count();
        assert_eq!(n, 1, "exactly one WillDestroy from the rule");
        let ev = actions
            .iter()
            .find(|e| matches!(e, GameEvent::WillDestroy { object, .. } if *object == bear))
            .unwrap();
        let GameEvent::WillDestroy { cause: Some(c), .. } = ev else {
            panic!("WillDestroy must carry a cause; got {ev:?}")
        };
        assert_eq!(
            c.agency,
            deckmaste_core::Agency::StateBasedAction,
            "lethal-damage destroy must carry StateBasedAction agency"
        );
        assert_eq!(
            c.verb,
            deckmaste_core::Ident::from("Destroy"),
            "lethal-damage destroy must carry the Destroy verb"
        );
    }

    /// [CR#704.5h]: a creature struck by deathtouch is destroyed even if the
    /// physical damage is sublethal.
    #[test]
    fn deathtouch_strike_rule_destroys() {
        let (mut state, bear) = bear_on_field();
        state.sba_rules = builtin().sba_rules;
        // 1 damage (< toughness 2) but struck by deathtouch
        state.objects.obj_mut(bear).damage = 1;
        state.objects.obj_mut(bear).struck_by_deathtouch = true;
        let actions = sba::sweep(&state);
        assert!(
            actions
                .iter()
                .any(|e| matches!(e, GameEvent::WillDestroy { object, .. } if *object == bear)),
            "a creature struck by deathtouch must be destroyed ([CR#704.5h]); got {actions:?}"
        );
    }

    /// [CR#704.5g,704.5h]: a creature with BOTH lethal damage AND a deathtouch
    /// strike emits exactly one `WillDestroy` — the `OneOf` in the rule
    /// prevents the rule from firing twice.
    #[test]
    fn lethal_and_deathtouch_emits_one_destroy() {
        let (mut state, bear) = bear_on_field(); // toughness 2
        state.sba_rules = builtin().sba_rules;
        state.objects.obj_mut(bear).damage = 2;
        state.objects.obj_mut(bear).struck_by_deathtouch = true;
        let actions = sba::sweep(&state);
        let n = actions
            .iter()
            .filter(|e| matches!(e, GameEvent::WillDestroy { object, .. } if *object == bear))
            .count();
        assert_eq!(
            n, 1,
            "OneOf dedups to a single WillDestroy — no double-destroy panic; got {actions:?}"
        );
    }

    /// [CR#704.5g]: a creature with SUBLETHAL damage and NO deathtouch is not
    /// destroyed.
    #[test]
    fn sublethal_no_deathtouch_survives() {
        let (mut state, bear) = bear_on_field();
        state.sba_rules = builtin().sba_rules;
        state.objects.obj_mut(bear).damage = 1; // < toughness 2, no deathtouch
        let actions = sba::sweep(&state);
        assert!(
            !actions
                .iter()
                .any(|e| matches!(e, GameEvent::WillDestroy { object, .. } if *object == bear)),
            "sublethal damage without deathtouch must not destroy; got {actions:?}"
        );
    }

    /// [CR#704.5f,704.5g]: a creature with toughness 0 and marked damage only
    /// gets the Move [CR#704.5f], NOT also a Destroy (the `toughness > 0` guard
    /// in the lethal-damage rule prevents it).
    #[test]
    fn zero_toughness_damaged_creature_moves_not_destroyed() {
        let (mut state, bear) = bear_on_field();
        state.sba_rules = builtin().sba_rules;
        state.counter_decls = builtin().counters;
        // Two -1/-1 counters: toughness 2 → 0.
        state
            .objects
            .obj_mut(bear)
            .counters
            .insert("M1M1Counter".into(), 2);
        state.objects.obj_mut(bear).damage = 5;
        let actions = sba::sweep(&state);
        assert!(
            actions.iter().any(|e| matches!(
                e,
                GameEvent::ZoneWillChange { object, to: Zone::Graveyard, .. } if *object == bear
            )),
            "toughness-0 creature must get the Move; got {actions:?}"
        );
        assert!(
            !actions
                .iter()
                .any(|e| matches!(e, GameEvent::WillDestroy { object, .. } if *object == bear)),
            "toughness > 0 guard: no lethal-damage destroy on a 0-toughness creature; \
             got {actions:?}"
        );
    }
}
