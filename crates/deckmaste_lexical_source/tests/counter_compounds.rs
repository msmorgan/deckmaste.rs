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

fn value(owner: &str, form: WordForm, number: Number) -> LexicalReading {
    LexicalReading::Word(LexicalValue {
        lexeme: owner.into(),
        form,
        features: FeatureBundle {
            number: Some(number),
            ..Default::default()
        },
        variant: 0,
        capitalization: SurfaceCase::Declared,
    })
}

#[test]
fn counter_class_exports_owned_complete_noun_paradigms() {
    let compounds: Vec<_> = LEXICON
        .lexemes()
        .values()
        .filter(|entry| {
            entry.id.starts_with("lexeme:counter_kind/") && entry.id.ends_with("/compound-noun")
        })
        .collect();
    assert_eq!(compounds.len(), 73);
    for compound in compounds {
        assert_eq!(compound.category, Category::Noun);
        assert_eq!(compound.properties.countability, [Countability::Count]);
        assert_eq!(
            compound.properties.features["CompoundHead"],
            "lexeme:CommonNoun/Counter"
        );
        assert_eq!(compound.source.kind, SourceKind::Plugin);
        assert_eq!(
            compound.id.as_str(),
            format!("{}/compound-noun", compound.source.owner)
        );
        assert!(
            compound
                .source
                .path
                .starts_with("plugins_v2/builtin/macros/counter_kinds/")
        );
        assert_ne!(compound.surface_structure, SurfaceStructure::Opaque);
        assert_eq!(
            compound.properties.frames,
            [] as [deckmaste_lexical::Frame; 0]
        );
        assert_eq!(compound.properties.features["SlashPremodifierUse"], "No");
        assert!(!compound.properties.features.contains_key("NounPremodifier"));
    }
}

#[test]
fn independently_constructed_compound_values_satisfy_both_lexical_laws() {
    // Authentic constituents: Coretapper's “a charge counter”; planeswalker
    // Oracle texts refer to “loyalty counters”; +1/+1 and -1/-1 counters are
    // explicitly authored counter-kind spellings.
    for (name, singular, plural, structure) in [
        (
            "chargeCounter",
            "charge counter",
            "charge counters",
            SurfaceStructure::Multiword,
        ),
        (
            "loyaltyCounter",
            "loyalty counter",
            "loyalty counters",
            SurfaceStructure::Multiword,
        ),
        (
            "p1p1Counter",
            "+1/+1 counter",
            "+1/+1 counters",
            SurfaceStructure::MeasuredCompound,
        ),
        (
            "m1m1Counter",
            "-1/-1 counter",
            "-1/-1 counters",
            SurfaceStructure::MeasuredCompound,
        ),
    ] {
        let owner = format!("lexeme:counter_kind/{name}/compound-noun");
        assert_eq!(
            LEXICON.lexemes()[owner.as_str()].surface_structure,
            structure
        );
        for (text, form, number) in [
            (singular, WordForm::Singular, Number::Singular),
            (plural, WordForm::Plural, Number::Plural),
        ] {
            let expected = value(&owner, form, number);
            assert_eq!(LEXICON.realize(&expected).unwrap(), text);
            if structure == SurfaceStructure::MeasuredCompound {
                assert_eq!(
                    LEXICON.surface_features(&expected).unwrap().onset,
                    Some(Onset::Consonant)
                );
            }
            let actual: BTreeSet<_> = LEXICON
                .analyze(text)
                .matches
                .into_iter()
                .filter(|matched| matched.start == 0 && matched.end == text.chars().count())
                .filter_map(|matched| match &matched.reading {
                    LexicalReading::Word(value) if value.lexeme == owner.as_str() => {
                        Some(matched.reading)
                    }
                    _ => None,
                })
                .collect();
            assert_eq!(actual, BTreeSet::from([expected]));
        }
    }
}

