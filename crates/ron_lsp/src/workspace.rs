use std::collections::HashMap;
use std::fs;
use std::path::Path;
use std::path::PathBuf;

use serde_json::Value;
use serde_json::json;

use crate::source::Position;
use crate::source::identifiers;
use crate::source::position_at;

#[derive(Debug)]
pub struct Location {
    path: PathBuf,
    start: Position,
    end: Position,
}

impl Location {
    pub fn to_json(&self) -> Value {
        let uri = format!("file://{}", self.path.to_string_lossy().replace(' ', "%20"));
        json!({
            "uri": uri,
            "range": {
                "start": { "line": self.start.line, "character": self.start.character },
                "end": { "line": self.end.line, "character": self.end.character }
            }
        })
    }
}

#[derive(Default)]
pub struct WorkspaceIndex {
    definitions: HashMap<String, Vec<Location>>,
}

impl WorkspaceIndex {
    pub fn build(root: &Path) -> Self {
        let mut index = Self::default();
        index.scan_macros(&root.join("plugins"));
        index.scan_rust(&root.join("crates/deckmaste_core/src"));
        index
    }

    pub fn definitions(&self, name: &str) -> &[Location] {
        self.definitions.get(name).map_or(&[], Vec::as_slice)
    }

    fn scan_macros(&mut self, root: &Path) {
        for path in files_recursive(root, "ron") {
            if !path.components().any(|part| part.as_os_str() == "macros") {
                continue;
            }
            let Ok(text) = fs::read_to_string(&path) else {
                continue;
            };
            if let Some((offset, name)) = macro_name(&text) {
                self.insert(name, &path, &text, offset);
            }
        }
    }

    fn scan_rust(&mut self, root: &Path) {
        for path in files_recursive(root, "rs") {
            let Ok(text) = fs::read_to_string(&path) else {
                continue;
            };
            for (offset, name) in rust_declarations(&text) {
                self.insert(name, &path, &text, offset);
            }
        }
    }

    fn insert(&mut self, name: &str, path: &Path, text: &str, offset: usize) {
        let start = position_at(text, offset);
        let end = position_at(text, offset + name.len());
        self.definitions
            .entry(name.to_owned())
            .or_default()
            .push(Location {
                path: path.to_owned(),
                start,
                end,
            });
    }
}

fn macro_name(text: &str) -> Option<(usize, &str)> {
    for (offset, token) in identifiers(text) {
        if token != "name" {
            continue;
        }
        let tail = text.get(offset + token.len()..)?;
        let quote = tail.find('"')?;
        let value_start = offset + token.len() + quote + 1;
        let value_end = value_start + text[value_start..].find('"')?;
        return Some((value_start, &text[value_start..value_end]));
    }
    None
}

fn rust_declarations(text: &str) -> Vec<(usize, &str)> {
    let tokens: Vec<_> = identifiers(text).collect();
    let mut declarations = Vec::new();
    for window in tokens.windows(2) {
        let [(keyword_offset, keyword), (name_offset, name)] = window else {
            continue;
        };
        if !matches!(*keyword, "enum" | "struct") {
            continue;
        }
        let between = &text[keyword_offset + keyword.len()..*name_offset];
        if between.chars().all(char::is_whitespace) {
            declarations.push((*name_offset, *name));
        }
    }

    // Enum variants are declarations too. Restrict this lightweight scan to
    // the first identifier on a line inside a public enum body; that excludes
    // payload types and nearly all expression-level uses.
    let mut enum_depth = None;
    let mut depth = 0_usize;
    let mut offset = 0;
    for line in text.split_inclusive('\n') {
        let trimmed = line.trim_start();
        if enum_depth.is_none()
            && (trimmed.starts_with("pub enum ") || trimmed.starts_with("pub(crate) enum "))
        {
            enum_depth = Some(depth + 1);
        } else if enum_depth.is_some_and(|start| depth >= start)
            && let Some((relative, name)) = identifiers(line).next()
            && name.chars().next().is_some_and(char::is_uppercase)
            && !matches!(name, "Self" | "Box" | "Vec" | "Option")
        {
            declarations.push((offset + relative, name));
        }
        depth += line.chars().filter(|ch| *ch == '{').count();
        depth = depth.saturating_sub(line.chars().filter(|ch| *ch == '}').count());
        if enum_depth.is_some_and(|start| depth < start) {
            enum_depth = None;
        }
        offset += line.len();
    }
    declarations
}

fn files_recursive(root: &Path, extension: &str) -> Vec<PathBuf> {
    let mut pending = vec![root.to_owned()];
    let mut files = Vec::new();
    while let Some(dir) = pending.pop() {
        let Ok(entries) = fs::read_dir(dir) else {
            continue;
        };
        for entry in entries.flatten() {
            let path = entry.path();
            if path.is_dir() {
                pending.push(path);
            } else if path.extension().is_some_and(|ext| ext == extension) {
                files.push(path);
            }
        }
    }
    files.sort();
    files
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn locates_macro_name() {
        let text = "// name: \"fake\"\n(name: \"AnyTarget\", kinds: [Predicate], body: X)";
        let (offset, name) = macro_name(text).unwrap();
        assert_eq!(name, "AnyTarget");
        assert_eq!(&text[offset..offset + name.len()], name);
    }

    #[test]
    fn locates_rust_types_and_enum_variants() {
        let text = "pub struct Each { x: Thing }\npub enum Reference {\n    This,\n    It,\n    Target(u8),\n}\n";
        let names: Vec<_> = rust_declarations(text)
            .into_iter()
            .map(|(_, name)| name)
            .collect();
        assert_eq!(names, ["Each", "Reference", "This", "It", "Target"]);
    }
}
