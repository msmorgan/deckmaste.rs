use std::collections::HashSet;
use std::sync::Arc;

use deckmaste_core::Uint;
use deckmaste_core::Zone;
use rand::RngExt;

use crate::agenda::WorkItem;
use crate::decide::Decision;
use crate::decide::DecisionError;
use crate::decide::DecisionHandler;
use crate::decide::DecisionPointKind;
use crate::decide::PreGameKind;
use crate::event::CoinFlipped;
use crate::event::GameEvent;
use crate::event::Occurrence;
use crate::event::ZoneChange;
use crate::object::ObjectId;
use crate::player::PlayerId;
use crate::state::GameState;

/// [CR#514.1]: discard down to maximum hand size.
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct DiscardToHandSize {
    pub player: PlayerId,
    pub count: Uint,
}

/// [CR#701.9b]: a resolving discard — `player` chooses which `count` cards
/// from their hand to discard (`count` already clamped to the hand size).
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct DiscardCards {
    pub player: PlayerId,
    pub count: Uint,
}

/// Shared answer-validate step for [`DiscardToHandSize`] and [`DiscardCards`]
/// — both decisions ask "which `count` cards do you discard?" and differ only
/// in why ([CR#514.1] cleanup vs. [CR#701.9b] a resolving discard), so both
/// handlers below call this one body instead of duplicating it.
pub(super) fn resolve_discard(
    g: &mut GameState,
    player: PlayerId,
    count: Uint,
    answer: Decision,
) -> Result<(), DecisionError> {
    let Decision::Discard(objects) = answer else {
        return Err(DecisionError::WrongKind);
    };
    g.submit_discards(player, count, objects)
}

impl DecisionHandler for DiscardToHandSize {
    fn resolve(self, g: &mut GameState, answer: Decision) -> Result<(), DecisionError> {
        resolve_discard(g, self.player, self.count, answer)
    }
}

impl DecisionHandler for DiscardCards {
    fn resolve(self, g: &mut GameState, answer: Decision) -> Result<(), DecisionError> {
        resolve_discard(g, self.player, self.count, answer)
    }
}

/// [CR#705.2]: a called coin flip — the flipper calls heads or tails
/// before the draw. Answered with `Decision::Answer` (`true` = heads).
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct CallFlip {
    pub player: PlayerId,
}

impl DecisionHandler for CallFlip {
    fn resolve(self, g: &mut GameState, answer: Decision) -> Result<(), DecisionError> {
        let Decision::Answer(call) = answer else {
            return Err(DecisionError::WrongKind);
        };
        g.pending = None;
        let cont = g
            .choice
            .take()
            .expect("a CallFlip decision stashed its continuation");
        let crate::state::DecisionContinuation::CallFlip {
            player,
            remaining,
            mut events,
        } = cont
        else {
            unreachable!("a CallFlip decision stashed a non-CallFlip continuation: {cont:?}")
        };
        // [CR#705.2]: the flipper called; call == result → win.
        let heads: bool = g.rng.random();
        events.push(GameEvent::CoinFlipped(CoinFlipped {
            player,
            heads,
            won: Some(call == heads),
        }));
        let remaining = remaining - 1;
        if remaining > 0 {
            g.pending = Some(DecisionPointKind::CallFlip(CallFlip { player }));
            g.choice = Some(crate::state::DecisionContinuation::CallFlip {
                player,
                remaining,
                events,
            });
        } else {
            // ONE simultaneous batch, like the uncalled path
            // ([CR#603.2c] — the multi-discard precedent).
            g.schedule_front(vec![WorkItem::Emit(Occurrence::Batch(events))]);
        }
        Ok(())
    }
}

/// [CR#603.3b]: a player controlling several simultaneous triggers orders
/// them. The submitted `Order` is a permutation of `0..triggers.len()`.
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct OrderTriggers {
    pub player: PlayerId,
    pub triggers: Vec<crate::trigger::NotedTrigger>,
}

impl DecisionHandler for OrderTriggers {
    fn resolve(self, g: &mut GameState, answer: Decision) -> Result<(), DecisionError> {
        let Decision::Order(order) = answer else {
            return Err(DecisionError::WrongKind);
        };
        let OrderTriggers { player, triggers } = self;
        g.submit_order_triggers(player, &triggers, &order)
    }
}

/// Divide damage/counters among targets ([CR#601.2d,608.2d]) — shell.
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct Division {
    pub player: PlayerId,
    pub total: Uint,
    pub targets: Vec<ObjectId>,
}

