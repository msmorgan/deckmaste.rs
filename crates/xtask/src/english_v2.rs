//! Human-readable expansion of the English-v2 construction declaration.

mod ambiguity;
mod audit;
mod corpus;
mod coverage;
mod coverage_lock;
mod diagnostic;
mod inspect;
mod parse;
#[cfg(test)]
mod plan04_authority;
mod probe;
mod report;
mod roundtrip;
mod timing;

use std::fmt::Write as _;
use std::fs;
use std::io::Write;
use std::path::Path;
use std::path::PathBuf;

use anyhow::Context;
use clap::Args;
use clap::Subcommand;
use clap::ValueEnum;
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
    /// Run one named Plan-boundary command under its elapsed-time gate.
    PlanGate(timing::PlanGateArgs),
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
    #[arg(long, value_enum, default_value_t = ProbeRoot::Ability)]
    root: ProbeRoot,
    #[arg(long)]
    json: bool,
}

#[derive(Debug, Clone, Copy, Default, PartialEq, Eq, ValueEnum)]
enum ProbeRoot {
    #[default]
    Ability,
    Sentence,
    OracleText,
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
        EnglishV2Command::PlanGate(args) => {
            let mut stdout = std::io::stdout().lock();
            timing::run(args, &mut stdout)
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
        DeclarationKind::AbstractProduct => "abstract product",
        DeclarationKind::AbstractSum => "abstract sum",
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

    #[derive(Debug, clap::Parser)]
    struct ProbeCli {
        #[command(flatten)]
        args: ProbeArgs,
    }

    const PRODUCTION_SOURCE: &str = include_str!("../../deckmaste_english_v2/src/constructions.rs");
    const PLAN06_COVERED_IDS: &str = include_str!("english_v2/plan06_covered_ids.txt");
    const PLAN07_TARGETS: &str = include_str!("english_v2/plan07_targets.tsv");
    type ClosedLexemeDeclarations = std::collections::BTreeSet<String>;

    fn sha256_hex(bytes: &[u8]) -> String {
        use std::fmt::Write as _;

        use sha2::Digest as _;

        sha2::Sha256::digest(bytes)
            .iter()
            .fold(String::new(), |mut hexadecimal, byte| {
                write!(&mut hexadecimal, "{byte:02x}").expect("writing to String cannot fail");
                hexadecimal
            })
    }

    #[test]
    fn plan07_target_manifest_authenticates_frozen_classifier_evidence() {
        const EXPECTED_HEADERS: [&str; 7] = [
            "# English v2 Plan 07 frozen corpus target manifest",
            "# classifier_schema=1",
            "# source_fingerprint=e85359d7b8c578df13dff2fdf7c743a520a5b367d5ed25ab0a5f03cb8b3637dd",
            "# baseline_covered=412",
            "# classified=145",
            "# baseline_status=parse_failure",
            "# columns=category\\tid\\tcard_name\\tface_name\\tside\\tcontext_name\\ttext",
        ];
        const EXPECTED_COUNTS: [(&str, usize); 13] = [
            ("damage.any-target", 25),
            ("damage.binary-target-coordination", 8),
            ("damage.each-bare-head", 7),
            ("damage.one-prenominal-modifier", 2),
            ("destroy.all-bare-plural", 16),
            ("destroy.all-binary-coordination", 4),
            ("destroy.all-one-modifier", 12),
            ("destroy.binary-target-coordination", 17),
            ("destroy.fixed-or-x-target-count", 7),
            ("destroy.one-prenominal-modifier", 31),
            ("destroy.oxford-target-list", 5),
            ("destroy.simple-power-toughness-comparison", 9),
            ("destroy.target-bare-carrier", 2),
        ];

        assert!(PLAN07_TARGETS.ends_with("\n\n"));
        assert_eq!(
            PLAN07_TARGETS.lines().take(7).collect::<Vec<_>>(),
            EXPECTED_HEADERS,
        );
        assert_eq!(
            sha256_hex(PLAN07_TARGETS.as_bytes()),
            "3535a10fd5bcecc86dee14c1df28d4f66478f724b0dc465945063a6efb01723d",
        );

        let rows = PLAN07_TARGETS
            .lines()
            .skip(7)
            .filter(|line| !line.is_empty())
            .map(|line| line.split('\t').collect::<Vec<_>>())
            .collect::<Vec<_>>();
        assert_eq!(rows.len(), 145);

        assert_eq!(
            sha256_hex(PLAN06_COVERED_IDS.as_bytes()),
            "b611c795eb5dd6ede2c4dd9394e0dbed6154258992e72f6f618b12ffd6dace01",
        );
        assert_eq!(
            PLAN06_COVERED_IDS.lines().take(5).collect::<Vec<_>>(),
            [
                "# English v2 Plan 06 frozen covered-ID membership",
                "# fixture_schema=1",
                "# source_fingerprint=e85359d7b8c578df13dff2fdf7c743a520a5b367d5ed25ab0a5f03cb8b3637dd",
                "# covered=412",
                "# columns=id",
            ],
        );
        assert!(PLAN06_COVERED_IDS.ends_with("\n\n"));
        let baseline_ids = PLAN06_COVERED_IDS
            .lines()
            .skip(6)
            .filter(|line| !line.is_empty())
            .collect::<std::collections::BTreeSet<_>>();
        assert_eq!(baseline_ids.len(), 412);
        assert!(baseline_ids.iter().all(|id| {
            id.len() == 64
                && id
                    .bytes()
                    .all(|byte| byte.is_ascii_hexdigit() && !byte.is_ascii_uppercase())
        }));
        assert_eq!(
            baseline_ids.iter().copied().collect::<Vec<_>>(),
            PLAN06_COVERED_IDS
                .lines()
                .skip(6)
                .filter(|line| !line.is_empty())
                .collect::<Vec<_>>(),
            "frozen Plan 06 IDs must be strictly sorted and unique",
        );

        let mut previous_id: Option<&str> = None;
        let mut counts = std::collections::BTreeMap::new();
        let mut destroy = 0;
        let mut damage = 0;
        let lock: serde_json::Value =
            serde_json::from_str(include_str!("../../../english-v2-coverage.lock"))
                .expect("the integrated coverage lock parses");
        let covered = lock["covered"]
            .as_array()
            .expect("schema-2 coverage lock has covered identities")
            .iter()
            .map(|id| id.as_str().expect("covered identity is a string"))
            .collect::<std::collections::BTreeSet<_>>();
        assert!(covered.len() >= 412);
        assert!(
            baseline_ids.is_subset(&covered),
            "the evolving coverage lock must retain every frozen Plan 06 identity",
        );

        for fields in rows {
            assert_eq!(fields.len(), 7, "manifest row has complete metadata");
            let [
                category,
                id,
                card_name,
                _face_name,
                _side,
                context_name,
                text,
            ] = fields.as_slice()
            else {
                unreachable!("the seven-column assertion just succeeded")
            };
            assert_eq!(id.len(), 64);
            assert!(
                id.bytes()
                    .all(|byte| byte.is_ascii_hexdigit() && !byte.is_ascii_uppercase())
            );
            if let Some(previous) = previous_id {
                assert!(
                    previous < *id,
                    "manifest IDs must be strictly sorted and unique"
                );
            }
            previous_id = Some(id);
            assert!(!card_name.is_empty());
            assert!(!context_name.is_empty());
            assert!(!text.is_empty());
            assert!(
                !baseline_ids.contains(id),
                "Plan 07 target overlaps Plan 06"
            );
            *counts.entry(*category).or_insert(0usize) += 1;
            if category.starts_with("destroy.") {
                destroy += 1;
            } else if category.starts_with("damage.") {
                damage += 1;
            }
        }

        assert_eq!(counts.into_iter().collect::<Vec<_>>(), EXPECTED_COUNTS);
        assert_eq!((destroy, damage), (103, 42));
    }

    fn parse_probe_args(root: Option<&str>, json: bool, text: &str) -> ProbeArgs {
        use clap::Parser as _;

        let mut arguments = vec![
            "probe".to_owned(),
            "--text".to_owned(),
            text.to_owned(),
            "--context".to_owned(),
            "Probe Card".to_owned(),
        ];
        if let Some(root) = root {
            arguments.extend(["--root".to_owned(), root.to_owned()]);
        }
        if json {
            arguments.push("--json".to_owned());
        }
        ProbeCli::try_parse_from(arguments)
            .expect("the exact supported probe root spelling parses")
            .args
    }

    #[test]
    fn probe_root_accepts_only_the_three_exact_spellings_and_defaults_to_ability() {
        use clap::Parser as _;

        for root in [None, Some("ability"), Some("sentence"), Some("oracle-text")] {
            let _ = parse_probe_args(root, false, "Destroy target creature.");
        }

        for invalid in ["oracle_text", "OracleText", "document", ""] {
            let arguments = [
                "probe",
                "--text",
                "Destroy target creature.",
                "--context",
                "Probe Card",
                "--root",
                invalid,
            ];
            assert!(
                ProbeCli::try_parse_from(arguments).is_err(),
                "invalid root spelling {invalid:?} must be a Clap error"
            );
        }
    }

    #[test]
    fn probe_root_dispatch_names_the_single_selected_root_in_json_and_human_output() {
        let fixtures = [
            (None, "Destroy target creature.", "Ability"),
            (Some("ability"), "Destroy target creature.", "Ability"),
            (Some("sentence"), "Destroy target creature.", "Sentence"),
            (
                Some("oracle-text"),
                "Destroy target creature.\nYou gain 2 life.",
                "OracleText",
            ),
        ];

        for (root, text, expected) in fixtures {
            let mut json = Vec::new();
            probe::run(&parse_probe_args(root, true, text), &mut json)
                .expect("typed probe JSON command succeeds");
            let json: serde_json::Value = serde_json::from_slice(&json).unwrap();
            assert_eq!(json["root"], expected, "root selection {root:?}");

            let mut human = Vec::new();
            probe::run(&parse_probe_args(root, false, text), &mut human)
                .expect("typed probe human command succeeds");
            let human = String::from_utf8(human).unwrap();
            assert!(
                human
                    .lines()
                    .next()
                    .unwrap()
                    .contains(&format!("root={expected}")),
                "root selection {root:?}: {human}"
            );
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
                constant_string_concat_tokens(expression.mac.tokens.clone())
            }
            syn::Expr::Group(expression) => constant_string_expression(&expression.expr),
            syn::Expr::Paren(expression) => constant_string_expression(&expression.expr),
            _ => None,
        }
    }

