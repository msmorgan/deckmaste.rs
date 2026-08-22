---
needs: [english-v2-plan-05-structural-declarations]
---
**Complete and integrate Stage 5 Plan 06: guarded forms, generated selection
authority, and empty Oracle documents.** This plan-boundary completion ticket
carries only the reviewed Plan 06 work and must land before Plan 07
implementation starts.

The completed slice adds finite guarded multi-form declarations with a final
`otherwise` complement, compiler-proven satisfiability and pairwise
disjointness, and generated form-specific rule, build, render, traversal, and
diagnostic authority without stored form tags. `Demonstrative` now declares
`that` and `those` as guarded forms. The Plan 05 defect correction admits an
empty document only at the `OracleText` root; all 356 normalized empty-text
faces render to zero bytes with zero claims, while non-root sequence minima
remain intact. The declaration-language boundary is recorded in
`docs/decisions/english-v2-rewrite.md`.

Completion evidence on the task-closed tree:

- construction core: 258 unit tests passed;
- construction proc macro: all 42 compile-fail fixtures and 6
  compiled-consumer tests passed;
- English v2: 124 library tests, 66 integration tests, and 2 doc tests passed;
- xtask: 411 library tests passed, 1 ignored, and 11 CLI tests passed;
- strict four-crate Clippy and formatting gates passed;
- the normalized corpus is exactly 32,641 total / 412 selected, covered,
  clean, byte-exact, and totally owned / 32,229 ordinary failures, including
  exactly 356 selected-covered empty documents; every selected-uncovered,
  unresolved-tie, internal-failure, round-trip-mismatch, ownership-failure,
  gap, overlap, synthetic-claim, provenance-plan-mismatch,
  selection-exception-inventory, exception-use, and stored-form-tag counter is
  zero;
- the seven separate elapsed samples were expand 0.22s, report 0.18s, parse
  8.95s, roundtrip 8.29s, ambiguity 9.00s, coverage 9.05s, and
  require-complete 8.03s. Every sample stayed at or below 16.26 seconds and
  below its named warning threshold, so no `WARNING` line was emitted. The
  require-complete probe reported the expected 412-of-32,641 Stage 5 boundary.

Standard constraints apply.
