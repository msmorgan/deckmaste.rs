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
//! ([`collect_lines`]/[`evaluate_line`]): every `Ability::Keyword` and
//! `Ability::Spell` (with no `ability_word`) in every non-todo canon card
//! face, isolated into its own single-ability [`CardView`] and legacy-
//! rendered on its own. `Ability::Triggered`/`Static`/`Activated` lines are
//! not swept at all — none of the pilot's macros model a whole trigger,
//! static, or activated-cost shape, so a top-level match against one would
//! always be `Residual` anyway, and isolating a *sub*-fragment (a single
//! cost component out of an activated ability's list, in particular) has no
//! public single-component legacy renderer to isolate against. Concretely,
//! this means `SacrificeThis` — usable only inside an activated ability's
//! cost list — sees zero coverage from this sweep; that is a disclosed gap,
//! not a hidden one.
//!
//! A line is "covered" for either gate only when [`unify`] recovers a
//! top-level [`Recovered::Invocation`] AND (for G4 specifically) every
//! filler in that tree is itself an `Invocation` or `Literal` — never a
//! `Residual` — because a residual carries only a captured `View` with no
//! RON spelling ([`recovered_to_ron`] refuses it outright, honestly, rather
//! than guessing). G3's own coverage additionally requires
//! [`render_invocation_with`] to succeed, which fails for the identical
//! reason (see `deckmaste_frames::render`'s own module doc). A gate's
//! `covered`/`checked` denominator is therefore smaller than "every canon
//! line that mentions a pilot macro" — this is the honest boundary the round
//! ships with today, not a threshold this command loosens to manufacture a
//! green run.

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
    let lines = collect_lines(&canon_dir, &canon_plugin.macros)?;
    let results: Vec<LineResult> = lines
        .iter()
        .map(|line| evaluate_line(line, &lexicon, &catalogs, &plugin.macros))
        .collect();
    let sweep_elapsed = sweep_started.elapsed();
    println!(
        "swept {} canon line(s) in {:.2}s",
        lines.len(),
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

fn collect_lines(canon_dir: &Path, macros: &MacroSet) -> anyhow::Result<Vec<Line>> {
    let mut lines = Vec::new();
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
                if let Some(line) = testable_line(&path, face, is_legendary, ability) {
                    lines.push(line);
                }
            }
        }
    }
    Ok(lines)
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
) -> Option<Line> {
    let peeled = peel_expanded(ability);
    let (kind, ron_type, authored_view, tag, rendering_ability) = match peeled {
        Ability::Keyword(k) => (
            FragmentKind::KeywordLine,
            "KeywordAbility",
            guard::normalized(k.clone()),
            "Keyword",
            ability.clone(),
        ),
        Ability::Spell(s) if s.ability_word.is_none() => (
            FragmentKind::Sentence,
            "OneShotEffect",
            guard::normalized(s.effect.clone()),
            "Spell",
            ability.clone(),
        ),
        Ability::Triggered(t) if t.ability_word.is_none() => (
            FragmentKind::Sentence,
            "OneShotEffect",
            guard::normalized(t.effect.clone()),
            "Triggered-effect",
            Ability::Spell(std::sync::Arc::new(deckmaste_core::SpellAbility {
                ability_word: None,
                effect: t.effect.clone(),
            })),
        ),
        _ => return None,
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
        return None;
    }
    let text = rules.remove(0);
    Some(Line {
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

enum G3Outcome {
    /// Not a pilot-*macro* usage, or one this sweep cannot fully render —
    /// excluded from G3's population entirely (see the module doc).
    NotCovered,
    Equal,
    Mismatch {
        rendered: String,
    },
}

enum G4Outcome {
    /// Top-level `Residual`, or a filler this sweep cannot spell as RON —
    /// excluded from G4's population entirely (see the module doc).
    NotCovered,
    Equal,
    Diverged {
        recovered_ron: String,
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
        // the line is simply out of both populations.
        return LineResult {
            line,
            g3: G3Outcome::NotCovered,
            g4: G4Outcome::NotCovered,
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

fn evaluate_g4(recovered: &Recovered, line: &Line, macros: &MacroSet) -> G4Outcome {
    let Recovered::Invocation { .. } = recovered else {
        return G4Outcome::NotCovered;
    };
    let Ok(ron_text) = recovered_to_ron(recovered) else {
        return G4Outcome::NotCovered;
    };
    match guard::normalize_source(macros, line.ron_type, &ron_text) {
        Ok(recovered_view) if recovered_view == line.authored_view => G4Outcome::Equal,
        Ok(_) => G4Outcome::Diverged {
            recovered_ron: ron_text,
            reason: "expands to a different normal form than the authored RON".to_string(),
        },
        Err(error) => G4Outcome::Diverged {
            recovered_ron: ron_text,
            reason: format!("failed to expand as `{}`: {error:#}", line.ron_type),
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
        return G3Outcome::NotCovered;
    };
    let is_macro = lexicon
        .entries()
        .iter()
        .any(|candidate| candidate.name == *entry && candidate.origin == Origin::Macro);
    if !is_macro {
        return G3Outcome::NotCovered;
    }
    let Ok(rendered) = render_invocation_with(
        recovered,
        lexicon,
        FramePosition::Main,
        catalogs,
        &line.name,
        line.is_legendary,
    ) else {
        return G3Outcome::NotCovered;
    };
    let normalized_rendered = fidelity::normalize(&rendered, &line.name);
    let normalized_legacy = fidelity::normalize(&line.text, &line.name);
    if normalized_rendered == normalized_legacy {
        G3Outcome::Equal
    } else {
        G3Outcome::Mismatch { rendered }
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

fn report_g3(results: &[LineResult]) -> bool {
    let mut checked = 0usize;
    let mut equal = 0usize;
    let mut mismatches: Vec<(&str, &str, &str)> = Vec::new();
    for result in results {
        match &result.g3 {
            G3Outcome::NotCovered => {}
            G3Outcome::Equal => {
                checked += 1;
                equal += 1;
            }
            G3Outcome::Mismatch { rendered } => {
                checked += 1;
                mismatches.push((&result.line.label, &result.line.text, rendered));
            }
        }
    }
    for (label, legacy, rendered) in &mismatches {
        println!("G3 MISMATCH {label}");
        println!("  legacy render:   {legacy}");
        println!("  render_invocation: {rendered}");
    }
    let pass = mismatches.is_empty();
    println!(
        "G3 shadow parity: {}: {checked} canon pilot-macro usage(s) checked, {equal} equal, {} mismatched",
        if pass { "PASS" } else { "FAIL" },
        mismatches.len(),
    );
    pass
}

fn report_g4(results: &[LineResult]) -> bool {
    let mut covered = 0usize;
    let mut equal = 0usize;
    let mut diverged: Vec<(&str, &str, &str)> = Vec::new();
    for result in results {
        match &result.g4 {
            G4Outcome::NotCovered => {}
            G4Outcome::Equal => {
                covered += 1;
                equal += 1;
            }
            G4Outcome::Diverged {
                recovered_ron,
                reason,
            } => {
                covered += 1;
                diverged.push((&result.line.label, recovered_ron, reason));
            }
        }
    }
    for (label, recovered_ron, reason) in &diverged {
        println!("G4 DIVERGED {label}");
        println!("  recovered RON: {recovered_ron}");
        println!("  {reason}");
    }
    let pass = diverged.is_empty();
    println!(
        "G4 ground truth: {}: {covered} covered / {equal} equal / {} diverged",
        if pass { "PASS" } else { "FAIL" },
        diverged.len(),
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
