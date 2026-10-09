use deckmaste_english_v3::grammar::Category;

use crate::common::readings;
use crate::common::readings_on_card;

#[test]
fn loyalty_costs_use_oracle_signs_and_cannot_join_cost_lists() {
    for text in [
        "-2: Draw a card.",
        "{1}, −2: Draw a card.",
        "−2, {T}: Draw a card.",
        "{1}, 0: Draw a card.",
        "0, {T}: Draw a card.",
    ] {
        assert!(readings(text, Category::Document).is_empty(), "{text}");
    }
    // Ob Nixilis, the Hate-Twisted, Jace Beleren, and Jace, the Mind
    // Sculptor supply these attested costs.
    for text in ["−2", "+2", "0"] {
        assert_eq!(readings(text, Category::Cost).len(), 1, "{text}");
    }
    assert_eq!(readings("-3/-0", Category::SlashPair).len(), 1);
}

#[test]
fn a_filled_passive_rejects_a_second_internalised_complement_adjunct() {
    assert_eq!(
        readings(
            "Slivers can't be blocked except by Slivers by Walls.",
            Category::Document
        )
        .len(),
        3,
    );
    assert_eq!(
        readings(
            "Slivers can't be blocked except by Slivers.",
            Category::Document
        )
        .len(),
        1,
    );
    assert_eq!(
        readings("Slivers can't be blocked by Walls.", Category::Document).len(),
        9
    );
}

#[test]
fn singular_determiners_resolve_subject_relative_agreement() {
    assert!(
        readings(
            "Each creature and planeswalker that are tapped gains flying.",
            Category::Document
        )
        .is_empty()
    );
    // Goblin Chainwhirler's Object-gap relative has no Subject agreement.
    assert_eq!(
        readings(
            "each creature and planeswalker they control",
            Category::NounPhrase
        )
        .len(),
        2
    );
    assert!(
        readings(
            "One creature and planeswalker you control gain flying.",
            Category::Document
        )
        .is_empty()
    );
}

#[test]
fn spore_cloud_distributive_np_coordination_agrees_singular() {
    let text = "Each attacking creature and each blocking creature doesn't untap during its controller's next untap step.";
    let values = readings_on_card("Spore Cloud", text, Category::Sentence);
    assert!(!values.is_empty());
    assert!(
        readings_on_card(
            "Spore Cloud",
            &text.replace("doesn't", "don't"),
            Category::Sentence
        )
        .is_empty()
    );
    assert!(
        readings(
            "Each creature and each player gain flying.",
            Category::Document
        )
        .is_empty()
    );
}

#[test]
fn unattested_quantified_auxiliary_and_central_positions_are_pruned() {
    for text in [
        "They are each dealt 2 damage.",
        "They have each drawn a card.",
        "You may still cast it.",
        "Still destroy target creature.",
    ] {
        assert!(readings(text, Category::Document).is_empty(), "{text}");
    }
}
