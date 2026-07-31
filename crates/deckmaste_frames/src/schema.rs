//! The frame authoring schema: [`FrameSpec`] (English templates with typed
//! holes, plus the guard that decides when one applies) and
//! [`ConstructorFrames`] (the parallel catalog for raw `deckmaste_core`
//! constructors — canon RON is 83.5% raw constructors, so this catalog is
//! most of the lexicon, not an appendix; see
//! `docs/superpowers/research/2026-07-30-macro-frames/macro-schema-census.md`
//! §6).
//!
//! A frame's text carries two hole sigils, read by a later stage in this
//! crate: `~` (self-reference) and `<Param(i)>` (positional). This module
//! only carries the strings through serde — it never parses them — so they
//! must round-trip byte-for-byte.
//!
//! **The guard model.** A guard is a set of param pre-bindings (`when: [(0,
//! "You")]` — param index paired with the *normalized RON serialization* of
//! the required constant) plus an optional syntactic-position key. At render
//! time the most-specific satisfied guard wins; at match time a guarded
//! frame recovers its pre-bound params even though no hole for them surfaces
//! in the text. This is per-macro (or per-constructor) data rather than a
//! global rule because the alternation it encodes doesn't reduce to one: the
//! corpus shows imperative "Gain N life" at 0 attestations against 236 "You
//! gain N life", while imperative "Draw …" is 641 against a mere 58 "You
//! draw" — opposite skews for verbs that look symmetric on paper (see
//! `docs/superpowers/research/2026-07-30-macro-frames/corpus-alternation.md`).

use std::path::Path;
use std::path::PathBuf;

use anyhow::Context as _;
use serde::Deserialize;
use serde::Deserializer;
use serde::Serialize;
use serde::Serializer;

/// Where a frame is anchored syntactically — part of a guard's specificity,
/// alongside its `when` param pre-bindings. `Main` is a top-level sentence
/// (a `OneShotEffect`'s own clause); `Trigger` is a subordinate clause after
/// a trigger/condition lead-in ("Whenever …, …"). The corpus alternation
/// data motivating this split (position changes a verb's imperative rate by
/// double digits for several verbs — `tap`, `reveal`, `discard`, `create`,
/// `draw`, `put`, `mill`) lives in `corpus-alternation.md` §4.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, Deserialize, Serialize)]
pub enum FramePosition {
    Main,
    Trigger,
}

/// One English rendering of a macro or constructor: `text` carries the hole
/// sigils (`~`, `<Param(i)>`), `when` is the guard's param pre-bindings
/// (param index paired with the normalized RON serialization of the
/// required constant), and `position` is the guard's optional syntactic-
/// position key. An unguarded frame has both empty/absent.
///
/// Authored two ways in a `frames:` list, and both must deserialize (and
/// round-trip) to this same struct: a bare string is sugar for an unguarded
/// frame (`when: []`, `position: None`) —
///
/// ```ron
/// frames: ["draw <Param(1)> cards"]
/// ```
///
/// — or the full struct form, for a guarded frame —
///
/// ```ron
/// frames: [(text: "draw <Param(1)> cards", when: [(0, "You")], position: Main)]
/// ```
///
/// The sugar is implemented by [`FrameSpecRepr`] below (an untagged serde
/// intermediate), not by hand-parsing: `MacroDef`'s own deserialization
/// (`macro_ron::set`) is fully serde-driven, so `#[serde(untagged)]` applies
/// directly.
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct FrameSpec {
    pub text: String,
    pub when: Vec<(usize, String)>,
    pub position: Option<FramePosition>,
}

impl FrameSpec {
    /// An unguarded frame with no syntactic-position key — the shape every
    /// bare-string sugar produces. Exposed as a test helper: Tasks 4-8 build
    /// fixture frames without spelling out the guard fields every time.
    #[must_use]
    pub fn bare(text: &str) -> FrameSpec {
        FrameSpec {
            text: text.to_string(),
            when: Vec::new(),
            position: None,
        }
    }

    /// Whether this frame carries no guard at all — the condition under
    /// which [`Serialize`] takes the bare-string spelling instead of the
    /// full struct form.
    fn is_unguarded(&self) -> bool {
        self.when.is_empty() && self.position.is_none()
    }
}

