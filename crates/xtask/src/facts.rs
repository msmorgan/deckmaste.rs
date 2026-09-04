//! `cargo xtask facts` — the RON macro stubs are the workbench's label
//! vocabulary, so the Idris keyword facts table is generated from them
//! (`docs/decisions/workbench-ron-shaped-and-label-rulings.md` §2).
//!
//! Three subcommands:
//!  - `facts generate` — write `idris/src/Experimental/FactsGen.idr` from
//!    `plugins/builtin_v2/macros/stubs/keyword_abilities/*.ron` plus the
//!    gate-column overlay below.
//!  - `facts check` — regenerate in memory and fail if the committed module
//!    differs, so the generated file can't be hand-edited into drift.
//!  - `facts labels` — the two-way label-set report: every stub wants a
//!    table row and every table row wants a stub. Fails on any difference.

use std::fs;
use std::path::Path;
use std::path::PathBuf;

use anyhow::Context;
use clap::Args;
use clap::Subcommand;

#[derive(Debug, Args)]
pub struct FactsArgs {
    #[command(subcommand)]
    command: FactsCmd,
}

#[derive(Debug, Subcommand)]
enum FactsCmd {
    /// Write the generated Idris keyword facts module.
    Generate(PathArgs),
    /// Fail if the committed generated module differs from a fresh one.
    Check(PathArgs),
    /// Report labels present on only one of the stub/table sides.
    Labels(PathArgs),
}

#[derive(Debug, Args)]
pub struct PathArgs {
    /// Repository root. Defaults to the workspace this xtask was built in.
    #[arg(long = "root", value_name = "DIR")]
    root: Option<PathBuf>,
}

/// # Errors
/// If a stub or Idris source is unreadable, a stub's parameter signature is
/// outside the admitted vocabulary, or (for `check`/`labels`) the two sides
/// disagree.
pub fn run(args: &FactsArgs) -> anyhow::Result<()> {
    match &args.command {
        FactsCmd::Generate(a) => run_generate(&repo_root(a)),
        FactsCmd::Check(a) => run_check(&repo_root(a)),
        FactsCmd::Labels(a) => run_labels(&repo_root(a)),
    }
}

fn repo_root(args: &PathArgs) -> PathBuf {
    args.root
        .clone()
        .unwrap_or_else(|| Path::new(env!("CARGO_MANIFEST_DIR")).join("../.."))
}

const KEYWORD_STUBS: &str = "plugins/builtin_v2/macros/stubs/keyword_abilities";
const ACTION_STUBS: &str = "plugins/builtin_v2/macros/stubs/keyword_actions";
const COUNTER_STUBS: &str = "plugins/builtin_v2/macros/stubs/counter_kinds";
const WORDS: &str = "idris/src/Experimental/Words.idr";
const GENERATED: &str = "idris/src/Experimental/FactsGen.idr";

// ---------------------------------------------------------------------
// The keyword facts columns
// ---------------------------------------------------------------------

/// An admitted parameter shape, mirroring `KeywordShapes.KeywordParamShape`.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
enum Shape {
    Nothing,
    Cost,
    Quality,
    Subject,
    Number,
    Ability,
    CompoundQuality,
    CompoundNumber,
    DeckCondition,
}

impl Shape {
    fn idris(self) -> &'static str {
        match self {
            Shape::Nothing => "NoParam",
            Shape::Cost => "CostParam",
            Shape::Quality => "QualityParam",
            Shape::Subject => "SubjectParam",
            Shape::Number => "NumberParam",
            Shape::Ability => "AbilityParam",
            Shape::CompoundQuality => "CompoundParam QualityHead",
            Shape::CompoundNumber => "CompoundParam NumberHead",
            Shape::DeckCondition => "DeckConditionParam",
        }
    }

    /// The shape a stub's positional parameter-type names spell, or `None`
    /// when the signature is outside the workbench's admitted vocabulary.
    fn from_params(params: &[&str]) -> Option<Shape> {
        Some(match params {
            [] => Shape::Nothing,
            ["Cost"] => Shape::Cost,
            ["Quality"] => Shape::Quality,
            ["Subject"] => Shape::Subject,
            ["Amount"] => Shape::Number,
            ["Ability"] => Shape::Ability,
            ["Quality", "Cost"] => Shape::CompoundQuality,
            ["Amount", "Cost"] => Shape::CompoundNumber,
            ["Condition"] => Shape::DeckCondition,
            _ => return None,
        })
    }
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
enum Regime {
    AtCasting,
    AtResolution,
}

