use std::collections::BTreeSet;
use std::path::Path;

use anyhow::Context;
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

pub(crate) fn apply(root: &Path, output: &mut LexicalSources) -> anyhow::Result<()> {
    let mut supplements = super::ron_documents(root)?
        .into_iter()
        .filter_map(|(path, source)| {
            ron::from_str::<Supplement>(&source)
                .ok()
                .map(|supplement| (path, supplement))
        });
    let (path, supplement) = supplements
        .next()
        .context("no supplemental lexical declarations found in the transitional source tree")?;
    anyhow::ensure!(
        supplements.next().is_none(),
        "multiple supplemental lexical declarations found in the transitional source tree"
    );
    let path = path.strip_prefix(root).unwrap_or(&path).to_string_lossy();
    apply_declarations(&path, supplement, output)
}

fn apply_declarations(
    path: &str,
    supplement: Supplement,
    output: &mut LexicalSources,
) -> anyhow::Result<()> {
    output.lexemes.extend(supplement.lexemes);
    let mut replaced = BTreeSet::new();
    for replacement in supplement.overrides {
        let lexeme = output
            .lexemes
            .iter_mut()
            .find(|lexeme| lexeme.id == replacement.owner)
            .with_context(|| format!("{path}: unknown override owner {}", replacement.owner))?;
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
                anyhow::ensure!(
                    replaced.insert((replacement.owner.clone(), slot.form, slot.features.clone())),
                    "{path}: overlapping overrides for {} {:?}",
                    replacement.owner,
                    slot.form
                );
                if let Some(authored) = &slot.surfaces {
                    anyhow::ensure!(
                        authored
                            .iter()
                            .all(|surface| replacement.surfaces.contains(surface)),
                        "{path}: override for {} {:?} discards an authored surface {authored:?}; reconcile its source",
                        replacement.owner,
                        slot.form
                    );
                }
                slot.surfaces = Some(replacement.surfaces.clone());
                changed = true;
            }
        }
        anyhow::ensure!(
            changed,
            "{path}: no slot selected for {} {:?}",
            replacement.owner,
            replacement.form
        );
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

        assert!(
            error
                .to_string()
                .contains("discovered-source: unknown override owner missing-owner")
        );
    }
}
