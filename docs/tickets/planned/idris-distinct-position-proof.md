---
needs: []
---
**Idris `distinctOk` only proves `i < total`: thread the current slot
position through and prove `i < current`, rejecting self-reference and
enforcing the canonical later-slot direction.** Design context:
`docs/decisions/authoring-spelling-lowering.md`
(§7). Found by the 2026-08-02 codex consultation; the Rust engine already
rejects self and out-of-range sibling references — the mirror lags it.

## Scope

- `distinctOk` (or successor on the authoring mirror, once
  `idris-mirror-authoring` lands) receives the slot's own position; prove
  every sibling index strictly earlier.
- Canonical well-formedness while there: sibling lists sorted and
  duplicate-free; each undirected distinctness edge stored on the later
  slot only.

## Gates

Standard constraints apply. `idris/scripts/build` PASS; idris-check pass
set unchanged over canon; a negative Idris fixture for a self-referencing
and a forward-referencing `Distinct`.
