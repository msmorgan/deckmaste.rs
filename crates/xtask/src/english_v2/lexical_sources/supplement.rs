use std::collections::BTreeSet;
use std::fs;
use std::path::Path;

use anyhow::Context;
use deckmaste_lexical::Lexeme;
use deckmaste_lexical::Number;
use deckmaste_lexical::Person;
use deckmaste_lexical::WordForm;
use serde::Deserialize;

use crate::english_v2::lexical_sources::LexicalSources;

const PATH: &str = "crates/deckmaste_english_v2/src/lexical_supplement.ron";

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

pub(super) fn apply(root: &Path, output: &mut LexicalSources) -> anyhow::Result<()> {
    let source = fs::read_to_string(root.join(PATH))?;
    let supplement: Supplement =
        ron::from_str(&source).context("reading supplemental lexical declarations")?;
    output.lexemes.extend(supplement.lexemes);
    let mut replaced = BTreeSet::new();
    for replacement in supplement.overrides {
        let lexeme = output
            .lexemes
            .iter_mut()
            .find(|lexeme| lexeme.id == replacement.owner)
            .with_context(|| format!("{PATH}: unknown override owner {}", replacement.owner))?;
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
                    "{PATH}: overlapping overrides for {} {:?}",
                    replacement.owner,
                    slot.form
                );
                if let Some(authored) = &slot.surfaces {
                    anyhow::ensure!(
                        authored
                            .iter()
                            .all(|surface| replacement.surfaces.contains(surface)),
                        "{PATH}: override for {} {:?} discards an authored surface {authored:?}; reconcile its source",
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
            "{PATH}: no slot selected for {} {:?}",
            replacement.owner,
            replacement.form
        );
        lexeme.properties.features.insert(
            format!(
                "OverrideSource:{:?}:{:?}:{:?}",
                replacement.form, replacement.number, replacement.person
            ),
            PATH.into(),
        );
    }
    Ok(())
}
