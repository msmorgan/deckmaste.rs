---
needs: []
---
DONE: `GameState::mana_cost` now runs the [CR#601.2f] pipeline
(`cast::modified_mana_cost`). Applicable `CostModifier` rows are the spell's
OWN statics (affinity's `of: Ref(This)`, `Scaled(Reduce, times: CountOf)`
[CR#702.41a]) plus battlefield statics whose `of` admits the spell (sphere
taxers/reducers), each evaluated in a frame anchored to the row's carrier.
Increases and mandatory additional MANA apply before reductions; `{N}`
reductions floor the generic component at `{0}`; a colored reduction removes
only its matching pip. The legal.rs P0.W2 presence guard converted to this
pipeline. Loud residue: non-mana components on a cost-change row
(`todo!("P0.W2 residue…")`); the optional-`Additional` (kicker, [CR#118.8b])
announce stays inert pending [[core-alt-costs]]; activation-cost modifiers
("abilities cost less") are a separate lane the guard never covered — ticket
when a card needs them.

NOTE — scope carve-out: convoke / delve / improvise are **not** cost modifiers.
They are per-pip *alternative payment* applied after the total cost locks in, not
reductions to it ([CR#702.51b]). They belong to `core-pip-payment` (planned/), not
this ticket.
