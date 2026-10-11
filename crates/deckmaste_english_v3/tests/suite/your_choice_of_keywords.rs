use std::collections::BTreeSet;

use crate::common::{lexicon, readings, readings_on_card, readings_with_lexicon};
use deckmaste_english_v3::grammar::{Category, FrameValue, Reading, Word};
use deckmaste_lexical::{
    Case, FeatureBundle, Frame, FrameItem, FrameSlot, LexicalReading, LexicalValue, Lexicon,
    Number, Person, Relation, SurfaceCase, WordForm,
};

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

fn selected_frame() -> Frame {
    Frame {
        kind: "Nominal".into(),
        items: vec![FrameItem::Argument(FrameSlot {
            relation: Relation::Complement,
            category: "SelectedKeywordPreposition".into(),
        })],
    }
}

fn keyword(name: &str) -> Reading {
    Reading::BareKeyword {
        form: 0,
        head: word(&format!("lexeme:keyword_ability/{name}")),
    }
}

fn choice(keywords: Reading) -> Reading {
    let mut head = word("lexeme:CommonNoun/Choice");
    let LexicalReading::Word(value) = &mut head.value else { unreachable!() };
    value.form = WordForm::Singular;
    value.features.number = Some(Number::Singular);
    head.countability = Some(true);
    head.frame = Some(
        lexicon().lexemes()[&value.lexeme]
            .properties
            .frames
            .iter()
            .position(|frame| frame == &selected_frame())
            .unwrap(),
    );
    let mut possessor = word("vocab:PossessiveDeterminerPronoun/Your");
    let LexicalReading::Word(value) = &mut possessor.value else { unreachable!() };
    value.features = FeatureBundle {
        case: Some(Case::Genitive),
        number: Some(Number::Singular),
        person: Some(Person::Second),
        ..Default::default()
    };
    Reading::PossessiveNounPhrase {
        form: 0,
        possessor,
        head: Box::new(Reading::SelectedComplementNominal {
            form: 1,
            head,
            complement: Box::new(Reading::KeywordComplementPreposition {
                category: Category::SelectedKeywordPreposition,
                form: 0,
                head: word("vocab:Preposition/Of"),
                complement: Box::new(keywords),
            }),
        }),
    }
}

fn without_keyword_frame() -> Lexicon {
    let mut lexemes: Vec<_> = lexicon().lexemes().values().cloned().collect();
    for lexeme in &mut lexemes {
        lexeme
            .properties
            .frames
            .retain(|frame| frame != &selected_frame());
    }
    Lexicon::new(lexemes).unwrap()
}

#[test]
fn independently_constructed_choice_retains_the_whole_keyword_complement() {
    // Ticket NP probe and the alternatives printed on Alchemist's Gift.
    let alternatives = Reading::Coordination {
        category: Category::KeywordPhrase,
        form: 0,
        left: Box::new(keyword("deathtouch")),
        coordinator: word("vocab:Coordinator/Or"),
        right: Box::new(keyword("lifelink")),
    };
    for (text, keywords) in [
        ("your choice of flying", keyword("flying")),
        ("your choice of deathtouch or lifelink", alternatives),
    ] {
        let expected = choice(keywords);
        expected.admit(lexicon()).unwrap();
        assert_eq!(expected.realize(lexicon()).unwrap(), text);
        assert_eq!(
            readings(text, Category::NounPhrase),
            BTreeSet::from([expected])
        );
    }
}

