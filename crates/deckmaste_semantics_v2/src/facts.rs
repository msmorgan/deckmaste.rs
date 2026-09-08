//! The generated registry fact columns. Mirrors
//! `lean/Semantics/Check/FactTypes.lean` constructor-for-constructor and
//! field-for-field; Lean is the specification (see
//! `docs/decisions/semantics-v2.md` §10).
//!
//! These are the columns a registry declaration carries about itself — what
//! `lean/Semantics/Check/Facts.lean` tabulates and the `Check/` laws read.
//! They are not syntax: no card writes one, so no type here is a macro
//! position and none derives `SupportsMacros` or `Expand`. A declaration body
//! that carries them (`plugins_v2/builtin/macros/designations`,
//! `.../counter_kinds`) is read as these types through
//! [`crate::ron::raw_options`].
//!
//! A Lean field with a default (`:= none`, `:= []`, `:= true`) is a
//! `#[serde(default)]` field here, so a declaration writes only the columns
//! that differ from the default.
//!
//! Only the generated tables' row types are here, because only those are
//! mirrored. A fact type the checker owns rather than the registry —
//! `ActFacts` and its columns — is declared beside its hand-written table in
//! `lean/Semantics/Check/Words.lean` and has no Rust mirror at all.

use serde::Deserialize;
use serde::Serialize;

use crate::words::CardType;
use crate::words::DesignationLabel;
use crate::words::DesignationScope;
use crate::words::KeywordLabel;
use crate::words::Kind;
use crate::words::RoomHalf;
use crate::words::Subtype;
use crate::words::Zone;

/// What a subtype declares about the card frame it sits on: a Saga's chapter
/// frame [CR#714.1], an Adventure's inset [CR#715.1], a Room's doors
/// [CR#709.5j].
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, Deserialize, Serialize)]
pub enum FrameFeature {
    Chapters,
    AdventureInset,
    Doors,
}

/// One subtype's frame column.
#[derive(Debug, Clone, PartialEq, Eq, Hash, Deserialize, Serialize)]
pub struct SubtypeFacts {
    pub subtype: Subtype,
    pub frame: FrameFeature,
}

/// One designation label's columns.
#[derive(Debug, Clone, PartialEq, Eq, Hash, Deserialize, Serialize)]
pub struct DesignationFacts {
    pub label: DesignationLabel,
    pub scope: DesignationScope,
    /// Whether an instruction may confer it directly ("becomes the monarch");
    /// a designation that only a rule confers (the commander) is not conferred
    /// by text.
    pub effectful: bool,
    pub zone: Option<Zone>,
    pub r#type: Option<CardType>,
    /// Which half of a Room permanent this designation unlocks, for the two
    /// that do.
    #[serde(default)]
    pub half: Option<RoomHalf>,
}

/// One counter kind's columns.
#[derive(Debug, Clone, PartialEq, Eq, Hash, Deserialize, Serialize)]
pub struct CounterFacts {
    pub label: String,
    /// What the counter is placed on: an object or a player [CR#122.1].
    pub holder: Kind,
}

/// The role of one ordered keyword argument.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, Deserialize, Serialize)]
pub enum KeywordParamShape {
    Cost,
    Quality,
    Subject,
    Number,
    Ability,
    DeckCondition,
}

/// Registry-supplied checking data for one argument position. Quality domains
/// may be restricted to objects; an absent restriction retains the predicate's
/// inferred domain.
#[derive(Debug, Clone, PartialEq, Eq, Hash, Deserialize, Serialize)]
pub struct KeywordParamSpec {
    pub shape: KeywordParamShape,
    #[serde(default)]
    pub quality_domain: Option<Kind>,
}

/// One ordered argument schema a keyword admits.
pub type KeywordSchema = Vec<KeywordParamSpec>;

/// When a keyword's ability applies relative to the stack.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, Deserialize, Serialize)]
pub enum StackRegime {
    AtCasting,
    AtResolution,
}

/// The general category an ability belongs to [CR#113.3]: a statement that is
/// simply true, a trigger condition with an effect, or a cost with an effect
/// [CR#113.3b,113.3c,113.3d]. A keyword ability's definition is written in one
/// of these three [CR#702.1].
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, Deserialize, Serialize)]
pub enum AbilityCategory {
    Static,
    Triggered,
    Activated,
}

/// One keyword ability's columns.
#[derive(Debug, Clone, PartialEq, Eq, Hash, Deserialize, Serialize)]
#[expect(
    clippy::struct_excessive_bools,
    reason = "one field per Lean fact column, and those columns are booleans"
)]
pub struct KeywordFacts {
    pub word: KeywordLabel,
    /// Ordered argument schemas admitted by the keyword declaration and its
    /// variants.
    #[serde(default)]
    pub argument_schemas: Vec<KeywordSchema>,
    #[serde(default)]
    pub counter_eligible: bool,
    #[serde(default)]
    pub regime: Option<StackRegime>,
    /// True where the ability is the spell's own, so no permanent holds it
    /// [CR#113.6].
    #[serde(default)]
    pub functions_on_stack: bool,
    #[serde(default = "yes")]
    pub on_permanent_card: bool,
    #[serde(default)]
    pub on_instant_or_sorcery_card: bool,
    #[serde(default)]
    pub paid_cost: bool,
    /// The categories the keyword's definition is written in [CR#702.1]. An
    /// empty list is a keyword whose definition the workbench has not yet
    /// declared.
    #[serde(default)]
    pub definition: Vec<AbilityCategory>,
    #[serde(default)]
    pub wants_modes: bool,
}

/// Lean's `onPermanentCard := true` default.
fn yes() -> bool {
    true
}
