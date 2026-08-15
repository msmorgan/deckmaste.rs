use std::collections::BTreeSet;

use anyhow::Context;
use deckmaste_data::mtgjson::AtomicCards;

pub(crate) fn extract(atomic: &[u8]) -> anyhow::Result<BTreeSet<String>> {
    let atomic = AtomicCards::parse(atomic).context("parsing AtomicCards.json")?;
    let mut names = BTreeSet::new();

    for card in atomic
        .data
        .values()
        .flatten()
        .filter(|card| card.vintage_playable())
    {
        let selected = card.face_name.as_deref().unwrap_or(card.name.as_str());
        anyhow::ensure!(
            !selected.contains(" // "),
            "card-names: composite name without faceName: {selected:?}"
        );
        names.insert(selected.to_owned());
    }

    Ok(names)
}
