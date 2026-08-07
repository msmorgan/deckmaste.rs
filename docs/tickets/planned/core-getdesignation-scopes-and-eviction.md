---
needs: []
---
**Extend the `GetDesignation` grant verb past its player-scope-flag v1.** The
verb (`crates/deckmaste_core/src/action.rs`) is the generic designation grant,
but v1 covers only the present-or-absent player flag. Its own doc comment names
the two uncovered cases and their consumers:

- **Single-holder with eviction** — monarch, the initiative. Granting moves the
  designation off the prior holder.
- **Object-scope grants** — goad, suspected.

Ring-bearer needs both at once: object-scope, `PerPlayer` uniqueness, and
eviction — the designation ends when another creature becomes that player's
Ring-bearer, or when another player gains control of the bearer [CR#701.54a].

**The data model is already complete — this is verb and runtime work, not a
taxonomy change.** `crates/deckmaste_core/src/designation.rs` already carries
`DesignationScope`, `DesignationUniqueness` (including `PerPlayer`), and
`DesignationDef::Stored { scope, shape, uniqueness, persistence, payload }`. A
`Stored` row with object scope and per-player uniqueness is expressible today;
nothing grants or evicts against it.

Also in scope: `DesignationDecl` has no loader wiring (`designation.rs`), so the
open registry is unreachable from cards. Only two rows exist —
`plugins/builtin/macros/designations/{Commander,Monarch}.ron`.

Five mechanics are blocked on this: monarch, the initiative, goad, suspected,
Ring-bearer.

Standard constraints apply.
