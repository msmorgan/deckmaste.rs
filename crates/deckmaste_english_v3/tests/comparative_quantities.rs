mod common;

use common::assert_constituents;
use common::lexicon;
use common::readings;
use deckmaste_english_v3::grammar::Category;
use deckmaste_english_v3::grammar::Reading;
use deckmaste_english_v3::grammar::Word;
use deckmaste_lexical::FeatureBundle;
use deckmaste_lexical::LexicalReading;
use deckmaste_lexical::LexicalValue;
use deckmaste_lexical::Number;
use deckmaste_lexical::Numeral;
use deckmaste_lexical::SurfaceCase;
use deckmaste_lexical::WordForm;

#[test]
fn comparative_quantities_compose_with_the_existing_counted_nominal() {
    // Warcry Phoenix, Garrison Excavator, Ironhoof Ox, Eidolon of Rhetoric,
    // and Shadowborn Demon: quoted failing units, with bare-cardinal controls.
    for (text, quantity, nominal) in [
        ("three or more creatures", "three or more", "creatures"),
        ("one or more cards", "one or more", "cards"),
        ("more than one creature", "more than one", "creature"),
        ("more than one spell", "more than one", "spell"),
        (
            "fewer than six creature cards",
            "fewer than six",
            "creature cards",
        ),
    ] {
        let values = readings(text, Category::NounPhrase);
        assert_eq!(values.len(), 1, "{text}: {values:?}");
        let Reading::CountedNounPhrase {
            quantity: observed,
            head,
            ..
        } = values.first().unwrap()
        else {
            panic!("comparative quantity must determine the existing counted NP")
        };
        assert_eq!(observed.realize(lexicon()).unwrap(), quantity);
        assert_eq!(head.realize(lexicon()).unwrap(), nominal);
    }
    for text in [
        "three creatures",
        "one card",
        "one creature",
        "one spell",
        "six creature cards",
    ] {
        assert!(!readings(text, Category::NounPhrase).is_empty(), "{text}");
    }
    for (text, quantity) in [
        (
            "Whenever you attack with three or more creatures, you may pay {2}{R}.",
            "three or more",
        ),
        (
            "Whenever one or more cards leave your graveyard, create a 2/2 red and white Spirit creature token.",
            "one or more",
        ),
        (
            "This creature can't be blocked by more than one creature.",
            "more than one",
        ),
        (
            "Each player can't cast more than one spell each turn.",
            "more than one",
        ),
    ] {
        assert_constituents(
            text,
            Category::Document,
            &[(Category::QuantitativeDeterminer, quantity)],
        );
    }
    // Shadowborn Demon's existential host is independently unimplemented;
    // its complete quantified NP still witnesses this ticket's construction.
    assert_constituents(
        "fewer than six creature cards in your graveyard",
        Category::NounPhrase,
        &[(Category::QuantitativeDeterminer, "fewer than six")],
    );
}

#[test]
fn scalar_hosts_can_reuse_the_single_numeral_coordination() {
    // Depressurize: "Then if that creature's power is 0 or less, destroy it."
    // The outer copular host is owned by its sibling ticket.
    assert_eq!(
        readings("0 or less", Category::ComparativeQuantity).len(),
        1
    );
}

fn word(owner: &str) -> Word {
    Word {
        value: LexicalReading::Word(LexicalValue {
            lexeme: owner.into(),
            form: WordForm::Invariant,
            features: FeatureBundle::default(),
            variant: 0,
            capitalization: SurfaceCase::Declared,
        }),
        frame: None,
        countability: None,
    }
}

fn cardinal(value: i32) -> Reading {
    Reading::Cardinal {
        form: 0,
        head: Word {
            value: LexicalReading::Numeral {
                value,
                notation: Numeral::Cardinal,
                capitalization: SurfaceCase::Declared,
            },
            frame: None,
            countability: None,
        },
    }
}

fn coordination(value: i32, owner: &str) -> Reading {
    Reading::NumeralComparativeCoordination {
        form: 0,
        left: Box::new(Reading::CardinalQuantityConjunct {
            form: 0,
            value: Box::new(cardinal(value)),
        }),
        marker: word("vocab:Coordinator/Or"),
        right: word(owner),
    }
}

