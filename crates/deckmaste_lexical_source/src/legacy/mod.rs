//! Transitional readers for declarations being salvaged from English v2.
//!
//! Nothing outside this crate observes these source formats. The public
//! interface returns normalized lexical declarations.

use std::fs;
use std::path::Path;
use std::path::PathBuf;

use anyhow::Context;

pub(crate) mod core;
pub(crate) mod plugins;

const ENGLISH_V2_SOURCE_DIR: &str = "crates/deckmaste_english_v2/src";

pub(crate) fn ron_documents(root: &Path) -> anyhow::Result<Vec<(PathBuf, String)>> {
    let directory = root.join(ENGLISH_V2_SOURCE_DIR);
    let mut paths = Vec::new();
    collect_ron_documents(&directory, &mut paths)?;
    paths.sort();
    paths
        .into_iter()
        .map(|path| {
            let source = fs::read_to_string(&path)
                .with_context(|| format!("reading transitional document {}", path.display()))?;
            Ok((path, source))
        })
        .collect()
}

fn collect_ron_documents(directory: &Path, paths: &mut Vec<PathBuf>) -> anyhow::Result<()> {
    let entries = fs::read_dir(directory).with_context(|| {
        format!(
            "reading transitional source directory {}",
            directory.display()
        )
    })?;
    for entry in entries {
        let entry =
            entry.with_context(|| format!("reading an entry in {}", directory.display()))?;
        let path = entry.path();
        let file_type = entry
            .file_type()
            .with_context(|| format!("reading source type for {}", path.display()))?;
        if file_type.is_dir() {
            collect_ron_documents(&path, paths)?;
        } else if file_type.is_file()
            && path.extension().is_some_and(|extension| extension == "ron")
        {
            paths.push(path);
        }
    }
    Ok(())
}
