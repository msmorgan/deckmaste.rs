use deckmaste_english_v3::grammar::Category;
use deckmaste_english_v3::grammar::Reading;

use crate::common;

fn count(text: &str, expected: usize) {
    let values = common::readings(text, Category::Document);
    eprintln!("{text:?}: {} Readings", values.len());
    assert_eq!(values.len(), expected, "{text}");
    for value in values {
        value.admit(common::lexicon()).unwrap();
        value.visit(&mut |_| {}).unwrap();
    }
}

#[test]
fn unattested_have_nominal_exception_is_rejected() {
    count("This creature has flying except for three.", 0);
}

#[test]
fn scheming_fence_retains_its_loyalty_ability_residual() {
    count(
        "This creature has all activated abilities of the chosen permanent except for loyalty abilities.",
        0,
    );
}

#[test]
fn passive_host_with_an_internalised_marker_rejects_another_internalised_complement() {
    count("Slivers can't be blocked by Walls except by Slivers.", 0);
}

#[test]
fn shifting_sliver_keeps_its_unfilled_passive_exception() {
    count("Slivers can't be blocked except by Slivers.", 1);
}

#[test]
fn restricted_pp_cannot_be_laundered_through_an_embedded_complement() {
    count("Exile a card of without paying its mana cost.", 0);
    assert!(common::readings("of of paying its mana cost", Category::PrepositionPhrase).is_empty());
}

#[test]
fn satyr_wayfinder_keeps_its_nested_source_complement() {
    common::assert_constituents(
        "You may put a land card from among them into your hand.",
        Category::Document,
        &[
            (Category::PrepositionPhrase, "from among them"),
            (Category::PrepositionPhrase, "among them"),
        ],
    );
}

fn matrix_exception(text: &str, host: &str, modifier: &str) {
    let values = common::readings(text, Category::Document);
    eprintln!("{text:?}: {} Readings", values.len());
    assert!(!values.is_empty(), "{text}");
    for value in values {
        let mut attachments = Vec::new();
        value
            .visit(&mut |node| {
                if let Reading::LicensedMatrixComplementPredicate { head, modifier, .. } = node {
                    attachments.push((
                        head.realize(common::lexicon()).unwrap(),
                        modifier.realize(common::lexicon()).unwrap(),
                    ));
                }
            })
            .unwrap();
        assert_eq!(attachments, vec![(host.into(), modifier.into())]);
        value.admit(common::lexicon()).unwrap();
    }
}

#[test]
fn season_of_the_witch_nominal_exception_allows_a_comma() {
    // Season of the Witch's consequent.
    matrix_exception(
        "Destroy all creatures, except for Walls.",
        "Destroy all creatures",
        "except for Walls",
    );
}

#[test]
fn content_exception_allows_a_space() {
    // Quantum Misalignment's copy description; bare separator attested on
    // Faerie Artisans.
    matrix_exception(
        "Create a token that's a copy of target creature except it isn't legendary.",
        "Create a token that's a copy of target creature",
        "except it isn't legendary",
    );
}

#[test]
fn separator_does_not_supply_a_missing_host_licence() {
    count("Draw a card except for three.", 0);
    count("Draw a card, except for three.", 0);
}
