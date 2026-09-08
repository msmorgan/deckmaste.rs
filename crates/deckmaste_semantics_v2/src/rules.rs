//! The rules-as-data tables a plugin authors beside its cards: state-based
//! actions, conferrals, damage results, and the predefined-token catalog.
//! Mirrors `lean/Semantics/Rules.lean`.
//!
//! They are ordinary semantics-language RON, read through the same expander as
//! cards and tokens (`docs/decisions/semantics-v2.md` §11), and the Lean
//! `Semantics.Check.Rules` is what checks them — this crate refuses nothing.
//!
//! Two field types the crate ticket left narrower than v1's are confirmed here
//! rather than widened, each because the Lean shape it mirrors already covers
//! the meaning:
//!
//! - [`ConferralRule::confer`] is an [`Ability`], where v1 wrote a `Property`.
//!   Every conferral the rules define gives an ability — a planeswalker's
//!   entry replacement [CR#306.5b], a basic land type's mana ability
//!   [CR#305.6] — and `Ability` already spans the static, activated, triggered
//!   and keyword forms a `Property` split apart, so a second type would be a
//!   twin of `Ability::Static` rather than a wider one.
//! - [`DamageResultRule::remove`] is a [`CounterKind`], where v1 wrote a
//!   `CounterRef` resolved against a declared counter registry. `CounterKind`
//!   IS that key: the Lean checker reads `Named`'s label against the generated
//!   counter facts, refusing an undeclared label and requiring the counter's
//!   declared holder to be the kind the recipient binds.

use macro_ron::Expand;
use serde::Deserialize;
use serde::Serialize;

use crate::abilities::Ability;
use crate::abilities::CharacteristicBundle;
use crate::abilities::Instruction;
use crate::abilities::StaticSpec;
use crate::phrase::Condition;
use crate::phrase::Predicate;
use crate::words::CardType;
use crate::words::CounterKind;
use crate::words::DesignationLabel;
use crate::words::DesignationScope;
use crate::words::Kind;
use crate::words::RoomHalf;
use crate::words::Subtype;
use crate::words::TurnPart;
use crate::words::Zone;

/// A rules-defined state-based action ([CR#704.1]), authored under
/// `rules/sba/`. Read it as: *for every object or player matching `scope`,
/// with the discourse bound to that object, if `when` holds the engine
/// performs `then`*.
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

/// One entry of the predefined-token catalog ([CR#111.10]), authored under
/// `tokens/`: the name an effect creating the token writes ("create a Treasure
/// token") and the characteristics the rules define it with. The name is the
/// file's stem; the characteristics are the file's contents, which is the same
/// [`CharacteristicBundle`] a written token spec carries [CR#111.3].
#[derive(Debug, Clone, PartialEq, Eq, Hash, Serialize)]
pub struct PredefinedToken {
    pub name: String,
    pub token: CharacteristicBundle,
}

/// What a registry definition supplies to everything it names
/// ([CR#113.12,305.6]). The scope is the definition itself — the counter's
/// bearer, the subtype's object — so a row here writes no predicate, where a
/// [`ConferralRule`] under `rules/grant/` must.
///
/// The four flavors are the four things the rules do with a definition:
/// an intrinsic ability of the object [CR#305.6]; a continuous property with
/// no ability behind it [CR#113.12]; a state-based action [CR#704.1]; and a
/// turn-based action [CR#703.1].
#[derive(Debug, Clone, PartialEq, Eq, Hash, Deserialize, Expand, Serialize)]
pub enum Conferral {
    Ability { confer: Ability },
    Property { spec: StaticSpec },
    StateBased { when: Condition, then: Instruction },
    TurnBased { part: TurnPart, then: Instruction },
}

/// What one registry declaration MEANS: the rules content the declared name
/// stands for, and the body every `plugins_v2/builtin/macros/{counter_kinds,
/// subtypes,designations}` declaration carries.
///
/// `cargo xtask facts generate` writes `lean/Semantics/Check/Facts.lean` from
/// these: a counter definition's `Named` kind is a [`crate::facts::CounterFacts`]
/// row, a subtype definition a [`crate::facts::SubtypeFacts`] row, a
/// designation definition a [`crate::facts::DesignationFacts`] row. A counter
/// named by a `CounterKind` constructor of its own — a +X/+Y counter
/// [CR#122.1a] or a keyword counter [CR#122.1b] — contributes no row to the
/// table of named counters, because nothing looks it up by label.
#[derive(Debug, Clone, PartialEq, Eq, Hash, Deserialize, Expand, Serialize)]
pub enum Definition {
    Counter {
        kind: CounterKind,
        holder: Kind,
        confers: Vec<Conferral>,
    },
    Subtype {
        subtype: Subtype,
        rules: Vec<Conferral>,
    },
    Designation {
        label: DesignationLabel,
        scope: DesignationScope,
        effectful: bool,
        zone: Option<Zone>,
        r#type: Option<CardType>,
        half: Option<RoomHalf>,
    },
}

/// The three tables a plugin's `rules/` directory defines, concatenated across
/// its files. The predefined-token catalog is separate, because its entries
/// are named and a plugin writes them under `tokens/`.
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
