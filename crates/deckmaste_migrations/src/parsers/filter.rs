//! English object description → `Filter` RON string (the shared subject parser
//! used by `static_ability`, and the home of future target/selection filter
//! parsing). Strict: an unrecognized head noun or any unconsumed token declines
//! (`None`) — a wrong filter would graduate a wrong card.

use regex::Regex;

/// Parse an object-description phrase into a `Filter` RON string, or `None`.
pub(crate) fn parse_phrase(phrase: &str) -> Option<String> {
    let mut rest = phrase.trim();
    let mut prefix_atoms: Vec<String> = Vec::new();

    loop {
        // Determiners — consumed, no atom.
        if let Some(r) = strip_word(rest, &["a", "an", "each", "all", "any"]) {
            rest = r;
            continue;
        }
        // Self-exclusion.
        if let Some(r) = strip_word(rest, &["other", "another"]) {
            prefix_atoms.push("Not(Ref(This))".into());
            rest = r;
            continue;
        }
        // Status adjectives.
        if let Some((atom, r)) = strip_status(rest) {
            prefix_atoms.push(atom);
            rest = r;
            continue;
        }
        // Color / type negation: `non<color>` / `non<type>`.
        if let Some((atom, r)) = strip_negation(rest) {
            prefix_atoms.push(atom);
            rest = r;
            continue;
        }
        // Color adjective.
        if let Some((atom, r)) = strip_color(rest) {
            prefix_atoms.push(atom);
            rest = r;
            continue;
        }
        break;
    }

    // Postfix relative clauses (peel off the END).
    // Peels right-to-left off the end, so with multiple postfix clauses the atoms
    // land in reverse source order.
    let mut postfix_atoms: Vec<String> = Vec::new();
    loop {
        // "on the battlefield" is the default scope: consume, emit no atom.
        if let Some(r) = rest.trim_end().strip_suffix(" on the battlefield") {
            rest = r;
            continue;
        }
        if let Some((atom, r)) = strip_postfix(rest) {
            postfix_atoms.push(atom);
            rest = r;
            continue;
        }
        break;
    }

    // What's left must be exactly the head noun.
    let head = head_noun(rest)?;
    let mut atoms = vec![head];
    atoms.extend(prefix_atoms);
    atoms.extend(postfix_atoms);
    Some(combine(atoms))
}

/// Peel one trailing relative clause off `s`, returning (atom, head-remainder).
fn strip_postfix(s: &str) -> Option<(String, &str)> {
    let s = s.trim_end();
    for (suffix, atom) in [
        (" you control", "ControlledBy(Ref(You))"),
        (
            " an opponent controls",
            "ControlledBy(OpponentOf(Ref(You)))",
        ),
        (
            " your opponents control",
            "ControlledBy(OpponentOf(Ref(You)))",
        ),
        (" you own", "Owner(Ref(You))"),
    ] {
        if let Some(head) = s.strip_suffix(suffix) {
            return Some((atom.to_string(), head));
        }
    }
    // Stat clauses via regex (power/toughness, greater/less).
    let re = Regex::new(r"(?i) with (power|toughness) (\d+) or (greater|less)$").unwrap();
    if let Some(caps) = re.captures(s) {
        let stat = if caps[1].eq_ignore_ascii_case("power") { "Power" } else { "Toughness" };
        let n = &caps[2];
        let cmp = if caps[3].eq_ignore_ascii_case("greater") { "AtLeast" } else { "AtMost" };
        let head = &s[..caps.get(0).unwrap().start()];
        return Some((format!("Stat({stat}, {cmp}, Literal({n}))"), head));
    }
    if let Some(head) = s.strip_suffix(" with a +1/+1 counter on it") {
        return Some(("HasCounter(\"+1/+1\")".to_string(), head));
    }
    None
}

/// Strip a leading whole word (case-insensitive) from `s`, returning the rest.
fn strip_word<'a>(s: &'a str, words: &[&str]) -> Option<&'a str> {
    let (first, rest) = s.split_once(' ')?;
    words
        .iter()
        .any(|w| first.eq_ignore_ascii_case(w))
        .then(|| rest.trim_start())
}

fn strip_status(s: &str) -> Option<(String, &str)> {
    let (first, rest) = s.split_once(' ')?;
    let atom = match first.to_ascii_lowercase().as_str() {
        "tapped" => "Status(Tapped)",
        "untapped" => "Status(Untapped)",
        "attacking" => "Attacking",
        "blocking" => "Blocking",
        _ => return None,
    };
    Some((atom.to_string(), rest.trim_start()))
}

fn color_code(word: &str) -> Option<&'static str> {
    Some(match word.to_ascii_lowercase().as_str() {
        "white" => "White",
        "blue" => "Blue",
        "black" => "Black",
        "red" => "Red",
        "green" => "Green",
        _ => return None,
    })
}