    fn constant_string_concat_tokens(tokens: proc_macro2::TokenStream) -> Option<String> {
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
                && let Some(spelling) = constant_string_concat_tokens(group.stream())
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
                && let Some(spelling) = constant_string_concat_tokens(mac.tokens.clone())
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
        "construction paragraph",
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
        "abstract sum DocumentBlock",
        "abstract product OracleText",
        "root Ability",
        "root Sentence",
        "root OracleText",
    ];

    const CONSTRUCTION_ORIGINS: &[&str] = &[
        "construction paragraph",
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
        "root OracleText",
    ];

    const VISITOR_ORIGINS: &[&str] = &[
        "construction paragraph",
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
        "construction paragraph",
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
                &["construction paragraph", "construction triggered"]
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
            "type DocumentBlock"
            | "function render_document_block"
            | "function walk_document_block" => &["abstract sum DocumentBlock"],
            "type OracleText"
            | "impl OracleText"
            | "function render_oracle_text_blocks_sequence"
            | "function render_oracle_text"
            | "function walk_oracle_text_blocks_sequence"
            | "function walk_oracle_text" => &["abstract product OracleText"],
            "type Paragraph" | "impl Paragraph" | "function walk_paragraph" => {
                &["construction paragraph"]
            }
            "type Triggered" | "impl Triggered" | "function walk_triggered" => {
                &["construction triggered"]
            }
            "function render_paragraph_sentences_sequence"
            | "function walk_paragraph_sentences_sequence" => &["construction Paragraph"],
            "function render_triggered_effects_sequence"
            | "function walk_triggered_effects_sequence" => &["construction Triggered"],
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
            | "type PrefixPosition"
            | "type StructuralTransition"
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
            | "type BuildViolation"
            | "impl BuildViolation"
            | "type BuildRejection"
            | "impl BuildRejection for BuildRejection"
            | "impl std::fmt::Display for BuildRejection"
            | "type BuildValue"
            | "type NonterminalCategory"
            | "impl NonterminalCategory for NonterminalCategory"
            | "impl std::fmt::Display for NonterminalCategory"
            | "impl Category for Category"
            | "impl From<Category> for NonterminalCategory"
            | "trait GeneratedRoot"
            | "trait GeneratedParseRoot"
            | "impl GeneratedRoot for Ability"
            | "impl GeneratedRoot for Sentence"
            | "impl GeneratedRoot for OracleText"
            | "impl GeneratedParseRoot for Ability"
            | "impl GeneratedParseRoot for Sentence"
            | "impl GeneratedParseRoot for OracleText"
            | "impl StructuralTransition for StructuralTransition"
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
            | "constant REQUIRED_DECLARATIONS"
            | "type SequenceOwner"
            | "type FixedSurfaceAtom"
            | "function sequence_separator"
            | "function sequence_terminator" => ALL_DECLARATION_ORIGINS,
            "function scan_lexical" => SCANNER_ORIGINS,
            "impl Ability" | "impl Render for Ability" | "function render_ability_with_claims" => {
                &["root Ability"]
            }
            "impl Sentence"
            | "impl Render for Sentence"
            | "function render_sentence_with_claims" => &["root Sentence"],
            "function write_oracle_text_render"
            | "impl Render for OracleText"
            | "function render_oracle_text_with_claims" => &["root OracleText"],
            "function render_ability_body" => &["construction paragraph", "construction triggered"],
            "trait Visitor" => VISITOR_ORIGINS,
            "function walk_self_reference_spelling" => &["identity SelfReferenceSpelling"],
            "type Category" => &[
                "construction paragraph",
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
                "abstract sum DocumentBlock",
                "abstract product OracleText",
            ],
            "type Construction"
            | "impl Construction"
            | "type RuleId"
            | "impl RuleId"
            | "function build_checked"
            | "function build" => CONSTRUCTION_ORIGINS,
            "impl Category" => &["root Ability", "root Sentence", "root OracleText"],
            "constant RULES" => &[
                "construction paragraph",
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
                "root OracleText",
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
        "type DocumentBlock",
        "type OracleText",
        "type Paragraph",
        "impl Paragraph",
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
        "type PrefixPosition",
        "type StructuralTransition",
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
        "type BuildViolation",
        "impl BuildViolation",
        "type BuildRejection",
        "impl BuildRejection for BuildRejection",
        "impl std::fmt::Display for BuildRejection",
        "type BuildValue",
        "type NonterminalCategory",
        "impl NonterminalCategory for NonterminalCategory",
        "impl std::fmt::Display for NonterminalCategory",
        "impl Category for Category",
        "impl From<Category> for NonterminalCategory",
        "trait GeneratedRoot",
        "trait GeneratedParseRoot",
        "impl GeneratedRoot for Ability",
        "impl GeneratedRoot for Sentence",
        "impl GeneratedRoot for OracleText",
        "impl GeneratedParseRoot for Ability",
        "impl GeneratedParseRoot for Sentence",
        "impl GeneratedParseRoot for OracleText",
        "impl StructuralTransition for StructuralTransition",
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
        "type SequenceOwner",
        "type FixedSurfaceAtom",
        "function sequence_separator",
        "function sequence_terminator",
        "function render_oracle_text_blocks_sequence",
        "function render_oracle_text",
        "function render_paragraph_sentences_sequence",
        "function render_triggered_effects_sequence",
        "function render_document_block",
        "impl Ability",
        "impl Render for Ability",
        "function render_ability_with_claims",
        "impl Sentence",
        "impl Render for Sentence",
        "function render_sentence_with_claims",
        "function write_oracle_text_render",
        "impl Render for OracleText",
        "function render_oracle_text_with_claims",
        "function render_ability_body",
        "function render_sentence_body",
        "function render_clause",
        "function render_noun_phrase",
        "function render_verb_phrase",
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
        "function walk_paragraph_sentences_sequence",
        "function walk_triggered_effects_sequence",
        "function walk_document_block",
        "function walk_oracle_text_blocks_sequence",
        "function walk_oracle_text",
        "function walk_paragraph",
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
        "impl Construction",
        "impl Category",
        "type RuleId",
        "impl RuleId",
        "constant RULES",
        "function build_checked",
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

        assert_eq!(EXPECTED_ITEM_KEYS.len(), 189);
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
             // roots (3)\n\
             // - root Ability\n\
             // - root Sentence\n\
             // - root OracleText\n"
        );
    }

    #[test]
    fn expansion_is_stable_and_the_generated_rust_reparses() {
        let first = expand_source(PRODUCTION_SOURCE).expect("production declaration expands");
        let second = expand_source(PRODUCTION_SOURCE).expect("repeat expands");
        assert_eq!(first, second);

        let parsed = syn::parse_file(&first).expect("comment headings preserve reparsable Rust");
        assert_eq!(parsed.items.len(), 189);
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
                "valid_in",
            ] {
                assert!(!contains_production_identifier(source, forbidden));
            }
            assert!(!contains_production_method_call(source, "surface"));
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
