//! Subtype definitions for a plugin: one `<Ident>.ron` per catalog subtype
//! under `macros/types/<category>/`, written final (not as stubs) as a
//! meta-macro invocation — `name:` the registration ident, `template:` the
//! printed name.

/// Filename for a subtype: capitalize the first character and strip
/// non-alphanumerics, without splitting words (e.g. "Power-Plant" ->
/// "`PowerPlant`", "Urza's" -> "Urzas"). Doubles as the macro's registration
/// `name`, so it must stay a bare-invocable identifier.
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
        let dest_dir = plugin.subtype_macros_dir(category)?;
        // The sibling `builtin` plugin (the universal prelude) may already
        // define this category's subtypes — e.g. Aura/Equipment/Fortification
        // carry an Innate `confers:` that a bare confers-LESS stub here would
        // shadow under "last plugin wins". Skip any subtype builtin provides
        // so its def is the one in scope for this plugin's cards.
        let builtin_dir = plugin.sibling_builtin_subtype_dir(category);
        let mut idents: std::collections::HashMap<String, String> =
            std::collections::HashMap::new();
        for subtype in &catalog.data {
            let ident = type_filename(subtype);
            // Two printed names must not chop to one ident: the second
            // would silently skip behind `is_unimplemented`.
            if let Some(previous) = idents.insert(ident.clone(), subtype.to_string()) {
                anyhow::bail!(
                    "subtype idents collide: {previous:?} and {subtype:?} both produce `{ident}`"
                );
            }
            // Defined by the builtin prelude already (with its `confers:`):
            // don't emit a stub that would override it.
            if builtin_dir
                .as_ref()
                .is_some_and(|dir| dir.join(format!("{ident}.ron")).exists())
            {
                continue;
            }
            // Subtypes are written final, not as stubs: skip once the
            // definition exists (generated here or hand-edited).
            let dest = dest_dir.join(format!("{ident}.ron"));
            if !super::is_unimplemented(&dest) {
                continue;
            }
            // `name` is the registration ident (what cards invoke);
            // `template` is the printed name — inert metadata today,
            // reserved for rules-text rendering, and omitted when it equals
            // the ident (the meta-macro defaults it to `name`). Quotable
            // as-is: catalog names carry only letters, apostrophes, spaces,
            // and hyphens, none of which need escaping in a RON string.
            let invocation = if *ident == **subtype {
                format!("{prefix}Type(name: \"{ident}\")\n")
            } else {
                format!("{prefix}Type(name: \"{ident}\", template: \"{subtype}\")\n")
            };
            std::fs::write(&dest, invocation)?;
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
