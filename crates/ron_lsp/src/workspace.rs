use std::collections::HashMap;
use std::fs;
use std::path::Path;
use std::path::PathBuf;

use lsp_types::Location as LspLocation;

use crate::convert;
use crate::source::Position;
use crate::source::identifiers;
use crate::source::position_at;

#[derive(Debug, Clone)]
pub struct Location {
    path: PathBuf,
    start: Position,
    end: Position,
}

impl Location {
    pub fn to_lsp(&self) -> Option<LspLocation> {
        Some(LspLocation {
            uri: convert::to_uri(&self.path)?,
            range: convert::to_lsp_range(self.start, self.end),
        })
    }
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum SymbolKind {
    Card,
    Macro,
    Keyword,
    AbilityWord,
    RustType,
}

#[derive(Debug)]
pub struct Symbol {
    pub name: String,
    pub container: String,
    pub kind: SymbolKind,
    pub location: Location,
    pub detail: Option<String>,
}

#[derive(Default)]
pub struct WorkspaceIndex {
    definitions: HashMap<String, Vec<Location>>,
    symbols: Vec<Symbol>,
    symbol_index: HashMap<String, usize>,
}

impl WorkspaceIndex {
    pub fn build(root: &Path) -> Self {
        let mut index = Self::default();
        let plugins = root.join("plugins");
        index.scan_cards(&plugins);
        index.scan_macros(&plugins);
        index.scan_named(&plugins, "keyword_abilities", SymbolKind::Keyword);
        index.scan_named(&plugins, "keyword_actions", SymbolKind::Keyword);
        index.scan_named(&plugins, "ability_words", SymbolKind::AbilityWord);
        index.scan_rust(&root.join("crates/deckmaste_core/src"));
        index
    }

    pub fn definitions(&self, name: &str) -> &[Location] {
        self.definitions.get(name).map_or(&[], Vec::as_slice)
    }

    pub fn symbol(&self, name: &str) -> Option<&Symbol> {
        self.symbol_index.get(name).map(|&i| &self.symbols[i])
    }

    /// Case-insensitive substring match against symbol name OR container, so a
    /// container query (e.g. `wizards/cards`) lists a plugin's contents.
    pub fn search(&self, query: &str) -> Vec<&Symbol> {
        let needle = query.to_lowercase();
        self.symbols
            .iter()
            .filter(|symbol| {
                symbol.name.to_lowercase().contains(&needle)
                    || symbol.container.to_lowercase().contains(&needle)
            })
            .collect()
    }

    /// Cards are indexed by filename stem only — no file reads — so this stays
    /// cheap across a plugin's tens of thousands of card files.
    fn scan_cards(&mut self, plugins: &Path) {
        for plugin in subdirs(plugins) {
            let Ok(entries) = fs::read_dir(plugin.join("cards")) else {
                continue;
            };
            for entry in entries.flatten() {
                let path = entry.path();
                let Some(name) = card_stem(&path) else {
                    continue;
                };
                let container = container_for(plugins, &path, "cards");
                self.push_symbol(name, container, SymbolKind::Card, top_location(&path), None);
            }
        }
    }

    fn scan_macros(&mut self, plugins: &Path) {
        for path in files_recursive(plugins, "ron") {
            if !path.components().any(|part| part.as_os_str() == "macros") {
                continue;
            }
            let Ok(text) = fs::read_to_string(&path) else {
                continue;
            };
            let Some((offset, name)) = macro_name(&text) else {
                continue;
            };
            let location = span_location(&path, &text, offset, name.len());
            let container = container_for(plugins, &path, "macros");
            let detail = macro_details(&text);
            self.push_symbol(
                name.to_owned(),
                container,
                SymbolKind::Macro,
                location,
                detail,
            );
        }
    }

    fn scan_named(&mut self, plugins: &Path, category: &str, kind: SymbolKind) {
        for plugin in subdirs(plugins) {
            for path in files_recursive(&plugin.join(category), "ron") {
                let Ok(text) = fs::read_to_string(&path) else {
                    continue;
                };
                let Some((offset, name)) = macro_name(&text) else {
                    continue;
                };
                let location = span_location(&path, &text, offset, name.len());
                let container = container_for(plugins, &path, category);
                self.push_symbol(name.to_owned(), container, kind, location, None);
            }
        }
    }

    fn scan_rust(&mut self, root: &Path) {
        for path in files_recursive(root, "rs") {
            let Ok(text) = fs::read_to_string(&path) else {
                continue;
            };
            for (offset, name) in rust_declarations(&text) {
                let location = span_location(&path, &text, offset, name.len());
                self.push_symbol(
                    name.to_owned(),
                    "deckmaste_core".to_owned(),
                    SymbolKind::RustType,
                    location,
                    None,
                );
            }
        }
    }

