//! Hand-spelled region witnesses — one test per card for the nineteen
//! witnesses `core-regions-substrate` inherited from the absorbed engine
//! tickets (`docs/decisions/core-explicit-regions.md`).
//!
//! The generated wizards corpus cannot exercise these. Eighteen of the
//! nineteen cards are `.ron.todo` with the witnessed clause parked in an
//! `Unparsed` line, so a corpus sweep that strips those lines sees no
//! region-bearing ability at all for fourteen of them. Each fixture below
//! therefore spells the witnessed ability in semantic RON by hand, lowers it
//! through the production `Plugin::card_from_str` path (read → `lower_card` →
//! `validate_card_regions`), and runs it in a real game, asserting the
//! outcome the absorbed ticket named for that card's family.
//!
//! Two elisions keep each fixture on its witness, and neither touches the
//! ability under test:
//!
//! * A printed MANA COST is replaced by `{0}` unless the ability itself reads
//!   it. A card needs SOME mana cost: having none is an unpayable cost
//!   ([CR#202.1b,118.6]), and an unpayable cost can't be paid ([CR#601.2h]), so
//!   the cast never completes. Which cost it is, though, is announcement-stage
//!   business — a free spell walks the same announce → target → resolve path
//!   ([CR#601.2a..601.2i]). Steel Hellkite keeps its `{X}`, because its body
//!   reads that announced value.
//! * A creature type is spelled only where a filter reads it.
//!
//! Where the grammar (or the engine behind it) cannot yet spell a clause, the
//! test carries `#[ignore]` naming the exact blocker — never a passing test
//! over a different ability.

use std::path::Path;
use std::sync::Arc;

use deckmaste_card::Card;
use deckmaste_core::EndingStep;
use deckmaste_core::PhaseStep;
use deckmaste_core::Zone;
use deckmaste_engine::Action;
use deckmaste_engine::ChooseTargets;
use deckmaste_engine::Decision;
use deckmaste_engine::DecisionPointKind;
use deckmaste_engine::GameConfig;
use deckmaste_engine::GameState;
use deckmaste_engine::ManaProvenance;
use deckmaste_engine::ObjectId;
use deckmaste_engine::PaymentCommand;
use deckmaste_engine::PaymentSubject;
use deckmaste_engine::PlayerConfig;
use deckmaste_engine::PlayerId;
use deckmaste_engine::Priority;
use deckmaste_engine::Progress;
use deckmaste_engine::StartingPlayer;
use deckmaste_engine::StepOutcome;
use deckmaste_engine::WorkItem;
use deckmaste_plugin::plugin::Plugin;

// --- plugins and card construction -------------------------------------------

fn builtin() -> Plugin {
    Plugin::load(Path::new(env!("CARGO_MANIFEST_DIR")).join("../../plugins/builtin")).unwrap()
}

fn canon() -> Plugin {
    Plugin::load_with_sibling_prelude(
        Path::new(env!("CARGO_MANIFEST_DIR")).join("../../plugins/canon"),
    )
    .unwrap()
}

/// A hand-spelled semantic card, read and lowered through the production
/// boundary. A refusal (read, lowering, or region validation) panics here with
/// the card's own source, which is what makes an unspellable clause visible as
/// a fixture failure rather than a silent omission.
fn fixture(source: &str) -> Arc<Card> {
    Arc::new(
        canon()
            .card_from_str(source)
            .unwrap_or_else(|error| panic!("fixture did not load: {error}\n{source}"))
            .core,
    )
}

fn basic(name: &str) -> Arc<Card> {
    Arc::new(builtin().card(name).unwrap().core)
}

fn deck(card: &Arc<Card>, n: usize) -> Vec<Arc<Card>> {
    vec![Arc::clone(card); n]
}

/// A game with the builtin rules-as-data wired in (so lethal-damage and
/// zero-loyalty state-based actions fire) and canon's counter/type registries
/// (so `+1/+1` and `LoyaltyCounter` resolve).
fn game(decks: Vec<Vec<Arc<Card>>>, seed: u64) -> GameState {
    let builtin = builtin();
    let canon = canon();
    GameState::new(GameConfig {
        players: decks
            .into_iter()
            .map(|deck| PlayerConfig { deck })
            .collect(),
        seed,
        starting_life: 20,
        starting_player: StartingPlayer::Fixed(PlayerId(0)),
        sba_rules: builtin.sba_rules.clone(),
        conferral_rules: builtin.conferral_rules.clone(),
        damage_result_rules: builtin.damage_result_rules.clone(),
        counter_decls: canon.counters.clone(),
        subtypes: canon.subtypes.clone(),
        types: canon.types.clone(),
    })
}

// --- object helpers ----------------------------------------------------------

fn face_name(state: &GameState, id: ObjectId) -> &str {
    match state.def(id) {
        Card::Normal(f)
        | Card::DoubleFaced { front: f, .. }
        | Card::Split { left: f, .. }
        | Card::Flip { normal: f, .. }
        | Card::Adventurer { normal: f, .. } => &f.characteristics.name,
    }
}

fn is_card(state: &GameState, id: ObjectId, name: &str) -> bool {
    state
        .objects
        .obj(id)
        .card_id()
        .is_some_and(|_| face_name(state, id) == name)
}

/// Moves the first `name` card out of `player`'s library into their hand. The
/// public `GameState` zone fields make this deterministic placement possible
/// without depending on a random opening seven.
fn into_hand(state: &mut GameState, player: PlayerId, name: &str) -> ObjectId {
    let i = player.index();
    if let Some(&obj) = state.zones.libraries[i]
        .iter()
        .find(|&&o| is_card(state, o, name))
    {
        state.zones.libraries[i].retain(|&o| o != obj);
        state.objects.obj_mut(obj).zone = Some(Zone::Hand);
        state.zones.hands[i].push(obj);
        return obj;
    }
    *state.zones.hands[i]
        .iter()
        .find(|&&o| is_card(state, o, name))
        .unwrap_or_else(|| panic!("a {name} owned by player {}", player.0))
}

/// Moves the first `name` card `player` owns (library, then hand) straight onto
/// the battlefield.
fn onto_battlefield(state: &mut GameState, player: PlayerId, name: &str) -> ObjectId {
    let obj = into_hand(state, player, name);
    let i = player.index();
    state.zones.hands[i].retain(|&o| o != obj);
    state.objects.obj_mut(obj).zone = Some(Zone::Battlefield);
    state.zones.battlefield.push(obj);
    obj
}

/// Moves the first `name` card `player` owns into their graveyard.
fn into_graveyard(state: &mut GameState, player: PlayerId, name: &str) -> ObjectId {
    let obj = into_hand(state, player, name);
    let i = player.index();
    state.zones.hands[i].retain(|&o| o != obj);
    state.objects.obj_mut(obj).zone = Some(Zone::Graveyard);
    state.zones.graveyards[i].push(obj);
    obj
}

/// How many of `ids` are objects whose face name is `name`. A zone change
/// remints the object ([CR#400.7]), so a destination is counted by card, never
/// by the id the card had before it moved.
fn count_named(state: &GameState, ids: &[ObjectId], name: &str) -> usize {
    ids.iter().filter(|&&o| is_card(state, o, name)).count()
}

/// Every battlefield object `player` controls whose face name is `name`.
fn battlefield_named(state: &GameState, player: PlayerId, name: &str) -> Vec<ObjectId> {
    state
        .zones
        .battlefield
        .iter()
        .copied()
        .filter(|&o| state.objects.obj(o).controller == player && is_card(state, o, name))
        .collect()
}

/// `player`'s player-proxy object — what a `TargetOne(Player)` slot announces.
fn proxy(state: &GameState, player: PlayerId) -> ObjectId {
    state.player(player).object
}

// --- driving -----------------------------------------------------------------

fn step_to_stop(state: &mut GameState) -> (Vec<Progress>, StepOutcome) {
    let mut trace = Vec::new();
    loop {
        match state.step() {
            StepOutcome::Progress(p) => trace.push(p),
            stop => return (trace, stop),
        }
    }
}

fn count(n: deckmaste_core::Uint) -> usize {
    usize::try_from(n).expect("a fixture count fits usize")
}

/// The answers a fixture doesn't care about: payments auto-pay, triggers keep
/// their arbitrary order, combat declares nothing, and a `ChooseObjects` prompt
/// takes the first legal minimum (so "each player sacrifices N" picks N of that
/// player's own candidates — the point being *whose* candidates the prompt
/// offers, which the assertions read off the board).
fn routine(state: &GameState, pending: &DecisionPointKind) -> Option<Decision> {
    match pending {
        DecisionPointKind::Priority(_) => Some(Decision::Act(Action::Pass)),
        DecisionPointKind::PayMana(_) => Some(Decision::Pay(state.auto_pay_pending())),
        DecisionPointKind::Payment(_) => state.auto_payment_pending(),
        DecisionPointKind::ChooseManaReversals(prompt) => prompt
            .legal
            .iter()
            .max_by_key(|set| set.len())
            .cloned()
            .map(Decision::ManaReversals),
        DecisionPointKind::OrderTriggers(prompt) => {
            Some(Decision::Order((0..prompt.triggers.len()).collect()))
        }
        DecisionPointKind::DeclareAttackers(_) => Some(Decision::Attackers(Vec::new())),
        DecisionPointKind::DeclareBlockers(_) => Some(Decision::Blocks(Vec::new())),
        DecisionPointKind::DiscardToHandSize(prompt) => Some(Decision::Discard(
            state.zones.hands[prompt.player.index()]
                .iter()
                .copied()
                .take(count(prompt.count))
                .collect(),
        )),
        DecisionPointKind::ChooseObjects(prompt) => Some(Decision::Chosen(
            prompt
                .candidates
                .iter()
                .copied()
                .take(count(prompt.min))
                .collect(),
        )),
        DecisionPointKind::LegendRule(prompt) => prompt
            .candidates
            .first()
            .map(|&id| Decision::Chosen(vec![id])),
        DecisionPointKind::YesNo(_) => Some(Decision::Answer(true)),
        _ => None,
    }
}

/// Steps until `stop` accepts a priority window, answering every decision on
/// the way: the fixture's own `answer` first, then [`routine`]. A prompt
/// neither handles panics, so a fixture never silently skips a choice its
/// assertion depends on.
fn drive<S, A>(state: &mut GameState, stop: S, answer: &mut A) -> Vec<Progress>
where
    S: Fn(&GameState, PlayerId) -> bool,
    A: FnMut(&GameState, &DecisionPointKind) -> Option<Decision>,
{
    let mut trace = Vec::new();
    for _ in 0..2000 {
        let (more, outcome) = step_to_stop(state);
        trace.extend(more);
        let pending = match outcome {
            StepOutcome::NeedsDecision(pending) => pending,
            other => panic!("the game ended before the fixture's stop: {other:?}"),
        };
        if let DecisionPointKind::Priority(Priority { player, .. }) = &pending
            && stop(state, *player)
        {
            return trace;
        }
        let decision = answer(state, &pending)
            .or_else(|| routine(state, &pending))
            .unwrap_or_else(|| panic!("the fixture has no answer for {pending:?}"));
        state
            .submit_decision(decision)
            .unwrap_or_else(|error| panic!("{error} answering {pending:?}"));
    }
    panic!("the fixture never reached its stop");
}

