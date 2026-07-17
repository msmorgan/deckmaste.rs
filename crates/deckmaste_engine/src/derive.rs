//! Minimal characteristics derivation: what abilities an object has. The
//! seed of the stage-4 layer system — in the skeleton, the face's intrinsic
//! printed abilities only; type/subtype-conferred abilities ([CR#305.6] falls
//! out of the data; the engine never special-cases land subtypes) are folded
//! in later, per pass, by `layer::fold_conferred_abilities`.

use deckmaste_core::Ability;
use deckmaste_core::Action;
use deckmaste_core::Card;
use deckmaste_core::CardFace;
use deckmaste_core::ColorOrColorless;
use deckmaste_core::CostComponent;
use deckmaste_core::Count;
use deckmaste_core::ManaSpec;
use deckmaste_core::OneShotEffect;
use deckmaste_core::PlayerAction;
use deckmaste_core::Uint;

use crate::object::ObjectId;
use crate::object::ObjectSource;
use crate::state::GameState;

/// The face an object presents. Skeleton: the front face.
#[must_use]
pub fn face(card: &Card) -> &CardFace {
    match card {
        Card::Normal(f) | Card::TwoFaced { front: f, .. } => f,
    }
}

/// A face's INTRINSIC printed abilities only (`face.abilities`) — NOT
/// type/subtype conferrals. Type/subtype conferral is re-derived from the
/// object's CURRENT characteristics each layer pass by
/// `layer::fold_conferred_abilities`, not cached here — which is what makes
/// conferral track layer-4 type/subtype changes, both ADDs and REMOVALS.
/// (Conferred abilities still arrive wrapped in `Ability::Innate` via the
/// ONE emission path, [`Property::conferred_ability`] — a rule of the object
/// ([CR#305.6,113.12]): a basic land that loses all abilities still taps for
/// its color, and card-facing ability queries don't see the conferral — that
/// now happens in the fold, not here.) `Action::ActivateAbility` indexes the
/// [`usable_abilities`] view of this list. Computed once per card at setup
/// (`Cards::push`) and cached on the `CardInstance`.
#[must_use]
pub(crate) fn printed_of_face(face: &CardFace) -> Vec<Ability> {
    face.abilities.clone()
}

/// The object's PRINTED abilities, from the per-card cache.
///
/// This is the cycle-safe base used by the layer pipeline itself
/// (`layer::base_values` and `layer::gather`). External callers should
/// use [`abilities`] to get the layer-6–derived list instead.
///
/// # Panics
///
/// Panics on a player proxy — callers guard on `card_id()` first.
#[must_use]
pub(crate) fn printed_abilities(state: &GameState, id: ObjectId) -> &[Ability] {
    let card = state.objects.obj(id).card_id().expect("card-backed object");
    &state.cards.get(card).printed
}

/// Predicate-scoped ability conferrals ([Task 2], `ConferralRule`): for each
/// rule in `state.conferral_rules` whose `scope` matches `id`
/// (`crate::matches` — the same predicate-scope matcher `global_sba_rules`
/// uses, `sba.rs`), the ability it contributes via
/// [`Property::conferred_ability`] — `Ability::Innate`-wrapped, the SAME
/// emission path subtype conferral uses, so a predicate-scoped conferral is
/// exactly as strip-immune / card-facing-invisible as a subtype's
/// ([CR#305.6,113.12]).
///
/// Deliberately NOT folded into `layer::base_values`/`gather` (the
/// `layers()` computation itself): `crate::matches` resolves a
/// `Characteristic` scope (e.g. `Type(Planeswalker)`) by calling
/// `state.layers()` (`has_type` et al.), so folding conferral rules INSIDE
/// `layers()` would recurse without bound. Callers that assemble an object's
/// derived ability list from an ALREADY-COMPUTED `LayeredView` ([`abilities`],
/// [`usable_abilities`]) fold this in afterward instead — one extra, self-
/// contained `layers()` recompute per matching id, not a cycle.
#[must_use]
pub(crate) fn conferred_rule_abilities(state: &GameState, id: ObjectId) -> Vec<Ability> {
    state
        .conferral_rules
        .iter()
        .filter(|rule| crate::matches(state, id, &rule.scope))
        .filter_map(|rule| rule.confer.conferred_ability())
        .collect()
}

