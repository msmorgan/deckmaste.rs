---
needs: [english-v2-open-grammar-environment, english-v2-lexeme-morphology]
---
**Complete and integrate Stage 5 Plan 04: generated construction
invariants.** This is the plan-boundary completion ticket required by the
Stage 5 lifecycle; it carries only the already-reviewed Plan 04 work and
must land before Plan 05 implementation starts.

The completed slice makes generated declaration invariants the production
authority, removes the retired checked-constructor/refinement machinery, and
replaces the abandoned partial-Rust authority interpreter with a fail-closed
current-tree audit. The audit uses an explicit production-file inventory,
literal forbidden-token checks, compile-fail privacy/constructor tests, and
disposable-copy RED/GREEN authentication. A combined scratch perturbation
proves that Cargo/rustc expansion follows both a declaration surface and a
declared invariant without consulting a consumed mirror.

Completion evidence on the final reviewed tree:

- construction-core: 202 unit tests passed;
- construction proc macro: all 39 compile-fail fixtures and both compiled
  consumer tests passed;
- English v2: all unit, integration, and compile-fail doc tests passed;
- xtask: 400 library tests passed, 1 ignored, and 11 CLI tests passed;
- strict four-crate Clippy and formatting gates passed;
- the normalized corpus stayed exactly 32,285 total / 48 accepted, selected,
  covered, clean, byte-exact, and totally owned / 32,237 ordinary failures,
  with every ambiguity, internal, round-trip, ownership, gap, overlap,
  exception-use, synthetic-claim, and provenance-mismatch counter at zero;
- all seven timed gates stayed at or below the recorded 16.26-second Plan 04
  ceiling, and their semantic output matched the locked baseline.

The final independent review found no open findings. Standard constraints
apply.
