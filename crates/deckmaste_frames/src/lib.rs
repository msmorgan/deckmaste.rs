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

pub mod view;

pub use view::View;
