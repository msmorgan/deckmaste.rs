//! Spec §4: ONE restricted read API. A typed card or token may be read only
//! through `Plugin::card`/`token`/`card_from_str`/`token_from_str`, because
//! that is where the semantic term is retained and where `lower` is called —
//! the single point at which provenance will later be erased.
//!
//! `Plugin::rendering_card`/`rendering_card_from_str` is the one further entry
//! that reads a typed card. It is not a second read path: it runs the
//! restricted read first and propagates its error, so it can only ever return
//! a value the API above accepted, with identity-macro provenance dropped.
//! What it IS is a value the engine never sees, valid only while every
//! variant-named macro mirrors its variant faithfully — so it belongs to the
//! legacy renderer and nothing else. [`rendering_view_calls_are_confined`]
//! keeps it there, as a SEPARATE scan: such a call names no `read_str` and no
//! `Card`, so the matcher below is structurally blind to it.
//!
//! MECHANICAL, like `no_dead_grammar.rs`: it PARSES the workspace source with
//! `syn` rather than carrying a hand-maintained list that can silently rot.
//!
//! Parsing, not grepping, because the reads this must catch are multi-line —
//! `let card: Card = plugin\n    .macros\n    .read_str(&source)` puts the type
//! and the call on different lines, so no line-based pattern sees both. And the
//! restriction is on the TYPE read, not on touching `Plugin::macros`: many
//! legitimate sites read `Ability`, `OneShotEffect`, `Predicate`, `TypeDef`,
//! `EventFilter` and friends through the same macro set, and spec §4 leaves
//! them alone ("restriction is opt-in at the entry, never a global default").
//!
//! # What is matched
//!
//! Every form below is pinned by `the_matcher_catches_every_form_it_claims_to`,
//! which runs this visitor over inline fixtures — the gate reporting green must
//! mean "found nothing", never "matches nothing".
//!
//! - a turbofish, `read_str::<Card>(…)`;
//! - a `let` annotation, on one line or several, including the `let Ok(x) = …
//!   else` form;
//! - either of those wrapped — `Vec<Card>`, `Option<Card>`, `&Card`, `(Card,
//!   Card)`, and nested combinations;
//! - either of those inside a macro invocation (`assert_eq!(m.read_str::<Card>(
//!   s).unwrap(), …)`), which `syn` keeps as an opaque token stream that no
//!   visitor descends into by default. This idiom is everywhere in test code,
//!   and test code is where much of the routed inventory lived;
//! - `use …::Card as Alias`, which would otherwise defeat the last-segment
//!   match in every form above.
//!
//! # What is not
//!
//! Reachable in principle, not implemented, because nothing in the tree needs
//! it and each would cost real machinery:
//!
//! - a read typed by its *binding* rather than an annotation — `let x =
//!   m.read_str(s)?;` where `x` is pinned by the enclosing function's return
//!   type or by a use further down. Return position is parseable; the general
//!   case is not, so this would be a partial measure either way.
//! - a `type MyCard = Card;` alias. Deliberate: *following* an alias to its use
//!   sites is name resolution, not parsing, so flagging the declaration alone
//!   would be a partial measure. (Distinguishing it from the associated type
//!   every `Lower` impl writes — `type Target = deckmaste_card::Card;` — is not
//!   the obstacle: `syn` visits standalone aliases through `visit_item_type`
//!   and associated ones through `visit_impl_item_type`.)
//! - a read at a generic parameter, `let v: A = macros.read_str(s)?` with `A`
//!   chosen by the caller — what `tests/corpus_identity.rs`'s core-reader
//!   oracle does. Knowing `A` is `Card` here is monomorphisation, not parsing.
//!   That file is deliberately NOT in [`ALLOWED`]: every restricted read in it
//!   arrives this way, so it has nothing for an allowance to permit — and
//!   listing it would skip the file wholesale, hiding a literal
//!   `read_str::<Card>(…)` added there later. It stays scannable.
//! - a read spelled with a different ENTRY POINT —
//!   `ron::de::from_str::<Card>(…)`, or any other deserializer.
//!   `Card`/`CardFace`/`Token` all derive plain `Deserialize`, and this matcher
//!   keys on the literal method name `read_str`, so a bypass that never names
//!   it is invisible. The most reachable accidental hole here; widening the
//!   matched name set is the fix if one ever appears.
//! - a macro body whose tokens parse as neither expressions nor statements
//!   (`matches!(x, Ok(_))` — `Ok(_)` is a pattern), and macros that assemble
//!   the call out of fragments rather than spelling it.

use std::path::Path;
use std::path::PathBuf;

use syn::parse::Parser as _;

