use std::collections::BTreeSet;
use std::path::Path;
use std::sync::LazyLock;

use deckmaste_lexical::*;

static LEXICON: LazyLock<Lexicon> = LazyLock::new(|| {
    let root = Path::new(env!("CARGO_MANIFEST_DIR")).join("../..");
    Lexicon::new(
        deckmaste_lexical_source::load_workspace(&root)
            .unwrap()
            .lexemes,
    )
    .unwrap()
});

fn value(owner: &str, form: WordForm, number: Number, casing: SurfaceCase) -> LexicalReading {
    LexicalReading::Word(LexicalValue {
        lexeme: owner.into(),
        form,
        features: FeatureBundle {
            number: Some(number),
            ..Default::default()
        },
        variant: 0,
        capitalization: casing,
    })
}

fn complete_owner(text: &str, owner: &str) -> BTreeSet<LexicalReading> {
    LEXICON
        .analyze(text)
        .matches
        .into_iter()
        .filter(|matched| matched.start == 0 && matched.end == text.chars().count())
        .filter_map(|matched| match &matched.reading {
            LexicalReading::Word(value) if value.lexeme == owner => Some(matched.reading),
            _ => None,
        })
        .collect()
}

#[test]
fn production_multiword_nouns_have_complete_independent_number_and_case_paradigms() {
    for (owner, singular, plural) in [
        ("lexeme:CommonNoun/ManaValue", "mana value", "mana values"),
        ("lexeme:CommonNoun/ManaCost", "mana cost", "mana costs"),
    ] {
        let entry = &LEXICON.lexemes()[owner];
        assert_eq!(entry.category, Category::Noun);
        assert_eq!(entry.properties.countability, [Countability::Count]);
        assert_eq!(entry.surface_structure, SurfaceStructure::Multiword);
        assert_eq!(entry.binding, Binding::Free);
        assert_eq!(entry.source.owner, owner);
        let mut expected = BTreeSet::new();
        for (surface, form, number) in [
            (singular, WordForm::Singular, Number::Singular),
            (plural, WordForm::Plural, Number::Plural),
        ] {
            for casing in [SurfaceCase::Declared, SurfaceCase::Initial] {
                let expected_value = value(owner, form, number, casing);
                let spelling = if casing == SurfaceCase::Initial {
                    format!("M{}", &surface[1..])
                } else {
                    surface.to_owned()
                };
                assert_eq!(
                    LEXICON.realize(&expected_value).unwrap().as_bytes(),
                    spelling.as_bytes()
                );
                assert_eq!(
                    complete_owner(&spelling, owner),
                    BTreeSet::from([expected_value.clone()])
                );
                expected.insert(match expected_value {
                    LexicalReading::Word(value) => value,
                    _ => unreachable!(),
                });
            }
        }
        assert_eq!(
            LEXICON
                .values()
                .filter(|value| value.lexeme == owner)
                .cloned()
                .collect::<BTreeSet<_>>(),
            expected
        );
    }
}

#[test]
fn exact_internal_spaces_and_free_edges_do_not_guess_partial_multiword_entries() {
    for (owner, head, separate_owner) in [
        (
            "lexeme:CommonNoun/ManaValue",
            "value",
            "lexeme:CommonNoun/Value",
        ),
        (
            "lexeme:CommonNoun/ManaCost",
            "cost",
            "lexeme:CommonNoun/Cost",
        ),
    ] {
        for text in [
            format!("mana  {head}"),
            format!("mana\t{head}"),
            format!("mana\n{head}"),
            format!("mana{head}"),
            format!("xmana {head}"),
            format!("mana {head}x"),
            format!("mana {head}sx"),
        ] {
            assert!(!LEXICON.analyze(&text).matches.iter().any(|matched| matches!(&matched.reading, LexicalReading::Word(value) if value.lexeme == owner)), "{text:?}");
        }
        let analyzed = LEXICON.analyze(&format!("mana {head}"));
        for (start, end, expected_owner) in [
            (0, 4, "lexeme:CommonNoun/Mana"),
            (5, 5 + head.len(), separate_owner),
        ] {
            assert!(analyzed.matches.iter().any(|matched| matched.start == start && matched.end == end && matches!(&matched.reading, LexicalReading::Word(value) if value.lexeme == expected_owner)));
        }
        assert!(complete_owner(&format!("Mana {}", head.to_ascii_uppercase()), owner).is_empty());
    }
}