/// Steps until a `ChooseTargets` prompt surfaces, answering routine prompts on
/// the way — the entry a fixture needs when it must submit an ILLEGAL target
/// set and read the refusal, which `drive` would turn into a panic.
fn to_target_prompt(state: &mut GameState) -> ChooseTargets {
    for _ in 0..200 {
        let (_, outcome) = step_to_stop(state);
        let StepOutcome::NeedsDecision(pending) = outcome else {
            panic!("the game ended before a target prompt: {outcome:?}");
        };
        if let DecisionPointKind::ChooseTargets(prompt) = pending {
            return prompt;
        }
        let decision =
            routine(state, &pending).unwrap_or_else(|| panic!("no routine answer for {pending:?}"));
        state
            .submit_decision(decision)
            .expect("a routine answer is legal");
    }
    panic!("no target prompt surfaced");
}

/// Steps until a `Retarget` prompt surfaces, answering routine prompts on the
/// way. Cross-target retarget fixtures need to submit one illegal final set,
/// observe the refusal, then retry the same pending decision.
fn to_retarget_prompt(state: &mut GameState) -> deckmaste_engine::Retarget {
    for _ in 0..200 {
        let (_, outcome) = step_to_stop(state);
        let StepOutcome::NeedsDecision(pending) = outcome else {
            panic!("the game ended before a retarget prompt: {outcome:?}");
        };
        if let DecisionPointKind::Retarget(prompt) = pending {
            return prompt;
        }
        let decision =
            routine(state, &pending).unwrap_or_else(|| panic!("no routine answer for {pending:?}"));
        state
            .submit_decision(decision)
            .expect("a routine answer is legal");
    }
    panic!("no retarget prompt surfaced");
}

/// The `ActivateAbility` action for `object` offered by the priority window in
/// flight — read off the legal list rather than guessing an ability index.
fn activation_of(state: &GameState, object: ObjectId) -> Action {
    let Some(DecisionPointKind::Priority(Priority { legal, .. })) = &state.pending else {
        panic!("activation_of expects a priority window in flight");
    };
    legal
        .iter()
        .find(|action| matches!(action, Action::ActivateAbility { object: o, .. } if *o == object))
        .cloned()
        .unwrap_or_else(|| panic!("no activatable ability of {object:?} is legal: {legal:?}"))
}

/// The declare-attackers answer sending each of `attackers` at the next living
/// opponent's player proxy — the plain "attack the player" declaration
/// ([CR#508.1b]).
fn attack_player(state: &GameState, attackers: &[ObjectId]) -> Decision {
    Decision::Attackers(
        attackers
            .iter()
            .map(|&a| {
                let defender = state.next_live_after(state.objects.obj(a).controller);
                (a, state.player(defender).object)
            })
            .collect(),
    )
}

/// No fixture-specific answer — every prompt takes the routine one.
fn plain(_: &GameState, _: &DecisionPointKind) -> Option<Decision> {
    None
}

/// Runs to `player`'s priority in `phase`.
fn to_phase<A>(state: &mut GameState, player: PlayerId, phase: PhaseStep, answer: &mut A)
where
    A: FnMut(&GameState, &DecisionPointKind) -> Option<Decision>,
{
    drive(state, |s, p| p == player && s.turn.current == phase, answer);
}

/// Runs until the stack is empty again — the "everything announced has
/// resolved" stop.
fn settle<A>(state: &mut GameState, answer: &mut A) -> Vec<Progress>
where
    A: FnMut(&GameState, &DecisionPointKind) -> Option<Decision>,
{
    drive(state, |s, _| s.stack.is_empty(), answer)
}

/// Floats `n` mana of `color` into `player`'s pool and re-opens the priority
/// window so the injected pool shows in the legal list.
fn float(state: &mut GameState, player: PlayerId, color: deckmaste_core::Color, n: u32) {
    state
        .player_mut(player)
        .mana_pool
        .add(color.into(), n, ManaProvenance::default());
    assert!(
        matches!(state.pending, Some(DecisionPointKind::Priority(_))),
        "float expects a priority window in flight"
    );
    state.pending = None;
    state.agenda.push_front(WorkItem::OpenPriority);
}

/// Casts `spell` out of its owner's hand from the priority window in flight and
/// runs until the stack empties again.
fn cast<A>(state: &mut GameState, spell: ObjectId, answer: &mut A) -> Vec<Progress>
where
    A: FnMut(&GameState, &DecisionPointKind) -> Option<Decision>,
{
    state
        .submit_decision(Decision::Act(Action::CastSpell { object: spell }))
        .expect("the fixture spell is castable");
    settle(state, answer)
}

/// Activates ability `index` of `object` from the priority window in flight and
/// runs until the stack empties again.
fn activate<A>(state: &mut GameState, object: ObjectId, index: usize, answer: &mut A)
where
    A: FnMut(&GameState, &DecisionPointKind) -> Option<Decision>,
{
    state
        .submit_decision(Decision::Act(Action::ActivateAbility {
            object,
            ability: index,
        }))
        .expect("the fixture ability is activatable");
    settle(state, answer);
}

/// Moves every card in `player`'s hand to the bottom of their library, so a
/// fixture can state that hand exactly instead of inheriting a random opening
/// seven.
fn empty_hand(state: &mut GameState, player: PlayerId) {
    let i = player.index();
    let held: Vec<ObjectId> = std::mem::take(&mut state.zones.hands[i]);
    for obj in held {
        state.objects.obj_mut(obj).zone = Some(Zone::Library);
        state.zones.libraries[i].push_back(obj);
    }
}

/// An `answer` that supplies one fixed target set to the first `ChooseTargets`
/// prompt and records the legal candidate sets it was offered.
struct Targeting {
    picks: Vec<Vec<ObjectId>>,
    offered: Vec<Vec<Vec<ObjectId>>>,
}

impl Targeting {
    fn new(picks: Vec<Vec<ObjectId>>) -> Targeting {
        Targeting {
            picks,
            offered: Vec::new(),
        }
    }

    fn answer(&mut self, _: &GameState, pending: &DecisionPointKind) -> Option<Decision> {
        match pending {
            DecisionPointKind::ChooseTargets(ChooseTargets { legal, .. }) => {
                self.offered.push(legal.clone());
                Some(Decision::Targets(self.picks.clone()))
            }
            _ => None,
        }
    }

    /// The candidate set the engine offered for slot `slot` of the first
    /// target prompt, sorted for a stable comparison.
    fn slot(&self, slot: usize) -> Vec<ObjectId> {
        let mut set = self.offered.first().expect("a target prompt happened")[slot].clone();
        set.sort_unstable();
        set
    }
}

fn sorted(mut ids: Vec<ObjectId>) -> Vec<ObjectId> {
    ids.sort_unstable();
    ids
}

// --- shared fixture cards ----------------------------------------------------

/// A plain 2/2 with no abilities — the sacrifice/discard/return fodder the
/// per-player fixtures count.
const BEAR: &str = r#"Normal(name: "Witness Bear", mana_cost: [Generic(0)],
    types: [Creature], power: 2, toughness: 2)"#;

/// A 1/4 — a toughness no other number in the Tribute to Hunger fixture can be
/// confused with.
const TOUGH: &str = r#"Normal(name: "Witness Wall", mana_cost: [Generic(0)],
    types: [Creature], power: 1, toughness: 4)"#;

/// A noncreature, nonland card — what Duress may take and Pilfer may also take.
/// Mana value 0.
const TRINKET: &str = r#"Normal(name: "Witness Trinket", mana_cost: [Generic(0)],
    types: [Artifact])"#;

/// A noncreature permanent of the other type Trygon Predator can name.
const CHARM: &str = r#"Normal(name: "Witness Charm", mana_cost: [Generic(0)],
    types: [Enchantment])"#;

/// Mana value 2 — the value Steel Hellkite's {X} names.
const RELIC2: &str = r#"Normal(name: "Witness Relic", mana_cost: [Generic(2)],
    types: [Artifact])"#;

/// Mana value 3 — the near miss the same activation must leave alone.
const RELIC3: &str = r#"Normal(name: "Witness Reliquary", mana_cost: [Generic(3)],
    types: [Artifact])"#;

/// A synthetic cross-target spell used only for engine-path witnesses. The
/// second creature must be controlled by someone other than the first one's
/// controller, so its filter reads slot 0's announced register.
const CROSS_TARGET_RETURN: &str = r#"Normal(
    name: "Cross-Target Return",
    mana_cost: [Generic(0)],
    types: [Instant],
    abilities: [
        Spell(effect: Targeted(
            targets: [
                TargetOne(Creature),
                TargetOne(And([
                    Creature,
                    Not(ControlledBy(Ref(ControllerOf(Target(0))))),
                ])),
            ],
            effect: Sequentially([Move(Target(0), Hand), Move(Target(1), Hand)]))),
    ])"#;

/// A fixture-local spelling of "{T}: Choose new targets for target spell."
const CROSS_TARGET_RETARGETER: &str = r#"Normal(
    name: "Cross-Target Retargeter",
    mana_cost: [Generic(0)],
    types: [Creature],
    abilities: [
        Activated(
            cost: [Tap],
            effect: Targeted(
                targets: [TargetOne(Kind(Spell))],
                effect: Retarget(mode: ChooseNew, of: Target(0), by: You))),
    ],
    power: 1,
    toughness: 1)"#;

// --- per-player choice -------------------------------------------------------

/// Burglar Rat — "When this creature enters, each opponent discards a card."
///
/// `engine-candidate-frame-bindings`, per-player choice: the `Each` body is a
/// region whose loop element is the bound opponent, and the discard reads that
/// element twice — the hand searched is theirs, and the chooser is them
/// ([CR#701.9b]). Two opponents make the per-element binding observable: each
/// discards one card of their own, and the controller discards nothing.
#[test]
fn burglar_rat_each_opponent_discards_from_their_own_hand() {
    const RAT: &str = r#"Normal(
        name: "Burglar Rat",
        mana_cost: [Generic(0)],
        types: [Creature],
        subtypes: [Rat],
        abilities: [
            Triggered(
                event: ThisEnters,
                effect: Each(
                    binder: Existing(SelectAll(OpponentOf(Ref(You)))),
                    effect: Discards(It, 1))),
        ],
        power: 1,
        toughness: 1)"#;

    let rat = fixture(RAT);
    let bear = fixture(BEAR);
    let mut p0 = deck(&rat, 4);
    p0.extend(deck(&bear, 16));
    let mut state = game(vec![p0, deck(&bear, 20), deck(&bear, 20)], 11);
    let spell = into_hand(&mut state, PlayerId(0), "Burglar Rat");

    to_phase(
        &mut state,
        PlayerId(0),
        PhaseStep::PrecombatMain,
        &mut plain,
    );
    let hands: Vec<usize> = (0..3).map(|i| state.zones.hands[i].len()).collect();
    cast(&mut state, spell, &mut plain);

    assert_eq!(
        state.zones.graveyards[1].len(),
        1,
        "the first opponent discarded exactly one card"
    );
    assert_eq!(
        state.zones.graveyards[2].len(),
        1,
        "the second opponent discarded exactly one card"
    );
    assert!(
        state.zones.graveyards[0].is_empty(),
        "the controller is not an opponent of themself and discards nothing"
    );
    assert_eq!(
        state.zones.hands[1].len(),
        hands[1] - 1,
        "the discard came out of that opponent's own hand"
    );
    assert_eq!(
        state.zones.hands[2].len(),
        hands[2] - 1,
        "…and out of the other opponent's own hand"
    );
    assert_eq!(
        state.zones.hands[0].len(),
        hands[0] - 1,
        "the controller's hand lost only the Rat they cast"
    );
}

