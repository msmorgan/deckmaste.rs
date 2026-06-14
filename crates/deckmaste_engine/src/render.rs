//! Renderer-facing queries ([CR#601],[CR#602],[CR#605.1a]): public, read-only
//! views of the legal options at a decision point, so an in-process UI can
//! present them without re-deriving legality or reaching into engine internals.
//! Enabler for `tui-decision-actions`. No legality logic lives here — these
//! wrap the same derivations `legal_actions` uses, indexed identically.

use std::sync::Arc;

use deckmaste_core::Ability;
use deckmaste_core::ActivatedAbility;
use deckmaste_core::ColorOrColorless;
use deckmaste_core::Uint;

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
