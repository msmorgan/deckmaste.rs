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
fn pinned_maximum_quantities_keep_the_counted_nominal_structure() {
    // Sanguine Indulgence and Reasonable Doubt, respectively.
    for text in [
        "up to two target creature cards",
        "up to one target creature",
    ] {
        let values = readings(text, Category::NounPhrase);
        assert!(!values.is_empty(), "{text}");
        for value in values {
            let Reading::QuantifiedNounPhrase { quantity, head, .. } = value else {
                panic!("maximum quantity changed the Noun Phrase family: {value:?}")
            };
            assert_eq!(
                quantity.realize(lexicon()).unwrap(),
                if text.contains("two") { "up to two" } else { "up to one" }
            );
            assert_eq!(
                head.realize(lexicon()).unwrap(),
                if text.contains("cards") { "target creature cards" } else { "target creature" }
            );
            assert!(matches!(*head, Reading::TargetedNominal { .. }));
        }
    }
    assert!(readings("up to two", Category::Cardinal).is_empty());
    assert_eq!(
        readings("two target creature cards", Category::NounPhrase).len(),
        1
    );
    // The baseline retains both lexical "one" and Cardinal determiner analyses.
    assert_eq!(
        readings("one target creature", Category::NounPhrase).len(),
        2
    );
}

