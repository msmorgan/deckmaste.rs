//! English pluralization rules for a template's head noun: templates carry
//! grammatical number so one macro serves both a singular ("each other
//! creature you control") and plural ("Other creatures you control") surface
//! — see [`crate::render::template`] for the render-side consumer.
//!
//! Regular rules only: `+s`; `+es` after `s`/`x`/`z`/`ch`/`sh`; a
//! consonant-preceding `y` becomes `ies`; a trailing `f`/`fe` becomes `ves`.
//! The `es`/`ves` rules are lossy heuristics — they mis-plural nouns such as
//! `roof`->`rooves`, `chief`->`chieves`, and `monarch`->`monarches`; such nouns
//! must carry a `plural:` override.
//! A macro whose noun is irregular (or simply not covered by these rules)
//! declares its own `plural:` field instead (see `macro_ron::MacroDef`),
//! which always wins over this fallback.

/// Pluralize an English noun by regular rules (see module docs). Pure and
/// total: always produces a string, never declines.
#[must_use]
pub fn pluralize(word: &str) -> String {
    let lower = word.to_ascii_lowercase();
    if let Some(stem) = lower.strip_suffix("fe") {
        return format!("{}ves", &word[..stem.len()]);
    }
    if let Some(stem) = lower.strip_suffix('f') {
        return format!("{}ves", &word[..stem.len()]);
    }
    if lower.ends_with(|c| "sxz".contains(c)) || lower.ends_with("ch") || lower.ends_with("sh") {
        return format!("{word}es");
    }
    if let Some(stem) = lower.strip_suffix('y')
        && !stem.ends_with(|c| "aeiou".contains(c))
    {
        return format!("{}ies", &word[..stem.len()]);
    }
    format!("{word}s")
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn regular_plurals() {
        assert_eq!(pluralize("goblin"), "goblins");
        assert_eq!(pluralize("mercenary"), "mercenaries");
        assert_eq!(pluralize("elf"), "elves");
        assert_eq!(pluralize("wolf"), "wolves");
        assert_eq!(pluralize("witch"), "witches");
        assert_eq!(pluralize("creature"), "creatures");
    }

    /// A vowel-preceding `y` keeps the regular `+s` (`day` -> `days`, not
    /// `daies`) — the consonant-`y` rule must not overreach.
    #[test]
    fn vowel_preceding_y_is_regular() {
        assert_eq!(pluralize("day"), "days");
    }

    /// `s`/`x`/`z`-final nouns take `+es`.
    #[test]
    fn sibilant_endings_take_es() {
        assert_eq!(pluralize("box"), "boxes");
        assert_eq!(pluralize("buzz"), "buzzes");
        assert_eq!(pluralize("bus"), "buses");
    }
}
