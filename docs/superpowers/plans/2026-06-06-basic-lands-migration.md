# Basic Lands Migration (`_005`) Implementation Plan

> **For agentic workers:** REQUIRED SUB-SKILL: Use superpowers:subagent-driven-development (recommended) or superpowers:executing-plans to implement this plan task-by-task. Steps use checkbox (`- [ ]`) syntax for tracking.

**Goal:** Implement `_005_basic_lands` (todo files → proper `Normal(...)` land definitions), plus its supporting infrastructure: shared card-todo reading, builtin-as-prelude plugin loading, plugin validation, and the `xtask` dispatcher.

**Architecture:** Migrations read todos through a shared serde module and write output via `format!` string templates (validated by the real macro-aware reader, not a write schema). `deckmaste_cards` gains prelude-aware loading and a `validate_plugin` pass; both bins refactor to lib entry points that `cargo xtask` dispatches to in-process.

**Tech Stack:** Rust (edition 2024), ron 0.12, serde, clap 4, anyhow, jj for VCS.

**Spec:** `docs/superpowers/specs/2026-06-06-basic-lands-migration-design.md`

**VCS note — no git commits.** This repo is jj; the user keeps one feature commit. All work rides in the current working-copy commit `@` ("basic lands migration"); jj snapshots automatically. Each task ends with a **Checkpoint** step (tests + `jj st`) instead of a commit. Do not run `git commit` or `jj commit`.

**Verified by spike (2026-06-06):** mixed tagged/untagged `Stat` deserializes correctly through ron 0.12 with `implicit_some` (`power: 2` → `Some(Number(2))`, `power: Other("*")` → `Some(Other)`); `{name:?}` debug-escaping round-trips through RON string parsing (quotes, non-ASCII); `ron::value::RawValue::from_ron` accepts the rendered template and rejects truncated input. `CardFile::Card` is never constructed anywhere — safe to delete.

---

### Task 1: `is_todo_source` in `deckmaste_core::plugin`

The todo-stub convention is shared by writers (migrations) and the new validation pass, so the content check moves to core. No regex — plain line scan keeps core dependency-free.

**Files:**
- Modify: `crates/deckmaste_core/src/plugin.rs`
- Modify: `crates/deckmaste_migrations/src/migrations/mod.rs:1-31`

- [ ] **Step 1: Write the failing test**

Append to the `tests` module at the bottom of `crates/deckmaste_core/src/plugin.rs`:

```rust
    #[test]
    fn todo_sources() {
        assert!(is_todo_source("Todo(\n    layout: \"normal\",\n)"));
        // The Todo( line may follow a // CR comment line.
        assert!(is_todo_source("// CR 205.3i\nTodo(\n    layout: \"normal\",\n)"));
        assert!(is_todo_source("    Todo(layout: \"normal\")"));
        assert!(!is_todo_source(
            "Normal(\n    name: \"Plains\",\n    types: [Land],\n)"
        ));
        assert!(!is_todo_source(""));
    }
```

- [ ] **Step 2: Run test to verify it fails**

Run: `cargo test -p deckmaste_core todo_sources`
Expected: COMPILE ERROR — `is_todo_source` not found.

- [ ] **Step 3: Implement `is_todo_source`**

Add to `crates/deckmaste_core/src/plugin.rs`, after `card_filename`:

```rust
/// Whether card-file source is still an unimplemented stub. A stub is any
/// file with a line starting (modulo indentation) with `Todo(` — checked
/// per line because the `Todo(` may follow a `// CR ...` comment line, so
/// it is not necessarily at the start of the file. A convention check, not
/// a parser: migrations may only overwrite files while this holds.
#[must_use]
pub fn is_todo_source(source: &str) -> bool {
    source
        .lines()
        .any(|line| line.trim_start().starts_with("Todo("))
}
```

- [ ] **Step 4: Run test to verify it passes**

Run: `cargo test -p deckmaste_core todo_sources`
Expected: PASS

- [ ] **Step 5: Delegate the migrations' path-based check**

In `crates/deckmaste_migrations/src/migrations/mod.rs`, replace the `is_todo` function **and** delete the now-unused regex statics around it. The top of the file currently reads:

```rust
use std::path::Path;
use std::sync::LazyLock;

use regex::Regex;

use crate::layout::PluginLayout;
```

and `is_todo` is:

```rust
/// A file may be (over)written only while it is still an unimplemented stub.
/// (?m) anchors ^ at line starts: the Todo( line may follow a // CR comment
/// line, so it is not necessarily at the start of the file.
fn is_todo(path: &Path) -> anyhow::Result<bool> {
    static TODO_PATTERN: LazyLock<Regex> = LazyLock::new(|| Regex::new(r"(?m)^\s*Todo\(").unwrap());

    if !path.exists() {
        return Ok(true);
    }
    Ok(TODO_PATTERN.is_match(&std::fs::read_to_string(path)?))
}
```

Replace `is_todo` with:

```rust
/// A file may be (over)written only while it is still an unimplemented stub.
fn is_todo(path: &Path) -> anyhow::Result<bool> {
    if !path.exists() {
        return Ok(true);
    }
    Ok(deckmaste_core::plugin::is_todo_source(
        &std::fs::read_to_string(path)?,
    ))
}
```

`LazyLock` and `Regex` are still used by `to_rust_ident` at the bottom of the same file — keep those imports.

- [ ] **Step 6: Checkpoint**

Run: `cargo test -p deckmaste_core -p deckmaste_migrations && cargo clippy -p deckmaste_core -p deckmaste_migrations && cargo fmt`
Expected: all green, no clippy warnings. `jj st` shows the two modified files.

---

### Task 2: Shared `card_todo` module (read + write side of todo files)

Move `Stat`, `CardFile`, `CardFaceTodo`, and the `stat()` helper from `_004_card_todos.rs` into a sibling module, add `Deserialize`, delete the dead `CardFile::Card` variant. Todos become readable by every later migration.

**Files:**
- Create: `crates/deckmaste_migrations/src/migrations/card_todo.rs`
- Modify: `crates/deckmaste_migrations/src/migrations/_004_card_todos.rs`
- Modify: `crates/deckmaste_migrations/src/migrations/mod.rs:8-15` (module list)

- [ ] **Step 1: Create the module with the moved types (now `Deserialize` too)**

Create `crates/deckmaste_migrations/src/migrations/card_todo.rs`:

```rust
//! The card todo file shape: written by `_004`, read back by every later
//! migration that turns todos into real definitions. Plain ron/serde on
//! both sides — todo files quote everything, so no macro awareness needed.

use deckmaste_core::{Color, Ident, ManaCost};
use serde::{Deserialize, Serialize};

use crate::ron_output::one_line_if_single;

/// Numbers serialize untagged (`power: 2`); anything else keeps its tag
/// (`power: Other("*")`). Untagged variants must come last in the enum.
#[derive(Debug, PartialEq, Serialize, Deserialize)]
pub(super) enum Stat {
    Other(String),
    #[serde(untagged)]
    Number(serde_json::Number),
}

/// `Number(2)` for values that parse as JSON numbers, `Other("*")`
/// otherwise.
pub(super) fn stat(value: &str) -> Stat {
    match serde_json::from_str(value) {
        Ok(number) => Stat::Number(number),
        Err(_) => Stat::Other(value.to_owned()),
    }
}

/// A card todo file is always `Todo(layout: ..., faces: [...])`, with the
/// MTGJSON layout name verbatim and one anonymous struct per face.
#[derive(Debug, Serialize, Deserialize)]
pub(super) enum CardFile {
    Todo {
        layout: Ident,
        faces: Vec<CardFaceTodo>,
    },
}

/// One face of a todo. Every field a skip attr can omit carries
/// `#[serde(default)]` so the same shape reads back — the skip attrs stay
/// load-bearing (`implicit_some` cannot omit `None` fields by itself).
#[derive(Debug, Serialize, Deserialize)]
pub(super) struct CardFaceTodo {
    pub(super) name: String,
    #[serde(
        default,
        skip_serializing_if = "ManaCost::is_empty",
        serialize_with = "one_line_if_single"
    )]
    pub(super) mana_cost: ManaCost,
    #[serde(
        default,
        skip_serializing_if = "Vec::is_empty",
        serialize_with = "one_line_if_single"
    )]
    pub(super) color_indicator: Vec<Color>,
    #[serde(
        default,
        skip_serializing_if = "Vec::is_empty",
        serialize_with = "one_line_if_single"
    )]
    pub(super) supertypes: Vec<Ident>,
    #[serde(serialize_with = "one_line_if_single")]
    pub(super) types: Vec<Ident>,
    #[serde(
        default,
        skip_serializing_if = "Vec::is_empty",
        serialize_with = "one_line_if_single"
    )]
    pub(super) subtypes: Vec<Ident>,
    #[serde(
        default,
        skip_serializing_if = "Vec::is_empty",
        serialize_with = "one_line_if_single"
    )]
    pub(super) text: Vec<String>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub(super) power: Option<Stat>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub(super) toughness: Option<Stat>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub(super) loyalty: Option<Stat>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub(super) defense: Option<Stat>,
}
```

Differences from the `_004` originals, all deliberate:

- `CardFile::Card(Box<Card>)` is **deleted** — never constructed (verified), and the template write-side decision supersedes its purpose. With one variant the enum derives `Deserialize` with no untagged complications.
- `Deserialize` added everywhere; `#[serde(default)]` added to exactly the fields that have a skip attr.
- `Debug` added (used by tests).
- Items and fields are `pub(super)`: visible throughout the `migrations` module tree, nowhere else.

