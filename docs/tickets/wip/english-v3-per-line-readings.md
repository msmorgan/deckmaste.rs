---
needs: []
---
# Enumerate and digest Readings per ability line, not per face

Priority: HIGH. Owner direction (2026-10-09): "expand readings per-ability-line
instead of combinatorially exploding."

## Why

A face's Reading count today is the PRODUCT of its lines' counts, because the
census enumerates whole `Document` Readings. Goliath Daydreamer is
1,050 × 1,194 = 1,253,700 Readings; Chandra, Pyromaster was
76 × 3 × 2 × 2 × 3 × 30 × 3. Each worker materialises, validates and keeps a
32-byte dedup digest (twice: once in `Readings::seen_digests`, once in the
census `checked` set) for every one of those document combinations. On
2026-10-08 five concurrent bounded censuses, one of them at 2.5M Readings, ran
a 128 GB host out of memory. Per-line expansion bounds work and memory by the
largest single line (about 3,028 Readings, Shadow Puppeteers) and makes counts
additive.

## Where the product arises

`crates/xtask/src/english_v3/mod.rs:266-267` (`analyze_face`) calls
`forest.readings(Tracing::new(grammar))` on the `Document` root, and the loop
from there to `mod.rs:352` validates every yielded value and inserts its
`dedup_key()` into `checked`. The multiplication itself is the depth-first
traversal in `crates/deckmaste_english_v3/src/readings.rs`: at each
`Task::Family` / `Task::Edge` step (lines 166-189) every alternative of every
child is forked as a sibling continuation, so the `Document` node's
`first: Ability, rest: repeat(AbilityContinuation, "")` chain
(`declarations.rs:1843-1849`, `AbilityContinuation = ["\n", ability: Ability]`)
yields the cross product of its `Ability` children before `Task::Build` at
line 208 builds each document value. The chart is already packed; only the
enumeration multiplies.

## Goal

The census materialises Readings per top-level line and never materialises a
document-level combination.