impl Regime {
    fn idris(self) -> &'static str {
        match self {
            Regime::AtCasting => "AtCasting",
            Regime::AtResolution => "AtResolution",
        }
    }
}

/// One keyword's Idris-gate columns — the facts a stub does not carry. The
/// stub determines the row's base parameter shape; `extra` adds the further
/// shapes the CR admits for the same keyword (hexproof from [quality]
/// [CR#702.11d], [type]cycling [cost] [CR#702.29e]) and stands alone for a
/// row whose stub is missing.
#[derive(Debug, Clone, Copy)]
#[expect(
    clippy::struct_excessive_bools,
    reason = "one field per Idris gate column, and those columns are booleans"
)]
struct Row {
    label: &'static str,
    extra: &'static [Shape],
    counter_eligible: bool,
    regime: Option<Regime>,
    on_permanent_card: bool,
    on_spell_card: bool,
    paid_cost: bool,
    bodied: bool,
    wants_modes: bool,
}

/// The column defaults every row updates: a paramless permanent-card keyword
/// with no stack regime and no gate flags.
const D: Row = Row {
    label: "",
    extra: &[],
    counter_eligible: false,
    regime: None,
    on_permanent_card: true,
    on_spell_card: false,
    paid_cost: false,
    bodied: false,
    wants_modes: false,
};

