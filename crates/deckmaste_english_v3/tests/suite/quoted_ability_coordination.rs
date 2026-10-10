use deckmaste_english_v3::grammar::Category;
use deckmaste_english_v3::grammar::Reading;

use crate::common;

fn assert_unterminated_ability(text: &str, inner: &str) {
    let values = common::readings(text, Category::Document);
    assert!(!values.is_empty(), "{text}");
    let quotation = format!("\"{inner}\"");
    for value in values {
        let mut found = false;
        value
            .visit(&mut |node| {
                if let Reading::QuotedClause { clause, .. } = node
                    && node.realize(common::lexicon()).unwrap() == quotation
                {
                    assert_eq!(clause.category(), Category::Ability);
                    assert_eq!(clause.realize(common::lexicon()).unwrap(), inner);
                    found = true;
                }
            })
            .unwrap();
        assert!(found, "the omitted full stop belongs to a quoted Ability");
    }
}

#[test]
fn card_witnesses_quote_static_triggered_and_activated_abilities() {
    for (text, inner) in [
        (
            "Equipped creature has \"You may look at the top card of your library any time\" and \"Whenever this creature attacks, it explores.\"",
            "You may look at the top card of your library any time",
        ),
        (
            "−4: You get an emblem with \"Artifact creatures you control get +1/+1\" and \"Artifact spells you cast cost {1} less to cast.\"",
            "Artifact creatures you control get +1/+1",
        ),
        (
            "When this creature enters, each opponent creates a 1/1 red Pirate creature token with \"This token can't block\" and \"Creatures you control attack each combat if able.\"",
            "This token can't block",
        ),
        (
            "When this creature enters, create a colorless Equipment artifact token named Axe with \"Equipped creature gets +1/+0\" and equip {2}.",
            "Equipped creature gets +1/+0",
        ),
        (
            "Equipped creature has \"Whenever you draw a card, this creature gets +1/+1 and gains flying until end of turn\" and \"{4}: Draw a card.\"",
            "Whenever you draw a card, this creature gets +1/+1 and gains flying until end of turn",
        ),
    ] {
        assert_unterminated_ability(text, inner);
    }
}

#[test]
fn both_coordinates_keep_the_granted_ability_function() {
    for (text, coordinated) in [
        (
            "It has \"{T}: Draw a card\" and \"{T}: Add {G}.\"",
            "\"{T}: Draw a card\" and \"{T}: Add {G}.\"",
        ),
        (
            "It has \"{T}: Add {G}\" and flying.",
            "\"{T}: Add {G}\" and flying",
        ),
        (
            "It has flying and \"{T}: Add {G}.\"",
            "flying and \"{T}: Add {G}.\"",
        ),
    ] {
        let values = common::readings(text, Category::Document);
        assert!(!values.is_empty(), "{text}");
        for value in values {
            let mut found = false;
            value
                .visit(&mut |node| {
                    if let Reading::Coordination {
                        category,
                        left,
                        right,
                        ..
                    } = node
                        && *category == Category::GrantedAbility
                        && node.realize(common::lexicon()).unwrap() == coordinated
                    {
                        assert_eq!(left.category(), Category::GrantedAbility);
                        assert_eq!(right.category(), Category::GrantedAbility);
                        found = true;
                    }
                })
                .unwrap();
            assert!(found, "both coordinates are Granted Abilities");
        }
    }
}

#[test]
fn following_adjunct_and_predicate_coordinate_leave_the_ability_unterminated() {
    assert_unterminated_ability(
        "Target creature gains \"{T}: Add {G}\" until end of turn.",
        "{T}: Add {G}",
    );
    assert_unterminated_ability(
        "It has \"This creature can't block\" and is a Wizard.",
        "This creature can't block",
    );
}

#[test]
fn complete_and_lowercase_quotations_have_no_ability_twin() {
    // QA3 requires one quotation analysis, independently of matrix ambiguity.
    let complete = common::readings("\"{T}: Draw a card.\"", Category::QuotedText);
    assert_eq!(
        complete.len(),
        1,
        "the full stop stays inside the quotation"
    );
    for value in complete {
        let Reading::QuotedText { text, .. } = value else {
            panic!("a completed quotation retains its Document");
        };
        assert_eq!(text.category(), Category::Document);
    }
    let fragment = common::readings("\"this creature can't be blocked\"", Category::QuotedText);
    let expected: std::collections::BTreeSet<_> =
        common::readings("this creature can't be blocked", Category::Clause)
            .into_iter()
            .map(|clause| Reading::QuotedClause {
                form: 0,
                clause: Box::new(clause),
            })
            .collect();
    assert_eq!(
        fragment, expected,
        "the lower-case fragment retains only its Clauses"
    );
}

#[test]
fn omitted_sentence_terminal_is_confined_to_the_quotation() {
    for text in [
        "Draw a card",
        "Draw a card Draw a card.",
        "It has \"{T}: Draw a card.\".",
        "It has \"Draw a card. Draw a card\" and flying.",
    ] {
        assert!(
            common::readings(text, Category::Document).is_empty(),
            "{text}"
        );
    }
    // QA5 preserves the existing completed route without adding a blocker.
    // A completed ability cannot also select the new unterminated route.
    for ability in common::readings("{T}: Draw a card.", Category::Ability) {
        let quotation = Reading::QuotedClause {
            form: 1,
            clause: Box::new(ability),
        };
        assert!(quotation.admit(common::lexicon()).is_err());
    }
}

#[test]
fn independently_composed_unterminated_quotation_roundtrips() {
    // Glowcap Lantern's first quoted static ability. The independently chosen
    // quotation, ability and sentence forms must admit the same Clause value.
    let inner = "You may look at the top card of your library any time";
    let clauses = common::readings(inner, Category::Clause);
    assert!(!clauses.is_empty());
    for clause in clauses {
        let value = Reading::QuotedClause {
            form: 1,
            clause: Box::new(Reading::OrdinaryAbility {
                form: 1,
                body: Box::new(Reading::Sentence {
                    form: 1,
                    clause: Box::new(clause),
                }),
            }),
        };
        value.admit(common::lexicon()).unwrap();
        let surface = value.realize(common::lexicon()).unwrap();
        assert_eq!(surface, format!("\"{inner}\""));
        assert!(common::readings(&surface, Category::QuotedText).contains(&value));
    }
}
