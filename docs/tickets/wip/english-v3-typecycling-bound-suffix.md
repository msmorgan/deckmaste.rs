---
needs: [english-v3-landwalk-stem-class-derivation]
---
# Read the typecycling keyword: Swampcycling {2}

## Why

No typecycling keyword reads. On census `qsupksky` (default `@` `mqrlzszk` is empty; 32,828 supported faces, 25,474 covered, 7,354 unread; wave-5 recon of 2026-10-08), the recon bucket "typecycling
(*X*cycling / *basic landcycling*)" touches **95** unread faces and is the
sole cause on **91**. Corpus tokens: *basic landcycling* 36, *plainscycling*
13, *mountaincycling* 13, *swampcycling* 11, *forestcycling* 11,
*islandcycling* 11, *wizardcycling* 2, *halflingcycling* 1, *slivercycling* 1,
*landcycling* 1. Counts are unread faces *touched* (at least one localised failing unit matches) / *sole* (every failing unit matches and no other recon STRONG bucket does). They are surface counts, not gain forecasts.

Probes (`--category Document --readings 1`): "Swampcycling {2}" 0; "Cycling
{2}" 1.

## Goal

Typecycling reads as a keyword line with a bound *-cycling* suffix on a
declared stem (land subtypes, the attested creature subtypes, *land*) and on
*basic land*, with byte-exact realisation, in the same way as landwalk.

## Analysis

No CGEL citation applies (Oracle keyword formation).

Existing route: landwalk's bound suffix
(`plugins_v2/builtin/macros/keyword_abilities/landwalk.ron`,
`FixedKeyword(surface: "landwalk", bound_suffix: (surface: "walk"))`), the
`BoundKeywordStem` frame (`crates/deckmaste_english_v3/src/declarations.rs:489`)
and `BoundQualityKeyword` (`declarations.rs:1974`). `cycling.ron` declares
only `FixedKeyword(surface: "cycling")`; there is no typecycling keyword
declaration. The stem class is being derived from the declarations by
`english-v3-landwalk-stem-class-derivation` (wip), hence `needs`; typecycling
also needs creature-subtype stems (*Wizard*, *Sliver*, *Halfling*).

Orchestrator ruling 2026-10-08 (R8a): this ticket MAY add a typecycling
keyword declaration under `plugins_v2/builtin/macros/keyword_abilities/`
with `grammar:` fields only and no engine body. Model it on landwalk's
bound-suffix shape: a Quality stem plus the bound suffix *cycling*
(*Swampcycling {2}*), followed by the cost parameter as in `cycling.ron`.
Its stem class is declared on the keyword and derived from the declarations
the same way `english-v3-landwalk-stem-class-derivation` derives landwalk's
(a declared class field, members derived by the loader, never a word list);
the class here spans the land subtypes, the creature subtypes and the *land*
Type. Members are whatever the declared class derives, as in the landwalk
ticket; attestation is checked for the class, not per member. The engine
body is routed to `semantics-v2-keyword-ability-census`, which owns the
bodyless keyword-ability declarations (see Out of scope).

Orchestrator ruling 2026-10-08 (R8b): *basic landcycling* is the *basic*
supertype premodifying the generic *landcycling* keyword (the *land* Type
stem plus *cycling*); it is not a two-word stem and not a separate class
member. Gleam of Resistance and the other *basic landcycling* faces count
if that reads; if it does not read through the existing supertype
premodification, record the count and leave it for a follow-up rather than
adding a Construction for it.

The cost after a bound-stem keyword has no route today:
`BoundQualityKeyword` (`declarations.rs:1974`) takes no parameter, and the
cost-keyword forms (`declarations.rs:1891`) take a `lexical(Keyword)` head.
Keyword shapes are one Construction per parameter class (`CostKeyword`,
`QualityCostKeyword`, `AmountCostKeyword` …), so try first to refine
`BoundQualityKeyword` or `CostKeyword` to admit the combination. If neither
refinement works without two labels for one constituency, one new
bound-quality-plus-cost keyword Construction is permitted (orchestrator
ruling 2026-10-08, in the spirit of R8a); record it under Deviations.
Pre-screen disagreement (2026-10-08): the sizing pre-screen predicted a new
Construction (size M) where this draft assumed a refinement; resolved as the
refine-first, one-Construction-fallback rule above.

