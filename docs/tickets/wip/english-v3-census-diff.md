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

Measured on change `tvyrmynw` (tooling only; no grammar or lexicon change).

1. **Census delta**: none; no corpus input changed. Acceptance comparisons,
   category counts identical to the reviewers' Python and face-id sets equal:
   - `negative-common-noun-premodifier/final-faces.json` ->
     `past-participial-premodifier/after.json` (`ootnvmslqozx`): 32,828
     compared, 0 lost, 210 gained, 0 decreases, 2 increases (Dream Harvest
     1584 -> 3168, Tasha's Hideous Laughter 89 -> 178), 0 digest-only;
     covered 23,658 -> 23,868, Readings 1,672,796 -> 1,840,929. The 210
     gained ids equal `review-pastpart/gained.json`. Manifest with
     `--random 50 --seed 20261008`: 262 identities.
   - `derived-punctuation-forms/before.json` (`oxvkxppy`) ->
     `derived-punctuation-forms/after.json` (`xtxrnswzolym`): 32,828
     compared, 0 count changes, 1,981 digest-only; covered 23,460, Readings
     1,605,559 both sides. Digest-only ids equal that ticket's
     `comparison.json` change list.
2. **Witnesses**: the two comparisons above.
3. **Spot checks**: not applicable (no newly covered faces).
4. **Deviations and STOPs**: (a) the per-face input is a field projection
   (`CensusFace`: id, name, face name, analysed-source hash, checked
   Readings, Readings digest) rather than `report::FaceReport`, because
   `FaceReport` deserialisation requires every field and stitched reports
   such as `final-faces.json` carry no run metadata; the manifest reuses
   `raw_corpus::SelectionRequest`. (b) The 1,981 digest-only pair is the
   punctuation ticket's own `before.json` (the refreshed base, `oxvkxppy`),
   not `licence-hygiene/after.json` (that pair gives 107 gained, 1,952
   digest-only) or `licence-hygiene/refreshed-subset.json` (853 faces, 81
   digest-only). (c) Source-hash change compares `analyzed_source_sha256`
   and is reported beside, not instead of, the count category. (d) Seeded
   sampling uses an in-module SplitMix64 rather than a new dependency. No
   STOPs; no glossary gap.
5. **Tests**: `cargo test -p xtask --lib`: 381 passed, 1 ignored
   (pre-existing). `cargo test -p xtask --tests`: integration
   `card_names::nickname_catalog_preserves_the_complete_lexical_inventory`
   fails because the shared `data/gen/catalogs` nickname cache differs from
   the generated catalog; it reads only `data/` and the lexical sources,
   neither touched here, so it is environment state, not this landing.
   `cargo clippy -p xtask --tests -- -D warnings`: clean. Counts: 7 added
   (`english_v3::diff::tests`), 0 restored, 0 re-spelled, 0 ignored, 0
   removed.
6. **Timing**: no census run. One diff of two full 32,828-face reports
   (punctuation pair): 0.3 s wall, debug build, single thread.