#[test]
fn compound_export_does_not_promote_or_coordinate_its_internal_stems() {
    for stem in ["chargeCounter", "loyaltyCounter"] {
        let owner = format!("lexeme:counter_kind/{stem}");
        let original = &LEXICON.lexemes()[owner.as_str()];
        assert_eq!(original.category, Category::Keyword);
        assert_eq!(original.forms.len(), 1);
        assert_eq!(original.forms[0].form, WordForm::Invariant);
        assert!(!original.properties.features.contains_key("NounPremodifier"));
    }
    for text in [
        "charges counter",
        "loyalties counters",
        "charge and loyalty counters",
    ] {
        assert!(!LEXICON.analyze(text).matches.into_iter().any(|matched|
            matched.start == 0 && matched.end == text.chars().count()
                && matches!(matched.reading, LexicalReading::Word(value) if value.lexeme.ends_with("/compound-noun"))));
    }
}

#[test]
fn native_counter_notation_inventory_has_exact_independent_singular_and_plural_values() {
    // Oracle witnesses include Lightning Serpent, Ebon Praetor, Greater
    // Werewolf, Takklemaggot, Jabari's Influence, and Frankenstein's Monster.
    let expected_members = [
        ("P1P0", "+1/+0"),
        ("P2P2", "+2/+2"),
        ("M0M1", "-0/-1"),
        ("P0P1", "+0/+1"),
        ("M0M2", "-0/-2"),
        ("M2M2", "-2/-2"),
        ("P0P2", "+0/+2"),
        ("P1P2", "+1/+2"),
        ("M2M1", "-2/-1"),
        ("M1M0", "-1/-0"),
    ];
    let actual_owners: BTreeSet<_> = LEXICON
        .lexemes()
        .keys()
        .filter(|owner| owner.starts_with("lexeme:counter_kind_numeric/"))
        .map(std::string::ToString::to_string)
        .collect();
    assert_eq!(
        actual_owners,
        expected_members
            .iter()
            .map(|(name, _)| format!("lexeme:counter_kind_numeric/{name}/compound-noun"))
            .collect()
    );
    for (name, stem) in expected_members {
        let owner = format!("lexeme:counter_kind_numeric/{name}/compound-noun");
        let noun = &LEXICON.lexemes()[owner.as_str()];
        assert_eq!(noun.surface_structure, SurfaceStructure::MeasuredCompound);
        assert_eq!(noun.source.kind, SourceKind::Core);
        assert_eq!(
            noun.source.path,
            "crates/deckmaste_lexical_source/lexicon/core.ron"
        );
        assert_eq!(noun.properties.features["SlashPremodifierUse"], "No");
        assert_eq!(noun.properties.countability, [Countability::Count]);
        for (form, number, head) in [
            (WordForm::Singular, Number::Singular, "counter"),
            (WordForm::Plural, Number::Plural, "counters"),
        ] {
            let expected = value(&owner, form, number);
            let text = format!("{stem} {head}");
            assert_eq!(LEXICON.realize(&expected).unwrap(), text);
            assert_eq!(
                LEXICON.surface_features(&expected).unwrap().onset,
                Some(Onset::Consonant)
            );
            let actual: BTreeSet<_> = LEXICON
                .analyze(&text)
                .matches
                .into_iter()
                .filter(|matched| matched.start == 0 && matched.end == text.chars().count())
                .filter_map(|matched| match &matched.reading {
                    LexicalReading::Word(value) if value.lexeme == owner.as_str() => {
                        Some(matched.reading)
                    }
                    _ => None,
                })
                .collect();
            assert_eq!(actual, BTreeSet::from([expected]));
        }
    }
    for undeclared in ["+3/+7 counter", "-0/-3 counters", "-00/-1 counter"] {
        assert!(
            !LEXICON
                .analyze(undeclared)
                .matches
                .into_iter()
                .any(|matched| matched.start == 0
                    && matched.end == undeclared.chars().count()
                    && matches!(matched.reading, LexicalReading::Word(value)
                    if value.lexeme.starts_with("lexeme:counter_kind_numeric/")))
        );
    }
}
