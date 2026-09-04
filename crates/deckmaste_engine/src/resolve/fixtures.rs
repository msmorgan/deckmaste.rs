//! Shared fixtures for the `resolve` submodules' tests: game/board
//! construction and agenda-draining helpers used across themes.

use std::path::Path;
use std::sync::Arc;

use deckmaste_card::Card;
use deckmaste_card::CardFace;
use deckmaste_core::Ability;
use deckmaste_core::Action;
use deckmaste_core::Count;
use deckmaste_core::Instruction;
use deckmaste_core::Predicate;
use deckmaste_core::Reference;
use deckmaste_core::Subtype;
use deckmaste_core::Type;
use deckmaste_core::Zone;
use deckmaste_plugin::plugin::Plugin;

use crate::agenda::WorkItem;
use crate::event::GameEvent;
use crate::matches as obj_matches;
use crate::object::ObjectId;
use crate::object::ObjectSource;
use crate::player::PlayerId;
use crate::state::GameConfig;
use crate::state::GameState;
use crate::state::PlayerConfig;
use crate::state::StartingPlayer;
use crate::step::Progress;
use crate::step::StepOutcome;

pub(super) fn builtin() -> Plugin {
    Plugin::load(Path::new(env!("CARGO_MANIFEST_DIR")).join("../../plugins/builtin")).unwrap()
}

pub(super) fn canon() -> Plugin {
    Plugin::load_with_sibling_prelude(
        Path::new(env!("CARGO_MANIFEST_DIR")).join("../../plugins/canon"),
    )
    .unwrap()
}

pub(super) fn deck(card: &Arc<Card>, n: usize) -> Vec<Arc<Card>> {
    vec![Arc::clone(card); n]
}

