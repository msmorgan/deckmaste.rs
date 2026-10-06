mod common;

use common::assert_constituent_occurrences;
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
fn seedborn_muse_retains_exactly_two_adjunct_attachments() {
    // Seedborn Muse permits VP attachment to Untap and attachment inside the
    // Object Relative Clause to control. Nominal attachment is unlicensed.
    let text = "Untap all permanents you control during each other player's untap step.";
    assert_constituents(
        text,
        Category::Document,
        &[
            (Category::NounPhrase, "all permanents you control"),
            (
                Category::SecondaryVerbPhrase,
                "Untap all permanents you control",
            ),
            (
                Category::SecondaryVerbPhrase,
                "Untap all permanents you control during each other player's untap step",
            ),
        ],
    );
    assert_constituents(
        text,
        Category::Document,
        &[(
            Category::ObjectRelativeClause,
            "you control during each other player's untap step",
        )],
    );
    let mut attachments = std::collections::BTreeSet::new();
    for reading in common::readings(text, Category::Document) {
        reading
            .visit(&mut |node| {
                if let deckmaste_english_v3::grammar::Reading::PostmodifiedNominal {
                    modifier,
                    ..
                } = node
                {
                    assert_ne!(
                        modifier.realize(common::lexicon()).unwrap(),
                        "during each other player's untap step",
                        "During must not function as a nominal Modifier",
                    );
                }
                if node.category() == Category::ObjectRelativeClause {
                    attachments.insert(node.realize(common::lexicon()).unwrap());
                }
            })
            .unwrap();
    }
    assert_eq!(
        attachments,
        std::collections::BTreeSet::from([
            "you control".into(),
            "you control during each other player's untap step".into(),
        ])
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
    // The shorter Object Noun Phrase excludes the relative-clause Adjunct;
    // the longer one includes it. They occur in different Readings.
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

#[test]
fn repeated_surfaces_choose_each_constituents_actual_occurrence() {
    // Gravedigger: the first "creature" is a Nominal head; the second is
    // a noun Premodifier in "creature card".
    let text = "When this creature enters, you may return target creature card from your graveyard to your hand.";
    assert_constituent_occurrences(
        text,
        Category::Document,
        &[
            (Category::Nominal, "creature", 0),
            (Category::NounPremodifier, "creature", 1),
        ],
    );
}

#[test]
#[should_panic(expected = "choose an occurrence explicitly")]
fn an_unqualified_repeated_surface_is_rejected() {
    // Gravedigger's two occurrences must not be interchangeable.
    assert_constituents(
        "When this creature enters, you may return target creature card from your graveyard to your hand.",
        Category::Document,
        &[(Category::Nominal, "creature")],
    );
}

#[test]
#[should_panic(expected = "no single Reading")]
fn a_chosen_occurrence_cannot_match_a_constituent_elsewhere() {
    // The second "creature" in Gravedigger is a Premodifier, not a Nominal.
    assert_constituent_occurrences(
        "When this creature enters, you may return target creature card from your graveyard to your hand.",
        Category::Document,
        &[(Category::Nominal, "creature", 1)],
    );
}
