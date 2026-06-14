---
needs: []
---
Mana pool provenance and riders (spend-only-on restrictions, mana that doesn't
empty), triggered mana abilities, and conditional mana production (Cavern of
Souls-style riders).

DELIVERED: pool provenance (`ManaPool(Vec<ManaUnit>)`), riders through
production, `SpendOnly` enforced at payment + affordability, `Persistent`
(non-emptying) mana, and Cavern-style conditional production. Triggered mana
abilities ([CR#605.1b]) were carved into `engine-triggered-mana-abilities` (they
need a "tapped for mana" event the engine doesn't emit yet). On-spend riders
(`GrantOnSpend`/`TriggerOnSpend`) remain a seam (stored, not fired).
