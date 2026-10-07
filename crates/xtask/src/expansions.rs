//! The `expansions` command: print the fully expanded body of every macro
//! declaration a `plugins_v2` plugin defines, one file per declaration, so
//! two snapshots of the tree can be compared with `diff -r`.
//!
//! It is the declaration-side analogue of `card`. `lean check` proves a
//! re-spelling pure only for the declarations a canon card happens to use;
//! this covers every declaration, because it expands each one by itself.
//!
//! **What a file holds.** The declaration is invoked by name at the position
//! its body occupies — the first of its kinds that is a v2 syntax type
//! (`KeywordAction` → `Instruction`, `KeywordAbility` → `Ability`) — through
//! the same macro-aware reader a card is read with, and the TYPED value that
//! comes back is printed with `{:#?}`. The typed value is the canonical form:
//! every macro is resolved to the constructor basis (v2 expansion is
//! name-erasing, `deckmaste_semantics_v2::ron`), injections and elided
//! constructors are restored by the read, and the derived `Debug` prints every
//! field of every constructor by name. Two spellings print the same exactly
//! when they read to the same value. A registry declaration whose body is a
//! `Definition` node (counters, subtypes) is read as that node directly,
//! because invoking it at its own position yields only the term it denotes
//! and would hide its rules; a designation's body is a list of them.
//!
//! **Holes.** A declaration with parameters is instantiated with sample
//! arguments drawn from [`SAMPLES`], a fixed list written in the constructor
//! basis. A typed parameter takes the samples its declared type's validator
//! accepts, starting at its ordinal among same-typed parameters, so two
//! `Amount` parameters get different amounts. An `Any` parameter takes
//! [`WORD_SAMPLES`] and then the whole of [`SAMPLES`]. The first assignment (in list order, no two parameters given
//! the same text) whose invocation reads is used, and the invocation is
//! printed in the file's header. Defaulted and elidable parameters are left
//! out, so their default is what the body sees. Everything here is a function
//! of the declaration and the fixed list, so the choice is deterministic.
//!
//! Declarations that cannot be printed are listed in `_skipped.txt` with the
//! reason, never dropped; a declaration that should print and does not read
//! fails the command.

use std::collections::BTreeMap;
use std::fmt::Debug;
use std::fmt::Write as _;
use std::path::Path;
use std::path::PathBuf;

use anyhow::Context as _;
use clap::Args;
use deckmaste_semantics_v2::abilities;
use deckmaste_semantics_v2::card;
use deckmaste_semantics_v2::phrase;
use deckmaste_semantics_v2::reader::Declaration;
use deckmaste_semantics_v2::reader::Plugin;
use deckmaste_semantics_v2::rules;
use deckmaste_semantics_v2::rules::Definition;
use deckmaste_semantics_v2::triggers;
use deckmaste_semantics_v2::words;
use macro_ron::MacroSet;
use macro_ron::ParamType;
use macro_ron::Params;
use rayon::iter::IntoParallelIterator as _;
use rayon::iter::ParallelIterator as _;
use serde::de::DeserializeOwned;

/// The file the run index is written to, inside the output directory. Its
/// presence is also what marks a directory as one this command may clear.
const INDEX_FILE: &str = "_index.txt";
/// The file listing every declaration that was not printed, and why.
const SKIPPED_FILE: &str = "_skipped.txt";
/// How many sample assignments are tried for one declaration before it is
/// reported as uninstantiable.
const MAX_ATTEMPTS: usize = 50_000;

