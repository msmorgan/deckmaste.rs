//! `cargo xtask scaffold-identity` — writes one identity-macro def per
//! reachable `(kind, variant)` row that needs one (spec §5): a macro named
//! after its variant, expanding to that variant, mirroring the variant's
//! real arity and constructor defaults. Generated once as scaffolds, then
//! hand-owned — a scaffold is never overwritten once it exists, because
//! hand-written `frames:` data lives in these files afterward.
//!
//! A variant NAME can recur across multiple kinds (`Draw` at `EventFilter`
//! and at `OneShotEffect`/`KeywordAction`, `Creature` at `Predicate` and
//! `TypeDef`, …) — an existing, exercised pattern (`kinds: [A, B]` already
//! appears throughout the hand-written macro corpus) when every recurrence
//! shares one signature. So rows are grouped by variant name first: a group
//! that agrees on signature collapses into one `<Variant>.ron` def listing
//! every kind. A group that does NOT agree — same variant name, different
//! kinds each wanting a different arity/params — can't be squeezed into one
//! def (the params/body are one shared shape), so each distinct-signature
//! cluster gets its own `<Variant>~<kinds>.ron` file instead of a scaffold
//! that would be silently wrong for one of them.
//!
//! ## Optional-param elision
//!
//! A variant field carrying a constructor default is spelled BOTH ways by
//! the card corpus — supplied, and omitted for serde to fill — so a scaffold
//! that made it required would break the short spelling and one that dropped
//! it would break the long one. Such a param is scaffolded
//! `Elidable(Any)`: an omitted argument is forwarded as nothing, and
//! `macro_ron` drops the body entry holding its hole before the body is
//! read, so the destination applies exactly the default it applies for a
//! natively-read short spelling.
//!
//! ## Field splicing
//!
//! A newtype variant whose one field is itself a named struct
//! (`Ability::Activated(Arc<ActivatedAbility>)`, `OneShotEffect::May(May)`,
//! `Card::Normal(CardFace)`) is spelled FLAT by canon —
//! `Activated(cost: …, effect: …)`, never `Activated((cost: …))`. Its
//! signature is therefore `Named` over the payload struct's own fields (the
//! `#[macro_ron(spliced)]` marker plus `macro_ron::MacroFields`), and it
//! scaffolds like any other named signature. Before that, such a row
//! scaffolded as one opaque positional slot, which a macro call reads as
//! exactly one raw value — so it could not read canon's spelling at all.

use std::collections::BTreeMap;
use std::fmt::Write as _;
use std::path::Path;
use std::path::PathBuf;

use clap::Args;
use deckmaste_semantics::authoring::Coverage;
use deckmaste_semantics::authoring::UNCOVERABLE as UNSCAFFOLDABLE;
use deckmaste_semantics::authoring::classify;
use deckmaste_semantics::authoring::embed_safe;
use deckmaste_semantics::authoring::reachable_rows;
use macro_ron::KindSet;
use macro_ron::MacroDef;
use macro_ron::NamedParam;
use macro_ron::ParamDefault;
use macro_ron::VariantSignature;

#[derive(Debug, Args)]
pub struct ScaffoldIdentityArgs {
    /// List what would be written without writing anything.
    #[arg(long)]
    dry_run: bool,
}

/// This workspace's `plugins/builtin/macros/identity`, where scaffolds live.
fn identity_dir() -> PathBuf {
    Path::new(env!("CARGO_MANIFEST_DIR")).join("../../plugins/builtin/macros/identity")
}

// Rows `deckmaste_semantics::authoring::classify` marks `NeedsIdentityMacro`
// that this generator can't actually scaffold, because a downstream consumer
// rejects any macro def of that shape regardless of how it's typed —
// `UNSCAFFOLDABLE` (imported above as an alias of
// `deckmaste_semantics::authoring::UNCOVERABLE`, the coverage gate's own
// reason this same row is exempt, not just unscaffoldable; see its doc for
// the `(KeywordAbility, Composite)` detail).

/// `embed_safe` (`deckmaste_semantics::authoring::embed_safe`) is what filters
/// out an unsafe scaffold below. A bare self-referencing identity-macro body
/// (`body: Green`) is safe ONLY at a kind where the name is one of its own
/// direct declarations: reread there resolves as a genuine native variant and
/// stops. At an embedding kind, `deckmaste_plugin`'s embed-untagged
/// fallthrough treats "a macro of this name is registered at this kind" as
/// reason enough to skip the embedded type's own lookup
/// (`macro_ron::expand`'s embed-candidacy check), so the macro's own
/// bare-name body re-enters the SAME kind, finds the SAME macro again, and
/// never bottoms out — confirmed by generating a merged
/// `Green`/`AnyColor`/`Generic` scaffold and hitting `` `Green` is more than
/// 64 macro expansions deep`` from `cargo xtask validate` on real canon
/// tokens/basics. `embed_safe` used to be a hand-listed table here
/// (`ColorOrColorless`, `SimpleManaSymbol`, `ManaSymbol`, `ManaSpec`,
/// `ManaProduction`, `StatValue`, each paired with its own directly-declared
/// variant names, hand-verified against `deckmaste_semantics`'s source);
/// retired once `Kind::embeds`/`Kind::own_variants` made the same fact
/// computable, and moved to `deckmaste_semantics::authoring` so the coverage
/// gate can share it.
///
/// Every reachable row an identity macro must cover and this generator can
/// actually scaffold safely (see [`UNSCAFFOLDABLE`], [`embed_safe`]).
fn needs_identity_macro_rows() -> Vec<(String, String)> {
    reachable_rows()
        .into_iter()
        .filter(|(kind, variant)| classify(kind, variant) == Coverage::NeedsIdentityMacro)
        .filter(|(kind, variant)| {
            !UNSCAFFOLDABLE
                .iter()
                .any(|(k, v)| *k == kind && *v == variant)
        })
        .filter(|(kind, variant)| embed_safe(kind, variant))
        .collect()
}

/// # Errors
/// If the identity directory can't be created, a scaffold can't be written,
/// or a hand-owned file's `kinds:` doesn't cover a kind a merged cluster
/// needs (see [`plan`]'s doc).
pub fn run(args: &ScaffoldIdentityArgs) -> anyhow::Result<()> {
    let rows = needs_identity_macro_rows();
    let dir = identity_dir();
    if args.dry_run {
        // Read-only: no directory creation, no writes. `plan`/`declared_kinds`
        // both tolerate a missing `dir` (`Path::exists`/`read_to_string`
        // simply report "absent"), so this needs no special-casing beyond
        // skipping `create_dir_all`.
        let (planned, conflicts) = plan(&dir, &rows);
        for scaffold in &planned {
            println!("{}", scaffold.path.display());
        }
        eprintln!(
            "{} scaffold(s) would be written to {}",
            planned.len(),
            dir.display()
        );
        if !conflicts.is_empty() {
            eprintln!(
                "{} row(s) need a kind an existing hand-owned scaffold doesn't declare: {conflicts:?}",
                conflicts.len()
            );
        }
        return Ok(());
    }
    let written = scaffold_into(&dir, &rows)?;
    eprintln!("{written} scaffold(s) written to {}", dir.display());
    let registry_rows = read_committed_rows(&dir)?;
    std::fs::write(
        identity_registry_path(),
        render_identity_registry(&registry_rows),
    )?;
    eprintln!(
        "{} row(s) written to the compiled identity registry",
        registry_rows.len()
    );
    Ok(())
}

