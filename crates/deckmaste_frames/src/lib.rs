//! `deckmaste_frames` bridges RON macro definitions to Magic English through
//! "frames": English templates with typed holes.
//!
//! [`view`] is the foundation every other stage in this crate builds on: a
//! generic, serde-driven tree view of anything that derives `Serialize`. It
//! is the same driver pattern as `xtask`'s corpus-shape tool, with one
//! semantic change — scalar *values* are preserved rather than discarded, so
//! a later unifier can recover a literal like `Count = 3` from the tree.
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
//! with [`View::Hole`] nodes where the frame's `<Param(i)>` and `~` sigils
//! were, plus the side tables saying what each hole is and which nodes take
//! their inflection from it. It stands on three supporting modules:
//!
//! - [`witness`] — the reserved vocabulary a sigil is replaced by so the frame
//!   text can be parsed as ordinary English at all, and the build-time check
//!   that an authored frame does not spell one itself.
//! - [`view`] — the tree representation and the paths into it.
//! - [`guard`] — canonical form for a guard's pre-bound constant.
//!   [`guard::normalized`] is deliberately the *only* definition of "canonical"
//!   in the round: compile time, match time, and render time all call it, so a
//!   guard cannot be satisfied by one and missed by another.

pub mod compile;
pub mod guard;
pub mod view;
pub mod witness;

pub use compile::AgreeKind;
pub use compile::AgreementDep;
pub use compile::CompiledFrame;
pub use compile::CompiledGuard;
pub use compile::Hole;
pub use compile::HoleClass;
pub use compile::Normalization;
pub use compile::compile;
pub use compile::compile_with_macros;
pub use view::PathStep;
pub use view::TreePath;
pub use view::View;
