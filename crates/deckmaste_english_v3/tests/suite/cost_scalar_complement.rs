use deckmaste_english_v3::grammar::Category;

use crate::common;

#[test]
fn cost_witnesses_read_with_their_existing_adjuncts() {
    for (card, text) in [
        (
            "Wizard's Lightning",
            "This spell costs {2} less to cast if you control a Wizard.",
        ),
        (
            "Ghoultree",
            "This spell costs {1} less to cast for each creature card in your graveyard.",
        ),
        (
            "Herald of the Pantheon",
            "Enchantment spells you cast cost {1} less to cast.",
        ),
        (
            "Blossoming Tortoise",
            "Activated abilities of lands you control cost {1} less to activate.",
        ),
        ("Glowrider", "Noncreature spells cost {1} more to cast."),
        ("Inquisitive Glimmer", "Unlock costs you pay cost {1} less."),
    ] {
        assert!(
            !common::readings(text, Category::Document).is_empty(),
            "{card}: {text}"
        );
    }
}

use std::collections::BTreeSet;

use deckmaste_english_v3::grammar::FrameValue;
use deckmaste_english_v3::grammar::Reading;
use deckmaste_english_v3::grammar::Word;
use deckmaste_lexical::LexicalReading;
use deckmaste_lexical::Number;
use deckmaste_lexical::Numeral;
use deckmaste_lexical::Person;
use deckmaste_lexical::SurfaceCase;
use deckmaste_lexical::WordForm;

fn word(owner: &str, form: WordForm) -> Word {
    let value = common::lexicon()
        .values()
        .find(|value| {
            value.lexeme.as_str() == owner
                && value.form == form
                && (form != WordForm::Present
                    || (value.features.number == Some(Number::Singular)
                        && value.features.person == Some(Person::Third)))
        })
        .unwrap()
        .clone();
    Word {
        value: LexicalReading::Word(value),
        frame: None,
        countability: None,
    }
}

fn symbols(amount: i32) -> Reading {
    Reading::SymbolSequence {
        category: Category::CostSymbols,
        form: 0,
        first: Box::new(Reading::NumericCostSymbol {
            form: 0,
            number: Word {
                value: LexicalReading::Numeral {
                    value: amount,
                    notation: Numeral::Arabic(false),
                    capitalization: SurfaceCase::Declared,
                },
                frame: None,
                countability: None,
            },
        }),
        rest: vec![],
    }
}

fn hollow() -> Reading {
    let mut head = word("lexeme:keyword_action/cast", WordForm::Plain);
    head.frame = Some(0);
    Reading::ToInfinitive {
        category: Category::HollowInfinitivalComplement,
        form: 0,
        marker: word("vocab:InfinitivalMarker/To", WordForm::Invariant),
        predicate: Box::new(Reading::BareObjectGap { form: 0, head }),
    }
}

fn predicate(complement: Reading, infinitival: Option<Reading>) -> Reading {
    let mut head = word("core-verb:Cost", WordForm::Present);
    head.frame = Some(0);
    Reading::SelectedPredicate {
        form: 0,
        category: Category::FinitePredicate,
        head,
        complements: vec![
            FrameValue::Argument(Box::new(complement)),
            FrameValue::Optional(
                infinitival.map(|value| Box::new(FrameValue::Argument(Box::new(value)))),
            ),
        ],
    }
}

#[test]
fn independent_complements_preserve_both_laws_and_traversal_identity() {
    // Exact predicate fragments in Inquisitive Glimmer and Ghoultree.
    let measured = Reading::MeasuredScalarLocation {
        form: 0,
        modifier: Box::new(symbols(1)),
        head: word("vocab:Determinative/Less", WordForm::Invariant),
    };
    let mut without_infinitival = predicate(measured.clone(), None);
    let Reading::SelectedPredicate { category, head, .. } = &mut without_infinitival else {
        unreachable!()
    };
    *category = Category::SecondaryVerbPhrase;
    *head = word("core-verb:Cost", WordForm::Plain);
    head.frame = Some(0);
    for (value, text, category) in [
        (
            without_infinitival,
            "cost {1} less",
            Category::SecondaryVerbPhrase,
        ),
        (
            measured.clone(),
            "{1} less",
            Category::ScalarLocationComplement,
        ),
        (
            Reading::ScalarMeasureValue {
                category: Category::ScalarLocationComplement,
                form: 0,
                value: Box::new(symbols(0)),
            },
            "{0}",
            Category::ScalarLocationComplement,
        ),
        (hollow(), "to cast", Category::HollowInfinitivalComplement),
        (
            predicate(measured, Some(hollow())),
            "costs {1} less to cast",
            Category::FinitePredicate,
        ),
    ] {
        value.admit(common::lexicon()).unwrap();
        assert_eq!(value.realize(common::lexicon()).unwrap(), text);
        let parsed = common::readings(text, category);
        let mut expected = BTreeSet::from([value.clone()]);
        if category == Category::SecondaryVerbPhrase {
            let mut participial = value.clone();
            let Reading::SelectedPredicate { head, .. } = &mut participial else {
                unreachable!()
            };
            *head = word("core-verb:Cost", WordForm::PastParticiple);
            head.frame = Some(0);
            participial.admit(common::lexicon()).unwrap();
            assert_eq!(participial.realize(common::lexicon()).unwrap(), text);
            expected.insert(participial);
        }
        assert_eq!(parsed, expected);
        let observed = parsed.get(&value).unwrap();
        let (mut nodes, mut parsed_nodes, mut words, mut parsed_words) =
            (vec![], vec![], vec![], vec![]);
        value.visit(&mut |node| nodes.push(node.clone())).unwrap();
        observed
            .visit(&mut |node| parsed_nodes.push(node.clone()))
            .unwrap();
        value
            .visit_words(&mut |word| words.push(word.clone()))
            .unwrap();
        observed
            .visit_words(&mut |word| parsed_words.push(word.clone()))
            .unwrap();
        assert_eq!(nodes, parsed_nodes);
        assert_eq!(words, parsed_words);
    }
}

