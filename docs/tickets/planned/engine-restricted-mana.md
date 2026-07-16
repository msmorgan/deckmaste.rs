---
needs: []
---
Model spend-restricted mana and parse its rider: `{T}: Add one mana of any
color. Spend this mana only to cast a creature spell.` (also `… only to
activate abilities`, `… only to cast ~`, timing restrictions like `… during
the next turn`). Two halves:

1. **Engine**: mana produced with an attached spend restriction — a filter
   the payment step checks when that mana would pay for something. No such
   field exists on the mana representation today
   (`crates/deckmaste_core/src/mana.rs`). Restriction vocabulary: spell
   quality (type/subtype/color), ability activation, a named object, timing.
2. **Parse**: the trailing `Spend this mana only …` rider on mana abilities
   (same trailing-rider technique as `parse-activation-restrictions`),
   lowering onto the new restriction field.

Distinct from the filterland `ManaSpec::OneOf` single-color limitation noted
in the graduation-campaign frontier — that is about what CAN be added, this
is about what added mana may PAY for.

**~74 of 17,022 one-away cards** (2026-07-16 tally).

Verify: engine test paying a restricted mana into a matching vs non-matching
cost; `cargo xtask generate plugins/wizards` graduation delta.