fn word(owner: &str, form: WordForm, number: Option<Number>) -> Word {
    Word {
        value: LexicalReading::Word(LexicalValue {
            lexeme: owner.into(),
            form,
            features: FeatureBundle {
                number,
                ..Default::default()
            },
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

fn maximum(value: Reading) -> Reading {
    Reading::QuantitativePrepositionPhrase {
        category: Category::QuantitativePrepositionPhrase,
        form: 0,
        head: word("vocab:Preposition/Up", WordForm::Invariant, None),
        complement: Box::new(Reading::QuantitativePrepositionPhrase {
            category: Category::CardinalPrepositionPhrase,
            form: 0,
            head: word("vocab:Preposition/To", WordForm::Invariant, None),
            complement: Box::new(value),
        }),
    }
}

fn noun(owner: &str, number: Number) -> Reading {
    let mut head = word(
        owner,
        if number == Number::Singular { WordForm::Singular } else { WordForm::Plural },
        Some(number),
    );
    head.countability = Some(true);
    Reading::Noun { form: 0, head }
}

fn targeted(head: Reading) -> Reading {
    Reading::TargetedNominal {
        form: 0,
        marker: word("vocab:TargetingMarker/Target", WordForm::Invariant, None),
        head: Box::new(head),
    }
}

fn counted(value: Reading, head: Reading, bounded: bool) -> Reading {
    Reading::QuantifiedNounPhrase {
        form: 0,
        quantity: Box::new(if bounded {
            Reading::PrepositionDeterminer {
                form: 0,
                value: Box::new(maximum(value)),
            }
        } else {
            Reading::CardinalDeterminer {
                form: 0,
                value: Box::new(value),
            }
        }),
        head: Box::new(head),
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
fn independently_constructed_maximum_determiners_preserve_number_and_lexical_owners() {
    // Constituents of Sanguine Indulgence, Reasonable Doubt, and Kazandu
    // Stomper.
    let creature = noun("lexeme:type/creature", Number::Singular);
    let Reading::Noun {
        head: creature_word,
        ..
    } = creature.clone()
    else {
        unreachable!()
    };
    let cards = Reading::NounPremodifiedNominal {
        form: 0,
        modifier: Box::new(Reading::NounPremodifier {
            form: 0,
            head: creature_word,
        }),
        head: Box::new(noun("lexeme:CommonNoun/Card", Number::Plural)),
    };
    for (count, head, text) in [
        (2, targeted(cards), "up to two target creature cards"),
        (1, targeted(creature), "up to one target creature"),
        (
            2,
            noun("lexeme:type/land", Number::Plural),
            "up to two lands",
        ),
    ] {
        let quantity = maximum(cardinal(count));
        assert_laws(
            &quantity,
            if count == 1 { "up to one" } else { "up to two" },
            Category::QuantitativePrepositionPhrase,
        );
        let mut leaves = Vec::new();
        quantity
            .visit_words(&mut |word| leaves.push(word.clone()))
            .unwrap();
        let Reading::Cardinal {
            head: count_word, ..
        } = cardinal(count)
        else {
            unreachable!()
        };
        assert_eq!(
            leaves,
            [
                word("vocab:Preposition/Up", WordForm::Invariant, None),
                word("vocab:Preposition/To", WordForm::Invariant, None),
                count_word
            ]
        );
        let bounded = counted(cardinal(count), head.clone(), true);
        assert_laws(&bounded, text, Category::NounPhrase);
        let exact = counted(cardinal(count), head, false);
        let Reading::QuantifiedNounPhrase { quantity, .. } = &exact else {
            unreachable!()
        };
        assert_eq!(quantity.local_cost(), 0);
        assert_eq!(
            bounded.total_cost().unwrap(),
            exact.total_cost().unwrap() + 2
        );
        assert_laws(
            &exact,
            text.strip_prefix("up to ").unwrap(),
            Category::NounPhrase,
        );
    }
    // Rampaging War Mammoth: "destroy up to X target artifacts".
    let variable = Reading::VariableCount {
        form: 0,
        head: word("vocab:Variable/X", WordForm::Invariant, None),
    };
    assert_laws(
        &counted(
            variable,
            targeted(noun("lexeme:type/artifact", Number::Plural)),
            true,
        ),
        "up to X target artifacts",
        Category::NounPhrase,
    );
}

#[test]
fn maximum_quantity_preserves_finite_agreement_and_targeted_genitives() {
    // Rat Out, Torgaar's quantified possessor, and Tormod's Crypt's genitive.
    assert_constituents(
        "Up to one target creature gets -1/-1 until end of turn.",
        Category::Document,
        &[(Category::NounPhrase, "Up to one target creature")],
    );
    assert_laws(
        &counted(
            cardinal(1),
            targeted(noun("lexeme:CommonNoun/Player", Number::Singular)),
            true,
        ),
        "up to one target player",
        Category::NounPhrase,
    );
    let genitive = Reading::GenitiveNounPhrase {
        form: 0,
        possessor: Box::new(Reading::TargetNounPhrase {
            form: 0,
            marker: word("vocab:TargetingMarker/Target", WordForm::Invariant, None),
            head: Box::new(noun("lexeme:CommonNoun/Player", Number::Singular)),
        }),
        marker: word("vocab:Genitive/Default", WordForm::Invariant, None),
        head: Box::new(noun("lexeme:CommonNoun/Graveyard", Number::Singular)),
    };
    assert_laws(&genitive, "target player's graveyard", Category::NounPhrase);
    assert!(
        readings(
            "Up to one target creature get -1/-1 until end of turn.",
            Category::Document
        )
        .is_empty()
    );
}

#[test]
fn maximum_determiners_reject_unlicensed_complements_and_number_mismatches() {
    for text in [
        "up to one target creatures",
        "up to two target creature",
        "up to two life",
        "to two target creatures",
        "up two target creatures",
        "up to second target creature",
        "up to +2 target creatures",
        "up to up to two target creatures",
    ] {
        assert!(readings(text, Category::NounPhrase).is_empty(), "{text}");
    }
    for invalid in [
        counted(
            cardinal(1),
            targeted(noun("lexeme:type/creature", Number::Plural)),
            true,
        ),
        counted(
            cardinal(2),
            targeted(noun("lexeme:type/creature", Number::Singular)),
            true,
        ),
        Reading::QuantitativePrepositionPhrase {
            category: Category::QuantitativePrepositionPhrase,
            form: 0,
            head: word("vocab:Preposition/From", WordForm::Invariant, None),
            complement: Box::new(Reading::QuantitativePrepositionPhrase {
                category: Category::CardinalPrepositionPhrase,
                form: 0,
                head: word("vocab:Preposition/To", WordForm::Invariant, None),
                complement: Box::new(cardinal(2)),
            }),
        },
    ] {
        assert!(invalid.admit(lexicon()).is_err());
        assert!(invalid.realize(lexicon()).is_err());
    }
}

#[test]
fn pinned_clauses_contain_the_maximum_targets() {
    assert_constituents(
        "Suspect up to one target creature.",
        Category::Document,
        &[
            (Category::NounPhrase, "up to one target creature"),
            (Category::Nominal, "target creature"),
        ],
    );
    assert_constituents(
        "Return up to two target creature cards from your graveyard to your hand.",
        Category::Document,
        &[
            (Category::QuantitativePrepositionPhrase, "up to two"),
            (Category::Nominal, "target creature cards"),
        ],
    );
}
