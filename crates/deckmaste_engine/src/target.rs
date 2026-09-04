//! Targeting ([CR#115]): which objects a `Predicate` admits. Stage 2 wires only
//! the arms the corpus's `AnyTarget` reaches; the rest are `todo!`.

use deckmaste_core::CharacteristicPredicate;
use deckmaste_core::EntityClass;
use deckmaste_core::Ident;
use deckmaste_core::ObjectClass;
use deckmaste_core::Predicate;
use deckmaste_core::Reference;
use deckmaste_core::RelationPredicate;
use deckmaste_core::StatePredicate;
use deckmaste_core::Uint;
use deckmaste_core::Zone;

use crate::object::ObjectId;
use crate::object::ObjectSource;
use crate::stack::StackObject;
use crate::state::GameState;

/// Which Entity `id` is ([CR#109.1,102.1]): the engine addresses a player
/// through a proxy carrying an `ObjectId`, and that proxy is the adapter — a
/// player is an Entity, never an object.
#[must_use]
pub fn entity_class(state: &GameState, id: ObjectId) -> EntityClass {
    match state.objects.obj(id).source {
        ObjectSource::Player(_) => EntityClass::Player,
        ObjectSource::Card(_) => EntityClass::Object,
    }
}

/// Whether `id` is an activated or triggered ability on the stack
/// ([CR#602.2a,603.3]).
///
/// The stack is the only place abilityhood is knowable: an ability on the
/// stack carries a freshly minted `StackEntry.id` that *shares the source's
/// `ObjectSource`* (the card or player it came from), so every source-based
/// test would misread it as a spell or a player. That also covers an ability
/// minted from a token source — the ability on the stack, not the token.
fn is_ability_on_stack(state: &GameState, id: ObjectId) -> bool {
    state.stack.iter().any(|e| {
        e.id == id
            && matches!(
                e.object,
                StackObject::Triggered { .. } | StackObject::Activated { .. }
            )
    })
}

/// Whether `id` is a card-less copy ([CR#707.10]) — the engine's
/// `StackEntry.copy` marker. A copy TOKEN is not one: its copiable values are
/// baked into its definition at mint ([CR#707.1]) and it ceases under
/// [CR#111.7] rather than [CR#707.10a]'s copy-cease SBA. The marker is the
/// only per-object, non-aliased signal available, because a card-less copy's
/// `ObjectSource::Card` aliases the ORIGINAL's `CardId`.
fn is_cardless_copy(state: &GameState, id: ObjectId) -> bool {
    matches!(state.objects.obj(id).source, ObjectSource::Card(_))
        && state.stack.iter().any(|e| e.id == id && e.copy)
}

/// Whether `id` is in the CR object class `class` ([CR#109.1]). The classes
/// are independently testable and overlap: a card on the stack answers `true`
/// to both `Card` and `Spell`, and a battlefield token to both `Token` and
/// `Permanent`. A player answers `false` to every class.
#[must_use]
pub fn is_object_class(state: &GameState, id: ObjectId, class: ObjectClass) -> bool {
    if entity_class(state, id) == EntityClass::Player {
        return false;
    }
    let ability = is_ability_on_stack(state, id);
    if class == ObjectClass::AbilityOnStack {
        return ability;
    }
    // An ability on the stack has no card and no characteristics of its own,
    // so it is in no other class ([CR#405.1,113.7a]).
    if ability {
        return false;
    }
    let obj = state.objects.obj(id);
    let ObjectSource::Card(card) = obj.source else {
        return false;
    };
    let is_emblem = state.cards.get(card).is_emblem;
    let is_token = state.cards.get(card).is_token;
    let copy = is_cardless_copy(state, id);
    match class {
        ObjectClass::AbilityOnStack => unreachable!("answered above"),
        // [CR#108.2]: represented by a Magic card. A token is not a card
        // ([CR#108.2b]), an emblem is neither a card nor a permanent
        // ([CR#114.5]), and a card-less copy has no card ([CR#707.10]).
        ObjectClass::Card => !is_token && !is_emblem && !copy,
        ObjectClass::CopyOfACard => copy,
        ObjectClass::Emblem => is_emblem,
        // [CR#110.1]: a card or token on the battlefield.
        ObjectClass::Permanent => obj.zone == Some(Zone::Battlefield) && !is_emblem,
        // [CR#112.1]: a card on the stack — and a copy of a spell is itself a
        // spell even with no card ([CR#112.1a,707.10]).
        ObjectClass::Spell => obj.zone == Some(Zone::Stack),
        ObjectClass::Token => is_token,
    }
}

/// Whether `id` matches `filter`, with no carrier context — the targeting /
/// `candidates` path ([CR#115]). A thin wrapper over [`matches_with`] with no
/// watcher; filters needing a carrier (`Ref(This)`/`Ref(You)`) are unreachable
/// here (targeting threads no frame) and `todo!`.
#[must_use]
pub fn matches(state: &GameState, id: ObjectId, filter: &Predicate) -> bool {
    matches_with(state, id, filter, None)
}

/// The live object standing for `id`'s SOURCE: an activated/triggered
/// ability on the stack reads the object that generated it ([CR#113.7]);
/// anything else — a spell is itself a source ([CR#609.7a] lists "a spell on
/// the stack" among sources) — reads as itself. `None` when an ability's
/// source has no live object left ([CR#113.7a] the ability exists
/// independently of its source) — the LKI read there is an engine-breadth
/// seam.
pub(crate) fn source_of(state: &GameState, id: ObjectId) -> Option<ObjectId> {
    if is_ability_on_stack(state, id) {
        let source = state.objects.obj(id).source;
        state
            .objects
            .iter()
            .find(|ob| ob.source == source && ob.id != id)
            .map(|ob| ob.id)
    } else {
        Some(id)
    }
}

/// The combinator arms shared by every `Predicate` matcher: the logical
/// `And`/`Or`/`Not` and the `Any` wildcard. Returns `Some(result)` for one of
/// those arms — recursing each sub-filter through `eval`, the caller's own
/// leaf-aware matcher (which re-enters this walker for nested combinators) —
/// and `None` for any other (leaf) predicate, which the caller evaluates
/// itself. This owns the combinator recursion in one place so the live
/// ([`matches_with`]), snapshot (`GameState::filter_matches_snapshot`), and
/// derived-view (`layer::matches_derived`) matchers share it instead of
/// hand-copying it; the leaves legitimately differ (live / snapshot / derived)
/// and stay per-caller.
pub(crate) fn walk_combinators<F>(filter: &Predicate, eval: F) -> Option<bool>
where
    F: Fn(&Predicate) -> bool,
{
    match filter {
        Predicate::And(fs) => Some(fs.iter().all(&eval)),
        Predicate::Or(fs) => Some(fs.iter().any(&eval)),
        Predicate::Not(f) => Some(!eval(f)),
        Predicate::Any => Some(true),
        _ => None,
    }
}

