use std::collections::BTreeSet;
use std::path::Path;

use crate::LoadError;
use deckmaste_lexical::Lexeme;
use deckmaste_lexical::Number;
use deckmaste_lexical::Person;
use deckmaste_lexical::WordForm;
use serde::Deserialize;

use crate::LexicalSources;

#[derive(Deserialize)]
#[serde(deny_unknown_fields)]
struct Supplement {
    lexemes: Vec<Lexeme>,
    overrides: Vec<Override>,
}

#[derive(Deserialize)]
#[serde(deny_unknown_fields)]
struct Override {
    owner: String,
    form: WordForm,
    /// Selection of slots to replace; omitted dimensions select all their
    /// values.
    number: Option<Number>,
    person: Option<Person>,
    surfaces: Vec<String>,
}

pub(crate) fn apply(root: &Path, output: &mut LexicalSources) -> Result<(), LoadError> {
    let path = "crates/deckmaste_lexical_source/lexicon/overrides.ron";
    let full_path = root.join(path);
    let text = crate::error::read(&full_path)?;
    let supplement = crate::error::decode(&full_path, &text)?;
    apply_declarations(path, supplement, output)
}

fn apply_declarations(
    path: &str,
    supplement: Supplement,
    output: &mut LexicalSources,
) -> Result<(), LoadError> {
    output.lexemes.extend(supplement.lexemes);
    let mut replaced = BTreeSet::new();
    for replacement in supplement.overrides {
        let lexeme = output
            .lexemes
            .iter_mut()
            .find(|lexeme| lexeme.id == replacement.owner)
            .ok_or_else(|| LoadError::UnknownOverrideOwner {
                path: path.into(),
                owner: replacement.owner.clone(),
            })?;
        let mut changed = false;
        for slot in &mut lexeme.forms {
            if slot.form == replacement.form
                && replacement
                    .number
                    .is_none_or(|number| slot.features.number == Some(number))
                && replacement
                    .person
                    .is_none_or(|person| slot.features.person == Some(person))
            {
                if !replaced.insert((replacement.owner.clone(), slot.form, slot.features.clone())) {
                    return Err(LoadError::OverlappingOverrides {
                        path: path.into(),
                        owner: replacement.owner,
                        form: slot.form,
                    });
                }
                if let Some(authored) = &slot.surfaces
                    && !authored
                        .iter()
                        .all(|surface| replacement.surfaces.contains(surface))
                {
                    return Err(LoadError::DiscardedSurfaces {
                        path: path.into(),
                        owner: replacement.owner,
                        form: slot.form,
                        authored: authored.clone(),
                    });
                }
                slot.surfaces = Some(replacement.surfaces.clone());
                changed = true;
            }
        }
        if !changed {
            return Err(LoadError::NoOverrideSlot {
                path: path.into(),
                owner: replacement.owner,
                form: replacement.form,
            });
        }
        lexeme.properties.features.insert(
            format!(
                "OverrideSource:{:?}:{:?}:{:?}",
                replacement.form, replacement.number, replacement.person
            ),
            path.to_string(),
        );
    }
    Ok(())
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn unresolved_override_is_rejected_by_the_source_boundary() {
        let supplement = Supplement {
            lexemes: Vec::new(),
            overrides: vec![Override {
                owner: "missing-owner".to_owned(),
                form: WordForm::Present,
                number: None,
                person: None,
                surfaces: vec!["missing".to_owned()],
            }],
        };
        let mut output = LexicalSources {
            lexemes: Vec::new(),
            unmapped: Vec::new(),
        };

        let error = apply_declarations("discovered-source", supplement, &mut output).unwrap_err();

        match error {
            LoadError::UnknownOverrideOwner { path, owner } => {
                assert_eq!(path, std::path::Path::new("discovered-source"));
                assert_eq!(owner, "missing-owner");
            }
            other => panic!("{other:?}"),
        }
    }
}
