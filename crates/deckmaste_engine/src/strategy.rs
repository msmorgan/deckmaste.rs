//! Strategy evaluation context: the `ExecutionFrame` a data-driven strategy's sensing
//! (`Condition`/`Count`/`Reference`) is evaluated against. The controller
//! parameter binds to the deciding seat; the source parameter binds to the
//! candidate option being scored. The engine's existing
//! `eval_count`/`condition_holds`/`eval_reference` do the rest — there is no
//! second evaluator. `strategy-evaluator-core` builds the `StrategyEvaluator`
//! on top of this.

use deckmaste_core::Count;
use deckmaste_core::TargetSpec;
use deckmaste_core::Uint;

use crate::Action;
use crate::Decision;
use crate::DecisionPointKind;
use crate::object::ObjectId;
use crate::player::PlayerId;
use crate::stack::ExecutionFrame;
use crate::state::GameState;
use crate::strategy_def::BlockPolicy;
use crate::strategy_def::Extremum;
use crate::strategy_def::Preference;
use crate::strategy_def::Selector;
use crate::strategy_def::Strategy as StrategyDef;

/// The evaluation frame for scoring a `candidate` option from `seat`'s
/// perspective: the controller parameter resolves to `seat`, and the source
/// parameter resolves to `candidate` — or, when there is no candidate
/// (player-only sensing), to `seat`'s own player proxy. Sensing only: no
/// targets, trigger bindings, choice, or X. The engine's
/// `eval_count`/`condition_holds`/ `eval_reference` evaluate a strategy's
/// `Count`/`Condition`/`Reference` against this exactly as they do during
/// effect resolution.
pub(crate) fn eval_frame(
    state: &GameState,
    seat: PlayerId,
    candidate: Option<ObjectId>,
) -> ExecutionFrame {
    state.frame(candidate.unwrap_or_else(|| state.player(seat).object), seat)
}

/// A data-driven seat: answers the engine's decisions by walking a
/// [`Strategy`]'s ordered rules and ranking legal options with its selectors.
/// Implements the engine `Strategy` trait, so it drops into `play()`, the
/// harness, and the TUI driver wherever a hardcoded greedy seat went.
///
/// This is the core — rule-walk + selector engine + a total fallback — wired
/// for the priority window. The remaining per-decision handlers (targeting,
/// combat, discards, …) ride on top in `strategy-decision-handlers`; until then
/// those kinds take the total fallback's legal default.
///
/// [`Strategy`]: deckmaste_plugin::Strategy
pub struct StrategyEvaluator {
    strategy: StrategyDef,
    seat: PlayerId,
}

impl StrategyEvaluator {
    /// A seat driven by `strategy`.
    #[must_use]
    pub fn new(strategy: StrategyDef, seat: PlayerId) -> Self {
        Self { strategy, seat }
    }

    /// Build a seat from a semantic RON strategy (raw — no macro vocabulary;
    /// the macro-aware loader rides the plugin crate's `MacroSet`).
    ///
    /// # Errors
    ///
    /// Returns the parse error as a string when `src` is not a valid RON
    /// `Strategy`.
    pub fn from_ron(src: &str, seat: PlayerId) -> Result<Self, String> {
        let strategy = deckmaste_core::ron::options()
            .from_str::<StrategyDef>(src)
            .map_err(|e| e.to_string())?;
        Ok(Self::new(strategy, seat))
    }

    /// Rule-walk at a priority window: the first rule whose `when` holds AND
    /// whose `prefer` resolves to a legal action wins; falls through to `Pass`.
    fn decide_priority(&self, state: &GameState, legal: &[Action]) -> Action {
        let frame = eval_frame(state, self.seat, None);
        for rule in &self.strategy.rules {
            if !state.condition_holds(&rule.when, &frame) {
                continue;
            }
            if let Some(action) = self.priority_action(state, &rule.prefer, legal) {
                return action;
            }
        }
        Action::Pass
    }

    /// Map a `Preference` to a legal priority `Action`, or `None` when it does
    /// not apply to a priority window (`Attack`/`Block`/`Discard`) or has no
    /// legal instance — so the rule-walk falls through to the next rule.
    fn priority_action(
        &self,
        state: &GameState,
        prefer: &Preference,
        legal: &[Action],
    ) -> Option<Action> {
        match prefer {
            Preference::Pass => Some(Action::Pass),
            Preference::Concede => Some(Action::Concede),
            Preference::Play { what } => {
                let cands: Vec<ObjectId> = legal
                    .iter()
                    .filter_map(|a| match a {
                        Action::PlayLand { object } => Some(*object),
                        _ => None,
                    })
                    .collect();
                self.select(state, what, &cands)
                    .map(|object| Action::PlayLand { object })
            }
            Preference::Cast { what, .. } => {
                let cands: Vec<ObjectId> = legal
                    .iter()
                    .filter_map(|a| match a {
                        // Core exposes proposals without proving payment. The
                        // deterministic runner uses the existing read-only
                        // autotap plan only as advisory loop prevention; once
                        // selected, the cast still pays through PaymentCommand.
                        Action::CastSpell { object }
                            if state.autotap_for_cast(self.seat, *object).is_some() =>
                        {
                            Some(*object)
                        }
                        _ => None,
                    })
                    .collect();
                self.select(state, what, &cands)
                    .map(|object| Action::CastSpell { object })
            }
            Preference::Activate { what, .. } => {
                let cands: Vec<ObjectId> = legal
                    .iter()
                    .filter_map(|a| match a {
                        Action::ActivateAbility { object, ability }
                            if activation_direct_cost_is_usable(state, *object, *ability) =>
                        {
                            Some(*object)
                        }
                        _ => None,
                    })
                    .collect();
                let picked = self.select(state, what, &cands)?;
                legal.iter().find_map(|a| match a {
                    Action::ActivateAbility { object, ability }
                        if *object == picked
                            && activation_direct_cost_is_usable(state, *object, *ability) =>
                    {
                        Some(Action::ActivateAbility {
                            object: *object,
                            ability: *ability,
                        })
                    }
                    _ => None,
                })
            }
            // Not priority-window plays — handled at their own decision kinds.
            Preference::Attack { .. } | Preference::Block(_) | Preference::Discard { .. } => None,
        }
    }