impl DecisionHandler for Division {
    fn resolve(self, _g: &mut GameState, _answer: Decision) -> Result<(), DecisionError> {
        todo!(
            "engine seam: dividing/distributing an effect among chosen \
             targets ([CR#601.2d]) — the announced division isn't applied \
             yet; owner: engine-divided-distribution-as-you-choose"
        )
    }
}

/// Vote, each player in turn order ([CR#701.38a]) — shell. A resolver must
/// collect multiple votes granted to one player in the same turn
/// ([CR#701.38d]).
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct Vote {
    pub player: PlayerId,
    pub options: Uint,
}

impl DecisionHandler for Vote {
    fn resolve(self, _g: &mut GameState, _answer: Decision) -> Result<(), DecisionError> {
        todo!(
            "engine seam: voting among players in turn order ([CR#701.38a]) \
             — submitted votes aren't tallied yet; owner: engine-voting-procedure"
        )
    }
}

/// A fixed-window yes/no ("… unless you pay", [CR#608.2d]) — shell.
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct YesNo {
    pub player: PlayerId,
}

impl DecisionHandler for YesNo {
    fn resolve(self, g: &mut GameState, answer: Decision) -> Result<(), DecisionError> {
        let Decision::Answer(yes) = answer else {
            return Err(DecisionError::WrongKind);
        };
        g.pending = None;
        let cont = g
            .choice
            .take()
            .expect("a YesNo decision stashed its continuation");
        match cont {
            // [CR#118.12]: `Instruction::May` — yes runs `effect` then
            // `if_did`; no runs `if_not` (or nothing). Front-scheduled
            // in order so `effect` precedes `if_did`.
            crate::state::DecisionContinuation::May { may, frame } => {
                if yes
                    && let Some((caster, object, alternative_cost)) =
                        g.may_cast_referent(&may, &frame)
                {
                    let items = g.may_cast_as_effect_items(
                        object,
                        caster,
                        alternative_cost,
                        may.if_did,
                        may.if_not,
                        frame,
                    );
                    g.schedule_front(items);
                    return Ok(());
                }
                let branch: Vec<Arc<deckmaste_core::Instruction>> = if yes {
                    std::iter::once(may.effect).chain(may.if_did).collect()
                } else {
                    may.if_not.into_iter().collect()
                };
                let items = branch
                    .into_iter()
                    .map(|effect| WorkItem::RunEffect {
                        effect,
                        frame: frame.clone(),
                    })
                    .collect();
                g.schedule_front(items);
            }
            // [CR#601.2b,702.33d]: "pay this tagged optional cost
            // (again)?" — yes records the tag and adds its components
            // to the total ([CR#601.2f]); a repeatable row re-offers
            // ([CR#702.33c] multikicker), else the walk advances.
            crate::state::DecisionContinuation::OptionalCost {
                tag,
                components,
                repeatable,
                index,
            } => {
                if yes {
                    let announce = g
                        .announcing
                        .as_mut()
                        .expect("an optional-cost announce in flight");
                    match announce.paid_costs.iter_mut().find(|(t, _)| *t == tag) {
                        Some(entry) => entry.1 += 1,
                        None => announce.paid_costs.push((tag, 1)),
                    }
                    announce.optional_components.extend(components);
                }
                let index = if yes && repeatable { index } else { index + 1 };
                g.schedule_front(vec![WorkItem::AnnounceOptionalCosts { index }]);
            }
            other => {
                unreachable!("a YesNo decision stashed a non-YesNo continuation: {other:?}")
            }
        }
        Ok(())
    }
}

/// [CR#608.2c,608.2d]: a resolution-time number choice for a
/// `ChooseAndNote(key, NotedKind::Number)` ("choose a number"). Any
/// nonnegative value is legal (unbounded, like `ChooseXValue`); the answer
/// — reusing the `Decision::XValue(Uint)` shape, which is exactly a chosen
/// nonnegative number — is stored in the armed activation register.
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct ChooseNoteNumber {
    pub player: PlayerId,
    pub key: deckmaste_core::Ident,
}

impl DecisionHandler for ChooseNoteNumber {
    fn resolve(self, g: &mut GameState, answer: Decision) -> Result<(), DecisionError> {
        let Decision::XValue(n) = answer else {
            return Err(DecisionError::WrongKind);
        };
        // [CR#608.2c,607.2]: record the chosen number in the destination
        // register for indexed reads within THIS resolution. Any value >= 0 is legal (unbounded,
        // like the X-announce), so no re-validation gate is needed.
        // The answer reuses `Decision::XValue` — a chosen non-negative
        // number — rather than mint a note-only twin.
        g.pending = None;
        let Some(crate::state::DecisionContinuation::BindNumber { dest, activation }) =
            g.choice.take()
        else {
            return Err(DecisionError::Illegal {
                reason: format!("number choice `{}` has no destination register", self.key),
            });
        };
        g.activation_write_number(activation, dest, n);
        Ok(())
    }
}

