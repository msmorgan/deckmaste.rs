use std::collections::BTreeSet;

use deckmaste_english_v3::grammar::Category;
use deckmaste_english_v3::grammar::FrameValue;
use deckmaste_english_v3::grammar::Reading;
use deckmaste_english_v3::grammar::Word;
use deckmaste_lexical::Case;
use deckmaste_lexical::Countability;
use deckmaste_lexical::FeatureBundle;
use deckmaste_lexical::Finiteness;
use deckmaste_lexical::LexemeId;
use deckmaste_lexical::LexicalReading;
use deckmaste_lexical::LexicalValue;
use deckmaste_lexical::Number;
use deckmaste_lexical::Person;
use deckmaste_lexical::SurfaceCase;
use deckmaste_lexical::Tense;
use deckmaste_lexical::WordForm;

use crate::common;

const ACTION: &str = "lexeme:keyword_action/theRingTemptsYou";
const VERB: &str = "lexeme:keyword_action/theRingTemptsYou/verb";
const NOUN: &str = "lexeme:keyword_action/theRingTemptsYou/noun";
const BEARER: &str = "lexeme:designation/ringBearer";

fn word(owner: &str, form: WordForm, features: FeatureBundle) -> Word {
    Word {
        value: LexicalReading::Word(LexicalValue {
            lexeme: owner.into(),
            form,
            features,
            variant: 0,
            capitalization: SurfaceCase::Declared,
        }),
        frame: None,
        countability: None,
    }
}

fn owner(head: &Word) -> &LexemeId {
    let LexicalReading::Word(value) = &head.value else { panic!("declared word") };
    &value.lexeme
}

fn assert_ring_clause(text: &str) {
    let readings = common::readings(text, Category::Document);
    assert!(!readings.is_empty(), "{text}");
    for reading in readings {
        let mut clauses = Vec::new();
        reading
            .visit(&mut |node| {
                if let Reading::FiniteClause {
                    subject, predicate, ..
                } = node
                    && let Reading::SelectedPredicate {
                        head, complements, ..
                    } = &**predicate
                    && owner(head).as_str() == VERB
                {
                    clauses.push((subject.clone(), head.clone(), complements.clone()));
                }
            })
            .unwrap();
        assert!(!clauses.is_empty(), "{text}");
        for (subject, head, complements) in clauses {
            assert_eq!(subject.category(), Category::NominativePhrase);
            let mut nouns = Vec::new();
            subject
                .visit(&mut |node| {
                    if let Reading::Noun { head, .. } = node {
                        nouns.push(*owner(head));
                    }
                })
                .unwrap();
            assert_eq!(nouns, vec![LexemeId::new(NOUN)]);
            assert_eq!(owner(&head).as_str(), VERB);
            let [FrameValue::Argument(object)] = complements.as_slice() else {
                panic!("one selected Object");
            };
            assert_eq!(object.category(), Category::AccusativePhrase);
            assert_eq!(object.realize(common::lexicon()).unwrap(), "you");
        }
    }
}

#[test]
fn main_coordinated_and_saga_clauses_keep_subject_verb_and_object() {
    // Uruk-hai Berserker, Witch-king of Angmar, One Ring to Rule Them All.
    for text in [
        "When this creature enters, the Ring tempts you.",
        "Whenever one or more creatures deal combat damage to you, each opponent sacrifices a creature of their choice that dealt combat damage to you this turn. The Ring tempts you.",
        "I — The Ring tempts you, then each player mills cards equal to your Ring-bearer's power.",
    ] {
        assert_ring_clause(text);
    }
}

#[test]
fn trigger_condition_is_an_ordinary_finite_clause() {
    // Frodo Baggins's trigger condition.
    let text = "Whenever the Ring tempts you, draw a card.";
    common::assert_constituents(
        text,
        Category::Document,
        &[
            (Category::FiniteClause, "the Ring tempts you"),
            (Category::NominativePhrase, "the Ring"),
            (Category::AccusativePhrase, "you"),
        ],
    );
    assert_ring_clause(text);
}