/// The sample arguments for TYPED parameters, in the constructor basis, in
/// preference order.
///
/// Numbers come first because a numeral is the only spelling several types
/// share (`Amount`'s literal leaf, `Nat`), and listing no `Lit(…)` beside
/// them keeps distinct texts distinct values. Every entry but the final
/// `None` must read as at least one registered param type
/// (`every_sample_reads_as_some_param_type`).
/// Appending is safe; reordering or editing an entry changes the samples
/// every snapshot was taken with, so do it only between sweeps.
pub const SAMPLES: &[&str] = &[
    // Amount (literal leaf), Power, Toughness, Nat.
    "3",
    "4",
    "5",
    "6",
    // String and the `…Label` aliases.
    "\"Sample A\"",
    "\"Sample B\"",
    "\"Sample C\"",
    // NounPhrase / Subject.
    "You",
    "This",
    "Described(determiner: Bare, predicate: HasType(Creature))",
    "Described(determiner: Bare, predicate: HasType(Artifact))",
    // Predicate / Quality.
    "HasType(Land)",
    "HasType(Enchantment)",
    "IsToken",
    // Cost.
    "TapSymbol",
    "UntapSymbol",
    "ItsManaCost",
    // Ability.
    "MayBeginOnBattlefield",
    "Keyword(keyword: \"Sample\", params: [], body: [])",
    // Instruction.
    "Shuffle",
    "DrawGame",
    "RestartGame",
    // Condition.
    "RolledDoubles",
    "Not(RolledDoubles)",
    // Duration.
    "ThisTurn",
    "RestOfGame",
    // GameEvent.
    "Draws(player: You)",
    "LosesGame(player: You)",
    // Quantity.
    "ExactlyOf(amount: 7)",
    "UpToOf(amount: 8)",
    // ZoneExpr.
    "Zone(zone: Graveyard, scope: Bare)",
    "Zone(zone: Exile, scope: Bare)",
    // StaticSpec.
    "Modification(subject: This, stat: Power, delta: Up(amount: 2))",
    "Modification(subject: This, stat: Toughness, delta: Down(amount: 2))",
    // Subtype.
    "Of(host: Creature, label: \"Sample\")",
    "Of(host: Creature, label: \"Sample B\")",
    // CounterKind, CounterKindSource.
    "Named(name: \"Sample\")",
    "Named(name: \"Sample B\")",
    "Printed(kind: Named(name: \"Sample\"))",
    // TurnPart.
    "Upkeep",
    "DrawStep",
    // Color, CardType.
    "White",
    "Blue",
    "Creature",
    "Artifact",
    // Kind.
    "Object",
    "Player",
    // QualitySort.
    "CardName",
    // Lookback.
    "EarlierThisTurn",
    "LastTurn",
    // Deed.
    "Core(deed: Draw)",
    "Action(label: \"Sample\")",
    // NounWord.
    "Card",
    "Permanent",
    // The small closed word types.
    "CombatOnly",
    "NoncombatOnly",
    "AtLeast",
    "Less",
    "Secretly",
    "Openly",
    "Everywhere",
    "Sum",
    "Max",
    "Nth(n: 2)",
    "Nth(n: 3)",
    "OncePerTurn",
    "OncePerGame",
    "AsSorcery",
    "AsInstant",
    "Stat(stat: Power)",
    "Stat(stat: Toughness)",
    "TheAlternative",
    "TheAdditional",
    "Agent",
    "Patient",
    "WhileTrue(condition: RolledDoubles)",
    "Put",
    "Removed",
    "Emptying",
    "The",
    "Each",
    "AnyOrder",
    "RandomOrder",
    "Forbid",
    "Require",
    "NoPatient",
    "NoRider",
    "ByLabel(options: [\"Sample A\", \"Sample B\"])",
    "ByCandidate(candidates: You)",
    "Attributive",
    "ThisWay",
    "One",
    "Many",
    "AnyColor",
    "AnyType",
    "Colorless",
    "BasicTypesOnly",
    "Within(lookback: ThisTurn)",
    "OneZone(zone: Zone(zone: Graveyard, scope: Bare))",
    "ToCast(predicate: HasType(Creature))",
    "AsThose",
    "Greater(stat: Power, amount: 2)",
    // Stat, Delta, CharacteristicBundle.
    "Power",
    "Toughness",
    "Up(amount: 2)",
    "Down(amount: 3)",
    "(characteristics: (colors: [Green], types: [Creature], subtypes: [Of(host: Creature, label: \"Sample\")], power: 2, toughness: 3))",
    // Lists, for the plural param types.
    "[White]",
    "[Blue, Black]",
    "[Of(host: Creature, label: \"Sample\")]",
    "[MayBeginOnBattlefield]",
    "[Core(deed: Draw)]",
    "[Draws(player: You)]",
    "[]",
    // Last: the absent value, which hides the hole it fills. No param type
    // reads it; it is here for an `Any` hole at an `Option` position.
    "None",
];

