---
needs: []
---
**Publish the sealed `Feature` name inventory from `construction_core`.** The
English-v2 licensing-checker census in
`crates/xtask/src/english_v2/licensing_checkers.rs` decides whether a `checked
by` body reads a declared licence feature by matching identifiers against a
hand-copied `FEATURE_NAMES` string list. Those are exactly the variants
of `deckmaste_construction_core`'s sealed `Feature`
(`crates/deckmaste_construction_core/src/model.rs`), whose authoritative key
strings live in `crates/deckmaste_construction_core/src/feature.rs` behind
`pub(crate) fn key`. A newly added feature will not be recognised and its
readers will silently demote from `DeclaredLicenseFeature` to
`StructuralPredicate` — a split every landing record now reports.

2026-09-04: stale counts removed — was: "a hand-copied `FEATURE_NAMES` list of
21 strings" and "A twenty-second feature will not be recognised". The list is
already at 22 entries, which is the drift this ticket exists to end.

Pinned shape: `deckmaste_construction_core` exposes the sealed feature key
inventory as public API (the model type's own keys, not a second copy), and
xtask's classifier consumes it. Delete `FEATURE_NAMES`.

Fence: a second hand-written copy of the feature names anywhere.

Acceptance: `FEATURE_NAMES` is gone, the census's permitted split is unchanged
on the current grammar, and adding a `Feature` variant is picked up without an
xtask edit. Gate on `cargo test --workspace` — this is an emit-contract
surface. Standard constraints apply.

## Landing record

Measured on `vtqykwww` (`b77de2d7`, the reviewed tree over trunk
`ruyxpvnwrqwq`); lock `covered` 17,601 on that tree. Trunk advanced to
`rzuuowlxmuxz` during the corpus gates (the flavor-word override closure plus
Idris and ticket files); that diff touches no grammar declaration, no
`plugins/builtin_v2` stub and not the coverage lock, so it cannot reach a
corpus figure. After the second `kata refresh` only the gates it can reach
were re-run (xtask clippy and `cargo test -p xtask`).

| measure | before | after |
|---|---|---|
| corpus units | 32,641 | 32,641 |
| selected / covered | 17,601 / 17,601 | 17,601 / 17,601 |
| lock state | 17,601 covered, current | 17,601 covered, `+0 / -0`, byte-unchanged |
| selection census | unique 13,759 / specificity-resolved 3,842 | unique 13,759 / specificity-resolved 3,842 |
| unresolved ties | 0 | 0 |
| `licensing_checker_permitted` | 20 | 20 |
| `licensing_checker_forbidden` | 0 | 0 |
| construction declarations | 393 | 393 |
| published feature keys | n/a (22 hand-copied in xtask) | 22, generated from the sealed enum |

Newly covered identities: none (`newly covered` 0, `selected_uncovered_units`
0). Construction declarations counted as `construction <name>` sites under
`crates/deckmaste_english_v2/src`; the diff touches no declaration file, so
the delta is 0 by construction.

Gate artifacts (all foreground, `--workers 8`,
`DECKMASTE_COVERAGE_LOCK=report`):

```
cargo fmt --all                                     no changes
cargo clippy -p deckmaste_construction_core --all-targets -- -D warnings
                                                    Finished, 0 warnings
cargo clippy -p xtask --all-targets -- -D warnings   Finished, 0 warnings
cargo test --workspace                              all suites ok, 0 failed
cargo test -p xtask (post-refresh)                  459 passed; 0 failed; 1 ignored
coverage --check   summary selected_units=17601 covered_units=17601
                   selected_uncovered_units=0 unresolved_ties=0
                   roundtrip_mismatch_units=0 ownership_failure_units=0
                   licensing_checker_permitted=20 licensing_checker_forbidden=0
ambiguity --require-resolved
                   summary selected=17601 unique=13759
                   specificity_resolved=3842 unresolved_ties=0
roundtrip          exit 0, 17,601 accepted units
```

The one ignored xtask test is inherited and unrelated. No citation changed;
`cite check --list-noncompliant` is empty and `cite check` reports 0 stale.

### Review corrections

- **HIGH — the inventory was a second hand-written list.** `Feature::ALL` was
  a 22-line array typed out beside the sealed enum with no exhaustiveness
  check, so a new variant could still miss it silently: the drift moved from
  xtask into `construction_core` rather than ending, against this ticket's own
  fence. Fixed by generating the enum, `ALL` and `key` from one
  `feature_inventory!` table (the house idiom of `deckmaste_features`'
  `feature_inventory!` and `deckmaste_english`'s `vocabulary!`), so an
  inventory entry is the variant declaration. Falsified: adding a variant to
  the public `model::Feature` fails to compile at `feature.rs`'s
  `impl From<model::Feature> for Feature` (E0004) before it can reach the
  inventory, and the table then carries it into `feature_keys` with no further
  edit.
- **MEDIUM — the new test was circular.** `is_declared_feature_identifier(k)`
  for every `k` drawn from `feature_keys()` reduces to
  `to_snake_case(k) == k`, which holds for any inventory at all. Re-spelled as
  `classifier_reads_every_published_feature_key_as_a_declared_licence`, which
  runs the production `classify_body` path on a `role.<feature>` body per
  published key. Falsified: with the inventory read removed from the
  classifier it fails naming `concord_class`.
- **MEDIUM — no measurements.** The implementer's record was a refresh-blocker
  note; the sibling divergence was converged by the coordinator, `kata refresh`
  succeeded, and every number above was measured by the reviewer.
- **LOW — a third copy of the key strings.** `#[cfg(test)] Feature::snapshot`
  was a byte-identical hand copy of all 22 keys; it now delegates to `key`.

### Deviations and additions

- Generating the enum/`ALL`/`key` from one table goes beyond the ticket's
  letter ("expose the inventory"), and is what makes its acceptance clause
  ("adding a `Feature` variant is picked up") true rather than conventional.
- `Feature::snapshot` collapsed to `self.key()` (test-only, byte-identical
  output).
- No construction, form, frame, require or vocabulary was added, changed or
  deleted; no grammar file is in the diff.

Assurance: restored 0, re-spelled 1 (the circular inventory test), ignored 0,
added 1 (the implementer's test, kept as the re-spelled one), removed 0.

STOPs: none warranted, none taken. The implementer's recorded refresh blocker
was environmental, not a ruling question, and is resolved. No guard naming a
lexeme, construction, verb, noun, preposition or card exists in the diff; the
classifier reads a declared feature key and nothing else.

Glossary gap: none.

Performance advisory: coverage 222s wall against the 16.26s quiet-host
ceiling, 220,083 ns/B thread CPU; ambiguity 153s, 172,428 ns/B; roundtrip
192s, 179,813 ns/B. All three ran `criterion=capped_workers` with
`host_load_1m` between 25 and 39. Contention stamp: six executors live during
the gates (`english-v2-role-preemption-depth`, the construction_core phase-1
ticket, E15 and three terra tickets) plus this review, so the wall figures are
contention-bound and carry no parse-time signal.
