//! English object description → `Filter` RON string (the shared subject parser
//! used by `static_ability`, and the home of future target/selection filter
//! parsing). Strict: an unrecognized head noun or any unconsumed token declines
//! (`None`) — a wrong filter would graduate a wrong card.

use std::collections::HashSet;
use std::sync::LazyLock;

use regex::Regex;

/// Every printed subtype name across the Scryfall subtype catalogs (creature,
/// artifact, enchantment, land, planeswalker, battle, spell). Used to validate
/// a subtype-adjective ("Elf creatures") so a Title-Case non-subtype at a
/// sentence start (an anaphor like "Equipped", a negation like "Nontoken") is
/// never minted as a `Subtype`. Empty if the catalogs can't be read — then the
/// subtype-adjective production simply declines, never mis-parses.
static SUBTYPES: LazyLock<HashSet<String>> = LazyLock::new(|| {
    let mut set = HashSet::new();
    for category in [
        "creature",
        "artifact",
        "enchantment",
        "land",
        "planeswalker",
        "battle",
        "spell",
    ] {
        let Ok(bytes) = crate::data::scryfall::catalog_bytes(&format!("{category}-types")) else {
            continue;
        };
        let Ok(catalog) = crate::data::scryfall::Catalog::parse(&bytes) else {
            continue;
        };
        set.extend(catalog.data.iter().map(ToString::to_string));
    }
    set
});

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
        // Designation adjective before a type noun: "Commander creatures" →
        // Designated("Commander") (checked before subtype — it's not a subtype).
        if let Some((atom, r)) = strip_designation_adjective(rest) {
            prefix_atoms.push(atom);
            rest = r;
            continue;
        }
        // Subtype adjective before a type noun: "Elf creatures" → Subtype("Elf").
        if let Some((atom, r)) = strip_subtype_adjective(rest) {
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

/// A known subtype used as an adjective before a type noun: "Elf creatures" →
/// (`Subtype("Elf")`, "creatures …"). Fires only when `first` is a catalog
/// subtype (so anaphors like "Equipped", negations like "Nontoken", and
/// card-type/supertype words decline rather than mint a wrong atom) AND a type
/// noun follows (so a bare "Goblins" stays a head noun, and a plural like
/// "Goblins" — absent from the singular-keyed catalog — falls through too).
fn strip_subtype_adjective(s: &str) -> Option<(String, &str)> {
    let (first, rest) = s.split_once(' ')?;
    if !SUBTYPES.contains(first) || !is_type_noun(rest.split_whitespace().next()?) {
        return None;
    }
    Some((
        format!("Subtype(\"{}\")", crate::ident::to_rust_ident(first)),
        rest.trim_start(),
    ))
}

/// A known designation (taxonomy §8) → its `Designated` ident, else `None`.
/// "commander" is a designation (an attribute of the card, not a
/// characteristic), NOT a creature subtype ([CR#903.3]). There's
/// no designation catalog yet (`DesignationDecl` is an open `Ident` with no
/// loader), so the recognized set is explicit and grows as designations
/// surface. Matches singular or plural, any case.
fn designation_ident(word: &str) -> Option<&'static str> {
    match word.trim_end_matches('s').to_ascii_lowercase().as_str() {
        "commander" => Some("Commander"),
        _ => None,
    }
}

/// A known designation used as an adjective before a type noun:
/// "Commander creatures" → (`Designated("Commander")`, "creatures …"). Like
/// [`strip_subtype_adjective`], but for designations.
fn strip_designation_adjective(s: &str) -> Option<(String, &str)> {
    let (first, rest) = s.split_once(' ')?;
    let ident = designation_ident(first)?;
    if !is_type_noun(rest.split_whitespace().next()?) {
        return None;
    }
    Some((format!("Designated(\"{ident}\")"), rest.trim_start()))
}

/// A card-type noun (singular or plural) that can anchor a filter head:
/// `creature(s)`, `permanent(s)`, `artifact(s)`, `sorcer(y|ies)`, … Mirrors
/// the type set [`head_noun`] recognizes.
fn is_type_noun(word: &str) -> bool {
    let singular = word.strip_suffix("ies").map_or_else(
        || word.strip_suffix('s').unwrap_or(word).to_string(),
        |stem| format!("{stem}y"),
    );
    matches!(
        singular.to_ascii_lowercase().as_str(),
        "creature"
            | "permanent"
            | "planeswalker"
            | "battle"
            | "artifact"
            | "enchantment"
            | "land"
            | "instant"
            | "sorcery"
    )
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
    // A designation head ("commander") is not a subtype ([CR#903.3]).
    if let Some(ident) = designation_ident(w) {
        return Some(format!("Designated(\"{ident}\")"));
    }
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
        assert_eq!(
            parse_phrase("Elf on the battlefield").as_deref(),
            Some("Subtype(\"Elf\")")
        );
        assert_eq!(
            parse_phrase("creatures on the battlefield").as_deref(),
            Some("Creature")
        );
    }

    #[test]
    fn subtype_adjective_before_type_noun() {
        // "Elf creatures" → a creature with the Elf subtype.
        assert_eq!(
            parse_phrase("Elf creatures").as_deref(),
            Some("AllOf([Creature, Subtype(\"Elf\")])")
        );
        // Elvish Archdruid's anthem subject.
        assert_eq!(
            parse_phrase("Other Elf creatures you control").as_deref(),
            Some("AllOf([Creature, Not(Ref(This)), Subtype(\"Elf\"), ControlledBy(Ref(You))])")
        );
        // A bare subtype head still parses as the head (not an adjective).
        assert_eq!(
            parse_phrase("Goblins").as_deref(),
            Some("Subtype(\"Goblin\")")
        );
        assert_eq!(
            parse_phrase("Goblins you control").as_deref(),
            Some("AllOf([Subtype(\"Goblin\"), ControlledBy(Ref(You))])")
        );
    }

    #[test]
    fn non_subtype_words_before_a_type_noun_decline() {
        // Only catalog subtypes are adjectives. Card types, supertypes, attach
        // anaphors, and negations are NOT subtypes — decline (never mint a wrong
        // Subtype atom). These are the real corpus offenders the catalog gate
        // rules out.
        assert!(parse_phrase("Artifact creatures").is_none()); // card type
        assert!(parse_phrase("Legendary creatures").is_none()); // supertype
        assert!(parse_phrase("Equipped creature").is_none()); // attach anaphor
        assert!(parse_phrase("Nontoken creatures").is_none()); // negation
    }

    #[test]
    fn commander_is_a_designation_not_a_subtype() {
        // Head: "target commander" → Designated, not Subtype.
        assert_eq!(
            parse_phrase("commander").as_deref(),
            Some("Designated(\"Commander\")")
        );
        // Adjective: Bastion Protector / Bloodsworn Steward anthem subject.
        assert_eq!(
            parse_phrase("Commander creatures you control").as_deref(),
            Some("AllOf([Creature, Designated(\"Commander\"), ControlledBy(Ref(You))])")
        );
        // Plural head.
        assert_eq!(
            parse_phrase("commanders you control").as_deref(),
            Some("AllOf([Designated(\"Commander\"), ControlledBy(Ref(You))])")
        );
    }

    #[test]
    fn declines_unparsable() {
        assert!(parse_phrase("creatures wearing hats").is_none());
        assert!(parse_phrase("xyzzy plover blorp").is_none());
    }
}
