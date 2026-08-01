//! `cargo xtask macro inspect` — dump a compiled frame: text, kind, holes
//! with classes and paths, agreement dependencies, and guards. The authoring
//! schema and loader are `macro_ron::frames`' and the compiler is
//! `deckmaste_frames::compile`'s; xtask owns only the CLI and the printing.

use std::io;
use std::path::Path;
use std::path::PathBuf;

use anyhow::Context;
use clap::Args;
use clap::ValueEnum;
use deckmaste_cards::plugin::Plugin;
use deckmaste_english::Catalogs;
use deckmaste_english::FragmentKind;
use deckmaste_frames::CompiledFrame;
use deckmaste_frames::HoleClass;
use deckmaste_frames::View;
use deckmaste_frames::lexicon::macro_fragment_kind;
use macro_ron::MacroDef;
use macro_ron::Params;
use macro_ron::frames::FrameSpec;
use macro_ron::frames::load_constructor_frames;

/// A `clap`-friendly mirror of [`FragmentKind`] — `FragmentKind` itself has
/// no `ValueEnum` (it lives in a leaf crate with no `clap` dependency), so
/// `--kind` reads into this and converts.
#[derive(Debug, Clone, Copy, PartialEq, Eq, ValueEnum)]
enum Kind {
    Nominal,
    Sentence,
    Cost,
    KeywordLine,
    Ability,
}

impl From<Kind> for FragmentKind {
    fn from(kind: Kind) -> FragmentKind {
        match kind {
            Kind::Nominal => FragmentKind::Nominal,
            Kind::Sentence => FragmentKind::Sentence,
            Kind::Cost => FragmentKind::Cost,
            Kind::KeywordLine => FragmentKind::KeywordLine,
            Kind::Ability => FragmentKind::Ability,
        }
    }
}

/// Where a name's frames are authored. A `clap`-friendly mirror of
/// [`deckmaste_frames::lexicon::Origin`], for the same reason [`Kind`]
/// mirrors [`FragmentKind`]: the real type lives in a crate with no `clap`
/// dependency.
#[derive(Debug, Clone, Copy, PartialEq, Eq, ValueEnum)]
enum Origin {
    /// The constructor frame catalog under `<plugin-dir>/frames`.
    Constructor,
    /// A loaded macro definition's own `frames:` list.
    Macro,
}

#[derive(Debug, Args)]
pub(super) struct InspectArgs {
    /// The constructor (from the frame catalog) or macro name to inspect.
    name: String,

    /// Defaults to this workspace's `plugins/builtin`.
    plugin_dir: Option<PathBuf>,

    /// The English category to parse every one of `name`'s frames at.
    /// Most pilot entries are a sentence, hence the default — but the
    /// noun-phrase entries (`Target`, `This`) need `--kind nominal`;
    /// `cargo xtask macro inspect This` fails to compile at the default
    /// kind without it.
    #[arg(long, value_enum, default_value_t = Kind::Sentence)]
    kind: Kind,

    /// Which side of the lexicon to read `name` from. Required when the
    /// frame catalog and a loaded macro both define it — neither is more
    /// authoritative than the other, so the choice is the caller's.
    #[arg(long, value_enum)]
    origin: Option<Origin>,
}

pub(super) fn run(args: InspectArgs) -> anyhow::Result<()> {
    let plugin_dir = args.plugin_dir.unwrap_or_else(super::default_plugin_dir);
    let plugin = Plugin::load_with_sibling_prelude(&plugin_dir)
        .with_context(|| format!("loading plugin {}", plugin_dir.display()))?;
    let catalogs = Catalogs::default();
    let kind: FragmentKind = args.kind.into();

    let Resolved {
        source,
        declared,
        params,
        frames,
    } = resolve(&plugin, &plugin_dir, &args.name, args.origin)?;

    println!(
        "{source} `{}`{declared} ({} param(s)): {} frame(s)",
        args.name,
        params.len(),
        frames.len()
    );
    for (index, spec) in frames.iter().enumerate() {
        println!("\n--- frame [{index}] ---");
        println!("text: {:?}", spec.text);
        println!("kind: {kind:?}");
        if !spec.when.is_empty() {
            println!("when (authored, unexpanded): {:?}", spec.when);
        }
        if let Some(position) = spec.position {
            println!("position: {position:?}");
        }

        let compiled = deckmaste_frames::compile(spec, kind, &params, &catalogs, &plugin.macros)
            .with_context(|| format!("compiling frame [{index}] of `{}` at {kind:?}", args.name))?;
        write_compiled(io::stdout().lock(), &compiled)?;
    }
    Ok(())
}