/// Whether a damage mark's captured DEAL-TIME abilities satisfy `filter`
/// ([CR#120.3,702.2c]). The mark stores abilities, not an object, so only the
/// combinators and `Has(name)` can be answered — every other atom would need
/// the live source, which the deal-time reading deliberately does not consult.
/// Shared by both core spellings of the relation: the `WasDealtDamageBy` state
/// predicate below and `Condition::DealtDamageBy`'s evaluator.
pub(crate) fn source_abilities_match(
    filter: &Predicate,
    abilities: &[deckmaste_core::Ability],
) -> bool {
    use deckmaste_core::CharacteristicPredicate;
    if let Some(result) = walk_combinators(filter, |f| source_abilities_match(f, abilities)) {
        return result;
    }
    match filter {
        Predicate::Characteristic(CharacteristicPredicate::Has(name)) => abilities
            .iter()
            .any(|a| crate::layer::ability_is_named(a, &name.0)),
        _ => false,
    }
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
pub fn matches_with(
    state: &GameState,
    id: ObjectId,
    filter: &Predicate,
    watcher: Option<ObjectSource>,
) -> bool {
    matches_with_activation(state, id, filter, watcher, crate::ActivationId::NONE)
}

/// Region-aware form of [`matches_with`]. Resolution-time callers pass the
/// active register file; structural/static callers use the compatibility
/// wrapper above and therefore cannot observe region-local references.
#[must_use]
#[expect(clippy::too_many_lines, reason = "flat per-variant Predicate dispatch")]
pub(crate) fn matches_with_activation(
    state: &GameState,
    id: ObjectId,
    filter: &Predicate,
    watcher: Option<ObjectSource>,
    activation: crate::ActivationId,
) -> bool {
    // Combinators (`And`/`Or`/`Not`/`Any`) recurse through this same matcher
    // via the shared walker; leaves fall through to the match.
    if let Some(result) = walk_combinators(filter, |f| {
        matches_with_activation(state, id, f, watcher, activation)
    }) {
        return result;
    }
    match filter {
        Predicate::Entity(class) => entity_class(state, id) == *class,
        Predicate::Class(class) => is_object_class(state, id, *class),
        Predicate::Characteristic(CharacteristicPredicate::Type(t)) => {
            has_type(state, id, t.name())
        }
        // [CR#110.5a]: state, not characteristic — card/token objects only, so a
        // player proxy (zone None) never matches InZone.
        Predicate::State(StatePredicate::InZone(z)) => state.objects.obj(id).zone == Some(*z),
        // Combinators are handled by `walk_combinators` before this match.
        Predicate::And(_) | Predicate::Or(_) | Predicate::Not(_) | Predicate::Any => {
            unreachable!("combinator filters are handled by walk_combinators before the match")
        }
        // NOT a combinator, and not walked: provenance is erased at `lower`
        // (`deckmaste_lowering`), so no loaded value reaches here wrapped. The
        // arm survives only because the variant does; `core-demacro` deletes
        // both.
        // [CR#702.11d] "abilities … from [quality] sources": strict to stack
        // ABILITIES by construction — the candidate is an activated/triggered
        // ability on the stack whose SOURCE (the generating object,
        // [CR#113.7]) matches the inner filter.
        Predicate::FromSource(inner) => {
            is_ability_on_stack(state, id)
                && source_of(state, id).is_some_and(|src| {
                    matches_with_activation(state, src, inner, watcher, activation)
                })
        }
        // The candidate matches iff the condition holds with the iteration
        // anaphor `It` bound to it. `This`/`You` still anchor to the carrier, so
        // build a match frame from the watcher (source → live carrier id +
        // controller) and set `it` to the candidate. Re-binding across nested
        // relation filters is automatic: those arms recurse with a new `id`, so
        // a nested `Where` sees the related object as `It`. A frameless caller
        // (no watcher) or a gone carrier leaves `This` unresolvable — no match.
        Predicate::Where(cond) => match watcher {
            None => todo!(
                "engine seam: Predicate::Where at a frameless position — the matcher holds no \
                 carrier for This/You; owner: engine-frameless-carrier-threading"
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
                        let mut frame = state.frame(carrier, controller);
                        if activation != crate::ActivationId::NONE {
                            frame.activation = activation;
                        }
                        frame.activation = state.enter_candidate_region(cond, &frame, id);
                        state.condition_holds(&cond.body, &frame)
                    }
                }
            }
        },
        // [CR#702]: keyword presence by NAME against the DERIVED abilities
        // (granted keywords count; the carried Composite name survives
        // expansion). Per-call layers() rebuild — a perf seam if a hot path
        // ever evaluates Has in bulk.
        Predicate::Characteristic(CharacteristicPredicate::Has(kw)) => {
            let view = state.layers();
            view.get(id)
                .abilities
                .iter()
                .any(|a| crate::layer::ability_is_named(a, &kw.0))
        }
        // Subtype presence by NAME against the DERIVED subtype list
        // ([CR#205.3] — layer-4 type changes count); a player proxy has
        // none. Same per-call layers() perf seam as `Has`.
        Predicate::Characteristic(CharacteristicPredicate::Subtype(name)) => {
            state.objects.obj(id).card_id().is_some()
                && state
                    .layers()
                    .get(id)
                    .subtypes
                    .iter()
                    .any(|s| s.name == name.name())
        }
        // [CR#122.1] counters go on objects AND players — player counters
        // live on the player's proxy object, so one LIVE read serves both.
        Predicate::State(StatePredicate::HasCounter(kind)) => state
            .objects
            .obj(id)
            .counters
            .get(kind.as_str())
            .is_some_and(|&n| n > 0),
        // [CR#109.3] designations are non-characteristic state: a LIVE read
        // against the registry (object entry, or the player's for proxies).
        Predicate::State(StatePredicate::Designated(name)) => {
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

        // "the host of THIS attachment" ([CR#301.5,303.4]): `id` is the
        // permanent that THIS (the watcher) is attached to. Used by umbra/totem
        // armor to express "the enchanted permanent" as the watched subject of
        // a static other-watching replacement ([CR#702.89a]).
        //
        // Only `inner = This` is resolvable from the carrier alone (no ExecutionFrame).
        // The general `AttachHostOf(inner)` case needs `eval_reference` and is
        // therefore left as a seam.
        Predicate::Ref(Reference::AttachHostOf(inner))
            if matches!(inner.as_ref(), Reference::Reg(reference)
            if state.activation_reference_is(
                activation,
                *reference,
                &deckmaste_core::Provenance::Source,
            )) =>
        {
            match watcher {
                Some(w) => {
                    // Find the live object whose source is the watcher (the
                    // Aura) and check whether it is
                    // attached to `id` (the would-be
                    // destroyed creature). If the watcher has no live object on
                    // the battlefield (e.g. it left already), the match fails.
                    state
                        .objects
                        .iter()
                        .find(|o| o.source == w)
                        .is_some_and(|o| o.attached_to == Some(id))
                }
                None => todo!(
                    "engine seam: Ref(AttachHostOf(This)) at a frameless position \
                     ([CR#701.3]) — targeting threads no carrier; \
                     owner: engine-frameless-carrier-threading"
                ),
            }
        }

        // "named X" ([CR#201]): printed face name; a player proxy has no card.
        Predicate::Characteristic(CharacteristicPredicate::Named(name)) => {
            state.objects.obj(id).card_id().is_some()
                && &*crate::derive::face(state.def(id)).characteristics.name == name.as_str()
        }
        Predicate::Characteristic(CharacteristicPredicate::NamedReg(reference)) => {
            state.objects.obj(id).card_id().is_some()
                && state
                    .activation_symbol(activation, *reference)
                    .is_some_and(|expected| {
                        crate::derive::face(state.def(id))
                            .characteristics
                            .name
                            .as_ref()
                            == expected
                    })
        }
        // Color predicates over the DERIVED colors ([CR#105.2,202.2]) — a
        // layer-5 color change counts. Same per-call layers() perf seam as
        // `Has`. A player proxy is not a colored object.
        Predicate::Characteristic(CharacteristicPredicate::ColorIs(c)) => {
            state.objects.obj(id).card_id().is_some() && state.layers().get(id).colors.contains(c)
        }
        // [CR#105.2b]: two or more colors.
        Predicate::Characteristic(CharacteristicPredicate::Multicolored) => {
            state.objects.obj(id).card_id().is_some() && state.layers().get(id).colors.len() >= 2
        }
        // [CR#105.2c]: colorless is the ABSENCE of color, not a color itself.
        Predicate::Characteristic(CharacteristicPredicate::Colorless) => {
            state.objects.obj(id).card_id().is_some() && state.layers().get(id).colors.is_empty()
        }
        // Supertype presence over the DERIVED list ([CR#205.4a]); player proxy
        // has none.
        Predicate::Characteristic(CharacteristicPredicate::Supertype(s)) => {
            state.objects.obj(id).card_id().is_some()
                && state.layers().get(id).supertypes.contains(s)
        }
        // [CR#208,202.3]: a derived stat compares against a bound. A literal
        // bound evaluates directly; a dynamic bound (`CountOf`, `StatOf`, `X`)
        // resolves through `resolve_count`'s watcher-anchored `eval_count` —
        // reachable: Skulk's blocker filter is `Stat(Power, Greater,
        // StatOf(This, Power))` ([CR#702.118b]). A missing stat (a land has no
        // power) never satisfies the predicate.
        Predicate::Characteristic(CharacteristicPredicate::Stat(stat, cmp, count)) => {
            state.objects.obj(id).card_id().is_some()
                && stat_satisfies_bound(
                    derived_stat(state, id, *stat),
                    *cmp,
                    resolve_count(state, count, watcher, activation),
                )
        }

        // The object's controller, as a player proxy ([CR#109.5]). Recurses
        // with the SAME watcher so a nested `Ref(You)` still anchors right.
        Predicate::Relation(RelationPredicate::ControlledBy(f)) => {
            let c = state.objects.obj(id).controller;
            let proxy = state.player(c).object;
            matches_with_activation(state, proxy, f, watcher, activation)
        }
        // `id` is a player who is an opponent of a matching player
        // ([CR#102.2,102.3,810.1]): on a DIFFERENT team. Routed through
        // `same_team` so it stays sound when team play (Two-Headed Giant) is
        // modeled — today, with singleton teams, "different team" == "different
        // player", the prior behavior.
        Predicate::Relation(RelationPredicate::OpponentOf(f)) => {
            match state.objects.obj(id).source {
                ObjectSource::Player(p) => state.players.iter().any(|q| {
                    !state.same_team(p, q.id)
                        && matches_with_activation(state, q.object, f, watcher, activation)
                }),
                ObjectSource::Card(_) => false,
            }
        }
        // `id` is a player who is a teammate of a matching player
        // ([CR#102.3,810.1]): ANOTHER player (never `q` itself) on the SAME
        // team. Team membership isn't modeled yet (singleton teams), so this
        // matches nobody outside a team game — the correct answer for 1v1 and
        // free-for-all, where no two players share a team. See
        // `GameState::same_team`.
        Predicate::Relation(RelationPredicate::TeammateOf(f)) => match state.objects.obj(id).source
        {
            ObjectSource::Player(p) => state.players.iter().any(|q| {
                q.id != p
                    && state.same_team(p, q.id)
                    && matches_with_activation(state, q.object, f, watcher, activation)
            }),
            ObjectSource::Card(_) => false,
        },
        // The object's owner, as a player proxy ([CR#108.3]); a player proxy
        // has no owner.
        Predicate::Relation(RelationPredicate::Owner(f)) => {
            state.objects.obj(id).card_id().is_some()
                && matches_with_activation(
                    state,
                    state.player(state.owner_of(id)).object,
                    f,
                    watcher,
                    activation,
                )
        }
        // `id` is a player who controls a matching object — the inverse of
        // `ControlledBy` ([CR#109.5]). Zone-agnostic: the inner filter carries
        // any zone restriction (proxies, zone `None`, fall out of e.g.
        // `Permanent`). A card is never a controlling player.
        Predicate::Relation(RelationPredicate::Controls(f)) => match state.objects.obj(id).source {
            ObjectSource::Player(p) => state.objects.iter().any(|ob| {
                ob.controller == p && matches_with_activation(state, ob.id, f, watcher, activation)
            }),
            ObjectSource::Card(_) => false,
        },

        // [CR#110.5]: status. Tap state is stored; flip/face/phasing are not —
        // a filter over one must trip rather than silently read a default,
        // since the answer changes targeting legality. A player proxy has no
        // status.
        Predicate::State(StatePredicate::Status(status)) => {
            use deckmaste_core::Status;
            if state.objects.obj(id).card_id().is_none() {
                return false;
            }
            let tapped = state.objects.obj(id).tapped;
            match status {
                Status::Tapped => tapped,
                Status::Untapped => !tapped,
                Status::PhasedOut | Status::PhasedIn => todo!(
                    "engine seam: permanent status {status:?} ([CR#702.26a]) — phasing state \
                     unstored; owner: engine-phasing"
                ),
                Status::FaceDown | Status::FaceUp => todo!(
                    "engine seam: permanent status {status:?} ([CR#708.1]) — face-down state \
                     unstored; owner: engine-face-down"
                ),
                Status::Flipped | Status::Unflipped => todo!(
                    "engine seam: permanent status {status:?} ([CR#710.3]) — flipped state \
                     unstored; owner: engine-flipped-status"
                ),
            }
        }
        // [CR#302.6]: summoning sickness — read the tracked `summoning_sick` bool
        // off the live object. A player proxy is never sick (field defaults false).
        Predicate::State(StatePredicate::SummoningSick) => state.objects.obj(id).summoning_sick,
        // [CR#508.1a]: declared as an attacker, still in combat.
        Predicate::State(StatePredicate::Attacking) => state.combat.is_attacking(id),
        // [CR#509.1a]: declared as a blocker (blocking some attacker).
        Predicate::State(StatePredicate::Blocking) => state.combat.attacker_of(id).is_some(),
        // [CR#509.1h]: attacking and not (stickily) blocked.
        Predicate::State(StatePredicate::Unblocked) => {
            state.combat.is_attacking(id) && !state.combat.is_blocked(id)
        }

        // [CR#115.9b]: "that targets [desc]" — `id` is a stack object one of
        // whose chosen targets CURRENTLY matches. A departed target (its id no
        // longer live) is ignored, never read through LKI. A non-stack object
        // has no targets.
        Predicate::State(StatePredicate::Targets(f)) => {
            state.stack.iter().find(|e| e.id == id).is_some_and(|e| {
                // Any still-live member of any slot currently matching.
                e.targets.iter().flatten().any(|&t| {
                    state.objects.get(t).is_some()
                        && matches_with_activation(state, t, f, watcher, activation)
                })
            })
        }
        // [CR#115.9a]: "with [N] target(s)" — the count of target instances
        // chosen at stack-put, FLATTENED across slots. The bound resolves
        // through `resolve_count` (literal directly, dynamic via the
        // watcher-anchored evaluator). Non-stack → none.
        Predicate::State(StatePredicate::TargetCount(bound)) => {
            state.stack.iter().find(|e| e.id == id).is_some_and(|e| {
                bound.satisfied_by(
                    Uint::try_from(e.targets.iter().map(Vec::len).sum::<usize>())
                        .expect("target count fits Uint"),
                    |count| resolve_count(state, count, watcher, activation),
                )
            })
        }

        // [CR#301.5,303.4]: `id` is an attachment attached to a host matching
        // `inner` — read the attachment→host relation (engine-attach's
        // `attached_to`), then match the host, threading the watcher so a nested
        // `Ref` resolves against the carrier.
        Predicate::Relation(RelationPredicate::AttachedTo(inner)) => {
            state.objects.obj(id).attached_to.is_some_and(|host| {
                matches_with_activation(state, host, inner, watcher, activation)
            })
        }
        // The inverse ([CR#301.5,303.4]): `id` is a host with some attachment
        // matching `inner` (existential — `Attachment(Any)` = "has any
        // attachment").
        Predicate::Relation(RelationPredicate::Attachment(inner)) => {
            state.objects.iter().any(|o| {
                o.attached_to == Some(id)
                    && matches_with_activation(state, o.id, inner, watcher, activation)
            })
        }
        // [CR#404.2]: the candidate is directly Above/Below `r` in the SAME
        // ordered zone (graveyard or library), nothing between. `r` resolves
        // only through the frameless matcher's `watcher` (`This`/`You` —
        // Death Spark's "directly above it" only ever needs `This`);
        // anything else, an unresolvable reference, a candidate/anchor
        // outside an ordered zone, or a cross-zone/cross-player pair,
        // gracefully reads `false` — never a panic on a semantic-input error
        // or a not-yet-reachable reference shape.
        Predicate::Adjacent(dir, r) => resolve_frameless_reference(state, r, watcher, activation)
            .and_then(|anchor| {
                let a = ordered_zone_position(state, anchor)?;
                let c = ordered_zone_position(state, id)?;
                (a.0 == c.0).then_some((a.1, c.1))
            })
            .is_some_and(|(anchor_pos, cand_pos)| match dir {
                deckmaste_core::Adjacency::Above => cand_pos + 1 == anchor_pos,
                deckmaste_core::Adjacency::Below => anchor_pos + 1 == cand_pos,
            }),
        // [CR#119.1]: the player-scope stat twin of `Characteristic(Stat(..))`
        // — "a player has 13 or less life". Matches only a PLAYER proxy (a
        // card object never satisfies it, even walked as a `Countable::Players`
        // source alongside proxies); a non-literal bound never matches —
        // dynamic player-stat bounds are a vanishingly rare semantic shape, and
        // a silent no-match beats a panic on a semantic-input error.
        Predicate::PlayerStatCmp(attr, cmp, bound) => match state.objects.obj(id).source {
            ObjectSource::Player(p) => bound
                .literal_value()
                .is_some_and(|n| cmp.apply(state.player_attr(p, *attr), n)),
            ObjectSource::Card(_) => false,
        },
        // The move-provenance twin of `WasCastFrom` ([CR#701.17a,701.9a]).
        // Past-form `ZoneChange` history facts key on the pre-move (stale) `ObjectId`
        // — correlating one back to the CURRENT (post-remint) live object
        // needs either a persistent old→new provenance map (the engine only
        // keeps `moved_chain`, cleared at the start of every resolution,
        // [`resolve::eval_selection_set`]'s doc) or a dedicated per-object
        // "arrived from Z this turn" memo field `GameObject` does not carry.
        // Genuinely absent-subsystem (like the equally-unbuilt sibling
        // `DamagedBy`/`ExiledBy`), not a convenient-wrong default: fizzles to
        // `false`.
        Predicate::State(StatePredicate::WasPutFrom(_)) => false,
        // [CR#120.3,702.2c]: was `id` dealt damage by a source matching
        // `inner`? Existential over the object's marked damage, read against
        // each mark's DEAL-TIME abilities — the source may since have lost
        // the ability or left the battlefield, so the live source is never
        // consulted ([CR#704.5h]'s deathtouch clause is the first consumer).
        Predicate::State(StatePredicate::WasDealtDamageBy(inner)) => {
            state.objects.get(id).is_some_and(|obj| {
                obj.damage
                    .iter()
                    .any(|mark| source_abilities_match(inner, &mark.source_abilities))
            })
        }
        // ----- Seams: backed by subsystems not yet built -----
        // [CR#702.33d..702.33e]: the paid-cost linkage has no announce
        // record yet (engine-alt-costs).
        Predicate::State(StatePredicate::WasPaidWith(tag)) => todo!(
            "engine-alt-costs: WasPaidWith({tag:?}) needs the [CR#601.2b] optional-cost \
             announce record; owner: engine-alt-costs"
        ),
        // The alt-cost twin of `WasPaidWith` ([CR#118.9,702.34a]) — a filter
        // over "a spell cast with [keyword]". No card fixture forces it yet
        // (Flashback/Evoke read the alt cost via `Condition::CastWith`, not this
        // filter); it needs the same engine-alt-costs announce record.
        Predicate::State(StatePredicate::WasCastWith(tag)) => todo!(
            "engine-alt-costs: WasCastWith({tag:?}) needs the [CR#118.9,702.34a] alt-cost \
             announce record; owner: engine-alt-costs"
        ),
        // [CR#607]: linked-ability relations have no registry yet.
        Predicate::State(StatePredicate::RelatedBy(..)) => todo!(
            "engine seam: RelatedBy ([CR#607.1]) — no linked-ability relation registry; \
             owner: engine-filter-breadth"
        ),
        // A bare register read: the candidate IS the object the register holds.
        // The register file is addressed by `activation`; with none (a
        // structural caller) the intrinsic source/controller parameters still
        // answer from the carrier, and anything else reads `false` rather than
        // guessing. The fallback compares by `ObjectSource` rather than by id so
        // a reminted carrier still matches its own register.
        Predicate::Ref(Reference::Reg(reference)) => state
            .activation_product(activation, *reference)
            .and_then(|product| product.current)
            .map_or_else(
                || match (state.activation_provenance(activation, *reference), watcher) {
                    (Some(deckmaste_core::Provenance::Source), Some(source)) => {
                        state.objects.obj(id).source == source
                    }
                    (Some(deckmaste_core::Provenance::Controller), Some(source)) => {
                        let controller = state.controller_of_source(source);
                        matches!(state.objects.obj(id).source,
                            ObjectSource::Player(player) if Some(player) == controller)
                    }
                    (None, Some(source))
                        if Reference::Reg(*reference) == Reference::source_parameter() =>
                    {
                        state.objects.obj(id).source == source
                    }
                    (None, Some(source))
                        if Reference::Reg(*reference) == Reference::controller_parameter() =>
                    {
                        let controller = state.controller_of_source(source);
                        matches!(state.objects.obj(id).source,
                            ObjectSource::Player(player) if Some(player) == controller)
                    }
                    _ => false,
                },
                |bound| bound == id,
            ),
        // A DERIVED reference (`ControllerOf`, `OwnerOf`, `AttachHostOf`,
        // `OpponentOf`, `Coalesce`) is a pure expression over a register
        // (core-explicit-regions law 4), so a filter may name one wherever it
        // may name the register it derives from: resolve it against the same
        // activation record + carrier the `Reg` arm above reads, and match iff
        // it lands on THIS candidate. Unresolvable (a gone register, no
        // carrier, or a shape needing the selection evaluator's `ExecutionFrame` —
        // `Single`/`Source`) reads `false`, the same never-crash fizzle
        // `Predicate::Adjacent` takes.
        Predicate::Ref(r) => resolve_frameless_reference(state, r, watcher, activation)
            .is_some_and(|resolved| resolved == id),
    }
}

