---
needs: []
---
Parse the additional-cost line: `As an additional cost to cast ~, sacrifice
a creature.` / `… discard a card.` / `… pay N life.` / `… discard X cards.`,
including the choice form (`… sacrifice a blue permanent or pay {3}.`). The
engine/grammar side landed with `core-additional-cost-effect` and
`engine-cost-payment` (both done); the line still fails to parse — no
production in `parsers/` recognizes the `As an additional cost to cast ~,
<cost-action>` frame and lowers the cost-action phrase onto the additional-
cost shape those tickets built. The cost-action vocabulary should reuse the
existing cost parsers (`parsers/cost.rs`), not re-encode verbs.

**~105 of 17,022 one-away cards** (2026-07-16 tally).

Verify: `cargo xtask generate plugins/wizards` graduation delta;
`cargo xtask fidelity` on a sacrifice-cost and a discard-X card.