fn comparison(value: i32, owner: &str) -> Reading {
    Reading::ComparativeDeterminativePhrase {
        form: 0,
        head: word(owner),
        complement: Box::new(Reading::QuantitativePrepositionPhrase {
            category: Category::ComparativePrepositionPhrase,
            form: 0,
            head: word("vocab:Preposition/Than"),
            complement: Box::new(cardinal(value)),
        }),
    }
}

fn counted(value: Reading, owner: &str, number: Number) -> Reading {
    let mut head = word(owner);
    let LexicalReading::Word(noun) = &mut head.value else { unreachable!() };
    noun.form = if number == Number::Singular { WordForm::Singular } else { WordForm::Plural };
    noun.features.number = Some(number);
    head.countability = Some(true);
    Reading::CountedNounPhrase {
        form: 0,
        quantity: Box::new(Reading::ComparativeQuantityDeterminer {
            form: 0,
            value: Box::new(value),
        }),
        head: Box::new(Reading::Noun { form: 0, head }),
    }
}

fn assert_laws(value: &Reading, text: &str, category: Category) {
    value.admit(lexicon()).unwrap();
    assert_eq!(value.realize(lexicon()).unwrap(), text);
    let parsed = readings(text, category);
    assert!(
        parsed.contains(value),
        "independent value missing for {text:?}"
    );
    let observed = parsed.get(value).unwrap();
    let mut expected_nodes = Vec::new();
    let mut actual_nodes = Vec::new();
    value
        .visit(&mut |node| expected_nodes.push(node.clone()))
        .unwrap();
    observed
        .visit(&mut |node| actual_nodes.push(node.clone()))
        .unwrap();
    assert_eq!(actual_nodes, expected_nodes);
    let mut expected_words = Vec::new();
    let mut actual_words = Vec::new();
    value
        .visit_words(&mut |word| expected_words.push(word.clone()))
        .unwrap();
    observed
        .visit_words(&mut |word| actual_words.push(word.clone()))
        .unwrap();
    assert_eq!(actual_words, expected_words);
}

#[test]
fn independent_comparative_quantities_preserve_structure_and_lexical_identity() {
    // Constituents of Warcry Phoenix, Garrison Excavator, and Ironhoof Ox.
    for (quantity, surface, owner, number) in [
        (
            coordination(3, "vocab:ComparativeQuantifier/More"),
            "three or more",
            "lexeme:type/creature",
            Number::Plural,
        ),
        (
            coordination(1, "vocab:ComparativeQuantifier/More"),
            "one or more",
            "lexeme:CommonNoun/Card",
            Number::Plural,
        ),
        (
            comparison(1, "vocab:ComparativeQuantifier/More"),
            "more than one",
            "lexeme:type/creature",
            Number::Singular,
        ),
    ] {
        assert_laws(&quantity, surface, Category::ComparativeQuantity);
        let noun = if owner.ends_with("Card") {
            "cards"
        } else if number == Number::Singular {
            "creature"
        } else {
            "creatures"
        };
        assert_laws(
            &counted(quantity, owner, number),
            &format!("{surface} {noun}"),
            Category::NounPhrase,
        );
    }
    let pp = Reading::QuantitativePrepositionPhrase {
        category: Category::ComparativePrepositionPhrase,
        form: 0,
        head: word("vocab:Preposition/Than"),
        complement: Box::new(cardinal(1)),
    };
    assert_laws(&pp, "than one", Category::ComparativePrepositionPhrase);
    let mut leaves = Vec::new();
    pp.visit_words(&mut |word| leaves.push(word.clone()))
        .unwrap();
    let Reading::Cardinal { head, .. } = cardinal(1) else { unreachable!() };
    assert_eq!(leaves, [word("vocab:Preposition/Than"), head]);
    // Depressurize's Arabic quantity retains its numeral notation.
    let quantity = Reading::NumeralComparativeCoordination {
        form: 0,
        left: Box::new(Reading::NumericQuantityConjunct {
            form: 0,
            head: Word {
                value: LexicalReading::Numeral {
                    value: 0,
                    notation: Numeral::Arabic(false),
                    capitalization: SurfaceCase::Declared,
                },
                frame: None,
                countability: None,
            },
        }),
        marker: word("vocab:Coordinator/Or"),
        right: word("vocab:Determinative/Less"),
    };
    assert_laws(&quantity, "0 or less", Category::ComparativeQuantity);

    // A scalar coordination's numeric Conjunct does not become a Cardinal
    // measurement route in the independently constructed NP interface.
    let Reading::NumeralComparativeCoordination { left, .. } = quantity else {
        unreachable!()
    };
    let invalid = Reading::CardinalDeterminer {
        form: 0,
        value: left,
    };
    assert!(invalid.admit(lexicon()).is_err());
    assert!(invalid.realize(lexicon()).is_err());
}

