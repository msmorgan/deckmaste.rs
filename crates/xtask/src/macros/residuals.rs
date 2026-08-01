//! `cargo xtask macro residuals` — the corpus-wide matcher sweep and the
//! ranked census of what it could not recover.
//!
//! Where `macro pilot` gates a *narrowed* slice of canon against ground truth,
//! this command measures the matcher against the corpus as printed. Every
//! ability of every non-todo canon face is legacy-rendered in isolation, and
//! **every rules line that render produces** is one swept line: an
//! `ability_word` prefix, a static or activated ability, a modal's several
//! lines — none of them is set aside. The population is therefore the honest
//! denominator for "how much of canon can the lexicon recover", and it is
//! meant to be read as a low number.
//!
//! # The three classes
//!
//! Each line is parsed at its ability's own category — falling back to the
//! whole-ability category, see [`classify`] — and [`unify`]'d against the
//! whole lexicon, every kind at once, at [`FramePosition::Main`]:
//!
//! - **`Full`** — the top level recovered an entry and nothing anywhere
//!   underneath came back a [`Recovered::Residual`].
//! - **`Partial`** — the top level recovered an entry, but some filler did not.
//!   Each unrecovered filler contributes a [`Signature`].
//! - **`NoMatch`** — no entry matched the line at all. The whole line's
//!   [`View`] is then the residual, and contributes its [`Signature`] the same
//!   way: a line nothing covers is the *largest* thing a new frame could
//!   absorb, so leaving it out of the ranking would hide the top rows.
//!
//! A line no category parses is `NoMatch` too, and is counted separately in
//! the printed census — an english grammar gap is not a lexicon gap, and a
//! reader ranking frames to author needs to see which one they are looking at.
//!
//! # What a signature is
//!
//! A [`Signature`] is a **grouping key**, so two residuals of one shape must
//! key alike across cards, not merely across runs. It is the residual's
//! constructor path to [`SIGNATURE_DEPTH`], with scalar *values* dropped — a
//! card name and a count keep only their serde kind — and with every child
//! position the lexicon already recovers replaced by a `<Entry>` hole marker.
//! The hole
//! markers are what make a row a draft frame rather than a complaint: they say
//! which constituents are already spellable and which one is missing.
//!
//! `deckmaste_frames`'s own residual truncation is a *diagnostic* — it caps a
//! `Debug` string by character budget and keeps every scalar value in it — so
//! it cannot serve here: two cards' instances of one shape would carry their
//! own names and numbers and group into two rows.
//!
//! # A row is one key, not necessarily one shape
//!
//! [`View`] cannot see a **tuple variant's name**: `serialize_tuple_variant`
//! keeps neither the type nor the variant and yields a bare [`View::Seq`], so
//! `IndependentClause`'s `Transitive`, `Intransitive`, `Passive` and
//! `Predicated` all arrive as `[…, …]` and spell alike. That is upstream of
//! this module and cannot be recovered here. What survives the erasure is the
//! `kind` field of the `HeadedPredicate` underneath — one serde struct name
//! per clause kind — which the key does spell wherever that node is reached at
//! or above the cut.
//!
//! So the erasure is *mitigated*, not repaired, and one case remains: where a
//! `HeadedPredicate` lands **at** the cut it is named but its `kind` is a level
//! further down, and two clause kinds in that position still share a key.
//! Ranked rows are grouped by key, so read a row's site count as "sites keying
//! alike", which is a ceiling on what one draft frame could absorb rather than
//! a promise.
//!
//! # Drafting a catalog entry from a row
//!
//! `--drafts PATH` turns the top ranked rows into [`ConstructorFrames`]
//! entries, one per row, written to `PATH` as a catalog file — never into
//! `plugins/`, which is a human's decision to make. A row is a grouping key
//! over many sites, but only one concrete site can seed a draft's surface
//! text, so drafting reads the exemplar kept alongside each [`Row`]: the
//! first line the shape was found on.
//!
//! **Base text.** A [`Classification::NoMatch`] residual is, by construction
//! (`unify_at`'s own top-level failure case), the exemplar line's *whole*
//! parsed tree — so the exemplar's own rendered line is already that
//! residual's exact surface text, with nothing to reconstruct. Legacy render
//! spells a card's self-reference as its own printed name, never as `~`, so
//! that name is replaced back to `~` before anything else runs — otherwise
//! every draft from a whole-line residual would be a one-card frame, not a
//! general one. A residual that is not a whole line (only reachable through
//! [`Classification::Partial`]; the live corpus attests none) has no such
//! shortcut, and falls back to
//! [`deckmaste_frames::render::render_residual_text`]'s honest, narrower
//! reconstruction; a shape neither path can render is skipped rather than
//! guessed at.
//!
//! **Cutting holes.** Each [`TypedHole`] names the entry that already covers
//! one position. A *nullary* entry (`This`, `You`, `AnyTarget`, …) has no
//! params to vary its own frame text, so that text — `~` resolved to the
//! exemplar's own name — *is* the filler, no lookup into the card's data
//! required (and for a self-reference specifically, the base-text
//! substitution above has usually already spelled it `~` before the hole
//! search even runs, so nothing is left to cut there). Anything else falls
//! back to `render_residual_text` on the covered child. Either way the
//! filler's spelling is then searched for in the base text (ASCII
//! case-folded, forward from the previous cut, mirroring
//! `witness::reserved_tokens`'s own length-preserving fold) and replaced with
//! `<Param(i)>` — but only once the covering entry's own name is mapped to a
//! *legal* `params:` type (`This`/`You` are lexicon identities, not entries
//! in `deckmaste_cards::macros::param_types()`; both denote a
//! `deckmaste_core::Reference`, which is): a hole whose entry has no known
//! legal type is left uncut for the same reason a filler that cannot be
//! found is — an illegal `params:` entry would be a second field, beside
//! `body:`, a human would have to fix by hand.
//!
//! **Never a broken draft.** Every candidate is compiled
//! ([`deckmaste_frames::compile::compile`]) before it is kept; a draft whose
//! carved text does not compile at its own derived `kind` is dropped and the
//! reason is reported, rather than shipping a skeleton a human would paste in
//! and immediately get a build error from. `body:` is never set — D11's
//! "only `body:` blank" contract — which for [`ConstructorFrames`] means the
//! field is absent from the struct, not present and empty.

use std::collections::BTreeMap;
use std::collections::HashSet;
use std::fmt::Write as _;
use std::path::Path;
use std::path::PathBuf;
use std::time::Instant;

use clap::Args;
use deckmaste_cards::plugin::Plugin;
use deckmaste_cards::plugin::read;
use deckmaste_cards::render::CardView;
use deckmaste_cards::render::render;
use deckmaste_core::Ability;
use deckmaste_core::Card;
use deckmaste_core::Supertype;
use deckmaste_core::plugin::CARDS_DIR;
use deckmaste_core::plugin::is_todo_source;
use deckmaste_english::Catalogs;
use deckmaste_english::FragmentKind;
use deckmaste_english::parse_fragment;
use deckmaste_frames::Lexicon;
use deckmaste_frames::PathStep;
use deckmaste_frames::Recovered;
use deckmaste_frames::TreePath;
use deckmaste_frames::View;
use deckmaste_frames::render::render_residual_text;
use deckmaste_frames::unify;
use deckmaste_frames::view;
use macro_ron::MacroSet;
use macro_ron::frames::ConstructorFrames;
use macro_ron::frames::FrameKind;
use macro_ron::frames::FramePosition;
use macro_ron::frames::FrameSpec;
use macro_ron::frames::load_constructor_frames;
use ron::ser::PrettyConfig;

use super::pilot::faces;
use super::pilot::peel_expanded;
use super::pilot::real_catalogs;
use super::pilot::ron_files_recursive;

/// How deep a [`Signature`] spells the residual's constructor path before it
/// elides the rest.
///
/// A grouping key wants the shape a frame would have to *state*, not the whole
/// subtree: one unrecovered nominal prints to several hundred lines, and
/// keying on all of it would give almost every card a row of its own. Three
/// levels reaches clause structure — subject, predicate, and what sits under
/// each — and leaves the wording-specific interior below the key.
const SIGNATURE_DEPTH: usize = 3;

/// How many ranked rows a plain invocation prints. The full table is what
/// `--write-report` is for.
const PRINTED_ROWS: usize = 25;

/// How many top-ranked rows `--drafts` turns into catalog entries, absent an
/// explicit `--top`.
const DRAFT_TOP_N: usize = 20;

/// Every drafted `constructor:` carries this prefix — a placeholder identity,
/// never an authorial one, so a human copying an entry into `plugins/` is not
/// tempted to keep the machine-picked name, and a draft cannot collide with a
/// real catalog constructor by accident.
const DRAFT_PREFIX: &str = "Draft";

