---
needs: []
---
**Tests: new effects are covered only at the render/structural layer.** Found in
the 2026-06-29 code review.

The idris-port batch exercises most new effects' SEMANTICS only via render-string
or RON round-trip assertions; the engine behavior is untested for: `DivideAmong`
distribution, `MoveCounters`/`AllKinds`, redirectable-source fight (Pounce),
`TriggerMultiplier` (Panharmonicon extra firing), `MustPay` (Mana Leak
counter/charge), `ModifyPlayer` (Reliquary Tower no-max / Exploration extra land).
The three cards with no semantic test (Arc Lightning, Fate Transfer, Pounce) are
exactly the three with data bugs — a semantic test would have caught them.
(`PayPips` and `Modification` layers DO have real engine tests.)

Fix: add engine tests for each — e.g. Panharmonicon doubles an ETB trigger; Mana
Leak counters when {3} unpaid and resolves when paid; `DivideAmong` splits 3 among
chosen targets with each ≥1; Reliquary Tower removes the hand-size cap. Also
assert Convoke/Delve/Improvise keyword → `PayPips` expansion (currently only the
printed name is checked). Several of these will land naturally with the bug-fix
tickets; this ticket tracks the residual coverage.

Severity: **medium** (coverage gap that hid the bugs above). Effort: **M**.
