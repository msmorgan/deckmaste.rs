//! `cargo xtask lean macros` — the Lean macro layer derived from the
//! `plugins_v2/builtin` declarations (`docs/decisions/semantics-v2.md` §6,
//! §12.1; ticket `lean-macros-from-ron`).
//!
//! Three subcommands:
//!  - `generate` writes [`GENERATED`], one `semantic_macro` per declaration,
//!    in the namespace [`NAMESPACE`];
//!  - `check` fails when that file is stale, as `facts check` does;
//!  - `align` proves, for every hand-written `Semantics.Macros` macro whose
//!    name a declaration also has, that the two are the same function, and
//!    names every one that is not (the bench drifting from its RON target).
//!
//! **How a declaration becomes Lean.** `macro_ron` is a dumb substituter
//! (§12.2) and the loader's devices rewrite a body into constants and holes,
//! so the expansion of a declaration is its body with each argument spliced
//! in. The generator reads the declaration at its own position with sample
//! arguments ([`crate::expansions::SAMPLES`]), renders the typed value as
//! Lean with the gate's own renderer ([`lean_emit::term`]), then re-reads it
//! once per parameter with only that argument changed: the subterms that
//! changed are the parameter's holes. A parameter with a default is read once
//! more without it, which is what its default renders as. The Lean body is
//! the first rendering with each hole replaced by the binder's name, so it is
//! written in the constructor basis, fully expanded, exactly as the gate sees
//! the declaration.
//!
//! A declaration that cannot be derived — no sample reads, an untyped
//! parameter the body never uses, a default that names a sibling — is listed
//! at the foot of the file with the reason, never dropped.

use std::collections::BTreeMap;
use std::collections::BTreeSet;
use std::fmt::Write as _;
use std::fs;
use std::ops::Range;
use std::path::Path;
use std::path::PathBuf;
use std::process::Command;

use anyhow::Context as _;
use clap::Args;
use clap::Subcommand;
use deckmaste_semantics_v2::abilities;
use deckmaste_semantics_v2::card;
use deckmaste_semantics_v2::lean_emit;
use deckmaste_semantics_v2::phrase;
use deckmaste_semantics_v2::reader::Declaration;
use deckmaste_semantics_v2::reader::Plugin;
use deckmaste_semantics_v2::rules;
use deckmaste_semantics_v2::triggers;
use deckmaste_semantics_v2::words;
use macro_ron::MacroSet;
use macro_ron::ParamType;
use macro_ron::Params;
use rayon::iter::IntoParallelIterator as _;
use rayon::iter::ParallelIterator as _;
use serde::Serialize;
use serde::de::DeserializeOwned;

use crate::expansions::SAMPLES;
use crate::expansions::WORD_SAMPLES;

/// The generated module, relative to the repository root.
pub const GENERATED: &str = "lean/Semantics/Macros/Declared.lean";
/// The Lean namespace the generated macros live in.
pub const NAMESPACE: &str = "Semantics.Macros.Declared";
/// The hand-written macro layer the alignment run compares against.
const HAND: &str = "lean/Semantics/Macros.lean";
/// The alignment run's untracked library: root module and directory,
/// relative to `lean/`, and its `lake` target.
const ALIGN_ROOT: &str = "GeneratedMacroAlignment.lean";
const ALIGN_TARGET: &str = "GeneratedMacroAlignment";

#[derive(Debug, Args)]
pub struct LeanMacrosArgs {
    #[command(subcommand)]
    command: LeanMacrosCmd,
}

#[derive(Debug, Subcommand)]
enum LeanMacrosCmd {
    /// Write `lean/Semantics/Macros/Declared.lean` from the declarations.
    Generate,
    /// Fail if `lean/Semantics/Macros/Declared.lean` is stale.
    Check,
    /// Prove every hand macro with a declared namesake equal to it, and name
    /// every one that is not (builds an untracked Lean library).
    Align {
        /// Fail when any hand macro drifts (the gate, once the bench is cut
        /// over); without it the drift is reported and the run succeeds.
        #[arg(long)]
        deny_drift: bool,
    },
}

/// # Errors
/// If the plugin fails to load, a file cannot be written, the generated file
/// is stale (`check`), or `lake` cannot run (`align`).
pub fn run(args: &LeanMacrosArgs) -> anyhow::Result<()> {
    let root = repo_root();
    match args.command {
        LeanMacrosCmd::Generate => {
            let module = render(&root)?;
            let path = root.join(GENERATED);
            fs::write(&path, &module.text).with_context(|| format!("writing {GENERATED}"))?;
            println!(
                "wrote {GENERATED}: {} macros, {} not derived",
                module.derived.len(),
                module.skipped.len()
            );
            Ok(())
        }
        LeanMacrosCmd::Check => run_check(&root),
        LeanMacrosCmd::Align { deny_drift } => run_align(&root, deny_drift),
    }
}

fn repo_root() -> PathBuf {
    Path::new(env!("CARGO_MANIFEST_DIR")).join("../..")
}

fn run_check(root: &Path) -> anyhow::Result<()> {
    let path = root.join(GENERATED);
    let committed =
        fs::read_to_string(&path).with_context(|| format!("reading {}", path.display()))?;
    anyhow::ensure!(
        committed == render(root)?.text,
        "{GENERATED} is stale: re-run `cargo xtask lean macros generate`"
    );
    println!("{GENERATED} is up to date");
    Ok(())
}

// ---------------------------------------------------------------------------
// Readers
// ---------------------------------------------------------------------------

type LeanReader = fn(&MacroSet, &str) -> Result<String, String>;

/// Reads `source` as `T` and renders it as a Lean term.
fn read_lean<T: DeserializeOwned + Serialize>(
    macros: &MacroSet,
    source: &str,
) -> Result<String, String> {
    let value: T = macros.read_str(source).map_err(|error| error.to_string())?;
    lean_emit::term(&value).map_err(|error| error.to_string())
}

/// The Lean type of a macro of kind `kind`, and its reader — the same kind
/// set `crate::expansions` prints (`every_syntax_kind_has_a_lean_reader`).
fn kind_reader(kind: &str) -> Option<(&'static str, LeanReader)> {
    Some(match kind {
        "GameEvent" => ("Semantics.GameEvent", read_lean::<phrase::GameEvent>),
        "NounPhrase" => ("Semantics.NounPhrase", read_lean::<phrase::NounPhrase>),
        "Predicate" => ("Semantics.Predicate", read_lean::<phrase::Predicate>),
        "Amount" => ("Semantics.Amount", read_lean::<phrase::Amount>),
        "Quantity" => ("Semantics.Quantity", read_lean::<phrase::Quantity>),
        "ZoneExpr" => ("Semantics.ZoneExpr", read_lean::<phrase::ZoneExpr>),
        "Pile" => ("Semantics.Pile", read_lean::<phrase::Pile>),
        "Condition" => ("Semantics.Condition", read_lean::<phrase::Condition>),
        "ColorTerm" => ("Semantics.ColorTerm", read_lean::<phrase::ColorTerm>),
        "Ballot" => ("Semantics.Ballot", read_lean::<phrase::Ballot>),
        "Duration" => ("Semantics.Duration", read_lean::<phrase::Duration>),
        "Timing" => ("Semantics.Timing", read_lean::<phrase::Timing>),
        "UsageLimit" => ("Semantics.UsageLimit", read_lean::<triggers::UsageLimit>),
        "Instruction" => ("Semantics.Instruction", read_lean::<abilities::Instruction>),
        "StaticSpec" => ("Semantics.StaticSpec", read_lean::<abilities::StaticSpec>),
        "Cost" => ("Semantics.Cost", read_lean::<abilities::Cost>),
        "Ability" => ("Semantics.Ability", read_lean::<abilities::Ability>),
        "TokenSpec" => ("Semantics.TokenSpec", read_lean::<abilities::TokenSpec>),
        "CounterKindSource" => (
            "Semantics.CounterKindSource",
            read_lean::<abilities::CounterKindSource>,
        ),
        "CharacteristicBundle" => (
            "Semantics.CharacteristicBundle",
            read_lean::<abilities::CharacteristicBundle>,
        ),
        "PipPaid" => ("Semantics.PipPaid", read_lean::<abilities::PipPaid>),
        "ManaSubstitute" => (
            "Semantics.ManaSubstitute",
            read_lean::<abilities::ManaSubstitute>,
        ),
        "FaceDown" => ("Semantics.FaceDown", read_lean::<abilities::FaceDown>),
        "MoveRider" => ("Semantics.MoveRider", read_lean::<abilities::MoveRider>),
        "EntryRider" => ("Semantics.EntryRider", read_lean::<abilities::EntryRider>),
        "Window" => ("Semantics.Window", read_lean::<words::Window>),
        "ManaSymbol" => ("Semantics.ManaSymbol", read_lean::<words::ManaSymbol>),
        "SimpleManaSymbol" => (
            "Semantics.SimpleManaSymbol",
            read_lean::<words::SimpleManaSymbol>,
        ),
        "ColorOrColorless" => (
            "Semantics.ColorOrColorless",
            read_lean::<words::ColorOrColorless>,
        ),
        "Color" => ("Semantics.Color", read_lean::<words::Color>),
        "CounterKind" => ("Semantics.CounterKind", read_lean::<words::CounterKind>),
        "DesignationLabel" => (
            "Semantics.DesignationLabel",
            read_lean::<words::DesignationLabel>,
        ),
        "KeywordLabel" => ("Semantics.KeywordLabel", read_lean::<words::KeywordLabel>),
        "NounWord" => ("Semantics.NounWord", read_lean::<words::NounWord>),
        "CardType" => ("Semantics.CardType", read_lean::<words::CardType>),
        "Deed" => ("Semantics.Deed", read_lean::<words::Deed>),
        "CoreDeed" => ("Semantics.CoreDeed", read_lean::<words::CoreDeed>),
        "Subtype" => ("Semantics.Subtype", read_lean::<words::Subtype>),
        "TurnPart" => ("Semantics.TurnPart", read_lean::<words::TurnPart>),
        "Disclosure" => ("Semantics.Disclosure", read_lean::<words::Disclosure>),
        "Delta" => (
            "(Semantics.Delta Semantics.Amount)",
            read_lean::<words::Delta<phrase::Amount>>,
        ),
        "LevelBand" => ("Semantics.LevelBand", read_lean::<card::LevelBand>),
        "PrototypeFrame" => (
            "Semantics.PrototypeFrame",
            read_lean::<card::PrototypeFrame>,
        ),
        "Conferral" => ("Semantics.Conferral", read_lean::<rules::Conferral>),
        _ => return None,
    })
}

