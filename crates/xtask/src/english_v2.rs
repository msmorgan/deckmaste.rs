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

    const VOCAB_ORIGINS: &[&str] = &[
        "vocab TriggerWord",
        "vocab SubjectPronoun",
        "vocab ObjectPronoun",
        "vocab PossessiveDeterminerPronoun",
        "vocab PossessiveAbsolutePronoun",
        "vocab ReflexivePronoun",
        "vocab Variable",
        "vocab Color",
        "vocab Status",
        "vocab Designation",
        "vocab ChosenQuality",
        "vocab ControllerNoun",
        "vocab ScalarCharacteristic",
        "vocab Zone",
        "vocab NonCommonNoun",
        "vocab NonTargetCommonModifier",
        "vocab Supertype",
    ];

    const CONSTRUCTION_ORIGINS: &[&str] = &[
        "construction paragraph",
        "construction triggered",
        "construction imperative",
        "construction declarative",
        "construction with_where",
        "construction event",
        "construction where",
        "construction subject_nominal",
        "construction subject_pronoun",
        "construction object_nominal",
        "construction object_pronoun",
        "construction reflexive_object",
        "construction common_singular_head",
        "construction type_singular_head",
        "construction artifact_subtype_singular_head",
        "construction battle_subtype_singular_head",
        "construction creature_subtype_singular_head",
        "construction enchantment_subtype_singular_head",
        "construction land_subtype_singular_head",
        "construction planeswalker_subtype_singular_head",
        "construction spell_subtype_singular_head",
        "construction common_plural_head",
        "construction type_plural_head",
        "construction artifact_subtype_plural_head",
        "construction battle_subtype_plural_head",
        "construction creature_subtype_plural_head",
        "construction enchantment_subtype_plural_head",
        "construction land_subtype_plural_head",
        "construction planeswalker_subtype_plural_head",
        "construction spell_subtype_plural_head",
        "construction color_modifier",
        "construction status_modifier",
        "construction supertype_modifier",
        "construction common_noun_modifier",
        "construction type_modifier",
        "construction artifact_subtype_modifier",
        "construction battle_subtype_modifier",
        "construction creature_subtype_modifier",
        "construction enchantment_subtype_modifier",
        "construction land_subtype_modifier",
        "construction planeswalker_subtype_modifier",
        "construction spell_subtype_modifier",
        "construction non_color_modifier",
        "construction non_common_noun_modifier",
        "construction non_status_modifier",
        "construction non_supertype_modifier",
        "construction non_type_modifier",
        "construction non_artifact_subtype_modifier",
        "construction non_battle_subtype_modifier",
        "construction non_creature_subtype_modifier",
        "construction non_enchantment_subtype_modifier",
        "construction non_land_subtype_modifier",
        "construction non_planeswalker_subtype_modifier",
        "construction non_spell_subtype_modifier",
        "construction negative_modifier_member",
        "construction non_target_common_noun_modifier",
        "construction coordinated_modifier_member",
        "construction bare_singular_nominal",
        "construction modified_singular_nominal",
        "construction negative_modified_singular_nominal",
        "construction bare_plural_nominal",
        "construction modified_plural_nominal",
        "construction negative_modified_plural_nominal",
        "construction bare_singular_coordination_member",
        "construction modified_singular_coordination_member",
        "construction negative_modified_singular_coordination_member",
        "construction bare_plural_coordination_member",
        "construction modified_plural_coordination_member",
        "construction negative_modified_plural_coordination_member",
        "construction singular_and_nominal_coordination",
        "construction singular_or_nominal_coordination",
        "construction singular_and_or_nominal_coordination",
        "construction plural_and_nominal_coordination",
        "construction plural_or_nominal_coordination",
        "construction plural_and_or_nominal_coordination",
        "construction unmarked_singular_selector",
        "construction target_singular_selector",
        "construction target_singular_coordination_selector",
        "construction other_singular_selector",
        "construction other_target_singular_selector",
        "construction unmarked_plural_selector",
        "construction unmarked_plural_coordination_selector",
        "construction target_plural_selector",
        "construction target_plural_coordination_selector",
        "construction other_plural_selector",
        "construction other_target_plural_selector",
        "construction indefinite_reference",
        "construction named_card_reference",
        "construction ordinary_singular_reference",
        "construction ordinary_plural_reference",
        "construction definite_singular_reference",
        "construction definite_plural_reference",
        "construction any_target_reference",
        "construction another_reference",
        "construction each_reference",
        "construction all_reference",
        "construction fixed_reference",
        "construction variable_reference",
        "construction up_to_one_reference",
        "construction up_to_many_reference",
        "construction any_number_reference",
        "construction one_or_more_reference",
        "construction that_many",
        "construction counted_reference",
        "construction this_reference",
        "construction that_reference",
        "construction those_reference",
        "construction designated_singular_reference",
        "construction designated_plural_reference",
        "construction chosen_quality_reference",
        "construction possessed_singular_reference",
        "construction possessed_plural_reference",
        "construction possessive_absolute_reference",
        "construction singular_targeted_noun_phrase",
        "construction plural_targeted_noun_phrase",
        "construction full_and_noun_phrase_coordination",
        "construction full_or_noun_phrase_coordination",
        "construction full_and_or_noun_phrase_coordination",
        "construction coordinated_noun_phrase",
        "construction self_reference",
        "construction this_way",
        "construction that_much",
        "construction you_control",
        "construction opponent_controller",
        "construction opponent_controls",
        "construction you_own",
        "construction possessed_zone",
        "construction unpossessed_zone",
        "construction in_zone",
        "construction from_zone",
        "construction fixed_scalar_threshold",
        "construction variable_scalar_threshold",
        "construction characteristic_scalar",
        "construction mana_value_scalar",
        "construction scalar_or_less",
        "construction scalar_or_greater",
        "construction scalar_less_than",
        "construction scalar_greater_than",
        "construction scalar_less_than_or_equal_to",
        "construction count_or_more",
        "construction count_or_fewer",
        "construction scalar_qualification",
        "construction unqualified_controller_stage",
        "construction controller_qualified_reference",
        "construction unqualified_zone_stage",
        "construction zone_qualified_reference",
        "construction unqualified_numeric_stage",
        "construction scalar_qualified_reference",
        "construction count_comparison_reference",
        "construction qualified_noun_phrase",
        "construction possessive_self_reference",
        "construction possessive_plural_noun",
        "construction possessive",
        "construction destroy",
        "construction connive",
        "construction deal_damage",
        "construction gain_life",
        "construction number",
        "construction variable",
        "construction scalar_reference_amount",
        "construction cardinal",
    ];

    const SUBJECT_ORIGINS: &[&str] = &[
        "construction subject_nominal",
        "construction subject_pronoun",
    ];
    const OBJECT_ORIGINS: &[&str] = &[
        "construction object_nominal",
        "construction object_pronoun",
        "construction reflexive_object",
    ];
    const MODIFIER_ORIGINS: &[&str] = &[
        "construction color_modifier",
        "construction status_modifier",
        "construction supertype_modifier",
        "construction common_noun_modifier",
        "construction type_modifier",
        "construction artifact_subtype_modifier",
        "construction battle_subtype_modifier",
        "construction creature_subtype_modifier",
        "construction enchantment_subtype_modifier",
        "construction land_subtype_modifier",
        "construction planeswalker_subtype_modifier",
        "construction spell_subtype_modifier",
        "construction non_color_modifier",
        "construction non_common_noun_modifier",
        "construction non_status_modifier",
        "construction non_supertype_modifier",
        "construction non_type_modifier",
        "construction non_artifact_subtype_modifier",
        "construction non_battle_subtype_modifier",
        "construction non_creature_subtype_modifier",
        "construction non_enchantment_subtype_modifier",
        "construction non_land_subtype_modifier",
        "construction non_planeswalker_subtype_modifier",
        "construction non_spell_subtype_modifier",
    ];
    const NEGATIVE_MODIFIER_ORIGINS: &[&str] = &["construction negative_modifier_member"];
    const COORDINATED_MODIFIER_ORIGINS: &[&str] = &[
        "construction non_target_common_noun_modifier",
        "construction coordinated_modifier_member",
    ];

    const SINGULAR_NOMINAL_ORIGINS: &[&str] = &[
        "construction bare_singular_nominal",
        "construction modified_singular_nominal",
        "construction negative_modified_singular_nominal",
    ];
    const PLURAL_NOMINAL_ORIGINS: &[&str] = &[
        "construction bare_plural_nominal",
        "construction modified_plural_nominal",
        "construction negative_modified_plural_nominal",
    ];
    const SINGULAR_COORDINATION_MEMBER_ORIGINS: &[&str] = &[
        "construction bare_singular_coordination_member",
        "construction modified_singular_coordination_member",
        "construction negative_modified_singular_coordination_member",
    ];
    const PLURAL_COORDINATION_MEMBER_ORIGINS: &[&str] = &[
        "construction bare_plural_coordination_member",
        "construction modified_plural_coordination_member",
        "construction negative_modified_plural_coordination_member",
    ];
    const SINGULAR_NOMINAL_COORDINATION_ORIGINS: &[&str] = &[
        "construction singular_and_nominal_coordination",
        "construction singular_or_nominal_coordination",
        "construction singular_and_or_nominal_coordination",
    ];
    const PLURAL_NOMINAL_COORDINATION_ORIGINS: &[&str] = &[
        "construction plural_and_nominal_coordination",
        "construction plural_or_nominal_coordination",
        "construction plural_and_or_nominal_coordination",
    ];
    const TARGETED_NOUN_PHRASE_ORIGINS: &[&str] = &[
        "construction singular_targeted_noun_phrase",
        "construction plural_targeted_noun_phrase",
    ];
    const FULL_NOUN_PHRASE_COORDINATION_ORIGINS: &[&str] = &[
        "construction full_and_noun_phrase_coordination",
        "construction full_or_noun_phrase_coordination",
        "construction full_and_or_noun_phrase_coordination",
    ];
    const SINGULAR_SELECTOR_ORIGINS: &[&str] = &[
        "construction unmarked_singular_selector",
        "construction target_singular_selector",
        "construction target_singular_coordination_selector",
        "construction other_singular_selector",
        "construction other_target_singular_selector",
    ];
    const PLURAL_SELECTOR_ORIGINS: &[&str] = &[
        "construction unmarked_plural_selector",
        "construction unmarked_plural_coordination_selector",
        "construction target_plural_selector",
        "construction target_plural_coordination_selector",
        "construction other_plural_selector",
        "construction other_target_plural_selector",
    ];
    const UNQUALIFIED_REFERENCE_ORIGINS: &[&str] = &[
        "construction indefinite_reference",
        "construction named_card_reference",
        "construction ordinary_singular_reference",
        "construction ordinary_plural_reference",
        "construction definite_singular_reference",
        "construction definite_plural_reference",
        "construction any_target_reference",
        "construction another_reference",
        "construction each_reference",
        "construction all_reference",
        "construction fixed_reference",
        "construction variable_reference",
        "construction up_to_one_reference",
        "construction up_to_many_reference",
        "construction any_number_reference",
        "construction one_or_more_reference",
        "construction counted_reference",
        "construction this_reference",
        "construction that_reference",
        "construction those_reference",
        "construction designated_singular_reference",
        "construction designated_plural_reference",
        "construction chosen_quality_reference",
        "construction possessed_singular_reference",
        "construction possessed_plural_reference",
        "construction possessive_absolute_reference",
        "construction coordinated_noun_phrase",
        "construction self_reference",
        "construction count_comparison_reference",
    ];
    const NOUN_PHRASE_ORIGINS: &[&str] = &["construction qualified_noun_phrase"];

    static ALL_DECLARATION_ORIGINS: std::sync::LazyLock<Vec<&'static str>> =
        std::sync::LazyLock::new(|| {
            let mut origins = VOCAB_ORIGINS.to_vec();
            origins.extend([
                "morphology EnglishVerb",
                "morphology EnglishNoun",
                "lexeme CommonNoun",
                "lexeme VerbLexeme",
                "codec TypeNoun",
                "codec ArtifactSubtypeNoun",
                "codec BattleSubtypeNoun",
                "codec CreatureSubtypeNoun",
                "codec EnchantmentSubtypeNoun",
                "codec LandSubtypeNoun",
                "codec PlaneswalkerSubtypeNoun",
                "codec SpellSubtypeNoun",
                "identity SelfReferenceSpelling",
                "identity CardName",
                "codec CardinalNumber",
                "codec ScalarNumber",
            ]);
            origins.extend(CONSTRUCTION_ORIGINS);
            origins.extend([
                "abstract sum DocumentBlock",
                "abstract product OracleText",
                "root Ability",
                "root Sentence",
                "root Possessive",
                "root CardinalQuantity",
                "root MannerReference",
                "root CountReference",
                "root ScalarReference",
                "root OracleText",
            ]);
            origins
        });

    const SCANNER_ORIGINS: &[&str] = &[
        "vocab TriggerWord",
        "vocab SubjectPronoun",
        "vocab ObjectPronoun",
        "vocab PossessiveDeterminerPronoun",
        "vocab PossessiveAbsolutePronoun",
        "vocab ReflexivePronoun",
        "vocab Variable",
        "vocab Color",
        "vocab Status",
        "vocab Designation",
        "vocab ChosenQuality",
        "vocab ControllerNoun",
        "vocab ScalarCharacteristic",
        "vocab Zone",
        "vocab NonCommonNoun",
        "vocab NonTargetCommonModifier",
        "vocab Supertype",
        "lexeme CommonNoun",
        "lexeme VerbLexeme",
        "codec TypeNoun",
        "codec ArtifactSubtypeNoun",
        "codec BattleSubtypeNoun",
        "codec CreatureSubtypeNoun",
        "codec EnchantmentSubtypeNoun",
        "codec LandSubtypeNoun",
        "codec PlaneswalkerSubtypeNoun",
        "codec SpellSubtypeNoun",
        "identity SelfReferenceSpelling",
        "identity CardName",
        "codec CardinalNumber",
        "codec ScalarNumber",
        "construction triggered",
        "construction with_where",
        "root Ability",
        "root Sentence",
        "root Possessive",
        "root CardinalQuantity",
        "root MannerReference",
        "root CountReference",
        "root ScalarReference",
        "root OracleText",
    ];

    static VISITOR_ORIGINS: std::sync::LazyLock<Vec<&'static str>> =
        std::sync::LazyLock::new(|| {
            let mut category_origins = CONSTRUCTION_ORIGINS.to_vec();
            let first_targeted = category_origins
                .iter()
                .position(|origin| *origin == "construction singular_targeted_noun_phrase")
                .expect("targeted noun-phrase construction is present");
            let last_coordination = category_origins
                .iter()
                .position(|origin| *origin == "construction full_and_or_noun_phrase_coordination")
                .expect("full noun-phrase coordination construction is present");
            let targeted_categories = category_origins
                .drain(first_targeted..=last_coordination)
                .collect::<Vec<_>>();

            let that_many = category_origins
                .iter()
                .position(|origin| *origin == "construction that_many")
                .map(|index| category_origins.remove(index))
                .expect("count-deictic construction is present");
            let count_comparison = category_origins
                .iter()
                .position(|origin| *origin == "construction count_comparison_reference")
                .map(|index| category_origins.remove(index))
                .expect("count-comparison construction is present");
            let after_self_reference = category_origins
                .iter()
                .position(|origin| *origin == "construction self_reference")
                .expect("self-reference construction is present")
                + 1;
            let mut moved_categories = vec![count_comparison, that_many];
            moved_categories.extend(targeted_categories);
            category_origins.splice(after_self_reference..after_self_reference, moved_categories);

            let opponent_controller = category_origins
                .iter()
                .position(|origin| *origin == "construction opponent_controller")
                .map(|index| category_origins.remove(index))
                .expect("opponent-controller construction is present");
            let after_you_own = category_origins
                .iter()
                .position(|origin| *origin == "construction you_own")
                .expect("owner construction is present")
                + 1;
            category_origins.insert(after_you_own, opponent_controller);

            let mut origins = category_origins;
            origins.extend([
                "codec CardinalNumber",
                "codec ScalarNumber",
                "codec TypeNoun",
                "codec ArtifactSubtypeNoun",
                "codec BattleSubtypeNoun",
                "codec CreatureSubtypeNoun",
                "codec EnchantmentSubtypeNoun",
                "codec LandSubtypeNoun",
                "codec PlaneswalkerSubtypeNoun",
                "codec SpellSubtypeNoun",
            ]);
            origins.extend(CONSTRUCTION_ORIGINS);
            origins.extend(VOCAB_ORIGINS);
            origins.extend([
                "identity SelfReferenceSpelling",
                "lexeme CommonNoun",
                "lexeme VerbLexeme",
            ]);
            origins
        });

    static CATEGORY_ORIGINS: std::sync::LazyLock<Vec<&'static str>> =
        std::sync::LazyLock::new(|| {
            let mut origins = CONSTRUCTION_ORIGINS.to_vec();
            origins.extend(["abstract sum DocumentBlock", "abstract product OracleText"]);
            origins
        });

    static RULE_ORIGINS: std::sync::LazyLock<Vec<&'static str>> = std::sync::LazyLock::new(|| {
        let mut origins = CONSTRUCTION_ORIGINS.to_vec();
        origins.extend([
            "root Ability",
            "root Possessive",
            "root CardinalQuantity",
            "root MannerReference",
            "root CountReference",
            "root ScalarReference",
            "root OracleText",
        ]);
        origins
    });

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
            "type Subject"
            | "function render_subject"
            | "function agreement_for_subject"
            | "function walk_subject" => SUBJECT_ORIGINS,
            "type Object" | "function render_object" | "function walk_object" => OBJECT_ORIGINS,
            "type SingularHead"
            | "function render_singular_head"
            | "function agreement_for_singular_head"
            | "function number_for_singular_head"
            | "function onset_for_singular_head"
            | "function walk_singular_head" => &[
                "construction common_singular_head",
                "construction type_singular_head",
                "construction artifact_subtype_singular_head",
                "construction battle_subtype_singular_head",
                "construction creature_subtype_singular_head",
                "construction enchantment_subtype_singular_head",
                "construction land_subtype_singular_head",
                "construction planeswalker_subtype_singular_head",
                "construction spell_subtype_singular_head",
            ],
            "type PluralHead"
            | "function render_plural_head"
            | "function agreement_for_plural_head"
            | "function number_for_plural_head"
            | "function onset_for_plural_head"
            | "function possessive_ending_for_plural_head"
            | "function walk_plural_head" => &[
                "construction common_plural_head",
                "construction type_plural_head",
                "construction artifact_subtype_plural_head",
                "construction battle_subtype_plural_head",
                "construction creature_subtype_plural_head",
                "construction enchantment_subtype_plural_head",
                "construction land_subtype_plural_head",
                "construction planeswalker_subtype_plural_head",
                "construction spell_subtype_plural_head",
            ],
            "type NominalModifier"
            | "function render_nominal_modifier"
            | "function number_for_nominal_modifier"
            | "function onset_for_nominal_modifier"
            | "function walk_nominal_modifier" => MODIFIER_ORIGINS,
            "type NegativeNominalModifier"
            | "function render_negative_nominal_modifier"
            | "function walk_negative_nominal_modifier" => NEGATIVE_MODIFIER_ORIGINS,
            "type CoordinatedNominalModifier"
            | "function render_coordinated_nominal_modifier"
            | "function onset_for_coordinated_nominal_modifier"
            | "function walk_coordinated_nominal_modifier" => COORDINATED_MODIFIER_ORIGINS,
            "type SingularNominal"
            | "function render_singular_nominal"
            | "function agreement_for_singular_nominal"
            | "function number_for_singular_nominal"
            | "function onset_for_singular_nominal"
            | "function walk_singular_nominal" => SINGULAR_NOMINAL_ORIGINS,
            "type PluralNominal"
            | "function render_plural_nominal"
            | "function agreement_for_plural_nominal"
            | "function number_for_plural_nominal"
            | "function onset_for_plural_nominal"
            | "function walk_plural_nominal" => PLURAL_NOMINAL_ORIGINS,
            "type SingularCoordinationMember"
            | "function render_singular_coordination_member"
            | "function walk_singular_coordination_member" => SINGULAR_COORDINATION_MEMBER_ORIGINS,
            "type PluralCoordinationMember"
            | "function render_plural_coordination_member"
            | "function walk_plural_coordination_member" => PLURAL_COORDINATION_MEMBER_ORIGINS,
            "type SingularNominalCoordination"
            | "function render_singular_nominal_coordination"
            | "function walk_singular_nominal_coordination" => {
                SINGULAR_NOMINAL_COORDINATION_ORIGINS
            }
            "type PluralNominalCoordination"
            | "function render_plural_nominal_coordination"
            | "function walk_plural_nominal_coordination" => PLURAL_NOMINAL_COORDINATION_ORIGINS,
            "type SingularSelector"
            | "function render_singular_selector"
            | "function agreement_for_singular_selector"
            | "function number_for_singular_selector"
            | "function onset_for_singular_selector"
            | "function walk_singular_selector" => SINGULAR_SELECTOR_ORIGINS,
            "type PluralSelector"
            | "function render_plural_selector"
            | "function agreement_for_plural_selector"
            | "function number_for_plural_selector"
            | "function onset_for_plural_selector"
            | "function walk_plural_selector" => PLURAL_SELECTOR_ORIGINS,
            "type UnqualifiedReference"
            | "function render_unqualified_reference"
            | "function agreement_for_unqualified_reference"
            | "function number_for_unqualified_reference"
            | "function onset_for_unqualified_reference"
            | "function walk_unqualified_reference" => UNQUALIFIED_REFERENCE_ORIGINS,
            "type CountReference"
            | "function render_count_reference"
            | "function render_count_reference_body"
            | "function agreement_for_count_reference"
            | "function number_for_count_reference"
            | "function onset_for_count_reference"
            | "function walk_count_reference"
            | "type ThatMany"
            | "function walk_that_many" => &["construction that_many"],
            "type NounPhrase"
            | "function render_noun_phrase"
            | "function agreement_for_noun_phrase"
            | "function number_for_noun_phrase"
            | "function onset_for_noun_phrase"
            | "function walk_noun_phrase" => NOUN_PHRASE_ORIGINS,
            "type TargetedNounPhrase"
            | "function render_targeted_noun_phrase"
            | "function walk_targeted_noun_phrase" => TARGETED_NOUN_PHRASE_ORIGINS,
            "type FullNounPhraseCoordination"
            | "function render_full_noun_phrase_coordination"
            | "function walk_full_noun_phrase_coordination" => {
                FULL_NOUN_PHRASE_COORDINATION_ORIGINS
            }
            "type MannerReference"
            | "function render_manner_reference"
            | "function walk_manner_reference"
            | "type ThisWay"
            | "function walk_this_way" => &["construction this_way"],
            "type ScalarReference"
            | "function render_scalar_reference"
            | "function render_scalar_reference_body"
            | "function agreement_for_scalar_reference"
            | "function number_for_scalar_reference"
            | "function onset_for_scalar_reference"
            | "function walk_scalar_reference"
            | "type ThatMuch"
            | "function walk_that_much" => &["construction that_much"],
            "type ControllerOwnerQualification"
            | "function render_controller_owner_qualification"
            | "function walk_controller_owner_qualification" => &[
                "construction you_control",
                "construction opponent_controls",
                "construction you_own",
            ],
            "type SingularController"
            | "function render_singular_controller"
            | "function walk_singular_controller"
            | "type OpponentController"
            | "impl OpponentController"
            | "function walk_opponent_controller" => &["construction opponent_controller"],
            "type ZoneReference"
            | "function render_zone_reference"
            | "function walk_zone_reference" => &[
                "construction possessed_zone",
                "construction unpossessed_zone",
            ],
            "type ZoneQualification"
            | "function render_zone_qualification"
            | "function walk_zone_qualification" => {
                &["construction in_zone", "construction from_zone"]
            }
            "type ScalarThreshold"
            | "function render_scalar_threshold"
            | "function walk_scalar_threshold" => &[
                "construction fixed_scalar_threshold",
                "construction variable_scalar_threshold",
            ],
            "type ScalarMeasure"
            | "function render_scalar_measure"
            | "function walk_scalar_measure" => &[
                "construction characteristic_scalar",
                "construction mana_value_scalar",
            ],
            "type ScalarComparison"
            | "function render_scalar_comparison"
            | "function walk_scalar_comparison" => &[
                "construction scalar_or_less",
                "construction scalar_or_greater",
                "construction scalar_less_than",
                "construction scalar_greater_than",
                "construction scalar_less_than_or_equal_to",
            ],
            "type CountComparison"
            | "function render_count_comparison"
            | "function walk_count_comparison" => {
                &["construction count_or_more", "construction count_or_fewer"]
            }
            "type ScalarQualification"
            | "function render_scalar_qualification"
            | "function walk_scalar_qualification" => &["construction scalar_qualification"],
            "type ControllerStage"
            | "function render_controller_stage"
            | "function agreement_for_controller_stage"
            | "function number_for_controller_stage"
            | "function onset_for_controller_stage"
            | "function walk_controller_stage" => &[
                "construction unqualified_controller_stage",
                "construction controller_qualified_reference",
            ],
            "type ZoneStage"
            | "function render_zone_stage"
            | "function agreement_for_zone_stage"
            | "function number_for_zone_stage"
            | "function onset_for_zone_stage"
            | "function walk_zone_stage" => &[
                "construction unqualified_zone_stage",
                "construction zone_qualified_reference",
            ],
            "type NumericStage"
            | "function render_numeric_stage"
            | "function agreement_for_numeric_stage"
            | "function number_for_numeric_stage"
            | "function onset_for_numeric_stage"
            | "function walk_numeric_stage" => &[
                "construction unqualified_numeric_stage",
                "construction scalar_qualified_reference",
            ],
            "type PossessiveOwner"
            | "function render_possessive_owner"
            | "function number_for_possessive_owner"
            | "function possessive_ending_for_possessive_owner"
            | "function walk_possessive_owner" => &[
                "construction possessive_self_reference",
                "construction possessive_plural_noun",
            ],
            "type Possessive" | "function walk_possessive" => &["construction possessive"],
            "type VerbPhrase" | "function render_verb_phrase" | "function walk_verb_phrase" => &[
                "construction destroy",
                "construction connive",
                "construction deal_damage",
                "construction gain_life",
            ],
            "type Amount" | "function render_amount" | "function walk_amount" => &[
                "construction number",
                "construction variable",
                "construction scalar_reference_amount",
            ],
            "type CardinalQuantity"
            | "function render_cardinal_quantity_body"
            | "function cardinality_for_cardinal_quantity"
            | "function walk_cardinal_quantity"
            | "type CardinalQuantityValue"
            | "function walk_cardinal_quantity_value" => &["construction cardinal"],
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
            "function render_negative_modified_singular_nominal_modifiers_sequence"
            | "function walk_negative_modified_singular_nominal_modifiers_sequence" => {
                &["construction NegativeModifiedSingularNominal"]
            }
            "function render_negative_modified_plural_nominal_modifiers_sequence"
            | "function walk_negative_modified_plural_nominal_modifiers_sequence" => {
                &["construction NegativeModifiedPluralNominal"]
            }
            "function render_negative_modified_singular_coordination_member_modifiers_sequence"
            | "function walk_negative_modified_singular_coordination_member_modifiers_sequence" => {
                &["construction NegativeModifiedSingularCoordinationMember"]
            }
            "function render_negative_modified_plural_coordination_member_modifiers_sequence"
            | "function walk_negative_modified_plural_coordination_member_modifiers_sequence" => {
                &["construction NegativeModifiedPluralCoordinationMember"]
            }
            "function render_singular_and_nominal_coordination_members_sequence"
            | "function walk_singular_and_nominal_coordination_members_sequence" => {
                &["construction SingularAndNominalCoordination"]
            }
            "function render_singular_or_nominal_coordination_members_sequence"
            | "function walk_singular_or_nominal_coordination_members_sequence" => {
                &["construction SingularOrNominalCoordination"]
            }
            "function render_singular_and_or_nominal_coordination_members_sequence"
            | "function walk_singular_and_or_nominal_coordination_members_sequence" => {
                &["construction SingularAndOrNominalCoordination"]
            }
            "function render_plural_and_nominal_coordination_members_sequence"
            | "function walk_plural_and_nominal_coordination_members_sequence" => {
                &["construction PluralAndNominalCoordination"]
            }
            "function render_plural_or_nominal_coordination_members_sequence"
            | "function walk_plural_or_nominal_coordination_members_sequence" => {
                &["construction PluralOrNominalCoordination"]
            }
            "function render_plural_and_or_nominal_coordination_members_sequence"
            | "function walk_plural_and_or_nominal_coordination_members_sequence" => {
                &["construction PluralAndOrNominalCoordination"]
            }
            "function render_full_and_noun_phrase_coordination_members_sequence"
            | "function walk_full_and_noun_phrase_coordination_members_sequence" => {
                &["construction FullAndNounPhraseCoordination"]
            }
            "function render_full_or_noun_phrase_coordination_members_sequence"
            | "function walk_full_or_noun_phrase_coordination_members_sequence" => {
                &["construction FullOrNounPhraseCoordination"]
            }
            "function render_full_and_or_noun_phrase_coordination_members_sequence"
            | "function walk_full_and_or_noun_phrase_coordination_members_sequence" => {
                &["construction FullAndOrNounPhraseCoordination"]
            }
            "type Imperative" | "function walk_imperative" => &["construction imperative"],
            "type Declarative" | "function walk_declarative" => &["construction declarative"],
            "type WithWhere" | "impl WithWhere" | "function walk_with_where" => {
                &["construction with_where"]
            }
            "type EventClause" | "function walk_event_clause" => &["construction event"],
            "type WhereClause" | "function walk_where_clause" => &["construction where"],
            "type NominalSubject" | "function walk_nominal_subject" => {
                &["construction subject_nominal"]
            }
            "type PersonalSubject" | "function walk_personal_subject" => {
                &["construction subject_pronoun"]
            }
            "type NominalObject" | "function walk_nominal_object" => {
                &["construction object_nominal"]
            }
            "type PersonalObject" | "function walk_personal_object" => {
                &["construction object_pronoun"]
            }
            "type ReflexiveObject" | "function walk_reflexive_object" => {
                &["construction reflexive_object"]
            }
            "type CommonSingularHead" | "function walk_common_singular_head" => {
                &["construction common_singular_head"]
            }
            "type TypeSingularHead" | "function walk_type_singular_head" => {
                &["construction type_singular_head"]
            }
            "type ArtifactSubtypeSingularHead" | "function walk_artifact_subtype_singular_head" => {
                &["construction artifact_subtype_singular_head"]
            }
            "type BattleSubtypeSingularHead" | "function walk_battle_subtype_singular_head" => {
                &["construction battle_subtype_singular_head"]
            }
            "type CreatureSubtypeSingularHead" | "function walk_creature_subtype_singular_head" => {
                &["construction creature_subtype_singular_head"]
            }
            "type EnchantmentSubtypeSingularHead"
            | "function walk_enchantment_subtype_singular_head" => {
                &["construction enchantment_subtype_singular_head"]
            }
            "type LandSubtypeSingularHead" | "function walk_land_subtype_singular_head" => {
                &["construction land_subtype_singular_head"]
            }
            "type PlaneswalkerSubtypeSingularHead"
            | "function walk_planeswalker_subtype_singular_head" => {
                &["construction planeswalker_subtype_singular_head"]
            }
            "type SpellSubtypeSingularHead" | "function walk_spell_subtype_singular_head" => {
                &["construction spell_subtype_singular_head"]
            }
            "type CommonPluralHead" | "function walk_common_plural_head" => {
                &["construction common_plural_head"]
            }
            "type TypePluralHead" | "function walk_type_plural_head" => {
                &["construction type_plural_head"]
            }
            "type ArtifactSubtypePluralHead" | "function walk_artifact_subtype_plural_head" => {
                &["construction artifact_subtype_plural_head"]
            }
            "type BattleSubtypePluralHead" | "function walk_battle_subtype_plural_head" => {
                &["construction battle_subtype_plural_head"]
            }
            "type CreatureSubtypePluralHead" | "function walk_creature_subtype_plural_head" => {
                &["construction creature_subtype_plural_head"]
            }
            "type EnchantmentSubtypePluralHead"
            | "function walk_enchantment_subtype_plural_head" => {
                &["construction enchantment_subtype_plural_head"]
            }
            "type LandSubtypePluralHead" | "function walk_land_subtype_plural_head" => {
                &["construction land_subtype_plural_head"]
            }
            "type PlaneswalkerSubtypePluralHead"
            | "function walk_planeswalker_subtype_plural_head" => {
                &["construction planeswalker_subtype_plural_head"]
            }
            "type SpellSubtypePluralHead" | "function walk_spell_subtype_plural_head" => {
                &["construction spell_subtype_plural_head"]
            }
            "type ColorModifier" | "function walk_color_modifier" => {
                &["construction color_modifier"]
            }
            "type StatusModifier" | "function walk_status_modifier" => {
                &["construction status_modifier"]
            }
            "type SupertypeModifier" | "function walk_supertype_modifier" => {
                &["construction supertype_modifier"]
            }
            "type CommonNounModifier" | "function walk_common_noun_modifier" => {
                &["construction common_noun_modifier"]
            }
            "type TypeModifier" | "function walk_type_modifier" => &["construction type_modifier"],
            "type ArtifactSubtypeModifier" | "function walk_artifact_subtype_modifier" => {
                &["construction artifact_subtype_modifier"]
            }
            "type BattleSubtypeModifier" | "function walk_battle_subtype_modifier" => {
                &["construction battle_subtype_modifier"]
            }
            "type CreatureSubtypeModifier" | "function walk_creature_subtype_modifier" => {
                &["construction creature_subtype_modifier"]
            }
            "type EnchantmentSubtypeModifier" | "function walk_enchantment_subtype_modifier" => {
                &["construction enchantment_subtype_modifier"]
            }
            "type LandSubtypeModifier" | "function walk_land_subtype_modifier" => {
                &["construction land_subtype_modifier"]
            }
            "type PlaneswalkerSubtypeModifier" | "function walk_planeswalker_subtype_modifier" => {
                &["construction planeswalker_subtype_modifier"]
            }
            "type SpellSubtypeModifier" | "function walk_spell_subtype_modifier" => {
                &["construction spell_subtype_modifier"]
            }
            "type NonColorModifier" | "function walk_non_color_modifier" => {
                &["construction non_color_modifier"]
            }
            "type NonCommonNounModifier" | "function walk_non_common_noun_modifier" => {
                &["construction non_common_noun_modifier"]
            }
            "type NonStatusModifier" | "function walk_non_status_modifier" => {
                &["construction non_status_modifier"]
            }
            "type NonSupertypeModifier" | "function walk_non_supertype_modifier" => {
                &["construction non_supertype_modifier"]
            }
            "type NonTypeModifier" | "function walk_non_type_modifier" => {
                &["construction non_type_modifier"]
            }
            "type NonArtifactSubtypeModifier" | "function walk_non_artifact_subtype_modifier" => {
                &["construction non_artifact_subtype_modifier"]
            }
            "type NonBattleSubtypeModifier" | "function walk_non_battle_subtype_modifier" => {
                &["construction non_battle_subtype_modifier"]
            }
            "type NonCreatureSubtypeModifier" | "function walk_non_creature_subtype_modifier" => {
                &["construction non_creature_subtype_modifier"]
            }
            "type NonEnchantmentSubtypeModifier"
            | "function walk_non_enchantment_subtype_modifier" => {
                &["construction non_enchantment_subtype_modifier"]
            }
            "type NonLandSubtypeModifier" | "function walk_non_land_subtype_modifier" => {
                &["construction non_land_subtype_modifier"]
            }
            "type NonPlaneswalkerSubtypeModifier"
            | "function walk_non_planeswalker_subtype_modifier" => {
                &["construction non_planeswalker_subtype_modifier"]
            }
            "type NonSpellSubtypeModifier" | "function walk_non_spell_subtype_modifier" => {
                &["construction non_spell_subtype_modifier"]
            }
            "type NegativeModifierMember"
            | "impl NegativeModifierMember"
            | "function walk_negative_modifier_member" => {
                &["construction negative_modifier_member"]
            }
            "type NonTargetCommonNounModifier"
            | "function walk_non_target_common_noun_modifier" => {
                &["construction non_target_common_noun_modifier"]
            }
            "type CoordinatedModifierMember"
            | "impl CoordinatedModifierMember"
            | "function walk_coordinated_modifier_member" => {
                &["construction coordinated_modifier_member"]
            }
            "type BareSingularNominal" | "function walk_bare_singular_nominal" => {
                &["construction bare_singular_nominal"]
            }
            "type ModifiedSingularNominal" | "function walk_modified_singular_nominal" => {
                &["construction modified_singular_nominal"]
            }
            "type NegativeModifiedSingularNominal"
            | "impl NegativeModifiedSingularNominal"
            | "function walk_negative_modified_singular_nominal" => {
                &["construction negative_modified_singular_nominal"]
            }
            "type BarePluralNominal" | "function walk_bare_plural_nominal" => {
                &["construction bare_plural_nominal"]
            }
            "type ModifiedPluralNominal" | "function walk_modified_plural_nominal" => {
                &["construction modified_plural_nominal"]
            }
            "type NegativeModifiedPluralNominal"
            | "impl NegativeModifiedPluralNominal"
            | "function walk_negative_modified_plural_nominal" => {
                &["construction negative_modified_plural_nominal"]
            }
            "type BareSingularCoordinationMember"
            | "function walk_bare_singular_coordination_member" => {
                &["construction bare_singular_coordination_member"]
            }
            "type ModifiedSingularCoordinationMember"
            | "function walk_modified_singular_coordination_member" => {
                &["construction modified_singular_coordination_member"]
            }
            "type NegativeModifiedSingularCoordinationMember"
            | "impl NegativeModifiedSingularCoordinationMember"
            | "function walk_negative_modified_singular_coordination_member" => {
                &["construction negative_modified_singular_coordination_member"]
            }
            "type BarePluralCoordinationMember"
            | "function walk_bare_plural_coordination_member" => {
                &["construction bare_plural_coordination_member"]
            }
            "type ModifiedPluralCoordinationMember"
            | "function walk_modified_plural_coordination_member" => {
                &["construction modified_plural_coordination_member"]
            }
            "type NegativeModifiedPluralCoordinationMember"
            | "impl NegativeModifiedPluralCoordinationMember"
            | "function walk_negative_modified_plural_coordination_member" => {
                &["construction negative_modified_plural_coordination_member"]
            }
            "type SingularAndNominalCoordination"
            | "impl SingularAndNominalCoordination"
            | "function walk_singular_and_nominal_coordination" => {
                &["construction singular_and_nominal_coordination"]
            }
            "type SingularOrNominalCoordination"
            | "impl SingularOrNominalCoordination"
            | "function walk_singular_or_nominal_coordination" => {
                &["construction singular_or_nominal_coordination"]
            }
            "type SingularAndOrNominalCoordination"
            | "impl SingularAndOrNominalCoordination"
            | "function walk_singular_and_or_nominal_coordination" => {
                &["construction singular_and_or_nominal_coordination"]
            }
            "type PluralAndNominalCoordination"
            | "impl PluralAndNominalCoordination"
            | "function walk_plural_and_nominal_coordination" => {
                &["construction plural_and_nominal_coordination"]
            }
            "type PluralOrNominalCoordination"
            | "impl PluralOrNominalCoordination"
            | "function walk_plural_or_nominal_coordination" => {
                &["construction plural_or_nominal_coordination"]
            }
            "type PluralAndOrNominalCoordination"
            | "impl PluralAndOrNominalCoordination"
            | "function walk_plural_and_or_nominal_coordination" => {
                &["construction plural_and_or_nominal_coordination"]
            }
            "type UnmarkedSingularSelector" | "function walk_unmarked_singular_selector" => {
                &["construction unmarked_singular_selector"]
            }
            "type TargetSingularSelector" | "function walk_target_singular_selector" => {
                &["construction target_singular_selector"]
            }
            "type TargetSingularCoordinationSelector"
            | "function walk_target_singular_coordination_selector" => {
                &["construction target_singular_coordination_selector"]
            }
            "type OtherSingularSelector" | "function walk_other_singular_selector" => {
                &["construction other_singular_selector"]
            }
            "type OtherTargetSingularSelector" | "function walk_other_target_singular_selector" => {
                &["construction other_target_singular_selector"]
            }
            "type UnmarkedPluralSelector" | "function walk_unmarked_plural_selector" => {
                &["construction unmarked_plural_selector"]
            }
            "type UnmarkedPluralCoordinationSelector"
            | "function walk_unmarked_plural_coordination_selector" => {
                &["construction unmarked_plural_coordination_selector"]
            }
            "type TargetPluralSelector" | "function walk_target_plural_selector" => {
                &["construction target_plural_selector"]
            }
            "type TargetPluralCoordinationSelector"
            | "function walk_target_plural_coordination_selector" => {
                &["construction target_plural_coordination_selector"]
            }
            "type OtherPluralSelector" | "function walk_other_plural_selector" => {
                &["construction other_plural_selector"]
            }
            "type OtherTargetPluralSelector" | "function walk_other_target_plural_selector" => {
                &["construction other_target_plural_selector"]
            }
            "type IndefiniteReference" | "function walk_indefinite_reference" => {
                &["construction indefinite_reference"]
            }
            "type NamedCardReference" | "function walk_named_card_reference" => {
                &["construction named_card_reference"]
            }
            "type OrdinarySingularReference"
            | "impl OrdinarySingularReference"
            | "function walk_ordinary_singular_reference" => {
                &["construction ordinary_singular_reference"]
            }
            "type OrdinaryPluralReference" | "function walk_ordinary_plural_reference" => {
                &["construction ordinary_plural_reference"]
            }
            "type DefiniteSingularReference" | "function walk_definite_singular_reference" => {
                &["construction definite_singular_reference"]
            }
            "type DefinitePluralReference" | "function walk_definite_plural_reference" => {
                &["construction definite_plural_reference"]
            }
            "type AnyTargetReference" | "function walk_any_target_reference" => {
                &["construction any_target_reference"]
            }
            "type AnotherReference"
            | "impl AnotherReference"
            | "function walk_another_reference" => &["construction another_reference"],
            "type EachReference" | "function walk_each_reference" => {
                &["construction each_reference"]
            }
            "type AllReference" | "function walk_all_reference" => &["construction all_reference"],
            "type FixedReference" | "impl FixedReference" | "function walk_fixed_reference" => {
                &["construction fixed_reference"]
            }
            "type VariableReference" | "function walk_variable_reference" => {
                &["construction variable_reference"]
            }
            "type UpToOneReference"
            | "impl UpToOneReference"
            | "function walk_up_to_one_reference" => &["construction up_to_one_reference"],
            "type UpToManyReference"
            | "impl UpToManyReference"
            | "function walk_up_to_many_reference" => &["construction up_to_many_reference"],
            "type AnyNumberReference" | "function walk_any_number_reference" => {
                &["construction any_number_reference"]
            }
            "type OneOrMoreReference" | "function walk_one_or_more_reference" => {
                &["construction one_or_more_reference"]
            }
            "type CountedReference" | "function walk_counted_reference" => {
                &["construction counted_reference"]
            }
            "type ThisReference" | "function walk_this_reference" => {
                &["construction this_reference"]
            }
            "type ThatReference" | "function walk_that_reference" => {
                &["construction that_reference"]
            }
            "type ThoseReference" | "function walk_those_reference" => {
                &["construction those_reference"]
            }
            "type DesignatedSingularReference" | "function walk_designated_singular_reference" => {
                &["construction designated_singular_reference"]
            }
            "type DesignatedPluralReference" | "function walk_designated_plural_reference" => {
                &["construction designated_plural_reference"]
            }
            "type ChosenQualityReference" | "function walk_chosen_quality_reference" => {
                &["construction chosen_quality_reference"]
            }
            "type PossessedSingularReference" | "function walk_possessed_singular_reference" => {
                &["construction possessed_singular_reference"]
            }
            "type PossessedPluralReference" | "function walk_possessed_plural_reference" => {
                &["construction possessed_plural_reference"]
            }
            "type PossessiveAbsoluteReference"
            | "function walk_possessive_absolute_reference"
            | "function agreement_for_possessive_absolute_pronoun"
            | "function number_for_possessive_absolute_pronoun" => {
                &["construction possessive_absolute_reference"]
            }
            "type SingularTargetedNounPhrase" | "function walk_singular_targeted_noun_phrase" => {
                &["construction singular_targeted_noun_phrase"]
            }
            "type PluralTargetedNounPhrase" | "function walk_plural_targeted_noun_phrase" => {
                &["construction plural_targeted_noun_phrase"]
            }
            "type FullAndNounPhraseCoordination"
            | "impl FullAndNounPhraseCoordination"
            | "function walk_full_and_noun_phrase_coordination" => {
                &["construction full_and_noun_phrase_coordination"]
            }
            "type FullOrNounPhraseCoordination"
            | "impl FullOrNounPhraseCoordination"
            | "function walk_full_or_noun_phrase_coordination" => {
                &["construction full_or_noun_phrase_coordination"]
            }
            "type FullAndOrNounPhraseCoordination"
            | "impl FullAndOrNounPhraseCoordination"
            | "function walk_full_and_or_noun_phrase_coordination" => {
                &["construction full_and_or_noun_phrase_coordination"]
            }
            "type CoordinatedNounPhrase" | "function walk_coordinated_noun_phrase" => {
                &["construction coordinated_noun_phrase"]
            }
            "type SourceSelfReference"
            | "impl SourceSelfReference"
            | "function walk_source_self_reference" => &["construction self_reference"],
            "type YouControl" | "impl YouControl" | "function walk_you_control" => {
                &["construction you_control"]
            }
            "type OpponentControls" | "function walk_opponent_controls" => {
                &["construction opponent_controls"]
            }
            "type YouOwn" | "impl YouOwn" | "function walk_you_own" => &["construction you_own"],
            "type PossessedZone" | "function walk_possessed_zone" => {
                &["construction possessed_zone"]
            }
            "type UnpossessedZone" | "impl UnpossessedZone" | "function walk_unpossessed_zone" => {
                &["construction unpossessed_zone"]
            }
            "type InZone" | "function walk_in_zone" => &["construction in_zone"],
            "type FromZone" | "function walk_from_zone" => &["construction from_zone"],
            "type FixedScalarThreshold" | "function walk_fixed_scalar_threshold" => {
                &["construction fixed_scalar_threshold"]
            }
            "type VariableScalarThreshold" | "function walk_variable_scalar_threshold" => {
                &["construction variable_scalar_threshold"]
            }
            "type CharacteristicScalar" | "function walk_characteristic_scalar" => {
                &["construction characteristic_scalar"]
            }
            "type ManaValueScalar" | "function walk_mana_value_scalar" => {
                &["construction mana_value_scalar"]
            }
            "type ScalarOrLess" | "function walk_scalar_or_less" => {
                &["construction scalar_or_less"]
            }
            "type ScalarOrGreater" | "function walk_scalar_or_greater" => {
                &["construction scalar_or_greater"]
            }
            "type ScalarLessThan" | "function walk_scalar_less_than" => {
                &["construction scalar_less_than"]
            }
            "type ScalarGreaterThan" | "function walk_scalar_greater_than" => {
                &["construction scalar_greater_than"]
            }
            "type ScalarLessThanOrEqualTo" | "function walk_scalar_less_than_or_equal_to" => {
                &["construction scalar_less_than_or_equal_to"]
            }
            "type CountOrMore" | "function walk_count_or_more" => &["construction count_or_more"],
            "type CountOrFewer" | "function walk_count_or_fewer" => {
                &["construction count_or_fewer"]
            }
            "type ScalarQualificationValue" | "function walk_scalar_qualification_value" => {
                &["construction scalar_qualification"]
            }
            "type UnqualifiedControllerStage" | "function walk_unqualified_controller_stage" => {
                &["construction unqualified_controller_stage"]
            }
            "type ControllerQualifiedReference"
            | "function walk_controller_qualified_reference" => {
                &["construction controller_qualified_reference"]
            }
            "type UnqualifiedZoneStage" | "function walk_unqualified_zone_stage" => {
                &["construction unqualified_zone_stage"]
            }
            "type ZoneQualifiedReference" | "function walk_zone_qualified_reference" => {
                &["construction zone_qualified_reference"]
            }
            "type UnqualifiedNumericStage" | "function walk_unqualified_numeric_stage" => {
                &["construction unqualified_numeric_stage"]
            }
            "type ScalarQualifiedReference" | "function walk_scalar_qualified_reference" => {
                &["construction scalar_qualified_reference"]
            }
            "type CountComparisonReference"
            | "impl CountComparisonReference"
            | "function walk_count_comparison_reference" => {
                &["construction count_comparison_reference"]
            }
            "type QualifiedNounPhrase" | "function walk_qualified_noun_phrase" => {
                &["construction qualified_noun_phrase"]
            }
            "type PossessiveSelfReference"
            | "impl PossessiveSelfReference"
            | "function walk_possessive_self_reference" => {
                &["construction possessive_self_reference"]
            }
            "type PossessiveNoun" | "function walk_possessive_noun" => {
                &["construction possessive_plural_noun"]
            }
            "type PossessiveValue" | "function walk_possessive_value" => {
                &["construction possessive"]
            }
            "type Destroy" | "function walk_destroy" => &["construction destroy"],
            "type Connive" | "function walk_connive" => &["construction connive"],
            "type DealDamage" | "function walk_deal_damage" => &["construction deal_damage"],
            "type GainLife" | "function walk_gain_life" => &["construction gain_life"],
            "type NumberAmount" | "function walk_number_amount" => &["construction number"],
            "type VariableAmount" | "function walk_variable_amount" => &["construction variable"],
            "type ScalarReferenceAmount" | "function walk_scalar_reference_amount" => {
                &["construction scalar_reference_amount"]
            }
            "type TriggerWord" | "function render_trigger_word" | "function walk_trigger_word" => {
                &["vocab TriggerWord"]
            }
            "type SubjectPronoun"
            | "function render_subject_pronoun"
            | "function walk_subject_pronoun" => &["vocab SubjectPronoun"],
            "type ObjectPronoun"
            | "function render_object_pronoun"
            | "function walk_object_pronoun" => &["vocab ObjectPronoun"],
            "type PossessiveDeterminerPronoun"
            | "function render_possessive_determiner_pronoun"
            | "function walk_possessive_determiner_pronoun" => {
                &["vocab PossessiveDeterminerPronoun"]
            }
            "type PossessiveAbsolutePronoun"
            | "function render_possessive_absolute_pronoun"
            | "function walk_possessive_absolute_pronoun" => &["vocab PossessiveAbsolutePronoun"],
            "type ReflexivePronoun"
            | "function render_reflexive_pronoun"
            | "function walk_reflexive_pronoun" => &["vocab ReflexivePronoun"],
            "function agreement_for_subject_pronoun" => &["construction subject_pronoun"],
            "function agreement_for_object_pronoun" => &["construction object_pronoun"],
            "function agreement_for_reflexive_pronoun"
            | "function number_for_reflexive_pronoun" => &["construction reflexive_object"],
            "type Variable" | "function render_variable" | "function walk_variable" => {
                &["vocab Variable"]
            }
            "type Color" | "function render_color" | "function walk_color" => &["vocab Color"],
            "type Status" | "function render_status" | "function walk_status" => &["vocab Status"],
            "type Designation" | "function render_designation" | "function walk_designation" => {
                &["vocab Designation"]
            }
            "type ChosenQuality"
            | "function render_chosen_quality"
            | "function walk_chosen_quality" => &["vocab ChosenQuality"],
            "type ControllerNoun"
            | "function render_controller_noun"
            | "function walk_controller_noun" => &["vocab ControllerNoun"],
            "type ScalarCharacteristic"
            | "function render_scalar_characteristic"
            | "function walk_scalar_characteristic" => &["vocab ScalarCharacteristic"],
            "type Zone" | "function render_zone" | "function walk_zone" => &["vocab Zone"],
            "type NonCommonNoun"
            | "function render_non_common_noun"
            | "function walk_non_common_noun" => &["vocab NonCommonNoun"],
            "type NonTargetCommonModifier"
            | "function render_non_target_common_modifier"
            | "function walk_non_target_common_modifier" => &["vocab NonTargetCommonModifier"],
            "type Supertype" | "function render_supertype" | "function walk_supertype" => {
                &["vocab Supertype"]
            }
            "type CommonNoun"
            | "function surface_for_common_noun"
            | "function walk_common_noun" => &["lexeme CommonNoun"],
            "type VerbLexeme"
            | "function surface_for_verb_lexeme"
            | "function walk_verb_lexeme" => &["lexeme VerbLexeme"],
            "type DeclarationTypeNoun"
            | "impl DeclarationTypeNoun"
            | "type TypeNoun"
            | "function walk_declaration_type_noun"
            | "function walk_type_noun" => &["codec TypeNoun"],
            "type DeclarationArtifactSubtypeNoun"
            | "impl DeclarationArtifactSubtypeNoun"
            | "type ArtifactSubtypeNoun"
            | "function walk_declaration_artifact_subtype_noun"
            | "function walk_artifact_subtype_noun" => &["codec ArtifactSubtypeNoun"],
            "type DeclarationBattleSubtypeNoun"
            | "impl DeclarationBattleSubtypeNoun"
            | "type BattleSubtypeNoun"
            | "function walk_declaration_battle_subtype_noun"
            | "function walk_battle_subtype_noun" => &["codec BattleSubtypeNoun"],
            "type DeclarationCreatureSubtypeNoun"
            | "impl DeclarationCreatureSubtypeNoun"
            | "type CreatureSubtypeNoun"
            | "function walk_declaration_creature_subtype_noun"
            | "function walk_creature_subtype_noun" => &["codec CreatureSubtypeNoun"],
            "type DeclarationEnchantmentSubtypeNoun"
            | "impl DeclarationEnchantmentSubtypeNoun"
            | "type EnchantmentSubtypeNoun"
            | "function walk_declaration_enchantment_subtype_noun"
            | "function walk_enchantment_subtype_noun" => &["codec EnchantmentSubtypeNoun"],
            "type DeclarationLandSubtypeNoun"
            | "impl DeclarationLandSubtypeNoun"
            | "type LandSubtypeNoun"
            | "function walk_declaration_land_subtype_noun"
            | "function walk_land_subtype_noun" => &["codec LandSubtypeNoun"],
            "type DeclarationPlaneswalkerSubtypeNoun"
            | "impl DeclarationPlaneswalkerSubtypeNoun"
            | "type PlaneswalkerSubtypeNoun"
            | "function walk_declaration_planeswalker_subtype_noun"
            | "function walk_planeswalker_subtype_noun" => &["codec PlaneswalkerSubtypeNoun"],
            "type DeclarationSpellSubtypeNoun"
            | "impl DeclarationSpellSubtypeNoun"
            | "type SpellSubtypeNoun"
            | "function walk_declaration_spell_subtype_noun"
            | "function walk_spell_subtype_noun" => &["codec SpellSubtypeNoun"],
            "type CardinalNumber"
            | "function format_cardinal_number"
            | "function number_for_cardinal_number"
            | "function cardinality_for_cardinal_number"
            | "function parse_cardinal_number"
            | "function render_cardinal_number"
            | "function walk_cardinal_number" => &["codec CardinalNumber"],
            "type ScalarNumber"
            | "function format_scalar_number"
            | "function parse_scalar_number"
            | "function render_scalar_number"
            | "function walk_scalar_number" => &["codec ScalarNumber"],
            "type SelfReferenceSpelling" | "impl SelfReferenceSpelling" => {
                &["identity SelfReferenceSpelling"]
            }
            "type CardName"
            | "impl CardName"
            | "type CatalogProvider"
            | "impl CatalogProvider"
            | "constant REQUIRED_CATALOG_PROVIDERS"
            | "function walk_card_name" => &["identity CardName"],
            "type Agreement"
            | "type Cardinality"
            | "type Number"
            | "type Onset"
            | "type PossessiveEnding"
            | "type FeatureConstraint"
            | "type CasePosition"
            | "type PrefixPosition"
            | "type LexicalBoundary"
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
            | "impl GeneratedRoot for Possessive"
            | "impl GeneratedRoot for CardinalQuantity"
            | "impl GeneratedRoot for MannerReference"
            | "impl GeneratedRoot for CountReference"
            | "impl GeneratedRoot for ScalarReference"
            | "impl GeneratedRoot for OracleText"
            | "impl GeneratedParseRoot for Ability"
            | "impl GeneratedParseRoot for Sentence"
            | "impl GeneratedParseRoot for Possessive"
            | "impl GeneratedParseRoot for CardinalQuantity"
            | "impl GeneratedParseRoot for MannerReference"
            | "impl GeneratedParseRoot for CountReference"
            | "impl GeneratedParseRoot for ScalarReference"
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
            | "function sequence_terminator" => ALL_DECLARATION_ORIGINS.as_slice(),
            "function scan_lexical" | "function possessive_ending_at" => SCANNER_ORIGINS,
            "impl Ability" | "impl Render for Ability" | "function render_ability_with_claims" => {
                &["root Ability"]
            }
            "impl Sentence"
            | "impl Render for Sentence"
            | "function render_sentence_with_claims" => &["root Sentence"],
            "impl Possessive"
            | "impl Render for Possessive"
            | "function render_possessive_with_claims" => &["root Possessive"],
            "impl CardinalQuantity"
            | "impl Render for CardinalQuantity"
            | "function render_cardinal_quantity_with_claims" => &["root CardinalQuantity"],
            "impl MannerReference"
            | "impl Render for MannerReference"
            | "function render_manner_reference_with_claims" => &["root MannerReference"],
            "impl CountReference"
            | "impl Render for CountReference"
            | "function render_count_reference_with_claims" => &["root CountReference"],
            "impl ScalarReference"
            | "impl Render for ScalarReference"
            | "function render_scalar_reference_with_claims" => &["root ScalarReference"],
            "function write_oracle_text_render"
            | "impl Render for OracleText"
            | "function render_oracle_text_with_claims" => &["root OracleText"],
            "function render_ability_body" => &["construction paragraph", "construction triggered"],
            "trait Visitor" => VISITOR_ORIGINS.as_slice(),
            "function walk_self_reference_spelling" => &["identity SelfReferenceSpelling"],
            "type Category" => CATEGORY_ORIGINS.as_slice(),
            "type Construction"
            | "impl Construction"
            | "type RuleId"
            | "impl RuleId"
            | "function build_checked"
            | "function build" => CONSTRUCTION_ORIGINS,
            "impl Category" => &[
                "root Ability",
                "root Sentence",
                "root Possessive",
                "root CardinalQuantity",
                "root MannerReference",
                "root CountReference",
                "root ScalarReference",
                "root OracleText",
            ],
            "constant RULES" => RULE_ORIGINS.as_slice(),
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
        "type Subject",
        "type Object",
        "type SingularHead",
        "type PluralHead",
        "type NominalModifier",
        "type NegativeNominalModifier",
        "type CoordinatedNominalModifier",
        "type SingularNominal",
        "type PluralNominal",
        "type SingularCoordinationMember",
        "type PluralCoordinationMember",
        "type SingularNominalCoordination",
        "type PluralNominalCoordination",
        "type SingularSelector",
        "type PluralSelector",
        "type UnqualifiedReference",
        "type CountReference",
        "type TargetedNounPhrase",
        "type FullNounPhraseCoordination",
        "type MannerReference",
        "type ScalarReference",
        "type ControllerOwnerQualification",
        "type SingularController",
        "type ZoneReference",
        "type ZoneQualification",
        "type ScalarThreshold",
        "type ScalarMeasure",
        "type ScalarComparison",
        "type CountComparison",
        "type ScalarQualification",
        "type ControllerStage",
        "type ZoneStage",
        "type NumericStage",
        "type NounPhrase",
        "type PossessiveOwner",
        "type Possessive",
        "type VerbPhrase",
        "type Amount",
        "type CardinalQuantity",
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
        "type NominalSubject",
        "type PersonalSubject",
        "type NominalObject",
        "type PersonalObject",
        "type ReflexiveObject",
        "type CommonSingularHead",
        "type TypeSingularHead",
        "type ArtifactSubtypeSingularHead",
        "type BattleSubtypeSingularHead",
        "type CreatureSubtypeSingularHead",
        "type EnchantmentSubtypeSingularHead",
        "type LandSubtypeSingularHead",
        "type PlaneswalkerSubtypeSingularHead",
        "type SpellSubtypeSingularHead",
        "type CommonPluralHead",
        "type TypePluralHead",
        "type ArtifactSubtypePluralHead",
        "type BattleSubtypePluralHead",
        "type CreatureSubtypePluralHead",
        "type EnchantmentSubtypePluralHead",
        "type LandSubtypePluralHead",
        "type PlaneswalkerSubtypePluralHead",
        "type SpellSubtypePluralHead",
        "type ColorModifier",
        "type StatusModifier",
        "type SupertypeModifier",
        "type CommonNounModifier",
        "type TypeModifier",
        "type ArtifactSubtypeModifier",
        "type BattleSubtypeModifier",
        "type CreatureSubtypeModifier",
        "type EnchantmentSubtypeModifier",
        "type LandSubtypeModifier",
        "type PlaneswalkerSubtypeModifier",
        "type SpellSubtypeModifier",
        "type NonColorModifier",
        "type NonCommonNounModifier",
        "type NonStatusModifier",
        "type NonSupertypeModifier",
        "type NonTypeModifier",
        "type NonArtifactSubtypeModifier",
        "type NonBattleSubtypeModifier",
        "type NonCreatureSubtypeModifier",
        "type NonEnchantmentSubtypeModifier",
        "type NonLandSubtypeModifier",
        "type NonPlaneswalkerSubtypeModifier",
        "type NonSpellSubtypeModifier",
        "type NegativeModifierMember",
        "impl NegativeModifierMember",
        "type NonTargetCommonNounModifier",
        "type CoordinatedModifierMember",
        "impl CoordinatedModifierMember",
        "type BareSingularNominal",
        "type ModifiedSingularNominal",
        "type NegativeModifiedSingularNominal",
        "impl NegativeModifiedSingularNominal",
        "type BarePluralNominal",
        "type ModifiedPluralNominal",
        "type NegativeModifiedPluralNominal",
        "impl NegativeModifiedPluralNominal",
        "type BareSingularCoordinationMember",
        "type ModifiedSingularCoordinationMember",
        "type NegativeModifiedSingularCoordinationMember",
        "impl NegativeModifiedSingularCoordinationMember",
        "type BarePluralCoordinationMember",
        "type ModifiedPluralCoordinationMember",
        "type NegativeModifiedPluralCoordinationMember",
        "impl NegativeModifiedPluralCoordinationMember",
        "type SingularAndNominalCoordination",
        "impl SingularAndNominalCoordination",
        "type SingularOrNominalCoordination",
        "impl SingularOrNominalCoordination",
        "type SingularAndOrNominalCoordination",
        "impl SingularAndOrNominalCoordination",
        "type PluralAndNominalCoordination",
        "impl PluralAndNominalCoordination",
        "type PluralOrNominalCoordination",
        "impl PluralOrNominalCoordination",
        "type PluralAndOrNominalCoordination",
        "impl PluralAndOrNominalCoordination",
        "type UnmarkedSingularSelector",
        "type TargetSingularSelector",
        "type TargetSingularCoordinationSelector",
        "type OtherSingularSelector",
        "type OtherTargetSingularSelector",
        "type UnmarkedPluralSelector",
        "type UnmarkedPluralCoordinationSelector",
        "type TargetPluralSelector",
        "type TargetPluralCoordinationSelector",
        "type OtherPluralSelector",
        "type OtherTargetPluralSelector",
        "type IndefiniteReference",
        "type NamedCardReference",
        "type OrdinarySingularReference",
        "impl OrdinarySingularReference",
        "type OrdinaryPluralReference",
        "type DefiniteSingularReference",
        "type DefinitePluralReference",
        "type AnyTargetReference",
        "type AnotherReference",
        "impl AnotherReference",
        "type EachReference",
        "type AllReference",
        "type FixedReference",
        "impl FixedReference",
        "type VariableReference",
        "type UpToOneReference",
        "impl UpToOneReference",
        "type UpToManyReference",
        "impl UpToManyReference",
        "type AnyNumberReference",
        "type OneOrMoreReference",
        "type ThatMany",
        "type CountedReference",
        "type ThisReference",
        "type ThatReference",
        "type ThoseReference",
        "type DesignatedSingularReference",
        "type DesignatedPluralReference",
        "type ChosenQualityReference",
        "type PossessedSingularReference",
        "type PossessedPluralReference",
        "type PossessiveAbsoluteReference",
        "type SingularTargetedNounPhrase",
        "type PluralTargetedNounPhrase",
        "type FullAndNounPhraseCoordination",
        "impl FullAndNounPhraseCoordination",
        "type FullOrNounPhraseCoordination",
        "impl FullOrNounPhraseCoordination",
        "type FullAndOrNounPhraseCoordination",
        "impl FullAndOrNounPhraseCoordination",
        "type CoordinatedNounPhrase",
        "type SourceSelfReference",
        "impl SourceSelfReference",
        "type ThisWay",
        "type ThatMuch",
        "type YouControl",
        "impl YouControl",
        "type OpponentController",
        "impl OpponentController",
        "type OpponentControls",
        "type YouOwn",
        "impl YouOwn",
        "type PossessedZone",
        "type UnpossessedZone",
        "impl UnpossessedZone",
        "type InZone",
        "type FromZone",
        "type FixedScalarThreshold",
        "type VariableScalarThreshold",
        "type CharacteristicScalar",
        "type ManaValueScalar",
        "type ScalarOrLess",
        "type ScalarOrGreater",
        "type ScalarLessThan",
        "type ScalarGreaterThan",
        "type ScalarLessThanOrEqualTo",
        "type CountOrMore",
        "type CountOrFewer",
        "type ScalarQualificationValue",
        "type UnqualifiedControllerStage",
        "type ControllerQualifiedReference",
        "type UnqualifiedZoneStage",
        "type ZoneQualifiedReference",
        "type UnqualifiedNumericStage",
        "type ScalarQualifiedReference",
        "type CountComparisonReference",
        "impl CountComparisonReference",
        "type QualifiedNounPhrase",
        "type PossessiveSelfReference",
        "impl PossessiveSelfReference",
        "type PossessiveNoun",
        "type PossessiveValue",
        "type Destroy",
        "type Connive",
        "type DealDamage",
        "type GainLife",
        "type NumberAmount",
        "type VariableAmount",
        "type ScalarReferenceAmount",
        "type CardinalQuantityValue",
        "type TriggerWord",
        "type SubjectPronoun",
        "type ObjectPronoun",
        "type PossessiveDeterminerPronoun",
        "type PossessiveAbsolutePronoun",
        "type ReflexivePronoun",
        "type Variable",
        "type Color",
        "type Status",
        "type Designation",
        "type ChosenQuality",
        "type ControllerNoun",
        "type ScalarCharacteristic",
        "type Zone",
        "type NonCommonNoun",
        "type NonTargetCommonModifier",
        "type Supertype",
        "type CommonNoun",
        "function surface_for_common_noun",
        "type VerbLexeme",
        "function surface_for_verb_lexeme",
        "type DeclarationTypeNoun",
        "impl DeclarationTypeNoun",
        "type TypeNoun",
        "type DeclarationArtifactSubtypeNoun",
        "impl DeclarationArtifactSubtypeNoun",
        "type ArtifactSubtypeNoun",
        "type DeclarationBattleSubtypeNoun",
        "impl DeclarationBattleSubtypeNoun",
        "type BattleSubtypeNoun",
        "type DeclarationCreatureSubtypeNoun",
        "impl DeclarationCreatureSubtypeNoun",
        "type CreatureSubtypeNoun",
        "type DeclarationEnchantmentSubtypeNoun",
        "impl DeclarationEnchantmentSubtypeNoun",
        "type EnchantmentSubtypeNoun",
        "type DeclarationLandSubtypeNoun",
        "impl DeclarationLandSubtypeNoun",
        "type LandSubtypeNoun",
        "type DeclarationPlaneswalkerSubtypeNoun",
        "impl DeclarationPlaneswalkerSubtypeNoun",
        "type PlaneswalkerSubtypeNoun",
        "type DeclarationSpellSubtypeNoun",
        "impl DeclarationSpellSubtypeNoun",
        "type SpellSubtypeNoun",
        "type SelfReferenceSpelling",
        "impl SelfReferenceSpelling",
        "type CardName",
        "impl CardName",
        "type CardinalNumber",
        "type ScalarNumber",
        "type CatalogProvider",
        "impl CatalogProvider",
        "constant REQUIRED_CATALOG_PROVIDERS",
        "type Agreement",
        "type Cardinality",
        "type Number",
        "type Onset",
        "type PossessiveEnding",
        "type FeatureConstraint",
        "type CasePosition",
        "type PrefixPosition",
        "type LexicalBoundary",
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
        "impl GeneratedRoot for Possessive",
        "impl GeneratedRoot for CardinalQuantity",
        "impl GeneratedRoot for MannerReference",
        "impl GeneratedRoot for CountReference",
        "impl GeneratedRoot for ScalarReference",
        "impl GeneratedRoot for OracleText",
        "impl GeneratedParseRoot for Ability",
        "impl GeneratedParseRoot for Sentence",
        "impl GeneratedParseRoot for Possessive",
        "impl GeneratedParseRoot for CardinalQuantity",
        "impl GeneratedParseRoot for MannerReference",
        "impl GeneratedParseRoot for CountReference",
        "impl GeneratedParseRoot for ScalarReference",
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
        "function possessive_ending_at",
        "type SequenceOwner",
        "type FixedSurfaceAtom",
        "function sequence_separator",
        "function sequence_terminator",
        "function render_oracle_text_blocks_sequence",
        "function render_oracle_text",
        "function render_paragraph_sentences_sequence",
        "function render_triggered_effects_sequence",
        "function render_negative_modified_singular_nominal_modifiers_sequence",
        "function render_negative_modified_plural_nominal_modifiers_sequence",
        "function render_negative_modified_singular_coordination_member_modifiers_sequence",
        "function render_negative_modified_plural_coordination_member_modifiers_sequence",
        "function render_singular_and_nominal_coordination_members_sequence",
        "function render_singular_or_nominal_coordination_members_sequence",
        "function render_singular_and_or_nominal_coordination_members_sequence",
        "function render_plural_and_nominal_coordination_members_sequence",
        "function render_plural_or_nominal_coordination_members_sequence",
        "function render_plural_and_or_nominal_coordination_members_sequence",
        "function render_full_and_noun_phrase_coordination_members_sequence",
        "function render_full_or_noun_phrase_coordination_members_sequence",
        "function render_full_and_or_noun_phrase_coordination_members_sequence",
        "function render_document_block",
        "impl Ability",
        "impl Render for Ability",
        "function render_ability_with_claims",
        "impl Sentence",
        "impl Render for Sentence",
        "function render_sentence_with_claims",
        "impl Possessive",
        "impl Render for Possessive",
        "function render_possessive_with_claims",
        "impl CardinalQuantity",
        "impl Render for CardinalQuantity",
        "function render_cardinal_quantity_with_claims",
        "impl MannerReference",
        "impl Render for MannerReference",
        "function render_manner_reference_with_claims",
        "impl CountReference",
        "impl Render for CountReference",
        "function render_count_reference_with_claims",
        "impl ScalarReference",
        "impl Render for ScalarReference",
        "function render_scalar_reference_with_claims",
        "function write_oracle_text_render",
        "impl Render for OracleText",
        "function render_oracle_text_with_claims",
        "function render_ability_body",
        "function render_sentence_body",
        "function render_clause",
        "function render_subject",
        "function render_object",
        "function render_singular_head",
        "function render_plural_head",
        "function render_nominal_modifier",
        "function render_negative_nominal_modifier",
        "function render_coordinated_nominal_modifier",
        "function render_singular_nominal",
        "function render_plural_nominal",
        "function render_singular_coordination_member",
        "function render_plural_coordination_member",
        "function render_singular_nominal_coordination",
        "function render_plural_nominal_coordination",
        "function render_singular_selector",
        "function render_plural_selector",
        "function render_unqualified_reference",
        "function render_count_reference_body",
        "function render_targeted_noun_phrase",
        "function render_full_noun_phrase_coordination",
        "function render_scalar_reference_body",
        "function render_controller_owner_qualification",
        "function render_singular_controller",
        "function render_zone_reference",
        "function render_zone_qualification",
        "function render_scalar_threshold",
        "function render_scalar_measure",
        "function render_scalar_comparison",
        "function render_count_comparison",
        "function render_scalar_qualification",
        "function render_controller_stage",
        "function render_zone_stage",
        "function render_numeric_stage",
        "function render_noun_phrase",
        "function render_possessive_owner",
        "function render_verb_phrase",
        "function render_amount",
        "function render_cardinal_quantity_body",
        "function render_trigger_word",
        "function render_subject_pronoun",
        "function render_object_pronoun",
        "function render_possessive_determiner_pronoun",
        "function render_possessive_absolute_pronoun",
        "function render_reflexive_pronoun",
        "function render_variable",
        "function render_color",
        "function render_status",
        "function render_designation",
        "function render_chosen_quality",
        "function render_controller_noun",
        "function render_scalar_characteristic",
        "function render_zone",
        "function render_non_common_noun",
        "function render_non_target_common_modifier",
        "function render_supertype",
        "function agreement_for_subject_pronoun",
        "function agreement_for_object_pronoun",
        "function agreement_for_reflexive_pronoun",
        "function number_for_reflexive_pronoun",
        "function agreement_for_possessive_absolute_pronoun",
        "function number_for_possessive_absolute_pronoun",
        "function format_cardinal_number",
        "function number_for_cardinal_number",
        "function cardinality_for_cardinal_number",
        "function parse_cardinal_number",
        "function render_cardinal_number",
        "function format_scalar_number",
        "function parse_scalar_number",
        "function render_scalar_number",
        "function agreement_for_subject",
        "function agreement_for_singular_head",
        "function agreement_for_plural_head",
        "function agreement_for_singular_nominal",
        "function agreement_for_plural_nominal",
        "function agreement_for_singular_selector",
        "function agreement_for_plural_selector",
        "function agreement_for_unqualified_reference",
        "function agreement_for_count_reference",
        "function agreement_for_controller_stage",
        "function agreement_for_zone_stage",
        "function agreement_for_numeric_stage",
        "function agreement_for_noun_phrase",
        "function cardinality_for_cardinal_quantity",
        "function number_for_singular_head",
        "function number_for_plural_head",
        "function number_for_nominal_modifier",
        "function number_for_singular_nominal",
        "function number_for_plural_nominal",
        "function number_for_singular_selector",
        "function number_for_plural_selector",
        "function number_for_unqualified_reference",
        "function number_for_count_reference",
        "function number_for_controller_stage",
        "function number_for_zone_stage",
        "function number_for_numeric_stage",
        "function number_for_noun_phrase",
        "function number_for_possessive_owner",
        "function onset_for_singular_head",
        "function onset_for_plural_head",
        "function onset_for_nominal_modifier",
        "function onset_for_coordinated_nominal_modifier",
        "function onset_for_singular_nominal",
        "function onset_for_plural_nominal",
        "function onset_for_singular_selector",
        "function onset_for_plural_selector",
        "function onset_for_unqualified_reference",
        "function onset_for_count_reference",
        "function onset_for_controller_stage",
        "function onset_for_zone_stage",
        "function onset_for_numeric_stage",
        "function onset_for_noun_phrase",
        "function possessive_ending_for_plural_head",
        "function possessive_ending_for_possessive_owner",
        "trait Visitor",
        "function walk_ability",
        "function walk_sentence",
        "function walk_clause",
        "function walk_subject",
        "function walk_object",
        "function walk_singular_head",
        "function walk_plural_head",
        "function walk_nominal_modifier",
        "function walk_negative_nominal_modifier",
        "function walk_coordinated_nominal_modifier",
        "function walk_singular_nominal",
        "function walk_plural_nominal",
        "function walk_singular_coordination_member",
        "function walk_plural_coordination_member",
        "function walk_singular_nominal_coordination",
        "function walk_plural_nominal_coordination",
        "function walk_singular_selector",
        "function walk_plural_selector",
        "function walk_unqualified_reference",
        "function walk_count_reference",
        "function walk_targeted_noun_phrase",
        "function walk_full_noun_phrase_coordination",
        "function walk_manner_reference",
        "function walk_scalar_reference",
        "function walk_controller_owner_qualification",
        "function walk_singular_controller",
        "function walk_zone_reference",
        "function walk_zone_qualification",
        "function walk_scalar_threshold",
        "function walk_scalar_measure",
        "function walk_scalar_comparison",
        "function walk_count_comparison",
        "function walk_scalar_qualification",
        "function walk_controller_stage",
        "function walk_zone_stage",
        "function walk_numeric_stage",
        "function walk_noun_phrase",
        "function walk_possessive_owner",
        "function walk_possessive",
        "function walk_verb_phrase",
        "function walk_amount",
        "function walk_cardinal_quantity",
        "function walk_paragraph_sentences_sequence",
        "function walk_triggered_effects_sequence",
        "function walk_negative_modified_singular_nominal_modifiers_sequence",
        "function walk_negative_modified_plural_nominal_modifiers_sequence",
        "function walk_negative_modified_singular_coordination_member_modifiers_sequence",
        "function walk_negative_modified_plural_coordination_member_modifiers_sequence",
        "function walk_singular_and_nominal_coordination_members_sequence",
        "function walk_singular_or_nominal_coordination_members_sequence",
        "function walk_singular_and_or_nominal_coordination_members_sequence",
        "function walk_plural_and_nominal_coordination_members_sequence",
        "function walk_plural_or_nominal_coordination_members_sequence",
        "function walk_plural_and_or_nominal_coordination_members_sequence",
        "function walk_full_and_noun_phrase_coordination_members_sequence",
        "function walk_full_or_noun_phrase_coordination_members_sequence",
        "function walk_full_and_or_noun_phrase_coordination_members_sequence",
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
        "function walk_nominal_subject",
        "function walk_personal_subject",
        "function walk_nominal_object",
        "function walk_personal_object",
        "function walk_reflexive_object",
        "function walk_common_singular_head",
        "function walk_type_singular_head",
        "function walk_artifact_subtype_singular_head",
        "function walk_battle_subtype_singular_head",
        "function walk_creature_subtype_singular_head",
        "function walk_enchantment_subtype_singular_head",
        "function walk_land_subtype_singular_head",
        "function walk_planeswalker_subtype_singular_head",
        "function walk_spell_subtype_singular_head",
        "function walk_common_plural_head",
        "function walk_type_plural_head",
        "function walk_artifact_subtype_plural_head",
        "function walk_battle_subtype_plural_head",
        "function walk_creature_subtype_plural_head",
        "function walk_enchantment_subtype_plural_head",
        "function walk_land_subtype_plural_head",
        "function walk_planeswalker_subtype_plural_head",
        "function walk_spell_subtype_plural_head",
        "function walk_color_modifier",
        "function walk_status_modifier",
        "function walk_supertype_modifier",
        "function walk_common_noun_modifier",
        "function walk_type_modifier",
        "function walk_artifact_subtype_modifier",
        "function walk_battle_subtype_modifier",
        "function walk_creature_subtype_modifier",
        "function walk_enchantment_subtype_modifier",
        "function walk_land_subtype_modifier",
        "function walk_planeswalker_subtype_modifier",
        "function walk_spell_subtype_modifier",
        "function walk_non_color_modifier",
        "function walk_non_common_noun_modifier",
        "function walk_non_status_modifier",
        "function walk_non_supertype_modifier",
        "function walk_non_type_modifier",
        "function walk_non_artifact_subtype_modifier",
        "function walk_non_battle_subtype_modifier",
        "function walk_non_creature_subtype_modifier",
        "function walk_non_enchantment_subtype_modifier",
        "function walk_non_land_subtype_modifier",
        "function walk_non_planeswalker_subtype_modifier",
        "function walk_non_spell_subtype_modifier",
        "function walk_negative_modifier_member",
        "function walk_non_target_common_noun_modifier",
        "function walk_coordinated_modifier_member",
        "function walk_bare_singular_nominal",
        "function walk_modified_singular_nominal",
        "function walk_negative_modified_singular_nominal",
        "function walk_bare_plural_nominal",
        "function walk_modified_plural_nominal",
        "function walk_negative_modified_plural_nominal",
        "function walk_bare_singular_coordination_member",
        "function walk_modified_singular_coordination_member",
        "function walk_negative_modified_singular_coordination_member",
        "function walk_bare_plural_coordination_member",
        "function walk_modified_plural_coordination_member",
        "function walk_negative_modified_plural_coordination_member",
        "function walk_singular_and_nominal_coordination",
        "function walk_singular_or_nominal_coordination",
        "function walk_singular_and_or_nominal_coordination",
        "function walk_plural_and_nominal_coordination",
        "function walk_plural_or_nominal_coordination",
        "function walk_plural_and_or_nominal_coordination",
        "function walk_unmarked_singular_selector",
        "function walk_target_singular_selector",
        "function walk_target_singular_coordination_selector",
        "function walk_other_singular_selector",
        "function walk_other_target_singular_selector",
        "function walk_unmarked_plural_selector",
        "function walk_unmarked_plural_coordination_selector",
        "function walk_target_plural_selector",
        "function walk_target_plural_coordination_selector",
        "function walk_other_plural_selector",
        "function walk_other_target_plural_selector",
        "function walk_indefinite_reference",
        "function walk_named_card_reference",
        "function walk_ordinary_singular_reference",
        "function walk_ordinary_plural_reference",
        "function walk_definite_singular_reference",
        "function walk_definite_plural_reference",
        "function walk_any_target_reference",
        "function walk_another_reference",
        "function walk_each_reference",
        "function walk_all_reference",
        "function walk_fixed_reference",
        "function walk_variable_reference",
        "function walk_up_to_one_reference",
        "function walk_up_to_many_reference",
        "function walk_any_number_reference",
        "function walk_one_or_more_reference",
        "function walk_that_many",
        "function walk_counted_reference",
        "function walk_this_reference",
        "function walk_that_reference",
        "function walk_those_reference",
        "function walk_designated_singular_reference",
        "function walk_designated_plural_reference",
        "function walk_chosen_quality_reference",
        "function walk_possessed_singular_reference",
        "function walk_possessed_plural_reference",
        "function walk_possessive_absolute_reference",
        "function walk_singular_targeted_noun_phrase",
        "function walk_plural_targeted_noun_phrase",
        "function walk_full_and_noun_phrase_coordination",
        "function walk_full_or_noun_phrase_coordination",
        "function walk_full_and_or_noun_phrase_coordination",
        "function walk_coordinated_noun_phrase",
        "function walk_source_self_reference",
        "function walk_this_way",
        "function walk_that_much",
        "function walk_you_control",
        "function walk_opponent_controller",
        "function walk_opponent_controls",
        "function walk_you_own",
        "function walk_possessed_zone",
        "function walk_unpossessed_zone",
        "function walk_in_zone",
        "function walk_from_zone",
        "function walk_fixed_scalar_threshold",
        "function walk_variable_scalar_threshold",
        "function walk_characteristic_scalar",
        "function walk_mana_value_scalar",
        "function walk_scalar_or_less",
        "function walk_scalar_or_greater",
        "function walk_scalar_less_than",
        "function walk_scalar_greater_than",
        "function walk_scalar_less_than_or_equal_to",
        "function walk_count_or_more",
        "function walk_count_or_fewer",
        "function walk_scalar_qualification_value",
        "function walk_unqualified_controller_stage",
        "function walk_controller_qualified_reference",
        "function walk_unqualified_zone_stage",
        "function walk_zone_qualified_reference",
        "function walk_unqualified_numeric_stage",
        "function walk_scalar_qualified_reference",
        "function walk_count_comparison_reference",
        "function walk_qualified_noun_phrase",
        "function walk_possessive_self_reference",
        "function walk_possessive_noun",
        "function walk_possessive_value",
        "function walk_destroy",
        "function walk_connive",
        "function walk_deal_damage",
        "function walk_gain_life",
        "function walk_number_amount",
        "function walk_variable_amount",
        "function walk_scalar_reference_amount",
        "function walk_cardinal_quantity_value",
        "function walk_trigger_word",
        "function walk_subject_pronoun",
        "function walk_object_pronoun",
        "function walk_possessive_determiner_pronoun",
        "function walk_possessive_absolute_pronoun",
        "function walk_reflexive_pronoun",
        "function walk_variable",
        "function walk_color",
        "function walk_status",
        "function walk_designation",
        "function walk_chosen_quality",
        "function walk_controller_noun",
        "function walk_scalar_characteristic",
        "function walk_zone",
        "function walk_non_common_noun",
        "function walk_non_target_common_modifier",
        "function walk_supertype",
        "function walk_self_reference_spelling",
        "function walk_card_name",
        "function walk_common_noun",
        "function walk_verb_lexeme",
        "function walk_cardinal_number",
        "function walk_scalar_number",
        "function walk_declaration_type_noun",
        "function walk_type_noun",
        "function walk_declaration_artifact_subtype_noun",
        "function walk_artifact_subtype_noun",
        "function walk_declaration_battle_subtype_noun",
        "function walk_battle_subtype_noun",
        "function walk_declaration_creature_subtype_noun",
        "function walk_creature_subtype_noun",
        "function walk_declaration_enchantment_subtype_noun",
        "function walk_enchantment_subtype_noun",
        "function walk_declaration_land_subtype_noun",
        "function walk_land_subtype_noun",
        "function walk_declaration_planeswalker_subtype_noun",
        "function walk_planeswalker_subtype_noun",
        "function walk_declaration_spell_subtype_noun",
        "function walk_spell_subtype_noun",
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
        assert_eq!(verbs, ["Deal", "Gain", "Control", "Own", "Be"]);

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

        assert_eq!(EXPECTED_ITEM_KEYS.len(), 789);
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
             // roots (8)\n\
             // - root Ability\n\
             // - root Sentence\n\
             // - root Possessive\n\
             // - root CardinalQuantity\n\
             // - root MannerReference\n\
             // - root CountReference\n\
             // - root ScalarReference\n\
             // - root OracleText\n"
        );
    }

    #[test]
    fn expansion_is_stable_and_the_generated_rust_reparses() {
        let first = expand_source(PRODUCTION_SOURCE).expect("production declaration expands");
        let second = expand_source(PRODUCTION_SOURCE).expect("repeat expands");
        assert_eq!(first, second);

        let parsed = syn::parse_file(&first).expect("comment headings preserve reparsable Rust");
        assert_eq!(parsed.items.len(), 789);
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
