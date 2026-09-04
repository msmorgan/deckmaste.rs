---
needs: [english-v2-corpus-wide-visitor-traversal-property]
---
Extend the corpus-wide traversal property to lexeme leaves (visitor-traversal
landing review MEDIUM-1). Today the coverage gate proves every selected unit's
visitor enters exactly its ordered construction path, but leaf visits are
unobserved: a generated visitor that drops every lexeme/terminal leaf visit
(`emit/visit.rs` leaf arm returning `None`) leaves `coverage --check` green.
The materialized derivation already knows its leaves in surface order; record
the visitor's leaf visits alongside `enter_construction` and require ordered
equality with the derivation's leaves (or the surface token sequence) per
selected unit, reported in the coverage report as visited/expected leaf counts
with per-unit failure evidence. Mutation-verify: the leaf-drop mutation must
fail the gate. Zero grammar change; lock byte-unchanged. Gate scope: emit/
change → `cargo test --workspace`. Standard constraints apply.

Also (literal-opacity landing review LOW-1, same `emit/` directory): add a
short comment at the `Bound`/`Circumfix` arms of `emit/final_constituent.rs`
stating that the fold deliberately looks through delimiters — the predicate is
"rightmost non-delimiter constituent" — and that `quoted_ability` and the two
quote-terminator `checked by` sites depend on it. Facts only; no essay.

## Landing record (2026-09-04)

Measured on change `tlpzkynnuqmzkqvtrolkswylkmvqluus` with 16,771 covered
lock identities. The measured lock has 49,421 lines and SHA-256
`2cf7f9b716e13b27d626c60638d1266ca6cdc083bbcca87cb1435b4b00cd65ca`.

| gate | refreshed parent | measured tree |
| --- | ---: | ---: |
| selected and covered units | 16,771 | 16,771 |
| ordinary parse failures | 15,870 | 15,870 |
| unique selections | 11,515 | 11,515 |
| specificity-resolved selections | 5,256 | 5,256 |
| unresolved ties | 0 | 0 |
| construction declarations | 397 | 397 |
| coverage-lock identities | 16,771 | 16,771 |
| expected / visited construction entries | 731,469 / 731,469 | 731,469 / 731,469 |
| expected / visited lexeme leaves | not reported | 233,679 / 233,679 |

- Property implementation: each materialized derivation now retains its
  ordered, non-literal terminal Lexeme classes in surface order. The generated
  visitor reports each such leaf through the default no-op `enter_leaf` hook
  immediately before its typed callback, and the coverage gate requires the
  selected derivation's exact sequence to equal the visitor's sequence for
  every selected unit. The coverage report carries per-unit expected and
  visited leaf counts, aggregate counts, and — only on a mismatch — the two
  full sequences as failure evidence.
- Coverage and selection census: selected-uncovered units, leaf traversal
  failures, construction traversal failures, internal failures, exception
  resolutions and uses, round-trip mismatches, ownership failures, gaps,
  overlaps, synthetic claims, and provenance-plan mismatches are all zero.
  Literal / lexicon collisions remain 59. There are no newly covered
  identities and no changed selected analyses; unique and
  specificity-resolved selections are byte-for-byte census-equivalent to the
  refreshed parent.
- Coverage lock: byte-unchanged. Its source fingerprint is
  `e85359d7b8c578df13dff2fdf7c743a520a5b367d5ed25ab0a5f03cb8b3637dd`
  and its normalization digest is
  `f3a2fccd079f0bc53c79b4c23e28e0351b3cf69a5324b3893c695638935341c9`.
  No production Construction was added or removed, and no retirement manifest
  was created or used.
- Mutation verification: replacing the generated leaf hook emission with an
  empty token stream made `cargo xtask english_v2 coverage --check` exit 1:
  16,388 selected units became uncovered, expected leaves remained 233,679,
  visited leaves fell to 0, and the report showed the expected arrays against
  empty actual arrays. The mutation was immediately restored before the final
  gates.