/// Resolve `r` to a live [`ObjectId`] using ONLY what the frameless matcher
/// holds: the register file addressed by `activation` plus the carrier
/// `watcher`. `None` — never a panic — for a register with no live product, a
/// derivation over one, or a shape that genuinely needs a
/// [`crate::stack::ExecutionFrame`].
///
/// Derived references are pure expressions over a register
/// (`docs/decisions/core-explicit-regions.md` law 4), introducing no binding,
/// so every one of them is resolvable exactly where its innermost register is.
/// That is why this walks the derivation itself rather than deferring to
/// [`crate::state::GameState::eval_reference`]: that evaluator reads the
/// DERIVED controller through `state.layers()`, and this matcher is reachable
/// from inside a layer rebuild (`layer::condition_predicate_matches` delegates
/// player proxies here), where re-entering `layers()` would recurse. The stored
/// controller read below is the same one the sibling
/// `Predicate::Relation(ControlledBy)` arm uses, so the two spellings of
/// "controlled by the same player as X" agree; both share the narrowness that a
/// control-changing effect, which applies in layer 2 [CR#613.1b], is not seen.
fn resolve_frameless_reference(
    state: &GameState,
    r: &Reference,
    watcher: Option<ObjectSource>,
    activation: crate::ActivationId,
) -> Option<ObjectId> {
    match r {
        // The register's own product first (a resolution-time caller passed a
        // real activation), then the carrier fallbacks the `Predicate::Ref(Reg)`
        // arm takes for a structural caller with no activation record.
        &Reference::Reg(reference) => state
            .activation_product(activation, reference)
            .and_then(|product| product.current)
            .or_else(|| resolve_carrier_register(state, reference, watcher, activation)),
        // [CR#110.2]: every permanent has a controller, so this derives the
        // player under whose control the referenced object is. Only an object
        // on the stack or the battlefield has one at all [CR#109.4]; a player
        // proxy has none, so it fizzles rather than answering with itself.
        Reference::ControllerOf(inner) => {
            let object = state.objects.get(resolve_frameless_reference(
                state, inner, watcher, activation,
            )?)?;
            object.card_id()?;
            Some(state.player(object.controller).object)
        }
        // [CR#108.3]: the owner of a referenced card-backed object; a player
        // proxy has no owner.
        Reference::OwnerOf(inner) => {
            let id = resolve_frameless_reference(state, inner, watcher, activation)?;
            state.objects.get(id)?.card_id()?;
            Some(state.player(state.owner_of(id)).object)
        }
        // [CR#301.5,303.4]: the permanent an attachment is attached to. An
        // unattached attachment resolves to nothing.
        Reference::AttachHostOf(inner) => {
            let id = resolve_frameless_reference(state, inner, watcher, activation)?;
            state.objects.get(id)?.attached_to
        }
        // An opponent of the referenced player — in two-player the only one
        // [CR#102.2], and in multiplayer one of several [CR#102.3], picked here
        // as the SAME single product `resolve::query`'s evaluator picks so the
        // framed and frameless readings of one card agree. The existential "is
        // ANY opponent of" reading is `Predicate::Relation(OpponentOf)`, a
        // different form.
        Reference::OpponentOf(inner) => {
            let id = resolve_frameless_reference(state, inner, watcher, activation)?;
            let player = state.players.iter().find(|p| p.object == id)?;
            Some(state.player(state.next_live_after(player.id)).object)
        }
        // First reference in order that resolves, per the variant's contract.
        Reference::Coalesce(references) => references
            .iter()
            .find_map(|inner| resolve_frameless_reference(state, inner, watcher, activation)),
        // `Single` demotes a SELECTION, which only
        // `resolve::eval_selection_set` evaluates and only from an
        // `ExecutionFrame` — not a register derivation, so it does not resolve
        // here.
        Reference::Single(_) => None,
    }
}