/// `crates/deckmaste_semantics/src/identity_registry.rs` — the compiled trust
/// channel `is_identity_exempt` consults (spec §6). Regenerated in full every
/// run from whatever `.ron` defs are actually committed under
/// `plugins/builtin/macros/identity/`, never hand-edited.
fn identity_registry_path() -> PathBuf {
    Path::new(env!("CARGO_MANIFEST_DIR")).join("../deckmaste_semantics/src/identity_registry.rs")
}

/// Every `(kind, variant, signature)` row the REAL, currently-committed
/// `.ron` defs under `dir` cover — read from the files themselves (each
/// parsed as an actual [`MacroDef`], not re-derived from
/// [`needs_identity_macro_rows`]) so a hand-added extra kind on an existing
/// def (the [`plan`] conflict path this generator surfaces rather than
/// silently accepting) is reflected here too, exactly as committed.
///
/// `signature` is `Debug`-rendered from the DEF'S OWN declared
/// [`Params`](macro_ron::Params) — what the file actually says it accepts —
/// not re-derived from the kind registry's variant signature (which every
/// consumer can already look up from `kind`+`variant` alone, making it inert
/// data). Recording the def's own params instead means a future consumer can
/// use this column for a REAL check: whether a def's declared shape still
/// matches its variant's true signature.
fn read_committed_rows(dir: &Path) -> anyhow::Result<Vec<(String, String, String)>> {
    let macros = deckmaste_semantics::macros::macro_set();
    let mut rows = Vec::new();
    if !dir.exists() {
        return Ok(rows);
    }
    for entry in std::fs::read_dir(dir)? {
        let path = entry?.path();
        if path.extension().and_then(std::ffi::OsStr::to_str) != Some("ron") {
            continue;
        }
        let text = std::fs::read_to_string(&path)?;
        let def: MacroDef = macros
            .read_str(&text)
            .map_err(|e| anyhow::anyhow!("{}: {e}", path.display()))?;
        let variant = def.name.as_str();
        let signature = render_params(&def.params);
        for kind in &def.kinds {
            rows.push((
                kind.as_str().to_string(),
                variant.to_string(),
                signature.clone(),
            ));
        }
    }
    rows.sort();
    rows.dedup();
    Ok(rows)
}

/// A deterministic `Debug`-style rendering of a def's declared params.
/// Deliberately NOT `format!("{params:?}")`: this v1 registry has always
/// canonicalized named signatures by parameter name, independent of authored
/// declaration order. Keeping that representation avoids spurious drift in
/// the generated registry's `signature` column (exactly the kind of drift
/// [`drift::compiled_registry_matches_the_real_identity_directory`]
/// exists to rule out, not manufacture). Sorted by param name instead, so
/// two reads of the same def always render identically.
fn render_params(params: &macro_ron::Params) -> String {
    match params {
        macro_ron::Params::Positional(types) => format!("Positional({types:?})"),
        macro_ron::Params::Named(map) => {
            let mut entries: Vec<(&str, &macro_ron::ParamType)> =
                map.iter().map(|(name, ty)| (name.as_str(), ty)).collect();
            entries.sort_by_key(|(name, _)| *name);
            format!("Named({entries:?})")
        }
    }
}

/// Renders `identity_registry.rs`'s full source — struct definition and all,
/// not just the row list — so every run produces a self-contained file
/// regardless of what a previous run left behind.
fn render_identity_registry(rows: &[(String, String, String)]) -> String {
    let mut out = String::new();
    out.push_str(
        "//! GENERATED by `cargo xtask scaffold-identity` from every `.ron` def under\n\
         //! `plugins/builtin/macros/identity/`. Do not hand-edit — a hand edit is\n\
         //! silently overwritten the next time the generator runs, and (spec §6) a\n\
         //! hand-editable trust channel could be forged. Regenerate after touching\n\
         //! that directory.\n\
         //!\n\
         //! This is the compiled half of the identity-macro exemption: a Rust\n\
         //! `const` slice, never serialized into RON. `deckmaste_semantics::authoring`\n\
         //! consults it (never the RON files directly) to decide whether a\n\
         //! `(kind, variant)` row is covered.\n\n\
         /// One row of the compiled identity registry: an identity-macro def named\n\
         /// `variant` is registered at kind `kind`. `signature` is `Debug`-rendered\n\
         /// from the DEF'S OWN declared `macro_ron::Params` (not the kind registry's\n\
         /// variant signature, which every consumer can already look up from `kind`\n\
         /// and `variant` alone) — a future consumer can use it to check a def's\n\
         /// declared shape against its variant's true signature.\n\
         #[derive(Debug, Clone, Copy, PartialEq, Eq)]\n\
         pub struct IdentityRow {\n    \
             pub kind: &'static str,\n    \
             pub variant: &'static str,\n    \
             pub signature: &'static str,\n\
         }\n\n\
         /// The compiled identity-macro registry, generated from the committed\n\
         /// `.ron` defs under `plugins/builtin/macros/identity/`.\n\
         pub const IDENTITY_ROWS: &[IdentityRow] = &[\n",
    );
    for (kind, variant, signature) in rows {
        let _ = writeln!(
            out,
            "    IdentityRow {{ kind: {kind:?}, variant: {variant:?}, signature: {signature:?} }},"
        );
    }
    out.push_str("];\n");
    out
}

/// One planned scaffold file: its destination path, the variant it's an
/// identity macro for, the kinds it applies at, and the signature the body
/// mirrors — everything [`render`] needs, computed once by [`plan`] so the
/// real writer and `--dry-run` can't drift apart.
struct Scaffold {
    path: PathBuf,
    variant: String,
    kinds: Vec<String>,
    signature: VariantSignature,
}

/// Writes a scaffold def for each row that has none. Existing files are left
/// exactly as found — defs are hand-owned once created (spec §5). Errors
/// (after writing whatever it safely could) if a cluster's target file
/// already exists but its own `kinds:` doesn't cover every kind the cluster
/// needs — see [`plan`]'s doc.
pub fn scaffold_into(dir: &Path, rows: &[(String, String)]) -> anyhow::Result<usize> {
    std::fs::create_dir_all(dir)?;
    let (planned, conflicts) = plan(dir, rows);
    let mut written = 0;
    for scaffold in &planned {
        // Belt-and-suspenders alongside `plan`'s own check: hand-ownership
        // (spec §5) is this generator's one load-bearing invariant, so the
        // write site re-checks rather than trusting a single earlier read.
        // Counted here (not `planned.len()`), so a skip here is reflected in
        // the return value instead of over-reporting.
        if scaffold.path.exists() {
            continue;
        }
        std::fs::write(
            &scaffold.path,
            render(&scaffold.variant, &scaffold.kinds, &scaffold.signature),
        )?;
        written += 1;
    }
    anyhow::ensure!(
        conflicts.is_empty(),
        "{} row(s) need a kind an existing hand-owned scaffold doesn't declare \
         (add it to that file's `kinds:` by hand): {conflicts:?}",
        conflicts.len(),
    );
    Ok(written)
}