#[test]
fn same_spelled_catalog_entry_does_not_replace_the_ordinary_noun_identity() {
    let noun = LEXICON.lexemes()["lexeme:CommonNoun/ManaValue"].clone();
    // Deliberately synthetic catalog collision; no claim about a real card name.
    let catalog = Lexeme::invariant(
        "catalog:fixture/mana-value",
        "mana value",
        Category::Catalog,
        Source {
            kind: SourceKind::Catalog,
            path: "multiword-test-fixture".into(),
            owner: "catalog:fixture/mana-value".into(),
        },
    );
    let lexicon = Lexicon::new([noun, catalog]).unwrap();
    let owners: BTreeSet<_> = lexicon
        .analyze("mana value")
        .matches
        .into_iter()
        .filter_map(|matched| match matched.reading {
            LexicalReading::Word(value) => Some(value.lexeme),
            _ => None,
        })
        .collect();
    assert_eq!(
        owners,
        BTreeSet::from([
            "lexeme:CommonNoun/ManaValue".into(),
            "catalog:fixture/mana-value".into()
        ])
    );
    assert_eq!(
        lexicon.lexemes()["lexeme:CommonNoun/ManaValue"].category,
        Category::Noun
    );
    assert_eq!(
        lexicon.lexemes()["catalog:fixture/mana-value"].category,
        Category::Catalog
    );
}

#[test]
fn lexicalized_orientation_prepositions_own_exact_invariant_multiword_forms() {
    for (owner, surface) in [
        ("vocab:Preposition/FaceDown", "face down"),
        ("vocab:Preposition/FaceUp", "face up"),
    ] {
        let entry = &LEXICON.lexemes()[owner];
        assert_eq!(entry.category, Category::Preposition);
        assert_eq!(entry.surface_structure, SurfaceStructure::Multiword);
        assert!(entry.properties.countability.is_empty());
        assert_eq!(entry.properties.features["PrepositionComplement"], "None");
        let mut expected_values = BTreeSet::new();
        for casing in [SurfaceCase::Declared, SurfaceCase::Initial] {
            let expected = LexicalReading::Word(LexicalValue {
                lexeme: owner.into(),
                form: WordForm::Invariant,
                features: FeatureBundle::default(),
                variant: 0,
                capitalization: casing,
            });
            let spelling = if casing == SurfaceCase::Initial {
                format!("F{}", &surface[1..])
            } else {
                surface.into()
            };
            assert_eq!(
                LEXICON.realize(&expected).unwrap().as_bytes(),
                spelling.as_bytes()
            );
            assert_eq!(
                complete_owner(&spelling, owner),
                BTreeSet::from([expected.clone()])
            );
            let LexicalReading::Word(value) = expected else { unreachable!() };
            expected_values.insert(value);
        }
        assert_eq!(
            LEXICON
                .values()
                .filter(|value| value.lexeme == owner)
                .cloned()
                .collect::<BTreeSet<_>>(),
            expected_values
        );
        for malformed in [
            surface.replace(' ', "  "),
            surface.replace(' ', "\t"),
            surface.replace(' ', "\n"),
            surface.replace(' ', "-"),
            format!("{surface}s"),
        ] {
            assert!(!LEXICON.analyze(&malformed).matches.iter().any(|matched| matches!(&matched.reading, LexicalReading::Word(value) if value.lexeme == owner)), "{malformed:?}");
        }
    }
    // The existing attributive adjective keeps its own hyphenated spelling.
    assert!(LEXICON.analyze("face-down").matches.iter().any(|matched| matches!(&matched.reading, LexicalReading::Word(value) if value.lexeme == "vocab:AttributiveAdjective/FaceDown")));
}
