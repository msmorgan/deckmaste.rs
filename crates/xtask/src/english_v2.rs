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

use std::collections::BTreeMap;
use std::collections::BTreeSet;
use std::fmt::Write as _;
use std::fs;
use std::io::Write;
use std::path::Path;
use std::path::PathBuf;

use anyhow::Context;
use anyhow::ensure;
use clap::Args;
use clap::Subcommand;
use clap::ValueEnum;
use deckmaste_construction_core::DeclarationKind;
use deckmaste_construction_core::Expansion;
use deckmaste_construction_core::ItemKey;
use deckmaste_construction_core::NamedKind;
use deckmaste_construction_core::TerminalBindingDeclarationKind;
use deckmaste_english_v2::ast::CatalogProvider;
use deckmaste_english_v2::environment::CatalogProviderRow;
use deckmaste_english_v2::environment::CatalogProviderRows;
use deckmaste_english_v2::environment::ParserEnvironment;
use deckmaste_english_v2::parser::Parser;

const REQUIRE_COMPLETE_OUTCOME_ENV: &str = "DECKMASTE_ENGLISH_V2_REQUIRE_COMPLETE_OUTCOME";

const CARD_NAME_ONSET_OVERRIDES: [(&str, macro_ron::v2::Onset); 8] = [
    ("+2 Mace", macro_ron::v2::Onset::Consonant),
    ("Éomer of the Riddermark", macro_ron::v2::Onset::Vowel),
    ("Éomer, King of Rohan", macro_ron::v2::Onset::Vowel),
    ("Éomer, Marshal of Rohan", macro_ron::v2::Onset::Vowel),
    ("Éowyn, Fearless Knight", macro_ron::v2::Onset::Vowel),
    ("Éowyn, Lady of Rohan", macro_ron::v2::Onset::Vowel),
    ("Éowyn, Shieldmaiden", macro_ron::v2::Onset::Vowel),
    ("Óin the Brave", macro_ron::v2::Onset::Vowel),
];

fn catalog_surface_onset(surface: &str) -> Option<macro_ron::v2::Onset> {
    macro_ron::v2::normalize_surface_onset(surface, None).or_else(|| {
        CARD_NAME_ONSET_OVERRIDES
            .iter()
            .find_map(|(name, onset)| (*name == surface).then_some(*onset))
    })
}

fn ensure_card_name_onset_override_inventory<'a>(
    names: impl IntoIterator<Item = &'a str>,
    overrides: &[(&str, macro_ron::v2::Onset)],
) -> anyhow::Result<()> {
    let derived_exceptional = names
        .into_iter()
        .filter(|name| macro_ron::v2::normalize_surface_onset(name, None).is_none())
        .collect::<BTreeSet<_>>();
    let reviewed_exceptional = overrides
        .iter()
        .map(|(name, _)| *name)
        .collect::<BTreeSet<_>>();
    ensure!(
        reviewed_exceptional.len() == overrides.len(),
        "card-name onset override inventory contains a duplicate surface",
    );
    ensure!(
        derived_exceptional == reviewed_exceptional,
        "card-name onset override inventory differs from generated catalog exceptions: derived {derived_exceptional:?}, reviewed {reviewed_exceptional:?}",
    );
    Ok(())
}

#[derive(Debug)]
struct AdaptedCardNameCatalog {
    provider: CatalogProviderRows,
    context_onsets: BTreeMap<String, macro_ron::v2::Onset>,
}

fn adapt_card_name_catalog_provider(catalog_root: &Path) -> anyhow::Result<AdaptedCardNameCatalog> {
    let catalogs = deckmaste_catalogs::CatalogSet::load(catalog_root).with_context(|| {
        format!(
            "loading generated canonical catalogs from {}",
            catalog_root.display()
        )
    })?;
    let card_names = catalogs.get(deckmaste_catalogs::CatalogKind::CardNames);
    ensure_card_name_onset_override_inventory(
        card_names.iter().map(String::as_str),
        &CARD_NAME_ONSET_OVERRIDES,
    )?;

    let mut context_onsets = BTreeMap::new();
    let rows = card_names
        .iter()
        .map(|name| {
            let onset = catalog_surface_onset(name).with_context(|| {
                format!("freezing card-name catalog surface with unknown onset: {name:?}")
            })?;
            context_onsets.insert(name.clone(), onset);
            Ok(CatalogProviderRow::new(name.as_str(), name.as_str(), onset))
        })
        .collect::<anyhow::Result<Vec<_>>>()?;
    Ok(AdaptedCardNameCatalog {
        provider: CatalogProviderRows::new(CatalogProvider::CardNames, rows),
        context_onsets,
    })
}

