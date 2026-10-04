use crate::CatalogError;
use std::collections::BTreeSet;
use std::io::BufRead;

use deckmaste_data::scryfall::OracleCardReader;

pub(crate) fn extract(oracle_cards: impl BufRead) -> Result<BTreeSet<String>, CatalogError> {
    let mut names = BTreeSet::new();

    for card in OracleCardReader::new(oracle_cards) {
        let card = card?;
        if !card.supported() {
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

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn names_from_funny_sets_are_excluded_even_when_vintage_legal() {
        let records = concat!(
            r#"{"object":"card","id":"normal-p","oracle_id":"normal-o","name":"Normal","layout":"normal","set_type":"expansion","legalities":{"vintage":"legal"}}"#,
            "\n",
            r#"{"object":"card","id":"funny-p","oracle_id":"funny-o","name":"Funny","layout":"normal","set_type":"funny","legalities":{"vintage":"legal"}}"#,
        );
        assert_eq!(
            extract(records.as_bytes()).unwrap(),
            BTreeSet::from(["Normal".into()])
        );
    }
}
