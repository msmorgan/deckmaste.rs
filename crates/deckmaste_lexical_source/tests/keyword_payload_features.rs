use std::path::Path;
use std::sync::LazyLock;

use deckmaste_lexical::{Category, Lexeme};

static SOURCES: LazyLock<deckmaste_lexical_source::LexicalSources> = LazyLock::new(|| {
    deckmaste_lexical_source::load_workspace(&Path::new(env!("CARGO_MANIFEST_DIR")).join("../.."))
        .unwrap()
});

fn entry(lemma: &str, category: Category) -> &'static Lexeme {
    let matches: Vec<_> = SOURCES
        .lexemes
        .iter()
        .filter(|entry| entry.lemma == lemma && entry.category == category)
        .collect();
    assert_eq!(matches.len(), 1, "{lemma}");
    matches[0]
}

fn feature(entry: &Lexeme, name: &str, expected: &str) {
    assert_eq!(
        entry.properties.features.get(name).map(String::as_str),
        Some(expected),
        "{} {name}",
        entry.id
    );
}

#[test]
fn authored_keyword_payload_contracts_preserve_separators_and_markers() {
    // Oracle witnesses: Myr Enforcer, Pacifism, Rift Bolt, Kodama's Might,
    // Rust Goliath, Usher of the Fallen, and Lutri, the Spellchaser.
    for (lemma, class, marker, head, internal) in [
        ("affinity", "Quality", "For", "Space", "Space"),
        ("enchant", "Subject", "None", "Space", "Space"),
        ("suspend", "AmountCost", "None", "Space", "Dash"),
        ("splice", "QualityCost", "Onto", "Space", "Space"),
        (
            "prototype",
            "CostPowerToughness",
            "None",
            "Space",
            "SpacedDash",
        ),
        ("boast", "Ability", "None", "SpacedDash", "Space"),
        ("companion", "Condition", "None", "SpacedDash", "Space"),
    ] {
        let keyword = entry(lemma, Category::Keyword);
        feature(keyword, "KeywordParameterClass", class);
        feature(keyword, "KeywordMarker", marker);
        feature(keyword, "KeywordSeparator", head);
        feature(keyword, "KeywordParameterSeparator", internal);
    }
    feature(
        entry("offering", Category::Keyword),
        "KeywordPayloadOrder",
        "BeforeHead",
    );
    feature(
        entry("landwalk", Category::Keyword),
        "KeywordPayloadOrder",
        "BoundSuffix",
    );
    feature(entry("landwalk", Category::Keyword), "BoundKeyword", "No");
    feature(
        entry("affinity", Category::Keyword),
        "KeywordQualityNumber",
        "Plural",
    );
    feature(
        entry("splice", Category::Keyword),
        "KeywordQualityNumber",
        "Any",
    );
    for (marker, expected) in [("for", "For"), ("from", "From"), ("onto", "Onto")] {
        let keyword_marker = entry(marker, Category::Preposition);
        feature(keyword_marker, "KeywordMarker", expected);
    }
}

#[test]
fn explicit_bare_and_adjunct_licenses_override_category_defaults() {
    for (noun, bare, adjunct) in [
        ("turn", "Interval", "Temporal"),
        ("step", "None", "Temporal"),
        ("end", "Boundary", "None"),
        ("way", "None", "Manner"),
        ("creature", "None", "None"),
    ] {
        let noun = entry(noun, Category::Noun);
        feature(noun, "NominalBareClass", bare);
        feature(noun, "NominalAdjunctClass", adjunct);
    }
    for adjective in ["equipped", "enchanted", "fortified"] {
        feature(
            entry(adjective, Category::Adjective),
            "BareSingularUse",
            "Yes",
        );
    }
    feature(
        entry("kicked", Category::Adjective),
        "BareSingularUse",
        "No",
    );
    feature(entry("who", Category::Pronoun), "RelativeUse", "Yes");
    for pronoun in SOURCES
        .lexemes
        .iter()
        .filter(|entry| entry.lemma == "it" && entry.category == Category::Pronoun)
    {
        feature(pronoun, "RelativeUse", "No");
    }
    feature(
        entry("until", Category::Preposition),
        "NominalBareClass",
        "Boundary",
    );
    feature(
        entry("of", Category::Preposition),
        "NominalBareClass",
        "Interval",
    );
    feature(
        entry("this", Category::Determinative),
        "NominalAdjunctDeterminer",
        "Demonstrative",
    );
    feature(
        entry("each", Category::Determinative),
        "NominalAdjunctDeterminer",
        "Temporal",
    );
}

