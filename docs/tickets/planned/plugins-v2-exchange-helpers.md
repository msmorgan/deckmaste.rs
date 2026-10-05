---
needs: []
---
**Six flat helpers over the one `Exchange` instruction.** Split from
`plugins-v2-keyword-body-defects` on 2026-10-05. Standard constraints apply.

## Why

The keyword action `exchange` has no body; it exists so the parser can read
the verb. Being named `exchange`, it shadows any alias for the `Exchange`
instruction, so `keyword_abilities/auraSwap.ron` still writes the
`Exchange(…)` constructor and sits on the raw allowlist (`ALLOWED_RAW` in
`crates/deckmaste_semantics_v2/tests/keyword_bodies.rs`).

## What is decided

- Keep the ONE existing instruction, `Exchange(Exchanged)`, whose arms are
  `LifeTotals`, `ControlOf`, `CardsAcross`, `Zones`, `Values`, `TextBoxes`
  [CR#701.12a..701.12h]. The all-or-nothing rule ("if the entire exchange
  can't be completed, no part of the exchange occurs" [CR#701.12a]) stays on
  that one node.
- Add six flat helpers, one per arm: `exchangeControl(a, b)`,
  `exchangeLifeTotals(players)`, `exchangeCards(a, b)`, `exchangeZones(a, b)`,
  `exchangeValues(a, b)`, `exchangeTextBoxes(a, b)`.
- The bodyless `exchange` keyword action stays, for parsing.

## The work

1. Add the six helpers under `plugins_v2/builtin/macros/instructions/`.
2. Re-spell `auraSwap` (it writes `Exchange(CardsAcross(…))`) over
   `exchangeCards` and strike `keyword_abilities/auraSwap` from
   `ALLOWED_RAW`. No other file in `plugins_v2/` writes `Exchange(` today
   (canon's Arcanum Wings calls `auraSwap`).

## Proof

`cargo xtask expansions` before/after is byte-identical (a re-spelling), and
`keyword_bodies` passes with the shorter allowlist.

## Related

`lean-core-exchange-operands` (maybe/) asks whether life totals fold into
numeric operands; `research-exchange-textbox` surveys which arms cards use.
Neither blocks this.
