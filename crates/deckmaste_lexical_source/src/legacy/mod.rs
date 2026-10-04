//! Readers for authored lexical inventories and shared plugin declarations.
//!
//! The public interface returns normalized lexical declarations.

use std::fs;
use std::path::Path;
use std::path::PathBuf;

use crate::LoadError;

pub(crate) mod core;
pub(crate) mod plugins;

const LEXICAL_SOURCE_DIR: &str = "crates/deckmaste_lexical_source/lexicon";

pub(crate) fn ron_documents(root: &Path) -> Result<Vec<(PathBuf, String)>, LoadError> {
    let directory = root.join(LEXICAL_SOURCE_DIR);
    let mut paths = Vec::new();
    collect_ron_documents(&directory, &mut paths)?;
    paths.sort();
    paths
        .into_iter()
        .map(|path| {
            let source = crate::error::read(&path)?;
            Ok((path, source))
        })
        .collect()
}

fn collect_ron_documents(directory: &Path, paths: &mut Vec<PathBuf>) -> Result<(), LoadError> {
    let entries = fs::read_dir(directory).map_err(|source| LoadError::Io {
        operation: "reading source directory",
        path: directory.into(),
        source,
    })?;
    for entry in entries {
        let entry = entry.map_err(|source| LoadError::Io {
            operation: "reading directory entry",
            path: directory.into(),
            source,
        })?;
        let path = entry.path();
        let file_type = entry.file_type().map_err(|source| LoadError::Io {
            operation: "reading source type",
            path: path.clone(),
            source,
        })?;
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
