mod common;

use common::assert_constituents;
use deckmaste_english_v3::grammar::Category;

#[test]
fn several_constituents_witness_low_of_attachment() {
    // Prismatic Strands: "Prevent all damage that sources of the color of your
    // choice would deal this turn."
    assert_constituents(
        "sources of the color of your choice",
        Category::NounPhrase,
        &[
            (Category::NounPhrase, "the color of your choice"),
            (Category::NounPhrase, "your choice"),
        ],
    );
}

#[test]
fn separate_assertions_witness_alternative_attachments() {
    // Seedborn Muse's complete ability permits attachment of "during..." to
    // "Untap..." or to "you control...".
    let text = "Untap all permanents you control during each other player's untap step.";
    assert_constituents(
        text,
        Category::Document,
        &[(Category::NounPhrase, "all permanents you control")],
    );
    assert_constituents(
        text,
        Category::Document,
        &[(
            Category::NounPhrase,
            "all permanents you control during each other player's untap step",
        )],
    );
}

#[test]
fn complete_constituent_surfaces_keep_source_capitalization() {
    // Ancestral Recall: "Target player draws three cards."
    assert_constituents(
        "Target player draws three cards.",
        Category::Document,
        &[
            (Category::NounPhrase, "Target player"),
            (Category::NounPhrase, "three cards"),
        ],
    );
}

#[test]
#[should_panic(expected = "no single Reading")]
fn a_substring_crossing_constituent_boundaries_is_rejected() {
    // Ancestral Recall: "Target player draws three cards."
    assert_constituents(
        "Target player draws three cards.",
        Category::Document,
        &[(Category::NounPhrase, "player draws")],
    );
}

#[test]
#[should_panic(expected = "no single Reading")]
fn constituents_from_different_readings_do_not_jointly_witness_an_analysis() {
    let text = "Untap all permanents you control during each other player's untap step.";
    assert_constituents(
        text,
        Category::Document,
        &[
            (Category::NounPhrase, "all permanents you control"),
            (
                Category::NounPhrase,
                "all permanents you control during each other player's untap step",
            ),
        ],
    );
}

#[test]
#[should_panic(expected = "no single Reading")]
fn a_matching_surface_at_the_wrong_category_is_rejected() {
    // Ancestral Recall's object is a Noun Phrase.
    assert_constituents(
        "Target player draws three cards.",
        Category::Document,
        &[(Category::PrepositionPhrase, "three cards")],
    );
}
