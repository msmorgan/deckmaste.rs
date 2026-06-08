//! Naming helper shared across `extract` and the keyword parsers/stubs: a
//! display name (keyword ability, type, subtype) → a bare Rust identifier.

use std::sync::LazyLock;

use regex::Regex;

/// Converts a keyword ability name to a Rust identifier, e.g.
/// "Cumulative upkeep" -> "`CumulativeUpkeep`", "Jump-start" -> "`JumpStart`".
pub(crate) fn to_rust_ident(name: &str) -> String {
    static SPLIT: LazyLock<Regex> = LazyLock::new(|| Regex::new(r"[\s|-]+").unwrap());

    SPLIT
        .split(name)
        .flat_map(|word| {
            let mut chars = word.chars();
            chars
                .next()
                .into_iter()
                .flat_map(char::to_uppercase)
                .chain(chars)
        })
        .filter(char::is_ascii_alphanumeric)
        .collect::<String>()
}

#[cfg(test)]
mod tests {
    use super::to_rust_ident;

    #[test]
    fn rust_idents() {
        assert_eq!(to_rust_ident("Flying"), "Flying");
        assert_eq!(to_rust_ident("Cumulative upkeep"), "CumulativeUpkeep");
        assert_eq!(to_rust_ident("Jump-start"), "JumpStart");
        assert_eq!(to_rust_ident("Doctor's companion"), "DoctorsCompanion");
    }
}