#[test]
fn keyword_suffixes_keep_their_binding_and_supplement_identity() {
    let suffixes: Vec<_> = SOURCES
        .lexemes
        .iter()
        .filter(|entry| entry.id.ends_with("/bound-suffix"))
        .collect();
    assert_ne!(suffixes, [] as [&deckmaste_lexical::Lexeme; 0]);
    for suffix in suffixes {
        assert_eq!(suffix.binding, deckmaste_lexical::Binding::Suffix);
        assert_eq!(suffix.category, Category::Keyword);
        feature(suffix, "KeywordParameterClass", "Quality");
        feature(suffix, "KeywordPayloadOrder", "BoundSuffix");
        feature(suffix, "BoundKeyword", "Yes");
        feature(suffix, "KeywordMarker", "None");
        feature(suffix, "KeywordQualityNumber", "Any");
        assert_ne!(suffix.id.as_str(), suffix.source.owner);
    }
}

#[test]
fn selected_markers_and_recipient_first_deal_are_declared_lexical_frames() {
    for (lemma, marker) in [
        ("to", "To"),
        ("into", "Into"),
        ("at", "At"),
        ("with", "With"),
        ("of", "Of"),
        ("on", "On"),
        ("under", "Under"),
    ] {
        feature(entry(lemma, Category::Preposition), "KeywordMarker", marker);
    }
    // Trapjaw Tyrant: “Whenever this creature is dealt damage, ...”.
    let object = deckmaste_lexical::FrameItem::Argument(deckmaste_lexical::FrameSlot {
        relation: deckmaste_lexical::Relation::Object,
        category: "NounPhrase".into(),
    });
    let deal = entry("deal", Category::Verb);
    assert_eq!(deal.id, "core-verb:Deal");
    assert_eq!(
        deal.properties.frames.last(),
        Some(&deckmaste_lexical::Frame {
            kind: "Predicate".into(),
            items: vec![object.clone(), object],
        })
    );
    assert_eq!(deal.properties.frames.len(), 7);
}

#[test]
fn complete_put_and_look_frames_preserve_their_typed_np_slots() {
    let object = deckmaste_lexical::FrameItem::Argument(deckmaste_lexical::FrameSlot {
        relation: deckmaste_lexical::Relation::Object,
        category: "NounPhrase".into(),
    });
    let marker = |member: &str| deckmaste_lexical::FrameItem::Marker {
        vocabulary: "Preposition".into(),
        member: member.into(),
    };
    // Boldwyr Heavyweights: “put it onto the battlefield”.
    let put = entry("put", Category::Verb);
    assert_eq!(put.properties.frames.len(), 7);
    assert_eq!(
        put.properties.frames[6],
        deckmaste_lexical::Frame {
            kind: "Predicate".into(),
            items: vec![object.clone(), marker("Onto"), object.clone()],
        }
    );
    // Genesis Ultimatum: “Look at the top five cards of your library”.
    let look = entry("look", Category::Verb);
    assert_eq!(look.properties.frames.len(), 2);
    assert_eq!(
        look.properties.frames[1],
        deckmaste_lexical::Frame {
            kind: "Predicate".into(),
            items: vec![marker("At"), object],
        }
    );
    assert_ne!(look.properties.frames[0], look.properties.frames[1]);
}
