---
needs: []
---
**Post-reshape comment and naming rot: small fixes, one sweep.** Found
during the 2026-08-02 authoring-program recon; none block anything.

- `crates/deckmaste_core/src/ron.rs` (~line 70): kind-registration comment
  still claims "No card position reads TypeDef yet — types stays
  `Vec<Type>` until the type-flip migration"; the flip landed
  (`card.rs`/`token.rs` are `TypeDef` today).
- `crates/deckmaste_core/src/type.rs` (~lines 168-169): same stale
  type-line claim.
- `crates/deckmaste_engine/src/resolve/player_action.rs`: the filename
  outlived the deleted `PlayerAction` type; rename to match its content.
- Stale pre-reshape `By(...)` spellings in comments:
  `resolve/action.rs` (~786, ~2160), `deckmaste_migrations/src/resolve.rs`
  (~425), `macro_ron/src/tests.rs` fixture literals (~1293, ~1306 —
  harmless, body text is opaque to that crate, but confusing).

Line numbers are as of 2026-08-02 — re-grep at claim time.

## Gates

Standard constraints apply; comment-only/rename-only, so the suites are
the gate.