/// The Lean binder type of a parameter of the registered param type `name`,
/// and the reader that renders an argument of it — every name
/// `deckmaste_semantics_v2::ron::param_types` registers except `Any`
/// (`every_param_type_has_a_lean_reader`).
#[allow(clippy::too_many_lines, reason = "one arm per registered param type")]
fn param_reader(name: &str) -> Option<(&'static str, LeanReader)> {
    use deckmaste_semantics_v2::events;
    Some(match name {
        "String" | "AbilityWordLabel" | "FlavorWordLabel" => ("String", read_lean::<String>),
        "Nat" => ("Nat", read_lean::<u32>),
        "Power" | "Toughness" | "Amount" => ("Semantics.Amount", read_lean::<phrase::Amount>),
        "Quality" | "Predicate" => ("Semantics.Predicate", read_lean::<phrase::Predicate>),
        "Subject" | "NounPhrase" => ("Semantics.NounPhrase", read_lean::<phrase::NounPhrase>),
        "SearchScope" => ("Semantics.SearchScope", read_lean::<phrase::SearchScope>),
        "TurnRef" => ("Semantics.TurnRef", read_lean::<phrase::TurnRef>),
        "Abilities" => (
            "(List Semantics.Ability)",
            read_lean::<Vec<abilities::Ability>>,
        ),
        "AsThough" => ("Semantics.AsThough", read_lean::<abilities::AsThough>),
        "Compulsion" => ("Semantics.Compulsion", read_lean::<abilities::Compulsion>),
        "DamageScope" => ("Semantics.DamageScope", read_lean::<abilities::DamageScope>),
        "DeckCondition" => (
            "Semantics.DeckCondition",
            read_lean::<abilities::DeckCondition>,
        ),
        "DeonticPatient" => (
            "Semantics.DeonticPatient",
            read_lean::<abilities::DeonticPatient>,
        ),
        "DeonticRider" => (
            "Semantics.DeonticRider",
            read_lean::<abilities::DeonticRider>,
        ),
        "PipPaids" => (
            "(List Semantics.PipPaid)",
            read_lean::<Vec<abilities::PipPaid>>,
        ),
        "SpendPurpose" => (
            "Semantics.SpendPurpose",
            read_lean::<abilities::SpendPurpose>,
        ),
        "StaticSpecs" => (
            "(List Semantics.StaticSpec)",
            read_lean::<Vec<abilities::StaticSpec>>,
        ),
        "EntryRiders" => (
            "(List Semantics.EntryRider)",
            read_lean::<Vec<abilities::EntryRider>>,
        ),
        "MoveRiders" => (
            "(List Semantics.MoveRider)",
            read_lean::<Vec<abilities::MoveRider>>,
        ),
        "ChoiceDomain" => ("Semantics.ChoiceDomain", read_lean::<phrase::ChoiceDomain>),
        "AggregateOp" => ("Semantics.AggregateOp", read_lean::<words::AggregateOp>),
        "Arrangement" => ("Semantics.Arrangement", read_lean::<words::Arrangement>),
        "Colors" => ("(List Semantics.Color)", read_lean::<Vec<words::Color>>),
        "Comparator" => ("Semantics.Comparator", read_lean::<words::Comparator>),
        "CounterBatch" => ("Semantics.CounterBatch", read_lean::<events::CounterBatch>),
        "CounterMove" => ("Semantics.CounterMove", read_lean::<events::CounterMove>),
        "DamageKind" => ("Semantics.DamageKind", read_lean::<events::DamageKind>),
        "Deeds" => ("(List Semantics.Deed)", read_lean::<Vec<words::Deed>>),
        "GameEvents" => (
            "(List Semantics.GameEvent)",
            read_lean::<Vec<phrase::GameEvent>>,
        ),
        "JoinedHeaders" => (
            "(List Semantics.JoinedHeader)",
            read_lean::<Vec<triggers::JoinedHeader>>,
        ),
        "Kind" => ("Semantics.Kind", read_lean::<words::Kind>),
        "LevelRange" => ("Semantics.LevelRange", read_lean::<card::LevelRange>),
        "EventSide" => ("Semantics.EventSide", read_lean::<words::EventSide>),
        "ManaCost" => (
            "(List Semantics.ManaSymbol)",
            read_lean::<Vec<words::ManaSymbol>>,
        ),
        "ManaMatch" => ("Semantics.ManaMatch", read_lean::<words::ManaMatch>),
        "NounWord" => ("Semantics.NounWord", read_lean::<words::NounWord>),
        "Ordinal" => ("Semantics.Ordinal", read_lean::<words::Ordinal>),
        "PaidCostName" => ("Semantics.PaidCostName", read_lean::<words::PaidCostName>),
        "Plurality" => ("Semantics.Plurality", read_lean::<words::Plurality>),
        "ProjAxis" => ("Semantics.ProjAxis", read_lean::<words::ProjAxis>),
        "QualitySort" => ("Semantics.QualitySort", read_lean::<words::QualitySort>),
        "Period" => ("Semantics.Period", read_lean::<words::Period>),
        "CountBound" => ("Semantics.CountBound", read_lean::<triggers::CountBound>),
        "Role" => ("Semantics.Role", read_lean::<words::Role>),
        "Stat" => ("Semantics.Stat", read_lean::<words::Stat>),
        "Subtypes" => ("(List Semantics.Subtype)", read_lean::<Vec<words::Subtype>>),
        "VerbedMarking" => ("Semantics.VerbedMarking", read_lean::<words::VerbedMarking>),
        other => return kind_reader(other),
    })
}