/// Arbiter of Woe — "When this creature enters, each opponent discards a card
/// and loses 2 life. You draw a card and gain 2 life."
///
/// Per-player choice with a two-instruction loop body: both instructions read
/// the same loop element, and the two trailing instructions read the ability's
/// controller parameter instead. The card's "sacrifice a creature" additional
/// cost is an announcement-stage clause and is not spelled here.
#[test]
fn arbiter_of_woe_drains_each_opponent_and_pays_its_controller() {
    const ARBITER: &str = r#"Normal(
        name: "Arbiter of Woe",
        mana_cost: [Generic(0)],
        types: [Creature],
        subtypes: [Demon],
        abilities: [
            Keyword(Flying),
            Triggered(
                event: ThisEnters,
                effect: Sequentially([
                    Each(
                        binder: Existing(SelectAll(OpponentOf(Ref(You)))),
                        effect: Sequentially([Discards(It, 1), ChangeLife(It, Down(2))])),
                    Draw(1),
                    ChangeLife(You, Up(2)),
                ])),
        ],
        power: 5,
        toughness: 4)"#;

    let arbiter = fixture(ARBITER);
    let bear = fixture(BEAR);
    let mut p0 = deck(&arbiter, 4);
    p0.extend(deck(&bear, 16));
    let mut state = game(vec![p0, deck(&bear, 20), deck(&bear, 20)], 5);
    let spell = into_hand(&mut state, PlayerId(0), "Arbiter of Woe");

    to_phase(
        &mut state,
        PlayerId(0),
        PhaseStep::PrecombatMain,
        &mut plain,
    );
    let hands: Vec<usize> = (0..3).map(|i| state.zones.hands[i].len()).collect();
    cast(&mut state, spell, &mut plain);

    for opponent in 1..3 {
        assert_eq!(
            state.zones.graveyards[opponent].len(),
            1,
            "opponent {opponent} discarded one card"
        );
        assert_eq!(
            state.players[opponent].life, 18,
            "opponent {opponent} lost 2 life"
        );
    }
    assert_eq!(
        state.players[0].life, 22,
        "the controller gained 2 life, and lost none"
    );
    assert_eq!(
        state.zones.hands[0].len(),
        hands[0],
        "the controller drew one card back for the Arbiter that left their hand"
    );
    assert!(
        state.zones.graveyards[0].is_empty(),
        "the controller discarded nothing"
    );
}

/// Bloodtithe Collector — "When this creature enters, if an opponent lost life
/// this turn, each opponent discards a card."
///
/// The intervening-if is a history read ([CR#608.2a,608.2i]) over the same
/// opponent-of-the-controller predicate the loop uses. Both legs run in one
/// test: with no life loss the trigger does nothing, and after an opponent has
/// lost life this turn the same trigger drains their hand.
#[test]
fn bloodtithe_collector_discards_only_after_an_opponent_lost_life() {
    const COLLECTOR: &str = r#"Normal(
        name: "Bloodtithe Collector",
        mana_cost: [Generic(0)],
        types: [Creature],
        subtypes: [Vampire],
        abilities: [
            Keyword(Flying),
            Triggered(
                event: ThisEnters,
                condition: Happened(
                    event: LifeLost(who: OpponentOf(Ref(You))),
                    within: ThisTurn),
                effect: Each(
                    binder: Existing(SelectAll(OpponentOf(Ref(You)))),
                    effect: Discards(It, 1))),
        ],
        power: 3,
        toughness: 4)"#;
    const DRAIN: &str = r#"Normal(
        name: "Witness Drain",
        mana_cost: [Generic(0)],
        types: [Sorcery],
        abilities: [
            Spell(effect: Each(
                binder: Existing(SelectAll(OpponentOf(Ref(You)))),
                effect: ChangeLife(It, Down(1)))),
        ])"#;

    fn setup(seed: u64) -> GameState {
        let collector = fixture(COLLECTOR);
        let drain = fixture(DRAIN);
        let bear = fixture(BEAR);
        let mut p0 = deck(&collector, 4);
        p0.extend(deck(&drain, 4));
        p0.extend(deck(&bear, 16));
        game(vec![p0, deck(&bear, 24)], seed)
    }

    // Leg one: nothing has happened this turn, so the intervening "if" fails.
    let mut quiet = setup(3);
    let collector = into_hand(&mut quiet, PlayerId(0), "Bloodtithe Collector");
    to_phase(
        &mut quiet,
        PlayerId(0),
        PhaseStep::PrecombatMain,
        &mut plain,
    );
    let quiet_hand = quiet.zones.hands[1].len();
    cast(&mut quiet, collector, &mut plain);
    assert_eq!(
        quiet.zones.hands[1].len(),
        quiet_hand,
        "no opponent lost life this turn, so the trigger did nothing"
    );
    assert!(quiet.zones.graveyards[1].is_empty());

    // Leg two: the same trigger, after a drain has put a `LifeLost` fact for an
    // opponent into this turn's history.
    let mut bled = setup(3);
    let drain = into_hand(&mut bled, PlayerId(0), "Witness Drain");
    let collector = into_hand(&mut bled, PlayerId(0), "Bloodtithe Collector");
    to_phase(&mut bled, PlayerId(0), PhaseStep::PrecombatMain, &mut plain);
    cast(&mut bled, drain, &mut plain);
    assert_eq!(
        bled.players[1].life, 19,
        "the drain cost the opponent 1 life"
    );
    let bled_hand = bled.zones.hands[1].len();
    cast(&mut bled, collector, &mut plain);
    assert_eq!(
        bled.zones.hands[1].len(),
        bled_hand - 1,
        "an opponent lost life this turn, so each opponent discards"
    );
    assert_eq!(bled.zones.graveyards[1].len(), 1);
}

/// Blasphemous Edict — "Each player sacrifices thirteen creatures of their
/// choice."
///
/// The whole-table form of the per-player choice: the loop element is a PLAYER,
/// and the sacrifice pool inside the body is filtered by that element's
/// control, so each player sacrifices thirteen of their OWN creatures
/// ([CR#701.21a]) and never reaches across the table. The alternative "{B} if
/// there are thirteen or more creatures" cost is an announcement-stage clause
/// and is not spelled here.
#[test]
fn blasphemous_edict_each_player_sacrifices_thirteen_of_their_own() {
    const EDICT: &str = r#"Normal(
        name: "Blasphemous Edict",
        mana_cost: [Generic(0)],
        types: [Sorcery],
        abilities: [
            Spell(effect: Each(
                binder: Existing(SelectAll(Player)),
                effect: With(
                    binder: Choose(
                        quantity: Exactly(13),
                        filter: And([Creature, ControlledBy(Ref(It))]),
                        by: It),
                    body: Each(
                        binder: Existing(They),
                        effect: Sacrifice(ControllerOf(It), It))))),
        ])"#;

    let edict = fixture(EDICT);
    let bear = fixture(BEAR);
    let mut p0 = deck(&edict, 2);
    p0.extend(deck(&bear, 34));
    let mut state = game(vec![p0, deck(&bear, 36)], 9);
    let spell = into_hand(&mut state, PlayerId(0), "Blasphemous Edict");
    for _ in 0..14 {
        onto_battlefield(&mut state, PlayerId(0), "Witness Bear");
    }
    for _ in 0..13 {
        onto_battlefield(&mut state, PlayerId(1), "Witness Bear");
    }

    to_phase(
        &mut state,
        PlayerId(0),
        PhaseStep::PrecombatMain,
        &mut plain,
    );
    let graveyards: Vec<usize> = (0..2).map(|i| state.zones.graveyards[i].len()).collect();
    cast(&mut state, spell, &mut plain);

    assert_eq!(
        battlefield_named(&state, PlayerId(0), "Witness Bear").len(),
        1,
        "the caster sacrificed thirteen of their fourteen creatures"
    );
    assert!(
        battlefield_named(&state, PlayerId(1), "Witness Bear").is_empty(),
        "the opponent sacrificed all thirteen of theirs"
    );
    assert_eq!(
        state.zones.graveyards[0].len(),
        graveyards[0] + 13 + 1,
        "thirteen sacrificed creatures plus the spell itself"
    );
    assert_eq!(
        state.zones.graveyards[1].len(),
        graveyards[1] + 13,
        "the opponent's thirteen went to their own graveyard"
    );
}

// --- whole-zone selection and target-relative filters
// -------------------------

/// Angel of Finality — "When this creature enters, exile target player's
/// graveyard."
///
/// Two families at once. WHOLE-ZONE selection: the effect names no card, it
/// names a zone — and exiling is exactly a move into the exile zone
/// ([CR#701.13a]), so the group verb takes everything the selection finds.
/// TRIGGER-RELATIVE target: the selection's owner filter reads
/// the trigger's announced target register ([CR#603.3d] chooses it at
/// placement), so the untargeted player's graveyard is untouched.
#[test]
fn angel_of_finality_exiles_only_the_target_players_graveyard() {
    const ANGEL: &str = r#"Normal(
        name: "Angel of Finality",
        mana_cost: [Generic(0)],
        types: [Creature],
        subtypes: [Angel],
        abilities: [
            Keyword(Flying),
            Triggered(
                event: ThisEnters,
                effect: Targeted(
                    targets: [TargetOne(Player)],
                    effect: MoveGroup(
                        group: SelectAll(And([InZone(Graveyard), Owner(Ref(Target(0)))])),
                        arrangement: AnyOrder,
                        to: Exile))),
        ],
        power: 3,
        toughness: 4)"#;

    let angel = fixture(ANGEL);
    let bear = fixture(BEAR);
    let mut p0 = deck(&angel, 4);
    p0.extend(deck(&bear, 20));
    let mut state = game(vec![p0, deck(&bear, 24)], 17);
    let spell = into_hand(&mut state, PlayerId(0), "Angel of Finality");
    for _ in 0..2 {
        into_graveyard(&mut state, PlayerId(0), "Witness Bear");
    }
    for _ in 0..3 {
        into_graveyard(&mut state, PlayerId(1), "Witness Bear");
    }

    to_phase(
        &mut state,
        PlayerId(0),
        PhaseStep::PrecombatMain,
        &mut plain,
    );
    let victim = proxy(&state, PlayerId(1));
    let mut targeting = Targeting::new(vec![vec![victim]]);
    {
        let mut answer = |s: &GameState, p: &DecisionPointKind| targeting.answer(s, p);
        cast(&mut state, spell, &mut answer);
    }

    assert_eq!(
        targeting.slot(0),
        sorted(vec![proxy(&state, PlayerId(0)), victim]),
        "the target slot offers both players"
    );
    assert!(
        state.zones.graveyards[1].is_empty(),
        "the whole graveyard left, not one card of it"
    );
    assert_eq!(
        count_named(&state, &state.zones.exile, "Witness Bear"),
        3,
        "all three of the target player's graveyard cards are in exile"
    );
    assert_eq!(
        state.zones.graveyards[0].len(),
        2,
        "the untargeted player's graveyard is untouched"
    );
}