## Witnesses

Each is the face's only failing unit at `qsupksky` (0 Readings), each
control (*Cycling* with the same cost) 1; all `--category Document`:

- Canyon Crawler: "Swampcycling {2}"
- Shepherding Spirits: "Plainscycling {2}"
- Step Through: "Wizardcycling {2}" (vocabulary gap *Wizardcycling*)
- Generous Ent: "Forestcycling {1}"
- Gleam of Resistance: "Basic landcycling {1}{W}" 

## Method

Standard constraints apply (CLAUDE.md landing record, assurance, scope).
Deltas:

1. Verification budget (binding): one bounded census of every supported face
   whose raw text contains case-insensitive "cycling", plus 300 random covered controls
   (`cargo xtask english-v3 diff --manifest … --random 300 --seed 20261008`),
   with the identity manifest, 12 workers and `--samples-per-face 0`; compare
   it with `cargo xtask english-v3 diff` against the claim parent's census of
   the same manifest. Never run `--all`. Tests: `cargo test -p` and
   `cargo clippy -p … --tests -- -D warnings` for every crate whose code or consumed data changed (`deckmaste_english_v3` always; `deckmaste_lexical_source` if core.ron/verbs.ron/vocabulary.rs changed). No xtask tests, no
   Lean checks, no post-refresh re-census unless refresh touched grammar,
   lexicon or core. Exception for this ticket: the new `plugins_v2`
   declaration is a plugin definition change, so also run `cargo xtask lean
   check` once and `cargo test -p deckmaste_semantics_v2` (which loads
   `plugins_v2`), and `deckmaste_construction_core` if `BoundSuffixGrammar`
   changes. New tests go under tests/suite/ registered in
   tests/main.rs. Evidence lives under the workspace's ignored
   `target/english-v3/`, never `/tmp`.
2. Probe each witness before implementation and record the counts; write each
   witness as a test. Run the tests and clippy once, at the end.
3. Zero lost faces. A lost face is a defect: fix it within the ticket first;
   STOP only if the fix fails or needs a ruling.
4. Compare per-face Reading counts against the claim-parent census. Any
   decrease on a previously covered face is restored, or named and justified
   in the landing record; a decrease inherited from trunk is attributed by
   change id.
5. Spot-check at least 10 newly covered faces and list them by name with their
   Reading. A typecycling line read as plain *cycling* plus a stray word, or a stem outside the declared class, is a defect. A wrong analysis that parses is a defect, not a gain.
6. No guard may name a lexeme, construction, verb, noun, preposition or card;
   read declared features only. Tests are re-spelled, never deleted. No new
   xtask subcommands, flags, fixtures or tooling.
7. May add the bodyless typecycling declaration (R8a) and edit `grammar:` frame fields in `plugins_v2` keyword actions and lexicon
   frames in `crates/deckmaste_lexical_source/lexicon/*.ron`; plugin bodies
   untouched. Lexicon additions must be attested in the supported corpus
   (pruning rule); check attestation with `cargo xtask english-v3 grep`.
   Data over code: a declaration an existing Construction already reads is
   preferred to a grammar change.
8. A new frame or construction must not overlap an existing one on the same
   string (two labels for one constituency is a spurious duplicate, not an
   ambiguity); when it supersedes one, retire the old route and re-spell its
   tests. Prefer refining an existing Construction: the count is above the
   ADR's 250 ceiling.
9. Retire a superseded route on both the lexicon and the grammar side; do not
   leave unreachable declarations.
10. A CGEL citation may back only what the cited passage itself says; a project
    or orchestrator ruling is cited as a ruling, never attributed to CGEL.
11. The declaration question and *basic landcycling* are ruled (R8a, R8b in
    Analysis); do not STOP on them. If a `plugins_v2` consumer rejects a
    bodyless keyword declaration, that is a genuine blocker: STOP and report.

## Out of scope

- Landwalk residuals (*snow swampwalk*, *legendary landwalk*).
- Cycling-trigger text (*When you cycle this card*).
- The typecycling engine body (cycling with a library search instead of a
  draw): `semantics-v2-keyword-ability-census`, which owns the bodyless
  keyword-ability declarations.

## Landing record

