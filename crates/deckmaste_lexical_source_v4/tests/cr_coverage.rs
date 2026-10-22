//! Every catalog covers the inventory the Comprehensive Rules enumerate, less
//! the named items outside Vintage-playable Magic, and declares nothing the
//! CR does not name.

use std::collections::BTreeMap;
use std::collections::BTreeSet;
use std::path::PathBuf;

use deckmaste_lexical_source_v4::Sources;
use deckmaste_lexical_source_v4::schema::AbilityWordListing;
use deckmaste_lexical_source_v4::schema::SubtypeFamily;

/// Why a CR item has no catalog entry.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
enum Excluded {
    /// The Conspiracy, Plane, Phenomenon, Scheme and Vanguard card types and
    /// what only they use are permanently out of scope.
    OutOfScope,
    /// Used only on Un-set cards, which are deferred.
    UnSet,
    /// Printed only on digital-only or tournament-illegal cards.
    NoVintageCard,
    /// Registered as a keyword ability instead.
    Keyword,
    /// Registered only with the number it is printed with.
    WithItsNumber,
}

use Excluded::Keyword;
use Excluded::NoVintageCard;
use Excluded::OutOfScope;
use Excluded::UnSet;
use Excluded::WithItsNumber;

const CARD_TYPE_EXCLUSIONS: [(&str, Excluded); 5] = [
    ("conspiracy", OutOfScope),
    ("phenomenon", OutOfScope),
    ("plane", OutOfScope),
    ("scheme", OutOfScope),
    ("vanguard", OutOfScope),
];

/// Only scheme cards are ongoing [CR#205.4h].
const SUPERTYPE_EXCLUSIONS: [(&str, Excluded); 1] = [("ongoing", OutOfScope)];

/// Set in Motion and Abandon are Archenemy actions; Attractions are Un-set
/// cards.
const KEYWORD_ACTION_EXCLUSIONS: [(&str, Excluded); 4] = [
    ("[CR#701.32]", OutOfScope),
    ("[CR#701.33]", OutOfScope),
    ("[CR#701.51]", UnSet),
    ("[CR#701.52]", UnSet),
];

/// Hidden agenda is a Conspiracy ability; space sculptor and visit are
/// Un-set abilities.
const KEYWORD_ABILITY_EXCLUSIONS: [(&str, Excluded); 3] = [
    ("[CR#702.106]", OutOfScope),
    ("[CR#702.158]", UnSet),
    ("[CR#702.159]", UnSet),
];

/// Keyword abilities the CR defines outside the keyword-ability rules [CR#702],
/// by the rule that defines each and its name: *To solve* is one of the Case's
/// "two special keyword abilities" [CR#719.3,719.3a].
const KEYWORD_ABILITIES_OUTSIDE_702: [(&str, &str); 1] = [("[CR#719.3a]", "To Solve")];

/// Scryfall's inventory lists words that are not ability words of
/// Vintage-playable cards.
const SCRYFALL_ABILITY_WORD_EXCLUSIONS: [(&str, Excluded); 8] = [
    ("covercast", NoVintageCard),
    ("hero's reward", NoVintageCard),
    ("kinfall", UnSet),
    ("landship", UnSet),
    ("legacy", UnSet),
    ("underdog", UnSet),
    // A keyword ability [CR#702.179], not an ability word.
    ("start your engines!", Keyword),
    // Printed only with its number: descend 4, descend 8.
    ("descend", WithItsNumber),
];

fn repo() -> PathBuf {
    PathBuf::from(env!("CARGO_MANIFEST_DIR")).join("../..")
}