/// Groups `rows` by variant name, clusters same-name rows by signature, and
/// returns one [`Scaffold`] per resulting file that doesn't already exist
/// under `dir`, plus any `(kind, variant)` CONFLICTS: a cluster whose target
/// file already exists but whose own `kinds:` list doesn't cover every kind
/// the cluster needs. Hand-ownership means such a file can never be
/// auto-corrected — but silently treating "the file exists" as "this kind is
/// covered" would repeat, one level up, the exact silent-partial-coverage bug
/// the variant-name/kind-name collision handling exists to avoid (a hand-add
/// of one more kind sharing a name and signature with an existing scaffold
/// must be surfaced, not swallowed). A variant name used by only one kind
/// never touches the signature registry until it's confirmed to need writing
/// at all, so a row naming an unregistered kind (as the never-overwrite unit
/// test does) is safe as long as its target file already exists.
fn plan(dir: &Path, rows: &[(String, String)]) -> (Vec<Scaffold>, Vec<(String, String)>) {
    let mut by_variant: BTreeMap<&str, Vec<&str>> = BTreeMap::new();
    for (kind, variant) in rows {
        by_variant
            .entry(variant.as_str())
            .or_default()
            .push(kind.as_str());
    }

    let kind_set = deckmaste_semantics::ron::kinds();
    let mut planned = Vec::new();
    let mut conflicts = Vec::new();
    for (variant, kinds) in by_variant {
        if let [only_kind] = kinds[..] {
            let path = dir.join(format!("{variant}.ron"));
            if path.exists() {
                continue;
            }
            let signature = signature_of(&kind_set, only_kind, variant);
            planned.push(Scaffold {
                path,
                variant: variant.to_string(),
                kinds: vec![only_kind.to_string()],
                signature,
            });
            continue;
        }

        for (filename, cluster_kinds, signature) in cluster_by_signature(&kind_set, variant, &kinds)
        {
            let path = dir.join(&filename);
            if path.exists() {
                let declared = declared_kinds(&path);
                for kind in &cluster_kinds {
                    if !declared.contains(kind) {
                        conflicts.push((kind.clone(), variant.to_string()));
                    }
                }
                continue;
            }
            planned.push(Scaffold {
                path,
                variant: variant.to_string(),
                kinds: cluster_kinds,
                signature,
            });
        }
    }
    (planned, conflicts)
}

/// The `kinds: [...]` an existing def file declares, read as plain text
/// (not a full RON parse — this is only ever used to sanity-check a
/// hand-owned file against what a NEW cluster needs, not to load the def).
/// Empty if the file can't be read or doesn't have a recognizable
/// `kinds: [...]` list.
fn declared_kinds(path: &Path) -> Vec<String> {
    let Ok(text) = std::fs::read_to_string(path) else {
        return Vec::new();
    };
    let Some(after_key) = text.find("kinds:").map(|i| i + "kinds:".len()) else {
        return Vec::new();
    };
    let rest = &text[after_key..];
    let Some(open) = rest.find('[') else {
        return Vec::new();
    };
    let Some(close) = rest[open..].find(']') else {
        return Vec::new();
    };
    rest[open + 1..open + close]
        .split(',')
        .map(|s| s.trim().to_string())
        .filter(|s| !s.is_empty())
        .collect()
}

/// This variant's real signature at `kind`, from the kind registry's
/// `SupportsMacros`-derived signature table.
///
/// # Panics
/// If `kind`/`variant` name nothing in that table — every row this is
/// actually called with comes from `reachable_rows`, which only yields rows
/// the registry dispatches, so this would mean the registry and the
/// reachability inventory have drifted apart.
fn signature_of(kind_set: &KindSet, kind: &str, variant: &str) -> VariantSignature {
    kind_set
        .get(kind)
        .and_then(|k| k.signatures().iter().find(|(name, _)| *name == variant))
        .unwrap_or_else(|| panic!("no signature for `{kind}::{variant}`"))
        .1
}

/// Splits a variant name's kinds into clusters that agree on signature.
/// The largest cluster (ties broken by its first kind, alphabetically)
/// keeps the plain `<Variant>.ron` name; each smaller, genuinely-divergent
/// cluster gets `<Variant>~<its kinds joined by "+">.ron` instead, since it
/// can't share a def with the majority.
fn cluster_by_signature(
    kind_set: &KindSet,
    variant: &str,
    kinds: &[&str],
) -> Vec<(String, Vec<String>, VariantSignature)> {
    let mut clusters: Vec<(VariantSignature, Vec<&str>)> = Vec::new();
    for &kind in kinds {
        let signature = signature_of(kind_set, kind, variant);
        match clusters.iter_mut().find(|(sig, _)| *sig == signature) {
            Some(cluster) => cluster.1.push(kind),
            None => clusters.push((signature, vec![kind])),
        }
    }
    clusters.sort_by(|a, b| b.1.len().cmp(&a.1.len()).then_with(|| a.1[0].cmp(b.1[0])));

    clusters
        .into_iter()
        .enumerate()
        .map(|(i, (signature, mut cluster_kinds))| {
            cluster_kinds.sort_unstable();
            let filename = if i == 0 {
                format!("{variant}.ron")
            } else {
                format!("{variant}~{}.ron", cluster_kinds.join("+"))
            };
            (
                filename,
                cluster_kinds.into_iter().map(str::to_string).collect(),
                signature,
            )
        })
        .collect()
}

/// Renders a scaffold identity-macro def: same name as the variant, valid at
/// every kind in `kinds`, arity and defaults mirrored from `signature`, body
/// forwarding the args back into the bare variant so the macro is a
/// transparent stand-in for it.
fn render(variant: &str, kinds: &[String], signature: &VariantSignature) -> String {
    let (params, args) = match signature {
        VariantSignature::Unit => (String::from("[]"), String::new()),
        VariantSignature::Positional(params) => positional_scaffold(params),
        VariantSignature::Named(params) => named_scaffold(params),
    };
    let body = if args.is_empty() { variant.to_string() } else { format!("{variant}({args})") };
    let kinds = kinds.join(", ");
    format!(
        "(\n    name: \"{variant}\",\n    kinds: [{kinds}],\n    params: {params},\n    body: {body},\n)\n"
    )
}

/// How many leading positional params a scaffold keeps REQUIRED: everything
/// before the trailing defaulted run — except that a signature of exactly ONE
/// param keeps it required whatever its default says.
///
/// That exception is not cosmetic. `macro_ron` reads a one-param positional
/// call through the newtype channel, which has no zero-argument spelling, so
/// `params: [Elidable(Any)]` would declare a short form nothing could invoke
/// — and the macro loader now refuses that shape outright
/// (`InsertError::LoneElidablePositional`), so emitting it would write a
/// scaffold that fails to load. Keeping the param required preserves the
/// LONG spelling, which is the one that exists; the short spelling was
/// already unreachable through a macro either way. (Two or more params read
/// through the tuple channel, which does admit `M()`, so a trailing elidable
/// run is fine from there up.)
fn positional_required(params: &[ParamDefault]) -> usize {
    if params.len() == 1 {
        return 1;
    }
    params.len()
        - params
            .iter()
            .rev()
            .take_while(|p| !matches!(p, ParamDefault::Required))
            .count()
}

/// Whether a scaffold for `signature` declares any `Elidable(...)` param.
/// Built on [`positional_required`], the same rule [`positional_scaffold`]
/// renders from, so the `elision_gap` regression test's expectation can't
/// drift from what [`render`] actually emits.
#[cfg(test)]
fn scaffold_elides(signature: &VariantSignature) -> bool {
    match signature {
        VariantSignature::Unit => false,
        VariantSignature::Named(params) => params
            .iter()
            .any(|p| !matches!(p.default, ParamDefault::Required)),
        VariantSignature::Positional(params) => positional_required(params) < params.len(),
    }
}

