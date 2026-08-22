use std::collections::BTreeSet;
use std::fs;
use std::io::ErrorKind;
use std::path::Path;
use std::path::PathBuf;
use std::str::FromStr as _;

use proc_macro2::Delimiter;
use proc_macro2::TokenTree;
use sha2::Digest as _;
use syn::visit::Visit as _;

const INVENTORY_ROOTS: &[&str] = &[
    "crates/deckmaste_construction_core/src",
    "crates/deckmaste_construction/src",
    "crates/deckmaste_english_v2/src",
    "crates/xtask/src/english_v2",
];

const INVENTORY_FILES: &[&str] = &[
    "crates/deckmaste_construction/src/lib.rs",
    "crates/deckmaste_construction_core/src/emit/ast.rs",
    "crates/deckmaste_construction_core/src/emit/build.rs",
    "crates/deckmaste_construction_core/src/emit/mod.rs",
    "crates/deckmaste_construction_core/src/emit/render.rs",
    "crates/deckmaste_construction_core/src/emit/rules.rs",
    "crates/deckmaste_construction_core/src/emit/runtime.rs",
    "crates/deckmaste_construction_core/src/emit/scanner.rs",
    "crates/deckmaste_construction_core/src/emit/terminal.rs",
    "crates/deckmaste_construction_core/src/emit/visit.rs",
    "crates/deckmaste_construction_core/src/feature.rs",
    "crates/deckmaste_construction_core/src/format.rs",
    "crates/deckmaste_construction_core/src/identifier.rs",
    "crates/deckmaste_construction_core/src/lib.rs",
    "crates/deckmaste_construction_core/src/model.rs",
    "crates/deckmaste_construction_core/src/morphology.rs",
    "crates/deckmaste_construction_core/src/parse.rs",
    "crates/deckmaste_construction_core/src/plan.rs",
    "crates/deckmaste_construction_core/src/report.rs",
    "crates/deckmaste_construction_core/src/semantic.rs",
    "crates/deckmaste_construction_core/src/source.rs",
    "crates/deckmaste_construction_core/src/test_support.rs",
    "crates/deckmaste_construction_core/src/validate.rs",
    "crates/deckmaste_english_v2/src/ast.rs",
    "crates/deckmaste_english_v2/src/constructions.rs",
    "crates/deckmaste_english_v2/src/context.rs",
    "crates/deckmaste_english_v2/src/environment.rs",
    "crates/deckmaste_english_v2/src/lib.rs",
    "crates/deckmaste_english_v2/src/orthography.rs",
    "crates/deckmaste_english_v2/src/parser/diagnostic.rs",
    "crates/deckmaste_english_v2/src/parser/engine.rs",
    "crates/deckmaste_english_v2/src/parser/error.rs",
    "crates/deckmaste_english_v2/src/parser/homonym_pipeline.rs",
    "crates/deckmaste_english_v2/src/parser/materialize.rs",
    "crates/deckmaste_english_v2/src/parser/mod.rs",
    "crates/deckmaste_english_v2/src/parser/ownership.rs",
    "crates/deckmaste_english_v2/src/parser/scan.rs",
    "crates/deckmaste_english_v2/src/parser/selection.rs",
    "crates/deckmaste_english_v2/src/render.rs",
    "crates/deckmaste_english_v2/src/visit.rs",
    "crates/xtask/src/english_v2.rs",
    "crates/xtask/src/english_v2/ambiguity.rs",
    "crates/xtask/src/english_v2/audit.rs",
    "crates/xtask/src/english_v2/corpus.rs",
    "crates/xtask/src/english_v2/coverage.rs",
    "crates/xtask/src/english_v2/coverage_lock.rs",
    "crates/xtask/src/english_v2/diagnostic.rs",
    "crates/xtask/src/english_v2/inspect.rs",
    "crates/xtask/src/english_v2/parse.rs",
    "crates/xtask/src/english_v2/plan04_authority.rs",
    "crates/xtask/src/english_v2/probe.rs",
    "crates/xtask/src/english_v2/report.rs",
    "crates/xtask/src/english_v2/roundtrip.rs",
    "crates/xtask/src/english_v2/timing.rs",
];

const CATALOG_MANIFEST_ROOTS: &[&str] = &[
    "crates/deckmaste_construction_core",
    "crates/deckmaste_construction",
    "crates/deckmaste_english_v2",
    "crates/xtask",
];
const CATALOG_MANIFEST_FILES: &[&str] = &[
    "crates/deckmaste_construction_core/Cargo.toml",
    "crates/deckmaste_construction/Cargo.toml",
    "crates/deckmaste_english_v2/Cargo.toml",
    "crates/xtask/Cargo.toml",
];
const XTASK_CATALOG_DEPENDENCY: &str =
    "deckmaste_catalogs = { version = \"0.1.0\", path = \"../deckmaste_catalogs\" }";