/// Rule text by rule number, with curly apostrophes made straight as Oracle
/// text writes them.
fn rules() -> BTreeMap<String, String> {
    let text = std::fs::read_to_string(repo().join("data/rules/cr.txt")).unwrap();
    let mut rules = BTreeMap::new();
    for line in text.lines() {
        let Some((number, rest)) = line.split_once(' ') else {
            continue;
        };
        let number = number.trim_end_matches('.');
        let mut parts = number.split('.');
        let (Some(section), Some(sub), None) = (parts.next(), parts.next(), parts.next()) else {
            continue;
        };
        if section.len() == 3
            && section.chars().all(|c| c.is_ascii_digit())
            && sub.starts_with(|c: char| c.is_ascii_digit())
        {
            rules.insert(number.to_owned(), rest.replace('’', "'"));
        }
    }
    rules
}

/// The comma-separated list after `marker` in `rule`, up to its closing
/// period, split at commas and `and`, with `(see rule …)` and surrounding
/// space removed.
fn list(rules: &BTreeMap<String, String>, rule: &str, marker: &str) -> Vec<String> {
    let rule = number(rule);
    let text = &rules[rule];
    let start = text
        .find(marker)
        .unwrap_or_else(|| panic!("{rule}: no {marker:?}"))
        + marker.len();
    let mut depth = 0;
    let mut end = text.len();
    for (index, ch) in text[start..].char_indices() {
        match ch {
            '(' => depth += 1,
            ')' => depth -= 1,
            '.' if depth == 0 => {
                end = start + index;
                break;
            }
            _ => {}
        }
    }
    text[start..end]
        .split(',')
        .flat_map(|item| item.split(" and "))
        .map(|item| {
            let item = item.trim();
            let item = item.strip_prefix("and ").unwrap_or(item);
            let item = item.split(" (see").next().unwrap();
            item.trim().to_owned()
        })
        .filter(|item| !item.is_empty())
        .collect()
}

/// The rule number of a citation, without its brackets.
fn number(citation: &str) -> &str {
    citation
        .strip_prefix("[CR#")
        .and_then(|rest| rest.strip_suffix(']'))
        .unwrap_or_else(|| panic!("{citation} is not a citation"))
}

fn numbered<'a>(exclusions: &[(&'a str, Excluded)]) -> Vec<(&'a str, Excluded)> {
    exclusions
        .iter()
        .map(|(citation, why)| (number(citation), *why))
        .collect()
}

/// The `section.N` headings, by rule number, with their names.
fn headings(rules: &BTreeMap<String, String>, section: &str) -> BTreeMap<String, String> {
    rules
        .iter()
        .filter(|(number, _)| {
            number
                .strip_prefix(section)
                .and_then(|rest| rest.strip_prefix('.'))
                .is_some_and(|n| n.chars().all(|c| c.is_ascii_digit()) && n != "1")
        })
        .map(|(number, name)| (number.clone(), name.clone()))
        .collect()
}

/// Asserts `declared` is exactly `enumerated` less `excluded`, comparing
/// without case.
fn assert_covers(
    what: &str,
    enumerated: &[String],
    excluded: &[(&str, Excluded)],
    declared: &[String],
) {
    let lower = |items: &mut dyn Iterator<Item = &str>| -> BTreeSet<String> {
        items.map(str::to_lowercase).collect()
    };
    let enumerated = lower(&mut enumerated.iter().map(String::as_str));
    let excluded = lower(&mut excluded.iter().map(|(item, _)| *item));
    let declared = lower(&mut declared.iter().map(String::as_str));
    let stale: Vec<_> = excluded.difference(&enumerated).collect();
    assert!(
        stale.is_empty(),
        "{what}: exclusions the CR does not name: {stale:?}"
    );
    let both: Vec<_> = excluded.intersection(&declared).collect();
    assert!(both.is_empty(), "{what}: excluded but declared: {both:?}");
    let wanted: BTreeSet<_> = enumerated.difference(&excluded).cloned().collect();
    let missing: Vec<_> = wanted.difference(&declared).collect();
    assert!(missing.is_empty(), "{what}: not declared: {missing:?}");
    let extra: Vec<_> = declared.difference(&wanted).collect();
    assert!(
        extra.is_empty(),
        "{what}: declared but not in the CR: {extra:?}"
    );
}