/// Further samples for `Any` parameters only: values of the word types an
/// `Any` hole is spliced into that no registered param type names (a
/// constructor alias's field types, mostly). Tried BEFORE [`SAMPLES`] at an
/// `Any` hole, so a word position finds its value early. The same append-only
/// rule applies.
pub const WORD_SAMPLES: &[&str] = &[
    "AnyOnStack",
    "Plus",
    "Attached",
    "Enchanted",
    "AnEffect",
    "Token(spec: AsThose, riders: [])",
    "ThisDoor",
    "NoPossessor",
    "Chosen",
    "Again",
    "AnyResult",
    "Whole",
    "EntersTransformed",
    "Front",
    "Entry",
    "TheChoice",
    "Heads",
    "AttackerOf",
    "Attacking",
    "AsLongAs",
    "Required",
    "FromStack",
    "Unattributed",
    "PowerAlone",
    "Cards(cards: You)",
    "LookAt",
    "Wins",
    "Numbers",
    "CardType",
    "X",
    "Top",
    "ThisMana",
    "DifferentNames",
    "Before",
    "DamageDealt",
    "WinGame",
    "ColorsSpent",
    "Once",
    "Paid",
    "AllPlayers",
    "Owner",
    "NoPreventionOnly",
    "Repeatedly",
    "Tapped",
    "Legendary",
    "Count",
    "SomeTarget",
    "ThatDamage",
    "TopOfLibrary",
    "Battlefield",
    "DownX",
    "EndOf(part: Cleanup, whose: None)",
    "Host",
    "The(keyword: \"Sample\")",
    "AbilityWord(label: \"Sample\")",
    "More(amount: 2)",
    "Damage(source: This)",
    "AsPrintedCost(subject: This)",
    "Up",
    "true",
    "false",
    "Bare",
];

#[derive(Debug, Args)]
pub struct ExpansionsArgs {
    /// The `plugins_v2` plugin whose declarations to print. A plugin other
    /// than `plugins_v2/builtin` is loaded over builtin, as `lean check` does.
    #[arg(long, default_value_os_t = default_plugin_dir())]
    plugin: PathBuf,
    /// The directory to write into. It is replaced: it must be absent,
    /// empty, or an earlier output of this command.
    #[arg(long, default_value_os_t = default_out_dir())]
    out: PathBuf,
    /// Load `--plugin` by itself, without `plugins_v2/builtin` underneath —
    /// for a copy of builtin kept elsewhere (an earlier revision's tree).
    #[arg(long)]
    no_prelude: bool,
}

fn workspace_root() -> PathBuf {
    Path::new(env!("CARGO_MANIFEST_DIR")).join("../..")
}

fn default_plugin_dir() -> PathBuf {
    workspace_root().join("plugins_v2/builtin")
}

fn default_out_dir() -> PathBuf {
    workspace_root().join("target/expansions")
}

/// What became of one declaration.
#[derive(Debug, Clone, PartialEq, Eq)]
pub enum Outcome {
    /// Printed: the header and the expanded value.
    Printed(String),
    /// Not printable, for the stated reason.
    Skipped(String),
    /// Should have printed and did not read.
    Failed(String),
}

/// # Errors
/// If the plugin fails to load, the output directory cannot be replaced or
/// written, or any declaration fails to expand.
pub fn run(args: &ExpansionsArgs) -> anyhow::Result<()> {
    let plugin = load(&args.plugin, args.no_prelude)?;
    let macros_dir = plugin
        .root()
        .join(deckmaste_semantics_v2::reader::MACROS_DIR);
    let outcomes = expand_all(&plugin);

    prepare_out_dir(&args.out)?;
    let mut index = String::new();
    let mut skipped = String::new();
    let mut failed = Vec::new();
    let mut printed = 0usize;
    for (path, outcome) in &outcomes {
        let relative = path.strip_prefix(&macros_dir).unwrap_or(path);
        let shown = relative.display();
        match outcome {
            Outcome::Printed(text) => {
                let file = args.out.join(relative).with_extension("expanded");
                if let Some(parent) = file.parent() {
                    std::fs::create_dir_all(parent)
                        .with_context(|| format!("creating {}", parent.display()))?;
                }
                std::fs::write(&file, text)
                    .with_context(|| format!("writing {}", file.display()))?;
                printed += 1;
                writeln!(index, "printed  {shown}")?;
            }
            Outcome::Skipped(reason) => {
                writeln!(index, "skipped  {shown}")?;
                writeln!(skipped, "{shown}: {reason}")?;
            }
            Outcome::Failed(reason) => {
                writeln!(index, "FAILED   {shown}")?;
                failed.push(format!("{shown}: {reason}"));
            }
        }
    }
    let skipped_count = outcomes
        .values()
        .filter(|outcome| matches!(outcome, Outcome::Skipped(_)))
        .count();
    let summary = format!(
        "{} declarations: {printed} printed, {skipped_count} skipped, {} failed",
        outcomes.len(),
        failed.len()
    );
    writeln!(index, "{summary}")?;
    std::fs::write(args.out.join(INDEX_FILE), index)?;
    std::fs::write(args.out.join(SKIPPED_FILE), &skipped)?;

    println!("{summary}");
    println!("written to {}", args.out.display());
    if !skipped.is_empty() {
        println!("skipped:");
        for line in skipped.lines() {
            println!("  {line}");
        }
    }
    if !failed.is_empty() {
        for line in &failed {
            eprintln!("FAILED {line}");
        }
        anyhow::bail!("{} declarations failed to expand", failed.len());
    }
    Ok(())
}

