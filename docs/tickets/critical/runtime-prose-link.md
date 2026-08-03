---
needs: [plugin-repoint]
---
**Runtime prose link: carry authored terms to every render site — the
card-level companion table, the value-keyed ability provenance index, and
the renderer/fidelity input repoint to authored.** Minted from the
2026-08-02 three-model consultation on the provenance seam (all repo
claims below re-verified against source). Design context:
`docs/decisions/authoring-spelling-lowering.md` (§3 content-digest
language, §9 "a term means its image under `lower`", §12 dual-result).

## Why (the deadline)

Prose rendering today feeds on core-side `Expanded` invocation provenance
(`crates/deckmaste_tui/src/ui/detail.rs:60-63`; `Expanded` match sites
throughout `crates/deckmaste_plugin/src/render/` + `fidelity.rs`). The
repoint makes core values wrapper-free, so that prose path silently
degrades to the `[unrendered]`/Debug fallback the moment it lands, and
`core-demacro` then deletes the variants those match arms name — a
compile break. Neither the repoint's ~97-site `deckmaste_engine` sweep
nor demacro's fixture sweep prices this conversion. This ticket owns it;
`core-demacro` `needs:` it.

## Scope

- **Card link**: companion table `CardId → Arc<authoring::Card>` built at
  the game-composition layer from the loader's retained dual result.
  (Delimitation vs `plugin-repoint`: the pair shaping and the
  `Deck::resolve` non-collapse are the repoint's side of the seam; the
  table may land on whichever side is convenient.) `CardId` assignment is
  deck-order-deterministic pre-shuffle
  (`crates/deckmaste_engine/src/state.rs:543-557`); pin the zip with one
  ordering test. Tokens/emblems resolve `None` here — their prose comes
  from the index.
- **Ability provenance index**: load-time `lower(a) → a` over every
  authored ability subterm — cards, tokens, and the counter/subtype
  `confers` payloads (the conferral family constructs `Ability::Innate`
  values at layer time, `crates/deckmaste_engine/src/layer.rs:2011-2028`;
  index their conferred forms from the registry side). Soundness, all
  verified: layer-6 grants push verbatim clones
  (`layer.rs:1685-1689`), lowering is structurally compositional
  (`crates/deckmaste_lowering/src/lib.rs:17-27`), and layer-3
  text-changing is a documented empty slot (`layer.rs:1717-1721`).
  Collisions: any preimage is semantically exact (§9); pick first-loaded,
  deterministically. Misses degrade to the visible `[unrendered]`
  fallback — never a panic, never invented prose.
- **Renderer/fidelity input repoint**: the legacy renderer and fidelity
  consume authored terms (which KEEP `Expanded`, so the invocation
  templates keep producing the good prose); the TUI detail pane resolves
  effective abilities through the index. Engine dependency set unchanged
  (`deckmaste_card` + `deckmaste_core` only).
- **Divergence-ledger invariant**: lowering stays context-free at
  `Ability` granularity. One corpus test: every authored ability
  subterm's lowering appears in its lowered card.

## Non-goals

Occurrence-exact origin attribution ("granted by WHICH Anthem") and
origin refs threaded through the layer system — deferred to
`engine-ability-origin-refs` (maybe/) behind explicit triggers; if that
ever lands, this index slots in as its fallback.

## Gates

Standard constraints apply. `cargo xtask fidelity` PASS over the authored
path; a layer-6-grant fixture renders prose (not `[unrendered]`) in the
detail pane; a miss-fallback negative fixture; the `CardId` ordering
test; `cargo tree` shows no new `deckmaste_engine` dependencies.