#[test]
fn card_types_and_supertypes() {
    let rules = rules();
    let sources = Sources::load().unwrap();
    let card_types: Vec<String> = sources
        .types
        .card_types
        .iter()
        .map(|decl| decl.entry().lemma)
        .collect();
    assert_covers(
        "card types [CR#205.2a]",
        &list(&rules, "[CR#205.2a]", "The card types are "),
        &CARD_TYPE_EXCLUSIONS,
        &card_types,
    );
    assert_covers(
        "supertypes [CR#205.4a]",
        &list(&rules, "[CR#205.4a]", "The supertypes are "),
        &SUPERTYPE_EXCLUSIONS,
        &sources.types.supertypes,
    );
}

#[test]
fn subtypes_by_family() {
    let rules = rules();
    let sources = Sources::load().unwrap();
    let mut creature = vec!["Time Lord".to_owned()];
    creature.extend(list(&rules, "[CR#205.3m]", "one word long: "));
    let families = [
        (
            SubtypeFamily::Artifact,
            list(&rules, "[CR#205.3g]", "The artifact types are "),
        ),
        (
            SubtypeFamily::Enchantment,
            list(&rules, "[CR#205.3h]", "The enchantment types are "),
        ),
        (
            SubtypeFamily::Land,
            list(&rules, "[CR#205.3i]", "The land types are "),
        ),
        (
            SubtypeFamily::Planeswalker,
            list(&rules, "[CR#205.3j]", "The planeswalker types are "),
        ),
        (
            SubtypeFamily::Spell,
            list(&rules, "[CR#205.3k]", "The spell types are "),
        ),
        (SubtypeFamily::Creature, creature),
        (
            SubtypeFamily::Dungeon,
            list(&rules, "[CR#205.3p]", "That dungeon type is "),
        ),
        (
            SubtypeFamily::Battle,
            list(&rules, "[CR#205.3q]", "That battle type is "),
        ),
    ];
    // Planar types [CR#205.3n] belong to the out-of-scope Plane card type.
    assert_eq!(
        sources.subtypes.keys().copied().collect::<BTreeSet<_>>(),
        families.iter().map(|(family, _)| *family).collect()
    );
    for (family, enumerated) in &families {
        let declared: Vec<String> = sources.subtypes[family]
            .iter()
            .map(|decl| decl.entry().lemma)
            .collect();
        assert_covers(&format!("{family:?} types"), enumerated, &[], &declared);
        // Subtypes are spelled exactly as the CR spells them.
        let exact: BTreeSet<_> = enumerated.iter().collect();
        for lemma in &declared {
            assert!(
                exact.contains(lemma),
                "{family:?}: `{lemma}` is not spelled as in the CR"
            );
        }
    }
}

#[test]
fn keyword_actions() {
    let rules = rules();
    let sources = Sources::load().unwrap();
    let headings = headings(&rules, "701");
    let declared: BTreeMap<_, _> = sources
        .keyword_actions
        .iter()
        .map(|action| (action.rule.number().to_owned(), action.name.clone()))
        .collect();
    assert_eq!(
        declared.len(),
        sources.keyword_actions.len(),
        "a rule declared twice"
    );
    for (rule, name) in &declared {
        assert_eq!(
            headings.get(rule),
            Some(name),
            "{rule} is not the CR heading {name:?}"
        );
    }
    let enumerated: Vec<String> = headings.keys().cloned().collect();
    let declared: Vec<String> = declared.keys().cloned().collect();
    let excluded = numbered(&KEYWORD_ACTION_EXCLUSIONS);
    assert_covers(
        "keyword actions [CR#701.1]",
        &enumerated,
        &excluded,
        &declared,
    );
}