    /// The selector engine: from `candidates`, keep those matching `among`,
    /// then take the `pick` extremum by the per-candidate `by` count
    /// (evaluated with the candidate bound as `This`). `None` if nothing
    /// matches.
    fn select(
        &self,
        state: &GameState,
        selector: &Selector,
        candidates: &[ObjectId],
    ) -> Option<ObjectId> {
        let matching = candidates
            .iter()
            .copied()
            .filter(|&o| self.matches_among(state, selector, o));
        match selector.pick {
            Extremum::First => matching.into_iter().next(),
            Extremum::Min => matching.min_by_key(|&o| self.score(state, &selector.by, o)),
            Extremum::Max => matching.max_by_key(|&o| self.score(state, &selector.by, o)),
        }
    }

    /// Does `candidate` pass the selector's optional `among` filter? (`None` =
    /// the whole set.) The filter reads from the acting SEAT's perspective: the
    /// frame is anchored to the seat (not the candidate), so `Ref(You)` is the
    /// seat and `Not(Ref(You))` names its opponent — a strategy's "target the
    /// opponent" / "not my own permanents". The candidate is the object under
    /// test (matched by `Kind`/`Type`/`Named`/…, and bound as `Subject` for a
    /// `Where`). Ranking (`by`) keeps its own candidate-anchored frame in
    /// [`Self::score`], so `StatOf(This, …)` still ranks by the candidate.
    fn matches_among(&self, state: &GameState, selector: &Selector, candidate: ObjectId) -> bool {
        match &selector.among {
            None => true,
            Some(filter) => {
                let frame = eval_frame(state, self.seat, None);
                state.filter_matches_live(filter, candidate, state.frame_watcher(&frame))
            }
        }
    }

    /// The per-candidate ranking count, evaluated with `candidate` bound as
    /// `This`.
    fn score(&self, state: &GameState, by: &Count, candidate: ObjectId) -> Uint {
        let frame = eval_frame(state, self.seat, Some(candidate));
        state.eval_count(by, &frame)
    }

    /// The shared rule-walk: top-to-bottom, the first rule whose `when` holds
    /// (over a candidate-less sensing frame) for which `f` of its `prefer`
    /// yields a value. Each per-decision handler passes an `f` that extracts
    /// the part of the preference it needs.
    fn first_applicable<'a, T>(
        &'a self,
        state: &GameState,
        mut f: impl FnMut(&'a Preference) -> Option<T>,
    ) -> Option<T> {
        let frame = eval_frame(state, self.seat, None);
        self.strategy
            .rules
            .iter()
            .filter(|r| state.condition_holds(&r.when, &frame))
            .find_map(|r| f(&r.prefer))
    }

    /// Choose a target SET per spec slot ([CR#601.2c,115.7e]): pick each slot's
    /// count (its minimum, at least one where the bounds allow) preferring the
    /// applicable `Cast`/`Activate` preference's `target` selector, then the
    /// slot's legal order — greedily skipping candidates that break within-slot
    /// distinctness or a `Distinct` constraint against an earlier slot. Slots
    /// fill in order, so a `Distinct` slot sees its siblings' picks.
    fn decide_targets(
        &self,
        state: &GameState,
        specs: &[TargetSpec],
        legal: &[Vec<ObjectId>],
    ) -> Vec<Vec<ObjectId>> {
        let selector = self.first_applicable(state, |p| match p {
            Preference::Cast {
                target: Some(s), ..
            }
            | Preference::Activate {
                target: Some(s), ..
            } => Some(s),
            _ => None,
        });
        let mut chosen: Vec<Vec<ObjectId>> = vec![Vec::new(); specs.len()];
        for (i, spec) in specs.iter().enumerate() {
            let (min, max) = crate::resolve::slot_count_bounds(spec);
            // Pick the minimum, but at least one where the maximum permits (a
            // legal, minimal-yet-non-empty default for "up to N" slots).
            let want = max.map_or(min.max(1), |hi| min.max(1).min(hi));
            let want = usize::try_from(want).expect("target count fits usize");
            let siblings = crate::resolve::distinct_siblings(spec);
            // Selector's pick first (when it applies), then the slot's legal
            // order; dedup keeps the selector's pick from repeating.
            let preferred = selector.and_then(|s| self.select(state, s, &legal[i]));
            let ordered = preferred
                .into_iter()
                .chain(legal[i].iter().copied())
                .collect::<Vec<_>>();
            for cand in ordered {
                if chosen[i].len() >= want {
                    break;
                }
                let clashes = chosen[i].contains(&cand)
                    || siblings
                        .iter()
                        .any(|&s| chosen.get(s).is_some_and(|set| set.contains(&cand)));
                if !clashes {
                    chosen[i].push(cand);
                }
            }
        }
        chosen
    }

    /// Declare attackers: the legal attackers matching the applicable `Attack`
    /// preference's `among` filter (the whole legal set when `among` is
    /// `None`). No `Attack` rule → declare none. `pick`/`by` are unused
    /// here — attacking is a set decision, so only `among` narrows it.
    fn decide_attackers(&self, state: &GameState, legal: &[ObjectId]) -> Vec<(ObjectId, ObjectId)> {
        self.first_applicable(state, |p| match p {
            Preference::Attack { what } => Some(what),
            _ => None,
        })
        .map(|what| {
            legal
                .iter()
                .copied()
                .filter(|&o| self.matches_among(state, what, o))
                // [CR#508.1b]: this baseline AI always attacks the defending
                // player — each attacker is paired with the sole opponent's
                // player proxy. Planeswalker-target selection is a follow-up.
                .map(|o| {
                    let controller = state.objects.obj(o).controller;
                    (o, state.player(state.next_live_after(controller)).object)
                })
                .collect()
        })
        .unwrap_or_default()
    }

