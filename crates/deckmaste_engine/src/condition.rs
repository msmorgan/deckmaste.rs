//! Condition evaluation ([CR#603.4] intervening-if, [CR#602.5b] activation
//! restrictions). Numeric comparisons route through the one count evaluator
//! (`GameState::eval_count`) — there is no frame-free count subset to drift
//! from it. The remaining `todo!` arms (e.g. history-lookback windows) widen
//! this dispatch in place rather than growing a second evaluator.

use deckmaste_core::Condition;
use deckmaste_core::PhaseStep;

use crate::player::PlayerId;
use crate::stack::Frame;
use crate::state::GameState;

impl GameState {
    /// Evaluate a `Condition` against the current game state in `frame` — the
    /// resolution context whose `controller` is the evaluating player (the
    /// "you" of `YourTurn` and similar) and whose bindings/targets resolve the
    /// references a `Condition::Matches` reads.
    ///
    /// One evaluator serves every site: the activation gate ([CR#602.5b]) and
    /// the trigger-fire gate build a minimal frame from what is known then (no
    /// targets), and the resolution recheck of an intervening-if ([CR#603.4])
    /// passes the resolving entry's full frame.
    pub(crate) fn condition_holds(&self, cond: &Condition, frame: &Frame) -> bool {
        let you = frame.controller;
        match cond {
            // "if you control a creature" / "if a creature is on the battlefield"
            // — threads the frame's watcher (mirrors `Condition::Is` below) so
            // a carrier-relative filter INSIDE the existential ("a creature
            // card directly above it" = `Adjacent(Above, Ref(This))`, Death
            // Spark) can anchor `Ref(This)`/`Ref(You)` instead of hitting the
            // frameless-targeting seam.
            Condition::Exists(filter) => {
                let watcher = self.frame_watcher(frame);
                !crate::target::candidates_with(self, filter, Some(watcher)).is_empty()
            }

            // "if it is a [filter]" ([CR#603.4], "if it's a …"): resolve the
            // reference and test the filter, anchoring `Ref(This)`/`Ref(You)`
            // inside it to the ability's source.
            //
            // When the reference is an object that has LEFT (a dies-trigger's
            // `This`/`EventObject` — the live id is stale once it is in the
            // graveyard), evaluate against its last-known information
            // ([CR#603.10a]) via `filter_matches_snapshot`: Undying/Persist's
            // intervening-if "if it had no +1/+1 (resp. -1/-1) counters on it"
            // ([CR#702.93a,702.79a]) reads the DYING object's captured counters,
            // which no live object holds. A live object takes the live path.
            Condition::Matches(reference, filter) => {
                // [CR#120.3,702.2c,704.5h]: `Source` is a set-valued binding
                // over the marked-damage sources of the object under evaluation
                // (the frame's `This`) — "was dealt damage by a source matching
                // F". It reads existentially against each mark's DEAL-TIME
                // abilities (the source may since have left or lost the
                // ability), so no live-object resolution applies; the
                // lethal-damage SBA's deathtouch clause is
                // `Is(Source, Has(Deathtouch))`.
                if let deckmaste_core::Reference::Source = reference {
                    let this = self.eval_reference(&deckmaste_core::Reference::This, frame);
                    return self.objects.get(this).is_some_and(|obj| {
                        obj.damage
                            .iter()
                            .any(|mark| source_abilities_match(filter, &mark.source_abilities))
                    });
                }
                let watcher = self.frame_watcher(frame);
                let object = self.eval_reference(reference, frame);
                if self.objects.get(object).is_some() {
                    self.filter_matches_live(filter, object, watcher)
                } else if let Some(snapshot) = Self::bound_snapshot(reference, frame) {
                    self.filter_matches_snapshot(filter, snapshot, watcher)
                } else {
                    // A gone object with no bound LKI snapshot matches no
                    // CURRENT-state filter — it is not on the battlefield, has no
                    // characteristics to test — so the sound answer is `false`
                    // (never a panic; authoring mistakes fizzle, [CR#608.2b]).
                    // First consumer: fight's both-or-neither guard
                    // `Is(Target(n), Creature)` when a fighter has left the
                    // battlefield ([CR#701.14b]) — the whole fight then no-ops.
                    // A gone reference that WANTS its last-known state reads via
                    // the snapshot path above (trigger roles), not here.
                    false
                }
            }

            // [CR#701.3b,303.4d,704.5m]: the referenced attachment is legally
            // attached iff it HAS a host and that (attachment, host) pair
            // passes `attachment_legal` (the same predicate the attach no-op
            // uses). Unattached / illegal-host / self-attached all read false —
            // exactly the Aura graveyard SBA's "or is not attached" trigger
            // (`Sba(Not(LegallyAttached(Ref(This))), …)`). Generic: no subtype
            // branch.
            Condition::LegallyAttached(reference) => {
                let object = self.eval_reference(reference, frame);
                self.objects
                    .obj(object)
                    .attached_to
                    .is_some_and(|host| crate::legal::attachment_legal(self, object, host))
            }

            // Numeric comparison: both sides ride the one `eval_count`, so a
            // `CountOf` here counts live objects exactly as it does at
            // resolution — no frame-free subset to fall out of sync.
            // The in-flight announce slot still counts toward a Stack census:
            // its object already sits in the Stack zone before the entry
            // commits ([CR#601.2a], set in `begin_cast`), so the zone-based
            // `CountOf` picks it up without a special case.
            Condition::Compare(a, op, b) => {
                op.apply(self.eval_count(a, frame), self.eval_count(b, frame))
            }

            Condition::And(cs) => cs.iter().all(|c| self.condition_holds(c, frame)),
            Condition::Or(cs) => cs.iter().any(|c| self.condition_holds(c, frame)),
            Condition::Not(c) => !self.condition_holds(c, frame),

            // Look through a macro.
            Condition::Expanded(e) => self.condition_holds(&e.value, frame),

            // "[event] happened within [lookback]" ([CR#608.2i]): any
            // recorded fact matching the pattern through the one evaluator's
            // History lane — each entry's per-fact LKI view ([CR#603.10a]),
            // so departed participants read their snapshots, never the live
            // store. The full frame rides the bindings: `Ref(This)`/`Ref(You)`
            // anchor on the frame's watcher, and bound references
            // (`Used(of: …)`) resolve through it.
            Condition::Happened { event, within } => {
                let bindings = crate::eval::Bindings {
                    watcher: self.frame_watcher(frame),
                    frame: Some(frame),
                    shape_only: false,
                };
                self.history
                    .in_window_for(
                        *within,
                        self.turn.turn_number,
                        self.turn.current,
                        self.turn.active_player,
                        frame.controller,
                    )
                    .filter_map(|(_, entry)| entry.view.as_ref())
                    .any(|view| self.eval(event, view, crate::eval::Lane::History, &bindings))
            }

            // [CR#714.2b]: "the total was less than N and became at least N"
            // — read off the firing counter fact's before/after channel
            // (never recomputed; `value` is the checker's tie to that
            // antecedent). A doubled 0→2 placement is ONE fact whose
            // crossing satisfies both a threshold of 1 and of 2.
            Condition::Crossed { threshold, .. } => {
                let Some((before, after)) = frame.anaphora.crossed else {
                    todo!("Crossed with no before/after channel in the frame is not yet supported")
                };
                !threshold.satisfied_by(before, |c| self.eval_count(c, frame))
                    && threshold.satisfied_by(after, |c| self.eval_count(c, frame))
            }

            // [CR#702.33d]: "was kicked" — the resolving entry's announced
            // optional-cost record carries the tag ([CR#601.2b,607.2]; the
            // record rides the STACK entry, so the read holds while this
            // object resolves — an ETB "if it was kicked" recheck after the
            // permanent lands is engine-alt-costs follow-up work).
            Condition::PaidCost(tag) => self
                .stack
                .iter()
                .find(|e| e.id == frame.source)
                .is_some_and(|e| e.paid_costs.iter().any(|(t, n)| t == tag && *n > 0)),

            // "if its [keyword] cost was paid" ([CR#702.34a,702.74a]) — the
            // ALTERNATIVE-cost twin of `PaidCost`. The announce record that
            // marks a spell as cast with an alternative cost is engine-alt-costs
            // (unbuilt), so no object is yet recorded as cast that way: the
            // rider it gates (Flashback's exile, Evoke's sacrifice) does not
            // fire until then. Conservative `false`, never a panic.
            Condition::CastWith(_tag) => false,

            // It is the evaluating player's turn — the frame-robust sugar for
            // `TurnOf(Ref(You))` (reads `you` directly, no carrier needed).
            Condition::YourTurn => self.turn.active_player == you,

            // It is a matching player's turn ([CR#603.4]): the active player's
            // proxy satisfies the player predicate, with `Ref(You)`/`Ref(This)`
            // in it anchored to the ability's carrier. `TurnOf(Ref(You))`
            // reduces to `YourTurn`; `TurnOf(OpponentOf(Ref(You)))` is "during
            // an opponent's turn".
            Condition::TurnOf(filter) => {
                let watcher = self.frame_watcher(frame);
                let active = self.player(self.turn.active_player).object;
                self.filter_matches_live(filter, active, watcher)
            }

            // The current phase/step is exactly the given one.
            Condition::DuringPhase(p) => self.turn.current == *p,
        }
    }

