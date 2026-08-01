//! The frame authoring schema: [`FrameSpec`] (English templates with typed
//! holes, plus the guard that decides when one applies) and
//! [`ConstructorFrames`] (the parallel catalog for raw `deckmaste_core`
//! constructors — canon RON is 83.5% raw constructors, so this catalog is
//! most of the lexicon, not an appendix).
//!
//! Lives in `macro_ron`, not the `deckmaste_frames` bridge crate: schema
//! lives with the data (`MacroDef.frames` needs this type; see
//! [`crate::set::MacroDef`]), and the engine (`deckmaste_frames`, which will
//! need `macro_ron::MacroSet` itself to actually expand macro bodies)
//! depends on the schema, not the other way around — the reverse arrow
//! would cycle. This module has no dependency beyond `serde` and `ron`
//! (already `macro_ron` dependencies) and no English/rendering knowledge.
//!
//! A frame's text carries two hole sigils, read by a later stage
//! (`deckmaste_frames`): `~` (self-reference) and `<Param(i)>` (positional).
//! This module only carries the strings through serde — it never parses
//! them — so they must round-trip byte-for-byte.
//!
//! **The guard model.** A guard is a set of param pre-bindings (`when: [(0,
//! "You")]` — param index paired with a RON spelling of the required
//! constant) plus an optional syntactic-position key. At render time the
//! most-specific satisfied guard wins; at match time a guarded frame
//! recovers its pre-bound params even though no hole for them surfaces in
//! the text. Guard satisfaction itself is defined on *fully-expanded
//! canonical form* — parse the guard string as a RON term, run
//! `expand_all`, and compare the resulting `View` against the card-side
//! argument run through the same function — so the stored string may be any
//! parseable RON spelling of the constant (`"Exactly(1)"`, not necessarily
//! some single "normalized" text); that normalizer is a later stage's job
//! (Task 4), not this module's — this module stores guard strings verbatim,
//! with no semantics attached.
//!
//! This is per-macro (or per-constructor) data rather than a global rule
//! because the alternation it encodes doesn't reduce to one: the corpus
//! shows imperative "Gain N life" at 0 attestations against 236 "You gain N
//! life", while imperative "Draw …" is 641 against a mere 58 "You draw" —
//! opposite skews for verbs that look symmetric on paper (see
//! `docs/superpowers/research/2026-07-30-macro-frames/corpus-alternation.md`).

use std::fmt;
use std::io;
use std::path::Path;
use std::path::PathBuf;

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
/// (param index paired with a RON spelling of the required constant,
/// compiled to expanded canonical form at guard-check time — see the
/// module doc), and `position` is the guard's optional syntactic-position
/// key. An unguarded frame has both empty/absent.
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
/// (`crate::set`) is fully serde-driven, so `#[serde(untagged)]` applies
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
    ///
    /// Public because a guard also decides whether a frame is a *complete*
    /// rendering: a guarded frame pre-binds params, so its text omits them
    /// by construction (`Draws`'s `You`-guarded `"draw <Param(1)> cards"`
    /// never spells the subject). Any consumer that projects a frame's text
    /// as if it were the whole rendering — `cargo xtask macro templates`'s
    /// D10 projection is the one in tree — must consult this first.
    #[must_use]
    pub fn is_unguarded(&self) -> bool {
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

/// **Asymmetric by design, not merely permissively lenient**: an unguarded
/// `FrameSpec` (`when` empty, `position` absent) always serializes to the
/// bare string spelling, never the padded struct form — the guarded case is
/// the only one that pays for the full `(text: ..., when: ..., position:
/// ...)` shape. A reader who lands here directly (rather than via the
/// module doc) should not expect `Serialize`/`Deserialize` to be mirror
/// images of a single canonical shape.
impl Serialize for FrameSpec {
    fn serialize<S: Serializer>(&self, serializer: S) -> Result<S::Ok, S::Error> {
        FrameSpecRepr::from(self).serialize(serializer)
    }
}

/// The English category a constructor entry's frames are parsed at and
/// registered under. A schema-owned mirror of
/// `deckmaste_english::FragmentKind` — duplicated, not imported, because
/// this module has no dependency beyond `serde`/`ron` (see the module doc)
/// and must not gain one merely to name the category a frame belongs to.
/// The bridge crate carries the exhaustive conversion
/// (`deckmaste_frames::lexicon::fragment_kind_of`; not a `From` impl —
/// both this type and `FragmentKind` are foreign to that crate, and to
/// every crate that could host one); this module never interprets the
/// variant, only carries it through serde, exactly as it already does for
/// hole sigils and guard spellings.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, Deserialize, Serialize)]
pub enum FrameKind {
    Nominal,
    Sentence,
    Cost,
    KeywordLine,
    Ability,
}

/// One raw `deckmaste_core` constructor's entry in the frame catalog:
/// `constructor` is the bare variant name as written in RON (`"DealDamage"`,
/// `"GainLife"`), `params` names each positional hole's type in the order
/// the catalog's frames reference them (`<Param(0)>` is `params[0]`, and so
/// on), `frames` are its English renderings, same shape and guard model
/// as [`MacroDef::frames`](crate::set::MacroDef) (`FrameSpec` is shared
/// between the two — a guard means the same thing whether it guards a
/// macro's frame or a constructor's), and `kind` is the single category this
/// entry's frames register at. `kind` is required, not defaulted: a
/// constructor entry has no macro `kinds:` to read a category off (unlike
/// [`MacroDef`](crate::set::MacroDef)), so there is nothing to guess it
/// from, and registering an entry at every category it happens to parse
/// cleanly at costs several entries where one is meant — see
/// `deckmaste_frames::lexicon::Lexicon::assemble`.
#[derive(Debug, Clone, PartialEq, Eq, Deserialize, Serialize)]
pub struct ConstructorFrames {
    pub constructor: String,
    pub params: Vec<String>,
    pub frames: Vec<FrameSpec>,
    pub kind: FrameKind,
}

/// Why [`load_constructor_frames`] failed: an I/O error reading a directory
/// or file, or a RON parse error, each carrying the path it happened at.
/// `macro_ron`-native (no `anyhow` dependency here) — implements
/// [`std::error::Error`] so `anyhow`-using callers (every current caller)
/// still get `?`-conversion for free via `anyhow`'s blanket `From<E: Error>`.
#[derive(Debug)]
pub enum LoadError {
    Io {
        path: PathBuf,
        source: io::Error,
    },
    Parse {
        path: PathBuf,
        // Boxed: `ron::error::SpannedError` is ~128 bytes, which would make
        // `Result<_, LoadError>` itself large enough to trip
        // `clippy::result_large_err` at every call site.
        source: Box<ron::error::SpannedError>,
    },
}

impl fmt::Display for LoadError {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            LoadError::Io { path, source } => {
                write!(f, r#"reading "{}": {source}"#, path.display())
            }
            LoadError::Parse { path, source } => {
                write!(
                    f,
                    r#"parsing "{}" as constructor frames: {source}"#,
                    path.display()
                )
            }
        }
    }
}

