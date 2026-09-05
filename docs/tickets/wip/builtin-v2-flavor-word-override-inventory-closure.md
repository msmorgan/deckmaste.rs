---
needs: [builtin-v2-flavor-word-generator-onsets]
---
Flavor-word onset-override inventory: pin every closure branch and align the
closure rule with the card-name template (generator-onsets landing review M1,
L1). `authored_onset_overrides` in `crates/xtask/src/english_v2/flavor_words.rs`
hard-fails on missing, stale, and duplicate entries, but only `missing` has a
test; deleting the stale or duplicate branch leaves every gate green. Add one
test per branch, modelled on the card-name inventory test beside it
(`english_v2.rs`, the `CARD_NAME_ONSET_OVERRIDES` closure test). The card-name
inventory enforces two-way equality per the rewrite ADR (a surface the recipe
can classify may not carry an override); the flavor-word inventory currently
permits a redundant override. Adopt two-way equality here too so both
inventories share one closure rule, and extend the ADR sentence's scope from
"card-name rows" to both authored override inventories — that extension is
pre-ruled (coordinator, 2026-09-04): the claimant appends the dated amendment
in the same landing; it is not a STOP. Zero grammar change;
generated stubs byte-unchanged. Standard constraints apply.

## Landing record

Start: 2026-09-04T20:15:19-07:00. End: 2026-09-04T20:28:18-07:00.

- Measured on refreshed change `ytoprnyrrvrm`, with the schema-4 lock at
  17,601 covered identities. The flavor-word override inventory now derives
  the recipe-exception set and requires exact equality with the reviewed set;
  duplicate reviewed surfaces remain independently rejected. The three new
  tests cover a missing exception, a stale exception, and a duplicate
  exception. The generated flavor-word stubs are byte-unchanged (630 files).
- The pre-ruled 2026-09-04 ADR amendment extends the two-way onset-override
  closure from card-name rows to both authored override inventories. This is a
  data-inventory closure only: zero grammar changes, zero selected-analysis
  changes, and no generated stub changes.
- Coverage before/after: 17,601 -> 17,601 selected and covered units out of
  32,641; report-mode lock delta `+0/-0`, byte-unchanged. Newly covered
  identities and selected analyses: none. Identities no longer covered:
  none. No selected-uncovered units, unresolved ties, internal failures,
  roundtrip mismatches, ownership failures, traversal failures, or forbidden
  licensing checkers.
- Construction declarations before/after: 387 -> 387. Selection census:
  13,759 -> 13,759 unique and 3,842 -> 3,842 specificity-resolved; no
  exception-resolved selections. Licensing-checker census: 20 permitted / 0
  forbidden. Homograph inventory: 2 licensed entries; form-literal/vocabulary
  overlap inventory: 9 entries.
- Positive gates on the refreshed tree: `cargo fmt --all` completed; `cargo
  clippy -p xtask --all-targets -- -D warnings` completed; `cargo test -p
  xtask` reported `test result: ok` for 457 unit tests, 12 binary tests, 1
  determinism integration test, 1 flavor-word integration test, and 0
  doctests; `cargo xtask english_v2 flavor-words --check` reported
  `flavor-word stubs are up to date (630 files)`; report-mode coverage ended
  with `covered_units:17601`, `unresolved_ties:0`, `internal_failures:0`, and
  `lock_mode=report`; ambiguity reported `unresolved_ties=0`; roundtrip
  reported `clean 17601` and `mismatched 0`. No citations changed, so citation
  gates were not required.
- Performance advisory: coverage used 8 workers, took 114.723 s at 146,267
  ns/B, host load 10.07/8.33/4.81; ambiguity used 8 workers, took 109.955 s at
  121,744 ns/B, host load 15.78/10.15/6.06; roundtrip used 8 workers, took
  201.178 s at 165,814 ns/B, host load 38.18/22.67/11.72. These exceed the
  16.26 s quiet-host ceiling under concurrent load and are advisory; the
  sandbox cannot measure concurrent-process count for the reviewer stamp.
- Assurance census: restored 0; re-spelled 0; ignored 0; added 3; removed 0.
  Deviations and additions: none. STOP: none. Glossary gap: none.
