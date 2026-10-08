use deckmaste_english_v3::grammar::Category;

use crate::common;

#[test]
fn bare_infinitival_witnesses() {
    for text in [
        "You may pay {W}{U}{B}{R}{G} rather than pay this spell's mana cost.",
        "If three or more creatures are attacking, you may pay {U} rather than pay this spell's mana cost.",
        "You may discard a Plains card rather than pay this spell's mana cost.",
    ] {
        validate_comparisons(text, Category::Document);
        common::assert_constituents(
            text,
            Category::Document,
            &[(Category::SecondaryVerbPhrase, "pay this spell's mana cost")],
        );
    }
}

#[test]
fn nominal_witness() {
    validate_comparisons(
        "Enchanted creature gets +0/+2 and assigns combat damage equal to its toughness rather than its power.",
        Category::Document,
    );
    common::assert_constituents(
        "Enchanted creature gets +0/+2 and assigns combat damage equal to its toughness rather than its power.",
        Category::Document,
        &[(Category::NounPhrase, "its toughness rather than its power")],
    );
}

#[test]
fn gerund_witness() {
    // Access Maze's means Adjunct, independently of its out-of-scope initial frequency.
    validate_comparisons(
        "by paying life equal to its mana value rather than paying its mana cost",
        Category::PrepositionPhrase,
    );
    common::assert_constituents(
        "by paying life equal to its mana value rather than paying its mana cost",
        Category::PrepositionPhrase,
        &[(Category::SecondaryVerbPhrase, "paying its mana cost")],
    );
}

#[test]
fn complement_forms_and_markers_must_match() {
    for (text, category) in [
        (
            "You may pay {U} rather than paying this spell's mana cost.",
            Category::Document,
        ),
        (
            "by paying life equal to its mana value rather than pay its mana cost",
            Category::PrepositionPhrase,
        ),
        (
            "its toughness rather than pay its mana cost",
            Category::NounPhrase,
        ),
        (
            "You may pay {U} rather than its mana cost.",
            Category::Document,
        ),
        (
            "You may pay {U} rather to pay this spell's mana cost.",
            Category::Document,
        ),
        (
            "You may pay {U} rather than pays this spell's mana cost.",
            Category::Document,
        ),
    ] {
        assert!(common::readings(text, category).is_empty(), "{text}");
    }
    // A selected governor cannot be admitted by the old standalone route.
    assert!(common::readings("rather", Category::AdverbPhrase).is_empty());
    assert!(common::readings("rather than its power", Category::AdverbPhrase).is_empty());
    assert!(
        common::readings(
            "rather than its power",
            Category::ComparativeAdjectivePhrase
        )
        .is_empty()
    );
}

fn word(owner: &str) -> deckmaste_english_v3::grammar::Word {
    use deckmaste_lexical::{FeatureBundle, LexicalReading, LexicalValue, SurfaceCase, WordForm};
    deckmaste_english_v3::grammar::Word {
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
fn independent_comparisons_roundtrip_with_traversal_identity() {
    use deckmaste_english_v3::grammar::Reading;
    // Attested constituents from Lethargy Trap, Access Maze, and Gauntlets of Light.
    for (head, complement, category) in [
        (
            "pay {U}",
            "pay this spell's mana cost",
            Category::SecondaryVerbPhrase,
        ),
        (
            "paying life equal to its mana value",
            "paying its mana cost",
            Category::SecondaryVerbPhrase,
        ),
        ("its toughness", "its power", Category::NounPhrase),
    ] {
        let host = common::readings(head, category).into_iter().next().unwrap();
        let term_category = if category == Category::NounPhrase {
            Category::AccusativePhrase
        } else {
            category
        };
        let term = common::readings(complement, term_category)
            .into_iter()
            .next()
            .unwrap();
        let expanded = if category == Category::NounPhrase {
            Reading::NominalMatrixComparativeComplement {
                form: 0,
                head: word("vocab:Preposition/Than"),
                complement: Box::new(term),
            }
        } else {
            Reading::NonfiniteMatrixComparativeComplement {
                form: 0,
                head: word("vocab:Preposition/Than"),
                complement: Box::new(term),
            }
        };
        let expected = Reading::MatchedComparativeConstituent {
            category,
            form: 0,
            head: Box::new(host),
            comparison: Box::new(Reading::MatrixComparativePhrase {
                form: 0,
                governor: word("vocab:Adverb/Rather"),
                complement: Box::new(expanded),
            }),
        };
        let text = format!("{head} rather than {complement}");
        expected.admit(common::lexicon()).unwrap();
        assert_eq!(expected.realize(common::lexicon()).unwrap(), text);
        let values = common::readings(&text, category);
        let actual = values.get(&expected).unwrap();
        let mut expected_nodes = Vec::new();
        let mut actual_nodes = Vec::new();
        expected
            .visit(&mut |node| expected_nodes.push(node.clone()))
            .unwrap();
        actual
            .visit(&mut |node| actual_nodes.push(node.clone()))
            .unwrap();
        assert_eq!(actual_nodes, expected_nodes);
        let mut expected_words = Vec::new();
        let mut actual_words = Vec::new();
        expected
            .visit_words(&mut |leaf| expected_words.push(leaf.clone()))
            .unwrap();
        actual
            .visit_words(&mut |leaf| actual_words.push(leaf.clone()))
            .unwrap();
        assert_eq!(actual_words, expected_words);
    }
}

#[test]
fn governor_selection_reads_the_declared_feature() {
    let mut entries: Vec<_> = common::lexicon().lexemes().values().cloned().collect();
    let governor = entries
        .iter_mut()
        .find(|entry| entry.id.as_str() == "vocab:Adverb/Rather")
        .unwrap();
    governor
        .properties
        .features
        .insert("MatrixComparisonUse".into(), "No".into());
    let unlicensed = deckmaste_lexical::Lexicon::new(entries).unwrap();
    assert!(
        common::readings_with_lexicon(
            &unlicensed,
            "You may pay {U} rather than pay this spell's mana cost.",
            Category::Document
        )
        .is_empty()
    );
}

fn validate_comparisons(text: &str, category: Category) {
    use deckmaste_english_v3::grammar::Reading;
    let values = common::readings(text, category);
    assert!(!values.is_empty(), "{text}");
    eprintln!("{} Readings: {text}", values.len());
    for value in values {
        value.admit(common::lexicon()).unwrap();
        let mut comparisons = 0;
        value
            .visit(&mut |node| {
                if let Reading::MatrixComparativePhrase {
                    governor,
                    complement,
                    ..
                } = node
                {
                    assert_eq!(
                        common::lexicon().realize(&governor.value).unwrap(),
                        "rather"
                    );
                    assert_eq!(complement.category(), Category::MatrixComparativeComplement);
                    assert!(
                        complement
                            .realize(common::lexicon())
                            .unwrap()
                            .starts_with("than ")
                    );
                    comparisons += 1;
                }
            })
            .unwrap();
        assert_eq!(comparisons, 1);
    }
}