#[test]
fn keyword_abilities() {
    let rules = rules();
    let sources = Sources::load().unwrap();
    let mut headings = headings(&rules, "702");
    for (citation, name) in KEYWORD_ABILITIES_OUTSIDE_702 {
        let rule = number(citation);
        let parent = rule.trim_end_matches(|c: char| c.is_ascii_lowercase());
        assert!(
            rules[parent].contains("keyword abilities"),
            "{parent} does not define keyword abilities"
        );
        assert!(
            rules[rule]
                .to_lowercase()
                .starts_with(&format!("\u{201c}{}", name.to_lowercase())),
            "{rule} does not define {name:?}"
        );
        headings.insert(rule.to_owned(), name.to_owned());
    }
    let declared: BTreeMap<_, _> = sources
        .keyword_abilities
        .iter()
        .map(|ability| (ability.rule.number().to_owned(), ability.name.clone()))
        .collect();
    assert_eq!(
        declared.len(),
        sources.keyword_abilities.len(),
        "a rule declared twice"
    );
    for (rule, name) in &declared {
        assert_eq!(
            headings.get(rule),
            Some(name),
            "{rule} is not the CR heading {name:?}"
        );
    }
    let enumerated: Vec<String> = headings.keys().cloned().collect();
    let declared: Vec<String> = declared.keys().cloned().collect();
    assert_covers(
        "keyword abilities [CR#702.1]",
        &enumerated,
        &numbered(&KEYWORD_ABILITY_EXCLUSIONS),
        &declared,
    );
}

#[test]
fn ability_words() {
    let rules = rules();
    let sources = Sources::load().unwrap();
    let listed = |listing| -> Vec<String> {
        sources
            .ability_words
            .iter()
            .filter(|word| word.listed == listing)
            .map(|word| word.lemma.clone())
            .collect()
    };
    assert_covers(
        "ability words [CR#207.2c]",
        &list(&rules, "[CR#207.2c]", "The ability words are "),
        &[],
        &listed(AbilityWordListing::Rules),
    );
    // Scryfall's inventory adds the printed ability words the CR omits.
    let scryfall: serde_json::Value = serde_json::from_str(
        &std::fs::read_to_string(repo().join("data/catalogs/ability-words.json")).unwrap(),
    )
    .unwrap();
    let scryfall: Vec<String> = scryfall["data"]
        .as_array()
        .unwrap()
        .iter()
        .map(|word| word.as_str().unwrap().replace('’', "'"))
        .collect();
    let all: Vec<String> = sources
        .ability_words
        .iter()
        .map(|word| word.lemma.clone())
        .collect();
    let declared: BTreeSet<String> = all.iter().map(|word| word.to_lowercase()).collect();
    let excluded: BTreeSet<String> = SCRYFALL_ABILITY_WORD_EXCLUSIONS
        .iter()
        .map(|(word, _)| word.to_lowercase())
        .collect();
    for word in &excluded {
        assert!(
            scryfall.iter().any(|s| s.eq_ignore_ascii_case(word)),
            "excluded `{word}` is not in Scryfall's inventory"
        );
        assert!(!declared.contains(word), "excluded `{word}` is declared");
    }
    let missing: Vec<_> = scryfall
        .iter()
        .map(|word| word.to_lowercase())
        .filter(|word| !declared.contains(word) && !excluded.contains(word))
        .collect();
    assert!(
        missing.is_empty(),
        "Scryfall ability words not declared: {missing:?}"
    );
    let cr: BTreeSet<String> = list(&rules, "[CR#207.2c]", "The ability words are ")
        .iter()
        .map(|word| word.to_lowercase())
        .collect();
    for word in listed(AbilityWordListing::Printed) {
        assert!(
            !cr.contains(&word.to_lowercase()),
            "`{word}` is in the CR list"
        );
        assert!(
            scryfall.iter().any(|s| s.eq_ignore_ascii_case(&word)),
            "printed ability word `{word}` is not in Scryfall's inventory"
        );
    }
}