fn type_code(word: &str) -> Option<&'static str> {
    Some(match word.to_ascii_lowercase().as_str() {
        "creature" => "Creature",
        "artifact" => "Artifact",
        "enchantment" => "Enchantment",
        "land" => "Land",
        "planeswalker" => "Planeswalker",
        _ => return None,
    })
}

fn strip_color(s: &str) -> Option<(String, &str)> {
    let (first, rest) = s.split_once(' ')?;
    let atom = match first.to_ascii_lowercase().as_str() {
        "colorless" => "Colorless".to_string(),
        "multicolored" => "Multicolored".to_string(),
        other => format!("ColorIs({})", color_code(other)?),
    };
    Some((atom, rest.trim_start()))
}

/// `nonblack` → `Not(ColorIs(Black))`, `noncreature` → `Not(Type(Creature))`.
fn strip_negation(s: &str) -> Option<(String, &str)> {
    let (first, rest) = s.split_once(' ')?;
    let lower = first.to_ascii_lowercase();
    let stem = lower.strip_prefix("non")?;
    let atom = if let Some(c) = color_code(stem) {
        format!("Not(ColorIs({c}))")
    } else if let Some(t) = type_code(stem) {
        format!("Not(Type({t}))")
    } else {
        return None;
    };
    Some((atom, rest.trim_start()))
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
        "sorcery" => "Type(Sorcery)".to_string(),
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

    #[test]
    fn prefix_adjectives() {
        assert_eq!(
            parse_phrase("other Goblins").as_deref(),
            Some("AllOf([Subtype(\"Goblin\"), Not(Ref(This))])")
        );
        assert_eq!(
            parse_phrase("nonblack creatures").as_deref(),
            Some("AllOf([Creature, Not(ColorIs(Black))])")
        );
        assert_eq!(
            parse_phrase("black creatures").as_deref(),
            Some("AllOf([Creature, ColorIs(Black)])")
        );
        assert_eq!(
            parse_phrase("tapped creatures").as_deref(),
            Some("AllOf([Creature, Status(Tapped)])")
        );
        assert_eq!(parse_phrase("a creature").as_deref(), Some("Creature"));
        assert_eq!(
            parse_phrase("colorless creatures").as_deref(),
            Some("AllOf([Creature, Colorless])")
        );
        assert_eq!(
            parse_phrase("other nonblack creatures").as_deref(),
            Some("AllOf([Creature, Not(Ref(This)), Not(ColorIs(Black))])")
        );
    }

    #[test]
    fn postfix_clauses() {
        assert_eq!(
            parse_phrase("creatures you control").as_deref(),
            Some("AllOf([Creature, ControlledBy(Ref(You))])")
        );
        assert_eq!(
            parse_phrase("artifacts an opponent controls").as_deref(),
            Some("AllOf([Type(Artifact), ControlledBy(OpponentOf(Ref(You)))])")
        );
        assert_eq!(
            parse_phrase("creatures your opponents control").as_deref(),
            Some("AllOf([Creature, ControlledBy(OpponentOf(Ref(You)))])")
        );
        assert_eq!(
            parse_phrase("creatures with power 3 or greater").as_deref(),
            Some("AllOf([Creature, Stat(Power, AtLeast, Literal(3))])")
        );
        assert_eq!(
            parse_phrase("other creatures you control").as_deref(),
            Some("AllOf([Creature, Not(Ref(This)), ControlledBy(Ref(You))])")
        );
        assert_eq!(
            parse_phrase("creatures you own").as_deref(),
            Some("AllOf([Creature, Owner(Ref(You))])")
        );
        assert_eq!(
            parse_phrase("creatures with toughness 2 or less").as_deref(),
            Some("AllOf([Creature, Stat(Toughness, AtMost, Literal(2))])")
        );
        assert_eq!(
            parse_phrase("creatures with a +1/+1 counter on it").as_deref(),
            Some("AllOf([Creature, HasCounter(\"+1/+1\")])")
        );
        // word-number is out of the regex's \d+ scope → declines
        assert!(parse_phrase("creatures with power three or greater").is_none());
    }

    #[test]
    fn on_the_battlefield_is_consumed() {
        // "on the battlefield" is the default scope — consumed, no atom.
        assert_eq!(parse_phrase("Elf on the battlefield").as_deref(), Some("Subtype(\"Elf\")"));
        assert_eq!(
            parse_phrase("creatures on the battlefield").as_deref(),
            Some("Creature")
        );
    }

    #[test]
    fn declines_unparsable() {
        assert!(parse_phrase("creatures wearing hats").is_none());
        assert!(parse_phrase("xyzzy plover blorp").is_none());
    }
}
