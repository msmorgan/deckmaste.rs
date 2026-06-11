//! The committed `cr-citations.lock`: rule → content checksum.

use std::collections::BTreeMap;
use std::path::Path;

use anyhow::Context;
use serde::Deserialize;
use serde::Serialize;

#[derive(Debug, Clone, Default, Serialize, Deserialize)]
pub struct Lockfile {
    pub cr_date: String,
    pub wizards_url: String,
    pub checksums: BTreeMap<String, String>,
}

impl Lockfile {
    pub fn to_toml(&self) -> anyhow::Result<String> {
        toml::to_string_pretty(self).context("serializing lockfile")
    }

    pub fn from_toml(text: &str) -> anyhow::Result<Lockfile> {
        toml::from_str(text).context("parsing lockfile")
    }

    pub fn load(path: &Path) -> anyhow::Result<Lockfile> {
        let text =
            std::fs::read_to_string(path).with_context(|| format!("reading {}", path.display()))?;
        Lockfile::from_toml(&text)
    }

    pub fn save(&self, path: &Path) -> anyhow::Result<()> {
        std::fs::write(path, self.to_toml()?).with_context(|| format!("writing {}", path.display()))
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn round_trips_through_toml() {
        let mut lock = Lockfile {
            cr_date: "2026-04-17".into(),
            wizards_url: "https://example.test/cr.txt".into(),
            checksums: BTreeMap::new(),
        };
        lock.checksums
            .insert("702.158b".into(), "a1b2c3d4e5f6a7b8".into());
        lock.checksums
            .insert("602".into(), "0011223344556677".into());

        let text = lock.to_toml().unwrap();
        // Dotted rule numbers must be quoted keys under [checksums].
        assert!(text.contains("\"702.158b\" = \"a1b2c3d4e5f6a7b8\""));

        let back = Lockfile::from_toml(&text).unwrap();
        assert_eq!(back.cr_date, lock.cr_date);
        assert_eq!(back.checksums, lock.checksums);
    }
}
