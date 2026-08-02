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
//! **A catalog entry may carry a `body:`** ([`ConstructorFrames::body`]): the
//! value its frames stand for, as a term with `Param(i)` leaves, in exactly
//! the schema and syntax [`MacroDef::body`](crate::set::MacroDef) already
//! uses — so a wording can denote a value whose RON shape is not the flat
//! application of its own name. Filling that term is [`substitute_body`],
//! which is `macro_ron`'s own hole-splicing walk rather than a second one.
//! An entry with no body means the positional application it always did.
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
//! some single "normalized" text); that normalizer is a later stage's job,
//! not this module's — this module stores guard strings verbatim, with no
//! semantics attached.
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

use ron::value::RawValue;
use serde::Deserialize;
use serde::Deserializer;
use serde::Serialize;
use serde::Serializer;

use crate::MacroSet;

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
    /// The params whose holes accept **only an announcement filler** — a
    /// constituent covered by an entry that declares
    /// [`ConstructorFrames::announcement`].
    ///
    /// Authored as the param indices, beside the text that holes them:
    ///
    /// ```ron
    /// (text: "<Param(0)> deals <Param(1)> damage to <Param(2)>", announced: [2])
    /// ```
    ///
    /// It is a restriction on the *filler*, not a change to the hole's own
    /// structural class (`deckmaste_frames`'s `HoleClass`, which the parse
    /// decides): the same hole is a whole-subtree hole either way, and both
    /// wordings above and below stay one authored English string. The
    /// restriction runs in both directions — a listed param's hole rejects a
    /// non-announcement filler and an unlisted param's hole rejects an
    /// announcement one — which is what makes two entries carrying the *same*
    /// frame text over different values (an inline recipient versus one
    /// hoisted into a `targets:` announce list) cover disjoint English
    /// instead of competing for it. Magic's targeting rules are what make
    /// that disjointness real rather than stipulated: a targeted recipient is
    /// always announced [CR#601.2c], so the announced wording and the inline
    /// one never describe the same effect.
    pub announced: Vec<usize>,
}

impl FrameSpec {
    /// An unguarded frame with no syntactic-position key and no announced
    /// holes — the shape every bare-string sugar produces. Exposed as a test
    /// helper: fixture frames are built without spelling out the guard fields
    /// every time.
    #[must_use]
    pub fn bare(text: &str) -> FrameSpec {
        FrameSpec {
            text: text.to_string(),
            when: Vec::new(),
            position: None,
            announced: Vec::new(),
        }
    }

    /// Whether this frame carries no guard at all — no `when` pre-bindings
    /// and no `position` key. Not on its own the condition [`Serialize`]
    /// takes the bare-string spelling under: an unguarded frame with
    /// announced holes still serializes in the full struct form.
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

    /// Whether this frame carries nothing but its text — no guard *and* no
    /// announced holes — which is the condition [`Serialize`] takes the
    /// bare-string spelling under.
    ///
    /// Deliberately not folded into [`is_unguarded`](Self::is_unguarded):
    /// that predicate answers "does the text spell the whole rendering", and
    /// an announced hole still surfaces in the text, so a frame with one is
    /// projectable exactly as an unannounced frame is.
    fn is_bare(&self) -> bool {
        self.is_unguarded() && self.announced.is_empty()
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
        #[serde(default)]
        announced: Vec<usize>,
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
                announced,
            } => FrameSpec {
                text,
                when,
                position,
                announced,
            },
        }
    }
}

impl From<&FrameSpec> for FrameSpecRepr {
    fn from(spec: &FrameSpec) -> Self {
        if spec.is_bare() {
            FrameSpecRepr::Bare(spec.text.clone())
        } else {
            FrameSpecRepr::Full {
                text: spec.text.clone(),
                when: spec.when.clone(),
                position: spec.position,
                announced: spec.announced.clone(),
            }
        }
    }
}