/// The carrier-anchored reading of a register with no live product in the
/// activation record: the intrinsic source/controller parameters resolve
/// against `watcher` — the carrier of the ability doing the matching —
/// and everything else yields `None`.
fn resolve_carrier_register(
    state: &GameState,
    reference: deckmaste_core::RefId,
    watcher: Option<ObjectSource>,
    activation: crate::ActivationId,
) -> Option<ObjectId> {
    let w = watcher?;
    if state.activation_reference_is(activation, reference, &deckmaste_core::Provenance::Source) {
        state.objects.iter().find(|o| o.source == w).map(|o| o.id)
    } else if state.activation_reference_is(
        activation,
        reference,
        &deckmaste_core::Provenance::Controller,
    ) {
        Some(state.player(state.controller_of_source(w)?).object)
    } else {
        None
    }
}

/// The `(zone, owner)` key and top-distance (`0` = the very top) of a live
/// object in an ORDERED zone ([CR#404.2] — a graveyard is a single face-up
/// pile in a fixed order; a library likewise, [CR#401.2]). `None` outside
/// Graveyard/Library (the only zones kept in a fixed physical order) or for a
/// player proxy — [`Predicate::Adjacent`]'s never-crash fizzle reads this as
/// "no match".
///
/// `zones.graveyards` is push-appended as cards are put into it, so the
/// LAST-pushed entry sits physically on top ([`resolve::eval_selection_set`]'s
/// `TopOfGraveyard` arm reads the same convention); `zones.libraries`'
/// front is already the top ([CR#401.7]'s `FromTop(0)` anchor).
fn ordered_zone_position(
    state: &GameState,
    id: ObjectId,
) -> Option<((Zone, crate::player::PlayerId), usize)> {
    let zone = state.objects.get(id)?.zone?;
    match zone {
        Zone::Graveyard => {
            let owner = state.owner_of(id);
            let pile = &state.zones.graveyards[owner.index()];
            let idx = pile.iter().position(|&x| x == id)?;
            Some(((zone, owner), pile.len() - 1 - idx))
        }
        Zone::Library => {
            let owner = state.owner_of(id);
            let lib = &state.zones.libraries[owner.index()];
            let idx = lib.iter().position(|&x| x == id)?;
            Some(((zone, owner), idx))
        }
        _ => None,
    }
}

