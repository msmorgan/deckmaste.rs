//! `cargo xtask macro pilot` — the round's two ground-truth gates over real
//! canon cards:
//!
//! - **G3 shadow parity**: for every canon usage of a pilot *macro* (an
//!   `Origin::Macro` lexicon entry — not the raw constructor entries), does
//!   [`render_invocation_with`] reproduce, byte-for-byte after `fidelity`-style
//!   normalization, the same text the legacy per-ability renderer
//!   ([`deckmaste_cards::render`]) already prints for that ability? AND does
//!   `cargo xtask fidelity` still hold at its established figure — this command
//!   re-runs that exact check rather than asking a caller to remember to run
//!   both. (The plan states that figure as canon's 73 clean / 7 waived / 0
//!   failing; the total this command prints sums all four covered plugins, 78 /
//!   7 / 0, and is what [`COVERAGE_FLOOR`] pins.)
//! - **G4 ground truth**: for every canon ability line whose parse `unify`s to
//!   a non-`Residual` under the pilot lexicon, does the recovered structure —
//!   emitted as RON text and expanded — equal the card's own authored value,
//!   likewise expanded? Both directions go through
//!   [`deckmaste_frames::guard::normalized`]/`normalize_source`, the round's
//!   one canonicalizer, never a second implementation. The comparison is on
//!   **values**: an entry may stand for a value whose RON shape is not the flat
//!   application of its name, so one value has two spellings and only the
//!   expanded forms may be compared (see [`evaluate_g4`]).
//!
//! # Population and scope — read before trusting a "PASS"
//!
//! Both gates draw from the *same* per-line sweep
//! ([`collect_lines`]/[`evaluate_line`]): every `Ability::Keyword`, every
//! `Ability::Spell` with no `ability_word`, and every `Ability::Triggered`
//! with no `ability_word` (its *effect clause alone*, re-wrapped as a
//! synthetic spell — see [`testable_line`]'s own doc for why) in every
//! non-todo canon card face, isolated into its own single-ability
//! [`CardView`] and legacy-rendered on its own. `Ability::Static`/
//! `Activated`/`Innate` are **not** swept at all — none of the pilot's
//! macros model a whole static or activated-cost shape, so a top-level
//! match against one would always be `Residual` anyway, and isolating a
//! *sub*-fragment (a single cost component out of an activated ability's
//! list, in particular) has no public single-component legacy renderer to
//! isolate against. Concretely, this means `SacrificeThis` — usable only
//! inside an activated ability's cost list — sees zero coverage from this
//! sweep; that is a disclosed gap, not a hidden one. An `ability_word`-
//! carrying `Spell`/`Triggered`, and any isolated render that comes out to
//! something other than exactly one line (a modal's bulleted modes, a split
//! additional-cost clause — no pilot frame models a multi-sentence effect),
//! are excluded for the same reason: nothing this sweep could compare them
//! against fairly.
//!
//! **Every canon ability lands in exactly one bucket, at both layers —
//! nothing is silently dropped.** [`run`] prints a [`SweepExclusion`]
//! census, built from [`collect_lines`]'s own return value (why an ability
//! never became a swept `Line` at all — out of scope, an `ability_word`, or
//! a multi-line render), and
//! [`report_g3`]/[`report_g4`] print a further [`ExclusionReason`] census
//! for every swept line that is not equal/mismatched/diverged, each with an
//! `ensure!` tying its own two numbers together
//! (`checked/covered + excluded == lines swept`). So the population story
//! runs end to end — canon ability lines seen → swept → gated — and a line
//! disappearing from the numbers the way a whole macro's canon coverage
//! once did (this command's own fix-round Critical finding: every
//! `Keyword(Flying)` line was swept, then excluded with no record at all —
//! see the G5 findings file for the root cause and fix) cannot happen again
//! unnoticed, at either layer.
//!
//! **And the population itself is pinned, not merely printed.** Those two
//! partition checks are tautologies about bucketing, not about size — each
//! [`LineResult`] holds exactly one [`G3Outcome`] and one [`G4Outcome`] by
//! construction, so they hold however few lines there are, and they cannot
//! see a line lost *before* the results were built. Every gate verdict is an
//! `is_empty()`, which passes vacuously over an empty bucket. So the census
//! figures are compared against [`COVERAGE_FLOOR`] and **any decrease fails
//! the run** — read that constant before trusting a PASS from any gate here.
//!
//! G4's population is top-level `Recovered::Invocation`, any origin, full
//! stop — **not** "and every nested filler denotes a value too" (a fix-round
//! correction: it used to additionally require that, silently narrowing the
//! denominator). A line that recovers a non-`Residual` top level but leaves a
//! residual filler inside is *covered* and counted `Diverged` (with
//! `recovered_ron: None`, see [`G4Outcome::Diverged`]) — a recovery that is
//! provably incomplete is not asserted equal to the authored value, so it is
//! a real divergence, not an exclusion.
//!
//! G3's own population is the narrower one — "canon usage of a pilot
//! *macro*" — so a top-level `Origin::Constructor` match, or one
//! [`render_invocation_with`] cannot render for want of coverage (the
//! identical residual limitation, one level up — see
//! `deckmaste_frames::render`'s own module doc), is excluded from G3
//! specifically ([`ExclusionReason::NotMacroOrigin`]/
//! [`ExclusionReason::RenderFailed`]), recorded in its own census rather than
//! silently dropped. The one render failure that is **not** a coverage
//! exclusion is [`ReassembledDifferently`]: text that parses back as a
//! different constituency than the frame it came from is a defect in the
//! frame set, and fails this gate outright (see [`G3Outcome`]).

use std::collections::BTreeMap;
use std::path::Path;
use std::path::PathBuf;
use std::time::Instant;

use clap::Args;
use deckmaste_cards::fidelity;
use deckmaste_cards::fidelity::Oracle;
use deckmaste_cards::fidelity::Outcome as FidelityOutcome;
use deckmaste_cards::plugin::Plugin;
use deckmaste_cards::plugin::read;
use deckmaste_cards::render::CardView;
use deckmaste_cards::render::render;
use deckmaste_core::Ability;
use deckmaste_core::Card;
use deckmaste_core::CardFace;
use deckmaste_core::Supertype;
use deckmaste_core::plugin::CARDS_DIR;
use deckmaste_core::plugin::is_todo_source;
use deckmaste_english::CatalogKind;
use deckmaste_english::Catalogs;
use deckmaste_english::FragmentKind;
use deckmaste_english::parse_fragment;
use deckmaste_frames::Lexicon;
use deckmaste_frames::ReassembledDifferently;
use deckmaste_frames::Recovered;
use deckmaste_frames::View;
use deckmaste_frames::guard;
use deckmaste_frames::lexicon::Origin;
use deckmaste_frames::render_invocation_with;
use deckmaste_frames::unify;
use deckmaste_frames::view;
use macro_ron::MacroSet;
use macro_ron::frames::FramePosition;
use macro_ron::frames::load_constructor_frames;