/// The object's USABLE derived abilities after layer 6 — the layer view's
/// list with every `Innate` wrapper PEELED IN PLACE (same length, same
/// order): the indexable surface `Action::ActivateAbility { ability }`
/// points into, shared by `legal_actions`, `decide`'s `ActivateAbility`
/// arm, `begin_activate`, and the render views ("SAME list, SAME order").
/// An `Innate` conferral FUNCTIONS for its own controller ([CR#604.1] — a
/// basic land's conferred mana ability is activatable), it is only
/// invisible to CARD-FACING queries — that filter is [`abilities`].
///
/// Predicate-scoped `ConferralRule` conferrals ([`conferred_rule_abilities`])
/// are folded in here too, alongside the layer view's own list, and peeled
/// the same way.
#[must_use]
pub fn usable_abilities(state: &GameState, id: ObjectId) -> std::sync::Arc<Vec<Ability>> {
    let view = state.layers();
    let derived = &view.get(id).abilities;
    let conferred = conferred_rule_abilities(state, id);
    if conferred.is_empty() && !derived.iter().any(|a| matches!(a, Ability::Innate(_))) {
        return std::sync::Arc::clone(derived);
    }
    std::sync::Arc::new(
        derived
            .iter()
            .chain(conferred.iter())
            .map(|a| a.peel_innate().clone())
            .collect(),
    )
}

/// The object's CARD-FACING derived abilities after layer 6
/// ([CR#305.6,613.1f]): base = intrinsic printed abilities; type/subtype
/// conferrals are folded in at layer 4 (`fold_conferred_abilities`); layer 6
/// applies on top. `Innate` abilities are filtered OUT ([CR#113.12]): they are
/// rules of the object, not abilities other cards can see or count — an object
/// whose only abilities are `Innate` reads here as having none.
///
/// Builds a full `LayeredView` per call — fine for a one-shot read (e.g. at
/// resolution), but NEVER call it in a loop: build `state.layers()` once and
/// index the view instead. The layer pipeline itself uses
/// [`printed_abilities`] internally to break the `layers()` →
/// `derive::abilities` → `layers()` recursion. Engine machinery that must see
/// through `Innate` (the SBA sweep, `attachment_legal`, layer
/// static-application) reads the view's `abilities` directly and peels.
///
/// Predicate-scoped `ConferralRule` conferrals ([`conferred_rule_abilities`])
/// are folded in here too — and, being `Innate`, filtered right back out, the
/// same as a subtype conferral.
#[must_use]
pub fn abilities(state: &GameState, id: ObjectId) -> std::sync::Arc<Vec<Ability>> {
    let view = state.layers();
    let derived = &view.get(id).abilities;
    let conferred = conferred_rule_abilities(state, id);
    if conferred.is_empty() && !derived.iter().any(Ability::is_innate) {
        // No Innate present — return the shared Arc unchanged (the common case).
        return std::sync::Arc::clone(derived);
    }
    std::sync::Arc::new(
        derived
            .iter()
            .chain(conferred.iter())
            .filter(|a| !a.is_innate())
            .cloned()
            .collect(),
    )
}

/// The PRINTED abilities of whatever an `ObjectSource` names — the abilities
/// the trigger scan considers for a watcher. For a card-backed source this is
/// the face's printed list (the same spine that survives reminting and LKI);
/// a player proxy has none. Granted/conferred abilities are folded in by the
/// derived sibling [`derived_abilities_of`] (predicate-scoped conferrals);
/// this printed-only list is the INDEX-STABLE spine that the trigger/
/// replacement scans and their resolution reads key on ([CR#603.2]).
///
/// Composite keywords (ward, prowess) are spliced INLINE: the engine
/// executes the abilities a `KeywordAbility::Composite` carries, so the
/// trigger scan, placement, and resolution all index one flat space.
/// Intrinsics and other keyword shapes pass through untouched.
#[must_use]
pub fn abilities_of_source(state: &GameState, source: ObjectSource) -> Vec<Ability> {
    match source {
        ObjectSource::Card(card) => {
            let printed = &face(&state.cards.get(card).def).abilities;
            let mut out = Vec::with_capacity(printed.len());
            for ability in printed {
                flatten_composites(ability, &mut out);
            }
            out
        }
        ObjectSource::Player(_) => vec![],
    }
}