/// Choose a card name and note it for this resolution.
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct ChooseNoteCardName {
    pub player: PlayerId,
    pub key: deckmaste_core::Ident,
}

impl DecisionHandler for ChooseNoteCardName {
    fn resolve(self, g: &mut GameState, answer: Decision) -> Result<(), DecisionError> {
        let Decision::CardName(name) = answer else {
            return Err(DecisionError::WrongKind);
        };
        if name.is_empty() {
            return Err(DecisionError::Illegal {
                reason: "a card name can't be empty".to_owned(),
            });
        }
        g.pending = None;
        let Some(crate::state::DecisionContinuation::BindSymbol { dest, activation }) =
            g.choice.take()
        else {
            return Err(DecisionError::Illegal {
                reason: format!(
                    "card-name choice `{}` has no destination register",
                    self.key
                ),
            });
        };
        g.activation_write_symbol(activation, dest, name);
        Ok(())
    }
}

/// Order the replacement/prevention effects applicable to one event,
/// affected player/controller choosing ([CR#616.1]) — shell.
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct OrderReplacements {
    pub player: PlayerId,
    pub count: Uint,
}

impl DecisionHandler for OrderReplacements {
    fn resolve(self, _g: &mut GameState, _answer: Decision) -> Result<(), DecisionError> {
        todo!(
            "engine seam: ordering multiple applicable replacement/prevention \
             effects ([CR#616.1]) — the chosen order isn't applied yet; \
             owner: engine-order-replacements-decision"
        )
    }
}

/// [CR#616.1]: two or more replacement effects are applicable to one event;
/// the affected player chooses which to apply first. The loop resumes after
/// the choice via `ReplaceState` in `GameState.replace_state`.
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct ChooseReplacement {
    pub chooser: PlayerId,
    /// The replacement keys the player may choose among (all applicable
    /// and not yet in the [CR#614.5] lineage set for this event chain).
    pub applicable: Vec<crate::replace_registry::ReplacementKey>,
}

impl DecisionHandler for ChooseReplacement {
    fn resolve(self, g: &mut GameState, answer: Decision) -> Result<(), DecisionError> {
        let Decision::ReplacementChoice(key) = answer else {
            return Err(DecisionError::WrongKind);
        };
        // [CR#616.1]: validate the chosen key is in the offered set.
        if !self.applicable.contains(&key) {
            return Err(DecisionError::Illegal {
                reason: "chosen replacement key not in the applicable set".into(),
            });
        }
        // Take the suspended replacement state.
        let rs = g
            .replace_state
            .take()
            .expect("ChooseReplacement requires replace_state");
        g.pending = None;
        // Resume the replacement loop from the suspended state.
        crate::replace_registry::resume_replacements(g, rs, key);
        Ok(())
    }
}

/// A pre-game choice ([CR#103]) — shell.
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct PreGame {
    pub player: PlayerId,
    pub kind: PreGameKind,
}

impl DecisionHandler for PreGame {
    fn resolve(self, _g: &mut GameState, _answer: Decision) -> Result<(), DecisionError> {
        todo!(
            "engine seam: the pre-game procedure ([CR#103]) — submitted \
             pre-game choices aren't applied yet; owner: engine-pregame-procedure"
        )
    }
}

/// [CR#608.2d]: choose objects at resolution. `candidates` is the matching
/// set; the answer picks between `min` and `max` of them (both clamped to
/// `candidates.len()` — "as many as able").
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct ChooseObjects {
    pub player: PlayerId,
    pub candidates: Vec<ObjectId>,
    pub min: Uint,
    pub max: Uint,
}