    /// Declare blocks per the applicable `Block` preference's policy. No
    /// `Block` rule or `NoBlocks` → block nothing. `BlockAll` pairs each
    /// legal blocker with a declared attacker (round-robin); `ChumpBiggest`
    /// sends every legal blocker at the highest-power attacker. The engine
    /// re-validates each pair.
    fn decide_blocks(&self, state: &GameState, legal: &[ObjectId]) -> Vec<(ObjectId, ObjectId)> {
        let policy = self.first_applicable(state, |p| match p {
            Preference::Block(policy) => Some(*policy),
            _ => None,
        });
        let attackers = state.combat.attackers();
        let policy = match policy {
            None | Some(BlockPolicy::NoBlocks) => return vec![],
            Some(_) if attackers.is_empty() => return vec![],
            Some(policy) => policy,
        };
        // Repair the naive proposal against block legality so the strategy
        // never submits an illegal decision ([CR#509.1b,702.111b]) —
        // the play error a blind flyer-vs-ground round-robin would
        // otherwise `expect`-panic in the sim. A blocker is only paired
        // with an attacker its point-wise Cant(Block) rows permit, and
        // an attacker whose resulting blocker set trips an arrangement
        // bound (menace — too few blockers) is left unblocked.
        // `validate_blocks` enforces these same rules on submission.
        let view = state.layers();
        let rows = crate::legal::cant_block_rows(state, &view);
        let permits = |b: ObjectId, a: ObjectId| {
            crate::legal::block_forbidden_by(state, &rows, b, a).is_none()
        };
        let mut pairs: Vec<(ObjectId, ObjectId)> = match policy {
            // Handled by the early return; kept for exhaustiveness.
            BlockPolicy::NoBlocks => Vec::new(),
            // Spread blockers across attackers (round-robin), skipping to the
            // first attacker each blocker may legally block; a blocker no
            // attacker permits is dropped.
            BlockPolicy::BlockAll => legal
                .iter()
                .enumerate()
                .filter_map(|(i, &b)| {
                    (0..attackers.len())
                        .map(|k| attackers[(i + k) % attackers.len()])
                        .find(|&a| permits(b, a))
                        .map(|a| (b, a))
                })
                .collect(),
            // Gang the biggest attacker each blocker may legally block.
            BlockPolicy::ChumpBiggest => legal
                .iter()
                .filter_map(|&b| {
                    attackers
                        .iter()
                        .copied()
                        .filter(|&a| permits(b, a))
                        .max_by_key(|&a| view.power(a).unwrap_or(0))
                        .map(|a| (b, a))
                })
                .collect(),
        };
        // Drop any attacker whose blocker set trips an arrangement bound
        // (menace): the AI can't legally satisfy it, so it declines to block
        // it.
        let mut by_attacker: std::collections::HashMap<ObjectId, Vec<ObjectId>> =
            std::collections::HashMap::new();
        for &(b, a) in &pairs {
            by_attacker.entry(a).or_default().push(b);
        }
        let mut forbidden: std::collections::HashSet<ObjectId> = std::collections::HashSet::new();
        for (&a, blockers) in &by_attacker {
            if crate::legal::arrangement_forbidden_by(state, &rows, a, blockers).is_some() {
                forbidden.insert(a);
            }
        }
        pairs.retain(|&(_, a)| !forbidden.contains(&a));
        pairs
    }

    /// Choose `count` cards to discard: the applicable `Discard` preference's
    /// selector ranks the hand (`among`-matched first, falling back to the
    /// whole hand if too few match), and the `pick` end's first `count` are
    /// shed. No `Discard` rule → the first `count` in hand order.
    fn decide_discard(&self, state: &GameState, player: PlayerId, count: Uint) -> Vec<ObjectId> {
        let hand = &state.zones.hands[player.index()];
        let want = (count as usize).min(hand.len());
        let Some(s) = self.first_applicable(state, |p| match p {
            Preference::Discard { what } => Some(what),
            _ => None,
        }) else {
            return hand.iter().copied().take(want).collect();
        };
        let matched: Vec<ObjectId> = hand
            .iter()
            .copied()
            .filter(|&o| self.matches_among(state, s, o))
            .collect();
        let mut cands = if matched.len() >= want { matched } else { hand.clone() };
        match s.pick {
            Extremum::First => {}
            Extremum::Min => cands.sort_by_key(|&o| self.score(state, &s.by, o)),
            Extremum::Max => cands.sort_by_key(|&o| std::cmp::Reverse(self.score(state, &s.by, o))),
        }
        cands.truncate(want);
        cands
    }