/// Whether a card object has card type `ty` in the DERIVED view ([CR#613.1d]
/// layer-4 type changes count — an animated land or crewed Vehicle types as a
/// Creature); a player proxy has none. Same per-call `layers()` perf seam as
/// `Has`/`Subtype`.
fn has_type(state: &GameState, id: ObjectId, ty: Ident) -> bool {
    state.objects.obj(id).card_id().is_some()
        && state
            .layers()
            .get(id)
            .card_types
            .iter()
            .any(|t| t.name == ty)
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
                crate::derive::face(state.def(id))
                    .characteristics
                    .mana_cost
                    .mana_value(),
            )
            .expect("mana value fits Int"),
        ),
        // [CR#209.1,306.5a]: loyalty is the PRINTED loyalty characteristic off
        // the card face — never the live counter count (current loyalty is
        // `CounterCount(This, LoyaltyCounter)`). `base_stat` maps `Number(n)→n`,
        // `DefinedByAbility`/`Variable`/absent → 0, and `None` (no printed
        // loyalty) → `None`, mirroring the P/T arms above.
        deckmaste_core::Stat::Loyalty => crate::layer::base_stat(
            crate::derive::face(state.def(id))
                .characteristics
                .loyalty
                .as_ref(),
        ),
        deckmaste_core::Stat::Defense => Some(
            deckmaste_core::Int::try_from(
                state
                    .objects
                    .obj(id)
                    .counters
                    .get("DefenseCounter")
                    .copied()
                    .unwrap_or(0),
            )
            .expect("defense fits Int"),
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

/// A `Count` bound with NO watcher available at all: only a literal
/// evaluates (a dynamic bound — `CountOf`, `StatOf`, `X`, … — needs a
/// carrier). [`resolve_count`] tries the watcher-anchored path first and
/// falls back to this for the true-frameless case, which stays a loud seam:
/// audited (engine-candidate-frame-context) as unreached — every corpus
/// `Stat`/`TargetCount` predicate that carries a dynamic bound (Skulk) is
/// always evaluated WITH a watcher.
fn const_count(count: &deckmaste_core::Count) -> Uint {
    match count {
        deckmaste_core::Count::Literal(n) => *n,
        other => todo!(
            "engine seam: dynamic filter bound {other:?} needs a carrier frame — only \
             literal bounds evaluate in the frameless matcher; \
             owner: engine-candidate-frame-context"
        ),
    }
}

/// A `Count` bound for the live matcher, general enough to cover a dynamic
/// bound: a literal evaluates directly; anything else resolves through the
/// full evaluator ([`GameState::eval_count`]) via a bare [`ExecutionFrame`] anchored on
/// the watcher's live carrier — `This`/`You` inside the bound read that
/// carrier, mirroring the `Ref(This)`/`Ref(You)` arms above. Reachable: Skulk
/// ([CR#702.118b]) is `Stat(Power, Greater, StatOf(This, Power))`, evaluated
/// live via `filter_matches_live` with the ability's source as watcher. No
/// watcher, or a watcher whose carrier has already left, falls back to
/// [`const_count`]'s frameless seam.
fn resolve_count(
    state: &GameState,
    count: &deckmaste_core::Count,
    watcher: Option<ObjectSource>,
    activation: crate::ActivationId,
) -> Uint {
    if let deckmaste_core::Count::Literal(n) = count {
        return *n;
    }
    if let Some(w) = watcher
        && let Some(carrier) = state.objects.iter().find(|o| o.source == w)
    {
        let mut frame = state.frame(carrier.id, carrier.controller);
        if activation != crate::ActivationId::NONE {
            frame.activation = activation;
        }
        return state.eval_count(count, &frame);
    }
    const_count(count)
}

/// Compare a stat `value` (possibly missing, possibly negative) against an
/// ALREADY-RESOLVED `Uint` bound — the [`resolve_count`] twin of
/// [`stat_satisfies`] for the live matcher's `Stat` arm, which resolves a
/// dynamic bound before comparing. A negative value clamps to 0
/// ([CR#107.1b]); a missing stat (a land has no power) never satisfies the
/// predicate.
#[must_use]
fn stat_satisfies_bound(
    value: Option<deckmaste_core::Int>,
    cmp: deckmaste_core::Cmp,
    bound: Uint,
) -> bool {
    value.is_some_and(|v| {
        cmp.apply(
            Uint::try_from(v.max(0)).expect("clamped stat fits Uint"),
            bound,
        )
    })
}

/// Every object (card objects in their zones + player proxies) matching
/// `filter`, in deterministic id order. The frameless form — no carrier — for
/// the resolution-time selection sites (`Exists`, `Each`, `Selection`) whose
/// filters never carry a carrier-relative self-reference.
#[must_use]
pub fn candidates(state: &GameState, filter: &Predicate) -> Vec<ObjectId> {
    candidates_with(state, filter, None)
}

/// Every object matching `filter`, with `watcher` anchoring carrier-relative
/// self-references (`Ref(This)`/`Ref(You)`, and the `StatOf(This, …)` reads a
/// `Predicate::Where` reaches via [`GameState::condition_holds`]). The
/// targeting path passes the targeting object's `ObjectSource` so a target
/// filter can compare each candidate to the ability's source — "attacking
/// creature with power less than this creature's power" (Mentor, [CR#702.134a])
/// is a `Where(Compare(StatOf(Subject, Power), Less, StatOf(This, Power)))`
/// over this carrier. Frameless callers pass `None` (the [`candidates`]
/// shorthand).
#[must_use]
pub fn candidates_with(
    state: &GameState,
    filter: &Predicate,
    watcher: Option<ObjectSource>,
) -> Vec<ObjectId> {
    state
        .objects
        .iter()
        .map(|o| o.id)
        .filter(|&id| matches_with(state, id, filter, watcher))
        .collect()
}

/// Region-aware candidate enumeration used while executing an instruction.
#[must_use]
pub(crate) fn candidates_with_activation(
    state: &GameState,
    filter: &Predicate,
    watcher: Option<ObjectSource>,
    activation: crate::ActivationId,
) -> Vec<ObjectId> {
    state
        .objects
        .iter()
        .map(|o| o.id)
        .filter(|&id| matches_with_activation(state, id, filter, watcher, activation))
        .collect()
}

/// Whether `id` matches a predicate REGION. The region's declared candidate
/// domain gates the Entity boundary before the predicate runs ([CR#109.1,102.1]
/// — ADR law 2): an Object-domain region never sees a player, and a
/// Player-domain region never sees an object.
#[must_use]
pub(crate) fn matches_region_with_activation(
    state: &GameState,
    id: ObjectId,
    region: &deckmaste_core::Region<Predicate>,
    watcher: Option<ObjectSource>,
    activation: crate::ActivationId,
) -> bool {
    if !region.candidate_domain().admits(entity_class(state, id)) {
        return false;
    }
    let (source, controller) = if activation == crate::ActivationId::NONE {
        let Some((source, controller)) = watcher.and_then(|wanted| {
            state
                .objects
                .iter()
                .find(|object| object.source == wanted)
                .map(|object| (object.id, object.controller))
        }) else {
            return false;
        };
        (source, controller)
    } else {
        (
            state.activation_source(activation),
            state.activation_controller(activation),
        )
    };
    let mut frame = state.frame(source, controller);
    if activation != crate::ActivationId::NONE {
        frame.activation = activation;
    }
    frame.activation = state.enter_candidate_region(region, &frame, id);
    matches_with_activation(state, id, &region.body, watcher, frame.activation)
}

#[must_use]
pub(crate) fn candidates_region_with_activation(
    state: &GameState,
    region: &deckmaste_core::Region<Predicate>,
    watcher: Option<ObjectSource>,
    activation: crate::ActivationId,
) -> Vec<ObjectId> {
    state
        .objects
        .iter()
        .map(|object| object.id)
        .filter(|&id| matches_region_with_activation(state, id, region, watcher, activation))
        .collect()
}

#[cfg(test)]
mod tests {
    use std::path::Path;
    use std::sync::Arc;

    use deckmaste_core::Predicate;
    use deckmaste_core::TargetSpec;
    use deckmaste_core::Type;
    use deckmaste_core::Zone;
    use deckmaste_plugin::plugin::Plugin;

    use super::*;
    use crate::player::PlayerId;
    use crate::state::GameConfig;
    use crate::state::GameState;
    use crate::state::PlayerConfig;
    use crate::state::StartingPlayer;

    fn builtin() -> Plugin {
        Plugin::load(Path::new(env!("CARGO_MANIFEST_DIR")).join("../../plugins/builtin")).unwrap()
    }

    /// A bare two-player game with empty decks — for tests that hand-mint
    /// their own objects (mirrors `resolve::tests::game`).
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
        // Force a Grizzly Bears from player 0's hand onto the battlefield.
        let bear = *state.zones.hands[0]
            .iter()
            .find(|&&o| {
                matches(
                    &state,
                    o,
                    &Predicate::r#type(deckmaste_core::Type::Creature),
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
            starting_player: StartingPlayer::Fixed(PlayerId(1)),
            sba_rules: vec![],
            conferral_rules: vec![],
            damage_result_rules: vec![],
            counter_decls: std::collections::HashMap::new(),
            subtypes: std::collections::HashMap::new(),
            types: std::collections::HashMap::new(),
        });
        // Force a Forest from player 1's hand onto the battlefield.
        let land = *state.zones.hands[1]
            .iter()
            .find(|&&o| matches(&state, o, &Predicate::r#type(deckmaste_core::Type::Land)))
            .expect("a Forest in the opening hand (10-card mono deck)");
        state.remove_from_hand(PlayerId(1), land);
        state.objects.obj_mut(land).zone = Some(Zone::Battlefield);
        state.zones.battlefield.push(land);
        (state, land)
    }

    /// Regression ([CR#613.1d]): `Predicate::Type` reads the DERIVED type, not
    /// the printed face. A battlefield land animated into a creature by a
    /// layer-4 continuous effect matches `Type(Creature)` — the view says
    /// Creature, so the matcher must agree. (Before the fix `has_type` read
    /// the printed face and reported the land was not a creature,
    /// mis-typing every `candidates` / targeting caller.)
    #[test]
    fn type_filter_reads_derived_type_for_animated_land() {
        use deckmaste_core::Duration;
        use deckmaste_core::Modification;

        use crate::layer::ContinuousEffect;
        use crate::layer::ScopeResolved;
        use crate::object::Timestamp;

        let (mut state, land) = game_with_a_forest_on_the_field();
        let creature = Predicate::r#type(deckmaste_core::Type::Creature);
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
                deckmaste_core::Type::Creature.name(),
            ))],
            duration: Duration::EndOfGame,
            rows: vec![],
            origin: None,
            is_cda: false,
        });

        // The derived view says Creature...
        assert!(
            state.layers().get(land).has_type(Type::Creature),
            "sanity: the animated land derives as a creature"
        );
        // ...and the Type filter must agree ([CR#613.1d]).
        assert!(
            matches(&state, land, &creature),
            "Predicate::Type reads the derived type — the animated land matches Type(\"Creature\")"
        );
    }

    #[test]
    fn any_target_is_creatures_and_players_not_lands() {
        // Parse `AnyTarget` through the SEMANTIC macro registry, then lower —
        // the path production now takes (`semantics::TargetSpec` → `lower()`),
        // which erases the `Expanded` wrapper before the engine ever sees the
        // value. `resolve::target_spec_filter` is the engine's own
        // TargetSpec→Predicate extraction — the path real targeting funnels
        // through — so the test exercises it rather than hand-unwrapping the
        // expansion.
        use deckmaste_lowering::Lower;
        let semantic: deckmaste_semantics::TargetSpec =
            builtin().macros.read_str("AnyTarget").unwrap();
        let any_target: TargetSpec = semantic.lower();
        let filter = crate::resolve::target_spec_filter(&any_target);
        let (state, bear) = game_with_a_bear_on_the_field();
        let targets = candidates_region_with_activation(
            &state,
            filter,
            Some(state.objects.obj(bear).source),
            crate::ActivationId::NONE,
        );
        // Both player proxies + the lone battlefield creature; no lands (in
        // hand/library), no spells (stack empty).
        assert!(targets.contains(&bear));
        assert!(targets.contains(&state.players[0].object));
        assert!(targets.contains(&state.players[1].object));
        assert_eq!(targets.len(), 3);
    }

    /// A filter-position macro (`kinds: [Predicate]`) survives SEMANTIC
    /// expansion as `semantics::Predicate::Expanded`, but `lower` (the path
    /// production now takes) erases the wrapper before the engine ever sees
    /// the value — so `matches`/`candidates` must still evaluate the lowered
    /// body correctly. Guards the corpus-value path against being mistaken
    /// for dead code.
    #[test]
    fn matches_a_predicate_lowered_from_a_filter_macro() {
        use deckmaste_lowering::Lower;
        // `CreatureOrPlayer` reads (semantically) as
        // `Predicate::Expanded(.., value: Or([..]))`: the invocation survives
        // pre-lowering, wrapping its expanded body.
        let semantic: deckmaste_semantics::Predicate =
            builtin().macros.read_str("CreatureOrPlayer").unwrap();
        assert!(
            matches!(semantic, deckmaste_semantics::Predicate::Expanded(_)),
            "a filter macro should survive semantic parse as Predicate::Expanded, got {semantic:?}"
        );
        let filter: Predicate = semantic.lower();
        let (state, bear) = game_with_a_bear_on_the_field();
        // Evaluating the lowered macro body reaches the battlefield creature.
        assert!(candidates(&state, &filter).contains(&bear));
    }

    /// `InHand(who)` ([CR#701.9a] discard's domain) composes `InZone(Hand)`
    /// and an `Owner` match on the parameterized reference — a card in the
    /// named player's hand, not hardcoded to `You`'s controller (user
    /// ruling: the old `Selection::FromHand` was composite, not intrinsic,
    /// so discard now rides this filter through an ordinary `Choose`/
    /// `Random` binder). `Ref(You)` needs a carrier, so this reads through
    /// `matches_with` with an explicit watcher, not the frameless `matches`.
    #[test]
    fn in_hand_matches_a_card_in_the_named_players_hand() {
        use deckmaste_lowering::Lower;

        let (state, bear, _ghoul) = game_with_bear_and_ghoul();
        // Parsed through the SEMANTICS path (`semantics::Predicate` →
        // `lower()`), the path production now takes.
        let semantic: deckmaste_semantics::Predicate =
            builtin().macros.read_str("InHand(You)").unwrap();
        let filter: Predicate = semantic.lower();
        // Watched by the battlefield bear — still P0-controlled, so `You`
        // resolves to P0.
        let carrier = Some(state.objects.obj(bear).source);
        let p0_hand_card = state.zones.hands[0][0];
        let p1_hand_card = state.zones.hands[1][0];
        assert!(matches_with(&state, p0_hand_card, &filter, carrier));
        assert!(!matches_with(&state, p1_hand_card, &filter, carrier));
        // The battlefield bear itself is not in ANY hand.
        assert!(!matches_with(&state, bear, &filter, carrier));
    }

    // -------------------------------------------------------------------------
    // Characteristic arms: Named / ColorIs / Multicolored / Colorless /
    // Supertype / Stat
    // -------------------------------------------------------------------------

    use deckmaste_core::CharacteristicPredicate as CF;

    fn cf(c: CF) -> Predicate {
        Predicate::Characteristic(c)
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
            rows: vec![],
            origin: None,
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

    /// `Stat` over a DYNAMIC bound (`StatOf`, not a literal) resolves through
    /// the watcher-anchored `resolve_count`, rather than the frameless
    /// `const_count` seam — Skulk's actual blocker filter ([CR#702.118b],
    /// Furtive Homunculus): `Stat(Power, Greater, StatOf(This, Power))`.
    /// Regression for engine-candidate-frame-context's reachability finding.
    #[test]
    fn stat_predicate_resolves_a_dynamic_statof_bound_via_watcher() {
        use deckmaste_core::Cmp;
        use deckmaste_core::Count;
        use deckmaste_core::Duration;
        use deckmaste_core::Modification;
        use deckmaste_core::NumericOp;
        use deckmaste_core::Reference;
        use deckmaste_core::Stat;

        use crate::layer::ContinuousEffect;
        use crate::layer::ScopeResolved;
        use crate::object::Timestamp;

        let (mut state, bear) = game_with_a_bear_on_the_field(); // 2/2, the watcher
        let watcher = Some(state.objects.obj(bear).source);
        let skulk_filter = cf(CF::Stat(
            Stat::Power,
            Cmp::Greater,
            Count::StatOf(Reference::Reg(deckmaste_core::RefId(0)), Stat::Power),
        ));

        // A second 2/2 candidate does not exceed the watcher's power (2 is
        // not greater than 2) — the dynamic bound resolves to 2, not a panic.
        let candidate = {
            let bears = Arc::new(canon().card("Grizzly Bears").unwrap().core);
            let cid = state.cards.push(bears, PlayerId(1));
            let bid = state.objects.mint(
                ObjectSource::Card(cid),
                PlayerId(1),
                Some(Zone::Battlefield),
            );
            state.zones.battlefield.push(bid);
            bid
        };
        assert!(!matches_with(&state, candidate, &skulk_filter, watcher));

        // Boost the candidate to 3 power — now it exceeds the watcher's 2,
        // matching Skulk's "greater power" blocker filter.
        state.continuous.push(ContinuousEffect {
            timestamp: Timestamp(1_000),
            controller: PlayerId(1),
            scope: ScopeResolved::Locked(vec![candidate]),
            changes: vec![Modification::Power(NumericOp::Up(Count::Literal(1)))],
            duration: Duration::EndOfGame,
            rows: vec![],
            origin: None,
            is_cda: false,
        });
        assert!(matches_with(&state, candidate, &skulk_filter, watcher));
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
            &Predicate::State(StatePredicate::Status(Status::Untapped))
        ));
        assert!(!matches(
            &state,
            bear,
            &Predicate::State(StatePredicate::Status(Status::Tapped))
        ));
        state.objects.obj_mut(bear).tapped = true;
        assert!(matches(
            &state,
            bear,
            &Predicate::State(StatePredicate::Status(Status::Tapped))
        ));
        assert!(!matches(
            &state,
            bear,
            &Predicate::State(StatePredicate::Status(Status::Untapped))
        ));
    }

    /// [CR#302.6]: `SummoningSick` reads the live `summoning_sick` bool.
    #[test]
    fn summoning_sick_reads_the_live_flag() {
        let (mut state, bear) = game_with_a_bear_on_the_field();
        state.objects.obj_mut(bear).summoning_sick = true;
        assert!(matches(
            &state,
            bear,
            &Predicate::State(StatePredicate::SummoningSick)
        ));
        state.objects.obj_mut(bear).summoning_sick = false;
        assert!(!matches(
            &state,
            bear,
            &Predicate::State(StatePredicate::SummoningSick)
        ));
    }

    /// `Attacking`/`Blocking`/`Unblocked` read live combat state
    /// ([CR#508.1a,509.1a,509.1h]).
    #[test]
    fn combat_filters_read_combat_state() {
        let (mut state, attacker) = game_with_a_bear_on_the_field();
        // A second creature to block with.
        let blocker = {
            let bears = Arc::new(canon().card("Grizzly Bears").unwrap().core);
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
            &Predicate::State(StatePredicate::Attacking)
        ));
        assert!(!matches(
            &state,
            blocker,
            &Predicate::State(StatePredicate::Blocking)
        ));

        // Declare the attacker — attacking and (no blocker yet) unblocked.
        let def = state
            .player(state.next_live_after(state.objects.obj(attacker).controller))
            .object;
        state.combat.declare_attacker(attacker, def);
        assert!(matches(
            &state,
            attacker,
            &Predicate::State(StatePredicate::Attacking)
        ));
        assert!(matches(
            &state,
            attacker,
            &Predicate::State(StatePredicate::Unblocked)
        ));
        assert!(!matches(
            &state,
            blocker,
            &Predicate::State(StatePredicate::Blocking)
        ));

        // Declare the block — blocker is blocking, attacker no longer
        // unblocked.
        state.combat.declare_block(blocker, attacker);
        assert!(matches(
            &state,
            blocker,
            &Predicate::State(StatePredicate::Blocking)
        ));
        assert!(matches(
            &state,
            attacker,
            &Predicate::State(StatePredicate::Attacking)
        ));
        assert!(!matches(
            &state,
            attacker,
            &Predicate::State(StatePredicate::Unblocked)
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
            let forest = Arc::new(builtin().card("Forest").unwrap().core);
            let cid = state.cards.push(forest, PlayerId(1));
            state.objects.mint(
                ObjectSource::Card(cid),
                PlayerId(1),
                Some(Zone::Battlefield),
            )
        };
        (state, bear, p1_card)
    }

    fn marked() -> Predicate {
        Predicate::State(StatePredicate::HasCounter("mark".into()))
    }

    /// `ControlledBy` matches when the object's controller's proxy matches
    /// ([CR#109.5]).
    #[test]
    fn controlled_by_matches_controllers_proxy() {
        let (state, bear, p1_card) = marked_p0_game();
        let f = Predicate::Relation(RelationPredicate::ControlledBy(Arc::new(marked())));
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
        let f = Predicate::Relation(RelationPredicate::OpponentOf(Arc::new(marked())));
        assert!(matches(&state, p1, &f)); // P1 is an opponent of marked P0
        assert!(!matches(&state, p0, &f)); // P0's only opponent (P1) is un-marked
    }

    /// `Owner` matches when the object's owner's proxy matches ([CR#108.3]); a
    /// player proxy has no owner.
    #[test]
    fn owner_matches_owning_player() {
        let (state, bear, p1_card) = marked_p0_game();
        let f = Predicate::Relation(RelationPredicate::Owner(Arc::new(marked())));
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
        let f = Predicate::Relation(RelationPredicate::Controls(Arc::new(cf(CF::Type(
            Type::Creature.into(),
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
        let teammate = Predicate::Relation(RelationPredicate::TeammateOf(Arc::new(marked())));
        assert!(
            !matches(&state, p0, &teammate),
            "a player is never their own teammate"
        );
        assert!(
            !matches(&state, p1, &teammate),
            "no teammates exist in a 1v1 game (singleton teams)"
        );
        // The opponent relation still resolves (P1 is an opponent of marked
        // P0).
        let opponent = Predicate::Relation(RelationPredicate::OpponentOf(Arc::new(marked())));
        assert!(matches(&state, p1, &opponent));
    }

    /// An activated/triggered ability on the stack is `AbilityOnStack`, not
    /// `Spell` and not `Card`, even though its freshly minted stack id shares
    /// the source card's `ObjectSource` ([CR#602.2a,603.3,405.1]). A real
    /// spell stays a `Spell`, and the ability is a `Class(AbilityOnStack)`
    /// target candidate.
    #[test]
    fn ability_on_stack_is_the_ability_class_not_spell() {
        use crate::stack::StackEntry;
        use crate::stack::StackObject;
        use crate::trigger::TriggerBindings;

        let (mut state, _bear) = game_with_a_bear_on_the_field();
        let cid = state.cards.push(
            Arc::new(builtin().card("Forest").unwrap().core),
            PlayerId(0),
        );

        // A triggered ability's stack token (shares the card source).
        let ability_id =
            state
                .objects
                .mint(ObjectSource::Card(cid), PlayerId(0), Some(Zone::Stack));
        state.stack.push(StackEntry {
            activation: crate::ActivationId::NONE,
            paid_costs: Vec::new(),
            id: ability_id,
            object: StackObject::Triggered {
                source: ObjectSource::Card(cid),
                ability: 0,
                created: None,
                bindings: TriggerBindings::default(),
            },
            controller: PlayerId(0),
            targets: vec![],
            chosen_modes: std::sync::Arc::from([]),
            x: None,
            copy: false,
        });
        // A real spell on the stack, for contrast.
        let spell_id = state
            .objects
            .mint(ObjectSource::Card(cid), PlayerId(0), Some(Zone::Stack));
        state.stack.push(StackEntry {
            activation: crate::ActivationId::NONE,
            paid_costs: Vec::new(),
            id: spell_id,
            object: StackObject::Spell(spell_id),
            controller: PlayerId(0),
            targets: vec![],
            chosen_modes: std::sync::Arc::from([]),
            x: None,
            copy: false,
        });

        assert!(is_object_class(
            &state,
            ability_id,
            ObjectClass::AbilityOnStack
        ));
        assert!(!is_object_class(&state, ability_id, ObjectClass::Card));
        assert!(matches(
            &state,
            ability_id,
            &Predicate::Class(ObjectClass::AbilityOnStack)
        ));
        assert!(!matches(
            &state,
            ability_id,
            &Predicate::Class(ObjectClass::Spell)
        ));

        assert!(is_object_class(&state, spell_id, ObjectClass::Spell));
        // A card on the stack is a Card AND a Spell ([CR#108.2,112.1]) — the
        // two classes are independently testable and overlap.
        assert!(is_object_class(&state, spell_id, ObjectClass::Card));
        assert!(matches(
            &state,
            spell_id,
            &Predicate::And(Arc::from([
                Predicate::Class(ObjectClass::Card),
                Predicate::Class(ObjectClass::Spell),
            ]))
        ));
        assert!(matches(
            &state,
            spell_id,
            &Predicate::Class(ObjectClass::Spell)
        ));
        assert!(!matches(
            &state,
            spell_id,
            &Predicate::Class(ObjectClass::AbilityOnStack)
        ));

        // "counter target ability": the ability is a candidate, the spell
        // isn't.
        let abilities = candidates(&state, &Predicate::Class(ObjectClass::AbilityOnStack));
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
            let card = Arc::new(builtin().card("Forest").unwrap().core);
            let cid = state.cards.push(card, PlayerId(0));
            state
                .objects
                .mint(ObjectSource::Card(cid), PlayerId(0), Some(Zone::Stack))
        };
        state.stack.push(StackEntry {
            activation: crate::ActivationId::NONE,
            paid_costs: Vec::new(),
            id: spell,
            object: StackObject::Spell(spell),
            controller: PlayerId(0),
            targets: vec![vec![bear]],
            chosen_modes: std::sync::Arc::from([]),
            x: None,
            copy: false,
        });
        (state, spell, bear)
    }

    /// `Targets(f)` matches a stack object one of whose targets currently
    /// matches `f` ([CR#115.9b]); a non-stack object has no targets.
    #[test]
    fn targets_reads_a_stack_objects_chosen_targets() {
        let (state, spell, bear) = spell_targeting_bear();
        let targets_creature = Predicate::State(StatePredicate::Targets(Arc::new(cf(CF::Type(
            Type::Creature.into(),
        )))));
        let targets_land = Predicate::State(StatePredicate::Targets(Arc::new(cf(CF::Type(
            Type::Land.into(),
        )))));
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
        let targets_creature = Predicate::State(StatePredicate::Targets(Arc::new(cf(CF::Type(
            Type::Creature.into(),
        )))));
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
            &Predicate::State(StatePredicate::TargetCount(CountBound::Eq(Count::Literal(
                1
            )))),
        ));
        assert!(!matches(
            &state,
            spell,
            &Predicate::State(StatePredicate::TargetCount(CountBound::Eq(Count::Literal(
                2
            )))),
        ));
        assert!(matches(
            &state,
            spell,
            &Predicate::State(StatePredicate::TargetCount(CountBound::AtLeast(
                Count::Literal(1)
            ))),
        ));
        // A non-stack object has no targets → never satisfies a ≥ 1 bound.
        assert!(!matches(
            &state,
            bear,
            &Predicate::State(StatePredicate::TargetCount(CountBound::AtLeast(
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
            .find(|&&o| matches(&state, o, &cf(CF::Type(Type::Creature.into()))))
            .expect("a second Grizzly Bears in the opening hand");
        state.remove_from_hand(PlayerId(0), a);
        state.objects.obj_mut(a).zone = Some(Zone::Battlefield);
        state.zones.battlefield.push(a);
        state.objects.obj_mut(a).attached_to = Some(b);

        let attached_to_creature = Predicate::Relation(RelationPredicate::AttachedTo(Arc::new(
            cf(CF::Type(Type::Creature.into())),
        )));
        let has_any_attachment =
            Predicate::Relation(RelationPredicate::Attachment(Arc::new(Predicate::Any)));

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
        let bears = Arc::new(canon().card("Grizzly Bears").unwrap().core);
        let ghouls = Arc::new(canon().card("Diregraf Ghoul").unwrap().core);
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
            conferral_rules: vec![],
            damage_result_rules: vec![],
            counter_decls: std::collections::HashMap::new(),
            subtypes: std::collections::HashMap::new(),
            types: std::collections::HashMap::new(),
        });
        let creature = Predicate::r#type(deckmaste_core::Type::Creature);
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

    // -------------------------------------------------------------------------
    // The Mentor target filter ([CR#702.134a]): a cross-object stat comparison
    // in a TARGET filter — "attacking creature with power less than this
    // creature's power" — evaluated by the watcher-threaded `candidates_with`.
    // -------------------------------------------------------------------------

    /// Force a fresh battlefield creature of card `name` (from canon) under
    /// player 0, returning its object id.
    #[allow(
        dead_code,
        reason = "shared fixture retained for neighboring target cases"
    )]
    fn put_canon_creature(state: &mut GameState, name: &str) -> ObjectId {
        let card = Arc::new(canon().card(name).unwrap().core);
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
    #[allow(
        dead_code,
        reason = "shared fixture retained for neighboring target cases"
    )]
    fn mentor_board() -> (GameState, ObjectId, ObjectId, ObjectId) {
        let courser = Arc::new(canon().card("Centaur Courser").unwrap().core);
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
            conferral_rules: vec![],
            damage_result_rules: vec![],
            counter_decls: std::collections::HashMap::new(),
            subtypes: std::collections::HashMap::new(),
            types: std::collections::HashMap::new(),
        });
        let courser = put_canon_creature(&mut state, "Centaur Courser"); // 3/3 carrier
        let bears = put_canon_creature(&mut state, "Grizzly Bears"); // 2/2 lesser
        let hunter = put_canon_creature(&mut state, "Fangren Hunter"); // 4/4 greater
        for &id in &[courser, bears, hunter] {
            let def = state
                .player(state.next_live_after(state.objects.obj(id).controller))
                .object;
            state.combat.declare_attacker(id, def);
        }
        (state, courser, bears, hunter)
    }

    /// `Adjacent` (Death Spark's "a creature card directly above it"):
    /// three graveyard cards a→b→c bottom-to-top ([CR#404.2]); with `This`
    /// anchored on the middle card `b`, `Adjacent(Above, This)` admits ONLY
    /// `c` (directly above) and `Adjacent(Below, This)` admits ONLY `a`
    /// (directly below) — never the anchor itself nor the far card.
    #[test]
    fn adjacent_matches_the_one_neighbor_in_the_stated_direction() {
        use deckmaste_card::Card;
        use deckmaste_card::CardFace;
        use deckmaste_card::Characteristics;
        use deckmaste_core::Adjacency;

        let mut state = game();
        let p0 = PlayerId(0);
        let make_card = |name: &str| {
            Card::Normal(CardFace::from(Characteristics {
                name: name.into(),
                ..Characteristics::default()
            }))
        };
        let card_a = state.cards.push(Arc::new(make_card("Alpha")), p0);
        let card_b = state.cards.push(Arc::new(make_card("Beta")), p0);
        let card_c = state.cards.push(Arc::new(make_card("Gamma")), p0);
        let a = state
            .objects
            .mint(ObjectSource::Card(card_a), p0, Some(Zone::Graveyard));
        let b = state
            .objects
            .mint(ObjectSource::Card(card_b), p0, Some(Zone::Graveyard));
        let c = state
            .objects
            .mint(ObjectSource::Card(card_c), p0, Some(Zone::Graveyard));
        state.zones.graveyards[p0.index()].push(a);
        state.zones.graveyards[p0.index()].push(b);
        state.zones.graveyards[p0.index()].push(c);

        let watcher = Some(state.objects.obj(b).source);
        let above = Predicate::Adjacent(Adjacency::Above, Reference::Reg(deckmaste_core::RefId(0)));
        let below = Predicate::Adjacent(Adjacency::Below, Reference::Reg(deckmaste_core::RefId(0)));

        assert!(
            matches_with(&state, c, &above, watcher),
            "c is directly above b"
        );
        assert!(
            !matches_with(&state, a, &above, watcher),
            "a is not above b"
        );
        assert!(
            !matches_with(&state, b, &above, watcher),
            "b is not above itself"
        );

        assert!(
            matches_with(&state, a, &below, watcher),
            "a is directly below b"
        );
        assert!(
            !matches_with(&state, c, &below, watcher),
            "c is not below b"
        );

        // No watcher (frameless) → `This` is unresolvable → no match, never a
        // panic.
        assert!(!matches_with(&state, c, &above, None));
    }

    /// A per-candidate predicate is a REGION (ADR law 1): the candidate it
    /// tests is parameter zero and the carrier (`This`) is parameter one, so
    /// a cross-object comparison reads two declared registers rather than two
    /// anaphors.
    fn candidate_region_with_carrier(
        condition: deckmaste_core::Condition,
    ) -> Arc<deckmaste_core::Region<deckmaste_core::Condition>> {
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
            ]),
            condition,
        ))
    }

    /// The candidate under test inside [`candidate_region_with_carrier`].
    const CANDIDATE: deckmaste_core::RefId = deckmaste_core::RefId(0);
    /// The carrier (`This`) inside [`candidate_region_with_carrier`].
    const CARRIER: deckmaste_core::RefId = deckmaste_core::RefId(1);

    /// The Mentor target filter: an attacking creature whose power is less than
    /// the carrier's power ([CR#702.134a]) — the `Where` cross-object stat
    /// comparison the keyword macro emits, re-spelled as the two register
    /// reads the candidate region declares.
    fn mentor_target_filter() -> Predicate {
        use deckmaste_core::Cmp;
        use deckmaste_core::Condition;
        use deckmaste_core::Count;
        use deckmaste_core::Reference;
        use deckmaste_core::Stat;
        Predicate::And(
            vec![
                Predicate::State(StatePredicate::Attacking),
                Predicate::Where(candidate_region_with_carrier(Condition::Compare(
                    Count::StatOf(Reference::Reg(CANDIDATE), Stat::Power),
                    Cmp::Less,
                    Count::StatOf(Reference::Reg(CARRIER), Stat::Power),
                ))),
            ]
            .into(),
        )
    }

    fn where_is_candidate_color(c: deckmaste_core::Color) -> Predicate {
        Predicate::Where(candidate_region_with_carrier(
            deckmaste_core::Condition::Matches(
                deckmaste_core::Reference::Reg(CANDIDATE),
                Predicate::Characteristic(deckmaste_core::CharacteristicPredicate::ColorIs(c)),
            ),
        ))
    }

    /// `Where(SharesColor(candidate, carrier))` spelled out: the candidate
    /// shares a color with the carrier.
    fn where_shares_color_with_carrier() -> Predicate {
        use deckmaste_core::CharacteristicPredicate::ColorIs;
        use deckmaste_core::Color::Black;
        use deckmaste_core::Color::Blue;
        use deckmaste_core::Color::Green;
        use deckmaste_core::Color::Red;
        use deckmaste_core::Color::White;
        use deckmaste_core::Condition;
        use deckmaste_core::Reference;
        let branch = |c| {
            Condition::And(
                vec![
                    Condition::Matches(
                        Reference::Reg(CANDIDATE),
                        Predicate::Characteristic(ColorIs(c)),
                    ),
                    Condition::Matches(
                        Reference::Reg(CARRIER),
                        Predicate::Characteristic(ColorIs(c)),
                    ),
                ]
                .into(),
            )
        };
        Predicate::Where(candidate_region_with_carrier(Condition::Or(
            vec![
                branch(White),
                branch(Blue),
                branch(Black),
                branch(Red),
                branch(Green),
            ]
            .into(),
        )))
    }

    /// A per-candidate sanity slice of the same comparison via `matches_with`:
    /// the carrier anchors the region's source parameter, the candidate its
    /// candidate parameter.
    #[test]
    fn mentor_filter_matches_with_carrier() {
        let (state, courser, bears, hunter) = mentor_board();
        let carrier = Some(state.objects.obj(courser).source);
        let f = mentor_target_filter();
        assert!(matches_with(&state, bears, &f, carrier));
        assert!(!matches_with(&state, hunter, &f, carrier));
        assert!(!matches_with(&state, courser, &f, carrier));
    }

    /// `candidates_with(.., Some(carrier))` over the Mentor filter admits ONLY
    /// the lesser-power attacker: the 2/2 Bears (2 < 3), never the 4/4 Hunter
    /// (4 < 3 is false) nor the 3/3 carrier itself (3 < 3 is false). This is
    /// the load-bearing target-path fix — the carrier resolves through the
    /// threaded watcher into the candidate region's source parameter, so the
    /// dynamic `StatOf(carrier, Power)` bound evaluates instead of tripping
    /// the frameless `todo!`.
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
        // candidate StatOf(.., Power) read fails their match, never the
        // carrier's).
        assert_eq!(admitted, vec![bears], "exactly the lesser-power attacker");
    }

    /// The candidate register inside a `Where` condition resolves to the
    /// object being matched: a green Grizzly Bears satisfies
    /// `Where(Is(candidate, ColorIs(Green)))` but not `ColorIs(White)`.
    /// Re-spelled from `where_binds_subject_to_the_candidate` — the anaphor
    /// is now the region's candidate parameter.
    #[test]
    fn where_binds_the_candidate_register_to_the_candidate() {
        let (state, bear, _ghoul) = game_with_bear_and_ghoul();
        let carrier = Some(state.objects.obj(bear).source);
        assert!(matches_with(
            &state,
            bear,
            &where_is_candidate_color(deckmaste_core::Color::Green),
            carrier
        ));
        assert!(!matches_with(
            &state,
            bear,
            &where_is_candidate_color(deckmaste_core::Color::White),
            carrier
        ));
    }

    /// `SharesColor(candidate, carrier)` compares the candidate to the
    /// carrier: a green candidate shares a color with a green carrier; a black
    /// one does not. The candidate is parameter zero, the carrier parameter
    /// one (the `watcher`).
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

    /// FIXTURE — a nested per-candidate predicate reading the OUTER candidate.
    /// ADR law 1: every per-candidate predicate is a region, and a nested one
    /// declares its own candidate at register 0 with the enclosing candidate as
    /// a capture — so the depth-0 restriction is gone and an inner predicate
    /// can compare each inner candidate to the outer one.
    ///
    /// The filter is "a creature, if some OTHER creature exists": the inner
    /// predicate ranges over every creature and excludes the outer candidate by
    /// its captured register. With one creature on the battlefield nothing
    /// matches; with two, both do.
    #[test]
    fn a_nested_predicate_reads_the_enclosing_candidate() {
        use deckmaste_core::Cmp;
        use deckmaste_core::Condition;
        use deckmaste_core::Count;
        use deckmaste_core::Countable;
        use deckmaste_core::Reference;

        /// The enclosing candidate as seen from the inner filter region: the
        /// inner candidate takes register 0 and captures the `Where`
        /// condition's own candidate at register 1.
        const ENCLOSING_CANDIDATE: deckmaste_core::RefId = deckmaste_core::RefId(1);

        let inner = Arc::new(deckmaste_core::Region::new(
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
                    provenance: deckmaste_core::Provenance::Capture(deckmaste_core::RefId(0)),
                },
            ]),
            Predicate::And(
                vec![
                    Predicate::creature(),
                    Predicate::State(StatePredicate::InZone(Zone::Battlefield)),
                    Predicate::Not(Arc::new(Predicate::Ref(Reference::Reg(
                        ENCLOSING_CANDIDATE,
                    )))),
                ]
                .into(),
            ),
        ));
        let another_creature_exists = Predicate::And(
            vec![
                Predicate::creature(),
                Predicate::State(StatePredicate::InZone(Zone::Battlefield)),
                Predicate::Where(candidate_region_with_carrier(Condition::Compare(
                    Count::CountOf(Countable::Objects(inner)),
                    Cmp::AtLeast,
                    Count::Literal(1),
                ))),
            ]
            .into(),
        );

        // One creature: no OTHER creature exists, so it fails its own filter.
        let (state, lone) = game_with_a_bear_on_the_field();
        let carrier = Some(state.objects.obj(lone).source);
        assert!(
            !matches_with(&state, lone, &another_creature_exists, carrier),
            "the sole creature is excluded by its own captured register"
        );

        // Three creatures: each one has two others, so each matches.
        let (state, courser, bears, hunter) = mentor_board();
        let carrier = Some(state.objects.obj(courser).source);
        let admitted = candidates_with(&state, &another_creature_exists, carrier);
        for id in [courser, bears, hunter] {
            assert!(
                admitted.contains(&id),
                "every creature has another creature besides itself"
            );
        }
    }

    /// Regression (`engine-derived-reference-in-filters`): a DERIVED reference
    /// in a filter — the shape that reads the controller of something rather
    /// than the thing itself — resolves against the same carrier the bare
    /// register read uses ([CR#109.5]). Before the fix every derived
    /// `Reference` fell through to a `debug_assert!(false)`, so the natural
    /// spelling of "controlled by the same player as X" aborted a test build.
    #[test]
    fn derived_controller_ref_in_filter_resolves_against_the_carrier() {
        let (state, bear) = game_with_a_bear_on_the_field();
        let watcher = Some(state.objects.obj(bear).source);
        let controller_of_source = Predicate::Ref(deckmaste_core::Reference::ControllerOf(
            Arc::new(deckmaste_core::Reference::source_parameter()),
        ));

        assert!(
            matches_with(
                &state,
                state.player(PlayerId(0)).object,
                &controller_of_source,
                watcher
            ),
            "the Bear's controller is player 0, so player 0's proxy IS the derived reference"
        );
        assert!(
            !matches_with(
                &state,
                state.player(PlayerId(1)).object,
                &controller_of_source,
                watcher
            ),
            "the other player is not the Bear's controller"
        );
        assert!(
            !matches_with(&state, bear, &controller_of_source, watcher),
            "the object itself is not its own controller — the derivation is a player read"
        );
    }

    /// The two faithful spellings of "controlled by the same player as the
    /// carrier" — the derived reference and the relation-predicate detour the
    /// witness fixtures used while the derived one aborted — must agree on
    /// every candidate.
    #[test]
    fn derived_controller_ref_agrees_with_the_relation_spelling() {
        let (state, bear) = game_with_a_bear_on_the_field();
        let watcher = Some(state.objects.obj(bear).source);
        let derived = Predicate::Relation(RelationPredicate::ControlledBy(Arc::new(
            Predicate::Ref(deckmaste_core::Reference::ControllerOf(Arc::new(
                deckmaste_core::Reference::source_parameter(),
            ))),
        )));
        let relational = Predicate::Relation(RelationPredicate::ControlledBy(Arc::new(
            Predicate::Relation(RelationPredicate::Controls(Arc::new(Predicate::Ref(
                deckmaste_core::Reference::source_parameter(),
            )))),
        )));

        let ids: Vec<ObjectId> = state.objects.iter().map(|o| o.id).collect();
        assert!(ids.len() > 2, "sanity: the fixture has objects to compare");
        for id in ids {
            assert_eq!(
                matches_with(&state, id, &derived, watcher),
                matches_with(&state, id, &relational, watcher),
                "the two spellings disagree on {id:?}"
            );
        }
        assert!(
            matches_with(&state, bear, &derived, watcher),
            "sanity: the carrier is controlled by its own controller, so both spellings hold"
        );
    }

    /// [CR#108.3]: `OwnerOf` derives the owner, which for a card that never
    /// changed hands is the same player as its controller — but reads a
    /// different field, and a player proxy has no owner at all.
    #[test]
    fn derived_owner_ref_in_filter_resolves() {
        let (state, bear) = game_with_a_bear_on_the_field();
        let watcher = Some(state.objects.obj(bear).source);
        let owner_of_source = Predicate::Ref(deckmaste_core::Reference::OwnerOf(Arc::new(
            deckmaste_core::Reference::source_parameter(),
        )));

        assert!(
            matches_with(
                &state,
                state.player(PlayerId(0)).object,
                &owner_of_source,
                watcher
            ),
            "player 0 started the game with the Bear, so player 0 owns it"
        );
        assert!(
            !matches_with(
                &state,
                state.player(PlayerId(1)).object,
                &owner_of_source,
                watcher
            ),
            "player 1 does not own the Bear"
        );
    }

    /// A derivation nested over a PLAYER-valued one fizzles rather than
    /// answering with the player itself: only an object on the stack or the
    /// battlefield has a controller ([CR#109.4]).
    #[test]
    fn derived_ref_over_a_player_proxy_fizzles() {
        let (state, bear) = game_with_a_bear_on_the_field();
        let watcher = Some(state.objects.obj(bear).source);
        let controller_of_controller = Predicate::Ref(deckmaste_core::Reference::ControllerOf(
            Arc::new(deckmaste_core::Reference::ControllerOf(Arc::new(
                deckmaste_core::Reference::source_parameter(),
            ))),
        ));

        for id in state.objects.iter().map(|o| o.id).collect::<Vec<_>>() {
            assert!(
                !matches_with(&state, id, &controller_of_controller, watcher),
                "a player proxy has no controller, so nothing matches: {id:?}"
            );
        }
    }

    /// `Coalesce` takes the first reference in order that resolves — here an
    /// out-of-range register (nothing declares it, and no carrier reading
    /// answers for it) followed by the source parameter.
    #[test]
    fn coalesce_ref_in_filter_takes_the_first_resolvable() {
        let (state, bear) = game_with_a_bear_on_the_field();
        let watcher = Some(state.objects.obj(bear).source);
        let filter = Predicate::Ref(deckmaste_core::Reference::Coalesce(Arc::from(vec![
            deckmaste_core::Reference::Reg(deckmaste_core::RefId(97)),
            deckmaste_core::Reference::source_parameter(),
        ])));

        assert!(
            matches_with(&state, bear, &filter, watcher),
            "the unresolvable register is skipped and the carrier answers"
        );
        assert!(
            !matches_with(&state, state.player(PlayerId(1)).object, &filter, watcher),
            "and it resolves to exactly one object, not to everything"
        );
    }

    /// The never-crash floor that replaced the `debug_assert!(false)`: a
    /// reference the matcher genuinely cannot resolve reads "no match" instead
    /// of aborting the process ([Invalid semantic input
    /// fizzles](../../../docs/decisions/invalid-semantic-input-fizzles.md)).
    /// `Single` needs the selection evaluator's `ExecutionFrame`.
    #[test]
    fn unresolvable_ref_in_filter_fizzles_instead_of_asserting() {
        let (state, bear) = game_with_a_bear_on_the_field();
        let watcher = Some(state.objects.obj(bear).source);
        let unresolvable = [
            Predicate::Ref(deckmaste_core::Reference::Single(Arc::new(
                deckmaste_core::Selection::SelectAll(Arc::new(deckmaste_core::Region::candidate(
                    Predicate::Any,
                ))),
            ))),
            Predicate::Ref(deckmaste_core::Reference::AttachHostOf(Arc::new(
                deckmaste_core::Reference::source_parameter(),
            ))),
        ];
        for filter in &unresolvable {
            assert!(
                !matches_with(&state, bear, filter, watcher),
                "{filter:?} reads no match rather than aborting"
            );
        }
    }
}
