mod common;

use std::collections::BTreeSet;

use common::lexicon;
use common::readings;
use deckmaste_english_v3::grammar::Category;
use deckmaste_english_v3::grammar::Reading;
use deckmaste_english_v3::grammar::Word;
use deckmaste_lexical::FeatureBundle;
use deckmaste_lexical::Finiteness;
use deckmaste_lexical::LexicalReading;
use deckmaste_lexical::LexicalValue;
use deckmaste_lexical::Number;
use deckmaste_lexical::Person;
use deckmaste_lexical::SurfaceCase;
use deckmaste_lexical::Tense;
use deckmaste_lexical::WordForm;

#[test]
fn oracle_constituents_use_declared_verb_frames() {
    // Constituents from Kaza, Roil Chaser; Katara, the Fearless;
    // Sower of Temptation; and Elven Rite, respectively.
    for (text, category) in [
        ("this ability resolves", Category::FiniteClause),
        ("that ability triggers", Category::FiniteClause),
        (
            "this creature remains on the battlefield",
            Category::FiniteClause,
        ),
        ("Distribute two +1/+1 counters", Category::BarePredicate),
    ] {
        assert!(
            !readings(text, category).is_empty(),
            "no Reading for {text:?}"
        );
    }
    for text in [
        "this ability resolve",
        "these abilities resolves",
        "that ability trigger",
        "this creature remain on the battlefield",
        "this creature remains",
        "this creature remains on they",
    ] {
        assert!(
            readings(text, Category::FiniteClause).is_empty(),
            "invalid Reading for {text:?}"
        );
    }
    for value in readings(
        "this creature remains on the battlefield",
        Category::FiniteClause,
    ) {
        let Reading::FiniteClause {
            subject: _,
            predicate,
            ..
        } = value
        else {
            panic!("expected a finite clause");
        };
        let Reading::LocativePredicate {
            category: Category::FinitePredicate,
            form: 0,
            head,
            complement,
            ..
        } = *predicate
        else {
            panic!("expected the declared locative frame");
        };
        assert_eq!(head.frame, Some(1));
        let LexicalReading::Word(verb) = head.value else {
            panic!("expected a verb word");
        };
        assert_eq!(verb.lexeme, "core-verb:Remain");
        assert_eq!(verb.form, WordForm::Present);
        assert_eq!(complement.realize(lexicon()).unwrap(), "on the battlefield");
    }
}

#[test]
fn independent_intransitive_values_retain_frame_and_lexical_identity() {
    for (id, text) in [
        ("core-verb:Resolve", "resolves"),
        ("core-verb:Trigger", "triggers"),
    ] {
        let head = Word {
            value: LexicalReading::Word(LexicalValue {
                lexeme: id.into(),
                form: WordForm::Present,
                features: FeatureBundle {
                    number: Some(Number::Singular),
                    person: Some(Person::Third),
                    tense: Some(Tense::Present),
                    finiteness: Some(Finiteness::Finite),
                    ..FeatureBundle::default()
                },
                variant: 0,
                capitalization: SurfaceCase::Declared,
            }),
            countability: None,
            frame: Some(0),
        };
        let value = Reading::IntransitivePredicate {
            category: Category::FinitePredicate,
            form: 0,
            head: head.clone(),
        };
        value.admit(lexicon()).unwrap();
        assert_eq!(value.realize(lexicon()).unwrap(), text);
        assert_eq!(
            readings(text, Category::FinitePredicate),
            BTreeSet::from([value.clone()])
        );
        let mut nodes = Vec::new();
        value.visit(&mut |node| nodes.push(node.clone())).unwrap();
        assert_eq!(nodes, [value.clone()]);
        let mut leaves = Vec::new();
        value
            .visit_words(&mut |word| leaves.push(word.clone()))
            .unwrap();
        assert_eq!(leaves, [head.clone()]);
        let mut transitive = head;
        transitive.frame = Some(1);
        assert!(
            Reading::IntransitivePredicate {
                category: Category::FinitePredicate,
                form: 0,
                head: transitive
            }
            .admit(lexicon())
            .is_err()
        );
    }
}
