mod common;

use std::collections::BTreeSet;

use common::assert_constituents;
use common::lexicon;
use common::readings;
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

fn noun(id: &str, number: Number) -> Reading {
    let mut head = word(id);
    let LexicalReading::Word(value) = &mut head.value else { unreachable!() };
    value.form = if number == Number::Singular { WordForm::Singular } else { WordForm::Plural };
    value.features.number = Some(number);
    head.countability = Some(true);
    Reading::Noun { form: 0, head }
}

fn case(head: Reading, category: Category) -> Reading {
    Reading::CasePhrase {
        form: 0,
        category,
        head: Box::new(head),
    }
}

fn of(head: Reading, complement: Reading) -> Reading {
    Reading::PostmodifiedNominal {
        form: 0,
        head: Box::new(head),
        modifier: Box::new(Reading::PrepositionPhrase {
            form: 0,
            head: word("vocab:Preposition/Of"),
            complement: Box::new(case(complement, Category::AccusativePhrase)),
        }),
    }
}

fn determined(id: &str, head: Reading) -> Reading {
    Reading::DeterminedNounPhrase {
        form: 0,
        determiner: word(id),
        head: Box::new(head),
    }
}

fn targets() -> Reading {
    Reading::BarePlural {
        form: 0,
        head: Box::new(Reading::TargetedNominal {
            form: 0,
            marker: word("vocab:TargetingMarker/Target"),
            head: Box::new(noun("lexeme:type/creature", Number::Plural)),
        }),
    }
}

fn subject() -> Reading {
    determined(
        "vocab:Determinative/Any",
        of(
            noun("lexeme:CommonNoun/Number", Number::Singular),
            targets(),
        ),
    )
}

fn predicate(number: Number) -> Reading {
    let mut possessor = word("vocab:PossessiveDeterminerPronoun/Your");
    let LexicalReading::Word(value) = &mut possessor.value else { unreachable!() };
    value.features = FeatureBundle {
        number: Some(Number::Singular),
        person: Some(Person::Second),
        case: Some(Case::Genitive),
        ..Default::default()
    };
    let choice = Reading::PossessiveNounPhrase {
        form: 0,
        possessor,
        head: Box::new(noun("lexeme:CommonNoun/Choice", Number::Singular)),
    };
    let color = determined(
        "vocab:DefiniteMarker/The",
        of(noun("lexeme:CommonNoun/Color", Number::Singular), choice),
    );
    let mut head = word("core-verb:Become");
    head.frame = Some(0);
    let LexicalReading::Word(value) = &mut head.value else { unreachable!() };
    value.form = WordForm::Present;
    value.features = FeatureBundle {
        number: Some(number),
        person: Some(Person::Third),
        tense: Some(Tense::Present),
        finiteness: Some(Finiteness::Finite),
        ..Default::default()
    };
    Reading::SelectedPredicate {
        form: 0,
        category: Category::FinitePredicate,
        head,
        complements: vec![FrameValue::Argument(Box::new(Reading::NominalComplement {
            form: 0,
            phrase: Box::new(case(color, Category::AccusativePhrase)),
        }))],
    }
}

fn clause(subject: Reading, number: Number) -> Reading {
    Reading::Declarative {
        form: 0,
        clause: Box::new(Reading::FiniteClause {
            form: 0,
            subject: Box::new(case(subject, Category::NominativePhrase)),
            predicate: Box::new(predicate(number)),
        }),
    }
}

fn assert_laws(value: &Reading, text: &str, category: Category) {
    value.admit(lexicon()).unwrap();
    assert_eq!(value.realize(lexicon()).unwrap(), text);
    let parsed = readings(text, category);
    let observed = parsed
        .get(value)
        .unwrap_or_else(|| panic!("missing independent Reading of {text}: {value:?}"));
    let mut nodes = Vec::new();
    let mut observed_nodes = Vec::new();
    value.visit(&mut |node| nodes.push(node.clone())).unwrap();
    observed
        .visit(&mut |node| observed_nodes.push(node.clone()))
        .unwrap();
    assert_eq!(observed_nodes, nodes);
    let mut words = Vec::new();
    let mut observed_words = Vec::new();
    value
        .visit_words(&mut |word| words.push(word.clone()))
        .unwrap();
    observed
        .visit_words(&mut |word| observed_words.push(word.clone()))
        .unwrap();
    assert_eq!(observed_words, words);
}