/// A positional signature's scaffold `params:` list and the matching
/// `Param(0), Param(1), …` forwarding list.
///
/// The params past [`positional_required`] are scaffolded `Elidable(Any)`: a
/// call may supply them or stop short, and a stopped-short call drops the
/// corresponding tuple elements from the body, leaving the variant's own
/// constructor defaults to fill them. That mirrors the derive's own
/// contiguous-suffix rule, which is why `macro_ron` requires elidable
/// positional params to be trailing in the first place.
///
/// A defaulted param that is NOT trailing (only reachable through an
/// embed-marked tuple variant — never observed in the current registry)
/// stays a plain required `Any`: dropping it would shift later values into
/// the wrong field, and `Elidable` can't express a hole in the middle.
fn positional_scaffold(params: &[ParamDefault]) -> (String, String) {
    let required = positional_required(params);
    let types: Vec<String> = (0..params.len())
        .map(|i| if i < required { "Any".to_string() } else { "Elidable(Any)".to_string() })
        .collect();
    let args: Vec<String> = (0..params.len()).map(|i| format!("Param({i})")).collect();
    (format!("[{}]", types.join(", ")), args.join(", "))
}

/// A named signature's scaffold `params:` map and the matching
/// `name: Param(name), …` forwarding list.
///
/// A param with any kind of constructor default is scaffolded
/// `Elidable(Any)`, so both spellings the corpus uses read: supplied, the
/// argument is forwarded; omitted, the body's `name: Param(name)` entry is
/// dropped and the field's own default applies. This is deliberately
/// independent of WHICH default flavor the signature carries — `Implicit`
/// has no RON rendering at all, and `Expr`'s captured text is Rust source
/// for the derive's own codegen, so neither can be spliced into a def file
/// as a fill expression.
fn named_scaffold(params: &[NamedParam]) -> (String, String) {
    let mut entries = Vec::new();
    let mut args = Vec::new();
    for param in params {
        let name = param.name;
        let ty = match param.default {
            ParamDefault::Required => "Any",
            ParamDefault::Implicit | ParamDefault::Expr(_) => "Elidable(Any)",
        };
        entries.push(format!("\"{name}\": {ty}"));
        args.push(format!("{name}: Param({name})"));
    }
    let params = if entries.is_empty() {
        "{}".to_string()
    } else {
        format!("{{ {} }}", entries.join(", "))
    };
    (params, args.join(", "))
}

#[cfg(test)]
mod tests {
    /// Hand-ownership is the invariant that makes `frames:` survivable: a
    /// regeneration that overwrote a def would destroy hand-written frames.
    #[test]
    fn scaffolding_never_overwrites_an_existing_def() {
        let dir = tempfile::tempdir().unwrap();
        let path = dir.path().join("Any.ron");
        std::fs::write(&path, "HAND-OWNED").unwrap();
        super::scaffold_into(dir.path(), &[("Filter".into(), "Any".into())]).unwrap();
        assert_eq!(std::fs::read_to_string(&path).unwrap(), "HAND-OWNED");
    }

    /// Two kinds sharing a variant name with the SAME signature (an
    /// established pattern — see e.g. `action/Destroy.ron`'s
    /// `kinds: [OneShotEffect, KeywordAction]`) collapse into one def
    /// listing both kinds, not two files racing for the same path.
    #[test]
    fn same_signature_collision_merges_into_one_multi_kind_def() {
        let dir = tempfile::tempdir().unwrap();
        let written = super::scaffold_into(
            dir.path(),
            &[
                ("Action".into(), "RestartGame".into()),
                ("OneShotEffect".into(), "RestartGame".into()),
            ],
        )
        .unwrap();
        assert_eq!(written, 1);
        let text = std::fs::read_to_string(dir.path().join("RestartGame.ron")).unwrap();
        assert!(text.contains("kinds: [Action, OneShotEffect]"), "{text}");
    }

    /// Two kinds sharing a variant name with DIFFERENT signatures (e.g.
    /// `Tap` is a required-arg tuple at `Action`/`OneShotEffect` but a unit
    /// variant at `CostComponent`) can't share one def — each
    /// distinct-signature cluster gets its own file instead of a scaffold
    /// that would be wrong for one of them.
    #[test]
    fn diverging_signature_collision_writes_one_file_per_cluster() {
        let dir = tempfile::tempdir().unwrap();
        let written = super::scaffold_into(
            dir.path(),
            &[
                ("Action".into(), "Tap".into()),
                ("CostComponent".into(), "Tap".into()),
            ],
        )
        .unwrap();
        assert_eq!(written, 2);
        let majority = std::fs::read_to_string(dir.path().join("Tap.ron")).unwrap();
        assert!(majority.contains("kinds: [Action]"), "{majority}");
        assert!(majority.contains("body: Tap(Param(0))"), "{majority}");
        let minority = std::fs::read_to_string(dir.path().join("Tap~CostComponent.ron")).unwrap();
        assert!(minority.contains("kinds: [CostComponent]"), "{minority}");
        assert!(minority.contains("body: Tap,"), "{minority}");
    }

    /// Every real `NeedsIdentityMacro` row lands in exactly one planned
    /// file's `kinds` list — no row is silently dropped by the collision
    /// handling, and the collision handling really does fire (fewer files
    /// than rows) rather than being dead code on today's registry.
    #[test]
    fn every_row_is_covered_by_exactly_one_planned_file() {
        let dir = tempfile::tempdir().unwrap();
        let rows = super::needs_identity_macro_rows();
        let (planned, conflicts) = super::plan(dir.path(), &rows);
        assert!(
            conflicts.is_empty(),
            "unexpected conflicts against an empty dir: {conflicts:?}"
        );
        let mut covered: Vec<(String, String)> = Vec::new();
        for scaffold in &planned {
            for kind in &scaffold.kinds {
                covered.push((kind.clone(), scaffold.variant.clone()));
            }
        }
        covered.sort();
        let mut expected = rows.clone();
        expected.sort();
        assert_eq!(covered, expected);
        assert!(
            planned.len() < rows.len(),
            "expected some variant names to collide across kinds and merge"
        );
    }

    /// A cluster whose target file already exists (hand-owned) but whose own
    /// `kinds:` doesn't cover every kind the cluster needs is a genuine gap,
    /// not a silent no-op: `Foo.ron` hand-authored for kind `A` only, then a
    /// later `B` sharing `Foo`'s name and signature, must surface `B` as
    /// still uncovered rather than being treated as "the file exists, so
    /// this row is done" — the same silent-partial-coverage failure the
    /// variant-name collision handling exists to avoid, one level up.
    #[test]
    fn pre_existing_file_missing_a_needed_kind_is_a_conflict_not_a_silent_skip() {
        let dir = tempfile::tempdir().unwrap();
        std::fs::write(
            dir.path().join("RestartGame.ron"),
            "(\n    name: \"RestartGame\",\n    kinds: [Action],\n    params: [],\n    body: RestartGame,\n)\n",
        )
        .unwrap();
        let err = super::scaffold_into(
            dir.path(),
            &[
                ("Action".into(), "RestartGame".into()),
                ("OneShotEffect".into(), "RestartGame".into()),
            ],
        )
        .unwrap_err();
        let message = err.to_string();
        assert!(message.contains("OneShotEffect"), "{message}");
        assert!(message.contains("RestartGame"), "{message}");
        // Hand-ownership held: the pre-existing file is untouched.
        assert_eq!(
            std::fs::read_to_string(dir.path().join("RestartGame.ron")).unwrap(),
            "(\n    name: \"RestartGame\",\n    kinds: [Action],\n    params: [],\n    body: RestartGame,\n)\n"
        );
    }
}

