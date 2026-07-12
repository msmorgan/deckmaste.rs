//! `eval_count`: evaluate a [`Count`] — aggregates, history facts, devotion,
//! arithmetic — to a number.

use deckmaste_core::Count;
use deckmaste_core::Countable;
use deckmaste_core::Reference;
use deckmaste_core::Uint;

use crate::event::GameEvent;
use crate::object::ObjectId;
use crate::stack::Anaphora;
use crate::stack::Frame;
use crate::state::GameState;

impl GameState {
    /// Evaluate a `Count` to a concrete number.
    ///
    /// # Panics
    ///
    /// Panics on a `Count` not wired for Stage 3, on a `StatOf` whose object
    /// lacks the stat, and on a `ThatMuch` with no amount fixed in this
    /// resolution.
    #[expect(
        clippy::too_many_lines,
        reason = "one arm per Count kind — the value language's full surface"
    )]
    pub(crate) fn eval_count(&self, qty: &Count, frame: &Frame) -> Uint {
        match qty {
            Count::Literal(n) => *n,
            // "For each …": the filter's live cardinality over every object
            // (card objects in all zones + player proxies) — canonical
            // filters are context-free-correct, so they carry their own
            // zone/kind narrowing. The watcher anchors `Ref(This)` to the
            // frame's announce-time self, the way `eval_reference` does.
            Count::CountOf(source) => match source {
                // [CR#119.1]: a player filter's cardinality — players are
                // objects too (proxies in `self.objects`), matched by the
                // same live filter (`CountOf (Players OpponentOf)` = "the
                // number of opponents you have").
                Countable::Objects(filter) | Countable::Players(filter) => {
                    let watcher = self.frame_watcher(frame);
                    let n = self
                        .objects
                        .iter()
                        .filter(|ob| self.filter_matches_live(filter, ob.id, watcher))
                        .count();
                    Uint::try_from(n).expect("object count fits Uint")
                }
                // [CR#700.5]: devotion — count mana symbols in the
                // referenced object's printed cost matching `pred`. A
                // stale/absent/non-card-backed reference fizzles to 0
                // (never-crash on an authoring mistake).
                Countable::ManaSymbols(reference, pred) => {
                    let id = self.eval_reference(reference, frame);
                    let n = match self
                        .objects
                        .get(id)
                        .and_then(crate::object::GameObject::card_id)
                    {
                        Some(_) => crate::derive::face(self.def(id))
                            .mana_cost
                            .iter()
                            .filter(|sym| pred.matches(sym))
                            .count(),
                        None => 0,
                    };
                    Uint::try_from(n).expect("pip count fits Uint")
                }
                // [CR#105.2]: the cardinality of a one-object "set" — 1 when
                // the reference resolves to a real card-backed object, else 0
                // (never-crash, mirroring the `ManaSymbols` guard above).
                Countable::Singleton(reference) => {
                    let id = self.eval_reference(reference, frame);
                    match self
                        .objects
                        .get(id)
                        .and_then(crate::object::GameObject::card_id)
                    {
                        Some(_) => 1,
                        None => 0,
                    }
                }
                // [CR#107.4]: mana spent to cast/activate a referenced object,
                // filtered by `pred` (Adamant). The engine has no mana-spent
                // tracking yet (a separate, unforced piece of work) — fizzles
                // to 0, like the `ManaSymbols`/`CountDistinct` gaps below.
                Countable::ManaSpentMatching(..) => 0,
            },
            // "Equal to its power": resolve the reference, read the DERIVED
            // stat off the layer view ([CR#613]; per-call rebuild — the same
            // documented perf seam as `target::matches`'s `Has` arm). A
            // negative result counts as 0 ([CR#107.1b]).
            Count::StatOf(reference, stat) => {
                let id = self.eval_reference(reference, frame);
                let value = match stat {
                    deckmaste_core::Stat::Power => self
                        .layers()
                        .power(id)
                        .expect("StatOf(Power) on an object with a power"),
                    deckmaste_core::Stat::Toughness => self
                        .layers()
                        .toughness(id)
                        .expect("StatOf(Toughness) on an object with a toughness"),
                    // [CR#202.3]: the printed cost's total. The on-stack
                    // announced-X contribution ([CR#202.3e]) rides the
                    // announce-slot X work (see `Count::X` below).
                    deckmaste_core::Stat::ManaValue => {
                        let face = crate::derive::face(self.def(id));
                        deckmaste_core::Int::try_from(face.mana_cost.mana_value())
                            .expect("mana value fits Int")
                    }
                    // [CR#209.1,306.5a]: `Stat::Loyalty` is the PRINTED loyalty
                    // characteristic off the card face — never the live counter
                    // count (current loyalty is `CounterCount(This,
                    // LoyaltyCounter)`). `Number(n)→n`, `DefinedByAbility`/
                    // `Variable`/absent → 0 (the chosen-X for a `Variable`
                    // loyalty rides the unbuilt announce-slot X work).
                    deckmaste_core::Stat::Loyalty => {
                        crate::layer::base_stat(crate::derive::face(self.def(id)).loyalty.as_ref())
                            .unwrap_or(0)
                    }
                    deckmaste_core::Stat::Defense => deckmaste_core::Int::try_from(
                        self.objects
                            .obj(id)
                            .counters
                            .get("DefenseCounter")
                            .copied()
                            .unwrap_or(0),
                    )
                    .expect("defense fits Int"),
                };
                Uint::try_from(value.max(0)).expect("clamped stat fits Uint")
            }
            // [CR#119.1,402.2]: a player's numeric attribute — resolve to a
            // player proxy, then read the folded attribute ([CR#611] player
            // statics). A non-player reference fizzles to 0 (never-crash).
            Count::PlayerStatOf(reference, attr) => self
                .eval_player_ref(reference, frame)
                .map_or(0, |p| self.player_attr(p, *attr)),
            // [CR#102.1]: how many opponents the referenced player has — the
            // live players not on that player's team ([CR#102.4]).
            Count::Opponents(reference) => self
                .eval_player_ref(reference, frame)
                .map_or(0, |p| self.opponent_count(p)),
            // [CR#122.1]: the count of a counter kind on the resolved
            // object/player proxy, read off the raw counter map (not the
            // derived view — counter quantities are base state, so no layers
            // recursion). An absent kind is zero.
            //
            // [CR#603.10a] LKI: a dies/leaves trigger reads "for each +1/+1
            // counter on this permanent" AFTER the object is gone (Modular,
            // [CR#702.43a]). `This`/`EventObject` then resolves to the firing
            // object's now-stale id; the live object store no longer holds it,
            // so the count comes from the trigger's last-known snapshot instead.
            Count::CounterCount(reference, kind) => {
                let id = self.eval_reference(reference, frame);
                match self.objects.get(id) {
                    Some(o) => o.counters.get(kind.as_str()).copied().unwrap_or(0),
                    None => lki_counters(reference, frame)
                        .and_then(|c| c.get(kind.as_str()).copied())
                        .unwrap_or(0),
                }
            }
            // [CR#704.5q]: the lesser of two magnitudes (annihilation removes
            // the smaller of the two counter counts of each kind).
            Count::Min(a, b) => self.eval_count(a, frame).min(self.eval_count(b, frame)),
            // [CR#107.1] basic value arithmetic. A count never goes negative
            // ([CR#107.1b]), so `Minus` floors at 0 (saturating); `Plus`/`Times`
            // saturate at the `Uint` ceiling rather than wrapping.
            Count::Max(a, b) => self.eval_count(a, frame).max(self.eval_count(b, frame)),
            Count::Plus(a, b) => self
                .eval_count(a, frame)
                .saturating_add(self.eval_count(b, frame)),
            Count::Minus(a, b) => self
                .eval_count(a, frame)
                .saturating_sub(self.eval_count(b, frame)),
            Count::Times(a, b) => self
                .eval_count(a, frame)
                .saturating_mul(self.eval_count(b, frame)),
            // [CR#107.1a]: half, rounded per the mode ("half its power rounded
            // up" = `Half(RoundUp, StatOf(This, Power))`).
            Count::Half(mode, inner) => {
                let v = self.eval_count(inner, frame);
                match mode {
                    deckmaste_core::RoundMode::RoundUp => v.div_ceil(2),
                    deckmaste_core::RoundMode::RoundDown => v / 2,
                }
            }
            // [CR#107.1a]: `Half`'s general twin — divide by an arbitrary
            // count, rounded per the mode. A zero divisor fizzles to 0
            // (never-crash) rather than panicking on integer division.
            Count::Divide(mode, a, b) => {
                let a = self.eval_count(a, frame);
                let b = self.eval_count(b, frame);
                if b == 0 {
                    0
                } else {
                    match mode {
                        deckmaste_core::RoundMode::RoundUp => a.div_ceil(b),
                        deckmaste_core::RoundMode::RoundDown => a / b,
                    }
                }
            }
            // [CR#107.1]: remainder — parity checks read `Compare(Mod(x, 2),
            // Eq, 0)`. A zero divisor fizzles to 0 (never-crash).
            Count::Mod(a, b) => {
                let a = self.eval_count(a, frame);
                let b = self.eval_count(b, frame);
                if b == 0 { 0 } else { a % b }
            }
            // [CR#107.1]: exponentiation — doubling effects build `Pow(2,
            // X)`. Saturates at the `Uint` ceiling, like `Times`.
            Count::Pow(base, exp) => {
                let base = self.eval_count(base, frame);
                let exp = self.eval_count(exp, frame);
                base.saturating_pow(exp)
            }
            // [CR#115.9a]: how many targets `reference`'s own stack entry
            // carries — Strive's "for each target beyond the first" reads
            // `Minus(TargetsOf(This), 1)`. A reference that isn't (or is no
            // longer) on the stack fizzles to 0 (never-crash).
            Count::TargetsOf(reference) => {
                let id = self.eval_reference(reference, frame);
                self.stack.iter().find(|e| e.id == id).map_or(0, |e| {
                    // The FLATTENED target instance count across every slot
                    // ([CR#115.9a] — "for each target beyond the first").
                    Uint::try_from(e.targets.iter().map(Vec::len).sum::<usize>())
                        .expect("target count fits Uint")
                })
            }
            // [CR#107.3]: the size of the distinct union of a characteristic
            // across the matching objects (Domain = distinct land subtypes;
            // Coven = distinct creature powers; Tarmogoyf = distinct card types
            // in graveyards). Anchors `Ref(This)` like `CountOf`.
            Count::CountDistinct(characteristic, source) => match source {
                Countable::Objects(filter) => {
                    let watcher = self.frame_watcher(frame);
                    let mut seen = std::collections::BTreeSet::new();
                    for ob in self.objects.iter() {
                        if self.filter_matches_live(filter, ob.id, watcher) {
                            for key in self.distinct_keys(*characteristic, ob.id) {
                                seen.insert(key);
                            }
                        }
                    }
                    Uint::try_from(seen.len()).expect("distinct count fits Uint")
                }
                // [CR#105.2]: the distinct-union axis read off a SINGLE
                // object — Embiggen's "number of card types it has" =
                // `CountDistinct(Types, Singleton(This))`. A stale/absent/
                // non-card-backed reference fizzles to 0 (never-crash).
                Countable::Singleton(reference) => {
                    let id = self.eval_reference(reference, frame);
                    match self
                        .objects
                        .get(id)
                        .and_then(crate::object::GameObject::card_id)
                    {
                        Some(_) => {
                            let n = self.distinct_keys(*characteristic, id).len();
                            Uint::try_from(n).expect("distinct count fits Uint")
                        }
                        None => 0,
                    }
                }
                // None of these is a forced distinct-union path ([CR#700.5]
                // devotion has no distinct-union reading; Idris's
                // `readableOn` doesn't gate `ManaSpentMatching` either, but
                // the engine has no mana-spent tracking to read; `readableOn`
                // never grants `Players` a characteristic axis at all — a
                // player has no printed characteristic to distinctly union)
                // — fizzle to 0.
                Countable::ManaSymbols(..)
                | Countable::ManaSpentMatching(..)
                | Countable::Players(..) => 0,
            },
            // The amount fixed by an earlier instruction of this resolution —
            // recorded at the apply funnel (so it reads what actually
            // happened, post-replacement) — or, for a triggered ability, the
            // firing event's magnitude seeded from `TriggerBindings.that_much`
            // by `resolve_object`. Still loud when neither fixed an amount:
            // that is an authoring error (a `ThatMuch` with no antecedent
            // magnitude), not an engine seam.
            Count::ThatMany | Count::ThatMuch => self.that_much.unwrap_or_else(|| {
                panic!(
                    "ThatMany/ThatMuch with no amount fixed this resolution and no \
                     trigger-bound magnitude — the card authors a magnitude anaphor \
                     with no antecedent"
                )
            }),
            // [CR#601.2d]: the per-element share in scope inside a `Distribute`
            // body — `Distribute` puts it in the `allotment` slot per element
            // (the Idris `bindAllot`), and an inner `Each`/`Distribute` clears
            // it (the Idris allotment-clearing `bindIt`), so reading it outside a
            // `Distribute` body — or inside a nested loop that rebound `It` — is
            // a malformed card.
            Count::Allotment => frame.anaphora.allotment.expect(
                "Count::Allotment outside a Distribute body (or inside a nested Each/Distribute \
                 that cleared the outer share)",
            ),
            // [CR#107.3a]: while a spell/ability is on the stack, X equals the
            // value announced as it was cast (engine-x-costs threads it onto the
            // resolution frame). [CR#107.3f] text-X chosen at resolution is a
            // separate seam.
            Count::X => frame.anaphora.x.expect(
                "Count::X on a frame with no announced X — a card referenced X without an {X} cost",
            ),
            // [CR#608.2i]: count history facts matching `event` within `within` —
            // the count-valued twin of `Condition::Happened`, through the one
            // evaluator's History lane over per-fact LKI views ([CR#603.10a]).
            // The frame rides the bindings, so `Ref(This)`/`Ref(You)` anchor
            // on the evaluating source and a `Used(of: …)` resolves its
            // object-scoped identity ([CR#400.7]) — no per-form special case.
            Count::EventCount(event, within) => {
                let bindings = crate::eval::Bindings {
                    watcher: self.frame_watcher(frame),
                    frame: Some(frame),
                    shape_only: false,
                };
                let n = self
                    .history
                    .in_window(*within, self.turn.turn_number)
                    .filter_map(|(_, entry)| entry.view.as_ref())
                    .filter(|view| self.eval(event, view, crate::eval::Lane::History, &bindings))
                    .count();
                Uint::try_from(n).expect("event count fits Uint")
            }
            // [CR#608.2i,119.3]: sum the carried amount of history facts that
            // match `event` within `within` — the sum-valued twin of
            // `EventCount`; each matching entry's magnitude comes from
            // `game_event_amount`.
            Count::EventSum(event, within) => {
                let bindings = crate::eval::Bindings {
                    watcher: self.frame_watcher(frame),
                    frame: Some(frame),
                    shape_only: false,
                };
                self.history
                    .in_window(*within, self.turn.turn_number)
                    .filter(|(_, entry)| {
                        entry.view.as_ref().is_some_and(|view| {
                            self.eval(event, view, crate::eval::Lane::History, &bindings)
                        })
                    })
                    .map(|(_, entry)| Self::game_event_amount(&entry.fact))
                    .sum()
            }
            // [CR#607.2,608.2c]: a scalar note read back in the SAME resolution
            // ("that number"). A present `Number` note reads its value; a
            // MISSING key (or a future non-number value) is an authoring
            // mistake, not a legal 0 — the engine-stat-none-fizzle ruling says
            // fizzle the consuming read, never a silent bare 0. `eval_count`
            // returns a bare `Uint` with no fizzle channel, so — exactly as the
            // `unbound_ref` no-op does for references — we leave a LOUD
            // `eprintln!` breadcrumb and degrade to 0 (no `debug_assert`: the
            // fizzle must never panic; a fizzling `Count` read is the principled
            // fix, tracked by engine-stat-none-fizzle).
            Count::Noted(key) => match self.resolution_notes.get(key) {
                Some(crate::state::NotedValue::Number(n)) => *n,
                absent_or_mistyped => {
                    eprintln!(
                        "deckmaste: Count::Noted({key:?}) found no number note \
                         ({absent_or_mistyped:?}); treating as 0 — effect fizzles \
                         (engine-stat-none-fizzle)"
                    );
                    0
                }
            },
            // [CR#120.3]: the damage marked on the referenced object — read
            // directly off the base state (damage is not a derived stat).
            // [CR#702.33c..702.33d]: multikicker's per-payment count needs
            // the optional-cost announce record (engine-alt-costs).
            // [CR#702.33c]: multikicker's "for each time it was kicked" —
            // the resolving entry's announced record carries the tag's
            // multiplicity ([CR#601.2b,607.2]; the record rides the STACK
            // entry — the post-resolution recheck is engine-alt-costs
            // follow-up work).
            Count::TimesPaid(tag) => self
                .stack
                .iter()
                .find(|e| e.id == frame.source)
                .and_then(|e| e.paid_costs.iter().find(|(t, _)| t == tag).map(|(_, n)| *n))
                .unwrap_or(0),
            Count::Damage(reference) => {
                let id = self.eval_reference(reference, frame);
                self.objects.obj(id).total_damage()
            }
            Count::ManaAvailable(reference) => self.floated_mana(reference, frame),
            // [CR#107.1]: fold the per-element projection over the set —
            // devotion = `Aggregate(SumOf, Project(<your permanents>,
            // CountOf(ManaSymbols(It, CountsAs(Green)))))` ([CR#700.5]); a
            // cross-player fold = `Aggregate(MaxOf, Project(<all players>,
            // PlayerStatOf(It, Life)))` ([CR#119.1] — "the highest life total
            // among all players", Arbiter of Knollridge). `Countable::Objects`
            // and `Countable::Players` are the two `Projectable` sources
            // (Idris's own gate); a `ManaSymbols`/`Singleton`/
            // `ManaSpentMatching` source fizzles to the empty set. Each
            // candidate — object OR player proxy, `candidates_with` doesn't
            // distinguish (players are objects too) — binds `It` in a cloned
            // sub-frame, exactly as `Selection::Pick` does. Unlike `Pick`'s
            // frameless `candidates` (it has no real card needing a
            // carrier-relative `of` yet), a devotion-shaped `of` ("permanents
            // YOU control") needs `Ref(You)`/`Ref(This)` anchored, so this
            // passes the frame's watcher, same as `CountOf`/`CountDistinct`
            // above. Empty folds are 0 (never-crash); `Min`/`Max`/`Average`
            // over ∅ are 0 by convention (`Iterator::min`/`max`'s `None`
            // mapped to 0 below — safe on any input, not just the ≥1-player
            // case every real fixture happens to hit).
            Count::Aggregate(op, proj) => {
                let ids = match &proj.of {
                    Countable::Objects(filter) | Countable::Players(filter) => {
                        let watcher = self.frame_watcher(frame);
                        crate::target::candidates_with(self, filter, Some(watcher))
                    }
                    // Not a `Projectable` source (Idris gates
                    // `Aggregate`/`Project` to `Objects`/`Players`) — fizzle
                    // to the empty set.
                    Countable::ManaSymbols(..)
                    | Countable::Singleton(..)
                    | Countable::ManaSpentMatching(..) => Vec::new(),
                };
                let values: Vec<Uint> = ids
                    .into_iter()
                    .map(|id| {
                        let sub = Frame {
                            anaphora: Anaphora {
                                it: Some(self.it_binding(id)),
                                allotment: None,
                                ..frame.anaphora.clone()
                            },
                            ..frame.clone()
                        };
                        self.eval_count(&proj.by, &sub)
                    })
                    .collect();
                match op {
                    deckmaste_core::AggregateOp::SumOf => {
                        values.iter().fold(0, |a, v| a.saturating_add(*v))
                    }
                    deckmaste_core::AggregateOp::MinOf => values.iter().copied().min().unwrap_or(0),
                    deckmaste_core::AggregateOp::MaxOf => values.iter().copied().max().unwrap_or(0),
                    deckmaste_core::AggregateOp::AverageOf(mode) => {
                        if values.is_empty() {
                            0
                        } else {
                            let sum: Uint = values.iter().fold(0, |a, v| a.saturating_add(*v));
                            let n = Uint::try_from(values.len()).expect("count fits Uint");
                            match mode {
                                deckmaste_core::RoundMode::RoundUp => sum.div_ceil(n),
                                deckmaste_core::RoundMode::RoundDown => sum / n,
                            }
                        }
                    }
                }
            }
            Count::Expanded(e) => self.eval_count(&e.value, frame),
        }
    }

    /// [CR#106.4]: the referenced player's total unspent (floated) mana — the
    /// count of units currently in their pool. Backs
    /// [`Count::ManaAvailable`](deckmaste_core::Count::ManaAvailable), the
    /// mana-available reader a data-driven strategy's ramp gate senses. A
    /// non-player reference fizzles to 0 (never-crash), like `Opponents`.
    fn floated_mana(&self, reference: &Reference, frame: &Frame) -> Uint {
        self.eval_player_ref(reference, frame).map_or(0, |p| {
            Uint::try_from(self.player(p).mana_pool.units().len()).expect("mana pool fits Uint")
        })
    }

    /// The distinct-value keys a single object contributes to a
    /// [`Count::CountDistinct`] union along `characteristic` ([CR#109.3]).
    /// Each axis yields zero or more stable string keys (a creature with two
    /// card types contributes both); the caller unions them across the matched
    /// set. Derived axes (power/toughness) read the layer view; the rest read
    /// the printed face — a v1 that matches how `is_permanent_spell` reads
    /// printed types.
    fn distinct_keys(
        &self,
        characteristic: deckmaste_core::Characteristic,
        id: ObjectId,
    ) -> Vec<String> {
        use deckmaste_core::Characteristic as Ch;
        let face = crate::derive::face(self.def(id));
        match characteristic {
            Ch::Types => face.types.iter().map(|t| format!("{t:?}")).collect(),
            Ch::Subtypes => face.subtypes.iter().map(|s| s.name.to_string()).collect(),
            // [CR#205.3i]: only the five basic land types contribute keys.
            Ch::BasicLandTypes => face
                .subtypes
                .iter()
                .map(|s| s.name.to_string())
                .filter(|n| deckmaste_core::BASIC_LAND_TYPES.contains(&n.as_str()))
                .collect(),
            Ch::Supertypes => face.supertypes.iter().map(|s| format!("{s:?}")).collect(),
            Ch::Name => vec![face.name.clone()],
            Ch::ManaCost => vec![format!("{}", face.mana_cost.mana_value())],
            Ch::Colors => {
                let mut colors: Vec<deckmaste_core::Color> = face.color_indicator.clone();
                for sym in face.mana_cost.iter() {
                    colors.extend(crate::layer::symbol_colors(sym));
                }
                colors.iter().map(|c| format!("{c:?}")).collect()
            }
            Ch::Power => self
                .layers()
                .power(id)
                .map(|p| vec![p.max(0).to_string()])
                .unwrap_or_default(),
            Ch::Toughness => self
                .layers()
                .toughness(id)
                .map(|t| vec![t.max(0).to_string()])
                .unwrap_or_default(),
            Ch::Defense => vec![
                self.objects
                    .obj(id)
                    .counters
                    .get("DefenseCounter")
                    .copied()
                    .unwrap_or(0)
                    .to_string(),
            ],
        }
    }

    /// The magnitude carried by an amount-bearing history fact — the
    /// event-amount channel `Count::EventSum` sums ([CR#608.2i]): damage and
    /// life amounts ([CR#120.1,119.3]), counter deltas ([CR#122.1]), one per
    /// draw fact ([CR#121.2] — drawn one at a time), and one per moved card
    /// on a cause-amount zone change (a discard's/mill's card count,
    /// [CR#701.9a,701.17a]). A fact kind outside this set trips loudly: the
    /// Idris re-emit gate rejects `EventSum` over an event that guarantees no
    /// amount, so no sound card reaches the fallback.
    fn game_event_amount(fact: &GameEvent) -> Uint {
        match fact {
            GameEvent::LifeLost { amount, .. }
            | GameEvent::LifeGained { amount, .. }
            | GameEvent::DamageDealt { amount, .. }
            | GameEvent::CounterPlaced { amount, .. }
            | GameEvent::CounterRemoved { amount, .. } => *amount,
            GameEvent::WillDraw { .. } | GameEvent::ZoneChanged { .. } => 1,
            other => unreachable!("EventSum reached a fact kind with no amount channel: {other:?}"),
        }
    }

    /// Lands `player` has played this turn ([CR#305.2,608.2i]) — backing the
    /// one-land-per-turn rule. A play is a `ZoneChanged` to the battlefield
    /// whose cause verb is `Play`, by `player`.
    ///
    /// # Panics
    ///
    /// Panics only if the count exceeds `Uint` — unreachable in a real game.
    #[must_use]
    pub fn lands_played_this_turn(&self, player: crate::player::PlayerId) -> Uint {
        use deckmaste_core::Zone;

        use crate::event::GameEvent;
        let play = deckmaste_core::Ident::from("Play");
        let n = self
            .history
            .scan(deckmaste_core::Lookback::ThisTurn, self.turn.turn_number)
            .filter(|f| {
                matches!(f,
                    GameEvent::ZoneChanged { to: Zone::Battlefield, cause: Some(c), snapshot, .. }
                        if c.verb == play && snapshot.controller == player)
            })
            .count();
        Uint::try_from(n).expect("land count fits Uint")
    }

    /// Count of `AbilityUsed` events for the given `(object, ability)` pair
    /// within `within` — the primitive backing per-instance use-limit gates
    /// ([CR#602.5b,603.2h]) and history reads ([CR#608.2i]).
    ///
    /// `within` must be a history-lookback window (`ThisTurn` or `ThisGame`);
    /// timing windows produce 0 (same defensive contract as
    /// [`History::scan`]).
    pub(crate) fn ability_used_count(
        &self,
        object: crate::object::ObjectId,
        ability: Uint,
        within: deckmaste_core::Lookback,
    ) -> Uint {
        let n = self
            .history
            .scan(within, self.turn.turn_number)
            .filter(|f| {
                matches!(f,
                    GameEvent::AbilityUsed { object: o, ability: a }
                        if *o == object && *a == ability)
            })
            .count();
        Uint::try_from(n).expect("ability use count fits Uint")
    }
}