/// Where G4's own analysis of every divergence it can find lives — named in
/// the FAIL output itself so a reader of a red gate does not have to go
/// looking for it. `pub(super)` so `cargo xtask macro census` can name it
/// too, alongside its own condensed G4 status line.
pub(super) const G5_FINDINGS_FILE: &str =
    "docs/superpowers/research/2026-07-30-macro-frames/pilot-constituency-findings.md";

/// Every population figure the gate verdicts below are only meaningful
/// *relative to* — see [`COVERAGE_FLOOR`].
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
struct Coverage {
    /// Canon abilities visited at all.
    seen: usize,
    /// Of those, the ones that became a gated [`Line`].
    swept: usize,
    /// Of those, the ones G3 reached a real verdict on (equal or mismatch).
    g3_checked: usize,
    /// Of those, the ones G4 reached a real verdict on (equal or diverged).
    g4_covered: usize,
    /// `cargo xtask fidelity`'s clean total across the four covered plugins.
    fidelity_clean: usize,
    /// Its waived total. A *ceiling*, not a floor — see [`COVERAGE_FLOOR`].
    fidelity_waived: usize,
}

/// The census this pilot observed at round 1's exit, pinned.
///
/// **This is a floor, not a target.** Every gate verdict below is
/// `is_empty()` over a bucket — "no mismatches", "no divergences", "nothing
/// failing" — which says nothing at all about how many lines were in the
/// bucket to begin with. An empty corpus therefore prints `G3 PASS (0
/// checked)`, `G4 PASS (0 covered)` and exits 0
/// (`cargo xtask macro pilot --canon-dir /tmp/does-not-exist` reproduces it),
/// and so does *any* regression that shrinks the population instead of
/// breaking a comparison.
///
/// The exposure is structural, not hypothetical. `ParseFailed` and
/// `NotAnInvocation` are exclusions shared by **both** gates
/// ([`evaluate_line`]'s early return): an english change that stops a
/// covered line from parsing drops that line out of the covered population
/// entirely, and a line no gate reaches a verdict on can never land in
/// `diverged` — so the regression shrinks the population and G4 prints PASS
/// over ground it no longer walks. G3's own denominator has the same
/// exposure one layer along, through [`ExclusionReason::RenderFailed`] (0
/// today): a render regression turns a checked line into an exclusion while
/// G3 keeps printing PASS. Pinning `g3_checked`/`g4_covered` is what closes
/// both, because an exclusion of any kind lowers them.
///
/// So the numbers are compared, not just printed, and **any decrease fails
/// the run** ([`check_coverage`]). `fidelity_waived` is the one inverted
/// entry — a *ceiling*: the design exists to retire waivers, so a newly
/// waived card must fail here exactly as a newly failing one does, which
/// `total_failing == 0` alone cannot see.
///
/// A round that legitimately grows coverage raises these deliberately, in
/// the same commit that grows it. They are not to be lowered to make a run
/// green.
const COVERAGE_FLOOR: Coverage = Coverage {
    seen: 104,
    swept: 61,
    g3_checked: 10,
    g4_covered: 18,
    fidelity_clean: 78,
    fidelity_waived: 7,
};

#[derive(Debug, Args)]
pub(super) struct PilotArgs {
    /// The plugin the pilot lexicon (frames + macros) is assembled from.
    /// Defaults to this workspace's `plugins/builtin`.
    #[arg(long)]
    plugin_dir: Option<PathBuf>,
    /// The canon corpus to sweep. Defaults to this workspace's
    /// `plugins/canon`.
    #[arg(long)]
    canon_dir: Option<PathBuf>,
    /// The oracle snapshot `cargo xtask fidelity` diffs against — see that
    /// command. Defaults to this workspace's `data/derived/cards.jsonl`.
    #[arg(long)]
    oracle: Option<PathBuf>,
}

/// # Errors
/// If the pilot lexicon, canon corpus, or oracle snapshot fails to load, or
/// — the gate itself — G3 or G4 fails.
pub(super) fn run(args: PilotArgs) -> anyhow::Result<()> {
    let workspace_root = Path::new(env!("CARGO_MANIFEST_DIR")).join("../..");
    let plugin_dir = args
        .plugin_dir
        .unwrap_or_else(|| workspace_root.join("plugins/builtin"));
    let canon_dir = args
        .canon_dir
        .unwrap_or_else(|| workspace_root.join("plugins/canon"));
    let oracle_path = args
        .oracle
        .unwrap_or_else(|| workspace_root.join(fidelity::ORACLE_SNAPSHOT));

    let plugin = Plugin::load_with_sibling_prelude(&plugin_dir)?;
    let catalogs = real_catalogs(&workspace_root)?;
    let constructors = load_constructor_frames(&plugin_dir.join("frames"))?;
    let lexicon = Lexicon::assemble(&plugin.macros, &constructors, &catalogs)?;

    let canon_plugin = Plugin::load_with_sibling_prelude(&canon_dir)?;

    let sweep_started = Instant::now();
    let swept = collect_lines(&canon_dir, &canon_plugin.macros)?;
    let results: Vec<LineResult> = swept
        .lines
        .iter()
        .map(|line| evaluate_line(line, &lexicon, &catalogs, &plugin.macros))
        .collect();
    let sweep_elapsed = sweep_started.elapsed();
    println!(
        "canon ability line census: {} seen, {} swept, {} not swept — {}",
        swept.seen(),
        swept.lines.len(),
        swept.excluded.values().sum::<usize>(),
        format_census(&swept.excluded),
    );
    println!(
        "swept {} canon line(s) in {:.2}s",
        swept.lines.len(),
        sweep_elapsed.as_secs_f64()
    );

    let g3 = report_g3(&results)?;
    let fidelity = report_fidelity(&workspace_root, &oracle_path)?;
    let g4 = report_g4(&results)?;

    // The coverage floor first: a verdict over a shrunken population is not
    // a verdict, so "the numbers moved" outranks "the comparison held" and
    // has to be reported even when a gate also failed.
    check_coverage(&Coverage {
        seen: swept.seen(),
        swept: swept.lines.len(),
        g3_checked: g3.counted,
        g4_covered: g4.counted,
        fidelity_clean: fidelity.clean,
        fidelity_waived: fidelity.waived,
    })?;

    anyhow::ensure!(
        g3.pass && fidelity.pass && g4.pass,
        "macro pilot: {}",
        [
            (!g3.pass).then_some("G3 shadow parity failed"),
            (!fidelity.pass).then_some("fidelity regressed"),
            (!g4.pass).then_some("G4 ground truth failed"),
        ]
        .into_iter()
        .flatten()
        .collect::<Vec<_>>()
        .join("; ")
    );
    Ok(())
}