/// The reader for a rendered Lean type, where one of the tables above has it.
fn reader_for_lean_type(lean_type: &str) -> Option<LeanReader> {
    const NAMES: &[&str] = &[
        "GameEvent",
        "NounPhrase",
        "Predicate",
        "Amount",
        "Quantity",
        "ZoneExpr",
        "Pile",
        "Condition",
        "ColorTerm",
        "Ballot",
        "Duration",
        "Timing",
        "UsageLimit",
        "Instruction",
        "StaticSpec",
        "Cost",
        "Ability",
        "TokenSpec",
        "CounterKindSource",
        "CharacteristicBundle",
        "PipPaid",
        "ManaSubstitute",
        "FaceDown",
        "MoveRider",
        "EntryRider",
        "Window",
        "ManaSymbol",
        "SimpleManaSymbol",
        "ColorOrColorless",
        "Color",
        "CounterKind",
        "DesignationLabel",
        "KeywordLabel",
        "NounWord",
        "CardType",
        "Deed",
        "CoreDeed",
        "Subtype",
        "TurnPart",
        "Disclosure",
        "LevelBand",
        "PrototypeFrame",
        "Conferral",
        "SearchScope",
        "TurnRef",
        "AsThough",
        "Compulsion",
        "DamageScope",
        "DeckCondition",
        "DeonticPatient",
        "DeonticRider",
        "SpendPurpose",
        "ChoiceDomain",
        "AggregateOp",
        "Arrangement",
        "Comparator",
        "CounterBatch",
        "CounterMove",
        "DamageKind",
        "Kind",
        "LevelRange",
        "EventSide",
        "ManaMatch",
        "Ordinal",
        "PaidCostName",
        "Plurality",
        "ProjAxis",
        "QualitySort",
        "Period",
        "CountBound",
        "Role",
        "Stat",
        "VerbedMarking",
    ];
    let name = lean_type.strip_prefix("Semantics.")?;
    NAMES
        .contains(&name)
        .then(|| param_reader(name).map(|(_, reader)| reader))?
}

// ---------------------------------------------------------------------------
// The term tree
// ---------------------------------------------------------------------------

/// A rendered Lean term read back as a tree of balanced groups and atoms, so
/// two renderings can be compared position by position.
#[derive(Debug)]
enum Node {
    Atom(Range<usize>),
    Group {
        span: Range<usize>,
        children: Vec<Node>,
    },
}

impl Node {
    fn span(&self) -> Range<usize> {
        match self {
            Node::Atom(span) | Node::Group { span, .. } => span.clone(),
        }
    }

    fn children(&self) -> &[Node] {
        match self {
            Node::Atom(_) => &[],
            Node::Group { children, .. } => children,
        }
    }

    fn at(&self, path: &[usize]) -> Option<&Node> {
        match path.split_first() {
            None => Some(self),
            Some((&first, rest)) => self.children().get(first)?.at(rest),
        }
    }
}

/// A rendering and its tree. The root is a group spanning the whole text.
struct Term {
    text: String,
    root: Node,
}

impl Term {
    fn parse(text: String) -> Result<Term, String> {
        let bytes = text.as_bytes();
        let mut stack: Vec<(usize, Vec<Node>)> = vec![(0, Vec::new())];
        let mut i = 0;
        while i < bytes.len() {
            let byte = bytes[i];
            match byte {
                b' ' | b'\n' | b'\t' => i += 1,
                b'(' | b'[' | b'{' => {
                    stack.push((i, Vec::new()));
                    i += 1;
                }
                b')' | b']' | b'}' => {
                    let (start, children) = stack.pop().ok_or("unbalanced rendering")?;
                    i += 1;
                    stack
                        .last_mut()
                        .ok_or("unbalanced rendering")?
                        .1
                        .push(Node::Group {
                            span: start..i,
                            children,
                        });
                }
                b'"' => {
                    let start = i;
                    i += 1;
                    while i < bytes.len() && bytes[i] != b'"' {
                        i += if bytes[i] == b'\\' { 2 } else { 1 };
                    }
                    i += 1;
                    stack.last_mut().unwrap().1.push(Node::Atom(start..i));
                }
                b',' => {
                    stack.last_mut().unwrap().1.push(Node::Atom(i..i + 1));
                    i += 1;
                }
                _ => {
                    let start = i;
                    while i < bytes.len()
                        && !matches!(
                            bytes[i],
                            b' ' | b'\n' | b'\t' | b'(' | b')' | b'[' | b']' | b'{' | b'}' | b','
                        )
                    {
                        i += 1;
                    }
                    stack.last_mut().unwrap().1.push(Node::Atom(start..i));
                }
            }
        }
        let (_, children) = stack.pop().ok_or("unbalanced rendering")?;
        if !stack.is_empty() {
            return Err("unbalanced rendering".to_owned());
        }
        let root = Node::Group {
            span: 0..text.len(),
            children,
        };
        Ok(Term { text, root })
    }

    fn slice(&self, node: &Node) -> &str {
        &self.text[node.span()]
    }

    /// The paths of the outermost subterms where `self` and `other` differ.
    fn differences(&self, other: &Term) -> Vec<Vec<usize>> {
        let mut out = Vec::new();
        // The root is the virtual group around the whole rendering, not a
        // constructor application: compare its children.
        let (mine, theirs) = (self.root.children(), other.root.children());
        if mine.len() != theirs.len() {
            return vec![Vec::new()];
        }
        for (i, (x, y)) in mine.iter().zip(theirs).enumerate() {
            let mut path = vec![i];
            diff(self, x, other, y, &mut path, &mut out);
        }
        out
    }
}

fn diff(a: &Term, x: &Node, b: &Term, y: &Node, path: &mut Vec<usize>, out: &mut Vec<Vec<usize>>) {
    if a.slice(x) == b.slice(y) {
        return;
    }
    let same_shape = match (x, y) {
        (Node::Group { children: cx, .. }, Node::Group { children: cy, .. }) => {
            // Another constructor is another subterm, even at the same arity.
            cx.len() == cy.len()
                && a.text.as_bytes()[x.span().start] == b.text.as_bytes()[y.span().start]
                && (a.text.as_bytes()[x.span().start] != b'('
                    || !matches!(cx.first(), Some(Node::Atom(_)))
                    || cx.first().map(|head| a.slice(head)) == cy.first().map(|head| b.slice(head)))
        }
        _ => false,
    };
    if same_shape {
        for (i, (cx, cy)) in x.children().iter().zip(y.children()).enumerate() {
            path.push(i);
            diff(a, cx, b, cy, path, out);
            path.pop();
        }
    } else {
        out.push(path.clone());
    }
}

/// Whitespace-insensitive text, for comparing a subterm with an argument
/// rendered on its own.
fn squash(text: &str) -> String {
    text.split_whitespace().collect::<Vec<_>>().join(" ")
}

// ---------------------------------------------------------------------------
// Derivation
// ---------------------------------------------------------------------------

/// One parameter, as the generator handles it.
struct Param {
    /// The RON key, for a named signature.
    key: Option<String>,
    ty: ParamType,
    /// The candidate argument texts, in preference order: the arguments the
    /// plugins pass it, then the fixed samples.
    candidates: Vec<String>,
}

/// One declaration derived as a Lean macro.
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct Derived {
    /// The declaration's name.
    pub name: String,
    /// The Lean identifier (the name, keyword-escaped).
    pub ident: String,
    /// The binders, rendered (`(p : Semantics.Predicate)`).
    pub binders: Vec<String>,
    /// The binder names in order, and whether the RON signature is named.
    pub binder_names: Vec<String>,
    pub named: bool,
    /// The Lean result type.
    pub result: String,
    /// The body.
    pub body: String,
    /// The source file, relative to the plugin's `macros/`.
    pub source: String,
}

/// The rendered module and what went into it.
pub struct Module {
    pub text: String,
    pub derived: Vec<Derived>,
    pub skipped: Vec<(String, String)>,
}

/// Words Lean reserves that a declaration or binder name may be.
const LEAN_KEYWORDS: &[&str] = &[
    "at",
    "by",
    "class",
    "do",
    "else",
    "end",
    "exists",
    "for",
    "from",
    "fun",
    "have",
    "if",
    "in",
    "instance",
    "let",
    "match",
    "mut",
    "open",
    "repeat",
    "return",
    "show",
    "then",
    "unless",
    "until",
    "where",
    "while",
    "with",
    "as",
    "def",
    "theorem",
    "example",
    "structure",
    "section",
    "namespace",
    "variable",
    "universe",
    "import",
    "abbrev",
    "deriving",
    "private",
    "protected",
    "partial",
    "calc",
    "suffices",
    "obtain",
    "forall",
    "Type",
    "Sort",
    "Prop",
    "this",
    "matches",
    // The macro layer's own binder modes (`Semantics.Authoring`).
    "splice",
    "capture",
];

/// A RON name as a Lean identifier: Lean's trailing-underscore escape on a
/// reserved word, as `Macros.lean` writes them (`exists_`, `by_`).
fn lean_ident(name: &str) -> String {
    if LEAN_KEYWORDS.contains(&name) { format!("{name}_") } else { name.to_owned() }
}