#[derive(Debug)]
struct Resolved {
    /// `"constructor"` or `"macro"`, purely for the header line.
    source: &'static str,
    /// The resolved definition's declared macro `kinds:`, pre-formatted for
    /// the header line (empty for a catalog constructor, which declares none).
    /// Printed because a name several definitions carry resolves to one of
    /// them, and the reader has to be able to see which.
    declared: String,
    params: Vec<String>,
    frames: Vec<FrameSpec>,
}

/// Finds the framed `name`: a constructor frame catalog entry, or a loaded
/// macro definition carrying a `frames:` list.
///
/// # One name, several definitions
///
/// Both halves can be ambiguous, and they are ambiguous in different ways.
///
/// A [`MacroSet`](macro_ron::MacroSet) deliberately holds several
/// definitions under one name as long as their macro kinds differ, and the
/// live corpus does exactly that (`Creature`, `Draw`, `Draws` each sit in two
/// files) — while [`MacroSet::iter`](macro_ron::MacroSet::iter) documents its
/// order as unspecified, since it follows the backing hash maps. Picking the
/// first match therefore resolved a name differently between processes, and
/// in every corpus pair only one side is framed at all, so half the time the
/// answer was a spurious "has no frames defined yet". Two things fix that,
/// both borrowed from
/// [`Lexicon::assemble`](deckmaste_frames::Lexicon::assemble) rather than
/// invented here: only framed definitions are candidates (the set `assemble`
/// draws entries from), and the survivors are ordered by the key `assemble`
/// sorts entries by — English category first, via
/// [`kind_rank`](deckmaste_frames::lexicon::kind_rank), with the declared
/// kinds as the final tiebreak for two definitions that resolve to one
/// category (which `assemble` refuses outright, and this tool may still be
/// pointed at while that is being fixed).
///
/// A name carried by *both* the catalog and a framed macro is not ordered at
/// all: neither side is more authoritative, so it is refused and `--origin`
/// decides. Silently preferring one hid a real authoring collision.
///
/// # Errors
/// If `name` names neither, or names one with no frames defined yet, or is
/// carried by both origins with no `--origin` to choose, or names a macro
/// with a named (rather than positional) param signature — `<Param(i)>` is
/// positional-only, so a named-signature macro has no defined index mapping
/// for `macro inspect` to compile against.
fn resolve(
    plugin: &Plugin,
    plugin_dir: &Path,
    name: &str,
    origin: Option<Origin>,
) -> anyhow::Result<Resolved> {
    let frames_dir = plugin_dir.join("frames");
    let catalog = load_constructor_frames(&frames_dir).with_context(|| {
        format!(
            "loading the constructor frame catalog under {}",
            frames_dir.display()
        )
    })?;
    let catalog_entry = catalog.iter().find(|entry| entry.constructor == name);
    let defs = definitions_named(plugin, name);
    let framed: Vec<&MacroDef> = defs
        .iter()
        .copied()
        .filter(|def| !def.frames().is_empty())
        .collect();

    if origin.is_none()
        && catalog_entry.is_some_and(|entry| !entry.frames.is_empty())
        && !framed.is_empty()
    {
        anyhow::bail!(
            "`{name}` is both a constructor frame catalog entry and a framed macro \
             (kinds {}); pass `--origin constructor` or `--origin macro` to say which one to \
             inspect",
            declared_kinds(framed[0]),
        );
    }

    if origin != Some(Origin::Macro)
        && let Some(entry) = catalog_entry
    {
        anyhow::ensure!(
            !entry.frames.is_empty(),
            "constructor `{name}` is in the frame catalog but has no frames defined yet"
        );
        return Ok(Resolved {
            source: "constructor",
            declared: String::new(),
            params: entry.params.clone(),
            frames: entry.frames.clone(),
        });
    }

    if origin != Some(Origin::Constructor) && !defs.is_empty() {
        let def = *framed.first().ok_or_else(|| {
            anyhow::anyhow!("macro `{name}` is loaded but has no frames defined yet")
        })?;
        let params = match &def.params {
            Params::Positional(types) => types
                .iter()
                .map(|ty| ty.name.as_str().to_string())
                .collect(),
            Params::Named(_) => anyhow::bail!(
                "macro `{name}` has a named param signature; `macro inspect` supports \
                 positional-param macros and catalog constructors only, since `<Param(i)>` \
                 in a frame's text is a positional index"
            ),
        };
        return Ok(Resolved {
            source: "macro",
            declared: format!(" {}", declared_kinds(def)),
            params,
            frames: def.frames().to_vec(),
        });
    }

    anyhow::bail!(
        "no constructor or macro named `{name}` (checked the frame catalog under {} and \
         every macro `{}` loaded)",
        frames_dir.display(),
        plugin_dir.display(),
    );
}