    /// The last-known-information snapshot a trigger bound for `reference`, if
    /// any ([CR#603.10a]) — the fallback a `Condition::Matches` over a gone
    /// object reads. Only the snapshot-bearing references resolve: `This` →
    /// the firing object's self, `EventObject` → the moved object, `It` →
    /// the iteration/projection element. Looks through an `Expanded` macro
    /// reference, mirroring `eval_reference`.
    fn bound_snapshot<'f>(
        reference: &deckmaste_core::Reference,
        frame: &'f Frame,
    ) -> Option<&'f crate::lki::LkiSnapshot> {
        use deckmaste_core::Reference;
        // The iteration/projection element ([CR#608.2]): a card element carries
        // a snapshot; a player element is zoneless and has none.
        if let Reference::It = reference {
            return match frame.anaphora.it.as_ref()? {
                crate::stack::ItBinding::Object(s) => Some(s),
                crate::stack::ItBinding::Player(_) => None,
            };
        }
        match reference {
            Reference::This => frame.this.as_ref(),
            // The event OBJECT — the moved object's snapshot ([CR#603.2e]).
            Reference::EventObject => frame.anaphora.that_object.as_ref(),
            // An OBJECT patient carries a snapshot; a player patient has none.
            Reference::EventPatient => match frame.anaphora.that_patient.as_ref()? {
                crate::trigger::EventPatient::Object(s) => Some(s),
                crate::trigger::EventPatient::Player(_) => None,
            },
            Reference::Expanded(e) => Self::bound_snapshot(&e.value, frame),
            _ => None,
        }
    }

    /// [CR#307.1,117.1a]: `player` could cast a sorcery right now — their
    /// turn, a main phase, stack (and announce slot) empty. The same facts
    /// `Timing::SorcerySpeed`'s activation gate reads; `kw-flash`'s
    /// `May(Cast(window: InstantSpeed))` will relax the spell-side caller.
    #[must_use]
    pub(crate) fn sorcery_speed_ok(&self, player: PlayerId) -> bool {
        player == self.turn.active_player
            && matches!(
                self.turn.current,
                PhaseStep::PrecombatMain | PhaseStep::PostcombatMain
            )
            && self.stack.is_empty()
            && self.announcing.is_none()
    }
}

/// Does a damage source's DEAL-TIME abilities satisfy `filter`? The matcher
/// behind `Is(Source, F)` ([CR#702.2c]): only what a mark captures — the
/// source's abilities — is testable, so keyword predicates (`Has(Deathtouch)`)
/// and their combinators are honored and every other predicate fizzles to
/// `false` (an authoring mistake no-ops, never crashes). Peels
/// `Innate`/`Expanded` via `ability_is_named`, matching the live `Has` arm.
fn source_abilities_match(
    filter: &deckmaste_core::Predicate,
    abilities: &[deckmaste_core::Ability],
) -> bool {
    use deckmaste_core::CharacteristicPredicate;
    use deckmaste_core::Predicate;
    if let Some(result) =
        crate::target::walk_combinators(filter, |f| source_abilities_match(f, abilities))
    {
        return result;
    }
    match filter {
        Predicate::Characteristic(CharacteristicPredicate::Has(name)) => abilities
            .iter()
            .any(|a| crate::layer::ability_is_named(a, &name.0)),
        _ => false,
    }
}

#[cfg(test)]
mod tests {
    use std::path::Path;
    use std::sync::Arc;

