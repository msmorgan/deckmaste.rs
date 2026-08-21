---
needs: [english-v2-plan-04-generated-invariants]
---
**Complete and integrate Stage 5 Plan 05: structural declarations and the
`OracleText` root.** This plan-boundary completion ticket carries only the
reviewed Plan 05 work and must land before Plan 06 implementation starts.

The completed slice adds generated structural products, sums, and sequences;
distinguishes positional separators from owned terminators; derives
`CasePosition`; makes `OracleText` the parser's production document root; and
threads structural ownership, diagnostics, rendering, visiting, ambiguity
selection, reporting, and coverage through the generated declaration
authority. The declaration-language amendment is recorded in
`docs/decisions/english-v2-rewrite.md`.

Completion evidence on the final reviewed tree:

- construction-core: 242 unit tests passed;
- construction proc macro: all 42 compile-fail fixtures and 5 compiled-consumer
  tests passed;
- English v2: all 124 library tests, integration tests, and doc tests passed;
- xtask: 409 library tests passed, 1 ignored, and 11 CLI tests passed;
- strict four-crate Clippy and formatting gates passed;
- the normalized corpus is exactly 32,285 total / 56 accepted, selected,
  covered, clean, byte-exact, and totally owned / 32,229 ordinary failures,
  with every ambiguity, internal, round-trip, ownership, gap, overlap,
  exception-use, synthetic-claim, and provenance-mismatch counter at zero;
- all seven timed gates stayed at or below 16.26 seconds and below the named
  1.5x warning threshold for their Plan 04 samples.

The final independent review found no Critical or Important findings. Standard
constraints apply.