1. **Census delta**: `vzovykxk` bounded baseline (source-equivalent to claim parent `nkkxxuyz`) → implemented `vzovykxk`: 582 → 638 covered faces; 31,223 → 31,465 Readings. Selector: every supported face containing “cycling” (404), plus 300 random covered faces, seed 20261008; two overlaps, 702 identities. Built-in `english-v3 diff`: 56 gained, 646 unchanged; lost faces none; decreases, increases, digest-only and source changes all zero. Exact realisation, traversal and admission validated; zero issues or duplicate Readings. Evidence: `target/english-v3/english-v3-typecycling-bound-suffix/{before,after,diff}.json`.
2. **Witnesses** (`vzovykxk`, Document probes before implementation → after): Canyon Crawler “Swampcycling {2}” 0 → 1; Shepherding Spirits “Plainscycling {2}” 0 → 1; Step Through “Wizardcycling {2}” 0 → 1; Generous Ent “Forestcycling {1}” 0 → 1; Gleam of Resistance “Basic landcycling {1}{W}” 0 → 0, R8b residual. All four positive witnesses pass both roundtrip laws and traversal.
3. **Spot checks** (`vzovykxk`, full-face Readings): Canyon Crawler 1 (swamp + cycling {2}); Shepherding Spirits 1 (plains {2}); Step Through 1 (wizard {2}); Generous Ent 1 (forest {1}); Lórien Revealed 1 (island {1}); Oliphaunt 8 (mountain {1}); Hobbit Hole 14 (halfling {4}); Vedalken Aethermage 1 (wizard {3}); Homing Sliver 4 (sliver {3}, granted and own keyword); Sanctum Plowbeast 1 (plains {2}, island {2}). Each selected keyword is `BoundQualityCostKeyword` with the declared source noun stem, typecycling's bound suffix and CostSymbols; other full-face ambiguities remain. Probe evidence in `spot-analyses.json` and `spot-*.json`.
4. **Deviations and STOPs**: R8a permits the bodyless typecycling declaration; engine body remains owned by `semantics-v2-keyword-ability-census`. Refine-first considered: `BoundQualityKeyword` cannot take a mandatory cost for only QualityCost heads because construction forms must share fields/cardinalities and optional field features cannot be constrained; refining `CostKeyword` would require an additional composite head layer. Used the permitted one-construction fallback, `BoundQualityCostKeyword`. No superseded route or frame; plain Cycling and landwalk remain live. Class-derived CreatureSubtypeStem/Use features prevent new creature stems from licensing Wizardwalk/Sliverwalk. Added glossary entries for Bound Keyword Stem Class, Creature Subtype Stem and Creature Subtype Stem Use, citing the project ruling. No STOP. Supported-corpus grep attests the class, including Hobbit Hole's Halflingcycling {4}; corrected an unrun fixture's attribution/cost. R8b leaves 36 basic landcycling faces unread; no new Basic construction.
5. **Tests** (`vzovykxk`): PASS: `cargo test -p deckmaste_english_v3`; `cargo clippy -p deckmaste_english_v3 --tests -- -D warnings`; `cargo test -p deckmaste_lexical_source`; `cargo clippy -p deckmaste_lexical_source --tests -- -D warnings`; `cargo test -p deckmaste_construction_core`; `cargo clippy -p deckmaste_construction_core --tests -- -D warnings`; `cargo test -p deckmaste_semantics_v2`; `cargo clippy -p deckmaste_semantics_v2 --tests -- -D warnings`; one `cargo xtask lean check`. English: 616 tests; lexical source: 63; construction core: 468. Added 9 tests (8 English integration, 1 lexical-source unit); restored 0, re-spelled 4 (keyword suffix contract, keyword ability inventory, exact template inventory and template consistency count), ignored 0, removed 0. Original assertions retained, inventories extended for the new declaration. Initial English clippy clone-on-Copy finding fixed; lexical-source/core inventory assumptions re-spelled and their failed suites retried. Nonincremental fallback resolved rust-lld undefined-symbol failure; all commands use CARGO_INCREMENTAL=0, test execution RUST_TEST_THREADS=2.
6. **Timing** (`vzovykxk`): bounded census 8.94 s, 6 workers, GNU `/usr/bin/time -v` maximum resident set size 2,672,860 KiB. Preliminary 12-face selector: 2.47 s, 375,232 KiB. Foreground process-group memory supervision enforced 30 GB; no run exceeded the limit. No full corpus census.