#[test]
fn the_infinitival_is_hollow_and_the_measure_is_one_selected_complement() {
    for (text, measure, infinitival, gap) in [
        (
            "Enchantment spells you cast cost {1} less to cast.",
            "{1} less",
            "to cast",
            "cast",
        ),
        (
            "Noncreature spells cost {1} more to cast.",
            "{1} more",
            "to cast",
            "cast",
        ),
        (
            "Activated abilities of lands you control cost {1} less to activate.",
            "{1} less",
            "to activate",
            "activate",
        ),
    ] {
        common::assert_constituents(
            text,
            Category::Document,
            &[
                (Category::ScalarLocationComplement, measure),
                (Category::HollowInfinitivalComplement, infinitival),
            ],
        );
        for reading in common::readings(text, Category::Document) {
            let mut found = 0;
            reading
                .visit(&mut |node| {
                    if let Reading::ToInfinitive {
                        category: Category::HollowInfinitivalComplement,
                        predicate,
                        ..
                    } = node
                    {
                        assert_eq!(predicate.category(), Category::BareObjectGap);
                        assert_eq!(predicate.realize(common::lexicon()).unwrap(), gap);
                        found += 1;
                    }
                })
                .unwrap();
            assert_eq!(found, 1, "{text}");
        }
    }
}

#[test]
fn ordinary_infinitivals_and_unlicensed_measure_heads_are_rejected() {
    for (text, category) in [
        ("to cast this spell", Category::HollowInfinitivalComplement),
        ("to costs", Category::HollowInfinitivalComplement),
        ("{1} greater", Category::ScalarLocationComplement),
        ("{1} fewer", Category::ScalarLocationComplement),
        (
            "This spell costs {1} less to cast this spell.",
            Category::Document,
        ),
        ("This spell cost {1} less to casts.", Category::Document),
    ] {
        assert_eq!(common::readings(text, category), BTreeSet::new(), "{text}");
    }
    let mut wrong = hollow();
    let Reading::ToInfinitive {
        category: Category::HollowInfinitivalComplement,
        predicate,
        ..
    } = &mut wrong
    else {
        unreachable!()
    };
    let Reading::BareObjectGap { head, .. } = predicate.as_mut() else {
        unreachable!()
    };
    *head = word("core-verb:Die", WordForm::Plain);
    head.frame = Some(0);
    assert!(wrong.admit(common::lexicon()).is_err());
    assert!(wrong.realize(common::lexicon()).is_err());
    let wrong = Reading::MeasuredScalarLocation {
        form: 0,
        modifier: Box::new(symbols(1)),
        head: word("vocab:ScalarDegree/Greater", WordForm::Invariant),
    };
    assert!(wrong.admit(common::lexicon()).is_err());
    assert!(wrong.realize(common::lexicon()).is_err());
}

#[test]
fn optional_infinitival_and_mixed_symbols_have_unique_constituents() {
    // Inquisitive Glimmer, Ranar the Ever-Watchful, and Sapphire Leech.
    for (text, category, count) in [
        ("cost {1} less", Category::SecondaryVerbPhrase, 2),
        ("costs {0}", Category::FinitePredicate, 1),
        ("cost {1}{U} more to cast", Category::SecondaryVerbPhrase, 2),
    ] {
        assert_eq!(common::readings(text, category).len(), count, "{text}");
    }
    assert_eq!(
        common::readings("{1}{U} more", Category::ScalarLocationComplement).len(),
        1
    );
    assert_eq!(
        common::readings("to foretell", Category::HollowInfinitivalComplement).len(),
        1
    );
}
