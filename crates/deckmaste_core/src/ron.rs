//! Plain serde RON configuration for engine snapshots and fixtures.

/// The raw `ron::Options` everything is read and written with.
///
#[must_use]
pub fn raw_options() -> ron::Options {
    ron::Options::default().with_default_extension(ron::extensions::Extensions::IMPLICIT_SOME)
}

/// The options used for plain-serde core snapshots and fixtures.
#[must_use]
pub fn options() -> ron::Options {
    raw_options()
}