impl std::error::Error for LoadError {
    fn source(&self) -> Option<&(dyn std::error::Error + 'static)> {
        Some(match self {
            LoadError::Io { source, .. } => source,
            LoadError::Parse { source, .. } => source.as_ref(),
        })
    }
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
pub fn load_constructor_frames(dir: &Path) -> Result<Vec<ConstructorFrames>, LoadError> {
    let mut entries = Vec::new();
    for path in ron_files_recursive(dir)? {
        let source = std::fs::read_to_string(&path).map_err(|source| LoadError::Io {
            path: path.clone(),
            source,
        })?;
        let file: Vec<ConstructorFrames> =
            ron::from_str(&source).map_err(|source| LoadError::Parse {
                path: path.clone(),
                source: Box::new(source),
            })?;
        entries.extend(file);
    }
    Ok(entries)
}

/// The `.ron` files under `dir` at any depth, sorted; an absent directory is
/// empty. Mirrors `deckmaste_cards::plugin::ron_files_recursive` (kept as a
/// private copy rather than a shared dependency: `macro_ron` must not
/// depend on `deckmaste_cards`, which itself depends on `macro_ron` — that
/// would cycle just as surely as the arrow this module's relocation was
/// fixing).
fn ron_files_recursive(dir: &Path) -> Result<Vec<PathBuf>, LoadError> {
    if !dir.exists() {
        return Ok(vec![]);
    }
    let io_err = |source| LoadError::Io {
        path: dir.to_path_buf(),
        source,
    };
    let mut files = Vec::new();
    let mut subdirs = Vec::new();
    for entry in dir.read_dir().map_err(io_err)? {
        let entry = entry.map_err(io_err)?;
        let path = entry.path();
        if entry
            .file_type()
            .map_err(|source| LoadError::Io {
                path: path.clone(),
                source,
            })?
            .is_dir()
        {
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
             frames: ["<Param(0)> deals <Param(1)> damage to <Param(2)>"], kind: Sentence),
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
        assert_eq!(catalog[0].kind, FrameKind::Sentence);
    }

    /// `kind:` narrows a catalog entry to one registration category (an
    /// unscoped entry costs several entries where one is meant), so it must
    /// be required, not defaulted — an entry authored without it is a
    /// build-time mistake, not a legal "no preference" reading.
    #[test]
    fn constructor_frames_missing_kind_fails_to_deserialize() {
        let source = r#"[
            (constructor: "DealDamage", params: ["Reference", "Count", "Reference"],
             frames: ["<Param(0)> deals <Param(1)> damage to <Param(2)>"]),
        ]"#;
        let error = opts()
            .from_str::<Vec<ConstructorFrames>>(source)
            .expect_err("an entry with no `kind:` field must fail to deserialize");
        let message = error.to_string();
        assert!(
            message.to_lowercase().contains("kind"),
            "expected a field-naming error mentioning `kind`, got: {message}"
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

    #[test]
    fn load_error_implements_std_error_with_a_source() {
        // The `anyhow`-free contract Ruling A's controller sub-ruling
        // requires: callers using `anyhow::Result` still get `?` for free
        // via `anyhow`'s blanket `From<E: std::error::Error>`.
        fn assert_error<E: std::error::Error + 'static>() {}
        assert_error::<LoadError>();

        let dir = Path::new(env!("CARGO_MANIFEST_DIR")).join("Cargo.toml");
        let err = load_constructor_frames(&dir).unwrap_err();
        assert!(std::error::Error::source(&err).is_some());
    }
}
