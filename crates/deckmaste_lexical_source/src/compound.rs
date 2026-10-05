//! Declaration-owned supplemental compound nouns share their head paradigm.

use crate::{LexicalSources, LoadError};
use deckmaste_construction_core::macro_def as metadata;
use deckmaste_lexical::{Category, Source};

pub(crate) struct Declaration {
    pub source: Source,
    pub recipe: metadata::CompoundNounGrammar,
    pub noun_class: Option<metadata::NounClassSemantics>,
}

/// Supplementary compounds inherit the final declared noun-head paradigm.
pub(crate) fn add_compound_nouns(
    output: &mut LexicalSources,
    compounds: Vec<Declaration>,
) -> Result<(), LoadError> {
    for compound in compounds {
        let head = output
            .lexemes
            .iter()
            .find(|entry| entry.id == compound.recipe.head.as_str())
            .ok_or_else(|| LoadError::UnknownOwner {
                property: "compound noun head",
                owner: compound.recipe.head.clone(),
            })?;
        if head.category != Category::Noun {
            return Err(LoadError::InvalidCompoundHead {
                owner: compound.recipe.head,
            });
        }
        let mut noun = head.clone();
        noun.id = format!("{}/compound-noun", compound.source.owner)
            .as_str()
            .into();
        noun.lemma = format!("{} {}", compound.recipe.stem, head.lemma);
        noun.source = compound.source;
        for slot in &mut noun.forms {
            if let Some(surfaces) = &mut slot.surfaces {
                for surface in surfaces {
                    *surface = format!("{} {surface}", compound.recipe.stem);
                }
            }
        }
        noun.onsets.clear();
        noun.article_onsets.clear();
        if let Some(onset) = compound.recipe.stem_onset {
            let head_lexicon =
                deckmaste_lexical::Lexicon::new([head.clone()]).map_err(|source| {
                    LoadError::CompoundHeadLexical {
                        owner: head.id.to_string(),
                        source,
                    }
                })?;
            for value in head_lexicon
                .values()
                .filter(|value| value.capitalization == deckmaste_lexical::SurfaceCase::Declared)
            {
                let head_surface = head_lexicon
                    .realize(&deckmaste_lexical::LexicalReading::Word(value.clone()))
                    .map_err(|source| LoadError::CompoundHeadLexical {
                        owner: head.id.to_string(),
                        source,
                    })?;
                noun.onsets.insert(
                    format!("{} {head_surface}", compound.recipe.stem),
                    match onset {
                        metadata::Onset::Consonant => deckmaste_lexical::Onset::Consonant,
                        metadata::Onset::Vowel => deckmaste_lexical::Onset::Vowel,
                    },
                );
            }
        }
        for feature in ["Onset", "SingularOnset", "PluralOnset"] {
            noun.properties.features.remove(feature);
            noun.properties
                .features
                .remove(&format!("FeatureSource:{feature}"));
        }
        noun.properties
            .features
            .insert("CompoundHead".into(), compound.recipe.head);
        if let Some(class) = compound.noun_class {
            noun.properties.features.insert(
                "locative_temporal_license".into(),
                format!("{:?}", class.locative_temporal_license),
            );
            noun.properties
                .features
                .insert("relationality".into(), format!("{:?}", class.relationality));
        }
        noun.surface_structure = match compound.recipe.stem_structure {
            metadata::CompoundStemStructure::Word => deckmaste_lexical::SurfaceStructure::Multiword,
            metadata::CompoundStemStructure::Measure => {
                deckmaste_lexical::SurfaceStructure::MeasuredCompound
            }
        };
        output.lexemes.push(noun);
    }
    Ok(())
}