const CONSTRUCTIONS_PATH: &str = "crates/deckmaste_english_v2/src/constructions.rs";
const AST_EMITTER_PATH: &str = "crates/deckmaste_construction_core/src/emit/ast.rs";
const BUILD_EMITTER_PATH: &str = "crates/deckmaste_construction_core/src/emit/build.rs";
const PARSER_PATH: &str = "crates/deckmaste_construction_core/src/parse.rs";
const ENGINE_PATH: &str = "crates/deckmaste_english_v2/src/parser/engine.rs";
const ENGLISH_PARSER_PATH: &str = "crates/deckmaste_english_v2/src/parser/mod.rs";
const ENGLISH_SCANNER_PATH: &str = "crates/deckmaste_english_v2/src/parser/scan.rs";
const RENDER_EMITTER_PATH: &str = "crates/deckmaste_construction_core/src/emit/render.rs";
const REPORT_PATH: &str = "crates/xtask/src/english_v2/report.rs";
const CATALOG_ADAPTER_PATH: &str = "crates/xtask/src/english_v2.rs";
const CATALOG_ADAPTER_FUNCTION: &str = "adapt_card_name_catalog_provider";
const RETIREMENT_DIAGNOSTIC: &str =
    "`checked` metadata was retired after Stage 4; use generated invariants";

#[derive(Debug, Default, PartialEq, Eq)]
struct InventoryDiff {
    missing: Vec<String>,
    unexpected: Vec<String>,
}

#[derive(Debug, Clone, PartialEq, Eq)]
enum LexToken {
    Ident(String),
    Punct(char),
    Open(Delimiter),
    Close(Delimiter),
    Literal,
}

