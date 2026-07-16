---
needs: []
---
Parse count-scaled pump statics: `~ gets +1/+1 for each Elf you control.`
(also `… for each card in your graveyard`, `equipped/enchanted creature gets
+1/+1 for each …`). Lowers to the existing `Modify` static with a
`CountOf(<selection>)`-scaled magnitude — the count and selection parsers
exist; the static-ability parser (`parsers/static_ability.rs`) currently
declines the `for each` magnitude form.

Adjacent but SEPARATE: the full characteristic-defining `~'s power and
toughness are each equal to …` family (~112 one-away cards) is
`engine-cda-authorable-statics` (planned) — CDAs live in their own layer
and need that ticket's design; don't fold them in here.

**~124 of 17,022 one-away cards** (2026-07-16 tally).

Verify: `cargo xtask generate plugins/wizards` graduation delta;
`cargo xtask fidelity` on a for-each-Elf card.
