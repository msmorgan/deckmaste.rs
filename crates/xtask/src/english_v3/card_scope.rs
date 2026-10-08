use std::collections::BTreeSet;
use std::path::Path;

use anyhow::Result;
use deckmaste_lexical::LexemeId;

use crate::raw_corpus::{SelectedFace, SelectionRequest, load_selected};

pub(super) fn for_face(face: &SelectedFace) -> BTreeSet<LexemeId> {
    face.card_names
        .iter()
        .map(|name| LexemeId::new(&format!("catalog:card-names.txt/{name}")))
        .collect()
}

pub(super) fn for_card(data: &Path, name: &str) -> Result<BTreeSet<LexemeId>> {
    let selected = load_selected(
        data,
        &SelectionRequest {
            card_names: BTreeSet::from([name.into()]),
            ..SelectionRequest::default()
        },
        None,
    )?;
    Ok(selected.faces.iter().flat_map(for_face).collect())
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn probe_and_face_selection_share_all_units_of_the_card() {
        let data =
            Path::new(env!("CARGO_MANIFEST_DIR")).join("../../data/scryfall/oracle-cards.jsonl");
        let name = "Garruk Relentless";
        let selected = load_selected(
            &data,
            &SelectionRequest {
                card_names: BTreeSet::from([name.into()]),
                ..SelectionRequest::default()
            },
            None,
        )
        .unwrap();
        let expected = BTreeSet::from([
            LexemeId::new("catalog:card-names.txt/Garruk Relentless"),
            LexemeId::new("catalog:card-names.txt/Garruk, the Veil-Cursed"),
        ]);
        assert_eq!(selected.faces.len(), 1);
        assert_eq!(for_face(&selected.faces[0]), expected);
        assert_eq!(for_card(&data, name).unwrap(), expected);
        assert_eq!(
            for_card(&data, "Garruk, the Veil-Cursed").unwrap(),
            expected
        );
    }
}