/// Loads `dir`, over `plugins_v2/builtin` unless it is builtin itself or
/// `standalone` is set.
fn load(dir: &Path, standalone: bool) -> anyhow::Result<Plugin> {
    let builtin = default_plugin_dir();
    let same = |a: &Path, b: &Path| match (a.canonicalize(), b.canonicalize()) {
        (Ok(a), Ok(b)) => a == b,
        _ => false,
    };
    if standalone || same(dir, &builtin) {
        Plugin::load(dir).with_context(|| format!("loading {}", dir.display()))
    } else {
        let prelude =
            Plugin::load(&builtin).with_context(|| format!("loading {}", builtin.display()))?;
        Plugin::load_with_prelude(&prelude, dir)
            .with_context(|| format!("loading {}", dir.display()))
    }
}

/// Empties `out` for a fresh run, refusing a non-empty directory this command
/// did not write.
fn prepare_out_dir(out: &Path) -> anyhow::Result<()> {
    if out.exists() {
        let empty = out.read_dir()?.next().is_none();
        anyhow::ensure!(
            empty || out.join(INDEX_FILE).is_file(),
            "{} is not empty and holds no `{INDEX_FILE}`; refusing to replace it",
            out.display()
        );
        std::fs::remove_dir_all(out).with_context(|| format!("clearing {}", out.display()))?;
    }
    std::fs::create_dir_all(out).with_context(|| format!("creating {}", out.display()))?;
    Ok(())
}

/// Every declaration `plugin`'s `macros/` directory defines, by source file,
/// with what became of it. A definition registered under several kinds is one
/// file and one entry.
#[must_use]
pub fn expand_all(plugin: &Plugin) -> BTreeMap<PathBuf, Outcome> {
    let mut by_file: BTreeMap<&Path, &Declaration> = BTreeMap::new();
    for declaration in plugin.declarations.values() {
        by_file.entry(&declaration.path).or_insert(declaration);
    }
    let samples = Samples::new(&plugin.macros);
    by_file
        .into_par_iter()
        .map(|(path, declaration)| {
            (
                path.to_path_buf(),
                expand_one(&plugin.macros, &samples, declaration),
            )
        })
        .collect()
}

/// Reads `source` as `T` with `macros` and prints the value canonically.
fn render<T: DeserializeOwned + Debug>(macros: &MacroSet, source: &str) -> Result<String, String> {
    macros
        .read_str::<T>(source)
        .map(|value| format!("{value:#?}\n"))
        .map_err(|error| error.to_string())
}

type Renderer = fn(&MacroSet, &str) -> Result<String, String>;