/// The engine-internal ability enumeration of `source`, EXTENDED with the
/// abilities conferred onto the live object `id` — the DERIVED counterpart of
/// [`abilities_of_source`] that the replacement gather and trigger scan read so
/// a CONDITIONALLY-conferred replacement/triggered ability participates even
/// off the battlefield ([CR#603.2,616.1]). The result is three concatenated
/// sections:
///
/// 1. the PRINTED prefix — byte-for-byte `abilities_of_source(source)`, so a
///    printed ability keeps its enumeration index (the fired-trigger `(source,
///    index)` re-read at resolution stays valid);
/// 2. the predicate-scoped `ConferralRule` tail ([`conferred_rule_abilities`]);
/// 3. the LAYER-DERIVED tail — abilities the [CR#613] layer pipeline GRANTED
///    onto `id` beyond its printed base (`state.layers().get(id).abilities`
///    minus the printed prefix the view seeds from `instance.printed`).
///
/// Sections 2 and 3 both follow the index-stable prefix; they carry no
/// index-stable identity (a fired conferred trigger carries its body by value)
/// and are `Innate`-peeled + composite-spliced the same way. The returned
/// `usize` is the length of section 1 only.
///
/// The layer-derived tail is what makes Falkenrath Gorger's printed static
/// ("Each Vampire creature card you own that isn't on the battlefield has
/// madness") FUNCTION: the layer view scopes it exactly — the static is active
/// only while the Gorger is on the battlefield ([CR#611.3b]; `layer::gather`
/// sources statics from battlefield permanents only), and its `Owner(Ref(You))`
/// gate reaches precisely the Gorger controller's OWN off-battlefield Vampires
/// (the `Floating` scope spans objects in every zone). So the granted `Madness`
/// keyword lands on those cards' derived ability lists, and its spliced
/// self-replacement opens the [CR#616.1] discard window here — no companion
/// global `ConferralRule` (which has no source object and so cannot express the
/// owner/Gorger gate) is needed.
///
/// `id` is the live object carrying `source`; `None` (a gone/leaving object,
/// which has no live conferrals or grants) yields the printed list unchanged.
/// Reaches OFF-battlefield objects: both tails derive every card-backed object
/// in ANY zone (the layer base map is not battlefield-only), so an in-hand
/// card's conferred/granted replacement/trigger is enumerated here where
/// `abilities_of_source` alone would miss it.
///
/// PERF: builds `state.layers()` (and, for a game with `ConferralRule`s, the
/// conferral-scope match builds it again) on every call. The gather/scan are
/// hot paths, so this is a real per-event cost — accepted under the project's
/// perf-last priority; no caching added here.
#[must_use]
pub(crate) fn derived_abilities_of(
    state: &GameState,
    id: Option<ObjectId>,
    source: ObjectSource,
) -> (Vec<Ability>, usize) {
    let mut out = abilities_of_source(state, source);
    let printed_len = out.len();
    if let Some(id) = id {
        // Section 2: predicate-scoped `ConferralRule` grants — a global-rule
        // mechanism SEPARATE from the layer system (never folded into
        // `layers()`), so it does not overlap section 3.
        for conferred in conferred_rule_abilities(state, id) {
            flatten_composites(&conferred, &mut out);
        }
        // Section 3: the layer-derived tail. Skip the printed prefix the layer
        // view seeds from `instance.printed` (it is already section 1 in `out`,
        // via `abilities_of_source`) and fold only the appended layer additions
        // — Gorger-style `GainAbility` grants (and any layer-4 type/subtype
        // conferrals). The skip count is the UNFLATTENED printed length, matching
        // the view's unflattened prefix; a shorter derived list (an ability-
        // stripping layer op) simply yields no tail (never-crash). Calling
        // `layers()` here is not a cycle: `derived_abilities_of` is invoked only
        // by the replacement gather and trigger scan, never from inside the
        // layer pipeline, and the pipeline's own matcher (`matches_derived`)
        // never re-enters `layers()`.
        if let Some(chars) = state.layers().try_get(id) {
            for granted in chars.abilities.iter().skip(printed_base_len(state, id)) {
                flatten_composites(granted, &mut out);
            }
        }
    }
    (out, printed_len)
}

