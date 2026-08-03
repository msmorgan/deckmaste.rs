//! Spec §4: ONE restricted read API. A typed card or token may be read only
//! through `Plugin::card`/`token`/`card_from_str`/`token_from_str`, because
//! that is where the authored term is retained and where `lower` is called —
//! the single point at which provenance will later be erased.
//!
//! MECHANICAL, like `no_dead_grammar.rs`: it PARSES the workspace source with
//! `syn` rather than carrying a hand-maintained list that can silently rot.
//!
//! Parsing, not grepping, because the reads this must catch are multi-line —
//! `let card: Card = plugin\n    .macros\n    .read_str(&source)` puts the type
//! and the call on different lines, so no line-based pattern sees both. And the
//! restriction is on the TYPE read, not on touching `Plugin::macros`: ~40
//! legitimate sites read `Ability`, `OneShotEffect`, `Predicate`, `TypeDef`,
//! `EventFilter` and friends through the same macro set, and spec §4 leaves
//! them alone ("restriction is opt-in at the entry, never a global default").
//!
//! Two forms are matched, both of which the routed inventory actually used: a
//! turbofish (`read_str::<Card>(…)`) and a `let` annotation. A read whose type
//! comes from neither — inferred from a function's return position, or hidden
//! behind a generic parameter as `deckmaste_lowering/tests/corpus.rs` does —
//! is out of reach; closing that would mean type inference, not parsing.

use std::path::Path;
use std::path::PathBuf;

/// Files permitted to name a typed card read, with the reason each stands on.
const ALLOWED: &[&str] = &[
    // The API itself.
    "crates/deckmaste_plugin/src/plugin.rs",
    // macro_ron's own fixture types, unrelated to the grammar containers.
    "crates/macro_ron/src/tests.rs",
    // The migration oracle: reads via BOTH readers by design, and dies with
    // `core-demacro`. Belt and braces — it reads at a generic parameter today,
    // which the matcher cannot see anyway (see module doc).
    "crates/deckmaste_lowering/tests/corpus.rs",
];

/// The container types whose reads are restricted. Matched on the LAST path
/// segment, so `deckmaste_card::Card`, `deckmaste_authoring::Card`, and a bare
/// `Card` all count.
const RESTRICTED: &[&str] = &["Card", "CardFace", "Token"];

#[test]
fn typed_card_reads_go_through_the_restricted_api() {
    let root = Path::new(env!("CARGO_MANIFEST_DIR")).join("../..");
    // An allowance naming a file that no longer exists is unevaluatable rot:
    // fail loudly so whoever deleted the file drops its entry too.
    for allowed in ALLOWED {
        assert!(
            root.join(allowed).is_file(),
            "stale ALLOWED entry — no such file: {allowed}"
        );
    }
    let mut offenders = Vec::new();
    for path in rust_sources(&root) {
        let rel = path
            .strip_prefix(&root)
            .unwrap_or(&path)
            .to_string_lossy()
            .replace('\\', "/");
        if ALLOWED.iter().any(|a| rel.ends_with(a)) {
            continue;
        }
        let Ok(source) = std::fs::read_to_string(&path) else {
            continue;
        };
        let Ok(file) = syn::parse_file(&source) else {
            continue;
        };
        let mut visitor = ReadStrVisitor { hits: Vec::new() };
        syn::visit::visit_file(&mut visitor, &file);
        for hit in visitor.hits {
            offenders.push(format!("{rel}: {hit}"));
        }
    }
    assert!(
        offenders.is_empty(),
        "typed card/token reads outside the restricted API \
         (`Plugin::card`/`token`/`card_from_str`/`token_from_str`):\n{}",
        offenders.join("\n")
    );
}

struct ReadStrVisitor {
    hits: Vec<String>,
}

/// The last path segment of a type, when it is one of [`RESTRICTED`].
fn restricted_name(ty: &syn::Type) -> Option<String> {
    let syn::Type::Path(p) = ty else { return None };
    let last = p.path.segments.last()?.ident.to_string();
    RESTRICTED.contains(&last.as_str()).then_some(last)
}

/// True when this expression tree contains a `.read_str(…)` method call.
fn calls_read_str(expr: &syn::Expr) -> bool {
    struct Finder(bool);
    impl<'ast> syn::visit::Visit<'ast> for Finder {
        fn visit_expr_method_call(&mut self, node: &'ast syn::ExprMethodCall) {
            if node.method == "read_str" {
                self.0 = true;
            }
            syn::visit::visit_expr_method_call(self, node);
        }
    }
    let mut finder = Finder(false);
    syn::visit::visit_expr(&mut finder, expr);
    finder.0
}

impl<'ast> syn::visit::Visit<'ast> for ReadStrVisitor {
    /// Form 1 — the turbofish: `macros.read_str::<Card>(&source)`.
    fn visit_expr_method_call(&mut self, node: &'ast syn::ExprMethodCall) {
        if node.method == "read_str"
            && let Some(args) = &node.turbofish
            && let Some(name) = args.args.iter().find_map(|a| match a {
                syn::GenericArgument::Type(ty) => restricted_name(ty),
                _ => None,
            })
        {
            self.hits.push(format!("`read_str::<{name}>(…)`"));
        }
        syn::visit::visit_expr_method_call(self, node);
    }

    /// Form 2 — the annotation, which spans lines:
    /// `let card: Card = plugin\n    .macros\n    .read_str(&source)?;`
    fn visit_local(&mut self, node: &'ast syn::Local) {
        if let syn::Pat::Type(pat) = &node.pat
            && let Some(name) = restricted_name(&pat.ty)
            && let Some(init) = &node.init
            && calls_read_str(&init.expr)
        {
            self.hits
                .push(format!("annotated ``let … : {name} = …read_str(…)``"));
        }
        syn::visit::visit_local(self, node);
    }
}

/// Every `.rs` under `crates/` and the root binary's `src/`, sorted. Skips
/// `target/` — build artifacts are not source and would make the gate depend
/// on whether you had built.
fn rust_sources(root: &Path) -> Vec<PathBuf> {
    fn walk(dir: &Path, out: &mut Vec<PathBuf>) {
        let Ok(entries) = std::fs::read_dir(dir) else {
            return;
        };
        for entry in entries.flatten() {
            let path = entry.path();
            if path.is_dir() {
                if path.file_name().is_some_and(|n| n == "target") {
                    continue;
                }
                walk(&path, out);
            } else if path.extension().is_some_and(|e| e == "rs") {
                out.push(path);
            }
        }
    }
    let mut out = Vec::new();
    walk(&root.join("crates"), &mut out);
    walk(&root.join("src"), &mut out);
    out.sort();
    out
}
