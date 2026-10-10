use std::collections::BTreeSet;

use deckmaste_english_v3::grammar::Category;
use deckmaste_english_v3::grammar::FrameValue;
use deckmaste_english_v3::grammar::Reading;
use deckmaste_english_v3::grammar::Word;
use deckmaste_lexical::FeatureBundle;
use deckmaste_lexical::Finiteness;
use deckmaste_lexical::LexicalReading;
use deckmaste_lexical::LexicalValue;
use deckmaste_lexical::Number;
use deckmaste_lexical::SurfaceCase;
use deckmaste_lexical::WordForm;

use crate::common;

fn assert_purpose(text: &str) {
    assert_purpose_as(text, Category::Document);
}

fn assert_purpose_as(text: &str, category: Category) {
    let readings = common::readings(text, category);
    assert!(!readings.is_empty(), "{text}");
    for reading in readings {
        let mut purposes = 0;
        reading
            .visit(&mut |node| {
                if let Reading::ToInfinitive {
                    category: Category::PurposeInfinitival,
                    predicate,
                    ..
                } = node
                {
                    assert_eq!(predicate.category(), Category::BarePredicate);
                    purposes += 1;
                }
            })
            .unwrap();
        assert!(purposes > 0, "{text}: {reading:?}");
    }
}

#[test]
fn molten_note_admits_the_purpose_of_the_bare_passive() {
    assert_purpose(
        "Molten Note deals damage to target creature equal to the amount of mana spent to cast this spell.",
    );
}

#[test]
fn skyrider_elf_keeps_the_purpose_inside_the_mana_postmodifier() {
    assert_purpose(
        "Converge — This creature enters with a +1/+1 counter on it for each color of mana spent to cast it.",
    );
}

#[test]
fn infuse_with_the_elements_keeps_the_purpose_inside_the_where_supplement() {
    assert_purpose(
        "Converge — Put X +1/+1 counters on target creature, where X is the number of colors of mana spent to cast this spell.",
    );
}

#[test]
fn scaled_nurturer_admits_a_finite_predicate_purpose() {
    assert_purpose("When you spend this mana to cast a Dragon creature spell, you gain 2 life.");
}

#[test]
fn angel_of_jubilation_preserves_coordinated_hosts_and_predicates() {
    assert_purpose(
        "Players can't pay life or sacrifice creatures to cast spells or activate abilities.",
    );
}

#[test]
fn piracy_focuses_the_purpose_constituent() {
    assert_purpose("Spend this mana only to cast spells.");
    common::assert_constituents(
        "Spend this mana only to cast spells.",
        Category::Document,
        &[
            (Category::PurposeInfinitival, "only to cast spells"),
            (Category::PurposeInfinitival, "to cast spells"),
            (Category::BarePredicate, "cast spells"),
        ],
    );
}

#[test]
fn the_np_postmodifier_contains_the_passive_and_its_purpose() {
    common::assert_constituents(
        "mana spent to cast this spell",
        Category::NounPhrase,
        &[
            (Category::SecondaryVerbPhrase, "spent to cast this spell"),
            (Category::SecondaryVerbPhrase, "spent"),
            (Category::PurposeInfinitival, "to cast this spell"),
        ],
    );
    for reading in common::readings("mana spent to cast this spell", Category::NounPhrase) {
        let mut hosts = 0;
        reading
            .visit(&mut |node| {
                if let Reading::PurposePredicate { head, modifier, .. } = node {
                    assert_eq!(head.realize(common::lexicon()).unwrap(), "spent");
                    assert_eq!(modifier.category(), Category::PurposeInfinitival);
                    hosts += 1;
                }
            })
            .unwrap();
        assert_eq!(hosts, 1);
    }
    assert!(!common::readings("mana spent", Category::NounPhrase).is_empty());
}

#[test]
fn purpose_does_not_supply_a_negative_clause_or_a_noun_relative() {
    for text in [
        "to not cast this spell",
        "to can't cast this spell",
        "to cast",
        "only only to cast this spell",
    ] {
        assert!(
            common::readings(text, Category::PurposeInfinitival).is_empty(),
            "{text}"
        );
    }
    assert!(common::readings("mana to cast this spell", Category::NounPhrase).is_empty());
    // BQ1–BQ4 now license the measured mass Subject. Purpose attachment must
    // still not supply that Subject when its declared measurement licence is
    // absent; retain the original negative sentence and empty Reading set.
    let mut lexemes: Vec<_> = common::lexicon().lexemes().values().cloned().collect();
    lexemes
        .iter_mut()
        .find(|lexeme| lexeme.id == "vocab:ComparativeQuantifier/More")
        .unwrap()
        .properties
        .features
        .insert("MeasureModifierUse".into(), "No".into());
    let lexicon = deckmaste_lexical::Lexicon::new(lexemes).unwrap();
    assert_eq!(
        common::readings_with_lexicon(
            &lexicon,
            "If three or more mana was spent to cast it, draw a card.",
            Category::Document
        ),
        BTreeSet::new()
    );
}

fn word(owner: &str, form: WordForm) -> Word {
    Word {
        value: LexicalReading::Word(LexicalValue {
            lexeme: owner.into(),
            form,
            features: FeatureBundle::default(),
            variant: 0,
            capitalization: SurfaceCase::Declared,
        }),
        frame: None,
        countability: None,
    }
}

