//! `eval_count`: evaluate a [`Count`] — aggregates, history facts, devotion,
//! arithmetic — to a number.

use deckmaste_core::Count;
use deckmaste_core::Countable;
use deckmaste_core::Reference;
use deckmaste_core::Uint;

use crate::event::AbilityUsed;
use crate::event::CounterPlaced;
use crate::event::CounterRemoved;
use crate::event::DamageDealt;
use crate::event::GameEvent;
use crate::event::LifeGained;
use crate::event::LifeLost;
use crate::event::ZoneChange;
use crate::object::ObjectId;
use crate::stack::ExecutionFrame;
use crate::state::GameState;

impl GameState {
    /// Evaluate a `Count` to a concrete number.
    ///
    /// # Panics
    ///
    /// Panics on a `Count` not wired for Stage 3, on a `StatOf` whose object
    /// lacks the stat, and on a `ThatMuch` with no amount fixed in this
    /// resolution.
    /// A departed object's LAST KNOWN value for `stat` ([CR#608.2h,113.7a]).
    ///
    /// The snapshot keeps the object's card spine and its counters, so a
    /// printed power/toughness plus the counter adjustment is what remains
    /// knowable about it; continuous effects that were applying when it left
    /// are not recorded, which is the same degradation the snapshot already
    /// documents for its other fields.
    fn last_known_stat(
        &self,
        snapshot: &crate::lki::LkiSnapshot,
        stat: deckmaste_core::Stat,
    ) -> Uint {
        let crate::object::ObjectSource::Card(card) = snapshot.source else {
            return 0;
        };
        let face = crate::derive::face(&self.cards.get(card).def);
        let counter = |name: &str| {
            deckmaste_core::Int::try_from(snapshot.counters.get(name).copied().unwrap_or(0))
                .unwrap_or(deckmaste_core::Int::MAX)
        };
        let value = match stat {
            deckmaste_core::Stat::Power => {
                crate::layer::base_stat(face.characteristics.power.as_ref()).unwrap_or(0)
                    + counter("P1P1Counter")
                    - counter("M1M1Counter")
            }
            deckmaste_core::Stat::Toughness => {
                crate::layer::base_stat(face.characteristics.toughness.as_ref()).unwrap_or(0)
                    + counter("P1P1Counter")
                    - counter("M1M1Counter")
            }
            deckmaste_core::Stat::ManaValue => {
                deckmaste_core::Int::try_from(face.characteristics.mana_cost.mana_value())
                    .expect("mana value fits Int")
            }
            deckmaste_core::Stat::Loyalty => {
                crate::layer::base_stat(face.characteristics.loyalty.as_ref()).unwrap_or(0)
            }
            deckmaste_core::Stat::Defense => counter("DefenseCounter"),
        };
        Uint::try_from(value.max(0)).expect("clamped stat fits Uint")
    }

    #[expect(
        clippy::too_many_lines,
        reason = "one arm per Count kind — the value language's full surface"
    )]
    pub(crate) fn eval_count(&self, qty: &Count, frame: &ExecutionFrame) -> Uint {
        match qty {
            Count::Reg(register) => self
                .activation_number(frame.activation, *register)
                .unwrap_or(0),
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
                        .filter(|ob| {
                            crate::target::matches_region_with_activation(
                                self,
                                ob.id,
                                filter,
                                Some(watcher),
                                frame.activation,
                            )
                        })
                        .count();
                    Uint::try_from(n).expect("object count fits Uint")
                }
                // [CR#700.5]: devotion — count mana symbols in the
                // referenced object's printed cost matching `pred`. A
                // stale/absent/non-card-backed reference fizzles to 0
                // (never-crash on a semantic-input error).
                Countable::ManaSymbols(reference, pred) => {
                    let id = self.eval_reference(reference, frame);
                    let n = match self
                        .objects
                        .get(id)
                        .and_then(crate::object::GameObject::card_id)
                    {
                        Some(_) => crate::derive::face(self.def(id))
                            .characteristics
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
                let product = self.eval_reference_product(reference, frame);
                // [CR#608.2h]: current information while the object is still
                // the one the effect expected; LAST KNOWN information once it
                // has left that zone — "the sacrificed creature's power"
                // (Fling) and "the sacrificed creature's toughness" (Ayli)
                // both read a permanent that is already in a graveyard. A
                // cost's paid product may still be findable there
                // ([CR#400.7j]), but it is a different object ([CR#400.7]).
                let reminted = product
                    .lki
                    .as_ref()
                    .is_some_and(|snapshot| product.current != Some(snapshot.object));
                let live = (!reminted)
                    .then_some(product.current)
                    .flatten()
                    .filter(|&id| self.objects.get(id).is_some());
                let Some(id) = live else {
                    return product
                        .lki
                        .as_ref()
                        .map_or(0, |snapshot| self.last_known_stat(snapshot, *stat));
                };
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
                    // announce-slot X work.
                    deckmaste_core::Stat::ManaValue => {
                        let face = crate::derive::face(self.def(id));
                        deckmaste_core::Int::try_from(face.characteristics.mana_cost.mana_value())
                            .expect("mana value fits Int")
                    }
                    // [CR#209.1,306.5a]: `Stat::Loyalty` is the PRINTED loyalty
                    // characteristic off the card face — never the live counter
                    // count (current loyalty is `CounterCount(This,
                    // LoyaltyCounter)`). `Number(n)→n`, `DefinedByAbility`/
                    // `Variable`/absent → 0 (the chosen-X for a `Variable`
                    // loyalty rides the unbuilt announce-slot X work).
                    deckmaste_core::Stat::Loyalty => crate::layer::base_stat(
                        crate::derive::face(self.def(id))
                            .characteristics
                            .loyalty
                            .as_ref(),
                    )
                    .unwrap_or(0),
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
                let product = self.eval_reference_product(reference, frame);
                // [CR#608.2h]: current information while the object is still
                // the one the effect expected; LAST KNOWN information once it
                // has left that zone. A register holding a cost's paid product
                // resolves to the object's new incarnation in its new public
                // zone ([CR#400.7j] — the effect can still find it), but that
                // is a different object ([CR#400.7]), so "the sacrificed
                // creature's counters" is read off the snapshot.
                let reminted = product
                    .lki
                    .as_ref()
                    .is_some_and(|snapshot| product.current != Some(snapshot.object));
                let live = (!reminted)
                    .then_some(product.current)
                    .flatten()
                    .and_then(|id| self.objects.get(id));
                match live {
                    Some(o) => o.counters.get(kind.as_str()).copied().unwrap_or(0),
                    None => product
                        .lki
                        .as_ref()
                        .and_then(|snapshot| snapshot.counters.get(kind.as_str()).copied())
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
                        if crate::target::matches_region_with_activation(
                            self,
                            ob.id,
                            filter,
                            Some(watcher),
                            frame.activation,
                        ) {
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
            // firing event's magnitude seeded through the `EventAmount` parameter
            // by `resolve_object`. Still loud when neither fixed an amount:
            // that is a semantic-input error (a `ThatMuch` with no antecedent
            // magnitude), not an engine seam.
            // [CR#601.2d]: the per-element share in scope inside a `Distribute`
            // body — `Distribute` puts it in the `allotment` slot per element
            // (the Idris `bindAllot`), and an inner `Each`/`Distribute` clears
            // it (the Idris allotment-clearing `bindIt`), so reading it outside a
            // `Distribute` body — or inside a nested loop that rebound `It` — is
            // a malformed card.
            // [CR#107.3a]: while a spell/ability is on the stack, X equals the
            // value announced as it was cast (engine-x-costs threads it onto the
            // resolution frame). [CR#107.3f] text-X chosen at resolution is a
            // separate seam.
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
                    .in_window_for(
                        *within,
                        self.turn.turn_number,
                        self.turn.current,
                        self.turn.active_player,
                        frame.controller(self),
                    )
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
                    .in_window_for(
                        *within,
                        self.turn.turn_number,
                        self.turn.current,
                        self.turn.active_player,
                        frame.controller(self),
                    )
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
            // MISSING key (or a future non-number value) is a semantic-input
            // mistake, not a legal 0 — the engine-stat-none-fizzle ruling says
            // fizzle the consuming read, never a silent bare 0. `eval_count`
            // returns a bare `Uint` with no fizzle channel, so — exactly as the
            // `unbound_ref` no-op does for references — we leave a LOUD
            // `eprintln!` breadcrumb and degrade to 0 (no `debug_assert`: the
            // fizzle must never panic; a fizzling `Count` read is the principled
            // fix, tracked by engine-stat-none-fizzle).
            // [CR#120.3]: the damage marked on the referenced object — read
            // directly off the base state (damage is not a derived stat).
            // [CR#702.33c]: multikicker's "for each time it was kicked" — the
            // tag's multiplicity in THIS activation's announced record
            // ([CR#601.2b,607.2i]), the same channel `Condition::PaidCost`
            // reads, so it survives the stack -> battlefield remint with the
            // object ([CR#400.7d,702.33e]).
            Count::TimesPaid(tag) => self.activation_times_paid(frame.activation, tag),
            Count::Damage(reference) => {
                let id = self.eval_reference(reference, frame);
                self.objects.obj(id).total_damage()
            }
            Count::ManaAvailable(reference) => self.floated_mana(reference, frame),
            Count::ManaAvailableKind(reference, kind) => self
                .eval_player_ref(reference, frame)
                .map_or(0, |player| self.player(player).mana_pool.amount(*kind)),
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
            // distinguish (both are Entities) — binds `It` in a cloned
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
                        crate::target::candidates_region_with_activation(
                            self,
                            filter,
                            Some(watcher),
                            frame.activation,
                        )
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
                        let mut sub = frame.clone();
                        sub.activation = self.enter_candidate_region(&proj.by, frame, id);
                        self.eval_count(&proj.by.body, &sub)
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
            } /* Provenance is erased at `lower` (`deckmaste_lowering`), so no
               * loaded value reaches here wrapped. The arm survives only because
               * the variant does; `core-demacro` deletes both. */
        }
    }

    /// [CR#106.4]: the referenced player's total unspent (floated) mana — the
    /// count of units currently in their pool. Backs
    /// [`Count::ManaAvailable`](deckmaste_core::Count::ManaAvailable), the
    /// mana-available reader a data-driven strategy's ramp gate senses. A
    /// non-player reference fizzles to 0 (never-crash), like `Opponents`.
    fn floated_mana(&self, reference: &Reference, frame: &ExecutionFrame) -> Uint {
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
            Ch::Types => face
                .characteristics
                .types
                .iter()
                .map(|t| format!("{t:?}"))
                .collect(),
            Ch::Subtypes => face
                .characteristics
                .subtypes
                .iter()
                .map(|s| s.name.to_string())
                .collect(),
            // [CR#205.3i]: only the five basic land types contribute keys.
            Ch::BasicLandTypes => face
                .characteristics
                .subtypes
                .iter()
                .map(|s| s.name.to_string())
                .filter(|n| deckmaste_core::BASIC_LAND_TYPES.contains(&n.as_str()))
                .collect(),
            Ch::Supertypes => face
                .characteristics
                .supertypes
                .iter()
                .map(|s| format!("{s:?}"))
                .collect(),
            Ch::Name => vec![face.characteristics.name.to_string()],
            Ch::ManaCost => vec![format!("{}", face.characteristics.mana_cost.mana_value())],
            Ch::Colors => {
                let mut colors: Vec<deckmaste_core::Color> =
                    face.characteristics.color_indicator.clone();
                for sym in face.characteristics.mana_cost.iter() {
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
            GameEvent::LifeLost(LifeLost { amount, .. })
            | GameEvent::LifeGained(LifeGained { amount, .. })
            | GameEvent::DamageDealt(DamageDealt { amount, .. })
            | GameEvent::CounterPlaced(CounterPlaced { amount, .. })
            | GameEvent::CounterRemoved(CounterRemoved { amount, .. }) => *amount,
            GameEvent::ZoneChange(ZoneChange {
                snapshot: Some(_), ..
            }) => 1,
            other => unreachable!("EventSum reached a fact kind with no amount channel: {other:?}"),
        }
    }

    /// Lands `player` has played this turn ([CR#305.2,608.2i]) — backing the
    /// one-land-per-turn rule. A play is a past-form `ZoneChange` to the
    /// battlefield whose cause verb is `Play`, by `player`.
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
                    GameEvent::ZoneChange(ZoneChange {
                        snapshot: Some(snapshot),
                        to: Zone::Battlefield,
                        cause: Some(c),
                        ..
                    }) if c.verb == play && snapshot.controller == player)
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
                    GameEvent::AbilityUsed(AbilityUsed { object: o, ability: a })
                        if *o == object && *a == ability)
            })
            .count();
        Uint::try_from(n).expect("ability use count fits Uint")
    }
}

