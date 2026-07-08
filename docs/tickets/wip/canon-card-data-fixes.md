---
needs: []
---
**Canon-card data errors (3) found in the 2026-06-29 code review.**

- **Fate Transfer** (`plugins/canon/cards/Fate Transfer.ron`): cost is
  `[Generic(1), Black]` = {1}{B}; the real card is **{1}{U/B}**. The model
  supports two-color hybrid (Footlight Fiend uses `Hybrid(Black, Red)`). Fix to a
  blue/black hybrid pip.
- **Pounce** (`plugins/canon/cards/Pounce.ron`): `types: [Sorcery]`; the real
  card is an **Instant** (changes timing/castability). Fix the type.
- **Arc Lightning** (`plugins/canon/cards/Arc Lightning.ron`):
  `group: Choose(AnyNumber, CreatureOrPlayer)` — `AnyNumber` permits **0** targets
  (real card is one, two, or three → use the `Between(1,3)` macro) and
  `CreatureOrPlayer` excludes planeswalkers (use `AnyTarget`, as Tribal Flames
  does). Note: Arc Lightning won't be fully playable until
  `engine-divide-among-player-panic` and
  `engine-divided-distribution-as-you-choose` are fixed.

Fix the three RON files; add a canon test per card asserting the corrected
cost/type/targeting.

Severity: **medium** (rules-incorrect shipped data). Effort: **S**.