/// Compares `observed` against [`COVERAGE_FLOOR`], reporting **every**
/// breach at once rather than the first — a shrinking population usually
/// shrinks several figures together, and seeing which ones moved is most of
/// the diagnosis.
///
/// # Errors
/// If any pinned figure decreased (or, for `fidelity_waived`, increased).
fn check_coverage(observed: &Coverage) -> anyhow::Result<()> {
    let mut breaches: Vec<String> = [
        ("canon abilities seen", observed.seen, COVERAGE_FLOOR.seen),
        ("lines swept", observed.swept, COVERAGE_FLOOR.swept),
        (
            "G3 usages checked",
            observed.g3_checked,
            COVERAGE_FLOOR.g3_checked,
        ),
        (
            "G4 lines covered",
            observed.g4_covered,
            COVERAGE_FLOOR.g4_covered,
        ),
        (
            "fidelity cards clean",
            observed.fidelity_clean,
            COVERAGE_FLOOR.fidelity_clean,
        ),
    ]
    .into_iter()
    .filter(|(_, observed, floor)| observed < floor)
    .map(|(label, observed, floor)| format!("{label}: {observed}, floor {floor}"))
    .collect();
    if observed.fidelity_waived > COVERAGE_FLOOR.fidelity_waived {
        breaches.push(format!(
            "fidelity cards waived: {}, ceiling {} (a waiver is what this design exists to \
             retire, so gaining one is a regression)",
            observed.fidelity_waived, COVERAGE_FLOOR.fidelity_waived,
        ));
    }
    anyhow::ensure!(
        breaches.is_empty(),
        "macro pilot: coverage floor breached — every gate verdict above is over a smaller \
         population than the one they were established on, so a PASS from any of them means \
         nothing. Fix the shrinkage; do not lower `COVERAGE_FLOOR`. {}",
        breaches.join("; "),
    );
    Ok(())
}

/// A concise, per-gate G3/G4 summary — `cargo xtask macro census`'s "per-gate
/// pilot status" line. Runs the identical sweep-and-evaluate pipeline `run`
/// itself assembles (same [`Lexicon`], same [`collect_lines`]/
/// [`evaluate_line`], same [`evaluate_g3`]/[`evaluate_g4`] verdicts) so there
/// is no second implementation of "what counts as PASS" to drift from the
/// real gate — only the top-level assembly is duplicated, and only because
/// `run` prints far more than a census caller wants (every mismatch/
/// divergence, both exclusion censuses, sweep timing). Fidelity is
/// deliberately not part of this summary: it is already its own gate in the
/// battery (`cargo xtask fidelity`), not something a caller of `macro
/// census` needs restated.
pub(super) struct GateStatus {
    pub g3_checked: usize,
    pub g3_equal: usize,
    pub g3_mismatched: usize,
    pub g3_pass: bool,
    pub g4_covered: usize,
    pub g4_equal: usize,
    pub g4_diverged: usize,
    pub g4_pass: bool,
}

/// # Errors
/// If the pilot lexicon or canon corpus fails to load.
pub(super) fn gate_status(plugin_dir: &Path, canon_dir: &Path) -> anyhow::Result<GateStatus> {
    let workspace_root = Path::new(env!("CARGO_MANIFEST_DIR")).join("../..");
    let plugin = Plugin::load_with_sibling_prelude(plugin_dir)?;
    let catalogs = real_catalogs(&workspace_root)?;
    let constructors = load_constructor_frames(&plugin_dir.join("frames"))?;
    let lexicon = Lexicon::assemble(&plugin.macros, &constructors, &catalogs)?;

    let canon_plugin = Plugin::load_with_sibling_prelude(canon_dir)?;
    let swept = collect_lines(canon_dir, &canon_plugin.macros)?;
    let results: Vec<LineResult> = swept
        .lines
        .iter()
        .map(|line| evaluate_line(line, &lexicon, &catalogs, &plugin.macros))
        .collect();

    let mut g3_equal = 0usize;
    let mut g3_mismatched = 0usize;
    let mut g3_reassembled = 0usize;
    let mut g4_equal = 0usize;
    let mut g4_diverged = 0usize;
    for result in &results {
        match result.g3 {
            G3Outcome::Equal => g3_equal += 1,
            G3Outcome::Mismatch { .. } => g3_mismatched += 1,
            // Counted with the mismatches, not with the exclusions: the line
            // *was* checked, and it failed — see `G3Outcome`'s own doc.
            G3Outcome::ReassembledDifferently { .. } => g3_reassembled += 1,
            G3Outcome::Excluded(_) => {}
        }
        match result.g4 {
            G4Outcome::Equal => g4_equal += 1,
            G4Outcome::Diverged { .. } => g4_diverged += 1,
            G4Outcome::Excluded(_) => {}
        }
    }
    Ok(GateStatus {
        g3_checked: g3_equal + g3_mismatched + g3_reassembled,
        g3_equal,
        g3_mismatched: g3_mismatched + g3_reassembled,
        g3_pass: g3_mismatched + g3_reassembled == 0,
        g4_covered: g4_equal + g4_diverged,
        g4_equal,
        g4_diverged,
        g4_pass: g4_diverged == 0,
    })
}

// ---------------------------------------------------------------------------
// Corpus loading
// ---------------------------------------------------------------------------

pub(super) fn real_catalogs(workspace_root: &Path) -> anyhow::Result<Catalogs> {
    let dir = workspace_root.join("data/gen/catalogs");
    let load = |name: &str| -> anyhow::Result<Vec<String>> {
        let path = dir.join(format!("{name}.txt"));
        Ok(std::fs::read_to_string(&path)
            .map_err(|error| anyhow::anyhow!("reading {}: {error}", path.display()))?
            .lines()
            .map(str::to_string)
            .collect())
    };
    Ok(Catalogs::default()
        .with_catalog(CatalogKind::KeywordAbility, load("keyword-abilities")?)
        .with_catalog(CatalogKind::KeywordAction, load("keyword-actions")?)
        .with_catalog(CatalogKind::AbilityWord, load("ability-words")?)
        .with_catalog(CatalogKind::ArtifactType, load("artifact-types")?)
        .with_catalog(CatalogKind::BattleType, load("battle-types")?)
        .with_catalog(CatalogKind::CreatureType, load("creature-types")?)
        .with_catalog(CatalogKind::EnchantmentType, load("enchantment-types")?)
        .with_catalog(CatalogKind::LandType, load("land-types")?)
        .with_catalog(CatalogKind::PlaneswalkerType, load("planeswalker-types")?)
        .with_catalog(CatalogKind::SpellType, load("spell-types")?)
        .with_catalog(CatalogKind::Supertype, load("supertypes")?)
        .with_catalog(CatalogKind::CardType, load("card-types")?))
}