    /// A total, always-legal default for the decision kinds the core does not
    /// yet handle smartly (filled in by `strategy-decision-handlers`). Mirrors
    /// the harness's `mechanical` defaults, but never panics on the kinds that
    /// arise in v1 decks, nor on the simple shells.
    fn fallback(&self, state: &GameState, pending: &DecisionPointKind) -> Decision {
        match pending {
            // Stays total even though `decide` routes Priority itself.
            DecisionPointKind::Priority(crate::decide::pending::Priority { legal, .. }) => {
                Decision::Act(self.decide_priority(state, legal))
            }
            DecisionPointKind::DiscardToHandSize(crate::decide::pending::DiscardToHandSize {
                player,
                count,
            })
            | DecisionPointKind::DiscardCards(crate::decide::pending::DiscardCards {
                player,
                count,
            }) => {
                let hand = &state.zones.hands[player.index()];
                let n = (*count as usize).min(hand.len());
                Decision::Discard(hand.iter().copied().take(n).collect())
            }
            DecisionPointKind::ChooseManaColor(crate::decide::pending::ChooseManaColor {
                options,
                ..
            }) => Decision::ManaColor(*options.first().expect("a mana choice offers options")),
            // Greedy default: the first offered run (printed order).
            DecisionPointKind::ChooseManaMode(crate::decide::pending::ChooseManaMode {
                ..
            }) => Decision::ManaMode(0),
            DecisionPointKind::PayMana(crate::decide::pending::PayMana { .. }) => {
                Decision::Pay(state.auto_pay_pending())
            }
            DecisionPointKind::Payment(_) => {
                state.auto_payment_pending().expect("Payment is pending")
            }
            DecisionPointKind::ChooseManaReversals(
                crate::decide::pending::ChooseManaReversals { legal, .. },
            ) => Decision::ManaReversals(
                legal
                    .iter()
                    .max_by_key(|set| set.len())
                    .cloned()
                    .expect("a mana-reversal prompt offers at least one legal set"),
            ),
            DecisionPointKind::OrderTriggers(crate::decide::pending::OrderTriggers {
                triggers,
                ..
            }) => Decision::Order((0..triggers.len()).collect()),
            DecisionPointKind::ChooseTargets(crate::decide::pending::ChooseTargets {
                spec,
                legal,
                ..
            }) => Decision::Targets(crate::sim::pick_target_set(spec, legal)),
            // No-op defaults: declaring no attackers / no blocks is always legal.
            DecisionPointKind::DeclareAttackers(crate::decide::pending::DeclareAttackers {
                ..
            }) => Decision::Attackers(vec![]),
            DecisionPointKind::DeclareBlockers(crate::decide::pending::DeclareBlockers {
                ..
            }) => Decision::Blocks(vec![]),
            DecisionPointKind::AssignCombatDamage(crate::decide::pending::AssignCombatDamage {
                source,
                recipients,
                ..
            }) => {
                let power = state
                    .combat_damage
                    .as_ref()
                    .and_then(|cd| cd.queue.iter().find(|a| a.source == *source))
                    .map_or(0, |a| a.power);
                let first = *recipients
                    .first()
                    .expect("a multi-blocked source has recipients");
                Decision::Assignment(vec![(first, power)])
            }
            DecisionPointKind::ChooseObjects(crate::decide::pending::ChooseObjects {
                candidates,
                min,
                ..
            }) => Decision::Chosen(candidates.iter().copied().take(*min as usize).collect()),
            // [CR#601.2b,608.2c]: the minimal always-legal default (0) — the
            // X-announce and a resolution note number ("choose a number") alike,
            // both answered through the `Decision::XValue` shape.
            DecisionPointKind::ChooseXValue(crate::decide::pending::ChooseXValue { .. })
            | DecisionPointKind::ChooseNoteNumber(crate::decide::pending::ChooseNoteNumber {
                ..
            }) => Decision::XValue(0),
            DecisionPointKind::ChooseNoteCardName(crate::decide::pending::ChooseNoteCardName {
                player,
                ..
            }) => {
                let name = state.zones.hands[player.index()].first().map_or_else(
                    || "Mountain".to_owned(),
                    |&id| {
                        crate::derive::face(state.def(id))
                            .characteristics
                            .name
                            .to_string()
                    },
                );
                Decision::CardName(name)
            }
            // Simple shells: a legal minimal default.
            DecisionPointKind::ChooseModes(choice) => Decision::Modes(
                state
                    .announcing
                    .as_ref()
                    .and_then(|_| state.first_legal_announced_mode_selection(choice))
                    .unwrap_or_else(|| (0..choice.min).collect()),
            ),
            DecisionPointKind::Division(crate::decide::pending::Division {
                total,
                targets,
                ..
            }) => {
                let first = *targets.first().expect("a division has targets");
                Decision::Divide(vec![(first, *total)])
            }
            DecisionPointKind::Vote(crate::decide::pending::Vote { .. }) => Decision::VoteFor(0),
            DecisionPointKind::YesNo(crate::decide::pending::YesNo { .. }) => {
                Decision::Answer(false)
            }
            // A called flip is strategically null (a fair coin either way) —
            // always calling heads is a reasonable, always-legal default.
            DecisionPointKind::CallFlip(crate::decide::pending::CallFlip { .. }) => {
                Decision::Answer(true)
            }
            // [CR#401.4]: any permutation of the pile is legal; keep the
            // offered order (a total, always-legal default).
            DecisionPointKind::ArrangePile(crate::decide::pending::ArrangePile {
                objects, ..
            }) => Decision::Arranged(objects.clone()),
            DecisionPointKind::ChooseReplacement(crate::decide::pending::ChooseReplacement {
                applicable,
                ..
            }) => Decision::ReplacementChoice(
                *applicable
                    .first()
                    .expect("a replacement choice offers options"),
            ),
            // [CR#707.10c]: re-target a committed entry by keeping every
            // current target (see `crate::sim::keep_current_targets`).
            DecisionPointKind::Retarget(crate::decide::pending::Retarget {
                entry,
                spec,
                legal,
                ..
            }) => Decision::Targets(crate::sim::keep_current_targets(state, *entry, spec, legal)),
            // Deep engine choices with no trivial legal default; none arise in
            // v1 decks. Later tickets handle these explicitly.
            other @ (DecisionPointKind::ChooseCostOptions(
                crate::decide::pending::ChooseCostOptions { .. },
            )
            | DecisionPointKind::OrderReplacements(
                crate::decide::pending::OrderReplacements { .. },
            )
            | DecisionPointKind::PreGame(crate::decide::pending::PreGame { .. })
            | DecisionPointKind::LegendRule(crate::decide::pending::LegendRule {
                ..
            })) => {
                todo!(
                    "strategy fallback for {other:?} (no v1 deck surfaces it); \
                     owner: engine-shell-decision-strategies"
                )
            }
        }
    }
}

