use std::collections::BTreeSet;
use std::path::Path;
use std::sync::LazyLock;

use deckmaste_lexical::{
    Category, Countability, FeatureBundle, LexicalReading, LexicalValue, Lexicon, Number, Onset,
    SourceKind, SurfaceCase, WordForm,
};

static LEXICON: LazyLock<Lexicon> = LazyLock::new(|| {
    Lexicon::new(
        deckmaste_lexical_source::load_workspace(
            &Path::new(env!("CARGO_MANIFEST_DIR")).join("../.."),
        )
        .unwrap()
        .lexemes,
    )
    .unwrap()
});

fn value(owner: &str, number: Number, casing: SurfaceCase) -> LexicalValue {
    LexicalValue {
        lexeme: owner.into(),
        form: if number == Number::Singular { WordForm::Singular } else { WordForm::Plural },
        features: FeatureBundle {
            number: Some(number),
            ..FeatureBundle::default()
        },
        variant: 0,
        capitalization: casing,
    }
}

#[test]
fn negative_prefix_changes_the_pronounced_onset_without_changing_noun_number() {
    for parent in ["lexeme:type/artifact", "lexeme:creature_subtype/elf"] {
        for number in [Number::Singular, Number::Plural] {
            assert_eq!(
                LEXICON
                    .surface_features(&LexicalReading::Word(value(
                        parent,
                        number,
                        SurfaceCase::Declared
                    )))
                    .unwrap()
                    .onset,
                Some(Onset::Vowel)
            );
            assert_eq!(
                LEXICON
                    .surface_features(&LexicalReading::Word(value(
                        &format!("{parent}/non"),
                        number,
                        SurfaceCase::Declared
                    )))
                    .unwrap()
                    .onset,
                Some(Onset::Consonant)
            );
        }
    }
}

#[test]
fn declared_type_words_preserve_complete_independent_noun_paradigms() {
    // Elvish Archdruid uses both "each Elf" and "Elf creatures";
    // Go for the Throat uses "nonartifact creature". The modifier license
    // belongs to these count nouns without erasing their inflection identity.
    for (owner, singular, plural, negative_singular, negative_plural) in [
        (
            "lexeme:type/artifact",
            "artifact",
            Some("artifacts"),
            "nonartifact",
            Some("nonartifacts"),
        ),
        (
            "lexeme:type/sorcery",
            "sorcery",
            Some("sorceries"),
            "nonsorcery",
            Some("nonsorceries"),
        ),
        ("lexeme:type/kindred", "kindred", None, "nonkindred", None),
        (
            "lexeme:type/land",
            "land",
            Some("lands"),
            "nonland",
            Some("nonlands"),
        ),
        (
            "lexeme:creature_subtype/elf",
            "Elf",
            Some("Elves"),
            "non-Elf",
            Some("non-Elves"),
        ),
        (
            "lexeme:creature_subtype/vampire",
            "Vampire",
            Some("Vampires"),
            "non-Vampire",
            Some("non-Vampires"),
        ),
        (
            "lexeme:creature_subtype/mouse",
            "Mouse",
            Some("Mice"),
            "non-Mouse",
            Some("non-Mice"),
        ),
        (
            "lexeme:land_subtype/plains",
            "Plains",
            Some("Plains"),
            "non-Plains",
            Some("non-Plains"),
        ),
        (
            "lexeme:artifact_subtype/equipment",
            "Equipment",
            Some("Equipment"),
            "non-Equipment",
            Some("non-Equipment"),
        ),
        (
            "lexeme:planeswalker_subtype/jace",
            "Jace",
            None,
            "non-Jace",
            None,
        ),
    ] {
        for (id, singular, plural) in [
            (owner.to_owned(), singular, plural),
            (format!("{owner}/non"), negative_singular, negative_plural),
        ] {
            let lexeme = &LEXICON.lexemes()[&id];
            assert_eq!(lexeme.category, Category::Noun);
            assert_eq!(lexeme.properties.countability, [Countability::Count]);
            assert_eq!(lexeme.properties.features["NounPremodifier"], "Yes");
            assert_eq!(lexeme.source.kind, SourceKind::Plugin);
            assert_eq!(lexeme.source.owner, owner);
            assert_eq!(lexeme.source.path, LEXICON.lexemes()[owner].source.path);
            let mut expected = BTreeSet::new();
            for (number, spelling) in [(Number::Singular, Some(singular)), (Number::Plural, plural)]
            {
                for casing in [SurfaceCase::Declared, SurfaceCase::Initial] {
                    let val = value(&id, number, casing);
                    if let Some(spelling) = spelling {
                        if casing == SurfaceCase::Initial
                            && spelling.chars().next().unwrap().is_uppercase()
                        {
                            // Already-capitalized subtypes keep their declared
                            // identity, rather than adding an identical casing.
                            assert!(LEXICON.realize(&LexicalReading::Word(val)).is_err());
                            continue;
                        }
                        let surface = if casing == SurfaceCase::Initial {
                            let mut chars = spelling.chars();
                            format!("{}{}", chars.next().unwrap().to_uppercase(), chars.as_str())
                        } else {
                            spelling.into()
                        };
                        assert_eq!(
                            LEXICON.realize(&LexicalReading::Word(val.clone())).unwrap(),
                            surface
                        );
                        assert!(
                            LEXICON
                                .analyze(&surface)
                                .matches
                                .iter()
                                .any(|m| m.start == 0
                                    && m.end == surface.chars().count()
                                    && m.reading == LexicalReading::Word(val.clone()))
                        );
                        expected.insert(val);
                    } else {
                        assert!(LEXICON.realize(&LexicalReading::Word(val)).is_err());
                    }
                }
            }
            assert_eq!(
                LEXICON
                    .values()
                    .filter(|v| v.lexeme == id)
                    .cloned()
                    .collect::<BTreeSet<_>>(),
                expected
            );
        }
    }
}