#[test]
fn gain_and_lose_take_choice_as_object_with_keywords_inside_of() {
    for (card, text, verb, alternatives) in [
        (
            "Orcish Medicine",
            "Target creature gains your choice of lifelink or indestructible until end of turn.",
            "Gain",
            "lifelink or indestructible",
        ),
        (
            "Alchemist's Gift",
            "Target creature gets +1/+1 and gains your choice of deathtouch or lifelink until end of turn.",
            "Gain",
            "deathtouch or lifelink",
        ),
        (
            "Shifting Ceratops",
            "{G}: This creature gains your choice of reach, trample, or haste until end of turn.",
            "Gain",
            "reach, trample, or haste",
        ),
        (
            "Golem Artisan",
            "{2}: Target artifact creature gains your choice of flying, trample, or haste until end of turn.",
            "Gain",
            "flying, trample, or haste",
        ),
        (
            "Walking Sponge",
            "{T}: Target creature loses your choice of flying, first strike, or trample until end of turn.",
            "Lose",
            "flying, first strike, or trample",
        ),
    ] {
        let values = readings_on_card(card, text, Category::Document);
        assert!(!values.is_empty(), "{card}");
        for value in values {
            let mut objects = Vec::new();
            let mut keyword_pps = Vec::new();
            let mut duration_hosts = Vec::new();
            value
                .visit(&mut |node| {
                    if let Reading::SelectedPredicate {
                        head, complements, ..
                    } = node
                        && let LexicalReading::Word(head) = &head.value
                        && head.lexeme.as_str() == format!("core-verb:{verb}")
                    {
                        let [FrameValue::Argument(object)] = complements.as_slice() else {
                            panic!("{card}: gain/lose must use its Object frame");
                        };
                        assert_eq!(object.category(), Category::AccusativePhrase);
                        objects.push(object.realize(lexicon()).unwrap());
                    }
                    if let Reading::KeywordComplementPreposition {
                        category,
                        head,
                        complement,
                        ..
                    } = node
                    {
                        assert_eq!(*category, Category::SelectedKeywordPreposition);
                        assert_eq!(head, &word("vocab:Preposition/Of"));
                        keyword_pps.push(complement.realize(lexicon()).unwrap());
                    }
                    if let Reading::PrepositionPredicate { head, modifier, .. } = node
                        && modifier.realize(lexicon()).unwrap() == "until end of turn"
                    {
                        duration_hosts.push(head.realize(lexicon()).unwrap());
                    }
                })
                .unwrap();
            assert_eq!(
                objects,
                [format!("your choice of {alternatives}")],
                "{card}"
            );
            assert_eq!(keyword_pps, [alternatives], "{card}");
            let local_host = format!(
                "{} your choice of {alternatives}",
                if verb == "Gain" { "gains" } else { "loses" }
            );
            // W9-A preserves the same matrix Coordination scope as the
            // control with no choice construction; it is not a twin of-PP.
            if card == "Alchemist's Gift" {
                let coordinated_host = format!("gets +1/+1 and {local_host}");
                assert!(duration_hosts == [local_host] || duration_hosts == [coordinated_host]);
            } else {
                assert_eq!(duration_hosts, [local_host], "{card}");
            }
        }
    }
}

#[test]
fn declared_nominal_frame_is_required_and_parameterized_keywords_are_compositional() {
    let unlicensed = without_keyword_frame();
    for text in [
        "your choice of flying",
        "your choice of deathtouch or lifelink",
    ] {
        assert!(readings_with_lexicon(&unlicensed, text, Category::NounPhrase).is_empty());
    }
    for text in ["a spell of flying", "a creature of deathtouch or lifelink"] {
        assert!(readings(text, Category::NounPhrase).is_empty());
    }
    assert!(readings("of flying", Category::PrepositionPhrase).is_empty());
    // Jodah's Avenger and Urza's Avenger: no keyword-specific route.
    for text in [
        "your choice of double strike, protection from red, vigilance, or shadow",
        "your choice of banding, flying, first strike, or trample",
    ] {
        let values = readings(text, Category::NounPhrase);
        assert!(!values.is_empty(), "{text}");
        for value in values {
            let mut selected = Vec::new();
            value
                .visit(&mut |node| {
                    if let Reading::SelectedComplementNominal { complement, .. } = node {
                        assert_eq!(complement.category(), Category::SelectedKeywordPreposition);
                        selected.push(complement.realize(lexicon()).unwrap());
                    }
                })
                .unwrap();
            assert_eq!(selected, [text.strip_prefix("your choice ").unwrap()]);
        }
    }
}

#[test]
fn controls_preserve_their_complete_reading_sets() {
    let unlicensed = without_keyword_frame();
    for (text, category) in [
        (
            "Target creature gets +1/+1 and gains deathtouch or lifelink until end of turn.",
            Category::Document,
        ),
        (
            "Target creature gains deathtouch or lifelink until end of turn.",
            Category::Document,
        ),
        (
            "This creature enters with your choice of a reach counter or a vigilance counter on it.",
            Category::Document,
        ),
        (
            "When The Third Doctor enters, create your choice of a Clue token, a Food token, or a Treasure token.",
            Category::Document,
        ),
        (
            "Enchanted creature has flying, first strike, and vigilance.",
            Category::Document,
        ),
        ("a spell with flying", Category::NounPhrase),
        ("your choice of a creature or a land", Category::NounPhrase),
        (
            "Put your choice of a +1/+1 counter or a flying counter on target creature.",
            Category::Document,
        ),
    ] {
        let control = readings_with_lexicon(&unlicensed, text, category);
        assert!(!control.is_empty(), "{text}");
        assert_eq!(readings(text, category), control, "{text}");
    }
}
