use crate::LoadError;
use deckmaste_lexical::{Lexeme, LexicalRegistry};
use std::collections::{BTreeMap, BTreeSet};

pub(crate) fn apply(lexemes: &mut [Lexeme], registry: &LexicalRegistry) -> Result<(), LoadError> {
    let fail = |message| LoadError::Registry(message);
    let classes: BTreeMap<_, _> = registry
        .classes
        .iter()
        .map(|class| (class.name.as_str(), class))
        .collect();
    if classes.len() != registry.classes.len() {
        return Err(fail("duplicate class declaration".into()));
    }
    let mut names: BTreeSet<_> = classes.keys().copied().collect();
    let mut ids = BTreeSet::new();
    for reference in &registry.words {
        if !names.insert(reference.name.as_str()) || !ids.insert(reference.id) {
            return Err(fail("duplicate closed-word name or id".into()));
        }
        let word = lexemes
            .iter()
            .find(|word| word.id == reference.id)
            .ok_or_else(|| {
                fail(format!(
                    "{}: unknown closed-word id {}",
                    reference.name, reference.id
                ))
            })?;
        if word.category != reference.category {
            return Err(fail(format!(
                "{}: closed-word Category mismatch",
                reference.name
            )));
        }
        if !core_closed_class(word) {
            return Err(fail(format!(
                "{}: reference requires a core closed-class lexeme",
                reference.name
            )));
        }
    }
    let properties: BTreeMap<_, _> = registry
        .properties
        .iter()
        .map(|property| (property.name.as_str(), property))
        .collect();
    if properties.len() != registry.properties.len() {
        return Err(fail("duplicate property declaration".into()));
    }
    for word in lexemes {
        for class in &word.properties.classes {
            let declaration = classes
                .get(class.as_str())
                .ok_or_else(|| fail(format!("{}: unknown lexical class {class}", word.id)))?;
            if !declaration.categories.contains(&word.category) {
                return Err(fail(format!("{}: Category outside class {class}", word.id)));
            }
        }
        for (name, value) in &word.properties.features {
            if let Some(domain) = properties.get(name.as_str()) {
                let valid = if domain.set {
                    value == "None"
                        || value
                            .split('|')
                            .all(|v| domain.values.iter().any(|allowed| allowed == v))
                } else {
                    domain.values.contains(value)
                };
                if !valid {
                    return Err(fail(format!(
                        "{}: {value:?} outside domain of {name}",
                        word.id
                    )));
                }
            } else if !metadata(name)
                && !name
                    .strip_prefix(crate::native::FEATURE_SOURCE_PREFIX)
                    .is_some_and(|feature| properties.contains_key(feature) || metadata(feature))
            {
                return Err(fail(format!(
                    "{}: unknown lexical property {name}",
                    word.id
                )));
            }
        }
    }
    Ok(())
}

fn core_closed_class(word: &Lexeme) -> bool {
    use deckmaste_lexical::{Category, SourceKind};
    if word.source.kind != SourceKind::Core {
        return false;
    }
    match word.category {
        Category::Preposition
        | Category::Determinative
        | Category::Pronoun
        | Category::Coordinator
        | Category::Subordinator => true,
        Category::Verb => word
            .properties
            .frames
            .iter()
            .any(|frame| frame.kind == "Auxiliary"),
        Category::Adverb => [
            "vocab:PredicateNegator/",
            "vocab:FrequencyAdverb/",
            "vocab:FocusAdverb/",
        ]
        .iter()
        .any(|inventory| word.source.owner.starts_with(inventory)),
        _ => false,
    }
}

// Source-adapter metadata is retained during the staged migration. These keys
// are not grammar features and never grant membership in a lexical class.
fn metadata(name: &str) -> bool {
    // The producer owns this namespace; the cell suffix is opaque provenance,
    // not a grammatical value or an inventory of currently observed overrides.
    name.strip_prefix(crate::supplement::OVERRIDE_SOURCE_PREFIX)
        .is_some_and(|cell| !cell.is_empty())
        || matches!(
            name,
            "OriginalIdentity"
                | "FrameSource"
                | "Contraction"
                | "ParadigmSource"
                | "BareLocativeLicense"
                | "Compoundability"
                | "Countability"
                | "LocativeTemporalLicense"
                | "MannerAnaphorClass"
                | "Properness"
                | "Relationality"
                | "CompoundHead"
                | "source_recipe"
                | "locative_temporal_license"
                | "relationality"
                | "KeywordParameter"
                | "Onset"
                | "FrameMarker"
                | "HomographLicense"
                | "ModifierLicense"
                | "Cardinality"
                | "Quantification"
                | "FormSource"
                | "BareDurationLicense"
                | "DeterminerNumber"
        )
}

