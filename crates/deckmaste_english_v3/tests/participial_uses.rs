use std::collections::BTreeSet;
use std::path::Path;
use std::sync::LazyLock;

use deckmaste_english_v3::grammar::Category;
use deckmaste_english_v3::grammar::Grammar;
use deckmaste_english_v3::grammar::Reading;
use deckmaste_english_v3::grammar::Word;
use deckmaste_english_v3::parse;
use deckmaste_lexical::FeatureBundle;
use deckmaste_lexical::Finiteness;
use deckmaste_lexical::LexicalReading;
use deckmaste_lexical::LexicalValue;
use deckmaste_lexical::Lexicon;
use deckmaste_lexical::Number;
use deckmaste_lexical::Person;
use deckmaste_lexical::SurfaceCase;
use deckmaste_lexical::Tense;
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
fn noun(owner: &str) -> Reading {
    let mut head = word(owner);
    head.countability = Some(true);
    if let LexicalReading::Word(value) = &mut head.value {
        value.form = WordForm::Singular;
        value.features.number = Some(Number::Singular);
    }
    Reading::Noun { form: 0, head }
}
fn verb(owner: &str, form: WordForm, frame: usize) -> Word {
    let mut head = word(owner);
    head.frame = Some(frame);
    if let LexicalReading::Word(value) = &mut head.value {
        value.form = form;
        value.features = if form == WordForm::GerundParticiple {
            FeatureBundle {
                finiteness: Some(Finiteness::Nonfinite),
                ..Default::default()
            }
        } else if form == WordForm::Plain {
            FeatureBundle::default()
        } else {
            FeatureBundle {
                number: Some(Number::Singular),
                person: Some(Person::Third),
                tense: Some(Tense::Present),
                finiteness: Some(Finiteness::Finite),
                ..Default::default()
            }
        };
    }
    head
}
fn exact(text: &str, category: Category, expected: Reading) {
    expected.admit(&LEXICON).unwrap();
    assert_eq!(expected.realize(&LEXICON).unwrap(), text);
    let grammar = Grammar::default();
    let analyzed = LEXICON.analyze(text);
    let forest = parse(&grammar, &LEXICON, &analyzed, &category).unwrap();
    let actual: BTreeSet<_> = grammar
        .readings(&forest)
        .map(|value| value.unwrap())
        .collect();
    assert_eq!(actual, BTreeSet::from([expected.clone()]));
    let mut expected_words = vec![];
    expected
        .visit_words(&mut |word| expected_words.push(word.clone()))
        .unwrap();
    let mut actual_words = vec![];
    actual
        .first()
        .unwrap()
        .visit_words(&mut |word| actual_words.push(word.clone()))
        .unwrap();
    assert_eq!(actual_words, expected_words);
}
#[test]
fn authentic_attacking_and_blocking_modifiers_retain_verbal_identity() {
    // Armed Response and Silent Assassin, respectively.
    for (owner, text) in [
        ("core-verb:Attack", "target attacking creature"),
        ("core-verb:Block", "target blocking creature"),
    ] {
        exact(
            text,
            Category::NounPhrase,
            Reading::TargetNounPhrase {
                form: 0,
                marker: word("vocab:TargetingMarker/Target"),
                head: Box::new(Reading::ParticipialPremodifier {
                    form: 0,
                    modifier: Box::new(Reading::VerbalPremodifier {
                        form: 0,
                        head: verb(owner, WordForm::GerundParticiple, 0),
                    }),
                    head: Box::new(noun("lexeme:type/creature")),
                }),
            },
        );
    }
}
#[test]
fn authentic_tapped_and_untapped_modifiers_retain_adjective_identity() {
    // Assassinate/Royal Assassin; Traitor's Roar, respectively.
    for (owner, text) in [
        ("tap", "target tapped creature"),
        ("untap", "target untapped creature"),
    ] {
        exact(
            text,
            Category::NounPhrase,
            Reading::TargetNounPhrase {
                form: 0,
                marker: word("vocab:TargetingMarker/Target"),
                head: Box::new(Reading::PremodifiedNominal {
                    form: 0,
                    modifier: Box::new(Reading::Adjective {
                        form: 0,
                        head: word(&format!("lexeme:keyword_action/{owner}/adjective")),
                    }),
                    head: Box::new(noun("lexeme:type/creature")),
                }),
            },
        );
    }
}
#[test]
fn idyllic_beachfront_enters_with_an_adjectival_depictive() {
    let mut this = word("vocab:SingularDemonstrative/This");
    if let LexicalReading::Word(value) = &mut this.value {
        value.capitalization = SurfaceCase::Initial;
    }
    exact(
        "This land enters tapped.",
        Category::Sentence,
        Reading::Sentence {
            form: 0,
            clause: Box::new(Reading::Declarative {
                form: 0,
                clause: Box::new(Reading::FiniteClause {
                    form: 0,
                    subject: Box::new(Reading::CasePhrase {
                        form: 0,
                        category: Category::NominativePhrase,
                        head: Box::new(Reading::DeterminedNounPhrase {
                            form: 0,
                            determiner: this,
                            head: Box::new(noun("lexeme:type/land")),
                        }),
                    }),
                    predicate: Box::new(Reading::DepictivePredicate {
                        form: 0,
                        category: Category::FinitePredicate,
                        head: verb("core-verb:Enter", WordForm::Present, 0),
                        modifier: Box::new(Reading::AdjectivalDepictive {
                            form: 0,
                            phrase: Box::new(Reading::Adjective {
                                form: 0,
                                head: word("lexeme:keyword_action/tap/adjective"),
                            }),
                        }),
                    }),
                }),
            }),
        },
    );
}
#[test]
fn righteous_blow_keeps_coordinated_verbal_modifiers_under_the_nominal() {
    exact(
        "target attacking or blocking creature",
        Category::NounPhrase,
        Reading::TargetNounPhrase {
            form: 0,
            marker: word("vocab:TargetingMarker/Target"),
            head: Box::new(Reading::ParticipialPremodifier {
                form: 0,
                modifier: Box::new(Reading::Coordination {
                    form: 0,
                    category: Category::VerbalPremodifier,
                    left: Box::new(Reading::VerbalPremodifier {
                        form: 0,
                        head: verb("core-verb:Attack", WordForm::GerundParticiple, 0),
                    }),
                    coordinator: word("vocab:Coordinator/Or"),
                    right: Box::new(Reading::VerbalPremodifier {
                        form: 0,
                        head: verb("core-verb:Block", WordForm::GerundParticiple, 0),
                    }),
                }),
                head: Box::new(noun("lexeme:type/creature")),
            }),
        },
    );
}

