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
//! ## BLOCKED: optional-param elision
//!
//! A row whose signature has a defaulted param that can't be dropped
//! (`blocked_on_elision`) gets no scaffold at all, rather than one that
//! drops the param and silently breaks the long-form spelling. `macro_ron`
//! has no def-language form for "forward this argument if the call gives
//! it, omit it entirely from the body if not" — only "fill an omitted arg
//! with a known expression," which isn't the same thing and isn't even
//! available for a positional param or for a named param whose default has
//! no captured expression (every named param in practice). Tracked at
//! `docs/tickets/planned/macro-ron-optional-param-elision.md`, which carries
//! the sized proposal; this generator's `macro-author-surface` restriction
//! flip is blocked on it for the affected rows.

use std::collections::BTreeMap;
use std::path::Path;
use std::path::PathBuf;

use clap::Args;
use deckmaste_semantics::authoring::Coverage;
use deckmaste_semantics::authoring::classify;
use deckmaste_semantics::authoring::reachable_rows;
use macro_ron::KindSet;
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

/// A row `deckmaste_semantics::authoring::classify` marks `NeedsIdentityMacro`
/// that this generator can't actually scaffold, because a downstream
/// consumer rejects any macro def of that shape regardless of how it's
/// typed. Kept separate from `deckmaste_semantics::authoring`'s own
/// (kind, variant) whitelists — that crate can't see `deckmaste_plugin`'s
/// loader — and reported rather than silently forced into a scaffold that
/// cannot load.
///
/// `(KeywordAbility, Composite)`: every `KeywordAbility`-kind macro def is
/// run through `deckmaste_plugin`'s `keyword_shape`, a CLOSED vocabulary of
/// param shapes ([CR#702] one-liners: none/count/cost/count+cost/
/// predicate/predicate+cost/name). `Composite`'s real shape (`name: Ident,
/// abilities: Vec<Ability>`) fits none of them, so no def of that name at
/// that kind can load — not with `Any` params, not even with the real
/// types. `Composite` is itself the STRUCTURAL PAYLOAD a keyword macro's own
/// body expands into (`Ward([…])` -> `Composite(name: "Ward", abilities:
/// [...])`; see `KeywordAbility::Composite`'s doc), never spelled bare by a
/// card author — the same machinery role `Expanded` plays elsewhere, just
/// under a different name and so not caught by that filter. This is the one
/// entry that genuinely belongs here: nothing types `Composite` into loading,
/// so a local exclusion is the only fix available to a scaffold generator.
///
/// The five `KeywordAbility::ALL` intrinsics used to be listed here too, but
/// belong upstream instead — `classify` now returns `NativeAtom` for them
/// (`deckmaste_semantics::authoring::NATIVE_ATOMS`), so they never reach this
/// generator as `NeedsIdentityMacro` rows in the first place.
const UNSCAFFOLDABLE: &[(&str, &str)] = &[("KeywordAbility", "Composite")];

/// Kinds this generator can't scaffold at all, because a SEPARATE, older
/// macro-kind registry some workspace test still loads through doesn't know
/// the kind exists yet: every name in `deckmaste_semantics::ron::kinds()`
/// (what `reachable_rows`/this generator consult) that's absent from
/// `deckmaste_core::ron::kinds()`.
///
/// That second, hand-maintained registry is what
/// `deckmaste_plugin/tests/corpus_identity.rs`'s `load_core_macros` builds
/// its `MacroSet` from — by its own doc, "the last hand-rolled definition
/// walk in the tree; it dies with `core-demacro`", i.e. a known-transitional
/// harness for an in-flight core-to-semantics migration, not the real loader
/// (`deckmaste_plugin::macros::macro_set`, which DOES use the semantics
/// registry). `Color` and `Supertype` used to be the two real entries here —
/// `deckmaste_core::ron::kinds()` was simply missing both registrations, and
/// `deckmaste_core::Supertype` needed a `SupportsMacros` derive (mirroring
/// `deckmaste_core::Color`, which already had one) before it could gain a
/// `.kind()` to register at all — closed by registering both there, the same
/// fix already applied on the semantics side by this feature's Task 1. `Card`
/// remains listed in the diff (a struct kind, no variant dispatch, so it
/// never actually yields a row) — left unregistered in `deckmaste_core`
/// rather than chased, since nothing depends on it. Computed rather than
/// hand-listed so a FUTURE registry drift between the two keeps getting
/// caught here rather than surfacing as a confusing load failure downstream.
fn unscaffoldable_kinds() -> Vec<String> {
    let semantics: std::collections::BTreeSet<String> = deckmaste_semantics::ron::kinds()
        .iter()
        .map(|k| k.name().to_string())
        .collect();
    let core: std::collections::BTreeSet<String> = deckmaste_core::ron::kinds()
        .iter()
        .map(|k| k.name().to_string())
        .collect();
    semantics.difference(&core).cloned().collect()
}

