//! `cargo xtask macro pilot` — the round's two ground-truth gates over real
//! canon cards:
//!
//! - **G3 shadow parity**: for every canon usage of a pilot *macro* (an
//!   `Origin::Macro` lexicon entry — not the three raw constructor entries),
//!   does [`render_invocation_with`] reproduce, byte-for-byte after
//!   `fidelity`-style normalization, the same text the legacy per-ability
//!   renderer ([`deckmaste_cards::render`]) already prints for that ability?
//!   AND does `cargo xtask fidelity` still report 73 clean / 7 waived / 0
//!   failing — this command re-runs that exact check rather than asking a
//!   caller to remember to run both.
//! - **G4 ground truth**: for every canon ability line whose parse `unify`s to
//!   a non-`Residual` under the pilot lexicon, does the recovered structure —
//!   reconstructed as RON text and expanded — equal the card's own authored
//!   value, likewise expanded? Both directions go through
//!   [`deckmaste_frames::guard::normalized`]/`normalize_source`, the round's
//!   one canonicalizer, never a second implementation.
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
//! for every swept line that is not equal/mismatched/diverged, with a debug
//! assertion in each tying its own two numbers together
//! (`checked/covered + excluded == lines swept`). So the population story
//! runs end to end — canon ability lines seen → swept → gated — and a line
//! disappearing from the numbers the way a whole macro's canon coverage
//! once did (this command's own fix-round Critical finding: every
//! `Keyword(Flying)` line was swept, then excluded with no record at all —
//! see the G5 findings file for the root cause and fix) cannot happen again
//! unnoticed, at either layer.
//!
//! G4's population is exactly the brief's literal wording: top-level
//! `Recovered::Invocation`, any origin, full stop — **not** "and every
//! nested filler is RON-spellable too" (a fix-round correction: it used to
//! additionally require that, silently narrowing the denominator). A line
//! that recovers a non-`Residual` top level but leaves an unspellable
//! residual filler is *covered* and counted `Diverged` (with
//! `recovered_ron: None`, see [`G4Outcome::Diverged`]) — a recovery that is
//! provably incomplete is not asserted equal to the authored value, so it is
//! a real divergence, not an exclusion.
//!
//! G3's own population is the brief's other, narrower wording — "canon usage
//! of a pilot *macro*" — so a top-level `Origin::Constructor` match, or one
//! [`render_invocation_with`] itself cannot render (the identical residual
//! limitation, one level up — see `deckmaste_frames::render`'s own module
//! doc), is excluded from G3 specifically ([`ExclusionReason::NotMacroOrigin`]/
//! [`ExclusionReason::RenderFailed`]), recorded in its own census rather than
//! silently dropped.

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

    let g3_pass = report_g3(&results);
    let fidelity_pass = report_fidelity(&workspace_root, &oracle_path)?;
    let g4_pass = report_g4(&results);

    anyhow::ensure!(
        g3_pass && fidelity_pass && g4_pass,
        "macro pilot: {}",
        [
            (!g3_pass).then_some("G3 shadow parity failed"),
            (!fidelity_pass).then_some("fidelity regressed"),
            (!g4_pass).then_some("G4 ground truth failed"),
        ]
        .into_iter()
        .flatten()
        .collect::<Vec<_>>()
        .join("; ")
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
    let mut g4_equal = 0usize;
    let mut g4_diverged = 0usize;
    for result in &results {
        match result.g3 {
            G3Outcome::Equal => g3_equal += 1,
            G3Outcome::Mismatch { .. } => g3_mismatched += 1,
            G3Outcome::Excluded(_) => {}
        }
        match result.g4 {
            G4Outcome::Equal => g4_equal += 1,
            G4Outcome::Diverged { .. } => g4_diverged += 1,
            G4Outcome::Excluded(_) => {}
        }
    }
    Ok(GateStatus {
        g3_checked: g3_equal + g3_mismatched,
        g3_equal,
        g3_mismatched,
        g3_pass: g3_mismatched == 0,
        g4_covered: g4_equal + g4_diverged,
        g4_equal,
        g4_diverged,
        g4_pass: g4_diverged == 0,
    })
}

// ---------------------------------------------------------------------------
// Corpus loading
// ---------------------------------------------------------------------------