/// The declarations under `plugins_v2/builtin/macros/`, rendered as one Lean
/// module.
///
/// # Errors
/// If the plugin does not load.
pub fn render(root: &Path) -> anyhow::Result<Module> {
    let dir = root.join("plugins_v2/builtin");
    let plugin = Plugin::load(&dir).with_context(|| format!("loading {}", dir.display()))?;
    let macros_dir = plugin
        .root()
        .join(deckmaste_semantics_v2::reader::MACROS_DIR);
    let mut by_file: BTreeMap<&Path, &Declaration> = BTreeMap::new();
    for declaration in plugin.declarations.values() {
        by_file.entry(&declaration.path).or_insert(declaration);
    }
    let pool = variation_pool(root);
    let harvested = harvest_arguments(root);
    let words = lean_words(root, &plugin.macros);
    let outcomes: Vec<(String, String, Result<Derived, String>)> = by_file
        .into_par_iter()
        .map(|(path, declaration)| {
            let relative = path
                .strip_prefix(&macros_dir)
                .unwrap_or(path)
                .display()
                .to_string();
            let name = declaration.definition.name.as_str().to_owned();
            let outcome = derive(&plugin.macros, &pool, &harvested, &words, declaration).map(
                |mut derived| {
                    derived.source.clone_from(&relative);
                    derived
                },
            );
            (relative, name, outcome)
        })
        .collect();

    // Two declarations of one name (at different kinds) cannot share a Lean
    // name in one namespace: both are qualified by their result type.
    let mut counts: BTreeMap<String, usize> = BTreeMap::new();
    for (_, _, outcome) in &outcomes {
        if let Ok(derived) = outcome {
            *counts.entry(derived.ident.clone()).or_default() += 1;
        }
    }
    let mut derived = Vec::new();
    let mut skipped = Vec::new();
    for (relative, name, outcome) in outcomes {
        match outcome {
            Ok(mut one) => {
                if counts[&one.ident] > 1 {
                    let ty = one
                        .result
                        .trim_start_matches('(')
                        .trim_start_matches("Semantics.");
                    let ty = ty.split_whitespace().next().unwrap_or(ty);
                    one.ident = format!("{ty}.{}", one.ident);
                }
                derived.push(one);
            }
            Err(reason) => skipped.push((format!("{relative} ({name})"), reason)),
        }
    }
    derived.sort_by(|a, b| a.source.cmp(&b.source));

    let mut text = String::new();
    let _ = writeln!(
        text,
        "-- GENERATED by `cargo xtask lean macros generate` from \
         `plugins_v2/builtin/macros/`; do not edit.\n\
         -- `cargo xtask lean macros check` rejects a stale copy.\n\
         import Semantics.Macros.Primitives\n\n\
         /-!\n\
         # Semantics.Macros.Declared\n\n\
         One `semantic_macro` per `plugins_v2/builtin` macro declaration, derived from the \
         declaration\n\
         by `cargo xtask lean macros` (`docs/decisions/semantics-v2.md` §6, §12.1): the body is \
         the\n\
         declaration's own expansion, in the constructor basis, with each argument's subterms \
         replaced\n\
         by its binder. `cargo xtask lean macros align` proves a hand-written `Semantics.Macros` \
         macro\n\
         equal to its declared namesake here.\n\
         -/\n\n\
         set_option linter.unusedVariables false\n\n\
         namespace {NAMESPACE}\n"
    );
    for one in &derived {
        let _ = writeln!(text, "/-- `{}` -/", one.source);
        let mut head = format!("semantic_macro {}", one.ident);
        for binder in &one.binders {
            head.push(' ');
            head.push_str(binder);
        }
        let _ = writeln!(text, "{head} : {} :=\n  {}\n", one.result, one.body);
    }
    let _ = writeln!(text, "end {NAMESPACE}");
    if !skipped.is_empty() {
        let _ = writeln!(text, "\n/-\nNot derived ({}):", skipped.len());
        for (what, reason) in &skipped {
            let reason = reason.replace("-/", "- /").replace('\n', " ");
            let _ = writeln!(text, "- {what}: {reason}");
        }
        let _ = writeln!(text, "-/");
    }
    Ok(Module {
        text,
        derived,
        skipped,
    })
}

/// How many differing readings locate an untyped argument's holes.
const UNTYPED_READINGS: usize = 6;

/// The `Definition` constructors: a body headed by one is a registry node,
/// whose Lean form is `cargo xtask facts`'s, not a macro.
const DEFINITION_HEADS: &[&str] = &["Counter", "Subtype", "Designation"];

/// The candidate arguments for a parameter of type `name`.
fn candidates(macros: &MacroSet, name: &str) -> Result<Vec<&'static str>, String> {
    if name == "Any" {
        return Ok(WORD_SAMPLES.iter().chain(SAMPLES).copied().collect());
    }
    let types = deckmaste_semantics_v2::ron::param_types();
    let validator = types
        .get(name)
        .ok_or_else(|| format!("param type `{name}` is not registered"))?;
    let accepted: Vec<&'static str> = SAMPLES
        .iter()
        .copied()
        .filter(|sample| validator(sample, macros, false).is_ok())
        .collect();
    if accepted.is_empty() {
        return Err(format!("no sample reads as param type `{name}`"));
    }
    Ok(accepted)
}