#[cfg(test)]
mod tests {
    use super::*;
    use deckmaste_lexical::{Category, LexicalClass, PropertyDomain, Source, SourceKind};

    fn fixture() -> (Vec<Lexeme>, LexicalRegistry) {
        let word = Lexeme::invariant(
            "plugin:marker",
            "by",
            Category::Preposition,
            Source {
                kind: SourceKind::Plugin,
                path: "plugin.ron".into(),
                owner: "plugin:marker".into(),
            },
        );
        let registry = LexicalRegistry {
            classes: vec![LexicalClass {
                name: "Marker".into(),
                categories: vec![Category::Preposition],
            }],
            properties: vec![PropertyDomain {
                name: "Kind".into(),
                values: vec!["A".into(), "B".into()],
                set: false,
            }],
            ..LexicalRegistry::default()
        };
        (vec![word], registry)
    }

    fn reference(word: &Lexeme) -> deckmaste_lexical::ClosedWord {
        deckmaste_lexical::ClosedWord {
            name: "Word".into(),
            id: word.id,
            category: word.category,
        }
    }

    #[test]
    fn references_validate_existing_words_without_changing_them() {
        let (mut words, mut registry) = fixture();
        words[0].source.kind = SourceKind::Core;
        registry.words.push(reference(&words[0]));
        let expected = words.clone();
        apply(&mut words, &registry).unwrap();
        assert_eq!(words, expected);
    }

    #[test]
    fn unknown_reference_ids_and_wrong_categories_are_errors() {
        for category_mismatch in [false, true] {
            let (mut words, mut registry) = fixture();
            words[0].source.kind = SourceKind::Core;
            let mut word = reference(&words[0]);
            if category_mismatch {
                word.category = Category::Subordinator;
            } else {
                word.id = "missing".into();
            }
            registry.words.push(word);
            assert!(apply(&mut words, &registry).is_err());
        }
    }

    #[test]
    fn references_reject_noncore_sources_and_open_categories() {
        for (kind, category) in [
            (SourceKind::Plugin, Category::Preposition),
            (SourceKind::Catalog, Category::Preposition),
            (SourceKind::Core, Category::Noun),
            (SourceKind::Core, Category::Verb),
            (SourceKind::Core, Category::Adverb),
            (SourceKind::Core, Category::Catalog),
        ] {
            let (mut words, mut registry) = fixture();
            words[0].source.kind = kind;
            words[0].category = category;
            registry.words.push(reference(&words[0]));
            assert!(apply(&mut words, &registry).is_err());
        }
    }

    #[test]
    fn closed_adverb_inventories_and_auxiliary_frames_establish_eligibility() {
        let (mut words, mut registry) = fixture();
        words[0].source.kind = SourceKind::Core;
        words[0].source.owner = "vocab:FocusAdverb/Only".into();
        words[0].category = Category::Adverb;
        registry.words.push(reference(&words[0]));
        apply(&mut words, &registry).unwrap();
        words[0] = Lexeme::verb("core-verb:Be", "be", words[0].source.clone());
        words[0].properties.frames.push(deckmaste_lexical::Frame {
            kind: "Auxiliary".into(),
            items: vec![],
        });
        registry.words[0] = reference(&words[0]);
        let expected = words.clone();
        apply(&mut words, &registry).unwrap();
        assert_eq!(words, expected);
    }

    #[test]
    fn reference_names_and_ids_are_unique_across_registry_declarations() {
        for duplicate_id in [false, true] {
            let (mut words, mut registry) = fixture();
            words[0].source.kind = SourceKind::Core;
            registry.words.push(reference(&words[0]));
            let mut other = reference(&words[0]);
            if duplicate_id {
                other.name = "Other".into();
            } else {
                other.id = "different".into();
            }
            registry.words.push(other);
            assert!(apply(&mut words, &registry).is_err());
        }
    }

