//! `cargo xtask macro census` — the D10 ratchet's first reading: how many of
//! the corpus's true macro *definitions* (as opposed to meta-macro
//! *invocations*, which have no `frames:` slot of their own to carry — see
//! `KeywordAbility.ron`'s produced-def shape, which forwards `name`/
//! `template`/`kinds`/`body` but never `frames`) currently carry a
//! non-empty `frames:`, alongside the pilot's own per-gate G3/G4 status and
//! the corpus-wide count of `template:` fields the legacy mini-language
//! [`project`](super::templates) can't express.
//!
//! Four populations this command reports, deliberately kept distinct in
//! the printed output — conflating them reads as a regression when none
//! occurred:
//! - **framed / true definitions** — this module's own scan
//!   ([`scan_macro_dirs`]), over every `.ron` file under
//!   `plugins/builtin/macros` and `plugins/canon/macros` (`macro_ron`'s
//!   `MacroDef` schema, classified definition-vs-invocation by
//!   [`scan_macro_dir`]'s doc).
//! - **per-gate pilot status** — [`super::pilot::gate_status`], the same G3/G4
//!   verdicts `cargo xtask macro pilot` computes, condensed to one line each.
//!   This command **reports every population and gates on none**: a red verdict
//!   here is printed, never turned into a non-zero exit, and the command that
//!   owns those verdicts — and does fail on them — is `cargo xtask macro
//!   pilot`. A status that cannot be computed at all is printed as such for the
//!   same reason, so an unrelated breakage costs one line of the reading rather
//!   than all of it.
//! - **excepted-template count, corpus-wide** —
//!   [`super::templates::corpus_wide_excepted_names`], scanning every
//!   `template:` field in `plugins/builtin` (not just framed defs — see that
//!   function's own doc). This is a *different population* from `cargo xtask
//!   macro templates --check`'s own `excepted` count, which is scoped to framed
//!   defs only — printed side by side with each count's scope named, so a
//!   reader never mistakes one for the other.
//! - **corpus residual `Full` share** —
//!   [`super::residuals::corpus_full_share`], the same whole-corpus sweep and
//!   classification `cargo xtask macro residuals` itself runs, not an
//!   independently-derived count of this command's own. A different population
//!   again from the two above: those count `.ron` macro-definition *files*,
//!   this counts rendered rules *lines*.

use std::fs;
use std::path::Path;
use std::path::PathBuf;

use anyhow::Context;
use clap::Args;
use deckmaste_cards::macros::macro_set;
use macro_ron::MacroDef;

#[derive(Debug, Args)]
pub(super) struct CensusArgs {
    /// Defaults to this workspace's `plugins/builtin`.
    #[arg(long)]
    builtin_dir: Option<PathBuf>,
    /// Defaults to this workspace's `plugins/canon`.
    #[arg(long)]
    canon_dir: Option<PathBuf>,
}

pub(super) fn run(args: CensusArgs) -> anyhow::Result<()> {
    let workspace_root = Path::new(env!("CARGO_MANIFEST_DIR")).join("../..");
    let builtin_dir = args
        .builtin_dir
        .unwrap_or_else(|| workspace_root.join("plugins/builtin"));
    let canon_dir = args
        .canon_dir
        .unwrap_or_else(|| workspace_root.join("plugins/canon"));

    let defs = scan_macro_dirs(&[builtin_dir.clone(), canon_dir.clone()])?;
    println!(
        "macro definitions ({} + {}): {} framed / {} true definition(s) — {} total .ron file(s) \
         under macros/, {} of them meta-macro invocation(s) with no frames: slot of their own",
        builtin_dir.display(),
        canon_dir.display(),
        defs.framed,
        defs.definitions,
        defs.total_files,
        defs.invocations,
    );

    let excepted = super::templates::corpus_wide_excepted_names(&builtin_dir)?;
    println!(
        "excepted templates, corpus-wide (every template: field under {}, not just framed \
         defs): {} — a different population from `cargo xtask macro templates --check`'s own \
         `excepted` count, which is scoped to framed defs only",
        builtin_dir.display(),
        excepted.len(),
    );

    match super::residuals::corpus_full_share(&builtin_dir, &canon_dir) {
        Ok((full, lines)) => {
            println!(
                "corpus residual sweep, whole-line recovery: Full {full} / {lines} line(s) ({})",
                super::residuals::percent(full, lines),
            );
        }
        Err(error) => {
            println!("corpus residual sweep: could not compute ({error:#})");
        }
    }

    match super::pilot::gate_status(&builtin_dir, &canon_dir) {
        Ok(status) => {
            println!(
                "pilot G3 shadow parity: {}: {} checked ({} equal, {} mismatched)",
                if status.g3_pass { "PASS" } else { "FAIL" },
                status.g3_checked,
                status.g3_equal,
                status.g3_mismatched,
            );
            println!(
                "pilot G4 ground truth: {}: {} covered / {} equal / {} diverged{}",
                if status.g4_pass { "PASS" } else { "FAIL" },
                status.g4_covered,
                status.g4_equal,
                status.g4_diverged,
                if status.g4_pass {
                    String::new()
                } else {
                    format!(" — see {}", super::pilot::G5_FINDINGS_FILE)
                },
            );
        }
        Err(error) => {
            println!("pilot gate status: could not compute ({error:#})");
        }
    }
    Ok(())
}