/// One isolated, testable canon line: a single `Keyword` or `Spell` ability,
/// legacy-rendered on its own, alongside its own authored value already
/// normalized to a comparable [`View`] (see the module doc's "Population and
/// scope").
struct Line {
    /// `path.ron: Face (Keyword|Spell)`, for diagnostics.
    label: String,
    name: String,
    is_legendary: bool,
    kind: FragmentKind,
    text: String,
    /// The RON type this line's authored value normalizes at —
    /// `"KeywordAbility"` for a keyword line, `"OneShotEffect"` for a spell
    /// effect (which reads a raw `Action`-flattened constructor like
    /// `DealDamage(...)` directly too — `OneShotEffect::Act` is
    /// `#[macro_ron(flatten)]`, `crates/deckmaste_core/src/effect.rs`).
    ron_type: &'static str,
    authored_view: View,
}

/// Why one canon ability never became a swept [`Line`] at all — the layer
/// *above* [`ExclusionReason`]: that census accounts for every swept line;
/// this one accounts for every canon ability the sweep looked at in the
/// first place, so the population story runs end to end (canon ability
/// lines seen → swept → gated) with nothing narrowed silently at either
/// layer.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Ord, PartialOrd)]
enum SweepExclusion {
    /// `Ability::Static`/`Activated`/`Innate` (or anything else that is not
    /// `Keyword`/`Spell`/`Triggered`) — out of this sweep's scope by design
    /// (see the module doc's "Population and scope"), not a gate finding.
    OutOfScopeAbilityKind,
    /// A `Spell`/`Triggered` ability carrying an `ability_word` — the
    /// isolated render would prepend "Word — ", which this sweep's text
    /// comparison is not set up to strip.
    AbilityWord,
    /// The isolated render produced something other than exactly one line
    /// (a modal's bulleted modes, a split additional-cost clause) — no
    /// pilot frame models a multi-sentence effect.
    MultiLineEffect,
}

impl SweepExclusion {
    const fn label(self) -> &'static str {
        match self {
            SweepExclusion::OutOfScopeAbilityKind => "out-of-scope-ability-kind",
            SweepExclusion::AbilityWord => "ability-word",
            SweepExclusion::MultiLineEffect => "multi-line-effect",
        }
    }
}

/// Every canon ability visited, split into the [`Line`]s the sweep will
/// gate and a census of why every other ability was not one of them —
/// printed in full by [`run`] so "canon ability lines seen → swept → gated"
/// is a complete, honest chain, not just the gated tail of it.
struct Swept {
    lines: Vec<Line>,
    excluded: BTreeMap<&'static str, usize>,
}

impl Swept {
    /// Every canon ability this sweep visited, whether or not it became a
    /// `Line` — `lines.len() + excluded.values().sum()`.
    fn seen(&self) -> usize {
        self.lines.len() + self.excluded.values().sum::<usize>()
    }
}

fn collect_lines(canon_dir: &Path, macros: &MacroSet) -> anyhow::Result<Swept> {
    let mut lines = Vec::new();
    let mut excluded: BTreeMap<&'static str, usize> = BTreeMap::new();
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
                match testable_line(&path, face, is_legendary, ability) {
                    Ok(line) => lines.push(line),
                    Err(reason) => *excluded.entry(reason.label()).or_insert(0) += 1,
                }
            }
        }
    }
    Ok(Swept { lines, excluded })
}

pub(super) fn faces(card: &Card) -> Vec<&CardFace> {
    match card {
        Card::Normal(face) => vec![face],
        Card::TwoFaced { front, back, .. } => vec![front, back],
    }
}

/// Peels a leading `Ability::Expanded` wrapper — mirrors
/// `deckmaste_cards::render`'s own private `peel_expanded`, which this
/// module cannot call (it is not exported), kept minimal since only the
/// two ability kinds below are ever isolated.
pub(super) fn peel_expanded(ability: &Ability) -> &Ability {
    match ability {
        Ability::Expanded(exp) => peel_expanded(&exp.value),
        other => other,
    }
}

/// Builds one [`Line`] from `ability`, if it is one of the kinds this sweep
/// tests (see the module doc). Renders `ability` in isolation — a synthetic
/// single-ability [`CardView`] — so the legacy text is exactly what that one
/// ability contributes, independent of its siblings on the same face.
///
/// A `Triggered` ability's *effect clause* is tested too, not just
/// `Keyword`/`Spell`: the overwhelming majority of "draw a card"-shaped
/// canon text is a trigger's effect ("When ~ enters, draw a card."), not a
/// spell's own — excluding it would leave most of the pilot macros'
/// real usage untested. Its `event`/`condition`/`ability_word` are dropped
/// and `effect` alone is re-wrapped as a synthetic `Ability::Spell` for
/// rendering, reaching the *identical* `effect::effect` renderer a real
/// spell's effect does (`deckmaste_cards::render::rules`'s own dispatch) —
/// only the surrounding sentence frame (capitalization, no "When …,"
/// lead-in) differs, which does not matter here: this sweep tests the
/// effect's own structure, not the trigger wrapping it.
fn testable_line(
    path: &Path,
    face: &CardFace,
    is_legendary: bool,
    ability: &Ability,
) -> Result<Line, SweepExclusion> {
    let peeled = peel_expanded(ability);
    let (kind, ron_type, authored_view, tag, rendering_ability) = match peeled {
        Ability::Keyword(k) => (
            FragmentKind::KeywordLine,
            "KeywordAbility",
            guard::normalized(k.clone()),
            "Keyword",
            ability.clone(),
        ),
        Ability::Spell(s) => {
            if s.ability_word.is_some() {
                return Err(SweepExclusion::AbilityWord);
            }
            (
                FragmentKind::Sentence,
                "OneShotEffect",
                guard::normalized(s.effect.clone()),
                "Spell",
                ability.clone(),
            )
        }
        Ability::Triggered(t) => {
            if t.ability_word.is_some() {
                return Err(SweepExclusion::AbilityWord);
            }
            (
                FragmentKind::Sentence,
                "OneShotEffect",
                guard::normalized(t.effect.clone()),
                "Triggered-effect",
                Ability::Spell(std::sync::Arc::new(deckmaste_core::SpellAbility {
                    ability_word: None,
                    effect: t.effect.clone(),
                })),
            )
        }
        _ => return Err(SweepExclusion::OutOfScopeAbilityKind),
    };
    let isolated = CardView {
        name: &face.name,
        mana_cost: None,
        supertypes: &[],
        types: &[],
        subtypes: &[],
        power: None,
        toughness: None,
        abilities: std::slice::from_ref(&rendering_ability),
    };
    let mut rules = render(&isolated).rules;
    if rules.len() != 1 {
        // A multi-line spell effect (a modal's bulleted modes, an
        // additional-cost clause split onto its own line) is out of scope —
        // no pilot frame models a multi-sentence effect, so a fair G3/G4
        // comparison needs exactly one line to compare against.
        return Err(SweepExclusion::MultiLineEffect);
    }
    let text = rules.remove(0);
    Ok(Line {
        label: format!("{}: {} ({tag})", path.display(), face.name),
        name: face.name.to_string(),
        is_legendary,
        kind,
        text,
        ron_type,
        authored_view,
    })
}

