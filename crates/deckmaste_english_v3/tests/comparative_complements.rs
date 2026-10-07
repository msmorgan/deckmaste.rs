mod common;

use common::{assert_constituents, lexicon, readings};
use deckmaste_english_v3::grammar::{Category, Reading};

#[test]
fn equality_complements_belong_to_the_adjective_inside_the_object() {
    // Soul's Fire; Mirkwood Elk's second sentence.
    for (text, object, comparison, complement) in [
        (
            "Target creature you control deals damage equal to its power to any target.",
            "damage equal to its power",
            "equal to its power",
            "its power",
        ),
        (
            "You gain life equal to that card's power.",
            "life equal to that card's power",
            "equal to that card's power",
            "that card's power",
        ),
    ] {
        assert_constituents(text, Category::Document, &[
            (Category::NounPhrase, object),
            (Category::AdjectivePhrase, comparison),
            (Category::NounPhrase, complement),
        ]);
        for reading in readings(text, Category::Document) {
            reading.visit(&mut |node| {
                if node.category() == Category::PrepositionPhrase {
                    assert_ne!(node.realize(lexicon()).unwrap(), format!("to {complement}"));
                }
            }).unwrap();
        }
    }
    assert_eq!(readings("equal to its power", Category::AdjectivePhrase).len(), 1);
}

#[test]
fn equality_is_predicative_and_survives_a_postposed_recipient_order() {
    // Freedom Fighter Recruit; Massive Raid.
    assert_constituents(
        "Freedom Fighter Recruit's power is equal to the number of creatures you control.",
        Category::Document,
        &[(Category::AdjectivePhrase, "equal to the number of creatures you control")],
    );
    assert_constituents(
        "Massive Raid deals damage to any target equal to the number of creatures you control.",
        Category::Document,
        &[(Category::ComparativeAdjectivePhrase, "equal to the number of creatures you control")],
    );
}

#[test]
fn an_independent_nominal_comparison_preserves_structure_and_words() {
    use deckmaste_english_v3::grammar::Word;
    use deckmaste_lexical::{Case, FeatureBundle, LexicalReading, LexicalValue, Number,
        Person, SurfaceCase, WordForm};
    let word = |id: &str, form, features| Word {
        value: LexicalReading::Word(LexicalValue {
            lexeme: id.into(), form, features, variant: 0,
            capitalization: SurfaceCase::Declared,
        }),
        frame: None, countability: None,
    };
    let mut head = word("vocab:ScalarDegree/Equal", WordForm::Invariant, FeatureBundle::default());
    head.frame = Some(0);
    let marker = word("vocab:Preposition/To", WordForm::Invariant, FeatureBundle::default());
    let possessor = word("vocab:PossessiveDeterminerPronoun/Its", WordForm::Invariant,
        FeatureBundle { number: Some(Number::Singular), person: Some(Person::Third),
            case: Some(Case::Genitive), ..FeatureBundle::default() });
    let mut power = word("lexeme:CommonNoun/Power", WordForm::Singular,
        FeatureBundle { number: Some(Number::Singular), ..FeatureBundle::default() });
    power.countability = Some(true);
    let value = Reading::ComparativeAdjective {
        form: 0,
        phrase: Box::new(Reading::ComparativeAdjectivePhrase {
            form: 0,
            governor: Box::new(Reading::ComparativeGovernor {
                form: 0, head: head.clone(), marker: marker.clone(),
            }),
            complement: Box::new(Reading::NominalComparativeComplement {
                form: 0,
                value: Box::new(Reading::CasePhrase {
                    category: Category::AccusativePhrase, form: 0,
                    head: Box::new(Reading::PossessiveNounPhrase {
                        form: 0, possessor: possessor.clone(),
                        head: Box::new(Reading::Noun { form: 0, head: power.clone() }),
                    }),
                }),
            }),
        }),
    };
    // Soul's Fire's comparative constituent, constructed independently.
    value.admit(lexicon()).unwrap();
    assert_eq!(value.realize(lexicon()).unwrap(), "equal to its power");
    assert_eq!(readings("equal to its power", Category::AdjectivePhrase),
        std::collections::BTreeSet::from([value.clone()]));
    let mut leaves = Vec::new();
    value.visit_words(&mut |word| leaves.push(word.clone())).unwrap();
    assert_eq!(leaves, [head, marker, possessor, power]);
}