- [ ] **Step 2: Move the struct-shape tests into the new module**

Append to `crates/deckmaste_migrations/src/migrations/card_todo.rs` — these are the `stats`, `mana_symbols_serialize_flat`, `test_face`, `single_face_serialization`, and `multi_face_serialization` tests moved verbatim from `_004`, plus two new read-side tests (`reads_004_output`, `todo_round_trip`):

```rust
#[cfg(test)]
mod tests {
    use deckmaste_core::{Color, ManaSymbol};

    use super::*;
    use crate::ron_output::{ron_options, to_string_pretty};

    #[test]
    fn mana_symbols_serialize_flat() {
        // The Simple and Color wrapper variants are untagged: the nested
        // model must not show up in the RON.
        let symbols: ManaCost = "{R}{2}{C}{2/W}{W/P}{G/U/P}{X}{S}".parse().unwrap();
        assert_eq!(
            ron_options().to_string(&symbols).unwrap(),
            "[Red,Generic(2),Colorless,Hybrid(Generic(2),White),Phyrexian(White,None),\
             Phyrexian(Green,Blue),Variable,Snow]"
        );
    }

    #[test]
    fn stats() {
        let render = |value| ron_options().to_string(&stat(value)).unwrap();
        assert_eq!(render("2"), "2");
        assert_eq!(render("-1"), "-1");
        assert_eq!(render("*"), "Other(\"*\")");
        assert_eq!(render("1+*"), "Other(\"1+*\")");
        assert_eq!(render("X"), "Other(\"X\")");
    }

    /// A vanilla face has no mana cost and no text, like a basic land:
    /// optional fields must be skipped, not handed to the serializers.
    fn test_face(name: &str, vanilla: bool) -> CardFaceTodo {
        CardFaceTodo {
            name: name.to_owned(),
            mana_cost: if vanilla {
                ManaCost::default()
            } else {
                vec![
                    ManaSymbol::Hybrid(2.into(), Color::White),
                    ManaSymbol::Simple(Color::Green.into()),
                ]
                .into()
            },
            color_indicator: vec![],
            supertypes: vec![],
            types: vec!["Creature".into()],
            subtypes: vec!["Time Lord".into()],
            text: if vanilla {
                vec![]
            } else {
                vec![
                    "Flying".to_owned(),
                    "Doctor's \"companion\" rule.".to_owned(),
                ]
            },
            power: Some(stat("2")),
            toughness: Some(stat("*")),
            loyalty: None,
            defense: None,
        }
    }

    #[test]
    fn single_face_serialization() {
        let card = CardFile::Todo {
            layout: "normal".into(),
            faces: vec![test_face("Solo", false)],
        };
        let serialized = to_string_pretty(&card).unwrap();
        assert_eq!(
            serialized,
            r##"Todo(
    layout: "normal",
    faces: [
        (
            name: "Solo",
            mana_cost: [
                Hybrid(Generic(2), White),
                Green,
            ],
            types: ["Creature"],
            subtypes: ["Time Lord"],
            text: [
                "Flying",
                r#"Doctor's "companion" rule."#,
            ],
            power: 2,
            toughness: Other("*"),
        ),
    ],
)"##
        );
    }

    #[test]
    fn multi_face_serialization() {
        let card = CardFile::Todo {
            layout: "transform".into(),
            faces: vec![test_face("Front", false), test_face("Back", true)],
        };
        let serialized = to_string_pretty(&card).unwrap();
        // Each face is its own list element; Hybrid mana symbols stay
        // inline, and raw string bodies stay unindented.
        assert_eq!(
            serialized,
            r##"Todo(
    layout: "transform",
    faces: [
        (
            name: "Front",
            mana_cost: [
                Hybrid(Generic(2), White),
                Green,
            ],
            types: ["Creature"],
            subtypes: ["Time Lord"],
            text: [
                "Flying",
                r#"Doctor's "companion" rule."#,
            ],
            power: 2,
            toughness: Other("*"),
        ),
        (
            name: "Back",
            types: ["Creature"],
            subtypes: ["Time Lord"],
            power: 2,
            toughness: Other("*"),
        ),
    ],
)"##
        );
    }

    /// Reading the exact shape `_004` writes (real Snow-Covered Plains
    /// output): defaults fill the omitted fields.
    #[test]
    fn reads_004_output() {
        let source = r#"Todo(
    layout: "normal",
    faces: [
        (
            name: "Snow-Covered Plains",
            supertypes: [
                "Basic",
                "Snow",
            ],
            types: ["Land"],
            subtypes: ["Plains"],
        ),
    ],
)
"#;
        let CardFile::Todo { layout, faces } = ron_options().from_str(source).unwrap();
        assert_eq!(layout, "normal");
        assert_eq!(faces.len(), 1);
        assert_eq!(faces[0].name, "Snow-Covered Plains");
        assert_eq!(faces[0].supertypes, ["Basic", "Snow"]);
        assert_eq!(faces[0].subtypes, ["Plains"]);
        assert!(faces[0].mana_cost.is_empty());
        assert!(faces[0].text.is_empty());
        assert_eq!(faces[0].power, None);
    }

    /// Serialize -> deserialize -> serialize is a fixed point, stats and
    /// mana symbols included.
    #[test]
    fn todo_round_trip() {
        let card = CardFile::Todo {
            layout: "transform".into(),
            faces: vec![test_face("Front", false), test_face("Back", true)],
        };
        let serialized = to_string_pretty(&card).unwrap();
        let parsed: CardFile = ron_options().from_str(&serialized).unwrap();
        assert_eq!(to_string_pretty(&parsed).unwrap(), serialized);
    }
}
```

- [ ] **Step 3: Register the module**

In `crates/deckmaste_migrations/src/migrations/mod.rs`, the module list currently ends with `mod keyword_todos;`. Add `card_todo` in its alphabetical spot:

```rust
mod _000_keyword_ability_todos;
mod _001_keyword_action_todos;
mod _002_ability_word_todos;
mod _003_subtypes;
mod _004_card_todos;
mod _005_basic_lands;
mod _006_vanilla_creatures;
mod card_todo;
mod keyword_todos;
```

- [ ] **Step 4: Slim `_004_card_todos.rs` down to its own logic**

In `crates/deckmaste_migrations/src/migrations/_004_card_todos.rs`:

