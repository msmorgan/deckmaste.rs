---
needs: []
---
**Move `LoyaltyOncePerTurn` out of the card language: engine auto-applies
it.** 2026-07-18 deep-dive: the once-per-turn loyalty restriction is
[CR#306.5d] — a rule of the game applying to every planeswalker's loyalty
abilities, never printed on cards. Today it is a `UseLimit` variant that the
three loyalty macros (`LoyaltyPlus`/`LoyaltyMinus`/`LoyaltyZero.ron`) must
each remember to author (`limits: [LoyaltyOncePerTurn]`), and enforcement
already needs cost-shape introspection (`is_loyalty_ability` on the Cost)
that no event fact carries — it is object-scoped across sibling abilities,
unlike the per-ability `OncePerTurn`.

Do: the engine detects loyalty-cost abilities (`is_loyalty_ability`, already
exists in `activate.rs`) and applies the shared once-per-turn gate
intrinsically. Then drop the `UseLimit::LoyaltyOncePerTurn` variant, the
`limits:` boilerplate from the three loyalty macros, the render suppression
arm (it already prints nothing), and mirror the removal in idris
(`UsageLimit` in `Core.idr`).

Behavior unchanged; this is authoring-surface subtraction. Renderer note:
the restriction stays unprinted, matching real cards.
