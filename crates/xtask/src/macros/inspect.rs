//! `cargo xtask macro inspect` — dump a compiled frame: text, kind, holes
//! with classes and paths, agreement dependencies, and guards. Consumes
//! Task 3's schema/loader (`macro_ron::frames`) and Task 4's compiler
//! (`deckmaste_frames::compile`); xtask owns only the CLI and the printing.

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
    /// Every pilot entry but `Target` (a noun phrase, `--kind nominal`) is a
    /// sentence, hence the default.
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
        print_compiled(&compiled);
    }
    Ok(())
}

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

/// Prints a compiled frame's holes, agreement dependencies, and guards.
///
/// Reads holes from [`CompiledFrame::holes`] — already one entry per hole
/// (a `HoleClass::FieldSlice` claims several tree fields but is still one
/// `Hole`, never three) — and reports each hole's *site count* by scanning
/// the tree for `View::Hole { index, .. }` rather than trusting
/// `Hole::path` alone, since a multi-occurrence `~` keeps only its first
/// site there.
fn print_compiled(frame: &CompiledFrame) {
    println!("holes:");
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
        println!(
            "  [{}] param={:?} class={class} path={} site(s)={sites}",
            hole.index, hole.param, hole.path
        );
    }

    if frame.agreement.is_empty() {
        println!("agreement: (none)");
    } else {
        println!("agreement:");
        for dep in &frame.agreement {
            println!(
                "  {:?} at {} — normalized: {:?}",
                dep.kind, dep.site, dep.normalized
            );
        }
    }

    if frame.guards.is_empty() {
        println!("guards: (none)");
    } else {
        println!("guards:");
        for guard in &frame.guards {
            println!(
                "  param {} ({}): authored {:?} -> canonical {:?}",
                guard.param, guard.param_type, guard.source, guard.value
            );
        }
    }
}