1. Delete the `Stat`, `CardFile`, and `CardFaceTodo` definitions and the `stat()` function (lines 13–73 and 144–151 of the current file).
2. Delete the whole moved-out portion of the `tests` module: `mana_symbols_serialize_flat`, `stats`, `test_face`, `single_face_serialization`, `multi_face_serialization`. Keep `reminder_text` and `keyword_lines`.
3. Replace the import block at the top. It currently reads:

```rust
use std::sync::LazyLock;

use anyhow::Context;
use deckmaste_core::plugin::card_file;
use deckmaste_core::{Card, Color, Ident, ManaCost};
use regex::Regex;
use serde::Serialize;

use crate::data::DataStr;
use crate::data::mtgjson::AtomicCard;
use crate::ron_output::{one_line_if_single, to_string_pretty};
```

becomes:

```rust
use std::sync::LazyLock;

use anyhow::Context;
use deckmaste_core::plugin::card_file;
use deckmaste_core::Color;
use regex::Regex;

use super::card_todo::{CardFile, CardFaceTodo, stat};
use crate::data::DataStr;
use crate::data::mtgjson::AtomicCard;
use crate::ron_output::to_string_pretty;
```

(`rustfmt` + the compiler have the final word on exactly which imports survive — `Ident`/`ManaCost` are only referenced through field types now. The kept `tests` module still needs its `use super::*;` and nothing else.)

- [ ] **Step 5: Run the full migrations test suite**

Run: `cargo test -p deckmaste_migrations`
Expected: PASS — all moved tests green in their new home, `reminder_text`/`keyword_lines` green in `_004`, including the two new read-side tests.

- [ ] **Step 6: Checkpoint**

Run: `cargo clippy -p deckmaste_migrations && cargo fmt`
Expected: clean. `jj st` shows `card_todo.rs` added, `_004_card_todos.rs` and `mod.rs` modified.

---

### Task 3: The `_005_basic_lands` migration

**Files:**
- Modify: `crates/deckmaste_migrations/src/migrations/_005_basic_lands.rs` (replace the stub entirely)

- [ ] **Step 1: Write the failing tests**

Replace the entire contents of `crates/deckmaste_migrations/src/migrations/_005_basic_lands.rs` with the tests plus stub declarations (functions `todo!()` for now):

```rust
use anyhow::Context;

use super::card_todo::{CardFile, CardFaceTodo};
use crate::layout::PluginLayout;

/// A todo is a convertible basic land when it's a single normal face that
/// is nothing but a name plus Basic Land types. Any leftover rules text
/// (Wastes' "{T}: Add {C}.") means the ability model can't express the
/// card yet, so it stays a todo.
fn basic_land_face(card: &CardFile) -> Option<&CardFaceTodo> {
    todo!()
}

/// The finished definition in the builtin/cards house style: bare idents,
/// arrays inline.
fn render_land(face: &CardFaceTodo) -> String {
    todo!()
}

pub(super) struct BasicLands;

impl super::Migration for BasicLands {
    fn apply(&self, _plugin: &PluginLayout) -> anyhow::Result<()> { todo!() }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::ron_output::ron_options;

    fn todo(source: &str) -> CardFile { ron_options().from_str(source).unwrap() }

    const PLAINS: &str = r#"Todo(
    layout: "normal",
    faces: [
        (
            name: "Plains",
            supertypes: ["Basic"],
            types: ["Land"],
            subtypes: ["Plains"],
        ),
    ],
)
"#;

    const SNOW_PLAINS: &str = r#"Todo(
    layout: "normal",
    faces: [
        (
            name: "Snow-Covered Plains",
            supertypes: [
                "Basic",
                "Snow",
            ],
            types: ["Land"],
            subtypes: ["Plains"],
        ),
    ],
)
"#;

    const WASTES: &str = r#"Todo(
    layout: "normal",
    faces: [
        (
            name: "Wastes",
            supertypes: ["Basic"],
            types: ["Land"],
            text: ["{T}: Add {C}."],
        ),
    ],
)
"#;

    #[test]
    fn converts_plains() {
        let card = todo(PLAINS);
        let face = basic_land_face(&card).expect("Plains converts");
        assert_eq!(
            render_land(face),
            r#"Normal(
    name: "Plains",
    supertypes: [Basic],
    types: [Land],
    subtypes: [Plains],
)
"#
        );
    }

    #[test]
    fn converts_snow_lands_with_inline_supertypes() {
        let card = todo(SNOW_PLAINS);
        let face = basic_land_face(&card).expect("Snow-Covered Plains converts");
        assert_eq!(
            render_land(face),
            r#"Normal(
    name: "Snow-Covered Plains",
    supertypes: [Basic, Snow],
    types: [Land],
    subtypes: [Plains],
)
"#
        );
    }

    /// Wastes has no basic land type: its "{T}: Add {C}." is printed
    /// ability text the model can't express yet, so it stays a todo.
    #[test]
    fn skips_wastes_printed_ability() {
        assert!(basic_land_face(&todo(WASTES)).is_none());
    }

    #[test]
    fn skips_nonbasic_lands() {
        let nonbasic = r#"Todo(
    layout: "normal",
    faces: [
        (
            name: "Urza's Saga",
            supertypes: ["Legendary"],
            types: [
                "Enchantment",
                "Land",
            ],
            subtypes: [
                "Urza's",
                "Saga",
            ],
        ),
    ],
)
"#;
        assert!(basic_land_face(&todo(nonbasic)).is_none());
    }

    #[test]
    fn skips_multiface_and_nonnormal_layouts() {
        let mdfc = r#"Todo(
    layout: "modal_dfc",
    faces: [
        (
            name: "A",
            supertypes: ["Basic"],
            types: ["Land"],
            subtypes: ["Plains"],
        ),
        (
            name: "B",
            supertypes: ["Basic"],
            types: ["Land"],
            subtypes: ["Island"],
        ),
    ],
)
"#;
        assert!(basic_land_face(&todo(mdfc)).is_none());
    }
}
```

- [ ] **Step 2: Run tests to verify they fail**

Run: `cargo test -p deckmaste_migrations _005`
Expected: FAIL — `todo!()` panics (or compile error if signatures are off; fix signatures, not tests).

- [ ] **Step 3: Implement predicate, template, and `apply`**

Replace the three `todo!()` bodies:

```rust
fn basic_land_face(card: &CardFile) -> Option<&CardFaceTodo> {
    let CardFile::Todo { layout, faces } = card;
    match faces.as_slice() {
        [face]
            if *layout == "normal"
                && face.types == ["Land"]
                && face.supertypes.iter().any(|s| *s == "Basic")
                && face.text.is_empty() =>
        {
            Some(face)
        }
        _ => None,
    }
}

fn render_land(face: &CardFaceTodo) -> String {
    format!(
        "\
Normal(
    name: {name:?},
    supertypes: [{supertypes}],
    types: [Land],
    subtypes: [{subtypes}],
)
",
        name = face.name,
        supertypes = face.supertypes.join(", "),
        subtypes = face.subtypes.join(", "),
    )
}
```

and `apply`:

```rust
impl super::Migration for BasicLands {
    fn apply(&self, plugin: &PluginLayout) -> anyhow::Result<()> {
        let cards_dir = plugin.cards_dir()?;
        let mut paths: Vec<_> = std::fs::read_dir(&cards_dir)?
            .map(|entry| entry.map(|e| e.path()))
            .collect::<Result<_, _>>()?;
        paths.sort();

        for path in paths {
            if path.extension().is_none_or(|ext| ext != "ron") || !path.is_file() {
                continue;
            }
            let source = std::fs::read_to_string(&path)?;
            if !deckmaste_core::plugin::is_todo_source(&source) {
                continue;
            }
            let card: CardFile = crate::ron_output::ron_options()
                .from_str(&source)
                .with_context(|| format!("parsing todo {}", path.display()))?;
            let Some(face) = basic_land_face(&card) else {
                continue;
            };

            let definition = render_land(face);
            // The cheap guard templates get: the output must still be RON.
            // Real validation is `cargo xtask validate` through the reader.
            ron::value::RawValue::from_ron(&definition)
                .with_context(|| format!("invalid render for {}", path.display()))?;
            std::fs::write(&path, definition)?;
            eprintln!("wrote {}", path.display());
        }
        Ok(())
    }
}
```