/// The v2 type a macro of kind `kind` produces, as a renderer — every kind
/// `deckmaste_semantics_v2::ron::kinds` registers except the declaration
/// positions, which are loader tags with no type of their own
/// (`every_syntax_kind_has_a_renderer`).
fn renderer(kind: &str) -> Option<Renderer> {
    Some(match kind {
        "GameEvent" => render::<phrase::GameEvent>,
        "NounPhrase" => render::<phrase::NounPhrase>,
        "Predicate" => render::<phrase::Predicate>,
        "Amount" => render::<phrase::Amount>,
        "Quantity" => render::<phrase::Quantity>,
        "ZoneExpr" => render::<phrase::ZoneExpr>,
        "Condition" => render::<phrase::Condition>,
        "ColorTerm" => render::<phrase::ColorTerm>,
        "Ballot" => render::<phrase::Ballot>,
        "HeaderPossessor" => render::<phrase::HeaderPossessor>,
        "Duration" => render::<triggers::Duration>,
        "Timing" => render::<triggers::Timing>,
        "UsageLimit" => render::<triggers::UsageLimit>,
        "Instruction" => render::<abilities::Instruction>,
        "StaticSpec" => render::<abilities::StaticSpec>,
        "Cost" => render::<abilities::Cost>,
        "Ability" => render::<abilities::Ability>,
        "TokenSpec" => render::<abilities::TokenSpec>,
        "CounterKindSource" => render::<abilities::CounterKindSource>,
        "CharacteristicBundle" => render::<abilities::CharacteristicBundle>,
        "Window" => render::<words::Window>,
        "ManaSymbol" => render::<words::ManaSymbol>,
        "SimpleManaSymbol" => render::<words::SimpleManaSymbol>,
        "ColorOrColorless" => render::<words::ColorOrColorless>,
        "Color" => render::<words::Color>,
        "CounterKind" => render::<words::CounterKind>,
        "DesignationLabel" => render::<words::DesignationLabel>,
        "NounWord" => render::<words::NounWord>,
        "CardType" => render::<words::CardType>,
        "Deed" => render::<words::Deed>,
        "CoreDeed" => render::<words::CoreDeed>,
        "Subtype" => render::<words::Subtype>,
        "TurnPart" => render::<words::TurnPart>,
        "Disclosure" => render::<words::Disclosure>,
        // The param type of the same name is `Delta<Amount>` (`ron::param_types`).
        "Delta" => render::<words::Delta<phrase::Amount>>,
        "LevelBand" => render::<card::LevelBand>,
        "PrototypeFrame" => render::<card::PrototypeFrame>,
        "Conferral" => render::<rules::Conferral>,
        _ => return None,
    })
}

/// The `Definition` constructors: a body headed by one is a registry node.
const DEFINITION_HEADS: &[&str] = &["Counter", "Subtype", "Designation"];

fn expand_one(macros: &MacroSet, samples: &Samples, declaration: &Declaration) -> Outcome {
    let definition = &declaration.definition;
    let name = definition.name.as_str();
    let kinds: Vec<&str> = definition
        .kinds
        .iter()
        .map(macro_ron::Ident::as_str)
        .collect();
    let header = |read_at: &str, invocation: &str| {
        format!(
            "// declaration: {name} [{}]\n// read as: {read_at}\n// invocation: {invocation}\n",
            kinds.join(", ")
        )
    };
    let has_params = match &definition.params {
        Params::Positional(params) => !params.is_empty(),
        Params::Named(params) => !params.is_empty(),
    };

    if definition.body() == "()" {
        return Outcome::Skipped(
            "bodyless: the body is `()`, the meta-macro's omitted-argument default".to_owned(),
        );
    }
    if kinds.contains(&deckmaste_semantics_v2::ron::MACRO_KIND) {
        return Outcome::Skipped(
            "meta-macro: its body is a definition template, not a term; the declarations it \
             produces are printed in its place"
                .to_owned(),
        );
    }

    // A registry node — read the body itself, not the term it denotes.
    let head = definition.body_head(macros);
    let node_reader: Option<(&str, Renderer)> =
        if head.is_some_and(|head| DEFINITION_HEADS.contains(&head.as_str())) {
            Some(("Definition (the body)", render::<Definition>))
        } else if kinds == ["Designation"] {
            Some(("Vec<Definition> (the body)", render::<Vec<Definition>>))
        } else {
            None
        };
    if let Some((read_at, reader)) = node_reader {
        if has_params {
            return Outcome::Skipped(format!(
                "registry node body with parameters: read directly, so its holes cannot be \
                 filled (read as {read_at})"
            ));
        }
        return match reader(macros, definition.body()) {
            Ok(text) => Outcome::Printed(header(read_at, "(the body, read directly)") + &text),
            Err(error) => Outcome::Failed(format!("reading the body as {read_at}: {error}")),
        };
    }

    let Some((kind, reader)) = kinds
        .iter()
        .find_map(|kind| renderer(kind).map(|reader| (*kind, reader)))
    else {
        return Outcome::Skipped(format!(
            "no kind of [{}] is a v2 syntax type, and the body (`{}`) is not a registry node",
            kinds.join(", "),
            head.map_or("…", |head| head.as_str())
        ));
    };

    match instantiate(samples, name, &definition.params, |invocation| {
        reader(macros, invocation)
    }) {
        Ok(Instantiated { invocation, value }) => {
            Outcome::Printed(header(kind, &invocation) + &value)
        }
        Err(Uninstantiated::NoSlots(reason)) => Outcome::Skipped(reason),
        Err(Uninstantiated::NoneRead {
            attempts,
            first_error,
            any_untyped,
        }) => {
            if has_params {
                // Listed, not failed: the samples may simply not fit (an `Any`
                // hole at a word type no sample covers), or the signature may
                // admit no argument at all. A declaration that printed before
                // and is listed here after a re-spelling still shows in
                // `diff -r`, as a missing `.expanded` file.
                Outcome::Skipped(format!(
                    "no sample instantiation reads at {kind} ({attempts} tried{}); first: \
                     {first_error}",
                    if any_untyped { "" } else { ", every parameter typed" }
                ))
            } else {
                Outcome::Failed(format!("reading at {kind}: {first_error}"))
            }
        }
    }
}