/// The count of a live object's UNFLATTENED printed abilities — the length of
/// the prefix `layer::base_values` seeds the derived ability list from
/// (`instance.printed`) before any layer op appends. [`derived_abilities_of`]
/// skips exactly this many entries of the layer view to isolate the
/// layer-granted tail. A non-card-backed / absent id contributes 0.
fn printed_base_len(state: &GameState, id: ObjectId) -> usize {
    state
        .objects
        .get(id)
        .and_then(crate::object::GameObject::card_id)
        .map_or(0, |card| state.cards.get(card).printed.len())
}

/// Splice a composite keyword's members into `out` (recursively — a
/// composite may carry another); any other ability passes through as-is.
///
/// `Innate` is PEELED here ([CR#113.12,604.1]): this is the engine-internal
/// enumeration (trigger gathering, the `Has`/keyword reads), which sees the
/// inner ability, not the wrapper — so an `Innate(Triggered)`/`Innate(Keyword)`
/// is enumerated like its bare form. The card-facing FILTER lives in
/// `derive::abilities` (the invisibility surface), not here.
/// `abilities_of_source` is indexed self-consistently on both ends (trigger
/// placement + resolution read the same flattened list), so peeling keeps it
/// consistent.
pub(crate) fn flatten_composites(ability: &Ability, out: &mut Vec<Ability>) {
    // Look through a remembered Ability-kind macro invocation (`Chapter`,
    // [CR#714.2b]) — the engine enumerates its expansion, exactly like a
    // keyword's `Expanded` is looked through in `composite_members`.
    if let Ability::Expanded(e) = ability {
        flatten_composites(&e.value, out);
        return;
    }
    // Peel any `Innate` wrapper first, then re-dispatch on the inner ability
    // (which may itself be a composite keyword to splice).
    if let Ability::Innate(inner) = ability {
        flatten_composites(inner, out);
        return;
    }
    if let Ability::Keyword(k) = ability
        && let Some(members) = composite_members(k)
    {
        for member in members {
            flatten_composites(member, out);
        }
        return;
    }
    out.push(ability.clone());
}

/// The member list of a composite keyword, looked up through the remembered
/// macro invocation; `None` for intrinsics and other keyword shapes.
fn composite_members(keyword: &deckmaste_core::KeywordAbility) -> Option<&Vec<Ability>> {
    match keyword {
        deckmaste_core::KeywordAbility::Expanded(e) => composite_members(&e.value),
        deckmaste_core::KeywordAbility::Composite { abilities, .. } => Some(abilities),
        _ => None,
    }
}

/// Skeleton-subset mana-ability check (a subset of [CR#605.1a]): an activated
/// ability with no targets, cost exactly `[Tap]`, producing a fixed amount
/// of specific mana. Full [CR#605.1a] admits more (other costs, `AnyColor`,
/// loyalty exclusion) — not yet needed here. Returns what it produces.
/// Keyword wrappers are looked through.
#[must_use]
pub fn tap_mana_ability(ability: &Ability) -> Option<(ColorOrColorless, Uint)> {
    match ability {
        Ability::Activated(a)
            if crate::resolve::top_targets(&a.effect).is_empty()
                && a.cost.as_slice() == [CostComponent::Tap] =>
        {
            match &a.effect {
                // The produced-mana effect is a bare `AddMana` in RON, which
                // reads as `By(You, AddMana(…))` (the implicit-you default);
                // the agent is irrelevant for tap-for-mana derivation.
                OneShotEffect::Act(Action::By(
                    _,
                    PlayerAction::AddMana(
                        Count::Literal(n),
                        deckmaste_core::ManaProduction::Bare(ManaSpec::Specific(m)),
                    ),
                )) => Some((*m, *n)),
                _ => None,
            }
        }
        Ability::Expanded(e) => tap_mana_ability(&e.value),
        _ => None,
    }
}

#[cfg(test)]
mod tests {
    use std::sync::Arc;

    use deckmaste_core::Ability;
    use deckmaste_core::Card;
    use deckmaste_core::CardFace;
    use deckmaste_core::EventFilter;
    use deckmaste_core::OneShotEffect;
    use deckmaste_core::Reference;
    use deckmaste_core::TriggeredAbility;
    use deckmaste_core::Zone;