/// Files permitted to name a typed card read, with the reason each stands on.
const ALLOWED: &[&str] = &[
    // The API itself.
    "crates/deckmaste_plugin/src/plugin.rs",
    // macro_ron's own fixture types, unrelated to the grammar containers.
    "crates/macro_ron/src/tests.rs",
    // NOT here: `tests/corpus_identity.rs`, the migration oracle that reads the
    // corpus through both grammars. Its restricted reads all arrive at a
    // generic parameter, so the matcher never sees them (see "What is not") and
    // an entry would buy nothing while skipping the whole file — a literal
    // `read_str::<Card>` added there later would go unflagged.
];

/// The container types whose reads are restricted. Matched on the LAST path
/// segment, so `deckmaste_card::Card`, `deckmaste_semantics::Card`, and a bare
/// `Card` all count.
const RESTRICTED: &[&str] = &["Card", "CardFace", "Token"];

#[test]
fn typed_card_reads_go_through_the_restricted_api() {
    let root = Path::new(env!("CARGO_MANIFEST_DIR")).join("../..");
    // An allowance naming a file that no longer exists is unevaluatable rot:
    // fail loudly so whoever deleted the file drops its entry too. This also
    // catches a mis-rooted walk — if `root` were wrong, none of these resolve.
    for allowed in ALLOWED {
        assert!(
            root.join(allowed).is_file(),
            "stale ALLOWED entry — no such file: {allowed}"
        );
    }
    let sources = rust_sources(&root);
    // Vacuity guard: a walk that found nothing would report green forever.
    assert!(
        !sources.is_empty(),
        "no Rust sources found under {} — the walk is broken, not the tree clean",
        root.display()
    );
    let mut offenders = Vec::new();
    for path in sources {
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
        // The v2 grammar's `Card` is a different type with no restricted
        // read API; a file that reads through `deckmaste_semantics_v2` is
        // not this gate's subject.
        if source.contains("deckmaste_semantics_v2::") {
            continue;
        }
        let Ok(file) = syn::parse_file(&source) else {
            continue;
        };
        for hit in hits_in(&file) {
            offenders.push(format!("{rel}: {hit}"));
        }
    }
    assert!(
        offenders.is_empty(),
        "typed card/token reads outside the restricted API \
         (`Plugin::card`/`token`/`card_from_str`/`token_from_str`, plus the \
          legacy renderer's `rendering_card`/`rendering_card_from_str`):\n{}",
        offenders.join("\n")
    );
}

/// The rendering-view entries as they appear at a CALL — receiver, dot, name,
/// open paren. Matching the bare names would also match every doc comment and
/// error message that mentions them, including this file's and the one in
/// `tests/restricted_containers.rs` (which found this the first time it ran).
/// A rustdoc link spells them `Plugin::rendering_card`, with `::` and no
/// parenthesis, so the leading dot separates a call from prose about one.
const RENDERING_VIEW: &[&str] = &[".rendering_card(", ".rendering_card_from_str("];
const RENDERING_VIEW_CALLERS: &str = "crates/deckmaste_legacy_render/";

/// `Plugin::rendering_card`(`_from_str`) returns a card the ENGINE never sees:
/// the same source read free, so identity-macro provenance is absent. That is
/// only equivalent to the loaded card while every variant-named macro mirrors
/// its variant faithfully. Let one leak into a non-rendering consumer and it
/// would compare, lower, or grade against a value the loader does not produce
/// — the fidelity gate would report green on a card that does not exist.
///
/// A textual scan, not a syn walk: the call is an ordinary method call naming
/// no restricted type, so there is no shape to match on — the NAME is the whole
/// signal. It keys on the CALL spelling (see [`RENDERING_VIEW`]) rather than
/// the bare name, so prose about the entry does not read as a use of it.
#[test]
fn rendering_view_calls_are_confined() {
    let root = Path::new(env!("CARGO_MANIFEST_DIR")).join("../..");
    let sources = rust_sources(&root);
    assert!(
        !sources.is_empty(),
        "no Rust sources found under {} — the walk is broken, not the tree clean",
        root.display()
    );
    let mut offenders = Vec::new();
    let mut permitted = 0usize;
    for path in sources {
        let rel = path
            .strip_prefix(&root)
            .unwrap_or(&path)
            .to_string_lossy()
            .replace('\\', "/");
        let Ok(source) = std::fs::read_to_string(&path) else {
            continue;
        };
        if !RENDERING_VIEW.iter().any(|call| source.contains(call)) {
            continue;
        }
        if rel.starts_with(RENDERING_VIEW_CALLERS) {
            permitted += 1;
            continue;
        }
        // The same allowances as the read gate — chiefly the definition site,
        // where `rendering_card` delegates to `rendering_card_from_str`. Shared
        // rather than restated so both lists get the staleness check above.
        // This file names the call spellings as data.
        if ALLOWED.iter().any(|a| rel.ends_with(a)) || rel.ends_with("read_api_gate.rs") {
            continue;
        }
        offenders.push(rel);
    }
    // Vacuity guard: if the renderer stopped calling it, this gate would pass
    // by finding nothing rather than by holding a line.
    assert!(
        permitted > 0,
        "no {RENDERING_VIEW_CALLERS} file mentions the rendering view — either it \
         is unused (delete it) or the walk is broken"
    );
    assert!(
        offenders.is_empty(),
        "`Plugin::rendering_card`/`rendering_card_from_str` is for \
         {RENDERING_VIEW_CALLERS} only — it returns a card the engine never \
         loads:\n{}",
        offenders.join("\n")
    );
}

