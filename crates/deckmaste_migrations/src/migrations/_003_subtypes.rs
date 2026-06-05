pub(super) struct Subtypes;

/// Filename for a subtype: capitalize the first character and strip
/// non-alphanumerics, without splitting words (e.g. "Power-Plant" ->
/// "PowerPlant", "Urza's" -> "Urzas").
fn type_filename(name: &str) -> String {
    let mut chars = name.chars();
    chars
        .next()
        .into_iter()
        .flat_map(|first| first.to_uppercase())
        .chain(chars)
        .filter(char::is_ascii_alphanumeric)
        .collect()
}

impl super::Migration for Subtypes {
    fn apply(&self, plugin: &super::PluginLayout) -> anyhow::Result<()> {
        let categories: [(&str, &str, fn() -> anyhow::Result<Vec<String>>); 7] = [
            ("artifact", "Artifact", crate::data::artifact_types),
            ("battle", "Battle", crate::data::battle_types),
            ("creature", "Creature", crate::data::creature_types),
            ("enchantment", "Enchantment", crate::data::enchantment_types),
            ("land", "Land", crate::data::land_types),
            ("planeswalker", "Planeswalker", crate::data::planeswalker_types),
            ("spell", "Spell", crate::data::spell_types),
        ];

        for (category, prefix, types) in categories {
            let dest_dir = plugin.types_dir(category)?;
            for subtype in types()? {
                let dest = dest_dir.join(format!("{}.ron", type_filename(&subtype)));
                if !super::is_todo(&dest)? {
                    continue;
                }

                std::fs::write(&dest, format!("{prefix}Type(\"{subtype}\")\n"))?;
                eprintln!("wrote {}", dest.display());
            }
        }

        Ok(())
    }
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