#[test]
fn comparative_determiners_preserve_finite_agreement() {
    // Garrison Excavator's attested plural Subject; Ironhoof Ox's singular NP.
    assert!(
        !readings(
            "one or more cards leave your graveyard",
            Category::FiniteClause
        )
        .is_empty()
    );
    assert!(
        readings(
            "one or more cards leaves your graveyard",
            Category::FiniteClause
        )
        .is_empty()
    );
    assert!(
        readings(
            "one or more card leaves your graveyard",
            Category::FiniteClause
        )
        .is_empty()
    );
    for (value, text) in [
        (
            counted(
                coordination(1, "vocab:ComparativeQuantifier/More"),
                "lexeme:CommonNoun/Card",
                Number::Plural,
            ),
            "one or more cards",
        ),
        (
            counted(
                comparison(1, "vocab:ComparativeQuantifier/More"),
                "lexeme:type/creature",
                Number::Singular,
            ),
            "more than one creature",
        ),
    ] {
        assert_eq!(
            readings(text, Category::NounPhrase),
            std::collections::BTreeSet::from([value])
        );
    }
}

#[test]
fn comparative_quantity_selection_rejects_invalid_independent_values() {
    let invalid_coordination = Reading::NumeralComparativeCoordination {
        form: 0,
        left: Box::new(Reading::CardinalQuantityConjunct {
            form: 0,
            value: Box::new(cardinal(3)),
        }),
        marker: word("vocab:Coordinator/And"),
        right: word("vocab:ComparativeQuantifier/More"),
    };
    let wrong_conjunct = coordination(3, "vocab:Determinative/Many");
    let wrong_complement = Reading::ComparativeDeterminativePhrase {
        form: 0,
        head: word("vocab:ComparativeQuantifier/More"),
        complement: Box::new(Reading::QuantitativePrepositionPhrase {
            category: Category::ComparativePrepositionPhrase,
            form: 0,
            head: word("vocab:Preposition/To"),
            complement: Box::new(cardinal(1)),
        }),
    };
    for invalid in [
        invalid_coordination,
        wrong_conjunct,
        wrong_complement,
        counted(
            coordination(1, "vocab:ComparativeQuantifier/More"),
            "lexeme:CommonNoun/Card",
            Number::Singular,
        ),
        counted(
            comparison(1, "vocab:ComparativeQuantifier/More"),
            "lexeme:type/creature",
            Number::Plural,
        ),
    ] {
        assert!(invalid.admit(lexicon()).is_err());
        assert!(invalid.realize(lexicon()).is_err());
    }
}

#[test]
fn comparative_quantities_do_not_overlap_other_quantity_or_adjunct_routes() {
    // Attested quantity constituents from the five pinned witnesses.
    for text in [
        "three or more",
        "one or more",
        "more than one",
        "fewer than six",
        "0 or less",
    ] {
        assert_eq!(
            readings(text, Category::ComparativeQuantity).len(),
            1,
            "{text}"
        );
        assert_eq!(
            readings(text, Category::QuantitativeDeterminer).len(),
            1,
            "{text}"
        );
        assert!(readings(text, Category::Cardinal).is_empty(), "{text}");
    }
    assert!(readings("than one", Category::PrepositionPhrase).is_empty());
    for text in [
        "more than one creatures",
        "one or more card",
        "fewer than six creature card",
        "up than two creatures",
        "more to one creature",
        "three and more creatures",
        "three nor more creatures",
        "three or many creatures",
    ] {
        assert!(readings(text, Category::NounPhrase).is_empty(), "{text}");
    }
}