    fn push_symbol(
        &mut self,
        name: String,
        container: String,
        kind: SymbolKind,
        location: Location,
        detail: Option<String>,
    ) {
        self.definitions
            .entry(name.clone())
            .or_default()
            .push(location.clone());
        self.symbol_index.insert(name.clone(), self.symbols.len());
        self.symbols.push(Symbol {
            name,
            container,
            kind,
            location,
            detail,
        });
    }
}

fn span_location(path: &Path, text: &str, offset: usize, len: usize) -> Location {
    Location {
        path: path.to_owned(),
        start: position_at(text, offset),
        end: position_at(text, offset + len),
    }
}

fn top_location(path: &Path) -> Location {
    let top = Position {
        line: 0,
        character: 0,
    };
    Location {
        path: path.to_owned(),
        start: top,
        end: top,
    }
}

fn subdirs(root: &Path) -> Vec<PathBuf> {
    let Ok(entries) = fs::read_dir(root) else {
        return Vec::new();
    };
    let mut dirs: Vec<PathBuf> = entries
        .flatten()
        .map(|entry| entry.path())
        .filter(|path| path.is_dir())
        .collect();
    dirs.sort();
    dirs
}

/// Card name for `Foo.ron` / `Foo.ron.todo`; `None` for anything else.
fn card_stem(path: &Path) -> Option<String> {
    let name = path.file_name()?.to_str()?;
    let base = name
        .strip_suffix(".ron.todo")
        .or_else(|| name.strip_suffix(".ron"))?;
    Some(base.to_owned())
}

/// `"<plugin>/<category>"`, plugin taken from the first path component under
/// `plugins`.
fn container_for(plugins: &Path, path: &Path, category: &str) -> String {
    let plugin = path
        .strip_prefix(plugins)
        .ok()
        .and_then(|rel| rel.components().next())
        .map(|component| component.as_os_str().to_string_lossy().into_owned())
        .unwrap_or_default();
    format!("{plugin}/{category}")
}

/// Preformatted hover detail for a macro: its `template` and `kinds` fields.
fn macro_details(text: &str) -> Option<String> {
    let mut parts = Vec::new();
    if let Some(template) = string_field(text, "template") {
        parts.push(format!("template: {template}"));
    }
    if let Some(kinds) = bracket_field(text, "kinds") {
        parts.push(format!("kinds: {kinds}"));
    }
    (!parts.is_empty()).then(|| parts.join("\n"))
}

/// First `field: "…"` string value, ignoring strings and comments.
fn string_field<'a>(text: &'a str, field: &str) -> Option<&'a str> {
    for (offset, token) in identifiers(text) {
        if token != field {
            continue;
        }
        let tail = text.get(offset + token.len()..)?;
        let quote = tail.find('"')?;
        let rest = &tail[quote + 1..];
        let end = rest.find('"')?;
        return Some(&rest[..end]);
    }
    None
}

/// First `field: [ … ]` bracketed value (inclusive of the brackets).
fn bracket_field<'a>(text: &'a str, field: &str) -> Option<&'a str> {
    for (offset, token) in identifiers(text) {
        if token != field {
            continue;
        }
        let tail = text.get(offset + token.len()..)?;
        let open = tail.find('[')?;
        let mut depth = 0_usize;
        for (i, ch) in tail[open..].char_indices() {
            match ch {
                '[' => depth += 1,
                ']' => {
                    depth -= 1;
                    if depth == 0 {
                        return Some(&tail[open..=open + i]);
                    }
                }
                _ => {}
            }
        }
        return None;
    }
    None
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
struct TempTree {
    path: PathBuf,
}

#[cfg(test)]
impl TempTree {
    fn path(&self) -> &Path {
        &self.path
    }
}

#[cfg(test)]
impl Drop for TempTree {
    fn drop(&mut self) {
        let _ = fs::remove_dir_all(&self.path);
    }
}

/// Materialize `(relative_path, contents)` pairs under a unique temp dir.
/// Uniqueness comes from pid + a process-local counter — no clock, no RNG
/// (both are unavailable in this crate's test environment constraints).
#[cfg(test)]
fn tempdir_with(files: &[(&str, &str)]) -> TempTree {
    use std::sync::atomic::AtomicU32;
    use std::sync::atomic::Ordering;

    static COUNTER: AtomicU32 = AtomicU32::new(0);
    let n = COUNTER.fetch_add(1, Ordering::Relaxed);
    let root = std::env::temp_dir().join(format!("ron_lsp_test_{}_{n}", std::process::id()));
    for (relative, contents) in files {
        let path = root.join(relative);
        fs::create_dir_all(path.parent().unwrap()).unwrap();
        fs::write(&path, contents).unwrap();
    }
    TempTree { path: root }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn indexes_cards_by_filename_with_container() {
        let dir = tempdir_with(&[
            (
                "plugins/wizards/cards/Quillspike.ron.todo",
                "Normal(name: \"Quillspike\")",
            ),
            (
                "plugins/builtin/macros/cost/SacrificeThis.ron",
                "(name: \"SacrificeThis\", template: \"Sacrifice this permanent\", kinds: [CostComponent])",
            ),
        ]);
        let index = WorkspaceIndex::build(dir.path());

        let card = index.symbol("Quillspike").expect("card symbol");
        assert_eq!(card.name, "Quillspike");
        assert_eq!(card.kind, SymbolKind::Card);
        assert_eq!(card.container, "wizards/cards");
        assert!(card.location.to_lsp().is_some());

        let mac = index.symbol("SacrificeThis").expect("macro symbol");
        assert_eq!(mac.kind, SymbolKind::Macro);
        assert_eq!(mac.container, "builtin/macros");
        assert!(
            mac.detail
                .as_deref()
                .unwrap()
                .contains("Sacrifice this permanent")
        );
    }

    #[test]
    fn workspace_search_matches_name_and_container() {
        let dir = tempdir_with(&[
            (
                "plugins/wizards/cards/Quillspike.ron.todo",
                "Normal(name: \"Quillspike\")",
            ),
            (
                "plugins/wizards/cards/Crawlspace.ron.todo",
                "Normal(name: \"Crawlspace\")",
            ),
        ]);
        let index = WorkspaceIndex::build(dir.path());

        let by_name: Vec<_> = index
            .search("quill")
            .iter()
            .map(|symbol| symbol.name.as_str())
            .collect();
        assert_eq!(by_name, ["Quillspike"]);

        // A container query lists every card in the plugin.
        assert_eq!(index.search("wizards/cards").len(), 2);
    }

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