pub(super) fn game() -> GameState {
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

/// Pulls a second creature out of player 0's opening hand, drops it onto
/// the battlefield, and hands it to player 1 — owner stays player 0,
/// controller becomes player 1. Returns the object so tests can read
/// both sides.
pub(super) fn second_bear_to_player_1(state: &mut GameState) -> ObjectId {
    let theirs = *state.zones.hands[0]
        .iter()
        .find(|&&o| obj_matches(state, o, &Predicate::creature()))
        .expect("a second Grizzly Bears in the opening hand");
    state.zones.hands[PlayerId(0).index()].retain(|&o| o != theirs);
    state.objects.obj_mut(theirs).zone = Some(Zone::Battlefield);
    state.objects.obj_mut(theirs).controller = PlayerId(1);
    state.zones.battlefield.push(theirs);
    theirs
}

/// A two-player game; player 0's deck is Grizzly Bears.
/// Returns the state plus a creature object forced onto the battlefield.
pub(super) fn bear_on_field() -> (GameState, ObjectId) {
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
    let bear = *state.zones.hands[0]
        .iter()
        .find(|&&o| obj_matches(&state, o, &Predicate::r#type(Type::Creature)))
        .expect("a Grizzly Bears in the opening hand");
    state.zones.hands[PlayerId(0).index()].retain(|&o| o != bear);
    state.objects.obj_mut(bear).zone = Some(Zone::Battlefield);
    state.zones.battlefield.push(bear);
    (state, bear)
}

/// Two Grizzly Bears (player 0) forced onto the battlefield — `(a, b)`.
/// The attachment subsystem is type-agnostic in Stage 1 ([CR#701.3b]'s
/// type-based attachability is Task 4.x), so two bare permanents exercise
/// the relation/verb/event mechanism directly.
pub(super) fn two_permanents_on_field() -> (GameState, ObjectId, ObjectId) {
    let (mut state, a) = bear_on_field();
    let b = *state.zones.hands[0]
        .iter()
        .find(|&&o| obj_matches(&state, o, &Predicate::creature()))
        .expect("a second Grizzly Bears in the opening hand");
    state.zones.hands[PlayerId(0).index()].retain(|&o| o != b);
    state.objects.obj_mut(b).zone = Some(Zone::Battlefield);
    state.zones.battlefield.push(b);
    (state, a, b)
}

/// Process exactly a test-injected effect's front-scheduled work, WITHOUT
/// advancing the turn structure. `run_effect` schedules its work
/// (`Emit`/`RunEffect`/…) at the front of the agenda; this steps while the
/// front item is such injected work and stops the moment a turn-structure
/// item (`BeginStep`/`CheckSbas`/`OpenPriority`/…) or a decision would run.
/// A test-only driver for sequentially-injected effects: it never parks a
/// priority (so the next `run_effect`'s front work isn't blocked) and never
/// drains the turn loop dry.
pub(super) fn run_injected(state: &mut GameState) {
    for _ in 0..100 {
        let injected = matches!(
            state.agenda.front(),
            Some(
                WorkItem::Emit(_)
                    | WorkItem::RunEffect { .. }
                    | WorkItem::Resolve(_)
                    | WorkItem::ChooseNoteNumber { .. }
                    | WorkItem::ChooseNoteCardName { .. }
                    // A keyword action's finalization watcher is injected
                    // resolution work, like the `Emit` it records ([CR#616.1]).
                    | WorkItem::FinalizeAct { .. }
            )
        );
        if !injected || state.pending.is_some() {
            return;
        }
        let _ = state.step();
    }
}

/// Whether the history log holds a fact matching `pred` (game-wide).
pub(super) fn logged(state: &GameState, pred: impl Fn(&GameEvent) -> bool) -> bool {
    state
        .history
        .scan(deckmaste_core::Lookback::ThisGame, state.turn.turn_number)
        .any(pred)
}

/// Expand a builtin keyword macro invocation to an `Ability::Keyword` — through
/// the SEMANTICS path (`semantics::KeywordAbility` → `lower()`), the path
/// production now takes: lowering erases the `Expanded` wrapper before the
/// engine ever sees the value.
pub(super) fn keyword(invocation: &str) -> Ability {
    use deckmaste_lowering::Lower;
    let semantic: deckmaste_semantics::KeywordAbility =
        builtin().macros.read_str(invocation).unwrap();
    Ability::Keyword(semantic.lower())
}

/// Lower a semantic effect through a synthetic spell ability, then schedule
/// its declared region against `frame`. `target_count` supplies the announce
/// slots used by macro-level fixtures that name `Target(n)` directly.
pub(super) fn schedule_lowered_effect(
    state: &mut GameState,
    effect: deckmaste_semantics::OneShotEffect,
    target_count: usize,
    frame: &crate::stack::ExecutionFrame,
) {
    use deckmaste_lowering::Lower;

    let effect = if target_count == 0 {
        effect
    } else {
        deckmaste_semantics::OneShotEffect::Targeted(deckmaste_semantics::Targeted {
            targets: (0..target_count)
                .map(|_| {
                    deckmaste_semantics::TargetSpec::Target(
                        deckmaste_semantics::Quantity::one(),
                        deckmaste_semantics::Predicate::Any,
                    )
                })
                .collect::<Vec<_>>()
                .into(),
            effect: Arc::new(effect),
        })
    };
    let region = deckmaste_semantics::SpellAbility {
        ability_word: None,
        effect,
    }
    .lower()
    .effect;
    let mut region_frame = frame.clone();
    region_frame.activation = state.enter_region(&region, frame);
    let items = region
        .body
        .iter()
        .cloned()
        .map(|effect| WorkItem::RunEffect {
            effect: Arc::new(effect),
            frame: region_frame.clone(),
        })
        .collect();
    state.schedule_front(items);
}

/// A canon subtype value (with its `confers:` list) by printed name.
pub(super) fn subtype(name: &str) -> Subtype {
    canon()
        .subtypes
        .get(&deckmaste_core::Ident::from(name))
        .unwrap_or_else(|| panic!("canon defines the {name} subtype"))
        .clone()
}

/// Mint a card-backed object directly onto the battlefield (player 0).
pub(super) fn mint_on_field(state: &mut GameState, card: Card) -> ObjectId {
    let cid = state.cards.push(Arc::new(card), PlayerId(0));
    let id = state.objects.mint(
        ObjectSource::Card(cid),
        PlayerId(0),
        Some(Zone::Battlefield),
    );
    state.zones.battlefield.push(id);
    id
}

/// Steps the agenda to a stop (decision / game-over) or until `n` steps
/// elapse, returning the `Progress` trace — the in-crate analogue of
/// `skeleton::drain_progress`.
pub(super) fn drain_progress(state: &mut GameState, n: usize) -> Vec<Progress> {
    let mut out = Vec::new();
    for _ in 0..n {
        match state.step() {
            StepOutcome::Progress(p) => out.push(p),
            StepOutcome::NeedsDecision(_) | StepOutcome::GameOver(_) => break,
        }
    }
    out
}

/// Steps the agenda, PASSING priority for whoever holds it, until `n` steps
/// elapse or a non-priority decision (or game over) stops it. `drain_progress`
/// stops at the first priority window, which is not far enough to see a
/// triggered ability actually resolve.
pub(super) fn drain_passing_priority(state: &mut GameState, n: usize) {
    use crate::decide::Decision;
    use crate::decide::DecisionPointKind;

    for _ in 0..n {
        match state.step() {
            StepOutcome::Progress(_) => {}
            StepOutcome::NeedsDecision(DecisionPointKind::Priority(_)) => {
                if state
                    .submit_decision(Decision::Act(crate::decide::Action::Pass))
                    .is_err()
                {
                    return;
                }
            }
            // [CR#603.3b]: several of one player's triggers going on the stack
            // together are ordered by that player. Any order will do for a
            // fixture that asserts on the outcome of all of them.
            StepOutcome::NeedsDecision(DecisionPointKind::OrderTriggers(order)) => {
                let identity: Vec<usize> = (0..order.triggers.len()).collect();
                if state.submit_decision(Decision::Order(identity)).is_err() {
                    return;
                }
            }
            StepOutcome::NeedsDecision(_) | StepOutcome::GameOver(_) => return,
        }
    }
}

/// Mint a fresh card-backed object into `owner`'s hand. Returns its id.
pub(super) fn mint_in_hand(state: &mut GameState, owner: PlayerId, name: &str) -> ObjectId {
    let cid = state.cards.push(
        Arc::new(Card::Normal(CardFace {
            name: name.into(),
            types: vec![Type::Creature.def()],
            ..CardFace::default()
        })),
        owner,
    );
    let id = state
        .objects
        .mint(ObjectSource::Card(cid), owner, Some(Zone::Hand));
    state.zones.hands[owner.index()].push(id);
    id
}

/// Mint a fully-specified card into `owner`'s hand — abilities and a printed
/// mana cost (so a madness card is a castable creature spell). Returns its id.
pub(super) fn mint_in_hand_with(
    state: &mut GameState,
    owner: PlayerId,
    face: CardFace,
) -> ObjectId {
    let cid = state.cards.push(Arc::new(Card::Normal(face)), owner);
    let id = state
        .objects
        .mint(ObjectSource::Card(cid), owner, Some(Zone::Hand));
    state.zones.hands[owner.index()].push(id);
    id
}

/// Build a one-player game whose deck holds `names` (padded with Bears so
/// the opening draw never empties the library), force each named card onto
/// P0's battlefield, and return their ids.
pub(super) fn battlefield_with(names: &[&str]) -> (GameState, Vec<ObjectId>) {
    let mut deck: Vec<Arc<Card>> = names
        .iter()
        .map(|n| Arc::new(canon().card(n).unwrap().core))
        .collect();
    let bears = Arc::new(canon().card("Grizzly Bears").unwrap().core);
    while deck.len() < 12 {
        deck.push(Arc::clone(&bears));
    }
    let mut state = GameState::new(GameConfig {
        players: vec![PlayerConfig { deck }, PlayerConfig { deck: vec![] }],
        seed: 7,
        starting_life: 20,
        starting_player: StartingPlayer::Fixed(PlayerId(0)),
        sba_rules: vec![],
        conferral_rules: vec![],
        damage_result_rules: vec![],
        counter_decls: std::collections::HashMap::new(),
        subtypes: std::collections::HashMap::new(),
        types: std::collections::HashMap::new(),
    });
    let mut ids = Vec::new();
    for name in names {
        let p = PlayerId(0).index();
        let obj = state.zones.hands[p]
            .iter()
            .copied()
            .find(|&o| {
                state.objects.obj(o).card_id().is_some()
                    && matches!(state.def(o), Card::Normal(f)
        | Card::DoubleFaced { front: f, .. }
        | Card::Split { left: f, .. }
        | Card::Flip { normal: f, .. }
        | Card::Adventurer { normal: f, .. } if &*f.name == *name)
            })
            .or_else(|| {
                state.zones.libraries[p].iter().copied().find(|&o| {
                    state.objects.obj(o).card_id().is_some()
                        && matches!(state.def(o), Card::Normal(f)
        | Card::DoubleFaced { front: f, .. }
        | Card::Split { left: f, .. }
        | Card::Flip { normal: f, .. }
        | Card::Adventurer { normal: f, .. } if &*f.name == *name)
                })
            })
            .unwrap_or_else(|| panic!("no {name} in P0's hand or library"));
        state.zones.hands[p].retain(|&o| o != obj);
        state.zones.libraries[p].retain(|&o| o != obj);
        state.objects.obj_mut(obj).zone = Some(Zone::Battlefield);
        state.zones.battlefield.push(obj);
        ids.push(obj);
    }
    (state, ids)
}

/// The `Fight` grammar macro's expansion ([CR#701.14a]): `Composite Fight`
/// wrapping `If (both fighters are creatures on the battlefield —
/// [CR#701.14b]) (Simultaneously [each deals its power to the OTHER, source
/// = itself])`. Slots `x`/`y` are the two fighters. Mirrors
/// `plugins/builtin/macros/effect/Fight.ron` (the guard's `Permanent` is
/// spelled here as `InZone(Battlefield)`, an equivalent for the test).
pub(super) fn fight_effect(x: &Reference, y: &Reference) -> Instruction {
    use deckmaste_core::Condition;
    use deckmaste_core::Predicate;
    use deckmaste_core::Stat;
    use deckmaste_core::StatePredicate;
    let is_creature = |r: &Reference| {
        Condition::Matches(
            r.clone(),
            Predicate::And(
                vec![
                    Predicate::r#type(Type::Creature),
                    Predicate::State(StatePredicate::InZone(Zone::Battlefield)),
                ]
                .into(),
            ),
        )
    };
    let half = |tgt: &Reference, src: &Reference| {
        Instruction::act(Action::DealDamage(
            src.clone(),
            Count::StatOf(src.clone(), Stat::Power),
            tgt.clone(),
        ))
    };
    Instruction::act(Action::Composite {
        name: deckmaste_core::VerbName::from("Fight"),
        body: Arc::new(Instruction::If(deckmaste_core::If {
            condition: Condition::And(vec![is_creature(x), is_creature(y)].into()),
            then: Arc::new(Instruction::Simultaneously(
                vec![half(y, x), half(x, y)].into(),
            )),
            otherwise: None,
        })),
    })
}
