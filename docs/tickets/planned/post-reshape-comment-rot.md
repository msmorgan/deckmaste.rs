---
needs: []
---
**Post-reshape comment and naming rot: small fixes, one sweep.** Found
during the 2026-08-02 semantics-program recon; none block anything.

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
- `crates/deckmaste_core/src/effect.rs` (~154-157): the `Targeted` doc
  comment says announced slots are "read back by the anaphors
  (`It`/`That(Sort)`/`They`, or `Target(n)`)" — contradicting the
  targets-are-never-anaphors invariant documented in `reference.rs` /
  `target_spec.rs` and proven Idris-side. Rewrite to the indexed-channel
  reading.
- `crates/deckmaste_core/src/action.rs` (~61-63): `EnterRider`'s claim
  that non-battlefield riders are "rejected by the Idris re-emit gate" is
  false (no such proof exists) — coordinate with
  `idris-mirror-semantics`, which owns resolving it (mint the proof or
  correct this prose); do not fix independently.

Line numbers are as of 2026-08-02 — re-grep at claim time.

## Routed ledger items

- `idris/src/Bridge.idr`'s illustrative v1/v2 comparison pseudo-code still names
  `AnyTarget`, which left the core for `Macros.anyTarget`. Already stale before
  that round and out of its consumption boundary —
  `docs/tickets/done/workbench-union-family-macros.md`.

## Gates

Standard constraints apply; comment-only/rename-only, so the suites are
the gate.