- Positive gate artifacts after the required refresh: `cargo fmt --all --
  --check`; the exact generated-helper-cycle regression; `cargo test
  --workspace`; `cargo xtask english_v2 ambiguity --require-resolved`; and
  `cargo xtask english_v2 coverage --check` all exited zero. The workspace
  suite included 396 construction-core tests, 143 English-v2 library tests,
  and 422 xtask tests (1 ignored), plus integration and doc tests. A nonbinding
  clippy probe reached ten pre-existing `unneeded_wildcard_pattern` findings in
  refreshed-parent `deckmaste_lowering/src/card.rs`; this feature does not
  touch that file.
- Performance advisory: after the post-commit changed refresh,
  measured-tree coverage took 25.704 s at 111,764 accepted thread-CPU
  nanoseconds per byte (load 22.99 / 22.01 / 23.17), and ambiguity took 24.134
  s at 112,330 ns/B (load 21.26 / 21.85 / 23.18), with `pgrep -c -x codex`
  equal to 1 before both. Direct refreshed-parent coverage took 23.275 s at
  106,088 ns/B (load 12.75 / 19.54 / 22.69) with 4 Codex processes, and
  ambiguity took 27.644 s at 125,567 ns/B (load 21.63 / 20.95 / 23.03) with 3.
  All four used 24 workers and exceeded the 16.26 s quiet-host ceiling under
  elevated load or concurrent Codex activity; the advisory fired and both
  gates remained green.
- Assurance census: restored 0; re-spelled 0; ignored with blockers 0; added 0
  standalone test functions; removed 0. Six existing test functions were
  extended: three emitter fixtures and three coverage/report fixtures. An
  existing exact cycle regression caught a transient RHS-indexing error during
  implementation; indexed lookup now preserves malformed-family rejection.
- Deviations and additions: no grammar, selection policy, Construction, or
  lock change. Additions are the requested traversal evidence, report schema
  7, and the requested short delimiter-transparency comment. `open
  declaration` uses its declared semantic terminal label; no identity-specific
  guard was added.
- Glossary gap: **terminal leaf traversal** — the ordered visits to a
  derivation's non-literal terminal Lexemes in surface order. Existing Oracle
  English glossary entries define Construction, Constituent, and Lexeme, but
  do not name this evidence stream.
- STOPs: none. One refresh was delayed by a sibling workspace's divergent
  change; it was left untouched and the later refresh succeeded. The required
  post-commit refresh rewrote the stack, so the parent and measured tree were
  both remeasured and this record was updated. Coverage did not drop, no tie
  appeared, and no newly covered negative or wrong parse was admitted.

### Review corrections

Reviewed and re-measured on change `tmqswuknrvrrxqmzxpuyqswsqvxwstpk`, after
three refreshes that landed `core-entity-object-classes`,
`english-v2-licensing-checker-count`, `english-v2-closed-class-single-owner`
and `core: one candidate domain per predicate` under this stack. The
implementer's figures above were measured three refreshes earlier; the
re-measured figures below supersede them.

| gate | re-measured tree |
| --- | ---: |
| selected and covered units | 16,771 |
| selected uncovered units | 0 |
| ordinary parse failures | 15,870 |
| unique selections | 11,515 |
| specificity-resolved selections | 5,256 |
| unresolved ties | 0 |
| construction declarations | 395 |
| licensed vocab/lexicon homographs | 2 |
| form-literal vocab overlaps | 25 |
| expected / visited construction entries | 731,482 / 731,482 |
| expected / visited lexeme leaves | 235,683 / 235,683 |
| leaf traversal failure units | 0 |

The construction count (397 → 395), the construction entries (731,469 →
731,482) and the leaf count (233,679 → 235,683) all moved with the refreshed
base, not with this landing: `english-v2-closed-class-single-owner` retired two
constructions and moved 2,002 claims from form literals to vocabulary
(`form_literal_claims` 125,468 → 123,464, `vocab_claims` 52,287 → 54,289), and
a vocabulary claim is a non-literal terminal, so it counts as a leaf where a
form literal did not. It also replaced the single `literal_lexicon_collisions`
counter (59) with the two exact counters above. Selected and covered units,
the census, and the lock are unchanged across every refresh.

