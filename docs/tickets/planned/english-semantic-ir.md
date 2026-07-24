---
needs: [english-structural-recovery-zero, english-surface-fact-diet]
---
**Semantic IR between the English AST and RON — the quotient layer where
identities live.** Decided 2026-07-23: interpose a simplified IR rather
than translating English AST → RON directly. The rejected direct path
would scatter normalization into a single-direction translator and leave
engine-adjacent identities (keyword-action semantics, rules-term
membership, anaphora resolution) with no shared home — the english crate
deliberately excludes them (shapes, not identities). Layering contract:
the english crate owns **shape completeness** (structural recovery zero,
byte-exact round-trip), the IR owns **shape equivalence** (syntactic
alternations collapse), RON owns **meaning**.

- **IR = semantic quotient of the English AST.** Start from a
  near-identity mapping (the english-surface-fact-diet is step zero —
  comma/contraction/casing bits are the first facts the quotient
  forgets), then earn each further divergence one attested alternation
  family at a time (passive/active, distributive subjects, coordination
  flattening, …) — corpus-witnessed, never speculative. This is the
  standing defense against the two IR failure modes: shallow copy of the
  source, shadow of the target.
- **Gate: idempotence** — the analogue of roundtrip `--require-clean`.
  Synthesis/rendering need not reproduce source bytes (that is the point
  of the quotient), but `IR(parse(render(synthesize(ir)))) == ir` must
  hold corpus-wide. One invariant tests normalization and realization
  choice together, runnable from day one because the byte-exact renderer
  already exists.
- **Macro discovery is census over IR normal forms**: frequency-rank IR
  subtrees across the supported corpus; the high-frequency shapes are
  the RON macro candidates, ranked by how much corpus each absorbs (the
  recovery campaign's methodology, one layer up). Authored macros land
  in `plugins/builtin/macros/`; stubs are the wizards `*.todo.ron`
  files.
- **Generation replaces the current `template` mechanism**: RON → IR →
  English AST → existing renderer. The renderer is the already-built
  hard half; the new work is realization choice (voice, coordination
  shape, sentence breaks). Migrate measurably: generate against current
  template output corpus-wide and diff — never a rewrite on faith.
- **Readiness metric**: a `RecoveredText` span is an IR hole, so the
  per-face structural-recovery census doubles as the IR coverage report.
- Guard: while a construction still recovers opaquely, do not "fix it in
  the IR" — that bakes syntax gaps into the semantic layer. The english
  crate owes shape completeness first.

Standard constraints apply.
