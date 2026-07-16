---
needs: []
---
Parse self-referential cost modification: `~ costs {1} less to cast if you
control a Wizard.`, `~ costs {1} less to cast for each artifact you
control.`, the unconditional `~ costs {2} less to cast.`, and the tax twin
(`costs {N} more`). Sibling of `parse-cost-reduction-static` (done), which
covered the `<Type> spells you cast cost {N} less` static — this is the
same `CostModifier` emission scoped to the card itself, plus the conditional
(`if <condition>` via the Condition-macro path) and scaling (`for each
<selection>`) magnitude forms the done ticket didn't reach.

Parse half only: engine application of cost modifiers during casting is the
cost-modification pipeline work (tracked with `engine-alt-costs` and kin);
graduation is parse coverage.

**~102 of 17,022 one-away cards** (2026-07-16 tally).

Verify: `cargo xtask generate plugins/wizards` graduation delta;
`cargo xtask fidelity` on a conditional and a for-each reducer card.