Coverage lock byte-unchanged: 49,421 lines, SHA-256
`2cf7f9b716e13b27d626c60638d1266ca6cdc083bbcca87cb1435b4b00cd65ca`, source
fingerprint `e85359d7b8c578df13dff2fdf7c743a520a5b367d5ed25ab0a5f03cb8b3637dd`,
normalization digest
`f3a2fccd079f0bc53c79b4c23e28e0351b3cf69a5324b3893c695638935341c9`.
No identity newly covered, none retired, no census shift.

Findings and fixes:

- Strict clippy on `xtask` was red: the added leaf fields pushed
  `CoverageSummary::from_rows` to 164 lines against the 150-line
  `too_many_lines` limit, and the landing reported only a nonbinding probe of
  another crate. Fixed by splitting the per-row accumulation into
  `accumulate_traversal` and `accumulate_ownership`.
- The new report-schema assertion compared the emitted field against
  `REPORT_SCHEMA_VERSION` itself, so a version bump could not fail it. Fixed
  by pinning the literal.
- `emit/visit.rs` renders leaf labels through a second derivation that must
  agree with the emitted `TerminalClass::label` arms in `emit/runtime.rs`;
  nothing said so. Fixed with a three-line note at `terminal_label`. The two
  derivations are deliberately kept separate: the property's expected sequence
  comes from the materialized derivation's rule positions, and a shared
  renderer would not weaken that, but an undocumented pair of label sources
  would.
- Report schema collision: `english-v2-licensing-checker-count` and then
  `english-v2-closed-class-single-owner` each integrated a bump while this
  stack was parked, leaving trunk at 8 against this stack's own 8. Reconciled
  to a single monotonic schema 9 carrying all three landings' summary fields —
  `licensing_checker_permitted` / `licensing_checker_forbidden`,
  `licensed_vocab_lexicon_homographs` / `form_literal_vocab_overlaps`, and
  `expected_leaves` / `visited_leaves` / `leaf_traversal_failure_units` — with
  the version pin updated.
- Record wording: the per-unit rows carry leaf counts, not the leaf sequences;
  the sequences appear only inside the mismatch evidence. Corrected above.

Performance advisory (measured on the integrated tree): coverage 21.627 s at
103,958 ns/B (load 11.08 / 18.94 / 23.27); ambiguity 23.333 s at 112,537 ns/B
(load 11.63 / 18.36 / 22.94); both at 24 workers, both over the 16.26 s
quiet-host ceiling, both green. Contention stamp: 4-6 concurrent codex
executors on the host across every measurement in this review (`pgrep -c -x
codex` read 6 from inside the sandbox). The landing record's "pgrep = 1"
figures are the sandbox's view of the host and understate it; its "4 Codex
processes" / "3" figures are closer. Contention dominates the per-byte figure
at this load: the same tree measured 183,421 ns/B for coverage six minutes
earlier under load 35, so nothing here supports a per-byte conclusion about
this landing's own cost.

Assurance census (review): restored 0; re-spelled 0; ignored with blockers 0;
added 0; removed 0. Two existing coverage tests were adjusted (the schema pin
literal, and the split accumulators' fixtures are unchanged in what they
assert).

### STOP: red workspace suite on the refreshed base — resolved

`cargo test --workspace` — the gate this ticket's `emit/` change requires —
failed on an earlier refreshed base, before any commit in this stack:
`deckmaste_migrations` `resolve::tests::ascend_gate_const_matches_canonical_condition`
panicked with "ASCEND_GATE drifted from the canonical Ascend gate", the const
carrying `Candidate(Object)` where the canonical builder produced
`Candidate(Entity)`. Verified by running the test on the claim commit with none
of this stack applied: it failed identically. The drift belonged to the
already-integrated `core-entity-object-classes` landing; no commit here touches
`deckmaste_migrations` or `deckmaste_semantics`. The landing was held rather
than integrated onto a red workspace suite, and the question was routed to the
coordinator.

Resolved outside this ticket: the default line now carries `core: one
candidate domain per predicate`, which fixes the drift. On the refreshed base
`cargo test --workspace` exits 0 — 127 test targets ok, 6,087 tests passed, 0
failed, 3 ignored with their existing blockers.

Separately, `deckmaste_lowering` remains clippy-red on the refreshed base with
ten pre-existing `unneeded_wildcard_pattern` findings in `src/card.rs`; not
this ticket's file, not fixed here.