/// A kind that reads with `#[macro_ron(embed)]` (`Kind::embeds_untagged`,
/// set automatically by the derive whenever a variant carries that marker —
/// `ColorOrColorless`, `SimpleManaSymbol`, `ManaSymbol`, `ManaSpec`,
/// `ManaProduction`, `StatValue`, verified against `deckmaste_semantics`'s
/// source), paired with the SMALL closed set of variant names that kind
/// declares directly. Every OTHER name `reachable_rows` reports at that kind
/// is there only because the flattened dispatch set (`Kind::variants`)
/// transitively includes whatever the kind embeds — `Green` shows up at
/// `ManaSymbol` because `ManaSymbol` embeds `SimpleManaSymbol` embeds
/// `ColorOrColorless` embeds `Color`, not because `ManaSymbol` itself has a
/// `Green` variant.
///
/// A bare self-referencing identity-macro body (`body: Green`) is safe ONLY
/// at a kind where the name is one of these direct declarations: reread
/// there resolves as a genuine native variant and stops. At an embedding
/// kind, `deckmaste_plugin`'s embed-untagged fallthrough treats "a macro of
/// this name is registered at this kind" as reason enough to skip the
/// embedded type's own lookup (`macro_ron::expand`'s embed-candidacy check),
/// so the macro's own bare-name body re-enters the SAME kind, finds the SAME
/// macro again, and never bottoms out — confirmed by generating a merged
/// `Green`/`AnyColor`/`Generic` scaffold and hitting `` `Green` is more than
/// 64 macro expansions deep`` from `cargo xtask validate` on real canon
/// tokens/basics. `macro_ron::Kind` exposes no public query for "does this
/// kind embed", so this table is hand-verified against the source rather
/// than derived.
const EMBED_HOSTS: &[(&str, &[&str])] = &[
    ("ColorOrColorless", &["Colorless"]),
    ("SimpleManaSymbol", &["Generic"]),
    ("ManaSymbol", &["Variable", "Snow", "Hybrid", "Phyrexian"]),
    (
        "ManaSpec",
        &[
            "AnyColor",
            "OneOf",
            "OneOfRuns",
            "AmongColorsOf",
            "ProducedByEvent",
        ],
    ),
    ("ManaProduction", &["WithRiders"]),
    ("StatValue", &["DefinedByAbility", "Variable", "Number"]),
];

/// Whether `(kind, variant)` is safe for a bare self-referencing identity
/// scaffold: either `kind` isn't one of the [`EMBED_HOSTS`] at all, or
/// `variant` is one of that kind's own directly-declared names.
fn embed_safe(kind: &str, variant: &str) -> bool {
    match EMBED_HOSTS.iter().find(|(k, _)| *k == kind) {
        None => true,
        Some((_, own)) => own.contains(&variant),
    }
}

/// Whether `signature` has a shape this generator cannot render into a
/// scaffold without silently breaking the LONG-FORM spelling — see
/// [`needs_identity_macro_rows`]'s doc for the full explanation. Computed
/// structurally (arity/defaultedness), not by name, so a newly-added variant
/// with this shape is caught automatically rather than needing a name added
/// to a list by hand.
fn blocked_on_elision(signature: &VariantSignature) -> bool {
    match signature {
        VariantSignature::Unit => false,
        VariantSignature::Named(params) => params
            .iter()
            .any(|p| matches!(p.default, ParamDefault::Implicit)),
        VariantSignature::Positional(params) => {
            params
                .iter()
                .rev()
                .take_while(|p| !matches!(p, ParamDefault::Required))
                .count()
                > 0
        }
    }
}