    use deckmaste_cards::plugin::Plugin;
    use deckmaste_core::BeginningStep;
    use deckmaste_core::Cmp;
    use deckmaste_core::Condition;
    use deckmaste_core::Count;
    use deckmaste_core::EventFilter;
    use deckmaste_core::Lookback;
    use deckmaste_core::PhaseStep;
    use deckmaste_core::Predicate;
    use deckmaste_core::Reference;
    use deckmaste_core::StatePredicate;
    use deckmaste_core::Type;
    use deckmaste_core::Uint;
    use deckmaste_core::Zone;

    use crate::event::GameEvent;
    use crate::lki::LkiSnapshot;
    use crate::object::ObjectSource;
    use crate::player::PlayerId;
    use crate::stack::Anaphora;
    use crate::stack::Frame;
    use crate::state::GameConfig;
    use crate::state::GameState;
    use crate::state::PlayerConfig;
    use crate::state::StartingPlayer;
    use crate::test_support::frame_for;

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

    fn builtin() -> Plugin {
        Plugin::load(Path::new(env!("CARGO_MANIFEST_DIR")).join("../../plugins/builtin")).unwrap()
    }
    fn canon() -> Plugin {
        Plugin::load_with_sibling_prelude(
            Path::new(env!("CARGO_MANIFEST_DIR")).join("../../plugins/canon"),
        )
        .unwrap()
    }

