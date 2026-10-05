---
needs: []
---
# Share one Reading test helper with a constituent assertion

Give the `deckmaste_english_v3` integration tests a single shared support module
and add an assertion that states an intended analysis without spelling a full
`Reading` value.

Two facts motivate it. The function `readings(text, category)` (parse, check each
Reading realizes back to the text, return the set) is copy-pasted into 33 of the
37 test files, each with its own `static LEXICON`. And nearly every test asserts
a hand-built `Reading` value, so a change to how a construction is represented
means re-spelling expectations across the suite; the archived attempt at
`english-v3-generic-frame-consumption` rewrote about fifty test files that way,
which is exactly when a wrong expectation slips in unnoticed.

Pinned shape:

- one support module (`tests/common/`) owning the lexicon and `readings`; the 33
  copies are replaced by it. Existing assertions are not rewritten.
- an assertion of the form "some Reading of TEXT at CATEGORY contains a
  Constituent of Category C spanning exactly SUBSTRING", taking several such
  constituents at once, all required of the same Reading. It names Categories
  and source text only: no construction, form index, frame choice or field
  layout. Example: the Noun Phrase "sources of the color of your choice" with
  "the color of your choice" as a Noun Phrase asserts the low Of attachment.
- where a text has two justifiable Readings the grammar is not meant to choose
  between (Seedborn Muse's ability is the user's example), the test makes two
  such assertions on the same text.

The helper is for new tests and for tests a later ticket must re-spell anyway.
It does not replace value-equality tests where the exact value is the point, and
it is not a fixture: no data file, no expectations outside ordinary test
functions, nothing generated from parser output.

Acceptance: the shared module in use by every test file that had a copy; the
constituent assertion with tests of its own, including one that fails for the
right reason (a span that is not a constituent in any Reading); no test removed
or weakened. Standard constraints apply.

## Landing record

Change `pouqklpm` (test support only).

**PROVE.** All 33 local `readings` functions now import shared entry points from
`tests/common/`. The shared parser checks byte-exact realization for every
Reading and retains the duplicate-Reading assertion previously in `grammar.rs`.
The custom lexical environment in `head_primitives.rs` remains independently
constructed and is passed to the shared parser. A token comparison against the
claim tree found all 310 remaining functions in the 34 migrated files unchanged,
including all 133 existing tests there: exact value comparisons, independently
constructed values, lexical identities and traversal assertions are preserved.
The three other pre-existing integration files are unchanged. Assurance counts:
restored 0, re-spelled 0, newly ignored 0, added 6, removed 0; the integration
suite grows from 165 to 171 tests. The gate retains one pre-existing ignored
xtask test, `macros::templates::tests::macro_schema_census_count_matches_21`,
whose attribute says "cross-checks the live corpus against the census; run on
demand".

No grammar, lexical declaration, parser, compiler, source data or coverage lock
changes, so no covered identity or admitted Reading is added or lost. There are
no new admission/licensing guards. The shared lexical environment still loads
through `deckmaste_lexical_source::load_workspace`, preserving load errors.
Multiple grammatical Readings remain admitted under the current English
contract; the constituent assertion searches each complete Reading separately.

**DISCLOSE.** Six ordinary tests exercise the constituent assertion: the low
Of attachment in Prismatic Strands, both Seedborn Muse attachments, exact
capitalized constituent surfaces in Ancestral Recall, a substring crossing
constituent boundaries, the wrong Category, and incompatible spans taken from
different Readings. The three negative assertions must panic with the helper's
`no single Reading` diagnosis. Their positive witnesses establish that the text
parses. The helper compares an entire visited constituent's realization, never
parses a requested substring in isolation, and reports observed constituents
separately for each Reading on failure. Repeated substrings mean any occurrence;
this interface does not select an occurrence by offset.

Deviations and additions: `participial_uses.rs`, which has its own workspace
lexicon but no `readings` copy, imports the shared lexicon as well. Thin Ability
and Noun Phrase entry points and an explicit-lexicon entry point preserve all
existing call sites and expectations. No Constructions changed and no existing
tests were re-spelled or removed. No grammatical STOPs or glossary gaps.
Pre-existing strict Clippy diagnostics are disclosed below.