#[expect(
    clippy::too_many_lines,
    reason = "a data table: one line per keyword row"
)]
fn overlay() -> Vec<Row> {
    vec![
        Row {
            label: "Haste",
            counter_eligible: true,
            ..D
        },
        Row {
            label: "Flying",
            counter_eligible: true,
            ..D
        },
        Row {
            label: "Trample",
            counter_eligible: true,
            ..D
        },
        Row {
            label: "Vigilance",
            counter_eligible: true,
            ..D
        },
        Row {
            label: "Deathtouch",
            counter_eligible: true,
            regime: Some(Regime::AtResolution),
            ..D
        },
        Row {
            label: "DoubleStrike",
            counter_eligible: true,
            ..D
        },
        Row {
            label: "FirstStrike",
            counter_eligible: true,
            ..D
        },
        Row {
            label: "Reach",
            counter_eligible: true,
            ..D
        },
        Row {
            label: "Defender",
            ..D
        },
        Row {
            label: "Convoke",
            regime: Some(Regime::AtCasting),
            on_spell_card: true,
            ..D
        },
        Row {
            label: "Improvise",
            regime: Some(Regime::AtCasting),
            on_spell_card: true,
            ..D
        },
        Row {
            label: "Storm",
            regime: Some(Regime::AtCasting),
            on_spell_card: true,
            bodied: true,
            ..D
        },
        Row {
            label: "Lifelink",
            counter_eligible: true,
            regime: Some(Regime::AtResolution),
            ..D
        },
        Row {
            label: "Ward",
            paid_cost: true,
            bodied: true,
            ..D
        },
        Row {
            label: "Protection",
            ..D
        },
        Row {
            label: "Enchant",
            ..D
        },
        Row {
            label: "Equip",
            extra: &[Shape::CompoundQuality],
            paid_cost: true,
            ..D
        },
        Row {
            label: "Suspend",
            on_spell_card: true,
            paid_cost: true,
            ..D
        },
        Row {
            label: "Ascend",
            on_spell_card: true,
            ..D
        },
        Row {
            label: "Storied",
            ..D
        },
        Row {
            label: "Renown",
            bodied: true,
            ..D
        },
        Row {
            label: "Indestructible",
            counter_eligible: true,
            ..D
        },
        Row {
            label: "Flash",
            regime: Some(Regime::AtCasting),
            on_spell_card: true,
            ..D
        },
        Row {
            label: "Kicker",
            regime: Some(Regime::AtCasting),
            on_spell_card: true,
            paid_cost: true,
            ..D
        },
        Row {
            label: "Multikicker",
            extra: &[Shape::Cost],
            regime: Some(Regime::AtCasting),
            on_spell_card: true,
            paid_cost: true,
            ..D
        },
        Row {
            label: "CumulativeUpkeep",
            paid_cost: true,
            bodied: true,
            ..D
        },
        Row {
            label: "Echo",
            paid_cost: true,
            bodied: true,
            ..D
        },
        Row {
            label: "Hexproof",
            extra: &[Shape::Quality],
            counter_eligible: true,
            ..D
        },
        Row {
            label: "Menace",
            counter_eligible: true,
            ..D
        },
        Row {
            label: "Skulk",
            ..D
        },
        Row {
            label: "Bushido",
            bodied: true,
            ..D
        },
        Row {
            label: "Unearth",
            paid_cost: true,
            ..D
        },
        Row {
            label: "Flashback",
            on_permanent_card: false,
            on_spell_card: true,
            paid_cost: true,
            ..D
        },
        Row {
            label: "Dredge",
            on_spell_card: true,
            ..D
        },
        Row {
            label: "Retrace",
            on_spell_card: true,
            ..D
        },
        Row {
            label: "Cycling",
            extra: &[Shape::CompoundQuality],
            on_spell_card: true,
            paid_cost: true,
            ..D
        },
        Row {
            label: "Ninjutsu",
            paid_cost: true,
            ..D
        },
        Row {
            label: "Miracle",
            on_spell_card: true,
            paid_cost: true,
            ..D
        },
        Row {
            label: "Warp",
            regime: Some(Regime::AtCasting),
            on_spell_card: true,
            paid_cost: true,
            ..D
        },
        Row {
            label: "Afterlife",
            bodied: true,
            ..D
        },
        Row {
            label: "Boast",
            ..D
        },
        Row {
            label: "Exhaust",
            ..D
        },
        Row {
            label: "PowerUp",
            ..D
        },
        Row {
            label: "Affinity",
            regime: Some(Regime::AtCasting),
            on_spell_card: true,
            ..D
        },
        Row {
            label: "Annihilator",
            bodied: true,
            ..D
        },
        Row { label: "Fear", ..D },
        Row {
            label: "Shroud",
            ..D
        },
        Row {
            label: "Banding",
            ..D
        },
        Row {
            label: "BandsWithOther",
            extra: &[Shape::Quality],
            ..D
        },
        Row {
            label: "Landwalk",
            ..D
        },
        Row {
            label: "Changeling",
            on_spell_card: true,
            ..D
        },
        Row { label: "Crew", ..D },
        Row {
            label: "Saddle",
            ..D
        },
        Row {
            label: "PartnerWith",
            extra: &[Shape::Quality],
            ..D
        },
        Row {
            label: "Emerge",
            regime: Some(Regime::AtCasting),
            on_spell_card: true,
            paid_cost: true,
            ..D
        },
        Row {
            label: "Craft",
            paid_cost: true,
            ..D
        },
        Row {
            label: "Madness",
            on_spell_card: true,
            paid_cost: true,
            ..D
        },
        Row {
            label: "Prowl",
            regime: Some(Regime::AtCasting),
            on_spell_card: true,
            paid_cost: true,
            ..D
        },
        Row {
            label: "Surge",
            regime: Some(Regime::AtCasting),
            on_spell_card: true,
            paid_cost: true,
            ..D
        },
        Row {
            label: "Spectacle",
            regime: Some(Regime::AtCasting),
            on_spell_card: true,
            paid_cost: true,
            ..D
        },
        Row {
            label: "Freerunning",
            regime: Some(Regime::AtCasting),
            on_spell_card: true,
            paid_cost: true,
            ..D
        },
        Row {
            label: "Sneak",
            regime: Some(Regime::AtCasting),
            on_spell_card: true,
            paid_cost: true,
            ..D
        },
        Row {
            label: "Mayhem",
            on_spell_card: true,
            paid_cost: true,
            ..D
        },
        Row {
            label: "Disturb",
            on_spell_card: true,
            paid_cost: true,
            ..D
        },
        Row {
            label: "Morph",
            on_spell_card: true,
            paid_cost: true,
            ..D
        },
        Row {
            label: "Entwine",
            regime: Some(Regime::AtCasting),
            on_spell_card: true,
            paid_cost: true,
            wants_modes: true,
            ..D
        },
        Row {
            label: "Escalate",
            regime: Some(Regime::AtCasting),
            on_spell_card: true,
            paid_cost: true,
            wants_modes: true,
            ..D
        },
        Row {
            label: "Fuse",
            regime: Some(Regime::AtCasting),
            on_permanent_card: false,
            on_spell_card: true,
            ..D
        },
        Row {
            label: "Escape",
            on_spell_card: true,
            paid_cost: true,
            ..D
        },
        Row {
            label: "Foretell",
            on_spell_card: true,
            paid_cost: true,
            ..D
        },
        Row {
            label: "Bestow",
            paid_cost: true,
            ..D
        },
        Row {
            label: "Disguise",
            paid_cost: true,
            ..D
        },
        Row {
            label: "Mutate",
            regime: Some(Regime::AtCasting),
            paid_cost: true,
            ..D
        },
        Row {
            label: "Overload",
            regime: Some(Regime::AtCasting),
            on_permanent_card: false,
            on_spell_card: true,
            paid_cost: true,
            ..D
        },
        Row {
            label: "Dash",
            regime: Some(Regime::AtCasting),
            paid_cost: true,
            ..D
        },
        Row {
            label: "Evoke",
            paid_cost: true,
            ..D
        },
        Row {
            label: "Blitz",
            regime: Some(Regime::AtCasting),
            paid_cost: true,
            ..D
        },
        Row {
            label: "Cleave",
            regime: Some(Regime::AtCasting),
            on_permanent_card: false,
            on_spell_card: true,
            paid_cost: true,
            ..D
        },
        Row {
            label: "Harmonize",
            on_permanent_card: false,
            on_spell_card: true,
            paid_cost: true,
            ..D
        },
        Row {
            label: "Impending",
            regime: Some(Regime::AtCasting),
            paid_cost: true,
            ..D
        },
        Row {
            label: "Awaken",
            regime: Some(Regime::AtCasting),
            on_permanent_card: false,
            on_spell_card: true,
            paid_cost: true,
            ..D
        },
        Row {
            label: "Buyback",
            regime: Some(Regime::AtCasting),
            on_permanent_card: false,
            on_spell_card: true,
            paid_cost: true,
            ..D
        },
        Row {
            label: "Casualty",
            regime: Some(Regime::AtCasting),
            on_spell_card: true,
            paid_cost: true,
            ..D
        },
        Row {
            label: "Squad",
            regime: Some(Regime::AtCasting),
            paid_cost: true,
            ..D
        },
        Row {
            label: "Offspring",
            regime: Some(Regime::AtCasting),
            paid_cost: true,
            ..D
        },
        Row {
            label: "Gift",
            regime: Some(Regime::AtCasting),
            on_spell_card: true,
            paid_cost: true,
            ..D
        },
        Row {
            label: "Replicate",
            regime: Some(Regime::AtCasting),
            on_permanent_card: false,
            on_spell_card: true,
            paid_cost: true,
            ..D
        },
        // Rows the workbench was missing, added from their stubs. The gate
        // columns follow the CR entry: delve is a static ability functioning
        // while the spell is on the stack [CR#702.66a] (the convoke shape);
        // cascade [CR#702.85a] and prowess [CR#702.108a] are triggered
        // abilities with a quoted cast trigger, so they are bodied at
        // casting; exalted [CR#702.83a] is triggered off an attack, so its
        // body carries no stack regime; decayed [CR#702.147a] is a static
        // ability AND a triggered one, so it is not bodied; infect
        // [CR#702.90a] modifies damage, like deathtouch and lifelink; split
        // second [CR#702.61a] is a spell-card-only static; phasing
        // [CR#702.26a] and shadow [CR#702.28a] are permanent statics.
        // Decayed, exalted and shadow are keyword counters [CR#122.1b].
        Row {
            label: "Delve",
            regime: Some(Regime::AtCasting),
            on_spell_card: true,
            ..D
        },
        Row {
            label: "Infect",
            regime: Some(Regime::AtResolution),
            ..D
        },
        Row {
            label: "Cascade",
            regime: Some(Regime::AtCasting),
            on_spell_card: true,
            bodied: true,
            ..D
        },
        Row {
            label: "Prowess",
            regime: Some(Regime::AtCasting),
            bodied: true,
            ..D
        },
        Row {
            label: "SplitSecond",
            on_permanent_card: false,
            on_spell_card: true,
            ..D
        },
        Row {
            label: "Phasing",
            ..D
        },
        Row {
            label: "Decayed",
            counter_eligible: true,
            ..D
        },
        Row {
            label: "Exalted",
            counter_eligible: true,
            bodied: true,
            ..D
        },
        Row {
            label: "Shadow",
            counter_eligible: true,
            ..D
        },
        Row {
            label: "Companion",
            ..D
        },
    ]
}