/// The `.ron` files under `dir` at any depth, sorted; an absent directory is
/// empty. A private copy of the small walker every plugin-tree reader in
/// this workspace carries (`macro_ron::frames::ron_files_recursive`,
/// `deckmaste_cards::plugin::ron_files_recursive`, `crate::macros::templates`'s
/// own copy) — xtask depends on none of those crates' internals, so this
/// stays its own copy rather than a new public API surface for one caller.
pub(super) fn ron_files_recursive(dir: &Path) -> anyhow::Result<Vec<PathBuf>> {
    if !dir.exists() {
        return Ok(Vec::new());
    }
    let mut files = Vec::new();
    let mut subdirs = Vec::new();
    for entry in std::fs::read_dir(dir)
        .map_err(|error| anyhow::anyhow!("reading {}: {error}", dir.display()))?
    {
        let entry = entry?;
        let path = entry.path();
        if entry.file_type()?.is_dir() {
            subdirs.push(path);
        } else if path.extension().is_some_and(|ext| ext == "ron") {
            files.push(path);
        }
    }
    subdirs.sort();
    for subdir in subdirs {
        files.extend(ron_files_recursive(&subdir)?);
    }
    files.sort();
    Ok(files)
}

// ---------------------------------------------------------------------------
// Per-line evaluation
// ---------------------------------------------------------------------------

/// Why one line is excluded from a gate's checked/covered population —
/// every exit that is *not* a genuine equal/mismatch/diverged verdict has
/// one of these, and [`report_g3`]/[`report_g4`] print a full census of
/// them so no line disappears silently (the structural half of the fix-round
/// Critical finding: a whole macro's canon coverage vanished into an
/// unrecorded `NotCovered` exit).
#[derive(Debug, Clone, Copy, PartialEq, Eq, Ord, PartialOrd)]
enum ExclusionReason {
    /// The legacy-rendered text itself does not parse cleanly — a
    /// `deckmaste_english` grammar gap, not this gate's concern. Shared by
    /// G3 and G4 (neither can proceed past this point).
    ParseFailed,
    /// `unify` returned `Recovered::Residual` at the top level — the line
    /// does not use any lexicon entry at all. Shared by G3 and G4 (the
    /// brief's own G4 population line: "unify's to a non-Residual").
    NotAnInvocation,
    /// (G3 only.) The top-level recovered entry is a raw constructor
    /// (`Origin::Constructor`), not a pilot *macro* — outside G3's own
    /// population by the brief's literal wording ("canon usage of a pilot
    /// macro").
    NotMacroOrigin,
    /// (G3 only.) `render_invocation_with` itself returned `Err` — most
    /// commonly an unreconstructable residual filler (see
    /// `deckmaste_frames::render`'s own module doc).
    RenderFailed,
}

impl ExclusionReason {
    const fn label(self) -> &'static str {
        match self {
            ExclusionReason::ParseFailed => "parse-failed",
            ExclusionReason::NotAnInvocation => "not-an-invocation",
            ExclusionReason::NotMacroOrigin => "not-macro-origin",
            ExclusionReason::RenderFailed => "render-failed",
        }
    }
}

enum G3Outcome {
    Excluded(ExclusionReason),
    Equal,
    Mismatch {
        rendered: String,
    },
    /// The render's own re-parse cross-check refused the line
    /// ([`ReassembledDifferently`]). Deliberately **not** an
    /// [`ExclusionReason`]: an exclusion means "this gate has nothing to say
    /// about this line", and a frame that renders text reading as some other
    /// constituency is the opposite of nothing to say. Bucketing it with the
    /// ordinary render failures would file a defect as missing coverage.
    ReassembledDifferently {
        detail: String,
    },
}

enum G4Outcome {
    Excluded(ExclusionReason),
    Equal,
    Diverged {
        /// `None` when the recovery is **incomplete** — it holds a
        /// `Recovered::Residual`, which stands for a constituent no lexicon
        /// entry covered and therefore denotes no value at all. That is
        /// *still* a divergence, not an exclusion (see [`evaluate_g4`]'s
        /// doc): the population is "unify's to a non-Residual" at the top
        /// level, full stop, and a recovery that is provably incomplete
        /// cannot be asserted equal to the fully concrete authored value.
        ///
        /// `Some` on every other divergence, where the recovery *does* denote
        /// a value and this is its RON text — shown for the reader, never
        /// compared: the comparison is on expanded values (see
        /// [`evaluate_g4`]).
        recovered_ron: Option<String>,
        reason: String,
    },
}

struct LineResult<'a> {
    line: &'a Line,
    g3: G3Outcome,
    g4: G4Outcome,
}

fn evaluate_line<'a>(
    line: &'a Line,
    lexicon: &Lexicon,
    catalogs: &Catalogs,
    macros: &MacroSet,
) -> LineResult<'a> {
    let report = parse_fragment(
        &line.text,
        catalogs,
        line.kind,
        &line.name,
        line.is_legendary,
    );
    if !report.clean() {
        // The legacy renderer produced text the parser itself declines —
        // not this gate's concern (a `deckmaste_english` grammar gap), so
        // the line is simply out of both populations. Recorded (not
        // silently dropped) via `ExclusionReason::ParseFailed`.
        return LineResult {
            line,
            g3: G3Outcome::Excluded(ExclusionReason::ParseFailed),
            g4: G4Outcome::Excluded(ExclusionReason::ParseFailed),
        };
    }
    let fragment = report
        .into_fragment()
        .expect("a clean report has a fragment");
    let target = view::of(&fragment);
    let recovered = unify(&target, lexicon, FramePosition::Main);

    let g4 = evaluate_g4(&recovered, line, macros);
    let g3 = evaluate_g3(&recovered, line, lexicon, catalogs);
    LineResult { line, g3, g4 }
}

/// G4's population is a top-level `Recovered::Invocation` (any origin — macro
/// or constructor), full stop. An incomplete recovery does **not** narrow that
/// population (a fix-round correction: it used to, silently) — it is reported
/// as `Diverged` with `recovered_ron: None`.
///
/// # The comparison is on values, not spellings
///
/// Both sides go through
/// [`guard::normalize_source`]/
/// [`normalized`](deckmaste_frames::guard::normalized) — read at the line's own
/// RON type, fully expanded, compared as `View`s. So two spellings of one value
/// are equal, which they must be: an entry's emission is its body term with the
/// recovered arguments filled ([`Recovered::to_ron`]), and a body exists
/// precisely because the value's RON shape is not the flat application of the
/// entry's name. `GainLife(3)` on the card and `By(You, GainLife(3))` from the
/// recovery are the same `OneShotEffect`; a string comparison would call them
/// different, and did. The recovered RON text survives only as something to
/// *print*.
///
/// The one thing a value comparison still cannot do is compare against a
/// non-value: a recovery holding a `Recovered::Residual` denotes nothing at
/// all, and that is the `recovered_ron: None` arm.
fn evaluate_g4(recovered: &Recovered, line: &Line, macros: &MacroSet) -> G4Outcome {
    let Recovered::Invocation { .. } = recovered else {
        return G4Outcome::Excluded(ExclusionReason::NotAnInvocation);
    };
    match recovered.to_ron(macros) {
        Ok(ron_text) => match guard::normalize_source(macros, line.ron_type, &ron_text) {
            Ok(recovered_view) if recovered_view == line.authored_view => G4Outcome::Equal,
            Ok(_) => G4Outcome::Diverged {
                recovered_ron: Some(ron_text),
                reason: "expands to a different normal form than the authored RON".to_string(),
            },
            Err(error) => G4Outcome::Diverged {
                recovered_ron: Some(ron_text),
                reason: format!("failed to expand as `{}`: {error:#}", line.ron_type),
            },
        },
        // The error names the argument position and shows what the
        // unrecovered `View` held — see `Recovered::to_ron`. Without that
        // this arm printed identically for every divergence it produced,
        // and it currently produces *all* of them.
        Err(error) => G4Outcome::Diverged {
            recovered_ron: None,
            reason: format!(
                "the recovery is incomplete: a constituent no lexicon entry covers came \
                 back as a residual, which denotes no value to compare — {error:#}"
            ),
        },
    }
}

