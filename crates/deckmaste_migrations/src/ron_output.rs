//! How migration output files are rendered as RON: optional fields stay
//! unwrapped via implicit `Some`, multi-line strings are written verbatim,
//! and every array is chopped one element per line.

use serde::Serialize;

/// The shared config from [`deckmaste_core::ron`]: its extensions are
/// defaults, so no `#![enable(...)]` header is emitted.
pub(crate) fn ron_options() -> ron::Options {
    deckmaste_core::ron::raw_options()
}

/// Two-space indentation; multi-line text is written verbatim and arrays are
/// chopped, one element per line (no single-element-on-one-line shortcut).
/// Tuple members (e.g. `Hybrid(...)` mana symbols) stay inline, the default.
pub(crate) fn pretty_config() -> ron::ser::PrettyConfig {
    ron::ser::PrettyConfig::default()
        .extensions(ron_options().default_extensions)
        .escape_strings(false)
        .indentor("  ")
}

/// Renders a value with [`ron_options`] and [`pretty_config`].
pub(crate) fn to_string_pretty<T: Serialize>(value: &T) -> Result<String, ron::Error> {
    ron_options().to_string_pretty(value, pretty_config())
}