/// Every definition `plugin` holds under `name`, in a deterministic order —
/// see [`resolve`]'s own doc for why neither property comes for free.
///
/// [`MacroSet::iter`](macro_ron::MacroSet::iter) yields one pair *per kind*,
/// so a multi-kind definition appears several times and is deduplicated by
/// `(name, sorted kinds)` — the real identity of a definition, the same key
/// `Lexicon::assemble` deduplicates on.
fn definitions_named<'plugin>(plugin: &'plugin Plugin, name: &str) -> Vec<&'plugin MacroDef> {
    let mut found: Vec<&MacroDef> = Vec::new();
    for (_, def) in plugin.macros.iter() {
        if def.name.as_str() != name {
            continue;
        }
        if !found
            .iter()
            .any(|seen| declared_kinds(seen) == declared_kinds(def))
        {
            found.push(def);
        }
    }
    found.sort_by_key(|def| {
        (
            deckmaste_frames::lexicon::kind_rank(macro_fragment_kind(def)),
            declared_kinds(def),
        )
    });
    found
}

/// A definition's declared macro `kinds:`, sorted, as one readable string —
/// the header line's disambiguator and [`definitions_named`]'s identity key
/// and final tiebreak.
fn declared_kinds(def: &MacroDef) -> String {
    let mut kinds: Vec<&str> = def.kinds.iter().map(macro_ron::Ident::as_str).collect();
    kinds.sort_unstable();
    format!("[{}]", kinds.join(", "))
}

/// Writes a compiled frame's holes, agreement dependencies, and guards.
///
/// Takes a generic `writer` (rather than `println!`ing directly) so tests
/// can capture the output into a `Vec<u8>` instead of scraping stdout — the
/// same shape `crate::english::inspect::write_cards` already uses.
///
/// Reads holes from [`CompiledFrame::holes`] — already one entry per hole
/// (a `HoleClass::FieldSlice` claims several tree fields but is still one
/// `Hole`, never three) — and reports each hole's *site count* by scanning
/// the tree for `View::Hole { index, .. }` rather than trusting
/// `Hole::path` alone, since a multi-occurrence `~` keeps only its first
/// site there.
fn write_compiled(mut writer: impl io::Write, frame: &CompiledFrame) -> io::Result<()> {
    writeln!(writer, "holes:")?;
    for hole in &frame.holes {
        let sites = frame
            .tree
            .walk()
            .into_iter()
            .filter(|(_, node)| matches!(node, View::Hole { index, .. } if *index == hole.index))
            .count();
        let class = match &hole.class {
            HoleClass::FieldSlice { claimed } => format!("FieldSlice {claimed:?}"),
            other => format!("{other:?}"),
        };
        writeln!(
            writer,
            "  [{}] param={:?} class={class} path={} site(s)={sites}",
            hole.index, hole.param, hole.path
        )?;
    }

    if frame.agreement.is_empty() {
        writeln!(writer, "agreement: (none)")?;
    } else {
        writeln!(writer, "agreement:")?;
        for dep in &frame.agreement {
            writeln!(
                writer,
                "  {:?} at {} — normalized: {:?}",
                dep.kind, dep.site, dep.normalized
            )?;
        }
    }

    if frame.guards.is_empty() {
        writeln!(writer, "guards: (none)")?;
    } else {
        writeln!(writer, "guards:")?;
        for guard in &frame.guards {
            writeln!(
                writer,
                "  param {} ({}): authored {:?} -> canonical {:?}",
                guard.param, guard.param_type, guard.source, guard.value
            )?;
        }
    }
    Ok(())
}

#[cfg(test)]
mod tests {
    use super::*;

