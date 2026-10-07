---
needs: []
---
# Consolidate integration-test binaries so the grammar loads once per package

## Why (user, 2026-10-07)

Every `tests/*.rs` file compiles to its own test binary, and each binary
spends about 5 s loading the lexicon and compiling the declarations before it
runs tests that take milliseconds. Counted on 2026-10-07:

| Package | `tests/*.rs` files |
|---|---|
| `deckmaste_english_v3` | 63 (plus `tests/common/mod.rs`) |
| `deckmaste_construction_v3` | 13 |
| `deckmaste_lexical_source` | 9 |
| `xtask` | 2 |
| `deckmaste_construction_v3_core` | 0 |

The derived gate runs about 94 suites one after another: about 8 min on an
idle host and 18 min on a loaded one, twice per landing plus once per review.
In the user's words, "these rounds are taking way too long due to
long-running badly-architected commands".

## Goal

- One integration-test target per package, e.g. `tests/main.rs` declaring every
  existing test file as a `mod`. Shared fixtures load once per binary through
  the existing lazily initialised statics in
  `crates/deckmaste_english_v3/tests/common/mod.rs` (`LEXICON`, `lexicon()`,
  `readings()`, `assert_constituents()` and siblings). The other packages
  get the same shape, with a `common` module wherever they repeat setup.
- Every existing `#[test]` is kept with identical assertions. Tests are moved,
  never deleted or weakened (Assurance). The only content changes allowed are
  the ones the module structure forces: path and `use` fixes, and duplicate
  helper definitions merged into `common`. Each merge is disclosed.
- Gate derivation: `crates/xtask/src/gate.rs` maps a path to its owning
  package (`closure_for_paths`, `owner_for_path`) and runs `cargo test -p …` over the
  reverse-dependency closure (`test_command`). No package declares `[[test]]`
  or `autotests`. The derivation should need no change. The landing confirms
  that `cargo xtask gate --changed` still selects the same packages for a test-file
  path and says whether any change was needed.
- Document how to add a new test module: a new landing adds a `mod` line to
  `tests/main.rs`. Put this in `crates/deckmaste_english_v3/tests/README` or
  the `common` module's doc comment.

## Method

Standard constraints apply. Land as a series with one commit per package.
For each package, run `cargo test -p X -- --list` before and after, and check
that the two test-name lists are equal, allowing only the module-path prefix
the move adds. Clippy and fmt must be clean. Measure gate wall time before and
after on the same host state and record it as integer ns, with host load and
worker count.

## Out of scope

Changing any test's assertions; cargo-nextest or any other test-runner
tooling.

## Landing record

**PROVE — preservation and verification.** On the claimed tree `uootrxxxyroszslykkuwlonusyqsosnr`, compared with consolidation change `ttqknvrvqstvszoqnnqwvynswyultwlo` before refresh, every package's `cargo test -p PACKAGE -- --list` inventory is equal after qualifying old integration names by their former binary/module name. Multiplicities, nested names, unit tests and doctests are preserved. The normalized inventories and SHA-256 digests are in `target/test-binary-consolidation/inventory-comparison.json`.

| Package | Integration targets before / after | Integration tests before / after consolidation / after refresh | Unit tests | Doctests |
|---|---:|---:|---:|---:|
| `deckmaste_construction_v3` | 13 / 1 | 42 / 42 / 42 | 0 | 2 |
| `deckmaste_english_v3` | 64 / 1 | 361 / 361 / 365 | 0 | 0 |
| `deckmaste_lexical_source` | 9 / 1 | 32 / 32 / 32 | 25 | 0 |
| `xtask` | 2 / 1 | 13 / 13 / 13 | 373 | 0 |

