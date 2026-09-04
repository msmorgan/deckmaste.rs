//! `deckmaste_spelling` bridges RON macro definitions to Magic English through
//! "frames": English templates with typed holes.
//!
//! [`projection`] is the English-facing foundation: a tree of selected
//! constructions, stable forms, and named typed roles generated from the
//! declaration compiler. [`view`] remains a generic serde walker only for
//! canonical semantic RON values such as frame guards; it is not an English
//! matching contract.
//!
//! The frame authoring schema itself (`FrameSpec`, `FramePosition`,
//! `ConstructorFrames`, `load_constructor_frames`) lives in
//! `macro_ron::frames` — one path, `macro_ron::frames::FrameSpec` — not
//! here and not re-exported here: `MacroDef.frames` (`macro_ron::set`) needs
//! the type, so the schema has to live where `macro_ron` can reach it
//! without depending on this crate, and this crate depends on `macro_ron`
//! (not the reverse) to eventually read and expand macro definitions.
//!
//! # The stages
//!
//! [`compile()`] is the entry point: an authored
//! [`FrameSpec`](macro_ron::frames::FrameSpec) in, a
//! [`CompiledFrame`] out — a parsed English tree
//! with [`ProjectionTree::Hole`] nodes where the frame's `<Param(i)>` and `~`
//! sigils were, plus the side tables saying what each hole is and which nodes
//! take their inflection from it. It stands on three supporting modules:
//!
//! - [`witness`] — the reserved vocabulary a sigil is replaced by so the frame
//!   text can be parsed as ordinary English at all, and the build-time check
//!   that an authored frame does not spell one itself.
//! - [`projection`] — the construction tree and stable role paths.
//! - [`guard`] — canonical form for a guard's pre-bound constant.
//!   [`guard::normalized`] is deliberately the *only* definition of "canonical"
//!   in the round: compile time, match time, and render time all call it, so a
//!   guard cannot be satisfied by one and missed by another.
//!
//! [`lexicon`] and [`unify`] are the matching half: every compiled frame in
//! one place, and the walk that finds which one a piece of real English is,
//! reading its holes' fillers back out as the arguments of the invocation
//! that would render it. Matching is total — anything uncovered comes back
//! as [`Recovered::Residual`] rather than as an error.

pub mod compile;
pub mod guard;
pub mod lexicon;
pub mod projection;
pub mod render;
pub mod unify;
pub mod view;
pub mod witness;

pub use compile::AgreeKind;
pub use compile::CompiledFrame;
pub use compile::CompiledGuard;
pub use compile::FeatureDep;
pub use compile::Hole;
pub use compile::HoleClass;
pub use compile::Normalization;
pub use compile::compile;
pub use lexicon::Entry;
pub use lexicon::Lexicon;
pub use projection::ProjectionPath;
pub use projection::ProjectionStep;
pub use projection::ProjectionTree;
pub use render::ReassembledDifferently;
pub use render::render_invocation;
pub use render::render_invocation_with;
pub use unify::Recovered;
pub use unify::guard_holds;
pub use unify::is_self_reference;
pub use unify::unify;
pub use view::PathStep;
pub use view::TreePath;
pub use view::View;
