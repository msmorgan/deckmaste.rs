use std::collections::BTreeSet;

use crate::common::{lexicon, readings};
use deckmaste_english_v3::grammar::{Category, Reading, Word};
use deckmaste_lexical::{
    Case, FeatureBundle, LexicalReading, LexicalValue, Number, Person, SurfaceCase, WordForm,
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

fn partitive(owner: &str, complement: Reading) -> Reading {
    Reading::DeterminerHeadPartitiveNounPhrase {
        form: 0,
        head: word(owner),
        complement: Box::new(complement),
        modifiers: vec![],
    }
}

fn of_them() -> Reading {
    let mut them = word("vocab:ObjectPronoun/Them");
    let LexicalReading::Word(value) = &mut them.value else { unreachable!() };
    value.features = FeatureBundle {
        number: Some(Number::Plural),
        person: Some(Person::Third),
        case: Some(Case::Accusative),
        ..Default::default()
    };
    Reading::PrepositionPhrase {
        form: 0,
        head: word("vocab:Preposition/Of"),
        complement: Box::new(Reading::CasePhrase {
            form: 0,
            category: Category::AccusativePhrase,
            head: Box::new(Reading::AccusativePronoun {
                form: 0,
                head: them,
            }),
        }),
    }
}

fn assert_laws(value: Reading, text: &str) {
    value.admit(lexicon()).unwrap();
    assert_eq!(value.realize(lexicon()).unwrap(), text);
    // Full value equality proves a single route, including lexical owners and
    // the selected Complement's function; it also checks the reverse law.
    assert_eq!(
        readings(text, Category::NounPhrase),
        BTreeSet::from([value])
    );
}

fn assert_role(text: &str, surface: &str, owner: &str, oblique: &str) {
    let documents = readings(text, Category::Document);
    assert!(!documents.is_empty(), "{text}");
    let expected_complements = readings(&format!("of {oblique}"), Category::PrepositionPhrase);
    assert!(!expected_complements.is_empty(), "{oblique}");
    for document in documents {
        let mut found = false;
        document
            .visit(&mut |node| {
                if node.category() == Category::NounPhrase
                    && node.realize(lexicon()).unwrap() == surface
                {
                    let expected: BTreeSet<_> = expected_complements
                        .iter()
                        .map(|complement| partitive(owner, complement.clone()))
                        .collect();
                    assert!(
                        expected.contains(node),
                        "partitive head/Complement role for {surface}"
                    );
                    found = true;
                }
            })
            .unwrap();
        assert!(found, "missing partitive {surface} in {text}");
    }
}

#[test]
fn named_witnesses_select_the_of_complement_under_the_determinative() {
    for (text, surface, owner, oblique) in [
        (
            "{T}: Add one mana of any of the exiled card's colors.",
            "any of the exiled card's colors",
            "vocab:Determinative/Any",
            "the exiled card's colors",
        ),
        (
            "{R}, {T}, Exert this land: Add {R}{R}. If any of that mana is spent on a creature spell, it gains haste until end of turn.",
            "any of that mana",
            "vocab:Determinative/Any",
            "that mana",
        ),
        (
            "Whenever your opponents are dealt combat damage, if any of that damage was dealt by a Warrior, you draw a card and you lose 1 life.",
            "any of that damage",
            "vocab:Determinative/Any",
            "that damage",
        ),
        (
            "{1}{U}: Put a hatchling counter on this creature. Then if there are five or more hatchling counters on it, remove all of them and transform it.",
            "all of them",
            "vocab:FloatedQuantifier/All",
            "them",
        ),
        (
            "+1: Choose up to two target creatures. Until your next turn, whenever either of those creatures deals combat damage, you draw a card.",
            "either of those creatures",
            "vocab:Determinative/Either",
            "those creatures",
        ),
    ] {
        assert_role(text, surface, owner, oblique);
    }
}

#[test]
fn independent_plural_partitives_have_exactly_the_fused_head_route() {
    // Owner principle (2026-10-10): existing members license the natural
    // class, including hypothetical partitives, rather than attested strings.
    for (owner, surface) in [
        ("vocab:FloatedQuantifier/All", "all"),
        ("vocab:FloatedQuantifier/Both", "both"),
        ("vocab:FloatedQuantifier/Each", "each"),
        ("vocab:Determinative/Another", "another"),
        ("vocab:Determinative/Any", "any"),
        ("vocab:Determinative/Either", "either"),
        ("vocab:Determinative/Neither", "neither"),
        ("vocab:Determinative/Some", "some"),
        ("vocab:Determinative/Most", "most"),
        ("vocab:Determinative/Many", "many"),
        ("vocab:ComparativeQuantifier/More", "more"),
        ("vocab:ComparativeQuantifier/Fewer", "fewer"),
        ("vocab:Determinative/None", "none"),
    ] {
        assert_laws(partitive(owner, of_them()), &format!("{surface} of them"));
    }
}

#[test]
fn mass_partitives_use_the_same_selected_complement_construction() {
    for (owner, surface) in [
        ("vocab:Determinative/Any", "any"),
        ("vocab:Determinative/Some", "some"),
        ("vocab:FloatedQuantifier/All", "all"),
        ("vocab:Determinative/Most", "most"),
        ("vocab:Determinative/Much", "much"),
        ("vocab:Determinative/Less", "less"),
        ("vocab:ComparativeQuantifier/More", "more"),
        ("vocab:Determinative/None", "none"),
    ] {
        for complement in readings("of that mana", Category::PrepositionPhrase) {
            assert_laws(
                partitive(owner, complement),
                &format!("{surface} of that mana"),
            );
        }
    }
}

#[test]
fn unrestricted_plural_agreement_is_selected_by_the_verb_without_a_twin_np() {
    // PD2 attests both verb agreements; the owner principle licenses
    // hypothetical lines assembled with the same construction.
    for text in [
        "If any of those cards remain exiled, draw a card.",
        "If any of those cards shares a card type, draw a card.",
    ] {
        assert_role(
            text,
            "any of those cards",
            "vocab:Determinative/Any",
            "those cards",
        );
    }
    let complements = readings("of those cards", Category::PrepositionPhrase);
    let expected: BTreeSet<_> = complements
        .into_iter()
        .map(|complement| partitive("vocab:Determinative/Any", complement))
        .collect();
    assert_eq!(
        readings("any of those cards", Category::NounPhrase),
        expected
    );
}

#[test]
fn number_selection_follows_the_declared_head_use() {
    // Hypothetical class members are explicitly authorized by the owner principle.
    for text in [
        "All of them get +1/+1",
        "Both of them get +1/+1",
        "Many of them get +1/+1",
        "Each of them gets +1/+1",
        "Either of them gets +1/+1",
        "Another of them gets +1/+1",
        "Any of that mana is spent on a creature spell",
        "Much of that mana is spent on a creature spell",
    ] {
        assert!(!readings(text, Category::Clause).is_empty(), "{text}");
    }
    for text in [
        "All of them gets +1/+1",
        "Both of them gets +1/+1",
        "Many of them gets +1/+1",
        "Each of them get +1/+1",
        "Either of them get +1/+1",
        "Another of them get +1/+1",
        "Any of that mana are spent on a creature spell",
        "Much of that mana are spent on a creature spell",
    ] {
        assert!(readings(text, Category::Clause).is_empty(), "{text}");
    }
}

#[test]
fn licensed_head_marker_and_oblique_restrictions_are_retained() {
    for text in [
        "every of them",
        "the of them",
        "no of them",
        "none creatures",
        "any among them",
        "all among them",
        "each of it",
        "either of that mana",
        "another of that mana",
        "both of that mana",
        "many of that mana",
        "much of them",
        "any of the exiled card",
        "some of that creature",
        "all of that creature",
        "much of that creature",
    ] {
        assert!(readings(text, Category::NounPhrase).is_empty(), "{text}");
    }
}

#[test]
fn all_as_object_keeps_the_selected_partitive_complement() {
    assert_role(
        "Remove all of them.",
        "all of them",
        "vocab:FloatedQuantifier/All",
        "them",
    );
}

#[test]
fn noun_oblique_relatives_remain_inside_the_selected_complement() {
    for (head, owner) in [
        ("each", "vocab:FloatedQuantifier/Each"),
        ("any", "vocab:Determinative/Any"),
    ] {
        assert_role(
            &format!("Destroy {head} of those creatures that are Walls."),
            &format!("{head} of those creatures that are Walls"),
            owner,
            "those creatures that are Walls",
        );
    }
}

#[test]
fn fused_head_relatives_remain_a_shared_residual() {
    // PD7 (orchestrator, 2026-10-10): a relative on the fused head
    // is a follow-up shared by Determinative and Cardinal partitives.
    for head in ["any", "each", "two"] {
        assert!(
            readings(
                &format!("Destroy {head} of them that are Walls."),
                Category::Document,
            )
            .is_empty()
        );
    }
}