/// River's Rebuke — "Return all nonland permanents target player controls to
/// their owner's hand."
///
/// Target-relative filter: one `SelectAll` whose control clause reads the
/// announced target register. Everything the target controls that isn't a land
/// bounces; their land stays, and so does every permanent anyone else controls.
#[test]
fn rivers_rebuke_bounces_only_the_target_players_nonlands() {
    const REBUKE: &str = r#"Normal(
        name: "River's Rebuke",
        mana_cost: [Generic(0)],
        types: [Sorcery],
        abilities: [
            Spell(effect: Targeted(
                targets: [TargetOne(Player)],
                effect: MoveGroup(
                    group: SelectAll(And([
                        Permanent,
                        Not(Type(Land)),
                        ControlledBy(Ref(Target(0))),
                    ])),
                    arrangement: AnyOrder,
                    to: Hand))),
        ])"#;

    let rebuke = fixture(REBUKE);
    let bear = fixture(BEAR);
    let forest = basic("Forest");
    let mut p0 = deck(&rebuke, 2);
    p0.extend(deck(&bear, 20));
    let mut p1 = deck(&bear, 16);
    p1.extend(deck(&forest, 8));
    let mut state = game(vec![p0, p1], 21);
    let spell = into_hand(&mut state, PlayerId(0), "River's Rebuke");
    let mine = onto_battlefield(&mut state, PlayerId(0), "Witness Bear");
    for _ in 0..2 {
        onto_battlefield(&mut state, PlayerId(1), "Witness Bear");
    }
    let their_land = onto_battlefield(&mut state, PlayerId(1), "Forest");

    to_phase(
        &mut state,
        PlayerId(0),
        PhaseStep::PrecombatMain,
        &mut plain,
    );
    let held = count_named(&state, &state.zones.hands[1], "Witness Bear");
    let victim = proxy(&state, PlayerId(1));
    let mut targeting = Targeting::new(vec![vec![victim]]);
    {
        let mut answer = |s: &GameState, p: &DecisionPointKind| targeting.answer(s, p);
        cast(&mut state, spell, &mut answer);
    }

    assert!(
        battlefield_named(&state, PlayerId(1), "Witness Bear").is_empty(),
        "both nonland permanents the target controlled left the battlefield"
    );
    assert_eq!(
        count_named(&state, &state.zones.hands[1], "Witness Bear"),
        held + 2,
        "…and landed in their owner's hand"
    );
    assert!(
        state.zones.battlefield.contains(&their_land),
        "the target's land is not a nonland permanent and stays"
    );
    assert!(
        state.zones.battlefield.contains(&mine),
        "a permanent the caster controls is not the target's and stays"
    );
}

/// Tribute to Hunger — "Target opponent sacrifices a creature of their choice.
/// You gain life equal to that creature's toughness."
///
/// Target-relative filter for the sacrifice pool AND the reference-resolution
/// witness: the toughness read happens after the creature has left the
/// battlefield, so it must come off last known information ([CR#608.2h]) — the
/// 1/4 fixture makes 4 the only number that can produce the observed life
/// total.
#[test]
fn tribute_to_hunger_gains_the_sacrificed_creatures_last_known_toughness() {
    const TRIBUTE: &str = r#"Normal(
        name: "Tribute to Hunger",
        mana_cost: [Generic(0)],
        types: [Instant],
        abilities: [
            Spell(effect: Targeted(
                targets: [TargetOne(And([Player, OpponentOf(Ref(You))]))],
                effect: With(
                    binder: Choose(
                        quantity: Exactly(1),
                        filter: And([Creature, ControlledBy(Ref(Target(0)))]),
                        by: Target(0)),
                    body: Each(
                        binder: Existing(They),
                        effect: Sequentially([
                            Sacrifice(ControllerOf(It), It),
                            ChangeLife(You, Up(StatOf(It, Toughness))),
                        ]))))),
        ])"#;

    let tribute = fixture(TRIBUTE);
    let bear = fixture(BEAR);
    let wall = fixture(TOUGH);
    let mut p0 = deck(&tribute, 2);
    p0.extend(deck(&bear, 20));
    let mut p1 = deck(&wall, 4);
    p1.extend(deck(&bear, 20));
    let mut state = game(vec![p0, p1], 33);
    let spell = into_hand(&mut state, PlayerId(0), "Tribute to Hunger");
    let mine = onto_battlefield(&mut state, PlayerId(0), "Witness Bear");
    let theirs = onto_battlefield(&mut state, PlayerId(1), "Witness Wall");

    to_phase(
        &mut state,
        PlayerId(0),
        PhaseStep::PrecombatMain,
        &mut plain,
    );
    let victim = proxy(&state, PlayerId(1));
    let mut offered: Vec<(PlayerId, Vec<ObjectId>)> = Vec::new();
    let mut targeting = Targeting::new(vec![vec![victim]]);
    {
        let mut answer = |s: &GameState, p: &DecisionPointKind| match p {
            DecisionPointKind::ChooseObjects(prompt) => {
                offered.push((prompt.player, prompt.candidates.clone()));
                None
            }
            other => targeting.answer(s, other),
        };
        cast(&mut state, spell, &mut answer);
    }

    assert_eq!(
        offered,
        vec![(PlayerId(1), vec![theirs])],
        "the target opponent chooses, from their own creatures only \
         (the caster's {mine:?} is not offered)"
    );
    assert!(
        battlefield_named(&state, PlayerId(1), "Witness Wall").is_empty(),
        "the chosen creature left the battlefield"
    );
    assert_eq!(
        count_named(&state, &state.zones.graveyards[1], "Witness Wall"),
        1,
        "…sacrificed to its owner's graveyard ([CR#701.21a])"
    );
    assert_eq!(
        state.players[0].life, 24,
        "the caster gained 4 — the departed creature's last known toughness"
    );
}

/// Duress — "Target opponent reveals their hand. You choose a noncreature,
/// nonland card from it. That player discards that card."
///
/// Target-relative filter with a SPLIT of roles: the pool is the announced
/// target's hand, but the chooser is the ability's controller — two different
/// registers read by one binder. The caster's own noncreature, nonland cards
/// (Duress itself is one) are never offered, and neither is the target's
/// creature or land.
///
/// "Reveals their hand" is spelled as a reveal of each card in it: revealing is
/// a per-card act — showing that card to all players ([CR#701.20a]) — and the
/// grammar's `Reveal` names one object, so the hand is the loop, not the
/// operand.
#[test]
fn duress_offers_the_caster_only_the_targets_noncreature_nonlands() {
    const DURESS: &str = r#"Normal(
        name: "Duress",
        mana_cost: [Generic(0)],
        types: [Sorcery],
        abilities: [
            Spell(effect: Targeted(
                targets: [TargetOne(And([Player, OpponentOf(Ref(You))]))],
                effect: Sequentially([
                    Each(
                        binder: Existing(SelectAll(InHand(Target(0)))),
                        effect: Reveal(what: It)),
                    With(
                        binder: ChooseOne(
                            filter: And([
                                InHand(Target(0)),
                                Not(Type(Creature)),
                                Not(Type(Land)),
                            ]),
                            by: You),
                        body: Composite(name: Discard, body: Move(It, Graveyard))),
                ]))),
        ])"#;

    let duress = fixture(DURESS);
    let bear = fixture(BEAR);
    let trinket = fixture(TRINKET);
    let forest = basic("Forest");
    let mut p0 = deck(&duress, 4);
    p0.extend(deck(&bear, 20));
    let mut p1 = deck(&bear, 12);
    p1.extend(deck(&trinket, 6));
    p1.extend(deck(&forest, 6));
    let mut state = game(vec![p0, p1], 41);
    let spell = into_hand(&mut state, PlayerId(0), "Duress");
    empty_hand(&mut state, PlayerId(1));
    into_hand(&mut state, PlayerId(1), "Witness Bear");
    into_hand(&mut state, PlayerId(1), "Witness Bear");
    let taken = into_hand(&mut state, PlayerId(1), "Witness Trinket");
    into_hand(&mut state, PlayerId(1), "Forest");

    to_phase(
        &mut state,
        PlayerId(0),
        PhaseStep::PrecombatMain,
        &mut plain,
    );
    assert!(
        state.zones.hands[0]
            .iter()
            .filter(|&&o| is_card(&state, o, "Duress"))
            .count()
            >= 2,
        "the caster holds spare Duresses — noncreature, nonland cards of their own"
    );
    let victim = proxy(&state, PlayerId(1));
    let mut offered: Vec<(PlayerId, Vec<ObjectId>)> = Vec::new();
    let mut targeting = Targeting::new(vec![vec![victim]]);
    {
        let mut answer = |s: &GameState, p: &DecisionPointKind| match p {
            DecisionPointKind::ChooseObjects(prompt) => {
                offered.push((prompt.player, prompt.candidates.clone()));
                None
            }
            other => targeting.answer(s, other),
        };
        cast(&mut state, spell, &mut answer);
    }

    assert_eq!(
        offered,
        vec![(PlayerId(0), vec![taken])],
        "the caster chooses, and the only candidate is the target's one \
         noncreature nonland card"
    );
    assert_eq!(
        count_named(&state, &state.zones.graveyards[1], "Witness Trinket"),
        1,
        "the target discarded the chosen card ([CR#701.9a])"
    );
    assert_eq!(
        count_named(&state, &state.zones.hands[1], "Witness Bear"),
        2,
        "their creatures stayed in hand"
    );
}

