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
use deckmaste_construction_core::Expansion;
use deckmaste_construction_core::ItemKey;
use deckmaste_construction_core::NamedKind;
use deckmaste_construction_core::SourceDeclarationKind;
use deckmaste_construction_core::TerminalBindingDeclarationKind;
use deckmaste_english_v2::ast::CatalogProvider;
use deckmaste_english_v2::environment::CatalogProviderRow;
use deckmaste_english_v2::environment::CatalogProviderRows;
use deckmaste_english_v2::environment::ParserEnvironment;
use deckmaste_english_v2::parser::Parser;

const REQUIRE_COMPLETE_OUTCOME_ENV: &str = "DECKMASTE_ENGLISH_V2_REQUIRE_COMPLETE_OUTCOME";

const CARD_NAME_ONSET_OVERRIDES: [(&str, deckmaste_construction_core::macro_def::Onset); 8] = [
    (
        "+2 Mace",
        deckmaste_construction_core::macro_def::Onset::Consonant,
    ),
    (
        "Éomer of the Riddermark",
        deckmaste_construction_core::macro_def::Onset::Vowel,
    ),
    (
        "Éomer, King of Rohan",
        deckmaste_construction_core::macro_def::Onset::Vowel,
    ),
    (
        "Éomer, Marshal of Rohan",
        deckmaste_construction_core::macro_def::Onset::Vowel,
    ),
    (
        "Éowyn, Fearless Knight",
        deckmaste_construction_core::macro_def::Onset::Vowel,
    ),
    (
        "Éowyn, Lady of Rohan",
        deckmaste_construction_core::macro_def::Onset::Vowel,
    ),
    (
        "Éowyn, Shieldmaiden",
        deckmaste_construction_core::macro_def::Onset::Vowel,
    ),
    (
        "Óin the Brave",
        deckmaste_construction_core::macro_def::Onset::Vowel,
    ),
];

fn catalog_surface_onset(surface: &str) -> Option<deckmaste_construction_core::macro_def::Onset> {
    deckmaste_construction_core::macro_def::normalize_surface_onset(surface, None).or_else(|| {
        CARD_NAME_ONSET_OVERRIDES
            .iter()
            .find_map(|(name, onset)| (*name == surface).then_some(*onset))
    })
}

fn ensure_card_name_onset_override_inventory<'a>(
    names: impl IntoIterator<Item = &'a str>,
    overrides: &[(&str, deckmaste_construction_core::macro_def::Onset)],
) -> anyhow::Result<()> {
    let derived_exceptional = names
        .into_iter()
        .filter(|name| {
            deckmaste_construction_core::macro_def::normalize_surface_onset(name, None).is_none()
        })
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
    context_onsets: BTreeMap<String, deckmaste_construction_core::macro_def::Onset>,
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
    let declarations = deckmaste_construction_core::macro_def::read_builtin_v2(
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

    const EXPECTED_EXCEPTIONAL_ONSETS: [(&str, deckmaste_construction_core::macro_def::Onset); 8] = [
        (
            "+2 Mace",
            deckmaste_construction_core::macro_def::Onset::Consonant,
        ),
        (
            "Éomer of the Riddermark",
            deckmaste_construction_core::macro_def::Onset::Vowel,
        ),
        (
            "Éomer, King of Rohan",
            deckmaste_construction_core::macro_def::Onset::Vowel,
        ),
        (
            "Éomer, Marshal of Rohan",
            deckmaste_construction_core::macro_def::Onset::Vowel,
        ),
        (
            "Éowyn, Fearless Knight",
            deckmaste_construction_core::macro_def::Onset::Vowel,
        ),
        (
            "Éowyn, Lady of Rohan",
            deckmaste_construction_core::macro_def::Onset::Vowel,
        ),
        (
            "Éowyn, Shieldmaiden",
            deckmaste_construction_core::macro_def::Onset::Vowel,
        ),
        (
            "Óin the Brave",
            deckmaste_construction_core::macro_def::Onset::Vowel,
        ),
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
            Some(deckmaste_construction_core::macro_def::Onset::Consonant)
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
            .filter(|name| {
                deckmaste_construction_core::macro_def::normalize_surface_onset(name, None)
                    .is_none()
            })
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
            &[(
                "+2 Mace",
                deckmaste_construction_core::macro_def::Onset::Consonant,
            )],
        )
        .expect_err("an unreviewed exceptional surface must fail the production guard");
        assert!(format!("{unreviewed:#}").contains("Éomer's Cousin"));

        let stale = ensure_card_name_onset_override_inventory(
            ["+2 Mace"],
            &[
                (
                    "+2 Mace",
                    deckmaste_construction_core::macro_def::Onset::Consonant,
                ),
                (
                    "Óin the Brave",
                    deckmaste_construction_core::macro_def::Onset::Vowel,
                ),
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
    /// Maximum parser workers for this corpus run.
    #[arg(long, default_value_t = default_corpus_workers(), value_parser = parse_worker_count)]
    workers: usize,
}

fn default_corpus_workers() -> usize {
    std::thread::available_parallelism()
        .map_or(1, usize::from)
        .min(16)
}

fn parse_worker_count(value: &str) -> Result<usize, String> {
    let workers = value
        .parse::<usize>()
        .map_err(|error| format!("invalid worker count: {error}"))?;
    (workers > 0)
        .then_some(workers)
        .ok_or_else(|| "worker count must be at least 1".to_owned())
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
    #[arg(long, requires = "bless")]
    retire: Option<PathBuf>,
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

impl From<ProbeOnset> for deckmaste_construction_core::macro_def::Onset {
    fn from(onset: ProbeOnset) -> Self {
        match onset {
            ProbeOnset::Consonant => Self::Consonant,
            ProbeOnset::Vowel => Self::Vowel,
        }
    }
}

impl From<deckmaste_construction_core::macro_def::Onset> for ProbeOnset {
    fn from(onset: deckmaste_construction_core::macro_def::Onset) -> Self {
        match onset {
            deckmaste_construction_core::macro_def::Onset::Consonant => Self::Consonant,
            deckmaste_construction_core::macro_def::Onset::Vowel => Self::Vowel,
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

fn declaration_kind_name(kind: SourceDeclarationKind) -> &'static str {
    match kind {
        SourceDeclarationKind::Construction => "construction",
        SourceDeclarationKind::AbstractProduct => "abstract product",
        SourceDeclarationKind::AbstractSum => "abstract sum",
        SourceDeclarationKind::Vocab => "vocab",
        SourceDeclarationKind::Morphology => "morphology",
        SourceDeclarationKind::Lexeme => "lexeme",
        SourceDeclarationKind::Codec => "codec",
        SourceDeclarationKind::Identity => "identity",
        SourceDeclarationKind::Root => "root",
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