// ---------------------------------------------------------------------
// Reading the stubs
// ---------------------------------------------------------------------

/// The `.ron` file stems under `dir`, sorted.
fn stub_names(dir: &Path) -> anyhow::Result<Vec<String>> {
    let mut names = Vec::new();
    for entry in fs::read_dir(dir).with_context(|| format!("reading dir {}", dir.display()))? {
        let path = entry
            .with_context(|| format!("reading an entry of {}", dir.display()))?
            .path();
        if path.extension().is_some_and(|ext| ext == "ron")
            && let Some(stem) = path.file_stem().and_then(|s| s.to_str())
        {
            names.push(stem.to_owned());
        }
    }
    names.sort();
    Ok(names)
}

/// The value of a stub's top-level `field: …,` entry, verbatim and trimmed.
/// The stubs are one macro call per file, written at a fixed indent, so the
/// field's own line is the whole value for every field this command reads.
fn stub_field<'a>(src: &'a str, field: &str) -> Option<&'a str> {
    let head = format!("    {field}: ");
    src.lines()
        .find_map(|line| line.strip_prefix(&head))
        .map(|rest| rest.trim_end().trim_end_matches(','))
}

/// A keyword stub's declared parameter shape.
fn stub_shape(dir: &Path, name: &str) -> anyhow::Result<Option<Shape>> {
    let path = dir.join(format!("{name}.ron"));
    if !path.exists() {
        return Ok(None);
    }
    let src = fs::read_to_string(&path).with_context(|| format!("reading {}", path.display()))?;
    let raw = stub_field(&src, "params").unwrap_or("[]");
    let inner = raw
        .strip_prefix('[')
        .and_then(|r| r.strip_suffix(']'))
        .with_context(|| format!("{name}: `params` is not a list: {raw}"))?;
    let params: Vec<&str> = inner
        .split(',')
        .map(str::trim)
        .filter(|p| !p.is_empty())
        .collect();
    let shape = Shape::from_params(&params).with_context(|| {
        format!("{name}: parameter signature outside the workbench vocabulary: {raw}")
    })?;
    Ok(Some(shape))
}

