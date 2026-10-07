mod common;

use common::{lexicon, readings};
use deckmaste_english_v3::grammar::{Category, FrameValue, Reading};
use deckmaste_lexical::LexicalReading;

fn source_complement(text: &str, category: Category, object: &str, source: &str) {
    let values = readings(text, category);
    assert!(!values.is_empty(), "missing Reading of {text}");
    for value in values {
        let mut sources = Vec::new();
        value
            .visit(&mut |node| {
                if let Reading::SelectedPredicate {
                    head, complements, ..
                } = node
                    && let LexicalReading::Word(word) = &head.value
                    && word.lexeme == "core-verb:Remove"
                {
                    let [
                        FrameValue::Argument(theme),
                        FrameValue::Marked { marker, argument },
                    ] = complements.as_slice()
                    else {
                        panic!("Remove must retain its Object and selected source: {node:?}");
                    };
                    assert_eq!(theme.realize(lexicon()).unwrap(), object);
                    assert_eq!(lexicon().realize(&marker.value).unwrap(), "from");
                    assert_eq!(argument.realize(lexicon()).unwrap(), source);
                    sources.push(node.clone());
                }
            })
            .unwrap();
        assert!(!sources.is_empty(), "missing selected source in {text}");
    }
}

#[test]
fn spitting_hydra_remove_cost_selects_source() {
    source_complement(
        "{1}{R}, Remove a +1/+1 counter from this creature: It deals 1 damage to target creature.",
        Category::Document,
        "a +1/+1 counter",
        "this creature",
    );
}

#[test]
fn timberline_ridge_trigger_selects_source() {
    source_complement(
        "At the beginning of your upkeep, remove a depletion counter from this land.",
        Category::Document,
        "a depletion counter",
        "this land",
    );
}

#[test]
fn phyrexian_prowler_remove_cost_selects_source() {
    source_complement(
        "Remove a fade counter from this creature: This creature gets +1/+1 until end of turn.",
        Category::Document,
        "a fade counter",
        "this creature",
    );
}

#[test]
fn voracious_hatchling_triggers_select_source() {
    for color in ["white", "black"] {
        source_complement(
            &format!(
                "Whenever you cast a {color} spell, remove a -1/-1 counter from this creature."
            ),
            Category::Document,
            "a -1/-1 counter",
            "this creature",
        );
    }
}

#[test]
fn perfect_intimidation_imperative_selects_source() {
    source_complement(
        "Remove all counters from target creature.",
        Category::Document,
        "all counters",
        "target creature",
    );
}

#[test]
fn junk_golem_finite_clause_selects_source() {
    // Junk Golem: "...unless you remove a +1/+1 counter from it."
    source_complement(
        "you remove a +1/+1 counter from it",
        Category::FiniteClause,
        "a +1/+1 counter",
        "it",
    );
}

#[test]
fn sun_droplet_modal_selects_source() {
    source_complement(
        "At the beginning of each upkeep, you may remove a charge counter from this artifact.",
        Category::Document,
        "a charge counter",
        "this artifact",
    );
}