    // -----------------------------------------------------------------
    // `resolve` — name resolution and its refusals. Fixtures are hand-built
    // temp plugins, per the round's ruling that these tests must not depend
    // on the corpus (which frames nothing today).
    // -----------------------------------------------------------------

    /// A bare, macro-free reader: every fixture frame below is unguarded, so
    /// no guard constant ever needs a macro (e.g. `Exactly`) to expand.
    fn empty_macro_set() -> macro_ron::MacroSet {
        macro_ron::MacroSet::new(deckmaste_core::ron::kinds())
            .with_options(deckmaste_core::ron::raw_options())
    }

    const PRECEDENCE_CONSTRUCTOR: &str = r#"[
    (constructor: "Foo", params: ["Reference"], frames: ["<Param(0)> constructor-wins"], kind: Sentence),
]
"#;

    // Body is `0` (never the macro's own name) for the same reason
    // `templates.rs`'s fixtures do this: `body: Foo(...)` inside a def named
    // `Foo` reads as a self-invocation and trips `macro_ron`'s cycle check
    // at registration; these fixtures only need to *register*, never expand.
    const PRECEDENCE_MACRO: &str = r#"(
    name: "Foo",
    kinds: [OneShotEffect],
    params: [Reference, Count],
    frames: ["<Param(0)> macro-also-carries-it <Param(1)>"],
    body: 0,
)
"#;

    /// One name, three definitions across the fixtures below — the shape the
    /// live corpus has (`Creature`, `Draw`, `Draws` each sit in two files
    /// under different macro kinds). Different `kinds:` are what lets a
    /// `MacroSet` hold them at once.
    const TWIN_FRAMED: &str = r#"(
    name: "Twin",
    kinds: [OneShotEffect],
    params: [Reference, Count],
    frames: ["<Param(0)> twin <Param(1)>"],
    body: 0,
)
"#;

    const TWIN_UNFRAMED: &str = r#"(
    name: "Twin",
    kinds: [EventFilter],
    body: 0,
)
"#;

    const TWIN_FRAMED_NOMINAL: &str = r#"(
    name: "Twin",
    kinds: [Predicate],
    params: [Predicate],
    frames: ["twin <Param(0)>"],
    body: 0,
)
"#;

    #[test]
    fn resolve_refuses_a_name_the_catalog_and_a_macro_both_carry() {
        let dir = tempdir_with(&[
            ("frames/constructors.ron", PRECEDENCE_CONSTRUCTOR),
            ("macros/effect/Foo.ron", PRECEDENCE_MACRO),
        ]);
        let plugin = Plugin::load(dir.path()).unwrap();

        let error = resolve(&plugin, dir.path(), "Foo", None).unwrap_err();

        let message = format!("{error:#}");
        assert!(message.contains("--origin"), "{message}");
        assert!(message.contains("constructor"), "{message}");
        assert!(message.contains("macro"), "{message}");
    }

    /// The other half of the refusal above: `--origin` reaches either side,
    /// and reaches the one it names rather than the one that happened to be
    /// preferred. Both directions are asserted, so this cannot pass against
    /// an `--origin` that is ignored.
    #[test]
    fn origin_selects_which_side_of_a_shared_name_is_inspected() {
        let dir = tempdir_with(&[
            ("frames/constructors.ron", PRECEDENCE_CONSTRUCTOR),
            ("macros/effect/Foo.ron", PRECEDENCE_MACRO),
        ]);
        let plugin = Plugin::load(dir.path()).unwrap();

        let from_catalog = resolve(&plugin, dir.path(), "Foo", Some(Origin::Constructor)).unwrap();
        assert_eq!(from_catalog.source, "constructor");
        assert_eq!(from_catalog.frames[0].text, "<Param(0)> constructor-wins");

        let from_macro = resolve(&plugin, dir.path(), "Foo", Some(Origin::Macro)).unwrap();
        assert_eq!(from_macro.source, "macro");
        assert_eq!(
            from_macro.frames[0].text,
            "<Param(0)> macro-also-carries-it <Param(1)>"
        );
        assert!(
            from_macro.declared.contains("OneShotEffect"),
            "the header names the definition that was resolved: {:?}",
            from_macro.declared
        );
    }

    /// Two definitions may share a name under different macro kinds, and the
    /// live corpus has three such pairs. `MacroSet::iter` follows a hash map,
    /// so without an order imposed here the name resolves to whichever the
    /// map yielded first — differently between processes, and in each of
    /// those pairs only one side carries frames at all, so the *other*
    /// outcome is a spurious "has no frames defined yet".
    #[test]
    fn resolve_picks_the_framed_definition_of_a_twice_carried_name() {
        let dir = tempdir_with(&[
            ("macros/effect/Twin.ron", TWIN_FRAMED),
            ("macros/filter/Twin.ron", TWIN_UNFRAMED),
        ]);
        // Each load builds a fresh hash map, so this samples real orders
        // rather than one process's fixed one.
        for _ in 0..24 {
            let plugin = Plugin::load(dir.path()).unwrap();
            let resolved = resolve(&plugin, dir.path(), "Twin", None).unwrap();
            assert_eq!(resolved.frames[0].text, "<Param(0)> twin <Param(1)>");
        }
    }

    /// When both definitions of a name *are* framed — a state
    /// `Lexicon::assemble` refuses outright, and which `macro inspect` may
    /// still be pointed at while it is being fixed — the tie breaks on the
    /// English category, in the same order the assembled lexicon sorts
    /// entries by. `Predicate` resolves to a nominal and `OneShotEffect` to
    /// a sentence, and nominal sorts first.
    #[test]
    fn resolve_orders_two_framed_definitions_of_one_name_by_category() {
        let dir = tempdir_with(&[
            ("macros/effect/Twin.ron", TWIN_FRAMED),
            ("macros/filter/TwinNominal.ron", TWIN_FRAMED_NOMINAL),
        ]);
        for _ in 0..24 {
            let plugin = Plugin::load(dir.path()).unwrap();
            let resolved = resolve(&plugin, dir.path(), "Twin", None).unwrap();
            assert_eq!(resolved.frames[0].text, "twin <Param(0)>");
        }
    }

    #[test]
    fn resolve_falls_back_to_a_loaded_macro_when_no_constructor_matches() {
        let source = r#"(
    name: "Bar",
    kinds: [OneShotEffect],
    params: [Reference, Count],
    frames: ["<Param(0)> bar <Param(1)>"],
    body: 0,
)
"#;
        let dir = tempdir_with(&[("macros/effect/Bar.ron", source)]);
        let plugin = Plugin::load(dir.path()).unwrap();

        let resolved = resolve(&plugin, dir.path(), "Bar", None).unwrap();

        assert_eq!(resolved.source, "macro");
        assert_eq!(
            resolved.params,
            vec!["Reference".to_string(), "Count".to_string()]
        );
        assert_eq!(resolved.frames[0].text, "<Param(0)> bar <Param(1)>");
    }

    #[test]
    fn resolve_refuses_a_named_param_macro() {
        let source = r#"(
    name: "NamedOne",
    kinds: [OneShotEffect],
    params: { "x": Reference },
    frames: ["<Param(0)> whatever"],
    body: 0,
)
"#;
        let dir = tempdir_with(&[("macros/effect/NamedOne.ron", source)]);
        let plugin = Plugin::load(dir.path()).unwrap();

        let error = resolve(&plugin, dir.path(), "NamedOne", None).unwrap_err();

        assert!(
            format!("{error:#}").contains("named param signature"),
            "{error:#}"
        );
    }

    #[test]
    fn resolve_refuses_a_constructor_catalog_entry_with_no_frames_yet() {
        let source = r#"[
    (constructor: "Empty", params: [], frames: [], kind: Sentence),
]
"#;
        let dir = tempdir_with(&[("frames/constructors.ron", source)]);
        let plugin = Plugin::load(dir.path()).unwrap();

        let error = resolve(&plugin, dir.path(), "Empty", None).unwrap_err();

        assert!(
            format!("{error:#}").contains("no frames defined yet"),
            "{error:#}"
        );
        assert!(format!("{error:#}").contains("constructor"), "{error:#}");
    }

    #[test]
    fn resolve_refuses_a_loaded_macro_with_no_frames_yet() {
        let source = r#"(
    name: "NoFrames",
    kinds: [OneShotEffect],
    body: 0,
)
"#;
        let dir = tempdir_with(&[("macros/effect/NoFrames.ron", source)]);
        let plugin = Plugin::load(dir.path()).unwrap();

        let error = resolve(&plugin, dir.path(), "NoFrames", None).unwrap_err();

        assert!(
            format!("{error:#}").contains("no frames defined yet"),
            "{error:#}"
        );
        assert!(format!("{error:#}").contains("macro"), "{error:#}");
    }

    #[test]
    fn resolve_reports_a_name_that_matches_neither() {
        let dir = tempdir_with(&[]);
        let plugin = Plugin::load(dir.path()).unwrap();

        let error = resolve(&plugin, dir.path(), "Ghost", None).unwrap_err();

        assert!(
            format!("{error:#}").contains("no constructor or macro named"),
            "{error:#}"
        );
    }

    // -----------------------------------------------------------------
    // `write_compiled` — the two cross-task-risk contracts the brief's
    // interface corrections singled out: a `FieldSlice` hole must render as
    // ONE claimed-fields entry (never three), and a hole's true site count
    // must come from a tree scan, not from trusting `Hole::path` (which
    // keeps only the first site of a repeated `~`).
    // -----------------------------------------------------------------

    #[test]
    fn write_compiled_renders_a_field_slice_hole_as_one_claimed_group() {
        // The catalog's own `Target` shape: `target <Param(0)>` at `Nominal`
        // claims `modifiers`/`head`/`complements` as one hole — three tree
        // sites, one `Hole` entry (`crate::deckmaste_frames::compile`'s
        // `HoleClass::FieldSlice` doc).
        let frame = deckmaste_frames::compile(
            &FrameSpec::bare("target <Param(0)>"),
            FragmentKind::Nominal,
            &["Predicate".to_string()],
            &Catalogs::default(),
            &empty_macro_set(),
        )
        .unwrap();

        let mut out = Vec::new();
        write_compiled(&mut out, &frame).unwrap();
        let text = String::from_utf8(out).unwrap();

        assert_eq!(
            text.matches("class=FieldSlice").count(),
            1,
            "the three claimed fields must render as one hole line, not three: {text}"
        );
        assert!(
            text.contains(r#"class=FieldSlice ["modifiers", "head", "complements"]"#),
            "{text}"
        );
        assert!(
            text.contains("site(s)=3"),
            "three tree sites for the one claimed-field hole: {text}"
        );
    }

    #[test]
    fn write_compiled_counts_every_site_of_a_repeated_self_reference() {
        // `~ and ~` shares one `Hole` entry (SelfRef) but has two tree
        // sites; `Hole::path` alone would only ever point at the first.
        let frame = deckmaste_frames::compile(
            &FrameSpec::bare("~ and ~ get +<Param(0)>/+<Param(1)>"),
            FragmentKind::Sentence,
            &["Count".to_string(), "Count".to_string()],
            &Catalogs::default(),
            &empty_macro_set(),
        )
        .unwrap();

        let mut out = Vec::new();
        write_compiled(&mut out, &frame).unwrap();
        let text = String::from_utf8(out).unwrap();

        let self_ref_lines: Vec<&str> = text
            .lines()
            .filter(|line| line.contains("SelfRef"))
            .collect();
        assert_eq!(
            self_ref_lines.len(),
            1,
            "one Hole entry even though `~` appears twice: {text}"
        );
        assert!(
            self_ref_lines[0].contains("site(s)=2"),
            "{}",
            self_ref_lines[0]
        );
    }

    /// Minimal self-cleaning temp dir (avoids adding the `tempfile` crate) —
    /// same pattern `xtask::coverage`/`macros::templates` already carry.
    fn tempdir_with(files: &[(&str, &str)]) -> TempDir {
        let dir = TempDir::new();
        for (rel, body) in files {
            let path = dir.path().join(rel);
            std::fs::create_dir_all(path.parent().unwrap()).unwrap();
            std::fs::write(path, body).unwrap();
        }
        dir
    }

    struct TempDir(std::path::PathBuf);
    impl TempDir {
        fn new() -> Self {
            use std::sync::atomic::AtomicU32;
            use std::sync::atomic::Ordering;
            static N: AtomicU32 = AtomicU32::new(0);
            let base = std::env::temp_dir().join(format!(
                "xtask-macro-inspect-{}-{}",
                std::process::id(),
                N.fetch_add(1, Ordering::Relaxed)
            ));
            std::fs::create_dir_all(&base).unwrap();
            Self(base)
        }
        fn path(&self) -> &std::path::Path {
            &self.0
        }
    }
    impl Drop for TempDir {
        fn drop(&mut self) {
            let _ = std::fs::remove_dir_all(&self.0);
        }
    }
}
