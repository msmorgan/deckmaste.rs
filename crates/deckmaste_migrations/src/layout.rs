use std::path::{Path, PathBuf};

pub struct PluginLayout(PathBuf);

impl PluginLayout {
    pub fn new(base: &Path) -> anyhow::Result<Self> {
        let base = base.canonicalize()?;
        if !base.exists() {
            return Err(anyhow::anyhow!("Plugin base directory does not exist: {}", base.display()));
        }
        if !base.is_dir() {
            return Err(anyhow::anyhow!("Plugin base path is not a directory: {}", base.display()));
        }
        Ok(Self(base.to_owned()))
    }

    fn file(&self, path: &str) -> anyhow::Result<PathBuf> {
        let file_path = self.0.join(path);
        let file_name = file_path.file_name().unwrap();
        std::fs::create_dir_all(file_path.parent().unwrap())?;
        let parent_dir = file_path.parent().unwrap().canonicalize()?;
        if !parent_dir.starts_with(&self.0) {
            return Err(anyhow::anyhow!("path is outside of plugin layout: {}", file_path.display()));
        }
        Ok(parent_dir.join(file_name))
    }

    fn dir(&self, path: &str) -> anyhow::Result<PathBuf> {
        let dir_path = self.0.join(path);
        std::fs::create_dir_all(&dir_path)?;
        let dir_path = dir_path.canonicalize()?;
        if !dir_path.starts_with(&self.0) {
            return Err(anyhow::anyhow!("path is outside of plugin layout: {}", dir_path.display()));
        }
        Ok(dir_path)
    }

    pub fn keyword_abilities_dir(&self) -> anyhow::Result<PathBuf> {
        self.dir("keyword_abilities")
    }

    pub fn keyword_actions_dir(&self) -> anyhow::Result<PathBuf> {
        self.dir("keyword_actions")
    }

    pub fn ability_words_dir(&self) -> anyhow::Result<PathBuf> {
        self.dir("ability_words")
    }

    pub fn keyword_abilities_file(&self) -> anyhow::Result<PathBuf> {
        self.file("keyword_abilities.ron")
    }
}