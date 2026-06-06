//! The central RON configuration and helpers shared by everything that reads
//! or writes deckmaste's RON files.

/// The options every RON file is read and written with.
///
/// As default extensions these need no `#![enable(...)]` header:
/// `implicit_some` keeps `Option` fields flat, and `unwrap_variant_newtypes`
/// is what lets files spell a struct-carrying variant flat —
/// `Normal(name: ..., ...)` instead of `Normal((name: ...))`.
pub fn options() -> ron::Options {
    ron::Options::default().with_default_extension(
        ron::extensions::Extensions::IMPLICIT_SOME
            | ron::extensions::Extensions::UNWRAP_VARIANT_NEWTYPES,
    )
}