/// Pilfer — "Target opponent reveals their hand. You choose a nonland card from
/// it. That player discards that card."
///
/// Duress's twin with one clause removed: the same split-role target-relative
/// binder, over a pool that now admits creatures. The offered set is exactly
/// the target's three nonland cards — the difference from Duress is visible in
/// the candidate list, not just in the outcome.
#[test]
fn pilfer_offers_the_caster_every_nonland_card_in_the_targets_hand() {
    const PILFER: &str = r#"Normal(
        name: "Pilfer",
        mana_cost: [Generic(0)],
        types: [Sorcery],
        abilities: [
            Spell(effect: Targeted(
                targets: [TargetOne(And([Player, OpponentOf(Ref(You))]))],
                effect: Sequentially([
                    Each(
                        binder: Existing(SelectAll(InHand(Target(0)))),
                        effect: Reveal(what: It)),
                    With(
                        binder: ChooseOne(
                            filter: And([InHand(Target(0)), Not(Type(Land))]),
                            by: You),
                        body: Composite(name: Discard, body: Move(It, Graveyard))),
                ]))),
        ])"#;

    let pilfer = fixture(PILFER);
    let bear = fixture(BEAR);
    let trinket = fixture(TRINKET);
    let forest = basic("Forest");
    let mut p0 = deck(&pilfer, 4);
    p0.extend(deck(&bear, 20));
    let mut p1 = deck(&bear, 12);
    p1.extend(deck(&trinket, 6));
    p1.extend(deck(&forest, 6));
    let mut state = game(vec![p0, p1], 41);
    let spell = into_hand(&mut state, PlayerId(0), "Pilfer");
    empty_hand(&mut state, PlayerId(1));
    let first = into_hand(&mut state, PlayerId(1), "Witness Bear");
    let second = into_hand(&mut state, PlayerId(1), "Witness Bear");
    let third = into_hand(&mut state, PlayerId(1), "Witness Trinket");
    into_hand(&mut state, PlayerId(1), "Forest");

    to_phase(
        &mut state,
        PlayerId(0),
        PhaseStep::PrecombatMain,
        &mut plain,
    );
    let victim = proxy(&state, PlayerId(1));
    let mut offered: Vec<(PlayerId, Vec<ObjectId>)> = Vec::new();
    let mut targeting = Targeting::new(vec![vec![victim]]);
    {
        let mut answer = |s: &GameState, p: &DecisionPointKind| match p {
            DecisionPointKind::ChooseObjects(prompt) => {
                offered.push((prompt.player, prompt.candidates.clone()));
                None
            }
            other => targeting.answer(s, other),
        };
        cast(&mut state, spell, &mut answer);
    }

    let (chooser, mut candidates) = offered
        .first()
        .cloned()
        .expect("the choice from the target's hand happened");
    candidates.sort_unstable();
    assert_eq!(chooser, PlayerId(0), "the caster chooses");
    assert_eq!(
        candidates,
        sorted(vec![first, second, third]),
        "every nonland card in the target's hand is offered, and the land is not"
    );
    assert_eq!(
        count_named(&state, &state.zones.hands[1], "Forest"),
        1,
        "the land stayed in hand"
    );
    assert_eq!(
        state.zones.graveyards[1].len(),
        1,
        "exactly one card was discarded"
    );
}

/// Liliana, Dreadhorde General — the −4: "Each player sacrifices two creatures
/// of their choice."
///
/// The witnessed ability is the per-player-choice loyalty ability; the card's
/// other three abilities are not spelled here. Activating it from a loyalty
/// cost puts the same `Each`-over-players region behind an activated ability's
/// announcement, and each player empties two of their OWN creatures.
#[test]
fn liliana_dreadhorde_general_minus_four_makes_each_player_sacrifice_two() {
    const LILIANA: &str = r#"Normal(
        name: "Liliana, Dreadhorde General",
        mana_cost: [Generic(0)],
        supertypes: [Legendary],
        types: [Planeswalker],
        abilities: [
            LoyaltyMinus(n: 4, effect: Each(
                binder: Existing(SelectAll(Player)),
                effect: With(
                    binder: Choose(
                        quantity: Exactly(2),
                        filter: And([Creature, ControlledBy(Ref(It))]),
                        by: It),
                    body: Each(
                        binder: Existing(They),
                        effect: Sacrifice(ControllerOf(It), It))))),
        ],
        loyalty: 6)"#;

    let liliana = fixture(LILIANA);
    let bear = fixture(BEAR);
    let mut p0 = deck(&liliana, 2);
    p0.extend(deck(&bear, 22));
    let mut state = game(vec![p0, deck(&bear, 24)], 55);
    let spell = into_hand(&mut state, PlayerId(0), "Liliana, Dreadhorde General");
    for _ in 0..3 {
        onto_battlefield(&mut state, PlayerId(0), "Witness Bear");
        onto_battlefield(&mut state, PlayerId(1), "Witness Bear");
    }

    to_phase(
        &mut state,
        PlayerId(0),
        PhaseStep::PrecombatMain,
        &mut plain,
    );
    cast(&mut state, spell, &mut plain);
    let walker = battlefield_named(&state, PlayerId(0), "Liliana, Dreadhorde General")
        .first()
        .copied()
        .expect("Liliana resolved onto the battlefield");
    assert_eq!(
        state
            .objects
            .obj(walker)
            .counters
            .values()
            .copied()
            .sum::<u32>(),
        6,
        "she entered with six loyalty"
    );

    activate(&mut state, walker, 0, &mut plain);

    assert_eq!(
        battlefield_named(&state, PlayerId(0), "Witness Bear").len(),
        1,
        "the controller sacrificed two of their three creatures"
    );
    assert_eq!(
        battlefield_named(&state, PlayerId(1), "Witness Bear").len(),
        1,
        "so did the opponent, out of their own three"
    );
    assert_eq!(
        count_named(&state, &state.zones.graveyards[0], "Witness Bear"),
        2,
        "the controller's two went to the controller's graveyard"
    );
    assert_eq!(
        count_named(&state, &state.zones.graveyards[1], "Witness Bear"),
        2,
        "…and the opponent's two to theirs"
    );
}

/// Tinybones, Bauble Burglar — "{3}{B}, {T}: Each opponent discards a card.
/// Activate only as a sorcery."
///
/// The witnessed ability is the activated per-player discard; the card's stash
/// trigger and play permission are not spelled here. Behind an activated
/// ability's own region, the loop element still supplies both the hand searched
/// and the player choosing, for every opponent independently.
#[test]
fn tinybones_bauble_burglar_activated_ability_drains_each_opponent() {
    const TINYBONES: &str = r#"Normal(
        name: "Tinybones, Bauble Burglar",
        mana_cost: [Generic(0)],
        supertypes: [Legendary],
        types: [Creature],
        subtypes: [Rogue],
        abilities: [
            Activated(
                cost: [Tap],
                window: SorcerySpeed,
                effect: Each(
                    binder: Existing(SelectAll(OpponentOf(Ref(You)))),
                    effect: Discards(It, 1))),
        ],
        power: 1,
        toughness: 3)"#;

    let tinybones = fixture(TINYBONES);
    let bear = fixture(BEAR);
    let mut p0 = deck(&tinybones, 2);
    p0.extend(deck(&bear, 22));
    let mut state = game(vec![p0, deck(&bear, 24), deck(&bear, 24)], 63);
    let skeleton = onto_battlefield(&mut state, PlayerId(0), "Tinybones, Bauble Burglar");
    state.objects.obj_mut(skeleton).summoning_sick = false;

    to_phase(
        &mut state,
        PlayerId(0),
        PhaseStep::PrecombatMain,
        &mut plain,
    );
    let hands: Vec<usize> = (0..3).map(|i| state.zones.hands[i].len()).collect();
    activate(&mut state, skeleton, 0, &mut plain);

    assert!(
        state.objects.obj(skeleton).tapped,
        "the {{T}} cost was paid"
    );
    for (opponent, before) in hands.iter().enumerate().take(3).skip(1) {
        assert_eq!(
            state.zones.hands[opponent].len(),
            before - 1,
            "opponent {opponent} discarded from their own hand"
        );
        assert_eq!(
            state.zones.graveyards[opponent].len(),
            1,
            "…into their own graveyard"
        );
    }
    assert_eq!(
        state.zones.hands[0].len(),
        hands[0],
        "the activating player discards nothing"
    );
}

// --- trigger-relative targets and event filters
// -------------------------------

/// Trygon Predator — "Whenever this creature deals combat damage to a player,
/// you may destroy target artifact or enchantment that player controls."
///
/// Trigger-relative target: the target filter's control clause reads the event
/// PATIENT ([CR#120.3] — the damage recipient), so the candidate set offered at
/// placement ([CR#603.3d]) is exactly the damaged player's artifacts and
/// enchantments. An identical artifact under the attacker's own control is the
/// control: it is never offered.
#[test]
fn trygon_predator_targets_only_the_damaged_players_permanents() {
    const TRYGON: &str = r#"Normal(
        name: "Trygon Predator",
        mana_cost: [Generic(0)],
        types: [Creature],
        subtypes: [Beast],
        abilities: [
            Keyword(Flying),
            Triggered(
                event: DealsCombatDamage(Ref(This), Player),
                effect: Targeted(
                    targets: [TargetOne(And([
                        Permanent,
                        Or([Type(Artifact), Type(Enchantment)]),
                        ControlledBy(Ref(EventPatient)),
                    ]))],
                    effect: May(who: You, effect: Destroy(Target(0))))),
        ],
        power: 2,
        toughness: 3)"#;

    let trygon = fixture(TRYGON);
    let trinket = fixture(TRINKET);
    let charm = fixture(CHARM);
    let bear = fixture(BEAR);
    let mut p0 = deck(&trygon, 2);
    p0.extend(deck(&trinket, 4));
    p0.extend(deck(&bear, 18));
    let mut p1 = deck(&trinket, 6);
    p1.extend(deck(&charm, 6));
    p1.extend(deck(&bear, 12));
    let mut state = game(vec![p0, p1], 71);
    let predator = onto_battlefield(&mut state, PlayerId(0), "Trygon Predator");
    state.objects.obj_mut(predator).summoning_sick = false;
    let mine = onto_battlefield(&mut state, PlayerId(0), "Witness Trinket");
    let their_artifact = onto_battlefield(&mut state, PlayerId(1), "Witness Trinket");
    let their_enchantment = onto_battlefield(&mut state, PlayerId(1), "Witness Charm");

    let mut targeting = Targeting::new(vec![vec![their_artifact]]);
    let mut declared = false;
    {
        let mut answer = |s: &GameState, p: &DecisionPointKind| match p {
            DecisionPointKind::DeclareAttackers(_) if !declared => {
                declared = true;
                Some(attack_player(s, &[predator]))
            }
            other => targeting.answer(s, other),
        };
        to_phase(
            &mut state,
            PlayerId(0),
            PhaseStep::PostcombatMain,
            &mut answer,
        );
    }

    assert!(declared, "the Predator attacked");
    assert_eq!(
        state.players[1].life, 18,
        "two unblocked combat damage reached the defending player"
    );
    assert_eq!(
        targeting.slot(0),
        sorted(vec![their_artifact, their_enchantment]),
        "only the damaged player's artifact and enchantment are offered — \
         not the attacker's own {mine:?}"
    );
    assert!(
        !state.zones.battlefield.contains(&their_artifact),
        "the chosen target was destroyed"
    );
    assert!(
        state.zones.battlefield.contains(&mine),
        "the attacker's own artifact is untouched"
    );
    assert!(
        state.zones.battlefield.contains(&their_enchantment),
        "one target, one destruction"
    );
}

