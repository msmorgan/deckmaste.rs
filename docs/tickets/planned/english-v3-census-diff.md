---
needs: []
---
# Census diff subcommand: `cargo xtask english-v3 diff`

## Why

The census diff command was in the work lost in September 2026. Since then
every Codex ticket and every Opus review has re-written a Python face-by-face
comparison of two census JSON files, 10 to 15 minutes per run. The per-face
digest diff is also what enforces the no-silent-loss law under the 2026-10-08
landing record amendment, so it belongs in the tree.

## Goal

One command compares two `cargo xtask english-v3` census outputs face by face
and prints the categories reviews use, optionally writing the same data as
JSON and a seeded identity manifest for a bounded re-run.

## CLI contract

`cargo xtask english-v3 diff BEFORE.json AFTER.json [--json OUT] [--manifest OUT --random N --seed S]`

- Inputs are census outputs (current schema; per-face `checked_readings`,
  `readings_sha256`, analysed-source hash, identity and name). Reuse the
  report types in `crates/xtask/src/english_v3/report.rs`. A bounded census
  (identity-manifest subset) diffs against a full one by comparing only faces
  present in both, reporting the face counts present in each.
- Stdout, plain text: faces compared / only in before / only in after;
  source-hash changes; lost (covered to 0); gained (0 to covered); decreases
  (count fell, still covered); increases; digest-only changes (same count,
  different `readings_sha256`); unchanged; covered faces and total Readings
  before/after. Then per-category face lists with before/after counts (lost,
  decreases, increases, digest-only, gained), capped at 50 lines each with
  "... and N more".
- `--json OUT` writes the same data machine-readably, lists uncapped.
- `--manifest OUT --random N --seed S` writes an identity manifest (the format
  `--identity-manifest` reads) holding every gained, lost, decreased and
  increased face plus N seeded random faces covered in BEFORE and unchanged.
- Exit code 0 always: it is a report, not a gate.
- Documented beside the other english-v3 subcommands.

## Method

Standard constraints apply. Unit tests in the xtask `english_v3` module cover
each category with small synthetic census values built in code (no fixture
files). Run `cargo test -p xtask` and `cargo clippy -p xtask --tests -- -D
warnings`. Acceptance: the category counts match the reviewers' Python
comparisons on the past-participial-premodifier pair (210 gained, 2
increases, 0 lost) and the derived-punctuation-forms pair (1,981 digest-only,
0 count changes).

## Landing record

The six items of `CLAUDE.md` (Feature workflow). No grammar change, so the
census delta and witness items are the two acceptance comparisons.
