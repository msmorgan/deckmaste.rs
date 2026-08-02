//! The render-back fidelity gate as a standing test: every hand-authored
//! plugin's cards render back to their oracle text with zero unwaivered
//! diffs ([[cards-fidelity-target-sunset]]). The oracle snapshot lives under
//! the gitignored `data/` (never committed — see the no-committed-corpus
//! policy), so on a bare checkout (CI) this SKIPS loudly rather than fail —
//! the same pattern as the keyword-classification drift-guard. Run
//! `cargo xtask fidelity` for the full per-card report.

use std::path::Path;
use std::path::PathBuf;

use deckmaste_plugin::fidelity;
use deckmaste_plugin::fidelity::Oracle;

fn workspace_root() -> PathBuf {
    Path::new(env!("CARGO_MANIFEST_DIR")).join("../..")
}

/// `(plugin, strict_oracle)` — canon must have an oracle entry per card
/// (it encodes real cards); the other plugins' synthetic fixtures skip.
const COVERED: [(&str, bool); 4] = [
    ("builtin", false),
    ("canon", true),
    ("testing", false),
    ("demo", false),
];

#[test]
fn hand_authored_plugins_render_back_to_oracle() {
    let oracle_path = workspace_root().join(fidelity::ORACLE_SNAPSHOT);
    if !oracle_path.exists() {
        eprintln!(
            "SKIP: oracle snapshot {} not present (bare checkout) — the fidelity gate needs \
             the data/ fixtures",
            oracle_path.display()
        );
        return;
    }
    let oracle = Oracle::load(&oracle_path).expect("oracle snapshot loads");
    let mut failures = Vec::new();
    for (plugin, strict) in COVERED {
        let dir = workspace_root().join("plugins").join(plugin);
        let cards = fidelity::check_plugin(&dir, &oracle).expect("plugin loads");
        for card in &cards {
            if card.failure(strict).is_some() {
                failures.push(fidelity::report_block(card, strict));
            }
        }
    }
    assert!(
        failures.is_empty(),
        "fidelity gate: {} failing card face(s) — run `cargo xtask fidelity` for the full \
         report\n{}",
        failures.len(),
        failures.join("")
    );
}