fn evaluate_g3(
    recovered: &Recovered,
    line: &Line,
    lexicon: &Lexicon,
    catalogs: &Catalogs,
) -> G3Outcome {
    let Recovered::Invocation { entry, .. } = recovered else {
        return G3Outcome::Excluded(ExclusionReason::NotAnInvocation);
    };
    let is_macro = lexicon
        .entries()
        .iter()
        .any(|candidate| candidate.name == *entry && candidate.origin == Origin::Macro);
    if !is_macro {
        return G3Outcome::Excluded(ExclusionReason::NotMacroOrigin);
    }
    match render_invocation_with(
        recovered,
        lexicon,
        FramePosition::Main,
        // The canon line's own category — the same one `evaluate_line`
        // parsed it at, so the render is checked against the category the
        // legacy text was actually written in rather than whichever one the
        // lexicon happened to register the winning frame at first.
        line.kind,
        catalogs,
        &line.name,
        line.is_legendary,
    ) {
        Ok(rendered) => {
            let normalized_rendered = fidelity::normalize(&rendered, &line.name);
            let normalized_legacy = fidelity::normalize(&line.text, &line.name);
            if normalized_rendered == normalized_legacy {
                G3Outcome::Equal
            } else {
                G3Outcome::Mismatch { rendered }
            }
        }
        // Two different failures wear the same `Err`, and only one of them is
        // a coverage gap. `ReassembledDifferently` says the render produced
        // text that parses back as a different constituency than the frame it
        // came from — a defect in the frame set, which the gate reports rather
        // than sets aside.
        Err(error) => match error.downcast_ref::<ReassembledDifferently>() {
            Some(_) => G3Outcome::ReassembledDifferently {
                detail: format!("{error:#}"),
            },
            None => G3Outcome::Excluded(ExclusionReason::RenderFailed),
        },
    }
}

// ---------------------------------------------------------------------------
// Reporting
// ---------------------------------------------------------------------------

/// A per-reason exclusion tally, printed alongside every gate's verdict so
/// every swept line is accounted for somewhere in the output — the
/// structural half of the fix-round Critical finding. `BTreeMap` keeps the
/// printed order stable (alphabetical by reason label) run to run.
fn census(reasons: impl Iterator<Item = ExclusionReason>) -> BTreeMap<&'static str, usize> {
    let mut counts = BTreeMap::new();
    for reason in reasons {
        *counts.entry(reason.label()).or_insert(0usize) += 1;
    }
    counts
}

fn format_census(counts: &BTreeMap<&'static str, usize>) -> String {
    if counts.is_empty() {
        return "(none)".to_string();
    }
    counts
        .iter()
        .map(|(label, count)| format!("{label}: {count}"))
        .collect::<Vec<_>>()
        .join(", ")
}

/// One gate's verdict and the size of the population it was reached over —
/// the pair [`check_coverage`] needs, since the verdict alone cannot
/// distinguish "nothing was wrong" from "nothing was checked".
#[derive(Debug)]
struct GateReport {
    pass: bool,
    counted: usize,
}

fn report_g3(results: &[LineResult]) -> anyhow::Result<GateReport> {
    let mut equal = 0usize;
    let mut mismatches: Vec<(&str, &str, &str)> = Vec::new();
    let mut reassembled: Vec<(&str, &str)> = Vec::new();
    let excluded = census(results.iter().filter_map(|result| match &result.g3 {
        G3Outcome::Excluded(reason) => Some(*reason),
        _ => None,
    }));
    for result in results {
        match &result.g3 {
            G3Outcome::Excluded(_) => {}
            G3Outcome::Equal => equal += 1,
            G3Outcome::Mismatch { rendered } => {
                mismatches.push((&result.line.label, &result.line.text, rendered));
            }
            G3Outcome::ReassembledDifferently { detail } => {
                reassembled.push((&result.line.label, detail));
            }
        }
    }
    for (label, legacy, rendered) in &mismatches {
        println!("G3 MISMATCH {label}");
        println!("  legacy render:      {legacy}");
        println!("  render_invocation:  {rendered}");
    }
    let checked = equal + mismatches.len() + reassembled.len();
    let excluded_total: usize = excluded.values().sum();
    let pass = mismatches.is_empty();
    println!(
        "G3 shadow parity: {}: {checked} canon pilot-macro usage(s) checked ({equal} equal, {} \
         mismatched)",
        if pass { "PASS" } else { "FAIL" },
        mismatches.len(),
    );
    println!(
        "  exclusion census: {excluded_total} excluded of {} line(s) swept — {}",
        results.len(),
        format_census(&excluded),
    );
    // An `ensure!`, not a `debug_assert!`: this runs once per gate over a
    // `Vec` length, so the cost is nil, and a broken partition means the
    // printed census does not account for every swept line — which is a
    // gate failure, not a debug-build panic that vanishes in release.
    anyhow::ensure!(
        checked + excluded_total == results.len(),
        "every swept line must land in exactly one G3 bucket, but {checked} checked + \
         {excluded_total} excluded != {} swept",
        results.len(),
    );
    // A hard failure rather than a printed verdict: a wording that re-parses
    // into a different constituency than the frame it was rendered from is a
    // defect in the frame set, not a measurement of how much English is
    // covered, and it must not be absorbed by a gate that is otherwise
    // reporting a clean sweep.
    anyhow::ensure!(
        reassembled.is_empty(),
        "{} rendered line(s) did not reassemble the frame they were rendered from:\n  {}",
        reassembled.len(),
        reassembled
            .iter()
            .map(|(label, detail)| format!("{label}: {detail}"))
            .collect::<Vec<_>>()
            .join("\n  "),
    );
    Ok(GateReport {
        pass,
        counted: checked,
    })
}

