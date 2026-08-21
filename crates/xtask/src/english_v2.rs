//! Human-readable expansion of the English-v2 construction declaration.

mod ambiguity;
mod audit;
mod corpus;
mod coverage;
mod coverage_lock;
mod diagnostic;
mod inspect;
mod parse;
mod probe;
mod report;
mod roundtrip;

use std::fmt::Write as _;
use std::fs;
use std::io::Write;
use std::path::Path;
use std::path::PathBuf;

use anyhow::Context;
use clap::Args;
use clap::Subcommand;
use deckmaste_construction_core::DeclarationKind;
use deckmaste_construction_core::Expansion;
use deckmaste_construction_core::ItemKey;
use deckmaste_construction_core::NamedKind;
use deckmaste_construction_core::TerminalBindingDeclarationKind;
use deckmaste_english_v2::environment::ParserEnvironment;
use deckmaste_english_v2::parser::Parser;

fn parser_from_builtin_v2() -> Parser {
    let declarations = macro_ron::v2::read_builtin_v2(
        Path::new(env!("CARGO_MANIFEST_DIR")).join("../../plugins/builtin_v2"),
    )
    .expect("integrated builtin-v2 declarations load");
    let environment = ParserEnvironment::try_from_declarations(declarations)
        .expect("integrated builtin-v2 declarations freeze");
    Parser::new(environment).expect("builtin-v2 supplies every generated static declaration")
}

#[derive(Debug, Args)]
pub struct EnglishV2Args {
    #[command(subcommand)]
    command: EnglishV2Command,
}

#[derive(Debug, Subcommand)]
enum EnglishV2Command {
    /// Print generated items and counted declaration escape hatches.
    Expand,
    /// Audit every normalized corpus face with the English-v2 parser.
    Parse(ParseArgs),
    /// Gate exact rendering for every corpus face the parser accepts.
    Roundtrip(RoundtripArgs),
    /// Print schema-versioned counted English-v2 escape hatches.
    Report(ReportArgs),
    /// Census complete corpus selection decisions and unresolved ambiguities.
    Ambiguity(AmbiguityArgs),
    /// Report and gate full-corpus lexical ownership coverage.
    Coverage(CoverageArgs),
    /// Trace one explicit input through the bounded parser diagnostics.
    Probe(ProbeArgs),
    /// Trace one exact corpus unit through the bounded parser diagnostics.
    Inspect(InspectArgs),
}

#[derive(Debug, clap::Args)]
struct CorpusArgs {
    #[arg(long, default_value = "data/mtgjson/AtomicCards.json")]
    data: PathBuf,
}

#[derive(Debug, clap::Args)]
struct ParseArgs {
    #[command(flatten)]
    corpus: CorpusArgs,
    #[arg(long)]
    json: bool,
    #[arg(long)]
    require_complete: bool,
}

#[derive(Debug, clap::Args)]
struct RoundtripArgs {
    #[command(flatten)]
    corpus: CorpusArgs,
    #[arg(long)]
    json: bool,
    #[arg(long)]
    require_clean: bool,
}

#[derive(Debug, clap::Args)]
struct ReportArgs {
    #[arg(long)]
    json: bool,
}

#[derive(Debug, clap::Args)]
struct AmbiguityArgs {
    #[command(flatten)]
    corpus: CorpusArgs,
    #[arg(long)]
    json: bool,
    #[arg(long)]
    require_resolved: bool,
}

#[derive(Debug, clap::Args)]
#[group(id = "coverage_gate", multiple = false)]
struct CoverageArgs {
    #[command(flatten)]
    corpus: CorpusArgs,
    #[arg(long, default_value = "english-v2-coverage.lock")]
    lock: PathBuf,
    #[arg(long)]
    json: bool,
    #[arg(long, group = "coverage_gate")]
    check: bool,
    #[arg(long, group = "coverage_gate")]
    bless: bool,
}

#[derive(Debug, clap::Args)]
struct ProbeArgs {
    #[arg(long)]
    text: String,
    #[arg(long)]
    context: String,
    #[arg(long, default_value_t = 256)]
    limit: usize,
    #[arg(long)]
    json: bool,
}

#[derive(Debug, clap::Args)]
struct InspectArgs {
    #[arg(long)]
    id: String,
    #[arg(long, default_value = "data/mtgjson/AtomicCards.json")]
    data: PathBuf,
    #[arg(long, default_value_t = 256)]
    limit: usize,
    #[arg(long)]
    json: bool,
}

pub fn run(args: &EnglishV2Args) -> anyhow::Result<()> {
    match &args.command {
        EnglishV2Command::Expand => {
            let mut stdout = std::io::stdout().lock();
            run_from_path(&production_declaration_path(), &mut stdout)
        }
        EnglishV2Command::Parse(args) => {
            let mut stdout = std::io::stdout().lock();
            parse::run(args, &mut stdout)
        }
        EnglishV2Command::Roundtrip(args) => {
            let mut stdout = std::io::stdout().lock();
            roundtrip::run(args, &mut stdout)
        }
        EnglishV2Command::Report(args) => {
            let mut stdout = std::io::stdout().lock();
            report::run(args, &mut stdout)
        }
        EnglishV2Command::Ambiguity(args) => {
            let mut stdout = std::io::stdout().lock();
            ambiguity::run(args, &mut stdout)
        }
        EnglishV2Command::Coverage(args) => {
            let mut stdout = std::io::stdout().lock();
            coverage::run(args, &mut stdout)
        }
        EnglishV2Command::Probe(args) => {
            let mut stdout = std::io::stdout().lock();
            probe::run(args, &mut stdout)
        }
        EnglishV2Command::Inspect(args) => {
            let mut stdout = std::io::stdout().lock();
            inspect::run(args, &mut stdout)
        }
    }
}

fn production_declaration_path() -> PathBuf {
    Path::new(env!("CARGO_MANIFEST_DIR")).join("../deckmaste_english_v2/src/constructions.rs")
}

fn run_from_path(path: &Path, output: &mut dyn Write) -> anyhow::Result<()> {
    let source = fs::read_to_string(path).with_context(|| format!("reading {}", path.display()))?;
    let rendered = expand_source(&source)?;
    output
        .write_all(rendered.as_bytes())
        .context("writing English-v2 declaration expansion")
}

pub fn expand_source(source: &str) -> anyhow::Result<String> {
    let expansion = expansion_from_source(source)?;
    render_expansion(&expansion)
}

fn expansion_from_source(source: &str) -> anyhow::Result<Expansion> {
    let invocation =
        deckmaste_construction_core::invocation_from_source(source).map_err(anyhow::Error::new)?;
    deckmaste_construction_core::generate(invocation.tokens).map_err(anyhow::Error::new)
}

fn render_expansion(expansion: &Expansion) -> anyhow::Result<String> {
    let mut output = String::new();
    for item in expansion.items() {
        let key = item_key_name(&item.key);
        writeln!(&mut output, "// === {key} ===").expect("writing to String cannot fail");
        for origin in &item.origins {
            writeln!(
                &mut output,
                "// origin: {} {}",
                declaration_kind_name(origin.kind()),
                origin.name()
            )
            .expect("writing to String cannot fail");
        }
        output.push_str(
            &deckmaste_construction_core::format_generated_item(item)
                .map_err(anyhow::Error::new)?,
        );
    }

    output.push_str("// === counted escape hatches ===\n");
    let report = expansion.escape_hatches();
    writeln!(
        &mut output,
        "// morphology irregulars ({})",
        report.morphology_irregulars().len()
    )
    .expect("writing to String cannot fail");
    for irregular in report.morphology_irregulars() {
        writeln!(&mut output, "// - {}", irregular.identity())
            .expect("writing to String cannot fail");
        for row in irregular.overrides() {
            writeln!(
                &mut output,
                "//   - {} = {:?}",
                row.feature(),
                row.surface()
            )
            .expect("writing to String cannot fail");
        }
    }
    writeln!(
        &mut output,
        "// terminal bindings ({})",
        report.terminal_bindings().len()
    )
    .expect("writing to String cannot fail");
    for binding in report.terminal_bindings() {
        let kind = match binding.kind() {
            TerminalBindingDeclarationKind::Codec => "codec",
            TerminalBindingDeclarationKind::Identity => "identity",
        };
        writeln!(&mut output, "// - {kind} {}", binding.name())
            .expect("writing to String cannot fail");
    }
    writeln!(&mut output, "// roots ({})", report.roots().len())
        .expect("writing to String cannot fail");
    for name in report.roots() {
        writeln!(&mut output, "// - root {name}").expect("writing to String cannot fail");
    }
    Ok(output)
}

fn item_key_name(key: &ItemKey) -> String {
    match key {
        ItemKey::Named { kind, name } => {
            let kind = match kind {
                NamedKind::Type => "type",
                NamedKind::Trait => "trait",
                NamedKind::Function => "function",
                NamedKind::Constant => "constant",
            };
            format!("{kind} {name}")
        }
        ItemKey::Impl {
            trait_name: Some(trait_name),
            self_ty,
        } => format!("impl {trait_name} for {self_ty}"),
        ItemKey::Impl {
            trait_name: None,
            self_ty,
        } => format!("impl {self_ty}"),
    }
}

