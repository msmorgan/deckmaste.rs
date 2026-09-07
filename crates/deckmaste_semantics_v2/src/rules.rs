//! The three rules tables a plugin's `rules/` directory carries: state-based
//! actions, conferrals, and damage results.
//!
//! These have **no Lean declaration yet** — the workbench models a card, not
//! the rules-as-data tables the engine sweeps. `lean-rules-tables` adds them;
//! until it does, each type here is the minimal record of exactly the fields
//! today's `plugins/builtin/rules/*.ron` files carry, retyped onto the v2
//! grammar. Nothing is invented: no `when` on a conferral, no count on a
//! damage result, no fields a file does not write. When the Lean lands, these
//! join the mirror and the drift test, and any difference between the Lean
//! shape and this one is resolved in the Lean's favour.
//!
//! Two field TYPES are narrower than v1's, and deliberately so — both are
//! `lean-rules-tables`'s to widen or confirm, not this crate's:
//!
//! - [`ConferralRule::confer`] is an [`Ability`], where v1 wrote a `Property`
//!   (an ability, a continuous effect, a state-based or a turn-based one). The
//!   builtin table writes an ability and v2 has no `Property`, so the wider
//!   type would be invented here rather than mirrored.
//! - [`DamageResultRule::remove`] is a [`CounterKind`], where v1 wrote a
//!   `CounterRef` — an identity resolved against the plugin's declared counter
//!   registry. v2 has no such registry type, so a named counter is carried as
//!   the label `CounterKind::Named` holds and the binding to a declaration is
//!   not made here.
//!
//! They are ordinary semantics-language RON, read through the same expander as
//! cards and tokens (`docs/decisions/semantics-v2.md` §11).

use macro_ron::Expand;
use serde::Deserialize;
use serde::Serialize;

use crate::abilities::Ability;
use crate::abilities::Instruction;
use crate::phrase::Condition;
use crate::phrase::Predicate;
use crate::words::CounterKind;

/// A rules-defined state-based action ([CR#704.1]), authored under
/// `rules/sba/`. Read it as: *for every object matching `scope`, with the
/// discourse bound to that object, if `when` holds the engine performs
/// `then`*.
#[derive(Debug, Clone, PartialEq, Eq, Hash, Deserialize, Expand, Serialize)]
pub struct SbaRule {
    pub scope: Predicate,
    pub when: Condition,
    pub then: Instruction,
}

/// A rules-defined conferral, authored under `rules/grant/`. Read it as:
/// *every object matching `scope` has `confer`* — the same `Predicate`-scoped
/// shape as [`SbaRule`]'s `scope` with no `when`/`then` gate, because the
/// conferred ability carries its own conditionality (a replacement's trigger
/// event, an intervening if). [CR#306.5b] is the one the builtin table writes.
#[derive(Debug, Clone, PartialEq, Eq, Hash, Deserialize, Expand, Serialize)]
pub struct ConferralRule {
    pub scope: Predicate,
    pub confer: Ability,
}

/// A rules-defined damage result, authored under `rules/damage/`. Read it as:
/// *when damage is dealt to a permanent matching `recipient`, that many
/// `remove` counters are taken off it* — [CR#120.3c] for planeswalker loyalty,
/// [CR#120.3h] for battle defense. The removed count is always the damage
/// event's own amount ("that many"), so no count field is written.
#[derive(Debug, Clone, PartialEq, Eq, Hash, Deserialize, Expand, Serialize)]
pub struct DamageResultRule {
    pub recipient: Predicate,
    pub remove: CounterKind,
}

/// The three tables a plugin's `rules/` directory defines, concatenated across
/// its files.
#[derive(Debug, Clone, Default, PartialEq, Eq)]
pub struct RulesTables {
    /// `rules/sba/` — evaluated globally by the engine's state-based-action
    /// sweep [CR#704.3].
    pub sba: Vec<SbaRule>,
    /// `rules/grant/` — applied alongside a card's own printed and conferred
    /// abilities.
    pub conferral: Vec<ConferralRule>,
    /// `rules/damage/` — applied at deal time, in addition to any intrinsic
    /// result [CR#120.3].
    pub damage_result: Vec<DamageResultRule>,
}
