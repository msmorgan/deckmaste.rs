//! Generate a plugin's subtype meta-macro invocations. (Formerly the `_000`–
//! `_003` migrations; the keyword-ability / keyword-action / ability-word todo
//! stubs those once emitted were never consumed and have been retired.)

use std::path::Path;

use crate::layout::PluginLayout;

mod subtypes;

/// Whether a definition still needs generating, keyed on its finished `.ron`
/// path: a stub writer (re)generates its stub only while the finished `.ron`
/// doesn't exist.
fn is_unimplemented(final_path: &Path) -> bool {
    !final_path.exists()
}

/// Generate the plugin's subtype meta-macro invocations into `plugin_dir`.
/// Does not require `data/rules/cr.json`.
///
/// # Errors
/// If the plugin layout is unusable or the subtype generator fails.
pub fn generate_stubs(plugin_dir: &Path) -> anyhow::Result<()> {
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