/// The variation pool: every sample, every list of one sample, and every
/// capitalised word a `plugins_v2/builtin` declaration writes (the unit
/// constructors a word-typed `Any` hole takes), in a fixed order. Only a
/// parameter's SECOND reading draws on it, so its size costs one read per
/// entry at most.
fn variation_pool(root: &Path) -> Vec<String> {
    let mut pool: Vec<String> = WORD_SAMPLES
        .iter()
        .chain(SAMPLES)
        .map(|text| (*text).to_owned())
        .collect();
    pool.extend(
        SAMPLES
            .iter()
            .chain(WORD_SAMPLES)
            .map(|text| format!("[{text}]")),
    );
    let word = regex::Regex::new(r"\b[A-Z][A-Za-z0-9]*\b").expect("a valid pattern");
    let declared = regex::Regex::new(r#"name:\s*"([A-Za-z0-9_]+)""#).expect("a valid pattern");
    let mut words = BTreeSet::new();
    let mut stack = vec![root.join("plugins_v2/builtin/macros")];
    while let Some(dir) = stack.pop() {
        let Ok(entries) = fs::read_dir(&dir) else { continue };
        for entry in entries.flatten() {
            let path = entry.path();
            if path.is_dir() {
                stack.push(path);
            } else if let Ok(text) = fs::read_to_string(&path) {
                for found in word.find_iter(&text) {
                    words.insert(found.as_str().to_owned());
                }
                // The declaration's own name: a bare word like `flying` reads
                // where a keyword label, deed or counter kind is wanted.
                for found in declared.captures_iter(&text) {
                    words.insert(found[1].to_owned());
                }
            }
        }
    }
    // Every constructor the Lean syntax declares, as RON writes it: the unit
    // words an `Any` hole may take that no declaration happens to write.
    let constructor = regex::Regex::new(r"\|\s*([a-z][A-Za-z0-9_]*)").expect("a valid pattern");
    for file in [
        "Words",
        "Phrase",
        "Abilities",
        "Events",
        "Triggers",
        "Card",
        "Rules",
    ] {
        if let Ok(text) = fs::read_to_string(root.join(format!("lean/Semantics/{file}.lean"))) {
            for found in constructor.captures_iter(&text) {
                words.insert(lean_emit::rust_variant(&found[1]));
            }
        }
    }
    for reserved in ["Param", "Default", "Elidable", "Any"] {
        words.remove(reserved);
    }
    pool.extend(words);
    let mut seen = BTreeSet::new();
    pool.retain(|text| seen.insert(text.clone()));
    pool
}

/// The argument-free constructors of every Lean inductive the syntax files
/// declare, by type name, as RON writes them: what an `Any` hole whose
/// refusal names its type ("`x` is neither a variant of `T`") may take.
fn lean_words(root: &Path, macros: &MacroSet) -> BTreeMap<String, Vec<String>> {
    let header = regex::Regex::new(r"^\s*inductive\s+([A-Za-z0-9_]+)").expect("a valid pattern");
    let constructor =
        regex::Regex::new(r"\|\s*([a-z][A-Za-z0-9_]*)([^|]*)").expect("a valid pattern");
    let binder = regex::Regex::new(r"\(([^():]+):\s*([^()]+?)\)").expect("a valid pattern");
    let mut nullary: BTreeMap<String, Vec<String>> = BTreeMap::new();
    let mut applied: BTreeMap<String, Vec<(String, Vec<String>)>> = BTreeMap::new();
    let mut files = Vec::new();
    let mut stack = vec![root.join("lean/Semantics")];
    while let Some(dir) = stack.pop() {
        let Ok(entries) = fs::read_dir(&dir) else { continue };
        for entry in entries.flatten() {
            let path = entry.path();
            if path.is_dir() {
                stack.push(path);
            } else if path.extension().is_some_and(|ext| ext == "lean") {
                files.push(path);
            }
        }
    }
    files.sort();
    for path in files {
        let Ok(text) = fs::read_to_string(&path) else { continue };
        let mut current: Option<String> = None;
        for line in text.lines() {
            if let Some(found) = header.captures(line) {
                current = Some(found[1].to_owned());
                continue;
            }
            let Some(name) = &current else { continue };
            let code = line.split("--").next().unwrap_or("");
            let trimmed = code.trim_start();
            if trimmed.starts_with("deriving")
                || (!trimmed.starts_with('|') && !trimmed.is_empty() && !line.starts_with(' '))
            {
                current = None;
                continue;
            }
            if !trimmed.starts_with('|') {
                continue;
            }
            for found in constructor.captures_iter(code) {
                let word = lean_emit::rust_variant(&found[1]);
                let rest = found[2].trim();
                if rest.is_empty() {
                    nullary.entry(name.clone()).or_default().push(word);
                } else {
                    // One argument per binder name: `(a b : T)` is two.
                    let mut args = Vec::new();
                    for binding in binder.captures_iter(rest) {
                        let ty = binding[2].trim().to_owned();
                        for _ in binding[1].split_whitespace() {
                            args.push(ty.clone());
                        }
                    }
                    if !args.is_empty() {
                        applied.entry(name.clone()).or_default().push((word, args));
                    }
                }
            }
        }
    }
    // A sample for an argument type: a registered param type's first
    // sample, an inductive's first word, or the empty option and list.
    let sample = |ty: &str| -> Option<String> {
        let ty = ty.trim();
        if ty.starts_with("Option") {
            return Some("None".to_owned());
        }
        if ty.starts_with("List") {
            return Some("[]".to_owned());
        }
        match ty {
            "Nat" | "Int" => return Some("3".to_owned()),
            "String" => return Some("\"Sample\"".to_owned()),
            "Bool" => return Some("true".to_owned()),
            _ => {}
        }
        if let Ok(list) = candidates(macros, ty)
            && let Some(first) = list.first()
        {
            return Some((*first).to_owned());
        }
        nullary.get(ty).and_then(|words| words.first()).cloned()
    };
    let mut out = nullary.clone();
    for (ty, constructors) in &applied {
        for (word, args) in constructors {
            let filled: Option<Vec<String>> = args.iter().map(|arg| sample(arg)).collect();
            if let Some(filled) = filled {
                out.entry(ty.clone())
                    .or_default()
                    .push(format!("{word}({})", filled.join(", ")));
            }
        }
    }
    out
}

/// The type a refusal says it wanted where `text` stood.
fn wanted_type<'e>(error: &'e str, text: &str) -> Option<&'e str> {
    let marker = format!("`{text}` is neither a variant of `");
    let start = error.find(&marker)? + marker.len();
    let rest = &error[start..];
    Some(&rest[..rest.find('`')?])
}

/// Every argument a `plugins_v2` file passes to a macro, by macro name and
/// parameter (its key, or its position), in file order: the real arguments a
/// declaration is called with, tried before any fixed sample. An argument
/// that is itself a hole (`Param(…)`) is left out.
fn harvest_arguments(root: &Path) -> BTreeMap<String, BTreeMap<String, Vec<String>>> {
    let call = regex::Regex::new(r"\b([a-z][A-Za-z0-9_]*)\(").expect("a valid pattern");
    let keyed = regex::Regex::new(r"^\s*([a-z][A-Za-z0-9_]*)\s*:\s*").expect("a valid pattern");
    let mut out: BTreeMap<String, BTreeMap<String, Vec<String>>> = BTreeMap::new();
    let mut files = Vec::new();
    let mut stack = vec![root.join("plugins_v2")];
    while let Some(dir) = stack.pop() {
        let Ok(entries) = fs::read_dir(&dir) else { continue };
        for entry in entries.flatten() {
            let path = entry.path();
            if path.is_dir() {
                stack.push(path);
            } else if path.extension().is_some_and(|ext| ext == "ron") {
                files.push(path);
            }
        }
    }
    files.sort();
    for path in files {
        let Ok(text) = fs::read_to_string(&path) else { continue };
        let bytes = text.as_bytes();
        for found in call.captures_iter(&text) {
            let name = found[1].to_owned();
            let open = found.get(0).expect("the whole match").end();
            // The balanced argument list, split at its top-level commas.
            let mut depth = 0usize;
            let mut in_string = false;
            let mut args = Vec::new();
            let mut start = open;
            let mut i = open;
            let mut closed = false;
            while i < bytes.len() {
                let byte = bytes[i];
                if in_string {
                    if byte == b'\\' {
                        i += 1;
                    } else if byte == b'"' {
                        in_string = false;
                    }
                } else {
                    match byte {
                        b'"' => in_string = true,
                        b'(' | b'[' | b'{' => depth += 1,
                        b')' | b']' | b'}' if depth == 0 => {
                            args.push(&text[start..i]);
                            closed = true;
                            break;
                        }
                        b')' | b']' | b'}' => depth -= 1,
                        b',' if depth == 0 => {
                            args.push(&text[start..i]);
                            start = i + 1;
                        }
                        _ => {}
                    }
                }
                i += 1;
            }
            if !closed {
                continue;
            }
            let params = out.entry(name).or_default();
            for (position, arg) in args.iter().enumerate() {
                let (key, value) = match keyed.captures(arg) {
                    Some(captured) => (
                        captured[1].to_owned(),
                        arg[captured.get(0).expect("the whole match").end()..].trim(),
                    ),
                    None => (position.to_string(), arg.trim()),
                };
                if value.is_empty() || value.contains("Param(") || value.contains("//") {
                    continue;
                }
                let value = value.split_whitespace().collect::<Vec<_>>().join(" ");
                let seen = params.entry(key).or_default();
                if !seen.contains(&value) && seen.len() < 12 {
                    seen.push(value);
                }
            }
        }
    }
    out
}

/// The invocation text with `texts[i]` for each parameter, `None` leaving it
/// out.
fn invocation(name: &str, params: &[Param], texts: &[Option<&str>]) -> String {
    if params.is_empty() {
        return name.to_owned();
    }
    let arguments: Vec<String> = params
        .iter()
        .zip(texts)
        .filter_map(|(param, text)| {
            let text = (*text)?;
            Some(match &param.key {
                Some(key) => format!("{key}: {text}"),
                None => text.to_owned(),
            })
        })
        .collect();
    format!("{name}({})", arguments.join(", "))
}

