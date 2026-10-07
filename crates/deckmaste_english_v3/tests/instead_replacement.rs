mod common;

use std::collections::BTreeSet;

use common::assert_constituents;
use common::lexicon;
use common::readings;
use deckmaste_english_v3::grammar::Category;
use deckmaste_english_v3::grammar::Reading;
use deckmaste_english_v3::grammar::Word;
use deckmaste_lexical::FeatureBundle;
use deckmaste_lexical::LexicalReading;
use deckmaste_lexical::LexicalValue;
use deckmaste_lexical::SurfaceCase;
use deckmaste_lexical::WordForm;

#[test]
fn replacement_witnesses_retain_their_hosts_and_one_preposition_owner() {
    let witnesses = [
        // Forbidden Crypt's first replacement sentence.
        "If you would draw a card, return a card from your graveyard to your hand instead.",
        // Burn the Accursed.
        "If that creature would die this turn, exile it instead.",
        // Soldevi Excavations.
        "If this land would enter, sacrifice an untapped Island instead.",
        // Increasing Savagery: a non-modal conditional host.
        "If this spell was cast from a graveyard, put ten +1/+1 counters on that creature instead.",
        // Divine Resilience: an unpunctuated initial connective.
        "If this spell was kicked, instead any number of target creatures you control gain indestructible until end of turn.",
    ];
    for text in witnesses {
        let twin = text
            .replace(" instead of any other type", "")
            .replace("instead ", "")
            .replace(" instead", "");
        assert!(
            !readings(&twin, Category::Document).is_empty(),
            "host: {twin}"
        );
        let values = readings(text, Category::Document);
        assert!(!values.is_empty(), "replacement: {text}");
        for value in values {
            let mut owners = Vec::new();
            value
                .visit_words(&mut |word| {
                    if let LexicalReading::Word(word) = &word.value
                        && lexicon().lexemes()[&word.lexeme].lemma == "instead"
                    {
                        owners.push(word.lexeme);
                    }
                })
                .unwrap();
            assert_eq!(
                owners,
                vec![deckmaste_lexical::LexemeId::from(
                    "vocab:Preposition/Instead"
                )]
            );
            value
                .visit(&mut |node| {
                    assert!(
                        !matches!(node, Reading::OmittedPlain { .. }),
                        "no stranded auxiliary in {text}"
                    );
                    if let Reading::PostmodifiedNominal { modifier, .. } = node {
                        assert!(!modifier.realize(lexicon()).unwrap().starts_with("instead"));
                    }
                })
                .unwrap();
        }
    }
}

#[test]
fn bare_instead_is_a_preposition_phrase_with_no_nominal_function() {
    let expected = Reading::IntransitivePreposition {
        form: 0,
        head: Word {
            value: LexicalReading::Word(LexicalValue {
                lexeme: "vocab:Preposition/Instead".into(),
                form: WordForm::Invariant,
                features: FeatureBundle::default(),
                variant: 0,
                capitalization: SurfaceCase::Declared,
            }),
            frame: None,
            countability: None,
        },
    };
    expected.admit(lexicon()).unwrap();
    assert_eq!(expected.realize(lexicon()).unwrap(), "instead");
    let values = readings("instead", Category::PrepositionPhrase);
    assert_eq!(values, BTreeSet::from([expected.clone()]));
    let actual = values.get(&expected).unwrap();
    let mut expected_nodes = Vec::new();
    let mut actual_nodes = Vec::new();
    expected
        .visit(&mut |node| expected_nodes.push(node.clone()))
        .unwrap();
    actual
        .visit(&mut |node| actual_nodes.push(node.clone()))
        .unwrap();
    assert_eq!(actual_nodes, expected_nodes);
    let mut expected_words = Vec::new();
    let mut actual_words = Vec::new();
    expected
        .visit_words(&mut |word| expected_words.push(word.clone()))
        .unwrap();
    actual
        .visit_words(&mut |word| actual_words.push(word.clone()))
        .unwrap();
    assert_eq!(actual_words, expected_words);
    assert!(readings("instead", Category::AdverbPhrase).is_empty());
    assert!(readings("instead", Category::NounPhrase).is_empty());
    assert!(readings("instead of", Category::PrepositionPhrase).is_empty());
    assert!(readings("instead any other type", Category::PrepositionPhrase).is_empty());
}