/// The two population figures this sweep's reading is only meaningful
/// *relative to* — see [`RESIDUAL_FLOOR`].
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
struct Population {
    /// Canon ability lines swept.
    lines: usize,
    /// Of those, the ones classified [`Classification::Full`].
    full: usize,
}

/// The population this sweep is pinned at.
///
/// **This is a floor, not a target.** Nothing this command prints is a
/// comparison that can fail on its own: a census is a count, and a count over
/// an empty corpus is a clean-looking zero. Pointed at a directory with no
/// cards in it (`cargo xtask macro residuals --canon-dir /tmp/does-not-exist`)
/// the sweep would otherwise report `0 lines, 0 Full` and exit 0 — the exact
/// shape any regression that *shrinks* the corpus takes, including one that
/// stops a whole ability kind from rendering or parsing.
///
/// The `full` half is exposed the same way one layer along. Recovery is
/// classified, never asserted, so a lexicon change that turns `Full` lines
/// into `NoMatch` lines breaks no assertion here at all; it only makes a
/// number smaller. Pinning both is what closes that, because either
/// regression lowers one of them.
///
/// So the numbers are compared, not just printed, and **any decrease fails the
/// run** ([`check_floor`]). A round that legitimately grows either figure
/// raises it deliberately, in the same commit that grows it. They are not to
/// be lowered to make a run green.
const RESIDUAL_FLOOR: Population = Population {
    lines: 109,
    full: 10,
};

#[derive(Debug, Args)]
pub(super) struct ResidualArgs {
    /// The plugin the lexicon (frames + macros) is assembled from. Defaults
    /// to this workspace's `plugins/builtin`.
    #[arg(long)]
    plugin_dir: Option<PathBuf>,
    /// The canon corpus to sweep. Defaults to this workspace's
    /// `plugins/canon`.
    #[arg(long)]
    canon_dir: Option<PathBuf>,
    /// Write the whole ranked residual table here, as markdown. Without it,
    /// only the top rows are printed.
    #[arg(long)]
    write_report: Option<PathBuf>,
    /// Write draft catalog entries for the top-ranked residual signatures
    /// here, as a RON `ConstructorFrames` catalog file — the shape
    /// `load_constructor_frames` reads, ready to be reviewed and copied into
    /// a real catalog by hand. Without it, no drafts are generated. Never
    /// written under `plugins/` by this command; the path is the caller's.
    #[arg(long)]
    drafts: Option<PathBuf>,
    /// How many top-ranked rows `--drafts` drafts.
    #[arg(long, default_value_t = DRAFT_TOP_N)]
    top: usize,
}

/// # Errors
/// If the lexicon or canon corpus fails to load, if a report cannot be
/// written, or — the gate itself — if the population fell below
/// [`RESIDUAL_FLOOR`].
pub(super) fn run(args: ResidualArgs) -> anyhow::Result<()> {
    let workspace_root = Path::new(env!("CARGO_MANIFEST_DIR")).join("../..");
    let plugin_dir = args
        .plugin_dir
        .unwrap_or_else(|| workspace_root.join("plugins/builtin"));
    let canon_dir = args
        .canon_dir
        .unwrap_or_else(|| workspace_root.join("plugins/canon"));

    let plugin = Plugin::load_with_sibling_prelude(&plugin_dir)?;
    let catalogs = real_catalogs(&workspace_root)?;
    let constructors = load_constructor_frames(&plugin_dir.join("frames"))?;
    let lexicon = Lexicon::assemble(&plugin.macros, &constructors, &catalogs)?;

    let canon_plugin = Plugin::load_with_sibling_prelude(&canon_dir)?;
    let swept = collect_lines(&canon_dir, &canon_plugin.macros)?;

    let started = Instant::now();
    let outcomes: Vec<LineOutcome> = swept
        .lines
        .iter()
        .map(|line| classify(line, &lexicon, &catalogs))
        .collect();
    let elapsed = started.elapsed();

    let census = Census::of(&swept.lines, &outcomes);
    census.print(&swept, elapsed);

    let ranked = rank(&swept.lines, &outcomes);
    print_ranked(&ranked, census.residual_sites);
    if let Some(path) = &args.write_report {
        std::fs::write(path, report(&census, &ranked))
            .map_err(|error| anyhow::anyhow!("writing {}: {error}", path.display()))?;
        println!("wrote the full ranked table to {}", path.display());
    }
    if let Some(path) = &args.drafts {
        let (drafts, skipped) = draft_entries(&ranked, args.top, &lexicon, &catalogs);
        let text = deckmaste_core::ron::raw_options()
            .to_string_pretty(&drafts, PrettyConfig::default())
            .map_err(|error| anyhow::anyhow!("serializing drafts: {error}"))?;
        std::fs::write(path, text)
            .map_err(|error| anyhow::anyhow!("writing {}: {error}", path.display()))?;
        println!(
            "wrote {} draft entry(ies) from the top {} residual signature(s) to {} ({} could \
             not be drafted)",
            drafts.len(),
            args.top.min(ranked.len()),
            path.display(),
            skipped.len(),
        );
        for reason in &skipped {
            println!("  skipped {reason}");
        }
    }

    check_floor(&Population {
        lines: swept.lines.len(),
        full: census.full,
    })
}

/// Compares `observed` against [`RESIDUAL_FLOOR`], reporting **every** breach
/// at once rather than the first — the two figures usually move together, and
/// seeing which ones did is most of the diagnosis.
///
/// # Errors
/// If either pinned figure decreased.
fn check_floor(observed: &Population) -> anyhow::Result<()> {
    let breaches: Vec<String> = [
        ("canon lines swept", observed.lines, RESIDUAL_FLOOR.lines),
        ("lines fully recovered", observed.full, RESIDUAL_FLOOR.full),
    ]
    .into_iter()
    .filter(|(_, observed, floor)| observed < floor)
    .map(|(label, observed, floor)| format!("{label}: {observed}, floor {floor}"))
    .collect();
    anyhow::ensure!(
        breaches.is_empty(),
        "macro residuals: population floor breached — the census above is over a smaller corpus \
         than the one it was established on, so its percentages describe a different population. \
         Fix the shrinkage; do not lower `RESIDUAL_FLOOR`. {}",
        breaches.join("; "),
    );
    Ok(())
}

// ---------------------------------------------------------------------------
// Corpus loading
// ---------------------------------------------------------------------------

/// One isolated canon rules line, with everything [`classify`] needs to parse
/// it back.
struct Line {
    name: String,
    is_legendary: bool,
    /// The category this line is parsed at first — its ability's own.
    kind: FragmentKind,
    text: String,
}

/// Every canon line the sweep found, and the abilities it found them on.
struct Swept {
    lines: Vec<Line>,
    /// Canon abilities visited, whether or not any rendered.
    abilities: usize,
    /// Of those, the ones whose isolated render printed nothing at all — a
    /// zero-line ability contributes no swept line, so it is counted here
    /// rather than vanishing between the two totals.
    silent: usize,
}

/// Every ability of every non-todo canon face, each rendered on its own and
/// split into the lines that render printed.
///
/// The loader is the fidelity harness's: the plugin read with its sibling
/// prelude, `cards/` walked recursively, `.ron.todo` sources skipped, each
/// card read through the plugin's own `MacroSet`, both faces of a two-faced
/// card visited. Everything that differs from `macro pilot`'s sweep is
/// downstream of that: it sets three ability shapes aside and tests a
/// trigger's *effect clause* in place of the line the card prints, while this
/// one sets nothing aside and takes every line as printed.
fn collect_lines(canon_dir: &Path, macros: &MacroSet) -> anyhow::Result<Swept> {
    let mut lines = Vec::new();
    let mut abilities = 0usize;
    let mut silent = 0usize;
    for path in ron_files_recursive(&canon_dir.join(CARDS_DIR))? {
        let source = read(&path)?;
        if is_todo_source(&source) {
            continue;
        }
        let card: Card = macros
            .read_str(&source)
            .map_err(|error| anyhow::anyhow!("parsing {}: {error:#}", path.display()))?;
        for face in faces(&card) {
            let is_legendary = face.supertypes.contains(&Supertype::Legendary);
            for ability in &face.abilities {
                abilities += 1;
                let isolated = CardView {
                    name: &face.name,
                    mana_cost: None,
                    supertypes: &[],
                    types: &[],
                    subtypes: &[],
                    power: None,
                    toughness: None,
                    abilities: std::slice::from_ref(ability),
                };
                let rendered = render(&isolated).rules;
                if rendered.is_empty() {
                    silent += 1;
                }
                let kind = match peel_expanded(ability) {
                    Ability::Keyword(_) => FragmentKind::KeywordLine,
                    _ => FragmentKind::Sentence,
                };
                for text in rendered {
                    lines.push(Line {
                        name: face.name.to_string(),
                        is_legendary,
                        kind,
                        text,
                    });
                }
            }
        }
    }
    Ok(Swept {
        lines,
        abilities,
        silent,
    })
}