/// The compiled registry's whole reason to exist is that a loader can trust
/// it without re-reading RON — which only holds if it can't silently drift
/// from the `.ron` defs it claims to describe. Nothing enforced that: delete
/// or rename a def under `plugins/builtin/macros/identity/` without
/// re-running `cargo xtask scaffold-identity`, and `IDENTITY_ROWS` still
/// claims the row exists, forever, with no test to catch it.
#[cfg(test)]
mod drift {
    /// Compares the two as DATA (`IDENTITY_ROWS`'s own parsed rows against
    /// [`super::read_committed_rows`]'s fresh read of the real directory),
    /// never as generated-source TEXT — `render_identity_registry` emits
    /// one-line struct literals, but the committed file is `cargo fmt`'d
    /// multi-line, so a text/byte comparison would report spurious drift on
    /// formatting alone even with the rows perfectly in sync.
    #[test]
    fn compiled_registry_matches_the_real_identity_directory() {
        let mut from_disk = super::read_committed_rows(&super::identity_dir())
            .expect("the committed identity/ defs all still parse");
        from_disk.sort();

        let mut from_registry: Vec<(String, String, String)> =
            deckmaste_semantics::identity_registry::IDENTITY_ROWS
                .iter()
                .map(|row| {
                    (
                        row.kind.to_string(),
                        row.variant.to_string(),
                        row.signature.to_string(),
                    )
                })
                .collect();
        from_registry.sort();

        assert_eq!(
            from_registry, from_disk,
            "identity_registry.rs has drifted from plugins/builtin/macros/identity/ — \
             run `cargo xtask scaffold-identity` to regenerate"
        );
    }
}

/// The drift test above compares `(kind, variant, signature)` ROWS. A body can
/// satisfy every one of those while constructing something else entirely — and
/// these files are hand-owned by design, so nothing but a test can hold their
/// shape.
#[cfg(test)]
mod identity_shape {
    use std::collections::BTreeSet;

    /// Every committed identity def is a literal mirror of its own variant: the
    /// body constructs the variant the def is named for, and forwards exactly
    /// its declared params — no extras, none dropped.
    ///
    /// The nested-invocation case is the one that matters. An identity body
    /// calling another macro would put the card's argument text one hop further
    /// from the call site that wrote it, and these defs all declare `Any`
    /// params, whose validator accepts anything.
    #[test]
    fn every_identity_def_mirrors_its_own_variant() {
        let macros = deckmaste_semantics::macros::macro_set();
        let dir = super::identity_dir();
        let mut checked = 0usize;
        let mut offenders = Vec::new();
        for entry in std::fs::read_dir(&dir).expect("identity/ is readable") {
            let path = entry.expect("directory entry").path();
            if path.extension().and_then(std::ffi::OsStr::to_str) != Some("ron") {
                continue;
            }
            let text = std::fs::read_to_string(&path).expect("def file is readable");
            let def: macro_ron::MacroDef = macros
                .read_str(&text)
                .unwrap_or_else(|e| panic!("{}: {e}", path.display()));
            checked += 1;
            let head = def.body_head(&macros);
            let head = head.as_ref().map(macro_ron::Ident::as_str);
            if head != Some(def.name.as_str()) {
                offenders.push(format!(
                    "{}: body constructs `{}`, not `{}`",
                    path.display(),
                    head.unwrap_or("<no leading identifier>"),
                    def.name,
                ));
                continue;
            }
            let used: BTreeSet<String> = def
                .body_param_keys(&macros)
                .unwrap_or_else(|e| panic!("{}: {e}", path.display()))
                .into_iter()
                .collect();
            let declared: BTreeSet<String> = match &def.params {
                macro_ron::Params::Positional(types) => {
                    (0..types.len()).map(|i| i.to_string()).collect()
                }
                macro_ron::Params::Named(map) => {
                    map.iter().map(|(key, _)| key.as_str().to_owned()).collect()
                }
            };
            if used != declared {
                offenders.push(format!(
                    "{}: body holes {used:?} do not match declared params {declared:?}",
                    path.display(),
                ));
            }
        }
        // Vacuity floor: an empty or misdirected walk would otherwise pass.
        assert!(
            checked > 0,
            "no identity defs found under {} — the walk is broken, not the directory clean",
            dir.display()
        );
        assert!(
            offenders.is_empty(),
            "an identity def must construct its own variant and forward exactly its \
             declared params:\n{}",
            offenders.join("\n")
        );
    }
}

#[cfg(test)]
mod coverage_gap {
    /// Every `NeedsIdentityMacro` row this generator declines to scaffold is
    /// accounted for by one of the two documented exclusions
    /// ([`super::UNSCAFFOLDABLE`], [`super::embed_safe`]) — never a silent
    /// third reason. The gap itself (currently real: `deckmaste_plugin`'s
    /// `keyword_shape` closed vocabulary for `Composite`, and the
    /// embed-untagged recursion hazard for names only reachable at an
    /// embedding kind transitively) is reported upstream, not hidden here.
    #[test]
    fn every_excluded_row_is_an_explained_exclusion() {
        use deckmaste_semantics::authoring::Coverage;
        use deckmaste_semantics::authoring::classify;
        use deckmaste_semantics::authoring::reachable_rows;

        let all_needs: Vec<(String, String)> = reachable_rows()
            .into_iter()
            .filter(|(k, v)| classify(k, v) == Coverage::NeedsIdentityMacro)
            .collect();
        let scaffoldable = super::needs_identity_macro_rows();
        for row @ (kind, variant) in &all_needs {
            if scaffoldable.contains(row) {
                continue;
            }
            let listed = super::UNSCAFFOLDABLE
                .iter()
                .any(|(k, v)| *k == kind && *v == variant);
            assert!(
                listed || !super::embed_safe(kind, variant),
                "{kind}::{variant} was excluded without an explanation"
            );
        }
    }
}

