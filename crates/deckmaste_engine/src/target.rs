//! Targeting ([CR#115]): which objects a `Filter` admits. Stage 2 wires only
//! the arms the corpus's `AnyTarget` reaches; the rest are `todo!`.

use deckmaste_core::CharacteristicFilter;
use deckmaste_core::Filter;
use deckmaste_core::ObjectKind;
use deckmaste_core::Reference;
use deckmaste_core::RelationFilter;
use deckmaste_core::StateFilter;
use deckmaste_core::Type;
use deckmaste_core::Uint;
use deckmaste_core::Zone;

use crate::object::ObjectId;
use crate::object::ObjectSource;
use crate::stack::StackObject;
use crate::state::GameState;

/// The object's kind ([CR#109.1]) as the corpus needs it: an activated/
/// triggered ability on the stack is an `Ability`; a player proxy is a
/// `Player`; a card on the stack is a `Spell`; a created token is a `Token`
/// ([CR#111.6] — not a card); otherwise a `Card`.
///
/// The ability check outranks all the source-based ones: an ability on the
/// stack ([CR#602.2a,603.3]) carries a freshly minted `StackEntry.id` that
/// *shares the source's `ObjectSource`* (the card/player it came from), so the
/// match below would misread it as a `Spell` (card-in-stack) or `Player`. Its
/// stack identity is the only place its abilityhood is knowable, so consult the
/// stack first — that also covers an ability minted from a token source (the
/// ability on the stack, not the token itself).
#[must_use]
pub fn object_kind(state: &GameState, id: ObjectId) -> ObjectKind {
    if state.stack.iter().any(|e| {
        e.id == id
            && matches!(
                e.object,
                StackObject::Triggered { .. } | StackObject::Activated { .. }
            )
    }) {
        return ObjectKind::Ability;
    }
    let obj = state.objects.obj(id);
    match obj.source {
        ObjectSource::Player(_) => ObjectKind::Player,
        ObjectSource::Card(_) if obj.zone == Some(Zone::Stack) => ObjectKind::Spell,
        ObjectSource::Card(c) if state.cards.get(c).is_token => ObjectKind::Token,
        ObjectSource::Card(_) => ObjectKind::Card,
    }
}

/// Whether `id` matches `filter`, with no carrier context — the targeting /
/// `candidates` path ([CR#115]). A thin wrapper over [`matches_with`] with no
/// watcher; filters needing a carrier (`Ref(This)`/`Ref(You)`) are unreachable
/// here (targeting threads no frame) and `todo!`.
#[must_use]
pub fn matches(state: &GameState, id: ObjectId, filter: &Filter) -> bool {
    matches_with(state, id, filter, None)
}

