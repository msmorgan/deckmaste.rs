//! State-based actions ([CR#704]). Player losses: zero or less life
//! ([CR#704.5a]), drew from an empty library ([CR#704.5b]), ten or more
//! poison counters ([CR#704.5c]). A permanent with both +1/+1 and -1/-1
//! counters has the smaller count of each removed ([CR#704.5q]). Creatures
//! with lethal marked damage are destroyed ([CR#704.5g]); tokens stranded off
//! the battlefield cease to exist ([CR#704.5d]); a copy of a spell stranded
//! anywhere other than the stack ceases to exist too ([CR#707.10a]).

use deckmaste_core::Zone;

use crate::agenda::WorkItem;
use crate::event::AbilityCountered;
use crate::event::Act;
use crate::event::GameEvent;
use crate::event::LossReason;
use crate::event::Occurrence;
use crate::event::PlayerLost;
use crate::event::Unattached;
use crate::event::ZoneChange;
use crate::object::ObjectId;
use crate::stack::StackObject;
use crate::state::GameState;

/// One sweep ([CR#704.3]): the `PlayerLost`, `Act(Destroy)` (the replaceable
/// destruction event), `TokenCeased`, and off-stack-copy `AbilityCountered`
/// events this check would perform. The caller emits them and re-checks until
/// a sweep comes back empty. A destroy's LKI snapshot is captured later, at
/// the will-change apply the `Act(Destroy)` resolves into (the object is still
/// live then), not here.
#[must_use]
pub fn sweep(state: &GameState) -> Vec<GameEvent> {
    let mut actions = Vec::new();
    let view = state.layers();

    let poison: deckmaste_core::Ident = "Poison".into();
    for player in &state.players {
        if player.lost {
            continue;
        }
        // [CR#704.5a..704.5c] loss predicates; a CantLose gate suppresses the
        // outcome at this check ([CR#101.1,704.3], ADR U5 precedence), leaving
        // the underlying state (a "zombie" life ≤ 0 / poison ≥ 10) intact.
        let reason = if player.life <= 0 {
            Some(LossReason::LifeZero)
        } else if player.drew_from_empty {
            Some(LossReason::DrewFromEmpty)
        } else if state
            // [CR#704.5c]: player counters live on the player's PROXY
            // object ([CR#122.1] — counters go on objects and players; one
            // storage, never a parallel map), placed/removed by the
            // PutCounters/RemoveCounters apply arms. Two-Headed Giant swaps in
            // the fifteen-counter TEAM check ([CR#704.6b]) — variant-gated, not
            // built.
            .objects
            .obj(player.object)
            .counters
            .get(&poison)
            .is_some_and(|&n| n >= 10)
        {
            Some(LossReason::Poison)
        } else {
            None
        };
        if let Some(reason) = reason
            && !state.gate_suppresses(&view, player.id, deckmaste_core::OutcomeGateKind::CantLose)
        {
            actions.push(GameEvent::PlayerLost(PlayerLost {
                player: player.id,
                reason,
            }));
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

    // [CR#704.5d,707.10a]: a copy of a spell anywhere other than the stack
    // ceases to exist. The SCAN stays native (core-copy-grammar Task 5,
    // Path B — see the task report): the generic `SbaRule` domain is
    // battlefield objects ONLY (`global_sba_rules`'s
    // `state.zones.battlefield` loop, per `sba_rule.rs`'s "for every
    // battlefield object" doc) and never reaches a stranded STACK entry.
    // Widening that shared domain to also walk the stack would risk
    // misfiring every OTHER battlefield-scoped rule (toughness-zero,
    // loyalty-zero, battle-defense-zero — none of which guard their
    // `scope`/`when` against a same-shaped object still resolving on the
    // stack) against it — a correctness rewrite this task does not sign up
    // for. The EMISSION, though, is now data-usable: it speaks through
    // `Action::Cease` ([CR#704.5d,707.10a]) via the SAME two calls
    // (`run_sba_effect` + `stamp_sba_cause`) `global_sba_rules` makes per
    // rule, instead of building `AbilityCountered` inline — so the
    // cease-to-exist SHAPE is unified even though this rule's domain stays
    // native.
    //
    // The resolution/counter divert (`resolve_object`'s Spell arm,
    // `Action::Counter`) is the happy path — a copy vanishes there before it
    // ever reaches this sweep. This is the safety net for a copy some OTHER
    // (not yet built) generic zone-mover strands off-stack: `state.stack`
    // still carries the copy's entry (the mover hasn't reached
    // `remove_stack_entry` for it), but its backing object's zone reads
    // something other than `Stack`. `Action::Cease`'s resolve reuses
    // `AbilityCountered`'s apply (remove the stack entry, remove the
    // object, no zone move) — ceases it the same way the happy path does.
    for entry in &state.stack {
        if entry.copy
            && let StackObject::Spell(spell) = &entry.object
            && state
                .objects
                .get(*spell)
                .is_some_and(|o| o.zone != Some(Zone::Stack))
        {
            let frame = state.frame(entry.id, entry.controller);
            let effect = deckmaste_core::Instruction::act(deckmaste_core::Action::Cease(
                deckmaste_core::Reference::source_parameter(),
            ));
            for mut ev in run_sba_effect(state, &effect, &frame) {
                stamp_sba_cause(&mut ev);
                actions.push(ev);
            }
        }
    }

    // Attachment SBAs ([CR#704.5m..704.5p]) — GENERIC, no subtype branch.
    actions.extend(attachment_sbas(state, &view));

    actions
}

/// The attachment state-based actions ([CR#704.5m..704.5p]) — extracted from
/// [`sweep`] but logically part of the same [CR#704.3] check. Two passes,
/// both keyed on conferred data + the `attached_to` relation only; NEVER on
/// the Aura/Equipment/Fortification subtype:
///
/// 1. **Firing state-checked SBA rows.** Two sources, both `{when, then}`:
///    the type/subtype-conferred `Property::StateBased` flavor — the Aura
///    graveyard rule ([CR#704.5m]) is `StateBased(Not(LegallyAttached(
///    Ref(This))), Move(Ref(This), Graveyard))`, an ability-free conferral
///    because a [CR#704] game action is not an ability ([CR#704.1,704.1a])
///    — and the `StaticSpec::Sba` statics an OBJECT carries, which still
///    spell card-level state-checked statics such as
///    ascend ([CR#702.131b]). For each battlefield object, evaluate `when`
///    with `This` = the object; if true, run `then`'s events. Objects a
///    firing row removes this sweep are tracked so pass 2 doesn't
///    double-handle them.
/// 2. **Generic illegal-attachment cleanup.** Any object attached to an illegal
///    host (per `attachment_legal`) that no firing `Sba` removed → becomes
///    unattached and stays ([CR#704.5n] Equipment/Fortification; [CR#704.5p]
///    creature / battle / other permanent — engine-identical).
fn attachment_sbas(state: &GameState, view: &crate::layer::LayeredView) -> Vec<GameEvent> {
    let mut out = Vec::new();
    let mut removed_by_sba: std::collections::BTreeSet<ObjectId> =
        std::collections::BTreeSet::new();

    // (1) Firing state-checked SBA rows.
    for &id in &state.zones.battlefield {
        // A `This`-anchored frame: `condition_holds`/`action_items` resolve
        // `Ref(This)` to this object via the frame source ([CR#603.10a]).
        let frame = state.frame(id, state.objects.obj(id).controller);
        let mut rows: Vec<(deckmaste_core::Condition, deckmaste_core::Instruction)> = Vec::new();
        // Type/subtype-conferred SBAs ([CR#704.5m]) — the ability-free
        // `Property::StateBased` flavor, read off the DERIVED types/subtypes so
        // a layer-4 grant contributes exactly like a printed one. It confers no
        // ability ([`Property::conferred_ability`] is `None` for it), so the
        // static walk below never sees it; same read as
        // `counter_state_based_sbas` does for a counter's.
        let derived = view.get(id);
        for prop in derived
            .card_types
            .iter()
            .flat_map(|t| t.confers.iter())
            .chain(derived.subtypes.iter().flat_map(|s| s.confers.iter()))
        {
            if let deckmaste_core::Property::StateBased { condition, effect } = prop {
                rows.push(((**condition).clone(), (**effect).clone()));
            }
        }
        crate::legal::for_each_static(state, view, id, |e| {
            if let deckmaste_core::StaticSpec::Sba { when, then } = e {
                rows.push((when.as_ref().clone(), (**then).clone()));
            }
        });
        for (when, then) in rows {
            if !state.condition_holds(&when, &frame) {
                continue;
            }
            // Run `then` — `Act(<Action>)` (the Aura's `Move`, the Saga
            // generalization's `Sacrifice`) or a `Sequentially` of them.
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
            out.push(GameEvent::Unattached(Unattached {
                attachment: id,
                former_host: host,
            }));
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
        let frame = state.frame(id, obj.controller);
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
        let bare = state.frame(id, state.objects.obj(id).controller);
        for rule in &state.sba_rules {
            let mut frame = bare.clone();
            frame.activation = state.enter_region(&rule.region, &frame);
            let rule = &rule.region.body;
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
/// one already exists (e.g. the lethal-damage SBA's `Act(Destroy)` keeps its
/// "Destroy" verb, [CR#701.8b] — the SBA is destruction's OTHER cause). For
/// events whose `cause` is `None` (plain `Move` actions), upserts a
/// `StateBasedAction` cause so no rules-SBA event goes unattributed.
fn stamp_sba_cause(ev: &mut GameEvent) {
    // `AbilityCountered.cause` is a plain `Cause` (always present — no
    // `None`/upsert branch applies), so it gets its own short-circuit arm:
    // same "preserve verb, correct agency, drop agent" treatment as the
    // `Some` case below. This is what lets the copy-cease SBA
    // ([CR#704.5d,707.10a]) route its removal through `Action::Cease`'s
    // resolve (which stamps a generic `EffectInstruction` cause, since the
    // verb is reusable outside an SBA context) and still land on
    // `StateBasedAction`/no-agent here, same as any other rules-SBA event.
    if let GameEvent::AbilityCountered(AbilityCountered { cause, .. }) = ev {
        cause.agency = deckmaste_core::Agency::StateBasedAction;
        cause.agent = None;
        return;
    }
    let cause_slot = match ev {
        GameEvent::Act(Act { cause, .. })
        | GameEvent::ZoneChange(ZoneChange {
            snapshot: None,
            cause,
            ..
        }) => Some(cause),
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
                payment: None,
            });
        }
    }
}

/// Run an SBA `then`/`effect` purely (no apply) into the events it produces:
/// `Act(<Action>)`, or a `Sequentially` of effects (each evaluated against the
/// SAME pre-sweep state — so the annihilation's two `RemoveCounters` both read
/// the pre-removal counts). Choice-bearing shapes are a documented seam.
fn run_sba_effect(
    state: &GameState,
    effect: &deckmaste_core::Instruction,
    frame: &crate::stack::ExecutionFrame,
) -> Vec<GameEvent> {
    use deckmaste_core::Instruction;

    let mut out = Vec::new();
    match effect {
        Instruction::Act { action, .. } => {
            for item in state.action_items(action, frame) {
                if let WorkItem::Emit(occ) = item {
                    match occ {
                        Occurrence::Single(ev) => out.push(ev),
                        Occurrence::Batch(evs) => out.extend(evs),
                    }
                }
            }
        }
        Instruction::Sequentially(children) => {
            for child in children.iter() {
                out.extend(run_sba_effect(state, child, frame));
            }
        }
        // NOT unbuilt work: `docs/decisions/state-based-actions-are-data.md`
        // scopes the data-driven `SbaRule.then` path to unconditional effects,
        // because "choice cannot be represented by an unconditional effect".
        // Choice-bearing SBAs (the legend rule, illegal auras) stay imperative
        // native Rust and never route through this interpreter. So this arm is
        // a design boundary, not a seam awaiting a ticket.
        other => todo!(
            "SBA effect is only Act/Sequentially by design (got {other:?}) — a choice-bearing \
             SBA stays imperative; owner: docs/decisions/state-based-actions-are-data.md"
        ),
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
        let name = crate::derive::face(state.def(id)).name.to_string();
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
    #![allow(
        clippy::too_many_lines,
        reason = "e2e SBA scenarios in this test module read better whole than \
        split into helpers; exempted module-wide"
    )]

    use std::path::Path;
    use std::sync::Arc;

    use deckmaste_card::Card;
    use deckmaste_core::Predicate;
    use deckmaste_core::Type;
    use deckmaste_core::Zone;
    use deckmaste_plugin::plugin::Plugin;

    use crate::agenda::WorkItem;
    use crate::event::Act;
    use crate::event::CounterRemoved;
    use crate::event::DamageDealt;
    use crate::event::GameEvent;
    use crate::event::GotDesignation;
    use crate::event::Occurrence;
    use crate::event::PlayerLost;
    use crate::event::Unattached;
    use crate::event::ZoneChange;
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
    /// Only ever called from `#[cfg_attr(not(wizards_corpus), ignore)]`
    /// tests, so the corpus is guaranteed present when this runs (build.rs
    /// sets the `wizards_corpus` cfg from the directory's presence).
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
        let bears = Arc::new(canon().card("Grizzly Bears").unwrap().core);
        let forest = Arc::new(builtin().card("Forest").unwrap().core);
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
            conferral_rules: vec![],
            damage_result_rules: vec![],
            counter_decls: std::collections::HashMap::new(),
            subtypes: std::collections::HashMap::new(),
            types: std::collections::HashMap::new(),
        });
        // Force a Grizzly Bears from player 0's hand onto the battlefield.
        let bear = *state.zones.hands[0]
            .iter()
            .find(|&&o| obj_matches(&state, o, &Predicate::r#type(Type::Creature)))
            .expect("a Grizzly Bears in the opening hand");
        state.zones.hands[PlayerId(0).index()].retain(|&o| o != bear);
        state.objects.obj_mut(bear).zone = Some(Zone::Battlefield);
        state.zones.battlefield.push(bear);
        (state, bear)
    }

    /// Player 0's deck = Darksteel Myr (indestructible 0/1), one on the field.
    fn myr_on_field() -> (GameState, crate::object::ObjectId) {
        let myr = Arc::new(canon().card("Darksteel Myr").unwrap().core);
        let forest = Arc::new(builtin().card("Forest").unwrap().core);
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
            conferral_rules: vec![],
            damage_result_rules: vec![],
            counter_decls: std::collections::HashMap::new(),
            subtypes: std::collections::HashMap::new(),
            types: std::collections::HashMap::new(),
        });
        let m = *state.zones.hands[0]
            .iter()
            .find(|&&o| obj_matches(&state, o, &Predicate::r#type(Type::Creature)))
            .expect("a Darksteel Myr in the opening hand");
        state.zones.hands[PlayerId(0).index()].retain(|&o| o != m);
        state.objects.obj_mut(m).zone = Some(Zone::Battlefield);
        state.zones.battlefield.push(m);
        (state, m)
    }

    /// End-to-end through the WIZARDS load path ([CR#704.5m]): a graduated
    /// wizards Aura (Angelic Gift) carries the Aura subtype's ability-free
    /// `StateBased` graveyard rule *via the data*, not in-Rust scaffolding. Loaded over the
    /// builtin sibling prelude, put on the battlefield UNATTACHED, the generic
    /// SBA sweep fires its battlefield→graveyard move. This is the regression
    /// the fix targets: the Aura `confers:` lives in builtin and the generator
    /// emits no confers-less wizards stub to shadow it, so a fresh wizards card
    /// inherits the attachment rule.
    #[test]
    #[cfg_attr(
        not(wizards_corpus),
        ignore = "requires generated plugins/wizards corpus"
    )]
    fn wizards_aura_carries_conferred_graveyard_rule() {
        let gift = Arc::new(wizards().card("Angelic Gift").unwrap().core);
        // Sanity: the loaded card actually carries the Aura subtype's confer.
        // (the layer-4 `fold_conferred_abilities` is what flattens it onto the
        // derived object.)
        let face = crate::derive::face(&gift);
        assert!(
            face.subtypes.iter().any(|s| s
                .confers
                .iter()
                .any(|p| matches!(p, deckmaste_core::Property::StateBased { .. }))),
            "the wizards Aura card embeds the state-based confer; subtypes: {:?}",
            face.subtypes
        );

        let mut state = GameState::new(GameConfig {
            players: vec![PlayerConfig { deck: vec![] }, PlayerConfig { deck: vec![] }],
            seed: 1,
            starting_life: 20,
            starting_player: StartingPlayer::Fixed(PlayerId(0)),
            sba_rules: vec![],
            conferral_rules: vec![],
            damage_result_rules: vec![],
            counter_decls: std::collections::HashMap::new(),
            subtypes: std::collections::HashMap::new(),
            types: std::collections::HashMap::new(),
        });
        let card_id = state.cards.push(gift, PlayerId(0));
        let aura = state.objects.mint(
            ObjectSource::Card(card_id),
            PlayerId(0),
            Some(Zone::Battlefield),
        );
        state.zones.battlefield.push(aura);

        // Unattached → `LegallyAttached` is false → the conferred state-based
        // rule fires, moving the Aura to its owner's graveyard.
        let actions = sba::sweep(&state);
        assert!(
            actions.iter().any(|e| matches!(e,
                GameEvent::ZoneChange(ZoneChange { snapshot: None, object, to: Zone::Graveyard, .. }) if *object == aura)),
            "a graduated wizards Aura's conferred graveyard rule fires when unattached \
             ([CR#704.5m]); got {actions:?}"
        );
    }

    /// [CR#704.5g,702.12b]: an indestructible creature with lethal damage is
    /// NOT destroyed by the SBA — the sweep emits an `Act(Destroy)`, and the
    /// event-side cant pass ([CR#614.17]) in `apply_occurrence` suppresses it
    /// before `apply` runs. The Myr stays on the battlefield.
    #[test]
    fn indestructible_survives_lethal_damage() {
        let (mut state, myr) = myr_on_field();
        // Load builtin rules so the lethal-damage SBA fires via the rule path.
        state.sba_rules = builtin().sba_rules;
        state.objects.obj_mut(myr).set_marked_damage(1); // toughness 1 → lethal
        let actions = sba::sweep(&state);
        state.schedule_front(vec![WorkItem::Emit(Occurrence::Batch(actions))]);
        let _ = state.step(); // Act(Destroy) applies → replaced to nothing
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
        // the destroy as a replaceable `Act(Destroy)` intent (its apply commits
        // the battlefield→graveyard move when nothing replaces it), cause-tagged
        // as the SBA destruction verb ([CR#701.8b]).
        state.objects.obj_mut(bear).set_marked_damage(2);
        let actions = sba::sweep(&state);
        assert!(
            actions.iter().any(|e| matches!(
                e,
                GameEvent::Act(Act {
                    verb,
                    on,
                    cause: Some(c),
                    ..
                }) if verb.as_str() == "Destroy"
                    && on.as_slice() == [bear]
                    && c.verb == deckmaste_core::Ident::from("Destroy")
                    && c.agency == deckmaste_core::Agency::StateBasedAction
            )),
            "sweep should include an Act(Destroy) for Grizzly Bears at lethal damage"
        );

        // Sublethal: damage = 1 < toughness 2.
        state.objects.obj_mut(bear).set_marked_damage(1);
        let actions = sba::sweep(&state);
        assert!(
            actions.iter().all(
                |e| !matches!(e, GameEvent::Act(Act { verb, .. }) if verb.as_str() == "Destroy")
            ),
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
                    GameEvent::CounterRemoved(CounterRemoved { object, kind: k, amount: 2, .. })
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
                .all(|e| !matches!(e, GameEvent::CounterRemoved(CounterRemoved { .. }))),
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
        let forest = Arc::new(builtin().card("Forest").unwrap().core);
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
            conferral_rules: vec![],
            damage_result_rules: vec![],
            counter_decls: std::collections::HashMap::new(),
            subtypes: std::collections::HashMap::new(),
            types: std::collections::HashMap::new(),
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
    /// gone from the store, with no `ZoneChange` fact (ceasing to exist is
    /// not a move). A token still on the battlefield never ceases.
    #[test]
    fn dead_token_ceases_to_exist() {
        use deckmaste_core::Action;
        use deckmaste_core::Count;
        use deckmaste_core::Instruction;
        use deckmaste_core::Reference;
        use deckmaste_core::Token;

        let (mut state, src) = bear_on_field();
        let frame = state.frame(src, PlayerId(0));
        let token = Token {
            name: None,
            color_indicator: vec![].into(),
            supertypes: vec![].into(),
            types: vec![Type::Artifact.def()].into(),
            subtypes: vec![].into(),
            abilities: vec![].into(),
            power: None,
            toughness: None,
        };
        state.run_effect(
            Instruction::act(Action::Create {
                agent: Reference::Reg(deckmaste_core::RefId(1)),
                count: Count::Literal(1),
                token: token.into(),
                riders: vec![].into(),
            }),
            &frame,
        );
        let _ = state.step(); // TokenCreated applies
        let _ = state.step(); // its past-form ZoneChange fact
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
            GameEvent::ZoneChange(ZoneChange {
                snapshot: None,
                object: token_obj,
                from: Some(Zone::Battlefield),
                to: Zone::Graveyard,
                enters: None,
                position: None,
                face: None,
                cause: None,
            }),
        ))]);
        let _ = state.step(); // the move applies
        let _ = state.step(); // its past-form ZoneChange fact
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

    /// core-copy-grammar Task 5 (Path B): the copy-cease SBA's
    /// ([CR#704.5d,707.10a]) native scan still walks `state.stack` — the
    /// generic `SbaRule` domain (`global_sba_rules`'s
    /// `state.zones.battlefield` loop) never reaches a stranded stack
    /// entry, so data-fying the SCAN itself is out of scope here — but the
    /// removal it emits now speaks through the data-usable `Action::Cease`
    /// verb instead of borrowing `Counter`'s: the returned
    /// `AbilityCountered`'s cause carries the "Cease" verb and
    /// `StateBasedAction` agency with no agent, the same re-attribution a
    /// rules-driven `SbaRule`'s cause gets from `stamp_sba_cause`. A copy
    /// stranded off the stack — its `StackEntry` still lingers, only the
    /// backing object's zone reads something other than `Stack` —
    /// reproduces the exact shape `off_stack_copy_ceases_via_sba`
    /// (`tests/stack.rs`) reaches via a forced zone mutation on a fully
    /// cast-and-copied Bolt; built here by hand (mint + push a `StackEntry`
    /// directly) for a focused unit check on the emitted cause.
    #[test]
    fn stranded_copy_ceases_via_unified_cease_action() {
        use deckmaste_core::Agency;

        use crate::event::AbilityCountered;

        let bolt = Arc::new(canon().card("Lightning Bolt").unwrap().core);
        let mut state = game();
        let card_id = state.cards.push(bolt, PlayerId(0));
        // Stranded: the backing object's zone already reads something other
        // than `Stack` while its `StackEntry` still lingers — the exact
        // precondition the native scan checks (`entry.copy && ... zone !=
        // Some(Zone::Stack)`, below).
        let copy_obj = state.objects.mint(
            ObjectSource::Card(card_id),
            PlayerId(0),
            Some(Zone::Graveyard),
        );
        state.stack.push(crate::stack::StackEntry {
            activation: crate::ActivationId::NONE,
            id: copy_obj,
            object: crate::stack::StackObject::Spell(copy_obj),
            controller: PlayerId(0),
            targets: vec![],
            chosen_modes: std::sync::Arc::from([]),
            x: None,
            paid_costs: vec![],
            copy: true,
        });

        let actions = sba::sweep(&state);
        let removal = actions
            .iter()
            .find(|e| {
                matches!(e, GameEvent::AbilityCountered(AbilityCountered { id, .. }) if *id == copy_obj)
            })
            .unwrap_or_else(|| {
                panic!("[CR#707.10a]: sweep must cease the stranded copy; got {actions:?}")
            });
        let GameEvent::AbilityCountered(AbilityCountered { cause, .. }) = removal else {
            unreachable!("matched above");
        };
        assert_eq!(
            cause.verb.as_str(),
            "Cease",
            "[CR#704.5d,707.10a]: the emitted removal must speak through the \
             unified Cease verb, not the borrowed Counter one; got {cause:?}"
        );
        assert_eq!(
            cause.agency,
            Agency::StateBasedAction,
            "a state-based action has no agent; got {cause:?}"
        );
        assert!(
            cause.agent.is_none(),
            "state-based actions carry no agent; got {cause:?}"
        );
    }

    /// core-copy-grammar Task 5 (review follow-up): [CR#111.7] a token —
    /// INCLUDING a token copy ([CR#707.1]) — on the battlefield is NEVER
    /// touched by the copy-cease SBA ([CR#704.5d,707.10a]). Provable
    /// structurally (the native scan's domain is `state.stack` only, and a
    /// token never rides a `StackEntry` — it isn't cast), but this pins it
    /// behaviorally: mint a token COPY (the shape most likely to be
    /// confused for a `CopyOfACard`, per `target::is_object_class`'s [CR#109.1]
    /// carve-out —
    /// `token_copy_and_plain_token_both_classify_as_token_not_card_copy`,
    /// `resolve/player_action.rs`, pins the same carve-out at the
    /// classification layer), put it on the battlefield, run the FULL sweep
    /// (`sba::sweep` — there is no copy-cease-only entry point), and assert
    /// no `AbilityCountered` fires for it and the object survives.
    #[test]
    fn battlefield_token_copy_survives_the_copy_cease_sweep() {
        use deckmaste_core::Action;
        use deckmaste_core::CopySource;
        use deckmaste_core::CopySpec;
        use deckmaste_core::Count;
        use deckmaste_core::Instruction;
        use deckmaste_core::ObjectClass;
        use deckmaste_core::Reference;

        use crate::event::AbilityCountered;

        let (mut state, bear) = bear_on_field();
        let frame = state.frame(bear, PlayerId(0));
        state.run_effect(
            Instruction::act(Action::Create {
                agent: Reference::Reg(deckmaste_core::RefId(1)),
                count: Count::Literal(1),
                token: deckmaste_core::TokenSpec::Copy(
                    CopySpec {
                        source: CopySource::Object(Reference::Reg(deckmaste_core::RefId(0))),
                        exceptions: vec![],
                    }
                    .into(),
                ),
                riders: vec![].into(),
            }),
            &frame,
        );
        let _ = state.step(); // TokenCreated applies
        let _ = state.step(); // its past-form ZoneChange fact
        let &copy_token = state
            .zones
            .battlefield
            .iter()
            .find(|&&id| id != bear)
            .expect("the token copy on the battlefield");

        assert!(
            crate::target::is_object_class(&state, copy_token, ObjectClass::Token),
            "[CR#109.1,111.1]: a minted token copy classifies as Token, never \
             CardCopy — the copy-cease SBA's scope would never select it even \
             if data-fied"
        );

        let actions = sba::sweep(&state);
        assert!(
            actions.iter().all(|e| !matches!(
                e,
                GameEvent::AbilityCountered(AbilityCountered { id, .. }) if *id == copy_token
            )),
            "[CR#111.7]: a battlefield token copy must NEVER be removed by the \
             copy-cease SBA ([CR#704.5d,707.10a]) — its own cease rule is the \
             token SBA (a token stranded OFF the battlefield), not this one; \
             got {actions:?}"
        );
        assert!(
            state.objects.get(copy_token).is_some(),
            "[CR#111.7]: the token copy survives on the battlefield untouched"
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
        state.objects.obj_mut(bear).set_marked_damage(2);
        let actions = sba::sweep(&state);
        state.schedule_front(vec![WorkItem::Emit(Occurrence::Batch(actions))]);
        // Act(Destroy) applies (nothing replaces it) → future-form ZoneChange remints.
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

    use deckmaste_card::CardFace;
    use deckmaste_core::Ability;
    use deckmaste_core::Condition;
    use deckmaste_core::Deontic;
    use deckmaste_core::DeonticAction;
    use deckmaste_core::Instruction;
    use deckmaste_core::Reference;
    use deckmaste_core::StaticSpec;

    fn game() -> GameState {
        GameState::new(GameConfig {
            players: vec![PlayerConfig { deck: vec![] }, PlayerConfig { deck: vec![] }],
            seed: 1,
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

    fn controller_filter(body: Predicate) -> deckmaste_core::Region<Predicate> {
        deckmaste_core::Region::new(
            Arc::from([
                deckmaste_core::Param {
                    def: deckmaste_core::DefId(0),
                    kind: deckmaste_core::Kind::Entity,
                    provenance: deckmaste_core::Provenance::Candidate(
                        deckmaste_core::Domain::Entity,
                    ),
                },
                deckmaste_core::Param {
                    def: deckmaste_core::DefId(1),
                    kind: deckmaste_core::Kind::Entity,
                    provenance: deckmaste_core::Provenance::Controller,
                },
            ]),
            body,
        )
    }

    fn on_field(
        state: &mut GameState,
        name: &str,
        types: Vec<Type>,
        abilities: Vec<Ability>,
    ) -> crate::object::ObjectId {
        let card = Card::Normal(CardFace {
            name: name.into(),
            types: types.into_iter().map(Type::def).collect(),
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

    /// A CARD-LEVEL state-checked static ([CR#604.1], ascend's shape) spelling
    /// the same graveyard rule: `Static(Sba(Not(LegallyAttached(Ref(This))),
    /// Move(Ref(This), Graveyard)))`. The Aura SUBTYPE confers its [CR#704.5m]
    /// rule ability-free instead (`aura_subtype`).
    fn aura_graveyard_sba() -> Ability {
        Ability::r#static(StaticSpec::Sba {
            when: Arc::new(Condition::Not(Arc::new(Condition::LegallyAttached(
                Reference::Reg(deckmaste_core::RefId(0)),
            )))),
            then: Arc::new(Instruction::act(deckmaste_core::Action::move_to(
                Reference::Reg(deckmaste_core::RefId(0)),
                Zone::Graveyard,
            ))),
        })
    }

    /// The default-deny Enchant grant shape ([CR#702.5a]): `Static(May(Attach(
    /// what: Ref(This), to: Creature)))` — an attachment that may legally
    /// attach to a creature host (and to nothing else without a further grant).
    fn may_attach_creature() -> Ability {
        Ability::r#static(StaticSpec::Deontic(Deontic::May(DeonticAction::Attach {
            what: Predicate::Ref(Reference::Reg(deckmaste_core::RefId(0))),
            to: Predicate::r#type(Type::Creature),
        })))
    }

    /// [CR#704.5m]: an Aura (carrying the card-level graveyard `Sba`) that is
    /// UNATTACHED fires the SBA → a future-form `ZoneChange(Battlefield →
    /// Graveyard)` for it. Generic — driven by the `Sba` static, not the
    /// subtype.
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
                GameEvent::ZoneChange(ZoneChange { snapshot: None, object, to: Zone::Graveyard, .. }) if *object == aura)),
            "unattached Aura is moved to the graveyard ([CR#704.5m]); got {actions:?}"
        );
    }

    /// The Aura SUBTYPE's own declaration, conferring its [CR#704.5m] rule the
    /// ability-free way: `Property::StateBased { condition, effect }`. A
    /// [CR#704] state-based action is not an ability ([CR#704.1,704.1a]), so it
    /// is not `Property::Ability(Static(Sba(..)))`.
    fn aura_subtype() -> deckmaste_core::Subtype {
        deckmaste_core::Subtype {
            name: "Aura".into(),
            types: [Type::Enchantment].into(),
            confers: [deckmaste_core::Property::StateBased {
                condition: Arc::new(Condition::Not(Arc::new(Condition::LegallyAttached(
                    Reference::Reg(deckmaste_core::RefId(0)),
                )))),
                effect: Arc::new(Instruction::act(deckmaste_core::Action::move_to(
                    Reference::Reg(deckmaste_core::RefId(0)),
                    Zone::Graveyard,
                ))),
            }]
            .into(),
        }
    }

    fn on_field_with_subtypes(
        state: &mut GameState,
        name: &str,
        types: Vec<Type>,
        subtypes: Vec<deckmaste_core::Subtype>,
    ) -> crate::object::ObjectId {
        let card = Card::Normal(CardFace {
            name: name.into(),
            types: types.into_iter().map(Type::def).collect(),
            subtypes,
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

    /// [CR#704.5m] through the ability-free conferral: an Aura whose graveyard
    /// rule arrives as the subtype's `Property::StateBased` (no ability at all)
    /// still fires in the sweep. Same asserted outcome as
    /// `sba_attach_unattached_aura_goes_to_graveyard`, new spelling.
    #[test]
    fn state_based_conferral_moves_unattached_aura_to_graveyard() {
        let mut state = game();
        let aura = on_field_with_subtypes(
            &mut state,
            "Test Aura",
            vec![Type::Enchantment],
            vec![aura_subtype()],
        );
        let actions = sba::sweep(&state);
        assert!(
            actions.iter().any(|e| matches!(e,
                GameEvent::ZoneChange(ZoneChange { snapshot: None, object, to: Zone::Graveyard, .. }) if *object == aura)),
            "a `Property::StateBased` Aura conferral fires the [CR#704.5m] move; got {actions:?}"
        );
    }

    /// The taxonomic point ([CR#704.1a,604.1]): the `StateBased` conferral
    /// grants the Aura NO ability — it is invisible to the derived ability
    /// list — yet the SBA still fires (asserted above).
    #[test]
    fn state_based_conferral_grants_no_ability() {
        let mut state = game();
        let aura = on_field_with_subtypes(
            &mut state,
            "Test Aura",
            vec![Type::Enchantment],
            vec![aura_subtype()],
        );
        let view = state.layers();
        assert!(
            view.get(aura).abilities.is_empty(),
            "a state-based conferral is not an ability; got {:?}",
            view.get(aura).abilities
        );
    }

    /// Lock a `LoseAllAbilities` (layer-6, end-of-game) continuous effect onto
    /// `id` — strips every ability it has; ability-free type rules are
    /// untouched because they never entered the list.
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
            rows: vec![],
            origin: None,
            is_cda: false,
        });
    }

    /// Review #7 e2e, re-spelled onto the ability-free conferral
    /// ([CR#113.12,704.5m]): an Aura whose abilities are ALL stripped by an
    /// active `LoseAllAbilities` and is UNATTACHED still goes to the
    /// graveyard. The subtype's `Property::StateBased` rule is not an ability,
    /// so layer-6 removal cannot reach it and the sweep still fires it —
    /// emitting the battlefield→graveyard move, and (driven to completion)
    /// landing the reminted object in its owner's graveyard.
    #[test]
    fn ability_less_aura_still_graveyards() {
        let mut state = game();
        let aura = on_field_with_subtypes(
            &mut state,
            "Test Aura",
            vec![Type::Enchantment],
            vec![aura_subtype()],
        );
        // Strip ALL of the Aura's abilities. The conferred type rule is not
        // one, so it must survive.
        lose_all_abilities(&mut state, aura);

        // The sweep STILL emits the graveyard move for the unattached Aura.
        let actions = sba::sweep(&state);
        assert!(
            actions.iter().any(|e| matches!(e,
                GameEvent::ZoneChange(ZoneChange { snapshot: None, object, to: Zone::Graveyard, .. }) if *object == aura)),
            "the conferred graveyard rule survives LoseAllAbilities \
             ([CR#113.12,704.5m]); got {actions:?}"
        );

        // Drive it to completion: the Aura ends up in its owner's graveyard.
        state.schedule_front(vec![WorkItem::Emit(Occurrence::Batch(actions))]);
        let _ = state.step(); // the move applies (remint + LKI)
        let _ = state.step(); // its past-form ZoneChange fact
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
        // Under default-deny the Aura needs a `May(Attach to: Creature)` grant
        // for the attachment to a creature host to be legal.
        let aura = on_field(
            &mut state,
            "Test Aura",
            vec![Type::Enchantment],
            vec![aura_graveyard_sba(), may_attach_creature()],
        );
        let host = on_field(&mut state, "Bear", vec![Type::Creature], vec![]);
        state.objects.obj_mut(aura).attached_to = Some(host);
        let actions = sba::sweep(&state);
        assert!(
            !actions
                .iter()
                .any(|e| matches!(e, GameEvent::ZoneChange(ZoneChange { snapshot: None, object, .. }) if *object == aura)),
            "legally-attached Aura stays put; got {actions:?}"
        );
    }

    /// [CR#704.5n]: an Equipment (no firing `Sba`) attached to an ILLEGAL host
    /// (a non-creature its `May(Attach to: Creature)` grant does NOT cover)
    /// becomes unattached and stays — the generic illegal-attachment cleanup,
    /// NO subtype branch. Illegal because the grant doesn't reach this
    /// host, not merely because a grant is absent.
    #[test]
    fn sba_attach_illegal_equipment_unattaches() {
        let mut state = game();
        let equip = on_field(
            &mut state,
            "Test Equipment",
            vec![Type::Artifact],
            vec![may_attach_creature()],
        );
        // Illegally attached to a non-creature artifact — outside the grant.
        let rock = on_field(&mut state, "Rock", vec![Type::Artifact], vec![]);
        state.objects.obj_mut(equip).attached_to = Some(rock);

        let actions = sba::sweep(&state);
        assert!(
            actions.iter().any(|e| matches!(e,
                GameEvent::Unattached(Unattached { attachment, former_host })
                if *attachment == equip && *former_host == rock)),
            "illegally-attached Equipment becomes unattached ([CR#704.5n]); got {actions:?}"
        );
        // It does NOT go to the graveyard (no firing Sba).
        assert!(
            !actions
                .iter()
                .any(|e| matches!(e, GameEvent::ZoneChange(ZoneChange { snapshot: None, object, .. }) if *object == equip)),
            "Equipment stays on the battlefield, not graveyard"
        );
    }

    /// [CR#704.5p]: a permanent whose `May(Attach)` grant WOULD cover the host
    /// (a creature) but whose link is defeated by the host-side protection
    /// `Cant` → becomes unattached (engine-identical to [CR#704.5n]). The
    /// `Cant` subtracts from the grant — the link is illegal for the
    /// restriction, not for a missing grant.
    #[test]
    fn sba_attach_plain_permanent_illegal_link_unattaches() {
        let mut state = game();
        // A `Thing` whose grant covers creatures, illegally linked to a
        // protected creature (the host-side Cant defeats the grant).
        let thing = on_field(
            &mut state,
            "Thing",
            vec![Type::Artifact],
            vec![may_attach_creature()],
        );
        let host = on_field(&mut state, "Bear", vec![Type::Creature], vec![]);
        // Give the host a protection-shaped host-side Cant so the link is
        // illegal even though `thing`'s grant would otherwise cover it.
        let protected = on_field(
            &mut state,
            "Protected",
            vec![Type::Creature],
            vec![Ability::r#static(StaticSpec::Deontic(Deontic::Cant(
                DeonticAction::Attach {
                    what: Predicate::Any,
                    to: Predicate::Ref(Reference::Reg(deckmaste_core::RefId(0))),
                },
            )))],
        );
        let _ = host;
        state.objects.obj_mut(thing).attached_to = Some(protected);

        let actions = sba::sweep(&state);
        assert!(
            actions.iter().any(|e| matches!(e,
                GameEvent::Unattached(Unattached { attachment, former_host })
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
        use deckmaste_core::Countable;
        use deckmaste_core::RelationPredicate;
        use deckmaste_core::StatePredicate;

        let mut state = game();
        let name: deckmaste_core::Ident = "CitysBlessing".into();
        let p0 = PlayerId(0);

        // The Ascend static, built typed (mirrors the builtin macro's expansion).
        let gate = Condition::And(
            vec![
                Condition::Compare(
                    Count::CountOf(Countable::Objects(Arc::new(controller_filter(
                        Predicate::And(
                            vec![
                                Predicate::State(StatePredicate::InZone(Zone::Battlefield)),
                                Predicate::Relation(RelationPredicate::ControlledBy(Arc::new(
                                    Predicate::Ref(Reference::Reg(deckmaste_core::RefId(1))),
                                ))),
                            ]
                            .into(),
                        ),
                    )))),
                    Cmp::AtLeast,
                    Count::Literal(10),
                ),
                Condition::Not(Arc::new(Condition::Matches(
                    Reference::Reg(deckmaste_core::RefId(1)),
                    Predicate::State(StatePredicate::Designated(name)),
                ))),
            ]
            .into(),
        );
        let ascend = Ability::r#static(StaticSpec::Sba {
            when: Arc::new(gate),
            then: Arc::new(Instruction::act(Action::GetDesignation(
                Reference::Reg(deckmaste_core::RefId(1)),
                name,
            ))),
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
                .all(|e| !matches!(e, GameEvent::GotDesignation(GotDesignation { .. }))),
            "no blessing at nine permanents"
        );

        // Tenth permanent → the sweep emits the grant for p0.
        on_field(&mut state, "Filler8", vec![Type::Artifact], vec![]);
        let actions = sba::sweep(&state);
        assert!(
            actions.iter().any(|e| matches!(e,
                GameEvent::GotDesignation(GotDesignation { player, name: n }) if *player == p0 && *n == name)),
            "blessing granted at ten permanents; got {actions:?}"
        );

        // Apply it; the store holds it and a re-sweep emits nothing (no loop).
        state.schedule_front(vec![WorkItem::Emit(Occurrence::Batch(actions))]);
        let _ = state.step();
        assert!(state.designations.players.contains_key(&(p0, name)));
        assert!(
            sba::sweep(&state)
                .iter()
                .all(|e| !matches!(e, GameEvent::GotDesignation(GotDesignation { .. }))),
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
        use deckmaste_core::Countable;
        use deckmaste_core::RelationPredicate;
        use deckmaste_core::StatePredicate;

        let name: deckmaste_core::Ident = "CitysBlessing".into();
        let p0 = PlayerId(0);
        let p1 = PlayerId(1);

        // The Ascend static, built typed (mirrors the builtin macro's
        // expansion). `ControlledBy(Ref(You))` resolves `You` to the carrying
        // object's controller via the Sba frame, so each ascender counts ITS
        // controller's permanents and grants to that controller.
        let ascend = || {
            Ability::r#static(StaticSpec::Sba {
                when: Arc::new(Condition::And(
                    vec![
                        Condition::Compare(
                            Count::CountOf(Countable::Objects(Arc::new(controller_filter(
                                Predicate::And(
                                    vec![
                                        Predicate::State(StatePredicate::InZone(Zone::Battlefield)),
                                        Predicate::Relation(RelationPredicate::ControlledBy(
                                            Arc::new(Predicate::Ref(Reference::Reg(
                                                deckmaste_core::RefId(1),
                                            ))),
                                        )),
                                    ]
                                    .into(),
                                ),
                            )))),
                            Cmp::AtLeast,
                            Count::Literal(10),
                        ),
                        Condition::Not(Arc::new(Condition::Matches(
                            Reference::Reg(deckmaste_core::RefId(1)),
                            Predicate::State(StatePredicate::Designated(name)),
                        ))),
                    ]
                    .into(),
                )),
                then: Arc::new(Instruction::act(Action::GetDesignation(
                    Reference::Reg(deckmaste_core::RefId(1)),
                    name,
                ))),
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
                GameEvent::GotDesignation(GotDesignation { player, name: n }) if *player == p0 && *n == name)),
            "p0 gets the blessing; got {actions:?}"
        );
        assert!(
            actions.iter().any(|e| matches!(e,
                GameEvent::GotDesignation(GotDesignation { player, name: n }) if *player == p1 && *n == name)),
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
    /// future-form `ZoneChange` to `Graveyard`, NOT an `Act(Destroy)` (so
    /// regeneration and indestructible cannot save it).
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
                GameEvent::ZoneChange(ZoneChange { snapshot: None, object, to: Zone::Graveyard, .. }) if *object == bear
            )),
            "toughness-0 creature should be put into its graveyard (a Move, not a destroy); \
             got {actions:?}"
        );
        // Must NOT be an Act(Destroy) — regeneration/indestructible must not save it.
        assert!(
            !actions
                .iter()
                .any(|e| matches!(e, GameEvent::Act(Act { verb, on, .. }) if verb.as_str() == "Destroy" && on.as_slice() == [bear])),
            "toughness-0 is a put-into-graveyard, never a destroy; got {actions:?}"
        );
    }

    /// [CR#704.5f,702.12b]: indestructible does NOT save a creature whose
    /// toughness drops to 0. The toughness-0 SBA emits a future-form
    /// `ZoneChange` (a Move), not an `Act(Destroy)`, so the cant-happen
    /// guard never fires. After the sweep applies the creature is gone.
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
    /// The toughness-0 rule emits a `Move` (future-form `ZoneChange`) with
    /// `cause: None` today; after the stamp it must have `agency ==
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
                    GameEvent::ZoneChange(ZoneChange { snapshot: None, object, to: Zone::Graveyard, .. })
                    if *object == bear)
            })
            .expect("toughness-0 future-form ZoneChange must be present");
        let GameEvent::ZoneChange(ZoneChange {
            snapshot: None,
            cause: Some(c),
            ..
        }) = move_ev
        else {
            panic!(
                "rules-SBA future-form ZoneChange must carry a cause after the stamp; got {move_ev:?}"
            )
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
                GameEvent::ZoneChange(ZoneChange { snapshot: None, object, to: Zone::Graveyard, .. }) if *object == pw
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
                .any(|e| matches!(e, GameEvent::ZoneChange(ZoneChange { snapshot: None, object, .. }) if *object == pw)),
            "loyalty 3 planeswalker survives the sweep; got {actions:?}"
        );
    }

    /// [CR#704.5w]: a non-Siege battle with defense 0 is put into its owner's
    /// graveyard by the rules-SBA pass. This subtype-less fixture exercises
    /// that branch of the shared generic rule; the Siege exception is
    /// documented at the rule declaration. With `DefenseCounter` present
    /// the battle survives.
    #[test]
    fn defense_zero_battle_is_put_into_graveyard() {
        let (mut state, _bear) = bear_on_field();
        state.sba_rules = builtin().sba_rules;
        state.counter_decls = builtin().counters;
        let battle = on_field(&mut state, "Test Battle", vec![Type::Battle], vec![]);
        // 0 DefenseCounters ⇒ defense 0 ⇒ SBA fires.
        let actions = sba::sweep(&state);
        assert!(
            actions.iter().any(|e| matches!(
                e,
                GameEvent::ZoneChange(ZoneChange { snapshot: None, object, to: Zone::Graveyard, .. }) if *object == battle
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
                |e| matches!(e, GameEvent::ZoneChange(ZoneChange { snapshot: None, object, .. }) if *object == battle)
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
        let card = deckmaste_card::Card::Normal(deckmaste_card::CardFace {
            name: name.into(),
            types: vec![Type::Creature.def()],
            supertypes: vec![Supertype::Legendary],
            ..deckmaste_card::CardFace::default()
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
        let card = deckmaste_card::Card::Normal(deckmaste_card::CardFace {
            name: name.into(),
            types: vec![Type::Creature.def()],
            supertypes: vec![],
            ..deckmaste_card::CardFace::default()
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
    fn this_frame(state: &GameState, id: crate::object::ObjectId) -> crate::stack::ExecutionFrame {
        state.frame(id, state.objects.obj(id).controller)
    }

    /// [CR#120.3]: `Count::Damage(Reference::Reg(deckmaste_core::RefId(0)))` reads an object's marked
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
            Count::Damage(Reference::Reg(deckmaste_core::RefId(0))),
            Cmp::AtLeast,
            Count::StatOf(Reference::Reg(deckmaste_core::RefId(0)), Stat::Toughness),
        );
        state.objects.obj_mut(bear).set_marked_damage(2);
        assert!(
            state.condition_holds(&lethal, &frame),
            "2 damage >= toughness 2"
        );
        state.objects.obj_mut(bear).set_marked_damage(1);
        assert!(
            !state.condition_holds(&lethal, &frame),
            "1 damage < toughness 2"
        );
    }

    /// [CR#704.5h]: the deathtouch clause reads the DEAL-TIME abilities
    /// captured on the object's damage marks — false with no marks, false for
    /// a plain (non-deathtouch) mark, true once a deathtouch-sourced mark
    /// exists. Re-spelled from the retired `Is(Source, …)` reference form
    /// ([CR#120.1] — a damage source is a relation, not a referent); same
    /// subject, same asserted outcomes. Both surviving core spellings of that
    /// relation are pinned side by side: the `DealtDamageBy` condition the
    /// lethal-damage SBA authors, and the `WasDealtDamageBy` state predicate
    /// that carries the same query into filters and target constraints.
    #[test]
    fn dealt_damage_by_deathtouch_reads_deal_time_marks() {
        use deckmaste_core::Ability;
        use deckmaste_core::CharacteristicPredicate;
        use deckmaste_core::Condition;
        use deckmaste_core::KeywordAbility;
        use deckmaste_core::Predicate;
        use deckmaste_core::Reference;

        let (mut state, bear) = bear_on_field();
        let frame = this_frame(&state, bear);
        // The predicate spelling of the same relation, which — unlike the
        // condition — also rides filters and target constraints.
        let as_predicate = Condition::Matches(
            Reference::source_parameter(),
            Predicate::State(deckmaste_core::StatePredicate::WasDealtDamageBy(
                std::sync::Arc::new(Predicate::Characteristic(CharacteristicPredicate::Has(
                    "Deathtouch".into(),
                ))),
            )),
        );
        let cond = Condition::DealtDamageBy(
            Reference::source_parameter(),
            Predicate::Characteristic(CharacteristicPredicate::Has("Deathtouch".into())),
        );
        assert!(
            !state.condition_holds(&cond, &frame),
            "no damage marks → no deathtouch source"
        );
        assert!(
            !state.condition_holds(&as_predicate, &frame),
            "the predicate spelling agrees: no damage marks → no deathtouch source"
        );
        // A plain (non-deathtouch) source does not satisfy it.
        state.objects.obj_mut(bear).mark_damage(None, Vec::new(), 1);
        assert!(
            !state.condition_holds(&cond, &frame),
            "a non-deathtouch source does not satisfy Has(Deathtouch)"
        );
        assert!(
            !state.condition_holds(&as_predicate, &frame),
            "the predicate spelling agrees: a non-deathtouch source does not satisfy \
             Has(Deathtouch)"
        );
        // A deal-time deathtouch source does.
        state.objects.obj_mut(bear).mark_damage(
            None,
            vec![Ability::Keyword(KeywordAbility::Deathtouch)],
            1,
        );
        assert!(
            state.condition_holds(&cond, &frame),
            "a deathtouch-sourced mark → condition holds"
        );
        assert!(
            state.condition_holds(&as_predicate, &frame),
            "the predicate spelling agrees: a deathtouch-sourced mark → condition holds"
        );
    }

    // --- lethal-damage rule [CR#704.5g,704.5h] ----------------------------------

    /// [CR#704.5g]: a creature with lethal marked damage is destroyed via the
    /// rules-SBA rule, with `StateBasedAction` cause and verb "Destroy".
    #[test]
    fn lethal_damage_rule_destroys_via_state_based_action() {
        let (mut state, bear) = bear_on_field(); // toughness 2
        state.sba_rules = builtin().sba_rules;
        state.objects.obj_mut(bear).set_marked_damage(2);
        let actions = sba::sweep(&state);
        let n = actions
            .iter()
            .filter(|e| matches!(e, GameEvent::Act(Act { verb, on, .. }) if verb.as_str() == "Destroy" && on.as_slice() == [bear]))
            .count();
        assert_eq!(n, 1, "exactly one Act(Destroy) from the rule");
        let ev = actions
            .iter()
            .find(|e| matches!(e, GameEvent::Act(Act { verb, on, .. }) if verb.as_str() == "Destroy" && on.as_slice() == [bear]))
            .unwrap();
        let GameEvent::Act(Act { cause: Some(c), .. }) = ev else {
            panic!("Act(Destroy) must carry a cause; got {ev:?}")
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
        // 1 damage (< toughness 2), dealt by a deathtouch source.
        state.objects.obj_mut(bear).mark_damage(
            None,
            vec![deckmaste_core::Ability::Keyword(
                deckmaste_core::KeywordAbility::Deathtouch,
            )],
            1,
        );
        let actions = sba::sweep(&state);
        assert!(
            actions
                .iter()
                .any(|e| matches!(e, GameEvent::Act(Act { verb, on, .. }) if verb.as_str() == "Destroy" && on.as_slice() == [bear])),
            "a creature struck by deathtouch must be destroyed ([CR#704.5h]); got {actions:?}"
        );
    }

    /// [CR#702.2c,704.5h]: deal-time capture — a source that dealt deathtouch
    /// damage and THEN left the battlefield still causes the destroy. The mark
    /// records the source's deathtouch AT DEAL TIME (via the real `DamageDealt`
    /// apply, which reads the layered view), so removing the source afterward
    /// leaves the pending destroy intact.
    #[test]
    fn deal_time_deathtouch_survives_the_source_leaving() {
        use deckmaste_core::KeywordAbility;
        use deckmaste_core::Zone;

        use crate::combat::has_keyword;
        use crate::object::ObjectSource;

        // Grizzly Bears (toughness 2) is the target.
        let (mut state, bear) = bear_on_field();
        state.sba_rules = builtin().sba_rules;

        // A deathtouch source (Typhoid Rats, 1/1 deathtouch) on the battlefield.
        let rats = Arc::new(canon().card("Typhoid Rats").unwrap().core);
        let rats_card = state.cards.push(Arc::clone(&rats), PlayerId(1));
        let source = state.objects.mint(
            ObjectSource::Card(rats_card),
            PlayerId(1),
            Some(Zone::Battlefield),
        );
        state.zones.battlefield.push(source);
        assert!(
            has_keyword(&state.layers(), source, &KeywordAbility::Deathtouch),
            "pre-condition: the source carries Keyword(Deathtouch) at deal time"
        );

        // Deal 1 (sublethal vs toughness 2) damage from the deathtouch source
        // through the real event pipeline — the apply captures the source's
        // deal-time abilities onto the mark.
        state.schedule_front(vec![WorkItem::Emit(Occurrence::single(
            GameEvent::DamageDealt(DamageDealt {
                source,
                target: bear,
                amount: 1,
                combat: true,
            }),
        ))]);
        let _ = state.step(); // the DamageDealt applies, marking the damage
        assert_eq!(
            state.objects.obj(bear).total_damage(),
            1,
            "1 sublethal point marked on the target"
        );

        // The source now LEAVES the battlefield entirely.
        state.zones.battlefield.retain(|&o| o != source);
        state.objects.obj_mut(source).zone = Some(Zone::Graveyard);
        assert!(
            !has_keyword(&state.layers(), source, &KeywordAbility::Deathtouch)
                || !state.zones.battlefield.contains(&source),
            "the source has left the battlefield"
        );

        // The sweep still destroys the bear: deal-time deathtouch on the mark
        // does not depend on the (now-gone) source ([CR#704.5h]).
        let actions = sba::sweep(&state);
        assert!(
            actions
                .iter()
                .any(|e| matches!(e, GameEvent::Act(Act { verb, on, .. }) if verb.as_str() == "Destroy" && on.as_slice() == [bear])),
            "a source that dealt deathtouch damage then left still destroys the target; got {actions:?}"
        );
    }

    /// [CR#704.5g,704.5h]: a creature with BOTH lethal damage AND a deathtouch
    /// strike emits exactly one `Act(Destroy)` — the `Or` in the rule
    /// prevents the rule from firing twice.
    #[test]
    fn lethal_and_deathtouch_emits_one_destroy() {
        let (mut state, bear) = bear_on_field(); // toughness 2
        state.sba_rules = builtin().sba_rules;
        // Lethal marked damage AND a deathtouch source (both clauses hold).
        state.objects.obj_mut(bear).mark_damage(
            None,
            vec![deckmaste_core::Ability::Keyword(
                deckmaste_core::KeywordAbility::Deathtouch,
            )],
            2,
        );
        let actions = sba::sweep(&state);
        let n = actions
            .iter()
            .filter(|e| matches!(e, GameEvent::Act(Act { verb, on, .. }) if verb.as_str() == "Destroy" && on.as_slice() == [bear]))
            .count();
        assert_eq!(
            n, 1,
            "Or dedups to a single Act(Destroy) — no double-destroy panic; got {actions:?}"
        );
    }

    /// [CR#704.5g]: a creature with SUBLETHAL damage and NO deathtouch is not
    /// destroyed.
    #[test]
    fn sublethal_no_deathtouch_survives() {
        let (mut state, bear) = bear_on_field();
        state.sba_rules = builtin().sba_rules;
        state.objects.obj_mut(bear).set_marked_damage(1); // < toughness 2, no deathtouch
        let actions = sba::sweep(&state);
        assert!(
            !actions
                .iter()
                .any(|e| matches!(e, GameEvent::Act(Act { verb, on, .. }) if verb.as_str() == "Destroy" && on.as_slice() == [bear])),
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
        state.objects.obj_mut(bear).set_marked_damage(5);
        let actions = sba::sweep(&state);
        assert!(
            actions.iter().any(|e| matches!(
                e,
                GameEvent::ZoneChange(ZoneChange { snapshot: None, object, to: Zone::Graveyard, .. }) if *object == bear
            )),
            "toughness-0 creature must get the Move; got {actions:?}"
        );
        assert!(
            !actions
                .iter()
                .any(|e| matches!(e, GameEvent::Act(Act { verb, on, .. }) if verb.as_str() == "Destroy" && on.as_slice() == [bear])),
            "toughness > 0 guard: no lethal-damage destroy on a 0-toughness creature; \
             got {actions:?}"
        );
    }

    #[test]
    fn platinum_angel_parses_two_outcome_gates() {
        use deckmaste_core::Ability;
        use deckmaste_core::StaticSpec;
        let angel = canon().card("Platinum Angel").unwrap().core;
        let gates = crate::derive::face(&angel)
            .abilities
            .iter()
            .filter(|a| matches!(a, Ability::Static(s) if matches!(&s.body, StaticSpec::OutcomeGate { .. })))
            .count();
        assert_eq!(gates, 2, "Platinum Angel has two OutcomeGate statics");
    }

    #[test]
    fn gate_suppresses_matches_controller_and_opponent() {
        use deckmaste_core::OutcomeGateKind;

        use crate::object::ObjectSource;
        let angel = Arc::new(canon().card("Platinum Angel").unwrap().core);
        let forest = Arc::new(builtin().card("Forest").unwrap().core);
        let mut state = GameState::new(GameConfig {
            players: vec![
                PlayerConfig {
                    deck: deck(&forest, 10),
                },
                PlayerConfig {
                    deck: deck(&forest, 10),
                },
            ],
            seed: 1,
            starting_life: 20,
            starting_player: StartingPlayer::Fixed(PlayerId(0)),
            sba_rules: vec![],
            conferral_rules: vec![],
            damage_result_rules: vec![],
            counter_decls: std::collections::HashMap::new(),
            subtypes: std::collections::HashMap::new(),
            types: std::collections::HashMap::new(),
        });
        // Mint a Platinum Angel controlled by player 0 onto the battlefield.
        let card_id = state.cards.push(angel, PlayerId(0));
        let obj = state.objects.mint(
            ObjectSource::Card(card_id),
            PlayerId(0),
            Some(Zone::Battlefield),
        );
        state.zones.battlefield.push(obj);

        let view = state.layers();
        // "You can't lose": player 0 (controller) is CantLose-gated.
        assert!(state.gate_suppresses(&view, PlayerId(0), OutcomeGateKind::CantLose));
        // "Your opponents can't win": player 1 is CantWin-gated.
        assert!(state.gate_suppresses(&view, PlayerId(1), OutcomeGateKind::CantWin));
        // Negatives.
        assert!(!state.gate_suppresses(&view, PlayerId(0), OutcomeGateKind::CantWin));
        assert!(!state.gate_suppresses(&view, PlayerId(1), OutcomeGateKind::CantLose));
    }

    #[test]
    fn platinum_angel_suppresses_life_zero_loss() {
        use crate::object::ObjectSource;
        let angel = Arc::new(canon().card("Platinum Angel").unwrap().core);
        let forest = Arc::new(builtin().card("Forest").unwrap().core);
        let mut state = GameState::new(GameConfig {
            players: vec![
                PlayerConfig {
                    deck: deck(&forest, 10),
                },
                PlayerConfig {
                    deck: deck(&forest, 10),
                },
            ],
            seed: 1,
            starting_life: 20,
            starting_player: StartingPlayer::Fixed(PlayerId(0)),
            sba_rules: vec![],
            conferral_rules: vec![],
            damage_result_rules: vec![],
            counter_decls: std::collections::HashMap::new(),
            subtypes: std::collections::HashMap::new(),
            types: std::collections::HashMap::new(),
        });
        let card_id = state.cards.push(angel, PlayerId(0));
        let angel_obj = state.objects.mint(
            ObjectSource::Card(card_id),
            PlayerId(0),
            Some(Zone::Battlefield),
        );
        state.zones.battlefield.push(angel_obj);
        state.players[0].life = 0;

        // Gated: no loss emitted for player 0.
        let actions = sba::sweep(&state);
        assert!(
            !actions.iter().any(
                |e| matches!(e, GameEvent::PlayerLost(PlayerLost { player, .. }) if *player == PlayerId(0))
            ),
            "Platinum Angel suppresses the life-zero loss"
        );

        // Remove the Angel → the loss fires the next sweep (standing predicate).
        state.zones.battlefield.retain(|&o| o != angel_obj);
        state.objects.obj_mut(angel_obj).zone = None;
        let actions = sba::sweep(&state);
        assert!(
            actions.iter().any(|e| matches!(
                e,
                GameEvent::PlayerLost(PlayerLost { player, reason: crate::event::LossReason::LifeZero })
                    if *player == PlayerId(0)
            )),
            "loss fires once the gate is gone"
        );
    }

    #[test]
    fn platinum_angel_suppresses_poison_loss() {
        use crate::object::ObjectSource;
        let angel = Arc::new(canon().card("Platinum Angel").unwrap().core);
        let forest = Arc::new(builtin().card("Forest").unwrap().core);
        let mut state = GameState::new(GameConfig {
            players: vec![
                PlayerConfig {
                    deck: deck(&forest, 10),
                },
                PlayerConfig {
                    deck: deck(&forest, 10),
                },
            ],
            seed: 1,
            starting_life: 20,
            starting_player: StartingPlayer::Fixed(PlayerId(0)),
            sba_rules: vec![],
            conferral_rules: vec![],
            damage_result_rules: vec![],
            counter_decls: std::collections::HashMap::new(),
            subtypes: std::collections::HashMap::new(),
            types: std::collections::HashMap::new(),
        });
        let card_id = state.cards.push(angel, PlayerId(0));
        let angel_obj = state.objects.mint(
            ObjectSource::Card(card_id),
            PlayerId(0),
            Some(Zone::Battlefield),
        );
        state.zones.battlefield.push(angel_obj);
        let p0_proxy = state.player(PlayerId(0)).object;
        state
            .objects
            .obj_mut(p0_proxy)
            .counters
            .insert("Poison".into(), 10);

        // Gated: no loss emitted for player 0.
        let actions = sba::sweep(&state);
        assert!(
            !actions.iter().any(
                |e| matches!(e, GameEvent::PlayerLost(PlayerLost { player, .. }) if *player == PlayerId(0))
            ),
            "Platinum Angel suppresses the poison loss"
        );

        // Remove the Angel → the loss fires the next sweep (standing predicate).
        state.zones.battlefield.retain(|&o| o != angel_obj);
        state.objects.obj_mut(angel_obj).zone = None;
        let actions = sba::sweep(&state);
        assert!(
            actions.iter().any(|e| matches!(
                e,
                GameEvent::PlayerLost(PlayerLost { player, reason: crate::event::LossReason::Poison })
                    if *player == PlayerId(0)
            )),
            "loss fires once the gate is gone"
        );
    }

    #[test]
    fn gated_empty_draw_window_lapses() {
        use crate::agenda::WorkItem;
        use crate::object::ObjectSource;
        let angel = Arc::new(canon().card("Platinum Angel").unwrap().core);
        let forest = Arc::new(builtin().card("Forest").unwrap().core);
        let mut state = GameState::new(GameConfig {
            players: vec![
                PlayerConfig {
                    deck: deck(&forest, 10),
                },
                PlayerConfig {
                    deck: deck(&forest, 10),
                },
            ],
            seed: 1,
            starting_life: 20,
            starting_player: StartingPlayer::Fixed(PlayerId(0)),
            sba_rules: vec![],
            conferral_rules: vec![],
            damage_result_rules: vec![],
            counter_decls: std::collections::HashMap::new(),
            subtypes: std::collections::HashMap::new(),
            types: std::collections::HashMap::new(),
        });
        let card_id = state.cards.push(angel, PlayerId(0));
        let angel_obj = state.objects.mint(
            ObjectSource::Card(card_id),
            PlayerId(0),
            Some(Zone::Battlefield),
        );
        state.zones.battlefield.push(angel_obj);
        state.players[0].drew_from_empty = true;

        // One SBA check while gated: loss suppressed AND window closed.
        state.schedule_front(vec![WorkItem::CheckSbas]);
        let _ = state.step();
        assert!(
            !state.players[0].drew_from_empty,
            "gated empty-draw window is closed"
        );

        // Remove the gate; the lapsed window means no retroactive loss.
        state.zones.battlefield.retain(|&o| o != angel_obj);
        state.objects.obj_mut(angel_obj).zone = None;
        let actions = sba::sweep(&state);
        assert!(
            !actions.iter().any(
                |e| matches!(e, GameEvent::PlayerLost(PlayerLost { player, .. }) if *player == PlayerId(0))
            ),
            "no retroactive empty-draw loss after the window lapsed"
        );
    }

    /// [CR#104.2a]: the last-player-standing win pierces a `CantWin` gate —
    /// it is a DERIVED win in `check_game_end`, never routed through the
    /// gate-checked `WinGame` verb ([CR#104.2b]). Player 0 controls Abyssal
    /// Persecutor ("you can't win the game and your opponents can't lose the
    /// game"); player 1 concedes; player 0 still wins. This doubly exercises
    /// [CR#104.3a]: player 1's concede-loss lands DESPITE the Persecutor's
    /// `CantLose`-on-opponents gate — concession pierces every gate.
    #[test]
    fn cant_win_gated_player_still_wins_last_standing() {
        use crate::decide::Action;
        use crate::decide::Decision;
        use crate::decide::DecisionPointKind;
        use crate::object::ObjectSource;

        let persecutor = Arc::new(canon().card("Abyssal Persecutor").unwrap().core);
        let forest = Arc::new(builtin().card("Forest").unwrap().core);
        let mut state = GameState::new(GameConfig {
            players: vec![
                PlayerConfig {
                    deck: deck(&forest, 10),
                },
                PlayerConfig {
                    deck: deck(&forest, 10),
                },
            ],
            seed: 1,
            starting_life: 20,
            starting_player: StartingPlayer::Fixed(PlayerId(0)),
            sba_rules: vec![],
            conferral_rules: vec![],
            damage_result_rules: vec![],
            counter_decls: std::collections::HashMap::new(),
            subtypes: std::collections::HashMap::new(),
            types: std::collections::HashMap::new(),
        });
        let card_id = state.cards.push(persecutor, PlayerId(0));
        let obj = state.objects.mint(
            ObjectSource::Card(card_id),
            PlayerId(0),
            Some(Zone::Battlefield),
        );
        state.zones.battlefield.push(obj);

        for _ in 0..500 {
            match state.step() {
                StepOutcome::Progress(_) => {}
                StepOutcome::NeedsDecision(DecisionPointKind::Priority(
                    crate::decide::pending::Priority { player, .. },
                )) => {
                    let answer = if player == PlayerId(1) { Action::Concede } else { Action::Pass };
                    state.submit_decision(Decision::Act(answer)).unwrap();
                }
                StepOutcome::NeedsDecision(other) => {
                    panic!("unexpected decision before the concede resolved: {other:?}")
                }
                StepOutcome::GameOver(outcome) => {
                    assert_eq!(
                        outcome,
                        GameOutcome::Win(PlayerId(0)),
                        "[CR#104.2a]: the last-standing win pierces player 0's CantWin gate"
                    );
                    return;
                }
            }
        }
        panic!("game never ended within 500 steps");
    }
}