#[allow(
    clippy::too_many_lines,
    reason = "one pass per parameter, read in order"
)]
fn derive(
    macros: &MacroSet,
    pool: &[String],
    harvested: &BTreeMap<String, BTreeMap<String, Vec<String>>>,
    words: &BTreeMap<String, Vec<String>>,
    declaration: &Declaration,
) -> Result<Derived, String> {
    let definition = &declaration.definition;
    let name = definition.name.as_str();
    let kinds: Vec<&str> = definition
        .kinds
        .iter()
        .map(macro_ron::Ident::as_str)
        .collect();
    if definition.body() == "()" {
        return Err("bodyless".to_owned());
    }
    if kinds.contains(&deckmaste_semantics_v2::ron::MACRO_KIND) {
        return Err("meta-macro: its body is a definition template".to_owned());
    }
    if definition
        .body_head(macros)
        .is_some_and(|head| DEFINITION_HEADS.contains(&head.as_str()))
        || kinds == ["Designation"]
    {
        return Err("registry node: its Lean form is `cargo xtask facts`'s".to_owned());
    }
    let Some((result, reader)) = kinds.iter().find_map(|kind| kind_reader(kind)) else {
        return Err(format!(
            "no kind of [{}] is a v2 syntax type",
            kinds.join(", ")
        ));
    };

    let (named, entries): (bool, Vec<(Option<String>, ParamType)>) = match &definition.params {
        Params::Positional(params) => (false, params.iter().map(|ty| (None, ty.clone())).collect()),
        Params::Named(params) => (
            true,
            params
                .iter()
                .map(|(key, ty)| (Some(key.as_str().to_owned()), ty.clone()))
                .collect(),
        ),
    };
    let mut params = Vec::new();
    let mut ordinals: BTreeMap<String, usize> = BTreeMap::new();
    for (key, ty) in entries {
        if ty
            .default
            .as_deref()
            .is_some_and(|text| text.contains("Param("))
        {
            return Err(format!(
                "the default of `{}` names another parameter",
                key.as_deref().unwrap_or("?")
            ));
        }
        let mut list = candidates(macros, ty.name.as_str())?;
        let ordinal = ordinals.entry(ty.name.as_str().to_owned()).or_default();
        let len = list.len();
        list.rotate_left(*ordinal % len);
        *ordinal += 1;
        let position = params.len().to_string();
        let real = harvested
            .get(name)
            .and_then(|by_param| by_param.get(key.as_deref().unwrap_or(&position)))
            .cloned()
            .unwrap_or_default();
        let mut list: Vec<String> = real
            .into_iter()
            .chain(list.into_iter().map(str::to_owned))
            .collect();
        let mut seen = BTreeSet::new();
        list.retain(|text| seen.insert(text.clone()));
        params.push(Param {
            key,
            ty,
            candidates: list,
        });
    }

    let registered = deckmaste_semantics_v2::ron::param_types();
    let read = |texts: &[Option<&str>]| -> Result<Term, String> {
        let rendered = reader(macros, &invocation(name, &params, texts))?;
        Term::parse(rendered)
    };

    // The first assignment that reads, every parameter supplied, no two the
    // same text.
    let base = match first_assignment(&params, &read) {
        Ok(base) => base,
        Err(error) => descend(&params, pool, words, &read).ok_or(error)?,
    };
    let base_texts: Vec<Option<&str>> = base.iter().map(|text| Some(*text)).collect();
    let base_term = read(&base_texts)?;

    let mut holes: Vec<(Range<usize>, usize)> = Vec::new();
    let mut binders = Vec::new();
    let mut binder_names = Vec::new();
    for (index, param) in params.iter().enumerate() {
        // A RON key is the Lean binder's name, or (`followed_by`) the mirror
        // field's, which the mirror's own name mapping turns back into Lean's.
        let binder = lean_ident(&param.key.as_deref().map_or_else(
            || format!("arg{index}"),
            |key| lean_emit::lean_field(key).trim_end_matches('_').to_owned(),
        ));
        // A second reading, this argument alone changed.
        let mut varied: Option<(&str, Term)> = None;
        let mut extra: Vec<Vec<usize>> = Vec::new();
        let mut readings = 0usize;
        // Whether another argument read to the very same term: the body does
        // not mention this parameter.
        let mut unused = false;
        // An untyped parameter's own words, from the type a probe's refusal
        // names, come before the pool.
        let mut own: Vec<&str> = Vec::new();
        let mut listed: Vec<String> = Vec::new();
        if param.ty.name.as_str() == "Any" {
            let mut texts = base_texts.clone();
            texts[index] = Some("ZzProbe");
            if let Err(error) = read(&texts)
                && let Some(wanted) = wanted_type(&error, "ZzProbe")
                && let Some(list) = words.get(wanted)
            {
                own = list.iter().map(String::as_str).collect();
            }
            // A list hole: its element type, from a one-element probe.
            texts[index] = Some("[ZzProbe]");
            if let Err(error) = read(&texts)
                && let Some(wanted) = wanted_type(&error, "ZzProbe")
                && let Some(list) = words.get(wanted)
            {
                listed = list.iter().map(|word| format!("[{word}]")).collect();
            }
        }
        // A typed parameter's own words: its Lean type's constructors.
        if let Some((lean_type, _)) = param_reader(param.ty.name.as_str()) {
            let unparenthesised = lean_type.trim_matches(|c| c == '(' || c == ')');
            if let Some(element) = unparenthesised.strip_prefix("List Semantics.") {
                if let Some(list) = words.get(element) {
                    listed.extend(list.iter().map(|word| format!("[{word}]")));
                }
            } else if let Some(list) = unparenthesised
                .strip_prefix("Semantics.")
                .and_then(|ty| ty.split_whitespace().next())
                .and_then(|ty| words.get(ty))
            {
                own.extend(list.iter().map(String::as_str));
            }
        }
        own.extend(listed.iter().map(String::as_str));
        let validator = (param.ty.name.as_str() != "Any")
            .then(|| registered.get(param.ty.name.as_str()))
            .flatten();
        for candidate in own
            .iter()
            .copied()
            .chain(param.candidates.iter().chain(pool).map(String::as_str))
        {
            if candidate == base[index]
                || validator.is_some_and(|validate| validate(candidate, macros, false).is_err())
            {
                continue;
            }
            let mut texts = base_texts.clone();
            texts[index] = Some(candidate);
            // A different text may spell the same value; only a reading that
            // differs locates the holes.
            let reading = read(&texts);
            if reading
                .as_ref()
                .is_ok_and(|term| term.text == base_term.text)
            {
                unused = true;
            }
            if let Ok(term) = reading
                && term.text != base_term.text
            {
                extra.extend(base_term.differences(&term));
                if varied.is_none() {
                    varied = Some((candidate, term));
                }
                // A typed argument's extent is read off its own rendering;
                // an untyped one's is where several readings first differ.
                if param.ty.name.as_str() != "Any" || {
                    readings += 1;
                    readings >= UNTYPED_READINGS
                } {
                    break;
                }
            }
        }
        let typed = param_reader(param.ty.name.as_str());
        // The outermost differing subterms over every reading: a reading
        // whose argument has another head than the first's differs at the
        // argument's own root.
        let mut paths: Vec<Vec<usize>> = extra
            .iter()
            .filter(|path| {
                !extra
                    .iter()
                    .any(|other| other.len() < path.len() && path.starts_with(other))
            })
            .cloned()
            .collect();
        paths.sort();
        paths.dedup();
        // An untyped argument's extent: the nearest enclosing subterm whose
        // type has a reader that renders the argument as exactly that subterm.
        if typed.is_none() {
            for path in &mut paths {
                let mut probe = path.clone();
                loop {
                    let found = base_term.root.at(&probe).and_then(|node| {
                        let ty = infer_type(&base_term, node)?;
                        let reader = reader_for_lean_type(&ty)?;
                        let rendered = reader(macros, base[index]).ok()?;
                        (squash(&rendered) == squash(base_term.slice(node))).then_some(())
                    });
                    if found.is_some() {
                        *path = probe;
                        break;
                    }
                    if probe.is_empty() {
                        break;
                    }
                    probe.pop();
                }
            }
        }
        // Widen each difference to the whole argument where the argument's
        // own rendering is known.
        if let (Some((lean_type, arg_reader)), Some((other, other_term))) = (typed, &varied) {
            let _ = lean_type;
            let mine = arg_reader(macros, base[index]).map(|text| squash(&text));
            let theirs = arg_reader(macros, other).map(|text| squash(&text));
            if let (Ok(mine), Ok(theirs)) = (mine, theirs) {
                for path in &mut paths {
                    let mut probe = path.clone();
                    loop {
                        let here = base_term
                            .root
                            .at(&probe)
                            .map(|n| squash(base_term.slice(n)));
                        let there = other_term
                            .root
                            .at(&probe)
                            .map(|n| squash(other_term.slice(n)));
                        if here.as_deref() == Some(&mine) && there.as_deref() == Some(&theirs) {
                            *path = probe;
                            break;
                        }
                        if probe.is_empty() {
                            break;
                        }
                        probe.pop();
                    }
                }
            }
        }
        // A default: read without the argument.
        let mut default_text = None;
        let mut optional = false;
        if param.ty.default.is_some() || param.ty.elidable {
            let mut texts = base_texts.clone();
            texts[index] = None;
            let without =
                read(&texts).map_err(|error| format!("reading without `{binder}`: {error}"))?;
            // Where the default differs from the sample, the difference IS
            // the hole: a `none` default against `some value` (implicit_some)
            // marks the whole `some` application, whatever the value's shape.
            let defaulted = base_term.differences(&without);
            for path in &mut paths {
                if let Some(outer) = defaulted.iter().find(|outer| path.starts_with(outer)) {
                    path.clone_from(outer);
                }
            }
            for outer in defaulted {
                if !paths.iter().any(|path| outer.starts_with(path)) {
                    paths.push(outer);
                }
            }
            paths.sort();
            paths.dedup();
            if !paths.is_empty() {
                let mut rendered = BTreeSet::new();
                for path in &paths {
                    let node = without
                        .root
                        .at(path)
                        .ok_or_else(|| format!("`{binder}`'s default changes the term's shape"))?;
                    let text = without.slice(node).to_owned();
                    if text == "none"
                        && base_term
                            .root
                            .at(path)
                            .is_some_and(|node| base_term.slice(node).starts_with("(some "))
                    {
                        optional = true;
                    }
                    rendered.insert(text);
                }
                if rendered.len() != 1 {
                    return Err(format!("`{binder}`'s default renders differently per hole"));
                }
                default_text = rendered.into_iter().next();
            }
        }

        if varied.is_none()
            && !unused
            && typed.is_some()
            && !(param.ty.default.is_some() || param.ty.elidable)
        {
            return Err(format!(
                "no second argument to `{binder}` (`{}`) reads, so its holes cannot be located \
                 (first: `{}`)",
                param.ty.name.as_str(),
                base[index]
            ));
        }
        let lean_type = match typed {
            Some((lean_type, _)) if !optional => lean_type.to_owned(),
            Some((lean_type, _)) => format!("(Option {lean_type})"),
            None => {
                let inferred = paths.first().and_then(|path| {
                    base_term
                        .root
                        .at(path)
                        .and_then(|node| infer_type(&base_term, node))
                        .or_else(|| {
                            let (_, term) = varied.as_ref()?;
                            infer_type(term, term.root.at(path)?)
                        })
                });
                match inferred {
                    Some(ty) => ty,
                    None if paths.is_empty() => {
                        return Err(format!(
                            "`{binder}` is untyped (`{}`) and no second argument to it reads \
                             (first: `{}`)",
                            param.ty.name.as_str(),
                            base[index]
                        ));
                    }
                    None => "_".to_owned(),
                }
            }
        };
        let binder_text = match &default_text {
            Some(default) => format!("({binder} : {lean_type} := {default})"),
            None if param.ty.default.is_some() || param.ty.elidable => {
                // Unused: the body never mentions it, so any default is its
                // own; render the declared one when it reads.
                let default = param
                    .ty
                    .default
                    .as_deref()
                    .and_then(|text| typed.and_then(|(_, r)| r(macros, text).ok()))
                    .unwrap_or_else(|| "none".to_owned());
                format!("({binder} : {lean_type} := {default})")
            }
            None => format!("({binder} : {lean_type})"),
        };
        binders.push(binder_text);
        for path in paths {
            let node = base_term.root.at(&path).ok_or("a hole outside the term")?;
            holes.push((node.span(), index));
        }
        binder_names.push(binder);
    }

    // Holes are disjoint: nested ones are an extraction failure.
    holes.sort_by_key(|(span, _)| (span.start, std::cmp::Reverse(span.end)));
    holes.dedup();
    for pair in holes.windows(2) {
        if pair[1].0.start < pair[0].0.end {
            return Err("two parameters' subterms overlap".to_owned());
        }
    }
    let mut body = String::new();
    let mut at = 0;
    for (span, index) in &holes {
        body.push_str(&base_term.text[at..span.start]);
        body.push_str(&binder_names[*index]);
        at = span.end;
    }
    body.push_str(&base_term.text[at..]);

    Ok(Derived {
        name: name.to_owned(),
        ident: lean_ident(name),
        binders,
        binder_names,
        named,
        result: result.to_owned(),
        body,
        source: String::new(),
    })
}