Notes for the implementer:

- `*layout == "normal"` and `*s == "Basic"` go through `Ident`'s `PartialEq<&str>`; `face.types == ["Land"]` is `Vec<Ident> == [&str; 1]`; `.join(", ")` works because `Ident: Borrow<str>`. All already-existing impls in `deckmaste_core/src/ident.rs`.
- Conversion is idempotent for free: a converted file no longer matches `is_todo_source`, so re-runs skip it at the gate.
- The full-corpus parse (every todo deserializes into `CardFile`) is intentional — it's the shared module earning its keep early. If some exotic todo fails to parse, the error names the file; fix the module, that's the point of it being shared.

- [ ] **Step 4: Run tests to verify they pass**

Run: `cargo test -p deckmaste_migrations _005`
Expected: PASS (5 tests).

- [ ] **Step 5: Checkpoint**

Run: `cargo test -p deckmaste_migrations && cargo clippy -p deckmaste_migrations && cargo fmt`
Expected: clean. Do **not** run the migration against `plugins/` yet — end-to-end is Task 9, after validation exists.

---

### Task 4: Builtin-as-prelude loading in `deckmaste_cards`

Prelude semantics are **last plugin wins** (user decision 2026-06-06): definitions in a later layer override same-name entries from an earlier layer; duplicates *within* one plugin remain errors.

**Files:**
- Modify: `crates/deckmaste_cards/src/plugin.rs`
- Modify: `crates/deckmaste_cards/src/macros.rs` (two small additions)
- Modify: `crates/deckmaste_cards/Cargo.toml` (tempfile dev-dependency)

- [ ] **Step 1: Write the failing tests**