// ---------------------------------------------------------------------
// `facts generate` / `facts check`
// ---------------------------------------------------------------------

/// The generated module's text: one header line, then the rows as named-field
/// record updates over `defaultKeywordFacts`, so adding a column never
/// renumbers a row.
fn render(root: &Path) -> anyhow::Result<String> {
    let dir = root.join(KEYWORD_STUBS);
    let mut out = String::new();
    out.push_str(
        "-- Generated by `cargo xtask facts generate` from \
         plugins/builtin_v2/macros/stubs/keyword_abilities/*.ron and xtask's gate-column \
         overlay; edit those, never this file.\n",
    );
    out.push_str("module Experimental.FactsGen\n\n");
    out.push_str("import public Experimental.KeywordShapes\n\n");
    out.push_str("%default total\n\n");
    out.push_str("public export\nkeywordFacts : List KeywordFacts\nkeywordFacts =\n");

    for (i, row) in overlay().iter().enumerate() {
        let mut shapes: Vec<Shape> = stub_shape(&dir, row.label)?.into_iter().collect();
        for extra in row.extra {
            if !shapes.contains(extra) {
                shapes.push(*extra);
            }
        }
        anyhow::ensure!(
            !shapes.is_empty(),
            "{}: no stub and no overlay parameter shape",
            row.label
        );
        let shapes = shapes
            .iter()
            .map(|s| s.idris().to_owned())
            .collect::<Vec<_>>()
            .join(", ");

        let mut fields = vec![
            format!("word := \"{}\"", row.label),
            format!("paramShapes := [{shapes}]"),
        ];
        if row.counter_eligible {
            fields.push("counterEligible := True".to_owned());
        }
        if let Some(regime) = row.regime {
            fields.push(format!("regime := Just {}", regime.idris()));
        }
        if !row.on_permanent_card {
            fields.push("onPermanentCard := False".to_owned());
        }
        if row.on_spell_card {
            fields.push("onSpellCard := True".to_owned());
        }
        if row.paid_cost {
            fields.push("paidCost := True".to_owned());
        }
        if row.bodied {
            fields.push("bodied := True".to_owned());
        }
        if row.wants_modes {
            fields.push("wantsModes := True".to_owned());
        }

        let lead = if i == 0 { "  [" } else { "  ," };
        out.push_str(lead);
        out.push_str(" { ");
        out.push_str(&fields.join(", "));
        out.push_str(" } defaultKeywordFacts\n");
    }
    out.push_str("  ]\n");
    Ok(out)
}