fn real_catalogs(workspace_root: &Path) -> anyhow::Result<Catalogs> {
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

fn faces(card: &Card) -> Vec<&CardFace> {
    match card {
        Card::Normal(face) => vec![face],
        Card::TwoFaced { front, back, .. } => vec![front, back],
    }
}

/// Peels a leading `Ability::Expanded` wrapper — mirrors
/// `deckmaste_cards::render`'s own private `peel_expanded`, which this
/// module cannot call (it is not exported), kept minimal since only the
/// two ability kinds below are ever isolated.
fn peel_expanded(ability: &Ability) -> &Ability {
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
fn ron_files_recursive(dir: &Path) -> anyhow::Result<Vec<PathBuf>> {
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
    Mismatch { rendered: String },
}

enum G4Outcome {
    Excluded(ExclusionReason),
    Equal,
    Diverged {
        /// `None` when the recovered tree itself could not be spelled as
        /// RON at all (a nested residual filler blocked
        /// [`recovered_to_ron`]) — this is *still* a divergence, not an
        /// exclusion (see [`evaluate_g4`]'s doc): the brief's population is
        /// "unify's to a non-Residual" at the top level, full stop, and a
        /// recovery that is provably incomplete cannot be asserted equal to
        /// the authored value.
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

/// G4's population is exactly the brief's own wording: a top-level
/// `Recovered::Invocation` (any origin — macro or constructor), full stop.
/// A nested filler that cannot be spelled as RON does **not** narrow that
/// population (a fix-round correction: it used to, silently) — it is
/// reported as `Diverged` with `recovered_ron: None`, because recovery is
/// then *provably incomplete*, which cannot be asserted equal to the fully
/// concrete authored value.
fn evaluate_g4(recovered: &Recovered, line: &Line, macros: &MacroSet) -> G4Outcome {
    let Recovered::Invocation { .. } = recovered else {
        return G4Outcome::Excluded(ExclusionReason::NotAnInvocation);
    };
    match recovered_to_ron(recovered) {
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
        Err(error) => G4Outcome::Diverged {
            recovered_ron: None,
            reason: format!(
                "recovered as a non-residual invocation, but a nested filler could not be \
                 spelled as RON ({error:#}) — recovery is provably incomplete, so it cannot be \
                 asserted equal to the authored value"
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
        Err(_) => G3Outcome::Excluded(ExclusionReason::RenderFailed),
    }
}

/// Spells a fully non-residual [`Recovered`] tree as RON source text: an
/// invocation as `entry` (bare, if nullary) or `entry(arg, arg, ...)`,
/// recursively; a literal verbatim (already a valid RON leaf spelling —
/// digits, or a guard's own authored constant). Refuses a
/// [`Recovered::Residual`] outright: it carries only a captured `View` with
/// no RON spelling to fall back on (see the module doc).
///
/// # Errors
/// If `recovered` is, or contains, a `Residual`.
fn recovered_to_ron(recovered: &Recovered) -> anyhow::Result<String> {
    match recovered {
        Recovered::Literal(text) => Ok(text.clone()),
        Recovered::Invocation { entry, args, .. } => {
            if args.is_empty() {
                Ok(entry.clone())
            } else {
                let parts = args
                    .iter()
                    .map(recovered_to_ron)
                    .collect::<anyhow::Result<Vec<_>>>()?;
                Ok(format!("{entry}({})", parts.join(", ")))
            }
        }
        Recovered::Residual(_) => anyhow::bail!("a residual filler has no RON spelling"),
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

fn report_g3(results: &[LineResult]) -> bool {
    let mut equal = 0usize;
    let mut mismatches: Vec<(&str, &str, &str)> = Vec::new();
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
        }
    }
    for (label, legacy, rendered) in &mismatches {
        println!("G3 MISMATCH {label}");
        println!("  legacy render:      {legacy}");
        println!("  render_invocation:  {rendered}");
    }
    let checked = equal + mismatches.len();
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
    debug_assert_eq!(
        checked + excluded_total,
        results.len(),
        "every swept line must land in exactly one G3 bucket"
    );
    pass
}

fn report_g4(results: &[LineResult]) -> bool {
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
                .unwrap_or("(unspellable — see reason)")
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
    debug_assert_eq!(
        covered + excluded_total,
        results.len(),
        "every swept line must land in exactly one G4 bucket"
    );
    pass
}

/// Re-runs `cargo xtask fidelity`'s own gate over the four covered plugins
/// and reports whether it still holds at the round's established figure —
/// see the module doc: G3 is defined to include this, not just the
/// per-macro text comparison, so a caller never has to remember to run both.
fn report_fidelity(workspace_root: &Path, oracle_path: &Path) -> anyhow::Result<bool> {
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
    Ok(pass)
}