#[cfg(test)]
mod tests {
    #![allow(
        clippy::empty_line_after_doc_comments,
        reason = "related behavioral test rationale is intentionally grouped"
    )]

    use std::sync::Arc;

    use deckmaste_card::Card;
    use deckmaste_card::CardFace;
    use deckmaste_card::Characteristics;
    use deckmaste_core::Action;
    use deckmaste_core::Count;
    use deckmaste_core::Countable;
    use deckmaste_core::Instruction;
    use deckmaste_core::Lookback;
    use deckmaste_core::Predicate;
    use deckmaste_core::Reference;
    use deckmaste_core::StatePredicate;
    use deckmaste_core::Type;
    use deckmaste_core::Zone;

    use crate::event::AbilityUsed;
    use crate::event::GameEvent;
    use crate::event::LifeGained;
    use crate::event::LifeLost;
    use crate::event::ZoneChange;
    use crate::object::ObjectId;
    use crate::object::ObjectSource;
    use crate::player::PlayerId;
    use crate::resolve::fixtures::*;
    use crate::state::GameConfig;
    use crate::state::GameState;
    use crate::state::PlayerConfig;
    use crate::state::StartingPlayer;
    use crate::step::StepOutcome;
    use crate::test_support::frame_for;
    use crate::test_support::frame_src;
    use crate::test_support::frame_src_targets;

    #[test]
    fn mana_available_kind_counts_only_the_requested_kind() {
        use deckmaste_core::Color;
        use deckmaste_core::ColorOrColorless;

        use crate::player::ManaProvenance;

        let mut state = game();
        let player = PlayerId(0);
        state
            .player_mut(player)
            .mana_pool
            .add(Color::Green.into(), 3, ManaProvenance::default());
        state
            .player_mut(player)
            .mana_pool
            .add(Color::Black.into(), 1, ManaProvenance::default());
        let frame = frame_for(&state, player);

        assert_eq!(
            state.eval_count(
                &Count::ManaAvailableKind(
                    Reference::Reg(deckmaste_core::RefId(1)),
                    Color::Green.into()
                ),
                &frame,
            ),
            3,
        );
        assert_eq!(
            state.eval_count(
                &Count::ManaAvailableKind(
                    Reference::Reg(deckmaste_core::RefId(1)),
                    ColorOrColorless::Colorless,
                ),
                &frame,
            ),
            0,
        );
    }

    /// History tallies via `EventCount`/`EventSum` — the general primitives
    /// that subsume the old `Count::Query`/`eval_query` scalar family
    /// ([CR#608.2i]). Fixtures are the same events; assertions use the
    /// replacements.
    #[expect(
        clippy::too_many_lines,
        reason = "one fixture exercises all five history tallies (storm/draws/lands/life-lost/life-gained) end-to-end"
    )]
    #[test]
    fn history_tallies_via_event_count_sum() {
        use deckmaste_core::Agency;
        use deckmaste_core::Count;
        use deckmaste_core::EventFilter;
        use deckmaste_core::Predicate;
        use deckmaste_core::Reference;

        use crate::event::Cause;
        use crate::lki::LkiSnapshot;

        let mut state = game();
        state.turn.turn_number = 1;
        let p = PlayerId(0);
        // A frame anchored on player p — Ref(You) resolves to p's proxy.
        let frame = frame_for(&state, p);

        // Three spells cast this turn (game-wide); `EventCount(Cast, ThisTurn)`
        // returns the FULL count (3) — the plain tally, not the storm count.
        // Storm's "each OTHER spell cast before it" is the separate
        // `EventCount(AllOf[Cast, Before(This)], ThisTurn)` refinement
        // ([CR#702.40a]; see `storm_counts_other_spells_cast_before_it`
        // below), never a blanket −1 off this tally.
        // Mint real objects for the Cast path (performed_matches reads their
        // controllers via objects.obj, which panics on stale IDs).
        let sp1 = state.objects.mint(ObjectSource::Player(p), p, None);
        let sp2 = state.objects.mint(ObjectSource::Player(p), p, None);
        let sp3 = state.objects.mint(ObjectSource::Player(p), p, None);
        state.record_history_fact(1, None, GameEvent::SpellCast(sp1));
        state.record_history_fact(1, None, GameEvent::SpellCast(sp2));
        state.record_history_fact(1, None, GameEvent::SpellCast(sp3));
        let cast_event = EventFilter::Cast {
            who: Predicate::Any,
            what: Predicate::Any,
        };
        assert_eq!(
            state.eval_count(
                &Count::EventCount(Arc::new(cast_event), Lookback::ThisTurn),
                &frame
            ),
            3,
            "the plain cast tally this turn is the full count (storm's before-it \
             refinement is a separate count)"
        );

        // Two draws by p this turn → EventCount(Drawn, by: Ref(You)) = 2. A
        // draw's SUCCESS fact is the committed Library → Hand move tagged
        // `cause: Draw` ([CR#121.2] — one fact per card), which projects to
        // `FactKind::Drawn`.
        for _ in 0..2 {
            let drawn = state
                .objects
                .mint(ObjectSource::Player(p), p, Some(Zone::Hand));
            state.record_history_fact(
                1,
                None,
                GameEvent::ZoneChange(ZoneChange {
                    object: drawn,
                    snapshot: Some(Box::new(LkiSnapshot::capture(&state, drawn))),
                    from: Some(Zone::Library),
                    to: Zone::Hand,
                    enters: None,
                    position: None,
                    face: None,
                    cause: Some(Cause {
                        verb: "Draw".into(),
                        agency: Agency::EffectInstruction,
                        agent: None,
                        payment: None,
                    }),
                }),
            );
        }
        let draw_event = EventFilter::Drawn {
            who: Predicate::Ref(Reference::Reg(deckmaste_core::RefId(1))),
            amount: None,
        };
        assert_eq!(
            state.eval_count(
                &Count::EventCount(Arc::new(draw_event), Lookback::ThisTurn),
                &frame
            ),
            2,
            "draws by p this turn"
        );

        // One land played by p (a Play-caused battlefield entry).
        // `lands_played_this_turn` is the direct helper; EventCount(Play,
        // by: Ref(You)) is the generic equivalent.
        let land = state
            .objects
            .mint(ObjectSource::Player(p), p, Some(Zone::Battlefield));
        state.record_history_fact(
            1,
            None,
            GameEvent::ZoneChange(ZoneChange {
                object: land,
                snapshot: Some(Box::new(LkiSnapshot::capture(&state, land))),
                from: Some(Zone::Hand),
                to: Zone::Battlefield,
                enters: None,
                position: None,
                face: None,
                cause: Some(Cause {
                    verb: "Play".into(),
                    agency: Agency::SpecialAction,
                    agent: None,
                    payment: None,
                }),
            }),
        );
        assert_eq!(
            state.lands_played_this_turn(p),
            1,
            "lands played by p this turn (direct helper)"
        );
        let play_event = EventFilter::Played {
            who: Predicate::Ref(Reference::Reg(deckmaste_core::RefId(1))),
            what: Predicate::Any,
        };
        assert_eq!(
            state.eval_count(
                &Count::EventCount(Arc::new(play_event), Lookback::ThisTurn),
                &frame
            ),
            1,
            "lands played by p via EventCount"
        );

        // Life: lost 3 then 2 (=5), gained 4.
        // EventSum(LoseLife, by: Ref(You)) sums the amounts; EventSum(GainLife)
        // likewise ([CR#119.3]).
        state.record_history_fact(
            1,
            None,
            GameEvent::LifeLost(LifeLost {
                player: p,
                amount: 3,
                cause: None,
            }),
        );
        state.record_history_fact(
            1,
            None,
            GameEvent::LifeLost(LifeLost {
                player: p,
                amount: 2,
                cause: None,
            }),
        );
        state.record_history_fact(
            1,
            None,
            GameEvent::LifeGained(LifeGained {
                player: p,
                amount: 4,
                cause: None,
            }),
        );
        let lose_event = EventFilter::LifeLost {
            who: Predicate::Ref(Reference::Reg(deckmaste_core::RefId(1))),
            amount: None,
        };
        let gain_event = EventFilter::LifeGained {
            who: Predicate::Ref(Reference::Reg(deckmaste_core::RefId(1))),
            amount: None,
        };
        assert_eq!(
            state.eval_count(
                &Count::EventSum(Arc::new(lose_event), Lookback::ThisTurn),
                &frame
            ),
            5,
            "life lost by p this turn"
        );
        assert_eq!(
            state.eval_count(
                &Count::EventSum(Arc::new(gain_event), Lookback::ThisTurn),
                &frame
            ),
            4,
            "life gained by p this turn"
        );

        // Prior-turn entries are excluded once the turn advances.
        state.turn.turn_number = 2;
        let cast_event2 = EventFilter::Cast {
            who: Predicate::Any,
            what: Predicate::Any,
        };
        let draw_event2 = EventFilter::Drawn {
            who: Predicate::Ref(Reference::Reg(deckmaste_core::RefId(1))),
            amount: None,
        };
        assert_eq!(
            state.eval_count(
                &Count::EventCount(Arc::new(cast_event2), Lookback::ThisTurn),
                &frame
            ),
            0,
            "storm resets on new turn"
        );
        assert_eq!(
            state.eval_count(
                &Count::EventCount(Arc::new(draw_event2), Lookback::ThisTurn),
                &frame
            ),
            0,
            "draws reset on new turn"
        );
    }

    /// The storm count ([CR#702.40a]): "copy it for each OTHER spell that was
    /// cast BEFORE it this turn." Cast A, then B, then the storm spell S; the
    /// storm count evaluated in S's frame is exactly 2 (A and B) — not 3 (S's
    /// own cast is not "before" itself, dropped by `Before(This)`) and not 1.
    /// This is the count `Repeat(<count>, CopySpell(This))` drives, so 2 =
    /// two copies.
    #[test]
    fn storm_counts_other_spells_cast_before_it() {
        use deckmaste_core::EventFilter;

        let mut state = game();
        state.turn.turn_number = 1;
        let p = PlayerId(0);

        // A, B, then the storm spell S — cast in that order this turn. Real
        // objects (the Cast path reads controllers via objects.obj).
        let sp_a = state.objects.mint(ObjectSource::Player(p), p, None);
        let sp_b = state.objects.mint(ObjectSource::Player(p), p, None);
        let storm = state.objects.mint(ObjectSource::Player(p), p, None);
        state.record_history_fact(1, None, GameEvent::SpellCast(sp_a));
        state.record_history_fact(1, None, GameEvent::SpellCast(sp_b));
        state.record_history_fact(1, None, GameEvent::SpellCast(storm));

        // The storm count, evaluated in the storm spell's frame (This = S).
        let storm_count = Count::EventCount(
            Arc::new(EventFilter::AllOf(
                vec![
                    EventFilter::Cast {
                        who: Predicate::Any,
                        what: Predicate::Any,
                    },
                    EventFilter::Before(Reference::Reg(deckmaste_core::RefId(0))),
                ]
                .into(),
            )),
            Lookback::ThisTurn,
        );
        let frame = frame_src(&state, storm);
        assert_eq!(
            state.eval_count(&storm_count, &frame),
            2,
            "storm copies for A and B — the two OTHER spells cast before S \
             ([CR#702.40a]); S's own cast is excluded, so not 3, and both \
             predecessors count, so not 1"
        );
    }

    /// The held-priority case ([CR#603.3]): after casting the storm spell S,
    /// a player holds priority and casts a fourth spell T before the storm
    /// trigger resolves. T is in this turn's cast tally, and the trigger is
    /// placed ABOVE T on the stack — yet T was cast AFTER S, so it is not
    /// "cast before it" and the copy count stays 2. `Before(This)` keys on
    /// cast ORDER (log position), not resolution time, so T (a later cast) is
    /// excluded.
    #[test]
    fn storm_count_ignores_spells_cast_in_response() {
        use deckmaste_core::EventFilter;

        let mut state = game();
        state.turn.turn_number = 1;
        let p = PlayerId(0);

        let sp_a = state.objects.mint(ObjectSource::Player(p), p, None);
        let sp_b = state.objects.mint(ObjectSource::Player(p), p, None);
        let storm = state.objects.mint(ObjectSource::Player(p), p, None);
        state.record_history_fact(1, None, GameEvent::SpellCast(sp_a));
        state.record_history_fact(1, None, GameEvent::SpellCast(sp_b));
        state.record_history_fact(1, None, GameEvent::SpellCast(storm));

        let storm_count = Count::EventCount(
            Arc::new(EventFilter::AllOf(
                vec![
                    EventFilter::Cast {
                        who: Predicate::Any,
                        what: Predicate::Any,
                    },
                    EventFilter::Before(Reference::Reg(deckmaste_core::RefId(0))),
                ]
                .into(),
            )),
            Lookback::ThisTurn,
        );
        let frame = frame_src(&state, storm);
        assert_eq!(
            state.eval_count(&storm_count, &frame),
            2,
            "baseline before the response: A and B"
        );

        // T cast in response, AFTER S (a later log position).
        let sp_t = state.objects.mint(ObjectSource::Player(p), p, None);
        state.record_history_fact(1, None, GameEvent::SpellCast(sp_t));
        assert_eq!(
            state.eval_count(&storm_count, &frame),
            2,
            "T was cast after S — not 'before it' ([CR#603.3]) — so the copy \
             count stays 2, never 3"
        );
    }

    /// `ability_used_count` counts `AbilityUsed` events keyed by (object,
    /// ability) and respects the `Lookback` filter — `ThisTurn` excludes
    /// prior-turn entries, `ThisGame` includes them all.
    #[test]
    fn ability_used_count_keys_object_ability_window() {
        let mut state = game();
        state.turn.turn_number = 1;

        let obj_a = ObjectId::from_raw(10);
        let obj_b = ObjectId::from_raw(20);

        // Two uses of ability 0 on obj_a, one use of ability 1 on obj_a —
        // all on the current turn.
        state.record_history_fact(
            1,
            None,
            GameEvent::AbilityUsed(AbilityUsed {
                object: obj_a,
                ability: 0,
            }),
        );
        state.record_history_fact(
            1,
            None,
            GameEvent::AbilityUsed(AbilityUsed {
                object: obj_a,
                ability: 0,
            }),
        );
        state.record_history_fact(
            1,
            None,
            GameEvent::AbilityUsed(AbilityUsed {
                object: obj_a,
                ability: 1,
            }),
        );

        assert_eq!(state.ability_used_count(obj_a, 0, Lookback::ThisGame), 2);
        assert_eq!(state.ability_used_count(obj_a, 1, Lookback::ThisGame), 1);
        // obj_b has no uses recorded.
        assert_eq!(state.ability_used_count(obj_b, 0, Lookback::ThisGame), 0);

        // A use of (obj_a, 0) on a DIFFERENT turn.
        state.record_history_fact(
            2,
            None,
            GameEvent::AbilityUsed(AbilityUsed {
                object: obj_a,
                ability: 0,
            }),
        );

        // ThisTurn (still turn 1) excludes the turn-2 entry.
        assert_eq!(state.ability_used_count(obj_a, 0, Lookback::ThisTurn), 2);
        // ThisGame includes it.
        assert_eq!(state.ability_used_count(obj_a, 0, Lookback::ThisGame), 3);
    }

    /// `CountOf` is the filter's live cardinality; a `ControlledBy(Ref(You))`
    /// relation anchors to the frame's side via the watcher.
    #[test]
    fn count_of_counts_live_matching_objects() {
        let (mut state, bear) = bear_on_field();
        // A second bear onto the battlefield, then handed to player 1.
        let _ = second_bear_to_player_1(&mut state);

        let frame = frame_src(&state, bear);
        let creatures = Predicate::And(
            vec![
                Predicate::State(StatePredicate::InZone(Zone::Battlefield)),
                Predicate::creature(),
            ]
            .into(),
        );
        assert_eq!(
            state.eval_count(
                &Count::CountOf(Countable::Objects(Arc::new(
                    deckmaste_core::Region::candidate(creatures.clone()),
                ))),
                &frame
            ),
            2
        );

        // "Creatures you control": only the frame side's bear.
        let yours = Predicate::And(
            vec![
                creatures,
                Predicate::Relation(deckmaste_core::RelationPredicate::ControlledBy(Arc::new(
                    Predicate::Ref(Reference::Reg(deckmaste_core::RefId(1))),
                ))),
            ]
            .into(),
        );
        assert_eq!(
            state.eval_count(
                &Count::CountOf(Countable::Objects(Arc::new(
                    deckmaste_core::Region::candidate(yours),
                ))),
                &frame,
            ),
            1
        );
    }

    /// Mint a battlefield permanent with the given printed mana cost, with no
    /// other characteristics — the fixture the pip-count (devotion,
    /// [CR#700.5]) tests below drive.
    fn permanent_with_cost(state: &mut GameState, mana_cost: &str) -> ObjectId {
        let card = Card::Normal(CardFace::from(Characteristics {
            name: "Test Permanent".into(),
            mana_cost: mana_cost.parse().unwrap(),
            types: vec![Type::Artifact.def()],
            ..Characteristics::default()
        }));
        let cid = state.cards.push(Arc::new(card), PlayerId(0));
        let obj = state.objects.mint(
            ObjectSource::Card(cid),
            PlayerId(0),
            Some(Zone::Battlefield),
        );
        state.zones.battlefield.push(obj);
        obj
    }

    /// `CountOf(ManaSymbols(..))` ([CR#700.5] devotion) counts matching pips
    /// in the referenced object's printed cost: a plain colored count, a
    /// hybrid pip counting toward EACH of its colors, and an `Or` disjunction
    /// counting either.
    #[test]
    fn count_of_mana_symbols_counts_devotion_pips() {
        use deckmaste_core::SymbolPred;

        let mut state = game();
        let gg1 = permanent_with_cost(&mut state, "{G}{G}{1}");
        let frame = frame_src(&state, gg1);
        assert_eq!(
            state.eval_count(
                &Count::CountOf(Countable::ManaSymbols(
                    Arc::new(Reference::Reg(deckmaste_core::RefId(0))),
                    SymbolPred::CountsAs(deckmaste_core::Color::Green),
                )),
                &frame,
            ),
            2,
            "{{G}}{{G}}{{1}} has two green pips"
        );

        let gwgw = permanent_with_cost(&mut state, "{G/W}{G/W}");
        let frame = frame_src(&state, gwgw);
        assert_eq!(
            state.eval_count(
                &Count::CountOf(Countable::ManaSymbols(
                    Arc::new(Reference::Reg(deckmaste_core::RefId(0))),
                    SymbolPred::CountsAs(deckmaste_core::Color::Green),
                )),
                &frame,
            ),
            2,
            "each {{G/W}} hybrid pip counts toward green devotion"
        );
        assert_eq!(
            state.eval_count(
                &Count::CountOf(Countable::ManaSymbols(
                    Arc::new(Reference::Reg(deckmaste_core::RefId(0))),
                    SymbolPred::CountsAs(deckmaste_core::Color::White),
                )),
                &frame,
            ),
            2,
            "…and toward white devotion too"
        );
        assert_eq!(
            state.eval_count(
                &Count::CountOf(Countable::ManaSymbols(
                    Arc::new(Reference::Reg(deckmaste_core::RefId(0))),
                    SymbolPred::Or(
                        vec![
                            SymbolPred::CountsAs(deckmaste_core::Color::White),
                            SymbolPred::CountsAs(deckmaste_core::Color::Black),
                        ]
                        .into()
                    ),
                )),
                &frame,
            ),
            2,
            "Or([White, Black]) over {{G/W}}{{G/W}} matches on the White half of each pip"
        );
    }

    /// Mint a battlefield creature with an explicit power/toughness — the
    /// fixture the aggregate-fold (`SumOf` over `StatOf`) test drives.
    #[allow(
        dead_code,
        reason = "shared fixture retained for neighboring count cases"
    )]
    fn creature_with_power(state: &mut GameState, power: deckmaste_core::Int) -> ObjectId {
        let card = Card::Normal(CardFace::from(Characteristics {
            name: "Test Creature".into(),
            types: vec![Type::Creature.def()],
            power: Some(deckmaste_core::StatValue::Number(power)),
            toughness: Some(deckmaste_core::StatValue::Number(power)),
            ..Characteristics::default()
        }));
        let cid = state.cards.push(Arc::new(card), PlayerId(0));
        let obj = state.objects.mint(
            ObjectSource::Card(cid),
            PlayerId(0),
            Some(Zone::Battlefield),
        );
        state.zones.battlefield.push(obj);
        obj
    }

    /// `Count::Aggregate(op, Projection)` folds a per-element `Count` over the
    /// projected set ([CR#107.1]), mirroring `Selection::Pick`'s per-candidate
    /// `It` binding. Devotion decomposes to `Aggregate(SumOf, Project(<your
    /// permanents>, CountOf(ManaSymbols(It, CountsAs(Green)))))`
    /// ([CR#700.5]); `SumOf` over `StatOf(It, Power)` totals power; every
    /// `AggregateOp` folds the empty set to 0 (never-crash).

    /// The cross-player fold ([CR#119.1] Arbiter of Knollridge): `Aggregate`
    /// over a `Countable::Players` source reads each matching player's
    /// `PlayerStatOf(It, Life)` and folds per `AggregateOp` — the
    /// player-sourced twin of `aggregate_folds_a_projection_over_a_selection`'s
    /// object-sourced coverage above. `MaxOf` reads the higher of the two
    /// players' life totals ("the highest life total among all players");
    /// every `AggregateOp` folds an EMPTY player set to 0 (never-crash) —
    /// exercised on a real `Countable::Players` source, not just the object
    /// analog, since `Iterator::min`/`max`'s `None` case is the concrete
    /// panic risk (`.unwrap()` on an empty iterator) the never-crash ruling
    /// guards against.

    /// The player-scope stat predicate ([CR#119.1]): `CountOf(Players(
    /// PlayerStatCmp(Life, AtMost, N)))` counts players whose life total is
    /// at most `N` — "the number of players with 13 or less life". Threshold
    /// 13 catches only the 10-life player; threshold 5 catches neither;
    /// threshold 20 catches both. Also exercises the predicate's
    /// player-proxy-only match: the fixture mints a permanent (`src`) too, so
    /// a wrongly-matching non-player object would inflate the count.
    #[test]
    fn players_countable_counts_by_life_threshold() {
        use deckmaste_core::Cmp;
        use deckmaste_core::PlayerAttr;

        let mut state = game();
        let src = permanent_with_cost(&mut state, "{1}");
        let frame = frame_src(&state, src);
        state.player_mut(PlayerId(0)).life = 20;
        state.player_mut(PlayerId(1)).life = 10;

        let count_at_most = |threshold| {
            Count::CountOf(Countable::Players(Arc::new(
                deckmaste_core::Region::candidate(Predicate::PlayerStatCmp(
                    PlayerAttr::Life,
                    Cmp::AtMost,
                    Count::Literal(threshold),
                )),
            )))
        };

        assert_eq!(
            state.eval_count(&count_at_most(13), &frame),
            1,
            "only the 10-life player has 13 or less life"
        );
        assert_eq!(
            state.eval_count(&count_at_most(5), &frame),
            0,
            "neither player has 5 or less life"
        );
        assert_eq!(
            state.eval_count(&count_at_most(20), &frame),
            2,
            "both players have at most 20 life"
        );
    }

    /// Devotion end-to-end ([CR#700.5]), the two cases
    /// `aggregate_folds_a_projection_over_a_selection` doesn't already cover:
    /// a two-color disjunction (`Or([White, Black])`) summed across SEPARATE
    /// permanents (not just one object's multiple pips), and an actually
    /// empty battlefield (no permanents minted at all, not a `Not(Any)`
    /// filter trick).

    /// `StatOf` reads the DERIVED stat (a pump shows through) and the
    /// printed mana value ([CR#202.3]).
    #[test]
    fn stat_of_reads_derived_stats() {
        let (mut state, bear) = bear_on_field();
        let frame = frame_src_targets(&state, bear, vec![bear]);

        let power = Count::StatOf(
            Reference::Reg(deckmaste_core::RefId(6)),
            deckmaste_core::Stat::Power,
        );
        assert_eq!(state.eval_count(&power, &frame), 2);
        assert_eq!(
            state.eval_count(
                &Count::StatOf(
                    Reference::Reg(deckmaste_core::RefId(0)),
                    deckmaste_core::Stat::ManaValue
                ),
                &frame
            ),
            2,
            "Grizzly Bears costs {{1}}{{G}}"
        );

        // A +1/+0 continuous effect shows the read rides the layer view.
        let timestamp = state.objects.next_timestamp();
        state.continuous.push(crate::layer::ContinuousEffect {
            timestamp,
            controller: PlayerId(0),
            scope: crate::layer::ScopeResolved::Locked(vec![bear]),
            changes: vec![deckmaste_core::Modification::Power(
                deckmaste_core::NumericOp::Up(Count::Literal(1)),
            )],
            duration: deckmaste_core::Duration::EndOfGame,
            rows: vec![],
            origin: None,
            is_cda: false,
        });
        assert_eq!(state.eval_count(&power, &frame), 3);
    }

    /// "That much" reads the amount the damage instruction fixed: the two
    /// instructions run through the agenda, the `DamageDealt` apply records
    /// 3, and the later `GainLife(ThatMuch)` evaluation reads it back.

    #[test]
    fn count_x_reads_announced_value() {
        let (state, src) = bear_on_field();
        let mut frame = state.frame(src, PlayerId(0));
        let region = deckmaste_core::Region::new(
            [
                deckmaste_core::source_controller_params().as_ref(),
                &[deckmaste_core::Param {
                    def: deckmaste_core::DefId(2),
                    kind: deckmaste_core::Kind::Number,
                    provenance: deckmaste_core::Provenance::AnnouncedX,
                }],
            ]
            .concat()
            .into(),
            (),
        );
        frame.activation = state.enter_region(&region, &frame);
        state.frame_set_x(&mut frame, Some(3));
        assert_eq!(
            state.eval_count(&Count::Reg(deckmaste_core::RefId(2)), &frame),
            3
        );
    }

    /// An unavailable declared number parameter fizzles to 0
    /// (engine-stat-none-fizzle), never a panic or stale value.
    #[test]
    fn unavailable_declared_number_parameter_fizzles_to_zero() {
        let (state, a) = bear_on_field();
        let frame = frame_src(&state, a);
        assert_eq!(
            state.eval_count(&Count::Reg(deckmaste_core::RefId(6)), &frame),
            0
        );
    }

    /// [CR#122.1]: `Count::CounterCount(ref, kind)` reads how many `kind`
    /// counters sit on the resolved object/player proxy; an absent kind is 0.
    /// Counter kinds are rusty idents (`P1P1Counter`), not symbolic strings.
    #[test]
    fn counter_count_reads_the_objects_counter_map() {
        let (mut state, bear) = bear_on_field();
        state
            .objects
            .obj_mut(bear)
            .counters
            .insert("P1P1Counter".into(), 3);
        let frame = frame_src(&state, bear);
        assert_eq!(
            state.eval_count(
                &Count::CounterCount(
                    Arc::new(Reference::Reg(deckmaste_core::RefId(0))),
                    "P1P1Counter".into()
                ),
                &frame
            ),
            3
        );
        assert_eq!(
            state.eval_count(
                &Count::CounterCount(
                    Arc::new(Reference::Reg(deckmaste_core::RefId(0))),
                    "M1M1Counter".into()
                ),
                &frame
            ),
            0,
            "an absent counter kind reads as zero"
        );
    }

    /// A register a payment or instruction has not written yet holds no
    /// product, and every value read over it DEGRADES TO NULL rather than
    /// panicking (the fizzle law of
    /// `docs/decisions/invalid-semantic-input-fizzles.md`). `StatOf` is the
    /// read a cost's paid product travels through ("the sacrificed creature's
    /// power", [CR#118.8]), so it must never assert liveness.
    #[test]
    fn a_value_read_over_an_unwritten_register_degrades_to_null_not_a_panic() {
        let (state, bear) = bear_on_field();
        let frame = frame_src(&state, bear);
        // Register 7 is past `frame_src`'s whole parameter prefix: nothing has
        // ever written it.
        let unwritten = Reference::Reg(deckmaste_core::RefId(7));
        for stat in [
            deckmaste_core::Stat::Power,
            deckmaste_core::Stat::Toughness,
            deckmaste_core::Stat::ManaValue,
            deckmaste_core::Stat::Loyalty,
            deckmaste_core::Stat::Defense,
        ] {
            assert_eq!(
                state.eval_count(&Count::StatOf(unwritten.clone(), stat), &frame),
                0,
                "{stat:?} over an unwritten register reads zero"
            );
        }
        assert_eq!(
            state.eval_count(
                &Count::CounterCount(Arc::new(unwritten), "P1P1Counter".into()),
                &frame
            ),
            0,
            "a counter read over an unwritten register reads zero"
        );
    }

    /// [CR#603.10a,702.43a]: when the object a `CounterCount(This, _)` names is
    /// GONE (a dies trigger — Modular's "for each +1/+1 counter on this
    /// permanent" resolves after the creature left the battlefield), the count
    /// comes from the trigger's last-known snapshot, not the stale id. Without
    /// the LKI bridge `eval_count` would panic dereferencing the dead object.
    #[test]
    fn counter_count_reads_lki_when_the_object_is_gone() {
        let (mut state, bear) = bear_on_field();
        state
            .objects
            .obj_mut(bear)
            .counters
            .insert("P1P1Counter".into(), 2);
        // Snapshot the creature, then remove it — `bear` is now a stale id, the
        // exact state a dies trigger's `This` resolves to ([CR#603.10a]).
        let snapshot = crate::lki::LkiSnapshot::capture(&state, bear);
        state.objects.remove(bear);
        assert!(state.objects.get(bear).is_none(), "the object is gone");

        let mut frame = state.frame(bear, PlayerId(0));
        state.frame_set_source_lki(&mut frame, Some(snapshot));

        assert_eq!(
            state.eval_count(
                &Count::CounterCount(
                    Arc::new(Reference::Reg(deckmaste_core::RefId(0))),
                    "P1P1Counter".into()
                ),
                &frame
            ),
            2,
            "the dying creature's last-known +1/+1 counter count"
        );
        assert_eq!(
            state.eval_count(
                &Count::CounterCount(
                    Arc::new(Reference::Reg(deckmaste_core::RefId(0))),
                    "M1M1Counter".into()
                ),
                &frame
            ),
            0,
            "an absent kind on the snapshot reads as zero"
        );
    }

    /// [CR#209.1,306.5a]: `StatOf(_, Loyalty)` reads the PRINTED loyalty
    /// characteristic off the card face, NOT the live loyalty-counter count.
    /// A planeswalker printed at loyalty 4 carrying a single loyalty counter
    /// reads 4 (printed), never 1 (counters). Current on-battlefield loyalty is
    /// the separate `CounterCount(This, LoyaltyCounter)` read exercised below.
    #[test]
    fn stat_of_loyalty_reads_printed_loyalty() {
        use deckmaste_core::Stat;

        let (mut state, _bear) = bear_on_field();
        let card = Card::Normal(CardFace::from(Characteristics {
            name: "Test Walker".into(),
            types: vec![Type::Planeswalker.def()],
            loyalty: Some(deckmaste_core::StatValue::Number(4)),
            ..Characteristics::default()
        }));
        let cid = state.cards.push(Arc::new(card), PlayerId(0));
        let walker = state.objects.mint(
            ObjectSource::Card(cid),
            PlayerId(0),
            Some(Zone::Battlefield),
        );
        state.zones.battlefield.push(walker);
        // A single loyalty counter — the printed read must IGNORE it.
        state
            .objects
            .obj_mut(walker)
            .counters
            .insert("LoyaltyCounter".into(), 1);
        let frame = frame_src(&state, walker);
        assert_eq!(
            state.eval_count(
                &Count::StatOf(Reference::Reg(deckmaste_core::RefId(0)), Stat::Loyalty),
                &frame
            ),
            4,
            "printed loyalty (4), not the loyalty-counter count (1)"
        );
    }

    /// [CR#306.5c,122.1e]: CURRENT on-battlefield loyalty IS the loyalty-counter
    /// count, spelled `CounterCount(This, LoyaltyCounter)` — the companion to
    /// the printed `StatOf(_, Loyalty)` read above, now that
    /// `Stat::Loyalty` no longer means the counter count.
    #[test]
    fn counter_count_reads_current_loyalty() {
        let (mut state, bear) = bear_on_field();
        state
            .objects
            .obj_mut(bear)
            .counters
            .insert("LoyaltyCounter".into(), 4);
        let frame = frame_src(&state, bear);
        assert_eq!(
            state.eval_count(
                &Count::CounterCount(
                    Arc::new(Reference::Reg(deckmaste_core::RefId(0))),
                    "LoyaltyCounter".into()
                ),
                &frame
            ),
            4,
            "current loyalty = the live loyalty-counter count"
        );
    }

    /// [CR#109.2]: an activated ability that counts "Goblins you control" — a
    /// subtype description with no zone qualifier — means Goblin PERMANENTS on
    /// the battlefield. The canonical (`Permanent`-scoped) filter counts
    /// exactly the battlefield Goblins; the bare-subtype filter (no zone
    /// scope) ALSO matches the ability's own freshly-minted on-stack
    /// identity — which reuses the source's card id — so it over-counts by
    /// one. With three controlled Goblins (incl. the source) Krenko makes 3
    /// tokens, not 4. This pins the engine semantics the parser fix relies
    /// on (see `parsers::filter::head_noun`'s `Permanent` scope).
    #[test]
    fn count_you_control_excludes_the_activations_own_stack_copy() {
        // A Goblin permanent on the battlefield, player 0.
        fn goblin(state: &mut GameState, name: &str) -> ObjectId {
            mint_on_field(
                state,
                Card::Normal(CardFace::from(Characteristics {
                    name: name.into(),
                    types: vec![Type::Creature.def()],
                    subtypes: vec![subtype("Goblin")],
                    power: Some(deckmaste_core::StatValue::Number(1)),
                    toughness: Some(deckmaste_core::StatValue::Number(1)),
                    ..Characteristics::default()
                })),
            )
        }

        // Builds the Krenko scenario fresh (three controlled Goblins, incl. the
        // source, plus the activation's own Stack-zone copy of the source),
        // runs `Create(CountOf(filter), 1/1 Goblin)` once, and returns how many
        // tokens entered. A fresh state per call keeps the two filters'
        // token batches from feeding each other's count. `filter` is parsed
        // (and its `Permanent` macro expanded) through the live plugin macros.
        fn tokens_made(filter: &str) -> usize {
            let mut state = game();
            let source = goblin(&mut state, "Krenko, Mob Boss");
            let _g2 = goblin(&mut state, "Goblin Two");
            let _g3 = goblin(&mut state, "Goblin Three");

            // The activation mints a Stack-zone identity that REUSES the
            // source's card id ([CR#602.2a]) — the LKI copy that drives the
            // over-count. `eval_count` enumerates every object in the store, so
            // minting it into the Stack zone is enough for the unzoned filter
            // to reach it.
            let src_card = state.objects.obj(source).card_id().unwrap();
            let _stack_copy =
                state
                    .objects
                    .mint(ObjectSource::Card(src_card), PlayerId(0), Some(Zone::Stack));

            // `canon()` (not `builtin()`): the filter names the canon-declared
            // `Goblin` subtype, whose macro lives in canon — a bare
            // `Subtype(Goblin)` only expands with that macro in
            // scope. Parsed through the SEMANTICS path
            // (`semantics::Predicate` → `lower()`), the path
            // production now takes.
            let semantic: deckmaste_semantics::Predicate = canon().macros.read_str(filter).unwrap();
            let parsed: Predicate = deckmaste_lowering::Lower::lower(semantic);
            let frame = frame_src(&state, source);
            let before = state.zones.battlefield.len();
            state.run_effect(
                Instruction::act(Action::Create {
                    agent: Reference::Reg(deckmaste_core::RefId(1)),
                    count: Count::CountOf(Countable::Objects(Arc::new(
                        deckmaste_core::Region::candidate(parsed),
                    ))),
                    token: deckmaste_core::Token {
                        name: None,
                        color_indicator: vec![].into(),
                        supertypes: vec![].into(),
                        types: vec![Type::Creature.def()].into(),
                        subtypes: vec![subtype("Goblin")].into(),
                        abilities: vec![].into(),
                        power: Some(deckmaste_core::StatValue::Number(1)),
                        toughness: Some(deckmaste_core::StatValue::Number(1)),
                    }
                    .into(),
                    riders: vec![].into(),
                }),
                &frame,
            );
            // Drain the queued work (the TokenCreated batch + per-token
            // enters).
            while let StepOutcome::Progress(_) = state.step() {}
            state.zones.battlefield.len() - before
        }

        // Bare subtype (the pre-fix parser output): the Stack-zone copy is a
        // Goblin you control too, so it over-counts → 4.
        assert_eq!(
            tokens_made("And([Subtype(Goblin), ControlledBy(Ref(You))])"),
            4,
            "the unzoned filter wrongly counts the on-stack copy"
        );

        // The canonical battlefield-scoped filter (the post-fix parser output):
        // the Stack-zone copy is excluded → exactly the three battlefield
        // Goblins.
        assert_eq!(
            tokens_made("And([Permanent, Subtype(Goblin), ControlledBy(Ref(You))])"),
            3,
            "[CR#109.2]: the Permanent scope counts only battlefield Goblins"
        );
    }

    /// `Count::EventCount` is the count-valued twin of `Condition::Happened`:
    /// it scans the history log within the given window and returns how many
    /// facts match the `Event` pattern via `event_matches` ([CR#608.2i]).
    /// Two creature-death facts recorded this turn → count == 2; a non-matching
    /// pattern (zone-enter) or a turn with no facts → count == 0.
    #[test]
    fn event_count_counts_matching_history() {
        use deckmaste_core::EventFilter;

        use crate::lki::LkiSnapshot;

        let bears = Arc::new(canon().card("Grizzly Bears").unwrap().core);
        let forest = Arc::new(builtin().card("Forest").unwrap().core);
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

        // Build two creature-death GameEvents (same shape as the morbid test).
        let first_bear_card = state.cards.push(Arc::clone(&bears), PlayerId(0));
        let first_bear = state.objects.mint(
            ObjectSource::Card(first_bear_card),
            PlayerId(0),
            Some(Zone::Battlefield),
        );
        state.zones.battlefield.push(first_bear);
        let death1 = GameEvent::ZoneChange(ZoneChange {
            object: first_bear,
            snapshot: Some(Box::new(LkiSnapshot::capture(&state, first_bear))),
            from: Some(Zone::Battlefield),
            to: Zone::Graveyard,
            enters: None,
            position: None,
            face: None,
            cause: None,
        });

        let second_bear_card = state.cards.push(Arc::clone(&bears), PlayerId(0));
        let second_bear = state.objects.mint(
            ObjectSource::Card(second_bear_card),
            PlayerId(0),
            Some(Zone::Battlefield),
        );
        state.zones.battlefield.push(second_bear);
        let death2 = GameEvent::ZoneChange(ZoneChange {
            object: second_bear,
            snapshot: Some(Box::new(LkiSnapshot::capture(&state, second_bear))),
            from: Some(Zone::Battlefield),
            to: Zone::Graveyard,
            enters: None,
            position: None,
            face: None,
            cause: None,
        });

        // The creature-death event pattern (same as morbid
        // Condition::Happened).
        let death_pattern = EventFilter::ZoneChange {
            what: Predicate::creature(),
            from: Some(Zone::Battlefield),
            to: Some(Zone::Graveyard),
            cause: None,
        };

        // A non-matching pattern: creatures entering the battlefield.
        let enter_pattern = EventFilter::ZoneChange {
            what: Predicate::creature(),
            from: None,
            to: Some(Zone::Battlefield),
            cause: None,
        };

        let frame = frame_for(&state, PlayerId(0));

        // No deaths recorded yet → 0.
        assert_eq!(
            state.eval_count(
                &Count::EventCount(Arc::new(death_pattern.clone()), Lookback::ThisTurn),
                &frame
            ),
            0,
            "no deaths recorded yet"
        );

        // Record two deaths this turn.
        state.record_history_fact(1, None, death1);
        state.record_history_fact(1, None, death2);

        // Both deaths match → 2.
        assert_eq!(
            state.eval_count(
                &Count::EventCount(Arc::new(death_pattern.clone()), Lookback::ThisTurn),
                &frame
            ),
            2,
            "two creature deaths this turn"
        );

        // A non-matching pattern → 0.
        assert_eq!(
            state.eval_count(
                &Count::EventCount(Arc::new(enter_pattern), Lookback::ThisTurn),
                &frame
            ),
            0,
            "enter pattern does not match death facts"
        );

        // Advance to turn 2: ThisTurn sees 0, ThisGame sees 2.
        state.turn.turn_number = 2;
        assert_eq!(
            state.eval_count(
                &Count::EventCount(Arc::new(death_pattern.clone()), Lookback::ThisTurn),
                &frame
            ),
            0,
            "ThisTurn no longer sees last turn's deaths"
        );
        assert_eq!(
            state.eval_count(
                &Count::EventCount(Arc::new(death_pattern), Lookback::ThisGame),
                &frame
            ),
            2,
            "ThisGame still sees last turn's deaths"
        );
    }

    /// `EventSum` sums the `amount` field of matching `LifeLost` facts within
    /// the window ([CR#608.2i,119.3]). Two losses of 2 and 3 by the same player
    /// total 5; a third loss by an opponent does not contribute. After a turn
    /// advance `ThisTurn` reads 0 while `ThisGame` still reads 5.
    #[test]
    fn event_sum_totals_amounts() {
        use deckmaste_core::EventFilter;

        let mut state = GameState::new(GameConfig {
            players: vec![
                PlayerConfig { deck: Vec::new() },
                PlayerConfig { deck: Vec::new() },
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

        let frame = frame_for(&state, PlayerId(0));

        let lose_life_pattern = EventFilter::LifeLost {
            who: deckmaste_core::Predicate::Ref(deckmaste_core::Reference::Reg(
                deckmaste_core::RefId(1),
            )),
            amount: None,
        };

        // No facts yet → 0.
        assert_eq!(
            state.eval_count(
                &Count::EventSum(Arc::new(lose_life_pattern.clone()), Lookback::ThisTurn),
                &frame
            ),
            0,
            "no life-loss facts yet"
        );

        // Record two life-loss facts for player 0 (you) and one for player 1
        // (opponent).
        state.record_history_fact(
            1,
            None,
            GameEvent::LifeLost(LifeLost {
                player: PlayerId(0),
                amount: 2,
                cause: None,
            }),
        );
        state.record_history_fact(
            1,
            None,
            GameEvent::LifeLost(LifeLost {
                player: PlayerId(0),
                amount: 3,
                cause: None,
            }),
        );
        state.record_history_fact(
            1,
            None,
            GameEvent::LifeLost(LifeLost {
                player: PlayerId(1),
                amount: 10,
                cause: None,
            }),
        );

        // Only player 0's losses sum → 5.
        assert_eq!(
            state.eval_count(
                &Count::EventSum(Arc::new(lose_life_pattern.clone()), Lookback::ThisTurn),
                &frame
            ),
            5,
            "two life-loss facts for you: 2 + 3 = 5"
        );

        // Advance to turn 2: ThisTurn sees 0, ThisGame sees 5.
        state.turn.turn_number = 2;
        assert_eq!(
            state.eval_count(
                &Count::EventSum(Arc::new(lose_life_pattern.clone()), Lookback::ThisTurn),
                &frame
            ),
            0,
            "ThisTurn no longer sees last turn's life losses"
        );
        assert_eq!(
            state.eval_count(
                &Count::EventSum(Arc::new(lose_life_pattern), Lookback::ThisGame),
                &frame
            ),
            5,
            "ThisGame still sees 5 total life lost by you"
        );
    }

    /// `EventCount(Used(by: This))` is OBJECT-scoped: it resolves `by` to the
    /// frame's source `ObjectId` and counts that object's `AbilityUsed` facts,
    /// NOT a watcher-pattern match ([CR#608.2i,603.2]). Two uses by the frame
    /// object this turn → 2 (a third use by a DIFFERENT object is excluded);
    /// after a turn advance `ThisTurn` reads 0 while `ThisGame` still reads 2.
    #[test]
    fn event_count_used_counts_object_ability_uses() {
        use deckmaste_core::EventFilter;

        let mut state = GameState::new(GameConfig {
            players: vec![
                PlayerConfig { deck: Vec::new() },
                PlayerConfig { deck: Vec::new() },
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

        // The frame's source is `obj`; `Used(by: This)` resolves `This` to it.
        let obj = ObjectId::from_raw(1);
        let other = ObjectId::from_raw(2);
        let frame = frame_src(&state, obj);

        let used = |n| {
            Count::EventCount(
                Arc::new(EventFilter::Used {
                    of: Reference::Reg(deckmaste_core::RefId(0)),
                }),
                n,
            )
        };

        // No uses recorded yet → 0.
        assert_eq!(
            state.eval_count(&used(Lookback::ThisTurn), &frame),
            0,
            "no ability uses recorded yet"
        );

        // Two uses by `obj` this turn, and one by `other`.
        state.record_history_fact(
            1,
            None,
            GameEvent::AbilityUsed(AbilityUsed {
                object: obj,
                ability: 0,
            }),
        );
        state.record_history_fact(
            1,
            None,
            GameEvent::AbilityUsed(AbilityUsed {
                object: obj,
                ability: 0,
            }),
        );
        state.record_history_fact(
            1,
            None,
            GameEvent::AbilityUsed(AbilityUsed {
                object: other,
                ability: 0,
            }),
        );

        // Only `obj`'s two uses count (`This` == frame.source(self) == obj).
        assert_eq!(
            state.eval_count(&used(Lookback::ThisTurn), &frame),
            2,
            "two uses by the frame object; the other object's use is excluded"
        );

        // Advance to turn 2: ThisTurn sees 0, ThisGame still sees the 2.
        state.turn.turn_number = 2;
        assert_eq!(
            state.eval_count(&used(Lookback::ThisTurn), &frame),
            0,
            "ThisTurn no longer sees last turn's uses"
        );
        assert_eq!(
            state.eval_count(&used(Lookback::ThisGame), &frame),
            2,
            "ThisGame still sees last turn's two uses"
        );
    }

    /// The card-facing payoff: a self-use count drives a branching condition
    /// (`If(Compare(EventCount(Used(by: This), ThisTurn), Eq, 2), then,
    /// else)`). The `Compare` is FALSE after one recorded use of the frame
    /// object and TRUE after the second — the ability's own use-count keys
    /// the branch ([CR#608.2i]).
    #[test]
    fn event_count_self_drives_branching_condition() {
        use deckmaste_core::Cmp;
        use deckmaste_core::Condition;
        use deckmaste_core::EventFilter;

        let mut state = GameState::new(GameConfig {
            players: vec![
                PlayerConfig { deck: Vec::new() },
                PlayerConfig { deck: Vec::new() },
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

        let obj = ObjectId::from_raw(1);
        let frame = frame_src(&state, obj);

        // "if this object's abilities have been used exactly twice this turn".
        let twice = Condition::Compare(
            Count::EventCount(
                Arc::new(EventFilter::Used {
                    of: Reference::Reg(deckmaste_core::RefId(0)),
                }),
                Lookback::ThisTurn,
            ),
            Cmp::Eq,
            Count::Literal(2),
        );

        // One use → not yet two → branch is FALSE.
        state.record_history_fact(
            1,
            None,
            GameEvent::AbilityUsed(AbilityUsed {
                object: obj,
                ability: 0,
            }),
        );
        assert!(
            !state.condition_holds(&twice, &frame),
            "one self-use does not satisfy `== 2`"
        );

        // Second use → exactly two → branch is TRUE.
        state.record_history_fact(
            1,
            None,
            GameEvent::AbilityUsed(AbilityUsed {
                object: obj,
                ability: 0,
            }),
        );
        assert!(
            state.condition_holds(&twice, &frame),
            "two self-uses satisfy `== 2`"
        );
    }

    /// `Count` arithmetic ([CR#107.1]) evaluates structurally — no board
    /// needed. `Minus` floors at 0 ([CR#107.1b]); `Half` rounds per the mode.
    #[test]
    fn count_arithmetic_evaluates() {
        let state = game();
        let frame = frame_for(&state, PlayerId(0));
        let lit = |n| Arc::new(Count::Literal(n));
        let ev = |c: &Count| state.eval_count(c, &frame);
        assert_eq!(ev(&Count::Plus(lit(2), lit(3))), 5);
        assert_eq!(ev(&Count::Minus(lit(2), lit(5))), 0, "a count floors at 0");
        assert_eq!(ev(&Count::Times(lit(2), lit(3))), 6);
        assert_eq!(ev(&Count::Max(lit(2), lit(3))), 3);
        assert_eq!(
            ev(&Count::Half(deckmaste_core::RoundMode::RoundUp, lit(3))),
            2,
        );
        assert_eq!(
            ev(&Count::Half(deckmaste_core::RoundMode::RoundDown, lit(3))),
            1,
        );
    }

    /// A battlefield-scoped creature filter (canonical card filters carry
    /// their own zone narrowing) — keeps the deck's hand/library bears out of
    /// the matched set.
    fn creatures_in_play() -> Predicate {
        Predicate::And(
            vec![
                Predicate::State(StatePredicate::InZone(Zone::Battlefield)),
                Predicate::creature(),
            ]
            .into(),
        )
    }

    /// `CountDistinct` ([CR#107.3]) is the distinct-union size of an axis over
    /// the matched set: three creatures of powers 2/2/3 give two distinct
    /// powers (Coven), and three distinct toughnesses 2/4/3.
    #[test]
    fn count_distinct_over_creatures() {
        let (state, _) = battlefield_with(&["Grizzly Bears", "Giant Spider", "Centaur Courser"]);
        let frame = frame_for(&state, PlayerId(0));
        let powers = Count::CountDistinct(
            deckmaste_core::Characteristic::Power,
            Countable::Objects(Arc::new(deckmaste_core::Region::candidate(
                creatures_in_play(),
            ))),
        );
        assert_eq!(
            state.eval_count(&powers, &frame),
            2,
            "distinct powers {{2,3}}"
        );
        let toughnesses = Count::CountDistinct(
            deckmaste_core::Characteristic::Toughness,
            Countable::Objects(Arc::new(deckmaste_core::Region::candidate(
                creatures_in_play(),
            ))),
        );
        assert_eq!(
            state.eval_count(&toughnesses, &frame),
            3,
            "distinct toughnesses {{2,4,3}}",
        );
    }

    /// A per-candidate region: the candidate at parameter zero, then the
    /// enclosing source and controller — the prefix lowering's `in_child`
    /// builds for a nested predicate or projection.
    fn candidate_region<T>(body: T) -> Arc<deckmaste_core::Region<T>> {
        Arc::new(deckmaste_core::Region::new(
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
                    provenance: deckmaste_core::Provenance::Source,
                },
                deckmaste_core::Param {
                    def: deckmaste_core::DefId(2),
                    kind: deckmaste_core::Kind::Entity,
                    provenance: deckmaste_core::Provenance::Controller,
                },
            ]),
            body,
        ))
    }

    /// The element under test inside a [`candidate_region`] — the register the
    /// retired `Reference::It` named.
    const ELEMENT: deckmaste_core::RefId = deckmaste_core::RefId(0);
    /// The enclosing controller as seen from inside a [`candidate_region`].
    const NESTED_CONTROLLER: deckmaste_core::RefId = deckmaste_core::RefId(2);

    /// `Count::Aggregate(op, Projection)` folds a per-element `Count` over the
    /// projected set ([CR#107.1]), each element supplied through the
    /// projection region's candidate parameter. Devotion decomposes to
    /// `Aggregate(SumOf, Project(<your permanents>, CountOf(ManaSymbols(<the
    /// element>, CountsAs(Green)))))` ([CR#700.5]); `SumOf` over
    /// `StatOf(<element>, Power)` totals power; every `AggregateOp` folds the
    /// empty set to 0 (never-crash).
    #[test]
    fn aggregate_folds_a_projection_over_a_selection() {
        use deckmaste_core::AggregateOp;
        use deckmaste_core::Color;
        use deckmaste_core::Projection;
        use deckmaste_core::RelationPredicate;
        use deckmaste_core::StatePredicate;
        use deckmaste_core::SymbolPred;

        let mut state = game();
        let _ = permanent_with_cost(&mut state, "{2}{G}");
        let _ = permanent_with_cost(&mut state, "{G}{G}");
        let src = permanent_with_cost(&mut state, "{1}");
        let frame = frame_src(&state, src);

        let your_permanents = || {
            Predicate::And(
                vec![
                    Predicate::State(StatePredicate::InZone(Zone::Battlefield)),
                    Predicate::Relation(RelationPredicate::ControlledBy(Arc::new(Predicate::Ref(
                        Reference::Reg(NESTED_CONTROLLER),
                    )))),
                ]
                .into(),
            )
        };

        let devotion_green = Count::Aggregate(
            AggregateOp::SumOf,
            Projection {
                of: Countable::Objects(candidate_region(your_permanents())),
                by: candidate_region(Count::CountOf(Countable::ManaSymbols(
                    Arc::new(Reference::Reg(ELEMENT)),
                    SymbolPred::CountsAs(Color::Green),
                ))),
            },
        );
        assert_eq!(
            state.eval_count(&devotion_green, &frame),
            3,
            "{{2}}{{G}} + {{G}}{{G}} + {{1}} = 3 green pips total"
        );

        // `SumOf` over `StatOf(<element>, Power)`: total power of your
        // creatures.
        let mut power_state = game();
        let src = permanent_with_cost(&mut power_state, "{1}");
        let _ = creature_with_power(&mut power_state, 2);
        let _ = creature_with_power(&mut power_state, 5);
        let frame = frame_src(&power_state, src);
        let total_power = Count::Aggregate(
            AggregateOp::SumOf,
            Projection {
                of: Countable::Objects(candidate_region(Predicate::And(
                    vec![
                        Predicate::State(StatePredicate::InZone(Zone::Battlefield)),
                        Predicate::creature(),
                    ]
                    .into(),
                ))),
                by: candidate_region(Count::StatOf(
                    Reference::Reg(ELEMENT),
                    deckmaste_core::Stat::Power,
                )),
            },
        );
        assert_eq!(power_state.eval_count(&total_power, &frame), 7);

        // The empty set folds every `AggregateOp` to 0.
        for op in [
            AggregateOp::SumOf,
            AggregateOp::MinOf,
            AggregateOp::MaxOf,
            AggregateOp::AverageOf(deckmaste_core::RoundMode::RoundUp),
        ] {
            let empty_fold = Count::Aggregate(
                op,
                Projection {
                    of: Countable::Objects(candidate_region(Predicate::Not(Arc::new(
                        Predicate::Any,
                    )))),
                    by: candidate_region(Count::StatOf(
                        Reference::Reg(ELEMENT),
                        deckmaste_core::Stat::Power,
                    )),
                },
            );
            assert_eq!(
                power_state.eval_count(&empty_fold, &frame),
                0,
                "{op:?} over the empty set is 0"
            );
        }
    }

    /// Devotion end-to-end ([CR#700.5]), the two cases
    /// `aggregate_folds_a_projection_over_a_selection` doesn't already cover:
    /// a two-color disjunction (`Or([White, Black])`) summed across SEPARATE
    /// permanents (not just one object's multiple pips), and an actually
    /// empty battlefield (no permanents minted at all, not a `Not(Any)`
    /// filter trick).
    #[test]
    fn devotion_sums_a_color_disjunction_across_permanents_and_fizzles_to_zero_on_empty() {
        use deckmaste_core::AggregateOp;
        use deckmaste_core::Color;
        use deckmaste_core::Projection;
        use deckmaste_core::RelationPredicate;
        use deckmaste_core::StatePredicate;
        use deckmaste_core::SymbolPred;

        let your_permanents = || {
            Predicate::And(
                vec![
                    Predicate::State(StatePredicate::InZone(Zone::Battlefield)),
                    Predicate::Relation(RelationPredicate::ControlledBy(Arc::new(Predicate::Ref(
                        Reference::Reg(NESTED_CONTROLLER),
                    )))),
                ]
                .into(),
            )
        };
        let devotion_wb = || {
            Count::Aggregate(
                AggregateOp::SumOf,
                Projection {
                    of: Countable::Objects(candidate_region(your_permanents())),
                    by: candidate_region(Count::CountOf(Countable::ManaSymbols(
                        Arc::new(Reference::Reg(ELEMENT)),
                        SymbolPred::Or(
                            vec![
                                SymbolPred::CountsAs(Color::White),
                                SymbolPred::CountsAs(Color::Black),
                            ]
                            .into(),
                        ),
                    ))),
                },
            )
        };

        // `{W}{B}{W/B}` split across three separate permanents you control =
        // 1 + 1 + 1 = 3 (the hybrid pip counts toward both W and B devotion,
        // but only once per object — `Or` matches, it doesn't double-count).
        let mut state = game();
        let _ = permanent_with_cost(&mut state, "{W}");
        let _ = permanent_with_cost(&mut state, "{B}");
        let src = permanent_with_cost(&mut state, "{W/B}");
        let frame = frame_src(&state, src);
        assert_eq!(
            state.eval_count(&devotion_wb(), &frame),
            3,
            "{{W}} + {{B}} + {{W/B}} = 3 devotion to white-and-black"
        );

        // An empty battlefield — no permanents at all, not an artificial
        // never-matching filter — folds to 0 (never-crash). `src` itself
        // lives in hand, so `InZone(Battlefield)` matches nothing.
        let mut empty_state = game();
        let card = Card::Normal(CardFace::from(Characteristics {
            name: "Test Card".into(),
            mana_cost: "{1}".parse().unwrap(),
            types: vec![Type::Artifact.def()],
            ..Characteristics::default()
        }));
        let cid = empty_state.cards.push(Arc::new(card), PlayerId(0));
        let src = empty_state
            .objects
            .mint(ObjectSource::Card(cid), PlayerId(0), Some(Zone::Hand));
        let frame = frame_src(&empty_state, src);
        assert_eq!(
            empty_state.eval_count(&devotion_wb(), &frame),
            0,
            "no permanents on the battlefield → devotion 0"
        );
    }

    /// `Selection::Pick` ([CR#107.1]) takes the extremal element: the creature
    /// with the greatest power is the 3/3 Centaur Courser; the least-power pick
    /// is the whole tied 2-power group.
    #[test]
    fn pick_extremal_creature_by_power() {
        let (state, ids) = battlefield_with(&["Grizzly Bears", "Giant Spider", "Centaur Courser"]);
        let courser = ids[2];
        let frame = frame_for(&state, PlayerId(0));
        let pick = |op| deckmaste_core::Selection::Pick {
            op,
            proj: deckmaste_core::Projection {
                of: deckmaste_core::Countable::Objects(candidate_region(creatures_in_play())),
                by: candidate_region(Count::StatOf(
                    Reference::Reg(ELEMENT),
                    deckmaste_core::Stat::Power,
                )),
            },
        };
        assert_eq!(
            state.eval_selection_set(&pick(deckmaste_core::AggregateOp::MaxOf), &frame),
            vec![courser],
            "Centaur Courser (3 power) is the unique greatest",
        );
        let picked = state.eval_selection_set(&pick(deckmaste_core::AggregateOp::MinOf), &frame);
        assert_eq!(picked.len(), 2, "the two 2-power creatures tie for least");
        assert!(
            !picked.contains(&courser),
            "the 3-power creature is not least"
        );
    }

    /// The cross-player fold ([CR#119.1] Arbiter of Knollridge): `Aggregate`
    /// over a `Countable::Players` source reads each matching player's
    /// `PlayerStatOf(<element>, Life)` and folds per `AggregateOp` — the
    /// player-sourced twin of `aggregate_folds_a_projection_over_a_selection`'s
    /// object-sourced coverage above. `MaxOf` reads the higher of the two
    /// players' life totals ("the highest life total among all players");
    /// every `AggregateOp` folds an EMPTY player set to 0 (never-crash) —
    /// exercised on a real `Countable::Players` source, not just the object
    /// analog, since `Iterator::min`/`max`'s `None` case is the concrete
    /// panic risk (`.unwrap()` on an empty iterator) the never-crash ruling
    /// guards against.
    #[test]
    fn player_aggregate_folds_life_totals_and_fizzles_to_zero_on_empty() {
        use deckmaste_core::AggregateOp;
        use deckmaste_core::EntityClass;
        use deckmaste_core::PlayerAttr;
        use deckmaste_core::Projection;

        let mut state = game();
        let src = permanent_with_cost(&mut state, "{1}");
        let frame = frame_src(&state, src);
        state.player_mut(PlayerId(0)).life = 12;
        state.player_mut(PlayerId(1)).life = 20;

        let life_fold = |op: AggregateOp, who: Predicate| {
            Count::Aggregate(
                op,
                Projection {
                    of: Countable::Players(candidate_region(who)),
                    by: candidate_region(Count::PlayerStatOf(
                        Reference::Reg(ELEMENT),
                        PlayerAttr::Life,
                    )),
                },
            )
        };
        let all_players = || Predicate::Entity(EntityClass::Player);
        assert_eq!(
            state.eval_count(&life_fold(AggregateOp::MaxOf, all_players()), &frame),
            20,
            "the highest life total among all players"
        );
        assert_eq!(
            state.eval_count(&life_fold(AggregateOp::MinOf, all_players()), &frame),
            12,
            "the lowest life total among all players"
        );
        assert_eq!(
            state.eval_count(&life_fold(AggregateOp::SumOf, all_players()), &frame),
            32,
            "the total life across all players"
        );

        // An empty player set (a semantic-input error, but must stay safe on
        // ANY input, not just the real ≥1-player fixtures) folds every op to
        // 0 rather than panicking on `Iterator::min`/`max` of an empty set.
        for op in [
            AggregateOp::SumOf,
            AggregateOp::MinOf,
            AggregateOp::MaxOf,
            AggregateOp::AverageOf(deckmaste_core::RoundMode::RoundUp),
        ] {
            assert_eq!(
                state.eval_count(
                    &life_fold(op, Predicate::Not(Arc::new(Predicate::Any))),
                    &frame
                ),
                0,
                "{op:?} over the empty player set is 0"
            );
        }
    }

    /// "That much" reads the amount the damage instruction PINNED
    /// ([CR#608.2h]): the magnitude is a definition written before the damage
    /// verb runs, and the later life gain reads that register. Re-spelled from
    /// `that_much_gains_life_equal_to_damage_dealt` — `Count::ThatMuch` and
    /// `GameState.that_much` left with the discourse channel, so the anaphor
    /// is the `Let` lowering now emits ahead of every `DealDamage`.
    #[test]
    fn a_pinned_magnitude_gains_life_equal_to_damage_dealt() {
        // `frame_src_targets` declares source(0), controller(1), the four
        // event roles(2..=5), one announced target(6) and X(7), so the first
        // instruction definition is register 8.
        const AMOUNT: deckmaste_core::DefId = deckmaste_core::DefId(8);
        const TARGET: deckmaste_core::RefId = deckmaste_core::RefId(6);

        let (mut state, bear) = bear_on_field();
        let frame = frame_src_targets(&state, bear, vec![bear]);
        state.run_effect(
            Instruction::Sequentially(
                vec![
                    Instruction::Let(deckmaste_core::Let {
                        dest: AMOUNT,
                        expr: deckmaste_core::Expr::Number(Count::Literal(3)),
                    }),
                    Instruction::act(Action::deal_damage(
                        Reference::Reg(TARGET),
                        Count::Reg(AMOUNT.into()),
                    )),
                    Instruction::act(Action::ChangeLife(
                        Reference::controller_parameter(),
                        deckmaste_core::LifeOp::Up(Count::Reg(AMOUNT.into())),
                    )),
                ]
                .into(),
            ),
            &frame,
        );
        // Let → RunEffect(damage) → Emit(DamageDealt) → RunEffect(gain) →
        // Emit(LifeGained).
        for _ in 0..6 {
            let _ = state.step();
        }
        assert_eq!(state.objects.obj(bear).total_damage(), 3);
        assert_eq!(state.players[0].life, 23);
    }

    /// FIXTURE — a nested `Where` inside a `Pick`, reading BOTH candidates.
    /// ADR law 1: every per-candidate predicate is its own region, so the
    /// `Where` inside the pick's filter reads the PICK's candidate through its
    /// own parameter while the projection nested inside that `Where` reads its
    /// OWN per-element candidate through a distinct one. The depth-0
    /// restriction is gone ([CR#107.1]).
    ///
    /// "The creature with the greatest power among creatures some other
    /// creature outpowers": on a 2/2, 3/3, 4/4 board the filter admits the 2/2
    /// and the 3/3 (the 4/4 is outpowered by nobody), and the pick takes the
    /// 3/3. A reading that confused the two candidates would admit or pick the
    /// 4/4.
    #[test]
    fn a_where_nested_in_a_pick_reads_both_candidates() {
        use deckmaste_core::AggregateOp;
        use deckmaste_core::Cmp;
        use deckmaste_core::Condition;
        use deckmaste_core::Projection;
        use deckmaste_core::StatePredicate;

        let (state, ids) =
            battlefield_with(&["Grizzly Bears", "Fangren Hunter", "Centaur Courser"]);
        let (bears, hunter, courser) = (ids[0], ids[1], ids[2]);
        let frame = frame_for(&state, PlayerId(0));

        let battlefield_creatures = || {
            Predicate::And(
                vec![
                    Predicate::State(StatePredicate::InZone(Zone::Battlefield)),
                    Predicate::creature(),
                ]
                .into(),
            )
        };
        let power_of = |register| {
            candidate_region(Count::StatOf(
                Reference::Reg(register),
                deckmaste_core::Stat::Power,
            ))
        };
        // The greatest power on the board, folded over the projection's OWN
        // per-element candidate.
        let greatest_power = Count::Aggregate(
            AggregateOp::MaxOf,
            Projection {
                of: Countable::Objects(candidate_region(battlefield_creatures())),
                by: power_of(ELEMENT),
            },
        );
        // The pick's filter: a creature the greatest power exceeds. `ELEMENT`
        // here is the WHERE region's own candidate — the pick's candidate —
        // not the projection's.
        let outpowered = Predicate::And(
            vec![
                battlefield_creatures(),
                Predicate::Where(candidate_region(Condition::Compare(
                    greatest_power,
                    Cmp::Greater,
                    Count::StatOf(Reference::Reg(ELEMENT), deckmaste_core::Stat::Power),
                ))),
            ]
            .into(),
        );

        let picked = state.eval_selection_set(
            &deckmaste_core::Selection::Pick {
                op: AggregateOp::MaxOf,
                proj: Projection {
                    of: Countable::Objects(candidate_region(outpowered)),
                    by: power_of(ELEMENT),
                },
            },
            &frame,
        );
        assert_eq!(
            picked,
            vec![courser],
            "the 3/3 is the greatest-power creature that something outpowers"
        );
        assert!(
            !picked.contains(&hunter),
            "the 4/4 is outpowered by nobody, so the nested Where excludes it"
        );
        assert!(
            !picked.contains(&bears),
            "the 2/2 is not the greatest of those"
        );
    }
}
