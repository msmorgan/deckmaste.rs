use deckmaste_english_v3::grammar::Category;
use deckmaste_english_v3::grammar::Reading;
use deckmaste_english_v3::grammar::Word;
use deckmaste_lexical::FeatureBundle;
use deckmaste_lexical::LexicalReading;
use deckmaste_lexical::LexicalValue;
use deckmaste_lexical::Number;
use deckmaste_lexical::SurfaceCase;
use deckmaste_lexical::WordForm;

use crate::common::assert_constituents;
use crate::common::lexicon;
use crate::common::readings;

#[test]
fn mind_slash_predicative_as_now_reads() {
    assert_constituents(
        "Activate only as a sorcery.",
        Category::Document,
        &[
            (Category::PrepositionPhrase, "only as a sorcery"),
            (Category::PrepositionPhrase, "as a sorcery"),
        ],
    );
    assert_eq!(
        readings("only as a sorcery", Category::PrepositionPhrase).len(),
        1
    );
}

#[test]
fn celestial_enforcer_focuses_the_conditional_adjunct() {
    assert_constituents(
        "Activate only if you control a creature with flying.",
        Category::Document,
        &[
            (
                Category::PrepositionPhrase,
                "only if you control a creature with flying",
            ),
            (
                Category::PrepositionPhrase,
                "if you control a creature with flying",
            ),
            (Category::FiniteClause, "you control a creature with flying"),
        ],
    );
}

#[test]
fn hammer_of_bogardan_focuses_the_temporal_adjunct() {
    assert_constituents(
        "Activate only during your upkeep.",
        Category::Document,
        &[
            (Category::PrepositionPhrase, "only during your upkeep"),
            (Category::PrepositionPhrase, "during your upkeep"),
        ],
    );
}

#[test]
fn flywheel_racer_focuses_the_conditional_adjunct() {
    assert_constituents(
        "Activate only if this permanent is a creature.",
        Category::Document,
        &[
            (
                Category::PrepositionPhrase,
                "only if this permanent is a creature",
            ),
            (Category::FiniteClause, "this permanent is a creature"),
        ],
    );
}

#[test]
fn bare_as_keeps_its_preposed_licence_and_gains_final_position() {
    assert_eq!(
        readings("as a sorcery", Category::PrepositionPhrase).len(),
        1
    );
    assert_eq!(
        readings("Activate as a sorcery.", Category::Document).len(),
        1
    );
}

#[test]
fn selected_and_comparative_as_does_not_become_a_comparative_adjunct() {
    // Project ruling: fixed-cost-phrases, comparative-admission STOP,
    // resolved 2026-10-06. The implemented comparison stays NP-internal.
    let text = "Counter target spell. Search its controller's graveyard, hand, and library for all cards with the same name as that spell and exile them. Then that player shuffles.";
    let values = readings(text, Category::Document);
    assert!(!values.is_empty());
    for value in values {
        let mut comparisons = 0;
        value
            .visit(&mut |node| {
                if let deckmaste_english_v3::grammar::Reading::DiscontinuousComparativeNominal {
                    ..
                } = node
                {
                    assert_eq!(node.realize(lexicon()).unwrap(), "same name as that spell");
                    comparisons += 1;
                }
                if node.category() == Category::PrepositionPhrase {
                    assert_ne!(node.realize(lexicon()).unwrap(), "as that spell");
                }
            })
            .unwrap();
        assert_eq!(comparisons, 1);
    }
}

#[test]
fn selected_and_comparative_as_does_not_become_an_enter_adjunct() {
    // Project ruling: systemic-residuals, predicative-as, 2026-10-06.
    // The selected replacement now reads; it must never be a final Adjunct.
    let text = "You may have this creature enter as a copy of any creature on the battlefield.";
    crate::enter_as_copy_complement::selected_copy(
        text,
        &[
            "as a copy of any creature",
            "as a copy of any creature on the battlefield",
        ],
        false,
    );
}

#[test]
fn quakebringer_focuses_the_final_adjunct_in_a_finite_clause() {
    let text = "This ability triggers only if Quakebringer is on the battlefield or if Quakebringer is in your graveyard and you control a Giant";
    assert_constituents(
        text,
        Category::FiniteClause,
        &[
            (
                Category::PrepositionPhrase,
                "only if Quakebringer is on the battlefield or if Quakebringer is in your graveyard and you control a Giant",
            ),
            (Category::FiniteClause, "Quakebringer is on the battlefield"),
        ],
    );
}

#[test]
fn defiant_stand_coordinates_focused_adjuncts() {
    assert_constituents(
        "Cast this spell only during the declare attackers step and only if you've been attacked this step.",
        Category::Document,
        &[
            (
                Category::PrepositionPhrase,
                "only during the declare attackers step and only if you've been attacked this step",
            ),
            (
                Category::PrepositionPhrase,
                "only during the declare attackers step",
            ),
            (
                Category::PrepositionPhrase,
                "only if you've been attacked this step",
            ),
        ],
    );
}

#[test]
fn independent_nominal_and_interval_gaps_remain_visible() {
    // Dosan the Falling Leaf and Maddening Imp have independent host gaps.
    for text in [
        "Players can cast spells only during their own turns.",
        "Activate only during an opponent's turn and only before combat.",
    ] {
        assert!(readings(text, Category::Document).is_empty(), "{text}");
    }
}

