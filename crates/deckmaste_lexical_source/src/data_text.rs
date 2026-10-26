//! How a data file reaches the program: compiled in for release and wasm
//! builds, read from disk at run time for dev builds.
//!
//! A dev-build edit to a data file then needs no recompilation of this crate,
//! of the crates that depend on it, or of the tools linking them.

/// The text of a data file of the invoking crate, `$path` relative to the
/// crate's manifest directory, as a `LazyLock<String>`.
///
/// - **Release or wasm builds** (`not(debug_assertions)` or `target_arch =
///   "wasm32"`): compiled in with `include_str!`, so there is no filesystem
///   dependency.
/// - **Dev builds:** read from the file on first use, so editing the file needs
///   no rebuild.
///
/// # Panics
/// On first use of a dev build, naming the path, if the file cannot be read: a
/// `LazyLock` cannot return an error.
#[cfg(any(not(debug_assertions), target_arch = "wasm32"))]
#[macro_export]
macro_rules! data_text {
    ($path:literal) => {
        ::std::sync::LazyLock::new(|| {
            ::std::string::String::from(include_str!(concat!(
                env!("CARGO_MANIFEST_DIR"),
                "/",
                $path
            )))
        })
    };
}

/// The text of a data file of the invoking crate, `$path` relative to the
/// crate's manifest directory, as a `LazyLock<String>`.
///
/// - **Release or wasm builds** (`not(debug_assertions)` or `target_arch =
///   "wasm32"`): compiled in with `include_str!`, so there is no filesystem
///   dependency.
/// - **Dev builds:** read from the file on first use, so editing the file needs
///   no rebuild.
///
/// # Panics
/// On first use of a dev build, naming the path, if the file cannot be read: a
/// `LazyLock` cannot return an error.
#[cfg(not(any(not(debug_assertions), target_arch = "wasm32")))]
#[macro_export]
macro_rules! data_text {
    ($path:literal) => {
        ::std::sync::LazyLock::new(|| {
            $crate::read_data_file(concat!(env!("CARGO_MANIFEST_DIR"), "/", $path))
        })
    };
}

/// Reads a data file for [`data_text!`] in a dev build.
///
/// # Panics
/// If the file cannot be read, naming `path`.
#[doc(hidden)]
#[must_use]
pub fn read_data_file(path: &str) -> String {
    std::fs::read_to_string(path)
        .unwrap_or_else(|error| panic!("cannot read data file {path}: {error}"))
}