/// Predator Ooze — "Whenever a creature dealt damage by this creature this turn
/// dies, put a +1/+1 counter on this creature," beside its attack trigger and
/// indestructibility.
///
/// The `engine-snapshot-frame-references` witness: the dies-trigger's event
/// filter carries a per-candidate HISTORY predicate ([CR#608.2i]) that reads
/// the GONE creature as the candidate and the watching Ooze as the damage
/// source. A creature the Ooze killed in combat fires it; a creature that dies
/// the same turn without having been damaged by the Ooze does not.
///
/// The RECIPIENT half is the load-bearing read, and both its sides are
/// last-known information: the recorded damage fact's patient is the snapshot
/// taken when the fact was logged, and the candidate the `Where` region binds
/// is the snapshot of a creature that has already left the battlefield —
/// exactly when this trigger fires. [CR#608.2i] is what makes that legible:
/// the damaged creature need not still be in the zone it was in, as long as it
/// was there when the damage was dealt.
#[test]
fn predator_ooze_counts_only_creatures_it_damaged_this_turn() {
    const OOZE: &str = r#"Normal(
        name: "Predator Ooze",
        mana_cost: [Generic(0)],
        types: [Creature],
        abilities: [
            Keyword(Indestructible),
            Triggered(event: ThisAttacks, effect: PutCounters(This, P1P1Counter, 1)),
            Triggered(
                event: ZoneChange(
                    what: And([
                        Creature,
                        Where(Happened(
                            event: Damage(source: Ref(This), to: Ref(It)),
                            within: ThisTurn)),
                    ]),
                    from: Battlefield,
                    to: Graveyard),
                effect: PutCounters(This, P1P1Counter, 1)),
        ],
        power: 1,
        toughness: 1)"#;
    const BOLT: &str = r#"Normal(
        name: "Witness Doom",
        mana_cost: [Generic(0)],
        types: [Instant],
        abilities: [
            Spell(effect: Targeted(targets: [TargetOne(Creature)], effect: Destroy(Target(0)))),
        ])"#;

    let ooze = fixture(OOZE);
    let doom = fixture(BOLT);
    let bear = fixture(BEAR);
    let mut p0 = deck(&ooze, 2);
    p0.extend(deck(&doom, 4));
    p0.extend(deck(&bear, 18));
    let mut state = game(vec![p0, deck(&bear, 24)], 77);
    let slime = onto_battlefield(&mut state, PlayerId(0), "Predator Ooze");
    state.objects.obj_mut(slime).summoning_sick = false;
    let victim = onto_battlefield(&mut state, PlayerId(1), "Witness Bear");
    let bystander = onto_battlefield(&mut state, PlayerId(1), "Witness Bear");
    // Seeded BEFORE the run to combat, so the control's cast is offered by the
    // priority window the fixture stops on: `Priority::legal` is snapshotted
    // when the window opens, and `into_hand` moves a card behind the engine's
    // back.
    let doom_spell = into_hand(&mut state, PlayerId(0), "Witness Doom");

    let mut declared = false;
    let mut blocked = false;
    {
        let mut answer = |s: &GameState, p: &DecisionPointKind| match p {
            DecisionPointKind::DeclareAttackers(_) if !declared => {
                declared = true;
                Some(attack_player(s, &[slime]))
            }
            DecisionPointKind::DeclareBlockers(_) if !blocked => {
                blocked = true;
                Some(Decision::Blocks(vec![(victim, slime)]))
            }
            _ => None,
        };
        to_phase(
            &mut state,
            PlayerId(0),
            PhaseStep::PostcombatMain,
            &mut answer,
        );
    }

    assert!(declared && blocked, "the Ooze attacked and was blocked");
    assert!(
        state.zones.battlefield.contains(&slime),
        "the indestructible Ooze survived lethal combat damage ([CR#702.12b])"
    );
    assert!(
        !state.zones.battlefield.contains(&victim),
        "the blocker took two damage from the pumped Ooze and died"
    );
    assert_eq!(
        state
            .objects
            .obj(slime)
            .counters
            .values()
            .copied()
            .sum::<u32>(),
        2,
        "one counter for attacking, one for the creature it damaged dying"
    );

    // The control: a creature that dies this same turn WITHOUT the Ooze having
    // damaged it leaves the count alone.
    let mut targeting = Targeting::new(vec![vec![bystander]]);
    {
        let mut answer = |s: &GameState, p: &DecisionPointKind| targeting.answer(s, p);
        cast(&mut state, doom_spell, &mut answer);
    }
    assert!(
        !state.zones.battlefield.contains(&bystander),
        "the second creature died this turn too"
    );
    assert_eq!(
        state
            .objects
            .obj(slime)
            .counters
            .values()
            .copied()
            .sum::<u32>(),
        2,
        "…but the Ooze never damaged it, so no counter"
    );
}

/// Steel Hellkite — "{X}: Destroy each nonland permanent with mana value X
/// whose controller was dealt combat damage by this creature this turn.
/// Activate only once each turn."
///
/// The cross-reference witness in its hardest form: one per-candidate
/// predicate reads THREE registers — the announced X (the mana value to match),
/// the source (the damage dealer), and the candidate itself (whose controller
/// is the damage recipient, a history read, [CR#608.2i]). Two near misses are
/// the controls: the same mana value under an undamaged controller, and a
/// different mana value under the damaged one.
///
/// Run for both faithful spellings of "whose controller": the
/// relation-predicate detour and the DERIVED reference `Ref(ControllerOf(It))`
/// — the natural one, which `engine-derived-reference-in-filters` taught the
/// frameless matcher to resolve.
#[test]
fn steel_hellkite_destroys_by_announced_x_and_who_it_damaged() {
    for damaged in ["Controls(Ref(It))", "Ref(ControllerOf(It))"] {
        steel_hellkite_case(damaged);
    }
}

fn steel_hellkite_case(damaged: &str) {
    let hellkite = format!(
        r#"Normal(
        name: "Steel Hellkite",
        mana_cost: [Generic(0)],
        types: [Artifact, Creature],
        abilities: [
            Keyword(Flying),
            Activated(
                cost: [Mana([Variable])],
                limits: [OncePerTurn],
                effect: Each(
                    binder: Existing(SelectAll(And([
                        Permanent,
                        Not(Type(Land)),
                        Stat(ManaValue, Eq, X),
                        Where(Happened(
                            event: Damage(
                                source: Ref(This),
                                to: {damaged},
                                combat: true),
                            within: ThisTurn)),
                    ]))),
                    effect: Destroy(It))),
        ],
        power: 5,
        toughness: 5)"#
    );

    let hellkite = fixture(&hellkite);
    let relic2 = fixture(RELIC2);
    let relic3 = fixture(RELIC3);
    let bear = fixture(BEAR);
    let mut p0 = deck(&hellkite, 2);
    p0.extend(deck(&relic2, 4));
    p0.extend(deck(&bear, 18));
    let mut p1 = deck(&relic2, 6);
    p1.extend(deck(&relic3, 6));
    p1.extend(deck(&bear, 12));
    let mut state = game(vec![p0, p1], 91);
    let dragon = onto_battlefield(&mut state, PlayerId(0), "Steel Hellkite");
    state.objects.obj_mut(dragon).summoning_sick = false;
    let safe_value = onto_battlefield(&mut state, PlayerId(0), "Witness Relic");
    let doomed = onto_battlefield(&mut state, PlayerId(1), "Witness Relic");
    let wrong_value = onto_battlefield(&mut state, PlayerId(1), "Witness Reliquary");

    let mut declared = false;
    {
        let mut answer = |s: &GameState, p: &DecisionPointKind| match p {
            DecisionPointKind::DeclareAttackers(_) if !declared => {
                declared = true;
                Some(attack_player(s, &[dragon]))
            }
            _ => None,
        };
        to_phase(
            &mut state,
            PlayerId(0),
            PhaseStep::PostcombatMain,
            &mut answer,
        );
    }
    assert_eq!(
        state.players[1].life, 15,
        "five unblocked combat damage reached the defending player"
    );

    float(&mut state, PlayerId(0), deckmaste_core::Color::Green, 2);
    to_phase(
        &mut state,
        PlayerId(0),
        PhaseStep::PostcombatMain,
        &mut plain,
    );
    let action = activation_of(&state, dragon);
    state
        .submit_decision(Decision::Act(action))
        .expect("the {X} ability is activatable");
    {
        let mut answer = |_: &GameState, p: &DecisionPointKind| match p {
            DecisionPointKind::ChooseXValue(_) => Some(Decision::XValue(2)),
            _ => None,
        };
        settle(&mut state, &mut answer);
    }

    assert!(
        !state.zones.battlefield.contains(&doomed),
        "the damaged player's mana-value-2 nonland permanent was destroyed"
    );
    assert!(
        state.zones.battlefield.contains(&wrong_value),
        "their mana-value-3 permanent is the wrong X and survives"
    );
    assert!(
        state.zones.battlefield.contains(&safe_value),
        "the activating player's own mana-value-2 permanent survives — \
         its controller was not dealt combat damage by the Hellkite"
    );
    assert!(
        state.zones.battlefield.contains(&dragon),
        "the Hellkite's own mana value is 0, so it does not destroy itself"
    );
}

/// Castability precheck for a telescoping target list ([CR#601.2c]). Both
/// creatures in play have the same controller, so no complete announcement of
/// `CROSS_TARGET_RETURN` exists: whichever creature slot 0 names, slot 1 has
/// no candidate under a different controller. The spell must therefore be
/// absent from the priority window rather than leading to an unanswerable
/// target prompt.
#[test]
fn cross_target_spell_without_a_complete_announcement_is_not_castable() {
    let spell_card = fixture(CROSS_TARGET_RETURN);
    let bear = fixture(BEAR);
    let mut p0 = deck(&spell_card, 2);
    p0.extend(deck(&bear, 22));
    let mut state = game(vec![p0, deck(&bear, 24)], 98);
    let spell = into_hand(&mut state, PlayerId(0), "Cross-Target Return");
    onto_battlefield(&mut state, PlayerId(0), "Witness Bear");
    onto_battlefield(&mut state, PlayerId(0), "Witness Bear");

    to_phase(
        &mut state,
        PlayerId(0),
        PhaseStep::PrecombatMain,
        &mut plain,
    );
    let Some(DecisionPointKind::Priority(Priority { legal, .. })) = &state.pending else {
        panic!("the fixture stopped outside priority: {:?}", state.pending);
    };
    assert!(
        !legal
            .iter()
            .any(|action| matches!(action, Action::CastSpell { object } if *object == spell)),
        "a spell with no legal complete target announcement is not offered"
    );
}

struct CrossRetargetFixture {
    state: GameState,
    spell: ObjectId,
    current_second: ObjectId,
    replacement_first: ObjectId,
    replacement_second: ObjectId,
    prompt: deckmaste_engine::Retarget,
}