fn declaration_kind_name(kind: DeclarationKind) -> &'static str {
    match kind {
        DeclarationKind::Construction => "construction",
        DeclarationKind::Vocab => "vocab",
        DeclarationKind::Morphology => "morphology",
        DeclarationKind::Lexeme => "lexeme",
        DeclarationKind::Codec => "codec",
        DeclarationKind::Identity => "identity",
        DeclarationKind::Root => "root",
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    const PRODUCTION_SOURCE: &str = include_str!("../../deckmaste_english_v2/src/constructions.rs");
    type ClosedLexemeDeclarations = std::collections::BTreeSet<String>;

    #[allow(
        dead_code,
        unused_parens,
        reason = "the audit regression compiles every otherwise-forbidden impl spelling"
    )]
    mod plan04_compile_checked_impl_spellings {
        pub struct Triggered;

        impl (Triggered) {
            fn parenthesized() {}
        }

        type Alias = Triggered;
        impl Alias {
            fn direct_alias() {}
        }

        type TransitiveAlias = Alias;
        impl TransitiveAlias {
            fn transitive_alias() {}
        }

        use self::Triggered as UseAlias;
        impl UseAlias {
            fn use_alias() {}
        }

        macro_rules! inherent_template {
            ($body:item) => {
                impl Triggered {
                    $body
                }
            };
        }
        inherent_template! {
            fn macro_template() {}
        }

        mod exported {
            pub(super) type Alias = super::Triggered;
            pub(super) type Transitive = self::Alias;
            pub(super) use self::Transitive as Renamed;

            pub(super) mod nested {
                pub(crate) type Deep = super::Renamed;

                impl self::Deep {
                    fn nested_self_alias() {}
                }
            }

            impl self::Alias {
                fn module_self_alias() {}
            }
        }

        impl exported::Alias {
            fn qualified_alias() {}
        }

        use self::exported::Renamed as ImportedAlias;
        impl ImportedAlias {
            fn imported_alias() {}
        }

        mod from_super {
            pub(super) type Alias = super::exported::nested::Deep;

            impl Alias {
                fn super_alias() {}
            }
        }
    }

    fn contains_production_function(file: &syn::File, name: &str) -> bool {
        contains_production_authority(file, ProductionAuthorityKind::Function, name)
    }

    fn contains_production_authority(
        file: &syn::File,
        kind: ProductionAuthorityKind,
        name: &str,
    ) -> bool {
        use syn::visit::Visit as _;

        let mut finder = ProductionAuthorityFinder {
            kind,
            target: name,
            found: false,
        };
        finder.visit_file(file);
        finder.found
    }

    fn contains_production_identifier(file: &syn::File, name: &str) -> bool {
        use syn::visit::Visit as _;

        let mut finder = ProductionIdentifierFinder {
            target: name,
            found: false,
        };
        finder.visit_file(file);
        finder.found
    }

    fn contains_production_method_call(file: &syn::File, name: &str) -> bool {
        use syn::visit::Visit as _;

        let mut finder = ProductionMethodCallFinder {
            target: name,
            found: false,
        };
        finder.visit_file(file);
        finder.found
    }

    fn production_string_literals(file: &syn::File) -> Vec<String> {
        use syn::visit::Visit as _;

        let mut finder = ProductionStringLiteralFinder::default();
        finder.visit_file(file);
        finder.found
    }

    fn contains_test_only_generated_ghost(file: &syn::File) -> bool {
        use syn::visit::Visit as _;

        let mut finder = TestOnlyGeneratedGhostFinder::default();
        finder.visit_file(file);
        finder.found
    }

    fn contains_production_code_indirection(file: &syn::File) -> bool {
        use syn::visit::Visit as _;

        let mut finder = ProductionCodeIndirectionFinder::default();
        finder.visit_file(file);
        finder.found
    }

    fn discover_rust_sources(root: &Path) -> Vec<PathBuf> {
        fn visit(directory: &Path, found: &mut Vec<PathBuf>) {
            let mut entries = fs::read_dir(directory)
                .unwrap_or_else(|error| panic!("{}: {error}", directory.display()))
                .map(|entry| {
                    entry.unwrap_or_else(|error| panic!("{}: {error}", directory.display()))
                })
                .collect::<Vec<_>>();
            entries.sort_by_key(std::fs::DirEntry::file_name);

            for entry in entries {
                let path = entry.path();
                let file_type = entry
                    .file_type()
                    .unwrap_or_else(|error| panic!("{}: {error}", path.display()));
                assert!(
                    !file_type.is_symlink(),
                    "{}: symlinked audit entry is forbidden",
                    path.display()
                );
                if file_type.is_dir() {
                    visit(&path, found);
                } else if path.extension().is_some_and(|extension| extension == "rs") {
                    found.push(path);
                }
            }
        }

        let mut found = Vec::new();
        visit(root, &mut found);
        found.sort();
        found
    }

    fn parse_rust_sources(paths: &[PathBuf]) -> Vec<(String, syn::File)> {
        paths
            .iter()
            .map(|path| {
                let source = fs::read_to_string(path)
                    .unwrap_or_else(|error| panic!("{}: {error}", path.display()));
                let file = syn::parse_file(&source)
                    .unwrap_or_else(|error| panic!("{}: {error}", path.display()));
                (path.display().to_string(), file)
            })
            .collect()
    }

    fn english_runtime_authority_violations(
        path: &str,
        file: &syn::File,
        closed_lexemes: &ClosedLexemeDeclarations,
    ) -> Vec<String> {
        let mut violations = Vec::new();
        if contains_production_code_indirection(file) {
            violations.push(format!(
                "{path} uses production #[path] or code-including include! indirection"
            ));
        }
        for generated_type in ["Lexical", "Leaf", "TerminalClass", "NounNumber"] {
            for kind in [
                ProductionAuthorityKind::Enum,
                ProductionAuthorityKind::Struct,
                ProductionAuthorityKind::TypeAlias,
            ] {
                if contains_production_authority(file, kind, generated_type) {
                    violations.push(format!(
                        "{path} defines handwritten generated aggregate {generated_type}"
                    ));
                }
            }
        }
        for generated_authority in [
            "Sign",
            "SignedNumber",
            "SelfReferenceSpelling",
            "Noun",
            concat!("Catalog", "Identity"),
        ] {
            for kind in [
                ProductionAuthorityKind::Enum,
                ProductionAuthorityKind::Struct,
                ProductionAuthorityKind::TypeAlias,
            ] {
                if contains_production_authority(file, kind, generated_authority) {
                    violations.push(format!(
                        "{path} defines handwritten generated authority {generated_authority}"
                    ));
                }
            }
        }
        for forbidden_function in [
            "inflect",
            "scan_verb",
            "scan_bound_terminal",
            "scan_signed_number",
            "scan_noun",
            "noun_forms",
            "render_noun",
            "pluralize",
            "walk_declaration_noun",
            "walk_noun",
            "render_self_reference_spelling",
            "scan_self_reference_spelling",
            "walk_self_reference_spelling",
            "visit_self_reference_spelling",
        ] {
            if contains_production_function(file, forbidden_function) {
                violations.push(format!(
                    "{path} defines handwritten terminal path {forbidden_function}"
                ));
            }
        }
        if contains_closed_lexeme_surface_mirror(file, closed_lexemes) {
            violations.push(format!(
                "{path} retains a handwritten closed-lexeme surface mirror"
            ));
        }
        if contains_test_only_generated_ghost(file) {
            violations.push(format!(
                "{path} recreates a generated build/rules authority under cfg(test)"
            ));
        }
        let literals = production_string_literals(file);
        for owner in literals
            .iter()
            .filter(|literal| is_two_segment_closed_lexeme_owner(literal, closed_lexemes))
        {
            violations.push(format!(
                "{path} retains two-segment closed lexeme owner literal {owner}"
            ));
        }
        for forbidden in ["destroy", "destroys", "connive", "connives"] {
            if literals.iter().any(|literal| literal == forbidden) {
                violations.push(format!(
                    "{path} retains handwritten open-verb spelling {forbidden}"
                ));
            }
        }
        violations
    }

    fn contains_closed_lexeme_surface_mirror(
        file: &syn::File,
        closed_lexemes: &ClosedLexemeDeclarations,
    ) -> bool {
        use syn::visit::Visit as _;

        let mut finder = ClosedLexemeSurfaceMirrorFinder {
            declarations: closed_lexemes,
            found: false,
        };
        finder.visit_file(file);
        finder.found
    }

    fn is_two_segment_closed_lexeme_owner(
        literal: &str,
        closed_lexemes: &ClosedLexemeDeclarations,
    ) -> bool {
        let Some(owner) = literal.strip_prefix("lexeme:") else {
            return false;
        };
        let mut segments = owner.split('/');
        matches!(
            (segments.next(), segments.next(), segments.next()),
            (Some(declaration), Some(member), None)
                if closed_lexemes.contains(declaration) && !member.is_empty()
        )
    }

    fn closed_lexeme_declarations(expansion: &Expansion) -> ClosedLexemeDeclarations {
        expansion
            .terminal_contributions()
            .iter()
            .filter(|contribution| {
                contribution.kind() == deckmaste_construction_core::TerminalKind::Lexeme
            })
            .map(|contribution| contribution.name().to_owned())
            .collect()
    }

    fn uses_source_shaped_authority(file: &syn::File) -> bool {
        [
            "ValidatedDeclarations",
            "Declarations",
            "parse_declarations",
            "invocation_from_source",
        ]
        .iter()
        .any(|authority| contains_production_identifier(file, authority))
            || contains_production_method_call(file, "raw")
    }

    fn source_authority_paths(root: &Path) -> Vec<String> {
        let paths = discover_rust_sources(root);
        parse_rust_sources(&paths)
            .iter()
            .filter(|(_, file)| uses_source_shaped_authority(file))
            .map(|(path, _)| {
                Path::new(path)
                    .strip_prefix(root)
                    .unwrap_or_else(|error| panic!("{path}: {error}"))
                    .to_string_lossy()
                    .into_owned()
            })
            .collect()
    }

    fn plan04_source_fixture_paths(workspace_root: &Path) -> Vec<PathBuf> {
        fn visit(directory: &Path, found: &mut Vec<PathBuf>) {
            let mut entries = fs::read_dir(directory)
                .unwrap_or_else(|error| panic!("{}: {error}", directory.display()))
                .map(|entry| {
                    entry.unwrap_or_else(|error| panic!("{}: {error}", directory.display()))
                })
                .collect::<Vec<_>>();
            entries.sort_by_key(std::fs::DirEntry::file_name);

            for entry in entries {
                let path = entry.path();
                let file_type = entry
                    .file_type()
                    .unwrap_or_else(|error| panic!("{}: {error}", path.display()));
                assert!(
                    !file_type.is_symlink(),
                    "{}: symlinked audit entry is forbidden",
                    path.display()
                );
                if file_type.is_dir() {
                    visit(&path, found);
                } else if path.extension().is_some_and(|extension| {
                    matches!(extension.to_str(), Some("rs" | "ron" | "stderr" | "txt"))
                }) {
                    found.push(path);
                }
            }
        }

        let mut found = Vec::new();
        for root in [
            "crates/deckmaste_construction_core/src",
            "crates/deckmaste_construction_core/tests",
            "crates/deckmaste_construction/src",
            "crates/deckmaste_construction/tests",
            "crates/deckmaste_english_v2/src",
            "crates/deckmaste_english_v2/tests",
        ] {
            visit(&workspace_root.join(root), &mut found);
        }
        found.push(workspace_root.join("crates/xtask/src/english_v2/report.rs"));
        found.sort();
        found.dedup();
        found
    }

    fn plan04_module_location(workspace_root: &Path, path: &Path) -> (String, Vec<String>) {
        let relative = path
            .strip_prefix(workspace_root)
            .unwrap_or_else(|error| panic!("{}: {error}", path.display()));
        let components = relative
            .components()
            .map(|component| component.as_os_str().to_string_lossy().into_owned())
            .collect::<Vec<_>>();
        if let [crates, crate_name, surface, rest @ ..] = components.as_slice()
            && crates == "crates"
            && surface == "src"
            && !rest.is_empty()
        {
            let mut module = rest.to_vec();
            let file = module.pop().expect("source path has a file");
            if file != "lib.rs" && file != "main.rs" && file != "mod.rs" {
                module.push(
                    Path::new(&file)
                        .file_stem()
                        .expect("Rust source has a stem")
                        .to_string_lossy()
                        .into_owned(),
                );
            }
            return (format!("crate:{crate_name}"), module);
        }
        if let [crates, crate_name, surface, rest @ ..] = components.as_slice()
            && crates == "crates"
            && surface == "tests"
            && !rest.is_empty()
        {
            let integration_root = Path::new(&rest[0])
                .file_stem()
                .expect("integration test path has a stem")
                .to_string_lossy();
            let mut module = rest[1..].to_vec();
            if let Some(file) = module.pop()
                && file != "mod.rs"
            {
                module.push(
                    Path::new(&file)
                        .file_stem()
                        .expect("Rust source has a stem")
                        .to_string_lossy()
                        .into_owned(),
                );
            }
            return (format!("test:{crate_name}:{integration_root}"), module);
        }
        (format!("file:{}", relative.display()), Vec::new())
    }

    fn plan04_qualified_alias_graphs(
        workspace_root: &Path,
        paths: &[PathBuf],
    ) -> std::collections::BTreeMap<String, QualifiedAliasGraph> {
        let mut grouped =
            std::collections::BTreeMap::<String, Vec<(syn::File, Vec<String>)>>::new();
        for path in paths {
            if path.extension().is_none_or(|extension| extension != "rs") {
                continue;
            }
            let source = fs::read_to_string(path)
                .unwrap_or_else(|error| panic!("{}: {error}", path.display()));
            let file = syn::parse_file(&source)
                .unwrap_or_else(|error| panic!("{}: {error}", path.display()));
            let (key, module) = plan04_module_location(workspace_root, path);
            grouped.entry(key).or_default().push((file, module));
        }
        grouped
            .into_iter()
            .map(|(key, files)| {
                let graph = QualifiedAliasGraph::from_files(
                    files.iter().map(|(file, module)| (file, module.as_slice())),
                    false,
                );
                (key, graph)
            })
            .collect()
    }

    #[derive(Default)]
    struct Plan04RetiredAuthorityFinder {
        allow_report_schema_field: bool,
        skip_strict_test_only: bool,
        alias_scopes: Vec<std::collections::BTreeMap<String, String>>,
        qualified_aliases: QualifiedAliasGraph,
        module_path: Vec<String>,
        qualified_aliases_initialized: bool,
        violations: Vec<&'static str>,
    }

    #[derive(Clone, Default)]
    struct QualifiedAliasGraph {
        bindings: std::collections::BTreeMap<String, String>,
        symbols: std::collections::BTreeSet<String>,
        glob_imports: Vec<(Vec<String>, Vec<String>)>,
    }

    impl Plan04RetiredAuthorityFinder {
        fn record(&mut self, authority: &'static str) {
            if !self.violations.contains(&authority) {
                self.violations.push(authority);
            }
        }

        fn visible_aliases(&self) -> std::collections::BTreeMap<String, String> {
            let mut aliases = std::collections::BTreeMap::new();
            for scope in &self.alias_scopes {
                aliases.extend(scope.clone());
            }
            aliases
        }

        fn type_resolves_to(&self, ty: &syn::Type, target: &str) -> bool {
            type_path(ty).is_some_and(|path| {
                path.path
                    .segments
                    .last()
                    .is_some_and(|segment| segment.ident == target)
                    || self
                        .qualified_aliases
                        .path_resolves_to(path, &self.module_path, target)
            })
        }
    }

    fn plan04_host_authorities(file: &syn::File) -> Vec<&'static str> {
        use syn::visit::Visit as _;

        let mut finder = Plan04RetiredAuthorityFinder {
            skip_strict_test_only: true,
            ..Plan04RetiredAuthorityFinder::default()
        };
        finder.visit_file(file);
        finder.violations
    }

    fn plan04_host_authorities_with_report(
        host: &syn::File,
        report: &syn::File,
        host_module: &[String],
    ) -> Vec<&'static str> {
        use syn::visit::Visit as _;

        let mut report_module = host_module.to_vec();
        report_module.push("report".to_owned());
        let graph = QualifiedAliasGraph::from_files(
            [(host, host_module), (report, report_module.as_slice())],
            true,
        );
        let mut finder = Plan04RetiredAuthorityFinder {
            skip_strict_test_only: true,
            qualified_aliases: graph,
            module_path: host_module.to_vec(),
            qualified_aliases_initialized: true,
            ..Plan04RetiredAuthorityFinder::default()
        };
        finder.visit_file(host);
        finder.violations
    }

    fn plan04_retired_text_authorities(source: &str) -> Vec<&'static str> {
        [
            (r"\bstruct\s+Checked\b", "struct Checked"),
            (
                r"\bstruct\s+ConstructorBinding\b",
                "struct ConstructorBinding",
            ),
            (
                r"\benum\s+ConstructorArgument\b",
                "enum ConstructorArgument",
            ),
            (r"\bchecked_constructor\b", "checked_constructor"),
        ]
        .into_iter()
        .filter_map(|(pattern, authority)| {
            regex::Regex::new(pattern)
                .unwrap_or_else(|error| panic!("{authority} audit pattern: {error}"))
                .is_match(source)
                .then_some(authority)
        })
        .collect()
    }

    fn type_path(ty: &syn::Type) -> Option<&syn::TypePath> {
        match ty {
            syn::Type::Group(group) => type_path(&group.elem),
            syn::Type::Paren(paren) => type_path(&paren.elem),
            syn::Type::Path(path) if path.qself.is_none() => Some(path),
            _ => None,
        }
    }

    fn type_last_name(ty: &syn::Type) -> Option<String> {
        type_path(ty).and_then(|path| {
            path.path
                .segments
                .last()
                .map(|segment| segment.ident.to_string())
        })
    }

    fn qualified_name(module: &[String], name: &str) -> String {
        if module.is_empty() {
            name.to_owned()
        } else {
            format!("{}::{name}", module.join("::"))
        }
    }

    fn normalize_qualified_segments(segments: &[String], module: &[String]) -> Vec<String> {
        let mut normalized = module.to_vec();
        let mut index = 0;
        if segments.first().is_some_and(|segment| segment == "crate") {
            normalized.clear();
            index = 1;
        } else if segments.first().is_some_and(|segment| segment == "self") {
            index = 1;
        } else {
            while segments
                .get(index)
                .is_some_and(|segment| segment == "super")
            {
                normalized.pop();
                index += 1;
            }
        }
        normalized.extend(segments[index..].iter().cloned());
        normalized
    }

    fn syn_path_segments(path: &syn::Path) -> Vec<String> {
        path.segments
            .iter()
            .map(|segment| segment.ident.to_string())
            .collect()
    }

    impl QualifiedAliasGraph {
        fn from_file(file: &syn::File, root: &[String], skip_strict_test_only: bool) -> Self {
            Self::from_files([(file, root)], skip_strict_test_only)
        }

        fn from_files<'ast>(
            files: impl IntoIterator<Item = (&'ast syn::File, &'ast [String])> + Clone,
            skip_strict_test_only: bool,
        ) -> Self {
            let mut graph = Self::default();
            for (file, root) in files.clone() {
                graph.collect_symbols(&file.items, root, skip_strict_test_only);
            }
            for (file, root) in files {
                graph.collect_bindings(&file.items, root, skip_strict_test_only);
            }
            graph.expand_globs();
            graph
        }

        fn collect_symbols(
            &mut self,
            items: &[syn::Item],
            module: &[String],
            skip_strict_test_only: bool,
        ) {
            for item in items {
                if skip_strict_test_only && item_attrs(item).is_some_and(is_test_only) {
                    continue;
                }
                match item {
                    syn::Item::Enum(item) => {
                        self.symbols
                            .insert(qualified_name(module, &item.ident.to_string()));
                    }
                    syn::Item::Struct(item) => {
                        self.symbols
                            .insert(qualified_name(module, &item.ident.to_string()));
                    }
                    syn::Item::Type(item) => {
                        self.symbols
                            .insert(qualified_name(module, &item.ident.to_string()));
                    }
                    syn::Item::Union(item) => {
                        self.symbols
                            .insert(qualified_name(module, &item.ident.to_string()));
                    }
                    syn::Item::Mod(item) => {
                        let mut child = module.to_vec();
                        child.push(item.ident.to_string());
                        self.symbols.insert(child.join("::"));
                        if let Some((_, items)) = &item.content {
                            self.collect_symbols(items, &child, skip_strict_test_only);
                        }
                    }
                    _ => {}
                }
            }
        }

        fn collect_bindings(
            &mut self,
            items: &[syn::Item],
            module: &[String],
            skip_strict_test_only: bool,
        ) {
            for item in items {
                if skip_strict_test_only && item_attrs(item).is_some_and(is_test_only) {
                    continue;
                }
                match item {
                    syn::Item::Enum(item) => {
                        let name = qualified_name(module, &item.ident.to_string());
                        self.bindings.insert(name.clone(), name);
                    }
                    syn::Item::Struct(item) => {
                        let name = qualified_name(module, &item.ident.to_string());
                        self.bindings.insert(name.clone(), name);
                    }
                    syn::Item::Type(item) => {
                        if let Some(path) = type_path(&item.ty) {
                            let target = normalize_qualified_segments(
                                &syn_path_segments(&path.path),
                                module,
                            )
                            .join("::");
                            self.bindings
                                .insert(qualified_name(module, &item.ident.to_string()), target);
                        }
                    }
                    syn::Item::Union(item) => {
                        let name = qualified_name(module, &item.ident.to_string());
                        self.bindings.insert(name.clone(), name);
                    }
                    syn::Item::Use(item) => {
                        self.collect_use_tree(&item.tree, module, &[]);
                    }
                    syn::Item::Mod(item) => {
                        if let Some((_, items)) = &item.content {
                            let mut child = module.to_vec();
                            child.push(item.ident.to_string());
                            self.collect_bindings(items, &child, skip_strict_test_only);
                        }
                    }
                    _ => {}
                }
            }
        }

        fn collect_use_tree(&mut self, tree: &syn::UseTree, module: &[String], prefix: &[String]) {
            match tree {
                syn::UseTree::Path(path) => {
                    let mut next = prefix.to_vec();
                    next.push(path.ident.to_string());
                    self.collect_use_tree(&path.tree, module, &next);
                }
                syn::UseTree::Name(name) => {
                    let mut source = prefix.to_vec();
                    source.push(name.ident.to_string());
                    let source = normalize_qualified_segments(&source, module).join("::");
                    self.bindings
                        .insert(qualified_name(module, &name.ident.to_string()), source);
                }
                syn::UseTree::Rename(rename) => {
                    let mut source = prefix.to_vec();
                    source.push(rename.ident.to_string());
                    let source = normalize_qualified_segments(&source, module).join("::");
                    self.bindings
                        .insert(qualified_name(module, &rename.rename.to_string()), source);
                }
                syn::UseTree::Group(group) => {
                    for tree in &group.items {
                        self.collect_use_tree(tree, module, prefix);
                    }
                }
                syn::UseTree::Glob(_) => {
                    let source = normalize_qualified_segments(prefix, module);
                    self.glob_imports.push((module.to_vec(), source));
                }
            }
        }

        fn expand_globs(&mut self) {
            loop {
                let names = self
                    .symbols
                    .iter()
                    .chain(self.bindings.keys())
                    .cloned()
                    .collect::<Vec<_>>();
                let mut changed = false;
                for (destination, source) in self.glob_imports.clone() {
                    let source_prefix = if source.is_empty() {
                        String::new()
                    } else {
                        format!("{}::", source.join("::"))
                    };
                    for name in &names {
                        let Some(rest) = name.strip_prefix(&source_prefix) else {
                            continue;
                        };
                        if rest.contains("::") {
                            continue;
                        }
                        let local = qualified_name(&destination, rest);
                        if let std::collections::btree_map::Entry::Vacant(entry) =
                            self.bindings.entry(local)
                        {
                            entry.insert(name.clone());
                            changed = true;
                        }
                    }
                }
                if !changed {
                    break;
                }
            }
        }

        fn path_resolves_to(&self, path: &syn::TypePath, module: &[String], target: &str) -> bool {
            let segments = syn_path_segments(&path.path);
            let mut candidate = normalize_qualified_segments(&segments, module).join("::");
            let mut seen = std::collections::BTreeSet::new();
            while seen.insert(candidate.clone()) {
                if candidate
                    .rsplit("::")
                    .next()
                    .is_some_and(|name| name == target)
                {
                    return true;
                }
                if let Some(next) = self.bindings.get(&candidate) {
                    candidate = next.clone();
                    continue;
                }
                let parts = candidate.split("::").collect::<Vec<_>>();
                let mut expanded = None;
                for prefix_len in (1..parts.len()).rev() {
                    let prefix = parts[..prefix_len].join("::");
                    if let Some(target_prefix) = self.bindings.get(&prefix) {
                        expanded = Some(format!(
                            "{}::{}",
                            target_prefix,
                            parts[prefix_len..].join("::")
                        ));
                        break;
                    }
                }
                let Some(next) = expanded else {
                    return false;
                };
                candidate = next;
            }
            false
        }
    }

    fn collect_use_aliases(
        tree: &syn::UseTree,
        aliases: &mut std::collections::BTreeMap<String, String>,
    ) {
        match tree {
            syn::UseTree::Path(path) => collect_use_aliases(&path.tree, aliases),
            syn::UseTree::Name(name) => {
                aliases.insert(name.ident.to_string(), name.ident.to_string());
            }
            syn::UseTree::Rename(rename) => {
                aliases.insert(rename.rename.to_string(), rename.ident.to_string());
            }
            syn::UseTree::Group(group) => {
                for item in &group.items {
                    collect_use_aliases(item, aliases);
                }
            }
            syn::UseTree::Glob(_) => {}
        }
    }

    fn collect_item_aliases(
        items: &[syn::Item],
        skip_strict_test_only: bool,
    ) -> std::collections::BTreeMap<String, String> {
        let mut aliases = std::collections::BTreeMap::new();
        for item in items {
            if skip_strict_test_only && item_attrs(item).is_some_and(is_test_only) {
                continue;
            }
            match item {
                syn::Item::Enum(item) => {
                    aliases.insert(item.ident.to_string(), item.ident.to_string());
                }
                syn::Item::Struct(item) => {
                    aliases.insert(item.ident.to_string(), item.ident.to_string());
                }
                syn::Item::Type(item) => {
                    if let Some(target) = type_last_name(&item.ty) {
                        aliases.insert(item.ident.to_string(), target);
                    }
                }
                syn::Item::Union(item) => {
                    aliases.insert(item.ident.to_string(), item.ident.to_string());
                }
                syn::Item::Use(item) => collect_use_aliases(&item.tree, &mut aliases),
                _ => {}
            }
        }
        aliases
    }

    fn collect_macro_aliases(
        tokens: proc_macro2::TokenStream,
        aliases: &mut std::collections::BTreeMap<String, String>,
    ) {
        let tokens = tokens.into_iter().collect::<Vec<_>>();
        for (start, token) in tokens.iter().enumerate() {
            let is_alias_item = matches!(token, proc_macro2::TokenTree::Ident(ident) if ident == "type" || ident == "use");
            if is_alias_item {
                for (end, candidate) in tokens.iter().enumerate().skip(start + 1) {
                    if !matches!(candidate, proc_macro2::TokenTree::Punct(punct) if punct.as_char() == ';')
                    {
                        continue;
                    }
                    let candidate = tokens[start..=end]
                        .iter()
                        .cloned()
                        .collect::<proc_macro2::TokenStream>();
                    if let Ok(item) = syn::parse2::<syn::ItemType>(candidate.clone()) {
                        if let Some(path) = type_path(&item.ty) {
                            aliases.insert(
                                item.ident.to_string(),
                                syn_path_segments(&path.path).join("::"),
                            );
                        }
                        break;
                    }
                    if let Ok(item) = syn::parse2::<syn::ItemUse>(candidate) {
                        collect_use_aliases(&item.tree, aliases);
                        break;
                    }
                }
            }
        }
    }

    fn normalize_macro_metavariables(tokens: proc_macro2::TokenStream) -> proc_macro2::TokenStream {
        let tokens = tokens.into_iter().collect::<Vec<_>>();
        let mut normalized = proc_macro2::TokenStream::new();
        let mut index = 0;
        while index < tokens.len() {
            if matches!(&tokens[index], proc_macro2::TokenTree::Punct(punct) if punct.as_char() == '$')
                && let Some(proc_macro2::TokenTree::Ident(ident)) = tokens.get(index + 1)
            {
                let name = ident.to_string();
                normalized.extend([proc_macro2::TokenTree::Ident(proc_macro2::Ident::new(
                    &format!("__plan04_macro_{}", name.trim_start_matches("r#")),
                    ident.span(),
                ))]);
                index += 2;
                continue;
            }
            match &tokens[index] {
                proc_macro2::TokenTree::Group(group) => {
                    let mut replacement = proc_macro2::Group::new(
                        group.delimiter(),
                        normalize_macro_metavariables(group.stream()),
                    );
                    replacement.set_span(group.span());
                    normalized.extend([proc_macro2::TokenTree::Group(replacement)]);
                }
                token => normalized.extend([token.clone()]),
            }
            index += 1;
        }
        normalized
    }

    fn macro_rules_transcribers(
        tokens: proc_macro2::TokenStream,
    ) -> Option<Vec<proc_macro2::TokenStream>> {
        let tokens = tokens.into_iter().collect::<Vec<_>>();
        let mut transcribers = Vec::new();
        let mut index = 0;
        while index < tokens.len() {
            while matches!(
                tokens.get(index),
                Some(proc_macro2::TokenTree::Punct(punct))
                    if punct.as_char() == ';' || punct.as_char() == ','
            ) {
                index += 1;
            }
            if index == tokens.len() {
                break;
            }
            if !matches!(tokens.get(index), Some(proc_macro2::TokenTree::Group(_)))
                || !matches!(
                    tokens.get(index + 1),
                    Some(proc_macro2::TokenTree::Punct(punct)) if punct.as_char() == '='
                )
                || !matches!(
                    tokens.get(index + 2),
                    Some(proc_macro2::TokenTree::Punct(punct)) if punct.as_char() == '>'
                )
            {
                return None;
            }
            let Some(proc_macro2::TokenTree::Group(transcriber)) = tokens.get(index + 3) else {
                return None;
            };
            transcribers.push(transcriber.stream());
            index += 4;
        }
        Some(transcribers)
    }

    fn production_macro_tokens(tokens: proc_macro2::TokenStream) -> proc_macro2::TokenStream {
        let tokens = tokens.into_iter().collect::<Vec<_>>();
        let mut production = proc_macro2::TokenStream::new();
        let mut index = 0;
        while index < tokens.len() {
            let macro_name = match tokens.get(index) {
                Some(proc_macro2::TokenTree::Ident(ident)) => Some(ident.to_string()),
                _ => None,
            };
            if macro_name
                .as_deref()
                .is_some_and(|name| name == "concat" || name == "stringify")
                && matches!(
                    tokens.get(index + 1),
                    Some(proc_macro2::TokenTree::Punct(punct)) if punct.as_char() == '!'
                )
                && matches!(
                    tokens.get(index + 2),
                    Some(proc_macro2::TokenTree::Group(_))
                )
            {
                index += 3;
                continue;
            }
            if macro_name.as_deref() == Some("macro_rules")
                && matches!(
                    tokens.get(index + 1),
                    Some(proc_macro2::TokenTree::Punct(punct)) if punct.as_char() == '!'
                )
                && let Some(proc_macro2::TokenTree::Group(body)) = tokens.get(index + 3)
            {
                let transcribers =
                    macro_rules_transcribers(body.stream()).unwrap_or_else(|| vec![body.stream()]);
                for transcriber in transcribers {
                    production.extend([proc_macro2::TokenTree::Group(proc_macro2::Group::new(
                        proc_macro2::Delimiter::Brace,
                        production_macro_tokens(transcriber),
                    ))]);
                }
                index += 4;
                continue;
            }
            match &tokens[index] {
                proc_macro2::TokenTree::Group(group) => {
                    let mut filtered = proc_macro2::Group::new(
                        group.delimiter(),
                        production_macro_tokens(group.stream()),
                    );
                    filtered.set_span(group.span());
                    production.extend([proc_macro2::TokenTree::Group(filtered)]);
                }
                token => production.extend([token.clone()]),
            }
            index += 1;
        }
        production
    }

    fn macro_production_streams(mac: &syn::Macro) -> Vec<proc_macro2::TokenStream> {
        if mac.path.is_ident("concat") || mac.path.is_ident("stringify") {
            return Vec::new();
        }
        if mac.path.is_ident("macro_rules") {
            return macro_rules_transcribers(mac.tokens.clone())
                .unwrap_or_else(|| vec![mac.tokens.clone()])
                .into_iter()
                .map(production_macro_tokens)
                .collect();
        }
        vec![production_macro_tokens(mac.tokens.clone())]
    }

    fn macro_tokens_contain_inherent_impl(
        tokens: proc_macro2::TokenStream,
        target: &str,
        outer_aliases: &std::collections::BTreeMap<String, String>,
        qualified_aliases: &QualifiedAliasGraph,
        module: &[String],
    ) -> bool {
        let tokens = normalize_macro_metavariables(tokens);
        macro_tokens_contain_inherent_impl_with_aliases(
            tokens,
            target,
            outer_aliases,
            qualified_aliases,
            module,
        )
    }

    fn macro_type_resolves_to(
        ty: &syn::Type,
        target: &str,
        aliases: &std::collections::BTreeMap<String, String>,
        qualified_aliases: &QualifiedAliasGraph,
        module: &[String],
    ) -> bool {
        let Some(path) = type_path(ty) else {
            return false;
        };
        if path
            .path
            .segments
            .last()
            .is_some_and(|segment| segment.ident == target)
        {
            return true;
        }
        let mut candidate = syn_path_segments(&path.path).join("::");
        let mut seen = std::collections::BTreeSet::new();
        while seen.insert(candidate.clone()) {
            if let Some(next) = aliases.get(&candidate) {
                candidate = next.clone();
                continue;
            }
            let parts = candidate.split("::").collect::<Vec<_>>();
            let mut expanded = None;
            for prefix_len in (1..parts.len()).rev() {
                let prefix = parts[..prefix_len].join("::");
                if let Some(target_prefix) = aliases.get(&prefix) {
                    expanded = Some(format!(
                        "{}::{}",
                        target_prefix,
                        parts[prefix_len..].join("::")
                    ));
                    break;
                }
            }
            let Some(next) = expanded else {
                break;
            };
            candidate = next;
        }
        syn::parse_str::<syn::Type>(&candidate)
            .ok()
            .and_then(|ty| type_path(&ty).cloned())
            .is_some_and(|path| qualified_aliases.path_resolves_to(&path, module, target))
    }

    fn macro_tokens_contain_inherent_impl_with_aliases(
        tokens: proc_macro2::TokenStream,
        target: &str,
        aliases: &std::collections::BTreeMap<String, String>,
        qualified_aliases: &QualifiedAliasGraph,
        module: &[String],
    ) -> bool {
        let mut scoped_aliases = aliases.clone();
        collect_macro_aliases(tokens.clone(), &mut scoped_aliases);
        let tokens = tokens.into_iter().collect::<Vec<_>>();
        for (start, token) in tokens.iter().enumerate() {
            if matches!(token, proc_macro2::TokenTree::Ident(ident) if ident == "impl") {
                for (end, candidate) in tokens.iter().enumerate().skip(start + 1) {
                    if !matches!(candidate, proc_macro2::TokenTree::Group(group) if group.delimiter() == proc_macro2::Delimiter::Brace)
                    {
                        continue;
                    }
                    let mut candidate = tokens[start..end]
                        .iter()
                        .cloned()
                        .collect::<proc_macro2::TokenStream>();
                    candidate.extend([proc_macro2::TokenTree::Group(proc_macro2::Group::new(
                        proc_macro2::Delimiter::Brace,
                        proc_macro2::TokenStream::new(),
                    ))]);
                    if let Ok(item) = syn::parse2::<syn::ItemImpl>(candidate)
                        && item.trait_.is_none()
                        && macro_type_resolves_to(
                            &item.self_ty,
                            target,
                            &scoped_aliases,
                            qualified_aliases,
                            module,
                        )
                    {
                        return true;
                    }
                }
            }
            if let proc_macro2::TokenTree::Group(group) = token
                && macro_tokens_contain_inherent_impl_with_aliases(
                    group.stream(),
                    target,
                    &scoped_aliases,
                    qualified_aliases,
                    module,
                )
            {
                return true;
            }
        }
        false
    }

    impl<'ast> syn::visit::Visit<'ast> for Plan04RetiredAuthorityFinder {
        fn visit_file(&mut self, file: &'ast syn::File) {
            if !self.qualified_aliases_initialized {
                self.qualified_aliases = QualifiedAliasGraph::from_file(
                    file,
                    &self.module_path,
                    self.skip_strict_test_only,
                );
                self.qualified_aliases_initialized = true;
            }
            self.alias_scopes.push(collect_item_aliases(
                &file.items,
                self.skip_strict_test_only,
            ));
            for item in &file.items {
                self.visit_item(item);
            }
            self.alias_scopes.pop();
        }

        fn visit_item(&mut self, item: &'ast syn::Item) {
            if self.skip_strict_test_only && item_attrs(item).is_some_and(is_test_only) {
                return;
            }
            syn::visit::visit_item(self, item);
        }

        fn visit_item_mod(&mut self, item: &'ast syn::ItemMod) {
            if let Some((_, items)) = &item.content {
                self.module_path.push(item.ident.to_string());
                self.alias_scopes
                    .push(collect_item_aliases(items, self.skip_strict_test_only));
                for item in items {
                    self.visit_item(item);
                }
                self.alias_scopes.pop();
                self.module_path.pop();
            }
        }

        fn visit_impl_item(&mut self, item: &'ast syn::ImplItem) {
            if self.skip_strict_test_only && impl_item_attrs(item).is_some_and(is_test_only) {
                return;
            }
            syn::visit::visit_impl_item(self, item);
        }

        fn visit_item_struct(&mut self, item: &'ast syn::ItemStruct) {
            match item.ident.to_string().as_str() {
                "Checked" => self.record("struct Checked"),
                "ConstructorBinding" => self.record("struct ConstructorBinding"),
                _ => {}
            }
            syn::visit::visit_item_struct(self, item);
        }

        fn visit_item_enum(&mut self, item: &'ast syn::ItemEnum) {
            if item.ident == "ConstructorArgument" {
                self.record("enum ConstructorArgument");
            }
            syn::visit::visit_item_enum(self, item);
        }

        fn visit_item_impl(&mut self, item: &'ast syn::ItemImpl) {
            if item.trait_.is_none() {
                if self.type_resolves_to(&item.self_ty, "Triggered") {
                    self.record("impl Triggered");
                }
                if self.type_resolves_to(&item.self_ty, "SelfReferenceNp") {
                    self.record("impl SelfReferenceNp");
                }
            }
            syn::visit::visit_item_impl(self, item);
        }

        fn visit_ident(&mut self, ident: &'ast syn::Ident) {
            if ident == "checked_constructor" {
                self.record("checked_constructor");
            }
            if ident == "checked_constructor_bindings" && !self.allow_report_schema_field {
                self.record("checked_constructor_bindings outside the schema-2 report");
            }
        }

        fn visit_macro(&mut self, mac: &'ast syn::Macro) {
            let production_streams = macro_production_streams(mac);
            if production_streams.iter().any(|tokens| {
                macro_tokens_contain_identifier(tokens.clone(), "checked_constructor")
            }) {
                self.record("checked_constructor");
            }
            for (target, authority) in [
                ("Triggered", "impl Triggered"),
                ("SelfReferenceNp", "impl SelfReferenceNp"),
            ] {
                if production_streams.iter().any(|tokens| {
                    macro_tokens_contain_inherent_impl(
                        tokens.clone(),
                        target,
                        &self.visible_aliases(),
                        &self.qualified_aliases,
                        &self.module_path,
                    )
                }) {
                    self.record(authority);
                }
            }
            for (kind, name, authority) in [
                (ProductionAuthorityKind::Struct, "Checked", "struct Checked"),
                (
                    ProductionAuthorityKind::Struct,
                    "ConstructorBinding",
                    "struct ConstructorBinding",
                ),
                (
                    ProductionAuthorityKind::Enum,
                    "ConstructorArgument",
                    "enum ConstructorArgument",
                ),
            ] {
                if production_streams
                    .iter()
                    .any(|tokens| macro_tokens_contain_declaration(tokens.clone(), kind, name))
                {
                    self.record(authority);
                }
            }
            syn::visit::visit_macro(self, mac);
        }
    }

    #[derive(Default)]
    struct CheckedBlockFinder {
        skip_strict_test_only: bool,
        empty_blocks: Vec<bool>,
        string_blocks: usize,
        exact_empty_string_blocks: usize,
        unresolved_compositions: usize,
    }

    struct CheckedBlockCensus {
        structural: Vec<bool>,
        strings: usize,
        exact_empty_strings: usize,
        unresolved_compositions: usize,
    }

    fn checked_text_block_pattern() -> regex::Regex {
        regex::Regex::new(r"(?s)\bchecked(?:\s|/\*.*?\*/|//[^\n]*(?:\n|$))*\{")
            .expect("checked-block text-fixture audit pattern is valid")
    }

    impl CheckedBlockFinder {
        fn record_string_fixture(&mut self, fixture: &str) {
            self.string_blocks += checked_text_block_pattern().find_iter(fixture).count();
            self.exact_empty_string_blocks += fixture.matches("checked {}").count();
        }
    }

    fn collect_checked_blocks(tokens: proc_macro2::TokenStream, empty_blocks: &mut Vec<bool>) {
        let tokens = tokens.into_iter().collect::<Vec<_>>();
        for (index, token) in tokens.iter().enumerate() {
            if matches!(token, proc_macro2::TokenTree::Ident(ident) if ident == "checked")
                && let Some(proc_macro2::TokenTree::Group(group)) = tokens.get(index + 1)
                && group.delimiter() == proc_macro2::Delimiter::Brace
            {
                empty_blocks.push(group.stream().is_empty());
            }
            if let proc_macro2::TokenTree::Group(group) = token {
                collect_checked_blocks(group.stream(), empty_blocks);
            }
        }
    }

    fn string_composition_could_contain_checked(tokens: proc_macro2::TokenStream) -> bool {
        fn collect_fragments(tokens: proc_macro2::TokenStream, fragments: &mut Vec<String>) {
            for token in tokens {
                match token {
                    proc_macro2::TokenTree::Group(group) => {
                        let delimiters = match group.delimiter() {
                            proc_macro2::Delimiter::Brace => Some(("{", "}")),
                            proc_macro2::Delimiter::Bracket => Some(("[", "]")),
                            proc_macro2::Delimiter::Parenthesis | proc_macro2::Delimiter::None => {
                                None
                            }
                        };
                        if let Some((open, _)) = delimiters {
                            fragments.push(open.to_owned());
                        }
                        collect_fragments(group.stream(), fragments);
                        if let Some((_, close)) = delimiters {
                            fragments.push(close.to_owned());
                        }
                    }
                    proc_macro2::TokenTree::Ident(ident) => {
                        fragments.push(ident.to_string());
                    }
                    proc_macro2::TokenTree::Literal(literal) => {
                        let spelling = syn::parse_str::<syn::Lit>(&literal.to_string())
                            .ok()
                            .and_then(|literal| literal_constant_string(&literal))
                            .unwrap_or_else(|| literal.to_string());
                        fragments.push(spelling);
                    }
                    proc_macro2::TokenTree::Punct(_) => {}
                }
            }
        }

        let mut fragments = Vec::new();
        collect_fragments(tokens, &mut fragments);
        (0..fragments.len()).any(|start| {
            let mut candidate = String::new();
            fragments[start..].iter().any(|fragment| {
                candidate.push_str(fragment);
                checked_text_block_pattern().is_match(&candidate)
            })
        })
    }

    fn collect_macro_fixture_strings(
        tokens: proc_macro2::TokenStream,
        fixtures: &mut Vec<String>,
        unresolved_compositions: &mut usize,
    ) {
        let tokens = tokens.into_iter().collect::<Vec<_>>();
        let mut index = 0;
        while index < tokens.len() {
            if matches!(&tokens[index], proc_macro2::TokenTree::Ident(ident) if ident == "macro_rules")
                && matches!(
                    tokens.get(index + 1),
                    Some(proc_macro2::TokenTree::Punct(punct)) if punct.as_char() == '!'
                )
                && let Some(proc_macro2::TokenTree::Group(body)) = tokens.get(index + 3)
            {
                let transcribers =
                    macro_rules_transcribers(body.stream()).unwrap_or_else(|| vec![body.stream()]);
                for transcriber in transcribers {
                    collect_macro_fixture_strings(transcriber, fixtures, unresolved_compositions);
                }
                index += 4;
                continue;
            }
            let string_macro = match &tokens[index] {
                proc_macro2::TokenTree::Ident(ident) if ident == "concat" => Some("concat"),
                proc_macro2::TokenTree::Ident(ident) if ident == "stringify" => Some("stringify"),
                _ => None,
            };
            if let Some(string_macro) = string_macro
                && matches!(
                    tokens.get(index + 1),
                    Some(proc_macro2::TokenTree::Punct(punct)) if punct.as_char() == '!'
                )
                && let Some(proc_macro2::TokenTree::Group(group)) = tokens.get(index + 2)
            {
                let fixture = if string_macro == "concat" {
                    constant_concat_tokens(group.stream())
                } else {
                    Some(group.stream().to_string())
                };
                if let Some(fixture) = fixture {
                    fixtures.push(fixture);
                } else {
                    if string_composition_could_contain_checked(group.stream()) {
                        *unresolved_compositions += 1;
                    }
                    collect_macro_fixture_strings(
                        group.stream(),
                        fixtures,
                        unresolved_compositions,
                    );
                }
                index += 3;
                continue;
            }
            match &tokens[index] {
                proc_macro2::TokenTree::Literal(literal) => {
                    if let Ok(literal) = syn::parse_str::<syn::Lit>(&literal.to_string())
                        && let Some(fixture) = literal_constant_string(&literal)
                    {
                        fixtures.push(fixture);
                    }
                }
                proc_macro2::TokenTree::Group(group) => {
                    collect_macro_fixture_strings(
                        group.stream(),
                        fixtures,
                        unresolved_compositions,
                    );
                }
                proc_macro2::TokenTree::Ident(_) | proc_macro2::TokenTree::Punct(_) => {}
            }
            index += 1;
        }
    }

    impl<'ast> syn::visit::Visit<'ast> for CheckedBlockFinder {
        fn visit_item(&mut self, item: &'ast syn::Item) {
            if self.skip_strict_test_only && item_attrs(item).is_some_and(is_test_only) {
                return;
            }
            syn::visit::visit_item(self, item);
        }

        fn visit_impl_item(&mut self, item: &'ast syn::ImplItem) {
            if self.skip_strict_test_only && impl_item_attrs(item).is_some_and(is_test_only) {
                return;
            }
            syn::visit::visit_impl_item(self, item);
        }

        fn visit_lit_str(&mut self, literal: &'ast syn::LitStr) {
            self.record_string_fixture(&literal.value());
        }

        fn visit_lit_byte_str(&mut self, literal: &'ast syn::LitByteStr) {
            self.record_string_fixture(&String::from_utf8_lossy(&literal.value()));
        }

        fn visit_macro(&mut self, mac: &'ast syn::Macro) {
            for tokens in macro_production_streams(mac) {
                collect_checked_blocks(tokens, &mut self.empty_blocks);
            }
            let mut fixtures = Vec::new();
            if mac.path.is_ident("concat") {
                if let Some(fixture) = constant_concat_tokens(mac.tokens.clone()) {
                    fixtures.push(fixture);
                } else {
                    if string_composition_could_contain_checked(mac.tokens.clone()) {
                        self.unresolved_compositions += 1;
                    }
                    collect_macro_fixture_strings(
                        mac.tokens.clone(),
                        &mut fixtures,
                        &mut self.unresolved_compositions,
                    );
                }
            } else if mac.path.is_ident("stringify") {
                fixtures.push(mac.tokens.to_string());
            } else if mac.path.is_ident("macro_rules") {
                for transcriber in macro_rules_transcribers(mac.tokens.clone())
                    .unwrap_or_else(|| vec![mac.tokens.clone()])
                {
                    collect_macro_fixture_strings(
                        transcriber,
                        &mut fixtures,
                        &mut self.unresolved_compositions,
                    );
                }
            } else {
                collect_macro_fixture_strings(
                    mac.tokens.clone(),
                    &mut fixtures,
                    &mut self.unresolved_compositions,
                );
            }
            for fixture in fixtures {
                self.record_string_fixture(&fixture);
            }
        }
    }

    fn checked_block_census(file: &syn::File, skip_strict_test_only: bool) -> CheckedBlockCensus {
        use syn::visit::Visit as _;

        let mut finder = CheckedBlockFinder {
            skip_strict_test_only,
            ..CheckedBlockFinder::default()
        };
        finder.visit_file(file);
        CheckedBlockCensus {
            structural: finder.empty_blocks,
            strings: finder.string_blocks,
            exact_empty_strings: finder.exact_empty_string_blocks,
            unresolved_compositions: finder.unresolved_compositions,
        }
    }

    fn checked_block_census_in_rust_source(source: &str) -> CheckedBlockCensus {
        let file = syn::parse_file(source).expect("checked-block source reparses");
        checked_block_census(&file, false)
    }

    fn checked_blocks_in_rust_source(source: &str) -> Vec<bool> {
        checked_block_census_in_rust_source(source).structural
    }

    #[derive(Default)]
    struct CheckedBindingsProducerFinder {
        violations: Vec<String>,
    }

    fn member_is_checked_bindings(member: &syn::Member) -> bool {
        matches!(member, syn::Member::Named(name) if name == "checked_constructor_bindings")
    }

    fn expression_is_checked_bindings_field(expression: &syn::Expr) -> bool {
        matches!(
            expression,
            syn::Expr::Field(field) if member_is_checked_bindings(&field.member)
        )
    }

    fn expression_is_empty_vector(expression: &syn::Expr) -> bool {
        matches!(
            expression,
            syn::Expr::Macro(expression)
                if expression.mac.path.is_ident("vec") && expression.mac.tokens.is_empty()
        )
    }

    impl<'ast> syn::visit::Visit<'ast> for CheckedBindingsProducerFinder {
        fn visit_expr_struct(&mut self, expression: &'ast syn::ExprStruct) {
            for field in &expression.fields {
                if member_is_checked_bindings(&field.member)
                    && !expression_is_empty_vector(&field.expr)
                {
                    self.violations.push(
                        "checked_constructor_bindings has a nonempty or indirect producer"
                            .to_owned(),
                    );
                }
            }
            syn::visit::visit_expr_struct(self, expression);
        }

        fn visit_expr_reference(&mut self, expression: &'ast syn::ExprReference) {
            if expression_is_checked_bindings_field(&expression.expr) {
                if expression.mutability.is_some() {
                    self.violations.push(
                        "checked_constructor_bindings escapes through a mutable reference"
                            .to_owned(),
                    );
                }
                return;
            }
            syn::visit::visit_expr_reference(self, expression);
        }

        fn visit_expr_method_call(&mut self, expression: &'ast syn::ExprMethodCall) {
            if expression_is_checked_bindings_field(&expression.receiver) {
                if !matches!(
                    expression.method.to_string().as_str(),
                    "as_slice" | "is_empty" | "iter" | "len"
                ) {
                    self.violations.push(format!(
                        "checked_constructor_bindings invokes unproved method {}",
                        expression.method
                    ));
                }
                for argument in &expression.args {
                    self.visit_expr(argument);
                }
                return;
            }
            syn::visit::visit_expr_method_call(self, expression);
        }

        fn visit_expr_field(&mut self, expression: &'ast syn::ExprField) {
            if member_is_checked_bindings(&expression.member) {
                self.violations
                    .push("checked_constructor_bindings is moved or otherwise exposed".to_owned());
                return;
            }
            syn::visit::visit_expr_field(self, expression);
        }

        fn visit_pat_struct(&mut self, pattern: &'ast syn::PatStruct) {
            if pattern
                .fields
                .iter()
                .any(|field| member_is_checked_bindings(&field.member))
            {
                self.violations.push(
                    "checked_constructor_bindings is exposed through destructuring".to_owned(),
                );
            }
            syn::visit::visit_pat_struct(self, pattern);
        }

        fn visit_macro(&mut self, mac: &'ast syn::Macro) {
            use syn::parse::Parser as _;

            let expressions =
                syn::punctuated::Punctuated::<syn::Expr, syn::Token![,]>::parse_terminated
                    .parse2(mac.tokens.clone());
            if let Ok(expressions) = expressions {
                for expression in expressions {
                    self.visit_expr(&expression);
                }
            } else if macro_tokens_contain_identifier(
                mac.tokens.clone(),
                "checked_constructor_bindings",
            ) {
                self.violations.push(
                    "checked_constructor_bindings appears inside unproved macro syntax".to_owned(),
                );
            }
        }
    }

    #[derive(Clone, Copy, PartialEq, Eq)]
    enum ProductionAuthorityKind {
        Enum,
        Struct,
        TypeAlias,
        Function,
    }

    struct ProductionAuthorityFinder<'a> {
        kind: ProductionAuthorityKind,
        target: &'a str,
        found: bool,
    }

    struct ProductionIdentifierFinder<'a> {
        target: &'a str,
        found: bool,
    }

    struct ProductionMethodCallFinder<'a> {
        target: &'a str,
        found: bool,
    }

    #[derive(Default)]
    struct ProductionStringLiteralFinder {
        found: Vec<String>,
    }

    #[derive(Default)]
    struct TestOnlyGeneratedGhostFinder {
        in_test_only: bool,
        found: bool,
    }

    #[derive(Default)]
    struct ProductionCodeIndirectionFinder {
        found: bool,
    }

    struct ClosedLexemeSurfaceMirrorFinder<'a> {
        declarations: &'a ClosedLexemeDeclarations,
        found: bool,
    }

    struct ClosedLexemeIdentifierFinder<'a> {
        declarations: &'a ClosedLexemeDeclarations,
        found: bool,
    }

    fn item_attrs(item: &syn::Item) -> Option<&[syn::Attribute]> {
        match item {
            syn::Item::Const(item) => Some(&item.attrs),
            syn::Item::Enum(item) => Some(&item.attrs),
            syn::Item::ExternCrate(item) => Some(&item.attrs),
            syn::Item::Fn(item) => Some(&item.attrs),
            syn::Item::ForeignMod(item) => Some(&item.attrs),
            syn::Item::Impl(item) => Some(&item.attrs),
            syn::Item::Macro(item) => Some(&item.attrs),
            syn::Item::Mod(item) => Some(&item.attrs),
            syn::Item::Static(item) => Some(&item.attrs),
            syn::Item::Struct(item) => Some(&item.attrs),
            syn::Item::Trait(item) => Some(&item.attrs),
            syn::Item::TraitAlias(item) => Some(&item.attrs),
            syn::Item::Type(item) => Some(&item.attrs),
            syn::Item::Union(item) => Some(&item.attrs),
            syn::Item::Use(item) => Some(&item.attrs),
            _ => None,
        }
    }

    fn impl_item_attrs(item: &syn::ImplItem) -> Option<&[syn::Attribute]> {
        match item {
            syn::ImplItem::Const(item) => Some(&item.attrs),
            syn::ImplItem::Fn(item) => Some(&item.attrs),
            syn::ImplItem::Macro(item) => Some(&item.attrs),
            syn::ImplItem::Type(item) => Some(&item.attrs),
            _ => None,
        }
    }

    fn macro_tokens_contain_identifier(tokens: proc_macro2::TokenStream, target: &str) -> bool {
        tokens.into_iter().any(|token| match token {
            proc_macro2::TokenTree::Ident(ident) => ident == target,
            proc_macro2::TokenTree::Group(group) => {
                macro_tokens_contain_identifier(group.stream(), target)
            }
            proc_macro2::TokenTree::Punct(_) | proc_macro2::TokenTree::Literal(_) => false,
        })
    }

    fn macro_tokens_contain_code_indirection(tokens: proc_macro2::TokenStream) -> bool {
        use proc_macro2::Delimiter;
        use proc_macro2::TokenTree;

        let tokens = tokens.into_iter().collect::<Vec<_>>();
        for (index, token) in tokens.iter().enumerate() {
            if let TokenTree::Group(group) = token
                && macro_tokens_contain_code_indirection(group.stream())
            {
                return true;
            }
            if matches!(token, TokenTree::Ident(ident) if ident == "include")
                && matches!(
                    tokens.get(index + 1),
                    Some(TokenTree::Punct(punct)) if punct.as_char() == '!'
                )
                && matches!(tokens.get(index + 2), Some(TokenTree::Group(_)))
            {
                return true;
            }
            if matches!(token, TokenTree::Punct(punct) if punct.as_char() == '#')
                && let Some(TokenTree::Group(attribute)) = tokens.get(index + 1)
                && attribute.delimiter() == Delimiter::Bracket
                && syn::parse2::<syn::Meta>(attribute.stream()).is_ok_and(
                    |meta| matches!(meta, syn::Meta::NameValue(meta) if meta.path.is_ident("path")),
                )
            {
                return true;
            }
        }
        false
    }

    fn macro_tokens_contain_declaration(
        tokens: proc_macro2::TokenStream,
        kind: ProductionAuthorityKind,
        target: &str,
    ) -> bool {
        let keyword = match kind {
            ProductionAuthorityKind::Enum => "enum",
            ProductionAuthorityKind::Struct => "struct",
            ProductionAuthorityKind::TypeAlias => "type",
            ProductionAuthorityKind::Function => "fn",
        };
        let tokens = tokens.into_iter().collect::<Vec<_>>();
        for (index, token) in tokens.iter().enumerate() {
            if let proc_macro2::TokenTree::Group(group) = token
                && macro_tokens_contain_declaration(group.stream(), kind, target)
            {
                return true;
            }
            if matches!(token, proc_macro2::TokenTree::Ident(ident) if ident == keyword)
                && matches!(
                    tokens.get(index + 1),
                    Some(proc_macro2::TokenTree::Ident(ident)) if ident == target
                )
            {
                return true;
            }
        }
        false
    }

    fn type_contains_generated_reference(ty: &syn::Type) -> bool {
        use syn::visit::Visit as _;

        [
            "build",
            "RULES",
            "RuleId",
            "Lexical",
            "Leaf",
            "TerminalClass",
            "NounNumber",
        ]
        .iter()
        .any(|target| {
            let mut finder = ProductionIdentifierFinder {
                target,
                found: false,
            };
            finder.visit_type(ty);
            finder.found
        })
    }

    fn literal_constant_string(literal: &syn::Lit) -> Option<String> {
        match literal {
            syn::Lit::Str(literal) => Some(literal.value()),
            syn::Lit::ByteStr(literal) => String::from_utf8(literal.value()).ok(),
            _ => None,
        }
    }

    fn constant_string_expression(expression: &syn::Expr) -> Option<String> {
        match expression {
            syn::Expr::Lit(expression) => literal_constant_string(&expression.lit),
            syn::Expr::Macro(expression) if expression.mac.path.is_ident("concat") => {
                constant_concat_tokens(expression.mac.tokens.clone())
            }
            syn::Expr::Macro(expression) if expression.mac.path.is_ident("stringify") => {
                Some(expression.mac.tokens.to_string())
            }
            syn::Expr::Group(expression) => constant_string_expression(&expression.expr),
            syn::Expr::Paren(expression) => constant_string_expression(&expression.expr),
            _ => None,
        }
    }

    fn constant_concat_tokens(tokens: proc_macro2::TokenStream) -> Option<String> {
        use syn::parse::Parser as _;
        use syn::punctuated::Punctuated;

        let expressions = Punctuated::<syn::Expr, syn::Token![,]>::parse_terminated
            .parse2(tokens)
            .ok()?;
        let mut result = String::new();
        for expression in expressions {
            result.push_str(&constant_string_expression(&expression)?);
        }
        Some(result)
    }

    fn collect_macro_string_literals(tokens: proc_macro2::TokenStream, found: &mut Vec<String>) {
        let tokens = tokens.into_iter().collect::<Vec<_>>();
        for (index, token) in tokens.iter().enumerate() {
            if matches!(token, proc_macro2::TokenTree::Ident(ident) if ident == "concat")
                && matches!(
                    tokens.get(index + 1),
                    Some(proc_macro2::TokenTree::Punct(punct)) if punct.as_char() == '!'
                )
                && let Some(proc_macro2::TokenTree::Group(group)) = tokens.get(index + 2)
                && let Some(spelling) = constant_concat_tokens(group.stream())
            {
                found.push(spelling);
            }
            match token {
                proc_macro2::TokenTree::Literal(literal) => {
                    if let Ok(literal) = syn::parse_str::<syn::Lit>(&literal.to_string())
                        && let Some(spelling) = literal_constant_string(&literal)
                    {
                        found.push(spelling);
                    }
                }
                proc_macro2::TokenTree::Group(group) => {
                    collect_macro_string_literals(group.stream(), found);
                }
                proc_macro2::TokenTree::Ident(_) | proc_macro2::TokenTree::Punct(_) => {}
            }
        }
    }

    fn macro_arguments_contain_closed_surface_mirror(
        tokens: proc_macro2::TokenStream,
        closed_lexemes: &ClosedLexemeDeclarations,
    ) -> bool {
        use syn::parse::Parser as _;
        use syn::punctuated::Punctuated;
        use syn::visit::Visit as _;

        Punctuated::<syn::Expr, syn::Token![,]>::parse_terminated
            .parse2(tokens)
            .is_ok_and(|arguments| {
                arguments.into_iter().any(|expression| {
                    let mut finder = ClosedLexemeSurfaceMirrorFinder {
                        declarations: closed_lexemes,
                        found: false,
                    };
                    finder.visit_expr(&expression);
                    finder.found
                })
            })
    }

    fn macro_tokens_contain_closed_surface_mirror(
        tokens: proc_macro2::TokenStream,
        closed_lexemes: &ClosedLexemeDeclarations,
    ) -> bool {
        if macro_arguments_contain_closed_surface_mirror(tokens.clone(), closed_lexemes) {
            return true;
        }
        tokens.into_iter().any(|token| match token {
            proc_macro2::TokenTree::Group(group) => {
                let delimited =
                    proc_macro2::TokenStream::from(proc_macro2::TokenTree::Group(group.clone()));
                macro_arguments_contain_closed_surface_mirror(delimited, closed_lexemes)
                    || macro_tokens_contain_closed_surface_mirror(group.stream(), closed_lexemes)
            }
            proc_macro2::TokenTree::Ident(_)
            | proc_macro2::TokenTree::Punct(_)
            | proc_macro2::TokenTree::Literal(_) => false,
        })
    }

    fn use_tree_reexports_generated_item(tree: &syn::UseTree) -> bool {
        const GENERATED_ITEMS: &[&str] = &[
            "build",
            "RULES",
            "RuleId",
            "Lexical",
            "Leaf",
            "TerminalClass",
            "NounNumber",
        ];

        match tree {
            syn::UseTree::Path(path) => use_tree_reexports_generated_item(&path.tree),
            syn::UseTree::Name(name) => GENERATED_ITEMS.contains(&name.ident.to_string().as_str()),
            syn::UseTree::Rename(rename) => {
                GENERATED_ITEMS.contains(&rename.ident.to_string().as_str())
                    || GENERATED_ITEMS.contains(&rename.rename.to_string().as_str())
            }
            syn::UseTree::Glob(_) => true,
            syn::UseTree::Group(group) => group.items.iter().any(use_tree_reexports_generated_item),
        }
    }

    impl<'ast> syn::visit::Visit<'ast> for ProductionMethodCallFinder<'_> {
        fn visit_expr_method_call(&mut self, call: &'ast syn::ExprMethodCall) {
            if call.method == self.target {
                self.found = true;
            }
            syn::visit::visit_expr_method_call(self, call);
        }

        fn visit_item_fn(&mut self, item: &'ast syn::ItemFn) {
            if !is_test_only(&item.attrs) {
                syn::visit::visit_item_fn(self, item);
            }
        }

        fn visit_item_impl(&mut self, item: &'ast syn::ItemImpl) {
            if !is_test_only(&item.attrs) {
                syn::visit::visit_item_impl(self, item);
            }
        }

        fn visit_item_mod(&mut self, item: &'ast syn::ItemMod) {
            if !is_test_only(&item.attrs) {
                syn::visit::visit_item_mod(self, item);
            }
        }
    }

    impl<'ast> syn::visit::Visit<'ast> for ProductionStringLiteralFinder {
        fn visit_item(&mut self, item: &'ast syn::Item) {
            if item_attrs(item).is_some_and(is_test_only) {
                return;
            }
            syn::visit::visit_item(self, item);
        }

        fn visit_impl_item(&mut self, item: &'ast syn::ImplItem) {
            if impl_item_attrs(item).is_some_and(is_test_only) {
                return;
            }
            syn::visit::visit_impl_item(self, item);
        }

        fn visit_lit_str(&mut self, literal: &'ast syn::LitStr) {
            self.found.push(literal.value());
        }

        fn visit_lit_byte_str(&mut self, literal: &'ast syn::LitByteStr) {
            if let Ok(spelling) = String::from_utf8(literal.value()) {
                self.found.push(spelling);
            }
        }

        fn visit_macro(&mut self, mac: &'ast syn::Macro) {
            if mac.path.is_ident("concat")
                && let Some(spelling) = constant_concat_tokens(mac.tokens.clone())
            {
                self.found.push(spelling);
            }
            collect_macro_string_literals(mac.tokens.clone(), &mut self.found);
        }

        fn visit_item_fn(&mut self, item: &'ast syn::ItemFn) {
            if !is_test_only(&item.attrs) {
                syn::visit::visit_item_fn(self, item);
            }
        }

        fn visit_item_impl(&mut self, item: &'ast syn::ItemImpl) {
            if !is_test_only(&item.attrs) {
                syn::visit::visit_item_impl(self, item);
            }
        }

        fn visit_item_mod(&mut self, item: &'ast syn::ItemMod) {
            if !is_test_only(&item.attrs) {
                syn::visit::visit_item_mod(self, item);
            }
        }
    }

    impl<'ast> syn::visit::Visit<'ast> for TestOnlyGeneratedGhostFinder {
        fn visit_item_mod(&mut self, item: &'ast syn::ItemMod) {
            let previous = self.in_test_only;
            self.in_test_only |= is_test_only(&item.attrs);
            if self.in_test_only && matches!(item.ident.to_string().as_str(), "build" | "rules") {
                self.found = true;
            }
            syn::visit::visit_item_mod(self, item);
            self.in_test_only = previous;
        }

        fn visit_item_type(&mut self, item: &'ast syn::ItemType) {
            let test_only = self.in_test_only || is_test_only(&item.attrs);
            if test_only
                && (matches!(
                    item.ident.to_string().as_str(),
                    "Lexical" | "Leaf" | "TerminalClass" | "NounNumber"
                ) || type_contains_generated_reference(&item.ty))
            {
                self.found = true;
            }
            syn::visit::visit_item_type(self, item);
        }

        fn visit_item_use(&mut self, item: &'ast syn::ItemUse) {
            let test_only = self.in_test_only || is_test_only(&item.attrs);
            if test_only
                && !matches!(item.vis, syn::Visibility::Inherited)
                && use_tree_reexports_generated_item(&item.tree)
            {
                self.found = true;
            }
            syn::visit::visit_item_use(self, item);
        }
    }

    impl<'ast> syn::visit::Visit<'ast> for ProductionCodeIndirectionFinder {
        fn visit_item(&mut self, item: &'ast syn::Item) {
            if item_attrs(item).is_some_and(is_test_only) {
                return;
            }
            syn::visit::visit_item(self, item);
        }

        fn visit_impl_item(&mut self, item: &'ast syn::ImplItem) {
            if impl_item_attrs(item).is_some_and(is_test_only) {
                return;
            }
            syn::visit::visit_impl_item(self, item);
        }

        fn visit_item_mod(&mut self, item: &'ast syn::ItemMod) {
            if item.attrs.iter().any(|attr| attr.path().is_ident("path")) {
                self.found = true;
            }
            syn::visit::visit_item_mod(self, item);
        }

        fn visit_macro(&mut self, mac: &'ast syn::Macro) {
            if mac.path.is_ident("include")
                || macro_tokens_contain_code_indirection(mac.tokens.clone())
            {
                self.found = true;
            }
            syn::visit::visit_macro(self, mac);
        }
    }

    impl<'ast> syn::visit::Visit<'ast> for ClosedLexemeSurfaceMirrorFinder<'_> {
        fn visit_item(&mut self, item: &'ast syn::Item) {
            if item_attrs(item).is_some_and(is_test_only) {
                return;
            }
            syn::visit::visit_item(self, item);
        }

        fn visit_impl_item(&mut self, item: &'ast syn::ImplItem) {
            if impl_item_attrs(item).is_some_and(is_test_only) {
                return;
            }
            syn::visit::visit_impl_item(self, item);
        }

        fn visit_expr_array(&mut self, expression: &'ast syn::ExprArray) {
            let mut lexeme = ClosedLexemeIdentifierFinder {
                declarations: self.declarations,
                found: false,
            };
            lexeme.visit_expr_array(expression);
            let mut strings = ProductionStringLiteralFinder::default();
            strings.visit_expr_array(expression);
            if lexeme.found && !strings.found.is_empty() {
                self.found = true;
            }
            syn::visit::visit_expr_array(self, expression);
        }

        fn visit_expr_match(&mut self, expression: &'ast syn::ExprMatch) {
            let mut lexeme = ClosedLexemeIdentifierFinder {
                declarations: self.declarations,
                found: false,
            };
            lexeme.visit_expr_match(expression);
            let mut strings = ProductionStringLiteralFinder::default();
            strings.visit_expr_match(expression);
            if lexeme.found && !strings.found.is_empty() {
                self.found = true;
            }
            syn::visit::visit_expr_match(self, expression);
        }

        fn visit_macro(&mut self, mac: &'ast syn::Macro) {
            if macro_tokens_contain_closed_surface_mirror(mac.tokens.clone(), self.declarations) {
                self.found = true;
            }
        }
    }

    impl<'ast> syn::visit::Visit<'ast> for ClosedLexemeIdentifierFinder<'_> {
        fn visit_ident(&mut self, ident: &'ast syn::Ident) {
            let ident = ident.to_string();
            self.found |= self.declarations.contains(&ident);
        }
    }

    impl<'ast> syn::visit::Visit<'ast> for ProductionIdentifierFinder<'_> {
        fn visit_ident(&mut self, ident: &'ast syn::Ident) {
            if ident == self.target {
                self.found = true;
            }
        }

        fn visit_macro(&mut self, mac: &'ast syn::Macro) {
            if macro_tokens_contain_identifier(mac.tokens.clone(), self.target) {
                self.found = true;
            }
        }

        fn visit_item_fn(&mut self, item: &'ast syn::ItemFn) {
            if !is_test_only(&item.attrs) {
                syn::visit::visit_item_fn(self, item);
            }
        }

        fn visit_item_enum(&mut self, item: &'ast syn::ItemEnum) {
            if !is_test_only(&item.attrs) {
                syn::visit::visit_item_enum(self, item);
            }
        }

        fn visit_item_struct(&mut self, item: &'ast syn::ItemStruct) {
            if !is_test_only(&item.attrs) {
                syn::visit::visit_item_struct(self, item);
            }
        }

        fn visit_item_type(&mut self, item: &'ast syn::ItemType) {
            if !is_test_only(&item.attrs) {
                syn::visit::visit_item_type(self, item);
            }
        }

        fn visit_item_impl(&mut self, item: &'ast syn::ItemImpl) {
            if !is_test_only(&item.attrs) {
                syn::visit::visit_item_impl(self, item);
            }
        }

        fn visit_item_mod(&mut self, item: &'ast syn::ItemMod) {
            if !is_test_only(&item.attrs) {
                syn::visit::visit_item_mod(self, item);
            }
        }
    }

    impl<'ast> syn::visit::Visit<'ast> for ProductionAuthorityFinder<'_> {
        fn visit_macro(&mut self, mac: &'ast syn::Macro) {
            if (self.kind == ProductionAuthorityKind::Function
                && macro_tokens_contain_identifier(mac.tokens.clone(), self.target))
                || (self.kind != ProductionAuthorityKind::Function
                    && macro_tokens_contain_declaration(mac.tokens.clone(), self.kind, self.target))
            {
                self.found = true;
            }
        }

        fn visit_item_macro(&mut self, item: &'ast syn::ItemMacro) {
            if !is_test_only(&item.attrs) {
                syn::visit::visit_item_macro(self, item);
            }
        }

        fn visit_item_enum(&mut self, item: &'ast syn::ItemEnum) {
            if !is_test_only(&item.attrs)
                && self.kind == ProductionAuthorityKind::Enum
                && item.ident == self.target
            {
                self.found = true;
            }
        }

        fn visit_item_struct(&mut self, item: &'ast syn::ItemStruct) {
            if !is_test_only(&item.attrs)
                && self.kind == ProductionAuthorityKind::Struct
                && item.ident == self.target
            {
                self.found = true;
            }
        }

        fn visit_item_type(&mut self, item: &'ast syn::ItemType) {
            if !is_test_only(&item.attrs)
                && self.kind == ProductionAuthorityKind::TypeAlias
                && item.ident == self.target
            {
                self.found = true;
            }
        }

        fn visit_item_fn(&mut self, item: &'ast syn::ItemFn) {
            if !is_test_only(&item.attrs) {
                if self.kind == ProductionAuthorityKind::Function && item.sig.ident == self.target {
                    self.found = true;
                }
                syn::visit::visit_item_fn(self, item);
            }
        }

        fn visit_impl_item_fn(&mut self, item: &'ast syn::ImplItemFn) {
            if !is_test_only(&item.attrs) {
                if self.kind == ProductionAuthorityKind::Function && item.sig.ident == self.target {
                    self.found = true;
                }
                syn::visit::visit_impl_item_fn(self, item);
            }
        }

        fn visit_item_impl(&mut self, item: &'ast syn::ItemImpl) {
            if !is_test_only(&item.attrs) {
                syn::visit::visit_item_impl(self, item);
            }
        }

        fn visit_item_mod(&mut self, item: &'ast syn::ItemMod) {
            if !is_test_only(&item.attrs) {
                syn::visit::visit_item_mod(self, item);
            }
        }
    }

    #[derive(Clone, Copy)]
    struct BooleanPossibility {
        can_be_false: bool,
        can_be_true: bool,
    }

    impl BooleanPossibility {
        const FALSE: Self = Self {
            can_be_false: true,
            can_be_true: false,
        };
        const TRUE: Self = Self {
            can_be_false: false,
            can_be_true: true,
        };
        const UNKNOWN: Self = Self {
            can_be_false: true,
            can_be_true: true,
        };

        fn and(self, other: Self) -> Self {
            Self {
                can_be_false: self.can_be_false || other.can_be_false,
                can_be_true: self.can_be_true && other.can_be_true,
            }
        }

        fn or(self, other: Self) -> Self {
            Self {
                can_be_false: self.can_be_false && other.can_be_false,
                can_be_true: self.can_be_true || other.can_be_true,
            }
        }

        fn not(self) -> Self {
            Self {
                can_be_false: self.can_be_true,
                can_be_true: self.can_be_false,
            }
        }
    }

    fn cfg_possibility_when_not_testing(meta: &syn::Meta) -> BooleanPossibility {
        use syn::parse::Parser as _;
        use syn::punctuated::Punctuated;

        match meta {
            syn::Meta::Path(path) if path.is_ident("test") => BooleanPossibility::FALSE,
            syn::Meta::Path(_) | syn::Meta::NameValue(_) => BooleanPossibility::UNKNOWN,
            syn::Meta::List(list) => {
                let nested = Punctuated::<syn::Meta, syn::Token![,]>::parse_terminated
                    .parse2(list.tokens.clone());
                let Ok(nested) = nested else {
                    return BooleanPossibility::UNKNOWN;
                };
                if list.path.is_ident("all") {
                    nested
                        .iter()
                        .fold(BooleanPossibility::TRUE, |result, meta| {
                            result.and(cfg_possibility_when_not_testing(meta))
                        })
                } else if list.path.is_ident("any") {
                    nested
                        .iter()
                        .fold(BooleanPossibility::FALSE, |result, meta| {
                            result.or(cfg_possibility_when_not_testing(meta))
                        })
                } else if list.path.is_ident("not") && nested.len() == 1 {
                    cfg_possibility_when_not_testing(&nested[0]).not()
                } else {
                    BooleanPossibility::UNKNOWN
                }
            }
        }
    }

    fn is_test_only(attrs: &[syn::Attribute]) -> bool {
        let mut saw_cfg = false;
        let mut combined = BooleanPossibility::TRUE;
        for attribute in attrs {
            let syn::Meta::List(cfg) = &attribute.meta else {
                continue;
            };
            if !cfg.path.is_ident("cfg") {
                continue;
            }
            saw_cfg = true;
            let possibility = syn::parse2::<syn::Meta>(cfg.tokens.clone())
                .map_or(BooleanPossibility::UNKNOWN, |meta| {
                    cfg_possibility_when_not_testing(&meta)
                });
            combined = combined.and(possibility);
        }
        saw_cfg && !combined.can_be_true
    }

    const ALL_DECLARATION_ORIGINS: &[&str] = &[
        "vocab TriggerWord",
        "vocab Article",
        "vocab Demonstrative",
        "vocab Pronoun",
        "vocab Variable",
        "morphology EnglishVerb",
        "morphology EnglishNoun",
        "lexeme NounLexeme",
        "lexeme VerbLexeme",
        "codec Noun",
        "identity SelfReferenceSpelling",
        "codec SignedNumber",
        "construction spell",
        "construction triggered",
        "construction imperative",
        "construction declarative",
        "construction with_where",
        "construction event",
        "construction where",
        "construction pronoun",
        "construction common",
        "construction demonstrative",
        "construction target",
        "construction self_reference",
        "construction count",
        "construction destroy",
        "construction connive",
        "construction deal_damage",
        "construction gain_life",
        "construction number",
        "construction variable",
        "root Ability",
        "root Sentence",
    ];

    const CONSTRUCTION_ORIGINS: &[&str] = &[
        "construction spell",
        "construction triggered",
        "construction imperative",
        "construction declarative",
        "construction with_where",
        "construction event",
        "construction where",
        "construction pronoun",
        "construction common",
        "construction demonstrative",
        "construction target",
        "construction self_reference",
        "construction count",
        "construction destroy",
        "construction connive",
        "construction deal_damage",
        "construction gain_life",
        "construction number",
        "construction variable",
    ];

    const SCANNER_ORIGINS: &[&str] = &[
        "vocab TriggerWord",
        "vocab Article",
        "vocab Demonstrative",
        "vocab Pronoun",
        "vocab Variable",
        "lexeme NounLexeme",
        "lexeme VerbLexeme",
        "codec Noun",
        "identity SelfReferenceSpelling",
        "codec SignedNumber",
        "construction triggered",
        "construction with_where",
        "root Ability",
        "root Sentence",
    ];

    const VISITOR_ORIGINS: &[&str] = &[
        "construction spell",
        "construction triggered",
        "construction imperative",
        "construction declarative",
        "construction with_where",
        "construction event",
        "construction where",
        "construction pronoun",
        "construction common",
        "construction demonstrative",
        "construction target",
        "construction self_reference",
        "construction count",
        "construction destroy",
        "construction connive",
        "construction deal_damage",
        "construction gain_life",
        "construction number",
        "construction variable",
        "codec SignedNumber",
        "codec Noun",
        "construction spell",
        "construction triggered",
        "construction imperative",
        "construction declarative",
        "construction with_where",
        "construction event",
        "construction where",
        "construction pronoun",
        "construction common",
        "construction demonstrative",
        "construction target",
        "construction self_reference",
        "construction count",
        "construction destroy",
        "construction connive",
        "construction deal_damage",
        "construction gain_life",
        "construction number",
        "construction variable",
        "vocab TriggerWord",
        "vocab Article",
        "vocab Demonstrative",
        "vocab Pronoun",
        "vocab Variable",
        "identity SelfReferenceSpelling",
        "lexeme NounLexeme",
        "lexeme VerbLexeme",
    ];

    #[allow(
        clippy::too_many_lines,
        reason = "the complete production-origin oracle is deliberately explicit"
    )]
    fn expected_production_origins(item_key: &str) -> Option<&'static [&'static str]> {
        Some(match item_key {
            "type Ability" | "function walk_ability" => {
                &["construction spell", "construction triggered"]
            }
            "type Sentence" | "function render_sentence_body" | "function walk_sentence" => &[
                "construction imperative",
                "construction declarative",
                "construction with_where",
            ],
            "type Clause" | "function render_clause" | "function walk_clause" => {
                &["construction event", "construction where"]
            }
            "type NounPhrase"
            | "function render_noun_phrase"
            | "function agreement_for_noun_phrase"
            | "function number_for_noun_phrase"
            | "function walk_noun_phrase" => &[
                "construction pronoun",
                "construction common",
                "construction demonstrative",
                "construction target",
                "construction self_reference",
                "construction count",
            ],
            "type VerbPhrase" | "function render_verb_phrase" | "function walk_verb_phrase" => &[
                "construction destroy",
                "construction connive",
                "construction deal_damage",
                "construction gain_life",
            ],
            "type Amount" | "function render_amount" | "function walk_amount" => {
                &["construction number", "construction variable"]
            }
            "type Spell" | "function walk_spell" => &["construction spell"],
            "type Triggered" | "impl Triggered" | "function walk_triggered" => {
                &["construction triggered"]
            }
            "type Imperative" | "function walk_imperative" => &["construction imperative"],
            "type Declarative" | "function walk_declarative" => &["construction declarative"],
            "type WithWhere" | "impl WithWhere" | "function walk_with_where" => {
                &["construction with_where"]
            }
            "type EventClause" | "function walk_event_clause" => &["construction event"],
            "type WhereClause" | "function walk_where_clause" => &["construction where"],
            "type PronounNp" | "function walk_pronoun_np" => &["construction pronoun"],
            "type Common" | "function walk_common" => &["construction common"],
            "type DemonstrativeNp"
            | "function walk_demonstrative_np"
            | "function agreement_for_demonstrative"
            | "function number_for_demonstrative" => &["construction demonstrative"],
            "type TargetNp" | "function walk_target_np" => &["construction target"],
            "type SelfReferenceNp" | "impl SelfReferenceNp" | "function walk_self_reference_np" => {
                &["construction self_reference"]
            }
            "type CountNp" | "impl CountNp" | "function walk_count_np" => &["construction count"],
            "type Destroy" | "function walk_destroy" => &["construction destroy"],
            "type Connive" | "function walk_connive" => &["construction connive"],
            "type DealDamage" | "function walk_deal_damage" => &["construction deal_damage"],
            "type GainLife" | "function walk_gain_life" => &["construction gain_life"],
            "type NumberAmount" | "function walk_number_amount" => &["construction number"],
            "type VariableAmount" | "function walk_variable_amount" => &["construction variable"],
            "type TriggerWord" | "function render_trigger_word" | "function walk_trigger_word" => {
                &["vocab TriggerWord"]
            }
            "type Article" | "function render_article" | "function walk_article" => {
                &["vocab Article"]
            }
            "type Demonstrative"
            | "function render_demonstrative"
            | "function walk_demonstrative" => &["vocab Demonstrative"],
            "type Pronoun" | "function render_pronoun" | "function walk_pronoun" => {
                &["vocab Pronoun"]
            }
            "function agreement_for_pronoun" => &["construction pronoun", "construction count"],
            "type Variable" | "function render_variable" | "function walk_variable" => {
                &["vocab Variable"]
            }
            "type NounLexeme"
            | "function surface_for_noun_lexeme"
            | "function walk_noun_lexeme" => &["lexeme NounLexeme"],
            "type VerbLexeme"
            | "function surface_for_verb_lexeme"
            | "function walk_verb_lexeme" => &["lexeme VerbLexeme"],
            "type DeclarationNoun"
            | "impl DeclarationNoun"
            | "type Noun"
            | "function walk_declaration_noun"
            | "function walk_noun" => &["codec Noun"],
            "type Sign"
            | "type SignedNumber"
            | "function render_signed_number"
            | "function walk_sign"
            | "function walk_signed_number" => &["codec SignedNumber"],
            "type SelfReferenceSpelling" | "impl SelfReferenceSpelling" => {
                &["identity SelfReferenceSpelling"]
            }
            "type Agreement"
            | "type Number"
            | "type FeatureConstraint"
            | "type CasePosition"
            | "type ScanPosition"
            | "type DeclarationClass"
            | "type DeclarationMatcher"
            | "type DeclarationLeaf"
            | "type Lexical"
            | "type Leaf"
            | "type TerminalClass"
            | "type LexicalTerminal"
            | "type LexicalProvenanceKind"
            | "type LexicalOwnerTemplate"
            | "type LexicalOwnerIdentity"
            | "type LexicalOwner"
            | "impl Lexical for Lexical"
            | "impl LexicalTerminal for LexicalTerminal"
            | "impl TerminalClass for TerminalClass"
            | "impl std::fmt::Display for TerminalClass"
            | "impl DeclarationClass for DeclarationClass"
            | "function declaration_lexeme_owner_id"
            | "impl LexicalOwner for LexicalOwner"
            | "impl Debug for LexicalOwner"
            | "impl Clone for LexicalOwner"
            | "impl PartialEq for LexicalOwner"
            | "impl Eq for LexicalOwner"
            | "impl Ord for LexicalOwner"
            | "impl PartialOrd for LexicalOwner"
            | "impl LexicalOwnerTemplate for LexicalOwnerTemplate"
            | "constant REQUIRED_DECLARATIONS" => ALL_DECLARATION_ORIGINS,
            "function scan_lexical" => SCANNER_ORIGINS,
            "impl Ability" | "impl Render for Ability" | "function render_ability_with_claims" => {
                &["root Ability"]
            }
            "impl Sentence"
            | "impl Render for Sentence"
            | "function render_sentence_with_claims" => &["root Sentence"],
            "trait Visitor" => VISITOR_ORIGINS,
            "function walk_self_reference_spelling" => &["identity SelfReferenceSpelling"],
            "type Category" | "type Construction" | "type RuleId" | "impl RuleId" => {
                CONSTRUCTION_ORIGINS
            }
            "constant RULES" | "function build" => &[
                "construction spell",
                "construction triggered",
                "construction imperative",
                "construction declarative",
                "construction with_where",
                "construction event",
                "construction where",
                "construction pronoun",
                "construction common",
                "construction demonstrative",
                "construction target",
                "construction self_reference",
                "construction count",
                "construction destroy",
                "construction connive",
                "construction deal_damage",
                "construction gain_life",
                "construction number",
                "construction variable",
                "root Ability",
            ],
            _ => return None,
        })
    }

    struct AuditedTerminalContribution {
        origin: String,
        expected_generated_item_keys: Vec<ItemKey>,
    }

    fn audit_expected_terminal_items(
        expected: &[AuditedTerminalContribution],
        actual: &[deckmaste_construction_core::GeneratedItem],
        classify: impl Fn(&str) -> Option<Vec<String>>,
    ) -> Vec<String> {
        let mut failures = Vec::new();
        for contribution in expected {
            let origin = &contribution.origin;
            for key in &contribution.expected_generated_item_keys {
                let item_name = item_key_name(key);
                let actual_item = actual.iter().find(|item| item.key == *key);
                match actual_item {
                    Some(item)
                        if item.origins.iter().any(|candidate| {
                            format!(
                                "{} {}",
                                declaration_kind_name(candidate.kind()),
                                candidate.name(),
                            ) == *origin
                        }) => {}
                    Some(_) => failures.push(format!(
                        "generated terminal item `{item_name}` does not carry declaration origin `{origin}`"
                    )),
                    None => failures.push(format!(
                        "missing generated terminal item `{item_name}` for declaration origin `{origin}`"
                    )),
                }
                match classify(&item_name) {
                    Some(origins)
                        if !origins
                            .iter()
                            .any(|candidate| candidate == origin) =>
                    {
                        failures.push(format!(
                            "explicit origin classification for generated terminal item `{item_name}` omits declaration origin `{origin}`"
                        ));
                    }
                    Some(origins) => {
                        if let Some(item) = actual_item {
                            let actual_origins = item
                                .origins
                                .iter()
                                .map(|origin| {
                                    format!(
                                        "{} {}",
                                        declaration_kind_name(origin.kind()),
                                        origin.name()
                                    )
                                })
                                .collect::<Vec<_>>();
                            if actual_origins != origins {
                                failures.push(format!(
                                    "generated terminal item `{item_name}` origin classification differs: item has {actual_origins:?}, classification has {origins:?}"
                                ));
                            }
                        }
                    }
                    None => failures.push(format!(
                        "missing explicit origin classification for generated terminal item `{item_name}` from declaration origin `{origin}`"
                    )),
                }
            }
        }
        failures
    }

    fn expected_terminal_items(expansion: &Expansion) -> Vec<AuditedTerminalContribution> {
        expansion
            .terminal_contributions()
            .iter()
            .map(|contribution| AuditedTerminalContribution {
                origin: format!(
                    "{} {}",
                    declaration_kind_name(contribution.origin().kind()),
                    contribution.origin().name(),
                ),
                expected_generated_item_keys: contribution.expected_generated_item_keys().to_vec(),
            })
            .collect()
    }

    const EXPECTED_ITEM_KEYS: &[&str] = &[
        "type Ability",
        "type Sentence",
        "type Clause",
        "type NounPhrase",
        "type VerbPhrase",
        "type Amount",
        "type Spell",
        "type Triggered",
        "impl Triggered",
        "type Imperative",
        "type Declarative",
        "type WithWhere",
        "impl WithWhere",
        "type EventClause",
        "type WhereClause",
        "type PronounNp",
        "type Common",
        "type DemonstrativeNp",
        "type TargetNp",
        "type SelfReferenceNp",
        "impl SelfReferenceNp",
        "type CountNp",
        "impl CountNp",
        "type Destroy",
        "type Connive",
        "type DealDamage",
        "type GainLife",
        "type NumberAmount",
        "type VariableAmount",
        "type TriggerWord",
        "type Article",
        "type Demonstrative",
        "type Pronoun",
        "type Variable",
        "type NounLexeme",
        "function surface_for_noun_lexeme",
        "type VerbLexeme",
        "function surface_for_verb_lexeme",
        "type DeclarationNoun",
        "impl DeclarationNoun",
        "type Noun",
        "type SelfReferenceSpelling",
        "impl SelfReferenceSpelling",
        "type Sign",
        "type SignedNumber",
        "type Agreement",
        "type Number",
        "type FeatureConstraint",
        "type CasePosition",
        "type ScanPosition",
        "type DeclarationClass",
        "type DeclarationMatcher",
        "type DeclarationLeaf",
        "type Lexical",
        "type Leaf",
        "type TerminalClass",
        "type LexicalTerminal",
        "type LexicalProvenanceKind",
        "type LexicalOwnerTemplate",
        "type LexicalOwnerIdentity",
        "type LexicalOwner",
        "impl Lexical for Lexical",
        "impl LexicalTerminal for LexicalTerminal",
        "impl TerminalClass for TerminalClass",
        "impl std::fmt::Display for TerminalClass",
        "impl DeclarationClass for DeclarationClass",
        "function declaration_lexeme_owner_id",
        "impl LexicalOwner for LexicalOwner",
        "impl Debug for LexicalOwner",
        "impl Clone for LexicalOwner",
        "impl PartialEq for LexicalOwner",
        "impl Eq for LexicalOwner",
        "impl Ord for LexicalOwner",
        "impl PartialOrd for LexicalOwner",
        "impl LexicalOwnerTemplate for LexicalOwnerTemplate",
        "constant REQUIRED_DECLARATIONS",
        "function scan_lexical",
        "impl Ability",
        "impl Render for Ability",
        "function render_ability_with_claims",
        "impl Sentence",
        "impl Render for Sentence",
        "function render_sentence_with_claims",
        "function render_sentence_body",
        "function render_clause",
        "function render_verb_phrase",
        "function render_noun_phrase",
        "function render_amount",
        "function render_trigger_word",
        "function render_article",
        "function render_demonstrative",
        "function render_pronoun",
        "function render_variable",
        "function agreement_for_pronoun",
        "function agreement_for_demonstrative",
        "function number_for_demonstrative",
        "function render_signed_number",
        "function agreement_for_noun_phrase",
        "function number_for_noun_phrase",
        "trait Visitor",
        "function walk_ability",
        "function walk_sentence",
        "function walk_clause",
        "function walk_noun_phrase",
        "function walk_verb_phrase",
        "function walk_amount",
        "function walk_spell",
        "function walk_triggered",
        "function walk_imperative",
        "function walk_declarative",
        "function walk_with_where",
        "function walk_event_clause",
        "function walk_where_clause",
        "function walk_pronoun_np",
        "function walk_common",
        "function walk_demonstrative_np",
        "function walk_target_np",
        "function walk_self_reference_np",
        "function walk_count_np",
        "function walk_destroy",
        "function walk_connive",
        "function walk_deal_damage",
        "function walk_gain_life",
        "function walk_number_amount",
        "function walk_variable_amount",
        "function walk_trigger_word",
        "function walk_article",
        "function walk_demonstrative",
        "function walk_pronoun",
        "function walk_variable",
        "function walk_self_reference_spelling",
        "function walk_noun_lexeme",
        "function walk_verb_lexeme",
        "function walk_sign",
        "function walk_signed_number",
        "function walk_declaration_noun",
        "function walk_noun",
        "type Category",
        "type Construction",
        "type RuleId",
        "impl RuleId",
        "constant RULES",
        "function build",
    ];

    #[test]
    fn single_authority_inventory_audits_unexpected_nested_rust_source() {
        let temporary = tempfile::tempdir().expect("temporary audit root is available");
        let parser = temporary.path().join("parser");
        fs::create_dir(&parser).expect("nested parser directory is created");
        fs::write(
            parser.join("mirror.rs"),
            "fn scan_noun() { let _ = \"destroys\"; }",
        )
        .expect("unexpected Rust source is written");
        fs::write(parser.join("notes.txt"), "fn scan_noun() {}")
            .expect("non-Rust decoy is written");

        let paths = discover_rust_sources(temporary.path());
        assert_eq!(paths.len(), 1);
        assert!(paths[0].ends_with(Path::new("parser/mirror.rs")));
        let files = parse_rust_sources(&paths);
        let violations = english_runtime_authority_violations(
            &files[0].0,
            &files[0].1,
            &ClosedLexemeDeclarations::new(),
        );
        assert!(
            violations
                .iter()
                .any(|violation| violation.ends_with("handwritten terminal path scan_noun"))
        );
        assert!(
            violations.iter().any(|violation| {
                violation.ends_with("handwritten open-verb spelling destroys")
            })
        );
    }

    #[cfg(unix)]
    #[test]
    fn source_inventory_fails_closed_on_symlink_entries() {
        use std::os::unix::fs::symlink;

        let temporary = tempfile::tempdir().expect("temporary audit root is available");
        let outside = tempfile::tempdir().expect("outside source root is available");
        fs::write(outside.path().join("mirror.rs"), "fn scan_noun() {}")
            .expect("outside Rust source is written");
        symlink(outside.path(), temporary.path().join("linked"))
            .expect("source-directory symlink is created");

        let result = std::panic::catch_unwind(|| discover_rust_sources(temporary.path()));
        assert!(result.is_err(), "symlinked audit entries must fail closed");
    }

    #[test]
    fn production_indirection_predicate_rejects_path_and_code_include_only() {
        for source in [
            "#[path = \"outside.rs\"] mod mirror;",
            "include!(\"outside.inc\");",
            "#[cfg(any(test, unix))] mod fixture { include!(\"fixture.inc\"); }",
            "#[cfg(not(test))] mod fixture { include!(\"fixture.inc\"); }",
        ] {
            let file = syn::parse_file(source).expect("production indirection source reparses");
            assert!(contains_production_code_indirection(&file));
        }

        for source in [
            "const FIXTURE: &str = include_str!(\"fixture.rs\");",
            "#[cfg(test)] mod fixture { include!(\"fixture.inc\"); }",
            "#[cfg(all(test, unix))] mod fixture { include!(\"fixture.inc\"); }",
            "fn source_fence_message() { let _ = \"include!(path) is rejected\"; }",
        ] {
            let file = syn::parse_file(source).expect("allowed fixture source reparses");
            assert!(!contains_production_code_indirection(&file));
        }
    }

    #[test]
    fn production_indirection_predicate_rejects_nested_macro_tokens() {
        for source in [
            "passthrough! { include!(\"mirror.inc\"); }",
            "passthrough! { nested! { include!(\"mirror.inc\"); } }",
            "passthrough! { #[path = \"mirror.rs\"] mod mirror; }",
        ] {
            let file = syn::parse_file(source).expect("nested indirection source reparses");
            assert!(
                contains_production_code_indirection(&file),
                "nested token-visible indirection escaped: {source}"
            );
        }

        for source in [
            "passthrough! { \"include!(mirror.inc)\" }",
            "passthrough! { b\"#[path = mirror.rs] mod mirror;\" }",
        ] {
            let file = syn::parse_file(source).expect("nested literal decoy source reparses");
            assert!(
                !contains_production_code_indirection(&file),
                "literal decoy was treated as code: {source}"
            );
        }
    }

    #[test]
    fn production_indirection_predicate_rejects_macro_rules_body() {
        let source = syn::parse_file(
            "macro_rules! mirror { \
             () => { include!(\"mirror.inc\"); }; \
             ($name:ident) => { #[path = \"mirror.rs\"] mod $name; }; \
             }",
        )
        .expect("macro_rules indirection source reparses");

        assert!(contains_production_code_indirection(&source));
    }

    #[test]
    fn production_indirection_predicate_rejects_nested_path_before_visibility() {
        let source = syn::parse_file("passthrough! { #[path = \"mirror.rs\"] pub mod mirror; }")
            .expect("nested visible path source reparses");

        assert!(contains_production_code_indirection(&source));
    }

    #[test]
    fn production_indirection_predicate_rejects_nested_path_before_attribute() {
        let source = syn::parse_file(
            "macro_rules! mirror { \
             () => { #[path = \"mirror.rs\"] #[allow(dead_code)] mod mirror; }; \
             }",
        )
        .expect("nested attributed path source reparses");

        assert!(contains_production_code_indirection(&source));
    }

    #[test]
    fn complete_core_boundary_discovers_source_authority_in_nested_helper() {
        let temporary = tempfile::tempdir().expect("temporary core root is available");
        let helper = temporary.path().join("helper");
        fs::create_dir(&helper).expect("nested helper directory is created");
        fs::write(
            temporary.path().join("parse.rs"),
            "fn parse() { parse_declarations(tokens); }",
        )
        .expect("allowed compiler source is written");
        fs::write(
            helper.join("mirror.rs"),
            "fn moved() { parse_declarations(tokens); }",
        )
        .expect("unexpected helper source is written");

        assert_eq!(
            source_authority_paths(temporary.path()),
            ["helper/mirror.rs", "parse.rs"]
        );
    }

    #[test]
    fn authority_predicate_detects_macro_declarations() {
        let macro_mirror = syn::parse_file(
            "define_mirror! { enum Lexical { Mirror } struct Leaf; type NounNumber = (); }",
        )
        .expect("macro mirror source reparses");
        for (kind, name) in [
            (ProductionAuthorityKind::Enum, "Lexical"),
            (ProductionAuthorityKind::Struct, "Leaf"),
            (ProductionAuthorityKind::TypeAlias, "NounNumber"),
        ] {
            assert!(contains_production_authority(&macro_mirror, kind, name));
        }

        let dsl_mention = syn::parse_file("constructions! { terminal Noun { singular: noun } }")
            .expect("DSL mention source reparses");
        for kind in [
            ProductionAuthorityKind::Enum,
            ProductionAuthorityKind::Struct,
            ProductionAuthorityKind::TypeAlias,
        ] {
            assert!(!contains_production_authority(&dsl_mention, kind, "Noun"));
        }
    }

    #[test]
    fn ghost_predicate_detects_renamed_test_alias_rhs() {
        let renamed_alias =
            syn::parse_file("#[cfg(test)] type LegacyLexical = crate::constructions::Lexical;")
                .expect("renamed test alias source reparses");
        assert!(contains_test_only_generated_ghost(&renamed_alias));
    }

    #[test]
    fn strictly_test_only_cfg_classification_evaluates_nested_formulas() {
        for source in [
            "#[cfg(test)] fn candidate() {}",
            "#[cfg(all(test, unix))] fn candidate() {}",
            "#[cfg(not(not(test)))] fn candidate() {}",
            "#[cfg(all(not(not(test)), any(unix, windows)))] fn candidate() {}",
            "#[cfg(unix)] #[cfg(test)] fn candidate() {}",
        ] {
            let file = syn::parse_file(source).expect("strictly test-only source reparses");
            assert!(
                item_attrs(&file.items[0]).is_some_and(is_test_only),
                "expected strictly test-only cfg: {source}"
            );
        }

        for source in [
            "fn candidate() {}",
            "#[cfg(unix)] fn candidate() {}",
            "#[cfg(any(test, unix))] fn candidate() {}",
            "#[cfg(not(test))] fn candidate() {}",
            "#[cfg(not(not(not(test))))] fn candidate() {}",
            "#[cfg(all(unix, not(test)))] fn candidate() {}",
        ] {
            let file = syn::parse_file(source).expect("production-capable source reparses");
            assert!(
                !item_attrs(&file.items[0]).is_some_and(is_test_only),
                "expected production-capable cfg: {source}"
            );
        }
    }

    #[test]
    fn nested_strict_test_cfg_prunes_alias_but_not_production_capable_items() {
        let renamed_alias = syn::parse_file(
            "#[cfg(all(test, unix))] \
             type LegacyLexical = crate::constructions::Lexical;",
        )
        .expect("nested-cfg renamed alias source reparses");
        assert!(contains_test_only_generated_ghost(&renamed_alias));

        for source in [
            "#[cfg(any(test, unix))] fn scan_noun() {}",
            "#[cfg(not(test))] fn scan_noun() {}",
        ] {
            let file = syn::parse_file(source).expect("production-capable source reparses");
            assert!(contains_production_function(&file, "scan_noun"));
        }
    }

    #[test]
    fn production_spelling_census_evaluates_nested_concat() {
        let source = syn::parse_file(
            "fn spellings() { \
             let _ = concat!(\"de\", concat!(\"stro\", \"ys\")); \
             let _ = concat!(b\"con\", concat!(\"ni\", b\"ves\")); \
             }",
        )
        .expect("constant spelling source reparses");
        let spellings = production_string_literals(&source);

        assert!(spellings.iter().any(|spelling| spelling == "destroys"));
        assert!(spellings.iter().any(|spelling| spelling == "connives"));
    }

    #[test]
    fn production_spelling_census_detects_byte_strings() {
        let source = syn::parse_file("fn spellings() { let _ = b\"connives\"; }")
            .expect("byte spelling source reparses");
        let spellings = production_string_literals(&source);

        assert!(spellings.iter().any(|spelling| spelling == "connives"));
    }

    #[test]
    fn production_spelling_census_prunes_cfg_test_constants() {
        let source = syn::parse_file(
            "#[cfg(test)] const DECOY: &str = concat!(\"de\", \"stroys\"); \
             const PRODUCTION: &str = \"ordinary\";",
        )
        .expect("cfg-test constant source reparses");
        let spellings = production_string_literals(&source);

        assert!(!spellings.iter().any(|spelling| spelling == "destroys"));
        assert!(spellings.iter().any(|spelling| spelling == "ordinary"));
    }

    #[test]
    fn plan04_host_scan_includes_production_without_matching_its_test_module() {
        let host = syn::parse_file(
            "fn checked_constructor() {}\n\
             #[cfg(test)] mod tests { struct Checked; }",
        )
        .expect("synthetic audit host reparses");

        assert_eq!(plan04_host_authorities(&host), ["checked_constructor"]);
    }

    #[test]
    fn plan04_host_scan_links_file_backed_production_modules() {
        let host = syn::parse_file(
            "struct Triggered; mod report; impl report::Alias {} \
             #[cfg(test)] mod tests { type Alias = Triggered; }",
        )
        .expect("synthetic file-backed host reparses");
        let report = syn::parse_file("pub(super) type Alias = super::Triggered;")
            .expect("synthetic linked report reparses");

        assert_eq!(
            plan04_host_authorities_with_report(&host, &report, &[]),
            ["impl Triggered"]
        );
    }

    #[test]
    fn plan04_checked_blocks_are_detected_as_comment_insensitive_macro_tokens() {
        for source in [
            "fn fixture() { constructions! { checked /* legacy */ { constructor = old; } } }",
            "fn fixture() { constructions! { checked // legacy\n { access value; } } }",
            "fn fixture() { constructions! {\n\
                 construction legacy: Root {\n\
                     element Legacy {}\n\
                     checked /* retired metadata */ { visibility field = private; }\n\
                     form legacy = \"legacy\";\n\
                 }\n\
             } }",
        ] {
            assert_eq!(checked_blocks_in_rust_source(source), [false], "{source}");
        }
    }

    #[test]
    fn plan04_checked_blocks_are_detected_in_every_rust_string_fixture() {
        let fixtures = r##"
            const ORDINARY: &str = "checked { constructor = old; }";
            const RAW: &str = r#"checked /* legacy */ { constructor = old; }"#;
            const BYTES: &[u8] = b"checked // legacy\n { constructor = old; }";
            const NESTED: &str = "checked /* outer /* inner */ outer */ { constructor = old; }";
            wrapped! { "checked /* nested */ { constructor = old; }" }
        "##;
        let fixture_census = checked_block_census_in_rust_source(fixtures);
        assert!(fixture_census.structural.is_empty());
        assert_eq!(fixture_census.strings, 5);
        assert_eq!(fixture_census.exact_empty_strings, 0);

        let parser_like = r##"
            fn fixture() {
                parse(r#"construction only: Root { checked {} }"#);
                let _ = r#"checked /* legacy */ { constructor = old; }"#;
            }
        "##;
        let parser_census = checked_block_census_in_rust_source(parser_like);
        assert!(parser_census.structural.is_empty());
        assert_eq!(parser_census.strings, 2);
        assert_eq!(parser_census.exact_empty_strings, 1);

        let token_roles = r"
            const STRINGIFIED: &str = stringify!(checked {});
            macro_rules! recognizes { (checked {}) => {}; }
        ";
        let role_census = checked_block_census_in_rust_source(token_roles);
        assert!(role_census.structural.is_empty());
        assert_eq!(role_census.strings, 1);
        assert_eq!(role_census.exact_empty_strings, 0);
    }

    #[test]
    fn plan04_checked_block_census_evaluates_nested_string_macros() {
        let fixtures = r##"
            const DIRECT: &str = concat!(stringify!(checked), "{}");
            const RAW: &str = concat!(stringify!(checked), r#" /* legacy */ {"#);
            const BYTES: &[u8] = concat!(b"check", stringify!(ed), b" {}");
            const NESTED: &str = (concat!(concat!("check", "ed"), stringify!({})));
        "##;
        let fixture_census = checked_block_census_in_rust_source(fixtures);
        assert!(fixture_census.structural.is_empty());
        assert_eq!(fixture_census.strings, 4);
        assert_eq!(fixture_census.exact_empty_strings, 1);

        let parser_like = r##"
            const ALLOWED: &str = r#"checked {}"#;
            const EXTRA: &str = concat!(stringify!(checked), "{}");
        "##;
        let parser_census = checked_block_census_in_rust_source(parser_like);
        assert!(parser_census.structural.is_empty());
        assert_eq!(parser_census.strings, 2);
        assert_eq!(parser_census.exact_empty_strings, 1);
    }

    #[test]
    fn plan04_checked_block_census_fails_closed_on_unknown_string_composition() {
        let fixtures = r#"
            const TOKEN: &str = concat!(unknown!(checked), "{}");
            const FRAGMENTS: &str = concat!(unknown!("check"), "ed{}");
        "#;
        let fixture_census = checked_block_census_in_rust_source(fixtures);
        assert_eq!(fixture_census.unresolved_compositions, 2);

        let parser_like = r##"
            const ALLOWED: &str = r#"checked {}"#;
            const UNKNOWN: &str = concat!(unknown!(checked), "{}");
        "##;
        let parser_census = checked_block_census_in_rust_source(parser_like);
        assert_eq!(parser_census.strings, 1);
        assert_eq!(parser_census.exact_empty_strings, 1);
        assert_eq!(parser_census.unresolved_compositions, 1);
    }

    #[test]
    fn plan04_checked_bindings_producer_scan_rejects_every_mutable_escape() {
        use syn::visit::Visit as _;

        for source in [
            "fn candidate(report: &mut Report, entry: Binding) { \
                 report.checked_constructor_bindings = vec![entry]; \
             }",
            "fn candidate(report: &mut Report, entry: Binding) { \
                 Vec::push(&mut report.checked_constructor_bindings, entry); \
             }",
            "fn candidate(report: &mut Report, entry: Binding) { \
                 let bindings = &mut report.checked_constructor_bindings; \
                 mutate(bindings, entry); \
             }",
            "fn candidate(report: &mut Report, entry: Binding) { \
                 report.checked_constructor_bindings.resize_with(1, || entry); \
             }",
            "fn candidate(report: &mut Report, entries: Vec<Binding>) { \
                 report.checked_constructor_bindings.splice(.., entries); \
             }",
            "fn candidate(report: &mut Report, entry: Binding) { \
                 mutate(&mut report.checked_constructor_bindings, entry); \
             }",
            "fn candidate(report: &mut Report, entry: Binding) { \
                 mutate!(&mut report.checked_constructor_bindings, entry); \
             }",
            "fn candidate(report: &mut Report) { \
                 mutate! { binding => report.checked_constructor_bindings } \
             }",
        ] {
            let file = syn::parse_file(source).expect("producer escape source reparses");
            let mut finder = CheckedBindingsProducerFinder::default();
            finder.visit_file(&file);
            assert!(
                !finder.violations.is_empty(),
                "mutable escape passed: {source}"
            );
        }
    }

    #[test]
    fn plan04_generated_output_text_does_not_count_as_impl_authority() {
        let generated_output = "generated item: impl Triggered { ... }\n\
            generated item: impl SelfReferenceNp { ... }";

        assert!(plan04_retired_text_authorities(generated_output).is_empty());
    }

    #[test]
    fn plan04_impl_authority_requires_actual_rust_or_macro_syntax() {
        use syn::visit::Visit as _;

        for (source, expected) in [
            ("impl Triggered {}", "impl Triggered"),
            (
                "generated! { impl SelfReferenceNp {} }",
                "impl SelfReferenceNp",
            ),
        ] {
            let file = syn::parse_file(source).expect("impl authority source reparses");
            let mut finder = Plan04RetiredAuthorityFinder::default();
            finder.visit_file(&file);
            assert!(finder.violations.contains(&expected), "{source}");
        }

        let trait_impl =
            syn::parse_file("impl Render for Triggered {}").expect("trait impl source reparses");
        let mut finder = Plan04RetiredAuthorityFinder::default();
        finder.visit_file(&trait_impl);
        assert!(!finder.violations.contains(&"impl Triggered"));
    }

    #[test]
    fn plan04_impl_authority_normalizes_aliases_wrappers_and_macro_headers() {
        use syn::visit::Visit as _;

        for source in [
            "struct Triggered; impl (Triggered) {}",
            "struct Triggered; type Alias = Triggered; impl Alias {}",
            "struct Triggered; type Alias = Triggered; type Alias2 = Alias; impl Alias2 {}",
            "mod generated { pub struct Triggered; } \
             use generated::Triggered as Alias; impl Alias {}",
            "struct Triggered<T>(T); impl<T> Triggered<T> {}",
            "macro_rules! m { ($body:item) => { impl Triggered { $body } } }",
            "macro_rules! m { ($body:item) => { impl (crate::Triggered) { $body } } }",
            "macro_rules! m { ($body:item) => { \
                 type Alias = Triggered; impl Alias { $body } \
             } }",
            "macro_rules! m { ($alias:ident, $body:item) => { \
                 type $alias = Triggered; impl $alias { $body } \
             } }",
            "macro_rules! m { \
                 () => { type Alias = Triggered; impl Alias {} }; \
                 ($value:expr) => { type Alias = Unrelated; }; \
             }",
        ] {
            let file = syn::parse_file(source).expect("alternate impl authority reparses");
            let mut finder = Plan04RetiredAuthorityFinder::default();
            finder.visit_file(&file);
            assert!(finder.violations.contains(&"impl Triggered"), "{source}");
        }

        for source in [
            "struct Triggered; trait Render {} impl Render for Triggered {}",
            "macro_rules! m { ($body:item) => { impl Render for Triggered { $body } } }",
            "struct Unrelated; type Alias = Unrelated; impl Alias {}",
            "type Alias = Alias2; type Alias2 = Alias; impl Alias {}",
            "mod one { struct Triggered; type Alias = Triggered; } \
             mod two { struct Alias; impl Alias {} }",
            "struct Triggered; type Alias = Triggered; \
             mod inner { struct Alias; impl Alias {} }",
            "macro_rules! m { \
                 () => { type Alias = Unrelated; impl Alias {} }; \
                 ($value:expr) => { type Alias = Triggered; }; \
             }",
        ] {
            let file = syn::parse_file(source).expect("allowed impl authority reparses");
            let mut finder = Plan04RetiredAuthorityFinder::default();
            finder.visit_file(&file);
            assert!(!finder.violations.contains(&"impl Triggered"), "{source}");
        }

        let module = syn::parse_file("pub(super) type Alias = super::Triggered;")
            .expect("file-backed alias module reparses");
        let module_path = vec!["aliases".to_owned()];
        for root_source in [
            "struct Triggered; mod aliases; impl aliases::Alias {}",
            "struct Triggered; mod aliases; use aliases::Alias; impl Alias {}",
        ] {
            let root = syn::parse_file(root_source).expect("file-backed root reparses");
            let root_path = Vec::new();
            let graph = QualifiedAliasGraph::from_files(
                [
                    (&root, root_path.as_slice()),
                    (&module, module_path.as_slice()),
                ],
                false,
            );
            let mut finder = Plan04RetiredAuthorityFinder {
                qualified_aliases: graph,
                qualified_aliases_initialized: true,
                ..Plan04RetiredAuthorityFinder::default()
            };
            finder.visit_file(&root);
            assert!(
                finder.violations.contains(&"impl Triggered"),
                "{root_source}"
            );
        }
    }

    #[test]
    fn plan04_impl_authority_resolves_qualified_module_exports() {
        use syn::visit::Visit as _;

        for source in [
            "struct Triggered; mod aliases { pub(super) type Alias = super::Triggered; } \
             impl aliases::Alias {}",
            "struct Triggered; mod aliases { pub(super) type Alias = super::Triggered; } \
             use aliases::Alias; impl Alias {}",
            "struct Triggered; mod aliases { type Alias = super::Triggered; \
                 pub(super) use self::Alias as Exported; } \
             use self::aliases::Exported as Imported; impl Imported {}",
            "struct Triggered; mod aliases { type Alias = super::Triggered; \
                 impl self::Alias {} }",
            "struct Triggered; mod aliases { type Alias = super::Triggered; \
                 mod nested { type Deep = super::Alias; impl self::Deep {} } }",
            "struct Triggered; mod aliases { type Alias = super::Triggered; } \
             mod nested { type Deep = super::aliases::Alias; impl Deep {} }",
            "struct Triggered; mod aliases { pub type Alias = super::Triggered; } \
             use aliases::*; impl Alias {}",
        ] {
            let file = syn::parse_file(source).expect("qualified impl authority reparses");
            let mut finder = Plan04RetiredAuthorityFinder::default();
            finder.visit_file(&file);
            assert!(finder.violations.contains(&"impl Triggered"), "{source}");
        }

        for source in [
            "struct Triggered; mod one { type Alias = super::Triggered; } \
             mod two { struct Alias; impl self::Alias {} }",
            "struct Triggered; mod target { type Alias = super::Triggered; } \
             mod unrelated { pub(super) struct Alias; } \
             use unrelated::Alias; impl Alias {}",
            "mod aliases { type One = self::Two; type Two = self::One; impl One {} }",
            "mod unrelated { pub struct Alias; } use unrelated::*; impl Alias {}",
        ] {
            let file = syn::parse_file(source).expect("qualified allowed authority reparses");
            let mut finder = Plan04RetiredAuthorityFinder::default();
            finder.visit_file(&file);
            assert!(!finder.violations.contains(&"impl Triggered"), "{source}");
        }
    }

    #[test]
    fn plan04_impl_authority_scans_macro_transcribers_not_matchers_or_strings() {
        use syn::visit::Visit as _;

        for source in [
            "const TEXT: &str = stringify!(impl Triggered {});",
            "const TEXT: &str = concat!(stringify!(impl Triggered {}), \"\");",
            "macro_rules! recognizes { (impl Triggered {}) => { impl Unrelated {} }; }",
            "macro_rules! repeated { ($(impl Triggered {})*) => { impl Unrelated {} }; }",
        ] {
            let file = syn::parse_file(source).expect("non-authority macro source reparses");
            let mut finder = Plan04RetiredAuthorityFinder::default();
            finder.visit_file(&file);
            assert!(!finder.violations.contains(&"impl Triggered"), "{source}");
        }

        for source in [
            "macro_rules! emits { () => { impl Triggered {} }; }",
            "macro_rules! emits { () => { impl (crate::Triggered) {} }; }",
            "struct Triggered; mod aliases { pub type Alias = super::Triggered; } \
             macro_rules! emits { () => { impl crate::aliases::Alias {} }; }",
            "macro_rules! emits { () => { type Alias = Triggered; impl Alias {} }; }",
            "struct Triggered; mod aliases { pub type Alias = super::Triggered; } \
             macro_rules! emits { () => { \
                 type Local = crate::aliases::Alias; impl Local {} \
             }; }",
            "macro_rules! emits { ($alias:ident) => { \
                 type $alias = Triggered; impl $alias {} \
             }; }",
            "macro_rules! emits { \
                 (impl Triggered {}) => { impl Triggered {} }; \
                 ($other:tt) => { impl Unrelated {} }; \
             }",
        ] {
            let file = syn::parse_file(source).expect("authority macro source reparses");
            let mut finder = Plan04RetiredAuthorityFinder::default();
            finder.visit_file(&file);
            assert!(finder.violations.contains(&"impl Triggered"), "{source}");
        }
    }

    #[test]
    fn plan04_generated_invariants_are_single_authority() {
        use syn::visit::Visit as _;

        let workspace_root = Path::new(env!("CARGO_MANIFEST_DIR")).join("../..");
        let host_path = Path::new(env!("CARGO_MANIFEST_DIR")).join("src/english_v2.rs");
        let report_path = workspace_root.join("crates/xtask/src/english_v2/report.rs");
        let parser_path = workspace_root.join("crates/deckmaste_construction_core/src/parse.rs");
        let mut violations = Vec::new();
        let checked_text_block = checked_text_block_pattern();
        let mut parser_checked_census = None;
        let source_paths = plan04_source_fixture_paths(&workspace_root);
        let qualified_alias_graphs = plan04_qualified_alias_graphs(&workspace_root, &source_paths);

        for path in source_paths {
            let relative = path
                .strip_prefix(&workspace_root)
                .unwrap_or_else(|error| panic!("{}: {error}", path.display()));
            let source = fs::read_to_string(&path)
                .unwrap_or_else(|error| panic!("{}: {error}", path.display()));
            violations.extend(
                plan04_retired_text_authorities(&source)
                    .into_iter()
                    .map(|authority| format!("{} retains {authority}", relative.display())),
            );
            if path.extension().is_some_and(|extension| extension == "rs") {
                let file = syn::parse_file(&source)
                    .unwrap_or_else(|error| panic!("{}: {error}", path.display()));
                let checked_census = checked_block_census(&file, false);
                if path == parser_path {
                    parser_checked_census = Some(checked_census);
                } else if !checked_census.structural.is_empty()
                    || checked_census.strings != 0
                    || checked_census.unresolved_compositions != 0
                {
                    violations.push(format!(
                        "{} retains checked metadata block",
                        relative.display()
                    ));
                }
                let (module_key, module_path) = plan04_module_location(&workspace_root, &path);
                let mut finder = Plan04RetiredAuthorityFinder {
                    allow_report_schema_field: path == report_path,
                    qualified_aliases: qualified_alias_graphs
                        .get(&module_key)
                        .unwrap_or_else(|| panic!("{} has an alias graph", path.display()))
                        .clone(),
                    module_path,
                    qualified_aliases_initialized: true,
                    ..Plan04RetiredAuthorityFinder::default()
                };
                finder.visit_file(&file);
                violations.extend(
                    finder
                        .violations
                        .into_iter()
                        .map(|authority| format!("{} retains {authority}", relative.display())),
                );
            } else if checked_text_block.is_match(&source) {
                violations.push(format!(
                    "{} retains checked metadata block",
                    relative.display()
                ));
            }
        }

        let host_source = fs::read_to_string(&host_path).expect("audit host source is readable");
        let host = syn::parse_file(&host_source).expect("audit host source reparses");
        let report_source = fs::read_to_string(&report_path).expect("report source is readable");
        let report = syn::parse_file(&report_source).expect("report source reparses");
        violations.extend(
            plan04_host_authorities_with_report(&host, &report, &["english_v2".to_owned()])
                .into_iter()
                .map(|authority| format!("crates/xtask/src/english_v2.rs retains {authority}")),
        );
        let host_checked_census = checked_block_census(&host, true);
        if !host_checked_census.structural.is_empty()
            || host_checked_census.strings != 0
            || host_checked_census.unresolved_compositions != 0
        {
            violations
                .push("crates/xtask/src/english_v2.rs retains checked metadata block".to_owned());
        }

        let parser_checked_census = parser_checked_census
            .expect("the complete source inventory includes the construction parser");
        assert!(
            parser_checked_census.structural.is_empty(),
            "the parser's checked-metadata rejection fixture must remain input text, not Rust or macro syntax"
        );
        assert_eq!(
            parser_checked_census.strings, 1,
            "the parser must retain exactly one checked-metadata string fixture"
        );
        assert_eq!(
            parser_checked_census.exact_empty_strings, 1,
            "the parser's sole checked-metadata string fixture must be precisely empty"
        );
        assert_eq!(
            parser_checked_census.unresolved_compositions, 0,
            "the parser must not hide checked metadata in an unresolved string composition"
        );
        let parser_source = fs::read_to_string(&parser_path).expect("parser source is readable");
        assert_eq!(
            parser_source.matches("checked {}").count(),
            1,
            "the parser must retain exactly one precise empty checked-metadata fixture"
        );
        assert_eq!(
            parser_source
                .matches("`checked` metadata was retired after Stage 4; use generated invariants")
                .count(),
            2,
            "the parser and its regression must pin the exact retirement diagnostic"
        );

        let mut producer = CheckedBindingsProducerFinder::default();
        producer.visit_file(&report);
        violations.extend(
            producer
                .violations
                .into_iter()
                .map(|violation| format!("crates/xtask/src/english_v2/report.rs: {violation}")),
        );

        let build_path =
            workspace_root.join("crates/deckmaste_construction_core/src/emit/build.rs");
        let build_source = fs::read_to_string(&build_path).expect("build emitter is readable");
        let build = syn::parse_file(&build_source).expect("build emitter reparses");
        for authority in [
            "legacy_refinement",
            "validate_legacy_refinement_projection",
            "as_role_refinement",
            "RequireExprSource",
            "RequireSubjectSource",
            "PredicateAtomPlan",
            "PredicateMemberPlan",
            "PredicateSubjectPlan",
        ] {
            if contains_production_identifier(&build, authority) {
                violations.push(format!(
                    "crates/deckmaste_construction_core/src/emit/build.rs retains build-side require/refinement authority {authority}"
                ));
            }
        }

        violations.sort();
        violations.dedup();
        assert!(violations.is_empty(), "{}", violations.join("\n"));
    }

    #[test]
    #[allow(
        clippy::too_many_lines,
        reason = "the complete Plan 03 single-authority matrix is deliberately explicit"
    )]
    fn plan03_terminal_generation_is_single_authority() {
        let workspace_root = Path::new(env!("CARGO_MANIFEST_DIR")).join("../..");
        let expansion = expansion_from_source(PRODUCTION_SOURCE)
            .expect("production declaration expands from the sealed semantic plan");
        let closed_lexemes = closed_lexeme_declarations(&expansion);
        let construction_core_src = workspace_root.join("crates/deckmaste_construction_core/src");
        let construction_core_paths = discover_rust_sources(&construction_core_src);
        let construction_core_files = parse_rust_sources(&construction_core_paths);
        for (path, file) in &construction_core_files {
            assert!(
                !contains_production_code_indirection(file),
                "{path} uses production #[path] or code-including include! indirection"
            );
        }

        let mut emitter_paths = discover_rust_sources(&construction_core_src.join("emit"));
        emitter_paths.push(construction_core_src.join("report.rs"));
        emitter_paths.sort();
        emitter_paths.dedup();
        let emitter_files = parse_rust_sources(&emitter_paths);
        for (path, file) in &emitter_files {
            for source_authority in ["ValidatedDeclarations", "Declarations"] {
                assert!(
                    !contains_production_identifier(file, source_authority),
                    "{path} reads source-shaped authority {source_authority}"
                );
            }
            assert!(
                !contains_production_method_call(file, "raw"),
                "{path} calls ValidatedDeclarations::raw"
            );
            for source_reader in ["parse_declarations", "invocation_from_source"] {
                assert!(
                    !contains_production_identifier(file, source_reader),
                    "{path} rereads source-shaped declarations through {source_reader}"
                );
            }
            assert!(
                !contains_production_code_indirection(file),
                "{path} uses production #[path] or code-including include! indirection"
            );
        }

        assert_eq!(
            source_authority_paths(&construction_core_src),
            [
                "lib.rs",
                "model.rs",
                "parse.rs",
                "semantic.rs",
                "source.rs",
                "test_support.rs",
                "validate.rs",
            ],
            "construction-core source-shaped authority escaped its explicit compiler boundary"
        );

        let english_src = workspace_root.join("crates/deckmaste_english_v2/src");
        let runtime_paths = discover_rust_sources(&english_src);
        let runtime_files = parse_rust_sources(&runtime_paths);

        for (path, file) in &runtime_files {
            let violations = english_runtime_authority_violations(path, file, &closed_lexemes);
            assert!(violations.is_empty(), "{}", violations.join("\n"));
        }

        let xtask_src = workspace_root.join("crates/xtask/src");
        let mut xtask_paths = discover_rust_sources(&xtask_src.join("english_v2"));
        xtask_paths.push(xtask_src.join("english_v2.rs"));
        xtask_paths.sort();
        xtask_paths.dedup();
        let xtask_files = parse_rust_sources(&xtask_paths);
        for (path, file) in runtime_files.iter().chain(&xtask_files) {
            for catalog_authority in [
                concat!("Parser", "Catalogs"),
                concat!("Catalog", "Set"),
                concat!("Catalog", "Kind"),
                concat!("Catalog", "Identity"),
                concat!("deckmaste_", "catalogs"),
            ] {
                assert!(
                    !contains_production_identifier(file, catalog_authority),
                    "{path} retains catalog authority {catalog_authority}"
                );
            }
        }
        for (path, file) in &xtask_files {
            assert!(
                !contains_production_code_indirection(file),
                "{path} uses production #[path] or code-including include! indirection"
            );
        }

        let verbs = expansion
            .terminal_contributions()
            .iter()
            .find(|terminal| terminal.name() == "VerbLexeme")
            .expect("closed VerbLexeme provider exists")
            .variants()
            .iter()
            .map(deckmaste_construction_core::TerminalVariantContribution::name)
            .collect::<Vec<_>>();
        assert_eq!(verbs, ["Deal", "Gain", "Control", "Be"]);

        let adversarial = syn::parse_file(
            "mod nested { enum Noun { Mirror } }\n\
             struct Scanner;\n\
             impl Scanner { fn scan_signed_number(&self) {} }\n\
             mod morphology_shadow {\n\
                 fn inflect() {}\n\
                 struct BoundScanner;\n\
                 impl BoundScanner { fn scan_verb(&self) {} }\n\
                 fn wrappers() { generated!({ fn scan_bound_terminal() {} }); }\n\
                 const CLOSED_OWNER: &str = \"lexeme:VerbLexeme/Deal\";\n\
                 static CLOSED_SURFACES: &[(VerbLexeme, Agreement, &str)] =\n\
                     &[(VerbLexeme::Deal, Agreement::Bare, \"deal\")];\n\
                 fn surface(lexeme: VerbLexeme, agreement: Agreement) -> &'static str {\n\
                     match (lexeme, agreement) {\n\
                         (VerbLexeme::Deal, Agreement::Bare) => \"deal\",\n\
                         _ => \"deals\",\n\
                     }\n\
                 }\n\
                 fn macro_surface() {\n\
                     generated!([(NounLexeme::Player, Number::Singular, \"player\")]);\n\
                 }\n\
             }\n\
             fn macro_shadow() { helper!(({ scan_noun }), [walk_declaration_noun]); }\n\
             fn source_reader(tokens: Tokens) { crate::parse_declarations(tokens); }\n\
             #[cfg(test)] mod decoy {\n\
                 enum Lexical { Mirror }\n\
                 fn scan_noun() {}\n\
                 fn inflect() {}\n\
                 fn scan_verb() {}\n\
                 fn scan_bound_terminal() {}\n\
                 const CLOSED_OWNER: &str = \"lexeme:NounLexeme/Player\";\n\
                 static CLOSED_SURFACES: &[(NounLexeme, Number, &str)] =\n\
                     &[(NounLexeme::Player, Number::Singular, \"player\")];\n\
             }\n\
             fn later_production() { helper!({ render_noun }); }",
        )
        .expect("adversarial authority source reparses");
        assert!(contains_production_authority(
            &adversarial,
            ProductionAuthorityKind::Enum,
            "Noun"
        ));
        assert!(contains_production_function(
            &adversarial,
            "scan_signed_number"
        ));
        for function in ["scan_noun", "walk_declaration_noun", "render_noun"] {
            assert!(contains_production_function(&adversarial, function));
        }
        assert!(contains_production_identifier(
            &adversarial,
            "parse_declarations"
        ));
        assert!(!contains_production_authority(
            &adversarial,
            ProductionAuthorityKind::Enum,
            "Lexical"
        ));
        let violations =
            english_runtime_authority_violations("nested.rs", &adversarial, &closed_lexemes);
        for forbidden in ["inflect", "scan_verb", "scan_bound_terminal"] {
            assert!(
                violations
                    .iter()
                    .any(|violation| violation.contains(forbidden)),
                "missing `{forbidden}` violation in {violations:#?}"
            );
        }
        assert!(
            violations
                .iter()
                .any(|violation| violation.contains("closed-lexeme surface mirror")),
            "missing closed-surface violation in {violations:#?}"
        );
        assert!(
            violations
                .iter()
                .any(|violation| violation.contains("two-segment closed lexeme owner")),
            "missing owner violation in {violations:#?}"
        );
        for (shape, source) in [
            (
                "array",
                "static SURFACES: &[(VerbLexeme, Agreement, &str)] = \
                 &[(VerbLexeme::Deal, Agreement::Bare, \"deal\")];",
            ),
            (
                "match",
                "fn surface(value: VerbLexeme) -> &'static str { \
                 match value { VerbLexeme::Deal => \"deal\" } }",
            ),
            (
                "macro",
                "fn surface() { \
                 generated!([(NounLexeme::Player, Number::Singular, \"player\")]); }",
            ),
            (
                "multi-argument macro",
                "fn surface() { \
                 generated!(mode, [(NounLexeme::Player, Number::Singular, \"player\")]); }",
            ),
        ] {
            let sentinel = syn::parse_file(source)
                .unwrap_or_else(|error| panic!("{shape} surface sentinel reparses: {error}"));
            let violations =
                english_runtime_authority_violations(shape, &sentinel, &closed_lexemes);
            assert!(
                violations
                    .iter()
                    .any(|violation| violation.contains("closed-lexeme surface mirror")),
                "missing isolated {shape} surface violation in {violations:#?}"
            );
        }

        let ghost =
            syn::parse_file("#[cfg(test)] mod build { pub use crate::constructions::build; }")
                .expect("test-only ghost source reparses");
        assert!(contains_test_only_generated_ghost(&ghost));
        let ordinary_test =
            syn::parse_file("#[cfg(test)] mod rules_tests { use crate::constructions::RULES; }")
                .expect("ordinary test consumer source reparses");
        assert!(!contains_test_only_generated_ghost(&ordinary_test));
    }

    #[test]
    fn lexeme_authority_detection_ignores_cfg_test_decoys_and_substring_lookalikes() {
        let decoys = syn::parse_file(
            "#[cfg(test)] mod tests {\n\
                 fn inflect() {}\n\
                 fn scan_verb() {}\n\
                 fn scan_bound_terminal() {}\n\
                 const OWNER: &str = \"lexeme:VerbLexeme/Deal\";\n\
                 static SURFACES: &[(VerbLexeme, Agreement, &str)] =\n\
                     &[(VerbLexeme::Deal, Agreement::Bare, \"deal\")];\n\
             }\n\
             fn scan_verb_count() -> usize { 0 }\n\
             const DESCRIPTION: &str = \"fn inflect and scan_bound_terminal\";\n\
             const FEATURED_OWNER: &str = \"lexeme:VerbLexeme/Deal/bare\";\n\
             fn macro_decoy() { generated!(NounLexeme::Player, \"player\"); }\n\
             enum Term { Lexeme }\n\
             fn ordinary(value: Term) -> &'static str {\n\
                 match value { Term::Lexeme => \"ordinary\" }\n\
             }",
        )
        .expect("authority decoys reparse");

        let expansion = expansion_from_source(PRODUCTION_SOURCE)
            .expect("production declaration expands from the sealed semantic plan");
        let closed_lexemes = closed_lexeme_declarations(&expansion);
        let violations =
            english_runtime_authority_violations("decoys.rs", &decoys, &closed_lexemes);
        assert!(violations.is_empty(), "{violations:#?}");
    }

    #[test]
    fn lexeme_authority_detection_uses_declared_names_without_a_name_suffix_convention() {
        let renamed_source = PRODUCTION_SOURCE.replace("VerbLexeme", "VerbStem");
        let expansion = expansion_from_source(&renamed_source)
            .expect("a nonconventionally named lexeme declaration expands");
        let closed_lexemes = closed_lexeme_declarations(&expansion);
        assert!(closed_lexemes.contains("VerbStem"));

        let mirror = syn::parse_file(
            "static SURFACES: &[(VerbStem, Agreement, &str)] = \
             &[(VerbStem::Deal, Agreement::Bare, \"deal\")]; \
             const OWNER: &str = \"lexeme:VerbStem/Deal\";",
        )
        .expect("nonconventional lexeme mirror reparses");
        let violations =
            english_runtime_authority_violations("mirror.rs", &mirror, &closed_lexemes);
        assert!(
            violations
                .iter()
                .any(|violation| violation.contains("closed-lexeme surface mirror")),
            "missing declared-name surface guard in {violations:#?}",
        );
        assert!(
            violations
                .iter()
                .any(|violation| violation.contains("two-segment closed lexeme owner")),
            "missing declared-name owner guard in {violations:#?}",
        );
    }

    #[test]
    fn generated_terminal_items_are_all_classified_by_the_audit() {
        let expansion = expansion_from_source(PRODUCTION_SOURCE)
            .expect("production declaration expands from the sealed semantic plan");
        let expected = expected_terminal_items(&expansion);
        assert!(
            expected
                .iter()
                .any(|contribution| contribution.origin == "vocab TriggerWord"),
            "the plan-derived audit must include vocab terminals",
        );
        assert!(
            expected
                .iter()
                .any(|contribution| contribution.origin == "lexeme VerbLexeme"),
            "the plan-derived audit must include lexeme terminals",
        );
        let failures = audit_expected_terminal_items(&expected, expansion.items(), |item_name| {
            expected_production_origins(item_name)
                .map(|origins| origins.iter().map(|origin| (*origin).to_owned()).collect())
        });

        assert!(failures.is_empty(), "{}", failures.join("\n"));
    }

    #[test]
    fn generated_terminal_items_are_all_classified_only_after_emission_and_origin_classification() {
        let expansion = expansion_from_source(PRODUCTION_SOURCE)
            .expect("production declaration expands from the sealed semantic plan");
        let origin = expansion
            .terminal_contributions()
            .iter()
            .find(|contribution| contribution.name() == "VerbLexeme")
            .expect("synthetic mutation has a lexeme contribution")
            .origin()
            .clone();
        let origin_label = format!("{} {}", declaration_kind_name(origin.kind()), origin.name());
        let synthetic_key = ItemKey::Named {
            kind: NamedKind::Function,
            name: "synthetic_verb_lexeme_item".to_owned(),
        };
        let mut expected = expected_terminal_items(&expansion);
        expected
            .iter_mut()
            .find(|contribution| contribution.origin == origin_label)
            .expect("synthetic mutation finds the projected contribution")
            .expected_generated_item_keys
            .push(synthetic_key.clone());
        let production_classifier = |item_name: &str| {
            expected_production_origins(item_name)
                .map(|origins| origins.iter().map(|origin| (*origin).to_owned()).collect())
        };

        let failures =
            audit_expected_terminal_items(&expected, expansion.items(), production_classifier);
        assert_eq!(failures.len(), 2, "{failures:#?}");
        assert!(failures.iter().any(|failure| {
            failure.contains("function synthetic_verb_lexeme_item")
                && failure.contains("lexeme VerbLexeme")
                && failure.contains("missing generated terminal item")
        }));
        assert!(failures.iter().any(|failure| {
            failure.contains("function synthetic_verb_lexeme_item")
                && failure.contains("lexeme VerbLexeme")
                && failure.contains("missing explicit origin classification")
        }));

        let mut actual = expansion.items().to_vec();
        actual.push(deckmaste_construction_core::GeneratedItem {
            key: synthetic_key,
            tokens: proc_macro2::TokenStream::new(),
            origins: vec![origin],
        });
        let failures = audit_expected_terminal_items(&expected, &actual, production_classifier);
        assert_eq!(failures.len(), 1, "{failures:#?}");
        assert!(failures[0].contains("missing explicit origin classification"));

        let failures = audit_expected_terminal_items(&expected, &actual, |item_name| {
            if item_name == "function synthetic_verb_lexeme_item" {
                Some(vec!["lexeme VerbLexeme".to_owned()])
            } else {
                production_classifier(item_name)
            }
        });
        assert!(failures.is_empty(), "{failures:#?}");
    }

    #[test]
    fn production_expansion_prints_each_literal_item_key_once_with_every_origin() {
        let output = expand_source(PRODUCTION_SOURCE).expect("production declaration expands");
        let headings = output
            .lines()
            .filter_map(|line| line.strip_prefix("// === ")?.strip_suffix(" ==="))
            .filter(|heading| *heading != "counted escape hatches")
            .collect::<Vec<_>>();

        assert_eq!(EXPECTED_ITEM_KEYS.len(), 143);
        assert_eq!(headings, EXPECTED_ITEM_KEYS);
        for expected_key in EXPECTED_ITEM_KEYS {
            let header = format!("// === {expected_key} ===");
            assert_eq!(output.matches(&header).count(), 1, "{expected_key}");
            let after_header = output
                .split_once(&header)
                .expect("literal item header exists")
                .1;
            let item_section = after_header
                .split_once("// === ")
                .map_or(after_header, |(section, _)| section);
            let actual_origins = item_section
                .lines()
                .filter_map(|line| line.strip_prefix("// origin: "))
                .collect::<Vec<_>>();
            let expected_origins = expected_production_origins(expected_key).unwrap_or_else(|| {
                panic!("missing explicit production origin oracle for {expected_key}")
            });
            assert_eq!(actual_origins, expected_origins, "{expected_key}");
        }
    }

    #[test]
    fn production_counted_escape_hatches_have_exact_literal_names_and_order() {
        let output = expand_source(PRODUCTION_SOURCE).expect("production declaration expands");
        let report = output
            .split_once("// === counted escape hatches ===\n")
            .expect("counted report heading")
            .1;
        assert_eq!(
            report,
            "// morphology irregulars (1)\n\
             // - lexeme:VerbLexeme/Be\n\
             //   - bare = \"are\"\n\
             //   - third_person_singular = \"is\"\n\
             // terminal bindings (0)\n\
             // roots (2)\n\
             // - root Ability\n\
             // - root Sentence\n"
        );
    }

    #[test]
    fn expansion_is_stable_and_the_generated_rust_reparses() {
        let first = expand_source(PRODUCTION_SOURCE).expect("production declaration expands");
        let second = expand_source(PRODUCTION_SOURCE).expect("repeat expands");
        assert_eq!(first, second);

        let parsed = syn::parse_file(&first).expect("comment headings preserve reparsable Rust");
        assert_eq!(parsed.items.len(), 143);
    }

    #[test]
    fn handwritten_signed_decimal_authorities_are_absent_from_complete_sources() {
        let ast = syn::parse_file(include_str!("../../deckmaste_english_v2/src/ast.rs"))
            .expect("complete AST source reparses");
        let scan = syn::parse_file(include_str!(
            "../../deckmaste_english_v2/src/parser/scan.rs"
        ))
        .expect("complete scanner source reparses");
        let render = syn::parse_file(include_str!("../../deckmaste_english_v2/src/render.rs"))
            .expect("complete renderer source reparses");

        assert!(!contains_production_authority(
            &ast,
            ProductionAuthorityKind::Enum,
            "Sign"
        ));
        assert!(!contains_production_authority(
            &ast,
            ProductionAuthorityKind::Struct,
            "SignedNumber"
        ));
        assert!(!contains_production_function(&scan, "scan_signed_number"));
        assert!(!contains_production_function(
            &render,
            "render_signed_number"
        ));
    }

    #[test]
    fn handwritten_context_identity_authorities_are_absent_from_complete_sources() {
        let ast = syn::parse_file(include_str!("../../deckmaste_english_v2/src/ast.rs"))
            .expect("complete AST source reparses");
        let scan = syn::parse_file(include_str!(
            "../../deckmaste_english_v2/src/parser/scan.rs"
        ))
        .expect("complete scanner source reparses");
        let render = syn::parse_file(include_str!("../../deckmaste_english_v2/src/render.rs"))
            .expect("complete renderer source reparses");
        let visit = syn::parse_file(include_str!("../../deckmaste_english_v2/src/visit.rs"))
            .expect("complete visitor source reparses");

        assert!(!contains_production_authority(
            &ast,
            ProductionAuthorityKind::Enum,
            "SelfReferenceSpelling"
        ));
        for source in [&scan, &render] {
            for forbidden in [
                "SelfReferenceSpelling",
                "card_name",
                "abbreviated_card_name",
                "surface",
                "valid_in",
            ] {
                assert!(!contains_production_identifier(source, forbidden));
            }
            assert!(!contains_production_method_call(source, "spelling"));
        }
        assert!(!contains_production_identifier(
            &visit,
            "SelfReferenceSpelling"
        ));
        assert!(!contains_production_function(
            &visit,
            "walk_self_reference_spelling"
        ));
        assert!(!contains_production_function(
            &visit,
            "visit_self_reference_spelling"
        ));

        let invocation = deckmaste_construction_core::invocation_from_source(PRODUCTION_SOURCE)
            .expect("production source has one direct macro invocation");
        let declarations = deckmaste_construction_core::parse_declarations(invocation.tokens)
            .expect("production declaration parses");
        let binding = declarations
            .declarations
            .iter()
            .find_map(|declaration| match declaration {
                deckmaste_construction_core::Declaration::Identity(binding)
                    if binding.name == "SelfReferenceSpelling" =>
                {
                    Some(binding)
                }
                _ => None,
            })
            .expect("production has SelfReferenceSpelling identity");
        assert!(binding.generated_identity.is_some());
        assert!(binding.generated.is_none());
        assert!(binding.lexical_variant.is_none());
        assert!(binding.render.is_none());
        assert!(binding.build.is_none());
        assert!(binding.traversal.variants.is_empty());
        assert!(binding.traversal.calls.is_empty());
    }

    #[test]
    fn handwritten_declaration_noun_authorities_are_absent_from_complete_sources() {
        let ast = syn::parse_file(include_str!("../../deckmaste_english_v2/src/ast.rs"))
            .expect("complete AST source reparses");
        let scan = syn::parse_file(include_str!(
            "../../deckmaste_english_v2/src/parser/scan.rs"
        ))
        .expect("complete scanner source reparses");
        let render = syn::parse_file(include_str!("../../deckmaste_english_v2/src/render.rs"))
            .expect("complete renderer source reparses");
        let visit = syn::parse_file(include_str!("../../deckmaste_english_v2/src/visit.rs"))
            .expect("complete visitor source reparses");

        assert!(!contains_production_authority(
            &ast,
            ProductionAuthorityKind::Enum,
            "Noun"
        ));
        assert!(!contains_production_authority(
            &ast,
            ProductionAuthorityKind::Struct,
            "DeclarationNoun"
        ));
        for function in ["scan_noun", "noun_forms", "rendered_catalog"] {
            assert!(!contains_production_function(&scan, function));
        }
        for function in ["render_noun", "pluralize"] {
            assert!(!contains_production_function(&render, function));
        }
        assert!(!contains_production_function(
            &visit,
            "walk_declaration_noun"
        ));

        let invocation = deckmaste_construction_core::invocation_from_source(PRODUCTION_SOURCE)
            .expect("production source has one direct macro invocation");
        let declarations = deckmaste_construction_core::parse_declarations(invocation.tokens)
            .expect("production declaration parses");
        let binding = declarations
            .declarations
            .iter()
            .find_map(|declaration| match declaration {
                deckmaste_construction_core::Declaration::Codec(binding)
                    if binding.name == "Noun" =>
                {
                    Some(binding)
                }
                _ => None,
            })
            .expect("production has the generated Noun codec");
        assert!(binding.generated.is_some());
        assert!(binding.render.is_none());
        assert!(binding.build.is_none());
        assert!(binding.traversal.variants.is_empty());
        assert!(binding.traversal.calls.is_empty());
    }

    #[test]
    fn source_census_detects_nested_and_macro_declaration_noun_shadows() {
        let sentinel = syn::parse_file(
            "mod nested { enum Noun { Handwritten } }\n\
             fn scanner() { helper!(scan_noun); }\n\
             #[cfg(test)] fn render_noun() {}",
        )
        .expect("sentinel source reparses");
        assert!(contains_production_authority(
            &sentinel,
            ProductionAuthorityKind::Enum,
            "Noun"
        ));
        assert!(contains_production_identifier(&sentinel, "scan_noun"));
        assert!(!contains_production_function(&sentinel, "render_noun"));

        let test_only = syn::parse_file(
            "#[cfg(test)] mod tests {\n\
                 enum Noun { Handwritten }\n\
                 fn scanner() { helper!(scan_noun); }\n\
             }",
        )
        .expect("test-only sentinel reparses");
        assert!(!contains_production_identifier(&test_only, "Noun"));
        assert!(!contains_production_identifier(&test_only, "scan_noun"));
    }

    #[test]
    fn production_authority_predicate_detects_macro_token_function_shadows() {
        let sentinel = syn::parse_file(
            "fn authorities() { helper!((scan_noun), { render_noun }, [walk_declaration_noun], \"pluralize\"); }\n\
             #[cfg(test)] fn tests() { helper!(pluralize); }",
        )
        .expect("macro authority sentinel reparses");

        for function in ["scan_noun", "render_noun", "walk_declaration_noun"] {
            assert!(contains_production_function(&sentinel, function));
        }
        assert!(!contains_production_function(&sentinel, "pluralize"));
    }

    #[test]
    fn source_census_detects_nested_and_macro_context_identity_shadows() {
        let sentinel = syn::parse_file(
            "mod nested { struct Holder(SelfReferenceSpelling); }\n\
             fn scanner() { helper!(SelfReferenceSpelling); }\n\
             #[cfg(test)] mod tests { enum SelfReferenceSpelling { Full } }",
        )
        .expect("sentinel source reparses");
        assert!(contains_production_identifier(
            &sentinel,
            "SelfReferenceSpelling"
        ));

        let test_only = syn::parse_file(
            "#[cfg(test)] mod tests {\n\
                 fn scanner() { helper!(SelfReferenceSpelling); }\n\
             }\n\
             #[cfg(test)] enum SelfReferenceSpelling { Full }",
        )
        .expect("test-only sentinel reparses");
        assert!(!contains_production_identifier(
            &test_only,
            "SelfReferenceSpelling"
        ));

        let identifier_free_shadow = syn::parse_file(
            "fn render_shadow(value: Value, context: Context) {
                 value.spelling().surface(context);
                 context.card_name();
             }
             fn walk_self_reference_spelling() {}",
        )
        .expect("identifier-free authority sentinel reparses");
        assert!(!contains_production_identifier(
            &identifier_free_shadow,
            "SelfReferenceSpelling"
        ));
        assert!(contains_production_method_call(
            &identifier_free_shadow,
            "spelling"
        ));
        assert!(contains_production_identifier(
            &identifier_free_shadow,
            "surface"
        ));
        assert!(contains_production_identifier(
            &identifier_free_shadow,
            "card_name"
        ));
        assert!(contains_production_function(
            &identifier_free_shadow,
            "walk_self_reference_spelling"
        ));
    }

    #[test]
    fn source_census_detects_an_inherent_signed_decimal_scanner_shadow() {
        let sentinel = syn::parse_file(
            "struct Scanner;
             impl Scanner { fn scan_signed_number(&self) {} }
             mod nested {
                 enum Sign { Positive, Negative }
                 struct SignedNumber;
             }
             #[cfg(test)] mod tests {
                 fn render_signed_number() {}
             }",
        )
        .expect("sentinel source reparses");

        assert!(contains_production_function(
            &sentinel,
            "scan_signed_number"
        ));
        assert!(!contains_production_function(
            &sentinel,
            "render_signed_number"
        ));
        assert!(contains_production_authority(
            &sentinel,
            ProductionAuthorityKind::Enum,
            "Sign"
        ));
        assert!(contains_production_authority(
            &sentinel,
            ProductionAuthorityKind::Struct,
            "SignedNumber"
        ));
    }

    #[test]
    fn source_wiring_accepts_one_inline_invocation_and_preserves_core_diagnostics() {
        let output = expand_source(PRODUCTION_SOURCE).expect("one inline invocation is extracted");
        assert!(output.starts_with("// === type Ability ===\n"));
        let with_unrelated_macro = format!("helper!();\n{PRODUCTION_SOURCE}");
        assert_eq!(
            expand_source(&with_unrelated_macro).expect("unrelated macro is ignored"),
            output
        );

        for source in [
            "constructions! {} constructions! {}",
            "module::constructions! {}",
            "constructions!(include!(\"declarations.rs\"));",
        ] {
            let core_error = deckmaste_construction_core::invocation_from_source(source)
                .expect_err("core rejects fenced source")
                .to_string();
            let command_error = expand_source(source)
                .expect_err("xtask preserves core source error")
                .to_string();
            assert_eq!(command_error, core_error);
        }
    }

    #[test]
    fn validation_diagnostics_are_the_same_as_core() {
        let source = "constructions! { root Missing { punctuation = \".\"; eoi = true; standalone_render = true; } }";
        let invocation = deckmaste_construction_core::invocation_from_source(source).unwrap();
        let core_error = deckmaste_construction_core::generate(invocation.tokens)
            .expect_err("unknown root fails core validation")
            .to_string();
        let command_error = expand_source(source)
            .expect_err("unknown root fails xtask expansion")
            .to_string();
        assert_eq!(command_error, core_error);
    }

    #[test]
    fn file_runner_propagates_invalid_source_without_writing_output() {
        let directory = tempfile::tempdir().unwrap();
        let path = directory.path().join("constructions.rs");
        std::fs::write(&path, "constructions! {} constructions! {}").unwrap();
        let mut output = Vec::new();

        let error = run_from_path(&path, &mut output).expect_err("invalid input propagates");

        assert!(error.to_string().contains("exactly one"), "{error:#}");
        assert!(output.is_empty());
    }
}
