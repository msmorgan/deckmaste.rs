mod common;

use common::{assert_constituents, lexicon, readings};
use deckmaste_english_v3::grammar::Category;

fn interval_document(text: &str, constituents: &[(Category, &str)]) {
    assert_constituents(text, Category::Document, constituents);
    for reading in readings(text, Category::Document) {
        reading
            .visit(&mut |node| {
                if node.category() == Category::NounPremodifier {
                    assert_ne!(node.realize(lexicon()).unwrap(), "combat");
                }
            })
            .unwrap();
    }
}

#[test]
fn dire_fleet_warmonger_keeps_the_trigger_attachment() {
    interval_document(
        "At the beginning of combat on your turn, you may sacrifice another creature.",
        &[
            (
                Category::PrepositionPhrase,
                "At the beginning of combat on your turn",
            ),
            (Category::PrepositionPhrase, "on your turn"),
            (Category::BareTemporalNominal, "combat"),
        ],
    );
}

#[test]
fn might_of_the_ancestors_keeps_the_trigger_attachment() {
    interval_document(
        "At the beginning of combat on your turn, target creature you control gets +2/+0 and gains vigilance until end of turn.",
        &[
            (
                Category::PrepositionPhrase,
                "At the beginning of combat on your turn",
            ),
            (Category::PrepositionPhrase, "on your turn"),
            (Category::BareTemporalNominal, "combat"),
        ],
    );
}

#[test]
fn glyph_of_destruction_keeps_the_bare_boundary() {
    interval_document(
        "Target blocking Wall you control gets +10/+0 until end of combat.",
        &[
            (Category::PrepositionPhrase, "until end of combat"),
            (Category::BareTemporalNominal, "combat"),
        ],
    );
}

#[test]
fn phantom_whelp_keeps_the_bare_boundary() {
    interval_document(
        "When this creature attacks or blocks, return it to its owner's hand at end of combat.",
        &[
            (Category::PrepositionPhrase, "at end of combat"),
            (Category::BareTemporalNominal, "combat"),
        ],
    );
}

#[test]
fn echo_circlet_keeps_the_temporal_adjunct() {
    interval_document(
        "Equipped creature can block an additional creature each combat.",
        &[
            (Category::NominalAdjunctPhrase, "each combat"),
            (Category::Nominal, "combat"),
        ],
    );
}

#[test]
fn combat_damage_keeps_its_premodifier_analysis() {
    // Fog: "Prevent all combat damage that would be dealt this turn."
    assert_constituents(
        "combat damage",
        Category::Nominal,
        &[
            (Category::NounPremodifier, "combat"),
            (Category::Nominal, "damage"),
        ],
    );
    let actual = readings("combat damage", Category::Nominal);
    assert_eq!(actual.len(), 1);
    assert!(readings("combat", Category::NounPhrase).is_empty());
    assert!(readings("each combats", Category::NominalAdjunctPhrase).is_empty());
}

#[test]
fn bare_during_combat_is_an_adjunct_with_an_interval_complement() {
    // Basandra, Battle Seraph.
    assert_constituents(
        "Players can't cast spells during combat.",
        Category::Document,
        &[
            (Category::PrepositionPhrase, "during combat"),
            (Category::BareTemporalNominal, "combat"),
        ],
    );
}