// ---------------------------------------------------------------------------
// Classification
// ---------------------------------------------------------------------------

/// What the matcher made of one line.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
enum Classification {
    /// A top-level entry matched and every filler under it recovered too.
    Full,
    /// A top-level entry matched; at least one filler did not.
    Partial,
    /// No entry matched the line at all — including because no category
    /// parsed it, which [`LineOutcome::parsed_at`] distinguishes.
    NoMatch,
}

impl Classification {
    const fn label(self) -> &'static str {
        match self {
            Classification::Full => "Full",
            Classification::Partial => "Partial",
            Classification::NoMatch => "NoMatch",
        }
    }
}

/// One residual subtree, keyed by its truncated shape — see the module doc.
#[derive(Debug, Clone, PartialEq, Eq)]
struct Signature {
    key: String,
    /// Every position inside the residual the lexicon already recovers, in the
    /// order the key spells them.
    holes: Vec<TypedHole>,
}

/// One residual site: its grouping [`Signature`] plus the raw subtree it was
/// computed from. The census itself only ever reads `signature` — `view` is
/// kept solely so a ranked [`Row`]'s first site can seed a draft's surface
/// text (see the module doc's "Drafting a catalog entry from a row").
#[derive(Debug, Clone, PartialEq, Eq)]
struct ResidualSite {
    signature: Signature,
    view: View,
}

/// A position inside a residual that some entry already covers: a hole a draft
/// frame would come with pre-typed.
#[derive(Debug, Clone, PartialEq, Eq)]
struct TypedHole {
    /// Where in the residual. Resolves against the [`Signature`]'s own
    /// residual view (`root.walk()`'s addressing, unaffected by anything
    /// [`spell`] elides below the cut — see the module doc), which is what
    /// lets the draft generator find each hole's covered child back.
    path: TreePath,
    /// The entry that recovered it.
    entry: String,
}

/// One line's verdict.
#[derive(Debug, Clone, PartialEq, Eq)]
struct LineOutcome {
    classification: Classification,
    /// The category that accepted the line, or `None` if none did.
    parsed_at: Option<FragmentKind>,
    /// One per unrecovered subtree, outermost first. Empty exactly when the
    /// line is [`Classification::Full`] or nothing parsed it.
    residuals: Vec<ResidualSite>,
    /// Every constituent whose winning match had a **rival**: a second
    /// authored frame that also matched there. The unifier records a rival
    /// whether or not it matched as well, so most of these are strictly
    /// dominated — fewer nodes claimed, or fewer guards — and the specificity
    /// order settles them with nothing left over. Only a rival that ties on
    /// both counts is an ambiguity in the frame set, which is why the printed
    /// detail carries each side's claims and guards: the two cases are not
    /// distinguishable from the count alone. Assembly already refuses a
    /// lexicon whose frames are ambiguous by construction, so anything
    /// recorded here collides only on a particular card's wording.
    ambiguities: Vec<String>,
}

/// Parses `line` and classifies what the lexicon recovers from it.
///
/// The line is offered to its ability's own category first and to
/// [`FragmentKind::Ability`] second. The fallback loosens nothing about what
/// counts as covered — the same `unify` decides the match either way, on the
/// same entries. It is what lets an activated or loyalty line, whose leading
/// cost or loyalty symbol no sentence category accepts, contribute a
/// [`Signature`] at all, with whatever it does already cover hole-typed
/// inside it. Without it the shapes carrying the most obvious draft frames
/// would be the ones the ranking could not see.
fn classify(line: &Line, lexicon: &Lexicon, catalogs: &Catalogs) -> LineOutcome {
    let parsed = [line.kind, FragmentKind::Ability]
        .into_iter()
        .find_map(|kind| {
            let report = parse_fragment(&line.text, catalogs, kind, &line.name, line.is_legendary);
            report
                .clean()
                .then(|| report.into_fragment().map(|fragment| (kind, fragment)))
                .flatten()
        });
    let Some((parsed_at, fragment)) = parsed else {
        return LineOutcome {
            classification: Classification::NoMatch,
            parsed_at: None,
            residuals: Vec::new(),
            ambiguities: Vec::new(),
        };
    };

    let recovered = unify(&view::of(&fragment), lexicon, FramePosition::Main);
    let mut unrecovered = Vec::new();
    collect_residuals(&recovered, &mut unrecovered);
    let classification = match (&recovered, unrecovered.is_empty()) {
        (Recovered::Residual(_), _) => Classification::NoMatch,
        (_, true) => Classification::Full,
        (_, false) => Classification::Partial,
    };
    LineOutcome {
        classification,
        parsed_at: Some(parsed_at),
        residuals: unrecovered
            .into_iter()
            .map(|view| ResidualSite {
                signature: signature(view, lexicon),
                view: view.clone(),
            })
            .collect(),
        ambiguities: recovered
            .ambiguities()
            .into_iter()
            .map(str::to_string)
            .collect(),
    }
}

/// Every unrecovered subtree in `recovered`, outermost first.
fn collect_residuals<'a>(recovered: &'a Recovered, out: &mut Vec<&'a View>) {
    match recovered {
        Recovered::Residual(view) => out.push(view),
        Recovered::Invocation { args, .. } => {
            for arg in args {
                collect_residuals(arg, out);
            }
        }
        Recovered::Literal(_) => {}
    }
}

// ---------------------------------------------------------------------------
// Signatures
// ---------------------------------------------------------------------------

/// The grouping key for one unrecovered subtree — see the module doc.
fn signature(view: &View, lexicon: &Lexicon) -> Signature {
    let mut holes = Vec::new();
    let key = spell(view, 0, &TreePath::default(), lexicon, &mut holes);
    Signature { key, holes }
}

/// `view` as a constructor path, cut at [`SIGNATURE_DEPTH`], with each covered
/// child position replaced by its hole marker.
///
/// The cover probe runs *before* the depth cut so that a recovered constituent
/// sitting at the cut still names itself: a hole is the most informative thing
/// a row can say about a position, and eliding it would throw away exactly the
/// part a draft frame would keep. It never runs *below* the cut, which is what
/// keeps every hole visible in the key — see [`rank`], which relies on it.
///
/// A [`View::Newtype`] costs no depth. It is the shape category plumbing
/// takes — `Fragment::Sentence(…)`, `Phrase::NounPhrase(…)` — carrying one
/// child and no material of its own, which is exactly why the unifier peels
/// those chains at the root of every match attempt rather than matching
/// through them. Charging them would spend the whole budget on wrappers and
/// key every residual on the category it was parsed at.
///
/// The node *at* the cut is spelled by [`label`] rather than dropped. The
/// budget bounds descent, and naming a node is not descending into it; the
/// nodes that carry a tuple variant's only surviving discriminator sit exactly
/// there (see the module doc), so withholding the name merged shapes that
/// differ precisely in what a draft frame would have to state.
fn spell(
    view: &View,
    depth: usize,
    at: &TreePath,
    lexicon: &Lexicon,
    holes: &mut Vec<TypedHole>,
) -> String {
    if depth >= SIGNATURE_DEPTH {
        // Named, not elided. The cut bounds how deep the key *descends*, and a
        // node's own label costs no descent — while withholding it merges
        // shapes that differ exactly there, which is what a key must not do.
        return label(view);
    }
    let label = label(view);
    let below = depth + usize::from(!matches!(view, View::Newtype { .. }));
    let child = |(step, child): &(PathStep, &View), holes: &mut Vec<TypedHole>| {
        let path = at.then(*step);
        match covered_by(child, lexicon) {
            Some(entry) => {
                let marker = format!("<{entry}>");
                holes.push(TypedHole { path, entry });
                marker
            }
            None => spell(child, below, &path, lexicon, holes),
        }
    };
    match view {
        View::Seq(items) if items.is_empty() => "[]".to_string(),
        View::Seq(_) => {
            let parts: Vec<String> = view
                .children()
                .iter()
                .map(|pair| child(pair, holes))
                .collect();
            format!("[{}]", parts.join(", "))
        }
        View::Map(entries) if entries.is_empty() => "{}".to_string(),
        View::Map(_) => {
            let parts: Vec<String> = view
                .children()
                .iter()
                .map(|pair| child(pair, holes))
                .collect();
            format!("{{{}}}", parts.join(", "))
        }
        View::Newtype { .. } => {
            let inner: Vec<String> = view
                .children()
                .iter()
                .map(|pair| child(pair, holes))
                .collect();
            format!("{label}({})", inner.join(", "))
        }
        View::Node { fields, .. } if fields.is_empty() => label,
        View::Node { fields, .. } => {
            let named: Vec<String> = fields
                .iter()
                .zip(view.children().iter())
                .map(|((name, _), pair)| format!("{name}: {}", child(pair, holes)))
                .collect();
            format!("{label}{{{}}}", named.join(", "))
        }
        View::Scalar { .. } | View::Unit { .. } | View::Absent | View::Hole { .. } => label,
    }
}