fn report_g4(results: &[LineResult]) -> anyhow::Result<GateReport> {
    let mut equal = 0usize;
    let mut diverged: Vec<(&str, &Option<String>, &str)> = Vec::new();
    let excluded = census(results.iter().filter_map(|result| match &result.g4 {
        G4Outcome::Excluded(reason) => Some(*reason),
        _ => None,
    }));
    for result in results {
        match &result.g4 {
            G4Outcome::Excluded(_) => {}
            G4Outcome::Equal => equal += 1,
            G4Outcome::Diverged {
                recovered_ron,
                reason,
            } => diverged.push((&result.line.label, recovered_ron, reason)),
        }
    }
    for (label, recovered_ron, reason) in &diverged {
        println!("G4 DIVERGED {label}");
        println!(
            "  recovered RON: {}",
            recovered_ron
                .as_deref()
                .unwrap_or("(incomplete — see reason)")
        );
        println!("  {reason}");
    }
    let covered = equal + diverged.len();
    let excluded_total: usize = excluded.values().sum();
    let pass = diverged.is_empty();
    println!(
        "G4 ground truth: {}: {covered} covered / {equal} equal / {} diverged",
        if pass { "PASS" } else { "FAIL" },
        diverged.len(),
    );
    if !pass {
        println!(
            "  every divergence above is analyzed, named, and disposed of in {G5_FINDINGS_FILE}"
        );
    }
    println!(
        "  exclusion census: {excluded_total} excluded of {} line(s) swept — {}",
        results.len(),
        format_census(&excluded),
    );
    // See `report_g3`'s own note: an `ensure!`, not a `debug_assert!`.
    anyhow::ensure!(
        covered + excluded_total == results.len(),
        "every swept line must land in exactly one G4 bucket, but {covered} covered + \
         {excluded_total} excluded != {} swept",
        results.len(),
    );
    Ok(GateReport {
        pass,
        counted: covered,
    })
}

/// A fidelity run's verdict and the two totals [`COVERAGE_FLOOR`] pins.
struct FidelityReport {
    pass: bool,
    clean: usize,
    waived: usize,
}

/// Re-runs `cargo xtask fidelity`'s own gate over the four covered plugins
/// and reports whether it still holds at the round's established figure —
/// see the module doc: G3 is defined to include this, not just the
/// per-macro text comparison, so a caller never has to remember to run both.
///
/// The `pass` this returns is still just `failing == 0`; the *figure* half
/// of the plan's G3 ("still reports 73 clean / 7 waived / 0 failing") is
/// enforced by [`check_coverage`] against [`COVERAGE_FLOOR`], which is where
/// every other population number is pinned too. Without it, a regression
/// that converted a clean card into a *waived* one passed silently — and a
/// waiver is exactly what this design exists to retire.
fn report_fidelity(workspace_root: &Path, oracle_path: &Path) -> anyhow::Result<FidelityReport> {
    const COVERED: [(&str, bool); 4] = [
        ("builtin", false),
        ("canon", true),
        ("testing", false),
        ("demo", false),
    ];
    let oracle = Oracle::load(oracle_path)?;
    let mut total_clean = 0usize;
    let mut total_waived = 0usize;
    let mut total_failing = 0usize;
    for (name, strict) in COVERED {
        let dir = workspace_root.join("plugins").join(name);
        let cards = fidelity::check_plugin(&dir, &oracle)?;
        let mut clean = 0usize;
        let mut waived = 0usize;
        let mut no_oracle = 0usize;
        let mut failing = 0usize;
        for card in &cards {
            match (&card.outcome, &card.waiver) {
                (FidelityOutcome::Clean, None) => clean += 1,
                (FidelityOutcome::Diffs(_), Some(_)) => waived += 1,
                (FidelityOutcome::NoOracle, _) if !strict => no_oracle += 1,
                _ => failing += 1,
            }
        }
        total_clean += clean;
        total_waived += waived;
        total_failing += failing;
        println!(
            "  fidelity[{name}]: {} checked, {clean} clean, {waived} waived, {no_oracle} \
             without an oracle entry, {failing} failing",
            cards.len(),
        );
    }
    let pass = total_failing == 0;
    println!(
        "cargo xtask fidelity: {}: {total_clean} clean, {total_waived} waived, {total_failing} \
         failing (total across builtin/canon/testing/demo)",
        if pass { "PASS" } else { "FAIL" },
    );
    Ok(FidelityReport {
        pass,
        clean: total_clean,
        waived: total_waived,
    })
}

#[cfg(test)]
mod tests {
    use super::*;

    // -----------------------------------------------------------------
    // The coverage floor — the round's own Critical, at the verdict layer.
    // -----------------------------------------------------------------

    /// The reproduction from the review, run as a test: point the sweep at a
    /// directory that does not exist and every gate reports PASS over an
    /// empty population. Before the floor, this exited 0 — "G3 PASS (0
    /// checked), G4 PASS (0 covered)" — which is the same shape any
    /// regression that shrinks coverage takes, including one that "closes"
    /// G4's 8 divergences by making them fail to parse.
    ///
    /// It runs the whole command, not just [`check_coverage`], because the
    /// defect was never in the arithmetic: the census closed correctly at
    /// every printed level and the verdicts still read `is_empty()`. Only an
    /// end-to-end run can show that a shrunken population now fails.
    #[test]
    fn an_absent_corpus_breaches_the_floor_instead_of_printing_pass() {
        let error = run(PilotArgs {
            plugin_dir: None,
            canon_dir: Some(PathBuf::from("/tmp/definitely-not-a-corpus-dir")),
            oracle: None,
        })
        .expect_err("an empty corpus must not pass");
        let message = format!("{error:#}");
        assert!(message.contains("coverage floor breached"), "{message}");
        // Every population figure, not just the first one that dropped.
        for expected in [
            "canon abilities seen: 0",
            "lines swept: 0",
            "G3 usages checked: 0",
            "G4 lines covered: 0",
        ] {
            assert!(message.contains(expected), "missing {expected}: {message}");
        }
    }

    #[test]
    fn the_pinned_census_itself_satisfies_the_floor() {
        check_coverage(&COVERAGE_FLOOR).expect("the floor must not breach itself");
    }

    /// A render whose text parses back as a different constituency than the
    /// frame it came from fails the gate, and is counted as *checked* rather
    /// than excluded. Both halves matter: filed as an exclusion it would read
    /// as missing coverage, and left out of the checked count it would breach
    /// the population partition instead — a different, misleading error.
    #[test]
    fn a_line_that_did_not_reassemble_its_frame_fails_g3() {
        let line = Line {
            label: "fixture: Face (Spell)".to_string(),
            name: String::new(),
            is_legendary: false,
            kind: FragmentKind::Sentence,
            text: "fixture line".to_string(),
            ron_type: "OneShotEffect",
            authored_view: View::Absent,
        };
        let result = |g3| LineResult {
            line: &line,
            g3,
            g4: G4Outcome::Excluded(ExclusionReason::NotAnInvocation),
        };

        let clean = report_g3(&[result(G3Outcome::Equal)]).expect("an equal line passes G3");
        assert!(clean.pass);
        assert_eq!(clean.counted, 1);

        let error = report_g3(&[result(G3Outcome::ReassembledDifferently {
            detail: "the substituted text parses into a different tree".to_string(),
        })])
        .expect_err("a line that did not reassemble its frame must fail the gate");
        let message = format!("{error:#}");
        assert!(message.contains("did not reassemble"), "{message}");
        assert!(message.contains("fixture: Face (Spell)"), "{message}");
    }