#[test]
fn named_counter_kinds() {
    let rules = rules();
    let sources = Sources::load().unwrap();
    let declared: BTreeMap<String, Option<String>> = sources
        .counters
        .iter()
        .map(|counter| {
            let rule = counter.rule.as_ref().map(|rule| rule.number().to_owned());
            (counter.kind.to_lowercase(), rule)
        })
        .collect();
    assert_eq!(
        declared.len(),
        sources.counters.len(),
        "a counter kind declared twice"
    );
    // Keyword counters, less the closing "as well as any variants" clause.
    let mut named: Vec<(String, &str)> = list(
        &rules,
        "[CR#122.1b]",
        "The keywords that a keyword counter can be are ",
    )
    .into_iter()
    .filter(|keyword| !keyword.starts_with("as well as"))
    .map(|keyword| (keyword, "[CR#122.1b]"))
    .collect();
    for rule in [
        "[CR#122.1c]",
        "[CR#122.1d]",
        "[CR#122.1e]",
        "[CR#122.1f]",
        "[CR#122.1g]",
        "[CR#122.1h]",
        "[CR#122.1i]",
        "[CR#122.1j]",
    ] {
        let text = &rules[number(rule)];
        let end = text
            .find(" counter")
            .unwrap_or_else(|| panic!("{rule} names no counter"));
        let kind = text[..end].rsplit(' ').next().unwrap();
        named.push((kind.to_lowercase(), rule));
    }
    named.push(("+1/+1".to_owned(), "[CR#122.1a]"));
    named.push(("-1/-1".to_owned(), "[CR#122.1a]"));
    for (kind, rule) in &named {
        assert_eq!(
            declared.get(kind),
            Some(&Some(number(rule).to_owned())),
            "counter kind `{kind}` named by {rule}"
        );
    }
}

#[test]
fn phases_and_steps() {
    let rules = rules();
    let sources = Sources::load().unwrap();
    let declared: BTreeSet<String> = sources
        .turn_parts
        .iter()
        .filter_map(|part| part.noun.as_ref())
        .map(|noun| noun.entry().lemma.to_lowercase())
        .collect();
    let mut enumerated: Vec<String> = list(&rules, "[CR#500.1]", "in this order: ")
        .into_iter()
        .map(|phase| format!("{phase} phase"))
        .collect();
    for (rule, marker) in [
        ("[CR#501.1]", "in this order: "),
        ("[CR#506.1]", "which proceed in order: "),
        ("[CR#512.1]", "two steps: "),
    ] {
        enumerated.extend(
            list(&rules, rule, marker)
                .into_iter()
                .map(|step| format!("{step} step")),
        );
    }
    for part in enumerated {
        // Oracle names some steps without the head noun (`upkeep`).
        let bare = part.trim_end_matches(" step").to_owned();
        assert!(
            declared.contains(&part) || declared.contains(&bare),
            "turn part `{part}` [CR#500.1] is not declared"
        );
    }
}

#[test]
fn predefined_tokens() {
    let rules = rules();
    let sources = Sources::load().unwrap();
    let predefined = number("[CR#111.10]");
    let enumerated: Vec<String> = rules
        .keys()
        .filter(|rule| {
            rule.strip_prefix(predefined)
                .is_some_and(|letter| letter.len() == 1 && letter != ".")
        })
        .cloned()
        .collect();
    let declared: Vec<String> = sources
        .tokens
        .iter()
        .map(|token| token.rule.number().to_owned())
        .collect();
    assert_covers("predefined tokens [CR#111.10]", &enumerated, &[], &declared);
    for token in &sources.tokens {
        let text = &rules[token.rule.number()];
        assert!(
            text.contains(&format!(" {} token", token.name)),
            "{}: rule {} does not define a {} token",
            token.name,
            token.rule.number(),
            token.name
        );
    }
}