/// Whether the live object `id` matches `filter`. `watcher` is the carrier of
/// the ability doing the matching (`Some` in the trigger lane, `None` for
/// frameless targeting); it anchors `Ref(This)`/`Ref(You)` and threads into
/// nested relation/combinator sub-filters so a nested self-reference still
/// resolves against the carrier.
///
/// This is the single live-object matcher: [`matches`] passes `None`, and the
/// trigger lane's `filter_matches_live` passes `Some(watcher)`. The LKI sibling
/// for moved/gone objects is `GameState::filter_matches_snapshot`.
#[must_use]
#[expect(clippy::too_many_lines, reason = "flat per-variant Filter dispatch")]
pub fn matches_with(
    state: &GameState,
    id: ObjectId,
    filter: &Filter,
    watcher: Option<ObjectSource>,
) -> bool {
    match filter {
        Filter::Kind(k) => object_kind(state, id) == *k,
        Filter::Characteristic(CharacteristicFilter::Type(t)) => has_type(state, id, *t),
        // [CR#110.5a]: state, not characteristic — card/token objects only, so a
        // player proxy (zone None) never matches InZone.
        Filter::State(StateFilter::InZone(z)) => state.objects.obj(id).zone == Some(*z),
        Filter::AllOf(fs) => fs.iter().all(|f| matches_with(state, id, f, watcher)),
        Filter::OneOf(fs) => fs.iter().any(|f| matches_with(state, id, f, watcher)),
        Filter::Not(f) => !matches_with(state, id, f, watcher),
        // The candidate matches iff the condition holds with the iteration
        // anaphor `It` bound to it. `This`/`You` still anchor to the carrier, so
        // build a match frame from the watcher (source → live carrier id +
        // controller) and set `it` to the candidate. Re-binding across nested
        // relation filters is automatic: those arms recurse with a new `id`, so
        // a nested `Where` sees the related object as `It`. A frameless caller
        // (no watcher) or a gone carrier leaves `This` unresolvable — no match.
        Filter::Where(cond) => match watcher {
            None => todo!(
                "Filter::Where at a frameless position — the matcher holds no carrier for This/You"
            ),
            Some(w) => {
                match state
                    .objects
                    .iter()
                    .find(|ob| ob.source == w)
                    .map(|ob| (ob.id, ob.controller))
                {
                    None => false,
                    Some((carrier, controller)) => {
                        let frame = crate::stack::Frame {
                            endophora: crate::stack::Endophora {
                                it: Some(state.it_binding(id)),
                                ..crate::stack::Endophora::empty()
                            },
                            ..crate::stack::Frame::bare(carrier, controller)
                        };
                        state.condition_holds(cond, &frame)
                    }
                }
            }
        },
        Filter::Any => true,
        // A filter-position macro (`kinds: [Filter]` — `Self`, evasion sets,
        // protection qualities) survives expansion as `Filter::Expanded`;
        // evaluate it transparently through the remembered body. (Target-position
        // macros like `AnyTarget` are looked through earlier, in
        // `resolve::target_spec_filter`, so they never reach here.)
        Filter::Expanded(e) => matches_with(state, id, &e.value, watcher),
        // [CR#702]: keyword presence by NAME against the DERIVED abilities
        // (granted keywords count; the carried Composite name survives
        // expansion). Per-call layers() rebuild — a perf seam if a hot path
        // ever evaluates Has in bulk.
        Filter::Characteristic(CharacteristicFilter::Has(kw)) => {
            let view = state.layers();
            view.get(id)
                .abilities
                .iter()
                .any(|a| crate::layer::ability_is_named(a, &kw.0))
        }
        // Subtype presence by NAME against the DERIVED subtype list
        // ([CR#205.3] — layer-4 type changes count); a player proxy has
        // none. Same per-call layers() perf seam as `Has`.
        Filter::Characteristic(CharacteristicFilter::Subtype(name)) => {
            state.objects.obj(id).card_id().is_some()
                && state
                    .layers()
                    .get(id)
                    .subtypes
                    .iter()
                    .any(|s| s.name == *name)
        }
        // [CR#122.1] counters go on objects AND players — player counters
        // live on the player's proxy object, so one LIVE read serves both.
        Filter::State(StateFilter::HasCounter(kind)) => state
            .objects
            .obj(id)
            .counters
            .get(kind.as_str())
            .is_some_and(|&n| n > 0),
        // [CR#109.3] designations are non-characteristic state: a LIVE read
        // against the registry (object entry, or the player's for proxies).
        Filter::State(StateFilter::Designated(name)) => {
            state
                .designations
                .objects
                .get(&(id, *name))
                .is_some_and(|instances| !instances.is_empty())
                || matches!(
                    state.objects.obj(id).source,
                    ObjectSource::Player(p)
                        if state.designations.players.contains_key(&(p, *name)))
        }

        // "this object" ([CR#603.10a]): match only when `id` is the watching
        // object. Unreachable without a carrier (frameless targeting).
        Filter::Ref(Reference::This) => match watcher {
            Some(w) => state.objects.obj(id).source == w,
            None => todo!("Ref(This) at a frameless position — targeting threads no carrier"),
        },
        // "you" ([CR#109.5]): `id` is the watcher's controller's proxy.
        Filter::Ref(Reference::You) => match watcher {
            Some(w) => {
                let controller = state.controller_of_source(w);
                matches!(state.objects.obj(id).source,
                    ObjectSource::Player(p) if Some(p) == controller)
            }
            None => todo!("Ref(You) at a frameless position — targeting threads no carrier"),
        },
        // "the host of THIS attachment" ([CR#301.5,303.4]): `id` is the
        // permanent that THIS (the watcher) is attached to. Used by umbra/totem
        // armor to express "the enchanted permanent" as the watched subject of
        // a static other-watching replacement ([CR#702.89a]).
        //
        // Only `inner = This` is resolvable from the carrier alone (no Frame).
        // The general `AttachHostOf(inner)` case needs `eval_reference` and is
        // therefore left as a seam.
        Filter::Ref(Reference::AttachHostOf(inner))
            if matches!(inner.as_ref(), Reference::This) =>
        {
            match watcher {
                Some(w) => {
                    // Find the live object whose source is the watcher (the Aura)
                    // and check whether it is attached to `id` (the would-be
                    // destroyed creature). If the watcher has no live object on
                    // the battlefield (e.g. it left already), the match fails.
                    state
                        .objects
                        .iter()
                        .find(|o| o.source == w)
                        .is_some_and(|o| o.attached_to == Some(id))
                }
                None => todo!(
                    "Ref(AttachHostOf(This)) at a frameless position — targeting threads no carrier"
                ),
            }
        }

        // "named X" ([CR#201]): printed face name; a player proxy has no card.
        Filter::Characteristic(CharacteristicFilter::Named(name)) => {
            state.objects.obj(id).card_id().is_some()
                && crate::derive::face(state.def(id)).name.as_str() == name.as_str()
        }
        // Color predicates over the DERIVED colors ([CR#105.2,202.2]) — a
        // layer-5 color change counts. Same per-call layers() perf seam as
        // `Has`. A player proxy is not a colored object.
        Filter::Characteristic(CharacteristicFilter::ColorIs(c)) => {
            state.objects.obj(id).card_id().is_some() && state.layers().get(id).colors.contains(c)
        }
        // [CR#105.2b]: two or more colors.
        Filter::Characteristic(CharacteristicFilter::Multicolored) => {
            state.objects.obj(id).card_id().is_some() && state.layers().get(id).colors.len() >= 2
        }
        // [CR#105.2c]: colorless is the ABSENCE of color, not a color itself.
        Filter::Characteristic(CharacteristicFilter::Colorless) => {
            state.objects.obj(id).card_id().is_some() && state.layers().get(id).colors.is_empty()
        }
        // Supertype presence over the DERIVED list ([CR#205.4a]); player proxy
        // has none.
        Filter::Characteristic(CharacteristicFilter::Supertype(s)) => {
            state.objects.obj(id).card_id().is_some()
                && state.layers().get(id).supertypes.contains(s)
        }
        // [CR#208,202.3]: a derived stat compares against a bound. The bound is
        // a `Count`; only a literal evaluates here (a dynamic bound needs a
        // carrier frame the matcher does not carry — see `const_count`). A
        // missing stat (a land has no power) never satisfies the predicate.
        Filter::Characteristic(CharacteristicFilter::Stat(stat, cmp, count)) => {
            state.objects.obj(id).card_id().is_some()
                && stat_satisfies(derived_stat(state, id, *stat), *cmp, count)
        }

        // The object's controller, as a player proxy ([CR#109.5]). Recurses
        // with the SAME watcher so a nested `Ref(You)` still anchors right.
        Filter::Relation(RelationFilter::ControlledBy(f)) => {
            let c = state.objects.obj(id).controller;
            let proxy = state.player(c).object;
            matches_with(state, proxy, f, watcher)
        }
        // `id` is a player who is an opponent of a matching player
        // ([CR#102.2,102.3,810.1]): on a DIFFERENT team. Routed through
        // `same_team` so it stays sound when team play (Two-Headed Giant) is
        // modeled — today, with singleton teams, "different team" == "different
        // player", the prior behavior.
        Filter::Relation(RelationFilter::OpponentOf(f)) => match state.objects.obj(id).source {
            ObjectSource::Player(p) => state
                .players
                .iter()
                .any(|q| !state.same_team(p, q.id) && matches_with(state, q.object, f, watcher)),
            ObjectSource::Card(_) => false,
        },
        // `id` is a player who is a teammate of a matching player
        // ([CR#102.3,810.1]): ANOTHER player (never `q` itself) on the SAME
        // team. Team membership isn't modeled yet (singleton teams), so this
        // matches nobody outside a team game — the correct answer for 1v1 and
        // free-for-all, where no two players share a team. See
        // `GameState::same_team`.
        Filter::Relation(RelationFilter::TeammateOf(f)) => match state.objects.obj(id).source {
            ObjectSource::Player(p) => state.players.iter().any(|q| {
                q.id != p && state.same_team(p, q.id) && matches_with(state, q.object, f, watcher)
            }),
            ObjectSource::Card(_) => false,
        },
        // The object's owner, as a player proxy ([CR#108.3]); a player proxy
        // has no owner.
        Filter::Relation(RelationFilter::Owner(f)) => {
            state.objects.obj(id).card_id().is_some()
                && matches_with(state, state.player(state.owner_of(id)).object, f, watcher)
        }
        // `id` is a player who controls a matching object — the inverse of
        // `ControlledBy` ([CR#109.5]). Zone-agnostic: the inner filter carries
        // any zone restriction (proxies, zone `None`, fall out of e.g.
        // `Permanent`). A card is never a controlling player.
        Filter::Relation(RelationFilter::Controls(f)) => match state.objects.obj(id).source {
            ObjectSource::Player(p) => state
                .objects
                .iter()
                .any(|ob| ob.controller == p && matches_with(state, ob.id, f, watcher)),
            ObjectSource::Card(_) => false,
        },

        // [CR#110.5]: status. Tap state is stored; flip/face/phasing are not
        // (P0.W6) — a filter over one trips rather than silently read a
        // default. A player proxy has no status.
        Filter::State(StateFilter::Status(status)) => {
            use deckmaste_core::Status;
            if state.objects.obj(id).card_id().is_none() {
                return false;
            }
            let tapped = state.objects.obj(id).tapped;
            match status {
                Status::Tapped => tapped,
                Status::Untapped => !tapped,
                Status::Flipped
                | Status::Unflipped
                | Status::FaceDown
                | Status::FaceUp
                | Status::PhasedOut
                | Status::PhasedIn => todo!(
                    "engine-filter-breadth: Status({status:?}) — flip/face/phasing state unstored \
                     (P0.W6)"
                ),
            }
        }
        // [CR#508.1a]: declared as an attacker, still in combat.
        Filter::State(StateFilter::Attacking) => state.combat.is_attacking(id),
        // [CR#509.1a]: declared as a blocker (blocking some attacker).
        Filter::State(StateFilter::Blocking) => state.combat.attacker_of(id).is_some(),
        // [CR#509.1h]: attacking and not (stickily) blocked.
        Filter::State(StateFilter::Unblocked) => {
            state.combat.is_attacking(id) && !state.combat.is_blocked(id)
        }

        // [CR#115.9b]: "that targets [desc]" — `id` is a stack object one of
        // whose chosen targets CURRENTLY matches. A departed target (its id no
        // longer live) is ignored, never read through LKI. A non-stack object
        // has no targets.
        Filter::State(StateFilter::Targets(f)) => {
            state.stack.iter().find(|e| e.id == id).is_some_and(|e| {
                e.targets
                    .iter()
                    .any(|&t| state.objects.get(t).is_some() && matches_with(state, t, f, watcher))
            })
        }
        // [CR#115.9a]: "with [N] target(s)" — the count of target instances
        // chosen at stack-put. Bound is a literal (frameless). Non-stack → none.
        Filter::State(StateFilter::TargetCount(bound)) => {
            state.stack.iter().find(|e| e.id == id).is_some_and(|e| {
                bound.satisfied_by(
                    Uint::try_from(e.targets.len()).expect("target count fits Uint"),
                    const_count,
                )
            })
        }

        // [CR#301.5,303.4]: `id` is an attachment attached to a host matching
        // `inner` — read the attachment→host relation (engine-attach's
        // `attached_to`), then match the host, threading the watcher so a nested
        // `Ref` resolves against the carrier.
        Filter::Relation(RelationFilter::AttachedTo(inner)) => state
            .objects
            .obj(id)
            .attached_to
            .is_some_and(|host| matches_with(state, host, inner, watcher)),
        // The inverse ([CR#301.5,303.4]): `id` is a host with some attachment
        // matching `inner` (existential — `Attachment(Any)` = "has any
        // attachment").
        Filter::Relation(RelationFilter::Attachment(inner)) => state
            .objects
            .iter()
            .any(|o| o.attached_to == Some(id) && matches_with(state, o.id, inner, watcher)),
        // ----- Seams: backed by subsystems not yet built -----
        // [CR#607]: linked-ability relations have no registry yet.
        Filter::State(StateFilter::RelatedBy(..)) => todo!(
            "engine-filter-breadth: RelatedBy needs a CR#607 linked-ability relation registry \
             (unbuilt)"
        ),
        // Frame-needing references: the matcher carries only a `watcher`, not a
        // `Frame` with announced targets / trigger bindings. `This`/`You` are
        // handled above; the rest resolve only where a Frame exists
        // (`resolve::eval_reference`).
        Filter::Ref(r) => todo!(
            "engine-filter-breadth: Ref({r:?}) needs a carrier Frame (matcher holds only a watcher)"
        ),
    }
}