/// The positive control this gate would be worthless without: every form the
/// module doc claims to catch, caught here on a fixture. One form per row, so
/// a regression names what it broke rather than just going quiet.
#[test]
fn the_matcher_catches_every_form_it_claims_to() {
    let cases: &[(&str, &str, usize)] = &[
        (
            "turbofish",
            "fn f() { let a = m.read_str::<Card>(s).unwrap(); }",
            1,
        ),
        (
            "turbofish, qualified",
            "fn f() { let a = m.read_str::<deckmaste_card::Card>(s).unwrap(); }",
            1,
        ),
        (
            "turbofish, Token",
            "fn f() { let a = m.read_str::<Token>(s).unwrap(); }",
            1,
        ),
        (
            "turbofish, CardFace",
            "fn f() { let a = m.read_str::<CardFace>(s).unwrap(); }",
            1,
        ),
        (
            "annotation, one line",
            "fn f() { let b: Card = m.read_str(s).unwrap(); }",
            1,
        ),
        (
            "annotation, multi-line",
            r"fn f() {
    let b: Card = plugin
        .macros
        .read_str(&source)
        .unwrap();
}",
            1,
        ),
        (
            "let-else with turbofish",
            "fn f() { let Ok(c) = m.read_str::<Card>(s) else { return }; }",
            1,
        ),
        (
            "wrapped in Vec",
            "fn f() { let e: Vec<Card> = m.read_str(s).unwrap(); }",
            1,
        ),
        (
            "wrapped in Option",
            "fn f() { let e: Option<CardFace> = m.read_str(s).unwrap(); }",
            1,
        ),
        (
            "behind a reference",
            "fn f() { let e: &Card = m.read_str(s).unwrap(); }",
            1,
        ),
        (
            "in a tuple",
            "fn f() { let e: (Card, u8) = m.read_str(s).unwrap(); }",
            1,
        ),
        (
            "nested wrappers",
            "fn f() { let e: Vec<Option<Box<Card>>> = m.read_str(s).unwrap(); }",
            1,
        ),
        (
            "turbofish, wrapped",
            "fn f() { let a = m.read_str::<Vec<Card>>(s).unwrap(); }",
            1,
        ),
        (
            "inside a macro invocation",
            "fn f() { assert_eq!(m.read_str::<Card>(s).unwrap(), want); }",
            1,
        ),
        (
            "annotation inside a macro body",
            "fn f() { quoted! { let b: Card = m.read_str(s).unwrap(); } }",
            1,
        ),
        (
            "inside a nested macro invocation",
            "fn f() { assert!(matches(m.read_str::<Card>(s), want), \"{}\", x); }",
            1,
        ),
        ("aliased import", "use deckmaste_card::Card as CoreCard;", 1),
        (
            "aliased import, braced",
            "use deckmaste_card::{Token as Tok};",
            1,
        ),
        // Negative controls — spec §4 leaves these alone, and a matcher that
        // flagged them would be worse than none.
        (
            "unrestricted turbofish",
            "fn f() { let a = m.read_str::<Ability>(s).unwrap(); }",
            0,
        ),
        (
            "unrestricted annotation",
            "fn f() { let b: EventFilter = m.read_str(s).unwrap(); }",
            0,
        ),
        (
            "unrestricted, wrapped",
            "fn f() { let b: Vec<Ability> = m.read_str(s).unwrap(); }",
            0,
        ),
        (
            "restricted type, but not a read",
            "fn f() { let b: Card = plugin.card_from_str(s).unwrap().core; }",
            0,
        ),
        (
            "restricted type in a macro, but not a read",
            "fn f() { assert_eq!(plugin.card_from_str(s).unwrap().core, want); }",
            0,
        ),
        (
            "unaliased import",
            "use deckmaste_card::Card;\nuse deckmaste_core::Token;",
            0,
        ),
        (
            "associated type is not an alias",
            "impl Lower for X { type Target = deckmaste_card::Card; }",
            0,
        ),
    ];
    for (label, source, expected) in cases {
        let file = syn::parse_file(source)
            .unwrap_or_else(|e| panic!("{label}: the fixture itself does not parse: {e}"));
        let hits = hits_in(&file);
        assert_eq!(
            hits.len(),
            *expected,
            "{label}: expected {expected} hit(s), got {hits:?}\n--- fixture ---\n{source}"
        );
    }
}

