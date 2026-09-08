use std::collections::BTreeSet;
use std::io::BufRead;

use anyhow::Context;
use deckmaste_data::scryfall::OracleCardReader;

pub(crate) fn extract(oracle_cards: impl BufRead) -> anyhow::Result<BTreeSet<String>> {
    let mut names = BTreeSet::new();

    for card in OracleCardReader::new(oracle_cards) {
        let card = card.context("parsing Scryfall Oracle Cards JSONL")?;
        if !card.vintage_playable() {
            continue;
        }
        for unit in card.oracle_units() {
            let selected = unit.display_name();
            anyhow::ensure!(
                !selected.contains(" // "),
                "card-names: composite name without a selected face: {selected:?}"
            );
            names.insert(selected.to_owned());
        }
    }

    Ok(names)
}
