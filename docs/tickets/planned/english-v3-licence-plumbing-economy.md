---
needs: []
---
# Licence plumbing economy: percolation, generic concord, categories and frames instead of flags

## Why

The user approved this refactor on 2026-10-07. A permissive-admission
experiment that day neutralized the grammar's licence features and showed
that licences must stay at admission. Without them the corpus produced ×3.77
Readings and 26 exploded faces, and about a third of covered faces gained a
Reading already ruled wrong. The experiment is archived under the bookmark
`archive-english-v3-licence-experiment-ref`; it is a reference and must never
be integrated.

The licences themselves are correct, but the apparatus that carries them is
expensive. By the experiment's attribution it costs about 489 of 3,174
non-blank lines in `crates/deckmaste_english_v3/src/declarations.rs`, measured
on base `lryqtwskmyxs`. The file was then 374 lines over the 2,800-line
ceiling of the 2026-10-04 construction-economy amendment in
[english-lexical-analysis](../../decisions/english-lexical-analysis.md). The
grammar has 201 ordinary constructions and 43 schemas against the
250-construction ceiling. At mint (claim parent `pozynvyupsvk`) the file has
3,227 non-blank lines. The claimant re-measures every figure on their own claim
parent.

## Goal

- **Readings unchanged, face by face.** The census matches the claim parent
  exactly: the same covered set and the same Reading count on every face, with
  0 lost and 0 changed.
- **Line ceiling met.** `declarations.rs` ends at or under 2,800 non-blank lines.
- **Every per-word licence preserved.** These features keep their meaning and
  stay declared lexical properties that the grammar reads: PrepositionFunctionLicence,
  AuxiliaryEllipsisLicence, BareGenitiveHost, UnmarkedConjunctLicence,
  ClauseInitialAdjunct, VPFinalAdjunct and SelectedPrepositionUse. The
  2026-10-06 licence amendment and its pruning rule stand unchanged.

## Analysis

All line savings below are estimates from the experiment.

- **(a) Flags that duplicate a lexical category.** ComparativeQuantityUse,
  NoncorrelativeCoordination, ScalarComparativeUse, and
  MeasurePosition/CardinalMeasurementUse become `lexical(Category)` requirements.
  Their subcategories are declared in
  `crates/deckmaste_lexical_source/lexicon/vocabulary.rs`. Estimated saving:
  about 30 lines.
- **(b) One-off complement-shape flags on prepositions and adjectives.**
  - The flags: the Finite, Gerund, Quoted, Keyword, PrepositionPhrase and
    BareNominal complement flags, ExtentMarker, FrequencyUnit, DurationUse,
    and Nominal/CompoundComplementMarker.
  - The replacement: frames on prepositions and adjectives, in the style of
    `crates/deckmaste_lexical_source/lexicon/verbs.ron`, read the way
    `frame_marker_licence` reads verb frames.
  - Estimated saving: about 30 lines.
- **(c) Head-feature percolation.** A percolation capability in
  `crates/deckmaste_construction_v3_core` replaces about 128 `export X =
  head.X` lines. Estimated saving: about 120–150 lines.
- **(d) Generic coordination concord.** One concord declaration per feature
  replaces the per-feature tables. Each declaration uses one of four rules: all,
  any, intersection, or same-or-Mixed.
  - What it replaces: about 141 lines of `coordinated_*`, `*_concord` and
    `combined_*` tables, and about 75 policy lines.
  - What stays: the number, person and oblique agreement tables.
  - Estimated saving: about 150 lines.
- **(e) Wildcard table rows.** For example, `clitic_host` shrinks from 28
  rows to about 6.

Total plausible saving: 300–350 lines.

Two compiler capabilities are prerequisites inside this ticket: head-feature
percolation (c) and generic coordination concord (d). Each may land as its own
commit within the ticket, ahead of the declaration conversions that use it.

## Method

Standard constraints apply (CLAUDE.md), plus:

- **Census identity.** Prove the census identical, face by face, against the
  claim parent. Compare the Reading count on every face, not only the covered
  set.
- **Compiler tests.** Each compiler capability lands with its own tests in
  `crates/deckmaste_construction_v3_core`.
- **One class at a time.** Convert the flags class by class, (a) through (e),
  and run a census after each class.
- **No dead declarations.** Retire every superseded feature, table and policy,
  on both the grammar side and the lexicon side. Leave no unreachable
  declarations.
- **No naming guards.** No guard may name a word, construction or card.
- **Line accounting.** Report net non-blank lines for each step.
- **Evidence location.** Keep evidence under `target/english-v3/`, never `/tmp`.
- **CGEL citations.** A CGEL citation may support only what the cited passage
  says.

## Out of scope

- Any coverage gain or loss.
- The census memory change (digest dedup and bounded caches). It is a separate
  decision and has not been approved.
- Changes to grammatical analysis.

## Landing record

The claimant writes this record before integrate. It must include:

- **Inventories.** Before and after counts for non-blank declaration lines,
  ordinary constructions, schemas, features, tables and policies.
- **Census identity proof.** Covered set and per-face Reading counts against
  the claim parent: 0 lost, 0 changed.
- **Savings by class.** For each of (a) through (e), the lines actually saved
  against the estimate above.