fn parser_from_builtin_v2() -> anyhow::Result<Parser> {
    let declarations = macro_ron::v2::read_builtin_v2(
        Path::new(env!("CARGO_MANIFEST_DIR")).join("../../plugins/builtin_v2"),
    )
    .context("loading integrated builtin-v2 declarations")?;
    let catalog_root = Path::new(env!("CARGO_MANIFEST_DIR")).join("../../data/gen/catalogs");
    let adapted = adapt_card_name_catalog_provider(&catalog_root)?;
    let environment = ParserEnvironment::try_from_parts(declarations, [adapted.provider])
        .context("freezing integrated builtin-v2 declarations and catalog providers")?;
    Parser::new(environment)
        .context("building builtin-v2 parser from generated environment requirements")
}

#[cfg(test)]
mod catalog_adapter_tests {
    use super::*;

    const EXPECTED_EXCEPTIONAL_ONSETS: [(&str, macro_ron::v2::Onset); 8] = [
        ("+2 Mace", macro_ron::v2::Onset::Consonant),
        ("Éomer of the Riddermark", macro_ron::v2::Onset::Vowel),
        ("Éomer, King of Rohan", macro_ron::v2::Onset::Vowel),
        ("Éomer, Marshal of Rohan", macro_ron::v2::Onset::Vowel),
        ("Éowyn, Fearless Knight", macro_ron::v2::Onset::Vowel),
        ("Éowyn, Lady of Rohan", macro_ron::v2::Onset::Vowel),
        ("Éowyn, Shieldmaiden", macro_ron::v2::Onset::Vowel),
        ("Óin the Brave", macro_ron::v2::Onset::Vowel),
    ];

    #[test]
    fn named_catalog_adapter_supplies_canonical_seven_dwarves_row() {
        let catalog_root = Path::new(env!("CARGO_MANIFEST_DIR")).join("../../data/gen/catalogs");
        let provider = adapt_card_name_catalog_provider(&catalog_root)
            .expect("generated catalog adapts without parser-side discovery");
        let environment = ParserEnvironment::try_from_parts([], [provider.provider])
            .expect("adapter rows freeze without parser-side discovery");

        assert_eq!(
            environment.catalog_surface(
                deckmaste_english_v2::ast::CatalogProvider::CardNames,
                "Seven Dwarves",
            ),
            Some("Seven Dwarves")
        );
        assert_eq!(
            environment.catalog_onset(
                deckmaste_english_v2::ast::CatalogProvider::CardNames,
                "Seven Dwarves",
            ),
            Some(macro_ron::v2::Onset::Consonant)
        );
    }

    #[test]
    fn named_catalog_adapter_exception_inventory_is_closed_and_exact() {
        let catalog_root = Path::new(env!("CARGO_MANIFEST_DIR")).join("../../data/gen/catalogs");
        let raw_names = std::fs::read_to_string(catalog_root.join("card-names.txt"))
            .expect("raw generated card-name rows load independently");
        let names = raw_names.lines().collect::<Vec<_>>();
        let expected_exceptional = EXPECTED_EXCEPTIONAL_ONSETS
            .iter()
            .map(|(name, _)| *name)
            .collect::<std::collections::BTreeSet<_>>();
        let reviewed_exceptional = CARD_NAME_ONSET_OVERRIDES
            .iter()
            .map(|(name, _)| *name)
            .collect::<std::collections::BTreeSet<_>>();
        let actual_exceptional = names
            .iter()
            .copied()
            .filter(|name| macro_ron::v2::normalize_surface_onset(name, None).is_none())
            .collect::<std::collections::BTreeSet<_>>();

        assert_eq!(names.len(), 32_548, "every raw generated row is counted");
        assert_eq!(actual_exceptional, expected_exceptional);
        assert_eq!(actual_exceptional, reviewed_exceptional);
        assert_eq!(
            catalog_surface_onset("+3 Mace"),
            None,
            "a symbol prefix cannot become an unreviewed wildcard",
        );
        assert_eq!(
            catalog_surface_onset("Éomer's Cousin"),
            None,
            "an accented initial cannot become an unreviewed Unicode-class fallback",
        );
    }

