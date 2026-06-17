//! Strategy evaluation context: the `Frame` a data-driven strategy's sensing
//! (`Condition`/`Count`/`Reference`) is evaluated against. `Reference::You`
//! binds to the deciding seat; `Reference::This`/`~` binds to the candidate
//! option being scored. The engine's existing
//! `eval_count`/`condition_holds`/`eval_reference` do the rest — there is no
//! second evaluator. `strategy-evaluator-core` builds the `StrategyEvaluator`
//! on top of this.

use deckmaste_core::Count;
use deckmaste_core::Uint;
use deckmaste_core::strategy::BlockPolicy;
use deckmaste_core::strategy::Extremum;
use deckmaste_core::strategy::Preference;
use deckmaste_core::strategy::Selector;
use deckmaste_core::strategy::Strategy as StrategyDef;

use crate::Action;
use crate::Decision;
use crate::PendingDecision;
use crate::object::ObjectId;
use crate::player::PlayerId;
use crate::stack::Frame;
use crate::state::GameState;

/// The evaluation frame for scoring a `candidate` option from `seat`'s
/// perspective: `Reference::You` resolves to `seat`, and `Reference::This`/`~`
/// resolves to `candidate` — or, when there is no candidate (player-only
/// sensing), to `seat`'s own player proxy. Sensing only: no targets, trigger
/// bindings, choice, or X. The engine's `eval_count`/`condition_holds`/
/// `eval_reference` evaluate a strategy's `Count`/`Condition`/`Reference`
/// against this exactly as they do during effect resolution.
pub(crate) fn eval_frame(state: &GameState, seat: PlayerId, candidate: Option<ObjectId>) -> Frame {
    Frame {
        source: candidate.unwrap_or_else(|| state.player(seat).object),
        controller: seat,
        targets: Vec::new(),
        bindings: None,
        chosen: None,
        x: None,
        subject: None,
    }
}