#[test]
fn sway_subject_preserves_exact_readings_and_nominal_head() {
    let expected = BTreeSet::from([subject()]);
    assert_eq!(
        readings("any number of target creatures", Category::NounPhrase),
        expected
    );
    for value in expected {
        assert_laws(
            &value,
            "any number of target creatures",
            Category::NounPhrase,
        );
        let mut owners = Vec::new();
        value
            .visit_words(&mut |word| {
                let LexicalReading::Word(value) = &word.value else {
                    panic!("unexpected notation")
                };
                owners.push(value.lexeme.to_string());
            })
            .unwrap();
        assert_eq!(
            owners,
            [
                "vocab:Determinative/Any",
                "lexeme:CommonNoun/Number",
                "vocab:Preposition/Of",
                "vocab:TargetingMarker/Target",
                "lexeme:type/creature"
            ]
        );
    }
}

#[test]
fn independently_constructed_sway_clause_requires_plural_third_concord() {
    // Constituent of Sway of Illusion's first sentence, before its temporal PP.
    let text = "any number of target creatures become the color of your choice";
    let expected = BTreeSet::from([clause(subject(), Number::Plural)]);
    assert_eq!(readings(text, Category::Clause), expected);
    for value in expected {
        assert_laws(&value, text, Category::Clause);
    }
    let singular = clause(subject(), Number::Singular);
    assert!(singular.admit(lexicon()).is_err());
    assert!(singular.realize(lexicon()).is_err());
    assert!(
        readings(
            "any number of target creatures becomes the color of your choice",
            Category::Clause
        )
        .is_empty()
    );
    assert_constituents(
        "Any number of target creatures become the color of your choice until end of turn.\nDraw a card.",
        Category::Document,
        &[
            (Category::Nominal, "number"),
            (Category::PrepositionPhrase, "of target creatures"),
        ],
    );
}

#[test]
fn eerie_interlude_keeps_both_relative_clause_scopes_and_plural_concord() {
    let text = "any number of target creatures you control";
    let relative = |head: Reading| {
        let mut you = word("vocab:SubjectPronoun/You");
        let LexicalReading::Word(value) = &mut you.value else { unreachable!() };
        value.features = FeatureBundle {
            number: Some(Number::Singular),
            person: Some(Person::Second),
            case: Some(Case::Nominative),
            ..Default::default()
        };
        let mut control = word("core-verb:Control");
        control.frame = Some(0);
        let LexicalReading::Word(value) = &mut control.value else { unreachable!() };
        value.form = WordForm::Present;
        value.features = FeatureBundle {
            number: Some(Number::Singular),
            person: Some(Person::Second),
            tense: Some(Tense::Present),
            finiteness: Some(Finiteness::Finite),
            ..Default::default()
        };
        Reading::ObjectRelativeNominal {
            form: 0,
            head: Box::new(head),
            relative: Box::new(Reading::ZeroObjectRelativeClause {
                form: 0,
                subject: Box::new(case(
                    Reading::NominativePronoun { form: 0, head: you },
                    Category::NominativePhrase,
                )),
                predicate: Box::new(Reading::FiniteObjectGap {
                    form: 0,
                    head: control,
                }),
            }),
        }
    };
    let Reading::DeterminedNounPhrase { head, .. } = subject() else {
        unreachable!()
    };
    let low = Reading::BarePlural {
        form: 0,
        head: Box::new(Reading::TargetedNominal {
            form: 0,
            marker: word("vocab:TargetingMarker/Target"),
            head: Box::new(relative(noun("lexeme:type/creature", Number::Plural))),
        }),
    };
    let high = Reading::BarePlural {
        form: 0,
        head: Box::new(relative(Reading::TargetedNominal {
            form: 0,
            marker: word("vocab:TargetingMarker/Target"),
            head: Box::new(noun("lexeme:type/creature", Number::Plural)),
        })),
    };
    let expected = BTreeSet::from([
        determined("vocab:Determinative/Any", relative(*head)),
        determined(
            "vocab:Determinative/Any",
            of(noun("lexeme:CommonNoun/Number", Number::Singular), low),
        ),
        determined(
            "vocab:Determinative/Any",
            of(noun("lexeme:CommonNoun/Number", Number::Singular), high),
        ),
    ]);
    assert_eq!(readings(text, Category::NounPhrase), expected);
    for value in expected {
        assert_laws(&value, text, Category::NounPhrase);
        assert_eq!(
            value.admit(lexicon()).unwrap(),
            subject().admit(lexicon()).unwrap()
        );
        assert!(clause(value, Number::Singular).admit(lexicon()).is_err());
    }
    assert_constituents(
        text,
        Category::NounPhrase,
        &[
            (Category::Nominal, "number of target creatures"),
            (Category::ObjectRelativeClause, "you control"),
        ],
    );
    assert_constituents(
        text,
        Category::NounPhrase,
        &[
            (Category::NounPhrase, "target creatures you control"),
            (Category::ObjectRelativeClause, "you control"),
        ],
    );
}

