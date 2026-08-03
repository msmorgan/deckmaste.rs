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

## Why (this ticket owns the whole provenance flip)

Prose rendering today feeds on core-side `Expanded` invocation provenance
(`crates/deckmaste_tui/src/ui/detail.rs:60-63`; `Expanded` match sites
throughout `crates/deckmaste_plugin/src/render/` + `fidelity.rs`). Erasing
the wrappers without replacing that channel silently degrades every prose
path to the `[unrendered]`/Debug fallback, so the erasure and its
replacement land together HERE (spec §12, sequenced 2026-08-02):
`plugin-repoint` leaves the `Expansion` arms as identities so its lowered
core stays byte-identical and every provenance consumer keeps working,
and this ticket flips provenance and repoints its consumers in one
landing. Neither the repoint's ~97-site `deckmaste_engine` sweep nor
demacro's fixture sweep priced the renderer/fidelity conversion; it is
priced here. `core-demacro` `needs:` this, and deletes the core variants
last.

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
- **Provenance erasure**: the lowering map's `Expansion` arms become
  erasure arms — the crate's first non-identity arms — and the per-variant
  mapping tests they force are edited to their new expected shape in place
  (spec §16.3; this is the ticket that does it). The ~97 now-dead
  production `Expanded(…)` match sites across ~14 `deckmaste_engine`
  modules come out with them.
- **Renderer/fidelity move-and-repoint**: the legacy renderer and the
  fidelity harness that rides it (`fidelity.rs:44` uses
  `crate::render::render_card_face`) move together into
  `deckmaste_legacy_render` — `src/{render,template}/**` + `fidelity.rs`,
  ~13.2k of the plugin crate's ~21.1k lines — FUSED with the retype onto
  authored terms, one traversal rather than two (spec §1, owner-settled
  2026-08-02). Authored terms KEEP `Expanded`, so the invocation templates
  keep producing the good prose; the TUI detail pane resolves effective
  abilities through the index. Depending on `deckmaste_authoring` instead
  of `deckmaste_core` is what takes this code off `core-demacro`'s path,
  and makes its eventual death a crate removal. Engine dependency set
  unchanged (`deckmaste_card` + `deckmaste_core` only).
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
test; no `Expanded` match sites left in `deckmaste_engine`;
`deckmaste_plugin` no longer contains the renderer or fidelity;
`cargo tree` shows no new `deckmaste_engine` dependencies and
`deckmaste_legacy_render` depending on `deckmaste_authoring`, not
`deckmaste_core`.