/// Look through a remembered `Preference` macro invocation to the variant it
/// expanded to (the choose-a-play vocabulary), so the handlers match on the
/// concrete preference regardless of whether it was authored literally or as a
/// macro.
fn resolved(prefer: &Preference) -> &Preference {
    match prefer {
        Preference::Expanded(e) => resolved(&e.value),
        other => other,
    }
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
/// [`Strategy`]: deckmaste_cards::Strategy
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

    /// Build a seat from an authored RON strategy (raw — no macro vocabulary;
    /// the macro-aware loader rides the cards crate's `MacroSet`).
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
                        Action::CastSpell { object } => Some(*object),
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
                        Action::ActivateAbility { object, .. } => Some(*object),
                        _ => None,
                    })
                    .collect();
                let picked = self.select(state, what, &cands)?;
                legal.iter().find_map(|a| match a {
                    Action::ActivateAbility { object, ability } if *object == picked => {
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
            Preference::Expanded(e) => self.priority_action(state, &e.value, legal),
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
    /// the whole set.) Evaluated with the candidate bound as `This`.
    fn matches_among(&self, state: &GameState, selector: &Selector, candidate: ObjectId) -> bool {
        match &selector.among {
            None => true,
            Some(filter) => {
                let frame = eval_frame(state, self.seat, Some(candidate));
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
    /// (over a candidate-less sensing frame) for which `f` of its
    /// (macro-resolved) `prefer` yields a value. Each per-decision handler
    /// passes an `f` that extracts the part of the preference it needs.
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
            .find_map(|r| f(resolved(&r.prefer)))
    }

    /// Choose one target per spec slot: apply the applicable `Cast`/`Activate`
    /// preference's `target` selector to that slot's legal candidates, falling
    /// back to the first legal candidate when no rule supplies a selector.
    fn decide_targets(&self, state: &GameState, legal: &[Vec<ObjectId>]) -> Vec<ObjectId> {
        let target = self.first_applicable(state, |p| match p {
            Preference::Cast {
                target: Some(s), ..
            }
            | Preference::Activate {
                target: Some(s), ..
            } => Some(s),
            _ => None,
        });
        legal
            .iter()
            .map(|slot| {
                target
                    .and_then(|s| self.select(state, s, slot))
                    .or_else(|| slot.first().copied())
                    .expect("a target spec offers a candidate")
            })
            .collect()
    }

    /// Declare attackers: the legal attackers matching the applicable `Attack`
    /// preference's `among` filter (the whole legal set when `among` is
    /// `None`). No `Attack` rule → declare none. `pick`/`by` are unused
    /// here — attacking is a set decision, so only `among` narrows it.
    fn decide_attackers(&self, state: &GameState, legal: &[ObjectId]) -> Vec<ObjectId> {
        self.first_applicable(state, |p| match p {
            Preference::Attack { what } => Some(what),
            _ => None,
        })
        .map(|what| {
            legal
                .iter()
                .copied()
                .filter(|&o| self.matches_among(state, what, o))
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
        match policy {
            None | Some(BlockPolicy::NoBlocks) => vec![],
            Some(_) if attackers.is_empty() => vec![],
            Some(BlockPolicy::BlockAll) => legal
                .iter()
                .enumerate()
                .map(|(i, &b)| (b, attackers[i % attackers.len()]))
                .collect(),
            Some(BlockPolicy::ChumpBiggest) => {
                let biggest = attackers
                    .iter()
                    .copied()
                    .max_by_key(|&a| state.layers().power(a).unwrap_or(0))
                    .expect("attackers non-empty");
                legal.iter().map(|&b| (b, biggest)).collect()
            }
        }
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
    fn fallback(&self, state: &GameState, pending: &PendingDecision) -> Decision {
        match pending {
            // Stays total even though `decide` routes Priority itself.
            PendingDecision::Priority { legal, .. } => {
                Decision::Act(self.decide_priority(state, legal))
            }
            PendingDecision::DiscardToHandSize { player, count }
            | PendingDecision::DiscardCards { player, count } => {
                let hand = &state.zones.hands[player.index()];
                let n = (*count as usize).min(hand.len());
                Decision::Discard(hand.iter().copied().take(n).collect())
            }
            PendingDecision::ChooseManaColor { options, .. } => {
                Decision::ManaColor(*options.first().expect("a mana choice offers options"))
            }
            PendingDecision::PayMana { .. } => Decision::Pay(state.auto_pay_pending()),
            PendingDecision::OrderTriggers { triggers, .. } => {
                Decision::Order((0..triggers.len()).collect())
            }
            PendingDecision::ChooseTargets { legal, .. } => Decision::Targets(
                legal
                    .iter()
                    .map(|set| *set.first().expect("a target spec offers a candidate"))
                    .collect(),
            ),
            // No-op defaults: declaring no attackers / no blocks is always legal.
            PendingDecision::DeclareAttackers { .. } => Decision::Attackers(vec![]),
            PendingDecision::DeclareBlockers { .. } => Decision::Blocks(vec![]),
            PendingDecision::AssignCombatDamage {
                source, recipients, ..
            } => {
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
            PendingDecision::ChooseObjects {
                candidates, min, ..
            } => Decision::Chosen(candidates.iter().copied().take(*min as usize).collect()),
            PendingDecision::ChooseXValue { .. } => Decision::XValue(0),
            // Simple shells: a legal minimal default.
            PendingDecision::ChooseModes { min, .. } => Decision::Modes((0..*min).collect()),
            PendingDecision::Division { total, targets, .. } => {
                let first = *targets.first().expect("a division has targets");
                Decision::Divide(vec![(first, *total)])
            }
            PendingDecision::Vote { .. } => Decision::VoteFor(0),
            PendingDecision::YesNo { .. } => Decision::Answer(false),
            PendingDecision::ChooseReplacement { applicable, .. } => Decision::ReplacementChoice(
                *applicable
                    .first()
                    .expect("a replacement choice offers options"),
            ),
            // Deep engine choices with no trivial legal default; none arise in
            // v1 decks. Later tickets handle these explicitly.
            other @ (PendingDecision::ChooseCostOptions { .. }
            | PendingDecision::OrderReplacements { .. }
            | PendingDecision::PreGame { .. }
            | PendingDecision::LegendRule { .. }) => {
                todo!("strategy fallback for {other:?} (no v1 deck surfaces it)")
            }
        }
    }
}

impl crate::sim::Strategy for StrategyEvaluator {
    fn decide(&self, state: &GameState, pending: &PendingDecision) -> Decision {
        match pending {
            PendingDecision::Priority { legal, .. } => {
                Decision::Act(self.decide_priority(state, legal))
            }
            PendingDecision::ChooseTargets { legal, .. } => {
                Decision::Targets(self.decide_targets(state, legal))
            }
            PendingDecision::DeclareAttackers { legal, .. } => {
                Decision::Attackers(self.decide_attackers(state, legal))
            }
            PendingDecision::DeclareBlockers { legal, .. } => {
                Decision::Blocks(self.decide_blocks(state, legal))
            }
            PendingDecision::DiscardToHandSize { player, count }
            | PendingDecision::DiscardCards { player, count } => {
                Decision::Discard(self.decide_discard(state, *player, *count))
            }
            other => self.fallback(state, other),
        }
    }
}

#[cfg(test)]
mod tests {
    use std::path::Path;
    use std::sync::Arc;

    use deckmaste_cards::plugin::Plugin;
    use deckmaste_core::Card;
    use deckmaste_core::Cmp;
    use deckmaste_core::Condition;
    use deckmaste_core::Count;
    use deckmaste_core::Reference;
    use deckmaste_core::Stat;
    use deckmaste_core::Zone;
    use deckmaste_core::strategy::BlockPolicy;
    use deckmaste_core::strategy::Extremum;
    use deckmaste_core::strategy::Preference;
    use deckmaste_core::strategy::Rule;
    use deckmaste_core::strategy::Selector;
    use deckmaste_core::strategy::Strategy as StrategyDef;

    use super::StrategyEvaluator;
    use super::eval_frame;
    use crate::Action;
    use crate::Decision;
    use crate::PendingDecision;
    use crate::object::ObjectId;
    use crate::object::ObjectSource;
    use crate::player::PlayerId;
    use crate::sim::Strategy as _;
    use crate::state::GameConfig;
    use crate::state::GameState;
    use crate::state::PlayerConfig;
    use crate::state::StartingPlayer;

    fn empty_two_player() -> GameState {
        GameState::new(GameConfig {
            players: vec![PlayerConfig { deck: vec![] }, PlayerConfig { deck: vec![] }],
            seed: 1,
            starting_life: 20,
            starting_player: StartingPlayer::Fixed(PlayerId(0)),
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
        Condition::AllOf(vec![])
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
        let bears = Arc::new(canon().card("Grizzly Bears").unwrap());
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
            state.eval_reference(&Reference::You, &frame),
            state.player(PlayerId(0)).object,
        );
        assert_eq!(state.eval_reference(&Reference::This, &frame), bear);

        // A `Count` over the candidate: Grizzly Bears' power is 2.
        assert_eq!(
            state.eval_count(&Count::StatOf(Reference::This, Stat::Power), &frame),
            2,
        );

        // A `Condition` comparing the candidate's power against a literal: 2 >= 2.
        let cond = Condition::Compare(
            Count::StatOf(Reference::This, Stat::Power),
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
        });
        let frame = eval_frame(&state, PlayerId(1), None);
        assert_eq!(
            state.eval_reference(&Reference::You, &frame),
            state.player(PlayerId(1)).object,
        );
        assert_eq!(
            state.eval_reference(&Reference::This, &frame),
            state.player(PlayerId(1)).object,
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
        let pending = PendingDecision::Priority {
            player: PlayerId(0),
            legal: vec![Action::Pass],
        };
        assert_eq!(eval.decide(&state, &pending), Decision::Act(Action::Pass));
    }

    /// Selector engine: `Cast(pick: Max, by: power)` over two legal cast
    /// candidates picks the bigger creature (Grizzly Bears 2/2 over Willow Elf
    /// 1/1) — argmax of a `Count` over the legal set.
    #[test]
    fn priority_cast_selector_picks_by_extremum() {
        let willow = Arc::new(canon().card("Willow Elf").unwrap());
        let bears = Arc::new(canon().card("Grizzly Bears").unwrap());
        let mut state = empty_two_player();
        let willow_id = put_creature(&mut state, &willow, PlayerId(0));
        let bears_id = put_creature(&mut state, &bears, PlayerId(0));

        let strat = StrategyDef {
            name: "cast-biggest".into(),
            rules: vec![Rule {
                when: always(),
                prefer: Preference::Cast {
                    what: Selector {
                        pick: Extremum::Max,
                        by: Count::StatOf(Reference::This, Stat::Power),
                        among: None,
                    },
                    target: None,
                },
            }],
        };
        let eval = StrategyEvaluator::new(strat, PlayerId(0));
        let pending = PendingDecision::Priority {
            player: PlayerId(0),
            legal: vec![
                Action::CastSpell { object: willow_id },
                Action::CastSpell { object: bears_id },
            ],
        };
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
        let pending = PendingDecision::DeclareBlockers {
            player: PlayerId(1),
            legal: vec![],
        };
        assert_eq!(eval.decide(&state, &pending), Decision::Blocks(vec![]));
    }

    /// ChooseTargets: the applicable `Cast` preference's `target` selector
    /// picks the biggest creature among a slot's legal candidates.
    #[test]
    fn choose_targets_applies_the_target_selector_per_slot() {
        let willow = Arc::new(canon().card("Willow Elf").unwrap());
        let bears = Arc::new(canon().card("Grizzly Bears").unwrap());
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
                    by: Count::StatOf(Reference::This, Stat::Power),
                    among: None,
                }),
            }),
            PlayerId(0),
        );
        let pending = PendingDecision::ChooseTargets {
            player: PlayerId(0),
            spec: vec![],
            legal: vec![vec![willow_id, bears_id]],
        };
        assert_eq!(
            eval.decide(&state, &pending),
            Decision::Targets(vec![bears_id]),
        );
    }

    /// ChooseTargets with no `target` rule falls back to the first legal
    /// candidate per slot (still total/legal).
    #[test]
    fn choose_targets_without_a_rule_takes_first_legal() {
        let eval = StrategyEvaluator::new(always_prefer(Preference::Pass), PlayerId(0));
        let mut state = empty_two_player();
        let bears = Arc::new(canon().card("Grizzly Bears").unwrap());
        let a = put_creature(&mut state, &bears, PlayerId(1));
        let b = put_creature(&mut state, &bears, PlayerId(1));
        let pending = PendingDecision::ChooseTargets {
            player: PlayerId(0),
            spec: vec![],
            legal: vec![vec![a, b]],
        };
        assert_eq!(eval.decide(&state, &pending), Decision::Targets(vec![a]));
    }

    /// DeclareAttackers: an `Attack` preference declares the whole legal set
    /// (no `among`); no `Attack` rule declares none.
    #[test]
    fn declare_attackers_attacks_all_legal_then_none() {
        let bears = Arc::new(canon().card("Grizzly Bears").unwrap());
        let mut state = empty_two_player();
        let a = put_creature(&mut state, &bears, PlayerId(0));
        let b = put_creature(&mut state, &bears, PlayerId(0));
        let pending = PendingDecision::DeclareAttackers {
            player: PlayerId(0),
            legal: vec![a, b],
        };

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
        assert_eq!(
            attacker.decide(&state, &pending),
            Decision::Attackers(vec![a, b]),
        );

        let passive = StrategyEvaluator::new(always_prefer(Preference::Pass), PlayerId(0));
        assert_eq!(
            passive.decide(&state, &pending),
            Decision::Attackers(vec![])
        );
    }

    /// DeclareBlockers: `Block(NoBlocks)` declares no blocks even with legal
    /// blockers available.
    #[test]
    fn declare_blockers_no_blocks_policy_blocks_nothing() {
        let bears = Arc::new(canon().card("Grizzly Bears").unwrap());
        let mut state = empty_two_player();
        let blocker = put_creature(&mut state, &bears, PlayerId(1));
        let eval = StrategyEvaluator::new(
            always_prefer(Preference::Block(BlockPolicy::NoBlocks)),
            PlayerId(1),
        );
        let pending = PendingDecision::DeclareBlockers {
            player: PlayerId(1),
            legal: vec![blocker],
        };
        assert_eq!(eval.decide(&state, &pending), Decision::Blocks(vec![]));
    }

    /// Discard: the `Discard` selector ranks the hand and sheds the `count`
    /// cheapest (Min by mana value) — Willow Elf (1) over Grizzly Bears (2).
    #[test]
    fn discard_sheds_cheapest_by_mana_value() {
        let willow = Arc::new(canon().card("Willow Elf").unwrap());
        let bears = Arc::new(canon().card("Grizzly Bears").unwrap());
        let mut state = empty_two_player();
        let bears_id = put_in_hand(&mut state, &bears, PlayerId(0));
        let willow_id = put_in_hand(&mut state, &willow, PlayerId(0));
        let _ = bears_id;

        let eval = StrategyEvaluator::new(
            always_prefer(Preference::Discard {
                what: Selector {
                    pick: Extremum::Min,
                    by: Count::StatOf(Reference::This, Stat::ManaValue),
                    among: None,
                },
            }),
            PlayerId(0),
        );
        let pending = PendingDecision::DiscardCards {
            player: PlayerId(0),
            count: 1,
        };
        assert_eq!(
            eval.decide(&state, &pending),
            Decision::Discard(vec![willow_id]),
        );
    }
}
