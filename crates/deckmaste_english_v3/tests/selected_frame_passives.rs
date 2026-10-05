use std::collections::BTreeSet;
use std::path::Path;
use std::sync::LazyLock;

use deckmaste_english_v3::grammar::Category;
use deckmaste_english_v3::grammar::Grammar;
use deckmaste_english_v3::grammar::Reading;
use deckmaste_english_v3::grammar::Word;
use deckmaste_english_v3::parse;
use deckmaste_lexical::FeatureBundle;
use deckmaste_lexical::LexicalReading;
use deckmaste_lexical::LexicalValue;
use deckmaste_lexical::Lexicon;
use deckmaste_lexical::Number;
use deckmaste_lexical::SurfaceCase;
use deckmaste_lexical::WordForm;

static LEXICON: LazyLock<Lexicon> = LazyLock::new(|| {
    let root = Path::new(env!("CARGO_MANIFEST_DIR")).join("../..");
    Lexicon::new(
        deckmaste_lexical_source::load_workspace(&root)
            .unwrap()
            .lexemes,
    )
    .unwrap()
});

fn invariant(owner: &str, capitalization: SurfaceCase) -> Word {
    Word {
        value: LexicalReading::Word(LexicalValue {
            lexeme: owner.into(),
            form: WordForm::Invariant,
            features: FeatureBundle::default(),
            variant: 0,
            capitalization,
        }),
        frame: None,
        countability: None,
    }
}

fn noun(owner: &str) -> Reading {
    Reading::Noun {
        form: 0,
        head: Word {
            value: LexicalReading::Word(LexicalValue {
                lexeme: owner.into(),
                form: WordForm::Singular,
                features: FeatureBundle {
                    number: Some(Number::Singular),
                    ..Default::default()
                },
                variant: 0,
                capitalization: SurfaceCase::Declared,
            }),
            frame: None,
            countability: Some(true),
        },
    }
}

fn readings(text: &str, category: Category) -> BTreeSet<Reading> {
    let grammar = Grammar::default();
    let analyzed = LEXICON.analyze(text);
    let forest = parse(&grammar, &LEXICON, &analyzed, &category).unwrap();
    grammar
        .readings(&forest)
        .map(|value| {
            let value = value.unwrap();
            assert_eq!(value.realize(&LEXICON).unwrap(), text);
            value
        })
        .collect()
}

fn exact(text: &str, category: Category, expected: Reading) {
    expected
        .admit(&LEXICON)
        .unwrap_or_else(|error| panic!("{text}: {error:?}"));
    assert_eq!(expected.realize(&LEXICON).unwrap(), text);
    let actual = readings(text, category);
    assert_eq!(actual, BTreeSet::from([expected.clone()]));
    let mut expected_words = Vec::new();
    expected
        .visit_words(&mut |word| expected_words.push(word.clone()))
        .unwrap();
    let mut actual_words = Vec::new();
    actual
        .first()
        .unwrap()
        .visit_words(&mut |word| actual_words.push(word.clone()))
        .unwrap();
    assert_eq!(actual_words, expected_words);
}