    /// Each figure is checked independently, so a single shrinking
    /// population cannot hide behind the others holding.
    #[test]
    fn any_single_decrease_breaches_the_floor() {
        for (label, mutate) in [
            (
                "canon abilities seen",
                (|c: &mut Coverage| c.seen -= 1) as fn(&mut Coverage),
            ),
            ("lines swept", |c: &mut Coverage| c.swept -= 1),
            ("G3 usages checked", |c: &mut Coverage| c.g3_checked -= 1),
            ("G4 lines covered", |c: &mut Coverage| c.g4_covered -= 1),
            ("fidelity cards clean", |c: &mut Coverage| {
                c.fidelity_clean -= 1;
            }),
        ] {
            let mut observed = COVERAGE_FLOOR;
            mutate(&mut observed);
            let message = format!(
                "{:#}",
                check_coverage(&observed).expect_err("a decrease must breach the floor")
            );
            assert!(message.contains(label), "expected {label} in: {message}");
        }
    }

    /// `fidelity_waived` is the inverted entry: a card moving from clean to
    /// waived leaves `failing == 0` untouched, so `report_fidelity`'s own
    /// verdict cannot see it — and a waiver is exactly what the frames
    /// design exists to retire.
    #[test]
    fn a_newly_waived_card_breaches_the_ceiling() {
        let observed = Coverage {
            fidelity_clean: COVERAGE_FLOOR.fidelity_clean - 1,
            fidelity_waived: COVERAGE_FLOOR.fidelity_waived + 1,
            ..COVERAGE_FLOOR
        };
        let message = format!(
            "{:#}",
            check_coverage(&observed).expect_err("a new waiver must breach the ceiling")
        );
        assert!(message.contains("fidelity cards waived"), "{message}");
        assert!(message.contains("fidelity cards clean"), "{message}");
    }

    /// Growth is not a breach — a later round that legitimately widens the
    /// sweep must not have to touch the constant to stay green.
    #[test]
    fn growth_above_the_floor_is_not_a_breach() {
        check_coverage(&Coverage {
            seen: COVERAGE_FLOOR.seen + 20,
            swept: COVERAGE_FLOOR.swept + 20,
            g3_checked: COVERAGE_FLOOR.g3_checked + 5,
            g4_covered: COVERAGE_FLOOR.g4_covered + 5,
            fidelity_clean: COVERAGE_FLOOR.fidelity_clean + 7,
            fidelity_waived: 0,
        })
        .expect("more coverage and fewer waivers must pass");
    }

    // -----------------------------------------------------------------
    // G4 compares values, not spellings.
    // -----------------------------------------------------------------

    /// A line whose authored value is `authored`, read at `ron_type`.
    /// Everything else is diagnostic-only: [`evaluate_g4`] reads exactly
    /// `ron_type` and `authored_view`.
    fn line_authored(ron_type: &'static str, authored: &str) -> Line {
        let authored_view = guard::normalize_source(guard::core_reader(), ron_type, authored)
            .unwrap_or_else(|error| panic!("normalizing {authored} as {ron_type}: {error:#}"));
        Line {
            label: format!("<fixture>: {authored}"),
            name: String::new(),
            is_legendary: false,
            kind: FragmentKind::Sentence,
            text: String::new(),
            ron_type,
            authored_view,
        }
    }

    /// The gate's own comparison, on the family it was changed for: a
    /// recovery through a `body:` entry emits `By(You, GainLife(3))`, the
    /// card says `GainLife(3)`, and they are the same `OneShotEffect`.
    ///
    /// The control is the point — the two **spellings** genuinely differ, so
    /// a comparison on text would have called this a divergence, and did.
    #[test]
    fn g4_compares_expanded_values_not_spellings() {
        let recovered = Recovered::Invocation {
            entry: "GainLife".to_string(),
            args: vec![
                Recovered::Invocation {
                    entry: "You".to_string(),
                    args: Vec::new(),
                    body: None,
                    ambiguities: Vec::new(),
                },
                Recovered::Literal("3".to_string()),
            ],
            body: Some("By(Param(0), GainLife(Param(1)))".to_string()),
            ambiguities: Vec::new(),
        };
        let emitted = recovered
            .to_ron(guard::core_reader())
            .expect("a fully recovered tree emits");
        assert_eq!(emitted, "By(You, GainLife(3))");
        assert_ne!(emitted, "GainLife(3)", "the spellings really do differ");

        let line = line_authored("OneShotEffect", "GainLife(3)");
        assert!(
            matches!(
                evaluate_g4(&recovered, &line, guard::core_reader()),
                G4Outcome::Equal
            ),
            "expected Equal, got {:?}",
            g4_label(&evaluate_g4(&recovered, &line, guard::core_reader())),
        );

        // And it is a comparison, not an acceptance: a recovery denoting a
        // different value still diverges.
        let other = line_authored("OneShotEffect", "GainLife(4)");
        assert_eq!(
            g4_label(&evaluate_g4(&recovered, &other, guard::core_reader())),
            "diverged: expands to a different normal form than the authored RON",
        );
    }

    /// An incomplete recovery cannot be compared at all: a residual denotes
    /// no value, so it is a divergence with nothing to print — the one arm a
    /// value comparison does not remove.
    #[test]
    fn an_incomplete_recovery_diverges_with_no_value_to_show() {
        let recovered = Recovered::Invocation {
            entry: "GainLife".to_string(),
            args: vec![
                Recovered::Residual(View::Unit {
                    name: "Pronoun",
                    variant: Some("You"),
                }),
                Recovered::Literal("3".to_string()),
            ],
            body: Some("By(Param(0), GainLife(Param(1)))".to_string()),
            ambiguities: Vec::new(),
        };
        let line = line_authored("OneShotEffect", "GainLife(3)");
        let G4Outcome::Diverged {
            recovered_ron,
            reason,
        } = evaluate_g4(&recovered, &line, guard::core_reader())
        else {
            panic!("an incomplete recovery must diverge");
        };
        assert_eq!(recovered_ron, None, "there is no value to show");
        assert!(reason.contains("GainLife arg 0"), "{reason}");
    }

    fn g4_label(outcome: &G4Outcome) -> String {
        match outcome {
            G4Outcome::Equal => "equal".to_string(),
            G4Outcome::Excluded(reason) => format!("excluded: {}", reason.label()),
            G4Outcome::Diverged { reason, .. } => format!("diverged: {reason}"),
        }
    }
}
