mod common;

use std::collections::BTreeSet;

use deckmaste_english_v3::grammar::Category;
use deckmaste_english_v3::grammar::FrameValue;
use deckmaste_english_v3::grammar::Reading;
use deckmaste_english_v3::grammar::Word;
use deckmaste_lexical::Case;
use deckmaste_lexical::FeatureBundle;
use deckmaste_lexical::Finiteness;
use deckmaste_lexical::LexicalReading;
use deckmaste_lexical::LexicalValue;
use deckmaste_lexical::Number;
use deckmaste_lexical::Person;
use deckmaste_lexical::SurfaceCase;
use deckmaste_lexical::Tense;
use deckmaste_lexical::WordForm;

type WordIdentity = (LexicalReading, Option<String>, Option<bool>);
type Structure = Vec<(Category, &'static str, Vec<WordIdentity>)>;

fn structure(reading: &Reading) -> Structure {
    let mut nodes = Vec::new();
    reading
        .visit(&mut |node| {
            let mut words = Vec::new();
            node.visit_words(&mut |word| {
                let mut value = word.value.clone();
                let frame = word.frame.map(|index| {
                    let LexicalReading::Word(value) = &word.value else {
                        panic!("a selected frame belongs to a Word");
                    };
                    format!(
                        "{:?}",
                        common::lexicon().lexemes()[&value.lexeme].properties.frames[index]
                    )
                });
                if let LexicalReading::Word(value) = &mut value {
                    value.lexeme = match value.lexeme.as_str() {
                        "core-verb:BeContracted" => "core-verb:Be".into(),
                        "core-verb:HaveContracted" => "core-verb:Have".into(),
                        _ => value.lexeme,
                    };
                }
                words.push((value, frame, word.countability));
            })
            .unwrap();
            nodes.push((node.category(), node.construction(), words));
        })
        .unwrap();
    nodes
}

fn twins(contracted: &str, full: &str, category: Category) {
    let expected = common::readings(full, category);
    assert!(!expected.is_empty(), "uncontracted witness: {full}");
    let actual = common::readings(contracted, category);
    assert_eq!(actual.len(), expected.len(), "{contracted}");
    assert_eq!(
        actual.iter().map(structure).collect::<BTreeSet<_>>(),
        expected.iter().map(structure).collect::<BTreeSet<_>>(),
        "constituent structure and lexical identities: {contracted}"
    );
}

#[test]
fn risen_reef_clitic_keeps_the_nominal_predicative_complement() {
    twins(
        "If it's a land card, you may put it onto the battlefield tapped.",
        "If it is a land card, you may put it onto the battlefield tapped.",
        Category::Document,
    );
}

#[test]
fn shackle_slinger_clitic_keeps_the_adjectival_predicative_complement() {
    twins(
        "If it's tapped, put a stun counter on it.",
        "If it is tapped, put a stun counter on it.",
        Category::Document,
    );
}

#[test]
fn mirrorpool_clitic_keeps_the_relative_clause_subject_gap() {
    twins(
        "{4}{C}, {T}, Sacrifice this land: Create a token that's a copy of target creature you control.",
        "{4}{C}, {T}, Sacrifice this land: Create a token that is a copy of target creature you control.",
        Category::Document,
    );
}

#[test]
fn bounty_agent_clitic_keeps_the_relative_clause_complement_coordination() {
    twins(
        "{T}, Sacrifice this creature: Destroy target legendary permanent that's an artifact, creature, or enchantment.",
        "{T}, Sacrifice this creature: Destroy target legendary permanent that is an artifact, creature, or enchantment.",
        Category::Document,
    );
}

#[test]
fn goblin_cohort_clitic_keeps_the_perfect_complement() {
    twins(
        "This creature can't attack unless you've cast a creature spell this turn.",
        "This creature can't attack unless you have cast a creature spell this turn.",
        Category::Document,
    );
}

#[test]
fn panglacial_wurm_clitic_keeps_the_progressive_complement() {
    twins(
        "While you're searching your library, you may cast this card from your library.",
        "While you are searching your library, you may cast this card from your library.",
        Category::Document,
    );
}

#[test]
fn myth_unbound_clitic_has_keeps_the_perfect_passive_complement() {
    // Myth Unbound, Opal Palace and Study Hall: "... times it's been cast ...".
    twins("it's been cast", "it has been cast", Category::FiniteClause);
}

fn word(owner: &str, form: WordForm, features: FeatureBundle, frame: Option<usize>) -> Word {
    Word {
        value: LexicalReading::Word(LexicalValue {
            lexeme: owner.into(),
            form,
            features,
            variant: 0,
            capitalization: SurfaceCase::Declared,
        }),
        frame,
        countability: None,
    }
}

#[test]
fn independently_constructed_clitic_copula_and_passive_roundtrip() {
    // Shackle Slinger: the finite Clause inside "If it's tapped, ...".
    let subject = Reading::CasePhrase {
        form: 0,
        category: Category::NominativePhrase,
        head: Box::new(Reading::NominativePronoun {
            form: 0,
            head: word(
                "vocab:SubjectPronoun/It",
                WordForm::Invariant,
                FeatureBundle {
                    number: Some(Number::Singular),
                    person: Some(Person::Third),
                    case: Some(Case::Nominative),
                    ..Default::default()
                },
                None,
            ),
        }),
    };
    let auxiliary = |frame| {
        word(
            "core-verb:BeContracted",
            WordForm::Present,
            FeatureBundle {
                number: Some(Number::Singular),
                person: Some(Person::Third),
                tense: Some(Tense::Present),
                finiteness: Some(Finiteness::Finite),
                ..Default::default()
            },
            Some(frame),
        )
    };
    let copular = Reading::SelectedPredicate {
        form: 0,
        category: Category::FinitePredicate,
        head: auxiliary(0),
        complements: vec![FrameValue::Argument(Box::new(
            Reading::AdjectivalComplement {
                form: 0,
                phrase: Box::new(Reading::Adjective {
                    form: 0,
                    head: word(
                        "lexeme:keyword_action/tap/adjective",
                        WordForm::Invariant,
                        FeatureBundle::default(),
                        None,
                    ),
                }),
            },
        ))],
    };
    let passive = Reading::ParticipialAuxiliaryPredicate {
        form: 0,
        category: Category::FinitePredicate,
        head: auxiliary(2),
        complement: Box::new(Reading::OvertComplement {
            form: 0,
            category: Category::ParticipialComplement,
            predicate: Box::new(Reading::PassiveComplement {
                form: 0,
                head: Box::new(Reading::PassivePredicate {
                    form: 0,
                    head: word(
                        "lexeme:keyword_action/tap",
                        WordForm::PastParticiple,
                        FeatureBundle {
                            finiteness: Some(Finiteness::Nonfinite),
                            ..Default::default()
                        },
                        Some(0),
                    ),
                }),
            }),
        }),
    };
    let expected: BTreeSet<_> = [copular, passive]
        .into_iter()
        .map(|predicate| Reading::FiniteClause {
            form: 1,
            subject: Box::new(subject.clone()),
            predicate: Box::new(predicate),
        })
        .collect();
    let actual = common::readings("it's tapped", Category::FiniteClause);
    assert_eq!(actual, expected);
    for value in &expected {
        value.admit(common::lexicon()).unwrap();
        assert_eq!(value.realize(common::lexicon()).unwrap(), "it's tapped");
        let parsed = actual.get(value).unwrap();
        let mut expected_nodes = Vec::new();
        value
            .visit(&mut |node| expected_nodes.push(node.clone()))
            .unwrap();
        let mut actual_nodes = Vec::new();
        parsed
            .visit(&mut |node| actual_nodes.push(node.clone()))
            .unwrap();
        assert_eq!(actual_nodes, expected_nodes);
        let mut expected_words = Vec::new();
        value
            .visit_words(&mut |word| expected_words.push(word.clone()))
            .unwrap();
        let mut actual_words = Vec::new();
        parsed
            .visit_words(&mut |word| actual_words.push(word.clone()))
            .unwrap();
        assert_eq!(actual_words, expected_words);
        let mut spaced = value.clone();
        let Reading::FiniteClause { form, .. } = &mut spaced else {
            panic!("the witness is a finite Clause");
        };
        *form = 0;
        assert!(spaced.admit(common::lexicon()).is_err());
    }
    for mut joined in common::readings("it is tapped", Category::FiniteClause) {
        let Reading::FiniteClause { form, .. } = &mut joined else {
            panic!("the full twin is a finite Clause");
        };
        *form = 1;
        assert!(joined.admit(common::lexicon()).is_err());
    }
}

#[test]
fn clitic_auxiliaries_require_overt_complements_and_licensed_hosts() {
    for (text, category) in [
        ("it's", Category::FiniteClause),
        ("you've", Category::FiniteClause),
        ("that's", Category::SubjectRelativeClause),
        ("'re", Category::FinitePredicate),
        ("'ve", Category::FinitePredicate),
        ("you've a creature", Category::FiniteClause),
        // Leyline of Singularity with an unlicensed clitic host.
        (
            "all nonland permanents're legendary",
            Category::FiniteClause,
        ),
        ("you and they're tapped", Category::FiniteClause),
    ] {
        assert!(common::readings(text, category).is_empty(), "{text}");
    }
}

#[test]
fn clitic_be_retains_all_full_be_frames_and_have_retains_the_perfect_use() {
    let entries = common::lexicon().lexemes();
    let frames = |owner: &str| {
        entries[owner]
            .properties
            .frames
            .iter()
            .map(|frame| format!("{frame:?}"))
            .collect::<BTreeSet<_>>()
    };
    assert_eq!(frames("core-verb:BeContracted"), frames("core-verb:Be"));
    let perfect: BTreeSet<_> = entries["core-verb:Have"]
        .properties
        .frames
        .iter()
        .filter(|frame| frame.kind == "Auxiliary")
        .map(|frame| format!("{frame:?}"))
        .collect();
    assert_eq!(frames("core-verb:HaveContracted"), perfect);
    assert_eq!(
        entries["core-verb:HaveContracted"].properties.features["PredicateFrameUse"],
        "No"
    );
}
