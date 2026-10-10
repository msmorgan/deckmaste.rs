use deckmaste_english_v3::grammar::Category;
use deckmaste_english_v3::grammar::Reading;
use deckmaste_english_v3::grammar::Word;
use deckmaste_lexical::FeatureBundle;
use deckmaste_lexical::LexicalReading;
use deckmaste_lexical::LexicalValue;
use deckmaste_lexical::Number;
use deckmaste_lexical::SurfaceCase;
use deckmaste_lexical::WordForm;

use crate::common::lexicon;
use crate::common::readings;
use crate::common::readings_on_card;

#[test]
fn witnesses_keep_the_complement_inside_the_premodified_nominal() {
    for (text, nominal, head, complement) in [
        (
            "Destroy target land and all other lands with the same name as that land.",
            "same name as that land",
            "name",
            "that land",
        ),
        (
            "Exile target creature and all other creatures with the same name as that creature.",
            "same name as that creature",
            "name",
            "that creature",
        ),
        (
            "Return target nonland permanent and all other permanents with the same name as that permanent to their owners' hands.",
            "same name as that permanent",
            "name",
            "that permanent",
        ),
        (
            "Your opponents can't cast spells with the same name as a card exiled with Circu.",
            "same name as a card exiled with Circu",
            "name",
            "a card exiled with Circu",
        ),
        (
            "Search its owner's graveyard, hand, and library for all cards with the same name as that card and exile them.",
            "same name as that card",
            "name",
            "that card",
        ),
    ] {
        let values = if complement == "a card exiled with Circu" {
            readings_on_card("Circu, Dimir Lobotomist", text, Category::Document)
        } else {
            readings(text, Category::Document)
        };
        assert!(!values.is_empty(), "no Reading for {text}");
        for value in values {
            let mut comparisons = 0;
            value
                .visit(&mut |node| {
                    if let Reading::DiscontinuousComparativeNominal {
                        governor,
                        head: nominal_head,
                        marker,
                        complement: compared,
                        ..
                    } = node
                    {
                        let observed_nominal = node.realize(lexicon()).unwrap();
                        let observed_complement = compared.realize(lexicon()).unwrap();
                        if complement == "a card exiled with Circu" {
                            // Circu's passive Postmodifier and source PP can
                            // attach inside or outside the comparison NP.
                            assert!(
                                [
                                    ("same name as a card exiled with Circu", "a card exiled with Circu"),
                                    ("same name as a card exiled", "a card exiled"),
                                    ("same name as a card", "a card"),
                                ].contains(&(observed_nominal.as_str(), observed_complement.as_str())),
                                "unexpected Circu comparison: {observed_nominal:?}, {observed_complement:?}",
                            );
                        } else {
                            assert_eq!(observed_nominal, nominal);
                            assert_eq!(observed_complement, complement);
                        }
                        assert_eq!(nominal_head.realize(lexicon()).unwrap(), head);
                        assert_eq!(compared.category(), Category::AccusativePhrase);
                        let mut expected = word("vocab:AttributiveAdjective/Same");
                        expected.frame = Some(1);
                        assert_eq!(**governor, Reading::DiscontinuousComparativeGovernorHead { form: 0, head: expected });
                        assert_eq!(*marker, word("vocab:Preposition/As"));
                        comparisons += 1;
                    }
                    if node.category() == Category::PrepositionPhrase {
                        let surface = node.realize(lexicon()).unwrap();
                        assert_ne!(surface, format!("as {complement}"));
                        if complement == "a card exiled with Circu" {
                            assert_ne!(surface, "as a card exiled");
                            assert_ne!(surface, "as a card");
                        }
                    }
                })
                .unwrap();
            assert_eq!(comparisons, 1, "wrong comparison constituency for {text}");
        }
    }
}

fn word(id: &str) -> Word {
    Word {
        value: LexicalReading::Word(LexicalValue {
            lexeme: id.into(),
            form: WordForm::Invariant,
            features: FeatureBundle::default(),
            variant: 0,
            capitalization: SurfaceCase::Declared,
        }),
        frame: None,
        countability: None,
    }
}

fn noun(id: &str) -> Word {
    let mut value = word(id);
    let LexicalReading::Word(lexical) = &mut value.value else { unreachable!() };
    lexical.form = WordForm::Singular;
    lexical.features.number = Some(Number::Singular);
    value.countability = Some(true);
    value
}

#[test]
fn an_independent_comparative_nominal_roundtrips_with_its_selected_complement() {
    // Wake of Destruction's Nominal, constructed independently of parsing.
    let mut governor = word("vocab:AttributiveAdjective/Same");
    governor.frame = Some(1);
    let head = noun("lexeme:CommonNoun/Name");
    let marker = word("vocab:Preposition/As");
    let that = word("vocab:SingularDemonstrative/That");
    let land = noun("lexeme:type/land");
    let value = Reading::DiscontinuousComparativeNominal {
        category: Category::Nominal,
        form: 0,
        governor: Box::new(Reading::DiscontinuousComparativeGovernorHead {
            form: 0,
            head: governor.clone(),
        }),
        head: Box::new(Reading::Noun {
            form: 0,
            head: head.clone(),
        }),
        marker: marker.clone(),
        complement: Box::new(Reading::CasePhrase {
            category: Category::AccusativePhrase,
            form: 0,
            head: Box::new(Reading::DeterminedNounPhrase {
                form: 0,
                determiner: that.clone(),
                head: Box::new(Reading::Noun {
                    form: 0,
                    head: land.clone(),
                }),
            }),
        }),
    };
    value.admit(lexicon()).unwrap();
    let text = "same name as that land";
    assert_eq!(value.realize(lexicon()).unwrap(), text);
    assert_eq!(
        readings(text, Category::Nominal),
        std::collections::BTreeSet::from([value.clone()])
    );
    let mut leaves = Vec::new();
    value
        .visit_words(&mut |word| leaves.push(word.clone()))
        .unwrap();
    assert_eq!(leaves, [governor, head, marker, that, land]);
    let mut wrong_marker = value.clone();
    let Reading::DiscontinuousComparativeNominal { marker, .. } = &mut wrong_marker else {
        unreachable!()
    };
    *marker = word("vocab:Preposition/To");
    assert!(wrong_marker.admit(lexicon()).is_err());
    let mut wrong_governor = value;
    let Reading::DiscontinuousComparativeNominal { governor, .. } = &mut wrong_governor else {
        unreachable!()
    };
    let Reading::DiscontinuousComparativeGovernorHead { head, .. } = governor.as_mut() else {
        unreachable!()
    };
    head.frame = Some(0);
    assert!(wrong_governor.admit(lexicon()).is_err());
}

#[test]
fn the_anaphoric_same_without_as_keeps_its_readings() {
    // Sphinx of the Chimes: the comparative term is left understood.
    let text = "Discard two nonland cards with the same name: Draw four cards.";
    let values = readings(text, Category::Document);
    assert!(!values.is_empty());
    for value in values {
        let mut governors = Vec::new();
        value
            .visit_words(&mut |word| {
                if let LexicalReading::Word(lexical) = &word.value
                    && lexical.lexeme.as_str() == "vocab:AttributiveAdjective/Same"
                {
                    governors.push(word.frame);
                }
            })
            .unwrap();
        assert_eq!(governors, [Some(0)]);
    }
}
