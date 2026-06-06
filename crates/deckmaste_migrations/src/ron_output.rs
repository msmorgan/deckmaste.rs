//! How migration output files are rendered as RON: optional fields stay
//! unwrapped via implicit `Some`, multi-line strings are written verbatim,
//! and single-element arrays stay on one line.

use serde::Serialize;

/// The shared config from [`deckmaste_core::ron`]: its extensions are
/// defaults, so no `#![enable(...)]` header is emitted.
pub(crate) fn ron_options() -> ron::Options { deckmaste_core::ron::options() }

/// Multi-line text is written verbatim and arrays are chopped, one element
/// per line. Tuple members (e.g. `Hybrid(...)` mana symbols) stay inline,
/// the default.
pub(crate) fn pretty_config() -> ron::ser::PrettyConfig {
    ron::ser::PrettyConfig::default()
        .extensions(ron_options().default_extensions)
        .escape_strings(false)
}

/// Renders a value with [`ron_options`] and [`pretty_config`].
pub(crate) fn to_string_pretty<T: Serialize>(value: &T) -> Result<String, ron::Error> {
    ron_options().to_string_pretty(value, pretty_config())
}

/// Serializes a single-element array on one line (`[Red]`); longer arrays
/// fall through to the chopped pretty-printer. ron's config cannot express
/// this, so the compact form is pre-rendered and embedded as a `RawValue`.
pub(crate) fn one_line_if_single<T: Serialize, S: serde::Serializer>(
    array: &[T],
    serializer: S,
) -> Result<S::Ok, S::Error> {
    use serde::ser::Error as _;

    if array.len() != 1 {
        return array.serialize(serializer);
    }
    // The same dialect as the surrounding document, just inlined via the
    // depth limit; a divergent config here would render e.g. implicit Some
    // differently than the multi-line path.
    let compact = ron_options()
        .to_string_pretty(&array, pretty_config().depth_limit(0))
        .map_err(S::Error::custom)?;
    ron::value::RawValue::from_ron(&compact)
        .map_err(S::Error::custom)?
        .serialize(serializer)
}