impl<'de> Deserialize<'de> for FrameSpec {
    fn deserialize<D: Deserializer<'de>>(deserializer: D) -> Result<Self, D::Error> {
        FrameSpecRepr::deserialize(deserializer).map(FrameSpec::from)
    }
}

/// **Asymmetric by design, not merely permissively lenient**: a `FrameSpec`
/// carrying nothing but its text — `when` empty, `position` absent,
/// `announced` empty — always serializes to the bare string spelling, never
/// the padded struct form. Only a frame with something the bare string
/// cannot carry pays for the full `(text: ..., when: ..., position: ...,
/// announced: ...)` shape. A reader who lands here directly (rather than via
/// the module doc) should not expect `Serialize`/`Deserialize` to be mirror
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
    /// The value an invocation of this entry stands for, as a body term with
    /// `Param(i)` leaves — the same schema and the same syntax as
    /// [`MacroDef::body`](crate::set::MacroDef), authored bare, never quoted:
    ///
    /// ```ron
    /// body: By(Param(0), GainLife(Param(1))),
    /// ```
    ///
    /// Absent (the common case) means the implicit positional application
    /// `Constructor(Param(0), …, Param(n-1))`, so every entry authored before
    /// this field existed keeps its exact meaning.
    ///
    /// A body makes `constructor` the entry's **name** rather than a promise
    /// that a bare core variant of that spelling exists: the entry denotes
    /// whatever its body denotes, and the name is what diagnostics, the
    /// census, and the `(name, frame_index, origin)` identity read. That is
    /// what lets one English wording stand for a value the core repr does not
    /// spell in the same shape — a subject the repr defaults away, or a
    /// recipient the repr hoists into a `targets:` announce list and reads
    /// back positionally.
    ///
    /// Stored as the body's own RON source text, exactly as
    /// [`MacroDef::body`](crate::set::MacroDef) is (see
    /// [`crate::set::body_text`]); this module attaches no meaning to it, the
    /// same way it carries hole sigils and guard spellings through untouched.
    /// [`substitute_body`] is what fills the holes.
    #[serde(default, deserialize_with = "raw_body")]
    #[serde(
        skip_serializing_if = "Option::is_none",
        serialize_with = "raw_body_out"
    )]
    pub body: Option<String>,
    /// Whether an invocation of this entry **is a target announcement** — a
    /// `TargetSpec`, or a pro-form standing for one, of the kind a `targets:`
    /// list holds [CR#601.2c].
    ///
    /// Read by the filler discipline [`FrameSpec::announced`] describes: this
    /// flag is what makes a constituent count as an announcement, so the
    /// class is catalog data rather than a hardcoded English node shape.
    /// `false` for everything else, which is why it defaults.
    ///
    /// **Declared, never inferred.** An entry is an announcement only by
    /// carrying `announcement: true` — not by its `constructor` spelling, not
    /// by its declared param types, not by its frame text containing the word
    /// "target". The failure mode of a missing declaration is silent, and it
    /// bites the *other* entry: a hole marked
    /// [`announced`](FrameSpec::announced) accepts only a filler some
    /// declared entry covers, so a frame that marks a hole whose intended
    /// filler is undeclared never matches at all. Nothing diagnoses that —
    /// the wording is simply unreachable, and the catalog still assembles.
    ///
    /// Declaring one is a commitment in both directions, so an entry that
    /// *could* be an announcement is not declared one on sight: an unmarked
    /// hole rejects announcement fillers, so as a filler a declared entry's
    /// English fits marked holes and no others. Declare an entry in the same
    /// step as the announce-list wording that receives its fillers — the
    /// catalog pairs the `"any target"` pro-form with the announced
    /// damage entry's `announced: [2]`, and leaves `"target <predicate>"`
    /// undeclared precisely because no announce-list wording covers the
    /// effects it appears in, so declaring it would put those nominals out of
    /// reach of the plain holes that recover them today.
    ///
    /// Constructor entries only. A macro definition has no field for it
    /// (`MacroDef` gains frame metadata only when a frame demonstrably needs
    /// it), so a macro's frames are never announcements.
    #[serde(default, skip_serializing_if = "core::ops::Not::not")]
    pub announcement: bool,
}

