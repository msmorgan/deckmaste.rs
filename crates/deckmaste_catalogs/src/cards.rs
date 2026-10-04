use crate::CatalogError;
use std::collections::BTreeSet;
use std::io::BufRead;

use deckmaste_data::scryfall::OracleCardReader;

pub(crate) fn extract(oracle_cards: impl BufRead) -> Result<BTreeSet<String>, CatalogError> {
    let mut names = BTreeSet::new();

    for card in OracleCardReader::new(oracle_cards) {
        let card = card?;
        if !card.vintage_playable() {
            continue;
        }
        for unit in card.oracle_units() {
            let selected = unit.display_name();
            if selected.contains(" // ") {
                return Err(CatalogError::CompositeCardName {
                    name: selected.into(),
                });
            }
            names.insert(selected.to_owned());
        }
    }

    Ok(names)
}
