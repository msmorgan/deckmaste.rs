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
- Performance advisory (reviewer re-measurement, final tree): coverage used 8
  workers, took 134.515 s at 151,569 ns/B, host load 28.85/26.49/21.35;
  ambiguity used 8 workers, took 170.264 s at 202,240 ns/B, host load
  27.67/28.78/23.31. Both exceed the 16.26 s quiet-host ceiling and are
  advisory only: contention at measurement was five live executor workspaces
  plus two concurrent landing reviews (coordinator stamp; the implementer's
  sandbox cannot count concurrent processes, so its own contention line is
  not evidence). The implementer measured 146,267 ns/B (coverage) and
  121,744 ns/B (ambiguity) on its pre-refresh tree.
- Assurance census: restored 0; re-spelled 0; ignored 0; added 4 (3 by the
  implementer, 1 by review); removed 0. The `cargo test -p xtask` suite
  reports 1 ignored test; it is the pre-existing
  `crates/xtask/src/macros/templates.rs` on-demand corpus cross-check with a
  named blocker, untouched by this landing and arriving with the refresh.
  Deviations and additions: the review commit adds one test and one two-line
  cross-reference comment beyond the ticket's letter (both listed below).
  STOP: none. Glossary gap: none.

### Review corrections

Reviewed on change `txlqqrvw` (the gated tree; this record commit adds ticket
text only). Every closure branch was falsified once by mutating the guard and
re-running the branch tests.

- MEDIUM — the ticket's central change was unpinned. Restoring the pre-change
  rule (`derived ⊆ reviewed` plus `reviewed ⊆ surfaces`) leaves all three
  delivered tests passing: they pin the missing, stale, and duplicate branches
  that already existed, not the two-way equality this ticket adopts. The
  branch the adoption actually adds — an override on a surface the recipe can
  classify — had no test, and `render_stub` prefers an override over the
  recipe, so such an entry silently changes generated output. Fixed by adding
  `onset_override_inventory_rejects_an_override_on_a_classifiable_surface`;
  under the pre-change rule that test, and only that test, fails.
- MEDIUM — the ADR amendment was appended as a new `## Amendment:` section in
  the middle of the Terminals section, reparenting the unrelated
  `**Lexical coverage gate:**` paragraph under it. Fixed by re-spelling it in
  the dated inline form the same section already uses for in-context
  amendments (`**Plan 08 cross-envelope obligation (2026-08-23):**`), in
  place, with the amended sentence retained verbatim.
- LOW — the flavor-word guard mirrors the card-name guard's rule with no
  cross-reference. Fixed with a two-line doc comment naming
  `ensure_card_name_onset_override_inventory` as the mirrored rule; the ADR
  amendment is now the shared authority for both inventories.
- LOW — the implementer's commit lacked the crate prefix; re-described as
  `xtask: close the flavor-word onset-override inventory`.

Provenance inventories as named lists (the record's counts alone did not meet
the 2026-09-04 landing-record amendment). Licensed vocab/lexicon homographs (2):
`AttributiveAdjective::Untap` beside the `Untap` keyword-action Verb
declaration; `TargetingMarker::Target` beside `CommonNoun::Target`.
Form-literal/vocabulary overlap surfaces (9): `additional` at
`additional_cost` atom 2; `to` at `up_to_quantifying_determiner` atom 1; `the`
at `definite_next_mass_quantity_reference` atom 0; `next` at
`definite_next_mass_quantity_reference` atom 1; `to` at
`scalar_less_than_or_equal_to` atom 4; `the` at `number_of_scalar_value` atom
0; `the` at `greatest_scalar_value` atom 0; `other` at
`other_than_qualified_reference` atom 1; `the` at `positional_partitive` atom
0. Permitted licensing checkers: 20 emitted, 0 forbidden. The parser
environment loaded without error on every corpus command.

Branch falsification (each mutation applied to the guard, branch tests re-run,
guard restored):

| mutation | failing test |
| --- | --- |
| duplicate `ensure!` made unconditional | `..._rejects_a_duplicate_exception` |
| `==` weakened to `derived.is_subset(reviewed)` | `..._rejects_a_stale_exception` |
| `==` weakened to `reviewed.is_subset(derived)` | `..._rejects_a_missing_exception` |
| pre-change rule restored | `..._rejects_an_override_on_a_classifiable_surface` |

Reviewer gate artifacts on the gated tree: `cargo fmt --all --check` clean;
`cargo clippy -p xtask --all-targets -- -D warnings` clean; `cargo test -p
xtask` `test result: ok. 458 passed; 0 failed; 1 ignored` (lib) plus 12 binary,
1 determinism, 1 flavor-word integration test; `cargo xtask english_v2
flavor-words --check` reported `flavor-word stubs are up to date (630 files)`;
`DECKMASTE_COVERAGE_LOCK=report cargo xtask english_v2 coverage --check
--workers 8` reported `covered_units:17601`, `selected_units:17601`,
`selected_uncovered_units:0`, `unresolved_ties:0`, `internal_failures:0`,
`roundtrip_mismatch_units:0`, `ownership_failure_units:0`,
`traversal_failure_units:0`, `licensing_checker_forbidden:0`, `lock_mode=report`
with zero `newly covered` and zero `no longer covered` lines and the lock file
byte-unchanged; `cargo xtask english_v2 ambiguity --require-resolved --workers
8` reported `unique=13759`, `specificity_resolved=3842`, `exception_resolved=0`,
`unresolved_ties=0`; `cargo xtask english_v2 report` reported `construction
declarations (387)`. No citations changed. Selection neutrality: the census is
identical to the implementer's pre-refresh measurement, and the diff carries
zero grammar change.