/// Runner-policy guard for activation proposals whose direct tap/untap symbol
/// is visibly unavailable. Core deliberately enumerates those proposals and
/// discovers failure through payment; a deterministic strategy must avoid
/// proposing the same impossible action forever. All other payment choices
/// remain delegated to the explicit payment decision protocol.
fn activation_direct_cost_is_usable(state: &GameState, object: ObjectId, ability: usize) -> bool {
    crate::payment::automatic_activation_cost_usable(state, object, ability)
}

impl crate::sim::Strategy for StrategyEvaluator {
    fn decide(&self, state: &GameState, pending: &DecisionPointKind) -> Decision {
        match pending {
            DecisionPointKind::Priority(crate::decide::pending::Priority { legal, .. }) => {
                Decision::Act(self.decide_priority(state, legal))
            }
            DecisionPointKind::ChooseTargets(crate::decide::pending::ChooseTargets {
                spec,
                legal,
                ..
            }) => Decision::Targets(self.decide_targets(state, spec, legal)),
            DecisionPointKind::DeclareAttackers(crate::decide::pending::DeclareAttackers {
                legal,
                ..
            }) => Decision::Attackers(self.decide_attackers(state, legal)),
            DecisionPointKind::DeclareBlockers(crate::decide::pending::DeclareBlockers {
                legal,
                ..
            }) => Decision::Blocks(self.decide_blocks(state, legal)),
            DecisionPointKind::DiscardToHandSize(crate::decide::pending::DiscardToHandSize {
                player,
                count,
            })
            | DecisionPointKind::DiscardCards(crate::decide::pending::DiscardCards {
                player,
                count,
            }) => Decision::Discard(self.decide_discard(state, *player, *count)),
            other => self.fallback(state, other),
        }
    }
}

#[cfg(test)]
mod tests {
    use std::path::Path;
    use std::sync::Arc;

    use deckmaste_card::Card;
    use deckmaste_core::Cmp;
    use deckmaste_core::Condition;
    use deckmaste_core::Count;
    use deckmaste_core::Predicate;
    use deckmaste_core::Quantity;
    use deckmaste_core::Reference;
    use deckmaste_core::Stat;
    use deckmaste_core::TargetSpec;
    use deckmaste_core::Type;
    use deckmaste_core::Zone;
    use deckmaste_plugin::plugin::Plugin;

    use super::StrategyEvaluator;
    use super::eval_frame;
    use crate::Action;
    use crate::Decision;
    use crate::DecisionPointKind;
    use crate::object::ObjectId;
    use crate::object::ObjectSource;
    use crate::player::PlayerId;
    use crate::sim::Strategy as _;
    use crate::state::GameConfig;
    use crate::state::GameState;
    use crate::state::PlayerConfig;
    use crate::state::StartingPlayer;
    use crate::strategy_def::BlockPolicy;
    use crate::strategy_def::Extremum;
    use crate::strategy_def::Preference;
    use crate::strategy_def::Rule;
    use crate::strategy_def::Selector;
    use crate::strategy_def::Strategy as StrategyDef;

