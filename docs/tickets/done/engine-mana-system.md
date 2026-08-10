---
needs: []
---
Mana pool provenance and riders (spend-only-on restrictions, mana that doesn't
empty), triggered mana abilities, and conditional mana production (Cavern of
Souls-style riders).

DELIVERED: pool provenance (`ManaPool(Vec<ManaUnit>)`), riders through
production, `SpendOnly` enforced at payment + affordability, `Persistent`
(non-emptying) mana, and Cavern-style conditional production. Triggered mana
abilities ([CR#605.1b]) remain in `engine-payment-obligation-window`, whose
explicit core classification and full-state mana-resolution path also supply
the missing "tapped for mana" event. On-spend riders
(`GrantOnSpend`/`TriggerOnSpend`) remain a seam (stored, not fired).