    #[test]
    fn production_inventory_guard_rejects_unreviewed_and_stale_exceptional_surfaces() {
        let unreviewed = ensure_card_name_onset_override_inventory(
            ["+2 Mace", "Éomer's Cousin"],
            &[("+2 Mace", macro_ron::v2::Onset::Consonant)],
        )
        .expect_err("an unreviewed exceptional surface must fail the production guard");
        assert!(format!("{unreviewed:#}").contains("Éomer's Cousin"));

        let stale = ensure_card_name_onset_override_inventory(
            ["+2 Mace"],
            &[
                ("+2 Mace", macro_ron::v2::Onset::Consonant),
                ("Óin the Brave", macro_ron::v2::Onset::Vowel),
            ],
        )
        .expect_err("a stale exceptional override must fail the production guard");
        assert!(format!("{stale:#}").contains("Óin the Brave"));
    }

    #[test]
    fn named_catalog_adapter_freezes_each_reviewed_exception_onset() {
        let catalog_root = Path::new(env!("CARGO_MANIFEST_DIR")).join("../../data/gen/catalogs");
        let adapted = adapt_card_name_catalog_provider(&catalog_root)
            .expect("generated card-name metadata freezes");
        for (name, onset) in EXPECTED_EXCEPTIONAL_ONSETS {
            assert_eq!(adapted.context_onsets.get(name), Some(&onset), "{name}");
        }
        let environment = ParserEnvironment::try_from_parts([], [adapted.provider])
            .expect("adapter rows freeze without parser-side discovery");
        for (name, onset) in EXPECTED_EXCEPTIONAL_ONSETS {
            assert_eq!(
                environment.catalog_onset(CatalogProvider::CardNames, name),
                Some(onset),
                "{name}",
            );
        }
    }

    #[test]
    fn named_catalog_adapter_propagates_context_for_catalog_load_failure() {
        let temporary = tempfile::tempdir().expect("temporary adapter root");
        let missing = temporary.path().join("missing-catalogs");

        let error = adapt_card_name_catalog_provider(&missing)
            .expect_err("missing catalog directory must be a typed command error");
        let rendered = format!("{error:#}");

        assert!(rendered.contains("loading generated canonical catalogs from"));
        assert!(rendered.contains(&missing.display().to_string()));
    }
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
    #[arg(long, value_enum)]
    onset: ProbeOnset,
    #[arg(long)]
    legendary: bool,
    #[arg(long, default_value_t = 256)]
    limit: usize,
    #[arg(long, value_enum, default_value_t = ProbeRoot::Ability)]
    root: ProbeRoot,
    #[arg(long)]
    json: bool,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, ValueEnum)]
enum ProbeOnset {
    Consonant,
    Vowel,
}

impl From<ProbeOnset> for macro_ron::v2::Onset {
    fn from(onset: ProbeOnset) -> Self {
        match onset {
            ProbeOnset::Consonant => Self::Consonant,
            ProbeOnset::Vowel => Self::Vowel,
        }
    }
}

impl From<macro_ron::v2::Onset> for ProbeOnset {
    fn from(onset: macro_ron::v2::Onset) -> Self {
        match onset {
            macro_ron::v2::Onset::Consonant => Self::Consonant,
            macro_ron::v2::Onset::Vowel => Self::Vowel,
        }
    }
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
    type ClosedLexemeDeclarations = std::collections::BTreeSet<String>;