/// One node's own label, children aside. Scalar *values* are dropped and only
/// their serde kind survives — the property that makes a key group two cards'
/// instances of one shape instead of giving each its own row.
fn label(view: &View) -> String {
    match view {
        View::Scalar { kind, .. } => (*kind).to_string(),
        View::Unit { name, variant }
        | View::Newtype { name, variant, .. }
        | View::Node { name, variant, .. } => match variant {
            Some(variant) => format!("{name}::{variant}"),
            None => (*name).to_string(),
        },
        View::Seq(_) => "[]".to_string(),
        View::Map(_) => "{}".to_string(),
        View::Absent => "None".to_string(),
        View::Hole { class, .. } => format!("<hole {class:?}>"),
    }
}

/// The entry that recovers `view` whole, if one does.
///
/// A *whole* recovery, not merely a top-level match: an entry that matches
/// here but leaves residuals of its own has not covered this position, and
/// typing the hole with its name would claim coverage the census is meant to
/// be measuring the absence of.
fn covered_by(view: &View, lexicon: &Lexicon) -> Option<String> {
    let recovered = unify(view, lexicon, FramePosition::Main);
    match recovered {
        Recovered::Invocation { ref entry, .. } if !recovered.has_residual() => Some(entry.clone()),
        _ => None,
    }
}

// ---------------------------------------------------------------------------
// Reporting
// ---------------------------------------------------------------------------

/// The whole sweep's reading, in the shape both the printed summary and the
/// written report need.
struct Census {
    lines: usize,
    full: usize,
    partial: usize,
    no_match: usize,
    /// Of the `NoMatch` lines, the ones no category parsed at all.
    unparsed: usize,
    /// Residual subtrees found, across every line — the ranking's denominator.
    residual_sites: usize,
    /// Lines whose recovery met a rival frame at least once
    /// ([`LineOutcome::ambiguities`] — a rival is not by itself an ambiguity),
    /// and each distinct rivalry with how often it was reached.
    ambiguous_lines: usize,
    ambiguities: BTreeMap<String, usize>,
    /// How many lines each category accepted.
    parsed_at: BTreeMap<&'static str, usize>,
}

impl Census {
    fn of(lines: &[Line], outcomes: &[LineOutcome]) -> Census {
        let mut census = Census {
            lines: lines.len(),
            full: 0,
            partial: 0,
            no_match: 0,
            unparsed: 0,
            residual_sites: 0,
            ambiguous_lines: 0,
            ambiguities: BTreeMap::new(),
            parsed_at: BTreeMap::new(),
        };
        for outcome in outcomes {
            match outcome.classification {
                Classification::Full => census.full += 1,
                Classification::Partial => census.partial += 1,
                Classification::NoMatch => census.no_match += 1,
            }
            match outcome.parsed_at {
                Some(kind) => *census.parsed_at.entry(kind_label(kind)).or_insert(0) += 1,
                None => census.unparsed += 1,
            }
            census.residual_sites += outcome.residuals.len();
            census.ambiguous_lines += usize::from(!outcome.ambiguities.is_empty());
            for tie in &outcome.ambiguities {
                *census.ambiguities.entry(tie.clone()).or_insert(0) += 1;
            }
        }
        census
    }

    /// The three classes with their shares, in the order they are declared —
    /// written once and read by both the printed summary and the report, so
    /// the two cannot come to disagree about the same numbers.
    fn breakdown(&self) -> String {
        [
            (Classification::Full, self.full),
            (Classification::Partial, self.partial),
            (Classification::NoMatch, self.no_match),
        ]
        .into_iter()
        .map(|(class, count)| format!("{} {count} ({})", class.label(), percent(count, self.lines)))
        .collect::<Vec<_>>()
        .join(", ")
    }

    fn print(&self, swept: &Swept, elapsed: std::time::Duration) {
        println!(
            "canon ability line sweep: {} abilities on non-todo faces → {} line(s), {} of those \
             abilities rendering no line at all; classified in {:.2}s",
            swept.abilities,
            self.lines,
            swept.silent,
            elapsed.as_secs_f64(),
        );
        println!(
            "  {} — of the NoMatch lines, {} parsed at no category at all",
            self.breakdown(),
            self.unparsed,
        );
        println!(
            "  {} line(s) where more than one authored frame matched a constituent, in {} \
             distinct pairing(s) — read the claims/guards below: a pairing one side dominates \
             is settled by the specificity order, and only an equal one is an ambiguity",
            self.ambiguous_lines,
            self.ambiguities.len(),
        );
        for (tie, count) in &self.ambiguities {
            println!("    {count}x {tie}");
        }
        println!(
            "  parsed at: {}",
            if self.parsed_at.is_empty() {
                "(nothing)".to_string()
            } else {
                self.parsed_at
                    .iter()
                    .map(|(kind, count)| format!("{kind}: {count}"))
                    .collect::<Vec<_>>()
                    .join(", ")
            },
        );
    }
}

fn kind_label(kind: FragmentKind) -> &'static str {
    match kind {
        FragmentKind::Nominal => "Nominal",
        FragmentKind::Sentence => "Sentence",
        FragmentKind::Cost => "Cost",
        FragmentKind::KeywordLine => "KeywordLine",
        FragmentKind::Ability => "Ability",
    }
}

fn percent(part: usize, whole: usize) -> String {
    if whole == 0 {
        return "n/a".to_string();
    }
    #[expect(
        clippy::cast_precision_loss,
        reason = "corpus counts are far below f64's exact-integer range"
    )]
    let share = part as f64 * 100.0 / whole as f64;
    format!("{share:.1}%")
}

/// One row of the ranked census: a residual shape, how many sites wear it, and
/// what a draft frame for it would start from.
struct Row {
    signature: Signature,
    count: usize,
    /// The first distinct card names the shape was seen on, at most three.
    /// Joined with `;` wherever they are printed — a card name may hold a
    /// comma of its own.
    examples: Vec<String>,
    /// The first line this shape was found on, kept for [`draft_for_row`]'s
    /// surface-text extraction. Not printed by the census — only `examples`
    /// is, and only card names.
    exemplar: Exemplar,
}

/// The first concrete site a ranked [`Row`] was seen at: enough of that
/// line's own data to reconstruct surface text and resolve self-reference,
/// without keeping the whole [`Line`] (whose `kind` is the category it was
/// *offered* to, not the one that actually accepted it — see `parsed_at`).
#[derive(Clone)]
struct Exemplar {
    /// The residual subtree itself, exactly as `classify` captured it.
    view: View,
    /// The exemplar line's own rendered text. For a
    /// [`Classification::NoMatch`] residual this *is* the residual's surface
    /// text, whole — see the module doc.
    line_text: String,
    line_name: String,
    /// The category the exemplar line actually parsed at — `kind:` for a
    /// draft built from a whole-line ([`Classification::NoMatch`]) residual.
    parsed_at: FragmentKind,
}

/// Every residual shape found, most-absorbing first.
///
/// Ties break on the key itself, so the ranking is stable run to run — two
/// shapes with equal counts must not trade places between two invocations that
/// swept the same corpus.
///
/// Rows are keyed on [`Signature::key`] alone and keep the **first** site's
/// signature, `holes` and all. That is sound only because the key determines
/// them: [`spell`] writes a hole marker into the key at the position it found
/// it, and never probes below the cut, so two residuals with equal keys agree
/// on every hole `path` and `entry`. A change to `spell` that let a marker
/// become key-invisible would silently attach one arbitrary site's holes to a
/// whole row.
fn rank(lines: &[Line], outcomes: &[LineOutcome]) -> Vec<Row> {
    let mut rows: BTreeMap<String, Row> = BTreeMap::new();
    for (line, outcome) in lines.iter().zip(outcomes) {
        for residual in &outcome.residuals {
            let row = rows
                .entry(residual.signature.key.clone())
                .or_insert_with(|| Row {
                    signature: residual.signature.clone(),
                    count: 0,
                    examples: Vec::new(),
                    exemplar: Exemplar {
                        view: residual.view.clone(),
                        line_text: line.text.clone(),
                        line_name: line.name.clone(),
                        parsed_at: outcome
                            .parsed_at
                            .expect("a line contributing a residual was parsed at some category"),
                    },
                });
            row.count += 1;
            if row.examples.len() < 3 && !row.examples.contains(&line.name) {
                row.examples.push(line.name.clone());
            }
        }
    }
    let mut rows: Vec<Row> = rows.into_values().collect();
    rows.sort_by(|left, right| {
        right
            .count
            .cmp(&left.count)
            .then_with(|| left.signature.key.cmp(&right.signature.key))
    });
    rows
}