#[test]
fn negative_prefix_recipe_matches_whole_owner_and_rejects_malformed_spellings() {
    // Anowon, the Ruin Sage: "a non-Vampire creature of their choice".
    // Victim of Night likewise spells each capitalized subtype with a hyphen.
    for (owner, malformed) in [
        (
            "lexeme:type/artifact/non",
            vec![
                "non artifact",
                "non-artifact",
                "non--artifact",
                "nonartifact-",
                "-nonartifact",
                "xnonartifact",
            ],
        ),
        (
            "lexeme:creature_subtype/vampire/non",
            vec![
                "nonVampire",
                "non Vampire",
                "non-vampire",
                "non--Vampire",
                "non-Vampire-",
                "-non-Vampire",
                "xnon-Vampire",
            ],
        ),
    ] {
        for text in malformed {
            assert!(
                !LEXICON.analyze(text).matches.iter().any(|m| m.start == 0
                    && m.end == text.chars().count()
                    && matches!(&m.reading, LexicalReading::Word(v) if v.lexeme == owner)),
                "{owner}: {text}"
            );
        }
    }
}

#[test]
fn grammatical_type_nouns_remain_distinct_from_exact_catalog_atoms() {
    for (noun, catalog, text) in [
        (
            "lexeme:type/artifact",
            "catalog:card-types.txt/Artifact",
            "Artifact",
        ),
        (
            "lexeme:creature_subtype/vampire",
            "catalog:creature-types.txt/Vampire",
            "Vampire",
        ),
    ] {
        assert_eq!(LEXICON.lexemes()[catalog].category, Category::Catalog);
        assert_eq!(LEXICON.lexemes()[noun].category, Category::Noun);
        let owners: BTreeSet<_> = LEXICON
            .analyze(text)
            .matches
            .into_iter()
            .filter_map(|m| match m.reading {
                LexicalReading::Word(v) if m.start == 0 && m.end == text.chars().count() => {
                    Some(v.lexeme)
                }
                _ => None,
            })
            .collect();
        assert!(owners.contains(noun));
        assert!(owners.contains(catalog));
    }
}