    fn empty_two_player() -> GameState {
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

    fn put_creature(state: &mut GameState, card: &Arc<Card>, owner: PlayerId) -> ObjectId {
        let cid = state.cards.push(Arc::clone(card), owner);
        let id = state
            .objects
            .mint(ObjectSource::Card(cid), owner, Some(Zone::Battlefield));
        state.zones.battlefield.push(id);
        id
    }

    fn put_in_hand(state: &mut GameState, card: &Arc<Card>, owner: PlayerId) -> ObjectId {
        let cid = state.cards.push(Arc::clone(card), owner);
        let id = state
            .objects
            .mint(ObjectSource::Card(cid), owner, Some(Zone::Hand));
        state.zones.hands[owner.index()].push(id);
        id
    }

    fn always() -> Condition {
        Condition::And(vec![].into())
    }

    /// A one-rule strategy: `Always → prefer`.
    fn always_prefer(prefer: Preference) -> StrategyDef {
        StrategyDef {
            name: "t".into(),
            rules: vec![Rule {
                when: always(),
                prefer,
            }],
        }
    }

    fn canon() -> Plugin {
        Plugin::load_with_sibling_prelude(
            Path::new(env!("CARGO_MANIFEST_DIR")).join("../../plugins/canon"),
        )
        .unwrap()
    }

    /// The R1 spike: a *synthesized* frame (no resolving effect) drives the
    /// engine's existing evaluators. `You` → the seat, `This` → the candidate,
    /// and `Count`/`Condition` read live state. This is the whole premise of
    /// data-driven strategies — sensing reuses the card evaluators verbatim.
    #[test]
    fn synthesized_frame_drives_engine_evaluators() {
        let bears = Arc::new(canon().card("Grizzly Bears").unwrap().core);
        let mut state = GameState::new(GameConfig {
            players: vec![
                PlayerConfig {
                    deck: vec![Arc::clone(&bears); 10],
                },
                PlayerConfig { deck: vec![] },
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

        // A 2/2 Grizzly Bears on P0's battlefield — the candidate being scored.
        let bear_card = state.cards.push(Arc::clone(&bears), PlayerId(0));
        let bear = state.objects.mint(
            ObjectSource::Card(bear_card),
            PlayerId(0),
            Some(Zone::Battlefield),
        );
        state.zones.battlefield.push(bear);

        let frame = eval_frame(&state, PlayerId(0), Some(bear));

        // `You` → the seat's player proxy; `This` → the candidate object.
        assert_eq!(
            state.eval_reference(&Reference::Reg(deckmaste_core::RefId(1)), &frame),
            state.player(PlayerId(0)).object,
        );
        assert_eq!(
            state.eval_reference(&Reference::Reg(deckmaste_core::RefId(0)), &frame),
            bear
        );

        // A `Count` over the candidate: Grizzly Bears' power is 2.
        assert_eq!(
            state.eval_count(
                &Count::StatOf(Reference::Reg(deckmaste_core::RefId(0)), Stat::Power),
                &frame
            ),
            2,
        );

        // A `Condition` comparing the candidate's power against a literal: 2 >=
        // 2.
        let cond = Condition::Compare(
            Count::StatOf(Reference::Reg(deckmaste_core::RefId(0)), Stat::Power),
            Cmp::AtLeast,
            Count::Literal(2),
        );
        assert!(state.condition_holds(&cond, &frame));
    }

    /// With no candidate, `This` falls back to the seat's own player proxy —
    /// the shape for player-only sensing (mulligan keep/ship, life totals).
    #[test]
    fn frame_without_candidate_binds_this_to_seat_proxy() {
        let state = GameState::new(GameConfig {
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
        });
        let frame = eval_frame(&state, PlayerId(1), None);
        assert_eq!(
            state.eval_reference(&Reference::Reg(deckmaste_core::RefId(1)), &frame),
            state.player(PlayerId(1)).object,
        );
        assert_eq!(
            state.eval_reference(&Reference::Reg(deckmaste_core::RefId(0)), &frame),
            state.player(PlayerId(1)).object,
        );
    }

    /// An `among` predicate reads `This` from the acting seat's frame, not
    /// from each candidate being tested. With the opponent listed first,
    /// `Ref(This)` must still select the seat's own player proxy.
    #[test]
    fn selector_among_ref_this_is_anchored_to_the_seat_proxy() {
        let state = empty_two_player();
        let seat = PlayerId(1);
        let opponent_proxy = state.player(PlayerId(0)).object;
        let seat_proxy = state.player(seat).object;
        let selector = Selector {
            pick: Extremum::First,
            by: Count::Literal(1),
            among: Some(Predicate::Ref(Reference::Reg(deckmaste_core::RefId(0)))),
        };
        let eval = StrategyEvaluator::new(always_prefer(Preference::Pass), seat);

        assert_eq!(
            eval.select(&state, &selector, &[opponent_proxy, seat_proxy]),
            Some(seat_proxy),
        );
    }

    /// `Count::ManaAvailable(You)` reads the seat's floated pool total off a
    /// strategy sensing frame — the ramp-gate reader. Empty pool → 0; after
    /// floating three mana → 3; a non-player reference fizzles to 0.
    #[test]
    fn mana_available_reads_the_seats_floated_pool() {
        use deckmaste_core::Color;
        use deckmaste_core::ColorOrColorless;

        let mut state = empty_two_player();
        let frame = eval_frame(&state, PlayerId(0), None);
        assert_eq!(
            state.eval_count(
                &Count::ManaAvailable(Reference::Reg(deckmaste_core::RefId(1))),
                &frame
            ),
            0,
            "an empty pool reads 0",
        );

        state.player_mut(PlayerId(0)).mana_pool.add(
            ColorOrColorless::Color(Color::Green),
            2,
            crate::player::ManaProvenance::default(),
        );
        state.player_mut(PlayerId(0)).mana_pool.add(
            ColorOrColorless::Colorless,
            1,
            crate::player::ManaProvenance::default(),
        );
        let frame = eval_frame(&state, PlayerId(0), None);
        assert_eq!(
            state.eval_count(
                &Count::ManaAvailable(Reference::Reg(deckmaste_core::RefId(1))),
                &frame
            ),
            3,
            "three floated units read as 3",
        );
    }

    /// Rule-walk: the first rule whose `when` holds wins. `Always → Pass`
    /// yields a pass at a priority window.
    #[test]
    fn priority_first_matching_rule_yields_pass() {
        let strat = StrategyDef {
            name: "pass".into(),
            rules: vec![Rule {
                when: always(),
                prefer: Preference::Pass,
            }],
        };
        let eval = StrategyEvaluator::new(strat, PlayerId(0));
        let state = empty_two_player();
        let pending = DecisionPointKind::Priority(crate::decide::pending::Priority {
            player: PlayerId(0),
            legal: vec![Action::Pass],
        });
        assert_eq!(eval.decide(&state, &pending), Decision::Act(Action::Pass));
    }

    /// Selector engine: `Cast(pick: Max, by: power)` over two legal cast
    /// candidates picks the bigger creature (Grizzly Bears 2/2 over Willow Elf
    /// 1/1) — argmax of a `Count` over the legal set.
    #[test]
    fn priority_cast_selector_picks_by_extremum() {
        let willow = Arc::new(canon().card("Willow Elf").unwrap().core);
        let bears = Arc::new(canon().card("Grizzly Bears").unwrap().core);
        let mut state = empty_two_player();
        state.turn.active_player = PlayerId(0);
        state.turn.current = deckmaste_core::PhaseStep::PrecombatMain;
        let willow_id = put_in_hand(&mut state, &willow, PlayerId(0));
        let bears_id = put_in_hand(&mut state, &bears, PlayerId(0));
        state.player_mut(PlayerId(0)).mana_pool.add(
            deckmaste_core::ColorOrColorless::Color(deckmaste_core::Color::Green),
            2,
            crate::player::ManaProvenance::default(),
        );

        let strat = StrategyDef {
            name: "cast-biggest".into(),
            rules: vec![Rule {
                when: always(),
                prefer: Preference::Cast {
                    what: Selector {
                        pick: Extremum::Max,
                        by: Count::StatOf(Reference::Reg(deckmaste_core::RefId(0)), Stat::Power),
                        among: None,
                    },
                    target: None,
                },
            }],
        };
        let eval = StrategyEvaluator::new(strat, PlayerId(0));
        let pending = DecisionPointKind::Priority(crate::decide::pending::Priority {
            player: PlayerId(0),
            legal: vec![
                Action::CastSpell { object: willow_id },
                Action::CastSpell { object: bears_id },
            ],
        });
        assert_eq!(
            eval.decide(&state, &pending),
            Decision::Act(Action::CastSpell { object: bears_id }),
        );
    }

    /// Totality: an unhandled decision kind falls back to a legal default
    /// rather than panicking. A defender with no legal blockers blocks nothing.
    #[test]
    fn fallback_is_total_for_unhandled_kinds() {
        let strat = StrategyDef {
            name: "noop".into(),
            rules: vec![],
        };
        let eval = StrategyEvaluator::new(strat, PlayerId(1));
        let state = empty_two_player();
        let pending = DecisionPointKind::DeclareBlockers(crate::decide::pending::DeclareBlockers {
            player: PlayerId(1),
            legal: vec![],
        });
        assert_eq!(eval.decide(&state, &pending), Decision::Blocks(vec![]));
    }

    #[test]
    fn modal_fallback_skips_an_unsatisfiable_first_mode() {
        use deckmaste_card::CardFace;
        use deckmaste_card::Characteristics;
        use deckmaste_core::Ability;
        use deckmaste_core::CharacteristicPredicate;
        use deckmaste_core::ChooseSpec;
        use deckmaste_core::Instruction;
        use deckmaste_core::Modal;
        use deckmaste_core::Mode;
        use deckmaste_core::SpellAbility;

        let impossible_target = TargetSpec::Target(
            Quantity::one(),
            Arc::new(deckmaste_core::Region::candidate(
                Predicate::Characteristic(CharacteristicPredicate::Named("Missing target".into())),
            )),
        );
        let card = Arc::new(Card::Normal(CardFace::from(Characteristics {
            name: "Modal strategy fixture".into(),
            mana_cost: "{0}".parse().unwrap(),
            types: vec![Type::Instant.def()],
            abilities: vec![Ability::spell(SpellAbility {
                ability_word: None,
                cost: deckmaste_core::Cost::default(),
                targets: [].into(),
                effect: Instruction::Modal(Modal {
                    choose: ChooseSpec {
                        count: Quantity::one(),
                        up_to: false,
                        repeats: false,
                        chooser: Reference::Reg(deckmaste_core::RefId(1)),
                        rider: None,
                    },
                    modes: vec![
                        Mode {
                            targets: vec![impossible_target].into(),
                            effect: Instruction::Sequentially(Arc::from([])).into(),
                            cost: deckmaste_core::Cost::default(),
                        },
                        Mode {
                            targets: [].into(),
                            effect: Instruction::Sequentially(Arc::from([])).into(),
                            cost: deckmaste_core::Cost::default(),
                        },
                    ]
                    .into(),
                })
                .into(),
            })],
            ..Characteristics::default()
        })));
        let mut state = empty_two_player();
        let spell = put_in_hand(&mut state, &card, PlayerId(0));
        state.begin_cast(spell);
        assert_eq!(state.announce_modes(), 2);
        let pending = state.pending.clone().expect("ChooseModes is pending");
        let eval = StrategyEvaluator::new(always_prefer(Preference::Pass), PlayerId(0));

        assert_eq!(eval.decide(&state, &pending), Decision::Modes(vec![1]));
    }

    /// `ChooseTargets`: the applicable `Cast` preference's `target` selector
    /// picks the biggest creature among a slot's legal candidates.
    #[test]
    fn choose_targets_applies_the_target_selector_per_slot() {
        let willow = Arc::new(canon().card("Willow Elf").unwrap().core);
        let bears = Arc::new(canon().card("Grizzly Bears").unwrap().core);
        let mut state = empty_two_player();
        let willow_id = put_creature(&mut state, &willow, PlayerId(1));
        let bears_id = put_creature(&mut state, &bears, PlayerId(1));

        let eval = StrategyEvaluator::new(
            always_prefer(Preference::Cast {
                what: Selector {
                    pick: Extremum::First,
                    by: Count::Literal(1),
                    among: None,
                },
                target: Some(Selector {
                    pick: Extremum::Max,
                    by: Count::StatOf(Reference::Reg(deckmaste_core::RefId(0)), Stat::Power),
                    among: None,
                }),
            }),
            PlayerId(0),
        );
        let pending = DecisionPointKind::ChooseTargets(crate::decide::pending::ChooseTargets {
            player: PlayerId(0),
            spec: one_creature_target(),
            legal: vec![vec![willow_id, bears_id]],
        });
        assert_eq!(
            eval.decide(&state, &pending),
            Decision::Targets(vec![vec![bears_id]]),
        );
    }

    /// `ChooseTargets` with no `target` rule falls back to the first legal
    /// candidate per slot (still total/legal).
    #[test]
    fn choose_targets_without_a_rule_takes_first_legal() {
        let eval = StrategyEvaluator::new(always_prefer(Preference::Pass), PlayerId(0));
        let mut state = empty_two_player();
        let bears = Arc::new(canon().card("Grizzly Bears").unwrap().core);
        let a = put_creature(&mut state, &bears, PlayerId(1));
        let b = put_creature(&mut state, &bears, PlayerId(1));
        let pending = DecisionPointKind::ChooseTargets(crate::decide::pending::ChooseTargets {
            player: PlayerId(0),
            spec: one_creature_target(),
            legal: vec![vec![a, b]],
        });
        assert_eq!(
            eval.decide(&state, &pending),
            Decision::Targets(vec![vec![a]])
        );
    }

    /// One quantity-one "target creature" spec — the shape the real announce
    /// flow always pairs with a one-slot legal set.
    fn one_creature_target() -> Vec<TargetSpec> {
        vec![TargetSpec::Target(
            Quantity::one(),
            Arc::new(deckmaste_core::Region::candidate(Predicate::r#type(
                Type::Creature,
            ))),
        )]
    }

    /// `DeclareAttackers`: an `Attack` preference declares the whole legal set
    /// (no `among`); no `Attack` rule declares none.
    #[test]
    fn declare_attackers_attacks_all_legal_then_none() {
        let bears = Arc::new(canon().card("Grizzly Bears").unwrap().core);
        let mut state = empty_two_player();
        let a = put_creature(&mut state, &bears, PlayerId(0));
        let b = put_creature(&mut state, &bears, PlayerId(0));
        let p1_proxy = state.player(PlayerId(1)).object;
        let pending =
            DecisionPointKind::DeclareAttackers(crate::decide::pending::DeclareAttackers {
                player: PlayerId(0),
                legal: vec![a, b],
                legal_targets: vec![p1_proxy],
            });

        let attacker = StrategyEvaluator::new(
            always_prefer(Preference::Attack {
                what: Selector {
                    pick: Extremum::First,
                    by: Count::Literal(1),
                    among: None,
                },
            }),
            PlayerId(0),
        );
        // The baseline AI attacks the sole opponent's player proxy.
        assert_eq!(
            attacker.decide(&state, &pending),
            Decision::Attackers(vec![(a, p1_proxy), (b, p1_proxy)]),
        );

        let passive = StrategyEvaluator::new(always_prefer(Preference::Pass), PlayerId(0));
        assert_eq!(
            passive.decide(&state, &pending),
            Decision::Attackers(vec![])
        );
    }

    /// `DeclareBlockers`: `Block(NoBlocks)` declares no blocks even with legal
    /// blockers available.
    #[test]
    fn declare_blockers_no_blocks_policy_blocks_nothing() {
        let bears = Arc::new(canon().card("Grizzly Bears").unwrap().core);
        let mut state = empty_two_player();
        let blocker = put_creature(&mut state, &bears, PlayerId(1));
        let eval = StrategyEvaluator::new(
            always_prefer(Preference::Block(BlockPolicy::NoBlocks)),
            PlayerId(1),
        );
        let pending = DecisionPointKind::DeclareBlockers(crate::decide::pending::DeclareBlockers {
            player: PlayerId(1),
            legal: vec![blocker],
        });
        assert_eq!(eval.decide(&state, &pending), Decision::Blocks(vec![]));
    }

    /// `DeclareBlockers` × `BlockAll`: the strategy must never PROPOSE an
    /// illegal pair. With a flier AND a ground attacker on the board, the
    /// ground blocker is point-wise forbidden from the flier ([CR#702.9b])
    /// but may block the ground attacker — `decide_blocks` repairs the
    /// naive round-robin so the blocker is sent at an attacker it can
    /// legally block, and the whole proposal passes `validate_blocks`,
    /// keeping the sim's "a strategy submits only legal decisions"
    /// expectation honest.
    #[test]
    fn block_all_repairs_illegal_flyer_pairing() {
        let strix = Arc::new(canon().card("Baleful Strix").unwrap().core);
        let bears = Arc::new(canon().card("Grizzly Bears").unwrap().core);
        let mut state = empty_two_player();
        let flyer = put_creature(&mut state, &strix, PlayerId(0));
        let ground = put_creature(&mut state, &bears, PlayerId(0));
        let blocker = put_creature(&mut state, &bears, PlayerId(1));
        let defender_proxy = state.player(PlayerId(1)).object;
        state.combat.declare_attacker(flyer, defender_proxy);
        state.combat.declare_attacker(ground, defender_proxy);

        // Precondition: the flier's point-wise Cant(Block) is live — a ground
        // blocker can't legally block it, but it may block the ground attacker.
        let err = state.validate_blocks(&[(blocker, flyer)]).unwrap_err();
        assert!(
            err.to_string().contains("forbids"),
            "unexpected error: {err}"
        );
        // Surfacing keeps the ground blocker: the ground attacker permits it.
        let legal = crate::legal::legal_blockers(&state, PlayerId(1));
        assert!(
            legal.contains(&blocker),
            "the ground blocker is surfaced — the ground attacker permits it: {legal:?}"
        );

        let eval = StrategyEvaluator::new(
            always_prefer(Preference::Block(BlockPolicy::BlockAll)),
            PlayerId(1),
        );
        let pending = DecisionPointKind::DeclareBlockers(crate::decide::pending::DeclareBlockers {
            player: PlayerId(1),
            legal,
        });
        let Decision::Blocks(pairs) = eval.decide(&state, &pending) else {
            panic!("a DeclareBlockers decision yields Blocks");
        };
        assert!(
            !pairs.is_empty(),
            "the ground blocker CAN block the ground attacker — BlockAll blocks it: {pairs:?}"
        );
        assert!(
            pairs.iter().all(|&(_, a)| a != flyer),
            "no ground blocker is sent at the flier ([CR#702.9b]): {pairs:?}"
        );
        assert!(
            state.validate_blocks(&pairs).is_ok(),
            "the repaired proposal is a legal block assignment: {pairs:?}"
        );
    }

    /// Discard: the `Discard` selector ranks the hand and sheds the `count`
    /// cheapest (Min by mana value) — Willow Elf (1) over Grizzly Bears (2).
    #[test]
    fn discard_sheds_cheapest_by_mana_value() {
        let willow = Arc::new(canon().card("Willow Elf").unwrap().core);
        let bears = Arc::new(canon().card("Grizzly Bears").unwrap().core);
        let mut state = empty_two_player();
        let bears_id = put_in_hand(&mut state, &bears, PlayerId(0));
        let willow_id = put_in_hand(&mut state, &willow, PlayerId(0));
        let _ = bears_id;

        let eval = StrategyEvaluator::new(
            always_prefer(Preference::Discard {
                what: Selector {
                    pick: Extremum::Min,
                    by: Count::StatOf(Reference::Reg(deckmaste_core::RefId(0)), Stat::ManaValue),
                    among: None,
                },
            }),
            PlayerId(0),
        );
        let pending = DecisionPointKind::DiscardCards(crate::decide::pending::DiscardCards {
            player: PlayerId(0),
            count: 1,
        });
        assert_eq!(
            eval.decide(&state, &pending),
            Decision::Discard(vec![willow_id]),
        );
    }
}