fn spell(number: Number) -> Reading {
    let mut spell = word(
        "lexeme:CommonNoun/Spell",
        if number == Number::Singular { WordForm::Singular } else { WordForm::Plural },
    );
    let LexicalReading::Word(value) = &mut spell.value else { unreachable!() };
    value.features.number = Some(number);
    spell.countability = Some(true);
    Reading::Noun {
        form: 0,
        head: spell,
    }
}

fn purpose(object: Reading) -> Reading {
    let mut cast = word("lexeme:keyword_action/cast", WordForm::Plain);
    cast.frame = Some(0);
    Reading::ToInfinitive {
        category: Category::PurposeInfinitival,
        form: 0,
        marker: word("vocab:InfinitivalMarker/To", WordForm::Invariant),
        predicate: Box::new(Reading::BarePredicate {
            form: 0,
            head: Box::new(Reading::SelectedPredicate {
                category: Category::SecondaryVerbPhrase,
                form: 0,
                head: cast,
                complements: vec![FrameValue::Argument(Box::new(Reading::CasePhrase {
                    category: Category::AccusativePhrase,
                    form: 0,
                    head: Box::new(object),
                }))],
            }),
        }),
    }
}

fn assert_independent(expected: Reading, text: &str) {
    expected.admit(common::lexicon()).unwrap();
    assert_eq!(expected.realize(common::lexicon()).unwrap(), text);
    let actual = common::readings(text, expected.category());
    let expected = BTreeSet::from([expected]);
    assert_eq!(actual, expected);
    let expected = expected.first().unwrap();
    let actual = actual.first().unwrap();
    let mut expected_nodes = Vec::new();
    expected
        .visit(&mut |node| expected_nodes.push(node.clone()))
        .unwrap();
    let mut actual_nodes = Vec::new();
    actual
        .visit(&mut |node| actual_nodes.push(node.clone()))
        .unwrap();
    assert_eq!(actual_nodes, expected_nodes);
    let mut expected_words = Vec::new();
    expected
        .visit_words(&mut |leaf| expected_words.push(leaf.clone()))
        .unwrap();
    let mut actual_words = Vec::new();
    actual
        .visit_words(&mut |leaf| actual_words.push(leaf.clone()))
        .unwrap();
    assert_eq!(actual_words, expected_words);
}

#[test]
fn independently_constructed_purpose_passive_and_focus_preserve_identity() {
    // Attested constituents of Molten Note and Piracy, built without parsing.
    let purpose = purpose(Reading::DeterminedNounPhrase {
        form: 0,
        determiner: word("vocab:SingularDemonstrative/This", WordForm::Invariant),
        head: Box::new(spell(Number::Singular)),
    });
    assert_independent(purpose.clone(), "to cast this spell");
    let mut spent = word("lexeme:Verb/Spend", WordForm::PastParticiple);
    spent.frame = Some(0);
    let LexicalReading::Word(value) = &mut spent.value else { unreachable!() };
    value.features.finiteness = Some(Finiteness::Nonfinite);
    assert_independent(
        Reading::PurposePredicate {
            category: Category::SecondaryVerbPhrase,
            form: 0,
            head: Box::new(Reading::PassivePredicate {
                form: 0,
                head: spent,
            }),
            modifier: Box::new(purpose.clone()),
        },
        "spent to cast this spell",
    );
    assert_independent(
        Reading::FocusedPhrase {
            category: Category::PurposeInfinitival,
            form: 0,
            modifier: word("vocab:FocusAdverb/Only", WordForm::Invariant),
            head: Box::new(self::purpose(Reading::BareNounPhrase {
                form: 0,
                head: Box::new(spell(Number::Plural)),
            })),
        },
        "only to cast spells",
    );
}

#[test]
fn purpose_attachment_reads_the_declared_licence() {
    // Piracy and Molten Note keep their ordinary lexical frames. Changing
    // the declared permission alone disables purpose attachment in both hosts.
    let mut lexemes: Vec<_> = common::lexicon().lexemes().values().cloned().collect();
    let spend = lexemes
        .iter_mut()
        .find(|lexeme| lexeme.id == "lexeme:Verb/Spend")
        .unwrap();
    spend
        .properties
        .features
        .insert("PurposeAdjunctLicence".into(), "No".into());
    let lexicon = deckmaste_lexical::Lexicon::new(lexemes).unwrap();
    for (text, category) in [
        ("Spend this mana only to cast spells.", Category::Document),
        ("mana spent to cast this spell", Category::NounPhrase),
    ] {
        assert_eq!(
            common::readings_with_lexicon(&lexicon, text, category),
            BTreeSet::new()
        );
    }
    assert!(
        !common::readings_with_lexicon(&lexicon, "mana spent", Category::NounPhrase).is_empty()
    );
}

#[test]
fn stative_predicates_keep_their_frames_without_purpose_attachment() {
    for text in [
        "You have a card.",
        "This creature is a Dragon.",
        "This creature remains on the battlefield.",
    ] {
        assert!(
            !common::readings(text, Category::Document).is_empty(),
            "{text}"
        );
        let purpose = format!("{} to cast this spell.", text.trim_end_matches('.'));
        assert_eq!(
            common::readings(&purpose, Category::Document),
            BTreeSet::new(),
            "{purpose}"
        );
    }
}

#[test]
fn auxiliary_predicates_inherit_the_selected_predicates_purpose_licence() {
    assert_eq!(
        common::readings(
            "If {R} was spent to cast this spell, draw a card.",
            Category::Document
        ),
        BTreeSet::new()
    );
    assert_purpose_as("was spent to cast this spell", Category::FinitePredicate);
    assert_purpose("You have sacrificed a creature to cast this spell.");
}