/// Every restricted read in one parsed file.
fn hits_in(file: &syn::File) -> Vec<String> {
    let mut visitor = ReadStrVisitor { hits: Vec::new() };
    syn::visit::visit_file(&mut visitor, file);
    visitor.hits
}

struct ReadStrVisitor {
    hits: Vec<String>,
}

/// The name of a [`RESTRICTED`] container reachable in `ty`, matched on a
/// path's LAST segment and recursed through the wrappers a read can hide
/// behind: generic arguments (`Vec<Card>`), references, tuples, slices, arrays,
/// and parens/groups. Without the recursion, `Vec<Card>` reads as `Vec` and
/// passes.
fn restricted_name(ty: &syn::Type) -> Option<String> {
    match ty {
        syn::Type::Path(p) => {
            let last = p.path.segments.last()?;
            let name = last.ident.to_string();
            if RESTRICTED.contains(&name.as_str()) {
                return Some(name);
            }
            // Not restricted itself — but it may be wrapping one.
            let syn::PathArguments::AngleBracketed(args) = &last.arguments else {
                return None;
            };
            args.args.iter().find_map(|a| match a {
                syn::GenericArgument::Type(inner) => restricted_name(inner),
                _ => None,
            })
        }
        syn::Type::Reference(r) => restricted_name(&r.elem),
        syn::Type::Paren(p) => restricted_name(&p.elem),
        syn::Type::Group(g) => restricted_name(&g.elem),
        syn::Type::Slice(s) => restricted_name(&s.elem),
        syn::Type::Array(a) => restricted_name(&a.elem),
        syn::Type::Tuple(t) => t.elems.iter().find_map(restricted_name),
        _ => None,
    }
}

/// The statements a macro invocation's token stream parses into, best effort.
///
/// `syn` stores `Macro::tokens` as an opaque `TokenStream` and no visitor
/// descends it, so without this every `assert_eq!(m.read_str::<Card>(s), …)` is
/// invisible. Tried as a comma-separated expression list first (the shape of
/// almost every macro *call*), then as a statement sequence (the shape of a
/// `quote!`-style body). Neither parsing is an error — plenty of macro bodies
/// are neither, and a body this cannot read is not evidence of a bypass.
fn macro_body_stmts(mac: &syn::Macro) -> Vec<syn::Stmt> {
    let exprs = syn::punctuated::Punctuated::<syn::Expr, syn::Token![,]>::parse_terminated;
    if let Ok(parsed) = exprs.parse2(mac.tokens.clone()) {
        return parsed
            .into_iter()
            .map(|expr| syn::Stmt::Expr(expr, None))
            .collect();
    }
    syn::Block::parse_within
        .parse2(mac.tokens.clone())
        .unwrap_or_default()
}

/// True when this expression tree contains a `.read_str(…)` method call —
/// including inside a macro invocation it wraps.
fn calls_read_str(expr: &syn::Expr) -> bool {
    struct Finder(bool);
    impl<'ast> syn::visit::Visit<'ast> for Finder {
        fn visit_expr_method_call(&mut self, node: &'ast syn::ExprMethodCall) {
            if node.method == "read_str" {
                self.0 = true;
            }
            syn::visit::visit_expr_method_call(self, node);
        }

        fn visit_macro(&mut self, node: &'ast syn::Macro) {
            for stmt in macro_body_stmts(node) {
                let mut inner = Finder(false);
                syn::visit::visit_stmt(&mut inner, &stmt);
                self.0 |= inner.0;
            }
            syn::visit::visit_macro(self, node);
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
    /// Covers `let Ok(x) = …read_str::<Card>(…) else` too, via form 1.
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

    /// Form 3 — either of the above, inside a macro invocation.
    fn visit_macro(&mut self, node: &'ast syn::Macro) {
        for stmt in macro_body_stmts(node) {
            let mut inner = ReadStrVisitor { hits: Vec::new() };
            syn::visit::visit_stmt(&mut inner, &stmt);
            self.hits.extend(
                inner
                    .hits
                    .into_iter()
                    .map(|hit| format!("{hit} (in a macro body)")),
            );
        }
        syn::visit::visit_macro(self, node);
    }

    /// Form 4 — an aliased import, which would defeat every last-segment match
    /// above. There is no legitimate reason to rename a grammar container on
    /// import, so the rename itself is the finding.
    fn visit_use_rename(&mut self, node: &'ast syn::UseRename) {
        let original = node.ident.to_string();
        if RESTRICTED.contains(&original.as_str()) {
            self.hits.push(format!(
                "`use …{original} as {}` — an alias hides the read from this gate",
                node.rename
            ));
        }
        syn::visit::visit_use_rename(self, node);
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
