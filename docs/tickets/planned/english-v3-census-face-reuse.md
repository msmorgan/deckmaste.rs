---
needs: []
---
# Re-expand only the faces a grammar change can reach: per-face construction sets and grammar-aware census reuse

## Why

The user, 2026-10-07: "What I wanted to avoid was expanding all Avacyn readings
if nothing about her changed."

The landed `--incremental-from` (`english-v3-census-efficiency`) did not meet
this goal for grammar changes. It reuses faces only when the change is confined
to the three lexicon sources. Any other change, including every
`declarations.rs` edit, falls back to a full run and re-expands all 32,828
supported faces. That includes faces like Avacyn, Angel of Hope (9,801
Readings) that the change cannot reach.

Current shape:

- `crates/xtask/src/english_v3/incremental.rs:12-16` `LEXICON_SOURCES` lists
  `core.ron`, `verbs.ron` and `vocabulary.rs`. `Snapshot::capture` (`:31-89`)
  hashes 16 source directories, among them the whole of `crates/xtask/src`
  (`:50`), plus the root Cargo files and the oracle snapshot.
- `Base::incompatibility` (`:184-232`) returns `non-lexicon source changed:
  <path>` for the first changed key outside `LEXICON_SOURCES` (`:206-212`).
  So an unrelated xtask edit, such as the Lean checker, also disables reuse.
- `analyze` (`:247-323`) re-expands a face when its source changed or its
  recorded `analyzed_source` contains a surface form of a changed lexeme. It
  copies every other face from the base.
- `crates/xtask/src/english_v3/mod.rs:296-305` counts `constructions` per face
  from completed, validated Reading nodes only (`report.rs:139`, next to
  `readings_sha256` at `:135`). A construction that was predicted but never
  completed on a face is not recorded, so this map cannot decide reuse.
- The chart already holds the needed set, but `parse` drops it. `chart.rs:35-41`
  `Item` carries a `production` index. `predict` (`:234-252`) runs once per
  `(position, category)` through the `predicted` set (`:59`) and inserts one
  item per production of that category. `parse` (`:73-118`) returns only the
  `Forest`, whose `Metrics` (`forest.rs:70-82`) are counters. Collecting the
  production indices of `items`, or the predicted categories, at the end of
  `parse` is one pass over data the chart already built.
- A runtime `Production` (`deckmaste_english_v3/src/lib.rs:46-49`) has only a
  category and symbols, with no construction identity. Nodes name their
  construction only after materialization (`Reading::construction`,
  `deckmaste_construction_v3_core/src/emit.rs:544`). The compiler emits
  productions from IR rules (`emit.rs:48-55`, IR `Rule` at `ir.rs:97`), so it
  must also emit a production→construction table. `begin` returns no state for
  selected-frame templates (`runtime.rs.txt:439-447`). A template's
  constructions must still count as reachable when its category is predicted.

## Goal

1. **Measure first.** Record, for one full census, the set of constructions
   (with their categories) that appeared anywhere in each face's chart:
   predictions and dead ends included, not only those in completed Readings.
   Then report, for the diffs of three past landings, how many faces each would
   have re-expanded against 32,828:
   - grammar-only: `english-v3-cost-scalar-complement`
   - lexicon-only: `english-v3-combat-interval-noun`
   - one mixed landing that changed both

   If a typical grammar change reaches more than roughly half the faces, STOP
   and report rather than build steps 2–3. The mechanism would not pay back.
2. **Record per face.** Store the set compactly in `FaceReport`, next to
   `readings_sha256`, as sorted construction names or ids. Bump the schema
   version. The instrumentation must not change parse results.
3. **Grammar-aware reuse.** `--incremental-from` diffs the base and current
   declaration sources at the construction level:
   - a changed construction body maps to that construction;
   - a changed table or policy maps to every construction that reads it;
   - a changed feature maps to the constructions whose requires read it, plus
     the lexemes carrying it (those join the lexicon surface-form rule);
   - a new construction maps to its category: every face whose recorded set
     predicted that category is affected;
   - a removed construction maps to itself.

   Combine this with the existing lexicon surface-form rule. Re-expand a face
   only if its recorded set intersects the affected constructions or
   categories, or its text contains an affected surface form. Copy every other
   face from the base. A declaration change the diff cannot attribute falls back
   to a full run with a named reason. It never guesses.
4. **Narrow the snapshot.** Hash only the census-relevant xtask files
   (`crates/xtask/src/english_v3/`, `crates/xtask/src/raw_corpus.rs`) and the
   parser and lexicon crates. Unrelated xtask edits must not disable reuse.
5. **Soundness.** A reused result must equal a full run. Prove it on the three
   landings above: run both modes on each and diff the per-face digests. The
   required result is 0 differences.

## Method

Standard constraints apply. Tests for a landing:
`cargo test -p xtask -p deckmaste_english_v3` and clippy. No Lean checks. Deltas:

- Land as a series:
  1. measurement and recording;
  2. grammar-aware reuse;
  3. snapshot narrowing.
- Prove census identity on the recording step: per-face digests are identical
  to the previous census.
- Run one full census per proof, no more. Keep evidence under `target/`. Write
  timings as integer ns.
- Put new tests under `tests/suite/` and register them in `tests/main.rs`.
  These include soundness tests for each mapping rule in Goal 3: changed body,
  table or policy, feature, new construction and removed construction.

## Out of scope

- Changing what a Reading is or how it is validated.
- Sampled validation.
- Any new mandatory step in the landing contract. Reuse stays an option for
  iteration and review. A landing record's census may be an incremental run
  only after this ticket's soundness proof has landed, and the record must say
  which base it reused.

## Landing record

Include, beyond the standard tiers:

- the measurement table: faces re-expanded against the total for each of the
  three diffs;
- the three soundness proofs;
- the recording-step identity proof;
- the snapshot-narrowing evidence: an unrelated xtask edit keeps reuse on.