/// Drives a hand-spelled cross-target spell and retarget ability to the
/// `Retarget` decision. When `invalidate_current_second` is set, direct fixture
/// setup changes that target's controller after its legal announcement but
/// before the retarget effect resolves, making it already illegal under the
/// current prefix.
fn cross_retarget_fixture(seed: u64, invalidate_current_second: bool) -> CrossRetargetFixture {
    let spell_card = fixture(CROSS_TARGET_RETURN);
    let retargeter_card = fixture(CROSS_TARGET_RETARGETER);
    let bear = fixture(BEAR);
    let mut p0 = deck(&spell_card, 2);
    p0.extend(deck(&bear, 22));
    let mut p1 = deck(&retargeter_card, 2);
    p1.extend(deck(&bear, 22));
    let mut state = game(vec![p0, p1], seed);
    let spell = into_hand(&mut state, PlayerId(0), "Cross-Target Return");
    let retargeter = onto_battlefield(&mut state, PlayerId(1), "Cross-Target Retargeter");
    state.objects.obj_mut(retargeter).summoning_sick = false;
    let current_first = onto_battlefield(&mut state, PlayerId(0), "Witness Bear");
    let replacement_second = onto_battlefield(&mut state, PlayerId(0), "Witness Bear");
    let current_second = onto_battlefield(&mut state, PlayerId(1), "Witness Bear");
    let replacement_first = onto_battlefield(&mut state, PlayerId(1), "Witness Bear");

    to_phase(
        &mut state,
        PlayerId(0),
        PhaseStep::PrecombatMain,
        &mut plain,
    );
    state
        .submit_decision(Decision::Act(Action::CastSpell { object: spell }))
        .expect("the spell has a legal initial cross-target announcement");
    let prompt = to_target_prompt(&mut state);
    assert!(prompt.legal[0].contains(&current_first));
    assert!(prompt.legal[1].contains(&current_second));
    state
        .submit_decision(Decision::Targets(vec![
            vec![current_first],
            vec![current_second],
        ]))
        .expect("the initial targets have different controllers");
    drive(
        &mut state,
        |s, player| player == PlayerId(0) && s.stack.len() == 1,
        &mut plain,
    );

    state
        .submit_decision(Decision::Act(Action::Pass))
        .expect("P0 passes with the spell on the stack");
    drive(
        &mut state,
        |s, player| player == PlayerId(1) && s.stack.len() == 1,
        &mut plain,
    );
    let activate = activation_of(&state, retargeter);
    state
        .submit_decision(Decision::Act(activate))
        .expect("the retarget ability is activatable");
    let prompt = to_target_prompt(&mut state);
    assert!(prompt.legal[0].contains(&spell));
    state
        .submit_decision(Decision::Targets(vec![vec![spell]]))
        .expect("the committed spell is a legal target");
    drive(
        &mut state,
        |s, player| player == PlayerId(1) && s.stack.len() == 2,
        &mut plain,
    );
    if invalidate_current_second {
        state.objects.obj_mut(current_second).controller = PlayerId(0);
    }
    state
        .submit_decision(Decision::Act(Action::Pass))
        .expect("P1 passes");
    drive(
        &mut state,
        |s, player| player == PlayerId(0) && s.stack.len() == 2,
        &mut plain,
    );
    state
        .submit_decision(Decision::Act(Action::Pass))
        .expect("P0 passes");

    let prompt = to_retarget_prompt(&mut state);
    CrossRetargetFixture {
        state,
        spell,
        current_second,
        replacement_first,
        replacement_second,
        prompt,
    }
}

/// Choosing new targets evaluates only the FINAL target set ([CR#115.7e]),
/// but changing an earlier target may not make an unchanged later target
/// illegal ([CR#115.7d]). The initial pair is P0/P1. Changing slot 0 to a P1
/// creature while retaining slot 1's P1 creature is rejected; changing slot 1
/// to the P0 alternative is accepted. The accepted later target was not legal
/// under the entry's old slot-0 value, so its presence in the surfaced union
/// also witnesses cross-prefix re-enumeration.
#[test]
fn cross_target_retarget_rederives_later_slots_from_the_proposed_prefix() {
    let CrossRetargetFixture {
        mut state,
        spell,
        current_second,
        replacement_first,
        replacement_second,
        prompt,
    } = cross_retarget_fixture(99, false);
    assert!(
        prompt.legal[1].contains(&replacement_second),
        "the later-slot menu includes a target reachable after changing slot 0"
    );
    let refused = state
        .submit_decision(Decision::Targets(vec![
            vec![replacement_first],
            vec![current_second],
        ]))
        .expect_err("changing slot 0 must not invalidate unchanged slot 1");
    assert!(matches!(
        refused,
        deckmaste_engine::DecisionError::Illegal { .. }
    ));
    state
        .submit_decision(Decision::Targets(vec![
            vec![replacement_first],
            vec![replacement_second],
        ]))
        .expect("the final targets have different controllers");
    assert_eq!(
        state
            .stack
            .iter()
            .find(|entry| entry.id == spell)
            .expect("the retargeted spell remains on the stack")
            .targets,
        vec![vec![replacement_first], vec![replacement_second]]
    );
}

/// [CR#115.7d] separately permits an unchanged target that was ALREADY
/// illegal. Here control of slot 1 changes after announcement, making it
/// illegal under the current P0 slot 0. Changing slot 0 to another P0 creature
/// leaves slot 1 illegal, but does not CAUSE that illegality, so the retarget is
/// legal and the current target remains in place.
#[test]
fn cross_target_retarget_keeps_a_later_target_that_was_already_illegal() {
    let CrossRetargetFixture {
        mut state,
        spell,
        current_second,
        replacement_second,
        prompt,
        ..
    } = cross_retarget_fixture(100, true);
    assert!(
        prompt.legal[1].contains(&current_second),
        "the union rule keeps the current illegal target in the menu"
    );
    state
        .submit_decision(Decision::Targets(vec![
            vec![replacement_second],
            vec![current_second],
        ]))
        .expect("an already-illegal unchanged target remains keepable");
    assert_eq!(
        state
            .stack
            .iter()
            .find(|entry| entry.id == spell)
            .expect("the retargeted spell remains on the stack")
            .targets,
        vec![vec![replacement_second], vec![current_second]]
    );
}

/// Run Away Together — "Choose two target creatures controlled by different
/// players. Return those creatures to their owners' hands."
///
/// Cross-target: the second slot's filter reads the FIRST slot's announced
/// register, so a second creature under the first target's controller is not a
/// legal announcement ([CR#601.2c]).
///
/// The second slot's surfaced menu is the union over possible first targets;
/// the whole submitted proposal is then re-derived in declaration order, so a
/// same-controller pair is refused as an illegal announcement.
///
/// Both faithful spellings of "controlled by a different player" are exercised:
/// the relation-predicate detour, and the DERIVED reference
/// `Ref(ControllerOf(Target(0)))` — the natural spelling of "the controller of
/// the first target", which `engine-derived-reference-in-filters` taught the
/// frameless matcher to resolve. Same card, same refusal, either way.
#[test]
fn run_away_together_refuses_two_targets_under_one_controller() {
    for other_controller in [
        "ControlledBy(Controls(Ref(Target(0))))",
        "ControlledBy(Ref(ControllerOf(Target(0))))",
    ] {
        run_away_together_case(other_controller);
    }
}

fn run_away_together_case(other_controller: &str) {
    let run_away = format!(
        r#"Normal(
        name: "Run Away Together",
        mana_cost: [Generic(0)],
        types: [Instant],
        abilities: [
            Spell(effect: Targeted(
                targets: [
                    TargetOne(Creature),
                    TargetOne(And([
                        Creature,
                        Not({other_controller}),
                    ])),
                ],
                effect: Sequentially([Move(Target(0), Hand), Move(Target(1), Hand)]))),
        ])"#
    );

    let run_away = fixture(&run_away);
    let bear = fixture(BEAR);
    let mut p0 = deck(&run_away, 2);
    p0.extend(deck(&bear, 22));
    let mut state = game(vec![p0, deck(&bear, 24)], 101);
    let spell = into_hand(&mut state, PlayerId(0), "Run Away Together");
    let mine = onto_battlefield(&mut state, PlayerId(0), "Witness Bear");
    let also_mine = onto_battlefield(&mut state, PlayerId(0), "Witness Bear");
    let theirs = onto_battlefield(&mut state, PlayerId(1), "Witness Bear");

    to_phase(
        &mut state,
        PlayerId(0),
        PhaseStep::PrecombatMain,
        &mut plain,
    );
    let held = count_named(&state, &state.zones.hands[0], "Witness Bear");
    state
        .submit_decision(Decision::Act(Action::CastSpell { object: spell }))
        .expect("castable");
    let _prompt = to_target_prompt(&mut state);
    let refused = state
        .submit_decision(Decision::Targets(vec![vec![mine], vec![also_mine]]))
        .expect_err("two creatures under one controller is not a legal announcement");
    assert!(
        matches!(refused, deckmaste_engine::DecisionError::Illegal { .. }),
        "the refusal is an illegality, not a shape error: {refused:?}"
    );
    state
        .submit_decision(Decision::Targets(vec![vec![mine], vec![theirs]]))
        .expect("two creatures under different controllers is legal");
    settle(&mut state, &mut plain);

    assert!(
        battlefield_named(&state, PlayerId(1), "Witness Bear").is_empty(),
        "the opponent's chosen creature went home"
    );
    assert_eq!(
        battlefield_named(&state, PlayerId(0), "Witness Bear").len(),
        1,
        "one of the caster's two creatures went home, the untargeted one stayed"
    );
    assert_eq!(
        count_named(&state, &state.zones.hands[0], "Witness Bear"),
        held + 1,
        "each creature went to ITS OWN owner's hand"
    );
}

/// Fiery Annihilation — "Fiery Annihilation deals 5 damage to target creature.
/// Exile up to one target Equipment attached to that creature." (Its third
/// sentence, a floating death replacement, belongs to the replacement family
/// and is not spelled here.)
///
/// Cross-target: the second slot's filter is relative to the first slot's
/// announced creature, so an Equipment attached to a DIFFERENT creature is not
/// offered for it.
///
/// The same announcement order as Run Away Together, with a positive filter
/// rather than a negated one: `AttachedTo(Ref(Target(0)))` reads the creature
/// slot's register, so an Equipment on a DIFFERENT creature is not a legal
/// announcement.
///
/// Re-spelled from the offered-menu form the `core-regions-witness-fixtures`
/// round wrote (`targeting.slot(1) == vec![worn]`), which no engine can
/// satisfy: the menu is computed before the player announces anything, so the
/// creature the Equipment must be attached to is not yet chosen when it is
/// built. What the menu can honestly offer is the union over the creature
/// slot's own candidates; the cross-reference is a REFUSAL at announcement,
/// spelled here exactly as Run Away Together spells its own.
#[test]
fn fiery_annihilation_exiles_only_equipment_on_the_damaged_creature() {
    const FIERY: &str = r#"Normal(
        name: "Fiery Annihilation",
        mana_cost: [Generic(0)],
        types: [Instant],
        abilities: [
            Spell(effect: Targeted(
                targets: [
                    TargetOne(Creature),
                    Target(AtMost(1), And([
                        Permanent,
                        Subtype(Equipment),
                        AttachedTo(Ref(Target(0))),
                    ])),
                ],
                effect: Sequentially([
                    DealDamage(This, 5, Target(0)),
                    Move(Target(1), Exile),
                ]))),
        ])"#;
    const GEAR: &str = r#"Normal(name: "Witness Gear", mana_cost: [Generic(0)],
        types: [Artifact], subtypes: [Equipment])"#;

    let fiery = fixture(FIERY);
    let gear = fixture(GEAR);
    let bear = fixture(BEAR);
    let mut p0 = deck(&fiery, 2);
    p0.extend(deck(&bear, 22));
    let mut p1 = deck(&gear, 6);
    p1.extend(deck(&bear, 18));
    let mut state = game(vec![p0, p1], 111);
    let spell = into_hand(&mut state, PlayerId(0), "Fiery Annihilation");
    let equipped = onto_battlefield(&mut state, PlayerId(1), "Witness Bear");
    let other = onto_battlefield(&mut state, PlayerId(1), "Witness Bear");
    let worn = onto_battlefield(&mut state, PlayerId(1), "Witness Gear");
    let elsewhere = onto_battlefield(&mut state, PlayerId(1), "Witness Gear");
    state.objects.obj_mut(worn).attached_to = Some(equipped);
    state.objects.obj_mut(elsewhere).attached_to = Some(other);

    to_phase(
        &mut state,
        PlayerId(0),
        PhaseStep::PrecombatMain,
        &mut plain,
    );
    state
        .submit_decision(Decision::Act(Action::CastSpell { object: spell }))
        .expect("castable");
    let _prompt = to_target_prompt(&mut state);
    let refused = state
        .submit_decision(Decision::Targets(vec![vec![equipped], vec![elsewhere]]))
        .expect_err("an Equipment on another creature is not attached to the damaged one");
    assert!(
        matches!(refused, deckmaste_engine::DecisionError::Illegal { .. }),
        "the refusal is an illegality, not a shape error: {refused:?}"
    );
    state
        .submit_decision(Decision::Targets(vec![vec![equipped], vec![worn]]))
        .expect("the Equipment attached to the announced creature is legal");
    settle(&mut state, &mut plain);
    assert!(
        !state.zones.battlefield.contains(&equipped),
        "five damage killed the 2/2 it targeted"
    );
    assert_eq!(
        count_named(&state, &state.zones.exile, "Witness Gear"),
        1,
        "the attached Equipment was exiled"
    );
    assert!(
        state.zones.battlefield.contains(&elsewhere),
        "the other creature's Equipment is untouched"
    );
}

