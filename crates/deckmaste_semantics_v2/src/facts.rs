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

/// The Entity domain a referent is drawn from: a player, an object, or either
/// [CR#102.1,109.1].
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, Deserialize, Serialize)]
pub enum EntityDomain {
    Player,
    Object,
    Either,
}

/// An object class [CR#109.1]: the overlapping ways an object is classified.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, Deserialize, Serialize)]
pub enum ObjectClass {
    Card,
    Token,
    Spell,
    Permanent,
    Emblem,
    Ability,
}

/// What a referent must be: an Entity domain, optionally narrowed by object
/// classes and card types. Only an object has a class.
#[derive(Debug, Clone, PartialEq, Eq, Hash, Deserialize, Serialize)]
pub struct ReferentSort {
    pub domain: EntityDomain,
    #[serde(default)]
    pub classes: Vec<ObjectClass>,
    #[serde(default)]
    pub types: Vec<CardType>,
}

/// One participant slot of a deed: what fills it, whether it may be bare, and
/// the zone it is read in.
#[derive(Debug, Clone, PartialEq, Eq, Hash, Deserialize, Serialize)]
pub struct DeedRole {
    /// `None` = no noun fills this role.
    pub sort: Option<ReferentSort>,
    pub bare: bool,
    pub zone: Option<Zone>,
}

/// What a counterfactual premise ranges over.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, Deserialize, Serialize)]
pub enum PremiseSort {
    Object,
    Mana,
    Value,
}

/// The keyword actions [CR#701.1] a core constructor must name, declared so a
/// guard reads a feature rather than a verb's spelling.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, Deserialize, Serialize)]
pub enum DeedFeature {
    LibrarySearch,
    Sacrificing,
    Tapping,
}

/// What the checker knows about one deed. The record carries no label: the
/// deed itself is the key.
#[derive(Debug, Clone, PartialEq, Eq, Hash, Deserialize, Serialize)]
#[expect(
    clippy::struct_excessive_bools,
    reason = "one field per Lean fact column, and those columns are booleans"
)]
pub struct ActFacts {
    #[serde(default)]
    pub participle: Option<String>,
    #[serde(default)]
    pub dest: Option<Zone>,
    #[serde(default)]
    pub stepwise: bool,
    #[serde(default)]
    pub loci: Vec<Zone>,
    #[serde(default)]
    pub intransitive: bool,
    #[serde(default = "no_role")]
    pub agent_role: DeedRole,
    #[serde(default = "no_role")]
    pub patient_role: DeedRole,
    #[serde(default)]
    pub feature: Option<DeedFeature>,
    #[serde(default)]
    pub counterfactual: Option<PremiseSort>,
    #[serde(default)]
    pub rides: bool,
    #[serde(default)]
    pub plays: bool,
    #[serde(default)]
    pub bounded: bool,
    /// The deed opens an opponent's library ("fateseal" [CR#701.29a]); the
    /// same look over one's own library is a different deed.
    #[serde(default)]
    pub opponents_library: bool,
}

/// Lean's `noRole`: the role no noun fills.
fn no_role() -> DeedRole {
    DeedRole {
        sort: None,
        bare: false,
        zone: None,
    }
}

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
