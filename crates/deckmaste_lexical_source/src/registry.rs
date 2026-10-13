use crate::LoadError;
use deckmaste_lexical::{Lexeme, LexicalRegistry};
use std::collections::{BTreeMap, BTreeSet};

pub(crate) fn apply(
    lexemes: &mut [Lexeme],
    registry: &LexicalRegistry,
    additions: BTreeMap<String, Vec<String>>,
) -> Result<(), LoadError> {
    let fail = |message| LoadError::Registry(message);
    let classes: BTreeMap<_, _> = registry
        .classes
        .iter()
        .map(|class| (class.name.as_str(), class))
        .collect();
    if classes.len() != registry.classes.len() {
        return Err(fail("duplicate class declaration".into()));
    }
    let properties: BTreeMap<_, _> = registry
        .properties
        .iter()
        .map(|property| (property.name.as_str(), property))
        .collect();
    if properties.len() != registry.properties.len() {
        return Err(fail("duplicate property declaration".into()));
    }
    for (id, memberships) in additions {
        let word = lexemes
            .iter_mut()
            .find(|word| word.id == id.as_str())
            .ok_or_else(|| fail(format!("unknown class member id {id}")))?;
        for name in memberships {
            if !word.properties.classes.insert(name.clone()) {
                return Err(fail(format!("duplicate membership {id} in {name}")));
            }
        }
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
                    .strip_prefix("FeatureSource:")
                    .is_some_and(|feature| properties.contains_key(feature) || metadata(feature))
            {
                return Err(fail(format!(
                    "{}: unknown lexical property {name}",
                    word.id
                )));
            }
        }
    }
    let ids: BTreeSet<_> = registry.words.iter().map(|word| word.id).collect();
    if ids.len() != registry.words.len() {
        return Err(fail("duplicate closed-word id".into()));
    }
    Ok(())
}

// Source-adapter metadata is retained during the staged migration. These keys
// are not grammar features and never grant membership in a lexical class.
fn metadata(name: &str) -> bool {
    matches!(
        name,
        "OriginalIdentity"
            | "FrameSource"
            | "Contraction"
            | "ParadigmSource"
            | "OverrideSource:PastParticiple:None:None"
            | "OverrideSource:Preterite:None:None"
            | "OverrideSource:GerundParticiple:None:None"
            | "OverrideSource:Present:Some(Singular):Some(Third)"
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
            | "Finiteness"
            | "FormSource"
            | "BareDurationLicense"
            | "DeterminerNumber"
            | "BareLocativeComplement"
            | "PrepositionAttachment"
            | "PrepositionComplementKind"
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

    #[test]
    fn data_memberships_preserve_plugin_identity_forms_and_properties() {
        let (mut words, registry) = fixture();
        let mut expected = words[0].clone();
        expected.properties.classes.insert("Marker".into());
        apply(
            &mut words,
            &registry,
            [("plugin:marker".into(), vec!["Marker".into()])].into(),
        )
        .unwrap();
        assert_eq!(words, [expected]);
    }

    #[test]
    fn unknown_member_ids_and_classes_are_errors() {
        for (id, class) in [("missing", "Marker"), ("plugin:marker", "Missing")] {
            let (mut words, registry) = fixture();
            assert!(
                apply(
                    &mut words,
                    &registry,
                    [(id.into(), vec![class.into()])].into()
                )
                .is_err()
            );
        }
    }

    #[test]
    fn off_category_membership_and_duplicates_are_errors() {
        let (mut words, registry) = fixture();
        words[0].category = Category::Noun;
        assert!(
            apply(
                &mut words,
                &registry,
                [("plugin:marker".into(), vec!["Marker".into()])].into()
            )
            .is_err()
        );
        let (mut words, registry) = fixture();
        assert!(
            apply(
                &mut words,
                &registry,
                [(
                    "plugin:marker".into(),
                    vec!["Marker".into(), "Marker".into()]
                )]
                .into()
            )
            .is_err()
        );
    }

    #[test]
    fn unknown_properties_and_out_of_domain_values_are_errors() {
        for (name, value) in [("Missing", "A"), ("Kind", "Missing"), ("Kind", "A|B")] {
            let (mut words, registry) = fixture();
            words[0]
                .properties
                .features
                .insert(name.into(), value.into());
            assert!(apply(&mut words, &registry, BTreeMap::new()).is_err());
        }
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
            apply(&mut words, &registry, BTreeMap::new()).unwrap();
        }
        let (mut words, mut registry) = fixture();
        registry.properties[0].set = true;
        words[0]
            .properties
            .features
            .insert("Kind".into(), "A|Missing".into());
        assert!(apply(&mut words, &registry, BTreeMap::new()).is_err());
    }
}
