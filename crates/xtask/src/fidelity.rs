//! The `fidelity` command: the render-back fidelity gate (strong form) —
//! render every finished card back to templated English and diff it against
//! the oracle snapshot. xtask owns the CLI; the oracle loading, diffing,
//! waiver policy, and normalization live in `deckmaste_cards::fidelity`.

use std::path::Path;
use std::path::PathBuf;

use clap::Args;
use deckmaste_cards::fidelity;
use deckmaste_cards::fidelity::Oracle;
use deckmaste_cards::fidelity::Outcome;

/// The plugins the gate covers by default, with canon's stricter oracle
/// policy (every canon card must exist in the oracle — canon encodes real
/// cards; testing/demo fixtures are synthetic and skip when unmatched).
const COVERED: [(&str, bool); 4] = [
    ("builtin", false),
    ("canon", true),
    ("testing", false),
    ("demo", false),
];

#[derive(Debug, Args)]
pub struct FidelityArgs {
    /// Check one plugin directory instead of the default four
    /// (`plugins/{builtin,canon,testing,demo}`; canon additionally requires
    /// an oracle entry per card).
    plugin_dir: Option<PathBuf>,
    /// List the waiver inventory (every `// waiver:` annotation with its
    /// reason and current diff count) instead of gating.
    #[arg(long)]
    waivers: bool,
    /// The oracle snapshot (one JSON row per printing face). Defaults to
    /// this workspace's `data/derived/cards.jsonl`.
    #[arg(long)]
    oracle: Option<PathBuf>,
}

/// # Errors
/// If the oracle snapshot or a plugin fails to load, or — the gate itself —
/// any card has an unwaivered diff, a stale waiver, or (canon) no oracle
/// entry.
pub fn run(args: FidelityArgs) -> anyhow::Result<()> {
    let workspace_root = Path::new(env!("CARGO_MANIFEST_DIR")).join("../..");
    let oracle_path = args
        .oracle
        .unwrap_or_else(|| workspace_root.join(fidelity::ORACLE_SNAPSHOT));
    let oracle = Oracle::load(&oracle_path)?;

    let plugins: Vec<(PathBuf, bool)> = match args.plugin_dir {
        Some(dir) => {
            let strict = dir.file_name().and_then(|f| f.to_str()) == Some("canon");
            vec![(dir, strict)]
        }
        None => COVERED
            .iter()
            .map(|(name, strict)| (workspace_root.join("plugins").join(name), *strict))
            .collect(),
    };

    let mut failures = 0usize;
    for (dir, strict) in &plugins {
        let cards = fidelity::check_plugin(dir, &oracle)?;
        if args.waivers {
            for card in cards.iter().filter(|c| c.waiver.is_some()) {
                let diffs = match &card.outcome {
                    Outcome::Diffs(d) => d.len(),
                    _ => 0,
                };
                println!(
                    "{}: {} — {} diff(s){}",
                    card.path.display(),
                    card.waiver.as_deref().unwrap_or_default(),
                    diffs,
                    if diffs == 0 { " [STALE]" } else { "" },
                );
            }
            continue;
        }
        let mut clean = 0usize;
        let mut waived = 0usize;
        let mut no_oracle = 0usize;
        let mut failing = 0usize;
        for card in &cards {
            match (&card.outcome, &card.waiver) {
                (Outcome::Clean, None) => clean += 1,
                (Outcome::Diffs(_), Some(_)) => waived += 1,
                (Outcome::NoOracle, _) if !strict => no_oracle += 1,
                _ => failing += 1,
            }
            print!("{}", fidelity::report_block(card, *strict));
        }
        failures += failing;
        println!(
            "{}: {} face(s) checked, {clean} clean, {waived} waived, {no_oracle} without an \
             oracle entry, {failing} failing",
            dir.display(),
            cards.len(),
        );
    }
    if failures > 0 {
        anyhow::bail!("fidelity gate: {failures} failing card face(s)");
    }
    Ok(())
}
