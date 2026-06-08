/// Filename for a subtype: capitalize the first character and strip
/// non-alphanumerics, without splitting words (e.g. "Power-Plant" ->
/// "`PowerPlant`", "Urza's" -> "Urzas").
fn type_filename(name: &str) -> String {
    let mut chars = name.chars();
    chars
        .next()
        .into_iter()
        .flat_map(char::to_uppercase)
        .chain(chars)
        .filter(char::is_ascii_alphanumeric)
        .collect()
}

pub(super) fn generate(plugin: &super::PluginLayout) -> anyhow::Result<()> {
    // The catalog files are named "<category>-types.json".
    let categories: [(&str, &str); 7] = [
        ("artifact", "Artifact"),
        ("battle", "Battle"),
        ("creature", "Creature"),
        ("enchantment", "Enchantment"),
        ("land", "Land"),
        ("planeswalker", "Planeswalker"),
        ("spell", "Spell"),
    ];

    for (category, prefix) in categories {
        let catalog_bytes = crate::data::scryfall::catalog_bytes(&format!("{category}-types"))?;
        let catalog = crate::data::scryfall::Catalog::parse(&catalog_bytes)?;
        let dest_dir = plugin.types_dir(category)?;
        for subtype in &catalog.data {
            // Subtypes are written final, not as stubs: skip once the
            // definition exists (generated here or hand-edited).
            let dest = dest_dir.join(format!("{}.ron", type_filename(subtype)));
            if !super::is_unimplemented(&dest) {
                continue;
            }

            std::fs::write(&dest, format!("{prefix}Type(\"{subtype}\")\n"))?;
            eprintln!("wrote {}", dest.display());
        }
    }

    Ok(())
}

#[cfg(test)]
mod tests {
    use super::type_filename;

    #[test]
    fn type_filenames() {
        assert_eq!(type_filename("Advisor"), "Advisor");
        assert_eq!(type_filename("Power-Plant"), "PowerPlant");
        assert_eq!(type_filename("Urza's"), "Urzas");
        assert_eq!(type_filename("Time Lord"), "TimeLord");
    }
}
