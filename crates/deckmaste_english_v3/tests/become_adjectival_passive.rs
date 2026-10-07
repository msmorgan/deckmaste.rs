mod common;

use std::collections::BTreeSet;

use common::assert_constituents;
use common::lexicon;
use common::readings;
use deckmaste_english_v3::grammar::Category;
use deckmaste_english_v3::grammar::FrameValue;
use deckmaste_english_v3::grammar::Reading;
use deckmaste_english_v3::grammar::Word;
use deckmaste_lexical::LexicalReading;
use deckmaste_lexical::Number;
use deckmaste_lexical::Person;
use deckmaste_lexical::SurfaceCase;
use deckmaste_lexical::WordForm;

#[test]
fn flint_golem_becomes_blocked_has_an_adjectival_predicative_complement() {
    let text = "Whenever this creature becomes blocked, defending player mills three cards.";
    assert_eq!(readings(text, Category::Document).len(), 1);
    assert_constituents(
        text,
        Category::Document,
        &[
            (Category::FinitePredicate, "becomes blocked"),
            (Category::PredicativeComplement, "blocked"),
            (Category::AdjectivePhrase, "blocked"),
        ],
    );
}

#[test]
fn talruum_champion_keeps_the_selected_by_complement_inside_the_adjective() {
    let text = "Whenever this creature blocks or becomes blocked by a creature, that creature loses first strike until end of turn.";
    assert_eq!(readings(text, Category::Document).len(), 9);
    assert_constituents(
        text,
        Category::Document,
        &[
            (
                Category::FinitePredicate,
                "blocks or becomes blocked by a creature",
            ),
            (Category::PredicativeComplement, "blocked by a creature"),
            (Category::AdjectivePhrase, "blocked by a creature"),
            (Category::AccusativePhrase, "a creature"),
        ],
    );
}

#[test]
fn vedalken_ghoul_trained_cheetah_and_somberwald_alpha_preserve_the_adjective() {
    for (text, count) in [
        (
            "Whenever this creature becomes blocked, defending player loses 4 life.",
            1,
        ),
        (
            "Whenever this creature becomes blocked, it gets +1/+1 until end of turn.",
            3,
        ),
        (
            "Whenever a creature you control becomes blocked, it gets +1/+1 until end of turn.",
            3,
        ),
    ] {
        assert_eq!(readings(text, Category::Document).len(), count, "{text}");
        assert_constituents(
            text,
            Category::Document,
            &[
                (Category::FinitePredicate, "becomes blocked"),
                (Category::PredicativeComplement, "blocked"),
                (Category::AdjectivePhrase, "blocked"),
            ],
        );
    }
}

fn word(owner: &str, form: WordForm, frame: Option<usize>) -> Word {
    Word {
        value: LexicalReading::Word(
            lexicon()
                .values()
                .find(|value| {
                    value.lexeme == owner
                        && value.form == form
                        && value.capitalization == SurfaceCase::Declared
                        && (form != WordForm::Present
                            || (value.features.number == Some(Number::Singular)
                                && value.features.person == Some(Person::Third)))
                })
                .unwrap_or_else(|| panic!("missing lexical declaration {owner}"))
                .clone(),
        ),
        frame,
        countability: None,
    }
}

fn becomes(phrase: Reading) -> Reading {
    Reading::SelectedPredicate {
        category: Category::FinitePredicate,
        form: 0,
        head: word("core-verb:Become", WordForm::Present, Some(0)),
        complements: vec![FrameValue::Argument(Box::new(
            Reading::AdjectivalComplement {
                form: 0,
                phrase: Box::new(phrase),
            },
        ))],
    }
}