fn acc(head: Reading) -> Reading {
    Reading::CasePhrase {
        category: Category::AccusativePhrase,
        form: 0,
        head: Box::new(head),
    }
}
fn verb(owner: &str, form: WordForm, frame: usize) -> Word {
    let mut head = invariant(owner, SurfaceCase::Declared);
    head.frame = Some(frame);
    if let LexicalReading::Word(value) = &mut head.value {
        value.form = form;
        if matches!(form, WordForm::PastParticiple | WordForm::GerundParticiple) {
            value.features.finiteness = Some(deckmaste_lexical::Finiteness::Nonfinite);
        }
    }
    head
}
fn manner() -> Reading {
    Reading::NominalAdjunctPhrase {
        form: 0,
        determiner: invariant("vocab:SingularDemonstrative/This", SurfaceCase::Declared),
        head: Box::new(noun("lexeme:CommonNoun/Way")),
    }
}
#[test]
fn authentic_selected_object_and_passive_gap_preserve_frame_and_marker() {
    // Boldwyr Heavyweights: "...put it onto the battlefield."
    let mut it = invariant("vocab:ObjectPronoun/It", SurfaceCase::Declared);
    if let LexicalReading::Word(value) = &mut it.value {
        value.features = FeatureBundle {
            number: Some(Number::Singular),
            person: Some(deckmaste_lexical::Person::Third),
            case: Some(deckmaste_lexical::Case::Accusative),
            ..Default::default()
        };
    }
    let plain = Reading::SelectedObjectPrepositionPredicate {
        category: Category::SecondaryVerbPhrase,
        form: 0,
        head: verb("core-verb:Put", WordForm::Plain, 6),
        object: Box::new(acc(Reading::AccusativePronoun { form: 0, head: it })),
        marker: invariant("vocab:Preposition/Onto", SurfaceCase::Declared),
        complement: Box::new(acc(Reading::DeterminedNounPhrase {
            form: 0,
            determiner: invariant("vocab:DefiniteMarker/The", SurfaceCase::Declared),
            head: Box::new(noun("lexeme:CommonNoun/Battlefield")),
        })),
    };
    let mut participial = plain.clone();
    let Reading::SelectedObjectPrepositionPredicate { head, .. } = &mut participial else {
        unreachable!()
    };
    *head = verb("core-verb:Put", WordForm::PastParticiple, 6);
    // Put is syncretic: the isolated VP also has an ordinary past-participial reading.
    for value in [&plain, &participial] {
        value.admit(&LEXICON).unwrap();
        assert_eq!(
            value.realize(&LEXICON).unwrap(),
            "put it onto the battlefield"
        );
    }
    assert_eq!(
        readings("put it onto the battlefield", Category::SecondaryVerbPhrase),
        BTreeSet::from([plain, participial])
    );
    let mut article = invariant("vocab:Article/Indefinite", SurfaceCase::Declared);
    if let LexicalReading::Word(value) = &mut article.value {
        value.features.number = Some(Number::Singular);
    }
    // Planar Void: "Whenever another card is put into a graveyard from anywhere, exile that card."
    let expected = Reading::SelectedGapPrepositionPredicate {
        category: Category::SecondaryVerbPhrase,
        form: 0,
        head: verb("core-verb:Put", WordForm::PastParticiple, 3),
        marker: invariant("vocab:Preposition/Into", SurfaceCase::Declared),
        complement: Box::new(acc(Reading::IndefiniteNounPhrase {
            form: 0,
            determiner: article,
            head: Box::new(noun("lexeme:CommonNoun/Graveyard")),
        })),
    };
    exact(
        "put into a graveyard",
        Category::SecondaryVerbPhrase,
        expected.clone(),
    );
    let Reading::SelectedGapPrepositionPredicate {
        head, complement, ..
    } = expected
    else {
        unreachable!()
    };
    let marker = invariant("vocab:Preposition/To", SurfaceCase::Declared);
    assert!(
        Reading::SelectedGapPrepositionPredicate {
            category: Category::SecondaryVerbPhrase,
            form: 0,
            head,
            marker,
            complement
        }
        .admit(&LEXICON)
        .is_err()
    );
}
#[test]
fn authentic_retained_object_and_reduced_passives_keep_manner_attachment() {
    // Aggravate: "Each creature dealt damage this way attacks this turn if able."
    let mut damage = noun("lexeme:CommonNoun/Damage");
    if let Reading::Noun { head, .. } = &mut damage {
        head.countability = Some(false);
    }
    let passive = Reading::RetainedObjectPassive {
        form: 0,
        head: verb("core-verb:Deal", WordForm::PastParticiple, 6),
        object: Box::new(acc(Reading::BareMass {
            form: 0,
            head: Box::new(damage),
        })),
    };
    exact(
        "creature dealt damage this way",
        Category::Nominal,
        Reading::ParticipialPostmodifiedNominal {
            form: 0,
            head: Box::new(noun("lexeme:type/creature")),
            modifier: Box::new(Reading::NominalAdjunctPredicate {
                category: Category::SecondaryVerbPhrase,
                form: 0,
                head: Box::new(passive.clone()),
                modifier: Box::new(manner()),
            }),
        },
    );
    let Reading::RetainedObjectPassive { object, .. } = passive else {
        unreachable!()
    };
    assert!(
        Reading::RetainedObjectPassive {
            form: 0,
            head: verb("lexeme:keyword_action/discard", WordForm::PastParticiple, 0),
            object
        }
        .admit(&LEXICON)
        .is_err()
    );
    // Syphon Mind: "You draw a card for each card discarded this way."
    exact(
        "each card discarded this way",
        Category::NounPhrase,
        Reading::DeterminedNounPhrase {
            form: 0,
            determiner: invariant("vocab:FloatedQuantifier/Each", SurfaceCase::Declared),
            head: Box::new(Reading::ParticipialPostmodifiedNominal {
                form: 0,
                head: Box::new(noun("lexeme:CommonNoun/Card")),
                modifier: Box::new(Reading::NominalAdjunctPredicate {
                    category: Category::SecondaryVerbPhrase,
                    form: 0,
                    head: Box::new(Reading::PassivePredicate {
                        form: 0,
                        head: verb("lexeme:keyword_action/discard", WordForm::PastParticiple, 0),
                    }),
                    modifier: Box::new(manner()),
                }),
            }),
        },
    );
    // The generic noun modifier now permits gerund-participial clauses.
    // Preserve the original passive-only exclusion: attacking is not a bare passive.
    let invalid = Reading::PassiveComplement {
        form: 0,
        head: Box::new(Reading::IntransitivePredicate {
            category: Category::SecondaryVerbPhrase,
            form: 0,
            head: verb("core-verb:Attack", WordForm::GerundParticiple, 0),
        }),
    };
    assert!(invalid.admit(&LEXICON).is_err());
}