**REPORT.** This landing measures test behavior, not corpus behavior. The
coverage count, complete ambiguity census, Construction count, permitted
licensing-checker inventory, homograph inventory and form-literal/vocabulary
overlap inventory are unchanged by this tree. No corpus measurements are
claimed: the coverage-command wall-time advisory against the 16.26s ceiling and
per-byte thread-CPU telemetry were not rerun for this test-only change.

Validation: `cargo xtask gate --changed --run --clippy` derived
`cargo test -p deckmaste_construction_v3 -p deckmaste_english_v3 -p xtask`, which
passed, including the real Lean gate integration tests. The final focused run
`cargo test -p deckmaste_english_v3 --test reading_support` passed all six helper
tests. Nightly rustfmt checks pass for every changed Rust file.

The derived strict Clippy command
`cargo clippy -p deckmaste_construction_v3 -p deckmaste_english_v3 -p xtask --all-targets -- -D warnings`
failed on seven pre-existing diagnostics: five `too_many_lines` findings in
`deckmaste_construction_v3_core` (`emit.rs::ast`, `ir.rs::validate`,
`ir.rs::equations`, `parse.rs::Declaration::parse`, and
`parse.rs::parse_construction`), one `collapsible_if` in `ir.rs`, and one
`needless_borrows_for_generic_args` in `macro_ron/src/set.rs`. All four affected
files are byte-identical to the claim tree. No suppression of these diagnostics
or unrelated compiler change was added.

The additional changed-crate all-targets check with `--no-deps` exposed 21
inherited diagnostics in preserved test bodies (`needless_pass_by_value`,
`collapsible_if`, `cloned_ref_to_slice_refs`, `assert_is_empty`, `clone_on_copy`,
and `too_many_lines`). The unchanged-function token comparison above covers
every affected migrated function; `slice.rs` is byte-identical to the claim
tree. One new support-module attribute lacked an explicit reason; that was
fixed. Strict Clippy passes for all new support code and its tests with
`cargo clippy -p deckmaste_english_v3 --test reading_support --no-deps -- -D warnings`.

### Addendum (2026-10-05): occurrence matching and three attachments

Follow-up change `lnrvzkvl`. Constituent assertions now match actual parser-leaf
source positions. Unqualified repeated surfaces panic;
`assert_constituent_occurrences` selects zero-based occurrences. Three Gravedigger
tests cover both occurrences of `creature`, ambiguity rejection, and rejection
of an occurrence with the wrong Category.

The user's three-Reading ruling (2026-10-05) licenses Seedborn Muse's temporal
PP at `Untap …`, at `control` inside the Object Relative Clause, and at the
Nominal `permanents you control` (CGEL Ch. 5 §14.2, p. 446; §15, p. 454).
The active test distinguishes the first and third; nominal attachment requires
the full Nominal and exactly `you control` as the relative clause. Incompatible
Noun Phrase spans still reject combining different Readings. The missing second
attachment is owned by
[english-v3-relative-clause-adjuncts](../planned/english-v3-relative-clause-adjuncts.md);
its separate test is ignored with `blocked on english-v3-relative-clause-adjuncts`
and will merge back when that ticket lands. The Preference ticket records the
nominal attachment as last-ranked while retaining its Admission.

Counts: restored 0, re-spelled 0, added 4 (three occurrence tests and one blocked
attachment test), newly ignored 1 with the blocker above, removed 0. All 6
original helper test subjects remain, with the Seedborn test corrected and
tightened. The helper suite has 9 passing tests and 1 ignored; the English
integration suite has 175 tests (174 passing, 1 ignored). Grammar, lexicon,
compiler, coverage lock and other existing test files are unchanged; no admitted
Reading identities change.

`cargo xtask gate --changed --run --clippy` passed in full before and after
refresh. The refreshed derived test command was
`cargo test -p deckmaste_construction_v3 -p deckmaste_english_v3 -p xtask`;
the same package scope passed strict Clippy with `--all-targets -- -D warnings`.
The gate also retains its one pre-existing ignored xtask test. Nightly rustfmt
and `kata kanban check` pass. Citation checks report 0 non-compliant strings
and 0 stale citations; the piped diff audit has no CR citation sites to audit.
