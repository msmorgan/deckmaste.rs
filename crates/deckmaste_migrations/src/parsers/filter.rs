//! English object description → `Filter` RON string (the shared subject parser
//! used by `static_ability`, and the home of future target/selection filter
//! parsing). Strict: an unrecognized head noun or any unconsumed token declines
//! (`None`) — a wrong filter would graduate a wrong card.

/// Parse an object-description phrase into a `Filter` RON string, or `None`.
pub(crate) fn parse_phrase(phrase: &str) -> Option<String> {
    let mut atoms: Vec<String> = Vec::new();
    let rest = phrase.trim();

    // Head noun (required). Later tasks peel prefix adjectives / postfix clauses
    // around this; for now the whole phrase must be a bare head noun.
    let head = head_noun(rest)?;
    atoms.push(head);

    Some(combine(atoms))
}

/// Map a singular/plural type word to its builtin filter macro (battlefield-
/// scoped) or `Type(<T>)`; otherwise treat a single token as a subtype.
fn head_noun(word: &str) -> Option<String> {
    let w = word.trim();
    // Singularize: the `-ies → -y` irregular plural first (so "sorceries" →
    // "sorcery"), then the plain trailing `-s`.
    let singular = w.strip_suffix("ies").map_or_else(
        || w.strip_suffix('s').unwrap_or(w).to_string(),
        |stem| format!("{stem}y"),
    );
    let atom = match singular.to_ascii_lowercase().as_str() {
        "creature" => "Creature".to_string(),
        "permanent" => "Permanent".to_string(),
        "planeswalker" => "Planeswalker".to_string(),
        "battle" => "Battle".to_string(),
        "artifact" => "Type(Artifact)".to_string(),
        "enchantment" => "Type(Enchantment)".to_string(),
        "land" => "Type(Land)".to_string(),
        "instant" => "Type(Instant)".to_string(),
        "sorcery" => return Some("Type(Sorcery)".to_string()), // irregular plural
        other if !other.is_empty() && !other.contains(' ') => {
            format!("Subtype(\"{}\")", crate::ident::to_rust_ident(other))
        }
        _ => return None,
    };
    Some(atom)
}

fn combine(atoms: Vec<String>) -> String {
    if atoms.len() == 1 {
        atoms.into_iter().next().unwrap()
    } else {
        format!("AllOf([{}])", atoms.join(", "))
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn head_nouns() {
        assert_eq!(parse_phrase("creatures").as_deref(), Some("Creature"));
        assert_eq!(parse_phrase("permanents").as_deref(), Some("Permanent"));
        assert_eq!(parse_phrase("artifacts").as_deref(), Some("Type(Artifact)"));
        assert_eq!(
            parse_phrase("Goblins").as_deref(),
            Some("Subtype(\"Goblin\")")
        );
        assert_eq!(parse_phrase("sorceries").as_deref(), Some("Type(Sorcery)"));
    }
}
