use deckmaste_english_v3::grammar::Category;

use crate::common;

#[test]
fn incubation_druid() {
    let card = "Incubation Druid";
    let text = "{T}: Add one mana of any type that a land you control could produce.";
    assert_eq!(
        common::readings_on_card(card, text, Category::Document).len(),
        2
    );
    common::assert_constituents_on_card(
        card,
        text,
        Category::Document,
        &[(
            Category::ObjectRelativeClause,
            "that a land you control could produce",
        )],
    );
}

#[test]
fn break_the_ice() {
    let card = "Break the Ice";
    let text = "Destroy target land that is snow or could produce {C}.";
    assert_eq!(
        common::readings_on_card(card, text, Category::Document).len(),
        1
    );
    common::assert_constituents_on_card(
        card,
        text,
        Category::Document,
        &[(Category::ManaPhrase, "{C}")],
    );
}

#[test]
fn dictate_of_karametra() {
    let card = "Dictate of Karametra";
    let text = "Whenever a player taps a land for mana, that player adds one mana of any type that land produced.";
    assert_eq!(
        common::readings_on_card(card, text, Category::Document).len(),
        4
    );
    common::assert_constituents_on_card(
        card,
        text,
        Category::Document,
        &[(Category::ObjectRelativeClause, "that land produced")],
    );
}

#[test]
fn pale_moon() {
    let card = "Pale Moon";
    let text = "Until end of turn, if a player taps a nonbasic land for mana, it produces colorless mana instead of any other type.";
    assert_eq!(
        common::readings_on_card(card, text, Category::Document).len(),
        3
    );
    common::assert_constituents_on_card(
        card,
        text,
        Category::Document,
        &[(
            Category::FinitePredicate,
            "produces colorless mana instead of any other type",
        )],
    );
}

#[test]
fn miraris_wake() {
    let card = "Mirari's Wake";
    let text = "Whenever you tap a land for mana, add one mana of any type that land produced.";
    assert_eq!(
        common::readings_on_card(card, text, Category::Document).len(),
        4
    );
    common::assert_constituents_on_card(
        card,
        text,
        Category::Document,
        &[(Category::ObjectRelativeClause, "that land produced")],
    );
}

#[test]
fn newly_covered_faces_keep_object_gap_relatives() {
    for (card, text, count, relative) in [
        (
            "Exotic Orchard",
            "{T}: Add one mana of any color that a land an opponent controls could produce.",
            2,
            "that a land an opponent controls could produce",
        ),
        (
            "Reflecting Pool",
            "{T}: Add one mana of any type that a land you control could produce.",
            2,
            "that a land you control could produce",
        ),
        (
            "Quirion Explorer",
            "{T}: Add one mana of any color that a land an opponent controls could produce.",
            2,
            "that a land an opponent controls could produce",
        ),
        (
            "Fellwar Stone",
            "{T}: Add one mana of any color that a land an opponent controls could produce.",
            2,
            "that a land an opponent controls could produce",
        ),
        (
            "Naga Vitalist",
            "{T}: Add one mana of any type that a land you control could produce.",
            2,
            "that a land you control could produce",
        ),
        (
            "Harvester Druid",
            "{T}: Add one mana of any color that a land you control could produce.",
            2,
            "that a land you control could produce",
        ),
        (
            "Sylvok Explorer",
            "{T}: Add one mana of any color that a land an opponent controls could produce.",
            2,
            "that a land an opponent controls could produce",
        ),
        (
            "Mana Flare",
            "Whenever a player taps a land for mana, that player adds one mana of any type that land produced.",
            4,
            "that land produced",
        ),
        (
            "Heartbeat of Spring",
            "Whenever a player taps a land for mana, that player adds one mana of any type that land produced.",
            4,
            "that land produced",
        ),
        (
            "Zhur-Taa Ancient",
            "Whenever a player taps a land for mana, that player adds one mana of any type that land produced.",
            4,
            "that land produced",
        ),
    ] {
        assert_eq!(
            common::readings_on_card(card, text, Category::Document).len(),
            count,
            "{card}"
        );
        common::assert_constituents_on_card(
            card,
            text,
            Category::Document,
            &[(Category::ObjectRelativeClause, relative)],
        );
    }
}