/// A declaration invoked with sample arguments, and what the invocation read
/// as.
pub struct Instantiated<R> {
    /// The invocation text that read.
    pub invocation: String,
    /// What it read as.
    pub value: R,
}

/// Why no sample invocation of a declaration read.
pub enum Uninstantiated {
    /// A parameter type no sample fills, or that is not registered.
    NoSlots(String),
    /// Every distinct assignment tried was refused.
    NoneRead {
        attempts: usize,
        first_error: String,
        any_untyped: bool,
    },
}

/// Invokes the declaration `name` with its deterministic sample arguments —
/// the first assignment, in [`SAMPLES`] order with no two parameters given
/// the same text, that `read` accepts. This is the instantiation every
/// `.expanded` file is printed from; `cargo xtask facts` reads keyword
/// definitions through it too.
///
/// # Errors
/// [`Uninstantiated`] when no assignment reads.
pub fn instantiate<R>(
    samples: &Samples,
    name: &str,
    params: &Params,
    read: impl Fn(&str) -> Result<R, String>,
) -> Result<Instantiated<R>, Uninstantiated> {
    let slots = samples.slots(params).map_err(Uninstantiated::NoSlots)?;
    let any_untyped = slots.iter().any(|slot| slot.untyped);
    let mut first_error = None;
    let mut attempts = 0usize;
    let mut choice = vec![0usize; slots.len()];
    loop {
        let texts: Vec<&str> = slots
            .iter()
            .zip(&choice)
            .map(|(slot, &i)| slot.candidates[i])
            .collect();
        let distinct = texts
            .iter()
            .enumerate()
            .all(|(i, text)| !texts[..i].contains(text));
        if distinct {
            attempts += 1;
            let invocation = invocation(name, params, &slots, &texts);
            match read(&invocation) {
                Ok(value) => return Ok(Instantiated { invocation, value }),
                Err(error) => {
                    first_error.get_or_insert_with(|| format!("`{invocation}`: {error}"));
                }
            }
            if attempts >= MAX_ATTEMPTS {
                break;
            }
        }
        if !advance(&mut choice, &slots) {
            break;
        }
    }
    Err(Uninstantiated::NoneRead {
        attempts,
        first_error: first_error.unwrap_or_else(|| "no distinct sample assignment".to_owned()),
        any_untyped,
    })
}

/// Steps `choice` to the next assignment in lexicographic order (last slot
/// fastest); `false` once every assignment has been visited.
fn advance(choice: &mut [usize], slots: &[Slot]) -> bool {
    for (i, slot) in slots.iter().enumerate().rev() {
        choice[i] += 1;
        if choice[i] < slot.candidates.len() {
            return true;
        }
        choice[i] = 0;
    }
    false
}

/// The invocation text: the bare name for an empty signature, else the name
/// applied to the supplied arguments — positionally for a positional
/// signature, by name for a named one.
fn invocation(name: &str, params: &Params, slots: &[Slot], texts: &[&str]) -> String {
    let empty = match params {
        Params::Positional(params) => params.is_empty(),
        Params::Named(params) => params.is_empty(),
    };
    if empty {
        return name.to_owned();
    }
    let arguments: Vec<String> = slots
        .iter()
        .zip(texts)
        .map(|(slot, text)| match &slot.key {
            Some(key) => format!("{key}: {text}"),
            None => (*text).to_owned(),
        })
        .collect();
    format!("{name}({})", arguments.join(", "))
}

