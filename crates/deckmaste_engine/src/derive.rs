//! Minimal characteristics derivation: what abilities an object has. The
//! seed of the stage-4 layer system — in the skeleton, a flat concatenation
//! of printed face abilities and subtype-conferred abilities ([CR#305.6] falls
//! out of the data; the engine never special-cases land subtypes).

use deckmaste_core::Ability;
use deckmaste_core::Action;
use deckmaste_core::Card;
use deckmaste_core::CardFace;
use deckmaste_core::ColorOrColorless;
use deckmaste_core::CostComponent;
use deckmaste_core::Count;
use deckmaste_core::Effect;
use deckmaste_core::ManaSpec;
use deckmaste_core::PlayerAction;
use deckmaste_core::Property;
use deckmaste_core::Uint;

use crate::object::ObjectId;
use crate::object::ObjectSource;
use crate::state::GameState;

/// The face an object presents. Skeleton: the front face.
#[must_use]
pub fn face(card: &Card) -> &CardFace {
    match card {
        Card::Normal(f) | Card::ModalDfc(f, _) => f,
    }
}

/// A face's PRINTED abilities: printed plus subtype-conferred (the
/// `Property::Ability` arm; other flavors execute elsewhere), in that
/// order. `Action::ActivateAbility` indexes this list. Computed once per
/// card at setup (`Cards::push`) and cached on the `CardInstance`.
#[must_use]
pub(crate) fn printed_of_face(face: &CardFace) -> Vec<Ability> {
    face.abilities
        .iter()
        .cloned()
        .chain(face.subtypes.iter().flat_map(|s| {
            s.confers.iter().filter_map(|p| match p {
                Property::Ability(a) => Some((**a).clone()),
                _ => None,
            })
        }))
        .collect()
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

/// The object's CARD-FACING derived abilities after layer 6
/// ([CR#305.6,613.1f]): base = printed + subtype-conferred; layer 6 applies
/// on top. `Innate` abilities are filtered OUT ([CR#113.12]): they are rules
/// of the object, not abilities other cards can see or count — an object
/// whose only abilities are `Innate` reads here as having none.
///
/// Builds a full `LayeredView` per call — fine for a one-shot read (e.g. at
/// resolution), but NEVER call it in a loop: build `state.layers()` once and
/// index the view instead. The layer pipeline itself uses
/// [`printed_abilities`] internally to break the `layers()` →
/// `derive::abilities` → `layers()` recursion. Engine machinery that must see
/// through `Innate` (the SBA sweep, `attachment_legal`, layer
/// static-application) reads the view's `abilities` directly and peels.
#[must_use]
pub fn abilities(state: &GameState, id: ObjectId) -> std::sync::Arc<Vec<Ability>> {
    let view = state.layers();
    let derived = &view.get(id).abilities;
    if derived.iter().any(Ability::is_innate) {
        std::sync::Arc::new(derived.iter().filter(|a| !a.is_innate()).cloned().collect())
    } else {
        // No Innate present — return the shared Arc unchanged (the common case).
        std::sync::Arc::clone(derived)
    }
}

/// The PRINTED abilities of whatever an `ObjectSource` names — the abilities
/// the trigger scan considers for a watcher. For a card-backed source this is
/// the face's printed list (the same spine that survives reminting and LKI);
/// a player proxy has none. Granted/conferred abilities are a later stage
/// ([CR#603.2] watching abilities; Stage 3 has no continuous effects).
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
                Effect::Act(Action::By(
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
    use deckmaste_core::Effect;
    use deckmaste_core::Event;
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
            counter_decls: std::collections::HashMap::new(),
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
            event: Event::ZoneMove {
                what: deckmaste_core::Filter::Ref(Reference::This),
                from: None,
                to: Some(Zone::Graveyard),
                face: None,
                cause: None,
            },
            condition: None,
            limits: vec![],
            effect: Effect::Act(deckmaste_core::Action::By(
                Reference::You,
                deckmaste_core::PlayerAction::Draw(deckmaste_core::Count::Literal(1)),
            )),
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
}
