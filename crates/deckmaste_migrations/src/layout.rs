use std::path::{Path, PathBuf};

use deckmaste_core::plugin::{
    ABILITY_WORDS_DIR, CARDS_DIR, KEYWORD_ABILITIES_DIR, KEYWORD_ACTIONS_DIR, TYPES_DIR,
};

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

    pub fn types_dir(&self, category: &str) -> anyhow::Result<PathBuf> {
        self.dir(&format!("{TYPES_DIR}/{category}"))
    }

    pub fn cards_dir(&self) -> anyhow::Result<PathBuf> { self.dir(CARDS_DIR) }
}
