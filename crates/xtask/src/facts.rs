//! Generate workbench registry facts from the builtin declarations and
//! checker-column overlays. Lean receives actions, keywords, counters,
//! designations, and frame subtypes; the reference Idris module retains its
//! keyword table.
//!
//! `facts generate` writes both modules; `facts check` rejects drift in either
//! one. `facts labels` retains the reference Idris label-set audit and its
//! scope exceptions.

mod action_overlay;
mod lean;

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
    /// Write the generated Lean and Idris facts modules.
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

const KEYWORD_STUBS: &str = "plugins_v2/builtin/macros/stubs/keyword_abilities";
const ACTION_STUBS: &str = "plugins_v2/builtin/macros/stubs/keyword_actions";
const COUNTER_STUBS: &str = "plugins_v2/builtin/macros/stubs/counter_kinds";
const DESIGNATION_STUBS: &str = "plugins_v2/builtin/macros/stubs/designations";
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

/// The general category an ability is written in [CR#113.3]. A keyword's
/// definition — the body its `Ability.keyword` term may carry [CR#702.1] — is
/// an ability of one of these.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
enum Category {
    Static,
    Triggered,
    Activated,
}

impl Category {
    fn lean(self) -> &'static str {
        match self {
            Category::Static => ".static",
            Category::Triggered => ".triggered",
            Category::Activated => ".activated",
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
    functions_on_stack: bool,
    on_permanent_card: bool,
    on_spell_card: bool,
    paid_cost: bool,
    /// The categories this keyword's definition is written in, from its own
    /// [CR#702] entry. Empty is a keyword whose definition the workbench has
    /// not declared yet, and whose term therefore carries no body.
    definition: &'static [Category],
    wants_modes: bool,
}

/// The column defaults every row updates: a paramless permanent-card keyword
/// with no stack regime and no gate flags.
const D: Row = Row {
    label: "",
    extra: &[],
    counter_eligible: false,
    regime: None,
    functions_on_stack: false,
    on_permanent_card: true,
    on_spell_card: false,
    paid_cost: false,
    definition: &[],
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
            // "This creature can't be blocked except by creatures with flying
            // and/or reach" is one static ability [CR#702.9b].
            definition: &[Category::Static],
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
            definition: &[Category::Static],
            ..D
        },
        Row {
            label: "Convoke",
            regime: Some(Regime::AtCasting),
            functions_on_stack: true,
            on_spell_card: true,
            ..D
        },
        Row {
            label: "Improvise",
            regime: Some(Regime::AtCasting),
            functions_on_stack: true,
            on_spell_card: true,
            ..D
        },
        Row {
            label: "Storm",
            regime: Some(Regime::AtCasting),
            functions_on_stack: true,
            on_spell_card: true,
            definition: &[Category::Triggered],
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
            definition: &[Category::Triggered],
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
            definition: &[Category::Activated],
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
            definition: &[Category::Triggered],
            ..D
        },
        Row {
            label: "Indestructible",
            counter_eligible: true,
            definition: &[],
            ..D
        },
        Row {
            label: "Flash",
            regime: Some(Regime::AtCasting),
            functions_on_stack: true,
            on_spell_card: true,
            ..D
        },
        Row {
            label: "Kicker",
            regime: Some(Regime::AtCasting),
            functions_on_stack: true,
            on_spell_card: true,
            paid_cost: true,
            definition: &[Category::Static],
            ..D
        },
        Row {
            label: "Multikicker",
            extra: &[Shape::Cost],
            regime: Some(Regime::AtCasting),
            functions_on_stack: true,
            on_spell_card: true,
            paid_cost: true,
            ..D
        },
        Row {
            label: "CumulativeUpkeep",
            paid_cost: true,
            definition: &[Category::Triggered],
            ..D
        },
        Row {
            label: "Echo",
            paid_cost: true,
            definition: &[Category::Triggered],
            ..D
        },
        Row {
            label: "Hexproof",
            extra: &[Shape::Quality],
            counter_eligible: true,
            definition: &[Category::Static],
            ..D
        },
        Row {
            label: "Menace",
            counter_eligible: true,
            definition: &[Category::Static],
            ..D
        },
        Row {
            label: "Skulk",
            definition: &[Category::Static],
            ..D
        },
        Row {
            label: "Bushido",
            definition: &[Category::Triggered],
            ..D
        },
        Row {
            label: "Unearth",
            paid_cost: true,
            definition: &[Category::Activated],
            ..D
        },
        Row {
            label: "Flashback",
            on_permanent_card: false,
            on_spell_card: true,
            paid_cost: true,
            definition: &[Category::Static, Category::Static],
            ..D
        },
        Row {
            label: "Dredge",
            on_spell_card: true,
            definition: &[Category::Static],
            ..D
        },
        Row {
            label: "Retrace",
            on_spell_card: true,
            definition: &[Category::Static],
            ..D
        },
        Row {
            label: "Cycling",
            extra: &[Shape::CompoundQuality],
            on_spell_card: true,
            paid_cost: true,
            // "[Cost], Discard this card: Draw a card" is one activated
            // ability [CR#702.29a].
            definition: &[Category::Activated],
            ..D
        },
        Row {
            label: "Ninjutsu",
            paid_cost: true,
            definition: &[Category::Activated],
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
            functions_on_stack: true,
            on_spell_card: true,
            paid_cost: true,
            ..D
        },
        Row {
            label: "Afterlife",
            definition: &[Category::Triggered],
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
            functions_on_stack: true,
            on_spell_card: true,
            definition: &[Category::Static],
            ..D
        },
        Row {
            label: "Annihilator",
            definition: &[Category::Triggered],
            ..D
        },
        Row {
            label: "Fear",
            definition: &[Category::Static],
            ..D
        },
        Row {
            label: "Shroud",
            definition: &[Category::Static],
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
            definition: &[Category::Static],
            ..D
        },
        Row {
            label: "Changeling",
            on_spell_card: true,
            definition: &[Category::Static],
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
            functions_on_stack: true,
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
            definition: &[Category::Static, Category::Triggered],
            ..D
        },
        Row {
            label: "Prowl",
            regime: Some(Regime::AtCasting),
            functions_on_stack: true,
            on_spell_card: true,
            paid_cost: true,
            ..D
        },
        Row {
            label: "Surge",
            regime: Some(Regime::AtCasting),
            functions_on_stack: true,
            on_spell_card: true,
            paid_cost: true,
            definition: &[Category::Static],
            ..D
        },
        Row {
            label: "Spectacle",
            regime: Some(Regime::AtCasting),
            functions_on_stack: true,
            on_spell_card: true,
            paid_cost: true,
            definition: &[Category::Static],
            ..D
        },
        Row {
            label: "Freerunning",
            regime: Some(Regime::AtCasting),
            functions_on_stack: true,
            on_spell_card: true,
            paid_cost: true,
            ..D
        },
        Row {
            label: "Sneak",
            regime: Some(Regime::AtCasting),
            functions_on_stack: true,
            on_spell_card: true,
            paid_cost: true,
            ..D
        },
        Row {
            label: "Mayhem",
            on_spell_card: true,
            paid_cost: true,
            definition: &[Category::Static],
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
            functions_on_stack: true,
            on_spell_card: true,
            paid_cost: true,
            wants_modes: true,
            ..D
        },
        Row {
            label: "Escalate",
            regime: Some(Regime::AtCasting),
            functions_on_stack: true,
            on_spell_card: true,
            paid_cost: true,
            wants_modes: true,
            ..D
        },
        Row {
            label: "Fuse",
            regime: Some(Regime::AtCasting),
            functions_on_stack: true,
            on_permanent_card: false,
            on_spell_card: true,
            ..D
        },
        Row {
            label: "Escape",
            on_spell_card: true,
            paid_cost: true,
            definition: &[Category::Static],
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
            definition: &[Category::Static],
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
            functions_on_stack: true,
            paid_cost: true,
            ..D
        },
        Row {
            label: "Overload",
            regime: Some(Regime::AtCasting),
            functions_on_stack: true,
            on_permanent_card: false,
            on_spell_card: true,
            paid_cost: true,
            ..D
        },
        Row {
            label: "Dash",
            regime: Some(Regime::AtCasting),
            functions_on_stack: true,
            paid_cost: true,
            ..D
        },
        Row {
            label: "Evoke",
            paid_cost: true,
            definition: &[Category::Static, Category::Triggered],
            ..D
        },
        Row {
            label: "Blitz",
            regime: Some(Regime::AtCasting),
            functions_on_stack: true,
            paid_cost: true,
            ..D
        },
        Row {
            label: "Cleave",
            regime: Some(Regime::AtCasting),
            functions_on_stack: true,
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
            functions_on_stack: true,
            paid_cost: true,
            definition: &[
                Category::Static,
                Category::Static,
                Category::Static,
                Category::Triggered,
            ],
            ..D
        },
        Row {
            label: "Awaken",
            regime: Some(Regime::AtCasting),
            functions_on_stack: true,
            on_permanent_card: false,
            on_spell_card: true,
            paid_cost: true,
            ..D
        },
        Row {
            label: "Buyback",
            regime: Some(Regime::AtCasting),
            functions_on_stack: true,
            on_permanent_card: false,
            on_spell_card: true,
            paid_cost: true,
            definition: &[Category::Static, Category::Static],
            ..D
        },
        Row {
            label: "Casualty",
            regime: Some(Regime::AtCasting),
            functions_on_stack: true,
            on_spell_card: true,
            paid_cost: true,
            ..D
        },
        Row {
            label: "Squad",
            regime: Some(Regime::AtCasting),
            functions_on_stack: true,
            paid_cost: true,
            ..D
        },
        Row {
            label: "Offspring",
            regime: Some(Regime::AtCasting),
            functions_on_stack: true,
            paid_cost: true,
            ..D
        },
        Row {
            label: "Gift",
            regime: Some(Regime::AtCasting),
            functions_on_stack: true,
            on_spell_card: true,
            paid_cost: true,
            ..D
        },
        Row {
            label: "Replicate",
            regime: Some(Regime::AtCasting),
            functions_on_stack: true,
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
        // casting, though only cascade's functions on the stack [CR#113.6];
        // exalted [CR#702.83a] is triggered off an attack, so its
        // body carries no stack regime; decayed [CR#702.147a] is a static
        // ability AND a triggered one, which one `body` slot cannot hold, so
        // its definition stays undeclared; infect
        // [CR#702.90a] modifies damage, like deathtouch and lifelink; split
        // second [CR#702.61a] is a spell-card-only static; phasing
        // [CR#702.26a] and shadow [CR#702.28a] are permanent statics.
        // Decayed, exalted and shadow are keyword counters [CR#122.1b].
        Row {
            label: "Delve",
            regime: Some(Regime::AtCasting),
            functions_on_stack: true,
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
            functions_on_stack: true,
            on_spell_card: true,
            definition: &[Category::Triggered],
            ..D
        },
        Row {
            label: "Prowess",
            regime: Some(Regime::AtCasting),
            definition: &[Category::Triggered],
            ..D
        },
        Row {
            label: "SplitSecond",
            functions_on_stack: true,
            on_permanent_card: false,
            on_spell_card: true,
            definition: &[Category::Static],
            ..D
        },
        Row {
            label: "Phasing",
            ..D
        },
        Row {
            label: "Decayed",
            counter_eligible: true,
            definition: &[Category::Static, Category::Triggered],
            ..D
        },
        Row {
            label: "Exalted",
            counter_eligible: true,
            definition: &[Category::Triggered],
            ..D
        },
        Row {
            label: "Shadow",
            counter_eligible: true,
            definition: &[Category::Static, Category::Static],
            ..D
        },
        Row {
            label: "Companion",
            ..D
        },
        // The rowless stubs, rowed. Each gate column follows the keyword's own
        // CR entry [CR#702]: `definition` `Triggered` where the entry defines
        // one triggered ability with a quoted expansion, `paidCost` where it names a
        // "[keyword] cost" [CR#702.1a], `regime` `AtCasting` where the body
        // keys on a spell cast and `AtResolution` where it modifies the damage
        // its source deals [CR#120.3], `functionsOnStack` where the ability
        // functions only while the object is on the stack [CR#113.6],
        // `onPermanentCard` /
        // `onSpellCard` from the entry's own card-type wording and the printed
        // corpus, `wantsModes` for the modal-spell keywords [CR#702.172a,702.183a].
        // No new row is a keyword counter [CR#122.1b].
        Row {
            label: "Absorb",
            definition: &[Category::Static],
            ..D
        },
        Row {
            label: "Afflict",
            definition: &[Category::Triggered],
            ..D
        },
        Row {
            label: "Aftermath",
            on_permanent_card: false,
            on_spell_card: true,
            definition: &[Category::Static],
            ..D
        },
        Row {
            label: "Amplify",
            ..D
        },
        Row {
            label: "Assist",
            regime: Some(Regime::AtCasting),
            functions_on_stack: true,
            on_spell_card: true,
            ..D
        },
        Row {
            label: "AuraSwap",
            paid_cost: true,
            definition: &[Category::Activated],
            ..D
        },
        Row {
            label: "Backup",
            definition: &[Category::Triggered],
            ..D
        },
        Row {
            label: "Bargain",
            regime: Some(Regime::AtCasting),
            functions_on_stack: true,
            on_spell_card: true,
            // "A spell is 'bargained' if its controller sacrificed a permanent as
            // it was cast" [CR#702.166b] — a cost a later clause reads back.
            paid_cost: true,
            definition: &[Category::Static],
            ..D
        },
        Row {
            label: "BattleCry",
            definition: &[Category::Triggered],
            ..D
        },
        Row {
            label: "Bloodthirst",
            definition: &[Category::Static],
            ..D
        },
        Row {
            label: "Champion",
            ..D
        },
        Row {
            label: "Cipher",
            regime: Some(Regime::AtCasting),
            functions_on_stack: true,
            on_permanent_card: false,
            on_spell_card: true,
            ..D
        },
        Row {
            label: "Compleated",
            ..D
        },
        Row {
            label: "Conspire",
            regime: Some(Regime::AtCasting),
            functions_on_stack: true,
            on_spell_card: true,
            paid_cost: true,
            ..D
        },
        Row {
            label: "Daybound",
            ..D
        },
        Row {
            label: "Demonstrate",
            regime: Some(Regime::AtCasting),
            functions_on_stack: true,
            on_spell_card: true,
            definition: &[Category::Triggered],
            ..D
        },
        Row {
            label: "Dethrone",
            definition: &[Category::Triggered],
            ..D
        },
        Row {
            label: "Devoid",
            on_spell_card: true,
            definition: &[Category::Static],
            ..D
        },
        Row {
            label: "Devour",
            ..D
        },
        Row {
            label: "Embalm",
            paid_cost: true,
            definition: &[Category::Activated],
            ..D
        },
        Row {
            label: "Encore",
            paid_cost: true,
            ..D
        },
        Row {
            label: "Enlist",
            ..D
        },
        Row {
            label: "Epic",
            on_spell_card: true,
            ..D
        },
        Row {
            label: "Eternalize",
            paid_cost: true,
            definition: &[Category::Activated],
            ..D
        },
        Row {
            label: "Evolve",
            definition: &[Category::Triggered],
            ..D
        },
        Row {
            label: "Exploit",
            definition: &[Category::Triggered],
            ..D
        },
        Row {
            label: "Extort",
            regime: Some(Regime::AtCasting),
            definition: &[Category::Triggered],
            ..D
        },
        Row {
            label: "Fabricate",
            definition: &[Category::Triggered],
            ..D
        },
        Row {
            label: "Fading",
            definition: &[Category::Static, Category::Triggered],
            ..D
        },
        Row {
            label: "Firebending",
            definition: &[Category::Triggered],
            ..D
        },
        Row {
            label: "Flanking",
            definition: &[Category::Triggered],
            ..D
        },
        Row {
            label: "ForMirrodin",
            definition: &[Category::Triggered],
            ..D
        },
        Row {
            label: "Forecast",
            on_spell_card: true,
            ..D
        },
        Row {
            label: "Fortify",
            paid_cost: true,
            definition: &[Category::Activated],
            ..D
        },
        Row {
            label: "Frenzy",
            ..D
        },
        Row {
            label: "Graft",
            definition: &[Category::Static, Category::Triggered],
            ..D
        },
        Row {
            label: "Gravestorm",
            regime: Some(Regime::AtCasting),
            functions_on_stack: true,
            on_spell_card: true,
            definition: &[Category::Triggered],
            ..D
        },
        Row {
            label: "Haunt",
            on_spell_card: true,
            ..D
        },
        Row {
            label: "HiddenAgenda",
            ..D
        },
        Row {
            label: "Hideaway",
            ..D
        },
        Row {
            label: "Horsemanship",
            definition: &[Category::Static],
            ..D
        },
        Row {
            label: "Increment",
            regime: Some(Regime::AtCasting),
            definition: &[Category::Triggered],
            ..D
        },
        Row {
            label: "Infinity",
            ..D
        },
        Row {
            label: "Ingest",
            definition: &[Category::Triggered],
            ..D
        },
        Row {
            label: "Intimidate",
            ..D
        },
        Row {
            label: "JobSelect",
            definition: &[Category::Triggered],
            ..D
        },
        Row {
            label: "JumpStart",
            on_permanent_card: false,
            on_spell_card: true,
            ..D
        },
        Row {
            label: "LevelUp",
            paid_cost: true,
            definition: &[Category::Activated],
            ..D
        },
        Row {
            label: "LivingMetal",
            definition: &[Category::Static],
            ..D
        },
        Row {
            label: "LivingWeapon",
            definition: &[Category::Triggered],
            ..D
        },
        Row {
            label: "MaxSpeed",
            ..D
        },
        Row {
            label: "Melee",
            definition: &[Category::Triggered],
            ..D
        },
        Row {
            label: "Mentor",
            definition: &[Category::Triggered],
            ..D
        },
        Row {
            label: "Mobilize",
            definition: &[Category::Triggered],
            ..D
        },
        Row {
            label: "Modular",
            // "This permanent enters with N +1/+1 counters on it" and "When
            // this permanent is put into a graveyard from the battlefield, you
            // may put a +1/+1 counter on target artifact creature for each
            // +1/+1 counter on this permanent" — a static ability and a
            // triggered one [CR#702.43a].
            definition: &[Category::Static, Category::Triggered],
            ..D
        },
        Row {
            label: "MoreThanMeetsTheEye",
            paid_cost: true,
            ..D
        },
        Row {
            label: "Myriad",
            definition: &[Category::Triggered],
            ..D
        },
        Row {
            label: "Nightbound",
            ..D
        },
        Row {
            label: "Offering",
            regime: Some(Regime::AtCasting),
            functions_on_stack: true,
            ..D
        },
        Row {
            label: "Outlast",
            paid_cost: true,
            definition: &[Category::Activated],
            ..D
        },
        Row {
            label: "Paradigm",
            on_spell_card: true,
            ..D
        },
        Row {
            label: "Partner",
            ..D
        },
        Row {
            label: "Persist",
            definition: &[Category::Triggered],
            ..D
        },
        Row {
            label: "Plot",
            on_spell_card: true,
            paid_cost: true,
            ..D
        },
        Row {
            label: "Poisonous",
            definition: &[Category::Triggered],
            ..D
        },
        Row {
            label: "Provoke",
            definition: &[Category::Triggered],
            ..D
        },
        Row {
            label: "Rampage",
            definition: &[Category::Triggered],
            ..D
        },
        Row {
            label: "Ravenous",
            definition: &[Category::Static, Category::Triggered],
            ..D
        },
        Row {
            label: "ReadAhead",
            ..D
        },
        Row {
            label: "Rebound",
            regime: Some(Regime::AtCasting),
            functions_on_stack: true,
            on_permanent_card: false,
            on_spell_card: true,
            ..D
        },
        Row {
            label: "Reconfigure",
            paid_cost: true,
            definition: &[Category::Activated, Category::Activated],
            ..D
        },
        Row {
            label: "Recover",
            on_spell_card: true,
            paid_cost: true,
            definition: &[Category::Triggered],
            ..D
        },
        Row {
            label: "Reinforce",
            on_spell_card: true,
            paid_cost: true,
            definition: &[Category::Activated],
            ..D
        },
        Row { label: "Riot", ..D },
        Row {
            label: "Ripple",
            regime: Some(Regime::AtCasting),
            functions_on_stack: true,
            on_spell_card: true,
            ..D
        },
        Row {
            label: "Scavenge",
            paid_cost: true,
            definition: &[Category::Activated],
            ..D
        },
        Row {
            label: "Solved",
            ..D
        },
        Row {
            label: "Soulbond",
            ..D
        },
        Row {
            label: "Soulshift",
            definition: &[Category::Triggered],
            ..D
        },
        Row {
            label: "SpaceSculptor",
            ..D
        },
        Row {
            label: "Splice",
            on_spell_card: true,
            paid_cost: true,
            ..D
        },
        Row {
            label: "Spree",
            regime: Some(Regime::AtCasting),
            functions_on_stack: true,
            on_spell_card: true,
            wants_modes: true,
            ..D
        },
        Row {
            label: "StartYourEngines",
            ..D
        },
        Row {
            label: "Station",
            definition: &[Category::Activated],
            ..D
        },
        Row {
            label: "Sunburst",
            definition: &[Category::Static, Category::Static],
            ..D
        },
        Row {
            label: "Teamwork",
            regime: Some(Regime::AtCasting),
            functions_on_stack: true,
            on_spell_card: true,
            ..D
        },
        Row {
            label: "Tiered",
            regime: Some(Regime::AtCasting),
            functions_on_stack: true,
            on_spell_card: true,
            wants_modes: true,
            ..D
        },
        Row {
            label: "Toxic",
            ..D
        },
        Row {
            label: "Training",
            definition: &[Category::Triggered],
            ..D
        },
        Row {
            label: "Transfigure",
            paid_cost: true,
            definition: &[Category::Activated],
            ..D
        },
        Row {
            label: "Transmute",
            on_spell_card: true,
            paid_cost: true,
            definition: &[Category::Activated],
            ..D
        },
        Row {
            label: "Tribute",
            ..D
        },
        Row {
            label: "UmbraArmor",
            definition: &[Category::Static],
            ..D
        },
        Row {
            label: "Undaunted",
            regime: Some(Regime::AtCasting),
            functions_on_stack: true,
            on_spell_card: true,
            definition: &[Category::Static],
            ..D
        },
        Row {
            label: "Undying",
            definition: &[Category::Triggered],
            ..D
        },
        Row {
            label: "Unleash",
            ..D
        },
        Row {
            label: "Vanishing",
            definition: &[Category::Static, Category::Triggered, Category::Triggered],
            ..D
        },
        Row {
            label: "Visit",
            ..D
        },
        Row {
            label: "WebSlinging",
            regime: Some(Regime::AtCasting),
            functions_on_stack: true,
            paid_cost: true,
            definition: &[Category::Static],
            ..D
        },
        Row {
            label: "Wither",
            regime: Some(Regime::AtResolution),
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
    let declaration = deckmaste_construction_core::macro_def::read_str(&path, &src)?;
    let params = declaration
        .params()
        .unwrap_or_default()
        .iter()
        .map(deckmaste_construction_core::macro_def::ParameterType::as_str)
        .collect::<Vec<_>>();
    let shape = Shape::from_params(&params).with_context(|| {
        format!("{name}: parameter signature outside the workbench vocabulary: {params:?}")
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
         plugins_v2/builtin/macros/stubs/keyword_abilities/*.ron and xtask's gate-column \
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
        if row.functions_on_stack {
            fields.push("functionsOnStack := True".to_owned());
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
        // The reference table has no category list, and its own `bodied`
        // column means "the entry defines ONE triggered ability with a quoted
        // expansion" — so a multi-category definition is not bodied there.
        if row.definition == [Category::Triggered] {
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
    let lean_text = lean::render(root)?;
    let path = root.join(GENERATED);
    let text = render(root)?;
    fs::write(root.join(lean::GENERATED), lean_text)?;
    println!("wrote {}", lean::GENERATED);
    fs::write(&path, &text).with_context(|| format!("writing {}", path.display()))?;
    println!(
        "wrote {} ({} rows)",
        path.display(),
        text.lines().filter(|l| l.contains(":=")).count()
    );
    Ok(())
}

fn run_check(root: &Path) -> anyhow::Result<()> {
    let lean_path = root.join(lean::GENERATED);
    anyhow::ensure!(
        fs::read_to_string(&lean_path)? == lean::render(root)?,
        "{} is stale: re-run `cargo xtask facts generate`",
        lean_path.display()
    );
    println!("{} is up to date", lean_path.display());
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

/// A label the check does not require on the other side, and the reason it
/// prints. A gap closes with a row, with a stub, or with a reason here.
#[derive(Debug, Clone, Copy)]
struct Exempt {
    label: &'static str,
    reason: &'static str,
}

const KEYWORD_SCOPE: &str = "both directions bind — every keyword-ability stub [CR#702] wants a \
                             `keywordFacts` row and every row wants a stub.";

const ACT_SCOPE: &str = "every keyword-action stub [CR#701] wants an `actFacts` row; a row need \
                         not have a stub — `actFacts` also carries the turn-and-game deed \
                         vocabulary (attack, block, draw, …), which is wider than [CR#701], so \
                         the row-without-a-stub direction is informational.";

const COUNTER_SCOPE: &str = "every counter-kind stub wants a `counterFacts` row; a row need not \
                             have a stub — `counterFacts` carries the counter kinds the CR names \
                             [CR#122.1] and the printed ones the corpus attests, which is wider \
                             than the RON counter-kind macros, so the row-without-a-stub \
                             direction is informational.";

const DESIGNATION_SCOPE: &str = "both directions bind, through the stub-name → `Designation` \
                                 constructor mapping below, because a stub name is not the Idris \
                                 constructor name.";

const GATE_COLUMNS: &str = "gate columns (counterEligible, regime, functionsOnStack, \
                            onPermanentCard, onSpellCard, paidCost, definition, wantsModes) stay \
                            hand-kept in xtask's overlay: \
                            plugins_v2/builtin/macros/meta/KeywordAbility.ron declares no field \
                            that could carry them, and its `metadata` block is read into \
                            deckmaste_construction_core::macro_def::Metadata, which is \
                            deny_unknown_fields over spelling/grammar/noun_class/category, so a \
                            home in the stub schema moves that crate's English-v2 metadata seam \
                            and all 195 stubs rather than this overlay.";

const ROLE_COLUMNS: &str = "actFacts role columns (agentRole, patientRole) stay hand-kept per \
                            deed: they are the gate on a deontic clause ([noun] can't/must \
                            [deed]) through Effect.Deontic's DeedFits obligation, not a \
                            transcription of the deed's [CR#701] entry — the Destroy row carries \
                            roleTypes = [] though [CR#701.8a] destroys a permanent of any type — \
                            so each is authored as a bench sentence spells its deed. The \
                            destination column is a CR fact and is authored from the entry.";

const REGIME_AXIS: &str = "regime and functionsOnStack are two columns because they are two \
                           facts: Effect.keywordBodyFits reads regime as `the keyword's \
                           triggered body keys on a spell cast`, Effect.grantSubjectFits reads \
                           functionsOnStack as `the keyword's ability functions only while its \
                           object is on the stack` [CR#113.6,113.6d,113.6e]. The 46 cost, \
                           alternative-cast and cast-trigger keywords carry both; prowess, \
                           extort and increment carry regime alone, because each is a triggered \
                           ability of a permanent [CR#702.108a,702.101a,702.191a] and so \
                           functions on the battlefield [CR#113.6]; split second carries \
                           functionsOnStack alone, being a static ability that functions only \
                           while its spell is on the stack [CR#702.61a]. A keyword whose ability also \
                           functions in the zone a grant subject sits in — flashback \
                           [CR#702.34a] and its graveyard-cast siblings [CR#113.6e] — carries \
                           neither, so `Target instant or sorcery card in your graveyard gains \
                           flashback` stays spellable.";

/// Keyword-ability stubs the workbench does not row.
const KEYWORD_STUBS_EXEMPT: &[Exempt] = &[Exempt {
    label: "Prototype",
    reason: "a layout keyword, not a keyword-ability row: the prototype ability is the inset \
             frame itself [CR#702.160a,718.1], so the workbench spells it as the `Card` wrapper \
             `Prototype (inner) (alt)` whose pair is the alternative mana cost and printed box \
             [CR#718.1]; a row carrying its `[Cost, Power, Toughness]` signature would be a \
             second representation of the same ability",
}];

/// Rows whose label is a CR-defined variant of a keyword that has its own
/// stub, so the RON vocabulary spells the variant through that stub.
const KEYWORD_ROWS_EXEMPT: &[Exempt] = &[
    Exempt {
        label: "Multikicker",
        reason: "a variant of the kicker ability [CR#702.33c], spelled through the Kicker stub",
    },
    Exempt {
        label: "BandsWithOther",
        reason: "a special form of banding [CR#702.22b], spelled through the Banding stub",
    },
    Exempt {
        label: "PartnerWith",
        reason: "one of the partner abilities [CR#702.124j], spelled through the Partner stub",
    },
];

/// Designation stubs with no `Words.Designation` constructor. Every stub has
/// one, so the list is empty; a stub the CR made a non-object property would
/// be recorded here rather than rowed.
const DESIGNATION_STUBS_EXEMPT: &[Exempt] = &[];

/// The designation stubs whose name is not the Idris constructor's; every
/// other stub name is the constructor name.
const DESIGNATION_MAP: &[(&str, &[&str])] = &[
    ("Commander", &["CommanderD"]),
    ("Initiative", &["TheInitiative"]),
    ("DayNight", &["Day", "Night"]),
    ("Sector", &["AlphaSector", "BetaSector", "GammaSector"]),
];

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

/// Every label captured by `MkCounterFacts "…"`-style positional rows.
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

/// One `actFacts` row: the label its `plainAct` names, and the row text the
/// `{ field := … }` record-update overrides are read out of.
struct ActRow {
    label: String,
    #[cfg_attr(
        not(test),
        allow(dead_code, reason = "the override text is read by the row-shape tests")
    )]
    text: String,
}

/// The `actFacts` rows, read off the list literal in `Words.idr`. A row is
/// either a bare `plainAct "Label"` or a record update over one.
fn act_rows(src: &str) -> Vec<ActRow> {
    let mut rows = Vec::new();
    let mut current: Option<String> = None;
    let mut in_table = false;
    for line in src.lines() {
        if !in_table {
            in_table = line == "actFacts =";
            continue;
        }
        if line.starts_with("  [ ") || line.starts_with("  , ") {
            push_act_row(&mut rows, current.take());
            current = Some(line.to_owned());
            continue;
        }
        if line == "  ]" {
            break;
        }
        if let Some(text) = current.as_mut() {
            text.push('\n');
            text.push_str(line);
        }
    }
    push_act_row(&mut rows, current.take());
    rows
}

fn push_act_row(rows: &mut Vec<ActRow>, text: Option<String>) {
    let Some(text) = text else { return };
    let Some(rest) = text.split_once("plainAct \"").map(|(_, r)| r) else {
        return;
    };
    let Some((label, _)) = rest.split_once('"') else {
        return;
    };
    rows.push(ActRow {
        label: label.to_owned(),
        text,
    });
}

/// One record-update override on a row, read to the end of its line; `None`
/// where the row leaves the field at its `plainAct` default.
#[cfg(test)]
fn act_field<'a>(row: &'a ActRow, field: &str) -> Option<&'a str> {
    let rest = row.text.split_once(&format!("{field} := "))?.1;
    Some(rest.split('\n').next().unwrap_or(rest).trim_end())
}

/// The `Designation` constructors, read off the `designationFacts` clauses.
fn designation_ctors(src: &str) -> Vec<String> {
    src.lines()
        .filter_map(|line| {
            let rest = line.strip_prefix("designationFacts ")?;
            let (ctor, _) = rest.split_once(' ')?;
            ctor.starts_with(char::is_uppercase)
                .then(|| ctor.to_owned())
        })
        .collect()
}

/// A designation stub's constructor names.
fn designation_labels(stubs: &[String]) -> Vec<String> {
    stubs
        .iter()
        .flat_map(
            |stub| match DESIGNATION_MAP.iter().find(|(s, _)| s == stub) {
                Some((_, ctors)) => ctors.iter().map(|c| (*c).to_owned()).collect::<Vec<_>>(),
                None => vec![stub.clone()],
            },
        )
        .collect()
}

/// One table's two-way difference, with its scope rule and its recorded
/// reasons printed beside it.
struct Table<'a> {
    name: &'a str,
    scope: &'a str,
    stubs: Vec<String>,
    rows: Vec<String>,
    /// Stubs that need no row.
    stub_exempt: &'a [Exempt],
    /// Rows that need no stub.
    row_exempt: &'a [Exempt],
    /// Whether a row without a stub fails, or only reports.
    rows_bind: bool,
}

fn missing(theirs: &[String], keys: &[String], exempt: &[Exempt]) -> Vec<String> {
    let exempt: Vec<String> = exempt.iter().map(|e| normalize(e.label)).collect();
    theirs
        .iter()
        .filter(|label| {
            let key = normalize(label);
            !keys.contains(&key) && !exempt.contains(&key)
        })
        .cloned()
        .collect()
}

/// A recorded reason for a label that is present on both sides is stale, and
/// stale reasons are how a scope rule quietly stops meaning anything.
fn stale(exempt: &[Exempt], keys: &[String]) -> Vec<&'static str> {
    exempt
        .iter()
        .filter(|e| keys.contains(&normalize(e.label)))
        .map(|e| e.label)
        .collect()
}

/// Reports the table, and returns whether it is within its scope rule.
fn report(t: &Table) -> bool {
    let stub_keys: Vec<String> = t.stubs.iter().map(|s| normalize(s)).collect();
    let row_keys: Vec<String> = t.rows.iter().map(|s| normalize(s)).collect();
    let missing_rows = missing(&t.stubs, &row_keys, t.stub_exempt);
    let missing_stubs = missing(&t.rows, &stub_keys, t.row_exempt);
    let stale_reasons = [
        stale(t.stub_exempt, &row_keys),
        stale(t.row_exempt, &stub_keys),
    ]
    .concat();

    println!(
        "{}: {} stubs, {} rows, {} stubs without a row, {} rows without a stub{}",
        t.name,
        t.stubs.len(),
        t.rows.len(),
        missing_rows.len(),
        missing_stubs.len(),
        if t.rows_bind { "" } else { " (informational)" }
    );
    println!("  scope: {}", t.scope);
    for e in t.stub_exempt {
        println!(
            "  stub with no row, recorded reason: {} — {}",
            e.label, e.reason
        );
    }
    for e in t.row_exempt {
        println!(
            "  row with no stub, recorded reason: {} — {}",
            e.label, e.reason
        );
    }
    for label in &missing_rows {
        println!("  stub without a row: {label}");
    }
    for label in &missing_stubs {
        println!("  row without a stub: {label}");
    }
    for label in &stale_reasons {
        println!("  stale recorded reason (the label is on both sides): {label}");
    }
    missing_rows.is_empty()
        && stale_reasons.is_empty()
        && (!t.rows_bind || missing_stubs.is_empty())
}

/// The counter stubs' printed spellings. The workbench spells the +1/+1 and
/// -1/-1 counters as `BoostCounter`, not as a label, and a keyword counter
/// [CR#122.1b] as the eligible keyword's own row.
fn counter_stub_spellings(dir: &Path) -> anyhow::Result<Vec<String>> {
    let mut spellings = Vec::new();
    for name in stub_names(dir)? {
        if name == "P1P1Counter" || name == "M1M1Counter" {
            continue;
        }
        let path = dir.join(format!("{name}.ron"));
        let src =
            fs::read_to_string(&path).with_context(|| format!("reading {}", path.display()))?;
        let spelling =
            stub_field(&src, "spelling").with_context(|| format!("{name}: no `spelling` field"))?;
        spellings.push(spelling.trim_matches('"').to_owned());
    }
    Ok(spellings)
}

fn run_labels(root: &Path) -> anyhow::Result<()> {
    let words_path = root.join(WORDS);
    let words = fs::read_to_string(&words_path)
        .with_context(|| format!("reading {}", words_path.display()))?;

    let mut counter_rows = table_labels(&words, "MkCounterFacts");
    counter_rows.extend(
        overlay()
            .iter()
            .filter(|r| r.counter_eligible)
            .map(|r| r.label.to_owned()),
    );
    let designation_stubs = stub_names(&root.join(DESIGNATION_STUBS))?;

    let tables = [
        Table {
            name: "keyword abilities",
            scope: KEYWORD_SCOPE,
            stubs: stub_names(&root.join(KEYWORD_STUBS))?,
            rows: overlay().iter().map(|r| r.label.to_owned()).collect(),
            stub_exempt: KEYWORD_STUBS_EXEMPT,
            row_exempt: KEYWORD_ROWS_EXEMPT,
            rows_bind: true,
        },
        Table {
            name: "keyword actions",
            scope: ACT_SCOPE,
            stubs: stub_names(&root.join(ACTION_STUBS))?,
            rows: act_rows(&words).into_iter().map(|r| r.label).collect(),
            stub_exempt: &[],
            row_exempt: &[],
            rows_bind: false,
        },
        Table {
            name: "counter kinds",
            scope: COUNTER_SCOPE,
            stubs: counter_stub_spellings(&root.join(COUNTER_STUBS))?,
            rows: counter_rows,
            stub_exempt: &[],
            row_exempt: &[],
            rows_bind: false,
        },
        Table {
            name: "designations",
            scope: DESIGNATION_SCOPE,
            stubs: designation_labels(&designation_stubs),
            rows: designation_ctors(&words),
            stub_exempt: DESIGNATION_STUBS_EXEMPT,
            row_exempt: &[],
            rows_bind: true,
        },
    ];

    let mut ok = true;
    for table in &tables {
        ok &= report(table);
    }
    for (stub, ctors) in DESIGNATION_MAP {
        println!("  designation mapping: {stub} → {}", ctors.join(", "));
    }
    println!("{GATE_COLUMNS}");
    println!("{ROLE_COLUMNS}");
    println!("{REGIME_AXIS}");

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
    fn the_stack_column_is_the_grant_gate_and_the_regime_column_is_the_body_axis() {
        let text = render(&root()).expect("rendering the keyword facts module");
        for label in ["Prowess", "Extort", "Increment"] {
            assert!(
                text.contains(&format!(
                    "{{ word := \"{label}\", paramShapes := [NoParam], regime := Just \
                     AtCasting, bodied := True }}"
                )),
                "{label}: a triggered ability of a permanent must carry regime alone"
            );
        }
        assert!(text.contains(
            "{ word := \"Convoke\", paramShapes := [NoParam], regime := Just AtCasting, \
             functionsOnStack := True"
        ));
        assert!(text.contains(
            "{ word := \"SplitSecond\", paramShapes := [NoParam], functionsOnStack := True"
        ));
        for row in overlay() {
            if row.functions_on_stack && row.regime != Some(Regime::AtCasting) {
                assert_eq!(
                    row.label, "SplitSecond",
                    "{}: functionsOnStack without an AtCasting body regime; split second is \
                     the static ability that functions only while its spell is on the stack \
                     [CR#702.61a]",
                    row.label
                );
            }
        }
    }

    #[test]
    fn the_committed_module_matches_the_stubs() {
        run_check(&root()).expect("`cargo xtask facts check` must pass on a clean tree");
    }

    #[test]
    fn every_stub_has_a_row_or_a_recorded_reason() {
        run_labels(&root()).expect("`cargo xtask facts labels` must pass on a clean tree");
    }

    #[test]
    fn recorded_reasons_name_labels_that_are_really_there() {
        let stubs = stub_names(&root().join(KEYWORD_STUBS)).expect("reading the keyword stubs");
        for e in KEYWORD_STUBS_EXEMPT {
            assert!(
                stubs.contains(&e.label.to_owned()),
                "{}: no such stub",
                e.label
            );
        }
        let rows: Vec<&str> = overlay().iter().map(|r| r.label).collect();
        for e in KEYWORD_ROWS_EXEMPT {
            assert!(rows.contains(&e.label), "{}: no such row", e.label);
        }
        let designations =
            stub_names(&root().join(DESIGNATION_STUBS)).expect("reading the designation stubs");
        for e in DESIGNATION_STUBS_EXEMPT {
            assert!(
                designations.contains(&e.label.to_owned()),
                "{}: no such stub",
                e.label
            );
        }
        for (stub, _) in DESIGNATION_MAP {
            assert!(
                designations.contains(&(*stub).to_owned()),
                "{stub}: no such stub"
            );
        }
    }

    #[test]
    fn the_designation_map_reaches_the_idris_constructors() {
        let words = fs::read_to_string(root().join(WORDS)).expect("reading Words.idr");
        let ctors = designation_ctors(&words);
        assert!(ctors.contains(&"CommanderD".to_owned()));
        assert!(ctors.contains(&"Day".to_owned()) && ctors.contains(&"Night".to_owned()));
        for ctor in [
            "Harnessed",
            "Level",
            "Solved",
            "AlphaSector",
            "BetaSector",
            "GammaSector",
        ] {
            assert!(
                ctors.contains(&ctor.to_owned()),
                "{ctor}: no designationFacts row"
            );
        }
        assert!(
            !ctors.iter().any(|c| c == ":"),
            "a type signature was read as a constructor"
        );
        let mapped = designation_labels(&["DayNight".to_owned(), "Goaded".to_owned()]);
        assert_eq!(mapped, vec!["Day", "Night", "Goaded"]);
        assert_eq!(
            designation_labels(&["Sector".to_owned()]),
            vec!["AlphaSector", "BetaSector", "GammaSector"]
        );
    }

    #[test]
    fn keyword_action_destinations_come_from_their_cr_entry() {
        let words = fs::read_to_string(root().join(WORDS)).expect("reading Words.idr");
        let rows = act_rows(&words);
        for (label, zone) in [
            ("Create", "Battlefield"),
            ("Investigate", "Battlefield"),
            ("Incubate", "Battlefield"),
            ("Manifest", "Battlefield"),
            ("Cloak", "Battlefield"),
            ("Airbend", "Exile"),
            ("Collect Evidence", "Exile"),
        ] {
            let row = rows
                .iter()
                .find(|r| r.label == label)
                .unwrap_or_else(|| panic!("{label}: no `actFacts` row"));
            assert_eq!(
                act_field(row, "actDest"),
                Some(format!("Just {zone}").as_str()),
                "{label}: no {zone} destination"
            );
            assert_eq!(
                act_field(row, "participle"),
                None,
                "{label}: a participle where the row leaves it at its default"
            );
        }
    }

    #[test]
    fn act_rows_read_the_bare_and_the_overridden_spelling() {
        let words = fs::read_to_string(root().join(WORDS)).expect("reading Words.idr");
        let rows = act_rows(&words);
        let bare = rows
            .iter()
            .find(|r| r.label == "Proliferate")
            .expect("no Proliferate row");
        assert_eq!(act_field(bare, "actDest"), None);
        let destroy = rows
            .iter()
            .find(|r| r.label == "Destroy")
            .expect("no Destroy row");
        assert_eq!(
            act_field(destroy, "participle"),
            Some(r#"Just "destroyed""#)
        );
        assert_eq!(act_field(destroy, "actDest"), Some("Just Graveyard"));
        assert_eq!(
            act_field(destroy, "patientRole"),
            Some("MkDeedRole [Object] [] False (Just Battlefield)")
        );
        let attack = rows
            .iter()
            .find(|r| r.label == "Attack")
            .expect("no Attack row");
        assert_eq!(act_field(attack, "actFeature"), Some("Just Attacking"));
        let mut labels: Vec<&str> = rows.iter().map(|r| r.label.as_str()).collect();
        let count = labels.len();
        labels.sort_unstable();
        labels.dedup();
        assert_eq!(labels.len(), count, "an `actFacts` label is listed twice");
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
