---
needs: []
---
**Home the Saga sacrifice on `Property::StateBased`, not a card-level
static.** Residue of `state-checked-static-home` (2026-09-04). The testing
Sagas (`plugins/testing/cards/Test Saga*.ron`) author the [CR#714.4]
sacrifice as a card-level static, so after the rename it lowers to
`Static(ConditionallyDo(…))` — a rules-defined state-based action still
wearing a static ability. [CR#714.4] says the sacrifice is a state-based
action that doesn't use the stack; its home is a `Property::StateBased`
conferral on the Saga subtype (the shape Aura's [CR#704.5m] rule took in
`idris-sba-not-a-static-ability`) or an `SbaRule`. The v2 Saga stub
declares neither. Ascend is not in this bucket: [CR#702.131b] makes it a
static ability outright.

Fix: confer the [CR#714.4] rule on the Saga subtype in the v2 stub (or the
engine's SBA rule set), drop the card-level static from the test Sagas,
keep every Saga test green (re-spelled, never deleted), and make lowering
refuse a card-level static that restates a rules-defined SBA where it can
recognise one. Scope: workbench, core, engine, lowering's core-facing
output; `deckmaste_semantics` untouched.

Size: S–M. Done when: no test Saga carries the sacrifice as a static;
the engine still sacrifices a finished Saga; workspace tests green.
Standard constraints apply.