fn laws(expected: &Reading, text: &str) {
    expected.admit(lexicon()).unwrap();
    assert_eq!(expected.realize(lexicon()).unwrap(), text);
    let parsed = readings(text, expected.category());
    assert!(
        parsed.contains(expected),
        "missing independent Reading for {text}"
    );
    let actual = parsed.get(expected).unwrap();
    let mut expected_nodes = vec![];
    let mut actual_nodes = vec![];
    expected
        .visit(&mut |node| expected_nodes.push(node.clone()))
        .unwrap();
    actual
        .visit(&mut |node| actual_nodes.push(node.clone()))
        .unwrap();
    assert_eq!(actual_nodes, expected_nodes);
    let mut expected_words = vec![];
    let mut actual_words = vec![];
    expected
        .visit_words(&mut |word| expected_words.push(word.clone()))
        .unwrap();
    actual
        .visit_words(&mut |word| actual_words.push(word.clone()))
        .unwrap();
    assert_eq!(actual_words, expected_words);
}

#[test]
fn independent_simple_adjectives_preserve_identity_and_have_one_route() {
    // Flint Golem; Fallowsage; Mesmeric Orb.
    for (owner, text, framed) in [
        ("core-verb:Block/adjective", "becomes blocked", true),
        (
            "lexeme:keyword_action/tap/adjective",
            "becomes tapped",
            false,
        ),
        (
            "lexeme:keyword_action/untap/adjective",
            "becomes untapped",
            false,
        ),
    ] {
        let head = word(owner, WordForm::Invariant, framed.then_some(0));
        let phrase = if framed {
            Reading::IntransitiveAdjective { form: 0, head }
        } else {
            Reading::Adjective { form: 0, head }
        };
        let expected = becomes(phrase);
        laws(&expected, text);
        assert_eq!(
            readings(text, Category::FinitePredicate),
            BTreeSet::from([expected])
        );
    }
}

#[test]
fn independent_complemented_adjective_selects_its_marker_and_object() {
    // Talruum Champion.
    let creature = Reading::CasePhrase {
        category: Category::AccusativePhrase,
        form: 0,
        head: Box::new(Reading::IndefiniteNounPhrase {
            form: 0,
            determiner: word("vocab:Article/Indefinite", WordForm::Invariant, None),
            head: Box::new(Reading::Noun {
                form: 0,
                head: Word {
                    countability: Some(true),
                    ..word("lexeme:type/creature", WordForm::Singular, None)
                },
            }),
        }),
    };
    let phrase = Reading::ComplementedAdjective {
        form: 0,
        head: word("core-verb:Block/adjective", WordForm::Invariant, Some(1)),
        marker: word("vocab:Preposition/By", WordForm::Invariant, None),
        complement: Box::new(creature),
    };
    laws(&becomes(phrase.clone()), "becomes blocked by a creature");
    assert_eq!(
        readings("blocked by a creature", Category::AdjectivePhrase),
        BTreeSet::from([phrase.clone()])
    );
    let mut wrong_frame = phrase.clone();
    if let Reading::ComplementedAdjective { head, .. } = &mut wrong_frame {
        head.frame = Some(0);
    }
    assert!(wrong_frame.admit(lexicon()).is_err());
    let mut wrong_marker = phrase.clone();
    if let Reading::ComplementedAdjective { marker, .. } = &mut wrong_marker {
        *marker = word("vocab:Preposition/With", WordForm::Invariant, None);
    }
    assert!(wrong_marker.admit(lexicon()).is_err());
    let mut wrong_head = phrase;
    if let Reading::ComplementedAdjective { head, .. } = &mut wrong_head {
        *head = word(
            "lexeme:keyword_action/tap/adjective",
            WordForm::Invariant,
            None,
        );
    }
    assert!(wrong_head.admit(lexicon()).is_err());
}

#[test]
fn adjectival_complements_cannot_take_objects_or_verbal_inflections() {
    for text in [
        "becomes blocked a creature",
        "becomes blocks",
        "becomes blocking",
    ] {
        assert!(
            readings(text, Category::FinitePredicate).is_empty(),
            "{text}"
        );
    }
}
