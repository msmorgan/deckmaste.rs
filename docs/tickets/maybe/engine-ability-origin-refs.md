---
needs: [runtime-prose-link]
---
**Occurrence-exact ability origins: opaque source refs emitted at `lower`
time and propagated engine-side, with "why present" (cause) separated
from "what renders it" (source).** Deferred by the 2026-08-02
consultation: build ONLY when a trigger arrives — a UI needs per-source
attribution ("granted by which Anthem" — identical values from two lords
are indistinguishable in the value-keyed index), a real text-changing
effect lands (the layer-3 slot,
`crates/deckmaste_engine/src/layer.rs:1717-1721`), or idiom
misrendering from index collisions is observed in practice.

## Decision criterion (documented; re-check at claim time)

The `runtime-prose-link` value index suffices iff canonical spelling
factors through core: `lower(a) == lower(b)` ⇒ same rendered prose. The
law fails in general — invocation equality is provenance-sensitive by
design (`crates/macro_ron/src/expansion.rs:10-13`) and lowering erases
the wrappers — but holds in practice on a canonical-form authored
surface. This ticket exists for when "in practice" stops being good
enough.

## Shape (sketch; re-derive against then-current code)

- Correspondence emitted BY lowering at render-bearing subterms — never
  path arithmetic over lowered structure after the fact. Opaque typed
  tokens, outside core equality/hash/serde and all game decisions.
- Engine-side propagation surface (the cost — price it honestly): layer
  copy-on-write ability lists, the whole-pass fixpoint re-gathering
  (`layer.rs:2044-2059`), stack by-value bodies, conferral registries,
  token/emblem creation.
- The layer pipeline already tracks each continuous effect's source for
  [CR#613.7] dependency ordering — locate that hook at claim time rather
  than adding a parallel channel.
- Composes with the index: refs when present, index lookup as fallback;
  synthetic/no-preimage values fail visibly, never invent prose.

## Fixtures (decisive set)

Printed, lost, gained, granted-static-to-fixpoint, type-conferred,
copied, token/emblem, and source-less synthetic.