fn run_generate(root: &Path) -> anyhow::Result<()> {
    let path = root.join(GENERATED);
    let text = render(root)?;
    fs::write(&path, &text).with_context(|| format!("writing {}", path.display()))?;
    println!(
        "wrote {} ({} rows)",
        path.display(),
        text.lines().filter(|l| l.contains(":=")).count()
    );
    Ok(())
}

fn run_check(root: &Path) -> anyhow::Result<()> {
    let path = root.join(GENERATED);
    let fresh = render(root)?;
    let committed =
        fs::read_to_string(&path).with_context(|| format!("reading {}", path.display()))?;
    anyhow::ensure!(
        fresh == committed,
        "{} is stale: re-run `cargo xtask facts generate`",
        path.display()
    );
    println!("{} is up to date", path.display());
    Ok(())
}

// ---------------------------------------------------------------------
// `facts labels`
// ---------------------------------------------------------------------

/// A label reduced to its comparable core: the workbench spells a multi-word
/// label with spaces ("The Ring Tempts You") where the stub file names it in
/// one word, and a counter's stub gives its printed spelling ("double
/// strike") where the table gives the keyword (`DoubleStrike`).
fn normalize(label: &str) -> String {
    label
        .chars()
        .filter(char::is_ascii_alphanumeric)
        .flat_map(char::to_lowercase)
        .collect()
}

/// Every label captured by `MkActFacts "…"`-style rows in `Words.idr`.
fn table_labels(src: &str, ctor: &str) -> Vec<String> {
    let head = format!("{ctor} \"");
    src.lines()
        .filter_map(|line| {
            let rest = line.split_once(&head)?.1;
            let (label, _) = rest.split_once('"')?;
            Some(label.to_owned())
        })
        .collect()
}

/// One table's two-way difference, printed. Returns whether it matched.
fn report(table: &str, stubs: &[String], rows: &[String]) -> bool {
    let stub_keys: Vec<String> = stubs.iter().map(|s| normalize(s)).collect();
    let row_keys: Vec<String> = rows.iter().map(|s| normalize(s)).collect();
    let missing_rows: Vec<&String> = stubs
        .iter()
        .zip(&stub_keys)
        .filter(|(_, k)| !row_keys.contains(k))
        .map(|(s, _)| s)
        .collect();
    let missing_stubs: Vec<&String> = rows
        .iter()
        .zip(&row_keys)
        .filter(|(_, k)| !stub_keys.contains(k))
        .map(|(s, _)| s)
        .collect();
    println!(
        "{table}: {} stubs, {} rows, {} stubs without a row, {} rows without a stub",
        stubs.len(),
        rows.len(),
        missing_rows.len(),
        missing_stubs.len()
    );
    for label in &missing_rows {
        println!("  stub without a row: {label}");
    }
    for label in &missing_stubs {
        println!("  row without a stub: {label}");
    }
    missing_rows.is_empty() && missing_stubs.is_empty()
}