- **Segment from the packed forest.** A top-level line is one `Ability`
  constituent of the root `Document` as the grammar segments it (the `first`
  ability and each `AbilityContinuation`'s `ability`), not a `\n` split of the
  source. Walk the root's families and intermediate edges to collect, per line
  span, the completed `Ability` nodes that participate in a root derivation
  (one span can carry several packed `Ability` nodes, e.g. different
  `LabelKind` summaries; the line's Readings are the union of theirs). Add the
  minimal public forest API this needs, e.g. a segment walk plus
  `Readings` started from a chosen set of completed nodes instead of
  `forest.roots`. Quoted `Document`s nested inside a line are not top-level and
  are not split.
- **Segmentation must be unique.** If two root derivations disagree on the
  line spans, record an issue (e.g. `Issue::AmbiguousSegmentation`) for that
  face; never fall back silently to face-level enumeration. A `TypeLine` field
  census, and any root that is not a `Document`, is one segment.
- **Count and digest each line.** Per line: enumerate with
  `with_digest_dedup`, validate each Reading against that line's surface, keep
  a per-line `checked` set, and compute `readings_sha256` with
  `report::readings_digest`. Drop the line's set once its digest is taken.
- **Face record.** `checked_readings` is the SUM of line counts;
  `readings_sha256` is a SHA-256 over the ordered line digests (fixed-width,
  so concatenation is unambiguous); add
  `lines: [{span (start, end bytes in analyzed_source), text_sha256,
  checked_readings, readings_sha256}]`. `exact_readings`, `census`
  (`No`/`One`/`Multiple`/`Undetermined`) and `enumeration` derive from the line
  records: Complete iff every line is Complete, Limited if any line is
  Limited. `--reading-limit` caps requests per line. Samples and per-face
  `constructions` counts are taken over line Readings (the counts become
  per-line sums; say so in the report doc comment).
- **Laws per line.** The roundtrip law (`render(v) = s`) and the traversal law
  are checked for each line's Readings against that line's span of the
  analyzed source; the document-level `"\n"` literals carry no alternatives, so
  face-level realization is the concatenation and needs no separate
  enumeration. Confirm `RealizationContext` built over the whole analyzed
  source realizes an `Ability` Reading to its line text (adjust the context,
  not the law, if it does not).
- **Coverage unchanged in meaning.** A face is covered iff every line has at
  least one Reading, which is exactly today's "product > 0". A face with a
  zero-Reading line reports `checked_readings: 0` (the sum of the other lines
  must NOT make it look covered) while its `lines` array still shows which
  lines read; "0 Readings" semantics are otherwise unchanged.
- **Probe.** `english-v3 probe --category Sentence/Document/...` keeps its
  behaviour: it parses the given text at the given Category and enumerates as
  today. Optionally print per-line counts for `Document`; do not change what it
  admits.
- **Diff.** `english-v3 diff` keeps working unchanged on the face-level fields
  (`checked_readings`, `readings_sha256`, coverage). Add `--lines`: for faces
  present in both reports with line arrays, list the lines (face name, span or
  line index, before/after count) whose count or digest changed. With
  `--lines`, a report without line arrays is an error, not an empty diff.
- **Grep.** `grep --census --unread` / `--covered` unchanged.
- **Incremental and schema.** Bump `report::SCHEMA_VERSION` (7 → 8). The
  existing schema guard in `incremental.rs` then refuses old bases; reused
  faces carry their `lines` array forward.
- **ADR.** Add a short dated amendment to
  `docs/decisions/english-lexical-analysis.md`: the census counts and digests
  Readings per top-level line; a face's Reading set is the product of its line
  sets; the per-face digest the six-item record relies on is the hash of the
  ordered line digests. Every grammatical Reading stays retained; this changes
  the census representation, not admission.

## Consequences

Every multi-line face's `checked_readings` and `readings_sha256` change once at
landing (sum instead of product; new digest). Construction totals change for
the same reason. So this landing's proof is NOT a census diff against the
previous census. Instead:

1. **Identity check:** the set of covered faces is unchanged against the
   current batch base: 0 lost, 0 gained (the `diff` gained/lost lists).
2. **Product check:** for 300 random faces (seeded, listed in the evidence),
   the product of the new line counts equals the old face `checked_readings`
   (compute in `u128`). This is the correctness proof of the per-line split.
   Any mismatch is a defect to fix, not a disclosure.
3. The landing's full census becomes the new batch base; the orchestrator
   re-baselines on it.

Memory: report peak RSS of the full census before (the current base command)
and after (`/usr/bin/time -v`, maximum resident set size), same worker count;
expect a large drop.

## Method

Standard constraints apply. Tests: `cargo test -p xtask` (english_v3 module)
and `cargo test -p deckmaste_english_v3`, plus `cargo clippy -p <crate> --tests
-- -D warnings` for both. Add unit tests for: segmentation of a multi-line
`Document` from the forest; sum count and ordered-line digest; zero-Reading
line makes the face uncovered; ambiguous-segmentation issue; `diff --lines`;
schema bump refusing an old incremental base. Re-spell, never delete, any
existing test asserting face-level product counts.

One full census, the exception to the bounded-iteration rule, with
`--workers 6` for memory; iterate on a selector first. Evidence (census JSON,
`time -v` output, the 300-face product table) under `target/`.

Landing record: the six items, plus before/after totals (faces, covered,
summed Readings), the identity check result, the 300-face product check, and
peak RSS before/after.

## Out of scope

Reducing ambiguity itself (attachment policy). Packed / preferred-Reading
selection is `english-v3-packed-preferred-readings`: that ticket avoids the
same document cross product on the presentation path; this one removes it from
the census. They share the segment walk; whichever lands second reuses it.

## Landing record

1. **Census delta.** `yntwupzymxxl` before (schema 7, unchanged claim-parent
   grammar at launch) → `yntwupzymxxl` after (schema 8): 32,828 supported faces
   and 26,536 covered faces on each side; checked Readings 3,725,990 document
   combinations → 967,276 line Readings. Native `english-v3 diff` reports
   lost 0, gained 0, decreases 2,547, increases 12,056, digest-only 11,933,
   unchanged 6,292; identity-only and source-hash changes 0. Lost faces: none.
   Decreases are the prescribed product-to-sum representation change; no
   grammatical Reading was retired. The 300 covered identities selected by
   native diff with seed 20261008 have zero u128 product mismatches against
   the before counts (`yntwupzymxxl`, both phases). Evidence: `target/english-v3/
   english-v3-per-line-readings/{before,after,diff,product-proof}.json` and
   `product-table.tsv`; the before provenance records claim parent
   `wnrmklzsqrwv` and claim `nxzntorpvzvo`. Refresh to `nlvnvwnwovxs`
   brought trunk `lztruvkmznnx` (Exclude nominal Until attachments). Its
   bounded manifest covers all 5,695 supported faces with the word until
   plus 300 seeded controls, 5,944 identities after overlap: covered
   4,918 → 4,918, lost/gained 0, decreases 627, increases/digest-only 0,
   unchanged 5,317; line counts 208,096 → 136,645. Those removed Readings
   belong to the named trunk change, not this representation change;
   `refresh-diff.json` names every affected face and line.
2. **Witnesses.** `yntwupzymxxl` after: Goliath Daydreamer 2,244
   (lines 1,050 + 1,194; product 1,253,700); Chandra, Pyromaster 286
   (12 + 4 + 270; product 12,960); Shadow Puppeteers 3,038
   (1 + 9 + 3,028; product 27,252). Revitalize 2 (1 + 1); Forest 1
   (empty Document). Before implementing, the three ticket probes exhibited
   the old whole-Document enumeration; their capped outputs remain
   byte-identical after. This ticket addresses representation and work, so
   these witnesses have no admission failure.
3. **Spot checks.** No newly covered faces by design. Ten covered names
   checked at `yntwupzymxxl` after, with selected analysis and line-sum count:
   Lightning Bolt 1 (proper-name Subject in a Declarative; both ordinary and
   adventure faces); Seedborn Muse 6 (Imperative with clausal Preposition);
   Serra Angel 2 (Flying and Vigilance Keyword Lines); Giant Growth 2
   (Declarative with target Subject and duration); Llanowar Elves 1
   (Activated Ability, tap Symbol Cost); Opt 2 (Scry and Draw Imperatives);
   Abundant Growth 3 (Enchant Keyword Line, triggered clause, quoted
   Activated Ability grant); Ajani's Pridemate 1 (Whenever initial
   Preposition with clause complement); Revitalize 2 (gain-life Declarative
   and Draw Imperative); Forest 1 (EmptyDocument). `spot-checks.json` holds
   the selected samples for 11 faces under these ten names.
4. **Deviations and STOPs.** No grammar construction, lexical frame,
   admission guard or route added/deleted. EmptyDocument is represented by
   one empty segment to preserve its existing coverage and single Reading.
   Cycle and missing-segment diagnostics accompany ambiguous segmentation;
   a synthetic nested quoted Document checks structural segmentation and
   Unicode byte spans, without claiming an unattested Magic wording. The
   dated ADR amendment cites owner direction, not CGEL. No glossary gap or
   Resolved STOP: sample cost expectation, described in item 5. The
   orchestrator's standard brief initially omitted xtask
   test execution; that template was not a user override of this ticket's
   Method. Review requires `cargo test -p xtask english_v3`, recorded below.
   CLI checks exercise limit, schema fallback, Type Lines and line diff.
   Unparsed faces carry `lines: []`, without partial line records; this
   deviates from the ticket's wording about displaying an unread line.
   The largest single line in the full census is 43,560 Readings
   (Necromentia, `yntwupzymxxl`), not 3,028. `census: one` means that every
   line has exactly one Reading; consequently a face may be `one` while
   `checked_readings` is greater than one (Revitalize has two).
5. **Tests.** At `yntwupzymxxl`: `cargo test -p deckmaste_english_v3`
   passed (653 tests, zero failed/ignored);
   `cargo clippy -p deckmaste_english_v3 --tests -- -D warnings` passed;
   `cargo clippy -p xtask --tests -- -D warnings` passed after correcting
   four style errors (borrowed segment, explicit cycle arm, import placement,
   fixed-size byte chunks). No full test/census rerun after these equivalent
   style changes. At `yntwupzymxxl`: restored
   0, re-spelled 3 before review (per-line cap/count assertions, line-digest
   terminology, schema-7 fallback), ignored-with-blocker 0, added 10 (four English
   integration tests and six xtask tests), removed 0. Five CLI checks passed;
   three probe outputs unchanged; full census roundtrip/traversal issues 0.
   After the lexical dependency changed during refresh, bounded census
   issues remain 0 at `nlvnvwnwovxs`; the same English test and two clippy
   commands passed at `nlvnvwnwovxs`: English tests 657 passed, zero
   failed/ignored; both clippy commands passed with warnings denied.
   Review verification at `tmupwxzuslok`: `cargo test -p xtask english_v3`
   failed (46 passed, 1 failed, 0 ignored, 352 filtered out).
   `cheapest_frequency_sample_preserves_all_readings_in_census` retains its
   two Readings and selected FinitePredicate, but expects Document sample
   cost 20; the sampled Ability has actual cost 19 after removing the
   Document node. STOP resolved by orchestrator ruling (2026-10-09):
   re-spell the sample cost assertion to 19 because the per-line Reading
   no longer includes the Document node's cost of one; retain the
   Reading-count and selected-analysis value comparisons. At `tmupwxzuslok`,
   one additional test re-spelled for that reason (total re-spelled 4);
   restored/ignored/added/removed counts unchanged. At `tmupwxzuslok`,
   the one rerun of `cargo test -p xtask english_v3` passed: 47 passed,
   0 failed, 0 ignored, 352 filtered out in the library; other test binaries
   ran zero matching tests. The re-spelled sample-cost test passed.
6. **Timing.** `yntwupzymxxl` after: one full census, 241.000 seconds,
   six workers. GNU `/usr/bin/time -v` maximum resident set size:
   `yntwupzymxxl` before 22,357,172 KiB → `yntwupzymxxl` after
   17,215,724 KiB, identical worker count. The before command includes a
   cold cargo build (29:08 elapsed versus 724 seconds corpus wall): its
   22,357,172 KiB maximum covers rustc, linker and census, so it is an
   upper bound on census peak RSS. The earlier 23% reduction claim is
   withdrawn; these measurements do not establish a census-only percentage.
   The before census was not rerun. Selector was
   `--all --workers 6 --samples-per-face 0`; 32,828 faces. Iteration used
   the five witness names; the product proof used 300 seeded covered
   identities. Timing evidence: `before.time`, `after.time`, and report
   `wall_ns`, under the ticket's target directory.