/// Captures a `body:` term as source text, through the very deserializer
/// [`MacroDef`](crate::set::MacroDef)'s own body uses: `RawValue` takes the
/// term verbatim and [`crate::set::body_text`] trims it.
///
/// The field is optional by **absence**, not by an `Option` wrapper in the
/// data: a body is written bare (`body: By(…)`), exactly as a macro's is, so
/// `#[serde(default)]` supplies `None` when the field is missing and this
/// function only ever runs on a term that is actually there. Deserializing an
/// `Option<Box<RawValue>>` instead would demand the author write
/// `body: Some(By(…))`, which is not the schema `MacroDef` established.
fn raw_body<'de, D: Deserializer<'de>>(deserializer: D) -> Result<Option<String>, D::Error> {
    let raw = Box::<RawValue>::deserialize(deserializer)?;
    Ok(Some(crate::set::body_text(&raw).into_string()))
}

/// The mirror of [`raw_body`]: a body serializes back as the **term** it was
/// authored as, not as a string literal of that term's text, so a round trip
/// through serde reproduces a readable catalog file. An absent body is
/// skipped entirely (`skip_serializing_if`), which is why this only ever sees
/// `Some`. A body that is not readable as RON cannot have come from
/// [`raw_body`] and is refused rather than emitted as something that would
/// not read back.
#[expect(
    clippy::ref_option,
    reason = "serde's `serialize_with` hands the field by reference; `Option<&String>` would not typecheck against the derive"
)]
fn raw_body_out<S: Serializer>(body: &Option<String>, serializer: S) -> Result<S::Ok, S::Error> {
    let text = body
        .as_deref()
        .ok_or_else(|| serde::ser::Error::custom("an absent body is skipped, never serialized"))?;
    RawValue::from_ron(text)
        .map_err(serde::ser::Error::custom)?
        .serialize(serializer)
}

/// Fills `body`'s `Param(i)` holes from `args`, positionally, and returns the
/// resulting RON source.
///
/// **The recovery emission for a `body:` entry.** A match against such an
/// entry recovers one argument per declared param; the value the match stands
/// for is this substitution, not the flat `Name(arg, …)` spelling — the name
/// may not be a core variant at all (see [`ConstructorFrames::body`]).
///
/// The substitution is `macro_ron`'s own, not a second one: holes are located
/// by decomposing the term and spliced by byte offset, so a string literal
/// that happens to mention `Param` is never mistaken for a hole. `owner` is
/// the entry name, for error messages; `macros` supplies the RON dialect the
/// term is read in.
///
/// # Errors
/// If `body` is not readable as RON, or holes a param `args` has no entry for.
pub fn substitute_body(
    owner: &str,
    body: &str,
    args: &[&str],
    macros: &MacroSet,
) -> Result<String, String> {
    crate::expand::fill_positional_params(owner.into(), body, args, macros)
}