fn holes_of(row: &Row) -> String {
    if row.signature.holes.is_empty() {
        return "(none)".to_string();
    }
    row.signature
        .holes
        .iter()
        .map(|hole| format!("{} = {}", hole.path, hole.entry))
        .collect::<Vec<_>>()
        .join(", ")
}

fn print_ranked(rows: &[Row], sites: usize) {
    println!(
        "residual census: {sites} residual site(s) in {} distinct shape(s); top {} —",
        rows.len(),
        PRINTED_ROWS.min(rows.len()),
    );
    for (rank, row) in rows.iter().take(PRINTED_ROWS).enumerate() {
        println!("  {:>3}. {}x {}", rank + 1, row.count, row.signature.key);
        println!("       cards: {}", row.examples.join("; "));
        println!("       pre-typed holes: {}", holes_of(row));
    }
    if rows.len() > PRINTED_ROWS {
        println!(
            "  … and {} further shape(s) — `--write-report PATH` for the whole table",
            rows.len() - PRINTED_ROWS,
        );
    }
}

/// The whole ranked table as markdown, for `--write-report`.
fn report(census: &Census, rows: &[Row]) -> String {
    let mut out = String::new();
    out.push_str("# Canon residual census\n\n");
    let _ = writeln!(
        out,
        "{} canon ability line(s): {}. {} of the NoMatch lines parsed at no category.\n",
        census.lines,
        census.breakdown(),
        census.unparsed,
    );
    let _ = writeln!(
        out,
        "{} residual site(s) in {} distinct shape(s).\n",
        census.residual_sites,
        rows.len(),
    );
    out.push_str("| rank | sites | signature | cards | pre-typed holes |\n");
    out.push_str("| ---: | ----: | --- | --- | --- |\n");
    for (rank, row) in rows.iter().enumerate() {
        let _ = writeln!(
            out,
            "| {} | {} | `{}` | {} | {} |",
            rank + 1,
            row.count,
            row.signature.key.replace('|', "\\|"),
            row.examples.join("; "),
            holes_of(row).replace('|', "\\|"),
        );
    }
    out
}

// ---------------------------------------------------------------------------
// Draft entries
// ---------------------------------------------------------------------------

/// One row, drafted or not — see the module doc.
enum Draft {
    Entry(ConstructorFrames),
    /// Why this row could not be drafted, naming the row's own key so the
    /// reader can find it in the printed ranking.
    Skipped(String),
}

/// `--drafts`'s top-level pass: the first `top_n` rows, each turned into a
/// [`Draft`], split into what compiled and what did not.
fn draft_entries(
    rows: &[Row],
    top_n: usize,
    lexicon: &Lexicon,
    catalogs: &Catalogs,
) -> (Vec<ConstructorFrames>, Vec<String>) {
    let mut drafts = Vec::new();
    let mut skipped = Vec::new();
    let mut used_names = HashSet::new();
    for (rank, row) in rows.iter().take(top_n).enumerate() {
        match draft_for_row(row, rank, lexicon, catalogs, &mut used_names) {
            Draft::Entry(entry) => drafts.push(entry),
            Draft::Skipped(reason) => skipped.push(reason),
        }
    }
    (drafts, skipped)
}

/// Drafts one row: base surface text, holes cut to `<Param(i)>`, a derived
/// `kind:` and a placeholder `constructor:` — then compiled, so nothing
/// [`draft_entries`] keeps can fail a human's first `cargo build` of it.
fn draft_for_row(
    row: &Row,
    rank: usize,
    lexicon: &Lexicon,
    catalogs: &Catalogs,
    used_names: &mut HashSet<String>,
) -> Draft {
    let Some(base) = base_text(&row.exemplar) else {
        return Draft::Skipped(format!(
            "`{}`: no surface text could be reconstructed for this shape",
            row.signature.key
        ));
    };
    let (text, params) = carve_holes(
        &base,
        &row.signature.holes,
        &row.exemplar.view,
        lexicon,
        &row.exemplar.line_name,
    );
    let kind = residual_kind(&row.exemplar.view, row.exemplar.parsed_at);
    let spec = FrameSpec::bare(&text);
    if let Err(error) =
        deckmaste_frames::compile::compile(&spec, kind, &params, catalogs, lexicon.macros())
    {
        return Draft::Skipped(format!(
            "`{}`: draft frame {text:?} did not compile: {error:#}",
            row.signature.key
        ));
    }
    let base_name = placeholder_constructor_name(&text, &row.exemplar.view, rank);
    let mut name = base_name.clone();
    let mut suffix = 2;
    while !used_names.insert(name.clone()) {
        name = format!("{base_name}{suffix}");
        suffix += 1;
    }
    Draft::Entry(ConstructorFrames {
        constructor: name,
        params,
        frames: vec![spec],
        kind: to_frame_kind(kind),
        body: None,
        announcement: false,
    })
}

/// The exemplar's own surface text for its residual, whole — see the module
/// doc's "Base text" paragraph.
fn base_text(exemplar: &Exemplar) -> Option<String> {
    if matches!(
        &exemplar.view,
        View::Newtype {
            name: "Fragment",
            ..
        }
    ) {
        return Some(self_referenced(&exemplar.line_text, &exemplar.line_name));
    }
    render_residual_text(&exemplar.view).ok()
}

/// `text` with every literal occurrence of the card's own name spelled `~`
/// instead — the inverse of [`constant_filler`]'s own `~` resolution, and
/// necessary for the same reason: legacy render already turned every `~` in
/// the card's authored ability into the card's full printed name, so a
/// residual's own rendered text names one specific card everywhere a general
/// frame would say `~`. Left undone, a draft built from it would only ever
/// match the exemplar it came from.
///
/// This is a textual substitution, not a self-reference *analysis*: a card
/// whose ability text names itself **non**-self-referentially (a rules text
/// that quotes the card's own name incidentally, rather than meaning "this
/// object") would be rewritten the same way, wrongly. No site in the current
/// top-20 output does this — every `~` this function introduces is a genuine
/// self-reference, checked directly — but a future corpus draw could, and
/// this function cannot tell the two cases apart.
///
/// A card name is never empty in the corpus, but `str::replace` on an empty
/// pattern inserts at every char boundary — guarded rather than trusted.
fn self_referenced(text: &str, name: &str) -> String {
    if name.is_empty() {
        return text.to_string();
    }
    text.replace(name, "~")
}

/// The English category a draft built from this residual should declare.
///
/// A [`Classification::NoMatch`] residual is the exemplar line's whole parsed
/// tree, still wrapped in its `Fragment::<kind>` marker — read directly off
/// it, which is exactly "the fragment category the residual sat in". A
/// residual that is not a whole line carries no such marker; if it is (after
/// peeling newtype wrappers, the same plumbing [`spell`] costs no depth for)
/// a bare `NominalPhrase`, it is a nominal filler regardless of what category
/// its enclosing line parsed at. Anything else falls back to `fallback` —
/// the exemplar line's own category — as the least-wrong default.
fn residual_kind(view: &View, fallback: FragmentKind) -> FragmentKind {
    let mut node = view;
    loop {
        match node {
            View::Newtype {
                name: "Fragment",
                variant: Some(variant),
                ..
            } => return fragment_kind_named(variant).unwrap_or(fallback),
            View::Newtype { inner, .. } => node = inner,
            _ => break,
        }
    }
    if node.type_name() == Some("NominalPhrase") {
        return FragmentKind::Nominal;
    }
    fallback
}

fn fragment_kind_named(variant: &str) -> Option<FragmentKind> {
    match variant {
        "Nominal" => Some(FragmentKind::Nominal),
        "Sentence" => Some(FragmentKind::Sentence),
        "Cost" => Some(FragmentKind::Cost),
        "KeywordLine" => Some(FragmentKind::KeywordLine),
        "Ability" => Some(FragmentKind::Ability),
        _ => None,
    }
}

/// Mirrors `deckmaste_frames::lexicon::fragment_kind_of`'s table, in reverse.
/// That function is private to its crate (a constructor entry's required
/// `kind:` is the schema type; nothing in `deckmaste_frames` reads a
/// `FragmentKind` back out of one), so the five-arm match is repeated here
/// rather than exposed for one caller.
fn to_frame_kind(kind: FragmentKind) -> FrameKind {
    match kind {
        FragmentKind::Nominal => FrameKind::Nominal,
        FragmentKind::Sentence => FrameKind::Sentence,
        FragmentKind::Cost => FrameKind::Cost,
        FragmentKind::KeywordLine => FrameKind::KeywordLine,
        FragmentKind::Ability => FrameKind::Ability,
    }
}

