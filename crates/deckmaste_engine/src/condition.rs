//! Condition evaluation ([CR#603.4] intervening-if, [CR#602.5b] activation
//! restrictions). Numeric comparisons route through the one count evaluator
//! (`GameState::eval_count`) — there is no frame-free count subset to drift
//! from it. The remaining `todo!` arms (e.g. history-lookback windows) widen
//! this dispatch in place rather than growing a second evaluator.

use deckmaste_core::Condition;
use deckmaste_core::Phase;
use deckmaste_core::Window;

use crate::player::PlayerId;
use crate::stack::Frame;
use crate::state::GameState;

impl GameState {
    /// Evaluate a `Condition` against the current game state in `frame` — the
    /// resolution context whose `controller` is the evaluating player (the
    /// "you" of `YourTurn` and similar) and whose bindings/targets resolve the
    /// references a `Condition::Is` reads.
    ///
    /// One evaluator serves every site: the activation gate ([CR#602.5b]) and
    /// the trigger-fire gate build a minimal frame from what is known then (no
    /// targets), and the resolution recheck of an intervening-if ([CR#603.4])
    /// passes the resolving entry's full frame.
    pub(crate) fn condition_holds(&self, cond: &Condition, frame: &Frame) -> bool {
        let you = frame.controller;
        match cond {
            // "if you control a creature" / "if a creature is on the battlefield"
            Condition::Exists(filter) => !crate::target::candidates(self, filter).is_empty(),

            // "if it is a [filter]" ([CR#603.4], "if it's a …"): resolve the
            // reference to a live object and test the filter, anchoring
            // `Ref(This)`/`Ref(You)` inside it to the ability's source. A
            // snapshot-bound reference (e.g. `ThatObject` for an object that
            // already left) rides `eval_reference`'s reference-breadth seam
            // (`engine-resolve-selections`).
            Condition::Is(reference, filter) => {
                let object = self.eval_reference(reference, frame);
                self.filter_matches_live(filter, object, self.frame_watcher(frame))
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

            Condition::AllOf(cs) => cs.iter().all(|c| self.condition_holds(c, frame)),
            Condition::OneOf(cs) => cs.iter().any(|c| self.condition_holds(c, frame)),
            Condition::Not(c) => !self.condition_holds(c, frame),

            // Look through a macro.
            Condition::Expanded(e) => self.condition_holds(&e.value, frame),

            // "[event] happened within [window]" ([CR#608.2i]): scan the
            // history log for any recorded fact matching the pattern, reusing
            // the trigger event-matcher. The frame's source anchors a
            // `Ref(This)` in the pattern's filter ([CR#603.10a]).
            Condition::Happened { event, within } => {
                let watcher = self.frame_watcher(frame);
                match within {
                    Window::ThisTurn | Window::ThisGame => self
                        .history
                        .scan(*within, self.turn.turn_number)
                        .any(|fact| self.event_matches(event, fact, watcher)),
                    other => todo!("{other:?} is not a history-lookback window for Happened"),
                }
            }

            // It is the evaluating player's turn.
            Condition::YourTurn => self.turn.active_player == you,

            // The current phase/step is exactly the given one.
            Condition::DuringPhase(p) => self.turn.current == *p,
        }
    }

    /// [CR#307.1,117.1a]: `player` could cast a sorcery right now — their
    /// turn, a main phase, stack (and announce slot) empty. The same facts
    /// `Window::SorcerySpeed`'s activation gate reads; `kw-flash`'s
    /// `May(Cast(window: InstantSpeed))` will relax the spell-side caller.
    #[must_use]
    pub(crate) fn sorcery_speed_ok(&self, player: PlayerId) -> bool {
        player == self.turn.active_player
            && matches!(
                self.turn.current,
                Phase::PrecombatMain | Phase::PostcombatMain
            )
            && self.stack.is_empty()
            && self.announcing.is_none()
    }
}

#[cfg(test)]
mod tests {
    use std::path::Path;
    use std::sync::Arc;

