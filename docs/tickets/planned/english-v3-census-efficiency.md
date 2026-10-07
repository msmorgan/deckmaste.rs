---
needs: []
---
# Census efficiency: digest dedup, bounded caches, per-face Readings digest, lexicon-impact incremental mode

## Why

On 2026-10-07 a permissive-admission experiment pushed one census to 27 GB.
A read-only review then found that census memory grows with the number of
Readings (O(Readings)) in six places, none of them bounded (lines as of claim
time):

1. The per-face checked set, `crates/xtask/src/english_v3/mod.rs:220` and
   `:258`. It keeps the hex SHA-256 identity string plus the Reading's AST
   `Arc` for every checked Reading.
2. The enumerator's `seen: BTreeSet<M::Reading>`,
   `crates/deckmaste_english_v3/src/readings.rs:82` and `:218`. It keeps every
   distinct `TracedValue` together with its trace vectors.
3. The enumerator's `pending: Vec<Work<…>>`, `readings.rs:81`. It clones work
   items at `:119`–`:121`.
4. The materialization memo `Tracing.leaves` and `Tracing.builds`,
   `crates/xtask/src/english_v3/validation.rs:96`–`:97` (inserts at `:155`
   and `:182`).
5. The AdmissionCache `summaries` map,
   `crates/deckmaste_construction_v3_core/src/runtime.rs.txt:557` (insert at
   `:590`).
6. The all-faces report collector, `analyze_cards` in `english_v3/mod.rs`
   (`par_iter().map(analyze_face).collect()`), which holds every `FaceReport`
   until the end of the run.

The experiment built the small variant: dedup by digest, drop the retained
traces, and cap the memo and AdmissionCache at 100,000 entries each, clearing
when full. That code is the reference. It lives on the archived line
`archive-english-v3-licence-experiment-ref` (prkrlprq, "experiment (reference,
never integrate)", measured on base lryqtwskmyxs). Its census part is the
saved diff `census-memory.diff`, which touches `readings.rs`,
`validation.rs`, `english_v3/mod.rs` and `runtime.rs.txt`. The experiment
measured it on the baseline grammar with 4 workers:

| | before | after |
|---|---|---|
| peak RSS | 8,313,392 kB | 5,416,292 kB |
| corpus wall | 221,777,967,803 ns | 156,254,326,458 ns |

Results were identical face by face. The user approved landing it
(2026-10-07). Do not integrate the archived line itself: its
permissive-admission half is not part of this ticket.

## Goal

**A. Digest dedup, trace drop and bounded caches.** Land the census-memory
change. Prove census identity face by face against the claim parent: the
covered count, the Reading count for each face, and the inventories. Report
peak RSS and wall time before and after, at 4 workers and at 12. Every Reading
is still fully materialized and validated. Assurance is unchanged and nothing
is sampled. A bounded cache may only cost recomputation, never a result.

**B. A Readings digest for each face in the census JSON.** Hash the sorted list
of per-Reading digests for the face with a stable hash. Two runs can then be
compared by digest, which catches a face whose count stayed the same while
its Readings changed. Define the digest input exactly: which per-Reading
digest is used, how it is encoded, and how the list is sorted and separated.
Put a version field in the JSON so the definition can change later.

**C. An incremental mode.** Given an earlier census JSON and the current tree:

- diff `crates/deckmaste_lexical_source/lexicon/{core.ron,verbs.ron,vocabulary.rs}`;
- find the affected lexemes, then their surface forms;
- re-check only the faces whose text contains one of those forms;
- copy the earlier results for every other face.

Mark the output as incremental and record the path of the base census. A
grammar change (`declarations.rs`) turns incremental mode off, because the
method is sound only for changes confined to the lexicon. The one exception
would be a chart-level record of predicted categories. That record is out of
scope here and is a possible follow-up. One full `--all` run per landing
remains the authority.

## Method

Standard constraints apply. Land as a series: A, then B, then C.

- A: the identity proof above.
- B: a stability test. Running the same tree twice gives identical digests,
  and one lexicon change alters only the expected faces' digests.
- C: a soundness test. On a lexicon-only change, the incremental result must
  equal a full run. Assert this on the diff of a real past landing, e.g.
  `english-v3-combat-interval-noun`.

Timings are integer ns. No new fixtures are checked in. Evidence goes under
`target/english-v3/`.

## Out of scope

Sampled validation; counting at the forest level; tracking predicted
categories at the chart level (the follow-up above).

By-name corpus selection and nickname derivation are owned by `card-corpus-fast-lookup`.

## Landing record

The identity proof (A); RSS and wall tables at 4 and 12 workers; the digest
definition and its version (B); the evidence that incremental mode is sound
(C).