/// Every reachable row an identity macro must cover and this generator can
/// actually scaffold safely (see [`UNSCAFFOLDABLE`], [`unscaffoldable_kinds`],
/// [`EMBED_HOSTS`], [`blocked_on_elision`]).
///
/// [`blocked_on_elision`] rows are the one exclusion category that is NOT
/// "no coverage needed here" — these variants genuinely need an identity
/// macro, canon genuinely spells some of them long-form, and this generator
/// genuinely cannot render a scaffold that keeps both the short AND long
/// spelling round-tripping. See the module doc's "BLOCKED: optional-param
/// elision" section for why (the macro def grammar has no way to say
/// "forward this argument if given, omit the field entirely if not" — the
/// existing `Default(Type, expr)` sugar FILLS an omitted arg with a known
/// expression, which is a different thing, and isn't even available for
/// positional params or for named params whose default has no captured
/// expression, which is every named param in practice). Excluding these
/// rather than shipping a scaffold that drops the field is the load-bearing
/// choice here: the wrong scaffold parses today (native reads shadow it
/// until Task 6's restriction flips) and only breaks once restricted, at
/// which point every long-form card using it fails to load — excluding
/// instead makes the gap visible NOW, as a missing macro, rather than
/// LATER, as every long-form card using that name failing to load at once.
fn needs_identity_macro_rows() -> Vec<(String, String)> {
    let unscaffoldable_kinds = unscaffoldable_kinds();
    let kind_set = deckmaste_semantics::ron::kinds();
    reachable_rows()
        .into_iter()
        .filter(|(kind, variant)| classify(kind, variant) == Coverage::NeedsIdentityMacro)
        .filter(|(kind, _)| !unscaffoldable_kinds.contains(kind))
        .filter(|(kind, variant)| {
            !UNSCAFFOLDABLE
                .iter()
                .any(|(k, v)| *k == kind && *v == variant)
        })
        .filter(|(kind, variant)| embed_safe(kind, variant))
        .filter(|(kind, variant)| !blocked_on_elision(&signature_of(&kind_set, kind, variant)))
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
    Ok(())
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
/// # Panics
/// If `signature` is [`blocked_on_elision`] — that shape can't be rendered
/// without either silently dropping a real constructor default (the bug
/// this guard exists to catch) or the `macro_ron` elision feature this
/// generator doesn't have. Every row `render` is actually called with
/// already passed through [`needs_identity_macro_rows`]'s filter, which
/// excludes exactly this shape, so this should never fire — the panic is
/// defense in depth against a future call site bypassing that filter, not
/// an expected path.
fn render(variant: &str, kinds: &[String], signature: &VariantSignature) -> String {
    assert!(
        !blocked_on_elision(signature),
        "{variant} has a droppable constructor default and no way to elide it \
         (BLOCKED on macro_ron optional-param support); this row should have \
         been excluded before reaching render"
    );
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

/// A positional signature's scaffold `params:` list and the matching
/// `Param(0), Param(1), …` forwarding list.
///
/// Positional macro params can't carry a default at all — `Default(Type,
/// expr)` is named-signatures-only (the macro loader rejects it on a
/// positional list: "defaults are named-only"). `render`'s guard means
/// every `params` this actually runs on has no TRAILING defaulted run (that
/// shape is excluded before reaching here — see [`blocked_on_elision`]). A
/// defaulted param that is NOT trailing (only reachable through an
/// embed-marked tuple variant, per the derive's own contiguous-suffix rule
/// for ordinary tuple variants — never observed in the current registry) is
/// kept as a plain required `Any` param instead of dropped, since dropping
/// it would shift later values into the wrong field — a conservative
/// fallback for a shape this generator has no way to render faithfully
/// either way, just a differently-shaped problem than the trailing case.
fn positional_scaffold(params: &[ParamDefault]) -> (String, String) {
    let trailing_defaulted = params
        .iter()
        .rev()
        .take_while(|p| !matches!(p, ParamDefault::Required))
        .count();
    let kept = &params[..params.len() - trailing_defaulted];

    let types = vec!["Any".to_string(); kept.len()];
    let args: Vec<String> = (0..kept.len()).map(|i| format!("Param({i})")).collect();
    (format!("[{}]", types.join(", ")), args.join(", "))
}

/// A named signature's scaffold `params:` map and the matching
/// `name: Param(name), …` forwarding list. `render`'s guard means every
/// `params` this actually runs on has no `Implicit` entries (that shape is
/// excluded before reaching here — see [`blocked_on_elision`]), so every
/// param is `Required`; the two default flavors are unreachable by
/// construction (`Implicit` would be dropped, silently breaking the long
/// form — exactly the bug that guard exists to prevent; `Expr` never
/// occurs for a named/struct-variant signature at all — the derive rejects
/// `#[macro_ron(default = ...)]` on struct-variant fields at compile time,
/// per its own doc — and its captured text is Rust source for the derive's
/// codegen besides, not valid RON, so splicing it would be wrong even if it
/// somehow appeared).
fn named_scaffold(params: &[NamedParam]) -> (String, String) {
    let mut entries = Vec::new();
    let mut args = Vec::new();
    for param in params {
        let name = param.name;
        match param.default {
            ParamDefault::Required => entries.push(format!("\"{name}\": Any")),
            ParamDefault::Implicit | ParamDefault::Expr(_) => {
                unreachable!(
                    "named param `{name}` has a default; `render`'s blocked_on_elision \
                     guard should have excluded this signature"
                )
            }
        }
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

#[cfg(test)]
mod coverage_gap {
    /// Every `NeedsIdentityMacro` row this generator declines to scaffold is
    /// accounted for by one of the four documented exclusions
    /// ([`super::UNSCAFFOLDABLE`], [`super::unscaffoldable_kinds`],
    /// [`super::embed_safe`], [`super::blocked_on_elision`]) — never a
    /// silent fifth reason. The gap itself (currently real:
    /// `deckmaste_plugin`'s `keyword_shape` closed vocabulary for
    /// `Composite`, the embed-untagged recursion hazard for names only
    /// reachable at an embedding kind transitively, and the `macro_ron`
    /// optional-param elision gap) is reported upstream, not hidden here.
    #[test]
    fn every_excluded_row_is_an_explained_exclusion() {
        use deckmaste_semantics::authoring::Coverage;
        use deckmaste_semantics::authoring::classify;
        use deckmaste_semantics::authoring::reachable_rows;

        let kind_set = deckmaste_semantics::ron::kinds();
        let all_needs: Vec<(String, String)> = reachable_rows()
            .into_iter()
            .filter(|(k, v)| classify(k, v) == Coverage::NeedsIdentityMacro)
            .collect();
        let scaffoldable = super::needs_identity_macro_rows();
        let unscaffoldable_kinds = super::unscaffoldable_kinds();
        for row @ (kind, variant) in &all_needs {
            if scaffoldable.contains(row) {
                continue;
            }
            let listed = super::UNSCAFFOLDABLE
                .iter()
                .any(|(k, v)| *k == kind && *v == variant)
                || unscaffoldable_kinds.contains(kind);
            let signature = super::signature_of(&kind_set, kind, variant);
            assert!(
                listed
                    || !super::embed_safe(kind, variant)
                    || super::blocked_on_elision(&signature),
                "{kind}::{variant} was excluded without an explanation"
            );
        }
    }
}

#[cfg(test)]
mod elision_gap {
    /// No row `needs_identity_macro_rows` hands to the writer has a
    /// droppable constructor default — `blocked_on_elision` really does
    /// exclude every such row on the real registry, not just in a synthetic
    /// example. This is the standing regression test for Critical Finding 1
    /// (a scaffold that silently dropped an optional param, breaking every
    /// long-form corpus spelling once restriction flips): if a future
    /// registry change reintroduces one of these shapes without a
    /// corresponding fix, this fails loudly instead of shipping a wrong
    /// scaffold again.
    #[test]
    fn no_scaffoldable_row_has_a_droppable_default() {
        use deckmaste_semantics::authoring::Coverage;
        use deckmaste_semantics::authoring::classify;
        use deckmaste_semantics::authoring::reachable_rows;

        let kind_set = deckmaste_semantics::ron::kinds();
        let all_needs: Vec<(String, String)> = reachable_rows()
            .into_iter()
            .filter(|(k, v)| classify(k, v) == Coverage::NeedsIdentityMacro)
            .collect();
        let blocked_count = all_needs
            .iter()
            .filter(|(kind, variant)| {
                super::blocked_on_elision(&super::signature_of(&kind_set, kind, variant))
            })
            .count();
        assert!(
            blocked_count > 0,
            "expected the exclusion to fire on today's registry"
        );

        let scaffoldable = super::needs_identity_macro_rows();
        let still_blocked: Vec<_> = scaffoldable
            .iter()
            .filter(|(kind, variant)| {
                super::blocked_on_elision(&super::signature_of(&kind_set, kind, variant))
            })
            .collect();
        assert!(still_blocked.is_empty(), "{still_blocked:?}");
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
/// kind, and — because a short/long spelling distinction only exists for a
/// defaulted param, and every DEFAULTED shape is now excluded
/// (`blocked_on_elision`) rather than scaffolded — that an excluded row's
/// long-form spelling fails LOUDLY under restriction (a clear "not author
/// vocabulary" error) instead of loading wrong or silently.
#[cfg(test)]
mod restricted_read {
    use deckmaste_semantics::Action;
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

    /// A row `blocked_on_elision` excludes — `EventFilter::Cast`, both
    /// `who`/`what` `Implicit` — has NO macro of that name anywhere in the
    /// real, fully-loaded scaffold set, by design (Critical Finding 1: no
    /// scaffold exists that could keep both the short and long spelling
    /// round-tripping). Under restriction its long-form spelling must fail
    /// LOUDLY, not silently succeed with the wrong value and not panic
    /// internally — the visible difference between "excluded and reported"
    /// and "wrong but shipped".
    #[test]
    fn an_elision_blocked_rows_long_form_fails_loudly_under_restriction_with_no_macro() {
        let macros = real_scaffolds();
        let err = macros
            .read_str_restricted::<deckmaste_semantics::EventFilter>("Cast(who: You)")
            .unwrap_err();
        let message = err.to_string();
        assert!(message.contains("not author vocabulary"), "{message}");
    }
}
