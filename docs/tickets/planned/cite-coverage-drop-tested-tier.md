---
needs: []
---
**Retire the "Tested" tier of the CR-citation coverage ratchet.** Owner
decision, 2026-10-25. `tier_for_path` in `crates/xtask/src/coverage.rs` assigns
the tier purely by file path: any citation in a file under `/tests/`, `/Proofs/`
or `plugins/testing/` is "Tested". That measures only that a rule number appears
somewhere in a test file, not that anything tests the rule. In practice it
blocked honest retirements of pins and rewarded moving citation numbers between
pins. After this ticket the ratchet has two tiers, **Mentioned** and **Bound**,
and one floor (Bound).

Kept unchanged: `cargo xtask cite check` (staleness), `cite check
--list-noncompliant`, `cite audit --diff`, the Bound/Mentioned distinction, the
`--check`/`--bless` flags, `coverage.out_of_scope` handling, the scan universe
(`EXTENSIONS`, `SKIP_DIRS`, `sources.exclude`). Change nothing else. Do not edit
`cite.rs`, `cite-config.json`, or any file under `docs/memory/` (the owner
updates those).

Standard constraints apply, plus the deltas below.

## 1. Code change: `crates/xtask/src/coverage.rs`

1. Module doc and `Tier` doc: `Tier` becomes
   ```rust
   /// Binding strength of a citation site. Ordered `Mentioned < Bound`.
   #[derive(Debug, Clone, Copy, PartialEq, Eq, Ord, PartialOrd)]
   pub enum Tier {
       Mentioned,
       Bound,
   }
   ```
   (`Tested` variant deleted; keep the `Ord` derive, `scan_tree` uses `>`.)
2. `tier_for_path` becomes (keep the existing `#[must_use]` and
   `#[expect(clippy::case_sensitive_file_extension_comparisons, ...)]`, which
   still applies because of `ends_with(".md")`):
   ```rust
   /// Classify a repo-relative path into its citation tier: prose (anything
   /// under `docs/`, any `.md`) is `Mentioned`; every other scanned file,
   /// test files included, is `Bound`.
   pub fn tier_for_path(rel: &str) -> Tier {
       if rel.starts_with("docs/") || rel.ends_with(".md") {
           Tier::Mentioned
       } else {
           Tier::Bound
       }
   }
   ```
   The `is_test` computation and the `/tests/`, `/Proofs/`, `plugins/testing/`
   literals are deleted.
3. `Lock` becomes a single set. Rename the field `strong` to `bound` and delete
   `tested`:
   ```rust
   /// The committed coverage floor. A set, not a count: each named rule is
   /// pinned at Bound and may not regress.
   #[derive(Debug, Default, Deserialize, Serialize)]
   pub struct Lock {
       pub bound: Vec<String>,
   }
   ```
4. `lock_from_scan`: doc becomes "Collect the ratcheted rules (those cited at
   `Bound`)". Body: `bound` = scan entries with `t >= Tier::Bound`, sorted,
   deduped. No `tested` vector.
5. `check_ratchet`: only the `bound` loop remains. The regression message is
   `format!("{rule}: lost Bound coverage")` (was `lost Strong coverage`). The
   `lost Tested coverage` message is deleted.
6. `run`:
   - `--bless` prints `blessed cr-coverage.lock: {N} bound`.
   - `--check` success prints `coverage ratchet OK: {N} bound (>= floor)`.
   - Failure output is unchanged (`coverage REGRESSION: <line>` per line, then
     the `N coverage regression(s); add citations back or justify + re-bless`
     error).
7. Report (`report` and `ChapterCounts`):
   - `ChapterCounts` has fields `extant` and `bound` only.
   - Summary lines become exactly the header line (unchanged) plus one line:
     `  Bound (cited in code or tests): {bound:>5}  {pct:>3}%` where `bound` is
     the number of in-scope rules whose tier is `Tier::Bound`. The `Strong` and
     `Tested (gold)` lines are deleted.
   - Per-chapter rows print `  {label:<32} {bound}/{extant}  ({pct}%)`, i.e.
     keep the existing `{strong_ratio:>9} ({:>3}%)` formatting for the bound
     ratio and delete the trailing `{tested} ({pct}%)` column.
   - The chapter loop matches only `Some(Tier::Bound) => counts.bound += 1`.