/// Every positional `Param(i)` index `body` holes, in the order found —
/// what an entry's body actually uses, for checking it against the entry's
/// declared `params`.
///
/// # Errors
/// If `body` is not readable as RON, or holes a named param (`Param(cost)`):
/// a frame body is addressed positionally, like the frames' own
/// `<Param(i)>` sigils.
pub fn body_param_indices(body: &str, macros: &MacroSet) -> Result<Vec<usize>, String> {
    crate::expand::positional_param_indices(body, macros)
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
/// own convention (`deckmaste_plugin::plugin::ron_files_recursive`) for a
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
/// empty. Mirrors `deckmaste_plugin::plugin::ron_files_recursive` (kept as a
/// private copy rather than a shared dependency: `macro_ron` must not
/// depend on `deckmaste_plugin`, which itself depends on `macro_ron` — that
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
            announced: Vec::new(),
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

    /// The `deckmaste_core` kinds and param types a body term is read in the
    /// dialect of. No macros are registered, so nothing here depends on the
    /// plugin corpus — [`substitute_body`] needs a [`MacroSet`] only for its
    /// RON options.
    fn reader() -> MacroSet {
        MacroSet::new(crate::KindSet::default())
    }

    /// A `body:` is authored as a bare term, captured verbatim, and its
    /// `Param(i)` leaves fill positionally through the same splice-by-offset
    /// walk a `MacroDef` body's holes go through.
    #[test]
    fn constructor_entry_body_deserializes_and_parses_as_term() {
        let source = r#"[
            (constructor: "GainLife", params: ["Reference", "Count"], kind: Sentence,
             body: By(Param(0), GainLife(Param(1))),
             frames: ["<Param(0)> gains <Param(1)> life"]),
        ]"#;
        let catalog: Vec<ConstructorFrames> = opts().from_str(source).unwrap();
        let entry = &catalog[0];

        // Captured as the term's own source text — not quoted, not reshaped.
        assert_eq!(
            entry.body.as_deref(),
            Some("By(Param(0), GainLife(Param(1)))")
        );
        // And it is a term, so the body parser can say which params it holes.
        assert_eq!(
            body_param_indices(entry.body.as_deref().unwrap(), &reader()).unwrap(),
            vec![0, 1]
        );
        // Filling those holes is the entry's emission. A hole is replaced at
        // the extent `ron` located it at, so the emitted text's inner spacing
        // follows the body's rather than being normalized — it is re-read as
        // RON downstream, never compared as text.
        assert_eq!(
            substitute_body(
                &entry.constructor,
                entry.body.as_deref().unwrap(),
                &["You", "3"],
                &reader()
            )
            .unwrap(),
            "By(You, GainLife(3))"
        );
        // An entry with no body reads as `None`, so every pre-existing
        // catalog entry keeps meaning the positional application.
        assert_eq!(
            catalog_of(r#"(constructor: "This", params: [], kind: Nominal, frames: ["~"])"#).body,
            None
        );
    }

    /// Holes are located by decomposing the term, so a string literal
    /// mentioning `Param` is data, not a hole — the property that makes
    /// reusing `macro_ron`'s walk (rather than a textual replace) load-bearing
    /// rather than tidy.
    #[test]
    fn body_substitution_does_not_reach_inside_a_string_literal() {
        let filled = substitute_body(
            "Named",
            r#"LandType("Param(0)", Param(0))"#,
            &["Forest"],
            &reader(),
        )
        .unwrap();
        assert_eq!(filled, r#"LandType("Param(0)",Forest)"#);
    }

    /// A body holing a param the entry cannot supply is an error, not a
    /// silently surviving `Param(…)` leaf in the emitted value.
    #[test]
    fn body_substitution_refuses_a_hole_with_no_argument() {
        let error = substitute_body("Named", "By(Param(0), Param(7))", &["You"], &reader())
            .expect_err("Param(7) has no argument");
        assert!(error.contains("Param(7)"), "{error}");
    }

    /// A `body:` authored as a *string* is the near-miss this schema has to
    /// refuse loudly: `RawValue` would capture the quotes, and the resulting
    /// "body" holes nothing at all.
    #[test]
    fn a_quoted_body_holes_no_params() {
        let entry = catalog_of(
            r#"(constructor: "X", params: ["Count"], kind: Sentence, frames: ["<Param(0)> life"], body: "GainLife(Param(0))")"#,
        );
        assert_eq!(
            body_param_indices(entry.body.as_deref().unwrap(), &reader()).unwrap(),
            Vec::<usize>::new(),
            "a quoted body is a string literal, so it holes nothing — the \
             arity check in `Lexicon::assemble` is what turns that into a \
             loud failure"
        );
    }

    /// A body round-trips through serialize as the term it was authored as,
    /// so a catalog file rewritten from this schema still reads back.
    #[test]
    fn body_round_trips_as_a_term_not_a_string() {
        let entry = catalog_of(
            r#"(constructor: "GainLife", params: ["Reference", "Count"], kind: Sentence,
                body: By(Param(0), GainLife(Param(1))),
                frames: ["<Param(0)> gains <Param(1)> life"])"#,
        );
        let text = opts().to_string(&entry).unwrap();
        assert!(
            text.contains("body:By(Param(0), GainLife(Param(1)))"),
            "the body must serialize as a term, not a string or an `Option`: {text}"
        );
        assert_eq!(opts().from_str::<ConstructorFrames>(&text).unwrap(), entry);

        // And an absent body leaves no field behind to read back as `Some`.
        let bodiless =
            catalog_of(r#"(constructor: "This", params: [], kind: Nominal, frames: ["~"])"#);
        let text = opts().to_string(&bodiless).unwrap();
        assert!(!text.contains("body"), "{text}");
        assert_eq!(
            opts().from_str::<ConstructorFrames>(&text).unwrap(),
            bodiless
        );
    }

    /// The announce-class fields: a frame lists the params whose holes take
    /// only an announcement, and an entry says whether it *is* one. Both
    /// default, so every catalog entry authored before them reads unchanged.
    #[test]
    fn announce_class_fields_deserialize_and_default() {
        let announced = catalog_of(
            r#"(constructor: "TargetedDealDamage", params: ["Reference", "Count", "Reference"],
                kind: Sentence,
                body: Targeted(targets: [Param(2)], effect: Act(DealDamage(Param(0), Param(1), Target(0)))),
                frames: [(text: "<Param(0)> deals <Param(1)> damage to <Param(2)>", announced: [2])])"#,
        );
        assert_eq!(announced.frames[0].announced, vec![2]);
        assert!(!announced.announcement);
        assert!(
            announced.frames[0].is_unguarded(),
            "an announced hole still surfaces in the text, so the frame is \
             still a complete rendering"
        );

        let pro_form = catalog_of(
            r#"(constructor: "AnyTarget", params: [], kind: Nominal, announcement: true,
                frames: ["any target"])"#,
        );
        assert!(pro_form.announcement);
        assert_eq!(pro_form.frames[0].announced, Vec::<usize>::new());

        let plain =
            catalog_of(r#"(constructor: "This", params: [], kind: Nominal, frames: ["~"])"#);
        assert!(!plain.announcement);
        assert_eq!(plain.frames[0].announced, Vec::<usize>::new());

        // The defaulted `false` leaves no field behind either, so a generated
        // entry copied into a hand-authored catalog carries no noise in; a
        // declared `true` is data and always survives the round trip.
        let text = opts().to_string(&plain).unwrap();
        assert!(!text.contains("announcement"), "{text}");
        assert_eq!(opts().from_str::<ConstructorFrames>(&text).unwrap(), plain);
        let text = opts().to_string(&pro_form).unwrap();
        assert!(text.contains("announcement:true"), "{text}");
        assert_eq!(
            opts().from_str::<ConstructorFrames>(&text).unwrap(),
            pro_form
        );
    }

    /// An announced hole is part of the frame's identity, so the bare-string
    /// serialization sugar must not swallow it.
    #[test]
    fn an_announced_frame_does_not_serialize_as_a_bare_string() {
        let spec = FrameSpec {
            announced: vec![2],
            ..FrameSpec::bare("<Param(0)> deals <Param(1)> damage to <Param(2)>")
        };
        let text = opts().to_string(&spec).unwrap();
        assert!(text.starts_with('('), "must take the full form: {text}");
        assert_eq!(opts().from_str::<FrameSpec>(&text).unwrap(), spec);
    }

    fn catalog_of(entry: &str) -> ConstructorFrames {
        opts()
            .from_str::<ConstructorFrames>(entry)
            .unwrap_or_else(|error| panic!("reading {entry}: {error}"))
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