/// The last-known counter map for a snapshot-bearing reference whose object is
/// gone ([CR#603.10a]): `This`/`EventObject`/`EventPatient` read the snapshot
/// the fired trigger carried; `It` reads the iteration/projection element's
/// snapshot. `None` when the relevant slot is unbound, the element is a player
/// (zoneless, no snapshot), or the reference isn't a snapshot-bearing one.
fn lki_counters<'f>(
    reference: &Reference,
    frame: &'f Frame,
) -> Option<&'f std::collections::HashMap<deckmaste_core::Ident, Uint>> {
    // `It` reads the iteration/projection element's snapshot (a card element);
    // a player element is zoneless and has none.
    if let Reference::It = reference {
        return match frame.anaphora.it.as_ref()? {
            crate::stack::ItBinding::Object(s) => Some(&s.counters),
            crate::stack::ItBinding::Player(_) => None,
        };
    }
    let snapshot = match reference {
        Reference::This => frame.this.as_ref(),
        Reference::EventObject => frame.anaphora.that_object.as_ref(),
        Reference::EventPatient => match frame.anaphora.that_patient.as_ref()? {
            crate::trigger::EventPatient::Object(s) => Some(s),
            crate::trigger::EventPatient::Player(_) => None,
        },
        _ => None,
    }?;
    Some(&snapshot.counters)
}