impl DecisionHandler for ChooseObjects {
    fn resolve(self, g: &mut GameState, answer: Decision) -> Result<(), DecisionError> {
        let Decision::Chosen(chosen) = answer else {
            return Err(DecisionError::WrongKind);
        };
        let ChooseObjects {
            candidates,
            min,
            max,
            ..
        } = self;
        // [CR#608.2d]: distinct, all from the offered set, count in range.
        let chosen_count = Uint::try_from(chosen.len()).expect("chosen count fits Uint");
        let distinct: HashSet<_> = chosen.iter().copied().collect();
        let legal = distinct.len() == chosen.len()
            && chosen_count >= min
            && chosen_count <= max
            && chosen.iter().all(|id| candidates.contains(id));
        if !legal {
            return Err(DecisionError::Illegal {
                reason: "illegal object selection".into(),
            });
        }
        // All reads of `pending` are done; safe to mutate `g`.
        g.pending = None;
        match g
            .choice
            .take()
            .expect("a ChooseObjects decision stashed its continuation")
        {
            // [CR#608.2d]: the ordinary binder path — bind the picks
            // as `chosen` and re-run the choosing effect.
            crate::state::DecisionContinuation::BindChoice {
                dest,
                frame,
                if_none,
            } => {
                g.activation_write_objects(frame.activation, dest, &chosen);
                if chosen.is_empty() {
                    g.schedule_front(
                        if_none
                            .iter()
                            .cloned()
                            .map(|effect| WorkItem::RunEffect {
                                effect: std::sync::Arc::new(effect),
                                frame: frame.clone(),
                            })
                            .collect(),
                    );
                }
            }
            other => {
                unreachable!(
                    "a ChooseObjects decision stashes a BindChoice \
                     continuation, got {other:?}"
                )
            }
        }
        Ok(())
    }
}

/// [CR#704.5j] the legend rule: `player` controls two or more legendary
/// permanents with the same name (`candidates`); they choose exactly one to
/// keep and the rest are put into their owners' graveyards. Surfaced by the
/// SBA driver, resolved in `submit_decision`.
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct LegendRule {
    pub player: PlayerId,
    pub candidates: Vec<ObjectId>,
}

impl DecisionHandler for LegendRule {
    fn resolve(self, g: &mut GameState, answer: Decision) -> Result<(), DecisionError> {
        let Decision::Chosen(kept) = answer else {
            return Err(DecisionError::WrongKind);
        };
        // [CR#704.5j]: the player keeps exactly one of the candidate
        // legendaries; the rest are put into their owners' graveyards
        // as a move (not a destroy — indestructible does not prevent
        // this).
        let candidates = self.candidates;
        let kept_one = match kept.as_slice() {
            [one] if candidates.contains(one) => *one,
            _ => {
                return Err(DecisionError::Illegal {
                    reason: "the legend rule keeps exactly one of the candidates".into(),
                });
            }
        };
        let losers: Vec<GameEvent> = candidates
            .iter()
            .copied()
            .filter(|id| *id != kept_one)
            .map(|id| {
                GameEvent::ZoneChange(ZoneChange {
                    snapshot: None,
                    object: id,
                    from: Some(Zone::Battlefield),
                    to: Zone::Graveyard,
                    enters: None,
                    position: None,
                    face: None,
                    cause: None,
                })
            })
            .collect();
        g.pending = None;
        g.schedule_front(vec![
            WorkItem::Emit(Occurrence::Batch(losers)),
            WorkItem::CheckSbas,
        ]);
        Ok(())
    }
}

/// [CR#401.4]: `player` orders a pile of more than one card that came to
/// rest at one end of a library — scry's "on top … in any order" and
/// Brainstorm's "in any order". `objects` is the pile in its current
/// library order; the answer ([`Decision::Arranged`]) is a permutation of
/// it (top → down).
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct ArrangePile {
    pub player: PlayerId,
    pub objects: Vec<ObjectId>,
}

impl DecisionHandler for ArrangePile {
    fn resolve(self, g: &mut GameState, answer: Decision) -> Result<(), DecisionError> {
        let Decision::Arranged(order) = answer else {
            return Err(DecisionError::WrongKind);
        };
        // [CR#401.4]: the order must be a permutation of the offered
        // pile. Take the walk state, reorder that pile, then surface the
        // next pending pile (or finish).
        let arranger = self.player;
        let crate::state::DecisionContinuation::ArrangePiles { current, remaining } = g
            .choice
            .take()
            .expect("an ArrangePile decision stashed its continuation")
        else {
            unreachable!("an ArrangePile decision stashes an ArrangePiles continuation");
        };
        let want: HashSet<ObjectId> = current.objects.iter().copied().collect();
        let got: HashSet<ObjectId> = order.iter().copied().collect();
        if order.len() != current.objects.len() || want != got {
            // Restore the continuation so the (idempotent) decision can be
            // re-answered.
            g.choice =
                Some(crate::state::DecisionContinuation::ArrangePiles { current, remaining });
            return Err(DecisionError::Illegal {
                reason: "an arrangement is a permutation of the offered pile".into(),
            });
        }
        g.pending = None;
        g.apply_arranged(&current, &order);
        g.open_next_arrange(arranger, remaining);
        Ok(())
    }
}