/// The untagged serde intermediate implementing the frame-sugar contract: a
/// bare string is an unguarded frame; anything else is the full struct form.
/// `serde(untagged)` tries each variant in order and keeps the first that
/// matches the input shape, so a RON string literal takes `Bare` and a RON
/// struct/tuple takes `Full` — no explicit tag needed either way.
#[derive(Debug, Clone, Deserialize, Serialize)]
#[serde(untagged)]
enum FrameSpecRepr {
    Bare(String),
    Full {
        text: String,
        #[serde(default)]
        when: Vec<(usize, String)>,
        #[serde(default)]
        position: Option<FramePosition>,
    },
}

impl From<FrameSpecRepr> for FrameSpec {
    fn from(repr: FrameSpecRepr) -> Self {
        match repr {
            FrameSpecRepr::Bare(text) => FrameSpec::bare(&text),
            FrameSpecRepr::Full {
                text,
                when,
                position,
            } => FrameSpec {
                text,
                when,
                position,
            },
        }
    }
}

impl From<&FrameSpec> for FrameSpecRepr {
    fn from(spec: &FrameSpec) -> Self {
        if spec.is_unguarded() {
            FrameSpecRepr::Bare(spec.text.clone())
        } else {
            FrameSpecRepr::Full {
                text: spec.text.clone(),
                when: spec.when.clone(),
                position: spec.position,
            }
        }
    }
}

impl<'de> Deserialize<'de> for FrameSpec {
    fn deserialize<D: Deserializer<'de>>(deserializer: D) -> Result<Self, D::Error> {
        FrameSpecRepr::deserialize(deserializer).map(FrameSpec::from)
    }
}

impl Serialize for FrameSpec {
    fn serialize<S: Serializer>(&self, serializer: S) -> Result<S::Ok, S::Error> {
        FrameSpecRepr::from(self).serialize(serializer)
    }
}

/// One raw `deckmaste_core` constructor's entry in the frame catalog:
/// `constructor` is the bare variant name as written in RON (`"DealDamage"`,
/// `"GainLife"`), `params` names each positional hole's type in the order
/// the catalog's frames reference them (`<Param(0)>` is `params[0]`, and so
/// on), and `frames` are its English renderings, same shape and guard model
/// as [`MacroDef::frames`](https://docs.rs/macro_ron) (`FrameSpec` is
/// shared between the two — a guard means the same thing whether it guards
/// a macro's frame or a constructor's).
#[derive(Debug, Clone, PartialEq, Eq, Deserialize, Serialize)]
pub struct ConstructorFrames {
    pub constructor: String,
    pub params: Vec<String>,
    pub frames: Vec<FrameSpec>,
}

