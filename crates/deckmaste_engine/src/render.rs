//! Renderer-facing queries ([CR#601],[CR#602],[CR#605.1a]): public, read-only
//! views of the legal options at a decision point, so an in-process UI can
//! present them without re-deriving legality or reaching into engine internals.
//! Enabler for `tui-decision-actions`. No legality logic lives here — these
//! wrap the same derivations `legal_actions` uses, indexed identically.

use std::sync::Arc;

use deckmaste_core::Ability;
use deckmaste_core::ActivatedAbility;
use deckmaste_core::Card;
use deckmaste_core::ColorOrColorless;
use deckmaste_core::ManaCost;
use deckmaste_core::Uint;

use crate::decide::Action;
use crate::object::ObjectId;
use crate::state::GameState;

impl GameState {
    /// The derived abilities of `object` after the layer pipeline — the SAME
    /// list, in the SAME order, that `Action::ActivateAbility { ability }`
    /// indexes ([CR#613.1f]). A renderer resolves an offered activation with
    /// `state.abilities(object)[ability]`.
    #[must_use]
    pub fn abilities(&self, object: ObjectId) -> Arc<Vec<Ability>> {
        crate::derive::abilities(self, object)
    }

    /// The activated ability at `index` in `object`'s derived list, looking
    /// through macro/keyword wrappers; `None` if that index is not an activated
    /// ability ([CR#602.1a]). Cloned so the caller need not hold the `Arc`.
    #[must_use]
    pub fn activated_ability(&self, object: ObjectId, index: usize) -> Option<ActivatedAbility> {
        let abilities = self.abilities(object);
        crate::activate::as_activated(abilities.get(index)?).cloned()
    }

    /// If the ability at `index` is a (skeleton-subset) mana ability
    /// ([CR#605.1a]), what it produces: the single colour/colourless and the
    /// fixed amount. `None` for non-mana abilities.
    #[must_use]
    pub fn mana_ability(&self, object: ObjectId, index: usize) -> Option<(ColorOrColorless, Uint)> {
        let abilities = self.abilities(object);
        crate::derive::tap_mana_ability(abilities.get(index)?)
    }
}

/// A renderable description of one legal priority [`Action`] — everything a UI
/// needs to present it, fetched in one call. A query result (the `Action` is
/// echoed for submission), NOT a stored or serialized record.
#[derive(Debug, Clone)]
pub struct ActionView<'a> {
    /// The action to submit back via `Decision::Act` if the user picks this.
    pub action: Action,
    /// The object the action concerns, if any (`None` for Pass/Concede).
    pub source: Option<ObjectId>,
    /// The source object's face name, borrowed from the game state.
    pub name: Option<&'a str>,
    /// The per-kind render payload.
    pub kind: ActionViewKind,
}

/// The render payload of an [`ActionView`], by action kind.
#[derive(Debug, Clone)]
pub enum ActionViewKind {
    /// Pass priority.
    Pass,
    /// Concede ([CR#104.3a]).
    Concede,
    /// Play a land ([CR#116.2a]).
    PlayLand,
    /// Cast a spell ([CR#601]); `cost` is its mana cost (`None` for a
    /// no-mana-cost face).
    Cast { cost: Option<ManaCost> },
    /// Activate an ability ([CR#602]); `mana` flags a mana ability
    /// ([CR#605.1a]). `ability` is boxed: it dwarfs every other variant, so
    /// inlining it would bloat the whole enum (`clippy::large_enum_variant`).
    Activate {
        ability: Box<ActivatedAbility>,
        mana: bool,
    },
}

impl GameState {
    /// A renderable description of a legal priority `action` — name, cost, and
    /// ability bundled so a UI presents it without re-deriving anything. Built
    /// from the public query methods above; adds no legality logic.
    ///
    /// # Panics
    ///
    /// Panics on an `ActivateAbility` whose `ability` index is not an activated
    /// ability of `object` — i.e. an action that was never in a legal list.
    /// `describe_action` is meant to render actions the engine offered; the
    /// indexing contract there is `abilities(object)[ability]` ([CR#602.1a]).
    /// Also panics on `Action::Special` (a P0.W3 shell never enumerated).
    #[must_use]
    pub fn describe_action(&self, action: &Action) -> ActionView<'_> {
        let name = |id: ObjectId| -> Option<&str> {
            match self.def(id) {
                Card::Normal(f) | Card::ModalDfc(f, _) => Some(f.name.as_str()),
            }
        };
        match *action {
            Action::Pass => ActionView {
                action: action.clone(),
                source: None,
                name: None,
                kind: ActionViewKind::Pass,
            },
            Action::Concede => ActionView {
                action: action.clone(),
                source: None,
                name: None,
                kind: ActionViewKind::Concede,
            },
            Action::PlayLand { object } => ActionView {
                action: action.clone(),
                source: Some(object),
                name: name(object),
                kind: ActionViewKind::PlayLand,
            },
            Action::CastSpell { object } => ActionView {
                action: action.clone(),
                source: Some(object),
                name: name(object),
                kind: ActionViewKind::Cast {
                    cost: self.mana_cost(object),
                },
            },
            Action::ActivateAbility { object, ability } => {
                // One layer build for both reads (mana flag + the ability itself).
                let abilities = self.abilities(object);
                let entry = abilities.get(ability);
                let mana = entry.and_then(crate::derive::tap_mana_ability).is_some();
                let act = entry
                    .and_then(crate::activate::as_activated)
                    .cloned()
                    .expect("ActivateAbility indexes an activated ability");
                ActionView {
                    action: action.clone(),
                    source: Some(object),
                    name: name(object),
                    kind: ActionViewKind::Activate {
                        ability: Box::new(act),
                        mana,
                    },
                }
            }
            // Special actions ([CR#116.2]) are a P0.W3 shell never offered in a
            // legal list, so describe_action is never called on one. Mirror the
            // engine's loud seam rather than inventing a render shape for it.
            Action::Special(_) => todo!("P0.W3: special actions are not enumerated yet"),
        }
    }
}