/// Painful Quandary — "Whenever an opponent casts a spell, that player loses 5
/// life unless they discard a card."
///
/// Trigger-relative reference on the PLAYER channel: both the punished player
/// and the payer are the caster — the spell's controller from the moment it was
/// proposed ([CR#601.2a]) — read out of the trigger's own region, not out of
/// the ability's controller. Both branches of the punisher ([CR#118.12a]) run
/// in one test.
///
/// `Unless` builds `May { effect: Pay(cost) }`; the discard cost's chooser
/// declares its register in that enclosing effect region, just as a chooser in
/// an ability's announced `cost:` field does.
#[test]
fn painful_quandary_punishes_the_caster_it_triggered_on() {
    const QUANDARY: &str = r#"Normal(
        name: "Painful Quandary",
        mana_cost: [Generic(0)],
        types: [Enchantment],
        abilities: [
            Triggered(
                event: Cast(who: OpponentOf(Ref(You)), what: Any),
                effect: Unless(
                    effect: ChangeLife(EventActor, Down(5)),
                    who: EventActor,
                    unless: [DiscardCards(1)])),
        ])"#;
    const CANTRIP: &str = r#"Normal(
        name: "Witness Cantrip",
        mana_cost: [Generic(0)],
        types: [Instant],
        abilities: [Spell(effect: Sequentially([]))])"#;

    fn setup() -> (GameState, ObjectId) {
        let quandary = fixture(QUANDARY);
        let cantrip = fixture(CANTRIP);
        let bear = fixture(BEAR);
        let mut p0 = deck(&quandary, 2);
        p0.extend(deck(&bear, 22));
        let mut p1 = deck(&cantrip, 4);
        p1.extend(deck(&bear, 20));
        let mut state = game(vec![p0, p1], 121);
        onto_battlefield(&mut state, PlayerId(0), "Painful Quandary");
        let spell = into_hand(&mut state, PlayerId(1), "Witness Cantrip");
        (state, spell)
    }

    // Refusing the cost: the caster loses 5 life and keeps their hand.
    let (mut refused, spell) = setup();
    to_phase(
        &mut refused,
        PlayerId(1),
        PhaseStep::PrecombatMain,
        &mut plain,
    );
    let held = refused.zones.hands[1].len();
    {
        let mut answer = |_: &GameState, p: &DecisionPointKind| match p {
            DecisionPointKind::Payment(prompt)
                if matches!(prompt.subject, PaymentSubject::Effect { .. }) =>
            {
                Some(Decision::Payment(PaymentCommand::DeclinePayment))
            }
            _ => None,
        };
        cast(&mut refused, spell, &mut answer);
    }
    assert_eq!(
        refused.players[1].life, 15,
        "the caster — not the enchantment's controller — lost 5 life"
    );
    assert_eq!(
        refused.players[0].life, 20,
        "the enchantment's controller lost nothing"
    );
    assert_eq!(
        refused.zones.hands[1].len(),
        held - 1,
        "their hand lost only the spell they cast"
    );

    // Paying it: the caster discards instead, and keeps their life total.
    let (mut paid, spell) = setup();
    to_phase(&mut paid, PlayerId(1), PhaseStep::PrecombatMain, &mut plain);
    let held = paid.zones.hands[1].len();
    cast(&mut paid, spell, &mut plain);
    assert_eq!(paid.players[1].life, 20, "paying the cost avoids the loss");
    assert_eq!(
        paid.zones.hands[1].len(),
        held - 2,
        "the spell cast plus the card discarded to the punisher"
    );
    assert_eq!(
        paid.zones.graveyards[1].len(),
        2,
        "the discarded card and the resolved cantrip"
    );
}

// --- witnesses whose ability has no faithful spelling yet
// ---------------------

/// Deadly Brew — "Each player sacrifices a creature or planeswalker of their
/// choice. If you sacrificed a permanent this way, you may return another
/// permanent card from your graveyard to your hand."
///
/// The first sentence is the per-player choice this card was inherited for, and
/// it is spelled below. The second has no faithful spelling. "This way" names
/// the group the first sentence produced, but that group is chosen inside the
/// `Each`-over-players body — a nested region — and a register does not escape
/// its region. The one channel that crosses a region boundary is linked memory
/// ([CR#607.1], ADR law 8), which is a per-OBJECT link between two abilities
/// printed on one card, not a way for a later clause of the SAME resolution to
/// name an earlier clause's per-player choice. `Noting` binds only the newest
/// antecedent of the region it wraps, so it cannot reach in either. The
/// assertions below therefore describe the whole card, and the fixture cannot
/// yet satisfy them.
#[test]
#[ignore = "grammar: no way to name the group an enclosed per-player region \
            chose (`this way`), so Deadly Brew's second sentence has no \
            faithful spelling"]
fn deadly_brew_lets_the_caster_rebuy_after_the_table_sacrifices() {
    const BREW: &str = r#"Normal(
        name: "Deadly Brew",
        mana_cost: [Generic(0)],
        types: [Sorcery],
        abilities: [
            Spell(effect: Each(
                binder: Existing(SelectAll(Player)),
                effect: With(
                    binder: Choose(
                        quantity: Exactly(1),
                        filter: And([
                            Permanent,
                            Or([Type(Creature), Type(Planeswalker)]),
                            ControlledBy(Ref(It)),
                        ]),
                        by: It),
                    body: Each(
                        binder: Existing(They),
                        effect: Sacrifice(ControllerOf(It), It))))),
        ])"#;

    let brew = fixture(BREW);
    let bear = fixture(BEAR);
    let mut p0 = deck(&brew, 2);
    p0.extend(deck(&bear, 22));
    let mut state = game(vec![p0, deck(&bear, 24)], 131);
    let spell = into_hand(&mut state, PlayerId(0), "Deadly Brew");
    onto_battlefield(&mut state, PlayerId(0), "Witness Bear");
    onto_battlefield(&mut state, PlayerId(1), "Witness Bear");
    let rebuy = into_graveyard(&mut state, PlayerId(0), "Witness Bear");

    to_phase(
        &mut state,
        PlayerId(0),
        PhaseStep::PrecombatMain,
        &mut plain,
    );
    let held = count_named(&state, &state.zones.hands[0], "Witness Bear");
    cast(&mut state, spell, &mut plain);

    assert!(
        battlefield_named(&state, PlayerId(0), "Witness Bear").is_empty(),
        "the caster sacrificed their own creature"
    );
    assert!(
        battlefield_named(&state, PlayerId(1), "Witness Bear").is_empty(),
        "so did the opponent, from their own board"
    );
    assert!(
        !state.zones.graveyards[0].contains(&rebuy),
        "the caster sacrificed a permanent this way, so they may return \
         ANOTHER permanent card from their graveyard"
    );
    assert_eq!(
        count_named(&state, &state.zones.hands[0], "Witness Bear"),
        held + 1,
        "…and it went to their hand"
    );
}

/// Perforating Artist — "Raid — At the beginning of your end step, if you
/// attacked this turn, each opponent loses 3 life unless that player sacrifices
/// a nonland permanent of their choice or discards a card."
///
/// The Raid intervening-if and the per-opponent loop are spelled below. The
/// punisher is not, and cannot be: a semantic `Cost` is `[CostComponent]`, a
/// CONJUNCTION, and `Unless` takes exactly one of them — there is no
/// disjunctive cost node, so "sacrifices … OR discards a card" has no faithful
/// spelling. Either disjunct alone would also hit the second blocker
/// `painful_quandary_punishes_the_caster_it_triggered_on` names: a
/// decision-bearing cost inside a body's `Pay` declares no register in the
/// enclosing region and is refused at load. The assertions describe the whole
/// card, and the fixture cannot yet satisfy them.
#[test]
#[ignore = "grammar: a semantic `Cost` is a conjunction of components with no \
            disjunction, so `unless that player sacrifices … or discards a \
            card` has no faithful spelling"]
fn perforating_artist_taxes_each_opponent_who_will_not_pay() {
    const ARTIST: &str = r#"Normal(
        name: "Perforating Artist",
        mana_cost: [Generic(0)],
        types: [Creature],
        subtypes: [Devil],
        abilities: [
            Keyword(Deathtouch),
            Triggered(
                event: StepBegins(at: Ending(End), whose: Your),
                condition: Happened(
                    event: AttackDeclared(by: ControlledBy(Ref(You)), against: Any),
                    within: ThisTurn),
                effect: Each(
                    binder: Existing(SelectAll(OpponentOf(Ref(You)))),
                    effect: ChangeLife(It, Down(3)))),
        ],
        power: 3,
        toughness: 2)"#;

    let artist = fixture(ARTIST);
    let bear = fixture(BEAR);
    let mut p0 = deck(&artist, 2);
    p0.extend(deck(&bear, 22));
    let mut state = game(vec![p0, deck(&bear, 24)], 141);
    let devil = onto_battlefield(&mut state, PlayerId(0), "Perforating Artist");
    state.objects.obj_mut(devil).summoning_sick = false;
    onto_battlefield(&mut state, PlayerId(1), "Witness Bear");

    let mut declared = false;
    {
        let mut answer = |s: &GameState, p: &DecisionPointKind| match p {
            DecisionPointKind::DeclareAttackers(_) if !declared => {
                declared = true;
                Some(attack_player(s, &[devil]))
            }
            _ => None,
        };
        to_phase(
            &mut state,
            PlayerId(0),
            PhaseStep::Ending(EndingStep::End),
            &mut answer,
        );
    }

    assert!(
        declared,
        "Raid's condition is satisfied — the Artist attacked"
    );
    assert_eq!(
        state.players[1].life,
        17 + 3,
        "the opponent sacrificed a nonland permanent rather than lose 3 life"
    );
    assert!(
        battlefield_named(&state, PlayerId(1), "Witness Bear").is_empty(),
        "…and the permanent they chose was their own"
    );
}
