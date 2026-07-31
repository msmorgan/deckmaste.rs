//! `deckmaste_frames` bridges RON macro definitions to Magic English through
//! "frames": English templates with typed holes.
//!
//! [`view`] is the foundation every other stage in this crate builds on: a
//! generic, serde-driven tree view of anything that derives `Serialize`. It
//! is the same driver pattern as `xtask`'s corpus-shape tool, with one
//! semantic change — scalar *values* are preserved rather than discarded, so
//! a later unifier can recover a literal like `Count = 3` from the tree.

pub mod view;

pub use view::View;
