//! `cargo xtask macro inspect` — dump a compiled frame: text, kind, holes
//! with classes and paths, agreement dependencies, and guards. Consumes
//! Task 3's schema/loader (`macro_ron::frames`) and Task 4's compiler
//! (`deckmaste_frames::compile`); xtask owns only the CLI and the printing.

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

#[derive(Debug, Args)]
pub(super) struct InspectArgs {
    /// The constructor (from the frame catalog) or macro name to inspect.
    /// The constructor catalog is checked first, then every loaded macro.
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
}

pub(super) fn run(args: InspectArgs) -> anyhow::Result<()> {
    let plugin_dir = args.plugin_dir.unwrap_or_else(super::default_plugin_dir);
    let plugin = Plugin::load_with_sibling_prelude(&plugin_dir)
        .with_context(|| format!("loading plugin {}", plugin_dir.display()))?;
    let catalogs = Catalogs::default();
    let kind: FragmentKind = args.kind.into();

    let Resolved {
        source,
        params,
        frames,
    } = resolve(&plugin, &plugin_dir, &args.name)?;

    println!(
        "{source} `{}` ({} param(s)): {} frame(s)",
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
    params: Vec<String>,
    frames: Vec<FrameSpec>,
}

/// Finds `name` in the constructor frame catalog first, then among every
/// macro `plugin` has loaded (any kind).
///
/// # Errors
/// If `name` names neither, or names one with no frames defined yet, or
/// names a macro with a named (rather than positional) param signature —
/// `<Param(i)>` is positional-only, so a named-signature macro has no
/// defined index mapping for `macro inspect` to compile against.
fn resolve(plugin: &Plugin, plugin_dir: &Path, name: &str) -> anyhow::Result<Resolved> {
    let frames_dir = plugin_dir.join("frames");
    let catalog = load_constructor_frames(&frames_dir).with_context(|| {
        format!(
            "loading the constructor frame catalog under {}",
            frames_dir.display()
        )
    })?;
    if let Some(entry) = catalog.iter().find(|entry| entry.constructor == name) {
        anyhow::ensure!(
            !entry.frames.is_empty(),
            "constructor `{name}` is in the frame catalog but has no frames defined yet"
        );
        return Ok(Resolved {
            source: "constructor",
            params: entry.params.clone(),
            frames: entry.frames.clone(),
        });
    }

    if let Some((_, def)) = plugin
        .macros
        .iter()
        .find(|(_, def)| def.name.as_str() == name)
    {
        anyhow::ensure!(
            !def.frames().is_empty(),
            "macro `{name}` is loaded but has no frames defined yet"
        );
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
    frames: ["<Param(0)> macro-should-not-win <Param(1)>"],
    body: 0,
)
"#;

    #[test]
    fn resolve_prefers_the_constructor_catalog_over_a_same_named_macro() {
        let dir = tempdir_with(&[
            ("frames/constructors.ron", PRECEDENCE_CONSTRUCTOR),
            ("macros/effect/Foo.ron", PRECEDENCE_MACRO),
        ]);
        let plugin = Plugin::load(dir.path()).unwrap();

        let resolved = resolve(&plugin, dir.path(), "Foo").unwrap();

        assert_eq!(resolved.source, "constructor");
        assert_eq!(resolved.params, vec!["Reference".to_string()]);
        assert_eq!(resolved.frames.len(), 1);
        assert_eq!(resolved.frames[0].text, "<Param(0)> constructor-wins");
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

        let resolved = resolve(&plugin, dir.path(), "Bar").unwrap();

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

        let error = resolve(&plugin, dir.path(), "NamedOne").unwrap_err();

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

        let error = resolve(&plugin, dir.path(), "Empty").unwrap_err();

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

        let error = resolve(&plugin, dir.path(), "NoFrames").unwrap_err();

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

        let error = resolve(&plugin, dir.path(), "Ghost").unwrap_err();

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
