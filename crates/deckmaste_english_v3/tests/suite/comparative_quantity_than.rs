use deckmaste_english_v3::grammar::Category;
use deckmaste_english_v3::grammar::Reading;
use deckmaste_english_v3::grammar::Word;
use deckmaste_lexical::Case;
use deckmaste_lexical::FeatureBundle;
use deckmaste_lexical::LexicalReading;
use deckmaste_lexical::LexicalValue;
use deckmaste_lexical::Number;
use deckmaste_lexical::Person;
use deckmaste_lexical::SurfaceCase;
use deckmaste_lexical::WordForm;

use crate::common::lexicon;
use crate::common::readings;
use crate::common::readings_on_card;

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

#[test]
fn witnesses_keep_the_determiner_and_selected_comparison_with_the_nominal_head() {
    for (card, text, phrase, owner, nominal, complement) in [
        (
            "Sunstar Expansionist",
            "When this creature enters, if an opponent controls more lands than you, create a Lander token.",
            "more lands than you",
            "vocab:ComparativeQuantifier/More",
            "lands",
            "you",
        ),
        (
            "Okina Nightwatch",
            "As long as you have more cards in hand than each opponent, this creature gets +3/+3.",
            "more cards in hand than each opponent",
            "vocab:ComparativeQuantifier/More",
            "cards in hand",
            "each opponent",
        ),
        (
            "Unified Will",
            "Counter target spell if you control more creatures than that spell's controller.",
            "more creatures than that spell's controller",
            "vocab:ComparativeQuantifier/More",
            "creatures",
            "that spell's controller",
        ),
        (
            "Even the Odds",
            "Cast this spell only if you control fewer creatures than each opponent.",
            "fewer creatures than each opponent",
            "vocab:ComparativeQuantifier/Fewer",
            "creatures",
            "each opponent",
        ),
        (
            "Survival Cache",
            "You gain 2 life. Then if you have more life than an opponent, draw a card.",
            "more life than an opponent",
            "vocab:ComparativeQuantifier/More",
            "life",
            "an opponent",
        ),
    ] {
        let values = readings_on_card(card, text, Category::Document);
        assert!(!values.is_empty(), "{card}");
        for value in values {
            let mut comparisons = Vec::new();
            value
                .visit(&mut |node| {
                    if let Reading::DiscontinuousComparativeNominal {
                        category,
                        governor,
                        head,
                        marker,
                        complement: compared,
                        ..
                    } = node
                    {
                        assert_eq!(*category, Category::NounPhrase);
                        assert_eq!(*governor, word(owner));
                        assert_eq!(head.category(), Category::Nominal);
                        assert_eq!(head.realize(lexicon()).unwrap(), nominal);
                        assert_eq!(*marker, word("vocab:Preposition/Than"));
                        assert_eq!(compared.category(), Category::AccusativePhrase);
                        assert_eq!(compared.realize(lexicon()).unwrap(), complement);
                        comparisons.push(node.realize(lexicon()).unwrap());
                    }
                    if node.category() == Category::PrepositionPhrase {
                        assert_ne!(
                            node.realize(lexicon()).unwrap(),
                            format!("than {complement}")
                        );
                    }
                })
                .unwrap();
            assert_eq!(comparisons, [phrase]);
        }
    }
    // The less-life constituent is attested on Timely Reinforcements.
    let values = readings("less life than an opponent", Category::NounPhrase);
    assert!(!values.is_empty());
    for value in values {
        let Reading::DiscontinuousComparativeNominal {
            governor,
            head,
            complement,
            ..
        } = value
        else {
            panic!("less must determine the comparative NP");
        };
        assert_eq!(governor, word("vocab:Determinative/Less"));
        assert_eq!(head.realize(lexicon()).unwrap(), "life");
        assert_eq!(complement.realize(lexicon()).unwrap(), "an opponent");
    }
}

#[test]
fn head_selection_case_and_outer_determiner_follow_the_comparative_licence() {
    for text in [
        "more land than you",
        "fewer life than you",
        "less lands than you",
        "the more lands than you",
        "a more life than you",
        "more lands than he",
        "more lands as you",
    ] {
        assert!(readings(text, Category::NounPhrase).is_empty(), "{text}");
    }
}

#[test]
fn an_independently_constructed_comparative_np_roundtrips() {
    // Sunstar Expansionist's Object, independently constructed.
    let mut land = word("lexeme:type/land");
    let LexicalReading::Word(lexical) = &mut land.value else { unreachable!() };
    lexical.form = WordForm::Plural;
    lexical.features.number = Some(Number::Plural);
    land.countability = Some(true);
    let mut you = word("vocab:ObjectPronoun/You");
    let LexicalReading::Word(lexical) = &mut you.value else { unreachable!() };
    lexical.features = FeatureBundle {
        number: Some(Number::Singular),
        person: Some(Person::Second),
        case: Some(Case::Accusative),
        ..Default::default()
    };
    let value = Reading::DiscontinuousComparativeNominal {
        category: Category::NounPhrase,
        form: 0,
        governor: word("vocab:ComparativeQuantifier/More"),
        head: Box::new(Reading::Noun {
            form: 0,
            head: land,
        }),
        marker: word("vocab:Preposition/Than"),
        complement: Box::new(Reading::CasePhrase {
            category: Category::AccusativePhrase,
            form: 0,
            head: Box::new(Reading::AccusativePronoun { form: 0, head: you }),
        }),
    };
    value.admit(lexicon()).unwrap();
    assert_eq!(value.realize(lexicon()).unwrap(), "more lands than you");
    assert_eq!(
        readings("more lands than you", Category::NounPhrase),
        [value.clone()].into()
    );
    let mut wrong_category = value;
    let Reading::DiscontinuousComparativeNominal { category, .. } = &mut wrong_category else {
        unreachable!()
    };
    *category = Category::Nominal;
    assert!(wrong_category.admit(lexicon()).is_err());
}

#[test]
fn existing_comparisons_keep_their_roles() {
    let quantities = readings("more than two lands", Category::NounPhrase);
    assert!(!quantities.is_empty());
    for value in quantities {
        let Reading::QuantifiedNounPhrase { quantity, head, .. } = value else {
            panic!("pre-head than must remain inside the quantitative Determiner");
        };
        assert_eq!(quantity.realize(lexicon()).unwrap(), "more than two");
        assert_eq!(head.realize(lexicon()).unwrap(), "lands");
    }
    for (comparative, control) in [
        ("more lands than you", "lands"),
        ("fewer creatures than you", "creatures"),
        ("more life than an opponent", "life"),
        ("less life than an opponent", "life"),
    ] {
        let expected = readings(control, Category::NominativePhrase);
        let expected = expected.first().unwrap().admit(lexicon()).unwrap();
        let values = readings(comparative, Category::NominativePhrase);
        assert!(!values.is_empty());
        for value in values {
            // Full feature equality includes the head's Agreement Number.
            assert_eq!(value.admit(lexicon()).unwrap(), expected);
        }
    }
    assert!(
        !readings(
            "a card with the same name as that card",
            Category::NounPhrase
        )
        .is_empty()
    );
    assert!(
        !readings(
            "Its power is greater than its toughness.",
            Category::Document
        )
        .is_empty()
    );
}