#[cfg(test)]
mod elision_gap {
    /// Every row with a droppable constructor default is scaffolded, and its
    /// scaffold declares that param `Elidable(Any)` — the shape that keeps
    /// BOTH corpus spellings reading. These rows used to be excluded outright
    /// (no def-language form could express "forward it if given, drop it if
    /// not"), which is what Critical Finding 1 was about; the standing
    /// regression is now that they are covered rather than that they are
    /// skipped.
    #[test]
    fn every_droppable_default_row_is_scaffolded_as_elidable() {
        use deckmaste_semantics::authoring::Coverage;
        use deckmaste_semantics::authoring::classify;
        use deckmaste_semantics::authoring::reachable_rows;
        use macro_ron::ParamDefault;
        use macro_ron::VariantSignature;

        let kind_set = deckmaste_semantics::ron::kinds();
        let droppable = |signature: &VariantSignature| match signature {
            VariantSignature::Unit => false,
            VariantSignature::Named(params) => params
                .iter()
                .any(|p| !matches!(p.default, ParamDefault::Required)),
            VariantSignature::Positional(params) => {
                params.iter().any(|p| !matches!(p, ParamDefault::Required))
            }
        };
        let rows: Vec<(String, String)> = reachable_rows()
            .into_iter()
            .filter(|(k, v)| classify(k, v) == Coverage::NeedsIdentityMacro)
            .filter(|(k, v)| droppable(&super::signature_of(&kind_set, k, v)))
            .collect();
        assert!(
            !rows.is_empty(),
            "expected the shape to occur on today's registry"
        );

        // Coverage: none of them is withheld any more.
        let scaffoldable = super::needs_identity_macro_rows();
        let missing: Vec<_> = rows
            .iter()
            .filter(|row| !scaffoldable.contains(row))
            .collect();
        assert!(missing.is_empty(), "{missing:?}");

        // Rendering: `Elidable(Any)` appears exactly where the generator's own
        // rule says it should — never where a param must stay required (a lone
        // positional default, or one that isn't trailing), since `macro_ron`
        // refuses the first outright and the second can't be expressed.
        let mut elided_any = false;
        for (kind, variant) in &rows {
            let signature = super::signature_of(&kind_set, kind, variant);
            let text = super::render(variant, std::slice::from_ref(kind), &signature);
            if super::scaffold_elides(&signature) {
                elided_any = true;
                assert!(
                    text.contains("Elidable(Any)"),
                    "{kind}::{variant} scaffolded without an elidable param:\n{text}"
                );
            } else {
                assert!(
                    !text.contains("Elidable("),
                    "{kind}::{variant} scaffolded elidable against the rule:\n{text}"
                );
            }
        }
        assert!(
            elided_any,
            "expected some row to actually scaffold elidable"
        );
    }

    /// A lone defaulted positional param stays REQUIRED: `macro_ron` refuses
    /// `params: [Elidable(Any)]` (no zero-argument spelling exists for a
    /// one-param call), so emitting it would write a scaffold that can't
    /// load. This is the guard that replaced the retired `blocked_on_elision`
    /// panic for the one shape elision genuinely cannot serve.
    #[test]
    fn a_lone_defaulted_positional_param_stays_required() {
        use macro_ron::ParamDefault;
        use macro_ron::VariantSignature;

        let signature = VariantSignature::Positional(&[ParamDefault::Expr("None")]);
        assert!(!super::scaffold_elides(&signature));
        // A real `Action` variant name, so the loader's cycle check reads the
        // body's head as the native variant it is rather than a self-call.
        let text = super::render("Cast", &["Action".to_string()], &signature);
        assert!(text.contains("params: [Any],"), "{text}");
        assert!(text.contains("body: Cast(Param(0)),"), "{text}");

        // And what it renders actually loads — the point of the guard is that
        // the alternative (`Elidable(Any)`) would NOT.
        let mut macros = deckmaste_semantics::macros::macro_set();
        let def: macro_ron::MacroDef = macros.read_str(&text).unwrap();
        macros.replace(&def).unwrap();

        let elidable = text.replace("params: [Any],", "params: [Elidable(Any)],");
        let bad: macro_ron::MacroDef = macros.read_str(&elidable).unwrap();
        assert!(matches!(
            macros.replace(&bad),
            Err(macro_ron::InsertError::LoneElidablePositional { .. })
        ));
    }
}