#[test]
fn attested_ordinary_number_constituents_keep_singular_concord() {
    // Magma Sliver's "where X is the number of Slivers on the battlefield";
    // the shorter NP is a constituent under high PP attachment.
    let ordinary = determined(
        "vocab:DefiniteMarker/The",
        of(
            noun("lexeme:CommonNoun/Number", Number::Singular),
            Reading::BarePlural {
                form: 0,
                head: Box::new(noun("lexeme:creature_subtype/sliver", Number::Plural)),
            },
        ),
    );
    assert_laws(&ordinary, "the number of Slivers", Category::NounPhrase);
    let singular = determined(
        "vocab:DefiniteMarker/The",
        noun("lexeme:CommonNoun/Number", Number::Singular),
    );
    assert_eq!(
        ordinary.admit(lexicon()).unwrap(),
        singular.admit(lexicon()).unwrap()
    );
    assert!(clause(ordinary, Number::Plural).admit(lexicon()).is_err());
    // Pain's Reward: "a bid of any number".
    let bare = determined(
        "vocab:Determinative/Any",
        noun("lexeme:CommonNoun/Number", Number::Singular),
    );
    assert_laws(&bare, "any number", Category::NounPhrase);
    let any_target = determined(
        "vocab:Determinative/Any",
        noun("lexeme:CommonNoun/Target", Number::Singular),
    );
    assert_eq!(
        bare.admit(lexicon()).unwrap(),
        any_target.admit(lexicon()).unwrap()
    );
    assert!(clause(bare, Number::Plural).admit(lexicon()).is_err());
}

#[test]
fn transparent_quantification_rejects_singular_obliques_and_preserves_other_heads() {
    for text in ["any number of target creature", "any number of mana"] {
        assert!(readings(text, Category::NounPhrase).is_empty(), "{text}");
    }
    // Sway of Illusion's predicative Complement keeps ordinary head agreement
    // despite having an of-Complement; its lexical head is not transparent.
    let value = predicate(Number::Plural);
    assert_laws(
        &value,
        "become the color of your choice",
        Category::FinitePredicate,
    );
    let Reading::SelectedPredicate { complements, .. } = value else {
        unreachable!()
    };
    let FrameValue::Argument(complement) = &complements[0] else { unreachable!() };
    let Reading::NominalComplement { phrase, .. } = &**complement else {
        unreachable!()
    };
    let Reading::CasePhrase { head, .. } = &**phrase else { unreachable!() };
    let color = determined(
        "vocab:DefiniteMarker/The",
        noun("lexeme:CommonNoun/Color", Number::Singular),
    );
    assert_eq!(
        head.admit(lexicon()).unwrap(),
        color.admit(lexicon()).unwrap()
    );
}

#[test]
fn a_later_of_modifier_keeps_the_first_oblique_and_both_attachment_scopes() {
    // Grave Sifter: "returns any number of cards of that type ...".
    let cards = Reading::BarePlural {
        form: 0,
        head: Box::new(noun("lexeme:CommonNoun/Card", Number::Plural)),
    };
    let kind = determined(
        "vocab:SingularDemonstrative/That",
        noun("lexeme:CommonNoun/Type", Number::Singular),
    );
    let high = determined(
        "vocab:Determinative/Any",
        of(
            of(noun("lexeme:CommonNoun/Number", Number::Singular), cards),
            kind,
        ),
    );
    assert_laws(
        &high,
        "any number of cards of that type",
        Category::NounPhrase,
    );
    assert_eq!(
        high.admit(lexicon()).unwrap(),
        subject().admit(lexicon()).unwrap()
    );
    assert_constituents(
        "any number of cards of that type",
        Category::NounPhrase,
        &[
            (Category::Nominal, "number of cards"),
            (Category::PrepositionPhrase, "of that type"),
        ],
    );
    assert_constituents(
        "any number of cards of that type",
        Category::NounPhrase,
        &[(Category::NounPhrase, "cards of that type")],
    );
}