    use deckmaste_cards::plugin::Plugin;
    use deckmaste_core::BeginningStep;
    use deckmaste_core::CharacteristicFilter;
    use deckmaste_core::Cmp;
    use deckmaste_core::Condition;
    use deckmaste_core::Count;
    use deckmaste_core::Event;
    use deckmaste_core::Filter;
    use deckmaste_core::Phase;
    use deckmaste_core::Reference;
    use deckmaste_core::StateFilter;
    use deckmaste_core::Type;
    use deckmaste_core::Window;
    use deckmaste_core::Zone;

    use crate::event::GameEvent;
    use crate::lki::LkiSnapshot;
    use crate::object::ObjectSource;
    use crate::player::PlayerId;
    use crate::stack::Frame;
    use crate::state::GameConfig;
    use crate::state::GameState;
    use crate::state::PlayerConfig;
    use crate::state::StartingPlayer;
    use crate::trigger::TriggerBindings;

    fn game() -> GameState {
        GameState::new(GameConfig {
            players: vec![PlayerConfig { deck: vec![] }, PlayerConfig { deck: vec![] }],
            seed: 7,
            starting_life: 20,
            starting_player: StartingPlayer::Fixed(PlayerId(0)),
        })
    }

    /// A minimal player-anchored frame (no bindings, no targets) — the
    /// gate-time shape, enough to evaluate the context-free conditions these
    /// unit tests exercise.
    fn frame_for(state: &GameState, player: PlayerId) -> Frame {
        Frame {
            source: state.player(player).object,
            controller: player,
            targets: Vec::new(),
            bindings: None,
            chosen: None,
            x: None,
        }
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
        let death = GameEvent::ZoneChanged {
            snapshot: LkiSnapshot::capture(&state, bear),
            from: Some(Zone::Battlefield),
            to: Zone::Graveyard,
            face: None,
            cause: None,
        };

        let morbid_pattern = Event::ZoneMove {
            what: Filter::Characteristic(CharacteristicFilter::Type(Type::Creature)),
            from: Some(Zone::Battlefield),
            to: Some(Zone::Graveyard),
            face: None,
            cause: None,
        };
        let morbid = Condition::Happened {
            event: morbid_pattern.clone(),
            within: Window::ThisTurn,
        };
        let morbid_game = Condition::Happened {
            event: morbid_pattern,
            within: Window::ThisGame,
        };

        // No death yet → false.
        assert!(
            !state.condition_holds(&morbid, &frame_for(&state, PlayerId(0))),
            "no death recorded yet"
        );

        // Record the death this turn → ThisTurn and ThisGame both hold.
        state.history.record(1, death);
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

    /// `Condition::Is(ref, filter)` ([CR#603.4] "if it's a …") resolves the
    /// reference against the frame and tests the filter on it. `This`/`Target`
    /// pick the bear; the bear is a creature, not a land. A `Ref(This)` inside
    /// the filter anchors to the frame's source via `frame_watcher`, so
    /// `Is(This, Ref(This))` is true (the resolved object IS the watcher).
    #[test]
    fn is_reference_tests_filter_against_resolved_object() {
        use deckmaste_core::CharacteristicFilter;

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
            targets: vec![bear],
            bindings: Some(TriggerBindings {
                this: Some(LkiSnapshot::capture(&state, bear)),
                that_object: None,
                that_player: None,
            }),
            chosen: None,
            x: None,
        };

        let creature = Filter::Characteristic(CharacteristicFilter::Type(Type::Creature));
        let land = Filter::Characteristic(CharacteristicFilter::Type(Type::Land));

        // Is(This, …): the bear is a creature …
        assert!(
            state.condition_holds(&Condition::Is(Reference::This, creature.clone()), &frame),
            "the bear is a creature"
        );
        // … and not a land.
        assert!(
            !state.condition_holds(&Condition::Is(Reference::This, land), &frame),
            "the bear is not a land"
        );
        // Is(Target(0), …): the announced target is that same bear.
        assert!(
            state.condition_holds(&Condition::Is(Reference::Target(0), creature), &frame),
            "the target is a creature"
        );
        // Is(This, Ref(This)): the resolved object IS the watcher, so the
        // self-reference inside the filter anchors and matches.
        assert!(
            state.condition_holds(
                &Condition::Is(Reference::This, Filter::Ref(Reference::This)),
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
        use deckmaste_core::CharacteristicFilter;
        use deckmaste_core::Effect;
        use deckmaste_core::Event;
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
                types: vec![Type::Artifact],
                abilities: vec![Ability::Triggered(TriggeredAbility {
                    event: Event::OneOf(Vec::new()),
                    condition: Some(Condition::Exists(Filter::Characteristic(
                        CharacteristicFilter::Type(Type::Creature),
                    ))),
                    limits: Vec::new(),
                    targets: Vec::new(),
                    effect: Effect::Sequence(Vec::new()),
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
                id: stack_id,
                object: StackObject::Triggered {
                    source: ObjectSource::Card(card_id),
                    ability: 0,
                    bindings: TriggerBindings {
                        this: Some(LkiSnapshot::capture(&state, source)),
                        that_object: None,
                        that_player: None,
                    },
                },
                controller: PlayerId(0),
                targets: Vec::new(),
                x: None,
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

    /// `YourTurn` is true for the active player, false for the other.
    /// `DuringPhase` matches exactly the current phase and no other.
    #[test]
    fn your_turn_and_phase() {
        let mut state = game();
        state.turn.active_player = PlayerId(0);
        state.turn.current = Phase::PrecombatMain;

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
                &Condition::DuringPhase(Phase::PrecombatMain),
                &frame_for(&state, PlayerId(0))
            ),
            "DuringPhase(PrecombatMain) should hold during PrecombatMain"
        );
        assert!(
            !state.condition_holds(
                &Condition::DuringPhase(Phase::PostcombatMain),
                &frame_for(&state, PlayerId(0))
            ),
            "DuringPhase(PostcombatMain) should not hold during PrecombatMain"
        );
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
            Count::CountOf(Box::new(Filter::State(StateFilter::InZone(Zone::Stack)))),
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
        use deckmaste_core::CharacteristicFilter;

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
        });
        let bear_card = state.cards.push(bears, PlayerId(0));
        let bear = state.objects.mint(
            ObjectSource::Card(bear_card),
            PlayerId(0),
            Some(Zone::Battlefield),
        );
        state.zones.battlefield.push(bear);

        let creatures = Count::CountOf(Box::new(Filter::Characteristic(
            CharacteristicFilter::Type(Type::Creature),
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

    /// Combinators: `Not(AllOf([OneOf([])]))` is true because `OneOf([])` is
    /// vacuously false → `AllOf` of a false is false → `Not` of false is true.
    #[test]
    fn combinators() {
        let state = game();
        let p = PlayerId(0);
        let cond = Condition::Not(Box::new(Condition::AllOf(vec![Condition::OneOf(vec![])])));
        assert!(
            state.condition_holds(&cond, &frame_for(&state, p)),
            "Not(AllOf([OneOf([])])) should be true (vacuous OneOf false → AllOf false → Not true)"
        );
    }

    /// [CR#702.131c]: "if you have the city's blessing" reads the player-scope
    /// designation store via `Is(You, Designated(...))`.
    #[test]
    fn has_citys_blessing_reads_player_designation() {
        let mut state = game();
        let p0 = PlayerId(0);
        let cond = Condition::Is(
            Reference::You,
            Filter::State(StateFilter::Designated("CitysBlessing".into())),
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
        state.turn.current = Phase::PrecombatMain;

        assert!(
            state.sorcery_speed_ok(PlayerId(0)),
            "sorcery_speed_ok should be true for active player in main phase with empty stack"
        );
        assert!(
            !state.sorcery_speed_ok(PlayerId(1)),
            "sorcery_speed_ok should be false for non-active player"
        );

        // Wrong phase
        state.turn.current = Phase::Beginning(BeginningStep::Upkeep);
        assert!(
            !state.sorcery_speed_ok(PlayerId(0)),
            "sorcery_speed_ok should be false outside main phases"
        );
    }
}