/// The literal text a *nullary* entry's own frame stands for — `~` resolved
/// to `self_name`, otherwise the frame text verbatim. A nullary entry (`This`,
/// `You`, `AnyTarget`, …) has no params to vary its rendering, so this is the
/// filler with no need to look at the card's own data at all; an entry whose
/// frame text holes a param returns `None`, leaving the caller to fall back
/// to reconstructing the covered child directly.
fn constant_filler(entry_name: &str, lexicon: &Lexicon, self_name: &str) -> Option<String> {
    let entry = lexicon
        .entries()
        .iter()
        .find(|entry| entry.name == entry_name)?;
    let text = &entry.frame.spec.text;
    if text.contains("<Param(") {
        return None;
    }
    Some(text.replace('~', self_name))
}

/// The surface text a single [`TypedHole`] stood for, if it can be recovered
/// without guessing: the constant text of a nullary covering entry, or —
/// failing that — the covered child's own reconstruction via
/// [`render_residual_text`].
fn filler_text(
    hole: &TypedHole,
    residual_root: &View,
    lexicon: &Lexicon,
    self_name: &str,
) -> Option<String> {
    if let Some(text) = constant_filler(&hole.entry, lexicon, self_name) {
        return Some(text);
    }
    let child = hole.path.resolve(residual_root)?;
    render_residual_text(child).ok()
}

/// The `params:` type name a hole's covering entry stands for — not the
/// entry's own name, which is a lexicon identity (`This`, `You`) and not one
/// of `deckmaste_cards::macros::param_types()`'s legal names. Both of the
/// pro-forms this census's top rows actually cover denote a
/// `deckmaste_core::Reference`; anything not in this small, explicit table
/// returns `None`, and the caller leaves that hole uncut rather than emit a
/// `params:` entry a human would have to correct by hand — the one field
/// D11 promises stays untouched is `body:`, not this one too.
fn param_type_for(entry_name: &str) -> Option<&'static str> {
    match entry_name {
        "This" | "You" => Some("Reference"),
        _ => None,
    }
}

/// Cuts `<Param(i)>` markers into `base` at each hole's own surface span. See
/// the module doc's "Cutting holes" paragraph for the search and fold rules.
/// Returns the carved text and the drafted `params:` list — one entry per
/// hole actually cut, in the order cut, so `<Param(i)>` in the text and
/// `params[i]` always agree.
fn carve_holes(
    base: &str,
    holes: &[TypedHole],
    residual_root: &View,
    lexicon: &Lexicon,
    self_name: &str,
) -> (String, Vec<String>) {
    struct Cut {
        start: usize,
        end: usize,
    }
    let lowered = base.to_ascii_lowercase();
    let mut cuts: Vec<Cut> = Vec::new();
    let mut params: Vec<String> = Vec::new();
    let mut cursor = 0usize;
    for hole in holes {
        let Some(param_type) = param_type_for(&hole.entry) else {
            continue;
        };
        let Some(filler) = filler_text(hole, residual_root, lexicon, self_name) else {
            continue;
        };
        if filler.is_empty() {
            continue;
        }
        let needle = filler.to_ascii_lowercase();
        let Some(relative) = lowered[cursor..].find(&needle) else {
            continue;
        };
        let start = cursor + relative;
        let end = start + filler.len();
        params.push(param_type.to_string());
        cuts.push(Cut { start, end });
        cursor = end;
    }
    let mut text = String::with_capacity(base.len());
    let mut pos = 0usize;
    for (index, cut) in cuts.iter().enumerate() {
        text.push_str(&base[pos..cut.start]);
        let _ = write!(text, "<Param({index})>");
        pos = cut.end;
    }
    text.push_str(&base[pos..]);
    (text, params)
}

/// The closed set of function words a placeholder name skips over: articles,
/// the trigger/targeting vocabulary that dominates MTG's own templating
/// (`when`, `target`, `each`, …), pronouns, and the leftover word fragments
/// a drafted line's own sigils split into (`Param`, and the bare letters an
/// apostrophe or a P/T slash leaves behind, filtered separately by the
/// length check below). Deliberately small: this names a placeholder, not a
/// parse — a stopword this list misses just costs a slightly less apt name,
/// never a wrong one.
const NAME_STOPWORDS: &[&str] = &[
    "a", "an", "the", "when", "whenever", "target", "targets", "each", "all", "this", "that",
    "these", "those", "it", "its", "you", "your", "and", "or", "to", "of", "for", "with", "at",
    "on", "in", "if", "able", "can", "until", "end", "turn", "param",
];

/// A placeholder `constructor:` name derived from the drafted text's own
/// head lexeme(s) — the brief's own wording — rather than from the
/// residual's AST shape: a reader scanning the file should learn what a row
/// is *about*. `text` (the carved frame text, sigils and all) is split on
/// anything that isn't an ASCII letter, so `<Param(0)>`/`~`/digits/
/// punctuation all fall away on their own; [`NAME_STOPWORDS`] is filtered
/// out of what remains, and the first one or two surviving words are
/// `PascalCased` and joined. A residual with no usable head lexeme at all
/// (every word was a stopword, or the text was empty) falls back to
/// [`drill_label`]'s AST-shape label instead — still a legal identifier,
/// just a less specific one. Always [`DRAFT_PREFIX`]-prefixed and never
/// authorial — see the constant's own doc.
fn placeholder_constructor_name(text: &str, view: &View, rank: usize) -> String {
    let head: String = text
        .split(|c: char| !c.is_ascii_alphabetic())
        .filter(|word| word.len() > 1)
        .map(str::to_ascii_lowercase)
        .filter(|word| !NAME_STOPWORDS.contains(&word.as_str()))
        .take(2)
        .map(|word| {
            let mut chars = word.chars();
            match chars.next() {
                Some(first) => format!("{}{}", first.to_ascii_uppercase(), chars.as_str()),
                None => String::new(),
            }
        })
        .collect();
    let label = if head.is_empty() { drill_label(view, 2).to_string() } else { head };
    let cleaned: String = label.chars().filter(char::is_ascii_alphanumeric).collect();
    let cleaned = if cleaned.is_empty() { format!("Residual{rank}") } else { cleaned };
    format!("{DRAFT_PREFIX}{cleaned}")
}