/// A macro-definition population count over one or more plugins' `macros/`
/// trees — see the module doc's "framed / true definitions" population.
#[derive(Debug, Clone, Copy, Default, PartialEq, Eq)]
pub(super) struct DefCensus {
    /// Every `.ron` file seen under a scanned `macros/` tree.
    pub total_files: usize,
    /// Files whose root value is a meta-macro *invocation*
    /// (`KeywordAbility(...)`, `CreatureType(...)`), not a literal
    /// `MacroDef` struct.
    pub invocations: usize,
    /// `total_files - invocations` — every literal `MacroDef` definition;
    /// the D10 ratchet's own denominator.
    pub definitions: usize,
    /// Of `definitions`, how many currently carry a non-empty `frames:` —
    /// the ratchet's numerator. Invocations are never counted here even if
    /// the meta-macro they call happens to produce a def with frames: an
    /// invocation file has no `frames:` field of its own to have authored,
    /// so it is not part of this ratchet's population at all (see
    /// [`scan_macro_dir`]'s doc).
    pub framed: usize,
}

impl std::ops::AddAssign for DefCensus {
    fn add_assign(&mut self, other: Self) {
        self.total_files += other.total_files;
        self.invocations += other.invocations;
        self.definitions += other.definitions;
        self.framed += other.framed;
    }
}

/// Sums [`scan_macro_dir`] over every `<plugin_dir>/macros` in `plugin_dirs`.
///
/// # Errors
/// If any plugin's `macros/` tree fails to read.
pub(super) fn scan_macro_dirs(plugin_dirs: &[PathBuf]) -> anyhow::Result<DefCensus> {
    let mut total = DefCensus::default();
    for plugin_dir in plugin_dirs {
        total += scan_macro_dir(&plugin_dir.join(deckmaste_core::plugin::MACROS_DIR))?;
    }
    Ok(total)
}

/// Classifies every `.ron` file under `macros_dir`, without loading a
/// [`Plugin`](deckmaste_cards::plugin::Plugin) at all: [`macro_set`] is an
/// *empty* registry — every card kind and param type registered, but no
/// macro *definitions* of its own — so reading a file's source against it
/// succeeds for a literal `MacroDef` struct (whose `frames:`/`body:` fields
/// are captured as authored, no macro expansion needed to read them) and
/// fails for a meta-macro *invocation* (its root is a bare identifier the
/// empty registry has no definition to resolve, e.g. `KeywordAbility(...)`),
/// the exact "bare (no macros registered)" distinction
/// `super::templates`'s own module doc describes (roughly an eighth of the
/// corpus reads this way). Independent per file — this never registers one
/// file's definition to help another file's invocation resolve, unlike
/// `Plugin::load`'s retry loop, since that resolution is irrelevant to the
/// definition/invocation classification itself.
///
/// # Errors
/// If `macros_dir` fails to read.
fn scan_macro_dir(macros_dir: &Path) -> anyhow::Result<DefCensus> {
    let bare = macro_set();
    let mut census = DefCensus::default();
    for path in ron_files_recursive(macros_dir)? {
        let source =
            fs::read_to_string(&path).with_context(|| format!("reading {}", path.display()))?;
        census.total_files += 1;
        match bare.read_str::<MacroDef>(&source) {
            Ok(def) => {
                census.definitions += 1;
                if !def.frames().is_empty() {
                    census.framed += 1;
                }
            }
            Err(_) => census.invocations += 1,
        }
    }
    Ok(census)
}