#[test]
fn focus_does_not_supply_missing_constituents_or_other_functions() {
    for text in [
        "Activate only.",
        "Activate only only during your turn.",
        "Activate only before attackers are declared by.",
        "Activate only as tapped.",
        "Only during your turn, activate.",
    ] {
        assert!(readings(text, Category::Document).is_empty(), "{text}");
    }
    // FO2–FO3 (focus-only-object, 2026-10-10) supersede the blanket
    // prohibition at these two positions. Keep the same inputs as negatives
    // for unintended focus analyses; attested positive witnesses live in
    // focus_only_object, rather than making these invented effects witnesses.
    for text in ["Destroy only target creature.", "Only creatures attack."] {
        let unintended: Vec<_> = readings(text, Category::Document)
            .into_iter()
            .filter(|value| {
                let mut focused_np = false;
                let mut wrong_category = false;
                value
                    .visit(&mut |node| {
                        if let Reading::FocusedPhrase { category, head, .. } = node {
                            focused_np |= *category == Category::NounPhrase;
                            wrong_category |= *category != Category::NounPhrase
                                || head.category() != Category::NounPhrase;
                        }
                    })
                    .unwrap();
                !focused_np || wrong_category
            })
            .collect();
        assert_eq!(unintended, Vec::<Reading>::new(), "{text}");
    }
    assert!(readings("only during your turn", Category::NounPhrase).is_empty());
    // The vocabulary batch supplies Declare; the formerly lexical gap now reads.
    assert_constituents(
        "Activate only before attackers are declared.",
        Category::Document,
        &[(
            Category::PrepositionPhrase,
            "only before attackers are declared",
        )],
    );
    // The once-frequency ticket supplies the previously missing Adjunct.
    assert_constituents(
        "Activate only once each turn.",
        Category::Document,
        &[
            (Category::AdverbPhrase, "only once"),
            (Category::NominalAdjunctPhrase, "each turn"),
        ],
    );
}

fn invariant(owner: &str) -> Word {
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

#[test]
fn independently_constructed_focus_preserves_structure_and_lexical_identity() {
    // A focused temporal PP, constructed without parsing a template.
    let mut article = invariant("vocab:Article/Indefinite");
    let LexicalReading::Word(value) = &mut article.value else { unreachable!() };
    value.features.number = Some(Number::Singular);
    let mut noun = invariant("lexeme:CommonNoun/Turn");
    let LexicalReading::Word(value) = &mut noun.value else { unreachable!() };
    value.form = WordForm::Singular;
    value.features.number = Some(Number::Singular);
    noun.countability = Some(true);
    let nominal = Reading::IndefiniteNounPhrase {
        form: 0,
        determiner: article,
        head: Box::new(Reading::Noun {
            form: 0,
            head: noun,
        }),
    };
    let expected = Reading::FocusedPhrase {
        category: Category::PrepositionPhrase,
        form: 0,
        modifier: invariant("vocab:FocusAdverb/Only"),
        head: Box::new(Reading::PrepositionPhrase {
            form: 0,
            head: invariant("vocab:Preposition/During"),
            complement: Box::new(Reading::CasePhrase {
                category: Category::AccusativePhrase,
                form: 0,
                head: Box::new(nominal.clone()),
            }),
        }),
    };
    expected.admit(lexicon()).unwrap();
    assert_eq!(expected.realize(lexicon()).unwrap(), "only during a turn");
    let parsed = readings("only during a turn", Category::PrepositionPhrase);
    assert_eq!(parsed, [expected.clone()].into_iter().collect());
    let parsed = parsed.first().unwrap();
    let mut expected_nodes = Vec::new();
    expected
        .visit(&mut |node| expected_nodes.push(node.clone()))
        .unwrap();
    let mut parsed_nodes = Vec::new();
    parsed
        .visit(&mut |node| parsed_nodes.push(node.clone()))
        .unwrap();
    assert_eq!(parsed_nodes, expected_nodes);
    let mut words = Vec::new();
    parsed
        .visit_words(&mut |word| {
            let LexicalReading::Word(value) = &word.value else { panic!("non-word leaf") };
            words.push(value.lexeme.to_string());
        })
        .unwrap();
    assert_eq!(
        words,
        [
            "vocab:FocusAdverb/Only",
            "vocab:Preposition/During",
            "vocab:Article/Indefinite",
            "lexeme:CommonNoun/Turn"
        ]
    );

    let mut wrong = expected.clone();
    let Reading::FocusedPhrase { modifier, .. } = &mut wrong else {
        unreachable!()
    };
    *modifier = invariant("vocab:FrequencyAdverb/Once");
    assert!(wrong.admit(lexicon()).is_err());
    assert!(wrong.realize(lexicon()).is_err());

    let Reading::FocusedPhrase { head, .. } = &mut wrong else { unreachable!() };
    **head = Reading::PrepositionPhrase {
        form: 0,
        head: invariant("vocab:Preposition/Of"),
        complement: Box::new(Reading::CasePhrase {
            category: Category::AccusativePhrase,
            form: 0,
            head: Box::new(nominal),
        }),
    };
    let Reading::FocusedPhrase { modifier, .. } = &mut wrong else {
        unreachable!()
    };
    *modifier = invariant("vocab:FocusAdverb/Only");
    assert!(wrong.admit(lexicon()).is_err());
    assert!(wrong.realize(lexicon()).is_err());
}
