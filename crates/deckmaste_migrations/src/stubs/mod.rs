//! Generate a plugin's keyword-ability / keyword-action / ability-word macro
//! stubs and subtype meta-macro invocations. (Formerly the `_000`–`_003`
//! migrations.)

use std::path::Path;

use crate::layout::PluginLayout;

mod ability_word_todos;
mod keyword_ability_todos;
mod keyword_action_todos;
mod keyword_todos;
mod subtypes;

/// Whether a definition still needs generating, keyed on its finished `.ron`
/// path: a stub writer (re)generates its stub only while the finished `.ron`
/// doesn't exist.
fn is_unimplemented(final_path: &Path) -> bool {
    !final_path.exists()
}

/// Generate every macro stub + subtype meta-invocation into `plugin_dir`.
///
/// # Errors
/// If the plugin layout is unusable or any generator fails.
pub fn generate_stubs(plugin_dir: &Path) -> anyhow::Result<()> {
    let plugin = PluginLayout::new(plugin_dir)?;
    keyword_ability_todos::generate(&plugin)?;
    keyword_action_todos::generate(&plugin)?;
    ability_word_todos::generate(&plugin)?;
    subtypes::generate(&plugin)?;
    Ok(())
}

/// Generate only subtype meta-invocations, skipping keyword/action/ability-word
/// todo stubs. Does not require `data/rules/cr.json`.
///
/// # Errors
/// If the plugin layout is unusable or the subtype generator fails.
pub fn generate_subtype_stubs(plugin_dir: &Path) -> anyhow::Result<()> {
    let plugin = PluginLayout::new(plugin_dir)?;
    subtypes::generate(&plugin)?;
    Ok(())
}

#[cfg(test)]
mod tests {
    use super::is_unimplemented;

    #[test]
    fn unimplemented_tracks_finished_ron() {
        let dir = tempfile::tempdir().unwrap();
        let final_path = dir.path().join("Serra Angel.ron");
        // Nothing on disk: needs a stub.
        assert!(is_unimplemented(&final_path));
        // A finished card: implemented.
        std::fs::write(&final_path, "x").unwrap();
        assert!(!is_unimplemented(&final_path));
    }
}