/// The `.ron` files under `dir` at any depth, sorted; an absent directory is
/// empty. A private copy of the same small walker every plugin-tree reader
/// in this workspace carries (`macro_ron::frames::ron_files_recursive`,
/// `deckmaste_cards::plugin::ron_files_recursive`, `crate::macros::templates`
/// and `crate::macros::pilot`'s own copies) — xtask depends on none of those
/// crates' internals, so this stays its own copy rather than a new public
/// API surface for one more caller.
fn ron_files_recursive(dir: &Path) -> anyhow::Result<Vec<PathBuf>> {
    if !dir.exists() {
        return Ok(Vec::new());
    }
    let mut files = Vec::new();
    let mut subdirs = Vec::new();
    for entry in fs::read_dir(dir).with_context(|| format!("reading {}", dir.display()))? {
        let entry = entry?;
        let path = entry.path();
        if entry.file_type()?.is_dir() {
            subdirs.push(path);
        } else if path.extension().is_some_and(|ext| ext == "ron") {
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

    const FRAMED_DEF: &str = r#"(
    name: "Framed",
    kinds: [OneShotEffect],
    params: [Reference],
    frames: ["<Param(0)> is framed"],
    body: 0,
)
"#;

    const UNFRAMED_DEF: &str = r#"(
    name: "Unframed",
    kinds: [OneShotEffect],
    params: [Reference],
    body: 0,
)
"#;

    const EMPTY_FRAMES_DEF: &str = r#"(
    name: "EmptyFrames",
    kinds: [OneShotEffect],
    params: [Reference],
    frames: [],
    body: 0,
)
"#;

    const INVOCATION: &str = "KeywordAbility(name: \"Bogus\", template: \"bogus\", abilities: [Composite(name: \"Bogus\", abilities: [])])\n";

    #[test]
    fn scan_counts_one_framed_and_one_unframed_definition() {
        let dir = tempdir_with(&[
            ("macros/action/Framed.ron", FRAMED_DEF),
            ("macros/action/Unframed.ron", UNFRAMED_DEF),
        ]);

        let census = scan_macro_dir(&dir.path().join("macros")).unwrap();

        assert_eq!(census.total_files, 2);
        assert_eq!(census.invocations, 0, "neither fixture file is a call");
        assert_eq!(census.definitions, 2);
        assert_eq!(
            census.framed, 1,
            "only `Framed` carries a non-empty frames:"
        );
    }

    #[test]
    fn scan_treats_an_explicitly_empty_frames_list_as_unframed() {
        let dir = tempdir_with(&[("macros/action/EmptyFrames.ron", EMPTY_FRAMES_DEF)]);

        let census = scan_macro_dir(&dir.path().join("macros")).unwrap();

        assert_eq!(census.definitions, 1);
        assert_eq!(census.framed, 0);
    }

    #[test]
    fn scan_classifies_a_meta_macro_invocation_separately_from_a_definition() {
        let dir = tempdir_with(&[
            ("macros/action/Framed.ron", FRAMED_DEF),
            ("macros/keyword/Invoked.ron", INVOCATION),
        ]);

        let census = scan_macro_dir(&dir.path().join("macros")).unwrap();

        assert_eq!(census.total_files, 2);
        assert_eq!(
            census.invocations, 1,
            "an identifier-prefixed root with nothing registered to resolve it is an invocation"
        );
        assert_eq!(
            census.definitions, 1,
            "the invocation must not also be counted as a definition"
        );
        assert_eq!(census.framed, 1, "`Framed` is the one framed definition");
    }

    #[test]
    fn scan_of_a_missing_directory_is_empty() {
        let dir = tempdir_with(&[]);
        let census = scan_macro_dir(&dir.path().join("no-such-macros-dir")).unwrap();
        assert_eq!(census, DefCensus::default());
    }

    #[test]
    fn scan_macro_dirs_sums_across_plugin_directories() {
        let builtin = tempdir_with(&[("macros/action/Framed.ron", FRAMED_DEF)]);
        let canon = tempdir_with(&[("macros/action/Unframed.ron", UNFRAMED_DEF)]);

        let census =
            scan_macro_dirs(&[builtin.path().to_path_buf(), canon.path().to_path_buf()]).unwrap();

        assert_eq!(census.total_files, 2);
        assert_eq!(census.definitions, 2);
        assert_eq!(census.framed, 1);
    }

    /// Minimal self-cleaning temp dir (avoids adding the `tempfile` crate) —
    /// same pattern `xtask::macros::templates`/`xtask::macros::inspect`
    /// already carry.
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
                "xtask-macro-census-{}-{}",
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