8. Any remaining `strong`/`Strong`/`tested`/`Tested` identifier, comment or
   string in `coverage.rs` is removed or renamed to `bound`/`Bound`. Update the
   `cite.rs`-facing doc only if it says Tested or Strong (it currently does not;
   leave `cite.rs` alone).

## 2. Lock file migration: `cr-coverage.lock`

The old `strong` set (Tested ∪ Bound) is exactly the new `bound` set, because
test-file citations now classify as Bound. So regeneration must change **only**
the structure, not the rule list:

- Run `cargo xtask cite coverage --bless` from the workspace after the code
  change.
- The committed lock afterwards is a single-key JSON object,
  `{ "bound": [ ... ] }`, pretty-printed by `serde_json::to_string_pretty` with a
  trailing newline. The `tested` key and its whole array are gone. The `bound`
  array is byte-identical (same entries, same order) to the pre-change `strong`
  array. It still contains the existing odd first entry beginning
  `")\n        .and_then(|rest| rest.strip_suffix('`; do not touch or "fix" it.
- Verify with `jj diff --git -- cr-coverage.lock`: the only changes are deletion
  of the `"tested": [...]` array and the rename of `"strong"` to `"bound"`; no
  rule line is added or removed from the retained array. Cross-check:
  `jj file show -r @- cr-coverage.lock | jq -c '.strong' | sha256sum` equals
  `jq -c '.bound' cr-coverage.lock | sha256sum`.
- If a rule line does differ, the tree has drifted from the old lock; stop and
  report instead of blessing it through.

## 3. xtask tests (`#[cfg(test)]` in `coverage.rs`)

Retained unchanged: `brackets_*`, `attrs_extract_quoted_rules`,
`scan_tree_skips_local_workspaces`, `out_of_scope_*`, `tier_docs_nonmd_is_mentioned`.

Rewritten (same intent minus the Tested tier). R1, R2, R3, R4 below stand for
the fake rule-number literals the existing tests already use (R1 the first,
R2 the second, and so on); keep those literals in the test source.

- `tier_by_path`: expect `Bound` for `crates/deckmaste_engine/tests/activate.rs`,
  `plugins/testing/cards/Foo.ron`, `lean/Proofs/Foo.lean`,
  `crates/deckmaste_engine/src/step/mod.rs`,
  `plugins/builtin/rules/sba/toughness-zero.ron`, `idris/src/Core.idr`;
  `Mentioned` for `docs/tickets/done/foo.md`, `README.md`, and
  `crates/e/tests/notes.md` (markdown in a test dir is still prose). Reason:
  the old assertions pinned Tested for test paths, which is the retired
  behaviour.
- `tier_ordering`: assert only `Tier::Bound > Tier::Mentioned`.
- `scan_tree_strongest_tier_wins` is renamed
  `scan_tree_bound_beats_mentioned`. Fixture: `crates/e/src/a.rs` cites R1,
  `crates/e/tests/a.rs` cites R1, `docs/x.md` cites R2,
  `crates/e/tests/b.rs` has `#[cr("R3")]`. Expect `R1 -> Bound`,
  `R2 -> Mentioned`, `R3 -> Bound` (an attribute in a test file is Bound).
- `lock_from_scan_partitions_tiers` is renamed `lock_from_scan_keeps_bound_only`.
  Scan: R1 Bound, R2 Bound, R3 Mentioned. Expect
  `lock.bound == ["R1", "R2"]` (sorted, Mentioned excluded). Also assert
  that `serde_json::to_value(&lock)` is an object whose only key is `bound`.
