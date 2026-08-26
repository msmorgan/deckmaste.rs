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
#[cfg(test)]
mod plan09_frontier;
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

    #[derive(Debug, clap::Parser)]
    struct PlanGateCli {
        #[command(flatten)]
        args: timing::PlanGateArgs,
    }

    const PRODUCTION_SOURCE: &str = include_str!("../../deckmaste_english_v2/src/constructions.rs");
    const PLAN06_COVERED_IDS: &str = include_str!("english_v2/plan06_covered_ids.txt");
    const PLAN07_TARGETS: &str = include_str!("english_v2/plan07_targets.tsv");
    const PLAN08_CANDIDATE_POOL: &str = include_str!("english_v2/plan08_candidate_pool.tsv");
    const PLAN08_CANDIDATE_RESULTS: &str = include_str!("english_v2/plan08_candidate_results.tsv");
    const PLAN09_EXPANSION: &str = include_str!("english_v2/plan09_expansion.tsv");
    const PLAN09_TASK7_EXPANSION: &str = include_str!("english_v2/plan09_task7_expansion.tsv");
    const PLAN09_TASK8_EXPANSION: &str = include_str!("english_v2/plan09_task8_expansion.tsv");
    type ClosedLexemeDeclarations = std::collections::BTreeSet<String>;

    #[derive(Debug, Clone, Copy, PartialEq, Eq)]
    enum Plan08DeferredAuthority {
        Plan09TriggerPredicate {
            witness: Plan09TriggerWitness,
        },
        Plan09ActivatedCostAction,
        Plan10KeywordReference {
            catalog_entry: &'static str,
            prefix: &'static str,
        },
        Plan10AbilityWord {
            catalog_entry: &'static str,
        },
        Plan10FlavorWord {
            label: &'static str,
        },
        Plan10AdvancedCost,
    }

    #[derive(Debug, Clone, Copy, PartialEq, Eq)]
    enum Plan09TriggerWitness {
        PredicateRoot,
        GainAmount,
        DamageAmount,
        PredicateRootOrTemporalBody,
    }

    impl Plan08DeferredAuthority {
        const fn category(self) -> &'static str {
            match self {
                Self::Plan09TriggerPredicate { .. } | Self::Plan09ActivatedCostAction => {
                    "plan09-predicate"
                }
                Self::Plan10KeywordReference { .. }
                | Self::Plan10AbilityWord { .. }
                | Self::Plan10FlavorWord { .. }
                | Self::Plan10AdvancedCost => "plan10-attachment",
            }
        }
    }

    #[derive(Debug, Clone, Copy)]
    struct Plan08DeferredRule {
        pool_family: &'static str,
        failure_surface: &'static str,
        authority: Plan08DeferredAuthority,
    }

    #[derive(Clone, Copy)]
    struct Plan08BoundaryCatalogs<'a> {
        keyword_abilities: &'a str,
        ability_words: &'a str,
    }

    const fn plan09_trigger(failure_surface: &'static str) -> Plan08DeferredRule {
        Plan08DeferredRule {
            pool_family: "trigger-effect-selects",
            failure_surface,
            authority: Plan08DeferredAuthority::Plan09TriggerPredicate {
                witness: Plan09TriggerWitness::PredicateRoot,
            },
        }
    }

    const fn plan09_trigger_witness(
        failure_surface: &'static str,
        witness: Plan09TriggerWitness,
    ) -> Plan08DeferredRule {
        Plan08DeferredRule {
            pool_family: "trigger-effect-selects",
            failure_surface,
            authority: Plan08DeferredAuthority::Plan09TriggerPredicate { witness },
        }
    }

    const fn plan09_cost(failure_surface: &'static str) -> Plan08DeferredRule {
        Plan08DeferredRule {
            pool_family: "activated-effect-selects",
            failure_surface,
            authority: Plan08DeferredAuthority::Plan09ActivatedCostAction,
        }
    }

    const PLAN08_DEFERRED_RULES: [Plan08DeferredRule; 46] = [
        plan09_cost("Discard"),
        plan09_cost("Exile"),
        plan09_cost("Pay"),
        plan09_cost("Put"),
        plan09_cost("Remove"),
        plan09_cost("Sacrifice"),
        plan09_cost("Tap"),
        Plan08DeferredRule {
            pool_family: "activated-effect-selects",
            failure_surface: "Cohort",
            authority: Plan08DeferredAuthority::Plan10AbilityWord {
                catalog_entry: "Cohort",
            },
        },
        Plan08DeferredRule {
            pool_family: "activated-effect-selects",
            failure_surface: "Scorching",
            authority: Plan08DeferredAuthority::Plan10FlavorWord {
                label: "Scorching Ray",
            },
        },
        Plan08DeferredRule {
            pool_family: "activated-effect-selects",
            failure_surface: "or",
            authority: Plan08DeferredAuthority::Plan10AdvancedCost,
        },
        plan09_trigger("activate"),
        plan09_trigger("activates"),
        plan09_trigger("are"),
        plan09_trigger("attacks"),
        plan09_trigger("becomes"),
        plan09_trigger("blocks"),
        plan09_trigger("cast"),
        plan09_trigger("casts"),
        plan09_trigger_witness("combat", Plan09TriggerWitness::DamageAmount),
        plan09_trigger("control"),
        plan09_trigger("controls"),
        plan09_trigger("causes"),
        plan09_trigger_witness("create", Plan09TriggerWitness::PredicateRootOrTemporalBody),
        plan09_trigger_witness("damage", Plan09TriggerWitness::DamageAmount),
        plan09_trigger("didn't"),
        plan09_trigger("dies"),
        plan09_trigger("discards"),
        plan09_trigger_witness("draw", Plan09TriggerWitness::PredicateRootOrTemporalBody),
        plan09_trigger_witness("draws", Plan09TriggerWitness::PredicateRootOrTemporalBody),
        plan09_trigger("enter"),
        plan09_trigger("enters"),
        plan09_trigger("has"),
        plan09_trigger("have"),
        plan09_trigger("is"),
        plan09_trigger("leave"),
        plan09_trigger_witness("life", Plan09TriggerWitness::GainAmount),
        plan09_trigger("loses"),
        plan09_trigger("put"),
        plan09_trigger("roll"),
        plan09_trigger("sacrifice"),
        plan09_trigger("sacrifices"),
        plan09_trigger("taps"),
        plan09_trigger("cycles"),
        Plan08DeferredRule {
            pool_family: "trigger-effect-selects",
            failure_surface: "flying",
            authority: Plan08DeferredAuthority::Plan10KeywordReference {
                catalog_entry: "Flying",
                prefix: "",
            },
        },
        plan09_trigger("scry"),
        Plan08DeferredRule {
            pool_family: "trigger-effect-selects",
            failure_surface: "without",
            authority: Plan08DeferredAuthority::Plan10KeywordReference {
                catalog_entry: "Flying",
                prefix: "without ",
            },
        },
    ];

    #[derive(Clone, Copy)]
    struct Plan08AttachmentWitness {
        id: &'static str,
        failure_span: deckmaste_english_v2::parser::TextSpan,
        supported_primary: &'static str,
        supported_secondary: Option<&'static str>,
        primary_selects: bool,
    }

    const PLAN08_ATTACHMENT_WITNESSES: [Plan08AttachmentWitness; 3] = [
        Plan08AttachmentWitness {
            id: "298faf6c5ee4c3846b7426789dea4ae776cf6890f4016e3035388c353e4d6c0b",
            failure_span: deckmaste_english_v2::parser::TextSpan { start: 9, end: 11 },
            supported_primary: "{3}, {T}: This artifact deals 1 damage to any target.",
            supported_secondary: Some("{R}, {T}: This artifact deals 1 damage to any target."),
            primary_selects: true,
        },
        Plan08AttachmentWitness {
            id: "3bdbeecb5f1fbcaf544f3da00141d4b6b95eee9856a64aa72cfaf3c8aa65d051",
            failure_span: deckmaste_english_v2::parser::TextSpan { start: 0, end: 9 },
            supported_primary: "{8}: This creature deals 4 damage to each opponent.",
            supported_secondary: None,
            primary_selects: true,
        },
        Plan08AttachmentWitness {
            id: "d88429519c5e257ef7e45666074b18c0b0788a97cb6df2cad78914fcd7c84705",
            failure_span: deckmaste_english_v2::parser::TextSpan { start: 0, end: 6 },
            supported_primary: "{T}, Tap an untapped Ally you control: You gain 2 life.",
            supported_secondary: Some("{T}: You gain 2 life."),
            primary_selects: true,
        },
    ];

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

    fn plan08_deferred_rule(pool_family: &str, failure_surface: &str) -> Plan08DeferredRule {
        let rules = PLAN08_DEFERRED_RULES
            .iter()
            .copied()
            .filter(|rule| {
                rule.pool_family == pool_family && rule.failure_surface == failure_surface
            })
            .collect::<Vec<_>>();
        assert_eq!(
            rules.len(),
            1,
            "deferred boundary needs exactly one independent scope rule: {pool_family} {failure_surface:?}",
        );
        rules[0]
    }

    fn assert_plan08_deferred_authority(
        rule: Plan08DeferredRule,
        parser: &deckmaste_english_v2::parser::Parser,
        context: &deckmaste_english_v2::context::ParseContext<'_>,
        id: &str,
        text: &str,
        span: deckmaste_english_v2::parser::TextSpan,
        catalogs: Plan08BoundaryCatalogs<'_>,
    ) {
        let has_entry = |catalog: &str, entry: &str| catalog.lines().any(|row| row == entry);
        match rule.authority {
            Plan08DeferredAuthority::Plan09TriggerPredicate { witness } => {
                assert_eq!(rule.pool_family, "trigger-effect-selects");
                let prefix = &text[..span.start];
                let modal_body = "choose one —\n• You gain 1 life.\n• You gain 2 life.";
                let mut controlled = Vec::new();
                if matches!(
                    witness,
                    Plan09TriggerWitness::PredicateRoot
                        | Plan09TriggerWitness::PredicateRootOrTemporalBody
                ) {
                    controlled.extend(
                        ["gain", "gains"]
                            .map(|verb| format!("{prefix}{verb} 1 life, {modal_body}")),
                    );
                }
                if matches!(witness, Plan09TriggerWitness::GainAmount) {
                    controlled.push(format!("{prefix}1 life, {modal_body}"));
                }
                if matches!(witness, Plan09TriggerWitness::DamageAmount) {
                    controlled.push(format!("{prefix}1 damage to you, {modal_body}"));
                }
                if matches!(witness, Plan09TriggerWitness::PredicateRootOrTemporalBody) {
                    controlled
                        .extend(["gain", "gains"].map(|verb| format!("{prefix}{verb} 1 life.")));
                }
                let selected = controlled
                    .iter()
                    .filter(|witness| {
                        parser.analyze(witness, context).outcome()
                            == deckmaste_english_v2::parser::ParseAnalysisOutcome::Selected
                    })
                    .collect::<Vec<_>>();
                assert_eq!(selected.len(), 1, "{id}: {controlled:#?}");
                assert_unique_plan08_ability(parser, context, id, selected[0]);
            }
            Plan08DeferredAuthority::Plan09ActivatedCostAction => {
                assert_eq!(rule.pool_family, "activated-effect-selects");
                let colon = text
                    .find(": ")
                    .expect("activated witness has a cost boundary");
                assert!(span.end <= colon, "{id}");
                let prefix = &text[..span.start];
                let controlled = format!("{prefix}{{T}}{}", &text[colon..]);
                assert!(controlled.starts_with(prefix), "{id}");
                assert_unique_plan08_ability(parser, context, id, &controlled);
            }
            Plan08DeferredAuthority::Plan10KeywordReference {
                catalog_entry,
                prefix,
            } => {
                assert!(has_entry(catalogs.keyword_abilities, catalog_entry));
                let expected = format!("{prefix}{}", catalog_entry.to_ascii_lowercase());
                assert!(
                    text[span.start..].starts_with(&expected),
                    "{id}: {expected:?}"
                );
            }
            Plan08DeferredAuthority::Plan10AbilityWord { catalog_entry } => {
                assert!(has_entry(catalogs.ability_words, catalog_entry));
                assert!(text.starts_with(&format!("{catalog_entry} — ")));
            }
            Plan08DeferredAuthority::Plan10FlavorWord { label } => {
                assert!(text.starts_with(&format!("{label} — ")));
            }
            Plan08DeferredAuthority::Plan10AdvancedCost => {
                assert_eq!(rule.pool_family, "activated-effect-selects");
                assert_eq!(&text[span.start..span.end], "or");
            }
        }
    }

    fn assert_unique_plan08_ability(
        parser: &deckmaste_english_v2::parser::Parser,
        context: &deckmaste_english_v2::context::ParseContext<'_>,
        id: &str,
        text: &str,
    ) {
        use deckmaste_english_v2::parser::ParseAnalysisOutcome;
        use deckmaste_english_v2::parser::SelectionResolution;

        let analysis = parser.analyze(text, context);
        assert_eq!(analysis.outcome(), ParseAnalysisOutcome::Selected, "{id}");
        let decision = analysis.decision().expect("controlled witness decision");
        assert_eq!(decision.resolution(), SelectionResolution::Unique, "{id}");
        assert_eq!(decision.survivors().len(), 1, "{id}");
        assert!(decision.exception_uses().is_empty(), "{id}");
        let ownership = analysis.ownership().expect("controlled witness ownership");
        assert!(ownership.failures().is_empty(), "{id}: {ownership:?}");
        assert!(ownership.summary().covered(), "{id}: {ownership:?}");
        assert_eq!(ownership.rendered_text(), text, "{id}");
        assert_eq!(ownership.summary().gap_spans(), 0, "{id}");
        assert_eq!(ownership.summary().overlap_spans(), 0, "{id}");
        assert_eq!(ownership.summary().synthetic_claims(), 0, "{id}");
        assert_eq!(ownership.summary().provenance_plan_mismatches(), 0, "{id}");
    }

    fn assert_plan08_attachment_witness(
        witness: Plan08AttachmentWitness,
        parser: &deckmaste_english_v2::parser::Parser,
        context: &deckmaste_english_v2::context::ParseContext<'_>,
    ) {
        use deckmaste_english_v2::parser::ParseError;

        if witness.primary_selects {
            assert_unique_plan08_ability(parser, context, witness.id, witness.supported_primary);
        } else {
            let ParseError::Failure { span, .. } = parser
                .parse(witness.supported_primary, context)
                .expect_err("label removal advances to the next unsupported boundary")
            else {
                panic!(
                    "label removal has an ordinary later failure: {}",
                    witness.id
                );
            };
            assert!(span.start > 0, "{}", witness.id);
        }
        if let Some(secondary) = witness.supported_secondary {
            assert_unique_plan08_ability(parser, context, witness.id, secondary);
        }
    }

    #[test]
    fn plan_gate_cli_accepts_plan08_without_retiring_plan07() {
        use clap::Parser as _;

        let plan08 =
            PlanGateCli::try_parse_from(["plan-gate", "--profile", "08", "--gate", "parse"])
                .expect("Plan 08 timing profile parses");
        assert_eq!(
            format!("{:?}", plan08.args),
            "PlanGateArgs { profile: Plan08, gate: Parse }"
        );
        let plan07 =
            PlanGateCli::try_parse_from(["plan-gate", "--profile", "07", "--gate", "parse"])
                .expect("Plan 07 remains selectable for historical evidence");
        assert_eq!(
            format!("{:?}", plan07.args),
            "PlanGateArgs { profile: Plan07, gate: Parse }"
        );
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

    #[allow(
        clippy::too_many_lines,
        reason = "the complete deferred structural census is deliberately literal"
    )]
    #[test]
    fn plan08_deferred_boundary_witnesses_are_structural_not_punctuation() {
        use deckmaste_english_v2::parser::ParseError;

        let data =
            Path::new(env!("CARGO_MANIFEST_DIR")).join("../../data/mtgjson/AtomicCards.json");
        let corpus = corpus::Corpus::load(&data).expect("production MTGJSON corpus loads");
        let units = corpus
            .units()
            .iter()
            .map(|unit| (unit.id(), unit))
            .collect::<BTreeMap<_, _>>();
        let rows = PLAN08_CANDIDATE_RESULTS
            .lines()
            .skip(10)
            .map(|line| {
                let fields = line.split('\t').collect::<Vec<_>>();
                (fields[3], fields)
            })
            .collect::<BTreeMap<_, _>>();
        let parser = parser_from_builtin_v2().expect("production English-v2 parser loads");

        let catalog_root = Path::new(env!("CARGO_MANIFEST_DIR")).join("../../data/gen/catalogs");
        let keyword_abilities = fs::read_to_string(catalog_root.join("keyword-abilities.txt"))
            .expect("frozen keyword-ability catalog loads");
        let ability_words = fs::read_to_string(catalog_root.join("ability-words.txt"))
            .expect("frozen ability-word catalog loads");
        let review_witnesses = [
            (
                "c82603ae98da0c4df2e8e05fffe027dab0d880ae8daedee2d34b0bca20e50ec9",
                deckmaste_english_v2::parser::TextSpan { start: 25, end: 31 },
                "flying",
                "plan10-attachment",
            ),
            (
                "f13464c138a233223f1d566f1a74da3239942c3c36f9056b03c8db9f4bf90ef7",
                deckmaste_english_v2::parser::TextSpan { start: 20, end: 27 },
                "without",
                "plan10-attachment",
            ),
        ];
        let mut review_categories = Vec::new();
        for (id, expected_span, expected_surface, expected_category) in review_witnesses {
            let fields = rows
                .get(id)
                .unwrap_or_else(|| panic!("manifest contains review witness {id}"));
            let unit = units
                .get(id)
                .unwrap_or_else(|| panic!("corpus contains review witness {id}"));
            let context = deckmaste_english_v2::context::ParseContext::new(
                unit.context_name(),
                unit.is_legendary(),
                unit.context_onset(),
            )
            .expect("review-witness context is valid");
            let ParseError::Failure { span, .. } = parser
                .parse_oracle_text(unit.text(), &context)
                .expect_err("review witness reaches its missing boundary")
            else {
                panic!("review witness has an ordinary parse failure: {id}");
            };
            assert_eq!(span, expected_span, "{id}");
            assert_eq!(&unit.text()[span.start..span.end], expected_surface, "{id}");
            assert_eq!(fields[0], expected_category, "{id}");
            review_categories.push((id, fields[0]));
        }
        for (id, replacement_end, replacement) in [
            (
                "c82603ae98da0c4df2e8e05fffe027dab0d880ae8daedee2d34b0bca20e50ec9",
                31,
                "power 2 or less",
            ),
            (
                "f13464c138a233223f1d566f1a74da3239942c3c36f9056b03c8db9f4bf90ef7",
                34,
                "with power 2 or less",
            ),
        ] {
            let unit = units
                .get(id)
                .unwrap_or_else(|| panic!("corpus contains keyword-reference witness {id}"));
            let fields = rows
                .get(id)
                .unwrap_or_else(|| panic!("manifest contains keyword-reference witness {id}"));
            let start = fields[12].parse::<usize>().expect("failure start");
            let controlled = format!(
                "{}{replacement}{}",
                &unit.text()[..start],
                &unit.text()[replacement_end..],
            );
            assert!(controlled.starts_with(&unit.text()[..start]), "{id}");
            let context = deckmaste_english_v2::context::ParseContext::new(
                unit.context_name(),
                unit.is_legendary(),
                unit.context_onset(),
            )
            .expect("keyword-reference witness context is valid");
            if id == "c82603ae98da0c4df2e8e05fffe027dab0d880ae8daedee2d34b0bca20e50ec9" {
                assert_unique_plan08_ability(&parser, &context, id, &controlled);
            } else {
                let ParseError::Failure { span, .. } = parser
                    .parse_oracle_text(&controlled, &context)
                    .expect_err("the supported modifier advances to the coordinated object")
                else {
                    panic!("keyword-reference control has an ordinary failure: {id}");
                };
                assert_eq!(
                    span,
                    deckmaste_english_v2::parser::TextSpan { start: 82, end: 83 },
                    "{id}",
                );
                assert_eq!(&controlled[span.start..span.end], ",", "{id}");
            }
        }
        let mut predicted_counts = BTreeMap::new();
        let mut used_rules = BTreeSet::new();
        let mut mismatches = Vec::new();
        for fields in rows
            .values()
            .filter(|fields| fields[0] == "plan10-attachment")
        {
            let id = fields[3];
            let unit = units
                .get(id)
                .unwrap_or_else(|| panic!("corpus contains {id}"));
            let context = deckmaste_english_v2::context::ParseContext::new(
                unit.context_name(),
                unit.is_legendary(),
                unit.context_onset(),
            )
            .expect("production candidate context is valid");
            let ParseError::Failure { span, .. } = parser
                .parse_oracle_text(unit.text(), &context)
                .expect_err("deferred candidate has an ordinary parse failure")
            else {
                panic!("deferred candidate has an ordinary parse failure: {id}");
            };
            let failure_surface = &unit.text()[span.start..span.end];
            let rule = plan08_deferred_rule(fields[1], failure_surface);
            assert_plan08_deferred_authority(
                rule,
                &parser,
                &context,
                id,
                unit.text(),
                span,
                Plan08BoundaryCatalogs {
                    keyword_abilities: &keyword_abilities,
                    ability_words: &ability_words,
                },
            );
            used_rules.insert((rule.pool_family, rule.failure_surface));
            let expected_category = rule.authority.category();
            *predicted_counts.entry(expected_category).or_insert(0usize) += 1;
            if fields[0] != expected_category {
                mismatches.push((id, fields[4], fields[0], expected_category));
            }
        }
        let expected_rules = PLAN08_DEFERRED_RULES
            .iter()
            .filter(|rule| rule.authority.category() == "plan10-attachment")
            .map(|rule| (rule.pool_family, rule.failure_surface))
            .collect::<BTreeSet<_>>();
        assert_eq!(used_rules, expected_rules);
        assert_eq!(
            predicted_counts.into_iter().collect::<Vec<_>>(),
            [("plan10-attachment", 5usize)],
        );
        assert!(
            mismatches.is_empty(),
            "manifest disagrees with {mismatches:#?}"
        );
        assert_eq!(
            review_categories,
            [
                (
                    "c82603ae98da0c4df2e8e05fffe027dab0d880ae8daedee2d34b0bca20e50ec9",
                    "plan10-attachment",
                ),
                (
                    "f13464c138a233223f1d566f1a74da3239942c3c36f9056b03c8db9f4bf90ef7",
                    "plan10-attachment",
                ),
            ],
        );

        for witness in PLAN08_ATTACHMENT_WITNESSES {
            let id = witness.id;
            let fields = rows
                .get(id)
                .unwrap_or_else(|| panic!("manifest contains {id}"));
            assert_eq!(fields[0], "plan10-attachment", "{id}");
            let unit = units
                .get(id)
                .unwrap_or_else(|| panic!("corpus contains {id}"));
            let context = deckmaste_english_v2::context::ParseContext::new(
                unit.context_name(),
                unit.is_legendary(),
                unit.context_onset(),
            )
            .expect("production candidate context is valid");
            let ParseError::Failure { span, .. } = parser
                .parse_oracle_text(unit.text(), &context)
                .expect_err("deferred candidate has an ordinary parse failure")
            else {
                panic!("deferred candidate has an ordinary parse failure: {id}");
            };
            assert_eq!(span, witness.failure_span, "{id}");
            assert_plan08_attachment_witness(witness, &parser, &context);
        }
    }

    #[allow(
        clippy::too_many_lines,
        reason = "the frozen 427-row closure inventory is deliberately literal"
    )]
    #[test]
    fn plan08_candidate_results_are_a_complete_authenticated_partition() {
        use deckmaste_english_v2::parser::ParseAnalysisOutcome;
        use deckmaste_english_v2::parser::ParseError;
        use deckmaste_english_v2::parser::SelectionResolution;
        use deckmaste_english_v2::parser::TextSpan;

        const EXPECTED_HEADERS: [&str; 10] = [
            "# English v2 Plan 08 authenticated candidate results",
            "# classifier_schema=1",
            "# source_fingerprint=e85359d7b8c578df13dff2fdf7c743a520a5b367d5ed25ab0a5f03cb8b3637dd",
            "# candidate_pool_sha256=8b9870dba508681ecc25dc43b21500d82cd941d2408456810ad39f799ffb425f",
            "# baseline_covered=608",
            "# baseline_status=parse_failure",
            "# candidates=427",
            "# categories=plan08-selected:67,plan09-predicate:355,plan10-attachment:5,plan08-defect:0",
            "# selected_families=ability.activated:56,ability.triggered:11",
            "# columns=category\\tpool_family\\tplan08_family\\tid\\tcard_name_json\\tface_name_json\\tside_json\\tcontext_name_json\\tis_legendary\\tcontext_onset\\toracle_text_json\\textracted_effect_json\\tfailure_start\\tfailure_end\\tboundary",
        ];
        const EXPECTED_COUNTS: [(&str, usize); 3] = [
            ("plan08-selected", 67),
            ("plan09-predicate", 355),
            ("plan10-attachment", 5),
        ];
        const EXPECTED_SELECTED_FAMILIES: [(&str, usize); 2] =
            [("ability.activated", 56), ("ability.triggered", 11)];

        fn decode(value: &str) -> String {
            serde_json::from_str(value).expect("manifest JSON string is valid")
        }

        fn capitalize_initial(value: &str) -> String {
            let mut value = value.to_owned();
            let first = value
                .as_bytes()
                .first()
                .copied()
                .expect("extracted effect is nonempty");
            assert!(first.is_ascii_alphabetic());
            value.replace_range(0..1, &(first as char).to_ascii_uppercase().to_string());
            value
        }

        assert_eq!(
            sha256_hex(PLAN08_CANDIDATE_POOL.as_bytes()),
            "8b9870dba508681ecc25dc43b21500d82cd941d2408456810ad39f799ffb425f",
        );
        assert_eq!(
            sha256_hex(PLAN08_CANDIDATE_RESULTS.as_bytes()),
            "918d7eaf1d0d66342b308b7290a2ab19621325749c59de9a4341a6a56bfbeac4",
        );
        assert_eq!(
            PLAN08_CANDIDATE_RESULTS
                .lines()
                .take(EXPECTED_HEADERS.len())
                .collect::<Vec<_>>(),
            EXPECTED_HEADERS,
        );
        assert!(PLAN08_CANDIDATE_RESULTS.ends_with('\n'));

        let pool = PLAN08_CANDIDATE_POOL
            .lines()
            .skip(9)
            .map(|line| {
                let fields = line.split('\t').collect::<Vec<_>>();
                assert_eq!(fields.len(), 7);
                (fields[1], fields)
            })
            .collect::<BTreeMap<_, _>>();
        assert_eq!(pool.len(), 427);
        let rows = PLAN08_CANDIDATE_RESULTS
            .lines()
            .skip(EXPECTED_HEADERS.len())
            .map(|line| line.split('\t').collect::<Vec<_>>())
            .collect::<Vec<_>>();
        assert_eq!(rows.len(), 427);
        let plan09_boundaries = include_str!("english_v2/plan09_task7_progress.tsv")
            .lines()
            .filter(|line| line.starts_with("boundary\t"))
            .map(|line| {
                let fields = line.split('\t').collect::<Vec<_>>();
                assert_eq!(fields.len(), 7, "Plan 09 boundary row: {line}");
                (fields[1], fields)
            })
            .collect::<BTreeMap<_, _>>();
        assert_eq!(plan09_boundaries.len(), 355);

        let data =
            Path::new(env!("CARGO_MANIFEST_DIR")).join("../../data/mtgjson/AtomicCards.json");
        let corpus = corpus::Corpus::load(&data).expect("production MTGJSON corpus loads");
        assert_eq!(
            corpus.source_fingerprint(),
            "e85359d7b8c578df13dff2fdf7c743a520a5b367d5ed25ab0a5f03cb8b3637dd",
        );
        let units = corpus
            .units()
            .iter()
            .map(|unit| (unit.id(), unit))
            .collect::<BTreeMap<_, _>>();
        let parser = parser_from_builtin_v2().expect("production English-v2 parser loads");
        let catalog_root = Path::new(env!("CARGO_MANIFEST_DIR")).join("../../data/gen/catalogs");
        let keyword_abilities = fs::read_to_string(catalog_root.join("keyword-abilities.txt"))
            .expect("frozen keyword-ability catalog loads");
        let ability_words = fs::read_to_string(catalog_root.join("ability-words.txt"))
            .expect("frozen ability-word catalog loads");
        let mut category_counts = BTreeMap::new();
        let mut selected_families = BTreeMap::new();
        let mut ids = BTreeSet::new();
        let mut previous_id = None;

        for fields in rows {
            let [
                category,
                pool_family,
                plan08_family,
                id,
                card_name_json,
                face_name_json,
                side_json,
                context_name_json,
                is_legendary,
                context_onset,
                oracle_text_json,
                extracted_effect_json,
                failure_start,
                failure_end,
                boundary,
            ] = fields.as_slice()
            else {
                panic!("candidate result row has fifteen columns: {fields:?}");
            };
            assert!(
                previous_id.is_none_or(|previous| previous < *id),
                "candidate results are strictly ID-sorted",
            );
            previous_id = Some(*id);
            assert!(ids.insert(*id), "candidate result ID is unique: {id}");
            let pool_row = pool.get(id).unwrap_or_else(|| panic!("pool contains {id}"));
            assert_eq!(pool_row[0], *pool_family, "{id}");
            assert_eq!(pool_row[2], decode(card_name_json), "{id}");
            assert_eq!(
                pool_row[3],
                if decode(face_name_json).is_empty() {
                    "<none>".to_owned()
                } else {
                    decode(face_name_json)
                },
                "{id}",
            );
            assert_eq!(pool_row[4], decode(context_name_json), "{id}");
            assert_eq!(pool_row[5], decode(oracle_text_json), "{id}");
            assert_eq!(pool_row[6], decode(extracted_effect_json), "{id}");

            let unit = units
                .get(id)
                .unwrap_or_else(|| panic!("corpus contains {id}"));
            assert_eq!(unit.card_name(), decode(card_name_json), "{id}");
            assert_eq!(
                unit.face_name().unwrap_or_default(),
                decode(face_name_json),
                "{id}",
            );
            assert_eq!(unit.side().unwrap_or_default(), decode(side_json), "{id}");
            assert_eq!(unit.context_name(), decode(context_name_json), "{id}");
            assert_eq!(unit.is_legendary().to_string(), *is_legendary, "{id}");
            assert_eq!(
                match unit.context_onset() {
                    macro_ron::v2::Onset::Vowel => "vowel",
                    macro_ron::v2::Onset::Consonant => "consonant",
                },
                *context_onset,
                "{id}",
            );
            assert_eq!(unit.text(), decode(oracle_text_json), "{id}");
            let context = deckmaste_english_v2::context::ParseContext::new(
                unit.context_name(),
                unit.is_legendary(),
                unit.context_onset(),
            )
            .unwrap_or_else(|| panic!("candidate context is valid: {id}"));
            let analysis = parser.analyze_oracle_text(unit.text(), &context);
            *category_counts.entry(*category).or_insert(0usize) += 1;

            match *category {
                "plan08-selected" => {
                    assert_eq!(analysis.outcome(), ParseAnalysisOutcome::Selected, "{id}");
                    assert_eq!(*boundary, "selected", "{id}");
                    assert!(failure_start.is_empty() && failure_end.is_empty(), "{id}");
                    assert!(analysis.selected().is_some(), "{id}");
                    let ownership = analysis.ownership().expect("selected ownership");
                    assert_eq!(ownership.rendered_text(), unit.text(), "{id}");
                    assert!(ownership.summary().covered(), "{id}");
                    assert!(ownership.failures().is_empty(), "{id}");
                    assert_eq!(ownership.summary().gap_spans(), 0, "{id}");
                    assert_eq!(ownership.summary().overlap_spans(), 0, "{id}");
                    assert_eq!(ownership.summary().synthetic_claims(), 0, "{id}");
                    assert_eq!(ownership.summary().provenance_plan_mismatches(), 0, "{id}");

                    let decision = analysis.decision().expect("selected decision");
                    assert_eq!(decision.resolution(), SelectionResolution::Unique, "{id}");
                    assert_eq!(decision.survivors().len(), 1, "{id}");
                    assert!(decision.exception_uses().is_empty(), "{id}");
                    let selected_ordinal = decision.selected().expect("selected ordinal");
                    let path = decision
                        .candidates()
                        .iter()
                        .find(|candidate| candidate.ordinal() == selected_ordinal)
                        .expect("selected candidate")
                        .construction_path();
                    let (expected_family, construction) = match *pool_family {
                        "activated-effect-selects" => ("ability.activated", "AbilityActivated"),
                        "trigger-effect-selects" => ("ability.triggered", "AbilityTriggered"),
                        other => panic!("unknown pool family {other}: {id}"),
                    };
                    assert_eq!(*plan08_family, expected_family, "{id}");
                    assert!(path.iter().any(|name| name == construction), "{id}");
                    *selected_families.entry(*plan08_family).or_insert(0usize) += 1;
                }
                "plan09-predicate" => {
                    assert!(plan08_family.is_empty(), "{id}");
                    let progress = plan09_boundaries
                        .get(id)
                        .unwrap_or_else(|| panic!("Plan 09 progress contains {id}"));
                    match progress[3] {
                        "selected" => {
                            assert_eq!(analysis.outcome(), ParseAnalysisOutcome::Selected, "{id}");
                            let ownership = analysis.ownership().expect("selected ownership");
                            assert_eq!(ownership.rendered_text(), unit.text(), "{id}");
                            assert!(ownership.summary().covered(), "{id}");
                            assert!(ownership.failures().is_empty(), "{id}");
                            let decision = analysis.decision().expect("selected decision");
                            assert!(
                                matches!(
                                    decision.resolution(),
                                    SelectionResolution::Unique | SelectionResolution::Specificity
                                ),
                                "{id}",
                            );
                            assert!(decision.exception_uses().is_empty(), "{id}");
                        }
                        movement @ ("same" | "later") => {
                            assert_eq!(
                                analysis.outcome(),
                                ParseAnalysisOutcome::ParseFailure,
                                "{id}",
                            );
                            let expected_span = TextSpan {
                                start: progress[4].parse().expect("Plan 09 failure start"),
                                end: progress[5].parse().expect("Plan 09 failure end"),
                            };
                            let ParseError::Failure { span, .. } = analysis
                                .into_parse_result()
                                .expect_err("unselected Plan 09 row fails")
                            else {
                                panic!("Plan 09 row has an ordinary parse failure: {id}");
                            };
                            assert_eq!(span, expected_span, "{id}");
                            assert_eq!(
                                &unit.text()[span.start..span.end],
                                decode(progress[6]),
                                "{id}",
                            );
                            let prior_start =
                                failure_start.parse::<usize>().expect("failure start");
                            if movement == "later" {
                                assert!(span.start > prior_start, "{id}");
                            } else {
                                assert_eq!(span.start, prior_start, "{id}");
                            }
                        }
                        other => panic!("unknown Plan 09 movement {other}: {id}"),
                    }
                }
                "plan10-attachment" => {
                    assert_eq!(
                        analysis.outcome(),
                        ParseAnalysisOutcome::ParseFailure,
                        "{id}"
                    );
                    assert!(plan08_family.is_empty(), "{id}");
                    let expected_span = TextSpan {
                        start: failure_start.parse().expect("failure start"),
                        end: failure_end.parse().expect("failure end"),
                    };
                    let ParseError::Failure { span, .. } = analysis
                        .into_parse_result()
                        .expect_err("deferred row fails")
                    else {
                        panic!("deferred candidate has ordinary parse failure: {id}");
                    };
                    assert_eq!(span, expected_span, "{id}");
                    let effect_start = unit
                        .text()
                        .rfind(&decode(extracted_effect_json))
                        .expect("exact effect occurs in its source row");
                    assert!(span.start < effect_start, "{id}");

                    let extracted = capitalize_initial(&decode(extracted_effect_json));
                    let extracted_analysis = parser.analyze_oracle_text(&extracted, &context);
                    assert_eq!(
                        extracted_analysis.outcome(),
                        ParseAnalysisOutcome::Selected,
                        "isolated effect selects for {id}",
                    );
                    assert_eq!(
                        extracted_analysis
                            .ownership()
                            .expect("isolated effect ownership")
                            .rendered_text(),
                        extracted,
                        "{id}",
                    );
                    let failure_surface = &unit.text()[span.start..span.end];
                    let rule = plan08_deferred_rule(pool_family, failure_surface);
                    assert_plan08_deferred_authority(
                        rule,
                        &parser,
                        &context,
                        id,
                        unit.text(),
                        span,
                        Plan08BoundaryCatalogs {
                            keyword_abilities: &keyword_abilities,
                            ability_words: &ability_words,
                        },
                    );
                    assert_eq!(*category, rule.authority.category(), "{id}");
                    assert_eq!(*boundary, "attachment", "{id}");
                    if let Some(witness) = PLAN08_ATTACHMENT_WITNESSES
                        .iter()
                        .copied()
                        .find(|witness| witness.id == *id)
                    {
                        assert_eq!(span, witness.failure_span, "{id}");
                        assert_plan08_attachment_witness(witness, &parser, &context);
                    }
                }
                "plan08-defect" => panic!("Plan 08 grammar defect blocks closure: {id}"),
                other => panic!("unknown candidate result category {other}: {id}"),
            }
        }

        assert_eq!(ids.len(), 427);
        assert_eq!(
            category_counts.into_iter().collect::<Vec<_>>(),
            EXPECTED_COUNTS,
        );
        assert_eq!(
            selected_families.into_iter().collect::<Vec<_>>(),
            EXPECTED_SELECTED_FAMILIES,
        );
    }

    #[test]
    fn plan08_production_baseline_is_retained_through_the_plan09_delta() {
        let data =
            Path::new(env!("CARGO_MANIFEST_DIR")).join("../../data/mtgjson/AtomicCards.json");
        let mut coverage_json = Vec::new();
        coverage::run(
            &CoverageArgs {
                corpus: CorpusArgs { data: data.clone() },
                lock: PathBuf::from("english-v2-coverage.lock"),
                json: true,
                check: false,
                bless: false,
            },
            &mut coverage_json,
        )
        .expect("production coverage baseline renders");
        let coverage: serde_json::Value =
            serde_json::from_slice(&coverage_json).expect("coverage baseline is JSON");
        assert_eq!(
            coverage["source_fingerprint"],
            "e85359d7b8c578df13dff2fdf7c743a520a5b367d5ed25ab0a5f03cb8b3637dd"
        );
        assert_eq!(coverage["summary"]["total_units"], 32_641);
        assert_eq!(coverage["summary"]["selected_units"], 3_413);
        assert_eq!(coverage["summary"]["covered_units"], 3_413);
        assert_eq!(coverage["summary"]["parse_failures"], 29_228);
        for counter in [
            "selected_uncovered_units",
            "unresolved_ties",
            "internal_failures",
            "roundtrip_mismatch_units",
            "ownership_failure_units",
        ] {
            assert_eq!(coverage["summary"][counter], 0, "{counter}");
        }

        let mut ambiguity_json = Vec::new();
        ambiguity::run(
            &AmbiguityArgs {
                corpus: CorpusArgs { data },
                json: true,
                require_resolved: false,
            },
            &mut ambiguity_json,
        )
        .expect("production ambiguity baseline renders");
        let ambiguity: serde_json::Value =
            serde_json::from_slice(&ambiguity_json).expect("ambiguity baseline is JSON");
        assert_eq!(
            ambiguity["summary"]["unique"].as_u64().unwrap()
                + ambiguity["summary"]["specificity_resolved"]
                    .as_u64()
                    .unwrap(),
            3_413,
        );
        for counter in [
            "exception_resolved",
            "unresolved_ties",
            "internal_failures",
            "exception_uses",
        ] {
            assert_eq!(ambiguity["summary"][counter], 0, "{counter}");
        }
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

    #[allow(
        clippy::too_many_lines,
        reason = "the complete production-origin oracle is deliberately explicit"
    )]
    fn expected_production_origins(item_key: &str) -> Option<&'static [&'static str]> {
        Some(match item_key {
            "type FiniteCondition"
            | "type FiniteConditionValue"
            | "function render_finite_condition"
            | "function walk_finite_condition"
            | "function walk_finite_condition_value" => &["construction finite_condition"],
            "type ExistentialCondition"
            | "type ExistentialConditionValue"
            | "function render_existential_condition"
            | "function walk_existential_condition"
            | "function walk_existential_condition_value" => {
                &["construction existential_condition"]
            }
            "type ExistentialClause"
            | "function render_existential_clause"
            | "function walk_existential_clause" => &[
                "construction singular_existential_clause",
                "construction plural_existential_clause",
            ],
            "type SingularExistentialClause"
            | "impl SingularExistentialClause"
            | "function walk_singular_existential_clause" => {
                &["construction singular_existential_clause"]
            }
            "type PluralExistentialClause"
            | "impl PluralExistentialClause"
            | "function walk_plural_existential_clause" => {
                &["construction plural_existential_clause"]
            }
            "type AmongPhrase"
            | "type AmongPhraseValue"
            | "function render_among_phrase"
            | "function walk_among_phrase"
            | "function walk_among_phrase_value" => &["construction among_phrase"],
            "type Ability"
            | "function __deckmaste_construction_internal_render_root_ability"
            | "function walk_ability" => &[
                "construction plain",
                "construction triggered",
                "construction activated",
            ],
            "type AbilityBody" | "function render_ability_body" | "function walk_ability_body" => {
                &["construction sentences", "construction plain_modal"]
            }
            "type ModalMode"
            | "type ModalModeValue"
            | "impl ModalModeValue"
            | "function render_modal_mode"
            | "function walk_modal_mode"
            | "function walk_modal_mode_value" => &["construction modal_mode"],
            "type TriggerPrefix"
            | "function render_trigger_prefix"
            | "function walk_trigger_prefix" => &["construction finite", "construction temporal"],
            "type AtPhrase"
            | "type AtPhraseValue"
            | "impl AtPhraseValue"
            | "function render_at_phrase"
            | "function walk_at_phrase"
            | "function walk_at_phrase_value" => &["construction at_phrase"],
            "type CostSymbol" | "function render_cost_symbol" | "function walk_cost_symbol" => &[
                "construction generic_cost_symbol",
                "construction fixed_cost_symbol",
                "construction monocolored_hybrid_symbol",
            ],
            "type LoyaltyValue"
            | "function render_loyalty_value"
            | "function walk_loyalty_value" => &[
                "construction positive_loyalty",
                "construction zero_loyalty",
                "construction negative_loyalty",
            ],
            "type Sentence"
            | "function __deckmaste_construction_internal_render_root_sentence"
            | "function walk_sentence" => &[
                "construction imperative",
                "construction declarative",
                "construction attached",
                "construction with_where",
            ],
            "type PredicateCoordination"
            | "function render_predicate_coordination"
            | "function agreement_matches_for_predicate_coordination"
            | "function walk_predicate_coordination" => &[
                "construction and_predicate_coordination",
                "construction or_predicate_coordination",
                "construction and_or_predicate_coordination",
            ],
            "type FiniteClause"
            | "function render_finite_clause"
            | "function walk_finite_clause" => &[
                "construction plain_finite_clause",
                "construction auxiliary_finite_clause",
            ],
            "type ClauseCoordination"
            | "function render_clause_coordination"
            | "function walk_clause_coordination" => &[
                "construction and_clause_coordination",
                "construction or_clause_coordination",
                "construction and_or_clause_coordination",
            ],
            "type WhereClauseCategory"
            | "type WhereClause"
            | "function render_where_clause_category"
            | "function walk_where_clause_category"
            | "function walk_where_clause" => &["construction where"],
            "type Subject"
            | "function render_subject"
            | "function agreement_for_subject"
            | "function agreement_matches_for_subject"
            | "function walk_subject" => &[
                "construction subject_nominal",
                "construction subject_pronoun",
            ],
            "type Object"
            | "function render_object"
            | "function agreement_matches_for_object"
            | "function walk_object" => &[
                "construction object_nominal",
                "construction object_pronoun",
                "construction reflexive_object",
            ],
            "type SingularHead"
            | "function render_singular_head"
            | "function agreement_for_singular_head"
            | "function number_for_singular_head"
            | "function onset_for_singular_head"
            | "function agreement_matches_for_singular_head"
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
            | "function agreement_matches_for_plural_head"
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
            | "function agreement_matches_for_nominal_modifier"
            | "function walk_nominal_modifier" => &[
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
            ],
            "type NegativeNominalModifier"
            | "type NegativeModifierMember"
            | "impl NegativeModifierMember"
            | "function render_negative_nominal_modifier"
            | "function walk_negative_nominal_modifier"
            | "function walk_negative_modifier_member" => {
                &["construction negative_modifier_member"]
            }
            "type CoordinatedNominalModifier"
            | "function render_coordinated_nominal_modifier"
            | "function onset_for_coordinated_nominal_modifier"
            | "function walk_coordinated_nominal_modifier" => &[
                "construction non_target_common_noun_modifier",
                "construction coordinated_modifier_member",
            ],
            "type SingularNominal"
            | "function render_singular_nominal"
            | "function agreement_for_singular_nominal"
            | "function number_for_singular_nominal"
            | "function onset_for_singular_nominal"
            | "function agreement_matches_for_singular_nominal"
            | "function walk_singular_nominal" => &[
                "construction bare_singular_nominal",
                "construction modified_singular_nominal",
                "construction negative_modified_singular_nominal",
                "construction compound_modified_singular_nominal",
            ],
            "type PluralNominal"
            | "function render_plural_nominal"
            | "function agreement_for_plural_nominal"
            | "function number_for_plural_nominal"
            | "function onset_for_plural_nominal"
            | "function agreement_matches_for_plural_nominal"
            | "function walk_plural_nominal" => &[
                "construction bare_plural_nominal",
                "construction modified_plural_nominal",
                "construction compound_modified_plural_nominal",
                "construction negative_modified_plural_nominal",
            ],
            "type SingularCoordinationMember"
            | "function render_singular_coordination_member"
            | "function onset_for_singular_coordination_member"
            | "function agreement_matches_for_singular_coordination_member"
            | "function walk_singular_coordination_member" => &[
                "construction bare_singular_coordination_member",
                "construction modified_singular_coordination_member",
                "construction negative_modified_singular_coordination_member",
            ],
            "type PluralCoordinationMember"
            | "function render_plural_coordination_member"
            | "function agreement_matches_for_plural_coordination_member"
            | "function walk_plural_coordination_member" => &[
                "construction bare_plural_coordination_member",
                "construction modified_plural_coordination_member",
                "construction negative_modified_plural_coordination_member",
            ],
            "type SingularNominalCoordination"
            | "function render_singular_nominal_coordination"
            | "function walk_singular_nominal_coordination" => &[
                "construction singular_and_nominal_coordination",
                "construction singular_or_nominal_coordination",
                "construction singular_and_or_nominal_coordination",
            ],
            "type DeterminerScopedNominalCoordination"
            | "function render_determiner_scoped_nominal_coordination"
            | "function onset_for_determiner_scoped_nominal_coordination"
            | "function walk_determiner_scoped_nominal_coordination" => &[
                "construction determiner_scoped_and_nominal_pair",
                "construction determiner_scoped_and_nominal_series",
                "construction determiner_scoped_or_nominal_pair",
                "construction determiner_scoped_or_nominal_series",
                "construction determiner_scoped_and_or_nominal_pair",
                "construction determiner_scoped_and_or_nominal_series",
            ],
            "type PluralNominalCoordination"
            | "function render_plural_nominal_coordination"
            | "function walk_plural_nominal_coordination" => &[
                "construction plural_and_nominal_coordination",
                "construction plural_or_nominal_coordination",
                "construction plural_and_or_nominal_coordination",
            ],
            "type SingularSelector"
            | "function render_singular_selector"
            | "function agreement_for_singular_selector"
            | "function number_for_singular_selector"
            | "function agreement_matches_for_singular_selector"
            | "function walk_singular_selector" => &[
                "construction unmarked_singular_selector",
                "construction target_singular_selector",
                "construction target_singular_coordination_selector",
                "construction other_singular_selector",
                "construction other_target_singular_selector",
            ],
            "type PluralSelector"
            | "function render_plural_selector"
            | "function agreement_for_plural_selector"
            | "function number_for_plural_selector"
            | "function onset_for_plural_selector"
            | "function agreement_matches_for_plural_selector"
            | "function walk_plural_selector" => &[
                "construction unmarked_plural_selector",
                "construction unmarked_plural_coordination_selector",
                "construction target_plural_selector",
                "construction target_plural_coordination_selector",
                "construction other_plural_selector",
                "construction other_target_plural_selector",
            ],
            "type UnqualifiedReference"
            | "function render_unqualified_reference"
            | "function agreement_for_unqualified_reference"
            | "function number_for_unqualified_reference"
            | "function onset_for_unqualified_reference"
            | "function agreement_matches_for_unqualified_reference"
            | "function walk_unqualified_reference" => &[
                "construction indefinite_reference",
                "construction indefinite_coordination_reference",
                "construction named_card_reference",
                "construction ordinary_singular_reference",
                "construction ordinary_plural_reference",
                "construction definite_singular_reference",
                "construction definite_plural_reference",
                "construction any_target_reference",
                "construction another_reference",
                "construction another_coordination_reference",
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
                "construction demonstrative_possessive_reference",
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
            ],
            "type CountReference"
            | "type ThatMany"
            | "function __deckmaste_construction_internal_render_root_count_reference"
            | "function agreement_for_count_reference"
            | "function number_for_count_reference"
            | "function onset_for_count_reference"
            | "function agreement_matches_for_count_reference"
            | "function walk_count_reference"
            | "function walk_that_many" => &["construction that_many"],
            "type DeterminerPhrase"
            | "function render_determiner_phrase"
            | "function agreement_for_determiner_phrase"
            | "function number_for_determiner_phrase"
            | "function onset_for_determiner_phrase"
            | "function agreement_matches_for_determiner_phrase"
            | "function walk_determiner_phrase" => &[
                "construction target_determiner_phrase",
                "construction target_coordination_determiner_phrase",
                "construction indefinite_determiner_phrase",
                "construction this_determiner_phrase",
                "construction that_determiner_phrase",
                "construction another_determiner_phrase",
            ],
            "type FullNounPhraseCoordination"
            | "function render_full_noun_phrase_coordination"
            | "function agreement_for_full_noun_phrase_coordination"
            | "function number_for_full_noun_phrase_coordination"
            | "function agreement_matches_for_full_noun_phrase_coordination"
            | "function walk_full_noun_phrase_coordination" => &[
                "construction full_and_noun_phrase_coordination",
                "construction full_or_noun_phrase_coordination",
                "construction full_and_or_noun_phrase_coordination",
            ],
            "type MannerReference"
            | "type ThisWay"
            | "function walk_manner_reference"
            | "function walk_this_way" => &["construction this_way"],
            "type ScalarReference"
            | "type ThatMuch"
            | "function __deckmaste_construction_internal_render_root_scalar_reference"
            | "function agreement_matches_for_scalar_reference"
            | "function walk_scalar_reference"
            | "function walk_that_much" => &["construction that_much"],
            "type ControllerOwnerQualification"
            | "function render_controller_owner_qualification"
            | "function walk_controller_owner_qualification" => &[
                "construction you_control",
                "construction opponent_controls",
                "construction demonstrative_controls",
                "construction you_own",
            ],
            "type SingularController"
            | "type OpponentController"
            | "impl OpponentController"
            | "function render_singular_controller"
            | "function walk_singular_controller"
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
            | "type ScalarQualificationValue"
            | "function render_scalar_qualification"
            | "function walk_scalar_qualification"
            | "function walk_scalar_qualification_value" => &["construction scalar_qualification"],
            "type ControllerStage"
            | "function render_controller_stage"
            | "function agreement_for_controller_stage"
            | "function number_for_controller_stage"
            | "function onset_for_controller_stage"
            | "function agreement_matches_for_controller_stage"
            | "function walk_controller_stage" => &[
                "construction unqualified_controller_stage",
                "construction controller_qualified_reference",
                "construction other_than_qualified_reference",
            ],
            "type ZoneStage"
            | "function render_zone_stage"
            | "function agreement_for_zone_stage"
            | "function number_for_zone_stage"
            | "function onset_for_zone_stage"
            | "function agreement_matches_for_zone_stage"
            | "function walk_zone_stage" => &[
                "construction unqualified_zone_stage",
                "construction zone_qualified_reference",
            ],
            "type NumericStage"
            | "function render_numeric_stage"
            | "function agreement_for_numeric_stage"
            | "function number_for_numeric_stage"
            | "function onset_for_numeric_stage"
            | "function agreement_matches_for_numeric_stage"
            | "function walk_numeric_stage" => &[
                "construction unqualified_numeric_stage",
                "construction scalar_qualified_reference",
            ],
            "type NounPhrase"
            | "type QualifiedNounPhrase"
            | "function render_noun_phrase"
            | "function agreement_for_noun_phrase"
            | "function number_for_noun_phrase"
            | "function onset_for_noun_phrase"
            | "function agreement_matches_for_noun_phrase"
            | "function walk_noun_phrase"
            | "function walk_qualified_noun_phrase" => &["construction qualified_noun_phrase"],
            "type PossessiveOwner"
            | "function render_possessive_owner"
            | "function number_for_possessive_owner"
            | "function possessive_ending_for_possessive_owner"
            | "function walk_possessive_owner" => &[
                "construction possessive_self_reference",
                "construction possessive_plural_noun",
            ],
            "type Possessive"
            | "type PossessiveValue"
            | "function walk_possessive"
            | "function walk_possessive_value" => &["construction possessive"],
            "type VerbPhrase"
            | "function render_verb_phrase"
            | "function agreement_matches_for_verb_phrase"
            | "function walk_verb_phrase" => &[
                "construction intransitive_predicate",
                "construction transitive_predicate",
                "construction numerative_predicate",
                "construction deal_damage",
                "construction gain_life",
            ],
            "type Amount" | "function render_amount" | "function walk_amount" => &[
                "construction number",
                "construction variable",
                "construction scalar_reference_amount",
            ],
            "type CardinalQuantity"
            | "type CardinalQuantityValue"
            | "function __deckmaste_construction_internal_render_root_cardinal_quantity"
            | "function cardinality_for_cardinal_quantity"
            | "function walk_cardinal_quantity"
            | "function walk_cardinal_quantity_value" => &["construction cardinal"],
            "type ActivationCostComponent"
            | "function render_activation_cost_component"
            | "function walk_activation_cost_component" => {
                &["abstract sum ActivationCostComponent"]
            }
            "type Predicate"
            | "function render_predicate"
            | "function agreement_matches_for_predicate"
            | "function walk_predicate" => &["abstract sum Predicate"],
            "type Clause" | "function render_clause" | "function walk_clause" => {
                &["abstract sum Clause"]
            }
            "type ClauseAttachment"
            | "function render_clause_attachment"
            | "function walk_clause_attachment" => &["abstract sum ClauseAttachment"],
            "type ConditionClause"
            | "function render_condition_clause"
            | "function walk_condition_clause" => &["abstract sum ConditionClause"],
            "type DocumentBlock"
            | "function render_document_block"
            | "function walk_document_block" => &["abstract sum DocumentBlock"],
            "type OracleText"
            | "function render_oracle_text_blocks_sequence"
            | "function render_oracle_text"
            | "function walk_oracle_text_blocks_sequence"
            | "function walk_oracle_text" => &["abstract product OracleText"],
            "type Plain" | "function walk_plain" => &["construction plain"],
            "type Sentences" | "impl Sentences" | "function walk_sentences" => {
                &["construction sentences"]
            }
            "type PlainModal" | "impl PlainModal" | "function walk_plain_modal" => {
                &["construction plain_modal"]
            }
            "type Finite" | "function walk_finite" => &["construction finite"],
            "type Temporal" | "function walk_temporal" => &["construction temporal"],
            "type Triggered" | "function walk_triggered" => &["construction triggered"],
            "type GenericCostSymbol" | "function walk_generic_cost_symbol" => {
                &["construction generic_cost_symbol"]
            }
            "type FixedSymbol" | "function walk_fixed_symbol" => {
                &["construction fixed_cost_symbol"]
            }
            "type MonocoloredHybridSymbol" | "function walk_monocolored_hybrid_symbol" => {
                &["construction monocolored_hybrid_symbol"]
            }
            "type SymbolRun"
            | "impl SymbolRun"
            | "function render_symbol_run"
            | "function walk_symbol_run" => &["construction symbol_run"],
            "type PositiveLoyalty" | "function walk_positive_loyalty" => {
                &["construction positive_loyalty"]
            }
            "type ZeroLoyalty" | "function walk_zero_loyalty" => &["construction zero_loyalty"],
            "type NegativeLoyalty" | "function walk_negative_loyalty" => {
                &["construction negative_loyalty"]
            }
            "type Loyalty" | "function render_loyalty" | "function walk_loyalty" => {
                &["construction loyalty"]
            }
            "type CostClause"
            | "impl CostClause"
            | "function render_cost_clause"
            | "function walk_cost_clause" => &["construction cost_clause"],
            "type Activated" | "impl Activated" | "function walk_activated" => {
                &["construction activated"]
            }
            "type Imperative" | "impl Imperative" | "function walk_imperative" => {
                &["construction imperative"]
            }
            "type Declarative" | "function walk_declarative" => &["construction declarative"],
            "type Attached" | "function walk_attached" => &["construction attached"],
            "type PreposedIf" | "function render_preposed_if" | "function walk_preposed_if" => {
                &["construction preposed_if"]
            }
            "type PreposedIfPredicate"
            | "impl PreposedIfPredicate"
            | "function render_preposed_if_predicate"
            | "function walk_preposed_if_predicate" => &["construction preposed_if_predicate"],
            "type PostposedIf" | "function render_postposed_if" | "function walk_postposed_if" => {
                &["construction postposed_if"]
            }
            "type PostposedIfPredicate"
            | "impl PostposedIfPredicate"
            | "function render_postposed_if_predicate"
            | "function walk_postposed_if_predicate" => &["construction postposed_if_predicate"],
            "type PostposedUnless"
            | "function render_postposed_unless"
            | "function walk_postposed_unless" => &["construction postposed_unless"],
            "type PostposedUnlessPredicate"
            | "impl PostposedUnlessPredicate"
            | "function render_postposed_unless_predicate"
            | "function walk_postposed_unless_predicate" => {
                &["construction postposed_unless_predicate"]
            }
            "type PreposedAsLongAs"
            | "function render_preposed_as_long_as"
            | "function walk_preposed_as_long_as" => &["construction preposed_as_long_as"],
            "type PreposedAsLongAsPredicate"
            | "impl PreposedAsLongAsPredicate"
            | "function render_preposed_as_long_as_predicate"
            | "function walk_preposed_as_long_as_predicate" => {
                &["construction preposed_as_long_as_predicate"]
            }
            "type PreposedWhile"
            | "function render_preposed_while"
            | "function walk_preposed_while" => &["construction preposed_while"],
            "type PreposedWhilePredicate"
            | "impl PreposedWhilePredicate"
            | "function render_preposed_while_predicate"
            | "function walk_preposed_while_predicate" => {
                &["construction preposed_while_predicate"]
            }
            "type PreposedDuring"
            | "function render_preposed_during"
            | "function walk_preposed_during" => &["construction preposed_during"],
            "type PreposedDuringPredicate"
            | "impl PreposedDuringPredicate"
            | "function render_preposed_during_predicate"
            | "function walk_preposed_during_predicate" => {
                &["construction preposed_during_predicate"]
            }
            "type PreposedUntil"
            | "function render_preposed_until"
            | "function walk_preposed_until" => &["construction preposed_until"],
            "type PreposedUntilPredicate"
            | "impl PreposedUntilPredicate"
            | "function render_preposed_until_predicate"
            | "function walk_preposed_until_predicate" => {
                &["construction preposed_until_predicate"]
            }
            "type ThenSequence"
            | "impl ThenSequence"
            | "function render_then_sequence"
            | "function walk_then_sequence" => &["construction then_sequence"],
            "type ThenPredicateSequence"
            | "impl ThenPredicateSequence"
            | "function render_then_predicate_sequence"
            | "function walk_then_predicate_sequence" => &["construction then_predicate_sequence"],
            "type ReflexiveSubordinate"
            | "function render_reflexive_subordinate"
            | "function walk_reflexive_subordinate" => &["construction reflexive_subordinate"],
            "type ReflexivePredicateSubordinate"
            | "impl ReflexivePredicateSubordinate"
            | "function render_reflexive_predicate_subordinate"
            | "function walk_reflexive_predicate_subordinate" => {
                &["construction reflexive_predicate_subordinate"]
            }
            "type WithWhere" | "function walk_with_where" => &["construction with_where"],
            "type AndPredicateCoordination"
            | "impl AndPredicateCoordination"
            | "function walk_and_predicate_coordination" => {
                &["construction and_predicate_coordination"]
            }
            "type OrPredicateCoordination"
            | "impl OrPredicateCoordination"
            | "function walk_or_predicate_coordination" => {
                &["construction or_predicate_coordination"]
            }
            "type AndOrPredicateCoordination"
            | "impl AndOrPredicateCoordination"
            | "function walk_and_or_predicate_coordination" => {
                &["construction and_or_predicate_coordination"]
            }
            "type PlainFiniteClause"
            | "impl PlainFiniteClause"
            | "function walk_plain_finite_clause" => &["construction plain_finite_clause"],
            "type AuxiliaryFiniteClause"
            | "impl AuxiliaryFiniteClause"
            | "function walk_auxiliary_finite_clause" => &["construction auxiliary_finite_clause"],
            "type AndClauseCoordination"
            | "impl AndClauseCoordination"
            | "function walk_and_clause_coordination" => &["construction and_clause_coordination"],
            "type OrClauseCoordination"
            | "impl OrClauseCoordination"
            | "function walk_or_clause_coordination" => &["construction or_clause_coordination"],
            "type AndOrClauseCoordination"
            | "impl AndOrClauseCoordination"
            | "function walk_and_or_clause_coordination" => {
                &["construction and_or_clause_coordination"]
            }
            "type NominalSubject" | "function walk_nominal_subject" => {
                &["construction subject_nominal"]
            }
            "type PersonalSubject"
            | "function agreement_for_subject_pronoun"
            | "function walk_personal_subject" => &["construction subject_pronoun"],
            "type NominalObject" | "function walk_nominal_object" => {
                &["construction object_nominal"]
            }
            "type PersonalObject"
            | "function agreement_for_object_pronoun"
            | "function walk_personal_object" => &["construction object_pronoun"],
            "type ReflexiveObject"
            | "function agreement_for_reflexive_pronoun"
            | "function number_for_reflexive_pronoun"
            | "function walk_reflexive_object" => &["construction reflexive_object"],
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
            "type NonTargetCommonNounModifier"
            | "function walk_non_target_common_noun_modifier" => {
                &["construction non_target_common_noun_modifier"]
            }
            "type CoordinatedModifierMember"
            | "impl CoordinatedModifierMember"
            | "function walk_coordinated_modifier_member" => {
                &["construction coordinated_modifier_member"]
            }
            "type CompoundNominalModifier"
            | "type CompoundModifierMember"
            | "impl CompoundModifierMember"
            | "function render_compound_nominal_modifier"
            | "function onset_for_compound_nominal_modifier"
            | "function walk_compound_nominal_modifier"
            | "function walk_compound_modifier_member" => {
                &["construction compound_modifier_member"]
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
            "type CompoundModifiedSingularNominal"
            | "impl CompoundModifiedSingularNominal"
            | "function walk_compound_modified_singular_nominal" => {
                &["construction compound_modified_singular_nominal"]
            }
            "type CompoundModifiedPluralNominal"
            | "impl CompoundModifiedPluralNominal"
            | "function walk_compound_modified_plural_nominal" => {
                &["construction compound_modified_plural_nominal"]
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
            "type DeterminerScopedAndNominalPair"
            | "function walk_determiner_scoped_and_nominal_pair" => {
                &["construction determiner_scoped_and_nominal_pair"]
            }
            "type DeterminerScopedAndNominalSeries"
            | "impl DeterminerScopedAndNominalSeries"
            | "function walk_determiner_scoped_and_nominal_series" => {
                &["construction determiner_scoped_and_nominal_series"]
            }
            "type DeterminerScopedOrNominalPair"
            | "function walk_determiner_scoped_or_nominal_pair" => {
                &["construction determiner_scoped_or_nominal_pair"]
            }
            "type DeterminerScopedOrNominalSeries"
            | "impl DeterminerScopedOrNominalSeries"
            | "function walk_determiner_scoped_or_nominal_series" => {
                &["construction determiner_scoped_or_nominal_series"]
            }
            "type DeterminerScopedAndOrNominalPair"
            | "function walk_determiner_scoped_and_or_nominal_pair" => {
                &["construction determiner_scoped_and_or_nominal_pair"]
            }
            "type DeterminerScopedAndOrNominalSeries"
            | "impl DeterminerScopedAndOrNominalSeries"
            | "function walk_determiner_scoped_and_or_nominal_series" => {
                &["construction determiner_scoped_and_or_nominal_series"]
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
            "type IndefiniteCoordinationReference"
            | "function walk_indefinite_coordination_reference" => {
                &["construction indefinite_coordination_reference"]
            }
            "type NamedCardReference" | "function walk_named_card_reference" => {
                &["construction named_card_reference"]
            }
            "type OrdinarySingularReference"
            | "impl OrdinarySingularReference"
            | "function walk_ordinary_singular_reference" => {
                &["construction ordinary_singular_reference"]
            }
            "type OrdinaryPluralReference"
            | "impl OrdinaryPluralReference"
            | "function walk_ordinary_plural_reference" => {
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
            "type AnotherCoordinationReference"
            | "function walk_another_coordination_reference" => {
                &["construction another_coordination_reference"]
            }
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
            "type DemonstrativePossessiveReference"
            | "function walk_demonstrative_possessive_reference" => {
                &["construction demonstrative_possessive_reference"]
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
            | "function agreement_for_possessive_absolute_pronoun"
            | "function number_for_possessive_absolute_pronoun"
            | "function walk_possessive_absolute_reference" => {
                &["construction possessive_absolute_reference"]
            }
            "type TargetDeterminerPhrase" | "function walk_target_determiner_phrase" => {
                &["construction target_determiner_phrase"]
            }
            "type TargetCoordinationDeterminerPhrase"
            | "function walk_target_coordination_determiner_phrase" => {
                &["construction target_coordination_determiner_phrase"]
            }
            "type IndefiniteDeterminerPhrase" | "function walk_indefinite_determiner_phrase" => {
                &["construction indefinite_determiner_phrase"]
            }
            "type ThisDeterminerPhrase" | "function walk_this_determiner_phrase" => {
                &["construction this_determiner_phrase"]
            }
            "type ThatDeterminerPhrase" | "function walk_that_determiner_phrase" => {
                &["construction that_determiner_phrase"]
            }
            "type AnotherDeterminerPhrase"
            | "impl AnotherDeterminerPhrase"
            | "function walk_another_determiner_phrase" => {
                &["construction another_determiner_phrase"]
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
            "type DemonstrativeControls"
            | "impl DemonstrativeControls"
            | "function walk_demonstrative_controls" => &["construction demonstrative_controls"],
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
            "type UnqualifiedControllerStage" | "function walk_unqualified_controller_stage" => {
                &["construction unqualified_controller_stage"]
            }
            "type ControllerQualifiedReference"
            | "function walk_controller_qualified_reference" => {
                &["construction controller_qualified_reference"]
            }
            "type OtherThanQualifiedReference" | "function walk_other_than_qualified_reference" => {
                &["construction other_than_qualified_reference"]
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
            "type PossessiveSelfReference"
            | "impl PossessiveSelfReference"
            | "function walk_possessive_self_reference" => {
                &["construction possessive_self_reference"]
            }
            "type PossessiveNoun" | "function walk_possessive_noun" => {
                &["construction possessive_plural_noun"]
            }
            "type IntransitivePredicate" | "function walk_intransitive_predicate" => {
                &["construction intransitive_predicate"]
            }
            "type TransitivePredicate" | "function walk_transitive_predicate" => {
                &["construction transitive_predicate"]
            }
            "type NumerativePredicate" | "function walk_numerative_predicate" => {
                &["construction numerative_predicate"]
            }
            "type DealDamage" | "function walk_deal_damage" => &["construction deal_damage"],
            "type GainLife" | "function walk_gain_life" => &["construction gain_life"],
            "type NumberAmount" | "function walk_number_amount" => &["construction number"],
            "type VariableAmount" | "function walk_variable_amount" => &["construction variable"],
            "type ScalarReferenceAmount" | "function walk_scalar_reference_amount" => {
                &["construction scalar_reference_amount"]
            }
            "type Auxiliary" | "function render_auxiliary" | "function walk_auxiliary" => {
                &["vocab Auxiliary"]
            }
            "type FiniteCopula"
            | "function render_finite_copula"
            | "function walk_finite_copula" => &["vocab FiniteCopula"],
            "type BareCopula" | "function render_bare_copula" | "function walk_bare_copula" => {
                &["vocab BareCopula"]
            }
            "type PredicativeAdjective"
            | "function render_predicative_adjective"
            | "function walk_predicative_adjective" => &["vocab PredicativeAdjective"],
            "type DamageKind" | "function render_damage_kind" | "function walk_damage_kind" => {
                &["vocab DamageKind"]
            }
            "type FaceOrientation"
            | "function render_face_orientation"
            | "function walk_face_orientation" => &["vocab FaceOrientation"],
            "type AtBoundary" | "function render_at_boundary" | "function walk_at_boundary" => {
                &["vocab AtBoundary"]
            }
            "type TriggerMarker"
            | "function render_trigger_marker"
            | "function walk_trigger_marker" => &["vocab TriggerMarker"],
            "type CounterName" | "function render_counter_name" | "function walk_counter_name" => {
                &["vocab CounterName"]
            }
            "type DieShape" | "function render_die_shape" | "function walk_die_shape" => {
                &["vocab DieShape"]
            }
            "type LibraryPosition"
            | "function render_library_position"
            | "function walk_library_position" => &["vocab LibraryPosition"],
            "type TurnOwnerPostmodifier"
            | "function render_turn_owner_postmodifier"
            | "function walk_turn_owner_postmodifier" => &["vocab TurnOwnerPostmodifier"],
            "type TurnPart" | "function render_turn_part" | "function walk_turn_part" => {
                &["vocab TurnPart"]
            }
            "type TurnSpecifier"
            | "function render_turn_specifier"
            | "function walk_turn_specifier" => &["vocab TurnSpecifier"],
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
            "type SingularDemonstrative"
            | "function render_singular_demonstrative"
            | "function walk_singular_demonstrative" => &["vocab SingularDemonstrative"],
            "type ControllerNoun"
            | "function render_controller_noun"
            | "function walk_controller_noun" => &["vocab ControllerNoun"],
            "type FixedCostSymbol"
            | "function render_fixed_cost_symbol"
            | "function walk_fixed_cost_symbol" => &["vocab FixedCostSymbol"],
            "type ModalChooser"
            | "function render_modal_chooser"
            | "function walk_modal_chooser" => &["vocab ModalChooser"],
            "type ModalChoiceBounds"
            | "function render_modal_choice_bounds"
            | "function walk_modal_choice_bounds" => &["vocab ModalChoiceBounds"],
            "type MonocoloredHybridColor"
            | "function render_monocolored_hybrid_color"
            | "function walk_monocolored_hybrid_color" => &["vocab MonocoloredHybridColor"],
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
            "type DamageParticipleLexeme"
            | "function surface_for_damage_participle_lexeme"
            | "function walk_damage_participle_lexeme" => &["lexeme DamageParticipleLexeme"],
            "type MovementParticipleLexeme"
            | "function surface_for_movement_participle_lexeme"
            | "function walk_movement_participle_lexeme" => &["lexeme MovementParticipleLexeme"],
            "type OrientationParticipleLexeme"
            | "function surface_for_orientation_participle_lexeme"
            | "function walk_orientation_participle_lexeme" => {
                &["lexeme OrientationParticipleLexeme"]
            }
            "type CoreIntransitiveVerb"
            | "function surface_for_core_intransitive_verb"
            | "function walk_core_intransitive_verb" => &["lexeme CoreIntransitiveVerb"],
            "type CoreTransitiveVerb"
            | "function surface_for_core_transitive_verb"
            | "function walk_core_transitive_verb" => &["lexeme CoreTransitiveVerb"],
            "type CoreNumerativeVerb"
            | "function surface_for_core_numerative_verb"
            | "function walk_core_numerative_verb" => &["lexeme CoreNumerativeVerb"],
            "type DeclarationIntransitiveVerb"
            | "impl DeclarationIntransitiveVerb"
            | "type IntransitiveVerb"
            | "function walk_declaration_intransitive_verb"
            | "function walk_intransitive_verb" => &["codec IntransitiveVerb"],
            "type DeclarationTransitiveVerb"
            | "impl DeclarationTransitiveVerb"
            | "type TransitiveVerb"
            | "function walk_declaration_transitive_verb"
            | "function walk_transitive_verb" => &["codec TransitiveVerb"],
            "type DeclarationNumerativeVerb"
            | "impl DeclarationNumerativeVerb"
            | "type NumerativeVerb"
            | "function walk_declaration_numerative_verb"
            | "function walk_numerative_verb" => &["codec NumerativeVerb"],
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
            "type SelfReferenceSpelling"
            | "impl SelfReferenceSpelling"
            | "function walk_self_reference_spelling" => &["identity SelfReferenceSpelling"],
            "type CardName"
            | "impl CardName"
            | "type CatalogProvider"
            | "impl CatalogProvider"
            | "constant REQUIRED_CATALOG_PROVIDERS"
            | "function walk_card_name" => &["identity CardName"],
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
            "type LoyaltyMagnitude"
            | "function format_loyalty_magnitude"
            | "function parse_loyalty_magnitude"
            | "function render_loyalty_magnitude"
            | "function walk_loyalty_magnitude" => &["codec LoyaltyMagnitude"],
            "type ReflexiveSubordinateKind"
            | "function render_reflexive_subordinate_kind"
            | "function walk_reflexive_subordinate_kind" => &["vocab ReflexiveSubordinateKind"],
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
            | "type VerbFrameAtom"
            | "type VerbFrameKey"
            | "impl VerbFrameKey"
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
            | "type SequenceOwner"
            | "type FixedSurfaceAtom"
            | "function sequence_separator"
            | "function sequence_terminator" => &[
                "vocab Auxiliary",
                "vocab AtBoundary",
                "vocab TriggerMarker",
                "vocab TurnOwnerPostmodifier",
                "vocab TurnPart",
                "vocab TurnSpecifier",
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
                "vocab SingularDemonstrative",
                "vocab ControllerNoun",
                "vocab FixedCostSymbol",
                "vocab ModalChooser",
                "vocab ModalChoiceBounds",
                "vocab MonocoloredHybridColor",
                "vocab ScalarCharacteristic",
                "vocab Zone",
                "vocab NonCommonNoun",
                "vocab NonTargetCommonModifier",
                "vocab Supertype",
                "morphology EnglishVerb",
                "morphology EnglishNoun",
                "lexeme CommonNoun",
                "lexeme VerbLexeme",
                "lexeme CoreIntransitiveVerb",
                "lexeme CoreTransitiveVerb",
                "lexeme CoreNumerativeVerb",
                "codec IntransitiveVerb",
                "codec TransitiveVerb",
                "codec NumerativeVerb",
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
                "codec LoyaltyMagnitude",
                "abstract sum ActivationCostComponent",
                "abstract sum Predicate",
                "abstract sum Clause",
                "abstract sum ClauseAttachment",
                "abstract sum ConditionClause",
                "construction finite_condition",
                "construction existential_condition",
                "construction singular_existential_clause",
                "construction plural_existential_clause",
                "construction among_phrase",
                "construction plain",
                "construction sentences",
                "construction modal_mode",
                "construction plain_modal",
                "construction finite",
                "construction temporal",
                "construction at_phrase",
                "construction triggered",
                "construction generic_cost_symbol",
                "construction fixed_cost_symbol",
                "construction monocolored_hybrid_symbol",
                "construction symbol_run",
                "construction positive_loyalty",
                "construction zero_loyalty",
                "construction negative_loyalty",
                "construction loyalty",
                "construction cost_clause",
                "construction activated",
                "construction imperative",
                "construction declarative",
                "construction attached",
                "construction preposed_if",
                "construction preposed_if_predicate",
                "construction postposed_if",
                "construction postposed_if_predicate",
                "construction postposed_unless",
                "construction postposed_unless_predicate",
                "construction preposed_as_long_as",
                "construction preposed_as_long_as_predicate",
                "construction preposed_while",
                "construction preposed_while_predicate",
                "construction preposed_during",
                "construction preposed_during_predicate",
                "construction preposed_until",
                "construction preposed_until_predicate",
                "construction then_sequence",
                "construction then_predicate_sequence",
                "construction reflexive_subordinate",
                "construction reflexive_predicate_subordinate",
                "construction with_where",
                "construction and_predicate_coordination",
                "construction or_predicate_coordination",
                "construction and_or_predicate_coordination",
                "construction plain_finite_clause",
                "construction auxiliary_finite_clause",
                "construction and_clause_coordination",
                "construction or_clause_coordination",
                "construction and_or_clause_coordination",
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
                "construction compound_modifier_member",
                "construction bare_singular_nominal",
                "construction modified_singular_nominal",
                "construction negative_modified_singular_nominal",
                "construction bare_plural_nominal",
                "construction modified_plural_nominal",
                "construction compound_modified_singular_nominal",
                "construction compound_modified_plural_nominal",
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
                "construction determiner_scoped_and_nominal_pair",
                "construction determiner_scoped_and_nominal_series",
                "construction determiner_scoped_or_nominal_pair",
                "construction determiner_scoped_or_nominal_series",
                "construction determiner_scoped_and_or_nominal_pair",
                "construction determiner_scoped_and_or_nominal_series",
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
                "construction indefinite_coordination_reference",
                "construction named_card_reference",
                "construction ordinary_singular_reference",
                "construction ordinary_plural_reference",
                "construction definite_singular_reference",
                "construction definite_plural_reference",
                "construction any_target_reference",
                "construction another_reference",
                "construction another_coordination_reference",
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
                "construction demonstrative_possessive_reference",
                "construction those_reference",
                "construction designated_singular_reference",
                "construction designated_plural_reference",
                "construction chosen_quality_reference",
                "construction possessed_singular_reference",
                "construction possessed_plural_reference",
                "construction possessive_absolute_reference",
                "construction target_determiner_phrase",
                "construction target_coordination_determiner_phrase",
                "construction indefinite_determiner_phrase",
                "construction this_determiner_phrase",
                "construction that_determiner_phrase",
                "construction another_determiner_phrase",
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
                "construction demonstrative_controls",
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
                "construction other_than_qualified_reference",
                "construction unqualified_zone_stage",
                "construction zone_qualified_reference",
                "construction unqualified_numeric_stage",
                "construction scalar_qualified_reference",
                "construction count_comparison_reference",
                "construction qualified_noun_phrase",
                "construction possessive_self_reference",
                "construction possessive_plural_noun",
                "construction possessive",
                "construction intransitive_predicate",
                "construction transitive_predicate",
                "construction numerative_predicate",
                "construction deal_damage",
                "construction gain_life",
                "construction number",
                "construction variable",
                "construction scalar_reference_amount",
                "construction cardinal",
                "abstract sum DocumentBlock",
                "vocab ReflexiveSubordinateKind",
                "abstract product OracleText",
                "root Ability",
                "root Sentence",
                "root Possessive",
                "root CardinalQuantity",
                "root MannerReference",
                "root CountReference",
                "root ScalarReference",
                "root OracleText",
            ],
            "function scan_lexical" | "function possessive_ending_at" => &[
                "vocab Auxiliary",
                "vocab AtBoundary",
                "vocab TriggerMarker",
                "vocab TurnOwnerPostmodifier",
                "vocab TurnPart",
                "vocab TurnSpecifier",
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
                "vocab SingularDemonstrative",
                "vocab ControllerNoun",
                "vocab FixedCostSymbol",
                "vocab ModalChooser",
                "vocab ModalChoiceBounds",
                "vocab MonocoloredHybridColor",
                "vocab ScalarCharacteristic",
                "vocab Zone",
                "vocab NonCommonNoun",
                "vocab NonTargetCommonModifier",
                "vocab Supertype",
                "lexeme CommonNoun",
                "lexeme VerbLexeme",
                "codec IntransitiveVerb",
                "codec TransitiveVerb",
                "codec NumerativeVerb",
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
                "codec LoyaltyMagnitude",
                "construction finite_condition",
                "construction existential_condition",
                "construction triggered",
                "construction preposed_if",
                "construction preposed_if_predicate",
                "construction preposed_as_long_as",
                "construction preposed_as_long_as_predicate",
                "construction preposed_while",
                "construction preposed_while_predicate",
                "construction preposed_during",
                "construction preposed_during_predicate",
                "construction preposed_until",
                "construction preposed_until_predicate",
                "construction reflexive_subordinate",
                "construction reflexive_predicate_subordinate",
                "construction with_where",
                "construction determiner_scoped_and_nominal_series",
                "construction determiner_scoped_or_nominal_series",
                "construction determiner_scoped_and_or_nominal_series",
                "vocab ReflexiveSubordinateKind",
                "root Ability",
                "root Sentence",
                "root Possessive",
                "root CardinalQuantity",
                "root MannerReference",
                "root CountReference",
                "root ScalarReference",
                "root OracleText",
            ],
            "function render_sentences_sentences_sequence"
            | "function walk_sentences_sentences_sequence" => &["construction Sentences"],
            "function render_modal_mode_value_sentences_sequence"
            | "function walk_modal_mode_value_sentences_sequence" => {
                &["construction ModalModeValue"]
            }
            "function render_plain_modal_modes_sequence"
            | "function walk_plain_modal_modes_sequence" => &["construction PlainModal"],
            "function render_symbol_run_symbols_sequence"
            | "function walk_symbol_run_symbols_sequence" => &["construction SymbolRun"],
            "function render_activated_costs_sequence"
            | "function walk_activated_costs_sequence" => &["construction Activated"],
            "function render_then_sequence_members_sequence"
            | "function walk_then_sequence_members_sequence" => &["construction ThenSequence"],
            "function render_then_predicate_sequence_members_sequence"
            | "function walk_then_predicate_sequence_members_sequence" => {
                &["construction ThenPredicateSequence"]
            }
            "function render_and_predicate_coordination_members_sequence"
            | "function walk_and_predicate_coordination_members_sequence" => {
                &["construction AndPredicateCoordination"]
            }
            "function render_or_predicate_coordination_members_sequence"
            | "function walk_or_predicate_coordination_members_sequence" => {
                &["construction OrPredicateCoordination"]
            }
            "function render_and_or_predicate_coordination_members_sequence"
            | "function walk_and_or_predicate_coordination_members_sequence" => {
                &["construction AndOrPredicateCoordination"]
            }
            "function render_and_clause_coordination_members_sequence"
            | "function walk_and_clause_coordination_members_sequence" => {
                &["construction AndClauseCoordination"]
            }
            "function render_or_clause_coordination_members_sequence"
            | "function walk_or_clause_coordination_members_sequence" => {
                &["construction OrClauseCoordination"]
            }
            "function render_and_or_clause_coordination_members_sequence"
            | "function walk_and_or_clause_coordination_members_sequence" => {
                &["construction AndOrClauseCoordination"]
            }
            "function render_negative_modified_singular_nominal_modifiers_sequence"
            | "function walk_negative_modified_singular_nominal_modifiers_sequence" => {
                &["construction NegativeModifiedSingularNominal"]
            }
            "function render_compound_modified_singular_nominal_rest_sequence"
            | "function walk_compound_modified_singular_nominal_rest_sequence" => {
                &["construction CompoundModifiedSingularNominal"]
            }
            "function render_compound_modified_plural_nominal_rest_sequence"
            | "function walk_compound_modified_plural_nominal_rest_sequence" => {
                &["construction CompoundModifiedPluralNominal"]
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
            "function render_determiner_scoped_and_nominal_series_middle_sequence"
            | "function walk_determiner_scoped_and_nominal_series_middle_sequence" => {
                &["construction DeterminerScopedAndNominalSeries"]
            }
            "function render_determiner_scoped_or_nominal_series_middle_sequence"
            | "function walk_determiner_scoped_or_nominal_series_middle_sequence" => {
                &["construction DeterminerScopedOrNominalSeries"]
            }
            "function render_determiner_scoped_and_or_nominal_series_middle_sequence"
            | "function walk_determiner_scoped_and_or_nominal_series_middle_sequence" => {
                &["construction DeterminerScopedAndOrNominalSeries"]
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
            "trait Visitor" => &[
                "construction finite_condition",
                "construction existential_condition",
                "construction singular_existential_clause",
                "construction plural_existential_clause",
                "construction among_phrase",
                "construction plain",
                "construction triggered",
                "construction activated",
                "construction sentences",
                "construction plain_modal",
                "construction modal_mode",
                "construction finite",
                "construction temporal",
                "construction at_phrase",
                "construction generic_cost_symbol",
                "construction fixed_cost_symbol",
                "construction monocolored_hybrid_symbol",
                "construction positive_loyalty",
                "construction zero_loyalty",
                "construction negative_loyalty",
                "construction imperative",
                "construction declarative",
                "construction attached",
                "construction with_where",
                "construction and_predicate_coordination",
                "construction or_predicate_coordination",
                "construction and_or_predicate_coordination",
                "construction plain_finite_clause",
                "construction auxiliary_finite_clause",
                "construction and_clause_coordination",
                "construction or_clause_coordination",
                "construction and_or_clause_coordination",
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
                "construction compound_modifier_member",
                "construction bare_singular_nominal",
                "construction modified_singular_nominal",
                "construction negative_modified_singular_nominal",
                "construction compound_modified_singular_nominal",
                "construction bare_plural_nominal",
                "construction modified_plural_nominal",
                "construction compound_modified_plural_nominal",
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
                "construction determiner_scoped_and_nominal_pair",
                "construction determiner_scoped_and_nominal_series",
                "construction determiner_scoped_or_nominal_pair",
                "construction determiner_scoped_or_nominal_series",
                "construction determiner_scoped_and_or_nominal_pair",
                "construction determiner_scoped_and_or_nominal_series",
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
                "construction indefinite_coordination_reference",
                "construction named_card_reference",
                "construction ordinary_singular_reference",
                "construction ordinary_plural_reference",
                "construction definite_singular_reference",
                "construction definite_plural_reference",
                "construction any_target_reference",
                "construction another_reference",
                "construction another_coordination_reference",
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
                "construction demonstrative_possessive_reference",
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
                "construction that_many",
                "construction target_determiner_phrase",
                "construction target_coordination_determiner_phrase",
                "construction indefinite_determiner_phrase",
                "construction this_determiner_phrase",
                "construction that_determiner_phrase",
                "construction another_determiner_phrase",
                "construction full_and_noun_phrase_coordination",
                "construction full_or_noun_phrase_coordination",
                "construction full_and_or_noun_phrase_coordination",
                "construction this_way",
                "construction that_much",
                "construction you_control",
                "construction opponent_controls",
                "construction demonstrative_controls",
                "construction you_own",
                "construction opponent_controller",
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
                "construction other_than_qualified_reference",
                "construction unqualified_zone_stage",
                "construction zone_qualified_reference",
                "construction unqualified_numeric_stage",
                "construction scalar_qualified_reference",
                "construction qualified_noun_phrase",
                "construction possessive_self_reference",
                "construction possessive_plural_noun",
                "construction possessive",
                "construction intransitive_predicate",
                "construction transitive_predicate",
                "construction numerative_predicate",
                "construction deal_damage",
                "construction gain_life",
                "construction number",
                "construction variable",
                "construction scalar_reference_amount",
                "construction cardinal",
                "codec CardinalNumber",
                "codec ScalarNumber",
                "codec LoyaltyMagnitude",
                "codec TypeNoun",
                "codec ArtifactSubtypeNoun",
                "codec BattleSubtypeNoun",
                "codec CreatureSubtypeNoun",
                "codec EnchantmentSubtypeNoun",
                "codec LandSubtypeNoun",
                "codec PlaneswalkerSubtypeNoun",
                "codec SpellSubtypeNoun",
                "codec IntransitiveVerb",
                "codec TransitiveVerb",
                "codec NumerativeVerb",
                "construction finite_condition",
                "construction existential_condition",
                "construction singular_existential_clause",
                "construction plural_existential_clause",
                "construction among_phrase",
                "construction plain",
                "construction sentences",
                "construction modal_mode",
                "construction plain_modal",
                "construction finite",
                "construction temporal",
                "construction at_phrase",
                "construction triggered",
                "construction generic_cost_symbol",
                "construction fixed_cost_symbol",
                "construction monocolored_hybrid_symbol",
                "construction symbol_run",
                "construction positive_loyalty",
                "construction zero_loyalty",
                "construction negative_loyalty",
                "construction loyalty",
                "construction cost_clause",
                "construction activated",
                "construction imperative",
                "construction declarative",
                "construction attached",
                "construction preposed_if",
                "construction preposed_if_predicate",
                "construction postposed_if",
                "construction postposed_if_predicate",
                "construction postposed_unless",
                "construction postposed_unless_predicate",
                "construction preposed_as_long_as",
                "construction preposed_as_long_as_predicate",
                "construction preposed_while",
                "construction preposed_while_predicate",
                "construction preposed_during",
                "construction preposed_during_predicate",
                "construction preposed_until",
                "construction preposed_until_predicate",
                "construction then_sequence",
                "construction then_predicate_sequence",
                "construction reflexive_subordinate",
                "construction reflexive_predicate_subordinate",
                "construction with_where",
                "construction and_predicate_coordination",
                "construction or_predicate_coordination",
                "construction and_or_predicate_coordination",
                "construction plain_finite_clause",
                "construction auxiliary_finite_clause",
                "construction and_clause_coordination",
                "construction or_clause_coordination",
                "construction and_or_clause_coordination",
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
                "construction compound_modifier_member",
                "construction bare_singular_nominal",
                "construction modified_singular_nominal",
                "construction negative_modified_singular_nominal",
                "construction bare_plural_nominal",
                "construction modified_plural_nominal",
                "construction compound_modified_singular_nominal",
                "construction compound_modified_plural_nominal",
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
                "construction determiner_scoped_and_nominal_pair",
                "construction determiner_scoped_and_nominal_series",
                "construction determiner_scoped_or_nominal_pair",
                "construction determiner_scoped_or_nominal_series",
                "construction determiner_scoped_and_or_nominal_pair",
                "construction determiner_scoped_and_or_nominal_series",
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
                "construction indefinite_coordination_reference",
                "construction named_card_reference",
                "construction ordinary_singular_reference",
                "construction ordinary_plural_reference",
                "construction definite_singular_reference",
                "construction definite_plural_reference",
                "construction any_target_reference",
                "construction another_reference",
                "construction another_coordination_reference",
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
                "construction demonstrative_possessive_reference",
                "construction those_reference",
                "construction designated_singular_reference",
                "construction designated_plural_reference",
                "construction chosen_quality_reference",
                "construction possessed_singular_reference",
                "construction possessed_plural_reference",
                "construction possessive_absolute_reference",
                "construction target_determiner_phrase",
                "construction target_coordination_determiner_phrase",
                "construction indefinite_determiner_phrase",
                "construction this_determiner_phrase",
                "construction that_determiner_phrase",
                "construction another_determiner_phrase",
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
                "construction demonstrative_controls",
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
                "construction other_than_qualified_reference",
                "construction unqualified_zone_stage",
                "construction zone_qualified_reference",
                "construction unqualified_numeric_stage",
                "construction scalar_qualified_reference",
                "construction count_comparison_reference",
                "construction qualified_noun_phrase",
                "construction possessive_self_reference",
                "construction possessive_plural_noun",
                "construction possessive",
                "construction intransitive_predicate",
                "construction transitive_predicate",
                "construction numerative_predicate",
                "construction deal_damage",
                "construction gain_life",
                "construction number",
                "construction variable",
                "construction scalar_reference_amount",
                "construction cardinal",
                "vocab Auxiliary",
                "vocab AtBoundary",
                "vocab TriggerMarker",
                "vocab TurnOwnerPostmodifier",
                "vocab TurnPart",
                "vocab TurnSpecifier",
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
                "vocab SingularDemonstrative",
                "vocab ControllerNoun",
                "vocab FixedCostSymbol",
                "vocab ModalChooser",
                "vocab ModalChoiceBounds",
                "vocab MonocoloredHybridColor",
                "vocab ScalarCharacteristic",
                "vocab Zone",
                "vocab NonCommonNoun",
                "vocab NonTargetCommonModifier",
                "vocab Supertype",
                "vocab ReflexiveSubordinateKind",
                "identity SelfReferenceSpelling",
                "lexeme CommonNoun",
                "lexeme VerbLexeme",
                "lexeme CoreIntransitiveVerb",
                "lexeme CoreTransitiveVerb",
                "lexeme CoreNumerativeVerb",
            ],
            "type Category" => &[
                "abstract sum ActivationCostComponent",
                "abstract sum Predicate",
                "abstract sum Clause",
                "abstract sum ClauseAttachment",
                "abstract sum ConditionClause",
                "construction finite_condition",
                "construction existential_condition",
                "construction singular_existential_clause",
                "construction plural_existential_clause",
                "construction among_phrase",
                "construction plain",
                "construction sentences",
                "construction modal_mode",
                "construction plain_modal",
                "construction finite",
                "construction temporal",
                "construction at_phrase",
                "construction triggered",
                "construction generic_cost_symbol",
                "construction fixed_cost_symbol",
                "construction monocolored_hybrid_symbol",
                "construction symbol_run",
                "construction positive_loyalty",
                "construction zero_loyalty",
                "construction negative_loyalty",
                "construction loyalty",
                "construction cost_clause",
                "construction activated",
                "construction imperative",
                "construction declarative",
                "construction attached",
                "construction preposed_if",
                "construction preposed_if_predicate",
                "construction postposed_if",
                "construction postposed_if_predicate",
                "construction postposed_unless",
                "construction postposed_unless_predicate",
                "construction preposed_as_long_as",
                "construction preposed_as_long_as_predicate",
                "construction preposed_while",
                "construction preposed_while_predicate",
                "construction preposed_during",
                "construction preposed_during_predicate",
                "construction preposed_until",
                "construction preposed_until_predicate",
                "construction then_sequence",
                "construction then_predicate_sequence",
                "construction reflexive_subordinate",
                "construction reflexive_predicate_subordinate",
                "construction with_where",
                "construction and_predicate_coordination",
                "construction or_predicate_coordination",
                "construction and_or_predicate_coordination",
                "construction plain_finite_clause",
                "construction auxiliary_finite_clause",
                "construction and_clause_coordination",
                "construction or_clause_coordination",
                "construction and_or_clause_coordination",
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
                "construction compound_modifier_member",
                "construction bare_singular_nominal",
                "construction modified_singular_nominal",
                "construction negative_modified_singular_nominal",
                "construction bare_plural_nominal",
                "construction modified_plural_nominal",
                "construction compound_modified_singular_nominal",
                "construction compound_modified_plural_nominal",
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
                "construction determiner_scoped_and_nominal_pair",
                "construction determiner_scoped_and_nominal_series",
                "construction determiner_scoped_or_nominal_pair",
                "construction determiner_scoped_or_nominal_series",
                "construction determiner_scoped_and_or_nominal_pair",
                "construction determiner_scoped_and_or_nominal_series",
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
                "construction indefinite_coordination_reference",
                "construction named_card_reference",
                "construction ordinary_singular_reference",
                "construction ordinary_plural_reference",
                "construction definite_singular_reference",
                "construction definite_plural_reference",
                "construction any_target_reference",
                "construction another_reference",
                "construction another_coordination_reference",
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
                "construction demonstrative_possessive_reference",
                "construction those_reference",
                "construction designated_singular_reference",
                "construction designated_plural_reference",
                "construction chosen_quality_reference",
                "construction possessed_singular_reference",
                "construction possessed_plural_reference",
                "construction possessive_absolute_reference",
                "construction target_determiner_phrase",
                "construction target_coordination_determiner_phrase",
                "construction indefinite_determiner_phrase",
                "construction this_determiner_phrase",
                "construction that_determiner_phrase",
                "construction another_determiner_phrase",
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
                "construction demonstrative_controls",
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
                "construction other_than_qualified_reference",
                "construction unqualified_zone_stage",
                "construction zone_qualified_reference",
                "construction unqualified_numeric_stage",
                "construction scalar_qualified_reference",
                "construction count_comparison_reference",
                "construction qualified_noun_phrase",
                "construction possessive_self_reference",
                "construction possessive_plural_noun",
                "construction possessive",
                "construction intransitive_predicate",
                "construction transitive_predicate",
                "construction numerative_predicate",
                "construction deal_damage",
                "construction gain_life",
                "construction number",
                "construction variable",
                "construction scalar_reference_amount",
                "construction cardinal",
                "abstract sum DocumentBlock",
                "abstract product OracleText",
            ],
            "type Construction"
            | "impl Construction"
            | "type RuleId"
            | "impl RuleId"
            | "function build_checked"
            | "function build" => &[
                "construction finite_condition",
                "construction existential_condition",
                "construction singular_existential_clause",
                "construction plural_existential_clause",
                "construction among_phrase",
                "construction plain",
                "construction sentences",
                "construction modal_mode",
                "construction plain_modal",
                "construction finite",
                "construction temporal",
                "construction at_phrase",
                "construction triggered",
                "construction generic_cost_symbol",
                "construction fixed_cost_symbol",
                "construction monocolored_hybrid_symbol",
                "construction symbol_run",
                "construction positive_loyalty",
                "construction zero_loyalty",
                "construction negative_loyalty",
                "construction loyalty",
                "construction cost_clause",
                "construction activated",
                "construction imperative",
                "construction declarative",
                "construction attached",
                "construction preposed_if",
                "construction preposed_if_predicate",
                "construction postposed_if",
                "construction postposed_if_predicate",
                "construction postposed_unless",
                "construction postposed_unless_predicate",
                "construction preposed_as_long_as",
                "construction preposed_as_long_as_predicate",
                "construction preposed_while",
                "construction preposed_while_predicate",
                "construction preposed_during",
                "construction preposed_during_predicate",
                "construction preposed_until",
                "construction preposed_until_predicate",
                "construction then_sequence",
                "construction then_predicate_sequence",
                "construction reflexive_subordinate",
                "construction reflexive_predicate_subordinate",
                "construction with_where",
                "construction and_predicate_coordination",
                "construction or_predicate_coordination",
                "construction and_or_predicate_coordination",
                "construction plain_finite_clause",
                "construction auxiliary_finite_clause",
                "construction and_clause_coordination",
                "construction or_clause_coordination",
                "construction and_or_clause_coordination",
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
                "construction compound_modifier_member",
                "construction bare_singular_nominal",
                "construction modified_singular_nominal",
                "construction negative_modified_singular_nominal",
                "construction bare_plural_nominal",
                "construction modified_plural_nominal",
                "construction compound_modified_singular_nominal",
                "construction compound_modified_plural_nominal",
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
                "construction determiner_scoped_and_nominal_pair",
                "construction determiner_scoped_and_nominal_series",
                "construction determiner_scoped_or_nominal_pair",
                "construction determiner_scoped_or_nominal_series",
                "construction determiner_scoped_and_or_nominal_pair",
                "construction determiner_scoped_and_or_nominal_series",
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
                "construction indefinite_coordination_reference",
                "construction named_card_reference",
                "construction ordinary_singular_reference",
                "construction ordinary_plural_reference",
                "construction definite_singular_reference",
                "construction definite_plural_reference",
                "construction any_target_reference",
                "construction another_reference",
                "construction another_coordination_reference",
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
                "construction demonstrative_possessive_reference",
                "construction those_reference",
                "construction designated_singular_reference",
                "construction designated_plural_reference",
                "construction chosen_quality_reference",
                "construction possessed_singular_reference",
                "construction possessed_plural_reference",
                "construction possessive_absolute_reference",
                "construction target_determiner_phrase",
                "construction target_coordination_determiner_phrase",
                "construction indefinite_determiner_phrase",
                "construction this_determiner_phrase",
                "construction that_determiner_phrase",
                "construction another_determiner_phrase",
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
                "construction demonstrative_controls",
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
                "construction other_than_qualified_reference",
                "construction unqualified_zone_stage",
                "construction zone_qualified_reference",
                "construction unqualified_numeric_stage",
                "construction scalar_qualified_reference",
                "construction count_comparison_reference",
                "construction qualified_noun_phrase",
                "construction possessive_self_reference",
                "construction possessive_plural_noun",
                "construction possessive",
                "construction intransitive_predicate",
                "construction transitive_predicate",
                "construction numerative_predicate",
                "construction deal_damage",
                "construction gain_life",
                "construction number",
                "construction variable",
                "construction scalar_reference_amount",
                "construction cardinal",
            ],
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
            "constant RULES" => &[
                "construction finite_condition",
                "construction existential_condition",
                "construction singular_existential_clause",
                "construction plural_existential_clause",
                "construction among_phrase",
                "construction plain",
                "construction sentences",
                "construction modal_mode",
                "construction plain_modal",
                "construction finite",
                "construction temporal",
                "construction at_phrase",
                "construction triggered",
                "construction generic_cost_symbol",
                "construction fixed_cost_symbol",
                "construction monocolored_hybrid_symbol",
                "construction symbol_run",
                "construction positive_loyalty",
                "construction zero_loyalty",
                "construction negative_loyalty",
                "construction loyalty",
                "construction cost_clause",
                "construction activated",
                "construction imperative",
                "construction declarative",
                "construction attached",
                "construction preposed_if",
                "construction preposed_if_predicate",
                "construction postposed_if",
                "construction postposed_if_predicate",
                "construction postposed_unless",
                "construction postposed_unless_predicate",
                "construction preposed_as_long_as",
                "construction preposed_as_long_as_predicate",
                "construction preposed_while",
                "construction preposed_while_predicate",
                "construction preposed_during",
                "construction preposed_during_predicate",
                "construction preposed_until",
                "construction preposed_until_predicate",
                "construction then_sequence",
                "construction then_predicate_sequence",
                "construction reflexive_subordinate",
                "construction reflexive_predicate_subordinate",
                "construction with_where",
                "construction and_predicate_coordination",
                "construction or_predicate_coordination",
                "construction and_or_predicate_coordination",
                "construction plain_finite_clause",
                "construction auxiliary_finite_clause",
                "construction and_clause_coordination",
                "construction or_clause_coordination",
                "construction and_or_clause_coordination",
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
                "construction compound_modifier_member",
                "construction bare_singular_nominal",
                "construction modified_singular_nominal",
                "construction negative_modified_singular_nominal",
                "construction bare_plural_nominal",
                "construction modified_plural_nominal",
                "construction compound_modified_singular_nominal",
                "construction compound_modified_plural_nominal",
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
                "construction determiner_scoped_and_nominal_pair",
                "construction determiner_scoped_and_nominal_series",
                "construction determiner_scoped_or_nominal_pair",
                "construction determiner_scoped_or_nominal_series",
                "construction determiner_scoped_and_or_nominal_pair",
                "construction determiner_scoped_and_or_nominal_series",
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
                "construction indefinite_coordination_reference",
                "construction named_card_reference",
                "construction ordinary_singular_reference",
                "construction ordinary_plural_reference",
                "construction definite_singular_reference",
                "construction definite_plural_reference",
                "construction any_target_reference",
                "construction another_reference",
                "construction another_coordination_reference",
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
                "construction demonstrative_possessive_reference",
                "construction those_reference",
                "construction designated_singular_reference",
                "construction designated_plural_reference",
                "construction chosen_quality_reference",
                "construction possessed_singular_reference",
                "construction possessed_plural_reference",
                "construction possessive_absolute_reference",
                "construction target_determiner_phrase",
                "construction target_coordination_determiner_phrase",
                "construction indefinite_determiner_phrase",
                "construction this_determiner_phrase",
                "construction that_determiner_phrase",
                "construction another_determiner_phrase",
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
                "construction demonstrative_controls",
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
                "construction other_than_qualified_reference",
                "construction unqualified_zone_stage",
                "construction zone_qualified_reference",
                "construction unqualified_numeric_stage",
                "construction scalar_qualified_reference",
                "construction count_comparison_reference",
                "construction qualified_noun_phrase",
                "construction possessive_self_reference",
                "construction possessive_plural_noun",
                "construction possessive",
                "construction intransitive_predicate",
                "construction transitive_predicate",
                "construction numerative_predicate",
                "construction deal_damage",
                "construction gain_life",
                "construction number",
                "construction variable",
                "construction scalar_reference_amount",
                "construction cardinal",
                "root Ability",
                "root Possessive",
                "root CardinalQuantity",
                "root MannerReference",
                "root CountReference",
                "root ScalarReference",
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
        "type FiniteCondition",
        "type ExistentialCondition",
        "type ExistentialClause",
        "type AmongPhrase",
        "type Ability",
        "type AbilityBody",
        "type ModalMode",
        "type TriggerPrefix",
        "type AtPhrase",
        "type CostSymbol",
        "type LoyaltyValue",
        "type Sentence",
        "type PredicateCoordination",
        "type FiniteClause",
        "type ClauseCoordination",
        "type WhereClauseCategory",
        "type Subject",
        "type Object",
        "type SingularHead",
        "type PluralHead",
        "type NominalModifier",
        "type NegativeNominalModifier",
        "type CoordinatedNominalModifier",
        "type CompoundNominalModifier",
        "type SingularNominal",
        "type PluralNominal",
        "type SingularCoordinationMember",
        "type PluralCoordinationMember",
        "type SingularNominalCoordination",
        "type DeterminerScopedNominalCoordination",
        "type PluralNominalCoordination",
        "type SingularSelector",
        "type PluralSelector",
        "type UnqualifiedReference",
        "type CountReference",
        "type DeterminerPhrase",
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
        "type ActivationCostComponent",
        "type Predicate",
        "type Clause",
        "type ClauseAttachment",
        "type ConditionClause",
        "type DocumentBlock",
        "type OracleText",
        "type FiniteConditionValue",
        "type ExistentialConditionValue",
        "type SingularExistentialClause",
        "impl SingularExistentialClause",
        "type PluralExistentialClause",
        "impl PluralExistentialClause",
        "type AmongPhraseValue",
        "type Plain",
        "type Sentences",
        "impl Sentences",
        "type ModalModeValue",
        "impl ModalModeValue",
        "type PlainModal",
        "impl PlainModal",
        "type Finite",
        "type Temporal",
        "type AtPhraseValue",
        "impl AtPhraseValue",
        "type Triggered",
        "type GenericCostSymbol",
        "type FixedSymbol",
        "type MonocoloredHybridSymbol",
        "type SymbolRun",
        "impl SymbolRun",
        "type PositiveLoyalty",
        "type ZeroLoyalty",
        "type NegativeLoyalty",
        "type Loyalty",
        "type CostClause",
        "impl CostClause",
        "type Activated",
        "impl Activated",
        "type Imperative",
        "impl Imperative",
        "type Declarative",
        "type Attached",
        "type PreposedIf",
        "type PreposedIfPredicate",
        "impl PreposedIfPredicate",
        "type PostposedIf",
        "type PostposedIfPredicate",
        "impl PostposedIfPredicate",
        "type PostposedUnless",
        "type PostposedUnlessPredicate",
        "impl PostposedUnlessPredicate",
        "type PreposedAsLongAs",
        "type PreposedAsLongAsPredicate",
        "impl PreposedAsLongAsPredicate",
        "type PreposedWhile",
        "type PreposedWhilePredicate",
        "impl PreposedWhilePredicate",
        "type PreposedDuring",
        "type PreposedDuringPredicate",
        "impl PreposedDuringPredicate",
        "type PreposedUntil",
        "type PreposedUntilPredicate",
        "impl PreposedUntilPredicate",
        "type ThenSequence",
        "impl ThenSequence",
        "type ThenPredicateSequence",
        "impl ThenPredicateSequence",
        "type ReflexiveSubordinate",
        "type ReflexivePredicateSubordinate",
        "impl ReflexivePredicateSubordinate",
        "type WithWhere",
        "type AndPredicateCoordination",
        "impl AndPredicateCoordination",
        "type OrPredicateCoordination",
        "impl OrPredicateCoordination",
        "type AndOrPredicateCoordination",
        "impl AndOrPredicateCoordination",
        "type PlainFiniteClause",
        "impl PlainFiniteClause",
        "type AuxiliaryFiniteClause",
        "impl AuxiliaryFiniteClause",
        "type AndClauseCoordination",
        "impl AndClauseCoordination",
        "type OrClauseCoordination",
        "impl OrClauseCoordination",
        "type AndOrClauseCoordination",
        "impl AndOrClauseCoordination",
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
        "type CompoundModifierMember",
        "impl CompoundModifierMember",
        "type BareSingularNominal",
        "type ModifiedSingularNominal",
        "type NegativeModifiedSingularNominal",
        "impl NegativeModifiedSingularNominal",
        "type BarePluralNominal",
        "type ModifiedPluralNominal",
        "type CompoundModifiedSingularNominal",
        "impl CompoundModifiedSingularNominal",
        "type CompoundModifiedPluralNominal",
        "impl CompoundModifiedPluralNominal",
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
        "type DeterminerScopedAndNominalPair",
        "type DeterminerScopedAndNominalSeries",
        "impl DeterminerScopedAndNominalSeries",
        "type DeterminerScopedOrNominalPair",
        "type DeterminerScopedOrNominalSeries",
        "impl DeterminerScopedOrNominalSeries",
        "type DeterminerScopedAndOrNominalPair",
        "type DeterminerScopedAndOrNominalSeries",
        "impl DeterminerScopedAndOrNominalSeries",
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
        "type IndefiniteCoordinationReference",
        "type NamedCardReference",
        "type OrdinarySingularReference",
        "impl OrdinarySingularReference",
        "type OrdinaryPluralReference",
        "impl OrdinaryPluralReference",
        "type DefiniteSingularReference",
        "type DefinitePluralReference",
        "type AnyTargetReference",
        "type AnotherReference",
        "impl AnotherReference",
        "type AnotherCoordinationReference",
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
        "type DemonstrativePossessiveReference",
        "type ThoseReference",
        "type DesignatedSingularReference",
        "type DesignatedPluralReference",
        "type ChosenQualityReference",
        "type PossessedSingularReference",
        "type PossessedPluralReference",
        "type PossessiveAbsoluteReference",
        "type TargetDeterminerPhrase",
        "type TargetCoordinationDeterminerPhrase",
        "type IndefiniteDeterminerPhrase",
        "type ThisDeterminerPhrase",
        "type ThatDeterminerPhrase",
        "type AnotherDeterminerPhrase",
        "impl AnotherDeterminerPhrase",
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
        "type DemonstrativeControls",
        "impl DemonstrativeControls",
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
        "type OtherThanQualifiedReference",
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
        "type IntransitivePredicate",
        "type TransitivePredicate",
        "type NumerativePredicate",
        "type DealDamage",
        "type GainLife",
        "type NumberAmount",
        "type VariableAmount",
        "type ScalarReferenceAmount",
        "type CardinalQuantityValue",
        "type Auxiliary",
        "type AtBoundary",
        "type TriggerMarker",
        "type TurnOwnerPostmodifier",
        "type TurnPart",
        "type TurnSpecifier",
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
        "type SingularDemonstrative",
        "type ControllerNoun",
        "type FixedCostSymbol",
        "type ModalChooser",
        "type ModalChoiceBounds",
        "type MonocoloredHybridColor",
        "type ScalarCharacteristic",
        "type Zone",
        "type NonCommonNoun",
        "type NonTargetCommonModifier",
        "type Supertype",
        "type CommonNoun",
        "function surface_for_common_noun",
        "type VerbLexeme",
        "function surface_for_verb_lexeme",
        "type CoreIntransitiveVerb",
        "function surface_for_core_intransitive_verb",
        "type CoreTransitiveVerb",
        "function surface_for_core_transitive_verb",
        "type CoreNumerativeVerb",
        "function surface_for_core_numerative_verb",
        "type DeclarationIntransitiveVerb",
        "impl DeclarationIntransitiveVerb",
        "type IntransitiveVerb",
        "type DeclarationTransitiveVerb",
        "impl DeclarationTransitiveVerb",
        "type TransitiveVerb",
        "type DeclarationNumerativeVerb",
        "impl DeclarationNumerativeVerb",
        "type NumerativeVerb",
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
        "type LoyaltyMagnitude",
        "type ReflexiveSubordinateKind",
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
        "type VerbFrameAtom",
        "type VerbFrameKey",
        "impl VerbFrameKey",
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
        "function scan_lexical",
        "function possessive_ending_at",
        "type SequenceOwner",
        "type FixedSurfaceAtom",
        "function sequence_separator",
        "function sequence_terminator",
        "function render_oracle_text_blocks_sequence",
        "function render_oracle_text",
        "function render_sentences_sentences_sequence",
        "function render_modal_mode_value_sentences_sequence",
        "function render_plain_modal_modes_sequence",
        "function render_symbol_run_symbols_sequence",
        "function render_symbol_run",
        "function render_loyalty",
        "function render_cost_clause",
        "function render_activated_costs_sequence",
        "function render_preposed_if",
        "function render_preposed_if_predicate",
        "function render_postposed_if",
        "function render_postposed_if_predicate",
        "function render_postposed_unless",
        "function render_postposed_unless_predicate",
        "function render_preposed_as_long_as",
        "function render_preposed_as_long_as_predicate",
        "function render_preposed_while",
        "function render_preposed_while_predicate",
        "function render_preposed_during",
        "function render_preposed_during_predicate",
        "function render_preposed_until",
        "function render_preposed_until_predicate",
        "function render_then_sequence_members_sequence",
        "function render_then_sequence",
        "function render_then_predicate_sequence_members_sequence",
        "function render_then_predicate_sequence",
        "function render_reflexive_subordinate",
        "function render_reflexive_predicate_subordinate",
        "function render_and_predicate_coordination_members_sequence",
        "function render_or_predicate_coordination_members_sequence",
        "function render_and_or_predicate_coordination_members_sequence",
        "function render_and_clause_coordination_members_sequence",
        "function render_or_clause_coordination_members_sequence",
        "function render_and_or_clause_coordination_members_sequence",
        "function render_negative_modified_singular_nominal_modifiers_sequence",
        "function render_compound_modified_singular_nominal_rest_sequence",
        "function render_compound_modified_plural_nominal_rest_sequence",
        "function render_negative_modified_plural_nominal_modifiers_sequence",
        "function render_negative_modified_singular_coordination_member_modifiers_sequence",
        "function render_negative_modified_plural_coordination_member_modifiers_sequence",
        "function render_singular_and_nominal_coordination_members_sequence",
        "function render_singular_or_nominal_coordination_members_sequence",
        "function render_singular_and_or_nominal_coordination_members_sequence",
        "function render_determiner_scoped_and_nominal_series_middle_sequence",
        "function render_determiner_scoped_or_nominal_series_middle_sequence",
        "function render_determiner_scoped_and_or_nominal_series_middle_sequence",
        "function render_plural_and_nominal_coordination_members_sequence",
        "function render_plural_or_nominal_coordination_members_sequence",
        "function render_plural_and_or_nominal_coordination_members_sequence",
        "function render_full_and_noun_phrase_coordination_members_sequence",
        "function render_full_or_noun_phrase_coordination_members_sequence",
        "function render_full_and_or_noun_phrase_coordination_members_sequence",
        "function render_activation_cost_component",
        "function render_predicate",
        "function render_clause",
        "function render_clause_attachment",
        "function render_condition_clause",
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
        "function render_finite_condition",
        "function render_existential_condition",
        "function render_existential_clause",
        "function render_among_phrase",
        "function __deckmaste_construction_internal_render_root_ability",
        "function render_ability_body",
        "function render_modal_mode",
        "function render_trigger_prefix",
        "function render_at_phrase",
        "function render_cost_symbol",
        "function render_loyalty_value",
        "function __deckmaste_construction_internal_render_root_sentence",
        "function render_predicate_coordination",
        "function render_finite_clause",
        "function render_clause_coordination",
        "function render_where_clause_category",
        "function render_subject",
        "function render_object",
        "function render_singular_head",
        "function render_plural_head",
        "function render_nominal_modifier",
        "function render_negative_nominal_modifier",
        "function render_coordinated_nominal_modifier",
        "function render_compound_nominal_modifier",
        "function render_singular_nominal",
        "function render_plural_nominal",
        "function render_singular_coordination_member",
        "function render_plural_coordination_member",
        "function render_singular_nominal_coordination",
        "function render_determiner_scoped_nominal_coordination",
        "function render_plural_nominal_coordination",
        "function render_singular_selector",
        "function render_plural_selector",
        "function render_unqualified_reference",
        "function __deckmaste_construction_internal_render_root_count_reference",
        "function render_determiner_phrase",
        "function render_full_noun_phrase_coordination",
        "function __deckmaste_construction_internal_render_root_scalar_reference",
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
        "function __deckmaste_construction_internal_render_root_cardinal_quantity",
        "function render_auxiliary",
        "function render_at_boundary",
        "function render_trigger_marker",
        "function render_turn_owner_postmodifier",
        "function render_turn_part",
        "function render_turn_specifier",
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
        "function render_singular_demonstrative",
        "function render_controller_noun",
        "function render_fixed_cost_symbol",
        "function render_modal_chooser",
        "function render_modal_choice_bounds",
        "function render_monocolored_hybrid_color",
        "function render_scalar_characteristic",
        "function render_zone",
        "function render_non_common_noun",
        "function render_non_target_common_modifier",
        "function render_supertype",
        "function render_reflexive_subordinate_kind",
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
        "function format_loyalty_magnitude",
        "function parse_loyalty_magnitude",
        "function render_loyalty_magnitude",
        "function agreement_for_subject",
        "function agreement_for_singular_head",
        "function agreement_for_plural_head",
        "function agreement_for_singular_nominal",
        "function agreement_for_plural_nominal",
        "function agreement_for_singular_selector",
        "function agreement_for_plural_selector",
        "function agreement_for_unqualified_reference",
        "function agreement_for_count_reference",
        "function agreement_for_determiner_phrase",
        "function agreement_for_full_noun_phrase_coordination",
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
        "function number_for_determiner_phrase",
        "function number_for_full_noun_phrase_coordination",
        "function number_for_controller_stage",
        "function number_for_zone_stage",
        "function number_for_numeric_stage",
        "function number_for_noun_phrase",
        "function number_for_possessive_owner",
        "function onset_for_singular_head",
        "function onset_for_plural_head",
        "function onset_for_nominal_modifier",
        "function onset_for_coordinated_nominal_modifier",
        "function onset_for_compound_nominal_modifier",
        "function onset_for_singular_nominal",
        "function onset_for_plural_nominal",
        "function onset_for_singular_coordination_member",
        "function onset_for_determiner_scoped_nominal_coordination",
        "function onset_for_plural_selector",
        "function onset_for_unqualified_reference",
        "function onset_for_count_reference",
        "function onset_for_determiner_phrase",
        "function onset_for_controller_stage",
        "function onset_for_zone_stage",
        "function onset_for_numeric_stage",
        "function onset_for_noun_phrase",
        "function possessive_ending_for_plural_head",
        "function possessive_ending_for_possessive_owner",
        "function agreement_matches_for_predicate_coordination",
        "function agreement_matches_for_subject",
        "function agreement_matches_for_object",
        "function agreement_matches_for_singular_head",
        "function agreement_matches_for_plural_head",
        "function agreement_matches_for_nominal_modifier",
        "function agreement_matches_for_singular_nominal",
        "function agreement_matches_for_plural_nominal",
        "function agreement_matches_for_singular_coordination_member",
        "function agreement_matches_for_plural_coordination_member",
        "function agreement_matches_for_singular_selector",
        "function agreement_matches_for_plural_selector",
        "function agreement_matches_for_unqualified_reference",
        "function agreement_matches_for_count_reference",
        "function agreement_matches_for_determiner_phrase",
        "function agreement_matches_for_full_noun_phrase_coordination",
        "function agreement_matches_for_scalar_reference",
        "function agreement_matches_for_controller_stage",
        "function agreement_matches_for_zone_stage",
        "function agreement_matches_for_numeric_stage",
        "function agreement_matches_for_noun_phrase",
        "function agreement_matches_for_verb_phrase",
        "function agreement_matches_for_predicate",
        "trait Visitor",
        "function walk_finite_condition",
        "function walk_existential_condition",
        "function walk_existential_clause",
        "function walk_among_phrase",
        "function walk_ability",
        "function walk_ability_body",
        "function walk_modal_mode",
        "function walk_trigger_prefix",
        "function walk_at_phrase",
        "function walk_cost_symbol",
        "function walk_loyalty_value",
        "function walk_sentence",
        "function walk_predicate_coordination",
        "function walk_finite_clause",
        "function walk_clause_coordination",
        "function walk_where_clause_category",
        "function walk_subject",
        "function walk_object",
        "function walk_singular_head",
        "function walk_plural_head",
        "function walk_nominal_modifier",
        "function walk_negative_nominal_modifier",
        "function walk_coordinated_nominal_modifier",
        "function walk_compound_nominal_modifier",
        "function walk_singular_nominal",
        "function walk_plural_nominal",
        "function walk_singular_coordination_member",
        "function walk_plural_coordination_member",
        "function walk_singular_nominal_coordination",
        "function walk_determiner_scoped_nominal_coordination",
        "function walk_plural_nominal_coordination",
        "function walk_singular_selector",
        "function walk_plural_selector",
        "function walk_unqualified_reference",
        "function walk_count_reference",
        "function walk_determiner_phrase",
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
        "function walk_activation_cost_component",
        "function walk_predicate",
        "function walk_clause",
        "function walk_clause_attachment",
        "function walk_condition_clause",
        "function walk_sentences_sentences_sequence",
        "function walk_modal_mode_value_sentences_sequence",
        "function walk_plain_modal_modes_sequence",
        "function walk_symbol_run_symbols_sequence",
        "function walk_activated_costs_sequence",
        "function walk_then_sequence_members_sequence",
        "function walk_then_predicate_sequence_members_sequence",
        "function walk_and_predicate_coordination_members_sequence",
        "function walk_or_predicate_coordination_members_sequence",
        "function walk_and_or_predicate_coordination_members_sequence",
        "function walk_and_clause_coordination_members_sequence",
        "function walk_or_clause_coordination_members_sequence",
        "function walk_and_or_clause_coordination_members_sequence",
        "function walk_negative_modified_singular_nominal_modifiers_sequence",
        "function walk_compound_modified_singular_nominal_rest_sequence",
        "function walk_compound_modified_plural_nominal_rest_sequence",
        "function walk_negative_modified_plural_nominal_modifiers_sequence",
        "function walk_negative_modified_singular_coordination_member_modifiers_sequence",
        "function walk_negative_modified_plural_coordination_member_modifiers_sequence",
        "function walk_singular_and_nominal_coordination_members_sequence",
        "function walk_singular_or_nominal_coordination_members_sequence",
        "function walk_singular_and_or_nominal_coordination_members_sequence",
        "function walk_determiner_scoped_and_nominal_series_middle_sequence",
        "function walk_determiner_scoped_or_nominal_series_middle_sequence",
        "function walk_determiner_scoped_and_or_nominal_series_middle_sequence",
        "function walk_plural_and_nominal_coordination_members_sequence",
        "function walk_plural_or_nominal_coordination_members_sequence",
        "function walk_plural_and_or_nominal_coordination_members_sequence",
        "function walk_full_and_noun_phrase_coordination_members_sequence",
        "function walk_full_or_noun_phrase_coordination_members_sequence",
        "function walk_full_and_or_noun_phrase_coordination_members_sequence",
        "function walk_document_block",
        "function walk_oracle_text_blocks_sequence",
        "function walk_oracle_text",
        "function walk_finite_condition_value",
        "function walk_existential_condition_value",
        "function walk_singular_existential_clause",
        "function walk_plural_existential_clause",
        "function walk_among_phrase_value",
        "function walk_plain",
        "function walk_sentences",
        "function walk_modal_mode_value",
        "function walk_plain_modal",
        "function walk_finite",
        "function walk_temporal",
        "function walk_at_phrase_value",
        "function walk_triggered",
        "function walk_generic_cost_symbol",
        "function walk_fixed_symbol",
        "function walk_monocolored_hybrid_symbol",
        "function walk_symbol_run",
        "function walk_positive_loyalty",
        "function walk_zero_loyalty",
        "function walk_negative_loyalty",
        "function walk_loyalty",
        "function walk_cost_clause",
        "function walk_activated",
        "function walk_imperative",
        "function walk_declarative",
        "function walk_attached",
        "function walk_preposed_if",
        "function walk_preposed_if_predicate",
        "function walk_postposed_if",
        "function walk_postposed_if_predicate",
        "function walk_postposed_unless",
        "function walk_postposed_unless_predicate",
        "function walk_preposed_as_long_as",
        "function walk_preposed_as_long_as_predicate",
        "function walk_preposed_while",
        "function walk_preposed_while_predicate",
        "function walk_preposed_during",
        "function walk_preposed_during_predicate",
        "function walk_preposed_until",
        "function walk_preposed_until_predicate",
        "function walk_then_sequence",
        "function walk_then_predicate_sequence",
        "function walk_reflexive_subordinate",
        "function walk_reflexive_predicate_subordinate",
        "function walk_with_where",
        "function walk_and_predicate_coordination",
        "function walk_or_predicate_coordination",
        "function walk_and_or_predicate_coordination",
        "function walk_plain_finite_clause",
        "function walk_auxiliary_finite_clause",
        "function walk_and_clause_coordination",
        "function walk_or_clause_coordination",
        "function walk_and_or_clause_coordination",
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
        "function walk_compound_modifier_member",
        "function walk_bare_singular_nominal",
        "function walk_modified_singular_nominal",
        "function walk_negative_modified_singular_nominal",
        "function walk_bare_plural_nominal",
        "function walk_modified_plural_nominal",
        "function walk_compound_modified_singular_nominal",
        "function walk_compound_modified_plural_nominal",
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
        "function walk_determiner_scoped_and_nominal_pair",
        "function walk_determiner_scoped_and_nominal_series",
        "function walk_determiner_scoped_or_nominal_pair",
        "function walk_determiner_scoped_or_nominal_series",
        "function walk_determiner_scoped_and_or_nominal_pair",
        "function walk_determiner_scoped_and_or_nominal_series",
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
        "function walk_indefinite_coordination_reference",
        "function walk_named_card_reference",
        "function walk_ordinary_singular_reference",
        "function walk_ordinary_plural_reference",
        "function walk_definite_singular_reference",
        "function walk_definite_plural_reference",
        "function walk_any_target_reference",
        "function walk_another_reference",
        "function walk_another_coordination_reference",
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
        "function walk_demonstrative_possessive_reference",
        "function walk_those_reference",
        "function walk_designated_singular_reference",
        "function walk_designated_plural_reference",
        "function walk_chosen_quality_reference",
        "function walk_possessed_singular_reference",
        "function walk_possessed_plural_reference",
        "function walk_possessive_absolute_reference",
        "function walk_target_determiner_phrase",
        "function walk_target_coordination_determiner_phrase",
        "function walk_indefinite_determiner_phrase",
        "function walk_this_determiner_phrase",
        "function walk_that_determiner_phrase",
        "function walk_another_determiner_phrase",
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
        "function walk_demonstrative_controls",
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
        "function walk_other_than_qualified_reference",
        "function walk_unqualified_zone_stage",
        "function walk_zone_qualified_reference",
        "function walk_unqualified_numeric_stage",
        "function walk_scalar_qualified_reference",
        "function walk_count_comparison_reference",
        "function walk_qualified_noun_phrase",
        "function walk_possessive_self_reference",
        "function walk_possessive_noun",
        "function walk_possessive_value",
        "function walk_intransitive_predicate",
        "function walk_transitive_predicate",
        "function walk_numerative_predicate",
        "function walk_deal_damage",
        "function walk_gain_life",
        "function walk_number_amount",
        "function walk_variable_amount",
        "function walk_scalar_reference_amount",
        "function walk_cardinal_quantity_value",
        "function walk_auxiliary",
        "function walk_at_boundary",
        "function walk_trigger_marker",
        "function walk_turn_owner_postmodifier",
        "function walk_turn_part",
        "function walk_turn_specifier",
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
        "function walk_singular_demonstrative",
        "function walk_controller_noun",
        "function walk_fixed_cost_symbol",
        "function walk_modal_chooser",
        "function walk_modal_choice_bounds",
        "function walk_monocolored_hybrid_color",
        "function walk_scalar_characteristic",
        "function walk_zone",
        "function walk_non_common_noun",
        "function walk_non_target_common_modifier",
        "function walk_supertype",
        "function walk_reflexive_subordinate_kind",
        "function walk_self_reference_spelling",
        "function walk_card_name",
        "function walk_common_noun",
        "function walk_verb_lexeme",
        "function walk_core_intransitive_verb",
        "function walk_core_transitive_verb",
        "function walk_core_numerative_verb",
        "function walk_cardinal_number",
        "function walk_scalar_number",
        "function walk_loyalty_magnitude",
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
        "function walk_declaration_intransitive_verb",
        "function walk_intransitive_verb",
        "function walk_declaration_transitive_verb",
        "function walk_transitive_verb",
        "function walk_declaration_numerative_verb",
        "function walk_numerative_verb",
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
        assert_eq!(
            verbs,
            [
                "Add", "Deal", "Draw", "Enter", "Gain", "Lose", "Pay", "Put", "Remove", "Roll",
                "Have", "Look", "Leave", "Control", "Own", "Return", "Become", "Be"
            ]
        );

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

    fn expansion_inventory(expansion: &Expansion) -> Vec<(String, Vec<String>)> {
        expansion
            .items()
            .iter()
            .map(|item| {
                (
                    item_key_name(&item.key),
                    item.origins
                        .iter()
                        .map(|origin| {
                            format!("{} {}", declaration_kind_name(origin.kind()), origin.name())
                        })
                        .collect(),
                )
            })
            .collect()
    }

    fn parse_plan09_expansion_inventory(source: &str) -> Vec<(String, String)> {
        source
            .lines()
            .filter(|line| !line.starts_with('#'))
            .map(|line| {
                let fields = line.split('\t').collect::<Vec<_>>();
                let ["item", item, origins_digest] = fields.as_slice() else {
                    panic!("generated expansion row has three exact fields: {fields:?}");
                };
                (
                    serde_json::from_str(item).expect("generated item key is JSON"),
                    (*origins_digest).to_owned(),
                )
            })
            .collect()
    }

    fn render_plan09_expansion_inventory(
        inventory: &[(String, Vec<String>)],
    ) -> Result<String, String> {
        let mut rendered = format!(
            concat!(
                "# English v2 Plan 09 generated expansion inventory\n",
                "# schema=1\n",
                "# item_count={}\n",
                "# columns=kind\\titem_json\\torigins_sha256\n",
            ),
            inventory.len(),
        );
        for (item, origins) in inventory {
            writeln!(
                &mut rendered,
                "item\t{}\t{}",
                serde_json::to_string(item)
                    .map_err(|error| format!("encoding generated item key: {error}"))?,
                sha256_hex(origins.join("\0").as_bytes()),
            )
            .map_err(|error| format!("rendering generated item row: {error}"))?;
        }
        Ok(rendered)
    }

    #[test]
    #[ignore = "explicit fixture refresh expands the complete production declaration"]
    fn refresh_plan09_expansion_inventory() {
        let expansion = expansion_from_source(PRODUCTION_SOURCE)
            .expect("production declaration expands from the sealed semantic plan");
        let rendered = render_plan09_expansion_inventory(&expansion_inventory(&expansion))
            .expect("generated expansion inventory renders");
        let path =
            Path::new(env!("CARGO_MANIFEST_DIR")).join("src/english_v2/plan09_task8_expansion.tsv");
        fs::write(path, rendered).expect("generated expansion inventory writes");
    }

    #[test]
    #[allow(
        clippy::too_many_lines,
        reason = "the authority audit compares each historical inventory and origin set together"
    )]
    fn production_expansion_prints_each_literal_item_key_once_with_every_origin() {
        let expansion = expansion_from_source(PRODUCTION_SOURCE)
            .expect("production declaration expands from the sealed semantic plan");
        let output = render_expansion(&expansion).expect("production expansion renders");
        let headings = output
            .lines()
            .filter_map(|line| line.strip_prefix("// === ")?.strip_suffix(" ==="))
            .filter(|heading| *heading != "counted escape hatches")
            .collect::<Vec<_>>();

        assert_eq!(EXPECTED_ITEM_KEYS.len(), 1_162);
        let prior = parse_plan09_expansion_inventory(PLAN09_EXPANSION);
        let task7 = parse_plan09_expansion_inventory(PLAN09_TASK7_EXPANSION);
        let fixture = parse_plan09_expansion_inventory(PLAN09_TASK8_EXPANSION);
        let live = expansion_inventory(&expansion);
        let live_digests = live
            .iter()
            .map(|(item, origins)| (item.clone(), sha256_hex(origins.join("\0").as_bytes())))
            .collect::<Vec<_>>();
        assert_eq!(prior.len(), 1_410);
        assert_eq!(task7.len(), 1_561);
        assert_eq!(fixture.len(), 1_627);
        assert_eq!(live_digests, fixture);
        let live_by_item = live_digests.iter().cloned().collect::<BTreeMap<_, _>>();
        let live_origins_by_item = live.iter().cloned().collect::<BTreeMap<_, _>>();
        let prior_by_item = prior.iter().cloned().collect::<BTreeMap<_, _>>();
        let task7_new_rows = task7
            .iter()
            .filter(|(item, _)| !prior_by_item.contains_key(item))
            .collect::<Vec<_>>();
        assert_eq!(task7_new_rows.len(), 151);
        assert_eq!(task7.len(), prior.len() + task7_new_rows.len());
        let task7_origin_names = BTreeSet::from([
            "vocab FiniteCopula",
            "vocab BareCopula",
            "vocab PredicativeAdjective",
            "vocab DamageKind",
            "vocab FaceOrientation",
            "morphology EnglishParticiple",
            "lexeme DamageParticipleLexeme",
            "lexeme MovementParticipleLexeme",
            "lexeme OrientationParticipleLexeme",
            "codec DamageParticipleHead",
            "codec MovementParticipleHead",
            "codec OrientationParticipleHead",
            "codec DeclaredTransitiveParticipleHead",
            "abstract sum PredicativeComplement",
            "abstract sum PredicativeStatus",
            "abstract sum PassivePredicate",
            "construction predicative_adjective",
            "construction predicative_color",
            "construction predicative_type",
            "construction predicative_status",
            "construction blocked_by_status",
            "construction predicative_ability",
            "construction predicative_power_toughness",
            "construction bare_copular_predicate",
            "construction change_state_predicate",
            "construction passive_damage_predicate",
            "construction passive_movement_predicate",
            "construction passive_orientation_predicate",
            "construction declared_transitive_passive_predicate",
            "construction bare_passive_predicate",
            "construction copular_clause",
            "construction passive_finite_clause",
            "construction deal_unspecified_damage",
            "construction gain_unspecified_life",
        ]);
        let task8_origin_names = BTreeSet::from([
            "vocab RequirementFrequency",
            "vocab ObjectOrder",
            "vocab PredicateDuration",
            "construction object_infinitive_predicate",
            "construction requirement_predicate",
            "construction as_though_predicate",
            "construction ordered_predicate",
            "construction counterfactual_status_clause",
            "construction purpose_predicate",
            "construction duration_predicate",
            "construction instead_predicate",
            "construction manner_predicate",
        ]);
        for task7_origin in &task7_origin_names {
            assert!(
                live.iter()
                    .any(|(_, origins)| origins.iter().any(|origin| origin == task7_origin)),
                "Task 7 origin is live: {task7_origin:?}",
            );
        }
        let task7_by_item = task7.iter().cloned().collect::<BTreeMap<_, _>>();
        let retained_task7_items = fixture
            .iter()
            .filter(|(item, _)| task7_by_item.contains_key(item))
            .map(|(item, _)| item)
            .collect::<Vec<_>>();
        assert_eq!(
            retained_task7_items,
            task7.iter().map(|(item, _)| item).collect::<Vec<_>>()
        );
        for (item, origins_digest) in &task7 {
            let live_origins = live_origins_by_item
                .get(item)
                .unwrap_or_else(|| panic!("Task 8 retains Task 7 generated item {item:?}"));
            let retained_origins = live_origins
                .iter()
                .filter(|origin| !task8_origin_names.contains(origin.as_str()))
                .cloned()
                .collect::<Vec<_>>();
            assert_eq!(
                sha256_hex(retained_origins.join("\0").as_bytes()),
                *origins_digest,
                "Task 8 retains the exact ordered Task 7 origins for {item:?}",
            );
        }
        let task8_new_rows = fixture
            .iter()
            .filter(|(item, _)| !task7_by_item.contains_key(item))
            .collect::<Vec<_>>();
        assert_eq!(task8_new_rows.len(), 66);
        for (item, origins_digest) in task8_new_rows {
            assert_eq!(live_by_item.get(item), Some(origins_digest));
        }
        for task8_origin in &task8_origin_names {
            assert!(
                live.iter()
                    .any(|(_, origins)| origins.iter().any(|origin| origin == task8_origin)),
                "Task 8 origin is live: {task8_origin:?}",
            );
        }
        for (item, origins_digest) in &prior {
            let live_origins = live_origins_by_item
                .get(item)
                .unwrap_or_else(|| panic!("Task 7 retains prior generated item {item:?}"));
            let retained_origins = live_origins
                .iter()
                .filter(|origin| {
                    !task7_origin_names.contains(origin.as_str())
                        && !task8_origin_names.contains(origin.as_str())
                })
                .cloned()
                .collect::<Vec<_>>();
            assert_eq!(
                sha256_hex(retained_origins.join("\0").as_bytes()),
                *origins_digest,
                "Task 7 retains the exact ordered prior origins for {item:?}",
            );
        }

        let retained_headings = headings
            .iter()
            .copied()
            .filter(|heading| EXPECTED_ITEM_KEYS.contains(heading))
            .collect::<Vec<_>>();
        assert!(
            retained_headings == EXPECTED_ITEM_KEYS,
            "retained pre-Task-5 item inventory first differs at {:?}",
            retained_headings
                .iter()
                .zip(EXPECTED_ITEM_KEYS)
                .enumerate()
                .find(|(_, (actual, expected))| actual != expected)
        );
        assert_eq!(headings.len() - retained_headings.len(), 465);
        for expected_key in headings {
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
            if let Some(prior_origins) = EXPECTED_ITEM_KEYS
                .contains(&expected_key)
                .then(|| expected_production_origins(expected_key))
                .flatten()
            {
                let retained_origins = actual_origins
                    .iter()
                    .copied()
                    .filter(|origin| prior_origins.contains(origin))
                    .collect::<Vec<_>>();
                assert_eq!(retained_origins, prior_origins, "{expected_key}");
            }
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
            "// morphology irregulars (7)\n\
             // - lexeme:CommonNoun/Ability\n\
             //   - plural = \"abilities\"\n\
             // - lexeme:CommonNoun/Die\n\
             //   - plural = \"dice\"\n\
             // - lexeme:VerbLexeme/Have\n\
             //   - third_person_singular = \"has\"\n\
             // - lexeme:VerbLexeme/Be\n\
             //   - bare = \"are\"\n\
             //   - third_person_singular = \"is\"\n\
             // - lexeme:CoreIntransitiveVerb/Die\n\
             //   - third_person_singular = \"dies\"\n\
             // - lexeme:DamageParticipleLexeme/Deal\n\
             //   - participle = \"dealt\"\n\
             // - lexeme:MovementParticipleLexeme/Put\n\
             //   - participle = \"put\"\n\
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
        assert_eq!(parsed.items.len(), 1_627);
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
