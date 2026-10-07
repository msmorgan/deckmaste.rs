use std::collections::BTreeMap;
use std::path::Path;

use deckmaste_catalogs::CardNameNicknames;

use crate::LoadError;

pub(crate) fn load(root: &Path) -> Result<BTreeMap<String, String>, LoadError> {
    let catalog = CardNameNicknames::load(
        &root.join("data/gen/catalogs"),
        &root.join("data/scryfall/oracle-cards.jsonl"),
    )?;
    Ok(catalog.nicknames().clone())
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn generated_nicknames_equal_the_streamed_computation() {
        let root = Path::new(env!("CARGO_MANIFEST_DIR")).join("../..");
        let bytes = std::fs::read(root.join("data/scryfall/oracle-cards.jsonl")).unwrap();
        let generated = CardNameNicknames::generate(&bytes).unwrap();
        assert_eq!(generated.nicknames(), &load(&root).unwrap());
    }
}
