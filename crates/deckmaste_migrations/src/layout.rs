use std::path::Path;
use std::path::PathBuf;

use deckmaste_core::plugin::ABILITY_WORDS_DIR;
use deckmaste_core::plugin::CARDS_DIR;
use deckmaste_core::plugin::KEYWORD_ABILITIES_DIR;
use deckmaste_core::plugin::KEYWORD_ACTIONS_DIR;
use deckmaste_core::plugin::MACROS_DIR;

pub struct PluginLayout(PathBuf);

impl PluginLayout {
    pub fn new(base: &Path) -> anyhow::Result<Self> {
        let base = base.canonicalize()?;
        if !base.exists() {
            return Err(anyhow::anyhow!(
                "Plugin base directory does not exist: {}",
                base.display()
            ));
        }
        if !base.is_dir() {
            return Err(anyhow::anyhow!(
                "Plugin base path is not a directory: {}",
                base.display()
            ));
        }
        Ok(Self(base.clone()))
    }

    fn dir(&self, path: &str) -> anyhow::Result<PathBuf> {
        let dir_path = self.0.join(path);
        std::fs::create_dir_all(&dir_path)?;
        let dir_path = dir_path.canonicalize()?;
        if !dir_path.starts_with(&self.0) {
            return Err(anyhow::anyhow!(
                "path is outside of plugin layout: {}",
                dir_path.display()
            ));
        }
        Ok(dir_path)
    }

    pub fn keyword_abilities_dir(&self) -> anyhow::Result<PathBuf> {
        self.dir(KEYWORD_ABILITIES_DIR)
    }

    pub fn keyword_actions_dir(&self) -> anyhow::Result<PathBuf> { self.dir(KEYWORD_ACTIONS_DIR) }

    pub fn ability_words_dir(&self) -> anyhow::Result<PathBuf> { self.dir(ABILITY_WORDS_DIR) }

    /// Where a category's subtype-definition macros live —
    /// `macros/types/<category>/` — under `macros/` since they are ordinary
    /// (meta-produced) macro definitions.
    pub fn subtype_macros_dir(&self, category: &str) -> anyhow::Result<PathBuf> {
        self.dir(&format!("{MACROS_DIR}/types/{category}"))
    }

    /// The sibling `builtin` plugin's subtype-macros dir for `category`, when
    /// distinct from this plugin (none if this layout *is* builtin, or no
    /// sibling exists). Mirrors the loader's prelude convention
    /// ([`Plugin::load_with_sibling_prelude`]): a sibling directory named
    /// `builtin` is the universal prelude. The generator consults it so it
    /// never writes a confers-LESS stub for a subtype that builtin already
    /// defines (with `confers:`) — a wizards stub would otherwise shadow
    /// builtin's def under "last plugin wins".
    ///
    /// Returns the path unconditionally (it need not exist on disk); callers
    /// probe individual `<ident>.ron` files under it.
    #[must_use]
    pub fn sibling_builtin_subtype_dir(&self, category: &str) -> Option<PathBuf> {
        let builtin = self.0.parent()?.join("builtin");
        // Don't treat ourselves as our own prelude.
        if builtin == self.0 {
            return None;
        }
        Some(builtin.join(format!("{MACROS_DIR}/types/{category}")))
    }

    pub fn cards_dir(&self) -> anyhow::Result<PathBuf> { self.dir(CARDS_DIR) }
}