    fn parse_probe_args(root: Option<&str>, json: bool, text: &str) -> ProbeArgs {
        use clap::Parser as _;

        let mut arguments = vec![
            "probe".to_owned(),
            "--text".to_owned(),
            text.to_owned(),
            "--context".to_owned(),
            "Probe Card".to_owned(),
            "--onset".to_owned(),
            "consonant".to_owned(),
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

    #[test]
    fn task10_removed_hardcoded_count_path_has_no_rust_source_residue() {
        let workspace_root = Path::new(env!("CARGO_MANIFEST_DIR")).join("../..");
        let mut paths = discover_rust_sources(&workspace_root.join("crates/deckmaste_english_v2"));
        let xtask_src = workspace_root.join("crates/xtask/src");
        paths.extend(discover_rust_sources(&xtask_src.join("english_v2")));
        paths.push(xtask_src.join("english_v2.rs"));
        paths.sort();
        paths.dedup();

        let forbidden_identifiers = [["Count", "Np"].concat(), ["count", "_np"].concat()];
        let count_construction = ["construction ", "count"].concat();
        let mut violations = Vec::new();
        for path in paths {
            let source = fs::read_to_string(&path)
                .unwrap_or_else(|error| panic!("{}: {error}", path.display()));
            for identifier in &forbidden_identifiers {
                if source.contains(identifier) {
                    violations.push(format!("{} contains {identifier}", path.display()));
                }
            }
            if source.match_indices(&count_construction).any(|(start, _)| {
                source[start + count_construction.len()..]
                    .chars()
                    .next()
                    .is_none_or(|next| !next.is_alphanumeric() && next != '_')
            }) {
                violations.push(format!(
                    "{} contains a construction named count",
                    path.display(),
                ));
            }
        }

        assert!(
            violations.is_empty(),
            "deleted hardcoded count-path residue:\n{}",
            violations.join("\n"),
        );
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

    struct AuditedTerminalContribution {
        origin: String,
        expected_generated_item_keys: Vec<ItemKey>,
    }

    fn expected_origin_classifier(
        expected: &[AuditedTerminalContribution],
    ) -> std::collections::BTreeMap<String, Vec<String>> {
        let mut classifier = std::collections::BTreeMap::<String, Vec<String>>::new();
        for contribution in expected {
            for key in &contribution.expected_generated_item_keys {
                let item_name = item_key_name(key);
                let origins = classifier.entry(item_name.clone()).or_default();
                assert!(
                    !origins.contains(&contribution.origin),
                    "terminal item `{item_name}` is classified twice for `{}`",
                    contribution.origin,
                );
                origins.push(contribution.origin.clone());
            }
        }
        classifier
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
        for (path, file) in &xtask_files {
            assert!(
                !contains_production_code_indirection(file),
                "{path} uses production #[path] or code-including include! indirection"
            );
        }

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
                     generated!([(CommonNoun::Player, Number::Singular, \"player\")]);\n\
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
                 const CLOSED_OWNER: &str = \"lexeme:CommonNoun/Player\";\n\
                 static CLOSED_SURFACES: &[(CommonNoun, Number, &str)] =\n\
                     &[(CommonNoun::Player, Number::Singular, \"player\")];\n\
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
                 generated!([(CommonNoun::Player, Number::Singular, \"player\")]); }",
            ),
            (
                "multi-argument macro",
                "fn surface() { \
                 generated!(mode, [(CommonNoun::Player, Number::Singular, \"player\")]); }",
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
             fn macro_decoy() { generated!(CommonNoun::Player, \"player\"); }\n\
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
        let classifier = expected_origin_classifier(&expected);
        assert!(
            expected
                .iter()
                .any(|contribution| contribution.origin == "vocab TriggerMarker"),
            "the plan-derived audit must include vocab terminals",
        );
        assert!(
            expected
                .iter()
                .any(|contribution| contribution.origin == "lexeme VerbLexeme"),
            "the plan-derived audit must include lexeme terminals",
        );
        let failures = audit_expected_terminal_items(&expected, expansion.items(), |item_name| {
            classifier.get(item_name).cloned()
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
        let mut classifier = expected_origin_classifier(&expected);
        expected
            .iter_mut()
            .find(|contribution| contribution.origin == origin_label)
            .expect("synthetic mutation finds the projected contribution")
            .expected_generated_item_keys
            .push(synthetic_key.clone());
        let production_classifier = |item_name: &str| classifier.get(item_name).cloned();

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

        classifier.insert(
            "function synthetic_verb_lexeme_item".to_owned(),
            vec!["lexeme VerbLexeme".to_owned()],
        );
        let failures = audit_expected_terminal_items(&expected, &actual, |item_name| {
            classifier.get(item_name).cloned()
        });
        assert!(failures.is_empty(), "{failures:#?}");
    }

    #[test]
    fn production_counted_escape_hatches_render_every_source_derived_row() {
        let expansion = expansion_from_source(PRODUCTION_SOURCE)
            .expect("production declaration expands from the sealed semantic plan");
        let output = render_expansion(&expansion).expect("production declaration renders");
        let report = output
            .split_once("// === counted escape hatches ===\n")
            .expect("counted report heading")
            .1;
        let escape_hatches = expansion.escape_hatches();
        assert!(report.starts_with(&format!(
            "// morphology irregulars ({})\n",
            escape_hatches.morphology_irregulars().len(),
        )));
        for irregular in escape_hatches.morphology_irregulars() {
            assert!(report.contains(&format!("// - {}\n", irregular.identity())));
            for row in irregular.overrides() {
                assert!(report.contains(&format!(
                    "//   - {} = {:?}\n",
                    row.feature(),
                    row.surface(),
                )));
            }
        }
        assert!(report.contains(&format!(
            "// terminal bindings ({})\n",
            escape_hatches.terminal_bindings().len(),
        )));
        assert!(report.contains(&format!("// roots ({})\n", escape_hatches.roots().len())));
        for root in escape_hatches.roots() {
            assert!(report.contains(&format!("// - root {root}\n")));
        }
    }

    #[test]
    fn expansion_is_stable_and_the_generated_rust_reparses() {
        let first = expand_source(PRODUCTION_SOURCE).expect("production declaration expands");
        let second = expand_source(PRODUCTION_SOURCE).expect("repeat expands");
        assert_eq!(first, second);

        let parsed = syn::parse_file(&first).expect("comment headings preserve reparsable Rust");
        let expansion = expansion_from_source(PRODUCTION_SOURCE)
            .expect("the same production declaration expands structurally");
        assert_eq!(parsed.items.len(), expansion.items().len());
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
        for name in [
            "TypeNoun",
            "DeclarationTypeNoun",
            "ArtifactSubtypeNoun",
            "DeclarationArtifactSubtypeNoun",
            "BattleSubtypeNoun",
            "DeclarationBattleSubtypeNoun",
            "CreatureSubtypeNoun",
            "DeclarationCreatureSubtypeNoun",
            "EnchantmentSubtypeNoun",
            "DeclarationEnchantmentSubtypeNoun",
            "LandSubtypeNoun",
            "DeclarationLandSubtypeNoun",
            "PlaneswalkerSubtypeNoun",
            "DeclarationPlaneswalkerSubtypeNoun",
            "SpellSubtypeNoun",
            "DeclarationSpellSubtypeNoun",
        ] {
            assert!(!contains_production_authority(
                &ast,
                ProductionAuthorityKind::Struct,
                name,
            ));
        }
        for function in ["scan_noun", "noun_forms", "rendered_catalog"] {
            assert!(!contains_production_function(&scan, function));
        }
        for function in ["render_noun", "pluralize"] {
            assert!(!contains_production_function(&render, function));
        }
        for function in [
            "walk_declaration_noun",
            "walk_declaration_type_noun",
            "walk_declaration_artifact_subtype_noun",
            "walk_declaration_battle_subtype_noun",
            "walk_declaration_creature_subtype_noun",
            "walk_declaration_enchantment_subtype_noun",
            "walk_declaration_land_subtype_noun",
            "walk_declaration_planeswalker_subtype_noun",
            "walk_declaration_spell_subtype_noun",
        ] {
            assert!(!contains_production_function(&visit, function));
        }

        let invocation = deckmaste_construction_core::invocation_from_source(PRODUCTION_SOURCE)
            .expect("production source has one direct macro invocation");
        let declarations = deckmaste_construction_core::parse_declarations(invocation.tokens)
            .expect("production declaration parses");
        let bindings = declarations
            .declarations
            .iter()
            .filter_map(|declaration| match declaration {
                deckmaste_construction_core::Declaration::Codec(binding)
                    if [
                        "TypeNoun",
                        "ArtifactSubtypeNoun",
                        "BattleSubtypeNoun",
                        "CreatureSubtypeNoun",
                        "EnchantmentSubtypeNoun",
                        "LandSubtypeNoun",
                        "PlaneswalkerSubtypeNoun",
                        "SpellSubtypeNoun",
                    ]
                    .iter()
                    .any(|name| binding.name == *name) =>
                {
                    Some(binding)
                }
                _ => None,
            })
            .collect::<Vec<_>>();
        assert_eq!(bindings.len(), 8, "production has every typed noun codec");
        for binding in bindings {
            assert!(matches!(
                binding.generated.as_ref(),
                Some(deckmaste_construction_core::GeneratedCodecRecipe::DeclarationNoun(
                    source
                )) if source.closed_slots.is_empty()
            ));
            assert!(binding.render.is_none());
            assert!(binding.build.is_none());
            assert!(binding.traversal.variants.is_empty());
            assert!(binding.traversal.calls.is_empty());
        }
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
        assert!(output.starts_with("// === type FiniteCondition ===\n"));
        let with_unrelated_macro = PRODUCTION_SOURCE.replacen(
            "use RulePosition::Lexical as L;",
            "helper!();\nuse RulePosition::Lexical as L;",
            1,
        );
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