#[derive(Clone, Copy)]
enum PathScope {
    All,
    Exact(&'static str),
}

#[derive(Clone, Copy)]
enum Atom {
    Ident(&'static str),
    Punct(char),
    Open(Delimiter),
    Close(Delimiter),
}

struct ForbiddenPattern {
    label: &'static str,
    paths: PathScope,
    atoms: &'static [Atom],
}

struct ExactCount {
    path: &'static str,
    label: &'static str,
    atoms: &'static [Atom],
    expected: usize,
}

struct ForbiddenOutside {
    label: &'static str,
    allowed_paths: &'static [&'static str],
    atoms: &'static [Atom],
}

const GLOBAL_FORBIDDEN: &[ForbiddenPattern] = &[
    ForbiddenPattern {
        label: "struct Checked",
        paths: PathScope::All,
        atoms: &[Atom::Ident("struct"), Atom::Ident("Checked")],
    },
    ForbiddenPattern {
        label: "struct ConstructorBinding",
        paths: PathScope::All,
        atoms: &[Atom::Ident("struct"), Atom::Ident("ConstructorBinding")],
    },
    ForbiddenPattern {
        label: "enum ConstructorArgument",
        paths: PathScope::All,
        atoms: &[Atom::Ident("enum"), Atom::Ident("ConstructorArgument")],
    },
    ForbiddenPattern {
        label: "checked_constructor",
        paths: PathScope::All,
        atoms: &[Atom::Ident("checked_constructor")],
    },
    ForbiddenPattern {
        label: "impl Triggered",
        paths: PathScope::All,
        atoms: &[Atom::Ident("impl"), Atom::Ident("Triggered")],
    },
    ForbiddenPattern {
        label: "impl SelfReferenceNp",
        paths: PathScope::All,
        atoms: &[Atom::Ident("impl"), Atom::Ident("SelfReferenceNp")],
    },
];

const BUILD_FORBIDDEN: &[ForbiddenPattern] = &[
    ForbiddenPattern {
        label: "legacy_refinement",
        paths: PathScope::Exact(BUILD_EMITTER_PATH),
        atoms: &[Atom::Ident("legacy_refinement")],
    },
    ForbiddenPattern {
        label: "validate_legacy_refinement_projection",
        paths: PathScope::Exact(BUILD_EMITTER_PATH),
        atoms: &[Atom::Ident("validate_legacy_refinement_projection")],
    },
    ForbiddenPattern {
        label: "as_role_refinement",
        paths: PathScope::Exact(BUILD_EMITTER_PATH),
        atoms: &[Atom::Ident("as_role_refinement")],
    },
    ForbiddenPattern {
        label: "RequireExprSource",
        paths: PathScope::Exact(BUILD_EMITTER_PATH),
        atoms: &[Atom::Ident("RequireExprSource")],
    },
    ForbiddenPattern {
        label: "RequireSubjectSource",
        paths: PathScope::Exact(BUILD_EMITTER_PATH),
        atoms: &[Atom::Ident("RequireSubjectSource")],
    },
    ForbiddenPattern {
        label: "PredicateAtomPlan",
        paths: PathScope::Exact(BUILD_EMITTER_PATH),
        atoms: &[Atom::Ident("PredicateAtomPlan")],
    },
    ForbiddenPattern {
        label: "PredicateMemberPlan",
        paths: PathScope::Exact(BUILD_EMITTER_PATH),
        atoms: &[Atom::Ident("PredicateMemberPlan")],
    },
    ForbiddenPattern {
        label: "PredicateSubjectPlan",
        paths: PathScope::Exact(BUILD_EMITTER_PATH),
        atoms: &[Atom::Ident("PredicateSubjectPlan")],
    },
];

const FORBIDDEN_OUTSIDE_EXCEPTIONS: &[ForbiddenOutside] = &[
    ForbiddenOutside {
        label: "constructor = outside allowed windows",
        allowed_paths: &[AST_EMITTER_PATH],
        atoms: &[Atom::Ident("constructor"), Atom::Punct('=')],
    },
    ForbiddenOutside {
        label: "checked identifier outside allowed windows",
        allowed_paths: &[PARSER_PATH, ENGINE_PATH],
        atoms: &[Atom::Ident("checked")],
    },
    ForbiddenOutside {
        label: "checked_constructor_bindings outside allowed windows",
        allowed_paths: &[REPORT_PATH],
        atoms: &[Atom::Ident("checked_constructor_bindings")],
    },
];

const TOTAL_COUNTS: &[ExactCount] = &[
    ExactCount {
        path: CONSTRUCTIONS_PATH,
        label: "direct constructions! invocation",
        atoms: &[
            Atom::Ident("constructions"),
            Atom::Punct('!'),
            Atom::Open(Delimiter::Brace),
        ],
        expected: 1,
    },
    ExactCount {
        path: AST_EMITTER_PATH,
        label: "constructor = total",
        atoms: &[Atom::Ident("constructor"), Atom::Punct('=')],
        expected: 3,
    },
    ExactCount {
        path: PARSER_PATH,
        label: "checked identifier total",
        atoms: &[Atom::Ident("checked")],
        expected: 2,
    },
    ExactCount {
        path: ENGINE_PATH,
        label: "checked identifier total",
        atoms: &[Atom::Ident("checked")],
        expected: 4,
    },
    ExactCount {
        path: REPORT_PATH,
        label: "checked_constructor_bindings identifier total",
        atoms: &[Atom::Ident("checked_constructor_bindings")],
        expected: 6,
    },
    ExactCount {
        path: ENGLISH_PARSER_PATH,
        label: "generated REQUIRED_CATALOG_PROVIDERS parser-construction authority",
        atoms: &[Atom::Ident("REQUIRED_CATALOG_PROVIDERS")],
        expected: 2,
    },
    ExactCount {
        path: ENGLISH_SCANNER_PATH,
        label: "indexed provider surface byte-limit lookup",
        atoms: &[Atom::Ident("catalog_surface_byte_limit")],
        expected: 1,
    },
    ExactCount {
        path: ENGLISH_SCANNER_PATH,
        label: "indexed provider exact-surface lookup",
        atoms: &[Atom::Ident("catalog_row_for_surface")],
        expected: 1,
    },
    ExactCount {
        path: RENDER_EMITTER_PATH,
        label: "generated renderer environment surface lookup",
        atoms: &[Atom::Ident("catalog_surface")],
        expected: 2,
    },
    ExactCount {
        path: RENDER_EMITTER_PATH,
        label: "generated renderer environment onset lookup",
        atoms: &[Atom::Ident("catalog_onset")],
        expected: 1,
    },
];

const ALLOWED_WINDOWS: &[ExactCount] = &[
    ExactCount {
        path: AST_EMITTER_PATH,
        label: "let constructor = method window",
        atoms: &[
            Atom::Ident("let"),
            Atom::Ident("constructor"),
            Atom::Punct('='),
            Atom::Ident("method"),
        ],
        expected: 3,
    },
    ExactCount {
        path: PARSER_PATH,
        label: "custom_keyword!(checked) window",
        atoms: &[
            Atom::Ident("custom_keyword"),
            Atom::Punct('!'),
            Atom::Open(Delimiter::Parenthesis),
            Atom::Ident("checked"),
            Atom::Close(Delimiter::Parenthesis),
        ],
        expected: 1,
    },
    ExactCount {
        path: PARSER_PATH,
        label: "keyword::checked window",
        atoms: &[
            Atom::Ident("keyword"),
            Atom::Punct(':'),
            Atom::Punct(':'),
            Atom::Ident("checked"),
        ],
        expected: 1,
    },
    ExactCount {
        path: ENGINE_PATH,
        label: "checked: Vec window",
        atoms: &[Atom::Ident("checked"), Atom::Punct(':'), Atom::Ident("Vec")],
        expected: 1,
    },
    ExactCount {
        path: ENGINE_PATH,
        label: "self.checked window",
        atoms: &[
            Atom::Ident("self"),
            Atom::Punct('.'),
            Atom::Ident("checked"),
        ],
        expected: 1,
    },
    ExactCount {
        path: ENGINE_PATH,
        label: "observed.checked window",
        atoms: &[
            Atom::Ident("observed"),
            Atom::Punct('.'),
            Atom::Ident("checked"),
        ],
        expected: 2,
    },
    ExactCount {
        path: REPORT_PATH,
        label: "checked_constructor_bindings: Vec window",
        atoms: &[
            Atom::Ident("checked_constructor_bindings"),
            Atom::Punct(':'),
            Atom::Ident("Vec"),
        ],
        expected: 1,
    },
    ExactCount {
        path: REPORT_PATH,
        label: "checked_constructor_bindings: vec![] window",
        atoms: &[
            Atom::Ident("checked_constructor_bindings"),
            Atom::Punct(':'),
            Atom::Ident("vec"),
            Atom::Punct('!'),
            Atom::Open(Delimiter::Bracket),
            Atom::Close(Delimiter::Bracket),
        ],
        expected: 2,
    },
    ExactCount {
        path: REPORT_PATH,
        label: "report.checked_constructor_bindings window",
        atoms: &[
            Atom::Ident("report"),
            Atom::Punct('.'),
            Atom::Ident("checked_constructor_bindings"),
        ],
        expected: 3,
    },
];

fn workspace_root() -> PathBuf {
    Path::new(env!("CARGO_MANIFEST_DIR")).join("../..")
}

fn normalized_relative(root: &Path, path: &Path) -> Result<String, String> {
    path.strip_prefix(root)
        .map_err(|error| {
            format!(
                "{} is not beneath {}: {error}",
                path.display(),
                root.display()
            )
        })
        .map(|relative| relative.to_string_lossy().replace('\\', "/"))
}

fn discover_rs_files(root: &Path, roots: &[&str]) -> Result<BTreeSet<String>, String> {
    fn visit(
        workspace_root: &Path,
        directory: &Path,
        found: &mut BTreeSet<String>,
    ) -> Result<(), String> {
        let entries = fs::read_dir(directory).map_err(|error| {
            format!(
                "reading inventory directory {}: {error}",
                directory.display()
            )
        })?;
        let mut entries = entries
            .map(|entry| {
                entry.map_err(|error| {
                    format!(
                        "reading inventory entry in {}: {error}",
                        directory.display()
                    )
                })
            })
            .collect::<Result<Vec<_>, _>>()?;
        entries.sort_by_key(fs::DirEntry::file_name);

        for entry in entries {
            let path = entry.path();
            let file_type = entry
                .file_type()
                .map_err(|error| format!("reading inventory type {}: {error}", path.display()))?;
            if file_type.is_symlink() {
                return Err(format!(
                    "symlinked inventory entry is forbidden: {}",
                    path.display()
                ));
            }
            if file_type.is_dir() {
                visit(workspace_root, &path, found)?;
            } else if file_type.is_file()
                && path.extension().is_some_and(|extension| extension == "rs")
            {
                found.insert(normalized_relative(workspace_root, &path)?);
            }
        }
        Ok(())
    }

    let mut found = BTreeSet::new();
    for relative in roots {
        visit(root, &root.join(relative), &mut found)?;
    }
    Ok(found)
}

fn inventory_diff(
    root: &Path,
    roots: &[&str],
    expected: &BTreeSet<String>,
) -> Result<InventoryDiff, String> {
    let mut actual = discover_rs_files(root, roots)?;
    let singleton = "crates/xtask/src/english_v2.rs";
    match fs::symlink_metadata(root.join(singleton)) {
        Ok(metadata) if metadata.file_type().is_file() => {
            actual.insert(singleton.to_owned());
        }
        Ok(metadata) if metadata.file_type().is_symlink() => {
            return Err(format!(
                "symlinked inventory entry is forbidden: {singleton}"
            ));
        }
        Ok(_) => {}
        Err(error) if error.kind() == ErrorKind::NotFound => {}
        Err(error) => return Err(format!("reading inventory entry {singleton}: {error}")),
    }
    Ok(InventoryDiff {
        missing: expected.difference(&actual).cloned().collect(),
        unexpected: actual.difference(expected).cloned().collect(),
    })
}

fn discover_catalog_manifests(root: &Path) -> Result<BTreeSet<String>, String> {
    fn visit(
        workspace_root: &Path,
        directory: &Path,
        found: &mut BTreeSet<String>,
    ) -> Result<(), String> {
        let entries = fs::read_dir(directory).map_err(|error| {
            format!(
                "reading manifest directory {}: {error}",
                directory.display()
            )
        })?;
        for entry in entries {
            let entry = entry.map_err(|error| {
                format!("reading manifest entry in {}: {error}", directory.display())
            })?;
            let path = entry.path();
            let file_type = entry
                .file_type()
                .map_err(|error| format!("reading manifest type {}: {error}", path.display()))?;
            if file_type.is_symlink() {
                return Err(format!(
                    "symlinked manifest entry is forbidden: {}",
                    path.display()
                ));
            }
            if file_type.is_dir() {
                visit(workspace_root, &path, found)?;
            } else if file_type.is_file() && entry.file_name() == "Cargo.toml" {
                found.insert(normalized_relative(workspace_root, &path)?);
            }
        }
        Ok(())
    }

    let mut found = BTreeSet::new();
    for relative in CATALOG_MANIFEST_ROOTS {
        visit(root, &root.join(relative), &mut found)?;
    }
    Ok(found)
}

fn catalog_manifest_violations(root: &Path) -> Result<Vec<String>, String> {
    let expected = CATALOG_MANIFEST_FILES
        .iter()
        .map(|path| (*path).to_owned())
        .collect::<BTreeSet<_>>();
    let actual = discover_catalog_manifests(root)?;
    let missing = expected
        .difference(&actual)
        .cloned()
        .collect::<BTreeSet<_>>();
    let mut violations = inventory_violations(InventoryDiff {
        missing: missing.iter().cloned().collect(),
        unexpected: actual.difference(&expected).cloned().collect(),
    });
    for relative in CATALOG_MANIFEST_FILES {
        if missing.contains(*relative) {
            continue;
        }
        let source = fs::read_to_string(root.join(relative))
            .map_err(|error| format!("reading {relative}: {error}"))?;
        let observed_name = source.matches("deckmaste_catalogs").count();
        let expected_name = 2 * usize::from(*relative == "crates/xtask/Cargo.toml");
        if observed_name != expected_name {
            violations.push(format!(
                "{relative}: literal deckmaste_catalogs dependency: expected count {expected_name}, observed count {observed_name}"
            ));
        }
        let observed_exact = source
            .lines()
            .filter(|line| *line == XTASK_CATALOG_DEPENDENCY)
            .count();
        let expected_exact = usize::from(*relative == "crates/xtask/Cargo.toml");
        if observed_exact != expected_exact {
            violations.push(format!(
                "{relative}: exact reviewed xtask catalog dependency: expected count {expected_exact}, observed count {observed_exact}"
            ));
        }
    }
    Ok(violations)
}

fn flatten(stream: proc_macro2::TokenStream, tokens: &mut Vec<LexToken>) {
    for token in stream {
        match token {
            TokenTree::Group(group) => {
                tokens.push(LexToken::Open(group.delimiter()));
                flatten(group.stream(), tokens);
                tokens.push(LexToken::Close(group.delimiter()));
            }
            TokenTree::Ident(ident) => tokens.push(LexToken::Ident(ident.to_string())),
            TokenTree::Punct(punct) => tokens.push(LexToken::Punct(punct.as_char())),
            TokenTree::Literal(_) => tokens.push(LexToken::Literal),
        }
    }
}

fn lex_source(source: &str) -> Result<Vec<LexToken>, String> {
    let stream = proc_macro2::TokenStream::from_str(source)
        .map_err(|error| format!("lexing Rust token trees: {error}"))?;
    let mut tokens = Vec::new();
    flatten(stream, &mut tokens);
    Ok(tokens)
}

fn atom_matches(atom: Atom, token: &LexToken) -> bool {
    match (atom, token) {
        (Atom::Ident(expected), LexToken::Ident(observed)) => expected == observed,
        (Atom::Punct(expected), LexToken::Punct(observed)) => expected == *observed,
        (Atom::Open(expected), LexToken::Open(observed))
        | (Atom::Close(expected), LexToken::Close(observed)) => expected == *observed,
        _ => false,
    }
}

fn count_pattern(tokens: &[LexToken], atoms: &[Atom]) -> usize {
    if atoms.is_empty() {
        return 0;
    }
    tokens
        .windows(atoms.len())
        .filter(|window| {
            atoms
                .iter()
                .copied()
                .zip(*window)
                .all(|(atom, token)| atom_matches(atom, token))
        })
        .count()
}

fn path_in_scope(path: &str, scope: PathScope) -> bool {
    match scope {
        PathScope::All => true,
        PathScope::Exact(expected) => path == expected,
    }
}

fn check_forbidden(path: &str, tokens: &[LexToken], patterns: &[ForbiddenPattern]) -> Vec<String> {
    patterns
        .iter()
        .filter(|pattern| path_in_scope(path, pattern.paths))
        .filter_map(|pattern| {
            let observed = count_pattern(tokens, pattern.atoms);
            (observed != 0).then(|| {
                format!(
                    "{path}: {}: expected count 0, observed count {observed}",
                    pattern.label
                )
            })
        })
        .collect()
}

fn check_exact_counts(path: &str, tokens: &[LexToken], counts: &[ExactCount]) -> Vec<String> {
    counts
        .iter()
        .filter(|count| count.path == path)
        .filter_map(|count| {
            let observed = count_pattern(tokens, count.atoms);
            (observed != count.expected).then(|| {
                format!(
                    "{path}: {}: expected count {}, observed count {observed}",
                    count.label, count.expected
                )
            })
        })
        .collect()
}

fn check_forbidden_outside(path: &str, tokens: &[LexToken]) -> Vec<String> {
    FORBIDDEN_OUTSIDE_EXCEPTIONS
        .iter()
        .filter(|pattern| !pattern.allowed_paths.contains(&path))
        .filter_map(|pattern| {
            let observed = count_pattern(tokens, pattern.atoms);
            (observed != 0).then(|| {
                format!(
                    "{path}: {}: expected count 0, observed count {observed}",
                    pattern.label
                )
            })
        })
        .collect()
}

fn check_raw_count(path: &str, source: &str, label: &str, needle: &str, expected: usize) -> String {
    let observed = source.matches(needle).count();
    format!("{path}: {label}: expected count {expected}, observed count {observed}")
}

fn audit_file(path: &str, source: &str, tokens: &[LexToken]) -> Vec<String> {
    let mut violations = check_forbidden(path, tokens, GLOBAL_FORBIDDEN);
    violations.extend(check_forbidden(path, tokens, BUILD_FORBIDDEN));
    violations.extend(check_forbidden_outside(path, tokens));
    violations.extend(check_exact_counts(path, tokens, TOTAL_COUNTS));
    violations.extend(check_exact_counts(path, tokens, ALLOWED_WINDOWS));

    if path == PARSER_PATH {
        let checked_block = check_raw_count(path, source, "checked {} raw source", "checked {}", 1);
        if source.matches("checked {}").count() != 1 {
            violations.push(checked_block);
        }
        let diagnostic = check_raw_count(
            path,
            source,
            "retirement diagnostic raw source",
            RETIREMENT_DIAGNOSTIC,
            2,
        );
        if source.matches(RETIREMENT_DIAGNOSTIC).count() != 2 {
            violations.push(diagnostic);
        }
    }
    violations
}

#[derive(Default)]
struct CatalogAuthorityAudit {
    adapter_functions: usize,
    adapter_paths: usize,
    forbidden_paths: usize,
    inside_adapter: bool,
    adapter_file: bool,
}

impl<'ast> syn::visit::Visit<'ast> for CatalogAuthorityAudit {
    fn visit_item_fn(&mut self, function: &'ast syn::ItemFn) {
        let is_adapter = self.adapter_file && function.sig.ident == CATALOG_ADAPTER_FUNCTION;
        if is_adapter {
            self.adapter_functions += 1;
        }
        let previous = self.inside_adapter;
        self.inside_adapter = previous || is_adapter;
        syn::visit::visit_item_fn(self, function);
        self.inside_adapter = previous;
    }

