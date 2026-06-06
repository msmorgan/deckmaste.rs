//! Validating a plugin's finished cards through the macro-aware reader.
//! This is the validation layer the template-based migrations rely on:
//! their output is checked by the real reader, not by a write schema.
//!
//! Planned: a lint pass over the [`Card`]s that parse, for shapes that
//! read fine but are always authoring mistakes. First candidates are the
//! degenerate sequences, once `Effect::Sequence(Vec<Effect>)` lands —
//! `Sequence([])` does nothing and `Sequence([x])` is just `x`.

use std::path::{Path, PathBuf};

use deckmaste_core::Card;
use deckmaste_core::plugin::{CARDS_DIR, is_todo_source};

use crate::plugin::{Plugin, read, ron_files_recursive};

/// A card file that failed to read as a [`Card`].
pub struct InvalidCard {
    pub path: PathBuf,
    pub error: ron::error::SpannedError,
}

/// What a validation pass saw: todos are skipped, everything else either
/// parsed (`valid`) or landed in `failures`.
pub struct Validation {
    pub valid: usize,
    pub todos: usize,
    pub failures: Vec<InvalidCard>,
}

/// Reads every non-todo `cards/**/*.ron` in the plugin — builtin sibling
/// prelude in scope — as a [`Card`], collecting failures instead of
/// stopping at the first. Todo detection delegates to [`is_todo_source`]:
/// a convention check, not a parser.
///
/// # Errors
/// If the plugin (or its prelude) fails to load, or a card file isn't
/// readable. Cards that read but don't parse are `failures`, not errors.
pub fn validate_plugin(plugin_dir: &Path) -> anyhow::Result<Validation> {
    let plugin = Plugin::load_with_sibling_prelude(plugin_dir)?;
    let mut validation = Validation {
        valid: 0,
        todos: 0,
        failures: Vec::new(),
    };
    for path in ron_files_recursive(&plugin_dir.join(CARDS_DIR))? {
        let source = read(&path)?;
        if is_todo_source(&source) {
            validation.todos += 1;
            continue;
        }
        match plugin.macros.read_str::<Card>(&source) {
            Ok(_) => validation.valid += 1,
            Err(error) => validation.failures.push(InvalidCard { path, error }),
        }
    }
    Ok(validation)
}
