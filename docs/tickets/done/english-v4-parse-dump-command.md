---
needs: [english-v4-corpus-stratification-and-judgment-harness]
---
# `cargo xtask english-v4 parse`: judgment-syntax parse and constituent dump

Add a `parse` subcommand to `cargo xtask english-v4`
(`crates/xtask/src/english_v4.rs`) for inspecting `deckmaste_english_v4`
derivations without writing one-off scratch binaries:

1. **Input syntax:** Accept one or more inputs in judgment-entry syntax (or bare
   Oracle text, defaulting to `+ <text>` when no leading `+`/`*` is given) so
   that category roots (`+NounPhrase ...`, `*PrepositionPhrase ...`), card scopes
   (`<card=Name>`), and optional `=> ...` assertions all work out of the box,
   plus an optional `--limit <k>` flag (defaulting to e.g. `8`) for how many
   Readings to print.
2. **Output:** For each input, print:
   - its profiled `Stratum` and `Syntactic Families`,
   - vocabulary gaps (if any), total derivation count in the Packed Forest, and
     judgment `Outcome` (`holds` or failure reason) when assertions or a `+`/`*`
     mark are present,
   - up to `k` cheapest admitted `Reading`s with their realized text, root/node
     production ids, and constituent table (`Category`, surface span, `function`,
     `head` lemma/lexeme, and `Gap`).
3. **Documentation:** Update `crates/deckmaste_english_v4/README.md` to document
   `cargo xtask english-v4 parse`.

## Gate

- `cargo test -p xtask` and `cargo clippy -p xtask --tests -- -D warnings` pass.
- `cargo xtask english-v4 parse "+ <card=Seedborn Muse> Untap all permanents you control."`
  prints the profiled stratum, derivation count, judgment outcome, and
  constituent structure.

Standard constraints apply.

Census delta (`vutvxqqo`): not applicable; this inspection command changes neither grammar nor census behavior, so no before/after census was run.
Judgment set (`vutvxqqo`): `cargo test -p deckmaste_english_v4 --test judgments` — 6 passed.
