---
needs: []
---
**Delete the engine's legacy unbound-`It` → lone-announced-target fallback
— it violates the documented targets-are-never-anaphors invariant.**
Design context:
`docs/decisions/authoring-spelling-lowering.md`
(§7, verification obligation 2). Reported by the 2026-08-02 codex
consultation: the evaluator falls back from an unbound `It` to a lone
announced target, and that compatibility path even chases post-move
identity — unlike `Target(n)` itself. The `reference.rs` docs and the
Idris mirror both say `It` NEVER names an announced target.

## Scope

- **Verify first** (the consultation sampled the `It` resolution site in
  `resolve/query.rs`, not the fallback arm itself): pin the exact arm and
  its move-chasing behavior before deleting.
- Delete the fallback; unbound `It` degrades to the null id like every
  other unbound read (per the invalid-authoring-fizzles decision), and the
  Rust-side load lint for bare target-body anaphors is
  `validate-unbound-anaphor-lint` — coordinate, don't duplicate.
- Sweep canon/wizards for terms that silently depended on the fallback
  (idris-check already refuses the shape for the emitted subset; the
  ungated remainder is the risk).

## Gates

Standard constraints apply. Engine suites green; idris-check pass set
unchanged; any card that changes behavior is listed in the change
description with its fix.
