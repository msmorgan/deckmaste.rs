---
needs: []
---
**Delete the orphaned `Symbol` enum** (`deckmaste_core/src/symbol.rs`:
Tap/Untap/Phyrexian/Energy/Pawprint/Ticket/Mana). 2026-07-18 deep-dive: zero
consumers — the only reference is the `lib.rs` re-export. No serde target, no
parse/render arm, no engine eval, no idris constructor.

Every variant's real work already lives elsewhere:

- Energy — graduated player-counter composite: `PayEnergy`/`GainEnergy` macros
  over `Remove/PutCounters` + the Energy player-counter def (47 corpus uses);
  glyph-run fold in `deckmaste_cards/src/energy.rs`. Idris agrees ("no
  dedicated PayEnergy verb").
- Ticket — unimplemented; when needed it follows the Energy template exactly
  ([CR#107.17a]: pay = remove that many ticket counters from yourself). No
  symbol primitive required.
- Pawprint — NOT a currency: [CR#107.18] "does not represent a cost, mana,
  counters, or any type of persistent resource"; it belongs to modal
  mode-selection grammar ([CR#700.2i]) if ever modelled.
- Tap/Untap — `CostComponent::Tap`/`Untap` duplicates.
- Phyrexian — `ManaSymbol::Phyrexian` duplicate.

Do: remove `symbol.rs` and the `lib.rs` `mod`/`pub use` lines. Nothing else
changes; no replacement needed.
