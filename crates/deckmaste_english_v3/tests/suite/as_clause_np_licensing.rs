use deckmaste_english_v3::grammar::Category;
use deckmaste_english_v3::grammar::Reading;
use deckmaste_lexical::LexicalReading;

use crate::common;

fn has_finite_as(value: &Reading) -> bool {
    let mut found = false;
    value
        .visit(&mut |node| {
            if let Reading::ClauseComplementPreposition { head, .. } = node
                && let LexicalReading::Word(word) = &head.value
                && word.lexeme.as_str() == "vocab:Preposition/As"
            {
                found = true;
            }
        })
        .unwrap();
    found
}

fn comparison_witness(card: &str, text: &str, category: Category, count: usize) {
    let values = common::readings_on_card(card, text, category);
    assert_eq!(values.len(), count, "{card}: {text}");
    for value in values {
        assert!(!has_finite_as(&value), "finite as-Clause in {card}");
        let mut comparisons = Vec::new();
        let mut durations = Vec::new();
        let mut adjuncts = Vec::new();
        value
            .visit(&mut |node| {
                let adjunct = match node {
                    Reading::ClausalPreposition { dependent, .. } => Some(dependent),
                    Reading::PrepositionPredicate { modifier, .. } => Some(modifier),
                    _ => None,
                };
                if let Some(adjunct) = adjunct
                    && let Reading::ClauseComplementPreposition { head, .. } = adjunct.as_ref()
                    && let LexicalReading::Word(word) = &head.value
                    && word.lexeme.as_str() == "vocab:Preposition/Until"
                {
                    adjuncts.push(adjunct.realize(common::lexicon()).unwrap());
                }
                if let Reading::DiscontinuousComparativeNominal { complement, .. } = node {
                    comparisons.push(complement.realize(common::lexicon()).unwrap());
                }
                if let Reading::ClauseComplementPreposition { head, .. } = node
                    && let LexicalReading::Word(word) = &head.value
                    && word.lexeme.as_str() == "vocab:Preposition/Until"
                {
                    durations.push(node.realize(common::lexicon()).unwrap());
                }
            })
            .unwrap();
        assert_eq!(comparisons, ["that permanent"]);
        let subject = if card == "Banishment" { "enchantment" } else { "creature" };
        assert_eq!(
            durations,
            [format!("until this {subject} leaves the battlefield")]
        );
        assert_eq!(adjuncts, durations);
        value.admit(common::lexicon()).unwrap();
        assert_eq!(value.realize(common::lexicon()).unwrap(), text);
    }
}

#[test]
fn comparison_and_duration_survive_without_the_finite_as_alternatives() {
    for (card, unit, face) in [
        (
            "Deputy of Detention",
            "Exile all other nonland permanents that player controls with the same name as that permanent until this creature leaves the battlefield.",
            "When this creature enters, exile target nonland permanent an opponent controls and all other nonland permanents that player controls with the same name as that permanent until this creature leaves the battlefield.",
        ),
        (
            "Banishment",
            "Exile all other nonland permanents your opponents control with the same name as that permanent until this enchantment leaves the battlefield.",
            "Flash\nWhen this enchantment enters, exile target nonland permanent an opponent controls and all other nonland permanents your opponents control with the same name as that permanent until this enchantment leaves the battlefield.",
        ),
    ] {
        comparison_witness(card, unit, Category::Sentence, 30);
        comparison_witness(card, face, Category::Document, 96);
    }
}

#[test]
fn the_negative_probe_has_no_finite_as_clause() {
    let text = "Exile a creature with the same name as that permanent until this creature leaves the battlefield.";
    comparison_witness("Deputy of Detention", text, Category::Document, 5);
}

#[test]
fn finite_as_complements_keep_their_attested_readings() {
    for (card, text, category, count) in [
        (
            "Utopia Sprawl",
            "Enchant Forest\nAs this Aura enters, choose a color.\nWhenever enchanted Forest is tapped for mana, its controller adds an additional one mana of the chosen color.",
            Category::Document,
            9,
        ),
        (
            "Kindred Discovery",
            "As this enchantment enters, choose a creature type.\nWhenever a creature you control of the chosen type enters or attacks, draw a card.",
            Category::Document,
            1,
        ),
        (
            "Lilah, Undefeated Slickshot",
            "as it resolves",
            Category::PrepositionPhrase,
            1,
        ),
    ] {
        let values = common::readings_on_card(card, text, category);
        assert_eq!(values.len(), count, "{card}");
        for value in values {
            assert!(has_finite_as(&value), "missing finite as-Clause in {card}");
            value.admit(common::lexicon()).unwrap();
            assert_eq!(value.realize(common::lexicon()).unwrap(), text);
        }
    }
}

#[test]
fn an_independent_until_modifier_is_rejected_by_admission() {
    // The two constituents of the retired Deputy of Detention Subject NP.
    let head = common::readings("permanent", Category::Nominal)
        .into_iter()
        .next()
        .unwrap();
    let modifier = common::readings("until this creature", Category::PrepositionPhrase)
        .into_iter()
        .next()
        .unwrap();
    let invalid = Reading::PostmodifiedNominal {
        form: 0,
        head: Box::new(head),
        modifier: Box::new(modifier),
    };
    assert!(invalid.admit(common::lexicon()).is_err());
    assert!(
        common::readings("that permanent until this creature", Category::NounPhrase).is_empty()
    );
}