#[test]
fn yore_tiller_nephilim_coordinates_adjectival_and_participial_depictives() {
    // Yore-Tiller Nephilim: "... to the battlefield tapped and attacking."
    exact(
        "tapped and attacking",
        Category::DepictivePhrase,
        Reading::Coordination {
            form: 0,
            category: Category::DepictivePhrase,
            left: Box::new(Reading::AdjectivalDepictive {
                form: 0,
                phrase: Box::new(Reading::Adjective {
                    form: 0,
                    head: word("lexeme:keyword_action/tap/adjective"),
                }),
            }),
            coordinator: word("vocab:Coordinator/And"),
            right: Box::new(Reading::ParticipialDepictive {
                form: 0,
                phrase: Box::new(Reading::IntransitivePredicate {
                    form: 0,
                    category: Category::SecondaryVerbPhrase,
                    head: verb("core-verb:Attack", WordForm::GerundParticiple, 0),
                }),
            }),
        },
    );
}

#[test]
fn copular_become_does_not_select_a_gerund_participial_predicate() {
    let grammar = Grammar::default();
    let analyzed = LEXICON.analyze("becomes attacking");
    let forest = parse(&grammar, &LEXICON, &analyzed, &Category::FinitePredicate).unwrap();
    assert_eq!(grammar.readings(&forest).count(), 0);
}

#[test]
fn hero_of_bladehold_auxiliary_frame_is_not_an_intransitive_depictive_host() {
    // Hero of Bladehold's "... tokens that are tapped and attacking."
    // The independently built wrong analysis treats the passive auxiliary
    // frame as an intransitive lexical host for a depictive adjunct.
    let mut are = verb("core-verb:Be", WordForm::Present, 1);
    if let LexicalReading::Word(value) = &mut are.value {
        value.features.number = Some(Number::Plural);
    }
    let modifier = Reading::Coordination {
        form: 0,
        category: Category::DepictivePhrase,
        left: Box::new(Reading::AdjectivalDepictive {
            form: 0,
            phrase: Box::new(Reading::Adjective {
                form: 0,
                head: word("lexeme:keyword_action/tap/adjective"),
            }),
        }),
        coordinator: word("vocab:Coordinator/And"),
        right: Box::new(Reading::ParticipialDepictive {
            form: 0,
            phrase: Box::new(Reading::IntransitivePredicate {
                form: 0,
                category: Category::SecondaryVerbPhrase,
                head: verb("core-verb:Attack", WordForm::GerundParticiple, 0),
            }),
        }),
    };
    let invalid = Reading::DepictivePredicate {
        form: 0,
        category: Category::FinitePredicate,
        head: are,
        modifier: Box::new(modifier),
    };
    assert!(invalid.admit(&LEXICON).is_err());
}

#[test]
fn grafdiggers_cage_enter_the_battlefield_keeps_an_object_complement() {
    // Grafdigger's Cage: "Creature cards in graveyards and libraries can't enter the battlefield."
    exact(
        "enter the battlefield",
        Category::SecondaryVerbPhrase,
        Reading::TransitivePredicate {
            form: 0,
            category: Category::SecondaryVerbPhrase,
            head: verb("core-verb:Enter", WordForm::Plain, 1),
            object: Box::new(Reading::CasePhrase {
                form: 0,
                category: Category::AccusativePhrase,
                head: Box::new(Reading::DeterminedNounPhrase {
                    form: 0,
                    determiner: word("vocab:DefiniteMarker/The"),
                    head: Box::new(noun("lexeme:CommonNoun/Battlefield")),
                }),
            }),
        },
    );
}