/// Whether a card object has card type `ty` in the DERIVED view ([CR#613.1d]
/// layer-4 type changes count — an animated land or crewed Vehicle types as a
/// Creature); a player proxy has none. Same per-call `layers()` perf seam as
/// `Has`/`Subtype`.
fn has_type(state: &GameState, id: ObjectId, ty: Type) -> bool {
    state.objects.obj(id).card_id().is_some() && state.layers().get(id).card_types.contains(&ty)
}

/// The DERIVED value of `stat` for the card-backed object `id`, or `None` when
/// the object lacks that stat (a land has no power). Mirrors `eval_count`'s
/// `StatOf` reads; the caller has already excluded player proxies.
fn derived_stat(
    state: &GameState,
    id: ObjectId,
    stat: deckmaste_core::Stat,
) -> Option<deckmaste_core::Int> {
    match stat {
        deckmaste_core::Stat::Power => state.layers().power(id),
        deckmaste_core::Stat::Toughness => state.layers().toughness(id),
        // [CR#202.3]: the printed cost's total (the on-stack announced-X
        // contribution rides the X announce slot, not yet wired).
        deckmaste_core::Stat::ManaValue => Some(
            deckmaste_core::Int::try_from(
                crate::derive::face(state.def(id)).mana_cost.mana_value(),
            )
            .expect("mana value fits Int"),
        ),
        deckmaste_core::Stat::Loyalty | deckmaste_core::Stat::Defense => todo!(
            "engine-filter-breadth: {stat:?} stat filter (planeswalker/battle counter machinery \
             unbuilt — mirrors eval_count)"
        ),
    }
}

/// Compare a stat `value` (possibly missing, possibly negative) against a
/// literal `Count` bound. A negative value clamps to 0 ([CR#107.1b], matching
/// `eval_count`); a missing stat (a land has no power) never satisfies the
/// predicate. Shared by the live matcher and `layer::matches_derived` so both
/// read the same comparison semantics.
#[must_use]
pub(crate) fn stat_satisfies(
    value: Option<deckmaste_core::Int>,
    cmp: deckmaste_core::Cmp,
    count: &deckmaste_core::Count,
) -> bool {
    match value {
        Some(v) => cmp.apply(
            Uint::try_from(v.max(0)).expect("clamped stat fits Uint"),
            const_count(count),
        ),
        None => false,
    }
}

/// A `Count` bound the frameless matcher can evaluate: only a literal (a
/// dynamic bound — `CountOf`, `StatOf`, `X`, … — needs a carrier `Frame` the
/// matcher does not hold). `Stat`/`TargetCount` predicates over dynamic bounds
/// are vanishingly rare; a loud seam beats a silently-wrong default.
fn const_count(count: &deckmaste_core::Count) -> Uint {
    match count {
        deckmaste_core::Count::Literal(n) => *n,
        deckmaste_core::Count::Expanded(e) => const_count(&e.value),
        other => todo!(
            "engine-filter-breadth: dynamic filter bound {other:?} needs a carrier frame \
             (only literal bounds evaluate in the frameless matcher)"
        ),
    }
}

/// Every object (card objects in their zones + player proxies) matching
/// `filter`, in deterministic id order. The frameless form — no carrier — for
/// the resolution-time selection sites (`Exists`, `Each`, `Selection`) whose
/// filters never carry a carrier-relative self-reference.
#[must_use]
pub fn candidates(state: &GameState, filter: &Filter) -> Vec<ObjectId> {
    candidates_with(state, filter, None)
}

/// Every object matching `filter`, with `watcher` anchoring carrier-relative
/// self-references (`Ref(This)`/`Ref(You)`, and the `StatOf(This, …)` reads a
/// `Filter::Where` reaches via [`GameState::condition_holds`]). The targeting
/// path passes the targeting object's `ObjectSource` so a target filter can
/// compare each candidate to the ability's source — "attacking creature with
/// power less than this creature's power" (Mentor, [CR#702.134a]) is a
/// `Where(Compare(StatOf(Subject, Power), Less, StatOf(This, Power)))` over
/// this carrier. Frameless callers pass `None` (the [`candidates`] shorthand).
#[must_use]
pub fn candidates_with(
    state: &GameState,
    filter: &Filter,
    watcher: Option<ObjectSource>,
) -> Vec<ObjectId> {
    state
        .objects
        .iter()
        .map(|o| o.id)
        .filter(|&id| matches_with(state, id, filter, watcher))
        .collect()
}