    /// Morbid ("a creature died this turn") is `Condition::Happened { ZoneMove
    /// { creature, Battlefield → Graveyard }, ThisTurn }`. It holds once a
    /// creature-death fact is in this turn's history; the `ThisGame` window
    /// sees it on a later turn while `ThisTurn` no longer does
    /// ([CR#608.2i]).
    #[test]
    fn happened_morbid_reads_history_window() {
        let bears = Arc::new(canon().card("Grizzly Bears").unwrap());
        let forest = Arc::new(builtin().card("Forest").unwrap());
        let mut state = GameState::new(GameConfig {
            players: vec![
                PlayerConfig {
                    deck: vec![Arc::clone(&bears); 10],
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
        state.turn.turn_number = 1;

        // Put a Grizzly Bears on the battlefield, snapshot it, build its death.
        let bear_card = state.cards.push(Arc::clone(&bears), PlayerId(0));
        let bear = state.objects.mint(
            ObjectSource::Card(bear_card),
            PlayerId(0),
            Some(Zone::Battlefield),
        );
        state.zones.battlefield.push(bear);
        let death = GameEvent::ZoneChange {
            object: bear,
            snapshot: Some(Box::new(LkiSnapshot::capture(&state, bear))),
            from: Some(Zone::Battlefield),
            to: Zone::Graveyard,
            enters: None,
            position: None,
            face: None,
            cause: None,
        };

        let morbid_pattern = EventFilter::ZoneChange {
            what: Predicate::creature(),
            from: Some(Zone::Battlefield),
            to: Some(Zone::Graveyard),
            cause: None,
        };
        let morbid = Condition::Happened {
            event: morbid_pattern.clone(),
            within: Lookback::ThisTurn,
        };
        let morbid_game = Condition::Happened {
            event: morbid_pattern,
            within: Lookback::ThisGame,
        };

        // No death yet → false.
        assert!(
            !state.condition_holds(&morbid, &frame_for(&state, PlayerId(0))),
            "no death recorded yet"
        );

        // Record the death this turn → ThisTurn and ThisGame both hold.
        state.record_history_fact(1, None, death);
        assert!(
            state.condition_holds(&morbid, &frame_for(&state, PlayerId(0))),
            "morbid holds after a creature dies this turn"
        );
        assert!(
            state.condition_holds(&morbid_game, &frame_for(&state, PlayerId(0))),
            "ThisGame sees this turn's death too"
        );

        // Advance a turn: ThisTurn no longer sees it; ThisGame still does.
        state.turn.turn_number = 2;
        assert!(
            !state.condition_holds(&morbid, &frame_for(&state, PlayerId(0))),
            "ThisTurn no longer sees last turn's death"
        );
        assert!(
            state.condition_holds(&morbid_game, &frame_for(&state, PlayerId(0))),
            "ThisGame still sees it"
        );
    }

    /// `Condition::Matches(ref, filter)` ([CR#603.4] "if it's a …") resolves
    /// the reference against the frame and tests the filter on it.
    /// `This`/`Target` pick the bear; the bear is a creature, not a land. A
    /// `Ref(This)` inside the filter anchors to the frame's source via
    /// `frame_watcher`, so `Is(This, Ref(This))` is true (the resolved
    /// object IS the watcher).
    #[test]
    fn is_reference_tests_filter_against_resolved_object() {
        let bears = Arc::new(canon().card("Grizzly Bears").unwrap());
        let forest = Arc::new(builtin().card("Forest").unwrap());
        let mut state = GameState::new(GameConfig {
            players: vec![
                PlayerConfig {
                    deck: vec![Arc::clone(&bears); 10],
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

        // A Grizzly Bears on the battlefield, and a frame whose `This`, sole
        // target, and source all point at it.
        let bear_card = state.cards.push(Arc::clone(&bears), PlayerId(0));
        let bear = state.objects.mint(
            ObjectSource::Card(bear_card),
            PlayerId(0),
            Some(Zone::Battlefield),
        );
        state.zones.battlefield.push(bear);
        let frame = Frame {
            source: bear,
            controller: PlayerId(0),
            this: Some(LkiSnapshot::capture(&state, bear)),
            defending_player: None,
            anaphora: Anaphora {
                targets: vec![vec![bear]],
                ..Anaphora::empty()
            },
        };

        let creature = Predicate::creature();
        let land = Predicate::type_(Type::Land);

        // Is(This, …): the bear is a creature …
        assert!(
            state.condition_holds(
                &Condition::Matches(Reference::This, creature.clone()),
                &frame
            ),
            "the bear is a creature"
        );
        // … and not a land.
        assert!(
            !state.condition_holds(&Condition::Matches(Reference::This, land), &frame),
            "the bear is not a land"
        );
        // Is(It, …): the lone announced target is that same bear.
        assert!(
            state.condition_holds(&Condition::Matches(Reference::It, creature), &frame),
            "the target is a creature"
        );
        // Is(This, Ref(This)): the resolved object IS the watcher, so the
        // self-reference inside the filter anchors and matches.
        assert!(
            state.condition_holds(
                &Condition::Matches(Reference::This, Predicate::Ref(Reference::This)),
                &frame
            ),
            "the resolved object is the frame's own source"
        );
    }

    /// [CR#603.4]: a triggered ability's intervening-if is rechecked as it
    /// resolves. A synthetic artifact (engine-test scaffolding, not a plugin
    /// fixture) whose trigger reads "if a creature is on the battlefield"
    /// resolves to its effect when a creature is present, and is removed from
    /// the stack with no effect — only the `AbilityResolved` that discards the
    /// entry, never a `RunEffect` — when the condition has become false. The
    /// no-condition/true path is also covered e2e by the integration suite
    /// (`etb_trigger_draws_a_card`). When canon grows an intervening-if card
    /// (parser/canon-slice work), an e2e fixture test is the natural follow-up.
    #[test]
    fn intervening_if_rechecked_at_resolution() {
        use deckmaste_core::Ability;
        use deckmaste_core::Card;
        use deckmaste_core::CardFace;
        use deckmaste_core::CharacteristicPredicate;
        use deckmaste_core::EventFilter;
        use deckmaste_core::OneShotEffect;
        use deckmaste_core::TriggeredAbility;

        use crate::agenda::WorkItem;
        use crate::stack::StackEntry;
        use crate::stack::StackObject;
        use crate::trigger::TriggerBindings;

        // Resolve the conditional trigger and report whether its effect was
        // scheduled (vs. removed with no effect).
        let runs_effect = |creature_present: bool| -> bool {
            let mut state = game();

            // A synthetic artifact whose sole ability triggers "if a creature
            // is on the battlefield". The event is irrelevant at resolution;
            // the effect is a no-op sequence — the test observes only whether
            // it gets scheduled.
            let card = Card::Normal(CardFace {
                name: "Conditional Trigger Artifact".into(),
                types: vec![Type::Artifact.def()],
                abilities: vec![Ability::Triggered(TriggeredAbility {
                    ability_word: None,
                    where_x: None,
                    from: None,
                    event: EventFilter::OneOf(Vec::new()),
                    condition: Some(Condition::Exists(Predicate::Characteristic(
                        CharacteristicPredicate::Type(Type::Creature.name()),
                    ))),
                    limits: Vec::new(),
                    effect: OneShotEffect::Sequentially(Vec::new()),
                })],
                ..CardFace::default()
            });
            let card_id = state.cards.push(Arc::new(card), PlayerId(0));
            let source = state.objects.mint(
                ObjectSource::Card(card_id),
                PlayerId(0),
                Some(Zone::Battlefield),
            );
            state.zones.battlefield.push(source);

            // The condition's subject: an actual creature, present or not.
            if creature_present {
                let bears = Arc::new(canon().card("Grizzly Bears").unwrap());
                let bear_card = state.cards.push(bears, PlayerId(0));
                let bear = state.objects.mint(
                    ObjectSource::Card(bear_card),
                    PlayerId(0),
                    Some(Zone::Battlefield),
                );
                state.zones.battlefield.push(bear);
            }

            // Put the fired trigger on the stack with its own minted id, then
            // resolve it.
            let stack_id =
                state
                    .objects
                    .mint(ObjectSource::Card(card_id), PlayerId(0), Some(Zone::Stack));
            state.stack.push(StackEntry {
                paid_costs: Vec::new(),
                id: stack_id,
                object: StackObject::Triggered {
                    source: ObjectSource::Card(card_id),
                    ability: 0,
                    created: None,
                    bindings: TriggerBindings {
                        this: Some(LkiSnapshot::capture(&state, source)),
                        ..Default::default()
                    },
                },
                controller: PlayerId(0),
                targets: Vec::new(),
                x: None,
                copy: false,
            });
            state.resolve_object(stack_id);

            state
                .agenda
                .iter()
                .any(|w| matches!(w, WorkItem::RunEffect { .. }))
        };

        assert!(
            runs_effect(true),
            "condition true at resolution → effect scheduled"
        );
        assert!(
            !runs_effect(false),
            "condition false at resolution → ability removed, no effect ([CR#603.4])"
        );
    }

    /// [CR#603.10a,702.93a,702.79a]: a dies-trigger intervening-if reads the
    /// DYING object's last-known counters. After the live object is gone (its
    /// id stale), `Is(This, HasCounter(P1P1Counter))` resolves against the LKI
    /// snapshot the fired trigger carries in `bindings.this`, not a live
    /// object. With +1/+1 counters captured it holds (so Undying's
    /// `Not(...)` intervening-if is false → no return); with none captured
    /// it is false (so Undying fires). The -1/-1 mirror covers Persist.
    #[test]
    fn is_this_over_dying_object_reads_snapshot_counters() {
        use deckmaste_core::CounterRef;

        // Build a frame whose `This` is the snapshot of a since-removed creature
        // carrying `counters`, and ask whether `Is(This, HasCounter(kind))`
        // holds. The live object is removed so only the snapshot path can answer.
        let holds = |counters: &[(&str, Uint)], kind: &str| -> bool {
            let bears = Arc::new(canon().card("Grizzly Bears").unwrap());
            let mut state = GameState::new(GameConfig {
                players: vec![
                    PlayerConfig {
                        deck: vec![Arc::clone(&bears); 4],
                    },
                    PlayerConfig {
                        deck: vec![Arc::clone(&bears); 4],
                    },
                ],
                seed: 9,
                starting_life: 20,
                starting_player: StartingPlayer::Fixed(PlayerId(0)),
                sba_rules: vec![],
                conferral_rules: vec![],
                damage_result_rules: vec![],
                counter_decls: std::collections::HashMap::new(),
                subtypes: std::collections::HashMap::new(),
                types: std::collections::HashMap::new(),
            });
            let bear_card = state.cards.push(Arc::clone(&bears), PlayerId(0));
            let bear = state.objects.mint(
                ObjectSource::Card(bear_card),
                PlayerId(0),
                Some(Zone::Battlefield),
            );
            for (k, n) in counters {
                state.objects.obj_mut(bear).counters.insert((*k).into(), *n);
            }
            // Snapshot the creature, then remove it (it "died"): the id is now
            // stale, so a live read would fail — only the snapshot can answer.
            let snapshot = LkiSnapshot::capture(&state, bear);
            state.objects.remove(bear);
            assert!(
                state.objects.get(bear).is_none(),
                "the dying object's id is stale after removal"
            );

            let frame = Frame {
                this: Some(snapshot),
                ..Frame::bare(bear, PlayerId(0))
            };
            let cond = Condition::Matches(
                Reference::This,
                Predicate::State(StatePredicate::HasCounter(CounterRef::from(kind))),
            );
            state.condition_holds(&cond, &frame)
        };

        // Undying: "had no +1/+1 counters" reads the snapshot's +1/+1 count.
        assert!(
            holds(&[("P1P1Counter", 1)], "P1P1Counter"),
            "a +1/+1 counter on the dying object reads true via LKI"
        );
        assert!(
            !holds(&[], "P1P1Counter"),
            "no counters → HasCounter(P1P1Counter) is false via LKI"
        );
        // A different kind on the object does not satisfy the queried kind.
        assert!(
            !holds(&[("M1M1Counter", 2)], "P1P1Counter"),
            "only -1/-1 counters → HasCounter(P1P1Counter) is false"
        );
        // Persist: the -1/-1 mirror.
        assert!(
            holds(&[("M1M1Counter", 1)], "M1M1Counter"),
            "a -1/-1 counter on the dying object reads true via LKI"
        );
        assert!(
            !holds(&[], "M1M1Counter"),
            "no counters → HasCounter(M1M1Counter) is false via LKI"
        );

        // The full Undying intervening-if `Not(Is(This, HasCounter(P1P1)))`:
        // false (don't return) when a +1/+1 counter was present, true (return)
        // when none.
        let undying_if = |counters: &[(&str, Uint)]| -> bool { !holds(counters, "P1P1Counter") };
        assert!(
            !undying_if(&[("P1P1Counter", 1)]),
            "Undying intervening-if is false when it had a +1/+1 counter"
        );
        assert!(
            undying_if(&[]),
            "Undying intervening-if is true when it had no +1/+1 counters"
        );
    }

    /// `YourTurn` is true for the active player, false for the other.
    /// `DuringPhase` matches exactly the current phase and no other.
    #[test]
    fn your_turn_and_phase() {
        let mut state = game();
        state.turn.active_player = PlayerId(0);
        state.turn.current = PhaseStep::PrecombatMain;

        // YourTurn
        assert!(
            state.condition_holds(&Condition::YourTurn, &frame_for(&state, PlayerId(0))),
            "YourTurn should hold for the active player"
        );
        assert!(
            !state.condition_holds(&Condition::YourTurn, &frame_for(&state, PlayerId(1))),
            "YourTurn should not hold for the non-active player"
        );

        // DuringPhase — exact match
        assert!(
            state.condition_holds(
                &Condition::DuringPhase(PhaseStep::PrecombatMain),
                &frame_for(&state, PlayerId(0))
            ),
            "DuringPhase(PrecombatMain) should hold during PrecombatMain"
        );
        assert!(
            !state.condition_holds(
                &Condition::DuringPhase(PhaseStep::PostcombatMain),
                &frame_for(&state, PlayerId(0))
            ),
            "DuringPhase(PostcombatMain) should not hold during PrecombatMain"
        );
    }

    /// `TurnOf(<player predicate>)` generalizes `YourTurn`: `TurnOf(Ref(You))`
    /// agrees with `YourTurn` for every (active player, evaluator) pairing, and
    /// `TurnOf(OpponentOf(Ref(You)))` is its complement — "during an opponent's
    /// turn" ([CR#603.4]).
    #[test]
    fn turn_of_generalizes_your_turn() {
        use deckmaste_core::RelationPredicate;

        let your_turn = Condition::YourTurn;
        let turn_of_you = Condition::TurnOf(Predicate::Ref(Reference::You));
        let turn_of_opp = Condition::TurnOf(Predicate::Relation(RelationPredicate::OpponentOf(
            Box::new(Predicate::Ref(Reference::You)),
        )));

        for active in [PlayerId(0), PlayerId(1)] {
            let mut state = game();
            state.turn.active_player = active;
            for evaluator in [PlayerId(0), PlayerId(1)] {
                let frame = frame_for(&state, evaluator);
                let is_your_turn = state.condition_holds(&your_turn, &frame);
                // `TurnOf(Ref(You))` matches `YourTurn` exactly.
                assert_eq!(
                    state.condition_holds(&turn_of_you, &frame),
                    is_your_turn,
                    "TurnOf(Ref(You)) must agree with YourTurn (active={active:?}, eval={evaluator:?})"
                );
                // An opponent's turn is the complement of your turn (1v1).
                assert_eq!(
                    state.condition_holds(&turn_of_opp, &frame),
                    !is_your_turn,
                    "TurnOf(OpponentOf(You)) is the complement of YourTurn (active={active:?}, eval={evaluator:?})"
                );
            }
        }
    }

    /// `Compare(CountOf(InZone(Stack)), Eq, Literal(0))` is the core of the
    /// builtin `SorcerySpeed` macro. Fresh game has an empty stack and no
    /// announce slot, so the condition holds. An in-flight announce makes it
    /// false — the announce slot counts as a stack occupant
    /// ([CR#601.2a]).
    #[test]
    fn compare_counts_stack_census() {
        let mut state = game();
        let cond = Condition::Compare(
            Count::CountOf(deckmaste_core::Countable::Objects(Box::new(
                Predicate::State(StatePredicate::InZone(Zone::Stack)),
            ))),
            Cmp::Eq,
            Count::Literal(0),
        );
        // Fresh game: stack empty, no announce slot.
        assert!(
            state.condition_holds(&cond, &frame_for(&state, PlayerId(0))),
            "Compare(CountOf(InZone(Stack)), Eq, Literal(0)) should hold on a fresh game (stack empty)"
        );

        // In-flight announce: the slot counts as a stack occupant.
        let spell = state.objects.mint(
            crate::object::ObjectSource::Player(PlayerId(0)),
            PlayerId(0),
            Some(deckmaste_core::Zone::Stack),
        );
        state.announcing = Some(crate::stack::PendingStackEntry {
            optional_components: Vec::new(),
            paid_costs: Vec::new(),
            id: spell,
            object: crate::stack::StackObject::Spell(spell),
            controller: PlayerId(0),
            origin: deckmaste_core::Zone::Hand,
            targets: vec![],
            x: None,
            concretized: None,
        });
        assert!(
            !state.condition_holds(&cond, &frame_for(&state, PlayerId(0))),
            "Compare(CountOf(InZone(Stack)), Eq, Literal(0)) should be false with an in-flight announce"
        );
    }

    /// `Compare` over a non-Stack `CountOf` — the release-blocker the
    /// frame-free evaluator `todo!`d. `Compare(CountOf(creature), Eq,
    /// Literal(1))` must evaluate the live creature cardinality through the
    /// unified `eval_count`, not panic. The bare `Type(Creature)` filter is
    /// zone-agnostic, so the decks are all-land (`GameState::new` mints
    /// deck cards as library objects); the one battlefield Grizzly Bears is
    /// then the only creature.
    #[test]
    fn compare_counts_nonstack_filter() {
        use deckmaste_core::CharacteristicPredicate;

        let bears = Arc::new(canon().card("Grizzly Bears").unwrap());
        let forest = Arc::new(builtin().card("Forest").unwrap());
        let mut state = GameState::new(GameConfig {
            players: vec![
                PlayerConfig {
                    deck: vec![Arc::clone(&forest); 4],
                },
                PlayerConfig {
                    deck: vec![Arc::clone(&forest); 4],
                },
            ],
            seed: 3,
            starting_life: 20,
            starting_player: StartingPlayer::Fixed(PlayerId(0)),
            sba_rules: vec![],
            conferral_rules: vec![],
            damage_result_rules: vec![],
            counter_decls: std::collections::HashMap::new(),
            subtypes: std::collections::HashMap::new(),
            types: std::collections::HashMap::new(),
        });
        let bear_card = state.cards.push(bears, PlayerId(0));
        let bear = state.objects.mint(
            ObjectSource::Card(bear_card),
            PlayerId(0),
            Some(Zone::Battlefield),
        );
        state.zones.battlefield.push(bear);

        let creatures = Count::CountOf(deckmaste_core::Countable::Objects(Box::new(
            Predicate::Characteristic(CharacteristicPredicate::Type(Type::Creature.name())),
        )));
        assert!(
            state.condition_holds(
                &Condition::Compare(creatures.clone(), Cmp::Eq, Count::Literal(1)),
                &frame_for(&state, PlayerId(0))
            ),
            "exactly one creature on the battlefield"
        );
        assert!(
            !state.condition_holds(
                &Condition::Compare(creatures, Cmp::Eq, Count::Literal(0)),
                &frame_for(&state, PlayerId(0))
            ),
            "there IS a creature, so == 0 is false"
        );
    }

    /// [CR#702.100a] Evolve's intervening-if is a CROSS-OBJECT stat compare:
    /// "that creature's power is greater than this creature's power AND/OR that
    /// creature's toughness is greater than this creature's toughness". Because
    /// `Condition::Compare` takes two full `Count`s (not a value-vs-literal),
    /// `Compare(StatOf(EventObject, _), Greater, StatOf(This, _))` compares two
    /// DIFFERENT objects' derived stats directly — `This` = the Evolve carrier
    /// (Grizzly Bears, 2/2), `EventObject` = the entering creature, bound by
    /// the trigger ([CR#603.10a]). The "and/or" is `Or`. No core
    /// addition is needed; this test pins that the gap is already
    /// representable AND engine-executable.
    #[test]
    fn evolve_cross_object_stat_compare() {
        use deckmaste_core::Stat;

        use crate::object::ObjectSource;

        let bears = Arc::new(canon().card("Grizzly Bears").unwrap()); // 2/2 carrier
        let courser = Arc::new(canon().card("Centaur Courser").unwrap()); // 3/3
        let spider = Arc::new(canon().card("Giant Spider").unwrap()); // 2/4
        let phantasm = Arc::new(canon().card("Phantasmal Bear").unwrap()); // 2/2
        let forest = Arc::new(builtin().card("Forest").unwrap());
        let mut state = GameState::new(GameConfig {
            players: vec![
                PlayerConfig {
                    deck: vec![Arc::clone(&forest); 4],
                },
                PlayerConfig {
                    deck: vec![Arc::clone(&forest); 4],
                },
            ],
            seed: 5,
            starting_life: 20,
            starting_player: StartingPlayer::Fixed(PlayerId(0)),
            sba_rules: vec![],
            conferral_rules: vec![],
            damage_result_rules: vec![],
            counter_decls: std::collections::HashMap::new(),
            subtypes: std::collections::HashMap::new(),
            types: std::collections::HashMap::new(),
        });

        // `This`: the Evolve carrier, a 2/2 Grizzly Bears.
        let carrier_card = state.cards.push(bears, PlayerId(0));
        let carrier = state.objects.mint(
            ObjectSource::Card(carrier_card),
            PlayerId(0),
            Some(Zone::Battlefield),
        );
        state.zones.battlefield.push(carrier);

        // The Evolve intervening-if, parameterized over the entering creature
        // already bound as the event `EventObject`.
        let evolve_if = Condition::Or(vec![
            Condition::Compare(
                Count::StatOf(Reference::EventObject, Stat::Power),
                Cmp::Greater,
                Count::StatOf(Reference::This, Stat::Power),
            ),
            Condition::Compare(
                Count::StatOf(Reference::EventObject, Stat::Toughness),
                Cmp::Greater,
                Count::StatOf(Reference::This, Stat::Toughness),
            ),
        ]);

        // Build a trigger-style frame whose `This` is the carrier and whose
        // `EventObject` is the just-entered creature `entrant`.
        let frame_for_entrant = |state: &GameState, entrant| Frame {
            this: Some(LkiSnapshot::capture(state, carrier)),
            anaphora: Anaphora {
                that_object: Some(LkiSnapshot::capture(state, entrant)),
                ..Anaphora::empty()
            },
            ..Frame::bare(carrier, PlayerId(0))
        };

        let enter = |state: &mut GameState, card: &Arc<deckmaste_core::Card>| {
            let cid = state.cards.push(Arc::clone(card), PlayerId(0));
            let id = state.objects.mint(
                ObjectSource::Card(cid),
                PlayerId(0),
                Some(Zone::Battlefield),
            );
            state.zones.battlefield.push(id);
            id
        };

        // 3/3 enters: greater power AND greater toughness → evolves.
        let big = enter(&mut state, &courser);
        assert!(
            state.condition_holds(&evolve_if, &frame_for_entrant(&state, big)),
            "a 3/3 entering vs a 2/2 carrier evolves (greater power and toughness)"
        );

        // 2/4 enters: equal power, GREATER toughness → still evolves (the OR's
        // toughness branch carries it).
        let tall = enter(&mut state, &spider);
        assert!(
            state.condition_holds(&evolve_if, &frame_for_entrant(&state, tall)),
            "a 2/4 entering vs a 2/2 carrier evolves on toughness alone (and/or)"
        );

        // 2/2 enters: equal power AND equal toughness, neither GREATER → does
        // not evolve ([CR#702.100a]: strictly greater, not >=).
        let equal = enter(&mut state, &phantasm);
        assert!(
            !state.condition_holds(&evolve_if, &frame_for_entrant(&state, equal)),
            "a 2/2 entering vs a 2/2 carrier does NOT evolve (no strictly-greater stat)"
        );
    }

    /// [CR#702.54a] Bloodthirst's gate is the history condition "an opponent
    /// was dealt damage this turn": `Happened(Damage(to:
    /// OpponentOf(Ref(You))), within: ThisTurn)`. The engine's `Happened` scans
    /// the turn history reusing the trigger matcher; `Damage` matches a
    /// `DamageDealt` fact with `to` = the LIVE recipient ([CR#120.1]), so a
    /// recipient who is an opponent of the carrier's controller passes
    /// `OpponentOf(Ref(You))`. `Lookback::ThisTurn` is a real history-lookback
    /// window — unlike Echo's "since your last upkeep", Bloodthirst needs no
    /// new primitive. This test pins that the gap is already engine-executable:
    /// false before any damage, true after damage to an opponent, and NOT
    /// fooled by damage dealt to the carrier's own controller.
    #[test]
    fn bloodthirst_opponent_damaged_this_turn() {
        use deckmaste_core::Predicate;
        use deckmaste_core::RelationPredicate;

        use crate::event::GameEvent;

        let mut state = game();
        state.turn.turn_number = 1;

        // The Bloodthirst gate, evaluated from player 0's seat (You = P0).
        let gate = Condition::Happened {
            event: EventFilter::Damage {
                source: Predicate::any(),
                to: Predicate::Relation(RelationPredicate::OpponentOf(Box::new(Predicate::Ref(
                    Reference::You,
                )))),
                combat: None,
                amount: None,
            },
            within: Lookback::ThisTurn,
        };

        // Player proxies are objects; damage to a player targets its proxy.
        let p0 = state.player(PlayerId(0)).object;
        let p1 = state.player(PlayerId(1)).object;

        // No damage yet → the gate is false.
        assert!(
            !state.condition_holds(&gate, &frame_for(&state, PlayerId(0))),
            "no opponent has been damaged yet"
        );

        // Damage dealt to player 0 (You) this turn must NOT satisfy "an
        // opponent was dealt damage" from player 0's seat.
        state.record_history_fact(
            1,
            None,
            GameEvent::DamageDealt {
                source: p1,
                target: p0,
                amount: 2,
                combat: false,
            },
        );
        assert!(
            !state.condition_holds(&gate, &frame_for(&state, PlayerId(0))),
            "damage to YOU is not damage to an opponent"
        );

        // Damage dealt to player 1 (an opponent of P0) this turn → the gate
        // holds from player 0's seat.
        state.record_history_fact(
            1,
            None,
            GameEvent::DamageDealt {
                source: p0,
                target: p1,
                amount: 3,
                combat: false,
            },
        );
        assert!(
            state.condition_holds(&gate, &frame_for(&state, PlayerId(0))),
            "an opponent was dealt damage this turn → Bloodthirst is on"
        );

        // Turn advances: the ThisTurn window no longer sees last turn's hit.
        state.turn.turn_number = 2;
        assert!(
            !state.condition_holds(&gate, &frame_for(&state, PlayerId(0))),
            "ThisTurn no longer sees last turn's opponent damage"
        );
    }

    /// THE per-fact LKI fix (engine-one-evaluator, [CR#603.10a]): a history
    /// query over a participant that has since LEFT reads its record-time
    /// snapshot — the previously documented
    /// `Happened(Damage(to: Type(Creature)))`-over-a-dead-recipient hazard
    /// (a live-store read through a stale id, an `obj()` panic) now
    /// answers correctly.
    #[test]
    fn happened_damage_to_a_departed_creature_reads_its_snapshot() {
        use deckmaste_core::CharacteristicPredicate;
        use deckmaste_core::Predicate;

        use crate::event::GameEvent;

        let mut state = game();
        state.turn.turn_number = 1;
        let bear = Arc::new(canon().card("Grizzly Bears").unwrap());
        let card = state.cards.push(bear, PlayerId(0));
        let id = state.objects.mint(
            ObjectSource::Card(card),
            PlayerId(0),
            Some(Zone::Battlefield),
        );
        state.zones.battlefield.push(id);
        let p1 = state.player(PlayerId(1)).object;

        // Damage recorded while the bear lives — its snapshot rides the
        // history entry.
        state.record_history_fact(
            1,
            None,
            GameEvent::DamageDealt {
                source: p1,
                target: id,
                amount: 2,
                combat: false,
            },
        );

        // The bear departs: its id is stale everywhere.
        state.zones.battlefield.retain(|&o| o != id);
        state.objects.remove(id);
        assert!(state.objects.get(id).is_none(), "the recipient is gone");

        let damaged = |filter: Predicate| Condition::Happened {
            event: EventFilter::Damage {
                source: Predicate::any(),
                to: filter,
                combat: None,
                amount: None,
            },
            within: Lookback::ThisTurn,
        };
        assert!(
            state.condition_holds(
                &damaged(Predicate::Characteristic(CharacteristicPredicate::Type(
                    Type::Creature.name()
                ))),
                &frame_for(&state, PlayerId(0)),
            ),
            "the departed recipient WAS a creature — read through its snapshot, no panic"
        );
        assert!(
            !state.condition_holds(
                &damaged(Predicate::Characteristic(CharacteristicPredicate::Type(
                    Type::Land.name()
                ))),
                &frame_for(&state, PlayerId(0)),
            ),
            "the snapshot still discriminates — the recipient was never a land"
        );
    }

    /// The StepBegins-in-history lift ([CR#603.2b,608.2i]): step onsets are
    /// recorded with the active player of record, so `whose` ([CR#503.1])
    /// evaluates in history reads — `EachPlayers` sees the onset, `Your`
    /// keys on the recorded active player, `AnOpponents` on its complement.
    #[test]
    fn happened_step_begins_reads_the_recorded_onset() {
        use deckmaste_core::WhoseTurn;

        use crate::event::GameEvent;

        let mut state = game();
        state.turn.turn_number = 1;
        state.turn.active_player = PlayerId(0);
        state.record_history_fact(
            1,
            None,
            GameEvent::StepBegan(PhaseStep::Beginning(BeginningStep::Upkeep)),
        );

        let began = |whose: WhoseTurn| Condition::Happened {
            event: EventFilter::StepBegins {
                at: PhaseStep::Beginning(BeginningStep::Upkeep),
                whose,
            },
            within: Lookback::ThisTurn,
        };
        assert!(
            state.condition_holds(
                &began(WhoseTurn::EachPlayers),
                &frame_for(&state, PlayerId(0))
            ),
            "the recorded upkeep onset is visible"
        );
        assert!(
            state.condition_holds(&began(WhoseTurn::Your), &frame_for(&state, PlayerId(0))),
            "it began on the reader's own turn of record"
        );
        assert!(
            !state.condition_holds(&began(WhoseTurn::Your), &frame_for(&state, PlayerId(1))),
            "it did not begin on the opponent-reader's turn"
        );
        assert!(
            state.condition_holds(
                &began(WhoseTurn::AnOpponents),
                &frame_for(&state, PlayerId(1))
            ),
            "from the opponent's seat the onset was on an opponent's turn"
        );
    }

    /// Echo's upkeep lookback starts at the controller's PREVIOUS upkeep while
    /// the current upkeep onset is firing. Once that upkeep has passed, the
    /// current onset becomes the newest anchor ([CR#702.30a]).
    #[test]
    fn happened_since_your_upkeep_uses_the_previous_onset_while_firing() {
        use crate::event::GameEvent;
        use crate::object::ObjectId;

        let mut state = game();
        let upkeep = PhaseStep::Beginning(BeginningStep::Upkeep);

        state.turn.turn_number = 0;
        state.record_history_fact(0, None, GameEvent::SpellCast(ObjectId::from_raw(20)));

        state.turn.turn_number = 1;
        state.turn.active_player = PlayerId(0);
        state.turn.current = upkeep;
        state.record_history_fact(1, None, GameEvent::StepBegan(upkeep));

        state.turn.turn_number = 2;
        state.turn.active_player = PlayerId(1);
        state.record_history_fact(2, None, GameEvent::SpellCast(ObjectId::from_raw(21)));

        state.turn.turn_number = 3;
        state.turn.active_player = PlayerId(0);
        state.turn.current = upkeep;
        state.record_history_fact(3, None, GameEvent::StepBegan(upkeep));

        let cast_since = Condition::Happened {
            event: EventFilter::Cast {
                who: Predicate::any(),
                what: Predicate::any(),
            },
            within: Lookback::SinceYour(upkeep),
        };
        assert!(
            state.condition_holds(&cast_since, &frame_for(&state, PlayerId(0))),
            "the spell after the prior upkeep is inside echo's firing window"
        );

        state.turn.current = PhaseStep::Beginning(BeginningStep::Draw);
        assert!(
            !state.condition_holds(&cast_since, &frame_for(&state, PlayerId(0))),
            "after upkeep, the current upkeep onset is the newest anchor"
        );
    }

    /// The lifted pattern-level `Within` refinement in history lanes
    /// ([CR#608.2i]): the fact's own recorded turn bounds the inner window
    /// even under a wider head window.
    #[test]
    fn happened_within_refinement_bounds_by_the_facts_turn() {
        use crate::event::GameEvent;

        let mut state = game();
        state.turn.turn_number = 2;
        // A life gain recorded LAST turn.
        state.record_history_fact(
            1,
            None,
            GameEvent::LifeGained {
                player: PlayerId(0),
                amount: 3,
            },
        );

        let gained_this_turn = Condition::Happened {
            event: EventFilter::Within(
                Box::new(EventFilter::LifeGained {
                    who: deckmaste_core::Predicate::any(),
                    amount: None,
                }),
                Lookback::ThisTurn,
            ),
            within: Lookback::ThisGame,
        };
        assert!(
            !state.condition_holds(&gained_this_turn, &frame_for(&state, PlayerId(0))),
            "last turn's gain is outside the inner ThisTurn window"
        );

        state.record_history_fact(
            2,
            None,
            GameEvent::LifeGained {
                player: PlayerId(0),
                amount: 1,
            },
        );
        assert!(
            state.condition_holds(&gained_this_turn, &frame_for(&state, PlayerId(0))),
            "this turn's gain satisfies the inner window"
        );
    }

    /// Combinators: `Not(And([Or([])]))` is true because `Or([])` is
    /// vacuously false → `And` of a false is false → `Not` of false is true.
    #[test]
    fn combinators() {
        let state = game();
        let p = PlayerId(0);
        let cond = Condition::Not(Box::new(Condition::And(vec![Condition::Or(vec![])])));
        assert!(
            state.condition_holds(&cond, &frame_for(&state, p)),
            "Not(And([Or([])])) should be true (vacuous Or false → And false → Not true)"
        );
    }

    /// [CR#702.131c]: "if you have the city's blessing" reads the player-scope
    /// designation store via `Is(You, Designated(...))`.
    #[test]
    fn has_citys_blessing_reads_player_designation() {
        let mut state = game();
        let p0 = PlayerId(0);
        let cond = Condition::Matches(
            Reference::You,
            Predicate::State(StatePredicate::Designated("CitysBlessing".into())),
        );
        assert!(
            !state.condition_holds(&cond, &frame_for(&state, p0)),
            "no blessing yet"
        );
        state.designations.players.insert(
            (p0, "CitysBlessing".into()),
            crate::state::DesignationValue::Flag,
        );
        assert!(
            state.condition_holds(&cond, &frame_for(&state, p0)),
            "blessing now read true via the player proxy"
        );
    }

    /// `sorcery_speed_ok` is gated by active player, main phase, and empty
    /// stack/announce.
    #[test]
    fn sorcery_speed_ok_gates() {
        let mut state = game();
        state.turn.active_player = PlayerId(0);
        state.turn.current = PhaseStep::PrecombatMain;

        assert!(
            state.sorcery_speed_ok(PlayerId(0)),
            "sorcery_speed_ok should be true for active player in main phase with empty stack"
        );
        assert!(
            !state.sorcery_speed_ok(PlayerId(1)),
            "sorcery_speed_ok should be false for non-active player"
        );

        // Wrong phase
        state.turn.current = PhaseStep::Beginning(BeginningStep::Upkeep);
        assert!(
            !state.sorcery_speed_ok(PlayerId(0)),
            "sorcery_speed_ok should be false outside main phases"
        );
    }
}