#[cfg(test)]
mod field_splice {
    /// A newtype variant over a named struct is spelled field-spliced by
    /// canon (`May(who: …)`, never `May((who: …))`), so its identity macro
    /// must take a NAMED signature over the payload struct's own fields —
    /// not the one positional slot a bare newtype tuple would suggest, which
    /// a macro call reads as exactly one raw value.
    #[test]
    fn a_newtype_over_a_named_struct_scaffolds_field_spliced() {
        use macro_ron::ParamDefault;
        use macro_ron::VariantSignature;

        let kind_set = deckmaste_semantics::ron::kinds();
        let signature = super::signature_of(&kind_set, "OneShotEffect", "May");
        let VariantSignature::Named(params) = signature else {
            panic!("OneShotEffect::May should carry a named signature, got {signature:?}");
        };
        let shape: Vec<(&str, bool)> = params
            .iter()
            .map(|p| (p.name, matches!(p.default, ParamDefault::Required)))
            .collect();
        assert_eq!(
            shape,
            [
                ("who", true),
                ("effect", true),
                ("if_did", false),
                ("if_not", false),
            ]
        );

        let text = super::render("May", &["OneShotEffect".to_string()], &signature);
        assert!(
            text.contains(r#"params: { "who": Any, "effect": Any, "if_did": Elidable(Any), "if_not": Elidable(Any) }"#),
            "{text}"
        );
        assert!(
            text.contains(
                "body: May(who: Param(who), effect: Param(effect), \
                 if_did: Param(if_did), if_not: Param(if_not)),"
            ),
            "{text}"
        );
    }

    /// A field-spliced payload field with no `#[serde(default)]` but an
    /// `Option` type still fills itself in when omitted (serde's own
    /// missing-field-is-`None` rule), so it scaffolds `Elidable(Any)` too —
    /// `CardFace::power` and kin, the shape `identity/Normal.ron` was
    /// hand-written around.
    #[test]
    fn an_optional_payload_field_scaffolds_elidable_without_a_serde_default() {
        use macro_ron::ParamDefault;
        use macro_ron::VariantSignature;

        let kind_set = deckmaste_semantics::ron::kinds();
        let signature = super::signature_of(&kind_set, "Card", "Normal");
        let VariantSignature::Named(params) = signature else {
            panic!("Card::Normal should carry a named signature, got {signature:?}");
        };
        let power = params
            .iter()
            .find(|p| p.name == "power")
            .expect("CardFace has a `power` field");
        assert_eq!(power.default, ParamDefault::Implicit);
    }

    /// The marker is opt-in, so the SET of marked variants is a
    /// hand-maintained artifact — the exact shape of thing that shipped this
    /// bug in the first place (the retired hand-listed hazard table named 16
    /// of the 19 real rows; `OneShotEffect::Delayed` is spelled in canon and
    /// still hid for the table's whole life, because nothing exercised it).
    /// So the invariant is asserted mechanically instead of remembered:
    /// **every newtype variant of a `SupportsMacros` enum whose payload —
    /// peeled of `Arc`/`Box`/`Rc` — names a named struct declared in the
    /// crate must carry `#[macro_ron(spliced)]`.** Anything else scaffolds as
    /// one opaque positional slot and cannot read canon's field-spliced
    /// spelling of itself.
    ///
    /// The `SupportsMacros` qualifier is load-bearing, not incidental: a
    /// newtype-over-named-struct variant in an enum with no macro dispatch
    /// (`Cause::Cause`, `FaceDownSpec::Listed`, `TokenSpec::Token`) is
    /// correctly unmarked — it never reaches the macro layer, so it has no
    /// signature anyone scaffolds from.
    ///
    /// Reads the sources the kind registry is built from, the way
    /// `cargo xtask map enums` does — the derive itself cannot answer this
    /// (a proc macro sees only its own item's tokens, and can never tell
    /// whether another type is a struct or an enum).
    #[test]
    fn every_newtype_over_a_crate_struct_is_marked_spliced() {
        let dir = std::path::Path::new(env!("CARGO_MANIFEST_DIR"))
            .join("../../crates/deckmaste_semantics/src");
        let mut sources = Vec::new();
        crate::map::collect_rs_files(&dir, &mut sources).expect("semantics sources are readable");
        sources.sort();
        let parsed: Vec<syn::File> = sources
            .iter()
            .map(|path| {
                let text = std::fs::read_to_string(path).expect("source is readable");
                syn::parse_file(&text).unwrap_or_else(|e| panic!("parsing {}: {e}", path.display()))
            })
            .collect();

        let mut structs: std::collections::BTreeSet<String> = std::collections::BTreeSet::new();
        for file in &parsed {
            collect_named_structs(&file.items, &mut structs);
        }
        assert!(
            structs.contains("CardFace") && structs.contains("May"),
            "the struct inventory did not come out — {} names found",
            structs.len()
        );

        let mut unmarked = Vec::new();
        let mut marked = 0usize;
        for file in &parsed {
            let mut enums = Vec::new();
            crate::map::collect_pub_enums(&file.items, &mut enums);
            for item in enums.iter().filter(|e| derives_supports_macros(&e.attrs)) {
                for variant in &item.variants {
                    let syn::Fields::Unnamed(fields) = &variant.fields else {
                        continue;
                    };
                    let [field] = &fields.unnamed.iter().collect::<Vec<_>>()[..] else {
                        continue;
                    };
                    if !payload_name(&field.ty).is_some_and(|name| structs.contains(&name)) {
                        continue;
                    }
                    if is_spliced(&variant.attrs) {
                        marked += 1;
                    } else {
                        unmarked.push(format!("{}::{}", item.ident, variant.ident));
                    }
                }
            }
        }
        assert!(marked > 0, "the walk found no spliced variants at all");
        assert!(
            unmarked.is_empty(),
            "these newtype-over-named-struct variants are missing \
             `#[macro_ron(spliced)]`, so they scaffold as one opaque positional \
             slot and cannot read their own field-spliced spelling: {unmarked:?}"
        );
    }

    /// Every named struct declared under the walked sources, by ident,
    /// descending into inline `mod { … }` blocks. Visibility is deliberately
    /// NOT filtered: a payload named by a public variant is reachable
    /// whatever its own `pub` spelling says.
    fn collect_named_structs(items: &[syn::Item], out: &mut std::collections::BTreeSet<String>) {
        for item in items {
            match item {
                syn::Item::Struct(s) if matches!(s.fields, syn::Fields::Named(_)) => {
                    out.insert(s.ident.to_string());
                }
                syn::Item::Mod(m) => {
                    if let Some((_, inner)) = &m.content {
                        collect_named_structs(inner, out);
                    }
                }
                _ => {}
            }
        }
    }

    /// Whether the item's `#[derive(...)]` lists `SupportsMacros`.
    fn derives_supports_macros(attrs: &[syn::Attribute]) -> bool {
        attrs
            .iter()
            .filter(|a| a.path().is_ident("derive"))
            .any(|a| {
                let mut found = false;
                let _ = a.parse_nested_meta(|meta| {
                    found |= meta.path.is_ident("SupportsMacros");
                    Ok(())
                });
                found
            })
    }

    /// Whether the variant carries `#[macro_ron(spliced)]`.
    fn is_spliced(attrs: &[syn::Attribute]) -> bool {
        attrs
            .iter()
            .filter(|a| a.path().is_ident("macro_ron"))
            .any(|a| {
                let mut found = false;
                let _ = a.parse_nested_meta(|meta| {
                    found |= meta.path.is_ident("spliced");
                    Ok(())
                });
                found
            })
    }

    /// A newtype payload's own type name, peeled of `Arc`/`Box`/`Rc` —
    /// mirroring `macro_ron_derive::generate::unwrapped`, which is what the
    /// derive resolves `MacroFields` through. `None` for anything that isn't
    /// a plain path (slices, tuples, references).
    fn payload_name(ty: &syn::Type) -> Option<String> {
        let syn::Type::Path(path) = ty else {
            return None;
        };
        let segment = path.path.segments.last()?;
        if matches!(segment.ident.to_string().as_str(), "Arc" | "Box" | "Rc")
            && let syn::PathArguments::AngleBracketed(args) = &segment.arguments
            && let Some(syn::GenericArgument::Type(inner)) = args.args.first()
        {
            return payload_name(inner);
        }
        Some(segment.ident.to_string())
    }
}

/// Reads real, generated scaffolds through an ACTUAL restricted read —
/// Critical Finding 2: `cargo xtask validate`/`cargo test --workspace` going
/// green only ever proved a scaffold PARSES as a macro def; nothing in the
/// suite up to this point ever read a card position with restriction on,
/// which is the only path (besides the embed-candidacy check that caught
/// Finding 3) that consults `macros.get` before a native variant shadows it
/// — see `macro_ron::expand`'s `native_variant_ok`. These tests build the
/// real `deckmaste_semantics::macros::macro_set()` registry, load EVERY
/// scaffold under `plugins/builtin/macros/identity/` (the actual generated
/// files, not a synthetic stand-in — a nested argument position, e.g. the
/// `Reference` a `Cast`/`Attach` call carries, needs ITS OWN identity-macro
/// coverage under restriction too, so an isolated single-def `MacroSet` can't
/// exercise a realistic card-shaped read), and read real card-shaped RON
/// through [`macro_ron::MacroSet::read_str_restricted`], covering: a unit
/// variant, a positional variant with every param required, a named/struct
/// variant with every param required, embed fallthrough down to a defining
/// kind, both spellings of an `Elidable(Any)` param — the short one that
/// omits it and the long one that supplies the value its field default would
/// otherwise fill — and every field-spliced newtype-over-named-struct row,
/// one by one.
#[cfg(test)]
mod restricted_read {
    use deckmaste_semantics::Action;
    use deckmaste_semantics::Card;
    use deckmaste_semantics::ManaProduction;
    use macro_ron::Expand;
    use macro_ron::MacroDef;

    /// The real semantics macro registry (kinds + param types) with every
    /// real, currently-committed scaffold loaded — see the module doc.
    fn real_scaffolds() -> macro_ron::MacroSet {
        let mut macros = deckmaste_semantics::macros::macro_set();
        for entry in std::fs::read_dir(super::identity_dir()).unwrap() {
            let path = entry.unwrap().path();
            let text = std::fs::read_to_string(&path).unwrap();
            let def: MacroDef = macros
                .read_str(&text)
                .unwrap_or_else(|e| panic!("{}: {e}", path.display()));
            macros
                .insert(&def)
                .unwrap_or_else(|e| panic!("{}: {e}", path.display()));
        }
        macros
    }

    /// A restricted read of `source` at type `T` must produce exactly what
    /// an UNRESTRICTED (native) read of the same text does — that equality
    /// is the whole point of an identity macro: routing through the macro
    /// changes nothing observable. `expand_all` strips the invocation
    /// PROVENANCE a "remembers expansion" kind (`Action` among them) wraps
    /// a macro-routed read in — expected and correct (a restricted read of
    /// *any* name always goes through a macro, by construction, so it
    /// always remembers; a native/unrestricted read never does) and not
    /// itself the thing under test here, which is the VALUE.
    fn assert_restricted_matches_native<T>(macros: &macro_ron::MacroSet, source: &str)
    where
        T: serde::de::DeserializeOwned + Expand + PartialEq + std::fmt::Debug,
    {
        let restricted: T = macros
            .read_str_restricted(source)
            .unwrap_or_else(|e| panic!("restricted read of {source:?} failed: {e}"));
        let native: T = macros.read_str(source).unwrap();
        assert_eq!(
            restricted.expand_all(),
            native.expand_all(),
            "restricted vs. native diverged for {source:?}"
        );
    }