#[cfg(test)]
mod tests {
    use std::path::Path;
    use std::sync::Arc;

    use deckmaste_cards::plugin::Plugin;
    use deckmaste_core::Filter;
    use deckmaste_core::TargetSpec;
    use deckmaste_core::Zone;

    use super::*;
    use crate::player::PlayerId;
    use crate::state::GameConfig;
    use crate::state::GameState;
    use crate::state::PlayerConfig;
    use crate::state::StartingPlayer;

    fn builtin() -> Plugin {
        Plugin::load(Path::new(env!("CARGO_MANIFEST_DIR")).join("../../plugins/builtin")).unwrap()
    }

    fn canon() -> Plugin {
        Plugin::load_with_sibling_prelude(
            Path::new(env!("CARGO_MANIFEST_DIR")).join("../../plugins/canon"),
        )
        .unwrap()
    }

    /// A two-player game; player 0's deck is Grizzly Bears, player 1's is
    /// Forest. Returns the state plus a creature object moved onto the
    /// battlefield.
    fn game_with_a_bear_on_the_field() -> (GameState, ObjectId) {
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
            counter_decls: std::collections::HashMap::new(),
            subtypes: std::collections::HashMap::new(),
        });
        // Force a Grizzly Bears from player 0's hand onto the battlefield.
        let bear = *state.zones.hands[0]
            .iter()
            .find(|&&o| {
                matches(
                    &state,
                    o,
                    &Filter::Characteristic(deckmaste_core::CharacteristicFilter::Type(
                        deckmaste_core::Type::Creature,
                    )),
                )
            })
            .expect("a Grizzly Bears in the opening hand (10-card mono deck)");
        state.remove_from_hand(PlayerId(0), bear);
        state.objects.obj_mut(bear).zone = Some(Zone::Battlefield);
        state.zones.battlefield.push(bear);
        (state, bear)
    }

    /// A two-player game (P0 Grizzly Bears, P1 Forest) with one of P1's
    /// Forests forced onto the battlefield. Returns the state plus the land
    /// object.
    fn game_with_a_forest_on_the_field() -> (GameState, ObjectId) {
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
            starting_player: StartingPlayer::Fixed(PlayerId(1)),
            sba_rules: vec![],
            counter_decls: std::collections::HashMap::new(),
            subtypes: std::collections::HashMap::new(),
        });
        // Force a Forest from player 1's hand onto the battlefield.
        let land = *state.zones.hands[1]
            .iter()
            .find(|&&o| {
                matches(
                    &state,
                    o,
                    &Filter::Characteristic(deckmaste_core::CharacteristicFilter::Type(
                        deckmaste_core::Type::Land,
                    )),
                )
            })
            .expect("a Forest in the opening hand (10-card mono deck)");
        state.remove_from_hand(PlayerId(1), land);
        state.objects.obj_mut(land).zone = Some(Zone::Battlefield);
        state.zones.battlefield.push(land);
        (state, land)
    }

    /// Regression ([CR#613.1d]): `Filter::Type` reads the DERIVED type, not the
    /// printed face. A battlefield land animated into a creature by a layer-4
    /// continuous effect matches `Type(Creature)` — the view says Creature, so
    /// the matcher must agree. (Before the fix `has_type` read the printed face
    /// and reported the land was not a creature, mis-typing every `candidates`
    /// / targeting caller.)
    #[test]
    fn type_filter_reads_derived_type_for_animated_land() {
        use deckmaste_core::Duration;
        use deckmaste_core::Modification;

        use crate::layer::ContinuousEffect;
        use crate::layer::ScopeResolved;
        use crate::object::Timestamp;

        let (mut state, land) = game_with_a_forest_on_the_field();
        let creature = Filter::Characteristic(deckmaste_core::CharacteristicFilter::Type(
            deckmaste_core::Type::Creature,
        ));
        // Sanity: a plain Forest is not a creature.
        assert!(
            !matches(&state, land, &creature),
            "a plain Forest is not a creature"
        );

        // Animate it: a layer-4 effect adds the Creature type ([CR#613.1d]).
        state.continuous.push(ContinuousEffect {
            timestamp: Timestamp(1_000),
            controller: PlayerId(1),
            scope: ScopeResolved::Locked(vec![land]),
            changes: vec![Modification::CardTypes(deckmaste_core::CollectionOp::Add(
                deckmaste_core::Type::Creature,
            ))],
            duration: Duration::EndOfGame,
            is_cda: false,
        });

        // The derived view says Creature...
        assert!(
            state
                .layers()
                .get(land)
                .card_types
                .contains(&deckmaste_core::Type::Creature),
            "sanity: the animated land derives as a creature"
        );
        // ...and the Type filter must agree ([CR#613.1d]).
        assert!(
            matches(&state, land, &creature),
            "Filter::Type reads the derived type — the animated land matches Type(Creature)"
        );
    }

    #[test]
    fn any_target_is_creatures_and_players_not_lands() {
        // `read_str` returns the remembered `TargetSpec::Expanded(AnyTarget)`.
        // `resolve::target_spec_filter` is the engine's own TargetSpec→Filter
        // extraction — the path real targeting funnels through — so the test
        // exercises it rather than hand-unwrapping the expansion.
        let any_target: TargetSpec = builtin().macros.read_str("AnyTarget").unwrap();
        let filter = crate::resolve::target_spec_filter(&any_target);
        let (state, bear) = game_with_a_bear_on_the_field();
        let targets = candidates(&state, filter);
        // Both player proxies + the lone battlefield creature; no lands (in
        // hand/library), no spells (stack empty).
        assert!(targets.contains(&bear));
        assert!(targets.contains(&state.players[0].object));
        assert!(targets.contains(&state.players[1].object));
        assert_eq!(targets.len(), 3);
    }

    /// A filter-position macro (`kinds: [Filter]`) survives expansion as
    /// `Filter::Expanded`; `matches` must look through it transparently.
    /// Guards the delegation arm against being mistaken for dead code.
    #[test]
    fn matches_looks_through_a_filter_macro() {
        // `CreatureOrPlayer` reads as `Filter::Expanded(.., value: OneOf([..]))`:
        // the invocation survives, wrapping its expanded body.
        let wrapped: Filter = builtin().macros.read_str("CreatureOrPlayer").unwrap();
        assert!(
            matches!(wrapped, Filter::Expanded(_)),
            "a filter macro should survive as Filter::Expanded, got {wrapped:?}"
        );
        let (state, bear) = game_with_a_bear_on_the_field();
        // Evaluating the wrapped macro reaches the battlefield creature through
        // the remembered body — delegation is transparent.
        assert!(candidates(&state, &wrapped).contains(&bear));
    }

    // -------------------------------------------------------------------------
    // Characteristic arms: Named / ColorIs / Multicolored / Colorless /
    // Supertype / Stat
    // -------------------------------------------------------------------------

    use deckmaste_core::CharacteristicFilter as CF;

    fn cf(c: CF) -> Filter {
        Filter::Characteristic(c)
    }

    /// `Named` matches a card by its printed name ([CR#201]); a player proxy
    /// (no card) never matches.
    #[test]
    fn named_matches_card_name() {
        let (state, bear) = game_with_a_bear_on_the_field();
        assert!(matches(
            &state,
            bear,
            &cf(CF::Named("Grizzly Bears".into()))
        ));
        assert!(!matches(&state, bear, &cf(CF::Named("Forest".into()))));
        assert!(!matches(
            &state,
            state.players[0].object,
            &cf(CF::Named("Grizzly Bears".into())),
        ));
    }

    /// `ColorIs` reads the object's color ([CR#105.2,202.2]).
    #[test]
    fn color_is_matches_objects_color() {
        use deckmaste_core::Color;
        let (state, bear) = game_with_a_bear_on_the_field();
        assert!(matches(&state, bear, &cf(CF::ColorIs(Color::Green))));
        assert!(!matches(&state, bear, &cf(CF::ColorIs(Color::Red))));
    }

    /// `Colorless` matches an object with no colors ([CR#105.2c]); a green
    /// creature does not.
    #[test]
    fn colorless_matches_a_colorless_object() {
        let (lstate, land) = game_with_a_forest_on_the_field();
        assert!(matches(&lstate, land, &cf(CF::Colorless)));
        let (bstate, bear) = game_with_a_bear_on_the_field();
        assert!(!matches(&bstate, bear, &cf(CF::Colorless)));
    }

    /// `Multicolored` matches two-or-more colors ([CR#105.2b]) — read off the
    /// DERIVED colors (a layer-5 color add counts).
    #[test]
    fn multicolored_matches_a_two_color_object() {
        use deckmaste_core::Color;
        use deckmaste_core::Duration;
        use deckmaste_core::Modification;

        use crate::layer::ContinuousEffect;
        use crate::layer::ScopeResolved;
        use crate::object::Timestamp;

        let (mut state, bear) = game_with_a_bear_on_the_field();
        // Mono-green Grizzly Bears is not multicolored…
        assert!(!matches(&state, bear, &cf(CF::Multicolored)));
        // …add Red via a continuous effect → two colors.
        state.continuous.push(ContinuousEffect {
            timestamp: Timestamp(1_000),
            controller: PlayerId(0),
            scope: ScopeResolved::Locked(vec![bear]),
            changes: vec![Modification::Colors(deckmaste_core::CollectionOp::Add(
                Color::Red,
            ))],
            duration: Duration::EndOfGame,
            is_cda: false,
        });
        assert!(matches(&state, bear, &cf(CF::Multicolored)));
    }

    /// `Supertype` reads the derived supertype list — a Forest is Basic, a
    /// creature is not.
    #[test]
    fn supertype_matches_basic_land() {
        use deckmaste_core::Supertype;
        let (lstate, land) = game_with_a_forest_on_the_field();
        assert!(matches(&lstate, land, &cf(CF::Supertype(Supertype::Basic))));
        let (bstate, bear) = game_with_a_bear_on_the_field();
        assert!(!matches(
            &bstate,
            bear,
            &cf(CF::Supertype(Supertype::Basic))
        ));
    }

    /// `Stat` compares the DERIVED stat with a literal bound ([CR#208,202.3]).
    #[test]
    fn stat_compares_derived_stats() {
        use deckmaste_core::Cmp;
        use deckmaste_core::Count;
        use deckmaste_core::Stat;
        let (state, bear) = game_with_a_bear_on_the_field(); // 2/2, mana value {1}{G} = 2
        assert!(matches(
            &state,
            bear,
            &cf(CF::Stat(Stat::Power, Cmp::AtLeast, Count::Literal(2))),
        ));
        assert!(!matches(
            &state,
            bear,
            &cf(CF::Stat(Stat::Power, Cmp::Greater, Count::Literal(2))),
        ));
        assert!(matches(
            &state,
            bear,
            &cf(CF::Stat(Stat::Toughness, Cmp::Eq, Count::Literal(2))),
        ));
        assert!(matches(
            &state,
            bear,
            &cf(CF::Stat(Stat::ManaValue, Cmp::Eq, Count::Literal(2))),
        ));
        // A land has no power → Stat(Power, …) is false even for a ≥ 0 bound.
        let (lstate, land) = game_with_a_forest_on_the_field();
        assert!(!matches(
            &lstate,
            land,
            &cf(CF::Stat(Stat::Power, Cmp::AtLeast, Count::Literal(0))),
        ));
    }

    // -------------------------------------------------------------------------
    // State arms: Status / Attacking / Blocking / Unblocked
    // -------------------------------------------------------------------------

    /// `Status(Tapped)`/`Status(Untapped)` read the object's tap flag
    /// ([CR#110.5]).
    #[test]
    fn status_reads_tapped_flag() {
        use deckmaste_core::Status;
        let (mut state, bear) = game_with_a_bear_on_the_field();
        assert!(matches(
            &state,
            bear,
            &Filter::State(StateFilter::Status(Status::Untapped))
        ));
        assert!(!matches(
            &state,
            bear,
            &Filter::State(StateFilter::Status(Status::Tapped))
        ));
        state.objects.obj_mut(bear).tapped = true;
        assert!(matches(
            &state,
            bear,
            &Filter::State(StateFilter::Status(Status::Tapped))
        ));
        assert!(!matches(
            &state,
            bear,
            &Filter::State(StateFilter::Status(Status::Untapped))
        ));
    }

    /// `Attacking`/`Blocking`/`Unblocked` read live combat state
    /// ([CR#508.1a,509.1a,509.1h]).
    #[test]
    fn combat_filters_read_combat_state() {
        let (mut state, attacker) = game_with_a_bear_on_the_field();
        // A second creature to block with.
        let blocker = {
            let bears = Arc::new(canon().card("Grizzly Bears").unwrap());
            let cid = state.cards.push(bears, PlayerId(1));
            let bid = state.objects.mint(
                ObjectSource::Card(cid),
                PlayerId(1),
                Some(Zone::Battlefield),
            );
            state.zones.battlefield.push(bid);
            bid
        };

        // Pre-declaration: nothing is in combat.
        assert!(!matches(
            &state,
            attacker,
            &Filter::State(StateFilter::Attacking)
        ));
        assert!(!matches(
            &state,
            blocker,
            &Filter::State(StateFilter::Blocking)
        ));

        // Declare the attacker — attacking and (no blocker yet) unblocked.
        state.combat.declare_attacker(attacker);
        assert!(matches(
            &state,
            attacker,
            &Filter::State(StateFilter::Attacking)
        ));
        assert!(matches(
            &state,
            attacker,
            &Filter::State(StateFilter::Unblocked)
        ));
        assert!(!matches(
            &state,
            blocker,
            &Filter::State(StateFilter::Blocking)
        ));

        // Declare the block — blocker is blocking, attacker no longer unblocked.
        state.combat.declare_block(blocker, attacker);
        assert!(matches(
            &state,
            blocker,
            &Filter::State(StateFilter::Blocking)
        ));
        assert!(matches(
            &state,
            attacker,
            &Filter::State(StateFilter::Attacking)
        ));
        assert!(!matches(
            &state,
            attacker,
            &Filter::State(StateFilter::Unblocked)
        ));
    }

    // -------------------------------------------------------------------------
    // Relation arms: ControlledBy / OpponentOf / Owner / Controls
    //
    // Players are matched via a counter planted on their proxy, so each
    // relation can be tested without a watcher (the frameless path).
    // -------------------------------------------------------------------------

    /// A green creature owned & controlled by P0, with a `"mark"` counter on
    /// P0's proxy. Returns the state, the creature, and a Forest owned &
    /// controlled by P1 (the un-marked side).
    fn marked_p0_game() -> (GameState, ObjectId, ObjectId) {
        let (mut state, bear) = game_with_a_bear_on_the_field();
        let p0 = state.players[0].object;
        state.objects.obj_mut(p0).counters.insert("mark".into(), 1);
        let p1_card = {
            let forest = Arc::new(builtin().card("Forest").unwrap());
            let cid = state.cards.push(forest, PlayerId(1));
            state.objects.mint(
                ObjectSource::Card(cid),
                PlayerId(1),
                Some(Zone::Battlefield),
            )
        };
        (state, bear, p1_card)
    }

    fn marked() -> Filter {
        Filter::State(StateFilter::HasCounter("mark".into()))
    }

    /// `ControlledBy` matches when the object's controller's proxy matches
    /// ([CR#109.5]).
    #[test]
    fn controlled_by_matches_controllers_proxy() {
        let (state, bear, p1_card) = marked_p0_game();
        let f = Filter::Relation(RelationFilter::ControlledBy(Box::new(marked())));
        assert!(matches(&state, bear, &f)); // controlled by marked P0
        assert!(!matches(&state, p1_card, &f)); // controlled by un-marked P1
    }

    /// `OpponentOf` matches a player who is an opponent of a matching player
    /// ([CR#102.2,102.3]).
    #[test]
    fn opponent_of_matches_opponent_of_matching_player() {
        let (state, _bear, _p1_card) = marked_p0_game();
        let p0 = state.players[0].object;
        let p1 = state.players[1].object;
        let f = Filter::Relation(RelationFilter::OpponentOf(Box::new(marked())));
        assert!(matches(&state, p1, &f)); // P1 is an opponent of marked P0
        assert!(!matches(&state, p0, &f)); // P0's only opponent (P1) is un-marked
    }

    /// `Owner` matches when the object's owner's proxy matches ([CR#108.3]); a
    /// player proxy has no owner.
    #[test]
    fn owner_matches_owning_player() {
        let (state, bear, p1_card) = marked_p0_game();
        let f = Filter::Relation(RelationFilter::Owner(Box::new(marked())));
        assert!(matches(&state, bear, &f)); // owned by marked P0
        assert!(!matches(&state, p1_card, &f)); // owned by un-marked P1
        assert!(!matches(&state, state.players[0].object, &f)); // a player has no owner
    }

    /// `Controls` matches a player who controls a matching object — the inverse
    /// of `ControlledBy` ([CR#109.5]). A non-player never matches.
    #[test]
    fn controls_matches_player_controlling_a_match() {
        let (state, bear, _p1_card) = marked_p0_game();
        let p0 = state.players[0].object;
        let p1 = state.players[1].object;
        let f = Filter::Relation(RelationFilter::Controls(Box::new(cf(CF::Type(
            Type::Creature,
        )))));
        assert!(matches(&state, p0, &f)); // P0 controls the bear
        assert!(!matches(&state, p1, &f)); // P1 controls no creature
        assert!(!matches(&state, bear, &f)); // a creature is not a controlling player
    }

    /// `TeammateOf` matches ANOTHER player on the same team. Team membership is
    /// not modeled (singleton teams), so it matches nobody in a 1v1 game —
    /// neither the matched player itself nor the opponent ([CR#102.3,810.1]).
    /// (`OpponentOf`, the sibling, still matches the opponent.)
    #[test]
    fn teammate_of_matches_nobody_without_team_modeling() {
        let (state, _bear, _p1_card) = marked_p0_game();
        let p0 = state.players[0].object;
        let p1 = state.players[1].object;
        let teammate = Filter::Relation(RelationFilter::TeammateOf(Box::new(marked())));
        assert!(
            !matches(&state, p0, &teammate),
            "a player is never their own teammate"
        );
        assert!(
            !matches(&state, p1, &teammate),
            "no teammates exist in a 1v1 game (singleton teams)"
        );
        // The opponent relation still resolves (P1 is an opponent of marked P0).
        let opponent = Filter::Relation(RelationFilter::OpponentOf(Box::new(marked())));
        assert!(matches(&state, p1, &opponent));
    }

    /// An activated/triggered ability on the stack is `ObjectKind::Ability`,
    /// not `Spell`, even though its freshly minted stack id shares the
    /// source card's `ObjectSource` ([CR#602.2a,603.3]). A real spell
    /// stays a `Spell`, and the ability is a `Kind(Ability)` target
    /// candidate.
    #[test]
    fn ability_on_stack_is_kind_ability_not_spell() {
        use crate::stack::StackEntry;
        use crate::stack::StackObject;
        use crate::trigger::TriggerBindings;

        let (mut state, _bear) = game_with_a_bear_on_the_field();
        let cid = state
            .cards
            .push(Arc::new(builtin().card("Forest").unwrap()), PlayerId(0));

        // A triggered ability's stack token (shares the card source).
        let ability_id =
            state
                .objects
                .mint(ObjectSource::Card(cid), PlayerId(0), Some(Zone::Stack));
        state.stack.push(StackEntry {
            id: ability_id,
            object: StackObject::Triggered {
                source: ObjectSource::Card(cid),
                ability: 0,
                bindings: TriggerBindings {
                    this: None,
                    that_object: None,
                    that_player: None,
                    that_patient: None,
                    defending_player: None,
                },
            },
            controller: PlayerId(0),
            targets: vec![],
            x: None,
        });
        // A real spell on the stack, for contrast.
        let spell_id = state
            .objects
            .mint(ObjectSource::Card(cid), PlayerId(0), Some(Zone::Stack));
        state.stack.push(StackEntry {
            id: spell_id,
            object: StackObject::Spell(spell_id),
            controller: PlayerId(0),
            targets: vec![],
            x: None,
        });

        assert_eq!(object_kind(&state, ability_id), ObjectKind::Ability);
        assert!(matches(
            &state,
            ability_id,
            &Filter::Kind(ObjectKind::Ability)
        ));
        assert!(!matches(
            &state,
            ability_id,
            &Filter::Kind(ObjectKind::Spell)
        ));

        assert_eq!(object_kind(&state, spell_id), ObjectKind::Spell);
        assert!(matches(&state, spell_id, &Filter::Kind(ObjectKind::Spell)));
        assert!(!matches(
            &state,
            spell_id,
            &Filter::Kind(ObjectKind::Ability)
        ));

        // "counter target ability": the ability is a candidate, the spell isn't.
        let abilities = candidates(&state, &Filter::Kind(ObjectKind::Ability));
        assert!(abilities.contains(&ability_id));
        assert!(!abilities.contains(&spell_id));
    }

    // -------------------------------------------------------------------------
    // State arms: Targets / TargetCount (over a stack object's chosen targets)
    // -------------------------------------------------------------------------

    /// A spell on the stack targeting `bear`. Returns the state, the spell's
    /// stack id, and the targeted bear.
    fn spell_targeting_bear() -> (GameState, ObjectId, ObjectId) {
        use crate::stack::StackEntry;
        use crate::stack::StackObject;
        let (mut state, bear) = game_with_a_bear_on_the_field();
        let spell = {
            let card = Arc::new(builtin().card("Forest").unwrap());
            let cid = state.cards.push(card, PlayerId(0));
            state
                .objects
                .mint(ObjectSource::Card(cid), PlayerId(0), Some(Zone::Stack))
        };
        state.stack.push(StackEntry {
            id: spell,
            object: StackObject::Spell(spell),
            controller: PlayerId(0),
            targets: vec![bear],
            x: None,
        });
        (state, spell, bear)
    }

    /// `Targets(f)` matches a stack object one of whose targets currently
    /// matches `f` ([CR#115.9b]); a non-stack object has no targets.
    #[test]
    fn targets_reads_a_stack_objects_chosen_targets() {
        let (state, spell, bear) = spell_targeting_bear();
        let targets_creature =
            Filter::State(StateFilter::Targets(Box::new(cf(CF::Type(Type::Creature)))));
        let targets_land = Filter::State(StateFilter::Targets(Box::new(cf(CF::Type(Type::Land)))));
        assert!(matches(&state, spell, &targets_creature));
        assert!(!matches(&state, spell, &targets_land));
        // A non-stack object (the bear itself) has no targets.
        assert!(!matches(&state, bear, &targets_creature));
    }

    /// `Targets` ignores a target that has since left ([CR#115.9b] — departed
    /// targets are not read through LKI).
    #[test]
    fn targets_ignores_a_departed_target() {
        let (mut state, spell, bear) = spell_targeting_bear();
        let targets_creature =
            Filter::State(StateFilter::Targets(Box::new(cf(CF::Type(Type::Creature)))));
        assert!(matches(&state, spell, &targets_creature));
        // The bear leaves — its id is now stale on the stack entry.
        state.objects.remove(bear);
        assert!(!matches(&state, spell, &targets_creature));
    }

    /// `TargetCount(bound)` counts the target instances chosen at stack-put
    /// ([CR#115.9a]); a non-stack object has none.
    #[test]
    fn target_count_reads_chosen_target_arity() {
        use deckmaste_core::Count;
        use deckmaste_core::CountBound;
        let (state, spell, bear) = spell_targeting_bear();
        assert!(matches(
            &state,
            spell,
            &Filter::State(StateFilter::TargetCount(CountBound::Eq(Count::Literal(1)))),
        ));
        assert!(!matches(
            &state,
            spell,
            &Filter::State(StateFilter::TargetCount(CountBound::Eq(Count::Literal(2)))),
        ));
        assert!(matches(
            &state,
            spell,
            &Filter::State(StateFilter::TargetCount(CountBound::AtLeast(
                Count::Literal(1)
            ))),
        ));
        // A non-stack object has no targets → never satisfies a ≥ 1 bound.
        assert!(!matches(
            &state,
            bear,
            &Filter::State(StateFilter::TargetCount(CountBound::AtLeast(
                Count::Literal(1)
            ))),
        ));
    }

    // -------------------------------------------------------------------------
    // Relation arms: AttachedTo / Attachment (engine-attach's attachment store)
    // -------------------------------------------------------------------------

    /// [CR#301.5,303.4]: with `a` attached to host `b` (a creature),
    /// `AttachedTo(Creature)` admits `a`, and `Attachment(Any)` admits `b`; the
    /// reverse pairings do not match.
    #[test]
    fn relation_filters_match_attachment_and_host() {
        let (mut state, b) = game_with_a_bear_on_the_field();
        // A second Grizzly Bears on the field plays the attachment `a`.
        let a = *state.zones.hands[0]
            .iter()
            .find(|&&o| matches(&state, o, &cf(CF::Type(Type::Creature))))
            .expect("a second Grizzly Bears in the opening hand");
        state.remove_from_hand(PlayerId(0), a);
        state.objects.obj_mut(a).zone = Some(Zone::Battlefield);
        state.zones.battlefield.push(a);
        state.objects.obj_mut(a).attached_to = Some(b);

        let attached_to_creature = Filter::Relation(RelationFilter::AttachedTo(Box::new(cf(
            CF::Type(Type::Creature),
        ))));
        let has_any_attachment =
            Filter::Relation(RelationFilter::Attachment(Box::new(Filter::Any)));

        assert!(
            matches(&state, a, &attached_to_creature),
            "a is attached to a creature"
        );
        assert!(
            !matches(&state, b, &attached_to_creature),
            "b (the host) is not itself attached to anything"
        );
        assert!(
            matches(&state, b, &has_any_attachment),
            "b has an attachment (a)"
        );
        assert!(
            !matches(&state, a, &has_any_attachment),
            "a has nothing attached to it"
        );
    }

    /// A two-player game with a green Grizzly Bears (P0) and a black Diregraf
    /// Ghoul (P1) both forced onto the battlefield — distinct colors, for the
    /// `Subject` / `Where` shares-color tests.
    fn game_with_bear_and_ghoul() -> (GameState, ObjectId, ObjectId) {
        let bears = Arc::new(canon().card("Grizzly Bears").unwrap());
        let ghouls = Arc::new(canon().card("Diregraf Ghoul").unwrap());
        let mut state = GameState::new(GameConfig {
            players: vec![
                PlayerConfig {
                    deck: vec![Arc::clone(&bears); 10],
                },
                PlayerConfig {
                    deck: vec![Arc::clone(&ghouls); 10],
                },
            ],
            seed: 1,
            starting_life: 20,
            starting_player: StartingPlayer::Fixed(PlayerId(0)),
            sba_rules: vec![],
            counter_decls: std::collections::HashMap::new(),
            subtypes: std::collections::HashMap::new(),
        });
        let creature = Filter::Characteristic(deckmaste_core::CharacteristicFilter::Type(
            deckmaste_core::Type::Creature,
        ));
        let bear = *state.zones.hands[0]
            .iter()
            .find(|&&o| matches(&state, o, &creature))
            .expect("a Grizzly Bears in P0's opening hand");
        state.remove_from_hand(PlayerId(0), bear);
        state.objects.obj_mut(bear).zone = Some(Zone::Battlefield);
        state.zones.battlefield.push(bear);
        let ghoul = *state.zones.hands[1]
            .iter()
            .find(|&&o| matches(&state, o, &creature))
            .expect("a Diregraf Ghoul in P1's opening hand");
        state.remove_from_hand(PlayerId(1), ghoul);
        state.objects.obj_mut(ghoul).zone = Some(Zone::Battlefield);
        state.zones.battlefield.push(ghoul);
        (state, bear, ghoul)
    }

    fn where_is_subject_color(c: deckmaste_core::Color) -> Filter {
        Filter::Where(Box::new(deckmaste_core::Condition::Is(
            deckmaste_core::Reference::It,
            Filter::Characteristic(deckmaste_core::CharacteristicFilter::ColorIs(c)),
        )))
    }

    /// `Where(SharesColor(Subject, This))` spelled out: the candidate shares a
    /// color with the carrier (`This`).
    fn where_shares_color_with_carrier() -> Filter {
        use deckmaste_core::CharacteristicFilter::ColorIs;
        use deckmaste_core::Color::Black;
        use deckmaste_core::Color::Blue;
        use deckmaste_core::Color::Green;
        use deckmaste_core::Color::Red;
        use deckmaste_core::Color::White;
        use deckmaste_core::Condition;
        use deckmaste_core::Reference;
        let branch = |c| {
            Condition::AllOf(vec![
                Condition::Is(Reference::It, Filter::Characteristic(ColorIs(c))),
                Condition::Is(Reference::This, Filter::Characteristic(ColorIs(c))),
            ])
        };
        Filter::Where(Box::new(Condition::OneOf(vec![
            branch(White),
            branch(Blue),
            branch(Black),
            branch(Red),
            branch(Green),
        ])))
    }

    /// `Subject` inside a `Where` condition resolves to the candidate being
    /// matched: a green Grizzly Bears satisfies `Where(Is(Subject,
    /// ColorIs(Green)))` but not `ColorIs(White)`.
    #[test]
    fn where_binds_subject_to_the_candidate() {
        let (state, bear, _ghoul) = game_with_bear_and_ghoul();
        let carrier = Some(state.objects.obj(bear).source);
        assert!(matches_with(
            &state,
            bear,
            &where_is_subject_color(deckmaste_core::Color::Green),
            carrier
        ));
        assert!(!matches_with(
            &state,
            bear,
            &where_is_subject_color(deckmaste_core::Color::White),
            carrier
        ));
    }

    /// `SharesColor(Subject, This)` compares the candidate to the carrier: a
    /// green candidate shares a color with a green carrier; a black one does
    /// not. `Subject` is the candidate, `This` the carrier (`watcher`).
    #[test]
    fn where_shares_color_compares_candidate_to_carrier() {
        let (state, bear, ghoul) = game_with_bear_and_ghoul();
        let carrier = Some(state.objects.obj(bear).source);
        assert!(matches_with(
            &state,
            bear,
            &where_shares_color_with_carrier(),
            carrier
        ));
        assert!(!matches_with(
            &state,
            ghoul,
            &where_shares_color_with_carrier(),
            carrier
        ));
    }

    // -------------------------------------------------------------------------
    // The Mentor target filter ([CR#702.134a]): a cross-object stat comparison
    // in a TARGET filter — "attacking creature with power less than this
    // creature's power" — evaluated by the watcher-threaded `candidates_with`.
    // -------------------------------------------------------------------------

    /// Force a fresh battlefield creature of card `name` (from canon) under
    /// player 0, returning its object id.
    fn put_canon_creature(state: &mut GameState, name: &str) -> ObjectId {
        let card = Arc::new(canon().card(name).unwrap());
        let cid = state.cards.push(card, PlayerId(0));
        let id = state.objects.mint(
            ObjectSource::Card(cid),
            PlayerId(0),
            Some(Zone::Battlefield),
        );
        state.zones.battlefield.push(id);
        id
    }

    /// A board with three creatures controlled by P0: a Centaur Courser (3/3,
    /// the Mentor carrier), a Grizzly Bears (2/2, lesser power), and a Fangren
    /// Hunter (4/4, greater power). All three are declared as attackers so
    /// `Attacking` admits each.
    fn mentor_board() -> (GameState, ObjectId, ObjectId, ObjectId) {
        let courser = Arc::new(canon().card("Centaur Courser").unwrap());
        let mut state = GameState::new(GameConfig {
            players: vec![
                PlayerConfig {
                    deck: vec![Arc::clone(&courser); 10],
                },
                PlayerConfig {
                    deck: vec![Arc::clone(&courser); 10],
                },
            ],
            seed: 1,
            starting_life: 20,
            starting_player: StartingPlayer::Fixed(PlayerId(0)),
            sba_rules: vec![],
            counter_decls: std::collections::HashMap::new(),
            subtypes: std::collections::HashMap::new(),
        });
        let courser = put_canon_creature(&mut state, "Centaur Courser"); // 3/3 carrier
        let bears = put_canon_creature(&mut state, "Grizzly Bears"); // 2/2 lesser
        let hunter = put_canon_creature(&mut state, "Fangren Hunter"); // 4/4 greater
        for &id in &[courser, bears, hunter] {
            state.combat.declare_attacker(id);
        }
        (state, courser, bears, hunter)
    }

    /// The Mentor target filter: an attacking creature whose power is less than
    /// the carrier's power ([CR#702.134a]) — the `Where`/`Subject`/`This`
    /// cross-object stat comparison the keyword macro emits.
    fn mentor_target_filter() -> Filter {
        use deckmaste_core::Cmp;
        use deckmaste_core::Condition;
        use deckmaste_core::Count;
        use deckmaste_core::Reference;
        use deckmaste_core::Stat;
        Filter::AllOf(vec![
            Filter::State(StateFilter::Attacking),
            Filter::Where(Box::new(Condition::Compare(
                Count::StatOf(Reference::It, Stat::Power),
                Cmp::Less,
                Count::StatOf(Reference::This, Stat::Power),
            ))),
        ])
    }

    /// `candidates_with(.., Some(carrier))` over the Mentor filter admits ONLY
    /// the lesser-power attacker: the 2/2 Bears (2 < 3), never the 4/4 Hunter
    /// (4 < 3 is false) nor the 3/3 carrier itself (3 < 3 is false). This is
    /// the load-bearing target-path fix — the carrier (`This`) resolves
    /// through the threaded watcher, so the dynamic `StatOf(This, Power)`
    /// bound evaluates instead of tripping the frameless `todo!`.
    #[test]
    fn mentor_filter_targets_only_lesser_power_attacker() {
        let (state, courser, bears, hunter) = mentor_board();
        let carrier = Some(state.objects.obj(courser).source);
        let admitted = candidates_with(&state, &mentor_target_filter(), carrier);
        assert!(
            admitted.contains(&bears),
            "the 2/2 Bears has power less than the 3/3 carrier — a legal Mentor target"
        );
        assert!(
            !admitted.contains(&hunter),
            "the 4/4 Hunter does NOT have lesser power — not a Mentor target"
        );
        assert!(
            !admitted.contains(&courser),
            "the carrier itself is not lesser-power than itself ([CR#702.134a])"
        );
        // Only the bears (the two player proxies have no power → the inner
        // StatOf(Subject,Power) read fails their match, never the carrier's).
        assert_eq!(admitted, vec![bears], "exactly the lesser-power attacker");
    }

    /// A per-candidate sanity slice of the same comparison via `matches_with`:
    /// the carrier anchors `This`, `Subject` is each candidate.
    #[test]
    fn mentor_filter_matches_with_carrier() {
        let (state, courser, bears, hunter) = mentor_board();
        let carrier = Some(state.objects.obj(courser).source);
        let f = mentor_target_filter();
        assert!(matches_with(&state, bears, &f, carrier));
        assert!(!matches_with(&state, hunter, &f, carrier));
        assert!(!matches_with(&state, courser, &f, carrier));
    }
}