/// The first assignment, in candidate order with no two parameters given the
/// same text, whose invocation reads.
fn first_assignment<'p>(
    params: &'p [Param],
    read: &impl Fn(&[Option<&str>]) -> Result<Term, String>,
) -> Result<Vec<&'p str>, String> {
    if params.is_empty() {
        read(&[])?;
        return Ok(Vec::new());
    }
    let mut choice = vec![0usize; params.len()];
    let mut first_error = None;
    let mut attempts = 0usize;
    loop {
        let texts: Vec<&'p str> = params
            .iter()
            .zip(&choice)
            .map(|(param, &i)| param.candidates[i].as_str())
            .collect();
        let distinct = texts
            .iter()
            .enumerate()
            .all(|(i, text)| !texts[..i].contains(text));
        if distinct {
            attempts += 1;
            let supplied: Vec<Option<&str>> = texts.iter().map(|text| Some(*text)).collect();
            match read(&supplied) {
                Ok(_) => return Ok(texts),
                Err(error) => {
                    first_error.get_or_insert(error);
                }
            }
            if attempts >= 20_000 {
                break;
            }
        }
        let mut advanced = false;
        for (i, param) in params.iter().enumerate().rev() {
            choice[i] += 1;
            if choice[i] < param.candidates.len() {
                advanced = true;
                break;
            }
            choice[i] = 0;
        }
        if !advanced {
            break;
        }
    }
    Err(format!(
        "no sample assignment reads ({attempts} tried); first: {}",
        first_error.unwrap_or_default()
    ))
}

/// The fallback search when no combination of samples reads: every
/// parameter starts at its first candidate (two may share a text here) and, while the invocation is
/// refused, the parameter whose text the refusal quotes moves to its next
/// candidate (the pool after its own list); a refusal quoting none moves the
/// parameter moved last, or else the untyped one moved least so far. Linear in the pool, where the
/// combinatorial search is exponential in the parameter count.
fn descend<'p>(
    params: &'p [Param],
    pool: &'p [String],
    words: &'p BTreeMap<String, Vec<String>>,
    read: &impl Fn(&[Option<&str>]) -> Result<Term, String>,
) -> Option<Vec<&'p str>> {
    let mut lists: Vec<Vec<&'p str>> = params
        .iter()
        .map(|param| {
            let mut list: Vec<&str> = param.candidates.iter().map(String::as_str).collect();
            if param.ty.name.as_str() == "Any" {
                list.extend(pool.iter().map(String::as_str));
            }
            list
        })
        .collect();
    let mut at = vec![0usize; params.len()];
    let mut moves = vec![0usize; params.len()];
    let mut last: Option<usize> = None;
    let mut retyped: BTreeSet<(usize, String)> = BTreeSet::new();
    for _ in 0..30_000 {
        let texts: Vec<&str> = lists.iter().zip(&at).map(|(list, &i)| list[i]).collect();
        let supplied: Vec<Option<&str>> = texts.iter().map(|text| Some(*text)).collect();
        let Err(error) = read(&supplied) else {
            return Some(texts);
        };
        let quoted: Vec<usize> = (0..params.len())
            .filter(|&i| error.contains(&format!("`{}`", texts[i])))
            .collect();
        // Several parameters holding the quoted text: move all but the first;
        // if the first was the culprit, the next refusal quotes it alone.
        let moving: Vec<usize> = match quoted.as_slice() {
            // A refusal quoting no argument (a word that wants arguments, a
            // list where a word belongs) is most likely the last move's.
            [] => vec![last.filter(|&i| at[i] + 1 < lists[i].len()).or_else(|| {
                (0..params.len())
                    .filter(|&i| params[i].ty.name.as_str() == "Any")
                    .filter(|&i| at[i] + 1 < lists[i].len())
                    .min_by_key(|&i| moves[i])
            })?],
            [one] => vec![*one],
            [_, rest @ ..] => rest.to_vec(),
        };
        // A refusal naming the type it wanted: that type's own words first.
        if let [index] = moving.as_slice()
            && let Some(wanted) = wanted_type(&error, texts[*index])
            && let Some(own) = words.get(wanted)
            && retyped.insert((*index, wanted.to_owned()))
        {
            let mut retyped_list: Vec<&str> = own.iter().map(String::as_str).collect();
            retyped_list.extend(lists[*index].iter().copied());
            lists[*index] = retyped_list;
            at[*index] = 0;
            last = Some(*index);
            continue;
        }
        for index in moving {
            last = Some(index);
            at[index] += 1;
            moves[index] += 1;
            if at[index] >= lists[index].len() {
                return None;
            }
        }
    }
    None
}

