---
needs: [core-emblems, core-getdesignation-scopes-and-eviction]
---
The Ring emblem and tempts-you progression (50 cards). The Ring is an emblem
with a level-style progression.

## Current state (2026-07-18 deep-dive)

The mechanic-specific action variant has been retired. A future `The Ring
tempts you` builtin macro will expand to `Action::Composite` plus the
designation/emblem primitives; there is no parse/render arm yet and zero live
corpus cards. Emblem minting is real (`GetEmblem` → command zone,
`core-emblems` done) and the designation machinery fits — your-Ring-bearer
uniqueness is exactly `DesignationUniqueness::PerPlayer`, and the Monarch
macro is the precedent for a designation as pure RON data.

## Shape

Composite body per [CR#701.54a..701.54c]: emit the tempted fact; if you have
no Ring emblem, get it; the emblem gains the next tier (the four tiers as
`Conditionally(tempt-count >= N)` clauses reading a per-player count, not
emblem mutation); choose a creature you control — it becomes your Ring-bearer
until another does or you lose control of it. Not a copiable value
[CR#701.54b].

## Blocking seams (three)

1. Object-scope, single-holder-with-eviction designation grant — owned by
   `core-getdesignation-scopes-and-eviction`. The `Stored{scope: Object,
   uniqueness: PerPlayer, …}` shape Ring-bearer needs already exists; what is
   missing is `GetDesignation` coverage for it, prior-holder eviction, and
   registry loader wiring.
2. Per-player Ring-temptation count feeding the tier conditions.
3. The unconditional tempted fact: [CR#701.54d] — the "whenever the Ring
   tempts you" trigger fires even if some or all of the [CR#701.54a] actions
   were impossible, so the verb must emit the fact before/regardless of the
   choice resolving.

Note (2026-09-07): the "future builtin macro" is a v2 declaration (`TheRingTemptsYou`) routed by `semantics-v2-keyword-action-residues`; the engine blockers above are unchanged.