    /// Unit variant, no params — `Action::RestartGame`.
    #[test]
    fn unit_variant_scaffold_round_trips_under_restriction() {
        let macros = real_scaffolds();
        assert_restricted_matches_native::<Action>(&macros, "RestartGame");
    }

    /// Positional variant, every param required (no dropped default) —
    /// `Action::AddMana(Reference, Count, ManaProduction)`. Every argument
    /// (`You`, `White`) is itself restricted vocabulary needing its own
    /// coverage — `Reference::You` has a real scaffold, and `White`
    /// exercises the embed-fallthrough chain (`ManaProduction` embeds
    /// `ManaSpec` embeds `ColorOrColorless` embeds `Color`) under
    /// restriction: no macro named `White` exists at any of the three
    /// embedding kinds (Finding 3's exclusion holds even in the real,
    /// fully-loaded set), only at `Color` itself.
    #[test]
    fn positional_all_required_scaffold_round_trips_under_restriction() {
        let macros = real_scaffolds();
        assert_restricted_matches_native::<Action>(&macros, "AddMana(You, 1, White)");
    }

    /// The same embed-fallthrough claim, isolated: reading a bare color
    /// directly at a `ManaProduction` position.
    #[test]
    fn embed_fallthrough_reaches_the_defining_kind_under_restriction() {
        let macros = real_scaffolds();
        assert_restricted_matches_native::<ManaProduction>(&macros, "White");
    }

    /// Named/struct variant, every param required — `Action::Attach { what:
    /// Reference, to: Reference }`.
    #[test]
    fn named_all_required_scaffold_round_trips_under_restriction() {
        let macros = real_scaffolds();
        assert_restricted_matches_native::<Action>(&macros, "Attach(what: This, to: You)");
    }
    /// The two-directional gate this capability exists for: an elidable
    /// param's LONG spelling (supplying the value the field's default would
    /// have filled) and its SHORT one (omitting it) must both read, and both
    /// must produce exactly what a native read of the same text produces.
    /// `EventFilter::Cast` is the real shape — both `who` and `what` carry
    /// `#[serde(default = "Predicate::any")]`, and canon spells it
    /// `Cast(who: Ref(You))`, one supplied and one omitted.
    #[test]
    fn an_elidable_params_spellings_all_round_trip_under_restriction() {
        let macros = real_scaffolds();
        for source in [
            "Cast(who: Ref(You), what: Any)",
            "Cast(who: Ref(You))",
            "Cast(what: Any)",
            "Cast()",
        ] {
            assert_restricted_matches_native::<deckmaste_semantics::EventFilter>(&macros, source);
        }
    }

    /// The same, positionally: `Action::Cast`'s trailing alternative-cost
    /// slot carries a constructor default, so a call may stop short of it —
    /// `Cast(You, That(Card))` is canon's own spelling.
    #[test]
    fn an_elidable_positional_params_arities_round_trip_under_restriction() {
        let macros = real_scaffolds();
        for source in ["Cast(You, That(Card))", "Cast(You, That(Card), [Tap])"] {
            assert_restricted_matches_native::<Action>(&macros, source);
        }
    }

    /// The field-splice gate, per row: EVERY newtype-over-named-struct
    /// variant, read at the spelling canon uses for it. This is the shape
    /// that used to fail outright — a single positional slot reads exactly
    /// one raw value, and `key: value` pairs are not one — so a scaffold
    /// only counts as fixed when its variant's own field-spliced spelling
    /// reads restricted and equals the native read.
    ///
    /// Sources take their SHAPE from the committed corpus where the row occurs
    /// there (`Activated`/`Triggered`/`Continuously`/`May`/`If`/`Each`/`With`/
    /// `Modal`/`Delayed`); several are trimmed or recombined so every nested
    /// name is one this set actually loads (a builtin macro living outside
    /// `identity/` is not), and the rows canon does not yet spell get a
    /// minimal hand-built value of the same shape.
    #[test]
    fn every_field_spliced_row_round_trips_under_restriction() {
        let macros = real_scaffolds();
        for source in [
            "Activated(cost: [Mana([Generic(1)]), Tap], effect: RestartGame)",
            "Triggered(event: StepBegins(at: Beginning(Upkeep), whose: Your), \
             effect: RestartGame)",
            "Spell(effect: RestartGame)",
        ] {
            assert_restricted_matches_native::<deckmaste_semantics::Ability>(&macros, source);
        }
        for source in [
            "Continuously(effect: Modify(It, SwitchPowerToughness), \
             duration: FixedUntil(EndOfTurn))",
            "Label(as: \"pile\", effect: RestartGame)",
            "SeparatePiles(group: SelectAll(Attacking), into: [\"left\", \"right\"])",
            "ChoosePile(from: Labels([\"left\", \"right\"]), then: RestartGame)",
            // Both spellings of the elidable half: canon's short one, and the
            // long one that supplies what the field default would have filled.
            "May(who: You, effect: RestartGame)",
            "May(who: You, effect: RestartGame, if_did: RestartGame, if_not: RestartGame)",
            "If(condition: Not(Exists(Attacking)), then: Tap(This))",
            "AdditionalCost(pay: [Do(Sacrifice(You, This))], body: RestartGame)",
            "Each(binder: Existing(SelectAll(Attacking)), effect: RestartGame)",
            "With(binder: TheRef(This), body: RestartGame)",
            "Distribute(amount: 1, binder: Existing(SelectAll(Attacking)), body: RestartGame)",
            "Noting(key: \"milled\", effect: RestartGame)",
            "Modal(choose: ChooseSpec(chooser: You, count: Range(1, 1)), \
             modes: [Mode(effect: RestartGame)])",
            "RevealUntil(whose: You, matches: Attacking, body: RestartGame)",
            "Delayed(event: StepBegins(at: Ending(End), whose: EachPlayers), \
             effect: RestartGame)",
            "Reflexive(event: StepBegins(at: Ending(End), whose: EachPlayers), \
             effect: RestartGame)",
        ] {
            assert_restricted_matches_native::<deckmaste_semantics::OneShotEffect>(&macros, source);
        }
        // `StaticEffect::CostOption` — the row the MECHANICAL survey caught
        // that two hand surveys missed, and spelled 4 times in canon.
        for source in [
            "CostOption(components: [Tap], tag: Kicker)",
            "CostOption(components: [Tap], tag: Kicker, repeatable: true)",
        ] {
            assert_restricted_matches_native::<deckmaste_semantics::StaticEffect>(&macros, source);
        }
    }

    /// `Card` is the restricted root itself — every committed card file's
    /// top-level shape (see `plugins/builtin/cards/Forest.ron`, used
    /// verbatim here). `Card::Normal(CardFace)` is the original
    /// newtype-over-named-struct row: hand-authored field-spliced once the
    /// coverage gate surfaced it, now generated from `CardFace`'s own fields
    /// like the rest of its class.
    #[test]
    fn card_normal_scaffold_round_trips_under_restriction() {
        let macros = real_scaffolds();
        assert_restricted_matches_native::<Card>(&macros, "Normal(name: \"Forest\", types: [])");
    }

    /// The other half of `Card`'s dispatch: `TwoFaced { layout, front, back
    /// }`, a named/struct variant with every param required.
    #[test]
    fn card_two_faced_scaffold_round_trips_under_restriction() {
        let macros = real_scaffolds();
        assert_restricted_matches_native::<Card>(
            &macros,
            "TwoFaced(layout: Transforming, front: (name: \"Front\", types: []), \
             back: (name: \"Back\", types: []))",
        );
    }
}