/// `view`'s own outermost label, peeled through newtypes exactly as
/// [`spell`] peels them. A handful of labels are pure category wrappers —
/// `Ability`'s `kind`, `Sentence`'s `body` — that would otherwise give every
/// triggered/activated/loyalty (or every declarative/imperative/complex)
/// residual the same placeholder name; for those, one field deeper is tried
/// instead, up to `budget` times, but only when the deeper node is itself
/// nameable — a bare `Seq` (as `SentenceBody::Independent`'s payload often
/// is) is not an improvement over the wrapper's own label, and is not used.
fn drill_label(view: &View, budget: usize) -> &'static str {
    let mut node = view;
    while let View::Newtype { inner, .. } = node {
        node = inner;
    }
    let label = node
        .variant_name()
        .or_else(|| node.type_name())
        .unwrap_or("Residual");
    if budget == 0 {
        return label;
    }
    let deeper_field = match label {
        "Ability" => "kind",
        "Sentence" => "body",
        _ => return label,
    };
    let View::Node { fields, .. } = node else {
        return label;
    };
    let Some((_, next)) = fields.iter().find(|(name, _)| *name == deeper_field) else {
        return label;
    };
    let mut peeled = next;
    while let View::Newtype { inner, .. } = peeled {
        peeled = inner;
    }
    if peeled
        .variant_name()
        .or_else(|| peeled.type_name())
        .is_some()
    {
        drill_label(next, budget - 1)
    } else {
        label
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    fn workspace_root() -> PathBuf {
        Path::new(env!("CARGO_MANIFEST_DIR")).join("../..")
    }

    fn fixture_lexicon() -> (Lexicon, Catalogs) {
        let root = workspace_root();
        let plugin_dir = root.join("plugins/builtin");
        let plugin = Plugin::load_with_sibling_prelude(&plugin_dir).expect("the builtin plugin");
        let catalogs = real_catalogs(&root).expect("the catalogs");
        let constructors =
            load_constructor_frames(&plugin_dir.join("frames")).expect("the frame catalog");
        let lexicon = Lexicon::assemble(&plugin.macros, &constructors, &catalogs)
            .expect("the lexicon assembles");
        (lexicon, catalogs)
    }

    fn fixture(name: &str, text: &str) -> Line {
        Line {
            name: name.to_string(),
            is_legendary: false,
            kind: FragmentKind::Sentence,
            text: text.to_string(),
        }
    }

    /// One hand-picked line per class.
    ///
    /// Two are canon's own printed lines. The middle one cannot be: canon
    /// holds no partial line at all — every wording the lexicon matches at
    /// the top level, it also recovers whole — so a partial has to be built,
    /// and it is built the smallest way there is. `DealsDamageToEach`'s
    /// wording is covered and canon uses it; the only constituent swapped is
    /// its predicate, for one the lexicon has no frame for (`creature`,
    /// `player`, and `<P> you control` are the framed predicates; a creature
    /// type is not one).
    #[test]
    fn the_three_classes_are_told_apart() {
        let (lexicon, catalogs) = fixture_lexicon();
        for (line, expected) in [
            (
                fixture(
                    "Lightning Bolt",
                    "Lightning Bolt deals 3 damage to any target.",
                ),
                Classification::Full,
            ),
            (
                fixture("Pyroclasm", "Pyroclasm deals 2 damage to each Goblin."),
                Classification::Partial,
            ),
            (
                fixture("Tome Scour", "Target player mills five cards."),
                Classification::NoMatch,
            ),
        ] {
            let outcome = classify(&line, &lexicon, &catalogs);
            assert_eq!(
                outcome.classification, expected,
                "{}: {}",
                line.name, line.text
            );
        }
    }

    /// A signature is a *grouping key*: two residuals of one shape must key
    /// alike, run to run and card to card. The second half is the
    /// load-bearing one — the two lines below differ in card name, in numeral
    /// and in creature type, and every one of those differences is inside the
    /// residual itself (a catalog atom carries its canonical and printed
    /// spellings as scalars). A key that kept scalar values would give them a
    /// row each and rank nothing.
    #[test]
    fn a_residual_signature_is_stable() {
        let (lexicon, catalogs) = fixture_lexicon();
        let line = fixture("Pyroclasm", "Pyroclasm deals 2 damage to each Goblin.");
        let first = classify(&line, &lexicon, &catalogs);
        let second = classify(&line, &lexicon, &catalogs);
        assert_eq!(
            first.classification,
            Classification::Partial,
            "the fixture must be partial for its residual to key"
        );
        assert!(!first.residuals.is_empty(), "a partial line has residuals");
        assert!(
            first
                .residuals
                .iter()
                .all(|residual| !residual.signature.key.is_empty()),
            "an empty key groups everything"
        );
        assert_eq!(first.residuals, second.residuals, "two runs, one key");

        let elsewhere = fixture(
            "Arc Lightning",
            "Arc Lightning deals 4 damage to each Wizard.",
        );
        let across = classify(&elsewhere, &lexicon, &catalogs);
        let keys = |outcome: &LineOutcome| -> Vec<String> {
            outcome
                .residuals
                .iter()
                .map(|residual| residual.signature.key.clone())
                .collect()
        };
        assert_eq!(
            keys(&first),
            keys(&across),
            "one shape on two cards is one key"
        );
        assert!(
            first.residuals[0]
                .signature
                .key
                .contains("CatalogKind::CreatureType"),
            "the shared key must still spell the shape it groups: {}",
            first.residuals[0].signature.key,
        );
        assert!(
            !first.residuals[0].signature.key.contains("Goblin"),
            "a key holding the card's own words groups nothing: {}",
            first.residuals[0].signature.key,
        );
    }

    /// The reproduction the floor exists for: sweep a corpus that is not
    /// there and every figure is a clean-looking zero. Nothing this command
    /// prints is an assertion, so without the floor that run exits 0 —
    /// the same shape as any regression that shrinks the corpus.
    #[test]
    fn an_absent_corpus_breaches_the_floor() {
        let error = run(ResidualArgs {
            plugin_dir: None,
            canon_dir: Some(PathBuf::from("/tmp/definitely-not-a-corpus-dir")),
            write_report: None,
            drafts: None,
            top: DRAFT_TOP_N,
        })
        .expect_err("an empty corpus must not pass");
        let message = format!("{error:#}");
        assert!(message.contains("population floor breached"), "{message}");
        for expected in ["canon lines swept: 0", "lines fully recovered: 0"] {
            assert!(message.contains(expected), "missing {expected}: {message}");
        }
    }

    #[test]
    fn the_pinned_population_itself_satisfies_the_floor() {
        check_floor(&RESIDUAL_FLOOR).expect("the floor must not breach itself");
    }

    /// Each figure is checked independently, so one shrinking number cannot
    /// hide behind the other holding.
    #[test]
    fn any_single_decrease_breaches_the_floor() {
        for (label, mutate) in [
            (
                "canon lines swept",
                (|population: &mut Population| population.lines -= 1) as fn(&mut Population),
            ),
            ("lines fully recovered", |population: &mut Population| {
                population.full -= 1;
            }),
        ] {
            let mut observed = RESIDUAL_FLOOR;
            mutate(&mut observed);
            let message = format!(
                "{:#}",
                check_floor(&observed).expect_err("a decrease must breach the floor")
            );
            assert!(message.contains(label), "expected {label} in: {message}");
        }
    }

    /// Growth is not a breach — a round that widens the sweep or authors a
    /// frame must not have to touch the constant to stay green.
    #[test]
    fn growth_above_the_floor_is_not_a_breach() {
        check_floor(&Population {
            lines: RESIDUAL_FLOOR.lines + 40,
            full: RESIDUAL_FLOOR.full + 10,
        })
        .expect("more corpus and more coverage must pass");
    }

    /// The ranking is a census, so its arithmetic has to close: every
    /// residual site lands in exactly one row, and the rows come back
    /// most-absorbing first.
    #[test]
    fn the_ranking_accounts_for_every_residual_site() {
        let lines = [fixture("A", ""), fixture("B", ""), fixture("C", "")];
        let site = |key: &str| ResidualSite {
            signature: Signature {
                key: key.to_string(),
                holes: Vec::new(),
            },
            view: View::Absent,
        };
        let outcome = |residuals: Vec<ResidualSite>| LineOutcome {
            classification: Classification::Partial,
            parsed_at: Some(FragmentKind::Sentence),
            residuals,
            ambiguities: Vec::new(),
        };
        let outcomes = [
            outcome(vec![site("wide"), site("narrow")]),
            outcome(vec![site("wide")]),
            outcome(vec![site("wide")]),
        ];
        let rows = rank(&lines, &outcomes);
        assert_eq!(rows.len(), 2);
        assert_eq!(rows[0].signature.key, "wide");
        assert_eq!(rows[0].count, 3);
        assert_eq!(rows[0].examples, ["A", "B", "C"]);
        assert_eq!(rows[1].signature.key, "narrow");
        assert_eq!(rows[1].count, 1);
        assert_eq!(
            rows.iter().map(|row| row.count).sum::<usize>(),
            Census::of(&lines, &outcomes).residual_sites,
        );
    }

    fn row_for(line: &Line, lexicon: &Lexicon, catalogs: &Catalogs) -> (Row, LineOutcome) {
        let outcome = classify(line, lexicon, catalogs);
        assert_eq!(
            outcome.residuals.len(),
            1,
            "the fixture is expected to contribute exactly one residual: {:?}",
            outcome.residuals
        );
        let residual = &outcome.residuals[0];
        let row = Row {
            signature: residual.signature.clone(),
            count: 1,
            examples: vec![line.name.clone()],
            exemplar: Exemplar {
                view: residual.view.clone(),
                line_text: line.text.clone(),
                line_name: line.name.clone(),
                parsed_at: outcome
                    .parsed_at
                    .expect("a line contributing a residual was parsed at some category"),
            },
        };
        (row, outcome)
    }

    /// Step 1's central requirement: the one hand-built `Partial` fixture
    /// this module has (canon itself holds none —
    /// `the_three_classes_are_told_apart`) must draft into something
    /// *authorable*, not merely printable — its `frames` text has to
    /// round-trip through the frame compiler at the draft's own declared
    /// `kind:`, with its own declared (here, empty) holes.
    ///
    /// The residual is the bare creature-type nominal `each` leaves
    /// uncovered, and it carries no pre-typed hole: a creature type is not
    /// one of the framed predicates, so `params: []` and a hole-free
    /// `frames:` text are the honest answer here, not a defect in the
    /// drafter.
    #[test]
    fn a_draft_from_the_partial_fixture_compiles() {
        let (lexicon, catalogs) = fixture_lexicon();
        let line = fixture("Pyroclasm", "Pyroclasm deals 2 damage to each Goblin.");
        let (row, outcome) = row_for(&line, &lexicon, &catalogs);
        assert_eq!(outcome.classification, Classification::Partial);
        assert!(
            row.signature.holes.is_empty(),
            "this residual has no pre-typed hole: {:?}",
            row.signature.holes
        );

        let mut used = HashSet::new();
        let draft = match draft_for_row(&row, 0, &lexicon, &catalogs, &mut used) {
            Draft::Entry(entry) => entry,
            Draft::Skipped(reason) => {
                panic!("the fixture's own residual must draft cleanly: {reason}")
            }
        };

        assert!(draft.params.is_empty(), "{:?}", draft.params);
        assert_eq!(draft.frames.len(), 1);
        let text = draft.frames[0].text.clone();
        assert_eq!(text, "Goblin");
        assert!(
            deckmaste_frames::witness::reserved_tokens(&text).is_empty(),
            "a witness leaked into the surface text: {text}"
        );
        assert!(draft.body.is_none(), "D11's contract: body: is left absent");
        assert_eq!(draft.kind, FrameKind::Nominal);
        assert!(draft.constructor.starts_with(DRAFT_PREFIX));

        // The central requirement, checked directly rather than only trusted
        // from `draft_for_row`'s own internal gate: the draft's own frame
        // text really does compile at the draft's own declared kind.
        let fragment_kind = residual_kind(&row.exemplar.view, row.exemplar.parsed_at);
        let compiled = deckmaste_frames::compile::compile(
            &draft.frames[0],
            fragment_kind,
            &draft.params,
            &catalogs,
            lexicon.macros(),
        )
        .expect("the draft's own frame text must compile at its own declared kind");
        assert!(compiled.holes.is_empty());
    }

    /// A self-reference hole needs no lookup into the card's own tree at
    /// all — `This`'s frame text is the constant `~`, resolved to the
    /// exemplar's own name — which is what lets this test build the hole by
    /// hand rather than through a full parse. `params:` reads the entry's
    /// *type* (`Reference`), never its lexicon identity (`This`) — the
    /// latter is not a legal `deckmaste_cards::macros::param_types()` name.
    #[test]
    fn a_self_reference_hole_carves_the_cards_own_name() {
        let (lexicon, _catalogs) = fixture_lexicon();
        let holes = vec![TypedHole {
            path: TreePath::default(),
            entry: "This".to_string(),
        }];
        let (text, params) = carve_holes(
            "Diregraf Ghoul enters the battlefield tapped.",
            &holes,
            &View::Absent,
            &lexicon,
            "Diregraf Ghoul",
        );
        assert_eq!(text, "<Param(0)> enters the battlefield tapped.");
        assert_eq!(params, vec!["Reference".to_string()]);
    }

    /// `You`, like `This`, is nullary — its own frame text (`"you"`) is the
    /// filler, case-insensitively found regardless of where the pronoun
    /// falls in the sentence — and, like `This`, drafts as its type
    /// (`Reference`), not its entry name.
    #[test]
    fn a_you_hole_carves_the_pronoun() {
        let (lexicon, _catalogs) = fixture_lexicon();
        let holes = vec![TypedHole {
            path: TreePath::default(),
            entry: "You".to_string(),
        }];
        let (text, params) = carve_holes("You draw a card.", &holes, &View::Absent, &lexicon, "X");
        assert_eq!(text, "<Param(0)> draw a card.");
        assert_eq!(params, vec!["Reference".to_string()]);
    }

    /// A hole naming no entry the lexicon actually has is left uncut rather
    /// than guessed at: the literal text survives, and `params:` gains no
    /// entry for it.
    #[test]
    fn an_unresolvable_hole_is_left_as_literal_text() {
        let (lexicon, _catalogs) = fixture_lexicon();
        let holes = vec![TypedHole {
            path: TreePath::default(),
            entry: "NoSuchEntry".to_string(),
        }];
        let (text, params) = carve_holes(
            "Sacrifice a creature.",
            &holes,
            &View::Absent,
            &lexicon,
            "X",
        );
        assert_eq!(text, "Sacrifice a creature.");
        assert!(params.is_empty());
    }

    /// A hole whose covering entry has no legal `params:` type — the entry
    /// exists and its constant text can be found, but nothing maps its name
    /// to one of `deckmaste_cards::macros::param_types()`'s names — is left
    /// uncut for the same reason: an illegal `params:` entry would be a
    /// second field a human has to fix by hand, breaching D11's "only
    /// `body:` blank" contract as surely as a wrong hole would.
    #[test]
    fn a_hole_with_no_legal_param_type_is_left_as_literal_text() {
        let (lexicon, _catalogs) = fixture_lexicon();
        let holes = vec![TypedHole {
            path: TreePath::default(),
            entry: "AnyTarget".to_string(),
        }];
        let (text, params) = carve_holes(
            "Sacrifice any target.",
            &holes,
            &View::Absent,
            &lexicon,
            "X",
        );
        assert_eq!(text, "Sacrifice any target.");
        assert!(params.is_empty());
    }

    /// A card's own self-reference is baked into legacy-rendered text as its
    /// full printed name, never as `~` — `base_text` must undo that, or
    /// every whole-line draft would be a one-card frame.
    #[test]
    fn base_text_generalizes_the_exemplars_own_name() {
        assert_eq!(
            self_referenced(
                "Diregraf Ghoul enters the battlefield tapped.",
                "Diregraf Ghoul"
            ),
            "~ enters the battlefield tapped."
        );
        assert_eq!(
            self_referenced(
                "[\u{2212}3]: Chandra, Torch of Defiance deals 4 damage to target creature.",
                "Chandra, Torch of Defiance"
            ),
            "[\u{2212}3]: ~ deals 4 damage to target creature."
        );
        // A name that never appears leaves the text untouched rather than
        // corrupting it.
        assert_eq!(self_referenced("Draw a card.", "Opt"), "Draw a card.");
    }

    /// A row `base_text` cannot reconstruct any surface text for at all
    /// (neither the whole-line shortcut — this view is not a `Fragment` —
    /// nor `render_residual_text`'s narrower one) is dropped rather than
    /// drafted with nothing to compile, and the reason names the row's own
    /// key so a reader can find it in the printed ranking. This pins the
    /// "drop with a reason" contract deterministically, independent of
    /// whatever the live corpus's own top rows happen to need it for today
    /// (see `every_draft_omits_body_and_leaks_no_witness` for that count).
    #[test]
    fn a_row_that_cannot_be_rendered_is_dropped_with_a_reason() {
        let (lexicon, catalogs) = fixture_lexicon();
        let row = Row {
            signature: Signature {
                key: "unrenderable-test-shape".to_string(),
                holes: Vec::new(),
            },
            count: 1,
            examples: vec!["Nobody".to_string()],
            exemplar: Exemplar {
                view: View::Seq(Vec::new()),
                line_text: String::new(),
                line_name: "Nobody".to_string(),
                parsed_at: FragmentKind::Sentence,
            },
        };
        let mut used = HashSet::new();
        match draft_for_row(&row, 0, &lexicon, &catalogs, &mut used) {
            Draft::Skipped(reason) => {
                assert!(reason.contains("unrenderable-test-shape"), "{reason}");
            }
            Draft::Entry(entry) => panic!("expected a skip, got a draft: {entry:?}"),
        }
    }

    /// D11's hard contracts, checked over every draft the real corpus
    /// produces rather than one hand-built case: `body:` is never set — not
    /// `None`, absent from the serialized entry entirely — and no reserved
    /// witness token ever reaches a draft's surface text, in memory or in
    /// the file `--drafts` writes. The arithmetic that makes "drop with a
    /// reason" honest is checked too: every one of the top-N rows becomes
    /// either a draft or a named skip, never silently neither — whether the
    /// live corpus's own top rows currently need the skip arm at all is not
    /// asserted here, since that count is corpus data, not a contract this
    /// generator owes (`a_row_that_cannot_be_rendered_is_dropped_with_a_reason`
    /// pins the mechanism itself).
    #[test]
    fn every_draft_omits_body_and_leaks_no_witness() {
        let (lexicon, catalogs) = fixture_lexicon();
        let root = workspace_root();
        let canon_dir = root.join("plugins/canon");
        let canon_plugin = Plugin::load_with_sibling_prelude(&canon_dir).expect("the canon plugin");
        let swept = collect_lines(&canon_dir, &canon_plugin.macros).expect("the corpus sweep");
        let outcomes: Vec<LineOutcome> = swept
            .lines
            .iter()
            .map(|line| classify(line, &lexicon, &catalogs))
            .collect();
        let ranked = rank(&swept.lines, &outcomes);
        let (drafts, skipped) = draft_entries(&ranked, DRAFT_TOP_N, &lexicon, &catalogs);
        assert!(
            !drafts.is_empty(),
            "the corpus's top residual signatures must draft at least one entry"
        );
        assert_eq!(
            drafts.len() + skipped.len(),
            DRAFT_TOP_N.min(ranked.len()),
            "every one of the top rows must become a draft or a named skip, never neither"
        );
        for draft in &drafts {
            assert!(
                draft.body.is_none(),
                "{}: body: must be absent",
                draft.constructor
            );
            for frame in &draft.frames {
                assert!(
                    deckmaste_frames::witness::reserved_tokens(&frame.text).is_empty(),
                    "{}: a witness leaked into {:?}",
                    draft.constructor,
                    frame.text
                );
            }
        }
        let text = deckmaste_core::ron::raw_options()
            .to_string_pretty(&drafts, PrettyConfig::default())
            .expect("the drafted catalog serializes");
        assert!(
            !text.contains("body:"),
            "a serialized draft must never spell body: {text}"
        );
    }
}
