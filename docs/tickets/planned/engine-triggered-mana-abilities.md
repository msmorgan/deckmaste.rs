---
needs: [engine-mana-system]
---
Triggered mana abilities ([CR#605.1b]): a triggered ability that requires no
target, triggers from a mana ability's activation / a permanent being tapped for
mana ([CR#106.12]), and could add mana resolves WITHOUT using the stack
([CR#605.3b]). Carved out of engine-mana-system: needs the engine to (1) emit a
"tapped for mana" fact carrying `Agency::ManaAbilityResolution` when a mana
ability resolves (today the stackless path taps with `CostPayment` and emits
`ManaAdded` — neither matches), (2) make triggers match it (a `StateBecomes`
tapped-cause coordinate, or a dedicated event), (3) a `triggered_mana_ability`
recognizer (no target + effect solely `AddMana` + mana-production trigger), and
(4) stackless resolution at the `PlaceTriggers` barrier. Unlocks the mana-adder
family (Gauntlet of Power, Mirari's Wake, Utopia Sprawl, Fertile Ground, Market
Festival, Regal Behemoth, …). Follow-ups beyond v1: "additional" mana and "one
mana of any type that land produced" (Nikya / Mirari's Wake) reference the
produced mana, needing the production result threaded to the trigger.
