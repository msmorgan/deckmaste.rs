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

use std::collections::BTreeMap;
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
use deckmaste_frames::Recovered;
use deckmaste_frames::View;
use deckmaste_frames::unify;
use deckmaste_frames::view;
use macro_ron::MacroSet;
use macro_ron::frames::FramePosition;
use macro_ron::frames::load_constructor_frames;

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

/// A position inside a residual that some entry already covers: a hole a draft
/// frame would come with pre-typed.
#[derive(Debug, Clone, PartialEq, Eq)]
struct TypedHole {
    /// Where in the residual, as a [`deckmaste_frames::TreePath`].
    path: String,
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
    residuals: Vec<Signature>,
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
            .map(|view| signature(view, lexicon))
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
    let key = spell(view, 0, "", lexicon, &mut holes);
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
    at: &str,
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
    let child = |(step, child): &(deckmaste_frames::PathStep, &View),
                 holes: &mut Vec<TypedHole>| {
        let path = format!("{at}{step}");
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
            let row = rows.entry(residual.key.clone()).or_insert_with(|| Row {
                signature: residual.clone(),
                count: 0,
                examples: Vec::new(),
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
                .all(|residual| !residual.key.is_empty()),
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
                .map(|residual| residual.key.clone())
                .collect()
        };
        assert_eq!(
            keys(&first),
            keys(&across),
            "one shape on two cards is one key"
        );
        assert!(
            first.residuals[0].key.contains("CatalogKind::CreatureType"),
            "the shared key must still spell the shape it groups: {}",
            first.residuals[0].key,
        );
        assert!(
            !first.residuals[0].key.contains("Goblin"),
            "a key holding the card's own words groups nothing: {}",
            first.residuals[0].key,
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
        let shape = |key: &str| Signature {
            key: key.to_string(),
            holes: Vec::new(),
        };
        let outcome = |residuals: Vec<Signature>| LineOutcome {
            classification: Classification::Partial,
            parsed_at: Some(FragmentKind::Sentence),
            residuals,
            ambiguities: Vec::new(),
        };
        let outcomes = [
            outcome(vec![shape("wide"), shape("narrow")]),
            outcome(vec![shape("wide")]),
            outcome(vec![shape("wide")]),
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
}