- `ratchet_passes_on_superset`: baseline `bound = [R1, R2]`, current
  `bound = [R1, R2, R4]`; expect no regressions.
- `ratchet_fails_and_names_regression`: baseline `bound = [R1, R2]`,
  current `bound = [R1]`; expect exactly one regression, containing `R2`
  and `lost Bound coverage`.
- `ratchet_end_to_end_over_temp_tree`: fixture `crates/e/tests/a.rs` citing
  R1 and `crates/e/src/b.rs` citing R2; expect
  `lock.bound == ["R1", "R2"]`. Then overwrite the test file with
  `// no cite`, rescan, and expect a regression naming `R1` and
  `lost Bound coverage` (a test-file citation is now an ordinary Bound pin, and
  removing it is a regression like any other, which is the intended behaviour).

Retired: none outright; the Tested-specific assertions inside the tests above
are removed because the tier no longer exists.

No test of `run`/`report` exists today; add none.

## 4. Docs and config lines

Only these places mention the tier or the gate. A repo-wide grep found no
mention of "Tested"/"Strong" coverage in `lean/CONTRACTS.md`, any README,
`docs/` outside `docs/tickets/done/` and `docs/archive/` (frozen, do not edit),
`scripts/`, or ticket templates.

1. `CLAUDE.md` line 28 (`AGENTS.md` is a symlink to it; edit only `CLAUDE.md`).
   The sentence
   still makes sense, so only name the lock and its floor. Replace
   `the coverage lock is the only tracked gate authority.` with
   `the citation-coverage lock (`cr-coverage.lock`, a Bound floor) is the only
   tracked gate authority.` Keep the line wrapping within the existing column
   width. The meaning of the rule is unchanged.
2. `.github/workflows/ci.yml` lines 132-133. Replace the comment
   ```
   # The coverage ratchet self-scans committed sources and compares them to
   # the committed cr-coverage.lock.
   ```
   with
   ```
   # The coverage ratchet self-scans committed sources and compares the set
   # of Bound rules to the floor in the committed cr-coverage.lock.
   ```
   Step name and command (`cargo xtask cite coverage --check`) stay.
3. No other line changes. `docs/tickets/planned/idris-retirement.md` mentions
   "the coverage ratchet" generically and stays correct.

### Memory files (owner updates; do not edit)

- `docs/memory/cite-coverage-in-briefs.md`: add that since 2026-10-25 the
  ratchet has only the Bound floor and a regression message reads
  `lost Bound coverage`; the re-bless workflow is unchanged.
- `docs/memory/MEMORY.md` line 34 (index entry for the above): optionally
  mention "Bound floor only".
- `docs/memory/measurements-are-not-goals.md`: optionally record the
  2026-10-25 decision as an instance (a path-derived tier measured presence,
  not testing).

## 5. Acceptance checks

Run from `.workspaces/cite-coverage-drop-tested`; all must pass:

- `cargo test -p xtask`
- `cargo clippy -p xtask --tests -- -D warnings`
- `cargo xtask cite coverage --check` (prints `coverage ratchet OK: N bound
  (>= floor)`, run after the bless)
- `cargo xtask cite check`, and `cargo xtask cite check --list-noncompliant`
  empty
- No remaining tier references:
  `rg -n -i 'Tier::Tested|"tested"|lost (tested|strong)|tested ∪|tested \(gold\)|Strong \(' --glob '!docs/tickets/**' --glob '!docs/archive/**' --glob '!target/**' .`
  prints nothing, and `rg -n -i 'tested|strong' crates/xtask/src/coverage.rs`
  prints nothing.
- `jq 'keys' cr-coverage.lock` prints `["bound"]`.

## 6. Commit plan

Two commits (jj, no trailers):

1. `xtask: drop the Tested coverage tier` — `coverage.rs` (code and tests) and
   the regenerated `cr-coverage.lock`.
2. `docs: name the Bound floor in the coverage gate wording` — `CLAUDE.md`
   (which `AGENTS.md` symlinks to) and `.github/workflows/ci.yml`.