    use crate::object::ObjectSource;
    use crate::player::PlayerId;
    use crate::state::GameConfig;
    use crate::state::GameState;
    use crate::state::PlayerConfig;
    use crate::state::StartingPlayer;

    fn game() -> GameState {
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

    /// [CR#113.12,604.1]: `abilities_of_source` (the engine-internal
    /// enumeration the trigger scan consumes) PEELS `Innate` — an
    /// `Innate(Triggered(...))` is enumerated as the bare `Triggered`, so the
    /// trigger gathering sees it. (The card-facing FILTER in
    /// `derive::abilities` is a separate surface and is unaffected.)
    #[test]
    fn abilities_of_source_peels_innate_triggered() {
        let mut state = game();
        let trigger = TriggeredAbility {
            ability_word: None,
            where_x: None,
            from: None,
            event: EventFilter::ZoneChange {
                what: deckmaste_core::Predicate::Ref(Reference::This),
                from: None,
                to: Some(Zone::Graveyard),
                cause: None,
            },
            condition: None,
            limits: vec![],
            effect: OneShotEffect::draw(Reference::You, deckmaste_core::Count::Literal(1)),
        };
        let card = Card::Normal(CardFace {
            name: "Innate Triggerer".into(),
            abilities: vec![Ability::Innate(Box::new(Ability::Triggered(
                trigger.clone(),
            )))],
            ..CardFace::default()
        });
        let card_id = state.cards.push(Arc::new(card), PlayerId(0));

        let derived = super::abilities_of_source(&state, ObjectSource::Card(card_id));
        assert_eq!(
            derived,
            vec![Ability::Triggered(trigger)],
            "abilities_of_source must peel Innate so the trigger scan sees the \
             Triggered ability (not the opaque Innate wrapper)"
        );
    }

    /// [Task 2]: a `ConferralRule { scope: Type(Planeswalker), confer:
    /// Property::Ability(...) }` folds into a MATCHING object's derived
    /// ability list ([CR#305.6] pattern, extended from a static subtype to a
    /// live predicate scope) — and NOT into a non-matching object's.
    #[test]
    fn conferred_ability_by_type() {
        use deckmaste_core::CharacteristicPredicate;
        use deckmaste_core::ConferralRule;
        use deckmaste_core::KeywordAbility;
        use deckmaste_core::Predicate;
        use deckmaste_core::Property;
        use deckmaste_core::Type;

        let mut state = game();
        state.conferral_rules = vec![ConferralRule {
            scope: Predicate::Characteristic(CharacteristicPredicate::Type(
                Type::Planeswalker.name(),
            )),
            confer: Property::Ability(Box::new(Ability::Keyword(KeywordAbility::Trample))),
        }];

        let walker = Card::Normal(CardFace {
            name: "Test Walker".into(),
            types: vec![Type::Planeswalker.def()],
            ..CardFace::default()
        });
        let walker_card = state.cards.push(Arc::new(walker), PlayerId(0));
        let walker_id = state.objects.mint(
            ObjectSource::Card(walker_card),
            PlayerId(0),
            Some(Zone::Battlefield),
        );
        state.zones.battlefield.push(walker_id);

        let bear = Card::Normal(CardFace {
            name: "Test Bear".into(),
            types: vec![Type::Creature.def()],
            ..CardFace::default()
        });
        let bear_card = state.cards.push(Arc::new(bear), PlayerId(0));
        let bear_id = state.objects.mint(
            ObjectSource::Card(bear_card),
            PlayerId(0),
            Some(Zone::Battlefield),
        );
        state.zones.battlefield.push(bear_id);

        assert!(
            super::usable_abilities(&state, walker_id)
                .iter()
                .any(|a| matches!(a, Ability::Keyword(KeywordAbility::Trample))),
            "the planeswalker gains the type-scoped conferred ability"
        );
        assert!(
            !super::usable_abilities(&state, bear_id)
                .iter()
                .any(|a| matches!(a, Ability::Keyword(KeywordAbility::Trample))),
            "a non-planeswalker does not gain the type-scoped conferred ability"
        );
    }
}