`plugin.rs` has no tests module today. Append one (these use the real `plugins/` tree — macros and types only, never the 31k cards, so they're fast):

```rust
#[cfg(test)]
mod tests {
    use deckmaste_core::Type;

    use super::*;

    fn plugins() -> PathBuf { Path::new(env!("CARGO_MANIFEST_DIR")).join("../../plugins") }

    #[test]
    fn sibling_prelude_brings_builtin_subtypes() {
        let wizards = Plugin::load_with_sibling_prelude(plugins().join("wizards")).unwrap();
        // Declared in builtin/types/land, visible through the prelude.
        assert!(wizards.subtypes.contains_key("Plains"));
        // wizards' own declarations load on top.
        assert!(wizards.subtypes.contains_key("Cave"));
    }

    #[test]
    fn builtin_loads_without_self_prelude() {
        let builtin = Plugin::load_with_sibling_prelude(plugins().join("builtin")).unwrap();
        assert!(builtin.subtypes.contains_key("Plains"));
    }

    /// Last plugin wins: a redeclaration overrides the prelude's version
    /// rather than erroring. wizards hits this for real — `_003` generates
    /// the full subtype set, overlapping builtin's declarations.
    #[test]
    fn redeclarations_override_the_prelude() {
        let mut prelude = Plugin::load(plugins().join("builtin")).unwrap();
        prelude.subtypes.get_mut("Plains").unwrap().types = vec![Type::Creature];
        let layered = Plugin::load_with_prelude(&prelude, plugins().join("builtin")).unwrap();
        // builtin's own LandType("Plains") declaration replaced the
        // doctored prelude entry.
        assert_eq!(layered.subtypes["Plains"].types, [Type::Land]);
    }

    /// Within one plugin, file order is alphabetical happenstance: two
    /// declarations of one name are an error, not "last wins".
    #[test]
    fn duplicates_within_a_plugin_error() {
        let root = tempfile::tempdir().unwrap();
        let types = root.path().join("types");
        std::fs::create_dir_all(&types).unwrap();
        std::fs::write(types.join("A.ron"), r#"Subtype(name: "X", types: [Land])"#).unwrap();
        std::fs::write(types.join("B.ron"), r#"Subtype(name: "X", types: [Land])"#).unwrap();
        let err = Plugin::load(root.path()).unwrap_err();
        assert!(err.to_string().contains("already defined"), "{err}");
    }
}
```

Notes: `HashMap` lookups with `&str` keys work because `Ident: Borrow<str>` (`subtypes["Plains"]` likewise via `Index`). The last test needs tempfile — add it with `cargo add --dev tempfile -p deckmaste_cards`.

- [ ] **Step 2: Run tests to verify they fail**

Run: `cargo test -p deckmaste_cards plugin`
Expected: COMPILE ERROR — `load_with_sibling_prelude` / `load_with_prelude` not found.

- [ ] **Step 3: Implement the prelude constructors**

In `crates/deckmaste_cards/src/plugin.rs`, refactor `Plugin::load` into a parameterized core plus three entry points. The existing `load` body keeps everything from the `// Macro definitions are self-describing.` comment down; only its head changes:

```rust
impl Plugin {
    /// # Errors
    /// If a macro definition or subtype declaration fails to read, expand,
    /// or register, or a directory isn't listable.
    pub fn load(root: impl Into<PathBuf>) -> anyhow::Result<Self> {
        Self::load_onto(MacroSet::default(), HashMap::new(), root.into())
    }

    /// Loads `root` with `prelude`'s macros and subtype declarations
    /// already in scope. Last plugin wins: `root`'s definitions override
    /// same-name entries from the prelude, while duplicates within `root`
    /// itself are still [`DuplicateMacro`](crate::macros::DuplicateMacro)
    /// errors.
    ///
    /// # Errors
    /// As [`Plugin::load`].
    pub fn load_with_prelude(prelude: &Plugin, root: impl Into<PathBuf>) -> anyhow::Result<Self> {
        Self::load_onto(prelude.macros.clone(), prelude.subtypes.clone(), root.into())
    }

    /// Loads `root` under the builtin convention: a sibling directory named
    /// `builtin` (that isn't `root` itself) is the prelude to every other
    /// plugin.
    ///
    /// # Errors
    /// As [`Plugin::load_with_prelude`]; `root` must exist.
    pub fn load_with_sibling_prelude(root: impl Into<PathBuf>) -> anyhow::Result<Self> {
        let root = root.into();
        let builtin = root.parent().unwrap_or(Path::new("")).join("builtin");
        if builtin.is_dir() && builtin.canonicalize()? != root.canonicalize()? {
            let prelude = Plugin::load(&builtin)
                .with_context(|| format!(r#"loading prelude "{}""#, builtin.display()))?;
            return Self::load_with_prelude(&prelude, root);
        }
        Self::load(root)
    }

    fn load_onto(
        mut macros: MacroSet,
        mut subtypes: HashMap<Ident, Subtype>,
        root: PathBuf,
    ) -> anyhow::Result<Self> {
        // Macro definitions are self-describing.
        for path in ron_files_recursive(&root.join(MACROS_DIR))? {
            ...existing body, with the two `let mut` bindings for `macros`
            and `subtypes` deleted (they're parameters now)...
        }
        ...
    }
```

Concretely: `load_onto` is the old `load` minus `let root = root.into();`, `let mut macros = MacroSet::default();`, and `let mut subtypes = HashMap::new();` — those become the parameters — plus **last-plugin-wins layering**, which needs three changes:

**(a)** Two overriding registration methods on `MacroSet` (in `crates/deckmaste_cards/src/macros.rs`, next to `insert` and `declare`):

```rust
    /// Registers `def` under each of its kinds, overriding same-kind
    /// entries already in scope. Layer-to-layer overriding is legal — last
    /// plugin wins — so the caller is responsible for rejecting duplicates
    /// *within* one layer.
    pub fn replace(&mut self, def: &MacroDef) {
        for &kind in &def.kinds {
            self.macros
                .entry(kind)
                .or_default()
                .insert(def.name, def.clone());
        }
    }

    /// Like [`MacroSet::declare`], but overriding: see
    /// [`MacroSet::replace`].
    pub fn redeclare(&mut self, name: Ident, declaration: &str) {
        self.replace(&MacroDef {
            name,
            kinds: vec![MacroKind::Subtype],
            params: Params::default(),
            body: declaration.trim().into(),
        });
    }
```

**(b)** In `load_onto`, the macro-definitions loop tracks what this plugin itself defined and overrides the inherited scope:

```rust
        // What this plugin itself defines, per kind. A name inherited from
        // the prelude may be overridden — last plugin wins — but two
        // definitions within one plugin still collide: file order here is
        // alphabetical happenstance, so "last" would be meaningless.
        let mut own = HashSet::new();

        // Macro definitions are self-describing.
        for path in ron_files_recursive(&root.join(MACROS_DIR))? {
            let def: MacroDef = deckmaste_core::ron::options()
                .from_str(&read(&path)?)
                .with_context(|| format!(r#"parsing macro "{}""#, path.display()))?;
            for &kind in &def.kinds {
                if !own.insert((kind, def.name)) {
                    return Err(DuplicateMacro {
                        kind,
                        name: def.name,
                    })
                    .with_context(|| format!(r#"loading "{}""#, path.display()));
                }
            }
            macros.replace(&def);
        }
```

**(c)** The declaration retry loop's `Ok(subtype)` arm does the same:

```rust
                    Ok(subtype) => {
                        if !own.insert((MacroKind::Subtype, subtype.name)) {
                            return Err(DuplicateMacro {
                                kind: MacroKind::Subtype,
                                name: subtype.name,
                            })
                            .with_context(|| format!(r#"declaring "{}""#, path.display()));
                        }
                        macros.redeclare(subtype.name, &declaration);
                        subtypes.insert(subtype.name, subtype);
                    }
```

Imports in `plugin.rs` grow accordingly: `use std::collections::{HashMap, HashSet};` and `use crate::macros::{DuplicateMacro, MacroDef, MacroKind, MacroSet};`. The `own`-set also subsumes `insert`'s old self-repeating-kind check for files loaded this way; `MacroSet::insert`/`declare` keep their checking semantics for other callers and tests.

- [ ] **Step 4: Run tests to verify they pass**

Run: `cargo test -p deckmaste_cards plugin`
Expected: PASS (5 tests).

- [ ] **Step 5: Checkpoint**

Run: `cargo test -p deckmaste_cards && cargo clippy -p deckmaste_cards && cargo fmt`
Expected: clean.

---

### Task 5: `validate_plugin` and the builtin `#[test]`

**Files:**
- Create: `crates/deckmaste_cards/src/validate.rs`
- Create: `crates/deckmaste_cards/tests/validate_builtin.rs`
- Modify: `crates/deckmaste_cards/src/lib.rs`
- Modify: `crates/deckmaste_cards/src/plugin.rs` (visibility of two helpers)

- [ ] **Step 1: Write the failing integration test**

Create `crates/deckmaste_cards/tests/validate_builtin.rs`:

```rust
//! Every finished (non-todo) builtin card must parse through the
//! macro-aware reader. Run plain `cargo test` and this guards the prelude
//! everything else depends on; wizards is the explicit
//! `cargo xtask validate plugins/wizards`.

use std::path::Path;

#[test]
fn builtin_cards_are_valid() {
    let builtin = Path::new(env!("CARGO_MANIFEST_DIR")).join("../../plugins/builtin");
    let validation = deckmaste_cards::validate::validate_plugin(&builtin).unwrap();
    for failure in &validation.failures {
        eprintln!("{}: {}", failure.path.display(), failure.error);
    }
    assert!(validation.failures.is_empty());
    // The hand-written builtin cards: 5 basics + Lightning Bolt +
    // Grizzly Bears at the time of writing. Floor, not exact, so adding
    // cards doesn't break the test.
    assert!(validation.valid >= 7, "only {} cards checked", validation.valid);
}
```

- [ ] **Step 2: Run test to verify it fails**

Run: `cargo test -p deckmaste_cards --test validate_builtin`
Expected: COMPILE ERROR — `validate` module not found.

- [ ] **Step 3: Implement `validate_plugin`**

First, in `crates/deckmaste_cards/src/plugin.rs`, widen two helpers used from the new module: `fn read(...)` → `pub(crate) fn read(...)` and `fn ron_files_recursive(...)` → `pub(crate) fn ron_files_recursive(...)`.

Create `crates/deckmaste_cards/src/validate.rs`:

```rust
//! Validating a plugin's finished cards through the macro-aware reader.
//! This is the validation layer the template-based migrations rely on:
//! their output is checked by the real reader, not by a write schema.

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
/// stopping at the first.
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
```

In `crates/deckmaste_cards/src/lib.rs`, add the module:

```rust
//! Reading card data from plugin directories.

pub mod expand;
pub mod macros;
pub mod plugin;
pub mod validate;
```

- [ ] **Step 4: Run test to verify it passes**

Run: `cargo test -p deckmaste_cards --test validate_builtin`
Expected: PASS — 7 valid builtin cards, 0 failures. (If a builtin card fails here, that's a real pre-existing bug surfacing; report it rather than papering over.)

- [ ] **Step 5: Checkpoint**

Run: `cargo test -p deckmaste_cards && cargo clippy -p deckmaste_cards && cargo fmt`
Expected: clean.

---

### Task 6: `deckmaste_cards` CLI entry points (`card`, `validate`)

**Files:**
- Create: `crates/deckmaste_cards/src/cli.rs`
- Modify: `crates/deckmaste_cards/src/lib.rs`
- Modify: `crates/deckmaste_cards/src/main.rs` (replace entirely)

- [ ] **Step 1: Create the CLI module**

Create `crates/deckmaste_cards/src/cli.rs`:

```rust
//! CLI entry points, shared by this crate's `card` bin and `cargo xtask`.
//! Each takes full argv (program name included) so both callers parse
//! identically.

use std::ffi::OsString;
use std::path::{Path, PathBuf};

use clap::Parser;

use crate::plugin::Plugin;

/// Shows a card as parsed from a plugin, with its macro references
/// expanded.
#[derive(Debug, Parser)]
struct CardArgs {
    pub plugin_dir: PathBuf,
    pub card_name: String,
}

/// The `card` entry point: parse one card (builtin sibling prelude in
/// scope) and print its expansion.
///
/// # Errors
/// If the plugin fails to load or the card is missing or invalid.
pub fn card<I: IntoIterator<Item = OsString>>(args: I) -> anyhow::Result<()> {
    let args = CardArgs::parse_from(args);
    let plugin = Plugin::load_with_sibling_prelude(&args.plugin_dir)?;
    let card = plugin.card(&args.card_name)?;

    println!(
        "{} expands to:\n",
        plugin.card_path(&args.card_name).display()
    );
    println!("{card:#?}");

    Ok(())
}

/// Validates every finished card in a plugin through the macro-aware
/// reader.
#[derive(Debug, Parser)]
struct ValidateArgs {
    /// Defaults to this workspace's `plugins/builtin`.
    pub plugin_dir: Option<PathBuf>,
}

/// The `validate` entry point: report every non-todo card that doesn't
/// parse.
///
/// # Errors
/// If the plugin fails to load, a file isn't readable, or any card is
/// invalid.
pub fn validate<I: IntoIterator<Item = OsString>>(args: I) -> anyhow::Result<()> {
    let args = ValidateArgs::parse_from(args);
    let plugin_dir = args
        .plugin_dir
        .unwrap_or_else(|| Path::new(env!("CARGO_MANIFEST_DIR")).join("../../plugins/builtin"));

    let validation = crate::validate::validate_plugin(&plugin_dir)?;
    for failure in &validation.failures {
        eprintln!("{}: {}", failure.path.display(), failure.error);
    }
    println!(
        "{}: {} valid, {} todos skipped, {} invalid",
        plugin_dir.display(),
        validation.valid,
        validation.todos,
        validation.failures.len()
    );
    if !validation.failures.is_empty() {
        anyhow::bail!("{} invalid card(s)", validation.failures.len());
    }
    Ok(())
}
```

Register it in `crates/deckmaste_cards/src/lib.rs`:

```rust
//! Reading card data from plugin directories.

pub mod cli;
pub mod expand;
pub mod macros;
pub mod plugin;
pub mod validate;
```

- [ ] **Step 2: Thin out `main.rs`**

Replace the entire contents of `crates/deckmaste_cards/src/main.rs`:

```rust
fn main() -> anyhow::Result<()> { deckmaste_cards::cli::card(std::env::args_os()) }
```

Note this changes `cargo card` behavior on purpose (spec §3): it now loads the builtin sibling prelude, where before it loaded the plugin bare.

- [ ] **Step 3: Verify both entry points by hand**

Run: `cargo card plugins/builtin "Lightning Bolt"`
Expected: the parsed `Normal(CardFace { ... })` debug output, as before.

Run: `cargo card plugins/wizards "Wastes"`
Expected: an error — Wastes is still a `Todo(...)`, which doesn't parse as a `Card`. The point is it *loads the plugin* (prelude resolution works) and fails on the card, not on plugin loading.

- [ ] **Step 4: Checkpoint**

Run: `cargo test -p deckmaste_cards && cargo clippy -p deckmaste_cards && cargo fmt`
Expected: clean.

---

### Task 7: `deckmaste_migrations` lib + CLI entry point

**Files:**
- Create: `crates/deckmaste_migrations/src/lib.rs`
- Create: `crates/deckmaste_migrations/src/cli.rs`
- Modify: `crates/deckmaste_migrations/src/main.rs` (replace entirely)

- [ ] **Step 1: Create the lib root**

Create `crates/deckmaste_migrations/src/lib.rs` (the module declarations move here from `main.rs`):

```rust
//! Migrations that build and refine the plugin data directories.

pub mod cli;
mod data;
mod layout;
mod migrations;
mod ron_output;
```

- [ ] **Step 2: Create the CLI entry point**

Create `crates/deckmaste_migrations/src/cli.rs` (the `Args` struct and dispatch move here from `main.rs`):

```rust
//! The CLI entry point, shared by this crate's bin and `cargo xtask`.

use std::ffi::OsString;
use std::path::PathBuf;

use clap::Parser;

/// Applies one migration, or all of them in order.
#[derive(Debug, Parser)]
struct Args {
    pub plugin_dir: PathBuf,
    pub migration_number: Option<usize>,
}

/// Parses full argv (program name included) and runs the migration(s).
///
/// # Errors
/// If the plugin layout is unusable or a migration fails.
pub fn run<I: IntoIterator<Item = OsString>>(args: I) -> anyhow::Result<()> {
    let args = Args::parse_from(args);

    match args.migration_number {
        Some(number) => crate::migrations::apply(&args.plugin_dir, number),
        None => crate::migrations::apply_all(&args.plugin_dir),
    }
}
```

- [ ] **Step 3: Thin out `main.rs`**

Replace the entire contents of `crates/deckmaste_migrations/src/main.rs`:

```rust
fn main() -> anyhow::Result<()> { deckmaste_migrations::cli::run(std::env::args_os()) }
```

- [ ] **Step 4: Verify**

Run: `cargo test -p deckmaste_migrations && cargo clippy -p deckmaste_migrations`
Expected: all tests still pass (they now run against the lib target), no warnings.

Run: `cargo migrate plugins/wizards 3`
Expected: runs migration 003 (subtypes) exactly as before — a quick no-behavior-change smoke test that doesn't need AllPrintings parsing. `jj st` should show no changes under `plugins/` afterward (003 output is already in place); if it shows churn, stop and investigate.

- [ ] **Step 5: Checkpoint**

Run: `cargo fmt` then `jj st`.
Expected: `lib.rs`/`cli.rs` added, `main.rs` shrunk; nothing under `plugins/` changed.

---

### Task 8: The `xtask` dispatcher

**Files:**
- Create: `crates/xtask/Cargo.toml`
- Create: `crates/xtask/src/main.rs`
- Modify: `Cargo.toml` (workspace members)
- Modify: `.cargo/config.toml` (alias)

- [ ] **Step 1: Create the crate**

Create `crates/xtask/Cargo.toml`:

```toml
[package]
name = "xtask"
version = "0.1.0"
edition = "2024"

[lints]
workspace = true

[dependencies]
anyhow = { version = "1.0.102" }
deckmaste_cards = { path = "../deckmaste_cards" }
deckmaste_migrations = { path = "../deckmaste_migrations" }
```

Create `crates/xtask/src/main.rs`:

```rust
//! Workspace automation: a pure dispatcher over the other crates' CLI
//! entry points — no subcommand logic of its own.

use std::ffi::{OsStr, OsString};

fn main() -> anyhow::Result<()> {
    let mut args = std::env::args_os();
    let _argv0 = args.next();
    let subcommand = args.next();
    // Entry points expect full argv; synthesize the program name the
    // subcommand was invoked as so clap's usage/errors read right.
    let argv = |name: &str| std::iter::once(OsString::from(format!("cargo xtask {name}")));

    match subcommand.as_deref().and_then(OsStr::to_str) {
        Some("validate") => deckmaste_cards::cli::validate(argv("validate").chain(args)),
        Some("card") => deckmaste_cards::cli::card(argv("card").chain(args)),
        Some("migrate") => deckmaste_migrations::cli::run(argv("migrate").chain(args)),
        Some(other) => anyhow::bail!("unknown subcommand {other:?}; expected validate | migrate | card"),
        None => anyhow::bail!("usage: cargo xtask <validate | migrate | card> [args...]"),
    }
}
```

- [ ] **Step 2: Wire up the workspace and alias**

In the root `Cargo.toml`, add the member:

```toml
[workspace]
members = [
    ".", "crates/deckmaste_cards",
    "crates/deckmaste_core",
    "crates/deckmaste_migrations",
    "crates/xtask",
]
```

In `.cargo/config.toml`, add to the `[alias]` table (after the `card` alias):

```toml
# `cargo xtask <validate|migrate|card> ...` -- workspace automation, a pure
# dispatcher over the crates' CLI entry points. Always release for the same
# reason as `migrate`.
xtask = "run --release -p xtask --"
```

- [ ] **Step 3: Smoke-test the dispatcher**

Run: `cargo xtask validate`
Expected: `plugins/builtin: 7 valid, 0 todos skipped, 0 invalid` (path prefix per your checkout), exit 0.

Run: `cargo xtask validate plugins/wizards`
Expected: `0 valid, 30881 todos skipped, 0 invalid` (everything is still a todo), exit 0.

Run: `cargo xtask bogus; echo $status`
Expected: the unknown-subcommand error, nonzero status. (fish: `$status`.)

- [ ] **Step 4: Checkpoint**

Run: `cargo test --workspace && cargo clippy --workspace && cargo fmt`
Expected: clean across the workspace, xtask included.

---

### Task 9: End-to-end run and verification

No new code — this task runs the migration for real and checks every spec claim. **Stop and investigate at any deviation; do not improvise fixes to plugin data.**

- [ ] **Step 1: Run migration 005 against wizards**

Run: `cargo xtask migrate plugins/wizards 5`
Expected: exactly 10 `wrote .../plugins/wizards/cards/<name>.ron` lines: Forest, Island, Mountain, Plains, Snow-Covered Forest, Snow-Covered Island, Snow-Covered Mountain, Snow-Covered Plains, Snow-Covered Swamp, Swamp (sorted order). No Wastes.

- [ ] **Step 2: Inspect the diff**

Run: `jj diff --stat plugins/`
Expected: exactly those 10 files changed.

Run: `diff plugins/builtin/cards/Plains.ron plugins/wizards/cards/Plains.ron`
Expected: **identical** — the template output matches the hand-written builtin style byte for byte.

Run: `cat "plugins/wizards/cards/Snow-Covered Plains.ron"`
Expected:

```ron
Normal(
    name: "Snow-Covered Plains",
    supertypes: [Basic, Snow],
    types: [Land],
    subtypes: [Plains],
)
```

Run: `head -1 plugins/wizards/cards/Wastes.ron "plugins/wizards/cards/Snow-Covered Wastes.ron"`
Expected: both still `Todo(`.

- [ ] **Step 3: Validate through the reader**

Run: `cargo xtask validate plugins/wizards`
Expected: `10 valid, 30871 todos skipped, 0 invalid`, exit 0.

Run: `cargo xtask card plugins/wizards "Snow-Covered Plains"`
Expected: the expanded `Normal(CardFace { ... })` with the `Plains` subtype resolved through the builtin prelude — this is the prelude working end to end.

- [ ] **Step 4: Idempotency**

Run: `cargo xtask migrate plugins/wizards 5`
Expected: zero `wrote` lines, `jj diff --stat plugins/` unchanged from Step 2.

- [ ] **Step 5: Migrations 0–4 unaffected**

Run: `cargo xtask migrate plugins/wizards` (full run — parses AllPrintings, takes a while)
Expected: `jj diff --stat plugins/` still shows only the same 10 files. Any other churn means a regression in the shared module move — stop and investigate.

- [ ] **Step 6: Full suite**

Run: `cargo test --workspace && cargo clippy --workspace && cargo fmt && jj st`
Expected: everything green; working copy contains the full feature.

---

## Addendum (2026-06-06): Tasks 10–11 — `_006_vanilla_creatures`

Per the spec addendum of the same date. Tasks 1–9 are complete and verified.

### Task 10: `_006_vanilla_creatures` + shared convert loop

**Files:**
- Modify: `crates/deckmaste_migrations/src/migrations/card_todo.rs` (add `convert_todos`)
- Modify: `crates/deckmaste_migrations/src/migrations/_005_basic_lands.rs` (apply() delegates to it)
- Modify: `crates/deckmaste_migrations/src/migrations/_006_vanilla_creatures.rs` (replace the stub entirely)

- [ ] **Step 1: Extract the shared walk loop into `card_todo.rs`**

`_005`'s `apply` body (flat scan → todo gate → parse → convert → RON guard → write) is the loop every todo-converting migration shares. Move it to the bottom of `card_todo.rs` (before the tests module):

```rust
/// Walks the plugin's cards directory and overwrites every todo for which
/// `convert` produces a finished definition. cards/ is flat: everything
/// `_004` writes goes through `card_file`, one path segment per card, so
/// no recursion here.
pub(super) fn convert_todos(
    plugin: &PluginLayout,
    convert: impl Fn(&CardFile) -> anyhow::Result<Option<String>>,
) -> anyhow::Result<()> {
    let cards_dir = plugin.cards_dir()?;
    let mut paths: Vec<_> = std::fs::read_dir(&cards_dir)?
        .map(|entry| entry.map(|e| e.path()))
        .collect::<Result<_, _>>()?;
    paths.sort();

    for path in paths {
        if path.extension().is_none_or(|ext| ext != "ron") || !path.is_file() {
            continue;
        }
        let source = std::fs::read_to_string(&path)?;
        if !deckmaste_core::plugin::is_todo_source(&source) {
            continue;
        }
        let card: CardFile = crate::ron_output::ron_options()
            .from_str(&source)
            .with_context(|| format!("parsing todo {}", path.display()))?;
        let Some(definition) = convert(&card)? else {
            continue;
        };

        // Cheap guard: the output must still be valid RON. Bare idents
        // like `Plains` are deliberately unresolved here -- only the
        // macro-aware reader (`cargo xtask validate`) can judge them.
        ron::value::RawValue::from_ron(&definition)
            .with_context(|| format!("invalid render for {}", path.display()))?;
        std::fs::write(&path, definition)?;
        eprintln!("wrote {}", path.display());
    }
    Ok(())
}
```

`card_todo.rs` gains `use anyhow::Context;` and `use crate::layout::PluginLayout;`.

`_005_basic_lands.rs`: delete its `apply` loop body (and now-unused imports — `anyhow::Context` goes; the compiler confirms) and replace with:

```rust
impl super::Migration for BasicLands {
    fn apply(&self, plugin: &PluginLayout) -> anyhow::Result<()> {
        super::card_todo::convert_todos(plugin, |card| {
            Ok(basic_land_face(card).map(render_land))
        })
    }
}
```

Run: `cargo test -p deckmaste_migrations` — all green (pure refactor so far).

- [ ] **Step 2: Write `_006` failing tests**

Replace `crates/deckmaste_migrations/src/migrations/_006_vanilla_creatures.rs` entirely with stubs + tests (fixtures are real wizards todo contents):

```rust
use deckmaste_core::Ident;
use serde::Serialize;

use super::card_todo::{CardFaceTodo, CardFile, Stat};
use crate::layout::PluginLayout;

/// A todo is a convertible vanilla creature when it's a single normal face
/// with Creature among its types, no rules text, and plain numeric stats.
/// `*` stats are characteristic-defining abilities in disguise, so they
/// stay todos with the other text-bearing cards.
fn vanilla_creature_face(card: &CardFile) -> Option<&CardFaceTodo> {
    todo!()
}

/// The finished definition in the builtin/cards house style: ident arrays
/// inline, multi-symbol mana costs chopped like the hand-written
/// Grizzly Bears.
fn render_creature(face: &CardFaceTodo) -> anyhow::Result<String> {
    todo!()
}

pub(super) struct VanillaCreatures;

impl super::Migration for VanillaCreatures {
    fn apply(&self, _plugin: &PluginLayout) -> anyhow::Result<()> { todo!() }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::ron_output::ron_options;

    fn todo(source: &str) -> CardFile { ron_options().from_str(source).unwrap() }

    fn render(source: &str) -> String {
        let card = todo(source);
        let face = vanilla_creature_face(&card).expect("fixture converts");
        render_creature(face).unwrap()
    }

    const GRIZZLY_BEARS: &str = r#"Todo(
    layout: "normal",
    faces: [
        (
            name: "Grizzly Bears",
            mana_cost: [
                Generic(1),
                Green,
            ],
            types: ["Creature"],
            subtypes: ["Bear"],
            power: 2,
            toughness: 2,
        ),
    ],
)
"#;

    /// Multi-symbol mana costs chop one per line — byte-identical to the
    /// hand-written builtin/cards/Grizzly Bears.ron.
    #[test]
    fn grizzly_bears_matches_builtin() {
        assert_eq!(
            render(GRIZZLY_BEARS),
            r#"Normal(
    name: "Grizzly Bears",
    mana_cost: [
        Generic(1),
        Green,
    ],
    types: [Creature],
    subtypes: [Bear],
    power: 2,
    toughness: 2,
)
"#
        );
    }

    /// Single-symbol costs stay on one line; multi-type arrays are inline.
    #[test]
    fn artifact_creature() {
        let golem = r#"Todo(
    layout: "normal",
    faces: [
        (
            name: "Obsianus Golem",
            mana_cost: [Generic(6)],
            types: [
                "Artifact",
                "Creature",
            ],
            subtypes: ["Golem"],
            power: 4,
            toughness: 6,
        ),
    ],
)
"#;
        assert_eq!(
            render(golem),
            r#"Normal(
    name: "Obsianus Golem",
    mana_cost: [Generic(6)],
    types: [Artifact, Creature],
    subtypes: [Golem],
    power: 4,
    toughness: 6,
)
"#
        );
    }

    /// No mana cost at all (the line is omitted), a color indicator, and
    /// Land Creature types.
    #[test]
    fn dryad_arbor() {
        let dryad = r#"Todo(
    layout: "normal",
    faces: [
        (
            name: "Dryad Arbor",
            color_indicator: [Green],
            types: [
                "Land",
                "Creature",
            ],
            subtypes: [
                "Forest",
                "Dryad",
            ],
            power: 1,
            toughness: 1,
        ),
    ],
)
"#;
        assert_eq!(
            render(dryad),
            r#"Normal(
    name: "Dryad Arbor",
    color_indicator: [Green],
    types: [Land, Creature],
    subtypes: [Forest, Dryad],
    power: 1,
    toughness: 1,
)
"#
        );
    }

    /// Supertypes render inline when present.
    #[test]
    fn legendary_vanilla() {
        let jedit = r#"Todo(
    layout: "normal",
    faces: [
        (
            name: "Jedit Ojanen",
            mana_cost: [
                Generic(4),
                White,
                White,
                Blue,
            ],
            supertypes: ["Legendary"],
            types: ["Creature"],
            subtypes: [
                "Cat",
                "Warrior",
            ],
            power: 5,
            toughness: 5,
        ),
    ],
)
"#;
        assert_eq!(
            render(jedit),
            r#"Normal(
    name: "Jedit Ojanen",
    mana_cost: [
        Generic(4),
        White,
        White,
        Blue,
    ],
    supertypes: [Legendary],
    types: [Creature],
    subtypes: [Cat, Warrior],
    power: 5,
    toughness: 5,
)
"#
        );
    }

    #[test]
    fn skips_creatures_with_text() {
        let courier = r#"Todo(
    layout: "normal",
    faces: [
        (
            name: "Transguild Courier",
            mana_cost: [Generic(4)],
            types: [
                "Artifact",
                "Creature",
            ],
            subtypes: ["Golem"],
            text: ["Transguild Courier is all colors."],
            power: 3,
            toughness: 3,
        ),
    ],
)
"#;
        assert!(vanilla_creature_face(&todo(courier)).is_none());
    }

    /// `*` stats mean a characteristic-defining ability: not vanilla even
    /// with no other text.
    #[test]
    fn skips_star_stats() {
        let goyf = r#"Todo(
    layout: "normal",
    faces: [
        (
            name: "Goyfish",
            mana_cost: [
                Generic(1),
                Green,
            ],
            types: ["Creature"],
            subtypes: ["Lhurgoyf"],
            power: Other("*"),
            toughness: Other("1+*"),
        ),
    ],
)
"#;
        assert!(vanilla_creature_face(&todo(goyf)).is_none());
    }

    #[test]
    fn skips_noncreatures_and_statless() {
        let land = r#"Todo(
    layout: "normal",
    faces: [
        (
            name: "Wastes",
            supertypes: ["Basic"],
            types: ["Land"],
            text: ["{T}: Add {C}."],
        ),
    ],
)
"#;
        assert!(vanilla_creature_face(&todo(land)).is_none());
    }
}
```

Run: `cargo test -p deckmaste_migrations _006` — failures/panics expected.

- [ ] **Step 3: Implement**

```rust
fn vanilla_creature_face(card: &CardFile) -> Option<&CardFaceTodo> {
    let CardFile::Todo { layout, faces } = card;
    match faces.as_slice() {
        [face]
            if *layout == "normal"
                && face.types.iter().any(|t| *t == "Creature")
                && face.text.is_empty()
                && matches!(face.power, Some(Stat::Number(_)))
                && matches!(face.toughness, Some(Stat::Number(_)))
                && face.loyalty.is_none()
                && face.defense.is_none() =>
        {
            Some(face)
        }
        _ => None,
    }
}

/// One leaf value (mana symbol, color, stat) spelled by the shared ron
/// config — tuple members stay inline, so `Hybrid(Generic(2), White)`
/// keeps its canonical spacing. The template owns the file shape; ron
/// only spells the tokens.
fn leaf<T: Serialize>(value: &T) -> anyhow::Result<String> {
    Ok(crate::ron_output::ron_options()
        .to_string_pretty(value, crate::ron_output::pretty_config())?)
}

/// `    field: [a, b],` — ident arrays stay inline; nothing for `[]`.
fn ident_line(out: &mut String, field: &str, idents: &[Ident]) {
    use std::fmt::Write;

    if !idents.is_empty() {
        writeln!(out, "    {field}: [{}],", idents.join(", ")).unwrap();
    }
}

fn render_creature(face: &CardFaceTodo) -> anyhow::Result<String> {
    use std::fmt::Write;

    let mut out = String::new();
    writeln!(out, "Normal(")?;
    writeln!(out, "    name: {:?},", face.name)?;
    match &*face.mana_cost {
        [] => {}
        [symbol] => writeln!(out, "    mana_cost: [{}],", leaf(symbol)?)?,
        symbols => {
            writeln!(out, "    mana_cost: [")?;
            for symbol in symbols {
                writeln!(out, "        {},", leaf(symbol)?)?;
            }
            writeln!(out, "    ],")?;
        }
    }
    if !face.color_indicator.is_empty() {
        let colors: Vec<String> = face
            .color_indicator
            .iter()
            .map(leaf)
            .collect::<anyhow::Result<_>>()?;
        writeln!(out, "    color_indicator: [{}],", colors.join(", "))?;
    }
    ident_line(&mut out, "supertypes", &face.supertypes);
    ident_line(&mut out, "types", &face.types);
    ident_line(&mut out, "subtypes", &face.subtypes);
    if let Some(power) = &face.power {
        writeln!(out, "    power: {},", leaf(power)?)?;
    }
    if let Some(toughness) = &face.toughness {
        writeln!(out, "    toughness: {},", leaf(toughness)?)?;
    }
    writeln!(out, ")")?;
    Ok(out)
}

impl super::Migration for VanillaCreatures {
    fn apply(&self, plugin: &PluginLayout) -> anyhow::Result<()> {
        super::card_todo::convert_todos(plugin, |card| {
            vanilla_creature_face(card).map(render_creature).transpose()
        })
    }
}
```

Notes: `leaf(power)` on `Stat::Number(2)` renders untagged → `2` (already pinned by `card_todo`'s `stats` test). If `writeln!` into `String` upsets clippy (infallible `fmt::Write`), `.unwrap()` or `?` per its suggestion — match whichever it asks for. If `&*face.mana_cost` doesn't coerce to `[ManaSymbol]`, use `face.mana_cost.as_slice()` or iterate — check how `ManaCost` derefs (one_line_if_single already coerces it to `&[T]`, so deref exists).

Run: `cargo test -p deckmaste_migrations _006` — 7 PASS; then the full crate suite (the `_005` tests prove the `convert_todos` extraction intact).

- [ ] **Step 4: Checkpoint**

`cargo test -p deckmaste_migrations && cargo clippy -p deckmaste_migrations && cargo +nightly fmt` — green; only pre-existing warnings. NO migration runs against `plugins/` (Task 11 does that).

### Task 11: End-to-end run + verification (controller-run)

- [ ] `cargo xtask migrate plugins/wizards 6` → expect exactly 338 `wrote` lines.
- [ ] `diff "plugins/builtin/cards/Grizzly Bears.ron" "plugins/wizards/cards/Grizzly Bears.ron"` → identical.
- [ ] Spot-check `Dryad Arbor.ron`, `Crookshank Kobolds.ron`, `Memnite.ron`.
- [ ] `cargo xtask validate plugins/wizards` → `348 valid, 30533 todos skipped, 0 invalid`.
- [ ] Re-run migration 6 → zero `wrote` lines (idempotent).
- [ ] `cargo test --workspace` green; `cargo +nightly fmt`.

## Self-Review (performed at planning time)

- **Spec coverage:** §1 shared module → Tasks 1–2; §2 migration → Task 3; §3 prelude (incl. `cargo card` convention and last-plugin-wins layering — user decision after discovering `_003`'s generated subtypes overlap builtin's six declarations, which made strict no-shadowing unable to load wizards at all) → Tasks 4, 6; §4 validation + builtin `#[test]` + defaults → Tasks 5, 6, 8; §5 xtask (pure dispatcher, in-process, release alias, aliases/bins stay) → Tasks 6–8; §6 verification → Task 9 mirrors it step for step.
- **Placeholders:** Task 4 Step 3 abbreviates the unchanged interior of `load_onto` (`...existing body...`) deliberately — the surrounding text states exactly which three lines change and that the rest moves verbatim; the code already exists in the file being edited.
- **Type consistency:** `validate_plugin(&Path) -> anyhow::Result<Validation>` is used identically in Tasks 5, 6; `cli::card`/`cli::validate`/`cli::run` all take `IntoIterator<Item = OsString>` full-argv (Task 8's dispatcher matches); `card_todo` items are `pub(super)` and only used within `migrations` (Tasks 2–3).