#[test]
fn designation_is_a_count_noun_under_a_genitive_determiner_and_in_the_plural() {
    // One Ring to Rule Them All and Lord of the Nazgûl.
    let readings = common::readings("your Ring-bearer", Category::NounPhrase);
    assert!(!readings.is_empty());
    for reading in readings {
        let Reading::PossessiveNounPhrase {
            possessor, head, ..
        } = reading
        else {
            panic!("genitive Determiner and nominal head");
        };
        let LexicalReading::Word(value) = &possessor.value else { panic!("pronoun") };
        assert_eq!(value.features.case, Some(Case::Genitive));
        let Reading::Noun { head, .. } = *head else { panic!("noun head") };
        assert_eq!(owner(&head).as_str(), BEARER);
        assert_eq!(head.countability, Some(true));
    }
    common::assert_constituents(
        "Wraiths you control have protection from Ring-bearers.",
        Category::Document,
        &[(Category::Nominal, "Ring-bearers")],
    );
    for reading in common::readings("Ring-bearers", Category::Nominal) {
        let Reading::Noun { head, .. } = reading else { panic!("plural noun") };
        let LexicalReading::Word(value) = &head.value else { panic!("noun word") };
        assert_eq!(value.lexeme, LexemeId::new(BEARER));
        assert_eq!(value.form, WordForm::Plural);
    }
}

#[test]
fn independently_constructed_transitive_predicate_roundtrips() {
    // The predicate printed on Uruk-hai Berserker.
    let mut head = word(
        VERB,
        WordForm::Present,
        FeatureBundle {
            number: Some(Number::Singular),
            person: Some(Person::Third),
            tense: Some(Tense::Present),
            finiteness: Some(Finiteness::Finite),
            case: None,
        },
    );
    head.frame = Some(0);
    let object = Reading::CasePhrase {
        category: Category::AccusativePhrase,
        form: 0,
        head: Box::new(Reading::AccusativePronoun {
            form: 0,
            head: word(
                "vocab:ObjectPronoun/You",
                WordForm::Invariant,
                FeatureBundle {
                    number: Some(Number::Singular),
                    person: Some(Person::Second),
                    case: Some(Case::Accusative),
                    ..Default::default()
                },
            ),
        }),
    };
    let expected = Reading::SelectedPredicate {
        category: Category::FinitePredicate,
        form: 0,
        head,
        complements: vec![FrameValue::Argument(Box::new(object))],
    };
    expected.admit(common::lexicon()).unwrap();
    assert_eq!(expected.realize(common::lexicon()).unwrap(), "tempts you");
    assert_eq!(
        common::readings("tempts you", Category::FinitePredicate),
        BTreeSet::from([expected])
    );
}

#[test]
fn plugin_provenance_and_regular_inflections_replace_the_keyword_leaf() {
    let lexicon = common::lexicon();
    assert!(!lexicon.lexemes().contains_key(&LexemeId::new(ACTION)));
    for (id, source_owner) in [(NOUN, ACTION), (VERB, ACTION), (BEARER, BEARER)] {
        let lexeme = &lexicon.lexemes()[&LexemeId::new(id)];
        assert_eq!(lexeme.source.owner, source_owner);
    }
    assert_eq!(
        lexicon.lexemes()[&LexemeId::new(NOUN)]
            .properties
            .countability,
        vec![Countability::Count]
    );
    for (form, spelling) in [
        (WordForm::Plain, "tempt"),
        (WordForm::Present, "tempts"),
        (WordForm::Preterite, "tempted"),
        (WordForm::PastParticiple, "tempted"),
    ] {
        assert!(lexicon.values().any(|v| v.lexeme == VERB
            && v.form == form
            && lexicon.realize(&LexicalReading::Word(v.clone())).unwrap() == spelling));
    }
}

#[test]
fn neighboring_keyword_actions_and_designation_controls_still_read() {
    for text in [
        "Venture into the dungeon.",
        "You become the monarch.",
        "Put a +1/+1 counter on your commander.",
    ] {
        assert!(
            !common::readings(text, Category::Document).is_empty(),
            "{text}"
        );
    }
}
