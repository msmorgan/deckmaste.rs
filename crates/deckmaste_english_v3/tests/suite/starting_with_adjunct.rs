use crate::common;
use deckmaste_english_v3::grammar::{Category, Reading, Word};
use deckmaste_lexical::{
    Case, FeatureBundle, LexicalReading, LexicalValue, Number, Person, SurfaceCase, WordForm,
};

fn starting_adjunct(text: &str) {
    let values = common::readings(text, Category::Document);
    assert!(!values.is_empty(), "missing Reading of {text:?}");
    for value in values {
        let mut adjuncts = Vec::new();
        let mut starting_phrases = Vec::new();
        value
            .visit(&mut |node| {
                if let Reading::PrepositionComplementPreposition {
                    head, complement, ..
                } = node
                    && common::lexicon()
                        .realize(&head.value)
                        .unwrap()
                        .eq_ignore_ascii_case("starting")
                {
                    assert!(matches!(
                        complement.as_ref(),
                        Reading::PrepositionPhrase { .. }
                    ));
                    starting_phrases.push(complement.realize(common::lexicon()).unwrap());
                }
                if let Reading::InitialPreposition { dependent, .. } = node
                    && let Reading::PrepositionComplementPreposition {
                        head, complement, ..
                    } = dependent.as_ref()
                    && common::lexicon()
                        .realize(&head.value)
                        .unwrap()
                        .eq_ignore_ascii_case("starting")
                {
                    adjuncts.push(complement.realize(common::lexicon()).unwrap());
                }
            })
            .unwrap();
        // Every Reading keeps the entire with-PP inside the clause Adjunct.
        assert_eq!(starting_phrases, vec!["with you"]);
        assert_eq!(adjuncts, vec!["with you"]);
        value.admit(common::lexicon()).unwrap();
    }
}

#[test]
fn horus_heresy_preposes_the_order_adjunct() {
    starting_adjunct(
        "III — Starting with you, each player chooses a creature. Destroy each creature chosen this way.",
    );
}

#[test]
fn eureka_preserves_independent_attachments() {
    starting_adjunct(
        "Starting with you, each player may put a permanent card from their hand onto the battlefield. Repeat this process until no one puts a card onto the battlefield.",
    );
}

#[test]
fn whims_of_the_fates_preposes_the_order_adjunct() {
    starting_adjunct(
        "Starting with you, each player separates all permanents they control into three piles. Then each player chooses one of their piles at random and sacrifices those permanents.",
    );
}

#[test]
fn druid_of_purification_places_the_order_after_the_trigger() {
    starting_adjunct(
        "When this creature enters, starting with you, each player may choose an artifact or enchantment you don't control. Destroy each permanent chosen this way.",
    );
}

#[test]
fn plague_of_vermin_preposes_the_order_adjunct() {
    starting_adjunct("Starting with you, each player may pay any amount of life.");
}

fn invariant(id: &str) -> Word {
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
fn independently_constructed_preposition_keeps_its_selected_complement() {
    let mut you = invariant("vocab:ObjectPronoun/You");
    let LexicalReading::Word(word) = &mut you.value else { unreachable!() };
    word.features = FeatureBundle {
        number: Some(Number::Singular),
        person: Some(Person::Second),
        case: Some(Case::Accusative),
        ..Default::default()
    };
    let expected = Reading::PrepositionComplementPreposition {
        form: 0,
        head: invariant("vocab:Preposition/Starting"),
        complement: Box::new(Reading::PrepositionPhrase {
            form: 0,
            head: invariant("vocab:Preposition/With"),
            complement: Box::new(Reading::CasePhrase {
                category: Category::AccusativePhrase,
                form: 0,
                head: Box::new(Reading::AccusativePronoun { form: 0, head: you }),
            }),
        }),
    };
    expected.admit(common::lexicon()).unwrap();
    assert_eq!(
        expected.realize(common::lexicon()).unwrap(),
        "starting with you"
    );
    assert_eq!(
        common::readings("starting with you", Category::PrepositionPhrase),
        std::collections::BTreeSet::from([expected]),
    );
}

#[test]
fn starting_selects_a_with_phrase_and_cannot_skip_its_complement() {
    for text in [
        "starting",
        "starting you",
        "starting during your turn",
        "starting from you",
    ] {
        assert!(
            common::readings(text, Category::PrepositionPhrase).is_empty(),
            "unlicensed Complement of {text:?}"
        );
    }
}

#[test]
fn starting_nominals_keep_the_attributive_adjective() {
    // Angel of Destiny and Yorion, Sky Nomad attest these constituents.
    for text in ["starting life total", "starting deck"] {
        let values = common::readings(text, Category::Nominal);
        assert!(!values.is_empty());
        for value in values {
            let mut owners = Vec::new();
            value
                .visit_words(&mut |word| {
                    if let LexicalReading::Word(word) = &word.value
                        && word.lexeme.as_str() == "vocab:AttributiveAdjective/Starting"
                    {
                        owners.push(word.lexeme);
                    }
                })
                .unwrap();
            assert_eq!(
                owners,
                vec![deckmaste_lexical::LexemeId::new(
                    "vocab:AttributiveAdjective/Starting"
                )]
            );
            let mut wrong = Vec::new();
            value
                .visit(&mut |node| {
                    if let Reading::PrepositionComplementPreposition { .. } = node {
                        wrong.push(node.clone());
                    }
                })
                .unwrap();
            assert_eq!(wrong, Vec::<Reading>::new());
        }
    }
}

#[test]
fn existing_preposition_controls_preserve_their_analyses() {
    for text in [
        "During your turn, each player may pay any amount of mana.",
        "As this creature enters, each player may pay any amount of mana.",
    ] {
        let values = common::readings(text, Category::Document);
        assert!(!values.is_empty());
        for value in values {
            let mut preposed = Vec::new();
            value
                .visit(&mut |node| {
                    if let Reading::InitialPreposition { dependent, .. } = node {
                        preposed.push(dependent.realize(common::lexicon()).unwrap());
                    }
                })
                .unwrap();
            assert_eq!(preposed, vec![text.split(", ").next().unwrap()]);
        }
    }
    let values = common::readings("with you", Category::PrepositionPhrase);
    assert!(!values.is_empty());
    for value in values {
        let Reading::PrepositionPhrase {
            head, complement, ..
        } = value
        else {
            panic!("with is the PP head")
        };
        assert_eq!(head, invariant("vocab:Preposition/With"));
        assert_eq!(complement.realize(common::lexicon()).unwrap(), "you");
    }
}