/// Reads every `constructors.ron`-shaped catalog file under `dir` (each file
/// holds a `[...]`-list of [`ConstructorFrames`], exactly like
/// `plugins/builtin/frames/constructors.ron`) and concatenates their
/// entries. An absent directory reads as empty, matching the macro loader's
/// own convention (`deckmaste_cards::plugin::ron_files_recursive`) for a
/// plugin layer that hasn't opted in yet.
///
/// # Errors
/// If `dir` exists but isn't readable, or a file under it isn't valid UTF-8
/// or doesn't parse as `Vec<ConstructorFrames>`.
pub fn load_constructor_frames(dir: &Path) -> anyhow::Result<Vec<ConstructorFrames>> {
    let mut entries = Vec::new();
    for path in ron_files_recursive(dir)? {
        let source = std::fs::read_to_string(&path)
            .with_context(|| format!(r#"reading "{}""#, path.display()))?;
        let file: Vec<ConstructorFrames> = ron::from_str(&source)
            .with_context(|| format!(r#"parsing "{}" as constructor frames"#, path.display()))?;
        entries.extend(file);
    }
    Ok(entries)
}

/// The `.ron` files under `dir` at any depth, sorted; an absent directory is
/// empty. Mirrors `deckmaste_cards::plugin::ron_files_recursive` (kept as a
/// private copy rather than a shared dependency: `deckmaste_frames` must not
/// depend on `deckmaste_cards`, which itself depends on `macro_ron`, which
/// depends on `deckmaste_frames` for this very schema — see the crate-level
/// dependency note on [`load_constructor_frames`]).
fn ron_files_recursive(dir: &Path) -> anyhow::Result<Vec<PathBuf>> {
    if !dir.exists() {
        return Ok(vec![]);
    }
    let context = || format!(r#"reading "{}""#, dir.display());
    let mut files = Vec::new();
    let mut subdirs = Vec::new();
    for entry in dir.read_dir().with_context(context)? {
        let entry = entry.with_context(context)?;
        let path = entry.path();
        if entry.file_type().with_context(context)?.is_dir() {
            subdirs.push(path);
        } else if path.extension().is_some_and(|ext| ext == "ron") && path.is_file() {
            files.push(path);
        }
    }
    subdirs.sort();
    for subdir in subdirs {
        files.extend(ron_files_recursive(&subdir)?);
    }
    files.sort();
    Ok(files)
}

#[cfg(test)]
mod tests {
    use super::*;

    fn opts() -> ron::Options {
        ron::Options::default()
    }

    #[test]
    fn bare_string_sugar_is_an_unguarded_frame() {
        let spec: FrameSpec = opts().from_str(r#""draw <Param(1)> cards""#).unwrap();
        assert_eq!(spec.text, "draw <Param(1)> cards");
        assert!(spec.when.is_empty());
        assert_eq!(spec.position, None);
    }

    #[test]
    fn full_form_carries_guard_and_position() {
        let spec: FrameSpec = opts()
            .from_str(r#"(text: "draw <Param(1)> cards", when: [(0, "You")], position: Main)"#)
            .unwrap();
        assert_eq!(spec.text, "draw <Param(1)> cards");
        assert_eq!(spec.when, vec![(0, "You".to_string())]);
        assert_eq!(spec.position, Some(FramePosition::Main));
    }

    #[test]
    fn bare_form_round_trips_through_serialize() {
        let spec = FrameSpec::bare("target <Param(0)>");
        let text = opts().to_string(&spec).unwrap();
        let back: FrameSpec = opts().from_str(&text).unwrap();
        assert_eq!(spec, back);
        // The bare spelling stays minimal: no `when`/`position` fields leak
        // into an unguarded frame's serialized form.
        assert_eq!(text, r#""target <Param(0)>""#);
    }

    #[test]
    fn guarded_form_round_trips_through_serialize() {
        let spec = FrameSpec {
            text: "draw <Param(1)> cards".into(),
            when: vec![(0, "You".to_string())],
            position: Some(FramePosition::Trigger),
        };
        let text = opts().to_string(&spec).unwrap();
        let back: FrameSpec = opts().from_str(&text).unwrap();
        assert_eq!(spec, back);
    }

    #[test]
    fn constructor_frames_catalog_entry_deserializes() {
        let source = r#"[
            (constructor: "DealDamage", params: ["Reference", "Count", "Reference"],
             frames: ["<Param(0)> deals <Param(1)> damage to <Param(2)>"]),
        ]"#;
        let catalog: Vec<ConstructorFrames> = opts().from_str(source).unwrap();
        assert_eq!(catalog.len(), 1);
        assert_eq!(catalog[0].constructor, "DealDamage");
        assert_eq!(catalog[0].params, vec!["Reference", "Count", "Reference"]);
        assert_eq!(
            catalog[0].frames,
            vec![FrameSpec::bare(
                "<Param(0)> deals <Param(1)> damage to <Param(2)>"
            )]
        );
    }

    #[test]
    fn load_constructor_frames_reads_the_seeded_catalog() {
        let dir = Path::new(env!("CARGO_MANIFEST_DIR")).join("../../plugins/builtin/frames");
        let catalog = load_constructor_frames(&dir).unwrap();
        let names: Vec<&str> = catalog.iter().map(|c| c.constructor.as_str()).collect();
        assert!(names.contains(&"DealDamage"), "{names:?}");
        assert!(names.contains(&"GainLife"), "{names:?}");
        assert!(names.contains(&"Target"), "{names:?}");
    }

    #[test]
    fn load_constructor_frames_of_absent_dir_is_empty() {
        let dir = Path::new(env!("CARGO_MANIFEST_DIR")).join("does/not/exist");
        assert_eq!(load_constructor_frames(&dir).unwrap(), vec![]);
    }
}