    fn visit_path(&mut self, path: &'ast syn::Path) {
        let catalog_crate = concat!("deckmaste_", "catalogs");
        if path
            .segments
            .first()
            .is_some_and(|segment| segment.ident == catalog_crate)
        {
            if self.inside_adapter {
                self.adapter_paths += 1;
            } else {
                self.forbidden_paths += 1;
            }
        }
        syn::visit::visit_path(self, path);
    }
}

fn catalog_authority_violations(path: &str, source: &str) -> Result<Vec<String>, String> {
    let syntax =
        syn::parse_file(source).map_err(|error| format!("{path}: parsing Rust: {error}"))?;
    let mut audit = CatalogAuthorityAudit {
        adapter_file: path == CATALOG_ADAPTER_PATH,
        ..CatalogAuthorityAudit::default()
    };
    audit.visit_file(&syntax);

    let mut violations = Vec::new();
    if audit.forbidden_paths != 0 {
        violations.push(format!(
            "{path}: deckmaste_catalogs path outside {CATALOG_ADAPTER_FUNCTION}: expected count 0, observed count {}",
            audit.forbidden_paths
        ));
    }
    if path == CATALOG_ADAPTER_PATH {
        if audit.adapter_functions != 1 {
            violations.push(format!(
                "{path}: named catalog adapter function: expected count 1, observed count {}",
                audit.adapter_functions
            ));
        }
        if audit.adapter_paths != 2 {
            violations.push(format!(
                "{path}: catalog dependency paths inside {CATALOG_ADAPTER_FUNCTION}: expected count 2, observed count {}",
                audit.adapter_paths
            ));
        }
    }
    Ok(violations)
}

fn catalog_token_violations(path: &str, tokens: &[LexToken]) -> Vec<String> {
    let catalog_crate = Atom::Ident(concat!("deckmaste_", "catalogs"));
    let observed = count_pattern(tokens, &[catalog_crate]);
    if path == CATALOG_ADAPTER_PATH {
        (observed != 2)
            .then(|| {
                format!(
                    "{path}: catalog dependency tokens inside {CATALOG_ADAPTER_FUNCTION}: expected count 2, observed count {observed}"
                )
            })
            .into_iter()
            .collect()
    } else {
        (observed != 0)
            .then(|| {
                format!(
                    "{path}: deckmaste_catalogs path outside {CATALOG_ADAPTER_FUNCTION}: expected count 0, observed count {observed}"
                )
            })
            .into_iter()
            .collect()
    }
}

fn inventory_violations(diff: InventoryDiff) -> Vec<String> {
    let mut violations = diff
        .missing
        .into_iter()
        .map(|path| format!("{path}: inventory member: expected count 1, observed count 0"))
        .collect::<Vec<_>>();
    violations.extend(
        diff.unexpected
            .into_iter()
            .map(|path| format!("{path}: inventory member: expected count 0, observed count 1")),
    );
    violations
}

fn audit_root(root: &Path) -> Result<Vec<String>, String> {
    let expected = INVENTORY_FILES
        .iter()
        .map(|path| (*path).to_owned())
        .collect();
    let diff = inventory_diff(root, INVENTORY_ROOTS, &expected)?;
    let missing = diff.missing.iter().cloned().collect::<BTreeSet<_>>();
    let mut violations = inventory_violations(diff);
    for relative in INVENTORY_FILES {
        if missing.contains(*relative) {
            continue;
        }
        let source = fs::read_to_string(root.join(relative))
            .map_err(|error| format!("reading {relative}: {error}"))?;
        let tokens = lex_source(&source).map_err(|error| format!("{relative}: {error}"))?;
        violations.extend(audit_file(relative, &source, &tokens));
        violations.extend(catalog_token_violations(relative, &tokens));
        violations.extend(catalog_authority_violations(relative, &source)?);
    }
    violations.extend(catalog_manifest_violations(root)?);
    violations.sort();
    violations.dedup();
    Ok(violations)
}

fn copy_inventory(source_root: &Path, destination_root: &Path) -> Result<(), String> {
    for relative in INVENTORY_FILES.iter().chain(CATALOG_MANIFEST_FILES) {
        let destination = destination_root.join(relative);
        fs::create_dir_all(destination.parent().expect("inventory path has a parent"))
            .map_err(|error| format!("creating parent for {relative}: {error}"))?;
        fs::copy(source_root.join(relative), &destination)
            .map_err(|error| format!("copying {relative}: {error}"))?;
    }
    Ok(())
}

fn hash_inventory(root: &Path) -> Result<[u8; 32], String> {
    let mut paths = INVENTORY_FILES.to_vec();
    paths.sort_unstable();
    let mut hash = sha2::Sha256::new();
    for relative in paths {
        let bytes = fs::read(root.join(relative))
            .map_err(|error| format!("reading {relative} for hash: {error}"))?;
        hash.update(relative.as_bytes());
        hash.update([0]);
        hash.update(bytes);
        hash.update([0]);
    }
    Ok(hash.finalize().into())
}

#[test]
fn plan04_closed_world_inventory_fails_loud() {
    let live = workspace_root();
    let temp = tempfile::tempdir().expect("temporary inventory root");
    copy_inventory(&live, temp.path()).expect("inventory copies");
    assert_eq!(audit_root(temp.path()), Ok(vec![]));

    let unreviewed_manifest = "crates/deckmaste_english_v2/generated/Cargo.toml";
    fs::create_dir_all(
        temp.path()
            .join(unreviewed_manifest)
            .parent()
            .expect("manifest fixture has parent"),
    )
    .expect("manifest fixture parent writes");
    fs::write(
        temp.path().join(unreviewed_manifest),
        "[package]\nname = \"rogue\"\n",
    )
    .expect("unreviewed manifest fixture writes");
    assert!(audit_root(temp.path()).unwrap().iter().any(|violation| {
        violation
            == &format!(
                "{unreviewed_manifest}: inventory member: expected count 0, observed count 1"
            )
    }));
    fs::remove_file(temp.path().join(unreviewed_manifest))
        .expect("unreviewed manifest fixture removes");

    let unexpected = "crates/deckmaste_english_v2/src/unreviewed.rs";
    fs::write(temp.path().join(unexpected), "fn unreviewed_surface() {}")
        .expect("unexpected fixture writes");
    assert_eq!(
        audit_root(temp.path()),
        Ok(vec![format!(
            "{unexpected}: inventory member: expected count 0, observed count 1"
        )]),
    );

    let missing = "crates/deckmaste_english_v2/src/lib.rs";
    fs::remove_file(temp.path().join(missing)).expect("missing fixture removes");
    assert_eq!(
        audit_root(temp.path()),
        Ok(vec![
            format!("{missing}: inventory member: expected count 1, observed count 0"),
            format!("{unexpected}: inventory member: expected count 0, observed count 1"),
        ]),
    );
}

#[test]
fn plan04_closed_world_lexical_tokens_reject_retired_authority() {
    let live = workspace_root();
    let temp = tempfile::tempdir().expect("temporary audit root");
    copy_inventory(&live, temp.path()).expect("inventory copies");
    let relative = "crates/deckmaste_english_v2/src/lib.rs";
    let fixture = temp.path().join(relative);
    let original = fs::read(&fixture).expect("copied fixture reads");
    let source = r#"
        macro_rules! emits { ($body:tt) => { impl Triggered $body }; }
        const PROSE: &str = "impl Triggered { checked_constructor }";
    "#;
    fs::write(&fixture, [original.as_slice(), source.as_bytes()].concat())
        .expect("lexical fixture writes only in disposable root");
    assert_eq!(
        audit_root(temp.path()),
        Ok(vec![format!(
            "{relative}: impl Triggered: expected count 0, observed count 1"
        )]),
    );
}

#[test]
fn plan04_catalog_dependency_is_confined_to_the_named_xtask_adapter() {
    let live = workspace_root();
    let temp = tempfile::tempdir().expect("temporary audit root");
    copy_inventory(&live, temp.path()).expect("inventory copies");
    assert_eq!(audit_root(temp.path()), Ok(vec![]));

    let relative = "crates/deckmaste_english_v2/src/environment.rs";
    let fixture = temp.path().join(relative);
    let original = fs::read(&fixture).expect("copied environment reads");
    fs::write(
        &fixture,
        [
            original.as_slice(),
            b"\nfn rogue_catalog_lookup() { let _ = deckmaste_catalogs::CatalogKind::CardNames; }\n",
        ]
        .concat(),
    )
    .expect("rogue dependency writes only in disposable root");
    assert_eq!(
        audit_root(temp.path()),
        Ok(vec![format!(
            "{relative}: deckmaste_catalogs path outside adapt_card_name_catalog_provider: expected count 0, observed count 1"
        )])
    );
}

#[test]
fn plan04_closed_world_disposable_root_authenticates_red_then_green() {
    let live = workspace_root();
    let before = hash_inventory(&live).expect("live inventory hashes");
    let temp = tempfile::tempdir().expect("temporary audit root");
    copy_inventory(&live, temp.path()).expect("inventory copies");
    let fixture = temp.path().join("crates/deckmaste_english_v2/src/lib.rs");
    let original = fs::read(&fixture).expect("copied fixture reads");
    fs::write(
        &fixture,
        [original.as_slice(), b"\nstruct Checked;\n"].concat(),
    )
    .expect("sentinel writes only in disposable root");
    assert!(audit_root(temp.path()).unwrap().iter().any(|v| {
        v.contains("crates/deckmaste_english_v2/src/lib.rs") && v.contains("struct Checked")
    }));
    fs::write(&fixture, &original).expect("disposable fixture restores");
    assert_eq!(audit_root(temp.path()), Ok(vec![]));
    assert_eq!(hash_inventory(&live).unwrap(), before);
}

#[test]
fn plan04_generated_invariants_are_single_authority() {
    let violations = audit_root(&workspace_root()).expect("closed-world authority audit runs");
    assert!(
        violations.is_empty(),
        "This audit proves the current repository architecture: the direct constructions! \
         invocation is the generated authority; deckmaste_catalogs appears only in the named \
         xtask adapter; generated required-provider metadata plus immutable environment lookup \
         are the only downstream catalog-identity authority; and inventoried production source \
         contains no retired competing authority. It does not decide arbitrary future Rust \
         metaprograms.\n{}",
        violations.join("\n"),
    );
}