/// One parameter to fill: its key (named signatures) and the samples it may
/// take, in preference order.
struct Slot {
    key: Option<String>,
    candidates: Vec<&'static str>,
    untyped: bool,
}

/// [`SAMPLES`] with each param type's validator, against one macro scope.
pub struct Samples<'m> {
    macros: &'m MacroSet,
    types: macro_ron::ParamTypeSet,
}

impl<'m> Samples<'m> {
    /// The samples over `macros`, the scope the invocations are read in.
    #[must_use]
    pub fn new(macros: &'m MacroSet) -> Self {
        Samples {
            macros,
            types: deckmaste_semantics_v2::ron::param_types(),
        }
    }

    /// The samples a param of type `name` accepts, in [`SAMPLES`] order.
    fn accepted(&self, name: &str) -> Result<Vec<&'static str>, String> {
        let validator = self
            .types
            .get(name)
            .ok_or_else(|| format!("param type `{name}` is not registered"))?;
        Ok(SAMPLES
            .iter()
            .copied()
            .filter(|sample| validator(sample, self.macros, false).is_ok())
            .collect())
    }

    /// The slots to fill for `params`: defaulted and elidable params are left
    /// to their default; each other param's candidates start at its ordinal
    /// among params of the same type, so same-typed params differ.
    fn slots(&self, params: &Params) -> Result<Vec<Slot>, String> {
        let entries: Vec<(Option<String>, &ParamType)> = match params {
            Params::Positional(params) => params.iter().map(|ty| (None, ty)).collect(),
            Params::Named(params) => params
                .iter()
                .map(|(key, ty)| (Some(key.as_str().to_owned()), ty))
                .collect(),
        };
        let mut ordinals: BTreeMap<String, usize> = BTreeMap::new();
        let mut slots = Vec::new();
        for (key, ty) in entries {
            if ty.default.is_some() || ty.elidable {
                continue;
            }
            let type_name = ty.name.as_str();
            let untyped = type_name == "Any";
            let mut candidates = if untyped {
                WORD_SAMPLES.iter().chain(SAMPLES).copied().collect()
            } else {
                self.accepted(type_name)?
            };
            if candidates.is_empty() {
                return Err(format!("no sample reads as param type `{type_name}`"));
            }
            let ordinal = ordinals.entry(type_name.to_owned()).or_default();
            let len = candidates.len();
            candidates.rotate_left(*ordinal % len);
            *ordinal += 1;
            slots.push(Slot {
                key,
                candidates,
                untyped,
            });
        }
        Ok(slots)
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    fn builtin() -> Plugin {
        Plugin::load(default_plugin_dir()).expect("plugins_v2/builtin loads")
    }

    /// Every kind the v2 dialect registers, other than the declaration
    /// positions, has a renderer — so no declaration is skipped for want of
    /// one as the mirror grows.
    #[test]
    fn every_syntax_kind_has_a_renderer() {
        // The family kinds that are ALSO a syntax type's serde name, and so
        // a real position (`ron::DECLARATION_KINDS`'s own doc).
        const TYPED_FAMILIES: &[&str] = &["CounterKind", "Subtype", "TurnPart"];
        for kind in deckmaste_semantics_v2::ron::kinds().iter() {
            let name = kind.name();
            let loader_tag = name == deckmaste_semantics_v2::ron::MACRO_KIND
                || (deckmaste_semantics_v2::ron::DECLARATION_KINDS.contains(&name)
                    && !TYPED_FAMILIES.contains(&name));
            assert_eq!(
                renderer(name).is_some(),
                !loader_tag,
                "kind `{name}`: a syntax kind needs a renderer, a loader tag has none"
            );
        }
    }

    /// Every sample reads as some registered param type: a typo would
    /// otherwise silently drop out of every candidate list.
    #[test]
    fn every_sample_reads_as_some_param_type() {
        let plugin = builtin();
        let samples = Samples::new(&plugin.macros);
        let names: Vec<String> = samples.types.names().map(str::to_owned).collect();
        let unread: Vec<&str> = SAMPLES
            .iter()
            .copied()
            .filter(|sample| *sample != "None")
            .filter(|sample| {
                !names
                    .iter()
                    .filter(|name| name.as_str() != "Any")
                    .any(|name| {
                        samples.types.get(name).is_some_and(|validator| {
                            validator(sample, &plugin.macros, false).is_ok()
                        })
                    })
            })
            .collect();
        assert!(
            unread.is_empty(),
            "samples reading as no param type: {unread:?}"
        );
    }

    /// The canonical-printing property on the case that motivated the
    /// command: amass's body spelled over helper macros and the same body
    /// spelled in raw constructors print identically, and printing is not
    /// fooled into equality — a changed amount prints differently.
    #[test]
    fn a_helper_spelling_and_its_constructor_spelling_print_the_same() {
        let plugin = builtin();
        let macros = &plugin.macros;
        let amass = plugin
            .declarations
            .values()
            .find(|declaration| declaration.definition.name.as_str() == "amass")
            .expect("builtin declares amass");
        let Outcome::Printed(printed) = expand_one(macros, &Samples::new(macros), amass) else {
            panic!("amass prints");
        };
        let invocation = printed
            .lines()
            .find_map(|line| line.strip_prefix("// invocation: "))
            .expect("the header names the invocation");
        assert_eq!(
            invocation,
            r#"amass(Of(host: Creature, label: "Sample"), 3)"#
        );
        let raw = |amount: u32| {
            format!(
                r#"Enact(verb: Action("Amass"), instruction: Sequentially([
                DoIf(
                    condition: Not(Exists(Described(
                        determiner: Bare,
                        predicate: And([
                            HasSubtype(Of(host: Creature, label: "Army")),
                            HasType(Creature),
                            HasPossessor(axis: Controller, possessor: Actor),
                        ]),
                    ))),
                    instruction: CreateObject(
                        count: 1,
                        spec: Token(
                            spec: Written((characteristics: (
                                colors: [Black],
                                types: [Creature],
                                subtypes: [Of(host: Creature, label: "Sample"), Of(host: Creature, label: "Army")],
                                power: 0,
                                toughness: 0,
                            ))),
                            riders: [],
                        ),
                    ),
                    otherwise: None,
                ),
                Choose(
                    first: None,
                    chosen: Described(
                        determiner: A(Unmarked),
                        predicate: And([
                            HasSubtype(Of(host: Creature, label: "Army")),
                            HasType(Creature),
                            HasPossessor(axis: Controller, possessor: Actor),
                        ]),
                    ),
                    disclosure: Openly,
                    when: None,
                    agent: Some(Actor),
                ),
                PutCounters(
                    amount: Lit(value: {amount}),
                    kind: Printed(Named(name: "p1p1Counter")),
                    on: Pro(reach: Word(Type(Creature)), plurality: One, window: Whole),
                ),
                DoIf(
                    condition: Not(Matches(
                        subject: Pro(reach: Bare, plurality: One, window: Whole),
                        predicate: HasSubtype(Of(host: Creature, label: "Sample")),
                    )),
                    instruction: Establish(
                        spec: CharacteristicChange(
                            subject: Pro(reach: Bare, plurality: One, window: Whole),
                            edits: [
                                TypeLine(op: Adds, changes: (subtypes: [Of(host: Creature, label: "Sample")])),
                            ],
                        ),
                        duration: None,
                    ),
                    otherwise: None,
                ),
            ]))"#
            )
        };
        let body = |text: &str| {
            text.lines()
                .filter(|line| !line.starts_with("// "))
                .collect::<Vec<_>>()
                .join("\n")
        };
        let constructors = render::<abilities::Instruction>(macros, &raw(3))
            .expect("the constructor spelling reads");
        assert_eq!(body(&printed), body(&constructors));
        let other = render::<abilities::Instruction>(macros, &raw(4))
            .expect("the constructor spelling reads");
        assert_ne!(body(&printed), body(&other));
    }

    /// Every `Any`-only sample is well-formed RON, so a typo cannot hide as
    /// a candidate that never reads.
    #[test]
    fn every_word_sample_is_ron() {
        for sample in WORD_SAMPLES {
            ::ron::from_str::<::ron::Value>(sample)
                .unwrap_or_else(|error| panic!("word sample `{sample}` is not RON: {error}"));
        }
    }

    /// Two same-typed params get different samples.
    #[test]
    fn same_typed_params_get_distinct_samples() {
        let plugin = builtin();
        let samples = Samples::new(&plugin.macros);
        let params =
            Params::Positional(vec![ParamType::plain("Amount"), ParamType::plain("Amount")]);
        let slots = samples.slots(&params).expect("Amount has samples");
        assert_ne!(slots[0].candidates[0], slots[1].candidates[0]);
    }
}
