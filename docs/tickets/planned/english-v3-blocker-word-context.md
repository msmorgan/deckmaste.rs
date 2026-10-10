---
needs: [english-v3-blocker-report]
---
# [design] Make blocker positions useful as word context

The full blocker report at `sqnyzpnl` groups 274 faces under `at:\s t`,
245 of them sole-blocker faces. Its examples stop at different words:
Guul Draz Vampire at *this*, Heartwarming Redemption at *the*, and both
Doubling Season lines at *twice*. The runtime's tokens are Unicode scalars,
so the prescribed two-token key preserves the chart boundary but cannot
separate these words. Evidence: `target/english-v3/blockers.json` and
`blockers.txt` in the coordinator workspace; 32,828 supported faces.

Design and implement whole-word context for `english-v3 blockers`. Before
implementation, settle whether words should supplement examples or replace
the displayed `at:` grouping, and how boundaries inside words, whitespace,
punctuation and end of input behave. Keep the exact Unicode-scalar position
available and record the key policy so reports remain comparable. Reuse
`AnalyzedText::words`, whose orthographic inventory is independent of parsing.

Standard constraints apply. No grammar, lexicon, census schema, dependency,
admission gate or new xtask subcommand. Preserve the report's B1–B4 rulings.
Tests assert context and ranking relations on synthetic lines, including a
Unicode example; run `cargo test -p xtask english_v3` and xtask clippy. Use a
bounded current census for report verification; no full census is required.
