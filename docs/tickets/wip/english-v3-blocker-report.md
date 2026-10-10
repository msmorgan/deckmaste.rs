---
needs: []
---
# Report why each unread face is unread: `cargo xtask english-v3 blockers`

Priority: high. Tooling only; no grammar or lexicon change.

## Why

The census records nothing about an unread face except its unknown words:
`lines` is empty, and no failing line, sentence or position is recorded. Work
has therefore been chosen by regex recon, which left 3,417 unread faces in no
bucket (wave-5 recon, 2026-10-08) and can only find constructions somebody
already suspected. At the per-line baseline (`sskyxnyvmruk`, 32,828 supported
faces, 6,292 unread) 2,027 unread faces carry at least one unknown word, and
nobody knows how many of them are unread for that reason alone. Owner
decision 2026-10-10: build the report before minting more construction
tickets.

## Goal

A subcommand `cargo xtask english-v3 blockers --census FILE [--json OUT]
[--limit N]` that, for every unread face in a schema-8 census, says which
units fail and where, and ranks the causes.

1. **Per-line verdict.** Split the face's analysed source into its ability
   lines and analyse each line on its own, with the face's card scope (the
   same scope `card_scope::for_face` gives the census and `probe --as-card`).
   Record for each line: reads or not, Reading count if it reads, and its
   unknown words.
2. **Localisation of a failing line.** For a line that does not read and has
   no unknown word, record the furthest input position the chart reached
   (the largest token index at which any item was scanned or completed from
   the line start), with the token at that position and the one before it.
   If the chart does not expose that today, expose it read-only from the
   chart statistics; do not change parsing.
3. **Blocker key.** Each failing line gets one key: `word:<lemma or lowercased
   text>` for each unknown word, otherwise `at:<previous token> <failing
   token>` from item 2. Keys are data for ranking, not grammar.
4. **Ranking.** The summary lists blocker keys by the number of faces they
   touch and by the number of faces where they are the *sole* blocker (the
   face has exactly one failing line and that line has exactly one key), with
   up to three example faces each. It also prints: unread faces, faces with
   exactly one failing line, faces whose every failing line is
   vocabulary-only, and line-level coverage over the whole census (lines that
   read / lines).
5. **JSON.** `--json` writes the per-face records (face id and name, per-line
   verdicts, keys) and the ranked table, so tickets can cite it and a later
   run can be compared.

## Analysis

Orchestrator rulings 2026-10-10:

- B1. Reuse the census's own line analysis (`analyze_line` and the segment
  logic that `english-v3-per-line-readings` added); the report must agree
  with the census: a face it calls fully reading must be covered in the
  census, and a covered face has no failing line. Assert that on the run and
  fail loudly on a mismatch.
- B2. Line splitting for an unread face cannot come from a Document parse
  (there is none). Split on the analysed source's newlines; where the census
  segments a covered face differently (quoted multi-line abilities, modal
  bullet lists), record the disagreement count in the summary rather than
  special-casing by card.
- B3. The report is diagnostic. It adds no field to the census schema and no
  gate; nothing may read it to admit or reject a Reading.
- B4. No Readings are materialised beyond what a line's count needs; run with
  the same worker pool as the census and stay under the census's peak memory.

## Witnesses

Probe-level expectations, each written as an xtask test on a synthetic
two- or three-line card (never a pinned corpus count):

- a face whose second line contains an undeclared word: line 1 reads, line 2
  fails with key `word:<that word>`, sole blocker;
- a face whose only line is declared vocabulary in an ungrammatical order:
  key `at:<prev> <token>` at the first token the chart cannot pass;
- a covered face: no failing line;
- two failing lines: listed under both keys, sole blocker for neither.

## Method

Standard constraints apply. Deltas:

1. No grammar, lexicon or census-schema change. `cargo test -p xtask
   english_v3` and `cargo clippy -p xtask --tests -- -D warnings`; if the
   chart statistic needs a field in `deckmaste_english_v3`, that crate's full
   suite and clippy too.
2. Run it once on a current full census. If none of the claim parent exists,
   run one `--all --workers 6 --samples-per-face 0` census under
   `/usr/bin/time -v` (about 4 minutes) and keep it at
   `/home/msmorgan/Projects/deckmaste.rs/target/english-v3/baseline-blockers.json`
   with the report beside it as `blockers.json`.
3. Tests assert relations (which line fails, which key, sole or not), never
   incidental numbers or Debug-printed trees.
4. Document the command in the same place `diff` and `grep` are documented.

## Out of scope

- Sentence-level splitting inside a line, suggestions of fixes, and any
  clustering smarter than the two key kinds above. Mint a follow-up if the
  `at:` keys prove too coarse.

## Landing record

Six items as specified in `docs/tickets/README.md`. Item 1 is replaced by the
report's own summary on the full census (unread faces, single-failing-line
faces, vocabulary-only faces, line-level coverage, top 30 keys by sole-blocker
faces); item 3 by ten faces checked by hand against `probe --as-card`.
