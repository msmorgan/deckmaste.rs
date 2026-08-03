//! The legacy renderer and the fidelity harness that rides it.
//!
//! Dying code, kept working until the spelling relation replaces it. It reads
//! AUTHORED terms (`docs/decisions/authoring-spelling-lowering.md` §1): those
//! keep their `Expanded` invocation provenance and so keep the rules-text
//! templates, while lowered core carries none. Depending on
//! `deckmaste_authoring` rather than `deckmaste_core` is what takes this code
//! off `core-demacro`'s path and makes its eventual death a crate removal.

pub mod fidelity;
pub mod render;
pub mod template;