    #[test]
    fn data_memberships_preserve_plugin_identity_forms_and_properties() {
        let (mut words, registry) = fixture();
        words[0].properties.classes.insert("Marker".into());
        let expected = words.clone();
        apply(&mut words, &registry).unwrap();
        assert_eq!(words, expected);
    }

    #[test]
    fn unknown_reference_ids_and_member_classes_are_errors() {
        let (mut words, mut registry) = fixture();
        let mut named = reference(&words[0]);
        named.id = "missing".into();
        registry.words.push(named);
        assert!(apply(&mut words, &registry).is_err());
        let (mut words, registry) = fixture();
        words[0].properties.classes.insert("Missing".into());
        assert!(apply(&mut words, &registry).is_err());
    }

    #[test]
    fn off_category_membership_and_duplicate_class_declarations_are_errors() {
        let (mut words, registry) = fixture();
        words[0].category = Category::Noun;
        words[0].properties.classes.insert("Marker".into());
        assert!(apply(&mut words, &registry).is_err());
        let (mut words, mut registry) = fixture();
        registry.classes.push(registry.classes[0].clone());
        assert!(apply(&mut words, &registry).is_err());
        // The removed ID-keyed patch format cannot bypass member declarations.
        let data = include_str!("../lexicon/core.ron");
        assert!(ron::from_str::<crate::native::Inventory>(data).is_ok());
        let retired = data.replacen(
            '(',
            "(class_additions: {\"plugin:marker\": [\"Marker\", \"Marker\"]},",
            1,
        );
        assert!(ron::from_str::<crate::native::Inventory>(&retired).is_err());
    }

    #[test]
    fn unknown_properties_and_out_of_domain_values_are_errors() {
        for (name, value) in [("Missing", "A"), ("Kind", "Missing"), ("Kind", "A|B")] {
            let (mut words, registry) = fixture();
            words[0]
                .properties
                .features
                .insert(name.into(), value.into());
            assert!(apply(&mut words, &registry).is_err());
        }
    }

    #[test]
    fn retired_keys_and_unknown_provenance_targets_are_rejected() {
        for name in [
            "BareLocativeComplement",
            "PrepositionAttachment",
            "PrepositionComplementKind",
            "Finiteness",
            "FeatureSource:Missing",
            "FeatureSource:BareLocativeComplement",
            "OverrideSource:",
        ] {
            let (mut words, registry) = fixture();
            words[0]
                .properties
                .features
                .insert(name.into(), "unused".into());
            assert!(apply(&mut words, &registry).is_err(), "{name}");
        }
    }

    #[test]
    fn provenance_namespaces_follow_producers_without_enumerating_cells() {
        let (mut words, registry) = fixture();
        let override_key = format!(
            "{}{:?}:{:?}:{:?}",
            crate::supplement::OVERRIDE_SOURCE_PREFIX,
            deckmaste_lexical::WordForm::Plain,
            Some(deckmaste_lexical::Number::Plural),
            Some(deckmaste_lexical::Person::First)
        );
        let property_key = format!("{}Kind", crate::native::FEATURE_SOURCE_PREFIX);
        words[0].properties.features.extend([
            (override_key, "overrides.ron".into()),
            (property_key, "core.ron".into()),
        ]);
        let expected = words.clone();
        apply(&mut words, &registry).unwrap();
        assert_eq!(words, expected);
    }

    #[test]
    fn finite_sets_validate_every_member_and_allow_empty_sets() {
        for value in ["None", "A", "A|B"] {
            let (mut words, mut registry) = fixture();
            registry.properties[0].set = true;
            words[0]
                .properties
                .features
                .insert("Kind".into(), value.into());
            apply(&mut words, &registry).unwrap();
        }
        let (mut words, mut registry) = fixture();
        registry.properties[0].set = true;
        words[0]
            .properties
            .features
            .insert("Kind".into(), "A|Missing".into());
        assert!(apply(&mut words, &registry).is_err());
    }
}