fn run_labels(root: &Path) -> anyhow::Result<()> {
    let words_path = root.join(WORDS);
    let words = fs::read_to_string(&words_path)
        .with_context(|| format!("reading {}", words_path.display()))?;

    let keyword_rows: Vec<String> = overlay().iter().map(|r| r.label.to_owned()).collect();
    let mut ok = report(
        "keyword abilities",
        &stub_names(&root.join(KEYWORD_STUBS))?,
        &keyword_rows,
    );

    ok &= report(
        "keyword actions",
        &stub_names(&root.join(ACTION_STUBS))?,
        &table_labels(&words, "MkActFacts"),
    );

    // A counter stub names its printed spelling; the workbench spells the
    // +1/+1 and -1/-1 counters as `BoostCounter`, not as a label, and a
    // keyword counter [CR#122.1b] as the eligible keyword's own row.
    let counter_dir = root.join(COUNTER_STUBS);
    let mut counter_stubs = Vec::new();
    for name in stub_names(&counter_dir)? {
        if name == "P1P1Counter" || name == "M1M1Counter" {
            continue;
        }
        let path = counter_dir.join(format!("{name}.ron"));
        let src =
            fs::read_to_string(&path).with_context(|| format!("reading {}", path.display()))?;
        let spelling =
            stub_field(&src, "spelling").with_context(|| format!("{name}: no `spelling` field"))?;
        counter_stubs.push(spelling.trim_matches('"').to_owned());
    }
    let mut counter_rows = table_labels(&words, "MkCounterFacts");
    counter_rows.extend(
        overlay()
            .iter()
            .filter(|r| r.counter_eligible)
            .map(|r| r.label.to_owned()),
    );
    ok &= report("counter kinds", &counter_stubs, &counter_rows);

    anyhow::ensure!(ok, "the stub and table label sets differ");
    Ok(())
}

#[cfg(test)]
mod tests {
    use super::*;

    fn root() -> PathBuf {
        Path::new(env!("CARGO_MANIFEST_DIR")).join("../..")
    }

    #[test]
    fn stub_signatures_map_to_workbench_shapes() {
        assert_eq!(Shape::from_params(&[]), Some(Shape::Nothing));
        assert_eq!(Shape::from_params(&["Cost"]), Some(Shape::Cost));
        assert_eq!(
            Shape::from_params(&["Amount", "Cost"]),
            Some(Shape::CompoundNumber)
        );
        assert_eq!(
            Shape::from_params(&["Quality", "Cost"]),
            Some(Shape::CompoundQuality)
        );
        assert_eq!(Shape::from_params(&["Cost", "Power", "Toughness"]), None);
    }

    #[test]
    fn overlay_labels_are_distinct() {
        let mut labels: Vec<&str> = overlay().iter().map(|r| r.label).collect();
        let count = labels.len();
        labels.sort_unstable();
        labels.dedup();
        assert_eq!(labels.len(), count, "a keyword label is listed twice");
    }

    #[test]
    fn hexproof_and_cycling_admit_their_cr_variants() {
        let text = render(&root()).expect("rendering the keyword facts module");
        assert!(text.contains(r#"{ word := "Hexproof", paramShapes := [NoParam, QualityParam]"#));
        assert!(text.contains(
            r#"{ word := "Cycling", paramShapes := [CostParam, CompoundParam QualityHead]"#
        ));
    }

    #[test]
    fn every_row_reaches_a_shape() {
        let text = render(&root()).expect("rendering the keyword facts module");
        assert_eq!(
            text.matches("paramShapes := []").count(),
            0,
            "a row rendered with no admitted parameter shape"
        );
        assert_eq!(text.matches("paramShapes := ").count(), overlay().len());
    }

    #[test]
    fn the_committed_module_matches_the_stubs() {
        run_check(&root()).expect("`cargo xtask facts check` must pass on a clean tree");
    }

    #[test]
    fn labels_normalize_across_the_two_spellings() {
        assert_eq!(
            normalize("The Ring Tempts You"),
            normalize("TheRingTemptsYou")
        );
        assert_eq!(normalize("double strike"), normalize("DoubleStrike"));
        assert_ne!(normalize("Ward"), normalize("Shroud"));
    }
}
