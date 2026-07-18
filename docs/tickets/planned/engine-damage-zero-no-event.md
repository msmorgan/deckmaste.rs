---
needs: []
---
# engine-damage-zero-no-event — suppress 0-amount DamageDealt in the effect lane

The one-shot damage lane emits a `DamageDealt` event unconditionally, even when the
computed amount is 0. `Action::DealDamage` (`crates/deckmaste_engine/src/resolve/action.rs:58-75`)
maps every target to `GameEvent::DamageDealt { amount, .. }` with no zero guard.

[CR#120.8]: "If a source would deal 0 damage, it does not deal damage at all. That means
abilities that trigger on damage being dealt won't trigger." So a 0-power fight, a pumped-to-0
source, or any 0-amount deal must emit **no** event — today it emits a spurious 0-amount
`DamageDealt` that would wrongly fire "whenever ~ is dealt damage" / "whenever ~ deals damage"
triggers and any damage-count consumer.

The **combat** lane already filters 0 (`crates/deckmaste_engine/src/decide/mod.rs:725`); the
one-shot lane does not — a divergence. Fix: drop 0-amount events in `occurrence_of`/at the
`DealDamage` emit (mirror the combat filter), honoring [CR#120.8]. Note the replacement-effect
subtlety: [CR#614.7a] — a replacement that would *increase* the damage of a source dealing 0
still does nothing, so filter AFTER replacement, not before.

Found while grounding `engine-act-fight-patient`; that feature deliberately does NOT depend on
this (fight-trigger subjects are read from the body instructions, not damage events), so fixing
this cannot break fight triggers.