/// The Lean type of a rendered subterm, read off its head: a constructor
/// `Semantics.T.c` is a `Semantics.T`, an ascribed structure its ascription.
fn infer_type(term: &Term, node: &Node) -> Option<String> {
    let text = term.slice(node);
    let head_type = |atom: &str| -> Option<String> {
        let rest = atom.strip_prefix("Semantics.")?;
        let (ty, _) = rest.rsplit_once('.')?;
        Some(format!("Semantics.{ty}"))
    };
    match node {
        Node::Atom(_) => {
            if text.starts_with('"') {
                Some("String".to_owned())
            } else if text.chars().all(|c| c.is_ascii_digit()) {
                Some("Nat".to_owned())
            } else if text == "true" || text == "false" {
                Some("Bool".to_owned())
            } else {
                head_type(text)
            }
        }
        Node::Group { children, .. } => match text.as_bytes()[0] {
            b'(' => {
                let first = children.first()?;
                let head = term.slice(first);
                if head == "some" {
                    let inner = infer_type(term, children.get(1)?)?;
                    return Some(format!("(Option {inner})"));
                }
                if children.len() == 3 && term.slice(&children[1]) == ":" {
                    return Some(term.slice(&children[2]).to_owned());
                }
                head_type(head)
            }
            b'[' => {
                let inner = infer_type(term, children.first()?)?;
                Some(format!("(List {inner})"))
            }
            _ => None,
        },
    }
}

// ---------------------------------------------------------------------------
// Alignment
// ---------------------------------------------------------------------------

/// The macros `Macros.lean` declares, by their name relative to
/// `Semantics.Macros` (`Actor.cloak` inside `namespace Actor`).
fn hand_macros(root: &Path) -> anyhow::Result<BTreeSet<String>> {
    let text = fs::read_to_string(root.join(HAND)).with_context(|| format!("reading {HAND}"))?;
    let mut scopes: Vec<String> = Vec::new();
    let mut out = BTreeSet::new();
    for line in text.lines() {
        if let Some(name) = line.strip_prefix("namespace ") {
            scopes.push(name.trim().to_owned());
        } else if line.starts_with("end ") && !scopes.is_empty() {
            scopes.pop();
        } else if let Some(rest) = line.strip_prefix("semantic_macro ")
            && let Some(name) = rest.split_whitespace().next()
        {
            let inner: Vec<&str> = scopes.iter().skip(1).map(String::as_str).collect();
            let mut full = inner.join(".");
            if !full.is_empty() {
                full.push('.');
            }
            full.push_str(name);
            out.insert(full);
        }
    }
    Ok(out)
}

/// One alignment obligation: a hand macro and its declared namesake.
struct Pair {
    hand: String,
    start_line: usize,
}

fn run_align(root: &Path, deny_drift: bool) -> anyhow::Result<()> {
    let module = render(root)?;
    let hand = hand_macros(root)?;
    let mut source = String::from(
        "-- GENERATED by `cargo xtask lean macros align`; untracked.\n\
         import Semantics.Macros\n\
         import Semantics.Macros.Declared\n\n\
         set_option maxRecDepth 4096\n\n",
    );
    let mut pairs = Vec::new();
    for derived in &module.derived {
        // The unprefixed macro, or else the performer helper of that name
        // (`Actor.cloak`), which §12.1 counts as the declaration's bench twin.
        let Some(hand_name) = [derived.ident.clone(), format!("Actor.{}", derived.ident)]
            .into_iter()
            .find(|name| hand.contains(name))
        else {
            continue;
        };
        let binders = derived.binders.join(" ");
        let arguments: Vec<String> = derived
            .binder_names
            .iter()
            .map(
                |name| {
                    if derived.named { format!("({name} := {name})") } else { name.clone() }
                },
            )
            .collect();
        let arguments = arguments.join(" ");
        let start_line = source.lines().count() + 1;
        let _ = writeln!(
            source,
            "example {binders} :\n    (Semantics.Macros.{hand} {arguments} : {result}) =\n      \
             {NAMESPACE}.{declared} {arguments} := rfl\n",
            hand = hand_name,
            declared = derived.ident,
            result = derived.result,
        );
        pairs.push(Pair {
            hand: hand_name.clone(),
            start_line,
        });
    }
    let lean = root.join("lean");
    fs::write(lean.join(ALIGN_ROOT), &source).with_context(|| format!("writing {ALIGN_ROOT}"))?;
    let output = Command::new("lake")
        .args(["build", ALIGN_TARGET])
        .current_dir(&lean)
        .output()
        .context("running lake")?;
    let log = format!(
        "{}{}",
        String::from_utf8_lossy(&output.stdout),
        String::from_utf8_lossy(&output.stderr)
    );
    // `error: <path>:<line>:<col>: …` — attribute each to the pair whose
    // block it falls in.
    let mut failed: BTreeMap<String, String> = BTreeMap::new();
    for line in log.lines() {
        let Some(rest) = line
            .split_once(&format!("{ALIGN_ROOT}:"))
            .map(|(_, rest)| rest)
        else {
            continue;
        };
        if !line.contains("error") {
            continue;
        }
        let Some(number) = rest.split(':').next().and_then(|n| n.parse::<usize>().ok()) else {
            continue;
        };
        if let Some(pair) = pairs.iter().rev().find(|pair| pair.start_line <= number) {
            failed
                .entry(pair.hand.clone())
                .or_insert_with(|| rest.chars().take(200).collect());
        }
    }
    let unattributed = !output.status.success() && failed.is_empty();
    println!(
        "{} hand macros with a declared namesake: {} aligned, {} drifted",
        pairs.len(),
        pairs.len() - failed.len(),
        failed.len()
    );
    // Why each drifts: the hand signature differs from the declaration's,
    // or the bodies differ, and if so whether the declaration's body carries
    // a loader device (§12.2) the hand macro does not spell.
    let devices = ["firstMention", "laterMention", "ownScope", "inCaller"];
    let mut classes: BTreeMap<&str, usize> = BTreeMap::new();
    for (name, reason) in &failed {
        let declared = module
            .derived
            .iter()
            .find(|derived| *name == derived.ident || *name == format!("Actor.{}", derived.ident));
        let class =
            if !reason.contains("Type mismatch") || reason.contains("Application type mismatch") {
                "signature"
            } else if declared
                .is_some_and(|derived| devices.iter().any(|device| derived.body.contains(device)))
            {
                "body, loader device"
            } else {
                "body"
            };
        *classes.entry(class).or_default() += 1;
        println!("  drifted {name} [{class}]: {reason}");
    }
    for (class, count) in &classes {
        println!("{count} drifted by {class}");
    }
    let report = lean.join(".lake/macro-alignment.log");
    fs::write(&report, &log).with_context(|| format!("writing {}", report.display()))?;
    println!("full lake output: {}", report.display());
    if unattributed {
        eprintln!("{log}");
        anyhow::bail!("lake failed outside every alignment block");
    }
    anyhow::ensure!(
        !deny_drift || failed.is_empty(),
        "{} hand macros drift from their declarations",
        failed.len()
    );
    Ok(())
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn every_param_type_has_a_lean_reader() {
        let types = deckmaste_semantics_v2::ron::param_types();
        let missing: Vec<&str> = types
            .names()
            .filter(|name| *name != "Any" && param_reader(name).is_none())
            .collect();
        assert!(
            missing.is_empty(),
            "param types with no Lean reader: {missing:?}"
        );
    }

    #[test]
    fn every_syntax_kind_has_a_lean_reader() {
        for kind in deckmaste_semantics_v2::ron::kinds().iter() {
            let name = kind.name();
            assert_eq!(
                kind_reader(name).is_some(),
                crate::expansions::has_renderer(name),
                "kind `{name}`: the Lean reader and the expansion printer disagree"
            );
        }
    }

    #[test]
    fn the_committed_module_matches_the_declarations() {
        run_check(&repo_root()).expect("`cargo xtask lean macros check` must pass on a clean tree");
    }

    #[test]
    fn a_hole_is_the_whole_argument() {
        let a = Term::parse("(f (g 3) x)".to_owned()).unwrap();
        let b = Term::parse("(f (g 4) x)".to_owned()).unwrap();
        assert_eq!(a.differences(&b), vec![vec![0, 1, 1]]);
        // Another constructor at the same arity is another subterm.
        let c = Term::parse("(f (h 3) x)".to_owned()).unwrap();
        assert_eq!(a.differences(&c), vec![vec![0, 1]]);
    }
}