#[test]
fn final_connective_retains_clause_and_coordinated_predicate_attachment() {
    // Wheel of Sun and Moon's replacement consequent.
    let text = "If a card would be put into enchanted player's graveyard from anywhere, instead that card is revealed and put on the bottom of that player's library.";
    // The source PP remains outside this ticket. The attested consequent
    // provides an initial connective and coordinated passive witness.
    let consequent = text.split_once(", ").unwrap().1.trim_end_matches('.');
    assert_constituents(
        consequent,
        Category::Clause,
        &[
            (Category::PrepositionPhrase, "instead"),
            (
                Category::FinitePredicate,
                "is revealed and put on the bottom of that player's library",
            ),
        ],
    );
    // Darksteel Colossus' attested consequent, excluding its source PP.
    let final_text = "reveal Darksteel Colossus and shuffle it into its owner's library instead";
    assert_constituents(
        final_text,
        Category::Clause,
        &[
            (Category::PrepositionPhrase, "instead"),
            (
                Category::SecondaryVerbPhrase,
                "reveal Darksteel Colossus and shuffle it into its owner's library",
            ),
        ],
    );
    assert_constituents(
        final_text,
        Category::Clause,
        &[
            (Category::PrepositionPhrase, "instead"),
            (
                Category::SecondaryVerbPhrase,
                "shuffle it into its owner's library instead",
            ),
        ],
    );
}

#[test]
fn reality_twist_retains_the_nominal_complement_of_the_connective() {
    // Reality Twist's full sentence independently lacks the Produce lexeme
    // and a reduced conditional host; inspect its attested PP constituent.
    assert_constituents(
        "instead of any other type",
        Category::PrepositionPhrase,
        &[(Category::AccusativePhrase, "any other type")],
    );
}

#[test]
fn passive_relative_witness_keeps_the_overt_auxiliary_complement() {
    // Pariah's replacement sentence: "dealt" belongs to the relative's
    // overt passive complement, not to a separate nominal postmodifier.
    let text = "All damage that would be dealt to you is dealt to enchanted creature instead.";
    let values = readings(text, Category::Document);
    assert!(!values.is_empty());
    for value in values {
        value
            .visit(&mut |node| {
                assert!(
                    !matches!(node, Reading::OmittedPlain { .. }),
                    "the attested passive complement is overt: {text}"
                );
            })
            .unwrap();
    }
}

#[test]
fn attested_auxiliary_ellipsis_controls_remain_licensed() {
    for text in [
        "If you do, draw a card.",
        // Roots of Wisdom: attested negative-modal ellipsis.
        "Mill three cards, then return a land card or Elf card from your graveyard to your hand. If you can't, draw a card.",
    ] {
        let values = readings(text, Category::Document);
        assert!(!values.is_empty(), "attested ellipsis: {text}");
        for value in values {
            let mut elided = false;
            value
                .visit(&mut |node| elided |= matches!(node, Reading::OmittedPlain { .. }))
                .unwrap();
            assert!(elided, "attested omitted complement: {text}");
        }
    }
}

#[test]
fn passive_relative_twin_retains_its_two_overt_readings() {
    let text = "All damage that would be dealt to you is dealt to enchanted creature.";
    let values = readings(text, Category::Document);
    assert_eq!(values.len(), 2, "retire four malformed stranding Readings");
    let mut relatives = BTreeSet::new();
    for value in values {
        let mut complete_relative = false;
        value
            .visit(&mut |node| {
                assert!(!matches!(node, Reading::OmittedPlain { .. }));
                if let Reading::SubjectRelativeClause { .. } = node {
                    relatives.insert(node.realize(lexicon()).unwrap());
                    complete_relative = true;
                }
            })
            .unwrap();
        assert!(complete_relative);
    }
    assert_eq!(
        relatives,
        BTreeSet::from([
            "that would be dealt".to_owned(),
            "that would be dealt to you".to_owned(),
        ])
    );
    for text in [
        "If it would be, return it to your hand.",
        "If you have, draw a card.",
        "If you can, draw a card.",
        "If you do and can, draw a card.",
    ] {
        assert!(
            readings(text, Category::Document).is_empty(),
            "unlicensed stranding: {text}"
        );
    }
}
