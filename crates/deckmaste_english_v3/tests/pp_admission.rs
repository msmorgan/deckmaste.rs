mod common;

use std::collections::BTreeSet;

use common::lexicon;
use common::readings;
use deckmaste_english_v3::grammar::Category;
use deckmaste_english_v3::grammar::Reading;
use deckmaste_english_v3::grammar::Word;
use deckmaste_lexical::FeatureBundle;
use deckmaste_lexical::LexicalReading;
use deckmaste_lexical::LexicalValue;
use deckmaste_lexical::Number;
use deckmaste_lexical::SurfaceCase;
use deckmaste_lexical::WordForm;

fn word(id: &str, form: WordForm, number: Option<Number>) -> Word {
    Word {
        value: LexicalReading::Word(LexicalValue {
            lexeme: id.into(),
            form,
            features: FeatureBundle {
                number,
                ..FeatureBundle::default()
            },
            variant: 0,
            capitalization: SurfaceCase::Declared,
        }),
        frame: None,
        countability: None,
    }
}

#[test]
fn nominal_complement_pp_cannot_become_a_free_clause_or_verb_adjunct() {
    let mut creature = word(
        "lexeme:type/creature",
        WordForm::Plural,
        Some(Number::Plural),
    );
    creature.countability = Some(true);
    let nominal = Reading::Noun {
        form: 0,
        head: creature,
    };
    let pp = Reading::PrepositionPhrase {
        form: 0,
        head: word("vocab:Preposition/Of", WordForm::Invariant, None),
        complement: Box::new(Reading::CasePhrase {
            category: Category::AccusativePhrase,
            form: 0,
            head: Box::new(Reading::BarePlural {
                form: 0,
                head: Box::new(nominal.clone()),
            }),
        }),
    };
    pp.admit(lexicon()).unwrap();
    assert_eq!(
        readings("of creatures", Category::PrepositionPhrase),
        BTreeSet::from([pp.clone()])
    );
    let clause = readings("draw cards", Category::Clause)
        .into_iter()
        .next()
        .unwrap();
    let finite = readings("draws cards", Category::FinitePredicate)
        .into_iter()
        .next()
        .unwrap();
    let secondary = readings("draw cards", Category::SecondaryVerbPhrase)
        .into_iter()
        .next()
        .unwrap();
    for invalid in [
        Reading::InitialPreposition {
            form: 0,
            dependent: Box::new(pp.clone()),
            clause: Box::new(clause.clone()),
        },
        Reading::ClausalPreposition {
            form: 0,
            clause: Box::new(clause),
            dependent: Box::new(pp.clone()),
        },
        Reading::PrepositionPredicate {
            category: Category::FinitePredicate,
            form: 0,
            head: Box::new(finite),
            modifier: Box::new(pp.clone()),
        },
        Reading::PrepositionPredicate {
            category: Category::SecondaryVerbPhrase,
            form: 0,
            head: Box::new(secondary),
            modifier: Box::new(pp.clone()),
        },
    ] {
        assert!(invalid.admit(lexicon()).is_err());
    }
    let value = Reading::PostmodifiedNominal {
        form: 0,
        head: Box::new(nominal),
        modifier: Box::new(pp),
    };
    value.admit(lexicon()).unwrap();
    assert!(readings("creatures of creatures", Category::Nominal).contains(&value));
    assert_eq!(value.realize(lexicon()).unwrap(), "creatures of creatures");
}

#[test]
fn pp_adjuncts_and_selected_locatives_keep_their_readings() {
    for (text, category) in [
        ("Draw cards from them.", Category::Document),
        ("Until you draw cards, draw cards.", Category::Document),
        (
            "Your maximum hand size is increased by two.",
            Category::Document,
        ),
        ("You gain X plus 3 life.", Category::Document),
        (
            "this creature remains on the battlefield",
            Category::FiniteClause,
        ),
        ("number of creatures", Category::Nominal),
    ] {
        assert!(
            !readings(text, category).is_empty(),
            "no Reading for {text:?}"
        );
    }
    for reading in readings(
        "Your maximum hand size is increased by two.",
        Category::Document,
    ) {
        let mut extents = vec![];
        reading
            .visit(&mut |node| {
                if let Reading::SelectedExtentPassive { complement, .. } = node {
                    extents.push((
                        node.realize(lexicon()).unwrap(),
                        complement.category(),
                        complement.realize(lexicon()).unwrap(),
                    ));
                }
            })
            .unwrap();
        assert_eq!(
            extents,
            vec![(
                "increased by two".into(),
                Category::ScalarExtentComplement,
                "by two".into()
            )]
        );
    }
    assert!(readings("Creatures attack by 2 plus 2.", Category::Document).is_empty());
}

#[test]
fn distribution_preserves_nominal_ambiguity_without_detached_of_adjuncts() {
    // Blessings of Nature and Grove's Bounty, with reminder text stripped.
    for text in [
        "Distribute four +1/+1 counters among any number of target creatures.\nMiracle {G}",
        "Distribute X +1/+1 counters among any number of target creatures you control.",
    ] {
        let values = readings(text, Category::Document);
        assert!(!values.is_empty());
        for value in values {
            value
                .visit(&mut |node| {
                    let pp = match node {
                        Reading::InitialPreposition { dependent, .. }
                        | Reading::ClausalPreposition { dependent, .. } => Some(dependent),
                        Reading::PrepositionPredicate {
                            category: Category::FinitePredicate,
                            form: 0,
                            modifier,
                            ..
                        }
                        | Reading::PrepositionPredicate {
                            category: Category::SecondaryVerbPhrase,
                            form: 0,
                            modifier,
                            ..
                        } => Some(modifier),
                        _ => None,
                    };
                    if let Some(pp) = pp {
                        let mut leaves = Vec::new();
                        pp.visit_words(&mut |word| leaves.push(word.value.clone()))
                            .unwrap();
                        let LexicalReading::Word(head) = &leaves[0] else {
                            panic!("expected a preposition word");
                        };
                        assert_ne!(head.lexeme, "vocab:Preposition/Of");
                    }
                })
                .unwrap();
        }
    }
}
