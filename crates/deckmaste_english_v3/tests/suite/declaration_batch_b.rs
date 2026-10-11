use crate::common;
use deckmaste_english_v3::grammar::{Category, Reading, Word};
use deckmaste_lexical::{LexicalReading, WordForm};

fn check(card: &str, text: &str, role: impl Fn(&Reading) -> bool) {
    let values = common::readings_on_card(card, text, Category::Document);
    assert!(!values.is_empty(), "{card}");
    for value in values {
        let mut found = false;
        value.visit(&mut |node| found |= role(node)).unwrap();
        assert!(found, "missing construction role on {card}");
        value.admit(common::lexicon()).unwrap();
        assert_eq!(value.realize(common::lexicon()).unwrap(), text);
    }
}

fn controlled_postmodifier(node: &Reading) -> bool {
    let Reading::ParticipialPostmodifiedNominal { modifier, .. } = node else {
        return false;
    };
    let mut found = false;
    modifier
        .visit_words(&mut |word| {
            if let LexicalReading::Word(value) = &word.value {
                found |= value.lexeme.as_str() == "core-verb:Control"
                    && value.form == WordForm::PastParticiple;
            }
        })
        .unwrap();
    found
}

#[test]
fn controlled_creatures_keep_the_verbal_postmodifier() {
    for (card, text) in [
        (
            "Barrin's Spite",
            "Choose two target creatures controlled by the same player. Their controller chooses and sacrifices one of them. Return the other to its owner's hand.",
        ),
        (
            "Run Away Together",
            "Choose two target creatures controlled by different players. Return those creatures to their owners' hands.",
        ),
    ] {
        check(card, text, controlled_postmodifier);
    }
}

#[test]
fn outside_is_the_preposition_phrase_complement_of_from() {
    for (card, text) in [
        (
            "Granted",
            "You may reveal a noncreature card you own from outside the game and put it into your hand.",
        ),
        (
            "Living Wish",
            "You may reveal a creature or land card you own from outside the game and put it into your hand. Exile Living Wish.",
        ),
    ] {
        check(card, text, |node| {
            if let Reading::PrepositionComplementPreposition {
                head, complement, ..
            } = node
            {
                return common::lexicon().realize(&head.value).unwrap() == "from"
                    && complement.realize(common::lexicon()).unwrap() == "outside the game";
            }
            false
        });
    }
}

fn central_instead(node: &Reading) -> bool {
    if let Reading::CentralAdverbComplement {
        category,
        modifier,
        predicate,
        ..
    } = node
    {
        return *category == Category::BareComplement
            && predicate.category() == Category::BarePredicate
            && modifier.value == invariant("vocab:Preposition/Instead").value;
    }
    false
}

#[test]
fn modal_instead_retains_its_preposition_identity_in_central_position() {
    for (card, text) in [
        (
            "Parallel Thoughts",
            "If you would draw a card, you may instead put the top card of the pile you exiled into your hand.",
        ),
        (
            "Retriever Phoenix",
            "As long as this card is in your graveyard, if you would learn, you may instead return this card to the battlefield.",
        ),
    ] {
        check(card, text, central_instead);
    }
}

fn invariant(owner: &str) -> Word {
    Word {
        value: LexicalReading::Word(deckmaste_lexical::LexicalValue {
            lexeme: owner.into(),
            form: WordForm::Invariant,
            features: deckmaste_lexical::FeatureBundle::default(),
            variant: 0,
            capitalization: deckmaste_lexical::SurfaceCase::Declared,
        }),
        frame: None,
        countability: None,
    }
}

#[test]
fn independently_assembled_central_preposition_roundtrips() {
    // Retriever Phoenix supplies the attested Bare Predicate.
    let predicates = common::readings(
        "return this card to the battlefield",
        Category::BarePredicate,
    );
    assert!(!predicates.is_empty());
    for predicate in predicates {
        let value = Reading::CentralAdverbComplement {
            category: Category::BareComplement,
            form: 1,
            modifier: invariant("vocab:Preposition/Instead"),
            predicate: Box::new(predicate),
        };
        value.admit(common::lexicon()).unwrap();
        assert_eq!(
            value.realize(common::lexicon()).unwrap(),
            " instead return this card to the battlefield"
        );
        assert!(
            common::readings(
                " instead return this card to the battlefield",
                Category::BareComplement
            )
            .contains(&value)
        );
    }
}

#[test]
fn central_preposition_requires_its_declared_licence() {
    let predicates = common::readings(
        "return this card to the battlefield",
        Category::BarePredicate,
    );
    assert!(!predicates.is_empty());
    for predicate in predicates {
        let value = Reading::CentralAdverbComplement {
            category: Category::BareComplement,
            form: 1,
            modifier: invariant("vocab:Preposition/Outside"),
            predicate: Box::new(predicate),
        };
        assert!(value.admit(common::lexicon()).is_err());
    }
}

#[test]
fn independently_assembled_passive_complement_retains_control_participle() {
    // Barrin's Spite's Bare Passive also supplies the copular passive Complement.
    let heads = common::readings(
        "controlled by the same player",
        Category::SecondaryVerbPhrase,
    );
    assert!(!heads.is_empty());
    for head in heads {
        let value = Reading::PassiveComplement {
            form: 0,
            head: Box::new(head),
        };
        value.admit(common::lexicon()).unwrap();
        assert_eq!(
            value.realize(common::lexicon()).unwrap(),
            "controlled by the same player"
        );
        assert!(
            common::readings(
                "controlled by the same player",
                Category::ParticipialPredicate
            )
            .contains(&value)
        );
    }
}

#[test]
fn independently_assembled_nested_source_roundtrips() {
    let complements = common::readings("outside the game", Category::PrepositionPhrase);
    assert!(!complements.is_empty());
    for complement in complements {
        let value = Reading::PrepositionComplementPreposition {
            form: 0,
            head: invariant("vocab:Preposition/From"),
            complement: Box::new(complement),
        };
        value.admit(common::lexicon()).unwrap();
        assert_eq!(
            value.realize(common::lexicon()).unwrap(),
            "from outside the game"
        );
        assert!(
            common::readings("from outside the game", Category::PrepositionPhrase).contains(&value)
        );
    }
}