The original tree had 88 integration targets (the ticket's English estimate was 63; the claimed tree had 64); the final tree has 4. All 448 original integration tests survive. Refresh inherits four additional `where` tests, for 452 final integration tests. Counts in the table are stamped with the claimed change above for before, and consolidation change `ttqknvrvqstvszoqnnqwvynswyultwlo` for its pre-refresh and refreshed states; the `covered` stamp is not applicable in each state. Assurance counts attributable to this ticket: restored 0, re-spelled 0, newly ignored 0, added 0, removed 0. One existing ignored unit test is unchanged: `macros::templates::tests::macro_schema_census_count_matches_21`, with its existing reason "cross-checks the live corpus against the census; run on demand". Existing tests are relocated; assertions are neither deleted nor weakened. The source token comparison covers 443 test bodies/templates, including one template generating six partitive tests. It normalizes import grouping and the disclosed fixture setup/borrow changes. Separate comparison preserves the entire incoming `where` module's tokens apart from its root `common` import and formatting.

The refreshed English package rerun passed all 365 tests and its doctest target. Existing exact-source roundtrip, independently constructed Reading, lexical ownership and traversal assertions remain in the package gate. This ticket adds no admission or licensing guard, changes no Production or vocabulary declaration, and causes no coverage retirement or re-coverage obligation. Production sources and declaration data match refreshed default. Corpus coverage, ambiguity, homograph, overlap, construction-count and per-byte CPU inventories were not remeasured for this test-layout change; no English coverage lock is consumed or changed by it (`covered` stamp: not applicable).

`cargo xtask gate --changed` still selects the same four packages for the owning test paths; moving below `tests/suite/` does not change path ownership. `gate.rs` and package manifests required no edits. Derived test command:

```sh
cargo test -p deckmaste_lexical_source -p deckmaste_construction_v3 -p deckmaste_english_v3 -p xtask
```

`cargo xtask gate --changed --run` passed: 851 tests passed, 0 failed, and the same 1 existing unit test ignored. The four integration targets passed 42, 365, 32 and 13 tests respectively. Citation checks report 0 stale and 0 noncompliant sites.

`cargo fmt --all -- --check` (the stable CI formatter) passed. Changed test modules additionally pass nightly formatting. An optional whole-workspace nightly check reports pre-existing differences in 157 unchanged files and zero changed test files; its evidence is `fmt.log`. `cargo clippy -p deckmaste_lexical_source -p deckmaste_construction_v3 -p deckmaste_english_v3 -p xtask --all-targets -- -D warnings` passed.

**DISCLOSE — merges and concurrent work.** Every shared-fixture merge:

- English: `generic_predicates::LEXICON`, `articles::workspace_lexicon` and `articles::lexical_source` use the existing `common::LEXICON` / `common::lexicon` fixture. The article test borrows that fixture directly; its assertion operands remain the same.
- English: `generic_predicates::ENVIRONMENT` becomes `common::ENVIRONMENT`, also reused by `common::readings` and `common::constituent_positions`. `readings_with_grammar` holds the unchanged roundtrip/deduplication assertions; `readings_with_lexicon` continues compiling the caller's custom Lexicon. The obsolete per-binary `dead_code` allowance is removed.
- Lexical source: the `LEXICON` statics in `counter_compounds`, `lexical_gaps`, `multiword_lexemes`, `next_lexical`, `singular_you` and `type_words`, plus `supertype_words`' inline workspace fixture, share `common::LEXICON`. `keyword_payload_features::SOURCES` shares `common::SOURCES`; Lexicon construction clones that already-loaded declaration vector, preserving its content and order.
- Construction v3's distinct synthetic grammars/Lexicons and the loader-error fixtures remain local. Xtask's helpers retain their existing behavior; no helper merge was needed there.

Kata refresh returned 69 for the anticipated modify/delete conflict at `tests/where_variable_clause.rs`, originating in English change `qtwrslvq`. Harmony resolved it in that oldest change: incoming content is preserved at `tests/suite/where_variable_clause.rs`, the old target is removed, and `tests/main.rs` retains its module registration. Refresh then returned 0, the conflict revset was empty, and the English package tests were rerun successfully. The incoming `host` → `anchor` helper-field update and these four tests belong to `english-v3-where-followup`, rather than this consolidation:

- `coordinated_definitions_retain_general_coordination`
- `independently_composed_medial_supplement_retains_its_anchor_and_continuation`
- `kraul_harpooner_keeps_sequencing_outside_the_medial_definition`
- `sardian_cliffstomper_keeps_coordination_in_the_anchor_condition`

No original `where` test body changed and none disappeared. `where.refresh-comparison.json`, `refreshed-inventory.json` and `where.incoming.rs` retain that evidence. Deviations/additions: CI's existing `--test lean_check` invocation now selects `--test main lean_check::`, retaining exactly those seven tests; this target-path fix is required by consolidation. The requested `tests/README` explains adding and registering modules. No Construction or test was added/deleted by this ticket, no ruling STOP was needed, and no glossary gap arose.

**REPORT — performance advisory and provenance.** Measurements use `time.monotonic_ns()` on the same 24-CPU host, with default Cargo/test/Rayon worker settings (24 default workers), ordinary profiles, and warmed Cargo/Lean caches. The initial combined feature-set rebuild and Lean warm-up are retained separately in `gate.before.warmup.*`; after compilation is isolated with `--no-run`. These timings are advisory: host load changed during concurrent work, and the after tree incorporates the incoming grammar/lexical changes above, so the difference is not an isolated causal benchmark of this ticket.

| Measurement / change ID (`covered`: not applicable) | Gate wall time | Host load before (1 / 5 / 15 min) | Host load after (1 / 5 / 15 min) | Default workers |
|---|---:|---|---|---:|
| Before, `uootrxxxyroszslykkuwlonusyqsosnr` | 322,190,786,440 ns | 5.84 / 11.28 / 17.71 | 11.72 / 11.28 / 15.66 | 24 |
| After, `ttqknvrvqstvszoqnnqwvynswyultwlo` | 194,703,838,209 ns | 18.78 / 14.77 / 13.62 | 10.54 / 13.39 / 13.36 | 24 |

All evidence, measurement scripts and source snapshots were written under this workspace's ignored `target/test-binary-consolidation/`, never `/tmp` and never a tracked source directory. The evidence directory will be retained in default's ignored `target/test-binary-consolidation/` when retiring the feature workspace.
